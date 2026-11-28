//! Integration, RFC 7541 / RFC 9204 / RFC 9218 compliance, ABI layout,
//! `#[no-alloc]`, linear `%1` `ResponseWriter` compile-fail, and 50,000-op
//! zero-allocation / zero-atomic-RC / leak-checked tests for `Net.Http.Types`
//! and `Net.Http.Priority`.
//!
//! Routed through `common::project` / `common::drive_with_profile` /
//! `common::check_interfaces` / `common::build_executable` /
//! `common::check_diagnostics` so the exact same suite runs against both the
//! Rust seed and the Stage 3 self-hosted compiler.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{Mode, Package};
use std::process::Command;

const NET_HTTP_TYPES: &str = include_str!("../prelude/Net/Http/Types.cinch");
const NET_HTTP_PRIORITY: &str = include_str!("../prelude/Net/Http/Priority.cinch");
const ENGINE_TYPES: &str = include_str!("../prelude/Engine/Types.cinch");
const PRIM_ATOMIC: &str = include_str!("../prelude/Prim/Atomic.cinch");
const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn project_with_http_core(tag: &str, main_src: &str) -> std::path::PathBuf {
    common::project(
        tag,
        &[
            ("Prim.Atomic.cinch", PRIM_ATOMIC),
            ("Engine.Types.cinch", ENGINE_TYPES),
            ("Net.Http.Types.cinch", NET_HTTP_TYPES),
            ("Net.Http.Priority.cinch", NET_HTTP_PRIORITY),
            ("Main.cinch", main_src),
        ],
    )
}

fn run_leak_checked(tag: &str, src: &str, expected: i32) {
    let dir = project_with_http_core(tag, src);
    let exe = common::build_executable(&dir, "Main.cinch", "net-http-core-test")
        .unwrap_or_else(|e| panic!("{tag}: build failed:\n{e}"));
    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "64M")
        .output()
        .expect("run built binary");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(
        out.status.code(),
        Some(expected),
        "{tag}: expected exit {expected}, got {:?};\nstdout:\n{stdout}\nstderr:\n{stderr}",
        out.status.code()
    );
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "{tag}: expected clean leak report (`leaked=0`); stderr:\n{stderr}"
    );
    common::cleanup(&dir);
}

// =========================================================================
// 1. Package manifest registration & direct stdlib resolution
// =========================================================================

#[test]
fn net_http_core_registered_in_stdlib_package_manifest() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, path) in [
        ("Net.Http.Types", "Net/Http/Types.cinch"),
        ("Net.Http.Priority", "Net/Http/Priority.cinch"),
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

    // Direct stdlib import without copying sources into the project directory.
    let dir = common::project(
        "http-core-stdlib-resolve",
        &[(
            "Main.cinch",
            r#"
module Main where
  import Data.Machine
  import Data.Option
  import Net.Http.Types
  import Net.Http.Priority

  main :: Unit ->{IO, MVar} Int
  main _ =
    let st = status-ok () in
    let p = stream-priority-default () in
    case prio-sched-new 8 of
      None -> 1
      Some sc ->
        let _ = prio-sched-destroy sc in
        if eq-u16 st.status-code 200 && eq-u8 p.prio-urgency 3 then 0 else 2
"#,
        )],
    );
    let code = common::drive_with_profile(
        Mode::Run,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        None,
        &TargetProfile::native(),
    )
    .expect("direct stdlib resolution of Net.Http.Types & Net.Http.Priority");
    common::cleanup(&dir);
    assert_eq!(code, 0);
}

// =========================================================================
// 2. ABI layouts & #[no-alloc] interface assertions
// =========================================================================

#[test]
fn net_http_core_abi_layouts_and_no_alloc_assertions() {
    let src = r#"
module Main where
  import Data.Machine
  import Net.Http.Types
  import Net.Http.Priority

  main :: Int
  main =
    let hp = header-pair-empty () in
    let rh = req-head-empty () in
    let sp = stream-priority-default () in
    if eq-u8 hp.hp-known hdr-unknown && eq-u8 rh.req-version 1 && eq-u8 sp.prio-urgency 3 then 0 else 1
"#;

    let dir = project_with_http_core("http-core-abi", src);
    let ifaces = common::check_interfaces(&dir, "Main.cinch").expect("check_interfaces");

    let types_iface = ifaces
        .iter()
        .find(|i| i.module == "Net.Http.Types")
        .expect("Net.Http.Types interface emitted");
    let prio_iface = ifaces
        .iter()
        .find(|i| i.module == "Net.Http.Priority")
        .expect("Net.Http.Priority interface emitted");

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

    // HttpStatus == 2B POD
    let st_l = layout_of(types_iface, "HttpStatus");
    assert_eq!(st_l.size, 2, "HttpStatus layout: {st_l:?}");
    assert_eq!(st_l.align, 2, "HttpStatus align: {st_l:?}");

    // HeaderPair == 20B POD
    let hp_l = layout_of(types_iface, "HeaderPair");
    assert_eq!(hp_l.size, 20, "HeaderPair layout: {hp_l:?}");
    assert_eq!(hp_l.align, 4, "HeaderPair align: {hp_l:?}");
    let hp_offsets: Vec<u64> = hp_l.fields.iter().map(|f| f.offset).collect();
    assert_eq!(hp_offsets, vec![0, 1, 2, 4, 8, 12, 16]);

    // RequestHead == 48B POD
    let rh_l = layout_of(types_iface, "RequestHead");
    assert_eq!(rh_l.size, 48, "RequestHead layout: {rh_l:?}");
    assert_eq!(rh_l.align, 8, "RequestHead align: {rh_l:?}");
    let rh_offsets: Vec<u64> = rh_l.fields.iter().map(|f| f.offset).collect();
    assert_eq!(rh_offsets, vec![0, 1, 2, 4, 8, 12, 16, 20, 24, 32, 40]);

    // StreamPriority == 4B POD
    let sp_l = layout_of(prio_iface, "StreamPriority");
    assert_eq!(sp_l.size, 4, "StreamPriority layout: {sp_l:?}");
    assert_eq!(sp_l.align, 2, "StreamPriority align: {sp_l:?}");
    let sp_offsets: Vec<u64> = sp_l.fields.iter().map(|f| f.offset).collect();
    assert_eq!(sp_offsets, vec![0, 1, 2]);

    // Verify #[no-alloc] facts on hot-path functions across both modules.
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

    for f in [
        "http-method-to-u8",
        "http-method-name",
        "http-version-to-u8",
        "http-version-name",
        "status-ok",
        "http-status-line",
        "known-header-bit",
        "known-header-hpack-index",
        "known-header-qpack-index",
        "known-header-wire-name",
        "known-header-match-dma",
        "known-header-match-bytes",
        "req-has-header",
        "req-has-flag",
    ] {
        assert_no_alloc(types_iface, f);
    }

    for f in [
        "stream-priority-default",
        "stream-priority-make-raw",
        "parse-priority-header-dma",
        "prio-sched-enqueue",
        "prio-sched-remove",
        "prio-sched-reprioritize",
        "prio-sched-next",
        "prio-sched-active-count",
    ] {
        assert_no_alloc(prio_iface, f);
    }

    assert_eq!(common::run(&dir, "Main.cinch"), 0);
    common::cleanup(&dir);
}

