//! Integration, RFC 7541 Appendix A/B/C exact wire-byte compliance, ABI layout,
//! `#[no-alloc]` verification, strict error/padding/overflow rejection, automatic
//! Never-Indexed policy, and 50,000-iteration steady-state zero-alloc / zero-atomic-RC
//! / leak-checked tests for `Net.Http.Hpack.{StaticTable,Huffman,Codec}`.
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
const HPACK_STATIC: &str = include_str!("../prelude/Net/Http/Hpack/StaticTable.cinch");
const HPACK_HUFFMAN: &str = include_str!("../prelude/Net/Http/Hpack/Huffman.cinch");
const HPACK_CODEC: &str = include_str!("../prelude/Net/Http/Hpack/Codec.cinch");
const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn project_with_hpack(tag: &str, main_src: &str) -> std::path::PathBuf {
    common::project(
        tag,
        &[
            ("Prim.Atomic.cinch", PRIM_ATOMIC),
            ("Engine.Types.cinch", ENGINE_TYPES),
            ("Net.Http.Types.cinch", NET_HTTP_TYPES),
            ("Net.Http.Hpack.StaticTable.cinch", HPACK_STATIC),
            ("Net.Http.Hpack.Huffman.cinch", HPACK_HUFFMAN),
            ("Net.Http.Hpack.Codec.cinch", HPACK_CODEC),
            ("Main.cinch", main_src),
        ],
    )
}

