//! The task scheduler: structured-concurrency tasks as green threads
//! (design/green-threads.md).
//!
//! # Model
//!
//! A task is a *fiber*: a body with a stack of its own that the runtime
//! can stop in the middle of a call and continue later, on any thread.
//! Fibers are multiplexed over a small set of OS threads:
//!
//! * *Workers* are threads this module starts, lazily, to run fibers and
//!   nothing else. There are never more than the budget allows
//!   ([`crate::par::budget`]: `CINCH_THREADS`, else the available
//!   parallelism), less one for every helping thread.
//! * A *helping thread* runs its own code and is a worker whenever that
//!   code waits: a park on it runs queued fibers until the thread's own
//!   wake arrives. The program's thread is one
//!   ([`crate::platform::cinch_run_main`]). With a budget of 1 it is the
//!   only thread that runs fibers at all, so every task of the program
//!   is suspended and resumed on that one thread.
//! * Any other thread (a parallel pool worker, a thread foreign code
//!   made) is not the scheduler's. It parks as a thread, and the first
//!   fiber such a thread spawns makes one worker allowed whatever the
//!   budget says, because that thread cannot run the fiber itself.
//!
//! There is one run queue, first in first out, behind one lock, and one
//! condition variable idle workers wait on. A fiber runs until it parks,
//! yields or ends; nothing preempts it. The points at which a fiber may
//! give its thread up are the park of [`crate::park`] (a join, a scope's
//! close, a transaction's `retry`, `ioSleep`) and the yield inside
//! `pollCancel` and `ioSleep 0`.
//!
//! Each fiber's stack is a reservation of [`crate::task::TASK_STACK_BYTES`]
//! of address space, committed as it is touched, with an inaccessible
//! band at its low end ([`crate::stack::map_stack`]). A fiber gets its
//! stack when it first runs, not when it is spawned, and a finished
//! fiber's stack goes to the next fiber that starts, so the stacks in
//! use track the fibers that are suspended or running, not the fibers
//! that exist.
//!
//! # The state machine
//!
//! | state | meaning | who moves it on |
//! |-------|---------|-----------------|
//! | `RUNNABLE` | in the run queue (or just taken from it) | the scheduler thread that takes it: `RUNNING` |
//! | `RUNNING` | its code is executing on some thread | the fiber itself: `PARKING` or `DONE` |
//! | `PARKING` | it has decided to park and is switching away | the scheduler thread it ran on: `PARKED` |
//! | `PARKED` | switched out, waiting for a wake | an unpark, or the scheduler thread that parked it: `RUNNABLE` |
//! | `DONE` | its body returned; its stack is recycled | nobody |
//!
//! Beside the state every fiber has a *token*, the wake of
//! [`crate::park`]: `unpark` sets it, and a park that finds it set
//! clears it and does not suspend.
//!
//! # The post-switch protocol
//!
//! A fiber cannot make itself `PARKED`: between that store and the
//! instruction that saves its registers another thread could resume it
//! on a stack it is still using. So the transition is split across the
//! switch, and both writers are sequentially consistent:
//!
//! 1. The fiber finds its token clear, stores `PARKING`, and switches to
//!    the scheduler context of the thread it is on.
//! 2. That thread, now off the fiber's stack with the fiber's registers
//!    saved, stores `PARKED` and then reads the token. If it is set, it
//!    exchanges `PARKED` for `RUNNABLE` and queues the fiber.
//! 3. `unpark` stores the token and then exchanges `PARKED` for
//!    `RUNNABLE`, queuing the fiber if the exchange succeeds.
//!
//! Steps 2 and 3 each store first and read second, so whenever an unpark
//! races a park at least one of them sees the other's store: the wake is
//! never lost. Both may see it, and then exactly one exchange succeeds:
//! the fiber is never queued twice. An unpark that finds `PARKING` only
//! leaves the token, and step 2 queues the fiber at once. The fiber
//! clears its token when it runs again; a wake that arrives after that
//! is kept for its next park, and one that arrived before is covered by
//! the condition the caller rechecks, which its waker set before
//! unparking. A yield is the same path with the token set by the fiber
//! itself, so it goes to the back of the queue.
//!
//! A fiber ends the same way: it stores `DONE`, switches away for the
//! last time, and the thread it ran on recycles the stack it has just
//! left.
//!
//! # State that follows the fiber
//!
//! Some of the runtime's per-thread state is really per task. The
//! scheduler exchanges it at every switch in and out, so a fiber finds
//! its own wherever it resumes and the thread underneath keeps its own:
//!
//! | state | where | follows |
//! |-------|-------|---------|
//! | the running task | `task.rs` `CURRENT_TASK` | the fiber |
//! | the stack fault zone | `stack.rs`, the thread-specific key | the fiber (its own stack's band) |
//! | the entered arena | `arena.rs` `CURRENT` | the fiber |
//! | the quote evaluator's nesting and pending fault | `eval.rs` `TLS` | the fiber |
//! | the running fiber, the scheduler context, the spare stack | this module's `RUN` | the thread |
//! | the small-object heap | `small.rs` `CURRENT`, `TEARDOWN` | the thread |
//! | the parallel pool registration | `par.rs` `LOCAL` | the thread |
//! | a wait being registered | `stm.rs` `WAITING` | the thread: set and taken with no suspension between |
//! | the cooperative ready queue | `coop.rs` `READY` | the thread: that backend has no fibers |
//! | test instrumentation | `heap.rs`, `rc.rs` | the thread |
//!
//! The heap stays with the thread because a heap has one owner at a
//! time and an allocation never spans a switch: a fiber allocates from
//! the heap of whichever thread it is on, and a block freed on another
//! thread goes back the way any cross-thread free does. The parallel
//! pool stays with the thread because a parallel entry joins everything
//! it published before it returns and its jobs are pure, so no fiber
//! switch happens inside one.
//!
//! # The migration rule
//!
//! After a switch a fiber may be on a different OS thread, and the
//! compiler is entitled to compute a thread-local's address once per
//! function. A frame that is live across a park must therefore never
//! reuse one. Two rules keep that true in this crate:
//!
//! * The functions that switch ([`park`], [`park_timeout`],
//!   [`yield_now`]) and every function that reads a thread-local on
//!   behalf of a frame that parks ([`current`], [`blocking_enter`],
//!   [`blocking_exit`], `task::current_task_canceled`, `stm::take_wait`,
//!   the join's hand-over of results) are `#[inline(never)]`: each call
//!   finds the thread it is on.
//! * No frame holds a reference derived from a thread-local across a
//!   call that may park.
//!
//! The scheduler's own frames are exempt because they do not move: a
//! fiber always switches back to the context of the thread it is
//! running on, so the frame that switched into a fiber is resumed on the
//! thread that left it. The allocator is reached through the allocator
//! symbol, which is a call boundary, and finds its heap from the thread
//! register on every call. Generated code has no thread-locals.
//!
//! # Blocking regions
//!
//! A fiber that blocks in the operating system blocks its worker. The
//! promise that a `Blocking` call blocks only its own task is kept by
//! bracketing such a call ([`blocking`], or the two externs generated
//! code calls around a `Blocking` foreign import): while a context is
//! inside a region, one more worker is allowed, and it is started as
//! soon as a fiber is runnable with no thread asleep to take it. When
//! the region ends the allowance shrinks again and a worker that finds
//! itself idle past it exits.
//!
//! # Timers
//!
//! A timed park ([`park_timeout`]) puts the fiber in a queue ordered by
//! deadline and parks it. Every scheduler thread fires the due timers
//! each time it looks for work, and one with no work sleeps no longer
//! than the earliest deadline. A timer is removed when its park returns
//! early.
//!
//! # What is not here
//!
//! There is no I/O reactor: a read that would block blocks its thread,
//! inside a blocking region or not. There is no preemption: a fiber that
//! computes without reaching a park or a yield keeps its thread. There
//! is no work stealing: one queue, one lock. The Wasm profile does not
//! use this module ([`crate::coop`] runs each body to completion on the
//! caller's stack, because that target has no second stack to switch
//! to), and a native target without a context switch in `sched/switch.rs`
//! runs one OS thread per task (`sched/thread.rs`), behind the same
//! entries.

