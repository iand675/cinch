//! Integration tests for Wave 5 `Net.Aeron` (`Net.Aeron.Protocol`,
//! `Net.Aeron.Transport`):
//!
//!   1. Stdlib manifest registration (`seed/prelude/.cinchpkg`), canonical
//!      documentation, exact POD ABI layouts (`AeronDataHeader = 32B`,
//!      `AeronStatusHeader = 36B`, `AeronNakHeader = 28B`, `AeronSetupHeader = 32B`),
//!      and transitive `#[no-alloc]` verification across protocol & transport hot paths.
//!   2. Wire protocol codec & transport mechanics: `DATA`, `SM`, `NAK`, `SETUP`,
//!      multi-frame `BEGIN`/`END` fragmentation, 3-partition term rotation
//!      (`active`/`dirty`/`clean`), LRAT-verified Castagnoli CRC32C tamper
//!      rejection, `SM` flow-control backpressure, `Net.Quic.Pacer` pacing /
//!      `SO_TXTIME` + burst coalescing, zero-copy `try-claim`/`commit`, and
//!      lock-free `Engine.Ring.Disruptor` local-host IPC bridge.
//!   3. **5% Lossy + 10% Reordering + 5% Duplicate Deterministic UDP Simulation**
//!      over `Engine.Sim`: streams 2,000 variable-size messages (crossing dozens
//!      of 3-partition term rotations and multi-fragment messages) -> verifies
//!      **100% lossless, strictly ordered, byte-exact delivery** via NAK
//!      retransmission and SM flow control, 0 hangs, 0 leaks.
//!   4. **Live macOS Loopback UDP Transport (`Engine.Reactor.Darwin`) +
//!      50,000-Message Zero-Alloc Hot Loop**: exchanges variable-size messages
//!      over real non-blocking loopback UDP sockets + runs 50,000 steady-state
//!      `aeron-pub-offer` + `aeron-pub-poll-tx` + `aeron-sub-on-datagram` +
//!      `aeron-sub-poll` cycles asserting `allocs == 0`, `atomic_rc == 0`, and
//!      `CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1` (`leaked=0`).
//!
//! Routed through `common::drive_with_profile` / `common::check_interfaces` /
//! `common::build_executable` so the exact same suite runs against both the
//! Rust seed and the Stage 3 self-hosted compiler.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{Mode, Package};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

const NET_AERON_PROTOCOL: &str = include_str!("../prelude/Net/Aeron/Protocol.cinch");
const NET_AERON_TRANSPORT: &str = include_str!("../prelude/Net/Aeron/Transport.cinch");
const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn fresh_dir(tag: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "cinch-net-aeron-{tag}-{}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn build_and_run_leak_checked(tag: &str, src: &str, expected_code: i32) -> (String, String) {
    let dir = fresh_dir(tag);
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let exe = common::build_executable(&dir, "Main.cinch", &format!("net-aeron-{tag}"))
        .unwrap_or_else(|e| panic!("{tag}: build failed:\n{e}"));

    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "512M")
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
        "{tag}: expected clean leak report (`leaked=0`), got stderr:\n{stderr}"
    );
    (stdout, stderr)
}

// =========================================================================
// 1. Manifest Registration, Exact POD ABI Layouts, Docs, & #[no-alloc]
// =========================================================================

#[test]
fn net_aeron_manifest_abi_layout_and_no_alloc() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, rel_path) in [
        ("Net.Aeron.Protocol", "Net/Aeron/Protocol.cinch"),
        ("Net.Aeron.Transport", "Net/Aeron/Transport.cinch"),
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

    let pos = |name: &str| {
        pkg.order
            .iter()
            .position(|m| m == name)
            .unwrap_or_else(|| panic!("{name} missing from order"))
    };
    assert!(pos("Engine.Simd.Crc32c") < pos("Net.Aeron.Protocol"));
    assert!(pos("Net.Aeron.Protocol") < pos("Net.Aeron.Transport"));
    assert!(pos("Net.Quic.Pacer") < pos("Net.Aeron.Transport"));
    assert!(pos("Engine.Ring.Disruptor") < pos("Net.Aeron.Transport"));

    common::assert_documented("Net.Aeron.Protocol", NET_AERON_PROTOCOL);
    common::assert_documented("Net.Aeron.Transport", NET_AERON_TRANSPORT);

    // Check exact POD ABI sizes & #[no-alloc] facts directly via stdlib interfaces.
    let dir = common::project(
        "net-aeron-abi-no-alloc",
        &[
            ("Net.Aeron.Protocol.cinch", NET_AERON_PROTOCOL),
            ("Net.Aeron.Transport.cinch", NET_AERON_TRANSPORT),
            (
                "Main.cinch",
                r#"
module Main where
  import Data.Machine
  import Net.Aeron.Protocol
  import Net.Aeron.Transport

  #[no-alloc]
  probe-protocol-no-alloc :: I64 -> I64 ->{MVar} U32
  probe-protocol-no-alloc buf pl =
    let dh = aeron-data-header-make 16 aeron-flag-unfragmented 0 100 1 7 0 in
    let n1 = aeron-encode-data-frame buf 256 dh pl 16 (eq-u32 0 0) in
    let v1 = aeron-validate-data-frame buf n1 (eq-u32 0 0) in
    let sm = aeron-sm-header-make 100 1 7 64 16384 999 in
    let n2 = aeron-encode-sm buf 256 sm in
    let nak = aeron-nak-header-make 100 1 7 64 128 in
    let n3 = aeron-encode-nak buf 256 nak in
    let stp = aeron-setup-header-make 100 1 7 7 65536 1408 in
    let n4 = aeron-encode-setup buf 256 stp in
    wrapping-add-u32 (wrapping-add-u32 n1 v1) (wrapping-add-u32 (wrapping-add-u32 n2 n3) n4)

  #[no-alloc]
  probe-transport-no-alloc :: AeronPublication -> AeronSubscription -> I64 -> U32 -> U64 ->{MVar} U32
  probe-transport-no-alloc pub sub buf len now =
    let _p = aeron-pub-offer pub buf len in
    let tx = aeron-pub-poll-tx pub buf 2048 now in
    let _rx = aeron-sub-on-datagram sub buf tx.tx-len now in
    let pol = aeron-sub-poll sub buf 2048 in
    let ctrl = aeron-sub-poll-control sub buf 256 now in
    let _pc = aeron-pub-on-control pub buf ctrl.ctrl-len in
    wrapping-add-u32 tx.tx-len pol.poll-len

  main :: Int
  main =
    let dh = aeron-data-header-make 0 aeron-flag-unfragmented 0 1 1 1 0 in
    let sm = aeron-sm-header-make 1 1 1 0 4096 1 in
    let nak = aeron-nak-header-make 1 1 1 0 64 in
    let stp = aeron-setup-header-make 1 1 1 1 65536 1408 in
    if eq-i32 dh.data-frame-length 32
      && eq-i32 sm.sm-frame-length 36
      && eq-i32 nak.nak-frame-length 28
      && eq-i32 stp.setup-frame-length 32
    then 0 else 1
"#,
            ),
        ],
    );

    let ifaces = common::check_interfaces(&dir, "Main.cinch")
        .expect("Net.Aeron.{Protocol,Transport} must pass interface & #[no-alloc] checks");

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

    let proto = iface_of("Net.Aeron.Protocol");
    let trans = iface_of("Net.Aeron.Transport");
    let main_i = iface_of("Main");

    // Exact wire header ABI sizes required by spec:
    assert_eq!(layout_of(proto, "AeronDataHeader").size, 32);
    assert_eq!(layout_of(proto, "AeronStatusHeader").size, 36);
    assert_eq!(layout_of(proto, "AeronNakHeader").size, 28);
    assert_eq!(layout_of(proto, "AeronSetupHeader").size, 32);

    for f in [
        "aeron-align32",
        "aeron-partition-index",
        "aeron-compute-position",
        "aeron-data-compute-crc",
        "aeron-data-verify-crc",
        "aeron-encode-data-frame",
        "aeron-validate-data-frame",
        "aeron-encode-sm",
        "aeron-decode-sm",
        "aeron-encode-nak",
        "aeron-decode-nak",
        "aeron-encode-setup",
        "aeron-decode-setup",
    ] {
        assert_no_alloc(proto, f);
    }

    for f in [
        "aeron-pub-try-claim",
        "aeron-pub-commit",
        "aeron-pub-offer",
        "aeron-pub-poll-tx",
        "aeron-pub-poll-tx-burst",
        "aeron-pub-on-control",
        "aeron-sub-on-datagram",
        "aeron-sub-poll-control",
        "aeron-sub-poll",
    ] {
        assert_no_alloc(trans, f);
    }

    assert_no_alloc(main_i, "probe-protocol-no-alloc");
    assert_no_alloc(main_i, "probe-transport-no-alloc");

    assert_eq!(common::run(&dir, "Main.cinch"), 0);
    common::cleanup(&dir);

    // Also verify direct stdlib resolution without copying sources into temp project.
    let dir_std = fresh_dir("stdlib-direct");
    std::fs::write(
        dir_std.join("Main.cinch"),
        "module Main where\n  import Net.Aeron.Protocol\n  import Net.Aeron.Transport\n  main :: Int\n  main = if eq-u32 (aeron-align32 33) 64 then 0 else 1\n",
    )
    .expect("write Main.cinch");
    let code = common::drive_with_profile(
        Mode::Run,
        StageMode::Refresh,
        &dir_std,
        "Main.cinch",
        None,
        &TargetProfile::native(),
    )
    .expect("direct stdlib import of Net.Aeron.{Protocol,Transport}");
    let _ = std::fs::remove_dir_all(&dir_std);
    assert_eq!(code, 0);
}

