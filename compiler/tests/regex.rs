//! P36 text and regex tests: `Text.Regex` (the RE2-style
//! linear-time engine: the pattern language, the named refusals, the
//! Pike VM match and the regex-driven scanning family) and
//! `Text.Scan` (the literal-string scanning and formatting
//! utilities).
//!
//! The headline is the no-ReDoS pin. `(a+)+b` against a long run of
//! `a` is the classic exponential backtracking bomb; here the
//! engine's VM dispatch count is OBSERVABLE (`regex-match-counted`)
//! and grows linearly in the input, pinned two ways: a
//! wall-clock-independent step bound (dispatches per input scalar)
//! and a doubling ratio, plus an honest wall-clock probe on 10k
//! scalars where a backtracker would never finish.
//!
//! Every program is a real `Main` compiled and run through the same
//! driver the CLI uses. The modules beside it are the checked in
//! `compiler/prelude/Text/Regex.cinch` and `Text/Scan.cinch` sources
//! (the driver does not embed them: the P30 note's pattern).

use cinch::driver::{driver, Mode};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const REGEX_SRC: &str = include_str!("../prelude/Text/Regex.cinch");
const SCAN_SRC: &str = include_str!("../prelude/Text/Scan.cinch");

// ----------------------------------------------------------------
// Shared harness
// ----------------------------------------------------------------

/// A result alias that defaults its error type (house rule).
type RunResult<T> = std::result::Result<T, String>;

/// A self-contained project: the two toolkit modules beside a `Main`.
fn write_project(name: &str, files: &[(&str, &str)]) -> PathBuf {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("cinch_regex_{}_{}_{}", name, std::process::id(), n));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    std::fs::write(dir.join("Text.Regex.cinch"), REGEX_SRC).expect("write Text.Regex");
    std::fs::write(dir.join("Text.Scan.cinch"), SCAN_SRC).expect("write Text.Scan");
    for (name, src) in files {
        std::fs::write(dir.join(name), src).unwrap_or_else(|e| panic!("write {name}: {e}"));
    }
    dir
}

/// Run a driver invocation on a thread with a large stack (the
/// checker recurses over a program's elaborated code).
fn drive(mode: Mode, entry: &Path, out: Option<&Path>) -> i32 {
    let entry = entry.to_path_buf();
    let out = out.map(|p| p.to_path_buf());
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver(mode, &entry, out.as_deref()))
        .expect("spawn harness thread")
        .join()
        .expect("harness thread")
        .unwrap_or_else(|e| panic!("compilation failed: {e}"))
}

/// Compile and run a `Main`; the exit code is the program's
/// `main :: Int` result (0 on success, the failing case number on a
/// named failure).
fn run_main(name: &str, main: &str) -> i32 {
    let dir = write_project(name, &[("main.cinch", main)]);
    let code = drive(Mode::Run, &dir.join("main.cinch"), None);
    let _ = std::fs::remove_dir_all(&dir);
    code
}

fn expect_exit(name: &str, main: &str, expected: i32) {
    let code = run_main(name, main);
    assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}");
}

/// Compile a `Main` to an executable and return it (kept alive in
/// its project directory until `remove_dir`).
fn build_exe(name: &str, main: &str) -> (PathBuf, PathBuf) {
    let dir = write_project(name, &[("main.cinch", main)]);
    let exe = dir.join("program");
    let code = drive(Mode::Build, &dir.join("main.cinch"), Some(&exe));
    assert_eq!(code, 0, "{name}: build failed");
    (dir, exe)
}

/// Run the executable with its output sink captured (`run_program_io`'s
/// shape).
fn run_exe(exe: &Path, input: &[u8]) -> (i32, String) {
    use std::io::Write;
    let mut child = std::process::Command::new(exe)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("spawn {}: {e}", exe.display()));
    child
        .stdin
        .take()
        .expect("child stdin")
        .write_all(input)
        .expect("feed input");
    let out = child.wait_with_output().expect("wait for child");
    (
        out.status.code().unwrap_or(1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
    )
}

/// The `main =` check chain: `if ckNN then (if ... else NN+1) else
/// 1`, so the exit code names the first failing check.
fn chain(checks: &[&str]) -> String {
    let mut body = String::from("0");
    for (i, name) in checks.iter().enumerate().rev() {
        body = format!("(if {name} then {body} else {})", i + 1);
    }
    body
}

/// A self-contained `(chunk-bytes ...)` expression over the source's
/// bytes (the checker recurses per literal element, so one giant
/// literal overflows it; the parsers toolkit's helper).
fn bytes_lit(s: &str) -> String {
    let chunks: Vec<String> = s
        .as_bytes()
        .chunks(100)
        .map(|c| {
            let nums: Vec<String> = c.iter().map(|b| b.to_string()).collect();
            format!("bytes-from-list [{}]", nums.join(", "))
        })
        .collect();
    format!("(chunk-bytes [{}])", chunks.join(", "))
}

