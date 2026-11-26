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

/// Limb threshold below which multiplication uses schoolbook directly.
const KARATSUBA_THRESHOLD: usize = 40;

/// Strip trailing zero limbs from a borrowed slice.
#[inline]
fn trim_zeros(mut s: &[u32]) -> &[u32] {
    while let Some(&0) = s.last() {
        s = &s[..s.len() - 1];
    }
    s
}

/// Add `a * b` into `out` (schoolbook). `out` must have length `>= a.len() + b.len()`.
fn schoolbook_add_mul(out: &mut [u32], a: &[u32], b: &[u32]) {
    let (outer, inner) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    for (i, &ai_u32) in outer.iter().enumerate() {
        let ai = ai_u32 as u64;
        if ai == 0 {
            continue;
        }
        let mut carry = 0u64;
        let dst = &mut out[i..i + inner.len()];
        for (d, &bj) in dst.iter_mut().zip(inner.iter()) {
            // (2^32-1)^2 + (2^32-1) + (2^32-1) = 2^64-1, so the
            // accumulator never overflows u64.
            let t = ai * (bj as u64) + (*d as u64) + carry;
            *d = t as u32;
            carry = t >> 32;
        }
        let mut k = i + inner.len();
        while carry > 0 {
            debug_assert!(k < out.len());
            let t = (out[k] as u64) + carry;
            out[k] = t as u32;
            carry = t >> 32;
            k += 1;
        }
    }
}

/// `dst += src` in place, returning the carry out past `dst`.
#[inline]
fn add_assign_slice(dst: &mut [u32], src: &[u32]) -> u64 {
    let mut carry = 0u64;
    for (d, &s) in dst.iter_mut().zip(src.iter()) {
        let sum = (*d as u64) + (s as u64) + carry;
        *d = sum as u32;
        carry = sum >> 32;
    }
    for d in &mut dst[src.len()..] {
        if carry == 0 {
            break;
        }
        let sum = (*d as u64) + carry;
        *d = sum as u32;
        carry = sum >> 32;
    }
    carry
}

/// `dst -= src` in place, returning the borrow out past `dst` (0 or 1).
#[inline]
fn sub_assign_slice(dst: &mut [u32], src: &[u32]) -> u64 {
    let mut borrow = 0u64;
    for (d, &s) in dst.iter_mut().zip(src.iter()) {
        let (d1, b1) = d.overflowing_sub(s);
        let (d2, b2) = d1.overflowing_sub(borrow as u32);
        *d = d2;
        borrow = (b1 as u64) + (b2 as u64);
    }
    for d in &mut dst[src.len()..] {
        if borrow == 0 {
            break;
        }
        let (d2, b2) = d.overflowing_sub(borrow as u32);
        *d = d2;
        borrow = b2 as u64;
    }
    borrow
}

/// Compute `dst = a + b` into a slice of length `>= a.len().max(b.len()) + 1`,
/// returning the normalized length written.
#[inline]
fn add_into_slice(dst: &mut [u32], a: &[u32], b: &[u32]) -> usize {
    let (small, large) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    let mut carry = 0u64;
    for i in 0..small.len() {
        let sum = (large[i] as u64) + (small[i] as u64) + carry;
        dst[i] = sum as u32;
        carry = sum >> 32;
    }
    for i in small.len()..large.len() {
        let sum = (large[i] as u64) + carry;
        dst[i] = sum as u32;
        carry = sum >> 32;
    }
    let mut len = large.len();
    if carry > 0 {
        dst[len] = carry as u32;
        len += 1;
    }
    while len > 0 && dst[len - 1] == 0 {
        len -= 1;
    }
    len
}

