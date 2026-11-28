//! Integration, RFC 9204 Appendix A (all 99 static entries), RFC 9204 Appendix B
//! exact wire-byte interop (B.1..B.5), RFC 7541 Appendix B Huffman + 4-bit nibble
//! DFA, malformed/attack rejection, `#[no-alloc]` interface safety, and 50,000-iter
//! zero-allocation / zero-atomic-RC / leak-checked tests for
//! `Net.Http.Qpack.StaticTable` and `Net.Http.Qpack.Codec`.
//!
//! Routed through `common::project` / `common::drive_with_profile` /
//! `common::check_interfaces` / `common::build_executable` so the exact same
//! suite runs against both the Rust seed and the Stage 3 self-hosted compiler.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{Mode, Package};
use std::process::Command;
use std::time::Instant;

const PRIM_ATOMIC: &str = include_str!("../prelude/Prim/Atomic.cinch");
const ENGINE_TYPES: &str = include_str!("../prelude/Engine/Types.cinch");
const NET_HTTP_TYPES: &str = include_str!("../prelude/Net/Http/Types.cinch");
const NET_HTTP_QPACK_STATIC: &str = include_str!("../prelude/Net/Http/Qpack/StaticTable.cinch");
const NET_HTTP_QPACK_CODEC: &str = include_str!("../prelude/Net/Http/Qpack/Codec.cinch");
const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn project_with_qpack(tag: &str, main_src: &str) -> std::path::PathBuf {
    common::project(
        tag,
        &[
            ("Prim.Atomic.cinch", PRIM_ATOMIC),
            ("Engine.Types.cinch", ENGINE_TYPES),
            ("Net.Http.Types.cinch", NET_HTTP_TYPES),
            ("Net.Http.Qpack.StaticTable.cinch", NET_HTTP_QPACK_STATIC),
            ("Net.Http.Qpack.Codec.cinch", NET_HTTP_QPACK_CODEC),
            ("Main.cinch", main_src),
        ],
    )
}

fn run_leak_checked(tag: &str, src: &str, expected: i32) -> (String, std::time::Duration) {
    let dir = project_with_qpack(tag, src);
    let exe = common::build_executable(&dir, "Main.cinch", "net-http-qpack-test")
        .unwrap_or_else(|e| panic!("{tag}: build failed:\n{e}"));
    let started = Instant::now();
    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "128M")
        .output()
        .expect("run built binary");
    let elapsed = started.elapsed();
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
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
    (stdout, elapsed)
}

// =========================================================================
// 1. Package manifest registration, direct stdlib resolution & #[no-alloc]
// =========================================================================

#[test]
fn net_http_qpack_registered_in_manifest_and_no_alloc_safe() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, path) in [
        ("Net.Http.Qpack.StaticTable", "Net/Http/Qpack/StaticTable.cinch"),
        ("Net.Http.Qpack.Codec", "Net/Http/Qpack/Codec.cinch"),
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
    let st_pos = pkg
        .order
        .iter()
        .position(|m| m == "Net.Http.Qpack.StaticTable")
        .unwrap();
    let codec_pos = pkg
        .order
        .iter()
        .position(|m| m == "Net.Http.Qpack.Codec")
        .unwrap();
    assert!(
        st_pos < codec_pos,
        "Net.Http.Qpack.StaticTable must precede Net.Http.Qpack.Codec in .cinchpkg order"
    );

    // Direct stdlib import without copying sources into the project directory.
    let dir_direct = common::project(
        "qpack-stdlib-resolve",
        &[(
            "Main.cinch",
            r#"
module Main where
  import Data.Machine
  import Data.Option
  import Net.Http.Qpack.StaticTable
  import Net.Http.Qpack.Codec

  main :: Unit ->{IO, MVar} Int
  main _ =
    case qpack-table-new 0 of
      None -> 1
      Some tbl ->
        let bs = qpack-table-blocked-streams tbl in
        let _ = qpack-table-destroy tbl in
        if eq-u8 qpack-static-count 99 && eq-u32 bs 0 then 0 else 2
"#,
        )],
    );
    let code = common::drive_with_profile(
        Mode::Run,
        StageMode::Refresh,
        &dir_direct,
        "Main.cinch",
        None,
        &TargetProfile::native(),
    )
    .expect("direct stdlib resolution of Net.Http.Qpack.*");
    common::cleanup(&dir_direct);
    assert_eq!(code, 0);

    // Verify POD ABI layouts and #[no-alloc] facts on hot-path functions.
    let src = r#"
module Main where
  import Data.Machine
  import Net.Http.Qpack.StaticTable
  import Net.Http.Qpack.Codec

  main :: Int
  main =
    if eq-u8 qpack-static-count 99 && eq-u64 (qpack-max-entries 220) 6 then 0 else 1
"#;
    let dir = project_with_qpack("qpack-abi-no-alloc", src);
    let ifaces = common::check_interfaces(&dir, "Main.cinch").expect("check_interfaces");

    let st_iface = ifaces
        .iter()
        .find(|i| i.module == "Net.Http.Qpack.StaticTable")
        .expect("Net.Http.Qpack.StaticTable interface emitted");
    let codec_iface = ifaces
        .iter()
        .find(|i| i.module == "Net.Http.Qpack.Codec")
        .expect("Net.Http.Qpack.Codec interface emitted");

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

    assert_eq!(layout_of(codec_iface, "QpackIntResult").size, 16);
    assert_eq!(layout_of(codec_iface, "QpackStrResult").size, 12);
    assert_eq!(layout_of(codec_iface, "QpackPrefixResult").size, 24);
    assert_eq!(layout_of(codec_iface, "QpackEncodeResult").size, 8);
    assert_eq!(layout_of(codec_iface, "QpackDecodeResult").size, 32);
    assert_eq!(layout_of(codec_iface, "QpackDecoderInst").size, 16);

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
        "qpack-static-is-valid",
        "qpack-static-name",
        "qpack-static-value",
        "qpack-static-name-len",
        "qpack-static-value-len",
        "qpack-static-known-header",
        "qpack-static-from-known",
        "qpack-static-is-sensitive",
        "qpack-static-write-name-dma",
        "qpack-static-write-value-dma",
        "qpack-static-match-name-dma",
        "qpack-static-match-exact-by-name-dma",
        "qpack-static-match-known-exact-dma",
        "qpack-static-match-pair-dma",
    ] {
        assert_no_alloc(st_iface, f);
    }

    for f in [
        "qpack-encode-int",
        "qpack-decode-int",
        "qpack-huffman-encoded-len",
        "qpack-huffman-encode",
        "qpack-huffman-decode",
        "qpack-encode-string",
        "qpack-decode-string",
        "qpack-max-entries",
        "qpack-encode-prefix",
        "qpack-decode-prefix",
        "qpack-table-set-capacity",
        "qpack-table-insert",
        "qpack-table-duplicate",
        "qpack-process-encoder-stream",
        "qpack-decode-decoder-inst",
        "qpack-apply-decoder-inst",
        "qpack-encode-section-zero-hol",
        "qpack-decode-section-zero-hol",
        "qpack-decode-section",
    ] {
        assert_no_alloc(codec_iface, f);
    }

    assert_eq!(common::run(&dir, "Main.cinch"), 0);
    common::cleanup(&dir);
}