// =========================================================================
// 3. All 61 KnownHeaders (RFC 7541 HPACK & RFC 9204 QPACK), case-insensitive
//    matching, bitmask uniqueness, status lines, and ResponseWriter output
// =========================================================================

#[test]
fn net_http_known_headers_rfc7541_rfc9204_matching_and_response_writer() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.String
  import Engine.Types
  import Net.Http.Types
  import Prim.Atomic

  -- Copy Bytes into DMA buffer at `base`.
  copy-bytes-to-dma :: Bytes -> I64 -> Nat ->{MVar} Unit
  copy-bytes-to-dma b base i =
    if i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 base (nat-to-i64 i)) (bytes-get-u8 b i) in
      copy-bytes-to-dma b base (i + 1)
    else ()

  -- Copy DMA [base, base + len) into a fresh Bytes buffer for exact comparison.
  copy-dma-to-bytes-loop :: I64 -> Bytes -> Nat -> Nat ->{MVar} Unit
  copy-dma-to-bytes-loop base dst i len =
    if i < len && i < bytes-len dst then
      let v = raw-load-u8 (wrapping-add-i64 base (nat-to-i64 i)) in
      let _ = bytes-set-u8 dst i v in
      copy-dma-to-bytes-loop base dst (i + 1) len
    else ()

  dma-to-string :: I64 -> U32 ->{MVar} String
  dma-to-string base len =
    let n = u32-to-nat len in
    let b = bytes-new n in
    let _ = copy-dma-to-bytes-loop base b 0 n in
    case string-from-bytes b of
      Some s -> s
      None -> ""

  -- Transform Bytes ASCII letters: mode 1 = UPPERCASE, mode 2 = alternating MiXeD.
  transform-case :: Bytes -> Nat -> U8 -> Unit
  transform-case b i mode =
    if i < bytes-len b then
      let c = bytes-get-u8 b i in
      let upper = if ge-u8 c 97 && le-u8 c 122 then wrapping-sub-u8 c 32 else c in
      let out = if eq-u8 mode 1 then upper else if nat-rem i 2 == 0 then upper else c in
      let _ = bytes-set-u8 b i out in
      transform-case b (i + 1) mode
    else ()

  -- Exact RFC 7541 Appendix A HPACK static table index verification for every KnownHeader.
  exp-hpack :: U8 -> U8
  exp-hpack id =
    if eq-u8 id hdr-authority then 1
    else if eq-u8 id hdr-method then 2
    else if eq-u8 id hdr-path then 4
    else if eq-u8 id hdr-scheme then 6
    else if eq-u8 id hdr-status then 8
    else if eq-u8 id hdr-accept then 19
    else if eq-u8 id hdr-accept-charset then 15
    else if eq-u8 id hdr-accept-encoding then 16
    else if eq-u8 id hdr-accept-language then 17
    else if eq-u8 id hdr-accept-ranges then 18
    else if eq-u8 id hdr-access-control-allow-origin then 20
    else if eq-u8 id hdr-age then 21
    else if eq-u8 id hdr-allow then 22
    else if eq-u8 id hdr-authorization then 23
    else if eq-u8 id hdr-cache-control then 24
    else if eq-u8 id hdr-connection then 0
    else if eq-u8 id hdr-content-disposition then 25
    else if eq-u8 id hdr-content-encoding then 26
    else if eq-u8 id hdr-content-language then 27
    else if eq-u8 id hdr-content-length then 28
    else if eq-u8 id hdr-content-location then 29
    else if eq-u8 id hdr-content-range then 30
    else if eq-u8 id hdr-content-type then 31
    else if eq-u8 id hdr-cookie then 32
    else if eq-u8 id hdr-date then 33
    else if eq-u8 id hdr-etag then 34
    else if eq-u8 id hdr-expect then 35
    else if eq-u8 id hdr-expires then 36
    else if eq-u8 id hdr-from then 37
    else if eq-u8 id hdr-host then 38
    else if eq-u8 id hdr-if-match then 39
    else if eq-u8 id hdr-if-modified-since then 40
    else if eq-u8 id hdr-if-none-match then 41
    else if eq-u8 id hdr-if-range then 42
    else if eq-u8 id hdr-if-unmodified-since then 43
    else if eq-u8 id hdr-last-modified then 44
    else if eq-u8 id hdr-link then 45
    else if eq-u8 id hdr-location then 46
    else if eq-u8 id hdr-max-forwards then 47
    else if eq-u8 id hdr-origin then 0
    else if eq-u8 id hdr-priority then 0
    else if eq-u8 id hdr-proxy-authenticate then 48
    else if eq-u8 id hdr-proxy-authorization then 49
    else if eq-u8 id hdr-range then 50
    else if eq-u8 id hdr-referer then 51
    else if eq-u8 id hdr-refresh then 52
    else if eq-u8 id hdr-retry-after then 53
    else if eq-u8 id hdr-sec-websocket-accept then 0
    else if eq-u8 id hdr-sec-websocket-key then 0
    else if eq-u8 id hdr-sec-websocket-version then 0
    else if eq-u8 id hdr-server then 54
    else if eq-u8 id hdr-set-cookie then 55
    else if eq-u8 id hdr-strict-transport-security then 56
    else if eq-u8 id hdr-te then 0
    else if eq-u8 id hdr-transfer-encoding then 57
    else if eq-u8 id hdr-upgrade then 0
    else if eq-u8 id hdr-user-agent then 58
    else if eq-u8 id hdr-vary then 59
    else if eq-u8 id hdr-via then 60
    else if eq-u8 id hdr-www-authenticate then 61
    else if eq-u8 id hdr-alt-svc then 0
    else 0

  -- Exact RFC 9204 Appendix A QPACK static table index verification for every KnownHeader.
  exp-qpack :: U8 -> U8
  exp-qpack id =
    if eq-u8 id hdr-authority then 0
    else if eq-u8 id hdr-method then 15
    else if eq-u8 id hdr-path then 1
    else if eq-u8 id hdr-scheme then 22
    else if eq-u8 id hdr-status then 24
    else if eq-u8 id hdr-accept then 29
    else if eq-u8 id hdr-accept-charset then 255
    else if eq-u8 id hdr-accept-encoding then 31
    else if eq-u8 id hdr-accept-language then 72
    else if eq-u8 id hdr-accept-ranges then 32
    else if eq-u8 id hdr-access-control-allow-origin then 35
    else if eq-u8 id hdr-age then 2
    else if eq-u8 id hdr-allow then 255
    else if eq-u8 id hdr-authorization then 84
    else if eq-u8 id hdr-cache-control then 36
    else if eq-u8 id hdr-connection then 255
    else if eq-u8 id hdr-content-disposition then 3
    else if eq-u8 id hdr-content-encoding then 42
    else if eq-u8 id hdr-content-language then 255
    else if eq-u8 id hdr-content-length then 4
    else if eq-u8 id hdr-content-location then 255
    else if eq-u8 id hdr-content-range then 255
    else if eq-u8 id hdr-content-type then 44
    else if eq-u8 id hdr-cookie then 5
    else if eq-u8 id hdr-date then 6
    else if eq-u8 id hdr-etag then 7
    else if eq-u8 id hdr-expect then 255
    else if eq-u8 id hdr-expires then 255
    else if eq-u8 id hdr-from then 255
    else if eq-u8 id hdr-host then 255
    else if eq-u8 id hdr-if-match then 255
    else if eq-u8 id hdr-if-modified-since then 8
    else if eq-u8 id hdr-if-none-match then 9
    else if eq-u8 id hdr-if-range then 89
    else if eq-u8 id hdr-if-unmodified-since then 255
    else if eq-u8 id hdr-last-modified then 10
    else if eq-u8 id hdr-link then 11
    else if eq-u8 id hdr-location then 12
    else if eq-u8 id hdr-max-forwards then 255
    else if eq-u8 id hdr-origin then 90
    else if eq-u8 id hdr-priority then 255
    else if eq-u8 id hdr-proxy-authenticate then 255
    else if eq-u8 id hdr-proxy-authorization then 255
    else if eq-u8 id hdr-range then 55
    else if eq-u8 id hdr-referer then 13
    else if eq-u8 id hdr-refresh then 255
    else if eq-u8 id hdr-retry-after then 255
    else if eq-u8 id hdr-sec-websocket-accept then 255
    else if eq-u8 id hdr-sec-websocket-key then 255
    else if eq-u8 id hdr-sec-websocket-version then 255
    else if eq-u8 id hdr-server then 92
    else if eq-u8 id hdr-set-cookie then 14
    else if eq-u8 id hdr-strict-transport-security then 56
    else if eq-u8 id hdr-te then 255
    else if eq-u8 id hdr-transfer-encoding then 255
    else if eq-u8 id hdr-upgrade then 255
    else if eq-u8 id hdr-user-agent then 95
    else if eq-u8 id hdr-vary then 59
    else if eq-u8 id hdr-via then 255
    else if eq-u8 id hdr-www-authenticate then 255
    else if eq-u8 id hdr-alt-svc then 83
    else 255

  write-one-known-hdr :: U8 -> I64 -> ResponseHeadPending %1 ->{MVar} U32
  write-one-known-hdr id val-addr w0 =
    let w1 = response-write-status w0 (status-ok ()) in
    let w2 = response-write-header-known w1 id val-addr 2 in
    let b0 = response-finish-head w2 in
    case response-end b0 of
      (closed, total) ->
        let _ = response-finish-closed closed in
        total

  -- Check one header across lowercase, UPPERCASE, MiXeD-CaSe, non-zero offset,
  -- 1-bit mutation rejection (including '-' -> 0x0D and ':' -> 0x1A!), HPACK/QPACK,
  -- bitmask uniqueness, and ResponseWriter known-header wire emission!
  check-header-at :: I64 -> I64 -> U8 -> U64 ->{MVar} U64
  check-header-at dma out-dma id seen-mask =
    if ge-u8 id known-header-count then seen-mask
    else
      let bit = known-header-bit id in
      let expected-bit = wrapping-shl-u64 1 (u8-to-u32 id) in
      if ne-u64 bit expected-bit then 0
      else if ne-u64 (bit-and-u64 seen-mask bit) 0 then 0
      else if ne-u8 (known-header-hpack-index id) (exp-hpack id) then 0
      else if ne-u8 (known-header-qpack-index id) (exp-qpack id) then 0
      else
        let name = known-header-wire-name id in
        let b-lo = string-to-bytes name in
        let n = bytes-len b-lo in
        let n32 = wrapping-i64-to-u32 (nat-to-i64 n) in
        -- Prefix padding to test non-zero offset in Bytes!
        let padded = bytes-new (n + 5) in
        let _ = bytes-copy-range padded b-lo 3 0 n in
        -- Lowercase match (Bytes + DMA)
        let _ = copy-bytes-to-dma b-lo dma 0 in
        let m-lo-b = known-header-match-bytes padded 3 n in
        let m-lo-d = known-header-match-dma dma n32 in
        -- UPPERCASE match (Bytes + DMA)
        let b-up = string-to-bytes name in
        let _ = transform-case b-up 0 1 in
        let _ = copy-bytes-to-dma b-up dma 0 in
        let m-up-b = known-header-match-bytes b-up 0 n in
        let m-up-d = known-header-match-dma dma n32 in
        -- MiXeD match (Bytes + DMA)
        let b-mx = string-to-bytes name in
        let _ = transform-case b-mx 0 2 in
        let _ = copy-bytes-to-dma b-mx dma 0 in
        let m-mx-b = known-header-match-bytes b-mx 0 n in
        let m-mx-d = known-header-match-dma dma n32 in
        -- Mutate last byte (flip ASCII letter or punctuation) -> MUST NOT match!
        let b-bad = string-to-bytes name in
        let _ = (if n > 0 then
          let last = n - 1 in
          if last < bytes-len b-bad then
            let old = bytes-get-u8 b-bad last in
            bytes-set-u8 b-bad last (if eq-u8 old 120 then 121 else 120)
          else ()
          else ()) in
        let _ = copy-bytes-to-dma b-bad dma 0 in
        let m-bad-b = known-header-match-bytes b-bad 0 n in
        let m-bad-d = known-header-match-dma dma n32 in
        -- Verify ResponseWriter emits exact `"HTTP/1.1 200 OK\r\n<wire-name>: ok\r\n\r\n"`
        let val-addr = wrapping-add-i64 dma 512 in
        let _ = raw-store-u8 val-addr 111 in -- 'o'
        let _ = raw-store-u8 (wrapping-add-i64 val-addr 1) 107 in -- 'k'
        let written = response-with-writer out-dma 256 (write-one-known-hdr id val-addr) in
        let got-wire = dma-to-string out-dma written in
        let exp-wire = string-concat "HTTP/1.1 200 OK\r\n" (string-concat name ": ok\r\n\r\n") in
        let wire-ok = eq-i64 (string-compare got-wire exp-wire) 0 in
        if eq-u8 m-lo-b id && eq-u8 m-lo-d id
          && eq-u8 m-up-b id && eq-u8 m-up-d id
          && eq-u8 m-mx-b id && eq-u8 m-mx-d id
          && eq-u8 m-bad-b hdr-unknown && eq-u8 m-bad-d hdr-unknown
          && wire-ok
        then check-header-at dma out-dma (wrapping-add-u8 id 1) (bit-or-u64 seen-mask bit)
        else 0

  -- Verify alpha-mask precision: 0x0D ('\r') MUST NOT match '-' (0x2D),
  -- and 0x1A MUST NOT match ':' (0x3A)!
  check-punctuation-precision :: I64 ->{MVar} Bool
  check-punctuation-precision dma =
    -- "content\rtype" (0x0D instead of '-') must return hdr-unknown!
    let b1 = string-to-bytes "content-type" in
    let _ = if 7 < bytes-len b1 then bytes-set-u8 b1 7 13 else () in
    let _ = copy-bytes-to-dma b1 dma 0 in
    let r1-b = known-header-match-bytes b1 0 12 in
    let r1-d = known-header-match-dma dma 12 in
    -- "\x1Apath" (0x1A instead of ':') must return hdr-unknown!
    let b2 = string-to-bytes ":path" in
    let _ = if 0 < bytes-len b2 then bytes-set-u8 b2 0 26 else () in
    let _ = copy-bytes-to-dma b2 dma 0 in
    let r2-b = known-header-match-bytes b2 0 5 in
    let r2-d = known-header-match-dma dma 5 in
    -- Out-of-bounds Bytes slice and unknown/empty lengths must return hdr-unknown!
    let r3-b = known-header-match-bytes b1 10 12 in
    let r4-d = known-header-match-dma dma 0 in
    let r5-d = known-header-match-dma dma 1 in
    let r6-d = known-header-match-dma dma 40 in
    eq-u8 r1-b hdr-unknown && eq-u8 r1-d hdr-unknown
      && eq-u8 r2-b hdr-unknown && eq-u8 r2-d hdr-unknown
      && eq-u8 r3-b hdr-unknown && eq-u8 r4-d hdr-unknown
      && eq-u8 r5-d hdr-unknown && eq-u8 r6-d hdr-unknown

  -- Verify all 17 HttpStatus constructors, immortal status lines, and DMA ResponseWriter output.
  check-one-status :: I64 -> HttpStatus -> U16 -> String ->{MVar} Bool
  check-one-status out-dma st exp-code exp-line =
    let line = http-status-line st in
    let written = response-with-writer out-dma 128 (\w0 ->
      let w1 = response-write-status w0 st in
      let b0 = response-finish-head w1 in
      case response-end b0 of
        (closed, total) ->
          let _ = response-finish-closed closed in
          total) in
    let got = dma-to-string out-dma written in
    let exp-full = string-concat exp-line "\r\n" in
    eq-u16 st.status-code exp-code
      && eq-i64 (string-compare line exp-line) 0
      && eq-i64 (string-compare got exp-full) 0

  check-all-statuses :: I64 ->{MVar} Bool
  check-all-statuses out-dma =
    check-one-status out-dma (status-ok ()) 200 "HTTP/1.1 200 OK\r\n"
      && check-one-status out-dma (status-created ()) 201 "HTTP/1.1 201 Created\r\n"
      && check-one-status out-dma (status-no-content ()) 204 "HTTP/1.1 204 No Content\r\n"
      && check-one-status out-dma (status-partial-content ()) 206 "HTTP/1.1 206 Partial Content\r\n"
      && check-one-status out-dma (status-moved-permanently ()) 301 "HTTP/1.1 301 Moved Permanently\r\n"
      && check-one-status out-dma (status-not-modified ()) 304 "HTTP/1.1 304 Not Modified\r\n"
      && check-one-status out-dma (status-bad-request ()) 400 "HTTP/1.1 400 Bad Request\r\n"
      && check-one-status out-dma (status-unauthorized ()) 401 "HTTP/1.1 401 Unauthorized\r\n"
      && check-one-status out-dma (status-forbidden ()) 403 "HTTP/1.1 403 Forbidden\r\n"
      && check-one-status out-dma (status-not-found ()) 404 "HTTP/1.1 404 Not Found\r\n"
      && check-one-status out-dma (status-method-not-allowed ()) 405 "HTTP/1.1 405 Method Not Allowed\r\n"
      && check-one-status out-dma (status-payload-too-large ()) 413 "HTTP/1.1 413 Payload Too Large\r\n"
      && check-one-status out-dma (status-too-many-requests ()) 429 "HTTP/1.1 429 Too Many Requests\r\n"
      && check-one-status out-dma (status-internal-error ()) 500 "HTTP/1.1 500 Internal Server Error\r\n"
      && check-one-status out-dma (status-bad-gateway ()) 502 "HTTP/1.1 502 Bad Gateway\r\n"
      && check-one-status out-dma (status-service-unavailable ()) 503 "HTTP/1.1 503 Service Unavailable\r\n"
      && check-one-status out-dma (status-switching-protocols ()) 101 "HTTP/1.1 101 Switching Protocols\r\n"

  check-methods-versions-reqhead :: I64 ->{MVar} Bool
  check-methods-versions-reqhead out-dma =
    let m-ok = (
      http-method-eq (http-method-from-u8 0) MethodGet
        && http-method-eq (http-method-from-u8 1) MethodHead
        && http-method-eq (http-method-from-u8 2) MethodPost
        && http-method-eq (http-method-from-u8 3) MethodPut
        && http-method-eq (http-method-from-u8 4) MethodDelete
        && http-method-eq (http-method-from-u8 5) MethodConnect
        && http-method-eq (http-method-from-u8 6) MethodOptions
        && http-method-eq (http-method-from-u8 7) MethodTrace
        && http-method-eq (http-method-from-u8 8) MethodPatch
        && http-method-eq (http-method-from-u8 255) MethodCustom
        && eq-u8 (http-method-to-u8 MethodPatch) 8
        && eq-u8 (http-method-to-u8 MethodCustom) 255
        && eq-i64 (string-compare (http-method-name MethodGet) "GET") 0
        && eq-i64 (string-compare (http-method-name MethodPost) "POST") 0
        && eq-i64 (string-compare (http-method-name MethodDelete) "DELETE") 0
        && eq-i64 (string-compare (http-method-name MethodPatch) "PATCH") 0) in
    let v-ok = (
      http-version-eq (http-version-from-u8 0) Http10
        && http-version-eq (http-version-from-u8 1) Http11
        && http-version-eq (http-version-from-u8 2) Http2
        && http-version-eq (http-version-from-u8 3) Http3
        && eq-i64 (string-compare (http-version-name Http11) "HTTP/1.1") 0
        && eq-i64 (string-compare (http-version-name Http2) "HTTP/2") 0
        && eq-i64 (string-compare (http-version-name Http3) "HTTP/3") 0) in
    let mask = bit-or-u64 (known-header-bit hdr-host) (known-header-bit hdr-content-type) in
    let flags = bit-or-u32 req-flag-keep-alive req-flag-has-cl in
    let req = RequestHead method-code-post version-code-http11 2 flags 5 10 16 4 64 128 mask in
    let rh-ok = (
      req-has-header req hdr-host
        && req-has-header req hdr-content-type
        && not (req-has-header req hdr-cookie)
        && not (req-has-header req hdr-unknown)
        && req-has-flag req req-flag-keep-alive
        && req-has-flag req req-flag-has-cl
        && not (req-has-flag req req-flag-chunked)
        && not (req-has-flag req req-flag-upgrade)) in
    -- Full HTTP/1.1 response with status + 2 headers + 2 body chunks
    let val-ct = string-to-bytes "text/plain" in
    let val-cl = string-to-bytes "11" in
    let body1 = string-to-bytes "hello " in
    let body2 = string-to-bytes "world" in
    let a-ct = wrapping-add-i64 out-dma 1024 in
    let a-cl = wrapping-add-i64 out-dma 1056 in
    let a-b1 = wrapping-add-i64 out-dma 1088 in
    let a-b2 = wrapping-add-i64 out-dma 1120 in
    let _ = copy-bytes-to-dma val-ct a-ct 0 in
    let _ = copy-bytes-to-dma val-cl a-cl 0 in
    let _ = copy-bytes-to-dma body1 a-b1 0 in
    let _ = copy-bytes-to-dma body2 a-b2 0 in
    let total = response-with-writer out-dma 512 (\w0 ->
      let w1 = response-write-status w0 (status-created ()) in
      let w2 = response-write-header-known w1 hdr-content-type a-ct 10 in
      let w3 = response-write-header-known w2 hdr-content-length a-cl 2 in
      let b0 = response-finish-head w3 in
      let b1 = response-write-body b0 a-b1 6 in
      let b2 = response-write-body b1 a-b2 5 in
      case response-end b2 of
        (closed, n) ->
          let _ = response-finish-closed closed in
          n) in
    let got-resp = dma-to-string out-dma total in
    let exp-resp = "HTTP/1.1 201 Created\r\ncontent-type: text/plain\r\ncontent-length: 11\r\n\r\nhello world" in
    -- Bounds safety: tiny cap (10B) with guard sentinel at byte 10 must never be overwritten!
    let _ = raw-store-u8 (wrapping-add-i64 out-dma 10) 0xAB in
    let _ = response-with-writer out-dma 10 (\w0 ->
      let w1 = response-write-status w0 (status-ok ()) in
      let b0 = response-finish-head w1 in
      let b1 = response-write-body b0 a-b1 6 in
      case response-end b1 of
        (closed, _) -> response-finish-closed closed) in
    let guard-ok = eq-u8 (raw-load-u8 (wrapping-add-i64 out-dma 10)) 0xAB in
    m-ok && v-ok && rh-ok && eq-i64 (string-compare got-resp exp-resp) 0 && guard-ok

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma-u64 = engine-dma-alloc 4096 4096 in
    if eq-u64 dma-u64 0 then 1
    else
      let dma = wrapping-u64-to-i64 dma-u64 in
      let out-dma = wrapping-add-i64 dma 2048 in
      let mask = check-header-at dma out-dma 0 0 in
      -- 61 unique bits set => popcount == 61, and unknown has bit 0 / hpack 0 / qpack 255
      let rc = (
        if ne-u32 (count-ones-u64 mask) 61 then 2
        else if ne-u64 (known-header-bit hdr-unknown) 0 then 3
        else if ne-u8 (known-header-hpack-index hdr-unknown) 0 then 4
        else if ne-u8 (known-header-qpack-index hdr-unknown) 255 then 5
        else if not (check-punctuation-precision dma) then 6
        else if not (check-all-statuses out-dma) then 7
        else if not (check-methods-versions-reqhead out-dma) then 8
        else 0) in
      let _ = engine-dma-free dma-u64 4096 in
      rc
