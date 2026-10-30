//! Heap accounting and the heap cap.
//!
//! Every heap allocation a compiled program performs reaches one function,
//! [`account_alloc`]: generated code allocates through
//! [`crate::abi::cinch_alloc_raw`] / [`crate::abi::cinch_alloc`], and both of
//! those plus every runtime-side `Box`/`Vec` (the `Nat`/`Int`/`Bytes` boxes,
//! bignum limbs, byte buffers, handle tables) go through the Rust global
//! allocator, which this module replaces with a counting wrapper over the
//! system allocator. Counting at the global allocator rather than at each
//! boxing call site means no runtime allocation can escape the cap, and a
//! real deallocation already decrements the current-bytes counter through
//! [`account_free`], so the reclamation slice only has to make `cinch_free`
//! actually deallocate.
//!
//! Environment (read once, without allocating, on the first allocation):
//!
//! * `CINCH_ALLOC_STATS=1`: at process exit write
//!   `cinch-alloc: blocks=<n> bytes=<n> peak=<n> header-blocks=<n>
//!   header-bytes=<n> immortal=<n> freed=<n> live=<n>` to stderr.
//!   `blocks`, `bytes` and `peak` cover every allocation (cumulative
//!   blocks and bytes, peak live bytes). The rest count only Cinch value
//!   blocks, the ones with a header word ([`crate::block`]): cumulative
//!   header blocks and their bytes including the header, how many of them
//!   were born immortal (interned `Sym` cells; static blocks live in the
//!   data section and are never allocated), how many were freed (by
//!   `cinch_free` or the drop machinery of [`crate::rc`]), and how many
//!   are live (`header-blocks - freed`, immortal ones included).
//!   `blocks - header-blocks` is the runtime's own allocations (limbs,
//!   buffer data, handle tables). The line is written by an `atexit`
//!   handler registered on the first allocation, so it also appears after a
//!   `cinch_panic`/`cinch_blackhole` fault and after a heap or thread
//!   `Exhausted` exit (all of which leave through `exit`), but not after a
//!   signal or a stack `Exhausted` exit (which leaves through `_exit`,
//!   [`crate::stack`]).
//! * `CINCH_HEAP_LIMIT=<n>[K|M|G]`: once the live heap (bytes allocated
//!   and not yet freed, whose high-water mark is the stats line's `peak`)
//!   would exceed `n`, exit `Exhausted` ([`crate::abi::EXIT_EXHAUSTED`])
//!   with `cinch: exhausted: heap (<n> bytes)`, where `<n>` is the live
//!   bytes the failing allocation would have brought the heap to. Unset
//!   means [`DEFAULT_HEAP_LIMIT`] (8 GiB); `0` disables the cap. An
//!   unparsable value falls back to the default rather than disabling
//!   the cap.
//! * `CINCH_LEAK_CHECK=1` and `CINCH_RC_DEBUG=1`: the diagnostics of
//!   [`crate::rc`].
//!
//! The cap bounds live bytes, not the lifetime total: reclamation frees
//! blocks as they die, so a program whose live heap stays under the cap
//! runs to completion however much it allocates over its life, and the
//! cap bounds the memory the process actually holds. A real allocation
//! failure (the system allocator answering null) is the same `Exhausted`
//! exit, never Rust's `handle_alloc_error` abort, so the global
//! allocator never answers null.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::sync::LazyLock;

/// The cap applied when `CINCH_HEAP_LIMIT` is unset: 8 GiB.
pub const DEFAULT_HEAP_LIMIT: u64 = 8 << 30;

static BLOCKS: AtomicU64 = AtomicU64::new(0);
static BYTES: AtomicU64 = AtomicU64::new(0);
static CURRENT: AtomicU64 = AtomicU64::new(0);
static PEAK: AtomicU64 = AtomicU64::new(0);
static HEADER_BLOCKS: AtomicU64 = AtomicU64::new(0);
static HEADER_BYTES: AtomicU64 = AtomicU64::new(0);
static IMMORTAL: AtomicU64 = AtomicU64::new(0);
static FREED: AtomicU64 = AtomicU64::new(0);

struct Config {
    /// 0 means uncapped.
    limit: u64,
    /// `CINCH_RC_DEBUG=1`.
    rc_debug: bool,
    /// `CINCH_LEAK_CHECK=1`.
    leak_check: bool,
}

unsafe extern "C" {
    fn getenv(name: *const std::ffi::c_char) -> *const std::ffi::c_char;
    fn atexit(f: extern "C" fn()) -> i32;
    fn write(fd: i32, buf: *const u8, n: usize) -> isize;
}

/// The environment, read through `getenv` so initialisation never
/// allocates (it runs inside the allocator).
static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    if env_bytes(c"CINCH_ALLOC_STATS") == Some(b"1") {
        unsafe { atexit(print_stats) };
    }
    let limit = match env_bytes(c"CINCH_HEAP_LIMIT") {
        None => DEFAULT_HEAP_LIMIT,
        Some(v) => parse_limit(v).unwrap_or(DEFAULT_HEAP_LIMIT),
    };
    Config {
        limit,
        rc_debug: env_bytes(c"CINCH_RC_DEBUG") == Some(b"1"),
        leak_check: env_bytes(c"CINCH_LEAK_CHECK") == Some(b"1"),
    }
});

