//! Mathematical natural numbers with unbounded exact semantics.
//!
//! A `Nat` is a sign-magnitude value whose sign is always non-negative; the
//! magnitude is a base 2^32 `Vec<u32>`, normalized (no leading zero limbs).
//! Zero is represented by the empty magnitude and is non-negative.

use std::cmp::Ordering;

use crate::error::{Error, Result};
use crate::limbs;

/// An unbounded natural number.
#[derive(Clone, Debug)]
pub struct Nat {
    limbs: Vec<u32>,
}

impl Nat {
    /// The natural number zero.
    pub fn zero() -> Nat {
        Nat { limbs: Vec::new() }
    }

    /// Whether the value is zero.
    pub fn is_zero(&self) -> bool {
        limbs::is_zero(&self.limbs)
    }

    /// Convert from an `i64`, faulting on negative input.
    pub fn from_i64(v: i64) -> Result<Nat> {
        if v < 0 {
            return Err(Error::NegativeInput);
        }
        Ok(Nat::from_u64(v as u64))
    }

    /// Convert to an `i64`, faulting when the value does not fit.
    pub fn to_i64(&self) -> Result<i64> {
        let u = magnitude_to_u64(&self.limbs)?;
        if u > i64::MAX as u64 {
            return Err(Error::OutOfRange);
        }
        Ok(u as i64)
    }

    /// The value modulo 2^64: its two lowest limbs.
    pub fn low_u64(&self) -> u64 {
        let lo = self.limbs.first().copied().unwrap_or(0) as u64;
        let hi = self.limbs.get(1).copied().unwrap_or(0) as u64;
        lo | (hi << 32)
    }

    /// Whether the value fits in an `i64`.
    pub fn fits_i64(&self) -> bool {
        self.to_i64().is_ok()
    }

    /// `self + other`.
    pub fn add(&self, other: &Nat) -> Nat {
        Nat {
            limbs: limbs::add_limbs(&self.limbs, &other.limbs),
        }
    }

    /// `self - other`, faulting when the result would be negative.
    pub fn sub(&self, other: &Nat) -> Result<Nat> {
        sub_mag(&self.limbs, &other.limbs).map(|limbs| Nat { limbs })
    }

    /// `self * other`.
    pub fn mul(&self, other: &Nat) -> Nat {
        Nat {
            limbs: limbs::mul_limbs(&self.limbs, &other.limbs),
        }
    }

    /// `(self / other, self % other)` with truncating division, faulting on
    /// division by zero.
    pub fn div_rem(&self, other: &Nat) -> Result<(Nat, Nat)> {
        let (q, r) = div_rem_mag(&self.limbs, &other.limbs)?;
        Ok((Nat { limbs: q }, Nat { limbs: r }))
    }

    /// `self % other`, faulting on division by zero.
    pub fn rem(&self, other: &Nat) -> Result<Nat> {
        Ok(self.div_rem(other)?.1)
    }

    /// Three-way comparison.
    pub fn cmp(&self, other: &Nat) -> Ordering {
        limbs::cmp_limbs(&self.limbs, &other.limbs)
    }

    /// Parse ASCII decimal digits into a natural number.
    ///
    /// Well-formed input is one or more ASCII digits (leading zeros are
    /// allowed). Empty input, signs, whitespace, and any other byte are
    /// malformed and fault.
    pub fn from_decimal(bytes: &[u8]) -> Result<Nat> {
        parse_digits(bytes)
            .map(|limbs| Nat { limbs })
            .ok_or(Error::MalformedDecimal)
    }

    /// The number of ASCII digits `to_decimal` would write (at least 1).
    pub fn decimal_len(&self) -> usize {
        decimal_len(&self.limbs)
    }

    /// Write the decimal digits into `buf` and return the length written.
    ///
    /// Faults with [`Error::BufferTooSmall`] when `buf` is shorter than
    /// [`Nat::decimal_len`] bytes; the buffer is then left untouched.
    pub fn to_decimal(&self, buf: &mut [u8]) -> Result<usize> {
        write_decimal(&self.limbs, false, buf)
    }

    /// Convert from a `u64`.
    pub fn from_u64(v: u64) -> Nat {
        Nat::from_u128(u128::from(v))
    }

    /// Convert from a `u128`.
    pub(crate) fn from_u128(mut v: u128) -> Nat {
        let mut limbs = Vec::new();
        while v > 0 {
            limbs.push(v as u32);
            v >>= 32;
        }
        Nat { limbs }
    }

    /// The value as a `u64`, when it fits.
    pub(crate) fn to_u64(&self) -> Option<u64> {
        magnitude_to_u64(&self.limbs).ok()
    }

    /// The value as a `u128`, faulting when it does not fit.
    pub(crate) fn to_u128_exact(&self) -> Result<u128> {
        magnitude_to_u128(&self.limbs)
    }

    /// Wrap a normalized magnitude (the output of the `limbs` operations
    /// and of the magnitude functions below).
    pub(crate) fn from_limbs(limbs: Vec<u32>) -> Nat {
        debug_assert!(limbs.last() != Some(&0), "magnitude not normalized");
        Nat { limbs }
    }

    /// The normalized little-endian magnitude.
    pub(crate) fn limbs(&self) -> &[u32] {
        &self.limbs
    }
}

/// The normalized magnitude of a `u64`, held in `scratch` so an immediate
/// operand reaches the shared limb arithmetic without allocating.
pub(crate) fn small_limbs(v: u64, scratch: &mut [u32; 2]) -> &[u32] {
    scratch[0] = v as u32;
    scratch[1] = (v >> 32) as u32;
    let len = if v == 0 {
        0
    } else if v >> 32 == 0 {
        1
    } else {
        2
    };
    &scratch[..len]
}

/// `a - b` on magnitudes, faulting when the result would be negative.
pub(crate) fn sub_mag(a: &[u32], b: &[u32]) -> Result<Vec<u32>> {
    if limbs::cmp_limbs(a, b) == Ordering::Less {
        return Err(Error::NegativeResult);
    }
    Ok(limbs::sub_limbs(a, b))
}

/// `(a / b, a % b)` on magnitudes, faulting on division by zero.
pub(crate) fn div_rem_mag(a: &[u32], b: &[u32]) -> Result<(Vec<u32>, Vec<u32>)> {
    if limbs::is_zero(b) {
        return Err(Error::DivByZero);
    }
    Ok(limbs::div_rem_limbs(a, b))
}

/// Powers of 10 for O(1) decimal digit counting (`POW10_U64[0] == 0` so `v = 0`
/// yields length 1 without a branch).
const POW10_U64: [u64; 20] = [
    0,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
    10_000_000_000_000,
    100_000_000_000_000,
    1_000_000_000_000_000,
    10_000_000_000_000_000,
    100_000_000_000_000_000,
    1_000_000_000_000_000_000,
    10_000_000_000_000_000_000,
];

/// Two-digit ASCII decimal lookup table (`"00".."99"`).
const DIGIT_PAIRS: &[u8; 200] = b"\
0001020304050607080910111213141516171819\
2021222324252627282930313233343536373839\
4041424344454647484950515253545556575859\
6061626364656667686970717273747576777879\
8081828384858687888990919293949596979899";

