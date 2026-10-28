//! Runtime support library for generated Cinch code.
//!
//! `cinchruntime` is linked into every compiled Cinch artifact (the driver
//! links `libcinchruntime.a` against the generated LLVM IR objects). It is
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
//! Every Cinch value is one 64-bit machine word (`i64` at the ABI),
//! matching `compiler/src/core.rs`:
//!
//! * `Int` (the 64-bit machine integer of the surface prelude): the `i64`
//!   itself.
//! * `Bool`: 0 or 1. `Unit`: 0.
//! * ADT values: `ptrtoint` of a heap block `{ i64 tag; i64 field0; ... }`
//!   with `tag` the constructor index in declaration order (the compiler's
//!   convention; this runtime only allocates such blocks through
//!   [`cinch_alloc`]).
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
//! code. A word produced by a `cinch_nat_*` or `cinch_int_*` entry point points at
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
//! never freed (see [`cinch_free`]).
//!
//! Blocks allocated through [`cinch_alloc`] are zero-filled byte blocks of a
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
//! | 70 | `BlackHole` (e.g. demanding a memo cell mid-evaluation) | [`cinch_blackhole`] |
//! | 71 | generic fault: `cinch_panic` and every runtime error except division by zero and the `BlackHole` fault | [`cinch_panic`] |
//! | 72 | division or remainder by zero | [`cinch_fault_div_zero`] |
//!
//! Exit code 70 is coupled to the code generator: `compiler/src/codegen.rs`
//! emits `__cinch_blackhole` calling `exit(i32 70)` directly. Both call
//! sites are the same defined fault and MUST use the same code; the
//! constant lives in [`abi::EXIT_BLACKHOLE`].
//!
//! In the safe Rust API ([`Nat`], [`Int`]) the same conditions surface as
//! [`Error`] values, so tooling and tests observe them without dying. The
//! extern "C" layer maps [`Error::DivByZero`] onto exit 72,
//! [`Error::BlackHole`] onto exit 70, and every other error onto exit 71.
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
//! * `cinch_blackhole() -> !`: the `BlackHole` fault, exit 70.
//! * `cinch_panic(msg: *const u8, len: usize) -> !`: write the message to
//!   stderr and exit 71. A null pointer or zero length writes nothing.
//! * `cinch_fault_div_zero() -> !`: division fault, exit 72.
//!
//! Allocation (sizes are byte counts):
//!
//! * `cinch_alloc(size: i64) -> i64`: allocate `size` zero-filled bytes
//!   (rounded up to a whole number of 64-bit words, minimum one word) and
//!   return the block as a word. Negative sizes, size overflow, and
//!   allocation failure fault through `cinch_panic`.
//! * `cinch_free(word: i64)`: currently a no-op; blocks are leaked by design
//!   until reclamation lands. The symbol exists so generated code can
//!   already emit balanced allocation and deallocation.
//!
//! `Nat` (unbounded natural numbers; negative input or results fault):
//!
//! * `cinch_nat_from_i64(v: i64) -> i64`: box a machine integer, faulting on
//!   negative `v`.
//! * `cinch_nat_to_i64(word: i64) -> i64`: value as a machine integer,
//!   faulting when out of `i64` range.
//! * `cinch_nat_fits_i64(word: i64) -> i64`: 1 or 0.
//! * `cinch_nat_add(a: i64, b: i64) -> i64`, `cinch_nat_mul(a, b) -> i64`:
//!   exact arithmetic.
//! * `cinch_nat_sub(a: i64, b: i64) -> i64`: exact difference, faulting on
//!   negative result (defined fault, not wrapping).
//! * `cinch_nat_div(a: i64, b: i64) -> i64`: the quotient of `a / b`;
//!   the remainder is `cinch_nat_rem(a, b) -> i64`. Both fault on division by
//!   zero.
//! * `cinch_nat_cmp(a: i64, b: i64) -> i64`: -1, 0 or 1.
//! * `cinch_nat_from_decimal(ptr: *const u8, len: usize) -> i64`: parse ASCII
//!   digits (`[0-9]+`, leading zeros allowed) into a word, faulting on
//!   malformed input.
//! * `cinch_nat_to_decimal(word: i64, buf: *mut u8, cap: usize) -> usize`:
//!   write the ASCII digits into `buf`, returning the length written, and
//!   fault when `cap` is too small (leaving the buffer untouched). Use
//!   `cinch_nat_decimal_len(word: i64) -> usize` to size the buffer.
//!
//! `Int` (unbounded integers):
//!
//! * `cinch_int_from_i64(v: i64) -> i64`: box a machine integer.
//! * `cinch_int_to_i64(word: i64) -> i64`: value as a machine integer,
//!   faulting when out of `i64` range.
//! * `cinch_int_fits_i64(word: i64) -> i64`: 1 or 0.
//! * `cinch_int_add(a: i64, b: i64) -> i64`, `cinch_int_sub(a, b) -> i64`,
//!   `cinch_int_mul(a, b) -> i64`, `cinch_int_neg(a) -> i64`: exact arithmetic.
//! * `cinch_int_div(a: i64, b: i64) -> i64`, `cinch_int_rem(a, b) -> i64`:
//!   division truncated toward zero with the remainder taking the sign of
//!   the dividend (Rust `/` and `%` semantics on the underlying values),
//!   faulting on division by zero.
//! * `cinch_int_cmp(a: i64, b: i64) -> i64`: -1, 0 or 1.
//! * `cinch_int_from_decimal(ptr: *const u8, len: usize) -> i64`: parse an
//!   optionally signed ASCII numeral (`[+-]?[0-9]+`, leading zeros
//!   allowed, `-0` is zero) into a word, faulting on malformed input.
//! * `cinch_int_to_decimal(word: i64, buf: *mut u8, cap: usize) -> usize`:
//!   write the ASCII representation (sign included) into `buf`, returning
//!   the length written, and fault when `cap` is too small. Use
//!   `cinch_int_decimal_len(word: i64) -> usize` to size the buffer.
//!
//! `Bytes` (byte buffers; the foundation for the FFI and string slices):
//!
//! * `cinch_bytes_new(len: i64) -> i64`: a zeroed buffer block, faulting on
//!   negative length.
//! * `cinch_bytes_len(word: i64) -> i64`: the length in bytes.
//! * `cinch_bytes_get(word: i64, index: i64) -> i64`: the byte (0..=255) at
//!   `index`, faulting out of bounds.
//! * `cinch_bytes_set(word: i64, index: i64, value: i64) -> i64`: write
//!   `value` (0..=255) at `index`, returning 0 (the Unit word) and
//!   faulting on an out-of-bounds index or a value outside 0..=255. A
//!   read-only data-section buffer (a string literal's backing) takes
//!   the defined read-only fault instead of a write.
//! * `cinch_bytes_data(word: i64) -> i64`, `cinch_bytes_from_data(ptr: *const
//!   u8, len: i64) -> i64`: the pointer-length crossing for buffer
//!   contents (the UTF-8 and symbol intrinsics and the canonical ABI
//!   glue).
//!
//! Interned symbols (primitive types specification section 9; see
//! [`sym`] for the cell discipline):
//!
//! * `cinch_sym_intern(ptr: *const u8, len: i64) -> i64`: the cell for the
//!   `len` bytes at `ptr`; the table is keyed by name, so an equal name
//!   resolves to the same cell (and to a tick literal's cell).
//! * `cinch_sym_name(word: i64) -> i64`: a fresh `Bytes` word holding the
//!   cell's name.
//! * `cinch_sym_eq(a: i64, b: i64) -> i64`: identity, 0 or 1 (an O(1)
//!   pointer test).
//! * `cinch_sym_cmp(a: i64, b: i64) -> i64`: -1, 0 or 1 of the
//!   lexicographic name order (never the cell addresses).
//!
//! Text conversion and content identity (JSON and canonical
//! serialization; see [`text`]):
//!
//! * `cinch_text_sha256(word: i64) -> i64`: the SHA-256 digest (FIPS
//!   180-4) of the buffer in `word`, as a fresh 32-byte `Bytes` word.
//! * `cinch_text_f64_to_decimal(v: f64) -> i64`: one float's shortest
//!   round-trip decimal text (`{:e}` scientific form) as a fresh
//!   `Bytes` word.
//! * `cinch_text_f64_from_decimal(word: i64) -> f64`: the float one
//!   decimal spelling stands for (correctly rounded), faulting on a
//!   spelling that is not a decimal number.
//!
//! Quotation evaluation (supercompilation and JIT specification
//! section 7; see [`eval`] for the quote block layout):
//!
//! * `cinch_eval(quote: i64) -> i64`: evaluate the reified quotation in the
//!   `Code` block `quote` to its value word; a failing evaluation takes
//!   a defined fault path and produces no value (exit 70 for the quoted
//!   program's own fault or a black-holed memo cell, 71 otherwise).
//!
//! Every bignum and byte-buffer fault exits through `cinch_panic` (71)
//! except division by zero (72); see `crate::error::Error`.
//!
//! Structured concurrency and runtime I/O (concurrency specification
//! sections 2, 4 and 9; one OS thread per task, fork-join happens-before,
//! cancellation as a sequentially consistent atomic):
//!
//! * `cinch_scope_open() -> i64`, `cinch_scope_close(scope: i64) -> i64`: the
//!   structured scope of `withScope`; close cancels and joins unjoined
//!   children (section 2).
//! * `cinch_task_spawn(scope: i64, closure: i64) -> i64`: run `closure` on a
//!   task thread, returning its handle. The body runs through the closure
//!   apply-stub ABI and returns the prelude `Result` word (the task
//!   outcome ABI of `crate::task`).
//! * `cinch_task_join_all_or_cancel(scope, handles: i64, n: i64, out: i64) -> i64`:
//!   `joinAllOrCancel`; 0 and the `Ok` payloads in `out`, or 1 and the
//!   first `Err` payload in `out[0]`.
//! * `cinch_task_poll_cancel() -> i64`: the cancellation observation point
//!   (section 4).
//! * `cinch_io_write(word: i64) -> i64`, `cinch_io_read(len: i64) -> i64`,
//!   `cinch_io_sleep(millis: i64) -> i64`: the `Io r` operations (stdout,
//!   stdin, and a cancellation-aware suspension point).
//! * `cinch_coop_*`: the cooperative single-threaded backend of [`coop`] (the Wasm profile's lowering target) mirrors this surface under these names with the same word conventions.

pub mod abi;
pub mod bytes;
pub mod coop;
pub mod error;
pub mod eval;
pub mod evolution;
mod int;
mod limbs;
mod nat;
pub mod string;
pub mod sym;
pub mod task;
pub mod text;

pub use error::{Error, Result};
pub use int::Int;
pub use nat::Nat;
pub use bytes::Bytes;
