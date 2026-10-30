//! Runtime string literals (D43's deferral closed) and string
//! immutability: a `"..."` literal in an ordinary term is the runtime
//! `String`, one shared read-only data-section constant over the
//! immutable `String` representation.
//!
//! Decisions this suite pins (unnumbered, filed alongside the closing
//! slices):
//!
//! - `String` is IMMUTABLE: no API ever exposes its backing mutably.
//!   `string-to-bytes` copies out, `string-from-bytes` copies in, the
//!   representation constructor is module-private, and mutation
//!   exists only on plain `Bytes` (the prelude `Data.String` states
//!   the contract).
//! - A literal is a TRUE data-section constant: the block header and
//!   the bytes in one shared private constant, referenced directly
//!   (zero construction, zero allocation; the emit-ir smoke shows the
//!   constant). Immutability makes the sharing semantics-transparent.
//! - The JPL rule flips honestly: a literal is NOT an allocation, so
//!   `#[no-alloc]`/`jpl` functions may use literals; the alloc-site
//!   fact fires only where construction genuinely allocates.
//! - Literal syntax reuses the lexer's escape decoding exactly (`\\`,
//!   `\"`, `\'`, `\n`, `\r`, `\t`, `\u{...}`), and the runtime bytes
//!   are that decoded string in UTF-8.
//! - The literal is a constant: the D92 evaluator runs it and a
//!   `LitFold` boundary over literal strings folds like any other
//!   constant.
//! - Staging strings are unchanged (compile-time position keeps the
//!   staging interpreter's string values).
//! - `pack :: List Char -> String` / `unpack :: String -> List Char`
//!   (Haskell's `Data.String` pair) live in the type's home;
//!   `Show Bool`/`Show Ordering` render through literals.

use cinch::check::{check_program_multi_full, StagingCx};
use cinch::core::TargetProfile;
use cinch::driver::{
    check_sources, driver_with, parse_prelude, parse_source, run_source, Mode, StageMode,
};
use cinch::Result;
use std::collections::HashMap;
use std::path::PathBuf;

fn expect_ok(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_err(name: &str, src: &str, needles: &[&str]) {
    match check_sources(src, "<test>") {
        Ok(_) => panic!("{name}: expected an error containing {needles:?}, but the program checked"),
        Err(e) => {
            let msg = e.to_string();
            for n in needles {
                assert!(msg.contains(n), "{name}: error {msg:?} does not contain {n:?}");
            }
        }
    }
}

/// The staging report of a checked source under `profile`.
fn check_report_profile(src: &str, profile: &TargetProfile) -> Result<cinch::check::StageReport> {
    let prelude = parse_prelude(&[])?;
    let user = parse_source(src, "<test>", &prelude.base, &HashMap::new())?;
    let mut hashes = HashMap::new();
    let mut files = HashMap::new();
    let mut fixities = prelude.fixities.clone();
    for (m, f) in &user.fixities {
        fixities.insert(m.clone(), f.clone());
    }
    for m in &user.modules {
        hashes.insert(m.name.text.clone(), String::new());
        files.insert(m.name.text.clone(), "<test>".to_string());
    }
    let mut scx = StagingCx::default();
    scx.fixities = fixities;
    scx.files = files;
    scx.entry_modules = user.modules.iter().map(|m| m.name.text.clone()).collect();
    let (_program, _ifaces, report, _diags) =
        check_program_multi_full(&prelude.modules, &user.modules, &hashes, &scx, &[], profile)?;
    Ok(report)
}

/// A temp project directory holding the given `(file, source)` pairs.
fn write_project(files: &[(&str, &str)]) -> PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "cinch_string_literals_{}_{}",
        std::process::id(),
        n
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (name, src) in files {
        std::fs::write(dir.join(name), src).expect("write file");
    }
    dir
}

// ---------------------------------------------------------- type and running

