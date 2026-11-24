//! Differential test suite verifying Text.Strconv against independent Go 1.24
//! strconv reference vectors (`tooling/strconv_vectors.go` -> `fixtures/strconv-vectors.json`).
mod common;

use serde_json::Value;

fn hex_of(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 2);
    for b in s.as_bytes() {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

fn cinch_i64(v: i64) -> String {
    if v == i64::MIN {
        "min-bound-i64".into()
    } else if v < 0 {
        format!("(wrapping-neg-i64 {})", -v)
    } else {
        format!("{v}")
    }
}

fn cinch_err_opt(err: &str) -> &'static str {
    match err {
        "" => "None",
        "syntax" => "(Some StrconvSyntax)",
        "range" => "(Some StrconvRange)",
        "bad_base" => "(Some StrconvBadBase)",
        "bad_bit_size" => "(Some StrconvBadBitSize)",
        "bad_format" => "(Some StrconvBadFormat)",
        other => panic!("unexpected error class: {other}"),
    }
}

fn run_strconv_rows(tag: &str, rows: &[String]) {
    let main_src = format!(
        "{HELPERS}\n  -- | Evaluate every vector row and return 200 on full pass or the failing row index.\n  main :: Int\n  main = first [\n    {}\n  ] 0\n",
        rows.join(",\n    ")
    );
    let dir = common::project(
        tag,
        &[
            ("Data.Float.cinch", include_str!("../prelude/Data/Float.cinch")),
            (
                "Data.Unicode.Tables.cinch",
                include_str!("../prelude/Data/Unicode/Tables.cinch"),
            ),
            (
                "Data.Unicode.Kernel.cinch",
                include_str!("../prelude/Data/Unicode/Kernel.cinch"),
            ),
            (
                "Text.Strconv.cinch",
                include_str!("../prelude/Text/Strconv.cinch"),
            ),
            ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
            ("Main.cinch", &main_src),
        ],
    );
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(
        code, 200,
        "{tag}: failed at row {code}: {}",
        rows.get(code as usize).map(String::as_str).unwrap_or("<out of range>")
    );
}

fn fixtures() -> Value {
    serde_json::from_str(include_str!("fixtures/strconv-vectors.json")).unwrap()
}

/// Every public and private declaration in Text.Strconv has canonical documentation.
#[test]
fn strconv_declarations_have_canonical_documentation() {
    common::assert_documented("Text.Strconv", include_str!("../prelude/Text/Strconv.cinch"));
}

/// Booleans and integers: parse-bool, format-bool, parse-int, parse-uint, atoi, format-int, format-uint, itoa.
#[test]
fn strconv_bool_and_int_vectors() {
    let f = fixtures();
    let mut rows = Vec::new();

    for v in f["parse_bool"].as_array().unwrap() {
        let inp = hex_of(v["input"].as_str().unwrap());
        let val = if v["value"].as_bool().unwrap() { "True" } else { "False" };
        let err = cinch_err_opt(v["err"].as_str().unwrap());
        rows.push(format!("chk-pbool \"{inp}\" {val} {err}"));
    }
    for v in f["format_bool"].as_array().unwrap() {
        let inp = if v["input"].as_bool().unwrap() { "True" } else { "False" };
        let out = hex_of(v["out"].as_str().unwrap());
        rows.push(format!("format-bool {inp} == hs \"{out}\""));
    }
    for v in f["parse_int"].as_array().unwrap() {
        let inp = hex_of(v["input"].as_str().unwrap());
        let base = cinch_i64(v["base"].as_i64().unwrap());
        let bits = cinch_i64(v["bit_size"].as_i64().unwrap());
        let val = cinch_i64(v["value"].as_i64().unwrap());
        let err = cinch_err_opt(v["err"].as_str().unwrap());
        rows.push(format!("chk-pint \"{inp}\" {base} {bits} {val} {err}"));
    }
    for v in f["parse_uint"].as_array().unwrap() {
        let inp = hex_of(v["input"].as_str().unwrap());
        let base = cinch_i64(v["base"].as_i64().unwrap());
        let bits = cinch_i64(v["bit_size"].as_i64().unwrap());
        let val = v["value"].as_u64().unwrap();
        let err = cinch_err_opt(v["err"].as_str().unwrap());
        rows.push(format!("chk-puint \"{inp}\" {base} {bits} {val} {err}"));
    }
    for v in f["atoi"].as_array().unwrap() {
        let inp = hex_of(v["input"].as_str().unwrap());
        let val = cinch_i64(v["value"].as_i64().unwrap());
        let err = cinch_err_opt(v["err"].as_str().unwrap());
        rows.push(format!("chk-atoi \"{inp}\" {val} {err}"));
    }
    for v in f["format_int"].as_array().unwrap() {
        let val = cinch_i64(v["value"].as_i64().unwrap());
        let base = cinch_i64(v["base"].as_i64().unwrap());
        let out = hex_of(v["out"].as_str().unwrap());
        rows.push(format!("chk-fint {val} {base} \"{out}\""));
    }
    for v in f["format_uint"].as_array().unwrap() {
        let val = v["value"].as_u64().unwrap();
        let base = cinch_i64(v["base"].as_i64().unwrap());
        let out = hex_of(v["out"].as_str().unwrap());
        rows.push(format!("chk-fuint {val} {base} \"{out}\""));
    }

    run_strconv_rows("strconv-bool-int", &rows);
}

