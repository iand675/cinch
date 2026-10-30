# Reclamation: precise reference counting (Perceus) for Cinch

Status: proposal for review, 2026-10-29. It closes the open row "Immutable
sharing and reclamation strategy" (`design/open-decisions-and-implementation-plan.md:65`).
Fixed by the user: Cinch stays GC-free (no tracing collector, ever), the
mechanism is Perceus-style precise reference counting (compiler-inserted
dup/drop, borrowed parameters, drop-reuse), and reclamation comes before
new bootstrap work. **Section 2 (the value representation) is the user's
decision.** It gets a recommendation, not a settlement. Everything else is
written so that most of it can land before that decision.

Conventions: `path:line` cites the tree as of this date (`codegen.rs` is under concurrent edit; its citations were re-verified at writing time). `[INFERENCE]` marks
a claim I derived and did not observe. "Measured" numbers come from the
accounting slice (lead report, 2026-10-29, `CINCH_ALLOC_STATS=1`). This
document does not re-run them.

---

## 1 Status quo and measured cost

### 1.1 What leaks and why

- `cinch_free` is a no-op: `compiler/runtime/src/abi.rs:297-298`
  (`pub unsafe extern "C" fn cinch_free(_word: i64) {}`). Its doc
  (`abi.rs:290-296`) and the ABI reference (`compiler/runtime/src/lib.rs:102-104`)
  say blocks are leaked until reclamation lands. No file in
  `compiler/src` calls it.
- Generated code allocates through `cinch_alloc_raw` (`abi.rs:263-288`,
  uninitialised, 8-byte aligned, size rounded to words), declared at
  `compiler/src/codegen.rs:138`. **30 emission sites** call it:
  `codegen.rs:1276` (apply-stub growth), `1731`, `1763`, `1772` (prim
  `Some`/`Err`/`Ok` results), `1962` (UTF-8 decode tuple), `2369`, `2381`,
  `2388`, `2399` (task join results), `3095`, `3108` (`ctor_body`: boundary
  and uniform constructor blocks), `3276`, `3671`, `3758` (FFI aggregate
  boxing), `3834`, `3860` (`mk_body`: dictionary and method closure),
  `4358` (`Core::Lam` closure), `4429` (`BoxAlloc` knot), `4473`
  (`LazyAlloc` memo cell), `6175` (quote block), `6249`, `6319`, `6340`,
  `6404`, `6516`, `6519`, `6566`, `6575`, `6694`, `6717` (wasm canonical-ABI
  glue). Several of these run once per call, so they execute without bound
  at runtime.
- Runtime values come from Rust boxes that are never reclaimed:
  `box_nat`/`box_int`/`box_bytes`/`box_sym` (`abi.rs:66`, `73`, `80`, `89`)
  each do `Box::into_raw(Box::new(Boxed { kind, value }))` (`abi.rs:67`,
  `74`, `81`, `90`). `abi.rs` has 7 `box_nat` call sites and 8 `box_int`
  call sites. `box_bytes` has 22 call sites across `abi.rs`, `eval.rs`,
  `net.rs`, `platform.rs`, `pty.rs`, `sym.rs` and `text.rs`. A nonzero
  `Nat` is two allocations: the `Boxed<Nat>` and its `Vec<u32>` limbs
  (`nat.rs:14`, `nat.rs:52`). A non-empty `Bytes` is also two: the
  `Boxed<Bytes>` and the `Box<[u8]>` data (`bytes.rs:80-93`, `bytes.rs:90`).
  The quote evaluator allocates closures and constructors through
  `cinch_alloc` (`eval.rs:783`, `868`, `887`).
- Every `Nat`/`Int` operation allocates a fresh box, even for small results.
  `cinch_nat_add` (`abi.rs:330`) is `box_nat(as_nat(a).add(as_nat(b)))`.
  Even the literal fast path allocates: `Core::ConstBig` that fits an
  `i64` calls `cinch_nat_from_i64` (`codegen.rs:4203-4206`).
- Some things are already freed exactly once: OS resources and scopes.
  `cinch_buf_free` (`platform.rs:141`), `cinch_fs_dir_close`
  (`platform.rs:346`), `cinch_proc_free` (`platform.rs:734`),
  `cinch_scope_close` (`abi.rs:617`), `coop.rs:481`, and the socket, pty
  and pipe handle tables, which remove their entries on close. Task handle
  words keep one `Arc` reference by design (`coop.rs:204`, `task.rs:133`).

A compiled program never returns memory, so its footprint equals its total
allocation, not its live set. The accounting runtime (`compiler/runtime/src/heap.rs`)
already knows this. Its cap is on cumulative bytes "since nothing is freed
yet" (`heap.rs:29-31`), and a real `dealloc` already decrements the
current-bytes counter, so the free slice "only has to make `cinch_free`
actually deallocate" (`heap.rs:9-13`).

### 1.2 Measured cost

| Program / input | Measured | Source |
|---|---|---|
| canonical printing of `Data.Machine.cinchif` (474,869 B) | 129.6 GB peak, killed | `bootstrap/Bootstrap.Manifest.cinch:28-35` |
| same, `Prelude.cinchif` (193,092 B) | 1.57 GB | `Bootstrap.Manifest.cinch:33` |
| same, `Effect.Process.cinchif` (15,329 B) | 477 MB | `Bootstrap.Manifest.cinch:32` |
| `bootstrap/programs/Run.Maps.cinch` under 2 GiB cap | completes 4000- and 32000-key phases (insert 368 ms, lookup 71 ms at 32000), then exit 73 at `blocks=92960242 bytes=2147483723` | measured |
| lexing | about 15 KB allocated per input byte | measured |
| parsing a 16 KB JSON document | 360 MB peak | measured |

The 129.6 GB row is mostly quadratic `string-concat` (agent LinearText is
fixing it), and reclamation would only turn that quadratic memory into
quadratic time. The `Run.Maps` row is the representation question in
miniature. It is about 93 million blocks at about 23 bytes each, which the
lead attributes to per-operation `Nat` boxing and AVL node copies. The
HashMap spine node is
`SBin :: Nat -> Nat -> List (k, v) -> Spine k v -> Spine k v -> Spine k v`
(`compiler/prelude/Data/HashMap.cinch:61-63`). The height and the digest are
both `Nat`, so every rebuilt node also allocates up to two `Nat` boxes plus
their limb vectors. [INFERENCE] With reclamation alone, that program's peak
would track the live map. Small-`Nat` immediates (section 2.6) remove most of
the block count, and drop-reuse (section 5) removes most of the node copies
when the map is uniquely held.

---

## 2 The representation question (the decision)

### 2.1 The problem

Every `Core` value is one untyped `i64` word (`compiler/src/core.rs:7-18`,
`compiler/runtime/src/lib.rs:13-28`). Machine scalars are raw. Heap values
are `ptrtoint` pointers. No `Core` node carries a type: the enum is
`core.rs:1600-1732`, and `lower()` (`compiler/src/check.rs:793-883`) is a
structural map that drops every type. A drop must know whether a word is a
pointer. Monomorphic code could know this from types. Polymorphic slots
cannot, because they hold either kind depending on instantiation. Those
slots are a `Cons` head, a tuple field, a closure env or saved-argument slot
(`codegen.rs:1269-1288`, `4350-4368`), a dictionary method slot
(`codegen.rs:3834-3865`), and a memo cell or knot value slot
(`codegen.rs:4425-4480`). Polymorphism is uniform-representation dictionary
passing with no monomorphisation. Every instantiation shares one body,
`InstDict`/`DictProj` are ordinary values (`core.rs:1654-1659`), and schemes
are prenex (`types.rs:2002-2017`, no `forall` variant in `Ty`,
`types.rs:1301-1399`), so every instantiation happens at a call site the
checker sees.

