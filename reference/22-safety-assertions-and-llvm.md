# Safety assertions, must-use results, and LLVM contracts

A safety assertion is a claim you write on a definition that the checker then has to verify. A verified claim becomes a contract fact: it is recorded in the interface, hashed into the interface identity, and restated in the emitted LLVM where LLVM has the vocabulary for it. A claim the checker cannot verify is a compile error naming the attribute, the rule and the site, never a comment that everyone trusts.

[Attributes and pragmas](23-attributes-and-pragmas.md) covers the rest of the `#[...]` vocabulary (hints, metadata, layout, foreign crossing). [Type layout](24-type-layout.md) covers the representation these attributes talk about. [Refinements](06-refinements.md) covers the evidence a `decide` check produces, which is what `#[assert-min]` counts.

# Where an assertion may appear

A safety assertion goes immediately before the signature of a definition, one attribute per line, with its argument in parentheses. In a module body, that reads:

```cinch
#[recursion("none")]
#[no-alloc]
identity-word :: U32 -> U32
```

`#[profile("jpl")]` goes on a module header, before `module`. The parenthesized spelling is the only accepted one: the space form that the specification and the diagnostic text use, `#[recursion "none"]`, is refused with `unterminated attribute`. Read every message the checker prints with that difference in mind.

# What each assertion verifies

The report's `attribute` column is the string the manifest records, which is not always the source spelling.

| Source attribute | What the checker verifies | Report row and result |
| --- | --- | --- |
| `#[recursion("none")]` | no recursive call in the definition or its group | `recursion=none`: `verified (no recursive calls)` |
| `#[recursion("tail")]` | every self or mutual call is in tail position | `recursion=tail`: `verified (D104 tail marks)` |
| `#[recursion("bounded(xs)")]` | a depth bound, proved structurally when the argument is a structural descent | `recursion=bounded(xs)`: `structural (depth <= xs: the size of the decreasing argument)` |
| `#[recursion("bounded(3)")]` | a numeric depth bound, proved by a structural descent, a literal counter ladder, or the call graph, and the result says which | `recursion=bounded(3)`: `call-graph (the literal counter ladder: depth <= 3)` |
| `#[stack("constant")]` | the admitted non-growing-stack discipline, which implies tail or non-recursive | `stack=constant`: `verified` |
| `#[stack("bounded(2)")]` | a bound on stack growth, with the method reported | `stack=bounded(2)`: `call-graph (no recursive calls; depth 0)` |
| `#[no-alloc]` | no allocation site, and no call that violates the contract, in the body | `no-alloc`: `verified (0 allocation site(s) in the body)` |
| `#[alloc("once")]` | at most one allocation site runs per call | `alloc=once`: `verified (1 allocation site(s) run per call)` |
| `#[max-complexity(2)]` | cyclomatic complexity, counted as branches plus one | `complexity<=2`: `verified (cyclomatic complexity 2 <= 2)` |
| `#[max-lines(2)]` | source lines the definition spans | `lines<=2`: `verified (1 source line(s) <= 2)` |
| `#[assert-min(2)]` | at least two runtime `decide` or assert checks in the body; proof obligations do not count | `assert-min=2`: `verified (2 runtime \`decide\` check(s) >= 2)` |
| `#[checked-params]` | every parameter is validated before its first use | `checked-params`: `verified (every parameter validated before first use)` |
| `#[no-macros]` | no macro expansion in the definition's provenance | `no-macros`: `verified (no macro expansion inside the definition)` |
| `#[no-raw-ptr]` | no raw pointer or opaque type in the signature, and no foreign pointer operation in the body | `no-raw-ptr`: `verified (no raw pointer or opaque type in the signature, no foreign pointer operation in the body)` |
| `#[deny-warnings]` | warnings in scope are compile errors | `deny-warnings`: `verified (warnings in scope are compile errors)` |

The allocation analysis classifies construction sites by kind: constructor allocation, a local function (a closure), a lambda, a thunk, `Mut`, strings, `Bytes` and foreign aggregates. A violation names the kind and the site.

