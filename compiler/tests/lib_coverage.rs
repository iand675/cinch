//! Coverage recorder tests: `Effect.Coverage`, the recorder half of
//! Ruby's `Coverage` (design/stdlib-parity-ruby.md, Tooling).
//!
//! Each test compiles a real multi-file project: the module under test
//! (`Effect/Coverage.cinch`) is written beside `Main.cinch` in a fresh
//! temp project directory and imported explicitly (`import
//! Effect.Coverage`), driven through the CLI pipeline like modules.rs.
//! `Effect.Coverage` is a new family module, so it is NOT in the
//! embedded prelude: the source is written out here, and the modules it
//! builds on (`Data.Map`'s tree, `Text.Show`'s formatting) resolve to
//! the embedded prelude. The sources exercise only consumer-visible
//! semantics:
//!   accumulation: repeated `coverage-hit` calls at one site count up,
//!                a fresh site starts at one, and other sites are
//!                untouched;
//!   order:        `coverage-counts` is ascending by label no matter
//!                what order the hits arrived in, and the report text
//!                follows it;
//!   merge:        identity on both sides, associativity over sample
//!                recorders, keywise addition at an overlapping label,
//!                and commutativity;
//!   total:        `coverage-total` equals the sum of the counts
//!                `coverage-counts` reports;
//!   empty:        the empty recorder reports an empty counts list, a
//!                zero total, and a well-formed total-only report.
//!
//! Each check is a top-level `let` in `main` returning 0 or its bit;
//! `main` sums them, so a nonzero exit code is a bitmask naming the
//! failed checks (check k contributes bit 2^(k-1)) and any failure mode
//! of one check shares that bit, keeping the total inside the 8-bit
//! exit status. The helpers are top-level because a `case` in a `let`
//! binder's right-hand side must indent past the binder's column under
//! the layout rules.

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const COVERAGE: &str = include_str!("../prelude/Effect/Coverage.cinch");

/// Helpers shared by the check programs: structural equality on
/// `List (String, Nat)` (there is no tuple `Eq` instance, and the
/// element pair is `String` and `Nat`), and the counts sum the total
/// agreement check compares against.
const HELPERS: &str = r#"
  same :: List (String, Nat) -> List (String, Nat) -> Bool
  same a b = case a of
    Nil -> case b of
      Nil -> True
      Cons _ _ -> False
    Cons p ps -> case b of
      Nil -> False
      Cons q qs -> if eq-pair p q then same ps qs else False

  eq-pair :: (String, Nat) -> (String, Nat) -> Bool
  eq-pair p q = case p of
    (k1, v1) -> case q of
      (k2, v2) -> if k1 == k2 then v1 == v2 else False

  total-of :: List (String, Nat) -> Nat
  total-of rows = case rows of
    Nil -> 0
    Cons p ps -> case p of
      (site, n) -> n + total-of ps
"#;

/// The `Main` module of a check program: the explicit import of the
/// module under test, the shared helpers, then the program body.
fn main_module(body: &str) -> String {
    let mut src = String::from("module Main where\n\n  import Effect.Coverage\n\n");
    src.push_str(HELPERS);
    src.push('\n');
    src.push_str(body);
    src.push('\n');
    src
}

/// Write the module under test and `Main.cinch` into a fresh temp
/// project directory and run it through the real driver.
fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_covtest_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [("Coverage.cinch", COVERAGE), ("Main.cinch", main_src)] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    match driver(Mode::Run, &dir.join("Main.cinch"), None) {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_exit(name: &str, body: &str, expected: i32) {
    let code = run_main(name, &main_module(body));
    assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}");
}

// ------------------------------------------------------------ accumulation

#[test]
fn hit_accumulation_counts_up_and_orders_by_label() {
    // Hits arrive out of label order ("b" before "a") and "a" repeats;
    // the counts list is ascending anyway, the report follows it, and a
    // fresh site starts at one.
    expect_exit(
        "coverage-hits",
        r#"
  main :: Int
  main =
    let c0 = coverage-new in
    let c1 = coverage-hit "b" c0 in
    let c2 = coverage-hit "a" c1 in
    let c3 = coverage-hit "a" c2 in
    let c4 = coverage-hit "a" c3 in
    let c5 = coverage-hit "solo" c0 in
    let k1 = if same (coverage-counts c4) (Cons ("a", 3) (Cons ("b", 1) Nil)) then 0 else 1 in
    let k2 = if coverage-report c4 == "a\t3\nb\t1\ntotal\t4\n" then 0 else 2 in
    let k3 = if coverage-total c4 == 4 then 0 else 4 in
    let k4 = if same (coverage-counts c5) (Cons ("solo", 1) Nil) then 0 else 8 in
    let k5 = if same (coverage-counts c0) Nil then 0 else 16 in
    k1 + k2 + k3 + k4 + k5
"#,
        0,
    );
}

