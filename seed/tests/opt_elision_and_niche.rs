//! End-to-end tests for boxing/RC/allocation elision optimizations and
//! user-defined refinement-driven niche fusion (`#[repr(niche)]`).
//!
//! Every test routes through `common::source_frontend` / `common::build` so the
//! exact same suite verifies both the Rust seed and the Stage 3 self-hosted
//! compiler.

mod common;

use common::source_frontend;
use std::process::Command;

fn run_checked_with_stats(tag: &str, source: &str, expected_exit: i32) -> (u64, u64) {
    let dir = common::project(tag, &[("Main.cinch", source)]);
    let exe = common::build(&dir, "Main.cinch", "prog")
        .unwrap_or_else(|e| panic!("{tag}: build failed: {e}"));
    let out = Command::new(&exe)
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_ALLOC_STATS", "1")
        .output()
        .expect("run built binary");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(expected_exit),
        "{tag}: unexpected exit status; stderr:\n{stderr}"
    );
    assert!(
        stderr.contains("leaked=0"),
        "{tag}: memory leak detected; stderr:\n{stderr}"
    );
    let blocks = stderr
        .lines()
        .find(|l| l.starts_with("cinch-alloc:"))
        .and_then(|l| {
            l.split_whitespace()
                .find_map(|w| w.strip_prefix("header-blocks="))
                .and_then(|v| v.parse::<u64>().ok())
        })
        .unwrap_or(0);
    let freed = stderr
        .lines()
        .find(|l| l.starts_with("cinch-alloc:"))
        .and_then(|l| {
            l.split_whitespace()
                .find_map(|w| w.strip_prefix("freed="))
                .and_then(|v| v.parse::<u64>().ok())
        })
        .unwrap_or(0);
    common::cleanup(&dir);
    (blocks, freed)
}

#[test]
fn niche_refinement_i64_with_multiple_nullaries() {
    let source = r#"module Main where
  type NonNegI64 = refine (x :: I64) with x >= 0

  #[repr(niche)]
  data Lookup where
    Found :: NonNegI64 -> Lookup
    NotFound :: Lookup
    Timeout :: Lookup

  make :: I64 -> Lookup
  make n =
    if n < 0 then NotFound
    else if n == 999 then Timeout
    else Found n

  score :: Lookup -> I64
  score r = case r of
    NotFound -> 1
    Timeout -> 2
    Found v -> v

  loop :: I64 -> I64 -> I64
  loop i acc =
    if i == 0 then acc
    else
      let v = if i == 500 then score (make 999) else if i == 250 then score (make (wrapping-sub-i64 0 5)) else score (make 1) in
      loop (wrapping-sub-i64 i 1) (wrapping-add-i64 acc v)

  main :: Int
  main =
    let total = loop 1000 0 in
    if total == 1001 then 42 else 1
"#;
    let (blocks, _) = run_checked_with_stats("niche_i64_nullaries", source, 42);
    assert!(blocks < 64, "niche-fused Lookup loop should allocate zero cells, got {blocks}");
    let ir = source_frontend::emit_ir(source).expect("emit IR");
    let score_fn = ir
        .split("\ndefine ")
        .find(|d| {
            d.lines()
                .next()
                .is_some_and(|hdr| hdr.contains("Main.score") && hdr.contains('{'))
        })
        .expect("Main.score in IR");
    assert!(
        !score_fn.contains("cinch_alloc"),
        "niche-fused score must not allocate:\n{score_fn}"
    );
}

