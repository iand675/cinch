//! The `extern "C"` ABI surface consumed by generated Cinch code.
//!
//! Every entry point takes and returns `i64` words (see the crate docs for
//! the word and heap block representation). Faults at this layer terminate
//! the process with a defined exit code after writing a message to stderr;
//! the safe Rust API in [`crate::nat`] and [`crate::int`] returns
//! [`crate::error::Error`] instead so callers (and tests) can observe them.

use crate::error::{Error, Result};
use crate::int::Int;
use crate::nat::Nat;

/// Exit code of the `BlackHole` fault (`cinch_blackhole`).
///
/// Must stay in sync with the code generator: `compiler/src/codegen.rs`
/// emits `__cinch_blackhole` calling `exit(i32 70)` directly. Both are
/// the same defined fault seen from two call sites.
pub const EXIT_BLACKHOLE: i32 = 70;

/// Exit code of the generic fault path (`cinch_panic` and every runtime error
/// except division by zero).
pub const EXIT_PANIC: i32 = 71;

/// Exit code of the division fault (`cinch_fault_div_zero`).
pub const EXIT_DIV_ZERO: i32 = 72;

/// Exit code of the heap-limit fault (`CINCH_HEAP_LIMIT`, [`crate::heap`]).
pub const EXIT_HEAP_LIMIT: i32 = 73;

/// Block kind tag: a `Nat` value.
pub const KIND_NAT: i64 = 1;

/// Block kind tag: an `Int` value.
pub const KIND_INT: i64 = 2;

/// Block kind tag: a `Bytes` buffer.
pub const KIND_BYTES: i64 = 3;

/// Block kind tag: a `Sym` interned name cell (primitive types
/// specification section 9).
pub const KIND_SYM: i64 = 4;

/// Block kind tag: a read-only `Bytes` buffer whose data lives in the
/// data section (a string literal's constant backing). The payload is
/// the same [`crate::bytes::Bytes`]; mutation attempts fault (a defined
/// fault, never a write) instead of touching constant data.
pub const KIND_BYTES_STATIC: i64 = 5;

/// Heap block behind a bignum word.
///
/// `kind` is the first machine word so a corrupt word faults instead of
/// being misinterpreted; the payload follows. The [`crate::bytes::Bytes`]
/// payload is a stable `#[repr(C)]` length-and-pointer pair (read-only
/// data-section buffers are the same shape, `KIND_BYTES_STATIC`); the
/// `Nat`/`Int` payloads are Rust values whose internal `Vec<u32>` header
/// is a standard library implementation detail: generated code treats
/// those blocks as opaque and only passes the word back to this runtime.
/// Every box is a header block ([`crate::block`]); the header's kind is a
/// separate namespace from this in-block `kind` word.
#[repr(C)]
struct Boxed<T> {
    kind: i64,
    value: T,
}

fn boxed<T>(header_kind: u8, rc: i64, kind: i64, value: T) -> i64 {
    crate::block::allocate_value(header_kind, rc, Boxed { kind, value }) as usize as i64
}

fn box_nat(v: Nat) -> i64 {
    boxed(crate::block::KIND_NAT, crate::block::RC_FRESH, KIND_NAT, v)
}

fn box_int(v: Int) -> i64 {
    boxed(crate::block::KIND_INT, crate::block::RC_FRESH, KIND_INT, v)
}

pub(crate) fn box_bytes(v: crate::bytes::Bytes) -> i64 {
    boxed(crate::block::KIND_BYTES, crate::block::RC_FRESH, KIND_BYTES, v)
}

/// Box one interned name cell (the cell is handed out once per name and
/// never freed, so its word is the name's identity): an immortal block.
pub(crate) fn box_sym(v: crate::sym::SymCell) -> i64 {
    boxed(crate::block::KIND_SYM, crate::block::RC_IMMORTAL, KIND_SYM, v)
}

/// Release the Rust resources a runtime box owns (a `Nat`'s limbs, a heap
/// buffer's data) before its block is deallocated. Every other kind owns
/// nothing outside its block.
///
/// # Safety
///
/// `p` is the payload pointer of a live block whose header kind is
/// `header_kind`, and is not read again.
unsafe fn drop_box_payload(p: *mut u8, header_kind: u8) {
    use crate::block::{KIND_BYTES as B, KIND_INT as I, KIND_NAT as N};
    match header_kind {
        N => unsafe { std::ptr::drop_in_place(&raw mut (*p.cast::<Boxed<Nat>>()).value) },
        I => unsafe { std::ptr::drop_in_place(&raw mut (*p.cast::<Boxed<Int>>()).value) },
        B => unsafe { (*p.cast::<Boxed<crate::bytes::Bytes>>()).value.release() },
        _ => {}
    }
}

/// Read a `Sym` word, faulting on null or wrong-kind blocks.
pub(crate) unsafe fn as_sym<'a>(word: i64) -> &'a crate::sym::SymCell {
    let p = word as *const Boxed<crate::sym::SymCell>;
    if p.is_null() {
        raise(Error::CorruptValue);
    }
    let b = unsafe { &*p };
    if b.kind != KIND_SYM {
        raise(Error::CorruptValue);
    }
    &b.value
}

