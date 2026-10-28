# Core language specification

Source: <https://chatgpt.com/space/page_1e97031528908191b713513863e30e6b>  
Exported: 2026-10-22T18:59:36Z

---

[Documentation overview](../overview.md)

Draft 0.2. This chapter defines the intended static and dynamic obligations of the language. MUST denotes a requirement of this draft. Proposed choices are identified explicitly; unresolved foundations are listed in Open decisions. The surface notation is provisional and no compiler implementation is claimed.

# 1 Evaluation and syntax

The language MUST use strict call-by-value evaluation for executable code. The proposed order is left-to-right for function arguments, record fields, and sequence elements. Pattern matching evaluates its scrutinee once. Short-circuit Boolean operators evaluate only the necessary operand. There is no implicit general laziness.

Blocks are expressions. Layout groups declarations and `do` statements; `let` introduces an immutable binding. `name patterns = body` defines executable functions; `spec` marks total ghost definitions; `proof` constructs erased evidence. An explicit arrow without an effect row is total and pure; unannotated local definitions infer their effects. The compiler MUST reject an unsupported totality claim or require an explicit `Diverge` effect.

```haskell
compose :: forall a b c. (b -> c) -> (a -> b) -> a -> c
compose f g x = f (g x)
```

This example assumes reusable callback values. Closure usage is inferred from captures; a closure that consumes a captured resource cannot automatically inhabit a reusable function type.

The following notation is used consistently throughout the draft:

| Notation | Meaning |
| --- | --- |
| `f :: forall a b. a -> b` and `f x = ...` | Generic function |
| `a -> b` | Total pure reusable function, subject to usage checking |
| `a ->{|e} b` | Function with effect row `e` |
| `Ref r a`, `Mut r a` | Shared or exclusive borrow with lifetime `r` |
| `a %1 -> b` | Consuming parameter |
| `erased p :: P` | Evidence unavailable for computational branching |
| `Eq x y` | Propositional equality |
| `Sigma (n :: Nat). Vec n a` | Dependent pair with computational first component |
| `using (dictionary :: Signature) => ...` | Explicit dictionary parameter eligible for implicit resolution |
| `[| expression |]`, `$(code)` | Typed code construction and insertion |
| `Later a`, `delay (\() -> expression)` | Guarded delayed observation; not an unrestricted thunk |

Space-separated type applications may contain types or values; compound indices use parentheses. `Nat` and `Int` are mathematical natural and signed integers. Machine integer types use explicit checked, wrapping, or saturating operations. Generic `+` in mathematical examples has mathematical semantics.

# 2 Dependent types

The core MUST support dependent function types, dependent pairs, indexed inductive families, dependent pattern matching, and a stratified universe hierarchy. `Type :: Type` MUST NOT be admitted. Universe inference and polymorphism are required; the exact cumulativity and level syntax remain open.

Types may depend on ordinary values, not only compile-time literals. Evaluating a program can produce an existential package whose type refers to the produced value. The type checker need not evaluate an unknown runtime value; it reasons symbolically in a context containing that value.

Inductive definitions MUST satisfy a sound positivity discipline. Recursive logical definitions MUST satisfy a sound termination discipline. Pattern coverage MUST account for indices and impossible constructors. General recursion is available only in a computation form whose divergence is explicit and which cannot fabricate logical evidence.

```haskell
data Vec (n :: Nat) (a :: Type) where
  Nil :: Vec 0 a
  Cons :: forall k. a -> Vec k a -> Vec (k + 1) a
```

This notation suppresses constructor-local implicit binders. The elaborated declaration binds each index explicitly.

Type checking MUST distinguish definitional equality from a propositional equality proved by the user. The proposed definitional equality uses a controlled, terminating reduction discipline. Arbitrary SMT equivalences and arbitrary optimizer rules MUST NOT silently extend conversion.

An explicit transport operation converts a value across a proved equality of indices. Automation may insert such transports, but the resulting core must be checked. Function extensionality, proof irrelevance, cubical equality, and the exact relationship between stream bisimilarity and identity remain foundation decisions; the draft does not assume all of them.

# 3 Logical evidence and erasure

