//! Mathematical integers with unbounded exact semantics.
//!
//! An `Int` is a sign-magnitude value: a negative flag plus a base 2^32
//! `Vec<u32>` magnitude, normalized (no leading zero limbs). Zero is
//! non-negative with the empty magnitude, so exactly one representation of
//! zero exists. Division truncates toward zero and the remainder takes the
//! sign of the dividend, matching `i64` `/` and `%` on the underlying values.

use std::cmp::Ordering;

use crate::error::{Error, Result};
use crate::limbs;
use crate::nat::{magnitude_to_u64, parse_digits, write_decimal};

/// An unbounded integer.
#[derive(Clone, Debug)]
pub struct Int {
    negative: bool,
    limbs: Vec<u32>,
}

impl Int {
    /// The integer zero.
    pub fn zero() -> Int {
        Int {
            negative: false,
            limbs: Vec::new(),
        }
    }

    /// Whether the value is zero.
    pub fn is_zero(&self) -> bool {
        limbs::is_zero(&self.limbs)
    }

    /// Whether the value is strictly negative.
    pub fn is_negative(&self) -> bool {
        self.negative
    }

    /// Convert from an `i64`.
    pub fn from_i64(v: i64) -> Int {
        let (negative, magnitude) = if v < 0 {
            (true, (v as i128).unsigned_abs())
        } else {
            (false, v as u64 as u128)
        };
        Int {
            negative: negative && magnitude != 0,
            limbs: nat_limbs_from_u128(magnitude),
        }
    }

    /// Convert to an `i64`, faulting when the value does not fit.
    pub fn to_i64(&self) -> Result<i64> {
        let u = magnitude_to_u64(&self.limbs)?;
        if self.negative {
            // i64::MIN has magnitude 2^63 and is the only value whose
            // magnitude exceeds i64::MAX.
            if u > (i64::MAX as u64) + 1 {
                return Err(Error::OutOfRange);
            }
            if u == (i64::MAX as u64) + 1 {
                return Ok(i64::MIN);
            }
            Ok(-(u as i64))
        } else {
            if u > i64::MAX as u64 {
                return Err(Error::OutOfRange);
            }
            Ok(u as i64)
        }
    }

    /// Whether the value fits in an `i64`.
    pub fn fits_i64(&self) -> bool {
        self.to_i64().is_ok()
    }

    /// `self + other`.
    pub fn add(&self, other: &Int) -> Int {
        if self.negative == other.negative {
            Int {
                negative: self.negative,
                limbs: limbs::add_limbs(&self.limbs, &other.limbs),
            }
        } else {
            match limbs::cmp_limbs(&self.limbs, &other.limbs) {
                Ordering::Equal => Int::zero(),
                Ordering::Greater => Int {
                    negative: self.negative,
                    limbs: limbs::sub_limbs(&self.limbs, &other.limbs),
                },
                Ordering::Less => Int {
                    negative: other.negative,
                    limbs: limbs::sub_limbs(&other.limbs, &self.limbs),
                },
            }
        }
    }

    /// `self - other`.
    pub fn sub(&self, other: &Int) -> Int {
        self.add(&other.neg())
    }

    /// `self * other`.
    pub fn mul(&self, other: &Int) -> Int {
        let limbs = limbs::mul_limbs(&self.limbs, &other.limbs);
        Int {
            negative: (self.negative != other.negative) && !limbs.is_empty(),
            limbs,
        }
    }

    /// `(self / other, self % other)`.
    ///
    /// Division truncates toward zero. The remainder satisfies
    /// `self == quotient * other + remainder` and takes the sign of `self`
    /// (so `-7 / 3` is `(-2, -1)` and `7 / -3` is `(-2, 1)`). Faults on
    /// division by zero.
    pub fn div_rem(&self, other: &Int) -> Result<(Int, Int)> {
        if other.is_zero() {
            return Err(Error::DivByZero);
        }
        let (q, r) = limbs::div_rem_limbs(&self.limbs, &other.limbs);
        let q = Int {
            negative: (self.negative != other.negative) && !q.is_empty(),
            limbs: q,
        };
        let r = Int {
            negative: self.negative && !r.is_empty(),
            limbs: r,
        };
        Ok((q, r))
    }

    /// `self / other` with division truncated toward zero.
    pub fn div(&self, other: &Int) -> Result<Int> {
        Ok(self.div_rem(other)?.0)
    }

    /// `self % other`, with the sign of the dividend.
    pub fn rem(&self, other: &Int) -> Result<Int> {
        Ok(self.div_rem(other)?.1)
    }