/// Fault through the `cinch_panic` path: write `msg` to stderr and exit 71.
pub(crate) fn panic_exit(msg: &[u8]) -> ! {
    use std::io::Write;
    let _ = std::io::stderr().write_all(b"cinchruntime: ");
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

// Small `Nat`/`Int` immediates (design/reclamation.md section 2.6). Heap
// blocks are 8-aligned, so a word whose low bit is 1 is never a pointer:
// it is the immediate `v << 1 | 1`. Every entry point returns the
// immediate whenever the value fits (the canonical form), so a box only
// ever holds a value outside the immediate range and two words of the
// same small value are the same word.

/// The largest `Nat` immediate, `2^63 - 1`: all 63 payload bits. It is
/// also `i64::MAX`, so every `Nat` an `i64` can carry is an immediate.
pub(crate) const NAT_IMMEDIATE_MAX: u64 = (1 << 63) - 1;

/// The smallest `Int` immediate, `-2^62`.
pub(crate) const INT_IMMEDIATE_MIN: i64 = -(1 << 62);

/// The largest `Int` immediate, `2^62 - 1`. With [`INT_IMMEDIATE_MIN`]
/// this is the 63-bit two's complement payload, decoded by one arithmetic
/// shift; a wider range would need a second tag bit.
pub(crate) const INT_IMMEDIATE_MAX: i64 = (1 << 62) - 1;

/// Whether a `Nat`/`Int` word is an immediate (and so not a pointer).
#[inline]
pub(crate) fn is_immediate(word: i64) -> bool {
    word & 1 != 0
}

#[inline]
fn nat_immediate(v: u64) -> i64 {
    debug_assert!(v <= NAT_IMMEDIATE_MAX);
    ((v << 1) | 1) as i64
}

#[inline]
fn nat_immediate_value(word: i64) -> u64 {
    (word as u64) >> 1
}

#[inline]
fn int_immediate(v: i64) -> i64 {
    debug_assert!((INT_IMMEDIATE_MIN..=INT_IMMEDIATE_MAX).contains(&v));
    (v << 1) | 1
}

#[inline]
fn int_immediate_value(word: i64) -> i64 {
    word >> 1
}

/// The canonical word of a machine-range `Nat` value.
fn nat_word_u128(v: u128) -> i64 {
    if v <= u128::from(NAT_IMMEDIATE_MAX) {
        nat_immediate(v as u64)
    } else {
        box_nat(Nat::from_u128(v))
    }
}

/// The canonical word of a `Nat`: the immediate when it fits, else a box.
pub(crate) fn nat_word(v: Nat) -> i64 {
    match v.to_u64() {
        Some(u) if u <= NAT_IMMEDIATE_MAX => nat_immediate(u),
        _ => box_nat(v),
    }
}

/// The canonical word of a machine-range `Int` value.
fn int_word_i128(v: i128) -> i64 {
    if (i128::from(INT_IMMEDIATE_MIN)..=i128::from(INT_IMMEDIATE_MAX)).contains(&v) {
        int_immediate(v as i64)
    } else {
        box_int(Int::from_i128(v))
    }
}

/// The canonical word of an `Int`: the immediate when it fits, else a box.
pub(crate) fn int_word(v: Int) -> i64 {
    match v.to_i64() {
        Ok(i) if (INT_IMMEDIATE_MIN..=INT_IMMEDIATE_MAX).contains(&i) => int_immediate(i),
        _ => box_int(v),
    }
}

/// Read a boxed `Nat` word, faulting on null or wrong-kind blocks. The
/// caller has already ruled out an immediate.
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

/// Read a boxed `Int` word, faulting on null or wrong-kind blocks. The
/// caller has already ruled out an immediate.
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

/// The magnitude of any `Nat` word: an immediate's lands in `scratch`.
unsafe fn nat_limbs<'a>(word: i64, scratch: &'a mut [u32; 2]) -> &'a [u32] {
    if is_immediate(word) {
        crate::nat::small_limbs(nat_immediate_value(word), scratch)
    } else {
        unsafe { as_nat(word) }.limbs()
    }
}

/// The sign and magnitude of any `Int` word: an immediate's magnitude
/// lands in `scratch`.
unsafe fn int_parts<'a>(word: i64, scratch: &'a mut [u32; 2]) -> (bool, &'a [u32]) {
    if is_immediate(word) {
        let v = int_immediate_value(word);
        (v < 0, crate::nat::small_limbs(v.unsigned_abs(), scratch))
    } else {
        unsafe { as_int(word) }.parts()
    }
}

/// The bignum path of a binary `Nat` operation (at least one operand is
/// boxed), returning the canonical word of the resulting magnitude.
unsafe fn nat_slow(a: i64, b: i64, op: impl FnOnce(&[u32], &[u32]) -> Result<Vec<u32>>) -> i64 {
    let (mut sa, mut sb) = ([0u32; 2], [0u32; 2]);
    let x = unsafe { nat_limbs(a, &mut sa) };
    let y = unsafe { nat_limbs(b, &mut sb) };
    match op(x, y) {
        Ok(limbs) => nat_word(Nat::from_limbs(limbs)),
        Err(e) => raise(e),
    }
}

