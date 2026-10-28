//! Structured concurrency and runtime I/O (concurrency specification
//! sections 2, 3, 4 and 9).
//!
//! # Execution model
//!
//! Every task is one OS thread (`std::thread`) owned by a structured
//! scope. This is the bootstrap's offload contract for `Blocking` foreign
//! calls (section 9): a blocking call blocks only its task's thread, never
//! a shared event loop. Thread creation and join are the memory model's
//! fork and join points: everything before a spawn happens-before the
//! task body, and everything the body does happens-before the join that
//! observes its completion (section 6; see the module docs of `crate`).
//! The cancellation token is a sequentially consistent atomic, so a
//! request is observed at the next poll point (`cinch_task_poll_cancel`,
//! `cinch_io_sleep`) without a data race.
//!
//! # Task outcome ABI
//!
//! A task body is a Cinch closure applied to the Unit word through the
//! closure apply-stub ABI (`compiler/src/core.rs`: the closure record is
//! `{ code; arity; nfilled; saved[0..] }` and `code` has signature
//! `i64 (ptr self, i64 argc, ptr argv)`). The body returns the prelude
//! `Result e a` word: a block whose tag word is [`TASK_ERR_TAG`] (the
//! prelude `Err`, failure) or [`TASK_OK_TAG`] (the prelude `Ok`, success)
//! and whose single field word is the payload. `joinAllOrCancel` reads
//! this convention: the first failure cancels the siblings and its
//! payload becomes the join's `Err` payload.
//!
//! # Reclamation
//!
//! Scope, task and handle records follow the runtime's leak-by-design
//! policy (`cinch_free` is a no-op until reclamation lands): they are freed
//! at their consumption point where that is exactly known, and leaked
//! otherwise. A handle word is never freed twice.

use std::cell::Cell;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::{Arc, Condvar, Mutex};

/// The prelude `Result` constructor tag of `Ok` (declaration order in
/// `compiler/prelude/Prelude.cinch`).
pub const TASK_OK_TAG: i64 = 1;

/// The prelude `Result` constructor tag of `Err`.
pub const TASK_ERR_TAG: i64 = 0;

/// The Unit word passed to a task body.
const UNIT_WORD: i64 = 0;

/// Task thread stack size: the same order as a process main stack, so
/// ordinary recursive Cinch code behaves the same inside a task.
const TASK_STACK_BYTES: usize = 8 << 20;

/// The closure apply-stub ABI (see `compiler/src/core.rs`).
type ApplyStub = unsafe extern "C" fn(i64, i64, *mut i64) -> i64;

/// Apply a Cinch closure to one argument through its apply stub.
///
/// # Safety
///
/// `closure` must be a well-formed closure record word (the `{ code;
/// arity; nfilled; saved[0..] }` block of `compiler/src/core.rs`) whose
/// stub accepts one argument.
pub unsafe fn apply_closure(closure: i64, arg: i64) -> i64 {
    let code = unsafe { *(closure as *const i64) };
    let stub: ApplyStub = unsafe { std::mem::transmute(code as usize) };
    let mut argv = [arg];
    unsafe { stub(closure, 1, argv.as_mut_ptr()) }
}

/// One spawned task's shared state.
pub struct TaskState {
    /// The cooperative cancellation request (section 4).
    cancel: AtomicBool,
    /// Set once the body has returned; the cleanup acknowledgment.
    done: AtomicBool,
    /// The body's `Result` word, valid once `done` is set.
    result: AtomicI64,
}

impl TaskState {
    fn new() -> TaskState {
        TaskState {
            cancel: AtomicBool::new(false),
            done: AtomicBool::new(false),
            result: AtomicI64::new(0),
        }
    }

    /// Whether cancellation has been requested for this task.
    pub fn is_canceled(&self) -> bool {
        self.cancel.load(Ordering::SeqCst)
    }

    /// Whether the body has returned (cleanup acknowledged).
    pub fn is_done(&self) -> bool {
        self.done.load(Ordering::SeqCst)
    }

    /// The body's `Result` word; only meaningful once [`TaskState::is_done`].
    pub fn result(&self) -> i64 {
        self.result.load(Ordering::SeqCst)
    }
}

/// A structured scope (section 2): the lifetime bound of its tasks.
pub struct Scope {
    inner: Mutex<ScopeInner>,
    cv: Condvar,
}

struct ScopeInner {
    /// Every child spawned into this scope, joined or not.
    children: Vec<Arc<TaskState>>,
}

