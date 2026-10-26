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
| `01-bounded-index.lang` | `Nat`, `Fin`, `Vec`, `Option`, `decide`, dependent results | todo |
| `02-vector-filter.lang` | `Sigma`, `Vec`, dependent pattern matching, `Copy` | todo |
| `03-map-fusion.lang` | `proof`, `refl`, `congr`, `Eq`, `rewrite ... justify` | todo |
| `04-productive-stream.lang` | `codata`, `Later`, `corec`, guarded `delay` | todo |
| `05-schema-generator.lang` | `stage`, `generate do`, `import stage`, compile-time capabilities | todo |
| `06-counter-upgrade.lang` | `record`, `%1` multiplicities, `upgrade`/migration evidence | todo |
| `07-rejected-programs.lang` | each listed rejection must be diagnosed | todo |
| `08-platform-simd.lang` | `U32.wrappingAdd`, `Ref`/`Vec`, `target rewrite`, capability dispatch | todo |
| `09-foreign-codec.lang` | `signature` with `opaque type`, erased fields, FFI effects | todo |
| `10-wasm-component.lang` | WIT adapter, `do` on `Result`, `decide`, component effects | todo |
| `11-structured-tasks.lang` | `using (io :: Io r)`, `spawn`, scopes, `->{Async r, IO r, Cancel r}` | todo |
| `12-runtime-effects.lang` | `do` with sequencing providers, module parameters, row-polymorphic effects | done (accepts end to end) |

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
| Compiled interface metadata artifacts | done (`.sliface` canonical JSON with content identity, emitted per module into `build/`, parsed back in tests) |
| Cross-module instance coherence and fixity export | done (unifiability check in the checker plus the metadata fast path; fixity/type/signature/instance exports; cross-module instances resolve) |
| Artifact manifests (assumptions, trusted components) | done (`<entry>.manifest.json` with module identities, union assumptions, `runtime_abi`; per-module `Assumptions` carry the primitive axioms) |

Caveat recorded: the whole project is compiled as one program (one IR, one
link) from the union of sources; compiling a single module against prebuilt
`.sliface` files without its dependencies' sources is the next step and
needs the checker to consume export tables instead of sources.

### Runtime library (P4)

| Requirement | Status |
| --- | --- |
| Rust runtime crate (`slruntime` staticlib + rlib) linked into artifacts; fault policy (`BlackHole` 70, panic 71, div-zero 72) | done (60 tests; `compiler/runtime/`; all `sl_*` externs `#[unsafe(no_mangle)]`) |
| Mathematical `Nat`/`Int` (bignum) | done (language types on the bignum runtime, unbounded digit-string literals, `Eq`/`Ord`/`Add`/`Sub`/`Mul`, `Neg` for `Int`, truncating `Nat` subtraction, `main :: Int` exit adapter) |
| `Bytes`, string foundations | done for `Bytes` (`new`/`len`/`get`/`set`, bounds faults at 71); string literals stay rejected until the staging or FFI slice gives them semantics |

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
| `foreign import ccall` intrinsics (`safelang.prim.*`) | done (namespaces reserved; retired names rejected as retired intrinsics, D73) |
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
| `upgrade` declarations, migration checking | done (`upgrade Old to New where { migrate = f, related = r }` (D82/D83); field-change classification with migration required for incompatible changes; a migration-free `upgrade` is a record view recording schema compatibility evidence in the `.sliface` `upgrades` table (D84); `#[repr]` changes alter the interface identity, forcing dependent replacement (D86); `tests/evolution.rs`) |
| Versioned dispatch, state migration | done (the runtime evolution arena: epochs and activation publishing, code retention with live-frame counts (spec section 6), complete-update-set gating (section 8), no-old-code-over-new-state with authorized reverse transitions (section 10), target compatibility before reachability (section 13); `sl_evo_*` externs, `nm`-verified; D85) |
| JIT deopt (tracked item) | done (transparent tiering over the same machinery: variants install/retire through runtime evolution only, consumption-safe deopt with the unbalanced-linear-transfer refusal, nothing in types/source/interface; `tests/evolution.rs`) |

### Wasm and components (P14) — webassembly and wasi specification

| Requirement | Status |
| --- | --- |
| Wasm artifact profile, feature selection | done (wasm32 triple + data layout with target-keyed widths (A4), `admit_artifact` reject-or-lower at artifact production with world-export obligations (D96/D99), cooperative task backend preserving D80 observations, and real builds honoring the profile at emission; `tests/wasm.rs` 11/11) |
| WIT parsing, component adapters | done (WIT subset parser; canonical-ABI tables and the runtime glue for scalars/bytes/handles/enums/flags and single-layer aggregates, asserted exactly on the codec case (D93/D94/D99); `BoundaryError`/`DecodeError` vocabulary; `WasiStream`/`WasiFuture` with degraded `Diverge` observations (D95); `eval` over the reified runtime evaluator with its end-to-end case green (D92); variant packaging round trip through the P13 machinery (D100); manifest `wasm` section) |