    /// `-self`; negation of zero is zero.
    pub fn neg(&self) -> Int {
        Int {
            negative: !self.negative && !self.is_zero(),
            limbs: self.limbs.clone(),
        }
    }

    /// Three-way comparison.
    pub fn cmp(&self, other: &Int) -> Ordering {
        // Invariant: a negative value is never zero, so a value that is not
        // negative is greater than any negative one.
        match (self.negative, other.negative) {
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
            (false, false) => limbs::cmp_limbs(&self.limbs, &other.limbs),
            (true, true) => limbs::cmp_limbs(&other.limbs, &self.limbs),
        }
    }

    /// Parse ASCII decimal text into an integer.
    ///
    /// Well-formed input is an optional `+` or `-` sign followed by one or
    /// more ASCII digits (leading zeros are allowed). `-0` denotes zero.
    /// Empty input, bare signs, whitespace, and any other byte are malformed
    /// and fault.
    pub fn from_decimal(bytes: &[u8]) -> Result<Int> {
        let (negative, digits) = match bytes.split_first() {
            Some((b'-', rest)) => (true, rest),
            Some((b'+', rest)) => (false, rest),
            _ => (false, bytes),
        };
        let limbs = parse_digits(digits).ok_or(Error::MalformedDecimal)?;
        Ok(Int {
            negative: negative && !limbs.is_empty(),
            limbs,
        })
    }

    /// The number of ASCII bytes `to_decimal` would write (at least 1).
    pub fn decimal_len(&self) -> usize {
        crate::nat::decimal_len(&self.limbs)
            + usize::from(self.negative && !self.limbs.is_empty())
    }

    /// Write the decimal representation into `buf` and return the length
    /// written, including any sign.
    ///
    /// Faults with [`Error::BufferTooSmall`] when `buf` is shorter than
    /// [`Int::decimal_len`] bytes; the buffer is then left untouched.
    pub fn to_decimal(&self, buf: &mut [u8]) -> Result<usize> {
        write_decimal(&self.limbs, self.negative, buf)
    }
}

impl PartialEq for Int {
    fn eq(&self, other: &Int) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Int {}

impl PartialOrd for Int {
    fn partial_cmp(&self, other: &Int) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Int {
    fn cmp(&self, other: &Int) -> Ordering {
        Int::cmp(self, other)
    }
}

/// Split a `u128` magnitude into normalized base 2^32 limbs.
fn nat_limbs_from_u128(mut v: u128) -> Vec<u32> {
    let mut limbs = Vec::new();
    while v > 0 {
        limbs.push(v as u32);
        v >>= 32;
    }
    limbs::normalize(&mut limbs);
    limbs
}

#[cfg(test)]
mod tests {
    use super::*;

    fn int(v: i64) -> Int {
        Int::from_i64(v)
    }

    #[test]
    fn from_i64_covers_the_full_range() {
        assert_eq!(int(i64::MIN).to_i64(), Ok(i64::MIN));
        assert_eq!(int(i64::MAX).to_i64(), Ok(i64::MAX));
        assert_eq!(int(0), Int::zero());
        assert_eq!(int(-0), Int::zero());
        assert!(int(-1).is_negative());
        assert!(!int(0).is_negative());
    }

    #[test]
    fn to_i64_range_boundaries() {
        // Magnitude 2^63: negative is i64::MIN, positive is out of range.
        let two_63 = Int::from_i64(i64::MIN).neg();
        assert_eq!(two_63.to_i64(), Err(Error::OutOfRange));
        assert!(!two_63.fits_i64());
        assert!(Int::from_i64(i64::MIN).fits_i64());
        // 2^63 + 1 does not fit in either direction.
        let over = two_63.add(&int(1));
        assert_eq!(over.to_i64(), Err(Error::OutOfRange));
        assert_eq!(over.neg().to_i64(), Err(Error::OutOfRange));
    }

    #[test]
    fn add_sub_cross_sign() {
        assert_eq!(int(7).add(&int(-3)), int(4));
        assert_eq!(int(-7).add(&int(3)), int(-4));
        assert_eq!(int(7).add(&int(-7)), Int::zero());
        assert_eq!(int(-7).add(&int(-3)), int(-10));
        assert_eq!(int(3).sub(&int(7)), int(-4));
        assert_eq!(int(-3).sub(&int(-7)), int(4));
        // Borrow across limb boundaries.
        let big = Int::from_i64(i64::MIN); // -2^63
        assert_eq!(big.sub(&int(1)).add(&int(1)), big);
    }

