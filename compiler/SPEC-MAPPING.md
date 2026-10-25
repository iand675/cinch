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
| 23 | core-lang#9-coinduction | Coinductive data and sound guarded corecursion | N | `codata`/`corec` parse-rejected; P8 |
| 24 | core-lang#9-productivity | `Later` force rules prevent unproductive recursion as total | N | no codata machinery; P8 |
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
| 39 | ffi#0-must-support | External libraries without implying proof of behavior | P | `foreign import ccall` limited to `safelang.prim.*`/`safelang.runtime.*` intrinsics; real C is P11 |
| 40 | ffi#1-header-not-safe | Generated bindings not automatically safe | N | no header generation; P11 |
| 41 | ffi#2-binding-specification | Bindings specify widths/offsets/alignment/discriminants/convention/ownership | N | only intrinsic signatures checked; P11 |
| 42 | ffi#3-pointer-roles | No pointer roles from integer casts | N | `RawPtr`/opaque parse but unchecked; P11 |
| 43 | ffi#4-effects-accounted | Bindings account for I/O, blocking, divergence, reentrancy, affinity, failure | N | effect rows on foreigns parse, not enforced; P11 |
| 44 | ffi#4-no-unwind-across-abi | No panics across an unimplemented ABI protocol | CV | single fault path (`sl_panic`, 71); no foreign-call boundary exists to test |

## concurrency-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 45 | conc#0-must-support | Concurrent application code with ownership/effect/cancellation rules | N | no concurrency primitives; P12 |
| 46 | conc#2-drop-handle | Dropping a task handle must not silently detach | N | no task handles; P12 |
| 47 | conc#6-memory-model-backends | Backends preserve the source memory model | N | single LLVM-text backend, no atomics; P12/P14 |

## modules-and-compilation-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 48 | mod#1-module-packaging | Modules package types, values, ops, associated families, laws, generators | N | flat `module`/`import`/classes only; module parameters rejected (`parser.rs`) |
| 49 | mod#1-abstraction-restricts-reflection | Abstraction restricts code and reflection equally | N | no reflection; P9 |
| 50 | mod#1-coherent-resolution | Coherent default instances; explicit dictionary selection available | P | cross-module coherence (`interface.rs::coherence_rejects_duplicate_instances`, `tests/modules.rs::instance_coherence_across_modules`); `@{...}` supplies rejected until P6 |
| 51 | mod#1-abstract-type-equality | Functor rules define abstract-type equality | N | no functors/opaque identity; P7/P9 |
| 52 | mod#2-assoc-types-not-semantics | Associated types not mistaken for shared semantics | N | no associated families; P9 |
| 53 | mod#3-associativity-evidence | Associativity evidence tied to the exact dictionary | N | no law modules; P7 |
| 54 | mod#4-compiled-interface-contents | Compiled interface carries signatures, identities, equations, effects, contracts, laws | P | `.sliface` has exports/instances/fixities/foreign/assumptions (no law identities); `interface.rs::round_trips`, `::hash_is_content_identity` |
| 55 | mod#4-dependency-kinds | Dependency kinds distinguished (interface/logical/impl/representation/generated/optimization) | P | interface deps + staleness chain (`::chain_detects_stale_metadata`, `::topo_order_respects_imports_and_detects_cycles`); finer kinds arrive with their features |
| 56 | mod#5-rewrite-rules | Rewrite declarations, sound substitution, bounded termination | N | `rewrite` parse-rejected; P10 |
| 57 | mod#6-fusion-preserves-observation | Fusion preserves observation model incl. divergence/effect order | N | no fusion; P10 |
| 58 | mod#7-staging-support | Syntax macros, typed code, compile-time computation | N | quotes/splices/`generate` parse-rejected; P9 |
| 59 | mod#8-hygiene | Binding identity, expansion provenance, checked generated decls | N | no macros; P9 |
| 60 | mod#9-stage-scheduling | Compile-time execution scheduled from a dependency graph with cycle diagnostics | N | staging absent; import cycles do have diagnostics (`tests/modules.rs::import_cycle_is_rejected`) |
| 61 | mod#10-compile-time-io | Stage I/O capabilities, resource release, build identity, no secrets | N | no staging; P9 |
| 62 | mod#12-trust-boundary-report | Implementation reports its trust boundary | P | assumption manifest (row 15); compiler-pass trust waits for P10's preservation discipline |
| 63 | mod#13-target-specialization | Target conditions discharged and preserved in the manifest | N | no target profiles; P10 |

## platform-proofs-and-simd-specification.md

| # | Anchor | Requirement | Status | Evidence |
| --- | --- | --- | --- | --- |
| 64 | platform#1-target-profile | Target profile identifies instruction semantics and deployment assumptions | N | one hard-coded LLVM target; P10 |
| 65 | platform#1-cross-compile-distinguish | Generator-host vs target capabilities distinguished | N | roadmap gap A4 (P16) |
| 66 | platform#2-capability-validity-scope | Capability validity scope defined | N | P10 |
| 67 | platform#3-apply-rule-in-valid-region | Target rewrites only where capability valid | N | P10 |
| 68 | platform#3-no-defeq-change | Target rules do not alter definitional equality | N | moot until P7/P10 |
| 69 | platform#6-scalable-vector-length | Scalable vectors do not assume fixed length | N | P10 |
| 70 | platform#8-backend-trust-identification | Artifact identifies checked vs trusted lowering | N | P10 |

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
| 82 | lazy#2-later-not-unrestricted-thunk | `Later a` not given unrestricted-thunk semantics | P | misuse impossible today (`Later` rejected); positive mechanism is P8 |
| 83 | lazy#4-no-silent-many-to-memo | No silent `%many` thunk to shared memo conversion | I | distinct type forms (`types.rs` `Mult::Many` vs `Ty::Lazy`); structural |
| 84 | lazy#5-cached-type-duplicable | Cached type `a` is duplicable (`Share`) | I | memo bare-use requires `Copy` via `copyOut` (D16/D19); `tests/ownership.rs` copy-coherence tests |
| 85 | lazy#6-eval-r-rule | A memo region's cells never contain `Eval r` | I | row lacks-constraints (`check_terms.rs` memo groups); `tests/integration.rs::memo_group_black_hole_is_rejected`, `::memo_group_mutual_demand_is_rejected` |
| 86 | lazy#6-blackhole-fault | `BlackHole` fault as defense in depth | I | `runtime/src/abi.rs::sl_blackhole` (70) |
| 87 | lazy#9-rejection-cases | The nine required rejection cases | P | demand cycles, double-force, thunk discipline enforced (`tests/ownership.rs::linear_thunk_is_forced_exactly_once`, `::affine_thunk_is_forced_at_most_once`, `::unannotated_thunk_defaults_to_affine`, `::linear_thunk_forced_once_per_branch_is_accepted`); thread-shared memo moot (P12); forced-dependent-erased-proof waits for P7 |

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
  decisions acknowledge this. P8 must pick one and record it.
