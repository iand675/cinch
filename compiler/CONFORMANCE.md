# Conformance matrix

Tracking artifact for turning the spec suite into a working compiler. Every
requirement in `specifications/` and `design/` should end up mapped to an
implementation and a test. Decisions made along the way are recorded here and
in the relevant spec pages.

Legend: **done** (implemented and tested), **partial** (started, gaps noted),
**todo**.

## Acceptance target

The example catalogue is the acceptance suite, per the implementation plan
("each milestone should run the relevant positive and negative cases in the
example catalogue once an implementation exists"):

| Example | Needs | Status |
| --- | --- | --- |
| `01-bounded-index.cinch` | `Nat`, `Fin`, `Vec`, `Option`, `decide`, dependent results | todo |
| `02-vector-filter.cinch` | `Sigma`, `Vec`, dependent pattern matching, `Copy` | todo |
| `03-map-fusion.cinch` | `proof`, `refl`, `congr`, `Eq`, `rewrite ... justify` | todo |
| `04-productive-stream.cinch` | `codata`, `Later`, `corec`, guarded `delay` | todo |
| `05-schema-generator.cinch` | `stage`, `generate do`, `import stage`, compile-time capabilities | todo |
| `06-counter-upgrade.cinch` | `record`, `%1` multiplicities, `upgrade`/migration evidence | todo |
| `07-rejected-programs.cinch` | each listed rejection must be diagnosed | todo |
| `08-platform-simd.cinch` | `U32.wrappingAdd`, `Ref`/`Vec`, `target rewrite`, capability dispatch | todo |
| `09-foreign-codec.cinch` | `signature` with `opaque type`, erased fields, FFI effects | todo |
| `10-wasm-component.cinch` | WIT adapter, `do` on `Result`, `decide`, component effects | todo |
| `11-structured-tasks.cinch` | `using (io :: Io r)`, `spawn`, scopes, `->{Async r, IO r, Cancel r}` | todo |
| `12-runtime-effects.cinch` | `do` with sequencing providers, module parameters, row-polymorphic effects | done (accepts end to end) |

## Feature matrix

### Surface data completeness (P1)

| Requirement (surface grammar / core) | Status |
| --- | --- |
| Tuple expressions and tuple types | done (structural `Ty::Tuple`, boxed product; D1) |
| List literals and list patterns | done (desugar to prelude `Cons`/`Nil`) |
| Literal patterns (int) | done (`EqInt` fall-through chains, source order) |
| Nested constructor patterns | done (pattern-matrix compiler) |
| Case guards `p \| e ->` | done (guard failure falls through; zero-word fallback where coverage unprovable) |
| Record declarations, construction `R { x = e }`, projection `e.x` | done (single-ctor ADT + field table; record patterns by name; `erased`/`codata` rejected) |

### Machine primitives (P2) — see `specifications/primitive-types-specification.md`

| Requirement | Status |
| --- | --- |
| `I8`-`I64`, `U8`-`U64`, `F32`, `F64`, `Char`, `Usize`, `Isize` | done (`Ty::Prim(Scalar)`; `Usize`/`Isize` 64-bit, target profile recorded) |
| `checked*` / `wrapping*` / `saturating*` families | done (full families for Add/Sub/Mul/Neg; checked-only Div/Rem; checked/wrapping shifts; total rotates; bit ops and inspection) |
| Division/remainder only checked; shifts; rotates; bit ops | done |
| Explicit conversions; literal range checking | done (complete conversion matrix; literal elaboration with range checks; `300 :: U8` and unconstrained literals rejected) |
| Float semantics: no `Eq`/`Ord`, IEEE compares, `totalOrder` | done (IEEE 754-2019 `minimum`/`maximum`, `totalOrder`, quiet compares; `Ordering` in the prelude) |

### Modules and interface metadata (P3) — design recorded in
`design/compiled-interface-metadata.md`

| Requirement (modules and compilation spec) | Status |
| --- | --- |
| Multi-file `import` resolution | done (file-stem module rule, dependency order, imported fixities, cycle and missing-file diagnostics) |
| Compiled interface metadata artifacts | done (`.cinchif` canonical JSON with content identity, emitted per module into `build/`, parsed back in tests) |
| Cross-module instance coherence and fixity export | done (unifiability check in the checker plus the metadata fast path; fixity/type/signature/instance exports; cross-module instances resolve) |
| Artifact manifests (assumptions, trusted components) | done (`<entry>.manifest.json` with module identities, union assumptions, `runtime_abi`; per-module `Assumptions` carry the primitive axioms) |

Caveat recorded: the whole project is compiled as one program (one IR, one
link) from the union of sources; compiling a single module against prebuilt
`.cinchif` files without its dependencies' sources is the next step and
needs the checker to consume export tables instead of sources.

### Runtime library (P4)

| Requirement | Status |
| --- | --- |
| Rust runtime crate (`cinchruntime` staticlib + rlib) linked into artifacts; fault policy (`BlackHole` 70, panic 71, div-zero 72) | done (60 tests; `compiler/runtime/`; all `cinch_*` externs `#[unsafe(no_mangle)]`) |
| Mathematical `Nat`/`Int` (bignum) | done (language types on the bignum runtime, unbounded digit-string literals, `Eq`/`Ord`/`Add`/`Sub`/`Mul`, `Neg` for `Int`, truncating `Nat` subtraction, `main :: Int` exit adapter) |
| `Bytes`, string foundations | done (`new`/`len`/`get`/`set`, bounds faults at 71; the `Boxed<Bytes>` payload is `repr(C) { len; data }` with read-only `KIND_BYTES_STATIC` buffers faulting `ReadOnly` at 71 (D300)); strings are IMMUTABLE values (D297: `string-to-bytes`/`string-from-bytes` copy, mutation lives only on plain `Bytes`; the `StrBuf` constructor is module-private, D298) with `"..."` literals as shared read-only data-section constants (D299: zero construction, zero allocation; `#[no-alloc]`/`jpl` accept literals); `pack`/`unpack` at `Data.String` (D301) |

### Ownership, multiplicities, regions (P5) — core spec sections 7 and 3

| Requirement | Status |
| --- | --- |
| `%0`/`%1`/`%aff`/`%many` usage checking | done (path-sensitive usage counts over `case`/`if`/sequences; disjoint projection places split, repeated places stack; `tests/ownership.rs`) |
| `Share` class; immutable sharing | done (`Copy` with `Share` alias, structural and non-extensible, `Bytes`/functions/thunks/`Mut` excluded; `copyOut` coherent with multiplicities) |
| `Ref r a` / `Mut r a`; region escape checking | done (borrow matching binds `Ref` places, `borrow`/`mutBorrow`, escape rejected at scope discharge, lexical `Mut` uniqueness) |
| Thunk multiplicity discipline | done (`force` consumes; `%1` exactly once, `%aff` at most once, unannotated thunks default `%aff`) |

### Effects, capabilities, sequencing (P6) — runtime capabilities spec

| Requirement | Status |
| --- | --- |
| Effect rows on arrows, honesty checking | done (with region-argument label unification across definitions) |
| Handler calculus (`handle ... with`, operation clauses) | done (one-shot `resume`, lexical operation dispatch, effect-typed handlers; per-call dispatch deferred, D23; `tests/effects.rs`) |
| `do` notation; sequencing modules (direct/Result/Option) | done (provider selection via `using` binders with `DirectSequencing`/`ResultSequencing`/`OptionSequencing`, `pure`/`bind` stay class methods, D22) |
| `using (io :: Io r)`, `with (io = ...) in`, `@{...}` | done (capability head `Io`, lexical-only solving, wrong-type supplies rejected; D21) |
| Runtime identity `r` | done (check-level `Ty::Region`, flex/rigid identities, `with`-scope escape checking) |

### Dependent core (P7) — core spec sections 1-5, 8

| Requirement | Status |
| --- | --- |
| Universes, `Type`, stratification | done (`Type 0 : Type 1 : ...`, `Prop : Type 0`; `Type : Type` rejected; D27) |
| Pi/Sigma, dependent functions and pairs | done (explicit `(x :: A) -> B` and `Sigma (x :: A). B`; construction and elimination tested and exercised by examples 01/02) |
| Conversion checking (definitional equality) | done (beta + iota-on-numerals + alpha + proof irrelevance; terminating by construction, no delta; D28) |
| Implicit arguments, inference | done (`forall` binders classified by use; value binders solved by unification and reified; D29) |
| Indexed families (`Vec`, `Fin`), dependent matching, coverage | done except rigid-index refinement (D35): `Vec`/`Fin`/`Dec` builtins (D30), general user indexed families, index equations solve/refute/unknown, refuted arms rejected |
| `Prop`, `proof`, `erased`, `Eq`/`refl`/`congr`, erasure | done (erased fields/args bind `%0` evidence and take no runtime slot; proof irrelevance at runtime; D31) |
| Totality (structural recursion), `decide`, branch evidence | done for the logical fragment (`spec`/`proof`: purity, self-structural decrease, mutual-cycle rejection); `decide` with Checked/Disproved folds, Unsupported/Timeout errors, Unresolved scoped runtime comparison; `Diverge` enforcement on executable definitions deferred (D32) |

### Codata (P8)

| Requirement | Status |
| --- | --- |
| `codata`, `Later`, guarded `delay`/`force` | done (`codata` as delayed-observation records, `Later` distinct from `Thunk` with a usage-multiplicity rule, D36/D37; `tests/codata.rs`) |
| `corec` with productivity checking | done (syntactic guard check: every recursive call under `delay` flowing to a `Later` observation, D38; unproductive recursion rejected naming the call) |

### Staging (P9) — modules spec sections 7-11

| Requirement | Status |
| --- | --- |
| `stage`, `[| |]`, `$()`, `quoteDecls` | done (`Code t` closed code with latent quote-body effects checked at insertion; compile-time interpreter with an 8M-step budget; D43/D44; `tests/staging.rs`) |
| `generate do`, `import stage`, captured replay | done (capability-refined `IoCap` stage I/O over file + subprocess, secrets redacted and never embeddable, refresh/replay/live modes with captured inputs and hash checks, manifest `staging` build identity; D45) |
| Hygienic macros `!` | done (token-tree macros, capture-avoiding renaming + definition-site qualification, per-node expansion provenance in spans, macro-owned attribute names; D46/D48) |

### Rewrites and optimizer (P10) — modules spec sections 5, 6, 13; platform spec

| Requirement | Status |
| --- | --- |
| `rewrite` / `target rewrite` with `justify` | done (D55: kernel evidence or a law method of a `signature`, recorded in `assumptions.declared` when undischarged; type preservation + resource side conditions; effectful rules rejected (D57); conversion never extended (D56 guard) |
| Optimizer with preservation discipline | done (managed transformations with the supercompilation chapter's barriers; residual recheck (D63); map fusion preserves values and effect order, blocked fusion reports the blocker (D60); attribute 2.4 weight reconciliation incl. the 3:58:29 case-merge rescale (D61); `%1` values generalize instead of duplicating; fuel/whistle bounds (D62); `tests/optimizer.rs`) |
| Target profiles, capability evidence, SIMD dispatch | done (marker-type target model and `Available` evidence (D58); `target rewrite` fires only in capability scope (D59); replaceable `Target.detectVecU32` dispatch; scalable profiles do not pin widths; manifest records trust level, target conditions, and the checked/trusted lowering split (D63)) |

### FFI (P11) — foreign function interface specification

| Requirement | Status |
| --- | --- |
| `foreign import ccall` intrinsics (`cinch.prim.*`) | done (namespaces reserved; retired names rejected as retired intrinsics, D73) |
| Real C symbols, `foreign export`, effects, layouts | done (binding contracts with widths/offsets/alignment/convention/symbol identity/roles/ownership in the interface foreign table; closed explicit effect rows with honesty (D68); C fixtures linked through clang; faults exit 71, never unwind; `tests/ffi.rs`) |
| `opaque type`, `RawPtr`, ownership roles | done (`RawPtr`/`Borrowed`/`Retained`/`Owned`/`Out`/`MaybeNull`, integer-to-pointer rejection, deallocator pairing checked program-wide and recorded (D69), nullability by default non-null (D70); aggregates as const-pointer copies, aggregate results rejected (D66); boundary type grammar (D67)) |

### Concurrency (P12) — concurrency specification

| Requirement | Status |
| --- | --- |
| Structured scopes, `spawn`, join/cancel | done (`withScope`/`spawn`/`joinAllOrCancel` with rigid per-call scope regions (D74), n-ary join with first-failure-cancels and full acknowledgment awaiting (D78), cancellation observed at defined points (D80); `tests/concurrency.rs`) |
| Task captures with ownership | done (handles are `%1` with no silent detach (D76); captures follow the P5 share-or-transfer discipline (D77): `%1`/`%aff` transfer exactly once including cancellation early-returns, `Ref`/`Copy` share, `Mut`/untransferred `Bytes`/`Lazy` refused) |
| `Io r` runtime wired to real operations | done (`ioWrite`/`ioRead`/`ioSleep`/`pollCancel` over Rust runtime threads (D81); `Blocking` foreigns run on the task's OS thread, cancellation-blind until return, accounted in the body row and folded into the scope row (D79); one OS thread per task with fork-join happens-before and sequentially consistent cancellation (D80)) |

### Runtime evolution (P13) — runtime evolution specification

| Requirement | Status |
| --- | --- |
| `upgrade` declarations, migration checking | done (`upgrade Old to New where { migrate = f, related = r }` (D82/D83); field-change classification with migration required for incompatible changes; a migration-free `upgrade` is a record view recording schema compatibility evidence in the `.cinchif` `upgrades` table (D84); `#[repr]` changes alter the interface identity, forcing dependent replacement (D86); `tests/evolution.rs`) |
| Versioned dispatch, state migration | done (the runtime evolution arena: epochs and activation publishing, code retention with live-frame counts (spec section 6), complete-update-set gating (section 8), no-old-code-over-new-state with authorized reverse transitions (section 10), target compatibility before reachability (section 13); `cinch_evo_*` externs, `nm`-verified; D85) |
| JIT deopt (tracked item) | done (transparent tiering over the same machinery: variants install/retire through runtime evolution only, consumption-safe deopt with the unbalanced-linear-transfer refusal, nothing in types/source/interface; `tests/evolution.rs`) |

### Wasm and components (P14) — webassembly and wasi specification

| Requirement | Status |
| --- | --- |
| Wasm artifact profile, feature selection | done (wasm32 triple + data layout with target-keyed widths (A4), `admit_artifact` reject-or-lower at artifact production with world-export obligations (D96/D99), cooperative task backend preserving D80 observations, and real builds honoring the profile at emission; `tests/wasm.rs` 11/11) |
| WIT parsing, component adapters | done (WIT subset parser; canonical-ABI tables and the runtime glue for scalars/bytes/handles/enums/flags and single-layer aggregates, asserted exactly on the codec case (D93/D94/D99); `BoundaryError`/`DecodeError` vocabulary; `WasiStream`/`WasiFuture` with degraded `Diverge` observations (D95); `eval` over the reified runtime evaluator with its end-to-end case green (D92); variant packaging round trip through the P13 machinery (D100); manifest `wasm` section) |

### Acceptance catalogue (P15)

Snapshot 2026-10-25 (`cinch check examples/source/*.cinch`, entry rows
update as the slices land; 07 must reject, the rest must accept):

| Program | First rejection | Owning slice |
| --- | --- | --- |
| 01-bounded-index | accepts (`cinch check` exit 0; `run` needs a `main` per D20) | done |
| 02-vector-filter | accepts (`cinch check` exit 0; refactored to `decide`/computed-index dispatch per D35 (D231)) | done |
| 03-map-fusion | accepts (`cinch check` exit 0; index lambdas + named-helper proof reduction + structural-induction `case` landed (D230/D270); the example carries its own `mapFusion` proof and cites it in `justify`) | done |
| 04-productive-stream | accepts (`cinch check` exit 0) | done |
| 05-schema-generator | accepts with its declared build inputs (`SCHEMA_DATABASE_URL` + a schema tool via `CINCH_SCHEMA_TOOL`); without them stops at the correct secret refusal (D45) | done (A6) |
| 06-counter-upgrade | accepts (`cinch check` exit 0) | done |
| 07-rejected-programs | rejected at parse (correct) | must stay rejected |
| 08-platform-simd | accepts (`cinch check` exit 0; its own theorem module states the instruction law as an obligation and `justify` cites it (D228)) | done |
| 09-foreign-codec | accepts (`cinch check` exit 0; its `DecodeError` comes from the codec adapter vocabulary) | done |
| 10-wasm-component | accepts (`cinch check` exit 0; `BoundaryError` from the WIT adapter vocabulary) | done |
| 11-structured-tasks | accepts (`cinch check` exit 0; its sketch vocabulary `Request`/`LoadError`/`fetch` is provided by the prelude over `ioRead`, D81) | done |
| 12-runtime-effects | accepts (`cinch check` exit 0; the parameter block instantiates at import (D115-D121) and the sketch vocabulary comes from the prelude over `ioRead` in dash spellings (D120/D228 sweep)) | done |

## Unplanned requirements (P16)

Four spec MUSTs have no owning phase in P1-P15 (found by the 2026-10-25
audit, `compiler/SPEC-MAPPING.md` "Roadmap gaps"):

| Requirement | Note |
| --- | --- |
| Positivity discipline for inductive definitions | done (P7: strict positivity for all `data`; negative and nested occurrences rejected) |
| Evidence invalidation over mutable state | done (A2/`de0780c`, D192-D197: flow-sensitive; Mut-region footprints, write triggers, deferred staleness; `tests/evidence_invalidation.rs` 12/12) |
| Temporal contracts (safety/liveness/recurrence/stability) | done (A3/`b25a1a8`, D198-D205: `contract` declarations with `always`/`eventually`/`recur`/`stable`; inductive safety and footprint stability from the D192 write log; bounded liveness from the P12 scope guarantees; bounded recurrence from the structural-depth proof; undecidable cases refused by name; facts in `.cinchif`/manifest) |
| Cross-compilation (target width/capabilities) | done (P14: `Usize`/`Isize` widths keyed on the target in literal checks, lowering, and the manifest (A4)) |
| Module parameters and signatures as module arguments | done (A5: `module M (p :: T, ...) where` with dictionary and value parameters, import-time instantiation units `import M (args) as N` with computed `unit_id`, instance coherence preserved (implicit resolution unchanged; explicit dictionaries coexist), parameter signatures in `.cinchif` with instantiation as representation (D115-D121); example 12 accepts) |
| Schema generation / database introspection API | done (P11+A6: `SchemaSnapshot`/`Database`/`SchemaRead` over the subprocess capability, `introspectConsistently`, `CodecGenerator.derive`; example 05 accepts with its build inputs) |

### Language surface DX (P19)

Specified in `specifications/surface-grammar-local-amendments.md`
sections 8-10 and `design/recursive-bindings-and-tail-calls.md`,
2026-10-26, at the user's direction: dash identifiers, ergonomic
negative literals, multi-binding `let`/guarded `letrec`, constant-stack
tail recursion (including mutual), class-desugared list/map literals,
and dash-style prelude function names.

| Requirement | Status |
| --- | --- |
| Dash identifiers and the minus whitespace contract (D102) | done (lexer + ast; whole minus table unit-tested (D128/D129); digits excluded from the right of a glue dash so `x-1` subtracts) |
| Multi-binding `let` and guarded `letrec` (D103): recursion only under lambda/`~`/`delay`, eager cycles rejected with the fix named | done (fixpoint boxes with black-hole fault 70 (D131-D137); `tests/letrec.rs` 9/9) |
| Tail recursion in constant stack, self and mutual (D104) | done (loop-header self-jumps + `musttail` mutual groups; 1,000,000-deep proofs; `tail_calls` manifest section with Tarjan SCC marks and fallback reasons (D138-D139); `tests/surface.rs` run-tests) |
| `[...]`/`{k => v}` literals desugaring to `FromList`/`FromMap`, with the prelude `Map` (D105) | done (`FromList`/`FromMap` classes, `Map` BST, expected-type instance selection, D92 fold pass with `literal_folds` boundary (D142/D147); `tests/surface.rs`) |
| Dash-style prelude function names (D106) | done (all 911 prelude-declared functions/methods renamed; compiler-side name references moved; builtin surface values and intrinsic symbols unchanged (D130)) |
| Export declarations and re-exports (`export module X`, `export A.b`); imports private until re-exported (D107) | done (surface is exactly the named set with `export` decls (D143/D146); instances/fixities/foreign table stay module-owned) |
| Import renaming and selection (`import X as Y`, `import X (a as b)`); duplicate imports error naming both sources (D108) | done (a rename is the name everywhere (D144); duplicate rule over explicit imports (D145)) |
| File-type import converters: extension-keyed compile-time conversion (`import data "config.json" as config`), built-ins json/txt/bytes, plugins as stage modules with provenance (D109) | done (json/txt/bytes built-ins with budget/depth caps, declared-type decode, `data_imports` provenance (D148/D149); plugin converters deferred: unknown extensions name the built-ins) |
| Collection literals fold at compile time to direct construction (macro expansion + stage evaluation + constant emission); runtime class call otherwise; fold boundary reported (D110) | done (fold pass in `finish_fn` via the D92 evaluator into P14 constants; boundary in `StageReport.literal_folds` and the manifest; semantics-transparent fallback (D147)) |

### Safety assertion attributes (P20)

Specified in `specifications/attribute-annotations-specification.md`
section 10 and `design/safety-assertion-attributes.md`, 2026-10-26, at
the user's direction: compile-time-verified assertions (tail
recursion, non-allocation, stack bounds) and the JPL "Power of 10"
rule mapping (six rules inherent to the language, the rest asserted).

| Requirement | Status |
| --- | --- |
| Assertion vocabulary: `#[recursion]`, `#[stack]`, `#[no-alloc]`/`#[alloc "once"]`, `#[max-complexity]`/`#[max-lines]`, `#[assert-min]`, `#[checked-params]`, `#[no-macros]`, `#[no-raw-ptr]`, `#[deny-warnings]` (D111) | done (full vocabulary; hyphenated names and parenthesized arguments (`#[recursion("tail")]`) per D184; `tests/safety.rs` 24/24) |
| Verified at the definition; violations name the offending site; satisfied assertions are contract facts in `.cinchif`/`iface_hash` like `repr` (D112) | done (`assertions`/`must_use` rows; identity pinned both ways; manifest `"safety"` section) |
| `bounded(n)` depth bounds via structural proofs first, conservative call-graph bounds otherwise, report saying which (D113) | done (structural named-size proofs vs the literal ladder (D182/D183); numeric-over-structural refused and pinned as a negative) |
| The `jpl` profile enforcing the whole set at rule numbers, never looser per-definition (D114); Result must-use default | done (module attribute + `TargetProfile::jpl()`; whole-program summary; `let _ = e` rejects rule 7, `ignore` passes) |

### LLVM contract surface (P27)

Specified in `design/llvm-contract-attributes.md`, 2026-10-26 at the
user's direction (what else like `musttail` to adopt from LLVM): the
pattern is translation - Cinch's analyses already prove most of
LLVM's attribute vocabulary. Implemented 2026-10-27 by
`LlvmContractSlice`/`-2` (recovered from a provider interruption).

| Requirement | Status |
| --- | --- |
| Effect-derived function attributes (`memory(none) speculatable`, `memory(read/readwrite)`, `nofree`, proven-only `willreturn`) | done (scan-proven and transitive; restated under P20/totality rows for checked definitions (D189)) |
| Usage-derived parameter attributes (`captures(none)`/`nonnull`/`noundef`/`align`, `!range`, `!align`) | done (presence/absence pairs; `Fin`/full-width ranges refused as unsound (D187)) |
| `llvm.assume` with operand bundles from P20 assertions and repr facts | done (no `range` bundle exists in this LLVM; ranges via the return attribute + `!range` (D188)) |
| `freeze` at untrusted boundaries; `noreturn` + `unreachable` fault audit | done (every untrusted ingress; all fault paths audited) |
| Aggregate FFI spelling | done as `byref(%__sl_layout_N)` + `sret`, NOT `byval` (disassembly proof: `byval` segfaults against C struct-by-value; `byref` preserves the C convention (D190)); `immarg`/`llvm.sideeffect` noted for P18 |

Evidence: `tests/llvm_contract.rs` 14/14 IR-observation tests, each
attribute present where proven and absent where unproven (catching
over-claiming too), including a real clang-compiled C round trip at
the aggregate boundary. Suite: 524 compiler + 107 runtime.

### Editor tooling (P23)

Committed `e9d9496` (`tooling/` only), 2026-10-27 by
`EditorToolingSlice`: the tree-sitter grammar (an all-external scanner
ported from `lexer.rs`/`layout.rs`; generated parser gitignored so the
corpus cannot pass against a stale parser), 88 corpus assertions
(minus table, dash names, ticks, prim/contract forms, every layout
case), vim/emacs/sublime/zed/vscode-cursor integrations with
spec-reserved keywords verbatim, and `check-grammar-drift.sh`
(regenerate + test, nonzero on divergence). Mid-run the grammar parsed
all 59 real `.cinch` files with zero ERROR nodes. The LSP is invoked
through `tooling/lsp-driver/` (`LspServer::run` over stdio; the
`cinch lsp` subcommand does not exist yet and the wrapper
auto-switches if it lands - tracked). Decisions D238-D244.

### Interned symbols (P24)

Specified in `specifications/primitive-types-specification.md`
section 9 and `specifications/surface-grammar-local-amendments.md`
section 12, 2026-10-26, at the user's direction: interned symbols as a
core type with tick literal syntax (the Template Haskell name-quote
correspondence: one lexeme in expression/pattern, staging `Name`, and
`''T` type-name contexts).

| Requirement | Status |
| --- | --- |
| `Sym`: word-handle interned cell; `intern`/`sym-name`; `Eq` = identity $O(1)$, `Ord` by name (deterministic); no machine-scalar confusion (D122) | done (builtin core type; kind-tagged leaked cells; four `nm`-verified externs; `Data.Sym` home; `tests/sym.rs` 10/10 (D232-D237)) |
| Tick syntax: `'foo` symbol literals (constants via compile-time interning and the P14 constant emission), `'a'` char precedence rule, `'foo`/`''T` name quotes in staging/macro positions (D123) | done (literal char rule taken literally (D233/D234); name lexemes survive macro token trees and splice-time reification works; the full D46 `Name` member is tracked separately) |
| Type-level symbols (promoted `'foo` for labels/schema names) deferred with the spelling reserved (D124) | done (the spelling is reserved and enforced (D234); promotion itself stays deferred pending a consumer (A6)) |

- **D122 (P24 Sym)**: `Sym` is a core runtime type: one-word handle to
  a process-wide interned cell; `intern : String -> Sym`,
  `sym-name : Sym -> String`; `Eq` is identity ($O(1)$), `Ord` is by
  name (deterministic across runs, never insertion order); `Show`
  prints `'foo`; no numeric classes; FFI takes `sym-name`/`Bytes` at
  the boundary; nothing exposes a cell address as data.
- **D123 (P24 tick syntax)**: one tick lexeme in three contexts (the
  Template Haskell name-quote correspondence): `'foo` is a `Sym`
  literal in expression/pattern position (interned at compile time and
  emitted as a constant via the P14 path; `intern` of an equal name
  resolves to the same cell), `'foo` is a value `Name` and `''T` a
  type `Name` in staging/macro positions (feeding D46's `Name`
  signature member); char literal precedence is lexical and
  deterministic: tick, one character, tick means `Char`.
- **D124 (P24 type-level symbols deferred)**: promoted `'foo` at the
  type level (labels, schema column names) is deferred with the
  spelling reserved; the likely first consumer is the A6 schema
  vocabulary.
- **D126 (P25 blocker, A7 higher-kinded types)**: the Category family
  is blocked on an unimplemented spec requirement: the checker cannot
  apply type variables (`f a` in a signature), cannot take partially
  applied type constructors as class/instance arguments (`Functor
  List`), has no `(->)` former, and has no kind binders for signature
  parameters. The modules specification section 1 already requires
  higher-kinded parameters, so the fix is a compiler slice (A7,
  scheduled behind P19): var-headed application with beta-reducing
  substitution, partially applied tycons as class arguments, `(->)`,
  kind binders. The defunctionalized encoding is REFUSED: it violates
  the D125 scheme (classes over type constructors live in
  `Category.*`) and would rewrite every generic caller at cutover.
  Same slice fixes the dotted module-name bug (`imports_of` keys
  imports by the last path segment while modules register under their
  full dotted name, so `module Category.Functor` never resolves);
  single-segment module headers are the accepted temporary deviation.
  CategoryAgent's probe signatures become the A7 acceptance fixtures.
- **P25 progress (reported)**: the Algebra family landed
  (`Algebra.Semigroup`/`Algebra.Monoid`, six instance-carrying
  newtypes, 17 `lib_algebra.rs` tests: laws exhaustive over sample
  triples). Two more checker gaps found and folded into A7:
  `check_instance` substitutes the class parameter into instance
  contexts WITHOUT capture avoidance (`using (sa :: Semigroup a) =>
  Semigroup (Dual a)` silently rewrites `sa` to the instance head;
  workaround is renaming params), and nullary class methods are
  rejected (`ty.arity() == 0`, so `mempty :: Unit -> a` per the
  prelude's own convention). Cross instances of prelude types are
  parked in the class modules and move to `Data.*` homes at the
  prelude split; `Nat` deliberately has no bare Semigroup (Sum/Product
  choose its monoids).
- **D128 (P19 dash glue detail)**: a glue dash requires a non-digit
  identifier character to its right (any identifier character left);
  digits inside a word (`x1`, `pow10`) continue identifiers, so `x-1`
  and `1-2` subtract while `int-digits-into` glues.
- **D129 (P19 negative-literal context)**: the literal sign applies
  when the `-` is followed by digits and preceded by whitespace, input
  start, or a non-closing delimiter; after `)`, `]`, `}`, `"`, `'`, or
  an identifier character it is the operator (`f ()-1` subtracts).
- **D130 (P106 rename scope)**: the dash rename covers the prelude's
  declared functions and methods only; compiler-installed builtin
  surface values (`newIO`, `withScope`, `ioWrite`, `quoteNat`,
  `readFile`, the staging API) keep their spellings, and dotted
  presentation method parts follow the dash spelling
  (`U32.wrapping-add`).
- **D131 (P19 letrec lexing)**: `letrec` lexes as a `Letrec` marker
  plus `Let` sharing one span (the `$(` precedent), so `layout.rs` is
  untouched and the `let` layout block opens at the first binding.
- **D132 (P19 guardedness on core)**: guardedness is decided on the
  elaborated core (unique locals; `Prim(Delay)` arguments are the
  delay guard); an unguarded cycle edge is rejected while acyclic
  unguarded uses are allowed; `force`-consumed guards are not cancelled
  syntactically and such programs reach the runtime black-hole fault.
- **D133 (P19 letrec binding forms)**: name equations (`x = e`, `f
  pats = body`, lambda-valued `=` compiles as the function form) and
  `~` memo bindings; destructuring binds are rejected by name.
- **D134 (P19 fixpoint boxes)**: every named value member of a
  `letrec` group gets a uniform two-word fixpoint box (order-free
  bindings); an early read is the black-hole fault (exit 70), never
  garbage.
- **D135 (P19 thunk usage interaction)**: `Ty::Thunk`'s `%aff` default
  rejects self-referencing thunk-typed values at usage check before
  the knot runs (a pre-existing rule, orthogonal to `letrec`).
- **D136 (P19 staging boundary)**: fixpoint boxes exist on the runtime
  path only; evaluating or reifying one at compile time is a named
  compile-time error.
- **D137 (P19 cps letrec)**: `cps_letrec` elaborates the group in
  ordinary style before the continuation; a member body that itself
  dispatches an operation elaborates without a resumption, as local
  lambda bodies do.
- **D138 (P19 tail-call manifest)**: the `tail_calls` manifest shape is
  `{"groups": [{"members": [...], "mark": "marked" | "fallback
  (<reason>)"}]}`; marking covers intra-group self/mutual tail edges
  only (the spec's guarantee), with reasons like `applied to N
  arguments, unit arity M`; dynamic tail calls and non-member calls do
  not affect a group's mark, and a dynamic cycle through a first-class
  self value is outside the static claim.
- **D139 (P19 uniform loop header)**: every function body runs under
  the loop header (one extra block per unit) so the self-jump path is
  uniform; non-recursive units get no special casing.
- **D140 (P19 bytes constant declare)**: `cinch_bytes_from_data` is
  declared `(i64, i64)`; every call site passes the pointer as an i64
  word and the native ABI passes it identically (it was previously
  called undeclared, which clang rejects).
- **D141 (P19 eval-VM bytes sharing)**: the compile-time VM's `Bytes`
  values are shared reference-counted buffers mirroring the runtime's
  in-place mutation (copy-on-write would silently change semantics
  under aliasing).
- **D142 (P19 map-literal default)**: map literals in infer position
  default to `Map k v`, symmetric with the list default (currently
  they reject naming the unlanded `from-map` class until the wiring
  lands).
- **D143 (P19 export surface)**: with `export` declarations a
  module's surface is exactly the named set (own names must be listed);
  without them, declared names as before. Instances, fixities, and the
  foreign table stay module-owned (coherence/contract facts, not
  names).
- **D144 (P19 renames)**: a rename is the name everywhere (`import X
  (a as b)` gives bare `b` and `X.b`; `a` is unreachable); selection
  and alias cannot combine in one import.
- **D145 (P19 duplicate rule)**: the duplicate-name rule covers
  explicit imports only; the prelude is the implicit base layer that
  explicit imports shadow (counting it would paralyze imports once
  library modules define overlapping names); own names shadow imports
  silently.
- **D146 (P19 re-exports)**: `export A.b` re-exports as `b`; `export
  module A` re-exports everything the module imports from A under its
  in-scope names.
- **D147 (P19 fold fallback)**: compile-time folding is
  semantics-transparent: non-stage-known constructions, evaluation
  failures (budget, faults, unimplemented prims), and non-reifiable
  results keep the runtime class call (`folded: false`); only a
  successful constant reify replaces the call. Loud failure was
  rejected: the evaluator's completeness must not gate builds of
  unrelated programs.
- **D148 (P19 Json family)**: the `Json` import value is a single
  self-contained family (`JArr`/`JArrEnd`, `JObj`/`JObjEnd`): the
  positivity discipline rejects nested and mutually recursive shapes,
  and data declarations convert in declaration order (no forward
  references).
- **D149 (P19 converters)**: converter extensions are `json`/`txt`/
  `bytes` exactly (a file must be named accordingly); plugin
  converters (stage modules per extension) are deferred and an unknown
  extension is a build error naming the built-ins; content hashes use
  the codebase's `content_hash` (consistent with `source_hash`/
  `StageInput`).
- **D150 (P25 NonEmpty via delegation, user direction)**: the bespoke
  `Data.NonEmpty` is dropped after A7 for a newtype-style
  `repr(transparent)` wrapper that delegates instance information to
  the container by dictionary coercion (zero cost; safe by D86's
  representation identity). Delegation is opt-in PER CLASS because law
  transfer is not blanket (Semigroup transfers; Monoid does not, since
  `mempty` is empty), and the P18 law-axiom layer checks transfer per
  derived instance. The non-emptiness invariant lives at the
  smart-constructor surface with the raw constructor kept out of the
  export list (D143). The same delegation form generalizes to
  sub-kind wrappers (Sorted, NonZero, Ascii).
- **D151 (P28 refinements, user revision of D150)**: sub-kind types
  (`NonEmpty`, `Sorted`, `NonZero`, `Ascii`) are REFINEMENTS, not
  wrappers: a refinement is a decidable predicate over the container's
  values whose witness is `Prop`-erased (D31), so the runtime
  representation IS the container (zero cost, no delegation code).
  Discharge is explicit in the `Dec`/`decide` world (total
  constructors like `from-list : List a -> Option (NonEmpty a)`, or a
  `Dec` argument that erases); no SMT and no refinement inference.
  Instances are the container's own methods, valid exactly when they
  PRESERVE the refinement (Semigroup for NonEmpty yes; Monoid no);
  the P18 law layer checks preservation per instance. True newtypes
  (Sum/Dual/Endo: distinct identities by design) keep the D150
  delegation form for the rare borrowed-instance case. Full design:
  `design/refinement-types.md`.
- **D152 (A7 naming, lisp-style per D106)**: the Functor method is
  `map` (not `fmap`); `Monad` has no `return` method (`return` is
  handler-clause syntax; Monad is `>>=`/`join` with `pure` on
  `Applicative`), and the monad laws state `pure` under an added
  `Applicative f` constraint, which exercises the bare-`pure` rule.
- **D153 (A7 bare-pure resolution)**: a `pure` carried by an in-scope
  non-provider class constraint answers a bare `pure` first (reached
  through the constraint's dictionary); otherwise the D22 provider
  meaning stands (`is_provider_pure` intercepts; first-registered
  provider wins); `Applicative.pure` is the qualified spelling. `do`
  providers are untouched.
- **D154 (A7 dollar in operators)**: `$` may continue an operator run
  (`<$>` is spellable) but not start one: a leading `$` stays reserved
  application and `$(` stays the splice introducer. The grammar's
  operator terminal is amended accordingly.
- **D155 (A7 kinds)**: unannotated signature/data parameter kinds
  infer to the annotated form (shared binder-kind resolution,
  annotation-vs-inference equality checked); `(->) r` is the pure
  total arrow (default multiplicity, pure row) with effectful arrows
  distinct; `(,)`/`(,,)` formers partial-apply structural tuples;
  record fields keep the full-application rule (`field :: Result e`
  rejects).
- **D156 (A7 Traversable shape)**: `Traversable`'s methods state
  `Applicative f` per method (without it a one-parameter instance is
  unimplementable); instances use the effect dictionary, and the
  first-error short-circuit order is preserved by the left bias of the
  `Result`/`Option` applicatives under traversal-order nesting.
- **D157 (A7 visitor instances)**: `Identity`/`Const` instances are
  defined through their plain-function bodies (the headers' designated
  plan); `Const` gets `Functor` only, its `Applicative` waits for the
  prelude split's instance homes (it needs `Monoid` on the payload).
- **D158 (A7 law-statement witnesses)**: the applicative-homomorphism
  test takes an `f b` witness; without it `f` is undetermined and
  dictionary resolution is ambiguous. Check-only law statements keep
  the two-argument shape.
- **D159 (A7 instance placement)**: instance families for
  `Prelude.cinch` types are parked in the class modules with the
  deviation stated in each header (they move to `Data.*` homes at the
  prelude split); `((->) r)` instances are structural and permanent;
  `Data.Compose` ships as type plus `get-compose` only.
- **D160 (A7 method-constraint conversion)**: a class method's own
  `using` constraint converts against the class parameters plus the
  method's accumulated binders (previously class parameters only,
  which rejected `traverse`'s `using (app :: Applicative f)` naming its
  own `forall f`).
- **D161-D174 (P25 integration, reported)**: `Data.Machine` groups the
  machine scalars' vocabulary; `Add/Sub/Mul/Neg/Div/Rem` live in
  `Algebra.Arithmetic` (operation classes, not structures) with `Bits`
  in `Algebra.Bits`; `Effect.Concurrent` holds `Request`/`LoadError`/
  `fetch` and `Effect.Io` the config vocabulary; P10 vocabulary splits
  into `Data.Producer`/`Data.Target`; `Semiring` is `zero`/`one`/`add`/
  `mul` (both monoids named on one carrier, so bare `Nat`/`Int` are
  Semirings where they cannot be Semigroups; no Num tower; machine
  scalars excluded) with instances in `Data.Nat`/`Data.Int`; `Group` is
  `invert` (`Group Int` in `Data.Int`, `Group (Dual t)` with its type,
  structural pairs/functions in `Algebra.Group`; `Nat` excluded,
  truncated subtraction); `Text.Show` is created fresh (its instances
  live with the conversion family to avoid an import cycle; `Show
  Bool`/`Show Ordering` landed with string literals (D291)); `Text.Codec` states
  the A6 `encode`/`decode` pair (D65's covered set, round-trip law);
  `Data.Set` is a documented unbalanced BST (duplicate insert keeps one
  copy) and `Data.HashMap` a documented fixed 8-bucket spine (first
  pair wins); `Sym` is skipped in the split and `Hash Sym` is
  identity-based, deferred to `Data.Sym`; an explicit source module
  shadows an embedded family copy (family-instance heads a shadow
  cannot satisfy drop rather than fail); instance heads convert after
  all modules are collected (the collection knot); forced export
  grammar: dotted module/member and parenthesized operator items,
  `export module X` matching full name or alias; WIT vocab names join
  the facade's export surface. Compatibility gate held byte-identical
  at a clean worktree (01/04/06/09/10/11 check; 02/03/05/08/12 fail
  exactly as at baseline; 07 rejects). Named gap: `tests/prelude_split.rs`
  unwritten (continuation). Proposed facade trim list recorded in the
  report (implementation helpers).
- **D175 (P25 facade trim, approved pending execution)**: the 22
  implementation helpers on the proposed trim list (`utf8-*` internals,
  `list-byte-count`/`bytes-fill`/`bytes-from-list`,
  `nat-digits-into`/`int-digits-into`/`pow10`,
  `copy-bytes-into`/`compare-bytes-from`/`string-length-from`/
  `string-char-at-from`/`string-byte-index-from`, `prod-fold-go`/
  `prod-take-step`) leave the curated facade: every one is behind a
  public wrapper, no `.cinch` source anywhere references them, and
  family-internal cross-module users import the homes directly. The
  facade surface is pinned at 1016 names by `tests/prelude_split.rs`
  (export list equals the checked surface name for name; every name
  resolves through the implicit layer; `invert`/`show`/`hash` are
  correctly absent and home-reachable). Execution of the trim rides
  the P15 sweep, which re-verifies the examples against the trimmed
  surface.
- **D176-D181 (P20 implementation, reported; detail in
  `history://SafetySlice`)**: the full assertion vocabulary with its
  argument grammar and placement rules verifies at the definition with
  offending-site errors (D111/D112 live); satisfied assertions become
  contract facts as `assertions` + `must_use` rows in `.cinchif` and
  `iface_hash` (the `repr` precedent); recursion/stack group
  verification reuses D104's `core::tail_report` plus a `core::calls_to`
  scan; `bounded(x)` proves structural depth first
  (`decrease_bound` size-index detection), falls back to conservative
  call-graph bounds with the report saying which, and REFUSES what it
  cannot bound (D113 live); the Result must-use default rejects
  discards without `ignore` (D114 live); the `jpl` profile exists both
  as a module attribute and `TargetProfile::jpl()` with the
  whole-program summary in `StageReport.safety`. Named follow-ups
  (continuation): the `tests/safety.rs` acceptance matrix (source
  syntaxes gathered), the manifest `"safety"` section, and empirical
  tuning of message needles and `decrease_bound` detection.
- **D182-D186 (P20 acceptance/tuning, reported)**: the size-index
  numeric claim in `decrease_bound` was a false positive (a well-typed
  self-descent preserves its type index; the checker refuses
  index-shrinking outright), so structural proofs establish the named
  size bound only and numeric `bounded(k)` claims use the conservative
  literal ladder or depth 0; names and numbers never substitute
  (`bounded(xs)` proves structurally, `bounded(3)` via the ladder; a
  numeric claim over a structural bound is deliberately refused and
  pinned as a negative); the literal ladder walks literal-stepped
  equations counting nested frames (its first form never established
  any bound); the lexer accepts hyphenated attribute names (they never
  lexed) and attribute arguments are parenthesized
  (`#[recursion("tail")]`, matching `#[weight(n)]`), with the `let _ =
  e` binder parseable as the must-use rule documents; in the
  allocation analysis a nullary `let n = e` is a strict binding, not a
  closure, and `mutBorrow` is the `Mut` introduction (an allocation
  site), while a runtime `decide` operand validates the parameters it
  examines (`checked-params`); `SafetyReport` carries structured rows
  and the `jpl` assertion count is the verified rows.
- **D187-D191 (P27, reported)**: `value_range` states `!range` only
  for `Bool` and the narrow primitive scalars - `Fin` values are
  constructor-block pointers at runtime so a range claim on the word
  would be unsound, and full-width spans (`U8` as `i8`, `U32` loads)
  are unstatable (`[a, a)` is the empty range the verifier rejects);
  the by-value aggregate parameter is spelled `byref(%__sl_layout_N)`,
  not `byval` (disassembly against clang-compiled C proves `byval`
  lowers to a stack-passed copy that segfaults against a C function
  taking `struct S` by value, while the C ABI passes a pointer to the
  caller's copy; `byref` states validity without changing the
  convention and the run test proves callee copy isolation; `sret` is
  clang's own spelling; per-target spelling is the cross-compilation
  follow-up); `llvm.assume` has no `range` operand bundle in this
  LLVM (accepted: align/cold/dereferenceable/
  dereferenceable_or_null/ignore/nonnull/noundef/separate_storage),
  so range facts use the `range` return attribute and `!range`
  metadata; `willreturn`/`nofree` are scan-proven and transitive
  (checked definitions restate them only under their P20/totality
  rows; `nofree` requires no opaque call in body or callees since an
  opaque call may deallocate); the interrupted flight's body scan had
  skipped assigning calls (effectful units claimed `memory(none)`) -
  the honest unit scan now classifies every call, load, store, and
  poison source, alongside two IR-spelling fixes (`freeze` width,
  `!align` node shape).
- **D192-D197 (P16 evidence invalidation, reported)**: a Prop-typed
  evidence item whose statement reads a `Mut` borrow carries that
  region as its footprint and any write to the region invalidates it
  (stale uses are check errors naming both the derivation and the
  invalidating site); `refSet m v` is a new builtin write through an
  exclusive borrow, lowering to `Core::RefSet`, an opaque write the
  optimizer may not reorder or propagate across; the write triggers
  are `refSet`, calls passing a `Mut` argument, calls whose effect row
  names the region with a write-capable head (the effect-row reading
  of "can write"), and opaque writes (FFI/foreign, eval, mutating
  `Bytes` primitives) which conservatively invalidate all
  `Mut`-sourced evidence; use sites (`Fin` bound fields, erased `Prop`
  params) record deferred uses checked after constraint solving (an
  effect-row region may only resolve there; regions compare via
  region resolution); `Ref`-sourced evidence survives its region (a
  `Ref` is read-only and P5 exclusivity forbids a coexisting `Mut`
  write) and immutable evidence never invalidates; `RawPtr` targets
  carry no evidence at all and `RawPtr`/FFI writes are opaque mutation
  (the conservative rule).
- **D198-D205 (P16 temporal contracts, reported)**: `contract`
  declarations are top-level, naming a definition of their own module,
  with formulas as specification notation over the definition's
  pattern parameters (no call-graph refs; one reserved word added);
  `always (P)` is an inductive invariant over the definition's
  per-step transitions, where the transitions ARE the D192 write log,
  establishment is stated (`assume = (P)`, checked equal), each step
  must leave `P` true (post-state decision, syntactic
  self-preservation, or inductive discharge), falsifying writes are
  rejected naming the step, and undecidable steps are refused;
  `stable (P)` is the dual of evidence invalidation (no write trigger
  touches the footprint; a touching write kills it naming the rule);
  `eventually (Q)` verifies only the finite P12 obligations
  (`joined`/`cleanup` from the `withScope` guarantee) and refuses
  open-ended claims, with liveness contracts stating assumptions
  (concurrency spec section 10); `recur (P)` is bounded by the D113
  structural-depth proof (the call-graph ladder is refused as a bound)
  and witnessed by a re-establishing step on every recursive cycle
  (call-by-value walk with AND-join at branches and fresh segments at
  lambdas; staging-construct recursion refused); a call's potential
  write discharges its step only from the definition's own recursion
  passing `P`'s variables unchanged (no cross-definition discharge);
  evidence-backed post-state matching was designed and REJECTED as
  unsound (the prop language is name-based; shadowing would break
  evidence matching); contract facts are `.cinchif` `contracts` rows
  hashed into `iface_hash` (pinned both ways), stated assumptions join
  `assumptions.declared`, and the manifest `temporal` section records
  each contract's verification result.
- **D206-D212 (P18, reported)**: `prim name :: type where` with
  per-family inline bodies (`family { fragment } [when cond] [justify
  e] [uses ...] [immarg ...]`), fragments captured verbatim by the
  lexer so fragment grammars never pass through the surface tokenizer;
  `llvm` fragments splice into their implementation unit, `asm`
  fragments are inline-asm templates (`$0` return, `$1...` args,
  constraints `r`/`f`) carrying `llvm.sideeffect`, `wasm` is
  registered but unemittable here, `clr`/`jvm` reject as reserved;
  containment validation names its violations (signature discipline,
  no-escape uses allowlist, no-unwind, determinism of effects,
  grammar); `immarg` is a per-body clause emitted as `!immarg !N`
  function metadata (LLVM admits the parameter attribute only on
  intrinsics); per-family implementation units bridge the closure ABI
  to the fragment's declared ABI; the prim ABI is machine-scalar
  arguments/result with closed effect rows (a pure row forbids stores,
  atomics, volatile, `uses` calls); capability selection orders `llvm`
  then `asm`, compiles every applicable body so each family's contract
  is observed, and is manifest-only (a source probe cannot observe the
  choice; `tests/prim.rs` proves it under two profiles); justified
  prims carry law identity in their contract and assumed prims carry
  `prim:unit#<bodyhash>` (body edits stale dependents via
  `iface_hash`); intrinsic-family lowering migrates onto prim bodies
  with the namespaces staying reserved (D73); `law` members register
  only under their `Signature.law` spelling (bare names stay out of
  the value namespace), law equations use named total functions
  (`Function.Combinators.compose` added), instances are checked
  sample-wise by evaluating real method code through the staging
  interpreter (scalars at bounds, `Bool` exhaustive, `Nat`/`Int`
  small, data over constructor samples; closures/dictionaries/floats
  and evaluation failures report `not checked`, never false failures),
  law-breaking instances reject naming the law, and undischarged
  citations record their identity in `assumptions.declared` with the
  manifest `laws` rows reporting each instance's honest status.
- **D213 (P18 effect purity and `unsafe`, user direction)**: a prim's
  effect row decides its treatment: an empty row means proven pure by
  containment (no evidence invalidation, no attribute-scan defeat,
  possibly `speculatable`); `Io`/write rows mean effectful per the row.
  `unsafe` marks bodies containment cannot verify (`asm` templates,
  `uses` into opaque code): the declared row is trusted rather than
  verified and the trust is recorded as an assumption with the
  `prim:unit#<bodyhash>` identity and manifest trust rows (an unsafe
  prim claiming purity is a visible assumption, never an invisible
  claim; scan-gated attributes stay scan-gated). Implementation queued
  behind the tooling slice; spec section 12.
- **D215-D220 (P21+P22 tooling, reported)**: one shared analysis
  layer (`analysis.rs`) drives both front ends (the REPL is backed by
  the language server exactly as directed); checker spans carry no
  file, so each file parses behind a fixed line band and attribution is
  unbanded at the output boundary (real source text is hashed, not the
  banded text); the REPL probe finishes exactly like a definition body
  (full pipeline, then the staging interpreter on a truncated temporary
  unit) with evaluation SKIPPED, not faked, when constraints stay
  residual (residual wants become the scheme's `using` context);
  signature discipline: lone signatures hold pending their equation,
  bare equations get synthesized signatures via probing `\pats ->
  body`, bailing to "state its signature" when the rendered scheme
  carries generated names (display forms are not re-parseable source);
  `:reload` forces a recheck while unchanged updates are no-ops;
  incremental reporting is honest (`rechecked` is the whole check
  slice including unchanged name-resolution dependencies, `kept` is
  cached reuse, `chain_ok` re-verifies the merged chain); the LSP is
  hand-rolled JSON-RPC (zero-dependency workspace) with full document
  sync, one-character diagnostic ranges, and URIs rooted at the client
  root while all other output is workspace-relative.
- **D214 (P18 platform-gated bodies, user direction)**: body
  conditions gain `when arch <name>` predicates keyed on the target
  profile's triple (x86_64, aarch64, wasm32, ...), with the selection
  rules unchanged (matching conditional wins; unconditional body is
  the baseline; overlapping conditions error; no-match rejects naming
  the platform; manifest-only with the platform in the body row's
  reason). The canonical shape is a portable `llvm` baseline plus
  per-arch `asm` bodies (popcnt vs cnt). `os`/pointer-width predicates
  deferred to a consumer. Spec section 13; folded into the running P18
  continuation.
- **D221-D227 (P18 continuation, reported)**: `unsafe` is a per-body
  prefix (`unsafe <family> { ... }`) with `unsafe prim ... where` sugar
  folding into bodies at parse, recognized contextually so `unsafe`
  stays usable as an identifier; trust identity hashes the individual
  body's content (family, condition, fragment, `uses`, `immarg`) so
  editing that body stales dependents, and EVERY `unsafe` body records
  trust since a non-pure row is also an unverifiable upper-bound claim;
  unverifiable surfaces (asm/wasm templates, `uses`/inline-asm calls
  into opaque code) REQUIRE `unsafe` regardless of the row while
  containment-visible violations (stores/atomics/volatile under a pure
  row) reject even under `unsafe`; `TargetProfile.triple` is the arch
  identity and `when arch` matches its first component exactly (codegen
  triple emission unchanged); selection precedence became
  cross-family conditional-first then family-order baselines (the
  canonical per-arch shape needs an `asm` conditional to beat an
  `llvm` baseline), per-family overlap errors unchanged; the manifest
  `condition` field prints the rendered predicate verbatim (it
  previously mislabeled everything `Available`); and an observation
  locking prior behavior: prim calls were already off the D194
  opaque-write list (they elaborate to plain unit calls) and the pure-
  prim evidence test now pins the row treatment as the whole story.
- **D228-D231 (P15 examples sweep, reported)**: a discharged
  example-side `proof` is not expressible today (`refl` cannot bridge
  under no-delta conversion (D28); the proof grammar is exactly
  `refl`/`congr e`/evidence-name with no transitivity or elimination;
  `congr`'s function argument is an index term), so example 08 states
  its instruction law in its own theorem module as an obligation and
  cites it with `justify` - preserving the "obligations, not supplied
  proofs" teaching; the alternative (match + prelude law citation) was
  probed green but rejected because it moves the rule off the
  example's own scalar model. Example 05's build inputs are now
  example-side (`schema-tool.sh` stub; `SCHEMA_DATABASE_URL` referenced
  by name with the value only in the tool's environment;
  `CINCH_SCHEMA_TOOL=./schema-tool.sh` resolves against the stage
  workdir); refresh and replay both exit 0 and the D45 refusal stands
  without the secret. Example 02 is refactored to `decide`-dispatched
  filtering (the D35 boundary taught in its comment; predicate and
  output order preserved by recording the decision before recursion;
  witness and computed-index output verified). Example 03's gate is
  recorded as language work: index/type-position lambdas, proof
  reduction beyond D28, and a `List.map` name resolution; the
  assumed-law shortcut would replace its structural-induction teaching,
  so it stays rejected until the language work lands.
- **D232-D237 (P24, reported)**: `Sym` is a builtin core type name
  (like `Bytes`) with `Data.Sym` as its API/instance home and exactly
  `Eq`/`Ord`/`Show` (no structural `Copy`); the lexical rule is taken
  literally (the char probe runs first, so `'''` is `Char '\''`; a tick
  name is the identifier after the tick with dash glue and primes as
  ident chars, so `'foo'` names `foo'`; names start with a letter or
  underscore); type-position `'foo`/`''T` reject naming D124 and
  `''T` in expression/pattern position rejects naming its staging-only
  role; runtime cells are kind-tagged (`KIND_SYM`), leaked and never
  freed, `Eq` is the pointer test with kind validation, `Ord` compares
  name bytes, `cinch_sym_intern` rejects non-UTF-8 names so `sym-name` is
  total, and the extern surface is exactly four `nm`-verified ops;
  compile-time `Value::Sym` interns by name (identity is name identity;
  no cell address is observable) with `sym*` prims constant-folding;
  and a latent emit-header bug was fixed (`@cinch_bytes_data` was called
  but never declared, so any program whose IR used the byte-span
  crossing failed to assemble). The D46 `Name` member is UNIMPLEMENTED
  and its exact gap is recorded: macro signatures admit only
  `Syntax`/`Code`/`Declarations`, `quoteDecls` names cannot be spliced,
  and no `Name`-to-symbol reification exists (tick lexemes survive as
  macro tokens and splice-time reification works; finishing it needs a
  `Name` type and signature member, name splices with hygiene renaming,
  and reification builtins).
- **D238-D244 (P23 editor tooling, reported)**: the tree-sitter
  grammar is `grammar.js` plus an all-external terminal scanner that
  is a straight port of `lexer.rs`/`layout.rs` (regex tokens cannot
  express the context rules), with generated parser output gitignored
  so corpus assertions cannot pass against a stale parser; infix
  chains parse through the documented fixity table as precedence
  layers (equal-precedence clashing-assoc pairs only appear
  parenthesized in valid programs, so sub-level order is unobservable
  on valid input; unknown operators sit in one flat layer above `$`);
  `unsafe` and attributes resolve by one-token lookahead (kept usable
  as identifiers per D221); prim fragments, token trees, and attribute
  arguments capture verbatim like the lexer; the "must indent farther"
  rule mirrors by refusing the block so the parse errors; `in` after a
  `with (...)` paren binds as the supply's `in`; Sym names follow
  identifier spelling (apostrophes included) with the char rule as the
  only tie. Documented accepted divergences: tabs in indentation are
  whitespace in the grammar (the compiler rejects), `#[` name adjacency
  is not enforced, do-let groups follow the EBNF (the bootstrap binds
  one value). The LSP wiring finding: `cinch lsp` does not exist as
  a subcommand (the driver offers `check`/`emit-ir`/`build`/`run`), so
  `tooling/lsp-driver/` is a cargo bin invoking `LspServer::run` over
  stdio verbatim, with the editor wrapper auto-switching to `cinch
  `lsp` if the subcommand ever lands (tracked).
- **D245-D247 (P23 recovery, reported)**: the CPU loop was `peekc`
  tail-calling itself when the replay buffer ran dry (an unconditional
  jump on the first peek of every parse); the fix COMPLETES the replay
  design rather than reverting it (one-token lookahead for contextual
  `unsafe`/attributes and the layout machine's virtual tokens must
  classify within one scan call, and the scanner cannot rewind past
  `mark_end`, so a post-mark overflow buffer is the minimal correct
  mechanism), with `take` recording past-mark characters, `mark`
  resetting the buffer, and layout columns tracked internally (the
  lexer cursor sits past the mark during replay); the 87th corpus case
  was a corrupted fixture (lost expected tree), restored not weakened;
  and the real-files property is now enforced by
  `check-real-files-parse.sh` (60 `.cinch` files repo-wide, zero
  ERROR/MISSING nodes; the prior "59" was a stale count). Residual
  documented divergences: tabs in indentation (compiler hard-errors;
  tree-sitter cannot express it), `#[` name adjacency, and do-let
  groups (the grammar follows the normative EBNF `do_statement = "let",
  local_block`; the BOOTSTRAP compiler binds one value per do `let` -
  the compiler holds the gap there).
- **D248-D255 (P28 refinements, reported)**: the surface spelling is
  `type R p = refine (x :: T) with P` (binder `::` or `:`; `refine`
  contextual; aliases refuse naming the form); predicate admissibility
  is a closed structural vocabulary (`NotNull`/`NonZero`/`Ascii`/
  `Ordered` over admitted operand shapes) plus the existing `Dec`
  propositions over numeric containers, with quantifiers, and/or,
  proposition variables, non-binder operands, non-numeric comparisons,
  and `Ordered` over generic element types refused by name; the
  witness `R x w` mirrors `Fin` (the slot takes a `Dec` discharge that
  must decide and erases, or `Yes`-extracted evidence with D192
  staleness applying at the use); preservation law identity is
  `<Refinement>.preserve-<method>` in the same honest manifest tier as
  class laws; runtime `decide` on vocabulary predicates lowers to
  baked tests (constructor-tag cases and synthesized
  `<builtin>.refine-*` units), never to source a shadowing module
  could lie with; static discharge folds constructor/numeral forms
  literally written (no value-equation substitution through
  `let`/match, so runtime discharge is the path for opaque values) and
  refinements have no constructor patterns (match via `unrefine`);
  instance transfer is rep-identity (the container's methods at the
  container's types, the dictionary presenting refined types); and the
  honest boundaries are reported per instance (nested refined results
  and `Monoid`'s unit laws citing an uncarrried `Semigroup.<>` report
  `not checked` - a pre-existing law-machinery gap now visible).
- **D256 (P28 compound predicates, user direction; revises D249's
  conjunction refusal)**: the predicate grammar gains `&&`/`||` over
  admissible predicates (conjunction/disjunction of decidable
  propositions is decidable; quantifiers remain refused per D113) and
  comparison atoms admit total expressions closed over the binder
  (`length xs > 100 && length xs <= 200 && Ascii xs`), with the
  runtime `decide` lowering computing measures under the baked-test
  rule (never source a shadowing module could lie with). Instance
  transfer checks preservation PER CONJUNCT (failures name the
  conjunct; disjunction uses the conservative per-disjunct check or
  reports `not checked`). The witness stays a single erased `Dec` of
  the whole predicate. Implementation queued behind the language-tail
  slice (shared `check`/`parser`).
- **D257 (P28 composability, user revision of D254's per-nominal
  transfer)**: refinements are a PREDICATE-INDEXED family, not
  nominal types with per-type instances. (1) The transfer rule is
  general - the container's method is available at `refined p` when it
  preserves `p` (coherence improves: one rep-identity dictionary per
  class-and-container, unique by construction); (2) the family carries
  a decidable implication preorder with implicit weakening along
  atom-set inclusion (`refined (p && q)` flows to `refined p`;
  syntactic, no SMT) and explicit `Dec`-carrying casts beyond it,
  while `unrefine` stays the explicit forgetful arrow; (3) `&&`/`||`
  are meet/join so compound refinements inherit instances
  automatically; (4) method outputs carry derived predicates
  (`post p q`, checked by the law layer; preservation is the closure
  special case; a method with no sound postcondition has no instance).
  Nominal names are readability sugar. Refusals unchanged: no
  inference, no SMT, discharge always explicit. Implementation rides
  the P28 continuation (replacing the per-conjunct plan of D256 with
  predicate-level rules plus per-conjunct preservation as the
  postcondition check).
- **D258 (P28 narrowings, user direction)**: the implication
  procedure generalizes from atom-set inclusion to a table-driven
  theory: interval containment for comparison/measure atoms
  (`> 1 && < 9` implies `> 0 && < 10` via `lo >= lo'`, `hi <= hi'`),
  documented vocabulary rows (`length xs > 0` implies `NotNull xs`),
  syntactic `p && q implies p`, and unknown pairs NOT implied (the
  cast needs a `Dec`, or refuses). Weakening remains implicit and
  free (the same word); **strengthening is always checked** - a
  `narrow` cast carries a `Dec` of the stronger predicate (statically
  folded or runtime-discharged); silent upward flow is the
  forged-witness hole the D192/D249 rules close. Instances and method
  results flow down the lattice for free (D257). The interval table is
  the shared home of bounded arithmetic for `bounded(n)`/`recur`
  (D113/D202).
- **D259 (P28 partial weakening, user direction)**: dropping some
  conjuncts is weakening to an intermediate lattice point (already
  implicit; instance availability is monotone under it - forgetting
  cannot lose methods). Explicitness takes two layers: type ascription
  is the everyday form (no new syntax), and `forget B from xs` /
  `forget [B, C] from xs` names what is given up (top-level conjuncts
  only; structural drops use ascription; dropping an absent atom names
  the error). Zero runtime cost; genuinely irreversible (restoring an
  atom needs a fresh `narrow`/`Dec`). The keep-set form is deferred
  pending API practice.
- **D260 (P28 instance heads at refinements, user direction)**:
  instance declarations admit refinements in their heads. Specialized
  methods at refined heads exist (Sorted's `Semigroup` is merge, not
  the container's append), and the general transfer rule becomes a
  library instance (`instance Semigroup (refined p (List a)) using
  (Semigroup (List a), Preserves (<>) p)`) with `Preserves` a built-in
  constraint the law layer discharges. Resolution is most-specific
  along the implication lattice (candidates: heads `r` implies;
  strongest wins; the generic `Preserves` instance sits at the top and
  is overridden by specificity); incomparable heads matching a common
  refinement are a coherence error naming both (D119 extended).
  Applicability follows predicate implication, never method totality.
  The `Preserves` context spelling is the implementing slice's call.
- **D261 (P28 constraint reading, user correction of D260)**: the
  primary form is Haskell-style contexts - refinement predicates are
  obligations in `using` contexts on instances AND signatures
  (`instance Semigroup (List Nat) using (Sorted ...)`,
  `merge : ... using (Sorted xs, Sorted ys)`), with caller-side
  discharge through the `Dec` evidence path (D192 staleness applies).
  `refined p T` heads are sugar for a head plus its predicate
  obligation (inter-derivable; the constraint form is more general).
  D260's `Preserves` constraint class is unnecessary: preservation
  obligations are predicates over the method's arguments/outputs.
  Resolution and coherence follow D260 with constraint sets ordered by
  the implication lattice. Constraints entail nothing (standing rule).
- **D262-D267 (language tail, reported)**: `cinch lsp` dispatches
  to `LspServer::run` over stdio (the tooling wrapper's auto-switch
  now fires; its header note is stale); do-let binds a whole local
  block group (`do_statement = "let", local_block`) through the shared
  elaboration, never binding a name twice (the letrec rule) while
  `let ... in` keeps its replace-nested-singles shadowing, with only
  simple value bindings sequencing under CPS elaboration (D24's
  message for parameterized/`~`, `cps_let`'s for destructuring);
  `Data.Enum` + range syntax `[e1..e2]`/`[e1, e2 .. e3]` desugaring to
  `from-list (enum-from-to ...)` at the expected `FromList` instance
  with `LitFold` kind `"range"` folding stage-known ranges (inclusive
  endpoints; step `e2 - e1` in code space; zero or runaway step is
  EMPTY, terminating where Haskell diverges; `Enum` codes are
  consecutive; bounded instances fault at their edges; `Enum Char`
  deferred on its conversion story). Two findings deferred to the
  continuation: `resolve_term`'s catch-all skips `Term::Map` (an infix
  chain inside a map literal reaches the checker unresolved), and the
  debug build's nested-let elaboration depth sits at the 2 MB test
  thread edge (a depth-limit diagnostic is the real hardening).
- **D268-D270 (language tail continuation, reported)**: nested
  `let`/`letrec` elaboration carries a named depth limit (128; the
  diagnostic fires at the 129th, 61 and 128 run on the main thread,
  wide stacks remain for deeper-but-legal programs) and
  `resolve_term` resolves infix chains inside map literals; `Name` is
  a compile-time builtin type and macro signature member with
  hygienic name splices in `quoteDecls` name positions (the chosen
  name binds at the insertion site as written while template binders
  rename apart) and `name-to-sym`/`sym-to-name` reification (tick
  quotes and bare names in `Name` arguments; holes accept `Name`,
  `Sym`, and single-name `Syntax`; one splice-reification story);
  example 03's gate is cleared by index/type-position lambdas
  (`Type::Lam` with capture-avoiding substitution and fuel-bounded
  beta), `List.map`-style qualified names in index position, proof
  reduction as named-helper reduction (delta on program equations
  plus case iota, fuel 64, proof-goal matching only; D28 conversion
  unchanged), structural-induction `case` in proofs (the goal refines
  per branch at the pattern's constructor form), `congr`'s lifting
  function explicit as an index term, and evidence formers taking
  value binders. The example carries its real proof with one `import
  Data.List` line (Main applied); the assumed-law shortcut stayed
  refused. **The acceptance catalogue is 12/12 as-intended.**
- **D271-D277 (lattice predicates and movement, reported)**: the
  predicate grammar admits `&&`/`||` and comparison atoms whose
  binder occurrences pass only through the baked measures (`length`,
  `toNat`) and arithmetic (a bare comparison over a non-numeric
  container is refused naming the measure form); the implication
  table is sound and deliberately incomplete (alpha-canonical
  identity, normalized inclusive-interval containment, And/Or
  intro/elim, the length and non-zero vocabulary rows,
  range-disjointness refutation; unknown pairs not implied) and lives
  in `types.rs` as the shared bounded-arithmetic home (`bounded(n)`/
  `recur` consumers not yet rewired); lattice weakening fires in
  `unify_arg` (same container plus derived implication means the same
  word; failure names both predicates and points at `narrow`); the
  `refined P T` former states its container and a named refinement at
  the same (container, predicate) is one lattice point;
  `forget` operates on top-level conjuncts up to binder renaming and
  refuses dropping the last conjunct (that direction is `unrefine`);
  `narrow` requires the target predicate to imply the value's (it
  never forgets), accepts a bare `decide` only when static, and takes
  case-bound `Yes` evidence otherwise; `decide` keeps refusing runtime
  negated propositions while a statically decided `Not P` works.
- **D278-D281 (Cinch rebrand sweep, reported)**: the exported example
  corpus is content-frozen under the rename (pure `git mv`, hash
  columns byte-identical; the `SAFELANG_SCHEMA_TOOL` comment residual
  in 05 flips at the SHA256SUMS re-pin); SHA256SUMS hash values were
  untouched and five entries plus three docs were already stale
  before the sweep (content evolved after the export pin) - a re-pin
  with a README evolution note is queued at P15; the zero-hit rule
  renamed the unrelated `u.lang` field to `lang_facts` (type name
  kept); and `render_ast_type` lacked the `Type::Lam` arm the tail
  slice's index lambdas needed - the user's 4-line fix is restored
  verbatim as its own commit (`4b7b5df`) and the stash dropped.
- **D282-D284 (lattice instance half, reported)**: `using`
  obligations on instances are per carrier argument (the binder is
  the predicate's own) while signature obligations bind the
  signature's dependent argument names; resolution matches heads by
  predicate implication with the strongest condition (head
  predicates folded with the constraint set) winning, stated heads
  beating generic on ties, incomparable maxima at a ground want
  rejecting as a coherence error naming both heads and the point, and
  underdetermined wants still deferring as `Ambiguous`; the transfer
  validity check runs per (point, method) at every refined want, law
  rows key to the container instance when the winner carries
  `Preserves` (rep-identity dictionary), and post rows record the
  `post p p = p` closure case (the general p != q postcondition is
  out of scope).
- **D285-D287 (instance-half acceptance fixes, reported)**:
  `Preserves (` op `)` is the constraint spelling and a named
  refinement in proposition position reads as its predicate
  everywhere; transfer obligations record only after zonking (a want
  left as a metavariable records nothing) and the law tier runs on a
  512 MiB scoped thread stack because bound-corner witnesses
  evaluate the real method over 200-element lists; `.cinchif`
  carries law rows (`Iface.laws`, `LawFact`) hashed into `iface_hash`
  with rows living in the dictionary-owning module, and signature
  obligations normalize to positional binders at conversion
  (substituted by position at the call; partial application
  refused). **P28-2 complete: `tests/lattice.rs` 17/17** (9
  predicate/movement + 8 instance acceptance; the probes caught 9
  real bugs before passing).
- **D288-D290 (P15 doc sweep, reported)**: the SPEC-MAPPING matrix is
  complete at 127 rows (I=103, P=15, N=8, CV=1) with every evidence
  pointer verified to exist and every P naming its exact gap (the
  P19 "1M-deep tail-call proofs" row cites only persisted evidence:
  `core.rs::tail_report`, the codegen loop header, the `tail_calls`
  manifest, `tests/safety.rs::recursion_tail_covers_self_and_mutual_
  groups` - the runtime depth proof is not a named test and a
  follow-up is queued); refinements carry their own amendment section
  on the core-language page; the P27 LLVM cluster (design-only, no
  spec page) has its notes split between attribute-annotations
  (vocabulary) and platform section 8 (checked-lowering facts) with
  `byref`/`sret` cross-referenced in the FFI note; and the Cinch
  naming note (D278-D281) is recorded on all 15 spec pages.
- **D291-D294 (runtime string literals, reported; the user's ask)**:
  a string literal in an ordinary term checks as the runtime `String`
  (the D43 deferral is closed): the literal elaborates to the
  `String` constructor over a constant `Bytes` payload (constant
  data, per-use construction - the exact form `string_const` and the
  `import data` converters bind); **the literal allocates** because a
  static shared buffer is unsound (the D89 buffer is mutable in
  place and `string-to-bytes` shares it), so the JPL no-allocation
  fact ("a string literal is constructed here") fires, pinned
  both ways plus a mutation-freshness probe (two evaluations never
  share a buffer); escapes bind to the lexer's decoding exactly
  (`\\`, `\"`, `\'`, `\n`, `\r`, `\t`, `\u{HEX}`; raw newline/EOF
  rejected; the surface grammar lists the identical set - no spec
  gap) and constants fold through D92 with a `LitFold` boundary for
  literal-string collections; string literal PATTERNS remain the
  parser's named refusal (pinned boundary). `Show Bool`/`Show
  Ordering` landed on the strength of it. The D124-cluster drift
  entry ("a stale compiler diagnostic still claims string literals
  are compile-time only") is RESOLVED; the only remaining
  "compile-time only" diagnostic is the `Stage.*` builtin-value
  refusal, which is correct. (The representation half - per-use
  construction, literal allocation, distinct buffers - is SUPERSEDED
  by D297-D301 below; the escape-set, pattern-refusal, and staging
  decisions remain as written.)
- **D297-D301 (string immutability revision, user-driven; supersedes
  the D291-D294 representation)**: **strings are immutable values**
  (the user: "shouldn't constant strings live in the data section and
  be immutable? I'm a little concerned about string mutation in place
  being a thing"). `Data.String` states the contract: no API exposes
  the backing mutably (`string-to-bytes` copies out,
  `string-from-bytes` copies in; a unique-consume fast path is future
  work), mutation exists only on plain `Bytes` (D297). The
  representation constructor is renamed `StrBuf` and is
  module-private - the ctor and its pattern were the last
  mutable-backing exposures, so both construction and destructuring
  leave the surface; consumers use the conversions (D298). A `"..."`
  literal is ONE shared read-only data-section constant per site
  (`ConstString`; zero construction, zero allocation; `Eq` may
  pointer-compare as a semantics-transparent fast path; sharing is
  unobservable under immutability) (D299); **the JPL rule flips
  honestly**: a literal is NOT an allocation, `#[no-alloc]`/`jpl`
  accept literals (the safety needle is now positive:
  "verified (0 allocation site(s))"), and the alloc fact fires only
  at genuine construction. The `Bytes` ABI is re-specced (D300):
  `repr(C) { len; data }` payload replacing the Rust `Vec<u8>`
  header (the old comments admitted it was a stdlib implementation
  detail), new block kind `KIND_BYTES_STATIC` (5) for read-only
  buffers, and `bytes-set` on one takes a new defined `ReadOnly`
  fault (71) instead of writing constant data; the reified quotation
  format gains opcode 19 `CONST_BYTES` (eight bytes per word, little
  endian), literals in quote bodies encoding as `CTOR` over it with
  the evaluator materializing fresh heap buffers per run. `pack ::
  List Char -> String` / `unpack :: String -> List Char` land at
  `Data.String` in Haskell's terminology (the user's naming call;
  `Text.Lex`'s old `text` helper is retired in its favor) (D301).
  The mutation-freshness probe is reversed into an immutability pin
  (two evaluations share one object; no path can observe mutation).
  One observed-once flake recorded honestly:
  `tests/params.rs::value_parameter_is_an_ordinary_binding` failed
  once under full parallel load and was green in four subsequent
  runs (params.rs untouched for months).
- **D302-D306 (P30 parsing toolkit, reported)**: `Text.Lex` is an
  in-language lexer + layout pass whose canonical token rendering is
  a live no-drift contract against `cinch::lexer`/`cinch::layout`
  (named refusals: attributes, `prim` fragments, tick name forms),
  and `Text.Parse` is the megaparsec-shaped combinator suite
  (consumed/empty distinction, `attempt-p`, `label-p`, spans via
  `spanned-p`, typed `Parse` effect with aborting `parse-fail` and
  resumable `parse-note`, recovery handlers as ordinary functions on
  the typed error per the D23 boundary (D304), and
  `ParseErrorBundle`/`render-bundle` with source gutter and caret
  (D303)). The bootstrap grammar test parses the declarative surface
  and expressions of real examples end to end over the toolkit, with
  the unproductions as named refusals (foreign/rewrite/upgrade/
  generate declarations, record+codata, refinement forms, module
  parameters, macro/quote/splice, `do`, `handle`, range literals,
  memo bindings, fixity resolution - closing this list is what the
  P35 full-grammar milestone still owes). **Compiler defect
  recorded (D302)**: eager mutual recursion between top-level VALUES
  crashes the binary (signal-killed; surfaced as exit 1) - the
  letrec fixpoint discipline does not reach top-level value
  bindings; the toolkit's workaround is lazy `MkParser` wrapping and
  the desired diagnostic is "top-level value X is part of an eager
  reference cycle; wrap its reference in a lambda or use letrec"
  (tracked). The refusals test exercises parse-error rendering
  (the grammar template already pins named refusals) (D305); the
  harness generates Cinch sources line by line (D306). The
  `text [...]` char-list constructions (173 sites) became string
  literals per the user's ask, and the `text` helper is retired in
  favor of `pack`/`unpack` (D301).
- **D307 (P30 API finding)**: `run-parser-pure` always appends
  `expect-eof` after the user parser, so whole-input enforcement is
  the runner's default (kept: it is the safe default for real
  grammars) but `expect-eof` cannot be used as a standalone parser -
  it runs twice and the second fails because it consumes rather than
  guards. The decided fix (queued): `expect-eof` becomes a
  non-consuming idempotent guard, pinned by a test.
- **D308 (export-surface ctor leak, gate-found and fixed)**: the
  facade pin (`facade_export_list_is_the_checked_surface`) failed at
  the integration gate because module-private constructors leaked
  through three layers: `module_iface` recorded every constructor of
  an exported type in the `.cinchif` export table regardless of the
  named set (renamed ctor exports also recorded the definition
  spelling), and both type-directed fast paths -
  `ctor_for_type` (pattern position) and `resolve_ctor_for` (term
  position) - resolved constructors against the scrutinee type's
  full registry with no surface check, so `import Data.String` could
  construct and match `StrBuf` (probe-proven before the fix). Now:
  the export record carries exactly the named constructors under
  their exported spellings, both fast paths gate on surface
  visibility, and the refusal is named ("constructor `StrBuf` is not
  exported by `Data.String`") in both positions. Pinned by
  `private_ctors_are_unreachable_through_an_import` (both
  positions). Known honest quirk: a type with hidden constructors
  can be matched only through its visible constructors plus a
  wildcard; a coverage error may name an unmatchable constructor.
  The gate also exposed that the single-constructor type-name alias
  (`String b` constructing the sole constructor) had survived the
  StrBuf rename as cross-module surface - it is construction and
  follows the constructor's privacy (pinned), and the two
  `tests/refinements.rs` fixtures that used it moved to
  `string-from-bytes`.
- **D309-D314 (P31 JSON and canonical serialization, reported)**:
  `Text.Json` parses and prints RFC 8259 JSON over the P30
  combinator toolkit (toolkit error bundles render JSON errors with
  the caret gutter) but **not through `Text.Lex`**: the escape
  vocabularies are mutually exclusive and `Text.Lex` folds numeric
  spellings (`007`/`0x1f` could not be refused), so the parser is
  character-level in the combinator shape; moving it onto the
  literal `Text.Parse.Parser` needs `Text.Lex` to export its `LTok`
  constructor (D308 privacy blocks it; flagged, no change now)
  (D309). The canonical form sorts object keys (stable tie order),
  formats integers decimal and floats in the runtime's shortest
  round-trip scientific spelling (`2.5` renders `2.5e0`), refuses
  duplicate keys at parse, and renders a non-finite `JFloat` as
  `null` (aeson's choice) - the one non-round-tripping rendering,
  JSON's domain being finite numbers (D310); ill-formed spines print
  as one trailing item/entry so the printer is total (D311);
  combinator names mirror `Text.Parse` exactly with import
  qualification disambiguating collisions (probe-verified) (D312).
  The runtime gains the `TextOp` intrinsic family
  `cinch.runtime.text*`: `cinch_text_sha256` (FIPS 180-4),
  `cinch_text_f64_to_decimal` (shortest round trip) and
  `cinch_text_f64_from_decimal` (correctly rounded, defined
  `MalformedDecimal` fault), nm-verified (D313); the foreign imports
  and the `JsonCodec I64`/`Bool` instances are parked in `Text.Json`
  pending their D159 homes (`Data.Machine`/`Data.Bytes`/`Data.Bool`)
  and `Text.Json`/`Text.Args` stay ordinary importable modules
  beside the entry (the P30 pattern, not base layer). **The goldens
  caught a real `Text.Show` bug (D314)**: `int-digits-into` wrote
  digits least-significant-first (`int-to-string 42` rendered
  `"24"`), hidden until now because the existing pins asserted only
  `show x == int-to-string x` self-consistency; fixed (digits at
  `pos + k - 1`), pinned at real digit strings, and `Text.Json`'s
  parallel `int-text` workaround retired. The lesson recorded: pins
  must assert real outputs, never agreement between two
  implementations of the same claim.
- **D315 (D159 home moves, reported)**: the text primitives live at
  the type homes (`sha256-bytes` in `Data.Bytes`, the F64 decimal
  pair in `Data.Machine`) and the `JsonCodec` instances live with
  their types (`Data.Machine`, `Data.Bool`). The move forced one
  judgment call: the `JsonCodec` class and its error vocabulary
  (`JsonDecodeError`/`TypeMismatch`/`OutOfRange`/`json-error-message`)
  moved from `Text.Json` to `Text.Codec`, because the type homes
  cannot import `Text.Json` (import cycle: `Text.Json` needs
  `Data.Machine`) and `Text.Codec` is the scheme's codec-class home
  - matching `ParseError` sitting with `Text.Parse`. Corollary, a
  recorded instances-with-their-type exception: `JsonCodec_Json`
  stays in `Text.Json` (moving it to `Data.Json` would cycle
  `Data.Json` <-> `Text.Codec`).
- **D316-D321 (P32 observability, reported)**: `Log r` (the `log` +
  `metric` point signals, `LogLevel`, `MetricKind`, structured `Json`
  record shapes and field builders) and `Trace r` (`span`/`end-span`,
  `TraceContext r` indexed by the runtime identity) are effect
  families in the `Io r` mold (D316); spans are lexical scopes as
  bracket pairs plus tail scopes needing no close, the close riding
  the one-shot resumption chain as a marker split by `span-scope`
  (D317) - a body-taking `span` cannot carry handled ops in its body
  under D23 (lambda bodies never dispatch) and is deferred to
  call-dispatched handling; collecting handlers cons records onto
  the resumption chain with stacked handlers merging through their
  prompts, the record shapes being the export contract (D318);
  `TraceContext r` rides `forall r.` signatures beside `Io r`
  (D319); the OTLP/JSON envelope groups resources/scope for logs,
  metrics and spans, stamps trace id/flags from the context, parents
  roots at the ingressed span id, and deliberately carries **no
  `timeUnixNano`** - the P34 clock owns time and the shape invents
  none (D320). Integration ruled (D321): `Effect.Log`/`Effect.Trace`/
  `Text.Otlp` stay ordinary importable modules beside the entry (the
  P30 pattern) - no facade re-export and no embedded-prelude entry -
  keeping the facade pin's exact surface contract intact. The DX
  bar's honest judgment: levels and fields are ceremony-free; the
  one wart is a mid-body span needing `end-span` (tail spans need
  nothing).
- **D322-D326 (quality chores, reported)**: the D302 top-level eager
  cycle is fixed as the **named refusal** ("top-level value X is part
  of an eager reference cycle; wrap its reference in a lambda or use
  letrec"), raised at check time over the local guardedness rule's
  exact discipline (same suspension positions, same walk stage,
  reach-over-all-edges so the eager-call cycle `x = f 1; f n = x`
  catches too) - the reproduction first showed the SIGSEGV (exit
  139) the refusal replaces (D322); the diagnostic names the
  referenced binding that closes the cycle at the owner's equation
  span (self-cycles name themselves) (D323); the rule covers ordinary
  top-level values only - instance methods, staged units and erased
  `spec`/`proof` definitions keep their own discipline - and a latent
  cycle in an unused value now refuses unconditionally (D324). The
  local rule's blind spots are inherited and named honestly:
  immediate beta-redexes and indirect-call cycles remain runtime
  nontermination. `expect-eof` is a non-consuming idempotent guard
  (empty stream or `TEOF` head succeeds; only `expect-tok` consumes
  `TEOF`) composing with any runner while `run-parser-pure` keeps its
  implicit whole-input default (D325), and the row-122 gap is closed
  with `tests/tail_calls.rs`: self and mutual tail recursion at
  1,000,001 calls with `tail_report` asserted `marked`, plus the
  overflow control at the same depth without tail calls (D326);
  `SPEC-MAPPING` row 122's evidence re-grounded (an editorial touch
  outside the fence, accepted).
- **D327 (Comp {e} a settled, Main design)**: the value-level
  computation type former is a **library newtype over effectful unit
  closures** - `newtype Comp (e :: Effect) a = MkComp (Unit ->{e} a)`
  with `suspend`/`run` (`Effect/Comp.cinch` home, D159 placement) -
  not a core form and not rows-on-the-container. The row parameter
  rides the existing effect-row type arguments (`Request {e}`'s
  machinery); running a `Comp` is ordinary application so the D24
  dispatch discipline holds unchanged (`Comp` values are lambda
  bodies in a box: latent, freely stored and shared as data);
  `handle (run c) with h` is the ordinary handler shape and
  row-polymorphic signatures apply unchanged; `Comp` gets no
  compiler-magic `Monad` - any instance is ordinary library code.
  Refusals kept: no implicit sequencing (data until `run`), no new
  effect-row subtyping. P33's Stream yields `Comp` steps (exact
  Stream shape is P33's); the note is
  `design/comp-computation-type.md` (options weighed: core form and
  rows-on-container both rejected with reasons).
- **D328-D336 (P34 platform layer, reported)**: `Set` is a
  height-carrying AVL (`Bin` node shape change reported, observable
  semantics unchanged) and `HashMap` a private height-augmented AVL
  spine keyed by hash digest with collision buckets (D328-D329);
  `Effect.Io`'s `Path` placeholder is replaced by `Data.Path`'s real
  type re-exported under the facade name and `read-config-bytes`
  becomes a real file read (D330); the clock/fs/process ops take
  `using (io :: Io r)` and perform `IO r` - a distinct `Clock r`/
  `Fs r` capability class is unimplementable today because the
  `with`/`@{}` supply and `is_capability_class` are hardcoded to
  `Io` in `check.rs`/`check_terms.rs` (the exact need recorded;
  tracked) (D331); `Effect.Clock.timeout` states its worked row
  literally (`Unit ->{IO r, Cancel r} a`) because row-polymorphic
  generalization is blocked by the `unify_rows` bug (D337) (D332);
  `codegen.rs` emits one `declare` per external symbol (duplicate
  declares were a clang redefinition error) (D333); the D45
  determinism pin is that no staging path exists for the clock -
  a foreign import cannot run at compile time (D334); and the Fs
  semantics are recorded (`fs-list` sorts ascending; `fs-write` is
  non-atomic with rename as the atomic op; word buffers cross the
  scalar-only FFI one byte at a time) (D335-D336). The P32 fold-in
  measurement lands with D337.
- **D337 (two compiler-side findings, PlatformSlice + ObsSlice
  measured)**: (a) `types.rs::unify_rows`'s `(Meta(m), _)` arm binds
  the metavariable to the other side's tail with EMPTY labels,
  dropping the opposite side's - a definition whose performed row
  carries a rigid tail (`forall ... ef.`) silently loses its labels
  (`{Async r, Cancel r, IO r, Diverge | ef}` unified against
  `{Diverge | ef}`); unclaimed by any fence, tracked as a bug fix
  (the fix unblocks `Effect.Clock.timeout` generalization). (b) The
  checker's per-expression elaboration recursion sits near the 2 MB
  test-thread stack at the prelude baseline: embedding three more
  modules (the P32 trio) tips tests over (passes at 3 MB; ObsSlice
  measured it as cumulative checked-program size, its own flat
  110-decl control overflowing too). The P32 modules stay UNEMBEDDED
  per D321 (consistent); the corollaries are recorded honestly:
  user programs must stay shallow per definition today, and the
  options are a harness stack raise (`.cargo/config.toml [env]`) or
  a generalized depth-limit diagnostic (D268's nested-let rule).
  Tracked.
- **D295-D296 (P29 incremental recompilation, reported)**: the
  `incremental` driver module delivers all three Blaze equivalents -
  `ItemGraph` (REAPER: lazily built, kinded `Surface`/`Body` edges
  from `.cinchif` export tables and contract-fact nodes, body edges
  over-approximate so the dirty set can only overshoot),
  `Session` (INCR: two-phase span-free change classification over
  surface/body hashes; body-only changes recheck one declaration,
  surface/contract changes recheck the dependents' closure with a
  surprise contract flip widening to fixpoint, bounded at 8 rounds;
  `SessionReport` + a deterministic `build/<entry>.rebuild.json`),
  and `link_units` (LINK: emission split into declaration-keyed
  objects (`decl_hash` index), re-placing only changed units,
  self-repairing on any drift - the cache is advisory by
  construction, pinned by byte-identity after deletion). Measured
  (debug, 50-module prelude embedded in every check): cold ~1.7 s,
  warm/unchanged ~16 ms with ZERO checker runs (the acceptance
  "graph traversal only" row holds), one-change rechecks one module
  and re-places 2 of 937 units - wall time stays near cold because
  the optimizer/emitter are whole-program passes outside the
  incremental machinery (named honestly as the bottleneck for a
  future declaration-level slice). The module registers as a
  `driver.rs` child via `#[path]` for `super::` reach with zero
  visibility widening (D295); the dirty rule is a conservative
  superset: surface/contract changes cascade over both edge kinds,
  body-only changes never propagate (D296).
- **D127 (P25 scaffolding ruling)**: FoldAgent's defunctionalized
  class shapes (`Foldable t a`, `Traversable t x g h tg hs`) are
  accepted as NON-CURATED scaffolding: they ship in the `Category`
  modules with their tests but stay out of the Prelude's curated
  export surface, and A7 collapses them in place to the constructor-ary
  shapes before they face users (the `hs`-duplicates-`h` slot exists
  because `sequence` determines `y` but not `x`, leaving an
  undetermined dictionary variable). This reconciles D126's refusal of
  defunctionalization AS THE INTENDED API with keeping the verified
  semantics tests (first-error short-circuit order, NonEmpty laws);
  CategoryAgent's Functor/Applicative/Monad stay deferred entirely
  (their defunctionalization would rewrite generic callers). New gap
  recorded: effect-row traversal targets have no value-level
  computation type former (effects exist only on arrows; `Thunk` is a
  lazy value, not an action) - a `Comp {e} a`-like design question for
  a later slice.
- **P25 progress (reported, Category slice)**: `Data.Identity` and
  `Data.Const` shipped with their instance method bodies as plain
  functions (the class shapes stay A7-deferred per D126) and
  `lib_category.rs` (7 law-group tests green at the time; rerun at
  integration). The full HK target surface and 17 probes are persisted
  as `compiler/tests/fixtures/hk` (A7's acceptance fixtures). Open
  questions recorded for the HK slice: bare `pure` resolution (D22
  provider methods vs `Category.Applicative.pure`; recommendation:
  bare uses keep the provider meaning, the class method is reached
  through its constraint or `Applicative.pure`), tuple instances need
  partially applied tuple formers, `Alternative (Result e)` is omitted
  by design (needs a Monoid constraint on `e`, belongs with Monoid),
  and a stale compiler diagnostic still claims string literals are
  compile-time only (runtime `String` shipped with the prelude
  batches).
- **D125 (P25 base library curation, user-approved 2026-10-26)**:
  pulled in: `Algebra.Group`/`Algebra.Semiring`, `Data.Set` +
  `Data.Hash`/`Data.HashMap` (symbol hashing exploits Sym interning),
  class law declarations as citable `justify` evidence (riding P18's
  law/assumption tier), `Text.Show`/`Text.Codec` (A6 alignment).
  Queued: `Data.Enum` + range syntax to the next surface slice.
  Held: Profunctor/Category/Kleisli, State/Reader newtypes. Out: the
  transformer zoo (row effects do it natively), IO-as-a-monad,
  numeric towers beyond Semiring, lenses.

### Refinement types (P28)

Implemented `dfc653d` (2026-10-27, `RefinementSlice`, D248-D255):
`type R p = refine (x :: T) with P` registers a nominal type whose
runtime representation IS the container (witness `Prop`-erased per
D31); a `Fin`-shaped transparent constructor `R x w` takes a `Dec`
discharge (erased) or `Yes`-extracted evidence; `unrefine` is the
identity projection (explicit, no silent subtyping); discharge lives
entirely in the `Dec`/`decide` world (forged and stale witnesses
refused, the latter via the D192 rule); instance transfer is
rep-identity (the container's methods, dictionary presenting refined
types) with preservation checked by the P18 law layer under
`<Refinement>.preserve-<method>` identities (`Semigroup (NonEmpty a)`
checked; `Monoid (NonEmpty a)` rejected naming `NonEmpty.preserve-mempty`).
The four refinements ship: `Data.NonEmpty` rewritten (bespoke type
gone), `Data.Sorted` (Semigroup = merge), `Data.NonZero`, `Data.Ascii`.
`tests/refinements.rs` 16/16 includes the IR erasure proof (projection
body is a bare `ret`; construction byte-identical to the container's);
`lib_foldable.rs` migrated 9/9. Suite 35 binaries + 111 runtime.

### Primitive operations and inline target IR (P18)

Specified in
`specifications/primitive-operations-and-inline-target-ir-specification.md`
(normative) and `design/inline-target-ir.md` (implementation reference),
2026-10-26, at the user's direction: `prim` declarations with inline
per-target-IR bodies (LLVM now; Wasm/asm/CLR/JVM families registered),
selected by capability evidence like `target rewrite`, justified by law
evidence or recorded as assumptions, with containment validation
(no-escape, no-unwind, declared effects). Replaces the closed `PrimOp`
set as the extension path so operations like arithmetic need no foreign
call surface. Implementation rides P18, after P14 clears the code
generator.

| Requirement | Status |
| --- | --- |
| `prim` declarations: signature discipline, inline bodies per target-IR family | spec'd |
| Selection by capability condition with baseline rule; selection never in the contract, always in the manifest | spec'd |
| `justify`/assumed tiers: justified bodies are representation (contract = signature + law identity); assumed bodies carry body identity | spec'd |
| Containment validation (no-escape via `uses` contracts, no-unwind, declared effects, family grammar) | spec'd |
| Migration path for the intrinsic families (intrinsic namespaces stay reserved, D73) | spec'd |

### Attributes, weights, inline hints, and layout (P17)

Specified in `specifications/attribute-annotations-specification.md`
(normative) and `design/attribute-annotations.md` (implementation
design), 2026-10-25. The weight algebra follows GHC proposal 744
(Branch Prediction Hints, Ian Duncan). Prerequisite for P10's optimizer
and P11's FFI layout work.

| Requirement | Status |
| --- | --- |
| `#[...]` attribute system: syntax, attachment points, compiler vs macro ownership, hygiene and provenance, identity rules | done (declarations, constructor/field declarations, function clauses, `case` alternatives, guards, `if` condition and branches, field types; `= literal` and literal-list args; unknown names outside `meta` rejected unless macro-declared; semantic attributes enter the contract projection, hints never do; `tests/attributes.rs`) |
| Manual branch weights `#[weight(n)]` with the marginalization and transformation rules; `cold`/`likely`/`unlikely` as wrappers over weights (2000/1/0) | done except section 2.4's transformation rules (P10's obligations); all-or-nothing drop + `incomplete-branch-weights` warning; inflation reported as `incoherent-weights` informationally; weights are `Maybe u32` beside each alternative in core; LLVM `branch_weights` on `br`/`sw` |
| Inline hints `#[inline]`/`#[inline(always)]`/`#[inline(never)]`/`#[noinline]` as erasable optimizer hints | done (LLVM `inlinehint`/`alwaysinline`/`noinline`; function-level `#[cold]` lowers to LLVM `cold`; not in the contract projection) |
| `repr(C)`/`repr(transparent)`/`repr(align)`/`repr(packed)`/discriminant representations with layout facts in the interface ABI table and the packed-borrow safety rule | done (C layout over source-order non-erased fields with tail padding; facts in the interface `abi.layouts` table hashed into `iface_hash`; `align(N)` only raises (power of two); `transparent` is the single field's layout; packed fields reject `borrow`/`mutBorrow` while plain copies run; `repr(uN)` records the discriminant) |

### Supercompilation and JIT (P10/P13/P14/P17)

Specified in `specifications/supercompilation-and-jit-specification.md`
(normative) and `design/supercompilation-and-jit.md` (implementation
reference), 2026-10-25. One driving engine, two consumers: an offline
supercompiler (P10) and a runtime JIT (P13/P14), with the PGO loop
through P17's branch weights. Implementation items ride those phases.

| Requirement | Status |
| --- | --- |
| What specialization manages (dictionary/handler/index/effect-static specialization, fusion, constant folding) with barrier rules (effect order, linear values, memo, codata, erasure, rigid indices) | done except handler-vtable specialization (opaque barrier, reported; rides P13) and index specialization via named sites (achieved through case folding over `Vec` chains and ground `decide` instead); D60 |
| Preservation obligations: residual program rechecked (types, effect order, `%1` usage, memo sharing, productivity, erasure, capability rows); specialization representation-only (contract identity unchanged) | done (D63 recheck; contract identity pinned unchanged by test) |
| Driving bounds (whistle, fuel as target profile parameters) and the compiler report obligations (where specialization stopped) | done (D62; fuel/whistle in `TargetProfile` and the manifest; `fuel-exhausted`, `linear value: generalizing`, `capability scope:` report entries) |
| JIT invisible to types/source/interface; consumption-safe deoptimization; variant install/retire through runtime evolution only | spec'd (P13/P14) |
| Explicit `eval` of typed quotes: no capability in the signature, never evidence, manifest-recorded, target profiles MAY forbid | spec'd (typed quotes exist; `eval` itself rides the P13/P14 items) |
| Trust levels: manifest identity, full recheck (default), step certificates (future) | done for levels 1-2 (D63: level 2 default, failing candidates discarded; level 3 future) |

## Spec requirement mapping

Every spec MUST is mapped to enforcement and tests in
`compiler/SPEC-MAPPING.md` (101 rows, statuses I/P/N/CV, plus the
wording flags for future decisions).

## Decisions log

- **D1 (P1)**: tuple types `(T1, ..., Tn)` are added to the type grammar. The
  draft EBNF has tuple expressions but no tuple types; without them tuples
  are untypeable. Tuples are structural products, boxed like ADTs. Recorded
  in `../specifications/surface-grammar-local-amendments.md`.
- **D2 (bootstrap carried forward)**: operators are class methods; bare `~`
  is reserved for memo bindings; `delay`/`force`/`demand` are builtin forms
  shadowable by user definitions.
- **D3**: effect rows are row-typed (labels, tails, lacks constraints), and
  effect honesty is checked by row unification; body effects ride the last
  arrow of a curried definition.
- **D4 (P3 design)**: compiled interface metadata is a canonical JSON
  `.cinchif` artifact per module (exports, instances for coherence, fixities,
  foreign contracts, assumptions manifest, ABI symbol table), identity by
  content hash, imported interfaces recorded as the "checked against" chain.
  See `../design/compiled-interface-metadata.md`.
- **D5 (P4)**: the runtime is a Rust crate (`compiler/runtime`, `cinchruntime`
  staticlib + rlib, std-only) linked with generated objects; not C. The
  runtime ABI reference lives in its `src/lib.rs` module docs and is cited by
  interface metadata (`runtime_abi`).
- **D6 (P2)**: machine-type operations are prelude names of the form
  `<op><Type>` (`wrappingAddU32`, `checkedAddI32`, `u8ToI32`, ...) bound
  directly from `foreign import ccall "cinch.prim.*"`; the primitive
  spec's dotted `U32.wrappingAdd` spelling is a presentation form for the
  module slice to provide.
- **D7 (P2)**: `reverseBits`/`swapBytes` return `t` (a 64-bit reversal does
  not fit `U32`); integer literals elaborate to `F32`/`F64` only when
  exactly representable, otherwise the error points at conversions; an
  ambiguous literal want defaults to `Int` only when nothing else can
  resolve it.
- **D8 (review)**: a match that runs out at runtime always takes the fault
  path (exit 70), never a zero word (a word crashed on boxed result types).
  Record construction evaluates fields in source order (core specification,
  left-to-right), while constructor arguments stay in declaration order.
- **D9 (review)**: `iface_hash` covers the contract only, excluding
  `source_hash` and `compiler`, per `design/compiled-interface-metadata.md`
  section 2.2.
- **D10 (P4)**: integer literals are canonical decimal digit strings end
  to end (hex folds at lex, a prefix minus folds into the string); machine
  types range-check from the string, `Nat` rejects negatives, and `Nat`/
  `Int` lower to `Core::ConstBig` with an `i64` fast path and
  `cinch_nat_from_decimal`/`cinch_int_from_decimal` otherwise. Float literal
  exactness is the odd-part rule (odd part fits the significand and the
  binary exponent is in range). The wrapping-64 bootstrap `Int` is gone.
- **D11 (P4)**: `natCmp`/`intCmp` return the raw -1/0/1 immediate typed at
  `I64` (the runtime extern set has no `natEq`/`intEq`); `Eq`/`Ord`
  instances derive as `eqI64 (intCmp x y) 0` style and compare the
  immediate against zero with the `I64` prims.
- **D12 (P4)**: the checker synthesizes a `"<module>.main.exit"` adapter
  calling `cinch_int_to_i64` on `main`'s bignum result and `Program.main`
  targets it, so the C `main` stays on raw words and hand-built codegen
  programs keep working.
- **D13 (P4)**: `Bytes` language signatures are `Nat`-typed with the
  machine-word conversion inside the intrinsic units; byte faults use the
  existing panic path (71), no new fault codes.
- **D14 (spec reconciliation)**: surface infix `&&`/`||` are special forms
  that short-circuit: the parser elaborates them to `if` (primitive types
  specification section 10 MUST). Parenthesized `(&&)`/`(||)` still name
  the strict prelude class methods, which are different operations.
  Ambiguous integer literals default to the mathematical `Int` only when
  pending literal wants block resolution (D7), now recorded in the spec.
- **D15 (P5 defaults)**: unannotated binders default `%many`, except
  `Thunk`-typed binders which default `%aff` (lazy spec section 3). Thunks
  are the slice's only owned resources with destruction; everything else
  `%many`, so previously checked programs keep their meaning.
- **D16 (P5 Copy)**: the sharing class is spelled `Copy` (primitive types
  spec sections 3/8) with `Share` as alias; structural instances (scalars,
  `Nat`/`Int`/`Bool`/`Unit`, products/ADTs of `Copy` parts coinductively,
  `Ref r a`), not user-extensible. `Bytes`, functions, thunks, cells, and
  `Mut` are not `Copy`. Evidence is structural or from a `using (copy ::
  Copy a)` binder; `copyOut` is the only implicit-copy site plus memo
  bare-use, coherent with `%0`/`%1`/`%aff`.
- **D17 (P5 places)**: disjoint projection places (`x.f.g`) fuse into one
  consumption of the root; repeated/nested places stack; non-variable
  patterns consume their column on the row's path. Branch-merged disjoint
  projections fuse conservatively (pointwise max).
- **D18 (P5 lazy positions)**: lambda/thunk bodies count uses as unbounded,
  memo bodies as at most once; capturing a live-region borrow/cell in a
  lambda is rejected (deferred to closure usage inference).
- **D19 (P5 regions)**: borrows are check-level over the uniform word model
  (no codegen change); `demand` returns `Ref r a` (lazy spec); region names
  unify by identity with flexible instantiation names; `Mut r` uniqueness
  is lexical. Memo bare-use requires the cached type to be `Copy`, which
  discharges lazy spec section 5's duplicability rule.
- **D20 (entry point)**: `Program.main` is `Option`: `check` accepts
  library modules without `main` (acceptance example 06 is one), while
  `emit-ir`/`build`/`run` name the `main :: Int` requirement at codegen.
  A `main` outside the entry module no longer errors at check time.
- **D21 (P6 capabilities)**: a `using (x :: C ...)` binder is a runtime
  capability iff its class head is the builtin `Io` head over a runtime
  identity `r`; everything else is a compile-time class constraint. `Io`
  wants are solved ONLY from lexical providers (nearest `using`/`with`
  binding or explicit `@{...}`; no instances, no global state), `with
  (io = e) in` checks `e` against `Io r` with `r` pinned by the body,
  and `@{...}` names capability parameters only. This resolves the
  capability-vs-constraint wording flag in `SPEC-MAPPING.md`.
- **D22 (P6 providers)**: `do` provider selection is the definition's
  `using` binder with a compiler-known provider head
  (`DirectSequencing`, `ResultSequencing e`, `OptionSequencing`, with
  `pure`/`bind` class methods and implicit instances); default is
  direct; two active providers in one scope are ambiguous at `do`/`pure`
  use. `pure`/`bind` remain class methods (D2).
- **D23 (P6 handlers)**: handlers are vtables of clause closures; the
  handled computation elaborates in continuation-passing style with
  delimited one-shot continuations; operation dispatch is lexical to the
  handled computation's own expression. An operation reached through a
  call is rejected at `handle` ("performed through a called
  computation"); call-dispatched handling needs answer-passing
  elaboration and is deferred. Unhandled operations are rejected at
  check time via effect honesty; the runtime fallback is the fault path
  (exit 70).
- **D24 (P6 do-let)**: `let x = v` in `do` binds one pure value strictly
  in either interpretation; destructuring, local-function, and memo
  `let` in `do` are rejected.
- **D25 (P6 refutable binds)**: a refutable `x <- e` needs a
  provider-typed failure operation; the three bootstrap providers have
  none, so refutable binds are rejected.
- **D26 (P6 resumptions)**: `resume k` is one-shot (`%aff`). When the
  resumption's continuation consumes a `%1` capture, inline handlers'
  `resume k` binders are promoted to `%1` so the capture is consumed
  exactly once; handlers reached through a value (or clauses without
  `resume k`) are rejected in that case. Continuation-body captures
  count at most once; lambda captures stay unbounded (D18).
- **D27 (P7 universes)**: `Type 0 : Type 1 : ...` and `Prop : Type 0`;
  `Type : Type` is never admitted, and a universe argument to a `Type 0`
  slot is rejected.
- **D28 (P7 conversion)**: definitional equality is beta (at
  elaboration), iota on numerals, alpha for Pi/Sigma, and proof
  irrelevance (proofs erase to unit). User definitions never unfold in
  conversion, so checking terminates by construction; general delta
  reduction is deferred.
- **D29 (P7 implicits)**: un-annotated `forall` binders classify by use
  (type parameter / index value / region); value binders become runtime
  arguments solved by unification and reified after solving. In index
  position `+ - *` are mathematical and `toNat`/`length` are the builtin
  total vocabulary; bare-numeral `decide` comparisons default to `Nat`.
- **D30 (P7 builtins)**: `Vec`/`Fin`/`Dec` are builtin families this
  slice (runtime helper `Vec.getCopy`; `Nil`/`Cons` collide with the
  prelude `List`); the indexed-declaration mechanism itself is general
  and tested with user families.
- **D31 (P7 erasure)**: `Prp`/`Sort` fields and parameters take no
  runtime slot; family indices are not stored in blocks; Sigma retains
  its witness component.
- **D32 (P7 totality)**: structural termination, purity, and mutual
  acyclicity are enforced for `spec`/`proof` only; requiring a `Diverge`
  effect on recursive executable definitions is deferred (it needs the
  effect vocabulary extension and would invalidate existing P5/P6
  tests).
- **D33 (P7 coverage)**: an index-refuted arm is rejected ("impossible
  arm"); a match stuck on rigid indices is rejected naming the equation
  rather than guessed at.
- **D34 (P7 type-level fixities)**: type-level operators have reserved
  fixities when undeclared (`*` 7, `+ -` 6 left, comparisons 4
  non-associative), so refinements need no fixity declarations.
- **D35 (P7 rigid indices)**: matching a constructor against a family
  applied to rigid indices (for example `Nil` against `Vec n a`) is
  rejected with the index equation: this slice does not refine rigid
  indices. Programs dispatch through `decide` or computed indices.
  Acceptance example 02 currently stops here.
- **D36 (P8 Later)**: the `Later`-vs-`Thunk` rule is a usage-multiplicity
  rule: `Later a` is a distinct type former (never unifies with `Thunk`),
  a `Later` binder lives at `%1`/`%aff` only (default `%aff`) and
  `force` consumes it; `%many` is rejected as unrestricted-thunk
  semantics. Supporting kinding: `Later` takes exactly one type argument
  and no effect row.
- **D37 (P8 codata)**: a codata type is a record of delayed
  observations; construction is only by the observed form `C { obs = e }`,
  each value a closure run at observation time (unbounded-use positions
  per D18); values box like ADTs (one closure word per observation;
  `Prp`/`Sort` observations erase); pattern matching on codata is
  refused; codata values are never `Copy`.
- **D38 (P8 productivity)**: the check is syntactic (design section 3):
  a `corec` body must build its codata type at every step, and every
  recursive call (SCC of the module call graph) must sit under `delay`
  flowing to a `Later`-typed observation; violations error naming the
  unguarded call. `corec` requires its marked signature.
- **D39 (P8 productivity loss)**: productivity is certified only by
  `corec`. A codata-producing definition with an unguarded call reflects
  the loss in its type (`Diverge` on the last arrow); an unguarded call
  directly in an observation value is refused. An unrestricted filter
  can therefore never present itself as a productive stream.
- **D40 (P8 Later purity)**: a `delay` building a `Later` must have an
  empty performed row and observation values must be effect-free;
  effectful deferral belongs to `Thunk {e}`.
- **D41 (P8 guard spelling)**: `\() -> e` lambdas accept the unit
  pattern as a unit-checked wildcard (the design's guard spelling;
  example 04 needs it).
- **P8 design divergences** (recorded per the slice report): the design
  note's "streams live in the prelude" is deferred (the prelude would
  shadow user `codata Stream` declarations in type lookup), and its
  `bisim` relation is outside this slice. D32's boundary is unchanged:
  divergence of non-codata-producing helpers is still unchecked (a
  pure-typed diverging helper can fake productivity through a guard).
- **D42 (P9 multi-module files)**: a source file may contain any number
  of `module` blocks. Imports resolve to a same-file module first, then
  `<Name>.cinch`, then a unique `module Name` beside the entry file
  (ambiguity is an error). `main` must be unique among the entry file's
  modules. The P3 one-module-per-file rule is superseded.
- **D43 (P9 staging core)**: `Code t` is closed code (no local capture;
  splices compose); quote-body effects are latent and checked at
  insertion (D44). Compile-time-only types (`Code`, `Syntax`,
  `Declarations`, `String`, `Secret`, capability tokens) never reach
  target code. Compile-time evaluation interprets the elaborated core
  (stage/macro code may call ordinary definitions and instance methods,
  compiled on demand, macros expanded first) under an 8M-step
  termination budget; lazy/memo/`Bytes`/float/`Option`-returning prims
  are refused at compile time.
- **D45 (P9 stage capabilities)**: capability values determine permitted
  targets (`IoCap` root from `Stage.snapshot`, refined by
  `withReadFiles`/`withWriteFiles`/`withProcess`; a reader has no write
  authority); builtin effect labels (`StageFile`/`StageProcess`/
  `StageSecret`/`StageEmit`) record occurrence; capability callbacks are
  effect-row-polymorphic. Build identity: manifest `staging` section
  (mode, compiler version, generated names, input kinds/names/hashes)
  plus `build/<entry>.stage.snap` captured contents; refresh/replay/live
  modes; replay rejects missing or hash-incompatible inputs; secrets
  record names only and cannot be embedded; failed stages kill spawned
  children and are never retried. `Stage.quoteNat`/`quoteInt`/
  `quoteBool` are the approved constant transport; database/network
  introspection is delegated to the subprocess capability this slice.
- **D46 (P9 macros)**: macros are compile-time functions over token-tree
  groups producing `Code`/`Declarations`/`Syntax`; hygiene is
  capture-avoiding template renaming plus definition-site qualification
  before hole substitution (spliced caller syntax keeps its bindings);
  generated nodes carry per-node expansion provenance in spans.
- **D47 (P9 stage scheduling)**: the staging graph covers units
  (stage/macro defs, generate blocks, ordinary defs) over
  statically-apparent `emit (quoteDecls ...)` outputs; computed emits
  cannot satisfy a staging dependency; cycles reject with the rendered
  chain.
- **D48 (P9 attributes in staging)**: `#[...]` attributes attach to
  declarations; unknown names outside `meta` need a macro declaration;
  macro-owned attribute names reject on cross-macro collision; `repr` is
  rejected, not ignored; macro-emitted attributes revalidate.
- **D49 (P17 if weights)**: the `if` condition position is a pair form:
  `if #[w] c then t else e` weights the true branch `w` and completes
  the pair with the reference base 1 for the false branch (so
  `if #[likely] c` is the conventional 2000:1 pair). Attributes before
  the `then`/`else` expressions annotate their branch directly, where
  the one-branch rule applies (one-sided annotation drops all weights
  and reports). An explicit else-side weight overrides the pair's
  implicit 1; annotating the true side twice (condition plus `then`) is
  an error.
- **D50 (P17 guards)**: a guard's weight annotates its pass outcome
  with the fallthrough completed by the base 1 (one guard is one
  decision point).
- **D51 (P17 type-position attributes)**: parsed at record field types
  and merged with the field-declaration position (`#[repr(align(8))]
  magic :: U64` and `magic :: #[repr(align(8))] U64` are one construct);
  other type positions are syntax errors naming section 1.1.
- **D52 (P17 repr scope)**: `repr(C)`/`align`/`packed` on a
  multi-constructor `data` are rejected (`repr(uN)` covers enums);
  `repr(transparent)` is exclusive; `packed`+`align` conflict;
  `repr(uN)`+anything conflicts; field-level `align`/`packed` require
  `repr(C)` on the declaration; `align` below natural is rejected at
  either level. A fixed-discriminant `data` records the discriminant as
  an interface fact and keeps the uniform-word representation (the
  payload layout is not fixed by that representation).
- **D53 (P17 duplicates)**: same-name duplicates on one construct are
  errors except macro-owned names (the macro governs its forms); `meta`
  is not exempt. One weight-family attribute per alternative
  (`#[likely] #[weight(5)]` is an error); one inline hint per function
  (`#[noinline] #[inline(never)]` rejects) and one `#[cold]`.
- **D54 (P17 reports)**: `incomplete-branch-weights` is a warning
  (weights dropped); `incoherent-weights` is informational (weights
  kept), per the chapter's open decision inheriting the reference's
  warn-and-drop. Both are non-fatal diagnostics from
  `check_program_multi_diag`; `check_program_multi` and the driver are
  unchanged.
- **D55 (P10 justification)**: `justify e` checks under the rule's
  equation (`Eq match replace`, reified with the index language):
  ordinary kernel evidence (`refl`, `congr`, `decide` extractions,
  declared `proof`/`spec` functions) or a law method of a `signature`
  (the platform chapter's instruction laws), recorded in
  `assumptions.declared` when no instance discharges it; plus type
  preservation and resource side conditions (template free vars bound by
  the pattern or `using` context; `%1`/`%aff` binders used exactly
  once). Conversion is never extended (D28).
- **D56 (P10 defeq guard)**: bare-variable patterns and equations at
  erased (`Prp`) or universe types are rejected ("would change
  definitional equality").
- **D57 (P10 effectful rules)**: a rule whose `match`/`replace`/`require`
  performs effects (or leaves a row open) is rejected: pure `Eq` evidence
  does not authorize reordering. At fire time, pure-hole callbacks are
  checked by body scan (buffer mutation, fault, unknown closures refuse).
- **D58 (P10 target model)**: target profiles and vector shapes are
  marker types (`VecU32Target t w`, `CapAvailable t w`) in the
  bootstrap; `Available v` is the operational evidence of the platform
  chapter's validity scope (lexical region + thread; no cached
  evidence).
- **D59 (P10 capability validity)**: a `target rewrite` fires only where
  the firing unit carries a matching capability parameter;
  `Target.detectVecU32` is a replaceable dispatch boundary (never
  folded).
- **D60 (P10 fusion)**: pull-based producer fusion composes callbacks at
  the same per-element positions, preserving effect order even for
  effectful callbacks; barriers are memo/thunk allocation (demand
  reordering) and, for eager pipelines under the `mapMap` law, callback
  purity. Case folds let-bind constructor fields, preserving strict
  evaluation order.
- **D61 (P10 weights)**: over compiled shapes (default alternatives
  expanded per constructor with scrutinee synonym `let`s): inner
  alternatives outside the default set are conditioned away unrescaled,
  then the rescale formula and GCD run; dead-alternative removal is
  negation conditioning; case-push duplicates decisions keeping weights.
- **D62 (P10 bounds)**: fuel charges every transformation (zero fuel =
  the generic form, the behavior baseline); whistle = recursion
  repetition plus 8x measure growth; bounds are `TargetProfile` fields
  and land in the manifest.
- **D63 (P10 trust)**: level 2 (full recheck) per artifact; the recheck
  covers core structure/scoping, `%1`/`%aff` usage equality, memo/demand
  counts, weight all-or-nothing, and contract identity; a failing
  candidate is discarded. Type/effect-order preservation is by
  construction (core carries no types) and is disclosed in
  `lowering.trusted`.
- **D64 (P10 naming)**: dotted presentations (`U32.wrappingAdd`,
  `Scalar`/`Simd.sumWrap32`, `Target.detectVecU32`) are prelude aliases;
  `Signature.method` resolves under the signature name.
- **D65 (P11 entry rule)**: `main :: Unit ->{e} Int` is accepted
  alongside `main :: Int`. D20 is otherwise unchanged: `Program.main`
  stays `Option`, library modules check without it, `emit-ir`/`build`/
  `run` still require `main`, except `foreign export` programs may be
  linked as libraries with the C side supplying `main`. The exit adapter
  is unchanged in shape.
- **D66 (P11 aggregates)**: aggregate parameters cross as
  const-pointers to call-scoped C-layout copies; aggregate results are
  rejected (use `Out` cells or `Owned` pointers) rather than guessing
  the platform by-value struct ABI.
- **D67 (P11 boundary grammar)**: scalars, `Unit` (void; parameters
  elided), pointer roles, opaque handles, and `repr(C)`/`packed`/
  `transparent` aggregates cross the boundary; boxed ADTs, uniform-word
  records, fixed-discriminant data, closures, dependent pairs, and
  proofs are rejected naming the recorded layout. `Out` cells are one
  word.
- **D68 (P11 effect honesty)**: foreign rows are closed and explicit;
  an import's result arrow names at least one accounted label (`Foreign
  <boundary>`, `IO`, `Global`, `Blocking`, `Diverge`, `Fail`,
  `Reentrant`, `ThreadAffine`). `Foreign` boundaries unify rigidly.
- **D69 (P11 ownership)**: `RawPtr`/`Borrowed`/`Retained`/`Owned` plus
  `Out`/`MaybeNull`; `Owned` is linear and only weakens to borrow/raw;
  every `Owned A` result requires a matching consuming foreign (same
  allocator), recorded as the contract's `deallocator` symbol and
  checked program-wide.
- **D70 (P11 nullability)**: pointer roles are non-null by default;
  `MaybeNull` marks nullable; a contract violation takes the fault path
  (71). Zero-length buffers cross as non-null dangling pointers per the
  C convention (recorded in the contract).
- **D71 (P11 signature type members)**: `opaque type`/`record` members
  in a `signature` desugar ahead of the signature (module-scope,
  order-dependent like other types); per-instance associated types are
  the P8 packaging gap.
- **D72 (A6 shape)**: `SchemaSnapshot`/`Database`/`SchemaRead` are
  compile-time-only; introspection runs over the subprocess capability
  (the credential travels in the tool environment under its own name
  and never enters captured inputs); `introspectConsistently` checks the
  document and records dialect/collation/extensions/nullability,
  provider identity, and a content hash; `CodecGenerator.derive` emits
  per-table record plus `encode`/`decode` as ordinary checked
  declarations. Codec columns cover `INTEGER` and `BOOLEAN` this slice;
  `REAL`/`TEXT` report explicitly at generation time.
- **D73 (P11 intrinsics)**: the intrinsic symbol namespaces stay
  reserved; a non-resolving name there is a retired-intrinsic error,
  never treated as a C symbol.
- **D74 (P12 surface)**: the structured-concurrency API is builtin
  values with compiler-known schemes (`withScope`, `spawn`,
  `joinAllOrCancel`, `ioWrite`/`ioRead`/`ioSleep`/`pollCancel`) and
  builtin types `Scope r s`/`Task s x a`. `withScope`'s scope region is
  rigid per call, so handles of distinct scopes never unify.
- **D75 (P12 task outcome)**: a task body is `Unit ->{e} Result x a`;
  `spawn` flattens to `Task s x a`; `joinAllOrCancel` returns `Result x
  (...)`, whose `Err` channel is the bodies' `Err` channel: the first
  observed failure is returned after every sibling's cleanup is awaited.
  Cancellation surfaces through the body's own `Err`.
- **D76 (P12 handle discipline)**: any binder whose type mentions
  `Task` is `%1` regardless of annotation; application sites refuse a
  task-mentioning parameter that is not `%1`/flex (closing the
  polymorphic-sink drop); `withScope`'s result must not mention the
  scope region; task/scope values cannot be captured by unbounded
  lambdas.
- **D77 (P12 captures)**: one-shot frames for task and scope bodies (a
  capture use counts once per spawn execution). Share-or-transfer:
  `%1`/`%aff` transfers exactly once (including on cancellation
  early-returns), `Ref`/`Copy` and shareable dictionaries share, `Mut`
  is always refused, untransferred `Bytes` and `Lazy` cells refused;
  live memo-region borrows only when the region predates the scope.
- **D78 (P12 join)**: `joinAllOrCancel` is n-ary (n >= 1, n-tuple
  payload); first observed failure cancels the rest and every
  acknowledgment is awaited (unbounded: `Diverge`); scope exit cancels
  and joins unjoined children as defense in depth.
- **D79 (P12 blocking)**: `Blocking`-labeled foreign calls run on the
  task's dedicated OS thread, are cancellation-blind until they return,
  and their label is accounted in the body row, folded into the
  spawning scope's performed row so effect honesty forces the program
  to carry `Blocking`.
- **D80 (P12 memory model)**: one OS thread per task; fork-join
  happens-before at spawn and join; cancellation is a sequentially
  consistent atomic observed only at defined points (`pollCancel`,
  `ioSleep`); no other atomics. The P14 Wasm backend must preserve the
  same permitted observations.
- **D81 (P12 Io surface and ABI)**: `ioWrite` (stdout, flushed),
  `ioRead` (stdin, `None` at end of input), `ioSleep` (millisecond
  slices, early return on cancellation), `pollCancel` (false on the main
  thread), effect-labeled per P6; the task outcome ABI fixes the
  prelude `Result` tags (Err 0 / Ok 1) and the apply-stub calling
  convention. The sketch's `Request`/`LoadError`/`fetch` vocabulary
  lives in the prelude over `ioRead` (a short read is the `LoadError`
  case), so acceptance example 11 accepts unchanged.
- **D82 (P13 upgrade surface)**: `upgrade Old to New where { migrate =
  f, related = r }` maps the reserved component names; `migrate` is the
  checked migration `Old %1 -> New`; migration field checking is
  field-change classification (kept/added/removed/retyped) with a
  migration required for incompatible changes and a type match on the
  migration itself.
- **D83 (P13 components)**: the migration and `related` components name
  visible values (Vars); the bootstrap evolves monomorphic records
  (parameterized-record evolution is a recorded limit).
- **D84 (P13 record views)**: a migration-free `upgrade` is a record
  view admitting only additive changes; added optional fields are
  recorded as missing-able, keeping missing distinct from present-`None`
  at the schema-evidence level. The value-level `Missing`/`Present`
  marker needs a partial-record view type former the bootstrap `Option`
  lacks (follow-up).
- **D85 (P13 evolution runtime)**: runtime evolution is the one
  code-retention/dispatch mechanism (supercompilation section 6); state
  migration advances the representation version and preserves the
  logical payload; value-level transforms dispatch on
  `(from_repr, to_repr)` (follow-up).
- **D86 (P13 repr identity)**: `#[repr]`-level changes are a
  representation change in the dependency table: they alter the
  interface identity so stale dependents must be replaced (the existing
  staleness chain detects it).
- **D87 (P13 related)**: `related` is accepted as evidence syntax but
  its proposition type is not yet checked (rides the view-type
  follow-up).
- **D88 (Prel String naming)**: the runtime text type is `String` (the
  user's naming call), declared in the prelude as a UTF-8 newtype over
  `Bytes`. D43's "compile-time-only `String`" deferral is resolved: the
  staging interpreter's strings are the same type observed at compile
  time. String literals remain a surface future item; strings are built
  from `Bytes`, `Char`, and the formatting functions.
- **D89 (Prel UTF-8 acceleration)**: the prelude `String` library is the
  REFERENCE SEMANTICS for UTF-8 (validation, decoding, encoding, code
  point indexing); the Rust runtime carries SIMD-accelerated
  implementations of the same rules (`cinch_utf8_*`, NEON/SSE2 fast paths
  with naive fallbacks, tested to agree). Acceleration is a
  representation detail behind the primitive operations chapter's
  justified bodies: the prelude may keep the pure reference or swap to
  `cinch.runtime.utf8*` intrinsics under the same tests (the
  reference-then-accelerate pattern).
- **D90 (Prel operator precedence)**: surface binary operator
  precedence follows the Rust reference table on the 0-9 scale
  (`* / %` 9, `+ -` 8, `<< >>` 7, `&` 6, `^` 5, `|` 4, comparisons 3,
  `&&` 2, `||` 1); the new operators are left associative like Rust's.
  The relative order of pre-existing operators is unchanged, so no
  previously valid program re-parses. `&`, `|`, `^`, `<<`, `>>` are
  wrappers over the `Bits` class (never new compiler mechanisms);
  bitwise NOT has no operator spelling (`!` and `~` are reserved).
- **D91 (Prel Rem/Div)**: `%` and `/` are the `Rem` and `Div` class
  methods with instances for the mathematical `Nat` and `Int` only;
  machine integers keep the explicit checked/wrapping/saturating
  families (primitive types specification sections 4 and 8). Division or
  remainder by zero takes the defined fault path (exit 72).
- **D92 (P14 eval mechanism)**: `eval` runs a build-time-reified
  program on the runtime evaluator (`codegen.rs` encoder and
  `runtime/src/eval.rs` decoder are one documented format) rather than
  embedding the compiler as a library; the supercompilation chapter's
  section 7 contract holds (typed by quotes, never evidence, recorded in
  `assumptions.foreign`, profiles MAY forbid it) and this is the only
  realistic lowering for wasm32 guests.
- **D93 (P14 adapter scope)**: every WIT function gets a call-side
  `Raw.<func>` binding; a world `export` additionally emits the
  canonical trampoline forwarding to the implementation unit (checked
  at admission).
- **D94 (P14 glue boundary)**: canonical tables are computed for all
  shapes; the runtime glue marshals scalars, byte buffers, handles,
  enums/flags, and single-layer records/variants of those. Nested
  aggregate payloads and more than 16 flat params are refused at
  emission with the shape named (honest boundary, no stubs).
- **D95 (P14 stream observations)**: `stream`/`future` cross as
  word-level host imports over the runtime block convention; full
  canonical stream interop is a named boundary. WASI streams are typed
  `WasiStream`/`WasiFuture` with `Diverge` observation rows, never the
  productive codata `Stream`.
- **D96 (P14 admission split)**: `check` is not artifact admission:
  `cinch check` accepts programs while `admit_artifact` enforces the
  assumed baseline and the `wasm.component-model` requirement for
  adapters on Wasm profiles (native lowers through the FFI shim).
- **D97 (P14 foreign label args)**: synthesized schemes quantify the
  `Foreign <boundary>` name like user `foreign import` schemes (it
  renames per call).
- **D98 (P14 decide operands)**: the index builtins `length`/`toNat`
  compute their runtime test values through `bytesLen`/`natFromI64`,
  matching their index reading.
- **P14 deferrals (reported)**: all four continuation items are closed
  (the quote-block link case is green; the canonical glue emits the
  codec marshalling; variant packaging round-trips through the P13
  machinery; the manifest `wasm` section records profiles and adapters).
  The standing emission boundary (D94) is unchanged: nested aggregate
  payloads, non-`u8` element lists, and more than 16 flat parameters
  refuse at emission with the shape named, while their canonical tables
  are still computed and recorded. Eval opcode semantics settled with
  the delegate (nonzero `IF` is true; `CASE` tags unique; `BIG` padding
  zero; nested faults via a per-thread pending fault) are recorded in
  the slice transcript.
- **D99 (P14 canonical glue target)**: the canonical ABI is the wasm32
  shape (i32 pointer slots, canonical memory layout), so adapter glue
  emits for Wasm artifact profiles only and native emission refuses with
  a named diagnostic (revising D96's FFI-shim remark: there is no native
  canonical lowering). `admit_artifact` runs at artifact production and
  enforces the assumed baseline and world-export implementation
  obligations (a `world` export without its implementation is rejected).
- **D100 (P14 packaging identity)**: a packaged variant's identity is
  the content hash of its emitted module produced through the same
  optimization path as a build, so profile parameters are part of the
  artifact's meaning; the loader record is deterministic and re-loadable
  by re-packaging.
- **D101 (P14 stale runtime libraries)**: `ensure_runtime` treats a
  staticlib older than any runtime source as absent and rebuilds; a
  stale `libcinchruntime.a` silently drops newer `cinch_*` entry points (the
  root cause of the eval link failure).
- **D102 (P19 dashes and minus)**: dash identifiers are written tight
  (`-` between identifier characters is glue; digits do not continue an
  identifier, so `x-1` is subtraction); a `-` tight only to following
  digits is a negative-literal sign (`f -42` applies to `-42`); other
  `-` is the binary minus; prefix `-` negates any expression through
  `Neg`. Prelude function names move to dash style; type/class names
  stay capitalized; intrinsic symbol names are unchanged.
- **D103 (P19 guarded letrec)**: `let` binds groups (`;`-separated,
  `where` declaration grammar); `letrec` admits recursion only under a
  lambda, a `~` memo body, or a `delay` (the explicit suspension
  positions), so recursion never introduces implicit laziness; eager
  cycles are rejected naming the variable and the missing guard.
- **D104 (P19 tail calls)**: tail-recursive definitions (self and
  mutual) run in constant stack: self tail calls compile to jumps,
  mutual tail calls use tail-marked calls at the ABI boundary; a group
  that cannot be tail-marked falls back and the report says so.
- **D105 (P19 collection literals)**: `[a, b, c]` desugars to
  `from-list` at the expected `FromList c e` instance (default `List`);
  `{k => v}` desugars to `from-map` over pairs at the `FromMap m k v`
  instance; the prelude gains `Map k v` (ordered map over `Ord k`).
- **D106 (P19 prelude naming)**: prelude function names use dash
  style; this supersedes the D6/D64 spelling notes for the prelude's own
  API surface (intrinsic symbol identity is unchanged).
- **D107 (P19 exports and re-exports)**: `export name, ...`
  declarations re-export names including imported ones (`export
  module X` bulk, `export A.b` named); without them a module exports
  its declared names as before and imports stay private.
- **D108 (P19 import renaming)**: `import X as Y` and `import X (a as
  b, c)` rename/select; duplicate unrenamed imported names are a
  compile error naming both sources.
- **D109 (P19 file-type converters)**: `import data "f.ext" as x`
  converts non-source files at compile time; extensions select
  converters (built-ins json/txt/bytes; plugins are stage modules under
  the P9 discipline); the manifest records converter identity and the
  file content hash (the contract covers the conversion); converter
  failures name the file and converter.
- **D110 (P19 compile-time literals)**: collection literals expand
  through the hygienic macro system to `from-list`/`from-map`; when the
  elements are stage-known the construction is evaluated at compile
  time and emitted as a constant (reusing the D92 evaluator and the P14
  constant emission), otherwise the runtime class call stands; the fold
  boundary is reported.
- **D111 (P20 assertion vocabulary)**: safety assertion attributes
  (`#[recursion "none"|"tail"|"bounded(n)"]`, `#[stack]`, `#[no-alloc]`/
  `#[alloc "once"]`, `#[max-complexity]`/`#[max-lines]`, `#[assert-min]`,
  `#[checked-params]`, `#[no-macros]`, `#[no-raw-ptr]`,
  `#[deny-warnings]`) map the JPL "Power of 10" rules to compile-time
  checks; six rules are inherent to the language (scoping, usage
  checking, `Result` as a type, no pointer arithmetic, hygienic bounded
  macros, no goto).
- **D112 (P20 assertions are contracts)**: each assertion is verified
  at the definition (violations name the rule and the offending site)
  and, when satisfied, recorded in `.cinchif` and hashed into
  `iface_hash` exactly like `repr` facts, so consumers may rely on it
  across calls; breaking an assertion does not compile.
- **D113 (P20 depth bounds)**: `bounded(n)` proves depth through the
  structural recursion machinery first and falls back to conservative
  call-graph bounds otherwise, with the report saying which was used;
  bounds that cannot be established are refused, never trusted.
- **D114 (P20 jpl profile)**: the `jpl` profile (module attribute or
  target profile) mandates the full set at the JPL rule numbers
  (complexity 10, lines 60, assert-min 2 on exported functions, no
  allocation outside initialization, no macros, no raw pointers,
  deny-warnings) with a whole-program summary; per-definition
  attributes may be stricter, never looser. Discarding a `Result`
  without `ignore` is a check error (rule 7 as a language default).
- **D115 (A5 instantiation)**: instantiation is import-time
  application with static arguments (`import M (args) as N`): a unit is
  a distinct compilation unit with computed identity
  `unit_id = hash(target | parameter signatures | argument identities)`;
  dictionary arguments pass through the existing instance machinery,
  so "instantiation is representation" holds (the parameterized
  module's `iface_hash` is invariant).
- **D116 (A5 no functor composition)**: an instantiation inside a
  parameterized module, a parameterized instance used as an argument,
  and index-position references to a parameterized module's functions
  are rejected with named errors; forwarding parameters through nested
  units is full functor machinery and rejection is honest rather than
  silently wrong.
- **D117 (A5 parameter kinds)**: capability (`Io`) and `Copy`
  parameters are rejected (capabilities stay lexical per D21; `Copy` is
  structural evidence); value parameters require `Copy` evidence
  because every definition shares them.
- **D118 (A5 definition scope)**: `stage`/`macro`/`generate`/
  `rewrite`/`upgrade` declarations in a parameterized module are
  rejected (their evidence or compile-time code would need the
  parameters); types, signatures, instances, effects, fixities, and
  foreign imports are unaffected.
- **D119 (A5 explicit dictionaries)**: explicit (named) dictionaries
  coexist for one class; coherence governs default resolution over
  implicit instances only (two instantiations publishing unifiable
  implicit copies are rejected as `ambiguous instance`, never merged);
  module arguments are explicit dictionary selection per the modules
  spec section 1.
- **D120 (A5 prelude sketch vocabulary)**: the sequencing chapter's
  `loadConfig` sketch vocabulary (`Path`, `Config`, `ConfigError`,
  `readConfigBytes`, `parseConfig`) joins the prelude over `ioRead`,
  mirroring D81's `fetch` pattern, so example 12 accepts end to end.
- **D121 (A5 import refs)**: a module's dependency on an interface is
  a set: import refs are deduplicated in the interface (two applied
  imports of one module record one dependency).
- **P13 deferrals (reported)**: handler-vtable specialization is blocked
  on propagating concrete vtables across function boundaries (dispatch
  sites can live in callees through `do`/`spawn`); explicit `eval` rides
  the P14 item; parameterized-record evolution and the value-level
  missing/present markers are follow-ups named in D83-D85.
