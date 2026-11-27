//! Native reader-writer lock boxes (`RwLockCell a` in `Effect.Concurrent.RwLock`).
//!
//! Multiple concurrent readers read the stored value without serializing behind
//! a single mutex; writers take exclusive ownership to replace the stored word.
//!
//! Ownership discipline matches [`crate::mvar`]:
//! * Every value stored into an `RwLock` is marked shared (`cinch_mark_mt`)
//!   *before* acquiring the lock, preventing deadlocks if the incoming graph
//!   reaches this lock and guaranteeing every reader's `cinch_dup` is atomic.
//! * Reads duplicate the stored word under the shared read lock and return an
//!   owned reference.
//! * Writes/swaps replace the stored word under the exclusive write lock and
//!   drop (or return) the previous value *after* releasing the write lock.
//! * Dropping the last reference to the `RwLock` box runs `Drop for RwLockBox`
//!   via `abi::drop_box_payload`, releasing the stored value cleanly.

use std::sync::RwLock;

#[cfg(not(test))]
unsafe extern "C" {
    pub fn cinch_rwlock_new(value: i64, format: i64) -> i64;
    pub fn cinch_rwlock_access(word: i64, value: i64, op: i64, out: *mut i64) -> i64;
}

use crate::{abi, block, rc};

// ABI operation codes shared with compiler intrinsics.
const OP_READ: i64 = 0;
const OP_WRITE: i64 = 1;
const OP_SWAP: i64 = 2;
const OP_TRY_READ: i64 = 3;
const OP_TRY_WRITE: i64 = 4;

struct Value {
    word: i64,
    format: i64,
}

impl Value {
    unsafe fn owned(word: i64, format: i64) -> Self {
        if format != block::SCAN_RAW as i64 && rc::reclaiming() {
            unsafe { rc::cinch_mark_mt(word) };
        }
        Self { word, format }
    }

    fn duplicate(&self) -> Self {
        if self.format != block::SCAN_RAW as i64 && rc::reclaiming() {
            unsafe { rc::cinch_dup(self.word) };
        }
        Self {
            word: self.word,
            format: self.format,
        }
    }

    fn into_word(self) -> i64 {
        let word = self.word;
        std::mem::forget(self);
        word
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        if self.format != block::SCAN_RAW as i64 && rc::reclaiming() {
            unsafe { rc::cinch_drop(self.word) };
        }
    }
}

pub(crate) struct RwLockBox {
    format: i64,
    lock: RwLock<Value>,
}

#[inline]
unsafe fn cell<'a>(word: i64) -> &'a RwLockBox {
    if word == 0
        || word & 7 != 0
        || block::kind(unsafe { block::read_header(word as usize as *const u8) })
            != block::KIND_RWLOCK
    {
        abi::panic_exit(b"compiler bug: corrupt RwLock box");
    }
    unsafe { &*(word as usize as *const RwLockBox) }
}

/// Allocate a new `RwLock` initialized with owned `value` of scan code `format`.
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_rwlock_new(value: i64, format: i64) -> i64 {
    if !(block::SCAN_RAW as i64..=block::SCAN_TOBJ as i64).contains(&format) {
        abi::panic_exit(b"compiler bug: invalid RwLock value format");
    }
    let val = unsafe { Value::owned(value, format) };
    block::allocate_value(
        block::KIND_RWLOCK,
        block::RC_FRESH,
        RwLockBox {
            format,
            lock: RwLock::new(val),
        },
    ) as usize as i64
}

/// Access a borrowed `RwLock` box.
/// * `OP_READ` (0): writes duplicated owned word to `*out`, returns 1.
/// * `OP_WRITE` (1): consumes `value`, replaces stored value, drops old value
///   outside the lock, writes 0 to `*out`, returns 1.
/// * `OP_SWAP` (2): consumes `value`, replaces stored value, writes old owned
///   word to `*out`, returns 1.
/// * `OP_TRY_READ` (3): non-blocking read; returns 1 and writes `*out` on
///   success, or 0 without writing `*out` if write-locked.
/// * `OP_TRY_WRITE` (4): consumes `value` on every outcome; returns 1 on
///   success, or 0 (dropping `value`) if contended.
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_rwlock_access(
    word: i64,
    value: i64,
    op: i64,
    out: *mut i64,
) -> i64 {
    let bx = unsafe { cell(word) };
    match op {
        OP_READ => {
            let guard = bx.lock.read().unwrap_or_else(|e| e.into_inner());
            let dup = guard.duplicate();
            drop(guard);
            unsafe { out.write(dup.into_word()) };
            1
        }
        OP_TRY_READ => match bx.lock.try_read() {
            Ok(guard) => {
                let dup = guard.duplicate();
                drop(guard);
                unsafe { out.write(dup.into_word()) };
                1
            }
            Err(_) => 0,
        },
        OP_WRITE => {
            let incoming = unsafe { Value::owned(value, bx.format) };
            let old = {
                let mut guard = bx.lock.write().unwrap_or_else(|e| e.into_inner());
                std::mem::replace(&mut *guard, incoming)
            };
            drop(old);
            unsafe { out.write(0) };
            1
        }
        OP_SWAP => {
            let incoming = unsafe { Value::owned(value, bx.format) };
            let old = {
                let mut guard = bx.lock.write().unwrap_or_else(|e| e.into_inner());
                std::mem::replace(&mut *guard, incoming)
            };
            unsafe { out.write(old.into_word()) };
            1
        }
        OP_TRY_WRITE => {
            let incoming = unsafe { Value::owned(value, bx.format) };
            match bx.lock.try_write() {
                Ok(mut guard) => {
                    let old = std::mem::replace(&mut *guard, incoming);
                    drop(guard);
                    drop(old);
                    1
                }
                Err(_) => {
                    drop(incoming);
                    0
                }
            }
        }
        _ => abi::panic_exit(b"compiler bug: invalid RwLock operation"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rwlock_concurrent_readers_and_writers_preserve_rc() {
        rc::cinch_reclaim_enable();
        unsafe {
            let initial = crate::abi::cinch_bytes_new(16);
            let rw = cinch_rwlock_new(initial, block::SCAN_OBJ as i64);
            rc::cinch_mark_mt(rw);

            let mut readers = Vec::new();
            for _ in 0..4 {
                rc::cinch_dup(rw);
                readers.push(std::thread::spawn(move || {
                    for _ in 0..500 {
                        let mut out = 0i64;
                        assert_eq!(cinch_rwlock_access(rw, 0, OP_READ, &mut out), 1);
                        assert_ne!(out, 0);
                        rc::cinch_drop(out);
                    }
                    rc::cinch_drop(rw);
                }));
            }

            for i in 1..=20 {
                let next = crate::abi::cinch_bytes_new(16 + i);
                let mut out = 0i64;
                assert_eq!(cinch_rwlock_access(rw, next, OP_SWAP, &mut out), 1);
                assert_ne!(out, 0);
                rc::cinch_drop(out);
            }

            for h in readers {
                h.join().unwrap();
            }

            let mut out = 0i64;
            assert_eq!(cinch_rwlock_access(rw, 0, OP_TRY_READ, &mut out), 1);
            assert_eq!(crate::abi::cinch_bytes_len(out), 36);
            rc::cinch_drop(out);

            let final_buf = crate::abi::cinch_bytes_new(8);
            assert_eq!(cinch_rwlock_access(rw, final_buf, OP_TRY_WRITE, &mut out), 1);
            rc::cinch_drop(rw);
        }
    }
}
