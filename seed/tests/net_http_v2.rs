//! Integration, formal-verification (M7 `Http2Machine`), RFC 9113 & RFC 9218
//! conformance/torture suite, CVE-2023-44487 Rapid-Reset & control-frame flood
//! defense, 1,000-trial `Effect.Test.Hegel` differential fuzzing, and 25,000
//! multiplexed request/response zero-allocation / zero-atomic-RC / leak-checked
//! tests for `Net.Http.V2.{Frame,Stream,Conn,Spec}`.
//!
//! Routed through `common::drive_with_profile_opts` so the exact same suite
//! runs against both the Rust seed and the Stage 3 self-hosted compiler.

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
        "cinch-net-http-v2-{tag}-{}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn build_and_run_leak_checked(tag: &str, src: &str, expected_code: i32) -> (String, String) {
    let dir = fresh_dir(tag);
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let exe = dir.join(format!("http2-{tag}"));
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
// 1. Manifest registration & #[no-alloc] safety assertions check
// =========================================================================

#[test]
fn net_http_v2_manifest_registration_and_no_alloc_check() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, rel_path) in [
        ("Net.Http.V2.Frame", "Net/Http/V2/Frame.cinch"),
        ("Net.Http.V2.Stream", "Net/Http/V2/Stream.cinch"),
        ("Net.Http.V2.Conn", "Net/Http/V2/Conn.cinch"),
        ("Net.Http.V2.Spec", "Net/Http/V2/Spec.cinch"),
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

    let pos_dma = pkg.order.iter().position(|n| n == "Engine.Mem.DmaPool").unwrap();
    let pos_prio = pkg.order.iter().position(|n| n == "Net.Http.Priority").unwrap();
    let pos_hpack = pkg.order.iter().position(|n| n == "Net.Http.Hpack.Codec").unwrap();
    let pos_frame = pkg.order.iter().position(|n| n == "Net.Http.V2.Frame").unwrap();
    let pos_stream = pkg.order.iter().position(|n| n == "Net.Http.V2.Stream").unwrap();
    let pos_conn = pkg.order.iter().position(|n| n == "Net.Http.V2.Conn").unwrap();
    assert!(pos_dma < pos_frame);
    assert!(pos_frame < pos_stream && pos_prio < pos_stream);
    assert!(pos_stream < pos_conn && pos_hpack < pos_conn);

    // Compile-time #[no-alloc] audit across all hot-path Frame, Stream & Conn entry points
    let dir = fresh_dir("no-alloc-check");
    std::fs::write(
        dir.join("Main.cinch"),
        r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Engine.Types
  import Engine.Mem.DmaPool
  import Net.Http.Priority
  import Net.Http.V2.Frame
  import Net.Http.V2.Stream
  import Net.Http.V2.Conn

  #[no-alloc]
  audit-h2-hot-path :: H2Conn -> H2StreamTable -> DmaSlice -> I64 -> I64 ->{MVar} U32
  audit-h2-hot-path conn tbl sl rx-buf tx-buf =
    let _pw = h2-preface-write-dma tx-buf 0 64 in
    let _pm = h2-preface-match-dma rx-buf 0 24 in
    let hdr = h2-frame-read-header-dma rx-buf 0 in
    let _vh = h2-frame-validate-header hdr h2-default-max-frame-size in
    let _wh = h2-frame-write-header-dma tx-buf 0 64 hdr in
    let _hr = h2-frame-write-headroom sl h2-frame-data h2-flag-end-stream 1 0 in
    let _pb = h2-frame-strip-padding-dma rx-buf 9 16 h2-flag-padded in
    let slot = h2-stream-open tbl 1 65535 65535 in
    let _lk = h2-stream-lookup tbl 1 in
    let _aw = h2-stream-add-send-window tbl slot 1024 in
    let _cw = h2-stream-consume-send-window tbl slot 16 in
    let _rw = h2-stream-consume-recv-window tbl slot 16 in
    let _adj = h2-stream-adjust-all-send-windows tbl (-100) in
    let _cl = h2-stream-close tbl 1 in
    let fr = h2-conn-feed conn rx-buf 0 64 tx-buf 0 256 in
    let wr = h2-conn-send-response conn 1 200 rx-buf 0 tx-buf 0 256 in
    let fl = h2-conn-flush-data conn 1024 tx-buf 0 256 in
    let _fs = h2-conn-frame-data-slice conn 1 sl 0 1 in
    wrapping-add-u32 fr.fr-consumed (wrapping-add-u32 wr.wr-tx-pos fl.wr-bytes-sent)

  main :: Int
  main = 0
"#,
    )
    .expect("write Main.cinch");

    let opts = DriverOptions::default();
    common::drive_with_profile_opts(
        Mode::Check,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        None,
        &TargetProfile::native(),
        &opts,
    )
    .expect("Net.Http.V2.{Frame,Stream,Conn} #[no-alloc] hot-path audit must check cleanly");
    let _ = std::fs::remove_dir_all(&dir);
}

// =========================================================================
// 2. Formal Model M7 (`Http2Machine`): capped=false, 0 violations under
//    CINCH_MODEL_FAIL_ON_CAPPED=1 + refinement with decreasing measure
// =========================================================================

