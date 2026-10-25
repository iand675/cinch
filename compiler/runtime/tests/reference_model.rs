//! Integration-style tests: the bignum runtime against `i128`/`u128`
//! reference models and the extern "C" word API.
//!
//! All pseudo-random inputs come from a deterministic LCG so failures are
//! reproducible. The reference values are built through chunk arithmetic
//! (independent of the decimal code), and decimal formatting is
//! cross-checked against the standard library's `Display` before it is used
//! as an oracle elsewhere.

use slruntime::{Error, Int, Nat};

// ---------------------------------------------------------------------------
// Deterministic pseudo-randomness
// ---------------------------------------------------------------------------

struct Lcg(u64);

impl Lcg {
    fn new(seed: u64) -> Lcg {
        Lcg(seed)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0
    }

    fn next_i128(&mut self) -> i128 {
        let hi = (self.next_u64() as u128) << 64;
        let lo = self.next_u64() as u128;
        (hi | lo) as i128
    }

    /// Uniform-ish value in `[-bound, bound)`.
    fn next_bounded(&mut self, bound: i128) -> i128 {
        let v = self.next_i128();
        (v % bound + bound) % bound
    }
}

// ---------------------------------------------------------------------------
// Building reference values without the decimal code
// ---------------------------------------------------------------------------

fn nat_from_u128(v: u128) -> Nat {
    let shift = Nat::from_i64(1i64 << 32).unwrap();
    let mut acc = Nat::zero();
    for i in (0..4).rev() {
        let chunk = ((v >> (32 * i)) & 0xFFFF_FFFF) as u64;
        acc = acc.mul(&shift).add(&Nat::from_i64(chunk as i64).unwrap());
    }
    acc
}

fn int_from_i128(v: i128) -> Int {
    let magnitude = v.unsigned_abs();
    let shift = Int::from_i64(1i64 << 32);
    let mut acc = Int::zero();
    for i in (0..4).rev() {
        let chunk = ((magnitude >> (32 * i)) & 0xFFFF_FFFF) as u64;
        acc = acc.mul(&shift).add(&Int::from_i64(chunk as i64));
    }
    if v < 0 {
        acc.neg()
    } else {
        acc
    }
}

/// Read a `Nat` back as `u128` through its decimal form.
fn nat_to_u128(n: &Nat) -> u128 {
    let mut buf = vec![0u8; n.decimal_len()];
    let len = n.to_decimal(&mut buf).unwrap();
    std::str::from_utf8(&buf[..len]).unwrap().parse().unwrap()
}

/// Read an `Int` back as `i128` through its decimal form.
fn int_to_i128(v: &Int) -> i128 {
    let mut buf = vec![0u8; v.decimal_len()];
    let len = v.to_decimal(&mut buf).unwrap();
    std::str::from_utf8(&buf[..len]).unwrap().parse().unwrap()
}

fn nat_to_string(n: &Nat) -> String {
    let mut buf = vec![0u8; n.decimal_len()];
    let len = n.to_decimal(&mut buf).unwrap();
    String::from_utf8(buf[..len].to_vec()).unwrap()
}

// ---------------------------------------------------------------------------
// i128 reference model (values in a range where every result fits i128)
// ---------------------------------------------------------------------------

const SMALL_BOUND: i128 = 1i128 << 62;

#[test]
fn nat_ops_match_the_u128_reference_model() {
    let mut rng = Lcg::new(0x5AFE_1A6E_5A7B_1234);
    for _ in 0..2000 {
        let a = (rng.next_bounded(SMALL_BOUND).unsigned_abs()) as u128;
        let b = (rng.next_bounded(SMALL_BOUND).unsigned_abs()) as u128;
        let (na, nb) = (nat_from_u128(a), nat_from_u128(b));
        assert_eq!(nat_to_u128(&na), a);
        assert_eq!(nat_to_u128(&nb), b);

        assert_eq!(nat_to_u128(&na.add(&nb)), a + b);
        assert_eq!(nat_to_u128(&na.mul(&nb)), a * b);
        match na.sub(&nb) {
            Ok(d) => assert_eq!(nat_to_u128(&d), a - b),
            Err(e) => {
                assert_eq!(e, Error::NegativeResult);
                assert!(a < b, "underflow reported for {a} >= {b}");
            }
        }
        if b == 0 {
            assert_eq!(na.div_rem(&nb), Err(Error::DivByZero));
        } else {
            let (q, r) = na.div_rem(&nb).unwrap();
            assert_eq!(nat_to_u128(&q), a / b);
            assert_eq!(nat_to_u128(&r), a % b);
            assert_eq!(q.mul(&nb).add(&r), na);
            assert!(r < nb);
        }
    }
}