/// Base for 64-bit decimal chunking: `10^19`, whose MSB (bit 63) is set so
/// Möller–Granlund 2-by-1 division needs zero normalization shift.
const DEC_BASE_U64: u64 = 10_000_000_000_000_000_000;
const DEC_BASE_DIGITS: usize = 19;

/// Base-case block size (in 19-digit chunks) and entry threshold for
/// divide-and-conquer bignum decimal parsing and formatting.
const DC_PARSE_BASE_CHUNKS: usize = 24;
const DC_PARSE_BASE_DIGITS: usize = DC_PARSE_BASE_CHUNKS * DEC_BASE_DIGITS;
const DC_PARSE_ENTRY_DIGITS: usize = DC_PARSE_BASE_DIGITS * 2;
const DC_FORMAT_BASE_CHUNKS: usize = 64;
const DC_FORMAT_BASE_DIGITS: usize = DC_FORMAT_BASE_CHUNKS * DEC_BASE_DIGITS;
const DC_FORMAT_ENTRY_DIGITS: usize = DC_FORMAT_BASE_DIGITS * 3;

/// Validate and convert 8 ASCII decimal digits simultaneously using Lemire's
/// SWAR technique. Returns `None` if any byte is outside `b'0'..=b'9'`.
#[inline(always)]
fn parse_8_digits(bytes: &[u8]) -> Option<u32> {
    let mut val = u64::from_le_bytes(bytes[..8].try_into().ok()?);
    // High nibbles of LHS are `val & 0xF0..`; equality with `0x33..` forces
    // every byte into `0x30..=0x3F`, so adding `0x06` per byte can never carry
    // across lanes, and its shifted high nibbles equal `0x03` iff every byte
    // is `<= 0x39`.
    if ((val & 0xF0F0_F0F0_F0F0_F0F0)
        | ((val.wrapping_add(0x0606_0606_0606_0606) & 0xF0F0_F0F0_F0F0_F0F0) >> 4))
        != 0x3333_3333_3333_3333
    {
        return None;
    }
    val &= 0x0F0F_0F0F_0F0F_0F0F;
    val = (val.wrapping_mul(1 + (10 << 8)) >> 8) & 0x00FF_00FF_00FF_00FF;
    val = (val.wrapping_mul(1 + (100 << 16)) >> 16) & 0x0000_FFFF_0000_FFFF;
    Some((val as u32) * 10_000 + (val >> 32) as u32)
}

/// Validate and convert 4 ASCII decimal digits simultaneously using SWAR.
#[inline(always)]
fn parse_4_digits(bytes: &[u8]) -> Option<u32> {
    let mut val = u32::from_le_bytes(bytes[..4].try_into().ok()?);
    if ((val & 0xF0F0_F0F0) | ((val.wrapping_add(0x0606_0606) & 0xF0F0_F0F0) >> 4)) != 0x3333_3333 {
        return None;
    }
    val &= 0x0F0F_0F0F;
    val = (val.wrapping_mul(1 + (10 << 8)) >> 8) & 0x00FF_00FF;
    Some((val & 0xFF) * 100 + (val >> 16))
}

/// The value of an all-digit numeral short enough to be a `u64` (at most
/// 19 digits, which never overflows), or `None` for anything else: the
/// caller falls back to the full parser, which also owns the errors.
#[inline(always)]
pub(crate) fn parse_small(bytes: &[u8]) -> Option<u64> {
    let len = bytes.len();
    if len == 0 || len > 19 {
        return None;
    }
    if len == 1 {
        let d = bytes[0].wrapping_sub(b'0');
        return if d <= 9 { Some(u64::from(d)) } else { None };
    }
    let mut acc = 0u64;
    let mut i = 0usize;
    if len >= 16 {
        let hi = u64::from(parse_8_digits(&bytes[0..8])?);
        let lo = u64::from(parse_8_digits(&bytes[8..16])?);
        acc = hi * 100_000_000 + lo;
        i = 16;
    } else if len >= 8 {
        acc = u64::from(parse_8_digits(&bytes[0..8])?);
        i = 8;
        if len - i >= 4 {
            acc = acc * 10_000 + u64::from(parse_4_digits(&bytes[8..12])?);
            i = 12;
        }
    } else if len >= 4 {
        acc = u64::from(parse_4_digits(&bytes[0..4])?);
        i = 4;
    }
    while i < len {
        let d = bytes[i].wrapping_sub(b'0');
        if d > 9 {
            return None;
        }
        acc = acc * 10 + u64::from(d);
        i += 1;
    }
    Some(acc)
}

/// Number of decimal digits of `v` (1..=20), in O(1) via `leading_zeros` and
/// `POW10_U64` (Kendall Willets / Lemire).
#[inline(always)]
fn u64_digit_count(v: u64) -> usize {
    let t = (((64 - (v | 1).leading_zeros()) * 1233) >> 12) as usize;
    t + 1 - usize::from(v < POW10_U64[t])
}

/// Number of ASCII bytes [`write_u64_decimal`] writes for `v`.
#[inline(always)]
pub(crate) fn u64_decimal_len(v: u64, negative: bool) -> usize {
    u64_digit_count(v) + usize::from(negative && v != 0)
}

#[inline(always)]
fn write_pair(n: usize, dst: &mut [u8]) {
    let idx = n * 2;
    dst[0] = DIGIT_PAIRS[idx];
    dst[1] = DIGIT_PAIRS[idx + 1];
}

/// Write exactly 8 zero-padded decimal digits of `v` (`v < 100_000_000`).
#[inline(always)]
fn write_8_digits(v: u32, dst: &mut [u8]) {
    debug_assert!(v < 100_000_000 && dst.len() == 8);
    let hi = v / 10_000;
    let lo = v - hi * 10_000;
    let a = hi / 100;
    let b = hi - a * 100;
    let c = lo / 100;
    let d = lo - c * 100;
    write_pair(a as usize, &mut dst[0..2]);
    write_pair(b as usize, &mut dst[2..4]);
    write_pair(c as usize, &mut dst[4..6]);
    write_pair(d as usize, &mut dst[6..8]);
}

/// Write exactly 19 zero-padded decimal digits of `v` (`v < 10^19`).
#[inline(always)]
fn write_19_digits(v: u64, dst: &mut [u8]) {
    debug_assert!(v < DEC_BASE_U64 && dst.len() == 19);
    let hi11 = v / 100_000_000;
    let lo8 = (v - hi11 * 100_000_000) as u32;
    let top3 = (hi11 / 100_000_000) as u32;
    let mid8 = (hi11 - (top3 as u64) * 100_000_000) as u32;
    let d0 = top3 / 100;
    let d12 = (top3 - d0 * 100) as usize;
    dst[0] = b'0' + d0 as u8;
    write_pair(d12, &mut dst[1..3]);
    write_8_digits(mid8, &mut dst[3..11]);
    write_8_digits(lo8, &mut dst[11..19]);
}

