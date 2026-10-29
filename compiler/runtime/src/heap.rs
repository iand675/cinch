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
//!   `cinch-alloc: blocks=<n> bytes=<n> peak=<n>` to stderr (cumulative
//!   blocks and bytes, peak live bytes). The line is written by an `atexit`
//!   handler registered on the first allocation, so it also appears after a
//!   `cinch_panic`/`cinch_blackhole`/heap-limit fault (all of which leave
//!   through `exit`), but not after a signal.
//! * `CINCH_HEAP_LIMIT=<n>[K|M|G]`: fault with exit code
//!   [`crate::abi::EXIT_HEAP_LIMIT`] and `cinch: heap limit exceeded (<n>
//!   bytes)` once cumulative allocated bytes would exceed `n`. Unset means
//!   [`DEFAULT_HEAP_LIMIT`] (8 GiB); `0` disables the cap. An unparsable
//!   value falls back to the default rather than disabling the cap.
//!
//! The limit is on cumulative allocation, not live bytes: nothing is freed
//! yet, so the two coincide for generated blocks, and counting cumulative
//! bytes keeps the cap meaningful for transient runtime buffers too.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering::Relaxed};
use std::sync::LazyLock;

/// The cap applied when `CINCH_HEAP_LIMIT` is unset: 8 GiB.
pub const DEFAULT_HEAP_LIMIT: u64 = 8 << 30;

static BLOCKS: AtomicU64 = AtomicU64::new(0);
static BYTES: AtomicU64 = AtomicU64::new(0);
static CURRENT: AtomicU64 = AtomicU64::new(0);
static PEAK: AtomicU64 = AtomicU64::new(0);
/// Set once the heap-limit fault starts: the exit path (stderr writes,
/// `atexit` handlers) may still allocate and must not fault again.
static FAULTING: AtomicBool = AtomicBool::new(false);

struct Config {
    /// 0 means uncapped.
    limit: u64,
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
    Config { limit }
});

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

/// Record an allocation of `size` bytes, faulting if it would cross the
/// heap cap. The single accounting point for every heap block.
pub fn account_alloc(size: usize) {
    let size = size as u64;
    let total = BYTES.fetch_add(size, Relaxed) + size;
    BLOCKS.fetch_add(1, Relaxed);
    let cur = CURRENT.fetch_add(size, Relaxed) + size;
    PEAK.fetch_max(cur, Relaxed);
    let limit = CONFIG.limit;
    if limit != 0 && total > limit && !FAULTING.swap(true, Relaxed) {
        heap_limit_exit(limit);
    }
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

fn heap_limit_exit(limit: u64) -> ! {
    let mut line = Line::new();
    line.push(b"cinch: heap limit exceeded (");
    line.num(limit);
    line.push(b" bytes)\n");
    line.flush();
    // `exit`, not `_exit`, so the stats handler still runs.
    std::process::exit(crate::error::Error::HeapLimit.exit_code())
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
    line.push(b"\n");
    line.flush();
}

/// A fixed stack buffer for the two stderr lines: neither may allocate
/// (one runs inside the allocator, the other after a heap fault).
struct Line {
    buf: [u8; 128],
    len: usize,
}

impl Line {
    fn new() -> Self {
        Line {
            buf: [0; 128],
            len: 0,
        }
    }

    fn push(&mut self, s: &[u8]) {
        let n = s.len().min(self.buf.len() - self.len);
        self.buf[self.len..self.len + n].copy_from_slice(&s[..n]);
        self.len += n;
    }

    fn num(&mut self, mut v: u64) {
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

    fn flush(&self) {
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
/// [`account_alloc`]/[`account_free`].
pub struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        account_alloc(layout.size());
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        account_alloc(layout.size());
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        account_free(layout.size());
        unsafe { System.dealloc(ptr, layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        account_free(layout.size());
        account_alloc(new_size);
        unsafe { System.realloc(ptr, layout, new_size) }
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