#[test]
fn formal_model_m7_http2_machine_verifies_uncapped_zero_violations() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("prelude/Net/Http/V2/Spec.cinch");
    let spec_str = spec_path.to_string_lossy().into_owned();
    let cinch_bin = env!("CARGO_BIN_EXE_cinch");

    // 1. Check via `common::drive_with_profile_opts` with `model_fail_on_capped = true`
    let dir = fresh_dir("m7-spec-check");
    std::fs::write(
        dir.join("Main.cinch"),
        r#"
module Main where
  import Net.Http.V2.Spec

  main :: Int
  main = if h2-safety-ok h2-model-init then 0 else 1
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
    .expect("Net.Http.V2.Spec must check cleanly with model_fail_on_capped=true");

    // 2. CLI `cinch model inspect <Spec.cinch> Http2Machine --json` with CINCH_MODEL_FAIL_ON_CAPPED=1
    let inspect_out = Command::new(cinch_bin)
        .args(["model", "inspect", &spec_str, "Http2Machine", "--json"])
        .env("CINCH_MODEL_FAIL_ON_CAPPED", "1")
        .output()
        .expect("run cinch model inspect Http2Machine");
    let stdout = String::from_utf8_lossy(&inspect_out.stdout);
    let stderr = String::from_utf8_lossy(&inspect_out.stderr);
    assert!(
        inspect_out.status.success(),
        "M7 Http2Machine failed under CINCH_MODEL_FAIL_ON_CAPPED=1:\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    let json: serde_json::Value =
        serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("invalid JSON: {e}\n{stdout}"));
    let machines = json
        .as_array()
        .expect("top-level machines array in model inspect JSON");
    let m7 = machines
        .iter()
        .find(|m| m.get("name").and_then(|v| v.as_str()) == Some("Http2Machine"))
        .unwrap_or_else(|| panic!("Http2Machine missing from JSON:\n{stdout}"));
    let capped = m7.get("capped").and_then(|v| v.as_bool()).expect("capped bool");
    let states = m7.get("state_count").and_then(|v| v.as_u64()).expect("state_count");
    let edges = m7.get("edge_count").and_then(|v| v.as_u64()).expect("edge_count");
    assert!(!capped, "M7 Http2Machine MUST have capped=false; json:\n{stdout}");
    assert!(states >= 30 && states <= 8192, "unexpected M7 state count: {states}");

    let contracts = m7
        .get("contracts")
        .and_then(|v| v.as_array())
        .expect("contracts array");
    assert!(
        contracts.len() >= 5,
        "expected >= 5 verified contracts (always, 2x stable, 2x leads_to), got {contracts:?}"
    );
    for c in contracts {
        assert_eq!(
            c.get("passed").and_then(|v| v.as_bool()),
            Some(true),
            "contract failed: {c:?}"
        );
    }

    let refines = m7
        .get("refinements")
        .and_then(|v| v.as_array())
        .expect("refinements array");
    let m7_ref = refines
        .iter()
        .find(|r| {
            r.get("concrete").and_then(|v| v.as_str()) == Some("Http2Machine")
                && r.get("abstract").and_then(|v| v.as_str()) == Some("AbstractMultiplexedHttp")
        })
        .unwrap_or_else(|| {
            panic!("missing Http2Machine refines AbstractMultiplexedHttp in JSON:\n{stdout}")
        });
    assert_eq!(
        m7_ref.get("passed").and_then(|v| v.as_bool()),
        Some(true),
        "refinement failed: {m7_ref:?}"
    );

    eprintln!(
        "[M7 Http2Machine] state_count={states} edge_count={edges} capped={capped} contracts={} refinement=PASS",
        contracts.len()
    );
    let _ = std::fs::remove_dir_all(&dir);
}

// =========================================================================
// 3. RFC 9113 / RFC 9218 Conformance, Torture & Rapid-Reset Defense Suite
// =========================================================================