"#;

    run_leak_checked("http-core-known-headers-and-writer", src, 0);
}

// =========================================================================
// 4. Linear %1 ResponseWriter compile-fail state machine proofs
// =========================================================================

#[test]
fn net_http_response_writer_linear_state_machine_compile_fail() {
    let expect_reject = |tag: &str, src: &str, needles: &[&str]| {
        let dir = project_with_http_core(tag, src);
        let err = common::check_diagnostics(&dir, "Main.cinch")
            .expect_err(&format!("{tag}: expected compile-time rejection"));
        common::cleanup(&dir);
        for n in needles {
            assert!(
                err.contains(n),
                "{tag}: expected diagnostic containing {n:?}, got:\n{err}"
            );
        }
    };

    // 1. Dropping ResponseHeadPending %1 without finishing must be rejected!
    expect_reject(
        "rw-drop-head-direct",
        r#"
module Main where
  import Net.Http.Types

  bad-drop-head :: ResponseHeadPending %1 ->{MVar} Unit
  bad-drop-head w = ()

  main :: Int
  main = 0
"#,
        &["multiplicity %1", "never used"],
    );

    // 2. Dropping ResponseHeadPending %1 inside `response-with-writer` must be rejected!
    expect_reject(
        "rw-drop-head-scoped",
        r#"
module Main where
  import Net.Http.Types

  bad-scoped :: I64 -> U32 ->{MVar} Unit
  bad-scoped addr cap = response-with-writer addr cap (\w -> ())

  main :: Int
  main = 0
"#,
        &["multiplicity %1", "never used"],
    );

    // 3. Dropping ResponseHeadPending %1 on only one branch must be rejected!
    expect_reject(
        "rw-drop-head-branch",
        r#"
module Main where
  import Net.Http.Types

  bad-branch :: Bool -> ResponseHeadPending %1 ->{MVar} Unit
  bad-branch ok w =
    if ok then
      let b = response-finish-head w in
      case response-end b of
        (c, _) -> response-finish-closed c
    else ()

  main :: Int
  main = 0
"#,
        &["multiplicity %1", "unused on some path"],
    );

    // 4. Calling `response-write-body` before `response-finish-head` must be rejected!
    expect_reject(
        "rw-body-before-head",
        r#"
module Main where
  import Net.Http.Types

  bad-order :: ResponseHeadPending %1 -> I64 -> U32 ->{MVar} ResponseBodyOpen
  bad-order w body-addr body-len = response-write-body w body-addr body-len

  main :: Int
  main = 0
"#,
        &["ResponseBodyOpen", "ResponseHeadPending"],
    );

    // 5. Calling `response-write-status` after `response-finish-head` must be rejected!
    expect_reject(
        "rw-status-after-head",
        r#"
module Main where
  import Net.Http.Types

  bad-status-after :: ResponseHeadPending %1 ->{MVar} ResponseHeadPending
  bad-status-after w =
    let body = response-finish-head w in
    response-write-status body (status-ok ())

  main :: Int
  main = 0
"#,
        &["ResponseHeadPending", "ResponseBodyOpen"],
    );

    // 6. Calling `response-write-header-known` after `response-finish-head` must be rejected!
    expect_reject(
        "rw-header-after-head",
        r#"
module Main where
  import Net.Http.Types

  bad-hdr-after :: ResponseHeadPending %1 -> I64 -> U32 ->{MVar} ResponseHeadPending
  bad-hdr-after w va vl =
    let body = response-finish-head w in
    response-write-header-known body hdr-content-type va vl

  main :: Int
  main = 0
"#,
        &["ResponseHeadPending", "ResponseBodyOpen"],
    );

    // 7. Double-calling `response-end` on a `ResponseBodyOpen %1` must be rejected!
    expect_reject(
        "rw-double-end",
        r#"
module Main where
  import Net.Http.Types

  bad-double-end :: ResponseBodyOpen %1 -> U32
  bad-double-end body =
    case response-end body of
      (c1, n1) ->
        let _ = response-finish-closed c1 in
        case response-end body of
          (c2, n2) ->
            let _ = response-finish-closed c2 in
            wrapping-add-u32 n1 n2

  main :: Int
  main = 0
"#,
        &["multiplicity %1", "used 2 time"],
    );

    // 8. Dropping `ResponseClosed %1` without calling `response-finish-closed` must be rejected!
    expect_reject(
        "rw-drop-closed",
        r#"
module Main where
  import Net.Http.Types

  bad-drop-closed :: ResponseClosed %1 -> Unit
  bad-drop-closed c = ()

  main :: Int
  main = 0
"#,
        &["multiplicity %1", "never used"],
    );

    // 9. Private constructor `MkResponseHeadPending` cannot be forged outside `Net.Http.Types`!
    expect_reject(
        "rw-private-ctor",
        r#"
module Main where
  import Net.Http.Types

  forge :: ResponseHeadPending
  forge = MkResponseHeadPending 0 0 0 200 0 0

  main :: Int
  main = 0
"#,
        &["MkResponseHeadPending"],
    );

    // 10. Valid full linear transition pipeline MUST compile cleanly!
    let valid_src = r#"
module Main where
  import Net.Http.Types

  good-pipeline :: ResponseHeadPending %1 -> I64 -> U32 -> I64 -> U32 ->{MVar} U32
  good-pipeline w0 va vl ba bl =
    let w1 = response-write-status w0 (status-ok ()) in
    let w2 = response-write-header-known w1 hdr-content-type va vl in
    let b0 = response-finish-head w2 in
    let b1 = response-write-body b0 ba bl in
    case response-end b1 of
      (closed, total) ->
        let _ = response-finish-closed closed in
        total

  main :: Int
  main = 0
"#;
    let dir_ok = project_with_http_core("rw-valid-pipeline", valid_src);
    common::check(&dir_ok, "Main.cinch")
        .expect("valid linear ResponseWriter pipeline must compile cleanly");
    common::cleanup(&dir_ok);
}

