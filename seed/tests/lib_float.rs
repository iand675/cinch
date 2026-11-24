//! Binary64 mathematics against the reference SDK's portable implementations (`tooling/float_vectors.go`):
//! every function's result on special values, threshold neighbourhoods and seeded random inputs over every
//! binade is the identical bit pattern (NaN payloads aside). The branch-free kernels also lower for other
//! targets.
mod common;

use serde_json::Value;

/// One-argument functions, two-argument functions and the split forms, by fixture key and Cinch checker.
const ROWS: &[(&str, &str)] = &[
    ("sqrt", "unary sqrt-f64"), ("floor", "unary floor-f64"), ("ceil", "unary ceil-f64"), ("trunc", "unary trunc-f64"),
    ("round", "unary round-f64"), ("roundeven", "unary round-to-even-f64"), ("cbrt", "unary cbrt-f64"),
    ("exp", "unary exp-f64"), ("exp2", "unary exp2-f64"), ("expm1", "unary expm1-f64"), ("log", "unary log-f64"),
    ("log2", "unary log2-f64"), ("log10", "unary log10-f64"), ("log1p", "unary log1p-f64"), ("logb", "unary logb-f64"),
    ("sin", "unary sin-f64"), ("cos", "unary cos-f64"), ("tan", "unary tan-f64"), ("asin", "unary asin-f64"),
    ("acos", "unary acos-f64"), ("atan", "unary atan-f64"), ("sinh", "unary sinh-f64"), ("cosh", "unary cosh-f64"),
    ("tanh", "unary tanh-f64"), ("asinh", "unary asinh-f64"), ("acosh", "unary acosh-f64"), ("atanh", "unary atanh-f64"),
    ("pow", "binary pow-f64"), ("atan2", "binary atan2-f64"), ("hypot", "binary hypot-f64"), ("mod", "binary mod-f64"),
    ("remainder", "binary remainder-f64"), ("nextafter", "binary nextafter-f64"), ("dim", "binary dim-f64"),
    ("max", "binary max-of-f64"), ("min", "binary min-of-f64"), ("copysign", "binary copysign-f64"),
    ("frexp", "frexp-row"), ("modf", "modf-row"), ("sincos", "sincos-row"), ("ldexp", "ldexp-row"),
    ("ilogb", "ilogb-row"), ("pow10", "pow10-row"),
];

