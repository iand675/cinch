//! Continuation-passing code compiles to direct code
//! (design/cps-optimisation.md). Each test pins one of the five rules:
//!
//! 1. guaranteed tail calls through unknown closures (`tailcc`, the apply
//!    stub's tail twin and the `.app` prefix that finds it);
//! 2. contification of a local continuation into a join point;
//! 3. projection out of a known capturing dictionary or polytype bundle;
//! 4. call-pattern specialisation on a known unit, closure or record of
//!    functions (`specialise-arguments`);
//! 5. frame allocation of a continuation closure that a non-tail callee
//!    consumes.
//!
//! Every run is leak-checked: a rule that drops a release or releases twice
//! shows up as a nonzero live count or a trap, not a silent pass.

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

fn ir(tag: &str, source: &str, opts: &DriverOptions) -> String {
    let (dir, out) = compile(tag, source, Mode::EmitIr, opts);
    let text = std::fs::read_to_string(&out).expect("read the emitted IR");
    common::cleanup(&dir);
    text
}

/// The definitions of the program's own units (`Main.*`), header included.
fn program_units(text: &str) -> impl Iterator<Item = &str> {
    text.split("\ndefine ").filter(|d| d.lines().next().is_some_and(|header| header.contains("@\"Main.")))
}

fn run_checked(exe: &Path, tag: &str) {
    let out = Command::new(exe)
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "2G")
        .output()
        .expect("run the built program");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(42), "{tag}: exit status; stderr:\n{stderr}");
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "{tag}: every block must be released exactly once; stderr:\n{stderr}"
    );
}

fn build_and_run(tag: &str, source: &str, opts: &DriverOptions) {
    let (dir, exe) = compile(tag, source, Mode::Build, opts);
    run_checked(&exe, tag);
    common::cleanup(&dir);
}

fn at(level: u8) -> DriverOptions {
    DriverOptions { opt_level: Some(level), ..DriverOptions::default() }
}

/// A continuation-passing sum: every continuation tail-calls the one it
/// captured, through an unknown closure.
const SUM_K: &str = r#"module Main where
  build :: I64 -> List I64 -> List I64
  build n acc = if n == 0 then acc else build (wrapping-sub-i64 n 1) (Cons 1 acc)
  sum-k :: List I64 -> (I64 -> I64) -> I64
  sum-k xs k = case xs of
    Nil -> k 0
    Cons x rest -> sum-k rest (\s -> k (wrapping-add-i64 s x))
  main :: Int
  main = int-from-i64 (sum-k (build 2000000 Nil) (\s -> s)) - 1999958
"#;

#[test]
fn an_unknown_tail_call_is_guaranteed_without_llvm_optimisation() {
    // Two million continuations deep: without a guaranteed tail call each
    // `k (s + x)` keeps a native frame, and an unoptimised build exhausts
    // the stack long before the end. With the rule, `-O0` runs in constant
    // stack.
    build_and_run("cps-tail-o0", SUM_K, &at(0));
    let text = ir("cps-tail-ir", SUM_K, &DriverOptions::default());
    assert!(text.contains("musttail call tailcc"), "the unknown tail call is a musttail call");
    assert!(
        text.contains("prefix <{ i64, i64 }>"),
        "every apply stub carries the tail twin's address in its prefix"
    );
    assert!(text.contains("@cinch.tail.argv = internal thread_local global [16 x i64]"));
}

#[test]
fn a_tail_only_local_continuation_becomes_a_join_point() {
    let source = r#"module Main where
  classify :: I64 -> I64 -> I64
  classify n m =
    let k = \s -> wrapping-add-i64 (wrapping-mul-i64 s 3) m in
    if n == 0 then k 1 else if n == 1 then k 2 else k n
  go :: I64 -> I64 -> I64
  go i acc = if i == 0 then acc else go (wrapping-sub-i64 i 1) (wrapping-add-i64 acc (classify (if i > 50 then 0 else if i > 20 then 1 else 2) 3))
  main :: Int
  main = int-from-i64 (wrapping-sub-i64 (go 100 0) 708)
"#;
    build_and_run("cps-contify", source, &at(2));
    let text = ir("cps-contify-ir", source, &DriverOptions::default());
    assert!(!text.contains("Main.classify.lam"), "the continuation is a join point, not a closure");
    // The rule is what removed it: without it the closure survives.
    let off = ir("cps-contify-off", source, &DriverOptions { no_contify: true, ..DriverOptions::default() });
    assert!(off.contains("Main.classify.lam"), "without contification the closure is built");
}

#[test]
fn a_rank_two_continuation_folds() {
    let source = r#"module Main where
  data Cont a where
    Cont :: (forall r. (a -> r) -> r) -> Cont a
  pure-c :: I64 -> Cont I64
  pure-c x = Cont (\k -> k x)
  run :: Cont I64 -> I64
  run c = case c of Cont f -> f (\v -> wrapping-add-i64 v 1)
  main :: Int
  main = int-from-i64 (run (pure-c 41))
"#;
    build_and_run("cps-cont", source, &at(2));
    let text = ir("cps-cont-ir", source, &DriverOptions::default());
    for unit in program_units(&text) {
        assert!(!unit.contains("@__apply"), "the polytype's continuation is called directly:\n{unit}");
    }
}

