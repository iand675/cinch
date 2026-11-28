//! Integration tests for `Engine.Timer` (4-level hierarchical hashed timer wheel
//! + `CoarseClockCache` RFC 9110 date formatter) and `Engine.Timer.Spec`
//! (compile-time TLA+-style formal proof M4 `TimerWheelMachine`).
//!
//! Routed through `common::project` / `common::drive_with_profile_opts` /
//! `common::build_executable` so the exact same suite runs against both the
//! Rust seed and the Stage 3 self-hosted compiler.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{DriverOptions, Mode, Package};
use std::process::Command;

const ENGINE_TYPES: &str = include_str!("../prelude/Engine/Types.cinch");
const PRIM_ATOMIC: &str = include_str!("../prelude/Prim/Atomic.cinch");
const ENGINE_TIMER: &str = include_str!("../prelude/Engine/Timer.cinch");
const ENGINE_TIMER_SPEC: &str = include_str!("../prelude/Engine/Timer/Spec.cinch");
const DATA_FINITE: &str = include_str!("../prelude/Data/Finite.cinch");
const EFFECT_TEST_HEGEL: &str = include_str!("../prelude/Effect/Test/Hegel.cinch");
const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn project_with_timer(tag: &str, main_src: &str) -> std::path::PathBuf {
    common::project(
        tag,
        &[
            ("Prim.Atomic.cinch", PRIM_ATOMIC),
            ("Engine.Types.cinch", ENGINE_TYPES),
            ("Engine.Timer.cinch", ENGINE_TIMER),
            ("Engine.Timer.Spec.cinch", ENGINE_TIMER_SPEC),
            ("Data.Finite.cinch", DATA_FINITE),
            ("Effect.Test.Hegel.cinch", EFFECT_TEST_HEGEL),
            ("Main.cinch", main_src),
        ],
    )
}

fn run_leak_checked(tag: &str, src: &str, expected: i32) {
    let dir = project_with_timer(tag, src);
    let exe = common::build_executable(&dir, "Main.cinch", "engine-timer-test")
        .unwrap_or_else(|e| panic!("{tag}: build failed: {e}"));
    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "128M")
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
    common::cleanup(&dir);
}

#[test]
fn engine_timer_and_spec_registered_in_stdlib_package_manifest() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, path) in [
        ("Engine.Timer", "Engine/Timer.cinch"),
        ("Engine.Timer.Spec", "Engine/Timer/Spec.cinch"),
    ] {
        let entry = pkg
            .modules
            .iter()
            .find(|m| m.name == name)
            .unwrap_or_else(|| panic!("{name} listed in seed/prelude/.cinchpkg modules"));
        assert_eq!(entry.path, path);
        assert!(
            pkg.order.iter().any(|m| m == name),
            "{name} listed in seed/prelude/.cinchpkg order"
        );
    }
}

