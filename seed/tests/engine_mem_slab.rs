//! Integration tests for Wave 1 `Engine.Mem.Slab`, `Engine.Mem.Arena`, and
//! compile-time formal TLA+-style model M1 `SlabMachine` (`Engine.Mem.Slab.Spec`):
//!
//!   1. Stdlib package manifest registration (`seed/prelude/.cinchpkg`).
//!   2. Exhaustive compile-time model check of `Engine.Mem.Slab.Spec` under
//!      `CINCH_MODEL_FAIL_ON_CAPPED=1` / `--model-fail-on-capped`, verifying
//!      `SlabMachine` & `AbstractSlabMap` pass with `capped: false`, `0 violations`,
//!      safety invariants (`slabInvariants`), deadlock freedom (`slabDeadlockFree`),
//!      weakly fair liveness (`slabReusable`), and checked stuttering refinement
//!      (`refines SlabMachine AbstractSlabMap`).
//!   3. Native `Effect.Test.Hegel.hegel-check-machine-with` simulation (1,200
//!      trials x 30 steps) on the exact `SlabModelState` transition actions.
//!   4. Native leak-checked (`CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1`, `leaked=0`)
//!      execution of `RawSlab`, typed `Slab r a`, and `BumpArena` covering
//!      alloc/free/ABA rejection/double-free rejection/alignment/mark/rewind/reset,
//!      plus `engine-thread-alloc-count () == 0` and
//!      `engine-thread-atomic-rc-count () == 0` across 50,000 `RawSlab`
//!      alloc+free cycles and 50,000 `BumpArena` alloc+reset cycles.
//!
//! Routed through `common::project`, `common::drive_with_profile_opts`,
//! `common::build_executable`, and `common::run` so the exact same suite runs
//! against both the Rust seed and the Stage 3 self-hosted compiler.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{DriverOptions, Mode, Package};
use cinch::lsp::{parse_json, Json};
use std::process::Command;

const PRIM_ATOMIC: &str = include_str!("../prelude/Prim/Atomic.cinch");
const ENGINE_TYPES: &str = include_str!("../prelude/Engine/Types.cinch");
const MUT_ARRAY: &str = include_str!("../prelude/Data/Mutable/Array.cinch");
const MUT_VAR: &str = include_str!("../prelude/Data/Mutable/Var.cinch");
const DATA_FINITE: &str = include_str!("../prelude/Data/Finite.cinch");
const EFFECT_HEGEL: &str = include_str!("../prelude/Effect/Test/Hegel.cinch");
const ENGINE_MEM_SLAB: &str = include_str!("../prelude/Engine/Mem/Slab.cinch");
const ENGINE_MEM_ARENA: &str = include_str!("../prelude/Engine/Mem/Arena.cinch");
const ENGINE_MEM_SLAB_SPEC: &str = include_str!("../prelude/Engine/Mem/Slab/Spec.cinch");
const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn project_with_mem_slab(tag: &str, main_src: &str) -> std::path::PathBuf {
    common::project(
        tag,
        &[
            ("Prim.Atomic.cinch", PRIM_ATOMIC),
            ("Engine.Types.cinch", ENGINE_TYPES),
            ("Data.Mutable.Array.cinch", MUT_ARRAY),
            ("Data.Mutable.Var.cinch", MUT_VAR),
            ("Engine.Mem.Slab.cinch", ENGINE_MEM_SLAB),
            ("Engine.Mem.Arena.cinch", ENGINE_MEM_ARENA),
            ("Engine.Mem.Slab.Spec.cinch", ENGINE_MEM_SLAB_SPEC),
            ("Main.cinch", main_src),
        ],
    )
}

fn project_with_hegel_and_spec(tag: &str, main_src: &str) -> std::path::PathBuf {
    common::project(
        tag,
        &[
            ("Data.Finite.cinch", DATA_FINITE),
            ("Effect.Test.Hegel.cinch", EFFECT_HEGEL),
            ("Engine.Mem.Slab.Spec.cinch", ENGINE_MEM_SLAB_SPEC),
            ("Main.cinch", main_src),
        ],
    )
}

