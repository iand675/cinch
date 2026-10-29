//! Text.OptionParser consumer semantics: the declarative flag
//! specification (`List OptionSpec`) over an argv-shaped `List String`.
//!
//! Each test compiles a real project: the module under test
//! (`Text.OptionParser.cinch`) plus, for the interoperation test,
//! `Text.Args.cinch`, beside a `Main.cinch`, driven through the CLI
//! pipeline like `json.rs` (the sources are written beside the entry,
//! which is how a non-embedded family module is loaded). The programs
//! exercise only consumer-visible semantics:
//!   the argv forms (long separate, `=`-joined, short attached and
//!     separate, clusters, `--` end, lone `-` positional),
//!   the named converters (string, integer, float, boolean presence),
//!   the bindings by long name, last-occurrence-wins repetition, the
//!     positionals in order and the present flags,
//!   each named refusal (unknown option, missing argument, invalid
//!     value, ambiguous prefix, unexpected value), and
//!   the usage table containing every declared option.
//!
//! Each check is a top-level `check-*` function returning 0 or its bit;
//! `main` sums them, so a nonzero exit code is a bitmask naming the
//! failed checks (check k contributes bit 2^(k-1)) and any failure mode
//! of one check shares that bit, keeping the total inside the 8-bit exit
//! status. Each test asserts exit 0, so a nonzero code names the check
//! that saw the wrong behavior. The checks are top-level because a `case`
//! in a `let` binder's right-hand side must indent past the binder's
//! column.

use cinch::driver::{driver, Mode};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

const OPTION_PARSER: &str = include_str!("../prelude/Text/OptionParser.cinch");
const ARGS: &str = include_str!("../prelude/Text/Args.cinch");
const SCAN_SRC: &str = include_str!("../prelude/Text/Scan.cinch");

/// The shared option specification: a string option with a short name, an
/// integer and a float converter, two presence flags (so the present-flag
/// list has an order to pin), and a second string option, plus two
/// long-only options whose names share a prefix (`color`/`columns`) so an
/// ambiguous abbreviation is reachable.
const SPECS: &str = r#"  specs :: List OptionSpec
  specs =
    Cons (OptSpec (Some "n") "name" ConvString "the name")
      (Cons (OptSpec (Some "c") "count" ConvInt "how many")
        (Cons (OptSpec (Some "r") "ratio" ConvFloat "the ratio")
          (Cons (OptSpec None "verbose" ConvFlag "chatty verbosity")
            (Cons (OptSpec (Some "q") "quiet" ConvFlag "quiet output")
              (Cons (OptSpec (Some "o") "output" ConvString "output path")
                (Cons (OptSpec None "color" ConvString "the color")
                  (Cons (OptSpec None "columns" ConvInt "terminal width")
                    Nil)))))))

  argv :: List String
  argv = Cons "--name" (Cons "alice" (Cons "-c3" (Cons "in.txt" (Cons "--output=out.txt" (Cons "-c" (Cons "4" (Cons "extra" (Cons "--verbose" (Cons "-q" (Cons "-r2.5" (Cons "--" (Cons "--not-an-option" (Cons "after" Nil)))))))))))))
"#;

