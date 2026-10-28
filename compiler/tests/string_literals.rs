//! Runtime string literals (D43's deferral closed): a `"..."` literal
//! in an ordinary term is the runtime `String`, constant UTF-8 data
//! over the D89 representation (`String` wraps a `Bytes` buffer and
//! the literal's decoded bytes are the private constant, the same
//! constant form the `import data` converters bind).
//!
//! Decisions this suite pins (unnumbered, filed alongside the closing
//! slices):
//!
//! - Literal syntax reuses the lexer's escape decoding exactly (`\\`,
//!   `\"`, `\'`, `\n`, `\r`, `\t`, `\u{...}`), and the runtime bytes
//!   are that decoded string in UTF-8.
//! - The literal is a constant: it elaborates directly to the
//!   constant form, so the D92 evaluator runs it and a `LitFold`
//!   boundary over literal strings folds like any other constant.
//! - Each evaluation constructs a fresh `Bytes` buffer (the D89 buffer
//!   is mutable in place and `String` shares it, so no static sharing),
//!   so the literal allocates where it runs: the `#[no-alloc]`/`jpl`
//!   alloc-site fact names it. This is the honest treatment, not a
//!   static constant exemption.
//! - Staging strings are unchanged: in compile-time position the
//!   literal keeps the staging interpreter's string value.
//! - `Show Bool`/`Show Ordering` render their fixed text through
//!   literals (the prelude waits that this closes).

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

fn expect_err_jpl(name: &str, src: &str, needles: &[&str]) {
    match check_report_profile(src, &TargetProfile::jpl()) {
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
fn check_report_profile(
    src: &str,
    profile: &TargetProfile,
) -> Result<cinch::check::StageReport> {
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
/// `Bytes` value (the buffer lives behind `string-to-bytes`).
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

// ------------------------------------------------------- equality and patterns

/// Literal equality (`Eq String` over `string-compare`) and literal
/// pattern use where the type supports it: the `String` constructor
/// pattern over a literal scrutinee. A literal PATTERN stays the
/// parser's named refusal in the bootstrap slice (the boundary this
/// pins).
#[test]
fn literal_equality_and_pattern_use() {
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

  pattern-probe :: Int
  pattern-probe = case "hi" of
    String b -> if bytes-len b == 2 then 0 else 2

  main :: Int
  main = eq-probe + pattern-probe
"#,
        0,
    );
    expect_err(
        "literal-patterns-are-a-named-refusal",
        "module Main where\n  f :: String -> Int\n  f s = case s of\n    \"a\" -> 0\n    _ -> 1\n  main :: Int\n  main = 0\n",
        &["literal patterns are not supported in the bootstrap slice"],
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
    Cons x rest -> case x of
      String b ->
        if bytes-len b == 1 then
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
  main = int-from-i64 (nat-to-i64 answer)
"#,
        42,
    );
}

// ----------------------------------------------------------- JPL no-allocation

/// The chosen rule, pinned: the literal is constant data whose
/// evaluation constructs a fresh `Bytes` buffer, so it allocates where
/// it runs and the alloc-site fact fires under `#[no-alloc]` and under
/// the `jpl` profile (it is NOT a static constant exempt from rule 3).
#[test]
fn jpl_no_allocation_treatment_pinned() {
    expect_err(
        "no-alloc-runtime-literal",
        r#"
module Main where
  #[no-alloc]
  greet :: String
  greet = "hello"

  main :: Int
  main = 0
"#,
        &[
            "`#[no-alloc]` on Main.greet",
            "no-allocation rule (JPL rule 3)",
            "a string literal is constructed here",
        ],
    );
    expect_err_jpl(
        "jpl-runtime-literal",
        r#"
module Main where
  pick :: Int -> String -> String
  pick x s = case decide (x < 5) of
    Yes _ -> case decide (x < 3) of
      Yes _ -> "hello"
      No _ -> s
    No _ -> s
"#,
        &[
            "the `jpl` profile admits no allocation outside initialization",
            "a string literal is constructed here",
            "no allocation site may run per call",
        ],
    );
}

/// The construction is per evaluation: two calls over the same literal
/// site get distinct `Bytes` buffers, so a mutation through one never
/// leaks into the other (the rule's semantic half: no static sharing).
#[test]
fn literal_evaluation_constructs_a_fresh_buffer() {
    expect_ok(
        "fresh-per-evaluation",
        r#"
module Main where
  poke-or-read :: Nat -> Nat
  poke-or-read n = case "ab" of
    String b ->
      if n == 0 then
        let u = bytes-set b 0 120 in 0
      else bytes-get b 0

  main :: Int
  main =
    let x = poke-or-read 0 in
    let y = poke-or-read 1 in
    if x == 0 then
      if y == 97 then 0 else 2
    else 1
"#,
        0,
    );
}

// --------------------------------------------------------------- Show family

/// `Show Bool` and `Show Ordering` are unblocked: the fixed text
/// renders through literals.
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

/// `emit-ir` a program using a literal and run it: the literal bytes
/// are a private constant and the runtime materialises them through
/// the constant path, over a `Bytes` round-trip probe.
#[test]
fn emit_ir_materialises_constant_data_and_the_program_runs() {
    let dir = write_project(&[(
        "Main.cinch",
        "module Main where\n\
         \x20 greeting :: String\n\
         \x20 greeting = \"hello\"\n\
         \x20 main :: Int\n\
         \x20 main =\n\
         \x20   let b = string-to-bytes greeting in\n\
         \x20   if bytes-len b == 5 then\n\
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
    assert!(
        ir.contains("cinch_bytes_from_data"),
        "the literal materialises through the constant path"
    );
    assert!(
        ir.contains("c\"hello\""),
        "the literal bytes are a private constant"
    );
    let code = driver_with(Mode::Run, StageMode::Refresh, &dir.join("Main.cinch"), None)
        .expect("run");
    assert_eq!(code, 0, "the emitted program runs");
}
