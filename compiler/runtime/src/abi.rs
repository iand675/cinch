//! The `extern "C"` ABI surface consumed by generated SafeLang code.
//!
//! Every entry point takes and returns `i64` words (see the crate docs for
//! the word and heap block representation). Faults at this layer terminate
//! the process with a defined exit code after writing a message to stderr;
//! the safe Rust API in [`crate::nat`] and [`crate::int`] returns
//! [`crate::error::Error`] instead so callers (and tests) can observe them.

use std::alloc::{alloc_zeroed, Layout};

use crate::error::{Error, Result};
use crate::int::Int;
use crate::nat::Nat;

/// Exit code of the `BlackHole` fault (`sl_blackhole`).
///
/// Must stay in sync with the code generator: `compiler/src/codegen.rs`
/// emits `__safelang_blackhole` calling `exit(i32 70)` directly. Both are
/// the same defined fault seen from two call sites.
pub const EXIT_BLACKHOLE: i32 = 70;

/// Exit code of the generic fault path (`sl_panic` and every runtime error
/// except division by zero).
pub const EXIT_PANIC: i32 = 71;

/// Exit code of the division fault (`sl_fault_div_zero`).
pub const EXIT_DIV_ZERO: i32 = 72;

/// Block kind tag: a `Nat` value.
pub const KIND_NAT: i64 = 1;

/// Block kind tag: an `Int` value.
pub const KIND_INT: i64 = 2;

/// Block kind tag: a `Bytes` buffer.
pub const KIND_BYTES: i64 = 3;

/// Heap block behind a bignum word.
///
/// `kind` is the first machine word so a corrupt word faults instead of
/// being misinterpreted; the payload follows. The payload is a Rust `Nat`
/// or `Int` value whose internal `Vec<u32>` header is a standard library
/// implementation detail: generated code treats the whole block as opaque
/// and only passes the word back to this runtime.
#[repr(C)]
struct Boxed<T> {
    kind: i64,
    value: T,
}

fn box_nat(v: Nat) -> i64 {
    Box::into_raw(Box::new(Boxed {
        kind: KIND_NAT,
        value: v,
    })) as usize as i64
}

fn box_int(v: Int) -> i64 {
    Box::into_raw(Box::new(Boxed {
        kind: KIND_INT,
        value: v,
    })) as usize as i64
}

pub(crate) fn box_bytes(v: crate::bytes::Bytes) -> i64 {
    Box::into_raw(Box::new(Boxed {
        kind: KIND_BYTES,
        value: v,
    })) as usize as i64
}

/// Fault through the `sl_panic` path: write `msg` to stderr and exit 71.
fn panic_exit(msg: &[u8]) -> ! {
    use std::io::Write;
    let _ = std::io::stderr().write_all(b"slruntime: ");
    let _ = std::io::stderr().write_all(msg);
    let _ = std::io::stderr().write_all(b"\n");
    std::process::exit(EXIT_PANIC)
}

/// Fault through the dedicated division path: exit 72.
fn div_zero_exit() -> ! {
    std::process::exit(EXIT_DIV_ZERO)
}

/// Map a Rust API error onto its defined runtime fault.
pub(crate) fn raise(err: Error) -> ! {
    if err == Error::DivByZero {
        div_zero_exit();
    }
    panic_exit(err.to_string().as_bytes())
}

/// Read a `Nat` word, faulting on null or wrong-kind blocks.
unsafe fn as_nat<'a>(word: i64) -> &'a Nat {
    let p = word as *const Boxed<Nat>;
    if p.is_null() {
        raise(Error::CorruptValue);
    }
    let b = unsafe { &*p };
    if b.kind != KIND_NAT {
        raise(Error::CorruptValue);
    }
    &b.value
}

/// Read an `Int` word, faulting on null or wrong-kind blocks.
unsafe fn as_int<'a>(word: i64) -> &'a Int {
    let p = word as *const Boxed<Int>;
    if p.is_null() {
        raise(Error::CorruptValue);
    }
    let b = unsafe { &*p };
    if b.kind != KIND_INT {
        raise(Error::CorruptValue);
    }
    &b.value
}

