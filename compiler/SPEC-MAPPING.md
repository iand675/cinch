# Spec requirement mapping

Requirement-by-requirement mapping of the specification suite onto the
compiler, per the P15 item in `CONFORMANCE.md`. Produced 2026-10-25 by a
read-only audit of `specifications/*.md` against `compiler/src`,
`compiler/prelude`, `compiler/runtime`, and the test suites; entries
updated against post-audit verification where noted.

Status legend: **I** = implemented, **P** = partial, **N** = not yet,
**CV** = cannot verify (no observable way to test today). Evidence is the
enforcing code and the test that exercises it; "structural" means the
property holds by construction rather than by a named test.

## core-language-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 1 | core-lang#1-strict-cbv | Strict call-by-value evaluation, no implicit laziness | I | `codegen.rs` eager word model; thunks only via `Thunk`/`~` (structural) |
| 2 | core-lang#1-scrutinee-once | Pattern matching evaluates its scrutinee once | CV | pattern-matrix compiler in `check_terms.rs` implies it; no test observes evaluation count |
| 3 | core-lang#1-short-circuit | `&&`/`||` short-circuit | I | `parser.rs` desugars infix `&&`/`||` to `if` (D14); `tests/integration.rs::surface_and_or_short_circuit` |
| 4 | core-lang#1-totality-diverge | Reject unsupported totality claims or require explicit `Diverge` | N | `Diverge` enforcement on executable definitions deferred (D32); logical fragment is checked terminologically |
| 5 | core-lang#2-dependent-core | Dependent function/pair types, indexed families, dependent matching, universes | I | Pi/Sigma, universes, indexed `Vec`/`Fin`/`Dec` builtins plus general user families (D27-D31); rigid-index refinement rejected conservatively (D35); `tests/dependent.rs` |
| 6 | core-lang#2-type-in-type | `Type :: Type` must not be admitted | I | stratified `Type 0 : Type 1 : ...`, `Type : Type` rejected (D27); `tests/dependent.rs` |
| 7 | core-lang#2-positivity | Inductive definitions satisfy positivity discipline | I | strict positivity for all `data` (P16 A1 closed); negative and nested occurrences rejected |
| 8 | core-lang#2-termination | Recursive logical definitions satisfy termination discipline | I | `spec`/`proof` purity + self-structural decrease + mutual-cycle rejection (D32); executable `Diverge` enforcement deferred |
| 9 | core-lang#2-coverage-indices | Pattern coverage accounts for indices/impossible constructors | I | index equations solve/refute/unknown; refuted arms rejected as impossible, refuted constructors need no branch (D33); rigid-index cases rejected naming the equation (D35) |
| 10 | core-lang#2-defeq | Definitional vs. propositional equality distinguished; solvers must not extend conversion | I | conversion is beta/iota/alpha/proof-irrelevance only (D28); `decide` never extends it |
| 11 | core-lang#3-logical-fragment | Logical fragment consistency, erasure correctness | I | `proof`/`spec`/`Prop` with erasure (no runtime slots), no evidence from failed checks; `tests/dependent.rs` |
| 12 | core-lang#4-gradual-strengthening | Gradual strengthening of specifications | P | `spec`/`proof` declarations exist; the full strengthening story rides on P7 refinements |
| 13 | core-lang#4-obligation-statuses | Disproved/unsupported/timeout obligation statuses distinct; tests never discharge universal obligations | I | `decide` folds Checked/Disproved, errors on Unsupported/Timeout, Unresolved runs scoped runtime comparison (D33) |
| 14 | core-lang#4-holes | Deployment profile rejects unresolved required holes | N (by over-rejection) | typed holes categorically rejected (`check_terms.rs`); no hole reaches codegen |
| 15 | core-lang#4-assumption-manifest | Trusted assumptions exposed in a transitive manifest | I | `interface.rs` `Assumptions`, `<entry>.manifest.json` (D4/D9); `tests/modules.rs::interface_round_trips_through_disk` |
| 16 | core-lang#5-evidence-invalidation | Evidence about mutable state invalidated on change | N | no refinement/flow system; roadmap gap A2 (P16) |
| 17 | core-lang#5-missing-vs-none | Missing fields vs `None` distinguishable | I | records require all fields (`check_decls.rs`); `Option` explicit (structural) |
| 18 | core-lang#6-effects-capabilities-both | Effects and capabilities both represented | I | effect rows honest; capabilities modeled as `Io`-headed runtime identities with lexical supplies (D21); `tests/effects.rs` capability tests |
| 19 | core-lang#6-handler-type | Handler effect typing; multi-shot resumption must not duplicate linear captures | P | `handle ... with` with effect-typed clauses and one-shot `resume` (D23/D26: `%1` captures promoted to exactly-once; call-dispatched handling deferred); `tests/effects.rs` handler and resumption tests |
| 20 | core-lang#7-memory-safety | No use-after-free, double-free, invalid mutable aliasing, data races | P | multiplicities + regions (`tests/ownership.rs::linear_double_use_is_rejected`, `::affine_double_use_is_rejected`, `::borrow_escape_is_rejected`, `::exclusive_borrows_are_lexically_unique`); races wait for P12 |
| 21 | core-lang#7-borrow-kinds | Shared/exclusive borrows, consuming transfer, synchronized sharing | P | `Ref`/`Mut`/`%1` transfer done (`::scoped_borrow_is_accepted`); synchronized sharing is P12 |
| 22 | core-lang#7-copy-semantics | Sharing/copying/cloning defined | I | structural `Copy` (`::copy_class_from_the_vector_filter_shape_works`, `::copy_class_is_structural_and_not_user_extensible`, `::copy_out_coheres_with_multiplicity`) |
| 23 | core-lang#9-coinduction | Coinductive data and sound guarded corecursion | I | `codata` delayed-observation records + `corec` with guard checking (D37/D38); `tests/codata.rs` |
| 24 | core-lang#9-productivity | `Later` force rules prevent unproductive recursion as total | I | unguarded `corec` rejected naming the call; productivity loss typed as `Diverge` (D39); `tests/codata.rs` |
| 25 | core-lang#10-temporal | Safety/liveness/recurrence/stability contracts with explicit assumptions | N | no temporal system; roadmap gap A3 (P16) |
| 26 | core-lang#11-acceptance-manifest | Manifest identifies axioms, external contracts, proof backends, trusted components | P | axioms/assumptions recorded (row 15); proof kernel is `refl`/`congr`/`decide` only |

