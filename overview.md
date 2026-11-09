# Language design overview

Source: <https://chatgpt.com/space/page_b647a1112d5c8191a20ef25f039a366a>  
Exported: 2026-10-22T18:59:36Z

---

Draft 0.2 • 22 October 2026 • Working title: Unnamed language

The language is a strict, fully dependently typed functional language for programs whose guarantees are checked before they are accepted for deployment. It combines Haskell-like syntax and abstraction, TypeScript-style local ergonomics, Rust-like control of memory and resources, and explicit support for proofs about ongoing processes.

Libraries can package types, operations, semantic models, proofs, code generators, and verified optimizations. Running modules can be replaced when a checked upgrade package and the current runtime state jointly establish compatibility.

This is a design specification, not documentation for an implemented compiler. The feature requirements below are established by the design conversation. Syntax, APIs, and selected mechanisms are proposals that make those requirements concrete. Examples use the draft 0.2 Haskell-like notation; their acceptance and rejection outcomes are intended conformance requirements, not reported compiler results.

# How this archive relates to the compiler

The Rust compiler in `compiler/` implements part of this draft. The [language tutorial](learn/00-tutorial.md) and the [compiler guide](reference/00-guide.md) document what it accepts today, and the [support matrix](reference/16-support-and-roadmap.md) records which parts of the draft are available, partial, reachable only through compiler APIs, or still planned. The pages in this archive are not updated to match the implementation: where one of them and a compiler diagnostic disagree, the diagnostic describes the behaviour you can run, and the page keeps the semantics the draft requires. Read the archive for intended meaning and for the reasoning behind a decision, not for current syntax.

# Documentation contents

## Specifications

- [Attribute annotations, representations, and branch weights](specifications/attribute-annotations-specification.md)
- [Concurrency specification](specifications/concurrency-specification.md)
- [Core language specification](specifications/core-language-specification.md)
- [Documentation comments specification](specifications/documentation-comments-specification.md)
- [Foreign function interface specification](specifications/foreign-function-interface-specification.md)
- [Lazy computation and reclamation specification](specifications/lazy-computation-and-reclamation-specification.md)
- [Modules and compilation specification](specifications/modules-and-compilation-specification.md)
- [Platform proofs and SIMD specification](specifications/platform-proofs-and-simd-specification.md)
- [Primitive operations and inline target IR specification](specifications/primitive-operations-and-inline-target-ir-specification.md)
- [Primitive types specification](specifications/primitive-types-specification.md)
- [Runtime capabilities, effects and sequencing](specifications/runtime-capabilities-effects-and-sequencing.md)
- [Runtime evolution specification](specifications/runtime-evolution-specification.md)
- [Supercompilation and JIT specification](specifications/supercompilation-and-jit-specification.md)
- [Surface grammar local amendments](specifications/surface-grammar-local-amendments.md)
- [Surface syntax and grammar](specifications/surface-syntax-and-grammar.md)
- [WebAssembly and WASI specification](specifications/webassembly-and-wasi-specification.md)

## Tutorials

- [Tutorial 1 Values and dependent types](tutorials/01-values-and-dependent-types.md)
- [Tutorial 2 Proofs and runtime checks](tutorials/02-proofs-and-runtime-checks.md)
- [Tutorial 3 Ownership and effects](tutorials/03-ownership-and-effects.md)
- [Tutorial 4 Modules and laws](tutorials/04-modules-and-laws.md)
- [Tutorial 5 Streams and coinduction](tutorials/05-streams-and-coinduction.md)
- [Tutorial 6 Processes and temporal contracts](tutorials/06-processes-and-temporal-contracts.md)
- [Tutorial 7 Rewrites and stream fusion](tutorials/07-rewrites-and-stream-fusion.md)
- [Tutorial 8 Staging and schema generation](tutorials/08-staging-and-schema-generation.md)
- [Tutorial 9 Compatible hot upgrades](tutorials/09-compatible-hot-upgrades.md)
- [Tutorial 10 Platform specific SIMD](tutorials/10-platform-specific-simd.md)
- [Tutorial 11 Foreign libraries](tutorials/11-foreign-libraries.md)
- [Tutorial 12 WebAssembly components](tutorials/12-webassembly-components.md)
- [Tutorial 13 Structured concurrency](tutorials/13-structured-concurrency.md)
- [Tutorial 14 Haskell-like syntax, effects and runtime selection](tutorials/14-haskell-like-syntax-effects-and-runtime-selection.md)

## Design records

Each page records a decision, its evidence, or the work remaining for one part of the design. Titles are the pages' own; several carry dated notes and open questions rather than settled rules.

