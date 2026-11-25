//! Complex binary64 mathematics (`Data.Complex`) against Go's `math/cmplx` and complex division
//! (`tooling/cmplx_vectors.go`): every function's result on special values (signed zeros, unit
//! values, infinities, NaN), branch cuts, poles, Payne-Hanek range reduction above 2^30,
//! rescaling thresholds, and seeded multi-scale random inputs matches bit for bit (NaN payloads
//! aside).
mod common;

use serde_json::Value;

/// Every fixture key paired with its Cinch checker expression.
const ROWS: &[(&str, &str)] = &[
    ("conj", "unary-c conj"),
    ("exp", "unary-c exp"),
    ("log", "unary-c log"),
    ("log10", "unary-c log10"),
    ("sqrt", "unary-c sqrt"),
    ("sin", "unary-c sin"),
    ("cos", "unary-c cos"),
    ("tan", "unary-c tan"),
    ("cot", "unary-c cot"),
    ("sinh", "unary-c sinh"),
    ("cosh", "unary-c cosh"),
    ("tanh", "unary-c tanh"),
    ("asin", "unary-c asin"),
    ("acos", "unary-c acos"),
    ("atan", "unary-c atan"),
    ("asinh", "unary-c asinh"),
    ("acosh", "unary-c acosh"),
    ("atanh", "unary-c atanh"),
    ("abs", "unary-f abs"),
    ("phase", "unary-f phase"),
    ("polar", "polar-row"),
    ("rect", "rect-row"),
    ("is_nan", "pred-row is-nan"),
    ("is_inf", "pred-row is-inf"),
    ("complex_inf", "const-row complex-inf"),
    ("complex_nan", "const-row complex-nan"),
    ("pow", "binary-c pow"),
    ("div", "binary-c div"),
];

/// Every `math/cmplx` function and overflow-safe `div` matches the Go reference vectors bit for bit.
#[test]
fn cmplx_matches_reference() {
    let f: Value = serde_json::from_str(include_str!("fixtures/cmplx-vectors.json")).unwrap();
    let mut rows: Vec<String> = ROWS
        .iter()
        .map(|(key, check)| format!("{check} {:?}", f[*key].as_str().unwrap()))
        .collect();
    // Extra safety checks: total behavior on Pow(0, complex(NaN, ±Inf)) and aliases.
    rows.push("extra-checks ()".to_string());
    let dir = common::project(
        "cmplx",
        &[
            ("Data.Float.cinch", include_str!("../prelude/Data/Float.cinch")),
            ("Data.Complex.cinch", include_str!("../prelude/Data/Complex.cinch")),
            ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
            ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
            (
                "Main.cinch",
                &format!(
                    "{HELPERS}\n  -- | Every row.\n  main :: Int\n  main = first [{}] 0\n",
                    rows.join(",\n    ")
                ),
            ),
        ],
    );
    assert_eq!(
        common::run(&dir, "Main.cinch"),
        200,
        "the exit status is the first failing row; 200 means all passed"
    );
    common::cleanup(&dir);
}

