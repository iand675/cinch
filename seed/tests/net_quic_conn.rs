//! Integration, formal-verification (M8 `QuicMachine`), protocol feature
//! (handshake, Stateless Retry, 3x anti-amplification, 0-RTT + AntiReplayFilter,
//! 1-RTT KeyUpdate, RFC 9221 DATAGRAM, UDP_GSO burst packing, RFC 9000 §9 NAT
//! Connection Migration), 1,000-seed deterministic chaos simulation, and
//! 20,000-cycle zero-allocation / zero-atomic-RC / leak-checked tests for
//! `Net.Quic.{Stream,Conn,Endpoint,Spec}`.
//!
//! Routed through `common::drive_with_profile_opts` / `common::check_interfaces`
//! so the exact same suite runs against both the Rust seed and the Stage 3
//! self-hosted compiler.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{DriverOptions, Mode, Package};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn fresh_dir(tag: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "cinch-net-quic-conn-{tag}-{}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn build_and_run_leak_checked(tag: &str, src: &str, expected_code: i32) -> (String, String) {
    let dir = fresh_dir(tag);
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let exe = dir.join(format!("quic-conn-{tag}"));
    let opts = DriverOptions::default();
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
        "{tag}: expected exit {expected_code}, got {:?}\nstdout:\n{stdout}\nstderr:\n{stderr}",
        out.status.code()
    );
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "{tag}: expected clean leak check (`leaked=0`), got stderr:\n{stderr}"
    );
    (stdout, stderr)
}

// =========================================================================
// 1. Manifest registration, direct stdlib import, & #[no-alloc] safety check
// =========================================================================

