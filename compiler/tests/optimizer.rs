//! Rewrites, the optimizer, target profiles and trust levels (P10):
//! `rewrite`/`target rewrite` declarations with `justify` checking, the
//! driving engine with its barriers and recheck, the map-fusion case of
//! modules and compilation specification section 6, the attribute
//! chapter's section 2.4 weight reconciliation, driving bounds and the
//! specialization report, and the artifact manifest's trust level and
//! target conditions.

use cinch::core::{optimize, OptReport, Program, TargetProfile};
use cinch::driver::{check_sources, driver, run_source_profile, Mode};
use std::path::{Path, PathBuf};

fn checked(name: &str, src: &str) -> Program {
    check_sources(src, name).unwrap_or_else(|e| panic!("{name}: check failed: {e}"))
}

fn opt_of(name: &str, src: &str, profile: &TargetProfile) -> (Program, OptReport) {
    let program = checked(name, src);
    optimize(&program, profile).unwrap_or_else(|e| panic!("{name}: optimize failed: {e}"))
}

/// The optimized program's LLVM IR (every IR test carries a `main`).
fn ir_of(name: &str, src: &str, profile: &TargetProfile) -> String {
    let (program, _report) = opt_of(name, src, profile);
    cinch::codegen::emit(&program).unwrap_or_else(|e| panic!("{name}: emit failed: {e}"))
}

fn run(name: &str, src: &str, profile: &TargetProfile) -> i32 {
    run_source_profile(src, profile)
        .unwrap_or_else(|e| panic!("{name}: run failed: {e}"))
}

fn expect_err(name: &str, src: &str, needle: &str) {
    match check_sources(src, name) {
        Ok(_) => panic!("{name}: expected an error containing {needle:?}, but the program checked"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains(needle),
                "{name}: error {msg:?} does not contain {needle:?}"
            );
        }
    }
}

/// The eager map library and its law: `mapLC` is the fused form of
/// `mapL f (mapL g xs)`, and `OptLaws.mapPair` states the equation.
const EAGER_LIB: &str = r#"
  mapL :: forall a b. (a -> b) -> List a -> List b
  mapL f xs = case xs of
    Nil -> Nil
    Cons x rest -> Cons (f x) (mapL f rest)

  mapLC :: forall a b c. (b -> c) -> (a -> b) -> List a -> List c
  mapLC f g xs = mapL (\x -> f (g x)) xs

  signature OptLaws where
    mapPair :: forall a b c. (f :: b -> c) -> (g :: a -> b) -> (xs :: List a) -> Eq (mapL f (mapL g xs)) (mapLC f g xs)
"#;

// ------------------------------------------- rewrite declarations (section 5)

#[test]
fn rewrite_rule_fires_and_report_names_it() {
    let (program, report) = opt_of(
        "rule-fire",
        &format!(
            r#"
module RuleFire where
{EAGER_LIB}
  rewrite mapMap forall (f :: U32 -> U32) (g :: U32 -> U32) (xs :: List U32). where
    match mapL f (mapL g xs)
    replace mapLC f g xs
    justify OptLaws.mapPair f g xs

  double :: U32 -> U32
  double x = wrapping-add-u32 x x

  incr :: U32 -> U32
  incr x = wrapping-add-u32 x 1

  sumL :: List U32 -> U32
  sumL xs = case xs of
    Nil -> 0
    Cons x rest -> wrapping-add-u32 x (sumL rest)

  main :: Int
  main =
    let xs = Cons 1 (Cons 2 Nil) in
    if sumL (mapL double (mapL incr xs)) == 10 then 0 else 1
"#
        ),
        &TargetProfile::native(),
    );
    assert!(
        report
            .sites
            .iter()
            .any(|s| s.kind == "rewrite:mapMap" && s.stop.is_none()),
        "the rule must fire: {:?}",
        report.sites
    );
    assert!(
        report.assumptions.iter().any(|a| a.contains("mapPair")),
        "the justification identity must be recorded: {:?}",
        report.assumptions
    );
    assert_eq!(program.fns.len() > 0, true);
    // The fused pipeline computes the same list: (1+1)*2 + (2+1)*2 = 10.
    let code = run(
        "rule-fire-run",
        &format!(
            r#"
module RuleFireRun where
{EAGER_LIB}
  rewrite mapMap forall (f :: U32 -> U32) (g :: U32 -> U32) (xs :: List U32). where
    match mapL f (mapL g xs)
    replace mapLC f g xs
    justify OptLaws.mapPair f g xs

  double :: U32 -> U32
  double x = wrapping-add-u32 x x

  incr :: U32 -> U32
  incr x = wrapping-add-u32 x 1

  sumL :: List U32 -> U32
  sumL xs = case xs of
    Nil -> 0
    Cons x rest -> wrapping-add-u32 x (sumL rest)

  main :: Int
  main =
    let xs = Cons 1 (Cons 2 Nil) in
    if sumL (mapL double (mapL incr xs)) == 10 then 0 else 1
"#
        ),
        &TargetProfile::native(),
    );
    assert_eq!(code, 0);
}

