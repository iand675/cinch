//! The small-object allocator: thread-owned pages of segregated size
//! classes, the backing store of every allocation of at most
//! [`MAX_SMALL`] bytes. [`crate::heap`] routes requests here and keeps
//! the accounting; this module only places and recycles memory.
//!
//! # Size classes and pages
//!
//! A request is rounded up to whole 8-byte words, so the [`CLASSES`]
//! classes are 8, 16, 24, ..., 256 bytes. A request for 16-byte
//! alignment is rounded to a multiple of 16 first, and every block of a
//! class whose size is a multiple of 16 is 16-aligned. A Cinch value
//! block (a header word plus whole payload words) therefore lands in the
//! class of exactly its own size. Larger requests and alignments above
//! 16 bytes go to the system allocator.
//!
//! Blocks live in pages of [`PAGE`] bytes, each aligned to its size, so
//! the page of a block is its address with the low bits cleared. A page
//! serves one class at a time. Its first [`PAGE_HEADER`] bytes hold the
//! class, the owning heap, a free list and a bump pointer; the rest is
//! blocks. A fresh page is carved by the bump pointer as blocks are
//! asked for, so its untouched tail never becomes resident.
//!
//! Pages come from one address range reserved, without access, on the
//! first allocation and committed [`COMMIT_CHUNK`] bytes at a time.
//! Whether a pointer belongs to this allocator is a test against that
//! range, so a request that fell back to the system allocator (the range
//! used up, or a thread already past its teardown) is freed back to the
//! system whatever its size. Pages are never returned to the operating
//! system: a page with no live block is cached by its heap (up to
//! [`CACHED_PAGES`]) or put in a shared pool, and reused for any class
//! by any thread.
//!
//! # Thread heaps
//!
//! Every thread allocates from its own [`Heap`]. Per class, the heap
//! keeps a list of the pages that have room; the head is the page the
//! fast path serves, popping its free list or bumping. A page with no
//! room left leaves the list, and comes back when one of its blocks is
//! freed. Frees by the owning thread push onto the page's free list:
//! neither path takes a lock or an atomic read-modify-write.
//!
//! # Frees from other threads
//!
//! Cinch blocks cross threads (a task's result moves to its joiner, and
//! a block shared across tasks dies on whichever thread drops it last).
//! A block freed by a thread other than its page's owner is pushed onto
//! the owning heap's remote list with one compare-and-swap. The list is
//! a stack with many pushers and one consumer that takes the whole list
//! at once, which is free of ABA. The owner drains it before taking a
//! new page, freeing each block as its own. A block's page cannot change
//! owner under a remote free: a page is recycled only once every block
//! in it has been freed and drained.
//!
//! A heap outlives its thread. At thread exit the heap drains what it
//! can, gives its empty pages to the pool, and is parked; the next thread
//! to start adopts a parked heap before making a new one, so there are
//! never more heaps than the peak number of live threads. Remote frees
//! keep arriving at a parked heap (a joiner freeing its task's result),
//! and a thread about to grow the range first collects them: it takes
//! the parked heap off the parked list, drains it as its temporary owner,
//! and pools the pages that emptied.

use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::{Cell, UnsafeCell};
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicUsize, Ordering::{Acquire, Relaxed, Release}};

/// The largest request served here; larger ones are the system's.
pub(crate) const MAX_SMALL: usize = 256;
/// The size classes: every multiple of 8 up to [`MAX_SMALL`].
const CLASSES: usize = MAX_SMALL / 8;
/// Bytes per page, and the alignment of every page.
pub(crate) const PAGE: usize = 64 << 10;
/// Bytes at the start of a page reserved for its [`Page`] header: one
/// 128-byte cache line, and a multiple of 16 so 16-byte classes stay
/// aligned.
const PAGE_HEADER: usize = 128;
/// Bytes of the reserved range made accessible at a time.
const COMMIT_CHUNK: usize = 4 << 20;
/// The range sizes tried at reservation, largest first. The largest is
/// eight times the default heap cap; past the range, small requests fall
/// back to the system allocator.
const RESERVATIONS: [usize; 3] = [64 << 30, 16 << 30, 4 << 30];
/// Empty pages a heap keeps for itself before pooling the rest.
const CACHED_PAGES: usize = 8;

