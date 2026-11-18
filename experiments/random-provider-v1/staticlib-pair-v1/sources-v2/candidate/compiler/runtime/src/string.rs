//! UTF-8 primitives behind the language-level `String` type.
//!
//! These are the accelerated implementations of the pure Prelude
//! reference semantics (`Prelude.cinch`'s `String` library): the fast
//! paths must agree with the reference on every input, which the tests
//! in `tests/string.rs` check directly against naive implementations.
//!
//! All entry points are `extern "C"` words over byte buffers (the
//! `Bytes` blocks): pointer + length pairs supplied by generated code.
//! Invalid input takes the defined fault path (exit 71) only for
//! contract violations (bad pointer or lengths); malformed UTF-8 is a
//! value-level answer (0 / -1 / `None` encoding), never a fault.

use crate::error::{Error, Result};

/// Validate a UTF-8 buffer: 1 = valid, 0 = malformed. Never faults on
/// malformed content.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_utf8_validate(ptr: *const u8, len: i64) -> i64 {
    match unsafe { bytes_slice(ptr, len) } {
        Ok(b) => i64::from(validate(b)),
        Err(_) => 0,
    }
}

/// Count Unicode scalar values in a valid UTF-8 buffer, or -1 when the
/// buffer is malformed (the count is undefined then).
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_utf8_count_chars(ptr: *const u8, len: i64) -> i64 {
    match unsafe { bytes_slice(ptr, len) } {
        Ok(b) => match count_chars(b) {
            Some(n) => n as i64,
            None => -1,
        },
        Err(_) => -1,
    }
}

/// Decode the scalar value at a byte index of a valid UTF-8 buffer.
/// Returns `codepoint + width * 2^32` (width 1..=4), or -1 when the
/// index is out of range or the encoding is malformed at that index.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_utf8_decode_at(ptr: *const u8, len: i64, index: i64) -> i64 {
    let Ok(b) = (unsafe { bytes_slice(ptr, len) }) else {
        return -1;
    };
    if index < 0 {
        return -1;
    }
    let idx = index as usize;
    match decode_at(b, idx) {
        Some((cp, width)) => cp as i64 + (width as i64) * (1i64 << 32),
        None => -1,
    }
}

/// Encode a Unicode scalar value into `out` (capacity `cap` bytes).
/// Returns the number of bytes written (1..=4), or -1 when the value is
/// not a scalar value (surrogates and above 0x10FFFF) or `cap` is too
/// small.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_utf8_encode(cp: i64, out: *mut u8, cap: i64) -> i64 {
    if cp < 0 || cp > 0x10FFFF || (0xD800..=0xDFFF).contains(&cp) {
        return -1;
    }
    if cap < 0 {
        return -1;
    }
    let buf = if cap == 0 {
        &mut [][..]
    } else if out.is_null() {
        let msg = b"compiler bug: cinch_utf8_encode: null output buffer";
        unsafe { crate::abi::cinch_panic(msg.as_ptr(), msg.len()) }
    } else {
        unsafe { std::slice::from_raw_parts_mut(out, cap as usize) }
    };
    match encode(cp as u32, buf) {
        Some(n) => n as i64,
        None => -1,
    }
}

/// Borrow a byte slice from a raw pointer and length (the `Bytes` FFI
/// convention; zero length accepts a null pointer).
unsafe fn bytes_slice<'a>(ptr: *const u8, len: i64) -> Result<&'a [u8]> {
    if len < 0 {
        return Err(Error::OutOfBounds);
    }
    if len == 0 {
        return Ok(&[]);
    }
    if ptr.is_null() {
        return Err(Error::CorruptValue);
    }
    Ok(unsafe { std::slice::from_raw_parts(ptr, len as usize) })
}

/// Scalar validation (the reference semantics).
pub fn validate(b: &[u8]) -> bool {
    let mut i = 0;
    while i < b.len() {
        match decode_at(b, i) {
            Some((_, w)) => i += w,
            None => return false,
        }
    }
    true
}

/// Count scalar values in a valid buffer; `None` when malformed.
pub fn count_chars(b: &[u8]) -> Option<usize> {
    // ASCII chunk fast path (portable SWAR); the SIMD paths below are
    // equivalent accelerations of this same rule.
    let mut i = 0;
    let mut n = 0usize;
    while i + 8 <= b.len() {
        let chunk = b[i..i + 8].try_into().expect("chunk");
        let word = u64::from_le_bytes(chunk);
        if word & 0x8080_8080_8080_8080 == 0 {
            n += 8;
            i += 8;
        } else {
            break;
        }
    }
    while i < b.len() {
        let (_, w) = decode_at(b, i)?;
        n += 1;
        i += w;
    }
    #[cfg(target_arch = "aarch64")]
    {
        // NEON: count ASCII bytes 16 at a time ahead of the scalar tail.
        return count_chars_simd(b, n, i);
    }
    #[cfg(target_arch = "x86_64")]
    {
        return count_chars_simd(b, n, i);
    }
    #[allow(unreachable_code)]
    Some(n)
}

