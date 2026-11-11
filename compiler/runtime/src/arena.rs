//! A per-thread bump arena for constructor cells.
//!
//! A phase of a batch compiler allocates traffic that dies with the phase:
//! constructor cells (`KIND_CTOR`) and closure or dictionary blocks. This
//! module serves those blocks out of chunked bump storage and frees the
//! storage wholesale, with no reference count of any kind on any block it
//! serves.
//!
//! # The mechanism
//!
//! [`cinch_arena_new`] creates an arena, enters it on the calling thread,
//! and returns it as an opaque handle word. While an arena is entered,
//! [`crate::block::allocate_block`] serves `KIND_CTOR`, `KIND_CLOSURE` and
//! `KIND_DICT` blocks from the arena's bump pointer instead of from the
//! size-class allocator. [`cinch_arena_leave`] frees every chunk at once
//! and restores the arena that was entered before it, so arenas nest.
//!
//! An arena is a chain of chunks. Each chunk is one
//! [`crate::heap::alloc_block`] allocation: a [`Chunk`] header (the next
//! chunk, the data area's size) followed by the 8-aligned data area. A
//! block is placed at the bump pointer: its escape word when it has one,
//! then its header, then its payload. The bump pointer moves past the
//! whole grant, so the payload order in a chunk is the allocation order.
//! Chunks are linked to the arena, not into each other's data, so the
//! allocator hands them back in any order at leave.
//!
//! # What a block served from an arena is
//!
//! A full header block with its own kind and layout, so a `case` reads
//! its tag and a projection reads its fields exactly as it reads a heap
//! block. Its header's `rc` is [`RC_IMMORTAL`] (the value a static block
//! carries), which is what makes the whole design work without touching a
//! count path:
//!
//! * `cinch_dup` returns without writing (its immortal arm), and so does
//!   the inlined dup helper of emitted code;
//! * `cinch_drop` returns without releasing (its immortal arm), and the
//!   inlined drop helper's `switch` sends the count `0` straight to its
//!   done label, so `cinch_drop_slow` never sees an arena block;
//! * the release walk and `cinch_frame_release` stop at it, so an outer
//!   block that dies during the phase does not walk into the arena;
//! * mark-on-spawn stops at it, so reading one from another task counts
//!   nothing.
//!
//! A block in an arena may hold words that point at outer heap blocks
//! (they keep their own counts and are freed normally), and its payload
//! is never scanned or released by this module. That is why the arena
//! serves only the kinds whose payload no runtime destructor owns: a
//! boxed `Nat`/`Int` (its `Vec<u32>` limbs), a `Bytes` (its data buffer)
//! and a `Sym` (already immortal) hold Rust-side resources that only the
//! counted free path drops, so they always come from the heap.
//!
//! # The invariant this module cannot check
//!
//! No word that points into an arena's chunks may be reachable after
//! `cinch_arena_leave`: the chunks are gone. The caller's contract is the
//! scope and nothing else: what the scope returns is a machine scalar or
//! a value of a kind the arena does not serve, and everything the arena
//! served is dead when the scope ends. Two consequences the caller owns:
//!
//! * a value served from an arena must not be stored in a structure that
//!   outlives the scope;
//! * a value served from an arena must not cross a task spawn: a child
//!   thread has its own heap and no arena, so it neither allocates into
//!   the arena nor keeps its chunks alive.
//!
//! # Environment
//!
//! The mechanism is off unless a program asks for it: `CINCH_ARENA=1`
//! makes [`cinch_arena_new`] enter a real arena, and unset (or any other
//! value) makes it return the null handle, so the program's arena scopes
//! are no-ops and every block comes from the counted heap. Off is the
//! default because a new allocation strategy should be asked for, and
//! because it is what makes "the mechanism is present but unused" a
//! measurable arm of the same build.
//!
//! # Statistics
//!
//! [`stats`] returns the blocks and bytes served from arenas since the
//! process started and the chunks and chunk bytes live now. An arena adds
//! its own totals at leave, so the counters are one atomic add per arena,
//! never one per block.

use std::alloc::Layout;
use std::cell::Cell;
use std::sync::atomic::{AtomicU64, AtomicU8, AtomicUsize, Ordering::Relaxed};

use crate::block::RC_IMMORTAL;
use crate::small;

/// The chunk header: the next chunk of the same arena, and the data
/// area's size in bytes. The data area follows at [`CHUNK_HEADER`].
struct Chunk {
    next: *mut Chunk,
    bytes: usize,
}

