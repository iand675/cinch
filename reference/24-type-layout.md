# Type layout and runtime representation

A source type, a native value word, an aggregate's byte layout, and a C calling convention are distinct layers. This chapter specifies explicit C/FFI byte layouts, alignment, packing, transparent wrappers, and foreign boundary marshaling. For the full native memory model — including default refinement-driven niche fusion, bit-packing, 3-bit pointer tagging, NaN-sentinel float sums, polymorphic niche specialization, `#[repr(value)]` POD structs, automatic unpacking, and CPR/CSR/WW register lowering — see [How memory layout works](31-memory-layout.md). For attribute attachment grammar and the complete `#[repr(...)]` index, see [attributes and pragmas](23-attributes-and-pragmas.md#syntax-and-attachment) and [every `repr` variant](23-attributes-and-pragmas.md#repr-every-current-variant).

# Mental model: four separate layers

| Layer | What it governs | Controlled by |
| --- | --- | --- |
| Source type | Type checking, inference, instance resolution, proofs | Declarations (`record`, `data`, `opaque`) |
| Native value word | Uniform 64-bit call/field slots and reference counting (`Raw`, `Obj`, `TObj`) | Compiler lowering, `#[repr(unboxed)]`, `#[unpack]` |
| Explicit byte layout | Field offsets, padding, alignment, and total byte size | `#[repr(C)]`, `#[repr(packed)]`, `#[repr(align(N))]`, `#[repr(transparent)]` |
| Foreign calling convention | Passing structs by pointer, register, `byval`, or `sret` | `foreign import`/`export`, `#[byval]`, `#[sret]` |

Explicit aggregate layouts describe a type's payload or marshaled bytes; they do **not** include the runtime allocation header. A value passed in a native 64-bit slot may be a pointer to that payload rather than the payload bytes inline.

Do not infer wire or persistence stability from the fact that a type compiles. `#[repr(C)]` follows the target platform's C ABI (endianness, scalar widths, alignment); it is not a portable serialization format. Use an explicit codec for network or file formats.

# Inspecting recorded layouts

`cinch check` writes a `.cinchif` interface file per module under `build/`. Its `abi.layouts` array records every type's computed representation:

```sh
cinch check Main.cinch
python3 - <<'PY'
import json
from pathlib import Path
interface = json.loads(Path("build/Main.cinchif").read_text())
for layout in interface["abi"]["layouts"]:
    print(layout["type"], layout["repr"], layout["size"], layout["align"], layout.get("discriminant"))
    for field in layout["fields"]:
        print(" ", field["name"], field["offset"], field["size"], field["align"])
PY
```

Consider a module declaring one `#[repr(C)]` record, one packed record, one transparent wrapper, one fixed-discriminant `data` type, and one unannotated record:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  #[repr(C)]
  record Header where
    kind :: U8
    length :: U32
    flags :: U16

  #[repr(C, packed(2))]
  record Packed2 where
    kind :: U8
    length :: U32
    flags :: U16

  #[repr(transparent)]
  record CounterWord where
    value :: U32

  #[repr(u8)]
  data Color where
    Red :: Color
    Blue :: Color

  record Plain where
    x :: I32

  main :: Int
  main = 0
```

Inspecting `build/Main.cinchif` prints:

```text
Header C 12 4 None
  kind 0 1 1
  length 4 4 4
  flags 8 2 2
Packed2 C, packed(2) 8 2 None
  kind 0 1 1
  length 2 4 4
  flags 6 2 2
CounterWord transparent 4 4 None
  value 0 4 4
Color u8 8 8 1
Plain uniform-word 8 8 None
```

Three rules show up immediately:

- A field's recorded `align` is its **natural** alignment, even when packing caps its placement alignment (`Packed2.length` is placed at offset 2 while keeping natural alignment 4).
- A fixed-discriminant `data` type (`Color`) records its discriminant width while keeping a 1-word native representation.
- An unannotated record (`Plain`) records `uniform-word` (size 8, alignment 8) with no C field layout.

This JSON output is a compiler interface artifact, not a source-level `sizeof`/`alignof` reflection API.

# Default layout (`uniform-word`)

Without a `#[repr(...)]` attribute, a record or `data` type has layout `uniform-word` (size 8, alignment 8): a 64-bit word holding a pointer to a runtime-managed heap object. Its fields are not laid out for C, and the FFI checker rejects it at foreign boundaries:

