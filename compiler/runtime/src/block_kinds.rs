// The block header encoding shared by the runtime (`crate::block`) and the
// code generator (`compiler/src/codegen.rs` includes this file verbatim), so
// the kind numbers and the bit layout have one definition. Keep it free of
// crate paths and inner attributes: it is compiled in both crates.
//
// The header is the 64-bit word at byte offset -8 of every block. Every
// field's width is a named constant below (`*_BITS`), each position
// (`*_SHIFT`), mask and range is derived from the widths, and every
// encoder and decoder goes through them, so a change of field widths is
// local to this file. Today, from the low bit:
//
//   rc      40 bits, signed: 0 immortal, > 0 a single-threaded count,
//           < 0 reserved for atomic (multi-threaded) counts
//   kind     8 bits: one of the `KIND_*` constants below
//   layout  16 bits: for a constructor block (`KIND_CTOR`), its
//           constructor's layout id; for every other kind, the payload
//           size in 64-bit words. `LAYOUT_BIG` is the escape: the
//           value is in the word before the header (offset -16), a
//           constructor descriptor for a constructor block, the byte
//           size otherwise
//
// A count never saturates and needs no overflow check: the heap is capped
// below 8 * 2^RC_BITS bytes, so fewer than `RC_MAX` words can ever hold a
// reference to one block. There is no sticky "immortal" state.
//
// Constructor blocks carry no tag word: the tag is in the layout id. The
// code generator gives each data type one group of `2^CTOR_TAG_BITS`
// consecutive ids, `id = group << CTOR_TAG_BITS | tag`, so a type's
// `base` is `group << CTOR_TAG_BITS` and `tag = id - base = id &
// CTOR_TAG_MASK`. The mask form needs no knowledge of the scrutinee's
// type, which is what lets a `case`, the quote evaluator and the task
// runtime read a tag without it. The id also indexes the program's
// layout table (`@__cinch_layouts`, registered through
// `cinch_layouts_register`), whose entry gives the payload size.
//
// A constructor that does not fit (a type with more than
// `2^CTOR_TAG_BITS` constructors, or a program with more types than
// `CTOR_GROUPS`) is escaped: its header layout is `LAYOUT_BIG` and the
// word at -16 is its full descriptor, `index << CTOR_ESC_INDEX_SHIFT |
// tag << CTOR_ESC_TAG_SHIFT | LAYOUT_BIG`. A descriptor is what generated
// code hands `cinch_alloc_ctor`: a regular id (below `LAYOUT_BIG`) or an
// escaped descriptor, and its low `LAYOUT_BITS` are the header's layout
// either way.
//
// The kind namespace is separate from constructor tags.

/// No kind: a zero header never belongs to a live block.
pub const KIND_NONE: u8 = 0;
/// A constructor block `{ fields... }` (the tag is in the header's layout
/// id), including tuples, the prelude ADTs built by primitives, and
/// boundary-layout (`repr(C)`) payloads.
pub const KIND_CTOR: u8 = 1;
/// A closure `{ code; arity; nfilled; saved... }`.
pub const KIND_CLOSURE: u8 = 2;
/// An instance dictionary `{ nmeth; methods... }`.
pub const KIND_DICT: u8 = 3;
/// A memo cell `{ state; thunk; value }`.
pub const KIND_MEMO: u8 = 4;
/// A fixpoint knot `{ state; value }`.
pub const KIND_KNOT: u8 = 5;
/// A boxed `Nat` (the runtime's `Boxed<Nat>`).
pub const KIND_NAT: u8 = 6;
/// A boxed `Int` (the runtime's `Boxed<Int>`).
pub const KIND_INT: u8 = 7;
/// A heap `Bytes` buffer block (`Boxed<Bytes>`), owning its data.
pub const KIND_BYTES: u8 = 8;
/// A read-only `Bytes` block whose data lives in the data section.
pub const KIND_BYTES_STATIC: u8 = 9;
/// An interned `Sym` cell.
pub const KIND_SYM: u8 = 10;
/// A quote code block `{ KIND_CODE; code; len; holes... }`.
pub const KIND_CODE: u8 = 11;
/// A machine scalar boxed for a polymorphic slot (reserved for the
/// representation decision; nothing allocates it yet).
pub const KIND_BOXED_SCALAR: u8 = 12;
/// Unscanned bytes generated code owns that are not a Cinch value: the
/// canonical ABI's lowered buffers on the Wasm target.
pub const KIND_RAW: u8 = 13;
/// A freed block (debug poisoning).
pub const KIND_FREED: u8 = 255;

/// Width of the signed reference count.
pub const RC_BITS: u32 = 40;
/// Width of the kind.
pub const KIND_BITS: u32 = 8;
/// Width of the layout.
pub const LAYOUT_BITS: u32 = 16;

/// Field positions, packed from the low bit in the order rc, kind, layout.
pub const RC_SHIFT: u32 = 0;
pub const KIND_SHIFT: u32 = RC_SHIFT + RC_BITS;
pub const LAYOUT_SHIFT: u32 = KIND_SHIFT + KIND_BITS;

const _: () = assert!(LAYOUT_SHIFT + LAYOUT_BITS == 64, "the header fields fill one word");
const _: () = assert!(RC_BITS >= 2 && RC_BITS <= 63);
const _: () = assert!(KIND_BITS <= 8, "kinds are u8");
const _: () = assert!(LAYOUT_BITS <= 16, "layouts are u16");

/// The low `bits` bits set.
const fn field_mask(bits: u32) -> u64 {
    (1u64 << bits) - 1
}