/// The bytes the chunk header takes before its data area. Two words, so
/// the data area is 8-aligned like the chunk allocation itself.
const CHUNK_HEADER: usize = 16;

/// The smallest chunk, and the size the first chunk starts from when the
/// caller asks for less: 64 KiB of constructor cells.
const MIN_CHUNK: usize = 64 << 10;

/// The largest chunk a doubling step asks for. A phase whose live set is
/// larger grows another chunk; the cap only keeps one request from
/// overshooting by a factor of two for no reason.
const MAX_CHUNK: usize = 64 << 20;

/// One entered arena. The record lives in one
/// [`crate::heap::alloc_block`] allocation, so the heap cap and the stats
/// line account for it, and it is freed by [`cinch_arena_leave`].
pub struct Arena {
    /// The arena that was current before this one, restored at leave.
    prev: *mut Arena,
    /// The newest chunk, the one teardown walks from.
    head: *mut Chunk,
    /// The chunk the bump pointer is in.
    current: *mut Chunk,
    /// Bytes of `current`'s data area handed out.
    used: usize,
    /// The data area size to ask for when a chunk must grow.
    next: usize,
    /// Blocks served, folded into [`stats`] at leave.
    blocks: u64,
    /// Bytes granted, prefix and payload, folded into [`stats`] at leave.
    bytes: u64,
    /// The payload pointer of every served block, in allocation order.
    /// Teardown walks this list and drops each block's counted children:
    /// a cell may hold a heap `Bytes` or a boxed `Nat` that only this
    /// reference kept alive, and those are the counted heap's to free.
    served: Vec<*mut u8>,
}

const _: () = assert!(CHUNK_HEADER % 8 == 0, "the data area keeps the allocation's alignment");
const _: () = assert!(MIN_CHUNK <= MAX_CHUNK);
const _: () = assert!(std::mem::size_of::<Arena>() % 8 == 0, "the record is a whole number of words");

/// Cumulative blocks and bytes served from arenas, added once per arena
/// at leave.
static SERVED_BLOCKS: AtomicU64 = AtomicU64::new(0);
static SERVED_BYTES: AtomicU64 = AtomicU64::new(0);
/// Chunks, and the bytes of chunk headers, data areas and arena records,
/// live now. A test sees the storage return to its baseline.
static LIVE_CHUNKS: AtomicU64 = AtomicU64::new(0);
static LIVE_CHUNK_BYTES: AtomicU64 = AtomicU64::new(0);

/// [`ENABLED`] before `CINCH_ARENA` is read.
const UNREAD: u8 = 0;
const ON: u8 = 1;
const OFF: u8 = 2;
static ENABLED: AtomicU8 = AtomicU8::new(UNREAD);

/// Whether arena entry allocates anything. `CINCH_ARENA=1` says yes and
/// the default says no; the read happens on the first
/// [`cinch_arena_new`] and allocates nothing.
fn enabled() -> bool {
    match ENABLED.load(Relaxed) {
        ON => true,
        OFF => false,
        _ => {
            let on = crate::heap::env_bytes(c"CINCH_ARENA") == Some(b"1");
            ENABLED.store(if on { ON } else { OFF }, Relaxed);
            on
        }
    }
}

/// A snapshot of the arena counters: `(blocks, bytes, live_chunks,
/// live_chunk_bytes)`. `blocks` and `bytes` are cumulative over every
/// arena that has been left; the chunk pair is live now.
pub fn stats() -> (u64, u64, u64, u64) {
    (
        SERVED_BLOCKS.load(Relaxed),
        SERVED_BYTES.load(Relaxed),
        LIVE_CHUNKS.load(Relaxed),
        LIVE_CHUNK_BYTES.load(Relaxed),
    )
}

/// The arena the calling thread has entered, or null.
///
/// The pointer is a thread-local, not a field of the thread's small heap:
/// a heap moves between threads (a thread parks it at exit and a later
/// thread adopts it, [`crate::small`]), so a heap field would carry one
/// thread's arena into another thread's allocations.
///
/// [`ENTERED`] is the fast path: while no thread in the process has an
/// arena (the default, and the disabled arm), the check is
/// one relaxed load of a word no thread writes. Once any arena is
/// entered, every allocation pays the thread-local read and compares the
/// pointed-to arena, which is the mechanism's cost while it operates.
#[inline(always)]
pub(crate) fn current() -> *mut Arena {
    if ENTERED.load(Relaxed) == 0 {
        return std::ptr::null_mut();
    }
    CURRENT.with(Cell::get)
}