/// Whether `CINCH_RC_DEBUG=1`: freed blocks are poisoned and kept, and the
/// count entry points validate headers ([`crate::rc`]).
pub fn rc_debug() -> bool {
    #[cfg(test)]
    if FORCE_RC_DEBUG.with(std::cell::Cell::get) {
        return true;
    }
    CONFIG.rc_debug
}

#[cfg(test)]
thread_local! {
    /// Debug mode for the calling test thread only, so a unit test can
    /// observe poisoning without the environment.
    pub(crate) static FORCE_RC_DEBUG: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Whether `CINCH_LEAK_CHECK=1` ([`crate::rc::cinch_exit_check`]).
pub fn leak_check() -> bool {
    CONFIG.leak_check
}

pub(crate) fn env_bytes(name: &std::ffi::CStr) -> Option<&'static [u8]> {
    let p = unsafe { getenv(name.as_ptr()) };
    if p.is_null() {
        None
    } else {
        Some(unsafe { std::ffi::CStr::from_ptr(p) }.to_bytes())
    }
}

/// Parse `<digits>[K|M|G]` (binary multiples, either case); also the
/// `CINCH_STACK_SIZE` syntax (`crate::platform::cinch_run_main`).
pub(crate) fn parse_limit(v: &[u8]) -> Option<u64> {
    let (digits, mult) = match v.last()? {
        b'K' | b'k' => (&v[..v.len() - 1], 1u64 << 10),
        b'M' | b'm' => (&v[..v.len() - 1], 1 << 20),
        b'G' | b'g' => (&v[..v.len() - 1], 1 << 30),
        _ => (v, 1),
    };
    if digits.is_empty() {
        return None;
    }
    let mut n: u64 = 0;
    for &d in digits {
        if !d.is_ascii_digit() {
            return None;
        }
        n = n.checked_mul(10)?.checked_add(u64::from(d - b'0'))?;
    }
    n.checked_mul(mult)
}

/// Record an allocation of `size` bytes, exiting `Exhausted` if it would
/// take the live heap past the cap. The single accounting point for
/// every heap block. Once an `Exhausted` exit is under way the cap no
/// longer applies: the exit path (stderr writes, `atexit` handlers) may
/// still allocate and must not exit twice.
pub fn account_alloc(size: usize) {
    let size = size as u64;
    BYTES.fetch_add(size, Relaxed);
    BLOCKS.fetch_add(1, Relaxed);
    let cur = CURRENT.fetch_add(size, Relaxed) + size;
    PEAK.fetch_max(cur, Relaxed);
    let limit = CONFIG.limit;
    if limit != 0 && cur > limit && !crate::abi::exhausting() {
        crate::abi::exhausted(crate::abi::Resource::Heap(cur));
    }
}

/// The system allocator answered null for an allocation already counted
/// by [`account_alloc`]: the `Exhausted` exit, reporting the live bytes
/// the program asked to hold. A failure on the exit path itself leaves
/// at once with the same code.
fn refused() -> ! {
    if crate::abi::exhausting() {
        crate::abi::exhausted_again();
    }
    crate::abi::exhausted(crate::abi::Resource::Heap(CURRENT.load(Relaxed)))
}

/// A request of `size` bytes that no allocation can satisfy (its size
/// does not fit an allocation layout): the same `Exhausted` exit, with
/// the live bytes it would have brought the heap to.
pub(crate) fn request_too_large(size: usize) -> ! {
    let n = CURRENT.load(Relaxed).saturating_add(size as u64);
    crate::abi::exhausted(crate::abi::Resource::Heap(n))
}

/// Record that a header block of `size` bytes (header included) was
/// allocated. Called by [`crate::block::allocate`] in addition to the global
/// allocator's [`account_alloc`], so header blocks stay a subset of
/// `blocks`.
pub fn account_block(size: usize, immortal: bool) {
    HEADER_BLOCKS.fetch_add(1, Relaxed);
    HEADER_BYTES.fetch_add(size as u64, Relaxed);
    if immortal {
        IMMORTAL.fetch_add(1, Relaxed);
    }
}

/// Record that a header block was freed (`cinch_free`, the drop
/// machinery, or a debug-mode quarantine).
pub fn account_block_free() {
    FREED.fetch_add(1, Relaxed);
    #[cfg(test)]
    THREAD_FREED.with(|c| c.set(c.get() + 1));
}

