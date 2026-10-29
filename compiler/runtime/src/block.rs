//! The block header and the header-aware allocator.
//!
//! Every Cinch heap block, whether generated code or this runtime allocates
//! it, and every static block the code generator emits, carries one header
//! word at offset -8 from the pointer its word holds. Every field offset
//! generated code and the runtime use stays relative to that pointer, so
//! the header is invisible to them. The encoding (rc, kind, layout) is in
//! `block_kinds.rs`, which the code generator includes verbatim.
//!
//! [`allocate`] and [`free`] are the only functions that touch the backing
//! allocator, so replacing it (the system allocator today, through the
//! counting wrapper of [`crate::heap`]) is a change to these two alone.
//!
//! Until the layout table lands (design/reclamation.md section 3), the
//! `layout` field holds the payload size in words, so [`free`] rebuilds the
//! allocation's `Layout` from the header alone. Payloads of [`LAYOUT_BIG`]
//! words or more keep their byte size in one extra word before the header.
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

/// The rounded payload size and full allocation layout for a request of
/// `size` bytes, or `None` on overflow. Payloads are whole words, minimum
/// one, so every block keeps the 8-byte alignment the immediate tag
/// relies on.
fn plan(size: usize) -> Option<(usize, Layout)> {
    let payload = match size.checked_add(7)? & !7 {
        0 => 8,
        s => s,
    };
    let prefix = if payload / 8 >= LAYOUT_BIG as usize {
        2 * HEADER_BYTES
    } else {
        HEADER_BYTES
    };
    let layout = Layout::from_size_align(payload.checked_add(prefix)?, 8).ok()?;
    Some((payload, layout))
}

/// Allocate a block with a `size`-byte payload (rounded up to whole words,
/// minimum one) and write its header with `rc` and `kind`. Returns the
/// payload pointer. The payload is zero-filled when `zeroed`, otherwise
/// uninitialised. Overflow and allocation failure fault (exit 71).
///
/// # Safety
///
/// The caller initialises every payload word it later reads.
pub unsafe fn allocate(size: usize, kind: u8, rc: i64, zeroed: bool) -> *mut u8 {
    let Some((payload, layout)) = plan(size) else {
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
    let words = payload / 8;
    let (p, field) = if words >= LAYOUT_BIG as usize {
        unsafe { base.cast::<u64>().write(payload as u64) };
        (unsafe { base.add(2 * HEADER_BYTES) }, LAYOUT_BIG)
    } else {
        (unsafe { base.add(HEADER_BYTES) }, words as u16)
    };
    unsafe { write_header(p, header(rc, kind, field)) };
    crate::heap::account_block(layout.size(), rc == RC_IMMORTAL);
    p
}

/// Move `value` into a fresh header block of `kind` and count `rc`, the
/// shape of every runtime-owned box.
pub fn allocate_value<T>(kind: u8, rc: i64, value: T) -> *mut T {
    const { assert!(std::mem::align_of::<T>() <= 8) };
    let p = unsafe { allocate(std::mem::size_of::<T>(), kind, rc, false) }.cast::<T>();
    unsafe { p.write(value) };
    p
}

/// The payload size in bytes recorded in the header `h` of the block at
/// `p`.
///
/// # Safety
///
/// `p` must be the payload pointer of a heap header block whose header is
/// `h`.
pub unsafe fn payload_bytes(p: *const u8, h: u64) -> usize {
    match layout(h) {
        LAYOUT_BIG => unsafe { p.cast::<u64>().sub(2).read() as usize },
        words => words as usize * 8,
    }
}

/// Return a heap block's memory to the allocator. The payload is not
/// inspected: a caller that owns resources inside it (a `Nat`'s limbs, a
/// buffer's data) releases them first.
///
/// # Safety
///
/// `p` must be the payload pointer of a live, mortal heap block allocated
/// by [`allocate`], not used again afterwards.
pub unsafe fn free(p: *mut u8) {
    let h = unsafe { read_header(p) };
    let payload = unsafe { payload_bytes(p, h) };
    let Some((_, layout)) = plan(payload) else {
        crate::abi::panic_exit(b"block header corrupt");
    };
    let prefix = layout.size() - payload;
    unsafe { std::alloc::dealloc(p.sub(prefix), layout) };
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
            assert_eq!(unsafe { payload_bytes(p, h) }, want, "size {size}");
            assert_eq!(layout(h) == LAYOUT_BIG, want / 8 >= 0xFFFF);
            // The whole payload is usable, including its last byte.
            unsafe { p.add(want - 1).write(0xAB) };
            unsafe { free(p) };
        }
    }
}