/// The bignum path of a binary `Int` operation (at least one operand is
/// boxed), returning the canonical word of the result.
unsafe fn int_slow(
    a: i64,
    b: i64,
    op: impl FnOnce(bool, &[u32], bool, &[u32]) -> Result<Int>,
) -> i64 {
    let (mut sa, mut sb) = ([0u32; 2], [0u32; 2]);
    let (an, x) = unsafe { int_parts(a, &mut sa) };
    let (bn, y) = unsafe { int_parts(b, &mut sb) };
    match op(an, x, bn, y) {
        Ok(v) => int_word(v),
        Err(e) => raise(e),
    }
}

fn ordering_word(o: std::cmp::Ordering) -> i64 {
    o as i64
}

/// Read a `Bytes` word, faulting on null or wrong-kind blocks. Both
/// buffer kinds read: heap buffers (`KIND_BYTES`) and read-only
/// data-section buffers (`KIND_BYTES_STATIC`, a string literal's
/// backing).
pub(crate) unsafe fn as_bytes<'a>(word: i64) -> &'a crate::bytes::Bytes {
    let p = word as *const Boxed<crate::bytes::Bytes>;
    if p.is_null() {
        raise(Error::CorruptValue);
    }
    let b = unsafe { &*p };
    if b.kind != KIND_BYTES && b.kind != KIND_BYTES_STATIC {
        raise(Error::CorruptValue);
    }
    &b.value
}

/// Read a `Bytes` word for mutation. Buffer writes are part of the
/// language-level `bytesSet` contract, so this is a deliberate mutable
/// borrow at the extern boundary; generated code sequences the writes.
/// A read-only data-section buffer (`KIND_BYTES_STATIC`, a string
/// literal's backing) takes the defined read-only fault instead of a
/// write to constant data.
unsafe fn as_bytes_mut<'a>(word: i64) -> &'a mut crate::bytes::Bytes {
    let p = word as *mut Boxed<crate::bytes::Bytes>;
    if p.is_null() {
        raise(Error::CorruptValue);
    }
    let b = unsafe { &mut *p };
    if b.kind != KIND_BYTES {
        if b.kind == KIND_BYTES_STATIC {
            raise(Error::ReadOnly);
        }
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
/// with `__cinch_blackhole` in the code generator.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_blackhole() -> ! {
    std::process::exit(EXIT_BLACKHOLE)
}

/// Generic fault: write `msg` to stderr and exit with code 71. A null
/// pointer or zero length writes no message.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_panic(msg: *const u8, len: usize) -> ! {
    match unsafe { bytes(msg, len) } {
        Ok(m) => panic_exit(m),
        Err(_) => panic_exit(b"panic with corrupt message pointer"),
    }
}

/// Division by zero fault. Exits with code 72.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_fault_div_zero() -> ! {
    div_zero_exit()
}

// ---------------------------------------------------------------------------
// Allocation
// ---------------------------------------------------------------------------

/// Validate an allocation request's size and header kind.
fn alloc_request(size: i64, kind: i64, negative: &[u8], bad_kind: &[u8]) -> (usize, u8) {
    if size < 0 {
        panic_exit(negative);
    }
    if !(1..=i64::from(crate::block::KIND_RAW)).contains(&kind) {
        panic_exit(bad_kind);
    }
    (size as usize, kind as u8)
}

/// Allocate a zero-filled heap block of `size` bytes with header kind
/// `kind` (a `KIND_*` of [`crate::block`]) and return it as a word.
///
/// The size is rounded up to a whole number of 64-bit words (minimum one
/// word). Negative sizes, an unknown kind, and allocation failures fault
/// through `cinch_panic`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_alloc(size: i64, kind: i64) -> i64 {
    let (size, kind) = alloc_request(
        size,
        kind,
        b"cinch_alloc called with negative size",
        b"cinch_alloc called with an unknown block kind",
    );
    (unsafe { crate::block::allocate(size, kind, crate::block::RC_FRESH, true) }) as usize as i64
}

/// Allocate a block of `size` uninitialised bytes (8-byte aligned, minimum
/// one word) with header kind `kind` and return the payload pointer: the
/// allocator generated code calls in place of `malloc`, so every block it
/// builds is counted and capped by [`crate::heap`]. The header at offset -8
/// gets `rc = 1`, `kind`, and the payload size ([`crate::block`]). Every
/// call site initialises each word it reads, so the payload is not
/// zero-filled (the same contract as the `malloc` it replaces). Negative
/// sizes, an unknown kind, size overflow, and allocation failure fault
/// through `cinch_panic`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_alloc_raw(size: i64, kind: i64) -> *mut u8 {
    let (size, kind) = alloc_request(
        size,
        kind,
        b"cinch_alloc_raw called with negative size",
        b"cinch_alloc_raw called with an unknown block kind",
    );
    unsafe { crate::block::allocate(size, kind, crate::block::RC_FRESH, false) }
}

/// Deallocate the header block behind `word`: the raw release of a block
/// whose count has reached zero. It does not visit the block's children;
/// it only releases what a runtime box owns outside its block (a `Nat`'s
/// limbs, a heap buffer's data), then returns the block to the allocator
/// with the size its header records.
///
/// An immediate word (low bit 1), the null word, and an immortal block
/// (`rc == 0`: static blocks and interned `Sym` cells) are ignored. Any
/// other word must be a live heap block allocated by this runtime.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_free(word: i64) {
    if word == 0 || word & 1 != 0 {
        return;
    }
    let p = word as usize as *mut u8;
    let h = unsafe { crate::block::read_header(p) };
    if crate::block::rc(h) == crate::block::RC_IMMORTAL {
        return;
    }
    unsafe {
        drop_box_payload(p, crate::block::kind(h));
        crate::block::free(p);
    }
    crate::heap::account_block_free();
}

