//! Text.Yaml: the YAML safe data subset (P37). Every test compiles a
//! real project through the CLI pipeline: the module under test
//! (`Text/Yaml.cinch`, an ordinary importable module beside the entry,
//! not a base-layer module, the P30/P31 pattern of D309) plus a
//! `Main.cinch` that exercises it.
//!
//! The Cinch program returns an exit code whose bits name the failed
//! checks (check k contributes bit 2^(k-1)), so a nonzero code is a
//! bitmask and each program stays inside the 8-bit exit status. The
//! checks are consumer-visible semantics: the structural value each
//! in-scope YAML form produces, the printer's round trip, and the named
//! refusal (kind plus the offending text in the message) of every form
//! the safe subset exists to refuse. Each program carries its own
//! structural comparison over `Json` (`same-json`), because the module
//! that owns the family's `json-eq`, `Text.Json`, is an ordinary module
//! beside the entry rather than part of the embedded family (D309), and
//! the projects here depend only on the embedded prelude plus the module
//! under test.
//!
//! One note on what "two independent copies" can be observed to mean:
//! `Json` is immutable and nothing can mutate a value through another
//! binding, so sharing between two alias sites has no observable
//! consequence. The checks pin the observable half (both sites hold the
//! anchored value, both are equal to the anchored node and to each
//! other, and the printer expands both sites into data rather than
//! emitting a reference); the copy itself is a construction fact, each
//! alias site rebuilding the spine through the module's `copy-json`.

use cinch::driver::{driver, Mode};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const YAML: &str = include_str!("../prelude/Text/Yaml.cinch");

fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_yamltest_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [("Yaml.cinch", YAML), ("Main.cinch", main_src)] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    match driver(Mode::Run, &dir.join("Main.cinch"), None) {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_ok(name: &str, body: &str) {
    let code = run_main(name, &program(body));
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

/// A test program: the shared preamble plus one `main` body.
fn program(body: &str) -> String {
    format!("{HEADER}{body}")
}

/// The shared preamble: the imports and the assertion helpers over the
/// parsed value (`field` is the structural lookup in a `JObj` spine,
/// `arr` builds a `JArr` spine from a list).
const HEADER: &str = r#"module Main where
  import Text.Yaml
  import Data.Int
  import Data.Json
  import Data.List
  import Data.Machine
  import Data.Option
  import Data.Result
  import Data.String

  -- The structural oracle. Local on purpose: `Text.Json` (and its
  -- `json-eq`) is an ordinary module beside the entry, not part of the
  -- embedded family (D309), so importing it would drag the whole P30
  -- toolkit into every project here. The comparison is the same
  -- constructor-by-constructor shape `Text.Json.json-eq` uses.
  same-json :: Json -> Json -> Bool
  same-json a b = case a of
    JNull -> case b of
      JNull -> True
      _ -> False
    JBool x -> case b of
      JBool y -> x == y
      _ -> False
    JNum x -> case b of
      JNum y -> eq-i64 (int-cmp x y) 0
      _ -> False
    JFloat x -> case b of
      JFloat y -> feq-f64 x y
      _ -> False
    JStr x -> case b of
      JStr y -> eq-i64 (string-compare x y) 0
      _ -> False
    JArr x xs -> case b of
      JArr y ys -> if same-json x y then same-json xs ys else False
      _ -> False
    JObj k x xs -> case b of
      JObj l y ys -> if eq-i64 (string-compare k l) 0 then (if same-json x y then same-json xs ys else False) else False
      _ -> False
    JArrEnd -> case b of
      JArrEnd -> True
      _ -> False
    JObjEnd -> case b of
      JObjEnd -> True
      _ -> False

  field :: String -> Json -> Option Json
  field k v = case v of
    JObj k2 x rest -> if eq-i64 (string-compare k k2) 0 then Some x else field k rest
    _ -> None

  arr :: List Json -> Json
  arr xs = case xs of
    Nil -> JArrEnd
    Cons y ys -> JArr y (arr ys)

  has-field :: String -> Json -> Json -> Bool
  has-field k expected v = case field k v of
    Some got -> same-json got expected
    None -> False

  field-is :: String -> Json -> Result YamlError Json -> Bool
  field-is k expected r = case r of
    Err e -> False
    Ok v -> has-field k expected v

  -- A float field, checked through the runtime's shortest round-trip
  -- decimal rendering (the bootstrap has no float literal in the surface
  -- grammar, and the rendering pins both the type and the value).
  field-float :: String -> String -> Result YamlError Json -> Bool
  field-float k expected r = case r of
    Err e -> False
    Ok v -> case field k v of
      Some got -> case got of
        JFloat f -> case string-from-bytes (f64-to-decimal f) of
          Some text -> eq-i64 (string-compare text expected) 0
          None -> False
        _ -> False
      None -> False

  fields-equal :: String -> String -> Result YamlError Json -> Bool
  fields-equal k1 k2 r = case r of
    Err e -> False
    Ok v -> case field k1 v of
      Some a -> case field k2 v of
        Some b -> same-json a b
        None -> False
      None -> False

  round-trips :: Result YamlError Json -> Bool
  round-trips r = case r of
    Err e -> False
    Ok v -> case yaml-parse (yaml-print v) of
      Err e -> False
      Ok v2 -> same-json v v2

  print-stable :: Result YamlError Json -> Bool
  print-stable r = case r of
    Err e -> False
    Ok v -> case yaml-parse (yaml-print v) of
      Err e -> False
      Ok v2 -> eq-i64 (string-compare (yaml-print v2) (yaml-print v)) 0

  refused :: YamlErrorKind -> Result YamlError Json -> Bool
  refused k r = case r of
    Ok v -> False
    Err e -> yaml-error-kind e == k

  refused-as :: YamlErrorKind -> String -> Result YamlError Json -> Bool
  refused-as k m r = case r of
    Ok v -> False
    Err e -> if yaml-error-kind e == k then eq-i64 (string-compare (yaml-error-message e) m) 0 else False
"#;

// ----------------------------------------------------------------
// The module itself
// ----------------------------------------------------------------

/// The integration step keys the embedded family by the file's dotted
/// path (`Text/Yaml.cinch` -> `Text.Yaml`), so the module name and the
/// path must agree, and the source must parse as exactly one module with
/// the prelude's base fixity table (D42's one-module-per-file rule and
/// the D109 path-derived family key). Parsing only: this does not need
/// the embedded family to typecheck.
#[test]
fn module_parses_as_text_yaml() {
    let prelude = cinch::driver::parse_prelude(&[]).expect("the embedded prelude parses");
    let parsed = cinch::driver::parse_source(YAML, "Text/Yaml.cinch", &prelude.base, &HashMap::new())
        .expect("Text/Yaml.cinch parses");
    assert_eq!(parsed.modules.len(), 1, "one module per file");
    assert_eq!(parsed.modules[0].name.text, "Text.Yaml");
}

// ----------------------------------------------------------------
// The in-scope forms
// ----------------------------------------------------------------

#[test]
fn block_mapping_sequence_and_scalar_typing() {
    expect_ok(
        "block-forms",
        r#"
  main :: Int
  main =
    let src = "name: cinch\ncount: 3\nratio: 0.5\nactive: true\nnothing: null\nitems:\n  - 1\n  - two\n  - true\n" in
    let r = yaml-parse src in
    let c1 = if field-is "name" (JStr "cinch") r then 0 else 1 in
    let c2 = if field-is "count" (JNum 3) r then 0 else 2 in
    let c3 = if field-float "ratio" "5e-1" r then 0 else 4 in
    let c4 = if field-is "active" (JBool True) r then 0 else 8 in
    let c5 = if field-is "nothing" JNull r then 0 else 16 in
    let c6 = if field-is "items" (arr (Cons (JNum 1) (Cons (JStr "two") (Cons (JBool True) Nil)))) r then 0 else 32 in
    c1 + c2 + c3 + c4 + c5 + c6
"#,
    );
}

#[test]
fn flow_collections() {
    expect_ok(
        "flow-forms",
        r#"
  main :: Int
  main =
    let src = "{a: 1, b: [2, 3], c: {d: true}, e: [], f: {}}\n" in
    let r = yaml-parse src in
    let c1 = if field-is "a" (JNum 1) r then 0 else 1 in
    let c2 = if field-is "b" (arr (Cons (JNum 2) (Cons (JNum 3) Nil))) r then 0 else 2 in
    let c3 = if field-is "c" (JObj "d" (JBool True) JObjEnd) r then 0 else 4 in
    let c4 = if field-is "e" JArrEnd r then 0 else 8 in
    let c5 = if field-is "f" JObjEnd r then 0 else 16 in
    let src2 = "outer:\n  inner: [1, {k: v}]\n" in
    let c6 = if field-is "outer" (JObj "inner" (arr (Cons (JNum 1) (Cons (JObj "k" (JStr "v") JObjEnd) Nil))) JObjEnd) (yaml-parse src2) then 0 else 32 in
    -- A plain flow key ends at its first ':', the 1.1-style form.
    let c7 = if field-is "a" (JNum 1) (yaml-parse "{a:1}\n") then 0 else 64 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7
"#,
    );
}

#[test]
fn quoted_scalars_comments_and_document_markers() {
    expect_ok(
        "quoted-and-markers",
        r#"
  main :: Int
  main =
    let src = "---\n# a leading comment\nsingle: 'it''s here'\ndouble: \"line\\nbreak\"\nplain: hello world  # trailing comment\nempty: ''\n...\n" in
    let r = yaml-parse src in
    let c1 = if field-is "single" (JStr "it's here") r then 0 else 1 in
    let c2 = if field-is "double" (JStr "line\nbreak") r then 0 else 2 in
    let c3 = if field-is "plain" (JStr "hello world") r then 0 else 4 in
    let c4 = if field-is "empty" (JStr "") r then 0 else 8 in
    c1 + c2 + c3 + c4
"#,
    );
}

#[test]
fn block_scalars_with_chomping() {
    expect_ok(
        "block-scalars",
        r#"
  main :: Int
  main =
    let src = "literal: |\n  line one\n  line two\nfolded: >\n  fold\n  ed\nstrip: |-\n  no newline\nkeep: |+\n  kept\n\ntail: 1\n" in
    let r = yaml-parse src in
    let c1 = if field-is "literal" (JStr "line one\nline two\n") r then 0 else 1 in
    let c2 = if field-is "folded" (JStr "fold ed\n") r then 0 else 2 in
    let c3 = if field-is "strip" (JStr "no newline") r then 0 else 4 in
    let c4 = if field-is "keep" (JStr "kept\n\n") r then 0 else 8 in
    let c5 = if field-is "tail" (JNum 1) r then 0 else 16 in
    c1 + c2 + c3 + c4 + c5
"#,
    );
}

/// The alias law: every alias site holds a copy of the anchored node,
/// two sites in one document each hold the value, and the printer
/// expands them (no reference survives into the printed form).
#[test]
fn anchors_and_aliases_resolve_to_copies() {
    expect_ok(
        "anchors",
        r#"
  main :: Int
  main =
    let src = "base: &b\n  x: 1\n  y: 2\nfirst: *b\nsecond: *b\npair: [*b, *b]\n" in
    let r = yaml-parse src in
    let bj = JObj "x" (JNum 1) (JObj "y" (JNum 2) JObjEnd) in
    let c1 = if field-is "first" bj r then 0 else 1 in
    let c2 = if field-is "second" bj r then 0 else 2 in
    let c3 = if fields-equal "first" "second" r then 0 else 4 in
    let c4 = if fields-equal "base" "first" r then 0 else 8 in
    let c5 = if field-is "pair" (arr (Cons bj (Cons bj Nil))) r then 0 else 16 in
    let c6 = if round-trips r then 0 else 32 in
    c1 + c2 + c3 + c4 + c5 + c6
"#,
    );
}

// ----------------------------------------------------------------
// Printing
// ----------------------------------------------------------------

#[test]
fn printer_round_trips_and_quotes_to_keep_types() {
    expect_ok(
        "printing",
        r#"
  main :: Int
  main =
    let src = "a: 1\nb:\n  - x\n  - y\nc:\n  d: true\n  e: null\nf: []\ng: {}\nh: 'true'\ni: \"1\"\nj: 'a: b'\nk: ' lead'\nl: \"tab\\there\"\n" in
    let r = yaml-parse src in
    let c1 = if round-trips r then 0 else 1 in
    let c2 = if print-stable r then 0 else 2 in
    let c3 = if field-is "h" (JStr "true") r then 0 else 4 in
    let c4 = if field-is "i" (JStr "1") r then 0 else 8 in
    let c5 = if field-is "j" (JStr "a: b") r then 0 else 16 in
    let c6 = if field-is "k" (JStr " lead") r then 0 else 32 in
    let c7 = if field-is "l" (JStr "tab\there") r then 0 else 64 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7
"#,
    );
}

// ----------------------------------------------------------------
// The named refusals
// ----------------------------------------------------------------

/// Tags are refused before the node behind them is read, and the
/// object-constructing family (the deserialization CVE this port exists
/// to close) has its own kind.
#[test]
fn tags_and_object_tags_are_refused() {
    expect_ok(
        "tags",
        r#"
  main :: Int
  main =
    let c1 = if refused-as YamlUnsupportedTag "tag !!str" (yaml-parse "a: !!str 1\n") then 0 else 1 in
    let c2 = if refused-as YamlUnsupportedTag "tag !custom" (yaml-parse "a: !custom x\n") then 0 else 2 in
    let c3 = if refused-as YamlObjectTag "object-constructing tag !ruby/object" (yaml-parse "!ruby/object {}\n") then 0 else 4 in
    let c4 = if refused YamlObjectTag (yaml-parse "a: !ruby/object:Thing {}\n") then 0 else 8 in
    let c5 = if refused YamlObjectTag (yaml-parse "a: !!python/object/apply:x 1\n") then 0 else 16 in
    c1 + c2 + c3 + c4 + c5
"#,
    );
}

#[test]
fn structural_refusals_are_named() {
    expect_ok(
        "structural-refusals",
        r#"
  main :: Int
  main =
    let c1 = if refused-as YamlMergeKey "merge key << is not implemented" (yaml-parse "<<: 1\n") then 0 else 1 in
    let c2 = if refused YamlMultiDocument (yaml-parse "a: 1\n---\nb: 2\n") then 0 else 2 in
    let c3 = if refused-as YamlUndefinedAlias "no anchor named missing" (yaml-parse "a: *missing\n") then 0 else 4 in
    let c4 = if refused-as YamlAliasCycle "the alias reaches its own anchor a" (yaml-parse "a: &a [*a]\n") then 0 else 8 in
    let c5 = if refused-as YamlDirective "directive %YAML" (yaml-parse "%YAML 1.2\na: 1\n") then 0 else 16 in
    let c6 = if refused YamlComplexKey (yaml-parse "? complex: 1\n") then 0 else 32 in
    let c7 = if refused-as YamlDuplicateKey "duplicate mapping key a" (yaml-parse "a: 1\na: 2\n") then 0 else 64 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7
"#,
    );
}

#[test]
fn scalar_and_shape_refusals_are_named() {
    expect_ok(
        "scalar-refusals",
        r#"
  main :: Int
  main =
    let c1 = if refused-as YamlUnsupportedScalar "scalar .inf" (yaml-parse "a: .inf\n") then 0 else 1 in
    let c2 = if refused YamlSyntaxError (yaml-parse "a: [1, 2\n") then 0 else 2 in
    let c3 = if refused YamlIndentationError (yaml-parse "a: 1\n  b: 2\n") then 0 else 4 in
    let c4 = if refused-as YamlIndentationError "tab in indentation" (yaml-parse "a:\n\t- 1\n") then 0 else 8 in
    -- The two block-placement boundaries: a collection cannot begin on the
    -- document marker's line, nor on its mapping key's line.
    let c5 = if refused YamlSyntaxError (yaml-parse "--- a: 1\n") then 0 else 16 in
    let c6 = if refused YamlSyntaxError (yaml-parse "a: - 1\n") then 0 else 32 in
    c1 + c2 + c3 + c4 + c5 + c6
"#,
    );
}