impl Scope {
    /// An empty scope.
    pub fn new() -> Scope {
        Scope {
            inner: Mutex::new(ScopeInner { children: Vec::new() }),
            cv: Condvar::new(),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, ScopeInner> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Spawn `closure` as a task of this scope; returns the handle word.
    pub fn spawn(self: &Arc<Scope>, closure: i64) -> i64 {
        let state = Arc::new(TaskState::new());
        let handle = Arc::into_raw(state.clone()) as i64;
        self.lock().children.push(state.clone());
        let scope = Arc::clone(self);
        let thread_state = Arc::clone(&state);
        std::thread::Builder::new()
            .stack_size(TASK_STACK_BYTES)
            .spawn(move || {
                CURRENT_TASK.with(|c| c.set(Some(Arc::clone(&thread_state))));
                // Every spawned body runs (section 4: a request is
                // cooperative; effects before the first poll point still
                // happen), and its return is the cleanup acknowledgment.
                let result = unsafe { apply_closure(closure, UNIT_WORD) };
                {
                    let inner = scope.lock();
                    thread_state.result.store(result, Ordering::SeqCst);
                    thread_state.done.store(true, Ordering::SeqCst);
                    drop(inner);
                }
                scope.cv.notify_all();
            })
            .unwrap_or_else(|e| panic!("cinchruntime: task spawn failed: {e}"));
        handle
    }

    /// `joinAllOrCancel` (section 2): wait for every task in `handles`,
    /// requesting cooperative cancellation of the rest once one fails.
    /// Returns 0 when every body returned `Ok` (their payloads in `out`),
    /// or 1 when a body failed (the first `Err` payload in `out[0]`).
    ///
    /// # Safety
    ///
    /// `handles` points at `n` handle words from [`Scope::spawn`]; `out`
    /// points at `n` writable words.
    pub unsafe fn join_all_or_cancel(
        self: &Arc<Scope>,
        handles: *const i64,
        n: usize,
        out: *mut i64,
    ) -> i64 {
        let states: Vec<Arc<TaskState>> = (0..n)
            .map(|i| unsafe {
                let w = *handles.add(i);
                // Handle words carry one reference, leaked by the
                // leak-by-design policy: borrow it without consuming.
                let raw = Arc::from_raw(w as *const TaskState);
                let s = Arc::clone(&raw);
                std::mem::forget(raw);
                s
            })
            .collect();
        let mut inner = self.lock();
        let mut first_err: Option<i64> = None;
        loop {
            let mut all_done = true;
            for s in &states {
                if !s.is_done() {
                    all_done = false;
                    continue;
                }
                let r = s.result();
                if first_err.is_none() && result_tag(r) == TASK_ERR_TAG {
                    first_err = Some(result_payload(r));
                }
            }
            if all_done {
                break;
            }
            // A failure requests sibling cancellation (section 2); the
            // loop still waits for every cleanup acknowledgment before
            // returning, so no task outlives the join.
            if first_err.is_some() {
                for s in &states {
                    if !s.is_done() {
                        s.cancel.store(true, Ordering::SeqCst);
                    }
                }
            }
            inner = self.cv.wait(inner).unwrap_or_else(|e| e.into_inner());
        }
        drop(inner);
        match first_err {
            Some(payload) => {
                unsafe { *out.add(0) = payload };
                1
            }
            None => {
                for (i, s) in states.iter().enumerate() {
                    unsafe { *out.add(i) = result_payload(s.result()) };
                }
                0
            }
        }
    }

    /// Scope exit (section 2): request cancellation of every child that no
    /// join has observed, then wait for each cleanup acknowledgment.
    pub fn close(self: &Arc<Scope>) {
        let mut inner = self.lock();
        loop {
            let mut pending = false;
            for s in &inner.children {
                if !s.is_done() {
                    s.cancel.store(true, Ordering::SeqCst);
                    pending = true;
                }
            }
            if !pending {
                break;
            }
            inner = self.cv.wait(inner).unwrap_or_else(|e| e.into_inner());
        }
    }
}

/// The tag word of a `Result` block.
fn result_tag(result: i64) -> i64 {
    unsafe { *(result as *const i64) }
}

/// The single field word of a `Result` block.
fn result_payload(result: i64) -> i64 {
    unsafe { *((result as *const i64).add(1)) }
}

thread_local! {
    /// The task state of the currently running task body, if any.
    static CURRENT_TASK: Cell<Option<Arc<TaskState>>> = const { Cell::new(None) };
}

/// Cancellation of the running task (section 4): false on the main thread,
/// which belongs to no task.
pub fn current_task_canceled() -> bool {
    CURRENT_TASK.with(|c| {
        let cur = c.take();
        let canceled = match &cur {
            Some(s) => s.is_canceled(),
            None => false,
        };
        c.set(cur);
        canceled
    })
}
