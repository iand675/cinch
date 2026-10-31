# Type layout and runtime representation

A source type, a native value word, an aggregate's byte layout, and a C calling convention are related but distinct. This chapter describes each current Rust compiler surface and labels the selected future representation work. All `#[repr(...)]` variants are indexed in [attributes and pragmas](23-attributes-and-pragmas.md#repr-every-current-variant).

# What layout describes

An explicit aggregate layout records:

- Size and aggregate alignment, in bytes.
- Each represented field's offset, size, and natural alignment.
- Source representation spelling and any recorded discriminant width.
- The relevant type/ABI identity used by interfaces and boundary marshaling.

Offsets in an aggregate's layout describe its payload or marshaled representation. They do not include the runtime allocation header. A value carried in a native function's word slot can be a pointer to that payload rather than the payload bytes themselves.

Do not infer persistence/wire compatibility from the fact that a type happens to work in a program. Use an explicit admitted layout for a boundary, inspect its recorded facts, and use a separate codec for a portable file or network format. `repr(C)` follows the supported target ABI; it is not an endian-neutral serialization format.

# Current native value representations

Current native lowering carries values in uniform 64-bit word slots and attaches representation classes before reclamation/specialization:

| Class | Meaning | Examples |
| --- | --- | --- |
| `Raw` | Never reference-count as an object pointer. | Machine scalars, Bool, Char, Unit, Sym handles, opaque/runtime handles and capabilities. |
| `Obj` | Counted pointer to a heap or static object. | Constructor products, tuples, closures, dictionaries, bytes/text, memo cells, quotes. |
| `TObj` | Tagged small immediate or object pointer. | Mathematical Nat and Int. |
| Parameter class | Temporary class of a generic type parameter before instantiation. | Resolved to a reached concrete class vector before emission. |

This classification controls retains/releases and pointer maps; it is not the same as choosing a packed C struct. The Rust implementation already specializes reached generic units by representation-class vector. The planned multiword/ad hoc representation system extends beyond that implemented finite class specialization.

Native small `Nat` values from 0 through `2^63 - 1` and small `Int` values from `-2^62` through `2^62 - 1` fit the immediate form. Values outside those ranges use bignum objects. These are representation thresholds, not the mathematical types' numeric limits. Machine integer arithmetic keeps its explicitly selected overflow policy.

Refinement witnesses are Prop-erased: `NonZero Int` carries the same runtime number as its carrier, and `unrefine` is an identity projection. That transparency does not let callers fabricate evidence. Erased fields have no runtime slot. `Unit` has zero byte size and alignment 1 in the explicit aggregate field-layout calculation, even though native calls use the compiler's uniform-word conventions.

# Default layout

Without an explicit layout recipe, the ABI layout description is `uniform-word`, size/alignment 8 in the current native implementation. Constructor products remain object-backed with runtime-managed payloads. This is not an assurance that a particular source record's C-compatible inline fields occupy eight bytes in total.

The current foreign checker refuses an ordinary `uniform-word` aggregate at a C boundary. It also refuses to guess a layout for unresolved generic fields. A source-level type name is not sufficient evidence for an external layout.

<span class="support-tag support-tag--planned">Coming soon</span> the broader compiler-chosen default-layout design includes deterministic field reordering/unpacking choices recorded in interfaces. Do not apply that future freedom as a description of today's explicit source-order layout algorithm.

# C layout: source order, padding, and alignment

`#[repr(C)]` uses represented fields in source order. For each field:

1. Obtain its size and natural alignment from the scalar type or an admitted nested layout.
2. Apply a field alignment floor, if present, then the applicable packing cap.
3. Round the current offset **up** to that effective placement alignment.
4. Place the field and advance by its size.
5. Set aggregate alignment to the maximum effective field alignment, raised by an admitted aggregate alignment floor, and round the final size up to it.

For these native scalar fields, U8 has size/alignment 1, U16 has 2, U32 has 4, and U64 has 8. Bool is 1/1; Char is its scalar representation rather than a UTF-8 byte. Target-sized scalars use their admitted profile width. Do not treat mathematical `Int` as `I64`.

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
| 1–3 | Padding | The next U32 starts at a multiple of 4. |
| 4–7 | `length`, 4 bytes | U32 natural alignment 4. |
| 8–9 | `flags`, 2 bytes | U16 natural alignment 2. |
| 10–11 | Tail padding | Aggregate size rounds to a multiple of 4. |

Size is **12**, alignment **4**, offsets **0, 4, 8**. Padding bytes are not application fields and must not be assumed to contain stable serialized data.

