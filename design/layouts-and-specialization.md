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

- **Default layout: compiler-chosen (user decision, 2026-10-29).** A type
  without a `repr` attribute gets the layout the compiler picks: it may
  reorder fields, and it may unpack small fixed-layout fields inline, as
  GHC's `-funbox-small-strict-fields` does. The baseline it starts from is
  one word per field, with the three classes of section 2 inside each word.
  The chosen layout is recorded in the interface, so consumers compile
  against it without seeing the definition. The algorithm is deterministic
  and versioned with `runtime_abi`, so two packages built by the same
  compiler agree on it. A layout change is an interface change, and the
  store recompiles the dependents. A type whose layout must not move (FFI,
  on-disk formats, hot-replacement boundaries) says so with `repr(C)` or
  another explicit `repr`; the compiler never changes an explicit layout.
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

## 4 Beyond one word: demand-driven layouts, boxed at expanding edges

Decided by the user (2026-10-29): multi-word layouts are specialized on
demand like the classes of section 2, and the compiler boxes at expanding
edges. No layout kinds or enumerations are needed: the final, whole-program
specialization always knows each instantiation's concrete layout.

**The termination problem.** Specializing on full layouts (Rust-style)
diverges on layout-growing polymorphic recursion: `data Nest a = Leaf a |
Node (Nest (a, a))` with an unboxed pair has layouts `a, (a,a),
((a,a),(a,a)), ...` that never repeat.

**It is detectable, in linear time.** Build the instantiation graph. Its
nodes are pairs (generic definition D, type parameter `a`). For every use of
a generic E inside D at arguments τ1..τm, and every parameter `b_j` of E, add
an edge from (D,`a`) to (E,`b_j`): a **0-edge** if τj is exactly `a`, a
**1-edge** if `a` occurs strictly inside τj. A cycle containing a 1-edge is
an **expanding cycle**.

Claim: specialization from `main` reaches finitely many instances iff no
reachable cycle contains a 1-edge.

- If: every 1-edge then lies between strongly connected components, so any
  instantiation path crosses at most N of them (N components). A 0-edge
  preserves a type argument's depth, and a 1-edge adds at most c (the
  deepest type expression in the program). Roots are written in the
  program, so every reached argument has depth at most d0 + N*c. Finitely
  many types of bounded depth exist over the program's finite set of type
  constructors, so the set of instances is finite.
- Only if: one trip around an expanding cycle maps `a` to a type strictly
  containing `a`; k trips give depth at least k, so the instances are
  infinite, and a static specializer follows every edge.

The check is Tarjan's strongly connected components plus one test per
1-edge. It has the same shape as Kennedy and Pierce's "expansive" criterion
for generic inheritance (2007). Rust specializes per type and finds the
problem dynamically, by its recursion limit.

**Only inline wrappers grow layouts.** A 1-edge whose wrapping type is a
pointer (any boxed constructor) leaves the layout at one `Obj` word, which
is why section 2's classes need no check at all. The layout check therefore
considers only 1-edges whose wrapper is laid out inline (a value type, or an
unpacked field).

**The rule.** At each expanding edge whose wrapper is inline, the compiler
boxes the wrapper at that edge: the value is stored behind a pointer there,
one allocation per level, which is what GHC does for nested datatypes. The
cycle then passes through a pointer and its layouts stop growing. The
compiler warns, naming the cycle; an attribute turns the warning into an
error for code that must never allocate a box (`#[no-alloc]` code gets the
error by default). Acyclic chains can still multiply instances, as C++
templates can; they are finite, and only reached instances are built.

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

implicit instance Bits :: Represent (List Bool) BitList where
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

- Under the implicit representation, `List Bool` is still the type
  `List Bool`; there is no wrapper type and no class constraint (the
  difference from data families). A named representation indexes the type
  (section 6.1) but stays transparent at the source.
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
  - coherence: exactly as for instances, with `implicit` marking the
    coherent default and named representations chosen in context (section
    6.1);
  - a representation is chosen by types only, never by multiplicity, region,
    effects or proofs (Rust's specialization is [known to be unsound](https://std-dev-guide.rust-lang.org/policy/specialization.html)
    because it chose implementations by erased lifetimes);
  - the representation is part of the interface; consumers compile their
    generic code against it.
- **Cost honesty**: semantics are guaranteed, asymptotics are not. `Cons` on
  a packed array is a copy, so a generic `map` at that instantiation can go
  quadratic without a `specialize map`. The manifest records which functions
  ran through the view and which took a fast path.

### 6.1 Implicit and named representations, chosen in context

User direction (2026-10-29): representations use the same `implicit`
keyword as instances, and code can choose a representation in context.

```
implicit instance Bits :: Represent (List Bool) BitList where ...  -- the coherent default
instance Dense :: Represent (List Bool) ByteList where ...         -- named, chosen with via
```

A representation is an instance of the compiler-known class
`Represent t c` (source type `t`, carrier `c`), whose methods are one
builder per constructor of `t` plus `view`, and whose laws are the
isomorphism of section 7. Making it an instance keeps the declaration form
uniform (`Name :: Head`, where `::` means "is evidence of", as in
`implicit instance Show_Nat :: Show Nat`) and makes coherence and the orphan
rule the instance machinery itself, not a copy of it. No `represent`
keyword exists.

