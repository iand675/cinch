//! Comprehensive tests for `Text.Gob` and `Effect.Gob` against Go 1.24's `encoding/gob`:
//! 1. Decodes all 76 Go-generated wire vectors (`seed/tests/fixtures/gob-vectors.json`) covering
//!    primitives, byte-reversed floats (including -0.0, +/-Inf, NaN, subnormal), strings, bytes,
//!    complex128, slices, fixed arrays, maps, structs with omitted zero fields and nested structs,
//!    and multi-value streams reusing transmitted type descriptors; plus full re-encode roundtrips.
//! 2. Bidirectional wire interop: Cinch encodes values and multi-message streams using `Text.Gob`
//!    and `Effect.Gob`, and Go's real `encoding/gob.NewDecoder` decodes and verifies every byte.
//! 3. Streaming `Effect.Gob` over `Effect.Stream` with type-ID reuse verification, 1-byte chunked
//!    readers, short-write writers, clean EOF vs truncation, and struct field helpers.
//! 4. Rejects all 18 malformed, truncated, overflow, unknown-type, duplicate-type, cyclic, and
//!    corrupt wire streams in both `Text.Gob` and `Effect.Gob`.
//! 5. Verifies canonical module-owned documentation on `Text.Gob` and `Effect.Gob`.
mod common;

use serde_json::Value;
use std::process::Command;

/// Shared prelude files required by `Text.Gob`, `Effect.Gob`, and the test helpers.
fn gob_project(tag: &str, main: &str) -> std::path::PathBuf {
    common::project(
        tag,
        &[
            ("Data.Float.cinch", include_str!("../prelude/Data/Float.cinch")),
            ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
            ("Effect.Stream.cinch", include_str!("../prelude/Effect/Stream.cinch")),
            ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
            ("Text.Gob.cinch", include_str!("../prelude/Text/Gob.cinch")),
            ("Effect.Gob.cinch", include_str!("../prelude/Effect/Gob.cinch")),
            ("Main.cinch", main),
        ],
    )
}