/// Every public and private declaration in `Data.Complex` has canonical module-owned documentation.
#[test]
fn complex_declarations_have_canonical_documentation() {
    common::assert_documented("Data.Complex", include_str!("../prelude/Data/Complex.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference complex binary64 consumers.
module Main where
  import Data.Float
  import Data.Complex
  import Data.Bytes.Word
  import Text.Hex

  -- | Decode hexadecimal fixture words into bytes.
  words :: String -> Bytes
  words text = case hex-decode text of Ok v -> v; Err e -> bytes-new 0

  -- | Read little-endian 64-bit word at index (zero out of bounds).
  w :: Bytes -> Nat -> U64
  w b i = if 8 * i + 8 <= bytes-len b then bytes-read-le64 b (8 * i) else 0

  -- | Read a binary64 value at word index.
  v :: Bytes -> Nat -> F64
  v b i = f64-from-bits (w b i)

  -- | Read a complex number starting at word index `i`.
  c :: Bytes -> Nat -> Complex
  c b i = complex (v b i) (v b (i + 1))

  -- | Whether a binary64 result has the exact expected bit pattern (any NaN for a NaN).
  same :: F64 -> U64 -> Bool
  same got want = eq-u64 (f64-bits got) want || (is-nan-f64 got && is-nan-f64 (f64-from-bits want))

  -- | Whether a complex result matches expected (re, im) bit patterns.
  same-c :: Complex -> U64 -> U64 -> Bool
  same-c got wr wi = same (real-part got) wr && same (imag-part got) wi

  -- | Every (z, f z) complex unary row.
  unary-c :: (Complex -> Complex) -> String -> Bool
  unary-c fn text = let b = words text in unary-c-from fn b 0 (bytes-len b / 32)

  -- | Complex unary rows from an index.
  unary-c-from :: (Complex -> Complex) -> Bytes -> Nat -> Nat -> Bool
  unary-c-from fn b i n =
    if i >= n then True
    else if same-c (fn (c b (4 * i))) (w b (4 * i + 2)) (w b (4 * i + 3)) then
      unary-c-from fn b (i + 1) n
    else False

  -- | Every (z, f z) complex-to-real row.
  unary-f :: (Complex -> F64) -> String -> Bool
  unary-f fn text = let b = words text in unary-f-from fn b 0 (bytes-len b / 24)

  -- | Complex-to-real rows from an index.
  unary-f-from :: (Complex -> F64) -> Bytes -> Nat -> Nat -> Bool
  unary-f-from fn b i n =
    if i >= n then True
    else if same (fn (c b (3 * i))) (w b (3 * i + 2)) then
      unary-f-from fn b (i + 1) n
    else False

  -- | Every (z, r, theta) polar row.
  polar-row :: String -> Bool
  polar-row text = let b = words text in polar-from b 0 (bytes-len b / 32)

  -- | Polar rows from an index.
  polar-from :: Bytes -> Nat -> Nat -> Bool
  polar-from b i n =
    if i >= n then True
    else case polar (c b (4 * i)) of
      (r, th) ->
        if same r (w b (4 * i + 2)) && same th (w b (4 * i + 3)) then
          polar-from b (i + 1) n
        else False

  -- | Every (r, theta, rect r theta) row.
  rect-row :: String -> Bool
  rect-row text = let b = words text in rect-from b 0 (bytes-len b / 32)

  -- | Rect rows from an index.
  rect-from :: Bytes -> Nat -> Nat -> Bool
  rect-from b i n =
    if i >= n then True
    else if same-c (rect (v b (4 * i)) (v b (4 * i + 1))) (w b (4 * i + 2)) (w b (4 * i + 3)) then
      rect-from b (i + 1) n
    else False

  -- | Every (z, pred z) predicate row.
  pred-row :: (Complex -> Bool) -> String -> Bool
  pred-row fn text = let b = words text in pred-from fn b 0 (bytes-len b / 24)

  -- | Predicate rows from an index.
  pred-from :: (Complex -> Bool) -> Bytes -> Nat -> Nat -> Bool
  pred-from fn b i n =
    if i >= n then True
    else
      let want = ne-u64 (w b (3 * i + 2)) 0 in
      if fn (c b (3 * i)) == want then pred-from fn b (i + 1) n
      else False

  -- | Constant complex value check.
  const-row :: Complex -> String -> Bool
  const-row val text = let b = words text in same-c val (w b 0) (w b 1)

  -- | Every (x, y, f x y) binary complex row.
  binary-c :: (Complex -> Complex -> Complex) -> String -> Bool
  binary-c fn text = let b = words text in binary-c-from fn b 0 (bytes-len b / 48)

  -- | Binary complex rows from an index.
  binary-c-from :: (Complex -> Complex -> Complex) -> Bytes -> Nat -> Nat -> Bool
  binary-c-from fn b i n =
    if i >= n then True
    else if same-c (fn (c b (6 * i)) (c b (6 * i + 2))) (w b (6 * i + 4)) (w b (6 * i + 5)) then
      binary-c-from fn b (i + 1) n
    else False

  -- | Additional checks: total pow(0, (NaN, +-Inf)), arg/phase and conjugate/conj parity.
  extra-checks :: Unit -> Bool
  extra-checks u =
    let p1 = pow (complex 0 0) (complex nan-f64 (inf-f64 1)) in
    let p2 = pow (complex 0 0) (complex nan-f64 (inf-f64 -1)) in
    let z = complex -3 4 in
    is-nan p1
      && is-nan p2
      && eq-u64 (f64-bits (arg z)) (f64-bits (phase z))
      && same-c (conjugate z) (f64-bits (real-part (conj z))) (f64-bits (imag-part (conj z)))

  -- | The index of the first failing row, or 200.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
