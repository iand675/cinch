//! Heap accounting and the heap cap.
//!
//! Every heap allocation a compiled program performs is accounted here:
//! generated code allocates through [`crate::abi::cinch_alloc_raw`] /
//! [`crate::abi::cinch_alloc`] and their shaped siblings, which reach
//! [`alloc_block`] through [`crate::block`], and every runtime-side
//! `Box`/`Vec` (the contents of the `Nat`/`Int`/`Bytes` boxes, bignum
//! limbs, byte buffers, handle tables) goes through the Rust global
//! allocator, which this module replaces with a counting allocator,
//! [`Counting`]. Counting at the global allocator rather than at each
//! boxing call site means no runtime allocation can escape the cap.
//!
//! Both paths place memory the same way: a request of at most
//! [`crate::small::MAX_SMALL`] bytes comes from the calling thread's
//! small-object heap ([`crate::small`]); anything larger, or a request
//! the small heap cannot serve, comes from the system allocator. A free
//! goes back to whichever of the two the address belongs to.
//!
//! Accounting is per allocation, in the bytes requested: a block costs
//! its layout's size (a header block: its header, escape word and
//! payload), not the size class or page it sits in, so the counters and
//! the cap mean what they meant over the system allocator alone. A
//! header block's size is a multiple of 8, so a small one fills its size
//! class exactly and its free is accounted from its page's class. The
//! cumulative counters ([`Tally`]) are kept per thread, on its small
//! heap, with no atomic read-modify-write, and summed when read. The
//! live heap is one process-wide counter that threads reserve from in
//! chunks, so a thread allocating and freeing rarely writes it, while
//! the cap and the stats line's `peak` stay exact ([`Live`]).
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
//! runs to completion however much it allocates over its life. The
//! memory the process holds is the live heap plus the small heap's
//! partly used and retained pages. A real allocation failure (the system
//! allocator answering null) is the same `Exhausted` exit, never Rust's
//! `handle_alloc_error` abort, so the global allocator never answers
//! null.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering::Relaxed};
use std::sync::LazyLock;

use crate::small::{self, Heap};

/// The cap applied when `CINCH_HEAP_LIMIT` is unset: 8 GiB.
pub const DEFAULT_HEAP_LIMIT: u64 = 8 << 30;

/// The counters of one thread (on its small heap), or of the threads
/// that have none ([`UNHOMED`]). The stats line's values are the sums
/// over every tally.
pub(crate) struct Tally {
    blocks: AtomicU64,
    bytes: AtomicU64,
    header_blocks: AtomicU64,
    header_bytes: AtomicU64,
    immortal: AtomicU64,
    freed: AtomicU64,
    /// Bytes this thread has added to [`Live::reserved`] beyond its own
    /// net allocation, spent before it touches the shared counter again
    /// ([`charge`]). Always 0 in [`UNHOMED`].
    credit: AtomicU64,
}

impl Tally {
    const fn new() -> Self {
        Tally {
            blocks: AtomicU64::new(0),
            bytes: AtomicU64::new(0),
            header_blocks: AtomicU64::new(0),
            header_bytes: AtomicU64::new(0),
            immortal: AtomicU64::new(0),
            freed: AtomicU64::new(0),
            credit: AtomicU64::new(0),
        }
    }
}

/// The tally of allocations made by a thread without a small heap (one
/// setting its heap up, or past its teardown). Shared, so its counters
/// take atomic adds.
static UNHOMED: Tally = Tally::new();

/// The tally a thread counts into: its own heap's, written with plain
/// load-and-store pairs because only the heap's owner writes it, or the
/// shared [`UNHOMED`] one.
struct Counter<'a> {
    tally: &'a Tally,
    owned: bool,
}

impl<'a> Counter<'a> {
    #[inline]
    fn of(local: Option<&'a Heap>) -> Self {
        match local {
            Some(h) => Counter { tally: &h.tally, owned: true },
            None => Counter { tally: &UNHOMED, owned: false },
        }
    }