/// Decode one scalar at `i`; `None` on malformed or truncated input.
pub fn decode_at(b: &[u8], i: usize) -> Option<(u32, usize)> {
    let x = *b.get(i)?;
    if x < 0x80 {
        return Some((u32::from(x), 1));
    }
    let (width, min, cp0) = if x & 0xE0 == 0xC0 {
        (2, 0x80u32, u32::from(x & 0x1F))
    } else if x & 0xF0 == 0xE0 {
        (3, 0x800, u32::from(x & 0x0F))
    } else if x & 0xF8 == 0xF0 {
        (4, 0x10000, u32::from(x & 0x07))
    } else {
        return None;
    };
    let mut cp = cp0;
    for k in 1..width {
        let y = *b.get(i + k)?;
        if y & 0xC0 != 0x80 {
            return None;
        }
        cp = (cp << 6) | u32::from(y & 0x3F);
    }
    // Overlong encodings, surrogates, and out-of-range are malformed.
    if cp < min || cp > 0x10FFFF || (0xD800..=0xDFFF).contains(&cp) {
        return None;
    }
    Some((cp, width))
}

/// Encode one scalar; `None` when the value is invalid or `out` is too
/// small.
pub fn encode(cp: u32, out: &mut [u8]) -> Option<usize> {
    let width = utf8_width(cp)?;
    if out.len() < width {
        return None;
    }
    match width {
        1 => out[0] = cp as u8,
        2 => {
            out[0] = 0xC0 | (cp >> 6) as u8;
            out[1] = 0x80 | (cp & 0x3F) as u8;
        }
        3 => {
            out[0] = 0xE0 | (cp >> 12) as u8;
            out[1] = 0x80 | ((cp >> 6) & 0x3F) as u8;
            out[2] = 0x80 | (cp & 0x3F) as u8;
        }
        _ => {
            out[0] = 0xF0 | (cp >> 18) as u8;
            out[1] = 0x80 | ((cp >> 12) & 0x3F) as u8;
            out[2] = 0x80 | ((cp >> 6) & 0x3F) as u8;
            out[3] = 0x80 | (cp & 0x3F) as u8;
        }
    }
    Some(width)
}

/// The encoded width of a scalar value, or `None` when it is not a
/// Unicode scalar value.
pub fn utf8_width(cp: u32) -> Option<usize> {
    if cp > 0x10FFFF || (0xD800..=0xDFFF).contains(&cp) {
        return None;
    }
    Some(match cp {
        0..=0x7F => 1,
        0x80..=0x7FF => 2,
        0x800..=0xFFFF => 3,
        _ => 4,
    })
}

// SIMD acceleration. The chunk paths are strict accelerations of the
// scalar rules above: an ASCII chunk contributes 8/16 bytes and one
// scalar value each; anything else falls back to the scalar decoder at
// the current position. Tests compare both paths directly.

#[cfg(target_arch = "x86_64")]
fn count_chars_simd(b: &[u8], mut n: usize, mut i: usize) -> Option<usize> {
    // SSE2 is baseline on x86_64; AVX2 is runtime-detected.
    unsafe {
        if is_x86_feature_detected!("avx2") {
            return count_chars_avx2(b, n, i);
        }
        use std::arch::x86_64::*;
        while i + 16 <= b.len() {
            let v = _mm_loadu_si128(b.as_ptr().add(i) as *const __m128i);
            if _mm_movemask_epi8(v) == 0 {
                n += 16;
                i += 16;
            } else {
                break;
            }
        }
    }
    while i < b.len() {
        let (_, w) = decode_at(b, i)?;
        n += 1;
        i += w;
    }
    Some(n)
}

#[cfg(target_arch = "x86_64")]
unsafe fn count_chars_avx2(b: &[u8], mut n: usize, mut i: usize) -> Option<usize> {
    unsafe {
        use std::arch::x86_64::*;
        while i + 32 <= b.len() {
            let v = _mm256_loadu_si256(b.as_ptr().add(i) as *const __m256i);
            if _mm256_movemask_epi8(v) == 0 {
                n += 32;
                i += 32;
            } else {
                break;
            }
        }
    }
    while i < b.len() {
        let (_, w) = decode_at(b, i)?;
        n += 1;
        i += w;
    }
    Some(n)
}

