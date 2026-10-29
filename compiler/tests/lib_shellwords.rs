//! Text.Shellwords consumer semantics: Bourne-shell-style word
//! splitting and single-quote argument escaping (the Ruby `Shellwords`
//! port curated in design/stdlib-parity-ruby.md), driven through the
//! CLI pipeline like lib_foldable.rs.
//!
//! compiler/prelude/Text/Shellwords.cinch is not embedded in the
//! prelude, so each test writes the module beside a Main that imports
//! it explicitly (the Text.*/Effect.* family pattern). The checks pin
//! consumer-visible semantics only:
//!   split:  Ruby's documented cases (whitespace separation, single
//!           quotes literal, double-quote escapes), adjacent quoting,
//!           an unquoted backslash escaping the next scalar, and the
//!           POSIX double-quote escape set (a backslash before `$`, a
//!           backtick, `"`, backslash or a newline is dropped; every
//!           other backslash is literal);
//!   empty:  a quoted empty argument is an empty word (single-quoted
//!           and double-quoted alike), never dropped, including
//!           between words;
//!   errors: an unterminated single or double quote is the named
//!           Result error (the double-quote case includes a trailing
//!           backslash), and every other input yields words;
//!   escape: the empty string is two single quotes, an all-safe string
//!           is bare, and anything else is single-quoted with each
//!           internal single quote written as close-escaped-reopen;
//!   join:   escape each element, separate with one space;
//!   round trip: split (join (Cons s Nil)) == Ok (Cons s Nil) over
//!           hostile strings (spaces, quotes, backslashes, newline,
//!           dollar, star, backtick).
//!
//! Each check is a top-level declaration returning 0 or its bit; main
//! sums them, so a nonzero exit code is a bitmask naming the failed
//! checks (check k contributes bit 2^(k-1)).

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const SCAN: &str = include_str!("../prelude/Text/Scan.cinch");
const SHELLWORDS: &str = include_str!("../prelude/Text/Shellwords.cinch");

fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_shellwords_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [
        ("Text.Scan.cinch", SCAN),
        ("Text.Shellwords.cinch", SHELLWORDS),
        ("Main.cinch", main_src),
    ] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    match driver(Mode::Run, &dir.join("Main.cinch"), None) {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_ok(name: &str, main_src: &str) {
    let code = run_main(name, main_src);
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

/// The helper declarations every program shares (list equality over
/// String, the Result-shape probes and the round-trip predicate).
const HELPERS: &str = r##"  same-str :: String -> String -> Bool
  same-str a b = string-compare a b == 0

  same-words :: List String -> List String -> Bool
  same-words a b = case a of
    Nil -> case b of
      Nil -> True
      Cons _ _ -> False
    Cons x xs -> case b of
      Nil -> False
      Cons y ys -> if same-str x y then same-words xs ys else False

  ok-words :: Result ShellwordsError (List String) -> List String -> Bool
  ok-words r expected = case r of
    Err _ -> False
    Ok ws -> same-words ws expected

  is-sq-err :: Result ShellwordsError (List String) -> Bool
  is-sq-err r = case r of
    Err e -> case e of
      UnterminatedSingleQuote -> True
      UnterminatedDoubleQuote -> False
    Ok _ -> False

  is-dq-err :: Result ShellwordsError (List String) -> Bool
  is-dq-err r = case r of
    Err e -> case e of
      UnterminatedSingleQuote -> False
      UnterminatedDoubleQuote -> True
    Ok _ -> False

  round-trip-ok :: String -> Bool
  round-trip-ok s = ok-words (shellwords-split (shellwords-join (Cons s Nil))) (Cons s Nil)
"##;

fn program(body: &str) -> String {
    format!("module Main where\n  import Text.Shellwords\n\n{HELPERS}\n{body}\n")
}


#[test]
fn split_canonical_and_quote_cases() {
    expect_ok(
        "split-canonical",
        &program(
            r##"  c1 :: Int
  c1 = if ok-words (shellwords-split "foo bar\\ baz") (Cons "foo" (Cons "bar baz" Nil)) then 0 else 1

  c2 :: Int
  c2 = if ok-words (shellwords-split "three blind \"mice\"") (Cons "three" (Cons "blind" (Cons "mice" Nil))) then 0 else 2

  c3 :: Int
  c3 = if ok-words (shellwords-split "here are \"two words\"") (Cons "here" (Cons "are" (Cons "two words" Nil))) then 0 else 4

  c4 :: Int
  c4 = if ok-words (shellwords-split "ruby my_prog.rb | less") (Cons "ruby" (Cons "my_prog.rb" (Cons "|" (Cons "less" Nil)))) then 0 else 8

  c5 :: Int
  c5 = if ok-words (shellwords-split "one two 'three four'") (Cons "one" (Cons "two" (Cons "three four" Nil))) then 0 else 16

  c6 :: Int
  c6 = if ok-words (shellwords-split "a\"b\"c") (Cons "abc" Nil) then 0 else 32

  c7 :: Int
  c7 = if ok-words (shellwords-split "a\\ b c") (Cons "a b" (Cons "c" Nil)) then 0 else 64

  c8 :: Int
  c8 = if ok-words (shellwords-split "\"back\\`tick\"") (Cons "back`tick" Nil) then 0 else 128

  main :: Int
  main = c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8
"##,
        ),
    );
}

#[test]
fn split_preserves_empty_quoted_words() {
    expect_ok(
        "split-empty",
        &program(
            r##"  c1 :: Int
  c1 = if ok-words (shellwords-split "''") (Cons "" Nil) then 0 else 1

  c2 :: Int
  c2 = if ok-words (shellwords-split "\"\"") (Cons "" Nil) then 0 else 2

  c3 :: Int
  c3 = if ok-words (shellwords-split "a '' b") (Cons "a" (Cons "" (Cons "b" Nil))) then 0 else 4

  c4 :: Int
  c4 = if ok-words (shellwords-split "a \"\" b") (Cons "a" (Cons "" (Cons "b" Nil))) then 0 else 8

  main :: Int
  main = c1 + c2 + c3 + c4
"##,
        ),
    );
}

#[test]
fn split_reports_unterminated_quotes_and_boundaries() {
    expect_ok(
        "split-errors",
        &program(
            r##"  c1 :: Int
  c1 = if is-sq-err (shellwords-split "a 'b") then 0 else 1

  c2 :: Int
  c2 = if is-dq-err (shellwords-split "\"abc") then 0 else 2

  c3 :: Int
  c3 = if is-dq-err (shellwords-split "\"abc\\") then 0 else 4

  c4 :: Int
  c4 = if ok-words (shellwords-split "") (Nil :: List String) then 0 else 8

  c5 :: Int
  c5 = if ok-words (shellwords-split "   ") (Nil :: List String) then 0 else 16

  c6 :: Int
  c6 = if ok-words (shellwords-split "a\\") (Cons "a\\" Nil) then 0 else 32

  c7 :: Int
  c7 = if ok-words (shellwords-split "a \"b\\qc\" d") (Cons "a" (Cons "b\\qc" (Cons "d" Nil))) then 0 else 64

  c8 :: Int
  c8 = if ok-words (shellwords-split "  a   b  ") (Cons "a" (Cons "b" Nil)) then 0 else 128

  main :: Int
  main = c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8
"##,
        ),
    );
}

#[test]
fn escape_forms_and_join() {
    expect_ok(
        "escape-forms",
        &program(
            r##"  c1 :: Int
  c1 = if same-str (shellwords-escape "") "''" then 0 else 1

  c2 :: Int
  c2 = if same-str (shellwords-escape "abc") "abc" then 0 else 2

  c3 :: Int
  c3 = if same-str (shellwords-escape "a b") "'a b'" then 0 else 4

  c4 :: Int
  c4 = if same-str (shellwords-escape "a'b") "'a'\\''b'" then 0 else 8

  c5 :: Int
  c5 = if same-str (shellwords-escape "*") "'*'" then 0 else 16

  c6 :: Int
  c6 = if same-str (shellwords-escape "$") "'$'" then 0 else 32

  c7 :: Int
  c7 = if same-str (shellwords-join (Cons "a b" (Cons "c" Nil))) "'a b' c" then 0 else 64

  c8 :: Int
  c8 = if same-str (shellwords-join (Nil :: List String)) "" then 0 else 128

  main :: Int
  main = c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8
"##,
        ),
    );
}

#[test]
fn escape_quotes_newlines_and_the_safe_set() {
    expect_ok(
        "escape-more",
        &program(
            r##"  c1 :: Int
  c1 = if same-str (shellwords-escape "a\nb") "'a\nb'" then 0 else 1

  c2 :: Int
  c2 = if same-str (shellwords-escape "-_.:,+/@") "-_.:,+/@" then 0 else 2

  c3 :: Int
  c3 = if same-str (shellwords-escape "a\\b") "'a\\b'" then 0 else 4

  c4 :: Int
  c4 = if same-str (shellwords-escape "a`b") "'a`b'" then 0 else 8

  c5 :: Int
  c5 = if same-str (shellwords-escape "a\"b") "'a\"b'" then 0 else 16

  main :: Int
  main = c1 + c2 + c3 + c4 + c5
"##,
        ),
    );
}

#[test]
fn round_trip_over_hostile_strings() {
    expect_ok(
        "round-trip",
        &program(
            r##"  c1 :: Int
  c1 = if round-trip-ok "a b" then 0 else 1

  c2 :: Int
  c2 = if round-trip-ok "'" then 0 else 2

  c3 :: Int
  c3 = if round-trip-ok "\"" then 0 else 4

  c4 :: Int
  c4 = if round-trip-ok "\\" then 0 else 8

  c5 :: Int
  c5 = if round-trip-ok "\n" then 0 else 16

  c6 :: Int
  c6 = if round-trip-ok "$*" then 0 else 32

  c7 :: Int
  c7 = if round-trip-ok "`x`" then 0 else 64

  c8 :: Int
  c8 = if round-trip-ok "It's a \"test\"\n$HOME/*" then 0 else 128

  main :: Int
  main = c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8
"##,
        ),
    );
}