fn run_leak_checked(tag: &str, src: &str, expected: i32) -> (String, std::time::Duration) {
    let dir = project_with_hpack(tag, src);
    let exe = common::build_executable(&dir, "Main.cinch", "net-http-hpack-test")
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
// 1. Manifest registration, direct stdlib resolution, ABI layouts & #[no-alloc]
// =========================================================================

#[test]
fn net_http_hpack_manifest_abi_and_no_alloc_check() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, path) in [
        ("Net.Http.Hpack.StaticTable", "Net/Http/Hpack/StaticTable.cinch"),
        ("Net.Http.Hpack.Huffman", "Net/Http/Hpack/Huffman.cinch"),
        ("Net.Http.Hpack.Codec", "Net/Http/Hpack/Codec.cinch"),
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

    // Direct stdlib resolution without copying sources into the project directory.
    let dir_std = common::project(
        "hpack-stdlib-resolve",
        &[(
            "Main.cinch",
            r#"
module Main where
  import Data.Machine
  import Net.Http.Hpack.StaticTable
  import Net.Http.Hpack.Huffman
  import Net.Http.Hpack.Codec

  main :: Unit ->{IO, MVar} Int
  main _ =
    let enc = hpack-encoder-new hpack-default-table-size hpack-str-auto in
    if not (hpack-encoder-valid enc) then 1
    else
      let dec = hpack-decoder-new hpack-default-table-size in
      if not (hpack-decoder-valid dec) then
        let _ = hpack-encoder-destroy enc in
        2
      else
        let _ = hpack-decoder-destroy dec in
        let _ = hpack-encoder-destroy enc in
        if eq-u32 hpack-static-table-len 61 && eq-u8 hpack-huffman-eos-len 30 then 0 else 3
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
    .expect("direct stdlib resolution of Net.Http.Hpack.*");
    common::cleanup(&dir_std);
    assert_eq!(code, 0);

    // ABI layout & #[no-alloc] interface assertions across all 3 HPACK modules.
    let src = r#"
module Main where
  import Data.Machine
  import Net.Http.Hpack.StaticTable
  import Net.Http.Hpack.Huffman
  import Net.Http.Hpack.Codec

  main :: Int
  main =
    let sm = hpack-static-match-none () in
    let hh = hpack-header-empty () in
    let de = hpack-dyn-entry-empty () in
    if eq-u32 sm.sm-index 0 && eq-u32 hh.hh-name-len 0 && eq-i64 de.de-name-addr 0 then 0 else 1
"#;
    let dir = project_with_hpack("hpack-abi-no-alloc", src);
    let ifaces = common::check_interfaces(&dir, "Main.cinch").expect("check_interfaces");

    let iface_of = |name: &str| {
        ifaces
            .iter()
            .find(|i| i.module == name)
            .unwrap_or_else(|| panic!("missing interface for {name}"))
    };
    let static_iface = iface_of("Net.Http.Hpack.StaticTable");
    let huff_iface = iface_of("Net.Http.Hpack.Huffman");
    let codec_iface = iface_of("Net.Http.Hpack.Codec");

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

    assert_eq!(layout_of(static_iface, "HpackStaticMatch").size, 8);
    assert_eq!(layout_of(huff_iface, "HpackHuffmanTable").size, 16);
    assert_eq!(layout_of(huff_iface, "HpackHuffmanResult").size, 8);
    assert_eq!(layout_of(codec_iface, "HpackIntResult").size, 16);
    assert_eq!(layout_of(codec_iface, "HpackWriteResult").size, 8);
    assert_eq!(layout_of(codec_iface, "HpackHeader").size, 20);
    assert_eq!(layout_of(codec_iface, "HpackDecodeResult").size, 16);
    assert_eq!(layout_of(codec_iface, "HpackDynEntry").size, 24);
    assert_eq!(layout_of(codec_iface, "HpackDynamicTable").size, 16);
    assert_eq!(layout_of(codec_iface, "HpackDecoder").size, 32);
    assert_eq!(layout_of(codec_iface, "HpackEncoder").size, 32);

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
        "hpack-static-name",
        "hpack-static-value",
        "hpack-static-name-len",
        "hpack-static-value-len",
        "hpack-static-from-known",
        "hpack-static-to-known",
        "hpack-static-write-name-dma",
        "hpack-static-write-value-dma",
        "hpack-static-match-known-dma",
        "hpack-static-match-dma",
        "hpack-is-sensitive-known",
        "hpack-is-sensitive-name-dma",
    ] {
        assert_no_alloc(static_iface, f);
    }

    for f in [
        "hpack-huffman-entry",
        "hpack-huffman-code",
        "hpack-huffman-len",
        "hpack-huffman-encode-len",
        "hpack-huffman-encode-len-pure",
        "hpack-huffman-encode-raw",
        "hpack-huffman-decode-raw",
    ] {
        assert_no_alloc(huff_iface, f);
    }

    for f in [
        "hpack-encode-int-raw",
        "hpack-decode-int-raw",
        "hpack-decode-int-u32-raw",
        "hpack-dyn-set-max-size",
        "hpack-dyn-insert",
        "hpack-dyn-get",
        "hpack-dyn-find",
        "hpack-encode-string-raw",
        "hpack-encode-indexed",
        "hpack-encode-table-size-update",
        "hpack-encode-header",
        "hpack-encode-header-with-mode",
        "hpack-decode-block",
    ] {
        assert_no_alloc(codec_iface, f);
    }

    assert_eq!(common::run(&dir, "Main.cinch"), 0);
    common::cleanup(&dir);
}

// =========================================================================
// 2. All RFC 7541 Appendix A (61 entries) & Appendix C (C.1 .. C.6) Vectors
// =========================================================================

#[test]
fn net_http_hpack_rfc7541_appendix_c_exact_wire_vectors() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Core
  import Data.Machine
  import Data.String
  import Engine.Types
  import Net.Http.Types
  import Net.Http.Hpack.StaticTable
  import Net.Http.Hpack.Huffman
  import Net.Http.Hpack.Codec
  import Prim.Atomic

  -- Parse a hex nibble `'0'..'9'` / `'a'..'f'` / `'A'..'F'`.
  hex-nib :: U8 -> U8
  hex-nib c =
    if ge-u8 c 48 && le-u8 c 57 then wrapping-sub-u8 c 48
    else if ge-u8 c 97 && le-u8 c 102 then wrapping-add-u8 (wrapping-sub-u8 c 97) 10
    else wrapping-add-u8 (wrapping-sub-u8 c 65) 10

  write-hex-loop :: Bytes -> I64 -> I64 -> I64 -> U32 ->{MVar} U32
  write-hex-loop b i n dst out =
    let i1 = wrapping-add-i64 i 1 in
    if 0 <= i && i < bytes-len-i64 b && 0 <= i1 && i1 < bytes-len-i64 b && lt-i64 i1 n then
      let hi = hex-nib (bytes-get-u8-at b i) in
      let lo = hex-nib (bytes-get-u8-at b i1) in
      let byte = bit-or-u8 (wrapping-shl-u8 hi 4) lo in
      let _ = raw-store-u8 (wrapping-add-i64 dst (u32-to-i64 out)) byte in
      write-hex-loop b (wrapping-add-i64 i 2) n dst (wrapping-add-u32 out 1)
    else out

  write-hex-to-dma :: I64 -> String ->{MVar} U32
  write-hex-to-dma dst s =
    let b = string-to-bytes s in
    write-hex-loop b 0 (bytes-len-i64 b) dst 0

  write-str-loop :: Bytes -> I64 -> I64 -> I64 ->{MVar} U32
  write-str-loop b i n dst =
    if 0 <= i && i < bytes-len-i64 b && lt-i64 i n then
      let _ = raw-store-u8 (wrapping-add-i64 dst i) (bytes-get-u8-at b i) in
      write-str-loop b (wrapping-add-i64 i 1) n dst
    else wrapping-i64-to-u32 n

  write-str-to-dma :: I64 -> String ->{MVar} U32
  write-str-to-dma dst s =
    let b = string-to-bytes s in
    write-str-loop b 0 (bytes-len-i64 b) dst

  dma-eq-loop :: I64 -> I64 -> U32 -> U32 ->{MVar} Bool
  dma-eq-loop a b i n =
    if ge-u32 i n then True
    else if eq-u8 (raw-load-u8 (wrapping-add-i64 a (u32-to-i64 i))) (raw-load-u8 (wrapping-add-i64 b (u32-to-i64 i))) then
      dma-eq-loop a b (wrapping-add-u32 i 1) n
    else False

  dma-eq-str :: I64 -> U32 -> String -> I64 ->{MVar} Bool
  dma-eq-str addr len exp-s tmp =
    let elen = write-str-to-dma tmp exp-s in
    if ne-u32 len elen then False
    else dma-eq-loop addr tmp 0 len

  dma-eq-hex :: I64 -> U32 -> String -> I64 ->{MVar} Bool
  dma-eq-hex addr len exp-hex tmp =
    let elen = write-hex-to-dma tmp exp-hex in
    if ne-u32 len elen then False
    else dma-eq-loop addr tmp 0 len

  hdr-eq :: I64 -> I64 -> U32 -> String -> String -> U8 -> I64 ->{MVar} Bool
  hdr-eq hdrs-addr text-addr idx exp-name exp-val exp-flags tmp =
    let h = hpack-header-read-at hdrs-addr idx in
    let n-ok = dma-eq-str (wrapping-add-i64 text-addr (u32-to-i64 h.hh-name-off)) h.hh-name-len exp-name tmp in
    let v-ok = dma-eq-str (wrapping-add-i64 text-addr (u32-to-i64 h.hh-val-off)) h.hh-val-len exp-val tmp in
    n-ok && v-ok && eq-u8 h.hh-flags exp-flags

  dyn-eq :: HpackDynamicTable -> U32 -> String -> String -> I64 ->{MVar} Bool
  dyn-eq dyn idx exp-name exp-val tmp =
    let e = hpack-dyn-get dyn idx in
    if not (hpack-dyn-entry-valid e) then False
    else
      let n-ok = dma-eq-str e.de-name-addr e.de-name-len exp-name tmp in
      let v-ok = dma-eq-str e.de-val-addr e.de-val-len exp-val tmp in
      n-ok && v-ok

  enc-hdr :: HpackEncoder -> U8 -> String -> String -> I64 -> U32 -> U32 -> I64 ->{MVar} HpackWriteResult
  enc-hdr enc mode name val dst pos cap scratch =
    let n-addr = scratch in
    let v-addr = wrapping-add-i64 scratch 256 in
    let n-len = write-str-to-dma n-addr name in
    let v-len = write-str-to-dma v-addr val in
    hpack-encode-header-with-mode enc mode n-addr n-len v-addr v-len dst pos cap

  -- Verify all 61 static table entries (names, values, DMA writers, KnownHeader mapping)
  verify-static-loop :: U32 -> I64 -> I64 ->{MVar} Bool
  verify-static-loop idx buf tmp =
    if gt-u32 idx 61 then True
    else
      let sname = hpack-static-name idx in
      let sval = hpack-static-value idx in
      let nlen = hpack-static-write-name-dma idx buf 128 in
      let vlen = hpack-static-write-value-dma idx (wrapping-add-i64 buf 128) 128 in
      let k = hpack-static-to-known idx in
      let back = hpack-static-from-known k in
      let sm = hpack-static-match-dma buf nlen (wrapping-add-i64 buf 128) vlen in
      let ok = (
        eq-u32 nlen (hpack-static-name-len idx)
        && eq-u32 vlen (hpack-static-value-len idx)
        && dma-eq-str buf nlen sname tmp
        && dma-eq-str (wrapping-add-i64 buf 128) vlen sval tmp
        && ne-u8 k hdr-unknown
        && ne-u32 back 0
        && eq-u32 sm.sm-index idx
        && eq-u8 sm.sm-exact 1) in
      if ok then verify-static-loop (wrapping-add-u32 idx 1) buf tmp
      else False

  -- RFC 7541 Appendix C.1: Prefix integer examples (C.1.1, C.1.2, C.1.3)
  test-c1 :: I64 -> I64 ->{MVar} Bool
  test-c1 buf tmp =
    -- C.1.1: 10 with 5-bit prefix => 0a
    let w1 = hpack-encode-int-raw buf 0 64 5 0 10 in
    let d1 = hpack-decode-int-raw buf 0 w1.hw-next-pos 5 in
    let ok1 = (eq-u32 w1.hw-status hpack-ok && eq-u32 w1.hw-next-pos 1
      && dma-eq-hex buf 1 "0a" tmp
      && eq-u32 d1.hi-status hpack-ok && eq-u32 d1.hi-next-pos 1 && eq-u64 d1.hi-value 10) in
    -- C.1.2: 1337 with 5-bit prefix => 1f 9a 0a
    let w2 = hpack-encode-int-raw buf 0 64 5 0 1337 in
    let d2 = hpack-decode-int-raw buf 0 w2.hw-next-pos 5 in
    let ok2 = (eq-u32 w2.hw-status hpack-ok && eq-u32 w2.hw-next-pos 3
      && dma-eq-hex buf 3 "1f9a0a" tmp
      && eq-u32 d2.hi-status hpack-ok && eq-u32 d2.hi-next-pos 3 && eq-u64 d2.hi-value 1337) in
    -- C.1.3: 42 with 8-bit prefix => 2a
    let w3 = hpack-encode-int-raw buf 0 64 8 0 42 in
    let d3 = hpack-decode-int-raw buf 0 w3.hw-next-pos 8 in
    let ok3 = (eq-u32 w3.hw-status hpack-ok && eq-u32 w3.hw-next-pos 1
      && dma-eq-hex buf 1 "2a" tmp
      && eq-u32 d3.hi-status hpack-ok && eq-u32 d3.hi-next-pos 1 && eq-u64 d3.hi-value 42) in
    ok1 && ok2 && ok3

  -- RFC 7541 Appendix C.2: Header field representation examples (C.2.1 .. C.2.4)
  test-c2 :: HpackHuffmanTable -> I64 -> I64 -> I64 -> I64 ->{IO, MVar} Bool
  test-c2 huff wire hdrs text tmp =
    let enc = hpack-encoder-new-with-huff 4096 hpack-str-raw huff in
    if not (hpack-encoder-valid enc) then False
    else
      let dec = hpack-decoder-new-with-huff 4096 huff in
      if not (hpack-decoder-valid dec) then
        let _ = hpack-encoder-destroy enc in
        False
      else
        -- C.2.1: Literal Header Field with Indexing (`custom-key: custom-header`)
        let w1 = enc-hdr enc hpack-mode-literal-incremental "custom-key" "custom-header" wire 0 256 tmp in
        let exp1 = "400a637573746f6d2d6b65790d637573746f6d2d686561646572" in
        let d1 = hpack-decode-block dec wire w1.hw-next-pos hdrs 16 text 1024 in
        let dd = hpack-decoder-dyn-table dec in
        let ok1 = (
          eq-u32 w1.hw-status hpack-ok
          && dma-eq-hex wire w1.hw-next-pos exp1 tmp
          && eq-u32 d1.hd-status hpack-ok && eq-u32 d1.hd-header-count 1
          && hdr-eq hdrs text 0 "custom-key" "custom-header" 0 tmp
          && eq-u32 (hpack-dyn-count dd) 1 && eq-u32 (hpack-dyn-size dd) 55
          && dyn-eq dd 0 "custom-key" "custom-header" tmp) in
        let _ = hpack-dyn-clear (hpack-encoder-dyn-table enc) in
        let _ = hpack-dyn-clear dd in
        -- C.2.2: Literal Header Field without Indexing (`:path: /sample/path`)
        let w2 = enc-hdr enc hpack-mode-without-indexing ":path" "/sample/path" wire 0 256 tmp in
        let exp2 = "040c2f73616d706c652f70617468" in
        let d2 = hpack-decode-block dec wire w2.hw-next-pos hdrs 16 text 1024 in
        let ok2 = (
          eq-u32 w2.hw-status hpack-ok
          && dma-eq-hex wire w2.hw-next-pos exp2 tmp
          && eq-u32 d2.hd-status hpack-ok && eq-u32 d2.hd-header-count 1
          && hdr-eq hdrs text 0 ":path" "/sample/path" 0 tmp
          && eq-u32 (hpack-dyn-count dd) 0 && eq-u32 (hpack-dyn-size dd) 0) in
        -- C.2.3: Literal Header Field Never Indexed (`password: secret`)
        let w3 = enc-hdr enc hpack-mode-never-indexed "password" "secret" wire 0 256 tmp in
        let exp3 = "100870617373776f726406736563726574" in
        let d3 = hpack-decode-block dec wire w3.hw-next-pos hdrs 16 text 1024 in
        let ok3 = (
          eq-u32 w3.hw-status hpack-ok
          && dma-eq-hex wire w3.hw-next-pos exp3 tmp
          && eq-u32 d3.hd-status hpack-ok && eq-u32 d3.hd-header-count 1
          && hdr-eq hdrs text 0 "password" "secret" hpack-flag-never-indexed tmp
          && eq-u32 (hpack-dyn-count dd) 0 && eq-u32 (hpack-dyn-size dd) 0) in
        -- C.2.4: Indexed Header Field (`:method: GET`)
        let w4 = enc-hdr enc hpack-mode-incremental ":method" "GET" wire 0 256 tmp in
        let d4 = hpack-decode-block dec wire w4.hw-next-pos hdrs 16 text 1024 in
        let ok4 = (
          eq-u32 w4.hw-status hpack-ok
          && dma-eq-hex wire w4.hw-next-pos "82" tmp
          && eq-u32 d4.hd-status hpack-ok && eq-u32 d4.hd-header-count 1
          && hdr-eq hdrs text 0 ":method" "GET" 0 tmp
          && eq-u32 (hpack-dyn-count dd) 0) in
        let _ = hpack-decoder-destroy dec in
        let _ = hpack-encoder-destroy enc in
        ok1 && ok2 && ok3 && ok4

  -- RFC 7541 Appendix C.3 (str-mode = raw) & C.4 (str-mode = huffman):
  -- Stateful 3-request sequence verifying exact wire bytes, decoded headers, and dynamic table state!
  test-c3-or-c4 :: HpackHuffmanTable -> U8 -> String -> String -> String -> I64 -> I64 -> I64 -> I64 ->{IO, MVar} Bool
  test-c3-or-c4 huff str-mode hex1 hex2 hex3 wire hdrs text tmp =
    let enc = hpack-encoder-new-with-huff 4096 str-mode huff in
    if not (hpack-encoder-valid enc) then False
    else
      let dec = hpack-decoder-new-with-huff 4096 huff in
      if not (hpack-decoder-valid dec) then
        let _ = hpack-encoder-destroy enc in
        False
      else
        let dd = hpack-decoder-dyn-table dec in
        -- Request 1
        let a0 = enc-hdr enc hpack-mode-incremental ":method" "GET" wire 0 512 tmp in
        let a1 = enc-hdr enc hpack-mode-incremental ":scheme" "http" wire a0.hw-next-pos 512 tmp in
        let a2 = enc-hdr enc hpack-mode-incremental ":path" "/" wire a1.hw-next-pos 512 tmp in
        let a3 = enc-hdr enc hpack-mode-incremental ":authority" "www.example.com" wire a2.hw-next-pos 512 tmp in
        let d1 = hpack-decode-block dec wire a3.hw-next-pos hdrs 16 text 1024 in
        let ok1 = (
          eq-u32 a3.hw-status hpack-ok
          && dma-eq-hex wire a3.hw-next-pos hex1 tmp
          && eq-u32 d1.hd-status hpack-ok && eq-u32 d1.hd-header-count 4
          && hdr-eq hdrs text 0 ":method" "GET" 0 tmp
          && hdr-eq hdrs text 1 ":scheme" "http" 0 tmp
          && hdr-eq hdrs text 2 ":path" "/" 0 tmp
          && hdr-eq hdrs text 3 ":authority" "www.example.com" 0 tmp
          && eq-u32 (hpack-dyn-count dd) 1 && eq-u32 (hpack-dyn-size dd) 57
          && dyn-eq dd 0 ":authority" "www.example.com" tmp) in
        -- Request 2
        let b0 = enc-hdr enc hpack-mode-incremental ":method" "GET" wire 0 512 tmp in
        let b1 = enc-hdr enc hpack-mode-incremental ":scheme" "http" wire b0.hw-next-pos 512 tmp in
        let b2 = enc-hdr enc hpack-mode-incremental ":path" "/" wire b1.hw-next-pos 512 tmp in
        let b3 = enc-hdr enc hpack-mode-incremental ":authority" "www.example.com" wire b2.hw-next-pos 512 tmp in
        let b4 = enc-hdr enc hpack-mode-incremental "cache-control" "no-cache" wire b3.hw-next-pos 512 tmp in
        let d2 = hpack-decode-block dec wire b4.hw-next-pos hdrs 16 text 1024 in
        let ok2 = (
          eq-u32 b4.hw-status hpack-ok
          && dma-eq-hex wire b4.hw-next-pos hex2 tmp
          && eq-u32 d2.hd-status hpack-ok && eq-u32 d2.hd-header-count 5
          && hdr-eq hdrs text 3 ":authority" "www.example.com" 0 tmp
          && hdr-eq hdrs text 4 "cache-control" "no-cache" 0 tmp
          && eq-u32 (hpack-dyn-count dd) 2 && eq-u32 (hpack-dyn-size dd) 110
          && dyn-eq dd 0 "cache-control" "no-cache" tmp
          && dyn-eq dd 1 ":authority" "www.example.com" tmp) in
        -- Request 3
        let c0 = enc-hdr enc hpack-mode-incremental ":method" "GET" wire 0 512 tmp in
        let c1 = enc-hdr enc hpack-mode-incremental ":scheme" "https" wire c0.hw-next-pos 512 tmp in
        let c2 = enc-hdr enc hpack-mode-incremental ":path" "/index.html" wire c1.hw-next-pos 512 tmp in
        let c3 = enc-hdr enc hpack-mode-incremental ":authority" "www.example.com" wire c2.hw-next-pos 512 tmp in
        let c4 = enc-hdr enc hpack-mode-incremental "custom-key" "custom-value" wire c3.hw-next-pos 512 tmp in
        let d3 = hpack-decode-block dec wire c4.hw-next-pos hdrs 16 text 1024 in
        let ok3 = (
          eq-u32 c4.hw-status hpack-ok
          && dma-eq-hex wire c4.hw-next-pos hex3 tmp
          && eq-u32 d3.hd-status hpack-ok && eq-u32 d3.hd-header-count 5
          && hdr-eq hdrs text 0 ":method" "GET" 0 tmp
          && hdr-eq hdrs text 1 ":scheme" "https" 0 tmp
          && hdr-eq hdrs text 2 ":path" "/index.html" 0 tmp
          && hdr-eq hdrs text 3 ":authority" "www.example.com" 0 tmp
          && hdr-eq hdrs text 4 "custom-key" "custom-value" 0 tmp
          && eq-u32 (hpack-dyn-count dd) 3 && eq-u32 (hpack-dyn-size dd) 164
          && dyn-eq dd 0 "custom-key" "custom-value" tmp
          && dyn-eq dd 1 "cache-control" "no-cache" tmp
          && dyn-eq dd 2 ":authority" "www.example.com" tmp) in
        let _ = hpack-decoder-destroy dec in
        let _ = hpack-encoder-destroy enc in
        ok1 && ok2 && ok3

  -- RFC 7541 Appendix C.5 (raw) & C.6 (huffman):
  -- Stateful 3-response sequence with 256-byte dynamic table & multi-entry eviction!
  test-c5-or-c6 :: HpackHuffmanTable -> U8 -> String -> String -> String -> I64 -> I64 -> I64 -> I64 ->{IO, MVar} Bool
  test-c5-or-c6 huff str-mode hex1 hex2 hex3 wire hdrs text tmp =
    let enc = hpack-encoder-new-with-huff 256 str-mode huff in
    if not (hpack-encoder-valid enc) then False
    else
      let dec = hpack-decoder-new-with-huff 256 huff in
      if not (hpack-decoder-valid dec) then
        let _ = hpack-encoder-destroy enc in
        False
      else
        let dd = hpack-decoder-dyn-table dec in
        -- Response 1 (4 headers -> table size 222)
        let a0 = enc-hdr enc hpack-mode-incremental ":status" "302" wire 0 512 tmp in
        let a1 = enc-hdr enc hpack-mode-incremental "cache-control" "private" wire a0.hw-next-pos 512 tmp in
        let a2 = enc-hdr enc hpack-mode-incremental "date" "Mon, 21 Oct 2013 20:13:21 GMT" wire a1.hw-next-pos 512 tmp in
        let a3 = enc-hdr enc hpack-mode-incremental "location" "https://www.example.com" wire a2.hw-next-pos 512 tmp in
        let d1 = hpack-decode-block dec wire a3.hw-next-pos hdrs 16 text 1024 in
        let ok1 = (
          eq-u32 a3.hw-status hpack-ok
          && dma-eq-hex wire a3.hw-next-pos hex1 tmp
          && eq-u32 d1.hd-status hpack-ok && eq-u32 d1.hd-header-count 4
          && hdr-eq hdrs text 0 ":status" "302" 0 tmp
          && hdr-eq hdrs text 1 "cache-control" "private" 0 tmp
          && hdr-eq hdrs text 2 "date" "Mon, 21 Oct 2013 20:13:21 GMT" 0 tmp
          && hdr-eq hdrs text 3 "location" "https://www.example.com" 0 tmp
          && eq-u32 (hpack-dyn-count dd) 4 && eq-u32 (hpack-dyn-size dd) 222
          && dyn-eq dd 0 "location" "https://www.example.com" tmp
          && dyn-eq dd 3 ":status" "302" tmp) in
        -- Response 2 (`:status: 307` evicts `:status: 302`; table size stays 222)
        let b0 = enc-hdr enc hpack-mode-incremental ":status" "307" wire 0 512 tmp in
        let b1 = enc-hdr enc hpack-mode-incremental "cache-control" "private" wire b0.hw-next-pos 512 tmp in
        let b2 = enc-hdr enc hpack-mode-incremental "date" "Mon, 21 Oct 2013 20:13:21 GMT" wire b1.hw-next-pos 512 tmp in
        let b3 = enc-hdr enc hpack-mode-incremental "location" "https://www.example.com" wire b2.hw-next-pos 512 tmp in
        let d2 = hpack-decode-block dec wire b3.hw-next-pos hdrs 16 text 1024 in
        let ok2 = (
          eq-u32 b3.hw-status hpack-ok
          && dma-eq-hex wire b3.hw-next-pos hex2 tmp
          && eq-u32 d2.hd-status hpack-ok && eq-u32 d2.hd-header-count 4
          && hdr-eq hdrs text 0 ":status" "307" 0 tmp
          && hdr-eq hdrs text 1 "cache-control" "private" 0 tmp
          && hdr-eq hdrs text 2 "date" "Mon, 21 Oct 2013 20:13:21 GMT" 0 tmp
          && hdr-eq hdrs text 3 "location" "https://www.example.com" 0 tmp
          && eq-u32 (hpack-dyn-count dd) 4 && eq-u32 (hpack-dyn-size dd) 222
          && dyn-eq dd 0 ":status" "307" tmp
          && dyn-eq dd 3 "cache-control" "private" tmp) in
        -- Response 3 (multi-entry eviction -> 3 entries, size 215)
        let c0 = enc-hdr enc hpack-mode-incremental ":status" "200" wire 0 512 tmp in
        let c1 = enc-hdr enc hpack-mode-incremental "cache-control" "private" wire c0.hw-next-pos 512 tmp in
        let c2 = enc-hdr enc hpack-mode-incremental "date" "Mon, 21 Oct 2013 20:13:22 GMT" wire c1.hw-next-pos 512 tmp in
        let c3 = enc-hdr enc hpack-mode-incremental "location" "https://www.example.com" wire c2.hw-next-pos 512 tmp in
        let c4 = enc-hdr enc hpack-mode-incremental "content-encoding" "gzip" wire c3.hw-next-pos 512 tmp in
        let cookie-val = "foo=ASDJKHQKBZXOQWEOPIUAXQWEOIU; max-age=3600; version=1" in
        let c5 = enc-hdr enc hpack-mode-incremental "set-cookie" cookie-val wire c4.hw-next-pos 512 tmp in
        let d3 = hpack-decode-block dec wire c5.hw-next-pos hdrs 16 text 1024 in
        let ok3 = (
          eq-u32 c5.hw-status hpack-ok
          && dma-eq-hex wire c5.hw-next-pos hex3 tmp
          && eq-u32 d3.hd-status hpack-ok && eq-u32 d3.hd-header-count 6
          && hdr-eq hdrs text 0 ":status" "200" 0 tmp
          && hdr-eq hdrs text 1 "cache-control" "private" 0 tmp
          && hdr-eq hdrs text 2 "date" "Mon, 21 Oct 2013 20:13:22 GMT" 0 tmp
          && hdr-eq hdrs text 3 "location" "https://www.example.com" 0 tmp
          && hdr-eq hdrs text 4 "content-encoding" "gzip" 0 tmp
          && hdr-eq hdrs text 5 "set-cookie" cookie-val 0 tmp
          && eq-u32 (hpack-dyn-count dd) 3 && eq-u32 (hpack-dyn-size dd) 215
          && dyn-eq dd 0 "set-cookie" cookie-val tmp
          && dyn-eq dd 1 "content-encoding" "gzip" tmp
          && dyn-eq dd 2 "date" "Mon, 21 Oct 2013 20:13:22 GMT" tmp) in
        let _ = hpack-decoder-destroy dec in
        let _ = hpack-encoder-destroy enc in
        ok1 && ok2 && ok3

  main :: Unit ->{IO, MVar} Int
  main _ =
    let huff = hpack-huffman-table-new () in
    if not (hpack-huffman-table-valid huff) then 1
    else
      let slab = engine-dma-alloc 8192 4096 in
      if eq-u64 slab 0 then
        let _ = hpack-huffman-table-destroy huff in
        2
      else
        let base = wrapping-u64-to-i64 slab in
        let wire = base in
        let hdrs = wrapping-add-i64 base 1024 in
        let text = wrapping-add-i64 base 2048 in
        let tmp  = wrapping-add-i64 base 4096 in
        let ok-static = verify-static-loop 1 wire tmp in
        let ok-c1 = test-c1 wire tmp in
        let ok-c2 = test-c2 huff wire hdrs text tmp in
        let ok-c3 = (test-c3-or-c4 huff hpack-str-raw
          "828684410f7777772e6578616d706c652e636f6d"
          "828684be58086e6f2d6361636865"
          "828785bf400a637573746f6d2d6b65790c637573746f6d2d76616c7565"
          wire hdrs text tmp) in
        let ok-c4 = (test-c3-or-c4 huff hpack-str-huffman
          "828684418cf1e3c2e5f23a6ba0ab90f4ff"
          "828684be5886a8eb10649cbf"
          "828785bf408825a849e95ba97d7f8925a849e95bb8e8b4bf"
          wire hdrs text tmp) in
        let ok-c5 = (test-c5-or-c6 huff hpack-str-raw
          "4803333032580770726976617465611d4d6f6e2c203231204f637420323031332032303a31333a323120474d546e1768747470733a2f2f7777772e6578616d706c652e636f6d"
          "4803333037c1c0bf"
          "88c1611d4d6f6e2c203231204f637420323031332032303a31333a323220474d54c05a04677a69707738666f6f3d4153444a4b48514b425a584f5157454f50495541585157454f49553b206d61782d6167653d333630303b2076657273696f6e3d31"
          wire hdrs text tmp) in
        let ok-c6 = (test-c5-or-c6 huff hpack-str-huffman
          "488264025885aec3771a4b6196d07abe941054d444a8200595040b8166e082a62d1bff6e919d29ad171863c78f0b97c8e9ae82ae43d3"
          "4883640effc1c0bf"
          "88c16196d07abe941054d444a8200595040b8166e084a62d1bffc05a839bd9ab77ad94e7821dd7f2e6c7b335dfdfcd5b3960d5af27087f3672c1ab270fb5291f9587316065c003ed4ee5b1063d5007"
          wire hdrs text tmp) in
        let _ = engine-dma-free slab 8192 in
        let _ = hpack-huffman-table-destroy huff in
        if not ok-static then 10
        else if not ok-c1 then 11
        else if not ok-c2 then 12
        else if not ok-c3 then 13
        else if not ok-c4 then 14
        else if not ok-c5 then 15
        else if not ok-c6 then 16
        else 0
"#;
    run_leak_checked("hpack-rfc7541-appendix-c", src, 0);
}

// =========================================================================
// 3. Huffman 256-Byte Round-Trip, Strict Error Rejection, Size Updates,
//    Self-Ref Eviction Safety & Automatic High-Entropy Never-Indexed Policy
// =========================================================================

#[test]
fn net_http_hpack_huffman_256_strict_errors_and_never_indexed_policy() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Core
  import Data.Machine
  import Data.String
  import Engine.Types
  import Net.Http.Types
  import Net.Http.Hpack.StaticTable
  import Net.Http.Hpack.Huffman
  import Net.Http.Hpack.Codec
  import Prim.Atomic

  write-str-loop :: Bytes -> I64 -> I64 -> I64 ->{MVar} U32
  write-str-loop b i n dst =
    if 0 <= i && i < bytes-len-i64 b && lt-i64 i n then
      let _ = raw-store-u8 (wrapping-add-i64 dst i) (bytes-get-u8-at b i) in
      write-str-loop b (wrapping-add-i64 i 1) n dst
    else wrapping-i64-to-u32 n

  write-str :: I64 -> String ->{MVar} U32
  write-str dst s =
    let b = string-to-bytes s in
    write-str-loop b 0 (bytes-len-i64 b) dst

  dma-eq-bytes :: I64 -> I64 -> U32 -> U32 ->{MVar} Bool
  dma-eq-bytes a b i n =
    if ge-u32 i n then True
    else if eq-u8 (raw-load-u8 (wrapping-add-i64 a (u32-to-i64 i))) (raw-load-u8 (wrapping-add-i64 b (u32-to-i64 i))) then
      dma-eq-bytes a b (wrapping-add-u32 i 1) n
    else False

  -- Test every single byte 0x00..0xFF individually + all 256 bytes concatenated + empty
  fill-all256 :: I64 -> U32 ->{MVar} Unit
  fill-all256 dst i =
    if ge-u32 i 256 then ()
    else
      let _ = raw-store-u8 (wrapping-add-i64 dst (u32-to-i64 i)) (wrapping-u32-to-u8 i) in
      fill-all256 dst (wrapping-add-u32 i 1)

  test-single-bytes :: HpackHuffmanTable -> I64 -> I64 -> I64 -> U32 ->{MVar} Bool
  test-single-bytes huff src enc-buf dec-buf sym =
    if ge-u32 sym 256 then True
    else
      let b = wrapping-u32-to-u8 sym in
      let _ = raw-store-u8 src b in
      let elen = hpack-huffman-encode-len huff src 1 in
      let elen-p = hpack-huffman-encode-len-pure src 1 in
      let er = hpack-huffman-encode-raw huff src 1 enc-buf 16 in
      let dr = hpack-huffman-decode-raw huff enc-buf er.hr-len dec-buf 16 in
      let ok = (
        eq-u32 elen elen-p
        && eq-u32 er.hr-status huff-ok && eq-u32 er.hr-len elen
        && eq-u32 dr.hr-status huff-ok && eq-u32 dr.hr-len 1
        && eq-u8 (raw-load-u8 dec-buf) b) in
      if ok then test-single-bytes huff src enc-buf dec-buf (wrapping-add-u32 sym 1)
      else False

  test-huffman-all-and-errors :: HpackHuffmanTable -> I64 ->{MVar} Int
  test-huffman-all-and-errors huff base =
    let src = base in
    let enc-buf = wrapping-add-i64 base 512 in
    let dec-buf = wrapping-add-i64 base 2048 in
    -- 1. Empty string round-trip
    let e0 = hpack-huffman-encode-raw huff src 0 enc-buf 64 in
    let d0 = hpack-huffman-decode-raw huff enc-buf 0 dec-buf 64 in
    if not (eq-u32 e0.hr-status huff-ok && eq-u32 e0.hr-len 0 && eq-u32 d0.hr-status huff-ok && eq-u32 d0.hr-len 0) then 20
    else if not (test-single-bytes huff src enc-buf dec-buf 0) then 21
    else
      -- 2. All 256 bytes concatenated round-trip
      let _ = fill-all256 src 0 in
      let elen-all = hpack-huffman-encode-len huff src 256 in
      let er-all = hpack-huffman-encode-raw huff src 256 enc-buf 1024 in
      let dr-all = hpack-huffman-decode-raw huff enc-buf er-all.hr-len dec-buf 512 in
      if ne-u32 er-all.hr-status huff-ok then 221
      else if ne-u32 er-all.hr-len elen-all then 222
      else if eq-u32 dr-all.hr-status huff-err-eos then 223
      else if eq-u32 dr-all.hr-status huff-err-padding then 224
      else if eq-u32 dr-all.hr-status huff-err-buf-overflow then 225
      else if ne-u32 dr-all.hr-status huff-ok then 226
      else if ne-u32 dr-all.hr-len 256 then 227
      else if not (dma-eq-bytes src dec-buf 0 256) then 228
      else
        -- 3. Reject EOS symbol (0xFF 0xFF 0xFF 0xFF)
        let _ = raw-store-u32 enc-buf 4294967295 in
        let r-eos = hpack-huffman-decode-raw huff enc-buf 4 dec-buf 64 in
        -- 4. Reject EOS after a valid symbol ('0' = 00000 + 30 ones -> 0x07 0xFF 0xFF 0xFF 0xFF)
        let _ = raw-store-u8 enc-buf 0x07 in
        let _ = raw-store-u32 (wrapping-add-i64 enc-buf 1) 4294967295 in
        let r-eos2 = hpack-huffman-decode-raw huff enc-buf 5 dec-buf 64 in
        -- 5. Reject non-1 padding ('0' = 00000 + 110 = 0x06, and 00000 + 000 = 0x00)
        let _ = raw-store-u8 enc-buf 0x06 in
        let r-pad1 = hpack-huffman-decode-raw huff enc-buf 1 dec-buf 64 in
        let _ = raw-store-u8 enc-buf 0x00 in
        let r-pad2 = hpack-huffman-decode-raw huff enc-buf 1 dec-buf 64 in
        -- 6. Reject padding > 7 bits (single 0xFF = 8 ones, and 0x07 0xFF = '0' + 11 ones)
        let _ = raw-store-u8 enc-buf 0xFF in
        let r-pad3 = hpack-huffman-decode-raw huff enc-buf 1 dec-buf 64 in
        let _ = raw-store-u8 enc-buf 0x07 in
        let _ = raw-store-u8 (wrapping-add-i64 enc-buf 1) 0xFF in
        let r-pad4 = hpack-huffman-decode-raw huff enc-buf 2 dec-buf 64 in
        -- 7. Reject output buffer overflow cleanly
        let _ = raw-store-u8 enc-buf 0x07 in -- decodes to "0" (1B)
        let r-ovf = hpack-huffman-decode-raw huff enc-buf 1 dec-buf 0 in
        if ne-u32 r-eos.hr-status huff-err-eos then 23
        else if ne-u32 r-eos2.hr-status huff-err-eos then 24
        else if ne-u32 r-pad1.hr-status huff-err-padding then 25
        else if ne-u32 r-pad2.hr-status huff-err-padding then 26
        else if ne-u32 r-pad3.hr-status huff-err-padding then 27
        else if ne-u32 r-pad4.hr-status huff-err-padding then 28
        else if ne-u32 r-ovf.hr-status huff-err-buf-overflow then 29
        else 0

  -- Test prefix integer overflow, DoS continuation chain, truncation, and invalid prefix
  fill-byte :: I64 -> U32 -> U8 -> U32 ->{MVar} Unit
  fill-byte dst i v n =
    if ge-u32 i n then ()
    else
      let _ = raw-store-u8 (wrapping-add-i64 dst (u32-to-i64 i)) v in
      fill-byte dst (wrapping-add-u32 i 1) v n

  test-int-errors :: I64 ->{MVar} Int
  test-int-errors buf =
    -- Invalid prefix bits 0 and 9
    let r0 = hpack-decode-int-raw buf 0 4 0 in
    let r9 = hpack-decode-int-raw buf 0 4 9 in
    let w0 = hpack-encode-int-raw buf 0 4 0 0 1 in
    -- Truncated varint: 0x1F alone, and 0x1F 0x80
    let _ = raw-store-u8 buf 0x1F in
    let rt1 = hpack-decode-int-raw buf 0 1 5 in
    let _ = raw-store-u8 (wrapping-add-i64 buf 1) 0x80 in
    let rt2 = hpack-decode-int-raw buf 0 2 5 in
    -- DoS zero-padded overlong continuation: 0x1F followed by 12 x 0x80 + 0x00
    let _ = fill-byte (wrapping-add-i64 buf 1) 0 0x80 12 in
    let _ = raw-store-u8 (wrapping-add-i64 buf 13) 0x00 in
    let rovf1 = hpack-decode-int-raw buf 0 14 5 in
    -- U64 overflow: 0xFF followed by 10 x 0xFF + 0x01
    let _ = fill-byte buf 0 0xFF 11 in
    let _ = raw-store-u8 (wrapping-add-i64 buf 11) 0x01 in
    let rovf2 = hpack-decode-int-raw buf 0 12 8 in
    -- U32 cap overflow (valid U64 = 5000000000 > 2^31 - 1)
    let wbig = hpack-encode-int-raw buf 0 32 5 0 5000000000 in
    let dbig64 = hpack-decode-int-raw buf 0 wbig.hw-next-pos 5 in
    let dbig32 = hpack-decode-int-u32-raw buf 0 wbig.hw-next-pos 5 in
    if ne-u32 r0.hi-status hpack-err-invalid-prefix then 30
    else if ne-u32 r9.hi-status hpack-err-invalid-prefix then 31
    else if ne-u32 w0.hw-status hpack-err-invalid-prefix then 32
    else if ne-u32 rt1.hi-status hpack-err-truncated then 33
    else if ne-u32 rt2.hi-status hpack-err-truncated then 34
    else if ne-u32 rovf1.hi-status hpack-err-int-overflow then 35
    else if ne-u32 rovf2.hi-status hpack-err-int-overflow then 36
    else if not (eq-u32 wbig.hw-status hpack-ok && eq-u32 dbig64.hi-status hpack-ok && eq-u64 dbig64.hi-value 5000000000) then 37
    else if ne-u32 dbig32.hi-status hpack-err-int-overflow then 38
    else 0

  -- Test protocol error cases, dynamic table size updates, §4.4 eviction safety, and Never-Indexed policy
  test-codec-errors-and-policy :: HpackHuffmanTable -> I64 ->{IO, MVar} Int
  test-codec-errors-and-policy huff base =
    let wire = base in
    let hdrs = wrapping-add-i64 base 1024 in
    let text = wrapping-add-i64 base 2048 in
    let scratch = wrapping-add-i64 base 4096 in
    let dec = hpack-decoder-new-with-huff 256 huff in
    if not (hpack-decoder-valid dec) then 40
    else
      -- 1. Index 0 (`0x80`) MUST be rejected
      let _ = raw-store-u8 wire 0x80 in
      let r-idx0 = hpack-decode-block dec wire 1 hdrs 16 text 1024 in
      -- 2. Dynamic index out of range (`0xBE` = index 62 when empty) MUST be rejected
      let _ = raw-store-u8 wire 0xBE in
      let r-idx62 = hpack-decode-block dec wire 1 hdrs 16 text 1024 in
      -- 3. Dynamic table size update > SETTINGS_HEADER_TABLE_SIZE (257 > 256) MUST be rejected
      let wu = hpack-encode-table-size-update wire 0 64 257 in
      let r-sz-big = hpack-decode-block dec wire wu.hw-next-pos hdrs 16 text 1024 in
      -- 4. Dynamic table size update AFTER a header representation (`0x82` then `0x20` = size 0) MUST be rejected
      let _ = raw-store-u8 wire 0x82 in
      let wu2 = hpack-encode-table-size-update wire 1 64 0 in
      let r-sz-after = hpack-decode-block dec wire wu2.hw-next-pos hdrs 16 text 1024 in
      -- 5. Valid multi-update at start of block (`256 -> 64 -> 128`) followed by `:method: GET`
      let ua = hpack-encode-table-size-update wire 0 64 64 in
      let ub = hpack-encode-table-size-update wire ua.hw-next-pos 64 128 in
      let _ = raw-store-u8 (wrapping-add-i64 wire (u32-to-i64 ub.hw-next-pos)) 0x82 in
      let r-sz-ok = hpack-decode-block dec wire (wrapping-add-u32 ub.hw-next-pos 1) hdrs 16 text 1024 in
      let dd = hpack-decoder-dyn-table dec in
      let sz-after-ok = hpack-dyn-max-size dd in
      -- 6. RFC 7541 §4.4: Insert entry larger than cur-max-size (128B) clears table without error!
      let nlen-a = write-str scratch "x-key" in
      let vlen-a = write-str (wrapping-add-i64 scratch 64) "val" in
      let _ = hpack-dyn-insert dd scratch nlen-a (wrapping-add-i64 scratch 64) vlen-a in
      let cnt-before = hpack-dyn-count dd in
      let _ = fill-byte (wrapping-add-i64 scratch 256) 0 97 120 in -- 120 + 5 + 32 = 157 > 128
      let ins-big = hpack-dyn-insert dd scratch nlen-a (wrapping-add-i64 scratch 256) 120 in
      let cnt-after = hpack-dyn-count dd in
      let _ = hpack-decoder-destroy dec in
      if ne-u32 r-idx0.hd-status hpack-err-invalid-index then 41
      else if ne-u32 r-idx62.hd-status hpack-err-invalid-index then 42
      else if ne-u32 r-sz-big.hd-status hpack-err-table-size-exceeds-limit then 43
      else if ne-u32 r-sz-after.hd-status hpack-err-table-size-after-header then 44
      else if not (eq-u32 r-sz-ok.hd-status hpack-ok && eq-u32 r-sz-ok.hd-header-count 1 && eq-u32 sz-after-ok 128) then 45
      else if not (eq-u32 cnt-before 1 && eq-u32 ins-big hpack-ok && eq-u32 cnt-after 0) then 46
      else
        -- 7. Automatic high-entropy Never-Indexed policy test (`hpack-mode-auto`)
        --    + encoder multi-resize min+max emission test!
        let enc = hpack-encoder-new-with-huff 4096 hpack-str-auto huff in
        if not (hpack-encoder-valid enc) then 47
        else
          let dec2 = hpack-decoder-new-with-huff 4096 huff in
          if not (hpack-decoder-valid dec2) then
            let _ = hpack-encoder-destroy enc in
            48
          else
            -- Trigger two resizes before emitting block: 4096 -> 128 -> 512
            let rs1 = hpack-encoder-set-max-table-size enc 128 in
            let rs2 = hpack-encoder-set-max-table-size enc 512 in
            -- Encode all 4 sensitive headers in `hpack-mode-auto`
            let n1 = write-str scratch "authorization" in
            let v1 = write-str (wrapping-add-i64 scratch 64) "Bearer secret-jwt-token" in
            let w1 = hpack-encode-header enc scratch n1 (wrapping-add-i64 scratch 64) v1 wire 0 512 in
            let n2 = write-str scratch "cookie" in
            let v2 = write-str (wrapping-add-i64 scratch 64) "sid=deadbeef" in
            let w2 = hpack-encode-header enc scratch n2 (wrapping-add-i64 scratch 64) v2 wire w1.hw-next-pos 512 in
            let n3 = write-str scratch "set-cookie" in
            let v3 = write-str (wrapping-add-i64 scratch 64) "sid=cafebabe; HttpOnly" in
            let w3 = hpack-encode-header enc scratch n3 (wrapping-add-i64 scratch 64) v3 wire w2.hw-next-pos 512 in
            let n4 = write-str scratch "proxy-authorization" in
            let v4 = write-str (wrapping-add-i64 scratch 64) "Basic dXNlcjpwYXNz" in
            let w4 = hpack-encode-header enc scratch n4 (wrapping-add-i64 scratch 64) v4 wire w3.hw-next-pos 512 in
            -- Decode block and verify:
            --   * Decoder dynamic table max-size updated to 512
            --   * ZERO sensitive headers inserted into encoder OR decoder dynamic table!
            --   * ALL 4 decoded headers carry `hpack-flag-never-indexed`!
            let dr = hpack-decode-block dec2 wire w4.hw-next-pos hdrs 16 text 1024 in
            let h0 = hpack-header-read-at hdrs 0 in
            let h1 = hpack-header-read-at hdrs 1 in
            let h2 = hpack-header-read-at hdrs 2 in
            let h3 = hpack-header-read-at hdrs 3 in
            let enc-cnt = hpack-dyn-count (hpack-encoder-dyn-table enc) in
            let dec-cnt = hpack-dyn-count (hpack-decoder-dyn-table dec2) in
            let dec-max = hpack-dyn-max-size (hpack-decoder-dyn-table dec2) in
            let _ = hpack-decoder-destroy dec2 in
            let _ = hpack-encoder-destroy enc in
            if not (eq-u32 rs1 hpack-ok && eq-u32 rs2 hpack-ok) then 49
            else if not (eq-u32 w4.hw-status hpack-ok && eq-u32 dr.hd-status hpack-ok && eq-u32 dr.hd-header-count 4) then 50
            else if not (eq-u32 enc-cnt 0 && eq-u32 dec-cnt 0 && eq-u32 dec-max 512) then 51
            else if not (
              eq-u8 h0.hh-known hdr-authorization && eq-u8 h0.hh-flags hpack-flag-never-indexed
              && eq-u8 h1.hh-known hdr-cookie && eq-u8 h1.hh-flags hpack-flag-never-indexed
              && eq-u8 h2.hh-known hdr-set-cookie && eq-u8 h2.hh-flags hpack-flag-never-indexed
              && eq-u8 h3.hh-known hdr-proxy-authorization && eq-u8 h3.hh-flags hpack-flag-never-indexed) then 52
            else 0

  main :: Unit ->{IO, MVar} Int
  main _ =
    let huff = hpack-huffman-table-new () in
    if not (hpack-huffman-table-valid huff) then 1
    else
      let slab = engine-dma-alloc 8192 4096 in
      if eq-u64 slab 0 then
        let _ = hpack-huffman-table-destroy huff in
        2
      else
        let base = wrapping-u64-to-i64 slab in
        let r1 = test-huffman-all-and-errors huff base in
        let r2 = if r1 /= 0 then r1 else test-int-errors base in
        let r3 = if r2 /= 0 then r2 else test-codec-errors-and-policy huff base in
        let _ = engine-dma-free slab 8192 in
        let _ = hpack-huffman-table-destroy huff in
        r3
"#;
    run_leak_checked("hpack-huffman-256-errors-policy", src, 0);
}