/// Write `v` (`v < 100_000_000`) into `dst`, whose length equals `digit_count(v)`.
#[inline(always)]
fn write_u32_exact(mut v: u32, dst: &mut [u8]) {
    let mut pos = dst.len();
    while v >= 100 {
        let q = v / 100;
        let r = (v - q * 100) as usize;
        v = q;
        pos -= 2;
        write_pair(r, &mut dst[pos..pos + 2]);
    }
    if v >= 10 {
        write_pair(v as usize, &mut dst[0..2]);
    } else {
        dst[0] = b'0' + v as u8;
    }
}

/// Write `v` into `dst`, whose length equals `u64_digit_count(v)`.
#[inline(always)]
fn write_u64_exact(v: u64, dst: &mut [u8]) {
    if v < 10 {
        dst[0] = b'0' + v as u8;
        return;
    }
    if v < 100 {
        write_pair(v as usize, &mut dst[0..2]);
        return;
    }
    if v < 100_000_000 {
        write_u32_exact(v as u32, dst);
        return;
    }
    let len = dst.len();
    let q = v / 100_000_000;
    let r = (v - q * 100_000_000) as u32;
    write_8_digits(r, &mut dst[len - 8..len]);
    if q < 100_000_000 {
        write_u32_exact(q as u32, &mut dst[..len - 8]);
    } else {
        let q2 = (q / 100_000_000) as u32;
        let r2 = (q - (q2 as u64) * 100_000_000) as u32;
        write_8_digits(r2, &mut dst[len - 16..len - 8]);
        write_u32_exact(q2, &mut dst[..len - 16]);
    }
}

/// [`write_decimal`] for a magnitude that fits a `u64`, without the
/// chunk vectors the bignum path allocates.
#[inline(always)]
pub(crate) fn write_u64_decimal(v: u64, negative: bool, buf: &mut [u8]) -> Result<usize> {
    if v < 10 {
        let sign = usize::from(negative && v != 0);
        let total = 1 + sign;
        if buf.len() < total {
            return Err(Error::BufferTooSmall);
        }
        if sign != 0 {
            buf[0] = b'-';
        }
        buf[sign] = b'0' + v as u8;
        return Ok(total);
    }
    let sign = usize::from(negative);
    let total = u64_digit_count(v) + sign;
    if buf.len() < total {
        return Err(Error::BufferTooSmall);
    }
    if sign != 0 {
        buf[0] = b'-';
    }
    write_u64_exact(v, &mut buf[sign..total]);
    Ok(total)
}

