//! Row-local names agree only when later constructor fields depend on them.
mod common;

use common::source_frontend::{check_sources, run_source};

#[test]
fn nested_json_patterns_can_use_different_row_local_names() {
    let source = r#"module Main where
  import Data.Json
  decode :: Json -> Int
  decode doc = case doc of
    JArr (JStr tag) (JArr single JArrEnd) -> 11
    JArr (JStr tag) (JArr first (JArr second JArrEnd)) -> 22
    other -> 0
  main :: Int
  main = if decode (JArr (JStr "one") (JArr (JNum 7) JArrEnd)) == 11
    && decode (JArr (JStr "two") (JArr (JNum 8) (JArr (JNum 9) JArrEnd))) == 22
    then 0 else 1
"#;
    assert_eq!(run_source(source).expect("compile nested JSON patterns"), 0);
}

#[test]
fn independent_evidence_field_names_can_differ_between_rows() {
    let source = r#"module Main where
  readFin :: Fin 10 -> Bool -> Nat
  readFin (Fin x bound) True = x
  readFin (Fin x witness) False = x
  main :: Int
  main = case decide (3 < 10) of
    Yes bound -> if readFin (Fin 3 bound) True == 3 && readFin (Fin 3 bound) False == 3 then 0 else 1
    No _ -> 2
"#;
    assert_eq!(
        run_source(source).expect("retain dependent field substitutions"),
        0
    );
}

#[test]
fn fields_mentioned_by_later_fields_still_require_consistent_names() {
    let source = r#"module Main where
  readFin :: Fin 10 -> Bool -> Nat
  readFin (Fin x bound) True = x
  readFin (Fin y witness) False = y
  main :: Int
  main = 0
"#;
    let error = match check_sources(source, "dependent-pattern-names") {
        Ok(_) => panic!("dependent field binders were silently mixed across rows"),
        Err(error) => error.to_string(),
    };
    assert!(
        error.contains("every row must bind the same names"),
        "{error}"
    );
}


#[test]
fn empty_data_and_refuted_index_columns_admit_empty_matches() {
    let cases = [
        ("data Empty where {}\n  absurd :: Empty -> Int\n  absurd value = case value of {}", true),
        ("data Signal (n :: Nat) where\n    Mono :: Int -> Signal 1\n    Stereo :: Int -> Int -> Signal 2\n  absurd :: Signal 0 -> Int\n  absurd value = case value of {}", true),
        ("data Signal (n :: Nat) where\n    Mono :: Int -> Signal 1\n  bad :: Signal 1 -> Int\n  bad value = case value of {}", false),
        ("data Signal (n :: Nat) where\n    Mono :: Int -> Signal 1\n  bad :: forall (n :: Nat). Signal n -> Int\n  bad value = case value of {}", false),
        ("opaque type Handle :: Type\n  bad :: Handle -> Int\n  bad value = case value of {}", false),
        ("import Data.Array\n  bad :: Array Int -> Int\n  bad value = case value of {}", false),
        ("signature Marker a where {}\n  bad :: Marker Int -> Int\n  bad value = case value of {}", false),
        ("record Empty where {}\n  bad :: Empty -> Int\n  bad value = case value of {}", false),
    ];
    for (body, valid) in cases {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        let result = check_sources(&source, "empty-data-column.cinch");
        assert_eq!(result.is_ok(), valid, "{source}\n{result:?}");
        if valid {
            assert_eq!(run_source(&source).expect("compile empty data match"), 0);
        }
    }
}


#[test]
fn concrete_lambda_patterns_infer_their_input_shape_and_keep_coverage() {
    let cases = [
        ("original nullary constructor lambda", "data Leaf where\n    End :: Leaf\n  read :: Leaf -> Int -> Int\n  read leaf n = (\\End value -> value) leaf n", true),
        ("constructor payload lambda", "data Box a where\n    Box :: a -> Box a\n  read :: Box Int -> Int\n  read box = (\\(Box value) -> value) box", true),
        ("constructor lambda keeps later argument distinct", "data Box a where\n    Box :: a -> Box a\n  read :: Box Int -> Int -> Int\n  read box increment = (\\(Box value) next -> value + next) box increment", true),
        ("tuple lambda", "read :: (Int, Bool) -> Int\n  read pair = (\\(value, flag) -> if flag then value else 0) pair", true),
        ("unit lambda", "read :: Unit -> Int\n  read unit = (\\() -> 7) unit", true),
        ("complete bool matrix", "read :: Bool -> Int\n  read flag = (\\value -> case value of True -> 1; False -> 0) flag", true),
        ("list matrix", "read :: List Int -> Int\n  read items = (\\value -> case value of [] -> 0; _ -> 1) items", true),
        ("literal matrix", "read :: Int -> Int\n  read number = (\\value -> case value of 0 -> 0; _ -> 1) number", true),
        ("incomplete constructor matrix", "data Leaf where\n    End :: Leaf\n    Item :: Int -> Leaf\n  bad :: Leaf -> Int\n  bad leaf = (\\End -> 0) leaf", false),
        ("incomplete bool matrix", "bad :: Bool -> Int\n  bad flag = (\\True -> 0) flag", false),
        ("different family at application", "data A where\n    First :: A\n  data B where\n    Second :: B\n  bad :: B -> Int\n  bad value = (\\First -> 0) value", false),
        ("incompatible tuple rows", "bad :: (Int, Int) -> Int\n  bad pair = (\\value -> case value of (x, y) -> x; (x, y, z) -> z) pair", false),
    ];
    for (name, body, valid) in cases {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        let result = check_sources(&source, "inferred-pattern-shape.cinch");
        assert_eq!(result.is_ok(), valid, "{name}: {result:?}\n{source}");
    }
    let runtime = r#"module Main where
  data Leaf where
    End :: Leaf
  data Box a where
    Box :: a -> Box a
  main :: Int
  main = if (\End value -> value) End 42 == 42
    && (\(Box value) next -> value + next) (Box 20) 22 == 42
    && (\(value, flag) -> if flag then value else 0) (42, True) == 42
    then 0 else 1
"#;
    assert_eq!(run_source(runtime).expect("compile inferred lambda pattern shapes"), 0);
}
