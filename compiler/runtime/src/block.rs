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
//! A shaped block (a constructor, closure or dictionary) describes
//! itself (design/self-describing-blocks.md): its header's layout field is
//! its tag and the counted mask of its fields, or, when they do not fit,
//! the escape tag with a pointer to a static descriptor in the word before
//! the header. There is no layout table. Every other kind's `layout` is its
//! payload size in words, with [`LAYOUT_BIG`] putting the byte size in the
//! extra word instead.
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
pub const fn layout(h: u64) -> u32 {
    header_layout(h)
}

/// Payload sizes of the shaped blocks the small-object heap could not hold
/// (its address range used up, or a thread past its teardown), by payload
/// address. An inline shaped block's size is not in its header, and a block
/// of the system allocator is freed with its size; this rare path is the
/// one that needs it recorded.
static FALLBACK_SIZES: std::sync::LazyLock<std::sync::Mutex<std::collections::HashMap<usize, usize>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));

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
        (words as u32, None)
    };
    unsafe { allocate_block(payload, header(rc, kind, field), ext, rc == RC_IMMORTAL, zeroed) }
}

/// Allocate a fresh block of a shaped kind (`kind_has_shape`) with `size`
/// payload bytes whose layout field is `field` (its tag and counted mask,
/// [`shape_field`]). Returns the payload pointer.
///
/// # Safety
///
/// As for [`allocate`]; `field` describes this payload, and a constructor
/// or dictionary of more than [`SHAPED_SMALL_WORDS`] payload words is
/// escaped instead ([`allocate_shaped_desc`]).
pub unsafe fn allocate_shaped(size: usize, kind: u8, field: u32, zeroed: bool) -> *mut u8 {
    let Some(payload) = round_payload(size) else {
        crate::heap::request_too_large(size);
    };
    if shape_escaped(field) {
        crate::abi::panic_exit(b"compiler bug: an escaped shaped block allocated without its descriptor");
    }
    let p = unsafe { allocate_block(payload, header(RC_FRESH, kind, field), None, false, zeroed) };
    // A closure's size is its prefix and filled count; a constructor's or a
    // dictionary's is the small heap's to know, and one the system holds
    // instead is recorded for its free.
    // An arena's blocks are immortal and freed with their chunk, so only a
    // mortal block of the system allocator needs the record.
    if kind != KIND_CLOSURE
        && !crate::small::owns(unsafe { p.sub(HEADER_BYTES) })
        && rc(unsafe { read_header(p) }) != RC_IMMORTAL
    {
        FALLBACK_SIZES.lock().unwrap_or_else(|e| e.into_inner()).insert(p as usize, payload);
    }
    p
}

/// [`allocate_shaped`] of an escaped block: its layout field is the escape
/// tag and the word before its header points to `desc`, a static
/// descriptor (`DESC_*`) that outlives the block.
///
/// # Safety
///
/// As for [`allocate`]; `desc` describes this payload.
pub unsafe fn allocate_shaped_desc(size: usize, kind: u8, desc: *const u64, zeroed: bool) -> *mut u8 {
    let Some(payload) = round_payload(size) else {
        crate::heap::request_too_large(size);
    };
    unsafe { allocate_block(payload, header(RC_FRESH, kind, ESCAPED_FIELD), Some(desc as u64), false, zeroed) }
}

/// Allocate a fresh constructor block: [`allocate_shaped`] of `KIND_CTOR`.
///
/// # Safety
///
/// As for [`allocate_shaped`].
pub unsafe fn allocate_ctor(size: usize, field: u32, zeroed: bool) -> *mut u8 {
    unsafe { allocate_shaped(size, KIND_CTOR, field, zeroed) }
}

/// Allocate an escaped constructor block: [`allocate_shaped_desc`] of
/// `KIND_CTOR`.
///
/// # Safety
///
/// As for [`allocate_shaped_desc`].
pub unsafe fn allocate_ctor_desc(size: usize, desc: *const u64, zeroed: bool) -> *mut u8 {
    unsafe { allocate_shaped_desc(size, KIND_CTOR, desc, zeroed) }
}

