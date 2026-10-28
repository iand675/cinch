//! Runtime error type shared by the safe Rust API.
//!
//! The extern "C" layer maps these errors onto defined runtime faults: the
//! process exits with the code returned by [`Error::exit_code`] after writing
//! a message to stderr. See the module docs of `crate` for the exit code
//! table.

use std::fmt;

/// Errors reported by the safe Rust API of the runtime.
///
/// Every variant corresponds to a defined runtime fault at the extern "C"
/// layer; nothing in this crate produces undefined behaviour on bad input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// A natural number was required but the input was negative
    /// (`Nat::from_i64` on a negative literal).
    NegativeInput,
    /// `Nat` subtraction produced a negative result.
    NegativeResult,
    /// Division or remainder by zero.
    DivByZero,
    /// The value does not fit in an `i64`.
    OutOfRange,
    /// Decimal text is not a well-formed numeral.
    MalformedDecimal,
    /// The output buffer is too small to hold the decimal digits.
    BufferTooSmall,
    /// A symbol name that is not valid UTF-8 reached `cinch_sym_intern`
    /// (primitive types specification section 9: `intern` takes a
    /// `String`, so every interned name is valid UTF-8 and `sym-name` is
    /// total).
    MalformedName,
    /// A byte-buffer index, length, or byte value was outside its allowed
    /// range (`Error::OutOfBounds` from [`crate::bytes`]).
    OutOfBounds,
    /// A write reached a read-only buffer (a string literal's constant
    /// data-section backing): the runtime takes the defined fault
    /// instead of writing to constant data.
    ReadOnly,
    /// An extern entry point received a null word or a word whose block kind
    /// does not match the entry point (corrupt generated code or data),
    /// including a quotation block or program stream that is malformed
    /// ([`crate::eval`]).
    CorruptValue,
    /// The quoted program raised its own `Fault`, or a memo cell was
    /// demanded while its evaluation was still running (the `BlackHole`
    /// fault of [`crate::eval`], the same fault as `cinch_blackhole`).
    BlackHole,
    /// A quotation is nested deeper than [`crate::eval::MAX_NESTING`]
    /// units of evaluator recursion (through its expression tree, its
    /// captured holes, or both); the evaluator refuses it instead of
    /// exhausting the native stack (resource exhaustion is a named
    /// failure mode).
    NestingTooDeep,
}

impl Error {
    /// Process exit code used when this error surfaces as a runtime fault.
    ///
    /// [`Error::DivByZero`] uses the dedicated division fault code (the same
    /// code `cinch_fault_div_zero` exits with) and [`Error::BlackHole`] the
    /// `BlackHole` code (the same code `cinch_blackhole` exits with); every
    /// other variant exits through the `cinch_panic` path.
    pub fn exit_code(self) -> i32 {
        match self {
            Error::DivByZero => crate::abi::EXIT_DIV_ZERO,
            Error::BlackHole => crate::abi::EXIT_BLACKHOLE,
            _ => crate::abi::EXIT_PANIC,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Error::NegativeInput => "natural number constructed from negative input",
            Error::NegativeResult => "natural subtraction produced a negative result",
            Error::DivByZero => "division or remainder by zero",
            Error::OutOfRange => "value does not fit in i64",
            Error::MalformedDecimal => "malformed decimal numeral",
            Error::BufferTooSmall => "decimal buffer too small",
            Error::MalformedName => "symbol name is not valid UTF-8",
            Error::OutOfBounds => "byte-buffer index, length, or byte value out of range",
            Error::ReadOnly => "a read-only buffer cannot be mutated",
            Error::CorruptValue => "null or wrong-kind word or corrupt generated data",
            Error::BlackHole => {
                "the quoted program faulted or a memo cell was demanded while evaluating"
            }
            Error::NestingTooDeep => "quotation nesting exceeds the evaluator's depth bound",
        };
        f.write_str(msg)
    }
}

impl std::error::Error for Error {}

/// Result alias for the runtime's safe Rust API.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn div_by_zero_exits_with_the_division_fault_code() {
        assert_eq!(Error::DivByZero.exit_code(), 72);
    }

    #[test]
    fn the_remaining_errors_exit_through_the_panic_path() {
        let others = [
            Error::NegativeInput,
            Error::NegativeResult,
            Error::OutOfRange,
            Error::MalformedDecimal,
            Error::BufferTooSmall,
            Error::OutOfBounds,
            Error::CorruptValue,
            Error::NestingTooDeep,
        ];
        for e in others {
            assert_eq!(e.exit_code(), 71, "{e:?} must exit through cinch_panic");
        }
    }

    #[test]
    fn display_names_each_variant() {
        assert_eq!(
            Error::DivByZero.to_string(),
            "division or remainder by zero"
        );
        assert_eq!(
            Error::NegativeResult.to_string(),
            "natural subtraction produced a negative result"
        );
    }
}
