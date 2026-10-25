# Attribute annotations, representations, and branch weights

Source: local draft in this repository (added after the documentation export)
Added: 2026-10-25

---

[Documentation overview](../overview.md)

Draft 0.1. This chapter defines the attribute annotation system: `#[...]`
annotations attached to AST nodes, their syntax, their ownership by either
the compiler or the macro machinery, and their identity rules. It then
defines the first attribute families: manual branch weights with the
`cold`/`likely`/`unlikely` wrappers, inline hints, and manual memory
layout, representation, and alignment control. MUST denotes a requirement
of this draft. Proposed choices are identified explicitly; unresolved items
are listed in Open decisions. The branch weight semantics in section 3
follow the weight mathematics of GHC proposal 744 (Branch Prediction
Hints, Ian Duncan), which this chapter treats as the reference for weight
algebra: relative weight spaces, marginalization through pattern
decomposition, and the simplifier reconciliation rules.

# 1 The attribute system

An attribute is a name with optional arguments written `#[...]` in prefix
position before the annotated construct:

```
attribute    = "#[", attribute_name, [ attribute_args ], "]"
attribute_name = name, { ".", name }
attribute_args = "=" literal
               | literal, { ",", literal }
```

Examples: `#[inline]`, `#[inline(always)]`, `#[weight(2000)]`,
`#[repr(align(16))]`, `#[cold]`, `#[meta(key = "value")]`.

## 1.1 Attachment points

Attributes MAY prefix the following constructs. An attribute on any other
construct is a syntax error.

- Declarations: `data`, `record`, `signature`, `instance`, `foreign`
  import and export, `upgrade`, `rewrite`, and top-level function
  equations.
- Data constructor declarations and record fields.
- Function clauses (between the function name and its first pattern, or
  before an equation's patterns), `case` alternatives, and guards (after
  `|`, before the guard expression).
- Expressions in branch positions: the branches of `if`, and the
  alternatives of `case`.
- Types in signatures where a representation attribute applies (section 4).

## 1.2 Ownership and validation

Attributes fall into two ownership classes:

- **Compiler attributes** are the ones defined in this chapter (and future
  compiler-defined families). The compiler MUST validate their arguments
  and their placement, and MUST reject an unknown attribute name that is
  not covered by the next point.
- **Macro attributes** are introduced or consumed by hygienic attribute
  macros (see [Modules and compilation specification](modules-and-compilation-specification.md)
  section 8). An attribute name a macro declares is valid wherever the
  macro accepts it; the macro is responsible for its argument grammar. A
  reserved `meta` namespace (`#[meta(...)]`) passes through to tooling
  without semantic effect and is always accepted.

Attributes are not ordinary names: they do not participate in name
resolution, cannot be shadowed, and a macro's attribute names carry binding
identity through expansion like any other identifier (hygiene). Attributes
introduced by macro expansion MUST record expansion provenance in source
maps, as required by the modules specification's hygiene rule.

## 1.3 Semantics classes and identity

Every compiler attribute belongs to exactly one of two semantics classes,
and the class determines the identity rules of
[Modules and compilation specification](modules-and-compilation-specification.md)
section 4 and `compiler/CONFORMANCE.md` decision D9 (contract-only
interface identity):

- **Semantic attributes** change the meaning of the program's
  representation or behavior at the ABI boundary: the representation and
  layout attributes of section 4. They MUST appear in the compiled
  interface's contract projection: changing one changes the module's
  content identity, and dependents MUST be rechecked.
- **Hint attributes** may only affect performance, never observable
  behavior: the weight attributes of section 3 and the inline attributes of
  section 5. They MUST NOT appear in the contract projection, MUST NOT
  affect type checking outcomes, and any optimizer that transforms a
  program MUST preserve or drop them without changing the program's
  meaning. A program with incorrect hints produces the same values, only
  possibly slower.

Two attributes on the same construct with the same name are an error
unless a macro declares a multi-attribute form.

