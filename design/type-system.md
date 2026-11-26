# Type system and dependent core

This document summarizes the architecture of Cinch's type system: the dependently typed core kernel, mutual inductive data groups, refinement types and decidable solvers, IEEE float refinements, Rank-N polymorphism, codata/guardedness, and proof automation.

Normative obligations live in [`specifications/core-language-specification.md`](../specifications/core-language-specification.md) and [`specifications/primitive-types-specification.md`](../specifications/primitive-types-specification.md). User-facing documentation lives in [`reference/04-types-and-kinds.md`](../reference/04-types-and-kinds.md), [`reference/06-refinements.md`](../reference/06-refinements.md), and [`reference/08-dependent-types-and-proofs.md`](../reference/08-dependent-types-and-proofs.md).

---

## 1. Dependent core kernel

Cinch elaborates surface programs into a small dependently typed core using bidirectional type checking layered over Hindley-Milner inference for the simply typed fragment, effect rows, and multiplicities.

```text
surface -> bidirectional elaboration -> typed Core -> checked erasure -> Core IR -> LLVM
                 ^                         |
                 | conversion              +-- proof, totality, usage & positivity checks
                 +-- unification (metas)
```

### Universes and terms
- `Type 0 : Type 1 : ...` is a stratified, predicative universe hierarchy (`Type : Type` is rejected).
- `Prop` is the universe of erased propositions (`Eq`, inequalities, structural predicates).
- Core terms comprise variables, `Pi (x : A) -> B`, `Sigma (x : A). B`, lambdas, applications, `let`, constructors, dependent pattern matching, `Type i`, `Prop`, erased proof terms, and effect-row / multiplicity annotations on arrows.
- `Nat` and `Int` are unbounded mathematical types in `Type 0` backed by tagged immediates and bignums in the runtime; machine scalars (`I8`..`I128`, `U8`..`U128`, `F32`, `F64`, `Bool`, `Char`, `Usize`, `Isize`) are fixed-layout primitives in `Type 0`.

### Conversion and erasure
- **Definitional equality** covers beta, delta (transparent definitions only), iota (constructor case reduction), and proof irrelevance for `Prop`. Conversion never unfolds non-decreasing recursive definitions and never invokes external solvers.
- **Machine arithmetic is excluded from conversion:** wrapping/checked/saturating operations are proved, not reduced definitionally.
- **Erasure** removes `Prop` terms, `%0` binders, and `erased` record fields before code generation. Erasure never drops computational witnesses (such as `Sigma` payloads or indices inspected at runtime) or observable effects. `Prop` elimination into `Type` is restricted to singleton/decidable paths.

### Indexed families and totality
- Inductive families (`Vec n a`, `Fin n`, `Dec p`, user-defined families) carry explicit constructor indices. Pattern coverage solves or refutes constructor index equations so impossible branches are eliminated; direct constructor matching on rigid indices is conservatively rejected (dispatch through `decide` or computed indices).
- Logical definitions (`spec`, `proof`) must be pure and structurally terminating; executable divergence is tracked with the `Diverge` effect and can never inhabit `Prop`.

---

## 2. Mutual data groups and SCC positivity

Inductive `data` and `record` declarations in a module may refer to each other in any textual order. Positivity is checked over strongly connected components (SCCs) of the module's data/record reference graph:

1. A member `S` of SCC `G` may occur in a constructor or record field only as `S a1 ... an` with no `ai` mentioning any member of `G` (regular recursion).
2. Arrows, `Pi`/`Sigma`, tuples, and `Thunk`/`Lazy`/`Later` payloads are traversed; function domains flip polarity.
3. Any negative occurrence of a group member is rejected.
4. Nesting a group member inside another type constructor (such as `List T` or a refinement carrier) is conservatively rejected until positivity-preserving nesting is designed; recursive collections inside a mutual group use explicit spine constructors.
5. `Ref r a` and `Mut r a` borrows are indirections rather than structural containment and are excluded from the positivity check.

Records participate in SCCs alongside `data` declarations so a negative occurrence cannot hide behind a cycle `A -> Record -> A`. Because SCC positivity is established when a module is checked, compiled interfaces require no extra group metadata.