#[test]
fn int_ops_match_the_i128_reference_model() {
    let mut rng = Lcg::new(0x1A7E_6E5A_7B12_3456);
    let mut cases: Vec<i128> = Vec::new();
    for _ in 0..256 {
        cases.push(rng.next_bounded(SMALL_BOUND));
    }
    // Fixed boundary values that stay inside the i128-safe range when
    // combined with another boundary value via + or *.
    for v in [
        i128::from(i64::MIN),
        i128::from(i64::MIN) + 1,
        -1,
        0,
        1,
        i128::from(i64::MAX) - 1,
        i128::from(i64::MAX),
        (1i128 << 32) - 1,
        1i128 << 32,
        (1i128 << 32) + 1,
        (1i128 << 64) - 1,
        1i128 << 64,
        (1i128 << 96) - 1,
        1i128 << 96,
    ] {
        cases.push(v);
        cases.push(-v);
    }

    for &a in &cases {
        let na = int_from_i128(a);
        assert_eq!(int_to_i128(&na), a, "round trip of {a}");
        assert_eq!(na.neg(), int_from_i128(-a), "neg of {a}");
        for &b in &cases {
            let nb = int_from_i128(b);
            if let Some(sum) = a.checked_add(b) {
                assert_eq!(int_to_i128(&na.add(&nb)), sum, "{a} + {b}");
            }
            if let Some(diff) = a.checked_sub(b) {
                assert_eq!(int_to_i128(&na.sub(&nb)), diff, "{a} - {b}");
            }
            if let Some(prod) = a.checked_mul(b) {
                assert_eq!(int_to_i128(&na.mul(&nb)), prod, "{a} * {b}");
            }
            match na.cmp(&nb) {
                std::cmp::Ordering::Less => assert!(a < b, "{a} < {b}"),
                std::cmp::Ordering::Equal => assert_eq!(a, b),
                std::cmp::Ordering::Greater => assert!(a > b, "{a} > {b}"),
            }
            if b == 0 {
                assert_eq!(na.div(&nb), Err(Error::DivByZero));
                assert_eq!(na.rem(&nb), Err(Error::DivByZero));
            } else {
                // Both small enough that i128 division cannot overflow
                // (the i128::MIN / -1 case is excluded by the bound).
                let (q, r) = na.div_rem(&nb).unwrap();
                assert_eq!(int_to_i128(&q), a / b, "{a} / {b}");
                assert_eq!(int_to_i128(&r), a % b, "{a} % {b}");
                assert_eq!(q.mul(&nb).add(&r), na, "identity for {a} / {b}");
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Large values beyond any fixed-width reference
// ---------------------------------------------------------------------------

fn lcg_decimal(rng: &mut Lcg, digits: usize) -> String {
    let mut s = String::new();
    for i in 0..digits {
        let d = (rng.next_u64() % 10) as u8;
        if i == 0 && d == 0 {
            s.push('7'); // no leading zeros in the canonical form
        } else {
            s.push((b'0' + d) as char);
        }
    }
    s
}

#[test]
fn large_decimal_round_trips_and_arithmetic_identities() {
    let mut rng = Lcg::new(0xB16B_00B5_1234_5678);
    for digits in [30usize, 200, 901, 1200] {
        for _ in 0..4 {
            let a_str = lcg_decimal(&mut rng, digits);
            let b_str = lcg_decimal(&mut rng, digits / 3 + 1);
            let a = Nat::from_decimal(a_str.as_bytes()).unwrap();
            let b = Nat::from_decimal(b_str.as_bytes()).unwrap();

            // Exact decimal round trip.
            assert_eq!(nat_to_string(&a), a_str, "{digits} digits");
            assert_eq!(a.decimal_len(), a_str.len());

            // Sign round trip for Int.
            let ai = Int::from_decimal(a_str.as_bytes()).unwrap();
            let mut with_sign = a_str.clone();
            with_sign.insert(0, '-');
            let neg = Int::from_decimal(with_sign.as_bytes()).unwrap();
            let mut buf = vec![0u8; with_sign.len()];
            let n = neg.to_decimal(&mut buf).unwrap();
            assert_eq!(&buf[..n], with_sign.as_bytes());
            assert_eq!(neg.decimal_len(), with_sign.len());
            assert_eq!(neg.neg(), ai);

            // Identities on values no fixed-width type can hold.
            let sum = a.add(&b);
            assert_eq!(sum.sub(&b).unwrap(), a);
            assert_eq!(a.sub(&a).unwrap(), Nat::zero());
            let prod = a.mul(&b);
            let (q, r) = prod.div_rem(&b).unwrap();
            assert_eq!(q, a);
            assert_eq!(r, Nat::zero());
            let (q, r) = a.div_rem(&b).unwrap();
            assert_eq!(q.mul(&b).add(&r), a);
            assert!(r < b);
            assert!(sum > a);
            assert!(prod > a);
        }
    }
}

// ---------------------------------------------------------------------------
// i64 range boundaries
// ---------------------------------------------------------------------------

#[test]
fn to_i64_boundaries_for_both_types() {
    let max = nat_from_u128(i64::MAX as u128);
    assert_eq!(max.to_i64(), Ok(i64::MAX));
    assert!(max.fits_i64());
    let over = max.add(&Nat::from_i64(1).unwrap());
    assert_eq!(over.to_i64(), Err(Error::OutOfRange));
    assert!(!over.fits_i64());

    let min = int_from_i128(i128::from(i64::MIN));
    assert_eq!(min.to_i64(), Ok(i64::MIN));
    assert!(min.fits_i64());
    let under = min.sub(&Int::from_i64(1));
    assert_eq!(under.to_i64(), Err(Error::OutOfRange));
    assert!(!under.fits_i64());

    let max_int = int_from_i128(i128::from(i64::MAX)).add(&Int::from_i64(1));
    assert_eq!(max_int.to_i64(), Err(Error::OutOfRange));
}

// ---------------------------------------------------------------------------
// to_decimal cap boundaries
// ---------------------------------------------------------------------------

#[test]
fn to_decimal_cap_boundary_on_random_values() {
    let mut rng = Lcg::new(0xCA9_01D_5AFE_1234);
    for _ in 0..200 {
        let v = rng.next_bounded(1i128 << 100);
        let n = int_from_i128(v);
        let need = n.decimal_len();
        assert_eq!(n.to_decimal(&mut vec![0u8; need]), Ok(need));
        assert_eq!(
            n.to_decimal(&mut vec![0u8; need - 1]),
            Err(Error::BufferTooSmall)
        );
        let nat = nat_from_u128(v.unsigned_abs());
        let need = nat.decimal_len();
        assert_eq!(nat.to_decimal(&mut vec![0u8; need]), Ok(need));
        assert_eq!(
            nat.to_decimal(&mut vec![0u8; need - 1]),
            Err(Error::BufferTooSmall)
        );
    }
}

// ---------------------------------------------------------------------------
// The extern "C" word API (non-faulting paths only)
// ---------------------------------------------------------------------------

#[test]
fn extern_word_api_arithmetic_and_conversion() {
    unsafe {
        let a = slruntime::abi::sl_int_from_i64(i64::MIN);
        let one = slruntime::abi::sl_int_from_i64(1);
        assert_eq!(slruntime::abi::sl_int_to_i64(a), i64::MIN);
        assert_eq!(
            slruntime::abi::sl_int_to_i64(slruntime::abi::sl_int_add(a, one)),
            i64::MIN + 1
        );
        // i64::MIN - 1 does not fit: the word is produced fine but the
        // conversion reports the range error (as an exit fault at the
        // extern layer, so only the fits predicate is safe to probe).
        let under = slruntime::abi::sl_int_sub(a, one);
        assert_eq!(slruntime::abi::sl_int_fits_i64(under), 0);
        assert_eq!(slruntime::abi::sl_int_cmp(a, under), 1);
    }
}

#[test]
fn extern_word_api_nat_boundaries_and_decimal() {
    unsafe {
        let max = slruntime::abi::sl_nat_from_i64(i64::MAX);
        assert_eq!(slruntime::abi::sl_nat_fits_i64(max), 1);
        let one = slruntime::abi::sl_nat_from_i64(1);
        let over = slruntime::abi::sl_nat_add(max, one);
        assert_eq!(slruntime::abi::sl_nat_fits_i64(over), 0);

        // Decimal round trip through the extern entry points.
        let text = b"340282366920938463463374607431768211455"; // 2^128 - 1
        let w = slruntime::abi::sl_nat_from_decimal(text.as_ptr(), text.len());
        assert_eq!(slruntime::abi::sl_nat_decimal_len(w), text.len());
        let mut buf = [0u8; 64];
        let n = slruntime::abi::sl_nat_to_decimal(w, buf.as_mut_ptr(), buf.len());
        assert_eq!(&buf[..n], text);

        // Comparison words.
        assert_eq!(slruntime::abi::sl_nat_cmp(one, max), -1);
        assert_eq!(slruntime::abi::sl_nat_cmp(max, one), 1);
        assert_eq!(slruntime::abi::sl_nat_cmp(max, max), 0);
    }
}

#[test]
fn extern_word_api_fault_codes_are_the_documented_ones() {
    // The extern layer raises these errors as process exits; the codes are
    // observable through the safe mapping.
    assert_eq!(Error::DivByZero.exit_code(), 72);
    assert_eq!(Error::NegativeResult.exit_code(), 71);
    assert_eq!(Error::CorruptValue.exit_code(), 71);
}

#[test]
fn extern_alloc_zero_fills_blocks() {
    unsafe {
        for size in [1i64, 7, 8, 9, 4096] {
            let p = slruntime::abi::sl_alloc(size);
            assert_ne!(p, 0);
            let bytes = std::slice::from_raw_parts(p as *const u8, size as usize);
            assert!(bytes.iter().all(|&b| b == 0), "size {size}");
            slruntime::abi::sl_free(p);
        }
    }
}
