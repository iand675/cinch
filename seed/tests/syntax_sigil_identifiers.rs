//! `!` and `?` continue an identifier after its first character
//! (surface grammar local amendments section 8): `print!`, `empty?` and
//! `read!?` are ordinary names, a sigil never starts one (`!=`, `?`, `(!)`
//! stay operator runs), and a `!` tight before a token tree opener is still
//! the macro invocation's bang. The reading is shared with the self-hosted
//! frontend, so these tests hold under either `CINCH_TEST_COMPILER`
//! selection.
mod common;

use common::source_frontend::{check_sources, check_source_interfaces, run_source};

fn run(source: &str) -> i32 {
    run_source(source).unwrap()
}

#[test]
fn sigil_names_define_apply_and_run() {
    // `empty?` and `print!` are definitions like any other; a run of
    // sigils is one name (`read!?`). `print! (read!? 42)` spaces the bang
    // that would otherwise open a macro token tree.
    let value = run(r#"module Main where
  empty? :: List Int -> Bool
  empty? xs = case xs of Nil -> True; Cons _ _ -> False
  print! :: Int -> Int
  print! value = value
  read!? :: Int -> Int
  read!? value = value
  main :: Int
  main = if empty? Nil then print! (read!? 42) else 0
"#);
    assert_eq!(value, 42);
}

#[test]
fn sigil_names_are_not_keywords() {
    // The keyword lookup compares whole spellings, so a sigil leaves an
    // ordinary identifier while the bare word stays reserved.
    let value = run(r#"module Main where
  if? :: Int -> Bool
  if? n = n == 0
  of? :: Int -> Int
  of? n = n
  when? :: Int -> Int
  when? n = n
  main :: Int
  main = if if? 0 then of? (when? 42) else 0
"#);
    assert_eq!(value, 42);
}

#[test]
fn sigil_operators_keep_their_reading_when_spaced() {
    // A sigil in symbol position is still an operator run: `?=` is one
    // operator, definable and usable infix.
    let value = run(r#"module Main where
  infix 5 ?=
  (?=) :: Int -> Int -> Bool
  (?=) x y = x == y
  main :: Int
  main = if 1 ?= 1 then 42 else 0
"#);
    assert_eq!(value, 42);
}

#[test]
fn a_tight_equals_after_a_sigil_name_defines_that_name() {
    // The documented consequence: `probe!=41` is the name `probe!` then
    // `=`, because the sigil belongs to the identifier. An operator that
    // begins with a sigil therefore needs a space before it (`x != y`).
    let value = run(r#"module Main where
  probe! :: Int
  probe!=40
  probe? :: Int
  probe?=41
  main :: Int
  main = probe! + (probe? - 39)
"#);
    assert_eq!(value, 42);
}

#[test]
fn a_bang_before_a_token_tree_is_still_a_macro_invocation() {
    // `print!(x)` names the macro `print`, never the value `print!`.
    let refused = check_sources(
        r#"module Main where
  print! :: Int -> Int
  print! value = value
  main :: Int
  main = print!(42)
"#,
        "sigil-macro-bang.cinch",
    );
    assert!(refused.is_err(), "a bang before a token tree is a macro call");
}

#[test]
fn sigil_names_export_their_own_spelling() {
    let interfaces = check_source_interfaces(
        r#"module Main where
  empty? :: List Int -> Bool
  empty? xs = case xs of Nil -> True; Cons _ _ -> False
  main :: Int
  main = 0
"#,
        "sigil-export.cinch",
    )
    .unwrap();
    let module = interfaces.iter().find(|iface| iface.module == "Main").unwrap();
    let export = module
        .exports
        .values
        .iter()
        .find(|value| value.name == "empty?")
        .expect("the sigil name is exported under its source spelling");
    assert_eq!(export.symbol, "Main.empty?");
}
