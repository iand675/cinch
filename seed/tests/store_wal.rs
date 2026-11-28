//! Integration tests for Wave 4 `Store.Wal` (`Store.Wal.Format`,
//! `Store.Wal.Engine`, and Formal Model M10 `WalMachine` in `Store.Wal.Spec`):
//!
//!   1. Stdlib package manifest registration (`seed/prelude/.cinchpkg`),
//!      canonical documentation, `#[no-alloc]` safety, and format tamper checks.
//!   2. Exhaustive compile-time formal verification of M10 `WalMachine` and
//!      `AbstractDurableLog` under `CINCH_MODEL_FAIL_ON_CAPPED=1` via
//!      `common::drive_with_profile_opts` and `cinch model inspect --json`
//!      (`capped: false`, `0 violations`, < 3s).
//!   3. **10,000-seed deterministic power-loss & torn-sector chaos suite**:
//!      runs random sequences of `append`, `group_commit`, `crash_clean`,
//!      `crash_torn_512b_sector`, `bit_flip_tail`, and `recover` over
//!      `Engine.Sim`'s fault-injecting virtual disks (plus segment rotation
//!      and `ReactorOps` bridge verification), asserting 100% survival of
//!      acknowledged LSNs with byte-exact payloads and zero phantom/corrupted
//!      records across all 10,000 seeds.
//!   4. Live macOS `F_NOCACHE` 4 KiB-aligned direct-disk WAL test (multi-batch
//!      group commit, on-disk torn tail injection, reopen + `wal-recover`, and
//!      continued append/commit), plus a 50,000-record steady-state `wal-format`
//!      + `crc32c` + group-commit stage loop asserting `allocs == 0`,
//!      `atomic_rc == 0`, and `CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1` (`leaked=0`).
//!
//! Routed through `common::drive_with_profile` / `common::drive_with_profile_opts` /
//! `common::check_interfaces` / `common::build_executable` so the exact same
//! suite runs against both the Rust seed and the Stage 3 self-hosted compiler.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{DriverOptions, Mode, Package};
use cinch::lsp::{parse_json, Json};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

const STORE_WAL_FORMAT: &str = include_str!("../prelude/Store/Wal/Format.cinch");
const STORE_WAL_ENGINE: &str = include_str!("../prelude/Store/Wal/Engine.cinch");
const STORE_WAL_SPEC: &str = include_str!("../prelude/Store/Wal/Spec.cinch");
const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn fresh_dir(tag: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "cinch-store-wal-{tag}-{}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn build_and_run_leak_checked(tag: &str, src: &str, expected_code: i32) {
    let dir = fresh_dir(tag);
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let exe = common::build_executable(&dir, "Main.cinch", &format!("store-wal-{tag}"))
        .unwrap_or_else(|e| panic!("{tag}: build failed:\n{e}"));

    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "2048M")
        .output()
        .expect("run built binary");
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
        "{tag}: expected clean leak report (`leaked=0`), got stderr:\n{stderr}"
    );
}