// =========================================================================
// 2. All 99 RFC 9204 Appendix A static table entries & KnownHeader round-trip
// =========================================================================

#[test]
fn net_http_qpack_static_table_all_99_entries_and_known_header_roundtrip() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.String
  import Engine.Types
  import Net.Http.Types
  import Net.Http.Qpack.StaticTable
  import Prim.Atomic

  str-eq :: String -> String -> Bool
  str-eq a b = eq-i64 (string-compare a b) 0

  copy-dma-to-bytes :: I64 -> Bytes -> Nat -> Nat ->{MVar} Unit
  copy-dma-to-bytes base dst i len =
    if i < len && i < bytes-len dst then
      let v = raw-load-u8 (wrapping-add-i64 base (nat-to-i64 i)) in
      let _ = bytes-set-u8 dst i v in
      copy-dma-to-bytes base dst (i + 1) len
    else ()

  dma-to-str :: I64 -> U32 ->{MVar} String
  dma-to-str base len =
    let n = u32-to-nat len in
    let b = bytes-new n in
    let _ = copy-dma-to-bytes base b 0 n in
    case string-from-bytes b of
      Some s -> s
      None -> ""

  to-upper-dma :: I64 -> U32 -> U32 ->{MVar} Unit
  to-upper-dma addr i len =
    if ge-u32 i len then ()
    else
      let p = wrapping-add-i64 addr (u32-to-i64 i) in
      let c = raw-load-u8 p in
      let u = if ge-u8 c 97 && le-u8 c 122 then wrapping-sub-u8 c 32 else c in
      let _ = raw-store-u8 p u in
      to-upper-dma addr (wrapping-add-u32 i 1) len

  check-entry :: U8 -> String -> String -> Bool
  check-entry idx exp-name exp-val =
    str-eq (qpack-static-name idx) exp-name
      && str-eq (qpack-static-value idx) exp-val
      && u32-to-nat (qpack-static-name-len idx) == string-byte-length exp-name
      && u32-to-nat (qpack-static-value-len idx) == string-byte-length exp-val

  -- Exhaustive check of every single index 0..98:
  --  1. Valid index, non-empty name, length matches immortal String
  --  2. DMA write-name & write-value reproduce exact immortal String bytes
  --  3. Case-insensitive name match (lower + UPPER) hits canonical name index
  --  4. Exact (name + value) match returns exact index `idx`!
  --  5. `qpack-static-match-pair-dma` returns `(qpack-match-exact << 8) | idx`!
  --  6. KnownHeader consistency: if `kh != hdr-unknown`, then `known-header-wire-name kh == name`
  --     and `qpack-static-from-known kh` shares the same name!
  check-all-entries :: I64 -> I64 -> U8 ->{MVar} Int
  check-all-entries nbuf vbuf idx =
    if ge-u8 idx qpack-static-count then 0
    else if not (qpack-static-is-valid idx) then 100
    else
      let name = qpack-static-name idx in
      let val = qpack-static-value idx in
      let nlen = qpack-static-name-len idx in
      let vlen = qpack-static-value-len idx in
      let nend = qpack-static-write-name-dma idx nbuf 0 256 in
      let vend = qpack-static-write-value-dma idx vbuf 0 256 in
      if ne-u32 nend nlen then 101
      else if ne-u32 vend vlen then 102
      else if not (str-eq (dma-to-str nbuf nend) name) then 103
      else if not (str-eq (dma-to-str vbuf vend) val) then 104
      else
        let midx-lo = qpack-static-match-name-dma nbuf nend in
        let exact = qpack-static-match-exact-by-name-dma midx-lo vbuf vend in
        let pair-res = qpack-static-match-pair-dma nbuf nend vbuf vend in
        let p-kind = wrapping-u16-to-u8 (wrapping-shr-u16 pair-res 8) in
        let p-idx = wrapping-u16-to-u8 pair-res in
        -- Uppercase name in place and re-match name!
        let _ = to-upper-dma nbuf 0 nend in
        let midx-up = qpack-static-match-name-dma nbuf nend in
        let kh = qpack-static-known-header idx in
        let kh-ok = (
          if eq-u8 kh hdr-unknown then True
          else
            let back = qpack-static-from-known kh in
            str-eq (known-header-wire-name kh) name
              && eq-u8 back midx-lo
              && eq-u8 (qpack-static-match-known-exact-dma kh vbuf vend) idx) in
        if eq-u8 midx-lo qpack-static-none then 105
        else if ne-u8 midx-up midx-lo then 106
        else if ne-u8 exact idx then 107
        else if ne-u8 p-kind qpack-match-exact || ne-u8 p-idx idx then 108
        else if not kh-ok then 109
        else check-all-entries nbuf vbuf (wrapping-add-u8 idx 1)

  -- Verify round-trip for all 61 KnownHeader IDs (0..60)
  check-known-roundtrip :: U8 -> Bool
  check-known-roundtrip kh =
    if ge-u8 kh known-header-count then True
    else
      let q = qpack-static-from-known kh in
      let ok = (
        if eq-u8 q qpack-static-none then True
        else qpack-static-is-valid q
          && eq-u8 (qpack-static-known-header q) kh
          && str-eq (qpack-static-name q) (known-header-wire-name kh)) in
      if ok then check-known-roundtrip (wrapping-add-u8 kh 1) else False

  spot-check-rfc9204-appendix-a :: Bool
  spot-check-rfc9204-appendix-a =
    check-entry 0 ":authority" ""
      && check-entry 1 ":path" "/"
      && check-entry 2 "age" "0"
      && check-entry 4 "content-length" "0"
      && check-entry 5 "cookie" ""
      && check-entry 14 "set-cookie" ""
      && check-entry 15 ":method" "CONNECT"
      && check-entry 16 ":method" "DELETE"
      && check-entry 17 ":method" "GET"
      && check-entry 18 ":method" "HEAD"
      && check-entry 19 ":method" "OPTIONS"
      && check-entry 20 ":method" "POST"
      && check-entry 21 ":method" "PUT"
      && check-entry 22 ":scheme" "http"
      && check-entry 23 ":scheme" "https"
      && check-entry 24 ":status" "103"
      && check-entry 25 ":status" "200"
      && check-entry 26 ":status" "304"
      && check-entry 27 ":status" "404"
      && check-entry 28 ":status" "503"
      && check-entry 29 "accept" "*/*"
      && check-entry 30 "accept" "application/dns-message"
      && check-entry 31 "accept-encoding" "gzip, deflate, br"
      && check-entry 35 "access-control-allow-origin" "*"
      && check-entry 36 "cache-control" "max-age=0"
      && check-entry 41 "cache-control" "public, max-age=31536000"
      && check-entry 42 "content-encoding" "br"
      && check-entry 43 "content-encoding" "gzip"
      && check-entry 45 "content-type" "application/javascript"
      && check-entry 46 "content-type" "application/json"
      && check-entry 47 "content-type" "application/x-www-form-urlencoded"
      && check-entry 52 "content-type" "text/html; charset=utf-8"
      && check-entry 53 "content-type" "text/plain"
      && check-entry 54 "content-type" "text/plain;charset=utf-8"
      && check-entry 56 "strict-transport-security" "max-age=31536000"
      && check-entry 57 "strict-transport-security" "max-age=31536000; includesubdomains"
      && check-entry 58 "strict-transport-security" "max-age=31536000; includesubdomains; preload"
      && check-entry 61 "x-content-type-options" "nosniff"
      && check-entry 62 "x-xss-protection" "1; mode=block"
      && check-entry 63 ":status" "100"
      && check-entry 71 ":status" "500"
      && check-entry 72 "accept-language" ""
      && check-entry 73 "access-control-allow-credentials" "FALSE"
      && check-entry 74 "access-control-allow-credentials" "TRUE"
      && check-entry 77 "access-control-allow-methods" "get, post, options"
      && check-entry 83 "alt-svc" "clear"
      && check-entry 84 "authorization" ""
      && check-entry 85 "content-security-policy" "script-src 'none'; object-src 'none'; base-uri 'none'"
      && check-entry 91 "purpose" "prefetch"
      && check-entry 92 "server" ""
      && check-entry 95 "user-agent" ""
      && check-entry 97 "x-frame-options" "deny"
      && check-entry 98 "x-frame-options" "sameorigin"
      && not (qpack-static-is-valid 99)
      && not (qpack-static-is-valid 255)
      && qpack-static-is-sensitive 5
      && qpack-static-is-sensitive 14
      && qpack-static-is-sensitive 84
      && not (qpack-static-is-sensitive 17)

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma-u64 = engine-dma-alloc 4096 4096 in
    if eq-u64 dma-u64 0 then 1
    else
      let nbuf = wrapping-u64-to-i64 dma-u64 in
      let vbuf = wrapping-add-i64 nbuf 1024 in
      let rc = (
        if not spot-check-rfc9204-appendix-a then 2
        else if not (check-known-roundtrip 0) then 3
        else check-all-entries nbuf vbuf 0) in
      let _ = engine-dma-free dma-u64 4096 in
      rc