/// The observers every program shares: string comparison, the binding
/// readers over the typed accessors, list agreement, and the error
/// matchers.
const HELPERS: &str = r#"  str-eq :: String -> String -> Bool
  str-eq a b = string-compare a b == 0

  parsed :: List String -> ParsedOptions
  parsed xs = case parse-options specs xs of
    Ok opts -> opts
    Err _ -> ParsedOptions Nil Nil

  value-of :: ParsedOptions -> String -> String
  value-of opts n = case option-value opts n of
    Some v -> value-to-string v
    None -> "<unset>"

  int-of :: ParsedOptions -> String -> Int
  int-of opts n = case option-value opts n of
    Some v -> case value-int v of
      Some k -> k
      None -> 0 - 1
    None -> 0 - 1

  float-of :: ParsedOptions -> String -> String
  float-of opts n = case option-value opts n of
    Some v -> case value-float v of
      Some f -> value-to-string (VFloat f)
      None -> "<not-float>"
    None -> "<unset>"

  eq-list-str :: List String -> List String -> Bool
  eq-list-str xs ys = case xs of
    Nil -> case ys of
      Nil -> True
      Cons _ _ -> False
    Cons a xs2 -> case ys of
      Nil -> False
      Cons b ys2 -> if str-eq a b then eq-list-str xs2 ys2 else False

  is-unknown :: Result OptionParseError ParsedOptions -> Bool
  is-unknown r = case r of
    Ok _ -> False
    Err e -> case e of
      UnknownOption _ -> True
      _ -> False

  is-missing :: Result OptionParseError ParsedOptions -> Bool
  is-missing r = case r of
    Ok _ -> False
    Err e -> case e of
      MissingArgument _ -> True
      _ -> False

  is-invalid :: Result OptionParseError ParsedOptions -> Bool
  is-invalid r = case r of
    Ok _ -> False
    Err e -> case e of
      InvalidValue _ _ -> True
      _ -> False

  is-unexpected :: Result OptionParseError ParsedOptions -> Bool
  is-unexpected r = case r of
    Ok _ -> False
    Err e -> case e of
      UnexpectedValue _ -> True
      _ -> False

  ambiguous-candidates :: Result OptionParseError ParsedOptions -> List String
  ambiguous-candidates r = case r of
    Ok _ -> Nil
    Err e -> case e of
      AmbiguousOption _ cs -> cs
      _ -> Nil
"#;

fn project_dir(name: &str) -> PathBuf {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_optparser_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    dir
}

/// Compose a `Main` module: the standard imports, then `body` (specs,
/// helpers and the eight checks), then a `main` summing them.
fn main_module(extra_imports: &str, body: &str) -> String {
    let mut src =
        String::from("module Main where\n\n  import Data.String\n  import Text.OptionParser\n");
    src.push_str(extra_imports);
    src.push_str(body);
    src.push('\n');
    src.push_str("  main :: Int\n");
    src.push_str("  main = check1 + check2 + check3 + check4 + check5 + check6 + check7 + check8\n");
    src
}

/// Run a driver invocation on a thread with a large stack (the checker
/// recurses over a program's elaborated code; this module's programs are
/// deeper than a default test thread's stack allows, the same reason
/// `json.rs`/`lib_toml.rs` drive on a large stack).
fn drive(entry: &Path) -> i32 {
    let entry = entry.to_path_buf();
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver(Mode::Run, &entry, None))
        .expect("spawn harness thread")
        .join()
        .expect("harness thread")
        .unwrap_or_else(|e| panic!("compilation failed: {e}"))
}

fn run_main(name: &str, main_src: &str, with_args: bool) -> i32 {
    let dir = project_dir(name);
    std::fs::write(dir.join("Text.OptionParser.cinch"), OPTION_PARSER).expect("write module");
    std::fs::write(dir.join("Text.Scan.cinch"), SCAN_SRC).expect("write Text.Scan");
    if with_args {
        std::fs::write(dir.join("Text.Args.cinch"), ARGS).expect("write Text.Args");
    }
    std::fs::write(dir.join("Main.cinch"), main_src).expect("write main");
    drive(&dir.join("Main.cinch"))
}