A bound is a claim the checker must establish, not a number it trusts. `#[recursion("bounded(3)")]` on a definition whose only descent is on an argument of unknown size is refused, and the refusal names the bound the proof actually gives: the established bound is structural (depth <= n: the size of the decreasing argument), which does not bound the depth by 3 (bounds that cannot be established are refused, never trusted; decision D113).

# The report

`cinch check` writes `build/<Module>.manifest.json`. Its `safety` object holds one row per verified assertion, plus the profile summary:

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

A definition that carries hints but no assertion has an empty list here, which is how you tell a hint from a verified fact: `#[inline(always)]` leaves `"assertions": []`.

# A definition with assertions, and the IR it emits

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

`cinch emit-ir Main.cinch -o main.ll` emits this definition and this attribute group:

```text
define range(i64 0, 4294967296) i64 @"Main.identity-word"(i64 noundef %p0) #1 {
```

```text
attributes #1 = { "probe-stack"="inline-asm" memory(none) nofree speculatable willreturn }
```

Which part comes from which analysis is decidable, because each claim is added only when its own analysis holds:

| Source | Attribute group on `Main.identity-word` |
| --- | --- |
| no attribute | `{ "probe-stack"="inline-asm" memory(none) speculatable }` |
| `#[no-alloc]` | `{ "probe-stack"="inline-asm" memory(none) nofree speculatable }` |
| `#[recursion("none")]` | `{ "probe-stack"="inline-asm" memory(none) speculatable willreturn }` |
| both, as above | `{ "probe-stack"="inline-asm" memory(none) nofree speculatable willreturn }` |

`memory(none)` and `speculatable` come from the empty effect row. `nofree` restates `#[no-alloc]`, and `willreturn` restates the totality proof; neither is guessed for a definition without the corresponding fact. `range(i64 0, 4294967296)` and `noundef` come from the parameter's type, not from any attribute: `U32` is a scalar word with a known width. `U8` states `range(i64 0, 256)`. A boxed `Int` parameter states neither, because the word spans the whole `i64` space:

```text
define i64 @"Main.idInt"(i64 %p0) #4 {
```

# A violation names the attribute, the rule, and the site

Allocation is the common case. `Cons` is an allocation site, and the message names the constructor:

<!-- cinch:reject contains="`#[no-alloc]` on Main.cons1 asserts the no-allocation rule (JPL rule 3): the constructor `Cons` is constructed here" -->
```cinch
module Main where
  #[no-alloc]
  cons1 :: List Nat -> List Nat
  cons1 xs = Cons 1 xs

  main :: Int
  main = 0
```

A recursion claim names the call that breaks it, and prints the specification's space form of the attribute rather than the source's parenthesized form:

<!-- cinch:reject contains="`#[recursion \"none\"]` on Main.loopy asserts the no-recursion rule (JPL rule 1): the recursive call at this site" -->
```cinch
module Main where
  #[recursion("none")]
  loopy :: Int -> Int
  loopy n = if n == 0 then 0 else loopy (n - 1)

  main :: Int
  main = loopy 2
```

`#[alloc("once")]` names the second site with `is constructed here as well`, and a construction under a closure with `is constructed under a closure here`. `#[max-complexity(2)]` reports the measured number against the claim.

# Must-use `Result` values

`Result` is not a value you may silently drop. A wildcard binding is not an acknowledgment:

<!-- cinch:reject contains="discarding a `Result`-typed expression without `ignore`" -->
```cinch
module Main where
  attempt :: Int -> Result Int Int
  attempt n = Ok n

  main :: Int
  main = let _ = attempt 1 in 0
```

Handle the outcome with `case`, or discard it deliberately with `ignore`. `ignore` records the decision; it does not convert a failure into a success, and it establishes no proof about the operation.

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

This is separate from the linearity of task handles, which is an ownership rule rather than a value-discard rule.

# The JPL profile

