# Spec requirement mapping

Requirement-by-requirement mapping of the specification suite onto the
compiler, per the P15 item in `CONFORMANCE.md`. Produced 2026-10-25 by a
read-only audit of `specifications/*.md` against `compiler/src`,
`compiler/prelude`, `compiler/runtime`, and the test suites; entries
updated against post-audit verification where noted. Swept 2026-10-28
(P15 conformance sweep): every row re-audited against the post-rename
Cinch tree (`.cinch` sources, `.cinchif` interfaces, `cinch_*` externs),
stale statuses re-grounded on landed decisions, and unmapped normative
MUSTs added as rows 107-127.

Status legend: **I** = implemented, **P** = partial, **N** = not yet,
**CV** = cannot verify (no observable way to test today). Evidence is the
enforcing code and the test that exercises it; "structural" means the
property holds by construction rather than by a named test.

## core-language-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 1 | core-lang#1-strict-cbv | Strict call-by-value evaluation, no implicit laziness | I | `codegen.rs` eager word model; thunks only via `Thunk`/`~` (structural) |
| 2 | core-lang#1-scrutinee-once | Pattern matching evaluates its scrutinee once | CV | pattern-matrix compiler in `check_terms.rs` implies it; no test observes evaluation count |
| 3 | core-lang#1-short-circuit | `&&`/`\|\|` short-circuit | I | `parser.rs` desugars infix `&&`/`\|\|` to `if` (D14); `tests/integration.rs::surface_and_or_short_circuit` |
| 4 | core-lang#1-totality-diverge | Reject unsupported totality claims or require explicit `Diverge` | N | `Diverge` enforcement on executable definitions deferred (D32); logical fragment is checked terminologically |
| 5 | core-lang#2-dependent-core | Dependent function/pair types, indexed families, dependent matching, universes | I | Pi/Sigma, universes, indexed `Vec`/`Fin`/`Dec` builtins plus general user families (D27-D31); rigid-index refinement rejected conservatively (D35); `tests/dependent.rs` |
| 6 | core-lang#2-type-in-type | `Type :: Type` must not be admitted | I | stratified `Type 0 : Type 1 : ...`, `Type : Type` rejected (D27); `tests/dependent.rs` |
| 7 | core-lang#2-positivity | Inductive definitions satisfy positivity discipline | I | strict positivity for all `data` (P16 A1 closed); negative and nested occurrences rejected |
| 8 | core-lang#2-termination | Recursive logical definitions satisfy termination discipline | I | `spec`/`proof` purity + self-structural decrease + mutual-cycle rejection (D32); executable `Diverge` enforcement deferred |
| 9 | core-lang#2-coverage-indices | Pattern coverage accounts for indices/impossible constructors | I | index equations solve/refute/unknown; refuted arms rejected as impossible, refuted constructors need no branch (D33); rigid-index cases rejected naming the equation (D35) |
| 10 | core-lang#2-defeq | Definitional vs. propositional equality distinguished; solvers must not extend conversion | I | conversion is beta/iota/alpha/proof-irrelevance only (D28); `decide` never extends it |
| 11 | core-lang#3-logical-fragment | Logical fragment consistency, erasure correctness | I | `proof`/`spec`/`Prop` with erasure (no runtime slots), no evidence from failed checks; `tests/dependent.rs` |
| 12 | core-lang#4-gradual-strengthening | Gradual strengthening of specifications | I | `spec`/`proof` declarations plus refinement obligations in `using` contexts (D261, D282-D287); an ordinary module promises only its own type and effects, added obligations are fresh proof obligations, and assumptions record transitively (row 15); `tests/lattice.rs`, `tests/refinements.rs` |
| 13 | core-lang#4-obligation-statuses | Disproved/unsupported/timeout obligation statuses distinct; tests never discharge universal obligations | I | `decide` folds Checked/Disproved, errors on Unsupported/Timeout, Unresolved runs scoped runtime comparison (D33) |
| 14 | core-lang#4-holes | Deployment profile rejects unresolved required holes | N (by over-rejection) | typed holes categorically rejected (`check_terms.rs`); no hole reaches codegen |
| 15 | core-lang#4-assumption-manifest | Trusted assumptions exposed in a transitive manifest | I | `interface.rs` `Assumptions`, `<entry>.manifest.json` (D4/D9); `tests/modules.rs::interface_round_trips_through_disk` |
| 16 | core-lang#5-evidence-invalidation | Evidence about mutable state invalidated on change | I | flow-sensitive invalidation: Mut-region footprints on Prop-typed evidence, write triggers (refSet/Mut-arg/effect-row/opaque), deferred staleness after solving, `Ref` survival, RawPtr carries no evidence (`de0780c`, D192-D197; `tests/evidence_invalidation.rs`) |
| 17 | core-lang#5-missing-vs-none | Missing fields vs `None` distinguishable | I | records require all fields (`check_decls.rs`); `Option` explicit (structural) |
| 18 | core-lang#6-effects-capabilities-both | Effects and capabilities both represented | I | effect rows honest; capabilities modeled as `Io`-headed runtime identities with lexical supplies (D21); `tests/effects.rs` capability tests |
| 19 | core-lang#6-handler-type | Handler effect typing; multi-shot resumption must not duplicate linear captures | P | `handle ... with` with effect-typed clauses and one-shot `resume` (D23/D26: `%1` captures promoted to exactly-once; call-dispatched handling deferred); `tests/effects.rs` handler and resumption tests |
| 20 | core-lang#7-memory-safety | No use-after-free, double-free, invalid mutable aliasing, data races | I | multiplicities + regions (`tests/ownership.rs::linear_double_use_is_rejected`, `::affine_double_use_is_rejected`, `::borrow_escape_is_rejected`, `::exclusive_borrows_are_lexically_unique`); races are prevented by the P12 capture discipline (D77: `Mut` refused across tasks, untransferred `Bytes`/`Lazy` cells refused; thread-shared memo rejected, `tests/concurrency.rs::a_thread_shared_memo_cell_is_rejected`) under D80's one-thread-per-task model |
| 21 | core-lang#7-borrow-kinds | Shared/exclusive borrows, consuming transfer, synchronized sharing | P | `Ref`/`Mut`/`%1` transfer done (`::scoped_borrow_is_accepted`); named gap: synchronized sharing (approved synchronized structures with invariants) has no implementation, bounded channels remain unimplemented (conc spec section 5) |
| 22 | core-lang#7-copy-semantics | Sharing/copying/cloning defined | I | structural `Copy` (`::copy_class_from_the_vector_filter_shape_works`, `::copy_class_is_structural_and_not_user_extensible`, `::copy_out_coheres_with_multiplicity`) |
| 23 | core-lang#9-coinduction | Coinductive data and sound guarded corecursion | I | `codata` delayed-observation records + `corec` with guard checking (D37/D38); `tests/codata.rs` |
| 24 | core-lang#9-productivity | `Later` force rules prevent unproductive recursion as total; unrestricted filters never present as productive streams | I | unguarded `corec` rejected naming the call; productivity loss typed as `Diverge` (D39, so an unrestricted filter can never claim a productive output); `tests/codata.rs` |
| 25 | core-lang#10-temporal | Safety/liveness/recurrence/stability contracts with explicit assumptions | I | `contract` declarations with `always`/`eventually`/`recur`/`stable` (D198-D205): inductive safety over the D192 write log, bounded liveness from the P12 scope guarantees, bounded recurrence from the D113 structural-depth proof, undecidable cases refused by name; facts in `.cinchif`/`iface_hash` and the manifest `temporal` section (`tests/temporal.rs::contract_facts_hash_both_ways`, `::safety_invariant_holds_across_a_mutating_loop`, `::recurrence_is_bounded_over_structural_recursion`) |
| 26 | core-lang#11-acceptance-manifest | Manifest identifies axioms, external contracts, proof backends, trusted components | P | axioms/assumptions recorded (row 15) with contracts (D198-D205), assertions (D112) and law rows (D285-D287) hashed into `iface_hash`, and `trust.level`/`lowering` trusted components (D63); named gap: the proof backend is the fixed `refl`/`congr`/`decide` kernel (D55) and is disclosed in prose, not a manifest field |
| 107 | core-lang#3-no-evidence-from-divergence | A looping function MUST NOT inhabit an arbitrary proposition; untrusted generated terms pass the same checking rules as handwritten terms | I | `spec`/`proof` bodies are checked pure and terminating, so executable recursion inhabits only computational types (D32); generated declarations run under ordinary checks (D46, `tests/staging.rs::generated_declaration_fails_ordinary_checks`) |
| 108 | core-lang#3-erasure-rules | Prop elimination blocks computational use of erased proof identity; erasure preserves executable semantics; indices/dictionaries/witnesses retained when execution requires; no erase-by-resemblance | I | D31 (`Prp`/`Sort` fields and parameters take no runtime slot, family indices not stored in blocks, Sigma retains its witness) plus proof irrelevance in conversion (D28); `tests/dependent.rs::erased_fields_are_absent_from_runtime` |
| 109 | core-lang#3-ghost-models | Ghost models of owned values MUST NOT duplicate ownership | P | Prop evidence over mutable state is flow-invalidated rather than duplicated (D192-D197, `tests/evidence_invalidation.rs`); named gap: the formal connection between ghost views and borrowing is an open core-design task (core spec section 3) |
| 110 | core-lang#4-decide-explicitness | Runtime checking explicit in the source or public effect and error contract; solver timeouts introduce no silent runtime failures | I | `decide` is a source-level form with explicit statuses: Unsupported/Timeout are check errors, Unresolved runs a scoped runtime comparison (D33); `tests/dependent.rs::decide_obligation_statuses_are_distinguished`, `::decide_success_and_runtime_branches` |
| 111 | core-lang#8-checkable-automation | Inferred termination/induction evidence returns a checkable result | P | structural termination checking is its own checkable result (D32); named refusal: no certificate-generating proof automation exists, the kernel is `refl`/`congr`/`decide` only (D55) |
| 112 | core-lang#10-temporal-assumptions | Temporal contracts MUST state scheduler, dependency, crash, network, and fairness assumptions when relied upon | I | liveness contracts must state their assumptions (`tests/temporal.rs::liveness_must_state_its_assumptions`); stated assumptions join `assumptions.declared` (D198-D205) |
| 113 | core-lang#10-liveness-refinement | Liveness evidence connected to the implementation by a refinement argument; safety refinement alone MUST NOT certify liveness | P | bounded liveness is verified only from the P12 scope guarantees (`tests/temporal.rs::liveness_join_completes_within_the_spawn_scope`, `::liveness_cleanup_runs_within_the_spawn_scope`) and open-ended claims are refused (`::liveness_unbounded_claim_is_refused_naming_the_reason`); named boundary (D201): no general refinement-argument machinery, undecidable claims refuse rather than guess |
| 114 | core-lang#11-trusted-solver-distinction | A trusted solver mode, if offered, is distinguished from certificate checking | N | named refusal: no trusted solver mode exists today; the kernel is `refl`/`congr`/`decide` (D55) and trusted assumptions record in `assumptions.declared` |