thread_local! {
    /// The arena the context this thread is running has entered, or
    /// null. The scheduler exchanges it at every switch
    /// ([`replace_current`]), so a task that entered an arena finds it
    /// on whichever thread resumes it, and no other task allocates from
    /// it.
    static CURRENT: Cell<*mut Arena> = const { Cell::new(std::ptr::null_mut()) };
}

/// Make `arena` the calling thread's current arena and answer the one it
/// replaces, without entering or leaving either. [`enter`] is this plus
/// the count of entered arenas; the scheduler uses it bare at a switch,
/// because a task's arena stays entered while the task is switched out
/// ([`ENTERED`] counts the arenas that are entered, running or not).
#[inline]
pub(crate) fn replace_current(arena: *mut Arena) -> *mut Arena {
    CURRENT.with(|c| c.replace(arena))
}

/// The arenas entered across the process, so a thread can skip the
/// thread-local read while no thread has one.
static ENTERED: AtomicUsize = AtomicUsize::new(0);

/// Make `arena` the calling thread's current arena (null leaves), and
/// keep [`ENTERED`] exact.
#[inline]
fn enter(arena: *mut Arena) {
    let previous = replace_current(arena);
    match (previous.is_null(), arena.is_null()) {
        (true, false) => {
            ENTERED.fetch_add(1, Relaxed);
        }
        (false, true) => {
            ENTERED.fetch_sub(1, Relaxed);
        }
        _ => {}
    }
}

/// The 8-aligned data area of `chunk`.
#[inline]
unsafe fn data(chunk: *mut Chunk) -> *mut u8 {
    unsafe { chunk.cast::<u8>().add(CHUNK_HEADER) }
}

/// The allocation layout of a chunk whose data area is `bytes` bytes.
fn chunk_layout(bytes: usize) -> Option<Layout> {
    Layout::from_size_align(CHUNK_HEADER.checked_add(bytes)?, 8).ok()
}

/// Allocate and link one chunk of at least `need` bytes, and make it the
/// current chunk. An allocation the heap cap refuses is the heap
/// `Exhausted` exit inside [`crate::heap::alloc_block`], never a null
/// answer.
unsafe fn grow(arena: &mut Arena, need: usize) {
    let wanted = arena.next.max(need).max(MIN_CHUNK).next_multiple_of(8);
    let Some(layout) = chunk_layout(wanted) else {
        crate::heap::request_too_large(wanted);
    };
    let chunk = unsafe { crate::heap::alloc_block(layout, false, false) }.cast::<Chunk>();
    unsafe { chunk.write(Chunk { next: arena.head, bytes: wanted }) };
    arena.head = chunk;
    arena.current = chunk;
    arena.used = 0;
    arena.next = wanted.saturating_mul(2).min(MAX_CHUNK);
    LIVE_CHUNKS.fetch_add(1, Relaxed);
    LIVE_CHUNK_BYTES.fetch_add(layout.size() as u64, Relaxed);
}

/// Bump-allocate `total` bytes (a block's prefix and payload) in the
/// calling thread's arena, record the served block, and return its payload
/// pointer (the grant's base plus `prefix`), or `None` when the thread has
/// no arena entered. The caller writes the escape word and the header into
/// the grant.
///
/// # Safety
///
/// The caller writes the grant's first bytes before any read of them and
/// never writes past `total`; `total` is the block's whole allocation, and
/// it initialises every payload word it later reads.
#[inline]
pub(crate) unsafe fn serve(total: usize, prefix: usize) -> Option<*mut u8> {
    let arena = current();
    if arena.is_null() {
        return None;
    }
    let arena = unsafe { &mut *arena };
    let total = total.max(8).next_multiple_of(8);
    if arena.current.is_null() || arena.used + total > unsafe { (*arena.current).bytes } {
        unsafe { grow(arena, total) };
    }
    let base = unsafe { data(arena.current).add(arena.used) };
    arena.used += total;
    arena.blocks += 1;
    arena.bytes += total as u64;
    let payload = unsafe { base.add(prefix) };
    arena.served.push(payload);
    Some(payload)
}