#[test]
fn engine_timer_m4_formal_model_verifies_uncapped_zero_violations() {
    let main_src = r#"
module Main where
  import Engine.Timer.Spec

  main :: Int
  main =
    if tw-place-level 0 3 == 0 && tw-place-level 0 4 == 1 then 0 else 1
"#;
    let dir = common::project(
        "engine-timer-m4-spec",
        &[
            ("Engine.Timer.Spec.cinch", ENGINE_TIMER_SPEC),
            ("Main.cinch", main_src),
        ],
    );

    // 1. Check via `common::drive_with_profile_opts` with `model_fail_on_capped = true`
    // (works under both Rust seed and Stage 3 self-hosted compiler).
    let mut opts = DriverOptions::default();
    opts.model_fail_on_capped = true;
    common::drive_with_profile_opts(
        Mode::Check,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        None,
        &TargetProfile::native(),
        &opts,
    )
    .expect("M4 TimerWheelMachine check must succeed with model_fail_on_capped=true");

    let manifest = std::fs::read_to_string(dir.join("build/Main.manifest.json"))
        .expect("read build/Main.manifest.json");
    for needle in [
        "\"contract\":\"noEarlyFireAndExactCount\"",
        "\"contract\":\"timerDeadlockFree\"",
        "\"contract\":\"allTimersFire\"",
        "\"contract\":\"allActiveDrain\"",
        "refines TimerWheelMachine AbstractTimerSet",
        "measure strictly decreasing",
        "0 violations",
    ] {
        assert!(
            manifest.contains(needle),
            "expected manifest to contain {needle:?}, got:\n{manifest}"
        );
    }
    assert!(
        !manifest.contains("[CAPPED") && !manifest.contains("capped: true"),
        "M4 must not be capped; manifest:\n{manifest}"
    );

    // 2. Also verify via `cinch model inspect --json` with `CINCH_MODEL_FAIL_ON_CAPPED=1`
    // on `Engine.Timer.Spec.cinch` directly and assert `capped: false` & 0 violations.
    let spec_file = dir.join("Engine.Timer.Spec.cinch");
    let bin = env!("CARGO_BIN_EXE_cinch");
    let inspect = Command::new(bin)
        .args([
            "model",
            "inspect",
            spec_file.to_str().expect("utf8 path"),
            "TimerWheelMachine",
            "--json",
        ])
        .env("CINCH_MODEL_FAIL_ON_CAPPED", "1")
        .output()
        .expect("run cinch model inspect");
    let inspect_stdout = String::from_utf8_lossy(&inspect.stdout);
    let inspect_stderr = String::from_utf8_lossy(&inspect.stderr);
    assert!(
        inspect.status.success(),
        "cinch model inspect failed:\nstdout:\n{inspect_stdout}\nstderr:\n{inspect_stderr}"
    );
    let json: serde_json::Value = serde_json::from_str(&inspect_stdout)
        .unwrap_or_else(|e| panic!("invalid JSON from model inspect: {e}\n{inspect_stdout}"));
    let machine_obj = json
        .get("machine")
        .or_else(|| json.as_array().and_then(|a| a.first()))
        .unwrap_or(&json);
    assert_eq!(
        machine_obj.get("capped").and_then(|v| v.as_bool()),
        Some(false),
        "TimerWheelMachine must have capped: false; json:\n{inspect_stdout}"
    );
    let states = machine_obj
        .get("state_count")
        .and_then(|v| v.as_u64())
        .expect("state_count field in model inspect json");
    let edges = machine_obj
        .get("edge_count")
        .and_then(|v| v.as_u64())
        .expect("edge_count field in model inspect json");
    eprintln!(
        "M4 TimerWheelMachine verified: state_count={states}, edge_count={edges}, capped=false"
    );
    assert!(
        states > 0 && states < 65536,
        "unexpected state count {states}; json:\n{inspect_stdout}"
    );

    common::cleanup(&dir);
}

