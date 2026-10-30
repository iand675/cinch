# Cinch runtime-fault inventory

Status: inventory taken 2026-10-29 by a read-only scout over the tree at
the time (before slices S1/S2 were integrated; line numbers drift). It
feeds `design/no-faults.md`.

## Closed since the inventory

- **Guards and literal matches (rows 70 and 140 below, item 1 of the
  escape-hatch list)**: closed 2026-10-30. A `case` whose rows can all fail
  by guard, and a literal match without a catch-all, are compile errors;
  the pattern compiler is the coverage check and can no longer emit
  `Core::Fault`, and `run_out` is deleted. `specifications/surface-grammar-local-amendments.md`
  section 16, `tests/guard_coverage.rs`, CONFORMANCE D401.
- **Resource exhaustion (all of section 5 except the I/O row)**: closed
  2026-10-30. The heap cap (now on live bytes), a real allocation
  failure, a native stack overflow on the program's or a task's thread,
  the evaluator's nesting bound, and a refused thread are one defined
  outcome: exit 73 with `cinch: exhausted: heap (<n> bytes)`,
  `cinch: exhausted: stack` or `cinch: exhausted: threads`
  (`design/no-faults.md` section 4.1, `compiler/runtime/src/stack.rs`,
  `tests/exhaustion.rs`). Nothing in the runtime aborts or dies on a
  signal for resource reasons any more.
- **Division and remainder by zero (`DivByZero`, the `nat-div`/`nat-rem`/
  `int-div`/`int-rem` rows below)**: closed 2026-10-30. The divisor is a
  `NonZero Nat`/`NonZero Int` (`type NonZero a = refine (n :: a) with
  NonZero n`, declared in `Algebra.Arithmetic`), and so is the `Div`/`Rem`
  classes' (`(/) :: a -> NonZero a -> a`). A numeral divisor decides its
  predicate at compile time with nothing written (`0` is a compile error);
  any other divisor is decided (`decide (NonZero d)`, a plain `if d == 0`
  guard, `non-zero`/`non-zero-nat`) or threaded as a `NonZero` parameter,
  and `checked-div`/`checked-rem` answer `None`. The runtime's zero check
  is a compiler-bug trap (exit 71, `compiler bug:`), exit 72 and
  `cinch_fault_div_zero` are gone. `tests/safe_division.rs`.
- **`Nat` subtraction below zero (`NegativeResult`, the `nat-sub` and `-`
  at `Nat` rows below)**: closed 2026-10-30. `nat-sub :: using (b <= a)
  => (a :: Nat) -> (b :: Nat) -> Nat`, and the `Sub_Nat` instance states
  the same obligation in a method signature inside the instance block
  (`(-) :: using (y <= x) => (x :: Nat) -> (y :: Nat) -> Nat`), so the
  `Sub` class and `-` at `Int` are unchanged. Numerals decide the
  obligation at compile time (`3 - 5` is refused naming `5 <= 3`); any
  other subtraction takes it from a branch fact, `Dec` evidence, or the
  enclosing definition's own `using` obligation. The `Sub_Nat`
  dictionary never reaches generic code (`using (Sub a)`), a
  sub-dictionary or a module argument, and its method is never a
  value. The truncating form is `saturating-sub`, and `checked-sub`
  answers `None`; `Data.Enum`'s `pred` and `to-enum` answer `Option`. The
  runtime's negative check is a compiler-bug trap (exit 71,
  `compiler bug:`). A `Nat` index difference reaches runtime only as a
  folded constant. `tests/safe_subtraction.rs`.