fn string_lit(s: &str) -> String {
    format!("(case string-from-bytes {} of Some s -> s; None -> empty-string)", bytes_lit(s))
}

/// The byte helpers the generated literals need.
const BYTE_HELPERS: &str = r#"  cat-bytes :: Bytes -> Bytes -> Bytes
  cat-bytes a b =
    let out = bytes-new (bytes-len a + bytes-len b) in
    let u = copy-bytes-into out a 0 0 (bytes-len a) in
    let v = copy-bytes-into out b (bytes-len a) 0 (bytes-len b) in
    out

  chunk-bytes :: List Bytes -> Bytes
  chunk-bytes cs = case cs of
    Nil -> bytes-new 0
    Cons c rest -> cat-bytes c (chunk-bytes rest)
"#;

// ----------------------------------------------------------------
// 1. The pattern language end to end: literals, classes, `.`,
//    quantifiers, alternation, groups and captures, anchors, escapes.
// ----------------------------------------------------------------

const LANGUAGE_HELPERS: &str = r#"  shows :: String -> String -> String -> Bool
  shows p t w = case parse-regex p of
    Err _ -> False
    Ok re -> case regex-match re t of
      None -> False
      Some m -> match-text m == w

  shows-group :: String -> String -> Nat -> String -> Bool
  shows-group p t k w = case parse-regex p of
    Err _ -> False
    Ok re -> case regex-match re t of
      None -> False
      Some m -> case match-group m k of
        None -> False
        Some g -> g == w

  no-match :: String -> String -> Bool
  no-match p t = case parse-regex p of
    Err _ -> False
    Ok re -> case regex-match re t of
      None -> True
      Some _ -> False

  shows-span :: String -> String -> Nat -> Nat -> Bool
  shows-span p t s0 e0 = case parse-regex p of
    Err _ -> False
    Ok re -> case regex-match re t of
      None -> False
      Some m -> if match-start m == s0 then match-end m == e0 else False

  group-count-is :: String -> String -> Nat -> Bool
  group-count-is p t n = case parse-regex p of
    Err _ -> False
    Ok re -> case regex-match re t of
      None -> False
      Some m -> match-group-count m == n

  count-is :: String -> String -> Nat -> Bool
  count-is p t n = case parse-regex p of
    Err _ -> False
    Ok re -> regex-count re t == n
"#;

const LANGUAGE_DEFS: &str = r#"  ck01 :: Bool
  ck01 = shows "abc" "xxabcyy" "abc"

  ck02 :: Bool
  ck02 = shows "a.c" "xabcx" "abc"

  ck03 :: Bool
  ck03 = shows "ab*" "cab" "ab"

  ck04 :: Bool
  ck04 = shows "a+" "baaa" "aaa"

  ck05 :: Bool
  ck05 = if shows "colou?r" "color" "color" then shows "colou?r" "xcolour" "colour" else False

  ck06 :: Bool
  ck06 = shows "a{2,4}" "aaaaa" "aaaa"

  ck07 :: Bool
  ck07 = shows "a{2,4}?" "aaaaa" "aa"

  ck08 :: Bool
  ck08 = shows "a{3}" "zaaazz" "aaa"

  ck09 :: Bool
  ck09 = shows "a{2,}" "zaaaz" "aaa"

  ck10 :: Bool
  ck10 = shows "ab|cd" "xcd" "cd"

  ck11 :: Bool
  ck11 = shows "[a-c]+" "zabcz" "abc"

  ck12 :: Bool
  ck12 = shows "[^0-9]+" "12ab34" "ab"

  ck13 :: Bool
  ck13 = shows "[\\x41-\\x43]+" "zABCz" "ABC"

  ck14 :: Bool
  ck14 = shows "\\d+" "ab123cd" "123"

  ck15 :: Bool
  ck15 = shows "\\w+@\\w+" "mail: bob@home" "bob@home"

  ck16 :: Bool
  ck16 = shows-group "(\\w+)@(\\w+)" "mail: bob@home" 1 "bob"

  ck17 :: Bool
  ck17 = shows-group "(\\w+)@(\\w+)" "mail: bob@home" 2 "home"

  ck18 :: Bool
  ck18 = shows-span "(a)(b)" "zab" 1 3

  ck19 :: Bool
  ck19 = if shows "^abc" "abc" "abc" then no-match "^a" "ba" else False

  ck20 :: Bool
  ck20 = if shows "c$" "abc" "c" then no-match "b$" "abc" else False

  ck21 :: Bool
  ck21 = if shows "\\bcat\\b" "the cat sat" "cat" then no-match "\\bcat\\b" "cats" else False

  ck22 :: Bool
  ck22 = shows-span "a+" "baab" 1 3

  ck23 :: Bool
  ck23 = shows "(a|ab)" "ab" "a"

  ck24 :: Bool
  ck24 = shows "(ab|a)" "ab" "ab"

  ck25 :: Bool
  ck25 = if shows "(a+)+b" "zaab" "aab" then shows-group "(a+)+b" "zaab" 1 "aa" else False

  g26a :: Bool
  g26a = if shows "((a)(b))" "zab" "ab" then shows-group "((a)(b))" "zab" 1 "ab" else False

  g26b :: Bool
  g26b = if g26a then shows-group "((a)(b))" "zab" 2 "a" else False

  ck26 :: Bool
  ck26 = if g26b then shows-group "((a)(b))" "zab" 3 "b" else False

  ck27 :: Bool
  ck27 = shows "" "ab" ""

  ck28 :: Bool
  ck28 = no-match "a.b" "a\nb"

  ck29 :: Bool
  ck29 = if shows "(?:ab)+" "xababy" "abab" then group-count-is "(?:ab)+" "xababy" 1 else False

  ck30 :: Bool
  ck30 = group-count-is "(a)(b)" "ab" 3

  ck31 :: Bool
  ck31 = shows "a*?" "aaa" ""

  ck32 :: Bool
  ck32 = if shows (regex-escape "a.b*c") "zz a.b*c xx" "a.b*c" then no-match (regex-escape "a.b") "axb" else False

  ck33 :: Bool
  ck33 = shows "\\x2b" "a+b" "+"

  ck34 :: Bool
  ck34 = count-is "a" "banana" 3

  ck35 :: Bool
  ck35 = shows-span "a" "banana" 1 2

  ck36 :: Bool
  ck36 = shows "[0-9]{2,3}" "x12345y" "123"

  ck37 :: Bool
  ck37 = count-is "a*" "bb" 3

  ck38 :: Bool
  ck38 = count-is "\\d+" "a1b22c333" 3