## primitive-types-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 27 | prim#2-usize-width | `Usize`/`Isize` at least 32 bits, same width | I | `types.rs` `Scalar::Usize/Isize` (64-bit) (structural) |
| 28 | prim#2-usize-cross-compile | Cross-compilation uses the target's width | I | wasm32 profile: widths keyed on the target in literal checks, lowering, and the manifest (P14) |
| 29 | prim#4-overflow-families | checked/wrapping/saturating Add/Sub/Mul/Neg | I | prelude + `core.rs::PrimOp`; `tests/primitives.rs::checked_overflow_returns_none`, `::saturating_clamps_to_bounds` |
| 30 | prim#4-div-rem-checked-only | Division/remainder only checked | I | `::checked_div_and_rem` |
| 31 | prim#4-shifts-rotates | Checked/wrapping shifts; total rotates | I | `::shifts_distinguish_invalid_counts`, `::rotates_are_total` |
| 32 | prim#4-bit-ops | Bitwise ops and inspection total | I | `::bit_operations_and_inspection` |
| 33 | prim#4-no-arith-instance | No `Add`/`Sub`/`Mul`/`Neg` for machine ints | I | prelude declares none; `tests/integration.rs::rejects_missing_instance` (mechanism) |
| 34 | prim#5-float-ieee | IEEE floats, stated families, no `Eq`/`Ord` | I | `::float_nan_behavior`, `::minimum_maximum_propagate_nan`, `::total_order_orders_neg_zero_and_nan`, `::f32_families_match_f64_shape` |
| 35 | prim#6-bool-char | `Bool`/`Char` semantics, short-circuit | I | row 3; `::char_values_and_ordering`, `::u32_to_char_rejects_non_scalars` |
| 36 | prim#7-conversions | Explicit conversions, literal range checks, narrow `Int` default | I | `::conversions_at_range_boundaries`, `::float_conversions`, `::literal_range_checking`, `::unconstrained_literal_is_an_error`, `::literal_patterns_are_range_checked`, `::char_literal_rejects_non_scalars` |
| 37 | prim#8-class-instances | `Copy`/`Eq`/`Ord` instance table | I | prelude + `check.rs` structural `Copy`; `tests/ownership.rs::copy_class_is_structural_and_not_user_extensible` |
| 38 | prim#9-ffi-wasm-simd-correspondence | Layout table is the FFI layout table; Wasm/SIMD lane correspondence | P | the layout table is the FFI layout table and enforced (row 41; C ABI mappings recorded as target ABI metadata, `abi.layouts`, `tests/attributes.rs`); Wasm lane correspondence is the generic `Vec`-shape machinery (rows 64-67); named gap: no SIMD backend ships, so end-to-end lane lowering is unverified; naming reconciled by D6 |

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
| 47 | conc#6-memory-model-backends | Backends preserve the source memory model | I | D80's model holds on both landed backends: one OS thread per task with fork-join happens-before and sequentially consistent cancellation (native), and the P14 cooperative scheduler preserving the same permitted observations (`runtime/src/coop.rs`; row 75) |

