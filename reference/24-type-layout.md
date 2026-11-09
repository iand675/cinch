# Type layout and runtime representation

A source type, a native value word, an aggregate's byte layout, and a C calling convention are related but distinct. This chapter records what the Rust compiler currently computes for each of them, shows the command that prints those facts, and labels the selected future representation work. All `#[repr(...)]` variants are indexed in [attributes and pragmas](23-attributes-and-pragmas.md#repr-every-current-variant).

# What layout describes

An explicit aggregate layout records:

- Size and aggregate alignment, in bytes.
- Each represented field's offset, size, and natural alignment.
- Source representation spelling and any recorded discriminant width.
- The relevant type/ABI identity used by interfaces and boundary marshaling.

Offsets in an aggregate's layout describe its payload or marshaled representation. They do not include the runtime allocation header. A value carried in a native function's word slot can be a pointer to that payload rather than the payload bytes themselves.

Do not infer persistence or wire compatibility from the fact that a type happens to work in a program. Use an explicit admitted layout for a boundary, inspect its recorded facts, and use a separate codec for a portable file or network format. `repr(C)` follows the supported target ABI; it is not an endian-neutral serialization format.

# Inspect the compiler's layout facts

`cinch check` writes one interface file per module under `build/`, and the entry module's `.cinchif` carries an `abi` section with the recorded layouts:

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

For a module holding one `#[repr(C)]` record, one packed record, one transparent wrapper, one fixed-discriminant `data` and one unannotated record:

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

the script prints, with field lines indented under their type:

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

Three facts follow from that output. The field `align` is the field type's **natural** alignment even when placement capped or raised it (`length` at offset 2 keeps natural alignment 4). A fixed discriminant is a recorded width on a value that stays one word. An unannotated record has the same size and alignment as any other word and no field facts at all.

This JSON workflow is an implementation artifact, not a `sizeof`/`alignof` source intrinsic or a stable promise about every future schema key. The current compiler exposes no general source-level layout-reflection API merely because the interface carries the facts.

# Default layout

Without an explicit layout recipe the recorded layout is `uniform-word` with size and alignment 8, as `Plain` shows above. Constructor products remain object-backed with runtime-managed payloads. That is not an assurance that a particular source record's C-compatible inline fields occupy eight bytes in total.

The current foreign checker refuses an ordinary `uniform-word` aggregate at a C boundary:

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

The same checker refuses to fix a layout for an unresolved generic field:

<!-- cinch:reject contains="not fixed" -->
```cinch
module Main where
  #[repr(C)]
  record Held a where
    value :: a

  main :: Int
  main = 0
```

<span class="support-tag support-tag--planned">Coming soon</span> the broader compiler-chosen default-layout design includes deterministic field reordering and unpacking choices recorded in interfaces. Do not apply that future freedom as a description of today's explicit source-order layout algorithm.

# C layout: source order, padding, and alignment

`#[repr(C)]` uses represented fields in source order. For each field:

1. Obtain its size and natural alignment from the scalar type or an admitted nested layout.
2. Apply a field alignment floor, if present, then the applicable packing cap.
3. Round the current offset **up** to that effective placement alignment.
4. Place the field and advance by its size.
5. Set aggregate alignment to the maximum effective field alignment, raised by an admitted aggregate alignment floor, and round the final size up to it.

For these native scalar fields, U8 has size and alignment 1, U16 has 2, U32 has 4, and U64 has 8. Bool is 1/1; Char is its scalar representation rather than a UTF-8 byte. Target-sized scalars use their admitted profile width. Do not treat mathematical `Int` as `I64`.

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

Its recorded layout is:

| Byte range | Contents | Reason |
| --- | --- | --- |
| 0 | `kind`, 1 byte | U8 needs alignment 1. |
| 1-3 | Padding | The next U32 starts at a multiple of 4. |
| 4-7 | `length`, 4 bytes | U32 natural alignment 4. |
| 8-9 | `flags`, 2 bytes | U16 natural alignment 2. |
| 10-11 | Tail padding | Aggregate size rounds to a multiple of 4. |

Size is **12**, alignment **4**, offsets **0, 4, 8**. Padding bytes are not application fields and must not be assumed to contain stable serialized data.

Fields keep source order. If you write the declaration until the offsets differ, they do:

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

