//! DER encoding and decoding against independent reference vectors from `tooling/asn1_vectors.go`:
//! every valid value encodes to the reference bytes and those bytes decode back to the same
//! encoding; DER violations are rejected; times convert to Unix seconds.
mod common;

use serde_json::Value;

/// Compile and run checks; the exit status is the first failing check, 200 if none.
fn run(checks: &[String]) {
    let dir = common::project("asn1", &[
        ("Text.Asn1.cinch", include_str!("../prelude/Text/Asn1.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every check.\n  main :: Int\n  main = first [{}] 0\n", checks.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing check; 200 means every check passed");
    common::cleanup(&dir);
}

/// Valid encodings, invalid encodings and time conversions in one run.
#[test]
fn asn1_matches_reference_der() {
    let f: Value = serde_json::from_str(include_str!("fixtures/asn1-vectors.json")).unwrap();
    let mut checks = f["valid"].as_array().unwrap().iter()
        .map(|r| format!("valid-case {:?} ({})", r["der"].as_str().unwrap(), r["expr"].as_str().unwrap()))
        .collect::<Vec<_>>();
    checks.extend(f["invalid"].as_array().unwrap().iter().map(|d| format!("invalid-case {:?}", d.as_str().unwrap())));
    checks.extend(f["times"].as_array().unwrap().iter().map(|t| format!("time-case {:?} {}", t[0].as_str().unwrap(), t[1])));
    run(&checks);
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn asn1_declarations_have_canonical_documentation() {
    common::assert_documented("Text.Asn1", include_str!("../prelude/Text/Asn1.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference DER vector consumers.
module Main where
  import Text.Asn1
  import Text.Hex
  -- | Parse recorded bytes.
  input :: String -> Bytes
  input t = case hex-decode t of Ok b -> b; Err e -> bytes-new 0
  -- | The value encodes to the reference bytes, and the reference bytes decode to a value with the same encoding.
  valid-case :: String -> Asn1 -> Bool
  valid-case der value =
    hex-encode (asn1-encode value) == der && (case asn1-decode (input der) of Ok back -> hex-encode (asn1-encode back) == der; Err e -> False)
  -- | A DER violation is rejected.
  invalid-case :: String -> Bool
  invalid-case der = case asn1-decode (input der) of Ok v -> False; Err e -> True
  -- | A decoded time converts to the expected Unix seconds.
  time-case :: String -> Int -> Bool
  time-case der seconds = case asn1-decode (input der) of
    Ok v -> (case asn1-time-seconds v of Some s -> s == seconds; None -> False)
    Err e -> False
  -- | Index of the first failing check, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