    #[test]
    fn mul_signs_and_zero() {
        assert_eq!(int(6).mul(&int(-7)), int(-42));
        assert_eq!(int(-6).mul(&int(7)), int(-42));
        assert_eq!(int(-6).mul(&int(-7)), int(42));
        assert_eq!(Int::zero().mul(&int(-7)), Int::zero());
        assert_eq!(int(-7).mul(&Int::zero()), Int::zero());
    }

    #[test]
    fn div_truncates_toward_zero_and_rem_matches_dividend_sign() {
        // Rust i64 semantics on the underlying values.
        for a in [-7i64, -6, -5, -3, -1, 0, 1, 3, 5, 6, 7] {
            for b in [-3i64, -2, -1, 1, 2, 3] {
                let (q, r) = int(a).div_rem(&int(b)).unwrap();
                assert_eq!(q, int(a / b), "{a} / {b}");
                assert_eq!(r, int(a % b), "{a} % {b}");
                assert_eq!(q.mul(&int(b)).add(&r), int(a), "identity for {a} / {b}");
            }
        }
    }

    #[test]
    fn div_by_zero_is_the_division_fault() {
        assert_eq!(int(5).div_rem(&Int::zero()), Err(Error::DivByZero));
        assert_eq!(int(5).div(&Int::zero()), Err(Error::DivByZero));
        assert_eq!(int(5).rem(&Int::zero()), Err(Error::DivByZero));
    }

    #[test]
    fn neg_is_an_involution_and_zero_is_fixed() {
        for v in [i64::MIN, -1000, -1, 0, 1, 1000, i64::MAX] {
            let x = int(v);
            assert_eq!(x.neg().neg(), x);
            // Negation actually flips the sign and cancels in sums.
            if v != 0 {
                assert_ne!(x.neg(), x);
                assert_eq!(x.neg().is_negative(), v > 0);
            }
            assert_eq!(x.add(&x.neg()), Int::zero());
        }
        assert_eq!(Int::zero().neg(), Int::zero());
        assert!(!Int::zero().neg().is_negative());
    }

    #[test]
    fn cmp_orders_across_signs() {
        assert_eq!(int(-1).cmp(&int(0)), Ordering::Less);
        assert_eq!(int(0).cmp(&int(1)), Ordering::Less);
        assert_eq!(int(-5).cmp(&int(-3)), Ordering::Less);
        assert_eq!(int(-3).cmp(&int(-5)), Ordering::Greater);
        assert_eq!(int(-5).cmp(&int(-5)), Ordering::Equal);
        assert_eq!(int(3).cmp(&int(5)), Ordering::Less);
        assert!(int(-7) < int(0));
        assert!(int(0) < int(7));
    }

    #[test]
    fn decimal_parse_format_round_trip_with_signs() {
        let cases: &[(&str, &str)] = &[
            ("0", "0"),
            ("-0", "0"),
            ("+0", "0"),
            ("-1", "-1"),
            ("+42", "42"),
            ("00042", "42"),
            ("-00042", "-42"),
            ("123456789012345678901234567890", "123456789012345678901234567890"),
        ];
        for &(input, want) in cases {
            let v = Int::from_decimal(input.as_bytes()).unwrap();
            let mut buf = [0u8; 64];
            let n = v.to_decimal(&mut buf).unwrap();
            assert_eq!(&buf[..n], want.as_bytes(), "input {input:?}");
            assert_eq!(v.decimal_len(), want.len(), "input {input:?}");
        }
    }

    #[test]
    fn malformed_decimal_inputs_fault() {
        for s in ["", "+", "-", " 1", "1 ", "12a", "1-2", "1+2", "--1", "\u{6771}"] {
            assert_eq!(
                Int::from_decimal(s.as_bytes()),
                Err(Error::MalformedDecimal),
                "{s:?}"
            );
        }
    }

    #[test]
    fn to_decimal_faults_on_small_buffer() {
        let v = Int::from_decimal(b"-1234567890123456789012").unwrap();
        let need = v.decimal_len();
        assert_eq!(v.to_decimal(&mut vec![0u8; need]), Ok(need));
        assert_eq!(
            v.to_decimal(&mut vec![0u8; need - 1]),
            Err(Error::BufferTooSmall)
        );
    }

    #[test]
    fn decimal_len_includes_the_sign() {
        let v = Int::from_decimal(b"-123456789012345678901").unwrap();
        assert_eq!(v.decimal_len(), 22);
        assert_eq!(Int::zero().decimal_len(), 1);
        assert_eq!(Int::from_i64(-1).decimal_len(), 2);
    }
}