const PROT_READ: i32 = 1;
const PROT_WRITE: i32 = 2;

const _: () = assert!(std::mem::size_of::<Page>() <= PAGE_HEADER);
const _: () = assert!(PAGE.is_power_of_two() && COMMIT_CHUNK.is_multiple_of(PAGE));

/// A free block: its first word links to the next free block.
struct Free {
    next: *mut Free,
}

/// A page header. Every field but `heap` and `size` belongs to the
/// owning heap's thread; another thread reads `heap` and `size` only for
/// a block it holds, which keeps the page from being recycled meanwhile.
#[repr(C)]
struct Page {
    /// Freed blocks, most recent first.
    free: *mut Free,
    /// The next block never handed out.
    bump: *mut u8,
    /// The end of the last whole block.
    end: *mut u8,
    /// Blocks handed out and not yet freed back to this page.
    used: u32,
    /// The block size of the class.
    size: u32,
    class: u8,
    /// Off its class list because it had no room.
    full: bool,
    /// The heap that owns the page.
    heap: AtomicPtr<Heap>,
    /// The class list links, or the cache link of an empty page.
    prev: *mut Page,
    next: *mut Page,
}

/// The page every empty class list points at: no free block and no bump
/// room, so the fast path falls through to [`Heap::refill`] without a
/// separate test. It is never written.
struct Sentinel(UnsafeCell<Page>);

// SAFETY: the sentinel is only ever read.
unsafe impl Sync for Sentinel {}

static EMPTY: Sentinel = Sentinel(UnsafeCell::new(Page {
    free: null_mut(),
    bump: null_mut(),
    end: null_mut(),
    used: 0,
    size: 0,
    class: 0,
    full: false,
    heap: AtomicPtr::new(null_mut()),
    prev: null_mut(),
    next: null_mut(),
}));

#[inline]
fn empty() -> *mut Page {
    EMPTY.0.get()
}

/// The page of a block this allocator owns.
#[inline]
fn page_of(p: *const u8) -> *mut Page {
    (p as usize & !(PAGE - 1)) as *mut Page
}

/// The remote-free list on a cache line of its own, so pushes from other
/// threads do not disturb the owner's fast path.
#[repr(align(128))]
struct Remote(AtomicPtr<Free>);

/// One thread's allocator state ([module docs](self)). The `Cell` fields
/// belong to the thread that owns the heap: its current thread, or the
/// thread collecting it while it is parked. `remote` and `tally` are the
/// only fields other threads touch.
#[repr(C)]
pub(crate) struct Heap {
    /// Per class, the head of the list of pages with room ([`empty`] when
    /// there are none).
    pages: [Cell<*mut Page>; CLASSES],
    /// Empty pages kept for reuse, linked through `next`.
    cache: Cell<*mut Page>,
    cached: Cell<usize>,
    /// The next parked heap, while this one is parked ([`SHARED`]'s lock).
    parked_next: Cell<*mut Heap>,
    /// The next heap ever made, immutable once published in [`HEAPS`].
    all_next: *mut Heap,
    /// The accounting counters of the thread that owns the heap.
    pub(crate) tally: crate::heap::Tally,
    /// Blocks of this heap's pages freed by other threads.
    remote: Remote,
}

// SAFETY: the `Cell` fields are only touched by the heap's one owner at a
// time (module docs); ownership moves under the `SHARED` lock.
unsafe impl Sync for Heap {}

/// The class of a `size`-byte request aligned to `align`, or `None` when
/// the system allocator serves it.
#[inline]
pub(crate) fn class_of(size: usize, align: usize) -> Option<usize> {
    if align > 16 {
        return None;
    }
    let size = if align > 8 { size.checked_add(15)? & !15 } else { size };
    if size > MAX_SMALL {
        return None;
    }
    Some(size.max(1).div_ceil(8) - 1)
}

