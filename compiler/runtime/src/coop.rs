//! Cooperative structured concurrency (concurrency specification sections
//! 2, 3, 4 and 9; WebAssembly and WASI specification section 7): the
//! single-threaded task scheduler for the Wasm profile.
//!
//! # Execution model
//!
//! Every task is one logical execution context driven by a cooperative
//! scheduler on the caller's thread (`crate::task` is the OS-thread
//! backend for the same source contract). A task body is a Cinch
//! closure applied to the Unit word through the closure apply-stub ABI
//! (`compiler/src/core.rs`: the closure record is `{ code; arity; nfilled;
//! saved[0..] }` and `code` has signature `i64 (ptr self, i64 argc, ptr
//! argv)`). The body returns the prelude `Result e a` word of the task
//! outcome ABI: a constructor block whose header tag is [`TASK_ERR_TAG`]
//! (the prelude `Err`, failure) or [`TASK_OK_TAG`] (the prelude `Ok`,
//! success) and whose single field word is the payload. Those conventions are
//! `crate::task`'s, reused here verbatim, so a body is identical in both
//! backends. Because the backend is single-threaded its state needs no
//! atomics (the current task is a thread-local pointer), but every
//! observable rule below is the atomics-based one.
//!
//! # Execution policy
//!
//! The interleaving is deterministic: the same spawn sequence and the
//! same bodies always produce the same run.
//!
//! * One FIFO ready queue per thread, shared by every scope on the
//!   thread. `spawn` appends the new task and returns its handle; the
//!   child does not begin inside `spawn`, so the parent runs on to its
//!   own next suspension point (or to completion) and children begin in
//!   spawn order. Everything the parent did before the spawn is visible
//!   to the body when it starts (the fork half of fork-join).
//! * A body, once started, runs to completion. The only handoff points
//!   are the suspension points: `cinch_coop_task_poll_cancel`,
//!   `cinch_coop_io_sleep`, and a body waiting inside
//!   `cinch_coop_task_join_all_or_cancel` or the close path.
//! * At a suspension point the running body yields to the scheduler,
//!   which drains the ready queue: each queued task is taken in FIFO
//!   order and run to completion (its own suspension points recursively
//!   drain the remaining queue first). The yielder resumes only once the
//!   queue is empty, so everything queued at its suspension point
//!   finishes, with its effects visible, before it continues.
//! * `join_all_or_cancel` (and the close path) drains until every task
//!   being awaited has completed and the ready queue is empty. The join
//!   is therefore also a scheduler barrier: every task spawned before it
//!   returns has finished and its effects are visible to the joiner (the
//!   join half of fork-join), which is exactly the guarantee the source
//!   contract makes. The OS-thread backend can return from a join over a
//!   subset of handles while unjoined siblings still run; the memory
//!   model (D80) leaves that cross-task order unordered, and this
//!   backend's stronger barrier is what makes "every child finished
//!   before the join returns" hold for every join.
//! * The first member to complete with `Err` (in scheduler completion
//!   order, including failures that predate the join call) supplies the
//!   join's `Err` payload and immediately requests cancellation of the
//!   unfinished members; the join still waits for every member to finish
//!   before returning. The result word convention is exactly
//!   `task::Scope::join_all_or_cancel`'s: 0 with the `Ok` payloads in
//!   `out`, or 1 with the first failure's payload in `out[0]`.
//! * The close path (close or drop) requests cancellation of every child
//!   no join has observed, then drains the same way, rescanning the child
//!   list as it goes. By the time it returns every child has finished,
//!   every child that reaches a poll point has observed the request, and
//!   nothing remains queued on the thread: a dropped handle is not a
//!   detach (D76), the scope always accounts for every child.
//! * Awaiting a body that is suspended on the call stack (joining one's
//!   own handle, an ancestor's, or a sibling suspended below) can never
//!   complete. Structured concurrency joins children only, so this is a
//!   caller bug; it faults through the generic `cinch_panic` path (exit 71)
//!   instead of spinning forever.
//!
//! # Cancellation
//!
//! A request is a per-task flag (the single-threaded counterpart of the
//! thread backend's sequentially consistent atomic) and is observed ONLY
//! at the two cancellation observation points, `cinch_coop_task_poll_cancel`
//! and `cinch_coop_io_sleep`, and by nothing else (D80). A body that never
//! reaches a poll point runs to completion unaffected by any request,
//! and its outcome is its own `Result` word: the scheduler never
//! fabricates a cancellation outcome. A body that polls observes the
//! request at the poll point itself: the call first yields (so a request
//! made while it yields is observed at that same point) and then reports
//! the flag. `cinch_coop_io_sleep` ends early at its observation point when
//! a request is pending and otherwise completes the requested duration
//! in logical time: this backend has no clock and the memory model
//! exposes no observation of duration, so the millisecond argument is
//! accepted for ABI parity while the call yields once and returns the
//! Unit word 0, exactly what `cinch_io_sleep` returns in the thread model.
//!
//! # Cooperative lowering for Wasm
//!
//! This module is the single-threaded cooperative backend the Wasm
//! profile lowers to. Each task is a logical execution context with
//! handoff only at the defined suspension points. Thread migration,
//! shared-memory atomics and true parallelism are NOT assumed, and no
//! instruction set beyond the baseline is required. The source-level
//! contract (fork-join happens-before at spawn and at join, and
//! cancellation observed only at the poll points) is identical to the
//! OS-thread backend, which is what makes the source contract
//! backend-independent.
//!
//! # Reclamation
//!
//! Scope, task and handle records follow the runtime's leak-by-design
//! policy: a scope word is freed at its one consumption point
//! (`cinch_coop_scope_close`) and handle words carry one leaked
//! reference. A handle word is never freed twice.
//!
//! Task bodies and results move with the same ownership as in
//! `crate::task` (the body to the task, consumed by its apply stub; the
//! result to the join that hands it out, or to the close path, which
//! drops it), and the join hands results over in the same shape
//! (`crate::task::join_failed`, `crate::task::join_succeeded`). Nothing
//! is marked atomic: every task runs on the scheduler's one thread, so no
//! count is ever changed from two threads (design/reclamation.md section
//! 8).