The logical fragment MUST remain consistent in the presence of effectful programs, divergence, staged computation, and hot replacement. A looping function MUST NOT inhabit an arbitrary proposition. Failed runtime checks MUST NOT produce evidence. Untrusted generated terms MUST pass the same checking rules as handwritten terms.

This draft uses `Prop` as a proposed notation for erased propositions. Its elimination rules MUST prevent computational decisions from depending on erased proof identity. Constructive witnesses required at runtime belong in `Type` or in another explicitly retained computational representation.

For example, `Sigma (n :: Nat). Vec n a` may retain `n`; a proof that `n <= limit` can be erased. A proof that a match exists does not itself provide executable search code unless the selected logical and representation discipline supplies a retained witness or certified terminating search.

Erasure MUST preserve the semantics of executable code. The compiler MUST retain indices, dictionaries, and witnesses when execution requires them, or derive their runtime values from retained data. It MUST NOT erase an argument merely because the surface syntax resembles a proof.

Ghost models of owned values MUST NOT duplicate ownership. A specification may mention an immutable mathematical view of a resource without creating a second executable resource. The formal connection between such views and borrowing is an open core-design task.

# 4 Incremental verification

The language MUST support gradual strengthening of specifications. A module with an ordinary type promises only that type and its baseline effects and resource guarantees. Adding a stronger contract creates new proof obligations; it does not retroactively establish properties absent from the earlier contract.

An obligation has one of these statuses:

| Status | Consequence |
| --- | --- |
| Checked static evidence | May be used wherever its assumptions hold |
| Explicit runtime decision | Evidence exists only on the successful branch for the checked values |
| Declared trusted assumption | Recorded transitively; allowed only by the selected policy |
| Unresolved | Editor exploration may continue; an artifact requiring it cannot be accepted |

The compiler MUST distinguish disproved obligations, unsupported reasoning, and proof-search timeout. None of the latter outcomes is a proof. Testing may find counterexamples but MUST NOT discharge a universal obligation by itself.

`decide(P)` is available only when an executable decision procedure for `P` is supplied. It returns `Yes(proof)` or `No(refutation)` with evidence scoped to the examined state. Runtime checking MUST be explicit in the source or public effect and error contract; solver timeouts MUST NOT silently introduce new runtime failures.

An editor may represent typed holes and unfinished proofs. A deployment profile MUST reject unresolved required holes. The design may permit trusted FFI or primitive assumptions, but it MUST expose them in a transitive assumption manifest. A profile named certified, if introduced, must have a precisely defined trust boundary rather than imply that hardware and all external systems are verified.

# 5 Refinement and flow analysis

Branch conditions, matches, and early returns SHOULD construct usable evidence automatically. Refinements are a convenient surface over the dependent core, not a restriction on its expressiveness.

```haskell
firstCopy :: forall a region. using (copy :: Copy a) =>
              Ref region (Array a) -> Option a
firstCopy xs = if length xs == 0
  then None
  else Some (Array.getCopy xs 0)
```

On the successful branch the bounds obligation is established. The shared borrow prevents incompatible mutation of the array during the operation. Evidence about mutable state MUST be invalidated or transformed when that state changes. A numeric fact about a previous array length does not authorize indexing a shorter array.

Refinement construction at external boundaries SHOULD produce useful error paths and validated domain types. Opaque nominal types distinguish values with the same representation, such as `UserId` and `InvoiceId`. Structural records support reusable data manipulation, row polymorphism, and explicit optional fields. Missing fields and fields containing `None` MUST remain distinguishable where the data model observes the difference.

# 6 Effects and capabilities

Effects describe computational behavior; capabilities authorize access to resources. Both MUST be represented. Public signatures expose effects; local inference may infer them. Higher-order effects are row-polymorphic.

```haskell
map :: forall a b e. (a ->{|e} b) -> List a ->{|e} List b
read :: forall region. Ref region FileReadCap -> Path
     ->{IO} Result ReadError Bytes
```

The draft distinguishes I/O, state, asynchronous suspension, divergence, and explicit failure mechanisms. The final effect vocabulary and handler calculus remain open. Typed `Result` values are the default recoverable-error notation in examples.

Handling an effect does not automatically establish totality. A handler can introduce divergence or other effects, which MUST be reflected in its type. Multi-shot resumption MUST NOT duplicate linear captures; the initial design should prefer one-shot handlers or require evidence that the captured continuation is duplicable.

