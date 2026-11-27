//! Native lock-free 64-bit hardware atomic cells (`AtomicI64`, `AtomicU64`,
//! `AtomicBool` in `Effect.Concurrent.Atomic`).
//!
//! Each cell is a reference-counted runtime box (`KIND_ATOMIC`) holding a
//! single cache-friendly `AtomicI64` payload word. Because the payload is a
//! raw machine word with no counted children, the generic RC mark and drop
//! walks scan zero slots; freeing the box when its last handle drops needs
//! no Rust destructor beyond reclaiming the block.
//!
//! Memory order codes (ABI):
//!   0 = Relaxed
//!   1 = Acquire
//!   2 = Release
//!   3 = AcqRel
//!   4 = SeqCst

use std::sync::atomic::{fence, AtomicI64, Ordering};

#[cfg(not(test))]
unsafe extern "C" {
    pub fn cinch_atomic_new(initial: i64) -> i64;
    pub fn cinch_atomic_load(word: i64, order: i64) -> i64;
    pub fn cinch_atomic_store(word: i64, value: i64, order: i64) -> i64;
    pub fn cinch_atomic_swap(word: i64, value: i64, order: i64) -> i64;
    pub fn cinch_atomic_fetch_add(word: i64, delta: i64, order: i64) -> i64;
    pub fn cinch_atomic_fetch_sub(word: i64, delta: i64, order: i64) -> i64;
    pub fn cinch_atomic_fetch_and(word: i64, mask: i64, order: i64) -> i64;
    pub fn cinch_atomic_fetch_or(word: i64, mask: i64, order: i64) -> i64;
    pub fn cinch_atomic_fetch_xor(word: i64, mask: i64, order: i64) -> i64;
    pub fn cinch_atomic_cas(word: i64, expected: i64, desired: i64, success_order: i64, failure_order: i64) -> i64;
    pub fn cinch_atomic_fence(order: i64) -> i64;
}

use crate::{abi, block};

pub(crate) struct AtomicCell {
    value: AtomicI64,
}

#[inline]
fn load_ordering(order: i64) -> Ordering {
    match order {
        0 => Ordering::Relaxed,
        1 => Ordering::Acquire,
        4 => Ordering::SeqCst,
        _ => abi::panic_exit(b"compiler bug: invalid atomic load ordering"),
    }
}

#[inline]
fn store_ordering(order: i64) -> Ordering {
    match order {
        0 => Ordering::Relaxed,
        2 => Ordering::Release,
        4 => Ordering::SeqCst,
        _ => abi::panic_exit(b"compiler bug: invalid atomic store ordering"),
    }
}

#[inline]
fn rmw_ordering(order: i64) -> Ordering {
    match order {
        0 => Ordering::Relaxed,
        1 => Ordering::Acquire,
        2 => Ordering::Release,
        3 => Ordering::AcqRel,
        4 => Ordering::SeqCst,
        _ => abi::panic_exit(b"compiler bug: invalid atomic RMW ordering"),
    }
}

#[inline]
fn fence_ordering(order: i64) -> Ordering {
    match order {
        1 => Ordering::Acquire,
        2 => Ordering::Release,
        3 => Ordering::AcqRel,
        4 => Ordering::SeqCst,
        _ => abi::panic_exit(b"compiler bug: invalid atomic fence ordering"),
    }
}

#[inline]
unsafe fn cell<'a>(word: i64) -> &'a AtomicCell {
    if word == 0
        || word & 7 != 0
        || block::kind(unsafe { block::read_header(word as usize as *const u8) })
            != block::KIND_ATOMIC
    {
        abi::panic_exit(b"compiler bug: corrupt Atomic cell");
    }
    unsafe { &*(word as usize as *const AtomicCell) }
}

/// Allocate a fresh atomic 64-bit word initialized to `initial`.
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_atomic_new(initial: i64) -> i64 {
    block::allocate_value(
        block::KIND_ATOMIC,
        block::RC_FRESH,
        AtomicCell {
            value: AtomicI64::new(initial),
        },
    ) as usize as i64
}

/// Load the 64-bit value with ordering `order`.
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_atomic_load(word: i64, order: i64) -> i64 {
    unsafe { cell(word) }.value.load(load_ordering(order))
}