#[test]
fn engine_timer_hegel_differential_1000_traces() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.NonZero
  import Data.Option
  import Data.Result
  import Effect.Test.Hegel
  import Engine.Types
  import Engine.Timer
  import Prim.Atomic

  -- Reference priority queue / active timer list entry:
  --   RefEntry id deadline raw-handle
  data RefEntry where
    RefEntry :: U32 -> U64 -> U64 -> RefEntry

  -- Random differential operation generated by Effect.Test.Hegel choice streams.
  data DiffOp where
    OpSchedule :: Nat -> DiffOp           -- schedule with delta in [0..70000] ms
    OpCancel :: Nat -> DiffOp             -- cancel idx-th active timer (or stale)
    OpReschedule :: Nat -> Nat -> DiffOp  -- reschedule idx-th active timer with new delta
    OpAdvance :: Nat -> DiffOp            -- advance clock by step in [0..20000] ms

  -- Generate realistic multi-level delays covering L0, L1, L2, and past/immediate.
  gen-delay :: Gen Nat
  gen-delay =
    gen-one-of [
      gen-nat 0,
      gen-range 1 255,
      gen-range 256 1024,
      gen-range 256 65535,
      gen-range 65536 140000
    ]

  gen-step :: Gen Nat
  gen-step =
    gen-one-of [
      gen-nat 0,
      gen-range 1 32,
      gen-range 1 300,
      gen-range 256 8000,
      gen-range 65536 90000
    ]

  gen-op :: Gen DiffOp
  gen-op =
    gen-one-of [
      gen-map OpSchedule gen-delay,
      gen-map OpSchedule gen-delay,
      gen-map OpCancel (gen-nat 15),
      gen-bind (gen-nat 15) (\i -> gen-map (OpReschedule i) gen-delay),
      gen-map OpAdvance gen-step
    ]

  gen-trace :: Gen (List DiffOp)
  gen-trace = gen-list 28 gen-op

  ref-len :: List RefEntry -> U32
  ref-len xs = case xs of
    Nil -> 0
    Cons _ rest -> wrapping-add-u32 1 (ref-len rest)

  ref-nth :: Nat -> List RefEntry -> Option RefEntry
  ref-nth i xs = case xs of
    Nil -> None
    Cons e rest -> if i == 0 then Some e else ref-nth (i - 1) rest

  ref-remove-id :: U32 -> List RefEntry -> List RefEntry
  ref-remove-id want-id xs = case xs of
    Nil -> Nil
    Cons e rest -> case e of
      RefEntry id _ _ ->
        if eq-u32 id want-id then rest
        else Cons e (ref-remove-id want-id rest)

  ref-replace :: U32 -> U64 -> U64 -> List RefEntry -> List RefEntry
  ref-replace want-id new-dl new-h xs = case xs of
    Nil -> Nil
    Cons e rest -> case e of
      RefEntry id _ _ ->
        if eq-u32 id want-id then Cons (RefEntry id new-dl new-h) rest
        else Cons e (ref-replace want-id new-dl new-h rest)

  ref-min-deadline-loop :: List RefEntry -> U64 -> U64
  ref-min-deadline-loop xs acc = case xs of
    Nil -> acc
    Cons e rest -> case e of
      RefEntry _ dl _ ->
        let best = if lt-u64 dl acc then dl else acc in
        ref-min-deadline-loop rest best

  ref-min-deadline :: List RefEntry -> Option U64
  ref-min-deadline xs = case xs of
    Nil -> None
    Cons e rest -> case e of
      RefEntry _ dl _ -> Some (ref-min-deadline-loop rest dl)

  -- Partition reference list into (due-count, due-xor-sum, remaining-list) for `dl <= cutoff`.
  ref-partition-due :: U64 -> List RefEntry -> U32 -> U64 -> List RefEntry -> (U32, U64, List RefEntry)
  ref-partition-due cutoff xs due-cnt due-sum keep = case xs of
    Nil -> (due-cnt, due-sum, keep)
    Cons e rest -> case e of
      RefEntry id dl _ ->
        if le-u64 dl cutoff then
          let tok = io-token-pack 1 7 id in
          ref-partition-due cutoff rest (wrapping-add-u32 due-cnt 1) (wrapping-add-u64 due-sum tok.io-token-raw) keep
        else
          ref-partition-due cutoff rest due-cnt due-sum (Cons e keep)

  -- Check wheel invariants against reference model after every single step.
  check-invariants :: TimerWheel -> U64 -> List RefEntry ->{MVar} Bool
  check-invariants tw now refs =
    let wheel-cnt = timer-wheel-active-count tw in
    let ref-cnt = ref-len refs in
    if ne-u32 wheel-cnt ref-cnt then False
    else
      case (timer-wheel-next-deadline-ms tw, ref-min-deadline refs) of
        (None, None) -> True
        (Some d, Some rmin) ->
          -- Wheel wake boundary must never oversleep past true minimum deadline,
          -- and must not be strictly below `now` (past-due timers report `now`).
          let eff-min = if lt-u64 rmin now then now else rmin in
          if gt-u64 d eff-min then False
          else if lt-u64 d now then False
          else
            -- If earliest timer is in the current L0 epoch, next-deadline-ms is exact!
            if eq-u64 (wrapping-shr-u64 (bit-xor-u64 now eff-min) 8) 0 then
              eq-u64 d eff-min
            else True
        _ -> False

  on-hegel-fire :: U64 -> IoToken ->{MVar} Unit
  on-hegel-fire scratch tok =
    let c = raw-load-u64 (wrapping-u64-to-i64 scratch) in
    let s = raw-load-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 8)) in
    let _ = raw-store-u64 (wrapping-u64-to-i64 scratch) (wrapping-add-u64 c 1) in
    raw-store-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 8)) (wrapping-add-u64 s tok.io-token-raw)

  -- Scratch DMA page (2 U64 slots: +0 fired_count, +8 fired_token_sum) so the
  -- advance callback checks exact fired set & monotonicity with zero boxing.
  run-trace-ops :: TimerWheel -> U64 -> U64 -> U32 -> List RefEntry -> List U64 -> List DiffOp ->{MVar} Bool
  run-trace-ops tw scratch now next-id refs stale ops =
    if not (check-invariants tw now refs) then False
    else case ops of
      Nil ->
        -- Drain everything remaining by jumping past max deadline and verifying exact count.
        let far = wrapping-add-u64 now 300000 in
        let _ = raw-store-u64 (wrapping-u64-to-i64 scratch) 0 in
        let _ = raw-store-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 8)) 0 in
        let fired = timer-wheel-advance tw far (on-hegel-fire scratch) in
        case ref-partition-due far refs 0 0 Nil of
          (exp-cnt, exp-sum, rem) ->
            let got-cnt = raw-load-u64 (wrapping-u64-to-i64 scratch) in
            let got-sum = raw-load-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 8)) in
            eq-u32 fired exp-cnt
              && eq-u64 got-cnt (u32-to-u64 exp-cnt)
              && eq-u64 got-sum exp-sum
              && eq-u32 (timer-wheel-active-count tw) 0
              && eq-u32 (ref-len rem) 0
      Cons op rest -> case op of
        OpSchedule d-nat ->
          let delta = nat-to-u64-clamp d-nat in
          let dl = wrapping-add-u64 now delta in
          let tok = io-token-pack 1 7 next-id in
          let h = timer-wheel-schedule tw dl tok in
          if gen-handle-is-null h then False
          else
            let refs2 = Cons (RefEntry next-id dl h.gen-handle-raw) refs in
            run-trace-ops tw scratch now (wrapping-add-u32 next-id 1) refs2 stale rest
        OpCancel pick ->
          let n = ref-len refs in
          if eq-u32 n 0 then
            -- Also verify cancelling a stale handle (if any) cleanly returns False.
            let stale-ok = (case stale of
              Nil -> True
              Cons sh _ -> not (timer-wheel-cancel tw (GenHandle sh))) in
            if not stale-ok then False
            else run-trace-ops tw scratch now next-id refs stale rest
          else
            let idx = (case non-zero-nat (u32-to-nat n) of
              Some nz -> nat-rem pick nz
              None -> 0) in
            case ref-nth idx refs of
              None -> False
              Some (RefEntry id _ h-raw) ->
                let h = GenHandle h-raw in
                let ok1 = timer-wheel-cancel tw h in
                let ok2 = timer-wheel-cancel tw h in
                if not ok1 || ok2 then False
                else
                  let refs2 = ref-remove-id id refs in
                  run-trace-ops tw scratch now next-id refs2 (Cons h-raw stale) rest
        OpReschedule pick d-nat ->
          let n = ref-len refs in
          if eq-u32 n 0 then
            run-trace-ops tw scratch now next-id refs stale rest
          else
            let idx = (case non-zero-nat (u32-to-nat n) of
              Some nz -> nat-rem pick nz
              None -> 0) in
            case ref-nth idx refs of
              None -> False
              Some (RefEntry id _ old-raw) ->
                let old-h = GenHandle old-raw in
                let new-dl = wrapping-add-u64 now (nat-to-u64-clamp d-nat) in
                let new-h = timer-wheel-reschedule tw old-h new-dl in
                if gen-handle-is-null new-h then False
                -- Old handle must now be stale!
                else if timer-wheel-cancel tw old-h then False
                else
                  let refs2 = ref-replace id new-dl new-h.gen-handle-raw refs in
                  run-trace-ops tw scratch now next-id refs2 (Cons old-raw stale) rest
        OpAdvance step-nat ->
          let cutoff = wrapping-add-u64 now (nat-to-u64-clamp step-nat) in
          let _ = raw-store-u64 (wrapping-u64-to-i64 scratch) 0 in
          let _ = raw-store-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 8)) 0 in
          let fired = timer-wheel-advance tw cutoff (on-hegel-fire scratch) in
          case ref-partition-due cutoff refs 0 0 Nil of
            (exp-cnt, exp-sum, rem) ->
              let got-cnt = raw-load-u64 (wrapping-u64-to-i64 scratch) in
              let got-sum = raw-load-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 8)) in
              if ne-u32 fired exp-cnt || ne-u64 got-cnt (u32-to-u64 exp-cnt) || ne-u64 got-sum exp-sum then False
              else run-trace-ops tw scratch cutoff next-id rem stale rest

  nat-to-u64-clamp :: Nat -> U64
  nat-to-u64-clamp n = case checked-nat-to-u64 n of
    Some v -> v
    None -> 0

  checked-nat-to-u64 :: Nat -> Option U64
  checked-nat-to-u64 n =
    if n == 0 then Some 0
    else
      let lo = nat-rem n 65536 in
      let hi = nat-div n 65536 in
      case checked-nat-to-u64 hi of
        None -> None
        Some hi-u64 ->
          let lo-u64 = nat-small-to-u64 lo in
          Some (wrapping-add-u64 (wrapping-shl-u64 hi-u64 16) lo-u64)

  nat-small-to-u64 :: Nat -> U64
  nat-small-to-u64 n =
    if n == 0 then 0
    else if n >= 256 then
      wrapping-add-u64 (wrapping-shl-u64 (nat-small-to-u64 (nat-div n 256)) 8) (nat-small-to-u64 (nat-rem n 256))
    else
      nat-byte-to-u64 n 0

  nat-byte-to-u64 :: Nat -> U64 -> U64
  nat-byte-to-u64 n acc =
    if n == 0 then acc else nat-byte-to-u64 (n - 1) (wrapping-add-u64 acc 1)

  run-hegel-trials :: TimerWheel -> U64 -> Nat -> Nat -> HegelSeed ->{MVar} Bool
  run-hegel-trials tw scratch done total seed =
    if done >= total then True
    else
      case gen-run-live seed 256 gen-trace of
        None -> False
        Some (ops, _, seed2) ->
          let start-now = timer-wheel-now-ms tw in
          if not (run-trace-ops tw scratch start-now 1 Nil Nil ops) then False
          else run-hegel-trials tw scratch (done + 1) total seed2

  main :: Unit ->{IO, MVar} Int
  main _ =
    let scratch = engine-dma-alloc 4096 4096 in
    if eq-u64 scratch 0 then 1
    else
      case timer-wheel-new 256 1000 of
        None ->
          let _ = engine-dma-free scratch 4096 in
          2
        Some tw ->
          -- 1,024 randomized Hegel differential traces against the live DMA wheel!
          let ok = run-hegel-trials tw scratch 0 1024 (hegel-seed 20261128) in
          let _ = timer-wheel-destroy tw in
          let _ = engine-dma-free scratch 4096 in
          if ok then 0 else 3