## modules-and-compilation-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 48 | mod#1-module-packaging | Modules package types, values, ops, associated families, laws, generators | I | multi-module files (D42), generators exported via `import stage`, manifest `staging.generated`, module parameters/signatures as module arguments with instantiation units and parameter identity in `.cinchif` (A5, D115-D121); per-instance associated type families remain the D71 gap (row 52) |
| 49 | mod#1-abstraction-restricts-reflection | Abstraction restricts code and reflection equally | N | no reflection; P9 |
| 50 | mod#1-coherent-resolution | Coherent default instances; explicit dictionary selection available | I | cross-module coherence (`interface.rs::coherence_rejects_duplicate_instances`, `tests/modules.rs::instance_coherence_across_modules`); explicit dictionary selection via named module arguments and explicit instances (D119); two instantiations publishing unifiable implicit copies rejected as `ambiguous instance` |
| 51 | mod#1-abstract-type-equality | Functor rules define abstract-type equality | N | no functors/opaque identity; P7/P9 |
| 52 | mod#2-assoc-types-not-semantics | Associated types not mistaken for shared semantics | N | no associated families; P9 |
| 53 | mod#3-associativity-evidence | Associativity evidence tied to the exact dictionary | I | `law` members under `Signature.law` spelling with sample-wise instance checking (D206-D212) and `LawFact` rows in `.cinchif` keyed to the dictionary-owning module (D285-D287); `justify` cites a law method of the exact signature (D55); `tests/laws.rs` |
| 54 | mod#4-compiled-interface-contents | Compiled interface carries signatures, identities, equations, effects, contracts, laws | P | `.cinchif` carries exports/instances/fixities/foreign/assumptions plus contracts (D198-D205), `assertions`/`must_use` (D112), law rows (`Iface.laws`/`LawFact`, D285-D287), `abi.layouts` and `upgrades`; `interface.rs::round_trips`, `::hash_is_content_identity`; named gap: per-instance associated type families are not representable in the interface (row 52, D71) |
| 55 | mod#4-dependency-kinds | Dependency kinds distinguished (interface/logical/impl/representation/generated/optimization) | P | interface deps + staleness chain (`::chain_detects_stale_metadata`, `::topo_order_respects_imports_and_detects_cycles`) with representation staleness forced through `iface_hash` (D86); named gap: imports record as one deduplicated set (D121), so the finer kinds are untagged |
| 56 | mod#5-rewrite-rules | Rewrite declarations, sound substitution, bounded termination | I | `rewrite`/`target rewrite` + `justify` (D55-D57: kernel evidence or law methods, effectful rules rejected, defeq guard); `tests/optimizer.rs` |
| 57 | mod#6-fusion-preserves-observation | Fusion preserves observation model incl. divergence/effect order | I | `map_fusion_preserves_values_and_effect_order`, `blocked_fusion_names_the_blocker` (D60) |
| 58 | mod#7-staging-support | Syntax macros, typed code, compile-time computation | I | `stage`/`[||]`/`$()`/`quoteDecls` + token-tree macros over a compile-time interpreter (D43/D46); `tests/staging.rs` |
| 59 | mod#8-hygiene | Binding identity, expansion provenance, checked generated decls | I | capture-avoiding renaming + definition-site qualification, per-node provenance in spans, generated decls under ordinary checks (D46); `tests/staging.rs` |
| 60 | mod#9-stage-scheduling | Compile-time execution scheduled from a dependency graph with cycle diagnostics | I | staging unit graph, statically-apparent emits, rendered cycle chain (D47); `tests/staging.rs::stage_dependency_cycle_shows_the_cycle` |
| 61 | mod#10-compile-time-io | Stage I/O capabilities, resource release, build identity, no secrets | I | file + subprocess capabilities, child cleanup, manifest `staging` identity + snapshot replay, redacted secrets (D45); database introspection runs over the subprocess capability by design (D72, not a gap) |
| 62 | mod#12-trust-boundary-report | Implementation reports its trust boundary | I | `trust.level` and the `lowering` checked/trusted split (D63) with prim trust rows (D213/D221-D227) and the assumption manifest (row 15); `manifest_records_trust_level_and_target_conditions` |
| 63 | mod#13-target-specialization | Target conditions discharged and preserved in the manifest | I | `target.conditions` in the manifest, `trust.level`, checked/trusted lowering split (D63); `manifest_records_trust_level_and_target_conditions` |
| 117 | mod#5-fp-law-semantics | Floating-point rewrite laws match the selected machine semantics; approximate transforms need an explicit error relation | P | `justify`/law tier enforces kernel or law-method evidence (D55-D57, row 56); named gaps: law instance checking reports floats `not checked` (D211) and no error-relation machinery exists (primitive types section 5 leaves the reassociation contract open) |
| 118 | mod#6-stream-kinds-distinguishable | Productive streams, finite producers, and resource sources remain distinguishable | I | distinct type formers: `codata Stream` (rows 23-24), `WasiStream`/`WasiFuture` with `Diverge` observation rows (row 74), step-form producers; fusion refuses to cross observation models (row 57) |
| 119 | mod#9-no-host-resource-leak | Host pointers, file descriptors, sockets, and compiler-process resources MUST NOT become target runtime resources by quotation | I | compile-time-only types never reach target code (D43); approved constant transport only (`Stage.quoteNat`/`quoteInt`/`quoteBool`, D45); secrets record names only |
| 120 | mod#9-host-target-separation | Host-sized values, layouts, endianness, and subprocess outputs MUST NOT silently determine target semantics | P | widths keyed on the target, never the host (row 28); declared inputs recorded with content hashes (`staging` identity D45, `data_imports` D148/D149); named gap: general endianness/layout transport rules stay an open decision (primitive types section 12) |
| 121 | mod#10-no-auto-retry | Retries MUST NOT assume stage I/O idempotence; failed stages never auto-retried | I | D45: failed stages kill spawned children and are never retried (structural; there is no retry path) |