"#;
    run_leak_checked("qpack-static-99", src, 0);
}

// =========================================================================
// 3. RFC 9204 Appendix B exact wire-byte interop vectors (B.1, B.2, B.3, B.4, B.5)
//    + RFC 7541 Appendix C.4 Huffman + §4.5.1.1 Wraparound + N=1 Sensitivity
// =========================================================================

#[test]
fn net_http_qpack_rfc9204_appendix_b_wire_vectors() {
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
  import Net.Http.Qpack.StaticTable
  import Net.Http.Qpack.Codec
  import Prim.Atomic

  str-eq :: String -> String -> Bool
  str-eq a b = eq-i64 (string-compare a b) 0

  put-str :: I64 -> String ->{MVar} U32
  put-str dst s =
    let b = string-to-bytes s in
    let n = bytes-len b in
    let _ = copy-b2d b dst 0 n in
    wrapping-i64-to-u32 (nat-to-i64 n)

  copy-b2d :: Bytes -> I64 -> Nat -> Nat ->{MVar} Unit
  copy-b2d b dst i n =
    if i < n && i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 dst (nat-to-i64 i)) (bytes-get-u8 b i) in
      copy-b2d b dst (i + 1) n
    else ()

  copy-d2b :: I64 -> Bytes -> Nat -> Nat ->{MVar} Unit
  copy-d2b src dst i n =
    if i < n && i < bytes-len dst then
      let _ = bytes-set-u8 dst i (raw-load-u8 (wrapping-add-i64 src (nat-to-i64 i))) in
      copy-d2b src dst (i + 1) n
    else ()

  dma-to-str :: I64 -> U32 ->{MVar} String
  dma-to-str addr len =
    let n = u32-to-nat len in
    let b = bytes-new n in
    let _ = copy-d2b addr b 0 n in
    case string-from-bytes b of
      Some s -> s
      None -> ""

  hex-nibble :: U8 -> U8
  hex-nibble c =
    if ge-u8 c 48 && le-u8 c 57 then wrapping-sub-u8 c 48
    else if ge-u8 c 97 && le-u8 c 102 then wrapping-add-u8 (wrapping-sub-u8 c 97) 10
    else if ge-u8 c 65 && le-u8 c 70 then wrapping-add-u8 (wrapping-sub-u8 c 65) 10
    else 0

  hex-to-dma-loop :: Bytes -> I64 -> Nat -> U32 ->{MVar} U32
  hex-to-dma-loop b dst i out =
    if i + 1 < bytes-len b then
      let hi = hex-nibble (bytes-get-u8 b i) in
      let lo = hex-nibble (bytes-get-u8 b (i + 1)) in
      let byte = bit-or-u8 (wrapping-shl-u8 hi 4) lo in
      let _ = raw-store-u8 (wrapping-add-i64 dst (u32-to-i64 out)) byte in
      hex-to-dma-loop b dst (i + 2) (wrapping-add-u32 out 1)
    else out

  put-hex :: I64 -> String ->{MVar} U32
  put-hex dst h =
    let b = string-to-bytes h in
    hex-to-dma-loop b dst 0 0

  dma-eq-loop :: I64 -> I64 -> U32 -> U32 ->{MVar} Bool
  dma-eq-loop a b i n =
    if ge-u32 i n then True
    else if eq-u8 (raw-load-u8 (wrapping-add-i64 a (u32-to-i64 i))) (raw-load-u8 (wrapping-add-i64 b (u32-to-i64 i))) then
      dma-eq-loop a b (wrapping-add-u32 i 1) n
    else False

  dma-eq-hex :: I64 -> U32 -> I64 -> String ->{MVar} Bool
  dma-eq-hex actual actual-len tmp-hex hex-str =
    let exp-len = put-hex tmp-hex hex-str in
    eq-u32 actual-len exp-len && dma-eq-loop actual tmp-hex 0 exp-len

  check-pair :: I64 -> I64 -> U32 -> String -> String ->{MVar} Bool
  check-pair pairs txt idx exp-name exp-val =
    let hp = qpack-pair-load pairs idx in
    let n = dma-to-str (wrapping-add-i64 txt (u32-to-i64 hp.hp-name-off)) hp.hp-name-len in
    let v = dma-to-str (wrapping-add-i64 txt (u32-to-i64 hp.hp-val-off)) hp.hp-val-len in
    str-eq n exp-name && str-eq v exp-val

  -- Test RFC 7541 Appendix C.4 Huffman vectors + full 256-byte round-trip!
  test-huffman :: I64 ->{MVar} Int
  test-huffman base =
    let src = base in
    let enc = wrapping-add-i64 base 512 in
    let dec = wrapping-add-i64 base 2048 in
    let tmp = wrapping-add-i64 base 3072 in
    -- "www.example.com" -> f1e3c2e5f23a6ba0ab90f4ff
    let l1 = put-str src "www.example.com" in
    let e1 = qpack-huffman-encode src l1 enc 0 256 in
    let d1 = qpack-huffman-decode enc 0 e1.qe-written dec 0 256 in
    if ne-u8 e1.qe-err qpack-ok || not (dma-eq-hex enc e1.qe-written tmp "f1e3c2e5f23a6ba0ab90f4ff") then 201
    else if ne-u8 d1.qs-err qpack-ok || not (str-eq (dma-to-str dec d1.qs-out-len) "www.example.com") then 202
    else
      -- "no-cache" -> a8eb10649cbf
      let l2 = put-str src "no-cache" in
      let e2 = qpack-huffman-encode src l2 enc 0 256 in
      let d2 = qpack-huffman-decode enc 0 e2.qe-written dec 0 256 in
      if ne-u8 e2.qe-err qpack-ok || not (dma-eq-hex enc e2.qe-written tmp "a8eb10649cbf") then 203
      else if ne-u8 d2.qs-err qpack-ok || not (str-eq (dma-to-str dec d2.qs-out-len) "no-cache") then 204
      else
        -- "custom-key" -> 25a849e95ba97d7f
        let l3 = put-str src "custom-key" in
        let e3 = qpack-huffman-encode src l3 enc 0 256 in
        let d3 = qpack-huffman-decode enc 0 e3.qe-written dec 0 256 in
        if ne-u8 e3.qe-err qpack-ok || not (dma-eq-hex enc e3.qe-written tmp "25a849e95ba97d7f") then 205
        else if ne-u8 d3.qs-err qpack-ok || not (str-eq (dma-to-str dec d3.qs-out-len) "custom-key") then 206
        else
          -- All 256 byte values 0..255 round-trip!
          let _ = fill-bytes256 src 0 in
          let e4 = qpack-huffman-encode src 256 enc 0 1024 in
          let d4 = qpack-huffman-decode enc 0 e4.qe-written dec 0 512 in
          if ne-u8 e4.qe-err qpack-ok || ne-u8 d4.qs-err qpack-ok || ne-u32 d4.qs-out-len 256 then 207
          else if not (dma-eq-loop src dec 0 256) then 208
          else 0

  fill-bytes256 :: I64 -> U32 ->{MVar} Unit
  fill-bytes256 dst i =
    if ge-u32 i 256 then ()
    else
      let _ = raw-store-u8 (wrapping-add-i64 dst (u32-to-i64 i)) (wrapping-u32-to-u8 i) in
      fill-bytes256 dst (wrapping-add-u32 i 1)

  -- Test RFC 9204 §4.5.1.1 wraparound example:
  -- max_table_cap = 100 (MaxEntries=3, FullRange=6), total_inserts=10, enc_ric=4 -> ric=9;
  -- S=1, delta=2 -> base=6.
  test-prefix-wraparound :: I64 ->{MVar} Int
  test-prefix-wraparound buf =
    let er = qpack-encode-prefix buf 0 32 100 9 6 in
    if ne-u8 er.qe-err qpack-ok || ne-u32 er.qe-written 2 then 220
    else if ne-u8 (raw-load-u8 buf) 4 || ne-u8 (raw-load-u8 (wrapping-add-i64 buf 1)) 130 then 221
    else
      let dr = qpack-decode-prefix buf 0 2 100 10 in
      if ne-u8 dr.qp-err qpack-ok || ne-u64 dr.qp-req-insert-count 9 || ne-u64 dr.qp-base 6 then 222
      else 0

  -- Test RFC 9204 Appendix B.1..B.5 exact wire bytes!
  test-appendix-b :: I64 ->{MVar} Int
  test-appendix-b base =
    let wire = base in
    let txt = wrapping-add-i64 base 1024 in
    let pairs = wrapping-add-i64 base 2048 in
    let tmp = wrapping-add-i64 base 3072 in
    let src = wrapping-add-i64 base 3584 in

    -- =====================================================================
    -- B.1: Literal Field Line with Name Reference (Static, Zero-HOL)
    -- Wire: 0000 510b 2f696e6465782e68746d6c  (:path = /index.html)
    -- =====================================================================
    let n0-len = put-str src ":path" in
    let v0-len = put-str (wrapping-add-i64 src 32) "/index.html" in
    let _ = qpack-pair-store pairs 0 (header-pair-make hdr-path 0 n0-len 32 v0-len) in
    let b1-enc = qpack-encode-section-zero-hol pairs 1 src qpack-huff-never wire 256 in
    if ne-u8 b1-enc.qe-err qpack-ok then 10
    else if not (dma-eq-hex wire b1-enc.qe-written tmp "0000510b2f696e6465782e68746d6c") then 11
    else
      let b1-dec = qpack-decode-section-zero-hol wire b1-enc.qe-written txt 512 pairs 16 in
      if ne-u8 b1-dec.qd-err qpack-ok || ne-u32 b1-dec.qd-header-count 1 then 12
      else if not (check-pair pairs txt 0 ":path" "/index.html") then 13
      else
        -- Also test Zero-HOL N=1 sensitivity protection for Authorization / Cookie / Set-Cookie
        let na = put-str src "authorization" in
        let va = put-str (wrapping-add-i64 src 32) "Bearer s3cr3t" in
        let _ = qpack-pair-store pairs 0 (header-pair-make hdr-authorization 0 na 32 va) in
        let ns-enc = qpack-encode-section-zero-hol pairs 1 src qpack-huff-never wire 256 in
        -- Byte 2 must have 01 N=1 T=1 prefix: (b2 & 0xF0) == 0x70 (112)
        let b2 = raw-load-u8 (wrapping-add-i64 wire 2) in
        let ns-dec = qpack-decode-section-zero-hol wire ns-enc.qe-written txt 512 pairs 16 in
        let hp-auth = qpack-pair-load pairs 0 in
        if ne-u8 ns-enc.qe-err qpack-ok || ne-u8 (bit-and-u8 b2 240) 112 then 14
        else if ne-u8 ns-dec.qd-err qpack-ok || ne-u8 hp-auth.hp-pad0 1 then 15
        else if not (check-pair pairs txt 0 "authorization" "Bearer s3cr3t") then 16
        else 0

  test-appendix-b-dynamic :: QpackDynamicTable -> I64 ->{MVar} Int
  test-appendix-b-dynamic tbl base =
    let wire = base in
    let txt = wrapping-add-i64 base 1024 in
    let pairs = wrapping-add-i64 base 2048 in
    let tmp = wrapping-add-i64 base 3072 in
    let src = wrapping-add-i64 base 3584 in

    -- =====================================================================
    -- B.2: Dynamic Table (Set Capacity=220, Insert Static 0 & 1, Stream 4, Ack)
    -- Encoder stream exact bytes:
    --   3fbd01 + c00f7777772e6578616d706c652e636f6d + c10c2f73616d706c652f70617468
    -- =====================================================================
    let r0 = qpack-enc-set-capacity wire 0 256 220 in
    let l-host = put-str src "www.example.com" in
    let r1 = qpack-enc-insert-name-ref wire r0.qe-written 256 1 0 qpack-huff-never src l-host in
    let l-path = put-str (wrapping-add-i64 src 64) "/sample/path" in
    let r2 = qpack-enc-insert-name-ref wire r1.qe-written 256 1 1 qpack-huff-never (wrapping-add-i64 src 64) l-path in
    if ne-u8 r0.qe-err qpack-ok || ne-u8 r1.qe-err qpack-ok || ne-u8 r2.qe-err qpack-ok then 20
    else if not (dma-eq-hex wire r2.qe-written tmp "3fbd01c00f7777772e6578616d706c652e636f6dc10c2f73616d706c652f70617468") then 21
    else
      let p2 = qpack-process-encoder-stream tbl wire 0 r2.qe-written in
      if ne-u8 p2.qe-err qpack-ok || ne-u64 (qpack-table-capacity tbl) 220
        || ne-u64 (qpack-table-size tbl) 106 || ne-u64 (qpack-table-insert-count tbl) 2 then 22
      else
        -- Stream 4: 0381 10 11 (RIC=2, Base=0, Post-Base 0 & 1)
        let s0 = qpack-encode-prefix wire 0 256 220 2 0 in
        let s1 = qpack-encode-indexed-post-base wire s0.qe-written 256 0 in
        let s2 = qpack-encode-indexed-post-base wire s1.qe-written 256 1 in
        if ne-u8 s2.qe-err qpack-ok || not (dma-eq-hex wire s2.qe-written tmp "03811011") then 23
        else
          let d2 = qpack-decode-section tbl wire s2.qe-written txt 512 pairs 16 in
          if ne-u8 d2.qd-err qpack-ok || ne-u32 d2.qd-header-count 2
            || ne-u64 d2.qd-req-insert-count 2 || ne-u64 d2.qd-base 0 then 24
          else if not (check-pair pairs txt 0 ":authority" "www.example.com") then 25
          else if not (check-pair pairs txt 1 ":path" "/sample/path") then 26
          else
            -- Decoder stream: 84 (Section Acknowledgment stream=4)
            let ack-e = qpack-dec-section-ack wire 0 32 4 in
            let ack-d = qpack-decode-decoder-inst wire 0 ack-e.qe-written in
            let ack-a = qpack-table-ack-ric tbl d2.qd-req-insert-count in
            if ne-u8 ack-e.qe-err qpack-ok || not (dma-eq-hex wire ack-e.qe-written tmp "84") then 27
            else if ne-u8 ack-d.qdi-err qpack-ok || ne-u8 ack-d.qdi-kind qpack-dinst-section-ack
              || ne-u64 ack-d.qdi-val 4 || ne-u8 ack-a qpack-ok
              || ne-u64 (qpack-table-known-received tbl) 2 then 28
            else
              -- =============================================================
              -- B.3: Speculative Insert (Literal Name) + Insert Count Increment
              -- Encoder: 4a637573746f6d2d6b65790c637573746f6d2d76616c7565
              -- Decoder: 01
              -- =============================================================
              let lk = put-str src "custom-key" in
              let lv = put-str (wrapping-add-i64 src 64) "custom-value" in
              let b3-e = qpack-enc-insert-literal wire 0 256 qpack-huff-never src lk qpack-huff-never (wrapping-add-i64 src 64) lv in
              if ne-u8 b3-e.qe-err qpack-ok
                || not (dma-eq-hex wire b3-e.qe-written tmp "4a637573746f6d2d6b65790c637573746f6d2d76616c7565") then 30
              else
                let b3-p = qpack-process-encoder-stream tbl wire 0 b3-e.qe-written in
                if ne-u8 b3-p.qe-err qpack-ok || ne-u64 (qpack-table-size tbl) 160
                  || ne-u64 (qpack-table-insert-count tbl) 3 then 31
                else
                  let inc-e = qpack-dec-insert-count-inc wire 0 32 1 in
                  let inc-d = qpack-decode-decoder-inst wire 0 inc-e.qe-written in
                  let inc-a = qpack-apply-decoder-inst tbl inc-d in
                  if ne-u8 inc-e.qe-err qpack-ok || not (dma-eq-hex wire inc-e.qe-written tmp "01") then 32
                  else if ne-u8 inc-d.qdi-err qpack-ok || ne-u8 inc-d.qdi-kind qpack-dinst-insert-inc
                    || ne-u64 inc-d.qdi-val 1 || ne-u8 inc-a qpack-ok
                    || ne-u64 (qpack-table-known-received tbl) 3 then 33
                  else
                    -- Also verify Literal Field Line With Post-Base Name Reference (0000Nxxx, §4.5.5)!
                    -- With RIC=3, Base=2, post-base 0 -> abs=2 ("custom-key"), val="pb-val", N=1
                    let lpv = put-str src "pb-val" in
                    let pb0 = qpack-encode-prefix wire 0 256 220 3 2 in
                    let pb1 = qpack-encode-literal-post-base-ref wire pb0.qe-written 256 1 0 qpack-huff-never src lpv in
                    let pb-dec = qpack-decode-section tbl wire pb1.qe-written txt 512 pairs 16 in
                    let hp-pb = qpack-pair-load pairs 0 in
                    if ne-u8 pb1.qe-err qpack-ok || ne-u8 pb-dec.qd-err qpack-ok
                      || ne-u32 pb-dec.qd-header-count 1 || ne-u8 hp-pb.hp-pad0 1
                      || not (check-pair pairs txt 0 "custom-key" "pb-val") then 34
                    else
                      -- =====================================================
                      -- B.4: Duplicate (02) + Stream 8 (0500 80 c1 81) + Cancel (48)
                      -- =====================================================
                      let dup-e = qpack-enc-duplicate wire 0 32 2 in
                      if ne-u8 dup-e.qe-err qpack-ok || not (dma-eq-hex wire dup-e.qe-written tmp "02") then 40
                      else
                        let dup-p = qpack-process-encoder-stream tbl wire 0 dup-e.qe-written in
                        if ne-u8 dup-p.qe-err qpack-ok || ne-u64 (qpack-table-size tbl) 217
                          || ne-u64 (qpack-table-insert-count tbl) 4 then 41
                        else
                          let t0 = qpack-encode-prefix wire 0 256 220 4 4 in
                          let t1 = qpack-encode-indexed-dynamic wire t0.qe-written 256 0 in
                          let t2 = qpack-encode-indexed-static wire t1.qe-written 256 1 in
                          let t3 = qpack-encode-indexed-dynamic wire t2.qe-written 256 1 in
                          if ne-u8 t3.qe-err qpack-ok || not (dma-eq-hex wire t3.qe-written tmp "050080c181") then 42
                          else
                            let d4 = qpack-decode-section tbl wire t3.qe-written txt 512 pairs 16 in
                            if ne-u8 d4.qd-err qpack-ok || ne-u32 d4.qd-header-count 3
                              || ne-u64 d4.qd-req-insert-count 4 || ne-u64 d4.qd-base 4 then 43
                            else if not (check-pair pairs txt 0 ":authority" "www.example.com") then 44
                            else if not (check-pair pairs txt 1 ":path" "/") then 45
                            else if not (check-pair pairs txt 2 "custom-key" "custom-value") then 46
                            else
                              let can-e = qpack-dec-stream-cancel wire 0 32 8 in
                              let can-d = qpack-decode-decoder-inst wire 0 can-e.qe-written in
                              if ne-u8 can-e.qe-err qpack-ok || not (dma-eq-hex wire can-e.qe-written tmp "48") then 47
                              else if ne-u8 can-d.qdi-err qpack-ok || ne-u8 can-d.qdi-kind qpack-dinst-stream-cancel
                                || ne-u64 can-d.qdi-val 8 then 48
                              else
                                -- ===========================================
                                -- B.5: Dynamic Name Ref Insert + Eviction!
                                -- Wire: 810d 637573746f6d2d76616c756532
                                -- Evicts abs=0, Size=215, InsertCount=5, Dropped=1
                                -- ===========================================
                                let lv2 = put-str src "custom-value2" in
                                let b5-e = qpack-enc-insert-name-ref wire 0 256 0 1 qpack-huff-never src lv2 in
                                if ne-u8 b5-e.qe-err qpack-ok
                                  || not (dma-eq-hex wire b5-e.qe-written tmp "810d637573746f6d2d76616c756532") then 50
                                else
                                  let b5-p = qpack-process-encoder-stream tbl wire 0 b5-e.qe-written in
                                  if ne-u8 b5-p.qe-err qpack-ok
                                    || ne-u64 (qpack-table-dropped-count tbl) 1
                                    || ne-u64 (qpack-table-insert-count tbl) 5
                                    || ne-u64 (qpack-table-size tbl) 215 then 51
                                  else
                                    -- Verify newly inserted entry abs=4 decodes as custom-key=custom-value2
                                    let u0 = qpack-encode-prefix wire 0 256 220 5 5 in
                                    let u1 = qpack-encode-indexed-dynamic wire u0.qe-written 256 0 in
                                    let d5 = qpack-decode-section tbl wire u1.qe-written txt 512 pairs 16 in
                                    if ne-u8 d5.qd-err qpack-ok || ne-u32 d5.qd-header-count 1 then 52
                                    else if not (check-pair pairs txt 0 "custom-key" "custom-value2") then 53
                                    else 0

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma-u64 = engine-dma-alloc 8192 4096 in
    if eq-u64 dma-u64 0 then 1
    else
      let base = wrapping-u64-to-i64 dma-u64 in
      let r1 = test-huffman base in
      if r1 /= 0 then let _ = engine-dma-free dma-u64 8192 in r1
      else
        let r2 = test-prefix-wraparound base in
        if r2 /= 0 then let _ = engine-dma-free dma-u64 8192 in r2
        else
          let r3 = test-appendix-b base in
          if r3 /= 0 then let _ = engine-dma-free dma-u64 8192 in r3
          else
            case qpack-table-new 220 of
              None -> let _ = engine-dma-free dma-u64 8192 in 2
              Some tbl ->
                let r4 = test-appendix-b-dynamic tbl base in
                let _ = qpack-table-destroy tbl in
                let _ = engine-dma-free dma-u64 8192 in
                r4
