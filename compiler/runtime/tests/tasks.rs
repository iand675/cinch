//! Structured-concurrency runtime tests (concurrency specification
//! sections 2 and 4): the task protocol of `compiler/runtime/src/task.rs`
//! through its `extern "C"` surface, with task bodies as stand-in closure
//! records over the closure apply-stub ABI (`compiler/src/core.rs`).

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use cinchruntime::abi::{
    cinch_scope_close, cinch_scope_open, cinch_task_join_all_or_cancel, cinch_task_poll_cancel,
    cinch_task_spawn,
};
use cinchruntime::task::{TASK_ERR_TAG, TASK_OK_TAG};

/// The body's observed the cancellation request.
static OBSERVED: AtomicBool = AtomicBool::new(false);

/// A `Result` block word (`{ tag; payload }`), as the task outcome ABI
/// defines it.
fn result_block(tag: i64, payload: i64) -> i64 {
    Box::into_raw(Box::new([tag, payload])) as usize as i64
}

/// A closure record `{ code; arity; nfilled }` whose apply stub is
/// `stub` (the units' `*.app` stubs have this signature).
fn closure(stub: unsafe extern "C" fn(i64, i64, *mut i64) -> i64) -> i64 {
    Box::into_raw(Box::new([stub as usize as i64, 1, 0])) as usize as i64
}

unsafe extern "C" fn stub_ok_42(_self: i64, _argc: i64, _argv: *mut i64) -> i64 {
    result_block(TASK_OK_TAG, 42)
}

unsafe extern "C" fn stub_ok_7(_self: i64, _argc: i64, _argv: *mut i64) -> i64 {
    result_block(TASK_OK_TAG, 7)
}

unsafe extern "C" fn stub_err_9(_self: i64, _argc: i64, _argv: *mut i64) -> i64 {
    result_block(TASK_ERR_TAG, 9)
}

/// A long-running body: it only returns after observing its
/// cancellation request (section 4's cooperative points).
unsafe extern "C" fn stub_until_canceled(_self: i64, _argc: i64, _argv: *mut i64) -> i64 {
    while cinch_task_poll_cancel() == 0 {
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    OBSERVED.store(true, Ordering::SeqCst);
    result_block(TASK_OK_TAG, 0)
}

#[test]
fn poll_cancel_is_false_on_the_main_thread() {
    assert_eq!(cinch_task_poll_cancel(), 0);
}

#[test]
fn a_spawned_task_result_joins_correctly() {
    let scope = cinch_scope_open();
    let h1 = unsafe { cinch_task_spawn(scope, closure(stub_ok_42)) };
    let h2 = unsafe { cinch_task_spawn(scope, closure(stub_ok_7)) };
    let handles = [h1, h2];
    let mut out = [0i64, 0i64];
    let status = unsafe {
        cinch_task_join_all_or_cancel(
            scope,
            handles.as_ptr() as i64,
            2,
            out.as_mut_ptr() as i64,
        )
    };
    assert_eq!(status, 0);
    assert_eq!(out, [42, 7]);
    unsafe { cinch_scope_close(scope) };
}

#[test]
fn join_all_or_cancel_cancels_the_siblings_on_failure() {
    OBSERVED.store(false, Ordering::SeqCst);
    let scope = cinch_scope_open();
    // The failing body and the sibling that only returns once canceled.
    let fail = unsafe { cinch_task_spawn(scope, closure(stub_err_9)) };
    let spin = unsafe { cinch_task_spawn(scope, closure(stub_until_canceled)) };
    let handles = [fail, spin];
    let mut out = [0i64, 0i64];
    let status = unsafe {
        cinch_task_join_all_or_cancel(
            scope,
            handles.as_ptr() as i64,
            2,
            out.as_mut_ptr() as i64,
        )
    };
    // The first failure's payload is the join's `Err` payload, and the
    // join waited for the sibling's cleanup acknowledgment.
    assert_eq!(status, 1);
    assert_eq!(out[0], 9);
    assert!(OBSERVED.load(Ordering::SeqCst), "the sibling never observed its cancellation request");
    unsafe { cinch_scope_close(scope) };
}

#[test]
fn scope_close_cancels_and_joins_unfinished_children() {
    OBSERVED.store(false, Ordering::SeqCst);
    let scope = cinch_scope_open();
    let _h = unsafe { cinch_task_spawn(scope, closure(stub_until_canceled)) };
    // Leaving the scope follows the cancellation-and-join protocol for
    // the child no join has observed (section 2).
    unsafe { cinch_scope_close(scope) };
    assert!(OBSERVED.load(Ordering::SeqCst), "the child never observed its cancellation request");
}

#[test]
fn handle_and_scope_words_are_distinct_allocations() {
    let scope = cinch_scope_open();
    let h1 = unsafe { cinch_task_spawn(scope, closure(stub_ok_7)) };
    let h2 = unsafe { cinch_task_spawn(scope, closure(stub_ok_7)) };
    assert_ne!(h1, h2);
    assert_ne!(h1, scope);
    let handles = [h1, h2];
    let mut out = [0i64, 0i64];
    let status = unsafe {
        cinch_task_join_all_or_cancel(scope, handles.as_ptr() as i64, 2, out.as_mut_ptr() as i64)
    };
    assert_eq!(status, 0);
    unsafe { cinch_scope_close(scope) };
}
