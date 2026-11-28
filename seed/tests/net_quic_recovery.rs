//! Integration, RFC 9002 compliance, BBRv2 + CUBIC 10 Gbps / 20 ms RTT
//! discrete-event bottleneck simulation, ABI layout, `#[no-alloc]`, and
//! 50,000-iteration zero-allocation / zero-atomic-RC / leak-checked tests for
//! `Net.Quic.{AckMap,Recovery,Bbr,Pacer}`.
//!
//! Routed through `common::project` / `common::drive_with_profile` /
//! `common::check_interfaces` / `common::build_executable` so the exact same
//! suite runs against both the Rust seed and the Stage 3 self-hosted compiler.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{Mode, Package};
use std::process::Command;

const NET_QUIC_ACKMAP: &str = include_str!("../prelude/Net/Quic/AckMap.cinch");
const NET_QUIC_RECOVERY: &str = include_str!("../prelude/Net/Quic/Recovery.cinch");
const NET_QUIC_BBR: &str = include_str!("../prelude/Net/Quic/Bbr.cinch");
const NET_QUIC_PACER: &str = include_str!("../prelude/Net/Quic/Pacer.cinch");
const ENGINE_TYPES: &str = include_str!("../prelude/Engine/Types.cinch");
const PRIM_ATOMIC: &str = include_str!("../prelude/Prim/Atomic.cinch");
const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn project_with_quic_recovery(tag: &str, main_src: &str) -> std::path::PathBuf {
    common::project(
        tag,
        &[
            ("Prim.Atomic.cinch", PRIM_ATOMIC),
            ("Engine.Types.cinch", ENGINE_TYPES),
            ("Net.Quic.AckMap.cinch", NET_QUIC_ACKMAP),
            ("Net.Quic.Recovery.cinch", NET_QUIC_RECOVERY),
            ("Net.Quic.Bbr.cinch", NET_QUIC_BBR),
            ("Net.Quic.Pacer.cinch", NET_QUIC_PACER),
            ("Main.cinch", main_src),
        ],
    )
}

fn run_leak_checked(tag: &str, src: &str, expected: i32) -> (String, String) {
    let dir = project_with_quic_recovery(tag, src);
    let exe = common::build_executable(&dir, "Main.cinch", "net-quic-recovery-test")
        .unwrap_or_else(|e| panic!("{tag}: build failed:\n{e}"));
    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "128M")
        .output()
        .expect("run built binary");
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    common::cleanup(&dir);
    assert_eq!(
        out.status.code(),
        Some(expected),
        "{tag}: expected exit {expected}, got {:?} (status: {:?});\nstdout:\n{stdout}\nstderr:\n{stderr}",
        out.status.code(),
        out.status
    );
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "{tag}: expected clean leak report (`leaked=0`); stderr:\n{stderr}"
    );
    (stdout, stderr)
}

// =========================================================================
// 1. Manifest registration, direct stdlib import, ABI layouts & #[no-alloc]
// =========================================================================

#[test]
fn net_quic_recovery_manifest_abi_and_no_alloc() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, path) in [
        ("Net.Quic.AckMap", "Net/Quic/AckMap.cinch"),
        ("Net.Quic.Recovery", "Net/Quic/Recovery.cinch"),
        ("Net.Quic.Bbr", "Net/Quic/Bbr.cinch"),
        ("Net.Quic.Pacer", "Net/Quic/Pacer.cinch"),
    ] {
        let entry = pkg
            .modules
            .iter()
            .find(|m| m.name == name)
            .unwrap_or_else(|| panic!("{name} missing from seed/prelude/.cinchpkg modules"));
        assert_eq!(entry.path, path);
        assert!(
            pkg.order.iter().any(|m| m == name),
            "{name} missing from seed/prelude/.cinchpkg order"
        );
    }

    // Direct stdlib import without copying module sources into temp project.
    let dir_std = common::project(
        "quic-recovery-stdlib-resolve",
        &[(
            "Main.cinch",
            r#"
module Main where
  import Data.Machine
  import Data.Option
  import Net.Quic.AckMap
  import Net.Quic.Recovery
  import Net.Quic.Bbr
  import Net.Quic.Pacer

  main :: Unit ->{IO, MVar} Int
  main _ =
    let am = ack-map-on-recv (ack-map-empty ()) 7 1000 1 in
    let ps = pacer-init 1200 1000 in
    case (recovery-new 64, bbr-new 1200 1000, cubic-new 1200) of
      (Some rec, Some bbr, Some cub) ->
        let ok = (eq-u64 (ack-map-largest am) 7
          && eq-u32 (pacer-max-burst-bytes ps) 12000
          && eq-u8 (bbr-mode-u8 bbr) bbr-mode-startup
          && eq-u64 (cubic-cwnd cub) 12000) in
        let _ = recovery-destroy rec in
        let _ = bbr-destroy bbr in
        let _ = cubic-destroy cub in
        if ok then 0 else 2
      _ -> 1
"#,
        )],
    );
    let code = common::drive_with_profile(
        Mode::Run,
        StageMode::Refresh,
        &dir_std,
        "Main.cinch",
        None,
        &TargetProfile::native(),
    )
    .expect("direct stdlib import of Net.Quic.{AckMap,Recovery,Bbr,Pacer}");
    common::cleanup(&dir_std);
    assert_eq!(code, 0);

    // Check ABI layouts and #[no-alloc] assertions across all 4 modules.
    let src = r#"
module Main where
  import Data.Machine
  import Net.Quic.AckMap
  import Net.Quic.Recovery
  import Net.Quic.Bbr
  import Net.Quic.Pacer

  main :: Int
  main =
    let am = ack-map-empty () in
    let rtt = rtt-estimator-init 0 in
    let pe = sent-packet-entry-empty () in
    let ps = pacer-init 1200 0 in
    if eq-u64 am.ack-largest 0
      && eq-u64 rtt.rtt-smoothed-ns quic-initial-rtt-ns
      && eq-u8 pe.pkt-status pkt-status-empty
      && eq-u32 ps.pacer-mtu-val 1200
    then 0 else 1