/// A record of functions threaded unchanged down a loop: the
/// instrumentation idiom.
const HOOKS_LOOP: &str = r#"module Main where
  data Hooks where
    Hooks :: (I64 -> I64) -> (I64 -> I64) -> Hooks
  default-hooks :: Hooks
  default-hooks = Hooks (\x -> wrapping-add-i64 x 1) (\x -> x)
  run :: Hooks -> I64 -> I64 -> I64
  run h i acc = if i == 0 then acc else case h of
    Hooks before after -> run h (wrapping-sub-i64 i 1) (after (before acc))
  main :: Int
  main = int-from-i64 (wrapping-sub-i64 (run default-hooks 1000 42) 1000)
"#;

#[test]
fn a_record_of_functions_is_specialised_into_its_loop() {
    build_and_run("cps-hooks", HOOKS_LOOP, &at(2));
    let text = ir("cps-hooks-ir", HOOKS_LOOP, &DriverOptions::default());
    let run = text
        .split("\ndefine ")
        .find(|d| d.contains("@\"Main.run.sp0\"(") && d.contains('{'))
        .expect("the loop has a copy specialised to the static record");
    assert!(!run.contains("@__apply"), "the hooks are known in every iteration:\n{run}");
    assert!(!run.contains("tail.argv"), "no hook is called through a closure:\n{run}");
    let off = ir("cps-hooks-off", HOOKS_LOOP, &DriverOptions { no_specialise_arguments: true, ..DriverOptions::default() });
    assert!(!off.contains("Main.run.sp"), "without the rule no copy is made");
}

#[test]
fn a_record_built_at_the_call_site_is_specialised() {
    // The record is a constructor application over closures that capture a
    // local, not a static unit: the copy takes the captured word.
    let source = r#"module Main where
  data Hooks where
    Hooks :: (I64 -> I64) -> (I64 -> I64) -> Hooks
  run :: Hooks -> I64 -> I64 -> I64
  run h i acc = if i == 0 then acc else case h of
    Hooks before after -> run h (wrapping-sub-i64 i 1) (after (before acc))
  start :: I64 -> I64
  start d = run (Hooks (\x -> wrapping-add-i64 x d) (\x -> wrapping-sub-i64 x d)) 1000 42
  main :: Int
  main = int-from-i64 (start 7)
"#;
    build_and_run("cps-hooks-local", source, &at(2));
    let text = ir("cps-hooks-local-ir", source, &DriverOptions::default());
    assert!(text.contains("Main.run.sp"), "the call site's record shape has a copy");
}

#[test]
fn a_consumed_continuation_takes_a_frame_slot() {
    // `with-twice` calls its continuation once, not in tail position, so
    // the closure each iteration builds over the counted `b` can live in
    // `go`'s frame: the callee's apply stub releases it. Specialisation and
    // inlining are off so the closure survives to the frame rule.
    let source = r#"module Main where
  data Box where
    Box :: I64 -> Box
  with-twice :: (I64 -> I64) -> I64 -> I64
  with-twice k x = wrapping-add-i64 (k (wrapping-add-i64 x x)) 1
  -- A callee that calls its continuation in its own tail position: the
  -- apply runs in the callee's replaced frame, and the closure sits in the
  -- caller's, which is still live.
  with-tail :: (I64 -> I64) -> I64 -> I64
  with-tail k x = k (wrapping-add-i64 x 1)
  go :: Box -> I64 -> I64 -> I64
  go b i acc = if i == 0 then acc else
    let t = with-tail (\y -> case b of Box n -> wrapping-sub-i64 y n) 0 in
    go b (wrapping-sub-i64 i 1) (with-twice (\y -> case b of Box n -> wrapping-add-i64 (wrapping-add-i64 y acc) (wrapping-add-i64 n t)) 0)
  main :: Int
  main = int-from-i64 (wrapping-add-i64 (go (Box 1) 10 0) 22)
"#;
    let isolated = DriverOptions { no_specialise_arguments: true, no_inline: true, ..DriverOptions::default() };
    for level in [0, 2] {
        build_and_run(&format!("cps-frame-o{level}"), source, &DriverOptions { opt_level: Some(level), ..isolated.clone() });
    }
    let text = ir("cps-frame-ir", source, &isolated);
    let go = text
        .split("\ndefine ")
        .find(|d| d.contains("@\"Main.go\"(") && d.contains('{'))
        .expect("Main.go is emitted");
    // Header, code, arity, filled count and the saved words, rounded to an
    // even word count: one saved word for `with-tail`'s continuation (6
    // words), three for `with-twice`'s (8).
    assert!(
        go.contains("alloca [6 x i64]") && go.contains("alloca [8 x i64]"),
        "both continuations are frame blocks in go:\n{go}"
    );
    assert!(
        text.contains("call void @cinch_frame_release"),
        "an apply stub releases a frame closure's payload when it is called"
    );
    let heap = ir("cps-frame-heap", source, &DriverOptions { no_stack_alloc: true, ..isolated });
    assert!(!heap.contains("cinch_frame_release"), "--no-stack-alloc leaves every closure on the heap");
}