// ------------------------------------------------------------------ merge

#[test]
fn merge_identity_associativity_and_keywise_sum() {
    // p = {a:1, b:1}, q = {b:2, c:1}, r = {a:1}. The identity is the
    // empty recorder, the operation is associative, equal labels add
    // (b is 1 + 2), and it is commutative.
    expect_exit(
        "coverage-merge",
        r#"
  main :: Int
  main =
    let e = coverage-new in
    let p = coverage-hit "a" (coverage-hit "b" e) in
    let q = coverage-hit "c" (coverage-hit "b" (coverage-hit "b" e)) in
    let r = coverage-hit "a" e in
    let k1 = if same (coverage-counts (coverage-merge e p)) (coverage-counts p) then 0 else 1 in
    let k2 = if same (coverage-counts (coverage-merge p e)) (coverage-counts p) then 0 else 2 in
    let k3 = if same (coverage-counts (coverage-merge e e)) Nil then 0 else 4 in
    let k4 = if same (coverage-counts (coverage-merge (coverage-merge p q) r)) (coverage-counts (coverage-merge p (coverage-merge q r))) then 0 else 8 in
    let k5 = if same (coverage-counts (coverage-merge p q)) (Cons ("a", 1) (Cons ("b", 3) (Cons ("c", 1) Nil))) then 0 else 16 in
    let k6 = if same (coverage-counts (coverage-merge p q)) (coverage-counts (coverage-merge q p)) then 0 else 32 in
    let k7 = if same (coverage-counts (coverage-merge p p)) (Cons ("a", 2) (Cons ("b", 2) Nil)) then 0 else 64 in
    let k8 = if coverage-total (coverage-merge p q) == 5 then 0 else 128 in
    k1 + k2 + k3 + k4 + k5 + k6 + k7 + k8
"#,
        0,
    );
}

// ------------------------------------------------------------------ total

#[test]
fn total_agrees_with_the_sum_of_the_counts() {
    // For the empty recorder, a fresh hit, a repeated hit, and a merge,
    // `coverage-total` equals the sum of the counts the list reports.
    expect_exit(
        "coverage-total",
        r#"
  main :: Int
  main =
    let e = coverage-new in
    let one = coverage-hit "s" e in
    let many = coverage-hit "s" (coverage-hit "s" (coverage-hit "t" (coverage-hit "t" (coverage-hit "t" e)))) in
    let mixed = coverage-merge one many in
    let k1 = if coverage-total e == total-of (coverage-counts e) then 0 else 1 in
    let k2 = if coverage-total one == total-of (coverage-counts one) then 0 else 2 in
    let k3 = if coverage-total many == total-of (coverage-counts many) then 0 else 4 in
    let k4 = if coverage-total mixed == total-of (coverage-counts mixed) then 0 else 8 in
    let k5 = if coverage-total many == 5 then 0 else 16 in
    let k6 = if coverage-total mixed == 6 then 0 else 32 in
    k1 + k2 + k3 + k4 + k5 + k6
"#,
        0,
    );
}

// ------------------------------------------------------------------ empty

#[test]
fn empty_recorder_reports_are_well_formed() {
    // The empty recorder has no sites and a zero total; its report is
    // the total line alone. A one-site recorder's report is the site
    // line then the total line.
    expect_exit(
        "coverage-empty",
        r#"
  main :: Int
  main =
    let e = coverage-new in
    let solo = coverage-hit "solo" e in
    let k1 = if same (coverage-counts e) Nil then 0 else 1 in
    let k2 = if coverage-total e == 0 then 0 else 2 in
    let k3 = if coverage-report e == "total\t0\n" then 0 else 4 in
    let k4 = if coverage-report solo == "solo\t1\ntotal\t1\n" then 0 else 8 in
    let k5 = if coverage-report (coverage-merge e e) == "total\t0\n" then 0 else 16 in
    let k6 = if coverage-report (coverage-merge e solo) == "solo\t1\ntotal\t1\n" then 0 else 32 in
    k1 + k2 + k3 + k4 + k5 + k6
"#,
        0,
    );
}