// ---------------------------------------------------------------------------
// Nat
// ---------------------------------------------------------------------------

/// An `i64` as a `Nat` word (always an immediate), faulting on negative
/// input.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_nat_from_i64(v: i64) -> i64 {
    if v < 0 {
        raise(Error::NegativeInput);
    }
    nat_immediate(v as u64)
}

/// The value of a `Nat` word as an `i64`, faulting when out of range.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_nat_to_i64(word: i64) -> i64 {
    if is_immediate(word) {
        return nat_immediate_value(word) as i64;
    }
    match unsafe { as_nat(word) }.to_i64() {
        Ok(v) => v,
        Err(e) => raise(e),
    }
}

/// 1 when the `Nat` word fits in an `i64`, else 0.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_nat_fits_i64(word: i64) -> i64 {
    if is_immediate(word) {
        return 1;
    }
    i64::from(unsafe { as_nat(word) }.fits_i64())
}

/// `a + b` for `Nat` words.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_nat_add(a: i64, b: i64) -> i64 {
    if is_immediate(a & b) {
        let sum = u128::from(nat_immediate_value(a)) + u128::from(nat_immediate_value(b));
        return nat_word_u128(sum);
    }
    unsafe { nat_slow(a, b, |x, y| Ok(crate::limbs::add_limbs(x, y))) }
}

/// `a - b` for `Nat` words, faulting on negative result.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_nat_sub(a: i64, b: i64) -> i64 {
    if is_immediate(a & b) {
        return match nat_immediate_value(a).checked_sub(nat_immediate_value(b)) {
            Some(d) => nat_immediate(d),
            None => raise(Error::NegativeResult),
        };
    }
    unsafe { nat_slow(a, b, crate::nat::sub_mag) }
}

/// `a * b` for `Nat` words.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_nat_mul(a: i64, b: i64) -> i64 {
    if is_immediate(a & b) {
        let product = u128::from(nat_immediate_value(a)) * u128::from(nat_immediate_value(b));
        return nat_word_u128(product);
    }
    unsafe { nat_slow(a, b, |x, y| Ok(crate::limbs::mul_limbs(x, y))) }
}

/// `a / b` for `Nat` words (the quotient; the remainder is available
/// through [`cinch_nat_rem`]), faulting on division by zero.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_nat_div(a: i64, b: i64) -> i64 {
    if is_immediate(a & b) {
        return match nat_immediate_value(a).checked_div(nat_immediate_value(b)) {
            Some(q) => nat_immediate(q),
            None => raise(Error::DivByZero),
        };
    }
    unsafe { nat_slow(a, b, |x, y| crate::nat::div_rem_mag(x, y).map(|(q, _)| q)) }
}

/// `a % b` for `Nat` words, faulting on division by zero.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_nat_rem(a: i64, b: i64) -> i64 {
    if is_immediate(a & b) {
        return match nat_immediate_value(a).checked_rem(nat_immediate_value(b)) {
            Some(r) => nat_immediate(r),
            None => raise(Error::DivByZero),
        };
    }
    unsafe { nat_slow(a, b, |x, y| crate::nat::div_rem_mag(x, y).map(|(_, r)| r)) }
}

/// Three-way comparison of `Nat` words: -1, 0 or 1.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_nat_cmp(a: i64, b: i64) -> i64 {
    if is_immediate(a & b) {
        return ordering_word(nat_immediate_value(a).cmp(&nat_immediate_value(b)));
    }
    let (mut sa, mut sb) = ([0u32; 2], [0u32; 2]);
    let x = unsafe { nat_limbs(a, &mut sa) };
    let y = unsafe { nat_limbs(b, &mut sb) };
    ordering_word(crate::limbs::cmp_limbs(x, y))
}

/// Parse ASCII decimal digits at `ptr` into a `Nat` word, faulting on
/// malformed input.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_nat_from_decimal(ptr: *const u8, len: usize) -> i64 {
    let text = match unsafe { bytes(ptr, len) } {
        Ok(t) => t,
        Err(e) => raise(e),
    };
    if let Some(v) = crate::nat::parse_small(text) {
        return nat_word_u128(u128::from(v));
    }
    match Nat::from_decimal(text) {
        Ok(v) => nat_word(v),
        Err(e) => raise(e),
    }
}

/// Write the decimal digits of a `Nat` word into `buf`, returning the
/// length written. Faults when `cap` is smaller than the digits (the
/// buffer is then untouched).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_nat_to_decimal(word: i64, buf: *mut u8, cap: usize) -> usize {
    let len = unsafe { cinch_nat_decimal_len(word) };
    if buf.is_null() {
        raise(Error::CorruptValue);
    }
    if cap < len {
        raise(Error::BufferTooSmall);
    }
    let dst = unsafe { std::slice::from_raw_parts_mut(buf, cap) };
    let written = if is_immediate(word) {
        crate::nat::write_u64_decimal(nat_immediate_value(word), false, dst)
    } else {
        unsafe { as_nat(word) }.to_decimal(dst)
    };
    match written {
        Ok(n) => n,
        Err(e) => raise(e),
    }
}

