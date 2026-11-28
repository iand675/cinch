//! Integration, conformance, end-to-end 4-mode streaming RPC (Unary,
//! Server-Streaming, Client-Streaming, Bidirectional-Streaming) + Trailers-Only
//! over real `Net.Http.V2.Conn` wire frames, and 50,000-cycle zero-allocation /
//! zero-atomic-RC / leak-checked tests for `Net.Http.Grpc`.
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
        "cinch-net-http-grpc-{tag}-{}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn build_and_run_leak_checked(tag: &str, src: &str, expected_code: i32) -> (String, String) {
    let dir = fresh_dir(tag);
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let exe = dir.join(format!("grpc-{tag}"));
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
// 1. Manifest registration, POD ABI layout, and #[no-alloc] safety check
// =========================================================================

#[test]
fn net_http_grpc_manifest_registration_pod_abi_and_no_alloc_check() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    let entry = pkg
        .modules
        .iter()
        .find(|m| m.name == "Net.Http.Grpc")
        .expect("Net.Http.Grpc missing from seed/prelude/.cinchpkg modules");
    assert_eq!(entry.path, "Net/Http/Grpc.cinch");
    assert!(
        pkg.order.iter().any(|m| m == "Net.Http.Grpc"),
        "Net.Http.Grpc missing from seed/prelude/.cinchpkg order"
    );

    let pos_dma = pkg.order.iter().position(|n| n == "Engine.Mem.DmaPool").unwrap();
    let pos_types = pkg.order.iter().position(|n| n == "Net.Http.Types").unwrap();
    let pos_hpack = pkg.order.iter().position(|n| n == "Net.Http.Hpack.Codec").unwrap();
    let pos_h2_conn = pkg.order.iter().position(|n| n == "Net.Http.V2.Conn").unwrap();
    let pos_grpc = pkg.order.iter().position(|n| n == "Net.Http.Grpc").unwrap();
    assert!(pos_dma < pos_grpc);
    assert!(pos_types < pos_grpc);
    assert!(pos_hpack < pos_grpc);
    assert!(pos_h2_conn < pos_grpc);

    // Compile-time #[no-alloc] audit across all hot-path Net.Http.Grpc entry points
    let dir = fresh_dir("no-alloc-audit");
    std::fs::write(
        dir.join("Main.cinch"),
        r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Engine.Types
  import Engine.Mem.DmaPool
  import Net.Http.V2.Conn
  import Net.Http.Grpc

  #[no-alloc]
  audit-grpc-hot-path :: H2Conn -> DmaSlice -> I64 -> I64 -> I64 ->{MVar} U32
  audit-grpc-hot-path conn sl rx tx scratch =
    let _sc = grpc-status-code-from-http 503 in
    let _rs = grpc-status-from-rst-error 8 in
    let fw = grpc-frame-encode-raw tx 0 256 0 rx 16 in
    let _hr = grpc-frame-encode-headroom-raw (wrapping-add-i64 tx 16) 0 8 in
    let _hs = grpc-frame-encode-headroom sl 0 8 in
    let fd = grpc-frame-decode-raw tx 0 256 grpc-default-max-message-size in
    let asm = grpc-assembler-init scratch 1024 grpc-default-max-message-size in
    let fi = grpc-frame-decode-incremental asm tx 0 21 in
    let _to = grpc-parse-timeout-dma rx 4 in
    let _ft = grpc-format-timeout-dma 5000000000 tx 0 64 in
    let _me = grpc-message-encode-dma rx 8 tx 0 64 in
    let _md = grpc-message-decode-dma rx 8 tx 0 64 in
    let _ct = grpc-validate-content-type-dma rx 16 in
    let _ps = grpc-parse-path-dma rx 20 in
    let _rq = grpc-validate-request-headers rx tx 4 in
    let _rp = grpc-parse-response-headers rx tx 4 in
    let _w1 = grpc-send-request-headers conn 1 rx 12 rx 9 5000000000 tx 0 256 in
    let _w2 = grpc-send-response-headers conn 1 tx 0 256 in
    let _w3 = grpc-send-message conn 1 0 rx 8 0 tx 0 256 in
    let _w4 = grpc-send-message-slice conn 1 0 sl 8 0 in
    let _w5 = grpc-send-end-stream conn 1 tx 0 256 in
    let _w6 = grpc-send-trailers conn 1 grpc-code-ok rx 0 tx 0 256 in
    let _w7 = grpc-send-trailers-only conn 1 grpc-code-unimplemented rx 4 tx 0 256 in
    let cf = grpc-client-feed conn rx 0 64 tx 0 256 in
    let c0 = grpc-server-accept-call conn 1 grpc-mode-unary grpc-default-max-message-size in
    let c1 = grpc-server-on-req-message c0 in
    let c2 = grpc-server-on-req-end c1 in
    let c3 = grpc-server-send-headers conn c2 tx 0 256 in
    let c4 = grpc-server-send-message conn c3 0 rx 8 tx 0 256 in
    let c5 = grpc-server-finish conn c4 grpc-code-ok rx 0 tx 0 256 in
    let cl0 = grpc-client-start-call conn 3 grpc-mode-bidi-streaming grpc-default-max-message-size rx 12 rx 9 0 tx 0 256 in
    let cl1 = grpc-client-send-message conn cl0 0 rx 8 0 tx 0 256 in
    let cl2 = grpc-client-half-close conn cl1 tx 0 256 in
    let cl3 = grpc-client-apply-feed conn cl2 cf in
    let cl4 = grpc-client-on-resp-message cl3 in
    wrapping-add-u32 fw.gfw-bytes-written
      (wrapping-add-u32 fd.gfd-consumed
        (wrapping-add-u32 fi.gfd-consumed (wrapping-add-u32 c5.gcs-last-tx-pos cl4.gcs-last-tx-pos)))

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
    .expect("Net.Http.Grpc #[no-alloc] audit must check cleanly");
    let _ = std::fs::remove_dir_all(&dir);
}

// =========================================================================
// 2. Framing, status codes, grpc-timeout (all 6 units + overflow/invalid),
//    grpc-message percent-encoding/decoding, content-type & path validation
// =========================================================================