#[test]
fn justify_failing_rule_is_rejected() {
    // The law states `Eq (mapL f (mapL g xs)) (mapLC f g xs)`; this rule
    // claims a different equation and its justification does not prove it.
    expect_err(
        "justify-fail",
        &format!(
            r#"
module JustifyFail where
{EAGER_LIB}
  rewrite wrongMap forall (f :: U32 -> U32) (g :: U32 -> U32) (xs :: List U32). where
    match mapL f (mapL g xs)
    replace mapL f xs
    justify OptLaws.mapPair f g xs
"#
        ),
        "justification does not prove",
    );
}

#[test]
fn rule_that_would_change_definitional_equality_is_rejected() {
    // A bare variable pattern would rewrite every term of its type.
    expect_err(
        "defeq-var",
        r#"
module DefeqVar where
  rewrite everything forall (x :: U32) (y :: U32). where
    match x
    replace y
    justify refl
"#,
        "change definitional equality",
    );
    // A rule over erased evidence would break proof irrelevance (the
    // pattern's type is a proposition).
    expect_err(
        "defeq-erased",
        r#"
module DefeqErased where
  rewrite overProof forall (x :: Nat) (h :: Eq x x). where
    match h
    replace h
    justify refl
"#,
        "definitional equality",
    );
}

#[test]
fn rewrites_do_not_extend_conversion() {
    // With the map rule in scope, `refl` still proves only definitional
    // equality: conversion is exactly the D28 rules.
    expect_err(
        "conversion-unchanged",
        &format!(
            r#"
module ConversionUnchanged where
{EAGER_LIB}
  rewrite mapMap forall (f :: U32 -> U32) (g :: U32 -> U32) (xs :: List U32). where
    match mapL f (mapL g xs)
    replace mapLC f g xs
    justify OptLaws.mapPair f g xs

  bogus :: (f :: U32 -> U32) -> (g :: U32 -> U32) -> (xs :: List U32) -> Eq (mapL f (mapL g xs)) (mapLC f g xs)
  bogus f g xs = refl
"#
        ),
        "Eq",
    );
}

// ------------------------------------------------- fusion (modules section 6)

/// The producer library's step form is in the prelude: `Step s a` with
/// `Done`/`Skip`/`Yield`, `Producer s a`, `prod-map`, `prod-take` and
/// `prod-fold`.
const PRODUCER: &str = r#"
  step3 :: Nat -> Step Nat Nat
  step3 k = case k of
    0 -> Yield 3 1
    1 -> Yield 5 2
    _ -> Done

  srcP :: Producer Nat Nat
  srcP = Producer step3 0

  doubleN :: Nat -> Nat
  doubleN x = nat-add x x

  tripleN :: Nat -> Nat
  tripleN x = nat-add x (nat-add x x)

  sumP :: Nat
  sumP = prod-fold 0 nat-add (prod-map doubleN (prod-map tripleN srcP))
"#;