/// Compute `a * b` into a zeroed `out` (`out.len() >= a.len() + b.len()`)
/// using schoolbook below [`KARATSUBA_THRESHOLD`], chunking for strongly
/// unbalanced operands, and additive Karatsuba for balanced large operands.
fn mul_into(out: &mut [u32], a: &[u32], b: &[u32], scratch: &mut [u32]) {
    let a = trim_zeros(a);
    let b = trim_zeros(b);
    if a.is_empty() || b.is_empty() {
        return;
    }
    let (small, large) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    if small.len() < KARATSUBA_THRESHOLD {
        schoolbook_add_mul(out, small, large);
        return;
    }

    // Strongly unbalanced (`large >= 1.5 * small`): split `large` into
    // `small`-sized chunks so every sub-product is balanced.
    if large.len() * 2 >= small.len() * 3 {
        let chunk_size = small.len();
        let mut pos = 0;
        while pos < large.len() {
            let end = (pos + chunk_size).min(large.len());
            let chunk = &large[pos..end];
            let prod_len = small.len() + chunk.len();
            let (prod, rest) = scratch.split_at_mut(prod_len);
            prod.fill(0);
            mul_into(prod, small, chunk, rest);
            let carry = add_assign_slice(&mut out[pos..], prod);
            debug_assert_eq!(carry, 0);
            pos = end;
        }
        return;
    }

    // Balanced additive Karatsuba:
    //   a = a0 + a1 * B^m, b = b0 + b1 * B^m
    //   a * b = z0 + ((a0 + a1)(b0 + b1) - z0 - z2) * B^m + z2 * B^(2m)
    let m = large.len().div_ceil(2);
    debug_assert!(m < small.len());
    let (a0, a1) = small.split_at(m);
    let (b0, b1) = large.split_at(m);

    // z0 = a0 * b0 directly into out[0..2m], z2 = a1 * b1 directly into out[2m..].
    let z2_len = a1.len() + b1.len();
    let (out_lo, out_hi) = out.split_at_mut(2 * m);
    mul_into(out_lo, a0, b0, scratch);
    mul_into(&mut out_hi[..z2_len], a1, b1, scratch);

    // Scratch layout: [sa: m+1][sb: m+1][z1: 2m+2][rec_scratch...]
    let (sa_buf, rest) = scratch.split_at_mut(m + 1);
    let (sb_buf, rest) = rest.split_at_mut(m + 1);
    let (z1_buf, rec_scratch) = rest.split_at_mut(2 * m + 2);

    let sa_len = add_into_slice(sa_buf, a0, a1);
    let sb_len = add_into_slice(sb_buf, b0, b1);
    let z1_cap = sa_len + sb_len;
    let z1 = &mut z1_buf[..z1_cap];
    z1.fill(0);
    mul_into(z1, &sa_buf[..sa_len], &sb_buf[..sb_len], rec_scratch);

    // z1 = (a0 + a1)(b0 + b1) - z0 - z2 >= 0.
    let b0_borrow = sub_assign_slice(z1, trim_zeros(out_lo));
    debug_assert_eq!(b0_borrow, 0);
    let b2_borrow = sub_assign_slice(z1, trim_zeros(&out_hi[..z2_len]));
    debug_assert_eq!(b2_borrow, 0);

    let z1_trimmed = trim_zeros(z1);
    let carry = add_assign_slice(&mut out[m..], z1_trimmed);
    debug_assert_eq!(carry, 0);
}

