// The block header encoding shared by the runtime (`crate::block`) and the
// code generator (`seed/src/codegen.rs` includes this file verbatim), so
// the kind numbers and the bit layout have one definition. Keep it free of
// crate paths and inner attributes: it is compiled in both crates.
//
// The header is the 64-bit word at byte offset -8 of every block. Every
// field's width is a named constant below (`*_BITS`), each position
// (`*_SHIFT`), mask and range is derived from the widths, and every
// encoder and decoder goes through them, so a change of field widths is
// local to this file. From the low bit (design/self-describing-blocks.md):
//
//   rc      35 bits, signed: 0 immortal, > 0 a single-threaded count,
//           -n an atomic count of n (a block shared across tasks, see
//           the runtime's `rc.rs`), and `RC_FRAME` for a block that
//           lives in an activation's frame rather than on the heap, with
//           `RC_FRAME_RELEASED` for one whose payload has been released
//           (see below); the field must be the low bits, so an atomic add or
//           subtract of one on the whole word changes only the count,
//           except the drop that takes the last reference, whose carry
//           its thread repairs
//   kind     5 bits: one of the `KIND_*` constants below
//   layout  24 bits: for a shaped block (a constructor, closure or
//           dictionary, `kind_has_shape`), its tag (low 8 bits) and the
//           counted mask of its fields (high 16 bits), see below; for
//           every other kind, the payload size in 64-bit words, where
//           `LAYOUT_BIG` says the byte size is in the word before the
//           header (offset -16)
//
// A count never saturates and needs no overflow check: the heap is capped
// at `HEAP_LIMIT_MAX` bytes, so far fewer than `RC_FRAME_RELEASED` words
// can ever hold a reference to one block. There is no sticky "immortal"
// state.
//
// Shaped blocks describe themselves: there is no layout table and no
// layout id. A shaped block's tag is its constructor's (0 for a closure or
// a dictionary), and bit `i` of its mask says whether its field `i` is
// counted: payload word `i` of a constructor, saved argument `i` of a
// closure (after `CLOSURE_PREFIX_WORDS`), slot `i` of a dictionary (after
// `DICT_PREFIX_WORDS`). Reading a tag needs no knowledge of the
// scrutinee's type, which is what lets a `case`, the quote evaluator and
// the task runtime read one.
//
// A shaped block whose description does not fit the header (a tag of
// `SHAPE_ESCAPED` or more, a counted field at `SHAPE_MASK_BITS` or beyond,
// or a constructor or dictionary of more than `SHAPED_SMALL_WORDS` payload
// words, whose free needs its size) is escaped: its tag field is
// `SHAPE_ESCAPED`, its mask is 0, and the word before its header points to
// a static descriptor (`DESC_*`): its tag, its payload size in words, and
// its counted mask over payload words, one bit per word. A descriptor is
// content: any artifact may emit an identical one, and nothing compares
// their addresses.
//
// A shaped block's size is not in its header: the small-object heap knows
// the size of every block it holds, an escaped block's descriptor records
// its size, and a closure is `CLOSURE_PREFIX_WORDS` plus its filled count.
//
// The kind namespace is separate from constructor tags.

/// No kind: a zero header never belongs to a live block.
pub const KIND_NONE: u8 = 0;
/// A constructor block `{ fields... }` (the tag is in the header's layout
/// field), including tuples, the prelude ADTs built by primitives, and
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
/// Immutable contiguous array: `{ format; len; elements... }`.
pub const KIND_ARRAY: u8 = 14;
/// Immutable row-major shaped array: `{ data; shape }`.
pub const KIND_TENSOR: u8 = 15;
/// Column tree: `{ rows; columns; tuple_desc; left; right }`.
pub const KIND_COLUMNAR: u8 = 16;
/// A transactional variable cell: `{ lock; value; format }` (`crate::stm`
/// in the runtime, `Effect.Stm` in the prelude). The value word is the one
/// scanned slot a live block ever has rewritten in place; its code is the
/// cell's own `format` word (`SCAN_RAW`, `SCAN_OBJ` or `SCAN_TOBJ`).
pub const KIND_TVAR: u8 = 17;
/// Native MVar box: Rust-owned mutex, slot and FIFO wait queues.
pub const KIND_MVAR: u8 = 18;
/// Read-only bytes backed by a file mapping released with the block.
pub const KIND_BYTES_MAPPED: u8 = 19;
/// Read-only byte span with a counted reference to its backing buffer.
pub const KIND_BYTES_VIEW: u8 = 20;
/// The owner follows the length and data pointer of the byte span.
pub const BYTES_VIEW_OWNER_WORD: u64 = 2;
/// Native lock-free 64-bit hardware atomic word cell.
pub const KIND_ATOMIC: u8 = 21;
/// Native reader-writer lock box: Rust-owned RwLock guarding a single Cinch word.
pub const KIND_RWLOCK: u8 = 22;
/// Last live kind; keep range validation in step with new runtime kinds.
pub const KIND_LAST: u8 = KIND_RWLOCK;
/// A freed block (debug poisoning): the largest value the field holds.
pub const KIND_FREED: u8 = 31;

