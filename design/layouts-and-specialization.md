# Layouts, representation and specialization

Status: design direction for review, 2026-10-29. It records the
conclusions of the reclamation design discussion with the user and the
choices still open. Companion to `design/reclamation.md` (Perceus reference
counting, which needs every word's representation) and to
`design/verified-primitives.md` (the proof chain that specialization's
justification rests on). Normative text lands in the specifications when a
slice implements it; until then this note is the reference.

## 1 Requirements (stated by the user)

- **GC-free.** No tracing collector, ever. Reclamation is Perceus-style
  precise reference counting (`design/reclamation.md`).
- **Control over memory layout** for types, as the attribute specification's
  `repr` family already begins to give (`specifications/attribute-annotations-specification.md`
  section 4).
- **Primitives never allocate, in any position** (`specifications/primitive-types-specification.md:22`
  stays true without amendment).
- **Generic data types with ad hoc specialized representations**: one
  `List a` used generically everywhere, and a different representation for
  particular instantiations (`List Bool` as a bitset) that generic code
  still works over unchanged.
- **Proofs carried by specialization**: an alternative implementation is
  justified equal to its baseline, not trusted.

## 2 Words: specialization over three representation classes (option C)

Every `Core` value is one untyped `i64` word today, so a drop cannot tell a
pointer from a scalar in a polymorphic slot. Of the three options laid out
in `design/reclamation.md` section 2 (A tagged scalars, B witnesses, C
specialization), the user's direction is **C, with demand-driven
specialization**:

- A binder or field's word has one of three classes: `Raw` (never counted:
  machine scalars, `Bool`, `Char`, `Unit`, handles), `Obj` (a counted
  pointer, possibly immortal), `TObj` (an inline small value with low bit 1,
  or a pointer: `Nat`/`Int` once small immediates land).
- A polymorphic function is compiled once per **class vector** of its type
  variables that some caller reaches, never per source type. `map` at
  `String -> String`, at `List Int -> Foo` and at every record type is one
  `[Obj, Obj]` copy; at `I64 -> Char` it is a `[Raw, Raw]` copy with no count
  operations. At most `3^k` versions exist for `k` type variables; only the
  reached ones are generated.
- **The set of versions is finite under polymorphic recursion**: `List a` is
  `Obj` whatever `a` is, so the collector always terminates. Rust, which
  specializes per type, refuses some polymorphic recursion for exactly this
  reason.
- **What it needs**: representation classes on `Core` binders and constructor
  fields (common to every option); polymorphic bodies in interfaces as IR
  (Rust ships generic MIR in crate metadata for the same purpose); a collector
  at the final compile that starts from `main`; specialized instance
  dictionaries and parameterized-module applications; abstract types exposing
  their class in the interface. Every instantiation is static (schemes are
  prenex, there are no existentials), so there is no run-time dispatch.
- **Fits the store**: the package store keeps bodies as objects AND IR (a
  user decision), and a generated copy is cached by (function content hash,
  class vector), so it is built once per machine.
- **Costs**: polymorphic code is compiled by its consumer (slower first
  builds, absorbed by the store); hot replacement must swap every version in
  use; binary size grows by the versions used.

