//! The block header and the header-aware allocator.
//!
//! Every Cinch heap block, whether generated code or this runtime allocates
//! it, and every static block the code generator emits, carries one header
//! word at offset -8 from the pointer its word holds. Every field offset
//! generated code and the runtime use stays relative to that pointer, so
//! the header is invisible to them. The encoding (rc, kind, layout) is in
//! `block_kinds.rs`, which the code generator includes verbatim.
//!
//! [`allocate_block`] and [`free`] are the only functions that touch the
//! backing allocator, so replacing it (the system allocator today, through
//! the counting wrapper of [`crate::heap`]) is a change to these two alone.
//!
//! A constructor block's `layout` is its layout id (the tag is in it, see
//! `block_kinds.rs`), and a closure's or a dictionary's is its shape's
//! table index. The program's layout table, which generated code hands
//! over through [`register_layouts`] before any of its code runs, gives
//! each id or shape its payload size and pointer map ([`Scan`]). Every
//! other kind's `layout` is its payload size in words. Either value, when
//! it does not fit the field, goes in one extra word before the header: a
//! payload of [`LAYOUT_BIG`] words or more keeps its byte size there, an
//! escaped descriptor itself.
//!
//! A word with low bit 1 is an immediate, never a block: nothing here may
//! be handed one.

use std::alloc::Layout;

include!("block_kinds.rs");

/// Bytes the header adds before every payload.
pub const HEADER_BYTES: usize = 8;

/// The header word of the block whose payload starts at `p`.
///
/// # Safety
///
/// `p` must be the payload pointer of a header block (heap or static).
#[inline]
pub unsafe fn read_header(p: *const u8) -> u64 {
    unsafe { p.cast::<u64>().sub(1).read() }
}

/// Overwrite the header word of the block whose payload starts at `p`.
///
/// # Safety
///
/// `p` must be the payload pointer of a heap header block.
#[inline]
pub unsafe fn write_header(p: *mut u8, h: u64) {
    unsafe { p.cast::<u64>().sub(1).write(h) }
}

/// The reference count field.
#[inline]
pub const fn rc(h: u64) -> i64 {
    header_rc(h)
}

/// The kind field.
#[inline]
pub const fn kind(h: u64) -> u8 {
    header_kind(h)
}

/// The layout field.
#[inline]
pub const fn layout(h: u64) -> u16 {
    header_layout(h)
}

/// The program's layout table (`@__cinch_layouts`): one entry per layout
/// id or shape index, giving its payload size and pointer map, followed
/// by the map extensions of the wide entries. Null until registered.
static LAYOUTS: std::sync::atomic::AtomicPtr<u64> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());
static LAYOUTS_LEN: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// Install the program's layout table of `len` words. The table is
/// read-only data that outlives every block, so a pointer suffices.
///
/// # Safety
///
/// `table` points to `len` readable words that stay valid for the rest
/// of the process.
pub unsafe fn register_layouts(table: *const u64, len: usize) {
    use std::sync::atomic::Ordering::Release;
    LAYOUTS_LEN.store(0, Release);
    LAYOUTS.store(table.cast_mut(), Release);
    LAYOUTS_LEN.store(len, Release);
}

/// The registered table as a slice (empty before registration).
fn layout_table() -> &'static [u64] {
    use std::sync::atomic::Ordering::Acquire;
    let len = LAYOUTS_LEN.load(Acquire);
    let table = LAYOUTS.load(Acquire);
    if table.is_null() {
        return &[];
    }
    unsafe { std::slice::from_raw_parts(table, len) }
}

/// Layout table entry `index`, or `None` when no table holds it.
fn layout_entry_at(index: u64) -> Option<u64> {
    layout_table().get(usize::try_from(index).ok()?).copied()
}

/// The rounded payload size of a `size`-byte request: whole words,
/// minimum one, so every block keeps the 8-byte alignment the immediate
/// tag relies on. `None` on overflow.
fn round_payload(size: usize) -> Option<usize> {
    match size.checked_add(7)? & !7 {
        0 => Some(8),
        s => Some(s),
    }
}

/// The full allocation layout of a `payload`-byte block whose prefix
/// holds the extra word (`ext`) or only the header.
fn plan(payload: usize, ext: bool) -> Option<Layout> {
    let prefix = if ext { 2 * HEADER_BYTES } else { HEADER_BYTES };
    Layout::from_size_align(payload.checked_add(prefix)?, 8).ok()
}

