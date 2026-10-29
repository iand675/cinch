//! Text.Scan consumer semantics for the text-level additions this slice made:
//! the `string-lines`/`string-unlines` round trip (with and without a trailing
//! newline, and on the empty string), `string-unwords`, the scalar-count
//! cutters `string-split-at`/`string-take`/`string-drop`, the predicate cutters
//! `string-take-while`/`string-drop-while`, and `string-char-index-of` (pinned
//! at scalar indices, not bytes). Every case is observed through a real program
//! run, never source text: each check is a top-level declaration returning 0 or
//! its bit, and `main` sums them, so a nonzero exit code is a bitmask naming
//! the failed checks (check k contributes bit 2^(k-1)). Text.Scan is not
//! embedded in the prelude, so each program writes it beside the Main that
//! imports it, exactly as lib_shellwords.rs does; Data.String IS embedded, so
//! no copy of it is written. The checks are top-level because a `case` in a
//! `let` binder's right-hand side must indent past the binder's column under
//! the layout rules.

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const SCAN: &str = include_str!("../prelude/Text/Scan.cinch");

fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_textops_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [("Text.Scan.cinch", SCAN), ("Main.cinch", main_src)] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    let path = dir.join("Main.cinch");
    let compiled = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver(Mode::Run, &path, None))
        .expect("spawn compiler thread")
        .join()
        .expect("compiler thread panicked");
    match compiled {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_ok(name: &str, body: &str) {
    let code = run_main(name, &program(body));
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

/// The shared declarations every program uses: string equality, list-of-string
/// equality, pair equality, the two `Option Nat` probes, and the digit
/// predicate the while-cutters are driven with. All top-level so a `case` never
/// sits in a `let` binder's right-hand side.
const HELPERS: &str = r#"  same-str :: String -> String -> Bool
  same-str a b = string-compare a b == 0

  same-words :: List String -> List String -> Bool
  same-words a b = case a of
    Nil -> case b of
      Nil -> True
      Cons _ _ -> False
    Cons x xs -> case b of
      Nil -> False
      Cons y ys -> if same-str x y then same-words xs ys else False

  pair-eq :: (String, String) -> (String, String) -> Bool
  pair-eq p q = case p of
    (a, b) -> case q of
      (c, d) -> if same-str a c then same-str b d else False

  index-is :: Option Nat -> Nat -> Bool
  index-is o n = case o of
    None -> False
    Some m -> m == n

  index-none :: Option Nat -> Bool
  index-none o = case o of
    None -> True
    Some _ -> False

  is-digit :: Char -> Bool
  is-digit c = is-ascii-digit c
"#;

fn program(body: &str) -> String {
    format!("module Main where\n  import Text.Scan\n\n{HELPERS}\n{body}\n")
}

/// The `string-lines`/`string-unlines` round trip and the two direct inverses.
/// `string-lines` drops one trailing empty line, so `string-unlines
/// (string-lines s)` is `s` with exactly one trailing LF (and the empty string
/// round-trips the empty string).
#[test]
fn lines_unlines_round_trip() {
    expect_ok(
        "lines-unlines",
        r#"  c1 :: Int
  c1 = if same-words (string-lines "a\nb\nc") (Cons "a" (Cons "b" (Cons "c" Nil))) then 0 else 1

  c2 :: Int
  c2 = if same-words (string-lines "a\nb\n") (Cons "a" (Cons "b" Nil)) then 0 else 2

  c3 :: Int
  c3 = if same-words (string-lines "") (Nil :: List String) then 0 else 4

  c4 :: Int
  c4 = if same-words (string-lines "\n") (Cons "" Nil) then 0 else 8

  c5 :: Int
  c5 = if same-str (string-unlines (Cons "a" (Cons "b" Nil))) "a\nb\n" then 0 else 16

  c6 :: Int
  c6 = if same-str (string-unlines Nil) "" then 0 else 32

  c7 :: Int
  c7 = if same-str (string-unlines (string-lines "a\nb\n")) "a\nb\n" then 0 else 64

  c8 :: Int
  c8 = if same-str (string-unlines (string-lines "a\nb")) "a\nb\n" then 0 else 128

  c9 :: Int
  c9 = if same-str (string-unlines (string-lines "")) "" then 0 else 256

  c10 :: Int
  c10 = if same-str (string-unlines (string-lines "\n")) "\n" then 0 else 512

  main :: Int
  main = c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8 + c9 + c10
"#,
    );
}

/// `string-unwords` separates with a single space; the empty list is the empty
/// string, a singleton has no separator, and interior empty strings still take
/// their two spaces (it is a join, not a filter).
#[test]
fn unwords_joins_with_single_space() {
    expect_ok(
        "unwords",
        r#"  c1 :: Int
  c1 = if same-str (string-unwords (Cons "a" (Cons "b" Nil))) "a b" then 0 else 1

  c2 :: Int
  c2 = if same-str (string-unwords Nil) "" then 0 else 2

  c3 :: Int
  c3 = if same-str (string-unwords (Cons "solo" Nil)) "solo" then 0 else 4

  c4 :: Int
  c4 = if same-str (string-unwords (Cons "a" (Cons "" (Cons "b" Nil)))) "a  b" then 0 else 8

  c5 :: Int
  c5 = if same-str (string-unwords (string-words "  a\tb  ")) "a b" then 0 else 16

  c6 :: Int
  c6 = if same-str (string-unwords (string-words "")) "" then 0 else 32

  main :: Int
  main = c1 + c2 + c3 + c4 + c5 + c6
"#,
    );
}

/// `string-split-at` / `string-take` / `string-drop` cut at a scalar count: 0
/// puts everything in the second part, a count past the end clamps, the empty
/// string yields empty parts, and the cut is at scalar boundaries (a multi-byte
/// scalar is never split).
#[test]
fn scalar_count_cutters() {
    expect_ok(
        "count-cutters",
        r#"  c1 :: Int
  c1 = if pair-eq (string-split-at 0 "hello") ("", "hello") then 0 else 1

  c2 :: Int
  c2 = if pair-eq (string-split-at 2 "hello") ("he", "llo") then 0 else 2

  c3 :: Int
  c3 = if pair-eq (string-split-at 5 "hello") ("hello", "") then 0 else 4

  c4 :: Int
  c4 = if pair-eq (string-split-at 99 "hello") ("hello", "") then 0 else 8

  c5 :: Int
  c5 = if pair-eq (string-split-at 0 "") ("", "") then 0 else 16

  c6 :: Int
  c6 = if same-str (string-take 2 "hello") "he" then 0 else 32

  c7 :: Int
  c7 = if same-str (string-take 0 "hello") "" then 0 else 64

  c8 :: Int
  c8 = if same-str (string-take 99 "hello") "hello" then 0 else 128

  c9 :: Int
  c9 = if same-str (string-take 3 "") "" then 0 else 256

  c10 :: Int
  c10 = if same-str (string-drop 2 "hello") "llo" then 0 else 512

  c11 :: Int
  c11 = if same-str (string-drop 99 "hello") "" then 0 else 1024

  c12 :: Int
  c12 = if same-str (string-drop 3 "") "" then 0 else 2048

  c13 :: Int
  c13 = if same-str (string-take 1 "\u{e9}x") "\u{e9}" then 0 else 4096

  c14 :: Int
  c14 = if same-str (string-drop 1 "\u{e9}x") "x" then 0 else 8192

  c15 :: Int
  c15 = if pair-eq (string-split-at 1 "\u{e9}x") ("\u{e9}", "x") then 0 else 16384

  main :: Int
  main = c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8 + c9 + c10 + c11 + c12 + c13 + c14 + c15
"#,
    );
}

/// `string-take-while` / `string-drop-while`: a matching run that does not
/// reach the end, one that consumes the whole string, a leading mismatch, and
/// the empty string.
#[test]
fn predicate_cutters() {
    expect_ok(
        "while-cutters",
        r#"  c1 :: Int
  c1 = if same-str (string-take-while is-digit "12x3") "12" then 0 else 1

  c2 :: Int
  c2 = if same-str (string-take-while is-digit "123") "123" then 0 else 2

  c3 :: Int
  c3 = if same-str (string-take-while is-digit "abc") "" then 0 else 4

  c4 :: Int
  c4 = if same-str (string-take-while is-digit "") "" then 0 else 8

  c5 :: Int
  c5 = if same-str (string-drop-while is-digit "12x3") "x3" then 0 else 16

  c6 :: Int
  c6 = if same-str (string-drop-while is-digit "123") "" then 0 else 32

  c7 :: Int
  c7 = if same-str (string-drop-while is-digit "abc") "abc" then 0 else 64

  c8 :: Int
  c8 = if same-str (string-drop-while is-digit "") "" then 0 else 128

  main :: Int
  main = c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8
"#,
    );
}

/// `string-char-index-of` reports the scalar index of the first occurrence,
/// `None` when the scalar is absent (including the empty string), and the index
/// counts scalars, not bytes.
#[test]
fn char_index_of_scalar_positions() {
    expect_ok(
        "char-index",
        r#"  c1 :: Int
  c1 = if index-is (string-char-index-of 'h' "hello") 0 then 0 else 1

  c2 :: Int
  c2 = if index-is (string-char-index-of 'l' "hello") 2 then 0 else 2

  c3 :: Int
  c3 = if index-is (string-char-index-of 'o' "hello") 4 then 0 else 4

  c4 :: Int
  c4 = if index-none (string-char-index-of 'z' "hello") then 0 else 8

  c5 :: Int
  c5 = if index-none (string-char-index-of 'x' "") then 0 else 16

  c6 :: Int
  c6 = if index-is (string-char-index-of 'x' "\u{e9}x") 1 then 0 else 32

  c7 :: Int
  c7 = if index-is (string-char-index-of '\u{e9}' "\u{e9}x") 0 then 0 else 64

  main :: Int
  main = c1 + c2 + c3 + c4 + c5 + c6 + c7
"#,
    );
}