/// Number of ASCII bytes [`cinch_nat_to_decimal`] would write.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_nat_decimal_len(word: i64) -> usize {
    if is_immediate(word) {
        return crate::nat::u64_decimal_len(nat_immediate_value(word), false);
    }
    unsafe { as_nat(word) }.decimal_len()
}

// ---------------------------------------------------------------------------
// Int
// ---------------------------------------------------------------------------

/// An `i64` as an `Int` word (an immediate inside the immediate range).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_int_from_i64(v: i64) -> i64 {
    int_word_i128(i128::from(v))
}

/// The value of an `Int` word as an `i64`, faulting when out of range.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_int_to_i64(word: i64) -> i64 {
    if is_immediate(word) {
        return int_immediate_value(word);
    }
    match unsafe { as_int(word) }.to_i64() {
        Ok(v) => v,
        Err(e) => raise(e),
    }
}

/// 1 when the `Int` word fits in an `i64`, else 0.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_int_fits_i64(word: i64) -> i64 {
    if is_immediate(word) {
        return 1;
    }
    i64::from(unsafe { as_int(word) }.fits_i64())
}

/// `a + b` for `Int` words.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_int_add(a: i64, b: i64) -> i64 {
    if is_immediate(a & b) {
        let sum = i128::from(int_immediate_value(a)) + i128::from(int_immediate_value(b));
        return int_word_i128(sum);
    }
    unsafe { int_slow(a, b, |an, x, bn, y| Ok(crate::int::add_parts(an, x, bn, y))) }
}

/// `a - b` for `Int` words.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_int_sub(a: i64, b: i64) -> i64 {
    if is_immediate(a & b) {
        let diff = i128::from(int_immediate_value(a)) - i128::from(int_immediate_value(b));
        return int_word_i128(diff);
    }
    unsafe { int_slow(a, b, |an, x, bn, y| Ok(crate::int::add_parts(an, x, !bn, y))) }
}

/// `a * b` for `Int` words.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_int_mul(a: i64, b: i64) -> i64 {
    if is_immediate(a & b) {
        let product = i128::from(int_immediate_value(a)) * i128::from(int_immediate_value(b));
        return int_word_i128(product);
    }
    unsafe { int_slow(a, b, |an, x, bn, y| Ok(crate::int::mul_parts(an, x, bn, y))) }
}

/// `a / b` for `Int` words, truncated toward zero, faulting on division by
/// zero.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_int_div(a: i64, b: i64) -> i64 {
    if is_immediate(a & b) {
        // `-2^62 / -1` is the one quotient that leaves the immediate
        // range; `int_word_i128` boxes it.
        let (x, y) = (int_immediate_value(a), int_immediate_value(b));
        return match i128::from(x).checked_div(i128::from(y)) {
            Some(q) => int_word_i128(q),
            None => raise(Error::DivByZero),
        };
    }
    unsafe { int_slow(a, b, |an, x, bn, y| crate::int::div_rem_parts(an, x, bn, y).map(|(q, _)| q)) }
}

/// `a % b` for `Int` words, with the sign of the dividend, faulting on
/// division by zero.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_int_rem(a: i64, b: i64) -> i64 {
    if is_immediate(a & b) {
        return match int_immediate_value(a).checked_rem(int_immediate_value(b)) {
            Some(r) => int_immediate(r),
            None => raise(Error::DivByZero),
        };
    }
    unsafe { int_slow(a, b, |an, x, bn, y| crate::int::div_rem_parts(an, x, bn, y).map(|(_, r)| r)) }
}

/// `-a` for an `Int` word.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_int_neg(a: i64) -> i64 {
    if is_immediate(a) {
        return int_word_i128(-i128::from(int_immediate_value(a)));
    }
    // A box can negate into the immediate range (`2^62` to `-2^62`).
    int_word(unsafe { as_int(a) }.neg())
}

/// Three-way comparison of `Int` words: -1, 0 or 1.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_int_cmp(a: i64, b: i64) -> i64 {
    if is_immediate(a & b) {
        return ordering_word(int_immediate_value(a).cmp(&int_immediate_value(b)));
    }
    let (mut sa, mut sb) = ([0u32; 2], [0u32; 2]);
    let (an, x) = unsafe { int_parts(a, &mut sa) };
    let (bn, y) = unsafe { int_parts(b, &mut sb) };
    ordering_word(crate::int::cmp_parts(an, x, bn, y))
}

/// Parse an optionally signed ASCII decimal numeral at `ptr` into an `Int`
/// word, faulting on malformed input.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_int_from_decimal(ptr: *const u8, len: usize) -> i64 {
    let text = match unsafe { bytes(ptr, len) } {
        Ok(t) => t,
        Err(e) => raise(e),
    };
    let (negative, digits) = match text.split_first() {
        Some((b'-', rest)) => (true, rest),
        Some((b'+', rest)) => (false, rest),
        _ => (false, text),
    };
    if let Some(m) = crate::nat::parse_small(digits) {
        let v = i128::from(m);
        return int_word_i128(if negative { -v } else { v });
    }
    match Int::from_decimal(text) {
        Ok(v) => int_word(v),
        Err(e) => raise(e),
    }
}

