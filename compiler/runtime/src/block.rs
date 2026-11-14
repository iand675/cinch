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
//! backing allocator, through [`crate::heap::alloc_block`] and
//! [`crate::heap::free_block`] (the small-object heap of [`crate::small`]
//! for every block of up to 256 bytes, the system allocator above that).
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
/// Header words are read and written as relaxed atomics: a block marked
/// for sharing across tasks (`crate::rc`, negative counts) has its count
/// changed by atomic read-modify-writes on other threads while this one
/// reads its kind or layout. A relaxed access is an ordinary load or
/// store on every supported target, so a block no other thread can reach
/// pays nothing for it; the ordering the counts need is in `crate::rc`.
///
/// # Safety
///
/// `p` must be the payload pointer of a header block (heap or static).
#[inline]
pub unsafe fn read_header(p: *const u8) -> u64 {
    unsafe { header_cell(p) }.load(std::sync::atomic::Ordering::Relaxed)
}

/// Overwrite the header word of the block whose payload starts at `p`.
///
/// # Safety
///
/// `p` must be the payload pointer of a heap header block.
#[inline]
pub unsafe fn write_header(p: *mut u8, h: u64) {
    unsafe { header_cell(p) }.store(h, std::sync::atomic::Ordering::Relaxed)
}

/// The header word of the block at `p` as an atomic. A static block's
/// header is read-only data: only relaxed loads may touch it, which
/// pointer-sized atomics allow on read-only memory.
///
/// # Safety
///
/// `p` must be the payload pointer of a header block (heap or static).
#[inline]
pub unsafe fn header_cell<'a>(p: *const u8) -> &'a std::sync::atomic::AtomicU64 {
    unsafe { std::sync::atomic::AtomicU64::from_ptr(p.cast::<u64>().sub(1).cast_mut()) }
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
pub(crate) fn layout_table() -> &'static [u64] {
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
/// `header`, and account for it. Returns the payload pointer. A payload
/// too large for any allocation layout is the heap `Exhausted` exit; the
/// counting global allocator ([`crate::heap`]) never answers null, so a
/// refused allocation never returns here either.
unsafe fn allocate_block(payload: usize, header: u64, ext: Option<u64>, immortal: bool, zeroed: bool) -> *mut u8 {
    let Some(layout) = plan(payload, ext.is_some()) else {
        crate::heap::request_too_large(payload);
    };
    // An entered arena ([`crate::arena`]) serves the kinds whose payload
    // no runtime destructor owns, and only when the caller does not ask
    // for zeroed memory (the shaped allocators never do). The block is a
    // full header block with its own kind and layout and `RC_IMMORTAL`
    // as its count, so every count path ignores it and the arena's leave
    // frees it with its chunk.
    let kind = header_kind(header);
    if !zeroed && matches!(kind, KIND_CTOR | KIND_CLOSURE | KIND_DICT) {
        let prefix = layout.size() - payload;
        if let Some(p) = unsafe { crate::arena::serve(layout.size(), prefix) } {
            if let Some(x) = ext {
                unsafe { p.sub(prefix).cast::<u64>().write(x) };
            }
            unsafe { write_header(p, crate::arena::arena_header(kind, header_layout(header))) };
            crate::alloc_kinds::note_arena(kind, layout.size());
            return p;
        }
    }
    let base = unsafe { crate::heap::alloc_block(layout, immortal, zeroed) };
    crate::alloc_kinds::note(header_kind(header), layout.size());
    let p = match ext {
        Some(x) => {
            unsafe { base.cast::<u64>().write(x) };
            unsafe { base.add(2 * HEADER_BYTES) }
        }
        None => unsafe { base.add(HEADER_BYTES) },
    };
    unsafe { write_header(p, header) };
    p
}

/// Allocate a block with a `size`-byte payload (rounded up to whole words,
/// minimum one) and write its header with `rc` and `kind`; the layout
/// field is the payload size. Returns the payload pointer. The payload is
/// zero-filled when `zeroed`, otherwise uninitialised. A size no
/// allocation can hold, and allocation failure, are the heap `Exhausted`
/// exit. The shaped kinds (constructors, closures, dictionaries) go
/// through [`allocate_shaped`].
///
/// # Safety
///
/// The caller initialises every payload word it later reads.
pub unsafe fn allocate(size: usize, kind: u8, rc: i64, zeroed: bool) -> *mut u8 {
    let Some(payload) = round_payload(size) else {
        crate::heap::request_too_large(size);
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
        crate::heap::request_too_large(size);
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
///
/// Every block whose codes fit one word (a shaped block whose entry
/// holds its map, which is every block of at most 16 words, and every
/// fixed kind but the arrays, quote blocks, tensors and column trees)
/// keeps them in `map`, validated once, so [`Scan::counted_below`] finds
/// the next counted slot with one bit search instead of decoding every
/// slot. The rest decode slot by slot.
pub struct Scan {
    words: usize,
    kind: u8,
    /// A shaped block's table entry, an array's element format, or zero.
    entry: u64,
    /// A wide entry's extension words.
    ext: &'static [u64],
    /// The codes of the payload words below `words`, `SCAN_BITS` each,
    /// when `inline`.
    map: u64,
    inline: bool,
}

/// The map bits of the codes of the first `words` slots.
#[inline]
const fn slots_below(words: usize) -> u64 {
    if words >= (64 / SCAN_BITS) as usize {
        !0
    } else {
        (1u64 << (SCAN_BITS as usize * words)) - 1
    }
}

/// The low bit of every code in a map word.
const CODE_LOW_BITS: u64 = 0x5555_5555_5555_5555;
const _: () = assert!(SCAN_BITS == 2 && SCAN_TOBJ == 2 && SCAN_CODE_MASK == 3);

impl Scan {
    /// The scan of the live heap or static block at `p` whose header is
    /// `h`. A shaped block whose layout the table does not record, a
    /// closure filled past its shape, an invalid pointer map code or a
    /// truncated extension is a compiler bug and traps.
    ///
    /// # Safety
    ///
    /// `p` is the payload pointer of a header block whose header is `h`.
    #[inline]
    pub unsafe fn of(p: *const u8, h: u64) -> Scan {
        unsafe { Scan::with(p, h, layout_table()) }
    }

    /// [`Scan::of`] against `table`, the registered layout table, which a
    /// walk visiting many blocks reads once.
    ///
    /// # Safety
    ///
    /// As for [`Scan::of`]; `table` is [`layout_table`]'s answer.
    #[inline(always)]
    pub(crate) unsafe fn with(p: *const u8, h: u64, table: &'static [u64]) -> Scan {
        let k = kind(h);
        if !kind_has_shape(k) {
            return unsafe { Scan::fixed(p, h, k) };
        }
        let l = layout(h);
        let ext = if l == LAYOUT_BIG { unsafe { ext_word(p) } } else { 0 };
        let Some(&entry) = usize::try_from(ctor_index(l, ext)).ok().and_then(|i| table.get(i)) else {
            crate::abi::panic_exit(b"compiler bug: shaped block with no registered layout");
        };
        let mapped = layout_entry_words(entry) as usize;
        // A constructor or dictionary is scanned over its mapped words: a
        // zero-field constructor's one allocated word holds nothing.
        let words = if k == KIND_CLOSURE {
            let words = unsafe { shaped_words(p, h, entry) };
            if words > mapped {
                crate::abi::panic_exit(b"compiler bug: closure filled past its shape");
            }
            words
        } else {
            mapped
        };
        if layout_entry_inline(entry) {
            let map = layout_entry_map(entry) & slots_below(words);
            return Scan::mapped(words, k, entry, map);
        }
        let start = layout_entry_map(entry) as usize;
        let n = scan_ext_len(mapped as u64) as usize;
        let Some(ext) = start.checked_add(n).and_then(|end| table.get(start..end)) else {
            crate::abi::panic_exit(b"compiler bug: layout map extension out of range");
        };
        Scan { words, kind: k, entry, ext, map: 0, inline: false }
    }

    /// The scan of a block whose kind fixes its map.
    #[inline(never)]
    unsafe fn fixed(p: *const u8, h: u64, k: u8) -> Scan {
        let Some((bytes, _)) = (unsafe { payload_bytes(p, h) }) else {
            crate::abi::panic_exit(b"compiler bug: block header corrupt");
        };
        let words = bytes / 8;
        match k {
            KIND_ARRAY => {
                let entry = unsafe { p.cast::<u64>().read() };
                Scan { words, kind: k, entry, ext: &[], map: 0, inline: false }
            }
            KIND_TENSOR | KIND_COLUMNAR | KIND_CODE => Scan { words, kind: k, entry: 0, ext: &[], map: 0, inline: false },
            KIND_MEMO => Scan::mapped(words, k, 0, scan_slot(SCAN_OBJ, MEMO_OWNED_WORD) & slots_below(words)),
            KIND_TVAR => {
                // The format word is written once, before the cell is
                // handed out, so reading it here races with nothing.
                let code = unsafe { p.cast::<u64>().add(TVAR_FORMAT_WORD as usize).read() };
                if code > SCAN_TOBJ || words != TVAR_WORDS as usize {
                    crate::abi::panic_exit(b"compiler bug: corrupt transactional variable cell");
                }
                Scan::mapped(words, k, 0, scan_slot(code, TVAR_VALUE_WORD))
            }
            _ => Scan::mapped(words, k, 0, 0),
        }
    }

    /// A scan whose codes are `map`, trapping on an invalid code.
    #[inline(always)]
    fn mapped(words: usize, kind: u8, entry: u64, map: u64) -> Scan {
        if map & (map >> 1) & CODE_LOW_BITS != 0 {
            crate::abi::panic_exit(b"compiler bug: invalid pointer map code");
        }
        Scan { words, kind, entry, ext: &[], map, inline: true }
    }

    /// The number of payload words to visit.
    pub fn words(&self) -> usize {
        self.words
    }

    /// The code (`SCAN_*`) of payload word `slot` (below [`Scan::words`]).
    /// An invalid code is a compiler bug and traps.
    #[inline]
    pub fn code(&self, slot: usize) -> u64 {
        if self.inline {
            return scan_code_at(self.map, slot as u64);
        }
        let slot = slot as u64;
        let code = if self.kind == KIND_ARRAY {
            if slot >= 2 && self.entry <= SCAN_TOBJ { self.entry } else { SCAN_RAW }
        } else if !kind_has_shape(self.kind) {
            fixed_scan_code(self.kind, slot)
        } else {
            scan_code_at(self.ext[(slot / SCAN_EXT_WORDS) as usize], slot % SCAN_EXT_WORDS)
        };
        if code > SCAN_TOBJ {
            crate::abi::panic_exit(b"compiler bug: invalid pointer map code");
        }
        code
    }

    /// The highest slot below `below` whose code is not `SCAN_RAW`, with
    /// that code: the next slot a reverse walk visits.
    #[inline(always)]
    pub fn counted_below(&self, below: usize) -> Option<(usize, u64)> {
        if !self.inline {
            // An array's map is one code for every element slot (word 2 on)
            // and raw below it ([`Scan::code`]), so the next counted slot is
            // the one just below the cursor, and a raw or packed array has
            // none. Answered here, inline, a counted array's release costs a
            // few instructions per slot and a raw array's nothing, instead of
            // re-deriving each slot's code out of line.
            if self.kind == KIND_ARRAY {
                if self.entry == SCAN_RAW || self.entry > SCAN_TOBJ {
                    return None;
                }
                return (below > 2).then(|| (below - 1, self.entry));
            }
            return self.decode_below(below);
        }
        let m = self.map & slots_below(below);
        if m == 0 {
            return None;
        }
        let slot = (63 - m.leading_zeros() as usize) / SCAN_BITS as usize;
        Some((slot, scan_code_at(m, slot as u64)))
    }

    /// [`Scan::counted_below`] for a block whose codes are decoded slot
    /// by slot.
    #[inline(never)]
    fn decode_below(&self, below: usize) -> Option<(usize, u64)> {
        (0..below).rev().find_map(|slot| match self.code(slot) {
            SCAN_RAW => None,
            code => Some((slot, code)),
        })
    }
}

/// The poison every payload word of a freed block holds in debug mode.
pub const POISON: u64 = 0xdead_beef_dead_beef;

/// Return a heap block's memory to the allocator and count it freed. The
/// payload is not inspected: a caller that owns resources inside it (a
/// `Nat`'s limbs, a buffer's data) releases them first. Under
/// `CINCH_RC_DEBUG=1` the block is poisoned instead (kind `KIND_FREED`,
/// every payload word [`POISON`]) and kept, so a later use finds the
/// poison rather than a reused block.
///
/// # Safety
///
/// `p` must be the payload pointer of a live, mortal heap block allocated
/// by [`allocate`] or [`allocate_shaped`], not used again afterwards.
pub unsafe fn free(p: *mut u8) {
    unsafe { free_in(p, read_header(p), crate::heap::rc_debug(), crate::small::local()) }
}

/// [`free`] of the block `p` whose header is `h`, for a caller that
/// already knows whether debug mode is on and the calling thread's small
/// heap ([`crate::small::local`]), as the release walk does once for
/// every block it frees.
///
/// # Safety
///
/// As for [`free`]; `local` is the calling thread's heap, or `None`.
#[inline(always)]
pub(crate) unsafe fn free_in(p: *mut u8, h: u64, debug: bool, local: Option<&crate::small::Heap>) {
    if debug {
        // A frame block lives in a stack frame: freeing it would hand the
        // stack's memory to the allocator. Only a compiler bug reaches
        // here, because the compiler elides every frame block's free, and
        // its payload's release leaves the header marked (both markers).
        if matches!(rc(h), RC_FRAME | RC_FRAME_RELEASED) {
            crate::abi::panic_exit(b"compiler bug: a stack frame block reached the free path");
        }
        let Some((payload, _)) = (unsafe { payload_bytes(p, h) }) else {
            crate::abi::panic_exit(b"compiler bug: shaped block with no registered layout");
        };
        let words = p.cast::<u64>();
        for i in 0..payload / 8 {
            unsafe { words.add(i).write(POISON) };
        }
        unsafe { write_header(p, header(RC_IMMORTAL, KIND_FREED, layout(h))) };
        crate::heap::account_block_free();
        return;
    }
    // The escape word, when there is one, is where the allocation starts.
    let prefix = if layout(h) == LAYOUT_BIG { 2 * HEADER_BYTES } else { HEADER_BYTES };
    let base = unsafe { p.sub(prefix) };
    unsafe {
        crate::heap::free_block(local, base, || {
            let Some((payload, ext)) = payload_bytes(p, h) else {
                crate::abi::panic_exit(b"compiler bug: shaped block with no registered layout");
            };
            match plan(payload, ext) {
                Some(layout) => layout,
                None => crate::abi::panic_exit(b"compiler bug: block header corrupt"),
            }
        })
    }
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