- **Coherence is instance coherence.** An `implicit instance` of
  `Represent` is eligible
  for implicit resolution. Two implicit representations whose heads unify
  reject as `ambiguous representation`, under the same orphan rule as
  instances (D119). Named representations coexist freely, because every use
  names one. With no implicit representation, a type has its ordinary
  (compiler-chosen, section 3) layout.
- **The one difference from instances.** An instance never changes how a
  value is stored, but a representation does. A `List Bool` built as
  `BitList` cannot be read by code compiled against `ByteList`, so the
  representation a value was built with must be visible wherever the value
  flows. A representation therefore indexes its type: `List Bool` (the
  implicit representation) and `List Bool via Dense` are
  different types for unification. They are transparent at the source:
  constructors, patterns and every generic function work on both through
  the views.
- **Choosing in context**, three ways:
  - a type annotation: `xs :: List Bool via Dense`. Inference propagates it,
    so `[True, False]` checked against that type builds a `ByteList`;
  - a scoped default, `with (List Bool via Dense) in e`, mirroring
    `with (io = ...) in`: inside `e`, `List Bool` constructions with no other
    constraint infer `via Dense`;
  - a function that requires one: `popcount :: List Bool via Bits -> Nat`,
    which then takes a fast path without the view.
- **Crossing representations is explicit.** A value of `List Bool via Dense`
  passed where `List Bool` is expected is a type error naming both
  representations, not a hidden O(n) conversion (the core language's
  "potentially expensive copies SHOULD be explicit"). A generated
  `rerepresent` converts, through the view and the target's builders. A value
  escaping a `with` scope keeps its `via` type, so the boundary is visible
  where it happens.
- **Generic code never names representations.** In `length :: forall a.
  List a -> Nat`, the representation index is a hidden parameter of the
  `List` application. It is resolved at each instantiation and becomes part
  of the specialization key (class vector, layout, representation), so the
  collector builds one copy per reached representation. `specialize` fast
  paths are keyed by (function, instantiation, representation).

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
4. Demand-driven multi-word layouts with expanding-edge boxing (section 4).
5. `specialize` bodies as instantiation-keyed rewrites (section 5).
6. Representation instances (section 6), on D55 and the law layer.

## 9 Open decisions

- Decided (2026-10-29): option C (D392), and compiler-chosen default
  layouts (section 3).
- Decided (2026-10-29): multi-word layouts are demand-driven, boxed at
  expanding edges (section 4); representations follow instance coherence
  with `implicit`, and are chosen in context (section 6.1).
- Decided (2026-10-29): representations are instances of `Represent t c`
  (`implicit instance Bits :: Represent (List Bool) BitList`); selection is
  `via` in types and `with (List Bool via Dense) in` for a scoped default.
- Spelling of value types: a `repr` attribute (proposed), a declaration form
  (`value data`), or OxCaml's `#` syntax.
- Spelling of specialized
  bodies (a `prim`-style declaration form proposed, since it carries a body
  and a trust tier).
