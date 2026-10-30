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
//! Values cross between threads at exactly two points, and the counts
//! follow them (design/reclamation.md section 8):
//!
//! * Spawn. The body closure moves to the task thread, which consumes it
//!   through the apply stub. Before the thread exists, [`Scope::spawn`]
//!   marks everything the closure reaches as atomically counted
//!   ([`crate::rc::cinch_mark_mt`]), since the parent may keep and drop
//!   references to the same blocks while the task runs.
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
    /// The closure's reference moves to the task thread, whose apply stub
    /// consumes it.
    pub fn spawn(self: &Arc<Scope>, closure: i64) -> i64 {
        // Everything the body reaches may now be counted from two threads
        // at once: make it atomic while only this thread can reach it. The
        // thread's creation below publishes the marks to the body.
        if crate::rc::reclaiming() {
            unsafe { crate::rc::cinch_mark_mt(closure) };
        }
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
                // The task's own stack is guarded like the program's, so
                // overflowing it is the `Exhausted` exit.
                let result = crate::stack::guarded(|| unsafe { apply_closure(closure, UNIT_WORD) });
                {
                    let inner = scope.lock();
                    thread_state.result.store(result, Ordering::SeqCst);
                    thread_state.done.store(true, Ordering::SeqCst);
                    drop(inner);
                }
                scope.cv.notify_all();
            })
            .unwrap_or_else(|e| crate::stack::spawn_refused(&e));
        handle
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
        let mut inner = self.lock();
        let mut first_err: Option<usize> = None;
        loop {
            let mut all_done = true;
            for (i, s) in states.iter().enumerate() {
                if !s.is_done() {
                    all_done = false;
                    continue;
                }
                if first_err.is_none() && result_tag(s.result()) == TASK_ERR_TAG {
                    first_err = Some(i);
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
        // The join is the one observer of what it hands out, so it takes
        // those results out of their task states. A failure takes only the
        // `Err`: the other members' results stay, and the scope's close
        // drops them.
        match first_err {
            Some(k) => unsafe { join_failed(states[k].result.swap(0, Ordering::SeqCst), out) },
            None => {
                let results: Vec<i64> = states.iter().map(|s| s.result.swap(0, Ordering::SeqCst)).collect();
                unsafe { join_succeeded(&results, out) }
            }
        }
    }

    /// Scope exit (section 2): request cancellation of every child that no
    /// join has observed, wait for each cleanup acknowledgment, and drop
    /// the results no join took.
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
        let unjoined: Vec<i64> = inner.children.iter().map(|s| s.result.swap(0, Ordering::SeqCst)).collect();
        drop(inner);
        for r in unjoined {
            unsafe { discard_result(r) };
        }
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
