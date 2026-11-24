# Current support and roadmap

This matrix summarizes what the **Rust compiler** (`seed/`) implements today, where partial boundaries lie, and which specified features remain planned. The Cinch-written self-hosted compiler (`compiler/`) is developed in parallel and does not narrow the Rust compiler's supported surface.

# Support labels

| Label | Meaning |
| --- | --- |
| <span class="support-tag support-tag--current">Current</span> | Implemented and usable end to end in the Rust compiler. |
| <span class="support-tag support-tag--partial">Partial</span> | A defined subset works; current limits are noted inline. |
| <span class="support-tag support-tag--api">API only</span> | Available through Rust library APIs or optional Cargo features without a default CLI workflow. |
| <span class="support-tag support-tag--planned">Coming soon</span> | Specified design direction not yet implemented. |

# Language and type system

| Feature | Status | Notes and guide |
| --- | --- | --- |
| Values, functions, `data`, `record`, `letrec`, strict evaluation | <span class="support-tag support-tag--current">Current</span> | [Expressions and patterns](03-expressions-and-patterns.md) |
| Exhaustive pattern coverage with guards | <span class="support-tag support-tag--current">Current</span> | Missing constructors fail compile; failing guards fault at runtime (exit 70). [Chapter 03](03-expressions-and-patterns.md) |
| Top-level polymorphism, rank-N types, and higher kinds | <span class="support-tag support-tag--current">Current</span> | [Types and kinds](04-types-and-kinds.md) |
| Local `forall` / `using` signatures | <span class="support-tag support-tag--partial">Partial</span> | Top-level signatures support full polymorphism and constraints; local `forall`/`using` annotations are not yet accepted. |
| Type classes (`signature`), instances, dictionaries, defaults | <span class="support-tag support-tag--current">Current</span> | Superclass entailment is explicit; state each needed constraint. [Chapter 07](07-modules-and-packages.md) |
| Associated type families | <span class="support-tag support-tag--planned">Coming soon</span> | Specified in [modules and compilation](../specifications/modules-and-compilation-specification.md). |
| Dependent core: universes, Pi/Sigma, indexed families, erasure, positivity | <span class="support-tag support-tag--current">Current</span> | Proofs and indices erase completely (`%0`). [Chapter 08](08-dependent-types-and-proofs.md) |
| Refinements, `decide`, branch facts, linear arithmetic | <span class="support-tag support-tag--current">Current</span> | [Refinements](06-refinements.md) |
| Effect rows, lexical capabilities (`Io r`), one-shot handlers | <span class="support-tag support-tag--partial">Partial</span> | One-shot `resume` and lexical handlers work; call-dispatched handlers and multi-shot resumption are deferred. [Chapter 09](09-effects-and-capabilities.md) |
| Stored effectful computations (`Comp {e} a`) | <span class="support-tag support-tag--planned">Coming soon</span> | Design in [computation type](../design/comp-computation-type.md). |
| Codata, `Later`, guarded `corec` | <span class="support-tag support-tag--current">Current</span> | [Laziness and codata](11-laziness-and-codata.md) |
| Numbers: arbitrary-precision `Int`/`Nat`, fixed-width scalars, checked/wrapping/saturating ops | <span class="support-tag support-tag--current">Current</span> | [Numbers and primitives](05-numbers-and-primitives.md) |
| Refined floating-point predicates (NaN/infinity guards) | <span class="support-tag support-tag--planned">Coming soon</span> | Design in [float refinements](../design/float-refinements.md). |
| Irrefutable function-argument patterns (`~p`) | <span class="support-tag support-tag--planned">Coming soon</span> | `~name` is currently the memo-binding form, not an irrefutable pattern. |
| Interned symbols (`Sym`), `Array`, `Tensor`, `Columnar` | <span class="support-tag support-tag--current">Current</span> | [Data.Sym](../library/Data.Sym.md), [Data.Array](../library/Data.Array.md), [Data.Tensor](../library/Data.Tensor.md) |
| Rewrite rules, fusion, driving, specialization reports | <span class="support-tag support-tag--current">Current</span> | [Rewrites and optimization](12-rewrites-and-optimization.md) |
| Staging: `[|...|]`, splices, `quoteDecls`, hygienic macros, `data` imports | <span class="support-tag support-tag--current">Current</span> | [Staging and macros](14-staging-and-macros.md) |
| Temporal contracts (`always`, `eventually`, `recur`, `stable`) | <span class="support-tag support-tag--partial">Partial</span> | Bounded claims are checked; general unbounded proofs are <span class="support-tag support-tag--planned">Coming soon</span>. [Chapter 17](17-upgrades-and-temporal-contracts.md) |
| Compiler attributes (`#[...]`), `repr`, `#[unpack]`/`#[nounpack]` | <span class="support-tag support-tag--current">Current</span> | All 25 compiler-owned attributes; inline value types and representation selection remain <span class="support-tag support-tag--planned">Coming soon</span>. [Chapter 23](23-attributes-and-pragmas.md) |
| Safety assertions and JPL profile | <span class="support-tag support-tag--current">Current</span> | [Safety assertions](22-safety-assertions-and-llvm.md) |