- Representation instances: `Copy`-only element types first (proposed).
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
- **Naive full-layout specialization** (Rust-style) diverges on
  `data Nest a = Leaf a | Node (Nest (a, a))`, whose unpacked layouts
  `a, (a,a), ((a,a),(a,a)), ...` never repeat. Three alternatives were
  considered: run-time specialization (the .NET CLR's answer: only reached
  layouts are ever built; Cinch has a JIT specification), a Swift-style
  generic fallback past a budget, and layouts in kinds with the kind checker
  refusing layout-growing recursion (GHC's [levity polymorphism](https://simon.peytonjones.org/assets/pdfs/levity-polymorphism.pdf),
  [Kinds Are Calling Conventions](https://dl.acm.org/doi/10.1145/3408986),
  OxCaml layouts). Enumerated layouts were proposed first, being finite by
  construction. They were dropped once the user asked for proof that the
  compiler cannot detect layout-growing recursion: it can, in linear time
  (section 4), so the compiler boxes at the expanding edge instead of making
  every definition list its layouts.
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

- `CINCH_HEAP_LIMIT` caps the heap (default 8 GiB; exit 73 with
  `cinch: exhausted: heap (<n> bytes)`). While nothing was reclaimed it
  capped total allocation; since reclamation landed (2026-10-30) it caps
  live bytes (`design/no-faults.md` section 4). Every program a `cargo test`
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

## Part II: lazy layout specialisation (2026-11-13)

Status: design direction, not implemented. Sections 11 to 16 answer a
question Part I left open. Part I establishes that a polymorphic value is
one word, that a word has one of three reclamation classes, and that
multi-word layouts are specialised on demand and boxed at expanding edges.
It does not say what a *slot width* is, what such a layout is *called*, or
who decides the width and when. This part does, because the user's goal is
memory on 32-bit targets, and a polymorphic `List a` stuck at an 8-byte
slot cannot deliver it.

Read with `design/narrow-value-representation.md`, which is the companion
and the boundary half of the same change. Section 11.0 states exactly what
this part assumes from that one.

### 11 The problem, measured

#### 11.0 What this part assumes from the narrow-value note

Assumed, not restated:

- A function boundary already crosses a value at a narrower width where the
  type proves one (`ParamContract::narrow`, `Crossing::Bits`/`Csize`,
  `compiler/tests/narrow_boundary.rs`). That is a *boundary* change: the
  body is unchanged, the entry widens and every return narrows.
- The uniform word stays the default. A 64-bit scalar stays 8 unboxed
  bytes; a heap slot stays 8 bytes (`narrow-value-representation.md`,
  "The boundary slice, taken").
- The immediate encoding follows `csize`: a `Nat` payload is
  `0..=2^(csize-1)-1`, an `Int` payload `-2^(csize-2)..=2^(csize-2)-1`.
- A layout change is an interface change, gated on `runtime_abi`.

New in this part: the *slot*. A boundary position can be narrow while
every heap slot is still 8 bytes, and that is the state of the tree today.
Narrowing the boundary does not shrink a block, so it does not address the
memory goal; a specialised slot does.

#### 11.1 How these numbers were taken

Every count below is observed, not estimated, except where a line says
"arithmetic".

- Block and byte counts: `CINCH_ALLOC_STATS=1 CINCH_ALLOC_KINDS=1`, read
  from the `cinch-alloc:` line (`blocks`, `bytes`, `peak`, `header-bytes`)
  and the `cinch-alloc-kind:` / `cinch-alloc-size:` censuses
  (`compiler/runtime/src/heap.rs`, `alloc_kinds.rs`). The size classes are
  the small allocator's own (`compiler/runtime/src/small.rs`): whole
  multiples of 8 bytes up to 256.
- Emitted layout: `cinch emit-ir` on the same source, reading the
  `cinch_alloc_ctor(i64 <payload>, i64 <descriptor>)` call and the
  `store i64` offsets that fill the block.
- The compiler bundle's own statistics: `build/bootstrap/stage3/compiler.ll`
  (43,959 lines), parsed for the payload-size histogram of every
  `cinch_alloc_ctor` call and for the decoded `@cinch.typed.layouts` table.
- Target comparison: `--target wasm32-unknown-unknown` for the 32-bit
  profile, whose `target datalayout` is `e-m:e-p:32:32-...` and whose
  `word_size` is 32 (`TargetProfile::wasm32`).
- Programs were run with the seed (`build/bootstrap/stage0/cinch`), which
  reads the prelude from source and needs no store, so a measurement is one
  binary and one source file.

#### 11.2 A cell is 8 header bytes plus 8 bytes per payload word

A constructor block is `8 * (1 + words)` bytes: one header word at offset
-8 and one word per field (`block.rs` `HEADER_BYTES`, `codegen.rs`
`ctor_block_word`). The header is 8 bytes **unconditionally**, including on
a 32-bit target, where it is half of a 16-byte cell.

`List Nat` built by `replicate 1000 7`, seed, native aarch64:

|Measurement|Value|
|---|---|
|`cinch-alloc-kind: kind=ctor`|`blocks=1000 bytes=24000`|
|`cinch-alloc-size: upto=24`|`blocks=1000 bytes=24000`|
|`cinch-alloc` total|`blocks=1008 bytes=24386 peak=24298`|
|Payload per `Cons` (IR)|`cinch_alloc_ctor(i64 16, ...)`, two `store i64`|

So 24 bytes per cell: 8 header, 8 for the `Nat`, 8 for the tail. The `Nat`
slot holds at most `2^63 - 1`; a 1000-element list of small numbers spends
8,000 bytes holding values that fit a byte.

#### 11.3 The same 24 bytes for an element that carries no data at all

`data Bit where Off; On`, a two-constructor nullary type, in a list of
1000: `kind=ctor blocks=1000 bytes=24000`, `upto=24 blocks=1000`. Identical
to `List Nat`. The element word is `Obj` (`repr.rs` `AdtRepr::Obj`), so a
one-bit tag costs a pointer to a shared static block
(`@"Main.On.blk#h"`, `static_ctor_block`).

#### 11.4 Narrow fields cost a word each, so the waste compounds

Measured, all with 1000-element lists, seed, native:

|Element type|Element block|Cell|Total for 1000|
|---|---|---|---|
|`Nat`|none (inline word)|24 B|24,000 B|
|`Bit` (nullary)|shared static|24 B|24,000 B|
|`Wrap` = `W U8`|16 B (one word for one byte)|24 B|24,016 B|
|`P8` = `MkP8 U8 U8`|24 B (two words for two bytes)|24 B|48,000 B|
|`P4` = `MkP4 U8 U8 U8 U8`|40 B (four words for four bytes)|24 B|64,000 B|

`MkP8`'s IR is `cinch_alloc_ctor(i64 16, ...)` with two `store i64`: 16
payload bytes for 2 bytes of information. `MkP4` is `i64 32`, four stores.

#### 11.5 On the real compiler, 62% of constructor sites are two words

`build/bootstrap/stage3/compiler.ll`, payload bytes per
`cinch_alloc_ctor` call, 16,746 call sites:

|Payload|Sites|Share|
|---|---|---|
|16 B (2 words)|10,430|62.3%|
|8 B (1 word)|3,787|22.6%|
|24 B (3 words)|1,487|8.9%|
|32 B (4 words)|663|4.0%|
|40 B and above|379|2.3%|

The two-word block is the common shape of this compiler's own data: a
`Cons` cell, a two-field node, a token with a span. Every one of those
words is an 8-byte slot whatever the field's type says.

#### 11.6 The 32-bit target changes nothing today, and that is the gap

A wasm32 module for the same `List Nat` program emits
`cinch_alloc_ctor(i64 16, i64 897)`, byte-identical to the native module,
and the runtime's `HEADER_BYTES` is 8 on every target. So a cell is 24
bytes on a 32-bit target too, while the pointer in it is 4 bytes and a
`Nat` immediate needs 31 bits. Measured by emitting both and diffing the
constructor bodies; the module's own `target datalayout` says `p:32:32`.

A real wasm32 module was built and executed to confirm the pipeline, not
just the IR: `emit-ir --target wasm32-unknown-unknown`, clang 21
(`--target=wasm32-unknown-unknown`), `wasm-ld` against
`compiler/runtime/target/wasm32-unknown-unknown/release/libcinchruntime.a`,
then Node. `List Nat` of 1000 elements answers 0. The freestanding module
has no libc and no `atexit`, so its `CINCH_ALLOC_STATS` line never prints;
the per-cell figures above are the same bytes by construction (the payload
is `i64` and the header is 8), which is why they are quoted from the native
census and the IR rather than claimed as a wasm census.

#### 11.7 What a 4-byte slot would buy (arithmetic on the measured sizes)

With a `csize` slot on a 32-bit target (4 bytes for a `Nat`, 4 for the
tail pointer), a `Cons` payload is 8 bytes and the cell 16:

|Case|Today|4-byte slot, 32-bit|Change|
|---|---|---|---|
|`List Nat`, 1000 cells|24,000 B|16,000 B|-33%|
|`List Bit`, 1000 cells|24,000 B|16,000 B|-33%|
|`MkP4` element block|40 B|24 B|-40%|
|`List Nat`, 200,000 cells (measured `blocks=200000 bytes=4800000`)|4,800,000 B|3,200,000 B|-33%|

With a 1-byte slot and byte packing, `MkP4` is 8 + 4 = 12 bytes (-70%) and
a `List U8` cell on 32-bit is 8 + 1 padded to 4 + 4 = 16 bytes (-33%).
Packing is a later stage; these two lines are arithmetic, not measurements.

**On a 64-bit target the same change buys almost nothing for a list**, and
this is the finding that decides the staging. The tail pointer is 8 bytes
and forces 8-byte alignment, so a `Cons` payload of `{U8, List}` is 1 byte
padded to 8 plus 8 = 16 bytes, exactly what it is today. A 64-bit `Nat`
needs its 63-bit immediate, so it cannot narrow at all. The 64-bit win is
confined to a type with several narrow fields in one block (`MkP4`: 40 to
24 bytes, -40%) and to element blocks, never to the list spine. The user's
motivation is therefore correct in a specific way: the memory win is a
32-bit-target win, and on 64-bit it is an element-block win.

### 12 Naming and identity: `List@unboxed_4_bytes` versus `List@boxed_X`

This is the section that decides whether the design is implementable. A
specialised layout has to be *nameable in an interface*, and two modules
compiled independently have to derive the *same name* for the same type at
the same target, or `List Nat` will mean one layout in the producer and
another in the consumer and the runtime will read a pointer map that is not
there.

#### 12.1 What exists, and why it is not enough

Today a block's layout is a **16-bit positional index** into one
program-wide table, not a derived identity.

- `block_kinds.rs` gives the header a 16-bit `layout` field. For a
  constructor block that field is a *descriptor*: `id = group << 6 | tag`,
  so the tag is recoverable as `id & CTOR_TAG_MASK` without knowing the
  scrutinee's type. `LAYOUT_BIG` escapes to a full descriptor in the word
  before the header.
- The index is assigned by `codegen::Layouts::place`/`add_type`, walking
  `p.fns` in a `BTreeMap` order and bumping a counter `next`. **It is a
  position in one program's emission order.** Nothing about it is derived
  from the type.
- Cross-module agreement is by *concatenation*, not by derivation. A
  library object records its table as `layout_words` in its ABI manifest
  (`codegen::object_abi`); a consumer prepends those words and starts its
  own indices past the prefix (`Bootstrap.RuntimeIr.cinch:4436`
  `library-layout-words`: `prefix <> library-layout-tail count ...`). The
  test that guards it asserts `&words[..prefix.len()] == &prefix`
  (`compiler/tests/bootstrap_runtime_objects.rs:151`).
- `Layouts::next` starts at 0 in every module, so the prefix rule is the
  only thing keeping two modules from handing out the same index to
  different maps. It works because a consumer is compiled *after* its
  provider and sees the provider's whole table.

Two facts follow, and they are the reason this section has to exist:

1. **The index is not an identity.** Two builds of the same module can
   assign different indices to the same type if the type's emission order
   changes, which is exactly why `compilation-speed.md` section 2.9 records
   that IR comparison has to normalise "the representation-class index that
   `cinch_alloc_shaped` takes".
2. **A lazily created layout has no position.** The whole point of lazy
   specialisation is that the module that *uses* `List Nat` may be compiled
   before, or without, the module that *defines* `List`. A positional index
   assigned at emission cannot be agreed on by two compilations that do not
   share an emission order.

So the name has to be derived from the type and the target, and the index
has to be a *lookup* of that name, not a counter.

#### 12.2 The name: a canonical descriptor, spelled in the suffix

A specialised layout's identity is a **canonical descriptor string**, and
the symbol suffix is that descriptor in a fixed grammar. It is not a hash.
A hash cannot be read in a diagnostic, cannot be diffed, and (the deciding
argument) cannot be checked: a consumer that receives a hash either trusts
it or re-derives it, and re-deriving it is the same work as comparing the
descriptor, so the hash buys nothing but an inability to explain a
mismatch.

The grammar, over the target profile `P` (word size `w = P.word_size / 8`
bytes, `csize`), and a type expression `T`:

```
layout-name  := "unboxed_" width "_bytes"      slot is inline, this many bytes
              | "boxed_" box                    slot is one counted pointer
              | "inline_" width "_bytes"        stored in the parent, no slot of its own
box          := "obj"                          a counted pointer, always w bytes
              | "imm" width                    an immediate, this many bytes
              | layout-name                    a nested layout, boxed by name
```

Worked examples, both derived by the rules of 12.3:

|Element type|64-bit native|wasm32|
|---|---|---|
|`Nat`|`List@unboxed_8_bytes`|`List@unboxed_4_bytes`|
|`Int`|`List@unboxed_8_bytes`|`List@unboxed_4_bytes`|
|`U8`|`List@unboxed_1_bytes`|`List@unboxed_1_bytes`|
|`U32`|`List@unboxed_4_bytes`|`List@unboxed_4_bytes`|
|`Bit` (two nullary constructors)|`List@unboxed_1_bytes`|`List@unboxed_1_bytes`|
|`List Nat` (the element is a pointer)|`List@boxed_obj`|`List@boxed_obj`|
|`Bytes`, `String`, a closure|`List@boxed_obj`|`List@boxed_obj`|
|a type variable `a`|`List@boxed_obj`|`List@boxed_obj`|

`Option U8` is the one row that is a design decision rather than a
consequence of 12.3: a tagged multi-constructor type needs a tag, and the
note's answer is that it stays `boxed_obj` until the tag can be packed into
the narrow slot without a second representation of the same value. Packing
it is stage 4 of section 16, gated on the measurement there.

Three properties of this spelling matter:

- **The target is inside the name**, so `List@unboxed_8_bytes` and
  `List@unboxed_4_bytes` are different units that never collide, and a
  cross-compile cannot pick up the host's answer. This is the same
  discipline `TargetProfile` already enforces: the profile comes from the
  triple, never from the build host.
- **The suffix is self-delimiting and unambiguous against today's class
  letters.** A class vector suffix is `@` then `R`/`O`/`T` only
  (`repr::letters`, `repr::base_unit` strips exactly that). A layout suffix
  begins `@unboxed_`, `@boxed_` or `@inline_`, none of which starts with a
  class letter, so `base_unit` can strip a layout segment with a second
  case and existing symbols keep their meaning. There is no `~` in the
  grammar because `~` is not a safe symbol character in LLVM IR.
- **It composes with the class vector.** A polymorphic unit specialised at
  a class vector *and* a layout is `Data.List.map@RO@unboxed_4_bytes`:
  classes first (the reclamation axis), layout second (the width axis). Two
  suffixes, two independent axes, one symbol.

#### 12.3 How the descriptor is computed

The function is total on the type and the profile, and it is the *only*
place a slot width is decided. This is a hard requirement, not a style
note; see 12.6.

```
slot(T, P) =
  | Ty::Nat                      -> unboxed_{csize}_bytes          -- 12.3.1
  | Ty::Int                      -> unboxed_{csize}_bytes
  | Ty::Prim(U8|I8)              -> unboxed_1_bytes
  | Ty::Prim(U16|I16)            -> unboxed_2_bytes
  | Ty::Prim(U32|I32|F32)        -> unboxed_4_bytes
  | Ty::Prim(U64|I64|F64)        -> unboxed_8_bytes
  | Ty::Prim(Usize|Isize)        -> unboxed_{csize}_bytes
  | Ty::Bool                     -> unboxed_1_bytes                -- 12.3.2
  | Ty::Char                     -> unboxed_4_bytes
  | Ty::Unit                     -> unboxed_0_bytes
  | Ty::Bytes | Ty::Fun | closure| unboxed_8_bytes   (a pointer; `boxed_obj`)
  | Ty::Adt(d, args)             -> layout-of d at args            -- 12.3.3
  | Ty::Param(_)                 -> boxed_obj                      -- 12.3.4
```

The rules, each of which is a decision the note has to defend:

**12.3.1 `Nat` and `Int` are `csize` bytes, never their payload width.**
A `Nat` immediate is `0..=2^(csize-1)-1` because the low bit is the tag and
the encoded word must fit the pointer-sized slot
(`narrow-value-representation.md`, "the immediate payload follows `csize`").
On 64-bit that is 63 bits, so the slot is 8 bytes and a `Nat` element does
not narrow; on 32-bit it is 31 bits, so the slot is 4 bytes and it halves.
The rule is not "narrow to the payload" but "the slot is the word the
immediate encoding was defined over", because the encoding and the width
are one change (the same note, "the word width and the encoding are one
change"). A 5-byte slot holding a 31-bit payload would need a 3-bit tag and
would break the `csize` crossing, for one byte.

**12.3.2 `Bool` and a nullary-constructor type are 1 byte, and this is a
semantic change, not an optimisation.** Today `Bool` is `Raw` and one word,
and a nullary constructor is a pointer to a shared static block. Storing
`Bool` as one byte and `Bit` as one byte makes the *value* no longer a
`Raw` word, so a polymorphic position that held a `Bool` cannot also hold
an `I64`. That is fine, because the layout suffix is on the *type*, not on
the word: `List Bool` and `List I64` are different units. It is not fine if
anything relies on the one-word encoding. The reclamation pointer map has
to grow a `SCAN_BYTE`-like code or a width byte; see 14.4.

**12.3.3 A data type's own slot is `boxed_obj`; the suffix names its
element slot.** A `List a` is a pointer whatever `a` is, so `List Nat` in a
field of another type is `boxed_obj`. The suffix on `List@unboxed_4_bytes`
names the *element* slot of that `List`, which is what the block layout is
computed from: `Cons`'s field 0 has the element's layout, field 1 is
`boxed_obj`. This distinction has to be explicit or the grammar means two
things at once, and it is why the examples in 12.2 are a column of *element
types*.

`List (List Nat)` is `List@boxed_obj` because its element is a `List`,
which is a pointer whatever that inner list's elements are. That is the same
termination argument Part I section 2 makes for classes: a
constructor-applied type is `Obj` whatever its arguments, so the recursion
bottoms out and a layout request never diverges.

**12.3.4 A type variable's slot is a placeholder in the generic body, and
`boxed_obj` in the layout vector.** The collector already copies a
polymorphic body once per reached class vector, and a copy carries concrete
classes (`Class::Param` is substituted by `repr::subst_all`). The layout
axis is a second vector of the same shape: a copy is made per reached
*(class vector, layout vector)* pair, and the layout vector's entries are
layout names. The generic body carries a layout placeholder and the
substitution is total, so a body never contains an unresolved slot width at
emission.

This is not monomorphisation, for the reason Part I section 2 gives for
classes: `List a` is `boxed_obj` whatever `a` is, so a layout vector never
grows with the type and the set of reached pairs is finite. The cost is one
copy per reached pair, which is the cost the class axis already pays.

#### 12.4 The two modules that disagree

The question the brief asks directly: one module asks for
`List@unboxed_4_bytes`, another for `List@boxed_8` for the same source type.

`List@boxed_8` is not in the grammar, and that is deliberate: a boxed slot
is `boxed_obj` (a pointer, `w` bytes), never `boxed_8`, because a boxed
slot's width is the target's pointer width and writing `8` into it would
let a 64-bit name be produced on a 32-bit target. The grammar has exactly
one spelling per (layout, target) pair, and it is total, so **the same
source type at the same target has exactly one name**. Two modules cannot
disagree by asking for different names; they can only disagree by having
been compiled for different targets, and that is refused at admission
(`profile.triple` is in the object ABI and the store key).

The residual case is a module that asks for `List@unboxed_4_bytes` while
its provider only has `List@boxed_obj` because the provider was compiled
before the layout existed. That is not a naming disagreement, it is a
*staleness* fact, and the answer is the same one the store already gives:
the layout set a module was compiled against is part of its interface
identity, so a consumer demanding a layout the provider does not publish is
`checked external variant is absent` (the message
`bootstrap_runtime_objects.rs` already asserts), not a silent mismatch.

#### 12.5 What the name is *for*: index lookup, not index arithmetic

The name is the identity; the 16-bit index is a per-process cache of it.
The mechanism:

- A `.cinchif` carries the module's **layout rows**: for each type it
  defines, the name and the descriptor words (size, alignment, pointer
  map). It does not carry an index.
- At link, the program collects the distinct names from every interface
  plus every name its own code requests, sorts them, and assigns indices
  0..n. That assignment is deterministic across builds of the same set and
  is what the layout table is emitted from.
- A block's header keeps the 16-bit index, so the runtime is unchanged: it
  reads `layout(h)` and looks up the table, exactly as today.
- The index is therefore an *implementation detail of one process*, and the
  name is the *interface identity*. Two modules agree because they agree on
  the name; they need not agree on the index, because the linker assigns
  it once for the process.

This keeps the property `block_kinds.rs` relies on (a tag readable without
the scrutinee's type) because the index keeps the `group << 6 | tag` shape.
It drops the property that the index is the emission order, which is what
makes lazy creation possible at all.

#### 12.6 Where the width is decided, once

The narrow-value work already in the tree has a live miscompile from
exactly this failure mode: a narrow comparison conjunct is lost when the
guard is itself a conjunction of narrow comparisons (lane `dasl-narrow-cond`,
`1395f02a`), and the seed returns the wrong answer. The cause is
structural: the width of a value is decided in more than one place
(`check_decls.rs` `crossing`, `codegen.rs` `Cross::of`, the apply stub, the
`musttail` prototype comparison) and the places can disagree.

The requirement this note imposes on the slot design is the same one, stated
so it cannot be missed:

> **A value's slot width is decided by one total function of (type,
> target), and every consumer reads that function's answer rather than
> recomputing it.**

Concretely:

- `slot(T, P)` (12.3) is the function. It is total: no `Option`, no
  fallback, no "the analysis could not prove one". A type whose width
  cannot be stated is `boxed_obj`, which is a *decision*, not a refusal.
- The boundary crossing reads the same function. `Crossing::Bits { bits,
  signed }` and `Crossing::Csize` become *views* of `slot`: the crossing of
  a `U8` position is `slot(U8, P)` when that is narrower than the word, and
  the slot of a `List U8` element is the same `1`. One answer, two uses.
- A crossing that would disagree with the slot is made impossible by
  construction: the emitter takes the width from the slot record on the
  binding, and there is no second code path that computes a width from a
  `Ty`. If a future change needs a second path, the type system should
  carry the width (a `Width` newtype on the binding) rather than the code
  recomputing it.

That is the design rule the miscompile argues for, and it is why 12.3 is a
total function rather than a proof obligation.

#### 12.7 Why a descriptor and not a hash

Three schemes were considered for the identity and one was taken.

|Scheme|Identity is|Rejected because|
|---|---|---|
|positional index (today)|emission order|not derivable by a second compiler, and `compilation-speed.md` 2.9 already has to normalise it away when comparing IR|
|hash of the descriptor|`fnv1a64(descriptor)`|cannot be read in a diagnostic, cannot be diffed when two modules disagree, and a consumer must either trust it or re-derive it (the same work as comparing the descriptor)|
|canonical descriptor string|the string|taken|

The decisive property is *explainability under disagreement*. When two
modules disagree, the compiler must say which two layouts differ and how,
and a string does that (`the provider publishes List@boxed_obj; this
module needs List@unboxed_4_bytes for a Nat element`). A hash says two
numbers are different. The existing code has this lesson already: the
layout mismatch message in `bootstrap_runtime_objects.rs` names the class
vector (`" [T]"`), not an index, and the note that documents the object ABI
says the manifest is a *contract* a consumer reads.

The one place a hash is right is the interface's own `iface_hash`, which is
`fnv1a64` over the canonical contract projection (`interface.rs:1114`). It
is a cache identity, explicitly "not a cryptographic artifact digest", and
it is allowed to be opaque because nothing ever explains a hash mismatch;
it recompiles. A layout name is different: it is part of the *contract* and
appears in symbols a human reads.

#### 12.8 How this fits the existing identity machinery

|Existing mechanism|What it does|What the layout scheme does with it|
|---|---|---|
|static block descriptor (a symbol *is* its descriptor: `static_payload`, `@"<sym>.blk#h"`)|makes a static block's payload reachable by a constant offset, never an alias|unchanged: a specialised layout changes the payload's *size and map*, not how it is addressed. The dedup rule still holds, and a narrow block is still `getelementptr` at `8 * words`|
|`repr::spec_name` / `base_unit` (`@` plus class letters)|names a specialised *unit* and recovers the generic one|extended with a second segment, `@unboxed_4_bytes`, after the class segment|
|`iface-schema-version` (`SCHEMA_VERSION = 10`)|gates interface shape|bumped when layout rows are added; a 10 reader cannot read an 11 interface and says so|
|`runtime_abi` (`RUNTIME_ABI_VERSION = 9`)|gates the layout algorithm and the runtime's ABI|bumped, because a 16-bit index into a table of a different shape, and a new pointer-map code, are runtime ABI|
|`Span` interned handle (`sym.rs`)|identity by pointer for a name|not applicable: a layout is not a value, and interning it would make identity depend on process state|
|`object_abi`'s `layout_words` prefix|concatenation agreement across objects|replaced for *new* layouts by name-keyed collection at link; the prefix rule stays as the mechanism for a provider's existing rows|

### 13 Where specialisation happens, lazily

#### 13.1 The pipeline today, and the two places a layout is decided

Reading `specialize.rs`, `core.rs::optimize` and `codegen.rs`, the order is:

```
check + lower      classes attached to binders and fields (repr_of)
core::optimize     simplifier, inliner, dead-unit elimination
specialize         collector: one copy per reached class vector,
                   constructors copied per vector, names minted by spec_name
rc::insert         dup/drop from the class vectors
codegen            Registry::build -> Layouts::lay_out_constructors,
                   which assigns the 16-bit indices
```

Two facts about that order decide the whole design:

1. **The collector runs before codegen and has the type information.**
   `specialize_with_roots` walks a worklist of `(unit, class vector)` pairs
   starting from roots and copies bodies, substituting classes. A layout
   vector is the same walk with a second component, and it must happen
   *there*, not in codegen, because codegen has already lost the types
   (`check.rs::lower` drops them; `UnitRepr` keeps classes but not layouts).
2. **Codegen assigns indices after every copy exists.** `Layouts::place`
   walks `p.fns` in `BTreeMap` order and bumps `next`. So by the time an
   index is assigned, the set of blocks is closed, and a layout request
   that arrives during emission cannot be honoured. That is the ordering
   constraint the brief asks about.

#### 13.2 What triggers a request, and what must be ordered

A specialised layout is requested when the collector reaches an
instantiation whose element slot is not `boxed_obj`. Concretely, the
request set is built by the same drain loop that builds the copy set:

```
request(D, v) where v is a layout vector:
   for each constructor unit C of D at v:
     for each field f of C:
        slot(f.ty at v) -> a layout name L
        request_layout(L)          -- idempotent, memoised by name
```

The ordering fact is subtler than "collect first", and the code says so.
`Registry::shape_desc` **is** called during emission, from `ctor_body`,
from the tuple and prelude-constructor helpers, from primitive results and
from the canonical-ABI lift (`codegen.rs:2269`, `2280`, `4343`, `5399`,
`6441`, `6900`). So a layout can be created after the unit that requested
it has been emitted, and that is fine, because:

> **The layout *table* is emitted after every unit body, so a request that
> arrives during emission is honoured. What must be decided before
> emission is only what an emitted body has already read.**

`Layouts::finish` is what emits `@__cinch_layouts`, and it runs after
`Registry::build` and after every body (`emit_module` builds the table from
`layouts.words()` at the end). A body reads a *descriptor word* at emission
time, so the descriptor for its own block must be stable when it is
emitted; the table entry for that descriptor may be written later.

There is exactly one decision that a late request can invalidate, and the
code already refuses it: whether **any** block may carry an escaped
descriptor, which changes every emitted tag read from `id & mask` to a
branch. `Layouts.escaped` is fixed before emission (`lay_out_constructors`
sets it for a canonical-ABI program), and a shape that escapes after that
sets `late_escape`, which `finish` turns into the refusal
`"a constructor shape escaped after the tag reads were emitted without
escapes"`. That is the ordering rule stated as code: *the escape decision
is pre-emission; everything else is not.*

For a specialised layout the same rule applies with one addition. A
specialised slot changes a block's *size*, and size is read at emission by
`ctor_alloc_call` (`cinch_alloc_ctor(i64 <bytes>, ...)`). So the size must
be decided before the constructor unit that builds the block is emitted,
which the collector guarantees because constructor units are in its
worklist. A layout requested later than that is a request for an
*anonymous shape* (a tuple, a primitive result, a lifted aggregate), which
is never specialised: those blocks keep one word per field, which is what
they have today. That is the refusal in 15.2, and it is a consequence of
the ordering rather than a separate policy.

Two further cases, and their answers:

- **A foreign call whose result is a specialised block.** The foreign
  plan's `CtorLayout` (`check.rs::LayoutRecipe`, `ForeignPlan.canon`) is
  computed during checking, so the size is known before the collector.
  No late request.
- **A quote or `eval` that builds a block at run time.** The evaluator
  builds blocks through `shape_desc` with the classes it is given, and a
  runtime-built block cannot be specialised, because its layout would have
  to be chosen at run time. It stays `boxed_obj`, the same answer as
  `#[no-alloc]` code: an allocation whose layout is not static is not
  specialised.

#### 13.3 How it is memoised, and what the memo key is

The memo is keyed by the **layout name**, not by the type and not by the
index:

```
layouts: BTreeMap<LayoutName, LayoutRow>
LayoutRow { size: u32, align: u32, map: Vec<SlotClass> }
```

Two properties follow and both are needed:

- **Idempotence across a whole program.** `List Nat` requested from three
  call sites is one row, one name, one index.
- **Determinism across builds.** A `BTreeMap` iterated in name order gives
  the same index assignment for the same set of names, so two builds of the
  same program emit the same table. That is what makes the fixed point
  (`stage2/compiler.ll` byte equal to `stage3/compiler.ll`) hold for a
  change that touches layouts, and it is the property the current counter
  does *not* have: the counter is deterministic for one program but its
  value depends on emission order, so any change that reorders types
  renumbers the table.

The store caches the row by name in the module's interface, so a consumer
reads it rather than recomputing it. That is the same shape as
`Iface.layouts` today (`AbiLayout`), extended from `repr` facts to slot
rows.

#### 13.4 What is not memoised, deliberately

The *decision* that a body is copied at a layout vector is not memoised
across programs, for the reason Part I section 2 gives for classes: the set
of reached vectors is a property of the whole program from `main`, so
caching it would cache a whole-program answer in a per-module artifact. A
library object records the layout rows it *defines* (a fact about the
module) and the ones it *demands* (a fact about its own bodies, which
becomes the `object_roots`-style demand list a consumer satisfies).

#### 13.5 Interaction with the expanding-edge rule of Part I section 4

Part I section 4 says an expanding edge whose wrapper is laid out inline
boxes the wrapper. That rule now has a second job: it is also what keeps
the layout request set finite when layouts are specialised. The argument is
the same one, applied to names rather than classes:

- A 1-edge whose wrapper is `boxed_obj` leaves the name unchanged
  (`List (List a)` is `boxed_obj` whatever `a` is), so it cannot generate a
  new name.
- A 1-edge whose wrapper is inline changes the name, and the expanding-cycle
  check refuses to follow it (it boxes at that edge), so the chain of names
  is finite.

So the finiteness proof Part I section 4 gives for layouts is exactly the
finiteness proof for *names*, which is a stronger result than the section
claimed and worth recording: the same Tarjan pass justifies both.
