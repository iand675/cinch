//! Structured concurrency and runtime I/O (concurrency specification
//! sections 2, 3, 4 and 9).
//!
//! # Execution model
//!
//! Every task is a fiber of [`crate::sched`] owned by a structured
//! scope: a body on a stack of its own, run by whichever scheduler
//! thread takes it from the run queue and suspended, not blocked, when
//! it waits (for a child in a join or a close, for a variable in
//! `retry`, in `ioSleep`). A `Blocking` foreign call still blocks only
//! its own task (section 9): the call is a blocking region, and the
//! scheduler starts another worker for the tasks that are runnable
//! meanwhile. Spawn and join are the memory model's fork and join
//! points: everything before a spawn happens-before the task body (the
//! fiber is queued under the scheduler's lock after the spawner's
//! writes, and taken from the queue under it), and everything the body
//! does happens-before the join that observes its completion (section 6;
//! see the module docs of `crate`). The cancellation token is a
//! sequentially consistent atomic, so a request is observed at the next
//! poll point (`cinch_task_poll_cancel`, `cinch_io_sleep`) without a
//! data race, and a request resumes a task suspended at a cancellable
//! wait. On a target with no context switch every task is one OS thread
//! instead, behind the same entries (`sched/thread.rs`).
//!
//! # Waiting for a child
//!
//! A join and a scope's close wait through [`crate::park`], never on a
//! condition variable, so a waiter that is a task gives its thread to
//! other tasks and the program's thread runs tasks while it waits. The
//! waiter registers its context in the scope under the scope's lock and
//! parks; a finishing body stores its result and its done flag under the
//! same lock, takes the registered contexts and unparks them after
//! releasing it. The result and the flag are therefore published before
//! the wake, and a waiter that registered too late to be taken saw the
//! flag instead.
//!
//! # Task outcome ABI
//!
//! A task body is a Cinch closure applied to the Unit word through the
//! closure apply-stub ABI (`compiler/src/core.rs`: the closure record is
//! `{ code; arity; nfilled; saved[0..] }` and `code` has signature
//! `i64 (ptr self, i64 argc, ptr argv)`). The body returns the prelude
//! `Result e a` word: a constructor block whose header tag is
//! [`TASK_ERR_TAG`] (the prelude `Err`, failure) or [`TASK_OK_TAG`] (the
//! prelude `Ok`, success) and whose single field word, at offset 0, is
//! the payload. `joinAllOrCancel` reads this convention: the first
//! failure cancels the siblings and becomes the join's `Err`.
//!
//! The join hands the members' results to generated code in the shape
//! [`join_failed`] and [`join_succeeded`] document: the failure's own
//! `Err` block, the one member's own `Ok` block, or the payloads of
//! several members with a mask saying which of them are counted, so the
//! join unit can build its tuple with the right pointer map. The task
//! bodies' result types are not visible to the join unit; the members'
//! blocks carry them.
//!
//! # Reclamation
//!
//! Values cross between tasks at exactly two points, and the counts
//! follow them (design/reclamation.md section 8):
//!
//! * Spawn. The body closure moves to the task, whose apply stub
//!   consumes it. Before the task can run anywhere, [`Scope::spawn`]
//!   marks everything the closure reaches as atomically counted
//!   ([`crate::rc::cinch_mark_mt`]), since the parent may keep and drop
//!   references to the same blocks while the task runs on another
//!   thread.
//! * Result. A finished body's `Result` word moves to its task state and
//!   from there to the join that hands it out, or, for a child no join
//!   took (a failed join's other members), to the scope's close, which
//!   drops it. Nothing marks it: the body has returned, and its stores of
//!   the result and the done flag (sequentially consistent, under the
//!   scope lock) happen before the joiner's loads of them, so the
//!   joiner's plain counts on the blocks the task built follow every
//!   count operation the task made.
//!
//! Scope and handle words are raw, never counted. Scope records are freed
//! at their one consumption point (`cinch_scope_close`); handle records
//! are leaked by design, so a handle word is never freed twice.
//!
//! All of this applies only to a program that counts references
//! ([`crate::rc::reclaiming`]); otherwise every block leaks, as that
//! program expects.

use std::cell::Cell;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::park::Parker;

/// The prelude `Result` constructor tag of `Ok` (declaration order in
/// `compiler/prelude/Prelude.cinch`).
pub const TASK_OK_TAG: i64 = 1;

