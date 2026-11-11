//! Transactional variable cells and the waits of blocked transactions:
//! the runtime half of `Effect.Stm` (design/stm.md).
//!
//! The transaction algorithm is not here. It is Cinch source in the
//! prelude, over inline target IR for the atomic word operations. This
//! module owns only what source cannot express: a block whose scanned
//! slot is rewritten in place, the one global the algorithm shares, and
//! suspension.
//!
//! # The cell
//!
//! A `TVar a` is a `KIND_TVAR` block of three words (`block_kinds.rs`):
//!
//! | word | name | content |
//! |------|------|---------|
//! | 0 | lock | `version << 1 | locked`, read and written atomically by the prelude |
//! | 1 | value | the current value, an ABI word of the cell's format |
//! | 2 | format | the value word's scan code (`SCAN_RAW`, `SCAN_OBJ`, `SCAN_TOBJ`), fixed at creation |
//!
//! The drop and mark walks of [`crate::rc`] scan the value word by the
//! format, so a cell keeps its value alive and releases it when the cell
//! dies, like a constructor field.
//!
//! # The one mutable scanned slot, and its two rules
//!
//! Every other block's scanned slots are written once. A cell's value
//! word is replaced by every commit that writes the variable, which makes
//! two rules load-bearing:
//!
//! 1. **A marked cell holds only marked values.** [`crate::rc`]'s walks
//!    stop at a block whose count is atomic, because everything it
//!    reaches is atomic already. A commit therefore marks a new value
//!    ([`cinch_stm_publish`]) before it stores it in a cell that other
//!    threads can reach, while the value is still private to the
//!    committing thread. An unmarked cell is reachable from one thread
//!    only, so it may hold anything.
//! 2. **A value word is replaced, and its reference copied, only under
//!    the cell's lock.** A reader that copied the pointer and counted it
//!    afterwards could count a block the writer had already released.
//!    The prelude takes the lock bit around the load and the count of a
//!    counted value. An uncounted value (a raw scalar, an immediate, the
//!    null word) needs no count and is read under the version check
//!    alone.
//!
//! # The global state
//!
//! [`cinch_stm_state`] answers the address of [`STATE`]: the global
//! version clock at byte 0 and the count of suspended transactions at
//! byte [`STATE_SLEEPERS_OFFSET`]. The prelude reads and advances the
//! clock itself and reads the sleeper count on every writing commit; the
//! two words sit on separate cache lines because the clock is written by
//! every writing commit and the sleeper count by none of them.
//!
//! # Waiting
//!
//! A transaction that reaches `retry` has read a set of variables and
//! must not run again until one of them changes. It summarizes the set
//! as a 64-bit filter (one bit per variable, chosen by the variable's
//! address) and waits:
//!
//! 1. [`cinch_stm_wait_begin`] registers the calling context
//!    ([`crate::park`]) with the filter and counts it in the sleepers.
//! 2. The prelude then validates its read set again. A variable that
//!    changed since the transaction read it means the wait is already
//!    over: [`cinch_stm_wait_cancel`].
//! 3. Otherwise [`cinch_stm_wait_park`] suspends the context until a
//!    commit wakes it.
//!
//! A writing commit releases its locks, then reads the sleeper count,
//! and when it is not zero calls [`cinch_stm_wake`] with the filter of
//! the variables it wrote; every registered context whose filter
//! intersects is resumed. Both sides store before they load, with
//! sequentially consistent accesses: the waiter registers and then
//! validates, the committer publishes its versions and then reads the
//! count. At least one of them sees the other, so a wake is never lost.
//! A filter collision resumes a transaction whose variables did not
//! change; it runs again, reaches `retry` again and waits again.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use crate::block::{self, SCAN_RAW, SCAN_TOBJ, TVAR_FORMAT_WORD, TVAR_LOCK_WORD, TVAR_VALUE_WORD, TVAR_WORDS};
use crate::{abi, park, rc};

fn corrupt() -> ! {
    abi::panic_exit(b"compiler bug: corrupt transactional variable cell")
}

/// The payload of the cell behind `word`, checking the kind.
///
/// # Safety
///
/// `word` is a word of a live block, 0, or an immediate.
unsafe fn cell(word: i64) -> *mut i64 {
    if word == 0 || word & 7 != 0 {
        corrupt();
    }
    let p = word as usize as *mut u8;
    if block::kind(unsafe { block::read_header(p) }) != block::KIND_TVAR {
        corrupt();
    }
    p.cast()
}

