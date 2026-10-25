# Open decisions and implementation plan

Source: <https://chatgpt.com/space/page_a2ab477ad93881918c2b00c47fe9c608>  
Exported: 2026-10-22T18:59:36Z

---

[Documentation overview](../overview.md)

The language requirements are ambitious but coherent enough to define a research and implementation program. This page separates established goals from proposed mechanisms and unresolved semantic choices. No language name has been selected.

# Established requirements

- Strict functional execution with full dependent types from the core.

- Memory safety and explicit ownership of resources, with useful error and effect guarantees.

- Incremental verification without treating missing evidence as established truth.

- Haskell-like higher-order abstraction and TypeScript-like local narrowing and record ergonomics.

- Modules or typeclass-style abstraction that carry different result families, laws, and guarantees.

- Inductive, coinductive, and temporal reasoning, including infinite behavior.

- User-defined rewrite rules and stream fusion with appropriate correctness evidence.

- Platform-specific proof modules and SIMD rewrites with explicit target semantics, availability, and fallback policies.

- Hygienic, cross-module metaprogramming with fewer accidental staging restrictions.

- Compile-time file, subprocess, network, and database access.

- Compatible hot swapping across interfaces, state, resources, protocols, temporal properties, and compiled dependencies.

- FFI support for external libraries with explicit ownership, ABI, effect and trust boundaries.

- First-class Wasm/WASI deployment and a concurrency model that composes with proofs and hot replacement.

# Proposed mechanisms

| Proposal | Why it fits |
| --- | --- |
| Dependent modules with implicit dictionary arguments | Unifies explicit abstraction and typeclass ergonomics |
| A quantitative dependent foundation | Connects erasure and usage, while leaving borrowing to be modeled explicitly |
| Total logical fragment and effectful executable computations | Permits services without admitting divergent proofs |
| Explicit runtime decisions | Makes dynamic verification visible and predictable |
| Proof-carrying optimizer rules | Lets libraries provide checked specialization and fusion |
| Target-indexed proof modules | Shares algorithm proofs while exposing architecture-specific instruction and memory laws |
| Guarded codata plus separate processes | Separates productive data from unbounded search and waiting |
| Captured stage inputs and replay | Supports real external generation and reproducible builds |
| Checked upgrade packages with runtime safe points | Combines static compatibility with operational coordination |
| C ABI plus checked wrappers and explicit foreign assumptions | Practical native integration with reviewable trust |
| WASI 0.3 component profile with pinned 0.2 compatibility | Portable typed interfaces and async interoperability |
| Structured scopes, bounded channels and optional STM | Haskell-like composition with explicit lifetimes and cancellation |

# Decisions that need a formal design

| Question | Decision needed before |
| --- | --- |
| Universe hierarchy, equality, proof irrelevance and elimination | Kernel implementation |
| Resource usage algebra and dependent borrowing | Claiming memory and logical soundness together |
| Effect calculus and continuation multiplicity | General handlers and asynchronous APIs |
| Guarded recursion, clocks or sized types | Accepting productive codata and coinductive proofs |
| Immutable sharing and reclamation strategy | Stable runtime representation and performance claims |
| Abstract type identity and functor generativity | Separate compilation and module evolution |
| Optimization equivalence and observable allocation failure | Certifying transformations consistently |
| Typed-code contexts and stage dependencies | Hygienic dependent metaprogramming |
| Migration atomicity and cancellation semantics | Production hot replacement |
| Certificate formats and trusted backends | Meaningful artifact assurance profiles |
| Instruction models, target capability lifetime and scalable widths | Sound platform-specific SIMD specialization |
| Native pointer provenance, callback entry and unwind policy | Publishing safe foreign wrappers |
| Source concurrency memory model and backend correspondence | Claiming data-race freedom across native and Wasm |
| Cancellation, task failure and cleanup semantics | Stabilizing async APIs |
| Exact component profiles, host support and adapter rules | Declaring a supported Wasm deployment matrix |
| STM rollback ownership and scheduling assumptions | Claiming transactional and progress guarantees |

Syntax remains provisional. A concrete surface grammar is now specified in draft 0.2; the sound core calculus and elaboration judgments still require formalization. Surface convenience must elaborate to explicit core evidence; it cannot substitute for these decisions.

# Implementation milestones

1. **Define the core.** Specify universes, dependent functions and pairs, inductive families, equality, erasure, and totality. Implement a small checker and reference evaluator. Full dependent types are present from this milestone.

2. **Integrate resources and effects.** Formalize usage, borrows, ghost views, failure, and cancellation. Prove the relevant preservation and logical-soundness results for the selected subset.

3. **Add elaboration and modules.** Implement inference, narrowing, dependent records, dictionary resolution, opaque interfaces, and useful proof diagnostics.

4. **Add infinite behavior.** Implement guarded codata and its proof principles. Define process semantics and the bridge from executable code to temporal models.

5. **Add metaprogramming.** Implement hygienic expansion, typed quotations, stage dependency scheduling, capability-scoped I/O, captured inputs, and replay.

6. **Add checked optimization.** Begin with directed rules for a small pure IR, then stream fusion, effect-aware lowerings, target-specific SIMD rules, and dependency manifests. Expand only when preservation arguments exist.

7. **Add runtime evolution.** Begin with drain-and-replace for private module state. Then add coexistence and explicit continuation migration with their additional proofs.

Each milestone should run the relevant positive and negative cases in the example catalogue once an implementation exists. The milestones do not defer dependent types; they progressively establish the other features around a dependent foundation.

# Research risks to resolve early

The most difficult interfaces are dependent types with mutable ownership, erased logic with executable witnesses, coinduction with effects, and optimization dependencies with live replacement. A prototype that treats these as unrelated plugins would miss the core design problem.

The initial deliverable should be a small coherent calculus and a few complete end-to-end examples, including a checked vector operation, a productive stream law, a generated schema interface, and a stateful upgrade. A large syntax surface without those examples would provide little evidence that the design works.

# Backend and concurrency implementation track

Alongside the core milestones, implement a small C ABI wrapper layer, a portable core-Wasm backend and WIT adapters. Begin concurrency with local structured scopes, cooperative cancellation, bounded channels and explicit blocking-call handling. Establish the source memory model before adding migratable tasks, atomics or claims about shared-memory execution.

Use an isolated codec component and a scoped two-request client as early integration examples. Test ownership at cancellation races, foreign callback draining, component trap handling, cross-target widths and Wasm feature selection. Extend the hot-upgrade milestone to these resources before adding arbitrary continuation migration. STM and advanced session or lock-free libraries remain later work built on the checked foundations.

# Draft 0.2 syntax and runtime decisions

Adopt a strict Haskell-like surface with unqualified module/typeclass-driven `do`. Suspension is an effect of ordinary functions, with lexical runtime capabilities passed explicitly in the core. The published EBNF fixes concrete syntax; the formal elaboration, handler ownership and indexed sequencing bridge remain proof and implementation work. Initial sequencing providers are direct, Result and Option. Arbitrary indexed providers require the checked bridge described in [Runtime capabilities, effects and sequencing](../specifications/runtime-capabilities-effects-and-sequencing.md).

