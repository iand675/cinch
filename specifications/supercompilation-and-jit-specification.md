# Supercompilation and JIT specification

Source: local draft in this repository (added after the documentation export)
Added: 2026-10-25

---

[Documentation overview](../overview.md)

Draft 0.1. This chapter defines what whole-program specialization
(supercompilation) and runtime specialization (JIT) may manage, what
they MUST preserve, and where they MUST stop. It follows the design
study `design/supercompilation-and-jit.md`, which remains the
implementation reference. MUST denotes a requirement of this draft.
Proposed choices are identified explicitly; unresolved items are listed
in Open decisions. The chapter is deliberately written in terms of
observable obligations rather than algorithms: a conforming
implementation may drive by any method whose residual programs satisfy
sections 3 and 4.

# 1 Terms and scope

- A **configuration** is a core expression together with the static
  shape of its environment (known constructors, known dictionaries,
  known indices).
- **Driving** is symbolic evaluation of configurations: unfold known
  calls, case on known constructors, propagate known values, and stop at
  barriers (section 3).
- A **whistle** fires when a configuration repeats up to renaming or
  grows past a measure bound. The driver then **folds** (emit a residual
  call) or **generalizes** (emit a residual `let` abstracting the
  differing part).
- The **residual program** is the typed core program the driver emits.
- A **variant** is a residual program installed in place of, or beside,
  its **generic form** (the program as written).
- **Offline supercompilation** runs at build time. The **JIT** runs at
  runtime. Both are consumers of one driver and both are governed by
  this chapter.

This chapter defines no surface syntax. Automatic specialization is
invisible to the language (section 6). Explicit runtime code generation
is a separate feature defined in section 7.

# 2 What specialization manages

Specialization MAY perform the following transformations, each subject
to the preservation obligations of section 4.

- **Known-application unfolding**: a call with a statically known
  definition and known arguments MAY be driven into its body, bounded by
  fuel (section 5).
- **Case-of-known-constructor**: a `case` on a known constructor MAY be
  reduced to the matching alternative; the decision point and its
  weights MAY be discarded.
- **Dictionary specialization**: a call constrained by a class whose
  dictionary is statically known MAY be specialized to direct method
  calls, removing dictionary projection and passing. The runtime
  capabilities specification's rule applies: specialization of a known
  dictionary is permitted, never guaranteed.
- **Handler specialization**: a `handle` with a statically known handler
  MAY be specialized to direct clause calls, removing vtable dispatch
  for the operations the driver can resolve.
- **Index specialization**: a family type applied to known indices MAY
  be driven to a fixed unrolling (`Vec n a` at a known `n`), and a
  ground `decide` comparison MAY fold to `Yes evidence` or `No
  refutation` at specialization time using the ordinary obligation
  statuses.
- **Effect-static specialization**: a computation whose effect row is
  statically known MAY be specialized to a concrete capability record,
  removing effect polymorphism at the specialized boundary.
- **Fusion**: producer/consumer pipelines over the step form (`Done`,
  `Skip(state)`, `Yield(value, state)`) MAY be driven to loops with no
  intermediate allocation, subject to the observation-preservation rule
  of the modules specification section 6.
- **Constant folding** of pure total primitives on known arguments
  (machine arithmetic families, `Nat`/`Int` arithmetic, comparisons,
  conversions), using the defined fault behavior for out-of-domain
  results (a folded `checkedDiv` on zero folds to `None`; a folded
  division that faults MUST NOT be silently dropped).

# 3 Barriers: where driving MUST stop symbolically

- **Effect barriers**: a primitive or call whose effect row is not empty
  and whose result the driver needs MUST residualize in its original
  observation order. Driving MAY continue symbolically in its result.
  No transformation may reorder, duplicate, or drop an effectful
  computation relative to another.
- **Opaque boundaries**: `foreign` calls, `eval`-generated code, and
  variant dispatch through a replaceable boundary (platform
  specification section 13) are opaque: arguments MAY be specialized
  around them, the call itself MUST residualize.