/// Floating-point parsing: all 318 decimal, hexadecimal, special, midpoint, double-rounding, and error vectors.
#[test]
fn strconv_parse_float_vectors() {
    let f = fixtures();
    let mut rows = Vec::new();
    for v in f["parse_float"].as_array().unwrap() {
        let inp = hex_of(v["input"].as_str().unwrap());
        let bits_size = v["bit_size"].as_i64().unwrap();
        let raw_bits = v["bits"].as_u64().unwrap();
        let is_nan = if v["is_nan"].as_bool().unwrap() { "True" } else { "False" };
        let err = cinch_err_opt(v["err"].as_str().unwrap());
        if bits_size == 64 {
            rows.push(format!("chk-pf64 \"{inp}\" {raw_bits} {is_nan} {err}"));
        } else {
            rows.push(format!("chk-pf32 \"{inp}\" {raw_bits} {is_nan} {err}"));
        }
    }
    run_strconv_rows("strconv-parse-float", &rows);
}

/// Floating-point formatting (binary64): formats b, e, E, f, g, G, x, X across shortest and explicit precisions.
#[test]
fn strconv_format_float_64_vectors() {
    let f = fixtures();
    let f64_rows: Vec<String> = f["format_float"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|v| v["bit_size"].as_i64().unwrap() == 64)
        .map(|v| {
            let bits = v["bits"].as_u64().unwrap();
            let fmt = v["fmt"].as_str().unwrap().chars().next().unwrap();
            let prec = cinch_i64(v["prec"].as_i64().unwrap());
            let out = hex_of(v["out"].as_str().unwrap());
            format!("chk-ff64 {bits} '{fmt}' {prec} \"{out}\"")
        })
        .collect();
    assert!(!f64_rows.is_empty());
    for (idx, chunk) in f64_rows.chunks(500).enumerate() {
        run_strconv_rows(&format!("strconv-ff64-{idx}"), chunk);
    }
}

/// Floating-point formatting (binary32): formats b, e, E, f, g, G, x, X across shortest and explicit precisions.
#[test]
fn strconv_format_float_32_vectors() {
    let f = fixtures();
    let f32_rows: Vec<String> = f["format_float"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|v| v["bit_size"].as_i64().unwrap() == 32)
        .map(|v| {
            let bits = v["bits"].as_u64().unwrap();
            let fmt = v["fmt"].as_str().unwrap().chars().next().unwrap();
            let prec = cinch_i64(v["prec"].as_i64().unwrap());
            let out = hex_of(v["out"].as_str().unwrap());
            format!("chk-ff32 {bits} '{fmt}' {prec} \"{out}\"")
        })
        .collect();
    assert!(!f32_rows.is_empty());
    for (idx, chunk) in f32_rows.chunks(500).enumerate() {
        run_strconv_rows(&format!("strconv-ff32-{idx}"), chunk);
    }
}