use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::Arc;

use crate::task::{apply_closure, discard_result, join_failed, join_succeeded, TASK_ERR_TAG, TASK_OK_TAG};

/// The Unit word passed to a task body (the same word `crate::task` passes).
const UNIT_WORD: i64 = 0;

/// The constructor tag of a `Result` block (in its header's layout id).
fn result_tag(result: i64) -> i64 {
    (unsafe { crate::block::ctor_tag_of(result as usize as *const u8) }) as i64
}

/// One spawned task's state.
///
/// The observable semantics are `crate::task::TaskState`'s; the fields
/// are `Cell`s because the scheduler never leaves the thread.
pub struct CoopTaskState {
    /// The body closure word, applied to [`UNIT_WORD`] when the task starts.
    body: i64,
    /// The cooperative cancellation request (section 4).
    cancel: Cell<bool>,
    /// Set once the body has returned; the cleanup acknowledgment.
    done: Cell<bool>,
    /// The body's `Result` word, valid once [`CoopTaskState::is_done`].
    result: Cell<i64>,
    /// Completion stamp: the scheduler assigns it when the body returns,
    /// giving completed bodies the total order the first-failure rule
    /// reads.
    seq: Cell<u64>,
}

impl CoopTaskState {
    fn new(body: i64) -> CoopTaskState {
        CoopTaskState {
            body,
            cancel: Cell::new(false),
            done: Cell::new(false),
            result: Cell::new(0),
            seq: Cell::new(0),
        }
    }

    /// Whether cancellation has been requested for this task.
    pub fn is_canceled(&self) -> bool {
        self.cancel.get()
    }

    /// Whether the body has returned (cleanup acknowledged).
    pub fn is_done(&self) -> bool {
        self.done.get()
    }

    /// The body's `Result` word; only meaningful once [`CoopTaskState::is_done`].
    pub fn result(&self) -> i64 {
        self.result.get()
    }

