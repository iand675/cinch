//! The suspension seam: how an execution context stops running until
//! another one wakes it.
//!
//! Everything in the runtime that waits for an event another context
//! produces (a transaction blocked in `retry`, [`crate::stm`]) goes
//! through this module instead of owning a condition variable, so the
//! question "what is suspended, and who resumes it" has one answer.
//!
//! # The contract
//!
//! A [`Parker`] names one execution context. [`current`] is the calling
//! context's, [`park`] suspends the calling context, and
//! [`Parker::unpark`] resumes the context it names. A wake is a token,
//! not a signal:
//!
//! * `unpark` on a context that is not parked leaves one token, and the
//!   context's next `park` consumes it and returns at once. A wake that
//!   races with a park is therefore never lost.
//! * Tokens do not accumulate: two `unpark`s before a `park` leave one.
//! * `park` may return with no `unpark` at all (a spurious wake). Every
//!   caller parks in a loop over its own condition, which it publishes
//!   before parking and the waker sets before unparking.
//!
//! # What a context is
//!
//! Today a context is an OS thread: every task owns one
//! ([`crate::task`]), so parking the context parks the thread. The seam
//! is where a scheduler that multiplexes tasks over fewer threads plugs
//! in: a parked green thread gives its worker to another runnable task,
//! and `unpark` makes it runnable again. Callers do not change, because
//! they only ever name a context and never a thread.
//!
//! The cooperative backend ([`crate::coop`]) has one thread and bodies
//! that run to completion, so it has no context to switch to. A wait
//! there drains the ready queue instead, and one that still cannot
//! proceed is reported rather than slept on ([`crate::stm`]).

/// One execution context that can be resumed.
#[derive(Clone)]
pub struct Parker {
    thread: std::thread::Thread,
}

impl Parker {
    /// Resume the context, or leave it one token if it is not parked.
    pub fn unpark(&self) {
        self.thread.unpark();
    }
}

/// The calling context.
pub fn current() -> Parker {
    Parker { thread: std::thread::current() }
}

/// Suspend the calling context until its token arrives. May return
/// spuriously: the caller rechecks its condition.
pub fn park() {
    std::thread::park();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[test]
    fn a_wake_before_the_park_is_kept_as_a_token() {
        current().unpark();
        // Returns at once: the token is consumed. A hang here is the bug.
        park();
    }

    #[test]
    fn a_parked_context_resumes_when_its_condition_is_set() {
        let ready = Arc::new(AtomicBool::new(false));
        let (send, recv) = std::sync::mpsc::channel();
        let waiter = {
            let ready = Arc::clone(&ready);
            std::thread::spawn(move || {
                send.send(current()).unwrap();
                while !ready.load(Ordering::SeqCst) {
                    park();
                }
            })
        };
        let parker = recv.recv().unwrap();
        ready.store(true, Ordering::SeqCst);
        parker.unpark();
        waiter.join().unwrap();
    }
}