/// `"hello"` has type `String`: the ascription checks and the
/// concatenation and length probes run end to end. A literal is not a
/// `Bytes` value (the buffer is behind the copying `string-to-bytes`).
#[test]
fn literal_has_string_type_and_round_trips() {
    expect_ok(
        "round-trip",
        r#"
module Main where
  greeting :: String
  greeting = "hello"

  cat :: String
  cat = string-concat greeting " world"

  bytes-probe :: Int
  bytes-probe =
    let b = string-to-bytes cat in
    if bytes-len b == 11 then
      if bytes-get b 0 == 104 then
        if bytes-get b 10 == 100 then 0 else 3
      else 2
    else 1

  lengths-probe :: Int
  lengths-probe =
    if string-byte-length cat == 11 then
      if string-length cat == 11 then 0 else 5
    else 4

  main :: Int
  main = bytes-probe + lengths-probe
"#,
        0,
    );
    expect_err(
        "literal-is-not-bytes",
        "module Main where\n  b :: Bytes\n  b = \"hello\"\n  main :: Int\n  main = 0\n",
        &["type mismatch", "String", "Bytes"],
    );
}

// ------------------------------------------------------------------- escapes

/// A probe per escape class the lexer supports: the runtime bytes are
/// the lexer's decoded string in UTF-8 (`\\`, `\"`, `\'`, `\n`, `\r`,
/// `\t`, `\u{...}`), and a raw multi-byte character agrees with its
/// escape spelling.
#[test]
fn escapes_match_the_lexer() {
    expect_ok(
        "escapes",
        r#"
module Main where
  probe-slash :: Int
  probe-slash =
    let b = string-to-bytes "a\\b" in
    if bytes-len b == 3 then
      if bytes-get b 1 == 92 then 0 else 2
    else 1

  probe-quote :: Int
  probe-quote =
    let b = string-to-bytes "a\"b" in
    if bytes-len b == 3 then
      if bytes-get b 1 == 34 then 0 else 2
    else 1

  probe-tick :: Int
  probe-tick =
    let b = string-to-bytes "a\'b" in
    if bytes-len b == 3 then
      if bytes-get b 1 == 39 then 0 else 2
    else 1

  probe-newline :: Int
  probe-newline =
    let b = string-to-bytes "a\nb" in
    if bytes-len b == 3 then
      if bytes-get b 1 == 10 then 0 else 2
    else 1

  probe-return :: Int
  probe-return =
    let b = string-to-bytes "a\rb" in
    if bytes-len b == 3 then
      if bytes-get b 1 == 13 then 0 else 2
    else 1

  probe-tab :: Int
  probe-tab =
    let b = string-to-bytes "a\tb" in
    if bytes-len b == 3 then
      if bytes-get b 1 == 9 then 0 else 2
    else 1

  probe-unicode :: Int
  probe-unicode =
    let b = string-to-bytes "\u{1F600}" in
    if bytes-len b == 4 then
      if bytes-get b 0 == 240 then
        if bytes-get b 1 == 159 then
          if bytes-get b 2 == 152 then
            if bytes-get b 3 == 128 then 0 else 4
          else 4
        else 4
      else 4
    else 4

  probe-raw-utf8 :: Int
  probe-raw-utf8 =
    let b = string-to-bytes "\u{E9}é" in
    if bytes-len b == 4 then
      if bytes-get b 0 == 195 then
        if bytes-get b 1 == 169 then
          if bytes-get b 2 == 195 then
            if bytes-get b 3 == 169 then 0 else 4
          else 4
        else 4
      else 4
    else 4

  main :: Int
  main = probe-slash + probe-quote + probe-tick + probe-newline + probe-return + probe-tab + probe-unicode + probe-raw-utf8
"#,
        0,
    );
}