#[test]
fn rfc9113_conformance_priority_window_and_rapid_reset_defense() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.String
  import Engine.Types
  import Engine.Mem.DmaPool
  import Net.Http.Types
  import Net.Http.Priority
  import Net.Http.Hpack.Codec
  import Net.Http.V2.Frame
  import Net.Http.V2.Stream
  import Net.Http.V2.Conn
  import Prim.Atomic

  copy-str-to-dma :: String -> I64 ->{MVar} U32
  copy-str-to-dma s dst =
    let b = string-to-bytes s in
    let n = bytes-len b in
    let _ = copy-bytes-to-dma b 0 dst n in
    wrapping-i64-to-u32 (nat-to-i64 n)

  copy-bytes-to-dma :: Bytes -> Nat -> I64 -> Nat ->{MVar} Unit
  copy-bytes-to-dma b i dst n =
    if i >= n then ()
    else if i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 dst (nat-to-i64 i)) (bytes-get-u8 b i) in
      copy-bytes-to-dma b (i + 1) dst n
    else ()

  -- Helper: write a HEADERS frame containing static-indexed GET `/` (`0x82 0x86 0x84`)
  write-simple-get-headers :: I64 -> U32 -> U32 -> U32 -> U8 ->{MVar} U32
  write-simple-get-headers dst pos cap sid flags =
    let p1 = h2-frame-write-header-dma dst pos cap (H2FrameHeader 3 h2-frame-headers flags 0 sid) in
    let _0 = raw-store-u8 (wrapping-add-i64 dst (u32-to-i64 p1)) 0x82 in
    let _1 = raw-store-u8 (wrapping-add-i64 dst (u32-to-i64 (wrapping-add-u32 p1 1))) 0x86 in
    let _2 = raw-store-u8 (wrapping-add-i64 dst (u32-to-i64 (wrapping-add-u32 p1 2))) 0x84 in
    wrapping-add-u32 p1 3

  -- Helper: perform preface + client SETTINGS exchange on a freshly created server conn
  handshake-server :: H2Conn -> I64 -> I64 ->{MVar} Bool
  handshake-server conn rx tx =
    let p-end = h2-preface-write-dma rx 0 256 in
    let fr0 = h2-conn-feed conn rx 0 p-end tx 0 256 in
    let s0 = h2-frame-write-header-dma rx 0 256 (H2FrameHeader 12 h2-frame-settings h2-flag-none 0 0) in
    let s1 = h2-write-settings-entry-dma rx s0 256 h2-setting-enable-connect-protocol 1 in
    let s2 = h2-write-settings-entry-dma rx s1 256 h2-setting-max-frame-size 32768 in
    let fr1 = h2-conn-feed conn rx 0 s2 tx 0 256 in
    let ack-hdr = h2-frame-read-header-dma tx 0 in
    eq-u32 fr0.fr-status h2-feed-ok
      && eq-u32 fr0.fr-consumed 24
      && eq-u8 (h2-conn-state conn) h2-conn-open
      && eq-u32 fr1.fr-status h2-feed-ok
      && eq-u32 fr1.fr-tx-pos 9
      && eq-u8 ack-hdr.fh-type h2-frame-settings
      && eq-u8 ack-hdr.fh-flags h2-flag-ack
      && h2-conn-peer-connect-enabled conn
      && eq-u32 (h2-conn-peer-max-frame-size conn) 32768

  -- Helper: rapid-reset burst loop (`HEADERS(sid) + RST_STREAM(sid)`) until GOAWAY(ENHANCE_YOUR_CALM)
  rapid-reset-loop :: H2Conn -> I64 -> I64 -> U32 -> U32 ->{MVar} U32
  rapid-reset-loop conn rx tx sid remaining =
    if eq-u32 remaining 0 then 0
    else
      let h-end = write-simple-get-headers rx 0 256 sid (bit-or-u8 h2-flag-end-headers h2-flag-end-stream) in
      let f1 = h2-conn-feed conn rx 0 h-end tx 0 256 in
      if ne-u32 f1.fr-status h2-feed-headers-ready then f1.fr-error
      else
        let r-end = h2-write-rst-stream-dma rx 0 256 sid h2-err-cancel in
        let f2 = h2-conn-feed conn rx 0 r-end tx 0 256 in
        if eq-u32 f2.fr-status h2-feed-goaway then f2.fr-error
        else rapid-reset-loop conn rx tx (wrapping-add-u32 sid 2) (wrapping-sub-u32 remaining 1)

  -- Helper: write a DATA frame filled with byte `val`
  fill-bytes :: I64 -> U32 -> U8 -> U32 ->{MVar} Unit
  fill-bytes dst i v n =
    if ge-u32 i n then ()
    else
      let _ = raw-store-u8 (wrapping-add-i64 dst (u32-to-i64 i)) v in
      fill-bytes dst (wrapping-add-u32 i 1) v n

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 131072 4096 in
    if eq-u64 dma 0 then 99
    else
      let rx = wrapping-u64-to-i64 dma in
      let tx = wrapping-add-i64 rx 65536 in
      let body = wrapping-add-i64 tx 32768 in

      -- === Test A: 9-byte headroom DMA framing (`DmaPool` + `h2-frame-write-headroom`) ===
      let ok-headroom = (
        case dma-pool-new 4096 4 of
          None -> False
          Some pool ->
            case dma-pool-acquire pool of
              None -> let _ = dma-pool-destroy pool in False
              Some h ->
                let (full-sl, h1) = dma-buf-as-slice h in
                let res = (
                  case dma-slice-reserve-headroom full-sl 9 of
                    None -> False
                    Some (_, payload-sl) ->
                      let plen = copy-str-to-dma "hello-h2" payload-sl.slice-addr in
                      let framed = h2-frame-write-headroom payload-sl h2-frame-data h2-flag-end-stream 0x80000007 plen in
                      let dec-hdr = h2-frame-read-header-dma framed.slice-addr 0 in
                      eq-u32 framed.slice-off 0
                        && eq-u32 framed.slice-len (wrapping-add-u32 plen 9)
                        && eq-u32 dec-hdr.fh-length plen
                        && eq-u8 dec-hdr.fh-type h2-frame-data
                        && eq-u8 dec-hdr.fh-flags h2-flag-end-stream
                        && eq-u32 dec-hdr.fh-stream-id 7 -- reserved bit 31 cleared!
                        && eq-u8 (raw-load-u8 (wrapping-add-i64 framed.slice-addr 9)) 104) in -- 'h'
                let _ = dma-pool-release pool h1 in
                let _ = dma-pool-destroy pool in
                res) in

      -- === Test B: Preface partial NeedMore & mismatch rejection ===
      let c-bad-pref = h2-conn-new h2-role-server 16 in
      let _ = copy-str-to-dma "PRI * HTTP/2.0" rx in -- only 14 bytes
      let fp-need = h2-conn-feed c-bad-pref rx 0 14 tx 0 256 in
      let _ = copy-str-to-dma "GET / HTTP/1.1\r\nHost: x\r\n" rx in
      let fp-bad = h2-conn-feed c-bad-pref rx 0 24 tx 0 256 in
      let go-hdr-bad = h2-frame-read-header-dma tx 0 in
      let ok-preface = (
        eq-u32 fp-need.fr-status h2-feed-need-more
          && eq-u32 fp-bad.fr-status h2-feed-goaway
          && eq-u32 fp-bad.fr-error h2-err-protocol-error
          && eq-u8 go-hdr-bad.fh-type h2-frame-goaway) in
      let _ = h2-conn-destroy c-bad-pref in

      -- === Test C: HPACK + CONTINUATION reassembly, WINDOW_UPDATE auto-tuning, & response ===
      let c-main = h2-conn-new h2-role-server 32 in
      let ok-hs = handshake-server c-main rx tx in
      -- Stream 1: HEADERS without END_HEADERS (`0x82` = :method GET) + CONTINUATION(END_HEADERS) (`0x86 0x84`)
      let p0 = h2-frame-write-header-dma rx 0 256 (H2FrameHeader 1 h2-frame-headers h2-flag-none 0 1) in
      let _ = raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 p0)) 0x82 in
      let fc1 = h2-conn-feed c-main rx 0 (wrapping-add-u32 p0 1) tx 0 256 in
      let lock-mid = h2-conn-cont-stream-id c-main in
      let p1 = h2-frame-write-header-dma rx 0 256 (H2FrameHeader 2 h2-frame-continuation h2-flag-end-headers 0 1) in
      let _ = raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 p1)) 0x86 in
      let _ = raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 (wrapping-add-u32 p1 1))) 0x84 in
      let fc2 = h2-conn-feed c-main rx 0 (wrapping-add-u32 p1 2) tx 0 256 in
      let ok-cont = (
        eq-u32 fc1.fr-status h2-feed-ok
          && eq-u32 lock-mid 1
          && eq-u32 fc2.fr-status h2-feed-headers-ready
          && eq-u32 (h2-conn-cont-stream-id c-main) 0
          && eq-u32 fc2.fr-aux 3
          && eq-u32 (h2-conn-decoded-hdr-count c-main) 3) in

      -- WINDOW_UPDATE auto-tuning: send 3 x 16384B DATA frames (= 49152B >= 32767B half window!)
      let _ = fill-bytes (wrapping-add-i64 rx 9) 0 65 16384 in
      let _ = h2-frame-write-header-dma rx 0 64 (H2FrameHeader 16384 h2-frame-data h2-flag-none 0 1) in
      let fd1 = h2-conn-feed c-main rx 0 16393 tx 0 256 in
      let fd2 = h2-conn-feed c-main rx 0 16393 tx 0 256 in
      -- After 2nd frame (32768B >= 65535/2 = 32767), BOTH connection (sid=0) and stream (sid=1) WINDOW_UPDATE emitted!
      let wu0 = h2-frame-read-header-dma tx 0 in
      let wu1 = h2-frame-read-header-dma tx 13 in
      let wu0-inc = h2-read-be32 (wrapping-add-i64 tx 9) in
      let wu1-inc = h2-read-be32 (wrapping-add-i64 tx 22) in
      let ok-wu = (
        eq-u32 fd1.fr-status h2-feed-data-ready && eq-u32 fd1.fr-tx-pos 0
          && eq-u32 fd2.fr-status h2-feed-data-ready && eq-u32 fd2.fr-tx-pos 26
          && eq-u8 wu0.fh-type h2-frame-window-update && eq-u32 wu0.fh-stream-id 0 && eq-u32 wu0-inc 32768
          && eq-u8 wu1.fh-type h2-frame-window-update && eq-u32 wu1.fh-stream-id 1 && eq-u32 wu1-inc 32768
          && eq-i64 (h2-conn-recv-window c-main) 65535) in

      -- Send END_STREAM DATA on stream 1 and send response (closing stream 1), then verify closed-stream reopen fails!
      let _ = h2-frame-write-header-dma rx 0 64 (H2FrameHeader 0 h2-frame-data h2-flag-end-stream 0 1) in
      let fd3 = h2-conn-feed c-main rx 0 9 tx 0 256 in
      let blen = copy-str-to-dma "ok-h2" body in
      let wr1 = h2-conn-send-response c-main 1 200 body blen tx 0 256 in
      let rh0 = h2-frame-read-header-dma tx 0 in
      let rd0 = h2-frame-read-header-dma tx (wrapping-add-u32 9 rh0.fh-length) in
      let reopen-end = write-simple-get-headers rx 0 256 1 h2-flag-end-headers in
      let f-reopen = h2-conn-feed c-main rx 0 reopen-end tx 0 256 in
      let ok-reopen = (
        eq-u32 fd3.fr-status h2-feed-data-ready
          && eq-u32 wr1.wr-error h2-err-no-error && eq-u32 wr1.wr-bytes-sent blen
          && eq-u8 rh0.fh-type h2-frame-headers && eq-u32 rh0.fh-stream-id 1
          && eq-u8 rd0.fh-type h2-frame-data && eq-u8 rd0.fh-flags h2-flag-end-stream && eq-u32 rd0.fh-length blen
          && eq-u32 (h2-stream-table-active-count (h2-conn-streams c-main)) 0
          && eq-u32 f-reopen.fr-status h2-feed-goaway
          && eq-u32 f-reopen.fr-error h2-err-stream-closed) in
      let _ = h2-conn-destroy c-main in

      -- === Test D: CONTINUATION interleaving rejection (`GOAWAY(PROTOCOL_ERROR)`) ===
      let c-ci = h2-conn-new h2-role-server 16 in
      let _ = handshake-server c-ci rx tx in
      let h-ci = write-simple-get-headers rx 0 256 1 h2-flag-none in -- NO END_HEADERS!
      let f-ci1 = h2-conn-feed c-ci rx 0 h-ci tx 0 256 in
      -- Interleave a PING frame while CONTINUATION lock is held on stream 1!
      let p-ping = h2-write-ping-dma rx 0 256 h2-flag-none body in
      let f-ci2 = h2-conn-feed c-ci rx 0 p-ping tx 0 256 in
      let ok-interleave = (
        eq-u32 f-ci1.fr-status h2-feed-ok
          && eq-u32 f-ci2.fr-status h2-feed-goaway
          && eq-u32 f-ci2.fr-error h2-err-protocol-error) in
      let _ = h2-conn-destroy c-ci in

      -- === Test E: Flow-control overflow (`FLOW_CONTROL_ERROR`) & signed I64 SETTINGS decrease ===
      let c-fc = h2-conn-new h2-role-server 16 in
      let _ = handshake-server c-fc rx tx in
      let h-fc = write-simple-get-headers rx 0 256 1 h2-flag-end-headers in
      let _ = h2-conn-feed c-fc rx 0 h-fc tx 0 256 in
      -- Decrease SETTINGS_INITIAL_WINDOW_SIZE from 65535 to 100 after consuming 1000 bytes -> negative send window (-900)!
      let st-fc = h2-conn-streams c-fc in
      let sl-fc = h2-stream-lookup st-fc 1 in
      let _ = h2-stream-consume-send-window st-fc sl-fc 1000 in -- now 64535
      let d0 = h2-frame-write-header-dma rx 0 256 (H2FrameHeader 6 h2-frame-settings h2-flag-none 0 0) in
      let d1 = h2-write-settings-entry-dma rx d0 256 h2-setting-initial-window-size 100 in
      let f-dec = h2-conn-feed c-fc rx 0 d1 tx 0 256 in
      let neg-win = h2-stream-slot-send-window st-fc sl-fc in -- 64535 + (100 - 65535) = -900!
      -- Now overflow connection send window (> 0x7FFFFFFF) via WINDOW_UPDATE
      let wu-ovf = h2-write-window-update-dma rx 0 256 0 2147483647 in
      let f-ovf = h2-conn-feed c-fc rx 0 wu-ovf tx 0 256 in
      let ok-fc = (
        eq-u32 f-dec.fr-status h2-feed-ok
          && eq-i64 neg-win (-900)
          && eq-u32 f-ovf.fr-status h2-feed-goaway
          && eq-u32 f-ovf.fr-error h2-err-flow-control-error) in
      let _ = h2-conn-destroy c-fc in

      -- === Test F: RFC 9218 PRIORITY_UPDATE + PriorityScheduler across 3 concurrent streams ===
      let c-pr = h2-conn-new h2-role-server 16 in
      let _ = handshake-server c-pr rx tx in
      -- Open streams 1, 3, 5
      let _ = h2-conn-feed c-pr rx 0 (write-simple-get-headers rx 0 256 1 (bit-or-u8 h2-flag-end-headers h2-flag-end-stream)) tx 0 256 in
      let _ = h2-conn-feed c-pr rx 0 (write-simple-get-headers rx 0 256 3 (bit-or-u8 h2-flag-end-headers h2-flag-end-stream)) tx 0 256 in
      let _ = h2-conn-feed c-pr rx 0 (write-simple-get-headers rx 0 256 5 (bit-or-u8 h2-flag-end-headers h2-flag-end-stream)) tx 0 256 in
      -- Send PRIORITY_UPDATE frames: stream 1 => u=5, stream 3 => u=1, i (incremental), stream 5 => u=1, i (incremental)
      let pu1-len = copy-str-to-dma "u=5" (wrapping-add-i64 rx 13) in
      let _ = h2-write-be32 (wrapping-add-i64 rx 9) 1 in
      let _ = h2-frame-write-header-dma rx 0 64 (H2FrameHeader (wrapping-add-u32 4 pu1-len) h2-frame-priority-update h2-flag-none 0 0) in
      let _ = h2-conn-feed c-pr rx 0 (wrapping-add-u32 13 pu1-len) tx 0 256 in

      let pu3-len = copy-str-to-dma "u=1, i" (wrapping-add-i64 rx 13) in
      let _ = h2-write-be32 (wrapping-add-i64 rx 9) 3 in
      let _ = h2-frame-write-header-dma rx 0 64 (H2FrameHeader (wrapping-add-u32 4 pu3-len) h2-frame-priority-update h2-flag-none 0 0) in
      let _ = h2-conn-feed c-pr rx 0 (wrapping-add-u32 13 pu3-len) tx 0 256 in

      let pu5-len = copy-str-to-dma "u=1, i" (wrapping-add-i64 rx 13) in
      let _ = h2-write-be32 (wrapping-add-i64 rx 9) 5 in
      let _ = h2-frame-write-header-dma rx 0 64 (H2FrameHeader (wrapping-add-u32 4 pu5-len) h2-frame-priority-update h2-flag-none 0 0) in
      let _ = h2-conn-feed c-pr rx 0 (wrapping-add-u32 13 pu5-len) tx 0 256 in

      -- Queue 8-byte bodies on streams 1, 3, 5, then flush with 4-byte quantum -> must schedule 3, 5, 3, 5, then 1, 1!
      let _ = fill-bytes body 0 88 8 in
      let _ = h2-conn-queue-body c-pr 1 200 body 8 tx 0 256 in
      let _ = h2-conn-queue-body c-pr 3 200 body 8 tx 0 256 in
      let _ = h2-conn-queue-body c-pr 5 200 body 8 tx 0 256 in
      let q1 = h2-conn-flush-data c-pr 4 tx 0 256 in
      let q2 = h2-conn-flush-data c-pr 4 tx 0 256 in
      let q3 = h2-conn-flush-data c-pr 4 tx 0 256 in
      let q4 = h2-conn-flush-data c-pr 4 tx 0 256 in
      let q5 = h2-conn-flush-data c-pr 4 tx 0 256 in
      let q6 = h2-conn-flush-data c-pr 4 tx 0 256 in
      let q7 = h2-conn-flush-data c-pr 4 tx 0 256 in
      let ok-prio = (
        eq-u32 q1.wr-stream-id 3 && eq-u32 q2.wr-stream-id 5
          && eq-u32 q3.wr-stream-id 3 && eq-u32 q4.wr-stream-id 5
          && eq-u32 q5.wr-stream-id 1 && eq-u32 q6.wr-stream-id 1
          && eq-u32 q7.wr-stream-id 0
          && eq-u32 (h2-stream-table-active-count (h2-conn-streams c-pr)) 0) in
      let _ = h2-conn-destroy c-pr in

      -- === Test G: CVE-2023-44487 Rapid-Reset Flood Defense (`GOAWAY(ENHANCE_YOUR_CALM)`) ===
      let c-rr = h2-conn-new h2-role-server 32 in
      let _ = handshake-server c-rr rx tx in
      let _ = h2-conn-set-flood-limits c-rr 8 32 in
      let rr-err = rapid-reset-loop c-rr rx tx 1 20 in
      let ok-rr = (
        eq-u32 rr-err h2-err-enhance-your-calm
          && eq-u8 (h2-conn-state c-rr) h2-conn-goaway
          && eq-u32 (h2-conn-goaway-error c-rr) h2-err-enhance-your-calm
          && eq-u32 (h2-conn-rst-count c-rr) 9) in
      let _ = h2-conn-destroy c-rr in

      let _ = engine-dma-free dma 131072 in
      if not ok-headroom then 10
      else if not ok-preface then 11
      else if not ok-hs then 12
      else if not ok-cont then 13
      else if not ok-wu then 14
      else if not ok-reopen then 15
      else if not ok-interleave then 16
      else if not ok-fc then 17
      else if not ok-prio then 18
      else if not ok-rr then 19
      else 0