/// Whether `p` is a block of this allocator.
#[inline]
pub(crate) fn owns(p: *const u8) -> bool {
    // A block reaches another thread only through a synchronisation that
    // also publishes the range, so relaxed loads see it; before the range
    // exists both are 0 and nothing is owned.
    (p as usize).wrapping_sub(RANGE_BASE.load(Relaxed)) < RANGE_LEN.load(Relaxed)
}

/// The size of the class of `p`, a live block of this allocator.
///
/// # Safety
///
/// `p` is a live block this allocator handed out.
#[inline]
pub(crate) unsafe fn block_size(p: *const u8) -> usize {
    unsafe { (*page_of(p)).size as usize }
}

/// The class of `p`, a live block of this allocator.
///
/// # Safety
///
/// As for [`block_size`].
#[inline]
pub(crate) unsafe fn class_at(p: *const u8) -> usize {
    unsafe { (*page_of(p)).class as usize }
}

/// Free `p`, a live block of this allocator. `local` is the calling
/// thread's heap ([`local`]), when it has one.
///
/// # Safety
///
/// `p` is a live block this allocator handed out, not used again.
#[inline]
pub(crate) unsafe fn free(p: *mut u8, local: Option<&Heap>) {
    let page = page_of(p);
    let owner = unsafe { (*page).heap.load(Relaxed) };
    match local {
        Some(h) if std::ptr::eq(h, owner) => unsafe { h.free_local(page, p.cast()) },
        _ => unsafe { (*owner).push_remote(p.cast()) },
    }
}

impl Heap {
    /// A block of class `class`, or null when no page can be had (the
    /// caller falls back to the system allocator).
    ///
    /// # Safety
    ///
    /// The calling thread owns the heap; `class < CLASSES`.
    #[inline]
    pub(crate) unsafe fn alloc(&self, class: usize) -> *mut u8 {
        debug_assert!(class < CLASSES);
        let page = unsafe { self.pages.get_unchecked(class) }.get();
        unsafe {
            let b = (*page).free;
            if !b.is_null() {
                (*page).free = (*b).next;
                (*page).used += 1;
                return b.cast();
            }
            let b = (*page).bump;
            if b < (*page).end {
                (*page).bump = b.add((*page).size as usize);
                (*page).used += 1;
                return b;
            }
            self.refill(class)
        }
    }

    /// The slow path of [`Heap::alloc`]: the head page of `class` has no
    /// room. Retire it from the list and serve the next page with room,
    /// then the blocks other threads freed, then a new page.
    #[cold]
    unsafe fn refill(&self, class: usize) -> *mut u8 {
        let cell = &self.pages[class];
        let head = cell.get();
        if head != empty() {
            unsafe {
                (*head).full = true;
                self.unlink(class, head);
            }
        }
        // Every page behind the head entered the list through a free, so
        // it has a free block.
        if cell.get() == empty() {
            unsafe { self.drain() };
        }
        if cell.get() == empty() {
            let page = self.take_page();
            if page.is_null() {
                return null_mut();
            }
            unsafe { self.format(page, class) };
            cell.set(page);
        }
        unsafe { self.alloc(class) }
    }

    /// Make the empty `page` a page of `class` owned by this heap.
    unsafe fn format(&self, page: *mut Page, class: usize) {
        let size = (class + 1) * 8;
        let data = unsafe { page.cast::<u8>().add(PAGE_HEADER) };
        let blocks = (PAGE - PAGE_HEADER) / size;
        unsafe {
            page.write(Page {
                free: null_mut(),
                bump: data,
                end: data.add(blocks * size),
                used: 0,
                size: size as u32,
                class: class as u8,
                full: false,
                heap: AtomicPtr::new(self as *const Heap as *mut Heap),
                prev: null_mut(),
                next: null_mut(),
            });
        }
    }

