//! End-to-end tests for string literal patterns (`case s of "default" -> ...`)
//! and list `[]` / `:` cons expressions and patterns.

mod common;
use cinch::driver::Mode;

fn run_main(tag: &str, src: &str) -> i32 {
    let dir = common::project(tag, &[("Main.cinch", src)]);
    let code = common::drive(Mode::Run, &dir, "Main.cinch", None)
        .unwrap_or_else(|e| panic!("{tag}: compilation/run failed: {e}"));
    common::cleanup(&dir);
    code
}

fn check_err(tag: &str, src: &str) -> String {
    let dir = common::project(tag, &[("Main.cinch", src)]);
    let err = common::check(&dir, "Main.cinch").expect_err("expected check error");
    common::cleanup(&dir);
    err.to_string()
}

#[test]
fn string_literal_patterns_basic_nested_guarded_and_wide() {
    let src = r#"module Main where
  data Tag where
    IgnoredDefault :: Tag
    Custom :: String -> Tag

  classify :: String -> Option Tag
  classify s = case s of
    "default" -> Some IgnoredDefault
    "" -> None
    "hello\nworld" -> Some (Custom "escaped")
    other -> Some (Custom other)

  nested :: (Option String, Int) -> Int
  nested pair = case pair of
    (Some "default", 1) -> 10
    (Some "default", n) | n > 5 -> n
    (Some "alpha", _) -> 20
    (None, _) -> 30
    _ -> 40

  -- 20 distinct literals triggers binary-search string dispatch (>= 16 arms)
  wide :: String -> Int
  wide s = case s of
    "k01" -> 1
    "k02" -> 2
    "k03" -> 3
    "k04" -> 4
    "k05" -> 5
    "k06" -> 6
    "k07" -> 7
    "k08" -> 8
    "k09" -> 9
    "k10" -> 10
    "k11" -> 11
    "k12" -> 12
    "k13" -> 13
    "k14" -> 14
    "k15" -> 15
    "k16" -> 16
    "k17" -> 17
    "k18" -> 18
    "k19" -> 19
    "k20" -> 20
    _ -> 99

  main :: Int
  main =
    let infer-it = \s -> case s of "yes" -> 1; _ -> 0 in
    let c1 = case classify "default" of Some IgnoredDefault -> 0; _ -> 1 in
    let c2 = case classify "" of None -> 0; _ -> 2 in
    let c3 = case classify "hello\nworld" of Some (Custom "escaped") -> 0; _ -> 4 in
    let c4 = case classify "other" of Some (Custom "other") -> 0; _ -> 8 in
    let c5 = if nested (Some "default", 1) == 10 && nested (Some "default", 7) == 7 && nested (Some "default", 2) == 40 && nested (Some "alpha", 0) == 20 && nested (None, 0) == 30 then 0 else 16 in
    let c6 = if wide "k01" == 1 && wide "k10" == 10 && wide "k20" == 20 && wide "zzz" == 99 then 0 else 32 in
    let c7 = if infer-it "yes" == 1 && infer-it "no" == 0 then 0 else 64 in
    c1 + c2 + c3 + c4 + c5 + c6 + c7
"#;
    assert_eq!(run_main("string_patterns", src), 0);
}

#[test]
fn string_pattern_requires_fallback() {
    let src = r#"module Main where
  f :: String -> Int
  f s = case s of
    "only" -> 1
"#;
    let err = check_err("string_non_exhaustive", src);
    assert!(err.contains("non-exhaustive"), "expected non-exhaustive error, got: {err}");
}

#[test]
fn list_cons_and_nil_expressions_and_patterns() {
    let src = r#"module Main where
  import Algebra.Semigroup

  sum-all :: List Int -> Int
  sum-all xs = case xs of
    [] -> 0
    x : rest -> x + sum-all rest

  first-two :: List Int -> Int
  first-two xs = case xs of
    a : b : _ -> a * 10 + b
    [a] -> a
    [] -> 0

  match-param :: List Int -> Int
  match-param [] = 0
  match-param (x : xs) = x + match-param xs

  nested-cons :: Option (List String) -> Int
  nested-cons opt = case opt of
    Some ("head" : "second" : _) -> 1
    Some ("head" : []) -> 2
    Some [] -> 3
    _ -> 4

  main :: Int
  main =
    let xs = 1 : 2 : 3 : [] in
    let ys = (1 + 2) : (4 : []) <> [5] in
    let c1 = if sum-all xs == 6 then 0 else 1 in
    let c2 = if first-two xs == 12 && first-two [7] == 7 && first-two [] == 0 then 0 else 2 in
    let c3 = if match-param ys == 12 then 0 else 4 in
    let c4 = if nested-cons (Some ("head" : "second" : [])) == 1 && nested-cons (Some ("head" : [])) == 2 && nested-cons (Some []) == 3 && nested-cons None == 4 then 0 else 8 in
    c1 + c2 + c3 + c4
"#;
    assert_eq!(run_main("list_cons_patterns", src), 0);
}
