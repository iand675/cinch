//! Data.List consumer semantics: the concrete list combinators GHC
//! `base` has and the prelude now carries (the four partial projections
//! `head`/`tail`/`last`/`init`, `zip`/`zip-with`/`unzip`, `split-at`,
//! `span`/`break`, `partition`, `find`/`find-index`/`elem-index`,
//! `lookup`, the bounded generators `iterate`/`unfold`/`cycle`,
//! `scanl`/`scanr`, `fold-left1`/`fold-right1`, `transpose`,
//! `sort`/`sort-by`, `group-by`, `intersperse`/`intercalate`, `nub` and
//! `subsequences`).
//!
//! Every program is a real `Main` module compiled and run through the
//! same driver the CLI uses (`run_source`), against the embedded
//! prelude: the facade re-exports `Data.List`, so its names need no
//! `import`. Each check is a top-level `check-*` function returning 0 or
//! its bit and `main` sums them, so a nonzero exit code is a bitmask
//! naming the failed checks (check k contributes bit 2^(k-1)), which
//! keeps the total inside the 8-bit exit status. The checks are
//! top-level because a `case` in a `let` binder's right-hand side must
//! indent past the binder's column under the layout rules.
//!
//! Structural equality is written out per carrier (`Data.Option` carries
//! no `Eq` instance, so an `Option (List Nat)` has none either), and the
//! two comparators the sort checks need are defined here.

use cinch::driver::run_source;

/// Helpers every program shares: structural equality per carrier and the
/// comparators and tuple projections the checks use.
const HELPERS: &str = r#"
  eq-list-nat :: List Nat -> List Nat -> Bool
  eq-list-nat xs ys = case xs of
    Nil -> case ys of
      Nil -> True
      Cons _ _ -> False
    Cons a ar -> case ys of
      Nil -> False
      Cons b bs -> if a == b then eq-list-nat ar bs else False

  eq-opt-nat :: Option Nat -> Option Nat -> Bool
  eq-opt-nat x y = case x of
    None -> case y of
      None -> True
      Some _ -> False
    Some a -> case y of
      None -> False
      Some b -> a == b

  eq-opt-int :: Option Int -> Option Int -> Bool
  eq-opt-int x y = case x of
    None -> case y of
      None -> True
      Some _ -> False
    Some a -> case y of
      None -> False
      Some b -> a == b

  eq-opt-list-nat :: Option (List Nat) -> Option (List Nat) -> Bool
  eq-opt-list-nat x y = case x of
    None -> case y of
      None -> True
      Some _ -> False
    Some a -> case y of
      None -> False
      Some b -> eq-list-nat a b

  eq-list-list-nat :: List (List Nat) -> List (List Nat) -> Bool
  eq-list-list-nat xs ys = case xs of
    Nil -> case ys of
      Nil -> True
      Cons _ _ -> False
    Cons a ar -> case ys of
      Nil -> False
      Cons b bs -> if eq-list-nat a b then eq-list-list-nat ar bs else False

  eq-pair-nat :: (Nat, Nat) -> (Nat, Nat) -> Bool
  eq-pair-nat x y = case x of
    (a, b) -> case y of
      (c, d) -> if a == c then b == d else False

  eq-list-pair-nat :: List (Nat, Nat) -> List (Nat, Nat) -> Bool
  eq-list-pair-nat xs ys = case xs of
    Nil -> case ys of
      Nil -> True
      Cons _ _ -> False
    Cons a ar -> case ys of
      Nil -> False
      Cons b bs -> if eq-pair-nat a b then eq-list-pair-nat ar bs else False

  eq-pair-list-nat :: (List Nat, List Nat) -> (List Nat, List Nat) -> Bool
  eq-pair-list-nat x y = case x of
    (a, b) -> case y of
      (c, d) -> if eq-list-nat a c then eq-list-nat b d else False

  pfst :: forall a b. (a, b) -> a
  pfst p = case p of
    (a, b) -> a

  nat-cmp3 :: Nat -> Nat -> Ordering
  nat-cmp3 x y = case x < y of
    True -> LT
    False -> case x > y of
      True -> GT
      False -> EQ

  rev-cmp :: Nat -> Nat -> Ordering
  rev-cmp x y = case x < y of
    True -> GT
    False -> case x > y of
      True -> LT
      False -> EQ

  pair-first-cmp :: (Nat, Nat) -> (Nat, Nat) -> Ordering
  pair-first-cmp x y = nat-cmp3 (pfst x) (pfst y)
