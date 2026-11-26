# Cinch design documentation

Architecture and design documents for the Cinch language, compilers (`seed/` and `compiler/`), runtime, and standard library.

## How the repository's docs fit together

| Directory | Role |
| --- | --- |
| [`learn/`](../learn/00-tutorial.md) | 22-chapter language tutorial (published at `/learn/`) |
| [`reference/`](../reference/00-guide.md) | 29-chapter compiler user's guide & support matrix (published at `/reference/`) |
| [`library/`](../library/00-index.md) | Generated API reference for the standard library (published at `/packages/stdlib/`) |
| [`specifications/`](../specifications/) | Normative language, grammar, FFI, concurrency, and runtime specifications |
| `design/` (this directory) | Architecture decisions, compiler internals, runtime designs, and roadmap |

## Architecture index

### Language & type system
- [**Type system & dependent core**](type-system.md) — Stratified universes, dependent functions/pairs, indexed families, mutual data groups and SCC positivity, refinement types and implication/linear solvers, IEEE float refinements, Rank-N polymorphism, codata/guardedness, and proof automation.
- [**Ownership, reclamation & allocators**](ownership-and-reclamation.md) — Usage multiplicities (`%0`/`%1`/`%aff`/`%many`), regions and borrows (`Ref`/`Mut`), Perceus precise reference counting, self-describing block headers (ABI 10), stack allocation of non-escaping blocks, arenas, and proven allocation budgets.
- [**Effects, concurrency & parallelism**](effects-and-concurrency.md) — Row-typed effects, handlers, lexical capabilities, sequencing modules, `Comp {e} a`, draft effect policies, green-thread fiber scheduler, native `MVar`, TL2 STM, pure parallelism, and data-parallel arrays / Metal GPU execution.
- [**Surface syntax & attributes**](surface-and-attributes.md) — AST attribute annotations (`#[...]`) and surface ergonomics (`letrec`, tail calls, identifiers).

### Compiler, IR & code generation
- [**Layouts, optimization & code generation**](layouts-and-codegen.md) — Representation classes (`Raw`, `Obj`, `TObj`, `Wide`), narrow boundary crossings, unboxed `I128`/`U128`, CPS-to-direct compilation (`tailcc`, contification, call-pattern specialization, constructed product results), inlining, inline target IR (`prim`), LLVM contract attributes, debug/unwind info, and JIT/supercompilation.
- [**Textual Core format**](core-output.md) — Annotated, verifiable textual Core IR (`cinch core emit|verify|diff`), stage mappings, editor support, and normative Core text grammar.
- [**Compilation, interfaces & performance**](compilation-and-interfaces.md) — Compiled interfaces (`.cinchif`, `.cpk`, schema 10 / ABI 10), standalone and package library objects, incremental/modular builds, IR sharding, PGO, and compilation-speed architecture.
- [**Self-hosting & bootstrap**](self-hosting.md) — Four-stage bootstrap ladder, pinned Stage 0 and library provider, fixed-point verification, and numbered acceptance criteria.

### Safety, verification & ecosystem
- [**Safety, no-faults & verification**](safety-and-verification.md) — Bulletproof/no-faults execution model, resource exhaustion boundaries, compile-time safety assertions (JPL profile), verified primitives, and formal verification roadmap.
- [**Package management & registry**](package-management.md) — Content-addressed release identity (`package_id`), checked interface compatibility, lockfiles, base-library package split, official packages, registry protocol, and SLSA provenance.
- [**Standard library architecture**](stdlib-architecture.md) — Arrays, mutable containers, datetime, JSON representation, and Go standard-library parity tracking (`stdlib-surface.json`).
- [**Roadmap & open decisions**](roadmap.md) — Active engineering priorities and unsettled design questions.
- [**References & related work**](references-and-related-work.md) — Primary literature and system inspirations.
