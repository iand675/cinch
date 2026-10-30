//! Reference counts: the count entry points, the release of dead blocks,
//! and the marking of blocks shared across tasks (design/reclamation.md
//! sections 3, 7, 8 and 9).
//!
//! A block word is any word that is neither 0 nor an immediate (low bit
//! 1). [`cinch_dup`] and [`cinch_drop`] ignore every other word and every
//! immortal block (`rc == 0`: statics, interned `Sym` cells), so a `TObj`
//! or nullable `Obj` word can be passed as it is. Generated code inlines
//! the same fast paths and calls [`cinch_drop_slow`] only when a count is
//! about to reach zero.
//!
//! # Counts shared across tasks
//!
//! A positive count is single-threaded: only one thread can reach the
//! block, and its count changes with ordinary loads and stores. A negative
//! count `-n` is atomic: `n` references, held by any number of threads,
//! changed only by atomic read-modify-writes ([`cinch_dup_mt`],
//! [`cinch_drop_mt`]). A block becomes atomic by marking, never the other
//! way: [`cinch_mark_mt`] negates the count of every mortal block
//! reachable from a task body before its thread starts (mark on spawn,
//! section 8), so the marked set is closed under reachability and an
//! unmarked block is reachable from one thread only. Blocks are immutable
//! once built, apart from slots their maps do not scan (a memo cell's
//! value, a knot's value), so a marked block never gains an unmarked
//! child. A block allocated later is unmarked, and stays thread-local
//! until a spawn marks it or a join hands it, with its finished task's
//! whole history, to the joiner.
//!
//! The count is the low `RC_BITS` of the header word, in two's
//! complement. An atomic dup subtracts one from the whole word: the field
//! of `-n` is `2^RC_BITS - n`, which never borrows from the kind above it.
//! An atomic drop adds one, which carries into the kind only when it
//! takes the last reference (`-1` to `0`); the thread that sees that
//! carry owns the block alone, so it rewrites the header from the value
//! its read-modify-write returned before anything reads the kind again.
//! The orderings are `Arc`'s: a dup is relaxed (its caller already holds
//! a reference, so the count cannot reach zero under it), a drop is a
//! release, and the drop that takes the last reference is followed by an
//! acquire fence, so every other thread's use of the block happens before
//! its release.
//!
//! # Release
//!
//! [`cinch_drop_slow`] frees a block whose last reference is being
//! dropped, and with it every block that reference kept alive. The walk
//! is iterative and needs no memory beyond the dead blocks themselves, so
//! a million-cell list is released in constant native stack. It is a
//! pointer-reversal traversal: a dead block's count field is no longer a
//! count, so it holds the block's resume cursor (the next slot to visit),
//! and the slot through which the walk descended into a dying child holds
//! the link back to the block's own parent. The header's kind and layout
//! stay intact, which is what the scan of the block needs when the walk
//! returns to it; a full pointer would not fit beside them in the header.
//!
//! Each block's counted slots ([`crate::block::Scan`]) are visited in
//! reverse slot order (D390: destruction runs in reverse), and a block is
//! freed after all of its children. A child whose count is above one is
//! decremented; a child at one dies and is descended into. An atomic
//! child is decremented atomically and dies when this walk took its last
//! reference: whichever thread drops a shared block last frees it. The
//! payload a runtime box owns outside its block (a bignum's limbs, a heap
//! buffer's data) is released with it.
//!
//! # Marking
//!
//! [`cinch_mark_mt`] is a pointer-reversal walk too, over live blocks,
//! and also needs no memory beyond them. A block's count must survive the
//! walk (it only changes sign), so there is no field for a resume cursor.
//! Instead the slot through which the walk descended holds the parent
//! link tagged with [`MARK_LINK`] in its low two bits, a pattern no
//! scanned slot otherwise holds (a block word has its low three bits
//! clear, an immediate its low bit set). Returning to a block, the walk
//! finds that slot by scanning down from the top for the tag, restores
//! the child's word in it, and resumes below it. Finding the slot costs
//! the counted slots above it, so a block of `w` scanned words costs at
//! most `w^2 / 2` extra reads in all; every block a map covers inline has
//! at most 16 words, and a constructor with one counted child (a list
//! cell) costs one. A block is marked when the walk first reaches it,
//! before its children, so a shared child or a cycle through a scanned
//! slot is visited once.
//!
//! # Diagnostics
//!
//! * `CINCH_RC_DEBUG=1`: a freed block is poisoned and quarantined
//!   ([`crate::block::free`]), and every count operation checks that the
//!   header belongs to a live block, so a use after free or a corrupt
//!   word traps instead of reading reused memory. These traps (and the
//!   ones below on an invalid kind or a count of the wrong sign) fire only
//!   on a compiler or runtime bug: design/no-faults.md's compiler-bug
//!   traps, not program faults.
//! * `CINCH_LEAK_CHECK=1`: [`cinch_exit_check`], which the generated
//!   entry glue calls after `main` returns, reports the live header
//!   blocks that are not immortal and turns a nonzero count into exit
//!   code [`crate::abi::EXIT_LEAK`].

