# Layouts, optimization, and code generation

This document covers how Cinch represents values in memory and across function boundaries, how the simplifier and code generator compile functional and continuation-passing code into direct native/Wasm code, and how inline target IR (`prim`), LLVM attributes, debug info, and JIT compilation work.

User-facing guides live in [`reference/12-rewrites-and-optimization.md`](../reference/12-rewrites-and-optimization.md), [`reference/15-ffi-layout-and-attributes.md`](../reference/15-ffi-layout-and-attributes.md), and [`reference/24-type-layout.md`](../reference/24-type-layout.md).

---

## 1. Representation classes and specialization

Every `Core` binder, parameter, result, and constructor field carries one of four representation classes (`seed/src/repr.rs`):

| Class | Letter | Words | Counted | LLVM type |
| --- | --- | --- | --- | --- |
| `Raw` | `R` | 1 | No | `i64` (or narrowed width at direct boundaries) |
| `Obj` | `O` | 1 | Yes | `i64` (heap/static pointer) |
| `TObj` | `T` | 1 | If bit 0 is `0` | `i64` (`Nat`/`Int` immediate or pointer) |
| `Wide` | `W` | 2 | No | `i128` (`I128`, `U128`) |

### Demand-driven class specialization
Generic functions and constructors are specialized on demand (`seed/src/specialize.rs`) by the class vector of their reached instantiations (e.g. `map@OR`, `Some@W`). Only class vectors actually reached in the program are emitted, bounding specialization cleanly without monomorphizing per concrete nominal type.

### Narrow boundary crossings
While polymorphic slots remain uniform machine words, monomorphic direct calls cross at their proved narrow width (`seed/src/codegen.rs`, `Registry::narrow_sigs`):
- Machine scalars cross at their native LLVM width (`U8`/`I8` as `i8`, `U16`/`I16` as `i16`, `U32`/`I32`/`F32` as `i32`), with signed values sign-extending and unsigned/float bits zero-extending at the boundary.
- Counted pointers, `Nat`, and `Int` cross at target pointer width (`csize`: `i64` on 64-bit native targets, `i32` on `wasm32`, matching the immediate bit bounds).
- Apply stubs (`U.app`, one per closure-taken unit) marshal between the word-based indirect apply frame and a unit's narrowed signature without introducing per-arity stub fan-out.

### Wide scalars (`I128`, `U128`)
`I128` and `U128` are kernel type names whose operations live in the opt-in `Data.Wide` module:
- **Wide positions** (locals, direct unit parameters/results, specialized constructor fields, `Proj`) hold `i128` directly (two consecutive uncounted words in constructor payloads, 8-aligned).
- **Word positions** (indirect `__apply` frames, closure environments, dictionary slots, `Demand`/`Lazy`/`TVar`/task payloads) hold one word.
- An explicit edge pass (`seed/src/wide.rs`) inserts `<builtin>.WideBox` / projection unboxing wherever a `Wide` value crosses into or out of a word position, before RC insertion so boxes are reference-counted normally.
- At the C FFI boundary, `I128`/`U128` cross as `__int128` / `unsigned __int128` (`i128`); `wasm32` links compiler-rt helpers (`__multi3`, `__divti3`, etc.) through `compiler_builtins`.

### Future layout work
Sub-word heap field packing (`List@unboxed_1_bytes`), `#[repr(unboxed)]` multi-word value types with expanding-edge boxing, and proof-justified custom representation instances (`Represent t c`, `via`) remain planned extensions on top of this model.

---

## 2. Compiling CPS and higher-order code to direct code

Six local, budgeted transformations (`seed/src/simplify.rs`, `seed/src/stack.rs`, `seed/src/codegen.rs`) eliminate closure allocation, indirect calls, and stack growth in continuation-passing and callback-heavy code:

| # | Optimization | Where | Flag / switch |
| --- | --- | --- | --- |
| 1 | **Guaranteed tail calls (`tailcc`)** | codegen | 64-bit native targets |
| 2 | **Contification** (tail-only local lambdas -> join points) | simplifier | `--no-contify` |
| 3 | **Direct projection from capturing dictionaries & polytypes** | simplifier | `--no-direct-dict-calls` |
| 4 | **Call-pattern specialization** on known closures & function records | simplifier | `--no-specialise-arguments` |
| 5 | **Frame-allocated continuations** for non-tail consumed closures | `stack.rs` | `--no-stack-alloc` |
| 6 | **Constructed product results (CPR)** in registers (1–3 fields) | codegen | `CINCH_NO_CPR=1` |
| 7 | **Constructed sum results (CSR)** in `{ i64, i64 }` registers (2-variant 0/1-word sums) | codegen | automatic (`#cc` / `#csr`) |