- **The `I64` conversions (`NegativeInput`, `OutOfRange`; the
  `nat-from-i64`, `nat-to-i64`/`int-to-i64` rows below)**: closed
  2026-10-30. `nat-from-i64 :: using (0 <= x) => (x :: I64) -> refined
  (n == x) Nat`, `nat-to-i64 :: using (n <= 9223372036854775807) =>
  ...`, `int-to-i64 :: using (-9223372036854775808 <= x, x <=
  9223372036854775807) => ...`; `checked-nat-from-i64`,
  `checked-nat-to-i64` and `checked-int-to-i64` answer `None`, and
  `nat-from-int`/`nat-to-int` convert `Int` and `Nat` of any size with
  no `I64` in between. Results in a known range state it
  (`design/refinement-types.md`, "refined results"): every widening
  integer conversion (`InRange lo hi r && r == x`), `mask-i64`,
  `char-code`, `nat-rem`, the integer bounds, `bytes-get`/`bytes-len`/
  `bytes-new`, and the runtime clock readings, so converting a value
  non-negative by construction needs nothing written. `Hash_Int` hashes
  the magnitude as a `Nat` (no conversion), `Data.Rational.to-f64`
  assembles a huge numerator from 32-bit limbs, and every intrinsic
  unit's `Nat` count or index crosses through `cinch_nat_to_len` (a
  bignum clamps to `i64::MAX`: an out-of-range index, or the heap
  `Exhausted` exit for a count). Both runtime checks are compiler-bug
  traps. `tests/range_facts.rs`, `tests/small_nats.rs`.
- **Byte buffers (`OutOfBounds`, `ReadOnly`; the `bytes-get`/`bytes-set`/
  `bytes-copy-range`/`bytes-new` rows, `argv`, the IO buffer paths)**:
  closed 2026-10-30 (NF5). `bytes-get`/`bytes-set :: using (i <
  bytes-len b) => ...`, a byte is an `InRange 0 255 Nat` both ways (a
  read's result carries it, a write's value proves it),
  `bytes-copy-range :: using (di + n <= bytes-len dst, si + n <=
  bytes-len src) => ...`, `bytes-new :: (n :: Nat) -> refined (bytes-len
  out == n) Bytes`, and `bytes-len` is at most `2^63 - 1`. A buffer's
  length never changes (the runtime's `len` is private and cleared only
  as the block is freed), so facts over `bytes-len b` survive writes to
  `b`. Loops discharge through their own guard; recursions carry their
  bound as their own `using` obligation; a buffer sized by a sum over a
  list is made at the end of a structural recursion whose result states
  `pos <= bytes-len out` (`concat-from`, `pack-from`, `words-from`,
  `bytes-from-list`). Three-term bounds (every copy range) discharge
  through the linear pass added to the fact checker
  (`design/proof-automation.md` section 5). `bytes-from-list` takes
  `List (InRange 0 255 Nat)`; `FromList Bytes` takes `U8` elements.
  `raw-arg` takes `using (i < arg-count)`. A string literal's read-only
  backing exists only inside a `String` (the `StrBuf` constructor and
  its reader are module-private, every conversion out copies), so no
  `Bytes` value is ever one. The word-buffer copy loops of `Effect.Fs`
  and `Effect.Process` answer a runtime error code as `IoError`/
  `ProcessIoError` instead of reading it as a byte. The runtime's
  index, range, value and read-only checks are compiler-bug traps; a
  failed stdout write is `OutputFailed` (still reachable, below).
  `tests/safe_bytes.rs`.

## Runtime exits classified (2026-10-30)

Every runtime exit that is neither `Exhausted` (73) nor a program result,
at the tree of 2026-10-30.

**Compiler-bug traps** (an invariant the compiler guarantees; exit 71,
message starting `compiler bug:`): the null or wrong-kind word checks
`abi.rs:174,182,295,304,368,383` (`Error::CorruptValue`, whose message
now says so) and `cinch_nat_to_decimal`/`cinch_int_to_decimal`'s null
buffer (`abi.rs:685,853`); `cinch_panic` with a corrupt message pointer
(`abi.rs:416`); the allocators' negative sizes and bad kinds
(`abi.rs:435,438,492,506,511`) and a corrupt layout table
(`abi.rs:525`); the join handle count (`abi.rs:1027`); a pointer and
length from generated code (`cinch_bytes_from_data`, `abi.rs:968,973`;
`cinch_sym_intern`, `sym.rs:85,90`; `cinch_utf8_encode`'s null buffer,
`string.rs:73`); a malformed quote block or program stream
(`Error::CorruptValue` throughout `eval.rs`, reported at `eval.rs:1149`);
the block layout and header checks (`block.rs:322,327,332,342,368,391,402`);
freed or corrupt counts (`rc.rs:139`); the program thread unwinding
(`platform.rs:577`); the cooperative join's handle count (`coop.rs:510`,
now `CorruptValue`).

**Faults a well-typed program can still reach** (left for the
migration, not fixed here):

