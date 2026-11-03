# Current support and what is coming soon

Current behaviour in this documentation follows the **Rust compiler in `compiler/`**. Specifications describe the intended language, including behaviour that is still planned. The Cinch-written bootstrap compiler is a separate implementation under development; its incomplete frontend is not the authority for Rust language support.

# Read the labels

| Label | Meaning |
| --- | --- |
| <span class="support-tag support-tag--current">Available</span> | Implemented by the Rust compiler; the guide describes the admitted surface and its constraints. |
| <span class="support-tag support-tag--partial">Partial</span> | Some of the specified feature works; the current and intended boundaries are stated separately. |
| <span class="support-tag support-tag--api">API only</span> | Exposed through Rust compiler/runtime APIs or fixtures; no corresponding end-user CLI workflow exists. |
| <span class="support-tag support-tag--planned">Coming soon</span> | Intended or selected design direction that is not usable yet. This label is not a release-date promise. |

A label applies to the named behaviour, not automatically to everything on the page. A feature can have working checks and runtime mechanisms while its command-line workflow remains planned. “Parses” alone is not evidence of checking, native lowering, or a complete operator workflow.

# Language and runtime coverage

| Area | Current Rust behaviour | Intended extension / boundary | Guide |
| --- | --- | --- | --- |
| Values, functions, ADTs, records, patterns | Available, with explicit signatures, coverage, guards, tuples, and strict evaluation. | Haskell syntax not mentioned in this guide should not be assumed. | [Expressions](03-expressions-and-patterns.md) |
| Polymorphism and kinds | Explicit top-level quantifiers, higher kinds, eligible local generalization, row unification. | Partial: locally annotated `forall`/`using` remains restricted. | [Types](04-types-and-kinds.md) |
| Signatures and instances | Dictionaries, unique implicit resolution, checked method defaults. | <span class="support-tag support-tag--planned">Coming soon</span> dependent signature fields and associated result families. Explicit constraints remain the current language rule; superclass entailment is not promised. | [Modules](07-modules-and-packages.md) |
| Dependent modules | Current nominal modules, checked visibility, signatures/dictionaries, value parameters. | <span class="support-tag support-tag--planned">Coming soon</span> associated type families and richer dependent/structural module interfaces; exact generativity rules remain open. | [Modules](07-modules-and-packages.md) |
| Module parameters | Dictionary and value parameters with recorded identity. | <span class="support-tag support-tag--planned">Coming soon</span> broader parameterized-module composition forms. | [Modules](07-modules-and-packages.md) |
| Numeric types and safe operations | Mathematical integers, explicit machine overflow families, proven bounds/subtraction/divisors/conversions. | <span class="support-tag support-tag--planned">Coming soon</span> revised unified refined-float design and its additional proof vocabulary. | [Numbers](05-numbers-and-primitives.md) |
| Symbols | Runtime interned symbols and staging name quotations. | <span class="support-tag support-tag--planned">Coming soon</span> general promoted/type-level symbol consumers. | [Numbers and symbols](05-numbers-and-primitives.md) |
| Dependent core | Pi/Sigma, indexed constructors, admitted coverage/conversion, erasure, positivity. | Partial: some rigid-index family elimination is refused; broader elimination remains intended. | [Dependent types](08-dependent-types-and-proofs.md) |
| Refinements and proofs | Named/compound predicates, decisions, branch facts, linear arithmetic, narrowing/forgetting, evidence invalidation. | <span class="support-tag support-tag--planned">Coming soon</span> broader proof automation and verified-primitive proof coverage. | [Refinements](06-refinements.md) |
| Effects and handlers | Rows, capabilities, one-shot handlers over direct expressions. | <span class="support-tag support-tag--planned">Coming soon</span> operations through arbitrary called computations; concrete handler-vtable specialization. | [Effects](09-effects-and-capabilities.md) |
| Ownership and memory | Multiplicities, structural Copy, regions, escape checks, reference-counted reclamation. | <span class="support-tag support-tag--planned">Coming soon</span> selected allocator APIs and representation/specialization extensions. | [Memory](10-ownership-and-memory.md) |
| Stored effectful computations | Explicit effectful closures and thunk forms are available. | <span class="support-tag support-tag--planned">Coming soon</span> the selected `Comp {e} a` library type and resource-safe effectful pull streams. | [Effects](09-effects-and-capabilities.md) |
| Thunks and memo cells | Explicit delay/force, scoped memo groups, Eval-region/black-hole checks. | Open design question: irrefutable function-argument patterns are not promised; current `~name` is memo syntax. | [Laziness](11-laziness-and-codata.md) |
| Codata and producers | Guarded corecursion, Later observations, productive/partial distinction. | Partial: admitted direct construction/guarded proof fragment, not general productivity inference. | [Codata](11-laziness-and-codata.md) |
| Rewrites and optimizer | Justification, law status, target rules, barriers, bounded driving, residual recheck. | <span class="support-tag support-tag--planned">Coming soon</span> higher-level step certificates and remaining specialization cases. | [Optimization](12-rewrites-and-optimization.md) |
| Structured concurrency | Scoped linear handles, share/transfer captures, cancellation/acknowledgment, blocking policy. | <span class="support-tag support-tag--planned">Coming soon</span> bounded channels, cancellation-safe send/receive, fuller race/select/supervision APIs. Optional STM remains a proposal. | [Concurrency](13-concurrency-and-runtime.md) |
| Staging and macros | Typed quotes, hygienic declaration generation, recorded capabilities, schema/tool inputs. | Partial: admitted constant/value transport only; secrets never lift into runtime constants. | [Staging](14-staging-and-macros.md) |
| Attributes and type layout | All 22 compiler-owned attribute names; explicit C/transparent/align/packed layouts and recorded discriminant facts. | <span class="support-tag support-tag--planned">Coming soon</span> inline value types, field unpacking, representation instances/selection, and broader multiword layouts. | [Attributes](23-attributes-and-pragmas.md), [type layout](24-type-layout.md) |
| FFI and inline primitives | Checked C boundaries, repr layouts, LLVM/native-asm paths, containment and assumptions. | <span class="support-tag support-tag--api">API only</span> supplied foreign-object linking; <span class="support-tag support-tag--planned">Coming soon</span> CLR/JVM execution backends. | [FFI](15-ffi-layout-and-attributes.md) |
| Runtime evolution and JIT machinery | Checked migrations, compatibility metadata, versioned runtime/variant mechanisms. | <span class="support-tag support-tag--planned">Coming soon</span> complete service/update-package/tiering CLI workflow. | [Upgrades](17-upgrades-and-temporal-contracts.md) |
| Temporal contracts | Admitted safety/stability checks, finite liveness, structural recurrence. | <span class="support-tag support-tag--planned">Coming soon</span> general unbounded process/temporal proofs. | [Temporal contracts](17-upgrades-and-temporal-contracts.md) |
| Incremental compilation | Rust item/session/cache mechanisms and editor recheck reports. | <span class="support-tag support-tag--api">API only</span> persistent/session integration; no watch/daemon CLI. | [Artifacts](18-artifacts-and-incremental.md) |
| Wasm and components | Wasm32 profile, admitted WIT/canonical adapters, degraded stream/future observations. | <span class="support-tag support-tag--api">API only</span> profiles; <span class="support-tag support-tag--planned">Coming soon</span> end-user cross-target/component packaging commands. | [Targets](19-targets-and-components.md) |
| REPL | Stateful Rust API with commands, declarations, completion, and admitted evaluator operations. | <span class="support-tag support-tag--planned">Coming soon</span> terminal frontend and complete runtime/project workflow; the guide documents the idealized experience. | [REPL](26-repl.md) |
| Language server | `cinch lsp`, shared analysis, document notifications, diagnostics, hover, definition, completion, and full semantic tokens. | <span class="support-tag support-tag--planned">Coming soon</span> packaged setup, complete workspace lifecycle, richer assistance, and refactoring; the guide separates these from existing handlers. | [Language server](27-language-server.md), [diagnostics](21-editors-and-diagnostics.md) |
| Libraries and packages | Embedded base plus source-distributed modules, checked local store/manifests. | <span class="support-tag support-tag--planned">Coming soon</span> remote package installation/publishing and real release bundles. | [Library directory](../library/00-index.md) |
| Self-hosting | Rust seed and Cinch bootstrap ladder with documented focused capabilities. | <span class="support-tag support-tag--planned">Coming soon</span> complete self-built frontend/backend parity and self-build acceptance. | [Self-hosting design](../design/self-hosting.md) |

