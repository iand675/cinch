//! Standalone control-flow tests for the copied entropy proposal.
//!
//! This wrapper deliberately cannot supply entropy. Any attempt to enter
//! the operating-system boundary fails the test instead of drawing bytes.

/// A panic-only stand-in for the runtime syscall boundary.
mod sys {
    /// Reject accidental real-provider calls during deterministic loop tests.
    pub(crate) fn getrandom(_buf: &mut [u8]) -> (i64, i64) {
        panic!("deterministic wrapper must never enter the entropy provider")
    }
}

/// The exact proposed module, including its original unit tests.
#[path = "../proposed/compiler/runtime/src/entropy.rs"]
mod entropy;
