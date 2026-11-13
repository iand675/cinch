//! Immutable arrays. Payloads live inside accounted header blocks. Generic
//! elements are ABI words; explicit scalar constructors pack at scalar width.
//! Tensor shares an array and an immutable shape. Columnar is a tree of
//! separate columns; zip never materializes rows.
use crate::{abi, block, rc};

fn corrupt() -> ! {
    abi::panic_exit(b"compiler bug: corrupt array value")
}

unsafe fn payload(word: i64, kind: u8) -> *mut i64 {
    if word == 0 || word & 7 != 0 {
        corrupt();
    }
    let p = word as usize as *mut u8;
    if block::kind(unsafe { block::read_header(p) }) != kind {
        corrupt();
    }
    p.cast()
}

unsafe fn field(word: i64, kind: u8, i: usize) -> i64 {
    unsafe { payload(word, kind).add(i).read() }
}

fn width(format: i64) -> usize {
    if format <= 2 {
        8
    } else {
        (format & 0x7f) as usize
    }
}
pub(crate) fn counted(format: i64) -> bool {
    format == 1 || format == 2
}

unsafe fn keep(word: i64, format: i64) -> i64 {
    if counted(format) {
        unsafe { rc::cinch_dup(word) };
    }
    word
}

pub(crate) unsafe fn array(len: i64, format: i64) -> i64 {
    if len < 0
        || !(0..=2).contains(&format)
            && !(format & 0x100 != 0 && matches!(width(format), 1 | 2 | 4 | 8))
    {
        corrupt();
    }
    let bytes = (len as usize)
        .checked_mul(width(format))
        .and_then(|n| n.checked_add(16))
        .unwrap_or_else(|| crate::heap::request_too_large(usize::MAX));
    let p =
        unsafe { block::allocate(bytes, block::KIND_ARRAY, block::RC_FRESH, true) }.cast::<i64>();
    unsafe {
        p.write(format);
        p.add(1).write(len);
    }
    p as usize as i64
}