/// The rejections match the lexer exactly: a raw newline (or the end of
/// input) inside a literal is unterminated, an unknown escape and a
/// non-scalar `\u{...}` are refused.
#[test]
fn lexer_rejections_match_the_lexer() {
    expect_err(
        "raw-newline",
        "module Main where\n  s :: String\n  s = \"a\nb\"\n  main :: Int\n  main = 0\n",
        &["unterminated string literal"],
    );
    expect_err(
        "unknown-escape",
        "module Main where\n  s :: String\n  s = \"a\\qb\"\n  main :: Int\n  main = 0\n",
        &["unknown escape sequence"],
    );
    expect_err(
        "non-scalar-escape",
        "module Main where\n  s :: String\n  s = \"\\u{110000}\"\n  main :: Int\n  main = 0\n",
        &["is not a Unicode scalar value"],
    );
}

// ------------------------------------------------------- equality and opacity

/// Literal equality stays byte semantics (`Eq String`/`Ord String` over
/// `string-compare`). The type is OPAQUE: the representation
/// constructor is module-private (immutability), and a literal pattern
/// stays the parser's named refusal in the bootstrap slice.
#[test]
fn literal_equality_stays_byte_semantics() {
    expect_ok(
        "equality",
        r#"
module Main where
  eq-probe :: Int
  eq-probe =
    if "abc" == "abc" then
      if "abc" == "abd" then 2
      else if "a" < "b" then
        if "b" > "a" then
          if string-compare "a" "a" == 0 then 0 else 5
        else 4
      else 3
    else 1

  main :: Int
  main = eq-probe
"#,
        0,
    );
    expect_err(
        "literal-patterns-are-a-named-refusal",
        "module Main where\n  f :: String -> Int\n  f s = case s of\n    \"a\" -> 0\n    _ -> 1\n  main :: Int\n  main = 0\n",
        &["literal patterns are not supported in the bootstrap slice"],
    );
    expect_err(
        "the-representation-constructor-is-private",
        "module Main where\n  f :: String -> Int\n  f s = case s of\n    String b -> 1\n  main :: Int\n  main = 0\n",
        &["String"],
    );
}

// ------------------------------------------------------------- constant fold

/// The literal is a constant: a collection literal over string
/// literals folds at the `LitFold` boundary to constant data (the
/// observable the boundary reports) while a construction over a
/// runtime value keeps its `from-list` call, and the folded constant
/// runs correctly.
#[test]
fn constants_fold_at_the_litfold_boundary() {
    let src = r#"
module Main where
  names :: List String
  names = ["a", "b"]

  pick :: String -> List String
  pick s = [s]

  probe-fresh :: Int
  probe-fresh = case names of
    Cons x rest ->
      if string-byte-length x == 1 then
        if length rest == 1 then 0 else 2
      else 1
    Nil -> 3

  probe-runtime :: Int
  probe-runtime = if length (pick "c") == 1 then 0 else 4

  main :: Int
  main = probe-fresh + probe-runtime
"#;
    expect_ok("folded-list-runs", src, 0);
    let report = check_report_profile(src, &TargetProfile::native()).expect("checks");
    let folds: Vec<(String, bool)> = report
        .literal_folds
        .iter()
        .map(|f| (f.kind.clone(), f.folded))
        .collect();
    assert_eq!(
        folds,
        vec![("list".to_string(), true), ("list".to_string(), false)],
        "the literal-string list folds to a constant and the runtime one keeps its class call"
    );
}

// -------------------------------------------------------------- staging side

/// Staging strings are unchanged: a literal in compile-time position
/// keeps the staging interpreter's string value and its consumers
/// (`strNat`) run as before.
#[test]
fn staging_strings_are_unchanged() {
    expect_ok(
        "staging-strNat",
        r#"
module Main where
  stage greet :: Nat -> Nat
  stage greet n = strNat "42"

  generate do
    emit (quoteDecls where
      answer :: Nat
      answer = $(quoteNat (greet 1)))

  main :: Int
  main = nat-to-int answer
"#,
        42,
    );
}

// ----------------------------------------------------------- JPL no-allocation

