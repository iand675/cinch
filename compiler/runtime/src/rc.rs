//! Reference counts: the count entry points and the release of dead
//! blocks (design/reclamation.md sections 3, 7 and 9).
//!
//! A block word is any word that is neither 0 nor an immediate (low bit
//! 1). [`cinch_dup`] and [`cinch_drop`] ignore every other word and every
//! immortal block (`rc == 0`: statics, interned `Sym` cells), so a `TObj`
//! or nullable `Obj` word can be passed as it is. Generated code inlines
//! the same fast paths and calls [`cinch_drop_slow`] only when a count is
//! about to reach zero.
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
//! decremented; a child at one dies and is descended into. The payload a
//! runtime box owns outside its block (a bignum's limbs, a heap buffer's
//! data) is released with it.
//!
//! # Diagnostics
//!
//! * `CINCH_RC_DEBUG=1`: a freed block is poisoned and quarantined
//!   ([`crate::block::free`]), and every count operation checks that the
//!   header belongs to a live block, so a use after free or a corrupt
//!   word traps instead of reading reused memory. These traps (and the
//!   ones below on an invalid kind or a negative count) fire only on a
//!   compiler or runtime bug: design/no-faults.md's compiler-bug traps,
//!   not program faults.
//! * `CINCH_LEAK_CHECK=1`: [`cinch_exit_check`], which the generated
//!   entry glue calls after `main` returns, reports the live header
//!   blocks that are not immortal and turns a nonzero count into exit
//!   code [`crate::abi::EXIT_LEAK`].

use crate::block::{self, Scan, SCAN_OBJ, SCAN_RAW};

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

/// The compiler-bug trap of a negative count: nothing allocates atomic
/// counts yet, so one is a corrupt header.
fn atomic() -> ! {
    crate::abi::panic_exit(b"compiler bug: atomic reference count on a single-threaded path")
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
        _ => atomic(),
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
        _ => atomic(),
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
            match block::rc(ch) {
                block::RC_IMMORTAL => {}
                block::RC_FRESH => {
                    // `c` dies: park `cur` (its cursor in its count field,
                    // its parent in the vacated slot) and descend.
                    unsafe {
                        block::write_header(cur, block::header(next as i64, block::kind(h), block::layout(h)));
                        slot.write(parent as usize as i64);
                    }
                    child = Some(c);
                    break;
                }
                n if n > 1 => unsafe {
                    block::write_header(c, block::header(n - 1, block::kind(ch), block::layout(ch)))
                },
                _ => atomic(),
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
}
