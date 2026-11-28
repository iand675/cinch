//! Integration, formal-verification (M9 `Http3Machine`), RFC 9114 + RFC 9218 +
//! RFC 9297 HTTP/3 conformance suite, end-to-end client/server HTTP/3 over real
//! `Net.Quic.Conn` packets, and 25,000-cycle zero-allocation / zero-atomic-RC /
//! leak-checked tests for `Net.Http.V3.{Frame,Conn,Spec}`.
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
        "cinch-net-http-v3-{tag}-{}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn build_and_run_leak_checked(tag: &str, src: &str, expected_code: i32) -> (String, String) {
    let dir = fresh_dir(tag);
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let exe = dir.join(format!("http3-{tag}"));
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
        .env("CINCH_HEAP_LIMIT", "128M")
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
// 1. Manifest registration & #[no-alloc] safety check
// =========================================================================

#[test]
fn net_http_v3_manifest_registration_and_no_alloc_check() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, rel_path) in [
        ("Net.Http.V3.Frame", "Net/Http/V3/Frame.cinch"),
        ("Net.Http.V3.Conn", "Net/Http/V3/Conn.cinch"),
        ("Net.Http.V3.Spec", "Net/Http/V3/Spec.cinch"),
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

    let pos_varint = pkg.order.iter().position(|n| n == "Net.Quic.Varint").unwrap();
    let pos_qconn = pkg.order.iter().position(|n| n == "Net.Quic.Conn").unwrap();
    let pos_qpack = pkg.order.iter().position(|n| n == "Net.Http.Qpack.Codec").unwrap();
    let pos_prio = pkg.order.iter().position(|n| n == "Net.Http.Priority").unwrap();
    let pos_frame = pkg.order.iter().position(|n| n == "Net.Http.V3.Frame").unwrap();
    let pos_conn = pkg.order.iter().position(|n| n == "Net.Http.V3.Conn").unwrap();
    let pos_spec = pkg.order.iter().position(|n| n == "Net.Http.V3.Spec").unwrap();
    assert!(pos_varint < pos_frame);
    assert!(pos_frame < pos_conn && pos_qconn < pos_conn && pos_qpack < pos_conn && pos_prio < pos_conn);
    assert!(pos_conn < pos_spec);

    // Compile-time #[no-alloc] audit across all hot-path Frame & Conn entry points
    let dir = fresh_dir("no-alloc-check");
    std::fs::write(
        dir.join("Main.cinch"),
        r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Engine.Types
  import Net.Http.Types
  import Net.Http.Priority
  import Net.Http.Qpack.Codec
  import Net.Http.V3.Frame
  import Net.Http.V3.Conn
  import Net.Http.V3.Spec
  import Net.Quic.Conn

  #[no-alloc]
  audit-h3-frame-hot :: I64 -> I64 ->{MVar} U32
  audit-h3-frame-hot buf scratch =
    let _u = h3-uni-write-type-raw buf 0 64 h3-uni-control in
    let _ur = h3-uni-read-type-raw buf 0 64 in
    let _se = h3-settings-encode-raw buf 0 64 (h3-settings-default ()) in
    let hdr = h3-frame-decode-header-raw buf 0 64 in
    let _sd = h3-settings-decode-raw buf hdr.h3f-hdr-len 8 in
    let _ge = h3-frame-encode-goaway-raw buf 0 64 12 in
    let _gd = h3-frame-decode-goaway-raw buf 2 1 in
    let _pe = h3-frame-encode-priority-update-raw buf 0 64 4 scratch 3 in
    let _pd = h3-frame-decode-priority-update-raw buf 4 4 in
    let _as = h3-format-alt-svc-raw buf 0 64 443 86400 in
    let _de = h3-dgram-encode-raw buf 0 64 4 scratch 8 in
    let _dd = h3-dgram-decode-raw buf 0 _de in
    wrapping-add-u32 _se (wrapping-add-u32 _as _dd.h3dg-payload-len)

  #[no-alloc]
  audit-h3-conn-hot :: H3Conn -> QuicConn -> I64 -> I64 ->{MVar} U32
  audit-h3-conn-hot conn qconn buf scratch =
    let _ic = h3-conn-init-quic-streams conn qconn in
    let _fu = h3-conn-feed-uni conn 2 buf 16 0 in
    let _fr = h3-conn-feed-request conn 0 buf 16 1 in
    let _er = h3-encode-request-raw buf 0 256 method-code-get scratch 1 scratch 4 scratch 0 scratch 64 in
    let _es = h3-encode-response-raw buf 0 256 200 scratch 4 scratch 64 in
    let _qr = h3-conn-queue-response conn 0 200 scratch 4 in
    let fl = h3-conn-flush-priority-dma conn 4 buf 0 256 in
    let _pq = h3-conn-poll-quic conn qconn in
    let _sd = h3-conn-send-datagram conn qconn 0 scratch 8 in
    let _rd = h3-conn-recv-datagram conn qconn buf 64 in
    wrapping-add-u32 _fr.h3fr-consumed (wrapping-add-u32 fl.h3fl-written _rd.h3dg-payload-len)

  main :: Int
  main = if h3-safety-ok h3-model-init then 0 else 1
"#,
    )
    .expect("write Main.cinch");

    let ifaces = common::check_interfaces(&dir, "Main.cinch")
        .expect("Net.Http.V3.{Frame,Conn,Spec} #[no-alloc] hot-path audit must check cleanly");
    let main_iface = ifaces
        .iter()
        .find(|i| i.module == "Main")
        .expect("Main interface present");
    for fn_name in ["audit-h3-frame-hot", "audit-h3-conn-hot"] {
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
// 2. Formal Model M9 (`Http3Machine`): capped=false, 0 violations under
//    CINCH_MODEL_FAIL_ON_CAPPED=1 + refinement onto AbstractMultiplexedHttp
// =========================================================================

#[test]
fn formal_model_m9_http3_machine_verifies_uncapped_zero_violations() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("prelude/Net/Http/V3/Spec.cinch");
    let spec_str = spec_path.to_string_lossy().into_owned();
    let cinch_bin = env!("CARGO_BIN_EXE_cinch");

    // 1. Check via `common::drive_with_profile_opts` with `model_fail_on_capped = true`
    let dir = fresh_dir("m9-spec-check");
    std::fs::write(
        dir.join("Main.cinch"),
        r#"
module Main where
  import Net.Http.V3.Spec

  main :: Int
  main = if h3-safety-ok h3-model-init then 0 else 1
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
    .expect("Net.Http.V3.Spec must check cleanly with model_fail_on_capped=true");
    let _ = std::fs::remove_dir_all(&dir);

    // 2. CLI `cinch model inspect <Spec.cinch> Http3Machine --json` with CINCH_MODEL_FAIL_ON_CAPPED=1
    let inspect_out = Command::new(cinch_bin)
        .args(["model", "inspect", &spec_str, "Http3Machine", "--json"])
        .env("CINCH_MODEL_FAIL_ON_CAPPED", "1")
        .output()
        .expect("run cinch model inspect Http3Machine");
    let stdout = String::from_utf8_lossy(&inspect_out.stdout);
    let stderr = String::from_utf8_lossy(&inspect_out.stderr);
    assert!(
        inspect_out.status.success(),
        "M9 Http3Machine failed under CINCH_MODEL_FAIL_ON_CAPPED=1:\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    let json: serde_json::Value =
        serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("invalid JSON: {e}\n{stdout}"));
    let machines = json
        .as_array()
        .expect("top-level machines array in model inspect JSON");
    let m9 = machines
        .iter()
        .find(|m| m.get("name").and_then(|v| v.as_str()) == Some("Http3Machine"))
        .unwrap_or_else(|| panic!("Http3Machine missing from JSON:\n{stdout}"));
    let capped = m9.get("capped").and_then(|v| v.as_bool()).expect("capped bool");
    let states = m9.get("state_count").and_then(|v| v.as_u64()).expect("state_count");
    let edges = m9.get("edge_count").and_then(|v| v.as_u64()).expect("edge_count");
    assert!(!capped, "M9 Http3Machine MUST have capped=false; json:\n{stdout}");
    assert!(states >= 20 && states <= 8192, "unexpected M9 state count: {states}");

    let contracts = m9
        .get("contracts")
        .and_then(|v| v.as_array())
        .expect("contracts array");
    assert!(
        contracts.len() >= 6,
        "expected >= 6 verified contracts (always, 3x stable, 2x leads_to), got {contracts:?}"
    );
    for c in contracts {
        assert_eq!(
            c.get("passed").and_then(|v| v.as_bool()),
            Some(true),
            "contract failed: {c:?}"
        );
    }

    let refines = m9
        .get("refinements")
        .and_then(|v| v.as_array())
        .expect("refinements array");
    let m9_ref = refines
        .iter()
        .find(|r| {
            r.get("concrete").and_then(|v| v.as_str()) == Some("Http3Machine")
                && r.get("abstract").and_then(|v| v.as_str()) == Some("AbstractMultiplexedHttp")
        })
        .unwrap_or_else(|| {
            panic!("missing Http3Machine refines AbstractMultiplexedHttp in JSON:\n{stdout}")
        });
    assert_eq!(
        m9_ref.get("passed").and_then(|v| v.as_bool()),
        Some(true),
        "refinement failed: {m9_ref:?}"
    );
    assert_eq!(
        m9_ref.get("has_measure").and_then(|v| v.as_bool()),
        Some(true),
        "refinement must have measure: {m9_ref:?}"
    );

    eprintln!(
        "[M9 Http3Machine] state_count={states} edge_count={edges} capped={capped} contracts={} refinement=PASS",
        contracts.len()
    );
}

// =========================================================================
// 3. RFC 9114 + RFC 9218 + RFC 9297 Conformance & Full End-to-End QUIC Suite
// =========================================================================

#[test]
fn rfc9114_rfc9297_conformance_and_e2e_quic_suite() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.String
  import Engine.Types
  import Net.Http.Types
  import Net.Http.Priority
  import Net.Http.Qpack.Codec
  import Net.Http.V3.Frame
  import Net.Http.V3.Conn
  import Net.Quic.Types
  import Net.Quic.Varint
  import Net.Quic.Conn
  import Prim.Atomic

  copy-bytes-to-dma :: Bytes -> Nat -> I64 -> Nat ->{MVar} Unit
  copy-bytes-to-dma b i dst n =
    if i >= n then ()
    else if i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 dst (nat-to-i64 i)) (bytes-get-u8 b i) in
      copy-bytes-to-dma b (i + 1) dst n
    else ()

  copy-str-to-dma :: String -> I64 ->{MVar} U32
  copy-str-to-dma s dst =
    let b = string-to-bytes s in
    let n = bytes-len b in
    let _ = copy-bytes-to-dma b 0 dst n in
    wrapping-i64-to-u32 (nat-to-i64 n)

  dma-eq-str-loop :: I64 -> Bytes -> Nat -> Nat ->{MVar} Bool
  dma-eq-str-loop addr b i n =
    if i >= n then True
    else if i < bytes-len b then
      let got = raw-load-u8 (wrapping-add-i64 addr (nat-to-i64 i)) in
      if eq-u8 got (bytes-get-u8 b i) then dma-eq-str-loop addr b (i + 1) n
      else False
    else False

  dma-eq-str :: I64 -> U32 -> String ->{MVar} Bool
  dma-eq-str addr len expected =
    let b = string-to-bytes expected in
    let n = bytes-len b in
    if u32-to-nat len /= n then False
    else dma-eq-str-loop addr b 0 n

  -- Test A: Frame codecs, forbidden H2 frames/settings, duplicates, GREASE, Alt-Svc, PRIORITY_UPDATE, RFC 9297 Datagrams
  test-frame-codecs :: I64 ->{MVar} Int
  test-frame-codecs buf =
    let tmp = wrapping-add-i64 buf 4096 in
    -- A1: Alt-Svc formatting
    let as1 = h3-format-alt-svc-raw buf 0 64 443 86400 in
    let ok-as1 = eq-u32 as1 19 && dma-eq-str buf as1 "h3=\":443\"; ma=86400" in
    let as2 = h3-format-alt-svc-raw buf 0 64 8443 3600 in
    let ok-as2 = eq-u32 as2 19 && dma-eq-str buf as2 "h3=\":8443\"; ma=3600" in
    -- A2: Forbidden HTTP/2 frame types (0x02, 0x06, 0x08, 0x09) -> H3_FRAME_UNEXPECTED
    let _f2 = h3-frame-encode-header-raw buf 0 64 0x02 0 in
    let d2 = h3-frame-decode-header-raw buf 0 64 in
    let _f6 = h3-frame-encode-header-raw buf 0 64 0x06 0 in
    let d6 = h3-frame-decode-header-raw buf 0 64 in
    let _f8 = h3-frame-encode-header-raw buf 0 64 0x08 0 in
    let d8 = h3-frame-decode-header-raw buf 0 64 in
    let _f9 = h3-frame-encode-header-raw buf 0 64 0x09 0 in
    let d9 = h3-frame-decode-header-raw buf 0 64 in
    let ok-forb-f = (
      eq-u8 d2.h3f-status h3-fdec-error && eq-u64 d2.h3f-error h3-err-frame-unexpected
        && eq-u8 d6.h3f-status h3-fdec-error && eq-u64 d6.h3f-error h3-err-frame-unexpected
        && eq-u8 d8.h3f-status h3-fdec-error && eq-u64 d8.h3f-error h3-err-frame-unexpected
        && eq-u8 d9.h3f-status h3-fdec-error && eq-u64 d9.h3f-error h3-err-frame-unexpected) in
    -- A3: GREASE identification & unknown frame skipping (0x21 = 0x1f*0 + 0x21, 0x40 = 0x1f*1 + 0x21)
    let ok-grease-id = h3-is-grease 0x21 && h3-is-grease 0x40 && not (h3-is-grease 0x04) in
    let _gp = copy-str-to-dma "junk" tmp in
    let gw = h3-frame-encode-raw buf 0 64 0x21 tmp 4 in
    let gskip = h3-frame-skip-unknown-raw buf 0 gw in
    let ok-grease = ok-grease-id && gt-u32 gw 4 && eq-u32 gskip gw in
    -- A4: SETTINGS encode/decode round-trip + forbidden H2 setting + duplicate ID rejection
    let sw = h3-settings-encode-raw buf 0 128 (h3-settings-default ()) in
    let sh = h3-frame-decode-header-raw buf 0 sw in
    let sd = h3-settings-decode-raw buf sh.h3f-hdr-len (wrapping-u64-to-u32 sh.h3f-payload-len) in
    let ok-set-rt = (
      eq-u8 sh.h3f-status h3-fdec-ok && eq-u64 sh.h3f-type h3-frame-settings
        && eq-u8 sd.h3sd-ok 1
        && eq-u64 sd.h3sd-settings.h3s-qpack-max-table-cap 0
        && eq-u64 sd.h3sd-settings.h3s-max-field-section-size 65536
        && eq-u64 sd.h3sd-settings.h3s-qpack-blocked-streams 0
        && eq-u8 sd.h3sd-settings.h3s-enable-connect-protocol 1
        && eq-u8 sd.h3sd-settings.h3s-h3-datagram 1) in
    -- Forbidden H2 setting 0x02 (ENABLE_PUSH)
    let _fs0 = raw-store-u8 buf 0x02 in
    let _fs1 = raw-store-u8 (wrapping-add-i64 buf 1) 0x00 in
    let fsd = h3-settings-decode-raw buf 0 2 in
    -- Duplicate setting ID (0x06 twice)
    let _ds0 = raw-store-u8 buf 0x06 in
    let _ds1 = raw-store-u8 (wrapping-add-i64 buf 1) 0x10 in
    let _ds2 = raw-store-u8 (wrapping-add-i64 buf 2) 0x06 in
    let _ds3 = raw-store-u8 (wrapping-add-i64 buf 3) 0x20 in
    let dsd = h3-settings-decode-raw buf 0 4 in
    -- Invalid H3_DATAGRAM value (2)
    let _bs0 = raw-store-u8 buf 0x33 in
    let _bs1 = raw-store-u8 (wrapping-add-i64 buf 1) 0x02 in
    let bsd = h3-settings-decode-raw buf 0 2 in
    let ok-set-err = (
      eq-u8 fsd.h3sd-ok 0 && eq-u64 fsd.h3sd-error h3-err-settings-error
        && eq-u8 dsd.h3sd-ok 0 && eq-u64 dsd.h3sd-error h3-err-settings-error
        && eq-u8 bsd.h3sd-ok 0 && eq-u64 bsd.h3sd-error h3-err-settings-error) in
    -- A5: PRIORITY_UPDATE (0x0f0700) round-trip
    let pv-len = copy-str-to-dma "u=1, i" tmp in
    let puw = h3-frame-encode-priority-update-raw buf 0 64 8 tmp pv-len in
    let puh = h3-frame-decode-header-raw buf 0 puw in
    let pud = h3-frame-decode-priority-update-raw buf puh.h3f-hdr-len (wrapping-u64-to-u32 puh.h3f-payload-len) in
    let ok-pu = (
      eq-u8 puh.h3f-status h3-fdec-ok && eq-u64 puh.h3f-type h3-frame-priority-update-req
        && eq-u8 pud.h3pu-ok 1 && eq-u64 pud.h3pu-element-id 8
        && dma-eq-str (wrapping-add-i64 buf (u32-to-i64 pud.h3pu-val-off)) pud.h3pu-val-len "u=1, i") in
    -- A6: RFC 9297 H3 Datagram quarter-stream-id varint prefix codec
    let dg-plen = copy-str-to-dma "dgram-msg" tmp in
    let dgw = h3-dgram-encode-raw buf 0 64 256 tmp dg-plen in
    let dgd = h3-dgram-decode-raw buf 0 dgw in
    let dg-bad = h3-dgram-encode-raw buf 0 64 1 tmp dg-plen in -- non-client-bidi must fail!
    let ok-dg = (
      gt-u32 dgw dg-plen && eq-u32 dg-bad 0
        && eq-u8 dgd.h3dg-ok 1 && eq-u64 dgd.h3dg-stream-id 256 && eq-u64 dgd.h3dg-quarter-id 64
        && dma-eq-str (wrapping-add-i64 buf (u32-to-i64 dgd.h3dg-payload-off)) dgd.h3dg-payload-len "dgram-msg") in
    if not ok-as1 || not ok-as2 then 10
    else if not ok-forb-f then 11
    else if not ok-grease then 12
    else if not ok-set-rt then 13
    else if not ok-set-err then 14
    else if not ok-pu then 15
    else if not ok-dg then 16
    else 0

  -- Test B: RFC 9114 critical-stream & frame-placement invariants + dynamic QPACK interop
  test-h3-invariants :: I64 ->{IO, MVar} Int
  test-h3-invariants buf =
    let tmp = wrapping-add-i64 buf 4096 in
    -- B1: H3_MISSING_SETTINGS when first frame on Control stream is GOAWAY
    let ok-miss = (
      case h3-conn-new h3-role-server of
        None -> False
        Some c ->
          let w0 = h3-uni-write-type-raw buf 0 64 h3-uni-control in
          let w1 = h3-frame-encode-goaway-raw buf w0 64 4 in
          let r = h3-conn-feed-uni c 2 buf (wrapping-add-u32 w0 w1) 0 in
          let ok = eq-u32 r.h3fr-status h3-feed-error && eq-u64 r.h3fr-error h3-err-missing-settings && eq-u8 (h3-conn-state c) h3-state-error in
          let _ = h3-conn-destroy c in ok) in
    -- B2: Duplicate peer Control stream -> H3_STREAM_CREATION_ERROR
    let ok-dup-ctrl = (
      case h3-conn-new h3-role-server of
        None -> False
        Some c ->
          let w = h3-conn-write-initial-control-stream c buf 0 128 in
          let r1 = h3-conn-feed-uni c 2 buf w 0 in
          let r2 = h3-conn-feed-uni c 6 buf w 0 in
          let ok = eq-u32 r1.h3fr-status h3-feed-settings-ready && eq-u8 (h3-conn-peer-ctrl-count c) 1 && eq-u32 r2.h3fr-status h3-feed-error && eq-u64 r2.h3fr-error h3-err-stream-creation-error && eq-u8 (h3-conn-peer-ctrl-count c) 1 in
          let _ = h3-conn-destroy c in ok) in
    -- B3: Duplicate peer QPACK Encoder/Decoder stream -> H3_STREAM_CREATION_ERROR
    let ok-dup-qpack = (
      case h3-conn-new h3-role-server of
        None -> False
        Some c ->
          let _e = h3-uni-write-type-raw buf 0 16 h3-uni-qpack-encoder in
          let r1 = h3-conn-feed-uni c 6 buf 1 0 in
          let r2 = h3-conn-feed-uni c 10 buf 1 0 in
          let ok = eq-u32 r1.h3fr-status h3-feed-ok && eq-u32 r2.h3fr-status h3-feed-error && eq-u64 r2.h3fr-error h3-err-stream-creation-error in
          let _ = h3-conn-destroy c in ok) in
    -- B4: Second SETTINGS on Control stream -> H3_FRAME_UNEXPECTED
    let ok-dup-set = (
      case h3-conn-new h3-role-server of
        None -> False
        Some c ->
          let w = h3-conn-write-initial-control-stream c buf 0 128 in
          let _r1 = h3-conn-feed-uni c 2 buf w 0 in
          let sw = h3-settings-encode-raw buf 0 128 (h3-settings-default ()) in
          let r2 = h3-conn-feed-uni c 2 buf sw 0 in
          let ok = eq-u32 r2.h3fr-status h3-feed-error && eq-u64 r2.h3fr-error h3-err-frame-unexpected in
          let _ = h3-conn-destroy c in ok) in
    -- B5: Closing critical Control stream -> H3_CLOSED_CRITICAL_STREAM
    let ok-close-crit = (
      case h3-conn-new h3-role-server of
        None -> False
        Some c ->
          let w = h3-conn-write-initial-control-stream c buf 0 128 in
          let _r1 = h3-conn-feed-uni c 2 buf w 0 in
          let r2 = h3-conn-feed-uni c 2 buf 0 1 in
          let ok = eq-u32 r2.h3fr-status h3-feed-error && eq-u64 r2.h3fr-error h3-err-closed-critical-stream in
          let _ = h3-conn-destroy c in ok) in
    -- B6: DATA on Control stream & SETTINGS on Request stream -> H3_FRAME_UNEXPECTED
    let ok-frame-place = (
      case h3-conn-new h3-role-server of
        None -> False
        Some c1 ->
          let w = h3-conn-write-initial-control-stream c1 buf 0 128 in
          let _r1 = h3-conn-feed-uni c1 2 buf w 0 in
          let dw = h3-frame-encode-raw buf 0 64 h3-frame-data tmp 0 in
          let r-bad-ctrl = h3-conn-feed-uni c1 2 buf dw 0 in
          let _ = h3-conn-destroy c1 in
          case h3-conn-new h3-role-server of
            None -> False
            Some c2 ->
              let sw = h3-settings-encode-raw buf 0 128 (h3-settings-default ()) in
              let r-bad-req = h3-conn-feed-request c2 0 buf sw 0 in
              let ok = eq-u32 r-bad-ctrl.h3fr-status h3-feed-error && eq-u64 r-bad-ctrl.h3fr-error h3-err-frame-unexpected && eq-u32 r-bad-req.h3fr-status h3-feed-error && eq-u64 r-bad-req.h3fr-error h3-err-frame-unexpected in
              let _ = h3-conn-destroy c2 in ok) in
    -- B7: GREASE unknown uni stream & GREASE frame after SETTINGS skipped cleanly
    let ok-grease-skip = (
      case h3-conn-new h3-role-server of
        None -> False
        Some c ->
          let w0 = h3-conn-write-initial-control-stream c buf 0 128 in
          let _ = copy-str-to-dma "grease-payload" tmp in
          let w1 = h3-frame-encode-raw buf w0 256 0x40 tmp 14 in
          let r1 = h3-conn-feed-uni c 2 buf (wrapping-add-u32 w0 w1) 0 in
          -- Also unknown GREASE uni stream (type 0x21) with FIN is allowed!
          let u0 = h3-uni-write-type-raw buf 0 64 0x21 in
          let r2 = h3-conn-feed-uni c 14 buf u0 1 in
          let ok = eq-u32 r1.h3fr-status h3-feed-settings-ready && eq-u32 r2.h3fr-status h3-feed-ok && eq-u8 (h3-conn-state c) h3-state-open in
          let _ = h3-conn-destroy c in ok) in
    -- B8: Bounded dynamic QPACK interop (encoder stream Insert + request referencing dynamic index 0)
    let ok-dyn-qpack = (
      case h3-conn-new-with-qpack h3-role-server 4096 of
        None -> False
        Some c ->
          -- Open peer QPACK encoder stream (sid=6) and send SetCapacity(4096) + InsertLiteral("x-cinch", "h3")
          let u0 = h3-uni-write-type-raw buf 0 64 h3-uni-qpack-encoder in
          let e1 = qpack-enc-set-capacity buf u0 256 4096 in
          let nlen = copy-str-to-dma "x-cinch" tmp in
          let vaddr = wrapping-add-i64 tmp 64 in
          let vlen = copy-str-to-dma "h3" vaddr in
          let e2 = qpack-enc-insert-literal buf e1.qe-written 256 qpack-huff-never tmp nlen qpack-huff-never vaddr vlen in
          let r-enc = h3-conn-feed-uni c 6 buf e2.qe-written 0 in
          -- Encode a HEADERS section with RIC=1, Base=1 referencing dynamic relative index 0 (`10_000000` = 0x80)
          let pfx = qpack-encode-prefix tmp 0 64 4096 1 1 in
          let fld = qpack-encode-indexed-dynamic tmp pfx.qe-written 64 0 in
          let hfw = h3-frame-encode-raw buf 0 128 h3-frame-headers tmp fld.qe-written in
          let r-req = h3-conn-feed-request c 0 buf hfw 1 in
          let hp = qpack-pair-load (h3-conn-decoded-hdrs-addr c) 0 in
          let txt = h3-conn-decoded-text-addr c in
          let ok = (
            eq-u32 r-enc.h3fr-status h3-feed-ok
              && eq-u64 (qpack-table-insert-count (h3-conn-qpack-table c)) 1
              && eq-u32 r-req.h3fr-status h3-feed-headers-ready
              && eq-u32 (h3-conn-decoded-hdr-count c) 1
              && dma-eq-str (wrapping-add-i64 txt (u32-to-i64 hp.hp-name-off)) hp.hp-name-len "x-cinch"
              && dma-eq-str (wrapping-add-i64 txt (u32-to-i64 hp.hp-val-off)) hp.hp-val-len "h3") in
          let _ = h3-conn-destroy c in ok) in
    if not ok-miss then 20
    else if not ok-dup-ctrl then 21
    else if not ok-dup-qpack then 22
    else if not ok-dup-set then 23
    else if not ok-close-crit then 24
    else if not ok-frame-place then 25
    else if not ok-grease-skip then 26
    else if not ok-dyn-qpack then 27
    else 0

  -- Test C: Full End-to-End Client-Server HTTP/3 over real Net.Quic.Conn packets
  test-e2e-h3-over-quic :: I64 ->{IO, MVar} Int
  test-e2e-h3-over-quic dma =
    let pkt-buf = dma in
    let str-buf = wrapping-add-i64 dma 4096 in
    let body-buf = wrapping-add-i64 dma 8192 in
    let dg-rx = wrapping-add-i64 dma 12288 in
    let cli-scid = quic-cid-encode-core (core-id 0) 101 0 8 in
    let srv-scid = quic-cid-encode-core (core-id 1) 202 0 8 in
    let odcid = quic-cid-encode-core (core-id 0) 303 0 8 in
    let cli-addr = (0x01001001 :: U64) in
    let srv-addr = (0x02000443 :: U64) in
    let cli-q = quic-conn-new-client cli-scid odcid srv-addr 1200 10000 111 in
    let srv-q = quic-conn-new-server srv-scid cli-scid odcid cli-addr 1200 10000 222 in
    case h3-conn-new h3-role-client of
      None ->
        let _0 = quic-conn-destroy cli-q in
        let _1 = quic-conn-destroy srv-q in 30
      Some cli-h3 ->
        case h3-conn-new h3-role-server of
          None ->
            let _0 = h3-conn-destroy cli-h3 in
            let _1 = quic-conn-destroy cli-q in
            let _2 = quic-conn-destroy srv-q in 31
          Some srv-h3 ->
            let rc = run-e2e cli-h3 cli-q srv-h3 srv-q cli-addr srv-addr pkt-buf str-buf body-buf dg-rx in
            let _0 = h3-conn-destroy cli-h3 in
            let _1 = h3-conn-destroy srv-h3 in
            let _2 = quic-conn-destroy cli-q in
            let _3 = quic-conn-destroy srv-q in
            rc

  run-e2e :: H3Conn -> QuicConn -> H3Conn -> QuicConn -> U64 -> U64 -> I64 -> I64 -> I64 -> I64 ->{IO, MVar} Int
  run-e2e cli-h3 cli-q srv-h3 srv-q cli-addr srv-addr pkt-buf str-buf body-buf dg-rx =
    -- 1. Drive QUIC handshake + H3 Control/QPACK uni stream & SETTINGS exchange
    let t1 = h3-conn-drive-quic cli-h3 cli-q srv-h3 srv-q cli-addr srv-addr pkt-buf 20000 16 in
    if not (quic-conn-is-established cli-q && quic-conn-is-established srv-q) then 32
    else if not (h3-conn-peer-settings-seen cli-h3 && h3-conn-peer-settings-seen srv-h3) then 33
    else if ne-u8 (h3-conn-peer-ctrl-count srv-h3) 1 || ne-u8 (h3-conn-peer-qenc-count srv-h3) 1 || ne-u8 (h3-conn-peer-qdec-count srv-h3) 1 then 34
    else if ne-u8 (h3-conn-peer-settings srv-h3).h3s-h3-datagram 1 || ne-u8 (h3-conn-peer-settings cli-h3).h3s-enable-connect-protocol 1 then 35
    else
      -- 2. Send 3 concurrent HTTP/3 requests (streams 0, 4, 8) + RFC 9218 PRIORITY_UPDATE frames!
      let auth-addr = str-buf in
      let auth-len = copy-str-to-dma "cinch.example" auth-addr in
      let p0-addr = wrapping-add-i64 str-buf 64 in
      let p0-len = copy-str-to-dma "/a" p0-addr in
      let p4-addr = wrapping-add-i64 str-buf 128 in
      let p4-len = copy-str-to-dma "/b" p4-addr in
      let p8-addr = wrapping-add-i64 str-buf 192 in
      let p8-len = copy-str-to-dma "/c" p8-addr in
      let req-body = wrapping-add-i64 str-buf 256 in
      let req-blen = copy-str-to-dma "hello-h3-body" req-body in
      let sid0 = h3-conn-send-request-quic cli-h3 cli-q method-code-get p0-addr p0-len auth-addr auth-len 0 0 in
      let sid4 = h3-conn-send-request-quic cli-h3 cli-q method-code-post p4-addr p4-len auth-addr auth-len req-body req-blen in
      let sid8 = h3-conn-send-request-quic cli-h3 cli-q method-code-get p8-addr p8-len auth-addr auth-len 0 0 in
      if ne-u64 sid0 0 || ne-u64 sid4 4 || ne-u64 sid8 8 then 36
      else
        let t2 = h3-conn-drive-quic cli-h3 cli-q srv-h3 srv-q cli-addr srv-addr pkt-buf t1 12 in
        if ne-u32 (h3-conn-active-req-count srv-h3) 3 then 37
        else if ne-u32 (h3-conn-req-data-total srv-h3 4) req-blen then 38
        else if not (h3-conn-req-fin-rcvd srv-h3 0 && h3-conn-req-fin-rcvd srv-h3 4 && h3-conn-req-fin-rcvd srv-h3 8) then 39
        else
          -- Send RFC 9218 PRIORITY_UPDATE frames over Control stream:
          -- sid=0 -> u=5, sid=4 -> u=1, i, sid=8 -> u=1, i
          let u5-len = copy-str-to-dma "u=5" (wrapping-add-i64 str-buf 512) in
          let u1i-len = copy-str-to-dma "u=1, i" (wrapping-add-i64 str-buf 544) in
          let _pu0 = h3-conn-send-priority-update-quic cli-h3 cli-q 0 (wrapping-add-i64 str-buf 512) u5-len in
          let _pu4 = h3-conn-send-priority-update-quic cli-h3 cli-q 4 (wrapping-add-i64 str-buf 544) u1i-len in
          let _pu8 = h3-conn-send-priority-update-quic cli-h3 cli-q 8 (wrapping-add-i64 str-buf 544) u1i-len in
          let t3 = h3-conn-drive-quic cli-h3 cli-q srv-h3 srv-q cli-addr srv-addr pkt-buf t2 8 in
          if ne-u8 (h3-conn-req-priority srv-h3 0).prio-urgency 5 || ne-u8 (h3-conn-req-priority srv-h3 4).prio-urgency 1 || ne-u8 (h3-conn-req-priority srv-h3 8).prio-incremental 1 then 40
          else
            -- 3. Server queues 8-byte bodies on streams 0, 4, 8 and flushes via PriorityScheduler (4B quantum -> 4, 8, 4, 8, 0, 0!)
            let _rb = copy-str-to-dma "resp-h3!" body-buf in
            let _q0 = h3-conn-queue-response srv-h3 0 200 body-buf 8 in
            let _q4 = h3-conn-queue-response srv-h3 4 200 body-buf 8 in
            let _q8 = h3-conn-queue-response srv-h3 8 200 body-buf 8 in
            let s1 = h3-conn-flush-priority-quic srv-h3 srv-q 4 in
            let s2 = h3-conn-flush-priority-quic srv-h3 srv-q 4 in
            let s3 = h3-conn-flush-priority-quic srv-h3 srv-q 4 in
            let s4 = h3-conn-flush-priority-quic srv-h3 srv-q 4 in
            let s5 = h3-conn-flush-priority-quic srv-h3 srv-q 4 in
            let s6 = h3-conn-flush-priority-quic srv-h3 srv-q 4 in
            let s7 = h3-conn-flush-priority-quic srv-h3 srv-q 4 in
            if ne-u64 s1 4 || ne-u64 s2 8 || ne-u64 s3 4 || ne-u64 s4 8 || ne-u64 s5 0 || ne-u64 s6 0 || ne-u64 s7 h3-sid-unset then 41
            else
              let t4 = h3-conn-drive-quic cli-h3 cli-q srv-h3 srv-q cli-addr srv-addr pkt-buf t3 12 in
              if ne-u16 (h3-conn-req-status cli-h3 0) 200 || ne-u16 (h3-conn-req-status cli-h3 4) 200 || ne-u16 (h3-conn-req-status cli-h3 8) 200 then 42
              else if ne-u32 (h3-conn-req-data-total cli-h3 0) 8 || ne-u32 (h3-conn-req-data-total cli-h3 4) 8 || ne-u32 (h3-conn-req-data-total cli-h3 8) 8 then 43
              else
                -- 4. RFC 9297 H3 Datagrams bidirectionally over real QUIC DATAGRAM frames!
                let dg-msg-len = copy-str-to-dma "h3-dgram-ping" body-buf in
                let _ds = h3-conn-send-datagram cli-h3 cli-q 4 body-buf dg-msg-len in
                let t5 = h3-conn-drive-quic cli-h3 cli-q srv-h3 srv-q cli-addr srv-addr pkt-buf t4 6 in
                let dr = h3-conn-recv-datagram srv-h3 srv-q dg-rx 256 in
                if eq-u8 dr.h3dg-ok 0 || ne-u64 dr.h3dg-stream-id 4 || ne-u64 dr.h3dg-quarter-id 1 || not (dma-eq-str dg-rx dr.h3dg-payload-len "h3-dgram-ping") then 44
                else
                  let dg-rep-len = copy-str-to-dma "h3-dgram-pong" body-buf in
                  let _ds2 = h3-conn-send-datagram srv-h3 srv-q 8 body-buf dg-rep-len in
                  let _t6 = h3-conn-drive-quic cli-h3 cli-q srv-h3 srv-q cli-addr srv-addr pkt-buf t5 6 in
                  let dr2 = h3-conn-recv-datagram cli-h3 cli-q dg-rx 256 in
                  if eq-u8 dr2.h3dg-ok 0 || ne-u64 dr2.h3dg-stream-id 8 || ne-u64 dr2.h3dg-quarter-id 2 || not (dma-eq-str dg-rx dr2.h3dg-payload-len "h3-dgram-pong") then 45
                  else
                    -- 5. GOAWAY graceful drain! Server sends GOAWAY(12) -> client sees GOAWAY, server rejects sid=12
                    let _ga = h3-conn-send-goaway-quic srv-h3 srv-q 12 in
                    let _t7 = h3-conn-drive-quic cli-h3 cli-q srv-h3 srv-q cli-addr srv-addr pkt-buf _t6 6 in
                    let rej = h3-conn-feed-request srv-h3 12 body-buf 0 0 in
                    if not (h3-conn-goaway-recv cli-h3) || ne-u64 (h3-conn-peer-goaway-id cli-h3) 12 || ne-u8 (h3-conn-state cli-h3) h3-state-goaway then 46
                    else if ne-u32 rej.h3fr-status h3-feed-error || ne-u64 rej.h3fr-error h3-err-request-rejected then 47
                    else 0

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 32768 4096 in
    if eq-u64 dma 0 then 99
    else
      let base = wrapping-u64-to-i64 dma in
      let r1 = test-frame-codecs base in
      let r2 = if r1 == 0 then test-h3-invariants base else r1 in
      let r3 = if r2 == 0 then test-e2e-h3-over-quic base else r2 in
      let _ = engine-dma-free dma 32768 in
      r3
"#;
    build_and_run_leak_checked("rfc9114-e2e-suite", src, 0);
}