use std::sync::atomic::{fence, AtomicBool, Ordering};

use crate::block::{self, Scan, SCAN_OBJ, SCAN_RAW};

/// The low bits of a slot [`cinch_mark_mt`] reversed: the parent link,
/// a block address or 0, with this tag in its two low bits.
const MARK_LINK: i64 = 2;

/// Whether the program counts references ([`cinch_reclaim_enable`]).
static RECLAIM: AtomicBool = AtomicBool::new(false);

/// Record that the running program counts references (it was compiled
/// with `TargetProfile::reclaim`). The code generator's global
/// constructor calls it before any generated code runs. Without it the
/// program owns no counts, so the runtime never marks a task body and
/// never drops a task result: every block leaks, as the program expects.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_reclaim_enable() {
    // Relaxed suffices: the constructor runs before `main`, and every
    // thread that reads the flag is spawned after it.
    RECLAIM.store(true, Ordering::Relaxed);
}

/// Whether the program counts references ([`cinch_reclaim_enable`]).
pub fn reclaiming() -> bool {
    RECLAIM.load(Ordering::Relaxed)
}

/// Whether `word` is a block pointer rather than 0 or an immediate.
#[inline]
fn is_block_word(word: i64) -> bool {
    word != 0 && word & 1 == 0
}

/// Whether `kind` is the kind of a live block.
#[inline]
fn live_kind(kind: u8) -> bool {
    (block::KIND_CTOR..=block::KIND_RAW).contains(&kind)
}

/// The compiler-bug trap of a count operation on a header no live block
/// has.
fn corrupt() -> ! {
    crate::abi::panic_exit(b"compiler bug: reference count operation on a freed or corrupt block")
}

/// Debug mode's header validation of the block at `word`.
#[inline]
fn check(word: i64, h: u64) {
    if word & 7 != 0 || !live_kind(block::kind(h)) {
        corrupt();
    }
}

/// Add one reference to the block behind `word`. Immediates, 0 and
/// immortal blocks are ignored.
///
/// # Safety
///
/// `word` is 0, an immediate, or a live block word.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_dup(word: i64) {
    if !is_block_word(word) {
        return;
    }
    let p = word as usize as *mut u8;
    let h = unsafe { block::read_header(p) };
    if crate::heap::rc_debug() {
        check(word, h);
    }
    match block::rc(h) {
        block::RC_IMMORTAL => {}
        // No overflow check: the heap cap keeps every count below `RC_MAX`
        // (block_kinds.rs).
        n if n > 0 => unsafe { block::write_header(p, block::header(n + 1, block::kind(h), block::layout(h))) },
        _ => unsafe { dup_atomic(p) },
    }
}

/// Drop one reference to the block behind `word`, releasing it (and what
/// only it kept alive) when that was the last. Immediates, 0 and immortal
/// blocks are ignored.
///
/// # Safety
///
/// `word` is 0, an immediate, or a live block word whose reference the
/// caller owns and does not use again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_drop(word: i64) {
    if !is_block_word(word) {
        return;
    }
    let p = word as usize as *mut u8;
    let h = unsafe { block::read_header(p) };
    if crate::heap::rc_debug() {
        check(word, h);
    }
    match block::rc(h) {
        block::RC_IMMORTAL => {}
        block::RC_FRESH => unsafe { release(p) },
        n if n > 1 => unsafe { block::write_header(p, block::header(n - 1, block::kind(h), block::layout(h))) },
        _ => unsafe { drop_atomic(p) },
    }
}

/// Release the block behind `word`, whose count is one: the reference
/// being dropped is its last. This is the zero-crossing path of an
/// inlined drop.
///
/// # Safety
///
/// `word` is a live mortal block word with `rc == 1` whose reference the
/// caller owns and does not use again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_drop_slow(word: i64) {
    if !is_block_word(word) {
        corrupt();
    }
    let p = word as usize as *mut u8;
    let h = unsafe { block::read_header(p) };
    if crate::heap::rc_debug() {
        check(word, h);
        if block::rc(h) != block::RC_FRESH {
            corrupt();
        }
    }
    unsafe { release(p) }
}

/// Add one reference to the block behind `word`, whose count is atomic
/// (negative). This is the shared path of an inlined dup.
///
/// # Safety
///
/// `word` is a live block word with a negative count.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_dup_mt(word: i64) {
    let p = word as usize as *mut u8;
    if crate::heap::rc_debug() {
        let h = unsafe { block::read_header(p) };
        check(word, h);
        if block::rc(h) >= 0 {
            corrupt();
        }
    }
    unsafe { dup_atomic(p) }
}

/// Drop one reference to the block behind `word`, whose count is atomic
/// (negative), releasing it when this was the last reference on any
/// thread. This is the shared path of an inlined drop.
///
/// # Safety
///
/// `word` is a live block word with a negative count whose reference the
/// caller owns and does not use again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_drop_mt(word: i64) {
    let p = word as usize as *mut u8;
    if crate::heap::rc_debug() {
        let h = unsafe { block::read_header(p) };
        check(word, h);
        if block::rc(h) >= 0 {
            corrupt();
        }
    }
    unsafe { drop_atomic(p) }
}