/// Read a `Bytes` word, faulting on null or wrong-kind blocks.
pub(crate) unsafe fn as_bytes<'a>(word: i64) -> &'a crate::bytes::Bytes {
    let p = word as *const Boxed<crate::bytes::Bytes>;
    if p.is_null() {
        raise(Error::CorruptValue);
    }
    let b = unsafe { &*p };
    if b.kind != KIND_BYTES {
        raise(Error::CorruptValue);
    }
    &b.value
}

/// Read a `Bytes` word for mutation. Buffer writes are part of the
/// language-level `bytesSet` contract, so this is a deliberate mutable
/// borrow at the extern boundary; generated code sequences the writes.
unsafe fn as_bytes_mut<'a>(word: i64) -> &'a mut crate::bytes::Bytes {
    let p = word as *mut Boxed<crate::bytes::Bytes>;
    if p.is_null() {
        raise(Error::CorruptValue);
    }
    let b = unsafe { &mut *p };
    if b.kind != KIND_BYTES {
        raise(Error::CorruptValue);
    }
    &mut b.value
}

/// Borrow a byte slice from a raw pointer and length.
unsafe fn bytes<'a>(ptr: *const u8, len: usize) -> Result<&'a [u8]> {
    if len == 0 {
        return Ok(&[]);
    }
    if ptr.is_null() {
        return Err(Error::CorruptValue);
    }
    Ok(unsafe { std::slice::from_raw_parts(ptr, len) })
}

// ---------------------------------------------------------------------------
// Faults
// ---------------------------------------------------------------------------

/// The `BlackHole` fault: demanding a memo cell that is already being
/// evaluated (or any other defined black hole). Exits with code 70, in sync
/// with `__safelang_blackhole` in the code generator.
#[unsafe(no_mangle)]
pub extern "C" fn sl_blackhole() -> ! {
    std::process::exit(EXIT_BLACKHOLE)
}

/// Generic fault: write `msg` to stderr and exit with code 71. A null
/// pointer or zero length writes no message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_panic(msg: *const u8, len: usize) -> ! {
    match unsafe { bytes(msg, len) } {
        Ok(m) => panic_exit(m),
        Err(_) => panic_exit(b"panic with corrupt message pointer"),
    }
}

/// Division by zero fault. Exits with code 72.
#[unsafe(no_mangle)]
pub extern "C" fn sl_fault_div_zero() -> ! {
    div_zero_exit()
}

// ---------------------------------------------------------------------------
// Allocation
// ---------------------------------------------------------------------------

/// Allocate a zero-filled heap block of `size` bytes and return it as a
/// word.
///
/// The size is rounded up to a whole number of 64-bit words (minimum one
/// word). Negative sizes and allocation failures fault through `sl_panic`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_alloc(size: i64) -> i64 {
    if size < 0 {
        panic_exit(b"sl_alloc called with negative size");
    }
    let raw = size as usize;
    let rounded = match raw.checked_add(7).map(|s| s & !7) {
        Some(0) => 8,
        Some(s) => s,
        None => panic_exit(b"sl_alloc size overflow"),
    };
    let Ok(layout) = Layout::from_size_align(rounded, 8) else {
        panic_exit(b"sl_alloc layout failure");
    };
    let p = unsafe { alloc_zeroed(layout) };
    if p.is_null() {
        panic_exit(b"sl_alloc out of memory");
    }
    p as usize as i64
}

/// Free a block previously returned by [`sl_alloc`].
///
/// Currently a no-op: blocks are leaked by design until memory reclamation
/// lands. The symbol exists so generated code can already emit balanced
/// allocation and deallocation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_free(_word: i64) {}

// ---------------------------------------------------------------------------
// Nat
// ---------------------------------------------------------------------------