/// Format a Rust string as a Cinch string literal.
fn lit(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\r' => out.push_str("\\r"),
            '\n' => out.push_str("\\n"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Format a signed 64-bit integer string as a Cinch `I64` expression (handles MinInt64 safely).
fn i64_expr(s: &str) -> String {
    if s == "-9223372036854775808" {
        "min-bound-i64".to_string()
    } else if let Some(rest) = s.strip_prefix('-') {
        format!("(wrapping-neg-i64 {rest})")
    } else {
        s.to_string()
    }
}

/// Format an unsigned 64-bit integer string as a Cinch `U64` expression.
fn u64_expr(s: &str) -> String {
    s.to_string()
}

/// Format a 16-hex-digit IEEE-754 bit pattern as a Cinch `F64` expression.
fn f64_bits_expr(hex16: &str) -> String {
    format!("(f64-from-bits 0x{hex16})")
}

/// Build the Cinch check expression for one Go reference vector.
fn vector_check_expr(v: &Value) -> String {
    let cat = v["category"].as_str().unwrap();
    let hex_wire = lit(v["hex"].as_str().unwrap());
    let exp = &v["expected"];
    match cat {
        "bool" => {
            let b = if exp.as_bool().unwrap() { "True" } else { "False" };
            format!("check-one {hex_wire} (GobBool {b})")
        }
        "int" => {
            let i = i64_expr(exp.as_str().unwrap());
            format!("check-one {hex_wire} (GobInt {i})")
        }
        "uint" => {
            let u = u64_expr(exp.as_str().unwrap());
            format!("check-one {hex_wire} (GobUint {u})")
        }
        "float" => {
            let f = f64_bits_expr(exp.as_str().unwrap());
            format!("check-one {hex_wire} (GobFloat {f})")
        }
        "string" => {
            let s = lit(exp.as_str().unwrap());
            format!("check-one {hex_wire} (GobString {s})")
        }
        "bytes" => {
            let bhex = lit(exp.as_str().unwrap());
            format!("check-one {hex_wire} (GobBytes (must-hex {bhex}))")
        }
        "complex" => {
            let arr = exp.as_array().unwrap();
            let re = f64_bits_expr(arr[0].as_str().unwrap());
            let im = f64_bits_expr(arr[1].as_str().unwrap());
            format!("check-one {hex_wire} (GobComplex {re} {im})")
        }
        "slice_int" => {
            let elems: Vec<String> = exp
                .as_array()
                .unwrap()
                .iter()
                .map(|x| format!("GobInt {}", i64_expr(x.as_str().unwrap())))
                .collect();
            format!("check-one {hex_wire} (gob-slice [{}])", elems.join(", "))
        }
        "slice_str" => {
            let elems: Vec<String> = exp
                .as_array()
                .unwrap()
                .iter()
                .map(|x| format!("GobString {}", lit(x.as_str().unwrap())))
                .collect();
            format!("check-one {hex_wire} (gob-slice [{}])", elems.join(", "))
        }
        "slice_nested" => {
            let outer: Vec<String> = exp
                .as_array()
                .unwrap()
                .iter()
                .map(|row| {
                    let inner: Vec<String> = row
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|x| format!("GobInt {}", i64_expr(x.as_str().unwrap())))
                        .collect();
                    format!("gob-slice [{}]", inner.join(", "))
                })
                .collect();
            format!("check-one {hex_wire} (gob-slice [{}])", outer.join(", "))
        }
        "array_int" => {
            let elems: Vec<String> = exp
                .as_array()
                .unwrap()
                .iter()
                .map(|x| format!("GobInt {}", i64_expr(x.as_str().unwrap())))
                .collect();
            format!("check-one {hex_wire} (gob-array [{}])", elems.join(", "))
        }
        "array_uint" => {
            let elems: Vec<String> = exp
                .as_array()
                .unwrap()
                .iter()
                .map(|x| format!("GobUint {}", u64_expr(x.as_str().unwrap())))
                .collect();
            format!("check-one {hex_wire} (gob-array [{}])", elems.join(", "))
        }
        "map_str_int" => {
            let mut pairs: Vec<String> = exp
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| format!("(GobString {}, GobInt {})", lit(k), i64_expr(v.as_str().unwrap())))
                .collect();
            pairs.sort();
            format!("check-map {hex_wire} [{}]", pairs.join(", "))
        }
        "map_int_str" => {
            let mut pairs: Vec<String> = exp
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| format!("(GobInt {}, GobString {})", i64_expr(k), lit(v.as_str().unwrap())))
                .collect();
            pairs.sort();
            format!("check-map {hex_wire} [{}]", pairs.join(", "))
        }
        "struct_empty" => format!("check-one {hex_wire} (gob-struct \"Empty\" [])"),
        "struct_point" => {
            let x = i64_expr(exp["X"].as_str().unwrap());
            let y = i64_expr(exp["Y"].as_str().unwrap());
            format!("check-one {hex_wire} (gob-struct \"Point\" [(\"X\", GobInt {x}), (\"Y\", GobInt {y})])")
        }
        "struct_outer" => {
            let title = lit(exp["Title"].as_str().unwrap());
            let ic = i64_expr(exp["Inner"]["Code"].as_str().unwrap());
            let il = lit(exp["Inner"]["Label"].as_str().unwrap());
            let cnt = i64_expr(exp["Count"].as_str().unwrap());
            let ec = i64_expr(exp["Extra"]["Code"].as_str().unwrap());
            let el = lit(exp["Extra"]["Label"].as_str().unwrap());
            format!(
                "check-one {hex_wire} (gob-struct \"Outer\" [(\"Title\", GobString {title}), (\"Inner\", gob-struct \"Inner\" [(\"Code\", GobInt {ic}), (\"Label\", GobString {il})]), (\"Count\", GobInt {cnt}), (\"Extra\", gob-struct \"Inner\" [(\"Code\", GobInt {ec}), (\"Label\", GobString {el})])])"
            )
        }
        "struct_person" => {
            let name = lit(exp["Name"].as_str().unwrap());
            let age = i64_expr(exp["Age"].as_str().unwrap());
            let active = if exp["Active"].as_bool().unwrap() { "True" } else { "False" };
            let score = f64_bits_expr(exp["Score"].as_str().unwrap());
            let tag = lit(exp["Tag"].as_str().unwrap());
            let scores: Vec<String> = exp["Scores"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| format!("GobInt {}", i64_expr(x.as_str().unwrap())))
                .collect();
            let meta: Vec<String> = exp["Meta"]
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| format!("(GobString {}, GobString {})", lit(k), lit(v.as_str().unwrap())))
                .collect();
            format!(
                "check-one {hex_wire} (gob-struct \"Person\" [(\"Name\", GobString {name}), (\"Age\", GobInt {age}), (\"Active\", GobBool {active}), (\"Score\", GobFloat {score}), (\"Tag\", GobBytes (must-hex {tag})), (\"Scores\", gob-slice [{}]), (\"Meta\", gob-map [{}])])",
                scores.join(", "),
                meta.join(", ")
            )
        }
        "struct_with_array" => {
            let id = i64_expr(exp["Id"].as_str().unwrap());
            let fixed: Vec<String> = exp["Fixed"]
                .as_array()
                .unwrap()
                .iter()
                .map(|x| format!("GobInt {}", i64_expr(x.as_str().unwrap())))
                .collect();
            let note = lit(exp["Note"].as_str().unwrap());
            format!(
                "check-one {hex_wire} (gob-struct \"WithArray\" [(\"Id\", GobInt {id}), (\"Fixed\", gob-array [{}]), (\"Note\", GobString {note})])",
                fixed.join(", ")
            )
        }
        "slice_points" => {
            let pts: Vec<String> = exp
                .as_array()
                .unwrap()
                .iter()
                .map(|p| {
                    let x = i64_expr(p["X"].as_str().unwrap());
                    let y = i64_expr(p["Y"].as_str().unwrap());
                    format!("gob-struct \"Point\" [(\"X\", GobInt {x}), (\"Y\", GobInt {y})]")
                })
                .collect();
            format!("check-one {hex_wire} (gob-slice [{}])", pts.join(", "))
        }
        "struct_with_complex" => {
            let c = exp["C"].as_array().unwrap();
            let zc = exp["ZeroC"].as_array().unwrap();
            let u = u64_expr(exp["U"].as_str().unwrap());
            format!(
                "check-one {hex_wire} (gob-struct \"WithComplex\" [(\"C\", GobComplex {} {}), (\"ZeroC\", GobComplex {} {}), (\"U\", GobUint {u})])",
                f64_bits_expr(c[0].as_str().unwrap()),
                f64_bits_expr(c[1].as_str().unwrap()),
                f64_bits_expr(zc[0].as_str().unwrap()),
                f64_bits_expr(zc[1].as_str().unwrap()),
            )
        }
        "stream_points" => {
            let pts: Vec<String> = exp
                .as_array()
                .unwrap()
                .iter()
                .map(|p| {
                    let x = i64_expr(p["X"].as_str().unwrap());
                    let y = i64_expr(p["Y"].as_str().unwrap());
                    format!("gob-struct \"Point\" [(\"X\", GobInt {x}), (\"Y\", GobInt {y})]")
                })
                .collect();
            format!("check-stream {hex_wire} [{}]", pts.join(", "))
        }
        "stream_mixed" => {
            format!(
                "check-stream {hex_wire} [GobInt (wrapping-neg-i64 42), GobString \"hello\", gob-struct \"Point\" [(\"X\", GobInt 3), (\"Y\", GobInt 4)], GobBool True, gob-struct \"Point\" [(\"X\", GobInt 5), (\"Y\", GobInt 6)], gob-slice [GobInt 7, GobInt 8], gob-slice [GobInt 9]]"
            )
        }
        other => panic!("unhandled vector category: {other}"),
    }
}

