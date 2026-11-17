//! Mutable slot storage for `Data.Mutable.*` (`cinch.runtime.slots*`).
//!
//! A slot array is an ordinary `KIND_ARRAY` block with a word element
//! format (0 raw, 1 counted object, 2 counted-or-tagged), so every walk the
//! runtime already has (drop, free, mark-on-spawn, leak accounting) reads
//! it with the map `KIND_ARRAY` always had. What differs is who may write
//! it: the library modules that foreign-import these entries own the block
//! outright and never hand out an `Array` alias of it, so a write in place
//! is unobservable to anything but the container that made it.
//!
//! The one new state is the *hole*: a counted slot holding word 0. Every
//! count path already skips word 0 (`rc::is_block_word`), so a hole costs
//! nothing to drop, free or scan, and a block of holes is what
//! [`cinch_slots_new`] allocates. A hole is never a value: the containers
//! track which slots are written (a length, a ring range, control bytes),
//! and the inline read and take paths the compiler emits trap on a counted
//! hole as defence in depth, because reading one would hand out a null
//! block word.
//!
//! The inline paths (length, read, write, take) are emitted by the
//! compiler (`compiler/src/slots.rs`) and only fall back here to report a
//! violated invariant. The bulk entries below move references between
//! slots without touching a count: a moved word is owned by exactly one
//! slot before and after, so the move itself is the whole ownership
//! transfer.
use crate::{abi, arrays, block, rc};

fn corrupt() -> ! {
    abi::panic_exit(b"compiler bug: corrupt mutable slot array")
}

/// The payload pointer of a word-format slot array, or a trap.
unsafe fn words(a: i64) -> (*mut i64, i64, i64) {
    if a == 0 || a & 7 != 0 {
        corrupt();
    }
    let p = a as usize as *mut u8;
    if block::kind(unsafe { block::read_header(p) }) != block::KIND_ARRAY {
        corrupt();
    }
    let p = p.cast::<i64>();
    let format = unsafe { p.read() };
    if !(0..=2).contains(&format) {
        corrupt();
    }
    let len = unsafe { p.add(1).read() };
    (unsafe { p.add(2) }, len, format)
}

fn in_range(start: i64, n: i64, len: i64) -> bool {
    start >= 0 && n >= 0 && start.checked_add(n).is_some_and(|end| end <= len)
}

/// `n` holes of the element format `f` (the element class's scan code).
/// The allocation is zero-filled, which is what makes every slot a hole.
///
/// # Safety
///
/// `f` is a word format (0, 1 or 2).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_slots_new(n: i64, f: i64) -> i64 {
    if !(0..=2).contains(&f) {
        corrupt();
    }
    unsafe { arrays::array(n, f) }
}

/// Move `n` words from `src[si..]` to `dst[di..]`, with `memmove`
/// semantics when the two are the same block. Each reference ends up owned
/// by exactly one slot:
///
/// 1. a counted word in the destination range is dropped first, unless the
///    same slot is also a source slot (its word is about to be carried away,
///    not overwritten);
/// 2. the words are copied;
/// 3. every source slot that is not also a destination slot becomes a hole.
///
/// So a move never duplicates or loses a reference, whatever the overlap.
/// Out-of-range arguments are a library bug and trap.
///
/// # Safety
///
/// `dst` and `src` are word-format slot arrays of the same element format.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_slots_move(dst: i64, di: i64, src: i64, si: i64, n: i64) -> i64 {
    let (dp, dlen, df) = unsafe { words(dst) };
    let (sp, slen, sf) = unsafe { words(src) };
    if df != sf || !in_range(di, n, dlen) || !in_range(si, n, slen) {
        corrupt();
    }
    if n == 0 {
        return 0;
    }
    let same = dst == src;
    let overlaps_src = |slot: i64| same && slot >= si && slot < si + n;
    let overlaps_dst = |slot: i64| same && slot >= di && slot < di + n;
    if arrays::counted(df) {
        for k in di..di + n {
            if !overlaps_src(k) {
                unsafe { rc::cinch_drop(dp.add(k as usize).read()) };
            }
        }
    }
    unsafe { std::ptr::copy(sp.add(si as usize), dp.add(di as usize), n as usize) };
    for k in si..si + n {
        if !overlaps_dst(k) {
            unsafe { sp.add(k as usize).write(0) };
        }
    }
    0
}

