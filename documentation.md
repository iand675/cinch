# Cinch documentation

Cinch is a strict functional language with dependent and refinement types, row-typed effects, linear ownership, and native LLVM compilation without a garbage collector. Every chapter below tracks the **Rust compiler** (`seed/`). Features still under construction are tagged <span class="support-tag support-tag--planned">Coming soon</span> alongside the current boundary.

# Start learning

[**Language tutorial →**](learn/00-tutorial.md)

Twenty-four chapters taking you from a first native binary through functions, algebraic data, explicit errors, modules, effects, ownership, refinements, dependent proofs, streams, structured concurrency, macros, FFI, hot upgrades, built-in state-machine model checking, and multi-component protocol composition. Chapters 1–21, 23, and 24 end with exercises and solutions; complete examples carry executable compiler verification markers.

| Topic | Chapters |
| --- | --- |
| Build, check, and run | [First program](learn/01-first-program.md) |
| Functions and algebraic data | [Functions](learn/03-functions.md), [Data and matching](learn/04-data-and-matching.md) |
| Effects and capabilities | [Effects and I/O](learn/10-effects-and-io.md) |
| Refinements and proofs | [Refinements](learn/13-refinements-and-safe-operations.md), [Dependent types and proofs](learn/14-dependent-types-and-proofs.md) |
| State machines & model checking | [State machines and model checking](learn/23-state-machines-and-model-checking.md), [Composing and debugging models](learn/24-composing-state-machines.md) |
| End-to-end project | [Putting it together](learn/22-putting-it-together.md) |

# Compiler user's guide

[**Compiler user's guide →**](reference/00-guide.md)

The complete reference manual in 30 chapters: CLI commands, lexical and expression syntax, types and kinds, primitives, refinements, modules, dependent proofs, effects, ownership, laziness, optimization, concurrency, staging, FFI, attributes, memory layout, allocators, REPL, LSP, documentation tools, package management, and a TLA+ guide with checkable value domains.

| If you need to… | Open |
| --- | --- |
| Build, run, cross-compile, or inspect Core/IR | [Using the compiler](reference/01-using-the-compiler.md) |
| Check syntax, precedence, or type rules | [Expressions and patterns](reference/03-expressions-and-patterns.md), [Types and kinds](reference/04-types-and-kinds.md) |
| Discharge a refinement or proof obligation | [Refinements](reference/06-refinements.md), [Dependent types and proofs](reference/08-dependent-types-and-proofs.md) |
| Work with effects, tasks, or memory limits | [Effects and capabilities](reference/09-effects-and-capabilities.md), [Ownership and memory](reference/10-ownership-and-memory.md), [Concurrency and runtime](reference/13-concurrency-and-runtime.md) |
| Look up a `#[...]` attribute or memory layout | [Attributes and pragmas](reference/23-attributes-and-pragmas.md), [Type layout](reference/24-type-layout.md), [Allocators](reference/25-allocators.md) |
| Find compiler-installed built-ins | [Compiler vocabulary](reference/20-compiler-vocabulary.md) |
| Use the REPL, LSP, docs, or packages | [REPL](reference/26-repl.md), [Language server](reference/27-language-server.md), [Documentation tools](reference/28-documentation.md), [Package management](reference/29-package-management.md) |
| Check feature support status | [Support matrix](reference/16-support-and-roadmap.md) |

# Standard library (`stdlib`) & API search

[**Search APIs by name or type signature →**](/packages/search/) · [**Browse `stdlib` modules →**](library/00-index.md)

Every standard library module is published under [`/packages/stdlib/`](/packages/stdlib/) from checked compiler interfaces (`.cinchif`) with pretty-printed, cross-linked type signatures, collapsible synopses, cross-module instance lists, and line-numbered source views. Use [**API Search**](/packages/search/) for Hoogle-style queries such as `(a -> b) -> List a -> List b`, `Result e a -> Option a`, `+Data.Map`, or `is:class`.

- [**Packages**](/packages/) — Registry-backed package catalogue (or the latest cached snapshot when offline).
- [**Downloads**](/downloads/) — Nightly standalone Stage 3 compiler binaries for Linux (x86-64, ARM64) and macOS (Apple Silicon), with SHA-256 verification.

# Find by task

| Task | Shortest path | Deeper reference |
| --- | --- | --- |
| Build and run a program | [Using the compiler](reference/01-using-the-compiler.md#commands) | [First program](learn/01-first-program.md) |
| Print output | [Effects and I/O](learn/10-effects-and-io.md) | [Compiler vocabulary](reference/20-compiler-vocabulary.md) |
| Model typed failures | [Options, results, and validation](learn/06-options-results-and-validation.md) | [Data.Result](library/Data.Result.md) |
| Refine a runtime value | [Refinements](learn/13-refinements-and-safe-operations.md) | [Refinements reference](reference/06-refinements.md) |
| Model-check a protocol or state machine | [State machines and model checking](learn/23-state-machines-and-model-checking.md), [Composing and debugging models](learn/24-composing-state-machines.md) | [Upgrades and temporal contracts](reference/17-upgrades-and-temporal-contracts.md), [TLA+ guide & value domains](reference/30-tla-plus-guide-and-value-domains.md) |
| Split a project into modules | [Modules](learn/08-modules.md) | [Modules and packages](reference/07-modules-and-packages.md) |
| Publish or verify a package | [Package management](reference/29-package-management.md) | [Artifacts and incremental builds](reference/18-artifacts-and-incremental.md) |
| Inspect Core or LLVM IR | [Using the compiler](reference/01-using-the-compiler.md#core-output) | [Rewrites and optimization](reference/12-rewrites-and-optimization.md), [Type layout](reference/24-type-layout.md) |