/// Quoting, unquoting, quoted-prefix, unquote-char, can-backquote, is-print, and is-graphic vectors.
#[test]
fn strconv_quote_unquote_and_rune_vectors() {
    let f = fixtures();
    let mut rows = Vec::new();

    for v in f["quote"].as_array().unwrap() {
        let inp_hex = v["input_hex"].as_str().unwrap();
        let valid_utf8 = if v["valid_utf8"].as_bool().unwrap() { "True" } else { "False" };
        let q = hex_of(v["quote"].as_str().unwrap());
        let a = hex_of(v["ascii"].as_str().unwrap());
        let g = hex_of(v["graphic"].as_str().unwrap());
        rows.push(format!("chk-quote \"{inp_hex}\" {valid_utf8} \"{q}\" \"{a}\" \"{g}\""));
    }
    for v in f["quote_rune"].as_array().unwrap() {
        let code = v["code"].as_i64().unwrap();
        let q = hex_of(v["quote"].as_str().unwrap());
        let a = hex_of(v["ascii"].as_str().unwrap());
        let g = hex_of(v["graphic"].as_str().unwrap());
        rows.push(format!("chk-qrune {code} \"{q}\" \"{a}\" \"{g}\""));
    }
    for v in f["unquote"].as_array().unwrap() {
        let inp = hex_of(v["input"].as_str().unwrap());
        let out_hex = v["out_hex"].as_str().unwrap();
        let valid_utf8 = if v["valid_utf8"].as_bool().unwrap() { "True" } else { "False" };
        let err = cinch_err_opt(v["err"].as_str().unwrap());
        rows.push(format!("chk-unquote \"{inp}\" \"{out_hex}\" {valid_utf8} {err}"));
    }
    for v in f["quoted_prefix"].as_array().unwrap() {
        let inp = hex_of(v["input"].as_str().unwrap());
        let out = hex_of(v["out"].as_str().unwrap());
        let err = cinch_err_opt(v["err"].as_str().unwrap());
        rows.push(format!("chk-qprefix \"{inp}\" \"{out}\" {err}"));
    }
    for v in f["unquote_char"].as_array().unwrap() {
        let inp = hex_of(v["input"].as_str().unwrap());
        let q = v["quote"].as_i64().unwrap();
        let val = v["value"].as_i64().unwrap();
        let mb = if v["multibyte"].as_bool().unwrap() { "True" } else { "False" };
        let tail = hex_of(v["tail"].as_str().unwrap());
        let err = cinch_err_opt(v["err"].as_str().unwrap());
        rows.push(format!("chk-uchar \"{inp}\" {q} {val} {mb} \"{tail}\" {err}"));
    }
    for v in f["can_backquote"].as_array().unwrap() {
        let inp = hex_of(v["input"].as_str().unwrap());
        let out = if v["out"].as_bool().unwrap() { "True" } else { "False" };
        rows.push(format!("can-backquote (hs \"{inp}\") == {out}"));
    }
    for v in f["rune_class"].as_array().unwrap() {
        let code = v["code"].as_i64().unwrap();
        let ip = if v["is_print"].as_bool().unwrap() { "True" } else { "False" };
        let ig = if v["is_graphic"].as_bool().unwrap() { "True" } else { "False" };
        rows.push(format!("chk-rclass {code} {ip} {ig}"));
    }

    run_strconv_rows("strconv-quote-rune", &rows);
}

