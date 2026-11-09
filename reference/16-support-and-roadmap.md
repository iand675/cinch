# Current support and what is coming soon

Current behaviour in this documentation follows the **Rust compiler in `compiler/`**. Specifications describe the intended language, including behaviour that is still planned. The Cinch-written bootstrap compiler is a separate implementation under development; its incomplete frontend is not the authority for Rust language support.

Every row below names one behaviour, the command or test that demonstrates it, its status, and the file or test that enforces it. Run the commands against the built compiler at `compiler/target/debug/cinch`.

# Read the labels

| Label | Meaning |
| --- | --- |
| <span class="support-tag support-tag--current">Current</span> | Implemented by the Rust compiler; the guide describes the admitted surface and its constraints. |
| <span class="support-tag support-tag--partial">Partial</span> | Some of the specified feature works; the current and intended boundaries are stated separately. |
| <span class="support-tag support-tag--api">API only</span> | Exposed through Rust compiler or runtime APIs, or through a build feature, with no end-user CLI workflow. |
| <span class="support-tag support-tag--planned">Coming soon</span> | Intended or selected design direction that is not usable yet. This label is not a release-date promise. |

A label applies to the named behaviour, not to everything on the page. A feature can have working checks and runtime mechanisms while its command-line workflow stays planned. Parsing alone is not evidence of checking, native lowering, or a complete operator workflow.

A test named as `suite.rs::test` runs a complete program through the library driver: it checks the sources, emits, links where the case needs it, and compares the observed exit code or error text. That is the demonstration when a behaviour has no single CLI spelling.

# Language and checking