    /// The body's completion stamp; only meaningful once [`CoopTaskState::is_done`].
    fn seq(&self) -> u64 {
        self.seq.get()
    }
}

/// A structured scope (section 2): the lifetime bound of its tasks.
///
/// Dropping the scope runs its close path (below): no child is left
/// running when the scope disappears.
pub struct CoopScope {
    /// Every child spawned into this scope, in spawn order, joined or not.
    children: RefCell<Vec<Arc<CoopTaskState>>>,
}

impl CoopScope {
    /// An empty scope.
    pub fn new() -> CoopScope {
        CoopScope {
            children: RefCell::new(Vec::new()),
        }
    }

    /// Spawn `body_word` as a task of this scope; returns the handle word
    /// (one leaked `Arc` reference, as in `crate::task::Scope::spawn`).
    ///
    /// The child is appended to the ready queue and starts at the next
    /// scheduler drain, never inside this call (see the execution policy).
    pub fn spawn(&self, body_word: i64) -> i64 {
        let state = Arc::new(CoopTaskState::new(body_word));
        let handle = Arc::into_raw(Arc::clone(&state)) as i64;
        self.children.borrow_mut().push(Arc::clone(&state));
        READY.with(|q| q.borrow_mut().push_back(state));
        handle
    }

    /// `joinAllOrCancel` (section 2): run the scheduler until every task
    /// in `handles` has completed and the ready queue is empty, requesting
    /// cooperative cancellation of the members once one fails, and hand
    /// the results over as `crate::task::Scope::join_all_or_cancel` does.
    ///
    /// # Safety
    ///
    /// `handles` points at `n` handle words from [`CoopScope::spawn`],
    /// none joined before; `out` points at `n` writable words.
    pub unsafe fn join_all_or_cancel(
        &self,
        handles: *const i64,
        n: usize,
        out: *mut i64,
    ) -> i64 {
        let members: Vec<Arc<CoopTaskState>> = (0..n)
            .map(|i| handle_state(unsafe { *handles.add(i) }))
            .collect();
        let set = Rc::new(JoinSet {
            members,
            first_err: Cell::new(None),
        });
        // Failures that predate this call follow the same rule.
        set.refresh();
        JOINS.with(|j| j.borrow_mut().push(Rc::clone(&set)));
        await_all(&set.members);
        JOINS.with(|j| {
            let popped = j.borrow_mut().pop();
            debug_assert!(popped.map_or(false, |p| Rc::ptr_eq(&p, &set)));
        });
        // The join is the one observer of what it hands out, so it takes
        // those results out of their task states. A failure takes only the
        // `Err`: the other members' results stay, and the close path drops
        // them.
        if let Some(k) = set.first_err.get() {
            return unsafe { join_failed(set.members[k].result.replace(0), out) };
        }
        let results: Vec<i64> = set.members.iter().map(|s| s.result.replace(0)).collect();
        // `first_err` is None exactly when no member carries the `Err`
        // tag.
        debug_assert!(results.iter().all(|r| result_tag(*r) == TASK_OK_TAG));
        unsafe { join_succeeded(&results, out) }
    }

    /// Scope exit (section 2): request cancellation of every child that no
    /// join has observed, then run the scheduler until each cleanup
    /// acknowledgment has arrived, and drop the results no join took. By
    /// return every child has finished and nothing remains queued on the
    /// thread.
    pub fn close(&self) {
        loop {
            let mut pending = Vec::new();
            for c in self.children.borrow().iter() {
                if !c.is_done() {
                    c.cancel.set(true);
                    pending.push(Arc::clone(c));
                }
            }
            if pending.is_empty() && queue_empty() {
                break;
            }
            if !pump_one() {
                no_progress_fault();
            }
        }
        let unjoined: Vec<i64> = self.children.borrow().iter().map(|c| c.result.replace(0)).collect();
        for r in unjoined {
            unsafe { discard_result(r) };
        }
    }
}

impl Drop for CoopScope {
    fn drop(&mut self) {
        self.close();
    }
}

/// A `joinAllOrCancel` wait set: its members and their first failure.
struct JoinSet {
    /// The joined tasks, in handle order.
    members: Vec<Arc<CoopTaskState>>,
    /// The index of the first member to fail in completion order.
    first_err: Cell<Option<usize>>,
}

