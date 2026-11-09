# Learn Cinch

A complete, ordered introduction to programming in Cinch with the **Rust compiler**. Start with a native executable, learn the ordinary language, then add effects, ownership, proofs, and advanced runtime features.

[Start chapter 1: build and run a program](01-first-program.md)

# Prerequisites

A terminal and a text editor, plus a Rust toolchain with Cargo to build the compiler and a compatible `clang` on `PATH` for native builds and runs. No Haskell or type theory background is assumed. [Chapter 1](01-first-program.md) builds the compiler and runs the first program.

# How to use the book

Work through the first ten chapters in order. Save each complete example as `Main.cinch` in a fresh directory unless the chapter gives a multi-file project. Commands assume you built the Rust compiler and put its executable on `PATH`. Runnable examples state the expected output or exit status, and a deliberately rejected program states the diagnostic it produces. A block that is not a complete module is labelled as a fragment and cannot be run on its own.

Each chapter explains the new syntax and walks through its meaning. Chapters 1 through 21 each end with an exercise and its answer. Chapters 11 to 21 extend the same foundations into the advanced language. Chapter 22 combines the basics in a complete order calculator and suggests incremental extensions.

<span class="support-tag support-tag--planned">Coming soon</span> marks specified behaviour that is not available yet. A nearby explanation states the current boundary and intended behaviour. The [support matrix](../reference/16-support-and-roadmap.md) collects these distinctions. Planned APIs are not prerequisites for any runnable example.

# If you already know Haskell

Cinch shares curried functions, algebraic data declarations, pattern matching, parameterized signatures with instances, and `do` notation, so parts of the book will read quickly. The differences are what the chapters spend their examples on.

| Familiar ground | Chapter | What differs in Cinch |
| --- | --- | --- |
| Curried functions and arrow types | [3](03-functions.md) | Top-level polymorphism is written with explicit quantifiers, and local helpers go in `where`. |
| `data` declarations and pattern matching | [4](04-data-and-matching.md) | Coverage is checked over constructors, record fields are projected with `item.price`, and record-update syntax is not supported. |
| Type classes | [9](09-signatures-and-instances.md) | A `signature` plus `instance` supplies dictionaries, constraints are written with `using`, and superclass entailment is not implicit. |
| `do` notation | [11](11-handlers-and-sequencing.md) | One lexical sequencing provider is selected per definition, and the effect row appears in the type. |
| Lazy evaluation and infinite lists | [2](02-values-and-layout.md), [15](15-streams-and-producers.md), [16](16-explicit-laziness.md) | Ordinary evaluation is strict. You delay work with a thunk or memo binding, and an infinite sequence must be productive at each observation. |

Start at [chapter 2](02-values-and-layout.md) for literal, layout, and application syntax. Chapters [11](11-handlers-and-sequencing.md) through [14](14-dependent-types-and-proofs.md) cover handlers, multiplicities and regions, refinement evidence, and dependent indices, which have no direct Haskell counterpart, so read them as new material rather than as translation.

# Part I: write and understand programs

| Chapter | What it covers |
| --- | --- |
| [1. Build and run your first program](01-first-program.md) | Build the compiler, save a complete program, run it, and see a type error on purpose. |
| [2. Values, expressions, and layout](02-values-and-layout.md) | Types of values and literals, indentation blocks, function application, and operators. |
| [3. Functions, types, and local definitions](03-functions.md) | Arrow types, curried application, explicit top-level quantifiers, and `where` helpers. |
| [4. Model data and handle every case](04-data-and-matching.md) | Data declarations, products, records with named fields, coverage, and pattern order. |
| [5. Recursion, lists, and maps](05-recursion-and-collections.md) | Take a list apart, transform and fold it, write tail recursion, and choose an ordered map. |
| [6. Missing values, errors, and validation](06-options-results-and-validation.md) | `Option`, `Result` with named errors, chained checks, and keeping expected failure away from faults. |
| [7. Numbers, text, and bytes](07-numbers-text-and-bytes.md) | Mathematical and machine integers, Unicode text and `Bytes` buffers, and formatting as a conversion. |
| [8. Split a program into modules](08-modules.md) | A two-module project, name lookup and collisions, the implicit Prelude, and explicit library imports. |
| [9. Signatures, instances, and reusable operations](09-signatures-and-instances.md) | Declare a `signature`, write instances, use library constraints, pass module parameters, and read a law's status. |
| [10. Effects, capabilities, and a program that prints](10-effects-and-io.md) | Print a value, sequence steps with `do`, and state the effects a definition performs. |

# Part II: effects, resources, and evidence

| Chapter | What it covers |
| --- | --- |
| [11. Handle effects and select sequencing](11-handlers-and-sequencing.md) | Declare an operation, write its handler, read the current handler boundary, and select a `Result` do provider. |
| [12. Ownership, borrowing, and regions](12-ownership-and-regions.md) | Read a multiplicity annotation, borrow for inspection, and see why mutation invalidates evidence. |
| [13. Refine values and make unsafe cases explicit](13-refinements-and-safe-operations.md) | Build a bounded refinement, divide by a `NonZero` divisor, and use the refinements the compiler ships. |
| [14. Dependent types, finite indices, and proofs](14-dependent-types-and-proofs.md) | Check an index before constructing it, keep a vector's length, package an unknown length, and compute with proofs. |
| [15. Productive streams and partial producers](15-streams-and-producers.md) | Build codata one layer at a time, see what mapping and filtering preserve, and bisimulate two streams. |
| [16. Delay work and memoize deliberately](16-explicit-laziness.md) | Delay a function body with a thunk, share a result with a memo binding, and match the tool to the lifetime. |
| [17. Structured tasks and cancellation](17-structured-concurrency.md) | Spawn and join a pair, treat join as a failure policy, and read capture modes on handles. |

# Part III: compilation, targets, and a complete program

| Chapter | What it covers |
| --- | --- |
| [18. Compile-time code and hygienic macros](18-staging-and-macros.md) | Generate a declaration, record compile-time inputs, and separate typed code from declaration syntax. |
| [19. Laws, rewrites, and performance](19-rewrites-and-performance.md) | State a `rewrite` equation, find the conditions on a pure transformation, use attribute hints, and measure the work. |
| [20. Foreign code, layouts, and target profiles](20-foreign-code-and-targets.md) | Declare a foreign boundary, handle imports, exports, and blocking, and read target evidence. |
| [21. State evolution and temporal contracts](21-upgrades-and-contracts.md) | Write a migration, version active code, and state the scope of a temporal claim. |
| [22. Build a small order calculator](22-putting-it-together.md) | Requirements, a complete implementation, boundary tests, and extensions in stages. |

# Keep a reference nearby

- [Compiler user's guide](../reference/00-guide.md): precise syntax, checking rules, commands, artifacts, and runtime controls.
- [Built-in and shipped libraries](../library/00-index.md): real module APIs extracted from checked interfaces, with distribution and load instructions.
- [Compiler-provided vocabulary](../reference/20-compiler-vocabulary.md): special forms and operations installed by the checker.
- [Original design overview](../overview.md): specifications, historical tutorials, and design background.

The design archive preserves intended semantics and earlier decisions. For current accepted behaviour, this book and the guide follow the Rust implementation and its executable tests. The Cinch-written bootstrap compiler has its own progress record.