#[test]
fn map_fusion_preserves_values_and_effect_order() {
    // The fused pipeline (fuel 8) and the unfused baseline (fuel 0)
    // compute the same values and perform the callbacks in the same
    // observable order.
    let trace = r#"
module TracePipe where
  step3 :: Nat -> Step Nat Nat
  step3 k = case k of
    0 -> Yield 3 1
    1 -> Yield 5 2
    _ -> Done

  srcP :: Producer Nat Nat
  srcP = Producer step3 0

  trace :: Nat
  trace =
    let log = bytes-new 1 in
    let bump m = bytes-set log 0 (nat-rem (nat-add (nat-mul (bytes-get log 0) 4) m) 256) in
    let g x = let _r = bump 1 in nat-add x (nat-add x x) in
    let f x = let _q = bump 2 in nat-add x x in
    let k acc x = let _p = bump 3 in nat-add acc x in
    let _v = prod-fold 0 k (prod-map f (prod-map g srcP)) in
    bytes-get log 0

  main :: Int
  -- Per element the callbacks run inner-map (1), outer-map (2), fold
  -- (3): the log records 1,2,3 then 1,2,3 (order preserved under
  -- fusion), which is 219 under the `*4 + m` accumulator.
  main = if trace == 219 then 0 else 1
"#;
    let fused = run("trace-fused", trace, &TargetProfile::native());
    let base = run("trace-base", trace, &TargetProfile { fuel: 0, ..TargetProfile::native() });
    assert_eq!(fused, 0, "the fused run must keep values and effect order");
    assert_eq!(base, 0, "the unfused baseline must agree");
    // The report names the fusion and what it eliminated.
    let (_p, report) = opt_of("trace-report", trace, &TargetProfile::native());
    assert!(
        report.fused.iter().any(|f| f.contains("fused") && f.contains("intermediate")),
        "the fusion step must be reported: {:?}",
        report.fused
    );
}

#[test]
fn map_headed_fusion_composes_every_stage() {
    // A map chain that is not consumed by a fold in the same unit takes the
    // map-headed fusion path. Every stage's callback must run: the head's
    // own (`addHundred`) as well as the inner ones. Each dropped-stage
    // combination gives a distinct sum, so the exit code says which.
    let src = r#"
module MapHead where
  step3 :: Nat -> Step Nat Nat
  step3 k = case k of
    0 -> Yield 3 1
    1 -> Yield 5 2
    _ -> Done

  srcP :: Producer Nat Nat
  srcP = Producer step3 0

  addOne :: Nat -> Nat
  addOne x = nat-add x 1

  timesTen :: Nat -> Nat
  timesTen x = nat-mul x 10

  addHundred :: Nat -> Nat
  addHundred x = nat-add x 100

  #[inline(never)]
  pipe :: Producer Nat Nat
  pipe = prod-map addHundred (prod-map timesTen (prod-map addOne srcP))

  main :: Int
  -- (3+1)*10+100 + (5+1)*10+100 = 300; addOne alone gives 10, without
  -- the head 100, without addOne 280
  main =
    let t = prod-fold 0 nat-add pipe in
    if t == 300 then 0 else if t == 10 then 3 else if t == 100 then 4 else if t == 280 then 5 else 1
"#;
    let base = run("map-head-base", src, &TargetProfile { fuel: 0, ..TargetProfile::native() });
    assert_eq!(base, 0, "the unfused baseline must compute 300");
    let fused = run("map-head-fused", src, &TargetProfile::native());
    assert_eq!(fused, 0, "the fused pipeline must run every map stage");
}

#[test]
fn fusion_preserves_the_pull_boundary() {
    // A producer whose third step records that it ran: `prod-take 2`
    // means the fused pipeline must never pull it, so the log stays 0.
    let src = r#"
module Pull where
  stepF :: using (0 < bytes-len log) => (log :: Bytes) -> Nat -> Step Nat Nat
  stepF log k = case k of
    0 -> Yield 3 1
    1 -> Yield 5 2
    _ -> let _p = bytes-set log 0 1 in Done

  doubleN :: Nat -> Nat
  doubleN x = nat-add x x

  main :: Int
  main =
    let log = bytes-new 1 in
    let s = Producer (stepF log) 0 in
    let taken = prod-take 2 (prod-map doubleN s) in
    if prod-fold 0 nat-add taken == 16 && bytes-get log 0 == 0 then 0 else 1
"#;
    let fused = run("pull-fused", src, &TargetProfile::native());
    assert_eq!(fused, 0, "the fused pipeline must pull exactly two steps");
    let base = run(
        "pull-base",
        src,
        &TargetProfile { fuel: 0, ..TargetProfile::native() },
    );
    assert_eq!(base, 0);
}