## primitive-types-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 27 | prim#2-usize-width | `Usize`/`Isize` at least 32 bits, same width | I | `types.rs` `Scalar::Usize/Isize` (64-bit) (structural) |
| 28 | prim#2-usize-cross-compile | Cross-compilation uses the target's width | N | single target; roadmap gap A4 (P16) |
| 29 | prim#4-overflow-families | checked/wrapping/saturating Add/Sub/Mul/Neg | I | prelude + `core.rs::PrimOp`; `tests/primitives.rs::checked_overflow_returns_none`, `::saturating_clamps_to_bounds` |
| 30 | prim#4-div-rem-checked-only | Division/remainder only checked | I | `::checked_div_and_rem` |
| 31 | prim#4-shifts-rotates | Checked/wrapping shifts; total rotates | I | `::shifts_distinguish_invalid_counts`, `::rotates_are_total` |
| 32 | prim#4-bit-ops | Bitwise ops and inspection total | I | `::bit_operations_and_inspection` |
| 33 | prim#4-no-arith-instance | No `Add`/`Sub`/`Mul`/`Neg` for machine ints | I | prelude declares none; `tests/integration.rs::rejects_missing_instance` (mechanism) |
| 34 | prim#5-float-ieee | IEEE floats, stated families, no `Eq`/`Ord` | I | `::float_nan_behavior`, `::minimum_maximum_propagate_nan`, `::total_order_orders_neg_zero_and_nan`, `::f32_families_match_f64_shape` |
| 35 | prim#6-bool-char | `Bool`/`Char` semantics, short-circuit | I | row 3; `::char_values_and_ordering`, `::u32_to_char_rejects_non_scalars` |
| 36 | prim#7-conversions | Explicit conversions, literal range checks, narrow `Int` default | I | `::conversions_at_range_boundaries`, `::float_conversions`, `::literal_range_checking`, `::unconstrained_literal_is_an_error`, `::literal_patterns_are_range_checked`, `::char_literal_rejects_non_scalars` |
| 37 | prim#8-class-instances | `Copy`/`Eq`/`Ord` instance table | I | prelude + `check.rs` structural `Copy`; `tests/ownership.rs::copy_class_is_structural_and_not_user_extensible` |
| 38 | prim#9-ffi-wasm-simd-correspondence | Layout table is the FFI layout table; Wasm/SIMD lane correspondence | N | naming reconciled by D6; layout table enforcement is P11/P14 |

