# Foreign code, representation attributes, and inline primitives

A `foreign` declaration or inline `prim` body crosses Cinch's trust boundary. The compiler enforces everything visible at the boundary—calling convention, C-compatible layouts, effect rows, ownership roles, and (for `prim`) inline IR containment—while recording LLVM code generation and the runtime under `lowering.trusted` in the artifact manifest rather than claiming a proof over external code.

# Foreign declarations

```text
foreign import ccall "symbol" local-name :: Type
foreign export ccall "symbol" local-name :: Type
```

The surface grammar reserves two calling conventions: `ccall` and `component`. Only `ccall` is supported on `foreign` declarations directly; WebAssembly Component Model boundaries are generated through WIT adapters (see [Chapter 19](19-targets-and-components.md)).

## Effects at the boundary

Foreign imports are never pure by default. The return arrow must declare at least one foreign-boundary effect from `Foreign <Boundary>`, `IO`, `Global`, `Blocking`, `Diverge`, `Fail`, `Reentrant`, or `ThreadAffine`:

<!-- cinch:reject contains="a foreign import must declare its effects" -->
```cinch
module Main where
  foreign import ccall "cinch_clock_wall_ns" wall-ns :: () -> U64

  main :: Int
  main = 0
```

The boundary tag after `Foreign` is user-chosen (`Lib`, `Codec`, `Fix`, `Component`, or any domain name) and is tracked through the effect row up to `main :: Unit ->{e} Int`.

## Marshaled types

Only types with a defined C ABI representation may cross a `ccall` boundary:

- **Admitted:** machine scalars (`I8`–`I64`, `U8`–`U64`, `Isize`, `Usize`, `F32`, `F64`), `Unit` (`()`), `Bytes`, `RawPtr t`, `opaque type`, and aggregates annotated with `#[repr(C)]`, `#[repr(packed)]`, or `#[repr(transparent)]`.
- **Rejected:** arbitrary-precision `Int` and `Nat`, closures, type-class dictionaries, vectors, dependent pairs, erased proofs, unannotated (`uniform-word`) records, and fixed-discriminant `data` types.

## Linking a foreign import

`cinch build` and `cinch run` automatically link the Cinch runtime archive (`libcinchruntime.a`), so runtime symbols such as `cinch_clock_wall_ns` resolve without extra flags:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  foreign import ccall "cinch_clock_wall_ns" wall-ns :: () ->{Foreign Lib, Diverge} U64

  main :: Unit ->{Foreign Lib, Diverge} Int
  main u = if eq-u64 (wall-ns ()) 0 then 1 else 0
```

In `cinch emit-ir`, the compiler emits the external C declaration plus a wrapper unit that passes the result through LLVM `freeze` so uninitialized or poison bits from foreign code cannot corrupt downstream optimization:

```llvm
declare i64 @"cinch_clock_wall_ns"()

define i64 @"Main.wall-ns"(i64 %p0) #0 {
entry:
  %t0 = call i64 @"cinch_clock_wall_ns"()
  %t1 = freeze i64 %t0
  ret i64 %t1
}
```

## Passing byte buffers to C

`Bytes` values are reference-counted handles, not raw pointers. `bytes-data` extracts the payload's base address as a machine word for C functions expecting `const char *` or `void *`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  foreign import ccall "strlen" c-strlen :: I64 ->{Foreign Lib, Diverge} I64

  main :: Unit ->{Foreign Lib, Diverge} Int
  main u =
    let raw = string-to-bytes "note.txt"
        n = bytes-len raw
        path = bytes-new (n + 1)
        _ = bytes-copy-range path raw 0 0 n
        _ = bytes-set path n 0
    in if eq-i64 (c-strlen (bytes-data path)) 8 then 0 else 1
```

Three invariants govern `bytes-data`:

- **Pinned payload:** a `Bytes` buffer's payload never moves while the handle is live.
- **Caller retains ownership:** `bytes-data` does not transfer or extend lifetime; keep the `Bytes` binding live across the foreign call to avoid dangling reads.
- **Not NUL-terminated:** `Bytes` tracks explicit length, so C strings require allocating `n + 1` bytes and writing a trailing `0`, as shown above.

## Calling OS libraries directly

