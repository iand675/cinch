//! Foldable/Traversable/NonEmpty consumer semantics: the fold and
//! traversal classes over List, Option and NonEmpty, the refinement's
//! free semigroup instance (the container's own `<>` over `List`), and
//! the showcase behavior of `traverse` into `Result` (short-circuiting
//! on the FIRST error) and into `Option`.
//!
//! Each test compiles a real multi-file project: the prelude modules
//! under test (Category/Foldable.cinch, Category/Traversable.cinch,
//! Data/NonEmpty.cinch) plus Algebra/Semigroup.cinch and the
//! Category.Functor/Category.Applicative/Category.Monad class modules
//! the instances and the free combinators build on, the class homes
//! those combinators constrain over (Algebra/Monoid.cinch, with the
//! Algebra/Group.cinch it imports, and Algebra/Semiring.cinch), driven
//! through the CLI pipeline like modules.rs. The
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
//!   Combinators: the free folds (`fold-map`, `sum`, `product`,
//!                `concat`) at the identity an empty container gives and
//!                over a populated one, the partial family
//!                (`fold-right1`/`fold-left1`, `maximum`/`minimum`,
//!                `maximum-by`/`minimum-by`) as `Option` values, the
//!                search family (`elem`/`not-elem`, `find`,
//!                `any`/`all` with the vacuous cases), and the
//!                traversals' left-to-right order (`for`, `map-m`,
//!                `for-m`, `sequence-a`) observed in the result.
//!
//! Each check is a top-level `check-*` function returning 0 or its bit;
//! `main` sums them, so a nonzero exit code is a bitmask naming the
//! failed checks (check k contributes bit 2^(k-1)) and any failure mode
//! of one check shares that bit, keeping the total inside the 8-bit
//! exit status. The checks are top-level because a `case` in a `let`
//! binder's right-hand side must indent past the binder's column under
//! the layout rules.

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const FOLDABLE: &str = include_str!("../prelude/Category/Foldable.cinch");
const TRAVERSABLE: &str = include_str!("../prelude/Category/Traversable.cinch");
const FUNCTOR: &str = include_str!("../prelude/Category/Functor.cinch");
const APPLICATIVE: &str = include_str!("../prelude/Category/Applicative.cinch");
const MONAD: &str = include_str!("../prelude/Category/Monad.cinch");
const NONEMPTY: &str = include_str!("../prelude/Data/NonEmpty.cinch");
const SEMIGROUP: &str = include_str!("../prelude/Algebra/Semigroup.cinch");
/// The class homes the free combinators constrain over, written beside
/// the entry like the modules under test: a project module that shadows
/// a prelude home can reference a class only when that class's home is
/// a module of the project too (or is in the facade's surface, as `Eq`
/// and `Ord` are), so `fold-map`'s `Monoid`/`Semigroup`,
/// `sum`/`product`'s `Semiring` and `map-m`'s `Monad` need their homes
/// in the project. Observed in the shared tree; the copies are
/// otherwise inert.
const MONOID: &str = include_str!("../prelude/Algebra/Monoid.cinch");
const GROUP: &str = include_str!("../prelude/Algebra/Group.cinch");
const SEMIRING: &str = include_str!("../prelude/Algebra/Semiring.cinch");

fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_foldtest_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [
        ("Foldable.cinch", FOLDABLE),
        ("Traversable.cinch", TRAVERSABLE),
        ("Functor.cinch", FUNCTOR),
        ("Applicative.cinch", APPLICATIVE),
        ("Monad.cinch", MONAD),
        ("NonEmpty.cinch", NONEMPTY),
        ("Semigroup.cinch", SEMIGROUP),
        ("Monoid.cinch", MONOID),
        ("Group.cinch", GROUP),
        ("Semiring.cinch", SEMIRING),
        ("Main.cinch", main_src),
    ] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
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

/// The shared `Main` header for the free-combinator checks below: the
/// imports and the structural equality helpers, because `List` and
/// `Option` ship no `Eq` instance (decision D119: the instances live
/// with whoever declares the container, and a program's own declaration
/// is the only unambiguous one). Each body below is therefore only its
/// `check-*` functions and `main`; the checks are top-level because a
/// `case` in a `let` binder's right-hand side must indent past the
/// binder's column under the layout rules, and each returns 0 or its bit
/// so a nonzero exit code names the failing check.
const COMBINATORS: &str = r#"module Main where
  import Category.Foldable
  import Category.Traversable
  import Category.Monad
  import Category.Applicative

  same-list :: List Int -> List Int -> Bool
  same-list a b = case a of
    Nil -> case b of
      Nil -> True
      Cons _ _ -> False
    Cons x xs -> case b of
      Nil -> False
      Cons y ys -> if x == y then same-list xs ys else False

  same-list-nat :: List Nat -> List Nat -> Bool
  same-list-nat a b = case a of
    Nil -> case b of
      Nil -> True
      Cons _ _ -> False
    Cons x xs -> case b of
      Nil -> False
      Cons y ys -> if x == y then same-list-nat xs ys else False

  same-option :: Option Int -> Option Int -> Bool
  same-option a b = case a of
    None -> case b of
      None -> True
      Some _ -> False
    Some x -> case b of
      None -> False
      Some y -> x == y

  by-desc :: Int -> Int -> Ordering
  by-desc y z = if y < z then GT else if y > z then LT else EQ