## platform-proofs-and-simd-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 64 | platform#1-target-profile | Target profile identifies instruction semantics and deployment assumptions | I | marker-type target model (`VecU32Target t w`) and `Available` evidence (D58) |
| 65 | platform#1-cross-compile-distinguish | Generator-host vs target capabilities distinguished | I | `admit_artifact` enforces the assumed baseline per target (D96); widths keyed on the target (P14) |
| 66 | platform#2-capability-validity-scope | Capability validity scope defined | I | `Available v` (lexical region + thread, no cached evidence) (D58) |
| 67 | platform#3-apply-rule-in-valid-region | Target rewrites only where capability valid | I | `target_rewrite_applies_in_capability_scope_only` (D59) |
| 68 | platform#3-no-defeq-change | Target rules do not alter definitional equality | I | conversion never extended (D56; `rewrites_do_not_extend_conversion`) |
| 69 | platform#6-scalable-vector-length | Scalable vectors do not assume fixed length | I | `scalable_profile_does_not_pin_a_width` |
| 70 | platform#8-backend-trust-identification | Artifact identifies checked vs trusted lowering | I | `lowering` checked/trusted split in the manifest (D63) |

## webassembly-and-wasi-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 71 | wasm#2-reject-missing-capability | Reject missing capability or select supported lowering | I | `admit_artifact` reject-or-lower (D96); `tests/wasm.rs` |
| 72 | wasm#3-representation | Wasm representation of allocations, refs, function values, type indices | I | 32-bit words/handles/table-index-free closures with the platform memory obligations in the ABI table (P14) |
| 73 | wasm#4-canonical-abi | Bindings implement the selected Canonical ABI | I | canonical tables + runtime glue (scalars/bytes/handles/enums/flags, single-layer aggregates) asserted exactly on the codec case (D93/D94/D99); uncovered shapes refuse at emission with the shape named |
| 74 | wasm#6-stream-not-productive | WASI streams do not become productive `Stream a` | I | `WasiStream`/`WasiFuture` with `Diverge` observations (D95) |
| 75 | wasm#7-structured-concurrency-backend | Structured concurrency works on cooperative and multithreaded backends | I | P12 threads + P14 cooperative scheduler, both preserving D80 observations |

