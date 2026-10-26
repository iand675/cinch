//! Prelude library tests: the `Bits` class and the `String` reference
//! semantics (the pure UTF-8 rules the runtime's SIMD paths accelerate).

use safelang::driver::{check_sources, run_source};

fn expect_ok(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_err(name: &str, src: &str, needle: &str) {
    match check_sources(src, "<test>") {
        Ok(_) => panic!("{name}: expected an error containing {needle:?}"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains(needle),
                "{name}: error {msg:?} does not contain {needle:?}"
            );
        }
    }
}

#[test]
fn bits_class_machine_operations() {
    expect_ok(
        "bits-ops",
        "module Main where\n\
         main :: Int\n\
         main =\n\
         \x20 let x = set-bit (set-bit (0 :: U64) 3) 65 in\n\
         \x20 let y = count-ones x in\n\
         \x20 let z = rotate-left x 1 in\n\
         \x20 if eq-u32 y 2 then\n\
         \x20   if test-bit z 4 then\n\
         \x20     if eq-u64 (clear-bit x 3) (set-bit (0 :: U64) 1) then\n\
         \x20       if eq-u32 (leading-zeros (0 :: U64)) 64 then 0 else 4\n\
         \x20     else 3\n\
         \x20   else 2\n\
         \x20 else 1\n",
        0,
    );
}

#[test]
fn bits_class_is_generic() {
    // One definition, used at two instances through dictionary passing.
    expect_ok(
        "bits-generic",
        "module Main where\n\
         \x20 maskCount :: forall a. using (bits :: Bits a) => a -> a -> U32\n\
         \x20 maskCount x m = count-ones (bit-and x m)\n\
         \x20 main :: Int\n\
         \x20 main =\n\
         \x20   let a = maskCount (255 :: U8) (bit-not (0 :: U8)) in\n\
         \x20   let b = maskCount (3 :: U64) (bit-not (0 :: U64)) in\n\
         \x20   if eq-u32 a 8 then (if eq-u32 b 2 then 0 else 2) else 1\n",
        0,
    );
}

#[test]
fn bits_shift_and_complement_laws() {
    expect_ok(
        "bits-laws",
        "module Main where\n\
         main :: Int\n\
         main =\n\
         \x20 let x = 201 :: U32 in\n\
         \x20 let y = complement-bit x 0 in\n\
         \x20 let s = shift-left x 1 in\n\
         \x20 if eq-u32 y 200 then\n\
         \x20   if eq-u32 s 402 then\n\
         \x20     if eq-u32 (count-ones (bit-xor x x)) 0 then 0 else 3\n\
         \x20   else 2\n\
         \x20 else 1\n",
        0,
    );
}

#[test]
fn string_from_bytes_validates_utf8() {
    expect_ok(
        "string-valid",
        "module Main where\n\
         main :: Int\n\
         main =\n\
         \x20 let b = bytes-new 6 in\n\
         \x20 let u0 = bytes-set b 0 104 in\n\
         \x20 let u1 = bytes-set b 1 195 in\n\
         \x20 let u2 = bytes-set b 2 169 in\n\
         \x20 let u3 = bytes-set b 3 108 in\n\
         \x20 let u4 = bytes-set b 4 108 in\n\
         \x20 let u5 = bytes-set b 5 111 in\n\
         \x20 case string-from-bytes b of\n\
         \x20   None -> 1\n\
         \x20   Some s ->\n\
         \x20     if nat-cmp (string-byte-length s) 6 == 0 then\n\
         \x20       if nat-cmp (string-length s) 5 == 0 then 0 else 3\n\
         \x20     else 2\n",
        0,
    );
}

#[test]
fn string_rejects_malformed_utf8() {
    expect_ok(
        "string-invalid",
        "module Main where\n\
         main :: Int\n\
         main =\n\
         \x20 let b = bytes-new 2 in\n\
         \x20 let u0 = bytes-set b 0 192 in\n\
         \x20 let u1 = bytes-set b 1 128 in\n\
         \x20 case string-from-bytes b of\n\
         \x20   None -> 0\n\
         \x20   Some s -> 1\n",
        0,
    );
}

#[test]
fn string_char_at_sees_code_points() {
    expect_ok(
        "string-charat",
        "module Main where\n\
         main :: Int\n\
         main =\n\
         \x20 let b = bytes-new 4 in\n\
         \x20 let u0 = bytes-set b 0 104 in\n\
         \x20 let u1 = bytes-set b 1 195 in\n\
         \x20 let u2 = bytes-set b 2 169 in\n\
         \x20 let u3 = bytes-set b 3 105 in\n\
         \x20 case string-from-bytes b of\n\
         \x20   None -> 1\n\
         \x20   Some s -> case string-char-at s 1 of\n\
         \x20     None -> 2\n\
         \x20     Some c ->\n\
         \x20       if eq-u32 (char-to-u32 c) 233 then\n\
         \x20         case string-char-at s 2 of\n\
         \x20           None -> 3\n\
         \x20           Some d -> if eq-u32 (char-to-u32 d) 105 then 0 else 4\n\
         \x20       else 5\n",
        0,
    );
}