#[test]
fn niche_nonzero_i64_uses_zero_sentinel() {
    let source = r#"module Main where
  import Data.NonZero

  #[repr(niche)]
  data OptNZ where
    NZNone :: OptNZ
    NZSome :: NonZero I64 -> OptNZ

  wrap :: I64 -> OptNZ
  wrap n = case decide (NonZero n) of
    Yes w -> NZSome (NonZero n w)
    No _ -> NZNone

  unwrap-or :: OptNZ -> I64 -> I64
  unwrap-or o d = case o of
    NZNone -> d
    NZSome v -> unrefine v

  main :: Int
  main =
    let a = unwrap-or (wrap 0) 10 in
    let b = unwrap-or (wrap 40) 0 in
    let c = unwrap-or (wrap (wrapping-sub-i64 0 8)) 0 in
    if wrapping-add-i64 (wrapping-add-i64 a b) c == 42 then 42 else 1
"#;
    let (blocks, _) = run_checked_with_stats("niche_nonzero", source, 42);
    assert!(blocks < 64, "NonZero niche should allocate nothing, got {blocks}");
}

#[test]
fn niche_obj_single_and_multi_nullary_leak_free() {
    let source = r#"module Main where
  #[repr(niche)]
  data OptStr where
    ONone :: OptStr
    OSome :: String -> OptStr

  #[repr(niche)]
  data TriStr where
    TNone :: TriStr
    TErr :: TriStr
    TOk :: String -> TriStr

  len-opt :: OptStr -> Nat
  len-opt o = case o of
    ONone -> 0
    OSome s -> string-length s

  len-tri :: TriStr -> Nat
  len-tri t = case t of
    TNone -> 1
    TErr -> 2
    TOk s -> string-length s

  go :: Nat -> Nat -> Nat
  go i acc =
    if i == 0 then acc
    else
      let s = string-concat "ab" "cd" in
      let o = if i == 1 then ONone else OSome s in
      let t = if i == 2 then TNone else if i == 3 then TErr else TOk (string-concat "xy" "z") in
      go (i - 1) (acc + len-opt o + len-tri t)

  main :: Int
  main =
    -- 10 iterations:
    -- len-opt: 1*0 + 9*4 = 36
    -- len-tri: 1*1 + 1*2 + 8*3 = 27
    -- total = 63
    if go 10 0 == 63 then 42 else 1
"#;
    run_checked_with_stats("niche_obj_multi", source, 42);
}

#[test]
fn niche_nested_obj_avoids_sentinel_collision() {
    let source = r#"module Main where
  #[repr(niche)]
  data OptStr where
    ONone :: OptStr
    OSome :: String -> OptStr

  #[repr(niche)]
  data OptOptStr where
    O2None :: OptOptStr
    O2Some :: OptStr -> OptOptStr

  classify :: OptOptStr -> I64
  classify x = case x of
    O2None -> 10
    O2Some inner -> case inner of
      ONone -> 20
      OSome s -> if string-length s == 4 then 12 else 99

  main :: Int
  main =
    let a = classify O2None in
    let b = classify (O2Some ONone) in
    let c = classify (O2Some (OSome (string-concat "ci" "nc"))) in
    if wrapping-add-i64 (wrapping-add-i64 a b) c == 42 then 42 else 1
"#;
    run_checked_with_stats("niche_nested_obj", source, 42);
}

#[test]
fn niche_disjoint_multi_payload_and_gap_nullary() {
    let source = r#"module Main where
  type PosI64 = refine (x :: I64) with x > 0
  type NegI64 = refine (e :: I64) with e < 0

  #[repr(niche)]
  data TriSign where
    Pos :: PosI64 -> TriSign
    Zero :: TriSign
    Neg :: NegI64 -> TriSign

  classify :: I64 -> TriSign
  classify n =
    if n > 0 then Pos n
    else if n < 0 then Neg n
    else Zero

  eval :: TriSign -> I64
  eval t = case t of
    Pos x -> wrapping-mul-i64 x 2
    Zero -> 10
    Neg e -> wrapping-sub-i64 0 e

  main :: Int
  main =
    let a = eval (classify 11) in
    let b = eval (classify 0) in
    let c = eval (classify (wrapping-sub-i64 0 10)) in
    if wrapping-add-i64 (wrapping-add-i64 a b) c == 42 then 42 else 1
"#;
    let (blocks, _) = run_checked_with_stats("niche_multi_payload", source, 42);
    assert!(blocks < 64, "multi-payload niche should allocate nothing, got {blocks}");
}