/// Width of the signed reference count.
pub const RC_BITS: u32 = 35;
/// Width of the kind.
pub const KIND_BITS: u32 = 5;
/// Width of the layout.
pub const LAYOUT_BITS: u32 = 24;

/// Field positions, packed from the low bit in the order rc, kind, layout.
pub const RC_SHIFT: u32 = 0;
pub const KIND_SHIFT: u32 = RC_SHIFT + RC_BITS;
pub const LAYOUT_SHIFT: u32 = KIND_SHIFT + KIND_BITS;

const _: () = assert!(LAYOUT_SHIFT + LAYOUT_BITS == 64, "the header fields fill one word");
const _: () = assert!(RC_BITS >= 2 && RC_BITS <= 63);
const _: () = assert!(RC_SHIFT == 0, "atomic counts add and subtract one on the whole header word");
const _: () = assert!(KIND_BITS <= 8, "kinds are u8");
const _: () = assert!(KIND_FREED as u64 == (1u64 << KIND_BITS) - 1 && KIND_LAST < KIND_FREED);
const _: () = assert!(LAYOUT_BITS <= 32, "layouts are u32");

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

/// The largest heap cap a count of [`RC_BITS`] stays clear of its frame
/// markers under: 64 GiB of live heap is 2^33 words, a reference is a
/// word, and `RC_FRAME_RELEASED` is `2^34 - 2`, which leaves 2^33 more for
/// references held on stacks. `crate::heap` clamps `CINCH_HEAP_LIMIT` to
/// it.
pub const HEAP_LIMIT_MAX: u64 = 1u64 << 36;

/// `rc` of a block that lives in a function's stack frame rather than on
/// the heap (`seed/src/stack.rs`): never counted, never freed, never
/// scanned. The value is [`RC_MAX`], a count no heap block can hold: the
/// heap cap bounds the live bytes of the process (8 GiB unless the caller
/// raises `CINCH_HEAP_LIMIT`, never past [`HEAP_LIMIT_MAX`]), so nothing can
/// hold `2^34 - 1` references to one block, and a count only ever moves by
/// one from [`RC_FRESH`]. A frame block is therefore distinguishable from
/// every reachable count, and a stale count operation on one is a no-op
/// instead of a corruption (the compiler elides those operations; the
/// runtime's paths are the backstop, and `CINCH_RC_DEBUG=1` traps on a
/// frame block that reaches the free path).
pub const RC_FRAME: i64 = RC_MAX;

/// `rc` of a frame block whose payload has been released: the sticky
/// marker that makes the release happen exactly once
/// (`crate::rc::cinch_frame_release`, design/stack-allocation.md
/// section 7.3). A frame block is released by whichever of three paths
/// reaches it first (its binder's static death, a callee that took it as
/// an owned parameter, or a parent frame block releasing the payload it
/// was stored in), so the payload words are dropped by that path and the
/// header field says so; every later path finds [`RC_FRAME_RELEASED`]
/// and does nothing.
///
/// The value is the count one below [`RC_MAX`], which no heap block can
/// hold for the same reason [`RC_FRAME`] cannot. The header's kind and
/// layout, and an escaped block's descriptor word at -16, are left exactly
/// as the allocation wrote them, so a released block still reads its tag
/// and its counted fields.
pub const RC_FRAME_RELEASED: i64 = RC_MAX - 1;

