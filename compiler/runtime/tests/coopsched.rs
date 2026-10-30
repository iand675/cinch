//! Cooperative structured-concurrency tests (concurrency specification
//! sections 2 and 4; WebAssembly and WASI specification section 7): the
//! cooperative task protocol of `compiler/runtime/src/coop.rs` through
//! its `extern "C"` surface, with task bodies as stand-in closure records
//! over the closure apply-stub ABI (`compiler/src/core.rs`). The
//! scheduler is single-threaded and deterministic, so every body runs on
//! the test's own thread and the tests drive it synchronously: nothing
//! here waits on wall-clock time.

use std::cell::{Cell, RefCell};

use cinchruntime::coop::{
    cinch_coop_io_sleep, cinch_coop_scope_close, cinch_coop_scope_open, cinch_coop_task_join_all_or_cancel,
    cinch_coop_task_poll_cancel, cinch_coop_task_spawn,
};
use cinchruntime::task::{TASK_ERR_TAG, TASK_OK_TAG};

/// The apply-stub signature of a body closure record.
type Stub = unsafe extern "C" fn(i64, i64, *mut i64) -> i64;

thread_local! {
    /// The effects the bodies performed, in scheduler order.
    static TRACE: RefCell<Vec<i64>> = const { RefCell::new(Vec::new()) };

    /// Bit mask of bodies that ran to completion: the per-child
    /// completion flags checked after joins and closes.
    static DONE: Cell<u64> = const { Cell::new(0) };

    /// The parent/child shared variable of the happens-before test.
    static VAR: Cell<i64> = const { Cell::new(0) };
}

/// The trace marker of a body's first effect (`bit` identifies the body).
fn code_start(bit: i64) -> i64 {
    10 + bit
}

/// The trace marker of a body's cancellation observation at a poll point.
fn code_obs(bit: i64) -> i64 {
    30 + bit
}

fn trace() -> Vec<i64> {
    TRACE.with(|t| t.borrow().clone())
}

fn done() -> u64 {
    DONE.with(|d| d.get())
}

fn reset_state() {
    TRACE.with(|t| t.borrow_mut().clear());
    DONE.with(|d| d.set(0));
    VAR.with(|v| v.set(0));
}

/// A `Result` block word (`{ payload }` with the tag in the header's
/// layout id), as the task outcome ABI defines it.
fn result_block(tag: i64, payload: i64) -> i64 {
    use cinchruntime::block::{ctor_id, static_header, KIND_CTOR};
    let hdr = static_header(KIND_CTOR, ctor_id(0, tag as u64) as u16) as i64;
    Box::into_raw(Box::new([hdr, payload])) as usize as i64 + 8
}

/// A closure record `{ code; arity; nfilled }` whose apply stub is `stub`.
fn closure(stub: Stub) -> i64 {
    Box::into_raw(Box::new([stub as usize as i64, 1, 0])) as usize as i64
}

/// A closure record with one saved word: `{ code; arity; nfilled; s0 }`.
fn closure1(stub: Stub, s0: i64) -> i64 {
    Box::into_raw(Box::new([stub as usize as i64, 1, 1, s0])) as usize as i64
}

/// A closure record with two saved words: `{ code; arity; nfilled; s0; s1 }`.
fn closure2(stub: Stub, s0: i64, s1: i64) -> i64 {
    Box::into_raw(Box::new([stub as usize as i64, 1, 2, s0, s1])) as usize as i64
}

/// Saved word `index` of the body's closure record.
fn saved(self_word: i64, index: usize) -> i64 {
    unsafe { *(self_word as *const i64).add(3 + index) }
}

fn spawn(scope: i64, stub: Stub) -> i64 {
    unsafe { cinch_coop_task_spawn(scope, closure(stub)) }
}

fn spawn1(scope: i64, stub: Stub, s0: i64) -> i64 {
    unsafe { cinch_coop_task_spawn(scope, closure1(stub, s0)) }
}

fn spawn2(scope: i64, stub: Stub, s0: i64, s1: i64) -> i64 {
    unsafe { cinch_coop_task_spawn(scope, closure2(stub, s0, s1)) }
}

/// `joinAllOrCancel` through the C ABI: the status word and the `out` block.
fn join(scope: i64, handles: &[i64]) -> (i64, Vec<i64>) {
    let mut out = vec![0i64; handles.len()];
    let status = unsafe {
        cinch_coop_task_join_all_or_cancel(
            scope,
            handles.as_ptr() as i64,
            handles.len() as i64,
            out.as_mut_ptr() as i64,
        )
    };
    (status, out)
}

/// `Ok` of the thread-local variable: the parent's pre-spawn effect.
unsafe extern "C" fn stub_ok_var(_self: i64, _argc: i64, _argv: *mut i64) -> i64 {
    result_block(TASK_OK_TAG, VAR.with(|v| v.get()))
}

/// Sets the thread-local variable (the child's effect) and returns `Ok 0`.
unsafe extern "C" fn stub_set_var(self_word: i64, _argc: i64, _argv: *mut i64) -> i64 {
    VAR.with(|v| v.set(saved(self_word, 0)));
    result_block(TASK_OK_TAG, 0)
}