## foreign-function-interface-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 39 | ffi#0-must-support | External libraries without implying proof of behavior | I | real `foreign import/export ccall` with binding contracts and effect honesty (D68); assumptions recorded |
| 40 | ffi#1-header-not-safe | Generated bindings not automatically safe | I | every binding carries explicit roles/ownership in its contract; nothing is inferred from headers (D66/D67/D69/D70) |
| 41 | ffi#2-binding-specification | Bindings specify widths/offsets/alignment/discriminants/convention/ownership | I | full contract in the interface foreign table (canonical JSON round-trips); `tests/ffi.rs` |
| 42 | ffi#3-pointer-roles | No pointer roles from integer casts | I | integer-to-pointer rejected at unification; roles arise only from foreign results (D69) |
| 43 | ffi#4-effects-accounted | Bindings account for I/O, blocking, divergence, reentrancy, affinity, failure | I | closed explicit rows over the accounted vocabulary, effect honesty enforced (D68) |
| 44 | ffi#4-no-unwind-across-abi | No panics across an unimplemented ABI protocol | I | fault path exits 71 and never unwinds, tested both directions (`tests/ffi.rs`) |

## concurrency-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 45 | conc#0-must-support | Concurrent application code with ownership/effect/cancellation rules | I | structured scopes, local tasks, ownership/effect/cancellation rules (D74-D79); bounded channels remain open (spec section 5) |
| 46 | conc#2-drop-handle | Dropping a task handle must not silently detach | I | handles are `%1` (D76); check-time rejection tested in both shapes (`tests/concurrency.rs::dropping_a_handle_is_rejected`) |
| 47 | conc#6-memory-model-backends | Backends preserve the source memory model | P | native backend implements and documents the model (D80: one OS thread per task, fork-join happens-before, sequentially consistent cancellation); the multi-backend half stays N until P14 |