"#;
    let dir = project_with_quic_recovery("quic-recovery-abi", src);
    let ifaces = common::check_interfaces(&dir, "Main.cinch").expect("check_interfaces");

    let iface_of = |name: &str| {
        ifaces
            .iter()
            .find(|i| i.module == name)
            .unwrap_or_else(|| panic!("missing interface for {name}"))
    };
    fn layout_of<'a>(
        iface: &'a cinch::interface::Iface,
        name: &str,
    ) -> &'a cinch::interface::AbiLayout {
        iface
            .layouts
            .iter()
            .find(|l| l.name == name)
            .unwrap_or_else(|| panic!("missing layout for {name} in {}", iface.module))
    }
    let assert_no_alloc = |iface: &cinch::interface::Iface, fn_name: &str| {
        let entry = iface
            .assertions
            .iter()
            .find(|a| a.name == fn_name)
            .unwrap_or_else(|| panic!("missing assertion for {}.{}", iface.module, fn_name));
        assert!(
            entry.facts.iter().any(|f| f == "no-alloc"),
            "expected no-alloc fact on {}.{}, got {:?}",
            iface.module,
            fn_name,
            entry.facts
        );
    };

    let ack_iface = iface_of("Net.Quic.AckMap");
    let rec_iface = iface_of("Net.Quic.Recovery");
    let bbr_iface = iface_of("Net.Quic.Bbr");
    let pac_iface = iface_of("Net.Quic.Pacer");

    assert_eq!(layout_of(ack_iface, "AckMap").size, 56);
    assert_eq!(layout_of(rec_iface, "SentPacketEntry").size, 48);
    assert_eq!(layout_of(rec_iface, "RttEstimator").size, 48);
    assert_eq!(layout_of(pac_iface, "PacerState").size, 48);

    for f in [
        "ack-map-empty",
        "ack-map-is-duplicate",
        "ack-map-has-pn",
        "ack-map-on-recv",
        "ack-map-on-recv-checked",
        "ack-map-first-range",
        "ack-map-extract-ranges-raw",
    ] {
        assert_no_alloc(ack_iface, f);
    }

    for f in [
        "rtt-estimator-init",
        "rtt-estimator-update",
        "rtt-estimator-pto-base-ns",
        "rtt-estimator-loss-delay-ns",
        "recovery-on-packet-sent",
        "recovery-on-ack-received",
        "recovery-on-ack-ranges-received",
        "recovery-pto-duration-ns",
        "recovery-loss-time-ns",
        "recovery-next-timeout-ns",
        "recovery-on-loss-timeout",
    ] {
        assert_no_alloc(rec_iface, f);
    }

    for f in [
        "bbr-on-ack",
        "bbr-on-ecn",
        "bbr-on-persistent-congestion",
        "bbr-cwnd",
        "bbr-pacing-rate",
        "bbr-bdp",
        "cubic-on-ack",
        "cubic-on-loss",
        "cubic-on-persistent-congestion",
        "cubic-cwnd",
        "cubic-pacing-rate",
        "cubic-icbrt-u64",
    ] {
        assert_no_alloc(bbr_iface, f);
    }

    for f in [
        "pacer-init",
        "pacer-refill",
        "pacer-next-send-time-ns",
        "pacer-next-burst-time-ns",
        "pacer-can-send-now",
        "pacer-burst-budget",
        "pacer-on-packet-sent",
    ] {
        assert_no_alloc(pac_iface, f);
    }

    assert_eq!(common::run(&dir, "Main.cinch"), 0);
    common::cleanup(&dir);
}

// =========================================================================
// 2. AckMap: 256-bit sliding window, reorder, duplicate, large jump,
//    and RFC 9000 multi-gap ACK range extraction across all 4 U64 words
// =========================================================================

#[test]
fn ack_map_sliding_window_reorder_dup_large_jump_and_multi_gap_ranges() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Engine.Types
  import Net.Quic.AckMap
  import Prim.Atomic

  load-u64 :: I64 -> U64 ->{MVar} U64
  load-u64 base off = raw-load-u64 (wrapping-add-i64 base (wrapping-u64-to-i64 off))

  -- Insert contiguous range [lo .. hi] into AckMap.
  insert-range :: AckMap -> U64 -> U64 -> U64 -> AckMap
  insert-range m lo hi ts =
    if gt-u64 lo hi then m
    else insert-range (ack-map-on-recv m lo ts 1) (wrapping-add-u64 lo 1) hi ts

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 4096 4096 in
    if eq-u64 dma 0 then 99
    else
      let scratch = wrapping-u64-to-i64 dma in
      -- 1. Empty map checks
      let m0 = ack-map-empty () in
      let ok0 = (not (ack-map-has-any m0)
        && not (ack-map-is-duplicate m0 0)
        && eq-u64 (ack-map-first-range m0) 0
        && eq-u32 (ack-map-extract-ranges-raw m0 scratch 16) 0) in

      -- 2. Out-of-order receipt + duplicate rejection + ack-eliciting counter
      let r1 = ack-map-on-recv-checked m0 10 1000 1 in
      let m1 = ack-recv-map r1 in
      let r2 = ack-map-on-recv-checked m1 8 1100 1 in
      let m2 = ack-recv-map r2 in
      let r3 = ack-map-on-recv-checked m2 9 1200 0 in
      let m3 = ack-recv-map r3 in
      -- Duplicate PN 9 -> must be rejected and NOT bump ack-eliciting-pending!
      let r-dup = ack-map-on-recv-checked m3 9 1300 1 in
      let m4 = ack-recv-map r-dup in
      let r5 = ack-map-on-recv-checked m4 5 1400 1 in
      let m5 = ack-recv-map r5 in
      let r6 = ack-map-on-recv-checked m5 6 1500 1 in
      let m6 = ack-recv-map r6 in
      -- Present set: {10, 9, 8, 6, 5}, missing: {7}
      -- Expected: largest=10, first_range=2 ([8..10]), 1 extra range: gap=0 (1 missing: 7), range_len=1 ([5..6])
      let cnt1 = ack-map-extract-ranges-raw m6 scratch 16 in
      let ok1 = (ack-recv-accepted r1 && ack-recv-accepted r2 && ack-recv-accepted r3
        && not (ack-recv-accepted r-dup)
        && ack-recv-accepted r5 && ack-recv-accepted r6
        && eq-u64 (ack-map-largest m6) 10
        && eq-u64 (ack-map-largest-recv-ns m6) 1000
        && eq-u32 (ack-map-ack-eliciting-pending m6) 4
        && ack-map-is-duplicate m6 10 && ack-map-is-duplicate m6 8 && not (ack-map-is-duplicate m6 7)
        && eq-u64 (ack-map-first-range m6) 2
        && eq-u32 cnt1 1
        && eq-u64 (load-u64 scratch 0) 0
        && eq-u64 (load-u64 scratch 8) 1) in

      -- 3. 256-packet window boundary & no false replay acceptance!
      -- Receive largest=300, then test pn=45 (diff=255 -> INSIDE window) and pn=44 (diff=256 -> TOO OLD!)
      let mw0 = ack-map-on-recv (ack-map-empty ()) 300 5000 1 in
      let rw-old = ack-map-on-recv-checked mw0 44 5100 1 in
      let rw-edge = ack-map-on-recv-checked mw0 45 5200 1 in
      let mw1 = ack-recv-map rw-edge in
      let rw-edge-dup = ack-map-on-recv-checked mw1 45 5300 1 in
      let ok2 = (not (ack-recv-accepted rw-old)
        && ack-map-is-duplicate mw0 44
        && not (ack-map-has-pn mw0 44)
        && ack-recv-accepted rw-edge
        && ack-map-has-pn mw1 45
        && not (ack-recv-accepted rw-edge-dup)) in

      -- 4. Large jump > 256: jump from 300 -> 1000 resets old bits cleanly
      let mj1 = ack-map-on-recv mw1 1000 9000 1 in
      let mj2 = ack-map-on-recv mj1 999 9100 1 in
      let rj-old = ack-map-on-recv-checked mj2 300 9200 1 in
      let ok3 = (eq-u64 (ack-map-largest mj2) 1000
        && eq-u64 (ack-map-first-range mj2) 1
        && ack-map-has-pn mj2 1000 && ack-map-has-pn mj2 999
        && not (ack-map-has-pn mj2 300)
        && not (ack-recv-accepted rj-old)
        && eq-u32 (ack-map-extract-ranges-raw mj2 scratch 16) 0) in

      -- 5. Multi-gap extraction spanning all 4 U64 words (bit indices 0..255)!
      -- Construct largest = 500 with 4 ranges across word boundaries:
      --   Range 0 (first): [498..500] (bits 0..2, first_range = 2)
      --   Gap 0: missing [435..497] -> 63 missing => gap = 62
      --   Range 1: [430..434] (bits 66..70, crosses into w1! 5 pkts => len = 4)
      --   Gap 1: missing [368..429] -> 62 missing => gap = 61
      --   Range 2: [365..367] (bits 133..135, in w2! 3 pkts => len = 2)
      --   Gap 2: missing [246..364] -> 119 missing => gap = 118
      --   Range 3: [245..245] (bit 255, top bit of w3! 1 pkt => len = 0)
      let mg0 = insert-range (ack-map-empty ()) 245 245 10000 in
      let mg1 = insert-range mg0 365 367 10100 in
      let mg2 = insert-range mg1 430 434 10200 in
      let mg3 = insert-range mg2 498 500 10300 in
      let cnt-mg = ack-map-extract-ranges-raw mg3 scratch 16 in
      let ok4 = (eq-u64 (ack-map-largest mg3) 500
        && eq-u64 (ack-map-first-range mg3) 2
        && eq-u32 cnt-mg 3
        && eq-u64 (load-u64 scratch 0) 62 && eq-u64 (load-u64 scratch 8) 4
        && eq-u64 (load-u64 scratch 16) 61 && eq-u64 (load-u64 scratch 24) 2
        && eq-u64 (load-u64 scratch 32) 118 && eq-u64 (load-u64 scratch 40) 0) in

      let _ = engine-dma-free dma 4096 in
      if not ok0 then 1
      else if not ok1 then 2
      else if not ok2 then 3
      else if not ok3 then 4
      else if not ok4 then 5
      else 0