const _: () = assert!(RC_FRAME == RC_MAX && RC_FRAME > RC_FRESH);
const _: () = assert!(RC_FRAME_RELEASED < RC_FRAME && RC_FRAME_RELEASED > RC_FRESH);
// Heap references at the cap use at most half the count range below the
// frame markers; the other half is the stacks'.
const _: () = assert!(2 * (HEAP_LIMIT_MAX / 8) <= RC_FRAME_RELEASED as u64 + 2);

/// An unshaped block's `layout` saying its byte size is stored out of
/// line, in the word at offset -16 (the payload is at least this many
/// words).
pub const LAYOUT_BIG: u32 = field_mask(LAYOUT_BITS) as u32;

/// Width of a shaped block's tag (the layout field's low bits).
pub const SHAPE_TAG_BITS: u32 = 8;
/// Width of a shaped block's counted mask (the layout field's high bits).
pub const SHAPE_MASK_BITS: u32 = LAYOUT_BITS - SHAPE_TAG_BITS;
/// The tag field of an escaped shaped block; every inline tag is below it.
pub const SHAPE_ESCAPED: u64 = field_mask(SHAPE_TAG_BITS);
/// A shaped block's tag field is `layout & SHAPE_TAG_MASK`.
pub const SHAPE_TAG_MASK: u64 = field_mask(SHAPE_TAG_BITS);
/// The largest constructor or dictionary payload, in words, the
/// small-object heap holds (256 bytes with the header), and so the largest
/// one whose free needs no size in its description.
pub const SHAPED_SMALL_WORDS: u64 = 31;

/// A static descriptor's word holding the block's tag.
pub const DESC_TAG_WORD: usize = 0;
/// A static descriptor's word holding the payload size in words.
pub const DESC_WORDS_WORD: usize = 1;
/// A static descriptor's first counted-mask word: payload word `i` is bit
/// `i % 64` of mask word `i / 64`.
pub const DESC_MASK_WORD: usize = 2;

const _: () = assert!(SHAPE_MASK_BITS == 16 && SHAPE_TAG_BITS + SHAPE_MASK_BITS == LAYOUT_BITS);

/// The words of a static descriptor for a block of `words` payload words.
pub const fn desc_words(words: u64) -> u64 {
    DESC_MASK_WORD as u64 + words.div_ceil(64)
}

/// The layout field of an inline shaped block: `tag` (below
/// [`SHAPE_ESCAPED`]) and the counted mask of its fields.
pub const fn shape_field(tag: u64, mask: u64) -> u32 {
    ((tag & SHAPE_TAG_MASK) | ((mask & field_mask(SHAPE_MASK_BITS)) << SHAPE_TAG_BITS)) as u32
}

/// The layout field of an escaped shaped block.
pub const ESCAPED_FIELD: u32 = SHAPE_ESCAPED as u32;

/// A shaped block's description as one word, for a consumer that takes
/// either form through one slot (the quote evaluator's `LAM` and `CTOR`
/// nodes): an inline layout field is `field << 1 | 1`; an escaped block's
/// is the address of its static descriptor, which is 8-aligned and so even.
pub const fn inline_description(field: u32) -> u64 {
    ((field as u64) << 1) | 1
}

/// The tag field of a shaped block's layout (`SHAPE_ESCAPED` when the tag
/// is in its descriptor).
pub const fn shape_field_tag(layout: u32) -> u64 {
    layout as u64 & SHAPE_TAG_MASK
}

/// The counted mask of a shaped block's layout, over its fields.
pub const fn shape_field_mask(layout: u32) -> u64 {
    (layout as u64 >> SHAPE_TAG_BITS) & field_mask(SHAPE_MASK_BITS)
}

/// Whether a shaped block's layout says it is escaped.
pub const fn shape_escaped(layout: u32) -> bool {
    shape_field_tag(layout) == SHAPE_ESCAPED
}

