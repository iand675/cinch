//! Row-local names agree only when later constructor fields depend on them.
use cinch::driver::{check_sources, run_source};

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
