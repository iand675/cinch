//! Foldable/Traversable/NonEmpty consumer semantics: the fold and
//! traversal classes over List, Option and NonEmpty, the refinement's
//! free semigroup instance (the container's own `<>` over `List`), and
//! the showcase behavior of `traverse` into `Result` (short-circuiting
//! on the FIRST error) and into `Option`.
//!
//! Each test compiles a real multi-file project: the prelude modules
//! under test (Category/Foldable.lang, Category/Traversable.lang,
//! Data/NonEmpty.lang) plus Algebra/Semigroup.lang and the
//! Category.Functor/Category.Applicative class modules the instances
//! build on, driven through the CLI pipeline like modules.rs. The
//! classes are the constructor-ary ones (`Foldable (t :: Type ->
//! Type)`, `Traversable (t :: Type -> Type)`); the sources exercise
//! only consumer-visible semantics and the classes' laws over sample
//! values:
//!   Foldable:    foldr/to-list agreement (`foldr Cons Nil (to-list xs)
//!                == to-list xs`), to-list idempotence, length/null
//!                agreement with to-list, foldl/foldr agreement.
//!   Semigroup:   associativity of `<>` on NonEmpty, and concatenation
//!                as list append on the elements.
//!   Traversable: identity (`traverse Ok == Ok` / `traverse Some ==
//!                Some`), and first-error short-circuit order for both
//!                `traverse` and `sequence`.
//!
//! Each check is a top-level `check-*` function returning 0 or its bit;
//! `main` sums them, so a nonzero exit code is a bitmask naming the
//! failed checks (check k contributes bit 2^(k-1)) and any failure mode
//! of one check shares that bit, keeping the total inside the 8-bit
//! exit status. The checks are top-level because a `case` in a `let`
//! binder's right-hand side must indent past the binder's column under
//! the layout rules.

use safelang::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const FOLDABLE: &str = include_str!("../prelude/Category/Foldable.lang");
const TRAVERSABLE: &str = include_str!("../prelude/Category/Traversable.lang");
const FUNCTOR: &str = include_str!("../prelude/Category/Functor.lang");
const APPLICATIVE: &str = include_str!("../prelude/Category/Applicative.lang");
const NONEMPTY: &str = include_str!("../prelude/Data/NonEmpty.lang");
const SEMIGROUP: &str = include_str!("../prelude/Algebra/Semigroup.lang");

fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "safelang_foldtest_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [
        ("Foldable.lang", FOLDABLE),
        ("Traversable.lang", TRAVERSABLE),
        ("Functor.lang", FUNCTOR),
        ("Applicative.lang", APPLICATIVE),
        ("NonEmpty.lang", NONEMPTY),
        ("Semigroup.lang", SEMIGROUP),
        ("Main.lang", main_src),
    ] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    match driver(Mode::Run, &dir.join("Main.lang"), None) {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_ok(name: &str, main_src: &str) {
    let code = run_main(name, main_src);
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

#[test]
fn foldable_list_semantics_and_laws() {
    expect_ok(
        "foldable-list",
        "module Main where\n\
         \x20 import Category.Foldable\n\
         \x20 same :: List Int -> List Int -> Bool\n\
         \x20 same a b = case a of\n\
         \x20   Nil -> case b of\n\
         \x20     Nil -> True\n\
         \x20     Cons y ys -> False\n\
         \x20   Cons x xs -> case b of\n\
         \x20     Nil -> False\n\
         \x20     Cons y ys -> if x == y then same xs ys else False\n\
         \x20 main :: Int\n\
         \x20 main =\n\
         \x20   let xs = Cons (1 :: Int) (Cons 2 (Cons 3 Nil)) in\n\
         \x20   let c1 = if foldr (\\x acc -> x + acc) 0 xs == 6 then 0 else 1 in\n\
         \x20   let c2 = if foldl (\\acc x -> acc + x) 0 xs == 6 then 0 else 2 in\n\
         \x20   let c3 = if same (foldr (\\v acc -> Cons v acc) Nil xs) xs then 0 else 4 in\n\
         \x20   let c4 = if same (to-list (to-list xs)) (to-list xs) then 0 else 8 in\n\
         \x20   let c5 = if length xs == 3 then 0 else 16 in\n\
         \x20   let c6 = if null xs == False then 0 else 32 in\n\
         \x20   let c7 = if length (Nil :: List Int) == 0 then 0 else 64 in\n\
         \x20   let c8 = if null (Nil :: List Int) == True then 0 else 128 in\n\
         \x20   c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8\n",
    );
}

#[test]
fn foldable_option_semantics_and_laws() {
    expect_ok(
        "foldable-option",
        "module Main where\n\
         \x20 import Category.Foldable\n\
         \x20 same :: List Int -> List Int -> Bool\n\
         \x20 same a b = case a of\n\
         \x20   Nil -> case b of\n\
         \x20     Nil -> True\n\
         \x20     Cons y ys -> False\n\
         \x20   Cons x xs -> case b of\n\
         \x20     Nil -> False\n\
         \x20     Cons y ys -> if x == y then same xs ys else False\n\
         \x20 main :: Int\n\
         \x20 main =\n\
         \x20   let s = Some (5 :: Int) in\n\
         \x20   let c1 = if foldr (\\x acc -> x + acc) 0 s == 5 then 0 else 1 in\n\
         \x20   let c2 = if foldl (\\acc x -> acc + x) 0 s == 5 then 0 else 2 in\n\
         \x20   let c3 = if same (to-list s) (Cons 5 Nil) then 0 else 4 in\n\
         \x20   let c4 = if length s == 1 then 0 else 8 in\n\
         \x20   let c5 = if null s == False then 0 else 16 in\n\
         \x20   let c6 = if same (foldr (\\v acc -> Cons v acc) Nil (to-list s)) (to-list s) then 0 else 32 in\n\
         \x20   let c7 = if foldr (\\x acc -> x + acc) 0 (None :: Option Int) == 0 then 0 else 64 in\n\
         \x20   let c8 = if same (to-list (None :: Option Int)) (Nil :: List Int) then 0 else 128 in\n\
         \x20   c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8\n",
    );
}

#[test]
fn foldable_nonempty_semantics_and_laws() {
    // The refinement's `Foldable` transfer reads the refined value as
    // the list it is: the container's own folds over the transfer
    // (checks are top-level because a `case` in a `let` binder's
    // right-hand side must indent past the binder's column).
    expect_ok(
        "foldable-nonempty",
        "module Main where\n\
         \x20 import Category.Foldable\n\
         \x20 import Data.NonEmpty\n\
         \x20 same :: List Int -> List Int -> Bool\n\
         \x20 same a b = case a of\n\
         \x20   Nil -> case b of\n\
         \x20     Nil -> True\n\
         \x20     Cons y ys -> False\n\
         \x20   Cons x xs -> case b of\n\
         \x20     Nil -> False\n\
         \x20     Cons y ys -> if x == y then same xs ys else False\n\
         \x20 check1 :: Int\n\
         \x20 check1 = case non-empty (Cons (1 :: Int) (Cons 2 (Cons 3 Nil))) of\n\
         \x20   None -> 1\n\
         \x20   Some ne -> if foldr (\\x acc -> x + acc) 0 ne == 6 then 0 else 1\n\
         \x20 check2 :: Int\n\
         \x20 check2 = case non-empty (Cons (1 :: Int) (Cons 2 (Cons 3 Nil))) of\n\
         \x20   None -> 2\n\
         \x20   Some ne -> if foldl (\\acc x -> acc + x) 0 ne == 6 then 0 else 2\n\
         \x20 check3 :: Int\n\
         \x20 check3 = case non-empty (Cons (1 :: Int) (Cons 2 (Cons 3 Nil))) of\n\
         \x20   None -> 4\n\
         \x20   Some ne -> if same (to-list ne) (Cons 1 (Cons 2 (Cons 3 Nil))) then 0 else 4\n\
         \x20 check4 :: Int\n\
         \x20 check4 = case non-empty (Cons (1 :: Int) (Cons 2 (Cons 3 Nil))) of\n\
         \x20   None -> 8\n\
         \x20   Some ne -> if length ne == 3 then 0 else 8\n\
         \x20 check5 :: Int\n\
         \x20 check5 = case non-empty (Cons (1 :: Int) (Cons 2 (Cons 3 Nil))) of\n\
         \x20   None -> 16\n\
         \x20   Some ne -> if null ne == False then 0 else 16\n\
         \x20 check6 :: Int\n\
         \x20 check6 = case non-empty (Cons (1 :: Int) (Cons 2 (Cons 3 Nil))) of\n\
         \x20   None -> 32\n\
         \x20   Some ne -> if same (foldr (\\v acc -> Cons v acc) Nil (to-list ne)) (to-list ne) then 0 else 32\n\
         \x20 check7 :: Int\n\
         \x20 check7 = case non-empty (Cons (7 :: Int) Nil) of\n\
         \x20   None -> 64\n\
         \x20   Some one -> if same (to-list one) (Cons 7 Nil) then 0 else 64\n\
         \x20 check8 :: Int\n\
         \x20 check8 = case non-empty (Cons (7 :: Int) Nil) of\n\
         \x20   None -> 128\n\
         \x20   Some one -> if length one == 1 then 0 else 128\n\
         \x20 main :: Int\n\
         \x20 main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8\n",
    );
}

#[test]
fn semigroup_nonempty_associativity_and_concatenation() {
    // The refinement's `Semigroup` transfer is the container's own `<>`
    // (list append): concatenation as list append on the elements,
    // associative over samples (the P18 law layer checks both in
    // compiler/tests/refinements.rs).
    expect_ok(
        "semigroup-nonempty",
        "module Main where\n\
         \x20 import Category.Foldable\n\
         \x20 import Data.NonEmpty\n\
         \x20 import Algebra.Semigroup\n\
         \x20 infixr 6 <>\n\
         \x20 same :: List Int -> List Int -> Bool\n\
         \x20 same a b = case a of\n\
         \x20   Nil -> case b of\n\
         \x20     Nil -> True\n\
         \x20     Cons y ys -> False\n\
         \x20   Cons x xs -> case b of\n\
         \x20     Nil -> False\n\
         \x20     Cons y ys -> if x == y then same xs ys else False\n\
         \x20 append :: List Int -> List Int -> List Int\n\
         \x20 append a b = case a of\n\
         \x20   Nil -> b\n\
         \x20   Cons x xs -> Cons x (append xs b)\n\
         \x20 check1 :: Int\n\
         \x20 check1 = case non-empty (Cons (1 :: Int) (Cons 2 Nil)) of\n\
         \x20   None -> 1\n\
         \x20   Some a -> case non-empty (Cons (3 :: Int) Nil) of\n\
         \x20     None -> 1\n\
         \x20     Some b -> if same (to-list (a <> b)) (append (to-list a) (to-list b)) then 0 else 1\n\
         \x20 check2 :: Int\n\
         \x20 check2 = case non-empty (Cons (1 :: Int) (Cons 2 Nil)) of\n\
         \x20   None -> 2\n\
         \x20   Some a -> case non-empty (Cons (3 :: Int) Nil) of\n\
         \x20     None -> 2\n\
         \x20     Some b -> case non-empty (Cons (4 :: Int) (Cons 5 Nil)) of\n\
         \x20       None -> 2\n\
         \x20       Some c -> if same (to-list ((a <> b) <> c)) (to-list (a <> (b <> c))) then 0 else 2\n\
         \x20 check3 :: Int\n\
         \x20 check3 = case non-empty (Cons (1 :: Int) (Cons 2 Nil)) of\n\
         \x20   None -> 4\n\
         \x20   Some a -> case non-empty (Cons (3 :: Int) Nil) of\n\
         \x20     None -> 4\n\
         \x20     Some b -> if same (to-list (a <> b)) (Cons 1 (Cons 2 (Cons 3 Nil))) then 0 else 4\n\
         \x20 check4 :: Int\n\
         \x20 check4 = case non-empty (Cons (1 :: Int) (Cons 2 Nil)) of\n\
         \x20   None -> 8\n\
         \x20   Some a -> case non-empty (Cons (3 :: Int) Nil) of\n\
         \x20     None -> 8\n\
         \x20     Some b -> if same (to-list (b <> a)) (Cons 3 (Cons 1 (Cons 2 Nil))) then 0 else 8\n\
         \x20 main :: Int\n\
         \x20 main = check1 + check2 + check3 + check4\n",
    );
}

#[test]
fn traverse_list_result_short_circuits_on_first_error() {
    expect_ok(
        "traverse-list-result",
        "module Main where\n\
         \x20 import Category.Foldable\n\
         \x20 import Category.Traversable\n\
         \x20 same :: List Int -> List Int -> Bool\n\
         \x20 same a b = case a of\n\
         \x20   Nil -> case b of\n\
         \x20     Nil -> True\n\
         \x20     Cons y ys -> False\n\
         \x20   Cons x xs -> case b of\n\
         \x20     Nil -> False\n\
         \x20     Cons y ys -> if x == y then same xs ys else False\n\
         \x20 half :: Int -> Result Int Int\n\
         \x20 half x = if x == 2 then Err 2 else if x == 4 then Err 4 else Ok x\n\
         \x20 bump :: Int -> Result Int Int\n\
         \x20 bump x = Ok (x + 10)\n\
         \x20 check-a :: Int\n\
         \x20 check-a =\n\
         \x20   let xs = Cons (1 :: Int) (Cons 2 (Cons 3 (Cons 4 Nil))) in\n\
         \x20   case (traverse half xs :: Result Int (List Int)) of\n\
         \x20     Err e -> if e == 2 then 0 else 1\n\
         \x20     Ok vs -> 1\n\
         \x20 check-b :: Int\n\
         \x20 check-b = case (traverse half (Cons (2 :: Int) (Cons 4 Nil)) :: Result Int (List Int)) of\n\
         \x20   Err e -> if e == 2 then 0 else 2\n\
         \x20   Ok vs -> 2\n\
         \x20 check-c :: Int\n\
         \x20 check-c =\n\
         \x20   let xs = Cons (1 :: Int) (Cons 2 (Cons 3 (Cons 4 Nil))) in\n\
         \x20   case (traverse bump xs :: Result Int (List Int)) of\n\
         \x20     Err e -> 4\n\
         \x20     Ok vs -> if same vs (Cons 11 (Cons 12 (Cons 13 (Cons 14 Nil)))) then 0 else 4\n\
         \x20 check-d :: Int\n\
         \x20 check-d =\n\
         \x20   let xs = Cons (1 :: Int) (Cons 2 (Cons 3 (Cons 4 Nil))) in\n\
         \x20   case (traverse (\\v -> Ok v) xs :: Result Int (List Int)) of\n\
         \x20     Err e -> 8\n\
         \x20     Ok vs -> if same vs xs then 0 else 8\n\
         \x20 main :: Int\n\
         \x20 main = check-a + check-b + check-c + check-d\n",
    );
}

#[test]
fn traverse_list_option_short_circuits() {
    expect_ok(
        "traverse-list-option",
        "module Main where\n\
         \x20 import Category.Foldable\n\
         \x20 import Category.Traversable\n\
         \x20 same :: List Int -> List Int -> Bool\n\
         \x20 same a b = case a of\n\
         \x20   Nil -> case b of\n\
         \x20     Nil -> True\n\
         \x20     Cons y ys -> False\n\
         \x20   Cons x xs -> case b of\n\
         \x20     Nil -> False\n\
         \x20     Cons y ys -> if x == y then same xs ys else False\n\
         \x20 not-two :: Int -> Option Int\n\
         \x20 not-two x = if x == 2 then None else Some x\n\
         \x20 bump :: Int -> Option Int\n\
         \x20 bump x = Some (x + 10)\n\
         \x20 check-a :: Int\n\
         \x20 check-a =\n\
         \x20   let xs = Cons (1 :: Int) (Cons 2 (Cons 3 Nil)) in\n\
         \x20   case (traverse not-two xs :: Option (List Int)) of\n\
         \x20     None -> 0\n\
         \x20     Some vs -> 1\n\
         \x20 check-b :: Int\n\
         \x20 check-b = case (traverse not-two (Cons (1 :: Int) Nil) :: Option (List Int)) of\n\
         \x20   None -> 2\n\
         \x20   Some vs -> if same vs (Cons 1 Nil) then 0 else 2\n\
         \x20 check-c :: Int\n\
         \x20 check-c =\n\
         \x20   let xs = Cons (1 :: Int) (Cons 2 (Cons 3 Nil)) in\n\
         \x20   case (traverse bump xs :: Option (List Int)) of\n\
         \x20     None -> 4\n\
         \x20     Some vs -> if same vs (Cons 11 (Cons 12 (Cons 13 Nil))) then 0 else 4\n\
         \x20 check-d :: Int\n\
         \x20 check-d =\n\
         \x20   let xs = Cons (1 :: Int) (Cons 2 (Cons 3 Nil)) in\n\
         \x20   case (traverse (\\v -> Some v) xs :: Option (List Int)) of\n\
         \x20     None -> 8\n\
         \x20     Some vs -> if same vs xs then 0 else 8\n\
         \x20 main :: Int\n\
         \x20 main = check-a + check-b + check-c + check-d\n",
    );
}

#[test]
fn traverse_option_container_into_result_and_option() {
    expect_ok(
        "traverse-option-container",
        "module Main where\n\
         \x20 import Category.Foldable\n\
         \x20 import Category.Traversable\n\
         \x20 same :: List Int -> List Int -> Bool\n\
         \x20 same a b = case a of\n\
         \x20   Nil -> case b of\n\
         \x20     Nil -> True\n\
         \x20     Cons y ys -> False\n\
         \x20   Cons x xs -> case b of\n\
         \x20     Nil -> False\n\
         \x20     Cons y ys -> if x == y then same xs ys else False\n\
         \x20 half :: Int -> Result Int Int\n\
         \x20 half x = if x == 2 then Err 2 else Ok x\n\
         \x20 not-two :: Int -> Option Int\n\
         \x20 not-two x = if x == 2 then None else Some x\n\
         \x20 check-a :: Int\n\
         \x20 check-a = case (traverse half (Some (2 :: Int)) :: Result Int (Option Int)) of\n\
         \x20   Err e -> if e == 2 then 0 else 1\n\
         \x20   Ok v -> 1\n\
         \x20 check-b :: Int\n\
         \x20 check-b = case (traverse half (Some (1 :: Int)) :: Result Int (Option Int)) of\n\
         \x20   Err e -> 2\n\
         \x20   Ok v -> case v of\n\
         \x20     None -> 2\n\
         \x20     Some y -> if y == 1 then 0 else 2\n\
         \x20 check-c :: Int\n\
         \x20 check-c = case (traverse half (None :: Option Int) :: Result Int (Option Int)) of\n\
         \x20   Err e -> 4\n\
         \x20   Ok v -> case v of\n\
         \x20     None -> 0\n\
         \x20     Some y -> 4\n\
         \x20 check-d :: Int\n\
         \x20 check-d = case (traverse not-two (Some (2 :: Int)) :: Option (Option Int)) of\n\
         \x20   None -> 0\n\
         \x20   Some v -> 8\n\
         \x20 main :: Int\n\
         \x20 main = check-a + check-b + check-c + check-d\n",
    );
}

#[test]
fn traverse_nonempty_into_result_and_option() {
    // The refinement's `Traversable` transfer is the container's own
    // traversal (first-error short-circuit in traversal order), its
    // refined result observed as the list it is.
    expect_ok(
        "traverse-nonempty",
        "module Main where\n\
         \x20 import Category.Foldable\n\
         \x20 import Category.Traversable\n\
         \x20 import Data.NonEmpty\n\
         \x20 same :: List Int -> List Int -> Bool\n\
         \x20 same a b = case a of\n\
         \x20   Nil -> case b of\n\
         \x20     Nil -> True\n\
         \x20     Cons y ys -> False\n\
         \x20   Cons x xs -> case b of\n\
         \x20     Nil -> False\n\
         \x20     Cons y ys -> if x == y then same xs ys else False\n\
         \x20 half :: Int -> Result Int Int\n\
         \x20 half x = if x == 2 then Err 2 else if x == 4 then Err 4 else Ok x\n\
         \x20 not-two :: Int -> Option Int\n\
         \x20 not-two x = if x == 2 then None else Some x\n\
         \x20 check-a :: Int\n\
         \x20 check-a = case non-empty (Cons (1 :: Int) (Cons 3 (Cons 5 Nil))) of\n\
         \x20   None -> 1\n\
         \x20   Some ne -> case (traverse half ne :: Result Int (NonEmpty Int)) of\n\
         \x20     Err e -> 1\n\
         \x20     Ok v -> if same (to-list v) (to-list ne) then 0 else 1\n\
         \x20 check-b :: Int\n\
         \x20 check-b = case non-empty (Cons (1 :: Int) (Cons 2 (Cons 4 Nil))) of\n\
         \x20   None -> 2\n\
         \x20   Some tail-bad -> case (traverse half tail-bad :: Result Int (NonEmpty Int)) of\n\
         \x20     Err e -> if e == 2 then 0 else 2\n\
         \x20     Ok v -> 2\n\
         \x20 check-c :: Int\n\
         \x20 check-c = case non-empty (Cons (2 :: Int) (Cons 4 Nil)) of\n\
         \x20   None -> 4\n\
         \x20   Some head-bad -> case (traverse half head-bad :: Result Int (NonEmpty Int)) of\n\
         \x20     Err e -> if e == 2 then 0 else 4\n\
         \x20     Ok v -> 4\n\
         \x20 check-d :: Int\n\
         \x20 check-d = case non-empty (Cons (1 :: Int) (Cons 3 (Cons 5 Nil))) of\n\
         \x20   None -> 8\n\
         \x20   Some ne -> case (traverse not-two ne :: Option (NonEmpty Int)) of\n\
         \x20     None -> 8\n\
         \x20     Some w -> if same (to-list w) (to-list ne) then 0 else 8\n\
         \x20 main :: Int\n\
         \x20 main = check-a + check-b + check-c + check-d\n",
    );
}

#[test]
fn sequence_short_circuits_on_first_error() {
    expect_ok(
        "sequence-short-circuit",
        "module Main where\n\
         \x20 import Category.Foldable\n\
         \x20 import Category.Traversable\n\
         \x20 import Data.NonEmpty\n\
         \x20 same :: List Int -> List Int -> Bool\n\
         \x20 same a b = case a of\n\
         \x20   Nil -> case b of\n\
         \x20     Nil -> True\n\
         \x20     Cons y ys -> False\n\
         \x20   Cons x xs -> case b of\n\
         \x20     Nil -> False\n\
         \x20     Cons y ys -> if x == y then same xs ys else False\n\
         \x20 check-a :: Int\n\
         \x20 check-a = case (sequence (Cons (Ok (1 :: Int)) (Cons (Err (7 :: Int)) (Cons (Err (9 :: Int)) Nil))) :: Result Int (List Int)) of\n\
         \x20   Err e -> if e == 7 then 0 else 1\n\
         \x20   Ok vs -> 1\n\
         \x20 check-b :: Int\n\
         \x20 check-b = case (sequence (Cons (Ok (1 :: Int)) (Cons (Ok (2 :: Int)) Nil)) :: Result Int (List Int)) of\n\
         \x20   Err e -> 2\n\
         \x20   Ok vs -> if same vs (Cons 1 (Cons 2 Nil)) then 0 else 2\n\
         \x20 check-c :: Int\n\
         \x20 check-c = case (sequence (Cons (Some (1 :: Int)) (Cons None (Cons (Some 3) Nil))) :: Option (List Int)) of\n\
         \x20   None -> 0\n\
         \x20   Some vs -> 4\n\
         \x20 check-d :: Int\n\
         \x20 check-d = case non-empty (Cons (Ok (1 :: Int)) (Cons (Err (7 :: Int)) (Cons (Err (9 :: Int)) Nil))) of\n\
         \x20   None -> 8\n\
         \x20   Some ne -> case (sequence ne :: Result Int (NonEmpty Int)) of\n\
         \x20     Err e -> if e == 7 then 0 else 8\n\
         \x20     Ok v -> 8\n\
         \x20 check-e :: Int\n\
         \x20 check-e = case non-empty (Cons (Ok (1 :: Int)) (Cons (Ok (2 :: Int)) Nil)) of\n\
         \x20   None -> 16\n\
         \x20   Some ne -> case (sequence ne :: Result Int (NonEmpty Int)) of\n\
         \x20     Err e -> 16\n\
         \x20     Ok v -> if same (to-list v) (Cons 1 (Cons 2 Nil)) then 0 else 16\n\
         \x20 main :: Int\n\
         \x20 main = check-a + check-b + check-c + check-d + check-e\n",
    );
}
