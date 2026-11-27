//! Direct calls on saturated applications, batched multi-arg `__apply`, and
//! Tier 1 online partial evaluation / memoized polyvariant specialization.
//!
//! Every test compiles through `common::drive_opts` so it runs identically
//! against both the Rust seed and Stage 3 (`run_stage_targets.py --stage 3`).

mod common;
use cinch::driver::{DriverOptions, Mode};
use std::path::{Path, PathBuf};
use std::process::Command;

fn compile(tag: &str, source: &str, mode: Mode, opts: &DriverOptions) -> (PathBuf, PathBuf) {
    let dir = common::project(tag, &[("Main.cinch", source)]);
    let out = dir.join(if matches!(mode, Mode::EmitIr) { "main.ll" } else { "main" });
    let code = common::drive_opts(mode, &dir, "Main.cinch", Some(out.clone()), opts)
        .unwrap_or_else(|e| panic!("{tag}: {e}"));
    assert_eq!(code, 0, "{tag}: compile");
    (dir, out)
}

fn ir(tag: &str, source: &str, opts: &DriverOptions) -> String {
    let (dir, out) = compile(tag, source, Mode::EmitIr, opts);
    let text = std::fs::read_to_string(&out).expect("read emitted IR");
    common::cleanup(&dir);
    text
}

fn run_checked(exe: &Path, tag: &str) {
    let out = Command::new(exe)
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "2G")
        .output()
        .expect("run built program");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(42), "{tag}: exit status; stderr:\n{stderr}");
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "{tag}: leak check failed; stderr:\n{stderr}"
    );
}

fn build_and_run(tag: &str, source: &str, opts: &DriverOptions) {
    let (dir, exe) = compile(tag, source, Mode::Build, opts);
    run_checked(&exe, tag);
    common::cleanup(&dir);
}

/// Extract non-stub user definitions belonging to module `Main` (handles both
/// seed `@"Main.foo"` and Stage 3 `@"cinch.typed.m.Main.foo"` naming).
fn main_user_defs(text: &str) -> Vec<&str> {
    text.split("\ndefine ")
        .skip(1)
        .filter(|d| {
            let header = d.lines().next().unwrap_or("");
            (header.contains("Main.") || header.contains("Main$2e"))
                && !header.contains(".app\"")
                && !header.contains(".tapp\"")
                && !header.contains(".object-closure\"")
                && !header.contains(".dictionary\"")
        })
        .collect()
}

fn main_apply_count(text: &str) -> usize {
    main_user_defs(text)
        .into_iter()
        .map(|d| d.matches("call i64 @__apply(").count())
        .sum()
}

fn has_pe_clone(text: &str, base: &str) -> bool {
    let dashed = base.replace('-', "$2d");
    text.lines().any(|l| {
        l.starts_with("define ")
            && (l.contains(&format!("{base}.sp"))
                || l.contains(&format!("{base}$pe"))
                || l.contains(&format!("{dashed}.sp"))
                || l.contains(&format!("{dashed}$pe"))
                || l.contains(&format!("{dashed}$24pe")))
    })
}

#[test]
fn imported_polymorphic_and_conditional_instance_methods_call_directly() {
    let source = r#"module Main where
  import Algebra.Semigroup

  sum-list :: List I64 -> I64
  sum-list xs = case xs of
    [] -> 0
    x : rest -> wrapping-add-i64 x (sum-list rest)

  compute :: I64 -> I64
  compute k =
    let a = Cons 10 (Cons 12 Nil) in
    let b = Cons k (Cons 15 Nil) in
    let c = a <> b in
    sum-list c

  main :: Int
  main = int-from-i64 (compute 5)
"#;
    let opts = DriverOptions::default();
    build_and_run("direct-imported-methods-run", source, &opts);
    let text = ir("direct-imported-methods-ir", source, &opts);
    assert_eq!(
        main_apply_count(&text),
        0,
        "imported polymorphic methods (<> on List, list literals) must call directly without @__apply"
    );
}