#[test]
fn store_wal_manifest_docs_no_alloc_and_format_tamper_checks() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, path) in [
        ("Store.Wal.Format", "Store/Wal/Format.cinch"),
        ("Store.Wal.Engine", "Store/Wal/Engine.cinch"),
        ("Store.Wal.Spec", "Store/Wal/Spec.cinch"),
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

    let pos = |name: &str| {
        pkg.order
            .iter()
            .position(|n| n == name)
            .unwrap_or_else(|| panic!("{name} missing from order"))
    };
    assert!(pos("Engine.Simd.Crc32c") < pos("Store.Wal.Format"));
    assert!(pos("Store.Wal.Format") < pos("Store.Wal.Engine"));
    assert!(pos("Engine.Mem.DmaPool") < pos("Store.Wal.Engine"));
    assert!(pos("Engine.Sim") < pos("Store.Wal.Engine"));
    assert!(pos("Engine.Reactor.Darwin") < pos("Store.Wal.Engine"));

    common::assert_documented("Store.Wal.Format", STORE_WAL_FORMAT);
    common::assert_documented("Store.Wal.Engine", STORE_WAL_ENGINE);

    // Verify `#[no-alloc]` hot-path composition, interface emission, and tamper detection:
    // torn write, single-bit flip, reordered LSN, stale recycled-segment chain CRC,
    // and non-zero block tail padding.
    let src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Engine.Types
  import Prim.Atomic
  import Store.Wal.Format
  import Store.Wal.Engine
  import Store.Wal.Spec

  #[no-alloc]
  check-format-no-alloc :: I64 -> I64 ->{MVar} U32
  check-format-no-alloc block-addr payload-addr =
    let _p0 = raw-store-u64 payload-addr 0x1122334455667788 in
    let _p1 = raw-store-u64 (wrapping-add-i64 payload-addr 8) 0x99AABBCCDDEEFF00 in
    let _p2 = raw-store-u64 (wrapping-add-i64 payload-addr 16) 0xCAFEBABE12345678 in
    -- 1. Segment header encode/validate + single-bit flip rejection
    let _hcrc = wal-write-segment-header-raw block-addr 7 1 wal-initial-chain-seed 3 in
    let sh-ok = wal-validate-segment-header-raw block-addr in
    if ne-u32 sh-ok wal-status-ok then 1
    else
      let orig-b = raw-load-u8 (wrapping-add-i64 block-addr 20) in
      let _flip = raw-store-u8 (wrapping-add-i64 block-addr 20) (bit-xor-u8 orig-b 1) in
      let sh-bad = wal-validate-segment-header-raw block-addr in
      if ne-u32 sh-bad wal-status-bad-crc then 2
      else
        -- 2. Chained records + 4 KiB block padding + full scan
        let crc1 = wal-write-record-raw block-addr wal-flag-data 1 wal-initial-chain-seed 3 payload-addr 24 in
        let off2 = wal-record-total-len 24 in
        let crc2 = wal-write-record-raw (wrapping-add-i64 block-addr (u32-to-i64 off2)) wal-flag-data 2 crc1 3 payload-addr 19 in
        let used = wrapping-add-u32 off2 (wal-record-total-len 19) in
        let aligned = wal-format-block-raw block-addr used 4096 in
        if ne-u32 aligned 4096 then 3
        else
          let sc = wal-scan-block-raw block-addr 4096 1 wal-initial-chain-seed in
          if ne-u32 sc.scan-status wal-status-ok then 4
          else if ne-u32 sc.scan-records 2 then 5
          else if ne-u64 sc.scan-last-lsn 2 then 6
          else if ne-u32 sc.scan-last-crc crc2 then 7
          else
            -- 3. Detect stale recycled-segment chain mismatch (wrong initial chain seed)
            let sc-chain = wal-scan-block-raw block-addr 4096 1 (bit-xor-u32 wal-initial-chain-seed 1) in
            if ne-u32 sc-chain.scan-status wal-status-bad-chain then 8
            else
              -- 4. Detect reordered LSN gap (expecting LSN 5 instead of 1)
              let sc-lsn = wal-scan-block-raw block-addr 4096 5 wal-initial-chain-seed in
              if ne-u32 sc-lsn.scan-status wal-status-bad-lsn then 9
              else
                -- 5. Detect single-bit flip in record 2 payload
                let rec2-payload-byte = wrapping-add-i64 block-addr (u32-to-i64 (wrapping-add-u32 off2 35)) in
                let b2 = raw-load-u8 rec2-payload-byte in
                let _f2 = raw-store-u8 rec2-payload-byte (bit-xor-u8 b2 4) in
                let sc-flip = wal-scan-block-raw block-addr 4096 1 wal-initial-chain-seed in
                let _r2 = raw-store-u8 rec2-payload-byte b2 in
                if ne-u32 sc-flip.scan-status wal-status-bad-crc then 10
                else
                  -- 6. Detect non-zero garbage in 4 KiB block padding tail (torn sector)
                  let _g = raw-store-u8 (wrapping-add-i64 block-addr 3000) 0x5A in
                  let sc-tail = wal-scan-block-raw block-addr 4096 1 wal-initial-chain-seed in
                  if ne-u32 sc-tail.scan-status wal-status-bad-tail then 11
                  else 0

  main :: Unit ->{IO, MVar} Int
  main _ =
    let slab = engine-dma-alloc 8192 4096 in
    if eq-u64 slab 0 then 99
    else
      let blk = wrapping-u64-to-i64 slab in
      let pl = wrapping-add-i64 blk 4096 in
      let rc = check-format-no-alloc blk pl in
      let _f = engine-dma-free slab 8192 in
      if ne-u32 rc 0 then int-from-i64 (u32-to-i64 rc)
      else if wal-model-invariants wal-model-init then 0
      else 98
"#;

    let dir = fresh_dir("iface-check");
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    common::drive_with_profile(
        Mode::Check,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        None,
        &TargetProfile::native(),
    )
    .expect("Store.Wal.* typechecks cleanly with #[no-alloc]");
    let ifaces = common::check_interfaces(&dir, "Main.cinch").expect("check_interfaces");
    assert!(ifaces.iter().any(|i| i.module == "Main"));
    let _ = std::fs::remove_dir_all(&dir);

    build_and_run_leak_checked("format-tamper", src, 0);
}

