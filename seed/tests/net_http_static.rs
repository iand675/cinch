//! Integration, path-traversal security corpus, MIME / pre-compressed variant
//! negotiation, ETag / `If-None-Match` (`304`), RFC 9110 `Range` (`206`/`416`),
//! `HEAD`, LRU eviction + TTL refresh, live macOS loopback 256 KiB `sendfile`,
//! and 25,000-iteration cache-hit zero-allocation tests for `Net.Http.Static`.
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
        "cinch-net-http-static-{tag}-{}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn escape_cinch_str(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn build_and_run_leak_checked(tag: &str, src: &str, expected_code: i32) -> (String, String) {
    let dir = fresh_dir(tag);
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let exe = dir.join(format!("http-static-{tag}"));
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
// 1. Manifest registration & compile-time #[no-alloc] hot-path audit
// =========================================================================

#[test]
fn net_http_static_manifest_registration_and_no_alloc_check() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    let entry = pkg
        .modules
        .iter()
        .find(|m| m.name == "Net.Http.Static")
        .expect("Net.Http.Static missing from seed/prelude/.cinchpkg modules");
    assert_eq!(entry.path, "Net/Http/Static.cinch");
    assert!(
        pkg.order.iter().any(|m| m == "Net.Http.Static"),
        "Net.Http.Static missing from seed/prelude/.cinchpkg order"
    );

    let pos_types = pkg.order.iter().position(|n| n == "Net.Http.Types").unwrap();
    let pos_conn = pkg.order.iter().position(|n| n == "Net.Http.V1.Conn").unwrap();
    let pos_timer = pkg.order.iter().position(|n| n == "Engine.Timer").unwrap();
    let pos_darwin = pkg.order.iter().position(|n| n == "Engine.Reactor.Darwin").unwrap();
    let pos_static = pkg.order.iter().position(|n| n == "Net.Http.Static").unwrap();
    assert!(pos_types < pos_static);
    assert!(pos_conn < pos_static);
    assert!(pos_timer < pos_static);
    assert!(pos_darwin < pos_static);

    let dir = fresh_dir("no-alloc-check");
    std::fs::write(
        dir.join("Main.cinch"),
        r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Engine.Timer
  import Net.Http.Types
  import Net.Http.Static

  #[no-alloc]
  audit-static-hot-path :: StaticFileCache -> CoarseClockCache -> I64 -> I64 -> I64 -> I64 ->{MVar} U32
  audit-static-hot-path cache clk req-addr out-addr hdr-addr cc-addr =
    let r0 = static-resolve-path-dma req-addr 11 0 0 out-addr 256 in
    let mid = static-mime-id-for-path-dma out-addr r0.sp-len in
    let am = static-parse-accept-encoding-dma req-addr 4 in
    let _enc = static-negotiate-encoding am accept-enc-any in
    let elen = static-format-etag-dma out-addr 48 1024 1700000000 (eq-u32 0 1) in
    let _inm = static-if-none-match-dma out-addr elen out-addr elen in
    let _rng = static-parse-range-dma req-addr 0 1024 in
    let view = static-cache-get-hit-dma cache req-addr 11 am 1000 in
    let plan = static-plan-response-dma method-code-get view 0 0 0 0 in
    let n = static-format-headers-dma hdr-addr 0 1024 plan clk (eq-u32 0 0) cc-addr 0 in
    wrapping-add-u32 n (u8-to-u32 mid)

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
    .expect("Net.Http.Static #[no-alloc] hot-path audit must check cleanly");
    let _ = std::fs::remove_dir_all(&dir);
}

// =========================================================================
// 2. Path-traversal security corpus (28 attack vectors + clean normalization)
// =========================================================================

