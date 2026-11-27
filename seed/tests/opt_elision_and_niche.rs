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

#[test]
fn auto_niche_default_without_attribute() {
    let source = r#"module Main where
  type NonNegI64 = refine (x :: I64) with x >= 0
  type PosI64 = refine (x :: I64) with x > 0
  type NegI64 = refine (x :: I64) with x < 0

  -- Unannotated closed sums: all eligible shapes auto-niche by default.
  data Lookup where
    Found :: NonNegI64 -> Lookup
    NotFound :: Lookup
    Timeout :: Lookup

  data TriSign where
    Neg :: NegI64 -> TriSign
    Zero :: TriSign
    Pos :: PosI64 -> TriSign

  data Color where
    Red :: Color
    Green :: Color
    Blue :: Color

  data OptColor where
    NoColor :: OptColor
    SomeColor :: Color -> OptColor

  data OptStr where
    NoStr :: OptStr
    HasStr :: String -> OptStr

  data OptOptStr where
    OuterNone :: OptOptStr
    OuterSome :: OptStr -> OptOptStr

  -- Unannotated non-nicheable sums silently fall back to boxed uniform-word.
  data MaybeI64 where
    NothingI64 :: MaybeI64
    JustI64 :: I64 -> MaybeI64

  data Peano where
    Z :: Peano
    S :: Peano -> Peano

  data TwoField where
    EmptyTwo :: TwoField
    PairTwo :: I64 -> I64 -> TwoField

  step-lookup :: I64 -> Lookup
  step-lookup i =
    if i < 0 then NotFound
    else if i == 1 then Timeout
    else Found i

  score-lookup :: Lookup -> I64
  score-lookup r = case r of
    Found v -> v
    NotFound -> 10
    Timeout -> 20

  classify :: I64 -> TriSign
  classify x =
    if x < 0 then Neg x
    else if x == 0 then Zero
    else Pos x

  score-sign :: TriSign -> I64
  score-sign s = case s of
    Neg n -> wrapping-sub-i64 0 n
    Zero -> 100
    Pos p -> p

  color-code :: OptColor -> I64
  color-code c = case c of
    NoColor -> 0
    SomeColor Red -> 1
    SomeColor Green -> 2
    SomeColor Blue -> 3

  loop-auto :: I64 -> I64 -> I64
  loop-auto i acc =
    if i <= 0 then acc
    else
      let a = score-lookup (step-lookup (if i == 1 then wrapping-sub-i64 0 1 else if i == 2 then 1 else i)) in
      let b = score-sign (classify (wrapping-sub-i64 i 1000)) in
      let c = color-code (if i == 5 then NoColor else SomeColor Green) in
      loop-auto (wrapping-sub-i64 i 1) (wrapping-add-i64 acc (wrapping-add-i64 a (wrapping-add-i64 b c)))

  peano-to-i64 :: Peano -> I64
  peano-to-i64 p = case p of
    Z -> 0
    S rest -> wrapping-add-i64 1 (peano-to-i64 rest)

  check-nested-obj :: OptOptStr -> I64
  check-nested-obj o = case o of
    OuterNone -> 1
    OuterSome NoStr -> 2
    OuterSome (HasStr _) -> 3

  check-fallbacks :: I64
  check-fallbacks =
    let m = case JustI64 7 of NothingI64 -> 0; JustI64 x -> x in
    let p = peano-to-i64 (S (S (S Z))) in
    let t = case PairTwo 3 4 of EmptyTwo -> 0; PairTwo x y -> wrapping-add-i64 x y in
    let n1 = check-nested-obj OuterNone in
    let n2 = check-nested-obj (OuterSome NoStr) in
    let n3 = check-nested-obj (OuterSome (HasStr "hi")) in
    wrapping-add-i64 (wrapping-add-i64 m p) (wrapping-add-i64 t (wrapping-add-i64 n1 (wrapping-add-i64 n2 n3)))

  main :: Int
  main =
    let total = loop-auto 2000 0 in
    let fb = check-fallbacks in
    if total > 0 && fb == 23 then 42 else 1
"#;
    let (blocks, _) = run_checked_with_stats("auto_niche_default", source, 42);
    assert!(
        blocks < 100,
        "2000-iteration loop over unannotated auto-niched sums should not allocate per iteration, got {blocks} blocks"
    );
}