"#;
    build_and_run_leak_checked("rfc9113-conformance", src, 0);
}

// =========================================================================
// 4. 1,000-trial Effect.Test.Hegel differential torture test (M7 oracle vs
//    live DMA H2Conn + H2StreamTable)
// =========================================================================

#[test]
fn hegel_1000_trial_differential_http2_torture_test() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Effect.Test.Hegel
  import Engine.Types
  import Net.Http.V2.Frame
  import Net.Http.V2.Stream
  import Net.Http.V2.Conn
  import Net.Http.V2.Spec
  import Prim.Atomic

  -- Pure generators for Effect.Test.Hegel M7 machine verification
  gen-fin2-with :: Fin 2 -> Fin 2 -> Gen (Fin 2)
  gen-fin2-with f0 f1 = gen-one-of [gen-pure f0, gen-pure f1]

  gen-fault-kind :: Gen FaultKind
  gen-fault-kind = gen-one-of [
    gen-pure FaultContInterleave,
    gen-pure FaultClosedReopen,
    gen-pure FaultWindowOverflow
  ]

  h2-actions-with :: Fin 2 -> Fin 2 -> List (StepAction H2ModelState)
  h2-actions-with f0 f1 =
    let g2 = gen-fin2-with f0 f1 in [
      step-action "preface" h2-step-preface,
      step-action-gen "open-direct" g2 (\sid s -> h2-step-open-stream sid OpenDirect s),
      step-action-gen "open-cont" g2 (\sid s -> h2-step-open-stream sid OpenWithCont s),
      step-action "continuation" h2-step-continuation,
      step-action-gen "recv-end-stream" g2 h2-step-recv-end-stream,
      step-action "window-update" h2-step-window-update,
      step-action-gen "send-response" g2 h2-step-send-response,
      step-action-gen "rst-stream" g2 h2-step-rst-stream,
      step-action-gen "fault" gen-fault-kind h2-step-fault,
      step-action "close" h2-step-close
    ]

  -- Write a valid 1-byte or 3-byte HPACK GET header frame
  write-hdr-frame :: I64 -> U32 -> U8 -> U32 ->{MVar} U32
  write-hdr-frame rx sid flags len =
    let p = h2-frame-write-header-dma rx 0 256 (H2FrameHeader len h2-frame-headers flags 0 sid) in
    let _ = raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 p)) 0x82 in
    let _ = if ge-u32 len 2 then raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 (wrapping-add-u32 p 1))) 0x86 else () in
    let _ = if ge-u32 len 3 then raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 (wrapping-add-u32 p 2))) 0x84 else () in
    wrapping-add-u32 p len

  -- Map M7 model stream index (0 -> sid 1, 1 -> sid 3)
  model-sid :: Nat -> U32
  model-sid idx = if idx == 0 then 1 else 3

  -- Verify concrete H2Conn matches M7 H2ModelState on every step
  conn-matches-model :: H2Conn -> H2ModelState ->{MVar} Bool
  conn-matches-model conn s =
    let st = h2-conn-streams conn in
    let cs = h2-conn-state conn in
    let phase-ok = (
      case s.conn-phase of
        MPreface -> eq-u8 cs h2-conn-preface
        MOpen -> eq-u8 cs h2-conn-open
        MGoAway -> eq-u8 cs h2-conn-goaway
        MClosed -> True) in
    let lock-ok = (
      if s.cont-lock == 0 then eq-u32 (h2-conn-cont-stream-id conn) 0
      else if s.cont-lock == 1 then eq-u32 (h2-conn-cont-stream-id conn) 1
      else eq-u32 (h2-conn-cont-stream-id conn) 3) in
    let s1-active = h2-ms-eq s.s1 MSHeadersCont || h2-ms-eq s.s1 MSOpen || h2-ms-eq s.s1 MSHalfClosedRemote in
    let s2-active = h2-ms-eq s.s2 MSHeadersCont || h2-ms-eq s.s2 MSOpen || h2-ms-eq s.s2 MSHalfClosedRemote in
    let exp-active = (if s1-active then 1 else 0) + (if s2-active then 1 else 0) in
    let act-ok = (
      if h2-mphase-eq s.conn-phase MClosed || h2-mphase-eq s.conn-phase MGoAway then True
      else u32-to-nat (h2-stream-table-active-count st) == exp-active) in
    let rst-ok = u32-to-nat (h2-conn-rst-count conn) == s.rst-count in
    h2-safety-ok s && phase-ok && lock-ok && act-ok && rst-ok

  -- Execute one differential step (op in 0..10) against BOTH M7 model & real DMA H2Conn!
  diff-step :: Fin 2 -> Fin 2 -> H2Conn -> I64 -> I64 -> I64 -> Nat -> H2ModelState ->{MVar} H2ModelState
  diff-step f0 f1 conn rx tx body op s =
    if op == 0 then
      case h2-step-preface s of
        None -> s
        Some s1 ->
          let n = h2-preface-write-dma rx 0 256 in
          let _ = h2-conn-feed conn rx 0 n tx 0 256 in
          s1
    else if op == 1 || op == 2 then
      let fsid = if op == 1 then f0 else f1 in
      let sid = if op == 1 then 1 else 3 in
      case h2-step-open-stream fsid OpenDirect s of
        None -> s
        Some s1 ->
          let n = write-hdr-frame rx sid h2-flag-end-headers 3 in
          let _ = h2-conn-feed conn rx 0 n tx 0 256 in
          s1
    else if op == 3 || op == 4 then
      let fsid = if op == 3 then f0 else f1 in
      let sid = if op == 3 then 1 else 3 in
      case h2-step-open-stream fsid OpenWithCont s of
        None -> s
        Some s1 ->
          let n = write-hdr-frame rx sid h2-flag-none 1 in
          let _ = h2-conn-feed conn rx 0 n tx 0 256 in
          s1
    else if op == 5 then
      case h2-step-continuation s of
        None -> s
        Some s1 ->
          let sid = if s.cont-lock == 1 then 1 else 3 in
          let p = h2-frame-write-header-dma rx 0 256 (H2FrameHeader 2 h2-frame-continuation h2-flag-end-headers 0 sid) in
          let _ = raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 p)) 0x86 in
          let _ = raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 (wrapping-add-u32 p 1))) 0x84 in
          let _ = h2-conn-feed conn rx 0 (wrapping-add-u32 p 2) tx 0 256 in
          s1
    else if op == 6 then
      let fsid = if h2-ms-eq s.s1 MSOpen then f0 else f1 in
      let sid = if h2-ms-eq s.s1 MSOpen then 1 else 3 in
      case h2-step-recv-end-stream fsid s of
        None -> s
        Some s1 ->
          let n = h2-frame-write-header-dma rx 0 256 (H2FrameHeader 0 h2-frame-data h2-flag-end-stream 0 sid) in
          let _ = h2-conn-feed conn rx 0 n tx 0 256 in
          s1
    else if op == 7 then
      case h2-step-window-update s of
        None -> s
        Some s1 ->
          let n = h2-write-window-update-dma rx 0 256 0 1024 in
          let _ = h2-conn-feed conn rx 0 n tx 0 256 in
          s1
    else if op == 8 then
      let fsid = if h2-ms-eq s.s1 MSOpen || h2-ms-eq s.s1 MSHalfClosedRemote then f0 else f1 in
      let sid = if h2-ms-eq s.s1 MSOpen || h2-ms-eq s.s1 MSHalfClosedRemote then 1 else 3 in
      case h2-step-send-response fsid s of
        None -> s
        Some s1 ->
          -- Close cleanly: if stream was still MSOpen, send END_STREAM first on wire so server closes slot on response
          let _ = (
            if (sid == 1 && h2-ms-eq s.s1 MSOpen) || (sid == 3 && h2-ms-eq s.s2 MSOpen) then
              let dn = h2-frame-write-header-dma rx 0 256 (H2FrameHeader 0 h2-frame-data h2-flag-end-stream 0 sid) in
              let _ = h2-conn-feed conn rx 0 dn tx 0 256 in
              ()
            else ()) in
          let _ = h2-conn-send-response conn sid 200 body 4 tx 0 256 in
          s1
    else if op == 9 then
      let fsid = if h2-ms-eq s.s1 MSOpen || h2-ms-eq s.s1 MSHalfClosedRemote then f0 else f1 in
      let sid = if h2-ms-eq s.s1 MSOpen || h2-ms-eq s.s1 MSHalfClosedRemote then 1 else 3 in
      case h2-step-rst-stream fsid s of
        None -> s
        Some s1 ->
          let n = h2-write-rst-stream-dma rx 0 256 sid h2-err-cancel in
          let _ = h2-conn-feed conn rx 0 n tx 0 256 in
          s1
    else if op == 10 then
      -- Protocol faults: CONTINUATION interleave / closed-stream reopen / window overflow
      if s.cont-lock /= 0 then
        case h2-step-fault FaultContInterleave s of
          None -> s
          Some s1 ->
            let n = h2-write-ping-dma rx 0 256 h2-flag-none body in
            let _ = h2-conn-feed conn rx 0 n tx 0 256 in
            s1
      else if s.s1-closed-ever || s.s2-closed-ever then
        case h2-step-fault FaultClosedReopen s of
          None -> s
          Some s1 ->
            let bad-sid = if s.s1-closed-ever then 1 else 3 in
            let n = write-hdr-frame rx bad-sid h2-flag-end-headers 3 in
            let _ = h2-conn-feed conn rx 0 n tx 0 256 in
            s1
      else
        case h2-step-fault FaultWindowOverflow s of
          None -> s
          Some s1 ->
            let n = h2-write-window-update-dma rx 0 256 0 2147483647 in
            let _ = h2-conn-feed conn rx 0 n tx 0 256 in
            s1
    else
      case h2-step-close s of
        None -> s
        Some s1 -> s1

  run-diff-trace :: Fin 2 -> Fin 2 -> H2Conn -> I64 -> I64 -> I64 -> U64 -> Nat -> H2ModelState ->{MVar} Bool
  run-diff-trace f0 f1 conn rx tx body rng steps s =
    if not (conn-matches-model conn s) then False
    else if steps == 0 then True
    else
      let rng1 = wrapping-add-u64 (wrapping-mul-u64 rng 6364136223846793005) 1442695040888963407 in
      let op = u32-to-nat (wrapping-u64-to-u32 (rem-u64 (wrapping-shr-u64 rng1 33) 12)) in
      let s1 = diff-step f0 f1 conn rx tx body op s in
      run-diff-trace f0 f1 conn rx tx body rng1 (steps - 1) s1

  run-diff-trials :: Fin 2 -> Fin 2 -> I64 -> I64 -> I64 -> U64 -> U64 ->{IO, MVar} Bool
  run-diff-trials f0 f1 rx tx body trial total =
    if ge-u64 trial total then True
    else
      let conn = h2-conn-new h2-role-server 8 in
      if not (h2-conn-valid conn) then False
      else
        let _ = h2-conn-set-flood-limits conn 1 64 in -- matches M7 rst-count >= 2 => GOAWAY!
        let seed = wrapping-mul-u64 (wrapping-add-u64 trial 1) 0x9E3779B97F4A7C15 in
        let ok = run-diff-trace f0 f1 conn rx tx body seed 20 h2-model-init in
        let _ = h2-conn-destroy conn in
        if ok then run-diff-trials f0 f1 rx tx body (wrapping-add-u64 trial 1) total
        else False

  main :: Unit ->{IO, MVar} Int
  main _ =
    case decide (0 < 2) of
      No _ -> 90
      Yes w0 -> case decide (1 < 2) of
        No _ -> 91
        Yes w1 ->
          let f0 = Fin 0 w0 in
          let f1 = Fin 1 w1 in
          -- 1. 1,000-trial Effect.Test.Hegel machine check over M7 Http2Machine
          let cfg = HegelConfig 1000 256 60 (hegel-seed 9113) in
          let acts = h2-actions-with f0 f1 in
          case hegel-check-machine-with cfg 28 h2-model-init acts h2-safety-ok of
            Err _ -> 1
            Ok _ ->
              -- 2. 1,000-trial differential torture test over real DMA H2Conn vs M7 oracle
              let dma = engine-dma-alloc 8192 4096 in
              if eq-u64 dma 0 then 2
              else
                let rx = wrapping-u64-to-i64 dma in
                let tx = wrapping-add-i64 rx 4096 in
                let body = wrapping-add-i64 tx 2048 in
                let _ = raw-store-u32 body 0x41424344 in
                let ok = run-diff-trials f0 f1 rx tx body 0 1000 in
                let _ = engine-dma-free dma 8192 in
                if ok then 0 else 3
