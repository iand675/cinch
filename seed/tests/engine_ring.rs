//! Integration, formal-verification (M2 `RingSyncMachine`), cache-line layout
//! audit, `%1` linear `ClaimToken` compile-fail, 1,000,000-message pinned-worker
//! SPSC & Multi-Producer Disruptor stress, and shared-memory `mmap` IPC tests
//! for `Engine.Ring.{Spsc,Disruptor,Shm,Spec}`.
//!
//! Routed through `common::drive_with_profile` / `common::drive_with_profile_opts` /
//! `common::check_interfaces` / `common::source_frontend` so the suite runs
//! against both the Rust seed and the Stage 3 self-hosted compiler.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{DriverOptions, Mode, Package};
use cinch::prim_verify::mem_contract::{audit_cache_line_file, MIN_CACHE_LINE_BYTES};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn fresh_dir(tag: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "cinch-engine-ring-{tag}-{}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

/// Compile a small C bridge to an object file and return its path.
fn c_fixture(name: &str, src: &str) -> PathBuf {
    let dir = fresh_dir(&format!("cfix-{name}"));
    let c_path = dir.join(format!("{name}.c"));
    let o_path = dir.join(format!("{name}.o"));
    std::fs::write(&c_path, src).expect("write C fixture");
    let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".to_string());
    let status = Command::new(&clang)
        .arg("-O2")
        .arg("-c")
        .arg(&c_path)
        .arg("-o")
        .arg(&o_path)
        .status()
        .expect("run clang");
    assert!(status.success(), "{name}: clang failed to compile C fixture");
    o_path
}

fn build_and_run_leak_checked(
    tag: &str,
    src: &str,
    objects: &[PathBuf],
    expected_code: i32,
) -> (std::time::Duration, String) {
    let dir = fresh_dir(tag);
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let exe = dir.join(format!("ring-{tag}"));
    let mut opts = DriverOptions::default();
    opts.native_objects = objects.to_vec();
    let build_res = common::drive_with_profile_opts(
        Mode::Build,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        Some(exe.clone()),
        &TargetProfile::native(),
        &opts,
    );
    if let Err(e) = build_res {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("{tag}: build failed:\n{e}");
    }

    let t0 = Instant::now();
    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "64M")
        .output()
        .expect("run built binary");
    let elapsed = t0.elapsed();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let _ = std::fs::remove_dir_all(&dir);

    assert_eq!(
        out.status.code(),
        Some(expected_code),
        "{tag}: expected exit {expected_code}, got {:?}\nstdout:\n{stdout}\nstderr:\n{stderr}",
        out.status.code()
    );
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "{tag}: expected clean leak check (`leaked=0`), got stderr:\n{stderr}"
    );
    (elapsed, stderr)
}

// =========================================================================
// 1. Stdlib manifest registration
// =========================================================================

#[test]
fn engine_ring_modules_registered_in_stdlib_package_manifest() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, rel_path) in [
        ("Engine.Ring.Spsc", "Engine/Ring/Spsc.cinch"),
        ("Engine.Ring.Disruptor", "Engine/Ring/Disruptor.cinch"),
        ("Engine.Ring.Shm", "Engine/Ring/Shm.cinch"),
        ("Engine.Ring.Spec", "Engine/Ring/Spec.cinch"),
    ] {
        let entry = pkg
            .modules
            .iter()
            .find(|m| m.name == name)
            .unwrap_or_else(|| panic!("{name} missing from seed/prelude/.cinchpkg modules"));
        assert_eq!(entry.path, rel_path);
        assert!(
            pkg.order.iter().any(|m| m == name),
            "{name} missing from seed/prelude/.cinchpkg order"
        );
    }
}

// =========================================================================
// 2. Formal Model M2 (`RingSyncMachine`): capped=false, 0 violations under
//    CINCH_MODEL_FAIL_ON_CAPPED=1
// =========================================================================