- [Allocators: a replaceable default, an allocation capability, proven budgets](design/allocators.md)
- [Array primitives](design/arrays.md)
- [Attribute annotations on the AST](design/attribute-annotations.md)
- [Base library: module scheme and buildout plan](design/base-library.md)
- [Checked bootstrap handlers](design/bootstrap-handlers.md)
- [The bootstrap ladder: implementation decisions](design/bootstrap-ladder.md)
- [Codata and guardedness design](design/codata-guardedness.md)
- [The value-level computation type former (`Comp {e} a`)](design/comp-computation-type.md)
- [Compiled interface metadata](design/compiled-interface-metadata.md)
- [Dependent core kernel design](design/dependent-core-kernel.md)
- [Effects, capabilities and sequencing design](design/effects-capabilities-sequencing.md)
- [Float refinements: NaN, infinities and the other IEEE hazards in the types](design/float-refinements.md)
- [Roadmap: Blaze-style uber-fast recompilation (P29)](design/incremental-recompilation.md)
- [Inline target IR and `prim` declarations](design/inline-target-ir.md)
- [Layouts, representation and specialization](design/layouts-and-specialization.md)
- [LLVM contract attributes: adopting the rest of the vocabulary](design/llvm-contract-attributes.md)
- [Next steps (as of 2026-11-07)](design/next-steps.md)
- [Bulletproof programs: a well-typed program does not fault](design/no-faults.md)
- [Evidence Map: Refinement/Dependent Machinery for Proven-Total Operations](design/no-faults-evidence-map.md)
- [Cinch runtime-fault inventory](design/no-faults-inventory.md)
- [Open decisions and implementation plan](design/open-decisions-and-implementation-plan.md)
- [Ownership, usage and regions design](design/ownership-and-usage.md)
- [Package management](design/package-management.md)
- [Proof automation: decision procedures and tactics](design/proof-automation.md)
- [Reclamation: precise reference counting (Perceus) for Cinch](design/reclamation.md)
- [Recursive bindings, tail calls, and surface ergonomics](design/recursive-bindings-and-tail-calls.md)
- [References and related work](design/references-and-related-work.md)
- [Refinement types: sub-kinds are refinements, not wrappers](design/refinement-types.md)
- [Executable library objects](design/runtime-object-linkage.md)
- [Safety assertion attributes (the JPL rules at compile time)](design/safety-assertion-attributes.md)
- [Self-hosting](design/self-hosting.md)
- [Standard library expansion and the bootstrap program (P30-P35)](design/stdlib-and-bootstrap.md)
- [Standard library parity: the Ruby catalog as checklist (P36-P40)](design/stdlib-parity-ruby.md)
- [Supercompilation and JIT systems](design/supercompilation-and-jit.md)
- [Verified primitives: a smaller trusted base than Lean](design/verified-primitives.md)

## Examples and context

- [Example catalogue](examples/catalogue.md)
- [Design conversation context](context/design-conversation.md)

# Goals

1. Make strong guarantees available in ordinary application and systems code, including full dependent types from the foundation.

2. Keep programming expressive: algebraic data types, higher-kinded polymorphism, pattern matching, records, modules, implicit dictionaries, and concise higher-order code.

3. Account for memory, effects, resource protocols, termination, productivity, and temporal behavior separately.

4. Let developers strengthen specifications incrementally without treating missing evidence as a theorem.

5. Let libraries extend syntax and compilation through hygienic metaprogramming and proof-checked optimization.

6. Support real compile-time work, including file reads, subprocesses, network requests, and database introspection.

7. Support hot replacement with explicit state migration, dependency tracking, and operational coordination.

8. Support platform-specific proofs and SIMD rewrites with checked target requirements and safe dispatch.

9. Support external libraries through explicit ABI, ownership, effects and trust contracts.

10. Treat WebAssembly and WASI as primary deployment targets, including components, browser integration and async interfaces.

11. Support structured concurrency, typed communication and ownership-aware parallelism, with explicit cancellation and progress contracts.

# The programming model

Ordinary evaluation is strict and proceeds in a specified order. Pure expressions are distinguished from computations that perform effects or may diverge. Infinite data uses explicit delayed observation and productivity checking; infinite services use process contracts and temporal reasoning.

Types can depend on values. A vector carries its length in its type, a connection carries the schema under which queries are valid, and an upgrade package carries a relation between old and new states. Dependent types are not limited to integer refinements or SMT-decidable predicates.

```haskell
head :: forall a n region. Ref region (Vec (n + 1) a) -> Ref region a
head xs = case xs of
  Cons x rest -> x
```

The returned reference borrows from `xs`; the elided lifetime is the input lifetime. An empty vector cannot be supplied. Pattern matching eliminates the impossible case because the constructor indices conflict.

Ownership controls mutable and external resources. Immutable values can be shared through approved representations. Sharing may use reference counting, persistent structures, or another selected runtime strategy; the exact memory-management backend remains open. Memory safety does not imply leak freedom or bounded allocation.

# Evidence at the boundary

Compilation elaborates source into a core program and checks the evidence required by its public contracts and build policy. Evidence may come from ordinary typing, branch refinement, ownership analysis, totality checking, explicit proof terms, or checked output from proof automation.

A runtime decision can produce evidence about a concrete value on its successful branch. It cannot establish an arbitrary universal claim or future liveness property. Missing proofs remain missing; tests and timeouts do not promote them to theorems.