#[test]
fn net_http_static_path_traversal_security_corpus() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Result
  import Data.String
  import Engine.Types
  import Net.Http.Static
  import Prim.Atomic

  is-rejected :: String ->{MVar} Bool
  is-rejected req =
    case static-resolve-path "/srv/www" req "index.html" of
      Err _ -> True
      Ok _ -> False

  resolves-to :: String -> String -> String ->{MVar} Bool
  resolves-to req idx expected =
    case static-resolve-path "/srv/www" req idx of
      Ok got -> got == expected
      Err _ -> False

  fill-a :: I64 -> U32 -> U32 ->{MVar} Unit
  fill-a base i n =
    if ge-u32 i n then ()
    else
      let _ = raw-store-u8 (wrapping-add-i64 base (u32-to-i64 i)) 97 in
      fill-a base (wrapping-add-u32 i 1) n

  main :: Unit ->{IO, MVar} Int
  main _ =
    -- 26 String-level attack vectors covering every traversal & smuggling class
    if not (is-rejected "/../etc/passwd") then 1
    else if not (is-rejected "/..") then 2
    else if not (is-rejected "/a/../../secret") then 3
    else if not (is-rejected "/a/b/..") then 4
    else if not (is-rejected "/%2e%2e/secret") then 5
    else if not (is-rejected "/%2E%2E/secret") then 6
    else if not (is-rejected "/.%2e/secret") then 7
    else if not (is-rejected "/%2e./secret") then 8
    else if not (is-rejected "/..%2fsecret") then 9
    else if not (is-rejected "/%2e%2e%2fsecret") then 10
    else if not (is-rejected "/%2e%2e%2Fsecret") then 11
    else if not (is-rejected "/%2e%2e%5csecret") then 12
    else if not (is-rejected "/%2e%2e%5Csecret") then 13
    else if not (is-rejected "/foo\\bar") then 14
    else if not (is-rejected "/a/b\\..\\c") then 15
    else if not (is-rejected "/foo%00.html") then 16
    else if not (is-rejected "/%252e%252e/etc/passwd") then 17
    else if not (is-rejected "/%252fetc/passwd") then 18
    else if not (is-rejected "/%255csecret") then 19
    else if not (is-rejected "/foo%2") then 20
    else if not (is-rejected "/foo%ZZ") then 21
    else if not (is-rejected "/foo%0abar") then 22
    else if not (is-rejected "/%c0%afetc/passwd") then 23
    else if not (is-rejected "") then 24
    else if not (is-rejected "etc/passwd") then 25
    else if not (is-rejected "../etc/passwd") then 26
    -- Bad index filename must be rejected
    else if (case static-resolve-path "/srv/www" "/" "../evil" of Err _ -> False; Ok _ -> True) then 27
    else
      -- DMA-level attack vectors: raw NUL byte & overlong > 1024B path
      let dma = engine-dma-alloc 4096 4096 in
      if eq-u64 dma 0 then 28
      else
        let buf = wrapping-u64-to-i64 dma in
        let out = wrapping-add-i64 buf 2048 in
        -- Vector 27: raw NUL byte `"/a\0b"`
        let _0 = raw-store-u8 buf 47 in
        let _1 = raw-store-u8 (wrapping-add-i64 buf 1) 97 in
        let _2 = raw-store-u8 (wrapping-add-i64 buf 2) 0 in
        let _3 = raw-store-u8 (wrapping-add-i64 buf 3) 98 in
        let r-nul = static-resolve-path-dma buf 4 0 0 out 1024 in
        -- Vector 28: overlong 1025-byte path `"/aaa...a"`
        let _4 = raw-store-u8 buf 47 in
        let _5 = fill-a buf 1 1025 in
        let r-long = static-resolve-path-dma buf 1025 0 0 out 1536 in
        -- Vector 29: exact 1024-byte path succeeds cleanly
        let r-max-ok = static-resolve-path-dma buf 1024 0 0 out 1536 in
        let _f = engine-dma-free dma 4096 in
        if ne-u8 r-nul.sp-status static-path-err-nul-byte then 29
        else if ne-u8 r-long.sp-status static-path-err-too-long then 30
        else if ne-u8 r-max-ok.sp-status static-path-ok || ne-u32 r-max-ok.sp-len 1023 then 31
        -- Legitimate path confinement & normalization checks
        else if not (resolves-to "/" "index.html" "/srv/www/index.html") then 32
        else if not (resolves-to "///" "index.html" "/srv/www/index.html") then 33
        else if not (resolves-to "/docs/" "index.html" "/srv/www/docs/index.html") then 34
        else if not (resolves-to "/css///app.css" "index.html" "/srv/www/css/app.css") then 35
        else if not (resolves-to "/a/./b.js" "index.html" "/srv/www/a/b.js") then 36
        else if not (resolves-to "/." "index.html" "/srv/www/index.html") then 37
        else if not (resolves-to "/hello%20world.txt" "index.html" "/srv/www/hello world.txt") then 38
        else if not (resolves-to "/app.js?v=42" "index.html" "/srv/www/app.js") then 39
        else if not (resolves-to "/blog/" "home.htm" "/srv/www/blog/home.htm") then 40
        else 0
"#;
    build_and_run_leak_checked("path-traversal-corpus", src, 0);
}

// =========================================================================
// 3. MIME, Pre-Compressed Variants, ETag/304, Range 206/416, HEAD, LRU & TTL
// =========================================================================