#[test]
fn formal_model_m2_ring_sync_machine_verifies_uncapped_zero_violations() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("prelude/Engine/Ring/Spec.cinch");
    let spec_str = spec_path.to_string_lossy().into_owned();
    let cinch_bin = env!("CARGO_BIN_EXE_cinch");

    // 1. `cinch check` with `CINCH_MODEL_FAIL_ON_CAPPED=1` on a project importing Engine.Ring.Spec
    let dir = fresh_dir("m2-spec-check");
    std::fs::write(
        dir.join("Main.cinch"),
        r#"
module Main where
  import Engine.Ring.Spec

  main :: Int
  main = if ring-bounds-ok ring-init then 0 else 1
"#,
    )
    .expect("write Main.cinch");

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
    .expect("Engine.Ring.Spec must check cleanly with model_fail_on_capped=true");

    // 2. CLI `cinch model inspect <Spec.cinch> RingSyncMachine --json --model-fail-on-capped`
    //    with `CINCH_MODEL_FAIL_ON_CAPPED=1`
    let inspect_out = Command::new(cinch_bin)
        .args(["model", "inspect", &spec_str, "RingSyncMachine", "--json"])
        .env("CINCH_MODEL_FAIL_ON_CAPPED", "1")
        .output()
        .expect("run cinch model inspect RingSyncMachine");
    let stdout = String::from_utf8_lossy(&inspect_out.stdout);
    let stderr = String::from_utf8_lossy(&inspect_out.stderr);
    assert!(
        inspect_out.status.success(),
        "M2 RingSyncMachine failed under CINCH_MODEL_FAIL_ON_CAPPED=1:\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    let json: serde_json::Value =
        serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("invalid JSON: {e}\n{stdout}"));
    let machines = json
        .as_array()
        .expect("top-level machines array in model inspect JSON");
    let m2 = machines
        .iter()
        .find(|m| m.get("name").and_then(|v| v.as_str()) == Some("RingSyncMachine"))
        .unwrap_or_else(|| panic!("RingSyncMachine missing from JSON:\n{stdout}"));
    let capped = m2
        .get("capped")
        .and_then(|v| v.as_bool())
        .expect("capped bool");
    let states = m2
        .get("state_count")
        .and_then(|v| v.as_u64())
        .expect("state_count");
    let transitions = m2
        .get("edge_count")
        .and_then(|v| v.as_u64())
        .expect("edge_count");
    let deadlocks = m2
        .get("deadlocks")
        .and_then(|v| v.as_array())
        .expect("deadlocks array");
    assert!(!capped, "M2 RingSyncMachine MUST have capped=false, got JSON:\n{stdout}");
    assert!(deadlocks.is_empty(), "M2 RingSyncMachine MUST have 0 deadlocks, got {deadlocks:?}");
    assert!(states > 50 && states < 65536, "unexpected M2 state count: {states}");

    // Verify all contracts and refinement are recorded as passed in JSON
    let contracts = m2
        .get("contracts")
        .and_then(|v| v.as_array())
        .expect("contracts array");
    assert!(
        contracts.len() >= 4,
        "expected >= 4 verified contracts (2 safety + 2 liveness), got {contracts:?}"
    );
    for c in contracts {
        assert_eq!(
            c.get("passed").and_then(|v| v.as_bool()),
            Some(true),
            "contract failed: {c:?}"
        );
    }
    let refines = m2
        .get("refinements")
        .and_then(|v| v.as_array())
        .expect("refinements array");
    let m2_ref = refines
        .iter()
        .find(|r| {
            r.get("concrete").and_then(|v| v.as_str()) == Some("RingSyncMachine")
                && r.get("abstract").and_then(|v| v.as_str()) == Some("AbstractFifoQueue")
        })
        .unwrap_or_else(|| panic!("missing RingSyncMachine refines AbstractFifoQueue in JSON:\n{stdout}"));
    assert_eq!(
        m2_ref.get("passed").and_then(|v| v.as_bool()),
        Some(true),
        "refinement failed: {m2_ref:?}"
    );
    assert_eq!(
        m2_ref.get("has_measure").and_then(|v| v.as_bool()),
        Some(true),
        "refinement must check stuttering measure: {m2_ref:?}"
    );

    println!(
        "[M2 RingSyncMachine] states={states} transitions={transitions} capped={capped} contracts={} refinement=PASS",
        contracts.len()
    );

    let _ = std::fs::remove_dir_all(&dir);
}

// =========================================================================
// 3. Cache-line isolation audit on `Engine.Ring.Spsc` (`0 violations`)
// =========================================================================