#[test]
fn net_http_grpc_framing_timeout_percent_codec_and_validation_conformance() {
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
  import Net.Http.V2.Frame
  import Net.Http.Grpc
  import Prim.Atomic

  copy-str :: String -> I64 ->{MVar} U32
  copy-str s dst =
    let b = string-to-bytes s in
    let n = bytes-len b in
    let _ = copy-bytes b 0 dst n in
    wrapping-i64-to-u32 (nat-to-i64 n)

  copy-bytes :: Bytes -> Nat -> I64 -> Nat ->{MVar} Unit
  copy-bytes b i dst n =
    if i >= n then ()
    else if i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 dst (nat-to-i64 i)) (bytes-get-u8 b i) in
      copy-bytes b (i + 1) dst n
    else ()

  dma-eq-str :: I64 -> U32 -> String ->{MVar} Bool
  dma-eq-str addr len expected =
    let b = string-to-bytes expected in
    let n = wrapping-i64-to-u32 (nat-to-i64 (bytes-len b)) in
    if ne-u32 len n then False
    else dma-eq-bytes-loop addr b 0 (bytes-len b)

  dma-eq-bytes-loop :: I64 -> Bytes -> Nat -> Nat ->{MVar} Bool
  dma-eq-bytes-loop addr b i n =
    if i >= n then True
    else if i < bytes-len b then
      if ne-u8 (raw-load-u8 (wrapping-add-i64 addr (nat-to-i64 i))) (bytes-get-u8 b i) then False
      else dma-eq-bytes-loop addr b (i + 1) n
    else False

  check-all-statuses :: U8 -> Bool
  check-all-statuses code =
    if gt-u8 code 16 then True
    else
      let st = grpc-status-from-u8 code in
      if ne-u8 (grpc-status-to-u8 st) code then False
      else check-all-statuses (wrapping-add-u8 code 1)

  parse-to-str :: I64 -> String ->{MVar} GrpcTimeoutResult
  parse-to-str buf s =
    let n = copy-str s buf in
    grpc-parse-timeout-dma buf n

  check-ct :: I64 -> String -> U8 ->{MVar} Bool
  check-ct buf s exp =
    let n = copy-str s buf in
    eq-u8 (grpc-validate-content-type-dma buf n) exp

  main :: Unit ->{IO, MVar} Int
  main _ =
    -- === A. All 17 canonical status codes + HTTP-to-gRPC mapping ===
    let ok-statuses = (
      check-all-statuses 0
        && grpc-status-eq (grpc-status-from-u8 99) GrpcUnknown
        && grpc-status-name GrpcOk == "OK"
        && grpc-status-name GrpcDeadlineExceeded == "DEADLINE_EXCEEDED"
        && grpc-status-name GrpcResourceExhausted == "RESOURCE_EXHAUSTED"
        && grpc-status-name GrpcUnimplemented == "UNIMPLEMENTED"
        && grpc-status-name GrpcUnauthenticated == "UNAUTHENTICATED"
        && grpc-status-eq (grpc-status-from-http 400) GrpcInternal
        && grpc-status-eq (grpc-status-from-http 401) GrpcUnauthenticated
        && grpc-status-eq (grpc-status-from-http 403) GrpcPermissionDenied
        && grpc-status-eq (grpc-status-from-http 404) GrpcUnimplemented
        && grpc-status-eq (grpc-status-from-http 429) GrpcUnavailable
        && grpc-status-eq (grpc-status-from-http 502) GrpcUnavailable
        && grpc-status-eq (grpc-status-from-http 503) GrpcUnavailable
        && grpc-status-eq (grpc-status-from-http 504) GrpcUnavailable
        && grpc-status-eq (grpc-status-from-http 418) GrpcUnknown) in

    let dma = engine-dma-alloc 32768 4096 in
    if eq-u64 dma 0 then 99
    else
      let buf = wrapping-u64-to-i64 dma in
      let out = wrapping-add-i64 buf 4096 in
      let scratch = wrapping-add-i64 out 4096 in
      let dec-out = wrapping-add-i64 scratch 4096 in

      -- === B. 5-byte Framing: headroom zero-copy, multi-chunk split, ResourceExhausted, invalid comp ===
      let ok-headroom = (
        case dma-pool-new 4096 4 of
          None -> False
          Some pool ->
            case dma-pool-acquire pool of
              None -> let _ = dma-pool-destroy pool in False
              Some h ->
                let (full-sl, h1) = dma-buf-as-slice h in
                let res = (
                  case dma-slice-reserve-headroom full-sl grpc-h2-headroom-len of
                    None -> False
                    Some (_, payload-sl) ->
                      let plen = copy-str "hello-grpc-zc" payload-sl.slice-addr in
                      let gsl = grpc-frame-encode-headroom payload-sl 0 plen in
                      let h2sl = h2-frame-write-headroom gsl h2-frame-data h2-flag-none 1 gsl.slice-len in
                      let h2hdr = h2-frame-read-header-dma h2sl.slice-addr 0 in
                      let gdec = grpc-frame-decode-raw h2sl.slice-addr 9 h2sl.slice-len 1024 in
                      eq-u32 gsl.slice-off 9
                        && eq-u32 gsl.slice-len (wrapping-add-u32 plen 5)
                        && eq-u32 h2sl.slice-off 0
                        && eq-u32 h2sl.slice-len (wrapping-add-u32 plen 14)
                        && eq-u32 h2hdr.fh-length (wrapping-add-u32 plen 5)
                        && eq-u8 gdec.gfd-state grpc-decode-ok
                        && eq-u8 gdec.gfd-compressed 0
                        && eq-u32 gdec.gfd-msg-len plen
                        && dma-eq-str gdec.gfd-msg-addr gdec.gfd-msg-len "hello-grpc-zc") in
                let _ = dma-pool-release pool h1 in
                let _ = dma-pool-destroy pool in
                res) in

      -- Encode 17-byte payload, then split across 4 arbitrary HTTP/2 DATA chunks:
      -- chunk 1 = 2 bytes (mid-header!), chunk 2 = 3 bytes (rest of header!),
      -- chunk 3 = 7 bytes (partial body), chunk 4 = 10 bytes (rest of body)!
      let mlen = copy-str "split-across-data" buf in
      let fw = grpc-frame-encode-raw out 0 256 1 buf mlen in
      let asm = grpc-assembler-init scratch 4096 1024 in
      let r1 = grpc-frame-decode-incremental asm out 0 2 in
      let r2 = grpc-frame-decode-incremental asm out 2 5 in
      let r3 = grpc-frame-decode-incremental asm out 5 12 in
      let r4 = grpc-frame-decode-incremental asm out 12 22 in
      -- Also verify empty 0-byte gRPC message split across 2+3 bytes!
      let fw0 = grpc-frame-encode-raw out 0 256 0 buf 0 in
      let z1 = grpc-frame-decode-incremental asm out 0 2 in
      let z2 = grpc-frame-decode-incremental asm out 2 5 in
      -- Oversized message (> max-msg-size = 10) -> GrpcResourceExhausted (8) immediately on header!
      let _ = raw-store-u8 out 0 in
      let _ = h2-write-be32 (wrapping-add-i64 out 1) 11 in
      let r-big = grpc-frame-decode-raw out 0 5 10 in
      -- Invalid compression flag = 2 -> GrpcInternal (13) immediately!
      let _ = raw-store-u8 out 0 in
      let _ = raw-store-u8 out 2 in
      let _ = h2-write-be32 (wrapping-add-i64 out 1) 4 in
      let r-comp = grpc-frame-decode-raw out 0 9 1024 in
      let ok-framing = (
        eq-u8 fw.gfw-status grpc-code-ok && eq-u32 fw.gfw-bytes-written 22
          && eq-u8 r1.gfd-state grpc-decode-need-more && eq-u32 r1.gfd-consumed 2
          && eq-u8 r2.gfd-state grpc-decode-need-more && eq-u32 r2.gfd-consumed 3
          && eq-u8 r3.gfd-state grpc-decode-need-more && eq-u32 r3.gfd-consumed 7
          && eq-u8 r4.gfd-state grpc-decode-ok && eq-u8 r4.gfd-compressed 1
          && eq-u32 r4.gfd-msg-len 17 && eq-u32 r4.gfd-consumed 10
          && dma-eq-str r4.gfd-msg-addr r4.gfd-msg-len "split-across-data"
          && eq-u32 fw0.gfw-bytes-written 5
          && eq-u8 z1.gfd-state grpc-decode-need-more
          && eq-u8 z2.gfd-state grpc-decode-ok && eq-u32 z2.gfd-msg-len 0
          && eq-u8 r-big.gfd-state grpc-decode-error && eq-u8 r-big.gfd-grpc-status grpc-code-resource-exhausted
          && eq-u8 r-comp.gfd-state grpc-decode-error && eq-u8 r-comp.gfd-grpc-status grpc-code-internal) in

      -- === C. All 6 grpc-timeout units (H/M/S/m/u/n) + formatter round-trip + overflow/invalid ===
      let th = parse-to-str buf "2H" in
      let tm = parse-to-str buf "15M" in
      let ts = parse-to-str buf "30S" in
      let tms = parse-to-str buf "250m" in
      let tus = parse-to-str buf "500u" in
      let tns = parse-to-str buf "42n" in
      let t-ovf = parse-to-str buf "99999999H" in
      let t-long = parse-to-str buf "123456789n" in
      let t-unit = parse-to-str buf "10x" in
      let t-neg = parse-to-str buf "-5S" in
      let t-empty = parse-to-str buf "S" in
      -- Formatter tests
      let fh = grpc-format-timeout-dma 7200000000000 out 0 64 in
      let ok-fh = dma-eq-str out fh "2H" in
      let fm = grpc-format-timeout-dma 900000000000 out 0 64 in
      let ok-fm = dma-eq-str out fm "15M" in
      let fs = grpc-format-timeout-dma 30000000000 out 0 64 in
      let ok-fs = dma-eq-str out fs "30S" in
      let fms = grpc-format-timeout-dma 250000000 out 0 64 in
      let ok-fms = dma-eq-str out fms "250m" in
      let fus = grpc-format-timeout-dma 500000 out 0 64 in
      let ok-fus = dma-eq-str out fus "500u" in
      let fns = grpc-format-timeout-dma 42 out 0 64 in
      let ok-fns = dma-eq-str out fns "42n" in
      let f0 = grpc-format-timeout-dma 0 out 0 64 in
      let ok-f0 = dma-eq-str out f0 "0n" in
      let ok-timeout = (
        eq-u8 th.gt-ok 1 && eq-u64 th.gt-nanos 7200000000000
          && eq-u8 tm.gt-ok 1 && eq-u64 tm.gt-nanos 900000000000
          && eq-u8 ts.gt-ok 1 && eq-u64 ts.gt-nanos 30000000000
          && eq-u8 tms.gt-ok 1 && eq-u64 tms.gt-nanos 250000000
          && eq-u8 tus.gt-ok 1 && eq-u64 tus.gt-nanos 500000
          && eq-u8 tns.gt-ok 1 && eq-u64 tns.gt-nanos 42
          && eq-u8 t-ovf.gt-ok 0 && eq-u8 t-ovf.gt-err grpc-timeout-err-overflow
          && eq-u8 t-long.gt-ok 0 && eq-u8 t-long.gt-err grpc-timeout-err-too-long
          && eq-u8 t-unit.gt-ok 0 && eq-u8 t-unit.gt-err grpc-timeout-err-bad-unit
          && eq-u8 t-neg.gt-ok 0 && eq-u8 t-neg.gt-err grpc-timeout-err-bad-digit
          && eq-u8 t-empty.gt-ok 0 && eq-u8 t-empty.gt-err grpc-timeout-err-short
          && ok-fh && ok-fm && ok-fs && ok-fms && ok-fus && ok-fns && ok-f0) in

      -- === D. grpc-message percent-encoding & decoding (ASCII, %, CR/LF, UTF-8 non-ASCII, tolerant invalid) ===
      -- Build raw message: "100% ok\n" + UTF-8 'é' (0xC3 0xA9)
      let p0 = copy-str "100% ok\n" buf in
      let _ = raw-store-u8 (wrapping-add-i64 buf (u32-to-i64 p0)) 0xC3 in
      let _ = raw-store-u8 (wrapping-add-i64 buf (u32-to-i64 (wrapping-add-u32 p0 1))) 0xA9 in
      let raw-mlen = wrapping-add-u32 p0 2 in
      let enc = grpc-message-encode-dma buf raw-mlen out 0 256 in
      let ok-enc-wire = dma-eq-str out enc.gc-written "100%25 ok%0A%C3%A9" in
      let dec = grpc-message-decode-dma out enc.gc-written dec-out 0 256 in
      -- Verify exact byte-for-byte round-trip including 0xC3 0xA9
      let ok-rt = (
        eq-u8 dec.gc-ok 1 && eq-u8 dec.gc-had-invalid 0
          && eq-u32 dec.gc-written raw-mlen
          && eq-u8 (raw-load-u8 (wrapping-add-i64 dec-out 8)) 0xC3
          && eq-u8 (raw-load-u8 (wrapping-add-i64 dec-out 9)) 0xA9) in
      -- Tolerant decode of broken "%ZZ%2" leaves verbatim and flags gc-had-invalid=1
      let blen = copy-str "a%ZZ%2" buf in
      let bdec = grpc-message-decode-dma buf blen dec-out 0 64 in
      let ok-msg = (
        eq-u8 enc.gc-ok 1 && ok-enc-wire && ok-rt
          && eq-u8 bdec.gc-ok 1 && eq-u8 bdec.gc-had-invalid 1
          && dma-eq-str dec-out bdec.gc-written "a%ZZ%2") in

      -- === E. Content-Type & /<package>.<Service>/<Method> path validation ===
      let ok-ct = (
        check-ct buf "application/grpc" grpc-ct-default
          && check-ct buf "application/grpc+proto" grpc-ct-proto
          && check-ct buf "application/grpc+json" grpc-ct-json
          && check-ct buf "application/grpc+proto; charset=utf-8" grpc-ct-proto
          && check-ct buf "application/grpc+thrift" grpc-ct-other
          && check-ct buf "application/grpc; foo=bar" grpc-ct-default
          && check-ct buf "application/grpc-web" grpc-ct-invalid
          && check-ct buf "application/json" grpc-ct-invalid) in

      let plen1 = copy-str "/acme.v1.Greeter/SayHello" buf in
      let ps1 = grpc-parse-path-dma buf plen1 in
      let ok-p1 = (
        eq-u8 ps1.gp-valid 1 && eq-u8 ps1.gp-has-package 1
          && dma-eq-str (wrapping-add-i64 buf (u32-to-i64 ps1.gp-full-service-off)) ps1.gp-full-service-len "acme.v1.Greeter"
          && dma-eq-str (wrapping-add-i64 buf (u32-to-i64 ps1.gp-package-off)) ps1.gp-package-len "acme.v1"
          && dma-eq-str (wrapping-add-i64 buf (u32-to-i64 ps1.gp-service-off)) ps1.gp-service-len "Greeter"
          && dma-eq-str (wrapping-add-i64 buf (u32-to-i64 ps1.gp-method-off)) ps1.gp-method-len "SayHello") in

      let plen2 = copy-str "/Echo/Ping" buf in
      let ps2 = grpc-parse-path-dma buf plen2 in
      let ok-p2 = (
        eq-u8 ps2.gp-valid 1 && eq-u8 ps2.gp-has-package 0
          && dma-eq-str (wrapping-add-i64 buf (u32-to-i64 ps2.gp-service-off)) ps2.gp-service-len "Echo"
          && dma-eq-str (wrapping-add-i64 buf (u32-to-i64 ps2.gp-method-off)) ps2.gp-method-len "Ping") in

      let pb1 = grpc-parse-path-dma buf (copy-str "no-leading-slash/M" buf) in
      let pb2 = grpc-parse-path-dma buf (copy-str "/a/b/c" buf) in
      let pb3 = grpc-parse-path-dma buf (copy-str "/Svc/" buf) in
      let pb4 = grpc-parse-path-dma buf (copy-str "//Method" buf) in
      let pb5 = grpc-parse-path-dma buf (copy-str "/.Svc/Method" buf) in
      let ok-path = (
        ok-p1 && ok-p2
          && eq-u8 pb1.gp-valid 0 && eq-u8 pb2.gp-valid 0
          && eq-u8 pb3.gp-valid 0 && eq-u8 pb4.gp-valid 0 && eq-u8 pb5.gp-valid 0) in

      let _ = engine-dma-free dma 32768 in
      if not ok-statuses then 10
      else if not ok-headroom then 11
      else if not ok-framing then 12
      else if not ok-timeout then 13
      else if not ok-msg then 14
      else if not ok-ct then 15
      else if not ok-path then 16
      else 0
