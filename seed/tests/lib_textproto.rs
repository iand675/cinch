//! Text protocol framing against independent reference vectors from `tooling/textproto_vectors.go`:
//! canonical header keys, MIME header blocks (with continuations, repeats and malformed lines),
//! status lines, multi-line responses, and dot decoding and encoding.
mod common;

use serde_json::Value;

/// Compile and run checks; the exit status is the first failing check, 200 if none.
fn run(checks: &[String]) {
    let dir = common::project("textproto", &[
        ("Text.TextProto.cinch", include_str!("../prelude/Text/TextProto.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every check.\n  main :: Int\n  main = first [{}] 0\n", checks.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing check; 200 means every check passed");
    common::cleanup(&dir);
}

/// Hex string of a JSON string value.
fn s(v: &Value) -> String {
    format!("{:?}", v.as_str().unwrap())
}

/// Every reference vector in one run.
#[test]
fn textproto_matches_reference_framing() {
    let f: Value = serde_json::from_str(include_str!("fixtures/textproto-vectors.json")).unwrap();
    let mut checks = Vec::new();
    for pair in f["canonical"].as_array().unwrap() {
        checks.push(format!("canonical-case {} {}", s(&pair[0]), s(&pair[1])));
    }
    for row in f["headers"].as_array().unwrap() {
        let fields = row["fields"].as_array().map(|fs| fs.iter().map(|f| {
            let values = f.as_array().unwrap();
            format!("({}, [{}])", s(&values[0]), values[1..].iter().map(s).collect::<Vec<_>>().join(", "))
        }).collect::<Vec<_>>().join(", ")).unwrap_or_default();
        checks.push(format!("header-case {} {} [{fields}] {}", s(&row["input"]), if row["ok"].as_bool().unwrap() { "True" } else { "False" }, s(&row["rest"])));
    }
    for row in f["codes"].as_array().unwrap() {
        checks.push(format!("code-case {} {} {} {} {} {}", s(&row["input"]), row["expected"], if row["multi"].as_bool().unwrap() { "True" } else { "False" },
            if row["ok"].as_bool().unwrap() { "True" } else { "False" }, row["code"], s(&row["message"])));
    }
    for row in f["dot_reads"].as_array().unwrap() {
        checks.push(format!("dot-read-case {} {} {}", s(&row["input"]), if row["ok"].as_bool().unwrap() { "True" } else { "False" }, s(&row["output"])));
    }
    for row in f["dot_writes"].as_array().unwrap() {
        checks.push(format!("hex (dot-encode (input {})) == {}", s(&row["input"]), s(&row["output"])));
    }
    checks.push("header-api ()".into());
    run(&checks);
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn textproto_declarations_have_canonical_documentation() {
    common::assert_documented("Text.TextProto", include_str!("../prelude/Text/TextProto.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference text-protocol vector consumers.
module Main where
  import Text.TextProto
  import Text.Hex
  -- | Parse recorded bytes.
  input :: String -> Bytes
  input t = case hex-decode t of Ok b -> b; Err e -> bytes-new 0
  -- | Hex of bytes.
  hex :: Bytes -> String
  hex b = hex-encode b
  -- | Canonical key.
  canonical-case :: String -> String -> Bool
  canonical-case key expected = hex (canonical-header-key (input key)) == expected
  -- | Header block: success with exactly the expected fields (any order) and remaining bytes, or failure.
  header-case :: String -> Bool -> List (String, List String) -> String -> Bool
  header-case block ok fields rest = case read-mime-header (input block) 0 of
    Err e -> not ok
    Ok (header, next) -> ok && list-length (header-keys header) == list-length fields && all-fields header fields
      && hex (suffix (input block) next) == rest
  -- | Every expected field has exactly its values.
  all-fields :: MimeHeader -> List (String, List String) -> Bool
  all-fields header fields = case fields of
    Nil -> True
    Cons field rest -> case field of (key, values) -> same-values (header-values header (input key)) values && all-fields header rest
  -- | Value lists equal in order.
  same-values :: List Bytes -> List String -> Bool
  same-values actual expected = case (actual, expected) of
    (Nil, Nil) -> True
    (Cons a more, Cons e es) -> hex a == e && same-values more es
    other -> False
  -- | Bytes from an offset.
  suffix :: Bytes -> Nat -> Bytes
  suffix b i = if i <= bytes-len b then let out = bytes-new (bytes-len b - i) in let c = bytes-copy-range out b 0 i (bytes-len b - i) in out else bytes-new 0
  -- | Status line or response.
  code-case :: String -> Nat -> Bool -> Bool -> Nat -> String -> Bool
  code-case text expected multi ok code message =
    case (if multi then read-response (input text) 0 expected else read-code-line (input text) 0 expected) of
      Ok (c, m, next) -> ok && c == code && hex m == message
      Err e -> not ok
  -- | Dot decoding.
  dot-read-case :: String -> Bool -> String -> Bool
  dot-read-case text ok output = case read-dot-bytes (input text) 0 of
    Ok (body, next) -> ok && hex body == output
    Err e -> not ok
  -- | Header construction, lookup, replacement, deletion and rendering.
  header-api :: Unit -> Bool
  header-api unit =
    let h0 = header-add (header-add (header-add mime-header (string-to-bytes "content-type") (string-to-bytes "a")) (string-to-bytes "X-Id") (string-to-bytes "1")) (string-to-bytes "CONTENT-TYPE") (string-to-bytes "b") in
    let h1 = header-set h0 (string-to-bytes "x-id") (string-to-bytes "2") in
    let h2 = header-delete h1 (string-to-bytes "Content-Type") in
    list-length (header-values h0 (string-to-bytes "Content-Type")) == 2
      && (case header-get h1 (string-to-bytes "X-ID") of Some v -> text v == "2"; None -> False)
      && list-length (header-keys h2) == 1
      && text (render-header h0) == "Content-Type: a\r\nContent-Type: b\r\nX-Id: 1\r\n\r\n"
      && text (trim-bytes (string-to-bytes " \t x y \r\n")) == "x y"
  -- | Bytes as text, or empty text for invalid UTF-8.
  text :: Bytes -> String
  text b = case string-from-bytes b of Some t -> t; None -> ""
  -- | Index of the first failing check, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