/// A fresh cell holding `value`, whose reference moves into it. `format`
/// is the value's scan code: the class the compiler knows the variable's
/// type has.
///
/// # Safety
///
/// `value` is an owned word of the class `format` names.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_stm_cell_new(value: i64, format: i64) -> i64 {
    if !(SCAN_RAW as i64..=SCAN_TOBJ as i64).contains(&format) {
        corrupt();
    }
    let p = unsafe { block::allocate(TVAR_WORDS as usize * 8, block::KIND_TVAR, block::RC_FRESH, false) }.cast::<i64>();
    unsafe {
        p.add(TVAR_LOCK_WORD as usize).write(0);
        p.add(TVAR_VALUE_WORD as usize).write(value);
        p.add(TVAR_FORMAT_WORD as usize).write(format);
    }
    p as usize as i64
}

/// Make the value `staged` holds safe to store in `target` (rule 1 of
/// the module docs): when `target` is shared and the value is counted
/// and not yet marked, mark everything the value reaches. Returns 1 when
/// it marked something and 0 otherwise.
///
/// Marking one value can mark another variable a later entry of the same
/// commit writes (a variable created in the transaction, stored in a
/// shared one, and written as well), so a commit repeats its pass over
/// its write set until a whole pass returns 0.
///
/// # Safety
///
/// `target` and `staged` are live cell words; `staged` and everything
/// unmarked its value reaches are reachable from the calling thread
/// only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_stm_publish(target: i64, staged: i64) -> i64 {
    if !rc::reclaiming() {
        return 0;
    }
    let t = unsafe { cell(target) };
    if block::rc(unsafe { block::read_header(t.cast()) }) >= 0 {
        return 0;
    }
    let s = unsafe { cell(staged) };
    if unsafe { s.add(TVAR_FORMAT_WORD as usize).read() } == SCAN_RAW as i64 {
        return 0;
    }
    let value = unsafe { s.add(TVAR_VALUE_WORD as usize).read() };
    if value == 0 || value & 1 != 0 {
        return 0;
    }
    if block::rc(unsafe { block::read_header(value as usize as *const u8) }) <= 0 {
        return 0;
    }
    unsafe { rc::cinch_mark_mt(value) };
    1
}

/// One 64-bit word alone on its cache lines. 128 bytes covers the
/// largest line of the supported targets (Apple's arm64 cores).
#[repr(C, align(128))]
struct Line(AtomicU64);

/// The state the prelude's algorithm shares across threads: the global
/// version clock, then the count of suspended transactions.
static STATE: [Line; 2] = [Line(AtomicU64::new(0)), Line(AtomicU64::new(0))];

/// Byte offset of the sleeper count in [`STATE`]. `Effect.Stm` repeats
/// it; `the_state_layout_is_the_one_the_prelude_reads` pins it.
pub const STATE_SLEEPERS_OFFSET: usize = 128;

const _: () = assert!(std::mem::size_of::<Line>() == STATE_SLEEPERS_OFFSET);

/// The address of the shared state (module docs).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_stm_state() -> i64 {
    STATE.as_ptr() as usize as i64
}

fn sleepers() -> &'static AtomicU64 {
    &STATE[1].0
}

/// One suspended transaction: the context to resume, the filter of the
/// variables it read, and whether a commit has resumed it.
struct Waiter {
    parker: park::Parker,
    filter: u64,
    woken: AtomicBool,
}

/// Every transaction between [`cinch_stm_wait_begin`] and the end of its
/// wait. Only waits and the commits that find sleepers take the lock.
static WAITERS: Mutex<Vec<Arc<Waiter>>> = Mutex::new(Vec::new());

thread_local! {
    /// The wait the calling context began and has not ended. Nothing
    /// suspends between [`cinch_stm_wait_begin`] and the call that ends
    /// the wait, so the record is found on the thread that left it.
    static WAITING: std::cell::RefCell<Option<Arc<Waiter>>> = const { std::cell::RefCell::new(None) };
}

fn waiters() -> std::sync::MutexGuard<'static, Vec<Arc<Waiter>>> {
    WAITERS.lock().unwrap_or_else(|e| e.into_inner())
}

