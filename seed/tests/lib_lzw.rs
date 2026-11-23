//! LZW compression and decompression against independent reference encodings from
//! `tooling/lzw_vectors.go`: both bit orders, literal widths 2 to 8, three corpus shapes and
//! sizes through 20000 bytes (several dictionary resets). Small encodings are compared byte for
//! byte; large ones by SHA-256 digest and length. Malformed streams report precise errors.
mod common;

use serde_json::Value;

/// Compile and run one batch of fixture rows; the exit status is the first failing row, 200 if none.
fn run_batch(rows: &[String]) {
    let dir = common::project("lzw", &[
        ("Text.Lzw.cinch", include_str!("../prelude/Text/Lzw.cinch")),
        ("Text.Sha256.cinch", include_str!("../prelude/Text/Sha256.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every row in this batch.\n  main :: Int\n  main = first [{}] 0\n", rows.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing row index in the batch; 200 means every row passed");
    common::cleanup(&dir);
}

/// Every reference encoding is reproduced exactly and decodes back to its corpus.
#[test]
fn lzw_matches_reference_encodings_in_both_bit_orders_and_widths() {
    let rows: Value = serde_json::from_str(include_str!("fixtures/lzw-vectors.json")).unwrap();
    let checks = rows
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            format!(
                "lzw-case {} {} {} {} {:?} {:?} {}",
                row["order"], row["width"], row["kind"], row["size"],
                row["encoded"].as_str().unwrap(), row["digest"].as_str().unwrap(), row["length"]
            )
        })
        .collect::<Vec<_>>();
    for batch in checks.chunks(45) {
        run_batch(batch);
    }
}

/// Invalid widths, oversized literals, undefined codes and truncated streams are rejected precisely.
#[test]
fn lzw_rejects_invalid_parameters_and_malformed_streams() {
    run_batch(&["errors ()".to_string()]);
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn lzw_declarations_have_canonical_documentation() {
    common::assert_documented("Text.Lzw", include_str!("../prelude/Text/Lzw.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference LZW vector consumers.
module Main where
  import Text.Lzw
  import Text.Hex
  import Text.Sha256
  -- | Parse independently recorded bytes.
  input :: String -> Bytes
  input text = case hex-decode text of Ok b -> b; Err e -> bytes-new 0
  -- | 2^n.
  pow :: Nat -> Nat
  pow n = if n == 0 then 1 else 2 * pow (n - 1)
  -- | Rebuild a corpus: a stepped sequence, a period-three pattern, or a 32-bit linear congruential stream.
  corpus :: Nat -> Nat -> Nat -> Bytes
  corpus kind width size = let out = bytes-new size in let filled = fill out kind width 0 12345 in out
  -- | Fill corpus bytes in order.
  fill :: Bytes -> Nat -> Nat -> Nat -> U32 -> Unit
  fill out kind width i state =
    if i < bytes-len out then
      let next = wrapping-add-u32 (wrapping-mul-u32 state 1103515245) 12345 in
      let value = value-of kind width i next in
      let written = if value < 256 then bytes-set out i value else () in
      fill out kind width (i + 1) (if kind == 2 then next else state)
    else ()
  -- | One corpus byte for the generator kind.
  value-of :: Nat -> Nat -> Nat -> U32 -> Nat
  value-of kind width i next = reduce (if kind == 0 then i * 37 + 11 else if kind == 1 then nat-rem i 3 else u8-to-nat (wrapping-u32-to-u8 (wrapping-shr-u32 next 16))) (pow width)
  -- | A value modulo a positive bound.
  reduce :: Nat -> Nat -> Nat
  reduce value bound = if bound > 0 then nat-rem value bound else 0
  -- | The bit order for a fixture order code.
  order :: Nat -> LzwOrder
  order code = if code == 0 then LzwLsb else LzwMsb
  -- | One row: compression matches the reference, and both encodings decode to the corpus.
  lzw-case :: Nat -> Nat -> Nat -> Nat -> String -> String -> Nat -> Bool
  lzw-case code width kind size encoded digest length =
    let plain = corpus kind width size in
    case lzw-compress (order code) width plain of
      Err e -> False
      Ok ours ->
        bytes-len ours == length && hex-encode (sha256-sum ours) == digest
          && (if bytes-len (input encoded) > 0 then hex-encode ours == encoded && decodes (order code) width (input encoded) plain else True)
          && decodes (order code) width ours plain
  -- | Decompression yields exactly the expected bytes.
  decodes :: LzwOrder -> Nat -> Bytes -> Bytes -> Bool
  decodes o width encoded expected = case lzw-decompress o width encoded of
    Ok out -> hex-encode out == hex-encode expected
    Err e -> False
  -- | Every error constructor for its misuse.
  errors :: Unit -> Bool
  errors unit =
    (case lzw-compress LzwLsb 1 (bytes-new 1) of Err (InvalidLiteralWidth 1) -> True; other -> False)
      && (case lzw-decompress LzwMsb 9 (bytes-new 1) of Err (InvalidLiteralWidth 9) -> True; other -> False)
      && (case lzw-compress LzwLsb 2 (bytes-from-u8s [1, 2, 3, 4]) of Err (LiteralTooLarge 3) -> True; other -> False)
      && (case lzw-decompress LzwLsb 8 (bytes-new 0) of Err UnexpectedLzwEnd -> True; other -> False)
      && (case lzw-decompress LzwLsb 8 (bytes-from-u8s [0, 1]) of Err UnexpectedLzwEnd -> True; other -> False)
      && (case lzw-decompress LzwLsb 2 (bytes-from-u8s [7]) of Err (InvalidLzwCode 7 0) -> True; other -> False)
      && lzw-error-message UnexpectedLzwEnd == "LZW data ended before the end-of-data code"
      && lzw-error-message (InvalidLzwCode 9 2) == "invalid LZW code 9 at code index 2"
  -- | Index of the first failing row, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