#[test]
fn local_conditional_instance_and_self_method_call_directly() {
    let source = r#"module Main where
  signature Score a where
    score :: a -> I64
    double-score :: a -> I64

  implicit instance Score_I64 :: Score I64 where
    score x = wrapping-add-i64 x 1
    double-score x = wrapping-add-i64 (score x) (score x)

  implicit instance Score_Option a :: using (sa :: Score a) => Score (Option a) where
    score opt = case opt of
      None -> 0
      Some v -> wrapping-add-i64 (score v) 10
    double-score opt = wrapping-add-i64 (score opt) (score opt)

  #[inline(never)]
  compute :: I64 -> I64
  compute x = double-score (Some x)

  main :: Int
  main = int-from-i64 (compute 10)
"#;
    let opts = DriverOptions::default();
    build_and_run("direct-local-cond-method-run", source, &opts);
    let text = ir("direct-local-cond-method-ir", source, &opts);
    // `compute`, `Score_I64.*`, and `Score_Option.double-score` all resolve to
    // known or self instance methods and must call directly without `@__apply`.
    // (Only `Score_Option.score` calls its abstract dictionary parameter `sa`.)
    let direct_defs: Vec<&str> = main_user_defs(&text)
        .into_iter()
        .filter(|d| {
            let h = d.lines().next().unwrap_or("");
            h.contains("compute")
                || h.contains("double-score")
                || h.contains("double$2dscore")
                || h.contains("Score_I64")
                || h.contains("Score$5fI64")
        })
        .collect();
    assert!(!direct_defs.is_empty(), "expected direct instance method defs in IR");
    let direct_applies: usize = direct_defs
        .iter()
        .map(|d| d.matches("call i64 @__apply(").count())
        .sum();
    assert_eq!(
        direct_applies,
        0,
        "local conditional instance call sites and self-methods must call directly without @__apply:\n{}",
        direct_defs
            .iter()
            .filter(|d| d.contains("call i64 @__apply("))
            .copied()
            .collect::<Vec<_>>()
            .join("\n---\n")
    );
}

#[test]
fn monomorphic_local_lambda_and_let_rec_call_directly() {
    let source = r#"module Main where
  compute :: I64 -> I64
  compute n =
    let f = \x y -> wrapping-add-i64 (wrapping-mul-i64 x 2) y in
    let immediate = (\a b -> wrapping-add-i64 a b) 4 6 in
    letrec sum-n = \k -> if k == 0 then 0 else wrapping-add-i64 k (sum-n (wrapping-sub-i64 k 1)) in
    wrapping-add-i64 (f (sum-n n) immediate) 2

  main :: Int
  main = int-from-i64 (compute 5)
"#;
    let opts = DriverOptions::default();
    build_and_run("direct-local-lambda-rec-run", source, &opts);
    let text = ir("direct-local-lambda-rec-ir", source, &opts);
    assert_eq!(
        main_apply_count(&text),
        0,
        "monomorphic local lambdas, immediately applied lambdas, and local recursive helpers must call directly without @__apply"
    );
}

#[test]
fn point_free_wrapper_calls_directly() {
    let source = r#"module Main where
  add-pair :: I64 -> I64 -> I64
  add-pair a b = wrapping-add-i64 a b

  add-ten :: I64 -> I64
  add-ten = add-pair 10

  #[inline(never)]
  compute :: I64 -> I64
  compute x = add-ten x

  main :: Int
  main = int-from-i64 (compute 32)
"#;
    let opts = DriverOptions::default();
    build_and_run("direct-point-free-run", source, &opts);
    let text = ir("direct-point-free-ir", source, &opts);
    let compute_def = main_user_defs(&text)
        .into_iter()
        .find(|d| d.lines().next().unwrap_or("").contains("compute"))
        .expect("compute definition in IR");
    assert_eq!(
        compute_def.matches("call i64 @__apply(").count(),
        0,
        "point-free partial-application wrapper must saturate directly in caller without @__apply:\n{compute_def}"
    );
}

#[test]
fn indirect_multi_arg_apply_is_batched() {
    let source = r#"module Main where
  #[inline(never)]
  apply3 :: (I64 -> I64 -> I64 -> I64) -> I64 -> I64 -> I64 -> I64
  apply3 f a b c = f a b c

  make-adder :: I64 -> (I64 -> I64 -> I64 -> I64)
  make-adder k = \a b c -> wrapping-add-i64 k (wrapping-add-i64 a (wrapping-add-i64 b c))

  main :: Int
  main = int-from-i64 (apply3 (make-adder 10) 11 12 9)
"#;
    let opts = DriverOptions { no_specialise_arguments: true, no_inline: true, ..DriverOptions::default() };
    build_and_run("batched-apply-run", source, &opts);
    let text = ir("batched-apply-ir", source, &opts);
    let apply3_def = main_user_defs(&text)
        .into_iter()
        .find(|d| {
            let h = d.lines().next().unwrap_or("");
            h.contains("apply3")
        })
        .expect("apply3 definition in IR");
    let calls: Vec<&str> = apply3_def
        .lines()
        .filter(|l| l.contains("call i64 @__apply("))
        .collect();
    assert_eq!(
        calls.len(),
        1,
        "apply3 should issue a single batched @__apply call, got:\n{}\nfull IR:\n{apply3_def}",
        calls.join("\n")
    );
    assert!(
        calls[0].contains(", i64 3, ptr "),
        "batched @__apply should pass argc = 3, got: {}",
        calls[0]
    );
}