    /// Free `b` into `page`, one of this heap's pages.
    #[inline]
    unsafe fn free_local(&self, page: *mut Page, b: *mut Free) {
        unsafe {
            (*b).next = (*page).free;
            (*page).free = b;
            (*page).used -= 1;
            if (*page).full || (*page).used == 0 {
                self.page_freed(page);
            }
        }
    }

    /// A free just gave room to a full page, or emptied a page: put the
    /// first back on its class list, and recycle the second unless it is
    /// the page its class allocates from.
    #[cold]
    unsafe fn page_freed(&self, page: *mut Page) {
        let class = unsafe { (*page).class } as usize;
        unsafe {
            if (*page).full {
                (*page).full = false;
                self.link_second(class, page);
            }
            if (*page).used == 0 && self.pages[class].get() != page {
                self.unlink(class, page);
                self.retire(page);
            }
        }
    }

    /// Insert `page` behind the head of its class list, so the page being
    /// allocated from stays current, or make it the head of an empty list.
    unsafe fn link_second(&self, class: usize, page: *mut Page) {
        let head = self.pages[class].get();
        unsafe {
            if head == empty() {
                (*page).prev = null_mut();
                (*page).next = null_mut();
                self.pages[class].set(page);
            } else {
                let next = (*head).next;
                (*page).prev = head;
                (*page).next = next;
                if !next.is_null() {
                    (*next).prev = page;
                }
                (*head).next = page;
            }
        }
    }

    /// Take `page` off its class list.
    unsafe fn unlink(&self, class: usize, page: *mut Page) {
        unsafe {
            let (prev, next) = ((*page).prev, (*page).next);
            if !next.is_null() {
                (*next).prev = prev;
            }
            if prev.is_null() {
                self.pages[class].set(if next.is_null() { empty() } else { next });
            } else {
                (*prev).next = next;
            }
            (*page).prev = null_mut();
            (*page).next = null_mut();
        }
    }

    /// Keep the empty `page` for this heap, or pool it once the cache is
    /// full.
    unsafe fn retire(&self, page: *mut Page) {
        if self.cached.get() < CACHED_PAGES {
            unsafe { (*page).next = self.cache.get() };
            self.cache.set(page);
            self.cached.set(self.cached.get() + 1);
        } else {
            SHARED.with(|s| unsafe { s.pool_push(page) });
        }
    }

    /// An empty page: from this heap's cache, the shared pool, a parked
    /// heap's remote frees, or the fresh range, in that order. Null when
    /// none is left.
    fn take_page(&self) -> *mut Page {
        let page = self.cache.get();
        if !page.is_null() {
            self.cache.set(unsafe { (*page).next });
            self.cached.set(self.cached.get() - 1);
            return page;
        }
        // Before growing the range, recycle what other threads freed into
        // parked heaps. A few collections at most: a heap whose frees
        // keep arriving must not hold the caller up.
        let mut collections = 0;
        loop {
            let collect = collections < 3;
            collections += 1;
            let (page, parked) = SHARED.with(|s| {
                if !s.pool.is_null() {
                    return (s.pool_pop(), None);
                }
                match collect.then(|| unsafe { s.unpark_pending() }).flatten() {
                    Some(h) => (null_mut(), Some(h)),
                    None => (unsafe { s.fresh() }, None),
                }
            });
            let Some(parked) = parked else {
                return page;
            };
            unsafe {
                (*parked).tidy();
                SHARED.with(|s| s.park(parked));
            }
        }
    }

    /// Free every block other threads pushed onto the remote list.
    unsafe fn drain(&self) {
        let mut b = self.remote.0.swap(null_mut(), Acquire);
        while !b.is_null() {
            unsafe {
                let next = (*b).next;
                self.free_local(page_of(b.cast()), b);
                b = next;
            }
        }
    }