# Where intended semantics live

| Specification | Reader-facing guide |
| --- | --- |
| [Core language](../specifications/core-language-specification.md) | [Types](04-types-and-kinds.md), [dependent proofs](08-dependent-types-and-proofs.md), [ownership](10-ownership-and-memory.md), [temporal contracts](17-upgrades-and-temporal-contracts.md) |
| [Surface grammar](../specifications/surface-syntax-and-grammar.md) and [amendments](../specifications/surface-grammar-local-amendments.md) | [Lexing/layout](02-lexical-structure.md), [expressions](03-expressions-and-patterns.md) |
| [Modules and compilation](../specifications/modules-and-compilation-specification.md) | [Modules](07-modules-and-packages.md), [staging](14-staging-and-macros.md), [artifacts](18-artifacts-and-incremental.md) |
| [Primitive types](../specifications/primitive-types-specification.md) | [Numeric/text primitives](05-numbers-and-primitives.md) |
| [Laziness and reclamation](../specifications/lazy-computation-and-reclamation-specification.md) | [Laziness](11-laziness-and-codata.md), [memory](10-ownership-and-memory.md) |
| [Effects and sequencing](../specifications/runtime-capabilities-effects-and-sequencing.md) | [Effects](09-effects-and-capabilities.md) |
| [Concurrency](../specifications/concurrency-specification.md) | [Tasks and runtime](13-concurrency-and-runtime.md) |
| [FFI](../specifications/foreign-function-interface-specification.md), [attributes](../specifications/attribute-annotations-specification.md), [inline IR](../specifications/primitive-operations-and-inline-target-ir-specification.md) | [Boundaries and primitives](15-ffi-layout-and-attributes.md), [all attributes](23-attributes-and-pragmas.md), [type layout](24-type-layout.md) |
| [Runtime evolution](../specifications/runtime-evolution-specification.md), [supercompilation/JIT](../specifications/supercompilation-and-jit-specification.md) | [Optimization](12-rewrites-and-optimization.md), [upgrades](17-upgrades-and-temporal-contracts.md) |
| [Platform/SIMD](../specifications/platform-proofs-and-simd-specification.md), [Wasm/WASI](../specifications/webassembly-and-wasi-specification.md) | [Target profiles](19-targets-and-components.md) |

The original documents retain historical implementation notes and open decisions. Some status rows predate later implementation work. When they conflict with the current source/tests, use the Rust implementation for present behaviour and retain the specification as the intended semantics. Specific future designs such as allocators and floats describe selected direction; unresolved choices remain unresolved rather than becoming a release promise.

# Maintaining the distinction

Each guide chapter names its owning Rust files and behaviour suites. Complete tutorial examples carry executable verification markers. Library pages are generated from checked `.cinchif` exports and source bytes, and their drift check rejects changed source sets. Update a Coming soon label only after the named behaviour is accepted through the relevant checking and execution path, then update the example and reference together.