`#[profile("jpl")]` on a module makes the full set mandatory for every definition in it: bounded or absent recursion, no allocation, complexity at most 10, at most 60 source lines, at least two runtime checks, no macros, no raw pointers, and warnings as errors. The rule numbers come from the profile, so an individual attribute can be stricter but never looser.

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

The manifest shows what the profile demanded and what it found, one row per rule and definition. These are the rows for `Main.classify`:

| Attribute | Result |
| --- | --- |
| `jpl recursion` | `verified (no recursive calls)` |
| `jpl complexity<=10` | `verified (cyclomatic complexity 3)` |
| `jpl lines<=60` | `verified (5 source lines)` |
| `jpl assert-min=2` | `verified (2 runtime \`decide\` checks)` |
| `jpl alloc=none` | `verified (0 allocation site(s))` |
| `jpl no-macros` | `verified (no macro expansion inside the definition)` |
| `jpl no-raw-ptr` | `verified (no raw pointer or opaque type in the signature, no foreign pointer operation in the body)` |
| `jpl deny-warnings` | `verified (warnings in scope are compile errors)` |

Falling short is an error that names the rule number and the measurement. A function with no runtime `decide` cannot satisfy the runtime-check rule, and a comparison of two literals does not count, because it was decided at compile time:

<!-- cinch:reject contains="the `jpl` profile mandates 2 runtime checks on exported functions (decision D114)" -->
```cinch
#[profile("jpl")]
module Main where
  double :: Int -> Int
  double x = x + x

  main :: Int
  main = 0
```

This profile is a compiler profile inspired by the published rule set. It does not certify a product against any external standard, does not prove foreign C code correct, and has no CLI switch of its own: the attribute is the only way to select it.

# What the analyses contribute to emitted IR

Beyond the assertion restatements above, the backend translates other established analyses into LLVM's vocabulary. Every row is checked in both directions by the IR suite: the attribute appears where the analysis proves the fact and is absent where it does not.

| Established fact | Emitted form |
| --- | --- |
| Empty effect row, no foreign calls | `memory(none)`, `speculatable` |
| An `Io`-headed or mutating definition | `memory(read)` or `memory(readwrite)` instead |
| `#[no-alloc]` | `nofree` |
| A totality proof | `willreturn` |
| A non-escaping borrow | `captures(none)`, as in `declare void @"fix_borrow"(ptr captures(none) nonnull)` |
| A scalars-only signature | `noundef` on each scalar parameter |
| A bounded return type (`U8`, `U32`, a `Fin` index) | `range(...)` on the define, and `!range` metadata where a call's result range is statable |
| `#[checked-params]` and boundary facts | `llvm.assume` with `align`, `nonnull`, `dereferenceable` or `range` operand bundles |
| A read from an untrusted buffer, pointer or foreign input | `freeze` before a branch or comparison |
| A fault exit | `noreturn` and a dead successor |
| A foreign aggregate boundary | `byval` and `sret` |
| A `prim` immediate parameter, an `asm` fragment | `immarg`, and `llvm.sideeffect` for the assembly |

These names describe emitted IR. They are not source attributes: writing `#[nonnull]` on a Cinch definition does not exist and does not prove anything. Fast-math flags are not a default permission to reassociate IEEE arithmetic, and a reserved or opaque primitive body keeps its explicit trust and containment status rather than becoming proved by its own annotation.

- <span class="support-tag support-tag--planned">Coming soon</span> **Broader primitive semantic proofs and solver coverage.** The [verified primitives](../design/verified-primitives.md) and [proof automation](../design/proof-automation.md) designs describe the intended work. The annotations emitted today are translations of analyses that already hold; they are not those future proofs.

Implementation: `compiler/src/check_decls.rs` (assertions and their analyses), `check_terms.rs` (must-use and the runtime-check count), `core.rs` (contract facts), `codegen.rs` (the IR restatements). Suites: `safety.rs`, `llvm_contract.rs`, `attributes.rs`, `tail_calls.rs`.