## runtime-evolution-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 76 | evo#1-artifact-identity | Artifact identity authenticated by deployment | I | versioned identities + `upgrades` chain recorded (D82-D86); "authenticated by deployment" is the deployment mechanism's contract (section 1), outside the compiler |
| 77 | evo#6-code-version-retention | Old code versions retained while reachable | I | live-frame counts keep retired code resident (`tests/evolution.rs`; `runtime/src/evolution.rs`) |
| 78 | evo#7-index-function-meaning | Replacements do not change index functions under checked code | P | record-state fields covered by the D82-D84 field-change classification (retyped fields require a migration); named gap: dependent index-function stability across upgrades is unproven, and `upgrade` evolves monomorphic records only (D83) |
| 79 | evo#8-incomplete-update-set | Incomplete update sets rejected; no false inline-update claims | I | `incomplete_update_set_is_rejected_at_activation` (dispatch unchanged) |
| 80 | evo#10-no-resume-old-over-new-state | No resuming old code over new state without authorized transition | I | `..._never_resume_over_new_state`; authorized reverse transitions tested in `evolution.rs` |
| 81 | evo#13-target-compat-before-reachable | Instruction-requirement changes established before reachability | I | `instruction_requirement_blocks_reachability` (current version unchanged) |

## lazy-computation-and-reclamation-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 82 | lazy#2-later-not-unrestricted-thunk | `Later a` not given unrestricted-thunk semantics | I | usage-multiplicity rule (D36): `Later` never unifies with `Thunk`, binders `%1`/`%aff` only |
| 83 | lazy#4-no-silent-many-to-memo | No silent `%many` thunk to shared memo conversion | I | distinct type forms (`types.rs` `Mult::Many` vs `Ty::Lazy`); structural |
| 84 | lazy#5-cached-type-duplicable | Cached type `a` is duplicable (`Share`) | I | memo bare-use requires `Copy` via `copyOut` (D16/D19); `tests/ownership.rs` copy-coherence tests |
| 85 | lazy#6-eval-r-rule | A memo region's cells never contain `Eval r` | I | row lacks-constraints (`check_terms.rs` memo groups); `tests/integration.rs::memo_group_black_hole_is_rejected`, `::memo_group_mutual_demand_is_rejected` |
| 86 | lazy#6-blackhole-fault | `BlackHole` fault as defense in depth | I | `runtime/src/abi.rs::cinch_blackhole` (70) |
| 87 | lazy#9-rejection-cases | The nine required rejection cases | P | demand cycles, double-force, thunk discipline enforced (`tests/ownership.rs`); thread-shared memo refused (`tests/concurrency.rs::a_thread_shared_memo_cell_is_rejected`); `%1` consumption holds on cancellation early-returns (D77); forced-dependent-erased-proof waits for P7 |
| 115 | lazy#6-profile-blackhole-discharge | A deployment profile claiming the BlackHole fault cannot occur MUST discharge the effect discipline and record its trust boundaries | P | the discharge mechanism exists (the `Eval r` lacks rule, row 85) and trust records into the manifest (`trust.level`/assumption rows, D63); named gap: no profile claims the fault cannot occur today, so only the recording half is exercised |
| 116 | lazy#10-reclamation-observable | Cost-model optimizations (stack cells, demand-count elision) MUST NOT change observable reclamation | N | named refusal: no such optimization is implemented (lazy section 10 is explicit that these are future optimizations) |