/// The wait in progress, taken out of the calling context.
fn take_wait() -> Arc<Waiter> {
    match WAITING.with(|w| w.borrow_mut().take()) {
        Some(w) => w,
        None => abi::panic_exit(b"compiler bug: a transaction ended a wait it did not begin"),
    }
}

/// Take `w` out of the registry and out of the count.
fn deregister(w: &Arc<Waiter>) {
    let mut all = waiters();
    if let Some(i) = all.iter().position(|x| Arc::ptr_eq(x, w)) {
        all.swap_remove(i);
        sleepers().fetch_sub(1, Ordering::SeqCst);
    }
}

/// Register the calling context as waiting for a commit that writes a
/// variable in `filter` (module docs, step 1). The caller validates its
/// read set next and ends the wait with exactly one of
/// [`cinch_stm_wait_park`] and [`cinch_stm_wait_cancel`].
#[unsafe(no_mangle)]
pub extern "C" fn cinch_stm_wait_begin(filter: i64) -> i64 {
    let w = Arc::new(Waiter { parker: park::current(), filter: filter as u64, woken: AtomicBool::new(false) });
    {
        let mut all = waiters();
        all.push(Arc::clone(&w));
        // Counted while the registry is held, so a committer that reads a
        // nonzero count finds the record.
        sleepers().fetch_add(1, Ordering::SeqCst);
    }
    if WAITING.with(|slot| slot.borrow_mut().replace(w)).is_some() {
        abi::panic_exit(b"compiler bug: a transaction began a wait inside a wait");
    }
    0
}

/// End a wait whose read set turned out to have changed already.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_stm_wait_cancel() -> i64 {
    deregister(&take_wait());
    0
}

/// Suspend the calling context until a commit resumes it: 0. When
/// `cancellable` is not 0, a cancellation request for the running task
/// ends the wait as well: 1. A request resumes the task's context
/// (`crate::task`), so the wait observes it without polling.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_stm_wait_park(cancellable: i64) -> i64 {
    let w = take_wait();
    let canceled = loop {
        if w.woken.load(Ordering::SeqCst) {
            break false;
        }
        if cancellable != 0 && crate::task::current_task_canceled() {
            break true;
        }
        suspend(&w);
    };
    deregister(&w);
    i64::from(canceled)
}

/// Stop running until the context's token arrives ([`park::park`]).
#[cfg(not(target_family = "wasm"))]
fn suspend(_w: &Waiter) {
    park::park();
}

/// The cooperative backend has one thread and no context to switch to
/// ([`crate::park`]). The wait runs every queued task instead, each to
/// its end; if none of them committed to a variable this transaction
/// read, nothing left in the program can, and the wait is reported
/// rather than slept on forever.
#[cfg(target_family = "wasm")]
fn suspend(w: &Waiter) {
    crate::coop::io_sleep(0);
    if !w.woken.load(Ordering::SeqCst) {
        abi::panic_exit(b"transaction blocked: `retry` waits for a variable no runnable task can change");
    }
}

/// Resume every suspended transaction whose filter intersects `filter`
/// (module docs). A committer calls it after releasing its locks, and
/// only when it read a nonzero sleeper count.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_stm_wake(filter: i64) -> i64 {
    let all = waiters();
    for w in all.iter() {
        if w.filter & filter as u64 != 0 {
            // The flag is the condition, the unpark the token: set first,
            // so the waiter's loop sees it whichever side runs first.
            w.woken.store(true, Ordering::SeqCst);
            w.parker.unpark();
        }
    }
    0
}

