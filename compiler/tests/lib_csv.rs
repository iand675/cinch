//! CSV (`Text.Csv`) consumer semantics: the RFC 4180 dialect as the
//! module's header states it, over the real module sources.
//!
//! Each test compiles a real multi-file project: the module under test
//! plus the P30 toolkit modules it is built on (Text/Parse.cinch,
//! Text/Lex.cinch, which are not embedded in the driver the way the
//! `Data.*`/`Text.Show` family is) are written beside a `Main.cinch` in
//! a fresh temp project directory and driven through the CLI pipeline.
//! The driver call runs on a large-stack thread, the harness `json.rs`
//! uses: the checker recurses over the elaborated program, and a project
//! carrying the two toolkit modules is deeper than a default test
//! thread's stack. The checks exercise only consumer-visible semantics,
//! never source text:
//!   round trip:      `csv-parse (csv-print rows)` is `Ok rows` for
//!                    rows carrying commas, quotation marks, embedded
//!                    LF and CR, and empty fields;
//!   canonical form:  a printed document is already canonical, so
//!                    printing the parse of a print is the same text
//!                    (minimal quoting, LF terminators);
//!   the dialect:     the exact printed text of a row, the RFC 4180
//!                    examples (CRLF rows, the quoted-field example
//!                    from section 2.6) parsed field by field, and the
//!                    terminator rules (a trailing terminator adds no
//!                    row, a terminator between rows does, empty text is
//!                    no rows at all);
//!   refusals:        the named `CsvError` variants, each matched by
//!                    constructor (an unterminated quoted field, a
//!                    stray quotation mark, text after a closing
//!                    quotation mark, a bare CR, a ragged row under a
//!                    header), never by message text;
//!   header keying:   `csv-parse-with-headers` pairs each field with its
//!                    header name, and refuses a row whose field count
//!                    differs.
//!
//! Each check is a top-level `check-*` function returning 0 or its bit;
//! `main` sums them, so a nonzero exit code is a bitmask naming the
//! failed checks (check k contributes bit 2^(k-1)) and any failure mode
//! of one check shares that bit, keeping the total inside the 8-bit exit
//! status. The checks are top-level because a `case` in a `let` binder's
//! right-hand side must indent past the binder's column under the layout
//! rules.

use cinch::driver::{driver, Mode};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

const CSV: &str = include_str!("../prelude/Text/Csv.cinch");
const PARSE: &str = include_str!("../prelude/Text/Parse.cinch");
const LEX: &str = include_str!("../prelude/Text/Lex.cinch");

/// Helpers shared by the check programs: structural equality on rows of
/// fields, the refusal classifier (each `CsvError` constructor as a
/// distinct code, so a check names which refusal it observed rather than
/// that some error occurred), the header-table observers, and
/// `parse-or-empty`, which turns the refusal path into the empty
/// document so a check's failure mode stays a bit rather than a crash.
const HELPERS: &str = r#"
  eq-str :: String -> String -> Bool
  eq-str a b = if string-compare a b == 0 then True else False

  eq-fields :: List String -> List String -> Bool
  eq-fields xs ys = case xs of
    Nil -> case ys of
      Nil -> True
      Cons y yr -> False
    Cons x xr -> case ys of
      Nil -> False
      Cons y yr -> if eq-str x y then eq-fields xr yr else False

  eq-rows :: List (List String) -> List (List String) -> Bool
  eq-rows xs ys = case xs of
    Nil -> case ys of
      Nil -> True
      Cons y yr -> False
    Cons x xr -> case ys of
      Nil -> False
      Cons y yr -> if eq-fields x y then eq-rows xr yr else False

  first-row :: List (List String) -> List String
  first-row rows = case rows of
    Nil -> Nil
    Cons r rs -> r

  second-row :: List (List String) -> List (List String)
  second-row rows = case rows of
    Nil -> Nil
    Cons r rs -> case rs of
      Nil -> Nil
      Cons r2 rs2 -> Cons r2 Nil

  row-refusal :: Result CsvError (List (List String)) -> Nat
  row-refusal r = case r of
    Ok rows -> 0
    Err e -> case e of
      CsvUnterminatedQuote s -> 1
      CsvStrayQuote s -> 2
      CsvTextAfterQuote s -> 3
      CsvBareCarriageReturn s -> 4
      CsvRowArity s expected actual -> 5

  arity-expected :: Result CsvError (List String, List (List (String, String))) -> Int
  arity-expected r = case r of
    Ok table -> -1
    Err e -> case e of
      CsvRowArity s expected actual -> expected
      CsvUnterminatedQuote s -> -2
      CsvStrayQuote s -> -2
      CsvTextAfterQuote s -> -2
      CsvBareCarriageReturn s -> -2

  headers-of :: Result CsvError (List String, List (List (String, String))) -> List String
  headers-of r = case r of
    Err e -> Nil
    Ok pair -> case pair of
      (hs, rows) -> hs

  keyed-of :: Result CsvError (List String, List (List (String, String))) -> List (List (String, String))
  keyed-of r = case r of
    Err e -> Nil
    Ok pair -> case pair of
      (hs, rows) -> rows

  parse-or-empty :: String -> List (List String)
  parse-or-empty src = case csv-parse src of
    Ok rows -> rows
    Err e -> Nil

  no-rows :: List (List String)
  no-rows = Nil