"#;
    run_leak_checked("qpack-appendix-b", src, 0);
}

// =========================================================================
// 4. Malformed & attack rejection (bad indices, RIC, Huffman, varints, cap=0)
// =========================================================================

#[test]
fn net_http_qpack_malformed_and_attack_rejection() {
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
  import Net.Http.Qpack.StaticTable
  import Net.Http.Qpack.Codec
  import Prim.Atomic

  hex-nibble :: U8 -> U8
  hex-nibble c =
    if ge-u8 c 48 && le-u8 c 57 then wrapping-sub-u8 c 48
    else if ge-u8 c 97 && le-u8 c 102 then wrapping-add-u8 (wrapping-sub-u8 c 97) 10
    else if ge-u8 c 65 && le-u8 c 70 then wrapping-add-u8 (wrapping-sub-u8 c 65) 10
    else 0

  hex-loop :: Bytes -> I64 -> Nat -> U32 ->{MVar} U32
  hex-loop b dst i out =
    if i + 1 < bytes-len b then
      let hi = hex-nibble (bytes-get-u8 b i) in
      let lo = hex-nibble (bytes-get-u8 b (i + 1)) in
      let _ = raw-store-u8 (wrapping-add-i64 dst (u32-to-i64 out)) (bit-or-u8 (wrapping-shl-u8 hi 4) lo) in
      hex-loop b dst (i + 2) (wrapping-add-u32 out 1)
    else out

  put-hex :: I64 -> String ->{MVar} U32
  put-hex dst h = hex-loop (string-to-bytes h) dst 0 0

  run-tests :: QpackDynamicTable -> QpackDynamicTable -> I64 ->{MVar} Int
  run-tests tbl0 tbl220 base =
    let wire = base in
    let txt = wrapping-add-i64 base 1024 in
    let pairs = wrapping-add-i64 base 2048 in

    -- 1. Out-of-range static index 99 (Indexed Static: 0xFF 0x24 => 63 + 36 = 99)
    let l1 = put-hex wire "0000ff24" in
    let d1 = qpack-decode-section-zero-hol wire l1 txt 512 pairs 16 in
    let d1b = qpack-decode-section tbl220 wire l1 txt 512 pairs 16 in
    if ne-u8 d1.qd-err qpack-err-invalid-static-index
      || ne-u8 d1b.qd-err qpack-err-invalid-static-index then 10
    else
      -- Out-of-range static index 99 in Literal With Name Reference (0x5F 0x54 0x00 => 15 + 84 = 99)
      let l2 = put-hex wire "00005f5400" in
      let d2 = qpack-decode-section-zero-hol wire l2 txt 512 pairs 16 in
      if ne-u8 d2.qd-err qpack-err-invalid-static-index then 11
      else
        -- Encoder static index 99 rejected by qpack-encode-indexed-static & encoder stream
        let e-bad = qpack-encode-indexed-static wire 0 16 99 in
        let l-enc99 = put-hex wire "ff2400" in
        let p-enc99 = qpack-process-encoder-stream tbl220 wire 0 l-enc99 in
        if ne-u8 e-bad.qe-err qpack-err-invalid-static-index
          || ne-u8 p-enc99.qe-err qpack-err-encoder-stream then 12
        else
          -- 2. Bad Required Insert Count:
          -- (a) enc_ric > 2 * MaxEntries (for cap=220, FullRange=12, enc_ric=13 -> error)
          let l3 = put-hex wire "0d00" in
          let p3 = qpack-decode-prefix wire 0 l3 220 0 in
          -- (b) enc_ric != 0 when max_table_cap == 0
          let l4 = put-hex wire "0100" in
          let p4 = qpack-decode-prefix wire 0 l4 0 0 in
          let d4 = qpack-decode-section-zero-hol wire l4 txt 512 pairs 16 in
          let d4b = qpack-decode-section tbl0 wire l4 txt 512 pairs 16 in
          -- (c) Negative Base: S=1, RIC=2, DeltaBase=2 (RIC <= DeltaBase -> invalid base!)
          let l5 = put-hex wire "0382" in
          let p5 = qpack-decode-prefix wire 0 l5 220 5 in
          -- (d) Blocked stream: RIC=2 when table insert_count=0 -> qpack-err-blocked
          let l6 = put-hex wire "0300" in
          let d6 = qpack-decode-section tbl220 wire l6 txt 512 pairs 16 in
          if ne-u8 p3.qp-err qpack-err-invalid-req-insert-count then 20
          else if ne-u8 p4.qp-err qpack-err-invalid-req-insert-count
            || ne-u8 d4.qd-err qpack-err-invalid-req-insert-count
            || ne-u8 d4b.qd-err qpack-err-invalid-req-insert-count then 21
          else if ne-u8 p5.qp-err qpack-err-invalid-base then 22
          else if ne-u8 d6.qd-err qpack-err-blocked then 23
          else
            -- 3. Bad Huffman padding & EOS attacks:
            -- (a) Non-1 padding bit at end of "www.example.com" (..f4fe instead of ..f4ff)
            let lh1 = put-hex wire "f1e3c2e5f23a6ba0ab90f4fe" in
            let dh1 = qpack-huffman-decode wire 0 lh1 txt 0 256 in
            -- (b) 8 bits of 1-padding (`0xFF`)
            let lh2 = put-hex wire "ff" in
            let dh2 = qpack-huffman-decode wire 0 lh2 txt 0 256 in
            -- (c) EOS symbol in stream (`0xFFFFFFFF`)
            let lh3 = put-hex wire "ffffffff" in
            let dh3 = qpack-huffman-decode wire 0 lh3 txt 0 256 in
            -- (d) '0' (00000) followed by 000 non-1 padding (`0x00`)
            let lh4 = put-hex wire "00" in
            let dh4 = qpack-huffman-decode wire 0 lh4 txt 0 256 in
            if ne-u8 dh1.qs-err qpack-err-huffman-invalid then 30
            else if ne-u8 dh2.qs-err qpack-err-huffman-invalid then 31
            else if ne-u8 dh3.qs-err qpack-err-huffman-invalid then 32
            else if ne-u8 dh4.qs-err qpack-err-huffman-invalid then 33
            else
              -- 4. Varint overflow & overlong continuation DoS attacks:
              -- (a) > 62-bit value: 0xFF + 8x 0xFF + 0x40 (bit 62 set)
              let lv1 = put-hex wire "ffffffffffffffffff40" in
              let iv1 = qpack-decode-int wire 0 lv1 8 in
              -- (b) Overlong zero-padded 10 continuation bytes (shift reaches 63!)
              let lv2 = put-hex wire "ff80808080808080808000" in
              let iv2 = qpack-decode-int wire 0 lv2 8 in
              -- (c) Truncated continuation byte
              let lv3 = put-hex wire "ff80" in
              let iv3 = qpack-decode-int wire 0 lv3 8 in
              -- (d) Max legal 62-bit integer MUST succeed!
              let e-max = qpack-encode-int wire 0 32 0 8 4611686018427387903 in
              let d-max = qpack-decode-int wire 0 e-max.qe-written 8 in
              if ne-u8 iv1.qi-err qpack-err-int-overflow then 40
              else if ne-u8 iv2.qi-err qpack-err-int-overflow then 41
              else if ne-u8 iv3.qi-err qpack-err-truncated then 42
              else if ne-u8 e-max.qe-err qpack-ok || ne-u8 d-max.qi-err qpack-ok
                || ne-u64 d-max.qi-val 4611686018427387903 then 43
              else
                -- 5. max_table_cap == 0 rejecting ALL dynamic references & encoder instructions!
                -- (a) Indexed Dynamic (`0000 80`)
                let lz1 = put-hex wire "000080" in
                let dz1 = qpack-decode-section-zero-hol wire lz1 txt 512 pairs 16 in
                let dz1b = qpack-decode-section tbl0 wire lz1 txt 512 pairs 16 in
                -- (b) Literal Name Ref Dynamic (`0000 40 00`)
                let lz2 = put-hex wire "00004000" in
                let dz2 = qpack-decode-section-zero-hol wire lz2 txt 512 pairs 16 in
                let dz2b = qpack-decode-section tbl0 wire lz2 txt 512 pairs 16 in
                -- (c) Post-Base Indexed (`0000 10`)
                let lz3 = put-hex wire "000010" in
                let dz3 = qpack-decode-section-zero-hol wire lz3 txt 512 pairs 16 in
                let dz3b = qpack-decode-section tbl0 wire lz3 txt 512 pairs 16 in
                -- (d) Post-Base Literal Name Ref (`0000 00 00`)
                let lz4 = put-hex wire "00000000" in
                let dz4 = qpack-decode-section-zero-hol wire lz4 txt 512 pairs 16 in
                let dz4b = qpack-decode-section tbl0 wire lz4 txt 512 pairs 16 in
                -- (e) Encoder stream instruction on max_table_cap == 0 table
                let lz5 = put-hex wire "20" in
                let pz5 = qpack-process-encoder-stream tbl0 wire 0 lz5 in
                if ne-u8 dz1.qd-err qpack-err-decompression-failed
                  || ne-u8 dz1b.qd-err qpack-err-decompression-failed then 50
                else if ne-u8 dz2.qd-err qpack-err-decompression-failed
                  || ne-u8 dz2b.qd-err qpack-err-decompression-failed then 51
                else if ne-u8 dz3.qd-err qpack-err-decompression-failed
                  || ne-u8 dz3b.qd-err qpack-err-decompression-failed then 52
                else if ne-u8 dz4.qd-err qpack-err-decompression-failed
                  || ne-u8 dz4b.qd-err qpack-err-decompression-failed then 53
                else if ne-u8 pz5.qe-err qpack-err-encoder-stream then 54
                else
                  -- 6. Encoder & Decoder stream error checks:
                  -- (a) Capacity > max_cap (221 > 220)
                  let ec1 = qpack-table-set-capacity tbl220 221 in
                  -- (b) Decoder Insert Count Increment == 0 (`0x00`)
                  let ld1 = put-hex wire "00" in
                  let dd1 = qpack-decode-decoder-inst wire 0 ld1 in
                  -- (c) Decoder Insert Count Increment > insert_count
                  let ld2 = put-hex wire "01" in
                  let dd2 = qpack-decode-decoder-inst wire 0 ld2 in
                  let da2 = qpack-apply-decoder-inst tbl220 dd2 in
                  if ne-u8 ec1 qpack-err-encoder-stream then 60
                  else if ne-u8 dd1.qdi-err qpack-err-decoder-stream then 61
                  else if ne-u8 da2 qpack-err-decoder-stream then 62
                  else 0

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma-u64 = engine-dma-alloc 4096 4096 in
    if eq-u64 dma-u64 0 then 1
    else
      let base = wrapping-u64-to-i64 dma-u64 in
      case qpack-table-new 0 of
        None -> let _ = engine-dma-free dma-u64 4096 in 2
        Some tbl0 ->
          case qpack-table-new 220 of
            None ->
              let _ = qpack-table-destroy tbl0 in
              let _ = engine-dma-free dma-u64 4096 in
              3
            Some tbl220 ->
              let rc = run-tests tbl0 tbl220 base in
              let _ = qpack-table-destroy tbl220 in
              let _ = qpack-table-destroy tbl0 in
              let _ = engine-dma-free dma-u64 4096 in
              rc
"#;
    run_leak_checked("qpack-malformed-attacks", src, 0);
}