    /// Push `b`, a block of one of this heap's pages, from a thread that
    /// does not own the heap.
    unsafe fn push_remote(&self, b: *mut Free) {
        let mut head = self.remote.0.load(Relaxed);
        loop {
            unsafe { (*b).next = head };
            match self.remote.0.compare_exchange_weak(head, b, Release, Relaxed) {
                Ok(_) => return,
                Err(h) => head = h,
            }
        }
    }

    /// Drain the remote list and pool every empty page, the cache
    /// included: the heap is about to be parked, or is parked and being
    /// collected, so nothing will allocate from it soon.
    unsafe fn tidy(&self) {
        unsafe { self.drain() };
        let mut pooled: *mut Page = null_mut();
        for class in 0..CLASSES {
            let head = self.pages[class].get();
            if head != empty() && unsafe { (*head).used } == 0 {
                unsafe {
                    self.unlink(class, head);
                    (*head).next = pooled;
                }
                pooled = head;
            }
        }
        let mut page = self.cache.replace(null_mut());
        self.cached.set(0);
        while !page.is_null() {
            unsafe {
                let next = (*page).next;
                (*page).next = pooled;
                pooled = page;
                page = next;
            }
        }
        if !pooled.is_null() {
            SHARED.with(|s| {
                while !pooled.is_null() {
                    unsafe {
                        let next = (*pooled).next;
                        s.pool_push(pooled);
                        pooled = next;
                    }
                }
            });
        }
    }
}

/// The calling thread's heap: null before its first allocation, then the
/// heap, or one of the markers below.
const STARTING: usize = 1;
const GONE: usize = 2;

thread_local! {
    static CURRENT: Cell<*mut Heap> = const { Cell::new(null_mut()) };
    static TEARDOWN: Teardown = const { Teardown };
}

/// Parks the thread's heap when the thread exits.
struct Teardown;

impl Drop for Teardown {
    fn drop(&mut self) {
        let h = CURRENT.with(|c| c.replace(GONE as *mut Heap));
        if SOLE_THREAD.load(Relaxed) == crate::sys::thread_word() {
            // A later thread may be given this thread's word.
            SOLE_THREAD.store(SOLE_RETIRED, Relaxed);
        }
        if h as usize > GONE {
            unsafe {
                crate::heap::release_credit(&(*h).tally);
                (*h).tidy();
                SHARED.with(|s| s.park(h));
            }
        }
    }
}

/// The program's thread ([`crate::platform::cinch_run_main`] claims it
/// with [`claim_sole`]) finds its heap through [`SOLE_THREAD`] and
/// [`SOLE_HEAP`]: a compare of its thread register
/// ([`crate::sys::thread_word`]) with a global word, instead of a
/// thread-local lookup, which on Darwin is a call through the variable's
/// descriptor on every allocation and free. Every other thread, and this
/// one once it tears down, uses [`CURRENT`]. `SOLE_THREAD` holds the
/// owner's word or a marker no thread word equals; only the owner can
/// match it, and the owner wrote `SOLE_HEAP` before it. The slot is
/// claimed once and never handed on.
static SOLE_THREAD: AtomicUsize = AtomicUsize::new(SOLE_UNCLAIMED);
static SOLE_HEAP: AtomicPtr<Heap> = AtomicPtr::new(null_mut());
const SOLE_UNCLAIMED: usize = 0;
const SOLE_CLAIMING: usize = 2;
const SOLE_RETIRED: usize = 4;

/// Give the calling thread the sole heap's fast path, if no thread has
/// had it.
pub(crate) fn claim_sole() {
    let word = crate::sys::thread_word();
    if word == crate::sys::NO_THREAD_WORD {
        return;
    }
    let Some(h) = local() else {
        return;
    };
    if SOLE_THREAD.compare_exchange(SOLE_UNCLAIMED, SOLE_CLAIMING, Relaxed, Relaxed).is_ok() {
        SOLE_HEAP.store(h as *const Heap as *mut Heap, Relaxed);
        SOLE_THREAD.store(word, Relaxed);
    }
}