#[cfg(target_arch = "aarch64")]
fn count_chars_simd(b: &[u8], mut n: usize, mut i: usize) -> Option<usize> {
    unsafe {
        use std::arch::aarch64::*;
        while i + 16 <= b.len() {
            let v = vld1q_u8(b.as_ptr().add(i));
            if vmaxvq_u8(v) < 0x80 {
                n += 16;
                i += 16;
            } else {
                break;
            }
        }
    }
    while i < b.len() {
        let (_, w) = decode_at(b, i)?;
        n += 1;
        i += w;
    }
    Some(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive_validate(b: &[u8]) -> bool {
        let mut i = 0;
        while i < b.len() {
            let x = b[i];
            let (w, cp) = if x < 0x80 {
                (1usize, u32::from(x))
            } else if x & 0xE0 == 0xC0 {
                (2, u32::from(x & 0x1F))
            } else if x & 0xF0 == 0xE0 {
                (3, u32::from(x & 0x0F))
            } else if x & 0xF8 == 0xF0 {
                (4, u32::from(x & 0x07))
            } else {
                return false;
            };
            if i + w > b.len() {
                return false;
            }
            let mut cp = cp;
            for k in 1..w {
                let y = b[i + k];
                if y & 0xC0 != 0x80 {
                    return false;
                }
                cp = (cp << 6) | u32::from(y & 0x3F);
            }
            if cp < 0x80 && w > 1
                || cp < 0x800 && w > 2
                || cp < 0x10000 && w > 3
                || cp > 0x10FFFF
                || (0xD800..=0xDFFF).contains(&cp)
            {
                return false;
            }
            i += w;
        }
        true
    }

    #[test]
    fn simd_and_naive_agree_on_the_mixed_corpus() {
        // ASCII, multibyte sequences, overlongs, surrogates, truncation
        // at every split point.
        let cases: Vec<Vec<u8>> = vec![
            vec![],
            b"hello".to_vec(),
            vec![0x00, 0x7F],
            "héllo wörld".as_bytes().to_vec(),
            "日本語テキスト".as_bytes().to_vec(),
            "a\u{1F600}b".as_bytes().to_vec(),
            vec![0xC0, 0x80],             // overlong NUL
            vec![0xED, 0xA0, 0x80],       // surrogate D800
            vec![0xF5, 0x80, 0x80, 0x80], // above 0x10FFFF
            vec![0xE0, 0x80, 0x80],       // overlong 3-byte
            vec![0xF0, 0x9F, 0x98],       // truncated 4-byte
            vec![0x80],                   // stray continuation
        ];
        let long_ascii: Vec<u8> = (0..1000u32).map(|i| (i % 95 + 32) as u8).collect();
        let long_mixed: Vec<u8> = "é".as_bytes()
            .iter()
            .cycle()
            .take(600)
            .copied()
            .collect();
        for mut c in cases {
            c.extend_from_slice(&long_ascii);
            c.extend_from_slice(&long_mixed);
            for split in [0usize, 1, 7, 8, 15, 16, 17, 31, 32, 33, c.len() / 2] {
                let sub = &c[..c.len().min(split + 400)];
                assert_eq!(validate(sub), naive_validate(sub), "validate split {split}");
                let expect = if naive_validate(sub) {
                    let mut n = 0;
                    let mut i = 0;
                    while i < sub.len() {
                        let (_, w) = decode_at(sub, i).expect("valid");
                        n += 1;
                        i += w;
                    }
                    Some(n)
                } else {
                    None
                };
                assert_eq!(count_chars(sub), expect, "count split {split}");
            }
        }
    }

    #[test]
    fn decode_encode_round_trip_at_boundaries() {
        for cp in [0u32, 0x7F, 0x80, 0x7FF, 0x800, 0xFFFF, 0x10000, 0x10FFFF] {
            let mut buf = [0u8; 4];
            let w = encode(cp, &mut buf).expect("encodes");
            assert_eq!(decode_at(&buf[..w], 0), Some((cp, w)));
            assert_eq!(count_chars(&buf[..w]), Some(1));
        }
        assert_eq!(encode(0xD800, &mut [0u8; 4]), None);
        assert_eq!(encode(0x110000, &mut [0u8; 4]), None);
        assert_eq!(encode(65, &mut [0u8; 0]), None);
    }
}