// =========================================================================
// 5. RFC 9218 Priority Header Parser, PriorityScheduler semantics, and
//    50,000-op steady-state 0-alloc / 0-atomic-RC / leaked=0 hot loop
// =========================================================================

#[test]
fn net_http_priority_rfc9218_scheduler_and_50k_zero_alloc_hot_loop() {
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
  import Prim.Atomic

  copy-bytes-to-dma :: Bytes -> I64 -> Nat ->{MVar} Unit
  copy-bytes-to-dma b base i =
    if i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 base (nat-to-i64 i)) (bytes-get-u8 b i) in
      copy-bytes-to-dma b base (i + 1)
    else ()

  check-parse :: I64 -> String -> U8 -> U8 ->{MVar} Bool
  check-parse dma s exp-u exp-i =
    let b = string-to-bytes s in
    let n = bytes-len b in
    let _ = copy-bytes-to-dma b dma 0 in
    let p = parse-priority-header-dma dma (wrapping-i64-to-u32 (nat-to-i64 n)) in
    eq-u8 p.prio-urgency exp-u && eq-u8 p.prio-incremental exp-i

  -- 1. RFC 9218 Section 5 / RFC 8941 Structured Fields parser vectors
  test-priority-parser :: I64 ->{MVar} Bool
  test-priority-parser dma =
    check-parse dma "" 3 0
      && check-parse dma "u=0" 0 0
      && check-parse dma "u=7" 7 0
      && check-parse dma "i" 3 1
      && check-parse dma "i=?1" 3 1
      && check-parse dma "i=?0" 3 0
      && check-parse dma "u=1, i" 1 1
      && check-parse dma "i, u=2" 2 1
      && check-parse dma "u=5;foo=bar, i=?1;baz=1" 5 1
      && check-parse dma "  u=0 ,  i  " 0 1
      && check-parse dma "ext=\"a,u=7,b\", u=6, i" 6 1
      && check-parse dma "u=1, u=4, i, i=?0" 4 0
      && check-parse dma "u=8, u=99, u=-1, i=1" 3 0
      && check-parse dma "u=2, u=10" 2 0
      && eq-u8 (parse-priority-header-dma 0 0).prio-urgency 3

  -- 2. Strict urgency ordering 0 < 1 < ... < 7 across all 8 buckets
  enqueue-all-urgencies :: PriorityScheduler -> U8 ->{MVar} Bool
  enqueue-all-urgencies sc u =
    if gt-u8 u 7 then True
    else
      -- Enqueue in reverse urgency order (7 down to 0) into slots (10 + u).
      let rev-u = wrapping-sub-u8 7 u in
      let slot = wrapping-add-u32 10 (u8-to-u32 rev-u) in
      if prio-sched-enqueue sc slot (stream-priority-make-raw rev-u 0) then
        enqueue-all-urgencies sc (wrapping-add-u8 u 1)
      else False

  drain-urgencies-in-order :: PriorityScheduler -> U8 ->{MVar} Bool
  drain-urgencies-in-order sc u =
    if gt-u8 u 7 then
      eq-u32 (prio-sched-next sc) prio-slot-none
        && eq-u32 (prio-sched-active-count sc) 0
        && eq-u8 (prio-sched-bucket-mask sc) 0
    else
      let exp-slot = wrapping-add-u32 10 (u8-to-u32 u) in
      let got = prio-sched-next sc in
      if ne-u32 got exp-slot then False
      else if not (prio-sched-remove sc got) then False
      else drain-urgencies-in-order sc (wrapping-add-u8 u 1)

  -- 3. FIFO for non-incremental (i == 0), round-robin for incremental (i == 1),
  --    and dynamic reprioritization (promote / demote / toggle incremental).
  test-scheduler-semantics :: Unit ->{IO, MVar} Int
  test-scheduler-semantics _ =
    case prio-sched-new 0 of
      Some s -> let _ = prio-sched-destroy s in 10
      None ->
        case prio-sched-new 64 of
          None -> 11
          Some sc ->
            let rc = (
              -- Empty scheduler checks
              if ne-u32 (prio-sched-capacity sc) 64 then 12
              else if ne-u32 (prio-sched-active-count sc) 0 then 13
              else if ne-u32 (prio-sched-next sc) prio-slot-none then 14
              -- Strict urgency 0..7 ordering
              else if not (enqueue-all-urgencies sc 0) then 15
              else if ne-u8 (prio-sched-bucket-mask sc) 255 then 16
              else if ne-u32 (prio-sched-active-count sc) 8 then 17
              else if not (drain-urgencies-in-order sc 0) then 18
              else
                -- Non-incremental FIFO in bucket u=2: streams 1, 2, 3
                let p-fifo = stream-priority-make-raw 2 0 in
                let ok-e1 = (prio-sched-enqueue sc 1 p-fifo
                  && prio-sched-enqueue sc 2 p-fifo
                  && prio-sched-enqueue sc 3 p-fifo
                  && not (prio-sched-enqueue sc 1 p-fifo)
                  && not (prio-sched-enqueue sc 64 p-fifo)) in
                -- Repeated `prio-sched-next` keeps returning head stream 1 (FIFO!)
                let f1a = prio-sched-next sc in
                let f1b = prio-sched-next sc in
                let _ = prio-sched-remove sc 1 in
                let f2a = prio-sched-next sc in
                let f2b = prio-sched-next sc in
                let _ = prio-sched-remove sc 2 in
                let f3 = prio-sched-next sc in
                let _ = prio-sched-remove sc 3 in
                let fifo-ok = (ok-e1 && eq-u32 f1a 1 && eq-u32 f1b 1
                  && eq-u32 f2a 2 && eq-u32 f2b 2 && eq-u32 f3 3
                  && eq-u32 (prio-sched-active-count sc) 0) in
                if not fifo-ok then 19
                else
                  -- Incremental round-robin in bucket u=4: streams 20, 21, 22
                  let p-rr = stream-priority-make-raw 4 1 in
                  let ok-e2 = (prio-sched-enqueue sc 20 p-rr
                    && prio-sched-enqueue sc 21 p-rr
                    && prio-sched-enqueue sc 22 p-rr) in
                  let r0 = prio-sched-next sc in
                  let r1 = prio-sched-next sc in
                  let r2 = prio-sched-next sc in
                  let r3 = prio-sched-next sc in
                  let r4 = prio-sched-next sc in
                  let r5 = prio-sched-next sc in
                  let rr-ok = (ok-e2 && eq-u32 r0 20 && eq-u32 r1 21 && eq-u32 r2 22
                    && eq-u32 r3 20 && eq-u32 r4 21 && eq-u32 r5 22) in
                  if not rr-ok then 20
                  else
                    -- Dynamic reprioritization:
                    -- Promote slot 22 from u=4 -> u=0 (non-inc): must immediately win!
                    let ok-p1 = prio-sched-reprioritize sc 22 (stream-priority-make-raw 0 0) in
                    let p-win1 = prio-sched-next sc in
                    let p-win2 = prio-sched-next sc in
                    -- Demote slot 22 from u=0 -> u=7 (inc): u=4 streams (20, 21) now win!
                    let ok-p2 = prio-sched-reprioritize sc 22 (stream-priority-make-raw 7 1) in
                    let d-win1 = prio-sched-next sc in
                    let d-win2 = prio-sched-next sc in
                    -- Toggle slot 20 in-place (u=4, i=1 -> u=4, i=0): stays at head!
                    let ok-p3 = prio-sched-reprioritize sc 20 (stream-priority-make-raw 4 0) in
                    let t-win1 = prio-sched-next sc in
                    let t-win2 = prio-sched-next sc in
                    -- Clean up
                    let _ = prio-sched-remove sc 20 in
                    let _ = prio-sched-remove sc 21 in
                    let _ = prio-sched-remove sc 22 in
                    let reprio-ok = (ok-p1 && ok-p2 && ok-p3
                      && eq-u32 p-win1 22 && eq-u32 p-win2 22
                      && eq-u32 d-win1 20 && eq-u32 d-win2 21
                      && eq-u32 t-win1 20 && eq-u32 t-win2 20
                      && not (prio-sched-reprioritize sc 20 (stream-priority-make-raw 1 0))
                      && not (prio-sched-remove sc 20)
                      && eq-u32 (prio-sched-active-count sc) 0) in
                    if reprio-ok then 0 else 21) in
            let _ = prio-sched-destroy sc in
            rc

  -- 4. 50,000-op steady-state hot loop across PriorityScheduler + Priority parser +
  --    KnownHeader DMA match + HttpMethod/HttpStatus tables:
  --    MUST record 0 heap allocs & 0 atomic RC ops!
  #[no-alloc]
  hot-loop :: PriorityScheduler -> I64 -> I64 -> U32 -> U64 ->{MVar} U64
  hot-loop sc hdr-dma prio-dma rem acc =
    if eq-u32 rem 0 then acc
    else
      let slot = bit-and-u32 rem 63 in
      let u = wrapping-u32-to-u8 (bit-and-u32 rem 7) in
      let inc = wrapping-u32-to-u8 (bit-and-u32 (wrapping-shr-u32 rem 3) 1) in
      let prio = stream-priority-make-raw u inc in
      let _ = prio-sched-enqueue sc slot prio in
      let parsed = parse-priority-header-dma prio-dma 6 in
      let _ = prio-sched-reprioritize sc slot parsed in
      let picked = prio-sched-next sc in
      let _ = prio-sched-remove sc slot in
      let hid = known-header-match-dma hdr-dma 12 in
      let hbit = known-header-bit hid in
      let hp = known-header-hpack-index hid in
      let qp = known-header-qpack-index hid in
      let req = RequestHead method-code-get version-code-http2 1 req-flag-keep-alive 0 1 0 0 0 0 hbit in
      let has = if req-has-header req hdr-content-type then 1 else 0 in
      let delta = wrapping-add-u64 (u32-to-u64 picked) (wrapping-add-u64 (u8-to-u64 hp) (wrapping-add-u64 (u8-to-u64 qp) has)) in
      hot-loop sc hdr-dma prio-dma (wrapping-sub-u32 rem 1) (wrapping-add-u64 acc delta)

  test-fiftyk-zero-alloc :: I64 ->{IO, MVar} Int
  test-fiftyk-zero-alloc dma =
    -- Pre-populate DMA slices: "Content-Type" (12B) at dma+0, "u=1, i" (6B) at dma+64
    let h-bytes = string-to-bytes "Content-Type" in
    let p-bytes = string-to-bytes "u=1, i" in
    let prio-dma = wrapping-add-i64 dma 64 in
    let _ = copy-bytes-to-dma h-bytes dma 0 in
    let _ = copy-bytes-to-dma p-bytes prio-dma 0 in
    case prio-sched-new 64 of
      None -> 30
      Some sc ->
        let _ = engine-thread-counters-reset () in
        let acc = hot-loop sc dma prio-dma 50000 0 in
        let allocs = engine-thread-alloc-count () in
        let atomic-rcs = engine-thread-atomic-rc-count () in
        let active = prio-sched-active-count sc in
        let _ = prio-sched-destroy sc in
        if eq-u64 acc 0 then 31
        else if ne-u32 active 0 then 32
        else if ne-u64 allocs 0 then 33
        else if ne-u64 atomic-rcs 0 then 34
        else 0

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma-u64 = engine-dma-alloc 4096 4096 in
    if eq-u64 dma-u64 0 then 1
    else
      let dma = wrapping-u64-to-i64 dma-u64 in
      let r1 = if test-priority-parser dma then 0 else 2 in
      let r2 = if r1 /= 0 then r1 else test-scheduler-semantics () in
      let r3 = if r2 /= 0 then r2 else test-fiftyk-zero-alloc dma in
      let _ = engine-dma-free dma-u64 4096 in
      r3
"#;

    run_leak_checked("http-priority-rfc9218-50k", src, 0);
}