/// Drop the words in `a[i..i + n]` and leave holes behind: the bulk form of
/// a take whose results are discarded (`clear`, `truncate`).
///
/// # Safety
///
/// `a` is a word-format slot array.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_slots_clear(a: i64, i: i64, n: i64) -> i64 {
    let (p, len, f) = unsafe { words(a) };
    if !in_range(i, n, len) {
        corrupt();
    }
    for k in i..i + n {
        let slot = unsafe { p.add(k as usize) };
        let w = unsafe { slot.read() };
        unsafe { slot.write(0) };
        if arrays::counted(f) {
            unsafe { rc::cinch_drop(w) };
        }
    }
    0
}

/// The trap behind the compiler's inline slot paths: an index at or past
/// the length (`code` 1) or a read or take of a counted hole (`code` 2).
/// Either is a bug in the container that issued it, never in its caller.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_slots_fault(code: i64) -> i64 {
    match code {
        1 => abi::panic_exit(b"compiler bug: mutable slot index out of range"),
        _ => abi::panic_exit(b"compiler bug: read of an empty mutable slot"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn put(a: i64, i: i64, w: i64) {
        let (p, _, _) = unsafe { words(a) };
        unsafe { p.add(i as usize).write(w) };
    }

    unsafe fn get(a: i64, i: i64) -> i64 {
        let (p, _, _) = unsafe { words(a) };
        unsafe { p.add(i as usize).read() }
    }

    #[test]
    fn new_slots_are_holes_of_the_given_length() {
        unsafe {
            let a = cinch_slots_new(5, 1);
            assert_eq!(arrays::cinch_array_len(a), 5);
            for i in 0..5 {
                assert_eq!(get(a, i), 0);
            }
            rc::cinch_drop(a);
        }
    }

    #[test]
    fn raw_move_between_blocks_and_within_one() {
        unsafe {
            let a = cinch_slots_new(6, 0);
            let b = cinch_slots_new(6, 0);
            for i in 0..6 {
                put(a, i, 10 + i);
            }
            cinch_slots_move(b, 1, a, 0, 3);
            assert_eq!((get(b, 0), get(b, 1), get(b, 2), get(b, 3)), (0, 10, 11, 12));
            assert_eq!((get(a, 0), get(a, 2), get(a, 3)), (0, 0, 13));
            // Overlapping, forward: [13, 14, 15] at 3.. moves to 2..
            cinch_slots_move(a, 2, a, 3, 3);
            assert_eq!((get(a, 2), get(a, 3), get(a, 4), get(a, 5)), (13, 14, 15, 0));
            // Overlapping, backward: 2..5 moves to 3..6.
            cinch_slots_move(a, 3, a, 2, 3);
            assert_eq!((get(a, 2), get(a, 3), get(a, 4), get(a, 5)), (0, 13, 14, 15));
            rc::cinch_drop(a);
            rc::cinch_drop(b);
        }
    }

    #[test]
    fn counted_move_keeps_every_reference_exactly_once() {
        unsafe {
            // Boxed elements: arrays of format 1 holding fresh array blocks.
            let a = cinch_slots_new(4, 1);
            let elems: Vec<i64> = (0..4).map(|_| cinch_slots_new(1, 0)).collect();
            for (i, e) in elems.iter().enumerate() {
                put(a, i as i64, *e);
            }
            // Shift left by one inside the block: slot 0's word is
            // dropped (it is overwritten), slot 3 becomes a hole.
            let victim = elems[0];
            rc::cinch_dup(victim);
            cinch_slots_move(a, 0, a, 1, 3);
            assert_eq!(block::rc(block::read_header(victim as usize as *const u8)), block::RC_FRESH);
            rc::cinch_drop(victim);
            assert_eq!((get(a, 0), get(a, 1), get(a, 2), get(a, 3)), (elems[1], elems[2], elems[3], 0));
            for k in 1..4 {
                assert_eq!(block::rc(block::read_header(elems[k] as usize as *const u8)), block::RC_FRESH);
            }
            cinch_slots_clear(a, 0, 4);
            for k in 0..4 {
                assert_eq!(get(a, k), 0);
            }
            rc::cinch_drop(a);
        }
    }
}