The runtime's own boxes put a first-word `kind` (`abi.rs:33-49`), but that
namespace collides with constructor tags, which also sit in word 0
(`codegen.rs:3109`, `4670-4671`), and with the quote block's
`KIND_CODE = 3` (`eval.rs:153`, the same number as `KIND_BYTES`). So word 0
cannot drive a generic drop either. Rejected outright:

- **Conservative pointer test** (is the word an address of a live block?).
  An `I64` whose value equals a live block's address would be decremented.
  That is unsound, not merely imprecise.
- **Global tagging** (OCaml-style 63-bit integers everywhere). It breaks the
  fixed layout of `I64`/`U64` (`specifications/primitive-types-specification.md:22`),
  `repr(C)` field layouts and SIMD lanes.
- **Always-heap-box scalars in polymorphic positions.** This is option A
  without immediates, and strictly worse than A.
- **Tracing**: excluded by the user.

### 2.2 What every option needs

1. **A representation class per binder.** Call the classes `Raw` (never
   counted: machine scalars, `Bool`, `Unit`, `Ptr` roles, task, scope and
   process handles, regions, erased indices), `Obj` (always a counted
   pointer, possibly immortal), and `TObj` (an immediate with low bit 1, or
   an `Obj`: `Nat`/`Int` once section 2.6 lands, and type-variable slots
   under option A). The source is the checker's binder table. Each
   `Obligation` holds `{name, mult, ty, ...}` (`check.rs:1606-1621`) and
   `check_usage` zonks every binder type (`check.rs:3435`). There is
   precedent: `param_contract` already classifies parameters by type
   (`Ty::Prim(_) | Ty::Bool => noundef`, `check_decls.rs:9653-9679`), and
   codegen does the same for prims (`codegen.rs:1168-1172`). A
   `repr_of(&Ty) -> Repr` belongs beside it.
2. **Where it lives in `Core`.** The driving optimizer runs between `lower()`
   and codegen (`driver.rs:1774` then `1784`; also `1228`/`1229`,
   `1860`/`1861`, `1896`/`1897`) and creates binders (`core.rs:5657`). A side
   table keyed by `LocalId` would go stale there. I recommend putting the
   class on the binding sites: `Let`, `CPat::Con` fields (`core.rs:1547-1553`),
   `FnDef` params (`core.rs:2667-2683`), and `Lam` env. Then every
   optimizer rewrite that moves a binding carries its class.
   [INFERENCE] That is mechanical churn across `core.rs` (6495 lines).
3. **Per-constructor field classes.** `FnDef.ctor` is only `(tag,
   field_count)` (`core.rs:2675-2677`). It needs the field classes from the
   declared field types (`ctor_parts`, used at `check.rs:3701`).
4. **Instantiation-site information** wherever a polymorphic unit is used
   at a known type: each option consumes it differently.
5. **A header word** per heap block (section 3) and a `runtime_abi` bump
   (`compiler/src/interface.rs:390`, `434`), since the representation pairing
   is recorded by the interface's `abi.convention`
   (`design/compiled-interface-metadata.md:120-123`).

Items 1, 2, 3 and 5 are common to all options.

### 2.3 Option A: tagged scalars in polymorphic positions (Lean 4, Koka)

A polymorphic slot is any slot whose declared type is a type variable or
`f a` with `f` a parameter, plus every slot of the uniform closure ABI. Such
a slot holds `TObj`: `n << 1 | 1` for an immediate scalar, or a pointer.
Monomorphic code stays raw.

- **Coercions.** The checker inserts explicit `Core::Tag(Scalar, e)` and
  `Core::Untag(Scalar, e)` (new nodes) where a scalar crosses into or out of
  a polymorphic slot. It knows the instantiated type there, at the same
  instantiation sites where it already inserts `InstDict` arguments
  (`check.rs:158-166`, `dict_expr_ecore`). The optimizer cancels `Untag(Tag x)`.
- **Closure ABI.** Indirect calls (`__apply`, `codegen.rs:1384-1393`) pass
  and return `TObj`. Each unit's `@"U.app"` stub (`codegen.rs:1255-1340`)
  untags scalar parameters and tags a scalar result from the unit's static
  parameter classes. Env and saved-argument slots are stored tagged, so a
  closure's slots from word 3 on are all `TObj` and are scanned using
  `filled` (word 2). This is Lean's model: closure fixed arguments are always
  boxed objects.
- **Range.** `Char`, `Bool`, `Unit`, and integers up to 32 bits always fit.
  `I64`, `U64`, `Isize`, `Usize`, `F64` and `F32` bit patterns that do not
  fit 63 bits take a heap fallback: a `KIND_BOXED_SCALAR` leaf block.
  Untrusted pointers (`Ptr`, opaque foreign handles, the `Box<Proc>` word
  from `platform.rs:565`) must be tagged too. They are aligned or arbitrary
  addresses that would otherwise look like objects. `F64` in a polymorphic
  slot nearly always allocates. Lean has the same rule.
- **Cost.** Dup or drop of a `TObj` is a low-bit test plus the `Obj` path.
  Tag is `shl`/`or`, plus a range check and possible allocation for 64-bit
  types. Untag is `sar`, or a load from the box.
- **Spec conflict.** "A primitive value is unboxed ... involves no
  indirection, allocation" (`primitive-types-specification.md:22`) fails
  for 64-bit scalars and floats in polymorphic slots. The resolution would
  be an amendment: "in monomorphic positions; a scalar in a polymorphic
  position may be tagged or boxed, which is unobservable". A also makes
  `Tag` of a 64-bit scalar an allocation site for `#[no-alloc]`
  (`design/safety-assertion-attributes.md:21`).

### 2.4 Option B: representation witnesses (Swift value-witness style)

Each type variable whose values the body dups, drops, or stores into a
block gets an implicit witness argument. Because the word is uniform, the
witness is one word holding its class (`Raw`/`Obj`/`TObj`), not a table of
functions. The checker already has the pattern: `Copy` is a compiler-known
structural class solved by `copy_structural` with a marker dictionary
lowered to `Unit` (`check.rs:6772-6779`, `3638-3712`, `164`). `Repr a` would be solved the same way
and lowered as an ordinary dictionary argument.

- **Blocks carry dynamic per-field masks.** A `Cons` built in `map` gets its
  head bit from the witness. Drop reads the mask, so block drop needs no
  witness. A bare `a`-typed value needs one, as in `if w then dec(x)`.
  Partial applications compute saved-slot masks from the saved witness
  arguments. Class dictionaries can carry their type parameter's witness.
- **Cost.** One extra argument per relevant type variable on every
  polymorphic call. Constructing in polymorphic code needs a branch or
  mask arithmetic. Dup or drop of a bare polymorphic value is a witness
  test. There is no tagging and no fallback allocation, and the full
  64-bit range is kept.
- **ABI.** Every polymorphic scheme and every constructor with a
  type-variable field changes arity. That arity is the `.cinchif`
  `ValueExport.arity` (`interface.rs:144-151`). Whether a type variable
  "needs" a witness must be decided from the signature alone, or it is not
  ABI-stable.

### 2.5 Option C: representation monomorphisation

Specialise each polymorphic unit per vector of its type parameters'
classes. There are at most `3^k` variants, and this is finite even under
polymorphic recursion, because the class of `List a` is `Obj` whatever `a`
is. Per-block masks are compile-time constants at every construction site.
There are no witnesses and no tagging.

- **Requirements.** Every use of a polymorphic global, constructor or
  instance must record the class vector of its instantiation (the same
  information as B, consumed statically). Specialisation must happen where
  types are known, which means the checker or a typed `Core`.