## supercompilation-and-jit-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 88 | scjit#2-managed-transformations | Known-application unfolding, case folding, dictionary/handler/index/effect-static specialization, fusion, constant folding | P | done except handler-vtable specialization (named blocker per the P13 deferrals report: dispatch sites can live in callees through `do`/`spawn`, so concrete vtables do not propagate across function boundaries) and named index-specialization sites (folded via `Vec` chains and ground `decide`); `tests/optimizer.rs` |
| 89 | scjit#3-barriers | Driving stops at effect barriers, opaque boundaries, and rigid indices; never duplicates or drops `%1`/`%aff` values, memo cells, or guards | I | D60 barriers; `linear_value_generalizes_instead_of_duplicating` |
| 90 | scjit#4-preservation | Residual programs rechecked (types, effect order, usage, sharing, productivity, erasure, capabilities); specialization representation-only | I | D63 recheck; `contract_identity_is_unchanged_by_specialization` |
| 91 | scjit#5-bounds | Driving terminates via whistle and fuel; bounds are target profile parameters; the report says where specialization stopped | I | D62; `fuel_exhaustion_preserves_behavior` |
| 92 | scjit#6-jit-invisible | JIT must not appear in types, source, or interface; deoptimization consumption-safe; variant lifecycle via runtime evolution only | I | variants install/retire through runtime evolution only; consumption-safe deopt with unbalanced-linear-transfer refusal; nothing in types/source/interface (`tests/evolution.rs`) |
| 93 | scjit#7-eval | `eval : Code a -> a` typed by quotes, never evidence, manifest-recorded, profiles MAY forbid | I | runtime evaluator + build-time reifier (D92); end-to-end case green (`eval [| 40 + 2 |]` = 42) |
| 94 | scjit#8-reporting | Manifest records specialization config, sites, trust level, code-generation use; no secrets or absolute paths | I | `specialization`, `trust.level`, `target.conditions`, `lowering`, the `wasm` section (profiles/adapters/lowerings), and `eval at <site>` assumption rows |
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
| 123 | attr#10-assertion-verified | Safety assertions verified at the definition; violations name the rule and the offending site | I | full vocabulary with hyphenated names and parenthesized arguments (D111, D176-D186); `tests/safety.rs` (24 tests, e.g. `::no_alloc_names_the_constructor`, `::checked_params_names_the_unchecked_use`) |
| 124 | attr#10-assertion-contract-facts | Satisfied assertions recorded in `.cinchif` and hashed into `iface_hash` exactly like `repr` facts | I | `assertions`/`must_use` rows (D112); `tests/safety.rs::contract_facts_hash_assertions_and_must_use_both_ways` |
| 125 | attr#10-bounded-depth | `bounded(n)` depth bounds proven structurally first, conservative call-graph bounds otherwise, report saying which; unboundable claims refused | I | structural named-size proofs vs the literal ladder (D113, D182-D183); `tests/safety.rs::bounded_claims_name_their_proof_method_or_are_refused` |
| 126 | attr#10-jpl-profile | `jpl` profile enforces the full set at the JPL rule numbers; per-definition attributes stricter only; Result must-use default | I | module attribute + `TargetProfile::jpl()` with whole-program summary (D114, D176-D186); `tests/safety.rs::jpl_profile_rejects_each_rule_and_never_looser`, `::jpl_profile_reports_the_whole_program_summary` |
| 127 | attr#3.2-inference-isolation | Weight inference never mixes with explicit weights and never changes program meaning | N | named: no weight inference is implemented (attr section 3.2 is a MAY), so both constraints are vacuous today and pinned here; the all-or-nothing drop (row 97) covers the mixing rule for explicit weights |