/// Allocate the backing memory of a block, write `ext` (when given) and
/// `header`, and account for it. Returns the payload pointer.
unsafe fn allocate_block(payload: usize, header: u64, ext: Option<u64>, immortal: bool, zeroed: bool) -> *mut u8 {
    let Some(layout) = plan(payload, ext.is_some()) else {
        crate::abi::panic_exit(b"block size overflow");
    };
    let base = unsafe {
        if zeroed {
            std::alloc::alloc_zeroed(layout)
        } else {
            std::alloc::alloc(layout)
        }
    };
    if base.is_null() {
        crate::abi::panic_exit(b"block allocation out of memory");
    }
    let p = match ext {
        Some(x) => {
            unsafe { base.cast::<u64>().write(x) };
            unsafe { base.add(2 * HEADER_BYTES) }
        }
        None => unsafe { base.add(HEADER_BYTES) },
    };
    unsafe { write_header(p, header) };
    crate::heap::account_block(layout.size(), immortal);
    p
}

/// Allocate a block with a `size`-byte payload (rounded up to whole words,
/// minimum one) and write its header with `rc` and `kind`; the layout
/// field is the payload size. Returns the payload pointer. The payload is
/// zero-filled when `zeroed`, otherwise uninitialised. Overflow and
/// allocation failure fault (exit 71). The shaped kinds (constructors,
/// closures, dictionaries) go through [`allocate_shaped`].
///
/// # Safety
///
/// The caller initialises every payload word it later reads.
pub unsafe fn allocate(size: usize, kind: u8, rc: i64, zeroed: bool) -> *mut u8 {
    let Some(payload) = round_payload(size) else {
        crate::abi::panic_exit(b"block size overflow");
    };
    let words = payload / 8;
    let (field, ext) = if words >= LAYOUT_BIG as usize {
        (LAYOUT_BIG, Some(payload as u64))
    } else {
        (words as u16, None)
    };
    unsafe { allocate_block(payload, header(rc, kind, field), ext, rc == RC_IMMORTAL, zeroed) }
}

/// Allocate a fresh block of a shaped kind (`kind_has_shape`) with `size`
/// payload bytes whose descriptor is `desc` (a layout id or shape index,
/// or an escaped descriptor that goes in the extra word). Returns the
/// payload pointer.
///
/// # Safety
///
/// As for [`allocate`]; `desc`'s layout table entry describes this
/// payload.
pub unsafe fn allocate_shaped(size: usize, kind: u8, desc: u64, zeroed: bool) -> *mut u8 {
    let Some(payload) = round_payload(size) else {
        crate::abi::panic_exit(b"block size overflow");
    };
    let field = ctor_layout_field(desc);
    let ext = (field == LAYOUT_BIG).then_some(desc);
    unsafe { allocate_block(payload, header(RC_FRESH, kind, field), ext, false, zeroed) }
}

/// Allocate a fresh constructor block: [`allocate_shaped`] of `KIND_CTOR`.
///
/// # Safety
///
/// As for [`allocate_shaped`].
pub unsafe fn allocate_ctor(size: usize, desc: u64, zeroed: bool) -> *mut u8 {
    unsafe { allocate_shaped(size, KIND_CTOR, desc, zeroed) }
}

/// Move `value` into a fresh header block of `kind` and count `rc`, the
/// shape of every runtime-owned box.
pub fn allocate_value<T>(kind: u8, rc: i64, value: T) -> *mut T {
    const { assert!(std::mem::align_of::<T>() <= 8) };
    let p = unsafe { allocate(std::mem::size_of::<T>(), kind, rc, false) }.cast::<T>();
    unsafe { p.write(value) };
    p
}

/// The word before the header of the block at `p` (the escape word).
///
/// # Safety
///
/// `p`'s header layout is [`LAYOUT_BIG`], so the word exists.
unsafe fn ext_word(p: *const u8) -> u64 {
    unsafe { p.cast::<u64>().sub(2).read() }
}

/// The constructor tag of the constructor block at `p`.
///
/// # Safety
///
/// `p` must be the payload pointer of a constructor block (heap or static).
pub unsafe fn ctor_tag_of(p: *const u8) -> u64 {
    let l = layout(unsafe { read_header(p) });
    let ext = if l == LAYOUT_BIG { unsafe { ext_word(p) } } else { 0 };
    ctor_tag(l, ext)
}

/// The layout table entry of the shaped block at `p` whose header is `h`.
///
/// # Safety
///
/// `p` is the payload pointer of a header block of a shaped kind.
unsafe fn shape_entry(p: *const u8, h: u64) -> Option<u64> {
    let l = layout(h);
    let ext = if l == LAYOUT_BIG { unsafe { ext_word(p) } } else { 0 };
    layout_entry_at(ctor_index(l, ext))
}