/// The payload words before a shaped kind's fields, which its mask does not
/// cover: a closure's code, arity and filled count, a dictionary's method
/// count, nothing for a constructor.
pub const fn shape_prefix_words(kind: u8) -> u64 {
    if kind == KIND_CLOSURE {
        CLOSURE_PREFIX_WORDS
    } else if kind == KIND_DICT {
        DICT_PREFIX_WORDS
    } else {
        0
    }
}

// Scan codes. Each payload word a release visits has a `SCAN_BITS` code:
// `SCAN_RAW` is never touched, `SCAN_OBJ` is a counted pointer (or 0, or
// an immortal block, both skipped), and `SCAN_TOBJ` is an immediate when
// its low bit is 1 and a counted pointer otherwise. Code 3 is invalid. A
// shaped block's counted fields read as `SCAN_TOBJ`: the release tests
// every counted word for an immediate, which is all the distinction ever
// bought there. The fixed kinds' codes are `fixed_scan_code`'s; an array's
// is its element format; a transactional cell's is its format word.

/// Width of one word's code.
pub const SCAN_BITS: u32 = 2;
pub const SCAN_RAW: u64 = 0;
pub const SCAN_OBJ: u64 = 1;
pub const SCAN_TOBJ: u64 = 2;
/// The mask of one code.
pub const SCAN_CODE_MASK: u64 = field_mask(SCAN_BITS);

const _: () = assert!(SCAN_RAW == 0, "a zero map scans nothing");
const _: () = assert!(SCAN_TOBJ <= SCAN_CODE_MASK && SCAN_OBJ <= SCAN_CODE_MASK);
const _: () = assert!(64 % SCAN_BITS == 0);

/// `code` placed at `slot` of a map word.
pub const fn scan_slot(code: u64, slot: u64) -> u64 {
    (code & SCAN_CODE_MASK) << (SCAN_BITS as u64 * slot)
}

/// The code at `slot` of a map word.
pub const fn scan_code_at(map: u64, slot: u64) -> u64 {
    (map >> (SCAN_BITS as u64 * slot)) & SCAN_CODE_MASK
}

/// Whether blocks of `kind` describe their fields in their header's layout
/// field (a tag and a counted mask) rather than record their size.
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
/// A memo cell's owned heap pointer, first the thunk and then any boxed cached value.
pub const MEMO_OWNED_WORD: u64 = 1;
/// A quote code block's fixed words (`{ KIND_CODE; code; len; holes }`)
/// before its hole words.
pub const CODE_PREFIX_WORDS: u64 = 4;

/// A transactional variable cell's versioned lock word: bit 0 is the
/// write lock, bit 1 the pin a reader holds while it counts the value,
/// and the bits above them the version of the last commit that wrote
/// the value.
pub const TVAR_LOCK_WORD: u64 = 0;
/// The cell's value word, scanned by the code in its format word.
pub const TVAR_VALUE_WORD: u64 = 1;
/// The scan code of the value word, fixed when the cell is made.
pub const TVAR_FORMAT_WORD: u64 = 2;
/// A cell's payload size in words.
pub const TVAR_WORDS: u64 = 3;

/// The code of payload word `slot` of a block whose kind fixes its map
/// (every kind but `kind_has_shape`'s). A memo's value word is raw
/// because its heap pointer, if any, lives in the scanned owned slot.
/// A knot's value has no class at the kind. A quote block's holes are quote blocks.
pub const fn fixed_scan_code(kind: u8, slot: u64) -> u64 {
    if kind == KIND_TENSOR {
        SCAN_OBJ
    } else if kind == KIND_COLUMNAR {
        if slot >= 3 { SCAN_OBJ } else { SCAN_RAW }
    } else if kind == KIND_MEMO {
        if slot == MEMO_OWNED_WORD {
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

/// Encode a header word. `rc` must be in `RC_MIN..=RC_MAX`; it is stored
/// in two's complement in its field.
pub const fn header(rc: i64, kind: u8, layout: u32) -> u64 {
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
pub const fn header_layout(h: u64) -> u32 {
    ((h >> LAYOUT_SHIFT) & field_mask(LAYOUT_BITS)) as u32
}

/// The header of a static (immortal) block: `layout` is the payload size
/// in words, or a shaped block's layout field.
pub const fn static_header(kind: u8, layout: u32) -> u64 {
    header(RC_IMMORTAL, kind, layout)
}
