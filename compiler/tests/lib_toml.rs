//! TOML family tests: `Text.Toml` (the TOML 1.0 subset that maps onto
//! the `Json` value family).
//!
//! Consumer-visible semantics: each in-scope form parses to the `Json`
//! value it names (asserted structurally, never on source text); the
//! round trip `toml-parse (toml-print v) == Ok v` holds over a nested
//! document in emission order; a duplicate key and a syntax error are
//! each observed as their named `TomlError` variant; and an out-of-scope
//! form (an offset date-time) and an unrepresentable value (`JNull`) are
//! deliberately refused with their named variants.
//!
//! Every program is the real `Text/Toml.cinch` source beside a `Main`,
//! compiled and run through the same driver the CLI uses (the module is
//! not embedded in the prelude, matching the other `Text.*` families).
//! Each check is a top-level `check-*` function returning 0 or its bit;
//! `main` sums them, so a nonzero exit code is a bitmask naming the
//! failed checks (check k contributes bit 2^(k-1)).

use cinch::driver::{driver, Mode};
use std::path::PathBuf;

const TOML_SRC: &str = include_str!("../prelude/Text/Toml.cinch");
// `Text.Toml` now joins its lines through `Text.Scan`'s `string-join`,
// so the toolkit has to be beside it (its own imports are embedded).
const SCAN_SRC: &str = include_str!("../prelude/Text/Scan.cinch");

/// The `Main` header: the imports, structural `Json` equality (order
/// sensitive; NaN equals NaN for the round-trip law), the `Json`
/// builders, and the parse-expectation helper.
const HEADER: &str = r#"
module Main where
  import Text.Toml
  import Data.Json
  import Data.String
  import Data.Int
  import Data.Machine
  import Data.List

  same :: Json -> Json -> Bool
  same a b = case a of
    JNull -> case b of
      JNull -> True
      _ -> False
    JBool x -> case b of
      JBool y -> if x == y then True else False
      _ -> False
    JNum x -> case b of
      JNum y -> eq-i64 (int-cmp x y) 0
      _ -> False
    JFloat x -> case b of
      JFloat y -> if is-na-nf64 x && is-na-nf64 y then True else feq-f64 x y
      _ -> False
    JStr x -> case b of
      JStr y -> string-compare x y == 0
      _ -> False
    JArr x xs -> case b of
      JArr y ys -> if same x y then same xs ys else False
      _ -> False
    JObj k x xs -> case b of
      JObj l y ys -> if string-compare k l == 0 then (if same x y then same xs ys else False) else False
      _ -> False
    JArrEnd -> case b of
      JArrEnd -> True
      _ -> False
    JObjEnd -> case b of
      JObjEnd -> True
      _ -> False

  arr-of :: List Json -> Json
  arr-of xs = case xs of
    Nil -> JArrEnd
    Cons y ys -> JArr y (arr-of ys)

  obj-of :: List (String, Json) -> Json
  obj-of xs = case xs of
    Nil -> JObjEnd
    Cons p ps -> case p of
      (k, v) -> JObj k v (obj-of ps)

  flt :: String -> F64
  flt s = f64-from-decimal (string-to-bytes s)

  expect-parse :: String -> Json -> Int -> Int
  expect-parse src want bit = case toml-parse src of
    Err e -> bit
    Ok got -> if same got want then 0 else bit
"#;

fn run_main(name: &str, body: &str) -> i32 {
    let dir: PathBuf = std::env::temp_dir().join(format!("cinch_tomltest_{}_{}", std::process::id(), name));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    let src = format!("{HEADER}{body}");
    std::fs::write(dir.join("Toml.cinch"), TOML_SRC).expect("write Text.Toml");
    std::fs::write(dir.join("Text.Scan.cinch"), SCAN_SRC).expect("write Text.Scan");
    std::fs::write(dir.join("main.cinch"), src).expect("write Main");
    let entry = dir.join("main.cinch");
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver(Mode::Run, &entry, None))
        .expect("spawn harness thread")
        .join()
        .expect("harness thread")
        .unwrap_or_else(|e| panic!("{name}: compilation failed: {e}"))
}