/// Give the processor to another runnable context: the last step of a
/// transaction's contention backoff, after spinning has not helped.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_stm_yield() -> i64 {
    std::thread::yield_now();
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::{Scan, SCAN_OBJ};

    /// A counted child a cell can hold: a buffer block.
    fn child() -> i64 {
        unsafe { abi::cinch_bytes_new(3) }
    }

    unsafe fn count(word: i64) -> i64 {
        block::rc(unsafe { block::read_header(word as usize as *const u8) })
    }

    #[test]
    fn the_state_layout_is_the_one_the_prelude_reads() {
        let base = cinch_stm_state() as usize;
        assert_eq!(base % 128, 0);
        assert_eq!(std::ptr::from_ref(sleepers()) as usize, base + STATE_SLEEPERS_OFFSET);
    }

    #[test]
    fn a_cell_scans_its_value_by_its_format() {
        unsafe {
            for (format, counted) in [(SCAN_RAW, false), (SCAN_OBJ, true), (SCAN_TOBJ, true)] {
                let v = if counted { child() } else { 0x1234 };
                let c = cinch_stm_cell_new(v, format as i64);
                let p = c as usize as *const u8;
                let scan = Scan::of(p, block::read_header(p));
                assert_eq!(scan.words(), TVAR_WORDS as usize);
                assert_eq!(scan.code(TVAR_LOCK_WORD as usize), SCAN_RAW);
                assert_eq!(scan.code(TVAR_VALUE_WORD as usize), format);
                assert_eq!(scan.code(TVAR_FORMAT_WORD as usize), SCAN_RAW);
                assert_eq!(scan.counted_below(TVAR_WORDS as usize).is_some(), counted);
                if counted {
                    // The cell owns the one reference: a second one
                    // survives the cell, and the cell's death drops its own.
                    rc::cinch_dup(v);
                    rc::cinch_drop(c);
                    assert_eq!(count(v), 1);
                    rc::cinch_drop(v);
                } else {
                    rc::cinch_drop(c);
                }
            }
        }
    }

    #[test]
    fn marking_a_cell_marks_the_value_it_holds() {
        unsafe {
            let v = child();
            let c = cinch_stm_cell_new(v, SCAN_OBJ as i64);
            rc::cinch_mark_mt(c);
            assert_eq!(count(c), -1);
            assert_eq!(count(v), -1);
            rc::cinch_drop(c);
        }
    }

    #[test]
    fn publish_marks_only_what_a_shared_target_needs() {
        unsafe {
            rc::cinch_reclaim_enable();
            // A private target takes an unmarked value as it is.
            let private = cinch_stm_cell_new(0, SCAN_OBJ as i64);
            let staged = cinch_stm_cell_new(child(), SCAN_OBJ as i64);
            assert_eq!(cinch_stm_publish(private, staged), 0);
            let v = cell(staged).add(TVAR_VALUE_WORD as usize).read();
            assert_eq!(count(v), 1);
            // A shared target needs the value marked, once.
            let shared = cinch_stm_cell_new(0, SCAN_OBJ as i64);
            rc::cinch_mark_mt(shared);
            assert_eq!(cinch_stm_publish(shared, staged), 1);
            assert_eq!(count(v), -1);
            assert_eq!(count(staged), 1, "the staging cell itself stays private");
            assert_eq!(cinch_stm_publish(shared, staged), 0);
            // Raw words and immediates are never blocks.
            let raw = cinch_stm_cell_new(0x1000, SCAN_RAW as i64);
            assert_eq!(cinch_stm_publish(shared, raw), 0);
            let small = cinch_stm_cell_new(7, SCAN_TOBJ as i64);
            assert_eq!(cinch_stm_publish(shared, small), 0);
            for w in [private, staged, shared, raw, small] {
                rc::cinch_drop(w);
            }
        }
    }

    #[test]
    fn a_commit_resumes_the_waiters_whose_filter_it_touches() {
        let (ready, started) = std::sync::mpsc::channel();
        let waiter = std::thread::spawn(move || {
            cinch_stm_wait_begin(0b0100);
            ready.send(()).unwrap();
            cinch_stm_wait_park(0)
        });
        started.recv().unwrap();
        // A commit to other variables leaves it asleep and counted.
        cinch_stm_wake(0b0011);
        assert!(sleepers().load(Ordering::SeqCst) >= 1);
        assert!(!waiter.is_finished());
        cinch_stm_wake(0b0110);
        assert_eq!(waiter.join().unwrap(), 0);
    }

    #[test]
    fn a_cancelled_wait_leaves_the_registry() {
        std::thread::spawn(|| {
            cinch_stm_wait_begin(1);
            let w = WAITING.with(|slot| slot.borrow().clone()).unwrap();
            assert!(waiters().iter().any(|x| Arc::ptr_eq(x, &w)));
            cinch_stm_wait_cancel();
            assert!(!waiters().iter().any(|x| Arc::ptr_eq(x, &w)));
        })
        .join()
        .unwrap();
    }
}
