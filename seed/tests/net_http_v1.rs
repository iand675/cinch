//! Integration, formal-verification (M5 `Http1Machine`), RFC 9112 smuggling &
//! conformance corpus, 1,000-trial Hegel differential fuzzing, live macOS
//! loopback HTTP/1.1 pipelined server, and 50,000-request zero-allocation
//! hot-loop tests for `Net.Http.V1.{Parser,Conn,Spec}`.
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
        "cinch-net-http-v1-{tag}-{}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn build_and_run_leak_checked(tag: &str, src: &str, expected_code: i32) -> (String, String) {
    let dir = fresh_dir(tag);
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let exe = dir.join(format!("http1-{tag}"));
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
fn net_http_v1_manifest_registration_and_no_alloc_check() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, rel_path) in [
        ("Net.Http.V1.Parser", "Net/Http/V1/Parser.cinch"),
        ("Net.Http.V1.Conn", "Net/Http/V1/Conn.cinch"),
        ("Net.Http.V1.Spec", "Net/Http/V1/Spec.cinch"),
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

    let pos_scan = pkg.order.iter().position(|n| n == "Engine.Simd.Scan").unwrap();
    let pos_types = pkg.order.iter().position(|n| n == "Net.Http.Types").unwrap();
    let pos_timer = pkg.order.iter().position(|n| n == "Engine.Timer").unwrap();
    let pos_parser = pkg.order.iter().position(|n| n == "Net.Http.V1.Parser").unwrap();
    let pos_conn = pkg.order.iter().position(|n| n == "Net.Http.V1.Conn").unwrap();
    assert!(pos_scan < pos_parser && pos_types < pos_parser);
    assert!(pos_parser < pos_conn && pos_timer < pos_conn);

    // Compile-time #[no-alloc] audit across all hot-path Parser & Conn entry points
    let dir = fresh_dir("no-alloc-check");
    std::fs::write(
        dir.join("Main.cinch"),
        r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Engine.Timer
  import Engine.Types
  import Net.Http.Types
  import Net.Http.V1.Parser
  import Net.Http.V1.Conn

  #[no-alloc]
  audit-hot-path :: I64 -> I64 -> I64 -> I64 -> I64 -> I64 -> CoarseClockCache ->{MVar} U32
  audit-hot-path conn-addr rx-base hdrs-out req-out tx-base iov-base clk =
    let _init = h1-conn-slot-init conn-addr 16 in
    let rv = h1-parse-request-dma rx-base 0 64 0 hdrs-out in
    let _cs = h1-parse-chunk-size rx-base 0 16 in
    let _cc = h1-parse-chunk-crlf rx-base 0 2 in
    let _tr = h1-parse-trailers rx-base 0 2 8 in
    let ev = h1-conn-step-dma conn-addr rx-base 0 64 0 hdrs-out req-out in
    let _c0 = h1-conn-on-response-sent (h1-conn-on-head (h1-conn-init 8) rv) in
    let n = h1-serialize-response-dma tx-base 0 1024 (status-ok ()) clk (eq-u32 0 0) 0 0 0 0 in
    let niov = h1-build-response-iovec iov-base 0 tx-base n 0 0 in
    wrapping-add-u32 ev.ev-consumed niov

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
    .expect("Net.Http.V1.{Parser,Conn} #[no-alloc] hot-path audit must check cleanly");
    let _ = std::fs::remove_dir_all(&dir);
}

// =========================================================================
// 2. Formal Model M5 (`Http1Machine`): capped=false, 0 violations under
//    CINCH_MODEL_FAIL_ON_CAPPED=1 + refinement with decreasing measure
// =========================================================================

#[test]
fn formal_model_m5_http1_machine_verifies_uncapped_zero_violations() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("prelude/Net/Http/V1/Spec.cinch");
    let spec_str = spec_path.to_string_lossy().into_owned();
    let cinch_bin = env!("CARGO_BIN_EXE_cinch");

    // 1. Check via `common::drive_with_profile_opts` with `model_fail_on_capped = true`
    let dir = fresh_dir("m5-spec-check");
    std::fs::write(
        dir.join("Main.cinch"),
        r#"
module Main where
  import Net.Http.V1.Spec

  main :: Int
  main = if h1-safety-ok h1-model-init then 0 else 1
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
    .expect("Net.Http.V1.Spec must check cleanly with model_fail_on_capped=true");

    // 2. CLI `cinch model inspect <Spec.cinch> Http1Machine --json` with CINCH_MODEL_FAIL_ON_CAPPED=1
    let inspect_out = Command::new(cinch_bin)
        .args(["model", "inspect", &spec_str, "Http1Machine", "--json"])
        .env("CINCH_MODEL_FAIL_ON_CAPPED", "1")
        .output()
        .expect("run cinch model inspect Http1Machine");
    let stdout = String::from_utf8_lossy(&inspect_out.stdout);
    let stderr = String::from_utf8_lossy(&inspect_out.stderr);
    assert!(
        inspect_out.status.success(),
        "M5 Http1Machine failed under CINCH_MODEL_FAIL_ON_CAPPED=1:\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    let json: serde_json::Value =
        serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("invalid JSON: {e}\n{stdout}"));
    let machines = json
        .as_array()
        .expect("top-level machines array in model inspect JSON");
    let m5 = machines
        .iter()
        .find(|m| m.get("name").and_then(|v| v.as_str()) == Some("Http1Machine"))
        .unwrap_or_else(|| panic!("Http1Machine missing from JSON:\n{stdout}"));
    let capped = m5.get("capped").and_then(|v| v.as_bool()).expect("capped bool");
    let states = m5.get("state_count").and_then(|v| v.as_u64()).expect("state_count");
    let edges = m5.get("edge_count").and_then(|v| v.as_u64()).expect("edge_count");
    assert!(!capped, "M5 Http1Machine MUST have capped=false; json:\n{stdout}");
    assert!(states >= 50 && states <= 4096, "unexpected M5 state count: {states}");

    let contracts = m5
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

    let refines = m5
        .get("refinements")
        .and_then(|v| v.as_array())
        .expect("refinements array");
    let m5_ref = refines
        .iter()
        .find(|r| {
            r.get("concrete").and_then(|v| v.as_str()) == Some("Http1Machine")
                && r.get("abstract").and_then(|v| v.as_str()) == Some("AbstractReqResPipe")
        })
        .unwrap_or_else(|| {
            panic!("missing Http1Machine refines AbstractReqResPipe in JSON:\n{stdout}")
        });
    assert_eq!(
        m5_ref.get("passed").and_then(|v| v.as_bool()),
        Some(true),
        "refinement failed: {m5_ref:?}"
    );

    eprintln!(
        "[M5 Http1Machine] state_count={states} edge_count={edges} capped={capped} contracts={} refinement=PASS",
        contracts.len()
    );
    let _ = std::fs::remove_dir_all(&dir);
}

// =========================================================================
// 3. RFC 9112 smuggling & malformed corpus (30+ attack vectors) + valid
//    pipelined, incremental NeedMore, chunked, 100-continue & Upgrade cases
// =========================================================================

#[test]
fn rfc9112_smuggling_corpus_and_conformance_suite() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.String
  import Engine.Timer
  import Engine.Types
  import Net.Http.Types
  import Net.Http.V1.Parser
  import Net.Http.V1.Conn
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
      if eq-u8 got exp then dma-eq-bytes base off b (i + 1) n
      else False
    else False

  expect-req-err :: I64 -> I64 -> String -> U8 ->{MVar} Bool
  expect-req-err buf hdrs wire want =
    let n = copy-str-to-dma wire buf in
    let rv = h1-parse-request-dma buf 0 n 0 hdrs in
    eq-u8 rv.rv-status want

  -- Build a request with `k` valid X-H headers (to test > 64 header cap).
  build-many-headers :: I64 -> U32 -> U32 -> U32 ->{MVar} U32
  build-many-headers buf pos i total =
    if ge-u32 i total then
      let _0 = raw-store-u8 (wrapping-add-i64 buf (u32-to-i64 pos)) 13 in
      let _1 = raw-store-u8 (wrapping-add-i64 buf (u32-to-i64 (wrapping-add-u32 pos 1))) 10 in
      wrapping-add-u32 pos 2
    else
      -- "X-A: 1\r\n" (8 bytes)
      let p = wrapping-add-i64 buf (u32-to-i64 pos) in
      let _0 = raw-store-u8 p 88 in
      let _1 = raw-store-u8 (wrapping-add-i64 p 1) 45 in
      let _2 = raw-store-u8 (wrapping-add-i64 p 2) 65 in
      let _3 = raw-store-u8 (wrapping-add-i64 p 3) 58 in
      let _4 = raw-store-u8 (wrapping-add-i64 p 4) 32 in
      let _5 = raw-store-u8 (wrapping-add-i64 p 5) 49 in
      let _6 = raw-store-u8 (wrapping-add-i64 p 6) 13 in
      let _7 = raw-store-u8 (wrapping-add-i64 p 7) 10 in
      build-many-headers buf (wrapping-add-u32 pos 8) (wrapping-add-u32 i 1) total

  -- Verify byte-by-byte incremental H1NeedMore on every prefix 0..n-1, then H1Ok at n.
  check-incremental-need-more :: I64 -> I64 -> U32 -> U32 ->{MVar} Bool
  check-incremental-need-more buf hdrs prefix full-len =
    if ge-u32 prefix full-len then
      let rv = h1-parse-request-dma buf 0 full-len 7 hdrs in
      eq-u8 rv.rv-status h1-ok && eq-u32 rv.rv-consumed full-len
    else
      let rv = h1-parse-request-dma buf 0 prefix 7 hdrs in
      if ne-u8 rv.rv-status h1-need-more then False
      else check-incremental-need-more buf hdrs (wrapping-add-u32 prefix 1) full-len

  -- Check all 17 pre-rendered status lines match Net.Http.Types.http-status-line byte-for-byte.
  check-status-match :: I64 -> U16 ->{MVar} Bool
  check-status-match tx code =
    let st = status-of code in
    let n = h1-write-status-line-dma tx 0 128 st in
    dma-eq-str tx 0 n (http-status-line st)

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 32768 4096 in
    if eq-u64 dma 0 then 99
    else
      let buf = wrapping-u64-to-i64 dma in
      let hdrs = wrapping-add-i64 buf 8192 in
      let conn = wrapping-add-i64 buf 12288 in
      let req-slot = wrapping-add-i64 buf 12544 in
      let tx = wrapping-add-i64 buf 16384 in

      -- === Part A: 30+ RFC 9112 smuggling & malformed attack vectors ===
      -- 1. CL + TE smuggling
      let ok01 = expect-req-err buf hdrs "POST / HTTP/1.1\r\nHost: a\r\nContent-Length: 5\r\nTransfer-Encoding: chunked\r\n\r\n" h1-err-smuggling-te-cl in
      -- 2. TE + CL smuggling
      let ok02 = expect-req-err buf hdrs "POST / HTTP/1.1\r\nHost: a\r\nTransfer-Encoding: chunked\r\nContent-Length: 5\r\n\r\n" h1-err-smuggling-te-cl in
      -- 3. Duplicate conflicting Content-Length
      let ok03 = expect-req-err buf hdrs "POST / HTTP/1.1\r\nHost: a\r\nContent-Length: 5\r\nContent-Length: 10\r\n\r\n" h1-err-duplicate-cl in
      -- 4. Duplicate identical Content-Length
      let ok04 = expect-req-err buf hdrs "POST / HTTP/1.1\r\nHost: a\r\nContent-Length: 5\r\nContent-Length: 5\r\n\r\n" h1-err-duplicate-cl in
      -- 5. Comma-separated Content-Length
      let ok05 = expect-req-err buf hdrs "POST / HTTP/1.1\r\nHost: a\r\nContent-Length: 5, 5\r\n\r\n" h1-err-bad-cl in
      -- 6. Negative Content-Length
      let ok06 = expect-req-err buf hdrs "POST / HTTP/1.1\r\nHost: a\r\nContent-Length: -1\r\n\r\n" h1-err-bad-cl in
      -- 7. Plus-signed Content-Length
      let ok07 = expect-req-err buf hdrs "POST / HTTP/1.1\r\nHost: a\r\nContent-Length: +5\r\n\r\n" h1-err-bad-cl in
      -- 8. Non-digit Content-Length
      let ok08 = expect-req-err buf hdrs "POST / HTTP/1.1\r\nHost: a\r\nContent-Length: 12x\r\n\r\n" h1-err-bad-cl in
      -- 9. U64 overflow in Content-Length
      let ok09 = expect-req-err buf hdrs "POST / HTTP/1.1\r\nHost: a\r\nContent-Length: 18446744073709551616\r\n\r\n" h1-err-bad-cl in
      -- 10. obs-fold with SP
      let ok10 = expect-req-err buf hdrs "GET / HTTP/1.1\r\nHost: a\r\n folded\r\n\r\n" h1-err-obs-fold in
      -- 11. obs-fold with HTAB
      let ok11 = expect-req-err buf hdrs "GET / HTTP/1.1\r\nHost: a\r\n\tfolded\r\n\r\n" h1-err-obs-fold in
      -- 12. Space before colon
      let ok12 = expect-req-err buf hdrs "GET / HTTP/1.1\r\nHost : a\r\n\r\n" h1-err-space-before-colon in
      -- 13. HTAB before colon
      let ok13 = expect-req-err buf hdrs "GET / HTTP/1.1\r\nHost\t: a\r\n\r\n" h1-err-space-before-colon in
      -- 14. Bare LF in request-line
      let ok14 = expect-req-err buf hdrs "GET / HTTP/1.1\nHost: a\r\n\r\n" h1-err-bare-lf in
      -- 15. Bare CR in request-line
      let ok15 = expect-req-err buf hdrs "GET / HTTP/1.1\rHost: a\r\n\r\n" h1-err-bare-cr in
      -- 16. Bare LF in header value
      let ok16 = expect-req-err buf hdrs "GET / HTTP/1.1\r\nHost: a\nX-Y: 1\r\n\r\n" h1-err-bare-lf in
      -- 17. Bare CR in header value
      let ok17 = expect-req-err buf hdrs "GET / HTTP/1.1\r\nHost: a\rX-Y: 1\r\n\r\n" h1-err-bare-cr in
      -- 18. NUL byte in header value
      let n18 = copy-str-to-dma "GET / HTTP/1.1\r\nHost: a\r\nX-B: aXb\r\n\r\n" buf in
      let _nul = raw-store-u8 (wrapping-add-i64 buf 32) 0 in
      let ok18 = eq-u8 (h1-parse-request-dma buf 0 n18 0 hdrs).rv-status h1-err-bad-header-value in
      -- 19. DEL (0x7F) byte in header value
      let n19 = copy-str-to-dma "GET / HTTP/1.1\r\nHost: a\r\nX-B: aXb\r\n\r\n" buf in
      let _del = raw-store-u8 (wrapping-add-i64 buf 32) 127 in
      let ok19 = eq-u8 (h1-parse-request-dma buf 0 n19 0 hdrs).rv-status h1-err-bad-header-value in
      -- 20. Non-tchar in method
      let ok20 = expect-req-err buf hdrs "G@T / HTTP/1.1\r\nHost: a\r\n\r\n" h1-err-bad-method in
      -- 21. Bad HTTP version (HTTP/2.0)
      let ok21 = expect-req-err buf hdrs "GET / HTTP/2.0\r\nHost: a\r\n\r\n" h1-err-bad-version in
      -- 22. Missing Host in HTTP/1.1
      let ok22 = expect-req-err buf hdrs "GET / HTTP/1.1\r\nUser-Agent: x\r\n\r\n" h1-err-missing-host in
      -- 23. Duplicate Host in HTTP/1.1
      let ok23 = expect-req-err buf hdrs "GET / HTTP/1.1\r\nHost: a\r\nHost: b\r\n\r\n" h1-err-duplicate-host in
      -- 24. Transfer-Encoding: gzip without final chunked
      let ok24 = expect-req-err buf hdrs "POST / HTTP/1.1\r\nHost: a\r\nTransfer-Encoding: gzip\r\n\r\n" h1-err-bad-te in
      -- 25. Duplicate chunked in Transfer-Encoding
      let ok25 = expect-req-err buf hdrs "POST / HTTP/1.1\r\nHost: a\r\nTransfer-Encoding: chunked, chunked\r\n\r\n" h1-err-bad-te in
      -- 26. Obfuscated Transfer-Encoding: xchunked
      let ok26 = expect-req-err buf hdrs "POST / HTTP/1.1\r\nHost: a\r\nTransfer-Encoding: xchunked\r\n\r\n" h1-err-bad-te in
      -- 27. Non-final chunked (chunked, gzip)
      let ok27 = expect-req-err buf hdrs "POST / HTTP/1.1\r\nHost: a\r\nTransfer-Encoding: chunked, gzip\r\n\r\n" h1-err-bad-te in
      -- 28. Whitespace inside Host value
      let ok28 = expect-req-err buf hdrs "GET / HTTP/1.1\r\nHost: good.com evil.com\r\n\r\n" h1-err-bad-header-value in
      -- 29. Empty header name
      let ok29 = expect-req-err buf hdrs "GET / HTTP/1.1\r\n: val\r\nHost: a\r\n\r\n" h1-err-bad-header-name in
      -- 30. > 64 headers (65 headers total including Host)
      let pfx30 = copy-str-to-dma "GET / HTTP/1.1\r\nHost: a\r\n" buf in
      let len30 = build-many-headers buf pfx30 0 64 in
      let ok30 = eq-u8 (h1-parse-request-dma buf 0 len30 0 hdrs).rv-status h1-err-too-many-headers in
      -- 31. Chunk size U64 overflow
      let n31 = copy-str-to-dma "10000000000000000\r\n" buf in
      let ok31 = eq-u8 (h1-parse-chunk-size buf 0 n31).cs-status h1-err-bad-chunk-size in
      -- 32. Chunk size with trailing space without semicolon
      let n32 = copy-str-to-dma "5 \r\n" buf in
      let ok32 = eq-u8 (h1-parse-chunk-size buf 0 n32).cs-status h1-err-bad-chunk-size in
      -- 33. Missing CRLF after chunk data (trailing garbage)
      let n33 = copy-str-to-dma "XY" buf in
      let ok33 = eq-u8 (h1-parse-chunk-crlf buf 0 n33) h1-err-bad-chunk-crlf in
      -- 34. Bare LF after chunk data
      let n34 = copy-str-to-dma "\n" buf in
      let ok34 = eq-u8 (h1-parse-chunk-crlf buf 0 n34) h1-err-bare-lf in
      -- 35. Forbidden Content-Length inside chunked trailers
      let n35 = copy-str-to-dma "Content-Length: 99\r\n\r\n" buf in
      let ok35 = eq-u8 (h1-parse-trailers buf 0 n35 16).tr-status h1-err-bad-trailer in

      if not (ok01 && ok02 && ok03 && ok04 && ok05 && ok06 && ok07 && ok08 && ok09 && ok10) then
        let _ = engine-dma-free dma 32768 in 10
      else if not (ok11 && ok12 && ok13 && ok14 && ok15 && ok16 && ok17 && ok18 && ok19 && ok20) then
        let _ = engine-dma-free dma 32768 in 11
      else if not (ok21 && ok22 && ok23 && ok24 && ok25 && ok26 && ok27 && ok28 && ok29 && ok30) then
        let _ = engine-dma-free dma 32768 in 12
      else if not (ok31 && ok32 && ok33 && ok34 && ok35) then
        let _ = engine-dma-free dma 32768 in 13
      else
        -- === Part B: Incremental H1NeedMore byte-by-byte without copying ===
        let n-inc = copy-str-to-dma "GET /api/v1/items?limit=10&sort=asc HTTP/1.1\r\nHost: api.cinch.dev\r\nAccept: application/json\r\n\r\n" buf in
        let ok-inc = check-incremental-need-more buf hdrs 0 n-inc in
        let rv-inc = h1-parse-request-dma buf 0 n-inc 7 hdrs in
        let p-span = request-view-path-span rv-inc in
        let q-span = request-view-query-span rv-inc in
        let h0 = request-view-header-at rv-inc 0 in
        let h1 = request-view-header-at rv-inc 1 in
        let ok-spans = (
          eq-u8 rv-inc.rv-method method-code-get
            && eq-u8 rv-inc.rv-version version-code-http11
            && eq-u16 rv-inc.rv-header-count 2
            && request-view-has-header rv-inc hdr-host
            && request-view-has-header rv-inc hdr-accept
            && request-view-has-flag rv-inc req-flag-keep-alive
            && eq-u32 p-span.span-buf 7
            && dma-eq-str buf p-span.span-off p-span.span-len "/api/v1/items"
            && dma-eq-str buf q-span.span-off q-span.span-len "limit=10&sort=asc"
            && eq-u8 h0.hp-known hdr-host
            && dma-eq-str buf h0.hp-val-off h0.hp-val-len "api.cinch.dev"
            && eq-u8 h1.hp-known hdr-accept
            && dma-eq-str buf h1.hp-val-off h1.hp-val-len "application/json") in

        -- === Part C: Pipelined fixed-body + chunked + 100-continue + Upgrade ===
        let pipe-len = copy-str-to-dma "POST /upload HTTP/1.1\r\nHost: a\r\nExpect: 100-continue\r\nContent-Length: 5\r\n\r\nhelloPOST /chunks HTTP/1.1\r\nHost: a\r\nTransfer-Encoding: gzip, chunked\r\n\r\n5;ext=1\r\nworld\r\n0\r\nX-Done: yes\r\n\r\nGET /ws HTTP/1.1\r\nHost: a\r\nConnection: Upgrade\r\nUpgrade: websocket\r\n\r\n" buf in
        let _ci = h1-conn-slot-init conn 8 in
        -- Step 1: POST /upload head (Expect: 100-continue, CL=5)
        let e1 = h1-conn-step-dma conn buf 0 pipe-len 0 hdrs req-slot in
        let off1 = e1.ev-consumed in
        let ok-e1 = (
          eq-u8 e1.ev-kind h1-ev-request-head
            && eq-u8 e1.ev-next-state h1-state-reading-fixed-body
            && ne-u8 (bit-and-u8 e1.ev-flags h1-ev-flag-expect-continue) 0
            && ne-u8 (bit-and-u8 e1.ev-flags h1-ev-flag-keep-alive) 0) in
        -- Step 2: Fixed body "hello" (5 bytes)
        let e2 = h1-conn-step-dma conn buf off1 pipe-len 0 hdrs req-slot in
        let off2 = wrapping-add-u32 off1 e2.ev-consumed in
        let ok-e2 = (
          eq-u8 e2.ev-kind h1-ev-body-chunk
            && eq-u8 e2.ev-next-state h1-state-reading-head
            && eq-u32 e2.ev-data-len 5
            && dma-eq-str buf e2.ev-data-off e2.ev-data-len "hello") in
        -- Step 3: Pipelined POST /chunks head (TE: gzip, chunked)
        let e3 = h1-conn-step-dma conn buf off2 pipe-len 0 hdrs req-slot in
        let off3 = wrapping-add-u32 off2 e3.ev-consumed in
        let ok-e3 = eq-u8 e3.ev-kind h1-ev-request-head && eq-u8 e3.ev-next-state h1-state-reading-chunk-size in
        -- Step 4: Chunk size "5;ext=1\r\n"
        let e4 = h1-conn-step-dma conn buf off3 pipe-len 0 hdrs req-slot in
        let off4 = wrapping-add-u32 off3 e4.ev-consumed in
        let ok-e4 = eq-u8 e4.ev-next-state h1-state-reading-chunk-data in
        -- Step 5: Chunk data "world"
        let e5 = h1-conn-step-dma conn buf off4 pipe-len 0 hdrs req-slot in
        let off5 = wrapping-add-u32 off4 e5.ev-consumed in
        let ok-e5 = eq-u8 e5.ev-next-state h1-state-reading-chunk-crlf && dma-eq-str buf e5.ev-data-off e5.ev-data-len "world" in
        -- Step 6: Chunk CRLF
        let e6 = h1-conn-step-dma conn buf off5 pipe-len 0 hdrs req-slot in
        let off6 = wrapping-add-u32 off5 e6.ev-consumed in
        let ok-e6 = eq-u8 e6.ev-next-state h1-state-reading-chunk-size in
        -- Step 7: Final 0 chunk size
        let e7 = h1-conn-step-dma conn buf off6 pipe-len 0 hdrs req-slot in
        let off7 = wrapping-add-u32 off6 e7.ev-consumed in
        let ok-e7 = eq-u8 e7.ev-next-state h1-state-reading-trailers in
        -- Step 8: Trailers "X-Done: yes\r\n\r\n"
        let e8 = h1-conn-step-dma conn buf off7 pipe-len 0 hdrs req-slot in
        let off8 = wrapping-add-u32 off7 e8.ev-consumed in
        let ok-e8 = eq-u8 e8.ev-kind h1-ev-request-complete && eq-u8 e8.ev-next-state h1-state-reading-head in
        -- Step 9: Upgrade request GET /ws
        let e9 = h1-conn-step-dma conn buf off8 pipe-len 0 hdrs req-slot in
        let off9 = wrapping-add-u32 off8 e9.ev-consumed in
        let c-pre-upg = h1-conn-load-dma conn in
        let c-upg = h1-conn-accept-upgrade c-pre-upg in
        let ok-e9 = (
          eq-u8 e9.ev-kind h1-ev-request-head
            && eq-u32 off9 pipe-len
            && eq-u32 c-pre-upg.conn-requests-parsed 3
            && eq-u16 c-pre-upg.conn-in-flight 3
            && h1-conn-upgrade-requested c-pre-upg
            && eq-u8 c-upg.conn-state h1-state-upgraded) in

        -- === Part D: HTTP/1.0 keep-alive default (close unless Connection: keep-alive) ===
        let n10a = copy-str-to-dma "GET /old HTTP/1.0\r\n\r\n" buf in
        let rv10a = h1-parse-request-dma buf 0 n10a 0 hdrs in
        let n10b = copy-str-to-dma "GET /old HTTP/1.0\r\nConnection: keep-alive\r\n\r\n" buf in
        let rv10b = h1-parse-request-dma buf 0 n10b 0 hdrs in
        let n11c = copy-str-to-dma "GET /bye HTTP/1.1\r\nHost: a\r\nConnection: close\r\n\r\n" buf in
        let rv11c = h1-parse-request-dma buf 0 n11c 0 hdrs in
        let ok-ka = (
          eq-u8 rv10a.rv-status h1-ok && not (request-view-has-flag rv10a req-flag-keep-alive)
            && eq-u8 rv10b.rv-status h1-ok && request-view-has-flag rv10b req-flag-keep-alive
            && eq-u8 rv11c.rv-status h1-ok && not (request-view-has-flag rv11c req-flag-keep-alive)) in

        -- === Part E: Response status-line parity + 100-continue + 101-upgrade + chunked frame ===
        let ok-st = (
          check-status-match tx 200 && check-status-match tx 201 && check-status-match tx 204
            && check-status-match tx 206 && check-status-match tx 301 && check-status-match tx 304
            && check-status-match tx 400 && check-status-match tx 401 && check-status-match tx 403
            && check-status-match tx 404 && check-status-match tx 405 && check-status-match tx 413
            && check-status-match tx 429 && check-status-match tx 500 && check-status-match tx 502
            && check-status-match tx 503 && check-status-match tx 101) in
        let n100 = h1-write-continue-dma tx 0 128 in
        let ok100 = dma-eq-str tx 0 n100 "HTTP/1.1 100 Continue\r\n\r\n" in
        let proto-len = copy-str-to-dma "websocket" buf in
        let n101 = h1-write-upgrade-dma tx 0 256 buf proto-len in
        let ok101 = dma-eq-str tx 0 n101 "HTTP/1.1 101 Switching Protocols\r\nConnection: Upgrade\r\nUpgrade: websocket\r\n\r\n" in
        let _ = engine-dma-free dma 32768 in
        if not (ok-inc && ok-spans) then 20
        else if not (ok-e1 && ok-e2 && ok-e3 && ok-e4 && ok-e5 && ok-e6 && ok-e7 && ok-e8 && ok-e9) then 21
        else if not ok-ka then 22
        else if not (ok-st && ok100 && ok101) then 23
        else 0
"#;
    build_and_run_leak_checked("rfc9112-corpus", src, 0);
}

