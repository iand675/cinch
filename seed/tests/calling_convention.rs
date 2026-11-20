//! The internal calling convention (design/cps-optimisation.md, "The
//! internal convention" and "Multi-word results").
//!
//! On a 64-bit native target the two ends of every guaranteed tail call run
//! under LLVM's `tailcc` as `<unit>#cc`, behind a C wrapper under the unit's
//! own symbol. Calls inside the module go to the body, an unknown tail call
//! passes its first words in registers to the callee's apply twin, and a
//! unit whose every return path builds one small constructor returns its
//! fields in registers instead of a block.
//!
//! Every run is leak-checked: a vanished block whose fields were counted
//! wrongly shows up as a nonzero live count or a trap.

mod common;
use cinch::driver::{DriverOptions, Mode};
use std::path::{Path, PathBuf};
use std::process::Command;

fn compile(tag: &str, source: &str, mode: Mode, opts: &DriverOptions) -> (PathBuf, PathBuf) {
    let dir = common::project(tag, &[("Main.cinch", source)]);
    let out = dir.join(if matches!(mode, Mode::EmitIr) { "main.ll" } else { "main" });
    let code = common::drive_opts(mode, &dir, "Main.cinch", Some(out.clone()), opts).unwrap_or_else(|e| panic!("{tag}: {e}"));
    assert_eq!(code, 0, "{tag}: compile");
    (dir, out)
}

fn ir(tag: &str, source: &str) -> String {
    ir_opts(tag, source, &DriverOptions::default())
}

fn ir_opts(tag: &str, source: &str, opts: &DriverOptions) -> String {
    let (dir, out) = compile(tag, source, Mode::EmitIr, opts);
    let text = std::fs::read_to_string(&out).expect("read the emitted IR");
    common::cleanup(&dir);
    text
}

/// Run `exe` leak-checked with allocation statistics; answer the number of
/// blocks it allocated.
fn run_checked(exe: &Path, tag: &str) -> u64 {
    let out = Command::new(exe)
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_ALLOC_STATS", "1")
        .env("CINCH_HEAP_LIMIT", "2G")
        .output()
        .expect("run the built program");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(42), "{tag}: exit status; stderr:\n{stderr}");
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "{tag}: every block must be released exactly once; stderr:\n{stderr}"
    );
    let blocks = stderr
        .split_whitespace()
        .find_map(|w| w.strip_prefix("blocks="))
        .unwrap_or_else(|| panic!("{tag}: no allocation statistics; stderr:\n{stderr}"));
    blocks.parse().expect("a block count")
}

fn build_and_run(tag: &str, source: &str, level: u8) -> u64 {
    build_and_run_opts(tag, source, &DriverOptions { opt_level: Some(level), ..DriverOptions::default() })
}

fn build_and_run_opts(tag: &str, source: &str, opts: &DriverOptions) -> u64 {
    let (dir, exe) = compile(tag, source, Mode::Build, opts);
    let blocks = run_checked(&exe, tag);
    common::cleanup(&dir);
    blocks
}

fn native_64() -> bool {
    cfg!(target_pointer_width = "64") && cfg!(any(target_arch = "x86_64", target_arch = "aarch64"))
}

/// A continuation of six parameters, tail-called a million deep through
/// an unknown closure. Six words is more than either target passes in
/// registers to the apply twin, so the last ones travel in the
/// thread-local buffer; `-O0` keeps no sibling-call rescue.
const WIDE_K: &str = r#"module Main where
  step :: I64 -> (I64 -> I64 -> I64 -> I64 -> I64 -> I64 -> I64) -> I64
  step n k = if n == 0 then k 1 2 3 4 5 6 else step (wrapping-sub-i64 n 1) (\a b c d e f -> k (wrapping-add-i64 a 1) b c d e (wrapping-add-i64 f 1))
  main :: Int
  main = int-from-i64 (wrapping-sub-i64 (step 1000000 (\a b c d e f -> wrapping-add-i64 a f)) 1999965)
"#;

#[test]
fn an_unknown_tail_call_wider_than_the_registers_is_guaranteed() {
    build_and_run("cc-wide-k-o0", WIDE_K, 0);
    build_and_run("cc-wide-k-o2", WIDE_K, 2);
    if native_64() {
        let text = ir("cc-wide-k-ir", WIDE_K);
        // One twin signature for every count: the closure, the count and
        // the register words.
        assert!(text.contains(".tapp\"(ptr %self, i64 %argc, i64 %a0"), "{text}");
        assert!(text.contains("musttail call tailcc i64 %"), "the unknown tail call is a musttail call");
    }
}

/// Every return path of `split` builds a pair, so its body returns the
/// pair's two words, and the recursive call and `main` take it apart
/// without a block.
const SPLIT: &str = r#"module Main where
  split :: List I64 -> (List I64, List I64)
  split xs = case xs of
    Nil -> (Nil, Nil)
    Cons x rest -> case split rest of
      (a, b) -> (Cons x b, a)
  len :: List I64 -> I64 -> I64
  len xs acc = case xs of
    Nil -> acc
    Cons _ r -> len r (wrapping-add-i64 acc 1)
  build :: I64 -> List I64 -> List I64
  build n acc = if n == 0 then acc else build (wrapping-sub-i64 n 1) (Cons n acc)
  main :: Int
  main = case split (build 1000 Nil) of
    (a, b) -> if wrapping-add-i64 (len a 0) (len b 0) == 1000 then 42 else 1
"#;