/// Box an `i64` as a `Nat`, faulting on negative input.
#[unsafe(no_mangle)]
pub extern "C" fn sl_nat_from_i64(v: i64) -> i64 {
    match Nat::from_i64(v) {
        Ok(n) => box_nat(n),
        Err(e) => raise(e),
    }
}

/// The value of a `Nat` word as an `i64`, faulting when out of range.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_nat_to_i64(word: i64) -> i64 {
    match unsafe { as_nat(word) }.to_i64() {
        Ok(v) => v,
        Err(e) => raise(e),
    }
}

/// 1 when the `Nat` word fits in an `i64`, else 0.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_nat_fits_i64(word: i64) -> i64 {
    i64::from(unsafe { as_nat(word) }.fits_i64())
}

/// `a + b` for `Nat` words.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_nat_add(a: i64, b: i64) -> i64 {
    box_nat(unsafe { as_nat(a) }.add(unsafe { as_nat(b) }))
}

/// `a - b` for `Nat` words, faulting on negative result.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_nat_sub(a: i64, b: i64) -> i64 {
    match unsafe { as_nat(a) }.sub(unsafe { as_nat(b) }) {
        Ok(v) => box_nat(v),
        Err(e) => raise(e),
    }
}

/// `a * b` for `Nat` words.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_nat_mul(a: i64, b: i64) -> i64 {
    box_nat(unsafe { as_nat(a) }.mul(unsafe { as_nat(b) }))
}

/// `a / b` for `Nat` words (the quotient; the remainder is available
/// through [`sl_nat_rem`]), faulting on division by zero.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_nat_div(a: i64, b: i64) -> i64 {
    match unsafe { as_nat(a) }.div_rem(unsafe { as_nat(b) }) {
        Ok((q, _)) => box_nat(q),
        Err(e) => raise(e),
    }
}

/// `a % b` for `Nat` words, faulting on division by zero.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_nat_rem(a: i64, b: i64) -> i64 {
    match unsafe { as_nat(a) }.rem(unsafe { as_nat(b) }) {
        Ok(v) => box_nat(v),
        Err(e) => raise(e),
    }
}

/// Three-way comparison of `Nat` words: -1, 0 or 1.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_nat_cmp(a: i64, b: i64) -> i64 {
    use std::cmp::Ordering;
    match unsafe { as_nat(a) }.cmp(unsafe { as_nat(b) }) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Parse ASCII decimal digits at `ptr` into a `Nat` word, faulting on
/// malformed input.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_nat_from_decimal(ptr: *const u8, len: usize) -> i64 {
    match unsafe { bytes(ptr, len) }.and_then(Nat::from_decimal) {
        Ok(v) => box_nat(v),
        Err(e) => raise(e),
    }
}

/// Write the decimal digits of a `Nat` word into `buf`, returning the
/// length written. Faults when `cap` is smaller than the digits (the
/// buffer is then untouched).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_nat_to_decimal(word: i64, buf: *mut u8, cap: usize) -> usize {
    let v = unsafe { as_nat(word) };
    let len = v.decimal_len();
    if buf.is_null() {
        raise(Error::CorruptValue);
    }
    if cap < len {
        raise(Error::BufferTooSmall);
    }
    let dst = unsafe { std::slice::from_raw_parts_mut(buf, cap) };
    match v.to_decimal(dst) {
        Ok(n) => n,
        Err(e) => raise(e),
    }
}

/// Number of ASCII bytes [`sl_nat_to_decimal`] would write.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_nat_decimal_len(word: i64) -> usize {
    unsafe { as_nat(word) }.decimal_len()
}

// ---------------------------------------------------------------------------
// Int
// ---------------------------------------------------------------------------

/// Box an `i64` as an `Int` word.
#[unsafe(no_mangle)]
pub extern "C" fn sl_int_from_i64(v: i64) -> i64 {
    box_int(Int::from_i64(v))
}

/// The value of an `Int` word as an `i64`, faulting when out of range.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_int_to_i64(word: i64) -> i64 {
    match unsafe { as_int(word) }.to_i64() {
        Ok(v) => v,
        Err(e) => raise(e),
    }
}