- ~~`abi.rs:608` `cinch_nat_sub` (`NegativeResult`), and `abi.rs:337` for
  the bignum path of the same operation~~: closed (see above); the check
  is now a compiler-bug trap.
- ~~`abi.rs:631,643,782,795` `cinch_nat_div`/`rem`, `cinch_int_div`/`rem`
  (`DivByZero`, exit 72)~~: closed (see above); the check is now a
  compiler-bug trap.
- ~~`abi.rs:566` `cinch_nat_from_i64` (`NegativeInput`); `abi.rs:579,729`
  `cinch_nat_to_i64`/`cinch_int_to_i64` (`OutOfRange`)~~: closed (see
  above); the checks are compiler-bug traps.
- `text.rs:446` `cinch_text_f64_from_decimal` (`MalformedDecimal`, the
  float slice's). `abi.rs:667,674,829,842` `cinch_nat_from_decimal`/
  `cinch_int_from_decimal` are reached only by the code generator's
  big-numeral constants (`codegen.rs` around 5271), whose digits are a
  numeral's by construction: no surface name exposes them, so no
  well-typed program reaches the malformed path (2026-10-30 audit).
- ~~`abi.rs` `cinch_bytes_new`/`get`/`set`/`copy` (`OutOfBounds`),
  `as_bytes_mut` (`ReadOnly`), `platform.rs` `cinch_argv_get`
  (`OutOfBounds`), `cinch_io_read` with a negative length~~: closed (see
  above); the checks are compiler-bug traps. `cinch_bytes_from_data`
  takes its pointer and length from generated code only: a compiler-bug
  trap already.
- `abi.rs` `cinch_io_write` on a write or flush failure
  (`OutputFailed`, exit 71): an operating-system outcome (a closed pipe,
  a full device) that no program can rule out; making it a `Result` of
  `io-write` is a separate API change.
- `sym.rs:96` `cinch_sym_intern` on non-UTF-8 bytes (`MalformedName`,
  through the raw `sym-intern-bytes`).
- `coop.rs:422` the cooperative scheduler's deadlock (awaiting a task
  suspended on the call stack): no static rule yet rules it out.
- `eval.rs` `Error::BlackHole` (a quoted program's `FAULT` opcode, memo
  re-entrancy), exit 70, and the codegen-emitted sites of section 2.
- `abi.rs:688,856` `BufferTooSmall`: reachable only from a foreign C
  caller (no generated code calls the `*_to_decimal` externs), so it
  belongs with section 6.


Goal: every way a **compiled, well-typed** Cinch program can stop abnormally today.
Two structurally different fault regimes exist:

- **Defined faults**: `cinch_panic`/`cinch_fault_div_zero`/`cinch_blackhole`/heap-limit exit, a
  deliberate `exit()` with a message on stderr. Exit codes: `70` BlackHole, `71` panic
  (generic), `72` DivByZero, `73` HeapLimit (`compiler/runtime/src/abi.rs:17-28`,
  `compiler/runtime/src/error.rs:48-60`).
- **Undefined termination**: native stack overflow (SIGSEGV/SIGABRT), OS thread-spawn
  panic, or anything a `foreign` call does. These bypass the exit-code table entirely.

---

## 1. Runtime fault paths (`compiler/runtime/src/*.rs`)

Grouped by `Error` variant (`compiler/runtime/src/error.rs:12-45`) / exit code.