"#;

#[test]
fn pattern_language_matches_and_captures_end_to_end() {
    let checks: &[&str] = &[
        "ck01", "ck02", "ck03", "ck04", "ck05", "ck06", "ck07", "ck08", "ck09", "ck10", "ck11",
        "ck12", "ck13", "ck14", "ck15", "ck16", "ck17", "ck18", "ck19", "ck20", "ck21", "ck22",
        "ck23", "ck24", "ck25", "ck26", "ck27", "ck28", "ck29", "ck30", "ck31", "ck32", "ck33",
        "ck34", "ck35", "ck36", "ck37", "ck38",
    ];
    let src = format!(
        "module Main where\n  import Text.Regex\n  import Data.String\n  import Data.Nat\n  import Data.Option\n\n{LANGUAGE_HELPERS}{LANGUAGE_DEFS}\n  main :: Int\n  main = {}\n",
        chain(checks)
    );
    expect_exit("pattern-language", &src, 0);
}

// ----------------------------------------------------------------
// 2. The named refusals: every construct outside the subset refuses
//    with an error naming itself, never a silent misbehavior.
// ----------------------------------------------------------------

const REFUSAL_DEFS: &str = r#"  backref :: Bool
  backref = case parse-regex "(a)\\1" of
    Err e -> case e of
      BackreferenceRefused _ -> True
      _ -> False
    Ok _ -> False

  digit-escape :: Bool
  digit-escape = case parse-regex "\\0" of
    Err e -> case e of
      BackreferenceRefused _ -> True
      _ -> False
    Ok _ -> False

  lookaround :: String -> Bool
  lookaround p = case parse-regex p of
    Err e -> case e of
      LookaroundRefused _ -> True
      _ -> False
    Ok _ -> False

  named-group :: String -> Bool
  named-group p = case parse-regex p of
    Err e -> case e of
      NamedGroupRefused _ -> True
      _ -> False
    Ok _ -> False

  flag-group :: Bool
  flag-group = case parse-regex "(?i)" of
    Err e -> case e of
      FlagGroupRefused _ -> True
      _ -> False
    Ok _ -> False

  unicode-property :: Bool
  unicode-property = case parse-regex "\\p{L}" of
    Err e -> case e of
      UnicodePropertyRefused _ -> True
      _ -> False
    Ok _ -> False

  unsupported :: String -> Bool
  unsupported p = case parse-regex p of
    Err e -> case e of
      UnsupportedEscape _ _ -> True
      _ -> False
    Ok _ -> False

  nested-repeat :: Bool
  nested-repeat = case parse-regex "a**" of
    Err e -> case e of
      NestedRepeatRefused _ -> True
      _ -> False
    Ok _ -> False

  empty-width-repeat :: String -> Bool
  empty-width-repeat p = case parse-regex p of
    Err e -> case e of
      EmptyWidthRepeatRefused _ -> True
      _ -> False
    Ok _ -> False

  dangling :: Bool
  dangling = case parse-regex "*a" of
    Err e -> case e of
      DanglingQuantifier _ -> True
      _ -> False
    Ok _ -> False

  repeat-limit :: String -> Bool
  repeat-limit p = case parse-regex p of
    Err e -> case e of
      RepeatLimitExceeded _ -> True
      _ -> False
    Ok _ -> False

  bad-repeat :: String -> Bool
  bad-repeat p = case parse-regex p of
    Err e -> case e of
      InvalidRepeatSyntax _ -> True
      _ -> False
    Ok _ -> False

  bad-range :: Bool
  bad-range = case parse-regex "a{2,1}" of
    Err e -> case e of
      InvalidRepeatRange _ -> True
      _ -> False
    Ok _ -> False

  bad-class-range :: String -> Bool
  bad-class-range p = case parse-regex p of
    Err e -> case e of
      InvalidClassRange _ -> True
      _ -> False
    Ok _ -> False

  unterminated :: Bool
  unterminated = case parse-regex "(" of
    Err e -> case e of
      UnterminatedGroup _ -> True
      _ -> False
    Ok _ -> False

  unterminated-class :: Bool
  unterminated-class = case parse-regex "[a" of
    Err e -> case e of
      UnterminatedClass _ -> True
      _ -> False
    Ok _ -> False

  stray-paren :: Bool
  stray-paren = case parse-regex "a)" of
    Err e -> case e of
      UnmatchedCloseParen _ -> True
      _ -> False
    Ok _ -> False

  trailing-backslash :: Bool
  trailing-backslash = case parse-regex "a\\" of
    Err e -> case e of
      TrailingBackslash _ -> True
      _ -> False
    Ok _ -> False

  bad-hex :: String -> Bool
  bad-hex p = case parse-regex p of
    Err e -> case e of
      InvalidHexEscape _ -> True
      _ -> False
    Ok _ -> False

  too-large :: Bool
  too-large = case parse-regex "(a{1000,1000}){1000,1000}" of
    Err e -> case e of
      ProgramTooLarge _ -> True
      _ -> False
    Ok _ -> False

  render-names :: Bool
  render-names = case parse-regex "(a)\\1" of
    Err e -> string-contains "backreference-refused" (render-regex-error e)
    Ok _ -> False

  r01 :: Bool
  r01 = backref

  r02 :: Bool
  r02 = digit-escape

  r03 :: Bool
  r03 = lookaround "(?=x)"

  r04 :: Bool
  r04 = lookaround "(?!x)"

  r05 :: Bool
  r05 = lookaround "(?<=x)"

  r06 :: Bool
  r06 = lookaround "(?<!x)"

  r07 :: Bool
  r07 = named-group "(?<name>x)"

  r08 :: Bool
  r08 = named-group "(?P<name>x)"

  r09 :: Bool
  r09 = flag-group

  r10 :: Bool
  r10 = unicode-property

  r11 :: Bool
  r11 = unsupported "\\q"

  r12 :: Bool
  r12 = unsupported "\\A"

  r13 :: Bool
  r13 = unsupported "\\z"

  r14 :: Bool
  r14 = nested-repeat

  r15 :: Bool
  r15 = empty-width-repeat "^*"

  r16 :: Bool
  r16 = empty-width-repeat "\\b+"

  r17 :: Bool
  r17 = bad-range

  r18 :: Bool
  r18 = repeat-limit "a{1001}"

  r19 :: Bool
  r19 = repeat-limit "a{2000}"

  r20 :: Bool
  r20 = unterminated

  r21 :: Bool
  r21 = unterminated-class

  r22 :: Bool
  r22 = stray-paren

  r23 :: Bool
  r23 = trailing-backslash

  r24 :: Bool
  r24 = bad-hex "\\x{110000}"

  r25 :: Bool
  r25 = bad-hex "\\x{}"

  r26 :: Bool
  r26 = bad-hex "\\xg"

  r27 :: Bool
  r27 = dangling

  r28 :: Bool
  r28 = bad-repeat "a{"

  r29 :: Bool
  r29 = bad-repeat "a{,3}"

  r30 :: Bool
  r30 = bad-class-range "[\\d-z]"

  r31 :: Bool
  r31 = bad-class-range "[z-a]"

  r32 :: Bool
  r32 = too-large

  r33 :: Bool
  r33 = render-names