/// 1 when the `Int` word fits in an `i64`, else 0.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_int_fits_i64(word: i64) -> i64 {
    i64::from(unsafe { as_int(word) }.fits_i64())
}

/// `a + b` for `Int` words.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_int_add(a: i64, b: i64) -> i64 {
    box_int(unsafe { as_int(a) }.add(unsafe { as_int(b) }))
}

/// `a - b` for `Int` words.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_int_sub(a: i64, b: i64) -> i64 {
    box_int(unsafe { as_int(a) }.sub(unsafe { as_int(b) }))
}

/// `a * b` for `Int` words.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_int_mul(a: i64, b: i64) -> i64 {
    box_int(unsafe { as_int(a) }.mul(unsafe { as_int(b) }))
}

/// `a / b` for `Int` words, truncated toward zero, faulting on division by
/// zero.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_int_div(a: i64, b: i64) -> i64 {
    match unsafe { as_int(a) }.div(unsafe { as_int(b) }) {
        Ok(v) => box_int(v),
        Err(e) => raise(e),
    }
}

/// `a % b` for `Int` words, with the sign of the dividend, faulting on
/// division by zero.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_int_rem(a: i64, b: i64) -> i64 {
    match unsafe { as_int(a) }.rem(unsafe { as_int(b) }) {
        Ok(v) => box_int(v),
        Err(e) => raise(e),
    }
}

/// `-a` for an `Int` word.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_int_neg(a: i64) -> i64 {
    box_int(unsafe { as_int(a) }.neg())
}

/// Three-way comparison of `Int` words: -1, 0 or 1.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_int_cmp(a: i64, b: i64) -> i64 {
    use std::cmp::Ordering;
    match unsafe { as_int(a) }.cmp(unsafe { as_int(b) }) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

/// Parse an optionally signed ASCII decimal numeral at `ptr` into an `Int`
/// word, faulting on malformed input.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_int_from_decimal(ptr: *const u8, len: usize) -> i64 {
    match unsafe { bytes(ptr, len) }.and_then(Int::from_decimal) {
        Ok(v) => box_int(v),
        Err(e) => raise(e),
    }
}

/// Write the decimal representation of an `Int` word (sign included) into
/// `buf`, returning the length written. Faults when `cap` is too small
/// (the buffer is then untouched).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_int_to_decimal(word: i64, buf: *mut u8, cap: usize) -> usize {
    let v = unsafe { as_int(word) };
    let len = v.decimal_len();
    if buf.is_null() {
        raise(Error::CorruptValue);
    }
    if cap < len {
        raise(Error::BufferTooSmall);
    }
    let dst = unsafe { std::slice::from_raw_parts_mut(buf, cap) };
    match v.to_decimal(dst) {
        Ok(n) => n,
        Err(e) => raise(e),
    }
}

/// Number of ASCII bytes [`sl_int_to_decimal`] would write.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_int_decimal_len(word: i64) -> usize {
    unsafe { as_int(word) }.decimal_len()
}

/// A zeroed `Bytes` block of `len` bytes, faulting on negative length.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_bytes_new(len: i64) -> i64 {
    match crate::bytes::Bytes::new(len) {
        Ok(b) => box_bytes(b),
        Err(e) => raise(e),
    }
}

/// The length in bytes of a `Bytes` word.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_bytes_len(word: i64) -> i64 {
    unsafe { as_bytes(word) }.len()
}

/// The byte (0..=255) at `index`, faulting out of bounds.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_bytes_get(word: i64, index: i64) -> i64 {
    match unsafe { as_bytes(word) }.get(index) {
        Ok(v) => v,
        Err(e) => raise(e),
    }
}

/// Write `value` (0..=255) at `index`; returns 0 (the Unit word),
/// faulting on out-of-bounds index or value.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_bytes_set(word: i64, index: i64, value: i64) -> i64 {
    match unsafe { as_bytes_mut(word) }.set(index, value) {
        Ok(()) => 0,
        Err(e) => raise(e),
    }
}