"#;
    run_leak_checked("ack-map-full", src, 0);
}

// =========================================================================
// 3. RFC 9002: RTT estimation, packet-threshold (3), time-threshold (9/8 RTT),
//    PTO exponential backoff, persistent congestion (>= 3 * PTO), & AckMap bridge
// =========================================================================

#[test]
fn rfc9002_rtt_loss_thresholds_pto_backoff_and_persistent_congestion() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Data.Option
  import Engine.Types
  import Net.Quic.AckMap
  import Net.Quic.Recovery
  import Prim.Atomic

  ms :: U64 -> U64
  ms x = wrapping-mul-u64 x 1000000

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 4096 4096 in
    if eq-u64 dma 0 then 99
    else
      let scratch = wrapping-u64-to-i64 dma in
      case recovery-new 256 of
        None ->
          let _ = engine-dma-free dma 4096 in
          98
        Some rec ->
          -- 1. Initial RTT & PTO check (RFC 9002: 333ms + 4*166.5ms + 25ms = 1024ms)
          let rtt0 = recovery-rtt rec in
          let pto0 = recovery-pto-duration-ns rec in
          let ok-init = (eq-u64 rtt0.rtt-smoothed-ns (ms 333)
            && eq-u64 rtt0.rtt-var-ns 166500000
            && eq-u64 pto0 (ms 1024)) in

          -- 2. Send PNs 0..5 at t=0..5ms, ACK [3..5] at t=25ms (latest_rtt = 25 - 5 = 20ms)
          --    Packet threshold = 3 => PNs 0, 1, 2 (0+3<=5, 1+3<=5, 2+3<=5) MUST be declared lost!
          let _ = recovery-on-packet-sent rec pn-space-app 0 1200 (ms 0) 1 1 in
          let _ = recovery-on-packet-sent rec pn-space-app 1 1200 (ms 1) 1 1 in
          let _ = recovery-on-packet-sent rec pn-space-app 2 1200 (ms 2) 1 1 in
          let _ = recovery-on-packet-sent rec pn-space-app 3 1200 (ms 3) 1 1 in
          let _ = recovery-on-packet-sent rec pn-space-app 4 1200 (ms 4) 1 1 in
          let _ = recovery-on-packet-sent rec pn-space-app 5 1200 (ms 5) 1 1 in
          let bif-before = recovery-bytes-in-flight rec in
          let o1 = recovery-on-ack-received rec pn-space-app 5 2 0 (ms 25) in
          let rtt1 = recovery-rtt rec in
          let ok-pkt-thresh = (eq-u64 bif-before 7200
            && eq-u32 o1.ack-out-acked-packets 3
            && eq-u64 o1.ack-out-acked-bytes 3600
            && eq-u32 o1.ack-out-lost-packets 3
            && eq-u64 o1.ack-out-lost-bytes 3600
            && eq-u64 (recovery-bytes-in-flight rec) 0
            && eq-u8 o1.ack-out-rtt-updated 1
            && eq-u64 rtt1.rtt-latest-ns (ms 20)
            && eq-u64 rtt1.rtt-smoothed-ns (ms 20)
            && eq-u64 rtt1.rtt-min-ns (ms 20)
            && eq-u64 rtt1.rtt-var-ns (ms 10)) in

          -- 3. RTT ack_delay adjustment (clamped to max_ack_delay = 25ms)
          --    Send PN 6 at t=100ms, ACK at t=130ms (latest_rtt = 30ms) with ack_delay = 50ms ->
          --    clamped to 25ms, but 30ms - 25ms = 5ms < min_rtt (20ms), so adjusted_rtt stays 30ms!
          --    Then send PN 7 at t=200ms, ACK at t=230ms (latest_rtt = 30ms) with ack_delay = 8ms ->
          --    30ms - 8ms = 22ms >= min_rtt (20ms), so adjusted_rtt = 22ms!
          let _ = recovery-on-packet-sent rec pn-space-app 6 1200 (ms 100) 1 1 in
          let _ = recovery-on-ack-received rec pn-space-app 6 0 (ms 50) (ms 130) in
          let rtt2 = recovery-rtt rec in
          -- srtt = (7*20 + 30)/8 = 21.25ms = 21250000ns, rttvar = (3*10 + 10)/4 = 10ms
          let _ = recovery-on-packet-sent rec pn-space-app 7 1200 (ms 200) 1 1 in
          let _ = recovery-on-ack-received rec pn-space-app 7 0 (ms 8) (ms 230) in
          let rtt3 = recovery-rtt rec in
          -- adj_rtt = 22ms, srtt = (7*21250000 + 22000000)/8 = 21343750ns
          let ok-rtt-adj = (eq-u64 rtt2.rtt-smoothed-ns 21250000
            && eq-u64 rtt2.rtt-var-ns 10000000
            && eq-u64 rtt3.rtt-smoothed-ns 21343750
            && eq-u64 rtt3.rtt-min-ns (ms 20)) in

          -- 4. Time-threshold loss detection: 9/8 * max(latest_rtt=30ms, srtt=21.34ms) = 33.75ms
          --    Send PN 10 at t=305ms, PN 11 at t=306ms. ACK only PN 11 at t=326ms (latest_rtt = 20ms):
          --    new srtt = (7*21343750 + 20000000)/8 = 21175781ns, latest_rtt = 20ms
          --    loss_delay = 9/8 * 21175781 = 23822753ns (~23.82ms).
          --    PN 10 is NOT lost at 326ms (10+3 > 11, and age=21ms < 23.82ms!).
          let _ = recovery-on-packet-sent rec pn-space-app 10 1200 (ms 305) 1 1 in
          let _ = recovery-on-packet-sent rec pn-space-app 11 1200 (ms 306) 1 1 in
          let o-tt1 = recovery-on-ack-received rec pn-space-app 11 0 0 (ms 326) in
          let lt1 = recovery-loss-time-ns rec in
          -- Now advance past lt1 and fire loss timeout -> PN 10 MUST be declared lost!
          let o-tt2 = recovery-on-loss-timeout rec lt1 in
          let ok-time-thresh = (eq-u32 o-tt1.ack-out-lost-packets 0
            && gt-u64 lt1 (ms 326)
            && eq-u32 o-tt2.ack-out-lost-packets 1
            && eq-u64 o-tt2.ack-out-lost-bytes 1200
            && eq-u64 (recovery-loss-time-ns rec) 0
            && eq-u64 (recovery-bytes-in-flight rec) 0) in

          -- 5. PTO exponential backoff: send PN 12, fire PTO 3 times -> 1x -> 2x -> 4x -> 8x, then ACK resets pto_count = 0!
          let _ = recovery-on-packet-sent rec pn-space-app 12 1200 (ms 400) 1 1 in
          let pto-b0 = recovery-pto-duration-ns rec in
          let op1 = recovery-on-loss-timeout rec (wrapping-add-u64 (ms 400) pto-b0) in
          let pto-b1 = recovery-pto-duration-ns rec in
          let op2 = recovery-on-loss-timeout rec (wrapping-add-u64 (ms 400) (wrapping-mul-u64 pto-b0 3)) in
          let pto-b2 = recovery-pto-duration-ns rec in
          let op3 = recovery-on-loss-timeout rec (wrapping-add-u64 (ms 400) (wrapping-mul-u64 pto-b0 7)) in
          let pto-b3 = recovery-pto-duration-ns rec in
          let _ = recovery-on-ack-received rec pn-space-app 12 0 0 (ms 1000) in
          let ok-pto = (eq-u8 op1.ack-out-pto-fired 1 && eq-u8 op2.ack-out-pto-fired 1 && eq-u8 op3.ack-out-pto-fired 1
            && eq-u64 pto-b1 (wrapping-shl-u64 pto-b0 1)
            && eq-u64 pto-b2 (wrapping-shl-u64 pto-b0 2)
            && eq-u64 pto-b3 (wrapping-shl-u64 pto-b0 3)
            && eq-u32 (recovery-pto-count rec) 0) in

          -- 6. Persistent congestion (>= 3 * PTO):
          --    Send PN 20 at t=2000ms, PN 21 at t=2000ms + 4*PTO, PN 22..24 at t=2000ms + 4*PTO + 1..3ms.
          --    ACK [22..24] -> PNs 20 & 21 lost with span = 4*PTO >= 3*PTO => persistent_congestion == 1!
          let pto-base = recovery-pto-duration-ns rec in
          let t-pc0 = ms 2000 in
          let t-pc1 = wrapping-add-u64 t-pc0 (wrapping-mul-u64 pto-base 4) in
          let _ = recovery-on-packet-sent rec pn-space-app 20 1200 t-pc0 1 1 in
          let _ = recovery-on-packet-sent rec pn-space-app 21 1200 t-pc1 1 1 in
          let _ = recovery-on-packet-sent rec pn-space-app 22 1200 (wrapping-add-u64 t-pc1 (ms 1)) 1 1 in
          let _ = recovery-on-packet-sent rec pn-space-app 23 1200 (wrapping-add-u64 t-pc1 (ms 2)) 1 1 in
          let _ = recovery-on-packet-sent rec pn-space-app 24 1200 (wrapping-add-u64 t-pc1 (ms 3)) 1 1 in
          -- Also test AckMap -> Recovery multi-range bridge for PNs 22, 24 (skipping 23, 21, 20):
          let am0 = ack-map-on-recv (ack-map-empty ()) 22 (wrapping-add-u64 t-pc1 (ms 20)) 1 in
          let am1 = ack-map-on-recv am0 24 (wrapping-add-u64 t-pc1 (ms 22)) 1 in
          let rcnt = ack-map-extract-ranges-raw am1 scratch 16 in
          let o-pc = recovery-on-ack-ranges-received rec pn-space-app (ack-map-largest am1) (ack-map-first-range am1) scratch rcnt 0 (wrapping-add-u64 t-pc1 (ms 23)) in
          let ok-pc = (eq-u32 rcnt 1
            && eq-u32 o-pc.ack-out-acked-packets 2
            && eq-u32 o-pc.ack-out-lost-packets 2
            && eq-u8 o-pc.ack-out-persistent-congestion 1) in

          let _ = recovery-destroy rec in
          let _ = engine-dma-free dma 4096 in
          if not ok-init then 1
          else if not ok-pkt-thresh then 2
          else if not ok-rtt-adj then 3
          else if not ok-time-thresh then 4
          else if not ok-pto then 5
          else if not ok-pc then 6
          else 0