#[test]
fn net_quic_conn_manifest_and_no_alloc() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, rel_path) in [
        ("Net.Quic.Stream", "Net/Quic/Stream.cinch"),
        ("Net.Quic.Conn", "Net/Quic/Conn.cinch"),
        ("Net.Quic.Endpoint", "Net/Quic/Endpoint.cinch"),
        ("Net.Quic.Spec", "Net/Quic/Spec.cinch"),
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

    // Verify direct stdlib import AND transitive #[no-alloc] safety across
    // Stream, Conn, and Endpoint hot-path APIs via `common::check_interfaces`.
    let dir = fresh_dir("stdlib-no-alloc-check");
    std::fs::write(
        dir.join("Main.cinch"),
        r#"
module Main where
  import Data.Machine
  import Net.Quic.Types
  import Net.Quic.Stream
  import Net.Quic.Conn
  import Net.Quic.Endpoint
  import Net.Quic.Spec

  #[no-alloc]
  probe-stream-no-alloc :: QuicStreamTable -> U32 -> I64 -> U32 ->{MVar} U32
  probe-stream-no-alloc tbl slot buf len =
    let _w = quic-stream-write-raw tbl slot buf len 0 in
    let _r = quic-stream-read-raw tbl slot buf len in
    let _rx = quic-stream-on-recv-frame tbl (StreamId 0) 0 buf len 0 in
    let _tx = quic-stream-poll-tx tbl len in
    let _fc = quic-stream-poll-flow-control tbl in
    _tx.stx-len

  #[no-alloc]
  probe-conn-no-alloc :: QuicConn -> I64 -> U32 -> U64 -> U64 ->{MVar} U32
  probe-conn-no-alloc conn buf len addr now =
    let _rx = quic-conn-on-datagram conn buf len addr now in
    let tx = quic-conn-poll-tx conn buf len now in
    let _to = quic-conn-on-timeout conn now in
    let _ku = quic-conn-initiate-key-update conn in
    let _dg = quic-conn-send-datagram conn buf 16 in
    let _dr = quic-conn-recv-datagram conn buf 16 in
    tx.tx-len

  #[no-alloc]
  probe-endpoint-no-alloc :: QuicEndpoint -> I64 -> U32 -> U64 -> U64 ->{MVar} U32
  probe-endpoint-no-alloc ep buf len addr now =
    let _scid = quic-endpoint-alloc-scid ep in
    let _lu = quic-endpoint-lookup-dcid ep _scid in
    let _rx = quic-endpoint-on-datagram-existing ep buf len addr now buf len in
    let gso = quic-endpoint-poll-gso-burst ep buf len 8 now in
    let _to = quic-endpoint-on-timeout ep now in
    gso.gso-total-len

  main :: Int
  main = if quic-safety-ok quic-model-init then 0 else 1
"#,
    )
    .expect("write Main.cinch");

    let ifaces = common::check_interfaces(&dir, "Main.cinch")
        .expect("Net.Quic.{Stream,Conn,Endpoint,Spec} must pass #[no-alloc] verification");
    let main_iface = ifaces
        .iter()
        .find(|i| i.module == "Main")
        .expect("Main interface present");
    for fn_name in [
        "probe-stream-no-alloc",
        "probe-conn-no-alloc",
        "probe-endpoint-no-alloc",
    ] {
        let entry = main_iface
            .assertions
            .iter()
            .find(|a| a.name == fn_name)
            .unwrap_or_else(|| panic!("missing assertion for {fn_name}"));
        assert!(
            entry.facts.iter().any(|f| f == "no-alloc"),
            "expected no-alloc fact on {fn_name}, got {:?}",
            entry.facts
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}

// =========================================================================
// 2. Formal Model M8 (`QuicMachine`): capped=false, 0 violations under
//    CINCH_MODEL_FAIL_ON_CAPPED=1 + refinement to AbstractOrderedByteStream
// =========================================================================

#[test]
fn formal_model_m8_quic_machine_verifies_uncapped_zero_violations() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("prelude/Net/Quic/Spec.cinch");
    let spec_str = spec_path.to_string_lossy().into_owned();
    let cinch_bin = env!("CARGO_BIN_EXE_cinch");

    // 1. Check via `common::drive_with_profile_opts` with `model_fail_on_capped = true`
    let dir = fresh_dir("m8-spec-check");
    std::fs::write(
        dir.join("Main.cinch"),
        r#"
module Main where
  import Net.Quic.Spec

  main :: Int
  main = if quic-safety-ok quic-model-init then 0 else 1
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
    .expect("Net.Quic.Spec must check cleanly with model_fail_on_capped=true");
    let _ = std::fs::remove_dir_all(&dir);

    // 2. CLI `cinch model inspect <Spec.cinch> QuicMachine --json` with CINCH_MODEL_FAIL_ON_CAPPED=1
    let inspect_out = Command::new(cinch_bin)
        .args(["model", "inspect", &spec_str, "QuicMachine", "--json"])
        .env("CINCH_MODEL_FAIL_ON_CAPPED", "1")
        .output()
        .expect("run cinch model inspect QuicMachine");
    let stdout = String::from_utf8_lossy(&inspect_out.stdout);
    let stderr = String::from_utf8_lossy(&inspect_out.stderr);
    assert!(
        inspect_out.status.success(),
        "M8 QuicMachine failed under CINCH_MODEL_FAIL_ON_CAPPED=1:\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    let json: serde_json::Value =
        serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("invalid JSON: {e}\n{stdout}"));
    let machines = json
        .as_array()
        .expect("top-level machines array in model inspect JSON");
    let m8 = machines
        .iter()
        .find(|m| m.get("name").and_then(|v| v.as_str()) == Some("QuicMachine"))
        .unwrap_or_else(|| panic!("QuicMachine missing from JSON:\n{stdout}"));
    let capped = m8.get("capped").and_then(|v| v.as_bool()).expect("capped bool");
    let states = m8
        .get("state_count")
        .and_then(|v| v.as_u64())
        .expect("state_count");
    let edges = m8
        .get("edge_count")
        .and_then(|v| v.as_u64())
        .expect("edge_count");
    assert!(!capped, "M8 QuicMachine MUST have capped=false; json:\n{stdout}");
    assert!(states > 10 && states < 65536, "unexpected M8 state count: {states}");

    let contracts = m8
        .get("contracts")
        .and_then(|v| v.as_array())
        .expect("contracts array");
    assert!(
        contracts.len() >= 5,
        "expected >= 5 verified contracts, got {contracts:?}"
    );
    for c in contracts {
        assert_eq!(
            c.get("passed").and_then(|v| v.as_bool()),
            Some(true),
            "contract failed: {c:?}"
        );
    }

    let refines = m8
        .get("refinements")
        .and_then(|v| v.as_array())
        .expect("refinements array");
    let m8_ref = refines
        .iter()
        .find(|r| {
            r.get("concrete").and_then(|v| v.as_str()) == Some("QuicMachine")
                && r.get("abstract").and_then(|v| v.as_str()) == Some("AbstractOrderedByteStream")
        })
        .unwrap_or_else(|| {
            panic!("missing QuicMachine refines AbstractOrderedByteStream in JSON:\n{stdout}")
        });
    assert_eq!(
        m8_ref.get("passed").and_then(|v| v.as_bool()),
        Some(true),
        "refinement failed: {m8_ref:?}"
    );
    assert_eq!(
        m8_ref.get("has_measure").and_then(|v| v.as_bool()),
        Some(true),
        "refinement must have measure: {m8_ref:?}"
    );

    eprintln!(
        "[M8 QuicMachine] state_count={states} edge_count={edges} capped={capped} contracts={} refinement=PASS",
        contracts.len()
    );
}

// =========================================================================
// 3. Full Protocol Suite: Handshake, Stateless Retry, 3x Anti-Amplification,
//    0-RTT + AntiReplayFilter, 1-RTT KeyUpdate, RFC 9221 DATAGRAM, UDP_GSO
//    Burst Packing, and NAT Connection Migration (PATH_CHALLENGE/RESPONSE)
// =========================================================================

#[test]
fn quic_protocol_handshake_retry_amp_0rtt_keyupdate_datagram_gso_migration() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Engine.Types
  import Net.Tls.QuicBridge
  import Net.Quic.Types
  import Net.Quic.Packet
  import Net.Quic.Stream
  import Net.Quic.Conn
  import Net.Quic.Endpoint
  import Prim.Atomic

  fill-pat :: I64 -> U32 -> U32 -> U8 ->{MVar} Unit
  fill-pat addr i n seed =
    if ge-u32 i n then ()
    else
      let v = wrapping-add-u8 (wrapping-mul-u8 (wrapping-u32-to-u8 i) 31) seed in
      let _ = raw-store-u8 (wrapping-add-i64 addr (u32-to-i64 i)) v in
      fill-pat addr (wrapping-add-u32 i 1) n seed

  verify-pat :: I64 -> U32 -> U32 -> U8 ->{MVar} Bool
  verify-pat addr i n seed =
    if ge-u32 i n then True
    else
      let got = raw-load-u8 (wrapping-add-i64 addr (u32-to-i64 i)) in
      let exp = wrapping-add-u8 (wrapping-mul-u8 (wrapping-u32-to-u8 i) 31) seed in
      if eq-u8 got exp then verify-pat addr (wrapping-add-u32 i 1) n seed
      else False

  -- Pump one direction between two QuicConns until no packets remain (bounded).
  pump-conn-dir :: QuicConn -> QuicConn -> U64 -> I64 -> U64 -> U32 ->{IO, MVar} U32
  pump-conn-dir src dst src-addr buf now-ns lim =
    if eq-u32 lim 0 then 0
    else
      let _s1 = if quic-conn-needs-crypto-step src then let _ = quic-conn-step-crypto src now-ns in () else () in
      let p = quic-conn-poll-tx src buf 2048 now-ns in
      if eq-u8 p.tx-ok 0 then 0
      else
        let _rx = quic-conn-on-datagram dst buf p.tx-len src-addr (wrapping-add-u64 now-ns 1000000) in
        let _s2 = if quic-conn-needs-crypto-step dst then let _ = quic-conn-step-crypto dst (wrapping-add-u64 now-ns 1000000) in () else () in
        wrapping-add-u32 1 (pump-conn-dir src dst src-addr buf (wrapping-add-u64 now-ns 100000) (wrapping-sub-u32 lim 1))

  handshake-pair :: QuicConn -> QuicConn -> U64 -> U64 -> I64 -> U64 -> U32 ->{IO, MVar} U64
  handshake-pair cli srv cli-addr srv-addr buf now-ns rounds =
    if eq-u32 rounds 0 then now-ns
    else if quic-conn-is-established cli && quic-conn-is-established srv && quic-conn-handshake-confirmed cli && quic-conn-handshake-confirmed srv then now-ns
    else
      let _a = pump-conn-dir cli srv cli-addr buf now-ns 8 in
      let t1 = wrapping-add-u64 now-ns 5000000 in
      let _b = pump-conn-dir srv cli srv-addr buf t1 8 in
      handshake-pair cli srv cli-addr srv-addr buf (wrapping-add-u64 t1 5000000) (wrapping-sub-u32 rounds 1)

  -- Test A: Strict 3x anti-amplification before address validation
  test-anti-amplification :: I64 ->{IO, MVar} Int
  test-anti-amplification buf =
    let srv-scid = quic-cid-encode-core (core-id 1) 101 0 8 in
    let cli-scid = quic-cid-encode-core (core-id 0) 202 0 8 in
    let odcid = quic-cid-encode-core (core-id 0) 303 0 8 in
    let cli-addr = (0x01001111 :: U64) in
    let srv-addr = (0x02002222 :: U64) in
    let cli = quic-conn-new-client cli-scid odcid srv-addr 1200 1000 11 in
    let srv = quic-conn-new-server srv-scid cli-scid odcid cli-addr 1200 1000 22 in
    if not (quic-conn-is-valid cli) || not (quic-conn-is-valid srv) then
      let _0 = quic-conn-destroy cli in
      let _1 = quic-conn-destroy srv in
      19
    else
      -- Feed a tiny 30-byte unauthenticated probe to server -> budget = 90 bytes
      let _z = fill-pat buf 0 30 7 in
      let _r0 = quic-conn-on-datagram srv buf 30 cli-addr 2000 in
      let rx0 = quic-conn-bytes-recv-unvalidated srv in
      -- Now feed a real 1200-byte Client Initial -> budget = 3 * 1230 = 3690 bytes
      let p0 = quic-conn-poll-tx cli buf 2048 3000 in
      let ok-init = ne-u8 p0.tx-ok 0 && ge-u32 p0.tx-len 1200 in
      let _r1 = quic-conn-on-datagram srv buf p0.tx-len cli-addr 4000 in
      let _sc = quic-conn-step-crypto srv 4000 in
      let t1 = quic-conn-poll-tx srv buf 2048 5000 in
      let t2 = quic-conn-poll-tx srv buf 2048 6000 in
      let sent-u = quic-conn-bytes-sent-unvalidated srv in
      let recv-u = quic-conn-bytes-recv-unvalidated srv in
      let amp-ok = le-u64 sent-u (wrapping-mul-u64 recv-u 3) in
      -- Verify explicit 3x anti-amplification block on unvalidated server:
      let cli2 = quic-conn-new-client cli-scid odcid srv-addr 1200 1000 31 in
      let srv2 = quic-conn-new-server srv-scid cli-scid odcid cli-addr 1200 1000 33 in
      let blk-ok = (if not (quic-conn-is-valid cli2) || not (quic-conn-is-valid srv2) then False
        else
          let p-small = quic-conn-poll-tx-capped cli2 buf 2048 256 2000 in
          let _r2 = quic-conn-on-datagram srv2 buf p-small.tx-len cli-addr 3000 in
          let _sc2 = quic-conn-step-crypto srv2 3000 in
          let _s1 = quic-conn-poll-tx srv2 buf 2048 4000 in
          let _s2 = quic-conn-poll-tx srv2 buf 2048 5000 in
          let _to1 = quic-conn-on-timeout srv2 2000000000 in
          let _s3 = quic-conn-poll-tx srv2 buf 2048 2000001000 in
          let _s4 = quic-conn-poll-tx srv2 buf 2048 2000002000 in
          let _to2 = quic-conn-on-timeout srv2 6000000000 in
          let _s5 = quic-conn-poll-tx srv2 buf 2048 6000001000 in
          let _s6 = quic-conn-poll-tx srv2 buf 2048 6000002000 in
          let _to3 = quic-conn-on-timeout srv2 16000000000 in
          let _s7 = quic-conn-poll-tx srv2 buf 2048 16000001000 in
          let _s8 = quic-conn-poll-tx srv2 buf 2048 16000002000 in
          let blks = quic-conn-amp-blocks srv2 in
          let su2 = quic-conn-bytes-sent-unvalidated srv2 in
          let ru2 = quic-conn-bytes-recv-unvalidated srv2 in
          gt-u64 blks 0 && le-u64 su2 (wrapping-mul-u64 ru2 3)) in
      let _d1 = quic-conn-destroy cli2 in
      let _d2 = quic-conn-destroy srv2 in
      let _dc = quic-conn-destroy cli in
      let _ds = quic-conn-destroy srv in
      if not ok-init then 10
      else if ne-u64 rx0 30 then 11
      else if not amp-ok then 12
      else if eq-u8 t1.tx-ok 0 || eq-u8 t2.tx-ok 0 then 13
      else if not blk-ok then 14
      else 0

  deliver-gso-segments :: QuicConn -> I64 -> U32 -> U32 -> U32 -> U64 -> U64 ->{MVar} U32
  deliver-gso-segments srv buf off total seg-sz src-addr now-ns =
    if ge-u32 off total then 0
    else
      let rem = wrapping-sub-u32 total off in
      let chunk = if lt-u32 rem seg-sz then rem else seg-sz in
      let _rx = quic-conn-on-datagram srv (wrapping-add-i64 buf (u32-to-i64 off)) chunk src-addr now-ns in
      wrapping-add-u32 1 (deliver-gso-segments srv buf (wrapping-add-u32 off chunk) total seg-sz src-addr (wrapping-add-u64 now-ns 1000))

  -- Test C: 0-RTT early data + AntiReplayFilter acceptance & replay rejection
  test-zero-rtt-anti-replay :: I64 ->{IO, MVar} Int
  test-zero-rtt-anti-replay dma =
    let pkt-buf = dma in
    let data-buf = wrapping-add-i64 dma 4096 in
    let rx-buf = wrapping-add-i64 dma 8192 in
    let ar = anti-replay-new 64 10000000000 in
    let zsec = bytes-from-u8s [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32] in
    let ticket-nonce = (0xfeedfacecafebeef :: U64) in
    let c1-scid = quic-cid-encode-core (core-id 0) 401 0 8 in
    let s1-scid = quic-cid-encode-core (core-id 1) 402 0 8 in
    let od1 = quic-cid-encode-core (core-id 0) 403 0 8 in
    let cli-addr = (0x01003001 :: U64) in
    let srv-addr = (0x02003002 :: U64) in
    let c1 = quic-conn-new-client c1-scid od1 srv-addr 1200 1000 1 in
    let s1 = quic-conn-new-server s1-scid c1-scid od1 cli-addr 1200 1000 2 in
    if not (quic-conn-is-valid c1) || not (quic-conn-is-valid s1) then
      let _0 = quic-conn-destroy c1 in
      let _1 = quic-conn-destroy s1 in
      let _dar = anti-replay-destroy ar in
      58
    else
      let _ar1 = quic-conn-set-anti-replay s1 ar in
      let _zk1 = quic-conn-set-zero-rtt-key c1 zsec ticket-nonce in
      let _zk2 = quic-conn-set-zero-rtt-key s1 zsec ticket-nonce in
      let _fp = fill-pat data-buf 0 32 77 in
      let _dg = quic-conn-send-datagram c1 data-buf 32 in
      let p-init = quic-conn-poll-tx c1 pkt-buf 2048 2000 in
      let _rx-i = quic-conn-on-datagram s1 pkt-buf p-init.tx-len cli-addr 3000 in
      let _sc-s = quic-conn-step-crypto s1 3000 in
      let p-zrtt = quic-conn-poll-tx c1 pkt-buf 2048 4000 in
      let _rx-z = quic-conn-on-datagram s1 pkt-buf p-zrtt.tx-len cli-addr 5000 in
      let acc1 = quic-conn-zero-rtt-accepted s1 in
      let dg1 = quic-conn-recv-datagram s1 rx-buf 64 in
      let ok1 = acc1 && eq-u32 dg1 32 && verify-pat rx-buf 0 32 77 in
      let _d1 = quic-conn-destroy c1 in
      let _d2 = quic-conn-destroy s1 in
      let c2 = quic-conn-new-client c1-scid od1 srv-addr 1200 6000 3 in
      let s2 = quic-conn-new-server s1-scid c1-scid od1 cli-addr 1200 6000 4 in
      if not (quic-conn-is-valid c2) || not (quic-conn-is-valid s2) then
        let _0 = quic-conn-destroy c2 in
        let _1 = quic-conn-destroy s2 in
        let _dar = anti-replay-destroy ar in
        59
      else
        let _ar2 = quic-conn-set-anti-replay s2 ar in
        let _zk3 = quic-conn-set-zero-rtt-key c2 zsec ticket-nonce in
        let _zk4 = quic-conn-set-zero-rtt-key s2 zsec ticket-nonce in
        let _dg2 = quic-conn-send-datagram c2 data-buf 32 in
        let p-i2 = quic-conn-poll-tx c2 pkt-buf 2048 7000 in
        let _rx-i2 = quic-conn-on-datagram s2 pkt-buf p-i2.tx-len cli-addr 8000 in
        let _sc-s2 = quic-conn-step-crypto s2 8000 in
        let p-z2 = quic-conn-poll-tx c2 pkt-buf 2048 9000 in
        let _rx-z2 = quic-conn-on-datagram s2 pkt-buf p-z2.tx-len cli-addr 10000 in
        let rej2 = quic-conn-zero-rtt-rejected s2 && not (quic-conn-zero-rtt-accepted s2) in
        let dg2 = quic-conn-recv-datagram s2 rx-buf 64 in
        let ar-rej = anti-replay-rejects ar in
        let _d3 = quic-conn-destroy c2 in
        let _d4 = quic-conn-destroy s2 in
        let _dar = anti-replay-destroy ar in
        if not ok1 then 51
        else if not rej2 then 52
        else if ne-u32 dg2 0 then 53
        else if eq-u64 ar-rej 0 then 54
        else 0

  -- Test B2: Dedicated clean test of full protocol returning exact error code
  test-full-protocol-checked :: I64 ->{IO, MVar} Int
  test-full-protocol-checked dma =
    let pkt-buf = dma in
    let retry-buf = wrapping-add-i64 dma 4096 in
    let gso-buf = wrapping-add-i64 dma 8192 in
    let data-buf = wrapping-add-i64 dma 32768 in
    let rx-buf = wrapping-add-i64 dma 49152 in
    let cli-addr = (0x01001001 :: U64) in
    let mig-addr = (0x01002002 :: U64) in
    let srv-addr = (0x02004433 :: U64) in
    let cli-ep = quic-endpoint-new (core-id 0) 8 1200 0x11223344 0xdeadbeef in
    let srv-ep = quic-endpoint-new (core-id 1) 8 1200 0x55667788 0xcafebabe in
    if not (quic-endpoint-is-valid cli-ep) || not (quic-endpoint-is-valid srv-ep) then
      let _dc = quic-endpoint-destroy cli-ep in
      let _ds = quic-endpoint-destroy srv-ep in
      49
    else
      let rc = run-full-protocol cli-ep srv-ep pkt-buf retry-buf gso-buf data-buf rx-buf cli-addr mig-addr srv-addr in
      let _dc = quic-endpoint-destroy cli-ep in
      let _ds = quic-endpoint-destroy srv-ep in
      rc

  run-full-protocol :: QuicEndpoint -> QuicEndpoint -> I64 -> I64 -> I64 -> I64 -> I64 -> U64 -> U64 -> U64 ->{IO, MVar} Int
  run-full-protocol cli-ep srv-ep pkt-buf retry-buf gso-buf data-buf rx-buf cli-addr mig-addr srv-addr =
    let _req = quic-endpoint-set-require-retry srv-ep 1 in
    let cslot = quic-endpoint-connect cli-ep srv-addr 10000 777 in
    if eq-u32 cslot ep-slot-none then 20
    else
      let cli = quic-endpoint-conn-at cli-ep cslot in
      let p1 = quic-conn-poll-tx cli pkt-buf 2048 11000 in
      if eq-u8 p1.tx-ok 0 || lt-u32 p1.tx-len 1200 then 21
      else
        let r1 = quic-endpoint-on-datagram srv-ep pkt-buf p1.tx-len cli-addr 12000 retry-buf 1024 in
        if ne-u8 r1.eprx-status ep-rx-retry-sent || eq-u32 r1.eprx-retry-len 0 || ne-u32 (quic-endpoint-active-count srv-ep) 0 then 22
        else if ne-u64 (quic-endpoint-retries-sent srv-ep) 1 then 23
        else
          let cr1 = quic-conn-on-datagram cli retry-buf r1.eprx-retry-len srv-addr 13000 in
          if ne-u8 cr1 quic-rx-retry then 24
          else
            let _sc1 = quic-conn-step-crypto cli 13000 in
            let p2 = quic-conn-poll-tx cli pkt-buf 2048 14000 in
            if eq-u8 p2.tx-ok 0 || lt-u32 p2.tx-len 1200 then 25
            else
              let r2 = quic-endpoint-on-datagram srv-ep pkt-buf p2.tx-len cli-addr 15000 retry-buf 1024 in
              if ne-u8 r2.eprx-status ep-rx-new-conn || eq-u32 r2.eprx-slot ep-slot-none then 26
              else
                let srv = quic-endpoint-conn-at srv-ep r2.eprx-slot in
                if not (quic-conn-addr-validated srv) then 27
                else
                  let t-est = handshake-pair cli srv cli-addr srv-addr pkt-buf 16000 12 in
                  if not (quic-conn-is-established cli && quic-conn-is-established srv && quic-conn-handshake-confirmed cli && quic-conn-handshake-confirmed srv) then 28
                  else
                    -- RFC 9221 DATAGRAM
                    let _d0 = fill-pat data-buf 0 64 91 in
                    let _ds1 = quic-conn-send-datagram cli data-buf 64 in
                    let _pm1 = pump-conn-dir cli srv cli-addr pkt-buf t-est 4 in
                    let dg-got = quic-conn-recv-datagram srv rx-buf 128 in
                    if ne-u32 dg-got 64 || not (verify-pat rx-buf 0 64 91) then 29
                    else
                      -- 1-RTT KeyUpdate (0 -> 1 -> 0)
                      let ku-ok1 = quic-conn-initiate-key-update cli in
                      if not ku-ok1 || ne-u8 (quic-conn-tx-key-phase cli) 1 then 30
                      else
                        let _d1 = fill-pat data-buf 0 48 123 in
                        let _ds2 = quic-conn-send-datagram cli data-buf 48 in
                        let _pm2 = pump-conn-dir cli srv cli-addr pkt-buf (wrapping-add-u64 t-est 5000000) 4 in
                        let dg2 = quic-conn-recv-datagram srv rx-buf 128 in
                        if ne-u32 dg2 48 || not (verify-pat rx-buf 0 48 123) then 31
                        else if ne-u8 (quic-conn-rx-key-phase srv) 1 || ne-u8 (quic-conn-tx-key-phase srv) 1 then 32
                        else
                          let _d2 = fill-pat data-buf 0 32 55 in
                          let _ds3 = quic-conn-send-datagram srv data-buf 32 in
                          let _pm3 = pump-conn-dir srv cli srv-addr pkt-buf (wrapping-add-u64 t-est 10000000) 4 in
                          let dg3 = quic-conn-recv-datagram cli rx-buf 128 in
                          if ne-u32 dg3 32 || not (verify-pat rx-buf 0 32 55) then 33
                          else
                            let _rf1 = quic-conn-key-update-refresh cli in
                            let _rf2 = quic-conn-key-update-refresh srv in
                            let ku-ok2 = quic-conn-initiate-key-update cli in
                            let _ds4 = quic-conn-send-datagram cli data-buf 32 in
                            let _pm4 = pump-conn-dir cli srv cli-addr pkt-buf (wrapping-add-u64 t-est 15000000) 4 in
                            let dg4 = quic-conn-recv-datagram srv rx-buf 128 in
                            if not ku-ok2 || ne-u8 (quic-conn-tx-key-phase cli) 0 || ne-u8 (quic-conn-rx-key-phase srv) 0 || ne-u32 dg4 32 then 34
                            else
                              -- UDP_GSO Burst Packing
                              let sslot = quic-conn-open-stream cli 0 in
                              let _fp = fill-pat data-buf 0 6144 41 in
                              let wr = quic-conn-stream-write cli sslot data-buf 6144 1 in
                              let t-gso = wrapping-add-u64 t-est 25000000 in
                              let gso = quic-endpoint-poll-gso-burst cli-ep gso-buf 16384 8 t-gso in
                              if ne-u32 wr 6144 || lt-u32 gso.gso-pkt-count 5 || ne-u32 gso.gso-seg-size 1200 then 35
                              else
                                let _del = deliver-gso-segments srv gso-buf 0 gso.gso-total-len gso.gso-seg-size cli-addr t-gso in
                                let _tail = pump-conn-dir cli srv cli-addr pkt-buf (wrapping-add-u64 t-gso 5000000) 4 in
                                let srv-s = quic-conn-stream-find srv (StreamId 0) in
                                let rd = quic-conn-stream-read srv srv-s rx-buf 8192 in
                                if ne-u32 rd 6144 || not (verify-pat rx-buf 0 6144 41) || not (quic-conn-stream-fin-reached srv srv-s) then 36
                                else
                                  -- NAT Connection Migration (PATH_CHALLENGE / PATH_RESPONSE)
                                  let _ak = pump-conn-dir srv cli srv-addr pkt-buf (wrapping-add-u64 t-gso 10000000) 4 in
                                  let _ds5 = quic-conn-send-datagram cli data-buf 24 in
                                  let t-mig = wrapping-add-u64 t-gso 20000000 in
                                  let pmig = quic-conn-poll-tx cli pkt-buf 2048 t-mig in
                                  let _rxm = quic-conn-on-datagram srv pkt-buf pmig.tx-len mig-addr (wrapping-add-u64 t-mig 1000000) in
                                  if not (quic-conn-is-path-validating srv) || ne-u64 (quic-conn-active-peer-addr srv) cli-addr then 37
                                  else
                                    let pch = quic-conn-poll-tx srv pkt-buf 2048 (wrapping-add-u64 t-mig 2000000) in
                                    if eq-u8 pch.tx-ok 0 || ne-u64 pch.tx-peer-addr mig-addr then 38
                                    else
                                      let _rxc = quic-conn-on-datagram cli pkt-buf pch.tx-len srv-addr (wrapping-add-u64 t-mig 3000000) in
                                      let prs = quic-conn-poll-tx cli pkt-buf 2048 (wrapping-add-u64 t-mig 4000000) in
                                      if eq-u8 prs.tx-ok 0 then 39
                                      else
                                        let _rxv = quic-conn-on-datagram srv pkt-buf prs.tx-len mig-addr (wrapping-add-u64 t-mig 5000000) in
                                        if quic-conn-is-path-validating srv then 40
                                        else if ne-u64 (quic-conn-active-peer-addr srv) mig-addr then 41
                                        else if ne-u64 (quic-conn-migration-count srv) 1 then 42
                                        else 0

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 65536 4096 in
    if eq-u64 dma 0 then 99
    else
      let base = wrapping-u64-to-i64 dma in
      let r1 = test-anti-amplification base in
      let r2 = if r1 == 0 then test-full-protocol-checked base else r1 in
      let r3 = if r2 == 0 then test-zero-rtt-anti-replay base else r2 in
      let _ = engine-dma-free dma 65536 in
      r3
"#;
    build_and_run_leak_checked("protocol-suite", src, 0);
}

// =========================================================================
// 4. 1,000-Seed Deterministic Chaos Simulation (5% drop + 10% reorder +
//    5% duplicate + mid-transfer NAT IP/port migration) over Engine.Sim
// =========================================================================

#[test]
fn quic_1000_seed_deterministic_chaos_simulation() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Text.Show
  import Engine.Types
  import Engine.Sim
  import Engine.Sim.Faults
  import Net.Quic.Types
  import Net.Quic.Stream
  import Net.Quic.Conn
  import Net.Quic.Endpoint
  import Prim.Atomic

  fill-seed-pat :: I64 -> U32 -> U32 -> U8 ->{MVar} Unit
  fill-seed-pat addr i n seed =
    if ge-u32 i n then ()
    else
      let v = wrapping-add-u8 (wrapping-mul-u8 (wrapping-u32-to-u8 i) 53) seed in
      let _ = raw-store-u8 (wrapping-add-i64 addr (u32-to-i64 i)) v in
      fill-seed-pat addr (wrapping-add-u32 i 1) n seed

  verify-seed-pat :: I64 -> U32 -> U32 -> U8 ->{MVar} Bool
  verify-seed-pat addr i n seed =
    if ge-u32 i n then True
    else
      let got = raw-load-u8 (wrapping-add-i64 addr (u32-to-i64 i)) in
      let exp = wrapping-add-u8 (wrapping-mul-u8 (wrapping-u32-to-u8 i) 53) seed in
      if eq-u8 got exp then verify-seed-pat addr (wrapping-add-u32 i 1) n seed
      else False

  chaos-faults :: SimFaultConfig
  chaos-faults = SimFaultConfig {
    udp-drop-ppm = 50000,
    udp-dup-ppm = 50000,
    udp-reorder-ppm = 100000,
    udp-corrupt-ppm = 0,
    udp-min-delay-ns = 1000000,
    udp-max-delay-ns = 5000000,
    tcp-partial-io-ppm = 0,
    tcp-rst-ppm = 0,
    disk-write-delay-ns = 0,
    disk-torn-sector-ppm = 0,
    disk-bitflip-ppm = 0,
    partition-mask = 0
  }

  -- Step loop for one chaos seed: handshake -> write 4096B stream -> mid-transfer NAT rebind -> verify exact delivery & migration.
  chaos-tick-loop :: forall (r :: Region). SimWorld r -> Fd -> Fd -> QuicEndpoint -> QuicEndpoint -> U32 -> I64 -> I64 -> I64 -> U8 -> U8 -> U32 -> U32 -> U32 ->{IO, IO r, Mut r, MVar} Int
  chaos-tick-loop w cfd sfd cep sep cslot scratch tx-data rx-data sent-done migrated srv-rx-off ticks seed =
    if eq-u32 ticks 0 then
      let cli = quic-endpoint-conn-at cep cslot in
      let srv-act = quic-endpoint-active-count sep in
      let srv = if gt-u32 srv-act 0 then quic-endpoint-conn-at sep 0 else quic-conn-empty () in
      let line = string-concat "FAIL seed=" (string-concat (show (u32-to-nat seed))
        (string-concat " now-ms=" (string-concat (show (u32-to-nat (wrapping-u64-to-u32 (div-u64 (sim-world-now-ns w) 1000000))))
        (string-concat " cli-st=" (string-concat (show (u8-to-nat (quic-conn-state cli)))
        (string-concat " cli-tx=" (string-concat (show (u32-to-nat (wrapping-u64-to-u32 (quic-conn-pkts-tx cli))))
        (string-concat " cli-rx=" (string-concat (show (u32-to-nat (wrapping-u64-to-u32 (quic-conn-pkts-rx cli))))
        (string-concat " srv-st=" (string-concat (if gt-u32 srv-act 0 then show (u8-to-nat (quic-conn-state srv)) else "none")
        (string-concat " srv-tx=" (string-concat (if gt-u32 srv-act 0 then show (u32-to-nat (wrapping-u64-to-u32 (quic-conn-pkts-tx srv))) else "0")
        (string-concat " srv-rx=" (string-concat (if gt-u32 srv-act 0 then show (u32-to-nat (wrapping-u64-to-u32 (quic-conn-pkts-rx srv))) else "0")
        (string-concat " rx-off=" (string-concat (show (u32-to-nat srv-rx-off)) "\n"))))))))))))))))) in
      let _p = (with (io = newIO ()) in ioWrite (string-to-bytes line)) in
      if not (quic-conn-is-established cli) then 213
      else if eq-u32 srv-act 0 then 216
      else if lt-u32 srv-rx-off 4096 then 217
      else
        let srv = quic-endpoint-conn-at sep 0 in
        let sslot = quic-conn-stream-find srv (StreamId 0) in
        if eq-u32 sslot stream-slot-none || not (quic-conn-stream-fin-reached srv sslot) then 218
        else if eq-u64 (quic-conn-migration-count srv) 0 then 219
        else 220
    else
      let _st1 = quic-endpoint-step w cfd cep scratch 16 in
      let _st2 = quic-endpoint-step w sfd sep (wrapping-add-i64 scratch 8192) 16 in
      let cli = quic-endpoint-conn-at cep cslot in
      let next-sent = (if eq-u8 sent-done 0 && quic-conn-is-established cli then
        let st = quic-conn-open-stream cli 0 in
        let _wr = quic-conn-stream-write cli st tx-data 4096 1 in
        (1 :: U8)
      else sent-done) in
      -- Trigger mid-transfer NAT port rebinding once stream is sent and >= 1 KiB received (or halfway through ticks)
      let next-mig = (if eq-u8 migrated 0 && ne-u8 next-sent 0 && (ge-u32 srv-rx-off 1024 || le-u32 ticks 400) then
        let new-port = wrapping-add-u16 20000 (wrapping-u32-to-u16 (bit-and-u32 seed 8191)) in
        let _rb = sim-udp-nat-rebind w cfd new-port in
        (1 :: U8)
      else migrated) in
      -- Read any newly reassembled stream bytes on server
      let srv-active = quic-endpoint-active-count sep in
      let next-rx-off = (if gt-u32 srv-active 0 then
        let srv = quic-endpoint-conn-at sep 0 in
        let sslot = quic-conn-stream-find srv (StreamId 0) in
        if ne-u32 sslot stream-slot-none then
          let rem = saturating-sub-u32 4096 srv-rx-off in
          if gt-u32 rem 0 then
            let got = quic-conn-stream-read srv sslot (wrapping-add-i64 rx-data (u32-to-i64 srv-rx-off)) rem in
            wrapping-add-u32 srv-rx-off got
          else srv-rx-off
        else srv-rx-off
      else srv-rx-off) in
      -- Check completion: all 4096 bytes + FIN delivered AND NAT migration validated!
      let done = (if gt-u32 srv-active 0 && eq-u32 next-rx-off 4096 && ne-u8 next-mig 0 then
        let srv = quic-endpoint-conn-at sep 0 in
        let sslot = quic-conn-stream-find srv (StreamId 0) in
        ne-u32 sslot stream-slot-none && quic-conn-stream-fin-reached srv sslot && ge-u64 (quic-conn-migration-count srv) 1
      else False) in
      if done then 0
      else
        -- If stream already delivered after NAT rebind but migration still waiting for a packet from new port, nudge with a small DATAGRAM
        let _nudge = (if ne-u8 next-mig 0 && eq-u32 next-rx-off 4096 && eq-u32 (bit-and-u32 ticks 3) 0 then
          let _d = quic-conn-send-datagram cli tx-data 8 in
          ()
        else ()) in
        -- Advance virtual simulation clock: 15ms step when packets are active, or bounded jump toward earliest RFC 9002 timer when quiescent
        let step-ns = (if eq-u32 _st1 0 && eq-u32 _st2 0 && eq-u64 (sim-world-pending-events w) 0 then
          if ne-u8 next-mig 0 && eq-u32 next-rx-off 4096 then (50000000 :: U64)
          else
            let now = sim-world-now-ns w in
            let t-cli = quic-conn-next-timeout-ns cli in
            let t-srv = if gt-u32 srv-active 0 then quic-conn-next-timeout-ns (quic-endpoint-conn-at sep 0) else 0 in
            let tgt-ns = (if eq-u64 t-cli 0 then t-srv
              else if eq-u64 t-srv 0 then t-cli
              else if lt-u64 t-cli t-srv then t-cli else t-srv) in
            let diff = if gt-u64 tgt-ns now then saturating-sub-u64 tgt-ns now else (15000000 :: U64) in
            if gt-u64 diff 500000000 then (500000000 :: U64) else diff
        else (15000000 :: U64)) in
        let _adv = sim-world-run-for w step-ns in
        chaos-tick-loop w cfd sfd cep sep cslot scratch tx-data rx-data next-sent next-mig next-rx-off (wrapping-sub-u32 ticks 1) seed

  run-one-chaos-seed :: forall (r :: Region). U32 -> I64 ->{IO, IO r, Mut r, MVar} Int
  run-one-chaos-seed seed dma =
    let scratch = dma in
    let tx-data = wrapping-add-i64 dma 16384 in
    let rx-data = wrapping-add-i64 dma 24576 in
    let s8 = wrapping-u32-to-u8 (wrapping-add-u32 seed 17) in
    let _fp = fill-seed-pat tx-data 0 4096 s8 in
    let seed64 = u32-to-u64 seed in
    let w = sim-world-new (wrapping-mul-u64 seed64 0x9e3779b97f4a7c15) chaos-faults in
    case sim-udp-bind w 1 10000 of
      Err _ -> 103
      Ok cfd -> case sim-udp-bind w 2 4433 of
        Err _ ->
          let _fc = sim-udp-close w cfd in
          103
        Ok sfd ->
          let cep = quic-endpoint-new (core-id 0) 4 1200 seed64 0x1234 in
          let sep = quic-endpoint-new (core-id 1) 4 1200 (wrapping-add-u64 seed64 1) 0x5678 in
          if not (quic-endpoint-is-valid cep) || not (quic-endpoint-is-valid sep) then
            let _dc = quic-endpoint-destroy cep in
            let _ds = quic-endpoint-destroy sep in
            let _fc = sim-udp-close w cfd in
            let _fs = sim-udp-close w sfd in
            102
          else
            let srv-addr = u32-to-u64 (sim-udp-addr-pack (sim-udp-addr-make 2 4433)) in
            let cslot = quic-endpoint-connect cep srv-addr 0 (wrapping-add-u64 seed64 99) in
            let rc = (if eq-u32 cslot ep-slot-none then 101
              else chaos-tick-loop w cfd sfd cep sep cslot scratch tx-data rx-data 0 0 0 600 seed) in
            let ok-bytes = if rc == 0 then verify-seed-pat rx-data 0 4096 s8 else False in
            let _dc = quic-endpoint-destroy cep in
            let _ds = quic-endpoint-destroy sep in
            let _fc = sim-udp-close w cfd in
            let _fs = sim-udp-close w sfd in
            if rc /= 0 then rc
            else if not ok-bytes then 201
            else 0

  run-all-seeds :: forall (r :: Region). U32 -> U32 -> I64 ->{IO, IO r, Mut r, MVar} Int
  run-all-seeds cur total dma =
    if gt-u32 cur total then 0
    else
      let rc = run-one-chaos-seed cur dma in
      if rc /= 0 then rc
      else run-all-seeds (wrapping-add-u32 cur 1) total dma

  main :: forall (r :: Region). Unit ->{IO, IO r, Mut r, MVar} Int
  main _ =
    let dma = engine-dma-alloc 32768 4096 in
    if eq-u64 dma 0 then 99
    else
      let rc = run-all-seeds 1 1000 (wrapping-u64-to-i64 dma) in
      let _ = engine-dma-free dma 32768 in
      rc
"#;
    build_and_run_leak_checked("chaos-1000-seeds", src, 0);
}