/// 1. Every Go 1.24 `encoding/gob` reference vector decodes accurately and round-trips cleanly.
#[test]
fn gob_decodes_all_go_reference_vectors() {
    let fixture: Value = serde_json::from_str(include_str!("fixtures/gob-vectors.json")).unwrap();
    let vectors = fixture["vectors"].as_array().unwrap();
    assert_eq!(vectors.len(), 76, "expected 76 Go gob reference vectors");

    let checks: Vec<String> = vectors.iter().map(vector_check_expr).collect();
    let main = format!(
        r#"-- | Verify all Go encoding/gob reference vectors and re-encode roundtrips.
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Float (f64-bits, f64-from-bits)
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Text.Gob
  import Text.Hex

  -- | Decode hex literal for test inputs.
  must-hex :: String -> Bytes
  must-hex s = case hex-decode s of
    Ok bs -> bs
    Err _ -> bytes-new 0

  -- | Check whether a (key, value) pair is present in a map entries list.
  map-has :: (GobValue, GobValue) -> List (GobValue, GobValue) -> Bool
  map-has item entries = case entries of
    Nil -> False
    Cons e rest -> case item of
      (k1, v1) -> case e of
        (k2, v2) -> if gob-value-eq k1 k2 && gob-value-eq v1 v2 then True else map-has item rest

  -- | Check whether every pair in xs appears in ys.
  map-subset :: List (GobValue, GobValue) -> List (GobValue, GobValue) -> Bool
  map-subset xs ys = case xs of
    Nil -> True
    Cons x rest -> map-has x ys && map-subset rest ys

  -- | Count list items.
  len-of :: forall a. List a -> Nat -> Nat
  len-of xs n = case xs of Nil -> n; Cons _ r -> len-of r (n + 1)

  -- | Order-independent equality for GobMap values.
  map-eq-unordered :: List (GobValue, GobValue) -> List (GobValue, GobValue) -> Bool
  map-eq-unordered a b = len-of a 0 == len-of b 0 && map-subset a b && map-subset b a

  -- | Decode one Go wire vector, check value equality, re-encode, and verify roundtrip.
  check-one :: String -> GobValue -> Bool
  check-one hex-wire expected =
    let wire = must-hex hex-wire in
    case gob-decode wire of
      Err _ -> False
      Ok pair -> case pair of
        (ty, val) ->
          if not (gob-value-eq val expected) then False
          else case gob-encode ty val of
            Err _ -> False
            Ok rewired -> case gob-decode-as ty rewired of
              Err _ -> False
              Ok val2 -> gob-value-eq val2 expected

  -- | Decode one Go map wire vector (unordered entries) and verify roundtrip.
  check-map :: String -> List (GobValue, GobValue) -> Bool
  check-map hex-wire expected =
    let wire = must-hex hex-wire in
    case gob-decode wire of
      Err _ -> False
      Ok pair -> case pair of
        (ty, val) ->
          let entries = gob-map-entries val in
          if not (map-eq-unordered entries expected) then False
          else case gob-encode ty val of
            Err _ -> False
            Ok rewired -> case gob-decode-as ty rewired of
              Err _ -> False
              Ok val2 -> map-eq-unordered (gob-map-entries val2) expected

  -- | Compare decoded stream values with expected values in order.
  stream-vals-eq :: List (GobType, GobValue) -> List GobValue -> Bool
  stream-vals-eq got want = case got of
    Nil -> case want of Nil -> True; Cons _ _ -> False
    Cons g gr -> case want of
      Nil -> False
      Cons w wr -> case g of (_, gv) -> gob-value-eq gv w && stream-vals-eq gr wr

  -- | Decode a multi-message Go stream, verify every item, re-encode stream, and re-decode.
  check-stream :: String -> List GobValue -> Bool
  check-stream hex-wire expected =
    let wire = must-hex hex-wire in
    case gob-decode-stream wire of
      Err _ -> False
      Ok items ->
        if not (stream-vals-eq items expected) then False
        else case gob-encode-stream items of
          Err _ -> False
          Ok rewired -> case gob-decode-stream rewired of
            Err _ -> False
            Ok items2 -> stream-vals-eq items2 expected

  -- | Return 200 if all checks pass, or 1-based index of the first failing vector.
  run-checks :: List Bool -> Int -> Int
  run-checks xs idx = case xs of
    Nil -> 200
    Cons ok rest -> if ok then run-checks rest (idx + 1) else idx

  -- | Run all 76 Go reference vector checks.
  main :: Int
  main = run-checks [
    {}
  ] 1
"#,
        checks.join(",\n    ")
    );

    let dir = gob_project("gob_vectors", &main);
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(code, 200, "first failing Go reference vector (1-based index): {code}");
}