/// The calling thread's heap, made or adopted on its first call. `None`
/// while the heap is being made (an allocation the setup itself makes)
/// and once the thread is past its teardown: those requests go to the
/// system allocator.
#[inline(always)]
pub(crate) fn local() -> Option<&'static Heap> {
    if crate::sys::thread_word() == SOLE_THREAD.load(Relaxed) {
        return Some(unsafe { &*SOLE_HEAP.load(Relaxed) });
    }
    local_slow()
}

/// [`local`] through the thread-local variable.
#[inline]
fn local_slow() -> Option<&'static Heap> {
    let h = CURRENT.with(Cell::get);
    if h as usize > GONE {
        return Some(unsafe { &*h });
    }
    if h.is_null() {
        return start();
    }
    None
}

#[cold]
fn start() -> Option<&'static Heap> {
    CURRENT.with(|c| c.set(STARTING as *mut Heap));
    // Register the teardown first: registering may allocate, and those
    // allocations see STARTING. A thread already tearing down cannot
    // register one, and allocates from the system from then on.
    if TEARDOWN.try_with(|_| ()).is_err() {
        CURRENT.with(|c| c.set(GONE as *mut Heap));
        return None;
    }
    let mut h = SHARED.with(|s| s.adopt());
    if h.is_null() {
        h = create();
    }
    CURRENT.with(|c| c.set(if h.is_null() { GONE as *mut Heap } else { h }));
    (!h.is_null()).then(|| unsafe { &*h })
}

/// A new heap, registered in [`HEAPS`]; null when the system refuses it.
fn create() -> *mut Heap {
    let layout = Layout::new::<Heap>();
    // Zeroed memory is a valid heap: null pointers, zero counts and
    // counters. Heap metadata is not counted in the live heap.
    let h = unsafe { System.alloc_zeroed(layout) }.cast::<Heap>();
    if h.is_null() {
        return h;
    }
    for cell in unsafe { &(*h).pages } {
        cell.set(empty());
    }
    let mut head = HEAPS.load(Relaxed);
    loop {
        unsafe { (*h).all_next = head };
        match HEAPS.compare_exchange_weak(head, h, Release, Relaxed) {
            Ok(_) => return h,
            Err(now) => head = now,
        }
    }
}

/// Every heap ever made, parked ones included, newest first.
pub(crate) fn heaps() -> impl Iterator<Item = &'static Heap> {
    let mut h = HEAPS.load(Acquire);
    std::iter::from_fn(move || {
        let cur = unsafe { h.as_ref() }?;
        h = cur.all_next;
        Some(cur)
    })
}

static HEAPS: AtomicPtr<Heap> = AtomicPtr::new(null_mut());
static RANGE_BASE: AtomicUsize = AtomicUsize::new(0);
static RANGE_LEN: AtomicUsize = AtomicUsize::new(0);

/// The state threads share: the reserved range, the page pool and the
/// parked heaps. Every use is short and allocation-free.
struct Shared {
    /// The range, once reserved: `[base, limit)`, carved up to `next`,
    /// accessible up to `committed`.
    base: usize,
    limit: usize,
    next: usize,
    committed: usize,
    /// Whether the reservation was attempted (it may have failed).
    reserved: bool,
    /// Empty pages, linked through `next`.
    pool: *mut Page,
    /// Parked heaps, linked through `parked_next`.
    parked: *mut Heap,
}

// SAFETY: the raw pointers name pages and heaps that any thread may use
// once it holds the lock.
unsafe impl Send for Shared {}

static SHARED: Locked<Shared> = Locked::new(Shared {
    base: 0,
    limit: 0,
    next: 0,
    committed: 0,
    reserved: false,
    pool: null_mut(),
    parked: null_mut(),
});

impl Shared {
    unsafe fn pool_push(&mut self, page: *mut Page) {
        unsafe { (*page).next = self.pool };
        self.pool = page;
    }