Panic, allocation failure, cancellation, and stack exhaustion need an explicit runtime policy. A no-panic contract cannot be inferred merely from the absence of an ordinary exception effect if the runtime exposes additional failure paths. Logical proofs may use an idealized unbounded-resource model only when that modeling assumption is stated.

# 7 Ownership and memory safety

Safe code MUST prevent use-after-free, double free, invalid mutable aliasing, and data races under the specified runtime memory model. The language MUST offer shared immutable borrows, exclusive mutable borrows, consuming ownership transfer, and safe synchronized sharing. Foreign and primitive implementations remain explicit trust boundaries unless verified.

The proposed foundation tracks erased, affine, linear, and unrestricted usage where appropriate. Exact usage algebra is unresolved. Affine values may be dropped through valid destruction. Linear obligations require exactly one valid consumption along every relevant path, including error and cancellation paths. Destructor effects and failure policies are part of the resource protocol.

Immutable values are not universally free to copy. Sharing, copying, and cloning MUST have defined semantics, and potentially expensive copies SHOULD be explicit. Closures capture values with their ownership and borrowing constraints. A callback cannot outlive a borrowed environment.

Ownership rules MUST interact soundly with dependencies: moving or mutating a value cannot leave live types or executable witnesses relying on invalid state. The implementation may use stable logical names, snapshots, borrowing restrictions, or a combination; this must be settled in the core calculus before claiming soundness.

Memory safety does not establish deadlock freedom, leak freedom, fairness, bounded memory use, or freedom from timing channels. Separate contracts may express these properties.

# 8 Induction and totality

Inductive proofs establish properties of finite constructions and reachable states. Total functions require structural recursion, a well-founded measure, or another checked termination argument. Automation may infer such evidence but MUST return a checkable result under the chosen kernel design.

Partial correctness states that a returning computation satisfies its postcondition. Total correctness additionally establishes termination under the relevant assumptions. Diverging services are not excluded from the language; they inhabit an explicit computation or process abstraction.

# 9 Coinduction and productivity

The language MUST support coinductive data and a sound guarded corecursion discipline. A productive stream provides every requested finite prefix in finite internal computation, under the declared assumptions. This differs from a process that may wait indefinitely for input or search indefinitely for its next output.

```haskell
codata Stream (a :: Type) where
  head :: a
  tail :: Later (Stream a)
```

`Later` is a guarded observation mechanism. Its force rules MUST prevent an unproductive recursive definition from masquerading as a total stream. This draft leaves the choice of guarded clocks, sized types, or another sound discipline open.

Stream bisimilarity is a relation whose evidence establishes equal current observations and recursively related future observations. It need not be definitionally identical to identity equality. Rewrites may use a named observational relation with congruence evidence rather than assume arbitrary bisimilar terms are interchangeable in every context.

Unrestricted filtering of an infinite productive stream MUST NOT promise a productive output stream. An infinite sequence of rejected values is possible. A stronger filter requires computational progress evidence sufficient to implement a total next-match search. A machine with terminating `Skip` steps can still skip forever.

# 10 Processes and temporal contracts

Process specifications may describe infinite traces, state transitions, and environmental assumptions. The language MUST distinguish safety, liveness, recurrence, and stability. Proposed notation includes `always(P)`, `eventually(P)`, and leads-to properties over correlated request identities.

An invariant proof can use induction over transitions and cover an infinite execution. Coinduction can establish behavioral equivalence. Liveness often requires well-founded progress arguments and fairness. No single proof technique subsumes all three uses.

Temporal contracts MUST state scheduler, dependency, crash, network, and fairness assumptions when relied upon. Liveness evidence MUST be connected to the implementation by a refinement argument. A finite model-checking run establishes only its modeled scope unless accompanied by a proof justifying generalization.

Safety refinement alone MUST NOT certify liveness preservation. Internal stuttering or an infinite sequence of upgrades may prevent progress while preserving all finite safety observations. Temporal obligations therefore extend to runtime scheduling and update policy.

# 11 Desired metatheory and trust

The core design requires proofs of type preservation, progress in the appropriate effectful sense, logical consistency or normalization for the logical fragment, sound usage checking, erasure correctness, and the selected productivity principle. These are implementation milestones, not established results of this draft.