// =========================================================================
// 4. 1,000-trial Effect.Test.Hegel differential/fuzz test over parser +
//    connection state machine + M5 Http1Machine oracle
// =========================================================================

#[test]
fn hegel_1000_trial_differential_parser_and_conn_state_machine() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Test.Hegel
  import Engine.Types
  import Net.Http.Types
  import Net.Http.V1.Parser
  import Net.Http.V1.Conn
  import Net.Http.V1.Spec
  import Prim.Atomic

  -- Differential state pairing formal M5 `H1ModelState` with concrete `H1Conn`.
  record DiffState where
    ds-spec :: H1ModelState
    ds-conn :: H1Conn

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

  -- Map ReqInput to real RFC 9112 wire bytes (valid or smuggling attack).
  req-wire :: ReqInput -> String
  req-wire inp = case inp of
    ReqGetKa -> "GET /ok HTTP/1.1\r\nHost: diff.local\r\n\r\n"
    ReqGetClose -> "GET /bye HTTP/1.1\r\nHost: diff.local\r\nConnection: close\r\n\r\n"
    ReqFixedBody -> "POST /fix HTTP/1.1\r\nHost: diff.local\r\nContent-Length: 2\r\n\r\n"
    ReqChunkedBody -> "POST /chk HTTP/1.1\r\nHost: diff.local\r\nTransfer-Encoding: chunked\r\n\r\n"
    ReqSmuggleTeCl -> "POST /smug HTTP/1.1\r\nHost: diff.local\r\nContent-Length: 4\r\nTransfer-Encoding: chunked\r\n\r\n"
    ReqMalformed -> "GET /bad HTTP/1.1\r\nHost : diff.local\r\n\r\n"

  -- Concrete step mirroring `h1-step-recv-head` using the real SIMD parser + H1Conn.
  diff-recv-head :: I64 -> I64 -> ReqInput -> DiffState ->{MVar} Option DiffState
  diff-recv-head buf hdrs inp ds =
    case h1-step-recv-head inp ds.ds-spec of
      None -> None
      Some next-spec ->
        let n = copy-str-to-dma (req-wire inp) buf in
        let rv = h1-parse-request-dma buf 0 n 0 hdrs in
        let next-conn = h1-conn-on-head ds.ds-conn rv in
        Some (DiffState next-spec next-conn)

  -- Concrete step mirroring `h1-step-body` using real chunk/trailer parsers + H1Conn.
  diff-body-step :: I64 -> DiffState ->{MVar} Option DiffState
  diff-body-step buf ds =
    case h1-step-body ds.ds-spec of
      None -> None
      Some next-spec ->
        let c = ds.ds-conn in
        let next-conn = (
          if eq-u8 c.conn-state h1-state-reading-fixed-body then
            h1-conn-on-fixed-body c 1
          else if eq-u8 c.conn-state h1-state-reading-chunk-size then
            let wire = if ds.ds-spec.body-rem > 0 then "1\r\n" else "0\r\n" in
            let n = copy-str-to-dma wire buf in
            h1-conn-on-chunk-size c (h1-parse-chunk-size buf 0 n)
          else if eq-u8 c.conn-state h1-state-reading-chunk-data then
            h1-conn-on-chunk-data c 1
          else if eq-u8 c.conn-state h1-state-reading-chunk-crlf then
            let n = copy-str-to-dma "\r\n" buf in
            h1-conn-on-chunk-crlf c (h1-parse-chunk-crlf buf 0 n)
          else
            let n = copy-str-to-dma "\r\n" buf in
            h1-conn-on-trailers c (h1-parse-trailers buf 0 n 8)) in
        Some (DiffState next-spec next-conn)

  -- Concrete step mirroring `h1-step-body-fault` using real malformed chunk/CRLF/trailer inputs.
  diff-body-fault :: I64 -> DiffState ->{MVar} Option DiffState
  diff-body-fault buf ds =
    case h1-step-body-fault ds.ds-spec of
      None -> None
      Some next-spec ->
        let c = ds.ds-conn in
        let next-conn = (
          if eq-u8 c.conn-state h1-state-reading-chunk-size then
            let n = copy-str-to-dma "ZZZ\r\n" buf in
            h1-conn-on-chunk-size c (h1-parse-chunk-size buf 0 n)
          else if eq-u8 c.conn-state h1-state-reading-chunk-crlf then
            let n = copy-str-to-dma "XX" buf in
            h1-conn-on-chunk-crlf c (h1-parse-chunk-crlf buf 0 n)
          else
            let n = copy-str-to-dma "Content-Length: 5\r\n\r\n" buf in
            h1-conn-on-trailers c (h1-parse-trailers buf 0 n 8)) in
        Some (DiffState next-spec next-conn)

  diff-send-resp :: DiffState -> Option DiffState
  diff-send-resp ds =
    case h1-step-send-response ds.ds-spec of
      None -> None
      Some next-spec -> Some (DiffState next-spec (h1-conn-on-response-sent ds.ds-conn))

  diff-close :: DiffState -> Option DiffState
  diff-close ds =
    case h1-step-close ds.ds-spec of
      None -> None
      Some next-spec -> Some (DiffState next-spec (h1-conn-on-eof ds.ds-conn))

  phase-matches-conn :: H1Phase -> U8 -> Bool
  phase-matches-conn ph st = case ph of
    MReadingHead -> eq-u8 st h1-state-reading-head
    MReadingFixedBody -> eq-u8 st h1-state-reading-fixed-body
    MReadingChunkSize -> eq-u8 st h1-state-reading-chunk-size
    MReadingChunkData -> eq-u8 st h1-state-reading-chunk-data
    MReadingChunkCrlf -> eq-u8 st h1-state-reading-chunk-crlf
    MReadingTrailers -> eq-u8 st h1-state-reading-trailers
    MClosing -> eq-u8 st h1-state-closing
    MClosed -> eq-u8 st h1-state-closing
    MProtocolError -> eq-u8 st h1-state-protocol-error

  diff-invariant :: DiffState -> Bool
  diff-invariant ds =
    let s = ds.ds-spec in
    let c = ds.ds-conn in
    h1-safety-ok s
      && phase-matches-conn s.phase c.conn-state
      && u32-to-nat c.conn-requests-parsed == s.requests-parsed
      && u32-to-nat c.conn-responses-sent == s.responses-sent
      && u16-to-nat c.conn-in-flight == s.in-flight
      && (not (h1-phase-eq s.phase MClosed) || h1-conn-can-close c)

  -- Pure generator of an action opcode sequence (`0..9`) for 1,000 Hegel trials.
  gen-req-input :: Gen ReqInput
  gen-req-input = gen-one-of [
    gen-pure ReqGetKa,
    gen-pure ReqGetClose,
    gen-pure ReqFixedBody,
    gen-pure ReqChunkedBody,
    gen-pure ReqSmuggleTeCl,
    gen-pure ReqMalformed
  ]

  -- Run 1,000-trial Hegel machine check on M5 `Http1Machine` first, AND run
  -- 1,000 randomized differential traces driving the real DMA SIMD parser + H1Conn!
  run-diff-step-code :: I64 -> I64 -> Nat -> DiffState ->{MVar} DiffState
  run-diff-step-code buf hdrs op ds =
    let opt = (
      if op == 0 then diff-recv-head buf hdrs ReqGetKa ds
      else if op == 1 then diff-recv-head buf hdrs ReqGetClose ds
      else if op == 2 then diff-recv-head buf hdrs ReqFixedBody ds
      else if op == 3 then diff-recv-head buf hdrs ReqChunkedBody ds
      else if op == 4 then diff-recv-head buf hdrs ReqSmuggleTeCl ds
      else if op == 5 then diff-recv-head buf hdrs ReqMalformed ds
      else if op == 6 then diff-body-step buf ds
      else if op == 7 then diff-body-fault buf ds
      else if op == 8 then diff-send-resp ds
      else diff-close ds) in
    case opt of
      Some next -> next
      None -> ds

  run-diff-trace :: I64 -> I64 -> U64 -> Nat -> DiffState ->{MVar} Bool
  run-diff-trace buf hdrs rng steps ds =
    if not (diff-invariant ds) then False
    else if steps == 0 then True
    else
      let rng1 = wrapping-add-u64 (wrapping-mul-u64 rng 6364136223846793005) 1442695040888963407 in
      let op = u32-to-nat (wrapping-u64-to-u32 (rem-u64 (wrapping-shr-u64 rng1 33) 10)) in
      let ds1 = run-diff-step-code buf hdrs op ds in
      run-diff-trace buf hdrs rng1 (steps - 1) ds1

  run-diff-trials :: I64 -> I64 -> U64 -> U64 ->{MVar} Bool
  run-diff-trials buf hdrs trial total =
    if ge-u64 trial total then True
    else
      let seed = wrapping-mul-u64 (wrapping-add-u64 trial 1) 0x9E3779B97F4A7C15 in
      let init-ds = DiffState h1-model-init (h1-conn-init 2) in
      if run-diff-trace buf hdrs seed 24 init-ds then
        run-diff-trials buf hdrs (wrapping-add-u64 trial 1) total
      else False

  main :: Unit ->{IO, MVar} Int
  main _ =
    -- 1. 1,000-trial Effect.Test.Hegel machine check over M5 Http1Machine
    let cfg = HegelConfig 1000 256 60 (hegel-seed 9112) in
    let acts = [
      step-action-gen "recv-head" gen-req-input h1-step-recv-head,
      step-action "body" h1-step-body,
      step-action "body-fault" h1-step-body-fault,
      step-action "send-response" h1-step-send-response,
      step-action "close" h1-step-close
    ] in
    case hegel-check-machine-with cfg 32 h1-model-init acts h1-safety-ok of
      Err _ -> 1
      Ok _ ->
        -- 2. 1,000-trial differential trace over live DMA SIMD parser + H1Conn vs M5 oracle
        let dma = engine-dma-alloc 8192 4096 in
        if eq-u64 dma 0 then 2
        else
          let buf = wrapping-u64-to-i64 dma in
          let hdrs = wrapping-add-i64 buf 4096 in
          let ok = run-diff-trials buf hdrs 0 1000 in
          let _ = engine-dma-free dma 8192 in
          if ok then 0 else 3