    #[inline]
    fn add(&self, field: fn(&Tally) -> &AtomicU64, n: u64) {
        let c = field(self.tally);
        if self.owned {
            c.store(c.load(Relaxed).wrapping_add(n), Relaxed);
        } else {
            c.fetch_add(n, Relaxed);
        }
    }
}

/// The sum of one counter over every tally.
fn total(field: fn(&Tally) -> &AtomicU64) -> u64 {
    small::heaps().fold(field(&UNHOMED).load(Relaxed), |n, h| n.wrapping_add(field(&h.tally).load(Relaxed)))
}

/// The process-wide live-heap counter and its high-water mark, on a cache
/// line of their own.
///
/// `reserved` is the live bytes plus every thread's unspent credit
/// ([`Tally::credit`]), so the live heap is `reserved` minus the credits
/// ([`live_now`]). A thread reserves [`CREDIT`] bytes at a time and
/// allocates from its credit with no shared write; a free adds to its
/// credit and hands back the excess over twice [`CREDIT`]. A thread that
/// only churns (allocates and frees as much) never touches `reserved`,
/// which would otherwise be one contended cache line under every
/// allocation of every task thread.
///
/// The credits never hide the cap. While `reserved` stays under it the
/// live heap does too. The first reservation that takes `reserved` past
/// the cap switches the process to [`EXACT`] mode, in which every
/// allocation and free changes `reserved` by its own size and the
/// credits left are frozen, so the check of the live heap against the
/// cap is exact from then on. `CINCH_ALLOC_STATS=1` runs in exact mode
/// from the start, so `peak` is the exact high-water mark of the live
/// heap; otherwise `peak` is the high-water mark of `reserved`, an upper
/// bound on it.
#[repr(align(128))]
struct Live {
    reserved: AtomicU64,
    peak: AtomicU64,
}

static LIVE: Live = Live {
    reserved: AtomicU64::new(0),
    peak: AtomicU64::new(0),
};

/// Bytes a thread reserves from [`Live::reserved`] at a time.
const CREDIT: u64 = 64 << 10;

/// [`MODE`] before the environment is read.
const UNREAD: u8 = 0;
/// Threads charge their credit ([`Live`]).
const BATCHED: u8 = 1;
/// Every allocation and free changes [`Live::reserved`].
const EXACT: u8 = 2;
static MODE: AtomicU8 = AtomicU8::new(UNREAD);

/// The cap as a bound `reserved` is compared with: `u64::MAX` when
/// uncapped, 0 before the environment is read.
static LIMIT: AtomicU64 = AtomicU64::new(0);

struct Config {
    /// 0 means uncapped.
    limit: u64,
    /// `CINCH_ALLOC_STATS=1`.
    stats: bool,
    /// `CINCH_RC_DEBUG=1`.
    rc_debug: bool,
    /// `CINCH_LEAK_CHECK=1`.
    leak_check: bool,
}

/// The environment, read through `crate::sys::getenv` so
/// initialisation never allocates (it runs inside the allocator).
static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let stats = env_bytes(c"CINCH_ALLOC_STATS") == Some(b"1");
    if stats {
        crate::sys::atexit(print_stats);
    }
    let limit = configured_limit(env_bytes(c"CINCH_HEAP_LIMIT"));
    Config {
        limit,
        stats,
        rc_debug: env_bytes(c"CINCH_RC_DEBUG") == Some(b"1"),
        leak_check: env_bytes(c"CINCH_LEAK_CHECK") == Some(b"1"),
    }
});

// Keep the default-setting rule testable without launching a generated
// program with its heap guard removed. Reading this setting does not
// change CONFIG or allocate memory.
fn configured_limit(setting: Option<&[u8]>) -> u64 {
    match setting {
        None => DEFAULT_HEAP_LIMIT,
        Some(value) => parse_limit(value).unwrap_or(DEFAULT_HEAP_LIMIT),
    }
}

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