"#;

/// Run `COMBINATORS` plus a body that ends in `main`, asserting exit 0.
fn expect_combinators(name: &str, body: &str) {
    expect_ok(name, &format!("{COMBINATORS}\n{body}\n"));
}

/// The Foldable free combinators over `List`: `fold-map`, `sum` and
/// `product` (with the identity the empty container must give),
/// `concat` at two carriers, and `maximum` on a container whose maximum
/// is not its last element.
#[test]
fn foldable_free_combinators_over_list() {
    expect_combinators(
        "foldable-free-list",
        r#"  xs :: List Int
  xs = Cons 1 (Cons 2 (Cons 3 Nil))

  big :: List Int
  big = Cons 3 (Cons 1 (Cons 2 Nil))

  check1 :: Int
  check1 = if same-list (fold-map (\y -> Cons y Nil) xs) xs then 0 else 1

  check2 :: Int
  check2 = if sum xs == 6 then 0 else 2

  check3 :: Int
  check3 = if sum (Nil :: List Int) == 0 then 0 else 4

  check4 :: Int
  check4 = if product xs == 6 then 0 else 8

  check5 :: Int
  check5 = if product (Nil :: List Int) == 1 then 0 else 16

  check6 :: Int
  check6 = if concat (Cons "ab" (Cons "cd" Nil)) == "abcd" then 0 else 32

  check7 :: Int
  check7 = if same-list-nat (concat (Cons (Cons 1 (Cons 2 Nil)) (Cons (Cons 3 Nil) Nil))) (Cons 1 (Cons 2 (Cons 3 Nil))) then 0 else 64

  check8 :: Int
  check8 = if same-option (maximum big) (Some 3) then 0 else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8"#,
    );
}

/// The same combinators at a second instance (`Option`), and the
/// partial folds: `fold-right1`/`fold-left1` report an empty container
/// as `None` and a one-element one as that element (the seed is never
/// combined with anything).
#[test]
fn foldable_free_combinators_over_option_and_the_partials() {
    expect_combinators(
        "foldable-free-option-partials",
        r#"  check1 :: Int
  check1 = if same-list (fold-map (\y -> Cons y Nil) (Some (7 :: Int))) (Cons 7 Nil) then 0 else 1

  check2 :: Int
  check2 = if same-list (fold-map (\y -> Cons y Nil) (None :: Option Int)) (Nil :: List Int) then 0 else 2

  check3 :: Int
  check3 = if sum (Some (4 :: Int)) == 4 then 0 else 4

  check4 :: Int
  check4 = if sum (None :: Option Int) == 0 then 0 else 8

  check5 :: Int
  check5 = if same-option (fold-right1 (\y acc -> y + acc) (Some (5 :: Int))) (Some 5) then 0 else 16

  check6 :: Int
  check6 = if same-option (fold-right1 (\y acc -> y + acc) (None :: Option Int)) None then 0 else 32

  check7 :: Int
  check7 = if same-option (fold-left1 (\acc y -> acc + y) (Some (5 :: Int))) (Some 5) then 0 else 64

  check8 :: Int
  check8 = if same-option (fold-left1 (\acc y -> acc + y) (None :: Option Int)) None then 0 else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8"#,
    );
}

/// The fold direction (a difference is not commutative, so the two
/// folds must disagree) and the whole search family: `elem`/`not-elem`
/// hit and miss, and `find` hit and miss.
#[test]
fn foldable_fold_direction_and_search_combinators() {
    expect_combinators(
        "foldable-direction-search",
        r#"  xs :: List Int
  xs = Cons 1 (Cons 2 (Cons 3 Nil))

  check1 :: Int
  check1 = if same-option (fold-right1 (\y acc -> y - acc) xs) (Some 2) then 0 else 1

  check2 :: Int
  check2 = if same-option (fold-left1 (\acc y -> acc - y) xs) (Some (0 - 4)) then 0 else 2

  check3 :: Int
  check3 = if elem 3 xs then 0 else 4

  check4 :: Int
  check4 = if elem 9 xs then 8 else 0

  check5 :: Int
  check5 = if not-elem 9 xs then 0 else 16

  check6 :: Int
  check6 = if not-elem 3 xs then 32 else 0

  check7 :: Int
  check7 = if same-option (find (\y -> y > 1) xs) (Some 2) then 0 else 64

  check8 :: Int
  check8 = if same-option (find (\y -> y > 9) xs) None then 0 else 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8"#,
    );
}