# 2 Branch weights: the primitive

The primitive control-flow annotation is a manual weight on one alternative
of a decision point. The construct is `#[weight(n)]` where `n` is a
non-negative integer literal (representable in 32 bits). Weight attributes
are valid on `case` alternatives, function clauses, guards, and the two
branches of `if`.

## 2.1 Weight algebra

The following rules are normative and follow the reference weight
mathematics.

- **Relativity.** Weights are relative within a single decision point
  (one `case`, one guard group, one `if`). Only ratios matter:
  `#[weight(2000)]` and `#[weight(1)]` is the same assignment as
  `#[weight(4000)]` and `#[weight(2)]`. A weight has no meaning in
  relation to the weights of a different decision point.
- **Zero.** `#[weight(0)]` means negligible probability but not
  impossibility. It is the signal for cold-code placement (section 3.3);
  it does NOT make the branch dead code, and the compiler MUST NOT
  eliminate it.
- **All-or-nothing.** If any alternative of a decision point carries a
  weight, every alternative MUST carry one (directly or through a wrapper,
  section 3). Partial annotation is ambiguous under relativity: when it is
  detected, the compiler MUST drop ALL weights of that decision point and
  report `incomplete-branch-weights`. This is the reference design's
  resolution of weight inference: inference never mixes with explicit
  weights (see Open decisions for the warning-level choice).
- **Storage.** Weights are `Word32` values attached to each alternative.
  In the compiler's core representation the weight is a property of the
  alternative itself (a `Maybe u32` beside it), never a parallel list on
  the decision node, so transformations cannot separate them.

## 2.2 Pattern decomposition and marginalization

When a weighted multi-clause function compiles to nested `case`
expressions (one per pattern column), each alternative's weight is the sum
of the weights of the clauses whose right-hand side is reachable through
that alternative. When every clause uses explicit constructor patterns,
this is exactly marginalization: treating clause weights as unnormalized
joint probabilities over the pattern columns, the outer alternative for
constructor `c_i` receives the marginal

$$w(c_i) = \sum_{j} w(c_i, p_j)$$

where $p_j$ ranges over the inner patterns paired with $c_i$, and each
inner alternative keeps its clause weight (the conditional). The invariant
is that the product of the weights along any root-to-clause path, with the
inner weights read as conditionals, reproduces the clause's flat weight.

Worked example (reference example from the proposal):

```
f #[weight(100)]  Nothing True  = a
f #[weight(2000)] Nothing False = b
f #[weight(50)]   (Just _) True  = c
f #[weight(50)]   (Just _) False = d
```

Outer case: `Nothing` = 100 + 2000 = 2100, `Just` = 50 + 50 = 100. Inner
case under `Nothing`: `True` = 100, `False` = 2000. The rule generalizes
to any number of columns by repeated marginalization.

## 2.3 Wildcards, or-patterns, and guard fallthrough

- A wildcard in a single-column match is one alternative (`DEFAULT`-like)
  and carries its clause weight directly; there is no distribution across
  constructors.
- In multi-column matches a wildcard (or or-pattern, or guard fallthrough)
  clause is reachable from multiple constructor groups. Each decision
  point keeps locally coherent ratios (the marginalization rule of 2.2
  applied to reachable right-hand sides), but the clause's weight is then
  counted at more than one path, so clause-level ratios are not preserved
  globally. This is inherent: two clause weights constrain three or more
  execution paths. Downstream consumers (layout, dispatch, allocation)
  require only per-decision-point coherence.
- When the compiler detects this weight inflation (a clause's effective
  total exceeds its stated weight), it MUST report
  `incoherent-weights`, informationally: the weights remain usable.
- A weight on an unreachable clause (covered by earlier patterns) is
  ignored; the ordinary redundancy warning applies.

## 2.4 Transformation rules

Optimizations that restructure decision points MUST reconcile weight
spaces as follows. These rules are the correctness core of the feature.