Both records are size 8 with alignment 4, and the field offsets are 0 and 4 in each, in the order written.

Nested explicit layouts resolve recursively. A `#[repr(C)]` record whose field is another `#[repr(C)]` record takes that field's computed size and alignment: an inner record of `U16` then `U8` is size 4 alignment 2, and placing it after a `U8` field puts it at offset 2 in a 12-byte outer record.

C-style field layouts apply to admitted one-constructor products. A multi-constructor `data` does not gain a tagged C union layout through `repr(C)`:

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

A single-constructor `data` does get a field layout through `repr(C)`; its fields are recorded as `$f0`, `$f1`, and so on. Explicit by-value layouts must have fixed fields: a generic type parameter with unknown layout is rejected, and an infinite recursive explicit-layout dependency is rejected:

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

# Current native value representations

Current native lowering carries values in uniform 64-bit word slots and attaches representation classes before reclamation and specialization:

| Class | Meaning | Examples |
| --- | --- | --- |
| `Raw` | Never reference-count as an object pointer. | Machine scalars, Bool, Char, Unit, Sym handles, opaque/runtime handles and capabilities. |
| `Obj` | Counted pointer to a heap or static object. | Constructor products, tuples, closures, dictionaries, bytes/text, memo cells, quotes. |
| `TObj` | Tagged small immediate or object pointer. | Mathematical Nat and Int. |
| Parameter class | Temporary class of a generic type parameter before instantiation. | Resolved to a reached concrete class vector before emission. |

This classification controls retains and releases and the pointer maps; it is not the same as choosing a packed C struct. The Rust implementation specializes reached generic units by representation-class vector, naming each copy `unit@RO`, `unit@OO` and so on after the vector's letters. The planned multiword and ad hoc representation system extends beyond that implemented finite class specialization.

Native small `Nat` values from 0 through `2^63 - 1` and small `Int` values from `-2^62` through `2^62 - 1` fit the immediate form: the word is `n << 1 | 1`, so its low bit distinguishes it from an aligned block pointer. Values outside those ranges use bignum objects. These are representation thresholds, not the mathematical types' numeric limits. Machine integer arithmetic keeps its explicitly selected overflow policy.

The encoding is visible in emitted IR. For a program that passes `42 :: Nat` to a constructor and a large `Nat` through a binding:

```text
  %t0 = call i64 @"Main.big"()
  %t1 = call i64 @"Main.Pair"(i64 85, i64 %t0)
```

`85` is `42 << 1 | 1`. The large value arrives as a word from `Main.big`, which boxes it.

Refinement witnesses are Prop-erased: `NonZero Int` carries the same runtime number as its carrier, and `unrefine` is an identity projection. That transparency does not let callers fabricate evidence. An `erased` field has no runtime slot at all, and the layout calculation skips it:

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

`Unit` has zero byte size and alignment 1 in the explicit aggregate field-layout calculation, even though native calls use the compiler's uniform-word conventions.

# Raising alignment

`#[repr(C, align(16))]` on `Header` keeps the field offsets 0, 4 and 8, raises aggregate alignment to 16, and rounds the size to 16. The recorded layout is `C, align(16)` with size 16 and alignment 16. The argument is a nonzero power of two:

<!-- cinch:reject contains="non-zero power of two" -->
```cinch
module Main where
  #[repr(C, align(3))]
  record Bad where
    a :: U8

  main :: Int
  main = 0
```

Asking for an alignment below the relevant natural alignment is an error, rather than a request to misalign the object:

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

A field can independently request a stronger alignment in a `repr(C)` record:

```cinch
#[repr(C)]
record AlignedField where
  first :: U8
  #[repr(align(8))] second :: U32
```

The recorded layout is size 16, alignment 8, with `second` at offset 8 and natural alignment 4. The modifier may instead prefix the field's type, and both spellings compute the same layout:

```cinch
#[repr(C)]
record PrefixField where
  first :: U8
  second :: #[repr(align(8))] U32
```

It records size 16, alignment 8, `second` at offset 8, identical to the attribute-on-declaration form. A field modifier whose alignment does not raise the field's own alignment is refused:

<!-- cinch:reject contains="on field a only raises" -->
```cinch
module Main where
  #[repr(C)]
  record F where
    #[repr(align(2))] a :: U32

  main :: Int
  main = 0
```

Field modifiers require the containing record's `C` layout:

<!-- cinch:reject contains="applies to fields of a `repr(C)` record" -->
```cinch
module Main where
  record G where
    #[repr(align(8))] a :: U32

  main :: Int
  main = 0
```

An erased field has no represented location to align:

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

Without the modifier, the same record records size 8 and alignment 8 with `val` at offset 0: the erased field took no bytes and no layout modifier.

# Packed layout

`#[repr(C, packed(2))]` on the same three-field Header caps placement alignment at 2:

| Field | Offset | Size | Natural alignment | Effective placement alignment |
| --- | --- | --- | --- | --- |
| `kind :: U8` | 0 | 1 | 1 | 1 |
| `length :: U32` | 2 | 4 | 4 | 2 |
| `flags :: U16` | 6 | 2 | 2 | 2 |

Size is **8**, aggregate alignment **2**. Byte 1 remains padding. Packing is an **alignment cap**, not a rule that every padding byte disappears. Bare `packed` means `packed(1)`; the same fields then occupy offsets **0, 1, 5**, size **7**, alignment **1**.

The metadata's field alignment is the field's natural alignment, not the capped placement alignment. In the `packed(2)` example the U32 field is naturally aligned to 4 but placed at offset 2. That distinction is what lets the checker detect unsafe borrows.

`packed` and aggregate `align` conflict, and both arguments must be nonzero powers of two:

<!-- cinch:reject contains="they conflict" -->
```cinch
module Main where
  #[repr(C, packed, align(4))]
  record P where
    a :: U8

  main :: Int
  main = 0
```

A field-level packing cap is admitted on a `repr(C)` record and participates in the effective placement calculation; it does not change the field type's natural alignment.

# Copying versus borrowing packed fields

A field projection can copy an unaligned packed value through the admitted byte-exact load and marshal path. A `Ref` or `Mut` claims suitably aligned access. Introducing such a borrow to an unaligned packed field is rejected; merely choosing a shorter-lived region does not repair its alignment.

<!-- cinch:reject contains="cannot be borrowed" -->
```cinch
module Main where
  #[repr(packed)]
  record Packed where
    first :: U8
    number :: U32

  borrow-number :: forall region. Packed -> Ref region U32
  borrow-number value = borrow value.number

  main :: Int
  main = 0
```

Use an owned copy when that is what the operation needs. Do not cast an address to a borrow and assume that the type annotation makes it aligned. Foreign pointer access still needs its separate ownership, bounds, initialization, and lifetime contract.

The boundary marshal reads each packed field at its recorded offset with its recorded width, never a word-width load across the padding. For a `#[repr(C, packed(2))]` record of `U8`, `U32`, `U16`, the emitted wrapper contains one `load i8`, one `load i32` and one `load i16`; the only `load i64` in it is the block header read at offset -8.

# Transparent wrappers

`#[repr(transparent)]` requires exactly **one non-erased represented field**, so an `erased` witness beside it does not break the rule. The recorded field offset is 0, and the aggregate has that field's size and alignment.

```cinch
#[repr(transparent)]
record CounterWord where
  value :: U32
```

The recorded layout is size 4, alignment 4, field offset 0. It combines with no other repr variant:

<!-- cinch:reject contains="combines with no other representation" -->
```cinch
module Main where
  #[repr(C, transparent)]
  record One where
    a :: U32

  main :: Int
  main = 0
```

It does not admit a field layout modifier. A modifier on its field is refused by the general rule that field representations belong to `repr(C)` records:

<!-- cinch:reject contains="applies to fields of a `repr(C)` record" -->
```cinch
module Main where
  #[repr(transparent)]
  record Two where
    #[repr(align(8))] a :: U32

  main :: Int
  main = 0
```

Exactly one represented field must remain:

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

A transparent wrapper over a scalar crosses the boundary as that scalar. For `#[repr(transparent)] record Word where value :: U32`, the emitted wrapper for `foreign import ccall "fix_word" readWord :: Word ->{Foreign Fix, IO} I64` is:

```text
define i64 @"Main.readWord"(i64 %p0) #0 {
entry:
  %t0 = trunc i64 %p0 to i32
  %t1 = call i64 @"fix_word"(i32 noundef %t0)
  %t2 = freeze i64 %t1
  ret i64 %t2
}
```