/// An environment entry's bytes, or `None` when it is unset. The C
/// string is only the caller's spelling: the read itself is
/// `crate::sys::getenv`, and the answer borrows the environment block,
/// so it allocates nothing.
pub(crate) fn env_bytes(name: &std::ffi::CStr) -> Option<&'static [u8]> {
    crate::sys::getenv(name.to_bytes())
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

/// Add `size` bytes to the live heap, exiting `Exhausted` if that takes
/// it past the cap ([`Live`]). Once an `Exhausted` exit is under way the
/// cap no longer applies: the exit path (stderr writes, `atexit`
/// handlers) may still allocate and must not exit twice.
#[inline]
fn charge(local: Option<&Heap>, size: usize) {
    let size = size as u64;
    if let Some(h) = local {
        if MODE.load(Relaxed) == BATCHED {
            let credit = &h.tally.credit;
            let c = credit.load(Relaxed);
            if c >= size {
                credit.store(c - size, Relaxed);
            } else {
                reserve(credit, size - c);
            }
            return;
        }
    }
    charge_exact(size);
}

/// Reserve `need` bytes beyond a thread's spent credit, and a fresh
/// credit with them; past the cap, switch to exact mode and check.
#[cold]
fn reserve(credit: &AtomicU64, need: u64) {
    let add = need + CREDIT;
    let reserved = LIVE.reserved.fetch_add(add, Relaxed) + add;
    credit.store(CREDIT, Relaxed);
    raise_peak(reserved);
    if reserved > LIMIT.load(Relaxed) {
        // Unspent credit could now carry the live heap past the cap
        // unchecked: hand this thread's back and charge exactly from
        // here on.
        MODE.store(EXACT, Relaxed);
        credit.store(0, Relaxed);
        let reserved = LIVE.reserved.fetch_sub(CREDIT, Relaxed) - CREDIT;
        check_cap(reserved);
    }
}

/// [`charge`] in exact mode, or for a thread without a heap; the first
/// allocation of the process reads the environment here.
#[cold]
fn charge_exact(size: u64) {
    if MODE.load(Relaxed) == UNREAD {
        let config = &*CONFIG;
        LIMIT.store(if config.limit == 0 { u64::MAX } else { config.limit }, Relaxed);
        let mode = if config.stats { EXACT } else { BATCHED };
        let _ = MODE.compare_exchange(UNREAD, mode, Relaxed, Relaxed);
    }
    let reserved = LIVE.reserved.fetch_add(size, Relaxed) + size;
    raise_peak(reserved);
    if reserved > LIMIT.load(Relaxed) {
        check_cap(reserved);
    }
}

/// `reserved`, the reservation just made, passed the cap: exit
/// `Exhausted` if the live heap it holds did.
#[cold]
fn check_cap(reserved: u64) {
    let limit = CONFIG.limit;
    if limit == 0 || crate::abi::exhausting() {
        return;
    }
    let live = reserved.saturating_sub(credits());
    if live > limit {
        crate::abi::exhausted(crate::abi::Resource::Heap(live));
    }
}

#[inline]
fn raise_peak(v: u64) {
    // Most reservations leave the mark alone; reading it first keeps them
    // from writing its cache line.
    if v > LIVE.peak.load(Relaxed) {
        LIVE.peak.fetch_max(v, Relaxed);
    }
}

/// Every thread's unspent credit.
fn credits() -> u64 {
    small::heaps().fold(0, |n, h| n.wrapping_add(h.tally.credit.load(Relaxed)))
}

/// The live heap: reserved bytes less unspent credit.
fn live_now() -> u64 {
    LIVE.reserved.load(Relaxed).saturating_sub(credits())
}

