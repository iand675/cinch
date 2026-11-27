//! Text conversion and content identity for the P31 canonical form
//! (JSON and canonical serialization): SHA-256 at content-addressing
//! strength and deterministic decimal formatting and parsing for
//! `F64`.
//!
//! The language surface reaches these through the `cinch.runtime.text*`
//! intrinsics (`seed/src/core.rs`, `TextOp`): `textSha256` hashes a
//! `Bytes` buffer into a fresh 32-byte `Bytes` buffer, `textF64ToDecimal`
//! renders one float's shortest round-trip decimal text and
//! `textF64FromDecimal` parses one back. Both decimal conversions agree
//! with the Rust standard library's `f64` formatting (`{:e}`, the
//! shortest digits that round trip) and `FromStr` (correctly rounded),
//! so `parse . print` is the identity on every finite `F64`.
//!
//! The content-integrity family also carries the other digests the
//! standard library round needs (Ruby's `Digest` and `OpenSSL::HMAC`
//! items, design/stdlib-parity-ruby.md): SHA-512 (FIPS 180-4), SHA-1
//! (FIPS 180-1), MD5 (RFC 1321) and HMAC-SHA-256 (RFC 2104, block size
//! 64). The portable implementations and platform SHA-256 are tested
//! against the published test vectors
//! (FIPS 180-4, FIPS 180-1, RFC 1321 and RFC 4231) in both the Rust
//! tests below and the prelude-facing `seed/tests/runtime_prims.rs`.
//! These are content-integrity primitives, not authentication schemes:
//! the digest says nothing about who sent the bytes.
//!
//! `cinch_text_f64_from_decimal` faults on a spelling that is not a
//! decimal number (`Error::MalformedDecimal`, exit 71): the JSON
//! grammar validates spellings before the conversion, so a failure is
//! a broken contract, not a user-level outcome.
//!
//! SHA-256 is implemented here (FIPS 180-4) rather than in the
//! language, matching the runtime rule that a content-identity hash is
//! a `cinch_*` extern (`#[unsafe(no_mangle)]`, `nm`-verified by the
//! tests below).

use crate::error::Error;

// ---------------------------------------------------------------------------
// SHA-256 (FIPS 180-4)
// ---------------------------------------------------------------------------

/// The 64 round constants (first 32 bits of the cube roots of the first
/// 64 primes).
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// Initial hash values for SHA-256 (first 32 bits of the square roots of
/// the first 8 primes).
const SHA256_IV: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
    0x5be0cd19,
];

/// The SHA-256 digest of `data`: 32 bytes, the FIPS 180-4 hash.
pub fn sha256(data: &[u8]) -> [u8; 32] {
    #[cfg(target_os = "macos")]
    if let Some(digest) = crate::sys::darwin::sha256(data) {
        return digest;
    }
    sha256_portable(data)
}

#[inline]
fn sha256_compress(h: &mut [u32; 8], chunk: &[u8]) {
    let mut w = [0u32; 64];
    for (i, word) in chunk.chunks_exact(4).enumerate() {
        w[i] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
    }
    for i in 16..64 {
        let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
        let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
        w[i] = w[i - 16]
            .wrapping_add(s0)
            .wrapping_add(w[i - 7])
            .wrapping_add(s1);
    }
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = *h;
    for i in 0..64 {
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch = (e & f) ^ (!e & g);
        let t1 = hh
            .wrapping_add(s1)
            .wrapping_add(ch)
            .wrapping_add(K[i])
            .wrapping_add(w[i]);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let t2 = s0.wrapping_add(maj);
        hh = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }
    h[0] = h[0].wrapping_add(a);
    h[1] = h[1].wrapping_add(b);
    h[2] = h[2].wrapping_add(c);
    h[3] = h[3].wrapping_add(d);
    h[4] = h[4].wrapping_add(e);
    h[5] = h[5].wrapping_add(f);
    h[6] = h[6].wrapping_add(g);
    h[7] = h[7].wrapping_add(hh);
}

