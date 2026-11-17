//! Internal magnitude arithmetic on base 2^32 limb vectors.
//!
//! A magnitude is a `Vec<u32>` in little-endian limb order (limb 0 is the
//! least significant) and is normalized: no leading zero limbs, so the
//! magnitude of zero is the empty vector. All functions in this module
//! maintain normalization and take preconditions where noted; they are the
//! single implementation shared by `Nat` and `Int`.

use std::cmp::Ordering;

/// Strip leading zero limbs so the vector is normalized.
pub(crate) fn normalize(v: &mut Vec<u32>) {
    while v.last() == Some(&0) {
        v.pop();
    }
}

/// Whether the magnitude is zero (empty after normalization).
pub(crate) fn is_zero(v: &[u32]) -> bool {
    v.is_empty()
}

/// Compare two normalized magnitudes.
pub(crate) fn cmp_limbs(a: &[u32], b: &[u32]) -> Ordering {
    if a.len() != b.len() {
        return a.len().cmp(&b.len());
    }
    for (x, y) in a.iter().rev().zip(b.iter().rev()) {
        match x.cmp(y) {
            Ordering::Equal => continue,
            other => return other,
        }
    }
    Ordering::Equal
}

/// `a + b` for normalized magnitudes.
pub(crate) fn add_limbs(a: &[u32], b: &[u32]) -> Vec<u32> {
    let (small, large) = if a.len() < b.len() { (a, b) } else { (b, a) };
    let mut out = Vec::with_capacity(large.len() + 1);
    let mut carry = 0u64;
    for i in 0..large.len() {
        let mut s = carry + large[i] as u64;
        if i < small.len() {
            s += small[i] as u64;
        }
        out.push(s as u32);
        carry = s >> 32;
    }
    if carry > 0 {
        out.push(carry as u32);
    }
    out
}

/// `a - b` for normalized magnitudes.
///
/// Precondition: `cmp_limbs(a, b) != Ordering::Less`, otherwise the result is
/// a wrapped value.
pub(crate) fn sub_limbs(a: &[u32], b: &[u32]) -> Vec<u32> {
    debug_assert!(cmp_limbs(a, b) != Ordering::Less);
    let mut out = Vec::with_capacity(a.len());
    let mut borrow = 0i64;
    for i in 0..a.len() {
        let mut d = a[i] as i64 - borrow;
        if i < b.len() {
            d -= b[i] as i64;
        }
        if d < 0 {
            d += 1i64 << 32;
            borrow = 1;
        } else {
            borrow = 0;
        }
        out.push(d as u32);
    }
    debug_assert_eq!(borrow, 0);
    normalize(&mut out);
    out
}

/// `a * b` for normalized magnitudes (schoolbook).
pub(crate) fn mul_limbs(a: &[u32], b: &[u32]) -> Vec<u32> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = vec![0u32; a.len() + b.len()];
    for i in 0..a.len() {
        let ai = a[i] as u64;
        if ai == 0 {
            continue;
        }
        let mut carry = 0u64;
        for j in 0..b.len() {
            // (2^32-1)^2 + (2^32-1) + (2^32-1) = 2^64-1, so the
            // accumulator never overflows u64.
            let t = ai * (b[j] as u64) + (out[i + j] as u64) + carry;
            out[i + j] = t as u32;
            carry = t >> 32;
        }
        let mut k = i + b.len();
        while carry > 0 {
            debug_assert!(k < out.len());
            let t = out[k] as u64 + carry;
            out[k] = t as u32;
            carry = t >> 32;
            k += 1;
        }
    }
    normalize(&mut out);
    out
}

/// `(u / d, u % d)` for a single-limb divisor, quotient normalized.
pub(crate) fn div_rem_by_word(u: &[u32], d: u32) -> (Vec<u32>, u32) {
    debug_assert!(d != 0);
    let mut q = u.to_vec();
    let r = div_rem_by_word_in_place(&mut q, d);
    (q, r)
}

/// Divide `v` in place by a single-limb divisor, returning the remainder.
pub(crate) fn div_rem_by_word_in_place(v: &mut Vec<u32>, d: u32) -> u32 {
    debug_assert!(d != 0);
    let mut rem: u64 = 0;
    for limb in v.iter_mut().rev() {
        let cur = (rem << 32) | (*limb as u64);
        *limb = (cur / d as u64) as u32;
        rem = cur % d as u64;
    }
    normalize(v);
    rem as u32
}