/// The chosen rule, pinned: a string literal is a shared read-only
/// constant (no construction), so it is NOT an allocation. A
/// `#[no-alloc]` function may use literals in ordinary terms (and in
/// staging terms), and the `jpl` profile's rule 3 reports zero
/// allocation sites; the alloc-site fact fires only where construction
/// genuinely allocates (e.g. `string-from-bytes`'s copy).
#[test]
fn jpl_no_allocation_treatment_pinned() {
    let report = check_report_profile(
        r#"
module Main where
  #[no-alloc]
  greet :: String
  greet = "hello"

  #[no-alloc]
  shout :: String -> String
  shout s = string-concat s "!"

  main :: Int
  main = 0
"#,
        &TargetProfile::native(),
    )
    .expect("no-alloc accepts literals");
    for def in ["Main.greet", "Main.shout"] {
        let row = report
            .safety
            .assertions
            .iter()
            .find(|a| a.definition == def && a.attribute == "no-alloc")
            .unwrap_or_else(|| panic!("no no-alloc row for {def}: {:?}", report.safety.assertions));
        assert!(
            row.result.contains("verified (0 allocation site(s)"),
            "{def}: {:?}",
            row.result
        );
    }
    let jpl = check_report_profile(
        r#"
module Main where
  pick :: Int -> String -> String
  pick x s = case decide (x < 5) of
    Yes _ -> case decide (x < 3) of
      Yes _ -> "hello"
      No _ -> s
    No _ -> s
"#,
        &TargetProfile::jpl(),
    )
    .expect("the jpl profile accepts literals (rule 3: no allocation)");
    let row = jpl
        .safety
        .assertions
        .iter()
        .find(|a| a.definition == "Main.pick" && a.attribute == "jpl alloc=none")
        .expect("the jpl alloc row");
    assert!(
        row.result.contains("verified (0 allocation site(s)"),
        "{:?}",
        row.result
    );
}

// -------------------------------------------------------------- immutability

/// The immutability contract, pinned both ways: mutation through the
/// copied-out `Bytes` is never observed through the `String`
/// (`string-to-bytes` copies), and a buffer wrapped by
/// `string-from-bytes` can be mutated afterwards without changing the
/// string (the conversion copies in).
#[test]
fn string_backing_is_never_mutable_through_the_api() {
    expect_ok(
        "copy-discipline",
        r#"
module Main where
  copy-out :: Int
  copy-out =
    let s = "ab" in
    let b = string-to-bytes s in
    -- "ab" is two bytes; the length checks prove the write and the read
    -- (a wrong length is its own failure code, never skipped silently).
    if bytes-len b == 2 then
      let u = bytes-set b 0 120 in
      if string-compare s "ab" == 0 then
        let c = string-to-bytes s in
        if bytes-len c == 2 then
          if bytes-get c 0 == 97 then 0 else 2
        else 4
      else 1
    else 4

  copy-in :: Int
  copy-in =
    let b = bytes-new 2 in
    let u1 = bytes-set b 0 97 in
    let u2 = bytes-set b 1 98 in
    case string-from-bytes b of
      None -> 5
      Some s ->
        let u3 = bytes-set b 0 120 in
        if string-compare s "ab" == 0 then 0 else 3

  main :: Int
  main = copy-out + copy-in
"#,
        0,
    );
}

// --------------------------------------------------------------- pack/unpack

/// `pack :: List Char -> String` and `unpack :: String -> List Char`
/// (Haskell's `Data.String` pair, in the type's home): content, both
/// round trips, and the multi-byte case.
#[test]
fn pack_unpack_round_trip() {
    expect_ok(
        "pack-unpack",
        r#"
module Main where
  pack-probe :: Int
  pack-probe =
    if string-compare (pack (Cons 'h' (Cons '\u{A1}' Nil))) "h\u{A1}" == 0 then
      if string-byte-length (pack (Cons '\u{1F600}' Nil)) == 4 then
        if string-byte-length (pack Nil) == 0 then 0 else 3
      else 2
    else 1

  unpack-probe :: Int
  unpack-probe = case unpack "h\u{A1}" of
    Cons a rest -> case rest of
      Cons b rest2 -> case rest2 of
        Nil ->
          if eq-u32 (char-to-u32 a) 104 then
            if eq-u32 (char-to-u32 b) 161 then 0 else 3
          else 2
        Cons _ _ -> 4
      Nil -> 5
    Nil -> 1

  round-trip :: Int
  round-trip =
    if string-compare (pack (unpack "h\u{A1}\u{1F600}")) "h\u{A1}\u{1F600}" == 0 then
      if string-length (pack (unpack "h\u{A1}\u{1F600}")) == 3 then 0 else 2
    else 1

  main :: Int
  main = pack-probe + unpack-probe + round-trip
"#,
        0,
    );
}