pub(crate) unsafe fn format(a: i64) -> i64 {
    unsafe { field(a, block::KIND_ARRAY, 0) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_len(a: i64) -> i64 {
    unsafe { field(a, block::KIND_ARRAY, 1) }
}

pub(crate) unsafe fn read(a: i64, i: i64) -> i64 {
    let f = unsafe { format(a) };
    let p = unsafe {
        payload(a, block::KIND_ARRAY)
            .add(2)
            .cast::<u8>()
            .add(i as usize * width(f))
    };
    let v = match width(f) {
        1 => unsafe { p.read() as u64 },
        2 => unsafe { p.cast::<u16>().read_unaligned() as u64 },
        4 => unsafe { p.cast::<u32>().read_unaligned() as u64 },
        8 => unsafe { p.cast::<u64>().read_unaligned() },
        _ => corrupt(),
    };
    if f > 2 && f & 0x80 != 0 {
        let shift = 64 - width(f) * 8;
        ((v << shift) as i64) >> shift
    } else {
        v as i64
    }
}

// The destination owns this word. Only unpublished fresh arrays are
// written; the parallel entries write distinct slots of one such array
// from several participants before their join publishes it.
pub(crate) unsafe fn write(a: i64, i: i64, value: i64) {
    let f = unsafe { format(a) };
    let p = unsafe {
        payload(a, block::KIND_ARRAY)
            .add(2)
            .cast::<u8>()
            .add(i as usize * width(f))
    };
    match width(f) {
        1 => unsafe { p.write(value as u8) },
        2 => unsafe { p.cast::<u16>().write_unaligned(value as u16) },
        4 => unsafe { p.cast::<u32>().write_unaligned(value as u32) },
        8 => unsafe { p.cast::<i64>().write_unaligned(value) },
        _ => corrupt(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_from_list(xs: i64, f: i64) -> i64 {
    let mut n = 0i64;
    let mut at = xs;
    while unsafe { block::ctor_tag_of(at as usize as *const u8) } == 1 {
        n = n
            .checked_add(1)
            .unwrap_or_else(|| crate::heap::request_too_large(usize::MAX));
        at = unsafe { (at as usize as *const i64).add(1).read() };
    }
    let out = unsafe { array(n, f) };
    at = xs;
    for i in 0..n {
        let value = unsafe { (at as usize as *const i64).read() };
        unsafe {
            write(out, i, keep(value, f));
        }
        at = unsafe { (at as usize as *const i64).add(1).read() };
    }
    out
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_replicate(n: i64, value: i64, f: i64) -> i64 {
    let out = unsafe { array(n, f) };
    for i in 0..n {
        unsafe { write(out, i, keep(value, f)) };
    }
    out
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_has(a: i64, i: i64) -> i64 {
    i64::from(i >= 0 && i < unsafe { cinch_array_len(a) })
}

/// An owned element, after a separate successful bounds check.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_get(a: i64, i: i64) -> i64 {
    if unsafe { cinch_array_has(a, i) } == 0 {
        corrupt();
    }
    unsafe { keep(read(a, i), format(a)) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_slice(a: i64, start: i64, n: i64) -> i64 {
    if start < 0
        || n < 0
        || start
            .checked_add(n)
            .is_none_or(|end| end > unsafe { cinch_array_len(a) })
    {
        return 0;
    }
    let f = unsafe { format(a) };
    let out = unsafe { array(n, f) };
    for i in 0..n {
        unsafe { write(out, i, keep(read(a, start + i), f)) };
    }
    out
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_set(a: i64, i: i64, value: i64) -> i64 {
    if unsafe { cinch_array_has(a, i) } == 0 {
        return 0;
    }
    let f = unsafe { format(a) };
    let n = unsafe { cinch_array_len(a) };
    let out = unsafe { array(n, f) };
    for j in 0..n {
        let v = if i == j { value } else { unsafe { read(a, j) } };
        unsafe { write(out, j, keep(v, f)) };
    }
    out
}

/// Whether the caller's word is the only reference to this block: a fresh
/// single-threaded count of one. Immortal blocks (statics, interned cells,
/// arena blocks) are shared by definition and atomic counts (`-n`) mean more
/// than one thread reached the block, so neither is exclusive.
unsafe fn exclusive(word: i64) -> bool {
    let p = word as usize as *const u8;
    block::rc(unsafe { block::read_header(p) }) == block::RC_FRESH
}

/// Write `value` at `i` in place when this block is the caller's only
/// reference, for a call site the code generator selected because the array
/// operand is consumed there (`design/arraylist.md` section 2.5). The result is
/// `None` (0) when `i` is out of bounds.
///
/// # The contract is the copying entry's
///
/// Both entries borrow the array argument and return an owned reference, which
/// is what the wrapping unit imposes: `prim_consumes` does not list the array
/// operations, so `ret_drops` releases every counted parameter at `ret`. This
/// entry therefore `cinch_dup`s the block for its result, exactly as
/// [`cinch_array_set`] gets an owned result from a fresh block. The only
/// difference between them is whether the write allocates.
///
/// # Why the count is checked rather than asserted
///
/// The code generator selects this entry where it proved the array operand is
/// consumed, but consumption is a property of the reference, not of the block:
/// a program may hold two words for one array and consume one of them, which
/// is correct code with a count of two at this call. A trap would fire on a
/// good program, so the count is a runtime fact the entry acts on. One means no
/// other word can observe the write, so it happens in place; above one some
/// other word can, so it copies, which is the same result the copying entry
/// produces.
///
/// # What it does to the counts
///
/// In place: the slot takes its own reference to `value` (`keep`) before the
/// displaced element's reference is released, so writing a value the slot
/// already holds cannot free it before it is stored, and the result's
/// reference is taken with `cinch_dup`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_set_inplace(a: i64, i: i64, value: i64) -> i64 {
    if unsafe { cinch_array_has(a, i) } == 0 {
        return 0;
    }
    let f = unsafe { format(a) };
    if exclusive(a) {
        let old = unsafe { read(a, i) };
        unsafe { write(a, i, keep(value, f)) };
        if counted(f) {
            unsafe { rc::cinch_drop(old) };
        }
        unsafe { rc::cinch_dup(a) };
        return a;
    }
    let n = unsafe { cinch_array_len(a) };
    let out = unsafe { array(n, f) };
    for j in 0..n {
        let v = if i == j { value } else { unsafe { read(a, j) } };
        unsafe { write(out, j, keep(v, f)) };
    }
    out
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_array_append(a: i64, b: i64) -> i64 {
    let an = unsafe { cinch_array_len(a) };
    let bn = unsafe { cinch_array_len(b) };
    // A word representation is a common, lossless format, including when
    // either input uses packed scalar storage.
    let af = unsafe { format(a) };
    let bf = unsafe { format(b) };
    if counted(af) != counted(bf) || counted(af) && af != bf {
        corrupt();
    }
    let f = if af == bf { af } else { 0 };
    let n = an
        .checked_add(bn)
        .unwrap_or_else(|| crate::heap::request_too_large(usize::MAX));
    let out = unsafe { array(n, f) };
    for i in 0..an {
        unsafe { write(out, i, keep(read(a, i), f)) };
    }
    for i in 0..bn {
        unsafe { write(out, an + i, keep(read(b, i), f)) };
    }
    out
}

unsafe fn tensor(data: i64, shape: i64) -> i64 {
    let p =
        unsafe { block::allocate(16, block::KIND_TENSOR, block::RC_FRESH, false) }.cast::<i64>();
    unsafe {
        p.write(data);
        p.add(1).write(shape);
        rc::cinch_dup(data);
        rc::cinch_dup(shape);
    }
    p as usize as i64
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_tensor_reshape(shape: i64, data: i64) -> i64 {
    let rank = unsafe { cinch_array_len(shape) };
    // A zero axis makes the product zero even if other dimensions are
    // arbitrarily large. Test it before converting or multiplying them.
    let zero = (0..rank).any(|i| unsafe { abi::cinch_nat_to_len(read(shape, i)) } == 0);
    let mut size = if zero { 0 } else { 1i64 };
    if !zero {
        for i in 0..rank {
            let d = unsafe { abi::cinch_nat_to_len(read(shape, i)) };
            let Some(n) = size.checked_mul(d) else {
                return 0;
            };
            size = n;
        }
    }
    if size != unsafe { cinch_array_len(data) } {
        return 0;
    }
    unsafe { tensor(data, shape) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_tensor_shape(a: i64) -> i64 {
    let s = unsafe { field(a, block::KIND_TENSOR, 1) };
    unsafe { rc::cinch_dup(s) };
    s
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_tensor_data(a: i64) -> i64 {
    let d = unsafe { field(a, block::KIND_TENSOR, 0) };
    unsafe { rc::cinch_dup(d) };
    d
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_tensor_index(a: i64, indices: i64) -> i64 {
    let shape = unsafe { field(a, block::KIND_TENSOR, 1) };
    let rank = unsafe { cinch_array_len(shape) };
    if rank != unsafe { cinch_array_len(indices) } {
        return -1;
    }
    let mut offset = 0i64;
    for axis in 0..rank {
        let d = unsafe { abi::cinch_nat_to_len(read(shape, axis)) };
        let i = unsafe { abi::cinch_nat_to_len(read(indices, axis)) };
        if i >= d {
            return -1;
        }
        let Some(next) = offset.checked_mul(d).and_then(|n| n.checked_add(i)) else {
            return -1;
        };
        offset = next;
    }
    offset
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_tensor_get(a: i64, i: i64) -> i64 {
    unsafe { cinch_array_get(field(a, block::KIND_TENSOR, 0), i) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_tensor_set(a: i64, i: i64, value: i64) -> i64 {
    // `tensor-set` is persistent: the original tensor keeps its element. The
    // data field is borrowed from `a`, so an update that saw a count of one
    // would write in place and change `a`. Holding a reference of our own
    // across the call makes the count two, so the update copies, and that
    // reference is released again below.
    let borrowed = unsafe { field(a, block::KIND_TENSOR, 0) };
    unsafe { rc::cinch_dup(borrowed) };
    let data = unsafe { cinch_array_set(borrowed, i, value) };
    unsafe { rc::cinch_drop(borrowed) };
    if data == 0 {
        return 0;
    }
    let out = unsafe { tensor(data, field(a, block::KIND_TENSOR, 1)) };
    unsafe { rc::cinch_drop(data) };
    out
}

unsafe fn column(rows: i64, columns: i64, desc: i64, left: i64, right: i64) -> i64 {
    let p =
        unsafe { block::allocate(40, block::KIND_COLUMNAR, block::RC_FRESH, false) }.cast::<i64>();
    unsafe {
        p.write(rows);
        p.add(1).write(columns);
        p.add(2).write(desc);
        p.add(3).write(left);
        p.add(4).write(right);
        rc::cinch_dup(left);
        rc::cinch_dup(right);
    }
    p as usize as i64
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_columnar_column(a: i64) -> i64 {
    unsafe { column(cinch_array_len(a), 1, -1, a, 0) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_columnar_rows(a: i64) -> i64 {
    unsafe { field(a, block::KIND_COLUMNAR, 0) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_columnar_columns(a: i64) -> i64 {
    unsafe { field(a, block::KIND_COLUMNAR, 1) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_columnar_child(a: i64, right: i64) -> i64 {
    if unsafe { field(a, block::KIND_COLUMNAR, 2) } == -1 {
        return 0;
    }
    let child = unsafe { field(a, block::KIND_COLUMNAR, if right == 0 { 3 } else { 4 }) };
    unsafe { rc::cinch_dup(child) };
    child
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_columnar_array(a: i64) -> i64 {
    if unsafe { field(a, block::KIND_COLUMNAR, 2) } != -1 {
        return 0;
    }
    let leaf = unsafe { field(a, block::KIND_COLUMNAR, 3) };
    unsafe { rc::cinch_dup(leaf) };
    leaf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_columnar_zip(a: i64, b: i64, desc: i64) -> i64 {
    let rows = unsafe { cinch_columnar_rows(a) };
    if rows != unsafe { cinch_columnar_rows(b) } {
        return 0;
    }
    let Some(n) =
        unsafe { cinch_columnar_columns(a) }.checked_add(unsafe { cinch_columnar_columns(b) })
    else {
        return 0;
    };
    unsafe { column(rows, n, desc, a, b) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_columnar_get(a: i64, i: i64) -> i64 {
    if i < 0 || i >= unsafe { cinch_columnar_rows(a) } {
        corrupt();
    }
    // Explicit postorder stack: arbitrarily many composed columns do not
    // consume the host stack. Each resulting tuple owns its two fields.
    let mut todo = vec![(a, false)];
    let mut values = Vec::new();
    while let Some((node, visited)) = todo.pop() {
        let desc = unsafe { field(node, block::KIND_COLUMNAR, 2) };
        let left = unsafe { field(node, block::KIND_COLUMNAR, 3) };
        if desc == -1 {
            values.push(unsafe { cinch_array_get(left, i) });
        } else if visited {
            let right = values.pop().unwrap();
            let left = values.pop().unwrap();
            let p = unsafe { block::allocate_ctor(16, desc as u64, false) }.cast::<i64>();
            unsafe {
                p.write(left);
                p.add(1).write(right);
            }
            values.push(p as usize as i64);
        } else {
            todo.push((node, true));
            todo.push((unsafe { field(node, block::KIND_COLUMNAR, 4) }, false));
            todo.push((left, false));
        }
    }
    values.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::heap;

    unsafe fn naturals(values: &[i64]) -> i64 {
        let a = unsafe { array(values.len() as i64, 2) };
        for (i, n) in values.iter().enumerate() {
            unsafe { write(a, i as i64, abi::cinch_nat_from_i64(*n)) };
        }
        a
    }

    #[test]
    fn packed_storage_has_scalar_stride_and_generic_updates_are_persistent() {
        unsafe {
            for (format, value, stride) in [
                (0x101, 255, 1),
                (0x181, -128, 1),
                (0x102, 65535, 2),
                (0x184, i32::MIN as i64, 4),
                (0x108, i64::MIN, 8),
            ] {
                let a = cinch_array_replicate(10, value, format);
                let p = a as usize as *const u8;
                let (bytes, _) = block::payload_bytes(p, block::read_header(p)).unwrap();
                assert_eq!(bytes, (16 + 10 * stride + 7) / 8 * 8);
                assert_eq!(cinch_array_get(a, 9), value);
                let changed = cinch_array_set(a, 0, 7);
                assert_eq!(cinch_array_get(a, 0), value);
                assert_eq!(cinch_array_get(changed, 0), 7);
                assert_eq!(cinch_array_set(a, 10, 3), 0);
                assert_eq!(cinch_array_has(a, -1), 0);
                assert_eq!(cinch_array_slice(a, i64::MAX, 1), 0);
                let empty = cinch_array_slice(a, 10, 0);
                assert_eq!(cinch_array_len(empty), 0);
                rc::cinch_drop(empty);
                rc::cinch_drop(changed);
                rc::cinch_drop(a);
            }
        }
    }

    #[test]
    fn element_pointer_maps_retain_objects_and_tagged_bignums() {
        unsafe {
            let bytes = abi::cinch_bytes_new(3);
            let a = cinch_array_replicate(3, bytes, 1);
            let p = a as usize as *const u8;
            let scan = block::Scan::of(p, block::read_header(p));
            assert_eq!(scan.code(0), block::SCAN_RAW);
            assert_eq!(scan.code(2), block::SCAN_OBJ);
            assert_eq!(
                block::rc(block::read_header(bytes as usize as *const u8)),
                4
            );
            rc::cinch_drop(bytes);
            let copy = cinch_array_get(a, 1);
            rc::cinch_drop(a);
            assert_eq!(abi::cinch_bytes_len(copy), 3);
            rc::cinch_drop(copy);

            let digits = b"184467440737095516160";
            let big = abi::cinch_nat_from_decimal(digits.as_ptr(), digits.len());
            let a = cinch_array_replicate(2, big, 2);
            rc::cinch_drop(big);
            let p = a as usize as *const u8;
            assert_eq!(
                block::Scan::of(p, block::read_header(p)).code(2),
                block::SCAN_TOBJ
            );
            let copy = cinch_array_get(a, 0);
            rc::cinch_drop(a);
            assert_eq!(abi::cinch_nat_to_len(copy), i64::MAX);
            rc::cinch_drop(copy);
        }
    }

    #[test]
    fn shapes_cover_rank_zero_zero_axes_overflow_and_row_major_indexing() {
        unsafe {
            let data = cinch_array_replicate(6, 42, 0);
            let shape = naturals(&[2, 3]);
            let tensor = cinch_tensor_reshape(shape, data);
            assert_ne!(tensor, 0);
            for (indices, expected) in [
                (vec![1, 2], 5),
                (vec![0, 3], -1),
                (vec![2, 0], -1),
                (vec![1], -1),
                (vec![1, 2, 0], -1),
            ] {
                let ix = naturals(&indices);
                assert_eq!(cinch_tensor_index(tensor, ix), expected);
                rc::cinch_drop(ix);
            }
            let huge = naturals(&[i64::MAX, i64::MAX]);
            assert_eq!(cinch_tensor_reshape(huge, data), 0);
            rc::cinch_drop(huge);
            rc::cinch_drop(tensor);
            rc::cinch_drop(data);
            rc::cinch_drop(shape);

            let scalar_shape = naturals(&[]);
            let scalar_data = cinch_array_replicate(1, 99, 0);
            let scalar = cinch_tensor_reshape(scalar_shape, scalar_data);
            assert_ne!(scalar, 0);
            assert_eq!(cinch_tensor_index(scalar, scalar_shape), 0);
            rc::cinch_drop(scalar);
            rc::cinch_drop(scalar_data);
            rc::cinch_drop(scalar_shape);
            let empty = cinch_array_replicate(0, 0, 0);
            let shape = naturals(&[i64::MAX, i64::MAX, 0]);
            let tensor = cinch_tensor_reshape(shape, empty);
            assert_ne!(tensor, 0);
            rc::cinch_drop(tensor);
            rc::cinch_drop(empty);
            rc::cinch_drop(shape);
        }
    }

    #[test]
    fn columns_share_buffers_and_reject_mismatched_lengths() {
        block::install_test_layouts();
        unsafe {
            let raw = cinch_array_replicate(2, 17, 0);
            let bytes = abi::cinch_bytes_new(3);
            let objects = cinch_array_replicate(2, bytes, 1);
            let a = cinch_columnar_column(raw);
            let b = cinch_columnar_column(objects);
            let table = cinch_columnar_zip(a, b, block::test_shapes::PAIR_RO as i64);
            assert_eq!(field(a, block::KIND_COLUMNAR, 3), raw);
            assert_eq!(field(b, block::KIND_COLUMNAR, 3), objects);
            assert_eq!(cinch_columnar_columns(table), 2);
            rc::cinch_drop(a);
            rc::cinch_drop(b);
            rc::cinch_drop(raw);
            rc::cinch_drop(objects);
            rc::cinch_drop(bytes);
            let row = cinch_columnar_get(table, 1);
            rc::cinch_drop(table);
            assert_eq!((row as usize as *const i64).read(), 17);
            assert_eq!(
                abi::cinch_bytes_len((row as usize as *const i64).add(1).read()),
                3
            );
            rc::cinch_drop(row);
            let empty = cinch_array_replicate(0, 0, 0);
            let one = cinch_array_replicate(1, 0, 0);
            let a = cinch_columnar_column(empty);
            let b = cinch_columnar_column(one);
            assert_eq!(cinch_columnar_zip(a, b, 0), 0);
            rc::cinch_drop(a);
            rc::cinch_drop(b);
            rc::cinch_drop(empty);
            rc::cinch_drop(one);
        }
    }

    /// The in-place entry allocates nothing when the reference it was handed
    /// is the only one, so a fill loop over a freshly built array is O(n)
    /// blocks rather than O(n^2). Its result is its own reference, so the
    /// count is one before and after.
    #[test]
    fn set_inplace_writes_in_place_and_keeps_the_count_at_one() {
        unsafe {
            let values: Vec<i64> = (0..8).map(|i| abi::cinch_nat_from_i64(i)).collect();
            let a = cinch_array_replicate(8, values[0], 2);
            let before = heap::stats().0;
            for (i, value) in values.iter().enumerate() {
                let written = cinch_array_set_inplace(a, i as i64, *value);
                assert_eq!(written, a, "an exclusive array is written in place");
                // The entry dups for its result and the caller drops that
                // reference, so one reference survives.
                rc::cinch_drop(written);
                assert_eq!(block::rc(block::read_header(a as usize as *const u8)), 1);
            }
            assert_eq!(heap::stats().0 - before, 0, "in-place writes allocate nothing");
            for (i, value) in values.iter().enumerate() {
                assert_eq!(cinch_array_get(a, i as i64), *value);
            }
            rc::cinch_drop(a);
        }
    }

    /// Consumption is a property of the reference, not of the block: a program
    /// may consume one of two words for one array. That call sees a count above
    /// one, so the entry copies, and the other word keeps the old contents.
    #[test]
    fn set_inplace_copies_when_another_reference_exists() {
        unsafe {
            let values: Vec<i64> = (0..4).map(|i| abi::cinch_nat_from_i64(i)).collect();
            let a = cinch_array_replicate(4, values[0], 2);
            rc::cinch_dup(a);
            let copied = cinch_array_set_inplace(a, 1, values[3]);
            assert_ne!(copied, a, "a second reference forces the copy");
            assert_eq!(cinch_array_get(a, 1), values[0]);
            assert_eq!(cinch_array_get(copied, 1), values[3]);
            rc::cinch_drop(copied);
            rc::cinch_drop(a);
            rc::cinch_drop(a);
        }
    }

    /// The in-place branch balances counted elements: the slot's new reference
    /// is taken before the displaced element's is released, so a write of the
    /// value already there cannot free it.
    #[test]
    fn set_inplace_balances_counts_for_counted_elements() {
        unsafe {
            let first = abi::cinch_bytes_new(2);
            let second = abi::cinch_bytes_new(2);
            let a = cinch_array_replicate(3, first, 1);
            assert_eq!(block::rc(block::read_header(first as usize as *const u8)), 4);
            let out = cinch_array_set_inplace(a, 1, second);
            assert_eq!(out, a);
            rc::cinch_drop(out);
            assert_eq!(block::rc(block::read_header(a as usize as *const u8)), 1);
            assert_eq!(block::rc(block::read_header(first as usize as *const u8)), 3);
            assert_eq!(block::rc(block::read_header(second as usize as *const u8)), 2);
            let same = cinch_array_set_inplace(a, 0, first);
            assert_eq!(same, a);
            rc::cinch_drop(same);
            assert_eq!(block::rc(block::read_header(first as usize as *const u8)), 3);
            rc::cinch_drop(a);
            rc::cinch_drop(first);
            rc::cinch_drop(second);
        }
    }

    /// `tensor-set` hands its tensor's borrowed data field to an array update,
    /// so it must hold a reference across the call: without that the update
    /// sees a count of one, writes in place, and the original tensor changes
    /// under its owner.
    #[test]
    fn tensor_set_leaves_the_original_tensor_alone() {
        unsafe {
            let data = naturals(&[0, 1, 2, 3]);
            let dims = naturals(&[2, 2]);
            let matrix = cinch_tensor_reshape(dims, data);
            assert_ne!(matrix, 0);
            let changed = cinch_tensor_set(matrix, 3, abi::cinch_nat_from_i64(90));
            assert_ne!(changed, 0);
            assert_eq!(cinch_tensor_get(matrix, 3), abi::cinch_nat_from_i64(3));
            assert_eq!(cinch_tensor_get(changed, 3), abi::cinch_nat_from_i64(90));
            rc::cinch_drop(changed);
            rc::cinch_drop(matrix);
            rc::cinch_drop(dims);
            rc::cinch_drop(data);
        }
    }

    #[test]
    fn mark_on_spawn_visits_counted_array_elements() {
        unsafe {
            let child = abi::cinch_bytes_new(3);
            let a = cinch_array_replicate(2, child, 1);
            rc::cinch_mark_mt(a);
            assert!(block::rc(block::read_header(a as usize as *const u8)) < 0);
            assert!(block::rc(block::read_header(child as usize as *const u8)) < 0);
            let owned = cinch_array_get(a, 0);
            rc::cinch_drop(a);
            assert_eq!(abi::cinch_bytes_len(owned), 3);
            rc::cinch_drop(owned);
            rc::cinch_drop(child);
        }
    }
}