- **Known scrutinee.** When only one alternative survives, the decision
  point is eliminated and its weights are discarded.
- **Decision-of-decision.** When a decision on a value produced by
  another decision is pushed inward, the outer alternatives keep their
  weights and each duplicated inner decision keeps the original inner
  weights. This is correct under the independence assumption that the
  inner outcome does not depend on which outer alternative was taken;
  when later simplification collapses an inner decision, its weights are
  discarded.
- **Dead-alternative elimination (flow sensitivity).** When an
  alternative becomes impossible inside a branch that already decided the
  scrutinee, dropping it is conditioning on its negation. The surviving
  weights MUST NOT be rescaled: relative weights already encode the
  correct conditional ratios. (Example: alternatives `10 : 20 : 10` with
  the first impossible leave `20 : 10`, which is exactly
  $P(B \mid x \neq A) = 2/3$.)
- **Decision merging.** When two decisions on the same scrutinee merge
  into one, the two relative weight spaces are reconciled: for each outer
  alternative of weight $w_i$, the merged weight is $w_i \cdot S$ where
  $S$ is the sum of the inner weights; for each inner alternative of
  weight $u_j$ under the outer default $w_d$, the merged weight is
  $w_d \cdot u_j$. The merged set is then divided by its greatest common
  divisor. Worked example: outer `10 : 290` (default 290) with inner
  `200 : 100` merges to `10*300 : 290*200 : 290*100` = `3000 : 58000 :
  29000`, reduced to `3 : 58 : 29`; $P(B \mid x \neq A) = 58/87 = 2/3$ is
  preserved exactly. If either decision is only partially annotated (its
  weights were dropped by the all-or-nothing rule), the merge proceeds
  with no weights at all.
- **Rewrite rules.** When a rewrite (see [Modules and compilation
  specification](modules-and-compilation-specification.md) section 5)
  replaces an expression, the target's weights are replaced entirely by
  the template's weights; there is no merging between them. A template
  without weights produces unweighted alternatives.
- **Inlining.** Weights are properties of alternatives, not of source
  functions: inlining preserves them (see section 5).

New decision points created by transformations (worker/wrapper, case
splitting, and similar) carry no weights.

# 3 Branch weight wrappers

`cold`, `likely`, and `unlikely` are not separate compiler mechanisms.
They are wrappers that expand to manual weights, exactly as the reference
design expresses them as ordinary functions over weights:

| Wrapper | Expands to | Reading |
| --- | --- | --- |
| `#[likely]` | `#[weight(2000)]` | this alternative dominates |
| `#[unlikely]` | `#[weight(1)]` | this alternative is rare |
| `#[cold]` | `#[weight(0)]` | negligible; cold-section candidate |

The numbers are the reference design's: 2000 and 1 are the conventional
likely pair (equivalently any ratio near 2000:1), and 0 is the cold
marker. Because the wrappers are weights, the all-or-nothing rule applies
to them uniformly: annotating one `if` branch `#[likely]` requires the
other branch to carry a weight or wrapper too (typically
`#[unlikely]`).

A `Bool` helper in the style of C/C++ `likely()` is likewise just a
function over weighted `case`; it is not a builtin:

```
likely :: Bool -> Bool
likely c = case c of
  #[weight(2000)] True  -> True
  #[weight(1)]    False -> False
```

## 3.1 Consumption

Weights flow to every consumer that can use frequency information: code
layout (hot paths as fall-through), switch dispatch (hoisting the
dominant alternative, weighted pivots), register allocation (frequency-
based spill choice), and LLVM `branch_weights` metadata on conditional
branches (`!prof !{!"branch_weights", i32 w_true, i32 w_false}`). Exact
lowering is a backend matter; the semantic contract is only that the
ratios of section 2 are what the backend receives.

## 3.2 Inference