/// Every function matches the reference bit for bit.
#[test]
fn float_functions_match_reference() {
    let f: Value = serde_json::from_str(include_str!("fixtures/float-vectors.json")).unwrap();
    let rows = ROWS.iter().map(|(key, check)| format!("{check} {:?}", f[*key].as_str().unwrap())).collect::<Vec<_>>();
    let dir = common::project("float", &[
        ("Data.Float.cinch", include_str!("../prelude/Data/Float.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every row.\n  main :: Int\n  main = first [{}] 0\n", rows.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing row; 200 means all passed");
    common::cleanup(&dir);
}

/// The kernels lower for 64-bit ARM, WebAssembly and a big-endian target.
#[test]
fn float_kernels_compile_for_arm_wasm_and_big_endian_targets() {
    let directory = tempfile::tempdir().unwrap();
    for (triple, arch, endian) in [
        ("aarch64-unknown-linux-gnu", "aarch64", "little"),
        ("wasm32-unknown-unknown", "wasm32", "little"),
        ("powerpc64-unknown-linux-gnu", "powerpc64", "big"),
    ] {
        let profile = cinch::driver::target_profile(&cinch::driver::DriverOptions { target: Some(triple.into()), ..Default::default() })
            .unwrap_or_else(|_| {
                let mut profile = cinch::core::TargetProfile::native();
                profile.triple = triple.into();
                profile.arch = arch.into();
                profile.endian = endian.into();
                profile
            });
        let project = directory.path().join(arch);
        std::fs::create_dir_all(&project).unwrap();
        std::fs::write(project.join("Data.Float.cinch"), include_str!("../prelude/Data/Float.cinch")).unwrap();
        std::fs::write(project.join("Main.cinch"), "module Main where\n  import Data.Float\n  -- | Reach the kernels.\n  main :: Int\n  main = if flt-f64 (add-f64 (pow-f64 (sin-f64 (exp-f64 2)) (log-f64 3)) (add-f64 (mod-f64 7 (atan2-f64 1 2)) (tanh-f64 (cbrt-f64 5)))) 0 then 1 else 0\n").unwrap();
        let ir_path = project.join("program.ll");
        let entry = project.join("Main.cinch");
        let ir = std::thread::Builder::new()
            .stack_size(256 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::driver_with_profile_opts(cinch::driver::Mode::EmitIr, cinch::check::StageMode::Refresh, &entry, Some(&ir_path), &profile, &cinch::driver::DriverOptions::default())
                    .map(|_| std::fs::read_to_string(&ir_path).unwrap())
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("@llvm.copysign.f64"));
        assert!(!ir.contains("@exp(") && !ir.contains("@log("), "no host libm calls");
        let output = std::process::Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
            .args(["-target", triple, "-O2", "-c", "-x", "ir"])
            .arg(project.join("program.ll"))
            .arg("-o")
            .arg(project.join("program.o"))
            .output()
            .unwrap();
        assert!(output.status.success(), "{triple}: {}", String::from_utf8_lossy(&output.stderr));
    }
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn float_declarations_have_canonical_documentation() {
    common::assert_documented("Data.Float", include_str!("../prelude/Data/Float.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference binary64 consumers.
module Main where
  import Data.Float
  import Data.Bytes.Word
  import Text.Hex
  -- | Fixture words.
  words :: String -> Bytes
  words text = case hex-decode text of Ok v -> v; Err e -> bytes-new 0
  -- | The word at an index (zero outside).
  w :: Bytes -> Nat -> U64
  w b i = if 8 * i + 8 <= bytes-len b then bytes-read-le64 b (8 * i) else 0
  -- | A value at an index.
  v :: Bytes -> Nat -> F64
  v b i = f64-from-bits (w b i)
  -- | Whether a result has the expected bits (any NaN for a NaN).
  same :: F64 -> U64 -> Bool
  same got want = eq-u64 (f64-bits got) want || (is-nan-f64 got && is-nan-f64 (f64-from-bits want))
  -- | Every (x, f x) pair.
  unary :: (F64 -> F64) -> String -> Bool
  unary f text = let b = words text in unary-from f b 0 (bytes-len b / 16)
  -- | Pairs from an index.
  unary-from :: (F64 -> F64) -> Bytes -> Nat -> Nat -> Bool
  unary-from f b i n = if i >= n then True else same (f (v b (2 * i))) (w b (2 * i + 1)) && unary-from f b (i + 1) n
  -- | Every (x, y, f x y) triple.
  binary :: (F64 -> F64 -> F64) -> String -> Bool
  binary f text = let b = words text in binary-from f b 0 (bytes-len b / 24)
  -- | Triples from an index.
  binary-from :: (F64 -> F64 -> F64) -> Bytes -> Nat -> Nat -> Bool
  binary-from f b i n = if i >= n then True else same (f (v b (3 * i)) (v b (3 * i + 1))) (w b (3 * i + 2)) && binary-from f b (i + 1) n
  -- | Every (x, fraction, exponent) triple.
  frexp-row :: String -> Bool
  frexp-row text = let b = words text in frexp-from b 0 (bytes-len b / 24)
  -- | Triples from an index.
  frexp-from :: Bytes -> Nat -> Nat -> Bool
  frexp-from b i n = if i >= n then True else (case frexp-f64 (v b (3 * i)) of (fr, e) -> same fr (w b (3 * i + 1)) && eq-u64 (wrapping-i64-to-u64 e) (w b (3 * i + 2))) && frexp-from b (i + 1) n
  -- | Every (x, integer, fraction) triple.
  modf-row :: String -> Bool
  modf-row text = let b = words text in modf-from b 0 (bytes-len b / 24)
  -- | Triples from an index.
  modf-from :: Bytes -> Nat -> Nat -> Bool
  modf-from b i n = if i >= n then True else (case modf-f64 (v b (3 * i)) of (a, c) -> same a (w b (3 * i + 1)) && same c (w b (3 * i + 2))) && modf-from b (i + 1) n
  -- | Every (x, sin, cos) triple.
  sincos-row :: String -> Bool
  sincos-row text = let b = words text in sincos-from b 0 (bytes-len b / 24)
  -- | Triples from an index.
  sincos-from :: Bytes -> Nat -> Nat -> Bool
  sincos-from b i n = if i >= n then True else (case sincos-f64 (v b (3 * i)) of (s, c) -> same s (w b (3 * i + 1)) && same c (w b (3 * i + 2))) && sincos-from b (i + 1) n
  -- | Every (fraction, exponent, result) triple.
  ldexp-row :: String -> Bool
  ldexp-row text = let b = words text in ldexp-from b 0 (bytes-len b / 24)
  -- | Triples from an index.
  ldexp-from :: Bytes -> Nat -> Nat -> Bool
  ldexp-from b i n = if i >= n then True else same (ldexp-f64 (v b (3 * i)) (wrapping-u64-to-i64 (w b (3 * i + 1)))) (w b (3 * i + 2)) && ldexp-from b (i + 1) n
  -- | Every (x, exponent) pair.
  ilogb-row :: String -> Bool
  ilogb-row text = let b = words text in ilogb-from b 0 (bytes-len b / 16)
  -- | Pairs from an index.
  ilogb-from :: Bytes -> Nat -> Nat -> Bool
  ilogb-from b i n = if i >= n then True else eq-u64 (wrapping-i64-to-u64 (ilogb-f64 (v b (2 * i)))) (w b (2 * i + 1)) && ilogb-from b (i + 1) n
  -- | Every (n, 10^n) pair.
  pow10-row :: String -> Bool
  pow10-row text = let b = words text in pow10-from b 0 (bytes-len b / 16)
  -- | Pairs from an index.
  pow10-from :: Bytes -> Nat -> Nat -> Bool
  pow10-from b i n = if i >= n then True else same (pow10-f64 (wrapping-u64-to-i64 (w b (2 * i)))) (w b (2 * i + 1)) && pow10-from b (i + 1) n
  -- | The index of the first failing row, or 200.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