| Feature | Demonstrated by | Status | Enforcing code or test |
| --- | --- | --- | --- |
| Values, functions, `data`, `record`, patterns, strict evaluation | `cinch run` on the [first program](../learn/01-first-program.md) | <span class="support-tag support-tag--current">Current</span> | `compiler/src/check_terms.rs`; `compiler/tests/integration.rs::surface_and_or_short_circuit` |
| Pattern coverage with guards | `guard_coverage.rs::unguarded_final_row_compiles_and_runs_every_row` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/check_terms.rs`; `compiler/tests/guard_coverage.rs` |
| Top-level polymorphism and higher kinds | `hk.rs::hk_surface_fixture_checks`, `hk.rs::lib_category_check_runs` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/types.rs`; `compiler/tests/hk.rs` |
| Local signatures that quantify (`forall`) or carry constraints (`using`) | `cinch check` refuses with `` local signatures with forall or using are not supported in the bootstrap slice `` | <span class="support-tag support-tag--partial">Partial</span> | `compiler/src/check_decls.rs` |
| Signatures, instances, dictionaries, checked method defaults | `modules.rs::instance_coherence_across_modules`, `class_defaults.rs::an_omitted_method_takes_its_default_and_a_definition_overrides_it` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/check_dictionary.rs`; `compiler/tests/modules.rs`, `compiler/tests/class_defaults.rs` |
| Dependent core: Pi and Sigma, indexed families, erasure, positivity | `dependent.rs::erased_fields_are_absent_from_runtime` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/check_terms.rs`, `compiler/src/check_erasure.rs`; `compiler/tests/dependent.rs` |
| Refinements, decisions, branch facts, linear arithmetic | `refinements.rs`, `lattice.rs` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/check_terms.rs`; `compiler/tests/refinements.rs`, `compiler/tests/lattice.rs` |
| Effects, capabilities, and handlers | `effects.rs::one_shot_resumption_rejects_a_second_resume` | <span class="support-tag support-tag--partial">Partial</span>: one-shot `resume`; call-dispatched handling and multi-shot resumption are deferred | `compiler/src/check_terms.rs`, `compiler/src/types.rs`; `compiler/tests/effects.rs` |
| Codata, guarded corecursion, `Later` observations | `codata.rs::unproductive_corec_is_rejected_naming_the_call` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/check_decls.rs`; `compiler/tests/codata.rs` |
| Numbers: mathematical integers, machine overflow families, proven divisors and conversions | `primitives.rs::checked_overflow_returns_none`, `primitives.rs::literal_range_checking` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/core.rs`, `compiler/src/types.rs`; `compiler/tests/primitives.rs`, `safe_division.rs`, `safe_subtraction.rs`, `small_nats.rs` |
| Refined floating-point vocabulary for NaN, infinities, and division hazards | Feasibility proposal for review | <span class="support-tag support-tag--planned">Coming soon</span> | [Float refinements design](../design/float-refinements.md) |
| Associated type families and per-instance family members | No implementation; `compiler/SPEC-MAPPING.md` row 52 records the gap | <span class="support-tag support-tag--planned">Coming soon</span> | [Modules and compilation](../specifications/modules-and-compilation-specification.md) |
| Irrefutable function-argument patterns | Open design question; the current `~name` form is the memo binding, not a pattern modifier | <span class="support-tag support-tag--planned">Coming soon</span> | [Laziness and reclamation](../specifications/lazy-computation-and-reclamation-specification.md) section 5 |
| Symbols: interned values and staging quotations | `sym.rs::intern_and_sym_name_round_trip_with_identity_across_the_boundary` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/core.rs`; `compiler/tests/sym.rs` |
| Arrays, tensors, columnar data | `arrays.rs::normal_arrays_preserve_values_and_return_none_at_boundaries` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/arrays.rs`, `compiler/src/arrays_codegen.rs`; `compiler/tests/arrays.rs` |
| Rewrites, fusion, driving, residual recheck | `optimizer.rs::map_fusion_preserves_values_and_effect_order`, `optimizer.rs::weights_survive_optimization` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/specialize.rs`; `compiler/tests/optimizer.rs` |
| Staging: quotes, splices, `quoteDecls`, macros, hygiene, stage scheduling | `staging.rs::stage_dependency_cycle_shows_the_cycle` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/check_decls.rs`, `compiler/src/provenance.rs`; `compiler/tests/staging.rs` |
| Temporal contracts: `always`, `eventually`, `recur`, `stable` | `temporal.rs::contract_facts_hash_both_ways`, `temporal.rs::liveness_must_state_its_assumptions` | <span class="support-tag support-tag--current">Current</span>, bounded claims only; general unbounded proofs are <span class="support-tag support-tag--planned">Coming soon</span> | `compiler/src/check_decls.rs`; `compiler/tests/temporal.rs` |
| Attributes and type layout | `attributes.rs::inline_hints_reach_llvm`, `attributes.rs::repr_layout_facts_recorded_in_the_abi_table`; the [attribute index](23-attributes-and-pragmas.md) | <span class="support-tag support-tag--current">Current</span> for all 22 names; inline value types, field unpacking, and representation selection are <span class="support-tag support-tag--planned">Coming soon</span> | `compiler/src/check_decls.rs` (`parse_repr_args`, `compute_layouts`), `compiler/src/interface.rs` (`AbiLayout`); `compiler/tests/attributes.rs`, `compiler/tests/repr_classes.rs` |
| Safety assertions verified at definitions | `safety.rs::jpl_profile_rejects_each_rule_and_never_looser`, `safety.rs::bounded_claims_name_their_proof_method_or_are_refused` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/check_decls.rs::verify_definition_safety`; `compiler/tests/safety.rs` |

# Runtime, memory, and boundaries

