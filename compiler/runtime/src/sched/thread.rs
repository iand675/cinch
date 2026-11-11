//! The thread backend: one OS thread per task, for targets with no
//! context switch in [`super::switch`]. There is nothing to schedule:
//! the operating system runs every task, a parked context is a parked
//! thread, and a blocking call blocks only the thread of the task that
//! made it. The entries below are the same ones the fiber backend
//! offers, so no caller knows which backend it has.

use std::sync::Arc;
use std::time::Duration;

use super::Stats;
use crate::task::TaskState;

pub(super) const FIBERS: bool = false;

/// One execution context that can be resumed: always a thread here.
#[derive(Clone)]
pub(crate) struct Context(std::thread::Thread);

impl Context {
    /// Resume the context, or leave it one token.
    pub(crate) fn unpark(&self) {
        self.0.unpark();
    }

    /// Whether both name the same context.
    pub(crate) fn is(&self, other: &Context) -> bool {
        self.0.id() == other.0.id()
    }
}

/// The calling context.
pub(crate) fn current() -> Context {
    Context(std::thread::current())
}

/// Suspend the calling context until its token arrives.
pub(crate) fn park() {
    std::thread::park();
}

/// Suspend the calling context until its token arrives or `timeout` has
/// passed.
pub(crate) fn park_timeout(timeout: Duration) {
    std::thread::park_timeout(timeout);
}

/// Every task has a thread of its own, so no task waits for this one to
/// give the processor up.
pub(crate) fn yield_now() {}

/// Run `body` on a thread of its own with `task` as its running task.
/// The thread's stack is guarded like the program's, so overflowing it
/// is the `Exhausted` exit, and a refused thread is one too.
pub(crate) fn spawn(task: Arc<TaskState>, body: Box<dyn FnOnce() + Send>) {
    std::thread::Builder::new()
        .stack_size(crate::task::TASK_STACK_BYTES)
        .spawn(move || {
            task.set_parker(crate::park::Parker::of(current()));
            crate::task::swap_current(Some(task));
            crate::stack::guarded(body)
        })
        .unwrap_or_else(|e| crate::stack::spawn_refused(&e));
}

/// A blocking call already blocks only its own thread.
pub(crate) fn blocking_enter() {}

pub(crate) fn blocking_exit() {}

/// The program's thread is an ordinary thread here.
pub(crate) fn program_thread<R>(body: impl FnOnce() -> R) -> R {
    body()
}

/// No caller is a fiber.
pub(crate) fn help_floor() -> Option<usize> {
    None
}

pub(super) fn stats() -> Stats {
    Stats::default()
}