That is the admitted scalar-like crossing through the field representation. It does not imply that every native use of a source wrapper is allocation-free, and it is not the future inline value type feature. Native object slots and boundary layout are different surfaces.

# Data tags and enum-like declarations

`#[repr(u8)]`, `u16`, `u32` or `u64` on `data` records a fixed discriminant width. A record has no alternative-constructor discriminant:

<!-- cinch:reject contains="has no constructor discriminant" -->
```cinch
module Main where
  #[repr(u8)]
  record R where
    a :: U8

  main :: Int
  main = 0
```

Only one discriminant choice is permitted, and it cannot combine with `C`, alignment, or packing. The recorded layout keeps the uniform word: `Color` above is size 8, alignment 8, discriminant 1. The implementation records the width in the interface while retaining the native uniform-word representation. It does not lay out a complete tagged union payload, guarantee that the whole value occupies one byte, or admit arbitrary fixed-discriminant data through the C boundary:

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

<span class="support-tag support-tag--planned">Coming soon</span> complete fixed-tag and payload lowering beyond the current recorded fact belongs to the wider representation work. Until then, use explicitly supported boundary data or a C shim rather than guessing a C enum or union representation.

# Foreign calls and layout

The boundary admits machine scalars, explicit pointer and ownership roles, opaque handles, and supported explicitly laid-out aggregates. Ordinary boxed values, language-native closures, dictionaries, vectors, dependent pairs, proofs, and unresolved generic layouts have no automatic C ABI.

A default admitted aggregate parameter crosses through a pointer to a call-scoped layout copy. For a `#[repr(C)]` record of two `U32` fields:

```text
  %t0 = alloca [8 x i8], align 4
  call void @llvm.memset.p0.i64(ptr %t0, i8 0, i64 8, i1 false)
  ...
  %t8 = call i64 @"sum_big"(ptr nonnull align 4 %t0)
```

`#[byval]` selects the explicitly represented aggregate-copy convention and states the pointee layout without changing the machine convention:

```text
  %t8 = call i64 @"sum_big"(ptr byref(%__sl_layout_0) align 4 nonnull align 4 %t0)
```

An aggregate result needs the admitted `#[sret]` result-out crossing, which passes a buffer and takes a void result:

```text
  %t0 = alloca [8 x i8], align 4
  call void @"make_big"(ptr sret(%__sl_layout_0) align 4 %t0, i64 noundef %p0)
```

The three attributes are checked against the binding they annotate:

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

The last diagnostic names the repairs: return through an `Out` cell or an `Owned` pointer, or spell the by-value crossing `#[sret]`.

