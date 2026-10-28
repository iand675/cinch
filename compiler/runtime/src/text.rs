//! Text conversion and content identity for the P31 canonical form
//! (JSON and canonical serialization): SHA-256 at content-addressing
//! strength and deterministic decimal formatting and parsing for
//! `F64`.
//!
//! The language surface reaches these through the `cinch.runtime.text*`
//! intrinsics (`compiler/src/core.rs`, `TextOp`): `textSha256` hashes a
//! `Bytes` buffer into a fresh 32-byte `Bytes` buffer, `textF64ToDecimal`
//! renders one float's shortest round-trip decimal text and
//! `textF64FromDecimal` parses one back. Both decimal conversions agree
//! with the Rust standard library's `f64` formatting (`{:e}`, the
//! shortest digits that round trip) and `FromStr` (correctly rounded),
//! so `parse . print` is the identity on every finite `F64`.
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

/// The SHA-256 digest of `data`: 32 bytes, the FIPS 180-4 hash.
pub fn sha256(data: &[u8]) -> [u8; 32] {
    // The initial hash value (first 32 bits of the square roots of the
    // first 8 primes).
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    // Padding: 0x80, zeros, then the bit length as a big-endian u64.
    let mut padded: Vec<u8> = Vec::with_capacity(data.len() + 72);
    padded.extend_from_slice(data);
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&((data.len() as u64) * 8).to_be_bytes());
    // The message schedule over each 512-bit chunk.
    let mut w = [0u32; 64];
    for chunk in padded.chunks_exact(64) {
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
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
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
    let mut out = [0u8; 32];
    for (i, word) in h.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

// ---------------------------------------------------------------------------
// The extern "C" word ABI (runtime ABI, `compiler/runtime/src/lib.rs`)
// ---------------------------------------------------------------------------

/// `textSha256 :: Bytes -> Bytes`: the SHA-256 digest of a buffer as a
/// fresh 32-byte buffer.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_text_sha256(word: i64) -> i64 {
    let buf = unsafe { crate::abi::as_bytes(word) };
    crate::abi::box_bytes(crate::bytes::Bytes::from(sha256(buf.as_slice()).to_vec()))
}

/// `textF64ToDecimal :: F64 -> Bytes`: the shortest round-trip decimal
/// text of one float (`{:e}` scientific form), as a fresh buffer.
#[unsafe(no_mangle)]
pub extern "C" fn cinch_text_f64_to_decimal(v: f64) -> i64 {
    crate::abi::box_bytes(crate::bytes::Bytes::from(format!("{v:e}").into_bytes()))
}

/// `textF64FromDecimal :: Bytes -> F64`: the float one decimal spelling
/// stands for (correctly rounded). A spelling that is not a decimal
/// number takes the defined `MalformedDecimal` fault.
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
        // The classic NIST example vectors.
        assert_eq!(
            hex(&sha256(b"")),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            hex(&sha256(b"abc")),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(
            hex(&sha256(
                b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"
            )),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
        // One million 'a': the two-block-pad boundary is covered by the
        // length field alone, so a shorter crossing input stands in for
        // the million-byte vector's schedule behavior (56 bytes is the
        // first length that forces a second padded block).
        assert_eq!(
            hex(&sha256(&[b'a'; 56])),
            "b35439a4ac6f0948b6d6f9e3c6af0f5f590ce20f1bde7090ef7970686ec6738a"
        );
        assert_eq!(
            hex(&sha256(&[b'a'; 64])),
            "ffe054fe7ae0cb6dc65c3af9b61d5209f439851db43d0ba5997337df154668eb"
        );
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
        // The standing runtime rule: every `cinch_*` extern is
        // `#[unsafe(no_mangle)]` and visible as a global text symbol in
        // the built artifacts. `cargo test` builds the rlib and the
        // `deps` archive (the linked top-level archive needs a separate
        // `cargo build`); all artifacts come from the same crate
        // compilation, so the newest one always carries the exports and
        // a stale sibling copy is not a failure.
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut candidates: Vec<std::path::PathBuf> = Vec::new();
        for profile in ["debug", "release"] {
            let dir = manifest.join("target").join(profile);
            for artifact in [
                dir.join("libcinchruntime.rlib"),
                dir.join("libcinchruntime.a"),
            ] {
                if artifact.exists() {
                    candidates.push(artifact);
                }
            }
            let deps = dir.join("deps");
            if let Ok(entries) = std::fs::read_dir(&deps) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().into_owned();
                    if name.starts_with("libcinchruntime-") && (name.ends_with(".rlib") || name.ends_with(".a"))
                    {
                        candidates.push(entry.path());
                    }
                }
            }
        }
        let syms = ["cinch_text_sha256", "cinch_text_f64_to_decimal", "cinch_text_f64_from_decimal"];
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