// =========================================================================
// 5. Steady-State 20,000 1-RTT Send/Recv Stream Packet Cycles:
//    0 heap allocs, 0 atomic RC ops, CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1
// =========================================================================

#[test]
fn quic_steady_state_20000_cycles_zero_alloc_zero_atomic_rc() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Data.Option
  import Engine.Types
  import Net.Quic.Types
  import Net.Quic.Stream
  import Net.Quic.Conn
  import Prim.Atomic

  #[no-alloc]
  fill-chunk :: I64 -> U32 -> U32 -> U8 ->{MVar} Unit
  fill-chunk addr i n seed =
    if ge-u32 i n then ()
    else
      let _ = raw-store-u8 (wrapping-add-i64 addr (u32-to-i64 i)) (wrapping-add-u8 (wrapping-u32-to-u8 i) seed) in
      fill-chunk addr (wrapping-add-u32 i 1) n seed

  #[no-alloc]
  check-chunk :: I64 -> U32 -> U32 -> U8 ->{MVar} Bool
  check-chunk addr i n seed =
    if ge-u32 i n then eq-u32 0 0
    else
      let got = raw-load-u8 (wrapping-add-i64 addr (u32-to-i64 i)) in
      let exp = wrapping-add-u8 (wrapping-u32-to-u8 i) seed in
      if eq-u8 got exp then check-chunk addr (wrapping-add-u32 i 1) n seed
      else eq-u32 0 1

  pump-hs-dir :: QuicConn -> QuicConn -> U64 -> I64 -> U64 -> U32 ->{IO, MVar} U32
  pump-hs-dir src dst src-addr buf now-ns lim =
    if eq-u32 lim 0 then 0
    else
      let _s1 = if quic-conn-needs-crypto-step src then let _ = quic-conn-step-crypto src now-ns in () else () in
      let p = quic-conn-poll-tx src buf 2048 now-ns in
      if eq-u8 p.tx-ok 0 then 0
      else
        let _rx = quic-conn-on-datagram dst buf p.tx-len src-addr (wrapping-add-u64 now-ns 1000000) in
        let _s2 = if quic-conn-needs-crypto-step dst then let _ = quic-conn-step-crypto dst (wrapping-add-u64 now-ns 1000000) in () else () in
        wrapping-add-u32 1 (pump-hs-dir src dst src-addr buf (wrapping-add-u64 now-ns 100000) (wrapping-sub-u32 lim 1))

  complete-hs :: QuicConn -> QuicConn -> U64 -> U64 -> I64 -> U64 -> U32 ->{IO, MVar} U64
  complete-hs cli srv cli-addr srv-addr buf now-ns rounds =
    if eq-u32 rounds 0 then now-ns
    else if quic-conn-is-established cli && quic-conn-is-established srv && quic-conn-handshake-confirmed cli && quic-conn-handshake-confirmed srv then now-ns
    else
      let _a = pump-hs-dir cli srv cli-addr buf now-ns 8 in
      let t1 = wrapping-add-u64 now-ns 5000000 in
      let _b = pump-hs-dir srv cli srv-addr buf t1 8 in
      complete-hs cli srv cli-addr srv-addr buf (wrapping-add-u64 t1 5000000) (wrapping-sub-u32 rounds 1)

  -- Steady-state #[no-alloc] hot loop (kept to 7 scalar/handle args for O(1) stack tailcc).
  -- Each cycle: client writes 256B -> poll-tx -> server on-datagram -> server stream-read ->
  -- verify bytes -> server poll-tx (ACK + MAX_DATA/MAX_STREAM_DATA) -> client on-datagram.
  #[no-alloc]
  hot-steady-loop :: QuicConn -> QuicConn -> U32 -> U32 -> I64 -> U32 -> U64 ->{MVar} U32
  hot-steady-loop cli srv cslot sslot dma rem now-ns =
    if eq-u32 rem 0 then 0
    else
      let pkt-buf = dma in
      let tx-buf = wrapping-add-i64 dma 2048 in
      let rx-buf = wrapping-add-i64 dma 4096 in
      let seed = wrapping-u32-to-u8 rem in
      let _f = fill-chunk tx-buf 0 256 seed in
      let wr = quic-conn-stream-write cli cslot tx-buf 256 0 in
      if ne-u32 wr 256 then 10
      else
        let p = quic-conn-poll-tx cli pkt-buf 2048 now-ns in
        if eq-u8 p.tx-ok 0 then 11
        else
          let rx-st = quic-conn-on-datagram srv pkt-buf p.tx-len 0x01001111 (wrapping-add-u64 now-ns 50000) in
          if ne-u8 rx-st quic-rx-ok then 12
          else
            let rd = quic-conn-stream-read srv sslot rx-buf 256 in
            if ne-u32 rd 256 then 13
            else if check-chunk rx-buf 0 256 seed then
              -- Flush ACK + flow control updates back to client so send ring & SentPacketRing recycle continuously
              let a1 = quic-conn-poll-tx srv pkt-buf 2048 (wrapping-add-u64 now-ns 100000) in
              let _r1 = (if ne-u8 a1.tx-ok 0 then
                let _ = quic-conn-on-datagram cli pkt-buf a1.tx-len 0x02002222 (wrapping-add-u64 now-ns 150000) in
                ()
              else ()) in
              let a2 = quic-conn-poll-tx srv pkt-buf 2048 (wrapping-add-u64 now-ns 160000) in
              let _r2 = (if ne-u8 a2.tx-ok 0 then
                let _ = quic-conn-on-datagram cli pkt-buf a2.tx-len 0x02002222 (wrapping-add-u64 now-ns 180000) in
                ()
              else ()) in
              hot-steady-loop cli srv cslot sslot dma (wrapping-sub-u32 rem 1) (wrapping-add-u64 now-ns 1000000)
            else 14

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 8192 4096 in
    if eq-u64 dma 0 then 99
    else
      let buf = wrapping-u64-to-i64 dma in
      let cscid = quic-cid-encode-core (core-id 0) 1 0 8 in
      let sscid = quic-cid-encode-core (core-id 1) 2 0 8 in
      let odcid = quic-cid-encode-core (core-id 0) 3 0 8 in
      let cli-addr = (0x01001111 :: U64) in
      let srv-addr = (0x02002222 :: U64) in
      let cli = quic-conn-new-client cscid odcid srv-addr 1200 1000 101 in
      let srv = quic-conn-new-server sscid cscid odcid cli-addr 1200 1000 202 in
      if not (quic-conn-is-valid cli) || not (quic-conn-is-valid srv) then
        let _dc = quic-conn-destroy cli in
        let _ds = quic-conn-destroy srv in
        let _df = engine-dma-free dma 8192 in
        98
      else
        let t-est = complete-hs cli srv cli-addr srv-addr buf 2000 12 in
        let cslot = quic-conn-open-stream cli 0 in
        -- Warm up 1 cycle so server opens remote stream slot 0 and caches active
        let _w0 = fill-chunk (wrapping-add-i64 buf 2048) 0 64 5 in
        let _wr0 = quic-conn-stream-write cli cslot (wrapping-add-i64 buf 2048) 64 0 in
        let p0 = quic-conn-poll-tx cli buf 2048 t-est in
        let _rx0 = quic-conn-on-datagram srv buf p0.tx-len cli-addr (wrapping-add-u64 t-est 50000) in
        let sslot = quic-conn-stream-find srv (StreamId 0) in
        let _rd0 = quic-conn-stream-read srv sslot (wrapping-add-i64 buf 4096) 64 in
        let a0 = quic-conn-poll-tx srv buf 2048 (wrapping-add-u64 t-est 100000) in
        let _ra0 = if ne-u8 a0.tx-ok 0 then let _ = quic-conn-on-datagram cli buf a0.tx-len srv-addr (wrapping-add-u64 t-est 150000) in () else () in
        -- Reset thread alloc & atomic-RC counters and run 20,000 steady-state 1-RTT stream cycles!
        let _rst = engine-thread-counters-reset () in
        let loop-rc = hot-steady-loop cli srv cslot sslot buf 20000 (wrapping-add-u64 t-est 1000000) in
        let allocs = engine-thread-alloc-count () in
        let atomic-rcs = engine-thread-atomic-rc-count () in
        let _dc = quic-conn-destroy cli in
        let _ds = quic-conn-destroy srv in
        let _df = engine-dma-free dma 8192 in
        if eq-u32 loop-rc 10 then 10
        else if eq-u32 loop-rc 11 then 11
        else if eq-u32 loop-rc 12 then 12
        else if eq-u32 loop-rc 13 then 13
        else if eq-u32 loop-rc 14 then 14
        else if ne-u32 loop-rc 0 then 15
        else if ne-u64 allocs 0 then 20
        else if ne-u64 atomic-rcs 0 then 21
        else 0
"#;
    build_and_run_leak_checked("hot-20k-zero-alloc", src, 0);
}