| `Error` variant | exit | site(s) | triggering operation | reachable by well-typed program |
|---|---|---|---|---|
| `DivByZero` | ~~72~~ 71 | `cinch_nat_div/rem`, `cinch_int_div/rem` | `nat-div`/`nat-rem`/`int-div`/`int-rem` with a zero divisor | **No (closed 2026-10-30)**: the divisor is a `NonZero`; the runtime check is a compiler-bug trap |
| `BlackHole` | 70 | `abi.rs:358` `cinch_blackhole`; `eval.rs:1044-1048` `cinch_eval` on `Err(BlackHole)`; codegen's `__cinch_demand`/`BoxGet` re-entrancy check | memo cell (`~`) demanded while its own thunk is running; a quoted `Code a`'s own `FAULT` opcode; a `letrec` box read before write | **Yes** for memo re-entrancy and `eval`'s embedded fault opcode (ordinary program logic, no static proof of acyclic demand order); **no** for the `letrec` box case: guarded by the static guarded-letrec rule (`check_terms.rs:9306-9341`), defense-in-depth only |
| `CorruptValue` | 71 | `abi.rs:112-121,201-211,220-229` (`as_sym`/`as_nat`/`as_int`/`as_bytes`/`as_bytes_mut`) on null/wrong-kind block; `eval.rs` quote-block/program-stream validation | a null or wrong-kind heap word reaching an extern entry point; a hand-built or corrupted quote block | **No** for ordinary generated code (the compiler only ever emits well-formed words); reachable only via raw-pointer `foreign`/FFI corruption or a compiler bug |
| `NegativeInput` | 71 | `abi.rs:463-466` `cinch_nat_from_i64`, `cinch_int_to_nat` | `nat-from-i64`/`nat-from-int` on a negative value | **No (closed 2026-10-30)**: the argument carries `0 <= x` evidence; the runtime check is a compiler-bug trap |
| `NegativeResult` | 71 | `abi.rs` `cinch_nat_sub` (immediate and bignum path via `nat::sub_mag`) | `nat-sub a b` with `b > a` | **No (closed 2026-10-30)**: `a - b` at `Nat` requires evidence that `b <= a`; the runtime check is a compiler-bug trap |
| `OutOfRange` | 71 | `abi.rs:473-479,616-620` `cinch_nat_to_i64`/`cinch_int_to_i64` | converting a bignum `Nat`/`Int` outside `i64` range to `I64` | **No (closed 2026-10-30)**: the argument carries range evidence (or the caller uses a `checked-*` form); the runtime check is a compiler-bug trap |
| `MalformedDecimal` | 71 | `abi.rs:552-562,666-677`; `eval.rs` `BIG` opcode decode; `text.rs` `cinch_text_f64_from_decimal` | a big numeral's constant (well-formed by construction); `f64-from-decimal` on non-numeral bytes | **No** for `Nat`/`Int` (only generated constants reach the decimal parsers; no surface name exposes them); **yes** for `f64-from-decimal` on untrusted text (the float slice) |
| `BufferTooSmall` | 71 | `abi.rs:566-578,695-706` `cinch_nat_to_decimal`/`cinch_int_to_decimal` | caller-supplied buffer shorter than `decimal_len` | **No** in practice: generated/prelude code always sizes the buffer from `decimal-len` first, but nothing in the type system *proves* it; a hand-written caller can still trigger it |
| `MalformedName` | 71 | `sym.rs:38-39` `intern` | a `Sym` name that is not valid UTF-8 | **No**: the only surface entry (`Data.Sym.intern :: String -> Sym`, `Sym.cinch:39-40`) converts through `string-to-bytes`, which is always valid UTF-8; dead for well-typed programs unless a raw `sym-intern-bytes :: Bytes -> Sym` is called directly with non-UTF-8 bytes (exposed but undocumented as a lower-level escape) |
| `OutOfBounds` | 71 | `bytes.rs` `Bytes::new/get/set`, `copy_range`; `abi.rs` `cinch_bytes_new/get/set/copy`; `platform.rs` `cinch_argv_get` | a `bytes-get`/`bytes-set` index outside `0..len`, a byte value outside `0..=255`, a `bytes-copy-range` range past either end, a negative length, an `argv` index past the count | **No (closed 2026-10-30, NF5)**: every index, range and value carries its evidence; the checks are compiler-bug traps. A failed stdout write is now `OutputFailed` (still reachable, section 5) |
| `ReadOnly` | 71 | `abi.rs` `as_bytes_mut` on `KIND_BYTES_STATIC` | `bytes-set`/`bytes-copy-range` targeting a string literal's constant backing | **No (closed 2026-10-30, NF5)**: the backing exists only inside a `String` (private constructor and reader); every conversion out copies; the check is a compiler-bug trap |
| `NestingTooDeep` | 71 | `eval.rs:162,288-299` `MAX_NESTING = 1024` | a quoted program (`Code a`) whose expression tree + captured holes nests past 1024 evaluator-recursion units | **Yes**: `eval`/quotation depth is a runtime property, not type-checked |
| `HeapLimit` | 73 | `heap.rs:118-130,175-179` `account_alloc`/`heap_limit_exit` | cumulative heap allocation exceeding `CINCH_HEAP_LIMIT` (default 8 GiB, every `cinch_alloc`/`cinch_alloc_raw`/Rust `Box`/`Vec` call) | **Yes**: resource exhaustion, see §5 |