// --------------------------------------------------------------- Show family

/// `Show Bool` and `Show Ordering` render their fixed text through
/// literals.
#[test]
fn show_renders_bool_and_ordering() {
    expect_ok(
        "show-bool-ordering",
        r#"
module Main where
  import Text.Show

  main :: Int
  main =
    if show True == "True" then
      if show False == "False" then
        if show LT == "LT" then
          if show EQ == "EQ" then
            if show GT == "GT" then 0 else 5
          else 4
        else 3
      else 2
    else 1
"#,
        0,
    );
}

// ----------------------------------------------------------------- IR smoke

/// `emit-ir` a program whose literal site runs twice and run it: the
/// literal object is a shared read-only constant (the block header and
/// the bytes) referenced directly (no materialisation call), over a
/// `Bytes` round-trip probe. The literal site compiles to one constant
/// in its unit and every evaluation references it; inlining copies
/// sites like any constant, and the sharing is unobservable precisely
/// because `String` is immutable (the mutation probes pin that
/// separately).
#[test]
fn emit_ir_references_the_constant_directly() {
    let dir = write_project(&[(
        "Main.cinch",
        "module Main where\n\
         \x20 greet :: Nat -> String\n\
         \x20 greet n = \"hello\"\n\
         \x20 main :: Int\n\
         \x20 main =\n\
         \x20   let a = string-to-bytes (greet 0) in\n\
         \x20   let b = string-to-bytes (greet 1) in\n\
         \x20   if bytes-len a == 5 && bytes-len b == 5 then\n\
         \x20     if bytes-get b 4 == 111 then 0 else 2\n\
         \x20   else 1\n",
    )]);
    let ir_path = dir.join("out.ll");
    driver_with(
        Mode::EmitIr,
        StageMode::Refresh,
        &dir.join("Main.cinch"),
        Some(&ir_path),
    )
    .expect("emit-ir");
    let ir = std::fs::read_to_string(&ir_path).expect("ir text");
    let lit_lines: Vec<&str> = ir.lines().filter(|l| l.contains("c\"hello\"")).collect();
    assert!(!lit_lines.is_empty(), "the literal emits a constant");
    // The site compiles to one shared constant in its unit (a second
    // evaluation of the site never constructs another).
    let site_lines: Vec<&str> = lit_lines
        .iter()
        .copied()
        .filter(|l| l.contains("Main.greet"))
        .collect();
    assert_eq!(
        site_lines.len(),
        1,
        "one constant for the literal site: {site_lines:?}"
    );
    for l in &lit_lines {
        assert!(
            l.contains("internal constant { i64, ptr, i64, i64, ptr,"),
            "the constant is the composite object (two headered blocks + bytes): {l}"
        );
        assert!(!l.contains("cinch_bytes_from_data"), "no materialisation: {l}");
    }
    let name = site_lines[0]
        .split_once(' ')
        .map(|(n, _)| n.trim_start_matches('@').to_string())
        .expect("constant name");
    // The value word points one header word into the composite.
    assert!(
        ir.contains(&format!("ptr @{name}, i64 0, i32 1) to i64")),
        "the constant is referenced directly"
    );
    let code = driver_with(Mode::Run, StageMode::Refresh, &dir.join("Main.cinch"), None)
        .expect("run");
    assert_eq!(code, 0, "the emitted program runs");
}