#[test]
fn niche_pure_nullary_enum_and_nested_wrapper() {
    let source = r#"module Main where
  #[repr(niche)]
  data Color where
    Red :: Color
    Green :: Color
    Blue :: Color

  #[repr(niche)]
  data OptColor where
    NoColor :: OptColor
    HasColor :: Color -> OptColor

  weight :: OptColor -> I64
  weight c = case c of
    NoColor -> 2
    HasColor col -> case col of
      Red -> 10
      Green -> 13
      Blue -> 17

  main :: Int
  main =
    let ab = wrapping-add-i64 (weight NoColor) (weight (HasColor Red)) in
    let cd = wrapping-add-i64 (weight (HasColor Green)) (weight (HasColor Blue)) in
    if wrapping-add-i64 ab cd == 42 then 42 else 1
"#;
    let (blocks, _) = run_checked_with_stats("niche_pure_enum", source, 42);
    assert!(blocks < 64, "pure enum niche should allocate nothing, got {blocks}");
}

#[test]
fn niche_refined_immediate_nat_uses_even_sentinels() {
    let source = r#"module Main where
  type SmallNat = refine (n :: Nat) with n < 1000

  #[repr(niche)]
  data SmallOpt where
    SNone :: SmallOpt
    STimeout :: SmallOpt
    SVal :: SmallNat -> SmallOpt

  make :: Nat -> SmallOpt
  make n = if n < 1000 then SVal n else STimeout

  read :: SmallOpt -> Nat
  read o = case o of
    SNone -> 2
    STimeout -> 10
    SVal n -> n

  main :: Int
  main =
    if read SNone + read (make 2000) + read (make 30) == 42 then 42 else 1
"#;
    let (blocks, _) = run_checked_with_stats("niche_imm_nat", source, 42);
    assert!(blocks < 64, "immediate Nat niche should allocate nothing, got {blocks}");
}

#[test]
fn niche_rejects_invalid_definitions() {
    let overlapping = r#"module Main where
  type NonNeg = refine (x :: I64) with x >= 0
  type AtMostTen = refine (y :: I64) with y <= 10

  #[repr(niche)]
  data BadOverlap where
    A :: NonNeg -> BadOverlap
    B :: AtMostTen -> BadOverlap
  main :: Int
  main = 0
"#;
    let err = source_frontend::check_sources(overlapping, "bad_overlap.cinch")
        .expect_err("overlapping refined payload domains must be rejected");
    assert!(
        err.contains("disjoint"),
        "expected disjoint domain diagnostic, got: {err}"
    );

    let unconstrained = r#"module Main where
  #[repr(niche)]
  data BadFull where
    FNone :: BadFull
    FSome :: I64 -> BadFull
  main :: Int
  main = 0
"#;
    let err = source_frontend::check_sources(unconstrained, "bad_full.cinch")
        .expect_err("unconstrained I64 + nullary must be rejected");
    assert!(
        err.contains("niche"),
        "expected niche diagnostic, got: {err}"
    );

    let on_record = r#"module Main where
  #[repr(niche)]
  record BadRec where
    x :: I64
  main :: Int
  main = 0
"#;
    assert!(
        source_frontend::check_sources(on_record, "bad_rec.cinch").is_err(),
        "repr(niche) on record must be rejected"
    );

    let two_fields = r#"module Main where
  #[repr(niche)]
  data BadTwo where
    Pair :: I64 -> I64 -> BadTwo
  main :: Int
  main = 0
"#;
    assert!(
        source_frontend::check_sources(two_fields, "bad_two.cinch").is_err(),
        "repr(niche) constructor with two fields must be rejected"
    );

    let combined = r#"module Main where
  #[repr(niche, C)]
  data BadCombo where
    One :: BadCombo
  main :: Int
  main = 0
"#;
    assert!(
        source_frontend::check_sources(combined, "bad_combo.cinch").is_err(),
        "repr(niche, C) must be rejected"
    );
}