/// 2. Bidirectional wire interop: Go's real `encoding/gob.NewDecoder` decodes Cinch-encoded streams.
#[test]
fn gob_cinch_encoded_streams_decode_in_go() {
    let main = r#"-- | Encode gob payloads using Text.Gob and Effect.Gob and write hex lines for Go verification.
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Float (f64-from-bits)
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Path
  import Data.Result
  import Data.String
  import Effect.Fs
  import Effect.Gob
  import Effect.Stream
  import Text.Gob
  import Text.Hex

  -- | Point struct descriptor matching Go Point{X, Y int64}.
  point-type :: GobType
  point-type = gob-type-struct "Point" [("X", GobTInt), ("Y", GobTInt)]

  -- | Inner struct descriptor matching Go Inner{Code int64, Label string}.
  inner-type :: GobType
  inner-type = gob-type-struct "Inner" [("Code", GobTInt), ("Label", GobTString)]

  -- | Outer struct descriptor matching Go Outer{Title string, Inner Inner, Count int64, Extra Inner}.
  outer-type :: GobType
  outer-type = gob-type-struct "Outer" [
    ("Title", GobTString),
    ("Inner", inner-type),
    ("Count", GobTInt),
    ("Extra", inner-type)
  ]

  -- | Person struct descriptor matching Go Person.
  person-type :: GobType
  person-type = gob-type-struct "Person" [
    ("Name", GobTString),
    ("Age", GobTInt),
    ("Active", GobTBool),
    ("Score", GobTFloat),
    ("Tag", GobTBytes),
    ("Scores", GobTSlice GobTInt),
    ("Meta", GobTMap GobTString GobTString)
  ]

  -- | WithArray struct descriptor matching Go WithArray{Id int64, Fixed [3]int64, Note string}.
  with-array-type :: GobType
  with-array-type = gob-type-struct "WithArray" [
    ("Id", GobTInt),
    ("Fixed", GobTArray GobTInt 3),
    ("Note", GobTString)
  ]

  -- | Encode one named test case as "name=hex\n".
  line-of :: String -> Result GobError Bytes -> String
  line-of name res = case res of
    Ok bs -> string-concat name (string-concat "=" (string-concat (hex-encode bs) "\n"))
    Err e -> string-concat name (string-concat "=ERR:" (string-concat (gob-error-message e) "\n"))

  -- | Encode a multi-message stream via Effect.Gob and return hex line.
  stream-line :: forall r. using (io :: Io r) => String -> List (GobType, GobValue) ->{IO r} String
  stream-line name items =
    let enc0 = gob-stream-encoder writer-buffer in
    case gob-stream-encode-all enc0 items of
      (Err e, _) -> string-concat name (string-concat "=ERR:" (string-concat (gob-stream-error-message e) "\n"))
      (Ok _, enc1) -> case writer-bytes (gob-stream-encoder-sink enc1) of
        None -> string-concat name "=ERR:no-bytes\n"
        Some bs -> string-concat name (string-concat "=" (string-concat (hex-encode bs) "\n"))

  -- | Concatenate all output lines.
  join-lines :: List String -> String
  join-lines xs = case xs of
    Nil -> ""
    Cons x r -> string-concat x (join-lines r)

  -- | Encode all interop cases and write them to cinch_gob_out.txt.
  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let raw-tag = case hex-decode "deadbeef" of Ok b -> b; Err _ -> bytes-new 0 in
    let p1 = gob-struct "Point" [("X", GobInt 11), ("Y", GobInt (wrapping-neg-i64 22))] in
    let p2 = gob-struct "Point" [("Y", GobInt 99)] in
    let p3 = gob-struct "Point" [] in
    let outer-val = gob-struct "Outer" [
      ("Title", GobString "from-cinch"),
      ("Inner", gob-struct "Inner" []),
      ("Count", GobInt 7),
      ("Extra", gob-struct "Inner" [("Code", GobInt (wrapping-neg-i64 3)), ("Label", GobString "nested")])
    ] in
    let person-val = gob-struct "Person" [
      ("Name", GobString "Grace"),
      ("Age", GobInt 85),
      ("Active", GobBool True),
      ("Score", GobFloat (f64-from-bits 0x4058A00000000000)),
      ("Tag", GobBytes raw-tag),
      ("Scores", gob-slice [GobInt 0, GobInt 100, GobInt (wrapping-neg-i64 5)]),
      ("Meta", gob-map [(GobString "lang", GobString "cinch"), (GobString "fmt", GobString "gob")])
    ] in
    let arr-struct = gob-struct "WithArray" [
      ("Id", GobInt 42),
      ("Fixed", gob-array [GobInt 0, GobInt 0, GobInt 0]),
      ("Note", GobString "zero-array")
    ] in
    let s-points = stream-line "effect_stream_points" [
      (point-type, p1),
      (point-type, p2),
      (point-type, p3)
    ] in
    let s-mixed = stream-line "effect_stream_mixed" [
      (GobTInt, GobInt min-bound-i64),
      (GobTString, GobString "hello-go"),
      (outer-type, outer-val),
      (point-type, p1),
      (point-type, p2)
    ] in
    let body = join-lines [
      line-of "bool_true" (gob-encode GobTBool (GobBool True)),
      line-of "int_min64" (gob-encode GobTInt (GobInt min-bound-i64)),
      line-of "int_neg129" (gob-encode GobTInt (GobInt (wrapping-neg-i64 129))),
      line-of "uint_max64" (gob-encode GobTUint (GobUint 18446744073709551615)),
      line-of "float_17" (gob-encode GobTFloat (GobFloat (f64-from-bits 0x4031000000000000))),
      line-of "float_neg0" (gob-encode GobTFloat (GobFloat (f64-from-bits 0x8000000000000000))),
      line-of "float_nan" (gob-encode GobTFloat (GobFloat (f64-from-bits 0x7FF8000000000001))),
      line-of "string_utf8" (gob-encode GobTString (GobString "Cinch→Go🚀")),
      line-of "bytes_raw" (gob-encode GobTBytes (GobBytes raw-tag)),
      line-of "complex_val" (gob-encode GobTComplex (GobComplex (f64-from-bits 0xBFF8000000000000) (f64-from-bits 0x4012000000000000))),
      line-of "slice_int" (gob-encode (GobTSlice GobTInt) (gob-slice [GobInt 0, GobInt 1, GobInt (wrapping-neg-i64 1), GobInt 1024])),
      line-of "array_int4" (gob-encode (GobTArray GobTInt 4) (gob-array [GobInt 5, GobInt 0, GobInt (wrapping-neg-i64 7), GobInt 9])),
      line-of "map_str_int" (gob-encode (GobTMap GobTString GobTInt) (gob-map [(GobString "k1", GobInt 1), (GobString "k2", GobInt 0), (GobString "k3", GobInt (wrapping-neg-i64 3))])),
      line-of "struct_outer" (gob-encode outer-type outer-val),
      line-of "struct_person" (gob-encode person-type person-val),
      line-of "struct_with_array" (gob-encode with-array-type arr-struct),
      s-points,
      s-mixed
    ] in
    case fs-write (path-from-string "cinch_gob_out.txt") (string-to-bytes body) of
      Ok _ -> 200
      Err _ -> 1
"#;

    let dir = gob_project("gob_interop", main);
    let orig_cwd = std::env::current_dir().unwrap();
    // Run Cinch binary inside dir so cinch_gob_out.txt lands in dir.
    let exe = common::build(&dir, "Main.cinch", "gob_interop_bin").expect("build interop binary");
    let status = Command::new(&exe).current_dir(&dir).status().expect("run interop binary");
    assert_eq!(status.code(), Some(200), "Cinch interop encoder exited with {:?}", status.code());

    // Write a real Go 1.24 verifier using encoding/gob.NewDecoder to decode EVERY Cinch-encoded stream!
    let go_verifier = r#"package main

import (
	"bytes"
	"encoding/gob"
	"encoding/hex"
	"fmt"
	"io"
	"math"
	"os"
	"reflect"
	"strings"
)

type Point struct {
	X int64
	Y int64
}

type Inner struct {
	Code  int64
	Label string
}

type Outer struct {
	Title string
	Inner Inner
	Count int64
	Extra Inner
}

type Person struct {
	Name   string
	Age    int64
	Active bool
	Score  float64
	Tag    []byte
	Scores []int64
	Meta   map[string]string
}

type WithArray struct {
	Id    int64
	Fixed [3]int64
	Note  string
}

func mustDecode[T any](raw []byte) T {
	var out T
	dec := gob.NewDecoder(bytes.NewReader(raw))
	if err := dec.Decode(&out); err != nil {
		panic(fmt.Sprintf("gob.Decode failed: %v", err))
	}
	var extra any
	if err := dec.Decode(&extra); err != io.EOF {
		panic(fmt.Sprintf("expected EOF after single decode, got %v", err))
	}
	return out
}

func main() {
	data, err := os.ReadFile("cinch_gob_out.txt")
	if err != nil {
		panic(err)
	}
	cases := map[string][]byte{}
	for _, line := range strings.Split(strings.TrimSpace(string(data)), "\n") {
		parts := strings.SplitN(line, "=", 2)
		if len(parts) != 2 || strings.HasPrefix(parts[1], "ERR:") {
			panic("bad line: " + line)
		}
		raw, err := hex.DecodeString(parts[1])
		if err != nil {
			panic(err)
		}
		cases[parts[0]] = raw
	}

	if v := mustDecode[bool](cases["bool_true"]); !v {
		panic("bool_true mismatch")
	}
	if v := mustDecode[int64](cases["int_min64"]); v != math.MinInt64 {
		panic(fmt.Sprintf("int_min64 got %d", v))
	}
	if v := mustDecode[int64](cases["int_neg129"]); v != -129 {
		panic(fmt.Sprintf("int_neg129 got %d", v))
	}
	if v := mustDecode[uint64](cases["uint_max64"]); v != math.MaxUint64 {
		panic(fmt.Sprintf("uint_max64 got %d", v))
	}
	if v := mustDecode[float64](cases["float_17"]); v != 17.0 {
		panic(fmt.Sprintf("float_17 got %v", v))
	}
	if v := mustDecode[float64](cases["float_neg0"]); math.Float64bits(v) != 0x8000000000000000 {
		panic(fmt.Sprintf("float_neg0 bits %016x", math.Float64bits(v)))
	}
	if v := mustDecode[float64](cases["float_nan"]); !math.IsNaN(v) {
		panic(fmt.Sprintf("float_nan got %v", v))
	}
	if v := mustDecode[string](cases["string_utf8"]); v != "Cinch→Go🚀" {
		panic(fmt.Sprintf("string_utf8 got %q", v))
	}
	if v := mustDecode[[]byte](cases["bytes_raw"]); !bytes.Equal(v, []byte{0xde, 0xad, 0xbe, 0xef}) {
		panic(fmt.Sprintf("bytes_raw got %x", v))
	}
	if v := mustDecode[complex128](cases["complex_val"]); v != complex(-1.5, 4.5) {
		panic(fmt.Sprintf("complex_val got %v", v))
	}
	if v := mustDecode[[]int64](cases["slice_int"]); !reflect.DeepEqual(v, []int64{0, 1, -1, 1024}) {
		panic(fmt.Sprintf("slice_int got %v", v))
	}
	if v := mustDecode[[4]int64](cases["array_int4"]); v != [4]int64{5, 0, -7, 9} {
		panic(fmt.Sprintf("array_int4 got %v", v))
	}
	wantMap := map[string]int64{"k1": 1, "k2": 0, "k3": -3}
	if v := mustDecode[map[string]int64](cases["map_str_int"]); !reflect.DeepEqual(v, wantMap) {
		panic(fmt.Sprintf("map_str_int got %v", v))
	}
	wantOuter := Outer{
		Title: "from-cinch",
		Inner: Inner{Code: 0, Label: ""},
		Count: 7,
		Extra: Inner{Code: -3, Label: "nested"},
	}
	if v := mustDecode[Outer](cases["struct_outer"]); v != wantOuter {
		panic(fmt.Sprintf("struct_outer got %+v", v))
	}
	wantPerson := Person{
		Name:   "Grace",
		Age:    85,
		Active: true,
		Score:  98.5,
		Tag:    []byte{0xde, 0xad, 0xbe, 0xef},
		Scores: []int64{0, 100, -5},
		Meta:   map[string]string{"lang": "cinch", "fmt": "gob"},
	}
	if v := mustDecode[Person](cases["struct_person"]); !reflect.DeepEqual(v, wantPerson) {
		panic(fmt.Sprintf("struct_person got %+v", v))
	}
	wantArr := WithArray{Id: 42, Fixed: [3]int64{0, 0, 0}, Note: "zero-array"}
	if v := mustDecode[WithArray](cases["struct_with_array"]); v != wantArr {
		panic(fmt.Sprintf("struct_with_array got %+v", v))
	}

	// Verify multi-message stream of Points (reusing type descriptor).
	decPts := gob.NewDecoder(bytes.NewReader(cases["effect_stream_points"]))
	wantPts := []Point{{X: 11, Y: -22}, {X: 0, Y: 99}, {X: 0, Y: 0}}
	for i, w := range wantPts {
		var p Point
		if err := decPts.Decode(&p); err != nil {
			panic(fmt.Sprintf("stream point %d decode: %v", i, err))
		}
		if p != w {
			panic(fmt.Sprintf("stream point %d got %+v want %+v", i, p, w))
		}
	}
	var eofCheck Point
	if err := decPts.Decode(&eofCheck); err != io.EOF {
		panic(fmt.Sprintf("expected EOF on point stream, got %v", err))
	}

	// Verify multi-message mixed stream (reusing Point descriptor).
	decMix := gob.NewDecoder(bytes.NewReader(cases["effect_stream_mixed"]))
	var m0 int64
	var m1 string
	var m2 Outer
	var m3, m4 Point
	if err := decMix.Decode(&m0); err != nil || m0 != math.MinInt64 {
		panic(fmt.Sprintf("mixed[0]: %v %d", err, m0))
	}
	if err := decMix.Decode(&m1); err != nil || m1 != "hello-go" {
		panic(fmt.Sprintf("mixed[1]: %v %q", err, m1))
	}
	if err := decMix.Decode(&m2); err != nil || m2 != wantOuter {
		panic(fmt.Sprintf("mixed[2]: %v %+v", err, m2))
	}
	if err := decMix.Decode(&m3); err != nil || m3 != (Point{X: 11, Y: -22}) {
		panic(fmt.Sprintf("mixed[3]: %v %+v", err, m3))
	}
	if err := decMix.Decode(&m4); err != nil || m4 != (Point{X: 0, Y: 99}) {
		panic(fmt.Sprintf("mixed[4]: %v %+v", err, m4))
	}
	if err := decMix.Decode(&eofCheck); err != io.EOF {
		panic(fmt.Sprintf("expected EOF on mixed stream, got %v", err))
	}
	fmt.Println("GO_INTEROP_OK: 18/18 Cinch-encoded streams decoded cleanly by Go encoding/gob")
}
"#;

    std::fs::write(dir.join("verify_gob.go"), go_verifier).expect("write Go verifier");
    let go_out = Command::new("go")
        .args(["run", "verify_gob.go"])
        .current_dir(&dir)
        .env("GOCACHE", "/tmp/go-cache")
        .env("GOTELEMETRY", "off")
        .output()
        .expect("run Go encoding/gob verifier");
    let _ = orig_cwd;
    if !go_out.status.success() {
        let cinch_out = std::fs::read_to_string(dir.join("cinch_gob_out.txt")).unwrap_or_default();
        common::cleanup(&dir);
        panic!(
            "Go encoding/gob failed to decode Cinch-encoded stream!\nstdout: {}\nstderr: {}\ncinch_gob_out:\n{}",
            String::from_utf8_lossy(&go_out.stdout),
            String::from_utf8_lossy(&go_out.stderr),
            cinch_out
        );
    }
    common::cleanup(&dir);
}