impl JoinSet {
    /// Observe failures among the members: the first one in completion
    /// order becomes the join's `Err` and requests cancellation of the
    /// unfinished members (the first-failure-cancels rule of section 2).
    /// Runs at join entry (for failures that predate the call) and after
    /// every task completion while the join is active.
    fn refresh(&self) {
        if self.first_err.get().is_some() {
            return;
        }
        let first = self
            .members
            .iter()
            .enumerate()
            .filter(|(_, s)| s.is_done() && result_tag(s.result()) == TASK_ERR_TAG)
            .min_by_key(|(_, s)| s.seq());
        if let Some((i, _)) = first {
            self.first_err.set(Some(i));
            for m in &self.members {
                if !m.is_done() {
                    m.cancel.set(true);
                }
            }
        }
    }

    /// [`JoinSet::refresh`] when `task` belongs to this wait set.
    fn on_completion(&self, task: &Arc<CoopTaskState>) {
        if self.members.iter().any(|m| Arc::ptr_eq(m, task)) {
            self.refresh();
        }
    }
}

thread_local! {
    /// The ready queue: every spawned task that has not begun running, in
    /// spawn order (the scheduler's single FIFO queue for the thread).
    static READY: RefCell<VecDeque<Arc<CoopTaskState>>> = const { RefCell::new(VecDeque::new()) };

    /// The stack of join wait sets currently draining the scheduler.
    static JOINS: RefCell<Vec<Rc<JoinSet>>> = const { RefCell::new(Vec::new()) };

    /// The task state of the currently running task body, if any.
    static CURRENT: Cell<Option<Arc<CoopTaskState>>> = const { Cell::new(None) };

    /// Monotonic completion stamps: a total order on body completions.
    static SEQ: Cell<u64> = const { Cell::new(0) };
}

/// Borrow the task state of a handle word from [`CoopScope::spawn`].
fn handle_state(word: i64) -> Arc<CoopTaskState> {
    // Handle words carry one reference, leaked by the leak-by-design
    // policy (see the module docs): borrow it without consuming.
    let raw = unsafe { Arc::from_raw(word as *const CoopTaskState) };
    let state = Arc::clone(&raw);
    std::mem::forget(raw);
    state
}

/// Whether the ready queue holds no task.
fn queue_empty() -> bool {
    READY.with(|q| q.borrow().is_empty())
}

/// Run the body of the next queued task to completion; false when the
/// queue is empty.
fn pump_one() -> bool {
    let next = READY.with(|q| q.borrow_mut().pop_front());
    match next {
        Some(state) => {
            run_task(&state);
            true
        }
        None => false,
    }
}

/// Run `state`'s body to completion and record its outcome.
fn run_task(state: &Arc<CoopTaskState>) {
    let previous = CURRENT.with(|c| c.take());
    CURRENT.with(|c| c.set(Some(Arc::clone(state))));
    // Every spawned body runs (section 4: a request is cooperative;
    // effects before the first poll point still happen), and its return
    // is the cleanup acknowledgment. Its suspension points drain the rest
    // of the ready queue before it resumes (execution policy).
    let result = unsafe { apply_closure(state.body, UNIT_WORD) };
    CURRENT.with(|c| c.set(previous));
    let seq = SEQ.with(|s| {
        let n = s.get();
        s.set(n + 1);
        n
    });
    state.seq.set(seq);
    state.result.set(result);
    state.done.set(true);
    // A failure is observed the moment it completes: every active join
    // that includes this task records the first failure and requests
    // cancellation of its remaining members (section 2).
    JOINS.with(|j| {
        for set in j.borrow().iter() {
            set.on_completion(state);
        }
    });
}

/// Run the scheduler until every task in `awaited` has completed and the
/// ready queue is empty (the fork-join barrier of the execution policy).
fn await_all(awaited: &[Arc<CoopTaskState>]) {
    while !awaited.iter().all(|s| s.is_done()) || !queue_empty() {
        if !pump_one() {
            no_progress_fault();
        }
    }
}