// =========================================================================
// 2. Wire Protocol Codec, Fragmentation, 3-Partition Term Rotation, CRC32C
//    Tamper Rejection, Flow-Control Backpressure, Pacing, & Disruptor IPC
// =========================================================================

#[test]
fn net_aeron_codec_fragmentation_rotation_crc_backpressure_pacing_and_ipc() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Data.Option
  import Engine.Ring.Disruptor
  import Engine.Types
  import Net.Aeron.Protocol
  import Net.Aeron.Transport
  import Prim.Atomic

  fill-pat :: I64 -> U32 -> U32 -> U8 ->{MVar} Unit
  fill-pat addr i n seed =
    if ge-u32 i n then ()
    else
      let v = wrapping-add-u8 (wrapping-mul-u8 (wrapping-u32-to-u8 i) 37) seed in
      let _ = raw-store-u8 (wrapping-add-i64 addr (u32-to-i64 i)) v in
      fill-pat addr (wrapping-add-u32 i 1) n seed

  verify-pat :: I64 -> U32 -> U32 -> U8 ->{MVar} Bool
  verify-pat addr i n seed =
    if ge-u32 i n then True
    else
      let got = raw-load-u8 (wrapping-add-i64 addr (u32-to-i64 i)) in
      let exp = wrapping-add-u8 (wrapping-mul-u8 (wrapping-u32-to-u8 i) 37) seed in
      if eq-u8 got exp then verify-pat addr (wrapping-add-u32 i 1) n seed
      else False

  -- 1. Wire protocol codec tests (DATA, SM, NAK, SETUP, align32, CRC32C tamper)
  test-codecs :: I64 ->{MVar} Int
  test-codecs slab =
    let wire = slab in
    let pl = wrapping-add-i64 slab 2048 in
    let _fp = fill-pat pl 0 93 11 in
    -- align32 checks
    let ok-al = (eq-u32 (aeron-align32 0) 0
      && eq-u32 (aeron-align32 1) 32
      && eq-u32 (aeron-align32 32) 32
      && eq-u32 (aeron-align32 33) 64
      && eq-u32 (aeron-align32 125) 128) in
    -- DATA encode/decode + CRC32C pass & single-bit tamper rejection
    let dh0 = aeron-data-header-make 93 aeron-flag-unfragmented 64 4242 7 100 0 in
    let n-data = aeron-encode-data-frame wire 2048 dh0 pl 93 True in
    let d-dec = aeron-decode-data-header wire in
    let v-ok = eq-u32 (aeron-validate-data-frame wire n-data True) aeron-status-ok in
    let crc-ok = aeron-data-verify-crc-strict wire n-data in
    -- Flip 1 bit in payload byte +50 -> MUST reject with aeron-status-bad-crc!
    let b50 = raw-load-u8 (wrapping-add-i64 wire 50) in
    let _flip = raw-store-u8 (wrapping-add-i64 wire 50) (bit-xor-u8 b50 0x04) in
    let v-bad-pl = eq-u32 (aeron-validate-data-frame wire n-data True) aeron-status-bad-crc in
    let crc-bad = not (aeron-data-verify-crc-strict wire n-data) in
    let _rest = raw-store-u8 (wrapping-add-i64 wire 50) b50 in
    -- Flip 1 bit in header term-id (+20) -> MUST ALSO reject with aeron-status-bad-crc!
    let b20 = raw-load-u8 (wrapping-add-i64 wire 20) in
    let _flip2 = raw-store-u8 (wrapping-add-i64 wire 20) (bit-xor-u8 b20 0x01) in
    let v-bad-hdr = eq-u32 (aeron-validate-data-frame wire n-data True) aeron-status-bad-crc in
    let _rest2 = raw-store-u8 (wrapping-add-i64 wire 20) b20 in
    let ok-data = (eq-u32 n-data 125 && eq-i32 d-dec.data-frame-length 125
      && eq-u8 d-dec.data-flags (bit-or-u8 aeron-flag-unfragmented aeron-flag-crc)
      && eq-u16 d-dec.data-frame-type aeron-frame-type-data
      && eq-u32 d-dec.data-term-offset 64 && eq-i32 d-dec.data-session-id 4242
      && eq-i32 d-dec.data-stream-id 7 && eq-i32 d-dec.data-term-id 100
      && v-ok && crc-ok && v-bad-pl && crc-bad && v-bad-hdr) in

    -- SM encode/decode/validate (36B, preserving full 64-bit receiver_id)
    let rid = (0x1122334455667788 :: I64) in
    let sm0 = aeron-sm-header-make 4242 7 105 1024 32768 rid in
    let n-sm = aeron-encode-sm wire 256 sm0 in
    let sm-ok = eq-u32 n-sm 36 && eq-u32 (aeron-validate-sm wire 36) aeron-status-ok in
    let sm1 = aeron-decode-sm wire in
    let ok-sm = (sm-ok && eq-i32 sm1.sm-session-id 4242 && eq-i32 sm1.sm-stream-id 7
      && eq-i32 sm1.sm-consumption-term-id 105 && eq-u32 sm1.sm-consumption-term-offset 1024
      && eq-u32 sm1.sm-receiver-window-length 32768 && eq-i64 (aeron-sm-receiver-id sm1) rid) in

    -- NAK encode/decode/validate (28B)
    let nak0 = aeron-nak-header-make 4242 7 104 2048 512 in
    let n-nak = aeron-encode-nak wire 256 nak0 in
    let nak-ok = eq-u32 n-nak 28 && eq-u32 (aeron-validate-nak wire 28) aeron-status-ok in
    let nak1 = aeron-decode-nak wire in
    let ok-nak = (nak-ok && eq-i32 nak1.nak-session-id 4242 && eq-i32 nak1.nak-stream-id 7
      && eq-i32 nak1.nak-term-id 104 && eq-u32 nak1.nak-term-offset 2048 && eq-u32 nak1.nak-length 512) in

    -- SETUP encode/decode/validate (32B)
    let stp0 = aeron-setup-header-make 4242 7 100 105 65536 1408 in
    let n-stp = aeron-encode-setup wire 256 stp0 in
    let stp-ok = eq-u32 n-stp 32 && eq-u32 (aeron-validate-setup wire 32) aeron-status-ok in
    let stp1 = aeron-decode-setup wire in
    let ok-stp = (stp-ok && eq-i32 stp1.setup-session-id 4242 && eq-i32 stp1.setup-stream-id 7
      && eq-i32 stp1.setup-initial-term-id 100 && eq-i32 stp1.setup-active-term-id 105
      && eq-u32 stp1.setup-term-length 65536 && eq-u32 stp1.setup-mtu 1408) in

    if not ok-al then 10
    else if not ok-data then 11
    else if not ok-sm then 12
    else if not ok-nak then 13
    else if not ok-stp then 14
    else 0

  -- Pump all queued TX frames from pub directly to sub
  pump-all-tx :: AeronPublication -> AeronSubscription -> I64 -> U64 -> U32 ->{MVar} U32
  pump-all-tx pub sub wire now lim =
    if eq-u32 lim 0 then 0
    else
      let r = aeron-pub-poll-tx pub wire 2048 now in
      if eq-u32 r.tx-len 0 then 0
      else
        let _rx = aeron-sub-on-datagram sub wire r.tx-len now in
        wrapping-add-u32 1 (pump-all-tx pub sub wire now (wrapping-sub-u32 lim 1))

  pump-ctrl :: AeronPublication -> AeronSubscription -> I64 -> U64 ->{MVar} Unit
  pump-ctrl pub sub wire now =
    let c = aeron-sub-poll-control sub wire 256 now in
    if gt-u32 c.ctrl-len 0 then
      let _ = aeron-pub-on-control pub wire c.ctrl-len in
      pump-ctrl pub sub wire now
    else ()

  -- 2. Fragmentation (BEGIN/END), 3-partition term rotation, CRC tamper rejection,
  --    SM flow-control backpressure, zero-copy try-claim, and pacer SO_TXTIME burst.
  test-transport-features :: I64 ->{IO, MVar} Int
  test-transport-features slab =
    let wire = slab in
    let tx-buf = wrapping-add-i64 slab 4096 in
    let rx-buf = wrapping-add-i64 slab 8192 in
    -- Small 4 KiB terms (mtu = 256 => max_payload = 224B, default window = 2048B)
    -- so multi-fragment messages + backpressure + 3-partition rotations happen rapidly!
    let pub = aeron-pub-new 77 3 10 4096 256 2048 1 0 1000 in
    let sub = aeron-sub-new 77 3 10 4096 2048 9001 1 1000 in
    if not (aeron-pub-is-valid pub) || not (aeron-sub-is-valid sub) then
      let _0 = aeron-pub-destroy pub in
      let _1 = aeron-sub-destroy sub in 20
    else
      -- A. SETUP + SM handshake
      let s0 = aeron-pub-poll-tx pub wire 2048 1000 in
      let r-stp = aeron-sub-on-datagram sub wire s0.tx-len 1000 in
      let c0 = aeron-sub-poll-control sub wire 256 1000 in
      let r-sm = aeron-pub-on-control pub wire c0.ctrl-len in
      let ok-hs = (eq-u16 s0.tx-frame-type aeron-frame-type-setup
        && eq-u8 r-stp aeron-rx-setup
        && eq-u16 c0.ctrl-frame-type aeron-frame-type-sm
        && eq-u8 r-sm aeron-ctrl-sm
        && aeron-pub-connected pub && aeron-sub-connected sub) in

      -- B. Fragmentation: offer 600B message (> 224B max payload -> 3 fragments: BEGIN, mid, END)
      let _f1 = fill-pat tx-buf 0 600 41 in
      let p1 = aeron-pub-offer pub tx-buf 600 in
      let tx1 = pump-all-tx pub sub wire 2000 16 in
      let pol1 = aeron-sub-poll sub rx-buf 2048 in
      let ok-frag = (gt-i64 p1 0 && eq-u32 tx1 3
        && eq-u8 pol1.poll-ok 1 && eq-u32 pol1.poll-len 600
        && verify-pat rx-buf 0 600 41) in

      -- C. Zero-copy try-claim + commit + CRC tamper rejection on receiver
      let cl = aeron-pub-try-claim pub 120 in
      let _fc = fill-pat cl.claim-payload-addr 0 120 73 in
      let _cm = aeron-pub-commit pub cl in
      let t-cl = aeron-pub-poll-tx pub wire 2048 3000 in
      -- Tamper byte +40 before feeding to subscriber -> MUST be rejected with aeron-rx-bad-crc!
      let orig = raw-load-u8 (wrapping-add-i64 wire 40) in
      let _tm = raw-store-u8 (wrapping-add-i64 wire 40) (bit-xor-u8 orig 0x80) in
      let bad-r = aeron-sub-on-datagram sub wire t-cl.tx-len 3000 in
      let _rt = raw-store-u8 (wrapping-add-i64 wire 40) orig in
      -- Now feed untampered frame directly from zero-copy `t-cl.tx-frame-addr`!
      let good-r = aeron-sub-on-datagram sub t-cl.tx-frame-addr t-cl.tx-len 3000 in
      let pol-cl = aeron-sub-poll sub rx-buf 2048 in
      let ok-claim = (gt-i64 cl.claim-status 0
        && eq-u8 bad-r aeron-rx-bad-crc && eq-u64 (aeron-sub-bad-crc sub) 1
        && eq-u8 good-r aeron-rx-ok
        && eq-u8 pol-cl.poll-ok 1 && eq-u32 pol-cl.poll-len 120
        && verify-pat rx-buf 0 120 73) in

      -- D. Flow-control SM backpressure + multi-term rotation across all 3 partitions (4+ rotations)
      let _pc1 = pump-ctrl pub sub wire 10000000 in
      let _f2 = fill-pat tx-buf 0 512 99 in
      -- Fill the 2048B window without sending SM -> must hit aeron-offer-back-pressured (-1)!
      let bp-hit = offer-until-bp pub tx-buf 512 16 in
      -- Drain TX, poll sub, send SM -> publisher limit advances and offer succeeds!
      let _d1 = pump-all-tx pub sub wire 20000000 64 in
      let _p1 = drain-sub-poll sub rx-buf 2048 64 in
      let _pc2 = pump-ctrl pub sub wire 30000000 in
      let after-sm = aeron-pub-offer pub tx-buf 512 in
      -- Deliver `after-sm` cleanly so `stream-rotations` starts with an empty receive queue
      let _d-asm = pump-all-tx pub sub wire 35000000 16 in
      let _p-asm = aeron-sub-poll sub rx-buf 2048 in
      let _pc3 = pump-ctrl pub sub wire 36000000 in
      -- Stream 40 more 512B messages (~24 KiB > 6 * 4 KiB terms) across all 3 partitions!
      let rot-ok = stream-rotations pub sub wire tx-buf rx-buf 0 40 40000000 in
      let pub-rots = aeron-pub-term-rotations pub in
      let sub-rots = aeron-sub-term-rotations sub in

      -- E. Pacer SO_TXTIME & burst coalescing check
      let _pr = aeron-pub-set-pacing-rate pub 1000000 in -- 1 MB/s
      let t-pace = (100000000 :: U64) in
      let _of1 = aeron-pub-offer pub tx-buf 160 in
      let _of2 = aeron-pub-offer pub tx-buf 160 in
      let _of3 = aeron-pub-offer pub tx-buf 160 in
      let burst = aeron-pub-poll-tx-burst pub wire 2048 8 t-pace in
      let next-t = aeron-pub-next-send-time-ns pub t-pace in
      let ok-pace = gt-u32 burst.burst-frames 0 && gt-u32 burst.burst-bytes 0 && ge-u64 next-t t-pace in

      let _dp = aeron-pub-destroy pub in
      let _ds = aeron-sub-destroy sub in
      if not ok-hs then 21
      else if not ok-frag then 22
      else if not ok-claim then 23
      else if not bp-hit then 24
      else if lt-i64 after-sm 0 then 25
      else if not rot-ok || lt-u32 pub-rots 4 || lt-u32 sub-rots 4 then 26
      else if not ok-pace then 27
      else 0

  offer-until-bp :: AeronPublication -> I64 -> U32 -> U32 ->{MVar} Bool
  offer-until-bp pub buf len tries =
    if eq-u32 tries 0 then False
    else
      let r = aeron-pub-offer pub buf len in
      if eq-i64 r aeron-offer-back-pressured then True
      else offer-until-bp pub buf len (wrapping-sub-u32 tries 1)

  drain-sub-poll :: AeronSubscription -> I64 -> U32 -> U32 ->{MVar} U32
  drain-sub-poll sub rx cap lim =
    if eq-u32 lim 0 then 0
    else
      let p = aeron-sub-poll sub rx cap in
      if eq-u8 p.poll-ok 0 then 0
      else wrapping-add-u32 1 (drain-sub-poll sub rx cap (wrapping-sub-u32 lim 1))

  stream-rotations :: AeronPublication -> AeronSubscription -> I64 -> I64 -> I64 -> U32 -> U32 -> U64 ->{MVar} Bool
  stream-rotations pub sub wire tx-buf rx-buf i total now =
    if ge-u32 i total then True
    else
      let s8 = wrapping-u32-to-u8 (wrapping-add-u32 i 5) in
      let _f = fill-pat tx-buf 0 512 s8 in
      let o = aeron-pub-offer pub tx-buf 512 in
      if lt-i64 o 0 then False
      else
        let _tx = pump-all-tx pub sub wire now 16 in
        let pol = aeron-sub-poll sub rx-buf 2048 in
        let _pc = pump-ctrl pub sub wire (wrapping-add-u64 now 6000000) in
        if ne-u8 pol.poll-ok 1 || ne-u32 pol.poll-len 512 || not (verify-pat rx-buf 0 512 s8) then False
        else stream-rotations pub sub wire tx-buf rx-buf (wrapping-add-u32 i 1) total (wrapping-add-u64 now 10000000)

  -- 3. Lock-free local-host Disruptor IPC bridge test (500 messages crossing term rotations)
  ipc-loop :: AeronPublication -> Disruptor -> AeronIpcSub -> I64 -> I64 -> U32 -> U32 ->{MVar} Bool
  ipc-loop pub ring ipc-sub tx-buf rx-buf i total =
    if ge-u32 i total then True
    else
      let s8 = wrapping-u32-to-u8 (wrapping-add-u32 i 19) in
      let len = wrapping-add-u32 32 (rem-u32 (wrapping-mul-u32 i 29) 192) in
      let _f = fill-pat tx-buf 0 len s8 in
      let pos = aeron-ipc-publish pub ring tx-buf len in
      if lt-i64 pos 0 then False
      else
        let pol = aeron-ipc-poll pub ring ipc-sub rx-buf 1024 in
        if ne-u8 pol.poll-ok 1 || ne-u32 pol.poll-len len || not (verify-pat rx-buf 0 len s8) then False
        else ipc-loop pub ring ipc-sub tx-buf rx-buf (wrapping-add-u32 i 1) total

  test-disruptor-ipc :: I64 ->{IO, MVar} Int
  test-disruptor-ipc slab =
    let tx-buf = wrapping-add-i64 slab 4096 in
    let rx-buf = wrapping-add-i64 slab 8192 in
    let pub = aeron-pub-new 88 9 0 4096 256 2048 1 0 0 in
    let ipc-sub = aeron-ipc-sub-new () in
    case disruptor-new 64 ProducerSingle of
      None ->
        let _0 = aeron-ipc-sub-destroy ipc-sub in
        let _1 = aeron-pub-destroy pub in 30
      Some ring ->
        let ok = ipc-loop pub ring ipc-sub tx-buf rx-buf 0 500 in
        let rots = aeron-pub-term-rotations pub in
        let _dr = disruptor-destroy ring in
        let _ds = aeron-ipc-sub-destroy ipc-sub in
        let _dp = aeron-pub-destroy pub in
        if not ok then 31
        else if lt-u32 rots 10 then 32
        else 0

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 16384 4096 in
    if eq-u64 dma 0 then 99
    else
      let slab = wrapping-u64-to-i64 dma in
      let r1 = test-codecs slab in
      let r2 = if r1 == 0 then test-transport-features slab else r1 in
      let r3 = if r2 == 0 then test-disruptor-ipc slab else r2 in
      let _f = engine-dma-free dma 16384 in
      r3