"#;

fn expect_ok(name: &str, body: &str) {
    let src = format!("module Main where\n{HELPERS}{body}");
    match run_source(&src) {
        Ok(0) => {}
        Ok(code) => panic!("{name}: check bitmask {code} (bit 2^(k-1) per failed check k)"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

#[test]
fn list_partial_projections() {
    expect_ok(
        "list-projections",
        r#"
  l3 :: List Nat
  l3 = Cons 1 (Cons 2 (Cons 3 Nil))

  l1 :: List Nat
  l1 = Cons 7 Nil

  nil :: List Nat
  nil = Nil

  check1 :: Int
  check1 = if eq-opt-nat (head l3) (Some 1) then 0 else 1

  check2 :: Int
  check2 = if eq-opt-nat (head nil) None then 0 else 2

  check3 :: Int
  check3 = if eq-opt-list-nat (tail l3) (Some (Cons 2 (Cons 3 Nil))) then 0 else 4

  check4 :: Int
  check4 = if eq-opt-list-nat (tail nil) None then 0 else 8

  check5 :: Int
  check5 = if eq-opt-nat (last l3) (Some 3) then 0 else 16

  check6 :: Int
  check6 = if eq-opt-nat (last nil) None then 0 else 32

  check7 :: Int
  check7 = if eq-opt-list-nat (init l3) (Some (Cons 1 (Cons 2 Nil))) then 0 else 64

  check8 :: Int
  check8 =
    if eq-opt-list-nat (init nil) None then
      if eq-opt-list-nat (init l1) None then
        if eq-opt-list-nat (tail l1) (Some Nil) then 0 else 128
      else 128
    else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8
"#,
    );
}

#[test]
fn list_zip_unzip_and_split_at() {
    expect_ok(
        "list-zip-split",
        r#"
  l3 :: List Nat
  l3 = Cons 1 (Cons 2 (Cons 3 Nil))

  l2 :: List Nat
  l2 = Cons 10 (Cons 20 Nil)

  nil :: List Nat
  nil = Nil

  check1 :: Int
  check1 = if eq-list-nat (zip-with (\x y -> x + y) l3 l2) (Cons 11 (Cons 22 Nil)) then 0 else 1

  check2 :: Int
  check2 = if eq-list-pair-nat (zip l3 l2) (Cons (1, 10) (Cons (2, 20) Nil)) then 0 else 2

  check3 :: Int
  check3 = if eq-list-pair-nat (zip l2 l3) (Cons (10, 1) (Cons (20, 2) Nil)) then 0 else 4

  check4 :: Int
  check4 =
    if eq-list-pair-nat (zip l3 nil) Nil then
      if eq-list-pair-nat (zip nil l3) Nil then 0 else 8
    else 8

  check5 :: Int
  check5 =
    if eq-pair-list-nat (unzip (Cons (1, 10) (Cons (2, 20) Nil))) (Cons 1 (Cons 2 Nil), Cons 10 (Cons 20 Nil)) then 0 else 16

  check6 :: Int
  check6 = if eq-pair-list-nat (split-at 0 l3) (Nil, l3) then 0 else 32

  check7 :: Int
  check7 = if eq-pair-list-nat (split-at 2 l3) (Cons 1 (Cons 2 Nil), Cons 3 Nil) then 0 else 64

  check8 :: Int
  check8 =
    if eq-pair-list-nat (split-at 5 l3) (l3, Nil) then
      if eq-pair-list-nat (split-at 1 nil) (Nil, Nil) then 0 else 128
    else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8
"#,
    );
}

#[test]
fn list_span_break_partition() {
    expect_ok(
        "list-span-break",
        r#"
  l5 :: List Nat
  l5 = Cons 1 (Cons 2 (Cons 3 (Cons 4 (Cons 5 Nil))))

  nil :: List Nat
  nil = Nil

  check1 :: Int
  check1 = if eq-pair-list-nat (span (\x -> x < 3) l5) (Cons 1 (Cons 2 Nil), Cons 3 (Cons 4 (Cons 5 Nil))) then 0 else 1

  check2 :: Int
  check2 = if eq-pair-list-nat (span (\x -> x < 9) l5) (l5, Nil) then 0 else 2

  check3 :: Int
  check3 = if eq-pair-list-nat (span (\x -> x < 0) l5) (Nil, l5) then 0 else 4

  check4 :: Int
  check4 = if eq-pair-list-nat (break (\x -> x > 3) l5) (Cons 1 (Cons 2 (Cons 3 Nil)), Cons 4 (Cons 5 Nil)) then 0 else 8

  check5 :: Int
  check5 = if eq-pair-list-nat (break (\x -> x == 5) l5) (Cons 1 (Cons 2 (Cons 3 (Cons 4 Nil))), Cons 5 Nil) then 0 else 16

  check6 :: Int
  check6 = if eq-pair-list-nat (partition (\x -> True) l5) (l5, Nil) then 0 else 32

  check7 :: Int
  check7 = if eq-pair-list-nat (partition (\x -> False) l5) (Nil, l5) then 0 else 64

  check8 :: Int
  check8 =
    if eq-pair-list-nat (partition (\x -> x < 3) l5) (Cons 1 (Cons 2 Nil), Cons 3 (Cons 4 (Cons 5 Nil))) then
      if eq-pair-list-nat (span (\x -> x == 1) l5) (Cons 1 Nil, Cons 2 (Cons 3 (Cons 4 (Cons 5 Nil)))) then
        if eq-pair-list-nat (break (\x -> True) nil) (Nil, Nil) then 0 else 128
      else 128
    else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8
"#,
    );
}

#[test]
fn list_find_index_and_lookup() {
    expect_ok(
        "list-find-index",
        r#"
  l5 :: List Nat
  l5 = Cons 1 (Cons 2 (Cons 3 (Cons 4 (Cons 5 Nil))))

  pairs :: List (Nat, Nat)
  pairs = Cons (1, 10) (Cons (2, 20) Nil)

  repeats :: List (Nat, Nat)
  repeats = Cons (1, 10) (Cons (1, 20) Nil)

  check1 :: Int
  check1 = if eq-opt-nat (find (\x -> x > 3) l5) (Some 4) then 0 else 1

  check2 :: Int
  check2 = if eq-opt-nat (find (\x -> x > 9) l5) None then 0 else 2

  check3 :: Int
  check3 = if eq-opt-nat (find-index (\x -> x == 3) l5) (Some 2) then 0 else 4

  check4 :: Int
  check4 = if eq-opt-nat (find-index (\x -> x == 9) l5) None then 0 else 8

  check5 :: Int
  check5 = if eq-opt-nat (elem-index 4 l5) (Some 3) then 0 else 16

  check6 :: Int
  check6 = if eq-opt-nat (elem-index 9 l5) None then 0 else 32

  check7 :: Int
  check7 = if eq-opt-nat (lookup 2 pairs) (Some 20) then 0 else 64

  check8 :: Int
  check8 =
    if eq-opt-nat (lookup 3 pairs) None then
      if eq-opt-nat (lookup 1 repeats) (Some 10) then 0 else 128
    else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8
"#,
    );
}

#[test]
fn list_scans_and_folds_with_an_element_seed() {
    expect_ok(
        "list-scans",
        r#"
  l3 :: List Nat
  l3 = Cons 1 (Cons 2 (Cons 3 Nil))

  nil :: List Nat
  nil = Nil

  li3 :: List Int
  li3 = Cons 1 (Cons 2 (Cons 3 Nil))

  check1 :: Int
  check1 = if eq-list-nat (scanl (\acc x -> acc + x) 0 l3) (Cons 0 (Cons 1 (Cons 3 (Cons 6 Nil)))) then 0 else 1

  check2 :: Int
  check2 = if eq-list-nat (scanl (\acc x -> acc + x) 0 nil) (Cons 0 Nil) then 0 else 2

  check3 :: Int
  check3 = if eq-list-nat (scanr (\x acc -> x + acc) 0 l3) (Cons 6 (Cons 5 (Cons 3 (Cons 0 Nil)))) then 0 else 4

  check4 :: Int
  check4 = if eq-list-nat (scanr (\x acc -> x + acc) 0 nil) (Cons 0 Nil) then 0 else 8

  check5 :: Int
  check5 =
    if eq-opt-nat (fold-left1 (\x y -> x + y) (Cons 5 Nil)) (Some 5) then
      if eq-opt-nat (fold-right1 (\x y -> x + y) (Cons 5 Nil)) (Some 5) then 0 else 16
    else 16

  check6 :: Int
  check6 =
    if eq-opt-nat (fold-left1 (\x y -> x + y) nil) None then
      if eq-opt-nat (fold-right1 (\x y -> x + y) nil) None then 0 else 32
    else 32

  check7 :: Int
  check7 = if eq-opt-int (fold-left1 (\x y -> x - y) li3) (Some (int-neg 4)) then 0 else 64

  check8 :: Int
  check8 = if eq-opt-int (fold-right1 (\x y -> x - y) li3) (Some 2) then 0 else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8
"#,
    );
}

#[test]
fn list_sort_group_and_transpose() {
    expect_ok(
        "list-sort-group",
        r#"
  l3 :: List Nat
  l3 = Cons 3 (Cons 1 (Cons 2 Nil))

  l5 :: List Nat
  l5 = Cons 5 (Cons 4 (Cons 3 (Cons 2 (Cons 1 Nil))))

  nil :: List Nat
  nil = Nil

  runs :: List Nat
  runs = Cons 1 (Cons 1 (Cons 2 (Cons 3 (Cons 3 Nil))))

  tagged :: List (Nat, Nat)
  tagged = Cons (2, 1) (Cons (1, 9) (Cons (1, 8) Nil))

  tagged4 :: List (Nat, Nat)
  tagged4 = Cons (3, 0) (Cons (2, 1) (Cons (2, 2) (Cons (1, 3) Nil)))

  ragged :: List (List Nat)
  ragged = Cons (Cons 1 (Cons 2 (Cons 3 Nil))) (Cons (Cons 4 (Cons 5 Nil)) (Cons (Cons 6 Nil) Nil))

  short :: List (List Nat)
  short = Cons (Cons 1 (Cons 2 Nil)) (Cons (Cons 3 Nil) Nil)

  check1 :: Int
  check1 =
    if eq-list-nat (sort l3) (Cons 1 (Cons 2 (Cons 3 Nil))) then
      if eq-list-nat (sort l5) (Cons 1 (Cons 2 (Cons 3 (Cons 4 (Cons 5 Nil))))) then
        if eq-list-nat (sort nil) Nil then 0 else 1
      else 1
    else 1

  check2 :: Int
  check2 = if eq-list-nat (sort-by nat-cmp3 l3) (Cons 1 (Cons 2 (Cons 3 Nil))) then 0 else 2

  check3 :: Int
  check3 = if eq-list-nat (sort-by rev-cmp l3) (Cons 3 (Cons 2 (Cons 1 Nil))) then 0 else 4

  check4 :: Int
  check4 =
    if eq-list-pair-nat (sort-by pair-first-cmp tagged) (Cons (1, 9) (Cons (1, 8) (Cons (2, 1) Nil))) then
      if eq-list-pair-nat (sort-by pair-first-cmp tagged4) (Cons (1, 3) (Cons (2, 1) (Cons (2, 2) (Cons (3, 0) Nil)))) then 0 else 8
    else 8

  check5 :: Int
  check5 = if eq-list-list-nat (group-by (\x y -> x == y) runs) (Cons (Cons 1 (Cons 1 Nil)) (Cons (Cons 2 Nil) (Cons (Cons 3 (Cons 3 Nil)) Nil))) then 0 else 16

  check6 :: Int
  check6 = if eq-list-list-nat (group-by (\x y -> x == y) nil) Nil then 0 else 32

  check7 :: Int
  check7 = if eq-list-list-nat (transpose ragged) (Cons (Cons 1 (Cons 4 (Cons 6 Nil))) (Cons (Cons 2 (Cons 5 Nil)) (Cons (Cons 3 Nil) Nil))) then 0 else 64

  check8 :: Int
  check8 =
    if eq-list-list-nat (transpose short) (Cons (Cons 1 (Cons 3 Nil)) (Cons (Cons 2 Nil) Nil)) then
      if eq-list-list-nat (transpose (Nil :: List (List Nat))) Nil then 0 else 128
    else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8
"#,
    );
}

#[test]
fn list_intersperse_intercalate_and_nub() {
    expect_ok(
        "list-join-nub",
        r#"
  l3 :: List Nat
  l3 = Cons 1 (Cons 2 (Cons 3 Nil))

  nil :: List Nat
  nil = Nil

  singleton :: List (List Nat)
  singleton = Cons (Cons 5 Nil) Nil

  dups :: List Nat
  dups = Cons 3 (Cons 1 (Cons 3 (Cons 2 (Cons 1 Nil))))

  same :: List Nat
  same = Cons 1 (Cons 1 (Cons 1 Nil))

  check1 :: Int
  check1 = if eq-list-nat (intersperse 0 nil) Nil then 0 else 1

  check2 :: Int
  check2 = if eq-list-nat (intersperse 0 (Cons 5 Nil)) (Cons 5 Nil) then 0 else 2

  check3 :: Int
  check3 = if eq-list-nat (intersperse 0 l3) (Cons 1 (Cons 0 (Cons 2 (Cons 0 (Cons 3 Nil))))) then 0 else 4

  check4 :: Int
  check4 = if eq-list-nat (intercalate (Cons 0 Nil) (Nil :: List (List Nat))) Nil then 0 else 8

  check5 :: Int
  check5 = if eq-list-nat (intercalate (Cons 0 Nil) singleton) (Cons 5 Nil) then 0 else 16

  check6 :: Int
  check6 = if eq-list-nat (intercalate (Cons 0 Nil) (Cons (Cons 1 (Cons 2 Nil)) (Cons (Cons 3 Nil) Nil))) (Cons 1 (Cons 2 (Cons 0 (Cons 3 Nil)))) then 0 else 32

  check7 :: Int
  check7 = if eq-list-nat (nub dups) (Cons 3 (Cons 1 (Cons 2 Nil))) then 0 else 64

  check8 :: Int
  check8 =
    if eq-list-nat (nub nil) Nil then
      if eq-list-nat (nub same) (Cons 1 Nil) then 0 else 128
    else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8
"#,
    );
}

#[test]
fn list_bounded_generators_and_subsequences() {
    expect_ok(
        "list-generators",
        r#"
  nil :: List Nat
  nil = Nil

  count-up :: Nat -> Option (Nat, Nat)
  count-up s = if s > 2 then None else Some (s, s + 1)

  never :: Nat -> Option (Nat, Nat)
  never _ = None

  check1 :: Int
  check1 = if eq-list-nat (iterate 0 (\n -> n + 1) 0) Nil then 0 else 1

  check2 :: Int
  check2 = if eq-list-nat (iterate 4 (\n -> n + 1) 0) (Cons 0 (Cons 1 (Cons 2 (Cons 3 Nil)))) then 0 else 2

  check3 :: Int
  check3 = if eq-list-nat (cycle 0 (Cons 1 (Cons 2 Nil))) Nil then 0 else 4

  check4 :: Int
  check4 = if eq-list-nat (cycle 2 (Cons 1 (Cons 2 Nil))) (Cons 1 (Cons 2 (Cons 1 (Cons 2 Nil)))) then 0 else 8

  check5 :: Int
  check5 = if eq-list-nat (cycle 3 nil) Nil then 0 else 16

  check6 :: Int
  check6 = if eq-list-nat (unfold count-up 0) (Cons 0 (Cons 1 (Cons 2 Nil))) then 0 else 32

  check7 :: Int
  check7 = if eq-list-nat (unfold never 0) Nil then 0 else 64

  check8 :: Int
  check8 = if eq-list-list-nat (subsequences (Cons 1 (Cons 2 Nil))) (Cons Nil (Cons (Cons 1 Nil) (Cons (Cons 2 Nil) (Cons (Cons 1 (Cons 2 Nil)) Nil)))) then 0 else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8
"#,
    );
}
