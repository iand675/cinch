//! Results that lie in a known range by construction state it in their
//! types (`design/refinement-types.md`, "refined results"): a widening
//! conversion's `InRange lo hi`, `mask-i64`'s `[0, m]`, `char-code`'s
//! scalar range, `nat-rem`'s `r < d`, and the value-keeping `Nat`/`Int`/
//! `I64` conversions. The range reaches the difference-logic discharge as
//! a fact (a let-bound result's refined type, or an application's result
//! refinement inside an obligation), and where no range is demanded the
//! value is its number. `nat-from-i64`, `nat-to-i64` and `int-to-i64`
//! state the range they need, so a value out of range is a compile error
//! and the `checked-*` forms answer `None`.

mod common;

use cinch::driver::{check_sources, run_source};

fn expect_run(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_err(name: &str, src: &str, needles: &[&str]) {
    match check_sources(src, "<test>") {
        Ok(_) => panic!("{name}: expected an error containing {needles:?}, but the program checked"),
        Err(e) => {
            let msg = e.to_string();
            for n in needles {
                assert!(msg.contains(n), "{name}: error {msg:?} does not contain {n:?}");
            }
        }
    }
}

#[test]
fn a_widening_conversion_discharges_a_subtraction_and_nat_from_i64() {
    expect_run(
        "widening",
        r#"
module Main where
  -- `u8-to-i64`'s range is `nat-from-i64`'s evidence, and the result
  -- keeps the value, so it is at most 255 for the subtraction.
  below-three-hundred :: U8 -> Nat
  below-three-hundred b =
    let w = u8-to-i64 b in
    let n = nat-from-i64 w in
    300 - n

  -- Directly in an obligation: the application's result refinement is
  -- the fact.
  direct :: U32 -> Nat
  direct c = nat-from-i64 (u32-to-i64 c)

  main :: Int
  main =
    if below-three-hundred (wrapping-i64-to-u8 200) == 100 && direct (wrapping-i64-to-u32 7) == 7 && u16-to-nat (wrapping-i64-to-u16 65535) == 65535
    then 0 else 1
"#,
        0,
    );
}

#[test]
fn the_mask_bounds_its_result() {
    expect_run(
        "mask",
        r#"
module Main where
  low-byte :: I64 -> Nat
  low-byte x = let m = mask-i64 x 255 in 255 - nat-from-i64 m

  main :: Int
  main = if low-byte 513 == 254 && low-byte max-bound-i64 == 0 then 0 else 1
"#,
        0,
    );
    // `bit-and-i64` itself promises nothing: a negative mask is an
    // ordinary use of it.
    expect_err(
        "bit-and-promises-nothing",
        r#"
module Main where
  f :: I64 -> Nat
  f x = nat-from-i64 (bit-and-i64 x 255)

  main :: Int
  main = 0
"#,
        &["the obligation `0 <= "],
    );
    // The mask's own range needs a non-negative mask.
    expect_err(
        "negative-mask",
        r#"
module Main where
  f :: I64 -> I64
  f x = mask-i64 x min-bound-i64

  main :: Int
  main = 0
"#,
        &["0 <= "],
    );
}

#[test]
fn char_code_is_a_scalar_value() {
    expect_run(
        "char-code",
        r#"
module Main where
  headroom :: Char -> Nat
  headroom c = 1114111 - char-code c

  main :: Int
  main = if char-code 'A' == 65 && headroom 'A' == 1114046 then 0 else 1
"#,
        0,
    );
}

#[test]
fn a_refined_result_is_its_number_in_ordinary_use() {
    expect_run(
        "no-friction",
        r#"
module Main where
  plain :: Nat -> Nat
  plain n = n * 2

  -- Class methods, a plain parameter, a named primitive and a
  -- comparison: no `unrefine` anywhere.
  sum-codes :: Char -> Char -> Nat
  sum-codes a b =
    let x = char-code a in
    let y = char-code b in
    if x < y then plain x + nat-add y 1 else x - y

  -- A range-refined function used as a value yields its numbers.
  codes :: List Nat
  codes = map char-code ['a', 'b']

  main :: Int
  main = if sum-codes 'a' 'b' == 293 && sum-codes 'b' 'a' == 1 && list-length codes == 2 then 0 else 1
"#,
        0,
    );
}

#[test]
fn an_out_of_range_use_is_refused() {
    expect_err(
        "signed-source",
        r#"
module Main where
  f :: I32 -> Nat
  f x = nat-from-i64 (i32-to-i64 x)

  main :: Int
  main = 0
"#,
        &["the obligation `0 <= "],
    );
    expect_err(
        "range-too-wide",
        r#"
module Main where
  f :: U16 -> Nat
  f x = 255 - u16-to-nat x

  main :: Int
  main = 0
"#,
        &["<= 255"],
    );
    expect_err(
        "nat-to-i64-unbounded",
        r#"
module Main where
  f :: Nat -> I64
  f n = nat-to-i64 n

  main :: Int
  main = 0
"#,
        &["9223372036854775807"],
    );
    // A declaration may state only what an intrinsic promises.
    expect_err(
        "false-promise",
        r#"
module Main where
  foreign import ccall "cinch.prim.i32ToI64" w :: I32 -> InRange 0 5 I64

  main :: Int
  main = 0
"#,
        &["the intrinsic promises"],
    );
}

#[test]
fn the_conversions_are_total() {
    expect_run(
        "total",
        r#"
module Main where
  big :: Nat
  big = nat-mul 9223372036854775807 9223372036854775807

  is-none :: forall a. Option a -> Bool
  is-none o = case o of
    None -> True
    Some _ -> False

  main :: Int
  main =
    let a = is-none (checked-nat-from-i64 min-bound-i64) in
    let b = is-none (checked-nat-to-i64 big) in
    let c = is-none (checked-int-to-i64 (nat-to-int big)) in
    let d = nat-from-int (nat-to-int big) == big in
    if a && b && c && d then 0 else 1
"#,
        0,
    );
}

#[test]
fn hash_int_hashes_a_huge_int_without_converting() {
    expect_run(
        "hash-int",
        r#"
module Main where
  import Data.Hash

  big :: Nat
  big = nat-mul 9223372036854775807 1000

  main :: Int
  main =
    let i = nat-to-int big in
    if hash i == big && hash (0 - i) == big && hash (int-from-i64 min-bound-i64) == 9223372036854775808 then 0 else 1
"#,
        0,
    );
}

#[test]
fn an_intrinsic_given_a_huge_nat_answers_totally() {
    // A `Nat` count past `i64` clamps in the intrinsic unit: the buffer
    // no allocation can hold is the defined heap `Exhausted` exit (73),
    // never a conversion fault.
    expect_run(
        "huge-count",
        r#"
module Main where
  foreign import ccall "cinch.runtime.randomBytes" raw-random :: Nat -> Bytes

  main :: Int
  main = if bytes-len (raw-random (nat-mul 9223372036854775807 4)) == 0 then 0 else 1
"#,
        73,
    );
}

#[test]
fn a_cps_call_drops_its_argument_fact() {
    // Under a handler a call elaborates in continuation-passing style: it
    // still checks, and its result is a plain number.
    expect_run(
        "cps-checks",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Char

  main :: Int
  main = handle (if nat-to-int (char-code (poke ())) == 65 then 0 else 1) with (handler Poke where
    return x -> x
    poke u resume k -> k 'A')
"#,
        0,
    );
    // No range is assumed there: a subtraction that would need one is
    // refused (under a handler `-` at `Nat` is not applied directly, so
    // its obligation cannot even be stated).
    expect_err(
        "cps-no-fact",
        r#"
module Main where
  effect Poke where
    poke :: Unit ->{Poke} Char

  top :: Nat
  top = 1114111

  main :: Int
  main = handle nat-to-int (top - char-code (poke ())) with (handler Poke where
    return x -> x
    poke u resume k -> k 'A')
"#,
        &["`-` needs `arg2 <= arg1`"],
    );
    // Outside a handler the remainder carries `r < 7`.
    expect_run(
        "direct-fact",
        r#"
module Main where
  f :: Nat -> Nat
  f v = let r = nat-rem v 7 in 6 - r

  main :: Int
  main = if f 10 == 3 then 0 else 1
"#,
        0,
    );
}

#[test]
fn a_function_value_keeps_its_refined_result() {
    // A function flows into a function type covariantly in its result:
    // an alias at the full refined type keeps the range (and the result
    // naming its argument), the same function at a plain type or under a
    // type variable is its numbers, and the alias's result is evidence
    // at its call.
    expect_run(
        "function-value",
        r#"
module Main where
  import Data.Machine

  len :: Bytes -> InRange 0 9223372036854775807 Nat
  len = bytes-len

  widen :: (a0 :: I8) -> refined ((-128 <= r && r <= 127) && r == a0) I16
  widen = i8-to-i16

  plain :: I8 -> I16
  plain = i8-to-i16

  room :: U8 -> Nat
  room b =
    let f = u8-to-nat in
    let n = f b in
    300 - n

  main :: Int
  main =
    let ws = map i8-to-i16 [wrapping-i64-to-i8 5] in
    if len (bytes-new 3) == 3 && widen (wrapping-i64-to-i8 7) == plain (wrapping-i64-to-i8 7) && room (wrapping-i64-to-u8 255) == 45 && list-length ws == 1
    then 0 else 1
"#,
        0,
    );
}