/// The data pointer of a `Bytes` word: how buffer contents cross to
/// pointer-and-length interfaces (the UTF-8 intrinsics and the canonical
/// ABI glue). The pointer is valid as long as the word lives; words are
/// never freed (see the module docs).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_bytes_data(word: i64) -> i64 {
    unsafe { as_bytes(word) }.as_slice().as_ptr() as usize as i64
}

/// A fresh `Bytes` word holding a copy of `len` bytes at `ptr` (the
/// canonical ABI copies its lists and strings; zero-copy needs its own
/// sharing arrangement). Negative lengths and null pointers with a
/// positive length fault.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_bytes_from_data(ptr: *const u8, len: i64) -> i64 {
    if len < 0 {
        raise(Error::OutOfBounds)
    }
    let n = match usize::try_from(len) {
        Ok(n) => n,
        Err(_) => raise(Error::OutOfBounds),
    };
    let slice: &[u8] = if n == 0 {
        &[]
    } else if ptr.is_null() {
        raise(Error::OutOfBounds)
    } else {
        unsafe { std::slice::from_raw_parts(ptr, n) }
    };
    box_bytes(crate::bytes::Bytes::from(slice.to_vec()))
}

// ------------------------------------------------------------ structured concurrency (P12)
//
// The scope word points at a boxed `Arc<crate::task::Scope>` (one
// reference consumed by `sl_scope_close`); a handle word is an
// `Arc<TaskState>` reference (never freed twice; see the module docs of
// `crate::task`). The task outcome and closure ABI conventions are
// documented in `crate::task`.

/// Open a structured scope: the `withScope` entry (concurrency
/// specification section 2).
#[unsafe(no_mangle)]
pub extern "C" fn sl_scope_open() -> i64 {
    let scope = std::sync::Arc::new(crate::task::Scope::new());
    Box::into_raw(Box::new(scope)) as usize as i64
}

/// Scope exit: cancel every unjoined child, wait for each cleanup
/// acknowledgment, and release the scope.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_scope_close(scope: i64) -> i64 {
    let owned = unsafe { Box::from_raw(scope as *mut std::sync::Arc<crate::task::Scope>) };
    owned.close();
    drop(owned);
    0
}

/// Spawn the body closure as a task of `scope`; returns the task handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_task_spawn(scope: i64, closure: i64) -> i64 {
    let scope = unsafe { &*(scope as *const std::sync::Arc<crate::task::Scope>) };
    scope.spawn(closure)
}

/// `joinAllOrCancel` over the `n` handles at `handles`: 0 and the `Ok`
/// payloads in `out` on success, 1 and the first `Err` payload in
/// `out[0]` on failure (the first failure cancels the siblings).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_task_join_all_or_cancel(
    scope: i64,
    handles: i64,
    n: i64,
    out: i64,
) -> i64 {
    if n < 0 {
        raise(Error::OutOfBounds);
    }
    let scope = unsafe { &*(scope as *const std::sync::Arc<crate::task::Scope>) };
    unsafe {
        scope.join_all_or_cancel(
            handles as *const i64,
            n as usize,
            out as *mut i64,
        )
    }
}

/// The cancellation observation point (section 4): 1 once cancellation
/// has been requested for the running task, else 0.
#[unsafe(no_mangle)]
pub extern "C" fn sl_task_poll_cancel() -> i64 {
    i64::from(crate::task::current_task_canceled())
}

/// Write a `Bytes` word to the runtime's output sink (stdout), flushed so
/// an observer sees it immediately; returns 0 (the Unit word).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_io_write(word: i64) -> i64 {
    use std::io::Write;
    let buf = unsafe { as_bytes(word) }.as_slice();
    let mut out = std::io::stdout();
    if out.write_all(buf).is_err() || out.flush().is_err() {
        raise(Error::OutOfBounds);
    }
    0
}

