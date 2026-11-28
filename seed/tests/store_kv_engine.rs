//! Integration tests for Wave 5 `Store.Kv` (`Store.Kv.Engine`,
//! `Store.Kv.Server`, and Formal Model M11 `KvMeshMachine` in `Store.Kv.Spec`):
//!
//!   1. Stdlib manifest registration (`seed/prelude/.cinchpkg`), CinchDoc
//!      coverage, and compile-time `#[no-alloc]` hot-path verification.
//!   2. Formal Model **M11 `KvMeshMachine`** & **`AbstractAtomicKv`** via both
//!      `common::drive_with_profile_opts` (`model_fail_on_capped = true`) and
//!      `cinch model inspect --json` under `CINCH_MODEL_FAIL_ON_CAPPED=1`
//!      (`capped: false`, `0 violations`, all contracts + cross-core
//!      linearizability refinement passing in `< 3s`).
//!   3. Full `KvShard` SwissTable + DMA arena + `TimerWheel` + `Store.Wal` AOF
//!      tests: insert/update/delete/probe-chain/tombstone reuse, `INCR`/`DECR`
//!      (incl. overflow & non-int rejection), active (`TimerWheel`) + passive
//!      TTL expiration, and `Store.Wal` AOF power-loss crash recovery.
//!   4. Multi-core pinned OS worker mesh linearizability (2 pinned cores via
//!      `Engine.Runtime` + `CoreMesh` SPSC concurrently incrementing keys on
//!      both shards) + live macOS loopback pipelined RESP2/RESP3 client
//!      (`-P 16` batches) verifying exact responses and `leaked=0`.
//!   5. Steady-state 100,000 `GET`/`SET` + RESP parse/encode hot loop asserting
//!      `engine-thread-alloc-count () == 0`, `engine-thread-atomic-rc-count () == 0`,
//!      and `CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1` (`leaked=0`).
//!
//! Routed through `common::drive_with_profile` / `common::drive_with_profile_opts` /
//! `common::check_interfaces` so the exact same suite runs against both the
//! Rust seed and the Stage 3 self-hosted compiler.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{DriverOptions, Mode, Package};
use cinch::lsp::{parse_json, Json};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

const STORE_KV_ENGINE: &str = include_str!("../prelude/Store/Kv/Engine.cinch");
const STORE_KV_SERVER: &str = include_str!("../prelude/Store/Kv/Server.cinch");
const STORE_KV_SPEC: &str = include_str!("../prelude/Store/Kv/Spec.cinch");
const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn fresh_dir(tag: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "cinch-store-kv-engine-{tag}-{}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

#[cfg(target_os = "macos")]
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
) -> (String, String) {
    let dir = fresh_dir(tag);
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let exe = dir.join(format!("kv-{tag}"));
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

    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "256M")
        .output()
        .expect("run built binary");
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let _ = std::fs::remove_dir_all(&dir);

    assert_eq!(
        out.status.code(),
        Some(expected_code),
        "{tag}: expected exit {expected_code}, got status={:?}\nstdout:\n{stdout}\nstderr:\n{stderr}",
        out.status
    );
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "{tag}: expected clean leak report (`leaked=0`), got stderr:\n{stderr}"
    );
    (stdout, stderr)
}

// =========================================================================
// 1. Manifest registration, CinchDoc coverage, and #[no-alloc] safety check
// =========================================================================

#[test]
fn store_kv_engine_manifest_docs_and_no_alloc_check() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, path) in [
        ("Store.Kv.Engine", "Store/Kv/Engine.cinch"),
        ("Store.Kv.Server", "Store/Kv/Server.cinch"),
        ("Store.Kv.Spec", "Store/Kv/Spec.cinch"),
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
    assert!(pos("Engine.Timer") < pos("Store.Kv.Engine"));
    assert!(pos("Store.Wal.Engine") < pos("Store.Kv.Engine"));
    assert!(pos("Store.Kv.Resp") < pos("Store.Kv.Server"));
    assert!(pos("Store.Kv.Engine") < pos("Store.Kv.Server"));
    assert!(pos("Engine.Ring.Spsc") < pos("Store.Kv.Server"));

    common::assert_documented("Store.Kv.Engine", STORE_KV_ENGINE);
    common::assert_documented("Store.Kv.Server", STORE_KV_SERVER);
    common::assert_documented("Store.Kv.Spec", STORE_KV_SPEC);

    // Compile-time #[no-alloc] audit across Store.Kv.Engine & Store.Kv.Server hot paths
    let src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Data.Option
  import Engine.Ring.Spsc
  import Engine.Types
  import Prim.Atomic
  import Store.Kv.Engine
  import Store.Kv.Resp
  import Store.Kv.Server
  import Store.Kv.Spec
  import Store.Wal.Engine

  #[no-alloc]
  audit-kv-no-alloc :: KvShard -> KvXCorePool -> WalEngine -> I64 -> I64 -> I64 ->{MVar} U32
  audit-kv-no-alloc sh xpool wal kbuf vbuf scratch =
    let _h = kv-hash-dma kbuf 4 in
    let owner = kv-owner-core kbuf 4 4 in
    let s1 = kv-shard-set-raw sh kbuf 4 vbuf 4 in
    let g1 = kv-shard-get sh kbuf 4 in
    let e1 = kv-shard-exists sh kbuf 4 in
    let _t1 = kv-shard-ttl sh kbuf 4 in
    let _pt1 = kv-shard-pttl sh kbuf 4 in
    let _w1 = kv-wal-log-set wal kbuf 4 vbuf 4 0 scratch 256 in
    let _w2 = kv-wal-log-del wal kbuf 4 scratch 256 in
    let _w3 = kv-wal-log-expire wal kbuf 4 5000 scratch 256 in
    let d1 = kv-shard-del-raw sh kbuf 4 in
    let i1 = kv-shard-incr sh kbuf 4 in
    let i2 = kv-shard-decr sh kbuf 4 in
    let sl = kv-xcore-slot-alloc xpool 0 in
    let _env = kv-mesh-pack-req 0 owner 1 sl cmd-get 0 4 0 in
    let _fr = kv-xcore-slot-free xpool 0 sl in
    if ne-u8 s1 kv-ok then 1
    else if eq-u8 g1.kv-found 0 then 2
    else if ne-u32 g1.kv-val-len 4 then 2
    else if ne-u8 e1 1 then 3
    else if ne-u8 d1 1 then 3
    else if ne-u8 i1.kv-int-status kv-ok then 4
    else if ne-i64 i1.kv-int-val 1 then 4
    else if ne-u8 i2.kv-int-status kv-ok then 5
    else if ne-i64 i2.kv-int-val 0 then 5
    else 0

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 4096 4096 in
    if eq-u64 dma 0 then 90
    else
      let base = wrapping-u64-to-i64 dma in
      let kbuf = base in
      let vbuf = wrapping-add-i64 base 64 in
      let scratch = wrapping-add-i64 base 256 in
      let _k = raw-store-u32 kbuf 0x3179656B in -- "key1"
      let _v = raw-store-u32 vbuf 0x316C6176 in -- "val1"
      case kv-shard-new (core-id 0) 64 65536 64 1000 of
        None ->
          let _f = engine-dma-free dma 4096 in 91
        Some sh ->
          case kv-xcore-pool-new 2 8 of
            None ->
              let _ds = kv-shard-destroy sh in
              let _f = engine-dma-free dma 4096 in 92
            Some xpool ->
              case wal-engine-new (fd-of 0) 16384 8192 1 of
                None ->
                  let _dx = kv-xcore-pool-destroy xpool in
                  let _ds = kv-shard-destroy sh in
                  let _f = engine-dma-free dma 4096 in 93
                Some wal ->
                  let rc = audit-kv-no-alloc sh xpool wal kbuf vbuf scratch in
                  let _dw = wal-engine-destroy wal in
                  let _dx = kv-xcore-pool-destroy xpool in
                  let _ds = kv-shard-destroy sh in
                  let _f = engine-dma-free dma 4096 in
                  if ne-u32 rc 0 then int-from-i64 (u32-to-i64 rc)
                  else if kv-mesh-invariants kv-mesh-init then 0
                  else 94
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
    .expect("Store.Kv.* typechecks cleanly with #[no-alloc]");
    let ifaces = common::check_interfaces(&dir, "Main.cinch").expect("check_interfaces");
    assert!(ifaces.iter().any(|i| i.module == "Main"));
    let _ = std::fs::remove_dir_all(&dir);

    build_and_run_leak_checked("manifest-no-alloc", src, &[], 0);
}