// =========================================================================
// 4. 50,000-Iteration Steady-State Zero-Alloc / Zero-Atomic-RC / Leak-Free Proof
// =========================================================================

#[test]
fn net_http_hpack_50k_steady_state_zero_alloc_zero_atomic_rc() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Core
  import Data.Machine
  import Data.String
  import Engine.Types
  import Net.Http.Types
  import Net.Http.Hpack.StaticTable
  import Net.Http.Hpack.Huffman
  import Net.Http.Hpack.Codec
  import Prim.Atomic

  write-str-loop :: Bytes -> I64 -> I64 -> I64 ->{MVar} U32
  write-str-loop b i n dst =
    if 0 <= i && i < bytes-len-i64 b && lt-i64 i n then
      let _ = raw-store-u8 (wrapping-add-i64 dst i) (bytes-get-u8-at b i) in
      write-str-loop b (wrapping-add-i64 i 1) n dst
    else wrapping-i64-to-u32 n

  write-str :: I64 -> String ->{MVar} U32
  write-str dst s =
    let b = string-to-bytes s in
    write-str-loop b 0 (bytes-len-i64 b) dst

  -- 100% #[no-alloc] steady-state 50,000 encode+decode loop exercising:
  --   1. Static indexed (`:method: GET`, `:scheme: https`, `:path: /index.html`)
  --   2. Dynamic table reuse (`:authority: api.cinch.dev`, `x-request-id: req-00001234`)
  --   3. Huffman-compressed literal incremental indexing with ring wrap & eviction
  --   4. Automatic Never-Indexed high-entropy `authorization` header!
  #[no-alloc]
  steady-loop
    :: HpackEncoder -> HpackDecoder
    -> I64 -> I64 -> I64 -> I64
    -> U32 -> U64
    ->{MVar} U64
  steady-loop enc dec fixture wire hdrs text rem acc =
    if eq-u32 rem 0 then acc
    else
      -- Fixture layout in DMA (all pre-populated before counter reset):
      --   +0   ":method" (7)          +16  "GET" (3)
      --   +32  ":scheme" (7)          +48  "https" (5)
      --   +64  ":path" (5)            +80  "/index.html" (11)
      --   +96  ":authority" (10)      +112 "api.cinch.dev" (13)
      --   +128 "x-request-id" (12)    +144 "req-00001234" (12)
      --   +160 "authorization" (13)   +176 "Bearer tok-abcdef0123456789" (27)
      -- Mutate last byte of x-request-id every iteration (mod 8) to exercise
      -- Huffman literal encodes + ring-buffer inserts + continuous evictions!
      let tag-b = wrapping-add-u8 48 (wrapping-u32-to-u8 (bit-and-u32 rem 7)) in
      let _ = raw-store-u8 (wrapping-add-i64 fixture 155) tag-b in
      let w0 = hpack-encode-header enc fixture 7 (wrapping-add-i64 fixture 16) 3 wire 0 512 in
      let w1 = hpack-encode-header enc (wrapping-add-i64 fixture 32) 7 (wrapping-add-i64 fixture 48) 5 wire w0.hw-next-pos 512 in
      let w2 = hpack-encode-header enc (wrapping-add-i64 fixture 64) 5 (wrapping-add-i64 fixture 80) 11 wire w1.hw-next-pos 512 in
      let w3 = hpack-encode-header enc (wrapping-add-i64 fixture 96) 10 (wrapping-add-i64 fixture 112) 13 wire w2.hw-next-pos 512 in
      let w4 = hpack-encode-header enc (wrapping-add-i64 fixture 128) 12 (wrapping-add-i64 fixture 144) 12 wire w3.hw-next-pos 512 in
      let w5 = hpack-encode-header enc (wrapping-add-i64 fixture 160) 13 (wrapping-add-i64 fixture 176) 27 wire w4.hw-next-pos 512 in
      let dr = hpack-decode-block dec wire w5.hw-next-pos hdrs 16 text 1024 in
      let h5 = hpack-header-read-at hdrs 5 in
      let ok-bit = if eq-u32 dr.hd-status hpack-ok then 1 else 0 in
      let step = (wrapping-add-u64
        (wrapping-add-u64 (u32-to-u64 ok-bit) (u32-to-u64 dr.hd-header-count))
        (wrapping-add-u64
          (wrapping-add-u64 (u8-to-u64 h5.hh-known) (u8-to-u64 h5.hh-flags))
          (u32-to-u64 dr.hd-bytes-written))) in
      steady-loop enc dec fixture wire hdrs text (wrapping-sub-u32 rem 1) (wrapping-add-u64 acc step)

  main :: Unit ->{IO, MVar} Int
  main _ =
    let huff = hpack-huffman-table-new () in
    if not (hpack-huffman-table-valid huff) then 1
    else
      let enc = hpack-encoder-new-with-huff 256 hpack-str-huffman huff in
      if not (hpack-encoder-valid enc) then
        let _ = hpack-huffman-table-destroy huff in
        2
      else
        let dec = hpack-decoder-new-with-huff 256 huff in
        if not (hpack-decoder-valid dec) then
          let _ = hpack-encoder-destroy enc in
          let _ = hpack-huffman-table-destroy huff in
          3
        else
          let slab = engine-dma-alloc 8192 4096 in
          if eq-u64 slab 0 then
            let _ = hpack-decoder-destroy dec in
            let _ = hpack-encoder-destroy enc in
            let _ = hpack-huffman-table-destroy huff in
            4
          else
            let base = wrapping-u64-to-i64 slab in
            let fixture = base in
            let wire = wrapping-add-i64 base 1024 in
            let hdrs = wrapping-add-i64 base 2048 in
            let text = wrapping-add-i64 base 3072 in
            let _ = write-str fixture ":method" in
            let _ = write-str (wrapping-add-i64 fixture 16) "GET" in
            let _ = write-str (wrapping-add-i64 fixture 32) ":scheme" in
            let _ = write-str (wrapping-add-i64 fixture 48) "https" in
            let _ = write-str (wrapping-add-i64 fixture 64) ":path" in
            let _ = write-str (wrapping-add-i64 fixture 80) "/index.html" in
            let _ = write-str (wrapping-add-i64 fixture 96) ":authority" in
            let _ = write-str (wrapping-add-i64 fixture 112) "api.cinch.dev" in
            let _ = write-str (wrapping-add-i64 fixture 128) "x-request-id" in
            let _ = write-str (wrapping-add-i64 fixture 144) "req-00001234" in
            let _ = write-str (wrapping-add-i64 fixture 160) "authorization" in
            let _ = write-str (wrapping-add-i64 fixture 176) "Bearer tok-abcdef0123456789" in
            -- Reset thread alloc & atomic RC counters immediately before 50,000 encode+decode loop!
            let _ = engine-thread-counters-reset () in
            let acc = steady-loop enc dec fixture wire hdrs text 50000 0 in
            let allocs = engine-thread-alloc-count () in
            let atomic-rcs = engine-thread-atomic-rc-count () in
            let _ = engine-dma-free slab 8192 in
            let _ = hpack-decoder-destroy dec in
            let _ = hpack-encoder-destroy enc in
            let _ = hpack-huffman-table-destroy huff in
            -- Total bytes written per request:
            --   (7+3) + (7+5) + (5+11) + (10+13) + (12+12) + (13+27) = 10+12+16+23+24+40 = 125
            -- Step per iteration: 1 + 6 + 13 + 1 + 125 = 146
            -- 50,000 * 146 = 7,300,000
            if ne-u64 acc 7300000 then 10
            else if ne-u64 allocs 0 then 11
            else if ne-u64 atomic-rcs 0 then 12
            else 0
"#;
    let (_stdout, elapsed) = run_leak_checked("hpack-50k-steady-state", src, 0);
    let secs = elapsed.as_secs_f64();
    let ops_per_sec = 50_000.0 / secs.max(1e-9);
    let ns_per_op = (secs * 1e9) / 50_000.0;
    eprintln!(
        "Net.Http.Hpack 50,000 encode+decode blocks (incl. process+setup): {:.2} ms ({:.2}k blocks/s, {:.1} ns/block), allocs=0, atomic_rcs=0, leaked=0",
        secs * 1000.0,
        ops_per_sec / 1e3,
        ns_per_op
    );
}