#[cfg(target_os = "macos")]
#[test]
fn net_http_static_mime_variants_etag_range_head_lru_ttl() {
    let root = fresh_dir("www-root");
    std::fs::write(root.join("index.html"), b"<h1>hello</h1>").unwrap();
    std::fs::write(root.join("app.js"), b"console.log('identity-payload-1234567890');").unwrap();
    std::fs::write(root.join("app.js.gz"), b"GZIP_COMPRESSED_DATA_12345").unwrap();
    std::fs::write(root.join("app.js.br"), b"BROTLI_COMPRESSED_1234").unwrap();
    std::fs::write(root.join("app.js.zst"), b"ZSTD_COMPRESSED_12").unwrap();
    std::fs::write(root.join("style.css"), b"body { color: black; margin: 0; }").unwrap();
    std::fs::write(root.join("style.css.gz"), b"CSS_GZIP_BYTES").unwrap();
    std::fs::write(root.join("a.txt"), b"alpha").unwrap();
    std::fs::write(root.join("b.txt"), b"bravo-6").unwrap();
    std::fs::write(root.join("c.txt"), b"charlie-9").unwrap();
    std::fs::create_dir_all(root.join("sub")).unwrap();
    std::fs::write(root.join("sub").join("index.html"), b"<p>sub</p>").unwrap();

    let root_esc = escape_cinch_str(&root.to_string_lossy());
    let a_path_esc = escape_cinch_str(&root.join("a.txt").to_string_lossy());

    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Engine.Reactor.Darwin
  import Engine.Timer
  import Engine.Types
  import Net.Http.Types
  import Net.Http.Static
  import Prim.Atomic
  import Prim.Sys.Io.Darwin

  copy-str-to-dma :: String -> I64 ->{MVar} U32
  copy-str-to-dma s dst =
    let b = string-to-bytes s in
    let n = bytes-len b in
    let _ = copy-bytes-loop b 0 dst n in
    wrapping-i64-to-u32 (nat-to-i64 n)

  copy-bytes-loop :: Bytes -> Nat -> I64 -> Nat ->{MVar} Unit
  copy-bytes-loop b i dst n =
    if i >= n then ()
    else if i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 dst (nat-to-i64 i)) (bytes-get-u8 b i) in
      copy-bytes-loop b (i + 1) dst n
    else ()

  -- Check if DMA haystack `[haddr, haddr+hlen)` contains ASCII needle string.
  dma-contains-str :: I64 -> U32 -> String ->{MVar} Bool
  dma-contains-str haddr hlen needle =
    let nb = string-to-bytes needle in
    let nlen = wrapping-i64-to-u32 (bytes-len-i64 nb) in
    if eq-u32 nlen 0 then True
    else if gt-u32 nlen hlen then False
    else sys-held nb (dma-contains-loop haddr hlen (bytes-data nb) nlen 0)

  dma-contains-loop :: I64 -> U32 -> I64 -> U32 -> U32 ->{MVar} Bool
  dma-contains-loop haddr hlen naddr nlen i =
    if gt-u32 (wrapping-add-u32 i nlen) hlen then False
    else if dma-bytes-eq (wrapping-add-i64 haddr (u32-to-i64 i)) naddr nlen 0 then True
    else dma-contains-loop haddr hlen naddr nlen (wrapping-add-u32 i 1)

  dma-bytes-eq :: I64 -> I64 -> U32 -> U32 ->{MVar} Bool
  dma-bytes-eq a b len j =
    if ge-u32 j len then True
    else if ne-u8 (raw-load-u8 (wrapping-add-i64 a (u32-to-i64 j))) (raw-load-u8 (wrapping-add-i64 b (u32-to-i64 j))) then False
    else dma-bytes-eq a b len (wrapping-add-u32 j 1)

  check-mimes :: Unit -> Bool
  check-mimes _ =
    static-mime-for-path "index.html" == "text/html; charset=utf-8"
      && static-mime-for-path "PAGE.HTM" == "text/html; charset=utf-8"
      && static-mime-for-path "/a/style.css" == "text/css; charset=utf-8"
      && static-mime-for-path "app.js" == "text/javascript; charset=utf-8"
      && static-mime-for-path "mod.mjs" == "text/javascript; charset=utf-8"
      && static-mime-for-path "data.json" == "application/json; charset=utf-8"
      && static-mime-for-path "logo.svg" == "image/svg+xml"
      && static-mime-for-path "pic.png" == "image/png"
      && static-mime-for-path "photo.jpg" == "image/jpeg"
      && static-mime-for-path "photo.JPEG" == "image/jpeg"
      && static-mime-for-path "img.webp" == "image/webp"
      && static-mime-for-path "img.avif" == "image/avif"
      && static-mime-for-path "font.WOFF2" == "font/woff2"
      && static-mime-for-path "core.wasm" == "application/wasm"
      && static-mime-for-path "readme.txt" == "text/plain; charset=utf-8"
      && static-mime-for-path "feed.xml" == "application/xml; charset=utf-8"
      && static-mime-for-path "doc.pdf" == "application/pdf"
      && static-mime-for-path "favicon.ico" == "image/x-icon"
      && static-mime-for-path "app.js.map" == "application/json; charset=utf-8"
      && static-mime-for-path "archive.xyz" == "application/octet-stream"
      && static-mime-for-path "Makefile" == "application/octet-stream"

  main :: Unit ->{IO, MVar} Int
  main _ =
    if not (check-mimes ()) then 1
    else
      case coarse-clock-new () of
        None -> 2
        Some clk ->
          let _rf = coarse-date-refresh clk 1764338400 in
          let dma = engine-dma-alloc 16384 4096 in
          if eq-u64 dma 0 then
            let _dc = coarse-clock-destroy clk in 3
          else
            let slab = wrapping-u64-to-i64 dma in
            let req-buf = slab in
            let ae-buf = wrapping-add-i64 slab 1024 in
            let inm-buf = wrapping-add-i64 slab 2048 in
            let rng-buf = wrapping-add-i64 slab 3072 in
            let cc-buf = wrapping-add-i64 slab 4096 in
            let hdr-buf = wrapping-add-i64 slab 8192 in
            let cc-len = copy-str-to-dma "public, max-age=3600" cc-buf in

            -- === Part A: Pre-compressed variant negotiation + ETag/304 + Range 206/416 + HEAD ===
            let root-b = string-to-bytes "__ROOT_DIR__" in
            let idx-b = string-to-bytes "index.html" in
            case static-cache-new root-b idx-b 16 5000000000 of
              None ->
                let _f = engine-dma-free dma 16384 in
                let _dc = coarse-clock-destroy clk in 4
              Some cache ->
                -- 1. Accept-Encoding: gzip, br, zstd on /app.js -> selects .zst (18 bytes)
                let p-js = copy-str-to-dma "/app.js" req-buf in
                let ae1-len = copy-str-to-dma "gzip, deflate, br, zstd" ae-buf in
                let am1 = static-parse-accept-encoding-dma ae-buf ae1-len in
                let v-zst = static-cache-lookup-dma cache req-buf p-js am1 1000000000 in
                let plan-zst = static-plan-response-dma method-code-get v-zst 0 0 0 0 in
                let h-zst = static-format-headers-dma hdr-buf 0 4096 plan-zst clk True cc-buf cc-len in
                let ok-zst = (
                  eq-u8 v-zst.sfv-status static-lookup-ok
                    && eq-u8 v-zst.sfv-encoding static-enc-zstd
                    && eq-u8 v-zst.sfv-mime mime-js
                    && eq-u64 v-zst.sfv-size 18
                    && dma-contains-str hdr-buf h-zst "HTTP/1.1 200 OK\r\n"
                    && dma-contains-str hdr-buf h-zst "Content-Type: text/javascript; charset=utf-8\r\n"
                    && dma-contains-str hdr-buf h-zst "Content-Encoding: zstd\r\n"
                    && dma-contains-str hdr-buf h-zst "Vary: Accept-Encoding\r\n"
                    && dma-contains-str hdr-buf h-zst "Content-Length: 18\r\n") in

                -- 2. Accept-Encoding: gzip, br, zstd;q=0 -> excludes zstd, selects .br (22 bytes)
                let ae2-len = copy-str-to-dma "gzip, br, zstd;q=0" ae-buf in
                let am2 = static-parse-accept-encoding-dma ae-buf ae2-len in
                let v-br = static-cache-lookup-dma cache req-buf p-js am2 1000000000 in
                let plan-br = static-plan-response-dma method-code-get v-br 0 0 0 0 in
                let h-br = static-format-headers-dma hdr-buf 0 4096 plan-br clk True cc-buf cc-len in
                let ok-br = (
                  eq-u8 v-br.sfv-encoding static-enc-br
                    && eq-u64 v-br.sfv-size 22
                    && dma-contains-str hdr-buf h-br "Content-Encoding: br\r\n") in

                -- 3. Accept-Encoding: gzip -> selects .gz (26 bytes)
                let ae3-len = copy-str-to-dma "gzip" ae-buf in
                let am3 = static-parse-accept-encoding-dma ae-buf ae3-len in
                let v-gz = static-cache-lookup-dma cache req-buf p-js am3 1000000000 in
                let ok-gz = eq-u8 v-gz.sfv-encoding static-enc-gzip && eq-u64 v-gz.sfv-size 26 in

                -- 4. Identity (am = 0) -> uncompressed app.js (43 bytes)
                let v-id = static-cache-lookup-dma cache req-buf p-js 0 1000000000 in
                let plan-id = static-plan-response-dma method-code-get v-id 0 0 0 0 in
                let h-id = static-format-headers-dma hdr-buf 0 4096 plan-id clk True cc-buf cc-len in
                let ok-id = (
                  eq-u8 v-id.sfv-encoding static-enc-identity
                    && eq-u64 v-id.sfv-size 43
                    && not (dma-contains-str hdr-buf h-id "Content-Encoding:")
                    && dma-contains-str hdr-buf h-id "Vary: Accept-Encoding\r\n") in

                -- 5. Fallback when .zst/.br missing on disk: /style.css with zstd,br,gzip -> selects .gz (14B)
                let p-css = copy-str-to-dma "/style.css" req-buf in
                let v-css = static-cache-lookup-dma cache req-buf p-css accept-enc-any 1000000000 in
                let ok-css = eq-u8 v-css.sfv-encoding static-enc-gzip && eq-u8 v-css.sfv-mime mime-css && eq-u64 v-css.sfv-size 14 in

                -- 6. ETag + If-None-Match (exact strong match, weak W/ list match, *, mismatch)
                let etag-len = u8-to-u32 v-id.sfv-etag-len in
                let inm-exact-hit = static-if-none-match-dma v-id.sfv-etag-addr etag-len v-id.sfv-etag-addr etag-len in
                -- Build comma-separated list `"nope", W/<etag>` in `inm-buf`
                let pref-len = copy-str-to-dma "\"nope\", " inm-buf in
                let weak-len = static-format-etag-dma (wrapping-add-i64 inm-buf (u32-to-i64 pref-len)) 64 v-id.sfv-size v-id.sfv-mtime-sec True in
                let inm-weak-hit = static-if-none-match-dma inm-buf (wrapping-add-u32 pref-len weak-len) v-id.sfv-etag-addr etag-len in
                let star-len = copy-str-to-dma "*" inm-buf in
                let inm-star-hit = static-if-none-match-dma inm-buf star-len v-id.sfv-etag-addr etag-len in
                let miss-len = copy-str-to-dma "\"dead-beef\"" inm-buf in
                let inm-miss = static-if-none-match-dma inm-buf miss-len v-id.sfv-etag-addr etag-len in
                let plan304 = static-plan-response-dma method-code-get v-id v-id.sfv-etag-addr etag-len 0 0 in
                let h304 = static-format-headers-dma hdr-buf 0 4096 plan304 clk True cc-buf cc-len in
                let ok-etag-notmod = (
                  inm-exact-hit && inm-weak-hit && inm-star-hit && not inm-miss
                    && eq-u16 plan304.srp-http-status 304
                    && eq-u8 plan304.srp-send-body 0
                    && dma-contains-str hdr-buf h304 "HTTP/1.1 304 Not Modified\r\n"
                    && dma-contains-str hdr-buf h304 "ETag: \""
                    && dma-contains-str hdr-buf h304 "Cache-Control: public, max-age=3600\r\n"
                    && dma-contains-str hdr-buf h304 "Vary: Accept-Encoding\r\n") in

                -- 7. Range 206 (closed, open-ended, suffix, clamped) + 416 (unsatisfiable/invalid)
                let r1-len = copy-str-to-dma "bytes=0-9" rng-buf in
                let p206a = static-plan-response-dma method-code-get v-id 0 0 rng-buf r1-len in
                let h206a = static-format-headers-dma hdr-buf 0 4096 p206a clk True cc-buf cc-len in
                let ok-r1 = (
                  eq-u16 p206a.srp-http-status 206
                    && eq-u64 p206a.srp-body-off 0
                    && eq-u64 p206a.srp-body-len 10
                    && dma-contains-str hdr-buf h206a "HTTP/1.1 206 Partial Content\r\n"
                    && dma-contains-str hdr-buf h206a "Content-Range: bytes 0-9/43\r\n"
                    && dma-contains-str hdr-buf h206a "Content-Length: 10\r\n") in

                let r2-len = copy-str-to-dma "bytes=33-" rng-buf in
                let p206b = static-plan-response-dma method-code-get v-id 0 0 rng-buf r2-len in
                let h206b = static-format-headers-dma hdr-buf 0 4096 p206b clk True cc-buf cc-len in
                let ok-r2 = (
                  eq-u16 p206b.srp-http-status 206
                    && eq-u64 p206b.srp-body-off 33
                    && eq-u64 p206b.srp-body-len 10
                    && dma-contains-str hdr-buf h206b "Content-Range: bytes 33-42/43\r\n") in

                let r3-len = copy-str-to-dma "bytes=-5" rng-buf in
                let p206c = static-plan-response-dma method-code-get v-id 0 0 rng-buf r3-len in
                let h206c = static-format-headers-dma hdr-buf 0 4096 p206c clk True cc-buf cc-len in
                let ok-r3 = (
                  eq-u16 p206c.srp-http-status 206
                    && eq-u64 p206c.srp-body-off 38
                    && eq-u64 p206c.srp-body-len 5
                    && dma-contains-str hdr-buf h206c "Content-Range: bytes 38-42/43\r\n") in

                let r4-len = copy-str-to-dma "bytes=100-200" rng-buf in
                let p416a = static-plan-response-dma method-code-get v-id 0 0 rng-buf r4-len in
                let h416a = static-format-headers-dma hdr-buf 0 4096 p416a clk True cc-buf cc-len in
                let r5-len = copy-str-to-dma "bytes=20-10" rng-buf in
                let p416b = static-plan-response-dma method-code-get v-id 0 0 rng-buf r5-len in
                let r6-len = copy-str-to-dma "bytes=-0" rng-buf in
                let p416c = static-plan-response-dma method-code-get v-id 0 0 rng-buf r6-len in
                let r7-len = copy-str-to-dma "items=0-5" rng-buf in
                let p416d = static-plan-response-dma method-code-get v-id 0 0 rng-buf r7-len in
                let ok-unsat = (
                  eq-u16 p416a.srp-http-status 416
                    && eq-u8 p416a.srp-send-body 0
                    && eq-u16 p416b.srp-http-status 416
                    && eq-u16 p416c.srp-http-status 416
                    && eq-u16 p416d.srp-http-status 416
                    && dma-contains-str hdr-buf h416a "HTTP/1.1 416 Range Not Satisfiable\r\n"
                    && dma-contains-str hdr-buf h416a "Content-Range: bytes */43\r\n"
                    && dma-contains-str hdr-buf h416a "Content-Length: 0\r\n") in

                -- 8. HEAD (headers with full Content-Length, 0 body bytes) + POST (405) + directory (403) + /sub/ (index.html)
                let p-head = static-plan-response-dma method-code-head v-id 0 0 0 0 in
                let p-post = static-plan-response-dma method-code-post v-id 0 0 0 0 in
                let h-post = static-format-headers-dma hdr-buf 0 4096 p-post clk True cc-buf cc-len in
                let p-dir = copy-str-to-dma "/sub" req-buf in
                let v-dir = static-cache-lookup-dma cache req-buf p-dir 0 1000000000 in
                let p-subdir = copy-str-to-dma "/sub/" req-buf in
                let v-subdir = static-cache-lookup-dma cache req-buf p-subdir 0 1000000000 in
                let ok-head-dir = (
                  eq-u16 p-head.srp-http-status 200
                    && eq-u8 p-head.srp-send-body 0
                    && eq-u64 p-head.srp-body-len 0
                    && eq-u64 p-head.srp-content-length 43
                    && eq-u16 p-post.srp-http-status 405
                    && dma-contains-str hdr-buf h-post "Allow: GET, HEAD\r\n"
                    && eq-u8 v-dir.sfv-status static-lookup-forbidden
                    && eq-u8 v-subdir.sfv-status static-lookup-ok
                    && eq-u64 v-subdir.sfv-size 10) in

                let _d1 = static-cache-destroy cache in

                -- === Part B: LRU Eviction + TTL Refresh against real files ===
                case static-cache-new root-b idx-b 2 5000000000 of
                  None ->
                    let _f = engine-dma-free dma 16384 in
                    let _dc = coarse-clock-destroy clk in 5
                  Some lru ->
                    let pa = copy-str-to-dma "/a.txt" req-buf in
                    let va1 = static-cache-lookup-dma lru req-buf pa 0 1000000000 in
                    let pb = copy-str-to-dma "/b.txt" req-buf in
                    let vb1 = static-cache-lookup-dma lru req-buf pb 0 1000000000 in
                    -- Touch /a.txt so it becomes MRU and /b.txt becomes LRU
                    let pa2 = copy-str-to-dma "/a.txt" req-buf in
                    let va2 = static-cache-lookup-dma lru req-buf pa2 0 2000000000 in
                    -- Insert /c.txt -> evicts /b.txt (closes b's FD), keeps /a.txt
                    let pc = copy-str-to-dma "/c.txt" req-buf in
                    let vc1 = static-cache-lookup-dma lru req-buf pc 0 2000000000 in
                    let ev1 = static-cache-evictions lru in
                    let cnt1 = static-cache-count lru in
                    -- /a.txt is still a cache HIT!
                    let pa3 = copy-str-to-dma "/a.txt" req-buf in
                    let va3 = static-cache-lookup-dma lru req-buf pa3 0 2500000000 in
                    -- /b.txt was evicted -> MISS, evicts /c.txt!
                    let pb2 = copy-str-to-dma "/b.txt" req-buf in
                    let vb2 = static-cache-lookup-dma lru req-buf pb2 0 2500000000 in
                    let ev2 = static-cache-evictions lru in

                    -- Mutate a.txt on disk from 5 bytes ("alpha") to 16 bytes ("alpha-updated-16")
                    let a-path = string-to-bytes "__A_FILE_PATH__" in
                    let oflags = wrapping-i64-to-i32 (bit-or-i64 (bit-or-i64 o-rdwr o-creat) o-trunc) in
                    let updated-ok = (
                      case darwin-open-direct a-path oflags of
                        Err _ -> False
                        Ok wfd ->
                          let nlen = copy-str-to-dma "alpha-updated-16" hdr-buf in
                          let nw = darwin-pwrite-dma wfd hdr-buf nlen 0 in
                          let _cl = darwin-close-fd wfd in
                          eq-i32 nw 16) in
                    -- Before TTL (t = 3s, loaded at 1s -> delta = 2s < 5s): still HIT with size 5
                    let pa4 = copy-str-to-dma "/a.txt" req-buf in
                    let va-pre-ttl = static-cache-lookup-dma lru req-buf pa4 0 3000000000 in
                    -- After TTL (t = 7s, loaded at 1s -> delta = 6s >= 5s): refreshed MISS with size 16!
                    let va-post-ttl = static-cache-lookup-dma lru req-buf pa4 0 7000000000 in

                    let ok-lru-ttl = (
                      eq-u8 va1.sfv-cache-hit 0 && eq-u64 va1.sfv-size 5
                        && eq-u8 vb1.sfv-cache-hit 0 && eq-u64 vb1.sfv-size 7
                        && eq-u8 va2.sfv-cache-hit 1
                        && eq-u8 vc1.sfv-cache-hit 0 && eq-u64 vc1.sfv-size 9
                        && eq-u64 ev1 1 && eq-u32 cnt1 2
                        && eq-u8 va3.sfv-cache-hit 1
                        && eq-u8 vb2.sfv-cache-hit 0 && eq-u64 ev2 2
                        && updated-ok
                        && eq-u8 va-pre-ttl.sfv-cache-hit 1 && eq-u64 va-pre-ttl.sfv-size 5
                        && eq-u8 va-post-ttl.sfv-cache-hit 0 && eq-u64 va-post-ttl.sfv-size 16) in

                    let _d2 = static-cache-destroy lru in
                    let _f = engine-dma-free dma 16384 in
                    let _dc = coarse-clock-destroy clk in
                    if not ok-zst then 10
                    else if not ok-br then 11
                    else if not ok-gz then 12
                    else if not ok-id then 13
                    else if not ok-css then 14
                    else if not ok-etag-notmod then 15
                    else if not ok-r1 then 16
                    else if not ok-r2 then 17
                    else if not ok-r3 then 18
                    else if not ok-unsat then 19
                    else if not ok-head-dir then 20
                    else if not ok-lru-ttl then 21
                    else 0
"#
    .replace("__ROOT_DIR__", &root_esc)
    .replace("__A_FILE_PATH__", &a_path_esc);

    build_and_run_leak_checked("mime-variants-etag-range-lru-ttl", &src, 0);
    let _ = std::fs::remove_dir_all(&root);
}