fn expect_ok(name: &str, body: &str) {
    let code = run_main(name, body);
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

#[test]
fn parse_scalar_forms() {
    expect_ok(
        "scalars",
        r#"
  check1 :: Int
  check1 = expect-parse "a = \"x\"\n" (obj-of [("a", JStr "x")]) 1
  check2 :: Int
  check2 = expect-parse "a = \"x\\ny\"\n" (obj-of [("a", JStr "x\ny")]) 2
  check3 :: Int
  check3 = expect-parse "a = 'x\\ny'\n" (obj-of [("a", JStr "x\\ny")]) 4
  check4 :: Int
  check4 = expect-parse "a = \"\"\"\nhi\nthere\"\"\"\n" (obj-of [("a", JStr "hi\nthere")]) 8
  check5 :: Int
  check5 = expect-parse "a = '''\nra\\w'''\n" (obj-of [("a", JStr "ra\\w")]) 16
  check6 :: Int
  check6 = expect-parse "a = 1_000\nb = -5\n" (obj-of [("a", JNum 1000), ("b", JNum (int-neg (int-from-i64 5)))]) 32
  check7 :: Int
  check7 = expect-parse "a = 0xdead_beef\nb = 0o755\nc = 0b1010\n" (obj-of [("a", JNum (int-from-i64 3735928559)), ("b", JNum (int-from-i64 493)), ("c", JNum (int-from-i64 10))]) 64
  check8 :: Int
  check8 = expect-parse "a = 3.14\nb = -5e-3\nc = inf\nd = nan\n" (obj-of [("a", JFloat (flt "3.14")), ("b", JFloat (flt "-5e-3")), ("c", JFloat (flt "inf")), ("d", JFloat (flt "nan"))]) 128
  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8
"#,
    );
}

#[test]
fn parse_container_forms() {
    expect_ok(
        "containers",
        r#"
  check1 :: Int
  check1 = expect-parse "a = [1, 2, 3]\n" (obj-of [("a", arr-of [JNum 1, JNum 2, JNum 3])]) 1
  check2 :: Int
  check2 = expect-parse "a = [\n  1,\n  2, # two\n]\n" (obj-of [("a", arr-of [JNum 1, JNum 2])]) 2
  check3 :: Int
  check3 = expect-parse "a = [1, \"two\", true]\n" (obj-of [("a", arr-of [JNum 1, JStr "two", JBool True])]) 4
  check4 :: Int
  check4 = expect-parse "a = { x = 1, y = \"z\" }\n" (obj-of [("a", obj-of [("x", JNum 1), ("y", JStr "z")])]) 8
  check5 :: Int
  check5 = expect-parse "a.b.c = 1\n" (obj-of [("a", obj-of [("b", obj-of [("c", JNum 1)])])]) 16
  check6 :: Int
  check6 = expect-parse "[table]\na = 1\n\n[table.sub]\nc = true\n" (obj-of [("table", obj-of [("a", JNum 1), ("sub", obj-of [("c", JBool True)])])]) 32
  check7 :: Int
  check7 = expect-parse "[[servers]]\nhost = \"a\"\n\n[[servers]]\nhost = \"b\"\n" (obj-of [("servers", arr-of [obj-of [("host", JStr "a")], obj-of [("host", JStr "b")]])]) 64
  check8 :: Int
  check8 = expect-parse "\"quoted key\" = 1\nbare-key_2 = 2\n" (obj-of [("quoted key", JNum 1), ("bare-key_2", JNum 2)]) 128
  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8
"#,
    );
}

#[test]
fn parse_string_escape_forms() {
    expect_ok(
        "escapes",
        r#"
  check1 :: Int
  check1 = expect-parse "a = \"\\u0041\"\n" (obj-of [("a", JStr "A")]) 1
  check2 :: Int
  check2 = expect-parse "a = \"\\U00000041\"\n" (obj-of [("a", JStr "A")]) 2
  check3 :: Int
  check3 = expect-parse "a = \"x\\ty\"\n" (obj-of [("a", JStr "x\ty")]) 4
  check4 :: Int
  check4 = expect-parse "a = \"x\\\"y\"\n" (obj-of [("a", JStr "x\"y")]) 8
  check5 :: Int
  check5 = expect-parse "a = \"\"\"\none \\\n   two\"\"\"\n" (obj-of [("a", JStr "one two")]) 16
  main :: Int
  main = check1 + check2 + check3 + check4 + check5
"#,
    );
}

#[test]
fn round_trip_and_emission_order() {
    expect_ok(
        "roundtrip",
        r#"
  mk-doc :: Nat -> Json
  mk-doc n = obj-of [("title", JStr "toml sample"), ("owner", obj-of [("name", JStr "cinch"), ("ports", arr-of [JNum 80, JNum 443]), ("enabled", JBool True)]), ("servers", arr-of [obj-of [("host", JStr "alpha"), ("port", JNum 8001)], obj-of [("host", JStr "beta"), ("port", JNum 8002)]])]

  check1 :: Int
  check1 = case toml-print (mk-doc 0) of
    Err e -> 1
    Ok s -> case toml-parse s of
      Err e -> 1
      Ok v2 -> if same v2 (mk-doc 0) then 0 else 1

  check2 :: Int
  check2 = case toml-print (mk-doc 0) of
    Err e -> 2
    Ok s -> if string-compare s "title = \"toml sample\"\n[owner]\nname = \"cinch\"\nports = [80, 443]\nenabled = true\n[[servers]]\nhost = \"alpha\"\nport = 8001\n[[servers]]\nhost = \"beta\"\nport = 8002" == 0 then 0 else 2

  check3 :: Int
  check3 = case toml-print (obj-of [("k", JStr "a\nb\"c")]) of
    Err e -> 4
    Ok s -> case toml-parse s of
      Err e -> 4
      Ok v2 -> if same v2 (obj-of [("k", JStr "a\nb\"c")]) then 0 else 4

  main :: Int
  main = check1 + check2 + check3
"#,
    );
}

#[test]
fn named_error_variants_and_refusals() {
    expect_ok(
        "errors",
        r#"
  check1 :: Int
  check1 = case toml-parse "a = 1\na = 2\n" of
    Err e -> case e of
      TomlDuplicateKey k -> if string-compare k "a" == 0 then 0 else 1
      _ -> 1
    Ok v -> 1

  check2 :: Int
  check2 = case toml-parse "a = \n" of
    Err e -> case e of
      TomlSyntax ln msg -> 0
      _ -> 2
    Ok v -> 2

  check3 :: Int
  check3 = case toml-parse "d = 1979-05-27T07:32:00Z\n" of
    Err e -> case e of
      TomlDateTime ln -> 0
      _ -> 4
    Ok v -> 4

  check4 :: Int
  check4 = case toml-print JNull of
    Err e -> case e of
      TomlNotRepresentable m -> 0
      _ -> 8
    Ok s -> 8

  check5 :: Int
  check5 = case toml-print (JNum 1) of
    Err e -> case e of
      TomlNotRepresentable m -> 0
      _ -> 16
    Ok s -> 16

  check6 :: Int
  check6 = case toml-parse "a = { x = 1, x = 2 }\n" of
    Err e -> case e of
      TomlDuplicateKey k -> 0
      _ -> 32
    Ok v -> 32

  main :: Int
  main = check1 + check2 + check3 + check4 + check5 + check6
"#,
    );
}