"#;
    run_leak_checked("rfc9002-recovery", src, 0);
}

// =========================================================================
// 4. Deterministic 10 Gbps / 20 ms RTT simulated bottleneck link test:
//    - BBRv2 + Recovery + Pacer at 0.1% loss (> 92% utilization, bounded queue)
//    - BBRv2 + Recovery + Pacer at 1.0% loss (> 92% utilization, bounded queue)
//    - BBRv2 ProbeRTT cycle (> 10.5s) cleanly probes min_rtt & recovers
//    - CUBIC + Recovery + Pacer at 0.1% loss
// =========================================================================

#[test]
fn simulated_10gbps_20ms_link_bbrv2_and_cubic_with_pacer() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.String
  import Data.Wide
  import Engine.Types
  import Net.Quic.AckMap
  import Net.Quic.Recovery
  import Net.Quic.Bbr
  import Net.Quic.Pacer
  import Prim.Atomic

  -- Link parameters: 10 Gbps (1,250,000,000 B/s), 20 ms propagation RTT,
  -- 12,000 B GSO bursts (10 * 1200B MSS) => 9,600 ns serialization per burst,
  -- 40 ms bottleneck router FIFO tail-drop buffer.
  #[inline(always)]
  #[no-alloc]
  both :: Bool -> Bool -> Bool
  both a b = if a then b else eq-u32 0 1

  u64-inf :: U64
  u64-inf = 18446744073709551615

  link-bps :: U64
  link-bps = 1250000000

  prop-rtt-ns :: U64
  prop-rtt-ns = 20000000

  max-queue-ns :: U64
  max-queue-ns = 40000000

  burst-bytes :: U32
  burst-bytes = 12000

  burst-ser-ns :: U64
  burst-ser-ns = 9600

  -- Deterministic xorshift64 PRNG for repeatable random packet loss.
  #[no-alloc]
  next-rng :: U64 -> U64
  next-rng s0 =
    let s1 = bit-xor-u64 s0 (wrapping-shl-u64 s0 13) in
    let s2 = bit-xor-u64 s1 (wrapping-shr-u64 s1 7) in
    bit-xor-u64 s2 (wrapping-shl-u64 s2 17)

  -- In-flight simulated ACK queue stored in DMA slab at `sbase + 4096`
  -- (cap = 16384, mask = 16383). Each entry is 16 bytes:
  --   +0 ack-arrive-ns :: U64
  --   +8 pn            :: U64
  -- Bytes `sbase + 0..255` hold simulation & PacerState scratchpad so the
  -- discrete-event loops stay within 5 scalar/handle register arguments (`tailcc`).
  q-mask :: U64
  q-mask = 16383

  #[inline(always)] #[no-alloc] s-addr :: U64 -> U64 -> I64
  s-addr base off = wrapping-u64-to-i64 (wrapping-add-u64 base off)

  #[inline(always)] #[no-alloc] s-qh :: U64
  s-qh = 0
  #[inline(always)] #[no-alloc] s-qt :: U64
  s-qt = 8
  #[inline(always)] #[no-alloc] s-lf :: U64
  s-lf = 16
  #[inline(always)] #[no-alloc] s-end :: U64
  s-end = 24
  #[inline(always)] #[no-alloc] s-warm :: U64
  s-warm = 32
  #[inline(always)] #[no-alloc] s-rng :: U64
  s-rng = 40
  #[inline(always)] #[no-alloc] s-loss :: U64
  s-loss = 48
  #[inline(always)] #[no-alloc] s-sbytes :: U64
  s-sbytes = 56
  #[inline(always)] #[no-alloc] s-maxq :: U64
  s-maxq = 64
  #[inline(always)] #[no-alloc] s-qsum :: U64
  s-qsum = 72
  #[inline(always)] #[no-alloc] s-qcnt :: U64
  s-qcnt = 80
  #[inline(always)] #[no-alloc] s-prtt :: U64
  s-prtt = 88
  #[inline(always)] #[no-alloc] s-cruise :: U64
  s-cruise = 89
  #[inline(always)] #[no-alloc] s-bump :: U64
  s-bump = 90
  #[inline(always)] #[no-alloc] s-pacer :: U64
  s-pacer = 96
  #[inline(always)] #[no-alloc] s-losses :: U64
  s-losses = 144

  #[inline(always)]
  #[no-alloc]
  store-pacer :: U64 -> PacerState ->{MVar} Unit
  store-pacer pb p =
    let _ = raw-store-u64 (s-addr pb 0) p.pacer-next-release in
    let _ = raw-store-u64 (s-addr pb 8) p.pacer-tokens in
    let _ = raw-store-u64 (s-addr pb 16) p.pacer-frac-num in
    let _ = raw-store-u64 (s-addr pb 24) p.pacer-last-update-ns in
    let _ = raw-store-u64 (s-addr pb 32) p.pacer-max-ahead in
    let _ = raw-store-u32 (s-addr pb 40) p.pacer-mtu-val in
    let _ = raw-store-u32 (s-addr pb 44) p.pacer-max-burst in
    ()

  #[inline(always)]
  #[no-alloc]
  load-pacer :: U64 ->{MVar} PacerState
  load-pacer pb =
    PacerState
      (raw-load-u64 (s-addr pb 0))
      (raw-load-u64 (s-addr pb 8))
      (raw-load-u64 (s-addr pb 16))
      (raw-load-u64 (s-addr pb 24))
      (raw-load-u64 (s-addr pb 32))
      (raw-load-u32 (s-addr pb 40))
      (raw-load-u32 (s-addr pb 44))

  #[inline(always)]
  #[no-alloc]
  q-entry-addr :: U64 -> U64 -> I64
  q-entry-addr sbase idx =
    wrapping-u64-to-i64 (wrapping-add-u64 (wrapping-add-u64 sbase 4096) (wrapping-shl-u64 (bit-and-u64 idx q-mask) 4))

  #[repr(C, value)]
  record SimResult where
    sim-steady-bytes :: U64
    sim-steady-ns :: U64
    sim-max-q-ns :: U64
    sim-q-sum-ns :: U64
    sim-q-samples :: U64
    sim-saw-probe-rtt :: U8
    sim-saw-cruise :: U8
    sim-pad0 :: U16
    sim-pad1 :: U32

  -- Discrete-event BBRv2 + Recovery + Pacer simulation loop (zero-alloc, 5 register args -> tailcc).
  #[no-alloc]
  sim-bbr-loop :: RecoveryState -> BbrState -> U64 -> U64 -> U64 ->{MVar} Unit
  sim-bbr-loop rec bbr sbase now next-pn =
    let end-ns = raw-load-u64 (s-addr sbase s-end) in
    if ge-u64 now end-ns then ()
    else
      let mode = bbr-mode-u8 bbr in
      let _ = if eq-u8 mode bbr-mode-probe-rtt then raw-store-u8 (s-addr sbase s-prtt) 1 else () in
      let _ = if eq-u8 mode bbr-mode-probe-bw-cruise then raw-store-u8 (s-addr sbase s-cruise) 1 else () in
      let qh = raw-load-u64 (s-addr sbase s-qh) in
      let qt = raw-load-u64 (s-addr sbase s-qt) in
      -- 1. Deliver all ACKs whose arrival timestamp <= now
      if both (lt-u64 qh qt) (le-u64 (raw-load-u64 (q-entry-addr sbase qh)) now) then
        let ea = q-entry-addr sbase qh in
        let ack-t = raw-load-u64 ea in
        let ack-pn = raw-load-u64 (wrapping-add-i64 ea 8) in
        let o = recovery-on-ack-received rec pn-space-app ack-pn 0 0 ack-t in
        let bif = recovery-bytes-in-flight rec in
        let _ = bbr-on-ack bbr o.ack-out-acked-bytes o.ack-out-lost-bytes bif o.ack-out-delivered-bytes o.ack-out-delivered-ns o.ack-out-prior-delivered-bytes o.ack-out-prior-delivered-ns o.ack-out-latest-rtt-ns o.ack-out-is-app-limited ack-t in
        let warmup = raw-load-u64 (s-addr sbase s-warm) in
        let _ = (
          if ge-u64 ack-t warmup then
            let sb = raw-load-u64 (s-addr sbase s-sbytes) in
            raw-store-u64 (s-addr sbase s-sbytes) (saturating-add-u64 sb o.ack-out-acked-bytes)
          else ()) in
        let _ = raw-store-u64 (s-addr sbase s-qh) (wrapping-add-u64 qh 1) in
        sim-bbr-loop rec bbr sbase now next-pn
      else
        -- 2. Try to send a GSO burst if cwnd and pacer allow
        let cwnd = bbr-cwnd bbr in
        let rate = bbr-pacing-rate bbr in
        let bif = recovery-bytes-in-flight rec in
        let room = saturating-sub-u64 cwnd bif in
        let send-sz = if ge-u64 room (u32-to-u64 burst-bytes) then burst-bytes else if ge-u64 room 1200 then (1200 :: U32) else (0 :: U32) in
        let pb = wrapping-add-u64 sbase s-pacer in
        let pacer = load-pacer pb in
        let can-pace = if gt-u32 send-sz 0 then pacer-can-send-now pacer now rate send-sz else eq-u32 0 1 in
        if both (gt-u32 send-sz 0) can-pace then
          let _ = recovery-on-packet-sent rec pn-space-app next-pn send-sz now 1 1 in
          let pacer2 = pacer-on-packet-sent pacer now rate send-sz in
          let _ = store-pacer pb pacer2 in
          let ser-ns = if eq-u32 send-sz burst-bytes then burst-ser-ns else 960 in
          let link-free = raw-load-u64 (s-addr sbase s-lf) in
          let q-start = if gt-u64 link-free now then link-free else now in
          let q-delay = wrapping-sub-u64 q-start now in
          let rng2 = next-rng (raw-load-u64 (s-addr sbase s-rng)) in
          let _ = raw-store-u64 (s-addr sbase s-rng) rng2 in
          let loss-ppm = raw-load-u64 (s-addr sbase s-loss) in
          let rand-drop = lt-u64 (rem-u64 rng2 1000000) loss-ppm in
          let tail-drop = gt-u64 q-delay max-queue-ns in
          let warmup = raw-load-u64 (s-addr sbase s-warm) in
          let _ = (
            if ge-u64 now warmup then
              let max-q = raw-load-u64 (s-addr sbase s-maxq) in
              let _ = if gt-u64 q-delay max-q then raw-store-u64 (s-addr sbase s-maxq) q-delay else () in
              let qsum = raw-load-u64 (s-addr sbase s-qsum) in
              let _ = raw-store-u64 (s-addr sbase s-qsum) (saturating-add-u64 qsum q-delay) in
              let qcnt = raw-load-u64 (s-addr sbase s-qcnt) in
              raw-store-u64 (s-addr sbase s-qcnt) (wrapping-add-u64 qcnt 1)
            else ()) in
          if if rand-drop then eq-u32 0 0 else tail-drop then
            sim-bbr-loop rec bbr sbase now (wrapping-add-u64 next-pn 1)
          else
            let link-free2 = wrapping-add-u64 q-start ser-ns in
            let _ = raw-store-u64 (s-addr sbase s-lf) link-free2 in
            let rtt-bump = raw-load-u8 (s-addr sbase s-bump) in
            let extra-rtt = if both (ne-u8 rtt-bump 0) (both (gt-u64 now 1000000000) (ne-u8 mode bbr-mode-probe-rtt)) then (1000000 :: U64) else (0 :: U64) in
            let ack-t = wrapping-add-u64 (wrapping-add-u64 link-free2 prop-rtt-ns) extra-rtt in
            let ea = q-entry-addr sbase qt in
            let _ = raw-store-u64 ea ack-t in
            let _ = raw-store-u64 (wrapping-add-i64 ea 8) next-pn in
            let _ = raw-store-u64 (s-addr sbase s-qt) (wrapping-add-u64 qt 1) in
            sim-bbr-loop rec bbr sbase now (wrapping-add-u64 next-pn 1)
        else
          -- 3. Advance discrete-event clock to earliest of: next ACK, next pacer burst release, or end-ns
          let next-ack = if lt-u64 qh qt then raw-load-u64 (q-entry-addr sbase qh) else u64-inf in
          let next-pace = if gt-u32 send-sz 0 then pacer-next-burst-time-ns pacer now rate send-sz else u64-inf in
          let next-t0 = if lt-u64 next-ack next-pace then next-ack else next-pace in
          let next-t1 = if lt-u64 next-t0 end-ns then next-t0 else end-ns in
          let next-now = if gt-u64 next-t1 now then next-t1 else wrapping-add-u64 now 1000 in
          let lt = recovery-loss-time-ns rec in
          let _ = (
            if both (ne-u64 lt 0) (le-u64 lt next-now) then
              let ol = recovery-on-loss-timeout rec next-now in
              bbr-on-ack bbr 0 ol.ack-out-lost-bytes (recovery-bytes-in-flight rec) ol.ack-out-delivered-bytes ol.ack-out-delivered-ns 0 0 0 0 next-now
            else ()) in
          sim-bbr-loop rec bbr sbase next-now next-pn

  -- Discrete-event CUBIC + Recovery + Pacer simulation loop (zero-alloc, 5 register args -> tailcc).
  #[no-alloc]
  sim-cubic-loop :: RecoveryState -> CubicState -> U64 -> U64 -> U64 ->{MVar} U64
  sim-cubic-loop rec cub sbase now next-pn =
    let end-ns = raw-load-u64 (s-addr sbase s-end) in
    if ge-u64 now end-ns then raw-load-u64 (s-addr sbase s-sbytes)
    else
      let qh = raw-load-u64 (s-addr sbase s-qh) in
      let qt = raw-load-u64 (s-addr sbase s-qt) in
      if both (lt-u64 qh qt) (le-u64 (raw-load-u64 (q-entry-addr sbase qh)) now) then
        let ea = q-entry-addr sbase qh in
        let ack-t = raw-load-u64 ea in
        let ack-pn = raw-load-u64 (wrapping-add-i64 ea 8) in
        let o = recovery-on-ack-received rec pn-space-app ack-pn 0 0 ack-t in
        let rtt = recovery-rtt rec in
        let _ = (
          if gt-u64 o.ack-out-acked-bytes 0 then
            cubic-on-ack cub o.ack-out-acked-bytes o.ack-out-largest-acked-pn o.ack-out-latest-rtt-ns rtt.rtt-smoothed-ns ack-t o.ack-out-prior-sent-ns
          else ()) in
        let _ = (
          if gt-u64 o.ack-out-lost-bytes 0 then
            let _ = cubic-on-loss cub (saturating-sub-u64 ack-t rtt.rtt-smoothed-ns) rtt.rtt-smoothed-ns ack-t in
            let ls = raw-load-u64 (s-addr sbase s-losses) in
            raw-store-u64 (s-addr sbase s-losses) (wrapping-add-u64 ls 1)
          else ()) in
        let warmup = raw-load-u64 (s-addr sbase s-warm) in
        let _ = (
          if ge-u64 ack-t warmup then
            let sb = raw-load-u64 (s-addr sbase s-sbytes) in
            raw-store-u64 (s-addr sbase s-sbytes) (saturating-add-u64 sb o.ack-out-acked-bytes)
          else ()) in
        let _ = raw-store-u64 (s-addr sbase s-qh) (wrapping-add-u64 qh 1) in
        sim-cubic-loop rec cub sbase now next-pn
      else
        let cwnd = cubic-cwnd cub in
        let rate = cubic-pacing-rate cub in
        let bif = recovery-bytes-in-flight rec in
        let room = saturating-sub-u64 cwnd bif in
        let send-sz = if ge-u64 room (u32-to-u64 burst-bytes) then burst-bytes else if ge-u64 room 1200 then (1200 :: U32) else (0 :: U32) in
        let pb = wrapping-add-u64 sbase s-pacer in
        let pacer = load-pacer pb in
        let can-pace = if gt-u32 send-sz 0 then pacer-can-send-now pacer now rate send-sz else eq-u32 0 1 in
        if both (gt-u32 send-sz 0) can-pace then
          let _ = recovery-on-packet-sent rec pn-space-app next-pn send-sz now 1 1 in
          let pacer2 = pacer-on-packet-sent pacer now rate send-sz in
          let _ = store-pacer pb pacer2 in
          let ser-ns = if eq-u32 send-sz burst-bytes then burst-ser-ns else 960 in
          let link-free = raw-load-u64 (s-addr sbase s-lf) in
          let q-start = if gt-u64 link-free now then link-free else now in
          let q-delay = wrapping-sub-u64 q-start now in
          let rng2 = next-rng (raw-load-u64 (s-addr sbase s-rng)) in
          let _ = raw-store-u64 (s-addr sbase s-rng) rng2 in
          let loss-ppm = raw-load-u64 (s-addr sbase s-loss) in
          let rand-drop = lt-u64 (rem-u64 rng2 1000000) loss-ppm in
          let tail-drop = gt-u64 q-delay max-queue-ns in
          if if rand-drop then eq-u32 0 0 else tail-drop then
            sim-cubic-loop rec cub sbase now (wrapping-add-u64 next-pn 1)
          else
            let link-free2 = wrapping-add-u64 q-start ser-ns in
            let _ = raw-store-u64 (s-addr sbase s-lf) link-free2 in
            let ack-t = wrapping-add-u64 link-free2 prop-rtt-ns in
            let ea = q-entry-addr sbase qt in
            let _ = raw-store-u64 ea ack-t in
            let _ = raw-store-u64 (wrapping-add-i64 ea 8) next-pn in
            let _ = raw-store-u64 (s-addr sbase s-qt) (wrapping-add-u64 qt 1) in
            sim-cubic-loop rec cub sbase now (wrapping-add-u64 next-pn 1)
        else
          let next-ack = if lt-u64 qh qt then raw-load-u64 (q-entry-addr sbase qh) else u64-inf in
          let next-pace = if gt-u32 send-sz 0 then pacer-next-burst-time-ns pacer now rate send-sz else u64-inf in
          let next-t0 = if lt-u64 next-ack next-pace then next-ack else next-pace in
          let next-t1 = if lt-u64 next-t0 end-ns then next-t0 else end-ns in
          let next-now = if gt-u64 next-t1 now then next-t1 else wrapping-add-u64 now 1000 in
          sim-cubic-loop rec cub sbase next-now next-pn

  -- Compute utilization in permille (e.g. 950 = 95.0%).
  util-permille :: U64 -> U64 -> U64
  util-permille delivered-bytes duration-ns =
    let cap-bytes = saturating-u128-to-u64 (div-u128 (wrapping-mul-u128 (u64-to-u128 link-bps) (u64-to-u128 duration-ns)) 1000000000) in
    let d128 = u64-to-u128 cap-bytes in
    if d128 /= 0 then
      let num = wrapping-mul-u128 (u64-to-u128 delivered-bytes) 1000 in
      saturating-u128-to-u64 (div-u128 num d128)
    else 0

  init-sim-scratch :: U64 -> U64 -> U64 -> U64 -> U64 -> U8 ->{MVar} Unit
  init-sim-scratch sbase rng-seed loss-ppm warmup-ns end-ns rtt-bump =
    let _ = raw-store-u64 (s-addr sbase s-qh) 0 in
    let _ = raw-store-u64 (s-addr sbase s-qt) 0 in
    let _ = raw-store-u64 (s-addr sbase s-lf) 0 in
    let _ = raw-store-u64 (s-addr sbase s-end) end-ns in
    let _ = raw-store-u64 (s-addr sbase s-warm) warmup-ns in
    let _ = raw-store-u64 (s-addr sbase s-rng) rng-seed in
    let _ = raw-store-u64 (s-addr sbase s-loss) loss-ppm in
    let _ = raw-store-u64 (s-addr sbase s-sbytes) 0 in
    let _ = raw-store-u64 (s-addr sbase s-maxq) 0 in
    let _ = raw-store-u64 (s-addr sbase s-qsum) 0 in
    let _ = raw-store-u64 (s-addr sbase s-qcnt) 0 in
    let _ = raw-store-u8 (s-addr sbase s-prtt) 0 in
    let _ = raw-store-u8 (s-addr sbase s-cruise) 0 in
    let _ = raw-store-u8 (s-addr sbase s-bump) rtt-bump in
    let _ = raw-store-u64 (s-addr sbase s-losses) 0 in
    store-pacer (wrapping-add-u64 sbase s-pacer) (pacer-init 1200 0)

  run-bbr-scenario :: U64 -> U64 -> U64 -> U64 -> U8 ->{IO, MVar} SimResult
  run-bbr-scenario sbase loss-ppm warmup-ns end-ns rtt-bump =
    case (recovery-new 16384, bbr-new 1200 0) of
      (Some rec, Some bbr) ->
        let _ = init-sim-scratch sbase 0x9E3779B97F4A7C15 loss-ppm warmup-ns end-ns rtt-bump in
        let _ = sim-bbr-loop rec bbr sbase 0 1 in
        let _ = recovery-destroy rec in
        let _ = bbr-destroy bbr in
        SimResult
          (raw-load-u64 (s-addr sbase s-sbytes))
          (saturating-sub-u64 end-ns warmup-ns)
          (raw-load-u64 (s-addr sbase s-maxq))
          (raw-load-u64 (s-addr sbase s-qsum))
          (raw-load-u64 (s-addr sbase s-qcnt))
          (raw-load-u8 (s-addr sbase s-prtt))
          (raw-load-u8 (s-addr sbase s-cruise))
          0 0
      (Some rec, None) ->
        let _ = recovery-destroy rec in
        SimResult 0 1 0 0 0 0 0 0 0
      (None, Some bbr) ->
        let _ = bbr-destroy bbr in
        SimResult 0 1 0 0 0 0 0 0 0
      (None, None) ->
        SimResult 0 1 0 0 0 0 0 0 0

  main :: Unit ->{IO, IO r, MVar} Int
  main _ =
    let qsize = (266240 :: U64) in -- 4096 scratch + 16384 * 16B FIFO
    let qbase = engine-dma-alloc qsize 4096 in
    if eq-u64 qbase 0 then 99
    else
      -- Scenario A: BBRv2 @ 10 Gbps / 20 ms RTT / 0.1% loss (3.0s run, steady window [0.8s..3.0s])
      let r-a = run-bbr-scenario qbase 1000 800000000 3000000000 0 in
      let util-a = util-permille r-a.sim-steady-bytes r-a.sim-steady-ns in
      let qsa = r-a.sim-q-samples in
      let mean-q-a = if qsa /= 0 then div-u64 r-a.sim-q-sum-ns qsa else 999999999 in

      -- Scenario B: BBRv2 @ 10 Gbps / 20 ms RTT / 1.0% loss (3.0s run, steady window [0.8s..3.0s])
      let r-b = run-bbr-scenario qbase 10000 800000000 3000000000 0 in
      let util-b = util-permille r-b.sim-steady-bytes r-b.sim-steady-ns in
      let qsb = r-b.sim-q-samples in
      let mean-q-b = if qsb /= 0 then div-u64 r-b.sim-q-sum-ns qsb else 999999999 in

      -- Scenario C: BBRv2 ProbeRTT trigger across 12.0s (triggers ProbeRTT after 10s, drains, recovers!)
      let r-c = run-bbr-scenario qbase 1000 1000000000 12000000000 1 in
      let util-c = util-permille r-c.sim-steady-bytes r-c.sim-steady-ns in

      -- Scenario D: CUBIC + Pacer @ 10 Gbps / 20 ms RTT / 0.1% loss (2.0s run, steady [0.5s..2.0s])
      let cub-bytes = (case (recovery-new 16384, cubic-new 1200) of
        (Some rec, Some cub) ->
          let _ = init-sim-scratch qbase 0x517CC1B727220A95 1000 500000000 2000000000 0 in
          let b = sim-cubic-loop rec cub qbase 0 1 in
          let _ = recovery-destroy rec in
          let _ = cubic-destroy cub in
          b
        (Some rec, None) ->
          let _ = recovery-destroy rec in
          0
        (None, Some cub) ->
          let _ = cubic-destroy cub in
          0
        (None, None) -> 0) in

      let cub-mb = div-u64 cub-bytes 1000000 in
      let mean-q-a-us = div-u64 mean-q-a 1000 in
      let max-q-a-us = div-u64 r-a.sim-max-q-ns 1000 in
      let mean-q-b-us = div-u64 mean-q-b 1000 in
      let max-q-b-us = div-u64 r-b.sim-max-q-ns 1000 in
      let u32s = \v -> nat-to-string (u32-to-nat (wrapping-u64-to-u32 v)) in
      let line = (string-concat "bbr_0.1pct_util_pm="
        (string-concat (u32s util-a)
        (string-concat " mean_q_us="
        (string-concat (u32s mean-q-a-us)
        (string-concat " max_q_us="
        (string-concat (u32s max-q-a-us)
        (string-concat " | bbr_1.0pct_util_pm="
        (string-concat (u32s util-b)
        (string-concat " mean_q_us="
        (string-concat (u32s mean-q-b-us)
        (string-concat " max_q_us="
        (string-concat (u32s max-q-b-us)
        (string-concat " | probe_rtt_12s_util_pm="
        (string-concat (u32s util-c)
        (string-concat " | cubic_0.1pct_MB="
        (string-concat (u32s cub-mb) "\n")))))))))))))))) in
      let _ = (with (io = newIO ()) in ioWrite (string-to-bytes line)) in
      let _ = engine-dma-free qbase qsize in
      -- Assert BBRv2 > 92% utilization (920 permille) at BOTH 0.1% and 1% loss,
      -- bounded queue delay (mean < 3ms, max < 15ms — no 40ms bufferbloat!),
      -- ProbeRTT visited & recovered cleanly (> 90% overall across 12s incl. 200ms ProbeRTT),
      -- and CUBIC transfers substantial volume (> 10 MB in 1.5s under 0.1% loss).
      if lt-u64 util-a 920 then 11
      else if eq-u8 r-a.sim-saw-cruise 0 then 12
      else if gt-u64 mean-q-a 3000000 then 13
      else if gt-u64 r-a.sim-max-q-ns 15000000 then 14
      else if lt-u64 util-b 920 then 21
      else if gt-u64 mean-q-b 3000000 then 22
      else if gt-u64 r-b.sim-max-q-ns 15000000 then 23
      else if eq-u8 r-c.sim-saw-probe-rtt 0 then 31
      else if lt-u64 util-c 900 then 32
      else if lt-u64 cub-bytes 10000000 then 41
      else 0