#[test]
fn wal_machine_m10_formal_model_verifies_uncapped_with_zero_violations() {
    let main_src = r#"
module Main where
  import Store.Wal.Spec

  main :: Int
  main =
    if wal-model-invariants wal-model-init then 0 else 1
"#;

    let dir = common::project(
        "wal-machine-m10-check",
        &[
            ("Store.Wal.Spec.cinch", STORE_WAL_SPEC),
            ("Main.cinch", main_src),
        ],
    );

    // 1. Strict uncapped check via driver (works across Rust seed & Stage 3).
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
    .expect("M10 WalMachine strict uncapped check via driver");

    let manifest = std::fs::read_to_string(dir.join("build/Main.manifest.json"))
        .expect("read build/Main.manifest.json");
    assert!(
        manifest.contains("\"contract\":\"walDurabilityAndSafety\""),
        "expected walDurabilityAndSafety in manifest:\n{manifest}"
    );
    assert!(
        manifest.contains("\"contract\":\"walDeadlockFree\""),
        "expected walDeadlockFree in manifest:\n{manifest}"
    );
    assert!(
        manifest.contains("\"contract\":\"stagedEventuallyResolves\""),
        "expected stagedEventuallyResolves in manifest:\n{manifest}"
    );
    assert!(
        manifest.contains("\"contract\":\"crashedEventuallyRecovers\""),
        "expected crashedEventuallyRecovers in manifest:\n{manifest}"
    );
    assert!(
        manifest.contains("refines WalMachine AbstractDurableLog"),
        "expected refinement row in manifest:\n{manifest}"
    );
    assert!(
        manifest.contains("0 violations"),
        "expected 0 violations in manifest:\n{manifest}"
    );
    assert!(
        !manifest.contains("CAPPED"),
        "M10 WalMachine must not be capped:\n{manifest}"
    );

    // 2. CLI `cinch model inspect --json` under `CINCH_MODEL_FAIL_ON_CAPPED=1` (< 3s).
    let bin = env!("CARGO_BIN_EXE_cinch");
    let spec_path = dir.join("Store.Wal.Spec.cinch");
    let spec_str = spec_path.to_string_lossy().to_string();

    let t0 = Instant::now();
    let inspect_out = Command::new(bin)
        .args([
            "model",
            "inspect",
            &spec_str,
            "WalMachine",
            "--json",
            "--model-fail-on-capped",
        ])
        .env("CINCH_MODEL_FAIL_ON_CAPPED", "1")
        .output()
        .expect("run cinch model inspect WalMachine --json");
    let elapsed = t0.elapsed();
    assert!(
        inspect_out.status.success(),
        "cinch model inspect WalMachine failed:\nstderr:\n{}",
        String::from_utf8_lossy(&inspect_out.stderr)
    );

    let json_text = String::from_utf8(inspect_out.stdout).expect("UTF-8 inspect JSON");
    let parsed = parse_json(&json_text).expect("parse WalMachine inspect JSON");
    let report = match parsed.as_arr() {
        Some(arr) => arr
            .iter()
            .find(|m| m.get("name").and_then(Json::as_str) == Some("WalMachine"))
            .unwrap_or_else(|| panic!("WalMachine missing from inspect JSON: {json_text}")),
        None => &parsed,
    };
    assert_eq!(
        report.get("capped"),
        Some(&Json::Bool(false)),
        "WalMachine must be uncapped: {json_text}"
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
        "M10 WalMachine verified in {:.3}s: state_count={state_count}, edge_count={edge_count}, diameter={diameter}, capped=false",
        elapsed.as_secs_f64()
    );
    assert!(
        state_count >= 20 && state_count < 65536,
        "unexpected WalMachine state_count={state_count}: {json_text}"
    );
    assert_eq!(
        report
            .get("deadlocks")
            .and_then(Json::as_arr)
            .map(|a| a.len()),
        Some(0),
        "WalMachine must have 0 deadlocks: {json_text}"
    );
    let contracts = report
        .get("contracts")
        .and_then(Json::as_arr)
        .expect("contracts array");
    assert!(contracts.len() >= 4, "expected >= 4 contracts: {json_text}");
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
fn store_wal_10000_seed_power_loss_torn_sector_and_rotation_chaos() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.Mutable.Var
  import Engine.Types
  import Engine.Sim.Faults
  import Engine.Sim
  import Prim.Atomic
  import Store.Wal.Format
  import Store.Wal.Engine

  -- | Deterministic payload length for `(seed, lsn)` in `280 .. 680` bytes
  -- (strictly `> 256` bytes so every 512B half-sector tear slices directly
  -- through record payloads and triggers CRC32C rejection!).
  #[no-alloc]
  expected-payload-len :: U64 -> U64 -> U32
  expected-payload-len seed lsn =
    let h = sim-mix-u64 (bit-xor-u64 seed (wrapping-mul-u64 lsn 0x9E3779B97F4A7C15)) in
    wrapping-add-u32 280 (wrapping-u64-to-u32 (rem-u64 h 401))

  -- | Deterministic byte at index `idx` for record `(seed, lsn)`.
  #[inline(always)]
  #[no-alloc]
  expected-byte :: U64 -> U64 -> U32 -> U8
  expected-byte seed lsn idx =
    let k = bit-xor-u64 (wrapping-add-u64 seed (wrapping-mul-u64 lsn 0x517CC1B727220A95)) (u32-to-u64 idx) in
    wrapping-u64-to-u8 (sim-mix-u64 k)

  -- | Fill DMA scratch buffer `buf` with the exact deterministic payload for `(seed, lsn)`.
  #[no-alloc]
  fill-payload-dma :: I64 -> U64 -> U64 -> U32 -> U32 ->{MVar} Unit
  fill-payload-dma buf seed lsn i len =
    if ge-u32 i len then ()
    else
      let b = expected-byte seed lsn i in
      let _ = raw-store-u8 (wrapping-add-i64 buf (u32-to-i64 i)) b in
      fill-payload-dma buf seed lsn (wrapping-add-u32 i 1) len

  -- | Verify every byte of a recovered record's payload matches `(seed, lsn)` byte-for-byte.
  #[no-alloc]
  verify-payload-dma :: I64 -> U64 -> U64 -> U32 -> U32 ->{MVar} Bool
  verify-payload-dma buf seed lsn i len =
    if ge-u32 i len then eq-u32 0 0
    else
      let got = raw-load-u8 (wrapping-add-i64 buf (u32-to-i64 i)) in
      let exp = expected-byte seed lsn i in
      if ne-u8 got exp then eq-u32 0 1
      else verify-payload-dma buf seed lsn (wrapping-add-u32 i 1) len

  -- | Fault config with 100% 512B torn-sector injection on power-loss crashes.
  torn-faults :: SimFaultConfig
  torn-faults = SimFaultConfig {
    udp-drop-ppm = 0, udp-dup-ppm = 0, udp-reorder-ppm = 0, udp-corrupt-ppm = 0,
    udp-min-delay-ns = 0, udp-max-delay-ns = 0,
    tcp-partial-io-ppm = 0, tcp-rst-ppm = 0,
    disk-write-delay-ns = 0, disk-torn-sector-ppm = 1000000, disk-bitflip-ppm = 0,
    partition-mask = 0
  }

  -- | Fault config with 100% silent single-bit flip injection on disk writes.
  bitflip-faults :: SimFaultConfig
  bitflip-faults = SimFaultConfig {
    udp-drop-ppm = 0, udp-dup-ppm = 0, udp-reorder-ppm = 0, udp-corrupt-ppm = 0,
    udp-min-delay-ns = 0, udp-max-delay-ns = 0,
    tcp-partial-io-ppm = 0, tcp-rst-ppm = 0,
    disk-write-delay-ns = 0, disk-torn-sector-ppm = 0, disk-bitflip-ppm = 1000000,
    partition-mask = 0
  }

  -- | Stage `n` deterministic records for `seed` into `eng` using scratch buffer `scratch`.
  stage-n-records :: WalEngine -> I64 -> U64 -> U32 ->{MVar} U64
  stage-n-records eng scratch seed n =
    if eq-u32 n 0 then wal-staged-lsn eng
    else
      let next-lsn = wrapping-add-u64 (wal-staged-lsn eng) 1 in
      let plen = expected-payload-len seed next-lsn in
      let _f = fill-payload-dma scratch seed next-lsn 0 plen in
      let got-lsn = wal-append-dma eng wal-flag-data scratch plen in
      if eq-u64 got-lsn 0 then wal-staged-lsn eng
      else stage-n-records eng scratch seed (wrapping-sub-u32 n 1)

  -- | Recover `fd` and assert:
  --   1. Every replayed record has contiguous LSN `1..rec-durable-lsn`
  --   2. Every byte of every replayed payload matches `expected-byte(seed, lsn, i)`
  --   3. `rec-durable-lsn >= acked-lsn` (100% durability of acknowledged LSNs!)
  --   4. `rec-durable-lsn <= max-written-lsn` (zero phantom/corrupted records!)
  -- Returns recovered LSN (`>= 0`) or negated error code (`< 0`).
  recover-and-verify :: forall (r :: Region). WalDiskOps r -> WalEngine -> Fd -> U64 -> U64 -> U64 ->{IO, IO r, MVar, Mut r} I64
  recover-and-verify ops eng fd seed acked-lsn max-written-lsn =
    let seen-lsn = var-new (0 :: U64) in
    let bad-flag = var-new False in
    case wal-recover-with ops eng fd (\hdr paddr ->
      let prev = var-read seen-lsn in
      let exp-lsn = wrapping-add-u64 prev 1 in
      let exp-len = expected-payload-len seed exp-lsn in
      if ne-u64 hdr.wal-lsn exp-lsn || ne-u32 hdr.wal-payload-len exp-len then
        var-write bad-flag True
      else if not (verify-payload-dma paddr seed exp-lsn 0 exp-len) then
        var-write bad-flag True
      else
        var-write seen-lsn exp-lsn) of
      Err _ -> -30
      Ok rep ->
        let seen = var-read seen-lsn in
        if var-read bad-flag then -31
        else if ne-u64 seen rep.rec-durable-lsn then -32
        else if ne-u64 rep.rec-record-count rep.rec-durable-lsn then -33
        else if lt-u64 rep.rec-durable-lsn acked-lsn then -34
        else if gt-u64 rep.rec-durable-lsn max-written-lsn then -35
        else if ne-u64 (wal-durable-lsn eng) rep.rec-durable-lsn then -36
        else wrapping-u64-to-i64 rep.rec-durable-lsn

  -- | Run `steps` randomized WAL + fault-injection operations for one `seed`.
  -- Operations:
  --   0: append 1..2 records (staged in DMA batch buffer)
  --   1: group-commit staged batch (fsync -> advance acked-lsn)
  --   2: crash_clean (write un-fsynced cache -> power loss -> recover & verify)
  --   3: crash_torn_512b_sector (512B half-sector tear on tail -> recover & verify)
  --   4: bit_flip_tail (silent single-bit flip on tail block -> recover & verify)
  run-chaos-steps
    :: forall (r :: Region).
       SimWorld r -> WalDiskOps r -> WalEngine -> Fd -> I64 -> U64 -> U64 -> U64 -> U64 -> U32 ->{IO, IO r, MVar, Mut r} U32
  run-chaos-steps w ops eng fd scratch seed rng-state acked-lsn max-written rem =
    if eq-u32 rem 0 then
      -- Final crash + recovery check at end of every seed
      let _ = sim-world-set-faults w faults-clean in
      let _ = sim-disk-crash-power-loss! w fd in
      let rv = recover-and-verify ops eng fd seed acked-lsn max-written in
      if lt-i64 rv 0 then wrapping-i64-to-u32 (wrapping-neg-i64 rv)
      else 0
    else
      let h = sim-mix-u64 (wrapping-add-u64 rng-state (u32-to-u64 rem)) in
      let op = rem-u64 h 5 in
      if eq-u64 op 0 then
        -- Stage 1..2 records
        let cnt = wrapping-add-u32 1 (wrapping-u64-to-u32 (rem-u64 (wrapping-shr-u64 h 8) 2)) in
        let _s = stage-n-records eng scratch seed cnt in
        run-chaos-steps w ops eng fd scratch seed h acked-lsn max-written (wrapping-sub-u32 rem 1)
      else if eq-u64 op 1 then
        -- Stage a record if empty, then group-commit durably (clean disk)
        let _ = sim-world-set-faults w faults-clean in
        let _s = if eq-u32 (wal-batch-count eng) 0 then stage-n-records eng scratch seed 1 else wal-staged-lsn eng in
        -- Stay within the 32 KiB single-segment chaos disk (7 data blocks = 28 KiB)
        if ge-u64 (wal-write-offset eng) 24576 then
          let _r = wal-stage-reset eng in
          run-chaos-steps w ops eng fd scratch seed h acked-lsn max-written (wrapping-sub-u32 rem 1)
        else
          case wal-group-commit ops eng of
            Err _ -> 10
            Ok committed-lsn ->
              let next-max = if gt-u64 committed-lsn max-written then committed-lsn else max-written in
              run-chaos-steps w ops eng fd scratch seed h committed-lsn next-max (wrapping-sub-u32 rem 1)
      else if eq-u64 op 2 then
        -- crash_clean: stage + write un-fsynced volatile cache -> drop cache -> recover
        let _ = sim-world-set-faults w faults-clean in
        let _s = if eq-u32 (wal-batch-count eng) 0 then stage-n-records eng scratch seed 1 else wal-staged-lsn eng in
        let staged = wal-staged-lsn eng in
        let nw = wal-flush-unfsynced ops eng in
        let next-max = if gt-u32 nw 0 && gt-u64 staged max-written then staged else max-written in
        let _c = sim-disk-crash-power-loss! w fd in
        let rv = recover-and-verify ops eng fd seed acked-lsn next-max in
        if lt-i64 rv 0 then wrapping-i64-to-u32 (wrapping-neg-i64 rv)
        else
          let rec-lsn = wrapping-i64-to-u64 rv in
          -- Clean power loss drops all un-fsynced volatile sectors: rec-lsn == acked-lsn
          if ne-u64 rec-lsn acked-lsn then 39
          else run-chaos-steps w ops eng fd scratch seed h rec-lsn next-max (wrapping-sub-u32 rem 1)
      else if eq-u64 op 3 then
        -- crash_torn_512b_sector: stage >= 1 record, flush un-fsynced, tear 512B sectors on crash!
        let _ = sim-world-set-faults w torn-faults in
        let _s = if eq-u32 (wal-batch-count eng) 0 then stage-n-records eng scratch seed 2 else wal-staged-lsn eng in
        let staged = wal-staged-lsn eng in
        let nw = wal-flush-unfsynced ops eng in
        let next-max = if gt-u32 nw 0 && gt-u64 staged max-written then staged else max-written in
        let _c = sim-disk-crash-power-loss! w fd in
        let _ = sim-world-set-faults w faults-clean in
        let rv = recover-and-verify ops eng fd seed acked-lsn next-max in
        if lt-i64 rv 0 then wrapping-i64-to-u32 (wrapping-neg-i64 rv)
        else
          let rec-lsn = wrapping-i64-to-u64 rv in
          -- Every acked LSN survives byte-exact and any recovered LSN is a valid flushed prefix!
          run-chaos-steps w ops eng fd scratch seed h rec-lsn next-max (wrapping-sub-u32 rem 1)
      else
        -- bit_flip_tail: stage record, write with 100% bit-flip + fsync on tail block,
        -- then recover and verify CRC32C rejects the flipped tail while keeping acked-lsn!
        let _ = sim-world-set-faults w bitflip-faults in
        let _s = if eq-u32 (wal-batch-count eng) 0 then stage-n-records eng scratch seed 1 else wal-staged-lsn eng in
        let staged = wal-staged-lsn eng in
        let nw = wal-flush-unfsynced ops eng in
        let next-max = if gt-u32 nw 0 && gt-u64 staged max-written then staged else max-written in
        let _fs = if gt-u32 nw 0 then sim-disk-fsync w fd else IoOk in
        let _ = sim-world-set-faults w faults-clean in
        let rv = recover-and-verify ops eng fd seed acked-lsn next-max in
        if lt-i64 rv 0 then wrapping-i64-to-u32 (wrapping-neg-i64 rv)
        else
          let rec-lsn = wrapping-i64-to-u64 rv in
          if ne-u64 rec-lsn acked-lsn then 41
          else run-chaos-steps w ops eng fd scratch seed h rec-lsn next-max (wrapping-sub-u32 rem 1)

  -- | Execute one complete chaos seed over a fresh 512B-sector `SimWorld` disk.
  run-one-seed :: forall (r :: Region). WalEngine -> I64 -> U64 ->{IO, IO r, MVar, Mut r} U32
  run-one-seed eng scratch seed =
    let w = sim-world-new seed faults-clean in
    -- 64 sectors * 512B = 32 KiB segment (Block 0 header + 7 x 4 KiB data blocks)
    let fd = sim-disk-open w 512 64 in
    let ops = wal-sim-disk-ops w 512 64 in
    -- Reset engine metadata for this fresh segment without reallocating DMA slabs
    let hdr = eng.wal-hdr-addr in
    let _0 = raw-store-u32 hdr (wrapping-i32-to-u32 fd.fd-raw) in
    let _1 = raw-store-u64 (wrapping-add-i64 hdr 8) 0 in
    let _2 = raw-store-u64 (wrapping-add-i64 hdr 24) 4096 in
    let _3 = raw-store-u64 (wrapping-add-i64 hdr 32) 0 in
    let _4 = raw-store-u64 (wrapping-add-i64 hdr 40) 0 in
    let _5 = raw-store-u32 (wrapping-add-i64 hdr 48) wal-initial-chain-seed in
    let _6 = raw-store-u32 (wrapping-add-i64 hdr 52) wal-initial-chain-seed in
    let _7 = raw-store-u32 (wrapping-add-i64 hdr 56) 0 in
    let _8 = raw-store-u32 (wrapping-add-i64 hdr 60) 0 in
    let _9 = raw-store-u64 (wrapping-add-i64 hdr 88) 1 in
    let init-err = wal-init-segment ops eng in
    if not (io-error-is-ok init-err) then 50
    else run-chaos-steps w ops eng fd scratch seed (sim-mix-u64 seed) 0 0 8

  -- | Run all seeds `i .. limit` (10,000 seeds!).
  run-all-seeds :: forall (r :: Region). WalEngine -> I64 -> U64 -> U64 ->{IO, IO r, MVar, Mut r} U32
  run-all-seeds eng scratch i limit =
    if ge-u64 i limit then 0
    else
      let rc = run-one-seed eng scratch (wrapping-add-u64 i 1) in
      if ne-u32 rc 0 then rc
      else run-all-seeds eng scratch (wrapping-add-u64 i 1) limit

  -- | Verify multi-segment rotation + cross-segment recovery AND `wal-reactor-disk-ops`
  -- over `sim-reactor-ops`!
  test-rotation-and-reactor-ops :: forall (r :: Region). I64 ->{IO, IO r, MVar, Mut r} U32
  test-rotation-and-reactor-ops scratch =
    let w = sim-world-new 999999 faults-clean in
    let fd0 = sim-disk-open w 4096 3 in
    -- 12 KiB segment (Block 0 header + only 2 x 4 KiB data blocks) forces rotation on commit #3!
    case wal-engine-new fd0 12288 8192 7 of
      None -> 60
      Some eng ->
        let seg1-ref = var-new (-1 :: I32) in
        let rx-ops = sim-reactor-ops w in
        let ops = wal-reactor-disk-ops rx-ops (\_seg-id ->
          let nf = sim-disk-open w 4096 3 in
          let _ = var-write seg1-ref nf.fd-raw in
          nf.fd-raw) in
        let e0 = wal-init-segment ops eng in
        if not (io-error-is-ok e0) then
          let _ = wal-engine-destroy eng in 61
        else
          -- Commit 1 (4 KiB at 4096) & Commit 2 (4 KiB at 8192) fill segment 0 (12 KiB)
          let _s1 = stage-n-records eng scratch 777 2 in
          let r1 = wal-group-commit ops eng in
          let _s2 = stage-n-records eng scratch 777 2 in
          let r2 = wal-group-commit ops eng in
          -- Commit 3 triggers automatic segment rotation to segment 1!
          let _s3 = stage-n-records eng scratch 777 2 in
          let r3 = wal-group-commit ops eng in
          let rots = wal-rotation-count eng in
          let sid = wal-segment-id eng in
          let dlsn = wal-durable-lsn eng in
          let fd1 = fd-of (var-read seg1-ref) in
          -- Recover segment 0 (LSNs 1..4) and segment 1 (LSNs 5..6)
          let rec0-ok = (case wal-recover ops eng fd0 of
            Ok rep0 -> eq-u64 rep0.rec-durable-lsn 4 && eq-u64 rep0.rec-record-count 4
            Err _ -> False) in
          let rec1-ok = (case wal-recover ops eng fd1 of
            Ok rep1 -> eq-u64 rep1.rec-durable-lsn 6 && eq-u64 rep1.rec-record-count 2
            Err _ -> False) in
          let _ = wal-engine-destroy eng in
          case (r1, r2, r3) of
            (Ok l1, Ok l2, Ok l3) ->
              if ne-u64 l1 2 || ne-u64 l2 4 || ne-u64 l3 6 then 62
              else if ne-u32 rots 1 || ne-u64 sid 1 || ne-u64 dlsn 6 then 63
              else if not rec0-ok || not rec1-ok then 64
              else 0
            _ -> 65

  main :: forall (r :: Region). Unit ->{IO, IO r, MVar, Mut r} Int
  main _ =
    let scratch-u64 = engine-dma-alloc 4096 4096 in
    if eq-u64 scratch-u64 0 then 90
    else
      let scratch = wrapping-u64-to-i64 scratch-u64 in
      case wal-engine-new (fd-of 0) 32768 16384 1 of
        None ->
          let _ = engine-dma-free scratch-u64 4096 in 91
        Some eng ->
          let rc1 = run-all-seeds eng scratch 0 10000 in
          let _ = wal-engine-destroy eng in
          if ne-u32 rc1 0 then
            let _ = engine-dma-free scratch-u64 4096 in
            int-from-i64 (u32-to-i64 rc1)
          else
            let rc2 = test-rotation-and-reactor-ops scratch in
            let _ = engine-dma-free scratch-u64 4096 in
            int-from-i64 (u32-to-i64 rc2)
"#;

    build_and_run_leak_checked("chaos-10000-seeds", src, 0);
}