/// `maximum`/`minimum` and the comparator forms: the empty container is
/// `None`, a maximum that is not last still wins, and the `-by` forms
/// decide by the comparator handed to them (the reversed comparator
/// selects the opposite end).
#[test]
fn foldable_ordering_combinators() {
    expect_combinators(
        "foldable-ordering",
        r#"  xs :: List Int
  xs = Cons 1 (Cons 2 (Cons 3 Nil))

  big :: List Int
  big = Cons 3 (Cons 1 (Cons 2 Nil))

  check1 :: Int
  check1 = if same-option (maximum big) (Some 3) then 0 else 1

  check2 :: Int
  check2 = if same-option (minimum big) (Some 1) then 0 else 2

  check3 :: Int
  check3 = if same-option (maximum (Nil :: List Int)) None then 0 else 4

  check4 :: Int
  check4 = if same-option (minimum (Nil :: List Int)) None then 0 else 8

  check5 :: Int
  check5 = if same-option (maximum-by by-desc xs) (Some 1) then 0 else 16

  check6 :: Int
  check6 = if same-option (minimum-by by-desc xs) (Some 3) then 0 else 32

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6"#,
    );
}

/// `any`/`all` over the empty container (the vacuous cases: `any` is
/// `False`, `all` is `True`) and over a populated one, hit and miss.
#[test]
fn foldable_boolean_combinators() {
    expect_combinators(
        "foldable-boolean",
        r#"  xs :: List Int
  xs = Cons 1 (Cons 2 (Cons 3 Nil))

  check1 :: Int
  check1 = if any (\y -> y > 9) (Nil :: List Int) then 1 else 0

  check2 :: Int
  check2 = if all (\y -> y > 9) (Nil :: List Int) then 0 else 2

  check3 :: Int
  check3 = if any (\y -> y == 2) xs then 0 else 4

  check4 :: Int
  check4 = if any (\y -> y > 9) xs then 8 else 0

  check5 :: Int
  check5 = if all (\y -> y < 4) xs then 0 else 16

  check6 :: Int
  check6 = if all (\y -> y < 3) xs then 32 else 0

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6"#,
    );
}

/// The Traversable free combinators, with the sequencing order observed
/// in the result: the first element's error wins (a left-to-right walk,
/// the same order the shipped `traverse` assertions pin), a successful
/// walk keeps the element order, and a failing walk inside `Option`
/// reports `None`.
#[test]
fn traversable_free_combinators_sequence_in_order() {
    expect_combinators(
        "traversable-free",
        r#"  xs :: List Int
  xs = Cons 1 (Cons 2 (Cons 3 Nil))

  double :: Int -> Int
  double y = y * 10

  check1 :: Int
  check1 = case (for (Cons (1 :: Int) (Cons (2 :: Int) Nil)) (\y -> Err y) :: Result Int (List Int)) of
    Err e -> if e == 1 then 0 else 1
    Ok _ -> 1

  check2 :: Int
  check2 = case (for xs (\y -> Ok (double y)) :: Result Int (List Int)) of
    Ok ys -> if same-list ys (Cons 10 (Cons 20 (Cons 30 Nil))) then 0 else 2
    Err _ -> 2

  check3 :: Int
  check3 = case (map-m (\y -> Ok (double y)) xs :: Result Int (List Int)) of
    Ok ys -> if same-list ys (Cons 10 (Cons 20 (Cons 30 Nil))) then 0 else 4
    Err _ -> 4

  check4 :: Int
  check4 = case (map-m (\y -> Some (double y)) xs :: Option (List Int)) of
    Some ys -> if same-list ys (Cons 10 (Cons 20 (Cons 30 Nil))) then 0 else 8
    None -> 8

  check5 :: Int
  check5 = case (map-m (\y -> if y == 2 then None else Some y) xs :: Option (List Int)) of
    None -> 0
    Some _ -> 16

  check6 :: Int
  check6 = case (for-m xs (\y -> Ok (double y)) :: Result Int (List Int)) of
    Ok ys -> if same-list ys (Cons 10 (Cons 20 (Cons 30 Nil))) then 0 else 32
    Err _ -> 32

  check7 :: Int
  check7 = case (sequence-a (Cons (Ok 1) (Cons (Ok 2) Nil)) :: Result Int (List Int)) of
    Ok ys -> if same-list ys (Cons 1 (Cons 2 Nil)) then 0 else 64
    Err _ -> 64

  check8 :: Int
  check8 = case (sequence-a (Cons (Ok 1) (Cons (Err 7) (Cons (Err 9) Nil))) :: Result Int (List Int)) of
    Err e -> if e == 7 then 0 else 128
    Ok _ -> 128

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8"#,
    );
}
