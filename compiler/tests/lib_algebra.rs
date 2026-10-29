//! Algebra family tests: `Algebra.Semigroup` and `Algebra.Monoid`.
//!
//! The consumer-visible semantics of `<>` and `mempty` per
//! instance, with the class laws asserted over sample values:
//! associativity of `<>`, and left/right unit for every `Monoid`
//! instance. The law checks are exhaustive over their sample sets (all
//! ordered triples of the samples, all samples for the units).
//!
//! Every program is the real prelude module sources plus a `Main`
//! module, compiled and run through the same driver the CLI uses.

use cinch::driver::{driver, run_source, Mode};
use std::path::PathBuf;

const SEMIGROUP: &str = include_str!("../prelude/Algebra/Semigroup.cinch");
const MONOID: &str = include_str!("../prelude/Algebra/Monoid.cinch");
/// `Text.Regex` is not part of the embedded prelude, so its consumer
/// test writes it beside the entry as a file project (the shape
/// `lib_benchmark.rs` uses for `Text.Scan`).
const REGEX: &str = include_str!("../prelude/Text/Regex.cinch");

/// Helpers shared by the law programs: structural equality on the
/// sample carriers (List and Option of Nat, Ordering), the newtype
/// observers, and the sample values indexed by tag so a law can be
/// checked over every ordered triple of samples with one recursion.
const HELPERS: &str = r#"
  eqListNat :: List Nat -> List Nat -> Bool
  eqListNat xs ys = case xs of
    Nil -> case ys of
      Nil -> True
      Cons _ _ -> False
    Cons a ar -> case ys of
      Nil -> False
      Cons b bs -> if a == b then eqListNat ar bs else False

  eqOptNat :: Option Nat -> Option Nat -> Bool
  eqOptNat x y = case x of
    None -> case y of
      None -> True
      Some _ -> False
    Some a -> case y of
      None -> False
      Some b -> a == b

  ordTag :: Ordering -> Nat
  ordTag o = case o of
    LT -> 0
    EQ -> 1
    GT -> 2

  eqOrd :: Ordering -> Ordering -> Bool
  eqOrd x y = nat-cmp (ordTag x) (ordTag y) == 0

  fromFirst :: forall a. First a -> Option a
  fromFirst x = case x of
    First o -> o

  fromLast :: forall a. Last a -> Option a
  fromLast x = case x of
    Last o -> o

  fromDual :: forall a. Dual a -> a
  fromDual x = case x of
    Dual p -> p

  applyEndo :: forall a. Endo a -> a -> a
  applyEndo x v = case x of
    Endo f -> f v

  fromSum :: Sum -> Nat
  fromSum x = case x of
    Sum n -> n

  fromProduct :: Product -> Nat
  fromProduct x = case x of
    Product n -> n

  pfst :: forall a b. (a, b) -> a
  pfst p = case p of
    (a, b) -> a

  psnd :: forall a b. (a, b) -> b
  psnd p = case p of
    (a, b) -> b

  natByTag :: Nat -> Nat
  natByTag n = if n == 0 then 0 else if n == 1 then 1 else 7

  listByTag :: Nat -> List Nat
  listByTag n = if n == 0 then Nil else if n == 1 then Cons 1 Nil else Cons 2 (Cons 3 Nil)

  optByTag :: Nat -> Option Nat
  optByTag n = if n == 0 then None else if n == 1 then Some 0 else Some 1

  ordByTag :: Nat -> Ordering
  ordByTag n = if n == 0 then LT else if n == 1 then EQ else GT

  fromAll :: All -> Bool
  fromAll x = case x of
    All b -> b

  fromAny :: Any -> Bool
  fromAny x = case x of
    Any b -> b
"#;

/// The `Main` module of a law program: the standard imports (with the
/// `<>` fixity, decision D2 per-module fixity declarations) around the
/// shared helpers and the program body.
fn main_module(body: &str) -> String {
    let mut src = String::from(
        "module Main where\n\n  import Algebra.Semigroup\n  import Algebra.Monoid\n\n  infixr 6 <>\n",
    );
    src.push_str(HELPERS);
    src.push('\n');
    src.push_str(body);
    src.push('\n');
    src
}