C-style field layouts apply to admitted one-constructor products. A multi-constructor `data` does not gain a tagged C union layout through `repr(C)`; the current checker refuses it. Explicit by-value layouts must have fixed fields; a generic type parameter with unknown layout is rejected, and an infinite recursive explicit-layout dependency is rejected.

# Raising alignment

`#[repr(C, align(16))]` on `Header` keeps the field offsets 0, 4, 8, raises aggregate alignment to 16, and rounds size to 16. The argument is a nonzero power of two. Asking for an alignment below the relevant natural alignment is an error, rather than a request to misalign the object.

A field can independently request a stronger alignment in a `repr(C)` record:

```cinch
#[repr(C)]
record AlignedField where
  first :: U8
  #[repr(align(8))] second :: U32
```

`second` starts at offset 8, aggregate alignment becomes 8, and size becomes 16. The natural U32 alignment remains 4 in the field fact; effective placement was raised to 8. The modifier may instead prefix the field's type; both syntactic positions are merged for validation.

Field modifiers require the containing record's `C` layout. They do not make arbitrary generic/default-layout fields independently addressable C storage. An erased field has no represented location to align.

# Packed layout

`#[repr(C, packed(2))]` on the same three-field Header caps placement alignment at 2:

| Field | Offset | Size | Natural alignment | Effective placement alignment |
| --- | --- | --- | --- | --- |
| `kind :: U8` | 0 | 1 | 1 | 1 |
| `length :: U32` | 2 | 4 | 4 | 2 |
| `flags :: U16` | 6 | 2 | 2 | 2 |

Size is **8**, aggregate alignment **2**. Byte 1 remains padding. Packing is an **alignment cap**, not a rule that every padding byte disappears. Bare `packed` means `packed(1)`; the same fields then occupy offsets **0, 1, 5**, size **7**, alignment **1**.

The metadata's field alignment is the field's **natural** alignment, not the capped placement alignment. In the packed(2) example, the U32 field is naturally aligned to 4 but placed at offset 2. This distinction lets the checker detect unsafe borrows.

`packed` and aggregate `align` conflict. Both packing and alignment arguments must be nonzero powers of two. A field-level packing cap is admitted on a `repr(C)` record and participates in the effective placement calculation; it does not change the field type's natural alignment.

# Copying versus borrowing packed fields

A field projection can copy an unaligned packed value through the admitted byte-exact load/marshal path. A `Ref` or `Mut` claims suitably aligned access. Introducing such a borrow to an unaligned packed field is rejected; merely choosing a shorter-lived region does not repair its alignment.

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

# Transparent wrappers

`#[repr(transparent)]` requires exactly **one non-erased represented field**. The recorded field offset is 0, and the aggregate has that field's size and alignment. It combines with no other repr variant; its field takes no layout modifier.

```cinch
#[repr(transparent)]
record CounterWord where
  value :: U32
```

The recorded layout is size 4, alignment 4, field offset 0. It is eligible for the admitted scalar-like foreign crossing through its field representation. This explicit ABI fact does not imply that every native use of a source wrapper is allocation-free or that `repr(transparent)` is the future inline-value-type feature. Native object slots and boundary layout are different surfaces.

# Data tags and enum-like declarations

`#[repr(u8)]`, `u16`, `u32`, or `u64` on `data` records a fixed discriminant width. A record has no alternative-constructor discriminant and rejects these choices. Only one discriminant choice is permitted, and it cannot combine with `C`, alignment, or packing.

The current implementation records that width in the interface **while retaining the uniform-word native representation**. It does not lay out a complete tagged union payload, guarantee that the whole value occupies one byte, or admit arbitrary fixed-discriminant data through the C boundary. The foreign checker explicitly rejects that missing aggregate representation.

<span class="support-tag support-tag--planned">Coming soon</span> complete fixed-tag/payload lowering beyond the current recorded fact belongs to the wider representation work. Until then, use explicitly supported boundary data or a C shim rather than guessing a C enum/union representation.

# Foreign calls and layout

The boundary admits machine scalars, explicit pointer/ownership roles, opaque handles, and supported explicitly laid-out aggregates. Ordinary boxed values, language-native closures/dictionaries/vectors/dependent pairs/proofs, and unresolved generic layouts have no automatic C ABI.

A default admitted aggregate parameter crosses through a pointer to a call-scoped layout copy. `#[byval]` selects the explicitly represented aggregate-copy convention. An aggregate result needs the admitted `#[sret]` result-out crossing; `transparent` over a scalar passes through as that field's representation. See [byval and sret](23-attributes-and-pragmas.md#foreign-aggregate-crossing-byval-and-sret) for the exact current LLVM convention and validation.

