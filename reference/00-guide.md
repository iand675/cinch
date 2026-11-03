# Cinch compiler user's guide

The language, commands, built-in vocabulary, libraries, and runtime supported by the **Rust compiler**. Use this guide to answer “what does this construct mean?”, “what does the compiler require?”, and “how do I build or diagnose this program?” For a first introduction, start with the [language tutorial](../learn/00-tutorial.md).

# Current and intended behaviour

<span class="support-tag support-tag--planned">Coming soon</span> identifies specified behaviour that is not implemented yet. **Partial** identifies a supported subset, and **API only** identifies mechanisms without a user-facing CLI workflow. Planned syntax is explained beside its current boundary. See the [complete support matrix](16-support-and-roadmap.md).

The Rust implementation is the present authority. This guide does not use the self-hosted bootstrap compiler's remaining gaps to understate Rust support, or use design aspirations to overstate current behaviour.

# Contents

| Chapter |
| --- |
| [Using the Rust compiler](01-using-the-compiler.md) |
| [Lexical structure and layout](02-lexical-structure.md) |
| [Expressions, bindings, and patterns](03-expressions-and-patterns.md) |
| [Types, kinds, declarations, and inference](04-types-and-kinds.md) |
| [Numbers, machine primitives, symbols, and text](05-numbers-and-primitives.md) |
| [Refinements, narrowing, and proof obligations](06-refinements.md) |
| [Modules, signatures, instances, and packages](07-modules-and-packages.md) |
| [Dependent types, the logical fragment, and evidence](08-dependent-types-and-proofs.md) |
| [Effects, handlers, capabilities, and sequencing](09-effects-and-capabilities.md) |
| [Ownership, regions, and memory reclamation](10-ownership-and-memory.md) |
| [Explicit laziness, memoization, and codata](11-laziness-and-codata.md) |
| [Rewrites, laws, specialization, and optimizer reports](12-rewrites-and-optimization.md) |
| [Structured concurrency and runtime operation](13-concurrency-and-runtime.md) |
| [Staging, quotation, generation, and data imports](14-staging-and-macros.md) |
| [Foreign code, representation attributes, and inline primitives](15-ffi-layout-and-attributes.md) |
| [Current support and what is coming soon](16-support-and-roadmap.md) |
| [Runtime upgrades and temporal contracts](17-upgrades-and-temporal-contracts.md) |
| [Artifacts, interfaces, stores, and incremental compilation](18-artifacts-and-incremental.md) |
| [Target profiles, platform proofs, SIMD, and Wasm](19-targets-and-components.md) |
| [Compiler-provided vocabulary and special forms](20-compiler-vocabulary.md) |
| [Editors, analysis APIs, and diagnostics](21-editors-and-diagnostics.md) |
| [Safety assertions, must-use results, and LLVM contracts](22-safety-assertions-and-llvm.md) |
| [Attributes and pragmas: the complete reference](23-attributes-and-pragmas.md) |
| [Type layout and runtime representation](24-type-layout.md) |
| [Allocators and heap customization](25-allocators.md) |
| [Interactive programming with the REPL — Coming soon](26-repl.md) |
| [Language server and editor integration — intended complete experience](27-language-server.md) |

# Find a specific operation

The [library directory](../library/00-index.md) contains the actual embedded and source-distributed APIs, including public types, constructors, class methods, values, fixities, and instances. [Compiler-provided vocabulary](20-compiler-vocabulary.md) covers special forms installed directly by the checker. Site search indexes these pages; the separate package catalogue still contains labelled sample data.

# Reference conventions

`a`, `b`, and `e` in signatures are parameters, not literal type names. `r` commonly denotes a runtime/region identity. Declaration fragments need their enclosing module and imports. A complete runnable example states its expected exit/output. An implementation-source pointer names the owning file in the repository, and a test-suite pointer names a behaviour suite rather than claiming that the whole suite was rerun when this page was written.