- **Linear values**: a `%1` or `%aff` value MUST NOT be duplicated into
  two residual positions, and MUST NOT be dropped. A configuration that
  would require either MUST generalize instead. `%many` values and pure
  computations MAY be duplicated.
- **Memo and demand structure**: a `~` memo cell or thunk MUST NOT be
  duplicated or reordered relative to its demand; the `Eval r` lacks
  discipline of the laziness specification MUST hold of the residual
  program unchanged.
- **Codata observations**: a `corec` construction MUST NOT be driven in
  a way that removes a `delay` guard from a recursive call (the
  productivity check of the codata rules re-verifies this).
- **Erased components**: evidence and erased fields MUST NOT appear in
  residual runtime code; specialization MUST NOT resurrect erased data.
- **Rigid indices**: driving MUST NOT invent refinements the checker
  would reject (the dependent core's rigid-index rule, decision D35).
  Stuck index equations residualize.

# 4 Preservation obligations

Every residual program MUST satisfy all of the following. Each is
checked by rerunning the ordinary machinery; an implementation MUST NOT
install a variant whose recheck fails.

| Property | Rechecked by |
| --- | --- |
| Types and indices | type checking, conversion as in the dependent core rules |
| Effect rows and observation order | effect honesty; handler and `do` elaboration |
| `%1`/`%aff` usage (consumed exactly once per path) | the usage checker |
| Memo/demand sharing | memo group typing (`Eval r` lacks) |
| Codata productivity | the guarded `corec` check |
| Erasure (no evidence at runtime) | the erasure pass |
| Capability requirements unchanged or narrowed | capability solving (the `Io` rule) |
| Contract identity | the compiled interface's contract projection |

The last row makes specialization **representation-only**: a variant's
compiled interface MUST be identical to its generic form's contract
projection. Specialization state (section 8) lives in the artifact
manifest, never in the interface identity, so a variant NEVER marks
dependents stale.

Effect order is the strictest of these: a residual program that computes
the same values in a different observable order is not conforming, even
if the source is pure overall. The modules specification's fusion rule
("fusion MUST preserve the selected observation model including
divergence and effect order") applies to every transformation in this
chapter, not only fusion.

# 5 Driving bounds

- Driving MUST terminate on every input. Termination MAY rely on a
  whistle (repetition or measure growth) and a fuel bound.
- Fuel and measure bounds are target profile parameters. A driver that
  exhausts its bounds MUST residualize what it has; running out of fuel
  MAY lose an optimization, and MUST NOT change the residual program's
  observed behavior. The compiler report MUST record where
  specialization stopped and which barrier or bound stopped it (modules
  specification section 10).
- Generalization MUST happen at whistles. An implementation MAY use a
  cost model to choose between folding and generalizing, and to attempt
  deeper case-splitting generalizations; the cost model MUST NOT affect
  correctness, and its parameters belong to the target profile.
- Whistle configuration, fuel, and cost model MUST appear in the
  artifact manifest so two builds with different specialization
  settings are distinguishable.

# 6 The JIT is invisible

How a program is executed at runtime is not part of the language.

- JIT tiering, guards, counters, and installed variants MUST NOT appear
  in types, in source syntax, or in the compiled interface. A program's
  meaning MUST NOT depend on whether any of it ran. This is the same
  rule the attribute chapter states for hint attributes: performance
  mechanisms MUST NOT affect type checking outcomes.
- A speculative variant MAY be installed under guards observed at
  runtime. Deoptimization on guard failure MUST be **consumption-safe**:
  every value the specialized frame consumed or held under a `%1` or
  `%aff` obligation MUST be transferred to the fallback frame exactly
  once. The recheck of section 4 applies to the specialize/deoptimize
  pair when the variant is built.
- Installing, replacing, or retiring a variant MUST follow the runtime
  evolution specification: old code versions are retained while live
  frames can enter them, an update set MUST be complete before
  activation, and a variant with new instruction requirements MUST
  establish their availability before it becomes reachable (platform
  specification section 13). A JIT MUST NOT introduce a second
  code-retention or dispatch mechanism.
- A JIT MUST NOT weaken the `upgrade` obligations: if a generic form is
  replaced by an authorized update, its specialized variants are part of
  the dependent update set and MUST be replaced or invalidated.

# 7 Explicit runtime code generation

Evaluating a typed quotation at runtime (the staging chapter's `Code a`)
runs the compiler as a library. This is a separate feature from the JIT.

- `eval : Code a -> a` is typed by the quotation's type; evaluating a
  quote of type `Code a` yields a value of type `a` and MUST NOT change
  the program's types. Quotes are typechecked when constructed, so no
  capability or effect belongs in `eval`'s signature beyond the
  quotation type itself.
- `eval` is never evidence: a runtime-generated value is executable
  code only and MUST NOT be usable in `spec`/`proof` or erased
  positions.
- A failing evaluation (malformed quote contents beyond what quote
  typing admits, resource exhaustion) MUST take the defined fault path;
  it MUST NOT produce a value of the quoted type.
- Use of `eval` MUST be recorded in the artifact manifest as a
  build-visible assumption, in the same table as `foreign` assumptions.
  A target profile MAY forbid runtime code generation entirely; a
  program that uses `eval` in such a profile MUST be rejected at
  compile time.

# 8 Reporting and identity

- The artifact manifest MUST record: whether specialization ran; the
  whistle, fuel, and cost-model configuration; each specialization site
  with its stopping reason; the trust level (section 9); and whether
  runtime code generation was used.
- The compiler report SHOULD identify which transformations occurred
  (fusion steps, dictionary and handler specializations, index
  unrollings), where specialization stopped, and which allocation,
  effect, or bound prevented a requested transformation (modules
  specification section 10).
- Secrets and absolute paths MUST NOT appear in residual code or
  reports.
- A variant's compiled interface identity is its generic form's
  (section 4). The manifest MAY record the variant's own content hash
  for build reproducibility; that hash MUST NOT be used for dependency
  staleness.

# 9 Trust levels

An implementation MUST identify, per artifact, which level it used
(the platform specification's checked-versus-trusted rule):

1. **Manifest identity only**: the driver is trusted; the manifest
   records what it did.
2. **Full recheck**: every residual program passes the section 4
   recheck before installation. This level MUST be the default; level 1
   is permitted only for whole-artifact offline output that is also
   rechecked at the next build.
3. **Step certificates**: each driving step carries a certificate
   checked against the preservation obligations (the `justify`
   machinery of the modules specification's rewrite rules). Reserved
   for future work.

A level-2 implementation that cannot recheck a candidate variant MUST
discard it and fall back to the generic form; it MUST NOT install it.

# 10 Limits

Conforming implementations are not required to, and MUST NOT pretend
to, manage the following:

- **Termination of driving is heuristic**: whistles and fuel are
  conservative; a missed fold is legal, a changed observation is not.
- **Size explosion** is bounded only by the cost model and fuel; the
  report MUST say when it stopped.
- **Effectful code** specializes around barriers, not through them
  (section 3); a pipeline whose performance needs effect reordering
  needs a rewrite rule with `justify`, not driving.
- **Rigid indices** follow the dependent core's limit (D35): no
  refinement the checker itself would reject.
- **Hidden divergence**: the codata rules' standing caveat applies; a
  productive-looking type over a diverging helper is not repaired or
  detected by specialization.
- **Opaque foreign code** and replaceable dispatch boundaries limit
  cross-boundary specialization (platform specification section 13).
- **No cross-contract specialization**: a variant MUST NOT change or
  assume beyond its generic form's contract projection, even when the
  whole program is visible offline.

# 11 Open decisions

- Whether users may direct specialization explicitly (for example a
  `rewrite`-level directive naming functions and arguments to drive) or
  it stays compiler-internal with target-profile settings; this draft
  permits only the latter.
- The default fuel and whistle measures per target profile.
- Whether section 9 level 3 certificates reuse the `justify` surface or
  need their own form.
- Whether specialized variants participate in `upgrade` verification as
  first-class update-set members or are invalidated and regenerated
  (section 6 requires one of the two; this draft does not choose).
- Whether the PGO loop (runtime counters emitting `#[weight(n)]` and
  `#[inline]` values into offline rebuilds, per the attribute chapter)
  is a toolchain feature or a user-invoked one.

# 12 Implementation status

Not implemented. The design study
`design/supercompilation-and-jit.md` is the implementation reference;
the implementation items ride the roadmap phases P10 (offline
supercompiler), P13 (JIT deoptimization and code retention), P14
(baseline/specialized variant packaging), and P17 (the PGO loop through
branch weights and inline hints) in `compiler/CONFORMANCE.md`.

# Amendments

Dated notes recording landed decisions (`compiler/CONFORMANCE.md`,
D-numbers) that amend or interpret this chapter's normative text.
2026-10-28, P15 conformance sweep.

- **Driving, fusion, and weights (D60-D61), 2026-10-28.** This amends
  sections 2-3: pull-based producer fusion composes callbacks at the
  same per-element positions, preserving effect order even for
  effectful callbacks; barriers are memo/thunk allocation (demand
  reordering) and, for eager pipelines under the `mapMap` law, callback
  purity; case folds let-bind constructor fields, preserving strict
  evaluation order (D60). Weight reconciliation under compiled shapes
  (default alternatives expanded per constructor with scrutinee synonym
  `let`s): inner alternatives outside the default set are conditioned
  away unrescaled, then the rescale formula and GCD run; dead-
  alternative removal is negation conditioning; case-push duplicates
  decisions keeping weights (D61).
- **Driving bounds (D62), 2026-10-28.** This amends section 5: fuel
  charges every transformation (zero fuel means the generic form, the
  behavior baseline); the whistle is recursion repetition plus 8x
  measure growth; bounds are `TargetProfile` fields and land in the
  manifest.
- **Trust level (D63), 2026-10-28.** This amends section 9: level 2
  (full recheck) per artifact with the recheck covering core
  structure/scoping, `%1`/`%aff` usage equality, memo/demand counts,
  weight all-or-nothing, and contract identity; a failing candidate is
  discarded. Type/effect-order preservation is by construction (core
  carries no types) and is disclosed in `lowering.trusted`.
- **`eval` mechanism (D92), 2026-10-28.** This amends section 7's
  opening claim ("runs the compiler as a library"): `eval` runs a
  build-time-reified program on the runtime evaluator (the compiler's
  encoder and `runtime/src/eval.rs` decoder are one documented format)
  rather than embedding the compiler as a library. The section 7
  contract holds unchanged (typed by quotes, never evidence,
  manifest-recorded in `assumptions.foreign`, profiles MAY forbid it),
  and this is the only realistic lowering for wasm32 guests.
- **Implementation status correction (2026-10-28).** Section 12's
  "Not implemented" predates the P10 landing: the offline driver,
  fusion, bounds, weight reconciliation and the level-2 recheck are
  implemented (D60-D63; `tests/optimizer.rs`). The remaining named
  barriers: handler-vtable specialization is blocked on propagating
  concrete vtables across function boundaries (dispatch sites can live
  in callees through `do`/`spawn`), and named index-specialization
  sites are folded via `Vec` chains and ground `decide` instead.
- **Cinch naming (D278-D281), 2026-10-28.** The language is Cinch:
  sources `.cinch`, interfaces `.cinchif`, runtime externs `cinch_*`;
  artifact names in this chapter read with those names.
