# Safety assertions, must-use results, and LLVM contracts

A **safety assertion** is a machine-checked property attached to a definition or module. Unlike an optimization hint (such as `#[inline]` or `#[unroll(4)]`), a safety assertion must be proved by the compiler:

- If the check succeeds, the verified claim becomes a **contract fact**: it is recorded in the module's interface (and hashed into `iface_hash`), reported in `build/<Entry>.manifest.json`, and translated into LLVM IR attributes wherever LLVM can express it.
- If the check fails, compilation fails with a diagnostic naming the attribute, the rule violated, and the offending site.

For complete attribute grammar, attachment positions, argument validation, and duplicate/conflict rules, see [Chapter 23](23-attributes-and-pragmas.md). For memory representation attributes (`#[repr]`, `#[unpack]`), see [Chapter 24](24-type-layout.md). For `decide` and refinement evidence, see [Chapter 06](06-refinements.md).

# Writing safety assertions

Per-definition assertions sit immediately before a function signature or top-level equation; `#[profile("jpl")]` and module-wide `#[deny-warnings]` sit before the `module` header:

```cinch
#[recursion("none")]
#[no-alloc]
identity-word :: U32 -> U32
```

In source code, arguments are written in parentheses (`#[recursion("none")]`) or with a compact `=` (`#[recursion="none"]`). Note that compiler diagnostics display the specification's space-separated spelling (`#[recursion "none"]`), which is not accepted source syntax.

# Verification semantics

