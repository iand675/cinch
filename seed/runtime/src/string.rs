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

#[cfg(not(test))]
unsafe extern "C" {
    pub fn cinch_utf8_validate(ptr: *const u8, len: i64) -> i64;
    pub fn cinch_utf8_count_chars(ptr: *const u8, len: i64) -> i64;
    pub fn cinch_utf8_decode_at(ptr: *const u8, len: i64, index: i64) -> i64;
    pub fn cinch_utf8_encode(cp: i64, out: *mut u8, cap: i64) -> i64;
}

/// Validate a UTF-8 buffer: 1 = valid, 0 = malformed. Never faults on
/// malformed content.
#[cfg(test)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cinch_utf8_validate(ptr: *const u8, len: i64) -> i64 {
    match unsafe { bytes_slice(ptr, len) } {
        Ok(b) => i64::from(validate(b)),
        Err(_) => 0,
    }
}

/// Count Unicode scalar values in a valid UTF-8 buffer, or -1 when the
/// buffer is malformed (the count is undefined then).
#[cfg(test)]
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
#[cfg(test)]
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
#[cfg(test)]
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

/// Validate a UTF-8 buffer using SIMD lookup tables (Keiser & Lemire,
/// "Validating UTF-8 In Less Than One Instruction Per Byte", SPE 2021)
/// on AArch64 NEON and x86_64 AVX2/SSSE3, with a portable 64-bit SWAR
/// ASCII fast path elsewhere.
pub fn validate(b: &[u8]) -> bool {
    #[cfg(target_arch = "aarch64")]
    {
        return unsafe { validate_neon(b) };
    }
    #[cfg(target_arch = "x86_64")]
    {
        return validate_x86(b);
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    {
        validate_swar(b)
    }
}

/// Count Unicode scalar values in a valid UTF-8 buffer, or `None` when
/// malformed. Combines Keiser-Lemire SIMD validation with single-pass
/// non-continuation byte counting (`(byte as i8) > -65`).
pub fn count_chars(b: &[u8]) -> Option<usize> {
    #[cfg(target_arch = "aarch64")]
    {
        return unsafe { count_chars_neon(b) };
    }
    #[cfg(target_arch = "x86_64")]
    {
        return count_chars_x86(b);
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    {
        count_chars_swar(b)
    }
}

/// Portable 64-bit SWAR ASCII-skipping UTF-8 validation (used on targets
/// without NEON/SSSE3/AVX2 and cross-checked in tests).
#[cfg_attr(
    any(target_arch = "aarch64", target_arch = "x86_64"),
    allow(dead_code)
)]
fn validate_swar(b: &[u8]) -> bool {
    let mut i = 0usize;
    let len = b.len();
    while i < len {
        while i + 8 <= len {
            let word = u64::from_ne_bytes(b[i..i + 8].try_into().expect("8-byte chunk"));
            if word & 0x8080_8080_8080_8080 == 0 {
                i += 8;
            } else {
                break;
            }
        }
        if i >= len {
            break;
        }
        match decode_at(b, i) {
            Some((_, w)) => i += w,
            None => return false,
        }
    }
    true
}