### Key mechanics
- **Selective `tailcc`:** only units in the greatest fixed point of guaranteed tail calls use LLVM's `tailcc` (`X#cc` behind a C-ABI wrapper `X`), avoiding x86-64 callee-pops overhead on non-tail leaves. On ELF 64-bit targets, apply stubs carry GHC-style prefix data (`CINCHTL2` + tail twin `X.tapp`) so tail calls through unknown closures execute as `musttail call tailcc` in constant stack (Mach-O keeps known `tailcc` calls but omits prefix-data indirect twins due to Mach-O text-relocation limits; Wasm/32-bit keep `#tail` widened workers).
- **Contification:** converts `let k = \... -> ...` where every occurrence of `k` is a saturated tail call into a `Join`/`Jump` block carrying counted captures explicitly so RC remains sound.
- **Call-pattern specialization:** redirects calls passing known closures, static records, or constructors containing functions to cached copies (`f.spN`, budgeted at 4/callee, 256/program, `<= 4000` Core nodes), including self-recursive calls.
- **Constructed product results (CPR):** units whose tail leaves all construct the same 1–3 word product return `{ i64, ... }` directly in registers; callers that immediately `case`-destructure the result (`virtual_let`) never allocate a heap block.
- **Constructed sum results (CSR):** functions returning an eligible two-variant sum with 0/1-word payloads (`Option`, `Result`, user two-constructor `data` types) emit an unboxed `{ i64, i64 }` (`{ tag, payload }`) worker (`#cc` in `seed/`, `#csr` in `compiler/`) called directly at `case`-scrutinized and recursive sites with exact Perceus RC on matched or discarded payloads, while keeping a standard boxed C-ABI wrapper for escaping or higher-order uses.

---

## 3. Unfolding inliner

The simplifier (`seed/src/simplify.rs`) performs budgeted unfolding inlining guided by AST size, call-site context (known arguments, constructor scrutiny, higher-order callbacks), and recursion guards:
- Small wrappers, immediate arithmetic/byte accessors, and single-use bindings inline unconditionally within budget.
- Recursive definitions are never unrolled blindly; worker/wrapper and call-pattern specialization expose the loop body instead.
- Inline primitives expose hot tagged/bounds paths to LLVM while keeping cold fallback/fault paths out of line.

---

## 4. Inline target IR (`prim`) and LLVM contract attributes

### `prim` declarations
Normative specification: [`specifications/primitive-operations-and-inline-target-ir-specification.md`](../specifications/primitive-operations-and-inline-target-ir-specification.md).

`prim` declarations externalize target-specific primitives (`llvm`, `wasm`, `asm`) as verbatim braced fragments gated by `when <target-cond>`, optional `justify <proof>`, and explicit foreign contracts (`uses`):
- Each body is validated for containment (e.g., pure `prim`s cannot perform hidden stores) and selected per target profile, with selections recorded in `build/<Entry>.manifest.json`.
- A `prim` is `justified` if every body carries checked evidence, and `assumed` otherwise (hashing the body into the module contract).

### LLVM contract attributes
Cinch translates facts its static analyses already prove directly into LLVM IR attributes without speculative guessing:
- **From effect rows & safety attributes:** pure functions without foreign calls emit `memory(none)` / `speculatable`; `#[no-alloc]` adds `nofree`; proven-terminating/bounded definitions emit `willreturn`.
- **From ownership & types:** non-escaping borrows emit `captures(none)` (on LLVM versions that support it), non-null references emit `nonnull`, scalars emit `noundef`, refined ranges emit `!range` metadata, and `#[byval]`/`#[sret]` control aggregate FFI passing.

---

## 5. Debug info, unwind info, and frame chain

The seed emitter supports DWARF debug metadata and native unwind tables (`seed/src/codegen.rs`, implemented for native targets):
- Emits DWARF compile unit, file/line locations (`!dbg`) mapped from Core `@span` annotations, and frame-pointer/unwind metadata (`uwtable`) so native debuggers (`lldb`/`gdb`) and profilers (`sample`, `perf`) resolve Cinch symbols and stack traces cleanly.
- Stack probing (`"probe-stack"="inline-asm"`) is emitted on every native unit so large frames cannot jump past the 64 KiB stack guard band.

---

## 6. Supercompilation and JIT

- **JIT (`--features jit`):** `cinch jit` compiles to LLVM IR and executes in-process via LLVM's ORC LLJIT against `libcinchruntime` (supporting single-file execution, prebuilt `--package-object` calls, and `--native-stage` / `--serve` hot loops with a resident Stage 3 compiler).
- **Offline supercompilation & verified rewrites:** `rewrite ... justify` rules fire during specialization when backed by kernel proofs or declared laws, with every firing and stop reason recorded in `build/<Entry>.manifest.json`. Full whole-program supercompilation and runtime adaptive JIT remain long-term research directions documented in [`specifications/supercompilation-and-jit-specification.md`](../specifications/supercompilation-and-jit-specification.md).