fn expect_ok(name: &str, main_src: &str) {
    let code = run_main(name, main_src, false);
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

fn body(extra: &str, checks: &str) -> String {
    format!("{SPECS}\n{HELPERS}\n{extra}\n{checks}\n")
}

/// The argv-form and converter program (check bit 1 name, 2 the repeated
/// integer, 4 the `=`-joined value, 8 the presence flag, 16 the
/// positionals including the option-shaped word after `--`, 32 the absent
/// option, 64 the present-flag list, 128 the float): a mixed argv with a
/// long separate option, a short attached integer, a positional, a
/// `=`-joined option, a separated long repeat of the integer (last wins),
/// a bare positional, a presence flag, an attached short float, and the
/// `--` option end whose later words are values even when they look like
/// options.
#[test]
fn argv_forms_bind_values_positionals_and_flags() {
    let checks = r#"  check1 :: Int
  check1 = if str-eq (value-of (parsed argv) "name") "alice" then 0 else 1

  check2 :: Int
  check2 = if int-of (parsed argv) "count" == 4 then 0 else 2

  check3 :: Int
  check3 = if str-eq (value-of (parsed argv) "output") "out.txt" then 0 else 4

  check4 :: Int
  check4 = if option-flag-present (parsed argv) "verbose" then 0 else 8

  check5 :: Int
  check5 = if eq-list-str (option-positionals (parsed argv)) (Cons "in.txt" (Cons "extra" (Cons "--not-an-option" (Cons "after" Nil)))) then 0 else 16

  check6 :: Int
  check6 = if option-has (parsed argv) "color" then 32 else 0

  check7 :: Int
  check7 = if eq-list-str (option-flags (parsed argv)) (Cons "verbose" (Cons "quiet" Nil)) then 0 else 64

  check8 :: Int
  check8 = if str-eq (float-of (parsed argv) "ratio") "2.5e0" then 0 else 128
"#;
    expect_ok("forms", &main_module("", &body("", checks)));
}

/// The short-form and cluster program over its own spec (single-character
/// names for the flag, the integer and the string; check bit 1 the
/// clustered flag, 2 the attached clustered value, 4 the cluster
/// positional, 8 the attached `=`-joined short value, 16 the separated
/// short string, 32 the missing-argument refusal, 64 the unknown-short
/// refusal, 128 the lone-dash positional).
#[test]
fn short_clusters_and_short_value_forms() {
    let extra = r#"  specsb :: List OptionSpec
  specsb =
    Cons (OptSpec (Some "v") "verbose" ConvFlag "chatty")
      (Cons (OptSpec (Some "c") "count" ConvInt "how many")
        (Cons (OptSpec (Some "n") "name" ConvString "the name")
          Nil))

  pb :: List String -> ParsedOptions
  pb xs = case parse-options specsb xs of
    Ok opts -> opts
    Err _ -> ParsedOptions Nil Nil

  cluster :: List String
  cluster = Cons "-vc3" (Cons "tail" Nil)
"#;
    let checks = r#"  check1 :: Int
  check1 = if option-flag-present (pb cluster) "verbose" then 0 else 1

  check2 :: Int
  check2 = if int-of (pb cluster) "count" == 3 then 0 else 2

  check3 :: Int
  check3 = if eq-list-str (option-positionals (pb cluster)) (Cons "tail" Nil) then 0 else 4

  check4 :: Int
  check4 = if int-of (pb (Cons "-c=7" Nil)) "count" == 7 then 0 else 8

  check5 :: Int
  check5 = if str-eq (value-of (pb (Cons "-n" (Cons "bob" Nil))) "name") "bob" then 0 else 16

  check6 :: Int
  check6 = if is-missing (parse-options specsb (Cons "-vc" Nil)) then 0 else 32

  check7 :: Int
  check7 = if is-unknown (parse-options specsb (Cons "-vz" Nil)) then 0 else 64

  check8 :: Int
  check8 = if eq-list-str (option-positionals (pb (Cons "-" Nil))) (Cons "-" Nil) then 0 else 128
"#;
    expect_ok("clusters", &main_module("", &body(extra, checks)));
}

/// The named-refusal program (check bit 1 the unknown long, 2 the unknown
/// short, 4 the invalid integer, 8 the invalid float, 16 the missing
/// argument, 32 the value on a flag, 64 the ambiguous prefix with its
/// candidates and a unique prefix that resolves, 128 the error rendering):
/// unknown long and short, invalid integer and float spellings, a missing
/// argument, a value on a flag, prefix matching both ways, and the
/// rendered error text.
#[test]
fn named_error_paths_and_prefix_matching() {
    let checks = r#"  check1 :: Int
  check1 = if is-unknown (parse-options specs (Cons "--nope" Nil)) then 0 else 1

  check2 :: Int
  check2 = if is-unknown (parse-options specs (Cons "-z" Nil)) then 0 else 2

  check3 :: Int
  check3 = if is-invalid (parse-options specs (Cons "--count=x" Nil)) then 0 else 4

  check4 :: Int
  check4 = if is-invalid (parse-options specs (Cons "--ratio=abc" Nil)) then 0 else 8

  check5 :: Int
  check5 = if is-missing (parse-options specs (Cons "--name" Nil)) then 0 else 16

  check6 :: Int
  check6 = if is-unexpected (parse-options specs (Cons "--verbose=1" Nil)) then 0 else 32

  check7 :: Int
  check7 = if eq-list-str (ambiguous-candidates (parse-options specs (Cons "--col" Nil))) (Cons "color" (Cons "columns" Nil)) && option-flag-present (parsed (Cons "--verb" Nil)) "verbose" then 0 else 64

  check8 :: Int
  check8 = if str-eq (render-option-parse-error (UnknownOption "--nope")) "unknown option: --nope" && str-eq (render-option-parse-error (AmbiguousOption "col" (Cons "color" (Cons "columns" Nil)))) "ambiguous option col: could be color, columns" then 0 else 128
"#;
    expect_ok("errors", &main_module("", &body("", checks)));
}

/// The usage table and the `Text.Args` interoperation (check bit 1 a long
/// name, 2 another long name, 4 the short/long/placeholder rendering, 8 a
/// help string, 16 a converter placeholder, 32 the agreed string binding,
/// 64 the agreed integer binding, 128 the short cluster `Text.Args`
/// refuses): `option-help` names every option, the two readers agree on
/// the long words, and `Text.Args` falls through as a positional the short
/// cluster that `Text.OptionParser` accepts.
#[test]
fn help_table_and_text_args_interoperation() {
    let extra = r#"  has-sub :: String -> String -> Bool
  has-sub needle s = has-at needle s 0

  has-at :: String -> String -> Nat -> Bool
  has-at needle s i =
    if i + string-length needle > string-length s then False
    else if str-eq (string-slice s i (i + string-length needle)) needle then True
    else has-at needle s (i + 1)

  argspecs :: List ArgSpec
  argspecs = Cons (ValueSpec "name") (Cons (ValueSpec "count") (Cons (FlagSpec "verbose") Nil))

  argv-a :: List String
  argv-a = Cons "--name=alice" (Cons "--count=3" (Cons "pos" Nil))

  args-ok :: List String -> Args
  args-ok xs = case parse-args argspecs xs of
    Ok a -> a
    Err _ -> Args Nil Nil

  opt-str :: Option String -> String
  opt-str o = case o of
    Some s -> s
    None -> "<novalue>"

  argvalue :: String -> List OptVal -> String
  argvalue n opts = case opts of
    Nil -> "<none>"
    Cons o rest -> case o of
      OptVal m v -> if str-eq m n then opt-str v else argvalue n rest
"#;
    let checks = r#"  check1 :: Int
  check1 = if has-sub "--name" (option-help specs) then 0 else 1

  check2 :: Int
  check2 = if has-sub "--columns" (option-help specs) then 0 else 2

  check3 :: Int
  check3 = if has-sub "-n, --name <value>" (option-help specs) then 0 else 4

  check4 :: Int
  check4 = if has-sub "how many" (option-help specs) then 0 else 8

  check5 :: Int
  check5 = if has-sub "--ratio <float>" (option-help specs) then 0 else 16

  check6 :: Int
  check6 = if str-eq (value-of (parsed argv-a) "name") (argvalue "name" (args-options (args-ok argv-a))) then 0 else 32

  check7 :: Int
  check7 = if str-eq (argvalue "count" (args-options (args-ok argv-a))) "3" then 0 else 64

  check8 :: Int
  check8 = if eq-list-str (args-values (args-ok (Cons "-c3" Nil))) (Cons "-c3" Nil) then 0 else 128
"#;
    let src = main_module(
        "\n  import Text.Args (ArgSpec, FlagSpec, ValueSpec, Args, OptVal, parse-args, args-options, args-values)\n",
        &body(extra, checks),
    );
    let code = run_main("interop", &src, true);
    assert_eq!(code, 0, "interop: check bitmask {code}");
}