"#;
    build_and_run_leak_checked("conformance", src, 0);
}

// =========================================================================
// 3. Full client-server ALL 4 RPC streaming modes (Unary, Server-Streaming,
//    Client-Streaming, Bidi-Streaming) + Trailers-Only immediate errors
//    (`Unimplemented`, `DeadlineExceeded`) over real `Net.Http.V2.Conn` wire frames!
// =========================================================================

#[test]
fn net_http_grpc_end_to_end_all_4_rpc_modes_and_trailers_only_over_h2_conn() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.String
  import Engine.Types
  import Net.Http.V2.Frame
  import Net.Http.V2.Stream
  import Net.Http.V2.Conn
  import Net.Http.Grpc
  import Prim.Atomic

  copy-str :: String -> I64 ->{MVar} U32
  copy-str s dst =
    let b = string-to-bytes s in
    let n = bytes-len b in
    let _ = copy-bytes b 0 dst n in
    wrapping-i64-to-u32 (nat-to-i64 n)

  copy-bytes :: Bytes -> Nat -> I64 -> Nat ->{MVar} Unit
  copy-bytes b i dst n =
    if i >= n then ()
    else if i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 dst (nat-to-i64 i)) (bytes-get-u8 b i) in
      copy-bytes b (i + 1) dst n
    else ()

  dma-eq-str :: I64 -> U32 -> String ->{MVar} Bool
  dma-eq-str addr len expected =
    let b = string-to-bytes expected in
    let n = wrapping-i64-to-u32 (nat-to-i64 (bytes-len b)) in
    if ne-u32 len n then False
    else dma-eq-bytes-loop addr b 0 (bytes-len b)

  dma-eq-bytes-loop :: I64 -> Bytes -> Nat -> Nat ->{MVar} Bool
  dma-eq-bytes-loop addr b i n =
    if i >= n then True
    else if i < bytes-len b then
      if ne-u8 (raw-load-u8 (wrapping-add-i64 addr (nat-to-i64 i))) (bytes-get-u8 b i) then False
      else dma-eq-bytes-loop addr b (i + 1) n
    else False

  -- Drain all gRPC frames inside a DATA slice `[src + off, src + end)` and count + sum first byte!
  drain-client-msgs :: GrpcFrameAssembler -> GrpcCallState -> I64 -> U32 -> U32 -> U32 ->{MVar} (GrpcCallState, U32)
  drain-client-msgs asm st src off end acc =
    if ge-u32 off end then (st, acc)
    else
      let dr = grpc-frame-decode-incremental asm src off end in
      if eq-u8 dr.gfd-state grpc-decode-ok then
        let st1 = grpc-client-on-resp-message st in
        let b0 = if gt-u32 dr.gfd-msg-len 0 then u8-to-u32 (raw-load-u8 dr.gfd-msg-addr) else 0 in
        drain-client-msgs asm st1 src (wrapping-add-u32 off dr.gfd-consumed) end (wrapping-add-u32 acc b0)
      else (st, acc)

  drain-server-msgs :: GrpcFrameAssembler -> GrpcCallState -> I64 -> U32 -> U32 -> U32 ->{MVar} (GrpcCallState, U32)
  drain-server-msgs asm st src off end acc =
    if ge-u32 off end then (st, acc)
    else
      let dr = grpc-frame-decode-incremental asm src off end in
      if eq-u8 dr.gfd-state grpc-decode-ok then
        let st1 = grpc-server-on-req-message st in
        let b0 = if gt-u32 dr.gfd-msg-len 0 then u8-to-u32 (raw-load-u8 dr.gfd-msg-addr) else 0 in
        drain-server-msgs asm st1 src (wrapping-add-u32 off dr.gfd-consumed) end (wrapping-add-u32 acc b0)
      else (st, acc)

  -- Feed all HTTP/2 wire frames from server TX buffer `[wire, wire + wire-len)` into client `H2Conn`
  feed-client-all :: H2Conn -> GrpcFrameAssembler -> GrpcCallState -> I64 -> U32 -> U32 -> I64 -> U32 ->{MVar} (GrpcCallState, U32)
  feed-client-all cli asm st wire off wire-len ack-tx acc =
    if ge-u32 off wire-len then (st, acc)
    else
      let cf = grpc-client-feed cli wire off wire-len ack-tx 0 512 in
      if eq-u32 cf.gcfr-consumed 0 then (st, acc)
      else
        let next-off = wrapping-add-u32 off cf.gcfr-consumed in
        if eq-u32 cf.gcfr-kind grpc-cf-data then
          let (st1, acc1) = drain-client-msgs asm st wire cf.gcfr-data-off (wrapping-add-u32 cf.gcfr-data-off cf.gcfr-data-len) acc in
          feed-client-all cli asm st1 wire next-off wire-len ack-tx acc1
        else
          let st1 = grpc-client-apply-feed cli st cf in
          feed-client-all cli asm st1 wire next-off wire-len ack-tx acc

  -- Feed all HTTP/2 wire frames from client TX buffer `[wire, wire + wire-len)` into server `H2Conn`
  feed-server-data-all :: H2Conn -> GrpcFrameAssembler -> GrpcCallState -> I64 -> U32 -> U32 -> I64 -> U32 ->{MVar} (GrpcCallState, U32)
  feed-server-data-all srv asm st wire off wire-len ack-tx acc =
    if ge-u32 off wire-len then (st, acc)
    else
      let fr = h2-conn-feed srv wire off wire-len ack-tx 0 512 in
      if eq-u32 fr.fr-consumed 0 then (st, acc)
      else
        let next-off = wrapping-add-u32 off fr.fr-consumed in
        if eq-u32 fr.fr-status h2-feed-data-ready then
          let doff = h2-conn-last-data-off srv in
          let dlen = h2-conn-last-data-len srv in
          let (st1, acc1) = drain-server-msgs asm st wire doff (wrapping-add-u32 doff dlen) acc in
          let slot = h2-stream-lookup (h2-conn-streams srv) st1.gcs-stream-id in
          let st2 = (
            if eq-u32 slot h2-slot-none then grpc-server-on-req-end st1
            else if eq-u8 (h2-stream-slot-state (h2-conn-streams srv) slot) h2-st-half-closed-remote then
              grpc-server-on-req-end st1
            else st1) in
          feed-server-data-all srv asm st2 wire next-off wire-len ack-tx acc1
        else
          feed-server-data-all srv asm st wire next-off wire-len ack-tx acc

  test-unary
    :: H2Conn -> H2Conn -> GrpcFrameAssembler -> GrpcFrameAssembler
    -> I64 -> I64 -> I64 -> I64 -> U32 -> I64 -> I64
    ->{MVar} Bool
  test-unary cli srv asm-c asm-s c2s s2c path-buf auth-buf auth-len msg-buf ack-buf =
    let u-path-len = copy-str "/demo.Greeter/SayHello" path-buf in
    let u-req-len = copy-str "hello-unary" msg-buf in
    let uc0 = grpc-client-start-call cli 1 grpc-mode-unary 65536 path-buf u-path-len auth-buf auth-len 5000000000 c2s 0 16384 in
    let uc1 = grpc-client-send-message cli uc0 0 msg-buf u-req-len 1 c2s uc0.gcs-last-tx-pos 16384 in
    let uf0 = h2-conn-feed srv c2s 0 uc1.gcs-last-tx-pos ack-buf 0 512 in
    let us0 = grpc-server-accept-call srv 1 grpc-mode-unary 65536 in
    let (us1, u-req-sum) = feed-server-data-all srv asm-s us0 c2s uf0.fr-consumed uc1.gcs-last-tx-pos ack-buf 0 in
    let u-res-len = copy-str "world-unary" msg-buf in
    let us2 = grpc-server-send-headers srv us1 s2c 0 16384 in
    let us3 = grpc-server-send-message srv us2 0 msg-buf u-res-len s2c us2.gcs-last-tx-pos 16384 in
    let us4 = grpc-server-finish srv us3 grpc-code-ok 0 0 s2c us3.gcs-last-tx-pos 16384 in
    let (uc2, u-res-sum) = feed-client-all cli asm-c uc1 s2c 0 us4.gcs-last-tx-pos ack-buf 0 in
    let uc-dup = grpc-server-on-req-message (grpc-call-new-server 99 grpc-mode-unary 65536 0) in
    let uc-dup2 = grpc-server-on-req-message uc-dup in
    eq-u32 uf0.fr-status h2-feed-headers-ready
      && eq-u8 us0.gcs-phase grpc-phase-active && eq-u64 us0.gcs-deadline-ns 5000000000
      && eq-u8 us1.gcs-phase grpc-phase-half-closed-req && eq-u32 us1.gcs-req-count 1 && eq-u32 u-req-sum 104
      && eq-u8 us4.gcs-phase grpc-phase-closed && eq-u32 us4.gcs-resp-count 1 && eq-u8 us4.gcs-trailers-only 0
      && eq-u8 uc2.gcs-phase grpc-phase-closed && eq-u8 uc2.gcs-status grpc-code-ok
      && eq-u32 uc2.gcs-resp-count 1 && eq-u32 u-res-sum 119
      && eq-u8 uc-dup2.gcs-phase grpc-phase-failed && eq-u8 uc-dup2.gcs-status grpc-code-internal

  test-server-stream
    :: H2Conn -> H2Conn -> GrpcFrameAssembler -> GrpcFrameAssembler
    -> I64 -> I64 -> I64 -> I64 -> U32 -> I64 -> I64
    ->{MVar} Bool
  test-server-stream cli srv asm-c asm-s c2s s2c path-buf auth-buf auth-len msg-buf ack-buf =
    let ss-path-len = copy-str "/demo.Feed/Subscribe" path-buf in
    let ss-req-len = copy-str "topic=prices" msg-buf in
    let sc0 = grpc-client-start-call cli 3 grpc-mode-server-streaming 65536 path-buf ss-path-len auth-buf auth-len 0 c2s 0 16384 in
    let sc1 = grpc-client-send-message cli sc0 0 msg-buf ss-req-len 1 c2s sc0.gcs-last-tx-pos 16384 in
    let sf0 = h2-conn-feed srv c2s 0 sc1.gcs-last-tx-pos ack-buf 0 512 in
    let ss0 = grpc-server-accept-call srv 3 grpc-mode-server-streaming 65536 in
    let (ss1, _) = feed-server-data-all srv asm-s ss0 c2s sf0.fr-consumed sc1.gcs-last-tx-pos ack-buf 0 in
    let _ = raw-store-u8 msg-buf 65 in
    let ss2 = grpc-server-send-message srv ss1 0 msg-buf 4 s2c 0 16384 in
    let _ = raw-store-u8 msg-buf 66 in
    let ss3 = grpc-server-send-message srv ss2 0 msg-buf 4 s2c ss2.gcs-last-tx-pos 16384 in
    let _ = raw-store-u8 msg-buf 67 in
    let ss4 = grpc-server-send-message srv ss3 0 msg-buf 4 s2c ss3.gcs-last-tx-pos 16384 in
    let _ = raw-store-u8 msg-buf 68 in
    let ss5 = grpc-server-send-message srv ss4 0 msg-buf 4 s2c ss4.gcs-last-tx-pos 16384 in
    let ss6 = grpc-server-finish srv ss5 grpc-code-ok 0 0 s2c ss5.gcs-last-tx-pos 16384 in
    let (sc2, ss-sum) = feed-client-all cli asm-c sc1 s2c 0 ss6.gcs-last-tx-pos ack-buf 0 in
    eq-u32 ss1.gcs-req-count 1 && eq-u32 ss6.gcs-resp-count 4 && eq-u8 ss6.gcs-phase grpc-phase-closed
      && eq-u8 sc2.gcs-phase grpc-phase-closed && eq-u8 sc2.gcs-status grpc-code-ok
      && eq-u32 sc2.gcs-resp-count 4 && eq-u32 ss-sum 266

  test-client-stream
    :: H2Conn -> H2Conn -> GrpcFrameAssembler -> GrpcFrameAssembler
    -> I64 -> I64 -> I64 -> I64 -> U32 -> I64 -> I64
    ->{MVar} Bool
  test-client-stream cli srv asm-c asm-s c2s s2c path-buf auth-buf auth-len msg-buf ack-buf =
    let cs-path-len = copy-str "/demo.Upload/PushBatch" path-buf in
    let cc0 = grpc-client-start-call cli 5 grpc-mode-client-streaming 65536 path-buf cs-path-len auth-buf auth-len 0 c2s 0 16384 in
    let _ = raw-store-u8 msg-buf 10 in
    let cc1 = grpc-client-send-message cli cc0 0 msg-buf 3 0 c2s cc0.gcs-last-tx-pos 16384 in
    let _ = raw-store-u8 msg-buf 20 in
    let cc2 = grpc-client-send-message cli cc1 0 msg-buf 3 0 c2s cc1.gcs-last-tx-pos 16384 in
    let _ = raw-store-u8 msg-buf 30 in
    let cc3 = grpc-client-send-message cli cc2 0 msg-buf 3 0 c2s cc2.gcs-last-tx-pos 16384 in
    let _ = raw-store-u8 msg-buf 40 in
    let cc4 = grpc-client-send-message cli cc3 0 msg-buf 3 0 c2s cc3.gcs-last-tx-pos 16384 in
    let cc5 = grpc-client-half-close cli cc4 c2s cc4.gcs-last-tx-pos 16384 in
    let cf0 = h2-conn-feed srv c2s 0 cc5.gcs-last-tx-pos ack-buf 0 512 in
    let cs0 = grpc-server-accept-call srv 5 grpc-mode-client-streaming 65536 in
    let (cs1, cs-sum) = feed-server-data-all srv asm-s cs0 c2s cf0.fr-consumed cc5.gcs-last-tx-pos ack-buf 0 in
    let _ = raw-store-u8 msg-buf 100 in
    let cs2 = grpc-server-send-message srv cs1 0 msg-buf 4 s2c 0 16384 in
    let cs3 = grpc-server-finish srv cs2 grpc-code-ok 0 0 s2c cs2.gcs-last-tx-pos 16384 in
    let (cc6, cc-res-sum) = feed-client-all cli asm-c cc5 s2c 0 cs3.gcs-last-tx-pos ack-buf 0 in
    eq-u32 cc5.gcs-req-count 4 && eq-u8 cc5.gcs-phase grpc-phase-half-closed-req
      && eq-u32 cs1.gcs-req-count 4 && eq-u32 cs-sum 100 && eq-u8 cs1.gcs-phase grpc-phase-half-closed-req
      && eq-u8 cs3.gcs-phase grpc-phase-closed && eq-u32 cs3.gcs-resp-count 1
      && eq-u8 cc6.gcs-phase grpc-phase-closed && eq-u8 cc6.gcs-status grpc-code-ok
      && eq-u32 cc6.gcs-resp-count 1 && eq-u32 cc-res-sum 100

  test-bidi-stream
    :: H2Conn -> H2Conn -> GrpcFrameAssembler -> GrpcFrameAssembler
    -> I64 -> I64 -> I64 -> I64 -> U32 -> I64 -> I64
    ->{MVar} Bool
  test-bidi-stream cli srv asm-c asm-s c2s s2c path-buf auth-buf auth-len msg-buf ack-buf =
    let bd-path-len = copy-str "/demo.Chat/Converse" path-buf in
    let bc0 = grpc-client-start-call cli 7 grpc-mode-bidi-streaming 65536 path-buf bd-path-len auth-buf auth-len 0 c2s 0 16384 in
    let _ = raw-store-u8 msg-buf 49 in
    let bc1 = grpc-client-send-message cli bc0 0 msg-buf 4 0 c2s bc0.gcs-last-tx-pos 16384 in
    let bf0 = h2-conn-feed srv c2s 0 bc1.gcs-last-tx-pos ack-buf 0 512 in
    let bs0 = grpc-server-accept-call srv 7 grpc-mode-bidi-streaming 65536 in
    let (bs1, bd-r1) = feed-server-data-all srv asm-s bs0 c2s bf0.fr-consumed bc1.gcs-last-tx-pos ack-buf 0 in
    let _ = raw-store-u8 msg-buf 97 in
    let bs2 = grpc-server-send-message srv bs1 0 msg-buf 4 s2c 0 16384 in
    let (bc2, bd-c1) = feed-client-all cli asm-c bc1 s2c 0 bs2.gcs-last-tx-pos ack-buf 0 in
    let _ = raw-store-u8 msg-buf 50 in
    let bc3 = grpc-client-send-message cli bc2 0 msg-buf 4 0 c2s 0 16384 in
    let _ = raw-store-u8 msg-buf 51 in
    let bc4 = grpc-client-send-message cli bc3 0 msg-buf 4 1 c2s bc3.gcs-last-tx-pos 16384 in
    let (bs3, bd-r2) = feed-server-data-all srv asm-s bs2 c2s 0 bc4.gcs-last-tx-pos ack-buf bd-r1 in
    let _ = raw-store-u8 msg-buf 98 in
    let bs4 = grpc-server-send-message srv bs3 0 msg-buf 4 s2c 0 16384 in
    let _ = raw-store-u8 msg-buf 99 in
    let bs5 = grpc-server-send-message srv bs4 0 msg-buf 4 s2c bs4.gcs-last-tx-pos 16384 in
    let bs6 = grpc-server-finish srv bs5 grpc-code-ok 0 0 s2c bs5.gcs-last-tx-pos 16384 in
    let (bc5, bd-c2) = feed-client-all cli asm-c bc4 s2c 0 bs6.gcs-last-tx-pos ack-buf bd-c1 in
    eq-u8 bc2.gcs-phase grpc-phase-active && eq-u32 bc2.gcs-resp-count 1 && eq-u32 bd-c1 97
      && eq-u32 bs3.gcs-req-count 3 && eq-u32 bd-r2 150
      && eq-u8 bs6.gcs-phase grpc-phase-closed && eq-u32 bs6.gcs-resp-count 3
      && eq-u8 bc5.gcs-phase grpc-phase-closed && eq-u8 bc5.gcs-status grpc-code-ok
      && eq-u32 bc5.gcs-resp-count 3 && eq-u32 bd-c2 294

  test-trailers-only-unimpl
    :: H2Conn -> H2Conn -> GrpcFrameAssembler
    -> I64 -> I64 -> I64 -> I64 -> U32 -> I64 -> I64 -> I64
    ->{MVar} Bool
  test-trailers-only-unimpl cli srv asm-s c2s s2c path-buf auth-buf auth-len err-buf dec-buf ack-buf =
    let un-path-len = copy-str "/unknown.Svc/Missing" path-buf in
    let tc0 = grpc-client-start-call cli 9 grpc-mode-unary 65536 path-buf un-path-len auth-buf auth-len 0 c2s 0 16384 in
    let tc1 = grpc-client-half-close cli tc0 c2s tc0.gcs-last-tx-pos 16384 in
    let tf0 = h2-conn-feed srv c2s 0 tc1.gcs-last-tx-pos ack-buf 0 512 in
    let ts0 = grpc-server-accept-call srv 9 grpc-mode-unary 65536 in
    let (ts1, _) = feed-server-data-all srv asm-s ts0 c2s tf0.fr-consumed tc1.gcs-last-tx-pos ack-buf 0 in
    let em1-len = copy-str "method /unknown.Svc/Missing not found" err-buf in
    let ts2 = grpc-server-finish srv ts1 grpc-code-unimplemented err-buf em1-len s2c 0 16384 in
    let cf-un = grpc-client-feed cli s2c 0 ts2.gcs-last-tx-pos ack-buf 0 512 in
    let tc2 = grpc-client-apply-feed cli tc1 cf-un in
    let meta-un = grpc-parse-response-headers (h2-conn-decoded-hdrs-addr cli) (h2-conn-decoded-text-addr cli) (u16-to-u32 cf-un.gcfr-hdr-count) in
    let dec-un = grpc-message-decode-dma (wrapping-add-i64 (h2-conn-decoded-text-addr cli) (u32-to-i64 meta-un.gres-msg-off)) meta-un.gres-msg-len dec-buf 0 256 in
    eq-u8 ts2.gcs-trailers-only 1 && eq-u8 ts2.gcs-phase grpc-phase-closed
      && eq-u32 cf-un.gcfr-kind grpc-cf-trailers-only && eq-u8 cf-un.gcfr-end-stream 1
      && eq-u8 tc2.gcs-phase grpc-phase-closed && eq-u8 tc2.gcs-trailers-only 1
      && eq-u8 tc2.gcs-status grpc-code-unimplemented
      && eq-u16 meta-un.gres-http-status 200
      && eq-u8 dec-un.gc-ok 1
      && dma-eq-str dec-buf dec-un.gc-written "method /unknown.Svc/Missing not found"

  test-trailers-only-deadline
    :: H2Conn -> H2Conn -> GrpcFrameAssembler
    -> I64 -> I64 -> I64 -> I64 -> U32 -> I64 -> I64 -> I64
    ->{MVar} Bool
  test-trailers-only-deadline cli srv asm-s c2s s2c path-buf auth-buf auth-len err-buf dec-buf ack-buf =
    let dl-path-len = copy-str "/demo.Slow/Wait" path-buf in
    let dc0 = grpc-client-start-call cli 11 grpc-mode-unary 65536 path-buf dl-path-len auth-buf auth-len 1000000 c2s 0 16384 in
    let dc1 = grpc-client-half-close cli dc0 c2s dc0.gcs-last-tx-pos 16384 in
    let df0 = h2-conn-feed srv c2s 0 dc1.gcs-last-tx-pos ack-buf 0 512 in
    let ds0 = grpc-server-accept-call srv 11 grpc-mode-unary 65536 in
    let (ds1, _) = feed-server-data-all srv asm-s ds0 c2s df0.fr-consumed dc1.gcs-last-tx-pos ack-buf 0 in
    let em2-len = copy-str "100% deadline exceeded\n" err-buf in
    let ds2 = grpc-server-finish srv ds1 grpc-code-deadline-exceeded err-buf em2-len s2c 0 16384 in
    let cf-dl = grpc-client-feed cli s2c 0 ds2.gcs-last-tx-pos ack-buf 0 512 in
    let dc2 = grpc-client-apply-feed cli dc1 cf-dl in
    let meta-dl = grpc-parse-response-headers (h2-conn-decoded-hdrs-addr cli) (h2-conn-decoded-text-addr cli) (u16-to-u32 cf-dl.gcfr-hdr-count) in
    let dec-dl = grpc-message-decode-dma (wrapping-add-i64 (h2-conn-decoded-text-addr cli) (u32-to-i64 meta-dl.gres-msg-off)) meta-dl.gres-msg-len dec-buf 0 256 in
    eq-u64 ds0.gcs-deadline-ns 1000000
      && eq-u8 ds2.gcs-trailers-only 1
      && eq-u32 cf-dl.gcfr-kind grpc-cf-trailers-only
      && eq-u8 dc2.gcs-phase grpc-phase-closed && eq-u8 dc2.gcs-status grpc-code-deadline-exceeded
      && eq-u8 dec-dl.gc-ok 1
      && dma-eq-str dec-buf dec-dl.gc-written "100% deadline exceeded\n"

  main :: Unit ->{IO, MVar} Int
  main _ =
    let cli = h2-conn-new h2-role-client 32 in
    let srv = h2-conn-new h2-role-server 32 in
    let dma = engine-dma-alloc 65536 4096 in
    if not (h2-conn-valid cli) || not (h2-conn-valid srv) || eq-u64 dma 0 then 99
    else
      let c2s = wrapping-u64-to-i64 dma in
      let s2c = wrapping-add-i64 c2s 16384 in
      let scratch-c = wrapping-add-i64 s2c 16384 in
      let scratch-s = wrapping-add-i64 scratch-c 8192 in
      let tmp = wrapping-add-i64 scratch-s 8192 in
      let path-buf = tmp in
      let auth-buf = wrapping-add-i64 tmp 256 in
      let msg-buf = wrapping-add-i64 tmp 512 in
      let err-buf = wrapping-add-i64 tmp 1024 in
      let dec-buf = wrapping-add-i64 tmp 1536 in
      let ack-buf = wrapping-add-i64 tmp 2048 in

      let asm-c = grpc-assembler-init scratch-c 8192 grpc-default-max-message-size in
      let asm-s = grpc-assembler-init scratch-s 8192 grpc-default-max-message-size in
      let auth-len = copy-str "localhost:50051" auth-buf in

      -- === 0. Real HTTP/2 Client-Server Handshake (Preface + SETTINGS + ACKs) ===
      let p0 = h2-preface-write-dma c2s 0 16384 in
      let p1 = h2-conn-write-initial-settings cli c2s p0 16384 in
      let hs0 = h2-conn-feed srv c2s 0 p1 s2c 0 16384 in
      let hs1 = h2-conn-feed srv c2s hs0.fr-consumed p1 s2c 0 16384 in
      let sp1 = h2-conn-write-initial-settings srv s2c hs1.fr-tx-pos 16384 in
      let ch0 = grpc-client-feed cli s2c 0 sp1 ack-buf 0 512 in
      let ch1 = grpc-client-feed cli s2c ch0.gcfr-consumed sp1 ack-buf 0 512 in
      let ok-handshake = (
        eq-u32 hs0.fr-status h2-feed-ok && eq-u8 (h2-conn-state srv) h2-conn-open
          && eq-u32 hs1.fr-status h2-feed-ok
          && eq-u32 ch0.gcfr-kind grpc-cf-ok
          && eq-u32 ch1.gcfr-kind grpc-cf-ok) in

      let ok-unary = test-unary cli srv asm-c asm-s c2s s2c path-buf auth-buf auth-len msg-buf ack-buf in
      let ok-server-stream = test-server-stream cli srv asm-c asm-s c2s s2c path-buf auth-buf auth-len msg-buf ack-buf in
      let ok-client-stream = test-client-stream cli srv asm-c asm-s c2s s2c path-buf auth-buf auth-len msg-buf ack-buf in
      let ok-bidi = test-bidi-stream cli srv asm-c asm-s c2s s2c path-buf auth-buf auth-len msg-buf ack-buf in
      let ok-unimpl = test-trailers-only-unimpl cli srv asm-s c2s s2c path-buf auth-buf auth-len err-buf dec-buf ack-buf in
      let ok-deadline = test-trailers-only-deadline cli srv asm-s c2s s2c path-buf auth-buf auth-len err-buf dec-buf ack-buf in

      -- Both client and server H2StreamTables must have 0 leaked active streams!
      let cli-active = h2-stream-table-active-count (h2-conn-streams cli) in
      let srv-active = h2-stream-table-active-count (h2-conn-streams srv) in

      let _ = engine-dma-free dma 65536 in
      let _ = h2-conn-destroy cli in
      let _ = h2-conn-destroy srv in

      if not ok-handshake then 10
      else if not ok-unary then 11
      else if not ok-server-stream then 12
      else if not ok-client-stream then 13
      else if not ok-bidi then 14
      else if not ok-unimpl then 15
      else if not ok-deadline then 16
      else if ne-u32 cli-active 0 then 17
      else if ne-u32 srv-active 0 then 18
      else 0