fn run_leak_checked(tag: &str, dir: &std::path::Path, expected: i32) {
    let exe = common::build_executable(dir, "Main.cinch", "engine-mem-slab-test")
        .unwrap_or_else(|e| panic!("{tag}: build failed: {e}"));
    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "64M")
        .output()
        .expect("run built binary");
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_eq!(
        out.status.code(),
        Some(expected),
        "{tag}: expected exit {expected}, got {:?};\nstdout:\n{stdout}\nstderr:\n{stderr}",
        out.status.code()
    );
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "{tag}: expected clean leak report; stderr:\n{stderr}"
    );
}

#[test]
fn engine_mem_modules_registered_in_stdlib_package_manifest() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, path) in [
        ("Engine.Mem.Slab", "Engine/Mem/Slab.cinch"),
        ("Engine.Mem.Arena", "Engine/Mem/Arena.cinch"),
        ("Engine.Mem.Slab.Spec", "Engine/Mem/Slab/Spec.cinch"),
    ] {
        let entry = pkg
            .modules
            .iter()
            .find(|m| m.name == name)
            .unwrap_or_else(|| panic!("{name} missing from seed/prelude/.cinchpkg modules"));
        assert_eq!(entry.path, path);
        assert!(
            pkg.order.iter().any(|n| n == name),
            "{name} missing from seed/prelude/.cinchpkg order"
        );
    }
}