#[test]
fn spsc_ring_header_passes_cache_line_isolation_audit() {
    let dir = fresh_dir("spsc-cache-line-audit");
    std::fs::write(
        dir.join("Main.cinch"),
        r#"
module Main where
  import Engine.Ring.Spsc

  main :: Int
  main = 0
"#,
    )
    .expect("write Main.cinch");

    let ifaces = common::check_interfaces(&dir, "Main.cinch").expect("check Engine.Ring.Spsc");
    let spsc = ifaces
        .iter()
        .find(|i| i.module == "Engine.Ring.Spsc")
        .expect("Engine.Ring.Spsc interface emitted");

    let hdr = spsc
        .layouts
        .iter()
        .find(|l| l.name == "SpscRingHeader")
        .expect("SpscRingHeader layout present");
    assert_eq!(hdr.size, 384, "SpscRingHeader size must be 384 (3 x 128B)");
    assert_eq!(hdr.align, 128, "SpscRingHeader align must be 128");
    let head_f = hdr.fields.iter().find(|f| f.name == "head").expect("head field");
    let tail_f = hdr.fields.iter().find(|f| f.name == "tail").expect("tail field");
    assert_eq!(head_f.offset, 0);
    assert_eq!(tail_f.offset, 128);
    assert!(tail_f.offset - head_f.offset >= 128);

    // Write `.cinchif` and audit both via Rust API and `cinch verify-prims --audit-cache-line=` CLI.
    let cinchif_path = dir.join("Engine.Ring.Spsc.cinchif");
    std::fs::write(&cinchif_path, spsc.to_json()).expect("write Engine.Ring.Spsc.cinchif");

    let rep = audit_cache_line_file(&cinchif_path, MIN_CACHE_LINE_BYTES)
        .expect("audit Engine.Ring.Spsc.cinchif");
    assert_eq!(rep.structs_checked, 2, "expected SpscRingHeader & SpscRing audited: {rep:?}");
    assert_eq!(rep.cursor_pairs_checked, 1, "expected (head, tail) cursor pair audited: {rep:?}");
    assert!(
        rep.violations.is_empty(),
        "SpscRingHeader & SpscRing must have 0 cache-line violations, got: {:?}",
        rep.violations
    );

    let cinch_bin = env!("CARGO_BIN_EXE_cinch");
    let cli_out = Command::new(cinch_bin)
        .arg("verify-prims")
        .arg(format!("--audit-cache-line={}", cinchif_path.display()))
        .output()
        .expect("run cinch verify-prims --audit-cache-line");
    let stdout = String::from_utf8_lossy(&cli_out.stdout);
    let stderr = String::from_utf8_lossy(&cli_out.stderr);
    assert!(
        cli_out.status.success(),
        "cinch verify-prims --audit-cache-line failed:\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("structs=2 cursor_pairs=1 violations=0"),
        "expected clean cache-line audit line, got:\n{stdout}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

// =========================================================================
// 4. Compile-fail `%1` linear `ClaimToken` tests
// =========================================================================

fn check_project_src(tag: &str, src: &str) -> Result<i32, String> {
    let dir = fresh_dir(tag);
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let res = common::drive_with_profile(
        Mode::Check,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        None,
        &TargetProfile::native(),
    );
    let _ = std::fs::remove_dir_all(&dir);
    res
}

#[test]
fn disruptor_linear_claim_token_forbids_drop_and_double_publish() {
    // Case A: Dropping a `%1` ClaimToken without calling `disruptor-publish` MUST fail to compile.
    let drop_direct = r#"
module Main where
  import Engine.Ring.Disruptor

  bad-forget-publish :: Disruptor -> ClaimToken %1 ->{MVar} Unit
  bad-forget-publish d tok = ()

  main :: Int
  main = 0
"#;
    let msg_drop = check_project_src("linear-drop-direct", drop_direct)
        .expect_err("dropping ClaimToken %1 MUST be rejected at compile time");
    assert!(
        msg_drop.contains("multiplicity %1") || msg_drop.contains("unused"),
        "expected %1 linearity diagnostic, got:\n{msg_drop}"
    );

    // Case B: Dropping a `%1` ClaimToken inside `disruptor-with-claim` callback MUST fail to compile.
    let drop_callback = r#"
module Main where
  import Engine.Ring.Disruptor

  bad-callback :: Disruptor ->{MVar} Option Unit
  bad-callback d = disruptor-with-claim d (\tok -> ())

  main :: Int
  main = 0
"#;
    let msg_cb = check_project_src("linear-drop-callback", drop_callback)
        .expect_err("dropping ClaimToken %1 inside disruptor-with-claim MUST be rejected");
    assert!(
        msg_cb.contains("multiplicity %1") || msg_cb.contains("unused"),
        "expected %1 linearity diagnostic in callback, got:\n{msg_cb}"
    );

    // Case C: Publishing a `%1` ClaimToken twice MUST fail to compile.
    let double_pub = r#"
module Main where
  import Engine.Ring.Disruptor

  bad-double-publish :: Disruptor -> ClaimToken %1 ->{MVar} Unit
  bad-double-publish d tok =
    let _ = disruptor-publish d tok in
    disruptor-publish d tok

  main :: Int
  main = 0
"#;
    let msg_dbl = check_project_src("linear-double-publish", double_pub)
        .expect_err("double-publishing ClaimToken %1 MUST be rejected");
    assert!(
        msg_dbl.contains("multiplicity %1") || msg_dbl.contains("more than once") || msg_dbl.contains("exactly once"),
        "expected %1 double-use diagnostic, got:\n{msg_dbl}"
    );

    // Case D: Valid linear threading (`claim-write-msg` -> `disruptor-publish`) MUST check cleanly.
    let valid_lin = r#"
module Main where
  import Engine.Types
  import Engine.Ring.Disruptor

  good-publish :: Disruptor -> MsgEnvelope -> ClaimToken %1 ->{MVar} Unit
  good-publish d msg tok =
    let tok1 = claim-write-msg msg tok in
    disruptor-publish d tok1

  main :: Int
  main = 0
"#;
    check_project_src("linear-valid-publish", valid_lin)
        .expect("valid linear ClaimToken publish must compile cleanly");
}

// =========================================================================
// 5. Functional unit test: SPSC, Batch, CoreMesh (4x3), Disruptor SP+Barrier
// =========================================================================

#[test]
fn spsc_batch_core_mesh_and_disruptor_unit_suite_leak_free() {
    let src = r#"
module Main where
  import Data.List
  import Data.Machine
  import Data.Option
  import Engine.Types
  import Engine.Ring.Spsc
  import Engine.Ring.Disruptor

  make-msg :: U64 -> MsgEnvelope
  make-msg seq =
    msg-envelope-make 7 (core-id 1) (core-id 2) seq
      (wrapping-mul-u64 seq 3)
      (wrapping-add-u64 seq 11)
      (wrapping-xor-u64 seq 0xCAFEBABE)
      4 5 6

  wrapping-xor-u64 :: U64 -> U64 -> U64
  wrapping-xor-u64 a b = bit-xor-u64 a b

  test-spsc-basic :: Unit ->{IO, MVar} I64
  test-spsc-basic _ =
    case spsc-new 3 of
      Some _ -> 10
      None ->
        case spsc-new 4 of
          None -> 11
          Some r ->
            if not (spsc-is-empty r) then 12
            else if spsc-is-full r then 13
            else
              let p0 = spsc-try-push r (make-msg 100) in
              let p1 = spsc-try-push r (make-msg 101) in
              let p2 = spsc-try-push r (make-msg 102) in
              let p3 = spsc-try-push r (make-msg 103) in
              let p4 = spsc-try-push r (make-msg 104) in
              if not p0 || not p1 || not p2 || not p3 || p4 then 14
              else if not (spsc-is-full r) || ne-u64 (spsc-len r) 4 then 15
              else
                let r0 = spsc-try-pop-unboxed r in
                if not (pop-envelope-is-some r0) then 16
                else
                  if not (msg-envelope-eq (pop-envelope-msg r0) (make-msg 100)) then 17
                  else
                    let p5 = spsc-try-push r (make-msg 104) in
                    if not p5 then 18
                    else
                      let r1 = spsc-try-pop-unboxed r in
                      let r2 = spsc-try-pop-unboxed r in
                      let r3 = spsc-try-pop-unboxed r in
                      let r4 = spsc-try-pop-unboxed r in
                      let r5 = spsc-try-pop-unboxed r in
                      let ok = pop-envelope-is-some r1 && eq-u64 r1.pop-seq 101 && pop-envelope-is-some r2 && eq-u64 r2.pop-seq 102 && pop-envelope-is-some r3 && eq-u64 r3.pop-seq 103 && pop-envelope-is-some r4 && eq-u64 r4.pop-seq 104 && not (pop-envelope-is-some r5) && spsc-is-empty r in
                      let freed = spsc-destroy r in
                      if ok && io-error-is-ok freed then 0 else 19

  test-core-mesh :: Unit ->{IO, MVar} I64
  test-core-mesh _ =
    case core-mesh-new 4 16 of
      None -> 20
      Some mesh ->
        -- Diagonal (same core) must return None
        case core-mesh-ring-for mesh (core-id 2) (core-id 2) of
          Some _ -> 21
          None ->
            let ok01 = core-mesh-send mesh (core-id 0) (core-id 1) (make-msg 10) in
            let ok03 = core-mesh-send mesh (core-id 0) (core-id 3) (make-msg 30) in
            let ok30 = core-mesh-send mesh (core-id 3) (core-id 0) (make-msg 300) in
            let ok21 = core-mesh-send mesh (core-id 2) (core-id 1) (make-msg 210) in
            let r01 = core-mesh-recv-unboxed mesh (core-id 0) (core-id 1) in
            let r03 = core-mesh-recv-unboxed mesh (core-id 0) (core-id 3) in
            let r30 = core-mesh-recv-unboxed mesh (core-id 3) (core-id 0) in
            let r21 = core-mesh-recv-unboxed mesh (core-id 2) (core-id 1) in
            let rempty = core-mesh-recv-unboxed mesh (core-id 1) (core-id 2) in
            let ok = ok01 && ok03 && ok30 && ok21 && pop-envelope-is-some r01 && eq-u64 r01.pop-seq 10 && pop-envelope-is-some r03 && eq-u64 r03.pop-seq 30 && pop-envelope-is-some r30 && eq-u64 r30.pop-seq 300 && pop-envelope-is-some r21 && eq-u64 r21.pop-seq 210 && not (pop-envelope-is-some rempty) in
            let freed = core-mesh-destroy mesh in
            if ok && io-error-is-ok freed then 0 else 22

  write-and-pub-step :: Disruptor -> MsgEnvelope -> ClaimToken %1 ->{MVar} Unit
  write-and-pub-step d msg tok =
    let tok1 = claim-write-msg msg tok in
    disruptor-publish d tok1

  test-disruptor-sp :: Unit ->{IO, MVar, Diverge} I64
  test-disruptor-sp _ =
    case disruptor-new 8 ProducerSingle of
      None -> 30
      Some d ->
        case disruptor-try-claim d of
          None -> 31
          Some tok0 ->
            let _p0 = disruptor-write-and-publish d (make-msg 500) tok0 in
            case disruptor-try-claim d of
              None -> 32
              Some tok1 ->
                let _p1 = write-and-pub-step d (make-msg 501) tok1 in
                let r2 = disruptor-with-claim d (disruptor-publish d) in
                case r2 of
                  None -> 33
                  Some _ ->
                    let bar = barrier-new d (Cons (disruptor-published-cursor-addr d) Nil) in
                    let hi = barrier-wait-for bar (WaitYieldSpin 4) 0 in
                    if hi /= 2 then 34
                    else
                      let m0 = disruptor-read-slot d 0 in
                      let m1 = disruptor-read-slot d 1 in
                      let _c = disruptor-commit-consumer d 2 in
                      let ok = msg-envelope-eq m0 (make-msg 500) && msg-envelope-eq m1 (make-msg 501) in
                      let freed = disruptor-destroy d in
                      if ok && io-error-is-ok freed then 0 else 35

  main :: Unit ->{IO, MVar, Diverge} Int
  main _ =
    let c1 = test-spsc-basic () in
    if ne-i64 c1 0 then int-from-i64 c1
    else
      let c2 = test-core-mesh () in
      if ne-i64 c2 0 then int-from-i64 c2
      else
        let c3 = test-disruptor-sp () in
        int-from-i64 c3
"#;

    let (_, _) = build_and_run_leak_checked("unit-suite", src, &[], 0);
}

// =========================================================================
// 6. Native pinned-worker stress tests:
//    - 2 pinned OS workers exchanging 1,000,000 MsgEnvelopes over SpscRing
//      (both single-item AND batched push/pop!)
//    - 2 pinned Multi-Producer Disruptor workers -> 1 pinned Consumer
//      using atomic-fetch-add-acqrel-i64 + SequenceBarrier + WaitAdaptivePark
//    - Verifying exact FIFO/sequence integrity, 0 heap allocs, 0 atomic RC ops,
//      and CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1 (leaked=0)!
// =========================================================================

#[test]
fn pinned_workers_1m_spsc_and_multi_producer_disruptor_zero_alloc_stress() {
    let bridge = c_fixture(
        "engine_ring_stress_bridge",
        r#"
#include <stdint.h>

extern int64_t ring_spsc_prod_1m(int64_t ring_base);
extern int64_t ring_spsc_cons_1m(int64_t ring_base);
extern int64_t ring_batch_prod_1m(int64_t ring_base);
extern int64_t ring_batch_cons_1m(int64_t ring_base);
extern int64_t ring_mp_prod_a(int64_t dis_base);
extern int64_t ring_mp_prod_b(int64_t dis_base);
extern int64_t ring_mp_cons(int64_t dis_base);

typedef struct {
    uint64_t header;
    int64_t code;
    int64_t arity;
    int64_t nfilled;
} CinchStaticClosure;

/* KIND_CLOSURE = 2, bits 35..40 (`block_kinds.rs`: RC_BITS = 35); rc = 0 (RC_IMMORTAL). */
#define IMMORTAL_CLOSURE_HEADER ((uint64_t)2ULL << 35)

#define DEF_CLO(NAME, FN) \
  static int64_t stub_##NAME(int64_t self, int64_t argc, int64_t *argv) { \
      (void)self; (void)argc; \
      return FN(argv[0]); \
  } \
  static CinchStaticClosure clo_##NAME; \
  int64_t ring_get_clo_##NAME(void) { \
      clo_##NAME.header = IMMORTAL_CLOSURE_HEADER; \
      clo_##NAME.code = (int64_t)(uintptr_t)stub_##NAME; \
      clo_##NAME.arity = 1; \
      clo_##NAME.nfilled = 0; \
      return (int64_t)(uintptr_t)&clo_##NAME.code; \
  }

DEF_CLO(spsc_prod, ring_spsc_prod_1m)
DEF_CLO(spsc_cons, ring_spsc_cons_1m)
DEF_CLO(batch_prod, ring_batch_prod_1m)
DEF_CLO(batch_cons, ring_batch_cons_1m)
DEF_CLO(mp_prod_a, ring_mp_prod_a)
DEF_CLO(mp_prod_b, ring_mp_prod_b)
DEF_CLO(mp_cons, ring_mp_cons)
"#,
    );

    let src = r#"
module Main where
  import Data.List
  import Data.Machine
  import Data.Option
  import Engine.Types
  import Engine.Ring.Spsc
  import Engine.Ring.Disruptor
  import Prim.Atomic

  foreign import ccall "ring_get_clo_spsc_prod" getCloSpscProd :: Unit ->{IO} I64
  foreign import ccall "ring_get_clo_spsc_cons" getCloSpscCons :: Unit ->{IO} I64
  foreign import ccall "ring_get_clo_batch_prod" getCloBatchProd :: Unit ->{IO} I64
  foreign import ccall "ring_get_clo_batch_cons" getCloBatchCons :: Unit ->{IO} I64
  foreign import ccall "ring_get_clo_mp_prod_a" getCloMpProdA :: Unit ->{IO} I64
  foreign import ccall "ring_get_clo_mp_prod_b" getCloMpProdB :: Unit ->{IO} I64
  foreign import ccall "ring_get_clo_mp_cons" getCloMpCons :: Unit ->{IO} I64

  -- =======================================================================
  -- Part A: 1,000,000 single-item MsgEnvelopes over SpscRing (0 alloc, 0 atomic RC)
  -- =======================================================================

  total-one-m :: U64
  total-one-m = 1000000

  push-one-m-loop :: SpscRing -> U64 ->{MVar, Diverge} Unit
  push-one-m-loop r seq =
    if ge-u64 seq total-one-m then ()
    else
      let msg = MsgEnvelope 1 0 1 seq (wrapping-mul-u64 seq 7) (bit-xor-u64 seq 0x5A5A5A5A) 0 0 0 0 in
      if spsc-try-push r msg then
        push-one-m-loop r (wrapping-add-u64 seq 1)
      else
        let _ = cpu-pause () in
        push-one-m-loop r seq

  worker-spsc-prod-one-m :: I64 ->{IO, MVar, Diverge} I64
  worker-spsc-prod-one-m base =
    let r = spsc-from-raw base in
    let _ = engine-thread-counters-reset () in
    let _ = push-one-m-loop r 0 in
    let allocs = engine-thread-alloc-count () in
    let rcs = engine-thread-atomic-rc-count () in
    if ne-u64 allocs 0 then -100
    else if ne-u64 rcs 0 then -101
    else 1000000

  pop-one-m-loop :: SpscRing -> U64 -> U64 ->{MVar, Diverge} I64
  pop-one-m-loop r expected checksum =
    if ge-u64 expected total-one-m then wrapping-u64-to-i64 checksum
    else
      let pop = spsc-try-pop-unboxed r in
      if pop-envelope-is-some pop then
        if ne-u64 pop.pop-seq expected then -201
        else if ne-u64 pop.pop-w0 (wrapping-mul-u64 expected 7) then -202
        else if ne-u64 pop.pop-w1 (bit-xor-u64 expected 0x5A5A5A5A) then -203
        else pop-one-m-loop r (wrapping-add-u64 expected 1) (wrapping-add-u64 checksum pop.pop-seq)
      else
        let _ = cpu-pause () in
        pop-one-m-loop r expected checksum

  worker-spsc-cons-one-m :: I64 ->{IO, MVar, Diverge} I64
  worker-spsc-cons-one-m base =
    let r = spsc-from-raw base in
    let _ = engine-thread-counters-reset () in
    let sum = pop-one-m-loop r 0 0 in
    let allocs = engine-thread-alloc-count () in
    let rcs = engine-thread-atomic-rc-count () in
    if lt-i64 sum 0 then sum
    else if ne-u64 allocs 0 then -210
    else if ne-u64 rcs 0 then -211
    else sum

  foreign export ccall "ring_spsc_prod_1m" worker-spsc-prod-one-m :: I64 ->{IO, MVar, Diverge} I64
  foreign export ccall "ring_spsc_cons_1m" worker-spsc-cons-one-m :: I64 ->{IO, MVar, Diverge} I64

  -- =======================================================================
  -- Part B: 1,000,000 batched MsgEnvelopes (batch=64) over SpscRing (0 alloc, 0 atomic RC)
  -- =======================================================================

  fill-batch-buf :: I64 -> U64 -> U64 -> U64 ->{MVar} Unit
  fill-batch-buf buf base-seq i n =
    if ge-u64 i n then ()
    else
      let s = wrapping-add-u64 base-seq i in
      let addr = wrapping-add-i64 buf (wrapping-shl-i64 (wrapping-u64-to-i64 i) 6) in
      let msg = MsgEnvelope 2 0 1 s (wrapping-add-u64 s 42) 0 0 0 0 0 in
      let _ = store-msg-envelope addr msg in
      fill-batch-buf buf base-seq (wrapping-add-u64 i 1) n

  push-batch-exact :: SpscRing -> I64 -> U64 ->{MVar, Diverge} Unit
  push-batch-exact r buf rem =
    if eq-u64 rem 0 then ()
    else
      let pushed = spsc-try-push-batch r buf rem in
      if eq-u64 pushed 0 then
        let _ = cpu-pause () in
        push-batch-exact r buf rem
      else
        let next-buf = wrapping-add-i64 buf (wrapping-shl-i64 (wrapping-u64-to-i64 pushed) 6) in
        push-batch-exact r next-buf (wrapping-sub-u64 rem pushed)

  batch-prod-loop :: SpscRing -> I64 -> U64 ->{MVar, Diverge} Unit
  batch-prod-loop r scratch seq =
    if ge-u64 seq total-one-m then ()
    else
      let rem = wrapping-sub-u64 total-one-m seq in
      let chunk = if lt-u64 rem 64 then rem else 64 in
      let _f = fill-batch-buf scratch seq 0 chunk in
      let _p = push-batch-exact r scratch chunk in
      batch-prod-loop r scratch (wrapping-add-u64 seq chunk)

  worker-batch-prod-one-m :: I64 ->{IO, MVar, Diverge} I64
  worker-batch-prod-one-m base =
    let r = spsc-from-raw base in
    let scratch-u = engine-dma-alloc 4096 4096 in
    if eq-u64 scratch-u 0 then -300
    else
      let scratch = wrapping-u64-to-i64 scratch-u in
      let _ = engine-thread-counters-reset () in
      let _ = batch-prod-loop r scratch 0 in
      let allocs = engine-thread-alloc-count () in
      let rcs = engine-thread-atomic-rc-count () in
      let _fr = engine-dma-free scratch-u 4096 in
      if ne-u64 allocs 0 then -301
      else if ne-u64 rcs 0 then -302
      else 1000000

  verify-batch-buf :: I64 -> U64 -> U64 -> U64 -> U64 ->{MVar} I64
  verify-batch-buf buf expected i n acc =
    if ge-u64 i n then wrapping-u64-to-i64 acc
    else
      let addr = wrapping-add-i64 buf (wrapping-shl-i64 (wrapping-u64-to-i64 i) 6) in
      let m = load-msg-envelope addr in
      let want = wrapping-add-u64 expected i in
      if ne-u64 m.msg-seq want then -310
      else if ne-u64 m.msg-w0 (wrapping-add-u64 want 42) then -311
      else verify-batch-buf buf expected (wrapping-add-u64 i 1) n (wrapping-add-u64 acc m.msg-seq)

  batch-cons-loop :: SpscRing -> I64 -> U64 -> U64 ->{MVar, Diverge} I64
  batch-cons-loop r scratch expected acc =
    if ge-u64 expected total-one-m then wrapping-u64-to-i64 acc
    else
      let rem = wrapping-sub-u64 total-one-m expected in
      let want = if lt-u64 rem 64 then rem else 64 in
      let popped = spsc-try-pop-batch r scratch want in
      if eq-u64 popped 0 then
        let _ = cpu-pause () in
        batch-cons-loop r scratch expected acc
      else
        let res = verify-batch-buf scratch expected 0 popped acc in
        if lt-i64 res 0 then res
        else batch-cons-loop r scratch (wrapping-add-u64 expected popped) (wrapping-i64-to-u64 res)

  worker-batch-cons-one-m :: I64 ->{IO, MVar, Diverge} I64
  worker-batch-cons-one-m base =
    let r = spsc-from-raw base in
    let scratch-u = engine-dma-alloc 4096 4096 in
    if eq-u64 scratch-u 0 then -320
    else
      let scratch = wrapping-u64-to-i64 scratch-u in
      let _ = engine-thread-counters-reset () in
      let sum = batch-cons-loop r scratch 0 0 in
      let allocs = engine-thread-alloc-count () in
      let rcs = engine-thread-atomic-rc-count () in
      let _fr = engine-dma-free scratch-u 4096 in
      if lt-i64 sum 0 then sum
      else if ne-u64 allocs 0 then -321
      else if ne-u64 rcs 0 then -322
      else sum

  foreign export ccall "ring_batch_prod_1m" worker-batch-prod-one-m :: I64 ->{IO, MVar, Diverge} I64
  foreign export ccall "ring_batch_cons_1m" worker-batch-cons-one-m :: I64 ->{IO, MVar, Diverge} I64

  -- =======================================================================
  -- Part C: Multi-Producer Disruptor (2 pinned producers -> 1 pinned consumer)
  --   Each producer claims 100,000 slots with `atomic-fetch-add-acqrel-i64`,
  --   writes `(producer_id, item_id, claimed_seq)` into the slot, and publishes.
  --   Consumer uses `SequenceBarrier` + `WaitAdaptivePark 32` to read all
  --   200,000 slots in strict sequence order `0..199,999`, verifying every
  --   sequence matches its slot, both producers delivered exact sums, and
  --   0 heap allocs / 0 atomic RC ops occurred!
  -- =======================================================================

  mp-per-prod :: U64
  mp-per-prod = 100000

  mp-total :: I64
  mp-total = 200000

  mp-prod-loop :: Disruptor -> U32 -> U64 ->{MVar, Diverge} Unit
  mp-prod-loop d pid i =
    if ge-u64 i mp-per-prod then ()
    else
      let seq = disruptor-claim-seq-blocking d WaitBusySpin in
      let addr = disruptor-slot-addr d seq in
      let msg = MsgEnvelope pid 0 2 (wrapping-i64-to-u64 seq) i 0 0 0 0 0 in
      let _w = store-msg-envelope addr msg in
      let _p = disruptor-publish-seq d seq in
      mp-prod-loop d pid (wrapping-add-u64 i 1)

  worker-mp-prod :: I64 -> U32 ->{IO, MVar, Diverge} I64
  worker-mp-prod base pid =
    let d = disruptor-from-raw base in
    let _ = engine-thread-counters-reset () in
    let _ = mp-prod-loop d pid 0 in
    let allocs = engine-thread-alloc-count () in
    let rcs = engine-thread-atomic-rc-count () in
    if ne-u64 allocs 0 then -400
    else if ne-u64 rcs 0 then -401
    else 100000

  worker-mp-prod-a :: I64 ->{IO, MVar, Diverge} I64
  worker-mp-prod-a base = worker-mp-prod base 1

  worker-mp-prod-b :: I64 ->{IO, MVar, Diverge} I64
  worker-mp-prod-b base = worker-mp-prod base 2

  mp-drain-range :: Disruptor -> I64 -> I64 -> U64 -> U64 ->{MVar} I64
  mp-drain-range d cur hi sum-a sum-b =
    if gt-i64 cur hi then
      -- Pack combined verification checksum `(sum-a + sum-b)`
      wrapping-u64-to-i64 (wrapping-add-u64 sum-a sum-b)
    else
      let m = disruptor-read-slot d cur in
      if ne-u64 m.msg-seq (wrapping-i64-to-u64 cur) then -410
      else if eq-u32 m.msg-kind 1 then
        mp-drain-range d (wrapping-add-i64 cur 1) hi (wrapping-add-u64 sum-a m.msg-w0) sum-b
      else if eq-u32 m.msg-kind 2 then
        mp-drain-range d (wrapping-add-i64 cur 1) hi sum-a (wrapping-add-u64 sum-b m.msg-w0)
      else -411

  mp-cons-loop :: SequenceBarrier -> Disruptor -> WaitStrategy -> I64 -> U64 ->{MVar, Diverge} I64
  mp-cons-loop bar d strat next-seq total-payload =
    if ge-i64 next-seq mp-total then wrapping-u64-to-i64 total-payload
    else
      let avail = barrier-wait-for bar strat next-seq in
      let last = if lt-i64 avail mp-total then avail else wrapping-sub-i64 mp-total 1 in
      let delta = mp-drain-range d next-seq last 0 0 in
      if lt-i64 delta 0 then delta
      else
        let _c = disruptor-commit-consumer d last in
        mp-cons-loop bar d strat (wrapping-add-i64 last 1) (wrapping-add-u64 total-payload (wrapping-i64-to-u64 delta))

  worker-mp-cons :: I64 ->{IO, MVar, Diverge} I64
  worker-mp-cons base =
    let d = disruptor-from-raw base in
    let bar = barrier-new d Nil in
    let strat = WaitAdaptivePark 32 in
    let _ = engine-thread-counters-reset () in
    let total = mp-cons-loop bar d strat 0 0 in
    let allocs = engine-thread-alloc-count () in
    let rcs = engine-thread-atomic-rc-count () in
    if lt-i64 total 0 then total
    else if ne-u64 allocs 0 then -420
    else if ne-u64 rcs 0 then -421
    else total

  foreign export ccall "ring_mp_prod_a" worker-mp-prod-a :: I64 ->{IO, MVar, Diverge} I64
  foreign export ccall "ring_mp_prod_b" worker-mp-prod-b :: I64 ->{IO, MVar, Diverge} I64
  foreign export ccall "ring_mp_cons" worker-mp-cons :: I64 ->{IO, MVar, Diverge} I64

  -- Expected sum(0..999_999) = 499_999_500_000
  expected-sum-one-m :: I64
  expected-sum-one-m = 499999500000

  -- Expected 2 * sum(0..99_999) = 2 * 4_999_950_000 = 9_999_900_000
  expected-mp-sum :: I64
  expected-mp-sum = 9999900000

  main :: Unit ->{IO, MVar} Int
  main _ =
    -- 1. Single-item 1M SPSC stress
    case spsc-new 4096 of
      None -> 1
      Some r1 ->
        let wp1 = engine-spawn-pinned (core-id 0) (wrapping-i64-to-u64 (getCloSpscProd ())) (spsc-base-addr r1) in
        let wc1 = engine-spawn-pinned (core-id 1) (wrapping-i64-to-u64 (getCloSpscCons ())) (spsc-base-addr r1) in
        let rp1 = engine-join wp1 in
        let rc1 = engine-join wc1 in
        let _f1 = spsc-destroy r1 in
        if ne-i64 rp1 1000000 then 2
        else if ne-i64 rc1 expected-sum-one-m then 3
        else
          -- 2. Batched 1M SPSC stress
          case spsc-new 4096 of
            None -> 4
            Some r2 ->
              let wp2 = engine-spawn-pinned (core-id 0) (wrapping-i64-to-u64 (getCloBatchProd ())) (spsc-base-addr r2) in
              let wc2 = engine-spawn-pinned (core-id 1) (wrapping-i64-to-u64 (getCloBatchCons ())) (spsc-base-addr r2) in
              let rp2 = engine-join wp2 in
              let rc2 = engine-join wc2 in
              let _f2 = spsc-destroy r2 in
              if ne-i64 rp2 1000000 then 5
              else if ne-i64 rc2 expected-sum-one-m then 6
              else
                -- 3. Multi-producer Disruptor stress (2 producers -> 1 consumer, WaitAdaptivePark)
                case disruptor-new 4096 ProducerMulti of
                  None -> 7
                  Some d ->
                    let wpa = engine-spawn-pinned (core-id 0) (wrapping-i64-to-u64 (getCloMpProdA ())) d.dis-base in
                    let wpb = engine-spawn-pinned (core-id 1) (wrapping-i64-to-u64 (getCloMpProdB ())) d.dis-base in
                    let wmc = engine-spawn-pinned (core-id 2) (wrapping-i64-to-u64 (getCloMpCons ())) d.dis-base in
                    let rpa = engine-join wpa in
                    let rpb = engine-join wpb in
                    let rmc = engine-join wmc in
                    let _fd = disruptor-destroy d in
                    if ne-i64 rpa 100000 then 8
                    else if ne-i64 rpb 100000 then 9
                    else if ne-i64 rmc expected-mp-sum then 10
                    else 0
"#;

    let (elapsed, _stderr) =
        build_and_run_leak_checked("pinned-1m-stress", src, &[bridge], 0);
    let secs = elapsed.as_secs_f64();
    let total_msgs = 2_200_000.0;
    println!(
        "[Engine.Ring 1M+1M SPSC + 200k MP Disruptor] elapsed={:.3}s aggregate={:.2}M msgs/sec (0 heap allocs, 0 atomic RC, leaked=0)",
        secs,
        (total_msgs / secs) / 1e6
    );
}

