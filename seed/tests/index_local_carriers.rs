//! Ordinary index annotations use checked lexical values and their carriers.
mod common;

use common::source_frontend::{check_sources, run_source};

const SOURCE: &str = r#"module Main where
  data Tagged (n :: Nat) where
    Tag :: Int -> Tagged n
  measure :: Int -> Nat
  measure value = let unit = ignore value in 40
  macro wrap :: Syntax -> Code Int
  macro wrap arg = Syntax.capture arg
  use :: (payload :: Int) -> Tagged (measure payload) -> Int
  use payload tag = case (tag :: Tagged (measure payload)) of { Tag value -> value }
  main :: Int
  main = use 7 (Tag 42)
"#;

#[test]
fn ordinary_index_annotations_resolve_locals_and_check_their_carriers() {
    let program = SOURCE;
    assert_eq!(
        run_source(&program).unwrap(),
        42
    );
    let wrong = program
        .replace("(payload :: Int)", "(payload :: Bytes)")
        .replace(
            "main = use 7 (Tag 42)",
            "main = let buffer = bytes-new 1 in use buffer (Tag 42)",
        );
    let error = check_sources(&wrong, "wrong-ordinary-index-carrier.cinch")
        .err()
        .expect("ordinary index annotations must retain their argument's actual carrier")
        .to_string();
    assert!(error.contains("Int") && error.contains("Bytes"), "{error}");
}

#[test]
fn implicit_nominal_indices_pass_the_selected_constructor_to_the_worker() {
    let source = r#"module Main where
  data Mode where
    Cold :: Mode
    Warm :: Mode
  data Tagged (mode :: Mode) where
    Token :: Tagged mode
  inspect :: forall (mode :: Mode). Tagged mode -> Int
  inspect token = case mode of
    Cold -> 3
    Warm -> 39
  main :: Int
  main = inspect (Token :: Tagged Cold) + inspect (Token :: Tagged Warm)
"#;
    assert_eq!(run_source(source).unwrap(), 42);
}

#[test]
fn implicit_string_length_indices_use_the_unicode_length_provider() {
    let source = r#"module Main where
  data Mark (n :: Nat) where
    Mark :: Mark n
  observe :: forall (n :: Nat). Mark n -> Int
  observe tag = if n == 2 then 42 else 1
  measure :: String -> Int
  measure value = observe (Mark :: Mark (string-length value))
  main :: Int
  main = measure "éa"
"#;
    assert_eq!(run_source(source).unwrap(), 42);
}

#[test]
fn builtin_index_observations_pass_the_measured_value_to_the_worker() {
    let source = r#"module Main where
  data Mark (n :: Nat) where
    Mark :: Mark n
  inspect :: forall (n :: Nat). Mark n -> Int
  inspect tag = if n == 4294967295 then 40 else if n == 3 then 2 else 0
  word :: U32 -> Int
  word value = inspect (Mark :: Mark (toNat value))
  buffer :: Bytes -> Int
  buffer value = inspect (Mark :: Mark (length value))
  main :: Int
  main = word 4294967295 + buffer (bytes-new 3)
"#;
    assert_eq!(run_source(source).unwrap(), 42);
    for (from, to) in [("toNat value", "length value"), ("length value", "toNat value")] {
        let error = check_sources(&source.replace(from, to), "wrong-builtin-index-carrier.cinch")
            .expect_err("a builtin observation must check its operand carrier")
            .to_string();
        assert!(
            error.contains("Bytes") || error.contains("machine integer") || error.contains("U32"),
            "{error}"
        );
    }
}

#[test]
fn builtin_length_and_the_byte_length_primitive_stay_distinct_in_types() {
    let source = r#"module Main where
  data Mark (n :: Nat) where
    Mark :: Mark n
  exchange :: (b :: Bytes) -> Mark (length b) -> Mark (bytes-len b)
  exchange b value = value
  main :: Int
  main = 0
"#;
    check_sources(source, "opaque-length-index.cinch")
        .expect_err("two executable observations cannot silently convert between indexed types");
}