## modules-and-compilation-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 48 | mod#1-module-packaging | Modules package types, values, ops, associated families, laws, generators | P | multi-module files (D42), generators exported via `import stage`, manifest `staging.generated`; module parameters still rejected (A5) |
| 49 | mod#1-abstraction-restricts-reflection | Abstraction restricts code and reflection equally | N | no reflection; P9 |
| 50 | mod#1-coherent-resolution | Coherent default instances; explicit dictionary selection available | P | cross-module coherence (`interface.rs::coherence_rejects_duplicate_instances`, `tests/modules.rs::instance_coherence_across_modules`); `@{...}` supplies rejected until P6 |
| 51 | mod#1-abstract-type-equality | Functor rules define abstract-type equality | N | no functors/opaque identity; P7/P9 |
| 52 | mod#2-assoc-types-not-semantics | Associated types not mistaken for shared semantics | N | no associated families; P9 |
| 53 | mod#3-associativity-evidence | Associativity evidence tied to the exact dictionary | N | no law modules; P7 |
| 54 | mod#4-compiled-interface-contents | Compiled interface carries signatures, identities, equations, effects, contracts, laws | P | `.sliface` has exports/instances/fixities/foreign/assumptions (no law identities); `interface.rs::round_trips`, `::hash_is_content_identity` |
| 55 | mod#4-dependency-kinds | Dependency kinds distinguished (interface/logical/impl/representation/generated/optimization) | P | interface deps + staleness chain (`::chain_detects_stale_metadata`, `::topo_order_respects_imports_and_detects_cycles`); finer kinds arrive with their features |
| 56 | mod#5-rewrite-rules | Rewrite declarations, sound substitution, bounded termination | I | `rewrite`/`target rewrite` + `justify` (D55-D57: kernel evidence or law methods, effectful rules rejected, defeq guard); `tests/optimizer.rs` |
| 57 | mod#6-fusion-preserves-observation | Fusion preserves observation model incl. divergence/effect order | I | `map_fusion_preserves_values_and_effect_order`, `blocked_fusion_names_the_blocker` (D60) |
| 58 | mod#7-staging-support | Syntax macros, typed code, compile-time computation | I | `stage`/`[||]`/`$()`/`quoteDecls` + token-tree macros over a compile-time interpreter (D43/D46); `tests/staging.rs` |
| 59 | mod#8-hygiene | Binding identity, expansion provenance, checked generated decls | I | capture-avoiding renaming + definition-site qualification, per-node provenance in spans, generated decls under ordinary checks (D46); `tests/staging.rs` |
| 60 | mod#9-stage-scheduling | Compile-time execution scheduled from a dependency graph with cycle diagnostics | I | staging unit graph, statically-apparent emits, rendered cycle chain (D47); `tests/staging.rs::stage_dependency_cycle_shows_the_cycle` |
| 61 | mod#10-compile-time-io | Stage I/O capabilities, resource release, build identity, no secrets | P | file + subprocess capabilities, child cleanup, manifest `staging` identity + snapshot replay, redacted secrets (D45); database/network introspection delegated to subprocess (A6) |
| 62 | mod#12-trust-boundary-report | Implementation reports its trust boundary | P | assumption manifest (row 15); compiler-pass trust now disclosed via the `lowering` checked/trusted split (D63) |
| 63 | mod#13-target-specialization | Target conditions discharged and preserved in the manifest | I | `target.conditions` in the manifest, `trust.level`, checked/trusted lowering split (D63); `manifest_records_trust_level_and_target_conditions` |

## platform-proofs-and-simd-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 64 | platform#1-target-profile | Target profile identifies instruction semantics and deployment assumptions | I | marker-type target model (`VecU32Target t w`) and `Available` evidence (D58) |
| 65 | platform#1-cross-compile-distinguish | Generator-host vs target capabilities distinguished | N | roadmap gap A4 (P16) |
| 66 | platform#2-capability-validity-scope | Capability validity scope defined | I | `Available v` (lexical region + thread, no cached evidence) (D58) |
| 67 | platform#3-apply-rule-in-valid-region | Target rewrites only where capability valid | I | `target_rewrite_applies_in_capability_scope_only` (D59) |
| 68 | platform#3-no-defeq-change | Target rules do not alter definitional equality | I | conversion never extended (D56; `rewrites_do_not_extend_conversion`) |
| 69 | platform#6-scalable-vector-length | Scalable vectors do not assume fixed length | I | `scalable_profile_does_not_pin_a_width` |
| 70 | platform#8-backend-trust-identification | Artifact identifies checked vs trusted lowering | I | `lowering` checked/trusted split in the manifest (D63) |

## webassembly-and-wasi-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 71 | wasm#2-reject-missing-capability | Reject missing capability or select supported lowering | N | P14 |
| 72 | wasm#3-representation | Wasm representation of allocations, refs, function values, type indices | N | P14 |
| 73 | wasm#4-canonical-abi | Bindings implement the selected Canonical ABI | N | P14 |
| 74 | wasm#6-stream-not-productive | WASI streams do not become productive `Stream a` | N | moot until P8/P14 |
| 75 | wasm#7-structured-concurrency-backend | Structured concurrency works on cooperative and multithreaded backends | N | P12/P14 |