"#;

    run_leak_checked("engine-timer-hegel-1024", src, 0);
}

#[test]
fn engine_timer_cascades_coarse_clock_and_zero_alloc_hot_loop() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Data.Option
  import Data.String
  import Engine.Types
  import Engine.Timer
  import Prim.Atomic

  -- Scratch layout (in 4096B DMA page):
  --   +0  count :: U64
  --   +8  last-tok :: U64
  --   +16 sum-tok :: U64
  --   +24 seq-ok :: U64 (1 = tokens arrived in strictly ascending slot order)
  --   +32 reentrant-handle :: U64 (handle to cancel from inside callback)
  --   +40 reentrant-cancelled :: U64

  on-fire-record :: U64 -> IoToken ->{MVar} Unit
  on-fire-record scratch tok =
    let slot = u32-to-u64 (io-token-slot tok) in
    let c = raw-load-u64 (wrapping-u64-to-i64 scratch) in
    let s = raw-load-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 16)) in
    let _ = raw-store-u64 (wrapping-u64-to-i64 scratch) (wrapping-add-u64 c 1) in
    let _ = raw-store-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 8)) slot in
    let _ = raw-store-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 16)) (wrapping-add-u64 s slot) in
    -- Expected firing order of slots: 1 (42), 2 (1000), 6 (1500), 3 (200000), 4 (50M), 5 (5G)
    let ok = (if eq-u64 c 0 then eq-u64 slot 1 else if eq-u64 c 1 then eq-u64 slot 2 else if eq-u64 c 2 then eq-u64 slot 6 else if eq-u64 c 3 then eq-u64 slot 3 else if eq-u64 c 4 then eq-u64 slot 4 else if eq-u64 c 5 then eq-u64 slot 5 else False) in
    if not ok then raw-store-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 24)) 0 else ()

  on-fire-reentrant :: TimerWheel -> U64 -> IoToken ->{MVar} Unit
  on-fire-reentrant tw scratch tok =
    if eq-u32 (io-token-slot tok) 10 then
      let bh = GenHandle (raw-load-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 32))) in
      if timer-wheel-cancel tw bh then
        raw-store-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 40)) 1
      else ()
    else ()

  -- 1. Multi-level cascades (L0 -> L1 -> L2 -> L3 -> Overflow) + next-deadline-ms
  --    + reschedule + stale handle rejection + backward advance + re-entrant cancel.
  test-wheel-levels :: U64 ->{IO, MVar} Int
  test-wheel-levels scratch =
    case timer-wheel-new 64 0 of
      None -> 10
      Some tw ->
        -- Empty wheel checks
        let empty-ok = (case timer-wheel-next-deadline-ms tw of
          None -> eq-u32 (timer-wheel-active-count tw) 0
          Some _ -> False) in
        if not empty-ok then
          let _ = timer-wheel-destroy tw in
          11
        else
          -- Schedule timers across L0, L1, L2, L3, and Overflow (> 2^32 ms = 49.7 days!)
          let h-l0 = timer-wheel-schedule tw 42 (io-token-pack 1 1 1) in
          let h-l1 = timer-wheel-schedule tw 1000 (io-token-pack 1 1 2) in
          let h-l2 = timer-wheel-schedule tw 200000 (io-token-pack 1 1 3) in
          let h-l3 = timer-wheel-schedule tw 50000000 (io-token-pack 1 1 4) in
          let h-ovf = timer-wheel-schedule tw 5000000000 (io-token-pack 1 1 5) in
          let h-can = timer-wheel-schedule tw 500 (io-token-pack 1 1 99) in
          let h-res = timer-wheel-schedule tw 30000000 (io-token-pack 1 1 6) in
          -- Cancel h-can twice: first True, second False
          let c1 = timer-wheel-cancel tw h-can in
          let c2 = timer-wheel-cancel tw h-can in
          -- Reschedule h-res from L3 down into L1 at 1500 ms (slot 6)
          let h-res2 = timer-wheel-reschedule tw h-res 1500 in
          -- Old h-res must be stale now
          let stale-res = timer-wheel-cancel tw h-res in
          if not c1 || c2 || gen-handle-is-null h-res2 || stale-res then
            let _ = timer-wheel-destroy tw in
            12
          else if ne-u32 (timer-wheel-active-count tw) 6 then
            let _ = timer-wheel-destroy tw in
            13
          else
            -- L0 earliest deadline must be exact: Some 42
            let nd0-ok = (case timer-wheel-next-deadline-ms tw of
              Some d -> eq-u64 d 42
              None -> False) in
            if not nd0-ok then
              let _ = timer-wheel-destroy tw in
              14
            else
              -- Advance to 41: nothing fires
              let f0 = timer-wheel-advance tw 41 (\_ -> ()) in
              if ne-u32 f0 0 then
                let _ = timer-wheel-destroy tw in
                15
              else
                -- Backward advance to 10: returns 0, now stays 41
                let fb = timer-wheel-advance tw 10 (\_ -> ()) in
                if ne-u32 fb 0 || ne-u64 (timer-wheel-now-ms tw) 41 then
                  let _ = timer-wheel-destroy tw in
                  16
                else
                  -- Reset scratch and fire L0 (42), L1 (1000, 1500), L2 (200000),
                  -- L3 (50000000), Overflow (5000000000) across incremental advances!
                  let _ = raw-store-u64 (wrapping-u64-to-i64 scratch) 0 in
                  let _ = raw-store-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 8)) 0 in
                  let _ = raw-store-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 16)) 0 in
                  let _ = raw-store-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 24)) 1 in
                  let on-fire = on-fire-record scratch in
                  -- Advance to 42 -> fires L0 (slot 1)
                  let f1 = timer-wheel-advance tw 42 on-fire in
                  -- Now earliest timer is 1000 (L1 bucket 3 = 768)
                  let nd1-ok = (case timer-wheel-next-deadline-ms tw of
                    Some d -> ge-u64 d 42 && le-u64 d 1000
                    None -> False) in
                  -- Advance to 1500 -> cascades L1, fires 1000 (slot 2) and 1500 (slot 6)
                  let f2 = timer-wheel-advance tw 1500 on-fire in
                  -- Advance to 250000 -> cascades L2 -> L1 -> L0, fires 200000 (slot 3)
                  let f3 = timer-wheel-advance tw 250000 on-fire in
                  -- Advance to 60000000 -> cascades L3 -> L2 -> L1 -> L0, fires 50000000 (slot 4)
                  let f4 = timer-wheel-advance tw 60000000 on-fire in
                  -- Next deadline for overflow timer at 5000000000 is 2^32 = 4294967296
                  let nd-ovf-ok = (case timer-wheel-next-deadline-ms tw of
                    Some d -> eq-u64 d 4294967296
                    None -> False) in
                  -- Advance to 5000000000 -> cascades Overflow -> L3 -> L2 -> L1 -> L0, fires slot 5
                  let f5 = timer-wheel-advance tw 5000000000 on-fire in
                  let total-c = raw-load-u64 (wrapping-u64-to-i64 scratch) in
                  let total-s = raw-load-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 16)) in
                  let seq-ok = raw-load-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 24)) in
                  -- Cancelled-after-fire handles must be rejected
                  let stale-fired = timer-wheel-cancel tw h-l0 || timer-wheel-cancel tw h-l1 || timer-wheel-cancel tw h-l2 || timer-wheel-cancel tw h-l3 || timer-wheel-cancel tw h-ovf || timer-wheel-cancel tw h-res2 in
                  -- Re-entrant cancel inside callback: schedule A and B at same ms, A cancels B!
                  let now2 = timer-wheel-now-ms tw in
                  let ha = timer-wheel-schedule tw (wrapping-add-u64 now2 5) (io-token-pack 2 1 10) in
                  let hb = timer-wheel-schedule tw (wrapping-add-u64 now2 5) (io-token-pack 2 1 20) in
                  let _ = raw-store-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 32)) hb.gen-handle-raw in
                  let _ = raw-store-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 40)) 0 in
                  let fr = timer-wheel-advance tw (wrapping-add-u64 now2 5) (on-fire-reentrant tw scratch) in
                  let reent-ok = eq-u32 fr 1 && eq-u64 (raw-load-u64 (wrapping-u64-to-i64 (wrapping-add-u64 scratch 40))) 1 && eq-u32 (timer-wheel-active-count tw) 0 && not (gen-handle-is-null ha) in
                  let _ = timer-wheel-destroy tw in
                  if not nd1-ok || not nd-ovf-ok then 17
                  else if ne-u32 f1 1 || ne-u32 f2 2 || ne-u32 f3 1 || ne-u32 f4 1 || ne-u32 f5 1 then 18
                  else if ne-u64 total-c 6 || ne-u64 total-s 21 || ne-u64 seq-ok 1 || stale-fired then 19
                  else if not reent-ok then 20
                  else 0

  -- 2. CoarseClockCache RFC 9110 IMF-fixdate across epoch, leap years, century, Y2K, & cache hits.
  check-date :: CoarseClockCache -> U64 -> String -> Bool ->{MVar} Bool
  check-date cache sec expected expect-reformat =
    let changed = coarse-date-refresh cache sec in
    let got = coarse-date-to-string cache in
    (changed == expect-reformat)
      && (string-compare got expected == 0)
      && eq-u64 (coarse-date-last-sec cache) sec
      && eq-u32 (coarse-date-len ()) 29

  test-coarse-clock :: Unit ->{IO, MVar} Int
  test-coarse-clock _ =
    case coarse-clock-new () of
      None -> 30
      Some cache ->
        let ok = (
          -- Unix epoch: 1970-01-01 00:00:00
          check-date cache 0 "Thu, 01 Jan 1970 00:00:00 GMT" True
            -- Same second cache hit -> returns False in 1 compare
            && check-date cache 0 "Thu, 01 Jan 1970 00:00:00 GMT" False
            -- Y2K eve & rollover
            && check-date cache 946684799 "Fri, 31 Dec 1999 23:59:59 GMT" True
            && check-date cache 946684800 "Sat, 01 Jan 2000 00:00:00 GMT" True
            -- Leap century 2000-02-29 & 2000-03-01
            && check-date cache 951868799 "Tue, 29 Feb 2000 23:59:59 GMT" True
            && check-date cache 951868800 "Wed, 01 Mar 2000 00:00:00 GMT" True
            -- Standard leap year 2024-02-29 12:34:56
            && check-date cache 1709210096 "Thu, 29 Feb 2024 12:34:56 GMT" True
            && check-date cache 1709210096 "Thu, 29 Feb 2024 12:34:56 GMT" False
            -- Prompt vector: 2026-11-28 14:00:00
            && check-date cache 1795874400 "Sat, 28 Nov 2026 14:00:00 GMT" True
            -- Non-leap century boundary: 2100-02-28 23:59:59 -> 2100-03-01 00:00:00
            && check-date cache 4107542399 "Sun, 28 Feb 2100 23:59:59 GMT" True
            && check-date cache 4107542400 "Mon, 01 Mar 2100 00:00:00 GMT" True) in
        let _ = coarse-clock-destroy cache in
        if ok then 0 else 31

  -- Top-level zero-alloc callback for the 50,000-op steady-state hot loop.
  hot-noop-cb :: IoToken ->{MVar} Unit
  hot-noop-cb _ = ()

  -- 3. 50,000 steady-state schedule + reschedule + cancel + advance +
  --    next-deadline-raw + coarse-date-refresh ops with 0 heap allocs & 0 atomic RCs!
  hot-loop :: TimerWheel -> CoarseClockCache -> U32 -> U64 -> U64 ->{MVar} U64
  hot-loop tw cache rem t fired-acc =
    if eq-u32 rem 0 then fired-acc
    else
      let tok1 = io-token-pack 1 1 rem in
      let tok2 = io-token-pack 2 1 rem in
      let h1 = timer-wheel-schedule tw (wrapping-add-u64 t 10) tok1 in
      let h2 = timer-wheel-schedule tw (wrapping-add-u64 t 500) tok2 in
      let h2b = timer-wheel-reschedule tw h2 (wrapping-add-u64 t 4) in
      let _ = timer-wheel-cancel tw h1 in
      let _ = timer-wheel-next-deadline-raw tw in
      let _ = coarse-date-refresh cache (wrapping-add-u64 1795874400 (wrapping-shr-u64 t 10)) in
      let f = timer-wheel-advance tw (wrapping-add-u64 t 10) hot-noop-cb in
      let _ = h2b in
      hot-loop tw cache (wrapping-sub-u32 rem 1) (wrapping-add-u64 t 10) (wrapping-add-u64 fired-acc (u32-to-u64 f))

  test-zero-alloc-fiftyk :: Unit ->{IO, MVar} Int
  test-zero-alloc-fiftyk _ =
    case (timer-wheel-new 1024 0, coarse-clock-new ()) of
      (Some tw, Some cache) ->
        -- Warm up cache once before resetting counters (or test miss + hit inside loop;
        -- both miss and hit in coarse-date-refresh are 0-alloc!).
        let _ = engine-thread-counters-reset () in
        let fired = hot-loop tw cache 50000 0 0 in
        let allocs = engine-thread-alloc-count () in
        let atomic-rcs = engine-thread-atomic-rc-count () in
        let active = timer-wheel-active-count tw in
        let _ = timer-wheel-destroy tw in
        let _ = coarse-clock-destroy cache in
        if ne-u64 fired 50000 then 41
        else if ne-u32 active 0 then 42
        else if ne-u64 allocs 0 then 43
        else if ne-u64 atomic-rcs 0 then 44
        else 0
      (Some tw, None) ->
        let _ = timer-wheel-destroy tw in
        40
      (None, Some cache) ->
        let _ = coarse-clock-destroy cache in
        40
      (None, None) -> 40

  main :: Unit ->{IO, MVar} Int
  main _ =
    let scratch = engine-dma-alloc 4096 4096 in
    if eq-u64 scratch 0 then 1
    else
      let r1 = test-wheel-levels scratch in
      let _ = engine-dma-free scratch 4096 in
      if r1 /= 0 then r1
      else
        let r2 = test-coarse-clock () in
        if r2 /= 0 then r2
        else test-zero-alloc-fiftyk ()
"#;

    run_leak_checked("engine-timer-runtime-50k", src, 0);
}