"#;

#[test]
fn named_refusals_refuse_by_name() {
    let checks: &[&str] = &[
        "r01", "r02", "r03", "r04", "r05", "r06", "r07", "r08", "r09", "r10", "r11", "r12", "r13",
        "r14", "r15", "r16", "r17", "r18", "r19", "r20", "r21", "r22", "r23", "r24", "r25", "r26",
        "r27", "r28", "r29", "r30", "r31", "r32", "r33",
    ];
    let src = format!(
        "module Main where\n  import Text.Regex\n  import Text.Scan\n  import Data.String\n  import Data.Nat\n  import Data.Option\n\n{REFUSAL_DEFS}\n  main :: Int\n  main = {}\n",
        chain(checks)
    );
    expect_exit("named-refusals", &src, 0);
}

// ----------------------------------------------------------------
// 3. The scanning/formatting utilities over real inputs: scan, split,
//    replace-all (function and template forms), and the Text.Scan
//    surface.
// ----------------------------------------------------------------

const SCAN_DEFS: &str = r#"  scan-join :: String -> String -> String -> String
  scan-join p t sep = case parse-regex p of
    Err _ -> "!"
    Ok re -> string-join sep (regex-scan-texts re t)

  split-join :: String -> String -> String -> String
  split-join p t sep = case parse-regex p of
    Err _ -> "!"
    Ok re -> string-join sep (regex-split re t)

  repl-all :: String -> String -> String
  repl-all p t = case parse-regex p of
    Err _ -> "!"
    Ok re -> regex-replace-all re bracket t

  repl-first :: String -> String -> String
  repl-first p t = case parse-regex p of
    Err _ -> "!"
    Ok re -> regex-replace-first re bracket t

  bracket :: Match -> String
  bracket m = string-concat "<" (string-concat (match-text m) ">")

  repl-tpl :: String -> String -> String -> String
  repl-tpl p tpl t = case parse-regex p of
    Err _ -> "!"
    Ok re -> regex-replace-all-template re tpl t

  count-of :: String -> String -> Nat
  count-of p t = case parse-regex p of
    Err _ -> 0
    Ok re -> regex-count re t

  idx-is :: String -> String -> Nat -> Bool
  idx-is needle t n = case string-index-of needle t of
    None -> False
    Some i -> i == n

  idx-none :: String -> String -> Bool
  idx-none needle t = case string-index-of needle t of
    None -> True
    Some _ -> False

  fmt-ok :: String -> List String -> String -> Bool
  fmt-ok tmpl args w = case format tmpl args of
    Err _ -> False
    Ok s -> s == w

  fmt-missing :: String -> List String -> Bool
  fmt-missing tmpl args = case format tmpl args of
    Err e -> case e of
      MissingFormatArgument _ -> True
      _ -> False
    Ok _ -> False

  fmt-unused :: String -> List String -> Bool
  fmt-unused tmpl args = case format tmpl args of
    Err e -> case e of
      UnusedFormatArgument _ -> True
      _ -> False
    Ok _ -> False

  fmt-brace :: String -> List String -> Bool
  fmt-brace tmpl args = case format tmpl args of
    Err e -> case e of
      UnmatchedFormatBrace _ -> True
      _ -> False
    Ok _ -> False

  two-a :: List String
  two-a = Cons "B" (Cons "D" Nil)

  three-a :: List String
  three-a = Cons "a" (Cons "b" (Cons "c" Nil))

  s01 :: Bool
  s01 = scan-join "\\w+" "hi there 42" "/" == "hi/there/42"

  s02 :: Bool
  s02 = split-join "[,;]+" "a,,b;c" "/" == "a/b/c"

  s03 :: Bool
  s03 = split-join "[,;]+" "a,b," "/" == "a/b"

  s04 :: Bool
  s04 = split-join "\\s+" " x y " "/" == "/x/y"

  s05 :: Bool
  s05 = repl-all "(\\d+)" "a1b22c" == "a<1>b<22>c"

  s06 :: Bool
  s06 = repl-first "(\\d+)" "a1b22c" == "a<1>b22c"

  s07 :: Bool
  s07 = repl-tpl "(\\w+)=(\\w+)" "$2=$1" "a=1 b=2" == "1=a 2=b"

  s08 :: Bool
  s08 = repl-tpl "a\\+b" "$$" "a+b" == "$"

  s09 :: Bool
  s09 = repl-tpl "(\\d)" "[$1]" "a1b" == "a[1]b"

  s10 :: Bool
  s10 = count-of "\\d" "a1b22" == 3

  s11 :: Bool
  s11 = regex-escape "a.b*c" == "a\\.b\\*c"

  s12 :: Bool
  s12 = idx-is "42" "call 42 now" 5

  s13 :: Bool
  s13 = idx-none "zz" "call"

  s14 :: Bool
  s14 = string-join "/" (string-split-on "," "a,b") == "a/b"

  s15 :: Bool
  s15 = string-join "/" (string-split-on "" "ab") == "a/b"

  s16 :: Bool
  s16 = string-join "," three-a == "a,b,c"

  s17 :: Bool
  s17 = string-strip "  hi \n" == "hi"

  s18 :: Bool
  s18 = string-chomp "hi\n\n" == "hi\n"

  s19 :: Bool
  s19 = string-chomp "hi\r\n" == "hi"

  s20 :: Bool
  s20 = string-chop "hi" == "h"

  s21 :: Bool
  s21 = string-upcase-ascii "mIx3d" == "MIX3D"

  s22 :: Bool
  s22 = string-downcase-ascii "mIx3d" == "mix3d"

  s23 :: Bool
  s23 = string-capitalize-ascii "hELLO" == "Hello"

  s24 :: Bool
  s24 = string-pad-left 5 "ab" == "   ab"

  s25 :: Bool
  s25 = string-pad-right 5 "ab" == "ab   "

  s26 :: Bool
  s26 = string-center 7 "ab" == "  ab   "

  s27 :: Bool
  s27 = string-normalize-newlines "a\r\nb\rc" == "a\nb\nc"

  s28 :: Bool
  s28 = fmt-ok "a{}c{}" two-a "aBcD"

  s29 :: Bool
  s29 = fmt-ok "x{{y" Nil "x{y"

  s30 :: Bool
  s30 = fmt-missing "{}{}" (Cons "A" Nil)

  s31 :: Bool
  s31 = fmt-unused "{}" two-a

  s32 :: Bool
  s32 = fmt-brace "{x" Nil

  s33 :: Bool
  s33 = fmt-brace "a}" Nil

  s34 :: Bool
  s34 = string-join "/" (string-words "  a  b\tc ") == "a/b/c"

  s35 :: Bool
  s35 = string-join "/" (string-lines "a\nb\n") == "a/b"

  s36 :: Bool
  s36 = string-join "/" (string-lines "a\n\nb") == "a//b"

  g37a :: Bool
  g37a = if string-starts-with "ab" "abc" then string-ends-with "bc" "abc" else False

  s37 :: Bool
  s37 = if g37a then string-contains "b" "abc" else False

  s38 :: Bool
  s38 = idx-is "" "abc" 0

  s39 :: Bool
  s39 = string-chop "" == ""

  s40 :: Bool
  s40 = case string-lines "" of
    Nil -> True
    Cons _ _ -> False