/// `a * b` for normalized magnitudes (schoolbook below [`KARATSUBA_THRESHOLD`],
/// Karatsuba above).
pub(crate) fn mul_limbs(a: &[u32], b: &[u32]) -> Vec<u32> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = vec![0u32; a.len() + b.len()];
    let (small, large) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    if small.len() < KARATSUBA_THRESHOLD {
        schoolbook_add_mul(&mut out, small, large);
    } else {
        // Monotonic scratch bound: chunking uses at most `2*small + (6*small + 64)`,
        // balanced Karatsuba uses at most `6*large + 64`.
        let scratch_len = if large.len() * 2 >= small.len() * 3 {
            8 * small.len() + 64
        } else {
            6 * large.len() + 64
        };
        let mut scratch = vec![0u32; scratch_len];
        mul_into(&mut out, small, large, &mut scratch);
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
///
/// Uses exact precomputed reciprocal multiplication (Granlund–Montgomery /
/// Möller–Granlund) so every limb computes quotient and remainder via one
/// `u64 × u64 -> u128` high-multiply, shift, and multiply-subtract with zero
/// per-limb hardware divisions.
pub(crate) fn div_rem_by_word_in_place(v: &mut Vec<u32>, d: u32) -> u32 {
    debug_assert!(d != 0);
    if v.is_empty() {
        return 0;
    }
    if d == 1 {
        normalize(v);
        return 0;
    }
    if d.is_power_of_two() {
        let shift = d.trailing_zeros();
        let rshift = 32 - shift;
        let rem = v[0] & (d - 1);
        let len = v.len();
        for i in 0..len - 1 {
            v[i] = (v[i] >> shift) | (v[i + 1] << rshift);
        }
        v[len - 1] >>= shift;
        normalize(v);
        return rem;
    }

    let d64 = d as u64;
    let k = 31 - d.leading_zeros();
    let mut rem: u64 = 0;

    if k <= 30 {
        // 2^k < d < 2^(k+1) <= 2^31:
        // M = ceil(2^(64+k) / d) fits in u64, and for every cur < d * 2^32,
        // cur * (M*d - 2^(64+k)) < d^2 * 2^32 < 2^(34+2k) <= 2^(64+k),
        // so floor((cur * M) / 2^(64+k)) == floor(cur / d) identically.
        let m = (((1u128 << (64 + k)) / (d as u128)) + 1) as u64;
        for limb in v.iter_mut().rev() {
            let cur = (rem << 32) | (*limb as u64);
            let q = ((((cur as u128) * (m as u128)) >> 64) as u64) >> k;
            rem = cur - q * d64;
            *limb = q as u32;
        }
    } else {
        // 2^31 < d < 2^32:
        // M = ceil(2^96 / d) = 2^64 + m with m < 2^64, and for every
        // cur < d * 2^32, cur * (M*d - 2^96) < d^2 * 2^32 < 2^96,
        // so floor((cur + floor((cur * m) / 2^64)) / 2^32) == floor(cur / d).
        let m = ((((1u128 << 96) / (d as u128)) + 1) - (1u128 << 64)) as u64;
        for limb in v.iter_mut().rev() {
            let cur = (rem << 32) | (*limb as u64);
            let hi = (((cur as u128) * (m as u128)) >> 64) as u64;
            let q = (cur + hi) >> 32;
            rem = cur - q * d64;
            *limb = q as u32;
        }
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

    fn div_rem_by_word_ref(u: &[u32], d: u32) -> (Vec<u32>, u32) {
        let mut q = u.to_vec();
        let mut rem: u64 = 0;
        for limb in q.iter_mut().rev() {
            let cur = (rem << 32) | (*limb as u64);
            *limb = (cur / d as u64) as u32;
            rem = cur % d as u64;
        }
        normalize(&mut q);
        (q, rem as u32)
    }

    fn schoolbook_mul_ref(a: &[u32], b: &[u32]) -> Vec<u32> {
        if a.is_empty() || b.is_empty() {
            return Vec::new();
        }
        let mut out = vec![0u32; a.len() + b.len()];
        schoolbook_add_mul(&mut out, a, b);
        normalize(&mut out);
        out
    }

    fn xorshift64(state: &mut u64) -> u64 {
        let mut x = *state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        *state = x;
        x
    }

    #[test]
    fn div_rem_by_word_reciprocal_matches_hardware_div_exhaustively() {
        let mut divisors: Vec<u32> = vec![
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            15,
            16,
            17,
            31,
            32,
            33,
            255,
            256,
            257,
            65_535,
            65_536,
            65_537,
            1_000_000_000,
            (1u32 << 30) - 1,
            1u32 << 30,
            (1u32 << 30) + 1,
            (1u32 << 31) - 2,
            (1u32 << 31) - 1,
            1u32 << 31,
            (1u32 << 31) + 1,
            (1u32 << 31) + 2,
            0xDEAD_BEEF,
            0xFFFF_FFFB,
            0xFFFF_FFFE,
            0xFFFF_FFFF,
        ];
        let mut rng = 0xA5A5_1234_5678_9ABCu64;
        // Add random divisors across every bit width 1..=32.
        for bits in 1..=32u32 {
            for _ in 0..4 {
                let raw = xorshift64(&mut rng) as u32;
                let d = if bits == 32 {
                    raw | 0x8000_0000
                } else {
                    (raw & ((1u32 << bits) - 1)) | (1u32 << (bits - 1))
                };
                if d != 0 {
                    divisors.push(d);
                }
            }
        }

        // Test every limb length 0..=64 with adversarial and random patterns.
        for len in 0..=64usize {
            let mut patterns: Vec<Vec<u32>> = Vec::new();
            patterns.push(vec![0xFFFF_FFFFu32; len]);
            if len > 0 {
                let mut pow2 = vec![0u32; len];
                pow2[len - 1] = 1;
                patterns.push(pow2);
                let mut alt: Vec<u32> = (0..len)
                    .map(|i| if i % 2 == 0 { 0xFFFF_FFFF } else { 0 })
                    .collect();
                alt[len - 1] = 0x8000_0000;
                patterns.push(alt);
                let mut rand_v: Vec<u32> = (0..len).map(|_| xorshift64(&mut rng) as u32).collect();
                normalize(&mut rand_v);
                patterns.push(rand_v);
            }
            for u in &patterns {
                for &d in &divisors {
                    let (got_q, got_r) = div_rem_by_word(u, d);
                    let (want_q, want_r) = div_rem_by_word_ref(u, d);
                    assert_eq!(got_q, want_q, "quotient mismatch len={len} d={d:#x}");
                    assert_eq!(got_r, want_r, "remainder mismatch len={len} d={d:#x}");
                }
            }
        }
    }

    #[test]
    fn karatsuba_mul_matches_schoolbook_on_boundary_and_random_inputs() {
        let mut rng = 0xDEAD_C0DE_CAFE_BABEu64;
        let rand_limbs = |len: usize, rng: &mut u64| -> Vec<u32> {
            let mut v: Vec<u32> = (0..len).map(|_| xorshift64(rng) as u32).collect();
            if let Some(last) = v.last_mut() {
                if *last == 0 {
                    *last = 1;
                }
            }
            v
        };

        // 1. All-0xFFFF_FFFF and sparse powers-of-2^32 cases across balanced & unbalanced shapes.
        let shapes: &[(usize, usize)] = &[
            (31, 31),
            (32, 32),
            (33, 33),
            (47, 48),
            (48, 72),
            (32, 200),
            (200, 32),
            (64, 64),
            (64, 256),
            (100, 100),
            (256, 256),
            (512, 512),
        ];
        for &(la, lb) in shapes {
            // All-ones (maximum carry propagation)
            let a_max = vec![0xFFFF_FFFFu32; la];
            let b_max = vec![0xFFFF_FFFFu32; lb];
            assert_eq!(
                mul_limbs(&a_max, &b_max),
                schoolbook_mul_ref(&a_max, &b_max),
                "all-max {la}x{lb}"
            );

            // Powers of 2^32 (sparse: triggers trim_zeros in sub-slices)
            let mut a_pow = vec![0u32; la];
            a_pow[la - 1] = 1;
            let mut b_pow = vec![0u32; lb];
            b_pow[lb - 1] = 1;
            assert_eq!(
                mul_limbs(&a_pow, &b_pow),
                schoolbook_mul_ref(&a_pow, &b_pow),
                "pow2^32 {la}x{lb}"
            );
            a_pow[0] = 1;
            b_pow[0] = 1;
            assert_eq!(
                mul_limbs(&a_pow, &b_pow),
                schoolbook_mul_ref(&a_pow, &b_pow),
                "2^32k+1 {la}x{lb}"
            );

            // Random full-width inputs
            let a_rnd = rand_limbs(la, &mut rng);
            let b_rnd = rand_limbs(lb, &mut rng);
            assert_eq!(
                mul_limbs(&a_rnd, &b_rnd),
                schoolbook_mul_ref(&a_rnd, &b_rnd),
                "random {la}x{lb}"
            );
        }

        // 2. Sweep sizes around threshold (28..=72) with varied aspect ratios.
        for la in [28usize, 31, 32, 33, 35, 40, 47, 48, 55, 63, 64, 65, 72] {
            for lb in [28usize, 32, 33, 47, 48, 64, 96, 130] {
                let a = rand_limbs(la, &mut rng);
                let b = rand_limbs(lb, &mut rng);
                assert_eq!(
                    mul_limbs(&a, &b),
                    schoolbook_mul_ref(&a, &b),
                    "sweep {la}x{lb}"
                );
            }
        }
    }
}
