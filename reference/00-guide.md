# Cinch compiler user's guide

This manual documents the Cinch language, compiler CLI, type and effect systems, memory model, runtime, and tooling as implemented by the **Rust compiler** (`seed/`). Reach for this guide when you need exact rules, syntax, flags, or diagnostic meanings. For a guided introduction with exercises, start with the [language tutorial](../learn/00-tutorial.md).

# Chapters

## Toolchain and workflow
- [01 · Using the compiler](01-using-the-compiler.md) — CLI commands, flags, Core dumps, cross-compilation, artifacts, and environment variables
- [18 · Artifacts, interfaces, and stores](18-artifacts-and-incremental.md) — `.cinchif` contracts, manifests, package store, and incremental APIs
- [19 · Targets, platform proofs, SIMD, and Wasm](19-targets-and-components.md) — Target triples, capability proofs, SIMD dispatch, and Wasm/WIT
- [21 · Editors and diagnostics](21-editors-and-diagnostics.md) — Reading diagnostics by compiler phase and analysis APIs
- [26 · Interactive programming with the REPL](26-repl.md) — Session commands, TUI, and project reloading
- [27 · Language server and editor integration](27-language-server.md) — `cinch lsp` protocol support and editor setup
- [28 · Documentation comments and API search](28-documentation.md) — `-- |` comments, `.cinchdoc`, `cinch doc search`, HTML docs, and doctests
- [29 · Packages, manifests, and the registry](29-package-management.md) — `.cinchpkg`, lockfiles, vendoring, publishing, and provenance

## Core language
- [02 · Lexical structure and layout](02-lexical-structure.md) — Identifiers, literals, comments, operators, and indentation layout
- [03 · Expressions, bindings, and patterns](03-expressions-and-patterns.md) — Application, `case`, guards, `let`/`letrec`, `where`, records, and lists
- [04 · Types, kinds, declarations, and inference](04-types-and-kinds.md) — Arrows, universes, quantifiers, ADTs, records, and inference limits
- [05 · Numbers, primitives, symbols, and text](05-numbers-and-primitives.md) — `Int`/`Nat`, machine scalars, overflow policies, `String`, `Bytes`, and `Sym`
- [07 · Modules, signatures, instances, and packages](07-modules-and-packages.md) — Module headers, conditional imports, exports, type classes, and coherence
- [20 · Compiler-provided vocabulary](20-compiler-vocabulary.md) — Built-in types, values, special forms, and reserved namespaces

## Safety, effects, and memory
- [06 · Refinements, narrowing, and obligations](06-refinements.md) — `refine`, `decide`, branch facts, narrowing, and mutation invalidation
- [08 · Dependent types, proofs, and erasure](08-dependent-types-and-proofs.md) — Indexed families, `Sigma`, `Prop`, `%0` erasure, `refl`/`congr`, and totality
- [09 · Effects, handlers, capabilities, and sequencing](09-effects-and-capabilities.md) — Effect rows, `Io r`, `using`/`with`, handlers, and `do` providers
- [10 · Ownership, regions, and reclamation](10-ownership-and-memory.md) — Multiplicities (`%0`, `%1`, `%aff`, `%many`), `Copy`, borrows, and reference counting
- [11 · Laziness, memoization, and codata](11-laziness-and-codata.md) — Thunks, `~` memo bindings, `Eval r`, `codata`, `Later`, and `corec`
- [13 · Structured concurrency and runtime](13-concurrency-and-runtime.md) — Scopes, `spawn`, cancellation, STM, and runtime diagnostics
- [17 · Runtime upgrades and temporal contracts](17-upgrades-and-temporal-contracts.md) — `upgrade` migrations, versioned dispatch, and temporal assertions
- [22 · Safety assertions, must-use, and LLVM contracts](22-safety-assertions-and-llvm.md) — Checked bounds, JPL profile, and LLVM attribute lowering

## Systems programming and optimization
- [12 · Rewrites, laws, and optimization](12-rewrites-and-optimization.md) — `rewrite` rules, law justification, fusion, and specialization reports
- [14 · Staging, quotation, and macros](14-staging-and-macros.md) — Compile-time `stage`, `[|...|]`, macros, and `data` imports
- [15 · FFI, representation, and inline primitives](15-ffi-layout-and-attributes.md) — `foreign import`/`export`, `#[byval]`/`#[sret]`, and `prim` bodies
- [23 · Attributes and pragmas](23-attributes-and-pragmas.md) — Complete index of compiler-owned `#[...]` attributes
- [24 · Type layout and runtime representation](24-type-layout.md) — C layout, alignment, packing, unpacking, and object headers
- [25 · Allocators and heap tuning](25-allocators.md) — Runtime memory variables, leak checking, heap caps, and planned allocators

## Status
- [16 · Current support and roadmap](16-support-and-roadmap.md) — Feature-by-feature status matrix across language, runtime, and tooling

# Finding library APIs

Use [**API Search**](/packages/search/) to query standard-library declarations by name or type signature (for example `(a -> b) -> List a -> List b` or `Result e a -> Option a`), or browse the [`stdlib` package](../library/00-index.md). Compiler-installed forms that do not live in a source module are indexed in [compiler vocabulary](20-compiler-vocabulary.md).

# Reference conventions

## Support labels

A support tag applies to the specific construct beside it, not to an entire chapter:

| Tag | Meaning |
| --- | --- |
| <span class="support-tag support-tag--current">Current</span> | Implemented end to end in the Rust compiler. |
| <span class="support-tag support-tag--partial">Partial</span> | A defined subset works; the chapter states the boundary directly. |
| <span class="support-tag support-tag--api">API only</span> | Exposed through Rust library APIs or optional build features without a standard CLI command. |
| <span class="support-tag support-tag--planned">Coming soon</span> | Specified design direction not yet implemented. |

## Executable examples

Complete modules in `cinch` fences are checked and executed against the Rust compiler by `website/scripts/verify-doc-examples.py` using HTML comment markers immediately before each fence:

| Marker | Assertion |
| --- | --- |
| `<!-- cinch:run exit=N [stdout="..."] -->` | Checks, builds, and runs the program, asserting exit status and optional exact stdout (JSON-encoded string). |
| `<!-- cinch:check -->` | Passes `cinch check` without linking (used for declarations such as external `foreign` symbols). |
| `<!-- cinch:reject contains="..." -->` | Fails `cinch check` with exit code 1 and stderr containing the quoted text. |
| `<!-- cinch:project group=NAME file=FILE -->` | Multi-file project sharing `group`; `exit=` and `stdout=` sit on the entry file. |

Unmarked `cinch` blocks are isolated syntax fragments, not standalone modules. (`cinch test` fences are a separate mechanism for library documentation comments; see [Chapter 28](28-documentation.md).)

## Reading diagnostics

Compiler diagnostics follow three formats depending on which stage rejected the program:

```text
cinch: Main.cinch: 3:1: expected `where`, found identifier `main`   -- lexer, layout, parser
cinch: 4:8: type mismatch: expected String, got Int (in Main.cinch) -- checker and later stages
cinch: Main.cinch: No such file or directory (os error 2)           -- driver, linker, store
```

Generated placeholder IDs inside diagnostics — regions (`r#845`), metavariables (`?i550`, `?r895`), and memo placeholders (`?m2015`) — are numbered per compilation, so only the text around the digits stays fixed across runs.

## Signature notation

Lowercase letters (`a`, `b`, `e`) in signatures are type or row variables; `r` denotes a runtime or region identity.
