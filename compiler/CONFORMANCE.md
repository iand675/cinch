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
| `12-runtime-effects.lang` | `do` with sequencing providers, module parameters, row-polymorphic effects | todo |

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

### Rewrites and optimizer (P10)

| Requirement | Status |
| --- | --- |
| `rewrite` / `target rewrite` with `justify` | todo |
| Optimizer with preservation discipline | todo |
| Target profiles, capability evidence, SIMD dispatch | todo |

### FFI (P11)

| Requirement | Status |
| --- | --- |
| `foreign import ccall` intrinsics (`safelang.prim.*`) | done |
| Real C symbols, `foreign export`, effects, layouts | todo |
| `opaque type`, `RawPtr`, ownership roles | todo |

### Concurrency (P12)

| Requirement | Status |
| --- | --- |
| Structured scopes, `spawn`, join/cancel | todo |
| Task captures with ownership | todo |

### Runtime evolution (P13)

| Requirement | Status |
| --- | --- |
| `upgrade` declarations, migration checking | todo |
| Versioned dispatch, state migration | todo |

### Wasm and components (P14)

| Requirement | Status |
| --- | --- |
| Wasm artifact profile, feature selection | todo |
| WIT parsing, component adapters | todo |

### Acceptance catalogue (P15)

Snapshot 2026-10-25 (`safelang check examples/source/*.lang`, entry rows
update as the slices land; 07 must reject, the rest must accept):

| Program | First rejection | Owning slice |
| --- | --- | --- |
| 01-bounded-index | accepts (`safelang check` exit 0; `run` needs a `main` per D20) | done |
| 02-vector-filter | line 7: `Nil` against `Vec n a` needs rigid-index refinement (D35) | P7 limit (D35) |
| 03-map-fusion | field projection in types | P13 |
| 04-productive-stream | accepts (`safelang check` exit 0) | done |
| 05-schema-generator | line 3: `unknown type SchemaSnapshot` (schema-introspection API sketch, modules spec section 11) | schema-generation slice |
| 06-counter-upgrade | accepts (`safelang check` exit 0) | done |
| 07-rejected-programs | rejected at parse (correct) | must stay rejected |
| 08-platform-simd | `rewrite` declarations unsupported | P10 |
| 09-foreign-codec | `Opaque` unsupported | P11 |
| 10-wasm-component | line 8: effect label `Foreign Component` is P11 vocabulary | P11 |
| 11-structured-tasks | unknown type `Request` (structured-concurrency vocabulary) | P12 |
| 12-runtime-effects | line 14: module parameters (`module ImportConfig (Seq :: ...)`) unsupported | modules follow-up (parameters) |

## Unplanned requirements (P16)

Four spec MUSTs have no owning phase in P1-P15 (found by the 2026-10-25
audit, `compiler/SPEC-MAPPING.md` "Roadmap gaps"):

| Requirement | Note |
| --- | --- |
| Positivity discipline for inductive definitions | done (P7: strict positivity for all `data`; negative and nested occurrences rejected) |
| Evidence invalidation over mutable state | flow-sensitive; adjacent to P7 refinements and P13 (A2) |
| Temporal contracts (safety/liveness/recurrence/stability) | closest to P12; unnamed there (A3) |
| Cross-compilation (target width/capabilities) | `Usize` width and capability checks must key on the target (A4) |
| Module parameters and signatures as module arguments | modules spec section 1 (module packaging); example 12 stops here (A5) |
| Schema generation / database introspection API | modules spec section 11 sketch; example 05 needs `SchemaSnapshot`, `CodecGenerator.derive`, `Stage.withDatabase`/`ReadSchema`/`introspectConsistently`; pair with P11 subprocess tooling (A6) |

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
| What specialization manages (dictionary/handler/index/effect-static specialization, fusion, constant folding) with barrier rules (effect order, linear values, memo, codata, erasure, rigid indices) | spec'd |
| Preservation obligations: residual program rechecked (types, effect order, `%1` usage, memo sharing, productivity, erasure, capability rows); specialization representation-only (contract identity unchanged) | spec'd |
| Driving bounds (whistle, fuel as target profile parameters) and the compiler report obligations (where specialization stopped) | spec'd |
| JIT invisible to types/source/interface; consumption-safe deoptimization; variant install/retire through runtime evolution only | spec'd |
| Explicit `eval` of typed quotes: no capability in the signature, never evidence, manifest-recorded, target profiles MAY forbid | spec'd |
| Trust levels: manifest identity, full recheck (default), step certificates (future) | spec'd |

## Spec requirement mapping

Every spec MUST is mapped to enforcement and tests in
`compiler/SPEC-MAPPING.md` (87 rows, statuses I/P/N/CV, plus the wording
flags for future decisions).

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