#[test]
fn tier1_pointer_tagged_multi_ptr_and_mixed_json_sums() {
    let source = r#"module Main where
  -- 2-way pointer-tagged Obj sum (`Ok` = tag 0, `Err` = tag 2), 1 word TObj.
  data StrResult where
    Err :: String -> StrResult
    Ok :: Bytes -> StrResult

  -- Nested Option around pointer-tagged sum (`NoRes` = 0, `HasRes` = inner).
  data OptStrResult where
    NoRes :: OptStrResult
    HasRes :: StrResult -> OptStrResult

  -- 6-variant mixed sum: 3 pointer arms (`JStr`=0, `JArr`=2, `JObj`=4) + 2 SmallRaw odd arms + nullary `0`!
  data MiniJson where
    JNull :: MiniJson
    JBool :: Bool -> MiniJson
    JCode :: I32 -> MiniJson
    JStr :: String -> MiniJson
    JArr :: List String -> MiniJson
    JObj :: List (String, String) -> MiniJson

  -- All-SmallRaw shifted-tag sum (`I32` + `Bool` + nullary -> 1 word Raw).
  data SmallEither where
    SEmpty :: SmallEither
    SCode :: I32 -> SmallEither
    SFlag :: Bool -> SmallEither

  score-res :: OptStrResult -> I64
  score-res o = case o of
    NoRes -> 1
    HasRes (Ok b) -> 2
    HasRes (Err s) -> 3

  score-json :: MiniJson -> I64
  score-json j = case j of
    JNull -> 10
    JBool b -> if b then 20 else 21
    JCode c -> if c == wrapping-sub-i32 0 42 then 30 else 31
    JStr _ -> 40
    JArr _ -> 50
    JObj _ -> 60

  score-small :: SmallEither -> I64
  score-small x = case x of
    SEmpty -> 1
    SCode c -> if c == wrapping-sub-i32 0 7 then 2 else 99
    SFlag b -> if b then 3 else 4

  loop-ptrs :: I64 -> String -> Bytes -> List String -> List (String, String) -> I64 -> I64
  loop-ptrs i s b xs ps acc =
    if i <= 0 then acc
    else
      let r = wrapping-add-i64 (score-res (HasRes (Ok b))) (wrapping-add-i64 (score-res (HasRes (Err s))) (score-res NoRes)) in
      let j1 = wrapping-add-i64 (score-json JNull) (wrapping-add-i64 (score-json (JBool True)) (score-json (JCode (wrapping-sub-i32 0 42)))) in
      let j2 = wrapping-add-i64 (score-json (JStr s)) (wrapping-add-i64 (score-json (JArr xs)) (score-json (JObj ps))) in
      let sm = wrapping-add-i64 (score-small SEmpty) (wrapping-add-i64 (score-small (SCode (wrapping-sub-i32 0 7))) (score-small (SFlag True))) in
      let sum = wrapping-add-i64 r (wrapping-add-i64 (wrapping-add-i64 j1 j2) sm) in
      loop-ptrs (wrapping-sub-i64 i 1) s b xs ps (wrapping-add-i64 acc sum)

  main :: Int
  main =
    let s = "hello" in
    let b = string-to-bytes "world" in
    let xs = ["a", "b"] in
    let ps = [("k", "v")] in
    let total = loop-ptrs 2000 s b xs ps 0 in
    -- Per iteration: (2+3+1) + (10+20+30+40+50+60) + (1+2+3) = 6 + 210 + 6 = 222.
    if total == 444000 then 42 else 1
"#;
    let (blocks, _) = run_checked_with_stats("tier1_pointer_tagged_json", source, 42);
    assert!(
        blocks < 100,
        "2000-iteration loop over 1-word pointer-tagged StrResult/MiniJson/SmallEither must not allocate wrappers, got {blocks} blocks"
    );
}

#[test]
fn tier3_nan_sentinel_f64_and_f32_sums() {
    let source = r#"module Main where
  data MaybeF64 where
    NoF64 :: MaybeF64
    HasF64 :: F64 -> MaybeF64

  data OptOptF64 where
    OuterNoF64 :: OptOptF64
    OuterHasF64 :: MaybeF64 -> OptOptF64

  data MaybeF32 where
    NoF32 :: MaybeF32
    HasF32 :: F32 -> MaybeF32

  eval-f64 :: OptOptF64 -> I64
  eval-f64 o = case o of
    OuterNoF64 -> 1
    OuterHasF64 NoF64 -> 2
    OuterHasF64 (HasF64 x) -> if feq-f64 x 3.5 then 3 else if feq-f64 x 0.0 then 4 else if is-na-nf64 x then 5 else 99

  eval-f32 :: MaybeF32 -> I64
  eval-f32 o = case o of
    NoF32 -> 10
    HasF32 x -> if feq-f32 x 1.5 then 20 else 99

  loop-f64 :: I64 -> F64 -> I64 -> I64
  loop-f64 i nan-v acc =
    if i <= 0 then acc
    else
      let a = eval-f64 OuterNoF64 in
      let b = eval-f64 (OuterHasF64 NoF64) in
      let c = eval-f64 (OuterHasF64 (HasF64 3.5)) in
      let d = eval-f64 (OuterHasF64 (HasF64 (sub-f64 0.0 0.0))) in
      let n = eval-f64 (OuterHasF64 (HasF64 nan-v)) in
      let e = eval-f32 (HasF32 1.5) in
      let f = eval-f32 NoF32 in
      let step = wrapping-add-i64 (wrapping-add-i64 (wrapping-add-i64 a b) n) (wrapping-add-i64 (wrapping-add-i64 c d) (wrapping-add-i64 e f)) in
      loop-f64 (wrapping-sub-i64 i 1) nan-v (wrapping-add-i64 acc step)

  main :: Int
  main =
    let nan-v = div-f64 0.0 0.0 in
    -- Per iteration: 1 + 2 + 3 + 4 + 5 + 20 + 10 = 45.
    if loop-f64 2000 nan-v 0 == 90000 then 42 else 1
"#;
    let (blocks, _) = run_checked_with_stats("tier3_nan_f64", source, 42);
    assert!(
        blocks < 100,
        "2000-iteration loop over NaN-boxed F64/F32 sums must not allocate, got {blocks} blocks"
    );
}

