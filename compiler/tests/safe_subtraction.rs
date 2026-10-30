//! `Nat` subtraction is total by construction (design/no-faults.md
//! section 7): `a - b` at `Nat` and `nat-sub a b` are the exact difference
//! and demand evidence that `b <= a`. Numerals decide it at compile time,
//! a branch fact (`if`, a guard, a `case` row) or `Dec` evidence supplies
//! it, and a definition's own `using (b <= a)` obligation holds in its
//! body. `saturating-sub` truncates at zero under its own name and
//! `checked-sub` answers `None`. The `Sub` class is unchanged: the `Nat`
//! instance states the obligation in a method signature, so `-` at `Int`
//! needs nothing, and that instance's dictionary never reaches generic
//! code, where `-` could be applied without the evidence.

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

// ------------------------------------------------------------- literals

#[test]
fn numeral_operands_decide_at_compile_time() {
    expect_run(
        "literal-in-order",
        r#"
module Main where
  five-less-three :: Nat
  five-less-three = 5 - 3

  same :: Nat
  same = nat-sub 4 4

  main :: Int
  main = if five-less-three == 2 && same == 0 then 0 else 1
"#,
        0,
    );
    expect_err(
        "literal-reversed",
        r#"
module Main where
  x :: Nat
  x = 3 - 5

  main :: Int
  main = 0
"#,
        &["the obligation `5 <= 3` of this call is false"],
    );
    expect_err(
        "literal-reversed-nat-sub",
        r#"
module Main where
  x :: Nat
  x = nat-sub 0 1

  main :: Int
  main = 0
"#,
        &["the obligation `1 <= 0` of this call is false"],
    );
    // At `Int` the same spelling needs nothing.
    expect_run(
        "int-unchanged",
        r#"
module Main where
  main :: Int
  main = if 3 - 5 == 0 - 2 then 0 else 1
"#,
        0,
    );
}

// ------------------------------------------------------------ evidence

#[test]
fn a_guard_supplies_the_evidence() {
    expect_run(
        "guarded",
        r#"
module Main where
  diff :: Nat -> Nat -> Nat
  diff a b = if b <= a then a - b else 0

  flipped :: Nat -> Nat -> Nat
  flipped a b = if a >= b then nat-sub a b else 0

  pred-or-zero :: Nat -> Nat
  pred-or-zero n = if n == 0 then 0 else n - 1

  main :: Int
  main = if diff 7 2 == 5 && diff 2 7 == 0 && flipped 9 4 == 5 && pred-or-zero 0 == 0 && pred-or-zero 6 == 5 then 0 else 1
"#,
        0,
    );
}

#[test]
fn an_unguarded_variable_subtraction_is_refused() {
    expect_err(
        "unguarded-operator",
        r#"
module Main where
  f :: Nat -> Nat -> Nat
  f a b = a - b

  main :: Int
  main = 0
"#,
        &["the obligation `b <= a`", "no evidence"],
    );
    expect_err(
        "unguarded-nat-sub",
        r#"
module Main where
  f :: Nat -> Nat -> Nat
  f a b = nat-sub a b

  main :: Int
  main = 0
"#,
        &["the obligation `b <= a`", "no evidence"],
    );
    // The wrong guard proves nothing about this difference.
    expect_err(
        "wrong-guard",
        r#"
module Main where
  f :: Nat -> Nat -> Nat
  f a b = if a <= b then a - b else 0

  main :: Int
  main = 0
"#,
        &["the obligation `b <= a`"],
    );
}

#[test]
fn case_rows_supply_the_evidence() {
    expect_run(
        "case-rows",
        r#"
module Main where
  -- A literal row's default branch knows `n /= 0`, so `n - 1` holds.
  down :: Nat -> Nat
  down n = case n of
    0 -> 0
    m -> m - 1

  -- A guard on a row carries its comparison into the row.
  gap :: Nat -> Nat -> Nat
  gap a b = case a of
    x | b < x -> x - b
    _ -> 0

  -- A comparison scrutinee binds each row's fact.
  spread :: Nat -> Nat -> Nat
  spread a b = case b <= a of
    True -> a - b
    False -> b - a

  main :: Int
  main = if down 0 == 0 && down 5 == 4 && gap 9 2 == 7 && gap 2 9 == 0 && spread 3 8 == 5 && spread 8 3 == 5 then 0 else 1
"#,
        0,
    );
}

#[test]
fn a_signature_obligation_holds_in_its_own_body() {
    // The callers discharge `1 <= depth`, so the body may count down
    // past a `depth == 1` test.
    expect_run(
        "own-obligation",
        r#"
module Main where
  unwind :: using (1 <= depth) => (depth :: Nat) -> Nat -> Nat
  unwind depth acc = if depth == 1 then acc else unwind (depth - 1) (acc + 1)

  main :: Int
  main = if unwind 4 0 == 3 && unwind 1 7 == 7 then 0 else 1
"#,
        0,
    );
    expect_err(
        "own-obligation-caller",
        r#"
module Main where
  unwind :: using (1 <= depth) => (depth :: Nat) -> Nat -> Nat
  unwind depth acc = if depth == 1 then acc else unwind (depth - 1) (acc + 1)

  main :: Int
  main = if unwind 0 0 == 0 then 0 else 1
"#,
        &["the obligation `1 <= 0` of this call is false"],
    );
}