| Feature | Demonstrated by | Status | Enforcing code or test |
| --- | --- | --- | --- |
| Ownership, multiplicities, regions, borrow escape checks | `ownership.rs::linear_double_use_is_rejected`, `ownership.rs::borrow_escape_is_rejected` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/check_terms.rs`; `compiler/tests/ownership.rs` |
| Reference-counted reclamation and allocation accounting | `reclamation.rs`, `alloc_accounting.rs` | <span class="support-tag support-tag--current">Current</span> | `compiler/runtime/src/rc.rs`; `compiler/tests/reclamation.rs`, `compiler/tests/alloc_accounting.rs` |
| Thunks, memo cells, black-hole and demand-cycle checks | `integration.rs::memo_group_black_hole_is_rejected` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/check_terms.rs`; `compiler/tests/letrec.rs`, `compiler/tests/integration.rs` |
| Structured concurrency: scopes, tasks, cancellation, handle linearity | `concurrency.rs::dropping_a_handle_is_rejected` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/check_terms.rs`, `compiler/runtime/src/coop.rs`; `compiler/tests/concurrency.rs` |
| Bounded channels and synchronized sharing | No implementation; the concurrency design records the gap | <span class="support-tag support-tag--planned">Coming soon</span> | [Concurrency specification](../specifications/concurrency-specification.md) section 5 |
| Software transactional memory | Proposed library addition (`atomically`, transactional effects) | <span class="support-tag support-tag--planned">Coming soon</span> | [Concurrency specification](../specifications/concurrency-specification.md) section 7 |
| Allocator selection and in-place update discipline | No implementation | <span class="support-tag support-tag--planned">Coming soon</span> | [Allocators design](../design/allocators.md) |
| Stored effectful computations, `Comp {e} a` | No implementation; decision D327 selects the shape | <span class="support-tag support-tag--planned">Coming soon</span> | [Computation type design](../design/comp-computation-type.md) |
| Foreign imports and exports, C ABI crossing, faults | `ffi.rs::foreign_import_calls_a_real_c_symbol`, `llvm_contract.rs::byval_and_sret_cross_a_real_c_boundary` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/check_decls.rs`, `compiler/src/codegen.rs`; `compiler/tests/ffi.rs`, `compiler/tests/llvm_contract.rs` |
| Inline primitive bodies: `llvm` and `asm` families, containment, tiers | `prim.rs::prim_with_llvm_and_asm_bodies_checks_and_compiles`, `prim.rs::assumed_prim_identity_is_the_body_hash` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/check_decls.rs`; `compiler/tests/prim.rs` |
| `clr` and `jvm` primitive body families | `prim.rs::clr_and_jvm_body_slots_are_reserved` | <span class="support-tag support-tag--planned">Coming soon</span> | `compiler/tests/prim.rs` (reserved slots named at parse) |
| Runtime evolution: upgrades, compatibility, variant lifecycle | `evolution.rs::incomplete_update_set_is_rejected_at_activation` | <span class="support-tag support-tag--current">Current</span> for the checked mechanisms; a service and update-package CLI workflow is <span class="support-tag support-tag--planned">Coming soon</span> | `compiler/runtime/src/evolution.rs`; `compiler/tests/evolution.rs` |
| Wasm32 profile, canonical ABI adapters, WASI streams | `wasm.rs::wasi_stream_is_an_asynchronous_source_not_a_productive_stream`, `wasm.rs::usize_width_and_emission_key_on_the_target` | <span class="support-tag support-tag--api">API only</span>: profiles are selected through the library, and no CLI flag chooses a target | `compiler/src/codegen.rs`, `compiler/tests/wasm.rs`, `compiler/tests/platform.rs` |

# Tools, packages, and artifacts

| Tool or workflow | Demonstrated by | Status | Enforcing code or test |
| --- | --- | --- | --- |
| Typecheck only | `cinch check Main.cinch` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/main.rs`, `compiler/src/driver.rs` |
| Build and run | `cinch build Main.cinch -o prog`, `cinch run Main.cinch` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/driver.rs`; `compiler/tests/integration.rs` |
| Emit LLVM IR | `cinch emit-ir Main.cinch -o out.ll` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/codegen.rs`; `compiler/tests/llvm_contract.rs` |
| Emit an object file | `cinch emit-object Lib.cinch -o out.o` on a library module; a module with an entry point is refused with `` object emission requires a library program without a process entry `` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/main.rs`; `compiler/tests/intrinsic_objects.rs` |
| Documentation extraction and rendering | `cinch doc extract`, `cinch doc render`, `cinch doc lookup`, `cinch doc index`, `cinch doc test`, `cinch doc markdown`, `cinch doc highlight` | <span class="support-tag support-tag--current">Current</span> | `compiler/src/cinchdoc.rs`, `compiler/src/highlight.rs` |
| Interactive session | `cinch repl` | <span class="support-tag support-tag--current">Current</span>: declarations, commands, completion, full-screen terminal frontend, native probes, transactional reload | `compiler/src/repl.rs`, `compiler/src/repl_tui.rs`; `compiler/tests/repl.rs` |
| Language server | `cinch lsp` over stdio | <span class="support-tag support-tag--current">Current</span> handlers: diagnostics, hover, definition, completion, semantic tokens | `compiler/src/lsp.rs`, `compiler/src/analysis.rs`; `compiler/tests/lsp.rs` |
| JIT execution | `cinch jit Main.cinch`; the prebuilt compiler answers `` this build was compiled without the jit feature `` | <span class="support-tag support-tag--api">API only</span>: requires a build with `--features jit` | `compiler/src/jit.rs`; `compiler/tests/jit.rs::cinch_jit_runs_a_checked_program` |
| Incremental recheck through the library | `incremental.rs` project probes | <span class="support-tag support-tag--api">API only</span>: no watch or daemon command | `compiler/src/incremental.rs`; `compiler/tests/incremental.rs` |
| Package install, publish, and provenance | `cinch pkg add`, `cinch pkg install`, `cinch pkg publish`, `cinch pkg verify --slsa` | <span class="support-tag support-tag--current">Current</span> through L1 provenance; client-side L2 builder trust and VSA issuance are <span class="support-tag support-tag--planned">Coming soon</span> | `compiler/src/pkg.rs`, `compiler/src/store.rs`; `compiler/tests/package_store.rs`, `compiler/tests/package_manifest.rs` |
| Cinch-written bootstrap compiler | `bootstrap_ladder.rs` assembles every module in `bootstrap/` as one project | <span class="support-tag support-tag--partial">Partial</span>: focused capabilities, not full frontend and backend parity | `bootstrap/*.cinch`; `compiler/tests/bootstrap_ladder.rs` and the `bootstrap_*` suites |
| Library catalogue | Generated pages under [the library directory](../library/00-index.md) | <span class="support-tag support-tag--current">Current</span>: generated from checked `.cinchif` exports, with recorded source hashes | `website/scripts/generate-library-reference.py` (asserts the recorded hashes), `compiler/src/cinchdoc.rs`; `compiler/tests/prelude_lib.rs` |

# Where intended semantics live

| Specification | Reader-facing guide |
| --- | --- |
| [Core language](../specifications/core-language-specification.md) | [Types](04-types-and-kinds.md), [dependent proofs](08-dependent-types-and-proofs.md), [ownership](10-ownership-and-memory.md), [temporal contracts](17-upgrades-and-temporal-contracts.md) |
| [Surface grammar](../specifications/surface-syntax-and-grammar.md) and [amendments](../specifications/surface-grammar-local-amendments.md) | [Lexing and layout](02-lexical-structure.md), [expressions](03-expressions-and-patterns.md) |
| [Modules and compilation](../specifications/modules-and-compilation-specification.md) | [Modules](07-modules-and-packages.md), [staging](14-staging-and-macros.md), [artifacts](18-artifacts-and-incremental.md) |
| [Primitive types](../specifications/primitive-types-specification.md) | [Numbers and primitives](05-numbers-and-primitives.md) |
| [Laziness and reclamation](../specifications/lazy-computation-and-reclamation-specification.md) | [Laziness](11-laziness-and-codata.md), [memory](10-ownership-and-memory.md) |
| [Effects and sequencing](../specifications/runtime-capabilities-effects-and-sequencing.md) | [Effects](09-effects-and-capabilities.md) |
| [Concurrency](../specifications/concurrency-specification.md) | [Tasks and runtime](13-concurrency-and-runtime.md) |
| [FFI](../specifications/foreign-function-interface-specification.md), [attributes](../specifications/attribute-annotations-specification.md), [inline IR](../specifications/primitive-operations-and-inline-target-ir-specification.md) | [Boundaries and primitives](15-ffi-layout-and-attributes.md), [all attributes](23-attributes-and-pragmas.md), [type layout](24-type-layout.md) |
| [Runtime evolution](../specifications/runtime-evolution-specification.md), [supercompilation and JIT](../specifications/supercompilation-and-jit-specification.md) | [Optimization](12-rewrites-and-optimization.md), [upgrades](17-upgrades-and-temporal-contracts.md) |
| [Platform proofs and SIMD](../specifications/platform-proofs-and-simd-specification.md), [Wasm and WASI](../specifications/webassembly-and-wasi-specification.md) | [Target profiles](19-targets-and-components.md) |

The original documents keep historical implementation notes and open decisions, and some of their status rows predate later work. Where one conflicts with the current source or tests, the Rust implementation describes present behaviour and the specification keeps the intended semantics. Designs for allocators, floats, and representation selection describe selected direction; unresolved choices stay unresolved rather than becoming a release promise.

# Maintaining the distinction

Each guide chapter names its owning Rust files and behaviour suites. Complete tutorial examples carry executable verification markers. Library pages are generated from checked `.cinchif` exports and source bytes, and their drift check rejects changed source sets.

Change a status row only when three things move together: the checking or execution path accepts the behaviour, a test or verified example demonstrates it, and the reference page states the argument grammar, the diagnostic text, and the current boundary. A label that says `Coming soon` is removed in the same change that lands the behaviour, never before.