/// `Err` of its saved payload.
unsafe extern "C" fn stub_err_saved(self_word: i64, _argc: i64, _argv: *mut i64) -> i64 {
    result_block(TASK_ERR_TAG, saved(self_word, 0))
}

/// Records its completion flag and returns `Ok` of its payload. It never
/// reaches a poll point, so no cancellation request can affect its run or
/// its outcome.
unsafe extern "C" fn stub_mark_ok(self_word: i64, _argc: i64, _argv: *mut i64) -> i64 {
    let (bit, payload) = (saved(self_word, 0), saved(self_word, 1));
    DONE.with(|d| d.set(d.get() | 1u64 << bit));
    result_block(TASK_OK_TAG, payload)
}

/// Records its start, polls at the cancellation observation point until
/// the request is observed (only there), records the observation, and
/// returns `Ok` of its payload. The loop is bounded only so that a
/// scheduler failing to deliver the request fails the test's assertion
/// instead of hanging the run.
unsafe extern "C" fn stub_spin_ok(self_word: i64, _argc: i64, _argv: *mut i64) -> i64 {
    let (bit, payload) = (saved(self_word, 0), saved(self_word, 1));
    TRACE.with(|t| t.borrow_mut().push(code_start(bit)));
    for _ in 0..64 {
        if cinch_coop_task_poll_cancel() != 0 {
            TRACE.with(|t| t.borrow_mut().push(code_obs(bit)));
            break;
        }
    }
    DONE.with(|d| d.set(d.get() | 1u64 << bit));
    result_block(TASK_OK_TAG, payload)
}

/// Records both markers and returns `Ok 0`: no suspension between them.
unsafe extern "C" fn stub_trail(self_word: i64, _argc: i64, _argv: *mut i64) -> i64 {
    TRACE.with(|t| {
        let mut t = t.borrow_mut();
        t.push(saved(self_word, 0));
        t.push(saved(self_word, 1));
    });
    result_block(TASK_OK_TAG, 0)
}

/// Records its markers around an `ioSleep` suspension and returns `Ok 0`.
unsafe extern "C" fn stub_sleep_trail(self_word: i64, _argc: i64, _argv: *mut i64) -> i64 {
    TRACE.with(|t| t.borrow_mut().push(saved(self_word, 0)));
    cinch_coop_io_sleep(0);
    TRACE.with(|t| t.borrow_mut().push(saved(self_word, 1)));
    result_block(TASK_OK_TAG, 0)
}

/// Records the words `pollCancel` and `ioSleep` return and returns `Ok 0`.
unsafe extern "C" fn stub_probe_words(_self: i64, _argc: i64, _argv: *mut i64) -> i64 {
    let polled = cinch_coop_task_poll_cancel();
    let slept = cinch_coop_io_sleep(7);
    TRACE.with(|t| t.borrow_mut().extend([polled, slept]));
    result_block(TASK_OK_TAG, 0)
}

#[test]
fn fork_join_happens_before_at_spawn_and_join() {
    reset_state();
    VAR.with(|v| v.set(7));
    let scope = cinch_coop_scope_open();
    let echo = spawn(scope, stub_ok_var);
    let setter = spawn1(scope, stub_set_var, 99);
    let (status, out) = join(scope, &[echo, setter]);
    // The parent's effect before the spawn is what the child read ...
    assert_eq!(status, 0);
    assert_eq!(out, vec![7, 0]);
    // ... and the child's effect is visible to the parent by the time the
    // join returns.
    assert_eq!(VAR.with(|v| v.get()), 99);
    unsafe { cinch_coop_scope_close(scope) };
}

#[test]
fn a_child_that_never_polls_keeps_its_own_outcome_under_cancellation() {
    reset_state();
    let scope = cinch_coop_scope_open();
    // The failing body runs first (spawn order), so its failure requests
    // cancellation of the sibling while that sibling is still queued.
    let failing = spawn1(scope, stub_err_saved, 9);
    let stubborn = spawn2(scope, stub_mark_ok, 0, 42);
    let (status, out) = join(scope, &[failing, stubborn]);
    assert_eq!(status, 1);
    assert_eq!(out[0], 9);
    // The request was pending the whole time the sibling ran, but the body
    // never reaches a poll point: it ran to completion (its completion
    // flag) and the join's error payload is the failure's, nothing
    // fabricated for the canceled sibling.
    assert_eq!(done(), 1);
    // The sibling's recorded outcome is its own `Ok` word.
    let (status, out) = join(scope, &[stubborn]);
    assert_eq!(status, 0);
    assert_eq!(out, vec![42]);
    unsafe { cinch_coop_scope_close(scope) };
}

