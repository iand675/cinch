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
| `DivByZero` | 72 | `abi.rs:374` `cinch_fault_div_zero`; raised from `cinch_nat_div/rem`, `cinch_int_div/rem` (`abi.rs:525,536,619,630`) | `nat-div`/`nat-rem`/`int-div`/`int-rem` with a zero divisor | **Yes**: `Nat`/`Int` division has no total prelude variant |
| `BlackHole` | 70 | `abi.rs:358` `cinch_blackhole`; `eval.rs:1044-1048` `cinch_eval` on `Err(BlackHole)`; codegen's `__cinch_demand`/`BoxGet` re-entrancy check | memo cell (`~`) demanded while its own thunk is running; a quoted `Code a`'s own `FAULT` opcode; a `letrec` box read before write | **Yes** for memo re-entrancy and `eval`'s embedded fault opcode (ordinary program logic, no static proof of acyclic demand order); **no** for the `letrec` box case: guarded by the static guarded-letrec rule (`check_terms.rs:9306-9341`), defense-in-depth only |
| `CorruptValue` | 71 | `abi.rs:112-121,201-211,220-229` (`as_sym`/`as_nat`/`as_int`/`as_bytes`/`as_bytes_mut`) on null/wrong-kind block; `eval.rs` quote-block/program-stream validation | a null or wrong-kind heap word reaching an extern entry point; a hand-built or corrupted quote block | **No** for ordinary generated code (the compiler only ever emits well-formed words); reachable only via raw-pointer `foreign`/FFI corruption or a compiler bug |
| `NegativeInput` | 71 | `abi.rs:463-466` `cinch_nat_from_i64` | `nat-from-i64` on a negative `I64` | **Yes**: no static proof the argument is non-negative |
| `NegativeResult` | 71 | `abi.rs:486-491` `cinch_nat_sub` (immediate and bignum path via `nat::sub_mag`) | `nat-sub a b` with `b > a` | **Yes**: `Nat` subtraction is the inductive truncated kind at the type, but the *runtime* has no saturating form; prelude exposes only the faulting `nat-sub` |
| `OutOfRange` | 71 | `abi.rs:473-479,616-620` `cinch_nat_to_i64`/`cinch_int_to_i64` | converting a bignum `Nat`/`Int` outside `i64` range to `I64` | **Yes**: callers routinely use `nat-to-i64`/`int-to-i64` without first checking `nat-fits-i64`/`int-fits-i64` |
| `MalformedDecimal` | 71 | `abi.rs:552-562,666-677`; `eval.rs` `BIG` opcode decode | `nat-from-decimal`/`int-from-decimal` on non-numeral bytes | **Yes** when parsing untrusted text (`String->Nat`/`Int` has no total prelude form) |
| `BufferTooSmall` | 71 | `abi.rs:566-578,695-706` `cinch_nat_to_decimal`/`cinch_int_to_decimal` | caller-supplied buffer shorter than `decimal_len` | **No** in practice: generated/prelude code always sizes the buffer from `decimal-len` first, but nothing in the type system *proves* it; a hand-written caller can still trigger it |
| `MalformedName` | 71 | `sym.rs:38-39` `intern` | a `Sym` name that is not valid UTF-8 | **No**: the only surface entry (`Data.Sym.intern :: String -> Sym`, `Sym.cinch:39-40`) converts through `string-to-bytes`, which is always valid UTF-8; dead for well-typed programs unless a raw `sym-intern-bytes :: Bytes -> Sym` is called directly with non-UTF-8 bytes (exposed but undocumented as a lower-level escape) |
| `OutOfBounds` | 71 | `bytes.rs:38-69` `Bytes::new/get/set`; `abi.rs:788-800,852-863` `cinch_bytes_get/set`, `cinch_bytes_copy`, `cinch_bytes_from_data`; `abi.rs:870-874` `cinch_task_join_all_or_cancel`; `platform.rs:516-521` `cinch_argv_get`; `abi.rs:955,967` `cinch_io_write/read` (buffer write failure reuses this code) | any `bytes-get`/`bytes-set` index outside `0..len`, a byte value outside `0..=255`, a negative/overflowing `bytes-copy-range` range, a negative `Bytes::new` length | **Yes, heavily**: `bytes-get`/`bytes-set` are the dominant faulting primitive in the whole prelude (see §4) |
| `ReadOnly` | 71 | `abi.rs:201-211` `as_bytes_mut` on `KIND_BYTES_STATIC` | `bytes-set`/`bytes-copy-range` targeting a string literal's constant backing | **Yes**: nothing in the type system distinguishes a literal's `Bytes` from a heap one |
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
| `Nat` subtraction below zero | `nat-sub` (`Data/Nat.cinch:28,50`) | **none**: no `Option`/saturating `Nat` subtraction exists anywhere in the prelude (checked by grep: no `checked-sub`/`saturating-sub`/`monus` hit for `Nat`); only machine-scalar `checked-sub-i64` etc. (`Data/Machine.cinch`) are total | dozens: `Date.cinch:160,373`, `Time.cinch:251,365`, `Nat.cinch:50,80`, `Base64.cinch:188`, plus every indirect use through `(-)` on `Nat` (the `Sub Nat` instance, `Nat.cinch:49-50`) |
| `Nat`/`Int` division or remainder by zero | `nat-div`/`nat-rem`/`int-div`/`int-rem` (`Data/Nat.cinch:30-31`, `Data/Int.cinch:24-25`) | `Data.Rational.div` wraps a zero-denominator check and returns `Result RationalError Rational` (`Rational.cinch:150-153`); `Complex.div` is IEEE (never faults, `Complex.cinch:119-121`); no general `Nat`/`Int` `Option`-returning division exists | **by far the densest faulting family**: used throughout `Date.cinch`, `Time.cinch`, `BigDecimal.cinch`, `Base64.cinch`, `IpAddr.cinch`, `Random.cinch`, `Digest.cinch`, `Json.cinch`, `Otlp.cinch`, `Regex.cinch`, `Scan.cinch`, `Show.cinch`, `Toml.cinch`, `Uri.cinch`: tens of call sites per file in hex/digit/rendering code, well over 150 call sites prelude-wide |
| `Bytes` out-of-bounds index/value | `bytes-get`/`bytes-set` (`Data/Bytes.cinch:33-34`) | `Data.Http.byte-at` (`Http.cinch:213-217`) wraps one `bytes-get` in a bounds check returning `Option Nat`: the **only** such wrapper found; every other caller indexes raw and relies on hand-proven loop bounds | the single most call-site-dense faulting primitive: `Bool.cinch`, `Bytes.cinch`, `List.cinch`, `Machine.cinch`, `String.cinch`, `StringIo.cinch`, `Fs.cinch`, `Http.cinch`, `Process.cinch`, `Pty.cinch`, `Random.cinch`, `Base64.cinch`, `Builder.cinch`, `Digest.cinch`, `Json.cinch`, `Lex.cinch`, `Otlp.cinch`, `Parse.cinch`, `Regex.cinch`, `Scan.cinch`: well over 100 call sites prelude-wide, plus more in `bootstrap/` and `examples/source` |
| decimal parse failure | `nat-from-decimal`/`int-from-decimal` (raw runtime imports) | surface parsers in `Text.Parse`/`Text.Json`/`Text.Toml`/`Text.Csv` validate digit-by-digit (`digits-fold`, `all-digits`) *before* calling the parser, so most prelude use is effectively guarded; no `Option`-returning `Nat`/`Int` decimal parse is exported directly | low direct exposure (a handful of call sites) because callers pre-validate; still unguarded if a program calls the raw parser on unvalidated text |
| `Sym` interning of non-UTF-8 bytes | `sym-intern-bytes :: Bytes -> Sym` (`Data/Sym.cinch:33`) | `Data.Sym.intern :: String -> Sym` (`Sym.cinch:39-40`) is the documented total front door (a `String`'s bytes are always valid UTF-8) | effectively zero exposure through the documented API; the raw foreign import is exposed but not meant to be called directly |
| eval nesting too deep | `cinch_eval`/`eval :: Code a -> a` | none: nesting depth is inherent to the quoted program's own shape, no static bound | rare (quotation/supercompilation code only) |
| read-only buffer write | `bytes-set`/`bytes-copy-range` on a string-literal `Bytes` | none: a `String`'s backing and a heap `Bytes` are the same runtime type at this layer | moderate; anywhere a string literal's bytes are treated as a mutable buffer |
| `Nat`/`Int` → `I64` range overflow | `nat-to-i64`/`int-to-i64` | `nat-fits-i64`/`int-fits-i64` (`cinch_nat_fits_i64`/`cinch_int_fits_i64`) exist as total pre-checks, but most call sites convert directly without checking (e.g. `Show.cinch:117`, `Fs.cinch:112,123`, `Process.cinch:141,151`) | tens of call sites, mostly unguarded |

Refinement-typed indices (`Dec`/`decide`, `design`'s total-API pattern) exist for several **container-level** invariants: `NonZero` (`Data/NonZero.cinch:32-34`), `NonEmpty` (`Data/NonEmpty.cinch:43-45`), `Sorted` (`Data/Sorted.cinch:30-32`), `Ascii` (`Data/Ascii.cinch:30-32`): but there is **no general `Fin n`/bounded-index refinement type** anywhere in `specifications/`, `design/`, or `compiler/prelude/`; `Bytes`/`List` indexing has no refined-index total alternative at all, which is why `bytes-get`/`bytes-set`/`nat-sub`/division dominate the exposure counts above.

---

## 5. Resource exhaustion

| mechanism | site | behavior | exit code / termination |
|---|---|---|---|
| Heap cap | `heap.rs:118-130,175-179`; env `CINCH_HEAP_LIMIT` (default 8 GiB, `0` disables) | cumulative allocation (not live bytes) exceeding the cap | defined fault, exit **73** |
| Native stack overflow | `platform.rs:539-566` `cinch_run_main`; 1 GiB reserved stack (`DEFAULT_STACK_SIZE`, `platform.rs:535`), overridable via `CINCH_STACK_SIZE` | the reservation is address space only (pages commit on touch); a sufficiently deep **non-tail** recursion (every generated local lives in its own stack slot) exhausts even the 1 GiB reservation | **undefined termination**: Rust's guard-page handler on the thread prints `thread '<unnamed>' has overflowed its stack` and calls `abort()` (SIGABRT/SIGSEGV depending on platform), *not* one of the defined `EXIT_*` codes. This is the one genuinely open-ended abnormal-termination path a well-typed, otherwise-correct recursive program can hit just by processing a large enough input: tail calls are musttail-marked by the code generator (`codegen.rs:5671-5675,5746-5755`) and so don't count, but any non-tail structural recursion over an unbounded input does. |
| Eval nesting | `eval.rs:162,288-299` `MAX_NESTING = 1024` | bounded explicitly so quotation evaluation faults (`NestingTooDeep`, exit 71) instead of overflowing the native stack | defined fault, exit 71 |
| OS thread spawn failure | `task.rs:152-153` `Scope::spawn`; `platform.rs:555-558` `cinch_run_main` (graceful fallback only for the *first*, outermost thread) | `Scope::spawn` (structured-concurrency task spawn) calls `panic!` if `std::thread::Builder::spawn` fails (e.g. OS thread/process limits exhausted) | **undefined termination**: a Rust `panic!` crossing an `extern "C"` boundary aborts the process immediately (Rust panics never unwind across `extern "C"`); no stderr message in the defined format, no `EXIT_*` code |
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
| 1 | `bytes-get` | `OutOfBounds`, exit 71 | 100+ (dominant across `Text/*`, `Data/*`, `Effect/*`) |
| 2 | `bytes-set` | `OutOfBounds`/`ReadOnly`, exit 71 | 90+ |
| 3 | `nat-div` | `DivByZero`, exit 72 | 60+ |
| 4 | `nat-rem` | `DivByZero`, exit 72 | 60+ |
| 5 | `int-div` | `DivByZero`, exit 72 | 25+ |
| 6 | `int-rem` | `DivByZero`, exit 72 | 25+ |
| 7 | `nat-sub` | `NegativeResult`, exit 71 | 15+ |
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