impl PartialEq for Nat {
    fn eq(&self, other: &Nat) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Nat {}

impl PartialOrd for Nat {
    fn partial_cmp(&self, other: &Nat) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Nat {
    fn cmp(&self, other: &Nat) -> Ordering {
        limbs::cmp_limbs(&self.limbs, &other.limbs)
    }
}

/// Magnitude to `u64`, faulting when more than two limbs are present.
/// A normalized magnitude of at most four limbs as a `u128`.
pub(crate) fn magnitude_to_u128(v: &[u32]) -> Result<u128> {
    if v.len() > 4 {
        return Err(Error::OutOfRange);
    }
    Ok(v.iter().rev().fold(0u128, |acc, l| (acc << 32) | u128::from(*l)))
}

pub(crate) fn magnitude_to_u64(v: &[u32]) -> Result<u64> {
    match v.len() {
        0 => Ok(0),
        1 => Ok(v[0] as u64),
        2 => Ok((v[0] as u64) | ((v[1] as u64) << 32)),
        _ => Err(Error::OutOfRange),
    }
}

/// Exact bit length of a normalized magnitude (0 for zero).
#[inline]
fn bit_len(limbs: &[u32]) -> usize {
    match limbs.last() {
        None => 0,
        Some(&top) => (limbs.len() - 1) * 32 + (32 - top.leading_zeros() as usize),
    }
}

/// Shift a normalized magnitude right by `bits`.
fn shr_limbs(limbs: &[u32], bits: usize) -> Vec<u32> {
    let word_shift = bits / 32;
    if word_shift >= limbs.len() {
        return Vec::new();
    }
    let bit_shift = bits % 32;
    if bit_shift == 0 {
        return limbs[word_shift..].to_vec();
    }
    let rshift = bit_shift;
    let lshift = 32 - bit_shift;
    let n = limbs.len() - word_shift;
    let mut out = Vec::with_capacity(n);
    for i in 0..n - 1 {
        out.push((limbs[word_shift + i] >> rshift) | (limbs[word_shift + i + 1] << lshift));
    }
    let top = limbs[limbs.len() - 1] >> rshift;
    if top != 0 {
        out.push(top);
    }
    limbs::normalize(&mut out);
    out
}

/// Shift a normalized magnitude left by `bits`.
fn shl_limbs(limbs: &[u32], bits: usize) -> Vec<u32> {
    if limbs.is_empty() || bits == 0 {
        return limbs.to_vec();
    }
    let word_shift = bits / 32;
    let bit_shift = bits % 32;
    let mut out = vec![0u32; word_shift + limbs.len() + usize::from(bit_shift != 0)];
    if bit_shift == 0 {
        out[word_shift..word_shift + limbs.len()].copy_from_slice(limbs);
        return out;
    }
    let rshift = 32 - bit_shift;
    let mut carry = 0u32;
    for (i, &w) in limbs.iter().enumerate() {
        out[word_shift + i] = (w << bit_shift) | carry;
        carry = w >> rshift;
    }
    if carry != 0 {
        out[word_shift + limbs.len()] = carry;
    } else {
        out.pop();
    }
    out
}

/// Compute `(hi << bits) | (src_low & ((1 << bits) - 1))`, normalized.
fn shl_or_low(hi: &[u32], bits: usize, src_low: &[u32]) -> Vec<u32> {
    let word_shift = bits / 32;
    let bit_shift = bits % 32;
    if hi.is_empty() {
        let take = (word_shift + usize::from(bit_shift != 0)).min(src_low.len());
        let mut out = src_low[..take].to_vec();
        if bit_shift != 0 && out.len() > word_shift {
            out[word_shift] &= (1u32 << bit_shift) - 1;
        }
        limbs::normalize(&mut out);
        return out;
    }
    let mut out = shl_limbs(hi, bits);
    let copy_words = word_shift.min(src_low.len());
    out[..copy_words].copy_from_slice(&src_low[..copy_words]);
    if bit_shift != 0 && word_shift < src_low.len() {
        out[word_shift] |= src_low[word_shift] & ((1u32 << bit_shift) - 1);
    }
    limbs::normalize(&mut out);
    out
}

/// Increment a normalized magnitude in place by 1.
#[inline]
fn add_one_in_place(v: &mut Vec<u32>) {
    for w in v.iter_mut() {
        let (next, overflow) = w.overflowing_add(1);
        *w = next;
        if !overflow {
            return;
        }
    }
    v.push(1);
}

/// Compute `2^exp - x` for normalized `x <= 2^exp`.
fn sub_pow2(exp: usize, x: &[u32]) -> Vec<u32> {
    let word = exp / 32;
    let bit = exp % 32;
    let mut pow2 = vec![0u32; word + 1];
    pow2[word] = 1u32 << bit;
    limbs::sub_limbs(&pow2, x)
}

/// Top 63 bits of a normalized magnitude with `bits >= 63`: `floor(limbs / 2^(bits - 63))`.
#[inline]
fn top_63_bits(limbs: &[u32], bits: usize) -> u64 {
    debug_assert!(bits >= 63);
    let shift = bits - 63;
    let w = shift / 32;
    let b = shift % 32;
    let mut acc = 0u128;
    for i in (w..limbs.len()).rev() {
        acc = (acc << 32) | (limbs[i] as u128);
    }
    (acc >> b) as u64
}

/// Compare a normalized magnitude against `2^exp`.
#[inline]
fn ge_pow2(x: &[u32], exp: usize) -> bool {
    bit_len(x) > exp
}

/// Compute `floor(2^(b + target_k) / d)` (or within 3 below it, which Barrett
/// division corrects in constant time) via truncated quadratic Newton–Raphson
/// using Karatsuba `limbs::mul_limbs`. Precondition: `b = bit_len(d) >= 63`,
/// `target_k >= 62`.
fn newton_reciprocal(d: &[u32], b: usize, target_k: usize) -> Vec<u32> {
    debug_assert!(b >= 63 && target_k >= 62);
    let d63 = top_63_bits(d, b);
    // Initial 62-bit lower bound: x0 = floor(2^125 / (d63 + 1)) <= 2^(b+62) / d.
    let x0 = ((1u128 << 125) / (u128::from(d63) + 1)) as u64;
    let mut x = Vec::with_capacity(2);
    let lo = x0 as u32;
    let hi = (x0 >> 32) as u32;
    if hi != 0 {
        x.push(lo);
        x.push(hi);
    } else if lo != 0 {
        x.push(lo);
    }
    let mut k = 62usize;
    while k < target_k {
        let s = (k - 2).min(target_k - k);
        // Truncate `d` to at most `2*k + 8` top bits (rounded up so `x * d_trunc`
        // overestimates `x * d`, preserving the lower-bound invariant `x * d <= 2^(b+k)`).
        let drop = b.saturating_sub(2 * k + 8);
        let b_trunc = b - drop;
        let prod = if drop == 0 {
            limbs::mul_limbs(&x, d)
        } else {
            let mut d_trunc = shr_limbs(d, drop);
            add_one_in_place(&mut d_trunc);
            limbs::mul_limbs(&x, &d_trunc)
        };
        let shifted = shl_limbs(&x, s);
        if ge_pow2(&prod, b_trunc + k) {
            // `x` is already accurate to `b_trunc` bits; no correction needed.
            x = shifted;
        } else {
            let err = sub_pow2(b_trunc + k, &prod);
            // Dropping `e_drop = rem_shift - (k + 8)` low bits of `err` changes
            // `(x * err) >> rem_shift` by `< x / 2^(k+8) <= 2^(-7)` while shrinking
            // `err_hi` to at most `s + 9` bits.
            let rem_shift = b_trunc + k - s;
            let corr = if rem_shift > k + 8 {
                let e_drop = rem_shift - (k + 8);
                let err_hi = shr_limbs(&err, e_drop);
                shr_limbs(&limbs::mul_limbs(&x, &err_hi), k + 8)
            } else {
                shr_limbs(&limbs::mul_limbs(&x, &err), rem_shift)
            };
            x = limbs::add_limbs(&shifted, &corr);
        }
        k += s;
    }
    x
}

/// Compute `(u / d, u % d)` using a precomputed Newton/Barrett reciprocal
/// `inv <= floor(2^(b + k) / d)` where `b = bit_len(d)` and `bit_len(u) <= b + k`.
fn barrett_div_rem(u: &[u32], d: &[u32], b: usize, k: usize, inv: &[u32]) -> (Vec<u32>, Vec<u32>) {
    if limbs::cmp_limbs(u, d) == Ordering::Less {
        return (Vec::new(), u.to_vec());
    }
    if bit_len(u) > b + k {
        return limbs::div_rem_limbs(u, d);
    }
    let u_hi = shr_limbs(u, b - 1);
    let mut q = shr_limbs(&limbs::mul_limbs(&u_hi, inv), k + 1);
    let qd = limbs::mul_limbs(&q, d);
    let mut r = limbs::sub_limbs(u, &qd);
    while limbs::cmp_limbs(&r, d) != Ordering::Less {
        r = limbs::sub_limbs(&r, d);
        add_one_in_place(&mut q);
    }
    (q, r)
}

/// Compute `5^exp` via binary exponentiation using `limbs::mul_limbs`.
fn pow5_limbs(exp: usize) -> Vec<u32> {
    if exp == 0 {
        return vec![1];
    }
    // 5^13 = 1_220_703_125 fits in u32; use 5^27 = 7_450_580_596_923_828_125 (u64)
    // as a compact starting base when exp is small.
    let mut res = vec![1u32];
    let mut base = vec![5u32];
    let mut e = exp;
    while e > 0 {
        if e & 1 != 0 {
            res = limbs::mul_limbs(&res, &base);
        }
        e >>= 1;
        if e > 0 {
            base = limbs::mul_limbs(&base, &base);
        }
    }
    res
}

/// Exact Möller–Granlund 2-by-1 division of `(u1 * 2^64 + u0)` by `10^19`,
/// requiring `u1 < 10^19`. Zero hardware divisions.
#[inline(always)]
fn div_rem_by_1e19(u1: u64, u0: u64) -> (u64, u64) {
    const D: u64 = DEC_BASE_U64;
    const V: u64 = ((u128::MAX / (D as u128)) - (1u128 << 64)) as u64;
    debug_assert!(u1 < D);
    let q = (u1 as u128) * (V as u128) + (((u1 as u128) << 64) | (u0 as u128));
    let mut q1 = ((q >> 64) as u64).wrapping_add(1);
    let q0 = q as u64;
    let mut r = u0.wrapping_sub(q1.wrapping_mul(D));
    if r > q0 {
        q1 = q1.wrapping_sub(1);
        r = r.wrapping_add(D);
    }
    if r >= D {
        q1 = q1.wrapping_add(1);
        r = r.wrapping_sub(D);
    }
    (q1, r)
}

/// Convert normalized base-2^32 limbs into little-endian base-2^64 words.
fn limbs_to_u64(limbs: &[u32]) -> Vec<u64> {
    let mut out = Vec::with_capacity(limbs.len().div_ceil(2));
    let mut i = 0;
    while i + 1 < limbs.len() {
        out.push((limbs[i] as u64) | ((limbs[i + 1] as u64) << 32));
        i += 2;
    }
    if i < limbs.len() {
        out.push(limbs[i] as u64);
    }
    out
}

/// Convert little-endian base-2^64 words into normalized base-2^32 limbs.
fn u64_to_limbs(words: &[u64]) -> Vec<u32> {
    let mut out = Vec::with_capacity(words.len() * 2);
    for &w in words {
        out.push(w as u32);
        out.push((w >> 32) as u32);
    }
    limbs::normalize(&mut out);
    out
}

/// `words = words * 10^19 + add` in place (base-2^64 words).
#[inline]
fn mul_add_u64_1e19(words: &mut Vec<u64>, add: u64) {
    let mut carry = add as u128;
    for w in words.iter_mut() {
        let t = (*w as u128) * (DEC_BASE_U64 as u128) + carry;
        *w = t as u64;
        carry = t >> 64;
    }
    if carry > 0 {
        words.push(carry as u64);
    }
}

/// Parse a non-empty slice of ASCII digits into normalized limbs using
/// base-2^64 accumulation with 19-digit SWAR chunks.
fn parse_digits_base(bytes: &[u8]) -> Option<Vec<u32>> {
    if bytes.len() <= DEC_BASE_DIGITS {
        let v = parse_small(bytes)?;
        if v == 0 {
            return Some(Vec::new());
        }
        let lo = v as u32;
        let hi = (v >> 32) as u32;
        return Some(if hi == 0 { vec![lo] } else { vec![lo, hi] });
    }
    let mut words = Vec::with_capacity(bytes.len() / 19 + 2);
    let mut idx = 0usize;
    let first_take = {
        let r = bytes.len() % DEC_BASE_DIGITS;
        if r == 0 { DEC_BASE_DIGITS } else { r }
    };
    let first_val = parse_small(&bytes[..first_take])?;
    if first_val != 0 {
        words.push(first_val);
    }
    idx += first_take;
    while idx < bytes.len() {
        let chunk = &bytes[idx..idx + DEC_BASE_DIGITS];
        let a = u64::from(parse_8_digits(&chunk[0..8])?);
        let b = u64::from(parse_8_digits(&chunk[8..16])?);
        let d16 = chunk[16].wrapping_sub(b'0');
        let d17 = chunk[17].wrapping_sub(b'0');
        let d18 = chunk[18].wrapping_sub(b'0');
        if d16 > 9 || d17 > 9 || d18 > 9 {
            return None;
        }
        let tail = u64::from(d16) * 100 + u64::from(d17) * 10 + u64::from(d18);
        let val = (a * 100_000_000 + b) * 1_000 + tail;
        mul_add_u64_1e19(&mut words, val);
        idx += DEC_BASE_DIGITS;
    }
    Some(u64_to_limbs(&words))
}

/// Recursive divide-and-conquer parser using a precomputed ladder
/// `pow5[j] = 5^(DC_PARSE_BASE_DIGITS << j)`.
fn parse_digits_dc(bytes: &[u8], max_level: usize, pow5: &[Vec<u32>]) -> Option<Vec<u32>> {
    if bytes.len() <= DC_PARSE_BASE_DIGITS {
        return parse_digits_base(bytes);
    }
    let mut level = max_level;
    while level > 0 && (DC_PARSE_BASE_DIGITS << level) >= bytes.len() {
        level -= 1;
    }
    let split = DC_PARSE_BASE_DIGITS << level;
    debug_assert!(split < bytes.len());
    let (hi_bytes, lo_bytes) = bytes.split_at(bytes.len() - split);
    let hi = parse_digits_dc(hi_bytes, level, pow5)?;
    let lo = parse_digits_dc(lo_bytes, level, pow5)?;
    if hi.is_empty() {
        return Some(lo);
    }
    let hi_pow5 = limbs::mul_limbs(&hi, &pow5[level]);
    let hi_scaled = shl_limbs(&hi_pow5, split);
    if lo.is_empty() {
        Some(hi_scaled)
    } else {
        Some(limbs::add_limbs(&hi_scaled, &lo))
    }
}

/// Parse `bytes` as ASCII digits into a normalized magnitude.
pub(crate) fn parse_digits(bytes: &[u8]) -> Option<Vec<u32>> {
    if bytes.is_empty() {
        return None;
    }
    // Strip leading zeros quickly (all stripped bytes are known to be b'0').
    let bytes = if bytes[0] == b'0' {
        match bytes.iter().position(|&b| b != b'0') {
            None => return Some(Vec::new()),
            Some(pos) => &bytes[pos..],
        }
    } else {
        bytes
    };
    if bytes.len() <= DC_PARSE_ENTRY_DIGITS {
        return parse_digits_base(bytes);
    }
    // Build power-of-5 ladder: pow5[j] = 5^(DC_PARSE_BASE_DIGITS << j).
    let mut pow5 = Vec::new();
    let mut span = DC_PARSE_BASE_DIGITS;
    let mut cur = pow5_limbs(span);
    loop {
        pow5.push(cur);
        let next_span = span << 1;
        if next_span >= bytes.len() {
            break;
        }
        cur = limbs::mul_limbs(pow5.last().unwrap(), pow5.last().unwrap());
        span = next_span;
    }
    parse_digits_dc(bytes, pow5.len() - 1, &pow5)
}

/// Extract 19-digit decimal chunks (least significant first) using base-2^64
/// Möller–Granlund division by `10^19`.
fn decimal_chunks_base(limbs: &[u32], out: &mut Vec<u64>) {
    if limbs.is_empty() {
        return;
    }
    let mut words = limbs_to_u64(limbs);
    while !words.is_empty() {
        let mut rem = 0u64;
        for w in words.iter_mut().rev() {
            let (q, r) = div_rem_by_1e19(rem, *w);
            *w = q;
            rem = r;
        }
        while words.last() == Some(&0) {
            words.pop();
        }
        out.push(rem);
    }
}

/// One level of the precomputed divide-and-conquer base-conversion ladder:
/// splits at `10^digits = 5^digits * 2^digits`.
struct FormatLevel {
    digits: usize,
    chunks: usize,
    pow5: Vec<u32>,
    b: usize,
    k: usize,
    inv: Vec<u32>,
}

/// Recursively split `val` into 19-digit chunks (least significant first).
/// When `exact_chunks > 0`, pads with trailing zero chunks up to `exact_chunks`
/// so upper limbs align with their decimal place value.
fn decimal_chunks_dc(
    val: &[u32],
    level: usize,
    levels: &[FormatLevel],
    exact_chunks: usize,
    out: &mut Vec<u64>,
) {
    let start = out.len();
    if val.is_empty() {
        if exact_chunks > 0 {
            out.resize(start + exact_chunks, 0);
        }
        return;
    }
    if val.len() <= 2 {
        let v = (val[0] as u64) | (val.get(1).copied().unwrap_or(0) as u64) << 32;
        if v >= DEC_BASE_U64 {
            let q = v / DEC_BASE_U64;
            out.push(v - q * DEC_BASE_U64);
            out.push(q);
        } else {
            out.push(v);
        }
        if exact_chunks > 0 {
            out.resize(start + exact_chunks, 0);
        }
        return;
    }
    // Find largest level <= `level` whose 10^digits might not exceed `val`.
    let mut lvl = level;
    let vbits = bit_len(val);
    loop {
        let fl = &levels[lvl];
        if fl.b + fl.digits <= vbits {
            break;
        }
        if lvl == 0 {
            decimal_chunks_base(val, out);
            if exact_chunks > 0 {
                out.resize(start + exact_chunks, 0);
            }
            return;
        }
        lvl -= 1;
    }
    let fl = &levels[lvl];
    // Split val = q * 10^digits + r via val = ((val >> digits) / 5^digits) * 10^digits + ...
    let u_hi = shr_limbs(val, fl.digits);
    let (q, r5) = barrett_div_rem(&u_hi, &fl.pow5, fl.b, fl.k, &fl.inv);
    let r = shl_or_low(&r5, fl.digits, val);
    if q.is_empty() {
        if lvl == 0 {
            decimal_chunks_base(&r, out);
        } else {
            decimal_chunks_dc(&r, lvl - 1, levels, 0, out);
        }
    } else if lvl == 0 {
        decimal_chunks_base(&r, out);
        out.resize(start + fl.chunks, 0);
        // `q` may still exceed 10^digits_0 if `lvl == 0` was the top level;
        // `decimal_chunks_base` handles any size cleanly.
        decimal_chunks_base(&q, out);
    } else {
        decimal_chunks_dc(&r, lvl - 1, levels, fl.chunks, out);
        // If `lvl` was the top balanced split (`digits` in `[N/3, 2N/3]`),
        // `q` can still be >= 10^digits, so allow reusing `lvl` when needed.
        decimal_chunks_dc(&q, lvl, levels, 0, out);
    }
    if exact_chunks > 0 {
        out.resize(start + exact_chunks, 0);
    }
}

/// Split a magnitude into 19-digit decimal chunks, least significant first.
///
/// Each chunk is in `0..10^19`; the most significant chunk is non-zero unless
/// the magnitude is zero (no chunks at all).
fn decimal_chunks(limbs: &[u32]) -> Vec<u64> {
    if limbs.is_empty() {
        return Vec::new();
    }
    let vbits = bit_len(limbs);
    // Upper bound on total digits: bits * log10(2) + 1.
    let est_digits = (((vbits as u128) * 1_292_913_987) >> 32) as usize + 1;
    let est_chunks = est_digits / DEC_BASE_DIGITS + 2;
    let mut chunks = Vec::with_capacity(est_chunks);
    if est_digits <= DC_FORMAT_ENTRY_DIGITS {
        decimal_chunks_base(limbs, &mut chunks);
        return chunks;
    }
    // Build power-of-5 ladder stopping near the midpoint (`<= 2/3 * est_digits`)
    // so the top split is balanced and never builds an oversized reciprocal.
    let max_split_digits = (est_digits * 2) / 3;
    let mut levels: Vec<FormatLevel> = Vec::new();
    let mut digits = DC_FORMAT_BASE_DIGITS;
    let mut chunk_span = DC_FORMAT_BASE_CHUNKS;
    let mut pow5 = pow5_limbs(digits);
    loop {
        let b = bit_len(&pow5);
        let next_digits = digits << 1;
        let is_top = next_digits > max_split_digits;
        // For non-top levels, val < 10^(2*digits) so u_hi < 2^(2*b + digits).
        // For the top level, u_hi has at most `vbits - digits` bits.
        let k = if is_top {
            (vbits.saturating_sub(digits)).saturating_sub(b).max(62) + 2
        } else {
            b + digits + 2
        };
        let inv = newton_reciprocal(&pow5, b, k);
        levels.push(FormatLevel {
            digits,
            chunks: chunk_span,
            pow5,
            b,
            k,
            inv,
        });
        if is_top {
            break;
        }
        let prev = &levels.last().unwrap().pow5;
        pow5 = limbs::mul_limbs(prev, prev);
        digits = next_digits;
        chunk_span <<= 1;
    }
    let top = levels.len() - 1;
    decimal_chunks_dc(limbs, top, &levels, 0, &mut chunks);
    while chunks.last() == Some(&0) {
        chunks.pop();
    }
    chunks
}

/// Number of ASCII digits the normalized magnitude formats to (at least 1).
pub(crate) fn decimal_len(limbs: &[u32]) -> usize {
    match limbs.len() {
        0 => 1,
        1 => u64_digit_count(limbs[0] as u64),
        2 => u64_digit_count((limbs[0] as u64) | ((limbs[1] as u64) << 32)),
        _ => {
            let bits = bit_len(limbs);
            // Exact integer bracket on log10(2):
            //   1_292_913_986 / 2^32 < log10(2) < 1_292_913_987 / 2^32.
            let lo = ((((bits - 1) as u128) * 1_292_913_986) >> 32) as usize + 1;
            let hi = (((bits as u128) * 1_292_913_987) >> 32) as usize + 1;
            if lo == hi {
                return lo;
            }
            if hi == lo + 1 {
                // Compare limbs against 10^lo = 5^lo * 2^lo via (limbs >> lo) vs 5^lo.
                let p5 = pow5_limbs(lo);
                let shifted = shr_limbs(limbs, lo);
                return if limbs::cmp_limbs(&shifted, &p5) == Ordering::Less {
                    lo
                } else {
                    hi
                };
            }
            let chunks = decimal_chunks(limbs);
            match chunks.len() {
                0 => 1,
                k => (k - 1) * DEC_BASE_DIGITS + u64_digit_count(chunks[k - 1]),
            }
        }
    }
}

/// Write the decimal representation of `limbs` into `buf`.
///
/// `negative` prepends a `-` sign. Returns the number of bytes written;
/// faults with [`Error::BufferTooSmall`] (without writing) when `buf` is too
/// short.
pub(crate) fn write_decimal(limbs: &[u32], negative: bool, buf: &mut [u8]) -> Result<usize> {
    match limbs.len() {
        0 => return write_u64_decimal(0, false, buf),
        1 => return write_u64_decimal(limbs[0] as u64, negative, buf),
        2 => {
            let v = (limbs[0] as u64) | ((limbs[1] as u64) << 32);
            return write_u64_decimal(v, negative, buf);
        }
        _ => {}
    }
    let chunks = decimal_chunks(limbs);
    let k = chunks.len();
    debug_assert!(k > 0);
    let top_width = u64_digit_count(chunks[k - 1]);
    let digits = (k - 1) * DEC_BASE_DIGITS + top_width;
    let sign = usize::from(negative);
    let total = digits + sign;
    if buf.len() < total {
        return Err(Error::BufferTooSmall);
    }
    if sign != 0 {
        buf[0] = b'-';
    }
    let mut pos = sign;
    write_u64_exact(chunks[k - 1], &mut buf[pos..pos + top_width]);
    pos += top_width;
    for &c in chunks[..k - 1].iter().rev() {
        write_19_digits(c, &mut buf[pos..pos + DEC_BASE_DIGITS]);
        pos += DEC_BASE_DIGITS;
    }
    debug_assert_eq!(pos, total);
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nat(v: u64) -> Nat {
        Nat::from_u64(v)
    }

    #[test]
    fn from_i64_rejects_negatives() {
        assert_eq!(Nat::from_i64(-1), Err(Error::NegativeInput));
        assert_eq!(Nat::from_i64(i64::MIN), Err(Error::NegativeInput));
        assert_eq!(Nat::from_i64(0).unwrap(), Nat::zero());
    }

    #[test]
    fn to_i64_boundaries() {
        assert_eq!(nat(i64::MAX as u64).to_i64(), Ok(i64::MAX));
        assert_eq!(Nat::from_i64(0).unwrap().to_i64(), Ok(0));
        // i64::MAX + 1 does not fit.
        let max_plus_one = nat(i64::MAX as u64).add(&nat(1));
        assert_eq!(max_plus_one.to_i64(), Err(Error::OutOfRange));
        assert!(!max_plus_one.fits_i64());
        assert!(nat(i64::MAX as u64).fits_i64());
        // 2^32 fits comfortably (two limbs).
        assert_eq!(nat(1 << 32).to_i64(), Ok(1 << 32));
    }

    #[test]
    fn add_sub_round_trip_and_underflow() {
        let a = nat(0xFFFF_FFFF).add(&nat(1)); // 2^32
        assert_eq!(a.sub(&nat(1)).unwrap(), nat(0xFFFF_FFFF));
        assert_eq!(a.sub(&a).unwrap(), Nat::zero());
        assert_eq!(a.sub(&a.add(&nat(1))), Err(Error::NegativeResult));
        assert_eq!(Nat::zero().sub(&nat(0)), Ok(Nat::zero()));
    }

    #[test]
    fn mul_and_div_rem_against_u64_reference() {
        for a in [0u64, 1, 0xFFFF_FFFF, 1 << 32, u64::MAX] {
            for b in [0u64, 1, 3, 0xFFFF_FFFF, 1 << 32, u64::MAX] {
                let (na, nb) = (nat(a), nat(b));
                let prod = na.mul(&nb);
                let mut buf = [0u8; 40];
                let len = prod.to_decimal(&mut buf).unwrap();
                assert_eq!(
                    std::str::from_utf8(&buf[..len]).unwrap(),
                    ((a as u128) * (b as u128)).to_string(),
                    "{a} * {b}"
                );
                if b == 0 {
                    assert_eq!(na.div_rem(&nb), Err(Error::DivByZero));
                    assert_eq!(na.rem(&nb), Err(Error::DivByZero));
                } else {
                    let (q, r) = na.div_rem(&nb).unwrap();
                    assert_eq!(q, nat(a / b), "{a} / {b}");
                    assert_eq!(r, nat(a % b), "{a} % {b}");
                    assert_eq!(q.mul(&nb).add(&r), na);
                }
            }
        }
    }

    #[test]
    fn div_by_zero_is_reported_and_traps_as_a_compiler_bug() {
        // The safe API reports the zero divisor; the extern layer can
        // only see one through a compiler bug (the divisor is `NonZero`).
        assert_eq!(Nat::zero().div_rem(&Nat::zero()), Err(Error::DivByZero));
        assert_eq!(Error::DivByZero.exit_code(), crate::abi::EXIT_PANIC);
    }

    #[test]
    fn cmp_orders_naturals() {
        assert_eq!(nat(1).cmp(&nat(2)), Ordering::Less);
        assert_eq!(nat(1 << 32).cmp(&nat(0xFFFF_FFFF)), Ordering::Greater);
        assert_eq!(Nat::zero().cmp(&Nat::zero()), Ordering::Equal);
        assert!(nat(5) < nat(6));
    }

    #[test]
    fn decimal_parse_format_round_trip() {
        // Explicit expectations: leading zeros collapse, zero stays "0".
        let cases: &[(&str, &str)] = &[
            ("0", "0"),
            ("000", "0"),
            ("1", "1"),
            ("9", "9"),
            ("10", "10"),
            ("123456789", "123456789"),
            ("1234567890", "1234567890"),
            ("0000000000000000005", "5"),
        ];
        for &(input, want) in cases {
            let v = Nat::from_decimal(input.as_bytes()).unwrap();
            let mut buf = [0u8; 64];
            let n = v.to_decimal(&mut buf).unwrap();
            assert_eq!(&buf[..n], want.as_bytes(), "input {input:?}");
            assert_eq!(v.decimal_len(), want.len(), "input {input:?}");
        }
    }

    #[test]
    fn decimal_round_trip_on_large_values() {
        let mut big = String::new();
        for i in 0..200 {
            big.push_str(&format!("{:09}", i * 7919 % 1_000_000_000));
        }
        let v = Nat::from_decimal(big.as_bytes()).unwrap();
        let mut buf = vec![0u8; big.len() + 8];
        let n = v.to_decimal(&mut buf).unwrap();
        let got = std::str::from_utf8(&buf[..n]).unwrap();
        // Leading zeros in the input collapse.
        assert_eq!(got, big.trim_start_matches('0'));
        // Round trip again through the parser.
        let v2 = Nat::from_decimal(got.as_bytes()).unwrap();
        assert_eq!(v, v2);
    }

    #[test]
    fn decimal_len_matches_written_length() {
        for s in ["0", "7", "999999999", "1000000000", "123456789012345678901234567890"] {
            let v = Nat::from_decimal(s.as_bytes()).unwrap();
            assert_eq!(v.decimal_len(), s.len(), "{s}");
            let mut buf = vec![0u8; s.len()];
            let n = v.to_decimal(&mut buf).unwrap();
            assert_eq!(n, s.len());
            assert_eq!(&buf[..n], s.as_bytes());
        }
    }

    #[test]
    fn to_decimal_faults_on_small_buffer_but_not_on_exact_cap() {
        let v = Nat::from_decimal(b"123456789012345678901").unwrap();
        let need = v.decimal_len();
        let mut exact = vec![0u8; need];
        assert_eq!(v.to_decimal(&mut exact), Ok(need));
        let mut short = vec![0u8; need - 1];
        assert_eq!(v.to_decimal(&mut short), Err(Error::BufferTooSmall));
        // The short buffer is untouched.
        assert!(short.iter().all(|&b| b == 0));
    }

    #[test]
    fn malformed_decimal_inputs_fault() {
        for s in ["", "+", "-", " 1", "1 ", "12a", "0x10", "1_0", "\u{6771}"] {
            assert_eq!(
                Nat::from_decimal(s.as_bytes()),
                Err(Error::MalformedDecimal),
                "{s:?}"
            );
        }
    }

    #[test]
    fn zero_has_empty_magnitude_and_is_non_negative() {
        let z = Nat::zero();
        assert!(z.is_zero());
        assert_eq!(z.decimal_len(), 1);
        assert_eq!(z.to_decimal(&mut [0u8; 1]), Ok(1));
    }

    #[test]
    fn u64_decimal_len_and_write_exhaustive_boundaries() {
        let mut vals = vec![0u64, 1, 2, 8, 9, 10, 11, 98, 99, 100, 101, u64::MAX - 1, u64::MAX];
        let mut p = 1u64;
        for _ in 0..19 {
            p *= 10;
            vals.push(p - 1);
            vals.push(p);
            if let Some(next) = p.checked_add(1) {
                vals.push(next);
            }
        }
        for k in 1..64u32 {
            let b = 1u64 << k;
            vals.push(b - 1);
            vals.push(b);
            vals.push(b + 1);
        }
        for v in vals {
            let s = v.to_string();
            assert_eq!(u64_decimal_len(v, false), s.len(), "pos len for {v}");
            let neg_s = if v == 0 { "0".to_string() } else { format!("-{s}") };
            assert_eq!(u64_decimal_len(v, true), neg_s.len(), "neg len for {v}");

            let mut buf = [0xAAu8; 32];
            let n = write_u64_decimal(v, false, &mut buf).unwrap();
            assert_eq!(&buf[..n], s.as_bytes(), "pos write for {v}");

            let n2 = write_u64_decimal(v, true, &mut buf).unwrap();
            assert_eq!(&buf[..n2], neg_s.as_bytes(), "neg write for {v}");

            // BufferTooSmall must leave buffer completely untouched.
            let mut short = vec![0x5Au8; s.len() - 1];
            assert_eq!(write_u64_decimal(v, false, &mut short), Err(Error::BufferTooSmall));
            assert!(short.iter().all(|&b| b == 0x5A));
        }
    }

    #[test]
    fn swar_parse_8_and_4_digits_brute_force_every_byte_and_lane() {
        // Test every byte 0..=255 at every lane position 0..8 for parse_8_digits.
        for lane in 0..8 {
            for byte in 0u8..=255 {
                let mut buf = *b"12345678";
                buf[lane] = byte;
                let got = parse_8_digits(&buf);
                if byte.is_ascii_digit() {
                    let want: u32 = std::str::from_utf8(&buf).unwrap().parse().unwrap();
                    assert_eq!(got, Some(want), "lane={lane} byte={byte:#x}");
                } else {
                    assert_eq!(got, None, "must reject lane={lane} byte={byte:#x}");
                }
            }
        }
        // Also test adjacent high-byte carry traps like 0xFF next to '0'.
        for lane in 0..7 {
            let mut buf = *b"00000000";
            buf[lane] = 0xFA;
            buf[lane + 1] = 0x2F;
            assert_eq!(parse_8_digits(&buf), None);
            buf[lane] = 0xFF;
            buf[lane + 1] = b'0';
            assert_eq!(parse_8_digits(&buf), None);
        }
        // Test every byte 0..=255 at every lane position 0..4 for parse_4_digits.
        for lane in 0..4 {
            for byte in 0u8..=255 {
                let mut buf = *b"9081";
                buf[lane] = byte;
                let got = parse_4_digits(&buf);
                if byte.is_ascii_digit() {
                    let want: u32 = std::str::from_utf8(&buf).unwrap().parse().unwrap();
                    assert_eq!(got, Some(want), "4d lane={lane} byte={byte:#x}");
                } else {
                    assert_eq!(got, None, "4d must reject lane={lane} byte={byte:#x}");
                }
            }
        }
        // Test parse_small across all lengths 0..=20 and bad bytes at every index.
        assert_eq!(parse_small(b""), None);
        assert_eq!(parse_small(b"12345678901234567890"), None);
        for len in 1..=19 {
            let s: String = (0..len).map(|i| (b'0' + ((i * 7 + 3) % 10) as u8) as char).collect();
            let want: u64 = s.parse().unwrap();
            assert_eq!(parse_small(s.as_bytes()), Some(want), "len={len}");
            for bad_pos in 0..len {
                for bad_byte in [0x00u8, b'/', b':', b' ', 0x80, 0xFF] {
                    let mut bad = s.as_bytes().to_vec();
                    bad[bad_pos] = bad_byte;
                    assert_eq!(parse_small(&bad), None, "len={len} pos={bad_pos} byte={bad_byte:#x}");
                }
            }
        }
    }

    #[test]
    fn moller_granlund_div_rem_1e19_matches_u128_reference() {
        const D: u64 = DEC_BASE_U64;
        let edge_u1 = [0u64, 1, 2, D / 2, D - 2, D - 1];
        let edge_u0 = [0u64, 1, 2, D - 1, D, D + 1, u64::MAX - 1, u64::MAX];
        for &u1 in &edge_u1 {
            for &u0 in &edge_u0 {
                let num = ((u1 as u128) << 64) | (u0 as u128);
                let want_q = (num / D as u128) as u64;
                let want_r = (num % D as u128) as u64;
                assert_eq!(div_rem_by_1e19(u1, u0), (want_q, want_r), "u1={u1} u0={u0}");
            }
        }
        let mut s = 0x1234_5678_9ABC_DEF0u64;
        for _ in 0..10_000 {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let u1 = s % D;
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let u0 = s;
            let num = ((u1 as u128) << 64) | (u0 as u128);
            assert_eq!(
                div_rem_by_1e19(u1, u0),
                ((num / D as u128) as u64, (num % D as u128) as u64)
            );
        }
    }

    #[test]
    fn dc_decimal_round_trip_powers_of_ten_zeros_and_huge_values() {
        // Exact powers of 10, 10^k - 1, 10^k + 1 across base and D&C thresholds.
        for &exp in &[18usize, 19, 20, 38, 100, 456, 912, 1215, 1216, 1217, 1823, 1824, 1825, 2500, 5000] {
            let p10 = format!("1{}", "0".repeat(exp));
            let p10_plus = format!("1{}1", "0".repeat(exp - 1));
            let p10_minus = "9".repeat(exp);
            for s in [p10, p10_plus, p10_minus] {
                let n = Nat::from_decimal(s.as_bytes()).unwrap();
                assert_eq!(n.decimal_len(), s.len(), "decimal_len for exp={exp} len={}", s.len());
                let mut buf = vec![0xCCu8; s.len()];
                let w = n.to_decimal(&mut buf).unwrap();
                assert_eq!(w, s.len());
                assert_eq!(&buf[..w], s.as_bytes(), "mismatch for exp={exp}");
                // Short buffer untouched.
                let mut short = vec![0x33u8; s.len() - 1];
                assert_eq!(n.to_decimal(&mut short), Err(Error::BufferTooSmall));
                assert!(short.iter().all(|&b| b == 0x33));
            }
        }
        // Large pseudo-random numbers up to 20,000 digits + sparse zero-filled middle.
        for &len in &[456usize, 457, 913, 2000, 5000, 20_000] {
            let mut s = String::with_capacity(len);
            let mut st = (len as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 1;
            for i in 0..len {
                st = st.wrapping_mul(6364136223846793005).wrapping_add(1);
                let d = ((st >> 32) % if i == 0 { 9 } else { 10 }) + if i == 0 { 1 } else { 0 };
                s.push((b'0' + d as u8) as char);
            }
            let n = Nat::from_decimal(s.as_bytes()).unwrap();
            assert_eq!(n.decimal_len(), len);
            let mut buf = vec![0u8; len];
            let w = n.to_decimal(&mut buf).unwrap();
            assert_eq!(w, len);
            assert_eq!(&buf[..w], s.as_bytes(), "random len={len}");
            // Malformed byte in the middle of a D&C input must fault.
            let mut bad = s.as_bytes().to_vec();
            bad[len / 2] = b'x';
            assert_eq!(Nat::from_decimal(&bad), Err(Error::MalformedDecimal));
        }
        // Leading zeros before a D&C-length number and all-zeros long string.
        let zeros = "0".repeat(3000);
        assert_eq!(Nat::from_decimal(zeros.as_bytes()).unwrap(), Nat::zero());
        let padded = format!("{zeros}123456789012345678901234567890");
        let n = Nat::from_decimal(padded.as_bytes()).unwrap();
        let mut buf = [0u8; 64];
        let w = n.to_decimal(&mut buf).unwrap();
        assert_eq!(&buf[..w], b"123456789012345678901234567890");
    }
}