#[test]
fn a_child_observes_cancellation_at_a_poll_point_only() {
    reset_state();
    let scope = cinch_coop_scope_open();
    // The observer starts first and suspends at its first poll point; the
    // failing sibling runs during that suspension and requests
    // cancellation, which the observer then observes at that poll point.
    let observer = spawn2(scope, stub_spin_ok, 0, 1);
    let failing = spawn1(scope, stub_err_saved, 9);
    let (status, out) = join(scope, &[observer, failing]);
    assert_eq!(status, 1);
    assert_eq!(out[0], 9);
    // The body's first effect ran before the request (its start marker
    // comes first) and the request was observed at the poll point itself
    // (its observation marker): nothing outside the poll points saw it.
    assert_eq!(trace(), vec![code_start(0), code_obs(0)]);
    assert_eq!(done(), 1);
    // The outcome recorded for the observer is its own `Ok` word.
    let (status, out) = join(scope, &[observer]);
    assert_eq!(status, 0);
    assert_eq!(out, vec![1]);
    unsafe { cinch_coop_scope_close(scope) };
}

#[test]
fn first_failure_cancels_the_siblings_and_the_join_awaits_all() {
    reset_state();
    let scope = cinch_coop_scope_open();
    let spinner = spawn2(scope, stub_spin_ok, 0, 1);
    let failing = spawn1(scope, stub_err_saved, 9);
    let stubborn = spawn2(scope, stub_mark_ok, 2, 3);
    let (status, out) = join(scope, &[spinner, failing, stubborn]);
    // The failure's payload is the join's `Err` payload ...
    assert_eq!(status, 1);
    assert_eq!(out[0], 9);
    // ... and by the time the join returns every other child has finished
    // (per-child completion flags): the spinner observed its request and
    // acknowledged it, and the sibling that never polls ran to completion.
    assert_eq!(done(), 0b101);
    assert!(trace().contains(&code_obs(0)));
    unsafe { cinch_coop_scope_close(scope) };
}

#[test]
fn dropped_handles_are_not_detached_and_close_leaves_nothing_queued() {
    reset_state();
    let scope = cinch_coop_scope_open();
    // The handles are never joined: at source level they are dropped, and
    // dropping a handle must not silently detach the task.
    let _dropped_a = spawn2(scope, stub_mark_ok, 0, 1);
    let _dropped_b = spawn2(scope, stub_spin_ok, 1, 2);
    unsafe { cinch_coop_scope_close(scope) };
    // The close path ran both children to completion (the spinner through
    // its cancellation acknowledgment at a poll point).
    assert_eq!(done(), 0b11);
    assert!(trace().contains(&code_obs(1)));
    // Nothing remains queued after close: the root context's poll is a
    // suspension point, so any queued task would have run and moved the
    // trace.
    let before = trace();
    assert_eq!(cinch_coop_task_poll_cancel(), 0);
    assert_eq!(trace(), before);
}

#[test]
fn the_spawn_sequence_fixes_the_interleaving() {
    // Documented policy: children begin in spawn order, and a body that
    // suspends completes after everything queued at its suspension point.
    // So `a` (spawned first) suspends in the middle, `b` and `c` run to
    // completion inside that suspension, and `a` completes last: the
    // markers arrive as 1, 2, 3, 4, 5, 6 in every run.
    let expected = vec![1, 2, 3, 4, 5, 6];
    for round in 0..3 {
        reset_state();
        let scope = cinch_coop_scope_open();
        let a = spawn2(scope, stub_sleep_trail, 1, 6);
        let b = spawn2(scope, stub_trail, 2, 3);
        let c = spawn2(scope, stub_trail, 4, 5);
        let (status, _) = join(scope, &[a, b, c]);
        assert_eq!(status, 0, "round {round}");
        assert_eq!(trace(), expected, "round {round}");
        unsafe { cinch_coop_scope_close(scope) };
    }
}

#[test]
fn poll_cancel_and_io_sleep_return_the_documented_words() {
    reset_state();
    // The root context belongs to no task: `pollCancel` is false there
    // and both calls are ordinary yields.
    assert_eq!(cinch_coop_task_poll_cancel(), 0);
    assert_eq!(cinch_coop_io_sleep(0), 0);
    // Inside a task with no request pending: `pollCancel` returns 0 and
    // `ioSleep` returns the Unit word 0.
    let scope = cinch_coop_scope_open();
    let probe = spawn(scope, stub_probe_words);
    let (status, out) = join(scope, &[probe]);
    assert_eq!(status, 0);
    assert_eq!(out, vec![0]);
    assert_eq!(trace(), vec![0, 0]);
    unsafe { cinch_coop_scope_close(scope) };
    // With a request pending (the failure runs first and cancels the
    // probe): `pollCancel` returns 1 at its observation point and
    // `ioSleep` still returns the Unit word 0, ending early there.
    reset_state();
    let scope = cinch_coop_scope_open();
    let failing = spawn1(scope, stub_err_saved, 9);
    let probe = spawn(scope, stub_probe_words);
    let (status, out) = join(scope, &[failing, probe]);
    assert_eq!(status, 1);
    assert_eq!(out[0], 9);
    assert_eq!(trace(), vec![1, 0]);
    unsafe { cinch_coop_scope_close(scope) };
}