use std::sync::Arc;
use std::time::Duration;

use crate::task::TaskState;

// The one place a backend is chosen. `--cfg cinch_thread_tasks` picks
// the thread backend on a target that has fibers, so the path every
// other target takes can be built and tested on this one.
#[cfg(all(
    not(target_family = "wasm"),
    any(target_arch = "aarch64", target_arch = "x86_64"),
    not(cinch_thread_tasks)
))]
#[path = "sched/fiber.rs"]
mod backend;
#[cfg(not(all(
    not(target_family = "wasm"),
    any(target_arch = "aarch64", target_arch = "x86_64"),
    not(cinch_thread_tasks)
)))]
#[path = "sched/thread.rs"]
mod backend;

pub(crate) use backend::Context;

/// Whether tasks are fibers on this target. Otherwise every task is one
/// OS thread and the entries below are that backend's.
pub const FIBERS: bool = backend::FIBERS;

/// The scheduler's counters, for tests and diagnostics. All zero on a
/// target without fibers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Stats {
    /// Whether tasks are fibers ([`FIBERS`]).
    pub fibers: bool,
    /// Worker threads alive now.
    pub workers: usize,
    /// Worker threads waiting for the run queue to fill.
    pub idle: usize,
    /// Fibers in the run queue.
    pub queued: usize,
    /// Contexts inside a blocking region.
    pub blocked: usize,
    /// Fibers in a timed park.
    pub timers: usize,
    /// Fibers spawned so far.
    pub spawned: u64,
    /// Times a scheduler thread switched into a fiber.
    pub switches: u64,
    /// Worker threads started so far.
    pub threads_started: u64,
    /// Fiber stacks mapped now, in use or kept for reuse.
    pub stacks_mapped: usize,
}

