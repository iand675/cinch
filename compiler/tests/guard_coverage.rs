//! Guard coverage: a guarded row covers nothing, so a match is accepted
//! only when its unguarded rows (and rows guarded by the literal `True`)
//! cover every constructor and value. A match whose guards could all fail
//! is a compile error, never a runtime fault.

use cinch::driver::{check_sources, run_source};

fn expect_ok(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_err(name: &str, src: &str, needle: &str) {
    match check_sources(src, "<test>") {
        Ok(_) => panic!("{name}: expected an error containing {needle:?}, but the program checked"),
        Err(e) => {
            let msg = e.to_string();
            assert!(msg.contains(needle), "{name}: error {msg:?} does not contain {needle:?}");
            assert!(
                msg.contains("add an unguarded final row"),
                "{name}: error {msg:?} does not suggest an unguarded final row"
            );
        }
    }
}

const SHAPE: &str = r#"
  data Shape where
    Circle :: Int -> Shape
    Square :: Int -> Shape
"#;

fn program(defs: &str, main: &str) -> String {
    format!("\nmodule Main where\n{SHAPE}\n{defs}\n  main :: Int\n  main = {main}\n")
}

#[test]
fn all_guarded_match_is_rejected() {
    expect_err(
        "all-guarded-data",
        &program(
            r#"
  f :: Shape -> Int
  f s = case s of
    Circle r | r > 0 -> 1
    Square w | w > 0 -> 2
"#,
            "f (Circle 1)",
        ),
        "constructor Circle is not covered by an unguarded row",
    );
    expect_err(
        "all-guarded-int",
        &program(
            r#"
  sign :: Int -> Int
  sign n = case n of
    x | x < 0 -> 1
    x | x >= 0 -> 2
"#,
            "sign 3",
        ),
        "a scrutinee value is not covered by an unguarded row",
    );
    expect_err(
        "all-guarded-literal",
        &program(
            r#"
  g :: Int -> Int
  g n = case n of
    0 | n == 0 -> 1
    _ | n > 5 -> 2
"#,
            "g 0",
        ),
        "a value other than 0 is not covered by an unguarded row",
    );
    expect_err(
        "all-guarded-bool",
        &program(
            r#"
  b :: Bool -> Int -> Int
  b p n = case p of
    True -> 1
    False | n > 0 -> 2
"#,
            "b True 0",
        ),
        "False is not covered by an unguarded row",
    );
}

#[test]
fn unguarded_final_row_compiles_and_runs_every_row() {
    let defs = r#"
  f :: Shape -> Int
  f s = case s of
    Circle r | r > 0 -> 1
    Square w | w > 0 -> 2
    _ -> 3

  sign :: Int -> Int
  sign n = case n of
    x | x < 0 -> 1
    _ -> 2
"#;
    let main = "if f (Circle 1) == 1 && f (Square 2) == 2 && f (Circle 0) == 3 && f (Square (-1)) == 3 && sign (-4) == 1 && sign 0 == 2 && sign 9 == 2 then 0 else 1";
    expect_ok("unguarded-final", &program(defs, main), 0);
    // Each row is reached: the exit code reports which one ran.
    expect_ok("row-1", &program(defs, "f (Circle 5)"), 1);
    expect_ok("row-2", &program(defs, "f (Square 5)"), 2);
    expect_ok("row-3", &program(defs, "f (Square 0)"), 3);
}

#[test]
fn mixed_rows_are_rejected_exactly_when_unguarded_rows_miss_a_constructor() {
    // Circle has an unguarded row; Square only a guarded one.
    expect_err(
        "mixed-missing-square",
        &program(
            r#"
  f :: Shape -> Int
  f s = case s of
    Circle r | r > 0 -> 1
    Circle r -> 2
    Square w | w > 9 -> 3
"#,
            "f (Circle 1)",
        ),
        "constructor Square is not covered by an unguarded row",
    );
    // Nested: the unguarded `Some` row misses `None` under `Square`.
    expect_err(
        "mixed-nested",
        r#"
module Main where
  data Shape where
    Circle :: Int -> Shape
    Square :: Int -> Shape

  f :: Option Shape -> Int
  f o = case o of
    Some (Circle r) -> 1
    Some (Square w) -> 2
    None | True && False -> 3

  main :: Int
  main = f None
"#,
        "constructor None is not covered by an unguarded row",
    );
    // The same guarded rows with every constructor covered unguarded.
    let defs = r#"
  f :: Shape -> Int
  f s = case s of
    Circle r | r > 0 -> 1
    Circle r -> 2
    Square w | w > 9 -> 3
    Square w -> 4
"#;
    expect_ok("mixed-covered-1", &program(defs, "f (Circle 1)"), 1);
    expect_ok("mixed-covered-2", &program(defs, "f (Circle 0)"), 2);
    expect_ok("mixed-covered-3", &program(defs, "f (Square 10)"), 3);
    expect_ok("mixed-covered-4", &program(defs, "f (Square 1)"), 4);
}

#[test]
fn true_guard_counts_as_unguarded() {
    let defs = r#"
  f :: Shape -> Int
  f s = case s of
    Circle r | r > 0 -> 1
    Circle r | True -> 2
    Square w | True -> 3

  sign :: Int -> Int
  sign n = case n of
    x | x < 0 -> 4
    x | True -> 5
"#;
    expect_ok("true-1", &program(defs, "f (Circle 1)"), 1);
    expect_ok("true-2", &program(defs, "f (Circle 0)"), 2);
    expect_ok("true-3", &program(defs, "f (Square 0)"), 3);
    expect_ok("true-4", &program(defs, "sign (-1)"), 4);
    expect_ok("true-5", &program(defs, "sign 1"), 5);
    // Only the literal `True` counts; an expression that happens to be
    // true is still a guard that the checker cannot see through.
    expect_err(
        "true-expression",
        &program(
            r#"
  sign :: Int -> Int
  sign n = case n of
    x | x < 0 -> 4
    x | True || False -> 5
"#,
            "sign 1",
        ),
        "a scrutinee value is not covered by an unguarded row",
    );
}