/// The payload words of the shaped block at `p`: a closure's prefix and
/// filled count, every other shaped kind's table entry (at least one
/// word, as allocated).
///
/// # Safety
///
/// As for [`shape_entry`].
unsafe fn shaped_words(p: *const u8, h: u64, entry: u64) -> usize {
    if kind(h) == KIND_CLOSURE {
        let filled = unsafe { p.cast::<u64>().add(CLOSURE_FILLED_WORD as usize).read() };
        (CLOSURE_PREFIX_WORDS as usize).saturating_add(filled as usize)
    } else {
        (layout_entry_words(entry) as usize).max(1)
    }
}

/// The payload size in bytes of the heap block at `p` whose header is
/// `h`, and whether its prefix holds the extra word. `None` for a shaped
/// block whose layout no registered table records.
///
/// # Safety
///
/// `p` must be the payload pointer of a heap header block whose header is
/// `h`.
pub unsafe fn payload_bytes(p: *const u8, h: u64) -> Option<(usize, bool)> {
    let l = layout(h);
    let big = l == LAYOUT_BIG;
    if kind_has_shape(kind(h)) {
        let entry = unsafe { shape_entry(p, h) }?;
        return Some((unsafe { shaped_words(p, h, entry) }.checked_mul(8)?, big));
    }
    let ext = if big { unsafe { ext_word(p) } } else { 0 };
    Some((if big { ext as usize } else { l as usize * 8 }, big))
}

/// How the drop machinery reads one block's payload: how many words it
/// has and which of them are counted.
pub struct Scan {
    words: usize,
    kind: u8,
    /// The table entry of a shaped block (0 otherwise).
    entry: u64,
    /// A wide entry's extension words.
    ext: &'static [u64],
}

impl Scan {
    /// The scan of the live heap or static block at `p` whose header is
    /// `h`. A shaped block whose layout the table does not record, a
    /// closure filled past its shape, or a truncated extension is a
    /// compiler bug and traps.
    ///
    /// # Safety
    ///
    /// `p` is the payload pointer of a header block whose header is `h`.
    pub unsafe fn of(p: *const u8, h: u64) -> Scan {
        let k = kind(h);
        if !kind_has_shape(k) {
            let Some((bytes, _)) = (unsafe { payload_bytes(p, h) }) else {
                crate::abi::panic_exit(b"compiler bug: block header corrupt");
            };
            return Scan { words: bytes / 8, kind: k, entry: 0, ext: &[] };
        }
        let Some(entry) = (unsafe { shape_entry(p, h) }) else {
            crate::abi::panic_exit(b"compiler bug: shaped block with no registered layout");
        };
        let words = unsafe { shaped_words(p, h, entry) };
        let mapped = layout_entry_words(entry) as usize;
        if k == KIND_CLOSURE && words > mapped {
            crate::abi::panic_exit(b"compiler bug: closure filled past its shape");
        }
        let ext = if layout_entry_inline(entry) {
            &[][..]
        } else {
            let table = layout_table();
            let start = layout_entry_map(entry) as usize;
            let n = scan_ext_len(mapped as u64) as usize;
            match start.checked_add(n).and_then(|end| table.get(start..end)) {
                Some(ext) => ext,
                None => crate::abi::panic_exit(b"compiler bug: layout map extension out of range"),
            }
        };
        // A constructor or dictionary is scanned over its mapped words: a
        // zero-field constructor's one allocated word holds nothing.
        let words = if k == KIND_CLOSURE { words } else { mapped };
        Scan { words, kind: k, entry, ext }
    }

    /// The number of payload words to visit.
    pub fn words(&self) -> usize {
        self.words
    }

    /// The code (`SCAN_*`) of payload word `slot` (below [`Scan::words`]).
    /// An invalid code is a compiler bug and traps.
    pub fn code(&self, slot: usize) -> u64 {
        let slot = slot as u64;
        let code = if !kind_has_shape(self.kind) {
            fixed_scan_code(self.kind, slot)
        } else if self.ext.is_empty() {
            scan_code_at(layout_entry_map(self.entry), slot)
        } else {
            scan_code_at(self.ext[(slot / SCAN_EXT_WORDS) as usize], slot % SCAN_EXT_WORDS)
        };
        if code > SCAN_TOBJ {
            crate::abi::panic_exit(b"compiler bug: invalid pointer map code");
        }
        code
    }
}

/// The poison every payload word of a freed block holds in debug mode.
pub const POISON: u64 = 0xdead_beef_dead_beef;