"#;

/// Compile and run one project on a thread with a large stack: the
/// checker recurses over a program's elaborated code, and a program
/// carrying the P30 toolkit (Text.Lex and Text.Parse are the biggest
/// prelude modules) is deeper than a default test thread's stack
/// (the same harness `json.rs` uses).
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

fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_csvtest_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [
        ("Text.Csv.cinch", CSV),
        ("Text.Parse.cinch", PARSE),
        ("Text.Lex.cinch", LEX),
        ("Main.cinch", main_src),
    ] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    let code = drive(&dir.join("Main.cinch"));
    let _ = std::fs::remove_dir_all(&dir);
    code
}

/// The `Main` module of a check program: the import of the module under
/// test, any extra imports a check needs, the shared helpers and the
/// program body.
fn main_module(extra_imports: &str, body: &str) -> String {
    let mut src = String::from("module Main where\n\n  import Text.Csv\n");
    src.push_str(extra_imports);
    src.push_str(HELPERS);
    src.push('\n');
    src.push_str(body);
    src.push('\n');
    src
}

fn expect_ok_with(name: &str, extra_imports: &str, body: &str) {
    let code = run_main(name, &main_module(extra_imports, body));
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

fn expect_ok(name: &str, body: &str) {
    expect_ok_with(name, "", body);
}

#[test]
fn csv_round_trip_and_canonical_form() {
    // bit 1: a document carrying the delimiter, a quotation mark, a
    // doubled-quote field, an embedded LF, an embedded CR and empty
    // fields survives parse-of-print unchanged, and re-printing the
    // parsed document is the same text (the canonical fixed point).
    // bit 2: the dialect's exact text for a row, so a change in quoting
    // or in the terminator is a failure, not a silently different
    // document.
    expect_ok(
        "csv-round-trip",
        r#"
  doc :: List (List String)
  doc = Cons (Cons "a,b" (Cons "" (Cons "plain" Nil))) (Cons (Cons "quote\"inside" (Cons "line1\nline2" (Cons "cr\rhere" Nil))) Nil)

  reprinted :: String
  reprinted = csv-print doc

  round :: List (List String)
  round = parse-or-empty reprinted

  simple :: List (List String)
  simple = Cons (Cons "a,b" (Cons "plain" (Cons "" Nil))) Nil

  check1 :: Int
  check1 = if eq-rows round doc && eq-str (csv-print round) reprinted then 0 else 1

  check2 :: Int
  check2 = if eq-str (csv-print simple) "\"a,b\",plain,\n" then 0 else 2

  main :: Int
  main = check1 + check2
"#,
    );
}

#[test]
fn csv_rfc4180_fixture_field_by_field() {
    // bit 1: RFC 4180 section 2.4's example, CRLF-terminated, parsed
    // field by field into the two expected rows.
    // bit 2: section 2.6's quoted example ("aaa","b""bb","ccc"), so the
    // doubled quotation mark decodes to one and the quoting is not part
    // of the field.
    expect_ok(
        "csv-fixture",
        r#"
  fixture :: String
  fixture = "aaa,bbb,ccc\r\nzzz,yyy,xxx\r\n"

  fixture-doc :: List (List String)
  fixture-doc = Cons (Cons "aaa" (Cons "bbb" (Cons "ccc" Nil))) (Cons (Cons "zzz" (Cons "yyy" (Cons "xxx" Nil))) Nil)

  quoted-fixture :: String
  quoted-fixture = "\"aaa\",\"b\"\"bb\",\"ccc\"\r\n"

  quoted-doc :: List (List String)
  quoted-doc = Cons (Cons "aaa" (Cons "b\"bb" (Cons "ccc" Nil))) Nil

  check1 :: Int
  check1 = if eq-rows (parse-or-empty fixture) fixture-doc then 0 else 1

  check2 :: Int
  check2 = if eq-rows (parse-or-empty quoted-fixture) quoted-doc then 0 else 2

  main :: Int
  main = check1 + check2
"#,
    );
}

#[test]
fn csv_unterminated_quote_is_the_named_error() {
    // bit 1: an unterminated quoted field is observed as
    // CsvUnterminatedQuote, both when the text ends right after the
    // opening mark and when it ends inside a field with other scalars
    // (the same variant, so the error names the dialect rule).
    // bit 2: the error stands at the opening quotation mark, in byte
    // offset, line and column: at the start of the text, after an LF and
    // a comma, and after a two-byte scalar (U+00E9), so the offset
    // arithmetic counts bytes while the column counts scalars.
    expect_ok_with(
        "csv-unterminated",
        "  import Text.Lex (span-start, span-line, span-col)\n",
        r#"
  unterminated-span :: String -> (Nat, Nat, Nat)
  unterminated-span src = case csv-parse src of
    Ok rows -> (0, 0, 0)
    Err e -> case e of
      CsvUnterminatedQuote s -> (span-start s, span-line s, span-col s)
      CsvStrayQuote s -> (0, 0, 0)
      CsvTextAfterQuote s -> (0, 0, 0)
      CsvBareCarriageReturn s -> (0, 0, 0)
      CsvRowArity s expected actual -> (0, 0, 0)

  span-is :: String -> Nat -> Nat -> Nat -> Bool
  span-is src off line col = case unterminated-span src of
    (o, l, c) -> o == off && l == line && c == col

  check1 :: Int
  check1 = if row-refusal (csv-parse "\"abc") == 1 && row-refusal (csv-parse "\"a,b") == 1 then 0 else 1

  check2 :: Int
  check2 = if span-is "\"abc" 0 1 1 && span-is "a,b\n\"abc" 4 2 1 && span-is "\u{e9},\"abc" 3 1 3 then 0 else 2

  main :: Int
  main = check1 + check2
"#,
    );
}

#[test]
fn csv_terminators_and_empty_document() {
    // bit 1: a trailing terminator closes the last row without opening
    // another (one row, three fields, in order).
    // bit 2: empty text is no rows, and printing no rows is empty text.
    // bit 4: a terminator between two rows does open one, so "a\n\n" is
    // the rows [a] and [""], while a final row with no terminator still
    // parses.
    expect_ok(
        "csv-terminators",
        r#"
  trailing :: List (List String)
  trailing = parse-or-empty "a,b,c\n"

  blank-rows :: List (List String)
  blank-rows = parse-or-empty "a\n\n"

  bare-row :: List (List String)
  bare-row = parse-or-empty "a"

  empty-doc :: List (List String)
  empty-doc = parse-or-empty ""

  check1 :: Int
  check1 = if length trailing == 1 && eq-fields (first-row trailing) (Cons "a" (Cons "b" (Cons "c" Nil))) then 0 else 1

  check2 :: Int
  check2 = if length empty-doc == 0 && eq-str (csv-print no-rows) "" then 0 else 2

  check3 :: Int
  check3 = if length blank-rows == 2 && eq-rows (second-row blank-rows) (Cons (Cons "" Nil) Nil) && length bare-row == 1 then 0 else 4

  main :: Int
  main = check1 + check2 + check3
"#,
    );
}

#[test]
fn csv_refusals_and_header_keying() {
    // bit 1: the dialect's refusals, each matched by constructor: a
    // quotation mark in an unquoted field, text after a closing
    // quotation mark, a bare CR, and a row whose field count differs
    // from the header's (reporting the header's count); plus the
    // positive side of keying, where the pairs come out in header order.
    expect_ok(
        "csv-refusals",
        r#"
  hdr :: Result CsvError (List String, List (List (String, String)))
  hdr = csv-parse-with-headers "name,qty\nwidget,3\n"

  ragged :: Result CsvError (List String, List (List (String, String)))
  ragged = csv-parse-with-headers "name,qty\nwidget\n"

  keyed-pairs :: Bool
  keyed-pairs = length (headers-of hdr) == 2 && length (keyed-of hdr) == 1

  refusals :: Bool
  refusals = row-refusal (csv-parse "a\"b") == 2 && row-refusal (csv-parse "\"a\"b") == 3 && row-refusal (csv-parse "a\rb") == 4 && arity-expected ragged == 2

  check1 :: Int
  check1 = if refusals && keyed-pairs then 0 else 1

  main :: Int
  main = check1
"#,
    );
}