<!-- cinch:reject contains="uniform-word" -->
```cinch
module Main where
  record R where
    x :: I32

  foreign import ccall "fix_take" take :: R ->{Foreign Fix, IO} I64

  main :: Int
  main = 0
```

```text
cinch: 5:3: the recorded layout of Main.R is `uniform-word` (no representation attribute), which has no C representation at the boundary (foreign function interface specification section 2: only `#[repr(C)]`, `#[repr(packed)]` and `#[repr(transparent)]` aggregates cross; boxed aggregates and fixed-discriminant data are rejected instead of guessed)
```

Explicit layouts require concrete field types; an unresolved type parameter is rejected:

<!-- cinch:reject contains="not fixed" -->
```cinch
module Main where
  #[repr(C)]
  record Held a where
    value :: a

  main :: Int
  main = 0
```

Unannotated declarations have no C FFI layout (`uniform-word` at the boundary), though the native backend automatically applies niche fusion, bit-packing, pointer tagging, and small leaf record unpacking where eligible; see [How memory layout works](31-memory-layout.md). For explicit inline multi-word POD value types, use `#[repr(value)]` (optionally combined with `C`, `align(N)`, or `packed(N)`).

# C layout: field order, padding, and alignment

`#[repr(C)]` lays out non-erased fields strictly in declaration order:

1. Take the field's size and natural alignment from its scalar type or nested explicit layout.
2. Apply any field-level `#[repr(align(N))]` floor, then any applicable `packed` cap.
3. Round the running offset **up** to that effective placement alignment.
4. Place the field and advance the offset by its size.
5. Set the aggregate's alignment to the maximum effective field alignment (raised by any aggregate `#[repr(align(N))]` floor), and round the final size up to a multiple of that alignment.

Scalar sizes and natural alignments:

- `U8`, `I8`, `Bool`: size 1, align 1
- `U16`, `I16`: size 2, align 2
- `U32`, `I32`, `F32`, `Char`: size 4, align 4
- `U64`, `I64`, `F64`: size 8, align 8
- `Usize`, `Isize`: target profile pointer width (8 bytes on 64-bit targets, 4 bytes on `wasm32`)
- `Unit`: size 0, align 1 in explicit aggregate calculations
- Mathematical `Int` and `Nat` are **not** `I64`/`U64`; they use tagged-word/bignum representations.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  #[repr(C)]
  record Header where
    kind :: U8
    length :: U32
    flags :: U16

  main :: Int
  main =
    let header = Header { kind = 1, length = 42, flags = 3 }
    in if header.kind == 1 && header.length == 42 && header.flags == 3 then 0 else 1
```

`Header` has size **12**, alignment **4**, and field offsets **0, 4, 8**:

| Byte range | Contents | Reason |
| --- | --- | --- |
| 0 | `kind` (1 byte) | `U8` alignment 1 |
| 1-3 | Padding | Aligns `length` to 4 |
| 4-7 | `length` (4 bytes) | `U32` alignment 4 |
| 8-9 | `flags` (2 bytes) | `U16` alignment 2 |
| 10-11 | Tail padding | Rounds aggregate size to a multiple of 4 |

Padding bytes are uninitialized filler and must not be treated as meaningful or deterministic data.

Fields stay in declaration order. Reordering fields in source changes their offsets directly:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  #[repr(C)]
  record SmallFirst where
    small :: U8
    wide :: U32

  #[repr(C)]
  record WideFirst where
    wide :: U32
    small :: U8

  main :: Int
  main =
    let a = SmallFirst { small = 1, wide = 2 }
    in let b = WideFirst { wide = 3, small = 4 }
    in if a.small == 1 && b.small == 4 then 0 else 1
```

