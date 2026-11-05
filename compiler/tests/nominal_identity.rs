use cinch::driver::{check_sources, run_source_with, StagingCx};
mod common;

fn refuses(source: &str) {
    let error = check_sources(source, "nominal-identity.cinch")
        .unwrap_err()
        .to_string();
    assert!(
        error.contains("A.Token") && error.contains("B.Token"),
        "{error}"
    );
}

#[test]
fn checked_quotation_cannot_change_its_nominal_owner() {
    refuses(include_str!(
        "../../experiments/malleable-syntax/regressions/nominal-identity.cinch"
    ));
}

#[test]
fn ordinary_functions_cannot_unify_same_spelled_types_from_different_modules() {
    refuses(
        r#"module A where
  data Token where
    AToken :: Token
  keep :: Token -> Token
  keep x = x
module B where
  data Token where
    BToken :: Token
module Main where
  import A (keep)
  import B
  wrong :: B.Token
  wrong = keep BToken
  main :: Int
  main = 0
"#,
    );
}

#[test]
fn a_macro_private_type_keeps_its_identity_when_the_caller_declares_the_same_name() {
    let source = r#"module A where
  export value
  data Token where
    AToken :: Int -> Token
  macro value :: Syntax -> Code Int
  macro value ignored = [| let x :: Token; x = AToken 42 in case x of { AToken n -> n } |]
module Main where
  import stage A
  data Token where
    CallerToken :: Bool -> Token
  main :: Int
  main = value!(())
"#;
    assert_eq!(
        run_source_with(source, &mut StagingCx::default()).unwrap(),
        42
    );
}

#[test]
fn type_aliases_and_reexports_preserve_the_defining_nominal_owner() {
    let source = r#"module A where
  data Token where
    AToken :: Int -> Token
module Facade where
  import A (Token as Public, AToken as Make)
  export Public, Make
module Main where
  import Facade as F
  read :: F.Public -> Int
  read (F.Make n) = n
  main :: Int
  main = read (F.Make 42)
"#;
    assert_eq!(
        run_source_with(source, &mut StagingCx::default()).unwrap(),
        42
    );
}

#[test]
fn same_named_records_keep_their_field_types_and_representations() {
    let source = r#"module A where
  record Token where
    n :: Int
  make :: Int -> Token
  make n = Token { n = n }
  read :: Token -> Int
  read token = token.n
module B where
  record Token where
    n :: Bool
  make :: Bool -> Token
  make n = Token { n = n }
  read :: Token -> Bool
  read token = token.n
module Main where
  import A (make as makeA, read as readA)
  import B (make as makeB, read as readB)
  main :: Int
  main = if readA (makeA 42) == 42 && readB (makeB True) then 0 else 1
"#;
    assert_eq!(
        run_source_with(source, &mut StagingCx::default()).unwrap(),
        0
    );
}

#[test]
fn imported_interfaces_record_the_actual_type_owner() {
    let dir = common::project("nominal_owner", &[
        ("A.cinch", "module A where\n  data Token where\n    AToken :: Token\n  make :: Token\n  make = AToken\n"),
        ("B.cinch", "module B where\n  data Token where\n    BToken :: Token\n"),
        ("Main.cinch", "module Main where\n  import A (make)\n  import B\n  main :: Int\n  main = 0\n"),
    ]);
    common::check(&dir, "Main.cinch").unwrap();
    let iface = cinch::interface::Iface::from_json(
        &std::fs::read_to_string(dir.join("build/A.cinchif")).unwrap(),
    )
    .unwrap();
    let make = iface
        .exports
        .values
        .iter()
        .find(|v| v.name == "make")
        .unwrap();
    assert!(make.scheme.contains("A.Token"), "{}", make.scheme);
    std::fs::write(dir.join("Main.cinch"), "module Main where\n  import A (make)\n  import B\n  wrong :: B.Token\n  wrong = make\n  main :: Int\n  main = 0\n").unwrap();
    let error = common::check(&dir, "Main.cinch").unwrap_err();
    assert!(
        error.contains("A.Token") && error.contains("B.Token"),
        "{error}"
    );
    common::cleanup(&dir);
}

#[test]
fn familiar_module_names_cannot_grant_new_types_intrinsic_identity() {
    let functor = format!(
        r#"{}
  export Functor, map, (<$>), Functor_Function, keepToken
  data Token where
    LToken :: Token
  keepToken :: Token -> Int
  keepToken LToken = 42
"#,
        include_str!("../prelude/Category/Functor.cinch")
    );
    let applicative = format!(
        r#"{}
  export Applicative, pure, (<*>), lift-a2, Applicative_Function, void, when, unless, ap, replicate-m, makeToken
  data Token where
    OToken :: Token
  makeToken :: Token
  makeToken = OToken
"#,
        include_str!("../prelude/Category/Applicative.cinch")
    );
    let dir = common::project("base_module_nominals", &[
        ("Functor.cinch", &functor), ("Applicative.cinch", &applicative),
        ("Main.cinch", "module Main where\n  import Category.Functor (keepToken)\n  import Category.Applicative (makeToken)\n  main :: Int\n  main = keepToken makeToken\n"),
    ]);
    let error = common::check(&dir, "Main.cinch").unwrap_err();
    assert!(
        error.contains("Category.Functor.Token") && error.contains("Category.Applicative.Token"),
        "{error}"
    );
    common::cleanup(&dir);
}
