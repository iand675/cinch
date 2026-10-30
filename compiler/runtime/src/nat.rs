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

/// The value of an all-digit numeral short enough to be a `u64` (at most
/// 19 digits, which never overflows), or `None` for anything else: the
/// caller falls back to the full parser, which also owns the errors.
pub(crate) fn parse_small(bytes: &[u8]) -> Option<u64> {
    if bytes.is_empty() || bytes.len() > 19 || !bytes.iter().all(u8::is_ascii_digit) {
        return None;
    }
    Some(bytes.iter().fold(0u64, |acc, &b| acc * 10 + u64::from(b - b'0')))
}

/// Number of ASCII bytes [`write_u64_decimal`] writes for `v`.
pub(crate) fn u64_decimal_len(v: u64, negative: bool) -> usize {
    let mut n = 1;
    let mut rest = v / 10;
    while rest > 0 {
        rest /= 10;
        n += 1;
    }
    n + usize::from(negative && v != 0)
}

/// [`write_decimal`] for a magnitude that fits a `u64`, without the
/// chunk vectors the bignum path allocates.
pub(crate) fn write_u64_decimal(v: u64, negative: bool, buf: &mut [u8]) -> Result<usize> {
    let total = u64_decimal_len(v, negative);
    if buf.len() < total {
        return Err(Error::BufferTooSmall);
    }
    let mut rest = v;
    for slot in buf[..total].iter_mut().rev() {
        *slot = b'0' + (rest % 10) as u8;
        rest /= 10;
    }
    if negative && v != 0 {
        buf[0] = b'-';
    }
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
pub(crate) fn magnitude_to_u64(v: &[u32]) -> Result<u64> {
    match v.len() {
        0 => Ok(0),
        1 => Ok(v[0] as u64),
        2 => Ok((v[0] as u64) | ((v[1] as u64) << 32)),
        _ => Err(Error::OutOfRange),
    }
}

/// Parse `bytes` as ASCII digits into a normalized magnitude.
pub(crate) fn parse_digits(bytes: &[u8]) -> Option<Vec<u32>> {
    if bytes.is_empty() || !bytes.iter().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let mut limbs: Vec<u32> = Vec::new();
    // Horner over 9-digit groups, most significant group first so that
    // every group but the first holds exactly nine digits.
    let mut idx = 0usize;
    let mut first = true;
    while idx < bytes.len() {
        let take = if first {
            let r = bytes.len() % 9;
            if r == 0 {
                9
            } else {
                r
            }
        } else {
            9
        };
        first = false;
        let mut group = 0u32;
        for &b in &bytes[idx..idx + take] {
            group = group * 10 + (b - b'0') as u32;
        }
        idx += take;
        limbs::mul_add_small(&mut limbs, 1_000_000_000, group);
    }
    Some(limbs)
}

/// Split a magnitude into 9-digit decimal chunks, least significant first.
///
/// Each chunk is in `0..1_000_000_000`; the most significant chunk is
/// non-zero unless the magnitude is zero (no chunks at all).
fn decimal_chunks(limbs: &[u32]) -> Vec<u32> {
    let mut rest = limbs.to_vec();
    let mut chunks = Vec::new();
    while !rest.is_empty() {
        chunks.push(limbs::div_rem_by_word_in_place(&mut rest, 1_000_000_000));
    }
    chunks
}

/// Number of ASCII digits the normalized magnitude formats to (at least 1).
pub(crate) fn decimal_len(limbs: &[u32]) -> usize {
    let chunks = decimal_chunks(limbs);
    match chunks.len() {
        0 => 1,
        k => (k - 1) * 9 + digit_count(chunks[k - 1]),
    }
}

fn digit_count(mut v: u32) -> usize {
    let mut n = 1;
    while v >= 10 {
        v /= 10;
        n += 1;
    }
    n
}

/// Write the decimal representation of `limbs` into `buf`.
///
/// `negative` prepends a `-` sign. Returns the number of bytes written;
/// faults with [`Error::BufferTooSmall`] (without writing) when `buf` is too
/// short.
pub(crate) fn write_decimal(limbs: &[u32], negative: bool, buf: &mut [u8]) -> Result<usize> {
    let chunks = decimal_chunks(limbs);
    let digits = match chunks.len() {
        0 => 1,
        k => (k - 1) * 9 + digit_count(chunks[k - 1]),
    };
    let total = digits + usize::from(negative && !limbs.is_empty());
    if buf.len() < total {
        return Err(Error::BufferTooSmall);
    }
    let mut pos = total;
    match chunks.len() {
        0 => {
            pos -= 1;
            buf[pos] = b'0';
        }
        _ => {
            for (i, &c) in chunks.iter().enumerate() {
                let last = i + 1 == chunks.len();
                let width = if last { digit_count(c) } else { 9 };
                pos -= width;
                write_chunk(c, &mut buf[pos..pos + width]);
            }
        }
    }
    if negative && !limbs.is_empty() {
        buf[0] = b'-';
    }
    Ok(total)
}

/// Write exactly `dst.len()` digits of `v` (1..=9), zero padded on the left.
fn write_chunk(mut v: u32, dst: &mut [u8]) {
    for slot in dst.iter_mut().rev() {
        *slot = b'0' + (v % 10) as u8;
        v /= 10;
    }
    debug_assert_eq!(v, 0);
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
}