## runtime-evolution-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 76 | evo#1-artifact-identity | Artifact identity authenticated by deployment | N | `upgrade` parse-rejected; P13 |
| 77 | evo#6-code-version-retention | Old code versions retained while reachable | N | P13 |
| 78 | evo#7-index-function-meaning | Replacements do not change index functions under checked code | N | P7/P13 |
| 79 | evo#8-incomplete-update-set | Incomplete update sets rejected; no false inline-update claims | N | P13 |
| 80 | evo#10-no-resume-old-over-new-state | No resuming old code over new state without authorized transition | N | P13 |
| 81 | evo#13-target-compat-before-reachable | Instruction-requirement changes established before reachability | N | P10/P13 |

## lazy-computation-and-reclamation-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 82 | lazy#2-later-not-unrestricted-thunk | `Later a` not given unrestricted-thunk semantics | I | usage-multiplicity rule (D36): `Later` never unifies with `Thunk`, binders `%1`/`%aff` only |
| 83 | lazy#4-no-silent-many-to-memo | No silent `%many` thunk to shared memo conversion | I | distinct type forms (`types.rs` `Mult::Many` vs `Ty::Lazy`); structural |
| 84 | lazy#5-cached-type-duplicable | Cached type `a` is duplicable (`Share`) | I | memo bare-use requires `Copy` via `copyOut` (D16/D19); `tests/ownership.rs` copy-coherence tests |
| 85 | lazy#6-eval-r-rule | A memo region's cells never contain `Eval r` | I | row lacks-constraints (`check_terms.rs` memo groups); `tests/integration.rs::memo_group_black_hole_is_rejected`, `::memo_group_mutual_demand_is_rejected` |
| 86 | lazy#6-blackhole-fault | `BlackHole` fault as defense in depth | I | `runtime/src/abi.rs::sl_blackhole` (70) |
| 87 | lazy#9-rejection-cases | The nine required rejection cases | P | demand cycles, double-force, thunk discipline enforced (`tests/ownership.rs`); thread-shared memo refused (`tests/concurrency.rs::a_thread_shared_memo_cell_is_rejected`); `%1` consumption holds on cancellation early-returns (D77); forced-dependent-erased-proof waits for P7 |

## supercompilation-and-jit-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 88 | scjit#2-managed-transformations | Known-application unfolding, case folding, dictionary/handler/index/effect-static specialization, fusion, constant folding | P | done except handler-vtable specialization (opaque barrier, rides P13) and named index-specialization sites (folded via `Vec` chains and ground `decide`); `tests/optimizer.rs` |
| 89 | scjit#3-barriers | Driving stops at effect barriers, opaque boundaries, and rigid indices; never duplicates or drops `%1`/`%aff` values, memo cells, or guards | I | D60 barriers; `linear_value_generalizes_instead_of_duplicating` |
| 90 | scjit#4-preservation | Residual programs rechecked (types, effect order, usage, sharing, productivity, erasure, capabilities); specialization representation-only | I | D63 recheck; `contract_identity_is_unchanged_by_specialization` |
| 91 | scjit#5-bounds | Driving terminates via whistle and fuel; bounds are target profile parameters; the report says where specialization stopped | I | D62; `fuel_exhaustion_preserves_behavior` |
| 92 | scjit#6-jit-invisible | JIT must not appear in types, source, or interface; deoptimization consumption-safe; variant lifecycle via runtime evolution only | N | rides P13 |
| 93 | scjit#7-eval | `eval : Code a -> a` typed by quotes, never evidence, manifest-recorded, profiles MAY forbid | N | typed quotes exist (P9); `eval` itself rides P13/P14 |
| 94 | scjit#8-reporting | Manifest records specialization config, sites, trust level, code-generation use; no secrets or absolute paths | P | `specialization`, `trust.level`, `target.conditions`, `lowering` in the manifest; stopped-at report entries (code-generation use stays with the P13/P14 `eval` item) |
| 95 | scjit#9-trust-levels | Per-artifact trust level identified; full recheck is the default; un-recheckable variants discarded | I | D63 (level 2 default; failing candidates discarded) |