/// Take `size` freed bytes off the live heap.
#[inline]
fn discharge(local: Option<&Heap>, size: usize) {
    let size = size as u64;
    if let Some(h) = local {
        if MODE.load(Relaxed) == BATCHED {
            let credit = &h.tally.credit;
            let c = credit.load(Relaxed) + size;
            if c <= 2 * CREDIT {
                credit.store(c, Relaxed);
            } else {
                credit.store(CREDIT, Relaxed);
                LIVE.reserved.fetch_sub(c - CREDIT, Relaxed);
            }
            return;
        }
    }
    LIVE.reserved.fetch_sub(size, Relaxed);
}

/// Hand the unspent credit of `tally`, a heap whose thread is exiting,
/// back to the live-heap counter.
pub(crate) fn release_credit(tally: &Tally) {
    let c = tally.credit.load(Relaxed);
    if c != 0 {
        tally.credit.store(0, Relaxed);
        LIVE.reserved.fetch_sub(c, Relaxed);
    }
}

/// Count one allocation of `size` bytes (and, when `header` holds
/// whether it is immortal, one header block), then add it to the live
/// heap: the single accounting point for every heap allocation.
#[inline]
fn account(local: Option<&Heap>, size: usize, header: Option<bool>) {
    let count = Counter::of(local);
    count.add(|t| &t.blocks, 1);
    count.add(|t| &t.bytes, size as u64);
    if let Some(immortal) = header {
        count.add(|t| &t.header_blocks, 1);
        count.add(|t| &t.header_bytes, size as u64);
        if immortal {
            count.add(|t| &t.immortal, 1);
        }
    }
    charge(local, size);
}

/// Memory for `layout`, already accounted: a small block from `local`
/// when the request is small, otherwise (or when no page can be had)
/// from the system allocator. Never null.
#[inline(always)]
unsafe fn place(local: Option<&Heap>, layout: Layout, zeroed: bool) -> *mut u8 {
    if let (Some(heap), Some(class)) = (local, small::class_of(layout.size(), layout.align())) {
        let p = unsafe { heap.alloc(class) };
        if !p.is_null() {
            if zeroed {
                unsafe { p.write_bytes(0, layout.size()) };
            }
            return p;
        }
    }
    unsafe { place_system(layout, zeroed) }
}

/// [`place`] from the system allocator.
#[inline(never)]
unsafe fn place_system(layout: Layout, zeroed: bool) -> *mut u8 {
    granted(unsafe {
        if zeroed {
            System.alloc_zeroed(layout)
        } else {
            System.alloc(layout)
        }
    })
}

/// Allocate and account the backing memory of a header block of
/// `layout` (8-byte aligned, a multiple of 8 bytes). It counts as one
/// allocation and one header block, immortal or not. Allocation failure
/// is the heap `Exhausted` exit, so the answer is never null.
///
/// # Safety
///
/// The caller writes every byte it later reads (unless `zeroed`).
#[inline]
pub(crate) unsafe fn alloc_block(layout: Layout, immortal: bool, zeroed: bool) -> *mut u8 {
    let local = small::local();
    account(local, layout.size(), Some(immortal));
    unsafe { place(local, layout, zeroed) }
}

/// Free the backing memory at `base` of a header block from
/// [`alloc_block`], counting the header block freed. `local` is the
/// calling thread's small heap ([`small::local`]), or `None`. `layout`
/// gives the block's allocation layout; it is only asked for when the
/// block came from the system allocator, since a small block's page
/// knows its size.
///
/// # Safety
///
/// `base` is the start of a live block [`alloc_block`] returned, not
/// used again.
#[inline(always)]
pub(crate) unsafe fn free_block(local: Option<&Heap>, base: *mut u8, layout: impl FnOnce() -> Layout) {
    count_freed(local);
    if small::owns(base) {
        discharge(local, unsafe { small::block_size(base) });
        unsafe { small::free(base, local) }
    } else {
        unsafe { free_system(local, base, layout()) }
    }
}

/// [`free_block`] of a block the system allocator holds.
#[inline(never)]
unsafe fn free_system(local: Option<&Heap>, base: *mut u8, layout: Layout) {
    discharge(local, layout.size());
    unsafe { System.dealloc(base, layout) }
}

