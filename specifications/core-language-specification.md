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