/// 3. Streaming Effect.Gob roundtrip: type-ID reuse, 1-byte chunked reader, short-write writer,
///    clean EOF vs truncation, typed field helpers, and error diagnostics.
#[test]
fn gob_effect_stream_multi_message_roundtrip() {
    let main = r#"-- | Test Effect.Gob streaming encoder and decoder behavior.
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Float (f64-from-bits)
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Gob
  import Effect.Stream
  import Text.Gob

  -- | Point struct type.
  point-type :: GobType
  point-type = gob-type-struct "Point" [("X", GobTInt), ("Y", GobTInt)]

  -- | Build a ByteReader that delivers at most 1 byte per stream-read call.
  one-byte-reader :: forall r. using (io :: Io r) => Bytes -> Nat -> ByteReader r
  one-byte-reader buf pos = stream-reader (\n ->
    if pos >= bytes-len buf then (Ok (bytes-new 0), one-byte-reader buf pos)
    else
      let out = bytes-new 1 in
      let copied = bytes-copy-range out buf 0 pos 1 in
      (Ok out, one-byte-reader buf (pos + 1)))

  -- | Build a ByteWriter that accepts at most 2 bytes per stream-write call, delegating to an inner sink.
  two-byte-writer :: forall r. using (io :: Io r) => ByteWriter r -> ByteWriter r
  two-byte-writer inner = stream-writer (\chunk ->
    let take = if bytes-len chunk <= 2 then bytes-len chunk else 2 in
    if take <= bytes-len chunk then
      let prefix = bytes-new take in
      let copied = bytes-copy-range prefix chunk 0 0 take in
      case stream-write inner prefix of
        (Err e, next-inner) -> (Err e, two-byte-writer next-inner)
        (Ok k, next-inner) -> (Ok k, two-byte-writer next-inner)
    else (Ok 0, two-byte-writer inner))

  -- | Check streaming encode/decode with type-ID reuse, chunking, short writes, and helpers.
  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let enc0 = gob-stream-encoder (two-byte-writer writer-buffer) in
    let p1 = gob-struct "Point" [("X", GobInt 10), ("Y", GobInt 20)] in
    let p2 = gob-struct "Point" [("X", GobInt 0), ("Y", GobInt (wrapping-neg-i64 5))] in
    let p3 = gob-struct "Point" [("X", GobInt 99), ("Y", GobInt 0)] in
    -- First Point transmits type descriptor + value
    case gob-stream-encode enc0 point-type p1 of
      (Err _, _) -> 1
      (Ok n1, enc1) ->
        -- Verify type ID 65 was assigned and recorded
        let tid-ok = case gob-encoder-type-id (gob-stream-encoder-state enc1) point-type of
              Some tid -> eq-i64 tid 65
              None -> False in
        if not tid-ok then 2
        else
          -- Second Point reuses type ID 65 -> strictly fewer bytes than first!
          case gob-stream-encode enc1 point-type p2 of
            (Err _, _) -> 3
            (Ok n2, enc2) ->
              if not (n2 < n1) then 4
              else
                -- Type mismatch must fail cleanly without corrupting stream
                case gob-stream-encode enc2 point-type (GobInt 7) of
                  (Ok _, _) -> 5
                  (Err err-mm, enc2b) ->
                    let mm-ok = case err-mm of
                          GobStreamFormatFailure (GobTypeMismatch _) -> True
                          _ -> False in
                    if not mm-ok then 6
                    else
                      -- Encode third Point via regular buffered writer to test gob-stream-encode-all too
                      let enc-b0 = gob-stream-encoder writer-buffer in
                      case gob-stream-encode-all enc-b0 [(point-type, p1), (point-type, p2), (point-type, p3)] of
                        (Err _, _) -> 7
                        (Ok _, enc-b1) -> case writer-bytes (gob-stream-encoder-sink enc-b1) of
                          None -> 8
                          Some wire ->
                            -- Decode via 1-byte-at-a-time chunked ByteReader!
                            let dec0 = gob-stream-decoder (one-byte-reader wire 0) in
                            case gob-stream-decode dec0 of
                              (Err _, _) -> 9
                              (Ok r1, dec1) -> case r1 of
                                (t1, v1) ->
                                  if not (gob-type-eq t1 point-type && gob-value-eq v1 p1) then 10
                                  else
                                    -- Inspect registered decoder type 65
                                    let dec-ty-ok = case gob-decoder-type (gob-stream-decoder-state dec1) 65 of
                                          Ok dt -> gob-type-eq dt point-type
                                          Err _ -> False in
                                    if not dec-ty-ok then 11
                                    else
                                      -- Check typed struct field accessors on decoded v1
                                      let fields-ok = case gob-field-int "X" v1 of
                                            Ok x -> (case gob-field-int "Y" v1 of
                                              Ok y -> eq-i64 x 10 && eq-i64 y 20
                                              Err _ -> False)
                                            Err _ -> False in
                                      if not fields-ok then 12
                                      else
                                        -- Decode remaining two values via gob-stream-decode-all
                                        case gob-stream-decode-all dec1 of
                                          (Err _, _) -> 13
                                          (Ok rest, dec2) ->
                                            let rest-ok = case rest of
                                                  Cons (_, v2) (Cons (_, v3) Nil) ->
                                                    gob-value-eq v2 p2 && gob-value-eq v3 p3
                                                  _ -> False in
                                            if not rest-ok then 14
                                            else
                                              -- Next decode must return GobStreamEndOfInput
                                              case gob-stream-decode dec2 of
                                                (Err GobStreamEndOfInput, _) ->
                                                  -- Also verify gob-decode-as rejects mismatched type
                                                  case gob-encode GobTInt (GobInt 5) of
                                                    Err _ -> 15
                                                    Ok ib -> case gob-decode-as GobTString ib of
                                                      Err (GobTypeMismatch _) ->
                                                        -- Verify error message helpers return non-empty text
                                                        if string-byte-length (gob-stream-error-message GobStreamEndOfInput) > 0
                                                          && string-byte-length (gob-error-message (GobUnknownTypeId 99)) > 0
                                                        then 200
                                                        else 17
                                                      _ -> 16
                                                _ -> 18
"#;

    let dir = gob_project("gob_effect", main);
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(code, 200, "Effect.Gob test failed at step {code}");
}