See [byval and sret](23-attributes-and-pragmas.md#foreign-aggregate-crossing-byval-and-sret) for the attribute contracts and [FFI and primitives](15-ffi-layout-and-attributes.md) for the rest of the boundary.

A field whose type is a boxed value crosses as one word, a pointer to the object payload. It is not recursively marshaled into a portable C struct merely because its enclosing type uses `C`. The marshal reads such a field with `load ptr` at its recorded offset and stores the word into the layout copy, so the C side receives a pointer whose target layout the declaration does not describe. Boundary admission validates the complete actual field and role plan, and ownership transfer, allocator identity, callbacks, blocking and no-unwind behaviour remain separate foreign obligations.

# Runtime headers, constructor tags, and pointer maps

Current native heap and static blocks carry a runtime header word at offset **-8** from their payload pointer, encoding a 40-bit signed count (low bits), an 8-bit kind, and a 16-bit layout id. The constants in `compiler/runtime/src/block_kinds.rs` derive every shift and mask from those widths. A payload of 65535 words or more stores the escape value in the layout field and keeps the real size or descriptor one word earlier, at offset -16. These are runtime ABI details, not leading fields a C declaration should imitate.

Generated code reads that header through a negative index, and reads a constructor's tag out of the layout id. A `case` over a two-field constructor compiles to:

```text
  %t6 = inttoptr i64 %t5 to ptr
  %t8 = getelementptr inbounds i8, ptr %t6, i64 -8
  %t9 = load atomic i64, ptr %t8 monotonic, align 8
  %t10 = lshr i64 %t9, 48
  %t11 = and i64 %t10, 63
  switch i64 %t11, label %bb2 [
  i64 0, label %bb3
]
```

48 is the count plus kind width, and 63 selects the tag from the layout id, whose high bits name the data type. Constructor tags are therefore encoded through layout identifiers and descriptors, not through a universally present first payload tag word. Pointer maps tell reclamation which fields can contain objects and which are raw or tagged immediates. Closures and dictionaries have their own shapes.

Runtime header formats and specialization symbols are versioned implementation facts. User code should not inspect them by integer or pointer casts or persist them as a file format. Their source authority is `compiler/runtime/src/block_kinds.rs` and `block.rs`, with the corresponding code generation, not a source `repr` promise.

# Diagnostics for unsupported combinations

| Written | Refusal |
| --- | --- |
| `#[repr(unboxed)]` | `` `unboxed` is not a representation `` |
| `#[repr()]` | `` `#[repr(...)]` takes at least one representation `` |
| `#[repr=C]` | `` `#[repr(...)]` takes its representations in parentheses `` |
| `#[repr = C]` | `` unterminated attribute `` (the space ends the attribute name) |
| `repr(align(0))`, `repr(align(3))` | `` `repr(align(N))` takes a non-zero power of two `` |
| `repr(align(N))` on a field or type | `` only raises `` |
| `repr(packed, align(N))` | `` they conflict `` |
| `repr(transparent, ...)` | `` combines with no other representation `` |
| `repr(u8, C)` | `` combines with no other representation here `` |
| `repr(C)` on a multi-constructor `data` | `` has no field layout to compute `` |
| `repr(u8)` on a `record` | `` has no constructor discriminant `` |
| field modifier outside `repr(C)` | `` applies to fields of a `repr(C)` record `` |
| modifier on an `erased` field | `` is erased and has no representation `` |
| `repr(C)` with a type-parameter field | `` this field's type is not fixed `` |
| recursive explicit layout | `` has an infinite explicit layout `` |
| one repr name twice | `` is written twice in one `#[repr(...)]` `` |

# Intended representation system

<span class="support-tag support-tag--planned">Coming soon</span> the selected [layout and specialization design](../design/layouts-and-specialization.md) adds several distinct capabilities. Their proposed syntax is illustrative until implemented:

| Planned facility | Intended guarantee or behaviour | Current boundary |
| --- | --- | --- |
| Inline value types, proposed `#[repr(unboxed)]` | An admitted fixed layout is stored inline wherever used. | `unboxed` is not an accepted repr variant. |
| Field unpacking, `#[unpack]` / `#[nounpack]` | Request or override inlining a fixed field's representation into its parent. | Neither is an accepted compiler-owned attribute. |
| Compiler-chosen default multiword layouts | Versioned deterministic field-layout choices recorded for consumers. | The current default remains the native uniform-word and object model described above. |
| Expanding-edge boxing | Bound demand-driven full-layout specialization by boxing layout-growing recursive edges. | Current finite Raw/Obj/TObj class specialization does not establish that future full-layout algorithm. |
| Representation instances, `Represent t carrier` | Builders and views relate an alternative carrier to canonical constructors, with optional fast paths. | No general shipped representation-instance API. |
| Named representation selection, `List Bool via Dense` | Preserve a representation index through types, with explicit conversion between representations. | `via` and scoped representation defaults are planned syntax, not current implicit conversions. |

In the intended model, generic functions keep their source interface while constructors and matches use the selected representation's builders and view. The representation must preserve the required observations and borrowing rules; a bit-packed Boolean representation cannot silently break generic code that needs a real field borrow. A view can also change cost: repeated cons on a packed array may require copying, so a representation is not a universal performance improvement.

The value-type spelling and some admission choices remain provisional. Keep them visibly planned rather than registering an inert metadata attribute that pretends to implement the storage change. Planned allocator selection and in-place or budget attributes are indexed in [all attributes](23-attributes-and-pragmas.md#planned-and-provisional-attribute-spellings).

Implementation authority: `compiler/src/check_decls.rs::parse_repr_args`, `compute_layouts`, `layout_of`, `build_layout`, `field_size_align`, `abi_param`; `compiler/src/interface.rs` for the `AbiLayout` record; `compiler/src/repr.rs`, `specialize.rs`, `codegen.rs`; runtime `block_kinds.rs`, `block.rs`, `rc.rs`. Behaviour suites: `compiler/tests/attributes.rs`, `ffi.rs`, `llvm_contract.rs`, `repr_classes.rs`, `reclamation.rs`.