### Acceptance catalogue (P15)

Snapshot 2026-10-25 (`safelang check examples/source/*.lang`, entry rows
update as the slices land; 07 must reject, the rest must accept):

| Program | First rejection | Owning slice |
| --- | --- | --- |
| 01-bounded-index | accepts (`safelang check` exit 0; `run` needs a `main` per D20) | done |
| 02-vector-filter | line 7: `Nil` against `Vec n a` needs rigid-index refinement (D35) | P7 limit (D35) |
| 03-map-fusion | line 6:36 `expected a type, found Backslash` (index terms lack lambda; even then its `refl` at `Nil` is unprovable under D28's no-delta conversion; needs a proof over named helpers or an assumed law) | P7 limit + example-side proof |
| 04-productive-stream | accepts (`safelang check` exit 0) | done |
| 05-schema-generator | accepts with its declared build inputs (`SCHEMA_DATABASE_URL` + a schema tool via `SAFELANG_SCHEMA_TOOL`); without them stops at the correct secret refusal (D45) | done (A6) |
| 06-counter-upgrade | accepts (`safelang check` exit 0) | done |
| 07-rejected-programs | rejected at parse (correct) | must stay rejected |
| 08-platform-simd | line 6:10 the justification cannot prove the rule's equation (the example matches its own local `scalar`; the supplied theorem is over the library pair; the example's header calls its theorem module "obligations, not supplied proofs") | example-side proof obligation |
| 09-foreign-codec | accepts (`safelang check` exit 0; its `DecodeError` comes from the codec adapter vocabulary) | done |
| 10-wasm-component | accepts (`safelang check` exit 0; `BoundaryError` from the WIT adapter vocabulary) | done |
| 11-structured-tasks | accepts (`safelang check` exit 0; its sketch vocabulary `Request`/`LoadError`/`fetch` is provided by the prelude over `ioRead`, D81) | done |
| 12-runtime-effects | accepts (`safelang check` exit 0; the parameter block `module ImportConfig (Seq :: ResultSequencing ConfigError)` instantiates at import, and the sketch vocabulary `Path`/`Config`/`ConfigError`/`readConfigBytes`/`parseConfig` comes from the prelude over `ioRead` following D81's pattern (D120)) | done |

## Unplanned requirements (P16)

Four spec MUSTs have no owning phase in P1-P15 (found by the 2026-10-25
audit, `compiler/SPEC-MAPPING.md` "Roadmap gaps"):

| Requirement | Note |
| --- | --- |
| Positivity discipline for inductive definitions | done (P7: strict positivity for all `data`; negative and nested occurrences rejected) |
| Evidence invalidation over mutable state | flow-sensitive; adjacent to P7 refinements and P13 (A2) |
| Temporal contracts (safety/liveness/recurrence/stability) | closest to P12; unnamed there (A3) |
| Cross-compilation (target width/capabilities) | done (P14: `Usize`/`Isize` widths keyed on the target in literal checks, lowering, and the manifest (A4)) |
| Module parameters and signatures as module arguments | done (A5: `module M (p :: T, ...) where` with dictionary and value parameters, import-time instantiation units `import M (args) as N` with computed `unit_id`, instance coherence preserved (implicit resolution unchanged; explicit dictionaries coexist), parameter signatures in `.sliface` with instantiation as representation (D115-D121); example 12 accepts) |
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
| Dash identifiers and the minus whitespace contract (D102) | spec'd |
| Multi-binding `let` and guarded `letrec` (D103): recursion only under lambda/`~`/`delay`, eager cycles rejected with the fix named | spec'd |
| Tail recursion in constant stack, self and mutual (D104) | spec'd |
| `[...]`/`{k => v}` literals desugaring to `FromList`/`FromMap`, with the prelude `Map` (D105) | spec'd |
| Dash-style prelude function names (D106) | spec'd |
| Export declarations and re-exports (`export module X`, `export A.b`); imports private until re-exported (D107) | spec'd |
| Import renaming and selection (`import X as Y`, `import X (a as b)`); duplicate imports error naming both sources (D108) | spec'd |
| File-type import converters: extension-keyed compile-time conversion (`import data "config.json" as config`), built-ins json/txt/bytes, plugins as stage modules with provenance (D109) | spec'd |
| Collection literals fold at compile time to direct construction (macro expansion + stage evaluation + constant emission); runtime class call otherwise; fold boundary reported (D110) | spec'd |

### Safety assertion attributes (P20)

Specified in `specifications/attribute-annotations-specification.md`
section 10 and `design/safety-assertion-attributes.md`, 2026-10-26, at
the user's direction: compile-time-verified assertions (tail
recursion, non-allocation, stack bounds) and the JPL "Power of 10"
rule mapping (six rules inherent to the language, the rest asserted).