// =========================================================================
// 5. Steady-state 50,000 Zero-HOL encode + decode iterations:
//    allocs == 0, atomic_rc == 0, CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1 (leaked=0)
// =========================================================================

#[test]
fn net_http_qpack_50k_zero_hol_zero_alloc_zero_atomic_rc() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.String
  import Engine.Types
  import Net.Http.Types
  import Net.Http.Qpack.StaticTable
  import Net.Http.Qpack.Codec
  import Prim.Atomic

  copy-b2d :: Bytes -> I64 -> Nat -> Nat ->{MVar} Unit
  copy-b2d b dst i n =
    if i < n && i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 dst (nat-to-i64 i)) (bytes-get-u8 b i) in
      copy-b2d b dst (i + 1) n
    else ()

  put-str-at :: I64 -> U32 -> String ->{MVar} U32
  put-str-at base off s =
    let b = string-to-bytes s in
    let n = bytes-len b in
    let _ = copy-b2d b (wrapping-add-i64 base (u32-to-i64 off)) 0 n in
    wrapping-i64-to-u32 (nat-to-i64 n)

  -- 100% #[no-alloc] hot loop: 50,000 full zero-HOL encode + decode rounds
  -- over an 8-header HTTP/3 request (static exact + static name-ref + N=1 sensitive
  -- + literal Huffman header).
  #[no-alloc]
  bench-loop :: I64 -> I64 -> I64 -> I64 -> I64 -> U32 -> U64 ->{MVar} U64
  bench-loop in-pairs src-buf wire-buf dec-txt dec-pairs rem acc =
    if eq-u32 rem 0 then acc
    else
      let er = qpack-encode-section-zero-hol in-pairs 8 src-buf qpack-huff-auto wire-buf 1024 in
      if ne-u8 er.qe-err qpack-ok then 0
      else
        let dr = qpack-decode-section-zero-hol wire-buf er.qe-written dec-txt 1024 dec-pairs 16 in
        if ne-u8 dr.qd-err qpack-ok then 0
        else
          let hp-auth = qpack-pair-load dec-pairs 5 in
          let hp-cookie = qpack-pair-load dec-pairs 6 in
          -- Accumulate header count (8) + auth N-bit (1) + cookie N-bit (1) + encoded len
          let step = wrapping-add-u64 (wrapping-add-u64 (u32-to-u64 dr.qd-header-count) (u8-to-u64 hp-auth.hp-pad0)) (wrapping-add-u64 (u8-to-u64 hp-cookie.hp-pad0) (u32-to-u64 er.qe-written)) in
          bench-loop in-pairs src-buf wire-buf dec-txt dec-pairs (wrapping-sub-u32 rem 1) (wrapping-add-u64 acc step)

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma-u64 = engine-dma-alloc 8192 4096 in
    if eq-u64 dma-u64 0 then 1
    else
      let base = wrapping-u64-to-i64 dma-u64 in
      -- Layout inside 8 KiB DMA slab:
      --   +0    : src-buf   (1024B)
      --   +1024 : in-pairs  (512B)
      --   +2048 : wire-buf  (1024B)
      --   +3072 : dec-txt   (1024B)
      --   +4096 : dec-pairs (512B)
      let src-buf = base in
      let in-pairs = wrapping-add-i64 base 1024 in
      let wire-buf = wrapping-add-i64 base 2048 in
      let dec-txt = wrapping-add-i64 base 3072 in
      let dec-pairs = wrapping-add-i64 base 4096 in

      -- Populate 8 realistic HTTP/3 request headers:
      -- 0: :method = GET (exact static 17)
      let n0 = put-str-at src-buf 0 ":method" in
      let v0 = put-str-at src-buf 16 "GET" in
      let _ = qpack-pair-store in-pairs 0 (header-pair-make hdr-method 0 n0 16 v0) in
      -- 1: :scheme = https (exact static 23)
      let n1 = put-str-at src-buf 32 ":scheme" in
      let v1 = put-str-at src-buf 48 "https" in
      let _ = qpack-pair-store in-pairs 1 (header-pair-make hdr-scheme 32 n1 48 v1) in
      -- 2: :path = / (exact static 1)
      let n2 = put-str-at src-buf 64 ":path" in
      let v2 = put-str-at src-buf 80 "/" in
      let _ = qpack-pair-store in-pairs 2 (header-pair-make hdr-path 64 n2 80 v2) in
      -- 3: :authority = api.cinch.dev (static name ref 0 + Huffman value)
      let n3 = put-str-at src-buf 96 ":authority" in
      let v3 = put-str-at src-buf 112 "api.cinch.dev" in
      let _ = qpack-pair-store in-pairs 3 (header-pair-make hdr-authority 96 n3 112 v3) in
      -- 4: accept-encoding = gzip, deflate, br (exact static 31)
      let n4 = put-str-at src-buf 144 "accept-encoding" in
      let v4 = put-str-at src-buf 168 "gzip, deflate, br" in
      let _ = qpack-pair-store in-pairs 4 (header-pair-make hdr-accept-encoding 144 n4 168 v4) in
      -- 5: authorization = Bearer tok-abc123 (static name ref 84, N=1 sensitive!)
      let n5 = put-str-at src-buf 200 "authorization" in
      let v5 = put-str-at src-buf 224 "Bearer tok-abc123" in
      let _ = qpack-pair-store in-pairs 5 (header-pair-make hdr-authorization 200 n5 224 v5) in
      -- 6: cookie = sid=xyz987654 (static name ref 5, N=1 sensitive!)
      let n6 = put-str-at src-buf 256 "cookie" in
      let v6 = put-str-at src-buf 272 "sid=xyz987654" in
      let _ = qpack-pair-store in-pairs 6 (header-pair-make hdr-cookie 256 n6 272 v6) in
      -- 7: x-request-id = req-987654321 (literal name + Huffman value)
      let n7 = put-str-at src-buf 304 "x-request-id" in
      let v7 = put-str-at src-buf 328 "req-987654321" in
      let _ = qpack-pair-store in-pairs 7 (header-pair-make hdr-unknown 304 n7 328 v7) in

      -- Warmup 1 iteration to read exact wire size, then reset counters for 50,000 iters!
      let w = qpack-encode-section-zero-hol in-pairs 8 src-buf qpack-huff-auto wire-buf 1024 in
      let exp-step = wrapping-add-u64 10 (u32-to-u64 w.qe-written) in
      let exp-total = wrapping-mul-u64 exp-step 50000 in

      let _ = engine-thread-counters-reset () in
      let acc = bench-loop in-pairs src-buf wire-buf dec-txt dec-pairs 50000 0 in
      let allocs = engine-thread-alloc-count () in
      let atomic-rcs = engine-thread-atomic-rc-count () in
      let _ = engine-dma-free dma-u64 8192 in

      if ne-u8 w.qe-err qpack-ok then 2
      else if ne-u64 acc exp-total then 10
      else if ne-u64 allocs 0 then 11
      else if ne-u64 atomic-rcs 0 then 12
      else 0
"#;
    let (_stdout, elapsed) = run_leak_checked("qpack-50k-zero-hol", src, 0);
    let secs = elapsed.as_secs_f64();
    let ops_per_sec = 50_000.0 / secs.max(1e-9);
    eprintln!(
        "Net.Http.Qpack 50,000 zero-HOL encode+decode rounds (incl. process+setup): {:.2} ms ({:.0} rounds/s), allocs=0, atomic_rcs=0, leaked=0",
        secs * 1000.0,
        ops_per_sec
    );
}