// ------------------------------------------------- the total named forms

#[test]
fn saturating_and_checked_subtraction_at_the_edges() {
    expect_run(
        "named-forms",
        r#"
module Main where
  none :: Option Nat -> Bool
  none o = case o of
    None -> True
    Some _ -> False

  is :: Nat -> Option Nat -> Bool
  is k o = case o of
    None -> False
    Some v -> v == k

  main :: Int
  main =
    if saturating-sub 7 3 == 4
      && saturating-sub 3 3 == 0
      && saturating-sub 3 4 == 0
      && saturating-sub 0 1 == 0
      && saturating-sub 18446744073709551616 1 == 18446744073709551615
      && is 4 (checked-sub 7 3)
      && is 0 (checked-sub 3 3)
      && none (checked-sub 3 4)
      && none (checked-sub 0 18446744073709551616)
      && is 1 (checked-sub 18446744073709551616 18446744073709551615)
    then 0 else 1
"#,
        0,
    );
}

// --------------------------------------------- the class and its holes

#[test]
fn generic_code_cannot_receive_the_nat_instance() {
    // Code over `Sub a` could apply `-` to anything, so the `Nat`
    // dictionary, whose method needs evidence of its arguments, is
    // refused there; `Int` passes as before.
    expect_run(
        "generic-int",
        r#"
module Main where
  twice-less :: forall a. using (s :: Sub a) => a -> a -> a
  twice-less x y = (x - y) - y

  main :: Int
  main = if twice-less 10 3 == 4 then 0 else 1
"#,
        0,
    );
    expect_err(
        "generic-nat",
        r#"
module Main where
  twice-less :: forall a. using (s :: Sub a) => a -> a -> a
  twice-less x y = (x - y) - y

  n :: Nat
  n = twice-less 10 3

  main :: Int
  main = 0
"#,
        &["instance `Sub_Nat`", "applied directly"],
    );
    // The method as a value is the same indirect use.
    expect_err(
        "method-as-value",
        r#"
module Main where
  apply :: (Nat -> Nat -> Nat) -> Nat
  apply f = f 3 5

  main :: Int
  main = if apply (-) == 0 then 0 else 1
"#,
        &["instance `Sub_Nat`", "applied directly"],
    );
}

#[test]
fn a_qualified_callee_carries_its_obligation() {
    // Regression: a qualified name used to skip the callee's obligations.
    expect_err(
        "qualified",
        r#"
module Main where
  import Data.Nat as N

  f :: Nat -> Nat -> Nat
  f a b = N.nat-sub a b

  main :: Int
  main = 0
"#,
        &["the obligation `b <= a`"],
    );
    expect_run(
        "qualified-guarded",
        r#"
module Main where
  import Data.Nat as N

  f :: Nat -> Nat -> Nat
  f a b = if b <= a then N.nat-sub a b else 0

  main :: Int
  main = if f 9 4 == 5 then 0 else 1
"#,
        0,
    );
}

#[test]
fn an_obligation_bearing_function_passed_inside_a_callee_is_refused() {
    // Regression: a function passed as an argument within a callee
    // expression (`(pick nat-sub) a b`) escaped the "applied directly"
    // rule, so its obligation never discharged.
    expect_err(
        "passed-in-callee",
        r#"
module Main where
  pick :: (Nat -> Nat -> Nat) -> Nat -> Nat -> Nat
  pick f a b = f a b

  main :: Int
  main = if (pick nat-sub) 3 5 == 0 then 0 else 1
"#,
        &["nat-sub must be applied directly"],
    );
    expect_err(
        "conditional-callee",
        r#"
module Main where
  main :: Int
  main = if (if True then nat-sub else nat-add) 3 5 == 0 then 0 else 1
"#,
        &["nat-sub must be applied directly"],
    );
}

#[test]
fn an_obligation_after_an_unnamed_argument_names_the_right_one() {
    // Regression: positions counted only named binders, so an obligation
    // on the second argument was checked against the first.
    expect_err(
        "positions",
        r#"
module Main where
  small :: using (x < 5) => Nat -> (x :: Nat) -> Nat
  small k x = k + x

  main :: Int
  main = if small 1 9 == 10 then 0 else 1
"#,
        &["the obligation `9 < 5` of this call is false"],
    );
    expect_run(
        "positions-ok",
        r#"
module Main where
  small :: using (x < 5) => Nat -> (x :: Nat) -> Nat
  small k x = k + x

  main :: Int
  main = if small 9 1 == 10 then 0 else 1
"#,
        0,
    );
}