/// The whole program: the real module sources, then `Main`.
fn whole_program(body: &str) -> String {
    let mut src = String::new();
    src.push_str(SEMIGROUP);
    src.push('\n');
    src.push_str(MONOID);
    src.push('\n');
    src.push_str(&main_module(body));
    src
}

fn expect_exit(name: &str, body: &str, expected: i32) {
    match run_source(&whole_program(body)) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

/// A temp project for the `Text.Regex` consumer: the real module source
/// beside a `Main`, so `import Text.Regex` resolves to it and every
/// other import falls back to the embedded prelude.
fn expect_regex_exit(name: &str, main: &str, expected: i32) {
    let dir = std::env::temp_dir().join(format!("cinch_algebra_regex_{}_{}", name, std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    std::fs::write(dir.join("Text.Regex.cinch"), REGEX).expect("write Text.Regex");
    std::fs::write(dir.join("main.cinch"), main).expect("write main");
    let code = driver(Mode::Run, &dir.join("main.cinch"), None).expect("regex run");
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}");
}

// ---------------------------------------------------------------- List

#[test]
fn list_append_is_associative() {
    // Every ordered triple of the sample lists: (xs <> ys) <> zs and
    // xs <> (ys <> zs) agree.
    expect_exit(
        "list-assoc",
        r#"
  assoc :: Nat -> Bool
  assoc n =
    if n == 0 then True
    else
      let m = n - 1 in
      let x = listByTag (m / 9) in
      let y = listByTag ((m / 3) % 3) in
      let z = listByTag (m % 3) in
      if eqListNat ((x <> y) <> z) (x <> (y <> z)) then assoc m else False

  main :: Int
  main =
    let c1 = assoc 27 in
    let c2 = eqListNat (listByTag 1 <> listByTag 2) (Cons 1 (Cons 2 (Cons 3 Nil))) in
    if c1 then (if c2 then 0 else 2) else 1
"#,
        0,
    );
}

#[test]
fn list_mempty_is_the_identity_for_append() {
    expect_exit(
        "list-units",
        r#"
  mtList :: List Nat
  mtList = mempty

  leftUnit :: Nat -> Bool
  leftUnit n =
    if n == 0 then True
    else
      let x = listByTag (n - 1) in
      if eqListNat (mtList <> x) x then leftUnit (n - 1) else False

  rightUnit :: Nat -> Bool
  rightUnit n =
    if n == 0 then True
    else
      let x = listByTag (n - 1) in
      if eqListNat (x <> mtList) x then rightUnit (n - 1) else False

  main :: Int
  main =
    let c1 = leftUnit 3 in
    let c2 = rightUnit 3 in
    if c1 then (if c2 then (if eqListNat mtList Nil then 0 else 3) else 2) else 1
"#,
        0,
    );
}

// -------------------------------------------------------------- Option

#[test]
fn option_first_some_is_associative() {
    expect_exit(
        "option-assoc",
        r#"
  assoc :: Nat -> Bool
  assoc n =
    if n == 0 then True
    else
      let m = n - 1 in
      let x = optByTag (m / 9) in
      let y = optByTag ((m / 3) % 3) in
      let z = optByTag (m % 3) in
      if eqOptNat ((x <> y) <> z) (x <> (y <> z)) then assoc m else False

  main :: Int
  main =
    let c1 = assoc 27 in
    let c2 = eqOptNat (Some 1 <> None) (Some 1) in
    let c3 = eqOptNat (None <> Some 2) (Some 2) in
    let c4 = eqOptNat (Some 1 <> Some 2) (Some 1) in
    if c1 then (if c2 then (if c3 then (if c4 then 0 else 4) else 3) else 2) else 1
"#,
        0,
    );
}

#[test]
fn option_mempty_is_the_identity_for_first_some() {
    expect_exit(
        "option-units",
        r#"
  mtOpt :: Option Nat
  mtOpt = mempty

  leftUnit :: Nat -> Bool
  leftUnit n =
    if n == 0 then True
    else
      let x = optByTag (n - 1) in
      if eqOptNat (mtOpt <> x) x then leftUnit (n - 1) else False

  rightUnit :: Nat -> Bool
  rightUnit n =
    if n == 0 then True
    else
      let x = optByTag (n - 1) in
      if eqOptNat (x <> mtOpt) x then rightUnit (n - 1) else False

  main :: Int
  main =
    let c1 = leftUnit 3 in
    let c2 = rightUnit 3 in
    if c1 then (if c2 then (if eqOptNat mtOpt None then 0 else 3) else 2) else 1
"#,
        0,
    );
}

// ------------------------------------------------------------ Ordering

#[test]
fn ordering_composition_is_associative() {
    expect_exit(
        "ordering-assoc",
        r#"
  assoc :: Nat -> Bool
  assoc n =
    if n == 0 then True
    else
      let m = n - 1 in
      let x = ordByTag (m / 9) in
      let y = ordByTag ((m / 3) % 3) in
      let z = ordByTag (m % 3) in
      if eqOrd ((x <> y) <> z) (x <> (y <> z)) then assoc m else False

  main :: Int
  main =
    let c1 = assoc 27 in
    let c2 = eqOrd (LT <> GT) LT in
    let c3 = eqOrd (EQ <> GT) GT in
    let c4 = eqOrd (GT <> EQ) GT in
    let c5 = eqOrd (GT <> LT) GT in
    if c1 then (if c2 then (if c3 then (if c4 then (if c5 then 0 else 5) else 4) else 3) else 2) else 1
"#,
        0,
    );
}

#[test]
fn ordering_mempty_is_the_identity_for_composition() {
    expect_exit(
        "ordering-units",
        r#"
  mtOrd :: Ordering
  mtOrd = mempty

  leftUnit :: Nat -> Bool
  leftUnit n =
    if n == 0 then True
    else
      let x = ordByTag (n - 1) in
      if eqOrd (mtOrd <> x) x then leftUnit (n - 1) else False

  rightUnit :: Nat -> Bool
  rightUnit n =
    if n == 0 then True
    else
      let x = ordByTag (n - 1) in
      if eqOrd (x <> mtOrd) x then rightUnit (n - 1) else False

  main :: Int
  main =
    let c1 = leftUnit 3 in
    let c2 = rightUnit 3 in
    if c1 then (if c2 then (if eqOrd mtOrd EQ then 0 else 3) else 2) else 1
"#,
        0,
    );
}

// -------------------------------------------------------------- String

#[test]
fn string_concatenation_is_associative_with_empty_string_identity() {
    expect_exit(
        "string",
        r#"
  s1 :: String
  s1 = nat-to-string 1

  s2 :: String
  s2 = nat-to-string 12

  s3 :: String
  s3 = nat-to-string 345

  mtStr :: String
  mtStr = mempty

  main :: Int
  main =
    let c1 = (s1 <> s2) <> s3 == s1 <> (s2 <> s3) in
    let c2 = nat-to-string 12 <> nat-to-string 345 == nat-to-string 12345 in
    let c3 = mtStr <> s2 == s2 in
    let c4 = s2 <> mtStr == s2 in
    let c5 = nat-cmp (string-byte-length mtStr) 0 == 0 in
    if c1 then (if c2 then (if c3 then (if c4 then (if c5 then 0 else 5) else 4) else 3) else 2) else 1
"#,
        0,
    );
}

// ---------------------------------------------------------- Sum/Product

#[test]
fn sum_newtype_adds_and_obeys_the_laws() {
    expect_exit(
        "sum",
        r#"
  assoc :: Nat -> Bool
  assoc n =
    if n == 0 then True
    else
      let m = n - 1 in
      let x = Sum (natByTag (m / 9)) in
      let y = Sum (natByTag ((m / 3) % 3)) in
      let z = Sum (natByTag (m % 3)) in
      if nat-cmp (fromSum ((x <> y) <> z)) (fromSum (x <> (y <> z))) == 0 then assoc m else False

  units :: Nat -> Bool
  units n =
    if n == 0 then True
    else
      let x = Sum (natByTag (n - 1)) in
      if nat-cmp (fromSum (mempty <> x)) (fromSum x) == 0 then
        if nat-cmp (fromSum (x <> mempty)) (fromSum x) == 0 then units (n - 1) else False
      else False

  main :: Int
  main =
    let c1 = assoc 27 in
    let c2 = units 3 in
    let c3 = nat-cmp (fromSum (Sum 2 <> Sum 3)) 5 == 0 in
    let c4 = nat-cmp (fromSum (mempty)) 0 == 0 in
    if c1 then (if c2 then (if c3 then (if c4 then 0 else 4) else 3) else 2) else 1
"#,
        0,
    );
}

#[test]
fn product_newtype_multiplies_and_obeys_the_laws() {
    expect_exit(
        "product",
        r#"
  assoc :: Nat -> Bool
  assoc n =
    if n == 0 then True
    else
      let m = n - 1 in
      let x = Product (natByTag (m / 9)) in
      let y = Product (natByTag ((m / 3) % 3)) in
      let z = Product (natByTag (m % 3)) in
      if nat-cmp (fromProduct ((x <> y) <> z)) (fromProduct (x <> (y <> z))) == 0 then assoc m else False

  units :: Nat -> Bool
  units n =
    if n == 0 then True
    else
      let x = Product (natByTag (n - 1)) in
      if nat-cmp (fromProduct (mempty <> x)) (fromProduct x) == 0 then
        if nat-cmp (fromProduct (x <> mempty)) (fromProduct x) == 0 then units (n - 1) else False
      else False

  main :: Int
  main =
    let c1 = assoc 27 in
    let c2 = units 3 in
    let c3 = nat-cmp (fromProduct (Product 2 <> Product 3)) 6 == 0 in
    let c4 = nat-cmp (fromProduct (mempty)) 1 == 0 in
    if c1 then (if c2 then (if c3 then (if c4 then 0 else 4) else 3) else 2) else 1
"#,
        0,
    );
}

// ---------------------------------------------------------------- Dual

#[test]
fn dual_newtype_reverses_composition_and_obeys_the_laws() {
    // Carrier: Dual over List Nat, so Dual's Semigroup needs the
    // `Semigroup (List t)` subdict and Dual's Monoid the `Monoid
    // (List t)` one.
    expect_exit(
        "dual",
        r#"
  assoc :: Nat -> Bool
  assoc n =
    if n == 0 then True
    else
      let m = n - 1 in
      let x = Dual (listByTag (m / 9)) in
      let y = Dual (listByTag ((m / 3) % 3)) in
      let z = Dual (listByTag (m % 3)) in
      if eqListNat (fromDual ((x <> y) <> z)) (fromDual (x <> (y <> z))) then assoc m else False

  units :: Nat -> Bool
  units n =
    if n == 0 then True
    else
      let x = Dual (listByTag (n - 1)) in
      if eqListNat (fromDual (mempty <> x)) (fromDual x) then
        if eqListNat (fromDual (x <> mempty)) (fromDual x) then units (n - 1) else False
      else False

  main :: Int
  main =
    let xs = listByTag 1 in
    let ys = listByTag 2 in
    let c1 = assoc 27 in
    let c2 = units 3 in
    let c3 = eqListNat (fromDual (Dual xs <> Dual ys)) (ys <> xs) in
    if c1 then (if c2 then (if c3 then 0 else 3) else 2) else 1
"#,
        0,
    );
}

// ----------------------------------------------------------- First/Last

#[test]
fn first_newtype_keeps_the_leftmost_some_and_obeys_the_laws() {
    expect_exit(
        "first",
        r#"
  assoc :: Nat -> Bool
  assoc n =
    if n == 0 then True
    else
      let m = n - 1 in
      let x = First (optByTag (m / 9)) in
      let y = First (optByTag ((m / 3) % 3)) in
      let z = First (optByTag (m % 3)) in
      if eqOptNat (fromFirst ((x <> y) <> z)) (fromFirst (x <> (y <> z))) then assoc m else False

  units :: Nat -> Bool
  units n =
    if n == 0 then True
    else
      let x = First (optByTag (n - 1)) in
      if eqOptNat (fromFirst (mempty <> x)) (fromFirst x) then
        if eqOptNat (fromFirst (x <> mempty)) (fromFirst x) then units (n - 1) else False
      else False

  main :: Int
  main =
    let c1 = assoc 27 in
    let c2 = units 3 in
    let c3 = eqOptNat (fromFirst (First (Some 1) <> First (Some 2))) (Some 1) in
    let c4 = eqOptNat (fromFirst (First None <> First (Some 2))) (Some 2) in
    let c5 = eqOptNat (fromFirst (mempty)) None in
    if c1 then (if c2 then (if c3 then (if c4 then (if c5 then 0 else 5) else 4) else 3) else 2) else 1
"#,
        0,
    );
}

#[test]
fn last_newtype_keeps_the_rightmost_some_and_obeys_the_laws() {
    expect_exit(
        "last",
        r#"
  assoc :: Nat -> Bool
  assoc n =
    if n == 0 then True
    else
      let m = n - 1 in
      let x = Last (optByTag (m / 9)) in
      let y = Last (optByTag ((m / 3) % 3)) in
      let z = Last (optByTag (m % 3)) in
      if eqOptNat (fromLast ((x <> y) <> z)) (fromLast (x <> (y <> z))) then assoc m else False

  units :: Nat -> Bool
  units n =
    if n == 0 then True
    else
      let x = Last (optByTag (n - 1)) in
      if eqOptNat (fromLast (mempty <> x)) (fromLast x) then
        if eqOptNat (fromLast (x <> mempty)) (fromLast x) then units (n - 1) else False
      else False

  main :: Int
  main =
    let c1 = assoc 27 in
    let c2 = units 3 in
    let c3 = eqOptNat (fromLast (Last (Some 1) <> Last (Some 2))) (Some 2) in
    let c4 = eqOptNat (fromLast (Last (Some 1) <> Last None)) (Some 1) in
    let c5 = eqOptNat (fromLast (mempty)) None in
    if c1 then (if c2 then (if c3 then (if c4 then (if c5 then 0 else 5) else 4) else 3) else 2) else 1
"#,
        0,
    );
}

// ---------------------------------------------------------------- Endo

#[test]
fn endo_newtype_composes_and_obeys_the_laws() {
    expect_exit(
        "endo",
        r#"
  step :: Nat -> Nat
  step n = n + 1

  double :: Nat -> Nat
  double n = n * 2

  plus3 :: Nat -> Nat
  plus3 n = n + 3

  endoByTag :: Nat -> Endo Nat
  endoByTag n = if n == 0 then Endo step else if n == 1 then Endo double else Endo plus3

  -- Agreement is observed by applying the endomorphism at sample
  -- points (its only consumer-visible behavior).
  endoEq :: Endo Nat -> Endo Nat -> Bool
  endoEq x y =
    if nat-cmp (applyEndo x 0) (applyEndo y 0) == 0 then
      if nat-cmp (applyEndo x 5) (applyEndo y 5) == 0 then
        nat-cmp (applyEndo x 9) (applyEndo y 9) == 0
      else False
    else False

  assoc :: Nat -> Bool
  assoc n =
    if n == 0 then True
    else
      let m = n - 1 in
      let x = endoByTag (m / 9) in
      let y = endoByTag ((m / 3) % 3) in
      let z = endoByTag (m % 3) in
      if endoEq ((x <> y) <> z) (x <> (y <> z)) then assoc m else False

  units :: Nat -> Bool
  units n =
    if n == 0 then True
    else
      let x = endoByTag (n - 1) in
      if endoEq (mempty <> x) x then
        if endoEq (x <> mempty) x then units (n - 1) else False
      else False

  main :: Int
  main =
    let c1 = assoc 27 in
    let c2 = units 3 in
    let c3 = nat-cmp (applyEndo (Endo step <> Endo double) 5) 11 == 0 in
    let c4 = nat-cmp (applyEndo (mempty) 7) 7 == 0 in
    if c1 then (if c2 then (if c3 then (if c4 then 0 else 4) else 3) else 2) else 1
"#,
        0,
    );
}

// ------------------------------------------------------------- Function

#[test]
fn function_semigroup_is_pointwise_and_obeys_the_laws() {
    expect_exit(
        "function-semigroup",
        r#"
  f1 :: Nat -> List Nat
  f1 n = Cons n Nil

  f0 :: Nat -> List Nat
  f0 n = Cons 0 Nil

  f2 :: Nat -> List Nat
  f2 n = Cons n (Cons n Nil)

  fByTag :: Nat -> Nat -> List Nat
  fByTag t n = if t == 0 then f1 n else if t == 1 then f0 n else f2 n

  -- Agreement is observed by applying at sample points.
  fEq :: (Nat -> List Nat) -> (Nat -> List Nat) -> Bool
  fEq f g =
    if eqListNat (f 0) (g 0) then
      if eqListNat (f 7) (g 7) then eqListNat (f 3) (g 3) else False
    else False

  assoc :: Nat -> Bool
  assoc n =
    if n == 0 then True
    else
      let m = n - 1 in
      let x = fByTag (m / 9) in
      let y = fByTag ((m / 3) % 3) in
      let z = fByTag (m % 3) in
      if fEq ((x <> y) <> z) (x <> (y <> z)) then assoc m else False

  main :: Int
  main =
    let c1 = assoc 27 in
    let c2 = eqListNat ((f1 <> f0) 9) (Cons 9 (Cons 0 Nil)) in
    if c1 then (if c2 then 0 else 2) else 1
"#,
        0,
    );
}

#[test]
fn function_mempty_is_the_pointwise_identity() {
    expect_exit(
        "function-monoid",
        r#"
  f1 :: Nat -> List Nat
  f1 n = Cons n Nil

  f0 :: Nat -> List Nat
  f0 n = Cons 0 Nil

  f2 :: Nat -> List Nat
  f2 n = Cons n (Cons n Nil)

  fByTag :: Nat -> Nat -> List Nat
  fByTag t n = if t == 0 then f1 n else if t == 1 then f0 n else f2 n

  fEq :: (Nat -> List Nat) -> (Nat -> List Nat) -> Bool
  fEq f g =
    if eqListNat (f 0) (g 0) then
      if eqListNat (f 7) (g 7) then eqListNat (f 3) (g 3) else False
    else False

  units :: Nat -> Bool
  units n =
    if n == 0 then True
    else
      let x = fByTag (n - 1) in
      if fEq (mempty <> x) x then
        if fEq (x <> mempty) x then units (n - 1) else False
      else False

  fId :: Nat -> List Nat
  fId = mempty

  main :: Int
  main =
    let c1 = units 3 in
    let c2 = eqListNat (fId 9) Nil in
    if c1 then (if c2 then 0 else 2) else 1
"#,
        0,
    );
}

// ---------------------------------------------------------------- Pair

#[test]
fn tuple_semigroup_and_monoid_are_componentwise_and_obeys_the_laws() {
    expect_exit(
        "pair",
        r#"
  pByTag :: Nat -> (List Nat, Ordering)
  pByTag t = (listByTag t, ordByTag t)

  pEq :: (List Nat, Ordering) -> (List Nat, Ordering) -> Bool
  pEq p q =
    if eqListNat (pfst p) (pfst q) then eqOrd (psnd p) (psnd q) else False

  assoc :: Nat -> Bool
  assoc n =
    if n == 0 then True
    else
      let m = n - 1 in
      let x = pByTag (m / 9) in
      let y = pByTag ((m / 3) % 3) in
      let z = pByTag (m % 3) in
      if pEq ((x <> y) <> z) (x <> (y <> z)) then assoc m else False

  units :: Nat -> Bool
  units n =
    if n == 0 then True
    else
      let x = pByTag (n - 1) in
      if pEq (mempty <> x) x then
        if pEq (x <> mempty) x then units (n - 1) else False
      else False

  main :: Int
  main =
    let c1 = assoc 27 in
    let c2 = units 3 in
    let c3 = eqListNat (pfst ((listByTag 1, LT) <> (listByTag 2, GT))) (listByTag 1 <> listByTag 2) in
    let c4 = eqOrd (psnd ((listByTag 1, GT) <> (listByTag 2, LT))) GT in
    if c1 then (if c2 then (if c3 then (if c4 then 0 else 4) else 3) else 2) else 1
"#,
        0,
    );
}

// -------------------------------------------------------------- mconcat

#[test]
fn mconcat_folds_through_the_identity_for_strings_lists_and_sums() {
    // `mconcat` is the class-generic fold: the empty list gives the
    // instance's `mempty` (the identity), a non-empty one gives the
    // running `<>`. Observed for String, List Nat (append) and Sum.
    expect_exit(
        "mconcat",
        r#"
  main :: Int
  main =
    let emptyStr = mconcat (Nil :: List String) in
    let str = mconcat (Cons (nat-to-string 1) (Cons (nat-to-string 2) Nil)) in
    let emptyList = mconcat (Nil :: List (List Nat)) in
    let appended = mconcat (Cons (listByTag 1) (Cons (listByTag 2) Nil)) in
    let emptySum = mconcat (Nil :: List Sum) in
    let summed = mconcat (Cons (Sum 2) (Cons (Sum 3) Nil)) in
    let c1 = emptyStr == empty-string in
    let c2 = str == nat-to-string 12 in
    let c3 = eqListNat emptyList Nil in
    let c4 = eqListNat appended (listByTag 1 <> listByTag 2) in
    let c5 = nat-cmp (fromSum emptySum) 0 == 0 in
    let c6 = nat-cmp (fromSum summed) 5 == 0 in
    if c1 then (if c2 then (if c3 then (if c4 then (if c5 then (if c6 then 0 else 6) else 5) else 4) else 3) else 2) else 1
"#,
        0,
    );
}

// ------------------------------------------------------------ All / Any

#[test]
fn all_and_any_fold_bool_lists_through_mconcat() {
    // The two boolean monoids: `All` (conjunction, identity True) and
    // `Any` (disjunction, identity False). The empty list reads through
    // `mconcat` as the identity, a mixed list shows each operation.
    expect_exit(
        "bool-monoids",
        r#"
  main :: Int
  main =
    let allEmpty = fromAll (mconcat (Nil :: List All)) in
    let anyEmpty = fromAny (mconcat (Nil :: List Any)) in
    let allTrue = fromAll (mconcat (Cons (All True) (Cons (All True) Nil))) in
    let allMixed = fromAll (mconcat (Cons (All True) (Cons (All False) Nil))) in
    let anyFalse = fromAny (mconcat (Cons (Any False) (Cons (Any False) Nil))) in
    let anyMixed = fromAny (mconcat (Cons (Any False) (Cons (Any True) Nil))) in
    let c1 = allEmpty in
    let c2 = not anyEmpty in
    let c3 = allTrue in
    let c4 = not allMixed in
    let c5 = not anyFalse in
    let c6 = anyMixed in
    let c7 = not (fromAll (All True <> All False)) in
    let c8 = fromAny (Any False <> Any True) in
    let c9 = fromAll (mempty) in
    let c10 = not (fromAny (mempty)) in
    if c1 then (if c2 then (if c3 then (if c4 then (if c5 then (if c6 then (if c7 then (if c8 then (if c9 then (if c10 then 0 else 10) else 9) else 8) else 7) else 6) else 5) else 4) else 3) else 2) else 1
"#,
        0,
    );
}

// ------------------------------------------------------ sconcat / stimes

#[test]
fn sconcat_and_stimes_combine_a_carrier() {
    // `sconcat` is the list-level `<>` with the empty case as `None`;
    // `stimes` repeats. A non-commutative carrier (List Nat) shows the
    // order is preserved, and Sum gives the numeric reading.
    expect_exit(
        "sconcat-stimes",
        r#"
  optSumIs :: Nat -> Option Sum -> Bool
  optSumIs n o = case o of
    None -> False
    Some s -> nat-cmp (fromSum s) n == 0

  isNone :: forall a. Option a -> Bool
  isNone o = case o of
    None -> True
    Some _ -> False

  optListIs :: Option (List Nat) -> List Nat -> Bool
  optListIs o xs = case o of
    None -> False
    Some ys -> eqListNat ys xs

  main :: Int
  main =
    let c1 = isNone (sconcat (Nil :: List Sum)) in
    let c2 = optSumIs 6 (sconcat (Cons (Sum 1) (Cons (Sum 2) (Cons (Sum 3) Nil)))) in
    let c3 = optListIs (sconcat (Cons (listByTag 0) (Cons (listByTag 1) (Cons (listByTag 2) Nil)))) (listByTag 0 <> (listByTag 1 <> listByTag 2)) in
    let c4 = nat-cmp (fromSum (stimes 0 (Sum 5))) 5 == 0 in
    let c5 = nat-cmp (fromSum (stimes 1 (Sum 5))) 5 == 0 in
    let c6 = nat-cmp (fromSum (stimes 3 (Sum 2))) 6 == 0 in
    let c7 = eqListNat (stimes 3 (Cons 1 Nil)) (Cons 1 (Cons 1 (Cons 1 Nil))) in
    if c1 then (if c2 then (if c3 then (if c4 then (if c5 then (if c6 then (if c7 then 0 else 7) else 6) else 5) else 4) else 3) else 2) else 1
"#,
        0,
    );
}

// ------------------------------------ the re-pointed callers (behaviour)

#[test]
fn string_concat_all_still_concatenates_in_order() {
    // `string-concat-all` (Data.String) is now `mconcat`; the same
    // output as before, since String's `<>` is concatenation and its
    // identity is the empty string.
    expect_exit(
        "string-concat-all",
        r#"
  main :: Int
  main =
    let empty = string-concat-all (Nil :: List String) in
    let three = string-concat-all (Cons (nat-to-string 1) (Cons (nat-to-string 2) (Cons (nat-to-string 3) Nil))) in
    let c1 = empty == empty-string in
    let c2 = three == nat-to-string 123 in
    if c1 then (if c2 then 0 else 2) else 1
"#,
        0,
    );
}

#[test]
fn regex_escape_goes_through_mconcat_behaviour_preserving() {
    // `regex-escape` (Text.Regex) used a private `concat-strings`; it
    // now calls `mconcat`. The escaped output is unchanged: a
    // metacharacter gains a leading backslash, plain characters pass
    // through, and the empty input is the empty string.
    let main = r#"
module Main where

  import Text.Regex

  main :: Int
  main =
    let plain = regex-escape "abc" in
    let dotted = regex-escape "a.b" in
    let empty = regex-escape "" in
    let c1 = plain == "abc" in
    let c2 = dotted == "a\\.b" in
    let c3 = nat-cmp (string-byte-length dotted) 4 == 0 in
    let c4 = empty == empty-string in
    if c1 then (if c2 then (if c3 then (if c4 then 0 else 4) else 3) else 2) else 1
"#;
    expect_regex_exit("regex-escape", main, 0);
}

// ------------------------------------------- the real files as project

#[test]
fn modules_load_as_a_multi_file_project() {
    // The real `compiler/prelude/Algebra/*.cinch` files, consumed the
    // way a project consumes them: `import Algebra.Semigroup` /
    // `import Algebra.Monoid` resolved from beside the entry file
    // (decision D42).
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_algebra_proj_{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    std::fs::write(dir.join("Semigroup.cinch"), SEMIGROUP).expect("write Semigroup");
    std::fs::write(dir.join("Monoid.cinch"), MONOID).expect("write Monoid");
    let main = main_module(
        r#"
  main :: Int
  main =
    let xs = Cons 1 (Cons 2 Nil) in
    let ys = Cons 3 Nil in
    let c1 = eqListNat ((xs <> ys) <> xs) (xs <> (ys <> xs)) in
    let c2 = eqListNat (mempty <> xs) xs in
    let c3 = nat-cmp (fromSum (Sum 2 <> Sum 3)) 5 == 0 in
    let c4 = nat-cmp (applyEndo (Endo (\n -> n + 1) <> Endo (\n -> n * 2)) 5) 11 == 0 in
    if c1 then (if c2 then (if c3 then (if c4 then 0 else 4) else 3) else 2) else 1
"#,
    );
    std::fs::write(dir.join("Main.cinch"), main).expect("write Main");
    let code = driver(Mode::Run, &dir.join("Main.cinch"), None).expect("multi-file run");
    assert_eq!(code, 0);
}