Both `SmallFirst` and `WideFirst` have size 8 and alignment 4, with offsets 0 and 4 in source order (`small` at 0 / `wide` at 4 in `SmallFirst`; `wide` at 0 / `small` at 4 in `WideFirst`).

Nested explicit layouts resolve recursively. A `#[repr(C)]` field inside a `#[repr(C)]` record uses the inner record's computed size and alignment (for example, an inner `(U16, U8)` of size 4 and alignment 2 placed after a leading `U8` starts at offset 2).

`#[repr(C)]` applies only to single-constructor products (`record` or single-constructor `data`, whose positional fields are recorded as `$f0`, `$f1`, ...). Multi-constructor `data` types are rejected:

<!-- cinch:reject contains="multi-constructor" -->
```cinch
module Main where
  #[repr(C)]
  data D where
    A :: D
    B :: D

  main :: Int
  main = 0
```

Recursive by-value layouts have infinite size and are rejected:

<!-- cinch:reject contains="infinite explicit layout" -->
```cinch
module Main where
  #[repr(C)]
  record Loop where
    next :: Loop

  main :: Int
  main = 0
```

```text
cinch: Main.Loop has an infinite explicit layout: its fields' layouts depend on the type itself (attribute annotations specification section 4)
```

# Native value representations

Native lowering passes values in uniform 64-bit word slots and assigns each slot a representation class used by reference counting and pointer maps:

| Class | Meaning | Examples |
| --- | --- | --- |
| `Raw` | Uncounted scalar or handle; never retained or released | Machine scalars, `Bool`, `Char`, `Unit`, `Sym`, opaque handles, capabilities |
| `Obj` | Counted pointer to a heap or static block | Records, ADTs, tuples, closures, dictionaries, `Bytes`, `String`, memo cells |
| `TObj` | Tagged immediate or counted object pointer | Mathematical `Nat` and `Int` |
| Parameter class | Polymorphic placeholder resolved per concrete call vector | Specializes units as `unit@RO`, `unit@OO`, etc. |

Small `Nat` values (`0` through `2^63 - 1`) and small `Int` values (`-2^62` through `2^62 - 1`) are encoded inline as `(n << 1) | 1`. The low `1` bit distinguishes an immediate from an 8-byte-aligned heap pointer; values outside those ranges box into bignum objects. These are purely representation thresholds, not numeric bounds.

In emitted LLVM IR, passing `42 :: Nat` alongside a heap-allocated `Nat` looks like:

```text
  %t0 = call i64 @"Main.big"()
  %t1 = call i64 @"Main.Pair"(i64 85, i64 %t0)
```

Here `85` is `(42 << 1) | 1`.

Refinement witnesses are `Prop`-erased: `NonZero Int` has the exact runtime representation of `Int`, and `unrefine` is a no-op. Likewise, `erased` record fields occupy no runtime slot and are skipped during layout calculation:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record Tagged where
    erased ok :: Eq 3 3
    val :: Nat

  get :: Tagged -> Nat
  get t = t.val

  main :: Int
  main = if get (Tagged { val = 4, ok = refl }) == 4 then 0 else 1
```

# Raising alignment (`align(N)`)

`#[repr(C, align(N))]` sets a minimum aggregate alignment. On `Header`, `#[repr(C, align(16))]` keeps field offsets at 0, 4, and 8 while raising aggregate alignment and rounded size to 16.

`N` must be a nonzero power of two:

<!-- cinch:reject contains="non-zero power of two" -->
```cinch
module Main where
  #[repr(C, align(3))]
  record Bad where
    a :: U8

  main :: Int
  main = 0
```

`align(N)` can only raise alignment; requesting a value below the natural alignment is an error (use `packed(N)` to lower alignment):

<!-- cinch:reject contains="only raises" -->
```cinch
module Main where
  #[repr(C, align(4))]
  record A where
    a :: U64

  main :: Int
  main = 0
```