## attribute-annotations-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 96 | attr#1-system | `#[...]` attachment points, argument grammar, compiler/macro ownership, identity split (semantic in contract projection, hints out) | I | `tests/attributes.rs`; identity both directions pinned |
| 97 | attr#2-weights | `#[weight(n)]` with relativity, zero, all-or-nothing; per-alternative storage; both weight reports | I | `tests/attributes.rs` (marginalization at the reference's 2100/100 numbers; partial drop + report) |
| 98 | attr#2.4-transformations | Decision-point weight reconciliation under optimization (dead-alternative conditioning, case merging, inlining) | I | `weights_survive_optimization` (unrescaled conditioning; case-merge rescale to the reference's 3:58:29) (D61) |
| 99 | attr#3-wrappers | `cold`/`likely`/`unlikely` wrappers over weights (0/2000/1), wrapper/manual IR equality | I | `tests/attributes.rs` |
| 100 | attr#4-representations | `repr(C)`/`transparent`/`align`/`packed`/`uN`, C layout algorithm, layout facts in the ABI table and contract hash, packed-borrow rule | I | `tests/attributes.rs`; boundary and runtime field access now use the recorded offsets (P11: `Program.plans`, packed byte-exact loads) |
| 101 | attr#5-inline | `#[inline]`/`#[inline(always)]`/`#[inline(never)]`/`#[noinline]` and function `#[cold]` as erasable hints | I | `tests/attributes.rs` (LLVM emission; duplicates rejected) |

## Roadmap gaps (unplanned requirements)

Flagged by the audit as MUSTs no P1-P15 phase owns; added as phase P16 in
`CONFORMANCE.md` and the roadmap:

- **A1 (core-lang#2-positivity)**: positivity discipline for inductive
  definitions, including ordinary non-dependent `data`, which the compiler
  accepts unchecked today. Needed before P7, not after.
- **A2 (core-lang#5-evidence-invalidation)**: flow-sensitive invalidation
  of evidence about mutable state. No phase claims it; adjacent to P7's
  refinements and P13's state migration.
- **A3 (core-lang#10-temporal-contracts)**: safety/liveness/recurrence/
  stability contract notation with explicit scheduler/crash/fairness
  assumptions. Closest to P12 but unnamed there.
- **A4 (cross-compilation)**: `Usize` width and capability checks against
  the target, not the generator host (prim#2, platform#1). P14's wasm32 is
  a special case; general target selection is unplanned.

## Wording flags (for future decisions)

- Primitive types section 9's "layout table is the FFI layout table":
  naming divergence already reconciled by D6; cross-reference D6 from that
  section when P11 lands.
- `core-lang#6` "capabilities" vs the runtime-capabilities chapter:
  `using (...)` is syntactically identical to an ordinary constraint and
  nothing states how to tell them apart. **Resolved by D21** (P6): the
  binder is a runtime capability iff its class head is the builtin `Io`
  head over a runtime identity `r`; everything else is a class
  constraint. `Io` solves only from lexical providers.
- Lazy computation section 5 phrases duplicability as settled while the
  ownership design note listed `Share` extensibility as open; resolved by
  D16 (structural, non-extensible `Copy`), recorded here for traceability.
- `Later`'s "MUST NOT be an unrestricted thunk" has no named enforcement
  mechanism (kinding vs usage vs erasure); the lazy spec's own open
  decisions acknowledge this. **Resolved by D36** (P8): a
  usage-multiplicity rule (`Later` binders are `%1`/`%aff` only, never
  unifying with `Thunk`), supported by kinding.