Nested layouts are resolved recursively. A field that is a boxed value is not automatically a recursively marshaled portable C struct merely because its enclosing type uses `C`. Boundary admission must validate the complete actual field/role plan. Nullability, allocator identity, ownership transfer, callbacks, blocking, and no-unwind behaviour remain separate foreign obligations.

# Runtime headers, constructor tags, and pointer maps

Current native heap/static blocks carry a runtime header word at offset **−8** from their payload pointer. Its fields record reference-count state, block kind, and a layout identifier. Oversized descriptors can use an additional preceding word. These are runtime ABI details, not leading fields that a C declaration should imitate.

Constructor tags are encoded through layout identifiers/descriptors, rather than a universally present first payload tag word. Pointer maps tell reclamation which fields can contain objects and which are raw or tagged immediates. Closures and dictionaries have their own shapes. An object-backed native `Header` can therefore carry runtime bookkeeping even though its explicit payload layout is the 12-byte table above.

Runtime header formats and specialization symbols are versioned implementation facts. User code should not inspect them by integer/pointer casts or persist them as a file format. Their source authority is `compiler/runtime/src/block_kinds.rs`/`block.rs`, with corresponding code generation, not a source `repr` promise.

# Inspect the compiler's actual layout facts

Check a module to produce its interface, then inspect its ABI section:

```sh
cinch check Main.cinch
python3 - <<'PY'
import json
from pathlib import Path
interface = json.loads(Path("build/Main.cinchif").read_text())
for layout in interface["abi"]["layouts"]:
    print(layout["type"], layout["size"], layout["align"], layout["fields"])
PY
```

The fields describe offsets/sizes/natural alignment. The interface also retains repr/discriminant facts. Layout changes participate in contract identity, so changing packing/alignment is not equivalent to changing an inline hint. Dependents and live state may need rechecking/migration compatibility handling.

This JSON inspection is an implementation artifact workflow, not a `sizeof`/`alignof` source intrinsic or a stable promise about every future schema key. The current compiler does not expose a general source-level layout-reflection API merely because the interface contains the facts.

# Intended representation system

<span class="support-tag support-tag--planned">Coming soon</span> the selected [layout and specialization design](../design/layouts-and-specialization.md) adds several distinct capabilities. Their proposed syntax is illustrative until implemented:

| Planned facility | Intended guarantee / behaviour | Current boundary |
| --- | --- | --- |
| Inline value types, proposed `#[repr(unboxed)]` | An admitted fixed layout is stored inline wherever used. | `unboxed` is not an accepted repr variant. |
| Field unpacking, `#[unpack]` / `#[nounpack]` | Request/override inlining a fixed field's representation into its parent. | Neither is an accepted compiler-owned attribute. |
| Compiler-chosen default multiword layouts | Versioned deterministic field-layout choices recorded for consumers. | The current default remains the native uniform-word/object model described above. |
| Expanding-edge boxing | Bound demand-driven full-layout specialization by boxing layout-growing recursive edges. | Current finite Raw/Obj/TObj class specialization does not establish that future full-layout algorithm. |
| Representation instances, `Represent t carrier` | Builders/views and proofs relate an alternative carrier to canonical constructors, with optional fast paths. | No general shipped representation-instance API. |
| Named representation selection, `List Bool via Dense` | Preserve a representation index through types; explicit conversion between representations. | `via`/scoped representation defaults are planned syntax, not current implicit conversions. |

In the intended model, generic functions keep their source interface while constructors and matches use the selected representation's builders/view. The representation must preserve the required observations and borrowing rules; a bit-packed Boolean representation cannot silently break generic code that needs a real field borrow. A view can also change cost: repeated cons on a packed array may require copying, so a representation is not a universal performance improvement.

The value-type spelling and some admission choices remain provisional. Keep them visibly planned rather than registering an inert metadata attribute that pretends to implement the storage change. Planned allocator selection and in-place/budget attributes are indexed separately in [all attributes](23-attributes-and-pragmas.md#planned-and-provisional-attribute-spellings).

Implementation authority: `compiler/src/check_decls.rs::parse_repr_args`, `compute_layouts`, `layout_of`, `build_layout`, `field_size_align`, `abi_param`; `compiler/src/repr.rs`, `specialize.rs`, `codegen.rs`; runtime `block_kinds.rs`, `block.rs`, `rc.rs`. Behaviour suites: `compiler/tests/attributes.rs`, `ffi.rs`, `llvm_contract.rs`, `repr_classes.rs`, `reclamation.rs`.
