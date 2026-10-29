//! Function.Combinators / Relation.Ord / Data.Bool consumer semantics.
//!
//! Each test compiles and runs a real program through the CLI pipeline
//! (`cinch::driver::{driver, Mode}`); the prelude is embedded, so the
//! project is a single `Main.cinch` that reaches the names through the
//! implicit surface (all three homes are re-exported by the facade).
//! Observed behaviour is the process exit code: each `main` returns 0
//! on success and a bitmask of the failed checks otherwise (check k
//! contributes bit 2^(k-1)), the pattern the sibling library tests use.
//! Every numeric literal is annotated where its type is not fixed by a
//! neighbouring operand, so no check rides on literal defaulting.
//!
//! Covered:
//!   Function.Combinators: `flip` reversing a non-commutative function,
//!     `on` comparing through a projection (and combining through one),
//!     the `apply-to` reverse-application pipe, `apply-n-times` at 0 and
//!     at n, `both` fan-out, and the pair projections `fst`/`snd`/
//!     `swap`/`curry`/`uncurry` with their round trips.
//!   Relation.Ord: `comparing` sorting a list of pairs by a key (with a
//!     local insertion sort consuming the `Ordering` comparator) and
//!     `Down` reversing an order, including through `comparing`.
//!   Data.Bool: `bool` on both branches.

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_funcord_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    std::fs::write(dir.join("Main.cinch"), main_src).expect("write Main.cinch");
    match driver(Mode::Run, &dir.join("Main.cinch"), None) {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_ok(name: &str, main_src: &str) {
    let code = run_main(name, main_src);
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

#[test]
fn flip_reverses_a_non_commutative_function() {
    expect_ok(
        "flip",
        r#"module Main where
  main :: Int
  main =
    let c1 = if flip (-) (3 :: Int) 10 == 7 then 0 else 1 in
    let c2 = if flip (-) (10 :: Int) 3 == 0 - 7 then 0 else 2 in
    let c3 = if flip (\a b -> a * 10 + b) (3 :: Int) 4 == 43 then 0 else 4 in
    let c4 = if flip const (1 :: Int) (2 :: Int) == 2 then 0 else 8 in
    let c5 = if const (1 :: Int) (2 :: Int) == 1 then 0 else 16 in
    c1 + c2 + c3 + c4 + c5
"#,
    );
}

#[test]
fn on_compares_through_a_projection() {
    expect_ok(
        "on",
        r#"module Main where
  main :: Int
  main =
    let p = (1 :: Int, 9 :: Int) in
    let q = (2 :: Int, 0) in
    let c1 = if on (<) fst p q then 0 else 1 in
    let c2 = if on (<) fst q p then 2 else 0 in
    let c3 = if on (<) snd p q then 4 else 0 in
    let c4 = if on (<) snd q p then 0 else 8 in
    let c5 = if on (+) fst p q == 3 then 0 else 16 in
    c1 + c2 + c3 + c4 + c5
"#,
    );
}

#[test]
fn apply_to_is_reverse_application() {
    expect_ok(
        "apply-to",
        r#"module Main where
  double :: Int -> Int
  double x = x * 2

  main :: Int
  main =
    let c1 = if apply-to (5 :: Int) (\x -> x + 1) == 6 then 0 else 1 in
    let c2 = if apply-to (3 :: Int) double == 6 then 0 else 2 in
    let c3 = if apply-to (3 :: Int) double == double 3 then 0 else 4 in
    let c4 = if apply-to True not == False then 0 else 8 in
    c1 + c2 + c3 + c4
"#,
    );
}

#[test]
fn apply_n_times_at_zero_and_at_n() {
    expect_ok(
        "apply-n-times",
        r#"module Main where
  main :: Int
  main =
    let c1 = if apply-n-times 0 (\x -> x + 1) (5 :: Int) == 5 then 0 else 1 in
    let c2 = if apply-n-times 3 (\x -> x + 1) (5 :: Int) == 8 then 0 else 2 in
    let c3 = if apply-n-times 3 (\x -> x * 2) (1 :: Int) == 8 then 0 else 4 in
    let c4 = if apply-n-times 0 (\x -> x * 2) (7 :: Int) == 7 then 0 else 8 in
    let c5 = if apply-n-times 1 (\x -> x * 2) (7 :: Int) == 14 then 0 else 16 in
    c1 + c2 + c3 + c4 + c5
"#,
    );
}

#[test]
fn both_fans_out() {
    expect_ok(
        "both",
        r#"module Main where
  main :: Int
  main =
    let c1 = if fst (both (\x -> x + 1) (\x -> x * 2) (5 :: Int)) == 6 then 0 else 1 in
    let c2 = if snd (both (\x -> x + 1) (\x -> x * 2) (5 :: Int)) == 10 then 0 else 2 in
    c1 + c2
"#,
    );
}

#[test]
fn tuple_projections_and_round_trips() {
    expect_ok(
        "tuples",
        r#"module Main where
  addp :: (Int, Int) -> Int
  addp p = case p of
    (a, b) -> a + b

  add :: Int -> Int -> Int
  add a b = a + b

  main :: Int
  main =
    let p = (1 :: Int, 2 :: Int) in
    let c1 = if fst p == 1 then 0 else 1 in
    let c2 = if snd p == 2 then 0 else 2 in
    let c3 = if fst (swap p) == 2 then 0 else 4 in
    let c4 = if snd (swap p) == 1 then 0 else 8 in
    let c5 = if curry addp 3 4 == addp (3, 4) then 0 else 16 in
    let c6 = if uncurry add (3, 4) == add 3 4 then 0 else 32 in
    let c7 = if uncurry (curry addp) (5, 6) == addp (5, 6) then 0 else 64 in
    let c8 = if curry (uncurry add) 7 8 == add 7 8 then 0 else 128 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8
"#,
    );
}

#[test]
fn comparing_sorts_by_key() {
    expect_ok(
        "comparing",
        r#"module Main where
  insert-ord :: forall a. (a -> a -> Ordering) -> a -> List a -> List a
  insert-ord cmp x xs = case xs of
    Nil -> Cons x Nil
    Cons y ys -> case cmp x y of
      GT -> Cons y (insert-ord cmp x ys)
      _ -> Cons x xs

  sort-ord :: forall a. (a -> a -> Ordering) -> List a -> List a
  sort-ord cmp xs = case xs of
    Nil -> Nil
    Cons y ys -> insert-ord cmp y (sort-ord cmp ys)

  same-pair :: (Int, Int) -> (Int, Int) -> Bool
  same-pair x y = if fst x == fst y then snd x == snd y else False

  same-pairs :: List (Int, Int) -> List (Int, Int) -> Bool
  same-pairs a b = case a of
    Nil -> case b of
      Nil -> True
      Cons _ _ -> False
    Cons x xs -> case b of
      Nil -> False
      Cons y ys -> if same-pair x y then same-pairs xs ys else False

  firsts :: List (Int, Int) -> List Int
  firsts xs = case xs of
    Nil -> Nil
    Cons p ps -> Cons (fst p) (firsts ps)

  same-ints :: List Int -> List Int -> Bool
  same-ints a b = case a of
    Nil -> case b of
      Nil -> True
      Cons _ _ -> False
    Cons x xs -> case b of
      Nil -> False
      Cons y ys -> if x == y then same-ints xs ys else False

  -- `Ordering` ships no `Eq` (Data.Ordering), so the three-way result is
  -- compared by matching.
  ord-is :: Ordering -> Ordering -> Bool
  ord-is a b = case a of
    LT -> case b of
      LT -> True
      _ -> False
    EQ -> case b of
      EQ -> True
      _ -> False
    GT -> case b of
      GT -> True
      _ -> False

  main :: Int
  main =
    let ps = Cons (3 :: Int, 30) (Cons (1, 10) (Cons (2, 20) Nil)) in
    let asc = sort-ord (comparing fst) ps in
    let desc = sort-ord (comparing (\p -> Down (fst p))) ps in
    let c1 = if same-pairs asc (Cons (1, 10) (Cons (2, 20) (Cons (3, 30) Nil))) then 0 else 1 in
    let c2 = if same-pairs desc (Cons (3, 30) (Cons (2, 20) (Cons (1, 10) Nil))) then 0 else 2 in
    let c3 = if same-ints (firsts asc) (Cons 1 (Cons 2 (Cons 3 Nil))) then 0 else 4 in
    let c4 = if same-ints (firsts desc) (Cons 3 (Cons 2 (Cons 1 Nil))) then 0 else 8 in
    let c5 = if ord-is (comparing fst (1 :: Int, 10 :: Int) (1 :: Int, 20 :: Int)) EQ then 0 else 16 in
    let c6 = if ord-is (comparing fst (1 :: Int, 10 :: Int) (2 :: Int, 20 :: Int)) LT then 0 else 32 in
    let c7 = if ord-is (comparing fst (2 :: Int, 20 :: Int) (1 :: Int, 10 :: Int)) GT then 0 else 64 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7
"#,
    );
}

#[test]
fn down_reverses_an_order() {
    expect_ok(
        "down",
        r#"module Main where
  main :: Int
  main =
    let c1 = if Down (5 :: Int) < Down 3 then 0 else 1 in
    let c2 = if Down (3 :: Int) < Down 5 then 2 else 0 in
    let c3 = if Down (3 :: Int) > Down 5 then 0 else 4 in
    let c4 = if Down (5 :: Int) > Down 3 then 8 else 0 in
    let c5 = if Down (4 :: Int) <= Down 4 then 0 else 16 in
    let c6 = if Down (4 :: Int) == Down 4 then 0 else 32 in
    let c7 = if Down (4 :: Int) == Down 5 then 64 else 0 in
    let c8 = if Down (4 :: Int) /= Down 5 then 0 else 128 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8
"#,
    );
}

#[test]
fn bool_folds_both_branches() {
    expect_ok(
        "bool",
        r#"module Main where
  main :: Int
  main =
    let c1 = if bool (0 :: Int) 1 True == 1 then 0 else 1 in
    let c2 = if bool (0 :: Int) 1 False == 0 then 0 else 2 in
    let c3 = if bool True False True == False then 0 else 4 in
    let c4 = if bool True False False == True then 0 else 8 in
    let c5 = if bool (2 :: Nat) 3 True == 3 then 0 else 16 in
    c1 + c2 + c3 + c4 + c5
"#,
    );
}