"#;

#[test]
fn scanning_and_formatting_utilities() {
    let checks: &[&str] = &[
        "s01", "s02", "s03", "s04", "s05", "s06", "s07", "s08", "s09", "s10", "s11", "s12", "s13",
        "s14", "s15", "s16", "s17", "s18", "s19", "s20", "s21", "s22", "s23", "s24", "s25", "s26",
        "s27", "s28", "s29", "s30", "s31", "s32", "s33", "s34", "s35", "s36", "s37", "s38", "s39",
        "s40",
    ];
    let src = format!(
        "module Main where\n  import Text.Regex\n  import Text.Scan\n  import Data.String\n  import Data.Nat\n  import Data.Option\n  import Data.List\n\n{SCAN_DEFS}\n  main :: Int\n  main = {}\n",
        chain(checks)
    );
    expect_exit("scan-format", &src, 0);
}

// ----------------------------------------------------------------
// 4. THE NO-REDOS PIN: `(a+)+b` over growing `a` runs. The VM
//    dispatch count is linear in the input (the wall-clock-independent
//    argument), and the 10k run completes against the clock where a
//    backtracking engine would never return.
// ----------------------------------------------------------------

const EVIL_SRC: &str = r#"module Main where
  import Text.Regex
  import Data.Nat
  import Data.Bytes
  import Data.String
  import Data.List
  import Data.Option
  import Text.Show

  run-of :: Nat -> String
  run-of n =
    let buf = bytes-new n in
    let u = fill buf 0 n 97 in
    case string-from-bytes buf of
      None -> empty-string
      Some t -> t

  run-of-b :: Nat -> String
  run-of-b n = string-concat (run-of n) "b"

  fill :: Bytes -> Nat -> Nat -> Nat -> Unit
  fill b i n v =
    if i >= n then ()
    else
      let u = bytes-set b i v in
      fill b (i + 1) n v

  fields :: List String -> String
  fields xs = case xs of
    Nil -> empty-string
    Cons y ys -> case ys of
      Nil -> y
      _ -> string-concat y (string-concat " " (fields ys))

  none-flag :: Option Match -> String
  none-flag m = case m of
    None -> "0"
    Some _ -> "1"

  fail-line :: Regex -> Nat -> String
  fail-line re n =
    case regex-match-counted re (run-of n) of
      (m, steps) -> fields (Cons "fail" (Cons (nat-to-string n) (Cons (nat-to-string steps) (Cons (none-flag m) Nil))))

  ok-line :: Regex -> Nat -> String
  ok-line re n =
    case regex-match-counted re (run-of-b n) of
      (m, steps) -> case m of
        None -> "ok nomatch"
        Some mm -> fields (Cons "ok" (Cons (nat-to-string n) (Cons (nat-to-string steps) (Cons (nat-to-string (match-start mm)) (Cons (nat-to-string (match-end mm)) (Cons (nat-to-string (string-length (match-text mm))) (Cons (group1-len mm) Nil)))))))

  group1-len :: Match -> String
  group1-len mm = case match-group mm 1 of
    None -> "-"
    Some g -> nat-to-string (string-length g)

  main :: Unit ->{IO r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    n -> n

  work :: forall r. using (io :: Io r) => Unit ->{IO r, Diverge} Int
  work u =
    case parse-regex "(a+)+b" of
      Err e -> case ioWrite (string-to-bytes (render-regex-error e)) of
        w -> 1
      Ok re ->
        let l1 = fail-line re 2500 in
        let l2 = fail-line re 5000 in
        let l3 = fail-line re 10000 in
        let l4 = ok-line re 10000 in
        let all = string-concat l1 (string-concat "\n" (string-concat l2 (string-concat "\n" (string-concat l3 (string-concat "\n" (string-concat l4 "\n")))))) in
        case ioWrite (string-to-bytes all) of
          w -> 0
"#;

/// One parsed `fail`/`ok` line of the evil probe.
struct EvilLine {
    kind: String,
    n: u64,
    steps: u64,
    matched: bool,
    start: u64,
    end: u64,
    text_len: u64,
    group1_len: Option<u64>,
}

fn parse_evil_line(line: &str) -> RunResult<EvilLine> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let kind = parts.first().map(|s| s.to_string()).unwrap_or_default();
    let num = |i: usize| -> RunResult<u64> {
        parts
            .get(i)
            .ok_or_else(|| format!("missing field {i} in line {line:?}"))?
            .parse()
            .map_err(|e| format!("bad field {i} in line {line:?}: {e}"))
    };
    match kind.as_str() {
        "fail" => Ok(EvilLine {
            kind,
            n: num(1)?,
            steps: num(2)?,
            matched: num(3)? != 0,
            start: 0,
            end: 0,
            text_len: 0,
            group1_len: None,
        }),
        "ok" => Ok(EvilLine {
            kind,
            n: num(1)?,
            steps: num(2)?,
            matched: true,
            start: num(3)?,
            end: num(4)?,
            text_len: num(5)?,
            group1_len: Some(num(6)?),
        }),
        other => Err(format!("unknown line kind {other:?} in {line:?}")),
    }
}