Each safety assertion runs a dedicated static analysis over the definition (see [Chapter 23](23-attributes-and-pragmas.md#safety-assertions) for the full table of spellings and argument forms):

- **Recursion and stack bounds (`#[recursion("none"|"tail"|"bounded(b)")]`, `#[stack("constant"|"bounded(b)")]`):**
  - `"none"` verifies that the definition participates in no self- or mutual-recursion cycle.
  - `"tail"` (and `#[stack("constant")]`) verifies that every self or mutual recursive call is in tail position (or absent).
  - `"bounded(b)"` requires a verified bound—either a structural descent on a named argument or size index, or a numeric depth bound proved by a literal counter ladder / call-graph analysis. Bounds that cannot be established are always refused, never trusted; for example, `#[recursion("bounded(3)")]` is rejected on a function whose only established bound is structural in an argument of unknown size, or on a mutual-recursion cycle with no proved depth bound.
- **Allocation bounds (`#[no-alloc]`, `#[alloc("once")]`):**
  The checker tracks every allocation site by kind: data-constructor allocation, local function / lambda closure, memo thunk, `Mut` cell, string literal, `Bytes` buffer, and foreign aggregate. `#[no-alloc]` forbids all of them (and calls to definitions that violate the contract); `#[alloc("once")]` permits at most one allocation site per call and forbids allocation inside closures.
- **Structural metrics (`#[max-complexity(n)]`, `#[max-lines(n)]`):**
  Cyclomatic complexity is measured as decision branches plus one; line count is the checked body's source line span.
- **Defensive runtime checks (`#[assert-min(n)]`, `#[checked-params]`):**
  `#[assert-min(n)]` counts runtime `decide` checks in the body—compile-time-folded proof obligations (such as `decide (2 < 5)`) do not count. `#[checked-params]` requires every parameter to be validated by a refutable `case` match or runtime `decide` before its first use.
- **Hygiene, pointers, and warnings (`#[no-macros]`, `#[no-raw-ptr]`, `#[deny-warnings]`):**
  `#[no-macros]` rejects any AST node carrying macro-expansion provenance; `#[no-raw-ptr]` forbids raw pointers and opaque types in the signature and foreign pointer operations in the body; `#[deny-warnings]` promotes in-scope warnings to errors.

## Manifest safety report

`cinch check` records every verified assertion—and the method that proved it—under the `safety` key of `build/<Entry>.manifest.json`:

```json
{
  "assertions": [
    {
      "attribute": "recursion=none",
      "definition": "Main.identity-word",
      "result": "verified (no recursive calls)"
    },
    {
      "attribute": "no-alloc",
      "definition": "Main.identity-word",
      "result": "verified (0 allocation site(s) in the body)"
    }
  ],
  "jpl": null
}
```

Hints such as `#[inline(always)]` or `#[unroll(4)]` never appear in `safety.assertions`, making verified facts unambiguously distinguishable from advisory hints.

## Example: verified assertions and emitted LLVM attributes

<!-- cinch:run exit=0 -->
```cinch
module Main where
  #[recursion("none")]
  #[no-alloc]
  identity-word :: U32 -> U32
  identity-word x = x

  main :: Int
  main = if identity-word 42 == 42 then 0 else 1
```

Running `cinch emit-ir Main.cinch -o main.ll` emits:

```llvm
define range(i64 0, 4294967296) i64 @"Main.identity-word"(i64 noundef %p0) #1 {
...
attributes #1 = { "probe-stack"="inline-asm" memory(none) nofree speculatable willreturn }
```

Each LLVM attribute corresponds strictly to an established analysis fact:

| Source annotations on `identity-word` | Emitted LLVM attribute group |
| --- | --- |
| None | `{ "probe-stack"="inline-asm" memory(none) speculatable }` |
| `#[no-alloc]` | `{ "probe-stack"="inline-asm" memory(none) nofree speculatable }` |
| `#[recursion("none")]` | `{ "probe-stack"="inline-asm" memory(none) speculatable willreturn }` |
| `#[recursion("none")]` + `#[no-alloc]` | `{ "probe-stack"="inline-asm" memory(none) nofree speculatable willreturn }` |

- `memory(none)` and `speculatable` come from the empty effect row.
- `nofree` restates `#[no-alloc]`.
- `willreturn` restates the totality/termination proof.
- `range(i64 0, 4294967296)` and `noundef` come from the `U32` scalar type (`U8` emits `range(i64 0, 256)`; a boxed `Int` spans all `i64` bit patterns and emits neither).

## Violation diagnostics

A violated assertion names the attribute, the rule, and the offending construct:

<!-- cinch:reject contains="`#[no-alloc]` on Main.cons1 asserts the no-allocation rule (JPL rule 3): the constructor `Cons` is constructed here" -->
```cinch
module Main where
  #[no-alloc]
  cons1 :: List Nat -> List Nat
  cons1 xs = Cons 1 xs

  main :: Int
  main = 0
```

<!-- cinch:reject contains="`#[recursion \"none\"]` on Main.loopy asserts the no-recursion rule (JPL rule 1): the recursive call at this site" -->
```cinch
module Main where
  #[recursion("none")]
  loopy :: Int -> Int
  loopy n = if n == 0 then 0 else loopy (n - 1)

  main :: Int
  main = loopy 2
```

Similarly, `#[alloc("once")]` identifies any second allocation site (`is constructed here as well`) or allocation inside a closure (`is constructed under a closure here`), and `#[max-complexity]` / `#[max-lines]` report the measured figure against the declared cap.

# Must-use `Result` values

`Result` values are must-use: binding one to `_` without handling or explicitly discarding it is a compile error:

<!-- cinch:reject contains="discarding a `Result`-typed expression without `ignore`" -->
```cinch
module Main where
  attempt :: Int -> Result Int Int
  attempt n = Ok n

  main :: Int
  main = let _ = attempt 1 in 0
```

Either match both outcomes with `case`, or acknowledge the discard explicitly with `ignore` (which returns `Unit` without pretending the operation succeeded):

<!-- cinch:run exit=0 -->
```cinch
module Main where
  attempt :: Int -> Result Int Int
  attempt n = Ok n

  settled :: Int -> Int
  settled n = case attempt n of
    Ok v -> v
    Err e -> 0

  discarded :: Int
  discarded = let _ = ignore (attempt 1) in 7

  main :: Int
  main = if settled 5 == 5 && discarded == 7 then 0 else 1
```

Must-use checking is independent of linear ownership (such as `Task` handles, which must be consumed exactly once regardless of `ignore`).

# The JPL profile

Attaching `#[profile("jpl")]` to a module header enforces a strict baseline across every definition in the module:

| Rule | Module-wide requirement |
| --- | --- |
| Recursion | Non-recursive, tail-recursive, or statically bounded (`recursion`/`stack` proof required for recursive definitions). |
| Allocation | No heap allocation unless the definition explicitly asserts `#[alloc("once")]` (initialization pattern). |
| Complexity & size | Cyclomatic complexity at most `10`; body at most `60` source lines. |
| Runtime checks | At least `2` runtime `decide` checks in each exported definition (compile-time folded checks do not count). |
| Macros & pointers | No macro expansion provenance, no raw pointers or opaque types in signatures, and no foreign pointer operations. |
| Warnings | All in-scope warnings are errors. |

Individual definitions may add **stricter** assertions (for example `#[max-complexity(4)]`), but never looser ones (`#[max-complexity(12)]` is rejected). `#[checked-params]` remains opt-in per definition. There is no CLI flag for profiles; `#[profile("jpl")]` on the module header is the sole activation mechanism.

<!-- cinch:run exit=0 -->
```cinch
#[profile("jpl")]
module Main where
  classify :: Nat -> Nat
  classify x = case decide (x < 5) of
    Yes _ -> case decide (x < 3) of
      Yes _ -> 1
      No _ -> 2
    No _ -> 3

  main :: Int
  main = case 2 + 2 of
    n -> case decide (n < 5) of
      Yes _ -> case decide (n < 3) of
        Yes _ -> 1
        No _ -> 0
      No _ -> 3
```

In `build/Main.manifest.json`, the profile reports one row per rule for every definition (for `Main.classify`: `jpl recursion`, `jpl complexity<=10`, `jpl lines<=60`, `jpl assert-min=2`, `jpl alloc=none`, `jpl no-macros`, `jpl no-raw-ptr`, and `jpl deny-warnings`, each with its measured value).

A definition that falls short of any rule fails compilation—for example, omitting runtime `decide` checks:

<!-- cinch:reject contains="the `jpl` profile mandates 2 runtime checks on exported functions" -->
```cinch
#[profile("jpl")]
module Main where
  double :: Int -> Int
  double x = x + x

  main :: Int
  main = 0
```

`#[profile("jpl")]` is a compiler-enforced discipline inspired by NASA/JPL's Power of 10 rules; it does not certify compliance with external safety standards and does not verify foreign C code.

# LLVM IR contract lowering

Beyond explicit safety assertions, the backend lowers verified compiler analyses into LLVM attributes and metadata whenever the analysis proves the underlying property (and omits them whenever it does not):

| Verified compiler fact | Emitted LLVM IR form |
| --- | --- |
| Empty effect row, no foreign calls | `memory(none)`, `speculatable` |
| `Io`-bearing or mutating definition | `memory(read)` or `memory(readwrite)` |
| `#[no-alloc]` | `nofree` |
| Totality / termination proof | `willreturn` |
| Non-escaping borrow parameter | `nocapture` (upgraded to `captures(none)` by LLVM 21+) |
| Scalar-only parameter | `noundef` |
| Bounded return type (`U8`, `U32`, `Fin`) | `range(...)` on `define` and `!range` metadata on calls where statable |
| `#[checked-params]` and boundary facts | `llvm.assume` with `align`, `nonnull`, `dereferenceable`, or `range` operand bundles |
| Read from untrusted buffer, pointer, or foreign input | `freeze` prior to branching or comparison |
| Fault exit | `noreturn` with unreachable/dead successor |
| Foreign aggregate crossing (`#[byval]`, `#[sret]`) | Target C ABI lowering: register pieces, `byval(<layout>)`, caller-copy pointer, or `sret(<layout>)` |
| `prim` `immarg` parameter / inline `asm` | `immarg` parameter attribute; `llvm.sideeffect` for inline assembly |

These LLVM names are backend outputs derived from checked invariants—they are **not** Cinch source attributes (writing `#[nonnull]` or `#[willreturn]` in source is an unknown-attribute error). Fast-math reassociation is never enabled by default, and `unsafe prim` bodies retain their explicit trust boundary rather than inheriting unproved guarantees.

<span class="support-tag support-tag--planned">Coming soon</span> **Broader primitive semantic proofs and solver coverage.** Planned verification of primitive bodies and expanded automated proof discharge will continue to require independently checkable evidence.
