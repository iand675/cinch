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
| Effect rows on arrows, honesty checking | done |
| Handler calculus (`handle ... with`, operation clauses) | todo |
| `do` notation; sequencing modules (direct/Result/Option) | todo |
| `using (io :: Io r)`, `with (io = ...) in`, `@{...}` | todo |
| Runtime identity `r` | todo |

### Dependent core (P7) — core spec sections 1-5, 8

| Requirement | Status |
| --- | --- |
| Universes, `Type`, stratification | todo |
| Pi/Sigma, dependent functions and pairs | todo |
| Conversion checking (definitional equality) | todo |
| Implicit arguments, inference | todo |
| Indexed families (`Vec`, `Fin`), dependent matching, coverage | todo |
| `Prop`, `proof`, `erased`, `Eq`/`refl`/`congr`, erasure | todo |
| Totality (structural recursion), `decide`, branch evidence | todo |

### Codata (P8)

| Requirement | Status |
| --- | --- |
| `codata`, `Later`, guarded `delay`/`force` | partial (`Later` is spec; memo/thunk machinery exists) |
| `corec` with productivity checking | todo |

### Staging (P9)

| Requirement | Status |
| --- | --- |
| `stage`, `[| |]`, `$()`, `quoteDecls` | todo |
| `generate do`, `import stage`, captured replay | todo |
| Hygienic macros `!` | todo |

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
| 01-bounded-index | dependent function and pair types unsupported | P7 |
| 02-vector-filter | unknown type `Vec` (indexed `Vec` arrives with the dependent core) | P7 |
| 03-map-fusion | field projection in types | P13 |
| 04-productive-stream | `codata` declarations unsupported | P8 |
| 05-schema-generator | `module` name at top level (stage/import syntax) | P9 |
| 06-counter-upgrade | accepts (`safelang check` exit 0) | done |
| 07-rejected-programs | rejected at parse (correct) | must stay rejected |
| 08-platform-simd | `rewrite` declarations unsupported | P10 |
| 09-foreign-codec | `Opaque` unsupported | P11 |
| 10-wasm-component | `erased` fields unsupported | P7 |
| 11-structured-tasks | `do` expressions unsupported | P6 |
| 12-runtime-effects | `do` expressions unsupported | P6 |

## Unplanned requirements (P16)

Four spec MUSTs have no owning phase in P1-P15 (found by the 2026-10-25
audit, `compiler/SPEC-MAPPING.md` "Roadmap gaps"):

| Requirement | Note |
| --- | --- |
| Positivity discipline for inductive definitions | applies to ordinary `data` today; needed before P7 (A1) |
| Evidence invalidation over mutable state | flow-sensitive; adjacent to P7 refinements and P13 (A2) |
| Temporal contracts (safety/liveness/recurrence/stability) | closest to P12; unnamed there (A3) |
| Cross-compilation (target width/capabilities) | `Usize` width and capability checks must key on the target (A4) |

### Attributes, weights, inline hints, and layout (P17)

Specified in `specifications/attribute-annotations-specification.md`
(normative) and `design/attribute-annotations.md` (implementation
design), 2026-10-25. The weight algebra follows GHC proposal 744
(Branch Prediction Hints, Ian Duncan). Prerequisite for P10's optimizer
and P11's FFI layout work.

| Requirement | Status |
| --- | --- |
| `#[...]` attribute system: syntax, attachment points, compiler vs macro ownership, hygiene and provenance, identity rules | spec'd |
| Manual branch weights `#[weight(n)]` with the marginalization and transformation rules; `cold`/`likely`/`unlikely` as wrappers over weights (2000/1/0) | spec'd |
| Inline hints `#[inline]`/`#[inline(always)]`/`#[inline(never)]` as erasable optimizer hints | spec'd |
| `repr(C)`/`repr(transparent)`/`repr(align)`/`repr(packed)`/discriminant representations with layout facts in the interface ABI table and the packed-borrow safety rule | spec'd |

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