```haskell
checkedIndex :: (n :: Nat) -> Nat -> Option (Fin n)
checkedIndex n i = case decide (i < n) of
  Yes bound -> Some (Fin i bound)
  No _ -> None
```

The proof `bound` is erased after checking. The comparison remains executable. Modules can expose weaker contracts during development and stronger ones as evidence becomes available.

# Libraries as semantic packages

A module can contain abstract types, operations, effect signatures, associated result families, semantic models, and laws. Implicit resolution selects modules where unambiguous; explicit module arguments choose alternatives.

`filter` illustrates why this matters. A finite list can return a finite list. A length-indexed vector returns an existential output length and a bound. A productive infinite stream cannot promise another productive stream without evidence of recurring matches. An effectful source must additionally preserve the chosen effect order and cleanup protocol.

Uniform operation names coexist with different result types and guarantees. Generic clients request the particular laws they use.

# Optimization and generation

Rewrite rules carry checked evidence under a named semantic relation. Definitional reduction in the type checker remains separate from optimizer rewriting. Stream fusion lowers explicit stream descriptions to state machines and loops while preserving the properties relevant to that abstraction. Target-specific laws can then justify SIMD lowering under explicit instruction, arithmetic, memory, and feature-availability assumptions.

Typed quotations carry binding contexts, effects, staging levels, and resource usage. Syntax macros are hygienic. Generators can be exported across modules and can be used within their own module when the compile-time dependency graph permits it.

Compile-time I/O is supported through capabilities. Refresh mode captures external inputs; replay mode uses captured inputs; live mode permits explicitly requested external reads. A schema snapshot justifies generated bindings relative to that snapshot. Runtime compatibility checks and a migration policy connect those assumptions to a live database.

# Safe evolution

A replacement module must refine the old published contract for existing clients. The upgrade package describes state migration, ownership transfer, protocol obligations, relevant temporal properties, and the treatment of in-flight execution.

Replaceable imports use a stable dispatch boundary. Specialized imports may be inlined or fused, so their consumers become part of the dependent update set. Old code remains resident while live frames, closures, callbacks, destructors, or other dependencies still require it.

# Foreign code, portable deployment and concurrency

Foreign interfaces separate raw ABI declarations, safe ownership wrappers and semantic contracts. Native code can be used under an audited or verified contract; isolated workers or Wasm components provide a different failure boundary. Checks on returned values establish only the properties they actually decide.

The proposed Wasm backend supports core modules, component interfaces and browser adapters. WASI 0.3 is the preferred component profile, with pinned 0.2 compatibility where needed. Imports express capabilities; dependent claims are checked at adapters, and unsupported instructions require a compatible module variant before instantiation.

Structured scopes bound task lifetimes. Bounded channels transfer ownership and expose backpressure; thread migration requires transfer evidence. Cancellation, cleanup and foreign-operation completion are distinct. STM is a proposed library for composable shared-state transactions. Temporal proofs identify their scheduler and environment assumptions.

# Reading paths

| Reader goal | Start here |
| --- | --- |
| Understand the whole design | This overview, then [Open decisions and implementation plan](design/open-decisions-and-implementation-plan.md) |
| Review language semantics | [Core language specification](specifications/core-language-specification.md) |
| Review abstraction and compilation | [Modules and compilation specification](specifications/modules-and-compilation-specification.md) |
| Review hot swapping | [Runtime evolution specification](specifications/runtime-evolution-specification.md) |
| Learn the proposed notation | [Tutorial 1 Values and dependent types](tutorials/01-values-and-dependent-types.md) through Tutorial 14, in order |
| Review concrete acceptance cases | [Example catalogue](examples/catalogue.md) and the accompanying `.cinch` files |
| Review external libraries | [Foreign function interface specification](specifications/foreign-function-interface-specification.md) |
| Review Wasm deployment | [WebAssembly and WASI specification](specifications/webassembly-and-wasi-specification.md) |
| Review concurrent execution | [Concurrency specification](specifications/concurrency-specification.md) |

# Scope and trust

The draft specifies desired guarantees and obligations; it does not claim a completed soundness proof. A small proof kernel is the target for logical evidence. The elaborator, semantic encoding, erasure, optimizer, code generator, runtime, FFI, and deployment coordinator each need a stated trust or verification story.

The specification cannot establish that a user-written contract captures the intended product behavior. It can establish that accepted code meets that contract within its explicit assumptions. Environment failures, hardware faults, timing leakage, and external services require their own models and policies.


# Syntax and runtime decision

Definitions use equations, curried application, layout and unqualified `do`. Ordinary functions expose suspension through effect rows. Runtime capabilities are lexical arguments, optionally passed implicitly. Sequencing modules carry the laws of direct binding or explicitly selected short-circuiting; no qualified `do` form is required.

See [Surface syntax and grammar](specifications/surface-syntax-and-grammar.md) and [Runtime capabilities, effects and sequencing](specifications/runtime-capabilities-effects-and-sequencing.md).