/// Portable 64-bit SWAR ASCII-skipping scalar count (used on targets
/// without NEON/SSSE3/AVX2 and cross-checked in tests).
#[cfg_attr(
    any(target_arch = "aarch64", target_arch = "x86_64"),
    allow(dead_code)
)]
fn count_chars_swar(b: &[u8]) -> Option<usize> {
    let mut i = 0usize;
    let mut n = 0usize;
    let len = b.len();
    while i < len {
        while i + 8 <= len {
            let word = u64::from_ne_bytes(b[i..i + 8].try_into().expect("8-byte chunk"));
            if word & 0x8080_8080_8080_8080 == 0 {
                n += 8;
                i += 8;
            } else {
                break;
            }
        }
        if i >= len {
            break;
        }
        let (_, w) = decode_at(b, i)?;
        n += 1;
        i += w;
    }
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

// ---------------------------------------------------------------------------
// Keiser-Lemire lookup4 SIMD UTF-8 validation & scalar counting.
//
// Reference: John Keiser & Daniel Lemire, "Validating UTF-8 In Less Than One
// Instruction Per Byte", Software: Practice and Experience 51(5), 2021
// (https://lemire.me/blog/2020/10/20/ridiculously-fast-unicode-utf-8-validation/).
//
// Each consecutive byte pair (prev1, curr) is classified by intersecting three
// 16-byte nibble lookup tables:
//   sc = BYTE_1_HIGH[prev1 >> 4] & BYTE_1_LOW[prev1 & 0x0F] & BYTE_2_HIGH[curr >> 4]
// Bits 0..=6 encode unconditionally illegal pairs; bit 7 (TWO_CONTS) marks two
// consecutive continuation bytes and is XORed with `must23_80` (0x80 iff
// prev2 >= 0xE0 or prev3 >= 0xF0). Any nonzero lane in `sc ^ must23_80` signals
// malformed UTF-8.
//
// For valid UTF-8, every non-continuation byte (`(byte as i8) > -65`) starts
// exactly one Unicode scalar value, so `count_chars` validates and counts in a
// single SIMD pass without ever falling back to byte-by-byte decoding.
// ---------------------------------------------------------------------------

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
mod simd_lut {
    pub const TOO_SHORT: u8 = 1 << 0;  // 0x01: lead followed by non-continuation
    pub const TOO_LONG: u8 = 1 << 1;   // 0x02: ASCII followed by continuation
    pub const OVERLONG_3: u8 = 1 << 2; // 0x04: E0 followed by 80..9F
    pub const TOO_LARGE: u8 = 1 << 3;  // 0x08: F4..FF followed by 90..BF
    pub const SURROGATE: u8 = 1 << 4;  // 0x10: ED followed by A0..BF
    pub const OVERLONG_2: u8 = 1 << 5; // 0x20: C0..C1 followed by 80..BF
    pub const OVERLONG_4: u8 = 1 << 6; // 0x40: F0 or F5..FF followed by 80..8F
    pub const TWO_CONTS: u8 = 1 << 7;  // 0x80: continuation followed by continuation

    const CARRY: u8 = TOO_SHORT | TOO_LONG | TWO_CONTS;

    pub const BYTE_1_HIGH: [u8; 16] = [
        TOO_LONG,                           // 0x0_
        TOO_LONG,                           // 0x1_
        TOO_LONG,                           // 0x2_
        TOO_LONG,                           // 0x3_
        TOO_LONG,                           // 0x4_
        TOO_LONG,                           // 0x5_
        TOO_LONG,                           // 0x6_
        TOO_LONG,                           // 0x7_
        TWO_CONTS,                          // 0x8_
        TWO_CONTS,                          // 0x9_
        TWO_CONTS,                          // 0xA_
        TWO_CONTS,                          // 0xB_
        TOO_SHORT | OVERLONG_2,             // 0xC_
        TOO_SHORT,                          // 0xD_
        TOO_SHORT | OVERLONG_3 | SURROGATE, // 0xE_
        TOO_SHORT | TOO_LARGE | OVERLONG_4, // 0xF_
    ];

    pub const BYTE_1_LOW: [u8; 16] = [
        CARRY | OVERLONG_2 | OVERLONG_3 | OVERLONG_4, // _0: C0, E0, F0
        CARRY | OVERLONG_2,                           // _1: C1
        CARRY,                                        // _2
        CARRY,                                        // _3
        CARRY | TOO_LARGE,                            // _4: F4
        CARRY | TOO_LARGE | OVERLONG_4,               // _5: F5
        CARRY | TOO_LARGE | OVERLONG_4,               // _6: F6
        CARRY | TOO_LARGE | OVERLONG_4,               // _7: F7
        CARRY | TOO_LARGE | OVERLONG_4,               // _8: F8
        CARRY | TOO_LARGE | OVERLONG_4,               // _9: F9
        CARRY | TOO_LARGE | OVERLONG_4,               // _A: FA
        CARRY | TOO_LARGE | OVERLONG_4,               // _B: FB
        CARRY | TOO_LARGE | OVERLONG_4,               // _C: FC
        CARRY | SURROGATE | TOO_LARGE | OVERLONG_4,   // _D: ED, FD
        CARRY | TOO_LARGE | OVERLONG_4,               // _E: FE
        CARRY | TOO_LARGE | OVERLONG_4,               // _F: FF
    ];

    pub const BYTE_2_HIGH: [u8; 16] = [
        TOO_SHORT,                                                   // 0x0_
        TOO_SHORT,                                                   // 0x1_
        TOO_SHORT,                                                   // 0x2_
        TOO_SHORT,                                                   // 0x3_
        TOO_SHORT,                                                   // 0x4_
        TOO_SHORT,                                                   // 0x5_
        TOO_SHORT,                                                   // 0x6_
        TOO_SHORT,                                                   // 0x7_
        TOO_LONG | OVERLONG_2 | TWO_CONTS | OVERLONG_3 | OVERLONG_4, // 0x8_
        TOO_LONG | OVERLONG_2 | TWO_CONTS | OVERLONG_3 | TOO_LARGE,  // 0x9_
        TOO_LONG | OVERLONG_2 | TWO_CONTS | SURROGATE | TOO_LARGE,   // 0xA_
        TOO_LONG | OVERLONG_2 | TWO_CONTS | SURROGATE | TOO_LARGE,   // 0xB_
        TOO_SHORT,                                                   // 0xC_
        TOO_SHORT,                                                   // 0xD_
        TOO_SHORT,                                                   // 0xE_
        TOO_SHORT,                                                   // 0xF_
    ];

    /// Thresholds for detecting an incomplete multi-byte sequence at the end
    /// of a 16-byte block: lane 13 >= 0xF0, lane 14 >= 0xE0, lane 15 >= 0xC0.
    pub const INCOMPLETE_16: [u8; 16] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xEF, 0xDF, 0xBF,
    ];
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
unsafe fn neon_check_block(
    curr: std::arch::aarch64::uint8x16_t,
    prev_input: &mut std::arch::aarch64::uint8x16_t,
    error: &mut std::arch::aarch64::uint8x16_t,
    t1: std::arch::aarch64::uint8x16_t,
    t2: std::arch::aarch64::uint8x16_t,
    t3: std::arch::aarch64::uint8x16_t,
) {
    use std::arch::aarch64::*;
    unsafe {
        let prev1 = vextq_u8::<15>(*prev_input, curr);
        let prev2 = vextq_u8::<14>(*prev_input, curr);
        let prev3 = vextq_u8::<13>(*prev_input, curr);

        let b1_hi = vqtbl1q_u8(t1, vshrq_n_u8::<4>(prev1));
        let b1_lo = vqtbl1q_u8(t2, vandq_u8(prev1, vdupq_n_u8(0x0F)));
        let b2_hi = vqtbl1q_u8(t3, vshrq_n_u8::<4>(curr));
        let sc = vandq_u8(vandq_u8(b1_hi, b1_lo), b2_hi);

        let is_3rd = vqsubq_u8(prev2, vdupq_n_u8(0xDF));
        let is_4th = vqsubq_u8(prev3, vdupq_n_u8(0xEF));
        let must23 = vorrq_u8(is_3rd, is_4th);
        let must23_80 = vandq_u8(vtstq_u8(must23, must23), vdupq_n_u8(0x80));

        *error = vorrq_u8(*error, veorq_u8(sc, must23_80));
        *prev_input = curr;
    }
}

#[cfg(target_arch = "aarch64")]
unsafe fn validate_neon(b: &[u8]) -> bool {
    use std::arch::aarch64::*;
    let len = b.len();
    if len == 0 {
        return true;
    }
    unsafe {
        let ptr = b.as_ptr();
        let zero = vdupq_n_u8(0);
        let t1 = vld1q_u8(simd_lut::BYTE_1_HIGH.as_ptr());
        let t2 = vld1q_u8(simd_lut::BYTE_1_LOW.as_ptr());
        let t3 = vld1q_u8(simd_lut::BYTE_2_HIGH.as_ptr());
        let inc_thresh = vld1q_u8(simd_lut::INCOMPLETE_16.as_ptr());

        let mut prev_input = zero;
        let mut prev_incomplete = zero;
        let mut error = zero;
        let mut i = 0usize;
        let mut check_countdown = 64u32;

        while i + 64 <= len {
            let v0 = vld1q_u8(ptr.add(i));
            let v1 = vld1q_u8(ptr.add(i + 16));
            let v2 = vld1q_u8(ptr.add(i + 32));
            let v3 = vld1q_u8(ptr.add(i + 48));
            let or_all = vorrq_u8(vorrq_u8(v0, v1), vorrq_u8(v2, v3));
            if vmaxvq_u8(or_all) < 0x80 {
                error = vorrq_u8(error, prev_incomplete);
                prev_incomplete = zero;
                prev_input = zero;
            } else {
                neon_check_block(v0, &mut prev_input, &mut error, t1, t2, t3);
                neon_check_block(v1, &mut prev_input, &mut error, t1, t2, t3);
                neon_check_block(v2, &mut prev_input, &mut error, t1, t2, t3);
                neon_check_block(v3, &mut prev_input, &mut error, t1, t2, t3);
                prev_incomplete = vqsubq_u8(v3, inc_thresh);
            }
            i += 64;
            check_countdown -= 1;
            if check_countdown == 0 {
                if vmaxvq_u8(error) != 0 {
                    return false;
                }
                check_countdown = 64;
            }
        }

        while i + 16 <= len {
            let v = vld1q_u8(ptr.add(i));
            if vmaxvq_u8(v) < 0x80 {
                error = vorrq_u8(error, prev_incomplete);
                prev_incomplete = zero;
                prev_input = zero;
            } else {
                neon_check_block(v, &mut prev_input, &mut error, t1, t2, t3);
                prev_incomplete = vqsubq_u8(v, inc_thresh);
            }
            i += 16;
        }

        let rem = len - i;
        if rem > 0 {
            let mut tail = [0u8; 16];
            std::ptr::copy_nonoverlapping(ptr.add(i), tail.as_mut_ptr(), rem);
            let v = vld1q_u8(tail.as_ptr());
            neon_check_block(v, &mut prev_input, &mut error, t1, t2, t3);
            prev_incomplete = vqsubq_u8(v, inc_thresh);
        }

        error = vorrq_u8(error, prev_incomplete);
        vmaxvq_u8(error) == 0
    }
}

#[cfg(target_arch = "aarch64")]
unsafe fn count_chars_neon(b: &[u8]) -> Option<usize> {
    use std::arch::aarch64::*;
    let len = b.len();
    if len == 0 {
        return Some(0);
    }
    unsafe {
        let ptr = b.as_ptr();
        let zero = vdupq_n_u8(0);
        let t1 = vld1q_u8(simd_lut::BYTE_1_HIGH.as_ptr());
        let t2 = vld1q_u8(simd_lut::BYTE_1_LOW.as_ptr());
        let t3 = vld1q_u8(simd_lut::BYTE_2_HIGH.as_ptr());
        let inc_thresh = vld1q_u8(simd_lut::INCOMPLETE_16.as_ptr());
        let cont_thresh = vdupq_n_s8(-65);

        let mut prev_input = zero;
        let mut prev_incomplete = zero;
        let mut error = zero;
        let mut total = 0usize;
        let mut acc = zero;
        let mut acc_blocks = 0u32;
        let mut i = 0usize;

        while i + 64 <= len {
            let v0 = vld1q_u8(ptr.add(i));
            let v1 = vld1q_u8(ptr.add(i + 16));
            let v2 = vld1q_u8(ptr.add(i + 32));
            let v3 = vld1q_u8(ptr.add(i + 48));
            let or_all = vorrq_u8(vorrq_u8(v0, v1), vorrq_u8(v2, v3));
            if vmaxvq_u8(or_all) < 0x80 {
                error = vorrq_u8(error, prev_incomplete);
                prev_incomplete = zero;
                prev_input = zero;
                total += 64;
            } else {
                neon_check_block(v0, &mut prev_input, &mut error, t1, t2, t3);
                neon_check_block(v1, &mut prev_input, &mut error, t1, t2, t3);
                neon_check_block(v2, &mut prev_input, &mut error, t1, t2, t3);
                neon_check_block(v3, &mut prev_input, &mut error, t1, t2, t3);
                prev_incomplete = vqsubq_u8(v3, inc_thresh);

                // vcgtq_s8 returns 0xFF (-1 mod 256) for non-continuation bytes;
                // subtracting 0xFF adds +1 per lane without needing negation.
                acc = vsubq_u8(acc, vcgtq_s8(vreinterpretq_s8_u8(v0), cont_thresh));
                acc = vsubq_u8(acc, vcgtq_s8(vreinterpretq_s8_u8(v1), cont_thresh));
                acc = vsubq_u8(acc, vcgtq_s8(vreinterpretq_s8_u8(v2), cont_thresh));
                acc = vsubq_u8(acc, vcgtq_s8(vreinterpretq_s8_u8(v3), cont_thresh));
                acc_blocks += 4;
                if acc_blocks >= 248 {
                    if vmaxvq_u8(error) != 0 {
                        return None;
                    }
                    total += usize::from(vaddlvq_u8(acc));
                    acc = zero;
                    acc_blocks = 0;
                }
            }
            i += 64;
        }

        if acc_blocks > 0 {
            total += usize::from(vaddlvq_u8(acc));
            acc = zero;
        }

        while i + 16 <= len {
            let v = vld1q_u8(ptr.add(i));
            if vmaxvq_u8(v) < 0x80 {
                error = vorrq_u8(error, prev_incomplete);
                prev_incomplete = zero;
                prev_input = zero;
                total += 16;
            } else {
                neon_check_block(v, &mut prev_input, &mut error, t1, t2, t3);
                prev_incomplete = vqsubq_u8(v, inc_thresh);
                acc = vsubq_u8(acc, vcgtq_s8(vreinterpretq_s8_u8(v), cont_thresh));
            }
            i += 16;
        }

        let rem = len - i;
        if rem > 0 {
            let mut tail = [0u8; 16];
            std::ptr::copy_nonoverlapping(ptr.add(i), tail.as_mut_ptr(), rem);
            let v = vld1q_u8(tail.as_ptr());
            neon_check_block(v, &mut prev_input, &mut error, t1, t2, t3);
            prev_incomplete = vqsubq_u8(v, inc_thresh);
            acc = vsubq_u8(acc, vcgtq_s8(vreinterpretq_s8_u8(v), cont_thresh));
            total += usize::from(vaddlvq_u8(acc));
            // Subtract the (16 - rem) zero padding bytes, each of which is ASCII.
            total -= 16 - rem;
        } else {
            total += usize::from(vaddlvq_u8(acc));
        }

        error = vorrq_u8(error, prev_incomplete);
        if vmaxvq_u8(error) == 0 {
            Some(total)
        } else {
            None
        }
    }
}

#[cfg(target_arch = "x86_64")]
fn validate_x86(b: &[u8]) -> bool {
    if is_x86_feature_detected!("avx2") {
        return unsafe { validate_avx2(b) };
    }
    if is_x86_feature_detected!("ssse3") {
        return unsafe { validate_ssse3(b) };
    }
    validate_swar(b)
}

#[cfg(target_arch = "x86_64")]
fn count_chars_x86(b: &[u8]) -> Option<usize> {
    if is_x86_feature_detected!("avx2") {
        return unsafe { count_chars_avx2(b) };
    }
    if is_x86_feature_detected!("ssse3") {
        return unsafe { count_chars_ssse3(b) };
    }
    count_chars_swar(b)
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "ssse3")]
#[inline]
unsafe fn ssse3_check_block(
    curr: std::arch::x86_64::__m128i,
    prev_input: &mut std::arch::x86_64::__m128i,
    error: &mut std::arch::x86_64::__m128i,
    t1: std::arch::x86_64::__m128i,
    t2: std::arch::x86_64::__m128i,
    t3: std::arch::x86_64::__m128i,
) {
    use std::arch::x86_64::*;
    let mask_0f = _mm_set1_epi8(0x0F);
    let prev1 = _mm_alignr_epi8::<15>(curr, *prev_input);
    let prev2 = _mm_alignr_epi8::<14>(curr, *prev_input);
    let prev3 = _mm_alignr_epi8::<13>(curr, *prev_input);

    let prev1_hi = _mm_and_si128(_mm_srli_epi16::<4>(prev1), mask_0f);
    let prev1_lo = _mm_and_si128(prev1, mask_0f);
    let curr_hi = _mm_and_si128(_mm_srli_epi16::<4>(curr), mask_0f);

    let sc = _mm_and_si128(
        _mm_and_si128(_mm_shuffle_epi8(t1, prev1_hi), _mm_shuffle_epi8(t2, prev1_lo)),
        _mm_shuffle_epi8(t3, curr_hi),
    );

    let is_3rd = _mm_subs_epu8(prev2, _mm_set1_epi8(0xDFu8 as i8));
    let is_4th = _mm_subs_epu8(prev3, _mm_set1_epi8(0xEFu8 as i8));
    let must23 = _mm_or_si128(is_3rd, is_4th);
    let must23_80 = _mm_and_si128(
        _mm_cmpgt_epi8(must23, _mm_setzero_si128()),
        _mm_set1_epi8(0x80u8 as i8),
    );

    *error = _mm_or_si128(*error, _mm_xor_si128(sc, must23_80));
    *prev_input = curr;
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "ssse3")]
unsafe fn validate_ssse3(b: &[u8]) -> bool {
    use std::arch::x86_64::*;
    let len = b.len();
    if len == 0 {
        return true;
    }
    let ptr = b.as_ptr();
    let zero = _mm_setzero_si128();
    let t1 = _mm_loadu_si128(simd_lut::BYTE_1_HIGH.as_ptr() as *const __m128i);
    let t2 = _mm_loadu_si128(simd_lut::BYTE_1_LOW.as_ptr() as *const __m128i);
    let t3 = _mm_loadu_si128(simd_lut::BYTE_2_HIGH.as_ptr() as *const __m128i);
    let inc_thresh = _mm_loadu_si128(simd_lut::INCOMPLETE_16.as_ptr() as *const __m128i);

    let mut prev_input = zero;
    let mut prev_incomplete = zero;
    let mut error = zero;
    let mut i = 0usize;

    while i + 16 <= len {
        let v = _mm_loadu_si128(ptr.add(i) as *const __m128i);
        if _mm_movemask_epi8(v) == 0 {
            error = _mm_or_si128(error, prev_incomplete);
            prev_incomplete = zero;
            prev_input = zero;
        } else {
            ssse3_check_block(v, &mut prev_input, &mut error, t1, t2, t3);
            prev_incomplete = _mm_subs_epu8(v, inc_thresh);
        }
        i += 16;
    }

    let rem = len - i;
    if rem > 0 {
        let mut tail = [0u8; 16];
        std::ptr::copy_nonoverlapping(ptr.add(i), tail.as_mut_ptr(), rem);
        let v = _mm_loadu_si128(tail.as_ptr() as *const __m128i);
        ssse3_check_block(v, &mut prev_input, &mut error, t1, t2, t3);
        prev_incomplete = _mm_subs_epu8(v, inc_thresh);
    }

    error = _mm_or_si128(error, prev_incomplete);
    _mm_movemask_epi8(_mm_cmpeq_epi8(error, zero)) == 0xFFFF
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "ssse3")]
unsafe fn count_chars_ssse3(b: &[u8]) -> Option<usize> {
    use std::arch::x86_64::*;
    let len = b.len();
    if len == 0 {
        return Some(0);
    }
    let ptr = b.as_ptr();
    let zero = _mm_setzero_si128();
    let t1 = _mm_loadu_si128(simd_lut::BYTE_1_HIGH.as_ptr() as *const __m128i);
    let t2 = _mm_loadu_si128(simd_lut::BYTE_1_LOW.as_ptr() as *const __m128i);
    let t3 = _mm_loadu_si128(simd_lut::BYTE_2_HIGH.as_ptr() as *const __m128i);
    let inc_thresh = _mm_loadu_si128(simd_lut::INCOMPLETE_16.as_ptr() as *const __m128i);
    let cont_thresh = _mm_set1_epi8(-65);

    let mut prev_input = zero;
    let mut prev_incomplete = zero;
    let mut error = zero;
    let mut total = 0usize;
    let mut i = 0usize;

    while i + 16 <= len {
        let v = _mm_loadu_si128(ptr.add(i) as *const __m128i);
        if _mm_movemask_epi8(v) == 0 {
            error = _mm_or_si128(error, prev_incomplete);
            prev_incomplete = zero;
            prev_input = zero;
            total += 16;
        } else {
            ssse3_check_block(v, &mut prev_input, &mut error, t1, t2, t3);
            prev_incomplete = _mm_subs_epu8(v, inc_thresh);
            let mask = _mm_movemask_epi8(_mm_cmpgt_epi8(v, cont_thresh)) as u32;
            total += mask.count_ones() as usize;
        }
        i += 16;
    }

    let rem = len - i;
    if rem > 0 {
        let mut tail = [0u8; 16];
        std::ptr::copy_nonoverlapping(ptr.add(i), tail.as_mut_ptr(), rem);
        let v = _mm_loadu_si128(tail.as_ptr() as *const __m128i);
        ssse3_check_block(v, &mut prev_input, &mut error, t1, t2, t3);
        prev_incomplete = _mm_subs_epu8(v, inc_thresh);
        let mask = _mm_movemask_epi8(_mm_cmpgt_epi8(v, cont_thresh)) as u32;
        total += mask.count_ones() as usize;
        total -= 16 - rem;
    }

    error = _mm_or_si128(error, prev_incomplete);
    if _mm_movemask_epi8(_mm_cmpeq_epi8(error, zero)) == 0xFFFF {
        Some(total)
    } else {
        None
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline]
unsafe fn avx2_check_block(
    curr: std::arch::x86_64::__m256i,
    prev_input: &mut std::arch::x86_64::__m256i,
    error: &mut std::arch::x86_64::__m256i,
    t1: std::arch::x86_64::__m256i,
    t2: std::arch::x86_64::__m256i,
    t3: std::arch::x86_64::__m256i,
) {
    use std::arch::x86_64::*;
    let mask_0f = _mm256_set1_epi8(0x0F);
    let prev_half = _mm256_permute2x128_si256::<0x21>(*prev_input, curr);
    let prev1 = _mm256_alignr_epi8::<15>(curr, prev_half);
    let prev2 = _mm256_alignr_epi8::<14>(curr, prev_half);
    let prev3 = _mm256_alignr_epi8::<13>(curr, prev_half);

    let prev1_hi = _mm256_and_si256(_mm256_srli_epi16::<4>(prev1), mask_0f);
    let prev1_lo = _mm256_and_si256(prev1, mask_0f);
    let curr_hi = _mm256_and_si256(_mm256_srli_epi16::<4>(curr), mask_0f);

    let sc = _mm256_and_si256(
        _mm256_and_si256(
            _mm256_shuffle_epi8(t1, prev1_hi),
            _mm256_shuffle_epi8(t2, prev1_lo),
        ),
        _mm256_shuffle_epi8(t3, curr_hi),
    );

    let is_3rd = _mm256_subs_epu8(prev2, _mm256_set1_epi8(0xDFu8 as i8));
    let is_4th = _mm256_subs_epu8(prev3, _mm256_set1_epi8(0xEFu8 as i8));
    let must23 = _mm256_or_si256(is_3rd, is_4th);
    let must23_80 = _mm256_and_si256(
        _mm256_cmpgt_epi8(must23, _mm256_setzero_si256()),
        _mm256_set1_epi8(0x80u8 as i8),
    );

    *error = _mm256_or_si256(*error, _mm256_xor_si256(sc, must23_80));
    *prev_input = curr;
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn validate_avx2(b: &[u8]) -> bool {
    use std::arch::x86_64::*;
    const INCOMPLETE_32: [u8; 32] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xEF, 0xDF, 0xBF,
    ];
    let len = b.len();
    if len == 0 {
        return true;
    }
    let ptr = b.as_ptr();
    let zero = _mm256_setzero_si256();
    let t1_128 = _mm_loadu_si128(simd_lut::BYTE_1_HIGH.as_ptr() as *const __m128i);
    let t2_128 = _mm_loadu_si128(simd_lut::BYTE_1_LOW.as_ptr() as *const __m128i);
    let t3_128 = _mm_loadu_si128(simd_lut::BYTE_2_HIGH.as_ptr() as *const __m128i);
    let t1 = _mm256_broadcastsi128_si256(t1_128);
    let t2 = _mm256_broadcastsi128_si256(t2_128);
    let t3 = _mm256_broadcastsi128_si256(t3_128);
    let inc_thresh = _mm256_loadu_si256(INCOMPLETE_32.as_ptr() as *const __m256i);

    let mut prev_input = zero;
    let mut prev_incomplete = zero;
    let mut error = zero;
    let mut i = 0usize;

    while i + 64 <= len {
        let v0 = _mm256_loadu_si256(ptr.add(i) as *const __m256i);
        let v1 = _mm256_loadu_si256(ptr.add(i + 32) as *const __m256i);
        if _mm256_movemask_epi8(_mm256_or_si256(v0, v1)) == 0 {
            error = _mm256_or_si256(error, prev_incomplete);
            prev_incomplete = zero;
            prev_input = zero;
        } else {
            avx2_check_block(v0, &mut prev_input, &mut error, t1, t2, t3);
            avx2_check_block(v1, &mut prev_input, &mut error, t1, t2, t3);
            prev_incomplete = _mm256_subs_epu8(v1, inc_thresh);
        }
        i += 64;
    }

    while i + 32 <= len {
        let v = _mm256_loadu_si256(ptr.add(i) as *const __m256i);
        if _mm256_movemask_epi8(v) == 0 {
            error = _mm256_or_si256(error, prev_incomplete);
            prev_incomplete = zero;
            prev_input = zero;
        } else {
            avx2_check_block(v, &mut prev_input, &mut error, t1, t2, t3);
            prev_incomplete = _mm256_subs_epu8(v, inc_thresh);
        }
        i += 32;
    }

    let rem = len - i;
    if rem > 0 {
        let mut tail = [0u8; 32];
        std::ptr::copy_nonoverlapping(ptr.add(i), tail.as_mut_ptr(), rem);
        let v = _mm256_loadu_si256(tail.as_ptr() as *const __m256i);
        avx2_check_block(v, &mut prev_input, &mut error, t1, t2, t3);
        prev_incomplete = _mm256_subs_epu8(v, inc_thresh);
    }

    error = _mm256_or_si256(error, prev_incomplete);
    _mm256_testz_si256(error, error) != 0
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn count_chars_avx2(b: &[u8]) -> Option<usize> {
    use std::arch::x86_64::*;
    const INCOMPLETE_32: [u8; 32] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xEF, 0xDF, 0xBF,
    ];
    let len = b.len();
    if len == 0 {
        return Some(0);
    }
    let ptr = b.as_ptr();
    let zero = _mm256_setzero_si256();
    let t1_128 = _mm_loadu_si128(simd_lut::BYTE_1_HIGH.as_ptr() as *const __m128i);
    let t2_128 = _mm_loadu_si128(simd_lut::BYTE_1_LOW.as_ptr() as *const __m128i);
    let t3_128 = _mm_loadu_si128(simd_lut::BYTE_2_HIGH.as_ptr() as *const __m128i);
    let t1 = _mm256_broadcastsi128_si256(t1_128);
    let t2 = _mm256_broadcastsi128_si256(t2_128);
    let t3 = _mm256_broadcastsi128_si256(t3_128);
    let inc_thresh = _mm256_loadu_si256(INCOMPLETE_32.as_ptr() as *const __m256i);
    let cont_thresh = _mm256_set1_epi8(-65);

    let mut prev_input = zero;
    let mut prev_incomplete = zero;
    let mut error = zero;
    let mut total = 0usize;
    let mut i = 0usize;

    while i + 64 <= len {
        let v0 = _mm256_loadu_si256(ptr.add(i) as *const __m256i);
        let v1 = _mm256_loadu_si256(ptr.add(i + 32) as *const __m256i);
        if _mm256_movemask_epi8(_mm256_or_si256(v0, v1)) == 0 {
            error = _mm256_or_si256(error, prev_incomplete);
            prev_incomplete = zero;
            prev_input = zero;
            total += 64;
        } else {
            avx2_check_block(v0, &mut prev_input, &mut error, t1, t2, t3);
            avx2_check_block(v1, &mut prev_input, &mut error, t1, t2, t3);
            prev_incomplete = _mm256_subs_epu8(v1, inc_thresh);
            let m0 = _mm256_movemask_epi8(_mm256_cmpgt_epi8(v0, cont_thresh)) as u32;
            let m1 = _mm256_movemask_epi8(_mm256_cmpgt_epi8(v1, cont_thresh)) as u32;
            total += (m0.count_ones() + m1.count_ones()) as usize;
        }
        i += 64;
    }

    while i + 32 <= len {
        let v = _mm256_loadu_si256(ptr.add(i) as *const __m256i);
        if _mm256_movemask_epi8(v) == 0 {
            error = _mm256_or_si256(error, prev_incomplete);
            prev_incomplete = zero;
            prev_input = zero;
            total += 32;
        } else {
            avx2_check_block(v, &mut prev_input, &mut error, t1, t2, t3);
            prev_incomplete = _mm256_subs_epu8(v, inc_thresh);
            let mask = _mm256_movemask_epi8(_mm256_cmpgt_epi8(v, cont_thresh)) as u32;
            total += mask.count_ones() as usize;
        }
        i += 32;
    }

    let rem = len - i;
    if rem > 0 {
        let mut tail = [0u8; 32];
        std::ptr::copy_nonoverlapping(ptr.add(i), tail.as_mut_ptr(), rem);
        let v = _mm256_loadu_si256(tail.as_ptr() as *const __m256i);
        avx2_check_block(v, &mut prev_input, &mut error, t1, t2, t3);
        prev_incomplete = _mm256_subs_epu8(v, inc_thresh);
        let mask = _mm256_movemask_epi8(_mm256_cmpgt_epi8(v, cont_thresh)) as u32;
        total += mask.count_ones() as usize;
        total -= 32 - rem;
    }

    error = _mm256_or_si256(error, prev_incomplete);
    if _mm256_testz_si256(error, error) != 0 {
        Some(total)
    } else {
        None
    }
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

    fn naive_count(b: &[u8]) -> Option<usize> {
        if !naive_validate(b) {
            return None;
        }
        let mut n = 0usize;
        let mut i = 0usize;
        while i < b.len() {
            let (_, w) = decode_at(b, i).expect("valid");
            n += 1;
            i += w;
        }
        Some(n)
    }

    fn assert_all_paths_agree(sub: &[u8], ctx: &str) {
        let expect_valid = naive_validate(sub);
        let expect_count = naive_count(sub);
        assert_eq!(validate(sub), expect_valid, "validate mismatch ({ctx})");
        assert_eq!(validate_swar(sub), expect_valid, "validate_swar mismatch ({ctx})");
        assert_eq!(count_chars(sub), expect_count, "count_chars mismatch ({ctx})");
        assert_eq!(
            count_chars_swar(sub),
            expect_count,
            "count_chars_swar mismatch ({ctx})"
        );
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
            for split in [0usize, 1, 7, 8, 15, 16, 17, 31, 32, 33, 63, 64, 65, c.len() / 2] {
                let sub = &c[..c.len().min(split + 400)];
                assert_all_paths_agree(sub, &format!("split {split}"));
            }
        }
    }

    #[test]
    fn exhaustive_all_1_and_2_byte_sequences() {
        for b0 in 0u8..=255 {
            assert_all_paths_agree(&[b0], &format!("1-byte {b0:#04x}"));
            for b1 in 0u8..=255 {
                let pair = [b0, b1];
                assert_all_paths_agree(&pair, &format!("2-byte [{b0:#04x}, {b1:#04x}]"));
            }
        }
    }

    #[test]
    fn exhaustive_boundary_3_and_4_byte_sequences_across_chunk_offsets() {
        let probes: &[&[u8]] = &[
            // Valid boundaries
            &[0x00],
            &[0x7F],
            &[0xC2, 0x80],
            &[0xDF, 0xBF],
            &[0xE0, 0xA0, 0x80],
            &[0xED, 0x9F, 0xBF],
            &[0xEE, 0x80, 0x80],
            &[0xEF, 0xBF, 0xBF],
            &[0xF0, 0x90, 0x80, 0x80],
            &[0xF3, 0xBF, 0xBF, 0xBF],
            &[0xF4, 0x8F, 0xBF, 0xBF],
            // Invalid boundaries: overlongs, surrogates, > U+10FFFF, stray continuations, truncations
            &[0x80],
            &[0xBF],
            &[0xC0, 0x80],
            &[0xC1, 0xBF],
            &[0xC2, 0x7F],
            &[0xC2, 0xC0],
            &[0xE0, 0x80, 0x80],
            &[0xE0, 0x9F, 0xBF],
            &[0xE0, 0xA0, 0x7F],
            &[0xED, 0xA0, 0x80],
            &[0xED, 0xBF, 0xBF],
            &[0xF0, 0x80, 0x80, 0x80],
            &[0xF0, 0x8F, 0xBF, 0xBF],
            &[0xF0, 0x90, 0x80, 0x7F],
            &[0xF4, 0x90, 0x80, 0x80],
            &[0xF5, 0x80, 0x80, 0x80],
            &[0xF8, 0x88, 0x80, 0x80],
            &[0xFE],
            &[0xFF],
            // Truncated leads
            &[0xC2],
            &[0xE2],
            &[0xE2, 0x82],
            &[0xF0],
            &[0xF0, 0x9F],
            &[0xF0, 0x9F, 0x98],
            // Extra continuation after valid 2/3/4-byte sequences
            &[0xC2, 0x80, 0x80],
            &[0xE2, 0x82, 0xAC, 0x80],
            &[0xF0, 0x9F, 0x98, 0x80, 0x80],
        ];

        // Test each probe at every prefix offset 0..=67, both at EOF and followed by 64 ASCII bytes
        // (to exercise the ASCII fast-path handoff after an incomplete or malformed sequence).
        for probe in probes {
            for prefix_len in 0..=67usize {
                let mut at_eof = vec![b'a'; prefix_len];
                at_eof.extend_from_slice(probe);
                assert_all_paths_agree(&at_eof, &format!("eof prefix={prefix_len} probe={probe:02x?}"));

                let mut with_ascii_suffix = at_eof.clone();
                with_ascii_suffix.extend_from_slice(&[b'z'; 64]);
                assert_all_paths_agree(
                    &with_ascii_suffix,
                    &format!("ascii-suffix prefix={prefix_len} probe={probe:02x?}"),
                );
            }
        }
    }

    #[test]
    fn large_buffers_exercise_accumulator_flush_and_early_exit() {
        // Exceed 248 * 16 = 3968 bytes to exercise multiple u8 lane-accumulator flushes
        // and the 64 * 64 = 4096-byte periodic error check.
        let unit = "héllo 日本語 🚀 ascii123 ".as_bytes();
        let mut buf = Vec::with_capacity(20_000);
        while buf.len() < 20_000 {
            buf.extend_from_slice(unit);
        }
        assert_all_paths_agree(&buf, "20k valid mixed");

        // Pure 4-byte emoji (maximum continuation density)
        let mut emoji = Vec::with_capacity(20_000);
        while emoji.len() < 20_000 {
            emoji.extend_from_slice("🚀".as_bytes());
        }
        assert_all_paths_agree(&emoji, "20k valid emoji");

        // Inject invalid byte at start, middle, and very end
        for pos in [0, 15, 16, 63, 64, 4095, 4096, 10_000, buf.len() - 1] {
            let mut bad = buf.clone();
            bad[pos] = 0xFF;
            assert_all_paths_agree(&bad, &format!("20k invalid at {pos}"));
        }
    }

    #[test]
    fn ffi_entry_points_handle_valid_malformed_and_empty_inputs() {
        unsafe {
            assert_eq!(cinch_utf8_validate(std::ptr::null(), 0), 1);
            assert_eq!(cinch_utf8_count_chars(std::ptr::null(), 0), 0);
            assert_eq!(cinch_utf8_validate(std::ptr::null(), 4), 0);
            assert_eq!(cinch_utf8_count_chars(std::ptr::null(), 4), -1);
            assert_eq!(cinch_utf8_validate(b"abc".as_ptr(), -1), 0);
            assert_eq!(cinch_utf8_count_chars(b"abc".as_ptr(), -1), -1);

            let ok = "héllo 日本語 🚀".as_bytes();
            assert_eq!(cinch_utf8_validate(ok.as_ptr(), ok.len() as i64), 1);
            assert_eq!(cinch_utf8_count_chars(ok.as_ptr(), ok.len() as i64), 11);

            let bad = b"hello\xED\xA0\x80world";
            assert_eq!(cinch_utf8_validate(bad.as_ptr(), bad.len() as i64), 0);
            assert_eq!(cinch_utf8_count_chars(bad.as_ptr(), bad.len() as i64), -1);
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