#[test]
fn refinement_proved_tobj_to_raw_elides_rc() {
    let source = r#"module Main where
  type SmallNat = refine (n :: Nat) with n < 1000

  -- Unused parameter `b` would require a `drop` if `SmallNat` were `TObj`;
  -- refinement-proved `TObj -> Raw` elides every RC operation completely.
  pick-first :: SmallNat -> SmallNat -> SmallNat
  pick-first a b = a

  main :: Int
  main = if pick-first 42 99 == 42 then 42 else 1
"#;
    run_checked_with_stats("refinement_tobj_raw", source, 42);
    let ir = source_frontend::emit_ir(source).expect("emit IR");
    let fn_ir = ir
        .split("\ndefine ")
        .find(|d| {
            d.lines().next().is_some_and(|hdr| {
                (hdr.contains("Main.pick-first") || hdr.contains("Main.pick$2dfirst"))
                    && hdr.contains('{')
            })
        })
        .expect("Main.pick-first in IR");
    assert!(
        !fn_ir.contains("cinch.rc.dup.tobj")
            && !fn_ir.contains("cinch.rc.drop.tobj")
            && !fn_ir.contains("cinch.typed.rc.dup")
            && !fn_ir.contains("cinch.typed.rc.drop"),
        "bounded refined Nat parameters must be Raw with zero TObj RC ops:\n{fn_ir}"
    );
}

#[test]
fn static_uniqueness_reuse_and_shared_safety() {
    let source = r#"module Main where
  data Pair where
    Pair :: I64 -> I64 -> Pair

  bump :: Pair -> Pair
  bump p = case p of
    Pair a b -> Pair (wrapping-add-i64 a 1) (wrapping-add-i64 b 2)

  sum-pair :: Pair -> I64
  sum-pair p = case p of
    Pair a b -> wrapping-add-i64 a b

  main :: Int
  main =
    let fresh = bump (Pair 10 20) in
    let shared = Pair 1 2 in
    let copy1 = bump shared in
    let copy2 = bump shared in
    let s1 = wrapping-add-i64 (sum-pair copy1) (sum-pair copy2) in
    let s2 = wrapping-add-i64 (sum-pair fresh) s1 in
    let total = wrapping-sub-i64 s2 (sum-pair shared) in
    if total == 42 then 42 else 1
"#;
    run_checked_with_stats("static_uniqueness_reuse", source, 42);
}

#[test]
fn small_sum_csr_and_partial_escape_split() {
    let source = r#"module Main where
  safe-step :: I64 -> Option I64
  safe-step n = if n == 0 then None else Some (wrapping-sub-i64 n 1)

  go :: I64 -> I64 -> I64
  go n acc = case safe-step n of
    None -> acc
    Some prev -> go prev (wrapping-add-i64 acc 1)

  main :: Int
  main = if go 2000 0 == 2000 then 42 else 1
"#;
    let (blocks, _) = run_checked_with_stats("small_sum_csr", source, 42);
    assert!(
        blocks < 100,
        "2000-iteration Option loop with CSR should allocate zero Option blocks, got {blocks}"
    );
}

#[test]
fn product_accumulator_loop_unboxing() {
    let source = r#"module Main where
  data Acc where
    Acc :: I64 -> I64 -> Acc

  run :: Acc -> I64 -> I64
  run a n = case a of
    Acc x y ->
      if n == 0 then wrapping-add-i64 x y
      else run (Acc (wrapping-add-i64 x 1) (wrapping-add-i64 y 2)) (wrapping-sub-i64 n 1)

  main :: Int
  main = if run (Acc 0 0) 2000 == 6000 then 42 else 1
"#;
    let (blocks, _) = run_checked_with_stats("product_acc_unbox", source, 42);
    assert!(
        blocks < 100,
        "2000-iteration product accumulator loop should unbox Acc, got {blocks} blocks"
    );
}