"#;
    build_and_run_leak_checked("hegel-1000-diff", src, 0);
}

// =========================================================================
// 5. Live macOS loopback plaintext HTTP/1.1 pipelined keep-alive server +
//    50,000-request steady-state parse+serialize zero-alloc hot loop
// =========================================================================

#[cfg(target_os = "macos")]
#[test]
fn live_macos_loopback_http1_pipelined_server_and_50000_req_zero_alloc_hot_loop() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Engine.Types
  import Engine.Timer
  import Engine.Reactor.Darwin
  import Net.Http.Types
  import Net.Http.V1.Parser
  import Net.Http.V1.Conn
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

  recv-exact :: DarwinReactor -> Fd -> I64 -> I64 -> U32 -> U32 -> U32 ->{IO, MVar} I32
  recv-exact rx fd buf cqe-buf got want budget =
    if ge-u32 got want then wrapping-u32-to-i32 got
    else if eq-u32 budget 0 then -60
    else
      let rem = wrapping-sub-u32 want got in
      let nr = darwin-recv-dma fd (wrapping-add-i64 buf (u32-to-i64 got)) rem in
      if gt-i32 nr 0 then
        recv-exact rx fd buf cqe-buf (wrapping-add-u32 got (wrapping-i32-to-u32 nr)) want budget
      else if eq-i32 nr 0 then 0
      else if eq-i32 nr -35 || eq-i32 nr -4 then
        let _ = darwin-reactor-poll rx 100000000 cqe-buf 16 in
        recv-exact rx fd buf cqe-buf got want (wrapping-sub-u32 budget 1)
      else nr

  accept-loop :: DarwinReactor -> Fd -> I64 -> U32 ->{IO, MVar} Fd
  accept-loop rx lfd cqe-buf budget =
    if eq-u32 budget 0 then fd-invalid ()
    else
      case darwin-tcp-accept-nonblock lfd of
        Ok cfd -> cfd
        Err IoAgain ->
          let _ = darwin-reactor-poll rx 100000000 cqe-buf 16 in
          accept-loop rx lfd cqe-buf (wrapping-sub-u32 budget 1)
        Err _ -> fd-invalid ()

  -- Drain pipelined requests in `rx-buf[off..len)` on `conn-addr`, coalescing
  -- responses into `tx-buf` with zero heap allocations!
  #[no-alloc]
  process-pipelined-requests
    :: I64 -> I64 -> U32 -> U32 -> I64 -> I64 -> I64 -> U32 -> U32 -> CoarseClockCache -> I64 -> U32 -> I64 -> U32 ->{MVar} U32
  process-pipelined-requests conn-addr rx-buf off len hdrs req-slot tx-buf tx-pos tx-cap clk ct-addr ct-len body-addr body-len =
    if ge-u32 off len then tx-pos
    else
      let ev = h1-conn-step-dma conn-addr rx-buf off len 0 hdrs req-slot in
      if eq-u8 ev.ev-kind h1-ev-request-head then
        let c = h1-conn-load-dma conn-addr in
        let ka = h1-conn-is-keep-alive c in
        let next-tx = h1-serialize-response-dma tx-buf tx-pos tx-cap (status-ok ()) clk ka ct-addr ct-len body-addr body-len in
        let _upd = h1-conn-store-dma conn-addr (h1-conn-on-response-sent c) in
        process-pipelined-requests conn-addr rx-buf (wrapping-add-u32 off ev.ev-consumed) len hdrs req-slot tx-buf next-tx tx-cap clk ct-addr ct-len body-addr body-len
      else tx-pos

  -- 50,000-request steady-state parse + conn-step + coalesced DMA response serialize hot loop.
  -- MUST execute with 0 heap allocations and 0 atomic RC operations!
  #[no-alloc]
  hot-loop50000
    :: I64 -> I64 -> U32 -> I64 -> I64 -> I64 -> I64 -> CoarseClockCache -> I64 -> U32 -> I64 -> U32 -> U32 -> U32 ->{MVar} U32
  hot-loop50000 conn-addr rx-buf rx-len hdrs req-slot tx-buf iov-buf clk ct-addr ct-len body-addr body-len i limit =
    if ge-u32 i limit then limit
    else
      let ev = h1-conn-step-dma conn-addr rx-buf 0 rx-len 0 hdrs req-slot in
      if ne-u8 ev.ev-kind h1-ev-request-head then 0
      else
        let c = h1-conn-load-dma conn-addr in
        let head-len = h1-serialize-response-head-dma tx-buf 0 1024 (status-ok ()) clk (eq-u32 0 0) (eq-u32 0 1) (u32-to-u64 body-len) ct-addr ct-len in
        let niov = h1-build-response-iovec iov-buf 0 tx-buf head-len body-addr body-len in
        let _u = h1-conn-store-dma conn-addr (h1-conn-on-response-sent c) in
        if ne-u32 niov 2 then 0
        else hot-loop50000 conn-addr rx-buf rx-len hdrs req-slot tx-buf iov-buf clk ct-addr ct-len body-addr body-len (wrapping-add-u32 i 1) limit

  main :: Unit ->{IO, MVar} Int
  main _ =
    case coarse-clock-new () of
      None -> 1
      Some clk ->
        let _rf = coarse-date-refresh clk 1764338400 in -- Fri, 28 Nov 2025 14:00:00 GMT
        let dma = engine-dma-alloc 32768 4096 in
        if eq-u64 dma 0 then
          let _ = coarse-clock-destroy clk in 2
        else
          let slab = wrapping-u64-to-i64 dma in
          -- DMA layout:
          --   +0     : client tx / pipelined requests (4 KiB)
          --   +4096  : server rx buffer (4 KiB)
          --   +8192  : server tx coalesced response buffer (4 KiB)
          --   +12288 : client rx buffer (4 KiB)
          --   +16384 : hdrs array (64 * 20B = 1280B)
          --   +18432 : conn slot (64B)
          --   +18560 : req-view slot (80B)
          --   +18688 : iovec[4] (64B)
          --   +18816 : content-type "text/plain"
          --   +18944 : body "pong!"
          --   +20480 : cqe-buf (256B)
          let cli-tx = slab in
          let srv-rx = wrapping-add-i64 slab 4096 in
          let srv-tx = wrapping-add-i64 slab 8192 in
          let cli-rx = wrapping-add-i64 slab 12288 in
          let hdrs = wrapping-add-i64 slab 16384 in
          let conn-addr = wrapping-add-i64 slab 18432 in
          let req-slot = wrapping-add-i64 slab 18560 in
          let iov-buf = wrapping-add-i64 slab 18688 in
          let ct-addr = wrapping-add-i64 slab 18816 in
          let body-addr = wrapping-add-i64 slab 18944 in
          let cqe-buf = wrapping-add-i64 slab 20480 in
          let ct-len = copy-str-to-dma "text/plain" ct-addr in
          let body-len = copy-str-to-dma "pong!" body-addr in

          -- === Part 1: Live macOS loopback HTTP/1.1 server handling 3 pipelined keep-alive requests ===
          case darwin-reactor-new 128 of
            Err _ ->
              let _ = engine-dma-free dma 32768 in
              let _ = coarse-clock-destroy clk in 3
            Ok rx ->
              case darwin-tcp-listen-reuseport 0 32 of
                Err _ ->
                  let _ = darwin-reactor-destroy rx in
                  let _ = engine-dma-free dma 32768 in
                  let _ = coarse-clock-destroy clk in 4
                Ok (lfd, port) ->
                  let _al = darwin-reactor-arm-read rx lfd (io-token-pack op-code-accept 1 1) in
                  case darwin-tcp-connect-nonblock ipv4-loopback port of
                    Err _ ->
                      let _ = darwin-close-fd lfd in
                      let _ = darwin-reactor-destroy rx in
                      let _ = engine-dma-free dma 32768 in
                      let _ = coarse-clock-destroy clk in 5
                    Ok cli ->
                      let _p0 = darwin-reactor-poll rx 500000000 cqe-buf 16 in
                      let srv = accept-loop rx lfd cqe-buf 64 in
                      if not (fd-is-valid srv) then
                        let _ = darwin-close-fd cli in
                        let _ = darwin-close-fd lfd in
                        let _ = darwin-reactor-destroy rx in
                        let _ = engine-dma-free dma 32768 in
                        let _ = coarse-clock-destroy clk in 6
                      else
                        let _a1 = darwin-reactor-arm-read rx srv (io-token-pack op-code-recv 1 2) in
                        let _a2 = darwin-reactor-arm-read rx cli (io-token-pack op-code-recv 1 3) in
                        let _p1 = darwin-reactor-poll rx 0 cqe-buf 16 in
                        -- Client pipelines 3 HTTP/1.1 keep-alive requests in one write
                        let req3-len = copy-str-to-dma "GET /p1 HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\nGET /p2 HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\nGET /p3 HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n" cli-tx in
                        let nw-cli = darwin-send-dma cli cli-tx req3-len in
                        let nr-srv = recv-exact rx srv srv-rx cqe-buf 0 req3-len 64 in
                        let _ci = h1-conn-slot-init conn-addr 8 in
                        -- Server parses all 3 pipelined requests & coalesces 3 responses into `srv-tx`
                        let resp-total = process-pipelined-requests conn-addr srv-rx 0 req3-len hdrs req-slot srv-tx 0 4096 clk ct-addr ct-len body-addr body-len in
                        let c-after = h1-conn-load-dma conn-addr in
                        -- Flush coalesced responses with single `darwin-writev-dma` syscall
                        let _iv = h1-iovec-set iov-buf 0 srv-tx (u32-to-u64 resp-total) in
                        let nw-srv = darwin-writev-dma srv iov-buf 1 in
                        -- Client reads exact coalesced response bytes
                        let nr-cli = recv-exact rx cli cli-rx cqe-buf 0 resp-total 64 in
                        -- Verify first 17 bytes are "HTTP/1.1 200 OK\r\n" and last 5 bytes are "pong!"
                        let live-ok = (
                          eq-i32 nw-cli (wrapping-u32-to-i32 req3-len)
                            && eq-i32 nr-srv (wrapping-u32-to-i32 req3-len)
                            && eq-u32 c-after.conn-requests-parsed 3
                            && eq-u32 c-after.conn-responses-sent 3
                            && eq-u16 c-after.conn-in-flight 0
                            && h1-conn-can-close c-after
                            && eq-i32 nw-srv (wrapping-u32-to-i32 resp-total)
                            && eq-i32 nr-cli (wrapping-u32-to-i32 resp-total)
                            && eq-u8 (raw-load-u8 cli-rx) 72
                            && eq-u8 (raw-load-u8 (wrapping-add-i64 cli-rx 9)) 50
                            && eq-u8 (raw-load-u8 (wrapping-add-i64 cli-rx (wrapping-sub-i64 (u32-to-i64 resp-total) 1))) 33) in
                        let _c1 = darwin-close-fd srv in
                        let _c2 = darwin-close-fd cli in
                        let _c3 = darwin-close-fd lfd in
                        let _d = darwin-reactor-destroy rx in

                        -- === Part 2: 50,000-request steady-state parse+serialize zero-alloc hot loop ===
                        let hot-req-len = copy-str-to-dma "GET /bench?k=42 HTTP/1.1\r\nHost: bench.cinch.dev\r\nUser-Agent: wrk/4.2\r\nAccept: */*\r\n\r\n" srv-rx in
                        let _ci2 = h1-conn-slot-init conn-addr 16 in
                        let _rst = engine-thread-counters-reset () in
                        let done = hot-loop50000 conn-addr srv-rx hot-req-len hdrs req-slot srv-tx iov-buf clk ct-addr ct-len body-addr body-len 0 50000 in
                        let allocs = engine-thread-alloc-count () in
                        let atomic-rcs = engine-thread-atomic-rc-count () in
                        let c-hot = h1-conn-load-dma conn-addr in
                        let _f = engine-dma-free dma 32768 in
                        let _dc = coarse-clock-destroy clk in
                        if not live-ok then 10
                        else if ne-u32 done 50000 then 11
                        else if ne-u32 c-hot.conn-requests-parsed 50000 || ne-u32 c-hot.conn-responses-sent 50000 then 12
                        else if ne-u64 allocs 0 then 13
                        else if ne-u64 atomic-rcs 0 then 14
                        else 0
"#;
    build_and_run_leak_checked("live-loopback-and-50000-hot", src, 0);
}