---

## 3. Refinement types and decidable solvers

A type that differs from its carrier solely by a truth about its inhabitants is a **refinement**, not a wrapper. Semantic distinctions (`Sum`, `Product`, `Dual`) remain newtypes; invariant-constrained values (`NonEmpty`, `Sorted`, `NonZero`, `Ascii`, `InRange lo hi`) are zero-cost refinements:

```cinch
type NonEmpty a = refine (xs :: List a) with NotNull xs
type InRange (lo :: Int) (hi :: Int) a = refine (x :: a) with lo <= x && x <= hi
```

### Representation, lattice, and frictionless flow
- **Zero runtime cost:** a refined value has the exact representation of its carrier; the predicate witness is `Prop`-erased.
- **Predicate-level instances:** container class methods transfer to refined types when the method preserves the predicate (checked by the law layer). Specialized refined instances (e.g. `Semigroup` via `merge` on sorted lists) resolve by most-specific head along the implication lattice; incomparable heads are a coherence error.
- **Weakening is free; narrowing is checked:** weakening along the decidable implication lattice (`p && q => p`, interval containment, structural rows like `length xs > 0 => NotNull xs`) is implicit and zero-cost (`forget` drops named top-level conjuncts explicitly). Strengthening (`narrow`) always requires a `Dec` witness.
- **Comparison-only refinements** (`InRange`, inequalities) flow back to their carrier without `unrefine`; structural refinements (`NonEmpty`, `Sorted`) project explicitly via `unrefine`.
- **Refined results:** function signatures may refine results in terms of earlier parameters (e.g. `nat-rem :: Nat -> (d :: NonZero Nat) -> refined (r < d) Nat`, `bytes-new :: (n :: Nat) -> refined (bytes-len out == n) Bytes`), substituting admissible caller fact terms at direct call sites.

### Branch facts and linear arithmetic discharge
Every branch of `if`, `case`, and pattern guards binds hidden `Prop` facts for admissible pure scrutinees (locals, numerals, exact `Nat`/`Int` arithmetic, `bytes-len`, `length`, `char-code`, pure global calls without mutable buffers or borrows). Facts are flow-sensitively invalidated when mutable state in their footprint changes.

Obligation discharge runs in three deterministic, bounded tiers (no undecidable SMT inference):

1. **Table-driven lattice implication:** syntactic identity, conjunct projection, interval containment/disjointness over literal bounds, `x /= 0 <=> NonZero x`, and structural vocabulary rules.
2. **Difference logic (Bellman-Ford):** solves systems of constraints `x - y <= c` over `Nat`, `Int`, machine integers (exact within range bounds, without linearizing wrapping machine arithmetic), and `Char`, with bounded case splits on `/=` facts. Produces a negative-cycle certificate (`DiffCert`) verified by an independent checker.
3. **Multi-term linear arithmetic (Fourier-Motzkin):** handles three-or-more-term obligations (such as `di + n <= bytes-len dst`) under a strict 256-row elimination cap, producing a checked Farkas certificate (`LinCert::Farkas`). Because it refutes the rational relaxation, integer-only contradictions are conservatively refused rather than assumed.

---

## 4. Float refinements

`F32` and `F64` remain unboxed IEEE 754 binary floats with exact semantics and no default `Eq`/`Ord` instances (because `NaN /= NaN` breaks equivalence laws). Float refinements express guarantees without changing the representation:

- **Predicates (proposed/planned tiers):** `NotNaN x`, `Finite x`, `NonZero x`, `Positive x`, `NonNegative x`, `InRange lo hi x` (all excluding `NaN`, with IEEE-accurate negation where `not (x < y)` only implies `x >= y` under `NotNaN`).
- **Operation guarantees:** `Finite` is not closed under `+`/`-`/`*`/`/` due to overflow, so arithmetic on `Finite` operands produces `NotNaN` unless outward-rounded interval arithmetic keeps the result inside the finite range. Total float-to-integer conversion is admitted when `InRange` lies within the target integer bounds.
- **Lawful vs unsafe instances:** `NotNaN F64` admits lawful `Eq`/`Ord`/`Hash` under `totalOrder` (which distinguishes `-0` from `+0` and is therefore a congruence for rewrites). Raw IEEE `Eq`/`Ord` instances belong behind explicit-only `unsafe instance` declarations (never `implicit`) recorded in the build manifest.
- **Sound fast-math:** `nnan`/`ninf` LLVM flags may only be emitted when refinements prove operands and results cannot be `NaN` or infinite; `reassoc` and `contract` remain forbidden without an explicit contract.