/// Complex number parsing and formatting (complex128 and complex64).
#[test]
fn strconv_complex_vectors() {
    let f = fixtures();
    let mut rows = Vec::new();
    for v in f["parse_complex"].as_array().unwrap() {
        let inp = hex_of(v["input"].as_str().unwrap());
        let bits_size = v["bit_size"].as_i64().unwrap();
        let rb = v["real_bits"].as_u64().unwrap();
        let rnan = if v["real_is_nan"].as_bool().unwrap() { "True" } else { "False" };
        let ib = v["imag_bits"].as_u64().unwrap();
        let inan = if v["imag_is_nan"].as_bool().unwrap() { "True" } else { "False" };
        let err = cinch_err_opt(v["err"].as_str().unwrap());
        if bits_size == 128 {
            rows.push(format!("chk-pc128 \"{inp}\" {rb} {rnan} {ib} {inan} {err}"));
        } else {
            rows.push(format!("chk-pc64 \"{inp}\" {rb} {rnan} {ib} {inan} {err}"));
        }
    }
    for v in f["format_complex"].as_array().unwrap() {
        let rb = v["real_bits"].as_u64().unwrap();
        let ib = v["imag_bits"].as_u64().unwrap();
        let fmt = v["fmt"].as_str().unwrap().chars().next().unwrap();
        let prec = cinch_i64(v["prec"].as_i64().unwrap());
        let bits_size = v["bit_size"].as_i64().unwrap();
        let out = hex_of(v["out"].as_str().unwrap());
        if bits_size == 128 {
            rows.push(format!("chk-fc128 {rb} {ib} '{fmt}' {prec} \"{out}\""));
        } else {
            rows.push(format!("chk-fc64 {rb} {ib} '{fmt}' {prec} \"{out}\""));
        }
    }
    run_strconv_rows("strconv-complex", &rows);
}

/// Append APIs, prefix immutability, int-size, and explicit error contracts (bad base, bad format, messages).
#[test]
fn strconv_append_and_error_contracts() {
    let rows: Vec<String> = vec![
        "int-size == 64".into(),
        "strconv-error-message StrconvSyntax == \"invalid syntax\"".into(),
        "strconv-error-message StrconvRange == \"value out of range\"".into(),
        "strconv-error-message StrconvBadBase == \"invalid base\"".into(),
        "strconv-error-message StrconvBadBitSize == \"invalid bit size\"".into(),
        "strconv-error-message StrconvBadFormat == \"invalid format\"".into(),
        "append-bool (string-to-bytes \"v=\") True == string-to-bytes \"v=true\"".into(),
        "append-bool (string-to-bytes \"v=\") False == string-to-bytes \"v=false\"".into(),
        "(case append-int (string-to-bytes \"x=\") (wrapping-neg-i64 42) 16 of Ok b -> b == string-to-bytes \"x=-2a\"; Err _ -> False)".into(),
        "(case append-int (string-to-bytes \"x=\") 10 1 of Err StrconvBadBase -> True; _ -> False)".into(),
        "(case append-uint (string-to-bytes \"u=\") 255 16 of Ok b -> b == string-to-bytes \"u=ff\"; Err _ -> False)".into(),
        "(case append-uint (string-to-bytes \"u=\") 10 37 of Err StrconvBadBase -> True; _ -> False)".into(),
        "(case format-int 10 1 of Err StrconvBadBase -> True; _ -> False)".into(),
        "(case format-int 10 37 of Err StrconvBadBase -> True; _ -> False)".into(),
        "(case format-uint 10 1 of Err StrconvBadBase -> True; _ -> False)".into(),
        "(case format-uint 10 37 of Err StrconvBadBase -> True; _ -> False)".into(),
        "(case append-float64 (string-to-bytes \"f=\") (f64-from-bits 4609434218613702656) 'f' 2 of Ok b -> b == string-to-bytes \"f=1.50\"; Err _ -> False)".into(),
        "(case append-float32 (string-to-bytes \"f=\") (f32-from-bits 1069547520) 'f' 2 of Ok b -> b == string-to-bytes \"f=1.50\"; Err _ -> False)".into(),
        "(case format-float64 1 'z' (wrapping-neg-i64 1) of Err StrconvBadFormat -> True; _ -> False)".into(),
        "(case format-float32 1 'z' (wrapping-neg-i64 1) of Err StrconvBadFormat -> True; _ -> False)".into(),
        "(case append-float64 (bytes-new 0) 1 'q' 2 of Err StrconvBadFormat -> True; _ -> False)".into(),
        "(case append-float32 (bytes-new 0) 1 'q' 2 of Err StrconvBadFormat -> True; _ -> False)".into(),
        "(case format-complex128 (1, 2) 'z' (wrapping-neg-i64 1) of Err StrconvBadFormat -> True; _ -> False)".into(),
        "(case format-complex64 (1, 2) 'z' (wrapping-neg-i64 1) of Err StrconvBadFormat -> True; _ -> False)".into(),
        "append-quote (string-to-bytes \"q:\") \"hi\" == string-to-bytes \"q:\\\"hi\\\"\"".into(),
        "append-quote-to-ascii (string-to-bytes \"q:\") \"世\" == string-to-bytes \"q:\\\"\\\\u4e16\\\"\"".into(),
        "append-quote-to-graphic (string-to-bytes \"q:\") \"\\u{00a0}\" == string-to-bytes \"q:\\\"\\u{00a0}\\\"\"".into(),
        "append-quote-rune (string-to-bytes \"r:\") 'a' == string-to-bytes \"r:'a'\"".into(),
        "append-quote-rune-to-ascii (string-to-bytes \"r:\") '世' == string-to-bytes \"r:'\\\\u4e16'\"".into(),
        "append-quote-rune-to-graphic (string-to-bytes \"r:\") '\\u{00a0}' == string-to-bytes \"r:'\\u{00a0}'\"".into(),
    ];
    run_strconv_rows("strconv-contracts", &rows);
}