"#;
    build_and_run_leak_checked("hegel-1000-diff", src, 0);
}

// =========================================================================
// 5. Steady-state 25,000 multiplexed request/response stream cycles:
//    allocs == 0, atomic_rc == 0, CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1
// =========================================================================

#[test]
fn steady_state_25000_multiplexed_streams_zero_alloc_zero_atomic_rc() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Engine.Types
  import Net.Http.V2.Frame
  import Net.Http.V2.Stream
  import Net.Http.V2.Conn
  import Prim.Atomic

  -- Pre-populate a 4-stream multiplexed batch (`HEADERS(END_HEADERS|END_STREAM)`)
  -- for stream IDs `base-sid, base-sid+2, base-sid+4, base-sid+6` (4 x 12 = 48 bytes)
  -- and feed + respond to all 4 streams with 0 heap allocations!
  #[no-alloc]
  write-batch4 :: I64 -> U32 ->{MVar} U32
  write-batch4 rx sid0 =
    let flags = bit-or-u8 h2-flag-end-headers h2-flag-end-stream in
    let p0 = h2-frame-write-header-dma rx 0 256 (H2FrameHeader 3 h2-frame-headers flags 0 sid0) in
    let _ = raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 p0)) 0x82 in
    let _ = raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 (wrapping-add-u32 p0 1))) 0x86 in
    let _ = raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 (wrapping-add-u32 p0 2))) 0x84 in
    let o1 = wrapping-add-u32 p0 3 in
    let p1 = h2-frame-write-header-dma rx o1 256 (H2FrameHeader 3 h2-frame-headers flags 0 (wrapping-add-u32 sid0 2)) in
    let _ = raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 p1)) 0x82 in
    let _ = raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 (wrapping-add-u32 p1 1))) 0x86 in
    let _ = raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 (wrapping-add-u32 p1 2))) 0x84 in
    let o2 = wrapping-add-u32 p1 3 in
    let p2 = h2-frame-write-header-dma rx o2 256 (H2FrameHeader 3 h2-frame-headers flags 0 (wrapping-add-u32 sid0 4)) in
    let _ = raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 p2)) 0x82 in
    let _ = raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 (wrapping-add-u32 p2 1))) 0x86 in
    let _ = raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 (wrapping-add-u32 p2 2))) 0x84 in
    let o3 = wrapping-add-u32 p2 3 in
    let p3 = h2-frame-write-header-dma rx o3 256 (H2FrameHeader 3 h2-frame-headers flags 0 (wrapping-add-u32 sid0 6)) in
    let _ = raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 p3)) 0x82 in
    let _ = raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 (wrapping-add-u32 p3 1))) 0x86 in
    let _ = raw-store-u8 (wrapping-add-i64 rx (u32-to-i64 (wrapping-add-u32 p3 2))) 0x84 in
    wrapping-add-u32 p3 3

  -- 6,250 batches x 4 concurrent multiplexed streams = 25,000 stream cycles!
  -- Every single call inside this loop is `#[no-alloc]`!
  #[no-alloc]
  hot-loop25000 :: H2Conn -> I64 -> I64 -> I64 -> U32 -> U32 -> U32 ->{MVar} U32
  hot-loop25000 conn rx tx body sid done limit =
    if ge-u32 done limit then done
    else
      let rx-len = write-batch4 rx sid in
      -- Feed all 4 concurrent request HEADERS frames before sending any response (true multiplexing!)
      let f0 = h2-conn-feed conn rx 0 rx-len tx 0 2048 in
      let o1 = f0.fr-consumed in
      let f1 = h2-conn-feed conn rx o1 rx-len tx 0 2048 in
      let o2 = wrapping-add-u32 o1 f1.fr-consumed in
      let f2 = h2-conn-feed conn rx o2 rx-len tx 0 2048 in
      let o3 = wrapping-add-u32 o2 f2.fr-consumed in
      let f3 = h2-conn-feed conn rx o3 rx-len tx 0 2048 in
      if ne-u32 f0.fr-status h2-feed-headers-ready then 0
      else if ne-u32 f1.fr-status h2-feed-headers-ready then 0
      else if ne-u32 f2.fr-status h2-feed-headers-ready then 0
      else if ne-u32 f3.fr-status h2-feed-headers-ready then 0
      else
        -- Send responses out-of-order (sid+4, sid, sid+6, sid+2) to stress open-addressed backshift deletion!
        let r0 = h2-conn-send-response conn (wrapping-add-u32 sid 4) 200 body 8 tx 0 2048 in
        let r1 = h2-conn-send-response conn sid 200 body 8 tx r0.wr-tx-pos 2048 in
        let r2 = h2-conn-send-response conn (wrapping-add-u32 sid 6) 200 body 8 tx r1.wr-tx-pos 2048 in
        let r3 = h2-conn-send-response conn (wrapping-add-u32 sid 2) 200 body 8 tx r2.wr-tx-pos 2048 in
        if ne-u32 r3.wr-error h2-err-no-error then 0
        else
          -- Periodically send connection WINDOW_UPDATE so 25,000 x 8B = 200 KiB never stalls conn send window
          let _ = (
            if lt-i64 (h2-conn-send-window conn) 16384 then
              let wu-len = h2-write-window-update-dma rx 0 64 0 65536 in
              let _ = h2-conn-feed conn rx 0 wu-len tx 0 256 in
              let _ = h2-conn-refill-budgets conn 0 1 in
              ()
            else ()) in
          hot-loop25000 conn rx tx body (wrapping-add-u32 sid 8) (wrapping-add-u32 done 4) limit

  main :: Unit ->{IO, MVar} Int
  main _ =
    let conn = h2-conn-new h2-role-server 64 in
    if not (h2-conn-valid conn) then 1
    else
      let dma = engine-dma-alloc 8192 4096 in
      if eq-u64 dma 0 then
        let _ = h2-conn-destroy conn in 2
      else
        let rx = wrapping-u64-to-i64 dma in
        let tx = wrapping-add-i64 rx 4096 in
        let body = wrapping-add-i64 tx 2048 in
        let _ = raw-store-u64 body 0x2132682D676E6F70 in -- "pong-h2!"
        -- Handshake preface once before entering steady-state zero-alloc measurement
        let plen = h2-preface-write-dma rx 0 64 in
        let fp = h2-conn-feed conn rx 0 plen tx 0 256 in
        if ne-u32 fp.fr-status h2-feed-ok then
          let _ = engine-dma-free dma 8192 in
          let _ = h2-conn-destroy conn in 3
        else
          let _ = engine-thread-counters-reset () in
          let completed = hot-loop25000 conn rx tx body 1 0 25000 in
          let allocs = engine-thread-alloc-count () in
          let atomic-rcs = engine-thread-atomic-rc-count () in
          let st = h2-conn-streams conn in
          let active = h2-stream-table-active-count st in
          let opened = h2-stream-table-total-opened st in
          let closed = h2-stream-table-total-closed st in
          let _ = engine-dma-free dma 8192 in
          let _ = h2-conn-destroy conn in
          if ne-u32 completed 25000 then 10
          else if ne-u64 allocs 0 then 11
          else if ne-u64 atomic-rcs 0 then 12
          else if ne-u32 active 0 then 13
          else if ne-u64 opened 25000 then 14
          else if ne-u64 closed 25000 then 15
          else 0
"#;
    build_and_run_leak_checked("steady-25000-zero-alloc", src, 0);
}