Additional direct `std::process::exit`/`panic!` sites not routed through `Error`:
- `abi.rs:125,130,359`: `panic_exit`, `div_zero_exit`, `cinch_blackhole` themselves (the exit mechanics, not new triggers).
- `task.rs:152-153`: `Scope::spawn` calls `.unwrap_or_else(|e| panic!(...))` if `std::thread::Builder::spawn` fails (OS thread-creation failure). **Yes, reachable** under resource exhaustion (too many OS threads/processes); this is a genuine Rust `panic!`: on an `extern "C"` boundary a Rust panic *aborts the process* (no unwind across FFI), not a defined exit code (§5).
- `platform.rs:555-566` `cinch_run_main`: if the 1 GiB stack reservation thread fails to spawn, it degrades gracefully by running `entry` on the calling thread instead of faulting; if the spawned thread's `join()` fails it maps to `EXIT_PANIC` (71).

---

## 2. Codegen-emitted fault sites (`compiler/src/codegen.rs`)

| site | emits | source construct | reachable by well-typed program |
|---|---|---|---|
| `codegen.rs:4429-4440` | `call void @__cinch_blackhole()` + `unreachable` for `Core::Fault` | a `case`/`match` whose guards all fail on every pattern-covered row (§3) | **Yes** |
| `codegen.rs:4588-4590` | `call void @exit(i32 70)` + `unreachable` | `Core::BoxGet` on an uninitialized `letrec` fixpoint box | dead by the static guarded-letrec rule in ordinary cases (§3) |
| `codegen.rs:6764-6766,7003-7005` | `call void @__cinch_blackhole()` + `unreachable` | a `CASE` dispatch over a block whose discriminant tag matches none of the compiled arms (an internal invariant for boundary-layout/flat-return call dispatch) | **No** for well-typed source: only reachable through a compiler-internal mismatch or corrupted data |
| `codegen.rs:2032-2040` | `fault_when` → `cinch_panic` | `utf8CountChars` on a malformed UTF-8 buffer | **Yes** if the `Bytes` wasn't validated with `utf8Validate` first: `Data.String`'s `utf8-count-chars`/`utf8-decode-at` contract treats a non-UTF-8 buffer as undefined and faults rather than returning `Option`/`Result` |
| `codegen.rs:2070-2082` | `fault_when` → `cinch_panic` | `utf8Encode` on a value that is not a Unicode scalar value | **Yes**: nothing in the type system restricts `Nat`/`I64` arguments to the scalar-value range; the prelude's `char-to-u32`/`u32-to-char` round trip (total, `Option`-returning) is the only safe front door, but the raw `utf8-encode-into` intrinsic is directly callable |
| `codegen.rs:3496-3513` | `check_non_null`/`ffi_fault_block` → `cinch_panic` | a `foreign import`/`export` boundary pointer parameter declared non-null receiving a null pointer at runtime | **Yes** for any `foreign` call with a non-owned/raw pointer argument sourced from outside the type checker's view (§6) |
| `codegen.rs:1904-1905,2556,2680,2829,3184,3366` | Rust-side `unreachable!()` | exhaustive matches over the compiler's own closed enums (`ArithMode`, binop tags, `ConvMode`, field shapes) | **No**: these are compiler-internal invariants, not reachable from any Cinch source; a hit here is a compiler bug, not a program fault |

Machine-scalar arithmetic (`I8`..`I64`/`U8`..`Usize`) is **not** a fault path at codegen level:
`ArithMode::Checked`/`CheckedDiv`/`CheckedRem`/`Shift{checked}`/`Neg{Checked}` all lower to
`ret_option` (an `Option` result, total, `codegen.rs:1904-1912,2654-2684`); only `ArithMode::Wrapping`
silently wraps (defined, non-faulting) and `ArithMode::Saturating` clamps (also non-faulting).

---

## 3. Checker-emitted `Fault` (`ECore::Fault`/`Core::Fault`, `compiler/src/check_terms.rs`, `check.rs`, `check_decls.rs`, `core.rs`)

