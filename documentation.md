# Cinch documentation

Learn to write Cinch programs, look up the compiler's rules, and find the real library APIs. The tutorial and reference follow the **Rust compiler**. Specified features still being built are marked <span class="support-tag support-tag--planned">Coming soon</span>, with the current boundary explained beside them.

# Start learning

[**Read the language tutorial →**](learn/00-tutorial.md)

A complete course from building a first executable to typed functions, data modelling, errors, collections, modules, effects, ownership, refinements, and dependent proofs. Continue into streams, structured tasks, macros, foreign code, and state evolution, then build an order calculator. Chapters include worked programs, expected results, exercises, and answers.

| Start here | You will learn |
| --- | --- |
| [Your first program](learn/01-first-program.md) | Build the Rust compiler, check source, and run an executable. |
| [Functions and data](learn/03-functions.md) | Read types, pass functions, model alternatives, and handle cases. |
| [Effects and I/O](learn/10-effects-and-io.md) | Print output and understand capabilities and effect rows. |
| [Refinements and proofs](learn/13-refinements-and-safe-operations.md) | Check dynamic inputs and retain evidence for safe operations. |
| [Complete project](learn/22-putting-it-together.md) | Combine pure calculations, explicit errors, and I/O. |

# Look up a language or compiler rule

[**Open the compiler user's guide →**](reference/00-guide.md)

Precise coverage of commands, syntax, types, instances, modules, effects, ownership, proofs, laziness, code generation, target profiles, runtime controls, and artifacts. Each chapter separates today's implementation from the intended language.

| Task | Reference |
| --- | --- |
| Build or run a project | [Commands and installation](reference/01-using-the-compiler.md) |
| Understand syntax or a type | [Expressions](reference/03-expressions-and-patterns.md), [types and kinds](reference/04-types-and-kinds.md) |
| Resolve a bounds/proof error | [Refinements and obligations](reference/06-refinements.md) |
| Use tasks, I/O, or memory controls | [Runtime](reference/13-concurrency-and-runtime.md), [ownership and memory](reference/10-ownership-and-memory.md) |
| Look up a `#[...]` pragma | [Complete attribute index](reference/23-attributes-and-pragmas.md) |
| Understand a type’s memory layout | [Type layout and representations](reference/24-type-layout.md) |
| Choose or customize an allocator | [Allocator design and heap customization](reference/25-allocators.md) |
| Find a special compiler name | [Compiler-provided vocabulary](reference/20-compiler-vocabulary.md) |
| Check whether a feature works yet | [Current support and Coming soon](reference/16-support-and-roadmap.md) |
| Configure an editor or diagnose a failure | [Editors and diagnostics](reference/21-editors-and-diagnostics.md) |
| Explore programs interactively | [REPL guide — Coming soon](reference/26-repl.md) |
| Set up compiler assistance in an editor | [Language server — complete experience Coming soon](reference/27-language-server.md) |

# Find an operation

[**Browse built-in and shipped libraries →**](library/00-index.md)

The actual library surface extracted from checked Rust compiler interfaces: exported values and signatures, types and constructors, methods, fixities, and instances. Each module identifies whether it is embedded or needs source files alongside the project, and includes source documentation and dependencies.

The [package catalogue](/packages/) and [downloads](/downloads/) currently contain labelled prototype/sample data. They are useful interface previews; use source builds and this library directory for working programs.

# Read the design

[**Language design overview and archive →**](overview.md)

The original specifications, advanced design tutorials, examples, and decision notes remain available at their existing URLs. They describe the intended language and its history. Their earlier implementation-status notes can lag the compiler. Use the [support matrix](reference/16-support-and-roadmap.md) to connect them to current behaviour and planned work.

This organization takes its learning/reference cues from [Haskell's documentation directory](https://www.haskell.org/documentation/) and the [GHC user's guide](https://downloads.haskell.org/ghc/latest/docs/users_guide/), while teaching Cinch's own syntax and semantics.