| Requirement | Status |
| --- | --- |
| Assertion vocabulary: `#[recursion]`, `#[stack]`, `#[no-alloc]`/`#[alloc "once"]`, `#[max-complexity]`/`#[max-lines]`, `#[assert-min]`, `#[checked-params]`, `#[no-macros]`, `#[no-raw-ptr]`, `#[deny-warnings]` (D111) | spec'd |
| Verified at the definition; violations name the offending site; satisfied assertions are contract facts in `.sliface`/`iface_hash` like `repr` (D112) | spec'd |
| `bounded(n)` depth bounds via structural proofs first, conservative call-graph bounds otherwise, report saying which (D113) | spec'd |
| The `jpl` profile enforcing the whole set at rule numbers, never looser per-definition (D114); Result must-use default | spec'd |

### Interned symbols (P24)

Specified in `specifications/primitive-types-specification.md`
section 9 and `specifications/surface-grammar-local-amendments.md`
section 12, 2026-10-26, at the user's direction: interned symbols as a
core type with tick literal syntax (the Template Haskell name-quote
correspondence: one lexeme in expression/pattern, staging `Name`, and
`''T` type-name contexts).

| Requirement | Status |
| --- | --- |
| `Sym`: word-handle interned cell; `intern`/`sym-name`; `Eq` = identity $O(1)$, `Ord` by name (deterministic); no machine-scalar confusion (D122) | spec'd |
| Tick syntax: `'foo` symbol literals (constants via compile-time interning and the P14 constant emission), `'a'` char precedence rule, `'foo`/`''T` name quotes in staging/macro positions (D123) | spec'd |
| Type-level symbols (promoted `'foo` for labels/schema names) deferred with the spelling reserved (D124) | spec'd |

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
- **D125 (P25 base library curation, user-approved 2026-10-26)**:
  pulled in: `Algebra.Group`/`Algebra.Semiring`, `Data.Set` +
  `Data.Hash`/`Data.HashMap` (symbol hashing exploits Sym interning),
  class law declarations as citable `justify` evidence (riding P18's
  law/assumption tier), `Text.Show`/`Text.Codec` (A6 alignment).
  Queued: `Data.Enum` + range syntax to the next surface slice.
  Held: Profunctor/Category/Kleisli, State/Reader newtypes. Out: the
  transformer zoo (row effects do it natively), IO-as-a-monad,
  numeric towers beyond Semiring, lenses.

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
  `.sliface` artifact per module (exports, instances for coherence, fixities,
  foreign contracts, assumptions manifest, ABI symbol table), identity by
  content hash, imported interfaces recorded as the "checked against" chain.
  See `../design/compiled-interface-metadata.md`.
- **D5 (P4)**: the runtime is a Rust crate (`compiler/runtime`, `slruntime`
  staticlib + rlib, std-only) linked with generated objects; not C. The
  runtime ABI reference lives in its `src/lib.rs` module docs and is cited by
  interface metadata (`runtime_abi`).
- **D6 (P2)**: machine-type operations are prelude names of the form
  `<op><Type>` (`wrappingAddU32`, `checkedAddI32`, `u8ToI32`, ...) bound
  directly from `foreign import ccall "safelang.prim.*"`; the primitive
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
  `sl_nat_from_decimal`/`sl_int_from_decimal` otherwise. Float literal
  exactness is the odd-part rule (odd part fits the significand and the
  binary exponent is in range). The wrapping-64 bootstrap `Int` is gone.
- **D11 (P4)**: `natCmp`/`intCmp` return the raw -1/0/1 immediate typed at
  `I64` (the runtime extern set has no `natEq`/`intEq`); `Eq`/`Ord`
  instances derive as `eqI64 (intCmp x y) 0` style and compare the
  immediate against zero with the `I64` prims.
- **D12 (P4)**: the checker synthesizes a `"<module>.main.exit"` adapter
  calling `sl_int_to_i64` on `main`'s bignum result and `Program.main`
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
  `<Name>.lang`, then a unique `module Name` beside the entry file
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
  implementations of the same rules (`sl_utf8_*`, NEON/SSE2 fast paths
  with naive fallbacks, tested to agree). Acceleration is a
  representation detail behind the primitive operations chapter's
  justified bodies: the prelude may keep the pure reference or swap to
  `safelang.runtime.utf8*` intrinsics under the same tests (the
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
  `safelang check` accepts programs while `admit_artifact` enforces the
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
  stale `libslruntime.a` silently drops newer `sl_*` entry points (the
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
  and, when satisfied, recorded in `.sliface` and hashed into
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