## primitive-operations-and-inline-target-ir-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 102 | prim#2-declarations | `prim` declarations with monomorphic closed-row types and inline per-family bodies | I | `prim name :: type where` with verbatim per-family fragments (llvm/asm bodies landed; wasm registered; clr/jvm reserved and named at parse) (D206-D212); `tests/prim.rs::prim_with_llvm_and_asm_bodies_checks_and_compiles`, `::clr_and_jvm_body_slots_are_reserved` |
| 103 | prim#4-validation | Containment validation: signature discipline, no-escape via `uses`, no-unwind, declared effects, family grammar | I | violations named per rule (D206-D212); `tests/prim.rs::escaping_body_fails_containment_naming_no_escape`, `::extra_parameters_fail_containment_naming_signature_discipline`, `::unwinding_body_fails_containment_naming_no_unwind`, `::effectful_body_fails_containment_naming_effects` |
| 104 | prim#5-justification | `justify`/assumed tiers with the law/assumption identity rules (justified = representation-only) | I | law identity in the contract; assumed bodies carry `prim:unit#<bodyhash>` and stale dependents through `iface_hash` (D206-D212, reusing D55 machinery); `tests/prim.rs::justified_prim_records_its_law_identity`, `::assumed_prim_identity_is_the_body_hash` |
| 105 | prim#6-selection | Capability-scoped selection with the baseline rule; selection in the manifest, never the contract | I | cross-family conditional-first then family-order baselines; overlapping conditions error; conditional-only prims reject naming the missing platform; manifest-only recording (D206-D212, D214, D221-D227; reuses D58/D59 machinery); `tests/prim.rs::capability_scoped_selection_is_manifest_only`, `::platform_gated_asm_bodies_select_per_arch`, `::conditional_only_prims_reject_naming_the_missing_platform` |
| 106 | prim#8-reporting | Per-prim report and manifest entries (family, condition, tier, uses, validation) | I | manifest rows carry the family, the verbatim rendered condition, tier, `uses`, and validation result (D206-D212, D221-D227); `tests/prim.rs::capability_scoped_selection_is_manifest_only`, `::assumed_prim_identity_is_the_body_hash` |