"#;
    build_and_run_leak_checked("codec-transport-ipc", src, 0);
}

// =========================================================================
// 3. 5% Lossy + 10% Reordering + 5% Duplicate Deterministic UDP Simulation
//    over Engine.Sim: 2,000 Variable-Size Messages Crossing Many Terms
// =========================================================================

#[test]
fn net_aeron_5pct_lossy_reordering_2000_msgs_deterministic_sim() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Result
  import Data.String
  import Effect.Io
  import Text.Show
  import Engine.Sim
  import Engine.Sim.Faults
  import Engine.Types
  import Net.Aeron.Protocol
  import Net.Aeron.Transport
  import Prim.Atomic

  -- Deterministic variable message length in [24 .. 840] bytes (mixing
  -- unfragmented single frames and 2-fragment BEGIN/END messages across mtu=512).
  msg-len-for :: U32 -> U32
  msg-len-for idx =
    let h = sim-mix-u64 (wrapping-add-u64 (u32-to-u64 idx) 0x123456789ABCDEF0) in
    wrapping-add-u32 24 (wrapping-u64-to-u32 (rem-u64 h 817))

  msg-byte-for :: U32 -> U32 -> U8
  msg-byte-for idx off =
    let k = bit-xor-u64 (wrapping-mul-u64 (wrapping-add-u64 (u32-to-u64 idx) 1) 0x9E3779B97F4A7C15) (u32-to-u64 off) in
    wrapping-u64-to-u8 (bit-xor-u64 k (wrapping-shr-u64 k 23))

  fill-msg :: I64 -> U32 -> U32 -> U32 ->{MVar} Unit
  fill-msg buf idx i n =
    if ge-u32 i n then ()
    else
      let _ = raw-store-u8 (wrapping-add-i64 buf (u32-to-i64 i)) (msg-byte-for idx i) in
      fill-msg buf idx (wrapping-add-u32 i 1) n

  verify-msg :: I64 -> U32 -> U32 -> U32 ->{MVar} Bool
  verify-msg buf idx i n =
    if ge-u32 i n then True
    else
      let got = raw-load-u8 (wrapping-add-i64 buf (u32-to-i64 i)) in
      if eq-u8 got (msg-byte-for idx i) then verify-msg buf idx (wrapping-add-u32 i 1) n
      else False

  copy-dma-to-bytes :: I64 -> Bytes -> Nat -> Nat ->{MVar} Unit
  copy-dma-to-bytes src dst i n =
    if i < n && i < bytes-len dst then
      let _ = bytes-set-u8 dst i (raw-load-u8 (wrapping-add-i64 src (nat-to-i64 i))) in
      copy-dma-to-bytes src dst (i + 1) n
    else ()

  copy-bytes-to-dma :: Bytes -> I64 -> Nat -> Nat ->{MVar} Unit
  copy-bytes-to-dma src dst i n =
    if i < n && i < bytes-len src then
      let _ = raw-store-u8 (wrapping-add-i64 dst (nat-to-i64 i)) (bytes-get-u8 src i) in
      copy-bytes-to-dma src dst (i + 1) n
    else ()

  -- 5% drop + 10% reorder + 5% duplicate UDP fault profile
  lossy-udp-faults :: SimFaultConfig
  lossy-udp-faults = SimFaultConfig {
    udp-drop-ppm = 50000,
    udp-dup-ppm = 50000,
    udp-reorder-ppm = 100000,
    udp-corrupt-ppm = 0,
    udp-min-delay-ns = 400000,
    udp-max-delay-ns = 1800000,
    tcp-partial-io-ppm = 0,
    tcp-rst-ppm = 0,
    disk-write-delay-ns = 0,
    disk-torn-sector-ppm = 0,
    disk-bitflip-ppm = 0,
    partition-mask = 0
  }

  -- Drain up to `lim` outbound frames from pub and send over lossy SimWorld UDP.
  pump-pub-tx :: forall (r :: Region). SimWorld r -> Fd -> SimUdpAddr -> AeronPublication -> I64 -> U64 -> U32 ->{Mut r, MVar} U32
  pump-pub-tx w pfd saddr pub wire now lim =
    if eq-u32 lim 0 then 0
    else
      let r = aeron-pub-poll-tx pub wire 2048 now in
      if eq-u32 r.tx-len 0 then 0
      else
        let n = u32-to-nat r.tx-len in
        let pkt = bytes-new n in
        let _c = copy-dma-to-bytes wire pkt 0 n in
        let _s = sim-udp-send w pfd saddr pkt in
        wrapping-add-u32 1 (pump-pub-tx w pfd saddr pub wire now (wrapping-sub-u32 lim 1))

  -- Drain incoming UDP datagrams at subscriber and ingest into `AeronSubscription`.
  pump-sub-rx :: forall (r :: Region). SimWorld r -> Fd -> AeronSubscription -> I64 -> U64 -> U32 ->{Mut r, MVar} U32
  pump-sub-rx w sfd sub wire now lim =
    if eq-u32 lim 0 then 0
    else case sim-udp-recv w sfd of
      Err _ -> 0
      Ok dg ->
        let n = bytes-len dg.dgram-payload in
        let n32 = wrapping-i64-to-u32 (nat-to-i64 n) in
        let _c = copy-bytes-to-dma dg.dgram-payload wire 0 n in
        let _rx = aeron-sub-on-datagram sub wire n32 now in
        wrapping-add-u32 1 (pump-sub-rx w sfd sub wire now (wrapping-sub-u32 lim 1))

  -- Poll all currently ready ordered messages from `sub` and verify byte-exactness.
  -- Returns updated `delivered` count, or `0xFFFFFFFF` on any ordering/content mismatch!
  drain-and-verify :: AeronSubscription -> I64 -> U32 -> U32 ->{MVar} U32
  drain-and-verify sub rx-buf delivered lim =
    if eq-u32 lim 0 then delivered
    else
      let pol = aeron-sub-poll sub rx-buf 2048 in
      if eq-u8 pol.poll-ok 0 then delivered
      else
        let exp-len = msg-len-for delivered in
        if ne-u32 pol.poll-len exp-len then 0xFFFFFFFF
        else if not (verify-msg rx-buf delivered 0 exp-len) then 0xFFFFFFFF
        else drain-and-verify sub rx-buf (wrapping-add-u32 delivered 1) (wrapping-sub-u32 lim 1)

  -- Drain outbound control frames (NAK / SM) from sub and send over lossy SimWorld UDP.
  pump-sub-ctrl :: forall (r :: Region). SimWorld r -> Fd -> SimUdpAddr -> AeronSubscription -> I64 -> U64 -> U32 ->{Mut r, MVar} U32
  pump-sub-ctrl w sfd paddr sub wire now lim =
    if eq-u32 lim 0 then 0
    else
      let c = aeron-sub-poll-control sub wire 256 now in
      if eq-u32 c.ctrl-len 0 then 0
      else
        let n = u32-to-nat c.ctrl-len in
        let pkt = bytes-new n in
        let _c = copy-dma-to-bytes wire pkt 0 n in
        let _s = sim-udp-send w sfd paddr pkt in
        wrapping-add-u32 1 (pump-sub-ctrl w sfd paddr sub wire now (wrapping-sub-u32 lim 1))

  -- Drain incoming control frames (NAK / SM) at publisher.
  pump-pub-ctrl :: forall (r :: Region). SimWorld r -> Fd -> AeronPublication -> I64 -> U32 ->{Mut r, MVar} U32
  pump-pub-ctrl w pfd pub wire lim =
    if eq-u32 lim 0 then 0
    else case sim-udp-recv w pfd of
      Err _ -> 0
      Ok dg ->
        let n = bytes-len dg.dgram-payload in
        let n32 = wrapping-i64-to-u32 (nat-to-i64 n) in
        let _c = copy-bytes-to-dma dg.dgram-payload wire 0 n in
        let _ctrl = aeron-pub-on-control pub wire n32 in
        wrapping-add-u32 1 (pump-pub-ctrl w pfd pub wire (wrapping-sub-u32 lim 1))

  -- Offer up to `burst` messages per tick while not back-pressured.
  offer-batch :: AeronPublication -> I64 -> U32 -> U32 -> U32 ->{MVar} U32
  offer-batch pub tx-buf offered total burst =
    if ge-u32 offered total || eq-u32 burst 0 then offered
    else
      let mlen = msg-len-for offered in
      let _f = fill-msg tx-buf offered 0 mlen in
      let pos = aeron-pub-offer pub tx-buf mlen in
      if lt-i64 pos 0 then offered
      else offer-batch pub tx-buf (wrapping-add-u32 offered 1) total (wrapping-sub-u32 burst 1)

  -- Main discrete-event simulation loop driving 2,000 messages to 100% lossless delivery.
  sim-loop
    :: forall (r :: Region).
       SimWorld r -> Fd -> Fd -> SimUdpAddr -> SimUdpAddr
    -> AeronPublication -> AeronSubscription -> I64
    -> U32 -> U32 -> U32 -> U32 ->{IO r, Mut r, MVar} Int
  sim-loop w pfd sfd paddr saddr pub sub slab offered delivered total ticks-left =
    if eq-u32 delivered total then 0
    else if eq-u32 ticks-left 0 then 50
    else
      let wire = slab in
      let tx-buf = wrapping-add-i64 slab 4096 in
      let rx-buf = wrapping-add-i64 slab 8192 in
      let now = sim-world-now-ns w in
      -- 1. Receive control (SM/NAK) on publisher
      let _pc = pump-pub-ctrl w pfd pub wire 32 in
      -- 2. Offer next batch of messages if flow-control window permits
      let next-off = offer-batch pub tx-buf offered total 8 in
      -- 3. Emit outbound DATA / PAD / NAK-retransmits / heartbeats
      let _pt = pump-pub-tx w pfd saddr pub wire now 16 in
      -- 4. Receive datagrams on subscriber, reassemble, and verify strictly ordered delivery
      let _sr = pump-sub-rx w sfd sub wire now 32 in
      let next-del = drain-and-verify sub rx-buf delivered 32 in
      if eq-u32 next-del 0xFFFFFFFF then 51
      else
        -- 5. Emit subscriber NAKs / SMs
        let _sc = pump-sub-ctrl w sfd paddr sub wire now 8 in
        -- 6. Advance virtual simulation clock by 1 ms
        let _adv = sim-world-run-for w 1000000 in
        sim-loop w pfd sfd paddr saddr pub sub slab next-off next-del total (wrapping-sub-u32 ticks-left 1)

  main :: forall (r :: Region). Unit ->{IO, IO r, MVar, Mut r} Int
  main _ =
    let dma = engine-dma-alloc 16384 4096 in
    if eq-u64 dma 0 then 99
    else
      let slab = wrapping-u64-to-i64 dma in
      let w = sim-world-new 0xAE50123456789ABC lossy-udp-faults in
      case sim-udp-bind w 1 40123 of
        Err _ ->
          let _f = engine-dma-free dma 16384 in 91
        Ok pfd -> case sim-udp-bind w 2 40124 of
          Err _ ->
            let _c1 = sim-udp-close w pfd in
            let _f = engine-dma-free dma 16384 in 92
          Ok sfd ->
            let paddr = sim-udp-addr-make 1 40123 in
            let saddr = sim-udp-addr-make 2 40124 in
            -- 16 KiB term length, 512B MTU, 8 KiB receiver window, CRC32C enabled, 200 MB/s pacing
            let pub = aeron-pub-new 2026 1 100 16384 512 8192 1 200000000 0 in
            let sub = aeron-sub-new 2026 1 100 16384 8192 77777 1 0 in
            let _t1 = aeron-sub-set-nak-timers sub 200000 3000000 in
            let _t2 = aeron-sub-set-sm-interval sub 4000000 in
            let _t3 = aeron-pub-set-heartbeat-interval pub 3000000 in
            let rc = sim-loop w pfd sfd paddr saddr pub sub slab 0 0 2000 20000 in
            let msgs = aeron-sub-msgs-delivered sub in
            let bytes = aeron-sub-bytes-delivered sub in
            let rots = aeron-pub-term-rotations pub in
            let naks = aeron-sub-naks-sent sub in
            let retx-real = aeron-pub-retransmits pub in
            let dups = aeron-sub-dups-dropped sub in
            let sms = aeron-sub-sms-sent sub in
            let bad-crc = aeron-sub-bad-crc sub in
            let line = string-concat "SIM_AERON_5PCT msgs=" (string-concat (show (u32-to-nat (wrapping-u64-to-u32 msgs)))
              (string-concat " bytes=" (string-concat (show (u32-to-nat (wrapping-u64-to-u32 bytes)))
              (string-concat " rotations=" (string-concat (show (u32-to-nat rots))
              (string-concat " naks=" (string-concat (show (u32-to-nat (wrapping-u64-to-u32 naks)))
              (string-concat " retransmits=" (string-concat (show (u32-to-nat (wrapping-u64-to-u32 retx-real)))
              (string-concat " dups_dropped=" (string-concat (show (u32-to-nat (wrapping-u64-to-u32 dups)))
              (string-concat " sms=" (string-concat (show (u32-to-nat (wrapping-u64-to-u32 sms))) "\n"))))))))))))) in
            let _out = (with (io = newIO ()) in ioWrite (string-to-bytes line)) in
            let _dp = aeron-pub-destroy pub in
            let _ds = aeron-sub-destroy sub in
            let _c1 = sim-udp-close w pfd in
            let _c2 = sim-udp-close w sfd in
            let _fm = engine-dma-free dma 16384 in
            if rc /= 0 then rc
            else if ne-u64 msgs 2000 then 60
            else if lt-u32 rots 30 then 61
            else if eq-u64 naks 0 || eq-u64 retx-real 0 || eq-u64 dups 0 then 62
            else if ne-u64 bad-crc 0 then 63
            else 0
