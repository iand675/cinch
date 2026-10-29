//! Data.Matrix consumer semantics: the dense `F64` matrix family's
//! laws and named refusals, as the Ruby `Matrix` behaviours the parity
//! note pins (design/stdlib-parity-ruby.md).
//!
//! Every test compiles the real module source
//! (compiler/prelude/Data/Matrix.cinch) beside a `Main` through the
//! CLI pipeline (`driver`, Mode::Run): the module plus the embedded
//! prelude family (`Data.List`, `Data.Machine`, ...) and the program's
//! `Main`. The programs exercise only consumer-visible semantics:
//!   identity:    `identity n * m == m` and `m * identity n == m`;
//!   transpose:   involution `transpose . transpose == id` and the
//!                anti-distributivity identity
//!                `(a * b).transpose == b.transpose * a.transpose`;
//!   determinant: a hand-computed 3x3, `det (identity n) == 1`, and a
//!                singular matrix's determinant 0 within tolerance;
//!   refusals:    ragged input, dimension mismatch (`add`/`sub`, `mul`),
//!                an out-of-range `at`, a non-square `determinant`, and
//!                a `mul-vector` width mismatch, each observed as its
//!                named error.
//!
//! Each check contributes bit 2^(k-1) and `main` sums the bits, so a
//! nonzero exit code is a bitmask naming the failed checks (the repo's
//! convention). Checks are top-level functions because a `case` in a
//! `let` binder's right-hand side must indent past the binder's column
//! under the layout rules.

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const MATRIX: &str = include_str!("../prelude/Data/Matrix.cinch");

/// Helpers shared by the programs: the `F64` readback (`ff`, from a
/// decimal string, since float literals have no AST node), the matrix
/// unwrapper for well-formed inputs, the `Result`/`MatrixError`
/// observers, and list/row builders.
const HELPERS: &str = r#"
  ff :: String -> F64
  ff s = f64-from-decimal (string-to-bytes s)

  un :: List (List F64) -> Matrix
  un xss = case matrix xss of
    Ok m -> m
    Err e -> identity 0

  row2 :: F64 -> F64 -> List F64
  row2 a b = Cons a (Cons b Nil)

  row3 :: F64 -> F64 -> F64 -> List F64
  row3 a b c = Cons a (Cons b (Cons c Nil))

  is-ok :: forall e a. Result e a -> Bool
  is-ok r = case r of
    Ok _ -> True
    Err _ -> False

  err-is :: forall a. (MatrixError -> Bool) -> Result MatrixError a -> Bool
  err-is p r = case r of
    Err e -> p e
    Ok _ -> False

  result-eq :: Result MatrixError Matrix -> Matrix -> Bool
  result-eq r m = case r of
    Ok x -> equals x m
    Err e -> False

  result-eq2 :: Result MatrixError Matrix -> Result MatrixError Matrix -> Bool
  result-eq2 r s = case r of
    Ok x -> case s of
      Ok y -> equals x y
      Err e -> False
    Err e -> False

  transpose-of :: Result MatrixError Matrix -> Result MatrixError Matrix
  transpose-of r = case r of
    Ok x -> Ok (transpose x)
    Err e -> Err e

  det-eq :: Result MatrixError F64 -> F64 -> Bool
  det-eq r expected = case r of
    Ok d -> fle-f64 (abs-f64 (sub-f64 d expected)) (ff "0.000001")
    Err e -> False

  at-eq :: Result MatrixError F64 -> F64 -> Bool
  at-eq r expected = case r of
    Ok x -> feq-f64 x expected
    Err e -> False

  eq-list-f64 :: List F64 -> List F64 -> Bool
  eq-list-f64 xs ys = case xs of
    Nil -> case ys of
      Nil -> True
      Cons y yr -> False
    Cons x xr -> case ys of
      Nil -> False
      Cons y yr -> feq-f64 x y && eq-list-f64 xr yr

  is-ragged :: MatrixError -> Bool
  is-ragged e = case e of
    RaggedRows i w f -> True
    _ -> False

  is-dim :: MatrixError -> Bool
  is-dim e = case e of
    DimensionMismatch r1 c1 r2 c2 -> True
    _ -> False

  is-vector :: MatrixError -> Bool
  is-vector e = case e of
    VectorMismatch c n -> True
    _ -> False

  is-index :: MatrixError -> Bool
  is-index e = case e of
    IndexOutOfRange i j r c -> True
    _ -> False

  is-notsquare :: MatrixError -> Bool
  is-notsquare e = case e of
    NotSquare r c -> True
    _ -> False
"#;