// =========================================================================
// 7. `ShmRing` cross-mapping zero-copy IPC test over a real temp-file `mmap`
// =========================================================================

#[test]
fn shm_ring_temp_file_mmap_init_attach_roundtrip_and_detach() {
    // Create and pre-size a temporary backing file from Rust so the test runs
    // cleanly on both macOS and Linux (using `sys-mmap`/`sys-munmap` in Cinch).
    let tmp_dir = fresh_dir("shm-file-backing");
    let shm_file = tmp_dir.join("cinch_ring_ipc.shm");
    let region_bytes: u64 = 16384; // 4 pages (> 384 + 128*64 = 8576 bytes)
    {
        let f = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&shm_file)
            .expect("create shm backing file");
        f.set_len(region_bytes).expect("size shm backing file");
    }

    // Tiny C helper opening the temp path O_RDWR so Cinch gets a live OS `Fd`
    // portably across macOS and Linux, then Cinch calls `shm-ring-map-fd` twice
    // (`MAP_SHARED`), `shm-ring-init`, `shm-ring-attach`, `shm-ring-try-push`,
    // `shm-ring-try-pop`, and `shm-ring-detach`!
    let path_c = shm_file.to_string_lossy().replace('\\', "\\\\").replace('"', "\\\"");
    let open_bridge = c_fixture(
        "shm_open_bridge",
        &format!(
            r#"
#include <fcntl.h>
#include <unistd.h>
#include <stdint.h>

int32_t test_open_shm_file(void) {{
    return (int32_t)open("{path_c}", O_RDWR);
}}
int32_t test_close_shm_fd(int32_t fd) {{
    return (int32_t)close(fd);
}}
"#
        ),
    );

    let src = r#"
