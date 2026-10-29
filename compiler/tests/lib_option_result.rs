//! `Data.Maybe`/`Data.Either` combinators: the folds, conversions and
//! projections a caller expects beside `Option` and `Result`, checked
//! for consumer-visible behaviour.
//!
//! Every check compiles and RUNS a real program through the CLI pipeline
//! (`driver`, `Mode::Run`), like lib_category.rs and lib_foldable.rs.
//! The `Main` modules import nothing, which is part of the claim under
//! test: every name here reaches a program through the implicit prelude
//! surface alone (`Data.Option` and `Data.Result` are facade
//! re-exports, so adding a name to their `export` line is all an
//! addition needs). The one exception is the `(<|>)` comparison, whose
//! program imports `Category.Alternative` explicitly, because the class
//! modules are deliberately not part of the implicit surface.
//!
//! Each program's `main` sums one top-level `check` function per
//! assertion, each returning 0 or its bit, so a nonzero exit code is the
//! bitmask of the failed checks (check k contributes bit 2^(k-1)). The
//! checks are top-level because a `case` in a `let` binder's right-hand
//! side must indent past the binder's column under the layout rules.
//!
//! Coverage: `maybe`/`either` on both branches; `from-option`/
//! `from-result` on the present and the absent case; `is-some`/
//! `is-none`/`is-ok`/`is-err` on both cases; `cat-options` mixing `Some`
//! and `None` (and all-`None`, and the empty list); `map-option`/
//! `map-result`/`map-err` leaving the other side untouched; the
//! `option-to-list`/`list-to-option` round trip including the empty list
//! to `None`; `result-to-option`/`option-to-result`; `or-else` on all
//! three shapes and its agreement with the `Alternative` choice; and
//! `partition-results` with the `results`/`errors` projections on
//! all-`Ok`, all-`Err`, mixed and empty lists.

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

/// The structural-equality helpers and shared sample values every
/// program carries. Two `Option`s, `List`s or `Result`s are compared
/// element by element by hand: the containers' own `Eq` instances are
/// deliberately not shipped with their types (D119), so a test that
/// needs one states it, and a helper keeps the comparison in the
/// program where the exit code can see it.
const HELPERS: &str = r#"  same-list :: forall a. using (ea :: Eq a) => List a -> List a -> Bool
  same-list a b = case a of
    Nil -> case b of
      Nil -> True
      Cons y ys -> False
    Cons x xs -> case b of
      Nil -> False
      Cons y ys -> if x == y then same-list xs ys else False

  same-option :: forall a. using (ea :: Eq a) => Option a -> Option a -> Bool
  same-option a b = case a of
    None -> case b of
      None -> True
      Some y -> False
    Some x -> case b of
      None -> False
      Some y -> x == y

  same-result :: forall e a. using (ee :: Eq e, ea :: Eq a) => Result e a -> Result e a -> Bool
  same-result a b = case a of
    Err p -> case b of
      Err q -> p == q
      Ok v -> False
    Ok x -> case b of
      Err q -> False
      Ok y -> x == y

  small-list :: List Int
  small-list = Cons 1 (Cons 2 Nil)

  all-ok :: List (Result Int Int)
  all-ok = Cons (Ok 1) (Cons (Ok 2) Nil)

  all-err :: List (Result Int Int)
  all-err = Cons (Err 10) (Cons (Err 20) Nil)

  mixed :: List (Result Int Int)
  mixed = Cons (Ok 1) (Cons (Err 10) (Cons (Ok 2) Nil))
"#;

/// One program: the module header, its imports when it needs any, the
/// shared helpers, then the checks and `main`.
fn program(imports: &str, body: &str) -> String {
    format!("module Main where\n{imports}{HELPERS}\n{body}\n")
}

