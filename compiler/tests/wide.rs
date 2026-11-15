//! The wide scalars `I128` and `U128` (`design/wide-scalars.md`): the
//! `Wide` representation class, 128-bit literals, and the operations of
//! `Data.Wide`. Phase 1 lands in increments: what the emitter lowers runs
//! here against real hardware, and every position it does not lower yet
//! is refused by name rather than miscompiled, which these tests pin too.

mod common;

use common::source_frontend::{check_sources, run_source};

fn run_ok(name: &str, src: &str) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, 0, "{name}: expected exit 0, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

/// Code generation refuses `src` with a message containing `needle`.
fn expect_refused(name: &str, src: &str, needle: &str) {
    let program = cinch::driver::check_sources(src, "<test>").unwrap_or_else(|e| panic!("{name}: check failed: {e}"));
    match cinch::codegen::emit(&program) {
        Ok(_) => panic!("{name}: expected code generation to refuse it"),
        Err(e) => {
            let msg = e.to_string();
            assert!(msg.contains(needle) && msg.contains("design/wide-scalars.md"), "{name}: unexpected refusal: {msg}");
        }
    }
}

fn program(ty: &str, literal: &str) -> String {
    format!(
        "module Main where\n  #[inline(never)]\n  ident :: {ty} -> {ty}\n  ident x = x\n\n  main :: Int\n  main = case ident ({literal}) of\n    _ -> 0\n"
    )
}

fn expect_checks(ty: &str, literal: &str) {
    if let Err(e) = check_sources(&program(ty, literal), "wide") {
        panic!("{ty} literal {literal} must check: {e}");
    }
}

fn expect_out_of_range(ty: &str, literal: &str) {
    match check_sources(&program(ty, literal), "wide") {
        Ok(_) => panic!("{ty} literal {literal} must be out of range"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains(&format!("integer literal {literal} is out of range for {ty}")),
                "{ty} literal {literal}: unexpected error {msg}"
            );
        }
    }
}

#[test]
fn i128_literals_are_range_checked_exactly() {
    // The bounds are -2^127 and 2^127 - 1; the checker's word path
    // computes in `i128` and cannot state them, so the wide path checks
    // a sign and a `u128` magnitude.
    expect_checks("I128", "170141183460469231731687303715884105727");
    expect_checks("I128", "-170141183460469231731687303715884105728");
    expect_checks("I128", "0");
    expect_checks("I128", "-1");
    expect_out_of_range("I128", "170141183460469231731687303715884105728");
    expect_out_of_range("I128", "-170141183460469231731687303715884105729");
}

#[test]
fn u128_literals_are_range_checked_exactly() {
    // `U128`'s maximum, 2^128 - 1, is beyond every `i128`.
    expect_checks("U128", "340282366920938463463374607431768211455");
    expect_checks("U128", "0");
    expect_out_of_range("U128", "340282366920938463463374607431768211456");
    expect_out_of_range("U128", "-1");
}

#[test]
fn wide_arithmetic_runs_at_128_bits() {
    // Every operand and result is an `i128`: products past 2^64, the wrap
    // at 2^128, the bounds' wrap, saturation in each direction, signed and
    // unsigned order, and the bitwise complement. Each check exits with
    // its own code, so a failure names the operation.
    run_ok(
        "wide-arithmetic",
        r#"
module Main where
  import Data.Wide

  main :: Int
  main =
    let big = wrapping-mul-u128 18446744073709551616 18446744073709551616 in
    let sq = wrapping-mul-u128 18446744073709551617 18446744073709551617 in
    if eq-u128 big 0 then
      (if eq-u128 sq 36893488147419103233 then
        (if eq-i128 (wrapping-add-i128 max-bound-i128 1) min-bound-i128 then
          (if eq-i128 (saturating-add-i128 max-bound-i128 1) max-bound-i128 then
            (if eq-u128 (saturating-sub-u128 0 1) 0 then
              (if eq-i128 (saturating-mul-i128 max-bound-i128 (-2)) min-bound-i128 then
                (if eq-i128 (saturating-neg-i128 min-bound-i128) max-bound-i128 then
                  (if lt-i128 (-1) 0 then
                    (if gt-u128 max-bound-u128 0 then
                      (if eq-u128 (bit-not-u128 0) max-bound-u128 then 0 else 10)
                    else 9)
                  else 8)
                else 7)
              else 6)
            else 5)
          else 4)
        else 3)
      else 2)
    else 1
"#,
    );
}