    fn pool_pop(&mut self) -> *mut Page {
        let page = self.pool;
        self.pool = unsafe { (*page).next };
        page
    }

    fn park(&mut self, h: *mut Heap) {
        unsafe { (*h).parked_next.set(self.parked) };
        self.parked = h;
    }

    /// A parked heap for a starting thread, or null.
    fn adopt(&mut self) -> *mut Heap {
        let h = self.parked;
        if !h.is_null() {
            self.parked = unsafe { (*h).parked_next.get() };
        }
        h
    }

    /// Unpark a heap with remote frees waiting, for the caller to collect
    /// and park again.
    unsafe fn unpark_pending(&mut self) -> Option<*mut Heap> {
        let mut link: *mut *mut Heap = &mut self.parked;
        unsafe {
            while !(*link).is_null() {
                let h = *link;
                if !(*h).remote.0.load(Relaxed).is_null() {
                    *link = (*h).parked_next.get();
                    return Some(h);
                }
                link = (*h).parked_next.as_ptr();
            }
        }
        None
    }

    /// A page from the unused part of the range, reserving the range on
    /// first use and committing it a chunk at a time. Null when the range
    /// is used up or cannot be had.
    unsafe fn fresh(&mut self) -> *mut Page {
        if !self.reserved {
            self.reserved = true;
            self.reserve();
        }
        if self.limit - self.next < PAGE {
            return null_mut();
        }
        if self.next == self.committed {
            let chunk = COMMIT_CHUNK.min(self.limit - self.committed);
            if crate::sys::mprotect(self.committed as i64, chunk, PROT_READ | PROT_WRITE).0 != 0 {
                return null_mut();
            }
            self.committed += chunk;
        }
        let page = self.next as *mut Page;
        self.next += PAGE;
        page
    }

    fn reserve(&mut self) {
        for size in RESERVATIONS {
            let (addr, _) = crate::sys::mmap_reserve(size + PAGE);
            if addr == -1 {
                continue;
            }
            let base = (addr as usize + PAGE - 1) & !(PAGE - 1);
            self.base = base;
            self.limit = base + size;
            self.next = base;
            self.committed = base;
            RANGE_BASE.store(base, Release);
            RANGE_LEN.store(size, Release);
            return;
        }
    }
}

/// A spin lock: the allocator cannot use a lock that might allocate, and
/// every critical section here is a few pointer moves (or, rarely, one
/// `mmap`/`mprotect`).
struct Locked<T> {
    held: AtomicBool,
    value: UnsafeCell<T>,
}

// SAFETY: the value is only reached through `with`, under the lock.
unsafe impl<T: Send> Sync for Locked<T> {}

impl<T> Locked<T> {
    const fn new(value: T) -> Self {
        Locked { held: AtomicBool::new(false), value: UnsafeCell::new(value) }
    }

    /// Run `f` on the value under the lock. `f` must not panic or take
    /// the lock again.
    fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut spins = 0u32;
        while self.held.compare_exchange_weak(false, true, Acquire, Relaxed).is_err() {
            spins += 1;
            if spins < 100 {
                std::hint::spin_loop();
            } else {
                std::thread::yield_now();
            }
        }
        let r = f(unsafe { &mut *self.value.get() });
        self.held.store(false, Release);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Allocate from the calling thread's heap, as the global allocator
    /// does for a small request.
    fn take(size: usize) -> *mut u8 {
        let class = class_of(size, 8).expect("small size");
        let p = unsafe { local().expect("thread heap").alloc(class) };
        assert!(!p.is_null() && owns(p), "a small block from the range");
        p
    }

    fn give(p: *mut u8) {
        unsafe { free(p, local()) }
    }

    #[test]
    fn classes_round_to_words_and_large_requests_fall_through() {
        assert_eq!(class_of(1, 8), Some(0));
        assert_eq!(class_of(8, 8), Some(0));
        assert_eq!(class_of(24, 8), Some(2));
        assert_eq!(class_of(24, 16), Some(3));
        assert_eq!(class_of(256, 8), Some(CLASSES - 1));
        assert_eq!(class_of(257, 8), None);
        assert_eq!(class_of(16, 32), None);
    }