/// 4. Malformed, truncated, overflow, unknown-type, duplicate-type, cyclic, and corrupt streams
///    are rejected cleanly by both `Text.Gob` and `Effect.Gob`.
#[test]
fn gob_rejects_malformed_and_corrupt_streams() {
    let fixture: Value = serde_json::from_str(include_str!("fixtures/gob-vectors.json")).unwrap();
    let invalid = fixture["invalid"].as_array().unwrap();
    assert_eq!(invalid.len(), 18, "expected 18 invalid gob vectors");

    let mut checks = Vec::new();
    for row in invalid {
        let hex = lit(row["hex"].as_str().unwrap());
        let reason = lit(row["reason"].as_str().unwrap());
        checks.push(format!("check-invalid {hex} {reason}"));
    }

    let main = format!(
        r#"-- | Verify all 18 invalid gob wire streams are rejected by Text.Gob and Effect.Gob.
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.List
  import Data.Machine
  import Data.Result
  import Data.String
  import Effect.Gob
  import Effect.Stream
  import Text.Gob
  import Text.Hex

  -- | Decode hex literal.
  must-hex :: String -> Bytes
  must-hex s = case hex-decode s of
    Ok bs -> bs
    Err _ -> bytes-new 0

  -- | Match GobError constructor against expected reason tag.
  matches-reason :: GobError -> String -> Bool
  matches-reason err reason = case err of
    GobTruncated -> eq-i64 (string-compare reason "truncated") 0
    GobOverflow -> eq-i64 (string-compare reason "overflow") 0
    GobUnknownTypeId _ -> eq-i64 (string-compare reason "unknown_type") 0
    GobMalformedWire _ -> eq-i64 (string-compare reason "malformed") 0
    GobDepthLimit -> eq-i64 (string-compare reason "depth_limit") 0
    GobTypeMismatch _ -> eq-i64 (string-compare reason "type_mismatch") 0

  -- | Verify both pure Text.Gob and streaming Effect.Gob reject the invalid wire bytes.
  check-invalid :: forall r. using (io :: Io r) => String -> String ->{{IO r}} Bool
  check-invalid hex-wire reason =
    let wire = must-hex hex-wire in
    let pure-ok = case gob-decode wire of
          Ok _ -> False
          Err e -> matches-reason e reason in
    let dec = gob-stream-decoder (reader-from-bytes wire) in
    let stream-ok = case gob-stream-decode dec of
          (Ok _, _) -> False
          (Err se, _) -> case se of
            GobStreamEndOfInput -> bytes-len wire == 0 && eq-i64 (string-compare reason "truncated") 0
            GobStreamFormatFailure ge -> matches-reason ge reason
            _ -> False in
    pure-ok && stream-ok

  -- | Evaluate checks in order, returning 200 on full pass or 1-based failing index.
  run-all :: List Bool -> Int -> Int
  run-all xs idx = case xs of
    Nil -> 200
    Cons ok rest -> if ok then run-all rest (idx + 1) else idx

  -- | Run all 18 invalid wire rejection tests.
  main :: Unit ->{{IO r}} Int
  main u = with (io = newIO ()) in run-all [
    {}
  ] 1
"#,
        checks.join(",\n    ")
    );

    let dir = gob_project("gob_invalid", &main);
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    assert_eq!(code, 200, "invalid vector check failed at 1-based index {code}");
}

/// 5. Every public and private declaration in Text.Gob and Effect.Gob has canonical documentation.
#[test]
fn gob_declarations_have_canonical_documentation() {
    common::assert_documented("Text.Gob", include_str!("../prelude/Text/Gob.cinch"));
    common::assert_documented("Effect.Gob", include_str!("../prelude/Effect/Gob.cinch"));
}