#[test]
fn tier1_pe_specializes_static_booleans_polyvariantly() {
    let source = r#"module Main where
  #[inline(never)]
  loop-mode :: Bool -> I64 -> I64 -> I64
  loop-mode fast n acc =
    if n == 0 then acc
    else
      let step = if fast then 3 else 1 in
      loop-mode fast (wrapping-sub-i64 n 1) (wrapping-add-i64 acc step)

  main :: Int
  main = int-from-i64 (wrapping-add-i64 (loop-mode True 10 0) (loop-mode False 12 0))
"#;
    let opts = DriverOptions::default();
    build_and_run("pe-bool-poly-run", source, &opts);
    let text = ir("pe-bool-poly-ir", source, &opts);
    assert!(
        has_pe_clone(&text, "loop-mode"),
        "loop-mode should be polyvariantly specialized on static Bool flags"
    );
}

#[test]
fn tier1_pe_specializes_static_constructors_and_records() {
    let source = r#"module Main where
  data Mode where
    Scale :: I64 -> Mode
    Offset :: I64 -> Mode

  #[inline(never)]
  run-mode :: Mode -> I64 -> I64 -> I64
  run-mode mode n acc =
    if n == 0 then acc
    else
      case mode of
        Scale k -> run-mode mode (wrapping-sub-i64 n 1) (wrapping-add-i64 acc (wrapping-mul-i64 k 2))
        Offset k -> run-mode mode (wrapping-sub-i64 n 1) (wrapping-add-i64 acc k)

  main :: Int
  main =
    let cfg1 = Scale 3 in
    let cfg2 = Offset 2 in
    int-from-i64 (wrapping-add-i64 (run-mode cfg1 5 0) (run-mode cfg2 6 0))
"#;
    let opts = DriverOptions::default();
    build_and_run("pe-ctor-poly-run", source, &opts);
    let text = ir("pe-ctor-poly-ir", source, &opts);
    assert!(
        has_pe_clone(&text, "run-mode"),
        "run-mode should be specialized on static Mode constructors bound via let"
    );
}

#[test]
fn tier1_pe_specializes_static_int_without_unrolling_recursion() {
    let source = r#"module Main where
  #[inline(never)]
  stride-sum :: I64 -> I64 -> I64 -> I64
  stride-sum stride n acc =
    if n == 0 then acc
    else
      let next = wrapping-add-i64 acc (if stride == 3 then 4 else 1) in
      stride-sum stride (wrapping-sub-i64 n 1) next

  main :: Int
  main = int-from-i64 (wrapping-add-i64 (stride-sum 3 10 0) (stride-sum 1 2 0))
"#;
    let opts = DriverOptions::default();
    build_and_run("pe-int-bounded-run", source, &opts);
    let text = ir("pe-int-bounded-ir", source, &opts);
    assert!(
        has_pe_clone(&text, "stride-sum"),
        "stride-sum should specialize on static stride without unrolling n"
    );
}

#[test]
fn tier1_pe_specializes_higher_order_function_argument() {
    let source = r#"module Main where
  inc2 :: I64 -> I64
  inc2 x = wrapping-add-i64 x 2

  #[inline(never)]
  repeat-apply :: (I64 -> I64) -> I64 -> I64 -> I64
  repeat-apply f n acc =
    if n == 0 then acc
    else repeat-apply f (wrapping-sub-i64 n 1) (f acc)

  main :: Int
  main = int-from-i64 (repeat-apply inc2 21 0)
"#;
    let opts = DriverOptions::default();
    build_and_run("pe-hof-run", source, &opts);
    let text = ir("pe-hof-ir", source, &opts);
    assert!(
        has_pe_clone(&text, "repeat-apply"),
        "repeat-apply should be specialized on known function inc2"
    );
    let spec_def = main_user_defs(&text)
        .into_iter()
        .find(|d| {
            let h = d.lines().next().unwrap_or("");
            h.contains("repeat-apply.sp")
                || h.contains("repeat-apply$pe")
                || h.contains("repeat$2dapply.sp")
                || h.contains("repeat$2dapply$pe")
                || h.contains("repeat$2dapply$24pe")
        })
        .expect("specialized repeat-apply definition");
    assert_eq!(
        spec_def.matches("call i64 @__apply(").count(),
        0,
        "specialized repeat-apply must call inc2 directly without @__apply"
    );
}