fn run_main(name: &str, src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_optres_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main");
    match driver(Mode::Run, &dir.join("Main.cinch"), None) {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_ok(name: &str, body: &str) {
    let code = run_main(name, &program("", body));
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

#[test]
fn option_folds_defaults_and_presence_tests() {
    expect_ok(
        "option-folds",
        "  check1 :: Int\n\
         \x20 check1 = if maybe (0 :: Int) (\\x -> x) (None :: Option Int) == 0 then 0 else 1\n\
         \n\x20 check2 :: Int\n\
         \x20 check2 = if maybe (0 :: Int) (\\x -> x) (Some (42 :: Int)) == 42 then 0 else 2\n\
         \n\x20 check3 :: Int\n\
         \x20 check3 = if maybe (7 :: Int) (\\x -> x + 1) (Some (1 :: Int)) == 2 then 0 else 4\n\
         \n\x20 check4 :: Int\n\
         \x20 check4 = if from-option (9 :: Int) (None :: Option Int) == 9 then 0 else 8\n\
         \n\x20 check5 :: Int\n\
         \x20 check5 = if from-option (9 :: Int) (Some (3 :: Int)) == 3 then 0 else 16\n\
         \n\x20 check6 :: Int\n\
         \x20 check6 = if is-some (Some (1 :: Int)) then (if is-some (None :: Option Int) then 32 else 0) else 32\n\
         \n\x20 check7 :: Int\n\
         \x20 check7 = if is-none (None :: Option Int) then (if is-none (Some (1 :: Int)) then 64 else 0) else 64\n\
         \n\x20 main :: Int\n\
         \x20 main = check1 + check2 + check3 + check4 + check5 + check6 + check7\n",
    );
}

#[test]
fn option_list_conversions_round_trip() {
    expect_ok(
        "option-list",
        "  check1 :: Int\n\
         \x20 check1 = if same-list (option-to-list (Some (5 :: Int))) (Cons 5 Nil) then 0 else 1\n\
         \n\x20 check2 :: Int\n\
         \x20 check2 = if same-list (option-to-list (None :: Option Int)) (Nil :: List Int) then 0 else 2\n\
         \n\x20 check3 :: Int\n\
         \x20 check3 = if same-option (list-to-option (Nil :: List Int)) (None :: Option Int) then 0 else 4\n\
         \n\x20 check4 :: Int\n\
         \x20 check4 = if same-option (list-to-option (Cons (5 :: Int) Nil)) (Some 5) then 0 else 8\n\
         \n\x20 check5 :: Int\n\
         \x20 check5 = if same-option (list-to-option small-list) (Some 1) then 0 else 16\n\
         \n\x20 check6 :: Int\n\
         \x20 check6 = if same-option (list-to-option (option-to-list (Some (7 :: Int)))) (Some 7) then 0 else 32\n\
         \n\x20 check7 :: Int\n\
         \x20 check7 = if same-option (list-to-option (option-to-list (None :: Option Int))) (None :: Option Int) then 0 else 64\n\
         \n\x20 check8 :: Int\n\
         \x20 check8 = if same-list (cat-options (Cons (Some 1) (Cons (None :: Option Int) (Cons (Some 3) Nil)))) (Cons 1 (Cons 3 Nil)) then 0 else 128\n\
         \n\x20 main :: Int\n\
         \x20 main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8\n",
    );
}

#[test]
fn cat_options_map_option_and_or_else() {
    expect_ok(
        "cat-map-alt",
        "  check1 :: Int\n\
         \x20 check1 = if same-list (cat-options (Cons (None :: Option Int) (Cons (None :: Option Int) Nil))) (Nil :: List Int) then 0 else 1\n\
         \n\x20 check2 :: Int\n\
         \x20 check2 = if same-list (cat-options (Nil :: List (Option Int))) (Nil :: List Int) then 0 else 2\n\
         \n\x20 check3 :: Int\n\
         \x20 check3 = if same-option (map-option (\\x -> x + 1) (Some (1 :: Int))) (Some 2) then 0 else 4\n\
         \n\x20 check4 :: Int\n\
         \x20 check4 = if same-option (map-option (\\x -> x + 1) (None :: Option Int)) (None :: Option Int) then 0 else 8\n\
         \n\x20 check5 :: Int\n\
         \x20 check5 = if same-option (or-else (Some (1 :: Int)) (Some 2)) (Some 1) then 0 else 16\n\
         \n\x20 check6 :: Int\n\
         \x20 check6 = if same-option (or-else (None :: Option Int) (Some 2)) (Some 2) then 0 else 32\n\
         \n\x20 check7 :: Int\n\
         \x20 check7 = if same-option (or-else (None :: Option Int) (None :: Option Int)) (None :: Option Int) then 0 else 64\n\
         \n\x20 main :: Int\n\
         \x20 main = check1 + check2 + check3 + check4 + check5 + check6 + check7\n",
    );
}

#[test]
fn either_from_result_and_result_presence_tests() {
    expect_ok(
        "result-folds",
        "  check1 :: Int\n\
         \x20 check1 = if either (\\e -> e + (100 :: Int)) (\\x -> x) (Err (5 :: Int)) == 105 then 0 else 1\n\
         \n\x20 check2 :: Int\n\
         \x20 check2 = if either (\\e -> e + (100 :: Int)) (\\x -> x) (Ok (9 :: Int)) == 9 then 0 else 2\n\
         \n\x20 check3 :: Int\n\
         \x20 check3 = if from-result (7 :: Int) (Err (1 :: Int)) == 7 then 0 else 4\n\
         \n\x20 check4 :: Int\n\
         \x20 check4 = if from-result (7 :: Int) (Ok (3 :: Int)) == 3 then 0 else 8\n\
         \n\x20 check5 :: Int\n\
         \x20 check5 = if is-ok ((Ok (1 :: Int)) :: Result Int Int) then (if is-ok ((Err (1 :: Int)) :: Result Int Int) then 16 else 0) else 16\n\
         \n\x20 check6 :: Int\n\
         \x20 check6 = if is-err ((Err (1 :: Int)) :: Result Int Int) then (if is-err ((Ok (1 :: Int)) :: Result Int Int) then 32 else 0) else 32\n\
         \n\x20 main :: Int\n\
         \x20 main = check1 + check2 + check3 + check4 + check5 + check6\n",
    );
}

#[test]
fn result_maps_and_conversions_touch_one_side() {
    expect_ok(
        "result-maps",
        "  check1 :: Int\n\
         \x20 check1 = if same-result ((map-result (\\x -> x + 1) (Ok (1 :: Int))) :: Result Int Int) (Ok 2) then 0 else 1\n\
         \n\x20 check2 :: Int\n\
         \x20 check2 = if same-result ((map-result (\\x -> x + 1) (Err (1 :: Int))) :: Result Int Int) (Err 1) then 0 else 2\n\
         \n\x20 check3 :: Int\n\
         \x20 check3 = if same-result ((map-err (\\e -> e + (10 :: Int)) (Err (1 :: Int))) :: Result Int Int) (Err 11) then 0 else 4\n\
         \n\x20 check4 :: Int\n\
         \x20 check4 = if same-result ((map-err (\\e -> e + (10 :: Int)) (Ok (3 :: Int))) :: Result Int Int) (Ok 3) then 0 else 8\n\
         \n\x20 check5 :: Int\n\
         \x20 check5 = if same-option (result-to-option ((Err (1 :: Int)) :: Result Int Int)) (None :: Option Int) then 0 else 16\n\
         \n\x20 check6 :: Int\n\
         \x20 check6 = if same-option (result-to-option ((Ok (2 :: Int)) :: Result Int Int)) (Some 2) then 0 else 32\n\
         \n\x20 check7 :: Int\n\
         \x20 check7 = if same-result (option-to-result (9 :: Int) (None :: Option Int)) (Err 9) then 0 else 64\n\
         \n\x20 check8 :: Int\n\
         \x20 check8 = if same-result (option-to-result (9 :: Int) (Some (3 :: Int))) (Ok 3) then 0 else 128\n\
         \n\x20 main :: Int\n\
         \x20 main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8\n",
    );
}

#[test]
fn partition_results_and_projections() {
    // A `case` over the returned pair lives in a top-level check, and
    // the two lists are compared with `same-list`, since a tuple of
    // lists has no shipped `Eq`.
    expect_ok(
        "partition",
        "  check1 :: Int\n\
         \x20 check1 = case partition-results all-ok of\n\
         \x20   (es, xs) -> if same-list es (Nil :: List Int) then (if same-list xs (Cons 1 (Cons 2 Nil)) then 0 else 1) else 1\n\
         \n\x20 check2 :: Int\n\
         \x20 check2 = case partition-results all-err of\n\
         \x20   (es, xs) -> if same-list xs (Nil :: List Int) then (if same-list es (Cons 10 (Cons 20 Nil)) then 0 else 2) else 2\n\
         \n\x20 check3 :: Int\n\
         \x20 check3 = case partition-results mixed of\n\
         \x20   (es, xs) -> if same-list es (Cons 10 Nil) then (if same-list xs (Cons 1 (Cons 2 Nil)) then 0 else 4) else 4\n\
         \n\x20 check4 :: Int\n\
         \x20 check4 = case partition-results (Nil :: List (Result Int Int)) of\n\
         \x20   (es, xs) -> if same-list es (Nil :: List Int) then (if same-list xs (Nil :: List Int) then 0 else 8) else 8\n\
         \n\x20 check5 :: Int\n\
         \x20 check5 = if same-list (results mixed) (Cons 1 (Cons 2 Nil)) then 0 else 16\n\
         \n\x20 check6 :: Int\n\
         \x20 check6 = if same-list (errors mixed) (Cons 10 Nil) then 0 else 32\n\
         \n\x20 check7 :: Int\n\
         \x20 check7 = if same-list (results all-err) (Nil :: List Int) then 0 else 64\n\
         \n\x20 check8 :: Int\n\
         \x20 check8 = if same-list (errors all-ok) (Nil :: List Int) then 0 else 128\n\
         \n\x20 main :: Int\n\
         \x20 main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8\n",
    );
}

#[test]
fn or_else_is_the_alternative_choice() {
    // `or-else` is the prelude's spelling of `Control.Applicative`'s
    // `(<|>)` for `Option`, which a caller reaches only with this
    // import; the check proves they agree on the three shapes.
    let body = "  check1 :: Int\n\
                \x20 check1 = if same-option (or-else (Some (1 :: Int)) (Some 2)) ((<|>) (Some (1 :: Int)) (Some 2)) then 0 else 1\n\
                \n\x20 check2 :: Int\n\
                \x20 check2 = if same-option (or-else (None :: Option Int) (Some 2)) ((<|>) (None :: Option Int) (Some 2)) then 0 else 2\n\
                \n\x20 check3 :: Int\n\
                \x20 check3 = if same-option (or-else (None :: Option Int) (None :: Option Int)) ((<|>) (None :: Option Int) (None :: Option Int)) then 0 else 4\n\
                \n\x20 main :: Int\n\
                \x20 main = check1 + check2 + check3\n";
    let code = run_main(
        "or-else-alt",
        &program("  import Category.Alternative\n", body),
    );
    assert_eq!(code, 0, "or-else-alt: check bitmask {code}");
}