#[test]
fn evil_pattern_is_linear_by_step_count_and_clock() {
    let (dir, exe) = build_exe("evil", EVIL_SRC);
    let t0 = Instant::now();
    let (code, out) = run_exe(&exe, b"");
    let elapsed = t0.elapsed();
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(code, 0, "evil probe: expected exit 0, got {code}; output:\n{out}");

    let mut lines = Vec::new();
    for line in out.lines() {
        if line.trim().is_empty() {
            continue;
        }
        lines.push(parse_evil_line(line).unwrap_or_else(|e| panic!("evil probe: {e}")));
    }
    assert_eq!(lines.len(), 4, "evil probe: expected 4 lines, got:\n{out}");

    let fails: Vec<&EvilLine> = lines.iter().filter(|l| l.kind == "fail").collect();
    let oks: Vec<&EvilLine> = lines.iter().filter(|l| l.kind == "ok").collect();
    assert_eq!(fails.len(), 3, "evil probe: expected 3 fail lines, got:\n{out}");
    assert_eq!(oks.len(), 1, "evil probe: expected 1 ok line, got:\n{out}");

    // The failure runs: no `b`, so no match, and the dispatch count
    // is the observable. This is the wall-clock-independent no-ReDoS
    // argument: a backtracking engine explores 2^(n-1) paths here;
    // the VM dispatch count is (measured) 24n - 8, so the two pins
    // below bound the work per input scalar and the growth when the
    // input doubles.
    for l in &fails {
        assert!(!l.matched, "evil probe: (a+)+b must not match a pure a-run of {}: {out}", l.n);
    }
    let (n1, s1) = (fails[0].n, fails[0].steps as f64);
    let (n2, s2) = (fails[1].n, fails[1].steps as f64);
    let (n3, s3) = (fails[2].n, fails[2].steps as f64);
    assert_eq!((n1, n2, n3), (2500, 5000, 10000), "evil probe: unexpected input sizes: {out}");

    // Pin 1: dispatches per input scalar stay under 64 (measured
    // 24; the compiled program is 14 instructions, so this is the
    // linear bound with a small constant).
    for l in &fails {
        assert!(
            l.steps <= 64 * l.n,
            "evil probe: {} dispatches over {} scalars exceeds 64 per scalar (the linear bound): {out}",
            l.steps,
            l.n
        );
    }
    // Pin 2: doubling the input at most 2.2x the work (measured
    // exactly 2.0x).
    assert!(s2 <= 2.2 * s1, "evil probe: steps(5000) = {s2} not within 2.2x of steps(2500) = {s1}: {out}");
    assert!(s3 <= 2.2 * s2, "evil probe: steps(10000) = {s3} not within 2.2x of steps(5000) = {s2}: {out}");

    // The success run: 10k a's and a b, matched whole, with the
    // capture group spanning the a-run. Still linear work.
    let ok = oks[0];
    assert_eq!(ok.n, 10000, "evil probe: unexpected ok size: {out}");
    assert!(ok.matched, "evil probe: (a+)+b must match 10k a's and a b: {out}");
    assert_eq!(ok.start, 0, "evil probe: match must start at 0: {out}");
    assert_eq!(ok.end, 10001, "evil probe: match must span to the b: {out}");
    assert_eq!(ok.text_len, 10001, "evil probe: match text must be the whole run: {out}");
    assert_eq!(ok.group1_len, Some(10000), "evil probe: group 1 must span the a-run: {out}");
    assert!(ok.steps <= 64 * ok.n, "evil probe: the successful run must stay linear too: {out}");

    // The honest timing probe: 10k scalars through the engine,
    // measured around the run (compilation excluded). A
    // backtracking engine on this pattern and input does not
    // complete in any wall clock; the bound below is a smoke
    // ceiling, not a performance claim.
    println!(
        "evil (a+)+b: steps 2500={} 5000={} 10000={} ({} per scalar), ok 10000={} steps, wall {:?}",
        fails[0].steps,
        fails[1].steps,
        fails[2].steps,
        fails[2].steps / fails[2].n,
        ok.steps,
        elapsed
    );
    assert!(
        elapsed < Duration::from_secs(120),
        "evil probe: the 10k run took {elapsed:?}; expected seconds at most"
    );
}