The acceptance manifest MUST identify primitive axioms, external contracts, proof backends, and other trusted components that affect claimed guarantees. Proof automation should generate certificates checked by the kernel where feasible. A trusted solver mode may be offered, but MUST be distinguished from certificate checking.

The module, optimizer, staging, and upgrade obligations in the other specification chapters extend this core. They cannot bypass it.

# Foreign and concurrent computation

Foreign declarations do not inherit ordinary pure-total function defaults. They expose their effect, failure, blocking, divergence and ownership contracts explicitly. Concurrent execution belongs to the effectful program layer; scheduler progress is not used as an unchecked premise in the total logical fragment.

Task captures, borrows across suspension and thread transfer require ownership evidence. Erasure does not remove runtime witnesses needed at an ABI or task boundary. See [Foreign function interface specification](foreign-function-interface-specification.md) and [Concurrency specification](concurrency-specification.md).

# Current syntax and sequencing

The draft 0.2 surface uses layout, equations and unqualified `do`. See [Surface syntax and grammar](surface-syntax-and-grammar.md) and [Runtime capabilities, effects and sequencing](runtime-capabilities-effects-and-sequencing.md) for formal syntax, runtime arguments, effect-polymorphic calls and sequencing-module selection.

# Amendments

Dated notes recording landed decisions (`compiler/CONFORMANCE.md`,
D-numbers) that amend or interpret this chapter's normative text.
2026-10-28, P15 conformance sweep.