/// Return a heap block's memory to the allocator. The payload is not
/// inspected: a caller that owns resources inside it (a `Nat`'s limbs, a
/// buffer's data) releases them first. Under `CINCH_RC_DEBUG=1` the
/// block is poisoned instead (kind `KIND_FREED`, every payload word
/// [`POISON`]) and kept, so a later use finds the poison rather than a
/// reused block.
///
/// # Safety
///
/// `p` must be the payload pointer of a live, mortal heap block allocated
/// by [`allocate`] or [`allocate_shaped`], not used again afterwards.
pub unsafe fn free(p: *mut u8) {
    let h = unsafe { read_header(p) };
    let Some((payload, ext)) = (unsafe { payload_bytes(p, h) }) else {
        crate::abi::panic_exit(b"compiler bug: shaped block with no registered layout");
    };
    if crate::heap::rc_debug() {
        let words = p.cast::<u64>();
        for i in 0..payload / 8 {
            unsafe { words.add(i).write(POISON) };
        }
        unsafe { write_header(p, header(RC_IMMORTAL, KIND_FREED, layout(h))) };
        return;
    }
    let Some(layout) = plan(payload, ext) else {
        crate::abi::panic_exit(b"compiler bug: block header corrupt");
    };
    let prefix = layout.size() - payload;
    unsafe { std::alloc::dealloc(p.sub(prefix), layout) };
}

/// A layout table for the unit tests. Entry `i < 200` records `i % 7`
/// unscanned payload words; the entries of [`test_shapes`] carry pointer
/// maps. Every test that needs a table installs this one, once, so tests
/// running side by side never see a registration in progress.
#[cfg(test)]
pub(crate) fn install_test_layouts() -> fn(u64) -> usize {
    use test_shapes::*;
    static TABLE: [u64; TABLE_LEN] = {
        let mut t = [0u64; TABLE_LEN];
        let mut i = 0;
        while i < 200 {
            t[i] = layout_entry(i as u64 % 7, 0);
            i += 1;
        }
        let mut s = 0;
        while s < SHAPES.len() {
            let (index, codes) = SHAPES[s];
            let mut map = 0;
            let mut j = 0;
            while j < codes.len() {
                map |= scan_slot(codes[j], j as u64);
                j += 1;
            }
            t[index as usize] = layout_entry(codes.len() as u64, map);
            s += 1;
        }
        // The wide shape's map lives in its extension words.
        t[WIDE as usize] = layout_entry(WIDE_WORDS, WIDE_EXT);
        let mut w = 0;
        while w < WIDE_WORDS {
            t[(WIDE_EXT + w / SCAN_EXT_WORDS) as usize] |= scan_slot(wide_code(w), w % SCAN_EXT_WORDS);
            w += 1;
        }
        t
    };
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| unsafe { register_layouts(TABLE.as_ptr(), TABLE.len()) });
    |index| index as usize % 7
}

/// The pointer-mapped entries of the unit tests' layout table.
#[cfg(test)]
pub(crate) mod test_shapes {
    use super::*;

    const R: u64 = SCAN_RAW;
    const O: u64 = SCAN_OBJ;
    const T: u64 = SCAN_TOBJ;

    /// `Cons` over a `TObj` head.
    pub const CONS_T: u64 = 200;
    /// `Cons` over an `Obj` head.
    pub const CONS_O: u64 = 201;
    /// `{ raw; obj }`.
    pub const PAIR_RO: u64 = 202;
    /// A zero-field constructor.
    pub const NIL: u64 = 203;
    /// Four counted fields, for the visiting order.
    pub const QUAD: u64 = 204;
    /// A closure shape over `(Obj, TObj, Raw)` parameters.
    pub const CLOSURE_OTR: u64 = 205;
    /// A dictionary `{ count; closure method; value method }`.
    pub const DICT_OR: u64 = 206;
    /// A 40-word constructor whose map needs an extension.
    pub const WIDE: u64 = 207;
    pub const WIDE_WORDS: u64 = 40;
    /// Where the wide shape's extension starts.
    pub const WIDE_EXT: u64 = 250;
    pub const TABLE_LEN: usize = (WIDE_EXT + scan_ext_len(WIDE_WORDS)) as usize;

    /// The wide shape's code of word `w`: `Obj`, `TObj`, `Raw` in turn.
    pub const fn wide_code(w: u64) -> u64 {
        match w % 3 {
            0 => O,
            1 => T,
            _ => R,
        }
    }

