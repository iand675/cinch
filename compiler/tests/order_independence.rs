//! Signature-level type references across the base family's homes.
//!
//! `Data.List.nth :: Nat -> List a -> Option a` and
//! `Data.Option.cat-options :: List (Option a) -> List a` make those two
//! homes mutually referential at the SIGNATURE level, and `Data.Result`'s
//! projections consume a `List` while `Data.Result` is checked before
//! `Data.List`. A signature's type names resolve only against the modules
//! checked before it, so no linear order of the three homes works. The
//! family's answer is structural: `Data.Core` declares the shared
//! inductive types (`List`, `Option`, `Result` and their constructors)
//! ahead of every home, and each public home re-exports its own type from
//! there (`design/base-library.md`, "Packages and the manifest").
//!
//! These tests pin the OUTCOME, not the ordering:
//!
//!   the trio resolves     one running program uses `nth` (List then
//!                         Option), `cat-options` (Option then List) and
//!                         the `Result` projections (`Data.Result` is
//!                         checked before both);
//!   unknowns still fail   a type that exists nowhere is still `unknown
//!                         type`, so resolution did not widen into "any
//!                         declared name anywhere";
//!   shadowing intact      a local `map` beats the prelude's (the
//!                         explicit-over-implicit rule of D145);
//!   re-export by name     a module declares a type, a second re-exports
//!                         it (`export T, Ctor`), and a third uses the
//!                         re-exported name in a signature and a body:
//!                         the idiom `Data.Core` and the public homes
//!                         rely on, pinned from both sides (an
//!                         unexported name stays out of scope).

use cinch::driver::{check_sources, driver, run_source, Mode};
use std::path::{Path, PathBuf};

// ----------------------------------------------------------------
// Shared helpers
// ----------------------------------------------------------------

fn expect_exit(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn write_project(files: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "cinch_order_independence_{}_{}",
        std::process::id(),
        files.as_ptr() as usize
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (name, src) in files {
        std::fs::write(dir.join(name), src).expect("write module");
    }
    dir
}

fn run_entry(dir: &Path, entry: &str) -> Result<i32, String> {
    driver(Mode::Run, &dir.join(entry), None).map_err(|e| e.to_string())
}

fn check_entry(dir: &Path, entry: &str) -> Result<(), String> {
    driver(Mode::Check, &dir.join(entry), None)
        .map(|_| ())
        .map_err(|e| e.to_string())
}

// ----------------------------------------------------------------
// The mutually referential homes
// ----------------------------------------------------------------

/// `nth` (List then Option), `cat-options` (Option then List) and the
/// `Result` projections (`either`, `is-ok`, `is-err`: checked before
/// either) in one program. Each check contributes a distinct bit to the
/// exit code, so a failure names itself.
#[test]
fn mutually_referential_types_compile_and_run() {
    expect_exit(
        "mutual-types",
        r#"module Main where
  xs :: List Nat
  xs = Cons 10 (Cons 20 (Cons 30 Nil))

  second :: Option Nat
  second = nth 1 xs

  second-value :: Nat
  second-value = from-option 0 second

  kept :: List Nat
  kept = cat-options (Cons (Some 7) (Cons None (Cons (Some 9) Nil)))

  emptied :: List Nat
  emptied = cat-options (Cons None (Cons None Nil))

  folded-ok :: Nat
  folded-ok = either (\e -> 0) (\x -> x) (Ok 5 :: Result Nat Nat)

  folded-err :: Nat
  folded-err = either (\e -> 0) (\x -> x) (Err 9 :: Result Nat Nat)

  ok-flag :: Bool
  ok-flag = is-ok (Ok 4 :: Result Nat Nat)

  err-flag :: Bool
  err-flag = is-err (Err 9 :: Result Nat Nat)

  main :: Int
  main = (if second-value == 20 then 0 else 1) + (if length kept == 2 then 0 else 2) + (if length emptied == 0 then 0 else 4) + (if folded-ok == 5 then 0 else 8) + (if folded-err == 0 then 0 else 16) + (if ok-flag then 0 else 32) + (if err-flag then 0 else 64)
"#,
        0,
    );
}

// ----------------------------------------------------------------
// Unknown types and shadowing
// ----------------------------------------------------------------

/// A type that no module declares is still rejected, with the message
/// naming it: making the trio resolve did not turn resolution into a
/// global name search.
#[test]
fn a_genuinely_unknown_type_is_still_rejected() {
    let src = r#"module Main where
  bogus :: NoSuchType -> Nat
  bogus x = 0

  main :: Int
  main = 0
"#;
    let err = check_sources(src, "Main.cinch")
        .err()
        .expect("an unknown type must not check")
        .to_string();
    assert!(
        err.contains("unknown type"),
        "the diagnostic should name the unknown type, got: {err}"
    );
}

/// A local `map` (Nat -> Nat) beats the prelude's `Data.List.map`: were
/// the family's to win, `map 7` would not typecheck and the program
/// would not compile at all, so a clean run proves the shadow.
#[test]
fn a_local_declaration_shadows_the_family_name() {
    expect_exit(
        "local-shadow",
        r#"module Main where
  map :: Nat -> Nat
  map n = 42

  main :: Int
  main = if map 7 == 42 then 0 else 1
"#,
        0,
    );
}

// ----------------------------------------------------------------
// Re-export by name (the Data.Core idiom)
// ----------------------------------------------------------------

/// The shared-type idiom: `Shared` declares the type, `Re` re-exports it
/// by name, and the entry uses the re-exported name in a signature and a
/// body. It must run; the second project pins the other side (a name the
/// middle module does not export stays out of scope).
#[test]
fn a_re_exported_type_reaches_its_consumer() {
    let dir = write_project(&[
        (
            "Shared.cinch",
            r#"module Shared where
  export T, MkT

  data T a where
    MkT :: a -> T a
"#,
        ),
        (
            "Re.cinch",
            r#"module Re where
  import Shared
  export T, MkT
"#,
        ),
        (
            "Main.cinch",
            r#"module Main where
  import Re

  unwrap :: forall a. T a -> a
  unwrap t = case t of
    MkT x -> x

  value :: T Nat
  value = MkT 7

  main :: Int
  main = if unwrap value == 7 then 0 else 1
"#,
        ),
    ]);
    check_entry(&dir, "Main.cinch").expect("the re-exporting project checks");
    let code = run_entry(&dir, "Main.cinch").expect("the re-exporting project runs");
    assert_eq!(code, 0, "the re-exported type should reach its consumer");
}

/// A module whose `export` list omits the type keeps it out of scope:
/// nothing is reachable merely by being declared elsewhere, which is what
/// makes `Data.Core`'s re-export the load-bearing step.
#[test]
fn a_type_left_out_of_an_export_list_stays_out_of_scope() {
    let dir = write_project(&[
        (
            "Hidden.cinch",
            r#"module Hidden where
  export h-secret

  data T a where
    MkT :: a -> T a

  h-secret :: Nat
  h-secret = 1
"#,
        ),
        (
            "Main.cinch",
            r#"module Main where
  import Hidden

  use :: T Nat -> Nat
  use t = 0

  main :: Int
  main = 0
"#,
        ),
    ]);
    let err = check_entry(&dir, "Main.cinch").expect_err("the unexported type must not check");
    assert!(
        err.contains("unknown type"),
        "the diagnostic should name the unknown type, got: {err}"
    );
}