const HELPERS: &str = r#"-- | Strconv differential test helpers.
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Char
  import Data.Float
  import Data.Int
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Text.Hex
  import Text.Strconv

  -- | Decode hex to Bytes.
  hb :: String -> Bytes
  hb h = case hex-decode h of Ok b -> b; Err _ -> bytes-new 0

  -- | Decode hex to UTF-8 String.
  hs :: String -> String
  hs h = case string-from-bytes (hb h) of Some s -> s; None -> empty-string

  -- | Compare two Option StrconvError values.
  eq-err-opt :: Option StrconvError -> Option StrconvError -> Bool
  eq-err-opt a b = case (a, b) of
    (None, None) -> True
    (Some x, Some y) -> x == y
    _ -> False

  -- | Check parse-bool.
  chk-pbool :: String -> Bool -> Option StrconvError -> Bool
  chk-pbool hex-in want-val want-err =
    case (parse-bool (hs hex-in), want-err) of
      (Ok v, None) -> v == want-val
      (Err e, Some we) -> e == we
      _ -> False

  -- | Check parse-int and parse-int-with-clamp.
  chk-pint :: String -> I64 -> I64 -> I64 -> Option StrconvError -> Bool
  chk-pint hex-in base bits want-val want-err =
    let s = hs hex-in in
    let ok1 = (case parse-int-with-clamp s base bits of (v, e) -> eq-i64 v want-val && eq-err-opt e want-err) in
    let ok2 = (case (parse-int s base bits, want-err) of (Ok v, None) -> eq-i64 v want-val; (Err e, Some we) -> e == we; _ -> False) in
    ok1 && ok2

  -- | Check parse-uint and parse-uint-with-clamp.
  chk-puint :: String -> I64 -> I64 -> U64 -> Option StrconvError -> Bool
  chk-puint hex-in base bits want-val want-err =
    let s = hs hex-in in
    let ok1 = (case parse-uint-with-clamp s base bits of (v, e) -> eq-u64 v want-val && eq-err-opt e want-err) in
    let ok2 = (case (parse-uint s base bits, want-err) of (Ok v, None) -> eq-u64 v want-val; (Err e, Some we) -> e == we; _ -> False) in
    ok1 && ok2

  -- | Check atoi and atoi-with-clamp.
  chk-atoi :: String -> I64 -> Option StrconvError -> Bool
  chk-atoi hex-in want-val want-err =
    let s = hs hex-in in
    let ok1 = (case atoi-with-clamp s of (v, e) -> eq-i64 v want-val && eq-err-opt e want-err) in
    let ok2 = (case (atoi s, want-err) of (Ok v, None) -> eq-i64 v want-val; (Err e, Some we) -> e == we; _ -> False) in
    ok1 && ok2

  -- | Check format-int (and itoa when base == 10).
  chk-fint :: I64 -> I64 -> String -> Bool
  chk-fint val base want-hex =
    let want = hs want-hex in
    let ok1 = (case format-int val base of Ok s -> s == want; Err _ -> False) in
    let ok2 = if eq-i64 base 10 then itoa val == want else True in
    ok1 && ok2

  -- | Check format-uint.
  chk-fuint :: U64 -> I64 -> String -> Bool
  chk-fuint val base want-hex =
    case format-uint val base of Ok s -> s == hs want-hex; Err _ -> False

  -- | Check F64 value against expected bit pattern or NaN.
  f64-matches :: F64 -> U64 -> Bool -> Bool
  f64-matches v want-bits want-nan =
    if want-nan then is-nan-f64 v else eq-u64 (f64-bits v) want-bits

  -- | Check F32 value against expected bit pattern or NaN.
  f32-matches :: F32 -> U32 -> Bool -> Bool
  f32-matches v want-bits want-nan =
    if want-nan then is-na-nf32 v else eq-u32 (f32-bits v) want-bits

  -- | Check parse-float64 and parse-float64-with-clamp.
  chk-pf64 :: String -> U64 -> Bool -> Option StrconvError -> Bool
  chk-pf64 hex-in want-bits want-nan want-err =
    let s = hs hex-in in
    let ok1 = (case parse-float64-with-clamp s of (v, e) -> f64-matches v want-bits want-nan && eq-err-opt e want-err) in
    let ok2 = (case (parse-float64 s, want-err) of (Ok v, None) -> f64-matches v want-bits want-nan; (Err e, Some we) -> e == we; _ -> False) in
    ok1 && ok2

  -- | Check parse-float32 and parse-float32-with-clamp.
  chk-pf32 :: String -> U32 -> Bool -> Option StrconvError -> Bool
  chk-pf32 hex-in want-bits want-nan want-err =
    let s = hs hex-in in
    let ok1 = (case parse-float32-with-clamp s of (v, e) -> f32-matches v want-bits want-nan && eq-err-opt e want-err) in
    let ok2 = (case (parse-float32 s, want-err) of (Ok v, None) -> f32-matches v want-bits want-nan; (Err e, Some we) -> e == we; _ -> False) in
    ok1 && ok2

  -- | Check format-float64.
  chk-ff64 :: U64 -> Char -> I64 -> String -> Bool
  chk-ff64 bits fmt prec want-hex =
    case format-float64 (f64-from-bits bits) fmt prec of
      Ok s -> s == hs want-hex
      Err _ -> False

  -- | Check format-float32.
  chk-ff32 :: U32 -> Char -> I64 -> String -> Bool
  chk-ff32 bits fmt prec want-hex =
    case format-float32 (f32-from-bits bits) fmt prec of
      Ok s -> s == hs want-hex
      Err _ -> False

  -- | Check quote-bytes* and (when valid UTF-8) quote*.
  chk-quote :: String -> Bool -> String -> String -> String -> Bool
  chk-quote inp-hex valid-utf8 q-hex a-hex g-hex =
    let b = hb inp-hex in
    let wq = hs q-hex in
    let wa = hs a-hex in
    let wg = hs g-hex in
    let ok-bytes = quote-bytes b == wq && quote-bytes-to-ascii b == wa && quote-bytes-to-graphic b == wg in
    let ok-str = (if valid-utf8 then (let s = hs inp-hex in quote s == wq && quote-to-ascii s == wa && quote-to-graphic s == wg) else True) in
    ok-bytes && ok-str

  -- | Check quote-rune, quote-rune-to-ascii, quote-rune-to-graphic.
  chk-qrune :: Nat -> String -> String -> String -> Bool
  chk-qrune code q-hex a-hex g-hex =
    case char-from-code code of
      None -> False
      Some c -> quote-rune c == hs q-hex && quote-rune-to-ascii c == hs a-hex && quote-rune-to-graphic c == hs g-hex

  -- | Check unquote-bytes and unquote.
  chk-unquote :: String -> String -> Bool -> Option StrconvError -> Bool
  chk-unquote inp-hex out-hex valid-utf8 want-err =
    let s = hs inp-hex in
    let ok-bytes = (case (unquote-bytes s, want-err) of (Ok b, None) -> b == hb out-hex; (Err e, Some we) -> e == we; _ -> False) in
    let ok-str = (case want-err of Some we -> (case unquote s of Err e -> e == we; Ok _ -> False); None -> (if valid-utf8 then (case unquote s of Ok t -> t == hs out-hex; Err _ -> False) else (case unquote s of Err StrconvSyntax -> True; _ -> False))) in
    ok-bytes && ok-str

  -- | Check quoted-prefix.
  chk-qprefix :: String -> String -> Option StrconvError -> Bool
  chk-qprefix inp-hex out-hex want-err =
    case (quoted-prefix (hs inp-hex), want-err) of
      (Ok s, None) -> s == hs out-hex
      (Err e, Some we) -> e == we
      _ -> False

  -- | Check unquote-char.
  chk-uchar :: String -> Nat -> Nat -> Bool -> String -> Option StrconvError -> Bool
  chk-uchar inp-hex q-code want-code want-mb tail-hex want-err =
    case char-from-code q-code of
      None -> False
      Some qch ->
        case (unquote-char (hs inp-hex) qch, want-err) of
          (Ok (ch, mb, tail), None) -> char-code ch == want-code && mb == want-mb && tail == hs tail-hex
          (Err e, Some we) -> e == we
          _ -> False

  -- | Check is-print and is-graphic.
  chk-rclass :: Nat -> Bool -> Bool -> Bool
  chk-rclass code want-print want-graphic =
    case char-from-code code of
      None -> False
      Some c -> is-print c == want-print && is-graphic c == want-graphic

  -- | Check parse-complex128 and parse-complex128-with-clamp.
  chk-pc128 :: String -> U64 -> Bool -> U64 -> Bool -> Option StrconvError -> Bool
  chk-pc128 inp-hex rb rnan ib inan want-err =
    let s = hs inp-hex in
    let ok1 = (case parse-complex128-with-clamp s of ((re, im), e) -> f64-matches re rb rnan && f64-matches im ib inan && eq-err-opt e want-err) in
    let ok2 = (case (parse-complex128 s, want-err) of (Ok (re, im), None) -> f64-matches re rb rnan && f64-matches im ib inan; (Err e, Some we) -> e == we; _ -> False) in
    ok1 && ok2

  -- | Check parse-complex64 and parse-complex64-with-clamp.
  chk-pc64 :: String -> U32 -> Bool -> U32 -> Bool -> Option StrconvError -> Bool
  chk-pc64 inp-hex rb rnan ib inan want-err =
    let s = hs inp-hex in
    let ok1 = (case parse-complex64-with-clamp s of ((re, im), e) -> f32-matches re rb rnan && f32-matches im ib inan && eq-err-opt e want-err) in
    let ok2 = (case (parse-complex64 s, want-err) of (Ok (re, im), None) -> f32-matches re rb rnan && f32-matches im ib inan; (Err e, Some we) -> e == we; _ -> False) in
    ok1 && ok2

  -- | Check format-complex128.
  chk-fc128 :: U64 -> U64 -> Char -> I64 -> String -> Bool
  chk-fc128 rb ib fmt prec want-hex =
    case format-complex128 (f64-from-bits rb, f64-from-bits ib) fmt prec of
      Ok s -> s == hs want-hex
      Err _ -> False

  -- | Check format-complex64.
  chk-fc64 :: U32 -> U32 -> Char -> I64 -> String -> Bool
  chk-fc64 rb ib fmt prec want-hex =
    case format-complex64 (f32-from-bits rb, f32-from-bits ib) fmt prec of
      Ok s -> s == hs want-hex
      Err _ -> False

  -- | Return the index of the first failing row, or 200 if all rows passed.
  first :: List Bool -> Int -> Int
  first xs i = case xs of
    Nil -> 200
    Cons ok rest -> if ok then first rest (i + 1) else i
"#;