/// `v * m + add` in place; `m` and `add` are single limbs.
pub(crate) fn mul_add_small(v: &mut Vec<u32>, m: u32, add: u32) {
    let mut carry = add as u64;
    for limb in v.iter_mut() {
        let t = (*limb as u64) * (m as u64) + carry;
        *limb = t as u32;
        carry = t >> 32;
    }
    while carry > 0 {
        v.push(carry as u32);
        carry >>= 32;
    }
    normalize(v);
}

/// `(u / v, u % v)` for normalized magnitudes with non-empty `v`.
pub(crate) fn div_rem_limbs(u: &[u32], v: &[u32]) -> (Vec<u32>, Vec<u32>) {
    debug_assert!(!v.is_empty());
    if cmp_limbs(u, v) == Ordering::Less {
        return (Vec::new(), u.to_vec());
    }
    let n = v.len();
    if n == 1 {
        let (q, r) = div_rem_by_word(u, v[0]);
        return (q, if r == 0 { Vec::new() } else { vec![r] });
    }

    // Knuth (TAOCP vol 2, sec 4.3.1) algorithm D on base 2^32 limbs.
    let m = u.len();
    let b: u64 = 1 << 32;
    let s = v[n - 1].leading_zeros();

    // vn = v << s (exactly n limbs), un = u << s (m+1 limbs).
    let mut vn = vec![0u32; n];
    for i in (1..n).rev() {
        vn[i] = (((v[i] as u64) << s) | ((v[i - 1] as u64) >> (32 - s))) as u32;
    }
    vn[0] = ((v[0] as u64) << s) as u32;
    let mut un = vec![0u32; m + 1];
    for i in (1..m).rev() {
        un[i] = (((u[i] as u64) << s) | ((u[i - 1] as u64) >> (32 - s))) as u32;
    }
    un[0] = ((u[0] as u64) << s) as u32;
    un[m] = ((u[m - 1] as u64) >> (32 - s)) as u32;
    debug_assert!(vn[n - 1] >= (1 << 31));

    let mut q = vec![0u32; m - n + 1];
    for j in (0..=(m - n)).rev() {
        let top = ((un[j + n] as u64) << 32) | (un[j + n - 1] as u64);
        // Estimate the quotient digit (D4). When the leading remainder limb
        // equals the leading divisor limb the naive quotient would be >= b,
        // so it is capped at b-1 and the remainder estimate is inflated;
        // an inflated estimate is >= b and short-circuits the D5 test below
        // (qhat * (vn[n-2]) <= (b-1)^2 < b^2 <= rhat * b + ...).
        let (mut qhat, mut rhat) = if un[j + n] == vn[n - 1] {
            (
                b - 1,
                (un[j + n] as u64) + (un[j + n - 1] as u64),
            )
        } else {
            (top / (vn[n - 1] as u64), top % (vn[n - 1] as u64))
        };
        debug_assert!(qhat < b);
        // D5: tighten the estimate against the next limb down. The loop can
        // only run when rhat < b, so the products below fit in u64.
        while rhat < b && qhat * (vn[n - 2] as u64) > (rhat << 32) + (un[j + n - 2] as u64)
        {
            qhat -= 1;
            rhat += vn[n - 1] as u64;
            if rhat >= b {
                break;
            }
        }

        // Multiply and subtract qhat * vn from un[j..=j+n].
        let mut k: i64 = 0;
        for i in 0..n {
            let p = qhat * (vn[i] as u64);
            let t = (un[i + j] as i64) - k - ((p & 0xFFFF_FFFF) as i64);
            un[i + j] = t as u32;
            k = ((p >> 32) as i64) - (t >> 32);
        }
        let t = (un[j + n] as i64) - k;
        un[j + n] = t as u32;

        let mut qj = qhat as u32;
        if t < 0 {
            // The estimate was one too large: give one divisor back.
            debug_assert!(qj > 0);
            qj -= 1;
            let mut c: u64 = 0;
            for i in 0..n {
                let t2 = (un[i + j] as u64) + (vn[i] as u64) + c;
                un[i + j] = t2 as u32;
                c = t2 >> 32;
            }
            un[j + n] = ((un[j + n] as u64) + c) as u32;
        }
        q[j] = qj;
    }
    debug_assert_eq!(un[n], 0, "remainder must fit in n limbs");

    // Unnormalize the remainder: un[0..n] >> s.
    let mut r = vec![0u32; n];
    for i in 0..n - 1 {
        r[i] = (((un[i] as u64) >> s) | ((un[i + 1] as u64) << (32 - s))) as u32;
    }
    r[n - 1] = ((un[n - 1] as u64) >> s) as u32;
    normalize(&mut q);
    normalize(&mut r);
    (q, r)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn n(v: u64) -> Vec<u32> {
        let mut out = Vec::new();
        let mut v = v;
        while v > 0 {
            out.push(v as u32);
            v >>= 32;
        }
        normalize(&mut out);
        out
    }

    fn to_u64(v: &[u32]) -> u64 {
        let mut acc: u64 = 0;
        for (i, &l) in v.iter().enumerate() {
            assert!(i < 2);
            acc |= (l as u64) << (32 * i);
        }
        acc
    }

    #[test]
    fn zero_is_the_empty_magnitude() {
        assert!(is_zero(&normalize_zero()));
        assert_eq!(normalize_zero(), Vec::new());
        assert_eq!(cmp_limbs(&[], &[]), Ordering::Equal);
        assert_eq!(add_limbs(&[], &[]), Vec::new());
        assert_eq!(mul_limbs(&[], &n(7)), Vec::new());
    }

    fn normalize_zero() -> Vec<u32> {
        let mut v = vec![0, 0, 0];
        normalize(&mut v);
        v
    }

    #[test]
    fn normalize_strips_only_leading_zeros() {
        let mut v = vec![5, 0, 0, 0];
        normalize(&mut v);
        assert_eq!(v, vec![5]);
    }

    #[test]
    fn add_carries_across_the_limb_boundary() {
        // (2^32 - 1) + 1 = 2^32.
        assert_eq!(add_limbs(&n(0xFFFF_FFFF), &n(1)), n(1 << 32));
        // 2^32 + (2^32 - 1) = 2^33 - 1.
        assert_eq!(add_limbs(&n(1 << 32), &n(0xFFFF_FFFF)), n((1 << 33) - 1));
        // Chain of carries across three limbs.
        let a = vec![0xFFFF_FFFF; 3];
        let b = vec![1];
        assert_eq!(add_limbs(&a, &b), vec![0, 0, 0, 1]);
    }

    #[test]
    fn sub_borrows_across_the_limb_boundary() {
        // 2^32 - 1 = (2^32) - 1.
        assert_eq!(sub_limbs(&n(1 << 32), &n(1)), n(0xFFFF_FFFF));
        // 2^64 - 1 minus 1 needs a borrow chain across two limbs.
        assert_eq!(sub_limbs(&n(u64::MAX), &n(1)), n(u64::MAX - 1));
        // Exact cancellation yields zero.
        assert_eq!(sub_limbs(&n(1 << 32), &n(1 << 32)), Vec::new());
    }

    #[test]
    fn cmp_orders_by_length_then_most_significant_limb() {
        assert_eq!(cmp_limbs(&n(1 << 32), &n(0xFFFF_FFFF)), Ordering::Greater);
        assert_eq!(cmp_limbs(&n(0xFFFF_FFFF), &n(1 << 32)), Ordering::Less);
        assert_eq!(cmp_limbs(&n(1 << 32), &n(1 << 32)), Ordering::Equal);
        assert_eq!(cmp_limbs(&[], &n(1)), Ordering::Less);
    }

    #[test]
    fn mul_handles_limb_boundaries_and_zero() {
        // (2^32-1)^2 = 2^64 - 2^33 + 1.
        assert_eq!(mul_limbs(&n(0xFFFF_FFFF), &n(0xFFFF_FFFF)), n128(
            (1u128 << 64) - (1u128 << 33) + 1
        ));
        assert_eq!(mul_limbs(&n(1 << 32), &n(1 << 32)), vec![0, 0, 1]);
        assert_eq!(mul_limbs(&[], &n(u64::MAX)), Vec::new());
        // Cross-check a grid of products against u64 arithmetic.
        for a in [0u64, 1, 2, 0xFFFF_FFFF, 1 << 32, u64::MAX] {
            for b in [0u64, 1, 3, 0xFFFF_FFFF, 1 << 32, u64::MAX] {
                let got = mul_limbs(&n(a), &n(b));
                let want = (a as u128) * (b as u128);
                let mut acc = 0u128;
                for (i, &l) in got.iter().enumerate() {
                    acc |= (l as u128) << (32 * i);
                }
                assert_eq!(acc, want, "{a} * {b}");
            }
        }
    }

    #[test]
    fn div_rem_by_word_matches_u64_reference() {
        for a in [0u64, 1, 999, 0xFFFF_FFFF, 1 << 32, u64::MAX] {
            for d in [1u32, 2, 10, 0xFFFF_FFFF] {
                let (q, r) = div_rem_by_word(&n(a), d);
                assert_eq!(to_u64(&q), a / d as u64, "{a} / {d}");
                assert_eq!(r, (a % d as u64) as u32, "{a} % {d}");
            }
        }
    }

    #[test]
    fn div_rem_limbs_small_grid_matches_u64_reference() {
        for a in [0u64, 1, 2, 3, 0xFFFF_FFFF, 1 << 32, (1 << 32) + 1, u64::MAX] {
            for b in [1u64, 2, 3, 0xFFFF_FFFF, 1 << 32, (1 << 32) + 1, u64::MAX] {
                let (q, r) = div_rem_limbs(&n(a), &n(b));
                assert_eq!(to_u64(&q), a / b, "{a} / {b}");
                assert_eq!(to_u64(&r), a % b, "{a} % {b}");
            }
        }
    }

    #[test]
    fn div_rem_limbs_multi_limb_identity() {
        // Values that force multi-limb divisors and nontrivial quotient digits.
        let cases: &[(u128, u128)] = &[
            ((1u128 << 96) - 1, (1u128 << 32) + 1),
            ((1u128 << 96) - 1, (1u128 << 64) - 1),
            ((1u128 << 64) + 12345, (1u128 << 32) + 7),
            (u128::MAX, (1u128 << 65) + 3),
            ((1u128 << 33) + 5, (1u128 << 32) + 3),
        ];
        for &(a, b) in cases {
            let av = n128(a);
            let bv = n128(b);
            let (q, r) = div_rem_limbs(&av, &bv);
            assert_eq!(add_limbs(&mul_limbs(&q, &bv), &r), av, "q*b+r == a for {a} / {b}");
            assert!(cmp_limbs(&r, &bv) == Ordering::Less, "r < b for {a} / {b}");
        }
    }

    #[test]
    fn div_rem_limbs_crafted_estimate_cap_vector() {
        // v = 2^63 + 1 has normalized leading limb 2^31. For
        // u = (2^33 - 1) * v + 2^63 = 2^96 + 2^33 - 1 the running remainder
        // after the first quotient digit is exactly 2^63, whose leading
        // limb equals v's leading limb: the capped estimate branch
        // (qhat = b - 1) is the only one that can produce the right digit.
        let v = vec![0x1u32, 0x8000_0000];
        let u = vec![0xFFFF_FFFF, 0x1, 0x0, 0x1];
        let (q, r) = div_rem_limbs(&u, &v);
        assert_eq!(q, vec![0xFFFF_FFFF, 0x1], "quotient 2^33 - 1");
        assert_eq!(r, vec![0x0, 0x8000_0000], "remainder 2^63");
        assert_eq!(add_limbs(&mul_limbs(&q, &v), &r), u);
        assert_eq!(cmp_limbs(&r, &v), Ordering::Less);
    }

    #[test]
    fn div_rem_limbs_crafted_add_back_vector() {
        // v = 2^95 + 2^32 - 1: leading limb 2^31 with a zero second limb,
        // which makes the D5 estimate test blind to the true difference.
        // u = 2v - 1 = 2^96 + 2^33 - 3 then estimates one quotient digit
        // too high and must be corrected by the multiply-subtract add-back.
        let v = vec![0xFFFF_FFFF, 0x0, 0x8000_0000];
        let u = vec![0xFFFF_FFFD, 0x1, 0x0, 0x1];
        let (q, r) = div_rem_limbs(&u, &v);
        assert_eq!(q, vec![1], "quotient 1");
        assert_eq!(r, vec![0xFFFF_FFFE, 0x0, 0x8000_0000], "remainder v - 1");
        assert_eq!(add_limbs(&mul_limbs(&q, &v), &r), u);
        assert_eq!(cmp_limbs(&r, &v), Ordering::Less);
    }

    #[test]
    fn div_rem_limbs_matches_an_independent_binary_division() {
        // Reference model: bit-by-bit restoring division, far simpler than
        // Knuth D and independent of it. Fuzzed limb counts hit every
        // branch of the quotient digit estimation.
        let mut state: u64 = 0x243F_6A88_85A3_08D3;
        let mut next = move || {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            state
        };
        for _ in 0..3000 {
            let n = 1 + (next() % 5) as usize;
            let mut v: Vec<u32> = Vec::new();
            for i in 0..n {
                let mut limb = next() as u32;
                // Half the time use adversarial limb values.
                if next() % 2 == 0 {
                    limb = match i {
                        _ if i + 1 == n => 1 << 31, // minimal normalized lead
                        _ => 0xFFFF_FFFF,
                    };
                }
                v.push(limb);
            }
            normalize(&mut v);
            if v.is_empty() {
                continue;
            }
            let m = n + (next() % 3) as usize;
            let mut u: Vec<u32> = Vec::new();
            for _ in 0..m {
                let mut limb = next() as u32;
                if next() % 2 == 0 {
                    limb = 0xFFFF_FFFF;
                }
                u.push(limb);
            }
            normalize(&mut u);
            if u.is_empty() {
                continue;
            }
            let (q, r) = div_rem_limbs(&u, &v);
            let (eq, er) = binary_div_rem(&u, &v);
            assert_eq!(q, eq, "quotient for {:x?} / {:x?}", u, v);
            assert_eq!(r, er, "remainder for {:x?} / {:x?}", u, v);
        }
    }

    /// Independent reference: restoring binary long division.
    fn binary_div_rem(u: &[u32], v: &[u32]) -> (Vec<u32>, Vec<u32>) {
        let mut rem: Vec<u32> = Vec::new();
        let mut qbits: Vec<bool> = Vec::new();
        for i in (0..u.len()).rev() {
            for bit in (0..32).rev() {
                shl1_or(&mut rem, (u[i] >> bit) & 1);
                if cmp_limbs(&rem, v) == Ordering::Less {
                    qbits.push(false);
                } else {
                    sub_in_place(&mut rem, v);
                    qbits.push(true);
                }
            }
        }
        let mut q = vec![0u32; (qbits.len() + 31) / 32];
        for (k, &b) in qbits.iter().rev().enumerate() {
            if b {
                q[k / 32] |= 1 << (k % 32);
            }
        }
        normalize(&mut q);
        (q, rem)
    }

    fn shl1_or(v: &mut Vec<u32>, bit: u32) {
        let mut carry = bit;
        for limb in v.iter_mut() {
            let next_carry = *limb >> 31;
            *limb = (*limb << 1) | carry;
            carry = next_carry;
        }
        if carry != 0 {
            v.push(1);
        }
    }

    fn sub_in_place(a: &mut Vec<u32>, b: &[u32]) {
        let mut borrow = 0i64;
        for i in 0..a.len() {
            let mut d = a[i] as i64 - borrow;
            if i < b.len() {
                d -= b[i] as i64;
            }
            if d < 0 {
                d += 1i64 << 32;
                borrow = 1;
            } else {
                borrow = 0;
            }
            a[i] = d as u32;
        }
        debug_assert_eq!(borrow, 0);
        normalize(a);
    }

    fn n128(v: u128) -> Vec<u32> {
        let mut out = Vec::new();
        let mut v = v;
        while v > 0 {
            out.push(v as u32);
            v >>= 32;
        }
        normalize(&mut out);
        out
    }

    #[test]
    fn mul_add_small_matches_reference() {
        let mut v = n(0xFFFF_FFFF);
        mul_add_small(&mut v, 10, 7);
        assert_eq!(to_u64(&v), 0xFFFF_FFFF * 10 + 7);
        let mut z = Vec::new();
        mul_add_small(&mut z, 10, 42);
        assert_eq!(z, vec![42]);
    }

    #[test]
    fn div_rem_by_word_in_place_normalizes() {
        let mut v = vec![0, 1]; // 2^32
        let r = div_rem_by_word_in_place(&mut v, 1);
        assert_eq!(r, 0);
        assert_eq!(v, vec![0, 1]);
        let mut w = vec![1, 1]; // 2^32 + 1
        let r = div_rem_by_word_in_place(&mut w, 2);
        assert_eq!(r, 1);
        assert_eq!(w, n(((1u64 << 32) + 1) / 2));
    }
}