/// Create an arena whose first chunk is at least `bytes` bytes, enter it
/// on the calling thread, and return it.
///
/// # Safety
///
/// The caller leaves it on the same thread and frees no word served from
/// it after that (this module's doc comment).
pub unsafe fn new(bytes: usize) -> *mut Arena {
    let arena = unsafe { crate::heap::alloc_block(Layout::new::<Arena>(), false, true) }.cast::<Arena>();
    LIVE_CHUNKS.fetch_add(1, Relaxed);
    LIVE_CHUNK_BYTES.fetch_add(Layout::new::<Arena>().size() as u64, Relaxed);
    let target = bytes.max(MIN_CHUNK).next_multiple_of(8).min(MAX_CHUNK);
    unsafe {
        arena.write(Arena {
            prev: current(),
            head: std::ptr::null_mut(),
            current: std::ptr::null_mut(),
            used: 0,
            next: target,
            blocks: 0,
            bytes: 0,
            served: Vec::new(),
        });
        grow(&mut *arena, target);
    }
    enter(arena);
    arena
}

/// Free every chunk of `arena`, fold its totals into [`stats`], restore
/// the arena entered before it, and free the record.
///
/// # Safety
///
/// `arena` is the calling thread's current arena, from [`new`], and no
/// word it served is reachable afterwards.
pub unsafe fn leave(arena: *mut Arena) {
    let local = small::local();
    let record = unsafe { &*arena };
    // Every word an arena block owns first: a served cell's fields are
    // ordinary words, and the heap blocks among them (a rendered `Bytes`,
    // a boxed `Nat`) were counted when they were built. Only an owner's
    // drop frees one, and the arena is the owner, so teardown returns them
    // before the memory they point into is gone. A child that is itself an
    // arena block answers nothing.
    let debug = crate::heap::rc_debug();
    for &served in &record.served {
        let h = unsafe { crate::block::read_header(served) };
        unsafe { crate::rc::release_children(served, h, debug) };
    }
    let mut chunk = record.head;
    while !chunk.is_null() {
        let next = unsafe { (*chunk).next };
        let bytes = unsafe { (*chunk).bytes };
        let layout = chunk_layout(bytes).expect("a chunk size that was allocated");
        unsafe { crate::heap::free_block(local, chunk.cast(), || layout) };
        LIVE_CHUNKS.fetch_sub(1, Relaxed);
        LIVE_CHUNK_BYTES.fetch_sub(layout.size() as u64, Relaxed);
        chunk = next;
    }
    SERVED_BLOCKS.fetch_add(record.blocks, Relaxed);
    SERVED_BYTES.fetch_add(record.bytes, Relaxed);
    let prev = record.prev;
    unsafe { std::ptr::drop_in_place(arena) };
    unsafe { crate::heap::free_block(local, arena.cast(), || Layout::new::<Arena>()) };
    LIVE_CHUNKS.fetch_sub(1, Relaxed);
    LIVE_CHUNK_BYTES.fetch_sub(Layout::new::<Arena>().size() as u64, Relaxed);
    enter(prev);
}

/// Enter a fresh arena whose first chunk is at least `bytes` bytes and
/// return its handle, or 0 unless `CINCH_ARENA=1` (every scope is then a
/// no-op and every block a heap block). A negative size is a
/// compiler-bug trap.
///
/// The handle is a machine word, never a block: generated code holds it
/// in an `I64` and emits no count operation for it.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_arena_new(bytes: i64) -> i64 {
    if bytes < 0 {
        crate::abi::panic_exit(b"compiler bug: cinch_arena_new called with a negative size");
    }
    if !enabled() {
        return 0;
    }
    unsafe { new(bytes as usize) as usize as i64 }
}

/// Leave and free the arena `handle` names, restoring the arena entered
/// before it, and return 0. The null handle (a disabled arena) is a
/// no-op. Leaving an arena that is not the current one is a compiler bug:
/// nesting is a stack discipline.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_arena_leave(handle: i64) -> i64 {
    if handle == 0 {
        return 0;
    }
    let arena = handle as usize as *mut Arena;
    if current() != arena {
        crate::abi::panic_exit(b"compiler bug: an arena was left out of order");
    }
    unsafe { leave(arena) };
    0
}