"#;
    let (stdout, _stderr) = build_and_run_leak_checked("sim-5pct-2000-msgs", src, 0);
    assert!(
        stdout.contains("SIM_AERON_5PCT msgs=2000"),
        "expected SIM_AERON_5PCT summary, got:\n{stdout}"
    );
    eprintln!("{stdout}");
}

// =========================================================================
// 4. Live macOS Loopback UDP Transport (`Engine.Reactor.Darwin`) +
//    50,000-Message Zero-Alloc / Zero-Atomic-RC Hot Loop
// =========================================================================

#[cfg(target_os = "macos")]
#[test]
fn net_aeron_darwin_live_udp_loopback_and_50000_zero_alloc_hot_loop() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Data.Result
  import Engine.Reactor.Darwin
  import Engine.Types
  import Net.Aeron.Protocol
  import Net.Aeron.Transport
  import Prim.Atomic

  #[no-alloc]
  fill-live :: I64 -> U32 -> U32 -> U32 ->{MVar} Unit
  fill-live buf seq i n =
    if ge-u32 i n then ()
    else
      let v = wrapping-u32-to-u8 (bit-xor-u32 (wrapping-mul-u32 (wrapping-add-u32 seq 1) 73) (wrapping-mul-u32 i 19)) in
      let _ = raw-store-u8 (wrapping-add-i64 buf (u32-to-i64 i)) v in
      fill-live buf seq (wrapping-add-u32 i 1) n

  #[no-alloc]
  verify-live :: I64 -> U32 -> U32 -> U32 ->{MVar} Bool
  verify-live buf seq i n =
    if ge-u32 i n then eq-u32 0 0
    else
      let got = raw-load-u8 (wrapping-add-i64 buf (u32-to-i64 i)) in
      let exp = wrapping-u32-to-u8 (bit-xor-u32 (wrapping-mul-u32 (wrapping-add-u32 seq 1) 73) (wrapping-mul-u32 i 19)) in
      if ne-u8 got exp then eq-u32 0 1
      else verify-live buf seq (wrapping-add-u32 i 1) n

  -- Drain all queued UDP datagrams on `sfd` into `sub`.
  drain-udp-sub :: Fd -> AeronSubscription -> I64 -> I64 -> I64 -> U64 -> U32 ->{IO, MVar} U32
  drain-udp-sub sfd sub rx-wire src-sa src-len now lim =
    if eq-u32 lim 0 then 0
    else
      let nr = darwin-udp-recvfrom-dma sfd rx-wire 2048 src-sa src-len in
      if gt-i32 nr 0 then
        let _ = aeron-sub-on-datagram sub rx-wire (wrapping-i32-to-u32 nr) now in
        wrapping-add-u32 1 (drain-udp-sub sfd sub rx-wire src-sa src-len now (wrapping-sub-u32 lim 1))
      else 0

  -- Drain all queued control UDP datagrams on `pfd` into `pub`.
  drain-udp-pub :: Fd -> AeronPublication -> I64 -> I64 -> I64 -> U32 ->{IO, MVar} U32
  drain-udp-pub pfd pub rx-wire src-sa src-len lim =
    if eq-u32 lim 0 then 0
    else
      let nr = darwin-udp-recvfrom-dma pfd rx-wire 256 src-sa src-len in
      if gt-i32 nr 0 then
        let _ = aeron-pub-on-control pub rx-wire (wrapping-i32-to-u32 nr) in
        wrapping-add-u32 1 (drain-udp-pub pfd pub rx-wire src-sa src-len (wrapping-sub-u32 lim 1))
      else 0

  -- Send up to `lim` outbound frames from `pub` directly over live loopback UDP (zero-copy from `tx.tx-frame-addr`!).
  send-udp-pub :: Fd -> I64 -> AeronPublication -> U64 -> U32 ->{IO, MVar} U32
  send-udp-pub pfd dst-sub pub now lim =
    if eq-u32 lim 0 then 0
    else
      let r = aeron-pub-poll-tx pub 0 0 now in
      if eq-u32 r.tx-len 0 then 0
      else
        let _ns = darwin-udp-sendto-dma pfd r.tx-frame-addr r.tx-len dst-sub in
        wrapping-add-u32 1 (send-udp-pub pfd dst-sub pub now (wrapping-sub-u32 lim 1))

  -- Send outbound control frames (NAK/SM) from `sub` over live loopback UDP.
  send-udp-ctrl :: Fd -> I64 -> AeronSubscription -> I64 -> U64 -> U32 ->{IO, MVar} U32
  send-udp-ctrl sfd dst-pub sub ctrl-buf now lim =
    if eq-u32 lim 0 then 0
    else
      let c = aeron-sub-poll-control sub ctrl-buf 256 now in
      if eq-u32 c.ctrl-len 0 then 0
      else
        let _ns = darwin-udp-sendto-dma sfd ctrl-buf c.ctrl-len dst-pub in
        wrapping-add-u32 1 (send-udp-ctrl sfd dst-pub sub ctrl-buf now (wrapping-sub-u32 lim 1))

  -- Drive 200 variable-size messages (crossing multiple 8 KiB term rotations + fragmentation)
  -- over live macOS loopback UDP sockets with kqueue readiness polling!
  live-udp-loop
    :: DarwinReactor -> Fd -> Fd -> AeronPublication -> AeronSubscription -> I64
    -> U32 -> U32 -> U32 -> U32 ->{IO, MVar} I32
  live-udp-loop rx pfd sfd pub sub slab offered delivered total budget =
    if ge-u32 delivered total then 0
    else if eq-u32 budget 0 then 40
    else
      -- Slab offsets:
      --   +0    : tx payload (2048B)
      --   +2048 : rx payload (2048B)
      --   +4096 : rx wire (2048B)
      --   +6144 : ctrl wire (256B)
      --   +6400 : dst-sub sockaddr (16B)
      --   +6432 : dst-pub sockaddr (16B)
      --   +6464 : src sockaddr (16B)
      --   +6496 : src-len (4B)
      --   +6656 : cqe-buf (256B)
      let tx-pl = slab in
      let rx-pl = wrapping-add-i64 slab 2048 in
      let rx-wire = wrapping-add-i64 slab 4096 in
      let ctrl-wire = wrapping-add-i64 slab 6144 in
      let dst-sub = wrapping-add-i64 slab 6400 in
      let dst-pub = wrapping-add-i64 slab 6432 in
      let src-sa = wrapping-add-i64 slab 6464 in
      let src-len = wrapping-add-i64 slab 6496 in
      let cqe-buf = wrapping-add-i64 slab 6656 in
      let now = darwin-clock-monotonic-ns () in
      let _pc = drain-udp-pub pfd pub rx-wire src-sa src-len 16 in
      let next-off = (if lt-u32 offered total then
          let mlen = wrapping-add-u32 48 (rem-u32 (wrapping-mul-u32 offered 137) 640) in
          let _f = fill-live tx-pl offered 0 mlen in
          let pos = aeron-pub-offer pub tx-pl mlen in
          if ge-i64 pos 0 then wrapping-add-u32 offered 1 else offered
        else offered) in
      let _st = send-udp-pub pfd dst-sub pub now 16 in
      let _sr = drain-udp-sub sfd sub rx-wire src-sa src-len now 32 in
      let pol = aeron-sub-poll sub rx-pl 2048 in
      let next-del = (if eq-u8 pol.poll-ok 1 then
          let exp-len = wrapping-add-u32 48 (rem-u32 (wrapping-mul-u32 delivered 137) 640) in
          if ne-u32 pol.poll-len exp-len then 0xFFFFFFFF
          else if not (verify-live rx-pl delivered 0 exp-len) then 0xFFFFFFFF
          else wrapping-add-u32 delivered 1
        else delivered) in
      if eq-u32 next-del 0xFFFFFFFF then 41
      else
        let _sc = send-udp-ctrl sfd dst-pub sub ctrl-wire now 8 in
        let _pl = if eq-u32 next-del delivered then darwin-reactor-poll rx 1000000 cqe-buf 16 else 0 in
        live-udp-loop rx pfd sfd pub sub slab next-off next-del total (wrapping-sub-u32 budget 1)

  test-live-darwin-udp :: I64 ->{IO, MVar} I32
  test-live-darwin-udp slab =
    case darwin-reactor-new 128 of
      Err _ -> 1
      Ok rx ->
        case darwin-udp-bind-reuseport 0 of
          Err _ ->
            let _ = darwin-reactor-destroy rx in 2
          Ok (pfd, pport) ->
            case darwin-udp-bind-reuseport 0 of
              Err _ ->
                let _ = darwin-close-fd pfd in
                let _ = darwin-reactor-destroy rx in 3
              Ok (sfd, sport) ->
                let dst-sub = wrapping-add-i64 slab 6400 in
                let dst-pub = wrapping-add-i64 slab 6432 in
                let cqe-buf = wrapping-add-i64 slab 6656 in
                let _sa1 = darwin-sockaddr-write-dma dst-sub ipv4-loopback sport in
                let _sa2 = darwin-sockaddr-write-dma dst-pub ipv4-loopback pport in
                let _a1 = darwin-reactor-arm-read rx pfd (io-token-pack op-code-recv-msg 1 1) in
                let _a2 = darwin-reactor-arm-read rx sfd (io-token-pack op-code-recv-msg 1 2) in
                let _p0 = darwin-reactor-poll rx 0 cqe-buf 16 in
                let now0 = darwin-clock-monotonic-ns () in
                let pub = aeron-pub-new 5050 2 1 8192 512 4096 1 0 now0 in
                let sub = aeron-sub-new 5050 2 1 8192 4096 8080 1 now0 in
                let _t1 = aeron-sub-set-nak-timers sub 200000 2000000 in
                let _t2 = aeron-sub-set-sm-interval sub 2000000 in
                let _t3 = aeron-pub-set-heartbeat-interval pub 2000000 in
                let rc = live-udp-loop rx pfd sfd pub sub slab 0 0 200 20000 in
                let rots = aeron-pub-term-rotations pub in
                let deliv = aeron-sub-msgs-delivered sub in
                let _dp = aeron-pub-destroy pub in
                let _ds = aeron-sub-destroy sub in
                let _c1 = darwin-close-fd sfd in
                let _c2 = darwin-close-fd pfd in
                let _dr = darwin-reactor-destroy rx in
                if ne-i32 rc 0 then rc
                else if ne-u64 deliv 200 || lt-u32 rots 5 then 42
                else 0

  -- Steady-state 50,000-message #[no-alloc] hot loop exercising:
  --   aeron-pub-offer -> aeron-pub-poll-tx (zero-copy!) -> aeron-sub-on-datagram ->
  --   aeron-sub-poll -> aeron-sub-poll-control (SM) -> aeron-pub-on-control
  -- across > 70 term rotations with 0 heap allocations and 0 atomic RC ops!
  #[no-alloc]
  drain-tx-hot :: AeronPublication -> AeronSubscription -> U64 -> U32 ->{MVar} U32
  drain-tx-hot pub sub now lim =
    if eq-u32 lim 0 then 0
    else
      let r = aeron-pub-poll-tx pub 0 0 now in
      if eq-u32 r.tx-len 0 then 0
      else
        let _rx = aeron-sub-on-datagram sub r.tx-frame-addr r.tx-len now in
        wrapping-add-u32 1 (drain-tx-hot pub sub now (wrapping-sub-u32 lim 1))

  #[no-alloc]
  hot-fifty-k-loop :: AeronPublication -> AeronSubscription -> I64 -> I64 -> I64 -> U32 -> U32 -> U64 ->{MVar} U64
  hot-fifty-k-loop pub sub tx-pl rx-pl ctrl-buf iter limit ok-cnt =
    if ge-u32 iter limit then ok-cnt
    else
      let now = wrapping-mul-u64 (u32-to-u64 iter) 1000 in
      let _w = raw-store-u64 tx-pl (u32-to-u64 iter) in
      let _w2 = raw-store-u64 (wrapping-add-i64 tx-pl 8) (bit-xor-u64 (u32-to-u64 iter) 0xA5A55A5A12345678) in
      let pos = aeron-pub-offer pub tx-pl 64 in
      let _tx = drain-tx-hot pub sub now 4 in
      let pol = aeron-sub-poll sub rx-pl 256 in
      let ctrl = aeron-sub-poll-control sub ctrl-buf 256 now in
      let _pc = if gt-u32 ctrl.ctrl-len 0 then aeron-pub-on-control pub ctrl-buf ctrl.ctrl-len else 0 in
      let match-ok = (if ge-i64 pos 0 then
          if eq-u8 pol.poll-ok 1 then
            if eq-u32 pol.poll-len 64 then eq-u64 (raw-load-u64 rx-pl) (u32-to-u64 iter)
            else eq-u32 0 1
          else eq-u32 0 1
        else eq-u32 0 1) in
      let next-ok = if match-ok then wrapping-add-u64 ok-cnt 1 else ok-cnt in
      hot-fifty-k-loop pub sub tx-pl rx-pl ctrl-buf (wrapping-add-u32 iter 1) limit next-ok

  test-fifty-k-zero-alloc :: I64 ->{IO, MVar} I32
  test-fifty-k-zero-alloc slab =
    let tx-pl = slab in
    let rx-pl = wrapping-add-i64 slab 2048 in
    let ctrl-buf = wrapping-add-i64 slab 4096 in
    let pub = aeron-pub-new 999 1 0 65536 1408 32768 1 0 0 in
    let sub = aeron-sub-new 999 1 0 65536 32768 12345 1 0 in
    -- Complete initial SETUP + SM outside hot loop
    let _tx0 = drain-tx-hot pub sub 0 4 in
    let c0 = aeron-sub-poll-control sub ctrl-buf 256 0 in
    let _pc0 = if gt-u32 c0.ctrl-len 0 then aeron-pub-on-control pub ctrl-buf c0.ctrl-len else 0 in
    -- Reset thread counters right before 50,000 steady-state cycles!
    let _rst = engine-thread-counters-reset () in
    let ok-cnt = hot-fifty-k-loop pub sub tx-pl rx-pl ctrl-buf 0 50000 0 in
    let allocs = engine-thread-alloc-count () in
    let atomic-rcs = engine-thread-atomic-rc-count () in
    let rots = aeron-pub-term-rotations pub in
    let _dp = aeron-pub-destroy pub in
    let _ds = aeron-sub-destroy sub in
    if ne-u64 ok-cnt 50000 then 50
    else if ne-u64 allocs 0 then 51
    else if ne-u64 atomic-rcs 0 then 52
    else if lt-u32 rots 50 then 53
    else 0

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 16384 4096 in
    if eq-u64 dma 0 then 99
    else
      let slab = wrapping-u64-to-i64 dma in
      let r1 = test-live-darwin-udp slab in
      let r2 = if eq-i32 r1 0 then test-fifty-k-zero-alloc slab else r1 in
      let _f = engine-dma-free dma 16384 in
      int-from-i64 (i32-to-i64 r2)
"#;
    build_and_run_leak_checked("darwin-udp-and-50k-zero-alloc", src, 0);
}