The compiler MAY infer weights for a decision point that carries no
explicit weights, using structure-based heuristics (a bottoming
alternative is rare, a recursive call is frequent). Inference MUST NEVER
combine with explicit weights: any explicit weight on a decision point
governs all of its alternatives (all-or-nothing), and a decision point
whose weights were dropped proceeds as if unannotated. Inference MUST NOT
change program meaning (section 1.3).

## 3.3 Hot and cold splitting

An alternative whose weight is 0, and any alternative whose incoming
frequency falls below the backend's cold threshold relative to its
function's total, MAY be emitted into a cold section (`__text_cold` on
Mach-O, `.text.unlikely` on ELF) with the corresponding layout
reordering. The threshold is a target profile parameter (see [Platform
proofs and SIMD specification](platform-proofs-and-simd-specification.md)
section 1).

# 4 Representation, layout, and alignment

Representation attributes are semantic attributes (section 1.3): they
change the ABI of a type and belong in the interface contract projection.

## 4.1 Attributes

- `#[repr(C)]` on a `data` or `record` declaration: the type has the
  platform C layout algorithm applied to its fields in source order
  (natural alignment, tail padding to the type's alignment). This is the
  only representation that MAY appear at a `foreign import`/`foreign
  export` boundary (see [Foreign function interface
  specification](foreign-function-interface-specification.md) section 2).
- `#[repr(transparent)]` on a single-field `data` or `record`: the type
  has exactly its field's representation, size, and alignment (a
  zero-cost wrapper, valid at the FFI boundary).
- `#[repr(align(N))]`: the type's alignment is at least `N`. `N` MUST be
  a power of two. Raising alignment MUST NOT change field offsets.
- `#[repr(packed)]` or `#[repr(packed(N))]`: padding is removed; fields
  sit at offsets rounded down to `min(N, field alignment)` with `N`
  defaulting to 1.
- `#[repr(u8|u16|u32|u64)]` on an `enum`-like `data`: the constructor
  discriminant has the given fixed-width representation.