| production site | source construct | reachable by well-typed program |
|---|---|---|
| `check_terms.rs:6647-6654` `run_out` | A `case`/`match` row matrix runs out at the fallback: **pattern** coverage is proven exhaustive by `matrix_covered` (`check_terms.rs:6955-...`, rejects non-exhaustive matrices at `check_terms.rs:6052-6054,6297-6299`), but **guard** coverage is never proven: `compile_rows` (`check_terms.rs:5686-5745`) falls through a guarded row that fails to the next row, and `run_out` is the terminal fallback once every row's guard has failed | **YES: this is the primary live hole.** Any `case`/`match` whose alternatives are all guarded (`pat | cond -> ...`), even with a catch-all pattern, compiles and type-checks fine but faults at runtime if no guard happens to hold for a given scrutinee. This is the single most direct way ordinary, well-typed Cinch source reaches `Core::Fault` (exit 70 via `__cinch_blackhole`). |
| `check_terms.rs:7858-7999` `infer_unhandled_op`/`dispatch_op` | an effect operation call for which no enclosing `handle` matches its label at the matching lexical extent (decision D23's "defense in depth") | **No**: "effect honesty rejects unhandled operations at check time before any program can run one" (`check_terms.rs:7737-7740`): the effect-row type system is supposed to make this statically dead; it is kept only as a second line of defense |
| `check_decls.rs:18182-18186` `eval_core` on `Core::Fault` | a **compile-time** staged computation (macro/`quote`/const-eval) whose body reduces to `Core::Fault` | **N/A at runtime**: this rejects the *compilation*, it never reaches a running program; listed for completeness since it shares the `Fault` constructor |

---

## 4. Prelude exposure of faulting primitives

For each faulting primitive: the `.cinch` functions that expose it with no total alternative, and existing total escapes.

| primitive | partial function(s) | total alternative in prelude | approximate call-site density |
|---|---|---|---|
| `Nat` subtraction below zero | **closed 2026-10-30**: `nat-sub` and `-` at `Nat` require `b <= a` | `saturating-sub` (truncating), `checked-sub` (`Option`) | all sites migrated (see the closing note above) |
| `Nat`/`Int` division or remainder by zero | **closed 2026-10-30**: `nat-div`/`nat-rem`/`int-div`/`int-rem` and `/`/`%` take a `NonZero` divisor | `checked-div`/`checked-rem` (`Option`), `non-zero`/`non-zero-nat`, `decide (NonZero d)`; `Data.BigDecimal.div` now answers `Option`, `Data.Rational.div` keeps its `Result` | all sites migrated (see the closing note above) |
| `Bytes` out-of-bounds index/value | **closed 2026-10-30 (NF5)**: `bytes-get`/`bytes-set`/`bytes-copy-range` take index and range obligations, a byte is an `InRange 0 255 Nat` | a guard (`if i < bytes-len b`), the buffer's own `bytes-new` length, an own `using` obligation, or a structural recursion whose result states its bound | all sites migrated (see the closing note above) |
| decimal parse failure | `nat-from-decimal`/`int-from-decimal` (raw runtime imports) | surface parsers in `Text.Parse`/`Text.Json`/`Text.Toml`/`Text.Csv` validate digit-by-digit (`digits-fold`, `all-digits`) *before* calling the parser, so most prelude use is effectively guarded; no `Option`-returning `Nat`/`Int` decimal parse is exported directly | low direct exposure (a handful of call sites) because callers pre-validate; still unguarded if a program calls the raw parser on unvalidated text |
| `Sym` interning of non-UTF-8 bytes | `sym-intern-bytes :: Bytes -> Sym` (`Data/Sym.cinch:33`) | `Data.Sym.intern :: String -> Sym` (`Sym.cinch:39-40`) is the documented total front door (a `String`'s bytes are always valid UTF-8) | effectively zero exposure through the documented API; the raw foreign import is exposed but not meant to be called directly |
| eval nesting too deep | `cinch_eval`/`eval :: Code a -> a` | none: nesting depth is inherent to the quoted program's own shape, no static bound | rare (quotation/supercompilation code only) |
| read-only buffer write | **closed 2026-10-30 (NF5)**: no `Bytes` value is a literal's backing | `string-to-bytes` copies (as before) | none |
| `Nat`/`Int` → `I64` range overflow | **closed 2026-10-30**: `nat-to-i64`/`int-to-i64` state the range they need | `checked-nat-to-i64`/`checked-int-to-i64` (`Option`), `nat-to-int`/`nat-from-int` (any size) | all sites migrated (see the closing note above) |

Refinement-typed indices (`Dec`/`decide`, `design`'s total-API pattern) exist for several **container-level** invariants: `NonZero` (`Data/NonZero.cinch:32-34`), `NonEmpty` (`Data/NonEmpty.cinch:43-45`), `Sorted` (`Data/Sorted.cinch:30-32`), `Ascii` (`Data/Ascii.cinch:30-32`): but there is **no general `Fin n`/bounded-index refinement type** anywhere in `specifications/`, `design/`, or `compiler/prelude/`; `Bytes`/`List` indexing has no refined-index total alternative at all, which is why `bytes-get`/`bytes-set`/`nat-sub`/division dominate the exposure counts above.

---

## 5. Resource exhaustion

| mechanism | site | behavior | exit code / termination |
|---|---|---|---|
| Heap cap | `heap.rs` `account_alloc`; env `CINCH_HEAP_LIMIT` (default 8 GiB, `0` disables) | **closed 2026-10-30**: the cap is on live bytes (allocated minus freed); a refused system allocation and a request too large for any layout take the same exit | `Exhausted`, exit **73**, `cinch: exhausted: heap (<n> bytes)` |
| Native stack overflow | `platform.rs` `cinch_run_main`, `task.rs` `Scope::spawn`, `stack.rs` `guarded` | **closed 2026-10-30**: a 64 KiB guard band at the low end of every Cinch thread's stack, a `SIGSEGV`/`SIGBUS` handler on an alternate stack, and inline stack probes on every generated unit | `Exhausted`, exit **73**, `cinch: exhausted: stack` |
| Eval nesting | `eval.rs` `MAX_NESTING = 1024` | **closed 2026-10-30**: the bound stands in for the stack, so passing it is the stack outcome | `Exhausted`, exit **73**, `cinch: exhausted: stack` |
| OS thread spawn failure | `task.rs` `Scope::spawn`, `platform.rs` `cinch_run_main` (the old fallback onto the unguarded main thread is gone) | **closed 2026-10-30**: `stack::spawn_refused` | `Exhausted`, exit **73**, `cinch: exhausted: threads` (`stack` when the refusal was `ENOMEM`) |
| File-descriptor / OS errors | `abi.rs:946-951` `cinch_io_write` maps a write/flush failure to `Error::OutOfBounds` (exit 71, a slight misnomer but still a defined fault); `platform.rs` process/pty/net handle operations return negated error codes (`-INVALID_ARGUMENT` etc.) as ordinary `Result`-shaped values, not faults | OS errors are generally surfaced as values, not faults, **except** `cinch_io_write`'s stdout failure | mostly fine; the one write-failure case reuses the `OutOfBounds` fault rather than returning a `Result` |

---

## 6. Foreign calls

- `foreign import`/`foreign export` declarations carry **no total-pure default**: "A `foreign` declaration has opaque external behavior until an effect and contract envelope is supplied. It does not inherit the total-pure default of an ordinary `fn`." (`specifications/foreign-function-interface-specification.md:40`).
- Pointer arguments/results get one of five roles: `RawPtr`, `Borrowed`, `Retained`, `Owned` (linear), plus `Out`/`MaybeNull` modifiers (`specifications/foreign-function-interface-specification.md:46,128-132`). `RawPtr` is explicitly the "raw unchecked pointer" role; the spec states safe callers "MUST NOT" construct roles by casting integers, but this is a documented *convention*, not something the type checker fully closes: the FFI boundary is deliberately where native-code crashes (segfault, use-after-free, out-of-bounds write in a foreign callee) can occur with no runtime fault wrapper at all.
- A non-null-annotated boundary pointer that *is* null is turned into the one checked case, the defined `cinch_panic` via `check_non_null`/`ffi_fault_block` (`codegen.rs:3496-3513`); everything else about what the foreign function itself does: writing past a buffer, dereferencing a dangling pointer, calling back into Cinch incorrectly: is outside the compiler's and runtime's reach.
- `#[no-raw-ptr]` (JPL rule 9, `check_decls.rs:12144-12176`) is an **opt-in** safety attribute: with it, a signature may carry no `RawPtr`/`opaque` type and a body may call no foreign pointer operation. Without it (the default), raw pointers and foreign calls are unrestricted. This attribute is the only mechanism that makes the FFI escape "extremely onerous to opt into" rather than freely available, and it is off by default.

---

## 7. Explicit escape hatches

- **No `fault`/`panic`/`unreachable`/`error`/`absurd` function exists in the prelude** (grep across `compiler/prelude`, `compiler/src`, `specifications` for `absurd` returns nothing). The only source-level way to *intentionally* fault is:
  1. Write a `case`/`match` whose alternatives are all guarded and whose guards can all fail (§3): reaches `Core::Fault` → exit 70 via `__cinch_blackhole`. This is accidental-looking but works as a deliberate "crash here" idiom today, and nothing stops it.
  2. Call a known-partial prelude/runtime primitive directly (`nat-sub`, `nat-div`/`int-div`, `bytes-get`/`bytes-set`, etc., §1/§4): the de facto `panic`/`assert` idiom in this language, reusing the generic exit-71 path.
  3. Write a `foreign import` to a C function that calls `abort()`/raises `SIGSEGV`: fully outside the language's own fault story (§6).
- `#[assert-min n]` (JPL rule 5, attribute annotations specification section 10) does **not** itself introduce a new fault: it is a *static* compiler check that a function's body contains at least `n` runtime `decide` sites (`check_decls.rs:12073-12099,13844-13849`); it counts existing `decide` calls, it does not insert new ones.
- `decide :: p -> Dec p` (the refinement discharge world, `check_terms.rs:2068-2241`) is **total**: it returns `Yes evidence | No refutation`, never faults. It is the mechanism by which `NonZero`/`NonEmpty`/`Sorted`/`Ascii` turn a partial construction into an `Option`-returning one (§4); it is not itself a fault path.
- There is no runtime `assert` keyword that can fail at runtime distinct from (1)/(2) above: "asserting" in this language *is* writing a guarded match or calling a partial primitive.

---

## Top 10 most-used faulting operations by call-site count (prelude-wide, approximate, descending)

| rank | operation | fault | approx. call sites |
|---|---|---|---|
| 1 | `bytes-get` | closed (`i < bytes-len b` evidence) | 100+ |
| 2 | `bytes-set` | closed (index evidence, byte-valued argument) | 90+ |
| 3 | `nat-div` | closed (divisor is `NonZero Nat`) | 60+ |
| 4 | `nat-rem` | closed (divisor is `NonZero Nat`) | 60+ |
| 5 | `int-div` | closed (divisor is `NonZero Int`) | 25+ |
| 6 | `int-rem` | closed (divisor is `NonZero Int`) | 25+ |
| 7 | `nat-sub` | closed (`b <= a` evidence) | 15+ |
| 8 | `nat-to-i64`/`int-to-i64` | `OutOfRange`, exit 71 | 10+ |
| 9 | guarded `case`/`match` with no unguarded catch-all (checker's `run_out`) | `Core::Fault`, exit 70 | present wherever guards are used without a trailing unconditional wildcard: not grep-countable, but the single highest-leverage *type-system gap* since it's the one path that compiles despite being genuinely unprovable |
| 10 | `nat-from-decimal`/`int-from-decimal` (direct, unvalidated) | `MalformedDecimal`, exit 71 | low single digits directly, but the underlying risk recurs anywhere text parsing skips pre-validation |

**For the "bulletproof" design goal, the three highest-leverage fixes are:** (a) give `Nat`
subtraction, and `Nat`/`Int` division/remainder, total (`Option`/refined-nonzero-divisor)
alternatives and steer the prelude onto them, closing the single widest exposure; (b) give
`Bytes`/indexable containers a bounds-refined index type (a general `Fin n` or an
`Option`-returning `bytes-get`/`bytes-set` used prelude-wide, mirroring `Http.byte-at`'s
one existing wrapper) to close the second-widest exposure; (c) require guard exhaustiveness
(or forbid an all-guarded match without a trailing unconditional arm) in the checker, closing
the one type-system hole (`run_out`/`Core::Fault`) that lets an otherwise "exhaustive" match
still fault at runtime.