---

## 5. Rank-N polymorphism

Cinch supports predicative Rank-N polymorphism (implemented in both `seed/` and `compiler/`): `forall` quantifiers may appear anywhere in a type (function arguments, constructor fields, records), binding types, higher-kinded constructors, effect rows, and regions. Nested `forall` blocks currently reject `using` constraints and value/proposition binders (those stay prenex).

### Checking rules
Checking follows GHC's simplified subsumption restricted to predicativity:
- **Check against `forall as. T`:** skolemize `as`, check the body under a pure effect frame (since polymorphic values may be instantiated per representation class), verify no skolem escapes into captured bindings or outer scopes, and emit `PolyIntro`.
- **Infer a polytype:** instantiate outermost quantifiers at variables, application heads, and projections (`PolyElim`).
- **Predicativity and invariance:** metavariables cannot be solved to polytypes; polytypes unify invariantly by skolemizing both sides together.

### Lowering
- In `seed/`, a `k`-binder polytype belongs to anonymous single-method family `#poly.<k>`, represented as a dictionary block with one slot per reached representation-class vector (reusing the existing specialization collector; no runtime dispatch).
- In `compiler/`, generalisations lower to class-indexed bundles (currently up to `k <= 2` binders within the 16-word descriptor cap).

---

## 6. Codata and guardedness

Cinch separates logical productivity guards from runtime laziness:

| Mechanism | Meaning | Reclamation |
| --- | --- | --- |
| `Later a` | Guarded observation available at the next tick | Logical modality |
| `Thunk {e} a` | Deferred computation (`delay` / `force`) | Consumed on force (`%1`/`%aff`) |
| `Lazy {e} r a` | Region-scoped memo cell (`~ x = e`) | Reclaimed at region exit |

`codata` declares observation records (`head`, `tail :: Later (Stream a)`) lowered as records of delayed observation closures. `corec` enforces guarded corecursion syntactically: every recursive call must appear under a `delay` placed in a `Later` observation position. Stream bisimulation is a propositional relation (`bisim`), never part of definitional equality.

---

## 7. Proof automation

Proof automation follows one invariant: **search is untrusted; the kernel checks the result.**

Automation produces one of three checkable artifacts:
1. **Proof terms** (`simp`-style rewrite chains, congruence-closure explanations, structural induction).
2. **Reflection** (a Cinch decision procedure proven sound once and evaluated in the kernel).
3. **Certificates** (negative-cycle `DiffCert`, Farkas `LinCert`, LRAT SAT proofs, SMT proof certificates) validated by a checked verifier.

Tactics will live in Cinch's own typed staging/macro layer (`stage`/`macro`) rather than a separate tactic language. Discharged proof artifacts are keyed by content hash in the package store so proofs are checked once and reused across builds.

---

## 8. Open decisions

- **Positivity-preserving nesting:** admitting `List T` and similar strictly positive containers inside mutual inductive groups.
- **Core equality & universes:** definitional vs propositional proof irrelevance, record eta rules, and full vs `Type 0`-only universe cumulativity.
- **Coinduction discipline:** guarded clocks vs sized types beyond the current syntactic `Later` check.
- **Float refinements:** whether lawful `NotNaN` `totalOrder` instances are `implicit` or explicit-only, and final alias/module naming.
- **Nested `forall` constraints:** passing dictionary arguments inside Rank-N polytype bundles.
- **Tactic surface & kernel reflection:** moving `DiffCert`/`LinCert` checking from Rust into reflected kernel checks and settling the `by <tactic>` / goal-context API.