#[cfg(test)]
thread_local! {
    static THREAD_FREED: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// The header blocks the calling test thread has freed: the global
/// counter also counts the tests running beside it.
#[cfg(test)]
pub(crate) fn thread_freed() -> u64 {
    THREAD_FREED.with(std::cell::Cell::get)
}

/// Held by the unit tests whose allocation volume would disturb a
/// live-bytes measurement running beside them.
#[cfg(test)]
pub(crate) static TEST_HEAVY: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Snapshot `(header_blocks, header_bytes, immortal, freed)`.
pub fn block_stats() -> (u64, u64, u64, u64) {
    (
        HEADER_BLOCKS.load(Relaxed),
        HEADER_BYTES.load(Relaxed),
        IMMORTAL.load(Relaxed),
        FREED.load(Relaxed),
    )
}

/// Header blocks allocated and not freed, immortal ones included.
pub fn live_blocks() -> u64 {
    let (hblocks, _, _, freed) = block_stats();
    hblocks.saturating_sub(freed)
}

/// Record that a block of `size` bytes was released.
pub fn account_free(size: usize) {
    CURRENT.fetch_sub(size as u64, Relaxed);
}

/// Snapshot `(blocks, bytes, current, peak)`.
pub fn stats() -> (u64, u64, u64, u64) {
    (
        BLOCKS.load(Relaxed),
        BYTES.load(Relaxed),
        CURRENT.load(Relaxed),
        PEAK.load(Relaxed),
    )
}

extern "C" fn print_stats() {
    let (blocks, bytes, _, peak) = stats();
    let mut line = Line::new();
    line.push(b"cinch-alloc: blocks=");
    line.num(blocks);
    line.push(b" bytes=");
    line.num(bytes);
    line.push(b" peak=");
    line.num(peak);
    let (hblocks, hbytes, immortal, freed) = block_stats();
    line.push(b" header-blocks=");
    line.num(hblocks);
    line.push(b" header-bytes=");
    line.num(hbytes);
    line.push(b" immortal=");
    line.num(immortal);
    line.push(b" freed=");
    line.num(freed);
    line.push(b" live=");
    line.num(hblocks.saturating_sub(freed));
    line.push(b"\n");
    line.flush();
}

/// Write the leak check's line, `cinch-leak: live=<n> immortal=<n>
/// leaked=<n>`, to stderr without allocating.
pub(crate) fn write_leak_report(live: u64, immortal: u64, leaked: u64) {
    let mut line = Line::new();
    line.push(b"cinch-leak: live=");
    line.num(live);
    line.push(b" immortal=");
    line.num(immortal);
    line.push(b" leaked=");
    line.num(leaked);
    line.push(b"\n");
    line.flush();
}

/// A fixed stack buffer for the runtime's no-allocation stderr lines (the
/// stats and leak lines, and the `Exhausted` line of
/// [`crate::abi::exhausted`], which runs inside the allocator or a signal
/// handler).
pub(crate) struct Line {
    buf: [u8; 256],
    len: usize,
}

impl Line {
    pub(crate) fn new() -> Self {
        Line {
            buf: [0; 256],
            len: 0,
        }
    }

    pub(crate) fn push(&mut self, s: &[u8]) {
        let n = s.len().min(self.buf.len() - self.len);
        self.buf[self.len..self.len + n].copy_from_slice(&s[..n]);
        self.len += n;
    }

    pub(crate) fn num(&mut self, mut v: u64) {
        let mut tmp = [0u8; 20];
        let mut i = tmp.len();
        loop {
            i -= 1;
            tmp[i] = b'0' + (v % 10) as u8;
            v /= 10;
            if v == 0 {
                break;
            }
        }
        self.push(&tmp[i..]);
    }

    pub(crate) fn flush(&self) {
        let mut off = 0;
        while off < self.len {
            let n = unsafe { write(2, self.buf[off..].as_ptr(), self.len - off) };
            if n <= 0 {
                return;
            }
            off += n as usize;
        }
    }
}

/// The counting global allocator: the system allocator plus
/// [`account_alloc`]/[`account_free`]. It never answers null: a refused
/// allocation is the heap `Exhausted` exit ([`refused`]).
pub struct Counting;

/// `p`, or the `Exhausted` exit when the system allocator refused.
fn granted(p: *mut u8) -> *mut u8 {
    if p.is_null() {
        refused();
    }
    p
}

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        account_alloc(layout.size());
        granted(unsafe { System.alloc(layout) })
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        account_alloc(layout.size());
        granted(unsafe { System.alloc_zeroed(layout) })
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        account_free(layout.size());
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        account_free(layout.size());
        account_alloc(new_size);
        granted(unsafe { System.realloc(ptr, layout, new_size) })
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limit_suffixes() {
        assert_eq!(parse_limit(b"0"), Some(0));
        assert_eq!(parse_limit(b"123"), Some(123));
        assert_eq!(parse_limit(b"64M"), Some(64 << 20));
        assert_eq!(parse_limit(b"2g"), Some(2 << 30));
        assert_eq!(parse_limit(b"4K"), Some(4096));
        assert_eq!(parse_limit(b"G"), None);
        assert_eq!(parse_limit(b"1x"), None);
        assert_eq!(parse_limit(b""), None);
    }

    #[test]
    fn allocations_are_counted() {
        let (b0, n0, _, _) = stats();
        let v = std::hint::black_box(vec![0u8; 4096]);
        let (b1, n1, _, peak) = stats();
        assert!(b1 > b0);
        assert!(n1 >= n0 + 4096);
        assert!(peak >= 4096);
        drop(v);
    }
}
