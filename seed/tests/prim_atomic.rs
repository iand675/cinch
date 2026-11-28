//! Integration, `mem_contract` audit, TCB allowlist, and multi-threaded pinned-worker
//! stress tests for `Prim.Atomic` (`seed/prelude/Prim/Atomic.cinch`).

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{DriverOptions, Mode};
use cinch::prim_verify::mem_contract::audit_file_contracts;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Compile a small C fixture to an object file and return its path.
fn c_fixture(name: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cinch-prim-atomic-{}-{}", name, std::process::id()));
    std::fs::create_dir_all(&dir).expect("scratch dir");
    let c_path = dir.join(format!("{name}.c"));
    let o_path = dir.join(format!("{name}.o"));
    std::fs::write(&c_path, src).expect("write fixture");
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

fn run_project(name: &str, src: &str, objects: &[PathBuf], expected: i32) {
    let dir = std::env::temp_dir().join(format!("cinch-prim-atomic-run-{}-{}", name, std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create project dir");
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let mut opts = DriverOptions::default();
    opts.native_objects = objects.to_vec();
    let res = common::drive_with_profile_opts(
        Mode::Run,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        None,
        &TargetProfile::native(),
        &opts,
    );
    let _ = std::fs::remove_dir_all(&dir);
    match res {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation/run failed: {e}"),
    }
}

#[test]
fn prim_atomic_mem_contract_audit_and_tcb_allowlist_pass() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir.parent().expect("repo root");
    let atomic_path = manifest_dir.join("prelude/Prim/Atomic.cinch");

    // 1. Direct Rust `mem_contract` audit: 0 violations, covering all atomics/fences/raw ops/asm.
    let report = audit_file_contracts(&atomic_path).expect("audit Prim/Atomic.cinch");
    assert!(
        report.violations.is_empty(),
        "Prim.Atomic must have 0 mem_contract violations, got: {:?}",
        report.violations
    );
    assert!(
        report.atomic_ops_checked >= 14,
        "expected >= 14 atomic/fence ops checked, got {}",
        report.atomic_ops_checked
    );
    assert!(
        report.memory_ops_checked >= 10,
        "expected >= 10 raw memory ops checked, got {}",
        report.memory_ops_checked
    );
    assert!(
        report.inline_asm_checked >= 6,
        "expected >= 6 inline asm hints (pause/yield/prefetch) checked, got {}",
        report.inline_asm_checked
    );

    // 2. CLI `cinch verify-prims --audit-unsafe seed/prelude/Prim/Atomic.cinch` exits 0.
    let cinch_bin = env!("CARGO_BIN_EXE_cinch");
    let cli_out = Command::new(cinch_bin)
        .arg("verify-prims")
        .arg("--audit-unsafe")
        .arg(&atomic_path)
        .output()
        .expect("run cinch verify-prims --audit-unsafe");
    assert!(
        cli_out.status.success(),
        "cinch verify-prims --audit-unsafe failed:\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&cli_out.stdout),
        String::from_utf8_lossy(&cli_out.stderr)
    );

    // 3. Check a real build manifest's `Prim.Atomic` prim entries against `tcb_allowlist.json`.
    let tmp = std::env::temp_dir().join(format!("cinch_prim_atomic_tcb_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).expect("create temp project");
    std::fs::write(
        tmp.join("Main.cinch"),
        r#"
module Main where
  import Prim.Atomic

  main :: Unit ->{MVar} Int
  main _ =
    let _ = atomic-fence-seqcst () in
    let _ = cpu-pause () in
    0
"#,
    )
    .expect("write Main.cinch");

    common::drive_with_profile(
        Mode::Check,
        StageMode::Refresh,
        &tmp,
        "Main.cinch",
        None,
        &TargetProfile::native(),
    )
    .expect("check program importing Prim.Atomic");

    let manifest_path = tmp.join("build/Main.manifest.json");
    let manifest_val: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&manifest_path).expect("read Main.manifest.json"),
    )
    .expect("parse Main.manifest.json");
    let atomic_prims: Vec<serde_json::Value> = manifest_val
        .get("prims")
        .and_then(|v| v.as_array())
        .expect("manifest prims array")
        .iter()
        .filter(|p| p.get("module").and_then(|m| m.as_str()) == Some("Prim.Atomic"))
        .cloned()
        .collect();
    assert!(
        atomic_prims.len() >= 25,
        "expected >= 25 Prim.Atomic prims in manifest, got {}",
        atomic_prims.len()
    );
    let filtered_manifest = tmp.join("PrimAtomic.manifest.json");
    std::fs::write(
        &filtered_manifest,
        serde_json::to_string_pretty(&serde_json::json!({
            "entry": "Main",
            "prims": atomic_prims,
        }))
        .unwrap(),
    )
    .expect("write PrimAtomic.manifest.json");

    let allowlist_script = repo_root.join("tooling/verification/check_tcb_allowlist.py");
    let tcb_out = Command::new("python3")
        .arg(&allowlist_script)
        .arg("--strict-jpl")
        .arg(&filtered_manifest)
        .output()
        .expect("run check_tcb_allowlist.py");
    assert!(
        tcb_out.status.success(),
        "tcb_allowlist check failed:\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&tcb_out.stdout),
        String::from_utf8_lossy(&tcb_out.stderr)
    );
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn prim_atomic_dma_scalar_ops_fences_raw_access_and_u64_wrappers() {
    let src = r#"
module Main where
  import Prim.Atomic
  import Data.Wide

  foreign import ccall "cinch_engine_dma_alloc" dmaAlloc :: I64 -> I64 ->{IO} I64
  foreign import ccall "cinch_engine_dma_free" dmaFree :: I64 -> I64 ->{IO} I64

  main :: Unit ->{IO, MVar} Int
  main _ =
    let page = dmaAlloc 4096 4096 in
    if page == 0 then 1
    else
      -- Prefetch and pause hints on live DMA page
      let _ = prefetch-write page in
      let _ = prefetch-read page in
      let _ = cpu-pause () in

      -- 1. Relaxed / acquire-release / seq_cst I64 store & load
      let _ = atomic-store-relaxed-i64 page 17 in
      let v1 = atomic-load-relaxed-i64 page in
      if v1 /= 17 then 2
      else
        let _ = atomic-store-release-i64 page 42 in
        let v2 = atomic-load-acquire-i64 page in
        if v2 /= 42 then 3
        else
          let _ = atomic-store-seqcst-i64 page -99 in
          let v3 = atomic-load-seqcst-i64 page in
          if v3 /= -99 then 4
          else
            -- 2. CAS acq_rel and seq_cst (failure and success)
            let casFail1 = atomic-cas-acqrel-i64 page 0 123 in
            if casFail1 then 5
            else if atomic-load-acquire-i64 page /= -99 then 6
            else
              let casOk1 = atomic-cas-acqrel-i64 page -99 1000 in
              if not casOk1 then 7
              else if atomic-load-acquire-i64 page /= 1000 then 8
              else
                let casFail2 = atomic-cas-seqcst-i64 page 999 2000 in
                if casFail2 then 9
                else
                  let casOk2 = atomic-cas-seqcst-i64 page 1000 2000 in
                  if not casOk2 then 10
                  else if atomic-load-seqcst-i64 page /= 2000 then 11
                  else
                    -- 3. Fetch-add (acq_rel & relaxed) and swap (acq_rel)
                    let old1 = atomic-fetch-add-acqrel-i64 page 345 in
                    if old1 /= 2000 then 12
                    else if atomic-load-acquire-i64 page /= 2345 then 13
                    else
                      let old2 = atomic-fetch-add-relaxed-i64 page -45 in
                      if old2 /= 2345 then 14
                      else if atomic-load-relaxed-i64 page /= 2300 then 15
                      else
                        let old3 = atomic-swap-acqrel-i64 page 7777 in
                        if old3 /= 2300 then 16
                        else if atomic-load-acquire-i64 page /= 7777 then 17
                        else
                          -- 4. Memory fences
                          let _ = atomic-fence-release () in
                          let _ = atomic-fence-acquire () in
                          let _ = atomic-fence-seqcst () in

                          -- 5. Raw non-atomic loads/stores (i64, u8, u32, u64, u128)
                          let offI64 = wrapping-add-i64 page 128 in
                          let _ = raw-store-i64 offI64 -1234567890123 in
                          if raw-load-i64 offI64 /= -1234567890123 then 18
                          else
                            let offU8 = wrapping-add-i64 page 144 in
                            let _ = raw-store-u8 offU8 0xA5 in
                            let _ = raw-store-u8 (wrapping-add-i64 offU8 1) 0x5A in
                            if ne-u8 (raw-load-u8 offU8) 0xA5 then 19
                            else if ne-u8 (raw-load-u8 (wrapping-add-i64 offU8 1)) 0x5A then 20
                            else
                              let offU32 = wrapping-add-i64 page 160 in
                              let _ = raw-store-u32 offU32 0xDEADBEEF in
                              if ne-u32 (raw-load-u32 offU32) 0xDEADBEEF then 21
                              else
                                let offU64 = wrapping-add-i64 page 176 in
                                let hiU64 = 18446744073709551600 in
                                let _ = raw-store-u64 offU64 hiU64 in
                                if ne-u64 (raw-load-u64 offU64) hiU64 then 22
                                else
                                  let offU128 = wrapping-add-i64 page 192 in
                                  let wideVal = simd-u64x2-pack 0x0123456789ABCDEF 0xFEDCBA9876543210 in
                                  let _ = raw-store-u128 offU128 wideVal in
                                  let wideGot = raw-load-u128 offU128 in
                                  if ne-u128 wideGot wideVal then 23
                                  else
                                    -- 6. U64 atomic wrappers (including values with bit 63 set)
                                    let offAtomU64 = wrapping-add-i64 page 256 in
                                    let baseU64 = 10000000000000000000 in
                                    let _ = atomic-store-relaxed-u64 offAtomU64 baseU64 in
                                    if ne-u64 (atomic-load-relaxed-u64 offAtomU64) baseU64 then 24
                                    else
                                      let _ = atomic-store-release-u64 offAtomU64 (wrapping-add-u64 baseU64 1) in
                                      if ne-u64 (atomic-load-acquire-u64 offAtomU64) (wrapping-add-u64 baseU64 1) then 25
                                      else
                                        let _ = atomic-store-seqcst-u64 offAtomU64 (wrapping-add-u64 baseU64 2) in
                                        if ne-u64 (atomic-load-seqcst-u64 offAtomU64) (wrapping-add-u64 baseU64 2) then 26
                                        else
                                          let uCasOk = atomic-cas-acqrel-u64 offAtomU64 (wrapping-add-u64 baseU64 2) (wrapping-add-u64 baseU64 10) in
                                          if not uCasOk then 27
                                          else
                                            let uCasOk2 = atomic-cas-seqcst-u64 offAtomU64 (wrapping-add-u64 baseU64 10) (wrapping-add-u64 baseU64 20) in
                                            if not uCasOk2 then 28
                                            else
                                              let uOld1 = atomic-fetch-add-acqrel-u64 offAtomU64 5 in
                                              if ne-u64 uOld1 (wrapping-add-u64 baseU64 20) then 29
                                              else
                                                let uOld2 = atomic-fetch-add-relaxed-u64 offAtomU64 7 in
                                                if ne-u64 uOld2 (wrapping-add-u64 baseU64 25) then 30
                                                else
                                                  let uOld3 = atomic-swap-acqrel-u64 offAtomU64 999 in
                                                  if ne-u64 uOld3 (wrapping-add-u64 baseU64 32) then 31
                                                  else if ne-u64 (atomic-load-acquire-u64 offAtomU64) 999 then 32
                                                  else
                                                    let rc = dmaFree page 4096 in
                                                    if rc /= 0 then 33
                                                    else 0
"#;
    run_project("dma-scalar-and-raw", src, &[], 0);
}

#[test]
fn prim_atomic_pinned_workers_fetch_add_and_spsc_ping_pong() {
    // Static immortal closure bridge for `cinch_engine_spawn_pinned`, matching
    // `seed/tests/engine_rt_worker.rs` and `seed/runtime/src/task.rs`.
    let bridge = c_fixture(
        "prim_atomic_worker_bridge",
        r#"
#include <stdint.h>

extern int64_t worker_inc_counter(int64_t dma_page);
extern int64_t worker_spsc_producer(int64_t dma_page);
extern int64_t worker_spsc_consumer(int64_t dma_page);

typedef struct {
    uint64_t header;
    int64_t code;
    int64_t arity;
    int64_t nfilled;
} CinchStaticClosure;

/* KIND_CLOSURE = 2, bits 35..40 (`block_kinds.rs`: RC_BITS = 35); rc = 0 (RC_IMMORTAL). */
#define IMMORTAL_CLOSURE_HEADER ((uint64_t)2ULL << 35)

static int64_t stub_inc_counter(int64_t self, int64_t argc, int64_t *argv) {
    (void)self;
    (void)argc;
    return worker_inc_counter(argv[0]);
}

static int64_t stub_spsc_producer(int64_t self, int64_t argc, int64_t *argv) {
    (void)self;
    (void)argc;
    return worker_spsc_producer(argv[0]);
}

static int64_t stub_spsc_consumer(int64_t self, int64_t argc, int64_t *argv) {
    (void)self;
    (void)argc;
    return worker_spsc_consumer(argv[0]);
}

static CinchStaticClosure clo_inc;
static CinchStaticClosure clo_prod;
static CinchStaticClosure clo_cons;

int64_t atomic_get_inc_closure(void) {
    clo_inc.header = IMMORTAL_CLOSURE_HEADER;
    clo_inc.code = (int64_t)(uintptr_t)stub_inc_counter;
    clo_inc.arity = 1;
    clo_inc.nfilled = 0;
    return (int64_t)(uintptr_t)&clo_inc.code;
}

int64_t atomic_get_prod_closure(void) {
    clo_prod.header = IMMORTAL_CLOSURE_HEADER;
    clo_prod.code = (int64_t)(uintptr_t)stub_spsc_producer;
    clo_prod.arity = 1;
    clo_prod.nfilled = 0;
    return (int64_t)(uintptr_t)&clo_prod.code;
}

int64_t atomic_get_cons_closure(void) {
    clo_cons.header = IMMORTAL_CLOSURE_HEADER;
    clo_cons.code = (int64_t)(uintptr_t)stub_spsc_consumer;
    clo_cons.arity = 1;
    clo_cons.nfilled = 0;
    return (int64_t)(uintptr_t)&clo_cons.code;
}
"#,
    );

    let src = r#"
module Main where
  import Prim.Atomic

  foreign import ccall "cinch_engine_dma_alloc" dmaAlloc :: I64 -> I64 ->{IO} I64
  foreign import ccall "cinch_engine_dma_free" dmaFree :: I64 -> I64 ->{IO} I64
  foreign import ccall "cinch_engine_spawn_pinned" spawnPinned :: I64 -> I64 -> I64 ->{IO} I64
  foreign import ccall "cinch_engine_join" joinPinned :: I64 ->{IO} I64
  foreign import ccall "atomic_get_inc_closure" getIncClosure :: Unit ->{IO} I64
  foreign import ccall "atomic_get_prod_closure" getProdClosure :: Unit ->{IO} I64
  foreign import ccall "atomic_get_cons_closure" getConsClosure :: Unit ->{IO} I64

  -- 100k atomic fetch-add loop on shared DMA address
  incLoop :: I64 -> I64 ->{MVar} I64
  incLoop addr rem =
    if rem <= 0 then 1
    else
      let _ = atomic-fetch-add-acqrel-i64 addr 1 in
      incLoop addr (wrapping-sub-i64 rem 1)

  workerIncCounter :: I64 ->{IO, MVar} I64
  workerIncCounter page = incLoop page 100000

  foreign export ccall "worker_inc_counter" workerIncCounter :: I64 ->{IO, MVar} I64

  -- Acquire/release SPSC ping-pong across 128-byte-isolated cache lines:
  --   page + 128: head (producer release -> consumer acquire)
  --   page + 256: tail (consumer release -> producer acquire)
  --   page + 384: payload slot (plain raw-store-i64 / raw-load-i64)
  waitEqual :: I64 -> I64 ->{MVar, Diverge} Unit
  waitEqual addr want =
    if atomic-load-acquire-i64 addr == want then ()
    else
      let _ = cpu-pause () in
      waitEqual addr want

  produceRounds :: I64 -> I64 -> I64 -> I64 -> I64 ->{MVar, Diverge} I64
  produceRounds headAddr tailAddr slotAddr seq limit =
    if seq > limit then seq
    else
      let _ = waitEqual tailAddr (wrapping-sub-i64 seq 1) in
      let _ = raw-store-i64 slotAddr seq in
      let _ = atomic-store-release-i64 headAddr seq in
      produceRounds headAddr tailAddr slotAddr (wrapping-add-i64 seq 1) limit

  consumeRounds :: I64 -> I64 -> I64 -> I64 -> I64 -> I64 ->{MVar, Diverge} I64
  consumeRounds headAddr tailAddr slotAddr seq limit acc =
    if seq > limit then acc
    else
      let _ = waitEqual headAddr seq in
      let v = raw-load-i64 slotAddr in
      let _ = atomic-store-release-i64 tailAddr seq in
      consumeRounds headAddr tailAddr slotAddr (wrapping-add-i64 seq 1) limit (wrapping-add-i64 acc v)

  workerSpscProducer :: I64 ->{IO, MVar, Diverge} I64
  workerSpscProducer page =
    produceRounds
      (wrapping-add-i64 page 128)
      (wrapping-add-i64 page 256)
      (wrapping-add-i64 page 384)
      1
      10000

  workerSpscConsumer :: I64 ->{IO, MVar, Diverge} I64
  workerSpscConsumer page =
    consumeRounds
      (wrapping-add-i64 page 128)
      (wrapping-add-i64 page 256)
      (wrapping-add-i64 page 384)
      1
      10000
      0

  foreign export ccall "worker_spsc_producer" workerSpscProducer :: I64 ->{IO, MVar, Diverge} I64
  foreign export ccall "worker_spsc_consumer" workerSpscConsumer :: I64 ->{IO, MVar, Diverge} I64

  main :: Unit ->{IO, MVar} Int
  main _ =
    let page = dmaAlloc 4096 4096 in
    if page == 0 then 1
    else
      -- Part A: 2 pinned OS workers x 100,000 atomic-fetch-add-acqrel-i64 == 200,000
      let _ = atomic-store-release-i64 page 0 in
      let incClo = getIncClosure () in
      let w0 = spawnPinned 0 incClo page in
      let w1 = spawnPinned 1 incClo page in
      if w0 <= 0 || w1 <= 0 then 2
      else
        let r0 = joinPinned w0 in
        let r1 = joinPinned w1 in
        if r0 /= 1 || r1 /= 1 then 3
        else
          let total = atomic-load-acquire-i64 page in
          if total /= 200000 then 4
          else
            -- Part B: 2 pinned OS workers acquire/release ping-pong for 10,000 rounds
            let _ = atomic-store-release-i64 (wrapping-add-i64 page 128) 0 in
            let _ = atomic-store-release-i64 (wrapping-add-i64 page 256) 0 in
            let _ = raw-store-i64 (wrapping-add-i64 page 384) 0 in
            let prodClo = getProdClosure () in
            let consClo = getConsClosure () in
            let wp = spawnPinned 0 prodClo page in
            let wc = spawnPinned 1 consClo page in
            if wp <= 0 || wc <= 0 then 5
            else
              let rp = joinPinned wp in
              let rc = joinPinned wc in
              if rp /= 10001 then 6
              else if rc /= 50005000 then 7
              else
                let freed = dmaFree page 4096 in
                if freed /= 0 then 8
                else 0
"#;
    run_project("pinned-workers", src, &[bridge], 0);
}