module Main where
  import Data.Machine
  import Data.Option
  import Engine.Types
  import Engine.Ring.Spsc
  import Engine.Ring.Shm
  import Prim.Atomic
  import Prim.Sys.Io

  foreign import ccall "test_open_shm_file" openShmFd :: forall r. Unit ->{IO r} I32
  foreign import ccall "test_close_shm_fd" closeShmFd :: forall r. I32 ->{IO r} I32

  make-ipc-msg :: U64 -> MsgEnvelope
  make-ipc-msg seq =
    msg-envelope-make 99 (core-id 3) (core-id 7) seq
      (wrapping-add-u64 seq 1000)
      (wrapping-mul-u64 seq 13)
      0x1122334455667788
      0xAABBCCDDEEFF0011
      0x0102030405060708
      0xF0E0D0C0B0A09080

  push-n :: ShmRing -> U64 -> U64 ->{MVar} Bool
  push-n r i n =
    if ge-u64 i n then True
    else if shm-ring-try-push r (make-ipc-msg i) then push-n r (wrapping-add-u64 i 1) n
    else False

  pop-verify-n :: ShmRing -> U64 -> U64 ->{MVar} Bool
  pop-verify-n r i n =
    if ge-u64 i n then True
    else
      let pop = shm-ring-try-pop-unboxed r in
      if not (pop-envelope-is-some pop) then False
      else if not (msg-envelope-eq (pop-envelope-msg pop) (make-ipc-msg i)) then False
      else pop-verify-n r (wrapping-add-u64 i 1) n

  run-shm-test :: forall r. using (io :: Io r) => Unit ->{IO r, MVar} I64
  run-shm-test _ =
    let raw-fd = openShmFd () in
    if lt-i32 raw-fd 0 then 1
    else
      let fd = fd-of raw-fd in
      let region-bytes = 16384 in
      -- 1. Attaching BEFORE init must fail (magic is 0, not "CINCHRIN")
      case shm-ring-map-fd fd region-bytes 0 False of
        Some bad ->
          let _ = shm-ring-detach bad in
          let _ = closeShmFd raw-fd in 2
        None ->
          -- 2. Producer maps + initializes SHM ring (cap = 128)
          case shm-ring-map-fd fd region-bytes 128 True of
            None ->
              let _ = closeShmFd raw-fd in 3
            Some prod-ring ->
              -- 3. Consumer maps the SAME file independently at a distinct virtual address and attaches!
              case shm-ring-map-fd fd region-bytes 0 False of
                None ->
                  let _ = shm-ring-detach prod-ring in
                  let _ = closeShmFd raw-fd in 4
                Some cons-ring ->
                  -- Verify two distinct virtual mappings backed by the same shared file
                  if eq-i64 prod-ring.shm-base cons-ring.shm-base then
                    let _ = shm-ring-detach cons-ring in
                    let _ = shm-ring-detach prod-ring in
                    let _ = closeShmFd raw-fd in 5
                  else if ne-u64 (shm-ring-capacity cons-ring) 128 then
                    let _ = shm-ring-detach cons-ring in
                    let _ = shm-ring-detach prod-ring in
                    let _ = closeShmFd raw-fd in 6
                  else
                    -- 4. Push 100 messages via mapping #1, pop & verify bit-exact via mapping #2
                    let ok-p1 = push-n prod-ring 0 100 in
                    let len1 = shm-ring-len cons-ring in
                    let ok-c1 = pop-verify-n cons-ring 0 100 in
                    -- 5. Wrap around the 128-slot ring: push 100 more (indices 100..199)
                    let ok-p2 = push-n prod-ring 100 200 in
                    let ok-c2 = pop-verify-n cons-ring 100 200 in
                    let empty-after = not (pop-envelope-is-some (shm-ring-try-pop-unboxed cons-ring)) in
                    let d1 = shm-ring-detach cons-ring in
                    let d2 = shm-ring-detach prod-ring in
                    let _cl = closeShmFd raw-fd in
                    if ok-p1 && eq-u64 len1 100 && ok-c1 && ok-p2 && ok-c2 && empty-after && io-error-is-ok d1 && io-error-is-ok d2
                    then 0
                    else 7

  main :: Unit ->{IO r, MVar} Int
  main _ = with (io = newIO ()) in
    int-from-i64 (run-shm-test ())
"#;

    let (_, _) = build_and_run_leak_checked("shm-ipc-mmap", src, &[open_bridge], 0);
    let _ = std::fs::remove_dir_all(&tmp_dir);
}
