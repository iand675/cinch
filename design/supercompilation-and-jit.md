# Supercompilation and JIT systems

Design study for whole-program specialization (offline supercompilation)
and runtime specialization (JIT) over the SafeLang core. Status: design
study; the implementation items ride the existing phases (section 9),
tracked in `compiler/CONFORMANCE.md`. The specs already
leave hooks for this work: the modules specification requires compiled
interfaces to record "representation information intentionally exposed
to specialization", requires compiler reports to identify "where
specialization stopped, and which allocation or effect prevented a
requested transformation", and devotes section 13 to target-specific
specialization; the platform and runtime evolution specifications define
how specialized code is replaced while frames are live. This note says
what a specialization pipeline would look like given the machinery the
implementation now has (typed quotes, effect rows, multiplicities,
dependent types, codata, versioned dispatch). No em dashes.

## 1 One driving engine, two consumers

Supercompilation and JIT are the same transformation at two times:

- **Offline supercompiler** (build time): drives the whole program or a
  chosen module, residualizes specialized code into the artifact,
  records what it did in the compiler report and artifact manifest.
- **Online JIT** (runtime): drives a hot closure against observed
  arguments, residualizes a specialized variant, installs it through the
  runtime evolution machinery, deoptimizes when guards fail.

Both reduce to a **driver** (symbolic evaluation of core terms), a
**generalizer** (deciding when to residualize instead of unfolding), and
a **rebinder** (let-insertion and folding). Both produce a residual
typed core program that the ordinary checker and code generator handle.
The design principle: the driver is a library over the core language,
not a second compiler front end.

## 2 What the language already gives the driver

- **Typed quotes (P9)** are the currency of explicit runtime code
  generation (section 7), not of the automatic JIT. `Code a` is
  typechecked at quote time, so evaluating a quote cannot change a
  program's type. The automatic JIT operates on core closures and never
  involves quotes or any language surface at all.
- **Effect rows (P3/P6)** tell the driver exactly where it may not
  reorder: driving may float and duplicate pure work, but an effectful
  sequence is a fixed observation order (modules specification section 6:
  fusion and every other transformation preserve the observation model
  including divergence and effect order).
- **Multiplicities and usage (P5)** bound duplication: a `%1` binder
  may be consumed once on every path, so the driver may not duplicate or
  discard a linear value while specializing. This is a hard soundness
  gate that languages without linearity do not have, and it is checked
  by rerunning the usage checker on the residual program.
- **Dependent indices (P7)** make specialization type-directed:
  `Vec n a` at a known `n` drives to a fixed unrolling; erased evidence
  never reaches residual runtime code (erasure discipline rechecked).
- **Codata productivity (P8)**: a residual `corec` body is rechecked by
  the productivity guard, so specializing a stream cannot silently
  unguard it.
- **Rewrite and `justify` (P10)** are the small-scale transformation
  surface; supercompilation is the wholesale one. Both must answer to
  the same preservation discipline.
- **Branch weights (P17)** close the loop: a JIT observes real branch
  frequencies and can emit `#[weight(n)]` values into offline variants
  (an in-language PGO; see section 7).

## 3 The driver

Symbolic evaluation over core terms with a configuration table
(expression plus environment shape):

- **Drive** a configuration by unfolding definitions, casing on known
  constructors, and pushing known applications. Divergence is handled by
  a **whistle**: when a configuration repeats (up to renaming) or grows
  past a measure (size, depth), the driver stops unfolding and
  **folds** (residual call) or **generalizes** (residual `let` with the
  differing part abstracted).
- **Generalization** is where supercompilers traditionally blow up. The
  conservative rule this design proposes: generalize at every whistle,
  never attempt clever case-splitting generalizations without a cost
  model, and make the cost model a target profile parameter.
- **Effect barriers**: driving stops symbolically at an effectful
  primitive whose arguments are not all known; the call residualizes in
  its original position and the driver continues in its result. Handler
  bodies may be driven with the handler's clause map visible
  (handler specialization, section 6), but an operation the driver
  cannot resolve residualizes.
- **Termination**: driving terminates by the whistle plus a fuel bound
  from the target profile. A driver that runs out of fuel residualizes
  what it has; this must never be a correctness issue, only a missed
  optimization, and the compiler report records where specialization
  stopped (modules specification section 10).

## 4 Safety invariants of every residual program

The residual program MUST satisfy all of the following, and each is
checkable by rerunning existing machinery:

| Invariant | Rechecked by |
| --- | --- |
| Types and indices unchanged | the type checker (conversion per D28) |
| Effect rows and observation order unchanged | effect honesty; `handle`/`do` elaboration |
| `%1`/`%aff` usage preserved (no duplicated or dropped consumption) | the P5 usage checker |
| Memo/demand sharing preserved (a `~` cell not duplicated) | memo group typing (`Eval r` lacks) |
| Codata productivity preserved | the P8 guard check |
| Erasure preserved (no evidence reaches runtime) | the erasure pass |
| Capability requirements unchanged or narrowed | `Io`-head solving (D21) |
| Contract identity unchanged | `.sliface` contract projection (D4): specialization is representation-only |

The last row is the interface rule: a specialized variant of a module
has the same contract hash as its generic form. Specialization MUST NOT
change what a dependent observes, so `.sliface` identity excludes
specialization state while the artifact manifest records it (section 8).

## 5 Trust model

Three levels, in increasing cost:

1. **Trusted pass with manifest identification.** The driver is part of
   the compiler; the artifact manifest records that specialization ran,
   what it did, and whether its output was rechecked. The platform
   specification's rule applies: an artifact MUST identify which
   lowering was checked versus trusted.
2. **Recheck-everything.** The residual program is fully retypechecked
   and re-linted (the table in section 4). This catches the driver's
   internal inconsistencies but not semantic miscompilation.