/// Read up to `len` bytes from the runtime's input source (stdin): the
/// `Bytes` word, or 0 when the source is at end of input (`ioRead`'s
/// `None`). A negative length faults.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sl_io_read(len: i64) -> i64 {
    use std::io::Read;
    if len < 0 {
        raise(Error::OutOfBounds);
    }
    let want = usize::try_from(len).unwrap_or(usize::MAX);
    let mut buf = vec![0u8; want];
    let mut filled = 0usize;
    while filled < want {
        match std::io::stdin().read(&mut buf[filled..]) {
            Ok(0) => break,
            Ok(n) => filled += n,
            Err(_) => break,
        }
    }
    if filled == 0 {
        return 0;
    }
    buf.truncate(filled);
    box_bytes(crate::bytes::Bytes::from(buf))
}

/// Sleep `millis` milliseconds as a suspension point: returns early with
/// 0 (the Unit word) once cancellation is requested for the running task
/// (concurrency specification section 4).
#[unsafe(no_mangle)]
pub extern "C" fn sl_io_sleep(millis: i64) -> i64 {
    let mut left = millis.max(0) as u64;
    while left > 0 {
        if crate::task::current_task_canceled() {
            break;
        }
        let slice = left.min(1);
        std::thread::sleep(std::time::Duration::from_millis(slice));
        left -= slice;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_codes_match_the_documented_fault_table() {
        assert_eq!(EXIT_BLACKHOLE, 70);
        assert_eq!(EXIT_PANIC, 71);
        assert_eq!(EXIT_DIV_ZERO, 72);
        // The code generator emits exit(70) for the same fault; if that
        // constant moves, EXIT_BLACKHOLE must move with it.
        let codegen_blackhole_exit = 70;
        assert_eq!(EXIT_BLACKHOLE, codegen_blackhole_exit);
    }

    #[test]
    fn kind_tags_are_distinct() {
        assert_ne!(KIND_NAT, KIND_INT);
    }

    #[test]
    fn boxing_round_trips_through_the_word_api() {
        unsafe {
            let w = sl_nat_from_i64(41);
            let sum = sl_nat_add(w, w);
            assert_eq!(sl_nat_to_i64(sum), 82);
            assert_eq!(sl_nat_cmp(w, sum), -1);
            assert_eq!(sl_nat_cmp(sum, w), 1);
            assert_eq!(sl_nat_cmp(w, w), 0);

            let i = sl_int_from_i64(-5);
            let j = sl_int_mul(i, i);
            assert_eq!(sl_int_to_i64(j), 25);
            assert_eq!(sl_int_to_i64(sl_int_neg(i)), 5);
        }
    }

    #[test]
    fn decimal_entry_points_round_trip() {
        unsafe {
            let text = b"123456789012345678901234567890";
            let w = sl_nat_from_decimal(text.as_ptr(), text.len());
            assert_eq!(sl_nat_decimal_len(w), text.len());
            let mut buf = [0u8; 32];
            let n = sl_nat_to_decimal(w, buf.as_mut_ptr(), buf.len());
            assert_eq!(&buf[..n], text);

            let t2 = b"-42";
            let i = sl_int_from_decimal(t2.as_ptr(), t2.len());
            assert_eq!(sl_int_decimal_len(i), 3);
            let n = sl_int_to_decimal(i, buf.as_mut_ptr(), buf.len());
            assert_eq!(&buf[..n], t2);
        }
    }

    #[test]
    fn alloc_zero_fills_and_free_keeps_the_symbol() {
        unsafe {
            let p = sl_alloc(24);
            assert_ne!(p, 0);
            let slice = std::slice::from_raw_parts(p as *const u8, 24);
            assert!(slice.iter().all(|&b| b == 0));
            sl_free(p);
        }
    }

    #[test]
    fn error_exit_codes_match_the_fault_paths() {
        assert_eq!(Error::DivByZero.exit_code(), EXIT_DIV_ZERO);
        assert_eq!(Error::NegativeResult.exit_code(), EXIT_PANIC);
    }
}