fn run_main(name: &str, body: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_matrix_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    let main_src = format!("module Main where\n\n  import Data.Matrix\n\n{HELPERS}\n{body}\n");
    std::fs::write(dir.join("Matrix.cinch"), MATRIX).expect("write Matrix.cinch");
    std::fs::write(dir.join("Main.cinch"), main_src).expect("write Main.cinch");
    match driver(Mode::Run, &dir.join("Main.cinch"), None) {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_ok(name: &str, body: &str) {
    let code = run_main(name, body);
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}


#[test]
fn identity_laws() {
    expect_ok(
        "identity",
        r#"
  main :: Int
  main =
    let m = un (Cons (row3 (ff "1") (ff "2") (ff "3")) (Cons (row3 (ff "4") (ff "5") (ff "6")) Nil)) in
    let left = mul (identity 2) m in
    let right = mul m (identity 3) in
    let c1 = if is-ok left then 0 else 1 in
    let c2 = if is-ok right then 0 else 2 in
    let c3 = if result-eq left m then 0 else 4 in
    let c4 = if result-eq right m then 0 else 8 in
    c1 + c2 + c3 + c4
"#,
    );
}

#[test]
fn transpose_involution_and_anti_distributivity() {
    expect_ok(
        "transpose",
        r#"
  main :: Int
  main =
    let a = un (Cons (row3 (ff "1") (ff "2") (ff "3")) (Cons (row3 (ff "0") (ff "1") (ff "4")) Nil)) in
    let b = un (Cons (row2 (ff "1") (ff "0")) (Cons (row2 (ff "2") (ff "1")) (Cons (row2 (ff "3") (ff "1")) Nil))) in
    let c1 = if equals (transpose (transpose a)) a then 0 else 1 in
    let ab = mul a b in
    let rhs = mul (transpose b) (transpose a) in
    let c2 = if result-eq2 (transpose-of ab) rhs then 0 else 2 in
    c1 + c2
"#,
    );
}

#[test]
fn determinant_hand_computed_identity_and_singular() {
    expect_ok(
        "determinant",
        r#"
  main :: Int
  main =
    let a = un (Cons (row3 (ff "1") (ff "2") (ff "3")) (Cons (row3 (ff "4") (ff "5") (ff "6")) (Cons (row3 (ff "7") (ff "8") (ff "10")) Nil))) in
    let c1 = if det-eq (determinant a) (ff "-3") then 0 else 1 in
    let c2 = if det-eq (determinant (identity 3)) (ff "1") then 0 else 2 in
    let sing = un (Cons (row2 (ff "1") (ff "2")) (Cons (row2 (ff "2") (ff "4")) Nil)) in
    let c3 = if det-eq (determinant sing) (ff "0") then 0 else 4 in
    c1 + c2 + c3
"#,
    );
}

#[test]
fn ragged_input_is_rejected_at_construction() {
    expect_ok(
        "ragged",
        r#"
  main :: Int
  main =
    let short = matrix (Cons (row2 (ff "1") (ff "2")) (Cons (row3 (ff "3") (ff "4") (ff "5")) Nil)) in
    let long = matrix (Cons (row3 (ff "1") (ff "2") (ff "3")) (Cons (row2 (ff "4") (ff "5")) Nil)) in
    let c1 = if err-is is-ragged short then 0 else 1 in
    let c2 = if err-is is-ragged long then 0 else 2 in
    c1 + c2
"#,
    );
}

#[test]
fn dimension_mismatches_are_named_errors() {
    expect_ok(
        "dims",
        r#"
  main :: Int
  main =
    let a2 = un (Cons (row2 (ff "1") (ff "2")) (Cons (row2 (ff "3") (ff "4")) Nil)) in
    let a3 = un (Cons (row3 (ff "1") (ff "0") (ff "0")) (Cons (row3 (ff "0") (ff "1") (ff "0")) (Cons (row3 (ff "0") (ff "0") (ff "1")) Nil))) in
    let b = un (Cons (row3 (ff "1") (ff "2") (ff "3")) (Cons (row3 (ff "4") (ff "5") (ff "6")) Nil)) in
    let c = un (Cons (row3 (ff "1") (ff "2") (ff "3")) (Cons (row3 (ff "4") (ff "5") (ff "6")) Nil)) in
    let c1 = if err-is is-dim (add a2 a3) then 0 else 1 in
    let c2 = if err-is is-dim (sub a2 a3) then 0 else 2 in
    let c3 = if err-is is-dim (mul b c) then 0 else 4 in
    let c4 = if err-is is-index (at a2 5 5) then 0 else 8 in
    let c5 = if err-is is-notsquare (determinant b) then 0 else 16 in
    let c6 = if err-is is-vector (mul-vector a2 (row3 (ff "1") (ff "2") (ff "3"))) then 0 else 32 in
    let c7 = if at-eq (at a2 1 1) (ff "4") then 0 else 64 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7
"#,
    );
}

#[test]
fn surface_operations_scale_within_and_tostring() {
    expect_ok(
        "surface",
        r#"
  vec-eq :: Result MatrixError (List F64) -> List F64 -> Bool
  vec-eq r expected = case r of
    Ok xs -> eq-list-f64 xs expected
    Err e -> False

  main :: Int
  main =
    let m = un (Cons (row2 (ff "1") (ff "2")) (Cons (row2 (ff "3") (ff "4")) Nil)) in
    let doubled = un (Cons (row2 (ff "2") (ff "4")) (Cons (row2 (ff "6") (ff "8")) Nil)) in
    let c1 = if equals-within (ff "0.000001") (scale (ff "2") m) doubled then 0 else 1 in
    let c2 = if equals-within (ff "0.000001") m m then 0 else 2 in
    let c3 = if vec-eq (mul-vector m (row2 (ff "1") (ff "1"))) (row2 (ff "3") (ff "7")) then 0 else 4 in
    let c4 = if string-compare (to-string m) "1e0 2e0\n3e0 4e0" == 0 then 0 else 8 in
    c1 + c2 + c3 + c4
"#,
    );
}