/// One more reference to the atomic block `p`: the count `-n` becomes
/// `-(n + 1)`, one less in the whole word (module docs).
///
/// # Safety
///
/// `p` is a live block with a negative count, one of whose references the
/// caller holds.
#[inline]
unsafe fn dup_atomic(p: *mut u8) {
    unsafe { block::header_cell(p) }.fetch_sub(1, Ordering::Relaxed);
}

/// One reference fewer to the atomic block `p`, releasing it when this
/// was the last.
///
/// # Safety
///
/// `p` is a live block with a negative count; the caller owns one of its
/// references and does not use it again.
#[inline]
unsafe fn drop_atomic(p: *mut u8) {
    if unsafe { take_atomic(p) } {
        unsafe { release(p) }
    }
}

/// Drop one reference to the atomic block `p` and report whether it was
/// the last. When it was, this thread owns `p` alone and its header is a
/// count of one again, as [`release`] expects of a dying block.
///
/// # Safety
///
/// As for [`drop_atomic`].
#[inline]
unsafe fn take_atomic(p: *mut u8) -> bool {
    let old = unsafe { block::header_cell(p) }.fetch_add(1, Ordering::Release);
    if block::rc(old) != -1 {
        return false;
    }
    // Every other thread's last use of `p` was released by its own
    // decrement; acquire them before reading or freeing the block.
    fence(Ordering::Acquire);
    // The add carried out of the count into the kind; restore the header
    // from the value before it.
    unsafe { block::write_header(p, block::header(block::RC_FRESH, block::kind(old), block::layout(old))) };
    true
}

/// Make every mortal block reachable from `word` atomically counted: the
/// mark on spawn of design/reclamation.md section 8. The task runtime
/// calls it on a body closure before the body's thread exists, and the
/// thread's creation publishes the marks to it. The walk stops at
/// immortal blocks, which are never counted, and at blocks already
/// marked, whose reachable blocks are all marked already (module docs).
///
/// # Safety
///
/// `word` is 0, an immediate, or a live block word, and every block
/// reachable from it that is not marked is reachable from this thread
/// only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_mark_mt(word: i64) {
    if !is_block_word(word) || !unsafe { claim(word) } {
        return;
    }
    let mut cur = word as usize as *mut u8;
    let mut parent: *mut u8 = std::ptr::null_mut();
    // The slots of `cur` still to visit, or `None` on a fresh visit.
    let mut resume: Option<usize> = None;
    loop {
        let scan = unsafe { Scan::of(cur, block::read_header(cur)) };
        let mut next = resume.take().unwrap_or_else(|| scan.words());
        let mut child: Option<*mut u8> = None;
        while next > 0 {
            next -= 1;
            if scan.code(next) == SCAN_RAW {
                continue;
            }
            let slot = unsafe { cur.cast::<i64>().add(next) };
            let w = unsafe { slot.read() };
            if is_block_word(w) && unsafe { claim(w) } {
                unsafe { slot.write(parent as usize as i64 | MARK_LINK) };
                child = Some(w as usize as *mut u8);
                break;
            }
        }
        if let Some(c) = child {
            parent = cur;
            cur = c;
            continue;
        }
        if parent.is_null() {
            return;
        }
        // Return to the parent: restore the reversed slot to the block
        // just finished and resume below it.
        let done = cur;
        cur = parent;
        let pscan = unsafe { Scan::of(cur, block::read_header(cur)) };
        let mut at = pscan.words();
        let link = loop {
            if at == 0 {
                corrupt();
            }
            at -= 1;
            if pscan.code(at) == SCAN_RAW {
                continue;
            }
            let w = unsafe { cur.cast::<i64>().add(at).read() };
            if w & 3 == MARK_LINK {
                break w;
            }
        };
        unsafe { cur.cast::<i64>().add(at).write(done as usize as i64) };
        parent = (link & !3) as usize as *mut u8;
        resume = Some(at);
    }
}

/// Mark the block behind the block word `word` when it is mortal and not
/// marked yet: negate its count. Returns whether it was marked now, so
/// that its children still need visiting.
///
/// # Safety
///
/// `word` is a live block word; when its count is positive, only this
/// thread can reach it.
#[inline]
unsafe fn claim(word: i64) -> bool {
    let p = word as usize as *mut u8;
    let h = unsafe { block::read_header(p) };
    if crate::heap::rc_debug() {
        check(word, h);
    } else if !live_kind(block::kind(h)) {
        corrupt();
    }
    let n = block::rc(h);
    if n <= 0 {
        return false;
    }
    unsafe { block::write_header(p, block::header(-n, block::kind(h), block::layout(h))) };
    true
}

/// Whether payload word `slot` of the constructor block behind `word` is
/// counted: its pointer map scans it (`Obj` or `TObj`).
///
/// # Safety
///
/// `word` is a live constructor block word with more than `slot` fields.
pub(crate) unsafe fn field_counted(word: i64, slot: usize) -> bool {
    let p = word as usize as *const u8;
    unsafe { Scan::of(p, block::read_header(p)) }.code(slot) != SCAN_RAW
}