#[inline]
fn sha256_finish_stream(mut h: [u32; 8], data: &[u8], total_bytes: u64) -> [u8; 32] {
    // Compress the input in place. Only the tail needs a writable buffer.
    let chunks = data.chunks_exact(64);
    let remainder = chunks.remainder();
    let mut padded = [0u8; 128];
    padded[..remainder.len()].copy_from_slice(remainder);
    padded[remainder.len()] = 0x80;
    let end = if remainder.len() < 56 { 64 } else { 128 };
    let bits = total_bytes.wrapping_mul(8).to_be_bytes();
    padded[end - bits.len()..end].copy_from_slice(&bits);
    for chunk in chunks.chain(padded[..end].chunks_exact(64)) {
        sha256_compress(&mut h, chunk);
    }
    let mut out = [0u8; 32];
    for (i, word) in h.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

fn sha256_portable(data: &[u8]) -> [u8; 32] {
    sha256_finish_stream(SHA256_IV, data, data.len() as u64)
}

// ---------------------------------------------------------------------------
// SHA-512 (FIPS 180-4)
// ---------------------------------------------------------------------------

/// The 80 round constants (first 64 bits of the cube roots of the first
/// 80 primes).
const K512: [u64; 80] = [
    0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc,
    0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118,
    0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694,
    0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
    0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
    0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70,
    0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
    0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
    0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30,
    0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
    0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b,
    0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178,
    0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
    0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817,
];

/// The SHA-512 digest of `data`: 64 bytes, the FIPS 180-4 hash. The
/// length field is the message's bit length as a big-endian 128-bit
/// integer (a `usize` byte count never overflows it).
pub fn sha512(data: &[u8]) -> [u8; 64] {
    #[cfg(target_os = "macos")]
    if let Some(digest) = crate::sys::darwin::sha512(data) {
        return digest;
    }
    sha512_portable(data)
}

fn sha512_portable(data: &[u8]) -> [u8; 64] {
    // The initial hash value (first 64 bits of the square roots of the
    // first 8 primes).
    let mut h: [u64; 8] = [
        0x6a09e667f3bcc908, 0xbb67ae8584caa73b, 0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
        0x510e527fade682d1, 0x9b05688c2b3e6c1f, 0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
    ];
    // Compress the input in place. Only the tail needs a writable buffer.
    let chunks = data.chunks_exact(128);
    let remainder = chunks.remainder();
    let mut padded = [0u8; 256];
    padded[..remainder.len()].copy_from_slice(remainder);
    padded[remainder.len()] = 0x80;
    let end = if remainder.len() < 112 { 128 } else { 256 };
    let bits = (data.len() as u128).wrapping_mul(8).to_be_bytes();
    padded[end - bits.len()..end].copy_from_slice(&bits);

    macro_rules! sha512_round {
        ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $hh:expr, $k:expr, $w:expr) => {{
            let s1 = $e.rotate_right(14) ^ $e.rotate_right(18) ^ $e.rotate_right(41);
            let ch = ($e & $f) ^ (!$e & $g);
            let t1 = $hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add($k)
                .wrapping_add($w);
            let s0 = $a.rotate_right(28) ^ $a.rotate_right(34) ^ $a.rotate_right(39);
            let maj = ($a & $b) ^ ($a & $c) ^ ($b & $c);
            let t2 = s0.wrapping_add(maj);
            $d = $d.wrapping_add(t1);
            $hh = t1.wrapping_add(t2);
        }};
    }

    macro_rules! sha512_sched {
        ($w:expr, $idx:expr) => {{
            let i = $idx & 15;
            let w15 = $w[(i + 1) & 15];
            let w2 = $w[(i + 14) & 15];
            let s0 = w15.rotate_right(1) ^ w15.rotate_right(8) ^ (w15 >> 7);
            let s1 = w2.rotate_right(19) ^ w2.rotate_right(61) ^ (w2 >> 6);
            $w[i] = $w[i]
                .wrapping_add(s0)
                .wrapping_add($w[(i + 9) & 15])
                .wrapping_add(s1);
            $w[i]
        }};
    }

    let mut w = [0u64; 16];
    for chunk in chunks.chain(padded[..end].chunks_exact(128)) {
        for (i, word) in chunk.chunks_exact(8).enumerate() {
            w[i] = u64::from_be_bytes([
                word[0], word[1], word[2], word[3], word[4], word[5], word[6], word[7],
            ]);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
        // Rounds 0..16: message schedule words come directly from the block.
        for base in (0..16).step_by(8) {
            sha512_round!(a, b, c, d, e, f, g, hh, K512[base], w[base]);
            sha512_round!(hh, a, b, c, d, e, f, g, K512[base + 1], w[base + 1]);
            sha512_round!(g, hh, a, b, c, d, e, f, K512[base + 2], w[base + 2]);
            sha512_round!(f, g, hh, a, b, c, d, e, K512[base + 3], w[base + 3]);
            sha512_round!(e, f, g, hh, a, b, c, d, K512[base + 4], w[base + 4]);
            sha512_round!(d, e, f, g, hh, a, b, c, K512[base + 5], w[base + 5]);
            sha512_round!(c, d, e, f, g, hh, a, b, K512[base + 6], w[base + 6]);
            sha512_round!(b, c, d, e, f, g, hh, a, K512[base + 7], w[base + 7]);
        }
        // Rounds 16..80: 16-word rolling message schedule, 8 rounds unrolled.
        for base in (16..80).step_by(8) {
            let w0 = sha512_sched!(w, base);
            sha512_round!(a, b, c, d, e, f, g, hh, K512[base], w0);
            let w1 = sha512_sched!(w, base + 1);
            sha512_round!(hh, a, b, c, d, e, f, g, K512[base + 1], w1);
            let w2 = sha512_sched!(w, base + 2);
            sha512_round!(g, hh, a, b, c, d, e, f, K512[base + 2], w2);
            let w3 = sha512_sched!(w, base + 3);
            sha512_round!(f, g, hh, a, b, c, d, e, K512[base + 3], w3);
            let w4 = sha512_sched!(w, base + 4);
            sha512_round!(e, f, g, hh, a, b, c, d, K512[base + 4], w4);
            let w5 = sha512_sched!(w, base + 5);
            sha512_round!(d, e, f, g, hh, a, b, c, K512[base + 5], w5);
            let w6 = sha512_sched!(w, base + 6);
            sha512_round!(c, d, e, f, g, hh, a, b, K512[base + 6], w6);
            let w7 = sha512_sched!(w, base + 7);
            sha512_round!(b, c, d, e, f, g, hh, a, K512[base + 7], w7);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }
    let mut out = [0u8; 64];
    for (i, word) in h.iter().enumerate() {
        out[i * 8..i * 8 + 8].copy_from_slice(&word.to_be_bytes());
    }
    out
}

// ---------------------------------------------------------------------------
// SHA-1 (FIPS 180-1)
// ---------------------------------------------------------------------------

/// The SHA-1 digest of `data`: 20 bytes, the FIPS 180-1 hash. Retained
/// for legacy content identity (the prelude's `Digest` surface); it is
/// broken against collision attacks and must not be used for new
/// security decisions.
pub fn sha1(data: &[u8]) -> [u8; 20] {
    let mut h: [u32; 5] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];
    // Compress the input in place. Only the tail needs a writable buffer.
    let chunks = data.chunks_exact(64);
    let remainder = chunks.remainder();
    let mut padded = [0u8; 128];
    padded[..remainder.len()].copy_from_slice(remainder);
    padded[remainder.len()] = 0x80;
    let end = if remainder.len() < 56 { 64 } else { 128 };
    let bits = (data.len() as u64).wrapping_mul(8).to_be_bytes();
    padded[end - bits.len()..end].copy_from_slice(&bits);

    macro_rules! sha1_step {
        ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $k:expr, $w:expr) => {{
            $e = $a
                .rotate_left(5)
                .wrapping_add($f)
                .wrapping_add($e)
                .wrapping_add($k)
                .wrapping_add($w);
            $b = $b.rotate_left(30);
        }};
    }

    let mut w = [0u32; 80];
    for chunk in chunks.chain(padded[..end].chunks_exact(64)) {
        for (i, word) in chunk.chunks_exact(4).enumerate() {
            w[i] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }
        let [mut a, mut b, mut c, mut d, mut e] = h;
        // Rounds 0..20
        for base in (0..20).step_by(5) {
            sha1_step!(a, b, c, d, e, d ^ (b & (c ^ d)), 0x5a827999u32, w[base]);
            sha1_step!(e, a, b, c, d, c ^ (a & (b ^ c)), 0x5a827999u32, w[base + 1]);
            sha1_step!(d, e, a, b, c, b ^ (e & (a ^ b)), 0x5a827999u32, w[base + 2]);
            sha1_step!(c, d, e, a, b, a ^ (d & (e ^ a)), 0x5a827999u32, w[base + 3]);
            sha1_step!(b, c, d, e, a, e ^ (c & (d ^ e)), 0x5a827999u32, w[base + 4]);
        }
        // Rounds 20..40
        for base in (20..40).step_by(5) {
            sha1_step!(a, b, c, d, e, b ^ c ^ d, 0x6ed9eba1u32, w[base]);
            sha1_step!(e, a, b, c, d, a ^ b ^ c, 0x6ed9eba1u32, w[base + 1]);
            sha1_step!(d, e, a, b, c, e ^ a ^ b, 0x6ed9eba1u32, w[base + 2]);
            sha1_step!(c, d, e, a, b, d ^ e ^ a, 0x6ed9eba1u32, w[base + 3]);
            sha1_step!(b, c, d, e, a, c ^ d ^ e, 0x6ed9eba1u32, w[base + 4]);
        }
        // Rounds 40..60
        for base in (40..60).step_by(5) {
            sha1_step!(a, b, c, d, e, (b & c) | (d & (b | c)), 0x8f1bbcdcu32, w[base]);
            sha1_step!(e, a, b, c, d, (a & b) | (c & (a | b)), 0x8f1bbcdcu32, w[base + 1]);
            sha1_step!(d, e, a, b, c, (e & a) | (b & (e | a)), 0x8f1bbcdcu32, w[base + 2]);
            sha1_step!(c, d, e, a, b, (d & e) | (a & (d | e)), 0x8f1bbcdcu32, w[base + 3]);
            sha1_step!(b, c, d, e, a, (c & d) | (e & (c | d)), 0x8f1bbcdcu32, w[base + 4]);
        }
        // Rounds 60..80
        for base in (60..80).step_by(5) {
            sha1_step!(a, b, c, d, e, b ^ c ^ d, 0xca62c1d6u32, w[base]);
            sha1_step!(e, a, b, c, d, a ^ b ^ c, 0xca62c1d6u32, w[base + 1]);
            sha1_step!(d, e, a, b, c, e ^ a ^ b, 0xca62c1d6u32, w[base + 2]);
            sha1_step!(c, d, e, a, b, d ^ e ^ a, 0xca62c1d6u32, w[base + 3]);
            sha1_step!(b, c, d, e, a, c ^ d ^ e, 0xca62c1d6u32, w[base + 4]);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }
    let mut out = [0u8; 20];
    for (i, word) in h.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

// ---------------------------------------------------------------------------
// MD5 (RFC 1321)
// ---------------------------------------------------------------------------

/// The MD5 digest of `data`: 16 bytes, the RFC 1321 hash (little-endian
/// words on the wire). Retained for legacy content identity; broken
/// against collision attacks, never a new security decision.
pub fn md5(data: &[u8]) -> [u8; 16] {
    /// The 64 additive constants (RFC 1321's `T` table:
    /// `floor(2^32 * abs(sin(i + 1)))`).
    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];
    let mut a0: u32 = 0x67452301;
    let mut b0: u32 = 0xefcdab89;
    let mut c0: u32 = 0x98badcfe;
    let mut d0: u32 = 0x10325476;
    // Compress the input in place. Only the tail needs a writable buffer.
    let chunks = data.chunks_exact(64);
    let remainder = chunks.remainder();
    let mut padded = [0u8; 128];
    padded[..remainder.len()].copy_from_slice(remainder);
    padded[remainder.len()] = 0x80;
    let end = if remainder.len() < 56 { 64 } else { 128 };
    // The bit length is little-endian for MD5.
    let bits = (data.len() as u64).wrapping_mul(8).to_le_bytes();
    padded[end - bits.len()..end].copy_from_slice(&bits);

    macro_rules! md5_step {
        ($a:expr, $b:expr, $c:expr, $d:expr, $f:expr, $m:expr, $k:expr, $s:expr) => {{
            $a = $b.wrapping_add(
                $a.wrapping_add($f)
                    .wrapping_add($k)
                    .wrapping_add($m)
                    .rotate_left($s),
            );
        }};
    }

    for chunk in chunks.chain(padded[..end].chunks_exact(64)) {
        let mut m = [0u32; 16];
        for (i, word) in chunk.chunks_exact(4).enumerate() {
            m[i] = u32::from_le_bytes([word[0], word[1], word[2], word[3]]);
        }
        let (mut a, mut b, mut c, mut d) = (a0, b0, c0, d0);
        // Round 1 (0..16): g = i
        for base in (0..16).step_by(4) {
            md5_step!(a, b, c, d, d ^ (b & (c ^ d)), m[base], K[base], 7);
            md5_step!(d, a, b, c, c ^ (a & (b ^ c)), m[base + 1], K[base + 1], 12);
            md5_step!(c, d, a, b, b ^ (d & (a ^ b)), m[base + 2], K[base + 2], 17);
            md5_step!(b, c, d, a, a ^ (c & (d ^ a)), m[base + 3], K[base + 3], 22);
        }
        // Round 2 (16..32): g = (5 * i + 1) & 15
        for base in (16..32).step_by(4) {
            md5_step!(a, b, c, d, c ^ (d & (b ^ c)), m[(5 * base + 1) & 15], K[base], 5);
            md5_step!(d, a, b, c, b ^ (c & (a ^ b)), m[(5 * (base + 1) + 1) & 15], K[base + 1], 9);
            md5_step!(c, d, a, b, a ^ (b & (d ^ a)), m[(5 * (base + 2) + 1) & 15], K[base + 2], 14);
            md5_step!(b, c, d, a, d ^ (a & (c ^ d)), m[(5 * (base + 3) + 1) & 15], K[base + 3], 20);
        }
        // Round 3 (32..48): g = (3 * i + 5) & 15
        for base in (32..48).step_by(4) {
            md5_step!(a, b, c, d, b ^ c ^ d, m[(3 * base + 5) & 15], K[base], 4);
            md5_step!(d, a, b, c, a ^ b ^ c, m[(3 * (base + 1) + 5) & 15], K[base + 1], 11);
            md5_step!(c, d, a, b, d ^ a ^ b, m[(3 * (base + 2) + 5) & 15], K[base + 2], 16);
            md5_step!(b, c, d, a, c ^ d ^ a, m[(3 * (base + 3) + 5) & 15], K[base + 3], 23);
        }
        // Round 4 (48..64): g = (7 * i) & 15
        for base in (48..64).step_by(4) {
            md5_step!(a, b, c, d, c ^ (b | !d), m[(7 * base) & 15], K[base], 6);
            md5_step!(d, a, b, c, b ^ (a | !c), m[(7 * (base + 1)) & 15], K[base + 1], 10);
            md5_step!(c, d, a, b, a ^ (d | !b), m[(7 * (base + 2)) & 15], K[base + 2], 15);
            md5_step!(b, c, d, a, d ^ (c | !a), m[(7 * (base + 3)) & 15], K[base + 3], 21);
        }
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }
    let mut out = [0u8; 16];
    out[0..4].copy_from_slice(&a0.to_le_bytes());
    out[4..8].copy_from_slice(&b0.to_le_bytes());
    out[8..12].copy_from_slice(&c0.to_le_bytes());
    out[12..16].copy_from_slice(&d0.to_le_bytes());
    out
}

// ---------------------------------------------------------------------------
// HMAC-SHA-256 (RFC 2104)
// ---------------------------------------------------------------------------

/// HMAC-SHA-256 of `message` under `key`: 32 bytes (RFC 2104, hash
/// SHA-256, block size 64). A key longer than the block is hashed first
/// (`H(key)`), per the specification; the two 64-byte pads are `k ^
/// 0x36` (inner) and `k ^ 0x5c` (outer).
pub fn hmac_sha256(key: &[u8], message: &[u8]) -> [u8; 32] {
    #[cfg(target_os = "macos")]
    {
        return crate::sys::darwin::hmac_sha256(key, message);
    }
    #[cfg(not(target_os = "macos"))]
    {
        hmac_sha256_portable(key, message)
    }
}

/// Zero-copy, zero-allocation portable HMAC-SHA-256: compresses the 64-byte
/// `ipad` directly into the initial SHA-256 state, streams `message` in place
/// with no buffer copy, and hashes the 96-byte `outer` block on the stack.
#[cfg_attr(target_os = "macos", allow(dead_code))]
fn hmac_sha256_portable(key: &[u8], message: &[u8]) -> [u8; 32] {
    const BLOCK: usize = 64;
    let mut k = [0u8; BLOCK];
    if key.len() > BLOCK {
        k[..32].copy_from_slice(&sha256(key));
    } else {
        k[..key.len()].copy_from_slice(key);
    }
    let mut ipad = [0x36u8; BLOCK];
    let mut opad = [0x5cu8; BLOCK];
    for i in 0..BLOCK {
        ipad[i] ^= k[i];
        opad[i] ^= k[i];
    }
    let mut inner_h = SHA256_IV;
    sha256_compress(&mut inner_h, &ipad);
    let inner_digest = sha256_finish_stream(inner_h, message, (BLOCK + message.len()) as u64);
    let mut outer = [0u8; BLOCK + 32];
    outer[..BLOCK].copy_from_slice(&opad);
    outer[BLOCK..].copy_from_slice(&inner_digest);
    sha256(&outer)
}

// ---------------------------------------------------------------------------
// The extern "C" word ABI (runtime ABI, `seed/runtime/src/lib.rs`)
// ---------------------------------------------------------------------------

#[cfg(not(test))]
unsafe extern "C" {
    pub fn cinch_text_sha256(word: i64) -> i64;
    pub fn cinch_text_sha512(word: i64) -> i64;
    pub fn cinch_text_sha1(word: i64) -> i64;
    pub fn cinch_text_md5(word: i64) -> i64;
    pub fn cinch_text_hmac_sha256(key: i64, message: i64) -> i64;
    pub fn cinch_text_f64_to_decimal(v: f64) -> i64;
    pub fn cinch_text_f64_from_decimal(word: i64) -> f64;
}

/// `textSha256 :: Bytes -> Bytes`: the SHA-256 digest of a buffer as a
/// fresh 32-byte buffer.
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_text_sha256(word: i64) -> i64 {
    let buf = unsafe { crate::abi::as_bytes(word) };
    crate::abi::box_bytes(crate::bytes::Bytes::from(sha256(buf.as_slice()).to_vec()))
}

/// `textSha512 :: Bytes -> Bytes`: the SHA-512 digest of a buffer as a
/// fresh 64-byte buffer (FIPS 180-4).
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_text_sha512(word: i64) -> i64 {
    let buf = unsafe { crate::abi::as_bytes(word) };
    crate::abi::box_bytes(crate::bytes::Bytes::from(sha512(buf.as_slice()).to_vec()))
}

/// `textSha1 :: Bytes -> Bytes`: the SHA-1 digest of a buffer as a
/// fresh 20-byte buffer (FIPS 180-1).
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_text_sha1(word: i64) -> i64 {
    let buf = unsafe { crate::abi::as_bytes(word) };
    crate::abi::box_bytes(crate::bytes::Bytes::from(sha1(buf.as_slice()).to_vec()))
}

/// `textMd5 :: Bytes -> Bytes`: the MD5 digest of a buffer as a fresh
/// 16-byte buffer (RFC 1321).
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_text_md5(word: i64) -> i64 {
    let buf = unsafe { crate::abi::as_bytes(word) };
    crate::abi::box_bytes(crate::bytes::Bytes::from(md5(buf.as_slice()).to_vec()))
}

/// `textHmacSha256 :: Bytes -> Bytes -> Bytes`: HMAC-SHA-256 of the
/// message under the key as a fresh 32-byte buffer (RFC 2104).
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_text_hmac_sha256(key: i64, message: i64) -> i64 {
    let key = unsafe { crate::abi::as_bytes(key) };
    let message = unsafe { crate::abi::as_bytes(message) };
    crate::abi::box_bytes(crate::bytes::Bytes::from(
        hmac_sha256(key.as_slice(), message.as_slice()).to_vec(),
    ))
}

/// `textF64ToDecimal :: F64 -> Bytes`: the shortest round-trip decimal
/// text of one float (`{:e}` scientific form), as a fresh buffer.
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn cinch_text_f64_to_decimal(v: f64) -> i64 {
    crate::abi::box_bytes(crate::bytes::Bytes::from(format!("{v:e}").into_bytes()))
}

/// `textF64FromDecimal :: Bytes -> F64`: the float one decimal spelling
/// stands for (correctly rounded). A spelling that is not a decimal
/// number takes the defined `MalformedDecimal` fault.
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_text_f64_from_decimal(word: i64) -> f64 {
    let buf = unsafe { crate::abi::as_bytes(word) };
    match std::str::from_utf8(buf.as_slice())
        .ok()
        .and_then(|s| s.parse::<f64>().ok())
    {
        Some(v) => v,
        None => crate::abi::raise(Error::MalformedDecimal),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(digest: &[u8; 32]) -> String {
        digest.iter().map(|b| format!("{b:02x}")).collect()
    }

    #[test]
    fn sha256_matches_the_fips_180_4_vectors() {
        let million = vec![b'a'; 1_000_000];
        let vectors: &[(&[u8], &str)] = &[
            (b"", "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"),
            (b"abc", "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"),
            (
                b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
                "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
            ),
            // 56 bytes forces a second padding block. 64 bytes fills the
            // first data block before adding padding.
            (&[b'a'; 56], "b35439a4ac6f0948b6d6f9e3c6af0f5f590ce20f1bde7090ef7970686ec6738a"),
            (&[b'a'; 64], "ffe054fe7ae0cb6dc65c3af9b61d5209f439851db43d0ba5997337df154668eb"),
            (&million, "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0"),
        ];
        for &(input, expected) in vectors {
            assert_eq!(hex(&sha256(input)), expected);
            assert_eq!(hex(&sha256_portable(input)), expected);
        }
    }

    fn hex64(digest: &[u8]) -> String {
        digest.iter().map(|b| format!("{b:02x}")).collect()
    }

    #[test]
    fn sha512_matches_the_fips_180_4_vectors() {
        let million = vec![b'a'; 1_000_000];
        let vectors: &[(&[u8], &str)] = &[
            (
                b"abc",
                "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f",
            ),
            (
                b"",
                "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e",
            ),
            (
                b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmnhijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
                "8e959b75dae313da8cf4f72814fc143f8f7779c6eb9f7fa17299aeadb6889018501d289e4900f7e4331b99dec4b5433ac7d329eeb6dd26545e96e55b874be909",
            ),
            (
                &million,
                "e718483d0ce769644e2e42c7bc15b4638e1f98b13b2044285632a803afa973ebde0ff244877ea60a4cb0432ce577c31beb009c5c2c49aa2e4eadb217ad8cc09b",
            ),
        ];
        for &(input, expected) in vectors {
            assert_eq!(hex64(&sha512(input)), expected);
            assert_eq!(hex64(&sha512_portable(input)), expected);
        }
    }

    #[test]
    fn sha1_matches_the_fips_180_1_vectors() {
        let million = vec![b'a'; 1_000_000];
        assert_eq!(hex64(&sha1(b"abc")), "a9993e364706816aba3e25717850c26c9cd0d89d");
        assert_eq!(hex64(&sha1(b"")), "da39a3ee5e6b4b0d3255bfef95601890afd80709");
        assert_eq!(
            hex64(&sha1(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq")),
            "84983e441c3bd26ebaae4aa1f95129e5e54670f1"
        );
        assert_eq!(hex64(&sha1(&million)), "34aa973cd4c4daa4f61eeb2bdbad27316534016f");
    }

    #[test]
    fn md5_matches_the_rfc_1321_vectors() {
        assert_eq!(hex64(&md5(b"abc")), "900150983cd24fb0d6963f7d28e17f72");
        assert_eq!(hex64(&md5(b"")), "d41d8cd98f00b204e9800998ecf8427e");
        assert_eq!(hex64(&md5(b"message digest")), "f96b697d7cb7938d525a2f31aaf161d0");
        assert_eq!(
            hex64(&md5(b"abcdefghijklmnopqrstuvwxyz")),
            "c3fcd3d76192e4007dfb496cca67e13b"
        );
        assert_eq!(
            hex64(&md5(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789")),
            "d174ab98d277d9f5a5611c2c9f419d9f"
        );
        assert_eq!(
            hex64(&md5(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890")),
            "57edf4a22be3c955ac49da2e2107b67a"
        );
    }

    #[test]
    fn hmac_sha256_matches_the_rfc_4231_vectors() {
        let vectors: &[(&[u8], &[u8], &str)] = &[
            // Case 1
            (
                &[0x0bu8; 20],
                b"Hi There",
                "b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7",
            ),
            // Case 2: key "Jefe", data "what do ya want for nothing?".
            (
                b"Jefe",
                b"what do ya want for nothing?",
                "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843",
            ),
            // Case 3
            (
                &[0xaau8; 20],
                &[0xddu8; 50],
                "773ea91e36800e46854db8ebd09181a72959098b3ef8c122d9635514ced565fe",
            ),
            // Case 6: a key longer than the 64-byte block exercises the
            // H(key) branch.
            (
                &[0xaau8; 131],
                b"Test Using Larger Than Block-Size Key - Hash Key First",
                "60e431591ee0b67f0d8a26aacbf5b77f8e0bc6213728c5140546040f0ee37f54",
            ),
            // Case 7
            (
                &[0xaau8; 131],
                b"This is a test using a larger than block-size key and a larger than block-size data. The key needs to be hashed before being used by the HMAC algorithm.",
                "9b09ffa71b942fcb27635fbcd5b0e944bfdc63644f0713938a7f51535c3a35e2",
            ),
        ];
        for &(key, msg, expected) in vectors {
            assert_eq!(hex64(&hmac_sha256(key, msg)), expected);
            assert_eq!(hex64(&hmac_sha256_portable(key, msg)), expected);
        }
    }

    #[test]
    fn digests_agree_across_padding_boundaries_and_large_inputs() {
        let lengths = [
            0usize, 1, 55, 56, 63, 64, 65, 111, 112, 119, 120, 127, 128, 129, 255, 256, 1_048_576,
        ];
        let max_len = *lengths.last().unwrap();
        let buf: Vec<u8> = (0..max_len).map(|i| (i.wrapping_mul(37).wrapping_add(13)) as u8).collect();
        let keys: &[&[u8]] = &[b"", b"short", &[0x5au8; 64], &[0x3cu8; 65], &[0x99u8; 131]];

        for &len in &lengths {
            let slice = &buf[..len];
            assert_eq!(sha256(slice), sha256_portable(slice), "sha256 len={len}");
            assert_eq!(sha512(slice), sha512_portable(slice), "sha512 len={len}");
            for &key in keys {
                assert_eq!(
                    hmac_sha256(key, slice),
                    hmac_sha256_portable(key, slice),
                    "hmac_sha256 key_len={} msg_len={len}",
                    key.len()
                );
            }
        }
        // Boundary checks for SHA-1 and MD5 at 55, 56, 63, 64, 65, 128 bytes.
        let sha1_bounds: &[(usize, &str)] = &[
            (55, "c1c8bbdc22796e28c0e15163d20899b65621d65a"),
            (56, "c2db330f6083854c99d4b5bfb6e8f29f201be699"),
            (63, "03f09f5b158a7a8cdad920bddc29b81c18a551f5"),
            (64, "0098ba824b5c16427bd7a1122a5a442a25ec644d"),
            (65, "11655326c708d70319be2610e8a57d9a5b959d3b"),
            (128, "ad5b3fdbcb526778c2839d2f151ea753995e26a0"),
        ];
        let md5_bounds: &[(usize, &str)] = &[
            (55, "ef1772b6dff9a122358552954ad0df65"),
            (56, "3b0c8ac703f828b04c6c197006d17218"),
            (63, "b06521f39153d618550606be297466d5"),
            (64, "014842d480b571495a4a0363793f7367"),
            (65, "c743a45e0d2e6a95cb859adae0248435"),
            (128, "e510683b3f5ffe4093d021808bc6ff70"),
        ];
        for &(n, expected) in sha1_bounds {
            assert_eq!(hex64(&sha1(&vec![b'a'; n])), expected, "sha1 len={n}");
        }
        for &(n, expected) in md5_bounds {
            assert_eq!(hex64(&md5(&vec![b'a'; n])), expected, "md5 len={n}");
        }
    }

    #[test]
    fn f64_decimal_text_round_trips() {
        for v in [
            0.0f64,
            -0.0,
            1.0,
            -1.0,
            0.5,
            2.5,
            100.0,
            1e21,
            1e-7,
            3.141592653589793,
            2.718281828459045,
            f64::MIN_POSITIVE,
            f64::MAX,
            5e-324,
            -42.625,
            123456789.123456789,
        ] {
            let word = cinch_text_f64_to_decimal(v);
            let back = unsafe { cinch_text_f64_from_decimal(word) };
            assert_eq!(back.to_bits(), v.to_bits(), "round trip of {v:e}");
        }
    }

    #[test]
    fn f64_decimal_text_is_the_shortest_scientific_form() {
        let text = |v: f64| {
            let word = cinch_text_f64_to_decimal(v);
            let buf = unsafe { crate::abi::as_bytes(word) };
            String::from_utf8(buf.as_slice().to_vec()).expect("ascii")
        };
        assert_eq!(text(2.5), "2.5e0");
        assert_eq!(text(-42.0), "-4.2e1");
        assert_eq!(text(100.0), "1e2");
    }

    #[test]
    fn the_text_externs_are_nm_visible() {
        let candidates = crate::test_artifacts::runtime_archives();
        let syms = [
            "cinch_text_sha256",
            "cinch_text_sha512",
            "cinch_text_sha1",
            "cinch_text_md5",
            "cinch_text_hmac_sha256",
            "cinch_text_f64_to_decimal",
            "cinch_text_f64_from_decimal",
        ];
        for archive in &candidates {
            let out = std::process::Command::new("nm")
                .arg(archive)
                .output()
                .expect("run nm on the runtime artifact");
            let listing = String::from_utf8_lossy(&out.stdout);
            let all_present = syms.iter().all(|sym| {
                listing.lines().any(|line| {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    // `nm` prints `address T name` (GNU) or `address T
                    // _name` (Mach-O); `U` marks an undefined reference,
                    // which is not an export.
                    fields.len() >= 3
                        && fields[1] == "T"
                        && (fields[2] == *sym || fields[2] == format!("_{sym}"))
                })
            });
            if all_present {
                return;
            }
        }
        panic!(
            "the cinch_text_* exports are not global text symbols in any runtime artifact ({candidates:?})"
        );
    }
}