/// The header a served block gets: its own kind and layout, and
/// [`RC_IMMORTAL`] as its count ([`crate::block::allocate_block`] writes
/// it into the grant).
pub(crate) const fn arena_header(kind: u8, layout: u16) -> u64 {
    crate::block::header(RC_IMMORTAL, kind, layout)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::test_shapes::CONS_T;
    use crate::block::{allocate_ctor, ctor_tag_of, header_kind, header_layout, read_header, KIND_CTOR};

    /// The arena counters are process-wide, so a test that asserts their
    /// baseline must not run beside another arena.
    static ARENA_TESTS: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn an_arena_serves_constructor_cells_from_its_chunks() {
        let _heavy = crate::heap::TEST_HEAVY.lock().unwrap_or_else(|e| e.into_inner());
        let _alone = ARENA_TESTS.lock().unwrap_or_else(|e| e.into_inner());
        let (header0, immortal0) = crate::heap::thread_header_blocks();
        let freed0 = crate::heap::thread_freed();
        let (blocks0, bytes0, chunks0, chunk_bytes0) = stats();
        crate::block::install_test_layouts();
        const N: usize = 4096;
        // `Cons` over an `Obj` head: two counted words, so teardown's scan
        // has a child to find.
        let desc = CONS_T;
        // A heap cell built before the arena. The first served cell below
        // owns it, and only the arena's teardown can return it.
        let child = unsafe { allocate_ctor(16, desc, false) }.cast::<i64>();
        unsafe { child.write(0) };
        unsafe { child.add(1).write(0) };
        let arena = unsafe { new(0) };
        assert_eq!(current(), arena, "new enters the arena on this thread");
        let mut words = Vec::with_capacity(N);
        for i in 0..N {
            let p = unsafe { allocate_ctor(16, desc, false) }.cast::<i64>();
            unsafe { p.write((i as i64) << 1 | 1) };
            unsafe { p.add(1).write(if i == 0 { child as usize as i64 } else { 0 }) };
            let h = unsafe { read_header(p.cast()) };
            assert_eq!(crate::block::rc(h), RC_IMMORTAL, "an arena block is uncounted");
            assert_eq!(header_kind(h), KIND_CTOR);
            assert_eq!(header_layout(h), crate::block::ctor_layout_field(desc));
            assert_eq!(unsafe { ctor_tag_of(p.cast()) }, desc & crate::block::CTOR_TAG_MASK);
            // dup and drop write nothing: the header word is unchanged.
            unsafe { crate::rc::cinch_dup(p as usize as i64) };
            assert_eq!(unsafe { read_header(p.cast()) }, h, "a dup writes no count");
            unsafe { crate::rc::cinch_drop(p as usize as i64) };
            assert_eq!(unsafe { read_header(p.cast()) }, h, "a drop is a no-op");
            words.push(p as usize as i64);
        }
        let (header1, immortal1) = crate::heap::thread_header_blocks();
        assert_eq!(immortal1, immortal0, "an arena block is never born on the heap");
        let (_, _, chunks1, chunk_bytes1) = stats();
        let chunk_delta = chunks1 - chunks0;
        assert!(chunk_delta >= 2, "the arena grew chunks");
        assert!(chunk_bytes1 > chunk_bytes0, "the chunks are live storage");
        assert_eq!(
            header1 - header0,
            chunk_delta + 1,
            "the general allocator sees the record, the chunks and the one child cell, not the {N} served cells"
        );
        let before_leave = crate::heap::thread_freed();
        unsafe { leave(arena) };
        assert_eq!(
            crate::heap::thread_freed() - before_leave,
            chunk_delta + 1,
            "leave frees the chunks, the record, and the heap cell a served cell owned"
        );
        assert!(current().is_null(), "leave restores the previous (none) arena");
        let (blocks2, bytes2, chunks2, chunk_bytes2) = stats();
        assert_eq!(blocks2 - blocks0, N as u64, "every cell counts once in the arena");
        assert!(bytes2 - bytes0 >= N as u64 * 24, "a cell costs its header and payload");
        assert_eq!(chunks2, chunks0, "every chunk is back");
        assert_eq!(chunk_bytes2, chunk_bytes0, "the chunk storage returns to its baseline");
        assert_eq!(
            crate::heap::thread_freed() - freed0,
            chunk_delta + 1,
            "the whole teardown is the record, its chunks, and one released child"
        );
        assert!(words.iter().all(|w| *w != 0));
    }

    #[test]
    fn arenas_nest() {
        let _alone = ARENA_TESTS.lock().unwrap_or_else(|e| e.into_inner());
        let outer = unsafe { new(4096) };
        let inner = unsafe { new(4096) };
        assert_eq!(current(), inner);
        unsafe { leave(inner) };
        assert_eq!(current(), outer, "leaving the inner one restores the outer one");
        unsafe { leave(outer) };
        assert!(current().is_null());
    }
}
