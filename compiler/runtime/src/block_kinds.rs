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
//           -n an atomic count of n (a block shared across tasks, see
//           the runtime's `rc.rs`); the field must be the low bits, so
//           an atomic add or subtract of one on the whole word changes
//           only the count, except the drop that takes the last
//           reference, whose carry its thread repairs
//   kind     8 bits: one of the `KIND_*` constants below
//   layout  16 bits: for a constructor block (`KIND_CTOR`), its
//           constructor's layout id; for a closure or a dictionary
//           (`KIND_CLOSURE`, `KIND_DICT`), its shape's layout table
//           index; for every other kind, the payload size in 64-bit
//           words. `LAYOUT_BIG` is the escape: the value is in the word
//           before the header (offset -16), a descriptor for the three
//           shaped kinds, the byte size otherwise
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
// A closure's or a dictionary's layout is the table index of its shape,
// described the same way with tag 0: a regular index below `LAYOUT_BIG`,
// or `LAYOUT_BIG` and the escaped descriptor at -16. A closure's shape is
// the classes of its unit's parameters, which its saved words are, and
// its size is not in the table: a closure is `CLOSURE_PREFIX_WORDS`
// words plus its filled count. A dictionary's shape is its instance's
// method slots, and its entry records its size.
//
// The layout table's entries carry the pointer maps the drop machinery
// scans (`SCAN_*` below). Every unshaped kind's map is fixed by the kind
// (`fixed_scan_code`).
//
// The kind namespace is separate from constructor tags.

/// No kind: a zero header never belongs to a live block.
pub const KIND_NONE: u8 = 0;
/// A constructor block `{ fields... }` (the tag is in the header's layout
/// id), including tuples, the prelude ADTs built by primitives, and
/// boundary-layout (`repr(C)`) payloads.
pub const KIND_CTOR: u8 = 1;
/// A closure `{ code; arity; nfilled; saved... }`; its layout is its
/// shape's descriptor.
pub const KIND_CLOSURE: u8 = 2;
/// An instance dictionary `{ nmeth; methods... }`; its layout is its
/// shape's descriptor.
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
const _: () = assert!(RC_SHIFT == 0, "atomic counts add and subtract one on the whole header word");
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
/// entry. The rest of the entry is the pointer map.
pub const LAYOUT_ENTRY_WORDS_BITS: u32 = 32;
const LAYOUT_ENTRY_MAP_BITS: u32 = 64 - LAYOUT_ENTRY_WORDS_BITS;

// Pointer maps. Each payload word of a block has a `SCAN_BITS` code:
// `SCAN_RAW` is never touched, `SCAN_OBJ` is a counted pointer (or 0, or
// an immortal block, both skipped), and `SCAN_TOBJ` is an immediate when
// its low bit is 1 and a counted pointer otherwise. Code 3 is invalid:
// registration refuses a table that uses it.
//
// A block of at most `SCAN_INLINE_WORDS` words keeps its codes in its
// entry's map bits, word `i` at bit `SCAN_BITS * i` of the map. A wider
// block's map bits are instead the table index of its extension:
// `scan_ext_len(words)` consecutive table words, word `i`'s code at bit
// `SCAN_BITS * (i % SCAN_EXT_WORDS)` of extension word `i /
// SCAN_EXT_WORDS`. Which form an entry has follows from its size, so no
// flag bit is spent.

/// Width of one word's code.
pub const SCAN_BITS: u32 = 2;
pub const SCAN_RAW: u64 = 0;
pub const SCAN_OBJ: u64 = 1;
pub const SCAN_TOBJ: u64 = 2;
/// The mask of one code.
pub const SCAN_CODE_MASK: u64 = field_mask(SCAN_BITS);
/// Payload words an entry's own map covers.
pub const SCAN_INLINE_WORDS: u64 = (LAYOUT_ENTRY_MAP_BITS / SCAN_BITS) as u64;
/// Payload words one extension word covers.
pub const SCAN_EXT_WORDS: u64 = (64 / SCAN_BITS) as u64;