3. **Translation validation per step.** Each driving step emits a
   certificate checked against the preservation discipline (the P10
   `justify` machinery generalized). Folding steps are justified by the
   recursive-call measure; generalization steps by a let-floating
   equivalence.

This design proposes shipping level 2 first (it is nearly free: the
checkers exist), keeping level 1's manifest identity regardless, and
treating level 3 as the P10 justify extension.

## 6 What specialization buys in this language

- **Dictionary specialization.** Class constraints elaborate to
  dictionary records (P3); when the driver sees a known dictionary it
  specializes the call, removing the projection and enabling further
  driving. The runtime capabilities specification already permits this
  ("implementations may specialize a statically known dictionary").
- **Handler specialization.** Handlers are vtables of clause closures
  (D23); a known handler at a `handle` site specializes to direct calls,
  removing the dispatch. The effect row of the handled computation
  guides which operations remain.
- **Index specialization.** `Vec n a` at a known `n` unrolls; `decide`
  with a ground comparison folds to `Yes`/`No` at compile time (the
  obligation statuses already distinguish these).
- **Fusion as state machines.** The modules specification's step form
  (`Done`, `Skip(state)`, `Yield(value, state)`) is a driving-friendly
  intermediate: stream pipelines drive to loops with no allocation,
  which is the map-fusion case P10 must demonstrate anyway.
- **Effect-static specialization.** A computation whose effect row is
  a known row can be specialized to a concrete capability record,
  removing effect polymorphism at the specialized boundary.

## 7 JIT architecture

The online half reuses P13 and P14 machinery rather than inventing a
second runtime story. One design position governs this section: **the
automatic JIT is invisible to the language**. How a function happens to
run is a performance property, never a semantic one, so it MUST NOT
appear in types, source syntax, or the compiled interface. This is the
same rule the attribute chapter gives hint attributes (section 1.3):
they may not affect type checking outcomes. Concretely:

- **Transparent tiering**: the baseline is the existing code
  generator's output; hot closures with observed stable arguments
  produce specialized variants (the P14 "baseline/specialized variant
  packaging" item). Nothing in the program names or requests this.
  Counters live at call sites and branch sites; the P17 branch weights
  are the same numbers seen from the other side.
- **Guards and deoptimization**: speculative specialization on a value
  installs a guard; guard failure transfers to the generic code through
  the versioned dispatch of P13. Deoptimization MUST be
  consumption-safe: if the specialized frame held a `%1` value, the
  fallback frame receives it exactly once. This is a runtime invariant
  checked when the specialized variant is built (the section 4 recheck
  of the specialize/deoptimize pair), not a type-level promise.
- **Live frames**: replacing an installed variant while frames are live
  is exactly runtime evolution specification section 6 ("the runtime
  MUST retain a code version while live frames can enter it"). The JIT
  MUST NOT introduce a second code-retention mechanism.
- **Wasm**: the same pipeline targets the Wasm artifact profile; the
  capability checks of the platform specification apply before any
  variant with new instruction requirements is reachable.

**Explicit runtime code generation is a separate feature.** `eval` of a
user-built `Code a` value (the staging slice's quotes) runs the compiler
as a library at runtime. It is type-safe by quote typing: `eval : Code a
-> a` cannot produce an ill-typed value because quotes are typed when
constructed, so no capability belongs in its signature and none is
proposed here. Its use is recorded in the artifact manifest (it is a
build-visible assumption like any other), a target profile MAY forbid it
entirely (a deployment that does not admit runtime code generation
simply never links the eval service), and its results are executable
code only, never evidence (section 10). The security story is the
manifest and target profile, exactly like `foreign import`: policy at
the deployment boundary, not per-function signatures.

## 8 Build identity and reporting

- The artifact manifest records: whether specialization ran, the
  driver's fuel and whistle configuration (target profile parameters),
  each specialization site with its stopping reason, and the trust level
  (section 5). This satisfies the modules specification's report MUST.
- Secrets and absolute paths MUST NOT appear in residual code or
  reports (the stage I/O rule of modules section 10 applies equally to
  runtime-generated code).
- A specialized variant's `.sliface` is byte-identical to its generic
  form's (section 4), so specialization never stales dependents.

## 9 Relationship to the roadmap

Pieces land in existing phases rather than a new one:

- P10 (rewrites, optimizer, target profiles): the offline supercompiler
  and the level-3 justify certificates; the map-fusion case is the first
  driving demonstration.
- P13 (runtime evolution): deoptimization and code retention for the
  JIT.
- P14 (Wasm and components): baseline/specialized variant packaging.
- P17 (attributes): the PGO loop in which JIT counters become
  `#[weight(n)]` values and `#[inline]` hints in offline rebuilds.

## 10 Open questions

- Cost model for generalization: what heuristic family (size, depth,
  constructor frequency) and where the knobs live (target profile vs
  module attributes).
- Whether `eval`'s result is usable in erased/`spec` positions (this
  design says no: runtime-generated code is executable code, never
  evidence).
- Interaction of specialization with `upgrade` (P13): is a specialized
  variant part of the update set when its generic form is replaced
  (the runtime evolution specification's "law-dependent specializations
  remain valid or are replaced" suggests yes).
- Whether driving should be exposed to users as a `rewrite`-level
  directive (specialize this function at these arguments) or stay
  compiler-internal.
- How much of level 3 (per-step certificates) can reuse the `justify`
  surface P10 defines for rewrite rules without inventing a second
  proof language.
- The D32 caveat applies: a diverging helper can hide behind a
  productive-looking type; driving a such a helper is fine (it just
  diverges residually), but fuel-bounded driving must not mistake a
  divergence for a whistle in a way that changes the residual program's
  observed behavior.