## surface-grammar-local-amendments.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 122 | surf#9-tail-constant-stack | Tail recursion MUST run in constant stack (self and mutual); unmarkable groups fall back with the report saying so | I | loop-header self jumps + `musttail` mutual groups (D104, D138-D139; `core.rs::tail_report`, `codegen.rs` loop header); `tail_calls` manifest section with Tarjan SCC marks and fallback reasons (`driver.rs`); tail marking exercised by `tests/safety.rs::recursion_tail_covers_self_and_mutual_groups` |

## Roadmap gaps (unplanned requirements)

Flagged by the audit as MUSTs no P1-P15 phase owns; added as phase P16 in
`CONFORMANCE.md` and the roadmap. All four are now closed (verified
2026-10-28):

- **A1 (core-lang#2-positivity)**: positivity discipline for inductive
  definitions, including ordinary non-dependent `data`. Closed by P7
  (strict positivity for all `data`; negative and nested occurrences
  rejected, row 7).
- **A2 (core-lang#5-evidence-invalidation)**: flow-sensitive invalidation
  of evidence about mutable state. Closed by `de0780c` (D192-D197,
  row 16).
- **A3 (core-lang#10-temporal-contracts)**: safety/liveness/recurrence/
  stability contract notation with explicit scheduler/crash/fairness
  assumptions. Closed by `b25a1a8` (D198-D205, rows 25 and 112-113).
- **A4 (cross-compilation)**: `Usize` width and capability checks against
  the target, not the generator host (prim#2, platform#1). Closed by
  P14 (widths keyed on the target in literal checks, lowering, and the
  manifest; rows 27-28, 65).

## Wording flags (for future decisions)

- Primitive types section 9's "layout table is the FFI layout table":
  naming divergence already reconciled by D6; the D6 cross-reference is
  landed as a dated amendment note on primitive-types-specification.md
  section 10 (2026-10-28 P15 sweep).
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
