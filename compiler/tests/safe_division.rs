//! Division is total by construction (`design/no-faults.md` section 3):
//! `nat-div`/`nat-rem`/`int-div`/`int-rem` and the `Div`/`Rem` operators
//! take a `NonZero` divisor. A numeral divisor decides its predicate at
//! compile time with nothing written, a variable divisor needs evidence
//! (a `decide` or a threaded `NonZero`), and `checked-div`/`checked-rem`
//! answer `None` for a zero divisor. Every program here runs through the
//! real driver.

use cinch::driver::{check_sources, run_source};

fn expect_run(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_err(name: &str, src: &str, needles: &[&str]) {
    match check_sources(src, name) {
        Ok(_) => panic!("{name}: expected an error, but the program checked"),
        Err(e) => {
            let msg = e.to_string();
            for n in needles {
                assert!(msg.contains(n), "{name}: error {msg:?} does not contain {n:?}");
            }
        }
    }
}

#[test]
fn literal_divisors_compile_unchanged_and_compute() {
    // Each failed check sets its own bit, so the exit code names it.
    expect_run(
        "literals",
        r#"
module Main where
  bit :: Bool -> Int -> Int
  bit ok b = if ok then 0 else b

  main :: Int
  main =
    bit (nat-div 100 10 == 10) 1
      + bit (nat-rem 103 10 == 3) 2
      + bit (int-div (-17) 5 == -3) 4
      + bit (int-rem (-17) 5 == -2) 8
      + bit ((100 :: Nat) / 7 == 14) 16
      + bit ((100 :: Nat) % 7 == 2) 32
      + bit ((-17 :: Int) / 5 == -3) 64
      + bit (int-div 7 (-1) == -7) 128
"#,
        0,
    );
}

#[test]
fn a_zero_literal_divisor_is_refused() {
    expect_err(
        "zero-literal",
        r#"
module Main where
  main :: Int
  main = if nat-div 7 0 == 0 then 0 else 1
"#,
        &["integer literal 0 is not a NonZero Nat", "NonZero"],
    );
}

#[test]
fn an_unrefined_variable_divisor_is_refused_naming_the_evidence() {
    expect_err(
        "unrefined-variable",
        r#"
module Main where
  per :: Nat -> Nat -> Nat
  per n d = nat-div n d

  main :: Int
  main = if per 10 2 == 5 then 0 else 1
"#,
        &["NonZero"],
    );
}

#[test]
fn the_carrier_must_be_a_number() {
    expect_err(
        "non-numeric-carrier",
        r#"
module Main where
  f :: NonZero Bool -> Int
  f b = 0

  main :: Int
  main = 0
"#,
        &["`NonZero` decides", "Bool"],
    );
}

#[test]
fn a_decided_divisor_runs_both_branches() {
    expect_run(
        "decided",
        r#"
module Main where
  per :: Nat -> Nat -> Nat
  per n d = case decide (NonZero d) of
    Yes w -> nat-div n (NonZero d w)
    No _ -> 999

  per-int :: Int -> Int -> Int
  per-int n d = case decide (NonZero d) of
    Yes w -> int-rem n (NonZero d w)
    No _ -> 999

  main :: Int
  main =
    if per 100 4 == 25 && per 100 0 == 999 && per-int (-7) 2 == -1 && per-int 5 0 == 999 then 0 else 1
"#,
        0,
    );
}

#[test]
fn a_threaded_divisor_needs_no_check_at_its_use() {
    expect_run(
        "threaded",
        r#"
module Main where
  share :: Nat -> NonZero Nat -> Nat
  share total parts = nat-div total parts + nat-rem total parts

  main :: Int
  main = if share 17 5 == 5 then 0 else 1
"#,
        0,
    );
}

#[test]
fn checked_division_is_none_on_zero() {
    expect_run(
        "checked",
        r#"
module Main where
  is-none :: Option Nat -> Bool
  is-none o = case o of
    None -> True
    Some _ -> False

  is-some :: Nat -> Option Nat -> Bool
  is-some v o = case o of
    None -> False
    Some x -> x == v

  main :: Int
  main =
    if is-none (checked-div 7 0) && is-none (checked-rem 7 0)
      && is-some 3 (checked-div 7 2) && is-some 1 (checked-rem 7 2)
    then 0 else 1
"#,
        0,
    );
}