#[test]
fn a_product_result_taken_apart_allocates_no_block() {
    for level in [0, 2] {
        let blocks = build_and_run(&format!("cc-split-o{level}"), SPLIT, level);
        if native_64() {
            // 1000 list cells and 1000 split cells; boxed pairs would add
            // another 1001.
            assert!(blocks < 2100, "-O{level}: the pairs are never built ({blocks} blocks)");
        }
    }
    if native_64() {
        let text = ir("cc-split-ir", SPLIT);
        // `split` makes no tail call, so its body keeps the C convention
        // and only its result changes.
        assert!(text.contains("define { i64, i64 } @\"Main.split#cc\"(i64 %p0)"), "{text}");
        // The C symbol is unchanged and boxes the result.
        assert!(text.contains("define i64 @\"Main.split\"(i64 %p0)"), "{text}");
    }
}

/// Products that escape: a triple stored in a list (boxed by the direct
/// call's wrapper), one reached through a closure (boxed by the apply
/// stub), one built by reusing its argument's cell, and a product result
/// tail-calling another. Each field is counted, so a lost or doubled
/// count shows up in the leak check.
const ESCAPES: &str = r#"module Main where
  build :: I64 -> List I64 -> List I64
  build n acc = if n == 0 then acc else build (wrapping-sub-i64 n 1) (Cons n acc)
  len :: List I64 -> I64 -> I64
  len xs acc = case xs of
    Nil -> acc
    Cons _ r -> len r (wrapping-add-i64 acc 1)
  spread :: I64 -> (List I64, I64, List I64)
  spread n = (build n Nil, n, build (wrapping-add-i64 n 1) Nil)
  swap :: (List I64, I64, List I64) -> (List I64, I64, List I64)
  swap t = case t of
    (a, n, b) -> (b, n, a)
  twice :: I64 -> (List I64, I64, List I64)
  twice n = swap (spread n)
  firsts :: List I64 -> List (List I64, I64, List I64)
  firsts xs = case xs of
    Nil -> Nil
    Cons x r -> Cons (twice x) (firsts r)
  total :: List (List I64, I64, List I64) -> I64 -> I64
  total ps acc = case ps of
    Nil -> acc
    Cons p r -> case p of
      (a, n, b) -> total r (wrapping-add-i64 acc (wrapping-add-i64 (len a 0) (wrapping-add-i64 n (len b 0))))
  apply-all :: (I64 -> (List I64, I64, List I64)) -> I64 -> I64 -> I64
  apply-all f i acc = if i == 0 then acc else case f i of
    (a, n, b) -> apply-all f (wrapping-sub-i64 i 1) (wrapping-add-i64 acc (wrapping-add-i64 (len a 0) (len b 0)))
  main :: Int
  main = if wrapping-add-i64 (total (firsts (build 10 Nil)) 0) (apply-all spread 10 0) == 295 then 42 else 1
"#;

#[test]
fn an_escaping_product_result_is_boxed_and_counted_once() {
    build_and_run("cc-escapes-o0", ESCAPES, 0);
    build_and_run("cc-escapes-o2", ESCAPES, 2);
    // Without inlining and argument specialisation `apply-all` reaches
    // `spread` through its closure, whose apply stub boxes the result.
    let plain = DriverOptions { no_inline: true, no_specialise_arguments: true, ..DriverOptions::default() };
    for level in [0, 2] {
        build_and_run_opts(&format!("cc-escapes-plain-o{level}"), ESCAPES, &DriverOptions { opt_level: Some(level), ..plain.clone() });
    }
    if native_64() {
        let text = ir("cc-escapes-ir", ESCAPES);
        assert!(text.contains("{ i64, i64, i64 } @\"Main.spread#cc\"("), "{text}");
        assert!(text.contains("{ i64, i64, i64 } @\"Main.twice#cc\"("), "{text}");
        let text = ir_opts("cc-escapes-plain-ir", ESCAPES, &plain);
        assert!(text.contains("define i64 @\"Main.spread.app\""), "{text}");
    }
}

/// Two product results tail-calling each other a million deep (the
/// group runs as one loop), and `start`, a product result whose one leaf
/// tail-calls another. No pair is built until `main` takes the last one
/// apart.
const PRODUCT_TAILS: &str = r#"module Main where
  ping :: I64 -> I64 -> (I64, I64)
  ping n acc = if n == 0 then (acc, n) else pong (wrapping-sub-i64 n 1) (wrapping-add-i64 acc 1)
  pong :: I64 -> I64 -> (I64, I64)
  pong n acc = if n == 0 then (acc, 0) else ping (wrapping-sub-i64 n 1) (wrapping-add-i64 acc 2)
  count-down :: I64 -> I64 -> (I64, I64)
  count-down n acc = if n == 0 then (acc, n) else count-down (wrapping-sub-i64 n 1) (wrapping-add-i64 acc 3)
  start :: I64 -> (I64, I64)
  start n = count-down n 0
  main :: Int
  main = case ping 1000000 0 of
    (acc, _) -> case start 1000 of
      (c, _) -> if wrapping-add-i64 acc c == 1503000 then 42 else 1
"#;

#[test]
fn product_results_tail_call_in_constant_stack() {
    for level in [0, 2] {
        let blocks = build_and_run(&format!("cc-product-tails-o{level}"), PRODUCT_TAILS, level);
        if native_64() {
            assert!(blocks < 16, "-O{level}: no pair is ever built ({blocks} blocks)");
        }
    }
    if native_64() {
        let text = ir("cc-product-tails-ir", PRODUCT_TAILS);
        assert!(text.contains("{ i64, i64 } @\"Main.count-down#cc\"("), "{text}");
        assert!(text.contains("{ i64, i64 } @\"Main.ping#cc\"("), "{text}");
    }
    // The cross-unit `musttail` of the aggregate is pinned on hand-built
    // Core (`codegen::tests::a_product_result_tail_call_passes_the_aggregate_on`):
    // the elaborator folds a one-call wrapper such as `start` into its
    // caller and a mutual group into one loop, so a surface program does
    // not keep that edge.
}
