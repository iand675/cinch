# Cinch formal verification delivery checklist

Specification: [Formal verification specification and implementation roadmap](formal-verification-roadmap.md)

Source: [Collaborative specification and checklist](https://chatgpt.com/space/page_6c092eb66dd08191bfd6538374983935). Track repository acceptance evidence and checkbox updates in this file; the source Page is not automatically synchronized.

Use this checklist to track delivery of the specification. A checkbox means that the stated acceptance condition has evidence for the recorded revision. Writing a design, implementing a feature, proving its rules sound, and demonstrating its usability are separate deliverables.

All boxes start unchecked because this tracker has not audited completion evidence. An unchecked item means not yet established here; it does not claim the existing compiler lacks the feature.

For an implementation item, attach the source or change, reproducible commands, positive and negative results, applicable Rust and actual self-hosted results, and independent replay where the item establishes logical evidence. Record limits and assumptions. Design, metatheory, performance, and usability items need their corresponding documents, proofs, measurements, or study results.

Use the stable item IDs in issues and evidence records. Track work as Not assessed, Planned, In progress, Blocked, Awaiting review, or Accepted. Check the box only at Accepted. Reassess affected entries when their implementation or semantic dependencies change.

## Foundation and independent checking

Phase 1 provides the foundation; Phase 9 extends executable assurance.

- [ ] FND-01 Publish exact K0 syntax, context formation, typing, conversion, universe, inductive, and recursion rules.

- [ ] FND-02 Check universe polymorphism and reject Type in Type, invalid level constraints, and escaping universe metavariables.

- [ ] FND-03 Implement proof irrelevance and Prop elimination; reject extraction of an executable witness from an erased existential.

- [ ] FND-04 Implement positivity and generated eliminators; reject negative recursion and invalid elimination targets.

- [ ] FND-05 Implement equality transport and controlled transparent reduction; keep arbitrary solver facts out of definitional equality.

- [ ] FND-06 Implement Setoid quotients, respectful lifting, and propositional induction; reject representative-sensitive operations.

- [ ] FND-07 Replay retained declarations and proofs in the independent checker; reject corrupted terms and unknown artifact semantics.

- [ ] FND-08 Publish the exact scope of metatheory results and named foundational principles, including unresolved soundness work.

## Dependent programming

Phase 2 must exercise unknown indices and ordinary abstraction.

- [ ] DEP-01 Implement scoped metavariables, occurs checking, higher-order pattern assignments, and explicit blocked outcomes.

- [ ] DEP-02 Match Vec n at an unknown rigid n using branch equations without globally assigning n.

- [ ] DEP-03 Check motives and transports for nested dependent matches, pairs, and records.

- [ ] DEP-04 Distinguish reachable, refuted, and unresolved coverage cases; retain evidence for omitted impossible branches.

- [ ] DEP-05 Accept checked positive nesting and valid mutual inductive declarations; reject a forged positivity claim.

- [ ] DEP-06 Accept structural, well-founded, and justified mutual recursion; reject missing decreases and divergent logical evidence.

- [ ] DEP-07 Preserve usage, effects, regions, and predicates through higher-rank callbacks and local generalization.

- [ ] DEP-08 Run symbolic append, zip, existential filtering, and universe-polymorphic examples through imports and both compiler paths.

## Contracts and readable syntax

Phases 2 and 3 deliver the forms and their semantics; Phase 4 exposes them interactively.

- [ ] CON-01 Parse shorthand named-result contracts and long-form requires and ensures into the same checked contract.

- [ ] CON-02 Classify whole shorthand clauses by resolved binder identity; test shadowing, implications, and disjunctions.

- [ ] CON-03 Require explicit roles for ambiguous stateful contracts; never guess entry versus exit state.

- [ ] CON-04 Define arbitrary predicates and user measures; separate refinement formation from runtime decidability.

- [ ] CON-05 Check weakening and strengthening evidence; reject forged witnesses and silently inserted fallible runtime checks.

- [ ] CON-06 Preserve result relationships through inferred lets, partial application, callbacks, containers, and exported signatures.

- [ ] CON-07 Express divide's exact quotient relation and use it in a separate remainder client without reopening the implementation.

- [ ] CON-08 Round-trip formatting and lossless contract expansion or compaction; preserve guarantees during signature extraction.

## Refinement inference

Phase 3 delivers inference with explicit supported limits.

- [ ] INF-01 Generate a finite qualifier pool with stable term, sort, dictionary, scope, and state identities.

- [ ] INF-02 Generate constrained Horn obligations for entry, branches, construction, calls, returns, and recursion.

- [ ] INF-03 Implement monotone candidate elimination with bounded termination and distinct unknown or resource-limit outcomes.

- [ ] INF-04 Check initialization, preservation, exit, and recursive justification for every accepted inferred invariant.

- [ ] INF-05 Check callback preconditions contravariantly and results covariantly; keep generic predicate parameters rigid.

- [ ] INF-06 Reject hidden public precondition strengthening or postcondition weakening; publish stable inferred interfaces explicitly.

- [ ] INF-07 Demonstrate inferred bounds and a recursive counter invariant; explain insufficient qualifiers and unsupported nonlinear goals.

## Tactics and metaprogramming

Phases 4 through 6 deliver the public API; dependent induction also requires Phase 2.

- [ ] TAC-01 Expose typed goal and context inspection with stable opaque identities and accurate state and usage information.

- [ ] TAC-02 Implement scoped expression construction, type inspection, controlled reduction, checking, and goal closure.

- [ ] TAC-03 Implement introduce, apply, refine, induction, and provisional goal graphs with no unresolved evidence at finalization.

- [ ] TAC-04 Roll back constraints, goals, usages, and declarations on failed alternatives; invalidate escaped transaction handles.

- [ ] TAC-05 Keep budgets and cancellation outside rollback; reject success without a completed checked goal.

- [ ] TAC-06 Implement library assumption, simplification, arithmetic, and composed tactics without adding strategy-specific compiler cases.

- [ ] TAC-07 Implement category-checked quotation, hygienic splicing, and typed declaration emission with collision and visibility checks.

- [ ] TAC-08 Generate Point equality with checked soundness and completeness proofs; reject unsupported fields and invalid generated proofs.

## Automated verification

Phase 5 combines methods while retaining the original claim.

- [ ] AUT-01 Generate contextual obligations plus a checked reconstruction theorem connecting them to program correctness.

- [ ] AUT-02 Reflect total definitions with controlled unfolding; do not treat fuel exhaustion or circular equations as proof.

- [ ] AUT-03 Instantiate quantified lemmas with checked scope, types, universes, premises, and originating theorem dependencies.

- [ ] AUT-04 Combine congruence, datatypes, integer arithmetic, and algebraic reasoning through checked shared-term explanations.

- [ ] AUT-05 Validate solver translations for Nat, exact subtraction, bitvectors, indexed data, and selected operation semantics.

- [ ] AUT-06 Replay supported external certificates without the solver; reject unsupported proof steps and altered certificates.

- [ ] AUT-07 Distinguish actual counterexamples from candidate models, unknown results, timeouts, and provider failures.

## State effects and resources

Phase 7 establishes sequential specifications; Phase 8 extends concurrent and infinite behavior.

- [ ] STA-01 Specify heap identities, allocation generations, permissions, aliases, and logical state versions.

- [ ] STA-02 Implement outcome-indexed specifications for return, error, cancellation, and specified terminal faults.

- [ ] STA-03 Check sequencing, branching, framing, and loop or recursion invariants against the computation semantics.

- [ ] STA-04 Distinguish partial from total correctness and record blocking, environment, and termination assumptions.

- [ ] STA-05 Keep snapshots about earlier states distinct from live observations; reject stale evidence and forged permission duplication.

- [ ] STA-06 Check borrow creation, writes, allocation, deallocation, and resource recovery on every specified outcome.

- [ ] STA-07 Preserve contracts through cross-call handlers and callbacks; reject repeated use of one-shot continuations.

- [ ] STA-08 Verify byte-copy content, bounds, unchanged state, framing, and termination; reject aliasing and off-by-one variants.

- [ ] STA-09 Specify and test productivity, interference, synchronization, and fairness before claiming concurrent correctness or liveness.

## Editor REPL and human usability

Phase 4 starts this work; persistent effectful sessions also depend on Phase 7.

- [ ] UX-01 Share semantic snapshots across CLI, editor, REPL, syntax extensions, and tactics.

- [ ] UX-02 Prevent stale diagnostics, goal results, and code actions from applying to a newer document revision.

- [ ] UX-03 Show expected types, hypotheses, effects, resources, state validity, and assumptions with progressively expandable detail.

- [ ] UX-04 Offer checked theorem application, case splitting, rewriting, proof traces, and lossless contract-view conversion.

- [ ] UX-05 Make completion and curried signature help account for contracts, effects, remaining arguments, and consumed resources.

- [ ] UX-06 Develop a proof through named holes in the editor and REPL, export source, and check it with the ordinary CLI.

- [ ] UX-07 Keep inspection from executing effects; make runtime execution, worker identity, capabilities, and interruption explicit.

- [ ] UX-08 Prevent undo, history replay, or reload from reviving consumed handles or claiming to reverse external effects.

- [ ] UX-09 Measure whether newcomers can explain contracts, repair calls, distinguish proof from execution, and finish a guided proof.

- [ ] UX-10 Measure declared latency and recovery targets on reference projects; keep navigation responsive during expensive search.

## Library syntax and macro tooling

Phases 4 and 6 integrate the experimental syntax work into production tooling.

- [ ] MAC-01 Load explicit, versioned syntax packages without ambient changes to unrelated host grammar.

- [ ] MAC-02 Check categories, binding relationships, hygiene, and evaluation structure of host-code holes.

- [ ] MAC-03 Complete and inspect host code inside unfinished syntax islands using the real typed context.

- [ ] MAC-04 Map nested expansion errors, definitions, and public names back to original source.

- [ ] MAC-05 Preserve meaning and raw text during formatting and rename; do not edit transient generated code as durable source.

- [ ] MAC-06 Record staging inputs and invalidate affected parses, expansions, interfaces, and proofs when dependencies change.

- [ ] MAC-07 Cancel or reject broken extensions without publishing partial declarations or freezing the workspace.

## Libraries dictionaries and scale

Phases 6 and 10 establish reusable coverage.

- [ ] LIB-01 Deliver logical, data, algebra/order, automation, verified-program, and low-level-model library layers with documented support.

- [ ] LIB-02 Supply reusable theorem APIs for append, vectors, maps, sorting, and other selected central operations.

- [ ] LIB-03 Implement checked superclass projections and associated result families; reject incoherent dictionary diamonds.

- [ ] LIB-04 Make theorem search check actual instantiations and show remaining premises and new assumptions.

- [ ] LIB-05 Separate opaque proof interfaces from transparent-body dependencies; invalidate changes by semantic identity.

- [ ] LIB-06 Measure cold/warm checking, memory, artifact size, and rebuild scope across interacting packages.

- [ ] LIB-07 Have independent contributors complete substantial developments without domain-specific compiler patches; document missing coverage.

## Primitive and executable assurance

Phase 9 connects mathematical claims to selected executable paths.

- [ ] EXE-01 Model machine operations with exact widths, signedness, overflow, conversion, shift, and failure semantics.

- [ ] EXE-02 Check primitive bodies against versioned instruction models; retain remaining backend and runtime trust.

- [ ] EXE-03 Check selected optimization equivalences and side conditions, including effects and resource observations.

- [ ] EXE-04 Validate erasure and required runtime witnesses; reject a proof-only value used to manufacture executable data.

- [ ] EXE-05 Separate kernel-checked and compiled-check paths in artifacts and policy enforcement.

- [ ] EXE-06 Invalidate or reject changed target capabilities, model versions, dictionary choices, and foreign contracts.

## Complete demonstration gates

These combine the earlier items and must use original source rather than hand-adjusted checker inputs.

- [ ] DEM-01 Verify exact division, retain its contract through partial application and an import, and prove the remainder client.

- [ ] DEM-02 Develop symbolic vectors and existential filtering through holes, then replay completed evidence independently.

- [ ] DEM-03 Verify recursive sorting with ordering and permutation properties under its stated termination and comparison assumptions.

- [ ] DEM-04 Construct quotient-based rationals and respectful operations; reject a representative-sensitive function.

- [ ] DEM-05 Verify mutable byte copy or sorting with snapshots, framing, permissions, and relevant cleanup paths.

- [ ] DEM-06 Debug a generated declaration in original syntax, export a proof, and replay it without the search implementation.

- [ ] DEM-07 Deliver an independent author's mathematical package combining structures, induction, quotients, and arithmetic.

## Release and comparison gates

Apply these to the declared release scope. An explicitly deferred feature stays unchecked and outside that scope.

- [ ] REL-01 Pin compiler, seed, library, interface, model, syntax-package, target, and checker versions.

- [ ] REL-02 Link each claimed checklist item to item-specific evidence and a reviewed scope statement.

- [ ] REL-03 Run the applicable original positive and negative corpus through Rust and the actual self-hosted compiler without hidden fallback.

- [ ] REL-04 Replay completed logical evidence independently and publish transitive assumptions and checking tiers.

- [ ] REL-05 Keep parser, runtime, test-sample, formal-proof, usability, and performance evidence separately classified.

- [ ] REL-06 Publish meaningful known failures and deferred items; reopen affected acceptances after semantic changes.

- [ ] REL-07 Compare Lean, F*, Idris, and Liquid Haskell on pinned workloads with matched claims, assumptions, and separate performance measures.

- [ ] REL-08 Make no broad parity or superiority claim from a bounded demonstration; link each comparison claim to its actual evidence.

## Evidence record template

Maintain one record per item in a linked issue or an evidence log. A code link alone does not replace a reproducible acceptance result.

```text
Item ID:
Owner:
State:
Revision and dependency versions:
Delivered scope:
Source or design reference:
Reproduction command or review procedure:
Positive and negative acceptance evidence:
Rust and self-hosted results, where applicable:
Independent checking or metatheory evidence, where applicable:
Assumptions and remaining limitations:
Reviewer and review date:
```

The next working session should assign owners, audit existing evidence, and select the first dependency-complete batch. Preserve these IDs when wording improves. If an item is split, retain its relationship to the original rather than silently marking a partial implementation complete.