Field order is source order (the layout table of [Primitive types
specification](primitive-types-specification.md) section 9 is the ground
truth for the field types' sizes and alignments). The compiler computes
offsets, size, and alignment by the C algorithm unless `packed` applies;
computing them any other way is a contract violation.

## 4.2 Safety rules

- A field whose offset is not a multiple of its natural alignment (as
  with `packed`) cannot be borrowed: `Ref r a` and `Mut r a` introduction
  on such a field MUST be rejected, since a reference asserts alignment.
  Raw access through `Bytes` or an FFI-provided pointer remains possible
  with the ordinary raw-memory obligations (platform specification
  section 2).
- `#[repr(align(N))]` with `N` below the type's natural alignment is an
  error; the attribute only raises alignment.
- Taking apart a `#[repr(transparent)]` wrapper (single-field) is layout-
  preserving in both directions.

## 4.3 Computed layout facts

A type's size, alignment, and field offsets are target-profile-indexed
facts. The compiler MUST record them in the compiled interface's ABI
section (they are part of the contract projection per section 1.3), and
dependent specifications MAY compute with them once the dependent core
lands (the `Usize` width fact of primitive types section 2 is the same
kind of target-indexed fact). Changing a representation attribute of a
persisted type is a state-representation change: [Runtime evolution
specification](runtime-evolution-specification.md) migration checking
applies.

# 5 Inline hints

`#[inline]`, `#[inline(always)]`, `#[inline(never)]`, and `#[noinline]`
are hint attributes on function declarations (equations):

- `#[inline]`: inlining is desirable at the optimizer's discretion.
- `#[inline(always)]`: inlining is required unless impossible (an
  obligation on the optimizer, not on semantics).
- `#[inline(never)]`: the function must not be inlined.
- `#[noinline]`: an accepted spelling of `#[inline(never)]`, matching
  LLVM and C/C++ attribute names. The two spellings carry the same hint
  and the same lowering; using both on one function is a duplicate
  attribute (section 1.3).

Inline hints MUST NOT change program meaning (section 1.3). They are not
part of the contract projection. When a function with inline hints and
branch weights is inlined, the weights travel with the inlined
alternatives (section 2.4), so a hint's effect on layout composes with
the weights already attached to the code. The optimizer's preservation
discipline (modules specification section 5, and the optimizer slice in
`compiler/CONFORMANCE.md`) treats all hints as erasable annotations.

A `#[cold]` MAY also appear on a function declaration, meaning the
function is unlikely to be called (its entry is a cold-section
candidate); this is the wrapper of section 3 applied to the function
entry rather than to an alternative.

# 6 Macro interaction

Attribute macros (modules specification section 8) MAY introduce,
consume, rewrite, or validate attributes. The rules that keep this sound:

- A macro that emits a compiler attribute MUST emit only valid forms; the
  compiler validates the expanded result under ordinary rules (generated
  declarations are checked under ordinary rules, per the modules
  specification).
- Attribute provenance survives expansion in source maps (section 1.2).
- A macro's own attribute namespace is hygienic: two macros' `#[...]`
  names cannot collide silently.

# 7 Worked example

```
module Fast where

  -- Layout: this type is the C ABI of a foreign record.
  #[repr(C)]
  record Header where
    #[repr(align(8))] magic :: U64
    len :: U32

  -- Branch weights: hot parse loop, cold error arm.
  classify :: U8 -> Nat
  classify b =
    case b of
      #[weight(2000)] 0 -> 0
      #[weight(1)]    _ -> 1

  -- The wrappers, mixed with a manual weight (complete annotation).
  fastPath :: Bool -> Bool -> Nat
  fastPath a b =
    if #[likely] a
    then if #[weight(100)] b then 1 else 2
    else 0

  -- Inline hint with weights preserved through inlining.
  #[inline]
  saturating :: Nat -> Nat
  saturating n =
    case n of
      #[cold] 0 -> 0
      #[weight(2000)] _ -> n
```

# 8 Relationship to the reference design

The weight algebra (relativity, zero, all-or-nothing, marginalization,
the transformation rules, and the constants 2000/1/0) follows GHC
proposal 744 (Branch Prediction Hints) and its review discussion, which
this project treats as the reference. Deliberate differences:

- The surface spelling is the `#[...]` attribute system of this chapter,
  not a `{-# WEIGHT n #-}` pragma; the semantics above are unchanged.
- `cold`, `likely`, and `unlikely` are wrappers over manual weights
  (section 3), matching the reference design's conclusion that they are
  expressible as weight assignments rather than separate mechanisms.
- Inference (section 3.2) is optional here rather than part of the core
  feature; the all-or-nothing rule is the same.

# 9 Open decisions

- Warning vs error for partial annotation (`incomplete-branch-weights`)
  and for weight inflation (`incoherent-weights`): the reference design
  warns and drops or keeps; this draft currently inherits that. A
  stricter profile that errors is a possible target-profile choice.
- Whether `#[repr(align(N))]` on fields (rather than whole types) is
  admitted; this draft allows it on types and on fields of `repr(C)`
  records with the constraint of section 4.2.
- Whether explicit numeric weights and wrappers may be mixed on one
  decision point (this draft allows it: wrappers ARE weights) and whether
  a warning should normalize surprising ratios.
- The cold-splitting threshold's exact default and its gating behind
  optimization levels (platform target profile parameter).
- Whether weight-0 alternatives may be excluded from coverage checking's
  exhaustiveness reasoning (this draft says no: coverage is semantic and
  weights are hints).
- Whether inline hints should have any effect on interface identity for
  cross-module inlining decisions (this draft says no: identity is
  contract-only per D9).

# 10 Implementation status

Not implemented. The compiler parses no `#[...]` attributes yet; the
forms above are rejected at the lexer. The slice order in
`compiler/CONFORMANCE.md` places the attribute system, weights, and
inline hints ahead of the optimizer and layout work that consume them.
The design context for the AST representation of attributes is
`design/attribute-annotations.md`.