// =========================================================================
// 4. Live macOS loopback 256 KiB `sendfile` + 206 range + 304 conditional +
//    25,000 cache-hit header-formatting zero-alloc hot loop
// =========================================================================

#[cfg(target_os = "macos")]
#[test]
fn live_macos_loopback_sendfile_256kib_and_25000_cache_hit_zero_alloc() {
    let root = fresh_dir("sendfile-www");
    // Deterministic 256 KiB binary payload: byte[i] = ((i * 73 + 19) & 0xFF)
    let mut payload = vec![0u8; 262_144];
    for (i, b) in payload.iter_mut().enumerate() {
        *b = ((i.wrapping_mul(73).wrapping_add(19)) & 0xFF) as u8;
    }
    std::fs::write(root.join("payload.bin"), &payload).expect("write 256 KiB payload.bin");
    let root_esc = escape_cinch_str(&root.to_string_lossy());

    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Engine.Reactor.Darwin
  import Engine.Timer
  import Engine.Types
  import Net.Http.Types
  import Net.Http.Static
  import Prim.Atomic

  copy-str-to-dma :: String -> I64 ->{MVar} U32
  copy-str-to-dma s dst =
    let b = string-to-bytes s in
    let n = bytes-len b in
    let _ = copy-bytes-loop b 0 dst n in
    wrapping-i64-to-u32 (nat-to-i64 n)

  copy-bytes-loop :: Bytes -> Nat -> I64 -> Nat ->{MVar} Unit
  copy-bytes-loop b i dst n =
    if i >= n then ()
    else if i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 dst (nat-to-i64 i)) (bytes-get-u8 b i) in
      copy-bytes-loop b (i + 1) dst n
    else ()

  accept-wait :: DarwinReactor -> Fd -> I64 -> U32 ->{IO, MVar} Fd
  accept-wait rx lfd cqe-buf budget =
    if eq-u32 budget 0 then fd-invalid ()
    else
      case darwin-tcp-accept-nonblock lfd of
        Ok cfd -> cfd
        Err IoAgain ->
          let _ = darwin-reactor-poll rx 100000000 cqe-buf 16 in
          accept-wait rx lfd cqe-buf (wrapping-sub-u32 budget 1)
        Err _ -> fd-invalid ()

  -- Drive headers (`darwin-send-dma`) + zero-copy body (`static-send-file-darwin`
  -- -> `darwin-sendfile-fd`) concurrently with non-blocking client `darwin-recv-dma`
  -- over live macOS loopback until `hdr-len + body-len` bytes are received.
  serve-and-recv
    :: DarwinReactor -> Fd -> Fd -> I64 -> U32 -> StaticResponsePlan -> I64 -> I64 -> U32 -> U64 -> U64 -> U32 ->{IO, MVar} I32
  serve-and-recv rx srv cli hdr-buf hdr-len plan rx-buf cqe-buf hdr-sent body-sent recvd budget =
    let total = wrapping-add-u64 (u32-to-u64 hdr-len) plan.srp-body-len in
    if ge-u64 recvd total then 0
    else if eq-u32 budget 0 then -60
    else
      -- Step 1: flush remaining header bytes if any
      let next-hs = (
        if lt-u32 hdr-sent hdr-len then
          let hrem = wrapping-sub-u32 hdr-len hdr-sent in
          let nw = darwin-send-dma srv (wrapping-add-i64 hdr-buf (u32-to-i64 hdr-sent)) hrem in
          if gt-i32 nw 0 then wrapping-add-u32 hdr-sent (wrapping-i32-to-u32 nw) else hdr-sent
        else hdr-sent) in
      -- Step 2: once headers are sent, stream body via `darwin-sendfile-fd`
      let next-bs = (
        if ge-u32 next-hs hdr-len && lt-u64 body-sent plan.srp-body-len then
          let sf = static-send-file-darwin srv plan body-sent in
          if gt-i64 sf 0 then wrapping-add-u64 body-sent (wrapping-i64-to-u64 sf) else body-sent
        else body-sent) in
      -- Step 3: drain client socket into `rx-buf`
      let rx-rem-u64 = wrapping-sub-u64 total recvd in
      let rx-want = if gt-u64 rx-rem-u64 65536 then (65536 :: U32) else wrapping-u64-to-u32 rx-rem-u64 in
      let nr = darwin-recv-dma cli (wrapping-add-i64 rx-buf (wrapping-u64-to-i64 recvd)) rx-want in
      let next-recvd = (
        if gt-i32 nr 0 then wrapping-add-u64 recvd (u32-to-u64 (wrapping-i32-to-u32 nr)) else recvd) in
      if ge-u64 next-recvd total then 0
      else
        let _p = darwin-reactor-poll rx 50000000 cqe-buf 16 in
        serve-and-recv rx srv cli hdr-buf hdr-len plan rx-buf cqe-buf next-hs next-bs next-recvd (wrapping-sub-u32 budget 1)

  -- Verify every byte of `[addr, addr + len)` matches `((file_off + i) * 73 + 19) & 0xFF`.
  verify-payload :: I64 -> U64 -> U64 -> U64 ->{MVar} Bool
  verify-payload addr file-off i len =
    if ge-u64 i len then True
    else
      let idx = wrapping-add-u64 file-off i in
      let expect = wrapping-u64-to-u8 (wrapping-add-u64 (wrapping-mul-u64 idx 73) 19) in
      let got = raw-load-u8 (wrapping-add-i64 addr (wrapping-u64-to-i64 i)) in
      if ne-u8 got expect then False
      else verify-payload addr file-off (wrapping-add-u64 i 1) len

  dma-eq-bytes :: I64 -> I64 -> U32 -> U32 ->{MVar} Bool
  dma-eq-bytes a b i n =
    if ge-u32 i n then True
    else if ne-u8 (raw-load-u8 (wrapping-add-i64 a (u32-to-i64 i))) (raw-load-u8 (wrapping-add-i64 b (u32-to-i64 i))) then False
    else dma-eq-bytes a b (wrapping-add-u32 i 1) n

  -- 25,000-iteration steady-state cache-hit lookup + plan + DMA header formatting hot loop.
  -- MUST execute with 0 heap allocations and 0 atomic RC operations!
  #[no-alloc]
  hot-loop25000
    :: StaticFileCache -> CoarseClockCache -> I64 -> U32 -> I64 -> U32 -> I64 -> U32 -> U32 ->{MVar} U32
  hot-loop25000 cache clk req-addr req-len cc-addr cc-len hdr-buf i limit =
    if ge-u32 i limit then limit
    else
      let view = static-cache-get-hit-dma cache req-addr req-len 0 2000000000 in
      if ne-u8 view.sfv-status static-lookup-ok then 0
      else
        let plan = static-plan-response-dma method-code-get view 0 0 0 0 in
        let n = static-format-headers-dma hdr-buf 0 4096 plan clk (eq-u32 0 0) cc-addr cc-len in
        if gt-u32 n 4096 then 0
        else hot-loop25000 cache clk req-addr req-len cc-addr cc-len hdr-buf (wrapping-add-u32 i 1) limit

  main :: Unit ->{IO, MVar} Int
  main _ =
    case coarse-clock-new () of
      None -> 1
      Some clk ->
        let _rf = coarse-date-refresh clk 1764338400 in
        -- Allocate 320 KiB DMA slab:
        --   +0      : scratch / req / range / cc / hdr / cqe (32 KiB)
        --   +32768  : client rx buffer (288 KiB, fits headers + 256 KiB payload)
        let dma = engine-dma-alloc 327680 4096 in
        if eq-u64 dma 0 then
          let _dc = coarse-clock-destroy clk in 2
        else
          let slab = wrapping-u64-to-i64 dma in
          let req-buf = slab in
          let rng-buf = wrapping-add-i64 slab 1024 in
          let cc-buf = wrapping-add-i64 slab 2048 in
          let hdr-buf = wrapping-add-i64 slab 4096 in
          let cqe-buf = wrapping-add-i64 slab 8192 in
          let rx-buf = wrapping-add-i64 slab 32768 in
          let cc-len = copy-str-to-dma "public, max-age=60" cc-buf in
          let req-len = copy-str-to-dma "/payload.bin" req-buf in
          let root-b = string-to-bytes "__ROOT_DIR__" in
          let idx-b = string-to-bytes "index.html" in
          case static-cache-new root-b idx-b 16 60000000000 of
            None ->
              let _f = engine-dma-free dma 327680 in
              let _dc = coarse-clock-destroy clk in 3
            Some cache ->
              case darwin-reactor-new 128 of
                Err _ ->
                  let _d = static-cache-destroy cache in
                  let _f = engine-dma-free dma 327680 in
                  let _dc = coarse-clock-destroy clk in 4
                Ok rx ->
                  case darwin-tcp-listen-reuseport 0 16 of
                    Err _ ->
                      let _dr = darwin-reactor-destroy rx in
                      let _d = static-cache-destroy cache in
                      let _f = engine-dma-free dma 327680 in
                      let _dc = coarse-clock-destroy clk in 5
                    Ok (lfd, port) ->
                      let _al = darwin-reactor-arm-read rx lfd (io-token-pack op-code-accept 1 1) in
                      case darwin-tcp-connect-nonblock ipv4-loopback port of
                        Err _ ->
                          let _cl = darwin-close-fd lfd in
                          let _dr = darwin-reactor-destroy rx in
                          let _d = static-cache-destroy cache in
                          let _f = engine-dma-free dma 327680 in
                          let _dc = coarse-clock-destroy clk in 6
                        Ok cli ->
                          let _p0 = darwin-reactor-poll rx 500000000 cqe-buf 16 in
                          let srv = accept-wait rx lfd cqe-buf 64 in
                          if not (fd-is-valid srv) then
                            let _c1 = darwin-close-fd cli in
                            let _c2 = darwin-close-fd lfd in
                            let _dr = darwin-reactor-destroy rx in
                            let _d = static-cache-destroy cache in
                            let _f = engine-dma-free dma 327680 in
                            let _dc = coarse-clock-destroy clk in 7
                          else
                            let _a1 = darwin-reactor-arm-read rx cli (io-token-pack op-code-recv 1 2) in
                            let _a2 = darwin-reactor-arm-write rx srv (io-token-pack op-code-send 1 3) in
                            let _p1 = darwin-reactor-poll rx 0 cqe-buf 16 in

                            -- === Transfer 1: Full 256 KiB GET via darwin-sendfile-fd ===
                            let view1 = static-cache-lookup-dma cache req-buf req-len 0 1000000000 in
                            let plan1 = static-plan-response-dma method-code-get view1 0 0 0 0 in
                            let hlen1 = static-format-headers-dma hdr-buf 0 4096 plan1 clk True cc-buf cc-len in
                            let rc1 = serve-and-recv rx srv cli hdr-buf hlen1 plan1 rx-buf cqe-buf 0 0 0 512 in
                            let ok200 = (
                              eq-i32 rc1 0
                                && eq-u16 plan1.srp-http-status 200
                                && eq-u64 plan1.srp-body-len 262144
                                && dma-eq-bytes rx-buf hdr-buf 0 hlen1
                                && verify-payload (wrapping-add-i64 rx-buf (u32-to-i64 hlen1)) 0 0 262144) in

                            -- === Transfer 2: 206 Partial Content (64 KiB slice bytes=65536-131071) ===
                            let view2 = static-cache-lookup-dma cache req-buf req-len 0 1500000000 in
                            let rng-len = copy-str-to-dma "bytes=65536-131071" rng-buf in
                            let plan2 = static-plan-response-dma method-code-get view2 0 0 rng-buf rng-len in
                            let hlen2 = static-format-headers-dma hdr-buf 0 4096 plan2 clk True cc-buf cc-len in
                            let rc2 = serve-and-recv rx srv cli hdr-buf hlen2 plan2 rx-buf cqe-buf 0 0 0 512 in
                            let ok206 = (
                              eq-i32 rc2 0
                                && eq-u8 view2.sfv-cache-hit 1
                                && eq-u16 plan2.srp-http-status 206
                                && eq-u64 plan2.srp-body-off 65536
                                && eq-u64 plan2.srp-body-len 65536
                                && dma-eq-bytes rx-buf hdr-buf 0 hlen2
                                && verify-payload (wrapping-add-i64 rx-buf (u32-to-i64 hlen2)) 65536 0 65536) in

                            -- === Transfer 3: 304 Not Modified conditional hit ===
                            let plan3 = static-plan-response-dma method-code-get view2 view2.sfv-etag-addr (u8-to-u32 view2.sfv-etag-len) 0 0 in
                            let hlen3 = static-format-headers-dma hdr-buf 0 4096 plan3 clk False cc-buf cc-len in
                            let rc3 = serve-and-recv rx srv cli hdr-buf hlen3 plan3 rx-buf cqe-buf 0 0 0 128 in
                            let ok304 = (
                              eq-i32 rc3 0
                                && eq-u16 plan3.srp-http-status 304
                                && eq-u8 plan3.srp-send-body 0
                                && eq-u64 plan3.srp-body-len 0
                                && dma-eq-bytes rx-buf hdr-buf 0 hlen3) in

                            let _c1 = darwin-close-fd srv in
                            let _c2 = darwin-close-fd cli in
                            let _c3 = darwin-close-fd lfd in
                            let _dr = darwin-reactor-destroy rx in

                            -- === Part 4: 25,000-iteration steady-state cache-hit header-format hot loop ===
                            let _rst = engine-thread-counters-reset () in
                            let done = hot-loop25000 cache clk req-buf req-len cc-buf cc-len hdr-buf 0 25000 in
                            let allocs = engine-thread-alloc-count () in
                            let atomic-rcs = engine-thread-atomic-rc-count () in
                            let total-hits = static-cache-hits cache in

                            let _dc2 = static-cache-destroy cache in
                            let _f = engine-dma-free dma 327680 in
                            let _dclk = coarse-clock-destroy clk in
                            if not ok200 then 10
                            else if not ok206 then 11
                            else if not ok304 then 12
                            else if ne-u32 done 25000 then 13
                            else if lt-u64 total-hits 25001 then 14
                            else if ne-u64 allocs 0 then 15
                            else if ne-u64 atomic-rcs 0 then 16
                            else 0
"#
    .replace("__ROOT_DIR__", &root_esc);

    build_and_run_leak_checked("live-sendfile-256kib-25k-hot", &src, 0);
    let _ = std::fs::remove_dir_all(&root);
}