"#;
    build_and_run_leak_checked("e2e-4-modes-trailers-only", src, 0);
}

// =========================================================================
// 4. Steady-state 50,000 gRPC frame encode/decode + timeout parse +
//    response/trailer cycle: allocs == 0, atomic_rc == 0, leaked == 0
// =========================================================================

#[test]
fn steady_state_50000_grpc_cycles_zero_alloc_zero_atomic_rc() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.String
  import Engine.Types
  import Net.Http.V2.Frame
  import Net.Http.V2.Stream
  import Net.Http.V2.Conn
  import Net.Http.Grpc
  import Prim.Atomic

  copy-str :: String -> I64 ->{MVar} U32
  copy-str s dst =
    let b = string-to-bytes s in
    let n = bytes-len b in
    let _ = copy-bytes b 0 dst n in
    wrapping-i64-to-u32 (nat-to-i64 n)

  copy-bytes :: Bytes -> Nat -> I64 -> Nat ->{MVar} Unit
  copy-bytes b i dst n =
    if i >= n then ()
    else if i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 dst (nat-to-i64 i)) (bytes-get-u8 b i) in
      copy-bytes b (i + 1) dst n
    else ()

  -- 50,000 steady-state cycles:
  --   * 5B gRPC frame encode + decode (`grpc-frame-encode-raw` + `grpc-frame-decode-raw`)
  --   * `grpc-timeout` format + parse round-trip (`grpc-format-timeout-dma` + `grpc-parse-timeout-dma`)
  --   * Real `Net.Http.V2.Conn` stream open -> `grpc-send-response-headers` ->
  --     `grpc-send-message` -> `grpc-send-trailers` -> stream close!
  -- Every single call inside this loop is statically verified `#[no-alloc]`!
  #[no-alloc]
  hot-loop50000
    :: H2Conn -> I64 -> I64 -> I64 -> I64 -> U32 -> U32 -> U32 -> U32
    ->{MVar} U32
  hot-loop50000 srv rx tx msg-buf to-buf sid done limit acc =
    if ge-u32 done limit then acc
    else
      -- 1. 5-byte gRPC frame encode + decode
      let fw = grpc-frame-encode-raw tx 0 256 0 msg-buf 16 in
      let fd = grpc-frame-decode-raw tx 0 fw.gfw-bytes-written grpc-default-max-message-size in
      if ne-u8 fd.gfd-state grpc-decode-ok then 0
      else
        -- 2. grpc-timeout format + parse
        let to-len = grpc-format-timeout-dma 250000000 to-buf 0 32 in
        let tr = grpc-parse-timeout-dma to-buf to-len in
        if eq-u8 tr.gt-ok 0 then 0
        else
          -- 3. Full H2 stream open (half-closed-remote) + gRPC response headers + message + trailers!
          let st = h2-conn-streams srv in
          let slot = h2-stream-open st sid 65535 65535 in
          if eq-u32 slot h2-slot-none then 0
          else
            let _ = h2-stream-slot-set-state st slot h2-st-half-closed-remote in
            let w0 = grpc-send-response-headers srv sid tx 0 1024 in
            let w1 = grpc-send-message srv sid 0 msg-buf 8 0 tx w0.wr-tx-pos 1024 in
            let w2 = grpc-send-trailers srv sid grpc-code-ok 0 0 tx w1.wr-tx-pos 1024 in
            if ne-u32 w2.wr-error h2-err-no-error then 0
            else
              -- Refill connection send window periodically so 50,000 x 13B = 650 KiB never stalls
              let _ = (
                if lt-i64 (h2-conn-send-window srv) 16384 then
                  let wu = h2-write-window-update-dma rx 0 64 0 65536 in
                  let _ = h2-conn-feed srv rx 0 wu tx 0 256 in
                  let _ = h2-conn-refill-budgets srv 0 1 in
                  ()
                else ()) in
              hot-loop50000 srv rx tx msg-buf to-buf (wrapping-add-u32 sid 2) (wrapping-add-u32 done 1) limit (wrapping-add-u32 acc 1)

  main :: Unit ->{IO, MVar} Int
  main _ =
    let srv = h2-conn-new h2-role-server 32 in
    let dma = engine-dma-alloc 16384 4096 in
    if not (h2-conn-valid srv) || eq-u64 dma 0 then 1
    else
      let rx = wrapping-u64-to-i64 dma in
      let tx = wrapping-add-i64 rx 4096 in
      let msg-buf = wrapping-add-i64 tx 4096 in
      let to-buf = wrapping-add-i64 msg-buf 1024 in
      let _ = copy-str "grpc-hot-payload" msg-buf in
      -- Complete HTTP/2 preface handshake before zero-alloc steady-state measurement
      let plen = h2-preface-write-dma rx 0 64 in
      let fp = h2-conn-feed srv rx 0 plen tx 0 256 in
      if ne-u32 fp.fr-status h2-feed-ok then
        let _ = engine-dma-free dma 16384 in
        let _ = h2-conn-destroy srv in 2
      else
        let _ = engine-thread-counters-reset () in
        let completed = hot-loop50000 srv rx tx msg-buf to-buf 1 0 50000 0 in
        let allocs = engine-thread-alloc-count () in
        let atomic-rcs = engine-thread-atomic-rc-count () in
        let st = h2-conn-streams srv in
        let active = h2-stream-table-active-count st in
        let opened = h2-stream-table-total-opened st in
        let closed = h2-stream-table-total-closed st in
        let _ = engine-dma-free dma 16384 in
        let _ = h2-conn-destroy srv in
        if ne-u32 completed 50000 then 10
        else if ne-u64 allocs 0 then 11
        else if ne-u64 atomic-rcs 0 then 12
        else if ne-u32 active 0 then 13
        else if ne-u64 opened 50000 then 14
        else if ne-u64 closed 50000 then 15
        else 0
"#;
    build_and_run_leak_checked("steady-50000-zero-alloc", src, 0);
}