/// Write the decimal representation of an `Int` word (sign included) into
/// `buf`, returning the length written. Faults when `cap` is too small
/// (the buffer is then untouched).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_int_to_decimal(word: i64, buf: *mut u8, cap: usize) -> usize {
    let len = unsafe { cinch_int_decimal_len(word) };
    if buf.is_null() {
        raise(Error::CorruptValue);
    }
    if cap < len {
        raise(Error::BufferTooSmall);
    }
    let dst = unsafe { std::slice::from_raw_parts_mut(buf, cap) };
    let written = if is_immediate(word) {
        let v = int_immediate_value(word);
        crate::nat::write_u64_decimal(v.unsigned_abs(), v < 0, dst)
    } else {
        unsafe { as_int(word) }.to_decimal(dst)
    };
    match written {
        Ok(n) => n,
        Err(e) => raise(e),
    }
}

/// Number of ASCII bytes [`cinch_int_to_decimal`] would write.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_int_decimal_len(word: i64) -> usize {
    if is_immediate(word) {
        let v = int_immediate_value(word);
        return crate::nat::u64_decimal_len(v.unsigned_abs(), v < 0);
    }
    unsafe { as_int(word) }.decimal_len()
}

/// A zeroed `Bytes` block of `len` bytes, faulting on negative length.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_bytes_new(len: i64) -> i64 {
    match crate::bytes::Bytes::new(len) {
        Ok(b) => box_bytes(b),
        Err(e) => raise(e),
    }
}

/// The length in bytes of a `Bytes` word.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_bytes_len(word: i64) -> i64 {
    unsafe { as_bytes(word) }.len()
}

/// The byte (0..=255) at `index`, faulting out of bounds.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_bytes_get(word: i64, index: i64) -> i64 {
    match unsafe { as_bytes(word) }.get(index) {
        Ok(v) => v,
        Err(e) => raise(e),
    }
}

/// Write `value` (0..=255) at `index`; returns 0 (the Unit word),
/// faulting on out-of-bounds index or value.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_bytes_set(word: i64, index: i64, value: i64) -> i64 {
    match unsafe { as_bytes_mut(word) }.set(index, value) {
        Ok(()) => 0,
        Err(e) => raise(e),
    }
}

/// Copy `n` bytes of `src` from `si` into `dst` at `di` (one `memmove`:
/// the two words may be the same buffer and the ranges may overlap);
/// returns 0 (the Unit word). A negative index or count or a range past
/// either end takes the out-of-bounds fault; a read-only `dst` takes the
/// read-only fault. A zero count is a no-op, as the Cinch-level loop it
/// replaces was.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_bytes_copy(dst: i64, src: i64, di: i64, si: i64, n: i64) -> i64 {
    if n == 0 {
        return 0;
    }
    // Each borrow ends before the next begins: `dst` and `src` may name
    // one buffer, and only raw pointers cross into the copy.
    let (dp, dl) = {
        let d = unsafe { as_bytes_mut(dst) };
        (d.data_ptr(), d.len())
    };
    let (sp, sl) = {
        let s = unsafe { as_bytes(src) };
        (s.data_ptr(), s.len())
    };
    match unsafe { crate::bytes::copy_range(dp, dl, di, sp, sl, si, n) } {
        Ok(()) => 0,
        Err(e) => raise(e),
    }
}

/// The lexicographic order of two `Bytes` words' contents: -1, 0 or 1,
/// read in place (no copy, no allocation).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_bytes_compare(a: i64, b: i64) -> i64 {
    let a = unsafe { as_bytes(a) }.as_slice();
    let b = unsafe { as_bytes(b) }.as_slice();
    a.cmp(b) as i64
}

/// The data pointer of a `Bytes` word: how buffer contents cross to
/// pointer-and-length interfaces (the UTF-8 intrinsics and the canonical
/// ABI glue). The pointer is valid as long as the word lives; words are
/// never freed (see the module docs).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_bytes_data(word: i64) -> i64 {
    unsafe { as_bytes(word) }.as_slice().as_ptr() as usize as i64
}

/// A fresh `Bytes` word holding a copy of `len` bytes at `ptr` (the
/// canonical ABI copies its lists and strings; zero-copy needs its own
/// sharing arrangement). Negative lengths and null pointers with a
/// positive length fault.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_bytes_from_data(ptr: *const u8, len: i64) -> i64 {
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
// reference consumed by `cinch_scope_close`); a handle word is an
// `Arc<TaskState>` reference (never freed twice; see the module docs of
// `crate::task`). The task outcome and closure ABI conventions are
// documented in `crate::task`.

/// Open a structured scope: the `withScope` entry (concurrency
/// specification section 2).
#[unsafe(no_mangle)]
pub extern "C" fn cinch_scope_open() -> i64 {
    let scope = std::sync::Arc::new(crate::task::Scope::new());
    Box::into_raw(Box::new(scope)) as usize as i64
}

/// Scope exit: cancel every unjoined child, wait for each cleanup
/// acknowledgment, and release the scope.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_scope_close(scope: i64) -> i64 {
    let owned = unsafe { Box::from_raw(scope as *mut std::sync::Arc<crate::task::Scope>) };
    owned.close();
    drop(owned);
    0
}