// =========================================================================
// 2. Formal Model M11 `KvMeshMachine` & `AbstractAtomicKv` linearizability
// =========================================================================

#[test]
fn kv_mesh_machine_m11_formal_model_verifies_uncapped_with_zero_violations() {
    let main_src = r#"
module Main where
  import Store.Kv.Spec

  main :: Int
  main =
    if kv-mesh-invariants kv-mesh-init then 0 else 1
"#;

    let dir = common::project(
        "kv-mesh-machine-m11-check",
        &[
            ("Store.Kv.Spec.cinch", STORE_KV_SPEC),
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
    .expect("M11 KvMeshMachine strict uncapped check via driver");

    let manifest = std::fs::read_to_string(dir.join("build/Main.manifest.json"))
        .expect("read build/Main.manifest.json");
    for expected in [
        "\"contract\":\"kvMeshSafety\"",
        "\"contract\":\"kvMeshDeadlockFree\"",
        "\"contract\":\"localReqCompletes\"",
        "\"contract\":\"remoteReqCompletes\"",
        "refines KvMeshMachine AbstractAtomicKv",
        "0 violations",
    ] {
        assert!(
            manifest.contains(expected),
            "expected `{expected}` in manifest:\n{manifest}"
        );
    }
    assert!(
        !manifest.contains("CAPPED"),
        "M11 KvMeshMachine must not be capped:\n{manifest}"
    );

    // 2. CLI `cinch model inspect --json` under `CINCH_MODEL_FAIL_ON_CAPPED=1` (< 3s).
    let bin = env!("CARGO_BIN_EXE_cinch");
    let spec_path = dir.join("Store.Kv.Spec.cinch");
    let spec_str = spec_path.to_string_lossy().to_string();

    let t0 = Instant::now();
    let inspect_out = Command::new(bin)
        .args([
            "model",
            "inspect",
            &spec_str,
            "KvMeshMachine",
            "--json",
            "--model-fail-on-capped",
        ])
        .env("CINCH_MODEL_FAIL_ON_CAPPED", "1")
        .output()
        .expect("run cinch model inspect KvMeshMachine --json");
    let elapsed = t0.elapsed();
    assert!(
        inspect_out.status.success(),
        "cinch model inspect KvMeshMachine failed:\nstderr:\n{}",
        String::from_utf8_lossy(&inspect_out.stderr)
    );

    let json_text = String::from_utf8(inspect_out.stdout).expect("UTF-8 inspect JSON");
    let parsed = parse_json(&json_text).expect("parse KvMeshMachine inspect JSON");
    let report = match parsed.as_arr() {
        Some(arr) => arr
            .iter()
            .find(|m| m.get("name").and_then(Json::as_str) == Some("KvMeshMachine"))
            .unwrap_or_else(|| panic!("KvMeshMachine missing from inspect JSON: {json_text}")),
        None => &parsed,
    };
    assert_eq!(
        report.get("capped"),
        Some(&Json::Bool(false)),
        "KvMeshMachine must be uncapped: {json_text}"
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
        "M11 KvMeshMachine verified in {:.3}s: state_count={state_count}, edge_count={edge_count}, diameter={diameter}, capped=false",
        elapsed.as_secs_f64()
    );
    assert!(
        state_count >= 100 && state_count < 65536,
        "unexpected KvMeshMachine state_count={state_count}: {json_text}"
    );
    assert_eq!(
        report
            .get("deadlocks")
            .and_then(Json::as_arr)
            .map(|a| a.len()),
        Some(0),
        "KvMeshMachine must have 0 deadlocks: {json_text}"
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

// =========================================================================
// 3. Full KvShard tests: SwissTable probe-chain/tombstone reuse, INCR/DECR,
//    EXPIRE/TTL active (TimerWheel) + passive expiry, and Store.Wal AOF recovery
// =========================================================================

#[test]
fn kv_shard_swisstable_tombstone_incr_ttl_and_wal_crash_recovery() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Engine.Sim
  import Engine.Sim.Faults
  import Engine.Types
  import Prim.Atomic
  import Store.Kv.Engine
  import Store.Wal.Engine

  copy-str-dma :: String -> I64 ->{MVar} U32
  copy-str-dma s dst =
    let b = string-to-bytes s in
    let n = bytes-len b in
    let _ = copy-bytes-dma b 0 dst n in
    wrapping-i64-to-u32 (nat-to-i64 n)

  copy-bytes-dma :: Bytes -> Nat -> I64 -> Nat ->{MVar} Unit
  copy-bytes-dma b i dst n =
    if i >= n then ()
    else if i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 dst (nat-to-i64 i)) (bytes-get-u8 b i) in
      copy-bytes-dma b (i + 1) dst n
    else ()

  dma-eq-str :: I64 -> U32 -> String ->{MVar} Bool
  dma-eq-str addr len expected =
    let b = string-to-bytes expected in
    if u32-to-nat len /= bytes-len b then False
    else dma-eq-bytes addr b 0 (bytes-len b)

  dma-eq-bytes :: I64 -> Bytes -> Nat -> Nat ->{MVar} Bool
  dma-eq-bytes addr b i n =
    if i >= n then True
    else if i < bytes-len b then
      if eq-u8 (raw-load-u8 (wrapping-add-i64 addr (nat-to-i64 i))) (bytes-get-u8 b i) then
        dma-eq-bytes addr b (i + 1) n
      else False
    else False

  -- Format deterministic key "k:<idx>" into `buf`, returning byte length.
  make-key :: I64 -> U32 ->{MVar} U32
  make-key buf idx =
    let _0 = raw-store-u8 buf 107 in -- 'k'
    let _1 = raw-store-u8 (wrapping-add-i64 buf 1) 58 in -- ':'
    let n = kv-format-i64-dma (wrapping-add-i64 buf 2) (u32-to-i64 idx) in
    wrapping-add-u32 2 n

  -- Format deterministic value "val-<tag>-<idx>" into `buf`, returning byte length.
  make-val :: I64 -> U32 -> U32 ->{MVar} U32
  make-val buf tag idx =
    let _0 = raw-store-u8 buf 118 in -- 'v'
    let _1 = raw-store-u8 (wrapping-add-i64 buf 1) 45 in -- '-'
    let n1 = kv-format-i64-dma (wrapping-add-i64 buf 2) (u32-to-i64 tag) in
    let p1 = wrapping-add-u32 2 n1 in
    let _2 = raw-store-u8 (wrapping-add-i64 buf (u32-to-i64 p1)) 45 in
    let p2 = wrapping-add-u32 p1 1 in
    let n2 = kv-format-i64-dma (wrapping-add-i64 buf (u32-to-i64 p2)) (u32-to-i64 idx) in
    wrapping-add-u32 p2 n2

  insert-range :: KvShard -> I64 -> I64 -> U32 -> U32 -> U32 ->{MVar} Bool
  insert-range sh kbuf vbuf i end tag =
    if ge-u32 i end then True
    else
      let kl = make-key kbuf i in
      let vl = make-val vbuf tag i in
      let st = kv-shard-set sh kbuf kl vbuf vl in
      if ne-u8 st kv-ok then False
      else insert-range sh kbuf vbuf (wrapping-add-u32 i 1) end tag

  delete-even :: KvShard -> I64 -> U32 -> U32 ->{MVar} Bool
  delete-even sh kbuf i end =
    if ge-u32 i end then True
    else
      let kl = make-key kbuf i in
      let d = kv-shard-del sh kbuf kl in
      if ne-u8 d 1 then False
      else delete-even sh kbuf (wrapping-add-u32 i 2) end

  verify-range :: KvShard -> I64 -> I64 -> U32 -> U32 -> U32 -> U32 ->{MVar} Bool
  verify-range sh kbuf vbuf i end step tag =
    if ge-u32 i end then True
    else
      let kl = make-key kbuf i in
      let vl = make-val vbuf tag i in
      let gr = kv-shard-get sh kbuf kl in
      if eq-u8 gr.kv-found 0 || ne-u32 gr.kv-val-len vl then False
      else if not (check-eq-dma gr.kv-val-addr vbuf 0 vl) then False
      else verify-range sh kbuf vbuf (wrapping-add-u32 i step) end step tag

  check-eq-dma :: I64 -> I64 -> U32 -> U32 ->{MVar} Bool
  check-eq-dma a b i n =
    if ge-u32 i n then True
    else if eq-u8 (raw-load-u8 (wrapping-add-i64 a (u32-to-i64 i))) (raw-load-u8 (wrapping-add-i64 b (u32-to-i64 i))) then
      check-eq-dma a b (wrapping-add-u32 i 1) n
    else False

  -- Part A: SwissTable 32-slot (2-group) insert, cross-group probe chains,
  -- delete creating tombstones, lookup across tombstones, update in place,
  -- and tombstone reuse!
  test-swisstable-and-tombstones :: I64 -> I64 ->{IO, MVar} U32
  test-swisstable-and-tombstones kbuf vbuf =
    case kv-shard-new (core-id 0) 32 16384 64 1000 of
      None -> 10
      Some sh ->
        -- Insert 26 keys into 32 slots (> 80% load, guarantees multi-group probe chains!)
        let ok1 = insert-range sh kbuf vbuf 0 26 1 in
        let c1 = kv-shard-live-count sh in
        -- Update all 26 keys in place with tag 2
        let ok2 = insert-range sh kbuf vbuf 0 26 2 in
        let ok2v = verify-range sh kbuf vbuf 0 26 1 2 in
        -- Delete 13 even keys (creates tombstones in filled groups)
        let ok3 = delete-even sh kbuf 0 26 in
        let c2 = kv-shard-live-count sh in
        -- Odd keys must still all be found across tombstones!
        let ok4 = verify-range sh kbuf vbuf 1 26 2 2 in
        -- Re-insert 13 brand new keys (100..113) reusing tombstones + freed arena chunks!
        let arena-before = kv-shard-arena-used sh in
        let ok5 = insert-range sh kbuf vbuf 100 113 3 in
        let arena-after = kv-shard-arena-used sh in
        let c3 = kv-shard-live-count sh in
        let ok6 = verify-range sh kbuf vbuf 1 26 2 2 && verify-range sh kbuf vbuf 100 113 1 3 in
        let _d = kv-shard-destroy sh in
        if not ok1 || ne-u32 c1 26 then 11
        else if not ok2 || not ok2v then 12
        else if not ok3 || ne-u32 c2 13 then 13
        else if not ok4 then 14
        else if not ok5 || ne-u32 c3 26 then 15
        else if gt-u32 arena-after arena-before then 16
        else if not ok6 then 17
        else 0

  -- Part B: INCR/DECR edge cases + active (TimerWheel) & passive TTL expiry
  test-incr-and-ttl :: I64 -> I64 ->{IO, MVar} U32
  test-incr-and-ttl kbuf vbuf =
    case kv-shard-new (core-id 0) 64 32768 128 1000 of
      None -> 20
      Some sh ->
        -- 1. INCR/DECR on missing key, existing int, non-int, and I64::MAX overflow
        let kl-cnt = copy-str-dma "counter" kbuf in
        let r1 = kv-shard-incr sh kbuf kl-cnt in
        let r2 = kv-shard-incr sh kbuf kl-cnt in
        let r3 = kv-shard-decr sh kbuf kl-cnt in
        let gr-cnt = kv-shard-get sh kbuf kl-cnt in
        let ok-cnt = (
          eq-u8 r1.kv-int-status kv-ok && eq-i64 r1.kv-int-val 1
            && eq-u8 r2.kv-int-status kv-ok && eq-i64 r2.kv-int-val 2
            && eq-u8 r3.kv-int-status kv-ok && eq-i64 r3.kv-int-val 1
            && eq-u8 gr-cnt.kv-found 1 && dma-eq-str gr-cnt.kv-val-addr gr-cnt.kv-val-len "1") in

        let kl-str = copy-str-dma "notnum" kbuf in
        let vl-str = copy-str-dma "hello" vbuf in
        let _s1 = kv-shard-set sh kbuf kl-str vbuf vl-str in
        let r-bad = kv-shard-incr sh kbuf kl-str in

        let kl-max = copy-str-dma "maxint" kbuf in
        let vl-max = copy-str-dma "9223372036854775807" vbuf in
        let _s2 = kv-shard-set sh kbuf kl-max vbuf vl-max in
        let r-ov = kv-shard-incr sh kbuf kl-max in

        -- 2. Active TimerWheel expiry (SET PX 50ms -> deadline 1050ms -> tick to 1060ms)
        let kl-act = copy-str-dma "ttl:active" kbuf in
        let vl-act = copy-str-dma "soon-gone" vbuf in
        let _sa = kv-shard-set-px sh kbuf kl-act vbuf vl-act 50 in
        let pttl-before = kv-shard-pttl sh kbuf kl-act in
        let evicted = kv-shard-tick sh 1060 in
        let gr-act = kv-shard-get sh kbuf kl-act in
        let ttl-act-after = kv-shard-ttl sh kbuf kl-act in

        -- 3. Passive expiry on read (SET EX 2s at t=1060 -> deadline 3060;
        --    advance shard clock to 3100 WITHOUT calling kv-shard-tick -> GET/EXISTS/TTL must NOT observe it!)
        let kl-pas = copy-str-dma "ttl:passive" kbuf in
        let vl-pas = copy-str-dma "hidden" vbuf in
        let _sp = kv-shard-set-ex sh kbuf kl-pas vbuf vl-pas 2 in
        let ttl-pas-live = kv-shard-ttl sh kbuf kl-pas in
        let _clk = kv-shard-set-now-ms sh 3100 in
        let gr-pas = kv-shard-get sh kbuf kl-pas in
        let ex-pas = kv-shard-exists sh kbuf kl-pas in

        -- 4. Stale timer cancellation: SET PX 100ms, then overwrite with persistent SET,
        --    then tick past 100ms -> persistent key MUST still be alive!
        let _t0 = kv-shard-tick sh 3100 in
        let kl-keep = copy-str-dma "ttl:overwritten" kbuf in
        let vl-k1 = copy-str-dma "temp" vbuf in
        let _sk1 = kv-shard-set-px sh kbuf kl-keep vbuf vl-k1 100 in
        let vl-k2 = copy-str-dma "forever" vbuf in
        let _sk2 = kv-shard-set sh kbuf kl-keep vbuf vl-k2 in
        let _t1 = kv-shard-tick sh 3500 in
        let gr-keep = kv-shard-get sh kbuf kl-keep in
        let ok-keep = eq-u8 gr-keep.kv-found 1 && dma-eq-str gr-keep.kv-val-addr gr-keep.kv-val-len "forever" in
        let ttl-keep = kv-shard-ttl sh kbuf kl-keep in

        -- 5. EXPIRE on existing key & missing key
        let kl-exp = copy-str-dma "exp:target" kbuf in
        let vl-exp = copy-str-dma "42" vbuf in
        let _se = kv-shard-set sh kbuf kl-exp vbuf vl-exp in
        let e-ok = kv-shard-expire sh kbuf kl-exp 5 in
        let r-inc-ttl = kv-shard-incr sh kbuf kl-exp in
        let ttl-preserved = kv-shard-ttl sh kbuf kl-exp in
        let _t2 = kv-shard-tick sh 9000 in
        let gr-exp-gone = kv-shard-get sh kbuf kl-exp in

        let _d = kv-shard-destroy sh in
        if not ok-cnt then 21
        else if ne-u8 r-bad.kv-int-status kv-err-not-int || ne-u8 r-ov.kv-int-status kv-err-overflow then 22
        else if ne-i64 pttl-before 50 || ne-u32 evicted 1 || ne-u8 gr-act.kv-found 0 || ne-i64 ttl-act-after (-2) then 23
        else if ne-i64 ttl-pas-live 2 || ne-u8 gr-pas.kv-found 0 || ne-u8 ex-pas 0 then 24
        else if not ok-keep || ne-i64 ttl-keep (-1) then 25
        else if ne-u8 e-ok 1 || ne-i64 r-inc-ttl.kv-int-val 43 || ne-i64 ttl-preserved 5 || ne-u8 gr-exp-gone.kv-found 0 then 26
        else 0

  -- Part C: Store.Wal AOF durability + power-loss crash recovery!
  test-wal-aof-recovery-full :: forall (r :: Region). I64 -> I64 ->{IO, IO r, MVar, Mut r} U32
  test-wal-aof-recovery-full kbuf vbuf =
    let w = sim-world-new 424242 faults-clean in
    let fd = sim-disk-open w 4096 16 in
    let ops = wal-sim-disk-ops w 4096 16 in
    case wal-engine-new fd 65536 16384 1 of
      None -> 30
      Some wal1 ->
        case kv-shard-new (core-id 0) 64 32768 64 1000 of
          None ->
            let _dw = wal-engine-destroy wal1 in 31
          Some sh1 ->
            let e0 = wal-init-segment ops wal1 in
            if not (io-error-is-ok e0) then
              let _ds = kv-shard-destroy sh1 in
              let _dw = wal-engine-destroy wal1 in 32
            else
              let kl1 = copy-str-dma "user:1" kbuf in
              let vl1 = copy-str-dma "alice" vbuf in
              let _l1 = kv-shard-set-wal sh1 wal1 kbuf kl1 vbuf vl1 in
              let kl2 = copy-str-dma "user:2" kbuf in
              let vl2 = copy-str-dma "bob" vbuf in
              let _l2 = kv-shard-set-wal sh1 wal1 kbuf kl2 vbuf vl2 in
              let _d2 = kv-shard-del-wal sh1 wal1 kbuf kl2 in
              let kl3 = copy-str-dma "hits" kbuf in
              let _i1 = kv-shard-incr-wal sh1 wal1 kbuf kl3 in
              let _i2 = kv-shard-incr-wal sh1 wal1 kbuf kl3 in
              let _i3 = kv-shard-incr-wal sh1 wal1 kbuf kl3 in
              let kl4 = copy-str-dma "session:old" kbuf in
              let vl4 = copy-str-dma "expired" vbuf in
              let _l4 = kv-shard-set-px-wal sh1 wal1 kbuf kl4 vbuf vl4 500 in
              let kl5 = copy-str-dma "session:live" kbuf in
              let vl5 = copy-str-dma "active" vbuf in
              let _l5 = kv-shard-set-px-wal sh1 wal1 kbuf kl5 vbuf vl5 5000 in
              case wal-group-commit ops wal1 of
                Err _ ->
                  let _ds = kv-shard-destroy sh1 in
                  let _dw = wal-engine-destroy wal1 in 33
                Ok committed-lsn ->
                  -- Stage an UN-FSYNCED write and simulate sudden power loss!
                  let kl-lost = copy-str-dma "uncommitted" kbuf in
                  let vl-lost = copy-str-dma "phantom" vbuf in
                  let _ll = kv-shard-set-wal sh1 wal1 kbuf kl-lost vbuf vl-lost in
                  let _uf = wal-flush-unfsynced ops wal1 in
                  let _cr = sim-disk-crash-power-loss! w fd in
                  let _ds1 = kv-shard-destroy sh1 in
                  let _dw1 = wal-engine-destroy wal1 in
                  -- Reopen fresh KvShard + WalEngine at t=2000ms and recover!
                  case wal-engine-new fd 65536 16384 2 of
                    None -> 34
                    Some wal2 ->
                      case kv-shard-new (core-id 0) 64 32768 64 2000 of
                        None ->
                          let _dw2 = wal-engine-destroy wal2 in 35
                        Some sh2 ->
                          case kv-shard-recover-wal ops wal2 fd sh2 2000 of
                            Err _ ->
                              let _ds2 = kv-shard-destroy sh2 in
                              let _dw2 = wal-engine-destroy wal2 in 36
                            Ok rep ->
                              let ku1 = copy-str-dma "user:1" kbuf in
                              let gu1 = kv-shard-get sh2 kbuf ku1 in
                              let ok-u1 = eq-u8 gu1.kv-found 1 && dma-eq-str gu1.kv-val-addr gu1.kv-val-len "alice" in
                              let ku2 = copy-str-dma "user:2" kbuf in
                              let gu2 = kv-shard-get sh2 kbuf ku2 in
                              let kh = copy-str-dma "hits" kbuf in
                              let gh = kv-shard-get sh2 kbuf kh in
                              let ok-h = eq-u8 gh.kv-found 1 && dma-eq-str gh.kv-val-addr gh.kv-val-len "3" in
                              let kold = copy-str-dma "session:old" kbuf in
                              let gold = kv-shard-get sh2 kbuf kold in
                              let klive = copy-str-dma "session:live" kbuf in
                              let glive = kv-shard-get sh2 kbuf klive in
                              let ok-live = eq-u8 glive.kv-found 1 && dma-eq-str glive.kv-val-addr glive.kv-val-len "active" in
                              let pttl-live = kv-shard-pttl sh2 kbuf klive in
                              let klost = copy-str-dma "uncommitted" kbuf in
                              let glost = kv-shard-get sh2 kbuf klost in
                              -- Tick recovered shard past 6000ms -> session:live expires via TimerWheel!
                              let ev = kv-shard-tick sh2 6100 in
                              let glive-after = kv-shard-get sh2 kbuf klive in
                              let _ds2 = kv-shard-destroy sh2 in
                              let _dw2 = wal-engine-destroy wal2 in
                              if ne-u64 rep.rec-durable-lsn committed-lsn then 37
                              else if not ok-u1 then 38
                              else if ne-u8 gu2.kv-found 0 then 39
                              else if not ok-h then 40
                              else if ne-u8 gold.kv-found 0 then 41
                              else if not ok-live || ne-i64 pttl-live 4000 then 42
                              else if ne-u8 glost.kv-found 0 then 43
                              else if ne-u32 ev 1 || ne-u8 glive-after.kv-found 0 then 44
                              else 0

  main :: forall (r :: Region). Unit ->{IO, IO r, MVar, Mut r} Int
  main _ =
    let dma = engine-dma-alloc 4096 4096 in
    if eq-u64 dma 0 then 99
    else
      let kbuf = wrapping-u64-to-i64 dma in
      let vbuf = wrapping-add-i64 kbuf 512 in
      let r1 = test-swisstable-and-tombstones kbuf vbuf in
      let r2 = if eq-u32 r1 0 then test-incr-and-ttl kbuf vbuf else r1 in
      let r3 = if eq-u32 r2 0 then test-wal-aof-recovery-full kbuf vbuf else r2 in
      let _f = engine-dma-free dma 4096 in
      int-from-i64 (u32-to-i64 r3)
"#;

    build_and_run_leak_checked("shard-swisstable-ttl-wal", src, &[], 0);
}

// =========================================================================
// 4. Multi-Core Pinned Worker Mesh Linearizability + Live Loopback RESP Test
// =========================================================================

#[cfg(target_os = "macos")]
#[test]
fn kv_mesh_multicore_pinned_workers_linearizability_and_live_loopback_resp() {
    let bridge = c_fixture(
        "kv_mesh_worker_bridge",
        r#"
#include <stdint.h>

extern int64_t kv_mesh_worker_entry(int64_t slot_addr);

typedef struct {
    uint64_t header;
    int64_t code;
    int64_t arity;
    int64_t nfilled;
} CinchStaticClosure;

#define IMMORTAL_CLOSURE_HEADER ((uint64_t)2ULL << 35)

static int64_t stub_kv_worker(int64_t self, int64_t argc, int64_t *argv) {
    (void)self;
    (void)argc;
    return kv_mesh_worker_entry(argv[0]);
}

static CinchStaticClosure clo_kv_worker;

int64_t kv_get_worker_closure(void) {
    clo_kv_worker.header = IMMORTAL_CLOSURE_HEADER;
    clo_kv_worker.code = (int64_t)(uintptr_t)stub_kv_worker;
    clo_kv_worker.arity = 1;
    clo_kv_worker.nfilled = 0;
    return (int64_t)(uintptr_t)&clo_kv_worker.code;
}
"#,
    );

    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Engine.Reactor.Darwin
  import Engine.Ring.Spsc
  import Engine.Runtime
  import Engine.Types
  import Prim.Atomic
  import Store.Kv.Engine
  import Store.Kv.Resp
  import Store.Kv.Server

  foreign import ccall "kv_get_worker_closure" getKvWorkerClo :: Unit ->{IO} I64

  -- Shared coord DMA slab layout (`cfg-user-ctx` points here, 4096B):
  --   +0  : xp-base           :: I64
  --   +8  : xp-total-bytes    :: U64
  --   +16 : xp-next-off       :: U64
  --   +24 : xp-slots-off      :: U64
  --   +32 : xp-cores          :: U32
  --   +36 : xp-slots-per-core :: U32
  --   +40 : atomic bound-port :: I64 (Core 0 publishes live loopback TCP port)
  --   +48 : atomic phase1-done:: I64 (incremented by each worker after 200 INCRs)
  --   +56 : atomic error-code :: I64
  --   +64 : k0-len            :: U32 (key owned by Core 0)
  --   +68 : k1-len            :: U32 (key owned by Core 1)
  --   +128: k0-bytes[32]
  --   +160: k1-bytes[32]

  load-xpool :: I64 ->{MVar} KvXCorePool
  load-xpool coord =
    KvXCorePool
      (raw-load-i64 coord)
      (raw-load-u64 (wrapping-add-i64 coord 8))
      (raw-load-u64 (wrapping-add-i64 coord 16))
      (raw-load-u64 (wrapping-add-i64 coord 24))
      (raw-load-u32 (wrapping-add-i64 coord 32))
      (raw-load-u32 (wrapping-add-i64 coord 36))

  copy-str-dma :: String -> I64 ->{MVar} U32
  copy-str-dma s dst =
    let b = string-to-bytes s in
    let n = bytes-len b in
    let _ = copy-bytes-dma b 0 dst n in
    wrapping-i64-to-u32 (nat-to-i64 n)

  copy-bytes-dma :: Bytes -> Nat -> I64 -> Nat ->{MVar} Unit
  copy-bytes-dma b i dst n =
    if i >= n then ()
    else if i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 dst (nat-to-i64 i)) (bytes-get-u8 b i) in
      copy-bytes-dma b (i + 1) dst n
    else ()

  dma-eq-str :: I64 -> U32 -> String ->{MVar} Bool
  dma-eq-str addr len expected =
    let b = string-to-bytes expected in
    if u32-to-nat len /= bytes-len b then False
    else dma-eq-bytes addr b 0 (bytes-len b)

  dma-eq-bytes :: I64 -> Bytes -> Nat -> Nat ->{MVar} Bool
  dma-eq-bytes addr b i n =
    if i >= n then True
    else if i < bytes-len b then
      if eq-u8 (raw-load-u8 (wrapping-add-i64 addr (nat-to-i64 i))) (bytes-get-u8 b i) then
        dma-eq-bytes addr b (i + 1) n
      else False
    else False

  -- Discover a short ASCII key whose `kv-owner-core(key, 2) == want-core`.
  find-key-for-core :: I64 -> U16 -> U32 ->{MVar} U32
  find-key-for-core dst want-core idx =
    let _0 = raw-store-u8 dst 115 in -- 's'
    let _1 = raw-store-u8 (wrapping-add-i64 dst 1) 104 in -- 'h'
    let _2 = raw-store-u8 (wrapping-add-i64 dst 2) 58 in -- ':'
    let n = kv-format-i64-dma (wrapping-add-i64 dst 3) (u32-to-i64 idx) in
    let len = wrapping-add-u32 3 n in
    if eq-u16 (kv-owner-core dst len 2) want-core then len
    else find-key-for-core dst want-core (wrapping-add-u32 idx 1)

  -- Phase 1 worker loop: execute 100 INCRs on k0 (Core 0) and 100 INCRs on k1 (Core 1)
  -- concurrently across both pinned cores, pumping inbox on every step!
  run-phase1-incrs :: KvMeshServer -> I64 -> U32 -> I64 -> U32 -> I64 -> U32 -> U32 ->{MVar} Bool
  run-phase1-incrs srv k0 k0-len k1 k1-len scratch i total =
    if ge-u32 i total then True
    else
      let _p = kv-mesh-poll-inbox srv 8 in
      let cid = kv-mesh-server-core srv in
      let self-raw = cid.core-id-raw in
      -- 1. Increment k0 (local on Core 0, cross-core SPSC on Core 1!)
      let ok0 = (
        if eq-u16 self-raw 0 then
          let ir0 = kv-shard-incr (kv-mesh-server-shard srv) k0 k0-len in
          eq-u8 ir0.kv-int-status kv-ok
        else
          let n = kv-mesh-remote-exec-sync srv 0 cmd-incr k0 k0-len 0 0 0 scratch 0 256 2 in
          gt-u32 n 0 && le-u32 n 256) in
      let _p2 = kv-mesh-poll-inbox srv 8 in
      -- 2. Increment k1 (cross-core SPSC on Core 0, local on Core 1!)
      let ok1 = (
        if eq-u16 self-raw 1 then
          let ir1 = kv-shard-incr (kv-mesh-server-shard srv) k1 k1-len in
          eq-u8 ir1.kv-int-status kv-ok
        else
          let n = kv-mesh-remote-exec-sync srv 1 cmd-incr k1 k1-len 0 0 0 scratch 0 256 2 in
          gt-u32 n 0 && le-u32 n 256) in
      if ok0 && ok1 then run-phase1-incrs srv k0 k0-len k1 k1-len scratch (wrapping-add-u32 i 1) total
      else False

  -- Core 0 steady-state loop: pump SPSC inbox + accept/serve live loopback RESP TCP client
  server-worker-loop :: WorkerState -> KvMeshServer -> Fd -> I32 -> I64 -> I64 -> I64 -> I64 -> I64 -> I64 ->{IO, MVar} I32
  server-worker-loop ws srv lfd peer-raw rx-buf tx-buf cmds-buf args-buf rx-used-addr proto-addr =
    if worker-is-shutdown-requested ws then peer-raw
    else
      let _in = kv-mesh-poll-inbox srv 32 in
      let cur-peer = (
        if lt-i32 peer-raw 0 then
          case darwin-tcp-accept-nonblock lfd of
            Ok cfd ->
              let _r = raw-store-u32 rx-used-addr 0 in
              let _p = raw-store-u8 proto-addr 2 in
              cfd.fd-raw
            Err _ -> -1
        else peer-raw) in
      let next-peer = (
        if ge-i32 cur-peer 0 then
          let now-ms = div-u64 (darwin-clock-monotonic-ns ()) 1000000 in
          let rc = kv-darwin-serve-once srv (fd-of cur-peer) rx-buf 16384 rx-used-addr tx-buf 16384 cmds-buf args-buf proto-addr now-ms in
          if lt-i32 rc 0 then
            let _cl = darwin-close-fd (fd-of cur-peer) in
            -1
          else cur-peer
        else cur-peer) in
      let _p = cpu-pause () in
      server-worker-loop ws srv lfd next-peer rx-buf tx-buf cmds-buf args-buf rx-used-addr proto-addr

  -- Core 1 steady-state loop: pump SPSC inbox for cross-core forwarded requests until shutdown
  peer-worker-loop :: WorkerState -> KvMeshServer ->{IO, MVar} Unit
  peer-worker-loop ws srv =
    if worker-is-shutdown-requested ws then ()
    else
      let n = kv-mesh-poll-inbox srv 32 in
      let _p = if eq-u32 n 0 then cpu-pause () else () in
      peer-worker-loop ws srv

  worker-entry :: I64 ->{IO, MVar} I64
  worker-entry slot-addr =
    let now-ms = div-u64 (darwin-clock-monotonic-ns ()) 1000000 in
    case runtime-boot-init slot-addr now-ms of
      None -> -1
      Some ws ->
        let coord = runtime-boot-user-ctx slot-addr in
        let cid = worker-core-id ws in
        let xpool = load-xpool coord in
        case kv-shard-new cid 256 65536 128 now-ms of
          None ->
            let _dw = worker-state-destroy ws in -2
          Some sh ->
            let srv = kv-mesh-server-new cid 2 sh (worker-mesh ws) xpool in
            let work-dma = engine-dma-alloc 65536 4096 in
            if eq-u64 work-dma 0 then
              let _ds = kv-shard-destroy sh in
              let _dw = worker-state-destroy ws in -3
            else
              let wbase = wrapping-u64-to-i64 work-dma in
              let rx-buf = wbase in
              let tx-buf = wrapping-add-i64 wbase 16384 in
              let cmds-buf = wrapping-add-i64 wbase 32768 in
              let args-buf = wrapping-add-i64 wbase 36864 in
              let scratch = wrapping-add-i64 wbase 45056 in
              let rx-used-addr = wrapping-add-i64 wbase 49152 in
              let proto-addr = wrapping-add-i64 wbase 49160 in
              let k0 = wrapping-add-i64 coord 128 in
              let k1 = wrapping-add-i64 coord 160 in
              let k0-len = raw-load-u32 (wrapping-add-i64 coord 64) in
              let k1-len = raw-load-u32 (wrapping-add-i64 coord 68) in
              if eq-u16 cid.core-id-raw 0 then
                case darwin-tcp-listen-reuseport 0 64 of
                  Err _ ->
                    let _a = atomic-store-release-i64 (wrapping-add-i64 coord 56) (-10) in
                    let _rdy = runtime-boot-mark-ready slot-addr in
                    let _fw = engine-dma-free work-dma 65536 in
                    let _ds = kv-shard-destroy sh in
                    let _dw = worker-state-destroy ws in -10
                  Ok (lfd, port) ->
                    let _sp = atomic-store-release-i64 (wrapping-add-i64 coord 40) (u16-to-i64 port) in
                    let _rdy = runtime-boot-mark-ready slot-addr in
                    -- Wait for both workers ready before starting concurrent Phase 1 INCRs
                    let ok-p1 = run-phase1-incrs srv k0 k0-len k1 k1-len scratch 0 100 in
                    let _e1 = if not ok-p1 then atomic-store-release-i64 (wrapping-add-i64 coord 56) (-11) else () in
                    let _d1 = atomic-fetch-add-acqrel-i64 (wrapping-add-i64 coord 48) 1 in
                    let last-peer = server-worker-loop ws srv lfd (-1) rx-buf tx-buf cmds-buf args-buf rx-used-addr proto-addr in
                    let _cp = if ge-i32 last-peer 0 then darwin-close-fd (fd-of last-peer) else IoOk in
                    let _cl = darwin-close-fd lfd in
                    let _pub = runtime-boot-publish-and-done slot-addr ws 0 0 (wrapping-u64-to-i64 (kv-shard-ops-count sh)) in
                    let _fw = engine-dma-free work-dma 65536 in
                    let _ds = kv-shard-destroy sh in
                    let _dw = worker-state-destroy ws in 0
              else
                let _rdy = runtime-boot-mark-ready slot-addr in
                let ok-p1 = run-phase1-incrs srv k0 k0-len k1 k1-len scratch 0 100 in
                let _e1 = if not ok-p1 then atomic-store-release-i64 (wrapping-add-i64 coord 56) (-21) else () in
                let _d1 = atomic-fetch-add-acqrel-i64 (wrapping-add-i64 coord 48) 1 in
                let _lp = peer-worker-loop ws srv in
                let _pub = runtime-boot-publish-and-done slot-addr ws 0 0 (wrapping-u64-to-i64 (kv-shard-ops-count sh)) in
                let _fw = engine-dma-free work-dma 65536 in
                let _ds = kv-shard-destroy sh in
                let _dw = worker-state-destroy ws in 0

  foreign export ccall "kv_mesh_worker_entry" worker-entry :: I64 ->{IO, MVar} I64

  wait-atomic-ge :: I64 -> I64 -> U64 ->{MVar} Bool
  wait-atomic-ge addr want rem =
    if ge-i64 (atomic-load-acquire-i64 addr) want then True
    else if eq-u64 rem 0 then False
    else
      let _p = cpu-pause () in
      wait-atomic-ge addr want (wrapping-sub-u64 rem 1)

  send-all-cli :: Fd -> I64 -> U32 -> U32 -> U32 ->{IO, MVar} Bool
  send-all-cli fd buf sent total budget =
    if ge-u32 sent total then True
    else if eq-u32 budget 0 then False
    else
      let rem = wrapping-sub-u32 total sent in
      let nw = darwin-send-dma fd (wrapping-add-i64 buf (u32-to-i64 sent)) rem in
      if gt-i32 nw 0 then send-all-cli fd buf (wrapping-add-u32 sent (wrapping-i32-to-u32 nw)) total budget
      else if eq-i32 nw (-35) || eq-i32 nw (-4) || eq-i32 nw (-57) then
        let _p = cpu-pause () in
        send-all-cli fd buf sent total (wrapping-sub-u32 budget 1)
      else False

  recv-exact-cli :: Fd -> I64 -> U32 -> U32 -> U32 ->{IO, MVar} Bool
  recv-exact-cli fd buf got want budget =
    if ge-u32 got want then True
    else if eq-u32 budget 0 then False
    else
      let rem = wrapping-sub-u32 want got in
      let nr = darwin-recv-dma fd (wrapping-add-i64 buf (u32-to-i64 got)) rem in
      if gt-i32 nr 0 then recv-exact-cli fd buf (wrapping-add-u32 got (wrapping-i32-to-u32 nr)) want budget
      else if eq-i32 nr (-35) || eq-i32 nr (-4) then
        let _p = cpu-pause () in
        recv-exact-cli fd buf got want (wrapping-sub-u32 budget 1)
      else False

  -- Encode a single RESP command array into `dst + pos`.
  append-cmd1 :: I64 -> U32 -> I64 -> String ->{MVar} U32
  append-cmd1 dst pos tmp a0 =
    let p0 = resp-write-array-header dst pos 16384 1 in
    let n0 = copy-str-dma a0 tmp in
    resp-write-bulk-raw dst p0 16384 tmp n0

  append-cmd2-raw :: I64 -> U32 -> I64 -> String -> I64 -> U32 ->{MVar} U32
  append-cmd2-raw dst pos tmp a0 kaddr klen =
    let p0 = resp-write-array-header dst pos 16384 2 in
    let n0 = copy-str-dma a0 tmp in
    let p1 = resp-write-bulk-raw dst p0 16384 tmp n0 in
    resp-write-bulk-raw dst p1 16384 kaddr klen

  append-cmd3-raw :: I64 -> U32 -> I64 -> String -> I64 -> U32 -> String ->{MVar} U32
  append-cmd3-raw dst pos tmp a0 kaddr klen a2 =
    let p0 = resp-write-array-header dst pos 16384 3 in
    let n0 = copy-str-dma a0 tmp in
    let p1 = resp-write-bulk-raw dst p0 16384 tmp n0 in
    let p2 = resp-write-bulk-raw dst p1 16384 kaddr klen in
    let n2 = copy-str-dma a2 tmp in
    resp-write-bulk-raw dst p2 16384 tmp n2

  append-mget2 :: I64 -> U32 -> I64 -> I64 -> U32 -> I64 -> U32 ->{MVar} U32
  append-mget2 dst pos tmp k0 k0-len k1 k1-len =
    let p0 = resp-write-array-header dst pos 16384 3 in
    let n0 = copy-str-dma "MGET" tmp in
    let p1 = resp-write-bulk-raw dst p0 16384 tmp n0 in
    let p2 = resp-write-bulk-raw dst p1 16384 k0 k0-len in
    resp-write-bulk-raw dst p2 16384 k1 k1-len

  -- Run live loopback -P 16 pipelined RESP2 + RESP3 client against Core 0
  -- exercising BOTH Core 0 local keys AND Core 1 cross-core forwarded keys!
  run-loopback-resp-client :: U16 -> I64 -> U32 -> I64 -> U32 ->{IO, MVar} U32
  run-loopback-resp-client port k0 k0-len k1 k1-len =
    case darwin-tcp-connect-nonblock ipv4-loopback port of
      Err _ -> 50
      Ok cfd ->
        let dma = engine-dma-alloc 32768 4096 in
        if eq-u64 dma 0 then
          let _cl = darwin-close-fd cfd in 51
        else
          let tx = wrapping-u64-to-i64 dma in
          let rx = wrapping-add-i64 tx 16384 in
          let tmp = wrapping-add-i64 tx 28672 in
          let k-ext0 = wrapping-add-i64 tx 29000 in
          let k-ext1 = wrapping-add-i64 tx 29064 in
          let ke0-len = find-key-for-core k-ext0 0 100 in
          let ke1-len = find-key-for-core k-ext1 1 200 in

          -- Build a 16-command (-P 16) pipelined RESP batch hitting BOTH cores!
          --  1. PING                                 -> +PONG\r\n
          --  2. GET k0 (Core 0: 100 local + 100 xcore INCRs == "200") -> $3\r\n200\r\n
          --  3. GET k1 (Core 1: 100 local + 100 xcore INCRs == "200") -> $3\r\n200\r\n
          --  4. SET ke0 "alpha" (Core 0 local)       -> +OK\r\n
          --  5. SET ke1 "beta"  (Core 1 forwarded!)  -> +OK\r\n
          --  6. GET ke0                              -> $5\r\nalpha\r\n
          --  7. GET ke1                              -> $4\r\nbeta\r\n
          --  8. MGET ke0 ke1 (spans BOTH cores!)     -> *2\r\n$5\r\nalpha\r\n$4\r\nbeta\r\n
          --  9. EXISTS ke1 (Core 1 forwarded!)       -> :1\r\n
          -- 10. EXPIRE ke1 60 (Core 1 forwarded!)    -> :1\r\n
          -- 11. TTL ke1 (Core 1 forwarded!)          -> :60\r\n
          -- 12. INCR k1 (Core 1: 200 -> 201)         -> :201\r\n
          -- 13. DEL ke1 (Core 1 forwarded!)          -> :1\r\n
          -- 14. HELLO 3 (switches conn to RESP3!)    -> %2\r\n$6\r\nserver\r\n$5\r\ncinch\r\n$5\r\nproto\r\n:3\r\n
          -- 15. GET ke1 (deleted -> RESP3 null!)     -> _\r\n
          -- 16. QUIT                                 -> +OK\r\n
          let p1 = append-cmd1 tx 0 tmp "PING" in
          let p2 = append-cmd2-raw tx p1 tmp "GET" k0 k0-len in
          let p3 = append-cmd2-raw tx p2 tmp "GET" k1 k1-len in
          let p4 = append-cmd3-raw tx p3 tmp "SET" k-ext0 ke0-len "alpha" in
          let p5 = append-cmd3-raw tx p4 tmp "SET" k-ext1 ke1-len "beta" in
          let p6 = append-cmd2-raw tx p5 tmp "GET" k-ext0 ke0-len in
          let p7 = append-cmd2-raw tx p6 tmp "GET" k-ext1 ke1-len in
          let p8 = append-mget2 tx p7 tmp k-ext0 ke0-len k-ext1 ke1-len in
          let p9 = append-cmd2-raw tx p8 tmp "EXISTS" k-ext1 ke1-len in
          let p10 = append-cmd3-raw tx p9 tmp "EXPIRE" k-ext1 ke1-len "60" in
          let p11 = append-cmd2-raw tx p10 tmp "TTL" k-ext1 ke1-len in
          let p12 = append-cmd2-raw tx p11 tmp "INCR" k1 k1-len in
          let p13 = append-cmd2-raw tx p12 tmp "DEL" k-ext1 ke1-len in
          let arg3 = wrapping-add-i64 tmp 64 in
          let k3 = copy-str-dma "3" arg3 in
          let p14 = append-cmd2-raw tx p13 tmp "HELLO" arg3 k3 in
          let p15 = append-cmd2-raw tx p14 tmp "GET" k-ext1 ke1-len in
          let p16 = append-cmd1 tx p15 tmp "QUIT" in

          let expected = "+PONG\r\n$3\r\n200\r\n$3\r\n200\r\n+OK\r\n+OK\r\n$5\r\nalpha\r\n$4\r\nbeta\r\n*2\r\n$5\r\nalpha\r\n$4\r\nbeta\r\n:1\r\n:1\r\n:60\r\n:201\r\n:1\r\n%2\r\n$6\r\nserver\r\n$5\r\ncinch\r\n$5\r\nproto\r\n:3\r\n_\r\n+OK\r\n" in
          let exp-len = wrapping-i64-to-u32 (nat-to-i64 (bytes-len (string-to-bytes expected))) in
          let sent-ok = send-all-cli cfd tx 0 p16 200000 in
          let recv-ok = if sent-ok then recv-exact-cli cfd rx 0 exp-len 5000000 else False in
          let match-ok = if recv-ok then dma-eq-str rx exp-len expected else False in
          let _cl = darwin-close-fd cfd in
          let _fd = engine-dma-free dma 32768 in
          if not sent-ok then 52
          else if not recv-ok then 53
          else if not match-ok then 54
          else 0

  main :: Unit ->{IO, MVar} Int
  main _ =
    let coord-u = engine-dma-alloc 4096 4096 in
    if eq-u64 coord-u 0 then 1
    else
      case kv-xcore-pool-new 2 32 of
        None ->
          let _f = engine-dma-free coord-u 4096 in 2
        Some xpool ->
          let coord = wrapping-u64-to-i64 coord-u in
          let _0 = raw-store-i64 coord xpool.xp-base in
          let _1 = raw-store-u64 (wrapping-add-i64 coord 8) xpool.xp-total-bytes in
          let _2 = raw-store-u64 (wrapping-add-i64 coord 16) xpool.xp-next-off in
          let _3 = raw-store-u64 (wrapping-add-i64 coord 24) xpool.xp-slots-off in
          let _4 = raw-store-u32 (wrapping-add-i64 coord 32) xpool.xp-cores in
          let _5 = raw-store-u32 (wrapping-add-i64 coord 36) xpool.xp-slots-per-core in
          let k0 = wrapping-add-i64 coord 128 in
          let k1 = wrapping-add-i64 coord 160 in
          let k0-len = find-key-for-core k0 0 0 in
          let k1-len = find-key-for-core k1 1 0 in
          let _6 = raw-store-u32 (wrapping-add-i64 coord 64) k0-len in
          let _7 = raw-store-u32 (wrapping-add-i64 coord 68) k1-len in
          let _fn = atomic-fence-release () in
          let cfg = WorkerConfig 2 4096 8 32 32 32 256 2 16384 128 500000 coord in
          let clo = wrapping-i64-to-u64 (getKvWorkerClo ()) in
          case runtime-spawn-workers cfg clo of
            None ->
              let _dx = kv-xcore-pool-destroy xpool in
              let _fc = engine-dma-free coord-u 4096 in 3
            Some rt ->
              let rdy = runtime-wait-all-ready rt 100000000 in
              let p1-ok = if rdy then wait-atomic-ge (wrapping-add-i64 coord 48) 2 200000000 else False in
              let port-i = atomic-load-acquire-i64 (wrapping-add-i64 coord 40) in
              let cli-rc = (
                if p1-ok && gt-i64 port-i 0 then
                  run-loopback-resp-client (wrapping-i64-to-u16 port-i) k0 k0-len k1 k1-len
                else 49) in
              let _sh = runtime-request-shutdown rt in
              let join-rc = runtime-join-all rt in
              let err-c = atomic-load-acquire-i64 (wrapping-add-i64 coord 56) in
              let ops0 = runtime-worker-slot-user-out rt (core-id 0) in
              let ops1 = runtime-worker-slot-user-out rt (core-id 1) in
              let _dr = runtime-destroy rt in
              let _dx = kv-xcore-pool-destroy xpool in
              let _fc = engine-dma-free coord-u 4096 in
              if not rdy then 4
              else if not p1-ok || ne-i64 err-c 0 then 5
              else if ne-u32 cli-rc 0 then int-from-i64 (u32-to-i64 cli-rc)
              else if ne-i64 join-rc 0 then 6
              else if lt-i64 ops0 200 || lt-i64 ops1 200 then 7
              else 0
"#;

    build_and_run_leak_checked("multicore-mesh-and-loopback-resp", src, &[bridge], 0);
}

// =========================================================================
// 5. Steady-state 100,000 GET/SET + RESP parse/encode hot loop asserting
//    allocs == 0, atomic_rc == 0, and CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1
// =========================================================================

#[test]
fn kv_steady_state_100000_get_set_resp_zero_alloc_zero_atomic_rc_zero_leak() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.String
  import Engine.Types
  import Prim.Atomic
  import Store.Kv.Engine
  import Store.Kv.Resp
  import Store.Kv.Server

  copy-str-dma :: String -> I64 ->{MVar} U32
  copy-str-dma s dst =
    let b = string-to-bytes s in
    let n = bytes-len b in
    let _ = copy-bytes-dma b 0 dst n in
    wrapping-i64-to-u32 (nat-to-i64 n)

  copy-bytes-dma :: Bytes -> Nat -> I64 -> Nat ->{MVar} Unit
  copy-bytes-dma b i dst n =
    if i >= n then ()
    else if i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 dst (nat-to-i64 i)) (bytes-get-u8 b i) in
      copy-bytes-dma b (i + 1) dst n
    else ()

  -- Execute 50,000 pipelined (SET + GET) pairs = 100,000 RESP commands end-to-end
  -- through SIMD RESP parse -> KvShard SwissTable + DMA arena -> coalesced RESP encode!
  hot-loop :: KvShard -> I64 -> U32 -> I64 -> I64 -> I64 -> U32 -> U64 ->{MVar} U64
  hot-loop sh rx rx-len tx cmds args rem ok-cmds =
    if eq-u32 rem 0 then ok-cmds
    else
      let res = kv-server-handle-conn-dma sh rx rx-len tx 4096 cmds args 2 1000 in
      if eq-u32 res.kcr-cmds-handled 2 && eq-u32 res.kcr-tx-written 16 then
        hot-loop sh rx rx-len tx cmds args (wrapping-sub-u32 rem 1) (wrapping-add-u64 ok-cmds 2)
      else ok-cmds

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 16384 4096 in
    if eq-u64 dma 0 then 90
    else
      let base = wrapping-u64-to-i64 dma in
      let rx = base in
      let tx = wrapping-add-i64 base 4096 in
      let cmds = wrapping-add-i64 base 8192 in
      let args = wrapping-add-i64 base 10240 in
      -- 2-command pipelined batch: SET k001 val01 (+OK\r\n = 5B) + GET k001 ($5\r\nval01\r\n = 11B) = 16B TX
      let wire = "*3\r\n$3\r\nSET\r\n$4\r\nk001\r\n$5\r\nval01\r\n*2\r\n$3\r\nGET\r\n$4\r\nk001\r\n" in
      let rx-len = copy-str-dma wire rx in
      case kv-shard-new (core-id 0) 256 65536 64 1000 of
        None ->
          let _f = engine-dma-free dma 16384 in 91
        Some sh ->
          -- Warm up key once before resetting counters
          let _warm = kv-server-handle-conn-dma sh rx rx-len tx 4096 cmds args 2 1000 in
          -- Reset thread allocation & atomic RC counters immediately before 100,000-op hot loop!
          let _rst = engine-thread-counters-reset () in
          let handled = hot-loop sh rx rx-len tx cmds args 50000 0 in
          let allocs = engine-thread-alloc-count () in
          let atomic-rcs = engine-thread-atomic-rc-count () in
          let _ds = kv-shard-destroy sh in
          let _f = engine-dma-free dma 16384 in
          if ne-u64 handled 100000 then 1
          else if ne-u64 allocs 0 then 2
          else if ne-u64 atomic-rcs 0 then 3
          else 0
"#;

    build_and_run_leak_checked("steady-state-100k-zero-alloc", src, &[], 0);
}
