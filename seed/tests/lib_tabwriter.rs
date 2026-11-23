//! Elastic tabstop formatting against independent reference vectors from
//! `tooling/tabwriter_vectors.go`: eleven configurations (padding bytes, tab padding,
//! right alignment, debug bars, tab indentation, discarded empty columns, HTML filtering
//! and escape stripping) over sixteen inputs, including writes split mid-cell, form feeds,
//! escapes, multibyte text and vertical tabs. Each case checks the output released
//! before the final flush and the complete output.
mod common;

use serde_json::Value;

/// Run every fixture row through the checked consumer, in bounded batches.
#[test]
fn tabwriter_matches_reference_output_and_flush_points() {
    let rows: Value = serde_json::from_str(include_str!("fixtures/tabwriter-vectors.json")).unwrap();
    let checks = rows
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            let writes = row["writes"].as_array().unwrap().iter().map(|w| format!("{:?}", w.as_str().unwrap())).collect::<Vec<_>>().join(", ");
            format!(
                "tab-case {} {} {} {} {} [{writes}] {:?} {:?}",
                row["minimum"], row["tab_width"], row["padding"], row["byte"], row["flags"],
                row["early"].as_str().unwrap(), row["output"].as_str().unwrap()
            )
        })
        .collect::<Vec<_>>();
    for batch in checks.chunks(44) {
        let dir = common::project("tabwriter", &[
            ("Text.TabWriter.cinch", include_str!("../prelude/Text/TabWriter.cinch")),
            ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
            ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
            ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
            ("Main.cinch", &format!("{HELPERS}\n  -- | Every row in this batch.\n  main :: Int\n  main = first [{}] 0\n", batch.join(",\n    "))),
        ]);
        assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing row index in the batch; 200 means every row passed");
        common::cleanup(&dir);
    }
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn tabwriter_declarations_have_canonical_documentation() {
    common::assert_documented("Text.TabWriter", include_str!("../prelude/Text/TabWriter.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference tabwriter vector consumers.
module Main where
  import Text.TabWriter
  import Text.Hex
  import Data.Bytes.Build
  -- | Parse independently recorded bytes.
  input :: String -> Bytes
  input text = case hex-decode text of Ok b -> b; Err e -> bytes-new 0
  -- | Apply writes in order, collecting released output.
  apply :: TabWriter -> List String -> List Bytes -> (List Bytes, TabWriter)
  apply writer writes acc = case writes of
    Nil -> (acc, writer)
    Cons chunk rest -> case tab-write writer (input chunk) of (out, next) -> apply next rest (Cons out acc)
  -- | Join written chunks into one input.
  joined :: List String -> List Bytes -> Bytes
  joined writes acc = case writes of
    Nil -> bytes-concat (reverse-bytes acc Nil)
    Cons chunk rest -> joined rest (Cons (input chunk) acc)
  -- | Reverse a chunk list.
  reverse-bytes :: List Bytes -> List Bytes -> List Bytes
  reverse-bytes items acc = case items of Nil -> acc; Cons x rest -> reverse-bytes rest (Cons x acc)
  -- | Bytes rendered as hex for comparison with the fixture.
  hex :: Bytes -> String
  hex b = hex-encode b
  -- | One fixture row: released output before flush, complete output, and the one-shot form.
  tab-case :: Nat -> Nat -> Nat -> InRange 0 255 Nat -> Nat -> List String -> String -> String -> Bool
  tab-case minimum width padding byte flags writes early output =
    let config = tab-config minimum width padding byte flags in
    case apply (tab-writer config) writes Nil of
      (released, writer) -> case tab-flush writer of
        (last, done) ->
          let before = bytes-concat (reverse-bytes released Nil) in
          hex before == early && hex (bytes-concat [before, last]) == output
            && hex (tab-format config (joined writes Nil)) == output
  -- | Index of the first failing row, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