- **Cost.** Zero runtime overhead beyond masks. Code size grows by up to
  `3^k` per function, typically `k <= 2` [INFERENCE]. Compile time grows.
- **ABI and separate compilation.** Today emission is whole-program
  (`incremental.rs:34`), so this is easy now. Later, interfaces must ship
  polymorphic bodies (template-style) or pre-instantiate all `3^k` variants
  per export. Hot replacement (`specifications/runtime-evolution-specification.md:31`,
  `:92`) must swap all instances.
- **Open front.** Parameterised modules (`check_decls.rs:2145-2150`) and
  future module functors over abstract types (`modules-and-compilation-specification.md:14`)
  are more instantiation points. [INFERENCE] They fit C only if functor
  application is also specialised.

### 2.6 Small `Nat`/`Int` immediates (independent of A/B/C)

`Nat`/`Int` words are opaque to generated code ("generated code MUST treat
the block as opaque", `lib.rs:51-53`). Heap blocks are 8-aligned
(`abi.rs:280`), so the runtime alone can adopt `TObj`: low bit 1 means an
immediate `n << 1 | 1`, which covers 63-bit `Nat` (unsigned) and signed
62-bit `Int`. Overflow promotes to a box. Codegen then folds `ConstBig`
small literals to constants (`codegen.rs:4194-4207`). This removes the box
from nearly every `Nat` operation. It matters most for HashMap heights and
digests and for loop counters (section 1.2). Under A it unifies with
polymorphic tagging for free. Under B and C, `Nat` is simply the `TObj`
class. It is the largest single block-count reduction available, and no
option precludes it.

**As built (S2).** The ranges are `Nat` `0 ..= 2^63 - 1` (exactly the
non-negative `i64` range, so `cinch_nat_from_i64` never allocates) and
`Int` `-2^62 ..= 2^62 - 1` (the 63-bit two's complement payload, one
arithmetic shift to decode). Results are canonical: every entry point
returns the immediate whenever the value is in range, so a box only ever
holds an out-of-range value, a box result that falls back in range
demotes, and two words of the same small value are the same word.
Mixed and boxed operands run the shared limb arithmetic over borrowed
magnitudes (an immediate's limbs sit on the stack), so no temporary
bignum is built for an immediate operand. Codegen folds an in-range
`ConstBig` to its constant word, and the quote encoder emits it as the
raw `INT` word, so the evaluator neither parses nor allocates it; an
out-of-range `BIG` constant is parsed once into its canonical word. The
compile-time evaluator (D92) carries decimal `Value::Big` and lowers to
`ConstBig`, so it materialises no runtime words. Measured with
`CINCH_ALLOC_STATS=1` under the 2 GiB cap: `Run.Maps` 15,457,002 blocks
to 3,007,402 (385.8 MB to 130.4 MB allocated, the latter with S1's
headers); the bootstrap lexer on `Data/HashMap.cinch` 3,639,966 to
673,892; the bootstrap parser on the same module 2,608,257 to 559,918.

### 2.7 Comparison

| | A: tagged polymorphic slots | B: witnesses | C: class monomorphisation |
|---|---|---|---|
| Info `Core` must carry | binder classes; `Tag`/`Untag` nodes | binder classes incl. `Poly(witness)`; witness args at instantiations | binder classes incl. `Var(i)`; class vectors at every polymorphic use |
| Checker change | coercions at instantiation sites | `Repr a` constraint solved like `Copy`; arity of every polymorphic scheme/constructor | instantiation class vectors; specialiser |
| Codegen change | stubs tag/untag; per-constructor static layouts | dynamic masks at construction; witness-guarded dup/drop | specialised units; static masks |
| Runtime change | `KIND_BOXED_SCALAR`; `TObj`-aware drop | mask-reading drop | mask-reading drop |
| `I64`/`U64`/`F64` in polymorphic slot | tag, or allocate if no fit (always for `F64`) | raw, no allocation | raw, no allocation |
| `Char`/`Bool`/`Unit` in polymorphic slot | tag, no allocation | raw | raw |
| Small `Nat` (2.6) | native fit | `TObj` class | `TObj` class |
| Dup/drop, polymorphic value | bit test | witness test | none (static) |
| Polymorphic call | tag/untag scalars | +1 arg per relevant type variable | none |
| Indirect call | tag/untag scalars in stub | none | none |
| Block scan info | static per constructor (layout id) | dynamic mask per block | static mask per specialised site |
| Drop-reuse | size match, static layout | size match, rewrite mask | size match |
| FFI `repr(C)` | unaffected (fields are scalars or roles, D66, `foreign-function-interface-specification.md:108-116`) | unaffected | unaffected |
| wasm32 | same scheme; pointers are 32-bit inside `i64` words (`codegen.rs:129-135`) | same | same |
| `.cinchif` / ABI | `runtime_abi` bump only; classes follow from types | arity changes per export | polymorphic bodies or pre-instantiations per export |
| Spec conflict | `primitive-types-specification.md:22` | none found | none found |
| Churn | moderate: coercions, stubs | high: every polymorphic signature and constructor | high: specialiser plus typed instantiation data |
| Precedent with Perceus/reuse | Lean 4, Koka | Swift (ARC, not Perceus reuse) | MLton/Rust-style specialisation |

### 2.8 Recommendation

**I recommend A as the ABI, with section 2.6 immediates, and C-style
specialisation of hot polymorphic code later as a pure optimisation.**

Why A:

- It changes no user-visible arity. The class of every slot follows from
  its declared type, so `.cinchif` needs only the `runtime_abi` bump.
- Blocks are self-describing with static per-constructor layouts, so the
  generic drop and reuse need no witnesses.
- Closures scan uniformly.
- It is the representation both production Perceus implementations use.

Its cost is concentrated: tag and untag ALU operations at polymorphic
boundaries, and allocation for 64-bit scalars and floats stored
polymorphically. [INFERENCE] In the bootstrap programs, digests and heights
are `Nat` (`HashMap.cinch:63`), so the fallback rarely fires. Whole-program
emission (`incremental.rs:34`) lets a later slice specialise hot
polymorphic units per class vector (option C) behind the A ABI, removing
the boxing where profiles show it.

**Choose B instead** if the user wants `primitive-types-specification.md:22`
to hold without amendment: no allocation for any primitive in any position.
B pays with an arity change on every polymorphic signature.

---

## 3 Block header

**Placement.** Put one header word at offset -8. Every existing offset stays
valid: no codegen site reads at a negative offset, and every `gep`,
`load_at` and `store_at` is relative to the pointer the allocator returned.
That covers constructor fields at `(1+i)*8` (`codegen.rs:3110-3111`,
`4734`), projections (`4289-4300`), dictionary methods (`4282-4288`,
`3855`), the closure words `0/8/16/24+` (`1257-1288`, `4354-4357`,
`1387-1389`), boundary payloads at `block+8` (`3091-3101`, `3262-3320`),
and the evaluator's closed-form layouts (`eval.rs:74-82`). The runtime's
`Boxed<T>` keeps `kind` first (`abi.rs:60-64`). The changes are confined to
these places:

- `cinch_alloc_raw` allocates `size+8` and returns `base+8`. `cinch_free`
  passes `ptr-8` and the recorded size to `dealloc`, which needs the original
  `Layout` (`abi.rs:280-283`).
- The runtime boxes move off `Box::into_raw` onto a header-aware allocator,
  dropping the payload with `drop_in_place`.
- Static objects grow a leading header and are addressed one word in. That
  covers `[1 x i64]` nullary blocks (`codegen.rs:1176-1181`), `@"U.clo"`
  (`codegen.rs:1244-1250`), and the string literal composite
  `{tag; buf_word; kind; len; data_word; data}` (`codegen.rs:4249-4260`),
  which needs a header before each of its two inner blocks, so its GEP
  indices `2` and `5` shift.

**Layout (recommended for A):**

`rc: i32 (low half) | kind: u8 | flags: u8 | layout: u16`

- `rc > 0`: single-threaded count. `rc < 0`: multi-threaded, atomic
  (section 8). `rc == 0`: immortal. All static objects, interned `Sym`
  cells (`sym.rs:10-12`: "never freed"; `abi.rs:86-89`) and
  `KIND_BYTES_STATIC` buffers get 0.
- `kind` covers ctor, closure, dictionary, memo, knot, nat, int, bytes,
  bytes-static, sym, code (quote), boxed scalar, and freed (debug).
- `layout` indexes a codegen-emitted table `@__cinch_layouts` of
  deduplicated `(size_words, scan mask)` shapes for constructor and
  boundary blocks. The other kinds derive size and scan from the kind:
  closure `3+filled` words with slots `3..` scanned
  (`codegen.rs:1257-1259`), dictionary `1+nmeth` with all methods scanned,
  memo `{state; thunk; value}`, knot `{state; value}`, and code `4+n`.
- Under B the `layout` field becomes an inline 16-bit mask with an
  overflow word.

**Overflow.** An increment that would pass `i32::MAX` sets the count sticky
(treated as immortal). The block then leaks, and the leak counter reports
it (section 9). Koka uses the same rule.

**As built (S1).** The encoding lives in
`compiler/runtime/src/block_kinds.rs`, which the runtime's `block.rs` and
the code generator both include verbatim, so the kind numbers cannot
drift. Where S1 differs from the plan above:

- `cinch_alloc_raw(size, kind)` takes the header kind as a second argument
  and the runtime writes the whole header in one store. Generated code
  never encodes a heap header itself; it only encodes static ones.
- Until the `@__cinch_layouts` table exists (S5), `layout` holds the
  payload size in words for every block, so `cinch_free` rebuilds the
  `Layout` from the header alone without a per-kind size rule. A payload
  of `0xFFFF` words or more stores `layout = 0xFFFF` and keeps its byte
  size in one extra word at offset -16. S5 replaces this meaning of the
  field with the table index; kinds that derive their size from the kind
  then stop reading it.
- One kind beyond the list above: `raw` (13), the unscanned byte buffers
  the Wasm canonical ABI glue lowers into. They are not Cinch values.
- Static blocks are `{ header; block }` storage `@"U.clo#h"`/`@"U.blk#h"`
  with `@"U.clo"`/`@"U.blk"` an LLVM alias one word in, so no reference
  site changed. The string composite is
  `{ hdr; tag; buf_word; hdr; kind; len; data_word; data }` (indices 1,
  4 and 7 are the addressed points).
- The stats line gains `header-blocks`, `header-bytes`, `immortal` (blocks
  allocated with `rc = 0`, i.e. interned `Sym` cells; static blocks are
  never allocated and are not counted) and `freed` (calls of `cinch_free`
  that deallocated). `live` stays with S3.
- The header is `rc: 40 bits signed (low) | kind: u8 | layout: u16`, with
  no `flags` field. Only the widths are named constants (`RC_BITS`,
  `KIND_BITS`, `LAYOUT_BITS` in `block_kinds.rs`); every shift, mask and
  the `RC_MAX`/`RC_MIN` range derive from them, so a width change is local
  to that file. No sticky overflow state was built, and the increment
  needs no check: the heap is capped below 8 * 2^40 bytes.
- Block memory comes only from `block::allocate` and goes back only
  through `block::free`, the one place a different backing allocator
  plugs in. The header spends no bits on allocator identity.

**As built (S1b).** The constructor tag is in the header and constructor
blocks have no tag word: a `Cons` cell is 24 bytes. Where S1b differs
from the row in section 10:

- Layout ids are assigned at whole-program emission, one group of
  `2^CTOR_TAG_BITS` (64) consecutive ids per data type, `id = group << 6 |
  tag`. So `tag = id - base` with `base = group << 6`, and every reader
  computes it as `id & CTOR_TAG_MASK`. `Core::Case` carries no type, so
  the mask is what lets a `case`, the quote evaluator and the task
  runtime read a tag without knowing the scrutinee's type. The cost is
  the same (load, shift, mask). The id space is sparse: 1023 groups in 16
  bits.
- The checker records each constructor unit's data type
  (`UnitRepr::data`); tuples, which have one constructor, use the unit
  name. The groups of `Option`, `Result` and `Ordering` are fixed (0, 1,
  2) because generated code builds them without a constructor unit in
  hand. A program that declares them with other payloads is refused.
  Blocks no unit names (canonical ABI lifts, boxed-out nested aggregates,
  foreign aggregate results) take an anonymous group per payload size:
  their tags read correctly, but S5's pointer maps will need their real
  types.
- `cinch_free` sizes a constructor block from `@__cinch_layouts` (one
  `i64` per id: the words in the low 32 bits, the high bits reserved for
  S5's pointer map). A global constructor registers the table through
  `cinch_layouts_register`. Other kinds keep size-in-header.
- Escape: a type of more than 64 constructors, or one past the last
  group, stores `LAYOUT_BIG` in the header and the descriptor `index << 40
  | tag << 16 | 0xFFFF` at -16. When a program has any escaped type (or
  any canonical ABI binding, whose enum tags arrive at run time), every
  tag read branches on `LAYOUT_BIG`. Otherwise tag reads stay
  branch-free.
- A `case` whose alternatives are all defaults does not read its
  scrutinee, so an immediate never reaches a header load. The evaluator
  follows the same rule.
- The runtime boxes (`Nat`, `Int`, `Bytes`, `Sym`) dropped their in-block
  kind word, which duplicated the header kind; the kind checks read the
  header instead. `cinch_alloc`/`cinch_alloc_raw` refuse `KIND_CTOR`;
  constructors go through `cinch_alloc_ctor(size, desc)`.
- Measured: `LIST_1000` went from 56,862 to 48,862 bytes (1,922 blocks
  both). `Run.Lex` over `Data/HashMap.cinch` went from 18,822,997 to
  15,077,685 bytes (header bytes 18,379,144 to 14,633,832, minus 20%),
  with 673,939 blocks and identical output.

---

## 4 Insertion algorithm on `Core`

**Placement.** A new pass, `compiler/src/rc.rs`, runs on the residual
program after `core::optimize` and before `codegen::emit_for`, at the four
driver sites listed in section 2.2. It must run after driving, because
driving duplicates and discards subterms (`core.rs:6203-6206`) and would
break the balance of inserted operations. New nodes:

- `Dup(x)` and `Drop(x)`, sequenced before an expression;
- `DropReuse(x, token)`;
- `Alloc@token`;
- `IsUnique(x)`.

Codegen inlines the fast paths. Dup is: if not `TObj`-immediate, load the
header; `rc > 0` means `rc++`, `rc < 0` means an atomic call, `rc == 0`
means skip. Drop is: `rc == 1` calls `cinch_drop_slow`, `rc > 1` does
`rc--`, and the other cases as for dup.

**Ownership.** Every `Obj`/`TObj` binder is owned or borrowed. Function
parameters get their mode from inference (below). `Let` binders and
constructor fields bound by `Case` are owned once the pass has dup'd them.
A borrow (`borrow e`/`mutBorrow e`) elaborates to the operand's own word
(`check_terms.rs:4222-4260`), so `Ref r a` binders are borrowed. `demand`
should return a borrow (`specifications/lazy-computation-and-reclamation-specification.md:75`),
so its result is borrowed as well.

**Rules.** These are per control-flow path, and they are garbage-free: a
drop goes as early as possible.

- **Owned use.** These positions consume one reference: an argument to an
  owned parameter, a constructor field, a `Lam` env element, a returned
  value, a `LazyInit` thunk, a `BoxInit` value, and a `RefSet` value. An
  owned variable with n owned uses on a path gets `n-1` dups. If it is
  dead on a path, it is dropped at the start of that path.
- **`Let(x, e, b)`.** If `x` is unused in `b`, drop `x` right after `e`.
- **`If` and `Case` branches.** Drop every owned variable live into the
  construct but unused in the branch at branch entry.
- **`Case`.** The fields of `CPat::Con` are borrowed loads from the
  scrutinee (`codegen.rs:4728-4737`). In each alternative, the pass dups
  each field that is used owned, then drops the scrutinee if it is dead.
  Under drop specialisation the alternative becomes
  `if IsUnique(x) { drop unused fields; free x's cell } else { dup used fields; dec x }`,
  so the unique path moves fields without count traffic. A non-variable
  scrutinee is first bound to an owned temporary. Patterns are flat
  (`core.rs:1547-1553`), so nested projection is just more `Case`s.
- **`Proj`, `DictProj`, `SpliceVal`, `BoxGet`, `Demand` results.** These
  are borrowed from their container. An owned use dups them. The container
  must stay live until the last borrowed use, so its drop goes after that
  use.
- **Borrowed positions with a computed argument.** For example,
  `natAdd (natMul a b) c`: runtime externs read their arguments by
  reference (`abi.rs:330`). The pass binds the temporary and drops it after
  the call. This is ANF-lite, introduced only where needed.
- **`App` of a known unit, saturated.** Ownership follows the callee's
  parameter modes. A variable whose last use is a borrowed argument is
  dropped after the call, with one exception: in tail position nothing may
  follow the call. The constant-stack rule for tail recursion is MUST-level
  (`design/recursive-bindings-and-tail-calls.md:67-77`), and codegen emits
  `musttail` and self-loops (`codegen.rs:4566-4634`). So every drop moves
  before the tail call, and borrow inference makes a parameter owned when it
  is passed on in tail position. This is Lean's tail-call preservation rule.
- **`App` through `__apply`** (dynamic head, partial or over-application).
  `__apply(f, argc, argv)` borrows `f` and takes the arguments owned. The
  `@"U.app"` stub adapts to the unit's modes:
  - *Saturate:* the saved arguments are copied to an `alloca`
    (`codegen.rs:1293-1316`). Saved arguments going to owned parameters are
    dup'd, or moved without dup if the closure is unique and consumed.
    Arguments going to borrowed parameters are dropped after the call.
  - *Grow:* the old closure's saved arguments are copied with `memcpy`
    (`codegen.rs:1269-1291`). Each copied object slot needs a dup, unless
    the old closure is unique, in which case the slots are moved and the old
    block freed.
  - *Over-apply:* the intermediate closure from the first call is owned, so
    it is dropped after the `__apply` on it (`codegen.rs:1331-1334`).
- **`Lam { fn_id, env }`.** The env elements are owned uses that move into
  the closure. The lifted function receives env as its leading parameters
  (`core.rs:1662-1663`). Those parameters are borrowed from the closure
  while it runs, because `__apply` borrows it.
- **`InstDict(g, args)`.** It calls `mk`, which stores subdictionaries into
  method closures (`codegen.rs:3860-3865`). So the arguments are owned and
  the result is a fresh owned dictionary. `mk` allocates on every call,
  even with no subdictionaries (`codegen.rs:3833-3834`). Zero-subdict
  instances should become immortal static dictionaries. That is an
  independent saving.
- **Prims and foreign calls.** Each `PrimOp` gets an ownership signature.
  Runtime externs borrow their arguments. A wrapping result such as
  `ret_option` (`codegen.rs:1731`) takes its value argument owned.
  Foreign calls pass scalars, roles and by-value copies of `repr(C)`
  aggregates (`codegen.rs:3671`, `3758`), so they borrow.
- **`RefSet(l, v)`.** It stores into the binder's own stack slot
  (`codegen.rs:4494-4501`), and the slot first holds the borrowed operand
  word. Proposed: elaborate `mutBorrow` as an owned copy (one dup at borrow
  time). Then `RefSet` drops the old slot value and stores the new one, and
  `l` is dropped at the end of its scope. [INFERENCE] The bootstrap's
  `Mut` never writes back to the owner today, so this matches its current
  semantics.
- **`Fault`.** The path ends (exit 70), so nothing is dropped.
- **`Quote`/`DeclsQuote`/`SyntaxLit`/`StrLit`.** These are compile-time
  only. Runtime quote blocks own their holes (`codegen.rs:6175-6178`).

**Borrowed parameter inference** (Lean 4 / Perceus). Start with every
`Obj`/`TObj` parameter borrowed. Mark a parameter owned if it is:

- stored, returned or captured;
- passed to an owned parameter;
- a reuse candidate;
- passed on in a tail call that needs it owned.

Iterate to a fixpoint over the whole-program call graph. Modes are ABI
once emission is per-unit, so `ValueExport` would gain modes, or exports
default to owned. Dictionary and capability parameters (`FnMeta.dicts`,
`core.rs:2316-2319`) nearly always stay borrowed.

**Checking.** A debug verifier re-derives ownership over the output and
asserts that each owned binder is consumed exactly once on each path
(Lean's IR checker). It runs under `debug_assertions` in tests.

---

## 5 Reuse and multiplicities

**Drop-reuse.** When a `Case` alternative consumes its scrutinee `x` and
allocates a block of the same `size_words`, emit `DropReuse(x, r)` at the
consumption: if `x` is unique, drop its unused fields and keep the cell as
`r`; otherwise decrement `x` and set `r = null`. Then emit `Alloc@r`: if
`r` is non-null, overwrite the header layout, tag and fields in place;
otherwise allocate. Reuse specialisation skips stores of unchanged fields.
This turns HashMap/AVL path copying (`HashMap.cinch:61-63`) and `map`-style
rebuilds into in-place updates whenever the structure is uniquely held. In
the lead's numbers, that is the "AVL node copies" half of the `Run.Maps`
block count. Grow-path closures and `Bytes` builders can reuse the same way.
A `Bytes` concat that owns a unique left operand can `realloc` in place
[INFERENCE], which complements the `Text.Builder` fix.

**What the checker actually enforces.** `Mult` is
`{Zero, One, Aff, Many, Flex}` (`types.rs:248-263`) and sits on arrows
(`types.rs:1333`, `1374`). Unannotated binders default to `%many`,
`Thunk`/`Later` to `%aff`, and anything mentioning `Task` to `%1`
(`types.rs:1751-1765`). `check_usage` compares use counts
(`check.rs:3425-3497`):

- `%0` must never be used;
- `%1` must be used exactly once on every path;
- `%aff` must be used at most once;
- `%many` is unchecked (`check.rs:3449`), so a `%many` `Bytes` or function
  binder may be used repeatedly even though neither type is `Copy`
  (`core-language-specification.md:219-221`). [INFERENCE] I found no other
  site enforcing it.

`FnMeta.linear` records `%1`/`%aff` parameters for the optimizer
(`core.rs:2307-2311`). The tests are `ownership.rs:35`, `51`, `67`, `85`,
`402`, `444` and `530`, `concurrency.rs:200` and `225`, `effects.rs:206`,
`evolution.rs:393`, and `optimizer.rs:535`.

**Static uniqueness.** A `%1` or `%aff` binder is never used twice, so the
pass inserts no dup for it. A dropped `%aff` binder gets exactly one drop.
But linearity is not uniqueness. A `%1` parameter can receive a value the
caller also holds (a `%many` binder used once here and again elsewhere), so
reuse still needs the runtime `IsUnique` check. The pass can still prove
`rc == 1` for values that are freshly allocated and never dup'd before
their consumption, and elide the check there. Guaranteed in-place update,
such as a `Unique` type discipline in the style of Clean, would be a new
language feature. That is an open question (section 12).

---

## 6 Cycles and laziness

Strict immutable data cannot form cycles. A block can only point at blocks
that existed before it. Local recursive functions do not form cycles
either: they become units, and a self-reference rebuilds `MkClos` from the
captured variables ("functions through their units",
`check_terms.rs:5485-5487`, `5504-5513`). What can form a cycle:

| Source | Cycle | Reclamation rule |
|---|---|---|
| `letrec` value knot `BoxAlloc`/`BoxInit` (`check_terms.rs:5494-5515`, `codegen.rs:4425-4468`) | box -> value -> closure env -> box | Ordinary RC for the acyclic part. At the end of the group's scope, if a `Core` escape analysis proves the value does not escape, tear it down: drop the value and reset `state` to 0. A later `BoxGet` then takes the existing black-hole fault (`codegen.rs:4456-4463`, exit 70) instead of a use-after-free. If escape is not proven, the cycle leaks, bounded to once per evaluation, and the leak check reports it as `cyclic`. |
| `~` memo group (`check_terms.rs:5489-5503`, cell `{state; thunk; value}` `codegen.rs:4469-4480`) | unforced thunks capturing sibling cells | `__cinch_demand` (`codegen.rs:1356-1380`) should drop the thunk after evaluation by storing 0, which removes that cell's outgoing edges. At region exit (the spec's teardown, lazy spec `:70`, `:73`), take each cell of a group proven non-escaping in reverse allocation order. If it is unforced, drop its thunk and set `state = 1` (a later demand faults through `bad`, exit 70). Then drop the group's reference. A cached value cannot contain a cell, because cells are not `Copy` (`core-language-specification.md:219-221`) and a cached value must be `Share` (lazy spec `:75`). But the bootstrap enforces neither `Share` nor region escape (lazy spec `:137`), so until it does, only proven non-escaping groups are torn down. |
| `Ref`/`RefSet` | none: `RefSet` writes a stack slot (`codegen.rs:4494-4501`), and `Bytes` writes are bytes, not words (`abi.rs:179-192`) | none needed |
| closures capturing themselves | only through a knot (above) | as knot |
| dictionaries | `InstDict` rebuilds per call, so there is no cycle | none needed |

No tracing is involved: every rule is a static scope end plus a local
traversal of one group. Teardown never frees a block that RC still counts
from outside. It only cuts the group's internal edges, and the fault
sentinel makes a wrong escape proof a defined fault rather than memory
unsafety.

---

## 7 Runtime-internal values

| Value | Today | Under RC |
|---|---|---|
| `Nat`/`Int` | `Boxed<Nat>` with a `Vec<u32>` limb vector (`nat.rs:14`), never freed | Immediates allocate nothing (2.6). A box drops its payload, which frees the limbs, then frees the block. Later: limbs inline in the block, one allocation instead of two. |
| `Bytes` | `Boxed<Bytes>{len,data}` (`bytes.rs:22`) plus a `Box<[u8]>` (`bytes.rs:90`) | Leaf: free the data, then the block. `KIND_BYTES_STATIC` gets `rc = 0`. Shared mutable buffers keep today's semantics. |
| `Sym` | interned and immortal (`sym.rs:10-12`, `abi.rs:86-89`) | `rc = 0`. It may be classed `Raw` to skip counting entirely. |
| Proc, dir and buf handles | `Box` pointers freed by explicit close (`platform.rs:132`/`141`, `312`/`346`, `565`/`734`) | Class `Raw`, never counted. Explicit linear consumption stays the only free. Tagged when stored polymorphically under A. |
| socket, pty and pipe handles | integer table keys removed on close (`net.rs`, `pty.rs`, `platform.rs`) | `Raw` |
| task and scope handles | `%1` (`types.rs:1756-1762`); the handle `Arc` is leaked by design (`coop.rs:204`, `task.rs:133`) | `Raw`. Join could release the handle `Arc` (a separate small fix). |
| quote code blocks | `KIND_CODE` (`eval.rs:153`), built at `codegen.rs:6175-6178` | kind `code`, holes scanned |

**`cinch_free` semantics.** It becomes the raw deallocation of a block
whose count has reached zero: header-aware, size from the header or kind,
no child traversal. This keeps its documented role of balancing
allocation. New entry points: `cinch_drop_slow(word)` for the
zero-crossing path, `cinch_dup_mt`/`cinch_drop_mt` for atomic counts, and
`cinch_mark_mt` (section 8).

**Iterative drop.** A long list must not overflow the stack. `cinch_drop_slow`
uses an intrusive stack threaded through the dead blocks' own header words:

1. pop block `b`;
2. for each scanned child `c`, decrement it; if it reaches zero, link `c`
   onto the stack through `c`'s header;
3. free `b`.

This needs no recursion and no extra memory. Children are visited in
reverse slot order (section 11, item 3). The evaluator's decoded tree is
already depth-bounded (`eval.rs:99-104`, `MAX_NESTING`).

---

## 8 Concurrency

Tasks are one OS thread each, with fork-join happens-before at spawn and
join and no other atomics (D80, `specifications/concurrency-specification.md:150-154`;
`task.rs`). Task bodies use one-shot frames. `%1`/`%aff` captures transfer
exactly once. `Ref`/`Copy` values and shareable dictionaries are shared.
`Mut` is refused, and so are untransferred `Bytes`/`Lazy` cells (D77,
`concurrency-specification.md:140-143`). The spec requires `Send`-like
captures and a `Sync`-like referent for cross-thread sharing
(`concurrency-specification.md:39-41`).

A shared `Copy` capture or `Ref` borrow is reachable from two threads. A
task that dups it races with the parent's count operations. Transferred
values can share substructure with values the parent kept, so ownership
transfer alone does not isolate them. Options:

1. **Mark on spawn (recommended, Lean 4).** `cinch_task_spawn(scope, closure)`
   (`abi.rs:625-628`) first calls `cinch_mark_mt(closure)`. That traverses
   the reachable graph iteratively, stops at immortal or already-marked
   blocks, and negates the counts. From then on those blocks use atomic
   operations, and every other block keeps plain ones. The fast path's sign
   test already separates the cases (section 3). Results returned by `join`
   need no marking: the child has finished, and join happens-before the
   parent's uses.
2. **Always atomic.** Simple, but it puts an atomic read-modify-write on
   every dup and drop in single-threaded code.
3. **Deep copy at spawn.** It contradicts "potentially expensive copies
   SHOULD be explicit" (`core-language-specification.md:132`).

The Wasm cooperative backend (`coop.rs`) is single-threaded, so it never
marks. Memo cells never cross threads (lazy spec `:77`, `:122`).

---

## 9 Verification plan

- **Counters.** `heap.rs` already counts blocks, bytes, current and peak
  (`heap.rs:40-43`, `104-119`) and prints them with `CINCH_ALLOC_STATS=1`
  (`heap.rs:17-22`). Add a separate counter for header blocks (Cinch values)
  versus Rust-internal allocations, an immortal count, and a sticky count.
  Add `live=<n>` to the stats line.
- **Leak check.** Add `CINCH_LEAK_CHECK=1`: after `main` returns
  (`codegen.rs:427`, `436`: insert a `cinch_exit_check` call before `ret`),
  exit nonzero (a new code, e.g. 74) if live header blocks minus immortal,
  sticky and reported-cyclic blocks is not 0. Rust-internal allocations,
  such as handle tables and the intentional task `Arc`s, are reported but
  not judged.
- **Per-program test.** Extend `compiler/tests/alloc_accounting.rs`
  (`stats_line_counts_list_cells_only_when_asked` at `:75`,
  `unbounded_allocation_hits_the_named_cap` at `:98`,
  `normal_program_runs_under_the_default_cap` at `:113`) with:
  - "a loop that builds and drops an N-cell list N times has peak about one
    list";
  - `live=0` for programs that free everything;
  - a 10^6-element list dropped without stack overflow.

  The `peak <= bytes` assertion becomes "peak much less than bytes" for
  those programs.
- **Memory benchmarks.** Run the `bootstrap/programs/Run.*.cinch` programs
  (via the `compiler/tests/bootstrap_*.rs` tests, `lib_hashmap_real.rs`,
  and `tooling/run-bootstrap-ladder.sh`) with `CINCH_HEAP_LIMIT=2G` and
  `CINCH_ALLOC_STATS=1`. Record blocks, bytes and peak per slice. The
  section 1.2 table is the baseline. Expected after slices S2 and S6:
  `Run.Maps` completes, with peak tracking the 32000-entry map
  [INFERENCE].
- **Use-after-free detection.**
  - A debug runtime mode, `CINCH_RC_DEBUG=1` (or a `rc-debug` cargo
    feature), makes codegen call out-of-line `cinch_dup`/`cinch_drop` that
    validate the header kind. Freed blocks are poisoned (kind `freed`,
    payload `0xdeadbeef...`) and quarantined instead of returned to the
    allocator.
  - ASan: link with `clang -fsanitize=address`. The runtime allocates
    through `System` (`heap.rs:228`, `malloc`), so ASan intercepts it even
    with an uninstrumented Rust runtime [INFERENCE]. Valgrind works on
    Linux only [INFERENCE].
- **Semantic guards** (existing). `compiler/tests/integration.rs` (32
  tests), `letrec.rs` (10), `ownership.rs` (19), `concurrency.rs` (14),
  `ffi.rs` (12), `codata.rs` (9), `wasm.rs` (11), `alloc_accounting.rs`
  (3), `lib_hashmap_real.rs`, and `bootstrap_*.rs`. Run them one target at a
  time: `cargo test --manifest-path compiler/Cargo.toml --test <name>`.
  The debug ownership verifier (section 4) runs in all of them.

---

## 10 Staged implementation plan

Each slice lands and is tested on its own. **[indep]** means the slice does
not depend on the section 2 decision. S0 has landed.

| Slice | Content | Files | Depends | Decision |
|---|---|---|---|---|
| S0 (landed) | Counting allocator, cap, stats; codegen routed to `cinch_alloc_raw` | `runtime/src/heap.rs`, `abi.rs:263-288`, `codegen.rs` | none | indep |
| S1 | Header word at -8; `cinch_alloc_raw` returns `base+8` with `rc=1`; codegen writes kind, layout and size at all 30 sites; immortal headers on `.blk`, `.clo` and the string composite; runtime boxes and `eval.rs` allocations on the header allocator; `cinch_free` really deallocates (no caller yet); `runtime_abi` bump | `runtime/src/{abi,heap,eval,lib}.rs`, new `runtime/src/block.rs`, `codegen.rs` (alloc sites, statics `1176-1181`, `1244-1250`, `4222-4266`), `interface.rs:434` | S0 | indep (the layout table's scan encoding follows the decision; the field can land zeroed) |
| S1b | Constructor tag folded into the header (user decision, 2026-10-29): a two-field cell drops from 32 to 24 bytes, as in OCaml, GHC, Lean and Koka. Layout ids are assigned so each data type's constructors (per class vector) are consecutive, and `tag = layout_id - base`, with `base` a compile-time constant at every `case` (one load from the header, a shift and a subtract, the same cost as today's load of field 0). Every field offset shifts by -8: ctor bodies, projections, `case`, `eval.rs` closed-form layouts, boundary payloads, wasm glue, static blocks | `codegen.rs`, `runtime/src/{block,eval,abi}.rs` | S1 | indep |
| S2 | Small `Nat`/`Int` immediates; `ConstBig` folding | `runtime/src/{abi,nat,int,lib}.rs`, `codegen.rs:4194-4221`, `eval.rs` big constants | none | indep |
| S3 | Runtime drop machinery: `cinch_drop_slow` (iterative, kind dispatch, payload drop), sticky overflow, `live`/immortal/sticky counters, `CINCH_LEAK_CHECK`, `CINCH_RC_DEBUG` quarantine | `runtime/src/{block,heap,abi}.rs`, `codegen.rs:427`, `436` (exit hook) | S1 | indep, except the scan rule |
| S4 | Representation classes: `repr_of(&Ty)`; classes on `Let`, `CPat`, params and env; per-constructor field classes; optimizer preserves them; plus the option-specific front end: A `Tag`/`Untag` coercions; B `Repr a` witnesses; C class vectors and specialiser | `check.rs` (`finalize_fn` `772-791`, `lower` `793-883`), `check_terms.rs` (instantiation sites), `check_decls.rs` (beside `param_contract` `9653`), `core.rs` (nodes, optimizer), `interface.rs` (B: arity) | none | **decision** (the class plumbing part is indep) |
| S5 | Layout emission: `@__cinch_layouts`; closure ABI per the decision (A: stubs tag/untag, env stored tagged) | `codegen.rs` (`ctor_body` `3071-3113`, `stub_def` `1255-1340`, `Core::Lam` `4350-4368`, `mk_body` `3830-3879`) | S1, S4 | **decision** |
| S6 | Perceus pass: dup/drop insertion, borrow inference with tail-call preservation, drop specialisation, ownership verifier; codegen of `Dup`/`Drop` fast paths; stub dups in `@"U.app"`; `__cinch_demand` clears the thunk; prim ownership table; **in the same slice**, the contract scan classifies dup as a write and drop as `Callee::Free` (section 11, item 9) | new `compiler/src/rc.rs`, `core.rs` (nodes), `driver.rs:1228`, `1774`, `1860`, `1896`, `codegen.rs` (`classify_callee` `886-901`, attributes `567-588`, `1347-1380`) | S1, S3, S4, S5 | depends |
| S7 | Drop-reuse and reuse specialisation; zero-subdict static dictionaries | `rc.rs`, `codegen.rs` (`ctor_body`, `mk_body` `3833-3834`) | S6 | depends |
| S8 | Cycle rules: `Core` escape analysis; knot and memo-group teardown with fault sentinels; `cyclic` leak reporting | `rc.rs`, `codegen.rs:4425-4480` | S6 | indep of A/B/C |
| S9 | Concurrency: `cinch_mark_mt`, atomic slow paths, spawn hook (thread backend only) | `runtime/src/{task,abi,block}.rs`, `codegen.rs` spawn emission (`242`) | S6 | indep |
| S10 | wasm canonical glue headers and frees | `codegen.rs:6174-6720` | S1, S6 | indep |
| S11 | Spec and doc amendments (section 11) and the `lib.rs` ABI reference; rewrite `heap.rs:29-31` once frees exist | `specifications/*`, `design/ownership-and-usage.md`, `design/open-decisions-and-implementation-plan.md:65`, `runtime/src/lib.rs:13-104`, `heap.rs` | with S6 | indep |
| S12 (later) | Class specialisation of hot polymorphic units behind the A ABI; tag moved into the header; inline limbs | `core.rs`, `codegen.rs`, runtime | S6 | A only |

S1, S2 and S4's class plumbing can run in parallel now. S3 follows S1. S2
alone should remove most of the `Run.Maps` blocks before any free is
emitted.

---

## 11 Conflicts with the specifications, with proposed resolutions

1. **No runtime support.** `design/ownership-and-usage.md:97` says "no new
   runtime support is needed; ownership is a static property."
   *Resolution:* amend it. Ownership stays static, while reclaiming shared
   immutable values needs runtime counts.
2. **Unboxed primitives.** `primitive-types-specification.md:22` (unboxed,
   "no indirection, allocation") conflicts with option A only.
   *Resolution:* qualify it to monomorphic positions, or choose B.
3. **Destruction order.** There are three orders:
   - reverse capture order for a destroyed thunk (lazy spec `:42`);
   - reverse allocation order at region teardown (lazy spec `:70`, `:73`);
   - declaration order for values with owned resources
     (`ownership-and-usage.md:72-73`).

   RC frees at last use, not at scope end. The compiler implements no
   destructors or `destroy` (only the comment at `types.rs:253`), so memory
   reclamation order is unobservable today. *Resolution:* state that
   memory reclamation is unordered and unobservable. Effectful destructors
   of resource values are a static resource protocol, called at the affine
   drop point in the spec's order, and are never driven by counts. Make the
   runtime visit fields in reverse slot order, which equals reverse capture
   order for closures. The declaration-order versus reverse-order
   disagreement between the two documents remains for the user.
4. **Captures die at teardown.** Lazy spec `:73` ("captures die at region
   teardown") and `:133` ("MUST NOT change observable reclamation") conflict
   with RC, which releases earlier. *Resolution:* read it as "no later than
   teardown". Earlier release of memory is not an observation.
5. **Unshared `%many` thunks.** Lazy spec `:51`, `:53` says a `%many` thunk is
   unshared and recomputes. RC shares the immutable closure block
   physically, but forcing still recomputes and there is no shared memo
   state. *Resolution:* consistent. Clarify that "unshared" means no shared
   memo.
6. **No other atomics.** D80 says "no other atomics"
   (`concurrency-specification.md:150-153`), which conflicts with
   multi-threaded atomic counts. *Resolution:* D80 governs source-observable
   atomics, and counts are runtime-internal. Clarify the text.
7. **Shared task captures.** D77 sharing of `Ref`/`Copy` captures
   (`concurrency-specification.md:140-143`) is not a conflict, but it is a
   cost: marking at spawn (section 8).
8. **Constant-stack tail recursion.** The MUST in
   `design/recursive-bindings-and-tail-calls.md:67-77` constrains
   insertion: no code after a tail call. *Resolution:* the section 4 rules.
9. **LLVM contract attributes.** The scan emits `memory(none)`,
   `speculatable` and `nofree` from body classification
   (`codegen.rs:567-588`, `812-823`). An inline dup writes a header and a
   drop may free, so these claims become false and LLVM could merge or
   speculate calls that free. *Resolution:* in S6, classify dup as a write
   and drop as `Callee::Free` (`codegen.rs:886-901`). A unit stays
   `memory(none)` only if it performs no count operations, for example when
   all its parameters are borrowed and it never allocates.
10. **`#[no-alloc]`.** `design/safety-assertion-attributes.md:21` interacts
    with option A: `Tag` of a 64-bit scalar is an allocation site. Reuse is
    not an allocation, and drops are fine.
11. **Escape checking and `Share`.** Lazy spec `:137` says neither region
    escape checking nor `Share` for memo values is implemented.
    *Resolution:* teardown only proven non-escaping groups (section 6).
12. **FFI retention.** The FFI spec makes `Owned` pointers linear with
    allocator identity (`foreign-function-interface-specification.md:130`,
    `:48`). They are class `Raw` and never counted. Boxed ADTs never cross
    the boundary (D66, `:108-116`). A `Retained` borrow of a Cinch block
    held past a call needs a dup for the retention interval and a drop on
    release. [INFERENCE] No current binding does this.
13. **Interface pairing.** `design/compiled-interface-metadata.md:120-123`
    is not a conflict: it requires the `runtime_abi` bump in S1.

---

## 12 Decided, and open questions for the user

Decided by the user, 2026-10-29:

- **Destructor order: reverse.** Effectful destructors run last-acquired,
  first-released, matching the lazy spec's teardown (`:42`, `:70`);
  `ownership-and-usage.md:72-73` is amended to reverse order in S11.
  Memory reclamation itself stays unordered and unobservable (section 11,
  item 3).
- **Uniqueness: both.** Reuse is guarded by the runtime `IsUnique` check
  everywhere, AND the checker establishes uniqueness statically where it
  can (a value proven unaliased skips the check and the fallback
  allocation). A static proof is an optimization of a check that is
  always correct without it, so a gap in the analysis costs speed, never
  safety.

- **Representation: option C (confirmed by the user, 2026-10-29), with
  demand-driven specialization**: polymorphic code is specialized
  per class vector (`Raw`/`Obj`/`TObj`) only where a caller reaches it, with
  polymorphic bodies carried as IR in interfaces. This supersedes section
  2.8's recommendation of A. The layout model built on it (value types,
  enumerated layouts, hand-written specialized bodies, transparent
  representation instances, and proof-carrying specialization) is
  `design/layouts-and-specialization.md`; its proof side is
  `design/verified-primitives.md` and `design/proof-automation.md`. Under C,
  slice S4 is the class annotations plus the specialization collector (no
  `Tag`/`Untag` coercions), S5's closure ABI needs no tagging, and S12
  (class specialization behind the A ABI) is absorbed into S4. S1-S3 and S9
  are unchanged.

Still open (the numbering of the original list; 1, 2 and 6 are decided
above):

Revised 2026-10-29 after the user's objection to proposed leaks: the
language is GC-free AND leak-free by construction. No proposal below leaks.

3. **Escaping cyclic `letrec` values.** DECIDED by the user (2026-10-29): a recursive value
   knot is region-scoped exactly like a `~` memo cell. It lives in the
   region of the scope that creates it, is torn down at that scope's exit,
   and may not escape (the escape checker that already enforces `Ref r a`
   and `Mut r a` rejects it at compile time, naming the binding). A value
   needed beyond the scope is rebuilt acyclically or moved into a structure
   parameterized by the region. The earlier proposal (a bounded, reported
   leak) is withdrawn: it leaked once per evaluation, so a `letrec` in a
   loop leaked without bound. How many existing programs return a knot is
   unmeasured; S8 measures it over the prelude, examples and bootstrap
   before the rule lands.
4. **Memo groups.** DECIDED by the user (2026-10-29): enforce the lazy specification's own
   rule (`Lazy {e} r a` does not outlive `r`, lines 73-74) in the checker,
   which the bootstrap does not yet do for cells. Every group is then torn
   down at region exit, with no escape analysis needed and no leak and no
   fault. The earlier "leak the unproven groups" proposal is withdrawn.
5. **Cross-task sharing.** Mark-on-spawn (proposed), always-atomic counts,
   or copy?
7. **`mutBorrow`.** An owned copy at borrow time (proposed), or real place
   borrows first?
8. **Header budget.** DECIDED (D398): one word with a 40-bit count,
   an 8-bit kind and a 16-bit layout id (with one reserved id meaning "the
   shape is in an extra word", so the 65,536-shape limit is soft). A 40-bit
   count cannot overflow: the allocator caps the heap below 8 TiB, and 2^40
   references to one block would need 8 TiB of pointer words, so the
   increment needs no check and no block ever leaks (`design/no-faults.md`). The earlier 32-bit sticky count is
   withdrawn because sticky means leaked.
9. **Separate compilation.** Should exported functions record inferred
   borrow modes in `.cinchif`, or default to owned?