const _: () = assert!(SCAN_RAW == 0, "a zero entry scans nothing");
const _: () = assert!(SCAN_TOBJ <= SCAN_CODE_MASK && SCAN_OBJ <= SCAN_CODE_MASK);
const _: () = assert!(LAYOUT_ENTRY_MAP_BITS % SCAN_BITS == 0 && 64 % SCAN_BITS == 0);

/// `code` placed at `slot` of a map word.
pub const fn scan_slot(code: u64, slot: u64) -> u64 {
    (code & SCAN_CODE_MASK) << (SCAN_BITS as u64 * slot)
}

/// The code at `slot` of a map word.
pub const fn scan_code_at(map: u64, slot: u64) -> u64 {
    (map >> (SCAN_BITS as u64 * slot)) & SCAN_CODE_MASK
}

/// A layout table entry for a block of `words` payload words whose map
/// bits are `map`: the packed codes when `words <= SCAN_INLINE_WORDS`,
/// else the table index of the extension.
pub const fn layout_entry(words: u64, map: u64) -> u64 {
    (words & field_mask(LAYOUT_ENTRY_WORDS_BITS)) | (map << LAYOUT_ENTRY_WORDS_BITS)
}

/// The payload size in words a layout table entry records.
pub const fn layout_entry_words(entry: u64) -> u64 {
    entry & field_mask(LAYOUT_ENTRY_WORDS_BITS)
}

/// The map bits of a layout table entry.
pub const fn layout_entry_map(entry: u64) -> u64 {
    entry >> LAYOUT_ENTRY_WORDS_BITS
}

/// Whether an entry's map bits are its codes (else an extension index).
pub const fn layout_entry_inline(entry: u64) -> bool {
    layout_entry_words(entry) <= SCAN_INLINE_WORDS
}

/// The extension words a map of `words` payload words needs (0 inline).
pub const fn scan_ext_len(words: u64) -> u64 {
    if words <= SCAN_INLINE_WORDS {
        0
    } else {
        words.div_ceil(SCAN_EXT_WORDS)
    }
}

/// Whether blocks of `kind` keep a layout table index (a descriptor) in
/// their header's layout field rather than their size.
pub const fn kind_has_shape(kind: u8) -> bool {
    kind == KIND_CTOR || kind == KIND_CLOSURE || kind == KIND_DICT
}

/// A closure's fixed words before its saved arguments: code, arity and
/// filled count, none of them counted.
pub const CLOSURE_PREFIX_WORDS: u64 = 3;
/// The closure word holding its filled count.
pub const CLOSURE_FILLED_WORD: u64 = 2;
/// A dictionary's method count word, before its method slots.
pub const DICT_PREFIX_WORDS: u64 = 1;
/// A memo cell's thunk word (`{ state; thunk; value }`).
pub const MEMO_THUNK_WORD: u64 = 1;
/// A quote code block's fixed words (`{ KIND_CODE; code; len; holes }`)
/// before its hole words.
pub const CODE_PREFIX_WORDS: u64 = 4;

/// The code of payload word `slot` of a block whose kind fixes its map
/// (every kind but `kind_has_shape`'s). A memo cell's value and a knot's
/// value have no class at the kind: they are `SCAN_RAW`, which can only
/// leak. A quote block's holes are quote blocks.
pub const fn fixed_scan_code(kind: u8, slot: u64) -> u64 {
    if kind == KIND_MEMO {
        if slot == MEMO_THUNK_WORD {
            SCAN_OBJ
        } else {
            SCAN_RAW
        }
    } else if kind == KIND_CODE {
        if slot < CODE_PREFIX_WORDS {
            SCAN_RAW
        } else {
            SCAN_OBJ
        }
    } else {
        SCAN_RAW
    }
}

/// The descriptor of shape `index` of a closure or dictionary: the index
/// itself when it fits the header, else the escaped form with tag 0.
pub const fn shape_desc(index: u64) -> u64 {
    if index < LAYOUT_BIG as u64 {
        index
    } else {
        ctor_escaped(index, 0)
    }
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