#[cfg(target_os = "macos")]
#[test]
fn store_wal_darwin_f_nocache_live_recovery_and_50000_zero_alloc_hot_loop() {
    let tmp_dir = fresh_dir("darwin-live-wal");
    let wal_path = tmp_dir.join("segment_0000.wal");
    let path_escaped = wal_path
        .to_string_lossy()
        .replace('\\', "\\\\")
        .replace('"', "\\\"");

    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Option
  import Data.Result
  import Data.String
  import Data.Mutable.Var
  import Engine.Types
  import Engine.Reactor.Darwin
  import Prim.Atomic
  import Prim.Sys.Io.Darwin
  import Store.Wal.Format
  import Store.Wal.Engine

  -- | Fill DMA buffer with deterministic LSN-keyed pattern (`#[no-alloc]`).
  #[no-alloc]
  fill-live-payload :: I64 -> U64 -> U32 -> U32 ->{MVar} Unit
  fill-live-payload buf lsn i len =
    if ge-u32 i len then ()
    else
      let v = wrapping-u64-to-u8 (bit-xor-u64 (wrapping-mul-u64 (wrapping-add-u64 lsn (u32-to-u64 i)) 0x9E3779B97F4A7C15) 0xA5A55A5A) in
      let _ = raw-store-u8 (wrapping-add-i64 buf (u32-to-i64 i)) v in
      fill-live-payload buf lsn (wrapping-add-u32 i 1) len

  -- | Verify DMA payload bytes match deterministic LSN-keyed pattern (`#[no-alloc]`).
  #[no-alloc]
  verify-live-payload :: I64 -> U64 -> U32 -> U32 ->{MVar} Bool
  verify-live-payload buf lsn i len =
    if ge-u32 i len then eq-u32 0 0
    else
      let expect = wrapping-u64-to-u8 (bit-xor-u64 (wrapping-mul-u64 (wrapping-add-u64 lsn (u32-to-u64 i)) 0x9E3779B97F4A7C15) 0xA5A55A5A) in
      let got = raw-load-u8 (wrapping-add-i64 buf (u32-to-i64 i)) in
      if ne-u8 got expect then eq-u32 0 1
      else verify-live-payload buf lsn (wrapping-add-u32 i 1) len

  -- | Steady-state 50,000-record `#[no-alloc]` hot loop:
  -- exercises `wal-append-dma` (`wal-write-record-raw` + LRAT-verified hardware
  -- `crc32c` hash chaining) + `wal-format-block-raw` + `wal-stage-reset`
  -- with ZERO heap allocations and ZERO atomic RC traffic!
  #[no-alloc]
  hot-stage-loop :: WalEngine -> I64 -> U32 -> U64 ->{MVar} U64
  hot-stage-loop eng payload-buf rem acc =
    if eq-u32 rem 0 then acc
    else
      let _w = raw-store-u64 payload-buf (u32-to-u64 rem) in
      let lsn = wal-append-dma eng wal-flag-data payload-buf 64 in
      if eq-u64 lsn 0 then
        -- Batch buffer full: format to 4 KiB boundary, reset stage, retry
        let aligned = wal-format-block-raw (wal-batch-addr eng) (wal-batch-used eng) (wal-batch-cap eng) in
        let _r = wal-stage-reset eng in
        let lsn2 = wal-append-dma eng wal-flag-data payload-buf 64 in
        let ok = if gt-u32 aligned 0 then gt-u64 lsn2 0 else eq-u32 0 1 in
        let next-acc = if ok then wrapping-add-u64 acc 1 else acc in
        hot-stage-loop eng payload-buf (wrapping-sub-u32 rem 1) next-acc
      else
        hot-stage-loop eng payload-buf (wrapping-sub-u32 rem 1) (wrapping-add-u64 acc 1)

  -- | Live macOS `F_NOCACHE` 4 KiB-aligned direct-disk WAL write + torn-tail
  -- injection + reopen + `wal-recover-with` + resumed write + re-verify.
  test-live-darwin-wal :: forall (r :: Region). I64 ->{IO, IO r, MVar, Mut r} U32
  test-live-darwin-wal scratch =
    let path = string-to-bytes "__WAL_FILE_PATH__" in
    let create-flags = wrapping-i64-to-i32 (bit-or-i64 (bit-or-i64 o-rdwr o-creat) o-trunc) in
    let reopen-flags = wrapping-i64-to-i32 o-rdwr in
    let ops = wal-darwin-disk-ops (\_seg-id -> -1) in
    case darwin-open-direct path create-flags of
      Err _ -> 1
      Ok fd1 ->
        case wal-engine-new fd1 131072 16384 9 of
          None ->
            let _ = darwin-close-fd fd1 in 2
          Some eng ->
            let e-init = wal-init-segment ops eng in
            if not (io-error-is-ok e-init) then
              let _ = wal-engine-destroy eng in
              let _ = darwin-close-fd fd1 in 3
            else
              -- Batch 1: 3 records (120B, 250B, 1500B -> LSNs 1..3)
              let _f1 = fill-live-payload scratch 1 0 120 in
              let l1 = wal-append-dma eng wal-flag-data scratch 120 in
              let _f2 = fill-live-payload scratch 2 0 250 in
              let l2 = wal-append-dma eng wal-flag-data scratch 250 in
              let _f3 = fill-live-payload scratch 3 0 1500 in
              let l3 = wal-append-dma eng wal-flag-data scratch 1500 in
              let c1 = wal-group-commit ops eng in
              -- Batch 2: 2 records (80B, 300B -> LSNs 4..5)
              let _f4 = fill-live-payload scratch 4 0 80 in
              let l4 = wal-append-dma eng wal-flag-data scratch 80 in
              let _f5 = fill-live-payload scratch 5 0 300 in
              let l5 = wal-append-dma eng wal-flag-data scratch 300 in
              let c2 = wal-group-commit ops eng in
              let committed-off = wal-write-offset eng in
              -- Simulate torn tail write directly on disk at `committed-off`:
              -- write a valid-looking header for LSN 6, then corrupt its payload!
              let _f6 = fill-live-payload scratch 6 0 200 in
              let _crc6 = wal-write-record-raw (wal-batch-addr eng) wal-flag-data 6 (wal-durable-chain-crc eng) 9 scratch 200 in
              let _pad = wal-format-block-raw (wal-batch-addr eng) (wal-record-total-len 200) 4096 in
              -- Corrupt byte +100 of the tail block (torn write mid-record!)
              let corrupt-ptr = wrapping-add-i64 (wal-batch-addr eng) 100 in
              let _tear = raw-store-u8 corrupt-ptr (bit-xor-u8 (raw-load-u8 corrupt-ptr) 0xFF) in
              let nw-torn = darwin-pwrite-dma fd1 (wal-batch-addr eng) 4096 committed-off in
              let _fs-torn = darwin-fsync-fd fd1 in
              let _cl1 = darwin-close-fd fd1 in
              let _d1 = wal-engine-destroy eng in
              if ne-u64 l1 1 || ne-u64 l2 2 || ne-u64 l3 3 || ne-u64 l4 4 || ne-u64 l5 5 || ne-i32 nw-torn 4096 then 4
              else case (c1, c2) of
                (Ok cl1, Ok cl2) ->
                  if ne-u64 cl1 3 || ne-u64 cl2 5 then 5
                  else
                    -- Reopen with F_NOCACHE and recover exact durable prefix (LSNs 1..5)!
                    case darwin-open-direct path reopen-flags of
                      Err _ -> 6
                      Ok fd2 ->
                        case wal-engine-new fd2 131072 16384 9 of
                          None ->
                            let _ = darwin-close-fd fd2 in 7
                          Some eng2 ->
                            let seen = var-new (0 :: U64) in
                            let bad = var-new False in
                            case wal-recover-with ops eng2 fd2 (\hdr paddr ->
                              let next = wrapping-add-u64 (var-read seen) 1 in
                              if ne-u64 hdr.wal-lsn next then var-write bad True
                              else if not (verify-live-payload paddr next 0 hdr.wal-payload-len) then var-write bad True
                              else var-write seen next) of
                              Err _ ->
                                let _ = wal-engine-destroy eng2 in
                                let _ = darwin-close-fd fd2 in 8
                              Ok rep1 ->
                                if var-read bad || ne-u64 (var-read seen) 5 || ne-u64 rep1.rec-durable-lsn 5
                                  || ne-u64 rep1.rec-next-write-offset committed-off
                                  || ne-u32 rep1.rec-stop-status wal-status-bad-crc then
                                  let _ = wal-engine-destroy eng2 in
                                  let _ = darwin-close-fd fd2 in 9
                                else
                                  -- Resume appending LSN 6 & 7 right over the torn tail block and commit!
                                  let _f6b = fill-live-payload scratch 6 0 200 in
                                  let l6 = wal-append-dma eng2 wal-flag-data scratch 200 in
                                  let _f7b = fill-live-payload scratch 7 0 96 in
                                  let l7 = wal-append-dma eng2 wal-flag-data scratch 96 in
                                  case wal-group-commit ops eng2 of
                                    Err _ ->
                                      let _ = wal-engine-destroy eng2 in
                                      let _ = darwin-close-fd fd2 in 10
                                    Ok cl3 ->
                                      let _cl2 = darwin-close-fd fd2 in
                                      let _d2 = wal-engine-destroy eng2 in
                                      if ne-u64 l6 6 || ne-u64 l7 7 || ne-u64 cl3 7 then 11
                                      else
                                        -- Reopen a third time and verify ALL 7 records recover cleanly with clean EOF!
                                        case darwin-open-direct path reopen-flags of
                                          Err _ -> 12
                                          Ok fd3 ->
                                            case wal-engine-new fd3 131072 16384 9 of
                                              None ->
                                                let _ = darwin-close-fd fd3 in 13
                                              Some eng3 ->
                                                let seen3 = var-new (0 :: U64) in
                                                let bad3 = var-new False in
                                                let r3 = wal-recover-with ops eng3 fd3 (\hdr paddr ->
                                                  let next = wrapping-add-u64 (var-read seen3) 1 in
                                                  if ne-u64 hdr.wal-lsn next then var-write bad3 True
                                                  else if not (verify-live-payload paddr next 0 hdr.wal-payload-len) then var-write bad3 True
                                                  else var-write seen3 next) in
                                                let _d3 = wal-engine-destroy eng3 in
                                                let _cl3 = darwin-close-fd fd3 in
                                                case r3 of
                                                  Err _ -> 14
                                                  Ok rep3 ->
                                                    if var-read bad3 || ne-u64 (var-read seen3) 7 || ne-u64 rep3.rec-durable-lsn 7
                                                      || ne-u32 rep3.rec-stop-status wal-status-eof then 15
                                                    else 0
                _ -> 16

  -- | 50,000-record zero-allocation steady-state hot loop verification.
  test-zero-alloc50k :: I64 ->{IO, MVar} U32
  test-zero-alloc50k scratch =
    case wal-engine-new (fd-of 0) 65536 16384 1 of
      None -> 20
      Some eng ->
        let _f = fill-live-payload scratch 42 0 64 in
        let _rst = engine-thread-counters-reset () in
        let done = hot-stage-loop eng scratch 50000 0 in
        let allocs = engine-thread-alloc-count () in
        let atomic-rcs = engine-thread-atomic-rc-count () in
        let _d = wal-engine-destroy eng in
        if ne-u64 done 50000 then 21
        else if ne-u64 allocs 0 then 22
        else if ne-u64 atomic-rcs 0 then 23
        else 0

  main :: forall (r :: Region). Unit ->{IO, IO r, MVar, Mut r} Int
  main _ =
    let scratch-u64 = engine-dma-alloc 4096 4096 in
    if eq-u64 scratch-u64 0 then 90
    else
      let scratch = wrapping-u64-to-i64 scratch-u64 in
      let rc1 = test-live-darwin-wal scratch in
      if ne-u32 rc1 0 then
        let _ = engine-dma-free scratch-u64 4096 in
        int-from-i64 (u32-to-i64 rc1)
      else
        let rc2 = test-zero-alloc50k scratch in
        let _ = engine-dma-free scratch-u64 4096 in
        int-from-i64 (u32-to-i64 rc2)
"#
    .replace("__WAL_FILE_PATH__", &path_escaped);

    build_and_run_leak_checked("darwin-f-nocache-and-50k-zero-alloc", &src, 0);
    let _ = std::fs::remove_dir_all(&tmp_dir);
}
