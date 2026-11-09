# Cinch documentation

Learn to write Cinch programs, look up the compiler's rules, and find the real library APIs. The tutorial and reference follow the **Rust compiler**. Specified features still being built are marked <span class="support-tag support-tag--planned">Coming soon</span>, with the current boundary explained beside them.

# Start learning

[**Read the language tutorial →**](learn/00-tutorial.md)

A complete course in 22 chapters, from building the compiler and running a first executable to typed functions, data modelling, errors, collections, modules, effects, ownership, refinements, and dependent proofs. The later chapters cover streams, structured tasks, macros, foreign code, and state evolution, then build an order calculator. Chapters 1 through 21 each end with an exercise and its answer, and complete examples state the output or exit status they produce. No Haskell or type theory background is assumed.

| Start here | You will learn |
| --- | --- |
| [Your first program](learn/01-first-program.md) | Build the Rust compiler, check source, run an executable, and read a type error. |
| [Functions](learn/03-functions.md) and [data](learn/04-data-and-matching.md) | Read arrow types, pass functions, model alternatives and records, and handle every case. |
| [Effects and I/O](learn/10-effects-and-io.md) | Print output and understand capabilities and effect rows. |
| [Refinements](learn/13-refinements-and-safe-operations.md) and [proofs](learn/14-dependent-types-and-proofs.md) | Check dynamic inputs, retain evidence for safe operations, and use length-indexed data. |
| [Complete project](learn/22-putting-it-together.md) | Combine pure calculations, explicit errors, and I/O, then extend the program in stages. |

# Look up a language or compiler rule

[**Open the compiler user's guide →**](reference/00-guide.md)

The reference in 29 chapters: commands, lexical structure, expressions and patterns, types and kinds, numbers and primitives, refinements, modules and packages, dependent proofs, effects, ownership and memory, laziness, rewrites, concurrency, staging, FFI and attributes, support status, upgrades, artifacts, targets and Wasm, compiler vocabulary, editors and diagnostics, safety assertions, the attribute index, type layout, allocators, the REPL, the language server, documentation comments, and package management. Support labels mark where a described mechanism is Partial, API only, or Coming soon.

| Task | Reference |
| --- | --- |
| Build or run a project | [Commands and installation](reference/01-using-the-compiler.md) |
| Understand syntax or a type | [Expressions](reference/03-expressions-and-patterns.md), [types and kinds](reference/04-types-and-kinds.md) |
| Resolve a bounds or proof error | [Refinements and obligations](reference/06-refinements.md) |
| Use tasks, I/O, or memory controls | [Runtime](reference/13-concurrency-and-runtime.md), [ownership and memory](reference/10-ownership-and-memory.md) |
| Look up a `#[...]` pragma | [Complete attribute index](reference/23-attributes-and-pragmas.md) |
| Write or look up API documentation | [Documentation comments and symbol lookup](reference/28-documentation.md) |
| Understand a type's memory layout | [Type layout and representations](reference/24-type-layout.md) |
| Choose or customize an allocator | [Allocator design and heap customization](reference/25-allocators.md) |
| Find a special compiler name | [Compiler-provided vocabulary](reference/20-compiler-vocabulary.md) |
| Check whether a feature works yet | [Current support and Coming soon](reference/16-support-and-roadmap.md) |
| Configure an editor or diagnose a failure | [Editors and diagnostics](reference/21-editors-and-diagnostics.md) |
| Explore programs interactively | [REPL guide](reference/26-repl.md) |
| Understand packages, manifests, or the registry | [Packages, manifests, and the registry](reference/29-package-management.md) |
| Set up compiler assistance in an editor | [Language server](reference/27-language-server.md), whose packaged setup is <span class="support-tag support-tag--planned">Coming soon</span> |

# Find by task

Each row names the page to open and the heading in it. The second column is the shortest path; the third holds the fuller or alternative account.

| Task | Start here | Also read |
| --- | --- | --- |
| Build and run a program | [Using the Rust compiler](reference/01-using-the-compiler.md), `Commands` | [First program](learn/01-first-program.md), `A complete program` |
| Print output | [Chapter 10](learn/10-effects-and-io.md), `A program that prints` | [Compiler vocabulary](reference/20-compiler-vocabulary.md), `Installed values` |
| Model an error | [Chapter 6](learn/06-options-results-and-validation.md), `Result says a computation can fail with a reason` | [Data.Result](library/Data.Result.md) for `map-result`, `map-err`, and `partition-results` |
| Check a value at runtime | [Chapter 13](learn/13-refinements-and-safe-operations.md), `Evidence from a decision at runtime` | [Refinements](reference/06-refinements.md), `Constructing a refined value` |
| Split a project into modules | [Chapter 8](learn/08-modules.md), `A module is a name and a surface` | [Modules](reference/07-modules-and-packages.md), `Module headers` |
| Publish a package | [Packages](reference/29-package-management.md), `Publishing` | The same page's `Command surface` lists every `cinch pkg` form |
| Read emitted IR | [Using the Rust compiler](reference/01-using-the-compiler.md), `Commands` (the `emit-ir` row) | [Rewrites and optimization](reference/12-rewrites-and-optimization.md), `Reading the optimizer report`; [type layout](reference/24-type-layout.md), `Inspect the compiler's layout facts` |

# Find an operation

[**Browse built-in and shipped libraries →**](library/00-index.md)

The library surface extracted from checked compiler interfaces: exported values and signatures, types and constructors, methods, fixities, and instances. The directory covers 97 modules, 55 embedded in the compiler and 42 shipped as source files that you copy beside a project. Each page names its imports under `Dependencies` when it declares any, ship-source pages list their complete non-embedded closure, and the index has a table from job to declaration.

The [package catalogue](/packages/) lists releases from a registry, or the last cached snapshot when a live build could not reach one. The [downloads page](/downloads/) serves labelled sample bundles that do not install a compiler; build from source for a working toolchain.

# Read the design

[**Language design overview and archive →**](overview.md)

The original specifications, advanced design tutorials, examples, and decision records remain available at their existing URLs. They describe the intended language and its history. Their earlier implementation-status notes can lag the compiler, and some pages predate later work, so use the [support matrix](reference/16-support-and-roadmap.md) to connect them to current behaviour and planned work.

This organization takes its learning and reference cues from [Haskell's documentation directory](https://www.haskell.org/documentation/) and the [GHC user's guide](https://downloads.haskell.org/ghc/latest/docs/users_guide/), while teaching Cinch's own syntax and semantics.