/// The scheduler's counters now.
pub fn stats() -> Stats {
    backend::stats()
}

/// The calling context: the running fiber, the helping thread, or the
/// thread.
pub(crate) fn current() -> Context {
    backend::current()
}

/// Suspend the calling context until its token arrives (the contract of
/// [`crate::park`]). A fiber gives its thread to the next runnable
/// fiber; a helping thread runs fibers until its token arrives.
pub(crate) fn park() {
    backend::park();
}

/// [`park`], returning as well once `timeout` has passed.
pub(crate) fn park_timeout(timeout: Duration) {
    backend::park_timeout(timeout);
}

/// Let another runnable fiber have the processor, if one waits.
pub(crate) fn yield_now() {
    backend::yield_now();
}

/// Run `body` as a task whose state is `task`: the task's context is
/// recorded in it before the body can run, and the body finds `task` as
/// its running task.
pub(crate) fn spawn(task: Arc<TaskState>, body: Box<dyn FnOnce() + Send>) {
    backend::spawn(task, body);
}

/// Run `body`, which blocks the calling thread in the operating system,
/// as a blocking region (module docs).
pub fn blocking<R>(body: impl FnOnce() -> R) -> R {
    backend::blocking_enter();
    let result = body();
    backend::blocking_exit();
    result
}

/// Enter a blocking region: generated code calls it before a foreign
/// import whose row carries `Blocking`. Returns 0.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_sched_blocking_enter() -> i64 {
    backend::blocking_enter();
    0
}

/// Leave the blocking region the matching
/// [`cinch_sched_blocking_enter`] entered. Returns 0.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_sched_blocking_exit() -> i64 {
    backend::blocking_exit();
    0
}

/// Run `body`, the program's own code, with the calling thread
/// registered as a helping thread. The thread must be inside
/// [`crate::stack::guarded`]: a fiber that overflows its stack while
/// this thread runs it is reported on this thread's signal stack.
pub(crate) fn program_thread<R>(body: impl FnOnce() -> R) -> R {
    backend::program_thread(body)
}

/// For a caller that is a fiber, the lowest stack address at which half
/// of its stack still remains; `None` for a thread.
pub(crate) fn help_floor() -> Option<usize> {
    backend::help_floor()
}