# Runtime, memory, and interop

| Feature | Status | Notes and guide |
| --- | --- | --- |
| Ownership, multiplicities (`%0`/`%1`/`%aff`/`%many`), regions, borrow checks | <span class="support-tag support-tag--current">Current</span> | [Ownership and memory](10-ownership-and-memory.md) |
| Reference-counted reclamation, stack allocation, leak/alloc diagnostics | <span class="support-tag support-tag--current">Current</span> | [Chapter 10](10-ownership-and-memory.md), [Chapter 25](25-allocators.md) |
| Thunks, `~` memo cells, black-hole and `Eval r` cycle checks | <span class="support-tag support-tag--current">Current</span> | [Laziness and codata](11-laziness-and-codata.md) |
| Structured concurrency: scopes, `spawn`, linear handles, cancellation | <span class="support-tag support-tag--current">Current</span> | [Concurrency and runtime](13-concurrency-and-runtime.md) |
| Software transactional memory (`TVar`, `atomically`, `retry`, `or-else`) | <span class="support-tag support-tag--current">Current</span> | [Effect.Stm](../library/Effect.Stm.md), [Chapter 13](13-concurrency-and-runtime.md) |
| Bounded channels and synchronized sharing | <span class="support-tag support-tag--planned">Coming soon</span> | Specified in [concurrency](../specifications/concurrency-specification.md). |
| Replaceable allocators, scoped arenas, proven allocation bounds | <span class="support-tag support-tag--planned">Coming soon</span> | [Allocators](25-allocators.md) |
| C FFI (`foreign import`/`export`), `#[byval]`/`#[sret]`, `opaque`, `RawPtr` | <span class="support-tag support-tag--current">Current</span> | [FFI and primitives](15-ffi-layout-and-attributes.md) |
| Native C++17 bindings, exports, shared libraries, and Godot GDExtension | <span class="support-tag support-tag--current">Current</span> | `cinch cxx`, `cinch build --shared`, `--native-object`. [Chapter 01](01-using-the-compiler.md) |
| Inline `prim` bodies (`llvm`, `asm`) with target `when` guards | <span class="support-tag support-tag--current">Current</span> | `clr` and `jvm` body families are reserved and <span class="support-tag support-tag--planned">Coming soon</span>. [Chapter 15](15-ffi-layout-and-attributes.md) |
| Runtime upgrades (`upgrade`, migrations, versioned dispatch) | <span class="support-tag support-tag--partial">Partial</span> | Checking and runtime activation work; a service/bundle CLI workflow is <span class="support-tag support-tag--planned">Coming soon</span>. [Chapter 17](17-upgrades-and-temporal-contracts.md) |
| Direct syscall layer (`Prim.Sys`) | <span class="support-tag support-tag--partial">Partial</span> | Native Darwin (`libSystem`) and Linux (`syscall`) backends cover core runtime ops; other targets fall back to `Prim.Sys.Portable`. |

# Targets, tooling, and packages