Because `ccall` links against the platform C library, OS interfaces can be bound directly in Cinch. The standard library splits low-level filesystem operations into `Prim.Sys.Darwin` (`foreign import ccall` bindings to `libSystem`), `Prim.Sys.Linux` (`prim` bodies issuing native `syscall` instructions), and `Prim.Sys.Portable`, with `Prim.Sys` dispatching between them by compile target.

On Darwin, a complete file create, write, seek, read, and unlink round trip uses `Prim.Sys.Darwin` directly (using fixed-arity `creat`, since variadic C functions such as `open` cannot be declared via `ccall`):

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.Bytes
  import Data.Nat
  import Data.Result
  import Data.String
  import Prim.Sys.Darwin

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    case sys-create "doc-fs-probe.txt" of
      Err e -> 1
      Ok fd -> case sys-write fd (string-to-bytes "hello") 0 5 of
        Err e -> 2
        Ok n -> case n == 5 of
          False -> 3
          True -> case sys-close fd of
            Err e -> 4
            Ok _ -> case sys-open-read "doc-fs-probe.txt" of
              Err e -> 5
              Ok rd -> case sys-lseek rd 0 sys-seek-end of
                Err e -> 6
                Ok size -> case size == 5 of
                  False -> 7
                  True -> case sys-lseek rd 0 sys-seek-set of
                    Err e -> 8
                    Ok _ ->
                      let buf = bytes-new 5 in
                      case sys-read rd buf 0 5 of
                        Err e -> 9
                        Ok got -> case got == 5 of
                          False -> 10
                          True -> case sys-close rd of
                            Err e -> 11
                            Ok _ -> case string-from-bytes buf of
                              None -> 12
                              Some text -> case text == "hello" of
                                False -> 13
                                True -> case sys-unlink "doc-fs-probe.txt" of
                                  Err e -> 14
                                  Ok _ -> 0
```

# Linking native objects, libraries, and C++

To link your own C, C++, or Rust code with a Cinch program, pass compiled objects or static archives with `--native-object` (repeatable), or emit native library outputs directly:

- `cinch build Main.cinch --native-object helper.o --native-object libfoo.a -o prog` links additional native objects into an executable (`cinch run` accepts `--native-object` as well).
- `cinch build Lib.cinch --shared -o libfoo.so` (or `.dylib`) emits a self-contained native shared library without requiring `main`.
- `cinch emit-object Lib.cinch -o libfoo.o` emits a closed native object file plus its `.abi.json` manifest.
- `cinch cxx <bindings.json> --out-dir <dir>` and `cinch cxx build ...` generate C++17 bindings and build native C++ exports or Godot GDExtensions.

Package-level native link inputs (`link`, `local_link`, `pkg-config`, and `--print-link-plan`) are covered in [Chapter 01](01-using-the-compiler.md#output-linking-and-optimization) and [Chapter 29](29-package-management.md).

A `foreign import` is a link-time obligation: `cinch check` validates its Cinch signature and layout, while unresolved symbols fail at link time when `clang` links the final binary:

```text
$ cinch build Main.cinch -o prog
cinch: clang failed to link prog.ll: undefined reference to `docs_no_such_symbol'
clang: error: linker command failed with exit code 1 (use -v to see invocation)
```

# Foreign exports

`foreign export ccall` exposes an already-declared Cinch function under a C symbol name (the Cinch signature must precede the export):

<!-- cinch:run exit=0 -->
```cinch
module Main where
  answer :: I64 -> I64
  answer x = wrapping-add-i64 x 1

  foreign export ccall "cinch_docs_answer" answer :: I64 -> I64

  main :: Int
  main = if answer 41 == 42 then 0 else 1
```

The backend emits a C-callable entry point that freezes each incoming machine word before invoking the Cinch unit:

```llvm
define i64 @"cinch_docs_answer"(i64 %c0) {
entry:
  %t0 = freeze i64 %c0
  %t1 = call i64 @"Main.answer"(i64 %t0)
  ret i64 %t1
}
```

# Aggregate crossings: `#[byval]` and `#[sret]`

An aggregate crossing a C boundary must declare an explicit C-compatible representation (`#[repr(C)]`, `#[repr(packed)]`, or `#[repr(transparent)]`); unannotated `uniform-word` records are rejected.

By default, an aggregate parameter crosses as a pointer to a call-scoped layout copy, and an aggregate return value is rejected unless marked. `#[byval]` and `#[sret]` attach to `foreign` declarations to request by-value parameter passing and by-value aggregate returns respectively:

<!-- cinch:check -->
```cinch
module Main where
  #[repr(C)]
  record Big where
    a :: I64
    b :: I64
    c :: I64

  #[byval]
  foreign import ccall "sum_big" sumBig :: Big ->{Foreign Lib, Diverge} I64

  #[sret]
  foreign import ccall "mk_big" mkBig :: I64 ->{Foreign Lib, Diverge} Big

  main :: Unit ->{Foreign Lib, Diverge} Int
  main u = if sumBig (mkBig 5) == 18 then 0 else 1
```

LLVM IR does not automatically lower C struct-by-value rules; Cinch lowers `#[byval]` and `#[sret]` directly according to the active target's C ABI:

| Target ABI | `#[byval]` parameter | `#[sret]` result |
| --- | --- | --- |
| **System V x86-64** (Linux, macOS) | Up to 16 bytes in registers (`i64` integer eightbytes; `double`, `<2 x float>`, or `float` FP eightbytes); larger, misaligned, or register-exhausted structs via `byval` stack copy. | Up to 16 bytes in registers; larger via `sret` buffer (`void` LLVM return). |
| **AAPCS64** (ARM64 Linux, macOS) | HFA of 1–4 `float`/`double` as `[N x float]` / `[N x double]`; otherwise up to 16 bytes as `i64` / `[2 x i64]`, larger as pointer (`ptr nonnull align N`) to caller's copy. | Same register rules; larger via `sret` buffer. |
| **Microsoft x64** | 1, 2, 4, or 8-byte struct as integer of that width; otherwise pointer to caller's copy. | Same integer rule; larger via `sret` buffer. |
| **wasm32** | Single-scalar struct as that scalar; otherwise `byval`. | Same scalar rule; larger via `sret` buffer. |

For the 24-byte `Big` above, x86-64 emits `ptr byval(%__sl_layout_0) align 8` for `sum_big`, ARM64 emits `ptr nonnull align 8`, and both emit `ptr sret(%__sl_layout_0) align 8` with a `void` return for `mk_big`. Aggregates containing 16-byte scalars or targeting an unsupported C ABI are rejected rather than guessed.

Invalid placements or mismatches are rejected during checking:

- `#[byval]` on a binding with no aggregate parameter.
- `#[sret]` on a binding whose return type is not an aggregate.
- An aggregate return value without `#[sret]` (return via an `Out` cell, `Owned` pointer, or add `#[sret]`).
- Either attribute placed on an ordinary Cinch definition instead of a `foreign` declaration.

# Representation and function attributes

Every attribute outside `#[meta(...)]` must be compiler-owned or declared by a macro (`macro name attributes (...)`); unrecognized names are check errors.

- **Layout (`#[repr(...)]`):** `#[repr(C)]`, `#[repr(packed)]` / `#[repr(packed(N))]`, `#[repr(align(N))]`, `#[repr(transparent)]`, and `#[repr(u8|u16|u32|u64)]` define explicit byte layouts and interface ABI identity. Changing a type's `#[repr]` changes its recorded layout and invalidates dependent interfaces (see [Chapter 17](17-upgrades-and-temporal-contracts.md)). For exact padding, alignment, and rejection rules, see [Chapter 24](24-type-layout.md) and [Chapter 23](23-attributes-and-pragmas.md#repr-every-current-variant).
- **Function and module hints:**

| Attribute | Placement | Effect |
| --- | --- | --- |
| `#[inline]`, `#[inline(always)]`, `#[inline(never)]`, `#[noinline]` | Function declaration | Inlining hint (at most one per function; refused on `foreign` declarations). `#[inline(never)]` / `#[noinline]` blocks both driver unfolding and Core inlining in addition to emitting LLVM `noinline`. |
| `#[cold]` | Function declaration or decision branch | Marks the function entry cold or sets branch weight `0`. |
| `#[unroll(N)]`, `#[unroll(0)]`, `#[unroll(disable)]` | Function declaration | Advisory loop-unrolling hint lowered to `llvm.loop.unroll.count` or `llvm.loop.unroll.disable`. Never alters semantics or unrolls at Core level; accepted on loop-free functions (emits no metadata). Counts above `2147483647` (`i32::MAX`) are rejected. |
| `#[weight(n)]`, `#[likely]`, `#[unlikely]` | Clause, `case` alternative, guard, `if` | Decision-point branch weights. |
| `#[meta(...)]` | Any declaration, field, constructor, or branch | Opaque tooling metadata with no semantic or runtime effect. |
| `#[profile("jpl")]`, `#[deny-warnings]` | Module header | Module-wide safety profile or warning escalation. |