"#;
    let (stdout, _stderr) = run_leak_checked("sim-10gbps-bbrv2-cubic", src, 0);
    eprintln!("[sim-10gbps-20ms] {}", stdout.trim());
}

// =========================================================================
// 5. Steady-state 50,000-op send + ack + BBRv2 + CUBIC + AckMap + Pacer loop:
//    allocs == 0, atomic_rc == 0, CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1 (leaked=0)
// =========================================================================

#[test]
fn steady_state_50000_send_ack_pace_zero_alloc_zero_atomic_rc() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Data.Option
  import Engine.Types
  import Net.Quic.AckMap
  import Net.Quic.Recovery
  import Net.Quic.Bbr
  import Net.Quic.Pacer
  import Prim.Atomic

  #[inline(always)] #[no-alloc] d-addr :: U64 -> U64 -> I64
  d-addr base off = wrapping-u64-to-i64 (wrapping-add-u64 base off)

  #[inline(always)] #[no-alloc]
  store-ackmap :: U64 -> AckMap ->{MVar} Unit
  store-ackmap ab m =
    let _ = raw-store-u64 (d-addr ab 0) m.ack-largest in
    let _ = raw-store-u64 (d-addr ab 8) m.ack-largest-recv-ns in
    let _ = raw-store-u64 (d-addr ab 16) m.ack-w0 in
    let _ = raw-store-u64 (d-addr ab 24) m.ack-w1 in
    let _ = raw-store-u64 (d-addr ab 32) m.ack-w2 in
    let _ = raw-store-u64 (d-addr ab 40) m.ack-w3 in
    let _ = raw-store-u32 (d-addr ab 48) m.ack-eliciting-pending in
    let _ = raw-store-u8 (d-addr ab 52) m.ack-has-any in
    ()

  #[inline(always)] #[no-alloc]
  load-ackmap :: U64 ->{MVar} AckMap
  load-ackmap ab =
    AckMap
      (raw-load-u64 (d-addr ab 0))
      (raw-load-u64 (d-addr ab 8))
      (raw-load-u64 (d-addr ab 16))
      (raw-load-u64 (d-addr ab 24))
      (raw-load-u64 (d-addr ab 32))
      (raw-load-u64 (d-addr ab 40))
      (raw-load-u32 (d-addr ab 48))
      (raw-load-u8 (d-addr ab 52))
      0 0

  #[inline(always)] #[no-alloc]
  store-pacer :: U64 -> PacerState ->{MVar} Unit
  store-pacer pb p =
    let _ = raw-store-u64 (d-addr pb 0) p.pacer-next-release in
    let _ = raw-store-u64 (d-addr pb 8) p.pacer-tokens in
    let _ = raw-store-u64 (d-addr pb 16) p.pacer-frac-num in
    let _ = raw-store-u64 (d-addr pb 24) p.pacer-last-update-ns in
    let _ = raw-store-u64 (d-addr pb 32) p.pacer-max-ahead in
    let _ = raw-store-u32 (d-addr pb 40) p.pacer-mtu-val in
    let _ = raw-store-u32 (d-addr pb 44) p.pacer-max-burst in
    ()

  #[inline(always)] #[no-alloc]
  load-pacer :: U64 ->{MVar} PacerState
  load-pacer pb =
    PacerState
      (raw-load-u64 (d-addr pb 0))
      (raw-load-u64 (d-addr pb 8))
      (raw-load-u64 (d-addr pb 16))
      (raw-load-u64 (d-addr pb 24))
      (raw-load-u64 (d-addr pb 32))
      (raw-load-u32 (d-addr pb 40))
      (raw-load-u32 (d-addr pb 44))

  -- Kept to 7 scalar/handle register arguments so `tailcc` runs 50,000 iterations in O(1) stack.
  #[no-alloc]
  hot-loop :: RecoveryState -> BbrState -> CubicState -> U64 -> U64 -> U64 -> U64 ->{MVar} U64
  hot-loop rec bbr cub dma i total now =
    if ge-u64 i total then raw-load-u64 (d-addr dma 384)
    else
      let scratch = wrapping-u64-to-i64 dma in
      let ab = wrapping-add-u64 dma 256 in
      let pb = wrapping-add-u64 dma 320 in
      let am = load-ackmap ab in
      let pacer = load-pacer pb in
      let pn = wrapping-add-u64 i 1 in
      let rate = bbr-pacing-rate bbr in
      let _ = pacer-can-send-now pacer now rate 1200 in
      let _ = pacer-burst-budget pacer now rate in
      let _ = pacer-next-send-time-ns pacer now rate in
      let pacer2 = pacer-on-packet-sent pacer now rate 1200 in
      let _ = store-pacer pb pacer2 in
      let _ = recovery-on-packet-sent rec pn-space-app pn 1200 now 1 1 in
      let recv-t = wrapping-add-u64 now 10000 in
      let am-res = ack-map-on-recv-checked am pn recv-t 1 in
      let am2 = ack-recv-map am-res in
      let _ = store-ackmap ab am2 in
      let fr = ack-map-first-range am2 in
      let _ = ack-map-extract-ranges-raw am2 scratch 8 in
      let ack-t = wrapping-add-u64 now 20000 in
      let o = recovery-on-ack-received rec pn-space-app pn 0 0 ack-t in
      let bif = recovery-bytes-in-flight rec in
      let _ = bbr-on-ack bbr o.ack-out-acked-bytes o.ack-out-lost-bytes bif o.ack-out-delivered-bytes o.ack-out-delivered-ns o.ack-out-prior-delivered-bytes o.ack-out-prior-delivered-ns o.ack-out-latest-rtt-ns o.ack-out-is-app-limited ack-t in
      let rtt = recovery-rtt rec in
      let _ = cubic-on-ack cub o.ack-out-acked-bytes o.ack-out-largest-acked-pn o.ack-out-latest-rtt-ns rtt.rtt-smoothed-ns ack-t o.ack-out-prior-sent-ns in
      let _ = recovery-pto-duration-ns rec in
      let acc = raw-load-u64 (d-addr dma 384) in
      let _ = raw-store-u64 (d-addr dma 384) (wrapping-add-u64 acc (wrapping-add-u64 o.ack-out-acked-bytes fr)) in
      hot-loop rec bbr cub dma (wrapping-add-u64 i 1) total (wrapping-add-u64 now 25000)

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 4096 4096 in
    if eq-u64 dma 0 then 99
    else
      case (recovery-new 1024, bbr-new 1200 0, cubic-new 1200) of
        (Some rec, Some bbr, Some cub) ->
          let _ = store-ackmap (wrapping-add-u64 dma 256) (ack-map-empty ()) in
          let _ = store-pacer (wrapping-add-u64 dma 320) (pacer-init 1200 0) in
          let _ = raw-store-u64 (d-addr dma 384) 0 in
          -- Warm up 1 iteration so initial states are active, then reset thread counters
          let _ = hot-loop rec bbr cub dma 0 1 0 in
          let _ = engine-thread-counters-reset () in
          let acc = hot-loop rec bbr cub dma 1 50001 50000 in
          let allocs = engine-thread-alloc-count () in
          let atomic-rcs = engine-thread-atomic-rc-count () in
          let bif = recovery-bytes-in-flight rec in
          let _ = recovery-destroy rec in
          let _ = bbr-destroy bbr in
          let _ = cubic-destroy cub in
          let _ = engine-dma-free dma 4096 in
          if eq-u64 acc 0 then 10
          else if ne-u64 bif 0 then 11
          else if ne-u64 allocs 0 then 12
          else if ne-u64 atomic-rcs 0 then 13
          else 0
        (Some rec, Some bbr, None) ->
          let _ = recovery-destroy rec in
          let _ = bbr-destroy bbr in
          let _ = engine-dma-free dma 4096 in
          90
        (Some rec, None, Some cub) ->
          let _ = recovery-destroy rec in
          let _ = cubic-destroy cub in
          let _ = engine-dma-free dma 4096 in
          91
        (None, Some bbr, Some cub) ->
          let _ = bbr-destroy bbr in
          let _ = cubic-destroy cub in
          let _ = engine-dma-free dma 4096 in
          92
        _ ->
          let _ = engine-dma-free dma 4096 in
          93
"#;
    run_leak_checked("hot-50k-zero-alloc", src, 0);
}