| Feature | Status | Notes and guide |
| --- | --- | --- |
| `check`, `emit-ir`, `emit-object`, `build`, `run`, `core` | <span class="support-tag support-tag--current">Current</span> | [Using the compiler](01-using-the-compiler.md) |
| Cross-compilation targets (`darwin`/`linux` x86-64 & ARM64, `wasm32-unknown-unknown`) | <span class="support-tag support-tag--partial">Partial</span> | `check`, `emit-ir`, and `emit-object` work for all 5 triples on any host. `wasm32-unknown-unknown` builds its own runtime archive and runs under Node; foreign native `build` requires `--runtime-archive` and cross `CINCH_CLANG`. |
| WASI streams and WIT canonical-ABI component adapters | <span class="support-tag support-tag--api">API only</span> | Available through compiler library APIs; component packaging CLI is <span class="support-tag support-tag--planned">Coming soon</span>. [Chapter 19](19-targets-and-components.md) |
| Host/target profile separation and conditional imports/declarations (`when`) | <span class="support-tag support-tag--current">Current</span> | Remote target evaluation for foreign compile-time macros is <span class="support-tag support-tag--planned">Coming soon</span>. [Chapter 07](07-modules-and-packages.md) |
| Documentation tooling (`extract`, `render`, `html`, `search`, `lookup`, `index`, `test`) | <span class="support-tag support-tag--current">Current</span> | Hoogle-style CLI & web search plus Hackage-style HTML output. [Chapter 28](28-documentation.md) |
| Interactive REPL (`cinch repl`) | <span class="support-tag support-tag--current">Current</span> | Full-screen TUI, completion, project reload, native probes. [Chapter 26](26-repl.md) |
| Language server (`cinch lsp`) | <span class="support-tag support-tag--current">Current</span> | Diagnostics, hover, definition, completion, semantic tokens over stdio. [Chapter 27](27-language-server.md) |
| JIT execution (`cinch jit`) | <span class="support-tag support-tag--api">API only</span> | Requires building with `--features jit`. |
| Incremental declaration-level recheck | <span class="support-tag support-tag--api">API only</span> | Used internally by REPL/LSP and library APIs; no batch `--watch` mode yet. [Chapter 18](18-artifacts-and-incremental.md) |
| Package management (`cinch pkg add\|install\|info\|publish\|verify`) | <span class="support-tag support-tag--current">Current</span> | Registry, path, and git packages with L1 SLSA provenance; L2 builder trust and VSAs are <span class="support-tag support-tag--planned">Coming soon</span>. [Chapter 29](29-package-management.md) |
| Native link plans (`link`, `local_link`, `--print-link-plan`, `pkg-config`) | <span class="support-tag support-tag--partial">Partial</span> | macOS and Linux linkers work; Windows `import_libs`/`dll_dirs` are <span class="support-tag support-tag--planned">Coming soon</span>. |
| Self-hosted Cinch compiler (`compiler/`) | <span class="support-tag support-tag--partial">Partial</span> | Multi-stage native bootstrap advancing toward parity with `seed/`. |

# Specification map

The original design specifications preserve full language rationale and intended semantics. Where an older specification note conflicts with current behavior, this guide reflects the Rust compiler:

| Specification | User's guide chapters |
| --- | --- |
| [Core language](../specifications/core-language-specification.md) | [04 · Types](04-types-and-kinds.md), [08 · Dependent proofs](08-dependent-types-and-proofs.md), [10 · Ownership](10-ownership-and-memory.md), [17 · Upgrades](17-upgrades-and-temporal-contracts.md) |
| [Surface syntax](../specifications/surface-syntax-and-grammar.md) & [amendments](../specifications/surface-grammar-local-amendments.md) | [02 · Lexical](02-lexical-structure.md), [03 · Expressions](03-expressions-and-patterns.md) |
| [Modules and compilation](../specifications/modules-and-compilation-specification.md) | [07 · Modules](07-modules-and-packages.md), [14 · Staging](14-staging-and-macros.md), [18 · Artifacts](18-artifacts-and-incremental.md) |
| [Primitive types](../specifications/primitive-types-specification.md) | [05 · Numbers and primitives](05-numbers-and-primitives.md) |
| [Laziness and reclamation](../specifications/lazy-computation-and-reclamation-specification.md) | [10 · Memory](10-ownership-and-memory.md), [11 · Laziness](11-laziness-and-codata.md) |
| [Effects and sequencing](../specifications/runtime-capabilities-effects-and-sequencing.md) | [09 · Effects and capabilities](09-effects-and-capabilities.md) |
| [Concurrency](../specifications/concurrency-specification.md) | [13 · Concurrency and runtime](13-concurrency-and-runtime.md) |
| [FFI](../specifications/foreign-function-interface-specification.md), [attributes](../specifications/attribute-annotations-specification.md), [inline IR](../specifications/primitive-operations-and-inline-target-ir-specification.md) | [15 · FFI](15-ffi-layout-and-attributes.md), [23 · Attributes](23-attributes-and-pragmas.md), [24 · Type layout](24-type-layout.md) |
| [Runtime evolution](../specifications/runtime-evolution-specification.md), [supercompilation](../specifications/supercompilation-and-jit-specification.md) | [12 · Optimization](12-rewrites-and-optimization.md), [17 · Upgrades](17-upgrades-and-temporal-contracts.md) |
| [Platform proofs and SIMD](../specifications/platform-proofs-and-simd-specification.md), [Wasm/WASI](../specifications/webassembly-and-wasi-specification.md) | [19 · Targets and components](19-targets-and-components.md) |
| [Documentation comments](../specifications/documentation-comments-specification.md) | [28 · Documentation](28-documentation.md) |