/// Move field `slot` out of the owned constructor block behind `word`
/// and drop the block: the caller receives the field's reference. A
/// unique block gives its field up without count traffic; a shared one
/// keeps its own, so the field is dupped first.
///
/// # Safety
///
/// `word` is a live constructor block word with more than `slot` fields,
/// whose reference the caller owns and does not use again.
pub(crate) unsafe fn take_field(word: i64, slot: usize) -> i64 {
    let p = word as usize as *mut u8;
    let h = unsafe { block::read_header(p) };
    if crate::heap::rc_debug() {
        check(word, h);
    }
    let at = unsafe { p.cast::<i64>().add(slot) };
    let field = unsafe { at.read() };
    if block::rc(h) == block::RC_FRESH {
        // Only the caller can reach the block: the slot gives its
        // reference up, and 0 is a word the release skips.
        unsafe {
            at.write(0);
            release(p);
        }
    } else {
        if unsafe { field_counted(word, slot) } {
            unsafe { cinch_dup(field) };
        }
        unsafe { cinch_drop(word) };
    }
    field
}

/// The iterative release (module docs): free `root` and every block only
/// it kept alive, children before parents, slots in reverse order.
///
/// # Safety
///
/// `root` is a live mortal block whose last reference the caller holds.
unsafe fn release(root: *mut u8) {
    let debug = crate::heap::rc_debug();
    let mut cur = root;
    let mut parent: *mut u8 = std::ptr::null_mut();
    // The slots of `cur` still to visit, or `None` on a fresh visit.
    let mut resume: Option<usize> = None;
    loop {
        let h = unsafe { block::read_header(cur) };
        if !live_kind(block::kind(h)) {
            corrupt();
        }
        let scan = unsafe { Scan::of(cur, h) };
        let mut next = resume.take().unwrap_or_else(|| scan.words());
        let mut child: Option<*mut u8> = None;
        while next > 0 {
            next -= 1;
            let code = scan.code(next);
            if code == SCAN_RAW {
                continue;
            }
            let slot = unsafe { cur.cast::<i64>().add(next) };
            let w = unsafe { slot.read() };
            if !is_block_word(w) {
                // An `Obj` slot never holds an immediate.
                if debug && code == SCAN_OBJ && w & 1 != 0 {
                    corrupt();
                }
                continue;
            }
            let c = w as usize as *mut u8;
            let ch = unsafe { block::read_header(c) };
            if debug {
                check(w, ch);
            }
            let dies = match block::rc(ch) {
                block::RC_IMMORTAL => false,
                block::RC_FRESH => true,
                n if n > 1 => {
                    unsafe { block::write_header(c, block::header(n - 1, block::kind(ch), block::layout(ch))) };
                    false
                }
                // Shared with other threads: whichever drop takes the last
                // reference frees it, here or there.
                _ => unsafe { take_atomic(c) },
            };
            if dies {
                // `c` dies: park `cur` (its cursor in its count field, its
                // parent in the vacated slot) and descend.
                unsafe {
                    block::write_header(cur, block::header(next as i64, block::kind(h), block::layout(h)));
                    slot.write(parent as usize as i64);
                }
                child = Some(c);
                break;
            }
        }
        if let Some(c) = child {
            parent = cur;
            cur = c;
            continue;
        }
        unsafe { dispose(cur, h) };
        if parent.is_null() {
            return;
        }
        // Return to the parent: its cursor names the slot holding its own
        // parent link.
        cur = parent;
        let ph = unsafe { block::read_header(cur) };
        let at = block::rc(ph) as usize;
        parent = unsafe { cur.cast::<i64>().add(at).read() } as usize as *mut u8;
        resume = Some(at);
    }
}

/// Free the dead block `p` (header `h`, its kind intact) after releasing
/// what its runtime box owns outside the block.
///
/// # Safety
///
/// `p` is a dead heap block whose children have been released.
unsafe fn dispose(p: *mut u8, h: u64) {
    #[cfg(test)]
    FREE_LOG.with(|log| {
        if let Some(v) = log.borrow_mut().as_mut() {
            v.push(p as usize);
        }
    });
    unsafe {
        crate::abi::drop_box_payload(p, block::kind(h));
        block::free(p);
    }
    crate::heap::account_block_free();
}

#[cfg(test)]
thread_local! {
    /// The blocks this thread's releases freed, in order, while a test
    /// records them.
    static FREE_LOG: std::cell::RefCell<Option<Vec<usize>>> = const { std::cell::RefCell::new(None) };
}