#[test]
fn string_concat_slice_and_order() {
    expect_ok(
        "string-ops",
        "module Main where\n\
         main :: Int\n\
         main =\n\
         \x20 let b1 = bytes-new 2 in\n\
         \x20 let a0 = bytes-set b1 0 97 in\n\
         \x20 let a1 = bytes-set b1 1 98 in\n\
         \x20 case string-from-bytes b1 of\n\
         \x20   None -> 1\n\
         \x20   Some x -> case string-from-bytes b1 of\n\
         \x20     None -> 2\n\
         \x20     Some y ->\n\
         \x20       let z = string-concat x y in\n\
         \x20       let w = string-slice z 1 3 in\n\
         \x20       if nat-cmp (string-byte-length z) 4 == 0 then\n\
         \x20         if nat-cmp (string-length w) 2 == 0 then\n\
         \x20           if string-compare x y == 0 then 0 else 4\n\
         \x20         else 3\n\
         \x20       else 2\n",
        0,
    );
}

#[test]
fn string_nat_and_int_formatting() {
    expect_ok(
        "string-format",
        "module Main where\n\
         main :: Int\n\
         main =\n\
         \x20 let s = nat-to-string 12345 in\n\
         \x20 let t = int-to-string (-42) in\n\
         \x20 let u = int-to-string 1000000000000000000000000 in\n\
         \x20 if nat-cmp (string-length s) 5 == 0 then\n\
         \x20   case string-char-at s 0 of\n\
         \x20     None -> 1\n\
         \x20     Some c ->\n\
         \x20       if eq-u32 (char-to-u32 c) 49 then\n\
         \x20         if nat-cmp (string-length t) 3 == 0 then\n\
         \x20           if nat-cmp (string-length u) 25 == 0 then 0 else 4\n\
         \x20         else 3\n\
         \x20       else 2\n\
         \x20 else 1\n",
        0,
    );
}

#[test]
fn string_equality_instance() {
    expect_ok(
        "string-eq",
        "module Main where\n\
         main :: Int\n\
         main =\n\
         \x20 let s = nat-to-string 7 in\n\
         \x20 let t = nat-to-string 7 in\n\
         \x20 let u = nat-to-string 8 in\n\
         \x20 if s == t then (if s /= u then 0 else 2) else 1\n",
        0,
    );
}

#[test]
fn bitwise_operators_and_rust_precedence() {
    // Rust order: * / % bind tighter than + -, which bind tighter than
    // << >>, then &, ^, |, then comparisons. Arithmetic and bitwise
    // operators are checked in their own type families (machine
    // integers have no arithmetic operator instances by design, spec
    // section 8): `1 + 7 % 4` = `1 + (7 % 4)` = 4 at Nat; `^` is XOR, so
    // `1 | 2 ^ 3` = `1 | (2 xor 3)` = 1 (wrong precedence would give
    // `(1 | 2) xor 3` = 0), `5 << 1 & 3` = `(5 << 1) & 3` = 2, and
    // `6 & 3 == 2` = `(6 & 3) == 2` at U32.
    expect_ok(
        "ops-precedence",
        "module Main where\n\
         main :: Int\n\
         main =\n\
         \x20 let a = (1 :: Nat) + 7 % 4 in\n\
         \x20 let b = (6 :: U32) & 3 == 2 in\n\
         \x20 let c = (1 :: U32) | 2 ^ 3 in\n\
         \x20 let d = (5 :: U32) << 1 & 3 in\n\
         \x20 if nat-cmp a 4 == 0 then\n\
         \x20   if b then\n\
         \x20     if eq-u32 c 1 then\n\
         \x20       if eq-u32 d 2 then 0 else 4\n\
         \x20     else 3\n\
         \x20   else 2\n\
         \x20 else 1\n",
        0,
    );
}

#[test]
fn bitwise_operators_on_machine_ints() {
    expect_ok(
        "ops-machine",
        "module Main where\n\
         main :: Int\n\
         main =\n\
         \x20 let x = 201 :: U32 in\n\
         \x20 let y = x & 15 in\n\
         \x20 let z = x | 14 in\n\
         \x20 let w = x ^ 255 in\n\
         \x20 let s = x >> 2 in\n\
         \x20 let t = (1 :: U32) << 5 in\n\
         \x20 if eq-u32 y 9 then\n\
         \x20   if eq-u32 z 207 then\n\
         \x20     if eq-u32 w 54 then\n\
         \x20       if eq-u32 s 50 then (if eq-u32 t 32 then 0 else 5) else 4\n\
         \x20     else 3\n\
         \x20   else 2\n\
         \x20 else 1\n",
        0,
    );
}

#[test]
fn modulo_and_division_on_math_types() {
    // Nat subtraction faults below zero, so the checks stay in Nat-safe
    // territory; division and remainder by zero are defined faults (72),
    // tested separately at the boundary.
    expect_ok(
        "ops-moddiv",
        "module Main where\n\
         main :: Int\n\
         main =\n\
         \x20 let a = (17 :: Nat) % 5 in\n\
         \x20 let b = (17 :: Nat) / 5 in\n\
         \x20 let c = (-17 :: Int) % 5 in\n\
         \x20 let d = (-17 :: Int) / 5 in\n\
         \x20 if nat-cmp a 2 == 0 then\n\
         \x20   if nat-cmp b 3 == 0 then\n\
         \x20     if int-cmp c (-2) == 0 then\n\
         \x20       if int-cmp d (-3) == 0 then 0 else 4\n\
         \x20     else 3\n\
         \x20   else 2\n\
         \x20 else 1\n",
        0,
    );
}

#[test]
fn string_library_type_errors_rejected() {
    expect_err(
        "string-type-error",
        "module Main where\n\
         main :: Int\n\
         main =\n\
         \x20 let s = nat-to-string 7 in\n\
         \x20 if eq-u32 s 7 then 0 else 1\n",
        "expected",
    );
}