// ----------------------------------------------------------------
// 5. Unicode at the documented boundary: matching is over scalar
//    values (never bytes), and the honest NOT-done list is pinned:
//    no canonical normalization (precomposed and decomposed
//    spellings of one grapheme are different inputs), no Unicode
//    case data (the case family is ASCII-only), `\w` is ASCII.
// ----------------------------------------------------------------

#[test]
fn unicode_scalars_flow_through_matching() {
    // Multi-byte subjects, built from bytes: precomposed e-acute,
    // decomposed e + combining acute, a Japanese run, and a coffee
    // cup. Expected texts in the program use the lexer's \u{...}
    // escapes so the source stays ASCII.
    let precomposed = string_lit("\u{e9}");
    let decomposed = string_lit("e\u{301}");
    let cafe_cup = string_lit("caf\u{e9}\u{2615}");
    let jp = string_lit("\u{65e5}\u{672c}\u{8a9e}");
    let jp_pair = string_lit("\u{65e5}\u{672c}");
    let accented = string_lit("x\u{e9}\u{fc}y");
    let cafe = string_lit("caf\u{e9}");
    let cafe_upper = string_lit("CAF\u{e9}");

    let checks: &[&str] = &[
        "u01", "u02", "u03", "u04", "u05", "u06", "u07", "u08", "u09", "u10", "u11",
    ];
    let defs = r#"  subj-cafe-cup :: String
  subj-cafe-cup = @CAFE_CUP@

  subj-jp :: String
  subj-jp = @JP@

  subj-jp-pair :: String
  subj-jp-pair = @JP_PAIR@

  subj-acc :: String
  subj-acc = @ACCENTED@

  subj-pre :: String
  subj-pre = @PRECOMPOSED@

  subj-dec :: String
  subj-dec = @DECOMPOSED@

  subj-cafe :: String
  subj-cafe = @CAFE@

  subj-cafe-up :: String
  subj-cafe-up = @CAFE_UPPER@

  u01 :: Bool
  u01 = shows "\\x{2615}" subj-cafe-cup "\u{2615}"

  u02 :: Bool
  u02 = count-is "." subj-jp 3

  u03 :: Bool
  u03 = count-is "." subj-cafe-cup 5

  u04 :: Bool
  u04 = shows "[\\x{E9}-\\x{FC}]+" subj-acc "\u{E9}\u{FC}"

  u05 :: Bool
  u05 = if shows "^\\x{E9}$" subj-pre "\u{E9}" then no-match "^\\x{E9}$" subj-dec else False

  u06 :: Bool
  u06 = shows "^e\\x{301}$" subj-dec "e\u{301}"

  u07 :: Bool
  u07 = string-upcase-ascii subj-cafe == subj-cafe-up

  u08 :: Bool
  u08 = shows-group "(\\x{65E5}\\x{672C})" subj-jp-pair 1 "\u{65E5}\u{672C}"

  u09 :: Bool
  u09 = shows-span "\\x{2615}" subj-cafe-cup 5 8

  u10 :: Bool
  u10 = shows-span "." subj-jp 0 3

  u11 :: Bool
  u11 = count-is "\\w+" subj-cafe 1
"#
    .replace("@CAFE_CUP@", &cafe_cup)
    .replace("@JP@", &jp)
    .replace("@JP_PAIR@", &jp_pair)
    .replace("@ACCENTED@", &accented)
    .replace("@PRECOMPOSED@", &precomposed)
    .replace("@DECOMPOSED@", &decomposed)
    .replace("@CAFE_UPPER@", &cafe_upper)
    .replace("@CAFE@", &cafe);
    let src = format!(
        "module Main where\n  import Text.Regex\n  import Text.Scan\n  import Data.String\n  import Data.Nat\n  import Data.Option\n  import Data.List\n  import Data.Bytes\n\n{BYTE_HELPERS}\n{LANGUAGE_HELPERS}{defs}\n  main :: Int\n  main = {}\n",
        chain(checks)
    );
    expect_exit("unicode-scope", &src, 0);
}
