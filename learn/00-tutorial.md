# Learn Cinch

A complete, ordered introduction to programming in Cinch with the **Rust compiler**. Start with a native executable, learn the ordinary language, then add effects, ownership, proofs, and advanced runtime features. No Haskell knowledge is required.

[Start chapter 1: build and run a program](01-first-program.md)

# How to use the book

Work through the first ten chapters in order. Save each complete example as `Main.cinch` in a fresh directory unless the chapter gives a multi-file project. Commands assume that you built the Rust compiler and placed its executable on `PATH`. Runnable examples state the expected output or exit status; deliberately invalid examples explain the required diagnostic. Short declaration fragments are labelled as fragments and are not standalone programs.

Each chapter explains the new syntax, walks through its meaning, and includes an exercise with an answer. Chapters 11–21 extend the same foundations into the advanced language. Chapter 22 combines the basics in a complete order calculator and suggests incremental extensions.

<span class="support-tag support-tag--planned">Coming soon</span> marks specified behaviour that is not available yet. A nearby explanation states the current boundary and intended behaviour. The [support matrix](../reference/16-support-and-roadmap.md) collects these distinctions. Future APIs are not prerequisites for completing the runnable examples.

# Part I: write and understand programs

| Chapter |
| --- |
| [1. Build and run your first program](01-first-program.md) |
| [2. Values, expressions, and layout](02-values-and-layout.md) |
| [3. Functions, types, and local definitions](03-functions.md) |
| [4. Model data and handle every case](04-data-and-matching.md) |
| [5. Recursion, lists, and maps](05-recursion-and-collections.md) |
| [6. Missing values, errors, and validation](06-options-results-and-validation.md) |
| [7. Numbers, text, and bytes](07-numbers-text-and-bytes.md) |
| [8. Split a program into modules](08-modules.md) |
| [9. Signatures, instances, and reusable operations](09-signatures-and-instances.md) |
| [10. Effects, capabilities, and a program that prints](10-effects-and-io.md) |

# Part II: effects, resources, and evidence

| Chapter |
| --- |
| [11. Handle effects and select sequencing](11-handlers-and-sequencing.md) |
| [12. Ownership, borrowing, and regions](12-ownership-and-regions.md) |
| [13. Refine values and make unsafe cases explicit](13-refinements-and-safe-operations.md) |
| [14. Dependent types, finite indices, and proofs](14-dependent-types-and-proofs.md) |
| [15. Productive streams and partial producers](15-streams-and-producers.md) |
| [16. Delay work and memoize deliberately](16-explicit-laziness.md) |
| [17. Structured tasks and cancellation](17-structured-concurrency.md) |

# Part III: compilation and runtime features

| Chapter |
| --- |
| [18. Compile-time code and hygienic macros](18-staging-and-macros.md) |
| [19. Laws, rewrites, and performance](19-rewrites-and-performance.md) |
| [20. Foreign code, layouts, and target profiles](20-foreign-code-and-targets.md) |
| [21. State evolution and temporal contracts](21-upgrades-and-contracts.md) |
| [22. Build a small order calculator](22-putting-it-together.md) |

# Keep a reference nearby

- [Compiler user's guide](../reference/00-guide.md): precise syntax, checking rules, commands, artifacts, and runtime controls.
- [Built-in and shipped libraries](../library/00-index.md): real module APIs extracted from checked interfaces, with distribution/dependency information.
- [Compiler-provided vocabulary](../reference/20-compiler-vocabulary.md): special forms and operations installed by the checker.
- [Original design overview](../overview.md): specifications, historical tutorials, and design background.

The design archive preserves intended semantics and earlier decisions. For current accepted behaviour, this book and guide follow the Rust implementation and its executable tests. The Cinch-written bootstrap compiler has its own progress record.
