# Cinch compiler user's guide

The language, commands, built-in vocabulary, libraries, and runtime supported by the **Rust compiler**. Use this guide to answer "what does this construct mean?", "what does the compiler require?", and "how do I build or diagnose this program?". For a first introduction, start with the [language tutorial](../learn/00-tutorial.md).

# Current and intended behaviour

<span class="support-tag support-tag--planned">Coming soon</span> identifies specified behaviour that is not implemented yet. **Partial** identifies a supported subset, and **API only** identifies mechanisms without a user-facing CLI workflow. Planned syntax is explained beside its current boundary. [Reference conventions](#reference-conventions) defines the four labels, the example markers, and the diagnostic shape; the [complete support matrix](16-support-and-roadmap.md) covers the language area by area.

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
| [Interactive programming with the REPL](26-repl.md) |
| [Language server and editor integration](27-language-server.md) |
| [Documentation comments and symbol lookup](28-documentation.md) |
| [Packages, manifests, and the registry](29-package-management.md) |

# Find a specific operation

The [library directory](../library/00-index.md) contains the actual embedded and source-distributed APIs, including public types, constructors, class methods, values, fixities, and instances. [Compiler-provided vocabulary](20-compiler-vocabulary.md) covers special forms installed directly by the checker. Site search indexes these pages; the separate package catalogue is generated at build time from a live registry, or from an explicit offline marker.

# Reference conventions

## Examples

A complete module in a `cinch` fence carries a marker comment on the line immediately before the fence. `website/scripts/verify-doc-examples.py` runs every marked example against the Rust compiler, and the markers below are the whole vocabulary.

| Marker | What it asserts |
| --- | --- |
| `<!-- cinch:run exit=0 -->` | The module passes `cinch check`, builds, and the executable exits with that status. Add `stdout="exact text"` to compare standard output as well. The value decodes as a JSON string, so `stdout="38\n"` means `38` followed by a newline and `stdout=""` means no output. |
| `<!-- cinch:check -->` | The module must pass `cinch check`, with no build and no execution. Use it for a program that is worth checking but cannot link here, such as a `foreign` import whose symbol this repository does not provide. |
| `<!-- cinch:reject contains="text" -->` | `cinch check` exits 1 and its standard error contains that substring. |
| `<!-- cinch:project group=NAME file=Main.cinch -->` | One marker per file of a multi-file project, all sharing the group name. `exit=` and `stdout=` describe the whole group and apply to the entry program; put them on the entry file's marker. Both default to exit 0 with no output comparison. |

A `cinch` block with no marker is a fragment. A fragment is not a program: it shows a declaration or expression in the context the surrounding prose names, and it is never a complete module. Fragments never start with `module`. Save an example as `Main.cinch` in its own directory and run the commands from that directory.

Fences tagged `cinch test` are a separate mechanism: they belong to the module documentation pipeline described in [documentation comments](28-documentation.md), which `cinch doc test` executes. They are not guide examples.

Every diagnostic quoted in this guide is copied from real compiler output. Three shapes appear, one per stage that can fail, and each keeps the position information its stage has. Lexical, layout, and parse failures name the file first; checker failures put the position first with the file in a trailing parenthetical; driver, linker, and store failures carry no position and name the path inside the message.

```text
cinch: Main.cinch: 3:1: expected `where`, found identifier `main`
cinch: 4:8: type mismatch: expected String, got Int (in Main.cinch)
cinch: Main.cinch: No such file or directory (os error 2)
```

A page that quotes several messages therefore shows more than one ordering, and the ordering tells you which stage refused the program. Generated identifiers inside a message are per-compilation: a region or runtime identity such as `r#845`, a metavariable such as `?i550` or `?r895`, and a task or memo placeholder such as `?m2015` are numbered afresh each run, so your message will show the same text with different digits. A `run` invocation can also exit 1 because the program returned that status, so run `check` separately when you need to distinguish a rejected program from a failing one.

## Support labels

| Label | Meaning |
| --- | --- |
| <span class="support-tag support-tag--current">Current</span> | The Rust compiler implements it, and the chapter describes the admitted surface and its constraints. |
| <span class="support-tag support-tag--partial">Partial</span> | Some of the specified feature works; the chapter states the working part and the part that does not. |
| <span class="support-tag support-tag--api">API only</span> | Reachable through Rust compiler or runtime APIs, with no end-user CLI workflow. |
| <span class="support-tag support-tag--planned">Coming soon</span> | Intended or selected design direction that is not usable yet. Not a release promise. |

A label applies to the named behaviour, not to the whole page. One chapter can carry a working runtime mechanism beside a planned command-line workflow, and each span marks which is which.

## Names and signatures

`a`, `b`, and `e` in signatures are parameters, not literal type names. `r` commonly denotes a runtime or region identity. A declaration fragment needs its enclosing module and its imports, so its signature alone is not something to paste into a file. A complete runnable example states its expected exit status or output. An implementation-source pointer names the owning file in the repository, and a test-suite pointer names a behaviour suite; neither claims that the whole suite was rerun when the page was written.