/// The prelude `Result` constructor tag of `Err`.
pub const TASK_ERR_TAG: i64 = 0;

/// The Unit word passed to a task body.
const UNIT_WORD: i64 = 0;

/// Task stack size: the same order as a process main stack, so ordinary
/// recursive Cinch code behaves the same inside a task.
pub(crate) const TASK_STACK_BYTES: usize = 8 << 20;

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
    /// The context the body runs in, set before the body can run: a
    /// cancellation request resumes it, so a body suspended at a
    /// cancellable wait observes the request ([`crate::park`]).
    parker: std::sync::OnceLock<Parker>,
}

impl TaskState {
    fn new() -> TaskState {
        TaskState {
            cancel: AtomicBool::new(false),
            done: AtomicBool::new(false),
            result: AtomicI64::new(0),
            parker: std::sync::OnceLock::new(),
        }
    }

    /// Record the context the body runs in. The scheduler calls it once,
    /// before the body can reach a wait.
    pub(crate) fn set_parker(&self, parker: Parker) {
        let _ = self.parker.set(parker);
    }

    /// Request cancellation (section 4) and resume the body if it is
    /// suspended. The flag is set first and is sequentially consistent
    /// with a waiter's own check, so a body either sees the flag before
    /// it parks or is parked when the wake arrives; a wake that reaches a
    /// body before it first parks is kept as its token. Only the first
    /// request wakes: after it the flag is set for good, and every
    /// cancellable wait reads the flag before it suspends.
    fn request_cancel(&self) {
        if !self.cancel.swap(true, Ordering::SeqCst) {
            if let Some(p) = self.parker.get() {
                p.unpark();
            }
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
}

struct ScopeInner {
    /// Every child spawned into this scope, joined or not. The list only
    /// grows.
    children: Vec<Arc<TaskState>>,
    /// How many children have finished with `Err`, so a join looks for a
    /// failure among its members only when there is a new one to find.
    failed: usize,
    /// The contexts parked in a join or the close until a child
    /// finishes. A finishing child takes the whole list.
    waiters: Vec<Parker>,
}

impl Scope {
    /// An empty scope.
    pub fn new() -> Scope {
        Scope {
            inner: Mutex::new(ScopeInner { children: Vec::new(), failed: 0, waiters: Vec::new() }),
        }
    }

    fn lock(&self) -> MutexGuard<'_, ScopeInner> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Spawn `closure` as a task of this scope; returns the handle word.
    /// The closure's reference moves to the task, whose apply stub
    /// consumes it.
    pub fn spawn(self: &Arc<Scope>, closure: i64) -> i64 {
        // Everything the body reaches may now be counted from two threads
        // at once: make it atomic while only this context can reach it.
        // Queueing the task below publishes the marks to the body.
        if crate::rc::reclaiming() {
            unsafe { crate::rc::cinch_mark_mt(closure) };
        }
        let state = Arc::new(TaskState::new());
        let handle = Arc::into_raw(state.clone()) as i64;
        self.lock().children.push(state.clone());
        let scope = Arc::clone(self);
        let task = Arc::clone(&state);
        // Every spawned body runs (section 4: a request is cooperative;
        // effects before the first poll point still happen), and its
        // return is the cleanup acknowledgment. The task's stack is
        // guarded like the program's, so overflowing it is the
        // `Exhausted` exit.
        crate::sched::spawn(
            state,
            Box::new(move || {
                let result = unsafe { apply_closure(closure, UNIT_WORD) };
                scope.finish(&task, result);
            }),
        );
        handle
    }

    /// Publish `task`'s result and wake whoever waits for a child of
    /// this scope (module docs, "Waiting for a child").
    fn finish(&self, task: &TaskState, result: i64) {
        let failed = result_tag(result) == TASK_ERR_TAG;
        let waiters = {
            let mut inner = self.lock();
            task.result.store(result, Ordering::SeqCst);
            task.done.store(true, Ordering::SeqCst);
            inner.failed += usize::from(failed);
            std::mem::take(&mut inner.waiters)
        };
        for waiter in &waiters {
            waiter.unpark();
        }
    }

    /// Suspend `me`, the calling context, until a child of this scope
    /// finishes. `inner` is the lock under which the caller found a
    /// child unfinished, so the child's finish cannot fall between that
    /// check and the registration. May return for another reason (a
    /// cancellation request for the caller's own task): the caller scans
    /// again.
    fn wait_for_child(&self, mut inner: MutexGuard<'_, ScopeInner>, me: &Parker) {
        if !inner.waiters.iter().any(|waiter| waiter.is(me)) {
            inner.waiters.push(me.clone());
        }
        // The lock is not held across the park: the caller may resume on
        // another thread.
        drop(inner);
        crate::park::park();
    }

    /// `joinAllOrCancel` (section 2): wait for every task in `handles`,
    /// requesting cooperative cancellation of the rest once one fails,
    /// and hand the results over ([`join_failed`], [`join_succeeded`]).
    ///
    /// # Safety
    ///
    /// `handles` points at `n` handle words from [`Scope::spawn`], none
    /// joined before; `out` points at `n` writable words.
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
        let first_err = self.await_members(&states);
        unsafe { hand_over(&states, first_err, out) }
    }

    /// Wait until every task in `states` has finished, requesting
    /// cancellation of the rest once one has failed; answers the first
    /// failure seen, by index.
    ///
    /// The waiter is woken once for every child that finishes, so each
    /// pass does work in proportion to what changed, not to the member
    /// count: it advances past the members that have finished since the
    /// last pass, and reads the members' results only when the scope
    /// counts a failure this join has not looked for yet.
    fn await_members(&self, states: &[Arc<TaskState>]) -> Option<usize> {
        let me = crate::park::current();
        let mut first_err: Option<usize> = None;
        // Every member before `finished` has finished.
        let mut finished = 0;
        let mut failures_seen = 0;
        loop {
            let inner = self.lock();
            while finished < states.len() && states[finished].is_done() {
                finished += 1;
            }
            if first_err.is_none() && inner.failed != failures_seen {
                failures_seen = inner.failed;
                first_err = states
                    .iter()
                    .position(|s| s.is_done() && result_tag(s.result()) == TASK_ERR_TAG);
                // A failure requests sibling cancellation (section 2); the
                // loop still waits for every cleanup acknowledgment before
                // returning, so no task outlives the join.
                if first_err.is_some() {
                    for s in &states[finished..] {
                        if !s.is_done() {
                            s.request_cancel();
                        }
                    }
                }
            }
            if finished == states.len() {
                return first_err;
            }
            self.wait_for_child(inner, &me);
        }
    }

    /// Scope exit (section 2): request cancellation of every child that no
    /// join has observed, wait for each cleanup acknowledgment, and drop
    /// the results no join took.
    pub fn close(self: &Arc<Scope>) {
        let unjoined = self.await_children();
        discard_results(&unjoined);
    }

    /// Cancel and wait for every unfinished child; answers the results
    /// no join took, taken out of their task states. Like a join's, each
    /// pass looks only at what is new: the children spawned since the
    /// last pass are asked to cancel, and the scan moves past the ones
    /// that have finished.
    fn await_children(&self) -> Vec<i64> {
        let me = crate::park::current();
        let mut asked = 0;
        let mut finished = 0;
        loop {
            let inner = self.lock();
            for s in &inner.children[asked..] {
                if !s.is_done() {
                    s.request_cancel();
                }
            }
            asked = inner.children.len();
            while finished < inner.children.len() && inner.children[finished].is_done() {
                finished += 1;
            }
            if finished == inner.children.len() {
                return inner.children.iter().map(|s| s.result.swap(0, Ordering::SeqCst)).collect();
            }
            self.wait_for_child(inner, &me);
        }
    }
}

/// Hand a finished join's results to its caller. The join is the one
/// observer of what it hands out, so it takes those results out of their
/// task states. A failure takes only the `Err`: the other members'
/// results stay, and the scope's close drops them.
///
/// Never inlined: it runs after the join's wait, possibly on another
/// thread than the wait began on, and releasing a block reads the
/// thread's heap (the migration rule of [`crate::sched`]).
///
/// # Safety
///
/// Every task in `states` has finished and none was joined before;
/// `out` points at `states.len()` writable words.
#[inline(never)]
unsafe fn hand_over(states: &[Arc<TaskState>], first_err: Option<usize>, out: *mut i64) -> i64 {
    match first_err {
        Some(k) => unsafe { join_failed(states[k].result.swap(0, Ordering::SeqCst), out) },
        None => {
            let results: Vec<i64> = states.iter().map(|s| s.result.swap(0, Ordering::SeqCst)).collect();
            unsafe { join_succeeded(&results, out) }
        }
    }
}

/// Drop the results a scope's close found unjoined. Never inlined, for
/// the reason [`hand_over`] is not.
#[inline(never)]
fn discard_results(results: &[i64]) {
    for r in results {
        unsafe { discard_result(*r) };
    }
}

/// A join's failure return: `out[0]` holds the `Err` block.
pub const JOIN_FAILED: i64 = -1;

/// The success mask reports members `0..JOIN_MASK_BITS`; it stays
/// non-negative, apart from [`JOIN_FAILED`].
pub const JOIN_MASK_BITS: usize = 62;

/// A failed join's hand-over (both backends): `out[0]` is the first
/// failure's own `Err` block, which is the join's result as it stands
/// (its map was built with the error's class), and the return is
/// [`JOIN_FAILED`].
///
/// # Safety
///
/// `err` is an owned `Err` block not used again; `out` points at one
/// writable word.
pub(crate) unsafe fn join_failed(err: i64, out: *mut i64) -> i64 {
    unsafe { *out = err };
    JOIN_FAILED
}

/// A successful join's hand-over (both backends) of the members' `Ok`
/// blocks, which the join now owns:
///
/// * one member: `out[0]` is its own `Ok` block, the join's result as it
///   stands, and the return is 0;
/// * several: `out[i]` is member `i`'s payload, an owned word moved out
///   of its `Ok` block, and the return is a mask whose bit `i` is set
///   when that payload is a counted word (its `Ok` block's map scans it),
///   for `i < JOIN_MASK_BITS`. The join unit picks its tuple's pointer map
///   by the mask. A program that does not count references never
///   releases the tuple, so its mask is 0 and the `Ok` blocks stay
///   allocated like every other block of that program.
///
/// # Safety
///
/// `results` are the members' `Ok` blocks, owned by the caller and not
/// used again; `out` points at `results.len()` writable words.
pub(crate) unsafe fn join_succeeded(results: &[i64], out: *mut i64) -> i64 {
    if let [only] = results {
        unsafe { *out = *only };
        return 0;
    }
    let reclaim = crate::rc::reclaiming();
    let mut mask = 0;
    for (i, r) in results.iter().enumerate() {
        let payload = if reclaim {
            if i < JOIN_MASK_BITS && unsafe { crate::rc::field_counted(*r, 0) } {
                mask |= 1 << i;
            }
            unsafe { crate::rc::take_field(*r, 0) }
        } else {
            result_payload(*r)
        };
        unsafe { *out.add(i) = payload };
    }
    mask
}

/// Drop a task result nobody received (a child no join took, a failed
/// join's other members), when the program counts references.
///
/// # Safety
///
/// `result` is 0 or an owned `Result` word not used again.
pub(crate) unsafe fn discard_result(result: i64) {
    if crate::rc::reclaiming() {
        unsafe { crate::rc::cinch_drop(result) };
    }
}

/// The constructor tag of a `Result` block (in its header's layout id).
fn result_tag(result: i64) -> i64 {
    (unsafe { crate::block::ctor_tag_of(result as usize as *const u8) }) as i64
}

/// The single field word of a `Result` block.
fn result_payload(result: i64) -> i64 {
    unsafe { *(result as *const i64) }
}

thread_local! {
    /// The task state of the task body the thread is running, if any.
    /// The scheduler exchanges it at every switch ([`swap_current`]), so
    /// it is the running fiber's task, not the thread's.
    static CURRENT_TASK: Cell<Option<Arc<TaskState>>> = const { Cell::new(None) };
}

/// Make `task` the calling thread's running task and answer the one it
/// replaces.
pub(crate) fn swap_current(task: Option<Arc<TaskState>>) -> Option<Arc<TaskState>> {
    CURRENT_TASK.with(|c| c.replace(task))
}

/// Cancellation of the running task (section 4): false on the main thread,
/// which belongs to no task.
///
/// Never inlined: a caller that parks between two calls may make them on
/// different threads, and each must read its own thread's slot (the
/// migration rule of [`crate::sched`]).
#[inline(never)]
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