    pub(super) const SHAPES: [(u64, &[u64]); 7] = [
        (CONS_T, &[T, O]),
        (CONS_O, &[O, O]),
        (PAIR_RO, &[R, O]),
        (NIL, &[]),
        (QUAD, &[O, O, O, O]),
        (CLOSURE_OTR, &[R, R, R, O, T, R]),
        (DICT_OR, &[R, O, R]),
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_fields_round_trip_at_their_extremes() {
        let h = header(-5, KIND_CLOSURE, 0x1234);
        assert_eq!((rc(h), kind(h), layout(h)), (-5, KIND_CLOSURE, 0x1234));
        // The full count range survives, and no field bleeds into its
        // neighbours.
        assert_eq!(RC_MAX, (1i64 << 39) - 1);
        for rcv in [0, 1, -1, RC_MAX, RC_MIN] {
            let h = header(rcv, KIND_FREED, u16::MAX);
            assert_eq!((rc(h), kind(h), layout(h)), (rcv, KIND_FREED, u16::MAX));
            let h = header(rcv, KIND_NONE, 0);
            assert_eq!((rc(h), kind(h), layout(h)), (rcv, KIND_NONE, 0));
        }
        let s = static_header(KIND_CTOR, 1);
        assert_eq!((rc(s), kind(s), layout(s)), (RC_IMMORTAL, KIND_CTOR, 1));
    }

    #[test]
    fn small_and_big_blocks_record_their_size() {
        for (size, want) in [
            (0usize, 8usize),
            (1, 8),
            (24, 24),
            (8 * 0xFFFE, 8 * 0xFFFE),
            (8 * 0xFFFF, 8 * 0xFFFF),
            (1 << 20, 1 << 20),
        ] {
            let p = unsafe { allocate(size, KIND_RAW, RC_FRESH, true) };
            assert_eq!(p as usize % 8, 0);
            let h = unsafe { read_header(p) };
            assert_eq!((rc(h), kind(h)), (RC_FRESH, KIND_RAW));
            let big = want / 8 >= 0xFFFF;
            assert_eq!(unsafe { payload_bytes(p, h) }, Some((want, big)), "size {size}");
            assert_eq!(layout(h) == LAYOUT_BIG, big);
            // The whole payload is usable, including its last byte.
            unsafe { p.add(want - 1).write(0xAB) };
            unsafe { free(p) };
        }
    }

    #[test]
    fn layout_ids_carry_the_tag_at_every_group() {
        assert_eq!(CTOR_GROUPS, 1023);
        for group in [0, 1, 17, CTOR_GROUPS - 1] {
            for tag in [0, 1, CTOR_TAG_MASK] {
                let id = ctor_id(group, tag);
                assert!(id < LAYOUT_BIG as u64);
                let field = ctor_layout_field(id);
                assert_eq!(field as u64, id);
                assert_eq!(ctor_tag(field, 0), tag);
                assert_eq!(ctor_index(field, 0), id);
                // `tag = id - base`, with the group's base.
                assert_eq!(id - (group << CTOR_TAG_BITS), tag);
            }
        }
        for (index, tag) in [(0u64, 0u64), (5, 64), (70_000, (1 << CTOR_ESC_TAG_BITS) - 1)] {
            let d = ctor_escaped(index, tag);
            assert_eq!(ctor_layout_field(d), LAYOUT_BIG);
            assert_eq!(ctor_tag(LAYOUT_BIG, d), tag);
            assert_eq!(ctor_index(LAYOUT_BIG, d), index);
        }
    }

    #[test]
    fn constructor_blocks_size_from_the_layout_table() {
        let words_of = install_test_layouts();
        // Regular ids: the header holds the id, the table the size; an
        // escaped descriptor adds the extra word and keeps its tag there.
        for (desc, tag) in [
            (ctor_id(0, 0), 0),
            (ctor_id(0, 3), 3),
            (ctor_id(2, 5), 5),
            (ctor_escaped(150, 64), 64),
            (ctor_escaped(13, 9000), 9000),
        ] {
            let index = ctor_index(ctor_layout_field(desc), desc);
            let words = words_of(index);
            let p = unsafe { allocate_ctor(8 * words, desc, true) };
            let h = unsafe { read_header(p) };
            assert_eq!((rc(h), kind(h)), (RC_FRESH, KIND_CTOR));
            assert_eq!(unsafe { ctor_tag_of(p) }, tag);
            let big = layout(h) == LAYOUT_BIG;
            assert_eq!(big, desc >= LAYOUT_BIG as u64);
            assert_eq!(unsafe { payload_bytes(p, h) }, Some((words.max(1) * 8, big)));
            unsafe { p.add(words.max(1) * 8 - 1).write(0xAB) };
            unsafe { free(p) };
        }
    }
}