#[test]
fn wide_values_flow_through_loops_joins_and_tail_groups() {
    // A self tail call over wide parameters is a loop that stores `i128`
    // slots; a value-position `if` delivers a wide value through a typed
    // destination; and a mutually recursive pair with wide parameters
    // (kept apart from the inliner) is one tail group, kept out of the
    // word-typed worker widening and still `musttail` between identical
    // `i128` prototypes, so 100001 levels of it run in constant stack.
    run_ok(
        "wide-control",
        r#"
module Main where
  import Data.Wide

  sumTo :: U128 -> U128 -> U128
  sumTo acc n = if eq-u128 n 0 then acc else sumTo (wrapping-add-u128 acc n) (wrapping-sub-u128 n 1)

  #[inline(never)]
  pick :: Bool -> I128 -> I128 -> I128
  pick b x y =
    let z = if b then x else y in
    wrapping-add-i128 z 1

  #[inline(never)]
  isEven :: U128 -> Bool
  isEven n = if eq-u128 n 0 then True else isOdd (wrapping-sub-u128 n 1)

  #[inline(never)]
  isOdd :: U128 -> Bool
  isOdd n = if eq-u128 n 0 then False else isEven (wrapping-sub-u128 n 1)

  main :: Int
  main =
    if eq-u128 (sumTo 0 1000000) 500000500000 then
      (if eq-i128 (pick False 7 (-170141183460469231731687303715884105728)) (-170141183460469231731687303715884105727) then
        (if isEven 100001 then 3 else 0)
      else 2)
    else 1
"#,
    );
}

#[test]
fn wide_fields_take_two_words_in_records_tuples_and_generic_copies() {
    // A record mixing word and wide fields lays out 1 + 2 + 1 + 2 words,
    // so every field after a wide one is projected at its word offset; a
    // tuple component and a generic constructor reached at `I128` (the
    // collector's `@W` copy) hold an `i128`; and the checked families
    // answer `Option` through `Some@W`, a two-word field.
    run_ok("wide-fields", r#"
module Main where
  import Data.Wide

  record Mixed where
    tag :: I64
    big :: U128
    after :: I64
    small :: I128

  data Box a where
    MkBox :: a -> Box a

  #[inline(never)]
  unbox :: forall a. Box a -> a
  unbox b = case b of
    MkBox x -> x

  #[inline(never)]
  mk :: U128 -> Mixed
  mk v = Mixed { tag = 7, big = v, after = 9, small = -3 }

  #[inline(never)]
  pair :: I128 -> (I64, I128)
  pair x = (5, x)

  main :: Int
  main =
    let m = mk 340282366920938463463374607431768211455 in
    if m.tag == 7 then
      (if eq-u128 m.big max-bound-u128 then
        (if m.after == 9 then
          (if eq-i128 m.small (-3) then
            (case checked-add-u128 max-bound-u128 1 of
              Some _ -> 5
              None -> case checked-mul-i128 3 (-170141183460469231731687303715884105) of
                None -> 6
                Some p -> if eq-i128 p (-510423550381407695195061911147652315) then
                    (case checked-div-i128 min-bound-i128 (-1) of
                      Some _ -> 8
                      None -> case checked-rem-u128 17 5 of
                        None -> 9
                        Some r -> if eq-u128 r 2 then
                            (case pair 12 of
                              (a, b) -> if a == 5 then
                                  (if eq-i128 (unbox (MkBox b)) 12 then 0 else 12)
                                else 11)
                          else 10)
                  else 7)
          else 4)
        else 3)
      else 2)
    else 1
"#);
}

#[test]
fn a_counted_field_after_a_wide_one_is_found_by_its_map() {
    // The pointer map describes words: the wide field is two raw words,
    // so the `String` after it is word 2, and reclamation counts it
    // there. A map that put it at word 1 would read half the `i128` as a
    // pointer.
    run_ok(
        "wide-map",
        r#"
module Main where
  import Data.Wide

  record Tagged where
    n :: I128
    label :: String

  #[inline(never)]
  mk :: I128 -> String -> Tagged
  mk n s = Tagged { n = n, label = s }

  #[inline(never)]
  size :: Tagged -> Nat
  size t = string-length t.label

  main :: Int
  main =
    let a = mk (-1) "wide" in
    let b = mk 170141183460469231731687303715884105727 "words" in
    if size a == 4 then (if size b == 5 then (if eq-i128 b.n max-bound-i128 then 0 else 3) else 2) else 1
"#,
    );
}

#[test]
fn unlowered_wide_positions_are_refused_by_name() {
    // A function with a wide boundary used as a value would be applied
    // through the word-typed apply frame, and a `repr(C)` record's wide
    // field is a C `__int128` at its recorded offset (the boundary
    // increment). Both are later increments, and each is refused, naming
    // the unit, rather than miscompiled.
    expect_refused(
        "wide-closure",
        r#"
module Main where
  import Data.Wide

  twice :: (I128 -> I128) -> I128 -> I128
  twice f x = f (f x)

  main :: Int
  main = if eq-i128 (twice wrapping-neg-i128 5) 5 then 0 else 1
"#,
        "I128",
    );
}