/// The leak check at program exit: under `CINCH_LEAK_CHECK=1`, write
/// `cinch-leak: live=<n> immortal=<n> leaked=<n>` to stderr, where
/// `leaked` is the live header blocks that are not immortal, and return
/// [`crate::abi::EXIT_LEAK`] when it is not zero. Otherwise `code`, the
/// program's own exit word, is returned unchanged. The generated entry
/// glue calls it after `main` returns, so a fault keeps its own exit code.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_exit_check(code: i64) -> i64 {
    if !crate::heap::leak_check() {
        return code;
    }
    let (_, _, immortal, _) = crate::heap::block_stats();
    let live = crate::heap::live_blocks();
    let leaked = live.saturating_sub(immortal);
    crate::heap::write_leak_report(live, immortal, leaked);
    if leaked == 0 {
        code
    } else {
        i64::from(crate::abi::EXIT_LEAK)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::test_shapes::*;
    use crate::block::{allocate_ctor, allocate_shaped, ctor_id, read_header, KIND_CLOSURE, KIND_DICT};

    /// A constructor block of table index `index` (a regular id below 200
    /// or a test shape) holding `fields`.
    fn ctor(index: u64, fields: &[i64]) -> i64 {
        crate::block::install_test_layouts();
        let p = unsafe { allocate_ctor(fields.len() * 8, index, false) }.cast::<i64>();
        for (i, f) in fields.iter().enumerate() {
            unsafe { p.add(i).write(*f) };
        }
        p as usize as i64
    }

    fn rc_of(word: i64) -> i64 {
        block::rc(unsafe { read_header(word as usize as *const u8) })
    }

    /// Run `f` recording the blocks its releases free on this thread.
    fn freed_by(f: impl FnOnce()) -> Vec<usize> {
        FREE_LOG.with(|l| *l.borrow_mut() = Some(Vec::new()));
        f();
        FREE_LOG.with(|l| l.borrow_mut().take().unwrap())
    }

    /// A static immortal block, like the code generator's `.blk`.
    fn immortal_nil() -> i64 {
        static IMMORTAL_NIL: [u64; 2] = [block::static_header(block::KIND_CTOR, NIL as u16), 0];
        IMMORTAL_NIL.as_ptr().wrapping_add(1) as usize as i64
    }

    #[test]
    fn a_million_cell_list_releases_in_constant_stack() {
        const N: usize = 1_000_000;
        // The list is 24 MB: keep the live-bytes measurement of
        // `abi::tests` from seeing it.
        let _heavy = crate::heap::TEST_HEAVY.lock().unwrap_or_else(|e| e.into_inner());
        // A small stack: any recursion per cell would overflow it.
        let handle = std::thread::Builder::new()
            .stack_size(256 * 1024)
            .spawn(|| {
                let mut xs = immortal_nil();
                for i in 0..N {
                    xs = ctor(CONS_T, &[((i as i64) << 1) | 1, xs]);
                }
                let freed = freed_by(|| unsafe { cinch_drop(xs) });
                freed.len()
            })
            .expect("spawn");
        assert_eq!(handle.join().expect("release thread"), N);
    }

    #[test]
    fn shared_substructure_is_freed_exactly_once() {
        let shared = ctor(CONS_T, &[(7 << 1) | 1, immortal_nil()]);
        let a = ctor(PAIR_RO, &[123, shared]);
        unsafe { cinch_dup(shared) };
        let b = ctor(PAIR_RO, &[456, shared]);
        assert_eq!(rc_of(shared), 2);
        let first = freed_by(|| unsafe { cinch_drop(a) });
        assert_eq!(first, vec![a as usize], "the shared tail survives the first owner");
        assert_eq!(rc_of(shared), 1);
        let second = freed_by(|| unsafe { cinch_drop(b) });
        assert_eq!(second, vec![shared as usize, b as usize]);
    }

    #[test]
    fn children_are_released_in_reverse_slot_order_before_their_parent() {
        let leaves: Vec<i64> = (0..4).map(|_| ctor(NIL, &[])).collect();
        // The third child has its own child, freed before it.
        let inner = ctor(CONS_O, &[ctor(NIL, &[]), immortal_nil()]);
        let grand = unsafe { (inner as usize as *const i64).read() };
        let quad = ctor(QUAD, &[leaves[0], leaves[1], inner, leaves[3]]);
        let order = freed_by(|| unsafe { cinch_drop(quad) });
        let want: Vec<usize> = [leaves[3], grand, inner, leaves[1], leaves[0], quad].iter().map(|w| *w as usize).collect();
        assert_eq!(order, want);
        // `leaves[2]` was never referenced: release one by itself.
        assert_eq!(freed_by(|| unsafe { cinch_drop(leaves[2]) }), vec![leaves[2] as usize]);
    }

    #[test]
    fn tobj_immediates_are_skipped_and_boxed_numbers_freed() {
        let digits = b"340282366920938463463374607431768211456";
        let big = unsafe { crate::abi::cinch_nat_from_decimal(digits.as_ptr(), digits.len()) };
        assert!(is_block_word(big));
        let small = unsafe { crate::abi::cinch_nat_from_i64(41) };
        assert!(!is_block_word(small));
        let tail = ctor(CONS_T, &[small, immortal_nil()]);
        let xs = ctor(CONS_T, &[big, tail]);
        let freed = freed_by(|| unsafe { cinch_drop(xs) });
        assert_eq!(freed, vec![tail as usize, big as usize, xs as usize]);
    }

    #[test]
    fn raw_slots_are_never_touched() {
        // An even raw word that is not a block: dereferencing it would
        // crash.
        let xs = ctor(PAIR_RO, &[0x10, immortal_nil()]);
        assert_eq!(freed_by(|| unsafe { cinch_drop(xs) }), vec![xs as usize]);
    }

    #[test]
    fn counts_above_one_only_decrement() {
        let xs = ctor(CONS_T, &[3, immortal_nil()]);
        unsafe {
            cinch_dup(xs);
            cinch_dup(xs);
        }
        assert_eq!(rc_of(xs), 3);
        assert!(freed_by(|| unsafe { cinch_drop(xs) }).is_empty());
        assert!(freed_by(|| unsafe { cinch_drop(xs) }).is_empty());
        assert_eq!(rc_of(xs), 1);
        assert_eq!(freed_by(|| unsafe { cinch_drop_slow(xs) }), vec![xs as usize]);
        // Immediates, null and immortal blocks are ignored.
        unsafe {
            cinch_dup(0);
            cinch_drop(0);
            cinch_dup(0x2b);
            cinch_drop(0x2b);
            cinch_dup(immortal_nil());
            cinch_drop(immortal_nil());
        }
        assert_eq!(rc_of(immortal_nil()), 0);
    }

    #[test]
    fn closures_scan_their_filled_slots_by_shape() {
        crate::block::install_test_layouts();
        let obj = ctor(NIL, &[]);
        let boxed = unsafe { crate::abi::cinch_int_from_i64(i64::MIN) };
        // Two of three parameters filled: the raw third slot is not there.
        let clo = unsafe { allocate_shaped(5 * 8, KIND_CLOSURE, CLOSURE_OTR, false) }.cast::<i64>();
        unsafe {
            clo.write(0x1000);
            clo.add(1).write(3);
            clo.add(2).write(2);
            clo.add(3).write(obj);
            clo.add(4).write(boxed);
        }
        let w = clo as usize as i64;
        let freed = freed_by(|| unsafe { cinch_drop(w) });
        assert_eq!(freed, vec![boxed as usize, obj as usize, w as usize]);
    }

    #[test]
    fn dictionaries_scan_closure_methods_but_not_value_methods() {
        crate::block::install_test_layouts();
        let method = ctor(NIL, &[]);
        let dict = unsafe { allocate_shaped(3 * 8, KIND_DICT, DICT_OR, false) }.cast::<i64>();
        unsafe {
            dict.write(2);
            dict.add(1).write(method);
            dict.add(2).write(0x40);
        }
        let w = dict as usize as i64;
        assert_eq!(freed_by(|| unsafe { cinch_drop(w) }), vec![method as usize, w as usize]);
    }

    #[test]
    fn wide_constructors_read_their_map_extension() {
        let mut fields = Vec::new();
        let mut want = Vec::new();
        for w in 0..WIDE_WORDS {
            match wide_code(w) {
                block::SCAN_OBJ => {
                    let c = ctor(NIL, &[]);
                    want.push(c as usize);
                    fields.push(c);
                }
                block::SCAN_TOBJ => fields.push(((w as i64) << 1) | 1),
                // An even raw word: touching it would crash.
                _ => fields.push(0x10),
            }
        }
        let wide = ctor(WIDE, &fields);
        want.reverse();
        want.push(wide as usize);
        assert_eq!(freed_by(|| unsafe { cinch_drop(wide) }), want);
    }

    #[test]
    fn memo_cells_release_their_thunk_only() {
        let thunk = ctor(NIL, &[]);
        let cell = unsafe { crate::abi::cinch_alloc(24, i64::from(block::KIND_MEMO)) };
        let p = cell as usize as *mut i64;
        unsafe {
            p.write(2);
            p.add(1).write(thunk);
            // The value has no class at the kind: an even raw word here
            // must not be touched.
            p.add(2).write(0x10);
        }
        assert_eq!(freed_by(|| unsafe { cinch_drop(cell) }), vec![thunk as usize, cell as usize]);
    }

    #[test]
    fn debug_mode_poisons_and_quarantines_freed_blocks() {
        crate::heap::FORCE_RC_DEBUG.with(|d| d.set(true));
        let xs = ctor(CONS_T, &[5, immortal_nil()]);
        unsafe { cinch_drop(xs) };
        let h = unsafe { read_header(xs as usize as *const u8) };
        assert_eq!(block::kind(h), block::KIND_FREED);
        let p = xs as usize as *const u64;
        assert_eq!(unsafe { [p.read(), p.add(1).read()] }, [block::POISON; 2]);
        crate::heap::FORCE_RC_DEBUG.with(|d| d.set(false));
    }

    #[test]
    fn regular_ids_scan_nothing_in_the_unmapped_test_entries() {
        // Entries below 200 have no pointer map: their words are raw.
        let w = ctor(ctor_id(0, 2), &[0x10, 0x20]);
        assert_eq!(freed_by(|| unsafe { cinch_drop(w) }), vec![w as usize]);
    }

    /// The first `n` payload words of the block behind `w`.
    fn words_of(w: i64, n: usize) -> Vec<i64> {
        (0..n).map(|i| unsafe { (w as usize as *const i64).add(i).read() }).collect()
    }

    #[test]
    fn concurrent_counts_on_one_marked_block_end_exact_and_free_once() {
        // Eight threads hammer one marked cell, each round four
        // read-modify-writes of the same header word, through both the
        // generic and the `_mt` entry points, under the debug checks (a
        // count that reached zero early would free the cell, and the next
        // operation on it would trap). 8 x 200,000 rounds is 6.4 million
        // contended updates of one cache line: a plain load and store in
        // place of the atomics loses updates within the first few
        // thousand, so a wrong count shows in every run.
        const THREADS: usize = 8;
        const ROUNDS: usize = 200_000;
        let leaf = ctor(NIL, &[]);
        let xs = ctor(CONS_O, &[leaf, immortal_nil()]);
        unsafe { cinch_mark_mt(xs) };
        assert_eq!((rc_of(xs), rc_of(leaf)), (-1, -1));
        let race = |last: bool| -> Vec<usize> {
            // One reference per thread, taken before the threads start,
            // as a spawn hands its body over.
            for _ in 0..THREADS {
                unsafe { cinch_dup(xs) };
            }
            let start = std::sync::Arc::new(std::sync::Barrier::new(THREADS + 1));
            let threads: Vec<_> = (0..THREADS)
                .map(|t| {
                    let start = std::sync::Arc::clone(&start);
                    std::thread::spawn(move || {
                        crate::heap::FORCE_RC_DEBUG.with(|d| d.set(true));
                        start.wait();
                        for r in 0..ROUNDS {
                            unsafe {
                                if (r + t) % 2 == 0 {
                                    cinch_dup(xs);
                                    cinch_dup(xs);
                                    cinch_drop(xs);
                                    cinch_drop(xs);
                                } else {
                                    cinch_dup_mt(xs);
                                    cinch_drop_mt(xs);
                                    cinch_dup_mt(xs);
                                    cinch_drop_mt(xs);
                                }
                            }
                        }
                        freed_by(|| unsafe { cinch_drop(xs) })
                    })
                })
                .collect();
            crate::heap::FORCE_RC_DEBUG.with(|d| d.set(true));
            start.wait();
            // The last round also drops this thread's own reference while
            // the others still run, so any of the nine may free the cell.
            let mut freed = if last { freed_by(|| unsafe { cinch_drop(xs) }) } else { Vec::new() };
            for t in threads {
                freed.extend(t.join().expect("count thread"));
            }
            crate::heap::FORCE_RC_DEBUG.with(|d| d.set(false));
            freed
        };
        // Every thread's operations balance: the count ends exactly where
        // it started and nothing is freed.
        assert!(race(false).is_empty());
        assert_eq!((rc_of(xs), rc_of(leaf)), (-1, -1));
        // Whichever thread drops last frees the cell and its child, once.
        let mut freed = race(true);
        freed.sort_unstable();
        let mut want = vec![xs as usize, leaf as usize];
        want.sort_unstable();
        assert_eq!(freed, want);
        for w in [xs, leaf] {
            assert_eq!(block::kind(unsafe { read_header(w as usize as *const u8) }), block::KIND_FREED);
        }
    }

    #[test]
    fn marking_stops_at_immortal_blocks() {
        // An immortal block (count 0) holding a mortal one, so the walk
        // could reach the mortal block only through it.
        let behind = ctor(NIL, &[]);
        let mut cell = [block::static_header(block::KIND_CTOR, CONS_O as u16), behind as u64, immortal_nil() as u64];
        let imm = cell.as_mut_ptr().wrapping_add(1) as usize as i64;
        let xs = ctor(CONS_O, &[imm, immortal_nil()]);
        unsafe { cinch_mark_mt(xs) };
        assert_eq!(rc_of(xs), -1);
        assert_eq!((rc_of(imm), rc_of(immortal_nil())), (0, 0));
        assert_eq!(rc_of(behind), 1, "the walk does not pass through an immortal block");
        // An immortal root, an immediate and 0 mark nothing.
        unsafe {
            cinch_mark_mt(imm);
            cinch_mark_mt(0x2b);
            cinch_mark_mt(0);
        }
        assert_eq!(rc_of(behind), 1);
        assert_eq!(words_of(xs, 2), vec![imm, immortal_nil()]);
        assert_eq!(freed_by(|| unsafe { cinch_drop(xs) }), vec![xs as usize]);
        assert_eq!(freed_by(|| unsafe { cinch_drop(behind) }), vec![behind as usize]);
    }

    #[test]
    fn a_marked_subgraph_mixes_with_unmarked_blocks() {
        // `body` stands for a task body: marking it marks `shared`.
        let tail = ctor(CONS_T, &[(1 << 1) | 1, immortal_nil()]);
        let shared = ctor(CONS_T, &[(2 << 1) | 1, tail]);
        let body = ctor(CONS_O, &[shared, immortal_nil()]);
        unsafe { cinch_mark_mt(body) };
        assert_eq!((rc_of(body), rc_of(shared), rc_of(tail)), (-1, -1, -1));
        // Blocks built after the spawn are unmarked, and point into it.
        unsafe { cinch_dup(shared) };
        let plain = ctor(PAIR_RO, &[7, shared]);
        unsafe { cinch_dup(shared) };
        let leaf = ctor(NIL, &[]);
        let last = ctor(NIL, &[]);
        let fresh = ctor(QUAD, &[leaf, shared, immortal_nil(), last]);
        assert_eq!((rc_of(plain), rc_of(fresh), rc_of(shared)), (1, 1, -3));
        // An unmarked block releases with plain counts and decrements its
        // marked child atomically.
        assert_eq!(freed_by(|| unsafe { cinch_drop(plain) }), vec![plain as usize]);
        assert_eq!(rc_of(shared), -2);
        // A second spawn marks what is new and stops at the marked
        // subgraph, whose count is not negated twice; every reversed slot
        // is back as it was.
        let root = ctor(CONS_O, &[fresh, immortal_nil()]);
        let before = words_of(fresh, 4);
        unsafe { cinch_mark_mt(root) };
        assert_eq!(words_of(fresh, 4), before);
        assert_eq!(words_of(root, 2), vec![fresh, immortal_nil()]);
        assert_eq!((rc_of(root), rc_of(fresh), rc_of(leaf), rc_of(last)), (-1, -1, -1, -1));
        assert_eq!((rc_of(shared), rc_of(tail)), (-2, -1));
        // The first body dies alone: `shared` is still held.
        assert_eq!(freed_by(|| unsafe { cinch_drop_mt(body) }), vec![body as usize]);
        assert_eq!(rc_of(shared), -1);
        // The second takes everything left, children in reverse slot
        // order before their parents, atomic counts reaching zero as they
        // go.
        let order = freed_by(|| unsafe { cinch_drop(root) });
        let want: Vec<usize> = [last, tail, shared, leaf, fresh, root].iter().map(|w| *w as usize).collect();
        assert_eq!(order, want);
    }

    #[test]
    fn marking_visits_a_shared_child_once_and_reads_closures_by_shape() {
        crate::block::install_test_layouts();
        let shared = ctor(NIL, &[]);
        unsafe { cinch_dup(shared) };
        let other = ctor(NIL, &[]);
        let boxed = unsafe { crate::abi::cinch_int_from_i64(i64::MIN) };
        // Three filled parameters `(Obj, TObj, Raw)`: the raw word is an
        // even non-pointer that must not be read as a block.
        let clo = unsafe { allocate_shaped(6 * 8, KIND_CLOSURE, CLOSURE_OTR, false) }.cast::<i64>();
        unsafe {
            clo.write(0x1000);
            clo.add(1).write(3);
            clo.add(2).write(3);
            clo.add(3).write(other);
            clo.add(4).write(boxed);
            clo.add(5).write(0x10);
        }
        let clo = clo as usize as i64;
        let quad = ctor(QUAD, &[shared, clo, shared, immortal_nil()]);
        let before = words_of(clo, 6);
        unsafe { cinch_mark_mt(quad) };
        assert_eq!(words_of(clo, 6), before);
        assert_eq!(words_of(quad, 4), vec![shared, clo, shared, immortal_nil()]);
        assert_eq!(rc_of(shared), -2, "a shared child is marked once");
        assert_eq!((rc_of(quad), rc_of(clo), rc_of(other), rc_of(boxed)), (-1, -1, -1, -1));
        let order = freed_by(|| unsafe { cinch_drop(quad) });
        let want: Vec<usize> = [boxed, other, clo, shared, quad].iter().map(|w| *w as usize).collect();
        assert_eq!(order, want);
    }

    #[test]
    fn a_million_cell_list_marks_in_constant_stack_and_keeps_its_words() {
        const N: usize = 1_000_000;
        let _heavy = crate::heap::TEST_HEAVY.lock().unwrap_or_else(|e| e.into_inner());
        let handle = std::thread::Builder::new()
            .stack_size(256 * 1024)
            .spawn(|| {
                let mut xs = immortal_nil();
                for i in 0..N {
                    xs = ctor(CONS_T, &[((i as i64) << 1) | 1, xs]);
                }
                unsafe { cinch_mark_mt(xs) };
                // Every cell is marked once and the spine is intact.
                let (mut cell, mut i) = (xs, N);
                while cell != immortal_nil() {
                    i -= 1;
                    assert_eq!(rc_of(cell), -1);
                    let w = words_of(cell, 2);
                    assert_eq!(w[0], ((i as i64) << 1) | 1);
                    cell = w[1];
                }
                assert_eq!(i, 0);
                // A marked list releases through the atomic path.
                freed_by(|| unsafe { cinch_drop(xs) }).len()
            })
            .expect("spawn");
        assert_eq!(handle.join().expect("mark thread"), N);
    }
}