    #[test]
    fn blocks_are_distinct_aligned_and_sized_by_class() {
        for size in (8..=MAX_SMALL).step_by(8) {
            let blocks: Vec<*mut u8> = (0..300).map(|_| take(size)).collect();
            for (i, &p) in blocks.iter().enumerate() {
                assert_eq!(p as usize % 8, 0);
                assert_eq!(unsafe { block_size(p) }, size);
                unsafe { p.write_bytes(i as u8, size) };
            }
            for (i, &p) in blocks.iter().enumerate() {
                let bytes = unsafe { std::slice::from_raw_parts(p, size) };
                assert!(bytes.iter().all(|&b| b == i as u8), "block {i} of class {size} overlapped");
            }
            blocks.into_iter().for_each(give);
        }
        let p = unsafe { local().unwrap().alloc(class_of(48, 16).unwrap()) };
        assert_eq!(p as usize % 16, 0);
        give(p);
    }

    #[test]
    fn a_freed_block_is_reused_by_the_next_request() {
        let a = take(40);
        give(a);
        let b = take(40);
        assert_eq!(a, b);
        give(b);
    }

    #[test]
    fn blocks_freed_on_another_thread_return_to_the_owner() {
        // Every other block of many pages is freed elsewhere, so no page
        // empties into the shared pool: the owner reuses them all once its
        // current page's untouched tail is used up.
        let n = 20_000;
        let blocks: Vec<usize> = (0..n).map(|_| take(72) as usize).collect();
        let (kept, freed): (Vec<_>, Vec<_>) = blocks.chunks(2).map(|c| (c[0], c[1])).unzip();
        let theirs: std::collections::HashSet<usize> = freed.iter().copied().collect();
        std::thread::spawn(move || {
            for p in freed {
                give(p as *mut u8);
            }
        })
        .join()
        .unwrap();
        let again: Vec<*mut u8> = (0..n / 2).map(|_| take(72)).collect();
        let reused = again.iter().filter(|&&p| theirs.contains(&(p as usize))).count();
        assert!(reused >= n / 2 - 2 * PAGE / 72, "only {reused} of {} remote frees were reused", n / 2);
        again.into_iter().for_each(give);
        kept.into_iter().for_each(|p| give(p as *mut u8));
    }

    #[test]
    fn concurrent_cross_thread_frees_keep_every_block_intact() {
        // Each thread allocates, stamps and hands blocks to the next
        // thread, which checks the stamp and frees them.
        let threads = 6;
        let rounds = 200;
        let per = 500;
        let (txs, rxs): (Vec<_>, Vec<_>) = (0..threads).map(|_| std::sync::mpsc::channel::<Vec<usize>>()).unzip();
        let mut rxs: Vec<Option<_>> = rxs.into_iter().map(Some).collect();
        let handles: Vec<_> = (0..threads)
            .map(|t| {
                let tx = txs[(t + 1) % threads].clone();
                let rx = rxs[t].take().unwrap();
                std::thread::spawn(move || {
                    for r in 0..rounds {
                        let size = 8 * (1 + (t + r) % CLASSES);
                        let batch: Vec<usize> = (0..per)
                            .map(|i| {
                                let p = take(size);
                                unsafe { p.cast::<u64>().write((t * 1_000_000 + i) as u64) };
                                p as usize
                            })
                            .collect();
                        tx.send(batch).unwrap();
                        let got = rx.recv().unwrap();
                        let from = (t + threads - 1) % threads;
                        for (i, p) in got.into_iter().enumerate() {
                            assert_eq!(unsafe { (p as *const u64).read() }, (from * 1_000_000 + i) as u64);
                            give(p as *mut u8);
                        }
                    }
                })
            })
            .collect();
        drop(txs);
        for h in handles {
            h.join().unwrap();
        }
    }
}