#[test]
fn tier1_polymorphic_outcome_and_maybe_sums() {
    let source = r#"module Main where
  -- Polymorphic Result-like and Option-like user sums:
  -- fuse to 1-word pointer-tagged TObj when payload params specialize to Obj,
  -- and fall back cleanly to boxed Obj when instantiated with Raw (e.g. I64).
  data Outcome e a where
    Fail :: e -> Outcome e a
    Good :: a -> Outcome e a

  data Maybe a where
    Nothing :: Maybe a
    Just :: a -> Maybe a

  data Lookup a where
    Missing :: Lookup a
    Denied :: Lookup a
    Found :: a -> Lookup a

  map-good :: forall e a b. (a -> b) -> Outcome e a -> Outcome e b
  map-good f r = case r of
    Fail e -> Fail e
    Good x -> Good (f x)

  unwrap-maybe :: forall a. a -> Maybe a -> a
  unwrap-maybe d m = case m of
    Nothing -> d
    Just x -> x

  score-lookup :: forall a. (a -> I64) -> Lookup a -> I64
  score-lookup f l = case l of
    Missing -> 1
    Denied -> 2
    Found x -> f x

  tag-outcome :: forall e a. Outcome e a -> I64
  tag-outcome r = case r of
    Fail _ -> 10
    Good _ -> 20

  loop-poly :: I64 -> String -> Bytes -> I64 -> I64
  loop-poly i s b acc =
    if i <= 0 then acc
    else
      let o1 = map-good (\x -> x) (Good b :: Outcome String Bytes) in
      let o2 = map-good (\x -> x) (Fail s :: Outcome String Bytes) in
      let m1 = unwrap-maybe s (Just s) in
      let m2 = unwrap-maybe s Nothing in
      let l1 = score-lookup (\_ -> 3) (Missing :: Lookup Bytes) in
      let l2 = score-lookup (\_ -> 3) (Denied :: Lookup Bytes) in
      let l3 = score-lookup (\_ -> 3) (Found b) in
      let t = wrapping-add-i64 (tag-outcome o1) (tag-outcome o2) in
      let ls = wrapping-add-i64 l1 (wrapping-add-i64 l2 l3) in
      let ok-m = if m1 == m2 then 1 else 0 in
      loop-poly (wrapping-sub-i64 i 1) s b (wrapping-add-i64 acc (wrapping-add-i64 t (wrapping-add-i64 ls ok-m)))

  main :: Int
  main =
    let s = "err" in
    let b = string-to-bytes "ok" in
    let total = loop-poly 2000 s b 0 in
    -- Also test boxed fallback (Raw payload) AND cross-representation conversion (Raw -> Obj)!
    let raw-to-obj = map-good (\_ -> b) (Good (42 :: I64) :: Outcome String I64) in
    let raw-fail = map-good (\_ -> b) (Fail s :: Outcome String I64) in
    let nested = case Just (Just s) of Nothing -> 0; Just Nothing -> 1; Just (Just _) -> 2 in
    -- Per iteration: (20+10) + (1+2+3) + 1 = 37 -> 2000 * 37 = 74000.
    if total == 74000 && tag-outcome raw-to-obj == 20 && tag-outcome raw-fail == 10 && nested == 2 then 42 else 1
"#;
    let (blocks, _) = run_checked_with_stats("tier1_poly_sums", source, 42);
    assert!(
        blocks < 100,
        "2000-iteration loop over polymorphic Outcome/Maybe/Lookup @Obj must not allocate wrappers, got {blocks} blocks"
    );
}