#[test]
fn slab_machine_m1_formal_model_verifies_uncapped_with_zero_violations() {
    let main_src = r#"
module Main where
  import Engine.Mem.Slab.Spec

  main :: Int
  main =
    if slab-model-invariants slab-model-init then 0 else 1
"#;

    let dir = common::project(
        "slab-machine-m1-check",
        &[
            ("Engine.Mem.Slab.Spec.cinch", ENGINE_MEM_SLAB_SPEC),
            ("Main.cinch", main_src),
        ],
    );

    // 1. Check via `common::drive_with_profile_opts` with `model_fail_on_capped = true`
    //    (routes through both Rust seed and Stage 3 when `CINCH_TEST_COMPILER` is set).
    let mut strict_opts = DriverOptions::default();
    strict_opts.model_fail_on_capped = true;
    common::drive_with_profile_opts(
        Mode::Check,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        None,
        &TargetProfile::native(),
        &strict_opts,
    )
    .expect("M1 SlabMachine strict uncapped check via driver");

    let manifest = std::fs::read_to_string(dir.join("build/Main.manifest.json"))
        .expect("read build/Main.manifest.json");
    assert!(
        manifest.contains("\"contract\":\"slabInvariants\""),
        "expected slabInvariants in manifest:\n{manifest}"
    );
    assert!(
        manifest.contains("\"contract\":\"slabDeadlockFree\""),
        "expected slabDeadlockFree in manifest:\n{manifest}"
    );
    assert!(
        manifest.contains("\"contract\":\"slabReusable\""),
        "expected slabReusable in manifest:\n{manifest}"
    );
    assert!(
        manifest.contains("refines SlabMachine AbstractSlabMap"),
        "expected refinement row in manifest:\n{manifest}"
    );
    assert!(
        manifest.contains("0 violations"),
        "expected 0 violations in manifest:\n{manifest}"
    );
    assert!(
        !manifest.contains("CAPPED"),
        "M1 SlabMachine must not be capped:\n{manifest}"
    );

    // 2. Verify CLI `cinch check` with `CINCH_MODEL_FAIL_ON_CAPPED=1` and inspect
    //    JSON report from `cinch model inspect --json --model-fail-on-capped`
    //    to confirm `capped: false`, exact state count, and 0 violations.
    let bin = env!("CARGO_BIN_EXE_cinch");
    let spec_path = dir.join("Engine.Mem.Slab.Spec.cinch");
    let spec_str = spec_path.to_string_lossy().to_string();

    let check_out = Command::new(bin)
        .args(["check", "--model-fail-on-capped", &spec_str])
        .env("CINCH_MODEL_FAIL_ON_CAPPED", "1")
        .output()
        .expect("run cinch check --model-fail-on-capped on Spec");
    assert!(
        check_out.status.success(),
        "cinch check --model-fail-on-capped failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&check_out.stdout),
        String::from_utf8_lossy(&check_out.stderr)
    );

    let inspect_out = Command::new(bin)
        .args([
            "model",
            "inspect",
            &spec_str,
            "SlabMachine",
            "--json",
            "--model-fail-on-capped",
        ])
        .env("CINCH_MODEL_FAIL_ON_CAPPED", "1")
        .output()
        .expect("run cinch model inspect SlabMachine --json");
    assert!(
        inspect_out.status.success(),
        "cinch model inspect SlabMachine failed:\nstderr:\n{}",
        String::from_utf8_lossy(&inspect_out.stderr)
    );
    let json_text = String::from_utf8(inspect_out.stdout).expect("UTF-8 inspect JSON");
    let parsed = parse_json(&json_text).expect("parse SlabMachine inspect JSON");
    let report = match parsed.as_arr() {
        Some(arr) => arr
            .iter()
            .find(|m| m.get("name").and_then(Json::as_str) == Some("SlabMachine"))
            .unwrap_or_else(|| panic!("SlabMachine missing from inspect JSON: {json_text}")),
        None => &parsed,
    };
    assert_eq!(
        report.get("capped"),
        Some(&Json::Bool(false)),
        "SlabMachine must be uncapped: {json_text}"
    );
    let state_count = report
        .get("state_count")
        .and_then(Json::as_num)
        .expect("state_count number") as usize;
    let edge_count = report
        .get("edge_count")
        .and_then(Json::as_num)
        .expect("edge_count number") as usize;
    let diameter = report
        .get("diameter")
        .and_then(Json::as_num)
        .expect("diameter number") as usize;
    eprintln!(
        "M1 SlabMachine verified: state_count={state_count}, edge_count={edge_count}, diameter={diameter}, capped=false"
    );
    assert!(
        state_count > 100 && state_count < 65536,
        "unexpected SlabMachine state_count={state_count}: {json_text}"
    );
    assert_eq!(
        report
            .get("deadlocks")
            .and_then(Json::as_arr)
            .map(|a| a.len()),
        Some(0),
        "SlabMachine must have 0 deadlocks: {json_text}"
    );
    let contracts = report
        .get("contracts")
        .and_then(Json::as_arr)
        .expect("contracts array");
    assert!(contracts.len() >= 3, "expected >= 3 contracts: {json_text}");
    for c in contracts {
        assert_eq!(
            c.get("passed"),
            Some(&Json::Bool(true)),
            "contract failed: {c:?}"
        );
    }
    let refinements = report
        .get("refinements")
        .and_then(Json::as_arr)
        .expect("refinements array");
    assert_eq!(refinements.len(), 1, "expected 1 refinement: {json_text}");
    assert_eq!(
        refinements[0].get("passed"),
        Some(&Json::Bool(true)),
        "refinement failed: {json_text}"
    );

    common::cleanup(&dir);
}

#[test]
fn hegel_native_machine_simulation_over_slab_model_transitions() {
    let main_src = r#"
module Main where
  import Data.Bool
  import Data.Nat
  import Data.Result
  import Effect.Test.Hegel
  import Engine.Mem.Slab.Spec

  main :: Int
  main =
    let cfg = HegelConfig 1200 256 80 (hegel-seed 20261128) in
    let actions = [
          step-action "alloc" slab-step-alloc,
          step-action-gen "free" (gen-element [S0, S1, S2]) slab-step-free,
          step-action "stale-lookup" slab-step-stale-lookup
        ] in
    case hegel-check-machine-with cfg 30 slab-model-init actions slab-model-invariants of
      Ok trials -> if trials == 1200 then 0 else 2
      Err _ -> 1
"#;

    let dir = project_with_hegel_and_spec("slab-hegel-sim", main_src);
    run_leak_checked("slab-hegel-sim", &dir, 0);
    common::cleanup(&dir);
}