- **Fault path and evaluation order (D8), 2026-10-28.** A match that
  runs out at runtime takes the defined fault path (exit 70), never a
  zero word. Record construction evaluates fields in source order
  (this chapter's left-to-right rule) while constructor arguments stay
  in declaration order.
- **Dependent core landed (D27-D35), 2026-10-28.** Universes are
  stratified (`Type 0 : Type 1 : ...`, `Prop : Type 0`); `Type : Type`
  is never admitted. Definitional equality is beta at elaboration, iota
  on numerals, alpha for Pi/Sigma, and proof irrelevance (proofs erase
  to unit); user definitions never unfold in conversion (D28, general
  delta reduction deferred; named-helper reduction for proof goals is a
  separate checked mechanism, D268-D270). Un-annotated `forall` binders
  classify by use; index-position `+ - *` are mathematical with
  `toNat`/`length` as the builtin total vocabulary (D29). `Vec`/`Fin`/
  `Dec` are builtin families with the indexed-declaration mechanism
  general (D30). Erasure: `Prp`/`Sort` fields and parameters take no
  runtime slot, family indices are not stored in blocks, `Sigma`
  retains its witness (D31). Totality: structural termination, purity
  and mutual acyclicity hold for `spec`/`proof`; requiring `Diverge` on
  recursive executable definitions is deferred (D32, a named boundary).
  Coverage solves index equations: refuted arms reject as impossible,
  stuck rigid-index matches reject naming the equation (D33, D35), and
  type-level operators have reserved fixities (D34).
- **Ownership and usage rules (D15-D19), 2026-10-28.** Unannotated
  binders default `%many`, `Thunk`-typed binders `%aff` (D15). The
  sharing class is `Copy` (`Share` alias), structural and not
  user-extensible; `Bytes`, functions, thunks, cells, and `Mut` are not
  `Copy`; `copyOut` is the only implicit-copy site plus memo bare-use
  (D16). Disjoint projection places fuse into one consumption of the
  root (D17). Lambda/thunk bodies count captures as unbounded, memo
  bodies at most once (D18). Borrows are check-level; `demand` returns
  `Ref r a`; `Mut` uniqueness is lexical (D19).
- **Evidence invalidation (D192-D197), 2026-10-28.** This amends
  section 5 (and section 7's dependency clause): a Prop-typed evidence
  item whose statement reads a `Mut` borrow carries that region as its
  footprint, and any write to the region invalidates it. Write triggers
  are `refSet m v` (a new builtin lowering to `Core::RefSet`, opaque to
  the optimizer), calls passing a `Mut` argument, calls whose effect
  row names the region with a write-capable head, and opaque writes
  (FFI/foreign, eval, mutating `Bytes` primitives) which
  conservatively invalidate all `Mut`-sourced evidence. Stale uses are
  check errors naming both the derivation and the invalidating site;
  use sites record deferred uses checked after constraint solving.
  `Ref`-sourced evidence survives its region (read-only; P5 exclusivity
  forbids a coexisting `Mut` write), immutable evidence never
  invalidates, and `RawPtr` targets carry no evidence at all.
  Ownership moves and mutations therefore never leave live types or
  executable witnesses on invalid state. Evidence:
  `tests/evidence_invalidation.rs`.
- **Temporal contracts (D198-D205), 2026-10-28.** This amends section
  10: `contract` declarations are top-level, naming a definition of
  their own module, with formulas over the definition's pattern
  parameters. `always (P)` is an inductive invariant over the
  definition's per-step transitions (the D192 write log), with stated
  establishment (`assume = (P)`, checked equal); falsifying writes are
  rejected naming the step and undecidable steps are refused. `stable
  (P)` is the dual of evidence invalidation (no write trigger touches
  the footprint). `eventually (Q)` verifies only the finite structured-
  concurrency obligations (`joined`/`cleanup`) and refuses open-ended
  claims, with liveness contracts stating their scheduler/crash/
  fairness assumptions. `recur (P)` is bounded by the structural-depth
  proof and witnessed by a re-establishing step on every recursive
  cycle. Contract facts are `.cinchif` `contracts` rows hashed into
  `iface_hash`, stated assumptions join `assumptions.declared`, and the
  manifest `temporal` section records each verification result.
  Evidence: `tests/temporal.rs`.
- **Refinement types (D151, D248-D261, D271-D277, D282-D287),
  2026-10-28.** This amends section 5's refinement surface. Sub-kind
  types (`NonEmpty`, `Sorted`, `NonZero`, `Ascii`) are refinements:
  `type R p = refine (x :: T) with P` registers a predicate-indexed
  family whose runtime representation IS the container and whose
  witness is `Prop`-erased. Discharge is explicit in the `Dec`/`decide`
  world (a total constructor or a `Dec` argument that erases); forged
  and stale witnesses are refused (staleness via the D192 rule). The
  family carries a decidable implication preorder with implicit
  weakening along atom-set inclusion and table-driven narrowings
  (interval containment, measure vocabulary rows), checked strengthening
  via `narrow` (a `Dec` of the stronger predicate), explicit `forget`
  for top-level conjuncts, and `unrefine` as the forgetful arrow.
  Instances transfer by representation identity: the container's
  methods at the container's types, valid exactly when they preserve
  the refinement, with preservation/postcondition obligations checked
  by the law layer (`Preserves (` op `)` as the constraint spelling;
  `post p p = p` rows). Instance heads may carry refinements (or the
  inter-derivable Haskell-style `using` obligations); resolution is
  most specific along the implication lattice with coherence errors
  naming incomparable heads (D119 extended). No inference, no SMT;
  compound predicates admit `&&`/`||` over decidable propositions.
  Evidence: `tests/refinements.rs`, `tests/lattice.rs`.
- **Codata and productivity (D37-D41), 2026-10-28.** This amends
  section 9: a codata type is a record of delayed observations,
  constructed only by the observed form `C { obs = e }`, boxed like ADTs
  (one closure word per observation; `Prp`/`Sort` observations erase);
  pattern matching on codata is refused and codata values are never
  `Copy` (D37). Guardedness is decided on the elaborated core: a
  `corec` body must build its codata type at every step and every
  recursive call must sit under `delay` flowing to a `Later`-typed
  observation, violations naming the unguarded call (D38). Productivity
  is certified only by `corec`; a codata-producing definition with an
  unguarded call reflects the loss in its type (`Diverge` on the last
  arrow), so an unrestricted filter never presents as a productive
  stream (D39). `delay` building a `Later` must have an empty performed
  row (D40), and `\() -> e` accepts the unit pattern as the guard
  spelling (D41). Recorded design divergences: streams in the prelude
  are deferred and `bisim` is out of scope; D32's boundary is unchanged
  (a pure-typed diverging helper can fake productivity through a
  guard).
- **Cinch naming (D278-D281), 2026-10-28.** The language is Cinch:
  `.cinch` sources, `.cinchif` compiled interfaces, `cinch_*` runtime
  externs, `cinchruntime` staticlib. Artifact names in this chapter
  read with those names.