/// The awaited task is suspended on the call stack (a yield below the
/// awaiter, or the awaiter itself): it cannot return while the awaiter
/// waits, and the scheduler has nothing left to run. Structured
/// concurrency joins children only, so this is a caller bug; fault
/// through the generic path (exit 71) instead of hanging.
fn no_progress_fault() -> ! {
    const MSG: &[u8] = b"cinchruntime: cooperative scheduler deadlock: awaiting a task that is suspended on the call stack";
    unsafe { crate::abi::cinch_panic(MSG.as_ptr(), MSG.len()) }
}

/// Cancellation of the running task (section 4): false on the root
/// context (the thread outside any task body), which belongs to no task.
/// The observable semantics are `crate::task::current_task_canceled`'s.
pub fn current_task_canceled() -> bool {
    CURRENT.with(|c| {
        let cur = c.take();
        let canceled = match &cur {
            Some(s) => s.is_canceled(),
            None => false,
        };
        c.set(cur);
        canceled
    })
}

/// The cancellation observation point, and a suspension point: yield to
/// the scheduler first (so a request made while yielding is observed at
/// this point), then report the running task's request.
pub fn poll_cancel() -> bool {
    drain();
    current_task_canceled()
}

/// The `ioSleep` suspension point (section 4). The duration completes in
/// logical time: the call yields to the scheduler once and returns the
/// Unit word 0, ending early at this observation point when a request is
/// pending (`_millis` is accepted for ABI parity with `cinch_io_sleep`).
pub fn io_sleep(_millis: i64) -> i64 {
    drain();
    if current_task_canceled() {
        return 0;
    }
    0
}

/// The suspension-point yield: hand the thread to the scheduler until the
/// ready queue is empty.
fn drain() {
    while pump_one() {}
}

// -------------------------------------------- cooperative structured concurrency (P14)
//
// The same word conventions as the `cinch_*` surface of `crate::abi`: the
// scope word points at a boxed `Arc<CoopScope>` (one reference consumed
// by `cinch_coop_scope_close`); a handle word is an `Arc<CoopTaskState>`
// reference (never freed twice; see the module docs). The task outcome
// and closure ABI conventions are `crate::task`'s.

/// Open a structured scope: the `withScope` entry (concurrency
/// specification section 2).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_coop_scope_open() -> i64 {
    let scope = Arc::new(CoopScope::new());
    Box::into_raw(Box::new(scope)) as usize as i64
}

/// Scope exit: cancel every unjoined child, wait for each cleanup
/// acknowledgment, and release the scope.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_coop_scope_close(scope: i64) -> i64 {
    let owned = unsafe { Box::from_raw(scope as *mut Arc<CoopScope>) };
    owned.close();
    drop(owned);
    0
}

/// Spawn the body closure as a task of `scope`; returns the task handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_coop_task_spawn(scope: i64, closure: i64) -> i64 {
    let scope = unsafe { &*(scope as *const Arc<CoopScope>) };
    scope.spawn(closure)
}

/// `joinAllOrCancel` over the `n` handles at `handles`: the hand-over of
/// `cinch_task_join_all_or_cancel` (a negative return is a failure, with
/// its `Err` block in `out[0]`).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_coop_task_join_all_or_cancel(
    scope: i64,
    handles: i64,
    n: i64,
    out: i64,
) -> i64 {
    if n < 0 {
        crate::abi::raise(crate::error::Error::OutOfBounds);
    }
    let scope = unsafe { &*(scope as *const Arc<CoopScope>) };
    unsafe { scope.join_all_or_cancel(handles as *const i64, n as usize, out as *mut i64) }
}

/// The cancellation observation point (section 4): 1 once cancellation
/// has been requested for the running task, else 0.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_coop_task_poll_cancel() -> i64 {
    i64::from(poll_cancel())
}

/// Sleep `millis` milliseconds as a suspension point: returns early with
/// 0 (the Unit word) once cancellation is requested for the running task
/// (concurrency specification section 4).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_coop_io_sleep(millis: i64) -> i64 {
    io_sleep(millis)
}