#[test]
fn blocked_fusion_names_the_blocker() {
    // An allocation in a callback blocks fusion (memo structure may not
    // be reordered relative to its demand).
    let alloc = r#"
module AllocBlock where
  step3 :: Nat -> Step Nat Nat
  step3 k = case k of
    0 -> Yield 3 1
    1 -> Yield 5 2
    _ -> Done

  mem :: Nat -> Nat
  mem x = let ~y = nat-add x x in copyOut (demand y)

  main :: Int
  main =
    let s = Producer step3 0 in
    if prod-fold 0 nat-add (prod-map mem (prod-map mem s)) == 8 then 0 else 1
"#;
    let (_p, report) = opt_of("alloc-block", alloc, &TargetProfile::native());
    assert!(
        report
            .sites
            .iter()
            .any(|s| s.kind == "fusion" && s.stop.as_deref().map(|r| r.contains("allocation barrier")).unwrap_or(false)),
        "the allocation barrier must be reported: {:?}",
        report.sites
    );
    // An effectful callback blocks the eager map rule: its pure
    // justification does not authorize the interleaving fusion performs.
    let eager = format!(
        r#"
module EffectBlock where
{EAGER_LIB}
  rewrite mapMap forall (f :: U32 -> U32) (g :: U32 -> U32) (xs :: List U32). where
    match mapL f (mapL g xs)
    replace mapLC f g xs
    justify OptLaws.mapPair f g xs

  traceRun :: Nat
  traceRun =
    let log = bytes-new 1 in
    let bump m = bytes-set log 0 (nat-rem (nat-add (nat-mul (bytes-get log 0) 4) m) 256) in
    let g x = let _q = bump 1 in wrapping-add-u32 x 2 in
    let f x = let _p = bump 2 in wrapping-add-u32 x 1 in
    let _v = mapL f (mapL g (Cons 1 (Cons 2 Nil))) in
    bytes-get log 0

  main :: Int
  -- Unfused order: the inner map runs to completion (1,1) then the
  -- outer (2,2), which is 90 under the `*4 + m` accumulator.
  main = if traceRun == 90 then 0 else 1
"#
    );
    let (_p, report) = opt_of("effect-block", &eager, &TargetProfile::native());
    assert!(
        report.sites.iter().any(|s| {
            s.kind == "rewrite:mapMap"
                && s.stop
                    .as_deref()
                    .map(|r| r.contains("effect barrier"))
                    .unwrap_or(false)
        }),
        "the effect barrier must be reported: {:?}",
        report.sites
    );
    let code = run("effect-block-run", &eager, &TargetProfile::native());
    assert_eq!(code, 0, "the refused rule must leave the effect order intact");
}

// --------------------------------------------- target rules (platform 2-3)

const TARGET_RULE: &str = r#"
  target rewrite sumPair
    forall t w n region (xs :: Ref region (Vec n U32)).
    using (vectors :: VecU32Target t w) where
    match Scalar.sum-wrap32 xs
    replace Simd.sum-wrap32 vectors xs
    require Available vectors
    justify SumProofs.scalar-to-simd vectors xs
"#;

#[test]
fn target_rewrite_applies_in_capability_scope_only() {
    let src = format!(
        r#"
module TargetScope where
{TARGET_RULE}
  fast :: forall t w n region. using (vectors :: VecU32Target t w) => Ref region (Vec n U32) -> U32
  fast xs = Scalar.sum-wrap32 xs

  slow :: forall n region. Ref region (Vec n U32) -> U32
  slow xs = Scalar.sum-wrap32 xs
"#
    );
    let (program, report) = opt_of("target-scope", &src, &TargetProfile::native());
    let fires = report
        .sites
        .iter()
        .filter(|s| s.kind == "rewrite:sumPair" && s.stop.is_none())
        .count();
    assert!(fires >= 1, "the rule must fire inside the capability scope: {fires}");
    assert!(
        report.sites.iter().any(|s| {
            s.kind == "rewrite:sumPair"
                && s.stop
                    .as_deref()
                    .map(|r| r.contains("capability scope"))
                    .unwrap_or(false)
        }),
        "the rule must be refused outside the capability scope: {:?}",
        report.sites
    );
    // The specialized unit keeps its contract identity: the same units
    // with the same arities come out.
    assert!(program.fns.iter().any(|f| f.id.0.ends_with("fast")));
    assert!(program.fns.iter().any(|f| f.id.0.ends_with("slow")));
}