Precedent: the .NET CLR generates instantiations just in time and shares
code between "compatible" instantiations, all reference types together and
value types with "the same pattern of traced pointers" ([Kennedy and Syme,
PLDI 2001](https://www.microsoft.com/en-us/research/wp-content/uploads/2001/01/designandimplementationofgenerics.pdf),
section 4.1). Swift's explicit specialization pitch names "specialize once
for all particular type layouts" as its partial-specialization direction
([pitch](https://forums.swift.org/t/pitch-explicit-specialization/76967)).

## 3 The layout model: every type has a layout

A **layout** is a size, an alignment, field offsets, and which words are
counted pointers (the pointer map reclamation needs).

- **Default layout**: one word, with the three classes of section 2 inside
  it. Without an attribute, a type is one word: a scalar, or a pointer to a
  block whose fields are one word each.
- **`repr` attributes** (`repr(C)`, `transparent`, `align(N)`, `packed(N)`,
  `u8..u64` discriminants) choose a type's own block layout exactly as the
  attribute specification says. New: the pointer map comes out of the same
  computation, and a `packed` layout may not contain counted pointers
  (unaligned count updates; unaligned borrows are already refused).
- **Value types** (working spelling `#[repr(unboxed)]`): the type is stored
  inline wherever it is used, as a C# `struct` or an OxCaml unboxed record is.
  An unboxed `Point` of two `I64`s has layout `bits64 & bits64`. With
  `repr(C)` it crosses the FFI boundary by value.
- **Field unpacking**: `#[unpack]` on a field of a fixed (monomorphic) type
  inlines that type's fields into the parent, as GHC's `UNPACK` does on strict
  fields (Cinch fields are strict by default, lazy specification section 7).
  `#[nounpack]` is the override.
- The three classes of section 2 are the sub-layouts of the default
  one-word layout, the same distinction OxCaml's "separability" axis draws
  inside its `value` layout ([OxCaml unboxed types](https://oxcaml.org/documentation/unboxed-types/intro/)).

## 4 Beyond one word: enumerated layouts

A multi-word value cannot sit in a default one-word slot. Rather than an
open `forall (l :: Layout)` (whose specialization would not terminate under
layout-changing polymorphic recursion such as `data Nest a = Leaf a | Node
(Nest (a, a))`), a definition **lists** the layouts it supports:

```
#[layouts(l = (word, float64, bits64 & bits64))]
map :: forall (a :: TYPE l) b. (a -> b) -> List a -> List b
```

- One copy is generated per listed layout: finite by enumeration, so the
  termination question disappears.
- Instantiating at a layout outside the list is a type error naming the list.
- The type checker selects the copy from the inferred layout; callers never
  name it. (OxCaml's [`ppx_template`](https://oxcaml.org/documentation/templates/reference)
  does the same generation as a preprocessor, which is why its callers must
  select a copy by name.)

## 5 Hand-written specialized bodies

A developer may give a function a body for a specific instantiation, for
example `sum` at `List F64` through a SIMD `prim`. The generic body is the
required baseline; the specialized body is chosen statically at that
instantiation and recorded in the manifest. This is the `prim` model (a
baseline body plus bodies chosen by a static condition) with the condition
being the instantiation instead of the target capability, and it is an
instantiation-keyed `rewrite` (D55): it reuses that machinery, checks and
justification tiers rather than adding a parallel mechanism.

## 6 Representation instances: generic types, ad hoc representations

```
data List a where
  Nil  :: List a
  Cons :: a -> List a -> List a

represent List Bool as BitList where
  Nil       = bit-list-empty
  Cons b bs = bit-list-cons b bs
  view xs   = case bit-list-uncons xs of
    None           -> Nil
    Some (b, rest) -> Cons b rest
  law view-nil   :: Eq (view bit-list-empty) Nil
  law view-cons  :: (b :: Bool) -> (bs :: BitList) -> Eq (view (bit-list-cons b bs)) (Cons b bs)
  law build-view :: (xs :: BitList) -> Eq (rebuild (view xs)) xs
  specialize length = bit-list-length
  specialize index  = bit-list-index
```

- `List Bool` is still the type `List Bool`; there is no wrapper type and no
  class constraint (the difference from data families).
- When the collector compiles a generic function at `List Bool`, building
  `Cons y ys` calls the representation's builder and matching `case xs of
  Cons y ys` reads `view xs` first: Wadler's views, which also became GHC's
  bidirectional pattern synonyms. Every existing generic function works
  unchanged; `specialize` bodies are fast paths.
- **The representation is a proven isomorphism** with the canonical
  constructors (section 7). If the laws hold, no program can distinguish the
  representation except by speed. Requiring an isomorphism avoids the problem
  Wadler's original views had when a view was not one.
- **Rules**, learned from C++'s `vector<bool>`, whose specialization changed
  the interface (`operator[]` returns a proxy, so generic code taking an
  element's address no longer compiles):
  - generic code must never fail to compile at a represented instantiation:
    a field borrow needs a borrowing view from the representation, or the
    representation is admitted only at `Copy` element types (the proposed
    starting point: it covers bitsets and packed numeric arrays);
  - coherence: exactly one representation per instantiation program-wide,
    declared in the type's module or the element type's module (the orphan
    rule), with disjoint keys;
  - a representation is chosen by types only, never by multiplicity, region,
    effects or proofs (Rust's specialization is [known to be unsound](https://std-dev-guide.rust-lang.org/policy/specialization.html)
    because it chose implementations by erased lifetimes);
  - the representation is part of the interface; consumers compile their
    generic code against it.
- **Cost honesty**: semantics are guaranteed, asymptotics are not. `Cons` on
  a packed array is a copy, so a generic `map` at that instantiation can go
  quadratic without a `specialize map`. The manifest records which functions
  ran through the view and which took a fast path.

## 7 One justification rule for every alternative implementation

> An alternative implementation chosen by a static condition must be
> justified equal to its baseline.

| Alternative | Condition | Baseline |
| --- | --- | --- |
| `prim` target body | target capability | the baseline body |
| `rewrite` | pattern match | the matched term |
| representation instance | type instantiation | the canonical constructors |
| `specialize` body | type instantiation | the generic definition |

Each is justified at one of three tiers, strongest first, all reusing D55's
checking and the law layer:

1. **Kernel proof**: `justify` with checked evidence.
2. **Sample-checked law**: tested against the baseline, reported as tested.
3. **Assumed**: recorded in the interface's `assumptions.declared`, visible
   to every consumer.

A safety profile may require tier 1 for every specialization in a module.
Proofs are `Prop` and erased (D31), so justification costs nothing at run
time. Because a representation is a proven isomorphism, anything proven
about generic code (`map`'s laws) holds at every represented instantiation
without re-proof. Bit-level representation proofs rest on machine-arithmetic
laws; `design/verified-primitives.md` proves those instead of assuming them,
and `design/proof-automation.md` lists the strategies that discharge the
other obligations (isomorphism laws by rewriting and induction, layout and
index bounds by linear arithmetic, glue by congruence closure).

## 8 Sequencing

1. Reclamation S1-S3 (header and a real free, small `Nat`/`Int`
   immediates, drop machinery), which do not depend on anything here.
2. Representation classes on `Core` and the specialization collector
   (reclamation S4/S5 under option C).
3. Value types and field unpacking (section 3).
4. Enumerated layouts (section 4).
5. `specialize` bodies as instantiation-keyed rewrites (section 5).
6. Representation instances (section 6), on D55 and the law layer.

## 9 Open decisions

- Confirm option C formally (the user's stated direction; recorded as D392).
- May the default layout change (the compiler reordering or unpacking fields
  of types without `repr`, recorded in the interface), or is it fixed, with
  an opt-in `#[repr(auto)]`? Proposed: fixed.
- Spelling of value types: a `repr` attribute (proposed), a declaration form
  (`value data`), or OxCaml's `#` syntax.
- Spelling of enumerated layouts (attribute proposed) and of specialized
  bodies (a `prim`-style declaration form proposed, since it carries a body
  and a trust tier).
- Representation instances: `Copy`-only element types first (proposed), and
  disjoint keys only (proposed) or a most-specific-match rule.
- Packed containers (a contiguous `F64` array) are an array type with its own
  layout, not a specialization of `List`; whether the base library grows one
  is separate.


## 10 Context for later agents

### 10.1 The user's requirements, verbatim (2026-10-29)

- "to be honest I was hoping that this language would be able to be gc-free"
  (chosen: Perceus-style precise reference counting; reclamation before any
  new bootstrap-ladder rung).
- "I'm heavily leaning towards C, but perhaps we could lazily specialize by
  carrying the IR needed so that we don't generate function implementations
  for unused versions" (section 2: demand-driven specialization).
- "I did previously say that I wanted control over memory layout for types,
  so how shall we align that with this" (section 3).
- "Could we do a similar thing [to `prim`'s per-architecture bodies] where we
  could allow developers to manually specialize generic types, using an
  OxCaml like approach" (sections 4 and 5).
- "data families are good, but I want my cake and eat it too. Generic data
  types, but specialized versions on an ad-hoc basis" (section 6).
- "that dovetails really nicely with the fact that our language is supposed
  to carry proofs. so you can bake in the proof mechanism to specialization!"
  (section 7).
- "I'd like this to be even more powerful than e.g. Lean" (the proof chain
  under section 7: `design/verified-primitives.md`).

### 10.2 Why not the alternatives (the reasoning trail)

- **Option A (Lean/Koka-style tagged scalars in polymorphic slots)** was the
  reclamation design's original recommendation (`design/reclamation.md`
  section 2.8). It changes no arity and has Perceus precedent, but an
  `I64`/`U64`/`F64` that does not fit 63 bits gets a heap box when stored
  polymorphically (a `List F64` boxes every element), which breaks the
  primitive-types specification's "no allocation" (line 22) and makes
  polymorphic containers of 64-bit values unusable in `#[no-alloc]` code.
- **Option B (Swift-style representation witnesses)** keeps the spec but
  adds an argument per type variable to every polymorphic function and
  constructor, changing the interface arity of every polymorphic export;
  per-block masks need header bits or an overflow word. Highest churn.
- **Option C** has zero runtime overhead and no boxing; its costs (consumer
  compilation, binary size, hot replacement of every version) are absorbed
  by the store and bounded by the versions actually reached.
- **Unbounded layout polymorphism (`forall (l :: Layout)`)** was rejected
  for the collector: specializing on full layouts (Rust-style) diverges on
  `data Nest a = Leaf a | Node (Nest (a, a))`, whose unpacked layouts
  `a, (a,a), ((a,a),(a,a)), ...` never repeat. Three alternatives were
  considered: run-time specialization (the .NET CLR's answer: only reached
  layouts are ever built; Cinch has a JIT specification), a Swift-style
  generic fallback past a budget, and layouts in kinds with the kind checker
  refusing layout-growing recursion (GHC's [levity polymorphism](https://simon.peytonjones.org/assets/pdfs/levity-polymorphism.pdf),
  [Kinds Are Calling Conventions](https://dl.acm.org/doi/10.1145/3408986),
  OxCaml layouts). Enumerated layouts (section 4) were chosen because they
  are finite by construction and give the user explicit control.
- **Data families** (per-instance representation as a distinct type) were
  rejected as the primary mechanism because generic code then needs a class
  to reach the specialized type; representation instances keep the type the
  same (section 6).
- **Rust specialization** was studied as a cautionary tale: full
  specialization is unsound because it dispatches on erased lifetimes; only
  `min_specialization` ("always applicable" impls) is used in `std`. Hence
  the "types only, never multiplicity/region/effects/proofs" rule.

### 10.3 Current state of the code (file:line at 2026-10-29)

- `Core` is untyped: `compiler/src/core.rs:1621` (`pub enum Core`),
  patterns at `core.rs:1567` (`CPat`), units at `core.rs:2687` (`FnDef`).
  The checker's `lower()` at `compiler/src/check.rs:793` drops every type;
  representation classes must be attached before or during `lower()`.
- Every value is one `i64` word; constructor blocks are `(1+n)*8` bytes with
  the tag in word 0 (`codegen.rs` `ctor_body`); nullary constructors and
  static closures are shared constants.
- Allocation: all codegen allocation goes through `cinch_alloc_raw`
  (`compiler/runtime/src/abi.rs:271`, 33 mentions in `codegen.rs`), counted
  by the `#[global_allocator]` in `compiler/runtime/src/heap.rs`.
  `cinch_free` is still a no-op (`abi.rs:298`): nothing is reclaimed yet.
- Byte copies and comparisons are runtime primitives as of 2026-10-29
  (`BytesOp::Copy`/`BytesOp::Compare` in `core.rs`, `cinch_bytes_copy`
  `abi.rs:572`, `cinch_bytes_compare` `abi.rs:595`): copying a byte in Cinch
  boxed a `Nat` offset per byte before this.
- Programs run on a 1 GiB reserved stack (`cinch_run_main`,
  `compiler/runtime/src/platform.rs:555`, override `CINCH_STACK_SIZE`), and
  mixed-arity mutual tail calls are `musttail` through widened workers
  `@"<unit>#tail"` (`core.rs:2977` `tail_widths`).
- Class methods may have defaults (D388), which representation instances'
  fast paths will interact with (a `specialize` body is per instantiation,
  a default is per class).

### 10.4 Measured evidence (2026-10-29, `CINCH_ALLOC_STATS=1`)

Allocation attribution of a parse run over 100 lines of `Data/List.cinch`
(StackAndAlloc's profiler, before the byte-primitive fixes):

| Kind of block | Share |
| --- | --- |
| `Nat` literal boxes (`cinch_nat_from_i64` per literal evaluation) | 41.4% |
| `Nat` arithmetic results | 26.8% |
| `bytes-get` results | 11.4% |
| `bytes-len` results | 7.7% |
| instance dictionaries rebuilt per call (`Add_Nat.mk`, `Eq_Nat.mk`, ...) | 8.8% |
| `bytes-new` | 2.3% |
| all real data (tokens, spans, AST) | about 1.6% |

After `bytesCopy`/`bytesCompare` and length-bucketed keyword tests: lexing
fell from 6,809 to 502 blocks per token; layout from 215 to 6; parse from 411
to 113. Remaining big wins, independent of the representation decision:
small `Nat`/`Int` immediates (reclamation S2) and constant dictionaries for
instances with no sub-dictionaries. Other measured points: `Run.Maps` hits
2 GiB at 93 million blocks (about 23 bytes each); one canonical print of the
475 KB `Data.Machine.cinchif` needs 7.4 GB and `Prelude.cinchif` 2.66 GB;
parsing `Data/Machine.cinch` exceeds 2 GiB (69 M blocks at a 1500 M cap).

### 10.5 Policies in force while nothing is reclaimed

- `CINCH_HEAP_LIMIT` caps total allocation (default 8 GiB; exit 73 with
  `cinch: heap limit exceeded (<n> bytes)`); every program a `cargo test`
  spawns inherits `CINCH_HEAP_LIMIT=2G` from `compiler/.cargo/config.toml`.
- A test that legitimately needs more than the cap is NOT given a larger
  cap: the input that exceeds it is named in a `BLOCKED_ON_RECLAMATION`
  const with its measured `cinch-alloc` line, the main test asserts it still
  stops at the cap (so one that starts passing says so), and an `#[ignore]`
  test runs it uncapped. Instances: `compiler/tests/bootstrap_manifest.rs`
  (`Data.Machine.cinchif`, `Prelude.cinchif`), `compiler/tests/bootstrap_parse.rs`
  (`Data.Machine`), `compiler/tests/lib_hashmap_real.rs` (both tests,
  ignored with the measured line). Each becomes one `--ignored` run away from
  green when reclamation lands.
- Never restructure program code to allocate less as a workaround for the
  leak: the fix is reclamation, and code bent around a leak is dead weight
  once it lands. Quadratic algorithms are a different matter and are fixed
  (right-nested `string-concat`, per-byte copy loops, insertion sorts).
- The user kills any process named `Main` that balloons; run compiled
  programs only on bounded inputs with a cap in their environment.

### 10.6 Gotchas found on the way

- A `.cinch` file beside the entry joins the project, so test corpora must
  be written outside the project (or after the build): copies named like
  prelude modules shadow the embedded family (D383).
- `Path::join` with an absolute path replaces the base: a test once wrote
  its corpus copies back into `examples/source/`.
- The incremental linker splits emitted IR by unit symbol; a new symbol
  shape (`@"<unit>#tail"`) must be owned by its unit in `split_ir`
  (`compiler/src/incremental.rs`), or reassembly drops it.