/// Spawn the body closure as a task of `scope`; returns the task handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_task_spawn(scope: i64, closure: i64) -> i64 {
    let scope = unsafe { &*(scope as *const std::sync::Arc<crate::task::Scope>) };
    scope.spawn(closure)
}

/// `joinAllOrCancel` over the `n` handles at `handles`: 0 and the `Ok`
/// payloads in `out` on success, 1 and the first `Err` payload in
/// `out[0]` on failure (the first failure cancels the siblings).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_task_join_all_or_cancel(
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
pub extern "C" fn cinch_task_poll_cancel() -> i64 {
    i64::from(crate::task::current_task_canceled())
}

/// Write a `Bytes` word to the runtime's output sink (stdout), flushed so
/// an observer sees it immediately; returns 0 (the Unit word).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_io_write(word: i64) -> i64 {
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
pub unsafe extern "C" fn cinch_io_read(len: i64) -> i64 {
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
pub extern "C" fn cinch_io_sleep(millis: i64) -> i64 {
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
            let w = cinch_nat_from_i64(41);
            let sum = cinch_nat_add(w, w);
            assert_eq!(cinch_nat_to_i64(sum), 82);
            assert_eq!(cinch_nat_cmp(w, sum), -1);
            assert_eq!(cinch_nat_cmp(sum, w), 1);
            assert_eq!(cinch_nat_cmp(w, w), 0);

            let i = cinch_int_from_i64(-5);
            let j = cinch_int_mul(i, i);
            assert_eq!(cinch_int_to_i64(j), 25);
            assert_eq!(cinch_int_to_i64(cinch_int_neg(i)), 5);
        }
    }

    #[test]
    fn decimal_entry_points_round_trip() {
        unsafe {
            let text = b"123456789012345678901234567890";
            let w = cinch_nat_from_decimal(text.as_ptr(), text.len());
            assert_eq!(cinch_nat_decimal_len(w), text.len());
            let mut buf = [0u8; 32];
            let n = cinch_nat_to_decimal(w, buf.as_mut_ptr(), buf.len());
            assert_eq!(&buf[..n], text);

            let t2 = b"-42";
            let i = cinch_int_from_decimal(t2.as_ptr(), t2.len());
            assert_eq!(cinch_int_decimal_len(i), 3);
            let n = cinch_int_to_decimal(i, buf.as_mut_ptr(), buf.len());
            assert_eq!(&buf[..n], t2);
        }
    }

    fn nat_dec(s: &str) -> i64 {
        unsafe { cinch_nat_from_decimal(s.as_ptr(), s.len()) }
    }

    fn int_dec(s: &str) -> i64 {
        unsafe { cinch_int_from_decimal(s.as_ptr(), s.len()) }
    }

    fn nat_text(w: i64) -> String {
        let mut buf = [0u8; 64];
        let n = unsafe { cinch_nat_to_decimal(w, buf.as_mut_ptr(), buf.len()) };
        assert_eq!(n, unsafe { cinch_nat_decimal_len(w) });
        String::from_utf8(buf[..n].to_vec()).unwrap()
    }

    fn int_text(w: i64) -> String {
        let mut buf = [0u8; 64];
        let n = unsafe { cinch_int_to_decimal(w, buf.as_mut_ptr(), buf.len()) };
        assert_eq!(n, unsafe { cinch_int_decimal_len(w) });
        String::from_utf8(buf[..n].to_vec()).unwrap()
    }

    #[test]
    fn nat_words_are_canonical_across_the_immediate_edge() {
        unsafe {
            let top = cinch_nat_from_i64(i64::MAX);
            assert!(is_immediate(top), "2^63-1 is the largest immediate");
            assert_eq!(top, nat_dec("9223372036854775807"));
            let one = cinch_nat_from_i64(1);
            // Overflow promotes: 2^63 is a box, and so is 2^64.
            let edge = cinch_nat_add(top, one);
            assert!(!is_immediate(edge));
            assert_eq!(nat_text(edge), "9223372036854775808");
            assert_eq!(cinch_nat_fits_i64(edge), 0);
            let wide = cinch_nat_mul(edge, cinch_nat_from_i64(2));
            assert_eq!(nat_text(wide), "18446744073709551616");
            // Demotion: a box result back in range is the immediate word,
            // equal to the word every other path produces for that value.
            assert_eq!(cinch_nat_sub(edge, one), top);
            assert_eq!(cinch_nat_div(wide, edge), cinch_nat_from_i64(2));
            assert_eq!(cinch_nat_rem(wide, cinch_nat_from_i64(10)), cinch_nat_from_i64(6));
            // Mixed operands compare by value.
            assert_eq!(cinch_nat_cmp(edge, top), 1);
            assert_eq!(cinch_nat_cmp(top, edge), -1);
            assert_eq!(cinch_nat_cmp(edge, nat_dec("9223372036854775808")), 0);
            assert_eq!(nat_text(top), "9223372036854775807");
            assert_eq!(nat_text(cinch_nat_from_i64(0)), "0");
            assert_eq!(nat_dec("0000"), cinch_nat_from_i64(0));
        }
    }

    #[test]
    fn int_words_are_canonical_across_both_immediate_edges() {
        unsafe {
            let max = cinch_int_from_i64(INT_IMMEDIATE_MAX);
            let min = cinch_int_from_i64(INT_IMMEDIATE_MIN);
            assert!(is_immediate(max) && is_immediate(min));
            let one = cinch_int_from_i64(1);
            let above = cinch_int_add(max, one);
            let below = cinch_int_sub(min, one);
            assert!(!is_immediate(above) && !is_immediate(below));
            assert_eq!(cinch_int_to_i64(above), 1 << 62);
            assert_eq!(cinch_int_to_i64(below), -(1 << 62) - 1);
            assert_eq!(int_text(above), "4611686018427387904");
            assert_eq!(int_text(min), "-4611686018427387904");
            // Negation crosses the asymmetric edge both ways.
            let neg_min = cinch_int_neg(min);
            assert!(!is_immediate(neg_min));
            assert_eq!(cinch_int_cmp(neg_min, above), 0);
            assert_eq!(cinch_int_neg(above), min);
            assert_eq!(cinch_int_neg(max), cinch_int_add(min, one));
            assert_eq!(cinch_int_to_i64(cinch_int_div(min, cinch_int_from_i64(-1))), 1 << 62);
            assert_eq!(cinch_int_sub(above, one), max);
            assert_eq!(cinch_int_add(below, one), min);
            // The i64 edges are boxes and convert back exactly.
            for v in [i64::MIN, i64::MAX, i64::MIN + 1] {
                let w = cinch_int_from_i64(v);
                assert!(!is_immediate(w));
                assert_eq!(cinch_int_to_i64(w), v);
                assert_eq!(int_text(w), v.to_string());
                assert_eq!(cinch_int_cmp(w, int_dec(&v.to_string())), 0);
            }
            assert_eq!(int_dec("-0"), cinch_int_from_i64(0));
            assert_eq!(int_dec("+17"), cinch_int_from_i64(17));
            assert_eq!(cinch_int_rem(cinch_int_from_i64(-7), cinch_int_from_i64(3)), cinch_int_from_i64(-1));
            assert_eq!(cinch_int_cmp(below, min), -1);
            assert_eq!(cinch_int_cmp(min, below), 1);
        }
    }

    #[test]
    fn alloc_zero_fills_and_writes_a_fresh_header() {
        unsafe {
            let p = cinch_alloc(24, i64::from(crate::block::KIND_MEMO));
            assert_ne!(p, 0);
            let slice = std::slice::from_raw_parts(p as *const u8, 24);
            assert!(slice.iter().all(|&b| b == 0));
            let h = crate::block::read_header(p as *const u8);
            assert_eq!(crate::block::rc(h), 1);
            assert_eq!(crate::block::kind(h), crate::block::KIND_MEMO);
            assert_eq!(crate::block::layout(h), 3);
        }
    }

    /// `cinch_free` releases mortal blocks (including a bignum's limbs and a
    /// buffer's data, through the payload drop) and leaves immediates,
    /// immortal statics and interned symbols alone. Only this test frees in
    /// this binary, so the `freed` counter delta is exact.
    #[test]
    fn free_releases_mortal_blocks_and_ignores_immortal_words() {
        let freed = || crate::heap::block_stats().3;
        unsafe {
            let before = freed();
            let ctor = cinch_alloc_raw(16, i64::from(crate::block::KIND_CTOR));
            ctor.cast::<i64>().write(1);
            ctor.cast::<i64>().add(1).write(42);
            cinch_free(ctor as usize as i64);
            let digits = b"123456789012345678901234567890123456789";
            cinch_free(cinch_nat_from_decimal(digits.as_ptr(), digits.len()));
            let buf = cinch_bytes_new(4096);
            cinch_bytes_set(buf, 4095, 7);
            cinch_free(buf);
            assert_eq!(freed() - before, 3);

            cinch_free(0);
            cinch_free(0x2b);
            static STATIC_BLK: [u64; 2] = [crate::block::static_header(crate::block::KIND_CTOR, 1), 0];
            cinch_free(STATIC_BLK.as_ptr().add(1) as usize as i64);
            let sym = crate::sym::intern(b"free-ignores-me").unwrap();
            cinch_free(sym);
            assert_eq!(as_sym(sym).name(), b"free-ignores-me");
            assert_eq!(crate::sym::intern(b"free-ignores-me").unwrap(), sym);
            assert_eq!(freed() - before, 3);

            // The payload drop really releases the box's out-of-block
            // memory: 64 MiB of buffers and bignum limbs churned through
            // `cinch_free` leave live bytes about where they were. Other
            // tests allocate concurrently, hence the slack.
            let live = || crate::heap::stats().2;
            let start = live();
            let huge = cinch_nat_mul(
                cinch_nat_from_decimal(digits.as_ptr(), digits.len()),
                cinch_nat_from_decimal(digits.as_ptr(), digits.len()),
            );
            for _ in 0..1024 {
                cinch_free(cinch_bytes_new(64 << 10));
                cinch_free(cinch_nat_mul(huge, huge));
            }
            assert!(live() < start + (16 << 20), "live {} -> {}", start, live());
        }
    }

    #[test]
    fn error_exit_codes_match_the_fault_paths() {
        assert_eq!(Error::DivByZero.exit_code(), EXIT_DIV_ZERO);
        assert_eq!(Error::NegativeResult.exit_code(), EXIT_PANIC);
    }
}