`#[meta(...)]` attaches arbitrary non-semantic metadata anywhere an attribute is permitted:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  #[meta(owner = "docs")]
  answer :: Int
  answer = 1

  main :: Int
  main = if answer == 1 then 0 else 1
```

For the full attribute grammar, placement matrix, and safety assertions (`#[no-alloc]`, `#[recursion(...)]`, `#[stack(...)]`, and friends), see [Chapter 23](23-attributes-and-pragmas.md) and [Chapter 22](22-safety-assertions-and-llvm.md).

# Inline primitives

A `prim` declaration pairs an ordinary Cinch type signature with one or more inline target-IR bodies:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  prim add-word :: U32 -> U32 -> U32 where
    llvm {
      %sum = add i32 %0, %1
      ret i32 %sum
    }

  main :: Int
  main = if add-word 40 2 == 42 then 0 else 1
```

The recognized target-IR family names are `llvm`, `wasm`, `asm`, `clr`, and `jvm`. Today, `llvm` and admitted `asm` lower on native targets (and `wasm` on WebAssembly profiles); `clr` and `jvm` are reserved and rejected by name (`the 'clr' target-IR family is reserved`).

Bodies accept trailing clauses:

- `when <atom>` — target-profile predicate selecting when the body applies.
- `justify <term>` — records soundness evidence for a representation choice.
- `uses (name :: type), ...` — declares external callee vocabulary the body may invoke.
- `immarg 0, 1` — marks parameters that must lower as immediate constants.
- Leading `unsafe` — opts out of containment checking for that body, trusting its declared effect row.

## Target conditions

Each `when` clause evaluates one atom against the **compile target profile** (never the build host), and any atom may be negated with `not (<atom>)`:

| Atom | Holds when |
| --- | --- |
| `Available <capability>` | The target profile carries capability evidence for `<capability>`. |
| `arch <name>` | Target architecture matches `x86_64`, `aarch64`, or `wasm32`. |
| `os <name>` | Target operating system matches `darwin`, `linux`, or `windows`. |
| `libc <name>` | Target C library matches `glibc`, `musl`, `libSystem`, or `none`. |
| `endian <little\|big>` | Target byte order matches. |
| `word-size <bits>` | Target `Usize`/`Isize` bit width is `32` or `64`. |
| `target <triple>` | Full target triple matches verbatim. |

Multiple `when` clauses on one body form a conjunction (`when os linux when arch aarch64`). The exact same condition vocabulary governs conditional imports (`import Mod when ...`) and conditional declarations (`when <atom> <decl>`); see [Chapter 07](07-modules-and-packages.md#conditional-imports-and-declarations).

A matching conditional body takes precedence over an unguarded fallback body; if no body matches the active target, checking fails with the failing atom and actual target values:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  prim pick :: U32 -> U32 where
    llvm {
      %s = add i32 1, %0
      ret i32 %s
    } when not (os windows)
    llvm {
      %s = add i32 %0, 1
      ret i32 %s
    }

  main :: Int
  main = if pick 1 == 2 then 0 else 1
```

## Containment rules

Non-`unsafe` inline bodies are parsed and validated before code generation to prevent silent escapes from Cinch's safety guarantees:

| Disallowed IR construct | Containment refusal |
| --- | --- |
| Direct I/O calls (e.g. `call i32 @puts(...)`) | `containment (no I/O): '@puts' is I/O (a body performs no I/O)` |
| Fault paths (`unreachable`) | `containment (no fault paths): 'unreachable' is a fault path beyond a declared failure mode` |
| Unwinding (`invoke`, `resume`, `landingpad`, `catchswitch`) | `containment (no-unwind): ... may raise or unwind out of the body` |
| Control-flow escape (`br`, `switch`, `indirectbr`) | `containment (no-escape): ... is a jump` |

Body selection decisions and their reasons are recorded in `build/<entry>.manifest.json`.

See the [FFI specification](../specifications/foreign-function-interface-specification.md) §§1–4, [attribute annotations specification](../specifications/attribute-annotations-specification.md), [inline target IR specification](../specifications/primitive-operations-and-inline-target-ir-specification.md), and the [LLVM contract attributes design note](../design/llvm-contract-attributes.md).