/// `rc` of a static or interned block: never counted, never freed.
pub const RC_IMMORTAL: i64 = 0;
/// `rc` of a freshly allocated heap block: one owner.
pub const RC_FRESH: i64 = 1;
/// The largest single-threaded count (unreachable by construction, see
/// above).
pub const RC_MAX: i64 = (1i64 << (RC_BITS - 1)) - 1;
/// The most negative count (the atomic range's far end).
pub const RC_MIN: i64 = -(1i64 << (RC_BITS - 1));

/// The `layout` value saying the real value is stored out of line, in
/// the word at offset -16: a constructor block's descriptor, or another
/// kind's payload byte size (the payload is at least this many words).
pub const LAYOUT_BIG: u16 = field_mask(LAYOUT_BITS) as u16;

/// Width of the tag inside a regular constructor layout id (its low bits).
pub const CTOR_TAG_BITS: u32 = 6;
/// The tag of a regular layout id is `id & CTOR_TAG_MASK`.
pub const CTOR_TAG_MASK: u64 = field_mask(CTOR_TAG_BITS);
/// How many data types get a group of regular ids: every id of every
/// group stays below `LAYOUT_BIG`.
pub const CTOR_GROUPS: u64 = (LAYOUT_BIG as u64) >> CTOR_TAG_BITS;
/// Where an escaped descriptor keeps its tag, and how wide it is.
pub const CTOR_ESC_TAG_SHIFT: u32 = LAYOUT_BITS;
pub const CTOR_ESC_TAG_BITS: u32 = 24;
/// Where an escaped descriptor keeps its layout table index (the rest of
/// the word).
pub const CTOR_ESC_INDEX_SHIFT: u32 = CTOR_ESC_TAG_SHIFT + CTOR_ESC_TAG_BITS;

const _: () = assert!(CTOR_TAG_BITS < LAYOUT_BITS);
const _: () = assert!(((CTOR_GROUPS << CTOR_TAG_BITS) - 1) < LAYOUT_BIG as u64);
const _: () = assert!(CTOR_ESC_INDEX_SHIFT < 64);

/// A regular constructor layout id: `tag` of the data type in `group`
/// (`group < CTOR_GROUPS`, `tag <= CTOR_TAG_MASK`).
pub const fn ctor_id(group: u64, tag: u64) -> u64 {
    (group << CTOR_TAG_BITS) | tag
}

/// The escaped descriptor of `tag` whose layout table entry is `index`.
pub const fn ctor_escaped(index: u64, tag: u64) -> u64 {
    (index << CTOR_ESC_INDEX_SHIFT) | (tag << CTOR_ESC_TAG_SHIFT) | LAYOUT_BIG as u64
}

/// The header layout field a descriptor stores: the id itself, or
/// `LAYOUT_BIG` for an escaped one.
pub const fn ctor_layout_field(desc: u64) -> u16 {
    (desc & field_mask(LAYOUT_BITS)) as u16
}

/// The constructor tag of a block whose header layout is `layout` and,
/// when that is `LAYOUT_BIG`, whose descriptor word at -16 is `ext`.
pub const fn ctor_tag(layout: u16, ext: u64) -> u64 {
    if layout == LAYOUT_BIG {
        (ext >> CTOR_ESC_TAG_SHIFT) & field_mask(CTOR_ESC_TAG_BITS)
    } else {
        layout as u64 & CTOR_TAG_MASK
    }
}

/// The layout table index of a constructor block, as for [`ctor_tag`].
pub const fn ctor_index(layout: u16, ext: u64) -> u64 {
    if layout == LAYOUT_BIG {
        ext >> CTOR_ESC_INDEX_SHIFT
    } else {
        layout as u64
    }
}

/// Width of the payload size, in words, in the low bits of a layout table
/// entry. The entry's high bits are reserved for the pointer map.
pub const LAYOUT_ENTRY_WORDS_BITS: u32 = 32;

/// A layout table entry for a constructor of `words` payload words.
pub const fn layout_entry(words: u64) -> u64 {
    words & field_mask(LAYOUT_ENTRY_WORDS_BITS)
}

/// The payload size in words a layout table entry records.
pub const fn layout_entry_words(entry: u64) -> u64 {
    entry & field_mask(LAYOUT_ENTRY_WORDS_BITS)
}

/// Encode a header word. `rc` must be in `RC_MIN..=RC_MAX`; it is stored
/// in two's complement in its field.
pub const fn header(rc: i64, kind: u8, layout: u16) -> u64 {
    ((rc as u64 & field_mask(RC_BITS)) << RC_SHIFT)
        | ((kind as u64 & field_mask(KIND_BITS)) << KIND_SHIFT)
        | ((layout as u64 & field_mask(LAYOUT_BITS)) << LAYOUT_SHIFT)
}

/// The signed reference count of header `h`.
pub const fn header_rc(h: u64) -> i64 {
    let raw = (h >> RC_SHIFT) & field_mask(RC_BITS);
    // Sign-extend from the field's top bit.
    ((raw << (64 - RC_BITS)) as i64) >> (64 - RC_BITS)
}

/// The kind of header `h`.
pub const fn header_kind(h: u64) -> u8 {
    ((h >> KIND_SHIFT) & field_mask(KIND_BITS)) as u8
}

/// The layout of header `h`.
pub const fn header_layout(h: u64) -> u16 {
    ((h >> LAYOUT_SHIFT) & field_mask(LAYOUT_BITS)) as u16
}

/// The header of a static (immortal) block: `layout` is the payload size
/// in words, or a constructor's layout field.
pub const fn static_header(kind: u8, layout: u16) -> u64 {
    header(RC_IMMORTAL, kind, layout)
}