```text
cinch: 3:10: `repr(align(4))` only raises: A's natural alignment is 8 (attribute annotations specification section 4.2)
```

Individual fields of a `#[repr(C)]` record can also raise their placement alignment, either on the field declaration or prefixed to the field's type:

```cinch
#[repr(C)]
record AlignedField where
  first :: U8
  #[repr(align(8))] second :: U32

#[repr(C)]
record PrefixField where
  first :: U8
  second :: #[repr(align(8))] U32
```

Both compute size 16, alignment 8, with `second` at offset 8 (retaining natural alignment 4). Field-level alignment obeys three checks:

1. It must strictly raise the field's natural alignment:

<!-- cinch:reject contains="on field a only raises" -->
```cinch
module Main where
  #[repr(C)]
  record F where
    #[repr(align(2))] a :: U32

  main :: Int
  main = 0
```

2. The enclosing record must be `#[repr(C)]`:

<!-- cinch:reject contains="applies to fields of a `repr(C)` record" -->
```cinch
module Main where
  record G where
    #[repr(align(8))] a :: U32

  main :: Int
  main = 0
```

3. It cannot annotate an `erased` field:

<!-- cinch:reject contains="is erased and has no representation" -->
```cinch
module Main where
  #[repr(C)]
  record Tagged where
    #[repr(align(8))] erased ok :: Eq 3 3
    val :: U32

  main :: Int
  main = 0
```

Without the attribute on `ok`, the erased field consumes zero bytes and `val` is placed at offset 0 with `U32`'s size and alignment (4).

# Packed layout (`packed` and `packed(N)`)

`#[repr(C, packed(N))]` caps each field's effective placement alignment at `N` (a nonzero power of two). Bare `packed` is `packed(1)`. On `Header`, `#[repr(C, packed(2))]` yields size **8** and alignment **2**:

| Field | Offset | Size | Natural align | Placement align |
| --- | --- | --- | --- | --- |
| `kind :: U8` | 0 | 1 | 1 | 1 |
| `length :: U32` | 2 | 4 | 4 | 2 |
| `flags :: U16` | 6 | 2 | 2 | 2 |

Byte 1 is still padding: `packed(N)` is an **alignment cap**, not a blanket removal of padding. With `packed(1)`, the offsets become **0, 1, 5**, size **7**, alignment **1**.

Aggregate `packed` and `align` conflict and cannot be combined on the same type:

<!-- cinch:reject contains="they conflict" -->
```cinch
module Main where
  #[repr(C, packed, align(4))]
  record P where
    a :: U8

  main :: Int
  main = 0
```

Field-level `#[repr(packed(N))]` is permitted on fields of a `#[repr(C)]` record and caps that field's placement alignment without changing its recorded natural alignment.

# Copying versus borrowing packed fields

Because a packed field's offset may not satisfy its type's natural alignment, taking a `Ref` or `Mut` borrow to an unaligned packed field is rejected at compile time (regardless of region lifetime):

<!-- cinch:reject contains="cannot be borrowed" -->
```cinch
module Main where
  #[repr(packed)]
  record Packed where
    first :: U8
    number :: U32

  use-ref :: forall region. Ref region U32 -> Int
  use-ref _ = 0

  borrow-number :: Packed -> Int
  borrow-number value = use-ref (borrow value.number)

  main :: Int
  main = 0
```

Project the field by value instead. The compiler generates byte-exact loads and stores at each field's recorded offset and width rather than wide unaligned word loads across padding (for `Packed2`, one `load i8`, one `load i32`, and one `load i16`).

# Transparent wrappers (`#[repr(transparent)]`)

`#[repr(transparent)]` gives a record or single-constructor `data` type the exact C layout and ABI of its single non-erased field (any number of `erased` witness fields are allowed alongside it):

```cinch
#[repr(transparent)]
record CounterWord where
  value :: U32
```

`CounterWord` has size 4, alignment 4, and `value` at offset 0. Three restrictions apply:

1. `transparent` cannot combine with any other `repr` variant:

<!-- cinch:reject contains="combines with no other representation" -->
```cinch
module Main where
  #[repr(C, transparent)]
  record One where
    a :: U32

  main :: Int
  main = 0
```

2. Its field cannot carry a `#[repr(...)]` modifier:

<!-- cinch:reject contains="applies to fields of a `repr(C)` record" -->
```cinch
module Main where
  #[repr(transparent)]
  record Two where
    #[repr(align(8))] a :: U32

  main :: Int
  main = 0
```

3. It must have exactly one non-erased represented field:

<!-- cinch:reject contains="needs exactly one represented field" -->
```cinch
module Main where
  #[repr(transparent)]
  record Two where
    a :: U32
    b :: U32

  main :: Int
  main = 0
```

At a foreign boundary, a transparent wrapper over a scalar passes as that scalar directly. For `foreign import ccall "fix_word" readWord :: CounterWord ->{Foreign Fix, IO} I64`, the generated wrapper truncates and calls the C symbol with `i32`:

```text
define i64 @"Main.readWord"(i64 %p0) #0 {
entry:
  %t0 = trunc i64 %p0 to i32
  %t1 = call i64 @"fix_word"(i32 noundef %t0)
  %t2 = freeze i64 %t1
  ret i64 %t2
}
```

`#[repr(transparent)]` governs foreign ABI layout; for allocation-free native single-field wrappers, see [Unboxed wrappers](#unboxed-wrappers).

# Fixed-discriminant `data` (`u8`, `u16`, `u32`, `u64`)

`#[repr(u8)]`, `#[repr(u16)]`, `#[repr(u32)]`, and `#[repr(u64)]` record a fixed discriminant width on a `data` declaration. They are rejected on `record` types, which have no constructor discriminant:

<!-- cinch:reject contains="has no constructor discriminant" -->
```cinch
module Main where
  #[repr(u8)]
  record R where
    a :: U8

  main :: Int
  main = 0
```

Only one discriminant width may be specified, and it cannot combine with `C`, `align`, `packed`, or `transparent`. Today, the compiler records the discriminant width in the `.cinchif` interface while keeping the value's native `uniform-word` representation; fixed-discriminant `data` types do not yet cross a C FFI boundary directly:

<!-- cinch:reject contains="has no C representation at the boundary" -->
```cinch
module Main where
  #[repr(u8)]
  data Color where
    Red :: Color
    Blue :: Color

  foreign import ccall "fix_color" color :: Color ->{Foreign Fix, IO} I64

  main :: Int
  main = 0
```

<span class="support-tag support-tag--planned">Coming soon</span> Compact tagged-union lowering and C enum/union boundary marshaling for fixed-discriminant `data` types are planned. Until then, pass an explicit integer scalar or use a small C shim.

# Foreign calls and layout

Foreign boundaries admit machine scalars, explicit pointer/ownership roles, opaque handles, and explicitly laid-out aggregates (`#[repr(C)]`, `#[repr(packed)]`, `#[repr(transparent)]`). Ordinary `uniform-word` values, closures, dictionaries, dependent pairs, and unresolved generics have no automatic C ABI.

By default, an admitted aggregate parameter crosses via a pointer to a call-scoped stack copy. For a `#[repr(C)]` record of two `U32` fields:

```text
  %t0 = alloca [8 x i8], align 4
  call void @llvm.memset.p0.i64(ptr %t0, i8 0, i64 8, i1 false)
  ...
  %t8 = call i64 @"sum_big"(ptr nonnull align 4 %t0)
```

Use `#[byval]` to pass an aggregate parameter by value according to the target's C ABI (for example, an 8-byte struct in one integer register on x86-64 and arm64):

```text
  %t8 = load i64, ptr %t0, align 8
  %t9 = call i64 @"sum_big"(i64 %t8)
```

Use `#[sret]` to return an aggregate by value. Small structs return in registers; larger structs (such as 24 bytes of `I64` fields) use an LLVM `sret(<layout>)` out-buffer with a `void` return, and large `#[byval]` parameters use `byval(<layout>)` on x86-64 or an indirect copy pointer on arm64 (see [aggregate crossings](15-ffi-layout-and-attributes.md#aggregate-crossings-byval-and-sret)).

Both `#[byval]` and `#[sret]` are validated against the foreign signature, and returning an aggregate without `#[sret]` is rejected:

<!-- cinch:reject contains="marshals no aggregate parameter" -->
```cinch
module Main where
  #[byval]
  foreign import ccall "fix" f :: I64 ->{Foreign Fix, IO} I64

  main :: Int
  main = 0
```

<!-- cinch:reject contains="result is not an aggregate" -->
```cinch
module Main where
  #[sret]
  foreign import ccall "fix" g :: I64 ->{Foreign Fix, IO} I64

  main :: Int
  main = 0
```

<!-- cinch:reject contains="cannot be an aggregate" -->
```cinch
module Main where
  #[repr(C)]
  record P where
    a :: I32

  foreign import ccall "fix_make" make :: I64 ->{Foreign Fix, Diverge} P

  main :: Int
  main = 0
```

To return an aggregate from C, annotate the import with `#[sret]`, write through an `Out` parameter, or return an `Owned` pointer. See [foreign aggregate crossings](23-attributes-and-pragmas.md#foreign-aggregate-crossing-byval-and-sret) and [FFI and primitives](15-ffi-layout-and-attributes.md) for the complete boundary contract.

Note that a boxed field inside a `#[repr(C)]` record crosses as a single pointer word—it is not recursively flattened into a C struct. Ownership transfer, allocator identity, callbacks, and no-unwind guarantees remain explicit obligations on the foreign declaration.

# Runtime headers, constructor tags, and pointer maps

Every heap-allocated, stack-frame, and static native block places an 8-byte self-describing header word at byte offset **-8** from its payload pointer (ABI 10; see [How memory layout works](31-memory-layout.md#heap-block-layout-and-self-describing-headers-abi-10)):

| Bit range | Width | Field | Meaning |
| --- | --- | --- | --- |
| `0..35` | 35 bits | `rc` | Signed reference count (`0` = immortal/static; `> 0` = single-threaded; `-n` = atomic shared count; `2^34 - 1` / `2^34 - 2` = live/released stack-frame block) |
| `35..40` | 5 bits | `kind` | Block kind (constructor, closure, dictionary, bytes, bignum, array, concurrency cell, etc.) |
| `40..64` | 24 bits | `layout` | Self-describing shape for constructors/closures/dictionaries (`8`-bit tag `0..254` + `16`-bit counted field mask, or `255` escaped to a static descriptor at offset `-16`), or word count for unshaped kinds |

Shaped blocks whose tag is `>= 255`, whose counted field sits at index `>= 16`, or whose payload exceeds 31 words store `255` (`SHAPE_ESCAPED`) in the tag bits and place a pointer to a static descriptor one word earlier at offset **-16**.

Constructor tags live in bits `40..48` of the header rather than in a leading payload word, and counted fields are described directly by the header's mask (or static descriptor) with no global layout table. Runtime headers and descriptors are internal ABI details; user and foreign code must never inspect or persist them directly.

# Unboxed wrappers

`#[repr(unboxed)]` gives a single-constructor `data` or `record` type the exact native runtime representation of its one non-erased field, eliminating wrapper allocation:

```cinch
#[repr(unboxed)]
data Count where
  CountOf :: U32 -> Count

countValue :: Count -> U32
countValue (CountOf n) = n
```

`Count` remains a distinct nominal type for type checking, instance resolution, and proofs (`Show Count` does not inherit `Show U32`, and propositions over `CountOf n` keep the constructor).

Supported inner payloads are machine scalars up to 64 bits, `Bool`, `Unit`, `Int`, `Nat`, `Bytes`, `Sym`, and other admitted unboxed wrappers.

## Unpacking a record field

On a monomorphic native record, `#[unpack]` inlines a nested product's represented fields directly into the enclosing record in declaration order:

```cinch
data Point where
  XY :: I64 -> I64 -> Point

record Sample where
  #[unpack] position :: Point
  reading :: I64
```

`Sample` occupies three words instead of a pointer plus a separate heap `Point`. Source types, constructor arguments, patterns, and projections are unchanged (projecting the whole `sample.position` value allocates a fresh `Point` box if needed). Use `#[nounpack]` to keep a field boxed explicitly.

`#[unpack]` requires a monomorphic `data` or `record` field with one ordinary constructor and at least one runtime field. Explicit C boundary layouts and products that themselves contain unpacked fields are not supported targets for `#[unpack]`.

## Constructed Sum Results (CSR)

Eligible two-variant sums whose variants each carry zero to two uniform-word payloads (`Option a`, `Result e a`, and user-defined two-constructor `data` types) are returned unboxed in a `{ i8, i64 }` or `{ i8, i64, i64 }` register aggregate (`{ tag, payload... }`) across direct `case`-scrutinized calls and recursive loops, avoiding heap allocation on every step.

- **Exact reference counting:** when a caller matches an unboxed CSR return—including wildcard branches that discard a counted payload—the compiler inserts exact Perceus retain/release operations on the payload registers without boxing the outer constructor.
- **Escaping and higher-order uses:** when a CSR-eligible function is passed as a first-class closure or its result escapes into a heap container, its standard C-ABI wrapper boxes the `{ tag, payload... }` aggregate transparently.
- **Inspecting the ABI:** `cinch emit-ir` shows `{ i8, i64 }` or `{ i8, i64, i64 }` worker signatures (`#cc` in the Rust seed, `#csr` in the self-hosted compiler) alongside the boxed wrapper.

See [rewrites and optimization](12-rewrites-and-optimization.md#constructed-product-and-sum-results) for how CSR complements Constructed Product Results (CPR) and fusion.

# Summary of rejected combinations

| Form | Diagnostic |
| --- | --- |
| `#[repr(unboxed)]` on a sum or multi-field product | `requires one constructor with exactly one represented field` |
| `#[repr()]` | `` `#[repr(...)]` takes at least one representation `` |
| `#[repr=C]` | `` `#[repr(...)]` takes its representations in parentheses `` |
| `#[repr = C]` | `unterminated attribute` |
| `repr(align(0))`, `repr(align(3))` | `` `repr(align(N))` takes a non-zero power of two `` |
| `repr(align(N))` below natural alignment | `only raises` |
| `repr(packed, align(N))` | `they conflict` |
| `repr(transparent, ...)` | `combines with no other representation` |
| `repr(u8, C)` | `combines with no other representation here` |
| `repr(C)` on a multi-constructor `data` | `has no field layout to compute` |
| `repr(u8)` on a `record` | `has no constructor discriminant` |
| Field `repr` outside `#[repr(C)]` | `` applies to fields of a `repr(C)` record `` |
| Field `repr` on an `erased` field | `is erased and has no representation` |
| `#[repr(C)]` with a type-parameter field | `this field's type is not fixed` |
| Recursive explicit layout | `has an infinite explicit layout` |
| Duplicate `repr` option | `` is written twice in one `#[repr(...)]` `` |

# Planned representation facilities

<span class="support-tag support-tag--planned">Coming soon</span> Further representation controls are planned and not yet implemented in the compiler:

| Planned facility | Intended behaviour | Current status |
| --- | --- | --- |
| Expanding-edge boxing | Bound full-layout specialization on layout-growing recursive types | Finite `Raw`/`Obj`/`TObj`/`Wide`/`Val`/`PolyNiche` class specialization |
| Representation instances (`Represent t carrier`) | Custom carriers with builder/view adapters | Not implemented |
| Named representation selection (`List Bool via Dense`) | Explicit type-indexed alternative carriers | Not implemented |

Planned allocator and budget attribute spellings are indexed in [attributes and pragmas](23-attributes-and-pragmas.md#planned-and-provisional-attribute-spellings).