/// Store `value` with ordering `order`. Returns 0 (`Unit`).
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_atomic_store(word: i64, value: i64, order: i64) -> i64 {
    unsafe { cell(word) }.value.store(value, store_ordering(order));
    0
}

/// Atomically replace the stored word with `value`, returning the previous word.
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_atomic_swap(word: i64, value: i64, order: i64) -> i64 {
    unsafe { cell(word) }.value.swap(value, rmw_ordering(order))
}

/// Wrapping atomic fetch-add, returning the previous word.
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_atomic_fetch_add(word: i64, delta: i64, order: i64) -> i64 {
    unsafe { cell(word) }
        .value
        .fetch_add(delta, rmw_ordering(order))
}

/// Wrapping atomic fetch-sub, returning the previous word.
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_atomic_fetch_sub(word: i64, delta: i64, order: i64) -> i64 {
    unsafe { cell(word) }
        .value
        .fetch_sub(delta, rmw_ordering(order))
}

/// Bitwise atomic fetch-and, returning the previous word.
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_atomic_fetch_and(word: i64, mask: i64, order: i64) -> i64 {
    unsafe { cell(word) }
        .value
        .fetch_and(mask, rmw_ordering(order))
}

/// Bitwise atomic fetch-or, returning the previous word.
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_atomic_fetch_or(word: i64, mask: i64, order: i64) -> i64 {
    unsafe { cell(word) }
        .value
        .fetch_or(mask, rmw_ordering(order))
}

/// Bitwise atomic fetch-xor, returning the previous word.
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_atomic_fetch_xor(word: i64, mask: i64, order: i64) -> i64 {
    unsafe { cell(word) }
        .value
        .fetch_xor(mask, rmw_ordering(order))
}

/// Strong compare-and-exchange. Returns 1 if the cell equaled `expected` and
/// was replaced with `desired`, or 0 otherwise.
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_atomic_cas(
    word: i64,
    expected: i64,
    desired: i64,
    success_order: i64,
    failure_order: i64,
) -> i64 {
    let ok = rmw_ordering(success_order);
    let fail = load_ordering(failure_order);
    match unsafe { cell(word) }
        .value
        .compare_exchange(expected, desired, ok, fail)
    {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

/// Issue a hardware/compiler atomic memory fence. Returns 0 (`Unit`).
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_atomic_fence(order: i64) -> i64 {
    fence(fence_ordering(order));
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rc;

    #[test]
    fn atomic_ops_and_multithreaded_fetch_add() {
        unsafe {
            let c = cinch_atomic_new(10);
            assert_eq!(cinch_atomic_load(c, 4), 10);
            assert_eq!(cinch_atomic_fetch_add(c, 5, 4), 10);
            assert_eq!(cinch_atomic_load(c, 4), 15);
            assert_eq!(cinch_atomic_cas(c, 15, 42, 4, 4), 1);
            assert_eq!(cinch_atomic_cas(c, 15, 99, 4, 4), 0);
            assert_eq!(cinch_atomic_load(c, 4), 42);
            assert_eq!(cinch_atomic_swap(c, 0, 4), 42);
            assert_eq!(cinch_atomic_fetch_or(c, 0b1010, 4), 0);
            assert_eq!(cinch_atomic_fetch_and(c, 0b1100, 4), 0b1010);
            assert_eq!(cinch_atomic_fetch_xor(c, 0b1111, 4), 0b1000);
            assert_eq!(cinch_atomic_load(c, 4), 0b0111);
            cinch_atomic_store(c, 0, 4);
            cinch_atomic_fence(4);

            rc::cinch_mark_mt(c);
            let mut handles = Vec::new();
            for _ in 0..4 {
                rc::cinch_dup(c);
                handles.push(std::thread::spawn(move || {
                    for _ in 0..5000 {
                        cinch_atomic_fetch_add(c, 1, 0);
                    }
                    rc::cinch_drop(c);
                }));
            }
            for h in handles {
                h.join().unwrap();
            }
            assert_eq!(cinch_atomic_load(c, 4), 20_000);
            rc::cinch_drop(c);
        }
    }
}