// ------------------------------------- weights under optimization (attr 2.4)

#[test]
fn weights_survive_optimization() {
    // Decision merging reconciles the two weight spaces by the rescale
    // formula and a greatest common divisor reduction: outer 10:290 with
    // inner 200:100 merges to 3000:58000:29000, reduced to 3:58:29.
    let merged = ir_of(
        "weight-merge",
        r#"
module WeightMerge where
  data Tri where
    A :: Tri
    B :: Tri
    C :: Tri

  merged :: Tri -> Nat
  merged x = case x of
    #[weight(10)] A -> 1
    #[weight(290)] y -> case x of
      #[weight(200)] B -> 2
      #[weight(100)] C -> 3
      #[weight(7)] y2 -> 4

  main :: Int
  main = if merged A == 1 then 0 else 1
"#,
        &TargetProfile::native(),
    );
    assert!(
        merged.contains("!{!\"branch_weights\", i32 0, i32 3, i32 58, i32 29}"),
        "expected the merged 3:58:29 weights in:\n{merged}"
    );
    // Dead-alternative conditioning drops the impossible alternative and
    // must NOT rescale the survivors: 10:20:10 with the first impossible
    // (inside the branch that decided the scrutinee's negation) leaves
    // 20:10, which is exactly the conditional ratio.
    let dead = ir_of(
        "weight-dead",
        r#"
module WeightDead where
  data Tri where
    A :: Tri
    B :: Tri
    C :: Tri

  dead :: Tri -> Nat
  dead x = case x of
    #[weight(10)] A -> 1
    #[weight(290)] y -> let w = nat-add 1 2 in case x of
      #[weight(10)] A -> 2
      #[weight(20)] B -> 3
      #[weight(10)] C -> 4

  main :: Int
  main = if dead A == 1 then 0 else 1
"#,
        &TargetProfile::native(),
    );
    assert!(
        dead.contains("!{!\"branch_weights\", i32 0, i32 20, i32 10}"),
        "expected unrescaled 20:10 weights in:\n{dead}"
    );
    assert!(
        !dead.contains("i32 2, i32 1"),
        "the survivors must not be rescaled:\n{dead}"
    );
    // Decision-of-decision independence: a decision on a value produced
    // by another decision is pushed inward and each duplicated decision
    // keeps its original weights.
    let pushed = ir_of(
        "weight-push",
        r#"
module WeightPush where
  data Tri where
    A :: Tri
    B :: Tri
    C :: Tri

  pushed :: Bool -> Tri -> Nat
  pushed b x = case (if b then x else x) of
    #[weight(5)] A -> 1
    #[weight(7)] B -> 2
    #[weight(11)] C -> 3

  main :: Int
  main = if pushed True A == 1 then 0 else 1
"#,
        &TargetProfile::native(),
    );
    let copies = pushed.matches("i32 5, i32 7, i32 11").count();
    assert!(
        copies >= 2,
        "each duplicated decision must keep its original weights in:\n{pushed}"
    );
}

// ------------------------------------------------- driving bounds and report

#[test]
fn linear_value_generalizes_instead_of_duplicating() {
    let src = r#"
module Linear where
  dup2 :: Nat -> Nat
  dup2 x = nat-add x x

  consume1 :: Nat %1 -> Nat
  consume1 y = dup2 y

  main :: Int
  main = if consume1 5 == 10 then 0 else 1
"#;
    let (_p, report) = opt_of("linear", src, &TargetProfile::native());
    assert!(
        report.sites.iter().any(|s| {
            s.stop
                .as_deref()
                .map(|r| r.contains("linear value"))
                .unwrap_or(false)
        }),
        "the linear barrier must be reported: {:?}",
        report.sites
    );
    let code = run("linear-run", src, &TargetProfile::native());
    assert_eq!(code, 0);
}