// =========================================================================
// 4. Steady-state 25,000 HTTP/3 HEADERS+DATA encode/decode + QPACK zero-HOL
//    request cycles: allocs == 0, atomic_rc == 0, leaked == 0
// =========================================================================

#[test]
fn steady_state_25000_http3_zero_hol_cycles_zero_alloc_zero_atomic_rc() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Engine.Types
  import Net.Http.Types
  import Net.Http.Qpack.Codec
  import Net.Http.V3.Frame
  import Net.Http.V3.Conn
  import Prim.Atomic

  -- 25,000 complete HTTP/3 zero-HOL request/response cycles:
  --   1) Encode HTTP/3 request (QPACK zero-HOL HEADERS + DATA frame)
  --   2) Feed request into H3Conn (decodes QPACK zero-HOL HEADERS + DATA span)
  --   3) Queue response & flush via PriorityScheduler (QPACK zero-HOL HEADERS + DATA)
  -- Every single operation in this loop is `#[no-alloc]`!
  #[no-alloc]
  hot-h3-loop :: H3Conn -> I64 -> I64 -> I64 -> I64 -> I64 -> I64 -> U64 -> U32 -> U32 ->{MVar} U32
  hot-h3-loop conn rx tx scratch path-addr auth-addr body-addr sid done limit =
    if ge-u32 done limit then done
    else
      let req-len = h3-encode-request-raw rx 0 512 method-code-post path-addr 1 auth-addr 9 body-addr 8 scratch 256 in
      if eq-u32 req-len 0 then 0
      else
        let fr = h3-conn-feed-request conn sid rx req-len 1 in
        if ne-u32 fr.h3fr-status h3-feed-headers-ready then 0
        else if ne-u32 (h3-conn-last-data-len conn) 8 then 0
        else if not (h3-conn-queue-response conn sid 200 body-addr 8) then 0
        else
          let fl = h3-conn-flush-priority-dma conn 64 tx 0 512 in
          if eq-u8 fl.h3fl-ok 0 then 0
          else if eq-u8 fl.h3fl-done 0 then 0
          else hot-h3-loop conn rx tx scratch path-addr auth-addr body-addr (wrapping-add-u64 sid 4) (wrapping-add-u32 done 1) limit

  main :: Unit ->{IO, MVar} Int
  main _ =
    case h3-conn-new h3-role-server of
      None -> 1
      Some conn ->
        let dma = engine-dma-alloc 8192 4096 in
        if eq-u64 dma 0 then
          let _ = h3-conn-destroy conn in 2
        else
          let rx = wrapping-u64-to-i64 dma in
          let tx = wrapping-add-i64 rx 2048 in
          let scratch = wrapping-add-i64 tx 2048 in
          let path-addr = wrapping-add-i64 scratch 1024 in
          let auth-addr = wrapping-add-i64 path-addr 64 in
          let body-addr = wrapping-add-i64 auth-addr 64 in
          -- "/" (1B)
          let _p = raw-store-u8 path-addr 47 in
          -- "localhost" (9B)
          let _a0 = raw-store-u64 auth-addr 0x736F686C61636F6C in
          let _a1 = raw-store-u8 (wrapping-add-i64 auth-addr 8) 116 in
          -- "pong-h3!" (8B)
          let _b0 = raw-store-u64 body-addr 0x2133682D676E6F70 in
          -- Establish peer Control stream + SETTINGS once before entering steady-state measurement
          let cw = h3-conn-write-initial-control-stream conn rx 0 256 in
          let fc = h3-conn-feed-uni conn 2 rx cw 0 in
          if ne-u32 fc.h3fr-status h3-feed-settings-ready then
            let _0 = engine-dma-free dma 8192 in
            let _1 = h3-conn-destroy conn in 3
          else
            let _ = engine-thread-counters-reset () in
            let completed = hot-h3-loop conn rx tx scratch path-addr auth-addr body-addr 0 0 25000 in
            let allocs = engine-thread-alloc-count () in
            let atomic-rcs = engine-thread-atomic-rc-count () in
            let active = h3-conn-active-req-count conn in
            let opened = h3-conn-total-req-opened conn in
            let closed = h3-conn-total-req-closed conn in
            let blk = qpack-table-blocked-streams (h3-conn-qpack-table conn) in
            let _0 = engine-dma-free dma 8192 in
            let _1 = h3-conn-destroy conn in
            if ne-u32 completed 25000 then 10
            else if ne-u64 allocs 0 then 11
            else if ne-u64 atomic-rcs 0 then 12
            else if ne-u32 active 0 then 13
            else if ne-u64 opened 25000 then 14
            else if ne-u64 closed 25000 then 15
            else if ne-u32 blk 0 then 16
            else 0
"#;
    build_and_run_leak_checked("steady-25000-zero-alloc", src, 0);
}