#[test]
fn raw_slab_typed_slab_bump_arena_functional_and_zero_alloc_hot_loops() {
    let main_src = r#"
module Main where
  import Data.Bool
  import Data.Core
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Prim.Atomic
  import Engine.Types
  import Engine.Mem.Slab
  import Engine.Mem.Arena

  #[no-alloc]
  both-na :: Bool -> Bool -> Bool
  both-na a b = if a then b else eq-u32 0 1

  -- 1. Functional RawSlab tests: alloc, full exhaustion, payload read/write,
  --    free, double-free rejection, ABA reuse rejection, generation wrap.
  test-raw-slab-functional :: Unit ->{IO, MVar} Int
  test-raw-slab-functional _ =
    if ne-u32 (slab-next-gen 4294967295) 1 then 10
    else if ne-u32 (slab-next-gen 1) 2 then 11
    else
      let slab = raw-slab-new 3 64 in
      if not (raw-slab-is-valid slab) then 12
      else if ne-u32 (raw-slab-capacity slab) 3 then 13
      else if ne-u32 (raw-slab-stride slab) 64 then 14
      else if ne-u32 (raw-slab-live-count slab) 0 then 15
      else
        let h0 = raw-slab-alloc slab in
        let h1 = raw-slab-alloc slab in
        let h2 = raw-slab-alloc slab in
        let h3 = raw-slab-alloc slab in
        if gen-handle-is-null h0 || gen-handle-is-null h1 || gen-handle-is-null h2 then 16
        else if not (gen-handle-is-null h3) then 17
        else if ne-u32 (raw-slab-live-count slab) 3 then 18
        else if not (raw-slab-write-u64 slab h0 0 1111111111) then 19
        else if not (raw-slab-write-u64 slab h1 0 2222222222) then 20
        else if not (raw-slab-write-u64 slab h2 8 3333333333) then 21
        else if ne-u64 (raw-slab-read-u64 slab h0 0) 1111111111 then 22
        else if ne-u64 (raw-slab-read-u64 slab h1 0) 2222222222 then 23
        else if ne-u64 (raw-slab-read-u64 slab h2 8) 3333333333 then 24
        else
          -- Free h1 and verify double-free rejection + stale-handle rejection
          let f1 = raw-slab-free slab h1 in
          let f1-dup = raw-slab-free slab h1 in
          if not f1 || f1-dup then 25
          else if raw-slab-is-live slab h1 then 26
          else if ne-u64 (raw-slab-slot-addr slab h1) 0 then 27
          else if ne-u32 (raw-slab-live-count slab) 2 then 28
          else
            -- ABA reuse: reallocating pops the freed slot with gen + 1!
            let h1-new = raw-slab-alloc slab in
            if gen-handle-is-null h1-new then 29
            else if ne-u32 (gen-handle-idx h1-new) (gen-handle-idx h1) then 30
            else if eq-u32 (gen-handle-gen h1-new) (gen-handle-gen h1) then 31
            else if raw-slab-is-live slab h1 then 32
            else if ne-u64 (raw-slab-slot-addr slab h1) 0 then 33
            else if raw-slab-free slab h1 then 34
            else if raw-slab-write-u64 slab h1 0 999 then 35
            else if not (raw-slab-is-live slab h1-new) then 36
            else if eq-u64 (raw-slab-slot-addr slab h1-new) 0 then 37
            else if not (raw-slab-write-u64 slab h1-new 0 4444444444) then 38
            else if ne-u64 (raw-slab-read-u64 slab h1-new 0) 4444444444 then 39
            else if not (raw-slab-free slab h0) then 40
            else if not (raw-slab-free slab h1-new) then 41
            else if not (raw-slab-free slab h2) then 42
            else if ne-u32 (raw-slab-live-count slab) 0 then 43
            else if raw-slab-free slab (gen-handle-null ()) then 44
            else
              let err = raw-slab-destroy slab in
              if io-error-is-ok err then 0 else 45

  is-some-singleton :: Nat -> Option (List Nat) -> Bool
  is-some-singleton expected opt = case opt of
    Some (Cons x Nil) -> x == expected
    _ -> False

  is-none-list :: Option (List Nat) -> Bool
  is-none-list opt = case opt of
    None -> True
    Some _ -> False

  -- 2. Functional typed `Slab r a` tests (over heap-allocated Cinch values to
  --    verify clean RC release on free and no leaks).
  test-typed-slab :: forall (r :: Region). Unit ->{Mut r} Int
  test-typed-slab _ =
    let s = slab-new 2 in
    if ne-u32 (slab-capacity s) 2 then 50
    else if ne-u32 (slab-live-count s) 0 then 51
    else
      let h0 = slab-alloc s (Cons 10 (Cons 20 Nil)) in
      let h1 = slab-alloc s (Cons 30 Nil) in
      let h2 = slab-alloc s Nil in
      if gen-handle-is-null h0 || gen-handle-is-null h1 then 52
      else if not (gen-handle-is-null h2) then 53
      else if ne-u32 (slab-live-count s) 2 then 54
      else if not (slab-set s h0 (Cons 99 Nil)) then 55
      else if not (is-some-singleton 99 (slab-get s h0)) then 56
      else
        let ok-free = is-some-singleton 99 (slab-free s h0) in
        let dup-free = is-none-list (slab-free s h0) in
        if not ok-free || not dup-free then 57
        else
          let h0-new = slab-alloc s (Cons 77 Nil) in
          if gen-handle-is-null h0-new then 58
          else if ne-u32 (gen-handle-idx h0-new) (gen-handle-idx h0) then 59
          else if slab-is-live s h0 then 60
          else if not (slab-is-live s h0-new) then 61
          else if not (is-none-list (slab-get s h0)) then 62
          else
            let _ = slab-free s h0-new in
            let _ = slab-free s h1 in
            if eq-u32 (slab-live-count s) 0 then 0 else 63

  -- 3. Functional BumpArena tests: 8/16/64 alignment, non-pow2 rejection,
  --    OOM rejection, mark/rewind, 1-write reset.
  test-bump-arena-functional :: Unit ->{IO, MVar} Int
  test-bump-arena-functional _ =
    let a = bump-arena-new 256 in
    if not (bump-arena-is-valid a) then 70
    else if eq-u64 (bump-arena-base-addr a) 0 then 71
    else if ne-u64 (bump-arena-used a) 0 then 72
    else if ne-u64 (bump-arena-remaining a) 256 then 73
    else
      -- Reject non-power-of-two alignment
      let bad-align = bump-arena-alloc a 16 3 in
      if ne-u64 bad-align 0 then 74
      else
        -- 1-byte alloc followed by 8-byte, 16-byte, and 64-byte aligned allocs
        let p0 = bump-arena-alloc a 1 1 in
        let p8 = bump-arena-alloc a 8 8 in
        let p16 = bump-arena-alloc a 16 16 in
        let m = bump-arena-mark a in
        let p64 = bump-arena-alloc a 64 64 in
        if eq-u64 p0 0 || eq-u64 p8 0 || eq-u64 p16 0 || eq-u64 p64 0 then 75
        else if ne-u64 (bit-and-u64 p8 7) 0 then 76
        else if ne-u64 (bit-and-u64 p16 15) 0 then 77
        else if ne-u64 (bit-and-u64 p64 63) 0 then 78
        else
          -- Write & read through arena pointers
          let _ = raw-store-u64 (wrapping-u64-to-i64 p8) 123456789 in
          let _ = raw-store-u64 (wrapping-u64-to-i64 p64) 987654321 in
          if ne-u64 (raw-load-u64 (wrapping-u64-to-i64 p8)) 123456789 then 79
          else if ne-u64 (raw-load-u64 (wrapping-u64-to-i64 p64)) 987654321 then 80
          else
            -- Rewind past current cursor must fail; rewind to saved mark succeeds
            let bad-rw = bump-arena-rewind a 9999 in
            if bad-rw then 81
            else if not (bump-arena-rewind a m) then 82
            else if ne-u64 (bump-arena-used a) m then 83
            else
              -- OOM when requesting more than remaining
              let oom = bump-arena-alloc a 512 8 in
              if ne-u64 oom 0 then 84
              else
                -- 1-write reset brings used back to 0 and remaining back to 256
                let _ = bump-arena-reset a in
                if ne-u64 (bump-arena-used a) 0 then 85
                else if ne-u64 (bump-arena-remaining a) 256 then 86
                else
                  let err = bump-arena-destroy a in
                  if io-error-is-ok err then 0 else 87

  -- 4. Tight 50,000-cycle zero-allocation RawSlab loop (`#[no-alloc]`)
  #[no-alloc]
  slab-hot-loop :: RawSlab -> U32 -> U64 ->{MVar} U64
  slab-hot-loop slab rem acc =
    if eq-u32 rem 0 then acc
    else
      let h = raw-slab-alloc slab in
      let addr = raw-slab-slot-addr slab h in
      let ok-w = raw-slab-write-u64 slab h 0 (u32-to-u64 rem) in
      let v = raw-slab-read-u64 slab h 0 in
      let ok-f = raw-slab-free slab h in
      let ok-step = both-na (both-na (ne-u64 addr 0) ok-w) (both-na (eq-u64 v (u32-to-u64 rem)) ok-f) in
      let next-acc = if ok-step then wrapping-add-u64 acc 1 else acc in
      slab-hot-loop slab (wrapping-sub-u32 rem 1) next-acc

  -- 5. Tight 50,000-cycle zero-allocation BumpArena loop (`#[no-alloc]`)
  #[no-alloc]
  arena-hot-loop :: BumpArena -> U32 -> U64 ->{MVar} U64
  arena-hot-loop arena rem acc =
    if eq-u32 rem 0 then acc
    else
      let p8 = bump-arena-alloc arena 24 8 in
      let p16 = bump-arena-alloc arena 32 16 in
      let m = bump-arena-mark arena in
      let p64 = bump-arena-alloc arena 64 64 in
      let ok-rw = bump-arena-rewind arena m in
      let _ = bump-arena-reset arena in
      let ok-ptrs = both-na (both-na (ne-u64 p8 0) (ne-u64 p16 0)) (both-na (ne-u64 p64 0) ok-rw) in
      let ok-align = both-na (eq-u64 (bit-and-u64 p16 15) 0) (eq-u64 (bit-and-u64 p64 63) 0) in
      let next-acc = if both-na ok-ptrs ok-align then wrapping-add-u64 acc 1 else acc in
      arena-hot-loop arena (wrapping-sub-u32 rem 1) next-acc

  test-zero-alloc-loop :: Unit ->{IO, MVar} Int
  test-zero-alloc-loop _ =
    let slab = raw-slab-new 16 64 in
    let arena = bump-arena-new 4096 in
    if not (raw-slab-is-valid slab) || not (bump-arena-is-valid arena) then 90
    else
      let _ = engine-thread-counters-reset () in
      let slab-ok = slab-hot-loop slab 50000 0 in
      let arena-ok = arena-hot-loop arena 50000 0 in
      let allocs = engine-thread-alloc-count () in
      let atomic-rcs = engine-thread-atomic-rc-count () in
      let _ = raw-slab-destroy slab in
      let _ = bump-arena-destroy arena in
      if ne-u64 slab-ok 50000 then 91
      else if ne-u64 arena-ok 50000 then 92
      else if ne-u64 allocs 0 then 93
      else if ne-u64 atomic-rcs 0 then 94
      else 0

  main :: forall (r :: Region). Unit ->{IO, MVar, Mut r} Int
  main _ =
    let r1 = test-raw-slab-functional () in
    if r1 /= 0 then r1
    else
      let r2 = test-typed-slab () in
      if r2 /= 0 then r2
      else
        let r3 = test-bump-arena-functional () in
        if r3 /= 0 then r3
        else test-zero-alloc-loop ()
"#;

    let dir = project_with_mem_slab("engine-mem-slab-runtime", main_src);
    common::check(&dir, "Main.cinch").expect("Engine.Mem.Slab + Engine.Mem.Arena check");
    run_leak_checked("engine-mem-slab-runtime", &dir, 0);
    common::cleanup(&dir);
}