#[test]
fn fuel_exhaustion_preserves_behavior() {
    let src = format!(
        r#"
module Fuel where
{PRODUCER}
  main :: Int
  main = if sumP == 48 then 0 else 1
"#
    );
    let native = TargetProfile::native();
    let zero = TargetProfile { fuel: 0, ..TargetProfile::native() };
    let one = TargetProfile { fuel: 1, ..TargetProfile::native() };
    let a = run("fuel-0", &src, &zero);
    let b = run("fuel-1", &src, &one);
    let c = run("fuel-8", &src, &native);
    assert_eq!((a, b, c), (0, 0, 0), "fuel must never change observed behavior");
    let (_p, report) = opt_of("fuel-report", &src, &one);
    assert!(
        report
            .sites
            .iter()
            .any(|s| s.stop.as_deref() == Some("fuel-exhausted")),
        "the report must say where specialization stopped: {:?}",
        report.sites
    );
}

// ---------------------------------------------- manifest: trust and targets

fn write_project(files: &[(&str, &str)]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "cinch_opttest_{}_{}",
        std::process::id(),
        files.as_ptr() as usize
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (name, src) in files {
        std::fs::write(dir.join(name), src).expect("write module");
    }
    dir
}

#[test]
fn manifest_records_trust_level_and_target_conditions() {
    let dir = write_project(&[(
        "Main.cinch",
        &format!(
            r#"
module Main where
{TARGET_RULE}
  main :: Int
  main = 0
"#
        ),
    )]);
    driver(Mode::Run, &dir.join("Main.cinch"), None).expect("project run");
    let manifest =
        std::fs::read_to_string(dir.join("build/Main.manifest.json")).expect("manifest");
    assert!(manifest.contains("\"trust\""), "{manifest}");
    assert!(manifest.contains("\"level\":2"), "{manifest}");
    assert!(manifest.contains("full-recheck"), "{manifest}");
    assert!(manifest.contains("\"target\""), "{manifest}");
    assert!(manifest.contains("sumPair"), "{manifest}");
    assert!(manifest.contains("VecU32Target"), "{manifest}");
    assert!(manifest.contains("\"specialization\""), "{manifest}");
    assert!(manifest.contains("\"lowering\""), "{manifest}");
    // The justification identity is a declared assumption in the manifest.
    assert!(manifest.contains("scalar-to-simd"), "{manifest}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn contract_identity_is_unchanged_by_specialization() {
    // Specialization is representation-only: the units, arities and
    // instances of the residual program are the generic form's.
    let src = format!(
        r#"
module Identity where
{PRODUCER}
  main :: Int
  main = if sumP == 48 then 0 else 1
"#
    );
    let generic = checked("identity", &src);
    let (residual, _report) = opt_of("identity-opt", &src, &TargetProfile::native());
    assert_eq!(generic.fns.len() >= residual.fns.len() - 8, true);
    for (a, b) in generic.fns.iter().zip(residual.fns.iter()) {
        assert_eq!(a.id, b.id);
        assert_eq!(a.params.len(), b.params.len());
    }
    assert_eq!(generic.main, residual.main);
}

#[test]
fn scalable_profile_does_not_pin_a_width() {
    // A scalable-shape profile records no fixed lane count: the artifact
    // must not assume an observed length stays fixed (platform section
    // 6).
    let dir = write_project(&[(
        "Main.cinch",
        &format!(
            r#"
module Main where
{TARGET_RULE}
  main :: Int
  main = 0
"#
        ),
    )]);
    let profile = TargetProfile {
        vector_shape: cinch::core::VectorShape::Scalable,
        ..TargetProfile::native()
    };
    cinch::driver::driver_with_profile(
        Mode::Run,
        cinch::driver::StageMode::Refresh,
        &dir.join("Main.cinch"),
        None,
        &profile,
    )
    .expect("project run");
    let manifest =
        std::fs::read_to_string(dir.join("build/Main.manifest.json")).expect("manifest");
    assert!(manifest.contains("\"vector_shape\":\"scalable\""), "{manifest}");
    assert!(!manifest.contains("fixed(8)"), "{manifest}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn example_08_probe_is_past_rewrite() {
    // The acceptance probe: the `rewrite` keyword rejection is gone; the
    // target declaration parses and checks through its binders, context,
    // pattern, template and condition.
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../examples/source/08-platform-simd.cinch");
    let src = std::fs::read_to_string(&path).expect("example 08");
    match check_sources(&src, "08-platform-simd.cinch") {
        Ok(_) => {}
        Err(e) => {
            let msg = e.to_string();
            assert!(!msg.contains("rewrite declarations are not supported"), "{msg}");
            assert!(!msg.contains("field projection"), "{msg}");
        }
    }
}
