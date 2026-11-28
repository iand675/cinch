//! Integration, RFC 6455 / RFC 8441 handshake & Autobahn conformance corpus,
//! 4-core pinned OS worker SPSC Pub/Sub broadcast (`WsHub` + `CoreMesh`), and
//! 50,000-frame steady-state zero-allocation hot-loop tests for
//! `Net.Http.WebSocket.{Frame,Hub}`.
//!
//! Routed through `common::drive_with_profile_opts` so the exact same suite
//! runs against both the Rust seed and the Stage 3 self-hosted compiler.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{DriverOptions, Mode, Package};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn fresh_dir(tag: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "cinch-net-http-ws-{tag}-{}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

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
    let exe = dir.join(format!("ws-{tag}"));
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
fn net_http_ws_manifest_registration_and_no_alloc_check() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, rel_path) in [
        ("Net.Http.WebSocket.Frame", "Net/Http/WebSocket/Frame.cinch"),
        ("Net.Http.WebSocket.Hub", "Net/Http/WebSocket/Hub.cinch"),
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

    let pos_mask = pkg.order.iter().position(|n| n == "Engine.Simd.Mask").unwrap();
    let pos_slab = pkg.order.iter().position(|n| n == "Engine.Mem.Slab").unwrap();
    let pos_spsc = pkg.order.iter().position(|n| n == "Engine.Ring.Spsc").unwrap();
    let pos_http = pkg.order.iter().position(|n| n == "Net.Http.Types").unwrap();
    let pos_sha1 = pkg.order.iter().position(|n| n == "Text.Sha1").unwrap();
    let pos_b64 = pkg.order.iter().position(|n| n == "Text.Base64").unwrap();
    let pos_frame = pkg.order.iter().position(|n| n == "Net.Http.WebSocket.Frame").unwrap();
    let pos_hub = pkg.order.iter().position(|n| n == "Net.Http.WebSocket.Hub").unwrap();
    assert!(pos_mask < pos_frame && pos_http < pos_frame && pos_sha1 < pos_frame && pos_b64 < pos_frame);
    assert!(pos_frame < pos_hub && pos_slab < pos_hub && pos_spsc < pos_hub);

    // Compile-time #[no-alloc] audit across all hot-path Frame & Hub entry points
    let dir = fresh_dir("no-alloc-check");
    std::fs::write(
        dir.join("Main.cinch"),
        r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Engine.Types
  import Net.Http.Types
  import Net.Http.WebSocket.Frame
  import Net.Http.WebSocket.Hub

  #[no-alloc]
  audit-ws-hot-path :: WsHub -> I64 -> I64 -> I64 ->{MVar} U32
  audit-ws-hot-path hub rx-buf tx-buf key-buf =
    let _acc = ws-compute-accept-dma key-buf 24 tx-buf in
    let _vk = ws-handshake-validate-h1-key-dma key-buf 24 in
    let n101 = ws-handshake-write-upgrade-dma tx-buf 0 256 key-buf 24 in
    let _c84 = ws-rfc8441-validate-connect-dma method-code-connect rx-buf 0 9 rx-buf 0 5 4 4 rx-buf 0 2 in
    let _h2 = ws-rfc8441-write-ok-h2-dma tx-buf 0 16 in
    let _h3 = ws-rfc8441-write-ok-h3-dma tx-buf 0 16 in
    let _u1 = ws-utf8-validate-step ws-utf8-accept 65 in
    let _u2 = ws-utf8-validate-raw ws-utf8-accept rx-buf 16 in
    let ps = ws-parser-init-server ws-default-max-frame-size in
    let _hdr = ws-parse-header-dma ps rx-buf 0 32 in
    let fr = ws-parse-frame-dma ps rx-buf 0 32 in
    let _ps2 = ws-parser-apply ps fr in
    let _e1 = ws-frame-encode-raw tx-buf 256 (eq-u32 0 0) ws-op-text rx-buf 8 in
    let _e2 = ws-frame-encode-headroom-raw (wrapping-add-i64 tx-buf 16) 8 (eq-u32 0 0) ws-op-binary in
    let _e3 = ws-frame-encode-masked-raw tx-buf 256 (eq-u32 0 0) ws-op-text 305419896 rx-buf 8 in
    let _ep = ws-encode-pong-dma tx-buf 256 rx-buf 4 in
    let _ec = ws-encode-close-dma tx-buf 256 ws-close-normal rx-buf 0 in
    let conn = ws-hub-connect hub (fd-of 7) 0 in
    let sub = ws-hub-subscribe hub conn 0 in
    let _pub = ws-hub-publish hub 0 ws-op-text (wrapping-i64-to-u64 rx-buf) 8 1 100 in
    let _dr = ws-hub-drain-remote hub 16 200 in
    let _ct = ws-hub-handle-control-dma hub conn fr rx-buf tx-buf 256 in
    let _un = ws-hub-unsubscribe hub sub in
    let _dc = ws-hub-disconnect hub conn in
    wrapping-add-u32 n101 (wrapping-add-u32 _pub.pub-local-delivered _dr.drain-delivered)

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
    .expect("Net.Http.WebSocket.{Frame,Hub} #[no-alloc] hot-path audit must check cleanly");
    let _ = std::fs::remove_dir_all(&dir);
}

// =========================================================================
// 2. RFC 6455 Handshake, RFC 8441 Extended CONNECT & Autobahn Conformance
// =========================================================================

#[test]
fn rfc6455_handshake_and_autobahn_conformance_suite() {
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
  import Net.Http.WebSocket.Frame
  import Net.Http.WebSocket.Hub
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

  dma-eq-str :: I64 -> U32 -> U32 -> String ->{MVar} Bool
  dma-eq-str base off len expected =
    let b = string-to-bytes expected in
    if u32-to-nat len /= bytes-len b then False
    else dma-eq-bytes base off b 0 (bytes-len b)

  dma-eq-bytes :: I64 -> U32 -> Bytes -> Nat -> Nat ->{MVar} Bool
  dma-eq-bytes base off b i n =
    if i >= n then True
    else if i < bytes-len b then
      let got = raw-load-u8 (wrapping-add-i64 base (wrapping-add-i64 (u32-to-i64 off) (nat-to-i64 i))) in
      let exp = bytes-get-u8 b i in
      if eq-u8 got exp then dma-eq-bytes base off b (i + 1) n else False
    else False

  fill-pattern :: I64 -> U32 -> U32 ->{MVar} Unit
  fill-pattern dst i n =
    if ge-u32 i n then ()
    else
      let v = wrapping-add-u8 (wrapping-u32-to-u8 (rem-u32 i 94)) 33 in
      let _ = raw-store-u8 (wrapping-add-i64 dst (u32-to-i64 i)) v in
      fill-pattern dst (wrapping-add-u32 i 1) n

  verify-pattern :: I64 -> U32 -> U32 ->{MVar} Bool
  verify-pattern src i n =
    if ge-u32 i n then True
    else
      let exp = wrapping-add-u8 (wrapping-u32-to-u8 (rem-u32 i 94)) 33 in
      let got = raw-load-u8 (wrapping-add-i64 src (u32-to-i64 i)) in
      if eq-u8 got exp then verify-pattern src (wrapping-add-u32 i 1) n else False

  -- Encode a masked client frame with `plen` bytes of deterministic ASCII pattern,
  -- verify incremental NeedMore on partial header/payload, parse + SIMD unmask, and check exact payload!
  test-payload-class :: I64 -> I64 -> U32 -> U32 -> U32 ->{MVar} Bool
  test-payload-class src-buf wire-buf plen mask-key expect-hdr-len =
    let _f = fill-pattern src-buf 0 plen in
    let wlen = ws-frame-encode-masked-raw wire-buf 131072 True ws-op-binary mask-key src-buf plen in
    let ps = ws-parser-init-server 131072 in
    -- Check incremental NeedMore at 1 byte and at wlen - 1 (when wlen > 1)
    let r-short1 = ws-parse-frame-dma ps wire-buf 0 1 in
    let r-short2 = ws-parse-frame-dma ps wire-buf 0 (wrapping-sub-u32 wlen 1) in
    let r-full = ws-parse-frame-dma ps wire-buf 0 wlen in
    eq-u32 wlen (wrapping-add-u32 expect-hdr-len plen)
      && eq-u8 r-short1.wfr-status ws-parse-need-more
      && eq-u8 r-short2.wfr-status ws-parse-need-more
      && eq-u8 r-full.wfr-status ws-parse-ok
      && eq-u16 r-full.wfr-header-len (wrapping-u32-to-u16 expect-hdr-len)
      && eq-u32 r-full.wfr-payload-len plen
      && eq-u32 r-full.wfr-consumed wlen
      && eq-u8 r-full.wfr-opcode ws-op-binary
      && eq-u8 r-full.wfr-fin 1
      && verify-pattern (wrapping-add-i64 wire-buf (u32-to-i64 r-full.wfr-payload-off)) 0 plen

  -- Helper: check a raw 2-byte header + masked payload triggers expected status & close code.
  check-raw-text-bytes :: I64 -> I64 -> U32 -> U8 -> U16 ->{MVar} Bool
  check-raw-text-bytes payload wire plen want-status want-code =
    let wlen = ws-frame-encode-masked-raw wire 4096 True ws-op-text 0x37FA213D payload plen in
    let ps = ws-parser-init-server 4096 in
    let res = ws-parse-frame-dma ps wire 0 wlen in
    eq-u8 res.wfr-status want-status && eq-u16 res.wfr-close-code want-code

  -- Helper: test a Close frame with given 16-bit status code (masked client -> server).
  check-close-code :: I64 -> I64 -> U16 -> U8 -> U16 ->{MVar} Bool
  check-close-code pbuf wire code want-status want-close =
    let _0 = raw-store-u8 pbuf (wrapping-u16-to-u8 (wrapping-shr-u16 code 8)) in
    let _1 = raw-store-u8 (wrapping-add-i64 pbuf 1) (wrapping-u16-to-u8 code) in
    let wlen = ws-frame-encode-masked-raw wire 256 True ws-op-close 0x11223344 pbuf 2 in
    let ps = ws-parser-init-server 4096 in
    let res = ws-parse-frame-dma ps wire 0 wlen in
    eq-u8 res.wfr-status want-status && eq-u16 res.wfr-close-code want-close

  -- Part 1 + 2: RFC 6455 §1.3 Handshake + RFC 8441 Extended CONNECT
  test-part1-and2 :: I64 -> I64 -> I64 ->{MVar} Int
  test-part1-and2 key-buf out-buf pbuf =
    let acc-str = ws-compute-accept "dGhlIHNhbXBsZSBub25jZQ==" in
    let klen = copy-str-to-dma "dGhlIHNhbXBsZSBub25jZQ==" key-buf in
    let acc-len = ws-compute-accept-dma key-buf klen out-buf in
    let key-ok = ws-handshake-validate-h1-key-dma key-buf klen in
    let bad-klen = copy-str-to-dma "not-a-valid-key" pbuf in
    let key-bad = not (ws-handshake-validate-h1-key-dma pbuf bad-klen) in
    let resp-len = ws-handshake-write-upgrade-dma out-buf 64 512 key-buf klen in
    let ok-h1 = (
      string-compare acc-str "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=" == 0
        && eq-u32 acc-len 28
        && dma-eq-str out-buf 0 28 "s3pPLMBiTxaQ9kYGzzhZRbK+xOo="
        && key-ok && key-bad
        && eq-u32 resp-len 193
        && dma-eq-str out-buf 64 129 "HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=\r\n\r\n") in
    if not ok-h1 then 10
    else
      let p-ws = copy-str-to-dma "websocket" pbuf in
      let p-bad = copy-str-to-dma "webtransp" (wrapping-add-i64 pbuf 32) in
      let s-https = copy-str-to-dma "https" (wrapping-add-i64 pbuf 64) in
      let s-ftp = copy-str-to-dma "ftp" (wrapping-add-i64 pbuf 96) in
      let v13 = copy-str-to-dma "13" (wrapping-add-i64 pbuf 128) in
      let v12 = copy-str-to-dma "12" (wrapping-add-i64 pbuf 144) in
      let r84-ok = ws-rfc8441-validate-connect-dma method-code-connect pbuf 0 p-ws pbuf 64 s-https 10 5 pbuf 128 v13 in
      let r84-bad-meth = not (ws-rfc8441-validate-connect-dma method-code-get pbuf 0 p-ws pbuf 64 s-https 10 5 pbuf 128 v13) in
      let r84-bad-proto = not (ws-rfc8441-validate-connect-dma method-code-connect pbuf 32 p-bad pbuf 64 s-https 10 5 pbuf 128 v13) in
      let r84-bad-scheme = not (ws-rfc8441-validate-connect-dma method-code-connect pbuf 0 p-ws pbuf 96 s-ftp 10 5 pbuf 128 v13) in
      let r84-bad-ver = not (ws-rfc8441-validate-connect-dma method-code-connect pbuf 0 p-ws pbuf 64 s-https 10 5 pbuf 144 v12) in
      let r84-bad-auth = not (ws-rfc8441-validate-connect-dma method-code-connect pbuf 0 p-ws pbuf 64 s-https 0 5 pbuf 128 v13) in
      let h2-n = ws-rfc8441-write-ok-h2-dma out-buf 0 16 in
      let h3-n = ws-rfc8441-write-ok-h3-dma out-buf 16 32 in
      let ok-r84 = (
        r84-ok && r84-bad-meth && r84-bad-proto && r84-bad-scheme && r84-bad-ver && r84-bad-auth
          && eq-u32 h2-n 1 && eq-u8 (raw-load-u8 out-buf) 136
          && eq-u32 h3-n 19
          && eq-u8 (raw-load-u8 (wrapping-add-i64 out-buf 16)) 0
          && eq-u8 (raw-load-u8 (wrapping-add-i64 out-buf 17)) 0
          && eq-u8 (raw-load-u8 (wrapping-add-i64 out-buf 18)) 217
          && eq-u16 (ws-rfc8441-ok-status ()).status-code 200) in
      if not ok-r84 then 11 else 0

  -- Part 3: All 3 payload length classes (<=125, 126/16-bit, 127/64-bit) + MSB=1 + max-frame-size
  test-part3-lengths :: I64 -> I64 ->{MVar} Int
  test-part3-lengths src-buf wire-buf =
    let ok-l0 = test-payload-class src-buf wire-buf 0 0xAABBCCDD 6 in
    let ok-l1 = test-payload-class src-buf wire-buf 1 0x12345678 6 in
    let ok-l125 = test-payload-class src-buf wire-buf 125 0xDEADBEEF 6 in
    let ok-l126 = test-payload-class src-buf wire-buf 126 0xCAFEBABE 8 in
    let ok-l1000 = test-payload-class src-buf wire-buf 1000 0x01020304 8 in
    let ok-l65535 = test-payload-class src-buf wire-buf 65535 0x55AA33CC 8 in
    let ok-l70000 = test-payload-class src-buf wire-buf 70000 0x99887766 14 in
    let _msb0 = raw-store-u8 wire-buf 130 in
    let _msb1 = raw-store-u8 (wrapping-add-i64 wire-buf 1) 255 in
    let _msb2 = raw-store-u64 (wrapping-add-i64 wire-buf 2) (swap-bytes-u64 9223372036854775809) in
    let _msb3 = raw-store-u32 (wrapping-add-i64 wire-buf 10) 0x11223344 in
    let r-msb = ws-parse-header-dma (ws-parser-init-server ws-default-max-frame-size) wire-buf 0 14 in
    let _big = ws-frame-encode-header-dma wire-buf 64 True ws-op-binary True 0x11223344 2048 in
    let r-big = ws-parse-header-dma (ws-parser-init-server 1024) wire-buf 0 16 in
    let ok-lens = (
      ok-l0 && ok-l1 && ok-l125 && ok-l126 && ok-l1000 && ok-l65535 && ok-l70000
        && eq-u8 r-msb.wfh-status ws-parse-err-protocol && eq-u16 r-msb.wfh-close-code ws-close-protocol-error
        && eq-u8 r-big.wfh-status ws-parse-err-too-big && eq-u16 r-big.wfh-close-code ws-close-message-too-big) in
    if ok-lens then 0 else 12

  -- Part 4: Fragmented multi-frame Text with split 4-byte UTF-8 + interleaved Ping/Pong
  test-part4-fragments :: I64 -> I64 ->{MVar} (Int, WsFrameResult)
  test-part4-fragments pbuf wire-buf =
    let _e0 = raw-store-u8 pbuf 240 in
    let _e1 = raw-store-u8 (wrapping-add-i64 pbuf 1) 159 in
    let _e2 = raw-store-u8 (wrapping-add-i64 pbuf 2) 154 in
    let _e3 = raw-store-u8 (wrapping-add-i64 pbuf 3) 128 in
    let ping-len = copy-str-to-dma "hi-ping" (wrapping-add-i64 pbuf 16) in
    let f1-n = ws-frame-encode-masked-raw wire-buf 256 False ws-op-text 0x10203040 pbuf 1 in
    let f2-n = ws-frame-encode-masked-raw (wrapping-add-i64 wire-buf (u32-to-i64 f1-n)) 256 False ws-op-cont 0x20304050 (wrapping-add-i64 pbuf 1) 1 in
    let fp-n = ws-frame-encode-masked-raw (wrapping-add-i64 wire-buf (u32-to-i64 (wrapping-add-u32 f1-n f2-n))) 256 True ws-op-ping 0x30405060 (wrapping-add-i64 pbuf 16) ping-len in
    let off-f3 = wrapping-add-u32 (wrapping-add-u32 f1-n f2-n) fp-n in
    let f3-n = ws-frame-encode-masked-raw (wrapping-add-i64 wire-buf (u32-to-i64 off-f3)) 256 False ws-op-cont 0x40506070 (wrapping-add-i64 pbuf 2) 1 in
    let off-f4 = wrapping-add-u32 off-f3 f3-n in
    let f4-n = ws-frame-encode-masked-raw (wrapping-add-i64 wire-buf (u32-to-i64 off-f4)) 256 True ws-op-cont 0x50607080 (wrapping-add-i64 pbuf 3) 1 in
    let tot-frag = wrapping-add-u32 off-f4 f4-n in
    let ps0 = ws-parser-init-server 4096 in
    let rf1 = ws-parse-frame-dma ps0 wire-buf 0 tot-frag in
    let ps1 = ws-parser-apply ps0 rf1 in
    let rf2 = ws-parse-frame-dma ps1 wire-buf rf1.wfr-consumed tot-frag in
    let ps2 = ws-parser-apply ps1 rf2 in
    let off2 = wrapping-add-u32 rf1.wfr-consumed rf2.wfr-consumed in
    let rfp = ws-parse-frame-dma ps2 wire-buf off2 tot-frag in
    let ps3 = ws-parser-apply ps2 rfp in
    let off3 = wrapping-add-u32 off2 rfp.wfr-consumed in
    let rf3 = ws-parse-frame-dma ps3 wire-buf off3 tot-frag in
    let ps4 = ws-parser-apply ps3 rf3 in
    let off4 = wrapping-add-u32 off3 rf3.wfr-consumed in
    let rf4 = ws-parse-frame-dma ps4 wire-buf off4 tot-frag in
    let ps5 = ws-parser-apply ps4 rf4 in
    -- Also test interleaved new Text while fragmented message in progress (ps1) -> 1002
    let _iv0 = raw-store-u8 (wrapping-add-i64 wire-buf 512) 129 in
    let _iv1 = raw-store-u8 (wrapping-add-i64 wire-buf 513) 128 in
    let _iv2 = raw-store-u32 (wrapping-add-i64 wire-buf 514) 0 in
    let e-interleave = eq-u16 (ws-parse-frame-dma ps1 wire-buf 512 518).wfr-close-code 1002 in
    let ok-frag = (
      eq-u8 rf1.wfr-status ws-parse-ok && eq-u8 rf1.wfr-fin 0 && eq-u8 ps1.ws-ps-frag-opcode ws-op-text
        && eq-u8 rf2.wfr-status ws-parse-ok && eq-u8 rf2.wfr-msg-opcode ws-op-text
        && eq-u8 rfp.wfr-status ws-parse-ok && eq-u8 rfp.wfr-opcode ws-op-ping
        && dma-eq-str wire-buf rfp.wfr-payload-off rfp.wfr-payload-len "hi-ping"
        && eq-u8 ps3.ws-ps-frag-opcode ws-op-text
        && eq-u8 rf3.wfr-status ws-parse-ok
        && eq-u8 rf4.wfr-status ws-parse-ok && eq-u8 rf4.wfr-fin 1
        && eq-u8 ps5.ws-ps-frag-opcode 0 && eq-u8 ps5.ws-ps-utf8-state ws-utf8-accept
        && e-interleave) in
    (if ok-frag then 0 else 13, rfp)

  -- Part 5: Strict UTF-8 Autobahn torture cases (valid + invalid -> 1007)
  test-part5-utf8 :: I64 -> I64 ->{MVar} Int
  test-part5-utf8 pbuf wire-buf =
    let vlen = copy-str-to-dma "Hello WebSocket SIMD UTF-8: café € 日本語 🚀" pbuf in
    let u-valid = check-raw-text-bytes pbuf wire-buf vlen ws-parse-ok 0 in
    let _ = raw-store-u8 pbuf 192 in
    let _ = raw-store-u8 (wrapping-add-i64 pbuf 1) 175 in
    let u-ov2 = check-raw-text-bytes pbuf wire-buf 2 ws-parse-err-invalid-utf8 ws-close-invalid-payload in
    let _ = raw-store-u8 pbuf 224 in
    let _ = raw-store-u8 (wrapping-add-i64 pbuf 1) 128 in
    let _ = raw-store-u8 (wrapping-add-i64 pbuf 2) 128 in
    let u-ov3 = check-raw-text-bytes pbuf wire-buf 3 ws-parse-err-invalid-utf8 ws-close-invalid-payload in
    let _ = raw-store-u8 pbuf 240 in
    let _ = raw-store-u8 (wrapping-add-i64 pbuf 1) 128 in
    let _ = raw-store-u8 (wrapping-add-i64 pbuf 2) 128 in
    let _ = raw-store-u8 (wrapping-add-i64 pbuf 3) 128 in
    let u-ov4 = check-raw-text-bytes pbuf wire-buf 4 ws-parse-err-invalid-utf8 ws-close-invalid-payload in
    let _ = raw-store-u8 pbuf 237 in
    let _ = raw-store-u8 (wrapping-add-i64 pbuf 1) 160 in
    let _ = raw-store-u8 (wrapping-add-i64 pbuf 2) 128 in
    let u-sur1 = check-raw-text-bytes pbuf wire-buf 3 ws-parse-err-invalid-utf8 ws-close-invalid-payload in
    let _ = raw-store-u8 pbuf 237 in
    let _ = raw-store-u8 (wrapping-add-i64 pbuf 1) 191 in
    let _ = raw-store-u8 (wrapping-add-i64 pbuf 2) 191 in
    let u-sur2 = check-raw-text-bytes pbuf wire-buf 3 ws-parse-err-invalid-utf8 ws-close-invalid-payload in
    let _ = raw-store-u8 pbuf 244 in
    let _ = raw-store-u8 (wrapping-add-i64 pbuf 1) 144 in
    let _ = raw-store-u8 (wrapping-add-i64 pbuf 2) 128 in
    let _ = raw-store-u8 (wrapping-add-i64 pbuf 3) 128 in
    let u-hi1 = check-raw-text-bytes pbuf wire-buf 4 ws-parse-err-invalid-utf8 ws-close-invalid-payload in
    let _ = raw-store-u8 pbuf 245 in
    let u-hi2 = check-raw-text-bytes pbuf wire-buf 1 ws-parse-err-invalid-utf8 ws-close-invalid-payload in
    let _ = raw-store-u8 pbuf 128 in
    let u-cont = check-raw-text-bytes pbuf wire-buf 1 ws-parse-err-invalid-utf8 ws-close-invalid-payload in
    let _ = raw-store-u8 pbuf 226 in
    let _ = raw-store-u8 (wrapping-add-i64 pbuf 1) 130 in
    let u-trunc = check-raw-text-bytes pbuf wire-buf 2 ws-parse-err-invalid-utf8 ws-close-invalid-payload in
    let _ = raw-store-u8 pbuf 3 in
    let _ = raw-store-u8 (wrapping-add-i64 pbuf 1) 232 in
    let _ = raw-store-u8 (wrapping-add-i64 pbuf 2) 255 in
    let cl-bad-u-len = ws-frame-encode-masked-raw wire-buf 256 True ws-op-close 0x12341234 pbuf 3 in
    let cl-bad-u = ws-parse-frame-dma (ws-parser-init-server 4096) wire-buf 0 cl-bad-u-len in
    let ok-utf8 = (
      u-valid && u-ov2 && u-ov3 && u-ov4 && u-sur1 && u-sur2 && u-hi1 && u-hi2 && u-cont && u-trunc
        && eq-u8 cl-bad-u.wfr-status ws-parse-err-invalid-utf8
        && eq-u16 cl-bad-u.wfr-close-code ws-close-invalid-payload) in
    if ok-utf8 then 0 else 14

  -- Part 6: All RFC 6455 protocol error cases -> 1002
  test-part6-proto :: I64 -> I64 ->{MVar} Int
  test-part6-proto pbuf wire-buf =
    let ps-srv = ws-parser-init-server 4096 in
    let ps-cli = ws-parser-init-client 4096 in
    let _ = raw-store-u8 wire-buf 193 in
    let _ = raw-store-u8 (wrapping-add-i64 wire-buf 1) 128 in
    let _ = raw-store-u32 (wrapping-add-i64 wire-buf 2) 0 in
    let e-rsv1 = eq-u16 (ws-parse-frame-dma ps-srv wire-buf 0 6).wfr-close-code 1002 in
    let _ = raw-store-u8 wire-buf 161 in
    let e-rsv2 = eq-u16 (ws-parse-frame-dma ps-srv wire-buf 0 6).wfr-close-code 1002 in
    let _ = raw-store-u8 wire-buf 145 in
    let e-rsv3 = eq-u16 (ws-parse-frame-dma ps-srv wire-buf 0 6).wfr-close-code 1002 in
    let _ = raw-store-u8 wire-buf 129 in
    let _ = raw-store-u8 (wrapping-add-i64 wire-buf 1) 0 in
    let e-unmasked = eq-u16 (ws-parse-frame-dma ps-srv wire-buf 0 2).wfr-close-code 1002 in
    let _ = raw-store-u8 wire-buf 129 in
    let _ = raw-store-u8 (wrapping-add-i64 wire-buf 1) 128 in
    let e-masked-s = eq-u16 (ws-parse-frame-dma ps-cli wire-buf 0 6).wfr-close-code 1002 in
    let _ = raw-store-u8 wire-buf 9 in
    let _ = raw-store-u8 (wrapping-add-i64 wire-buf 1) 128 in
    let e-fping = eq-u16 (ws-parse-frame-dma ps-srv wire-buf 0 6).wfr-close-code 1002 in
    let _ = raw-store-u8 wire-buf 8 in
    let e-fclose = eq-u16 (ws-parse-frame-dma ps-srv wire-buf 0 6).wfr-close-code 1002 in
    let _ = raw-store-u8 wire-buf 137 in
    let _ = raw-store-u8 (wrapping-add-i64 wire-buf 1) 254 in
    let e-bigctrl = eq-u16 (ws-parse-frame-dma ps-srv wire-buf 0 8).wfr-close-code 1002 in
    let _ = raw-store-u8 wire-buf 131 in
    let _ = raw-store-u8 (wrapping-add-i64 wire-buf 1) 128 in
    let e-op3 = eq-u16 (ws-parse-frame-dma ps-srv wire-buf 0 6).wfr-close-code 1002 in
    let _ = raw-store-u8 wire-buf 135 in
    let e-op7 = eq-u16 (ws-parse-frame-dma ps-srv wire-buf 0 6).wfr-close-code 1002 in
    let _ = raw-store-u8 wire-buf 139 in
    let e-opb = eq-u16 (ws-parse-frame-dma ps-srv wire-buf 0 6).wfr-close-code 1002 in
    let _ = raw-store-u8 wire-buf 143 in
    let e-opf = eq-u16 (ws-parse-frame-dma ps-srv wire-buf 0 6).wfr-close-code 1002 in
    let _ = raw-store-u8 wire-buf 128 in
    let _ = raw-store-u8 (wrapping-add-i64 wire-buf 1) 128 in
    let e-stray-cont = eq-u16 (ws-parse-frame-dma ps-srv wire-buf 0 6).wfr-close-code 1002 in
    let ok-a = e-rsv1 && e-rsv2 && e-rsv3 && e-unmasked && e-masked-s && e-fping && e-fclose && e-bigctrl && e-op3 && e-op7 && e-opb && e-opf && e-stray-cont in
    if not ok-a then 15
    else test-part6-close-codes pbuf wire-buf

  test-part6-close-codes :: I64 -> I64 ->{MVar} Int
  test-part6-close-codes pbuf wire-buf =
    let ps-srv = ws-parser-init-server 4096 in
    let e-cl0 = check-close-code pbuf wire-buf 0 ws-parse-err-protocol 1002 in
    let e-cl999 = check-close-code pbuf wire-buf 999 ws-parse-err-protocol 1002 in
    let e-cl1004 = check-close-code pbuf wire-buf 1004 ws-parse-err-protocol 1002 in
    let e-cl1005 = check-close-code pbuf wire-buf 1005 ws-parse-err-protocol 1002 in
    let e-cl1006 = check-close-code pbuf wire-buf 1006 ws-parse-err-protocol 1002 in
    let e-cl1015 = check-close-code pbuf wire-buf 1015 ws-parse-err-protocol 1002 in
    let e-cl2999 = check-close-code pbuf wire-buf 2999 ws-parse-err-protocol 1002 in
    let e-cl5000 = check-close-code pbuf wire-buf 5000 ws-parse-err-protocol 1002 in
    let v-cl1000 = check-close-code pbuf wire-buf 1000 ws-parse-ok 1000 in
    let v-cl3000 = check-close-code pbuf wire-buf 3000 ws-parse-ok 3000 in
    let v-cl4999 = check-close-code pbuf wire-buf 4999 ws-parse-ok 4999 in
    let _ = raw-store-u8 pbuf 42 in
    let cl1-len = ws-frame-encode-masked-raw wire-buf 64 True ws-op-close 0x55555555 pbuf 1 in
    let e-cl1b = eq-u16 (ws-parse-frame-dma ps-srv wire-buf 0 cl1-len).wfr-close-code 1002 in
    let ok-b = (
      e-cl0 && e-cl999 && e-cl1004 && e-cl1005 && e-cl1006 && e-cl1015 && e-cl2999 && e-cl5000
        && v-cl1000 && v-cl3000 && v-cl4999 && e-cl1b) in
    if ok-b then 0 else 16

  -- Part 7: Zero-copy headroom encoder + WsHub O(1) sub/unsub & automatic Ping/Close echo
  test-part7-headroom-hub :: I64 -> I64 -> I64 ->{IO, MVar} Int
  test-part7-headroom-hub pbuf out-buf wire-buf =
    let ps-srv = ws-parser-init-server 4096 in
    let ps-cli = ws-parser-init-client 4096 in
    let hr-payload = wrapping-add-i64 out-buf 64 in
    let hr-len = copy-str-to-dma "headroom-zero-copy" hr-payload in
    let hr-hdr = ws-frame-encode-headroom-raw hr-payload hr-len True ws-op-text in
    let hr-start = wrapping-sub-i64 hr-payload (u32-to-i64 hr-hdr) in
    let hr-res = ws-parse-frame-dma ps-cli hr-start 0 (wrapping-add-u32 hr-hdr hr-len) in
    let ok-hr = (
      eq-u32 hr-hdr 2 && eq-u8 hr-res.wfr-status ws-parse-ok
        && dma-eq-str hr-start hr-res.wfr-payload-off hr-res.wfr-payload-len "headroom-zero-copy") in
    case ws-hub-new-standalone (core-id 0) 64 16 128 of
      None -> 17
      Some hub ->
        let c1 = ws-hub-connect hub (fd-of 10) 1 in
        let c2 = ws-hub-connect hub (fd-of 11) 2 in
        let c3 = ws-hub-connect hub (fd-of 12) 3 in
        let s1 = ws-hub-subscribe hub c1 5 in
        let s2 = ws-hub-subscribe hub c2 5 in
        let s3 = ws-hub-subscribe hub c3 5 in
        let un2 = ws-hub-unsubscribe hub s2 in
        let un2-dup = not (ws-hub-unsubscribe hub s2) in
        let ps-pub = ws-hub-publish hub 5 ws-op-text (wrapping-i64-to-u64 hr-payload) hr-len 77 0 in
        let st1 = ws-hub-conn-stats hub c1 in
        let st2 = ws-hub-conn-stats hub c2 in
        let st3 = ws-hub-conn-stats hub c3 in
        let ping-len = copy-str-to-dma "hi-ping" pbuf in
        let ping-wire = ws-frame-encode-masked-raw wire-buf 128 True ws-op-ping 0x33445566 pbuf ping-len in
        let rfp = ws-parse-frame-dma ps-srv wire-buf 0 ping-wire in
        let act-pong = ws-hub-handle-control-dma hub c1 rfp wire-buf out-buf 256 in
        let pong-parsed = ws-parse-frame-dma ps-cli out-buf 0 act-pong.ctrl-tx-len in
        let pong-payload-ok = dma-eq-str out-buf pong-parsed.wfr-payload-off pong-parsed.wfr-payload-len "hi-ping" in
        let cl-out = wrapping-add-i64 out-buf 256 in
        let cl-ok-n = ws-frame-encode-masked-raw wire-buf 64 True ws-op-close 0x12345678 pbuf 0 in
        let cl-parsed = ws-parse-frame-dma ps-srv wire-buf 0 cl-ok-n in
        let act-cl = ws-hub-handle-control-dma hub c1 cl-parsed wire-buf cl-out 256 in
        let cl-echo-parsed = ws-parse-frame-dma ps-cli cl-out 0 act-cl.ctrl-tx-len in
        let dc1 = ws-hub-disconnect hub c1 in
        let dc3 = ws-hub-disconnect hub c3 in
        let rem-subs = ws-hub-topic-sub-count hub 5 in
        let _dh = ws-hub-destroy hub in
        let ok-hub = (
          ok-hr && un2 && un2-dup
            && eq-u32 ps-pub.pub-local-delivered 2
            && eq-u64 st1.cs-frames-sent 1 && eq-u64 st2.cs-frames-sent 0 && eq-u64 st3.cs-frames-sent 1
            && eq-u8 act-pong.ctrl-kind ws-ctrl-send-pong
            && eq-u8 pong-parsed.wfr-status ws-parse-ok && eq-u8 pong-parsed.wfr-opcode ws-op-pong
            && pong-payload-ok
            && eq-u8 act-cl.ctrl-kind ws-ctrl-send-close && eq-u16 act-cl.ctrl-close-code 1000
            && eq-u8 cl-echo-parsed.wfr-status ws-parse-ok && eq-u8 cl-echo-parsed.wfr-opcode ws-op-close
            && eq-u16 cl-echo-parsed.wfr-close-code 1000
            && dc1 && dc3 && eq-u32 rem-subs 0) in
        if ok-hub then 0 else 18

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 393216 4096 in
    if eq-u64 dma 0 then 99
    else
      let base = wrapping-u64-to-i64 dma in
      let key-buf = base in
      let out-buf = wrapping-add-i64 base 4096 in
      let pbuf = wrapping-add-i64 base 8192 in
      let src-buf = wrapping-add-i64 base 16384 in
      let wire-buf = wrapping-add-i64 base 163840 in
      let r12 = test-part1-and2 key-buf out-buf pbuf in
      if r12 /= 0 then let _ = engine-dma-free dma 393216 in r12
      else
        let r3 = test-part3-lengths src-buf wire-buf in
        if r3 /= 0 then let _ = engine-dma-free dma 393216 in r3
        else
          case test-part4-fragments pbuf wire-buf of
            (r4, _) ->
              if r4 /= 0 then let _ = engine-dma-free dma 393216 in r4
              else
                let r5 = test-part5-utf8 pbuf wire-buf in
                if r5 /= 0 then let _ = engine-dma-free dma 393216 in r5
                else
                  let r6 = test-part6-proto pbuf wire-buf in
                  if r6 /= 0 then let _ = engine-dma-free dma 393216 in r6
                  else
                    let r7 = test-part7-headroom-hub pbuf out-buf wire-buf in
                    let _ = engine-dma-free dma 393216 in
                    r7
"#;
    build_and_run_leak_checked("rfc6455-autobahn", src, &[], 0);
}

// =========================================================================
// 3. Multi-Core 4-Pinned-Worker SPSC Pub/Sub Broadcast Test
//    (4,096 virtual WebSocket subscribers across 4 OS cores, 100% delivery,
//     sub-5ms broadcast latency, 0 leaks, 0 hot-loop allocs)
// =========================================================================

#[test]
fn multicore_4_pinned_workers_spsc_pubsub_broadcast() {
    let bridge = c_fixture(
        "ws_hub_worker_bridge",
        r#"
#include <stdint.h>
#include <stdio.h>
#include <time.h>

extern int64_t ws_pubsub_worker_entry(int64_t arg);

typedef struct {
    uint64_t header;
    int64_t code;
    int64_t arity;
    int64_t nfilled;
} CinchStaticClosure;

/* KIND_CLOSURE = 2, bits 35..40 (`block_kinds.rs`: RC_BITS = 35); rc = 0 (RC_IMMORTAL). */
#define IMMORTAL_CLOSURE_HEADER ((uint64_t)2ULL << 35)

static int64_t stub_ws_worker(int64_t self, int64_t argc, int64_t *argv) {
    (void)self;
    (void)argc;
    return ws_pubsub_worker_entry(argv[0]);
}

static CinchStaticClosure clo_ws_worker;

int64_t ws_get_worker_closure(void) {
    clo_ws_worker.header = IMMORTAL_CLOSURE_HEADER;
    clo_ws_worker.code = (int64_t)(uintptr_t)stub_ws_worker;
    clo_ws_worker.arity = 1;
    clo_ws_worker.nfilled = 0;
    return (int64_t)(uintptr_t)&clo_ws_worker.code;
}

uint64_t ws_mono_ns(void) {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return (uint64_t)ts.tv_sec * 1000000000ULL + (uint64_t)ts.tv_nsec;
}

int64_t ws_print_pubsub_summary(uint64_t deliveries, uint64_t subscribers, uint64_t envelopes, uint64_t elapsed_us) {
    printf("PUBSUB_OK deliveries=%llu subscribers=%llu envelopes=%llu elapsed_us=%llu\n",
           (unsigned long long)deliveries,
           (unsigned long long)subscribers,
           (unsigned long long)envelopes,
           (unsigned long long)elapsed_us);
    fflush(stdout);
    return 0;
}
"#,
    );

    let src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Data.Option
  import Engine.Ring.Spsc
  import Engine.Types
  import Net.Http.WebSocket.Frame
  import Net.Http.WebSocket.Hub
  import Prim.Atomic

  foreign import ccall "ws_get_worker_closure" getWorkerClo :: Unit ->{IO} I64
  foreign import ccall "ws_mono_ns" monoNs :: Unit ->{IO} U64
  foreign import ccall "ws_print_pubsub_summary" printSummary :: U64 -> U64 -> U64 -> U64 ->{IO} I64

  -- Shared 4 KiB DMA control slab layout:
  --   +0   : mesh-base   :: I64
  --   +8   : mesh-cap    :: I64
  --   +16  : mesh-stride :: I64
  --   +24  : payload-addr:: U64  (shared read-only pre-framed WebSocket payload)
  --   +32  : payload-len :: U32
  --   +128 : atomic ready-count   :: I64  (barrier: all 4 workers initialized 1024 subs)
  --   +256 : atomic go-flag       :: I64  (coordinator releases all 4 cores simultaneously)
  --   +384 : atomic done-count    :: I64
  --   +512 : WorkerOut[4], each 128B:
  --            +0  : local-deliveries  :: U64
  --            +8  : remote-deliveries :: U64
  --            +16 : remote-envelopes  :: U64
  --            +24 : verified-conns    :: U64
  --            +32 : max-latency-ns    :: U64
  --            +40 : hot-allocs        :: U64
  --            +48 : hot-atomic-rcs    :: U64
  --            +56 : error-code        :: I64
  --   +1024: shared payload bytes (64B)

  -- Topology:
  --   4 pinned OS worker cores
  --   1,024 virtual WebSocket subscribers per core = 4,096 total subscribers
  --   16 topics (64 subscribers per topic per core = 256 subscribers per topic cluster-wide)
  --   Each core publishes 16 broadcasts (1 per topic) = 64 broadcasts cluster-wide
  --   Expected per core: 16 local broadcasts * 64 + 48 remote broadcasts * 64 = 4,096 deliveries/core
  --   Expected cluster-wide: 16,384 deliveries across 4,096 subscribers, 0 drops!

  conns-per-core :: U32
  conns-per-core = 1024

  topic-count :: U32
  topic-count = 16

  pubs-per-core :: U32
  pubs-per-core = 16

  register-subs-loop :: WsHub -> I64 -> U32 -> U32 ->{MVar} Bool
  register-subs-loop hub handles-buf i n =
    if ge-u32 i n then True
    else
      let c = ws-hub-connect hub (fd-of (wrapping-u32-to-i32 (wrapping-add-u32 i 100))) (u32-to-u64 i) in
      if gen-handle-is-null c then False
      else
        let topic = bit-and-u32 i 15 in
        let s = ws-hub-subscribe hub c topic in
        if gen-handle-is-null s then False
        else
          let _st = raw-store-u64 (wrapping-add-i64 handles-buf (wrapping-shl-i64 (u32-to-i64 i) 3)) c.gen-handle-raw in
          register-subs-loop hub handles-buf (wrapping-add-u32 i 1) n

  verify-conns-loop :: WsHub -> I64 -> U32 -> U32 -> U64 -> U32 -> U64 ->{MVar} U64
  verify-conns-loop hub handles-buf i n expect-frames expect-bytes acc =
    if ge-u32 i n then acc
    else
      let h = GenHandle (raw-load-u64 (wrapping-add-i64 handles-buf (wrapping-shl-i64 (u32-to-i64 i) 3))) in
      let st = ws-hub-conn-stats hub h in
      let ok = eq-u64 st.cs-frames-sent expect-frames && eq-u64 st.cs-bytes-sent (wrapping-mul-u64 expect-frames (u32-to-u64 expect-bytes)) in
      let acc1 = if ok then wrapping-add-u64 acc 1 else acc in
      verify-conns-loop hub handles-buf (wrapping-add-u32 i 1) n expect-frames expect-bytes acc1

  wait-go :: I64 ->{MVar} Unit
  wait-go ctrl =
    if ne-i64 (atomic-load-acquire-i64 (wrapping-add-i64 ctrl 256)) 0 then ()
    else
      let _p = cpu-pause () in
      wait-go ctrl

  publish-all-topics :: WsHub -> I64 -> U32 -> U32 -> U64 -> U32 -> U64 -> U64 -> U64 ->{MVar} Unit
  publish-all-topics hub out-slot t total payload-addr payload-len ts-ns acc-loc acc-drop =
    if ge-u32 t total then
      let _0 = raw-store-u64 out-slot acc-loc in
      raw-store-u64 (wrapping-add-i64 out-slot 64) acc-drop
    else
      let ps = ws-hub-publish hub t ws-op-text payload-addr payload-len (u32-to-u64 t) ts-ns in
      publish-all-topics hub out-slot (wrapping-add-u32 t 1) total payload-addr payload-len ts-ns
        (wrapping-add-u64 acc-loc (u32-to-u64 ps.pub-local-delivered))
        (wrapping-add-u64 acc-drop (u32-to-u64 ps.pub-remote-dropped))

  drain-until-target :: WsHub -> I64 -> U32 -> U32 -> U64 -> U64 -> U64 ->{IO, MVar} Unit
  drain-until-target hub out-slot target-envs spins got-envs got-del max-lat =
    if ge-u64 got-envs (u32-to-u64 target-envs) || eq-u32 spins 0 then
      let _0 = raw-store-u64 (wrapping-add-i64 out-slot 8) got-del in
      let _1 = raw-store-u64 (wrapping-add-i64 out-slot 16) got-envs in
      raw-store-u64 (wrapping-add-i64 out-slot 32) max-lat
    else
      let now = monoNs () in
      let ds = ws-hub-drain-remote hub 64 now in
      if gt-u32 ds.drain-envelopes 0 then
        let mlat = if gt-u64 ds.drain-max-latency-ns max-lat then ds.drain-max-latency-ns else max-lat in
        drain-until-target hub out-slot target-envs spins
          (wrapping-add-u64 got-envs (u32-to-u64 ds.drain-envelopes))
          (wrapping-add-u64 got-del (u32-to-u64 ds.drain-delivered))
          mlat
      else
        let _p = cpu-pause () in
        drain-until-target hub out-slot target-envs (wrapping-sub-u32 spins 1) got-envs got-del max-lat

  worker-entry :: I64 ->{IO, MVar} I64
  worker-entry arg-slot =
    let ctrl = raw-load-i64 arg-slot in
    let cid-u32 = raw-load-u32 (wrapping-add-i64 arg-slot 8) in
    let cid = core-id (wrapping-u32-to-u16 cid-u32) in
    let out-slot = wrapping-add-i64 (wrapping-add-i64 ctrl 512) (wrapping-shl-i64 (u32-to-i64 cid-u32) 7) in
    let mesh-base = raw-load-i64 ctrl in
    let mesh-cap = raw-load-i64 (wrapping-add-i64 ctrl 8) in
    let mesh-stride = raw-load-i64 (wrapping-add-i64 ctrl 16) in
    let payload-addr = raw-load-u64 (wrapping-add-i64 ctrl 24) in
    let payload-len = raw-load-u32 (wrapping-add-i64 ctrl 32) in
    case ws-hub-new-from-mesh-raw cid 4 mesh-base mesh-cap mesh-stride conns-per-core topic-count conns-per-core of
      None ->
        let _ = raw-store-i64 (wrapping-add-i64 out-slot 56) (-1) in
        let _ = atomic-fetch-add-acqrel-i64 (wrapping-add-i64 ctrl 384) 1 in
        -1
      Some hub ->
        let hbuf-u = engine-dma-alloc 16384 4096 in
        if eq-u64 hbuf-u 0 then
          let _ = ws-hub-destroy hub in
          let _ = raw-store-i64 (wrapping-add-i64 out-slot 56) (-2) in
          let _ = atomic-fetch-add-acqrel-i64 (wrapping-add-i64 ctrl 384) 1 in
          -2
        else
          let hbuf = wrapping-u64-to-i64 hbuf-u in
          let reg-ok = register-subs-loop hub hbuf 0 conns-per-core in
          let _rdy = atomic-fetch-add-acqrel-i64 (wrapping-add-i64 ctrl 128) 1 in
          let _go = wait-go ctrl in
          let _rst = engine-thread-counters-reset () in
          let t-pub = monoNs () in
          let _pub = publish-all-topics hub out-slot 0 pubs-per-core payload-addr payload-len t-pub 0 0 in
          let _dr = drain-until-target hub out-slot 48 50000000 0 0 0 in
          let hot-allocs = engine-thread-alloc-count () in
          let hot-arcs = engine-thread-atomic-rc-count () in
          let drops = raw-load-u64 (wrapping-add-i64 out-slot 64) in
          let verified = verify-conns-loop hub hbuf 0 conns-per-core 4 payload-len 0 in
          let _3 = raw-store-u64 (wrapping-add-i64 out-slot 24) verified in
          let _5 = raw-store-u64 (wrapping-add-i64 out-slot 40) hot-allocs in
          let _6 = raw-store-u64 (wrapping-add-i64 out-slot 48) hot-arcs in
          let err = if not reg-ok then (-3) else if ne-u64 drops 0 then (-4) else if ne-u64 hot-allocs 0 then (-5) else if ne-u64 hot-arcs 0 then (-6) else 0 in
          let _7 = raw-store-i64 (wrapping-add-i64 out-slot 56) err in
          let _fh = engine-dma-free hbuf-u 16384 in
          let _dh = ws-hub-destroy hub in
          let _dn = atomic-fetch-add-acqrel-i64 (wrapping-add-i64 ctrl 384) 1 in
          err

  foreign export ccall "ws_pubsub_worker_entry" worker-entry :: I64 ->{IO, MVar} I64

  wait-counter :: I64 -> I64 -> U32 ->{MVar} Bool
  wait-counter addr want spins =
    if ge-i64 (atomic-load-acquire-i64 addr) want then True
    else if eq-u32 spins 0 then False
    else
      let _p = cpu-pause () in
      wait-counter addr want (wrapping-sub-u32 spins 1)

  main :: Unit ->{IO, MVar} Int
  main _ =
    case core-mesh-new 4 256 of
      None -> 1
      Some mesh ->
        let ctrl-u = engine-dma-alloc 4096 4096 in
        if eq-u64 ctrl-u 0 then
          let _ = core-mesh-destroy mesh in 2
        else
          let ctrl = wrapping-u64-to-i64 ctrl-u in
          let p-addr = wrapping-add-i64 ctrl 1024 in
          -- Pre-fill 32-byte broadcast payload
          let _p0 = raw-store-u64 p-addr 0x57535F42524F4144 in
          let _p1 = raw-store-u64 (wrapping-add-i64 p-addr 8) 0x434153545F4D5347 in
          let _p2 = raw-store-u64 (wrapping-add-i64 p-addr 16) 0x3031323334353637 in
          let _p3 = raw-store-u64 (wrapping-add-i64 p-addr 24) 0x3839414243444546 in
          let _c0 = raw-store-i64 ctrl mesh.mesh-base in
          let _c1 = raw-store-i64 (wrapping-add-i64 ctrl 8) mesh.mesh-cap in
          let _c2 = raw-store-i64 (wrapping-add-i64 ctrl 16) mesh.mesh-ring-stride in
          let _c3 = raw-store-u64 (wrapping-add-i64 ctrl 24) (wrapping-i64-to-u64 p-addr) in
          let _c4 = raw-store-u32 (wrapping-add-i64 ctrl 32) 32 in

          -- Prepare 4 worker arg slots at ctrl + 2048 + i*16
          let a0 = wrapping-add-i64 ctrl 2048 in
          let a1 = wrapping-add-i64 ctrl 2064 in
          let a2 = wrapping-add-i64 ctrl 2080 in
          let a3 = wrapping-add-i64 ctrl 2096 in
          let _ = raw-store-i64 a0 ctrl in let _ = raw-store-u32 (wrapping-add-i64 a0 8) 0 in
          let _ = raw-store-i64 a1 ctrl in let _ = raw-store-u32 (wrapping-add-i64 a1 8) 1 in
          let _ = raw-store-i64 a2 ctrl in let _ = raw-store-u32 (wrapping-add-i64 a2 8) 2 in
          let _ = raw-store-i64 a3 ctrl in let _ = raw-store-u32 (wrapping-add-i64 a3 8) 3 in

          let clo = wrapping-i64-to-u64 (getWorkerClo ()) in
          let w0 = engine-spawn-pinned (core-id 0) clo a0 in
          let w1 = engine-spawn-pinned (core-id 1) clo a1 in
          let w2 = engine-spawn-pinned (core-id 2) clo a2 in
          let w3 = engine-spawn-pinned (core-id 3) clo a3 in
          if lt-i64 w0 0 || lt-i64 w1 0 || lt-i64 w2 0 || lt-i64 w3 0 then
            let _ = engine-dma-free ctrl-u 4096 in
            let _ = core-mesh-destroy mesh in 3
          else
            let rdy = wait-counter (wrapping-add-i64 ctrl 128) 4 50000000 in
            let t0 = monoNs () in
            let _go = atomic-store-release-i64 (wrapping-add-i64 ctrl 256) 1 in
            let done-ok = wait-counter (wrapping-add-i64 ctrl 384) 4 50000000 in
            let t1 = monoNs () in
            let elapsed-ns = saturating-sub-u64 t1 t0 in
            let r0 = engine-join w0 in
            let r1 = engine-join w1 in
            let r2 = engine-join w2 in
            let r3 = engine-join w3 in

            -- Inspect per-core delivery, verification, and latency stats
            let o0 = wrapping-add-i64 ctrl 512 in
            let o1 = wrapping-add-i64 ctrl 640 in
            let o2 = wrapping-add-i64 ctrl 768 in
            let o3 = wrapping-add-i64 ctrl 896 in
            let tot-loc = wrapping-add-u64 (wrapping-add-u64 (raw-load-u64 o0) (raw-load-u64 o1))
                                           (wrapping-add-u64 (raw-load-u64 o2) (raw-load-u64 o3)) in
            let tot-rem = wrapping-add-u64 (wrapping-add-u64 (raw-load-u64 (wrapping-add-i64 o0 8)) (raw-load-u64 (wrapping-add-i64 o1 8)))
                                           (wrapping-add-u64 (raw-load-u64 (wrapping-add-i64 o2 8)) (raw-load-u64 (wrapping-add-i64 o3 8))) in
            let tot-env = wrapping-add-u64 (wrapping-add-u64 (raw-load-u64 (wrapping-add-i64 o0 16)) (raw-load-u64 (wrapping-add-i64 o1 16)))
                                           (wrapping-add-u64 (raw-load-u64 (wrapping-add-i64 o2 16)) (raw-load-u64 (wrapping-add-i64 o3 16))) in
            let tot-ver = wrapping-add-u64 (wrapping-add-u64 (raw-load-u64 (wrapping-add-i64 o0 24)) (raw-load-u64 (wrapping-add-i64 o1 24)))
                                           (wrapping-add-u64 (raw-load-u64 (wrapping-add-i64 o2 24)) (raw-load-u64 (wrapping-add-i64 o3 24))) in
            let tot-del = wrapping-add-u64 tot-loc tot-rem in
            let err0 = raw-load-i64 (wrapping-add-i64 o0 56) in
            let err1 = raw-load-i64 (wrapping-add-i64 o1 56) in
            let err2 = raw-load-i64 (wrapping-add-i64 o2 56) in
            let err3 = raw-load-i64 (wrapping-add-i64 o3 56) in
            let elapsed-us = div-u64 elapsed-ns 1000 in
            let _pr = printSummary tot-del tot-ver tot-env elapsed-us in
            let _fc = engine-dma-free ctrl-u 4096 in
            let _fm = core-mesh-destroy mesh in
            if not (rdy && done-ok) then 10
            else if ne-i64 r0 0 || ne-i64 r1 0 || ne-i64 r2 0 || ne-i64 r3 0 then 11
            else if ne-i64 err0 0 || ne-i64 err1 0 || ne-i64 err2 0 || ne-i64 err3 0 then 12
            -- 4 cores * 16 local broadcasts * 64 subs = 4,096 local + 12,288 remote = 16,384 total deliveries
            else if ne-u64 tot-loc 4096 || ne-u64 tot-rem 12288 || ne-u64 tot-env 192 || ne-u64 tot-ver 4096 then 13
            -- Broadcast latency must be sub-5ms (< 5,000,000 ns)
            else if gt-u64 elapsed-ns 5000000 then 14
            else 0
"#;
    let (stdout, _stderr) =
        build_and_run_leak_checked("4core-spsc-pubsub", src, &[bridge], 0);
    assert!(
        stdout.contains("PUBSUB_OK deliveries=16384 subscribers=4096 envelopes=192"),
        "expected full 4-core broadcast summary in stdout, got:\n{stdout}"
    );
    eprintln!("[W4-http-ws 4-core PubSub] {}", stdout.trim());
}

// =========================================================================
// 4. Steady-State 50,000-Frame Parse + SIMD Unmask + UTF-8 + Encode Loop
//    asserting allocs == 0, atomic_rc == 0, and CINCH_LEAK_CHECK leaked=0
// =========================================================================

#[test]
fn steady_state_50000_frame_parse_unmask_utf8_encode_zero_alloc() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.String
  import Engine.Types
  import Net.Http.WebSocket.Frame
  import Net.Http.WebSocket.Hub
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

  -- 50,000-frame steady-state loop:
  --   1. Masked client frame encode (128-bit SIMD mask)
  --   2. Server RFC 6455 parse + in-place 128-bit SIMD unmask + strict UTF-8 validate
  --   3. Server headroom frame encode + raw frame encode
  --   4. WsHub local topic fanout
  -- Must execute with 0 heap allocations and 0 atomic RC ops!
  #[no-alloc]
  hot-loop50000 :: WsHub -> WsParserState -> I64 -> U32 -> I64 -> I64 -> U32 -> U32 ->{MVar} U32
  hot-loop50000 hub ps src-buf src-len rx-buf tx-buf i limit =
    if ge-u32 i limit then limit
    else
      let mask = wrapping-add-u32 0xA55A1234 i in
      let wlen = ws-frame-encode-masked-raw rx-buf 1024 (eq-u32 0 0) ws-op-text mask src-buf src-len in
      let fr = ws-parse-frame-dma ps rx-buf 0 wlen in
      if ne-u8 fr.wfr-status ws-parse-ok then 0
      else if ne-u32 fr.wfr-payload-len src-len then 0
      else
        let p-addr = wrapping-add-i64 rx-buf (u32-to-i64 fr.wfr-payload-off) in
        let hr = ws-frame-encode-headroom-raw p-addr fr.wfr-payload-len (eq-u32 0 0) ws-op-text in
        let enc = ws-frame-encode-raw tx-buf 1024 (eq-u32 0 0) ws-op-text src-buf src-len in
        let del = ws-hub-fanout-local hub 0 ws-op-text (wrapping-i64-to-u64 p-addr) fr.wfr-payload-len (u32-to-u64 i) in
        if ne-u32 hr 2 then 0
        else if ne-u32 enc (wrapping-add-u32 2 src-len) then 0
        else if ne-u32 del 1 then 0
        else hot-loop50000 hub ps src-buf src-len rx-buf tx-buf (wrapping-add-u32 i 1) limit

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 16384 4096 in
    if eq-u64 dma 0 then 1
    else
      let base = wrapping-u64-to-i64 dma in
      let src-buf = base in
      -- Leave 16B headroom before rx-buf so ws-frame-encode-headroom-raw can prepend header in place
      let rx-buf = wrapping-add-i64 base 4112 in
      let tx-buf = wrapping-add-i64 base 8192 in
      let src-len = copy-str-to-dma "Cinch zero-copy WebSocket frame payload: café € 🚀 [50000 steady-state]" src-buf in
      case ws-hub-new-standalone (core-id 0) 16 4 16 of
        None ->
          let _ = engine-dma-free dma 16384 in 2
        Some hub ->
          let conn = ws-hub-connect hub (fd-of 42) 0 in
          let _sub = ws-hub-subscribe hub conn 0 in
          let ps = ws-parser-init-server ws-default-max-frame-size in
          let _rst = engine-thread-counters-reset () in
          let done = hot-loop50000 hub ps src-buf src-len rx-buf tx-buf 0 50000 in
          let allocs = engine-thread-alloc-count () in
          let atomic-rcs = engine-thread-atomic-rc-count () in
          let st = ws-hub-conn-stats hub conn in
          let _dh = ws-hub-destroy hub in
          let _fd = engine-dma-free dma 16384 in
          if ne-u32 done 50000 then 10
          else if ne-u64 st.cs-frames-sent 50000 then 11
          else if ne-u64 allocs 0 then 12
          else if ne-u64 atomic-rcs 0 then 13
          else 0
"#;
    build_and_run_leak_checked("steady-50000-zero-alloc", src, &[], 0);
}
