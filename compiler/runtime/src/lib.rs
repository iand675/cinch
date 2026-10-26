//! Runtime support library for generated SafeLang code.
//!
//! `slruntime` is linked into every compiled SafeLang artifact (the driver
//! links `libslruntime.a` against the generated LLVM IR objects). It is
//! written in Rust with no third-party dependencies and builds both as a
//! `staticlib` (for linking) and as an `rlib` (so compiler-side tooling can
//! use the same value semantics, for example to evaluate literals at
//! compile time).
//!
//! This module comment is the runtime ABI reference: the code generator and
//! the compiled interface metadata cite it.
//!
//! # Word representation
//!
//! Every SafeLang value is one 64-bit machine word (`i64` at the ABI),
//! matching `compiler/src/core.rs`:
//!
//! * `Int` (the 64-bit machine integer of the surface prelude): the `i64`
//!   itself.
//! * `Bool`: 0 or 1. `Unit`: 0.
//! * ADT values: `ptrtoint` of a heap block `{ i64 tag; i64 field0; ... }`
//!   with `tag` the constructor index in declaration order (the compiler's
//!   convention; this runtime only allocates such blocks through
//!   [`sl_alloc`]).
//! * Function values and dictionaries: `ptrtoint` of their closure or
//!   record blocks (see `compiler/src/core.rs`).
//! * `Nat` and `Int` (the unbounded numbers): `ptrtoint` of a bignum heap
//!   block owned by this runtime (below).
//!
//! The unbounded `Nat` and `Int` of the core spec are distinct from the
//! 64-bit machine `Int` above; only the context says which is meant, and
//! their words are never confused because each is produced and consumed by
//! the correspondingly named entry points.
//!
//! # Heap layouts
//!
//! Bignum blocks are allocated by this runtime and are opaque to generated
//! code. A word produced by a `sl_nat_*` or `sl_int_*` entry point points at
//! a block with the following layout (offsets in bytes):
//!
//! | offset | size | field |
//! |--------|------|-------|
//! | 0 | 8 | `kind`: `i64`, `KIND_NAT` (1) or `KIND_INT` (2) |
//! | 8 | ... | payload: a Rust `Nat` value (`KIND_NAT`) or `Int` value (`KIND_INT`) |
//!
//! The `kind` word comes first so that a corrupt or mismatched word faults
//! (see Faults) instead of being reinterpreted. The payload is a
//! sign-magnitude bignum: a normalized base 2^32 `Vec<u32>` magnitude
//! (little-endian limbs, no leading zero limbs) plus, for `Int`, a negative
//! flag. Zero is non-negative and has the empty magnitude. The `Vec`
//! header inside the payload is a standard library implementation detail;
//! generated code MUST treat the block as opaque and only pass the word
//! back to this runtime. Words are immutable and may be shared; they are
//! never freed (see [`sl_free`]).
//!
//! Blocks allocated through [`sl_alloc`] are zero-filled byte blocks of a
//! whole number of 64-bit words, for the compiler's `{ i64 tag; ... }`
//! convention.
//!
//! # Faults
//!
//! Faults are defined runtime failures. At the extern "C" layer they write
//! a message to stderr (when one exists) and terminate the process:
//!
//! | exit code | fault | entry point |
//! |-----------|-------|-------------|
//! | 70 | `BlackHole` (e.g. demanding a memo cell mid-evaluation) | [`sl_blackhole`] |
//! | 71 | generic fault: `sl_panic` and every runtime error except division by zero | [`sl_panic`] |
//! | 72 | division or remainder by zero | [`sl_fault_div_zero`] |
//!
//! Exit code 70 is coupled to the code generator: `compiler/src/codegen.rs`
//! emits `__safelang_blackhole` calling `exit(i32 70)` directly. Both call
//! sites are the same defined fault and MUST use the same code; the
//! constant lives in [`abi::EXIT_BLACKHOLE`].
//!
//! In the safe Rust API ([`Nat`], [`Int`]) the same conditions surface as
//! [`Error`] values, so tooling and tests observe them without dying. The
//! extern "C" layer maps [`Error::DivByZero`] onto exit 72 and every other
//! error onto exit 71.
//!
//! # Extern symbols
//!
//! Every entry point takes and returns `i64` words or raw pointers. Word
//! arguments must be words produced by this runtime: null or wrong-kind
//! blocks fault as [`Error::CorruptValue`] (exit 71). Any other bogus word
//! used as a pointer is a caller bug and faults like any invalid pointer
//! dereference, not as a typed runtime fault.
//!
//! Faults:
//!
//! * `sl_blackhole() -> !`: the `BlackHole` fault, exit 70.
//! * `sl_panic(msg: *const u8, len: usize) -> !`: write the message to
//!   stderr and exit 71. A null pointer or zero length writes nothing.
//! * `sl_fault_div_zero() -> !`: division fault, exit 72.
//!
//! Allocation (sizes are byte counts):
//!
//! * `sl_alloc(size: i64) -> i64`: allocate `size` zero-filled bytes
//!   (rounded up to a whole number of 64-bit words, minimum one word) and
//!   return the block as a word. Negative sizes, size overflow, and
//!   allocation failure fault through `sl_panic`.
//! * `sl_free(word: i64)`: currently a no-op; blocks are leaked by design
//!   until reclamation lands. The symbol exists so generated code can
//!   already emit balanced allocation and deallocation.
//!
//! `Nat` (unbounded natural numbers; negative input or results fault):
//!
//! * `sl_nat_from_i64(v: i64) -> i64`: box a machine integer, faulting on
//!   negative `v`.
//! * `sl_nat_to_i64(word: i64) -> i64`: value as a machine integer,
//!   faulting when out of `i64` range.
//! * `sl_nat_fits_i64(word: i64) -> i64`: 1 or 0.
//! * `sl_nat_add(a: i64, b: i64) -> i64`, `sl_nat_mul(a, b) -> i64`:
//!   exact arithmetic.
//! * `sl_nat_sub(a: i64, b: i64) -> i64`: exact difference, faulting on
//!   negative result (defined fault, not wrapping).
//! * `sl_nat_div(a: i64, b: i64) -> i64`: the quotient of `a / b`;
//!   the remainder is `sl_nat_rem(a, b) -> i64`. Both fault on division by
//!   zero.
//! * `sl_nat_cmp(a: i64, b: i64) -> i64`: -1, 0 or 1.
//! * `sl_nat_from_decimal(ptr: *const u8, len: usize) -> i64`: parse ASCII
//!   digits (`[0-9]+`, leading zeros allowed) into a word, faulting on
//!   malformed input.
//! * `sl_nat_to_decimal(word: i64, buf: *mut u8, cap: usize) -> usize`:
//!   write the ASCII digits into `buf`, returning the length written, and
//!   fault when `cap` is too small (leaving the buffer untouched). Use
//!   `sl_nat_decimal_len(word: i64) -> usize` to size the buffer.
//!
//! `Int` (unbounded integers):
//!
//! * `sl_int_from_i64(v: i64) -> i64`: box a machine integer.
//! * `sl_int_to_i64(word: i64) -> i64`: value as a machine integer,
//!   faulting when out of `i64` range.
//! * `sl_int_fits_i64(word: i64) -> i64`: 1 or 0.
//! * `sl_int_add(a: i64, b: i64) -> i64`, `sl_int_sub(a, b) -> i64`,
//!   `sl_int_mul(a, b) -> i64`, `sl_int_neg(a) -> i64`: exact arithmetic.
//! * `sl_int_div(a: i64, b: i64) -> i64`, `sl_int_rem(a, b) -> i64`:
//!   division truncated toward zero with the remainder taking the sign of
//!   the dividend (Rust `/` and `%` semantics on the underlying values),
//!   faulting on division by zero.
//! * `sl_int_cmp(a: i64, b: i64) -> i64`: -1, 0 or 1.
//! * `sl_int_from_decimal(ptr: *const u8, len: usize) -> i64`: parse an
//!   optionally signed ASCII numeral (`[+-]?[0-9]+`, leading zeros
//!   allowed, `-0` is zero) into a word, faulting on malformed input.
//! * `sl_int_to_decimal(word: i64, buf: *mut u8, cap: usize) -> usize`:
//!   write the ASCII representation (sign included) into `buf`, returning
//!   the length written, and fault when `cap` is too small. Use
//!   `sl_int_decimal_len(word: i64) -> usize` to size the buffer.
//!
//! `Bytes` (byte buffers; the foundation for the FFI and string slices):
//!
//! * `sl_bytes_new(len: i64) -> i64`: a zeroed buffer block, faulting on
//!   negative length.
//! * `sl_bytes_len(word: i64) -> i64`: the length in bytes.
//! * `sl_bytes_get(word: i64, index: i64) -> i64`: the byte (0..=255) at
//!   `index`, faulting out of bounds.
//! * `sl_bytes_set(word: i64, index: i64, value: i64) -> i64`: write
//!   `value` (0..=255) at `index`, returning 0 (the Unit word) and
//!   faulting on an out-of-bounds index or a value outside 0..=255.
//!
//! Every bignum and byte-buffer fault exits through `sl_panic` (71)
//! except division by zero (72); see `crate::error::Error`.
//!
//! Structured concurrency and runtime I/O (concurrency specification
//! sections 2, 4 and 9; one OS thread per task, fork-join happens-before,
//! cancellation as a sequentially consistent atomic):
//!
//! * `sl_scope_open() -> i64`, `sl_scope_close(scope: i64) -> i64`: the
//!   structured scope of `withScope`; close cancels and joins unjoined
//!   children (section 2).
//! * `sl_task_spawn(scope: i64, closure: i64) -> i64`: run `closure` on a
//!   task thread, returning its handle. The body runs through the closure
//!   apply-stub ABI and returns the prelude `Result` word (the task
//!   outcome ABI of `crate::task`).
//! * `sl_task_join_all_or_cancel(scope, handles: i64, n: i64, out: i64) -> i64`:
//!   `joinAllOrCancel`; 0 and the `Ok` payloads in `out`, or 1 and the
//!   first `Err` payload in `out[0]`.
//! * `sl_task_poll_cancel() -> i64`: the cancellation observation point
//!   (section 4).
//! * `sl_io_write(word: i64) -> i64`, `sl_io_read(len: i64) -> i64`,
//!   `sl_io_sleep(millis: i64) -> i64`: the `Io r` operations (stdout,
//!   stdin, and a cancellation-aware suspension point).

pub mod abi;
pub mod bytes;
pub mod error;
pub mod evolution;
mod int;
mod limbs;
mod nat;
pub mod task;

pub use error::{Error, Result};
pub use int::Int;
pub use nat::Nat;
pub use bytes::Bytes;