/// A static descriptor for a block of `words` payload words of tag `tag`
/// whose counted payload words are the low bits of `mask`, interned so one
/// description has one descriptor for the life of the process. A block
/// whose tag arrives at run time is the only one code generation cannot
/// describe ahead of time.
pub fn interned_descriptor(tag: u64, words: u64, mask: u64) -> *const u64 {
    static INTERNED: std::sync::LazyLock<std::sync::Mutex<std::collections::HashMap<(u64, u64, u64), usize>>> =
        std::sync::LazyLock::new(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    let mut table = INTERNED.lock().unwrap_or_else(|e| e.into_inner());
    let at = *table.entry((tag, words, mask)).or_insert_with(|| {
        let mut d = vec![0u64; desc_words(words) as usize];
        d[DESC_TAG_WORD] = tag;
        d[DESC_WORDS_WORD] = words;
        if words > 0 {
            d[DESC_MASK_WORD] = if words >= 64 { mask } else { mask & ((1u64 << words) - 1) };
        }
        Box::leak(d.into_boxed_slice()).as_ptr() as usize
    });
    at as *const u64
}

/// Allocate a shaped block from a description word ([`inline_description`]
/// or a static descriptor's address).
///
/// # Safety
///
/// As for [`allocate_shaped`] or [`allocate_shaped_desc`], whichever the
/// word names.
pub unsafe fn allocate_described(size: usize, kind: u8, description: u64, zeroed: bool) -> *mut u8 {
    if description & 1 == 1 {
        unsafe { allocate_shaped(size, kind, (description >> 1) as u32, zeroed) }
    } else {
        unsafe { allocate_shaped_desc(size, kind, description as usize as *const u64, zeroed) }
    }
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

/// The static descriptor of the escaped shaped block at `p`.
///
/// # Safety
///
/// `p` is the payload pointer of an escaped shaped block (heap or static).
unsafe fn descriptor(p: *const u8) -> &'static [u64] {
    let d = unsafe { ext_word(p) } as usize as *const u64;
    if d.is_null() || d as usize % 8 != 0 {
        crate::abi::panic_exit(b"compiler bug: escaped block with no descriptor");
    }
    let words = unsafe { d.add(DESC_WORDS_WORD).read() };
    unsafe { std::slice::from_raw_parts(d, desc_words(words) as usize) }
}

/// The constructor tag of the constructor block at `p`.
///
/// # Safety
///
/// `p` must be the payload pointer of a constructor block (heap or static).
pub unsafe fn ctor_tag_of(p: *const u8) -> u64 {
    let l = layout(unsafe { read_header(p) });
    if shape_escaped(l) {
        (unsafe { descriptor(p) })[DESC_TAG_WORD]
    } else {
        shape_field_tag(l)
    }
}

/// The payload words of the closure at `p`: its prefix and filled count.
///
/// # Safety
///
/// `p` is the payload pointer of a closure block.
unsafe fn closure_words(p: *const u8) -> usize {
    let filled = unsafe { p.cast::<u64>().add(CLOSURE_FILLED_WORD as usize).read() };
    (CLOSURE_PREFIX_WORDS as usize).saturating_add(filled as usize)
}

/// The payload size in bytes of the heap block at `p` whose header is
/// `h`, and whether its prefix holds the extra word. `None` for an inline
/// constructor or dictionary that neither the small heap nor the fallback
/// record holds (a static or arena block, which nothing frees).
///
/// # Safety
///
/// `p` must be the payload pointer of a heap header block whose header is
/// `h`.
pub unsafe fn payload_bytes(p: *const u8, h: u64) -> Option<(usize, bool)> {
    let l = layout(h);
    let k = kind(h);
    if kind_has_shape(k) {
        let escaped = shape_escaped(l);
        if k == KIND_CLOSURE {
            return Some((unsafe { closure_words(p) }.checked_mul(8)?, escaped));
        }
        if escaped {
            let words = unsafe { descriptor(p) }[DESC_WORDS_WORD] as usize;
            return Some((words.max(1).checked_mul(8)?, true));
        }
        let base = unsafe { p.sub(HEADER_BYTES) };
        if crate::small::owns(base) {
            return Some((unsafe { crate::small::block_size(base) } - HEADER_BYTES, false));
        }
        let recorded = FALLBACK_SIZES.lock().unwrap_or_else(|e| e.into_inner()).get(&(p as usize)).copied();
        return recorded.map(|bytes| (bytes, false));
    }
    let big = l == LAYOUT_BIG;
    let ext = if big { unsafe { ext_word(p) } } else { 0 };
    Some((if big { ext as usize } else { l as usize * 8 }, big))
}

/// The scan codes of the low 32 bits of `mask`, one bit per word: every set
/// bit `i` becomes code [`SCAN_TOBJ`] at slot `i`.
#[inline(always)]
const fn counted_codes(mask: u64) -> u64 {
    // Spread the 32 low bits to the even bit positions, then move each to
    // its slot's high bit (`SCAN_TOBJ` is 2).
    let mut x = mask & 0xFFFF_FFFF;
    x = (x | (x << 16)) & 0x0000_FFFF_0000_FFFF;
    x = (x | (x << 8)) & 0x00FF_00FF_00FF_00FF;
    x = (x | (x << 4)) & 0x0F0F_0F0F_0F0F_0F0F;
    x = (x | (x << 2)) & 0x3333_3333_3333_3333;
    x = (x | (x << 1)) & 0x5555_5555_5555_5555;
    x << 1
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
    /// An array's element format, or zero.
    entry: u64,
    /// A wide escaped block's counted mask, one bit per payload word.
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
    /// `h`. A closure filled past its description, an invalid pointer map
    /// code or a malformed descriptor is a compiler bug and traps.
    ///
    /// # Safety
    ///
    /// `p` is the payload pointer of a header block whose header is `h`.
    #[inline(always)]
    pub unsafe fn of(p: *const u8, h: u64) -> Scan {
        let k = kind(h);
        if !kind_has_shape(k) {
            return unsafe { Scan::fixed(p, h, k) };
        }
        let l = layout(h);
        let prefix = shape_prefix_words(k) as usize;
        if !shape_escaped(l) {
            // The fields' mask, moved past the kind's prefix: a constructor
            // is scanned up to its last counted field, a closure up to its
            // filled count.
            let mut mask = shape_field_mask(l) << prefix;
            let words = if k == KIND_CLOSURE {
                let words = unsafe { closure_words(p) };
                if words < 64 {
                    mask &= (1u64 << words) - 1;
                }
                words
            } else {
                64 - mask.leading_zeros() as usize
            };
            return Scan { words, kind: k, entry: 0, ext: &[], map: counted_codes(mask), inline: true };
        }
        let desc = unsafe { descriptor(p) };
        let described = desc[DESC_WORDS_WORD] as usize;
        let words = if k == KIND_CLOSURE {
            let words = unsafe { closure_words(p) };
            if words > described {
                crate::abi::panic_exit(b"compiler bug: closure filled past its description");
            }
            words
        } else {
            described
        };
        let mask = &desc[DESC_MASK_WORD..];
        if words <= 32 {
            let low = mask.first().copied().unwrap_or(0) & if words == 32 { u32::MAX as u64 } else { (1u64 << words) - 1 };
            return Scan { words, kind: k, entry: 0, ext: &[], map: counted_codes(low), inline: true };
        }
        Scan { words, kind: k, entry: 0, ext: mask, map: 0, inline: false }
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
        } else if (self.ext[(slot / 64) as usize] >> (slot % 64)) & 1 == 1 {
            SCAN_TOBJ
        } else {
            SCAN_RAW
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
    #[cfg(test)]
    if kind(h) == KIND_ARRAY {
        crate::cycle::retire_array(p as usize);
    }
    if debug {
        // A frame block lives in a stack frame: freeing it would hand the
        // stack's memory to the allocator. Only a compiler bug reaches
        // here, because the compiler elides every frame block's free, and
        // its payload's release leaves the header marked (both markers).
        if matches!(rc(h), RC_FRAME | RC_FRAME_RELEASED) {
            crate::abi::panic_exit(b"compiler bug: a stack frame block reached the free path");
        }
        let Some((payload, _)) = (unsafe { payload_bytes(p, h) }) else {
            crate::abi::panic_exit(b"compiler bug: a shaped block of unknown size reached the free path");
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
    let escaped = if kind_has_shape(kind(h)) { shape_escaped(layout(h)) } else { layout(h) == LAYOUT_BIG };
    let prefix = if escaped { 2 * HEADER_BYTES } else { HEADER_BYTES };
    let base = unsafe { p.sub(prefix) };
    unsafe {
        crate::heap::free_block(local, base, || {
            let Some((payload, ext)) = payload_bytes(p, h) else {
                crate::abi::panic_exit(b"compiler bug: a shaped block of unknown size reached the free path");
            };
            if kind_has_shape(kind(h)) && !ext && kind(h) != KIND_CLOSURE {
                FALLBACK_SIZES.lock().unwrap_or_else(|e| e.into_inner()).remove(&(p as usize));
            }
            match plan(payload, ext) {
                Some(layout) => layout,
                None => crate::abi::panic_exit(b"compiler bug: block header corrupt"),
            }
        })
    }
}

/// Kept for the tests' call sites: there is no table to install any more.
#[cfg(test)]
pub(crate) fn install_test_layouts() {}

/// The shapes of the unit tests, as description words
/// ([`inline_description`], or a static descriptor's address).
#[cfg(test)]
pub(crate) mod test_shapes {
    use super::*;

    const R: u64 = SCAN_RAW;
    const O: u64 = SCAN_OBJ;
    const T: u64 = SCAN_TOBJ;

    /// `Cons` (tag 1) over a `TObj` head.
    pub const CONS_T: u64 = inline_description(shape_field(1, 0b11));
    /// `Cons` (tag 1) over an `Obj` head.
    pub const CONS_O: u64 = inline_description(shape_field(1, 0b11));
    /// `{ raw; obj }`.
    pub const PAIR_RO: u64 = inline_description(shape_field(0, 0b10));
    /// A zero-field constructor.
    pub const NIL: u64 = inline_description(shape_field(0, 0));
    /// Four counted fields, for the visiting order.
    pub const QUAD: u64 = inline_description(shape_field(0, 0b1111));
    /// A closure over `(Obj, TObj, Raw)` parameters.
    pub const CLOSURE_OTR: u64 = inline_description(shape_field(0, 0b011));
    /// A dictionary `{ count; closure method; value method }`.
    pub const DICT_OR: u64 = inline_description(shape_field(0, 0b01));
    /// A 40-word constructor, escaped to its descriptor.
    pub const WIDE_WORDS: u64 = 40;

    /// The header layout field of a test shape's description word.
    pub const fn field_of(description: u64) -> u32 {
        (description >> 1) as u32
    }

    /// The wide shape's code of word `w`: `Obj`, `TObj`, `Raw` in turn.
    pub const fn wide_code(w: u64) -> u64 {
        match w % 3 {
            0 => O,
            1 => T,
            _ => R,
        }
    }

    static WIDE_DESC: [u64; desc_words(WIDE_WORDS) as usize] = {
        let mut d = [0u64; desc_words(WIDE_WORDS) as usize];
        d[DESC_TAG_WORD] = 0;
        d[DESC_WORDS_WORD] = WIDE_WORDS;
        let mut w = 0;
        while w < WIDE_WORDS {
            if wide_code(w) != R {
                d[DESC_MASK_WORD + (w / 64) as usize] |= 1 << (w % 64);
            }
            w += 1;
        }
        d
    };

    /// The wide shape's description word: its descriptor's address.
    pub fn wide() -> u64 {
        WIDE_DESC.as_ptr() as usize as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_fields_round_trip_at_their_extremes() {
        let h = header(-5, KIND_CLOSURE, 0x12_3456);
        assert_eq!((rc(h), kind(h), layout(h)), (-5, KIND_CLOSURE, 0x12_3456));
        // The full count range survives, and no field bleeds into its
        // neighbours.
        assert_eq!(RC_MAX, (1i64 << 34) - 1);
        for rcv in [0, 1, -1, RC_MAX, RC_MIN] {
            let h = header(rcv, KIND_FREED, LAYOUT_BIG);
            assert_eq!((rc(h), kind(h), layout(h)), (rcv, KIND_FREED, LAYOUT_BIG));
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
            (8 * 0xFFFF, 8 * 0xFFFF),
            (8 * 0x10000, 8 * 0x10000),
            (1 << 20, 1 << 20),
        ] {
            let p = unsafe { allocate(size, KIND_RAW, RC_FRESH, true) };
            assert_eq!(p as usize % 8, 0);
            let h = unsafe { read_header(p) };
            assert_eq!((rc(h), kind(h)), (RC_FRESH, KIND_RAW));
            let big = want / 8 >= LAYOUT_BIG as usize;
            assert_eq!(unsafe { payload_bytes(p, h) }, Some((want, big)), "size {size}");
            assert_eq!(layout(h) == LAYOUT_BIG, big);
            // The whole payload is usable, including its last byte.
            unsafe { p.add(want - 1).write(0xAB) };
            unsafe { free(p) };
        }
    }

    #[test]
    fn shape_fields_carry_the_tag_and_the_mask() {
        for tag in [0, 1, 7, SHAPE_ESCAPED - 1] {
            for mask in [0, 1, 0b1010, 0xFFFF] {
                let f = shape_field(tag, mask);
                assert_eq!((shape_field_tag(f), shape_field_mask(f)), (tag, mask));
                assert!(!shape_escaped(f));
                let h = header(RC_FRESH, KIND_CTOR, f);
                assert_eq!(layout(h), f);
            }
        }
        assert!(shape_escaped(ESCAPED_FIELD));
        assert_eq!(inline_description(shape_field(3, 1)) & 1, 1);
    }

    #[test]
    fn constructors_read_their_tag_inline_or_from_their_descriptor() {
        use test_shapes::*;
        // Inline: the small heap knows the size, the header the tag.
        for (field, words, tag) in [(shape_field(0, 0), 0usize, 0u64), (shape_field(3, 0b1), 1, 3), (shape_field(5, 0b11), 2, 5)] {
            let p = unsafe { allocate_ctor(8 * words, field, true) };
            let h = unsafe { read_header(p) };
            assert_eq!((rc(h), kind(h)), (RC_FRESH, KIND_CTOR));
            assert_eq!(unsafe { ctor_tag_of(p) }, tag);
            assert_eq!(unsafe { payload_bytes(p, h) }, Some((words.max(1) * 8, false)));
            unsafe { free(p) };
        }
        // Escaped: the descriptor has the size and the tag.
        let p = unsafe { allocate_described(8 * WIDE_WORDS as usize, KIND_CTOR, wide(), true) };
        let h = unsafe { read_header(p) };
        assert!(shape_escaped(layout(h)));
        assert_eq!(unsafe { ctor_tag_of(p) }, 0);
        assert_eq!(unsafe { payload_bytes(p, h) }, Some((8 * WIDE_WORDS as usize, true)));
        unsafe { p.add(8 * WIDE_WORDS as usize - 1).write(0xAB) };
        unsafe { free(p) };
    }

    #[test]
    fn a_counted_mask_scans_exactly_its_fields() {
        let p = unsafe { allocate_ctor(8 * 4, shape_field(1, 0b1010), true) };
        let scan = unsafe { Scan::of(p, read_header(p)) };
        assert_eq!(scan.words(), 4);
        assert_eq!(scan.counted_below(4), Some((3, SCAN_TOBJ)));
        assert_eq!(scan.counted_below(3), Some((1, SCAN_TOBJ)));
        assert_eq!(scan.counted_below(1), None);
        unsafe { free(p) };
    }
}