/// The system allocator answered null for an allocation already counted
/// by [`account`]: the `Exhausted` exit, reporting the live bytes the
/// program asked to hold. A failure on the exit path itself leaves at
/// once with the same code.
fn refused() -> ! {
    if crate::abi::exhausting() {
        crate::abi::exhausted_again();
    }
    crate::abi::exhausted(crate::abi::Resource::Heap(live_now()))
}

/// A request of `size` bytes that no allocation can satisfy (its size
/// does not fit an allocation layout): the same `Exhausted` exit, with
/// the live bytes it would have brought the heap to.
pub(crate) fn request_too_large(size: usize) -> ! {
    let n = live_now().saturating_add(size as u64);
    crate::abi::exhausted(crate::abi::Resource::Heap(n))
}

/// Record that a header block was freed without returning its memory (a
/// debug-mode quarantine); [`free_block`] counts the others.
pub fn account_block_free() {
    count_freed(small::local());
}

#[inline]
fn count_freed(local: Option<&Heap>) {
    Counter::of(local).add(|t| &t.freed, 1);
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
        total(|t| &t.header_blocks),
        total(|t| &t.header_bytes),
        total(|t| &t.immortal),
        total(|t| &t.freed),
    )
}

/// Header blocks allocated and not freed, immortal ones included.
pub fn live_blocks() -> u64 {
    let (hblocks, _, _, freed) = block_stats();
    hblocks.saturating_sub(freed)
}

/// Snapshot `(blocks, bytes, current, peak)`: `current` is the live heap,
/// `peak` its high-water mark as [`Live`] describes (exact under
/// `CINCH_ALLOC_STATS=1`, an upper bound otherwise).
pub fn stats() -> (u64, u64, u64, u64) {
    (
        total(|t| &t.blocks),
        total(|t| &t.bytes),
        live_now(),
        LIVE.peak.load(Relaxed),
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
            let (n, _) = crate::sys::write(2, &self.buf[off..self.len]);
            if n <= 0 {
                return;
            }
            off += n as usize;
        }
    }
}

/// The counting global allocator: [`account`] plus the placement shared
/// with header blocks ([`place`]). It never answers null: a refused
/// allocation is the heap `Exhausted` exit ([`refused`]).
pub struct Counting;

/// `p`, or the `Exhausted` exit when the system allocator refused.
#[inline]
fn granted(p: *mut u8) -> *mut u8 {
    if p.is_null() {
        refused();
    }
    p
}

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let local = small::local();
        account(local, layout.size(), None);
        unsafe { place(local, layout, false) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let local = small::local();
        account(local, layout.size(), None);
        unsafe { place(local, layout, true) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let local = small::local();
        discharge(local, layout.size());
        if small::owns(ptr) {
            unsafe { small::free(ptr, local) }
        } else {
            unsafe { System.dealloc(ptr, layout) }
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let local = small::local();
        discharge(local, layout.size());
        account(local, new_size, None);
        if !small::owns(ptr) {
            return granted(unsafe { System.realloc(ptr, layout, new_size) });
        }
        // A small block stays put while its class still fits; otherwise
        // it moves to wherever the new size is placed.
        if small::class_of(new_size, layout.align()) == Some(unsafe { small::class_at(ptr) }) {
            return ptr;
        }
        let new = unsafe { place(local, Layout::from_size_align_unchecked(new_size, layout.align()), false) };
        unsafe {
            std::ptr::copy_nonoverlapping(ptr, new, layout.size().min(new_size));
            small::free(ptr, local);
        }
        new
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_and_invalid_limits_use_the_documented_default() {
        assert_eq!(configured_limit(None), 8 << 30);
        assert_eq!(configured_limit(Some(b"invalid")), 8 << 30);
        assert_eq!(configured_limit(Some(b"256M")), 256 << 20);
    }

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
