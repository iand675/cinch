//! Integration, POD ABI layout, `#[no-alloc]` verification, RESP2/RESP3
//! conformance & DoS corpus, 1,000-trial `Effect.Test.Hegel` differential
//! round-trip & split-boundary fuzzing, and 100,000-command steady-state
//! pipelined zero-allocation hot-loop tests for `Store.Kv.Resp`.
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
use std::time::Instant;

const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn fresh_dir(tag: &str) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "cinch-store-kv-resp-{tag}-{}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn build_and_run_leak_checked(tag: &str, src: &str, expected_code: i32) -> (String, String) {
    let dir = fresh_dir(tag);
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let exe = dir.join(format!("resp-{tag}"));
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
// 1. Manifest registration, POD ABI layout, and #[no-alloc] interface check
// =========================================================================

#[test]
fn store_kv_resp_manifest_pod_abi_and_no_alloc_check() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    let entry = pkg
        .modules
        .iter()
        .find(|m| m.name == "Store.Kv.Resp")
        .expect("Store.Kv.Resp missing from seed/prelude/.cinchpkg modules");
    assert_eq!(entry.path, "Store/Kv/Resp.cinch");

    let pos_scan = pkg
        .order
        .iter()
        .position(|n| n == "Engine.Simd.Scan")
        .expect("Engine.Simd.Scan in order");
    let pos_types = pkg
        .order
        .iter()
        .position(|n| n == "Engine.Types")
        .expect("Engine.Types in order");
    let pos_resp = pkg
        .order
        .iter()
        .position(|n| n == "Store.Kv.Resp")
        .expect("Store.Kv.Resp in order");
    assert!(pos_scan < pos_resp && pos_types < pos_resp);

    // Compile-time #[no-alloc] audit + runtime POD ABI & enum round-trip check
    let src = r##"
module Main where
  import Data.Bool
  import Data.Machine
  import Engine.Types
  import Prim.Atomic
  import Store.Kv.Resp

  #[no-alloc]
  audit-no-alloc-hot-path :: I64 -> I64 -> I64 -> I64 -> I64 ->{MVar} U32
  audit-no-alloc-hot-path rx-base cmds-out args-out tx-base enc-addr =
    let _e0 = resp-cmd-view-empty () in
    let _e1 = resp-batch-empty () in
    let _e2 = resp-value-empty () in
    let cv = resp-parse-command rx-base 0 32 1 args-out 0 16 in
    let br = resp-extract-pipeline-batch rx-base 0 64 1 cmds-out 16 args-out 64 16 in
    let rv = resp-parse-value rx-base 0 32 1 in
    let _rvh = resp-parse-value-head rx-base 0 32 1 in
    let _sp0 = resp-value-span rv in
    let _sp1 = resp-value-verbatim-enc-span rv in
    let _cm = resp-match-cmd-dma rx-base 0 3 in
    let _sa = resp-store-arg args-out 0 (byte-span-make 1 2 3 0) in
    let _la = resp-load-arg args-out 0 in
    let _sc = resp-store-cmd cmds-out 0 cv in
    let _lc = resp-load-cmd cmds-out 0 in
    let p0 = resp-write-ok tx-base 0 512 in
    let p1 = resp-write-pong tx-base p0 512 in
    let p2 = resp-write-null-v2 tx-base p1 512 in
    let p3 = resp-write-null-v3 tx-base p2 512 in
    let p4 = resp-write-zero tx-base p3 512 in
    let p5 = resp-write-one tx-base p4 512 in
    let p6 = resp-write-null-array tx-base p5 512 in
    let p7 = resp-write-simple-str tx-base p6 512 enc-addr 3 in
    let p8 = resp-write-error tx-base p7 512 enc-addr 3 in
    let p9 = resp-write-int tx-base p8 512 (-42) in
    let p10 = resp-write-bool tx-base p9 512 (eq-u32 0 0) in
    let p11 = resp-write-bulk-raw tx-base p10 512 enc-addr 3 in
    let p12 = resp-write-bulk-error tx-base p11 512 enc-addr 3 in
    let p13 = resp-write-verbatim tx-base p12 512 enc-addr enc-addr 3 in
    let p14 = resp-write-double-raw tx-base p13 512 enc-addr 3 in
    let p15 = resp-write-bignum-raw tx-base p14 512 enc-addr 3 in
    let p16 = resp-write-array-header tx-base p15 512 2 in
    let p17 = resp-write-map-header tx-base p16 512 1 in
    let p18 = resp-write-set-header tx-base p17 512 1 in
    let p19 = resp-write-push-header tx-base p18 512 1 in
    wrapping-add-u32 br.rb-bytes-consumed p19

  check-status-rt :: U8 -> Bool
  check-status-rt c =
    if gt-u8 c 17 then True
    else
      let st = resp-status-from-u8 c in
      if eq-u8 (resp-status-to-u8 st) c && resp-status-eq st st then
        check-status-rt (wrapping-add-u8 c 1)
      else False

  check-kind-rt :: U8 -> Bool
  check-kind-rt c =
    if gt-u8 c 15 then True
    else
      let k = resp-val-kind-from-u8 c in
      if eq-u8 (resp-val-kind-to-u8 k) c && resp-val-kind-eq k k then
        check-kind-rt (wrapping-add-u8 c 1)
      else False

  check-cmd-rt :: U8 -> Bool
  check-cmd-rt c =
    if gt-u8 c 15 then True
    else
      let k = resp-cmd-kind-from-u8 c in
      if eq-u8 (resp-cmd-kind-to-u8 k) c && resp-cmd-kind-eq k k then
        check-cmd-rt (wrapping-add-u8 c 1)
      else False

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 8192 4096 in
    if eq-u64 dma 0 then 99
    else
      let base = wrapping-u64-to-i64 dma in
      let cmds = wrapping-add-i64 base 1024 in
      let args = wrapping-add-i64 base 2048 in
      let tx = wrapping-add-i64 base 3072 in
      let enc = wrapping-add-i64 base 4096 in
      let _0 = raw-store-u8 enc 116 in
      let _1 = raw-store-u8 (wrapping-add-i64 enc 1) 120 in
      let _2 = raw-store-u8 (wrapping-add-i64 enc 2) 116 in
      let _run = audit-no-alloc-hot-path base cmds args tx enc in

      -- Verify DMA POD slot round-trips and non-overlapping strides
      let _sa0 = resp-store-arg args 0 (byte-span-make 11 22 33 44) in
      let _sa1 = resp-store-arg args 1 (byte-span-make 55 66 77 88) in
      let la0 = resp-load-arg args 0 in
      let la1 = resp-load-arg args 1 in
      let ok-arg = (
        eq-u32 resp-arg-stride 16
          && byte-span-eq la0 (byte-span-make 11 22 33 44)
          && byte-span-eq la1 (byte-span-make 55 66 77 88)) in

      let cv0 = RespCmdView resp-ok cmd-set 0 0 3 0 25 7 0 in
      let cv1 = RespCmdView resp-ok cmd-get 1 0 2 3 12 7 0 in
      let _sc0 = resp-store-cmd cmds 0 cv0 in
      let _sc1 = resp-store-cmd cmds 1 cv1 in
      let lc0 = resp-load-cmd cmds 0 in
      let lc1 = resp-load-cmd cmds 1 in
      let ok-cmd-slot = (
        eq-u32 resp-cmd-stride 24
          && eq-u8 lc0.rc-status resp-ok && eq-u8 lc0.rc-cmd cmd-set && eq-u8 lc0.rc-is-inline 0
          && eq-u32 lc0.rc-argc 3 && eq-u32 lc0.rc-arg-offset 0 && eq-u32 lc0.rc-consumed 25 && eq-u32 lc0.rc-buf-id 7
          && eq-u8 lc1.rc-status resp-ok && eq-u8 lc1.rc-cmd cmd-get && eq-u8 lc1.rc-is-inline 1
          && eq-u32 lc1.rc-argc 2 && eq-u32 lc1.rc-arg-offset 3 && eq-u32 lc1.rc-consumed 12 && eq-u32 lc1.rc-buf-id 7) in

      let ok-enums = (
        check-status-rt 0 && check-kind-rt 0 && check-cmd-rt 0
          && resp-status-is-ok resp-ok
          && resp-status-is-need-more resp-need-more
          && resp-status-is-err resp-err-bad-bulk-len
          && not (resp-status-is-err resp-ok)
          && not (resp-status-is-err resp-need-more)) in

      let _ = engine-dma-free dma 8192 in
      if not ok-arg then 1
      else if not ok-cmd-slot then 2
      else if not ok-enums then 3
      else 0
"##;
    build_and_run_leak_checked("manifest-pod-no-alloc", src, 0);
}

// =========================================================================
// 2. Complete RESP2 + RESP3 conformance, 16-command pipelining, byte-by-byte
//    partial reassembly, binary-safe bulk strings, and DoS/malformed corpus
// =========================================================================

#[test]
fn resp2_and_resp3_conformance_pipelining_partial_and_dos_corpus() {
    let src = r##"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.String
  import Engine.Types
  import Prim.Atomic
  import Store.Kv.Resp

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

  check-cmd-match :: I64 -> String -> U8 ->{MVar} Bool
  check-cmd-match buf s want =
    let n = copy-str-to-dma s buf in
    eq-u8 (resp-match-cmd-dma buf 0 n) want

  check-val-need-more-prefixes :: I64 -> U32 -> U32 ->{MVar} Bool
  check-val-need-more-prefixes buf prefix full-len =
    if ge-u32 prefix full-len then
      let rv = resp-parse-value buf 0 full-len 3 in
      eq-u8 rv.rv-status resp-ok && eq-u32 rv.rv-consumed full-len
    else
      let rv = resp-parse-value buf 0 prefix 3 in
      if ne-u8 rv.rv-status resp-need-more then False
      else check-val-need-more-prefixes buf (wrapping-add-u32 prefix 1) full-len

  check-cmd-need-more-prefixes :: I64 -> I64 -> U32 -> U32 ->{MVar} Bool
  check-cmd-need-more-prefixes buf args prefix full-len =
    if ge-u32 prefix full-len then
      let cv = resp-parse-command buf 0 full-len 5 args 0 16 in
      eq-u8 cv.rc-status resp-ok && eq-u32 cv.rc-consumed full-len
    else
      let cv = resp-parse-command buf 0 prefix 5 args 0 16 in
      if ne-u8 cv.rc-status resp-need-more then False
      else check-cmd-need-more-prefixes buf args (wrapping-add-u32 prefix 1) full-len

  expect-val-err :: I64 -> String -> U8 ->{MVar} Bool
  expect-val-err buf wire want =
    let n = copy-str-to-dma wire buf in
    eq-u8 (resp-parse-value buf 0 n 0).rv-status want

  expect-cmd-err :: I64 -> I64 -> String -> U32 -> U8 ->{MVar} Bool
  expect-cmd-err buf args wire max-a want =
    let n = copy-str-to-dma wire buf in
    eq-u8 (resp-parse-command buf 0 n 0 args 0 max-a).rc-status want

  write-deep-arrays :: I64 -> U32 -> U32 -> U32 ->{MVar} U32
  write-deep-arrays buf pos i depth =
    if ge-u32 i depth then
      resp-write-ok buf pos 4096
    else
      let p1 = resp-write-array-header buf pos 4096 1 in
      write-deep-arrays buf p1 (wrapping-add-u32 i 1) depth

  -- Build a 16-command redis-benchmark -P 16 pipeline batch in DMA
  build-pipe16 :: I64 -> U32 -> U32 -> U32 ->{MVar} U32
  build-pipe16 buf pos i total =
    if ge-u32 i total then pos
    else
      let wire = (if eq-u32 (rem-u32 i 2) 0 then
          "*3\r\n$3\r\nSET\r\n$4\r\nkey0\r\n$5\r\nval00\r\n"
        else
          "*2\r\n$3\r\nGET\r\n$4\r\nkey0\r\n") in
      let n = copy-str-to-dma wire (wrapping-add-i64 buf (u32-to-i64 pos)) in
      build-pipe16 buf (wrapping-add-u32 pos n) (wrapping-add-u32 i 1) total

  verify-pipe16 :: I64 -> I64 -> I64 -> U32 -> U32 ->{MVar} Bool
  verify-pipe16 buf cmds args i total =
    if ge-u32 i total then True
    else
      let cv = resp-load-cmd cmds i in
      let a0 = resp-load-arg args cv.rc-arg-offset in
      let a1 = resp-load-arg args (wrapping-add-u32 cv.rc-arg-offset 1) in
      let ok-one = (
        if eq-u32 (rem-u32 i 2) 0 then
          let a2 = resp-load-arg args (wrapping-add-u32 cv.rc-arg-offset 2) in
          eq-u8 cv.rc-cmd cmd-set && eq-u32 cv.rc-argc 3
            && dma-eq-str buf a0.span-off a0.span-len "SET"
            && dma-eq-str buf a1.span-off a1.span-len "key0"
            && dma-eq-str buf a2.span-off a2.span-len "val00"
        else
          eq-u8 cv.rc-cmd cmd-get && eq-u32 cv.rc-argc 2
            && dma-eq-str buf a0.span-off a0.span-len "GET"
            && dma-eq-str buf a1.span-off a1.span-len "key0") in
      if ok-one then verify-pipe16 buf cmds args (wrapping-add-u32 i 1) total
      else False

  run-part-a1 :: I64 -> I64 ->{MVar} Bool
  run-part-a1 buf scratch =
    -- 1. Simple String
    let n1 = copy-str-to-dma "+OK\r\n" buf in
    let v1 = resp-parse-value buf 0 n1 1 in
    let ok-v1 = eq-u8 v1.rv-status resp-ok && eq-u8 v1.rv-kind resp-kind-simple-string && eq-u32 v1.rv-consumed n1 && dma-eq-str buf v1.rv-data-off v1.rv-data-len "OK" in
    -- 2. Simple Error
    let n2 = copy-str-to-dma "-ERR syntax error\r\n" buf in
    let v2 = resp-parse-value buf 0 n2 1 in
    let ok-v2 = eq-u8 v2.rv-status resp-ok && eq-u8 v2.rv-kind resp-kind-simple-error && eq-u32 v2.rv-consumed n2 && dma-eq-str buf v2.rv-data-off v2.rv-data-len "ERR syntax error" in
    -- 3. Integer (0, +42, -42, I64::MAX, I64::MIN)
    let n3a = copy-str-to-dma ":0\r\n" buf in
    let v3a = resp-parse-value buf 0 n3a 1 in
    let n3b = copy-str-to-dma ":+9223372036854775807\r\n" buf in
    let v3b = resp-parse-value buf 0 n3b 1 in
    let n3c = copy-str-to-dma ":-9223372036854775808\r\n" buf in
    let v3c = resp-parse-value buf 0 n3c 1 in
    let ok-v3 = (
      eq-u8 v3a.rv-status resp-ok && eq-u8 v3a.rv-kind resp-kind-integer && eq-i64 v3a.rv-int 0
        && eq-u8 v3b.rv-status resp-ok && eq-u8 v3b.rv-kind resp-kind-integer && eq-i64 v3b.rv-int 9223372036854775807
        && eq-u8 v3c.rv-status resp-ok && eq-u8 v3c.rv-kind resp-kind-integer && eq-i64 v3c.rv-int (-9223372036854775808)) in
    -- 4. Bulk String (regular, empty, binary-safe with \r\n + \0 + 0xFF, and null $-1)
    let n4a = copy-str-to-dma "$0\r\n\r\n" buf in
    let v4a = resp-parse-value buf 0 n4a 1 in
    let n4b = copy-str-to-dma "$-1\r\n" buf in
    let v4b = resp-parse-value buf 0 n4b 1 in
    let _p40 = raw-store-u8 scratch 97 in
    let _p41 = raw-store-u8 (wrapping-add-i64 scratch 1) 13 in
    let _p42 = raw-store-u8 (wrapping-add-i64 scratch 2) 10 in
    let _p43 = raw-store-u8 (wrapping-add-i64 scratch 3) 0 in
    let _p44 = raw-store-u8 (wrapping-add-i64 scratch 4) 255 in
    let _p45 = raw-store-u8 (wrapping-add-i64 scratch 5) 98 in
    let n4c = resp-write-bulk-raw buf 0 4096 scratch 6 in
    let v4c = resp-parse-value buf 0 n4c 1 in
    let ok-v4 = (
      eq-u8 v4a.rv-status resp-ok && eq-u8 v4a.rv-kind resp-kind-bulk-string && eq-u32 v4a.rv-data-len 0
        && eq-u8 v4b.rv-status resp-ok && eq-u8 v4b.rv-kind resp-kind-null-bulk && eq-i64 v4b.rv-int (-1)
        && eq-u8 v4c.rv-status resp-ok && eq-u8 v4c.rv-kind resp-kind-bulk-string && eq-u32 v4c.rv-data-len 6
        && eq-u32 v4c.rv-consumed n4c
        && eq-u8 (raw-load-u8 (wrapping-add-i64 buf (u32-to-i64 v4c.rv-data-off))) 97
        && eq-u8 (raw-load-u8 (wrapping-add-i64 buf (u32-to-i64 (wrapping-add-u32 v4c.rv-data-off 1)))) 13
        && eq-u8 (raw-load-u8 (wrapping-add-i64 buf (u32-to-i64 (wrapping-add-u32 v4c.rv-data-off 2)))) 10
        && eq-u8 (raw-load-u8 (wrapping-add-i64 buf (u32-to-i64 (wrapping-add-u32 v4c.rv-data-off 3)))) 0
        && eq-u8 (raw-load-u8 (wrapping-add-i64 buf (u32-to-i64 (wrapping-add-u32 v4c.rv-data-off 4)))) 255
        && eq-u8 (raw-load-u8 (wrapping-add-i64 buf (u32-to-i64 (wrapping-add-u32 v4c.rv-data-off 5)))) 98) in
    -- 5. Array (nested + empty + null *-1)
    let n5a = copy-str-to-dma "*0\r\n" buf in
    let v5a = resp-parse-value buf 0 n5a 1 in
    let n5b = copy-str-to-dma "*-1\r\n" buf in
    let v5b = resp-parse-value buf 0 n5b 1 in
    let n5c = copy-str-to-dma "*2\r\n*1\r\n+nested\r\n:77\r\n" buf in
    let v5c = resp-parse-value buf 0 n5c 1 in
    let ok-v5 = (
      eq-u8 v5a.rv-status resp-ok && eq-u8 v5a.rv-kind resp-kind-array && eq-i64 v5a.rv-int 0
        && eq-u8 v5b.rv-status resp-ok && eq-u8 v5b.rv-kind resp-kind-null-array && eq-i64 v5b.rv-int (-1)
        && eq-u8 v5c.rv-status resp-ok && eq-u8 v5c.rv-kind resp-kind-array && eq-i64 v5c.rv-int 2 && eq-u32 v5c.rv-consumed n5c) in
    -- 6. RESP3 Null & 7. RESP3 Boolean
    let n6 = copy-str-to-dma "_\r\n" buf in
    let v6 = resp-parse-value buf 0 n6 1 in
    let ok-v6 = eq-u8 v6.rv-status resp-ok && eq-u8 v6.rv-kind resp-kind-null && eq-u32 v6.rv-consumed 3 in
    let n7a = copy-str-to-dma "#t\r\n" buf in
    let v7a = resp-parse-value buf 0 n7a 1 in
    let n7b = copy-str-to-dma "#f\r\n" buf in
    let v7b = resp-parse-value buf 0 n7b 1 in
    let ok-v7 = (
      eq-u8 v7a.rv-status resp-ok && eq-u8 v7a.rv-kind resp-kind-bool && eq-u8 v7a.rv-bool 1
        && eq-u8 v7b.rv-status resp-ok && eq-u8 v7b.rv-kind resp-kind-bool && eq-u8 v7b.rv-bool 0) in
    ok-v1 && ok-v2 && ok-v3 && ok-v4 && ok-v5 && ok-v6 && ok-v7

  run-part-a2 :: I64 ->{MVar} Bool
  run-part-a2 buf =
    -- 8. RESP3 Double (finite, scientific, inf, -inf, nan)
    let n8a = copy-str-to-dma ",3.14159\r\n" buf in
    let v8a = resp-parse-value buf 0 n8a 1 in
    let n8b = copy-str-to-dma ",-1.25e-10\r\n" buf in
    let v8b = resp-parse-value buf 0 n8b 1 in
    let n8c = copy-str-to-dma ",inf\r\n" buf in
    let v8c = resp-parse-value buf 0 n8c 1 in
    let n8d = copy-str-to-dma ",-inf\r\n" buf in
    let v8d = resp-parse-value buf 0 n8d 1 in
    let n8e = copy-str-to-dma ",nan\r\n" buf in
    let v8e = resp-parse-value buf 0 n8e 1 in
    let ok-v8 = (
      eq-u8 v8a.rv-status resp-ok && eq-u8 v8a.rv-kind resp-kind-double
        && eq-u8 v8b.rv-status resp-ok && eq-u8 v8b.rv-kind resp-kind-double
        && eq-u8 v8c.rv-status resp-ok && eq-u8 v8c.rv-kind resp-kind-double
        && eq-u8 v8d.rv-status resp-ok && eq-u8 v8d.rv-kind resp-kind-double
        && eq-u8 v8e.rv-status resp-ok && eq-u8 v8e.rv-kind resp-kind-double) in
    -- 9. RESP3 Big Number
    let n9 = copy-str-to-dma "(3492890328409238509324850943850943825024385\r\n" buf in
    let v9 = resp-parse-value buf 0 n9 1 in
    let ok-v9 = eq-u8 v9.rv-status resp-ok && eq-u8 v9.rv-kind resp-kind-bignum && dma-eq-str buf v9.rv-data-off v9.rv-data-len "3492890328409238509324850943850943825024385" in
    -- 10. RESP3 Bulk Error
    let n10 = copy-str-to-dma "!21\r\nSYNTAX invalid syntax\r\n" buf in
    let v10 = resp-parse-value buf 0 n10 1 in
    let ok-v10 = eq-u8 v10.rv-status resp-ok && eq-u8 v10.rv-kind resp-kind-bulk-error && dma-eq-str buf v10.rv-data-off v10.rv-data-len "SYNTAX invalid syntax" in
    -- 11. RESP3 Verbatim String
    let n11 = copy-str-to-dma "=15\r\ntxt:Some string\r\n" buf in
    let v11 = resp-parse-value buf 0 n11 1 in
    let enc11 = resp-value-verbatim-enc-span v11 in
    let pay11 = resp-value-span v11 in
    let ok-v11 = (
      eq-u8 v11.rv-status resp-ok && eq-u8 v11.rv-kind resp-kind-verbatim && eq-u32 v11.rv-consumed n11
        && dma-eq-str buf enc11.span-off enc11.span-len "txt"
        && dma-eq-str buf pay11.span-off pay11.span-len "Some string") in
    -- 12. RESP3 Map (walk all 4 children zero-copy) + byte-by-byte partial reassembly
    let n12 = copy-str-to-dma "%2\r\n+first\r\n:1\r\n+second\r\n#t\r\n" buf in
    let ok-v12-inc = check-val-need-more-prefixes buf 0 n12 in
    let v12 = resp-parse-value buf 0 n12 1 in
    let c0 = resp-parse-value buf v12.rv-data-off n12 1 in
    let off1 = wrapping-add-u32 v12.rv-data-off c0.rv-consumed in
    let c1 = resp-parse-value buf off1 n12 1 in
    let off2 = wrapping-add-u32 off1 c1.rv-consumed in
    let c2 = resp-parse-value buf off2 n12 1 in
    let off3 = wrapping-add-u32 off2 c2.rv-consumed in
    let c3 = resp-parse-value buf off3 n12 1 in
    let ok-v12 = (
      ok-v12-inc && eq-u8 v12.rv-status resp-ok && eq-u8 v12.rv-kind resp-kind-map
        && eq-i64 v12.rv-int 2 && eq-u32 v12.rv-aux-off 4 && eq-u32 v12.rv-consumed n12
        && dma-eq-str buf c0.rv-data-off c0.rv-data-len "first"
        && eq-i64 c1.rv-int 1
        && dma-eq-str buf c2.rv-data-off c2.rv-data-len "second"
        && eq-u8 c3.rv-kind resp-kind-bool && eq-u8 c3.rv-bool 1) in
    -- 13. RESP3 Set & 14. RESP3 Push
    let n13 = copy-str-to-dma "~3\r\n+orange\r\n+apple\r\n#f\r\n" buf in
    let v13 = resp-parse-value buf 0 n13 1 in
    let ok-v13 = eq-u8 v13.rv-status resp-ok && eq-u8 v13.rv-kind resp-kind-set && eq-i64 v13.rv-int 3 && eq-u32 v13.rv-consumed n13 in
    let n14 = copy-str-to-dma ">3\r\n+pubsub\r\n+message\r\n$5\r\nhello\r\n" buf in
    let v14 = resp-parse-value buf 0 n14 1 in
    let ok-v14 = eq-u8 v14.rv-status resp-ok && eq-u8 v14.rv-kind resp-kind-push && eq-i64 v14.rv-int 3 && eq-u32 v14.rv-consumed n14 in
    ok-v8 && ok-v9 && ok-v10 && ok-v11 && ok-v12 && ok-v13 && ok-v14

  run-part-b :: I64 ->{MVar} Bool
  run-part-b buf =
    let ok-swar1 = (
      check-cmd-match buf "GET" cmd-get && check-cmd-match buf "gEt" cmd-get
        && check-cmd-match buf "SET" cmd-set && check-cmd-match buf "set" cmd-set
        && check-cmd-match buf "DEL" cmd-del && check-cmd-match buf "DeL" cmd-del
        && check-cmd-match buf "TTL" cmd-ttl && check-cmd-match buf "ttl" cmd-ttl
        && check-cmd-match buf "MGET" cmd-mget && check-cmd-match buf "mGeT" cmd-mget
        && check-cmd-match buf "MSET" cmd-mset && check-cmd-match buf "mset" cmd-mset
        && check-cmd-match buf "INCR" cmd-incr && check-cmd-match buf "InCr" cmd-incr
        && check-cmd-match buf "DECR" cmd-decr && check-cmd-match buf "decr" cmd-decr) in
    let ok-swar2 = (
      check-cmd-match buf "EXPIRE" cmd-expire && check-cmd-match buf "ExPiRe" cmd-expire
        && check-cmd-match buf "EXISTS" cmd-exists && check-cmd-match buf "exists" cmd-exists
        && check-cmd-match buf "PING" cmd-ping && check-cmd-match buf "PiNg" cmd-ping
        && check-cmd-match buf "HELLO" cmd-hello && check-cmd-match buf "hElLo" cmd-hello
        && check-cmd-match buf "QUIT" cmd-quit && check-cmd-match buf "quit" cmd-quit
        && check-cmd-match buf "COMMAND" cmd-command && check-cmd-match buf "CoMmAnD" cmd-command
        && check-cmd-match buf "INFO" cmd-info && check-cmd-match buf "iNfO" cmd-info
        && check-cmd-match buf "G@T" cmd-unknown && check-cmd-match buf "GETX" cmd-unknown
        && check-cmd-match buf "SET1" cmd-unknown && check-cmd-match buf "PING!" cmd-unknown
        && check-cmd-match buf "CUSTOMCMD" cmd-unknown && check-cmd-match buf "" cmd-unknown) in
    ok-swar1 && ok-swar2

  run-part-cd :: I64 -> I64 -> I64 -> I64 ->{MVar} Bool
  run-part-cd buf cmds args scratch =
    let ni1 = copy-str-to-dma "PING\r\n" buf in
    let ok-inl1 = check-cmd-need-more-prefixes buf args 0 ni1 in
    let ci1 = resp-parse-command buf 0 ni1 2 args 0 16 in
    let ni2 = copy-str-to-dma "  get   mykey  \r\n" buf in
    let ci2 = resp-parse-command buf 0 ni2 2 args 0 16 in
    let a20 = resp-load-arg args 0 in
    let a21 = resp-load-arg args 1 in
    let ok-inl2 = (
      eq-u8 ci2.rc-status resp-ok && eq-u8 ci2.rc-cmd cmd-get && eq-u32 ci2.rc-argc 2
        && dma-eq-str buf a20.span-off a20.span-len "get"
        && dma-eq-str buf a21.span-off a21.span-len "mykey") in
    let ni3 = copy-str-to-dma "SET k1 \"hello world\"\r\n" buf in
    let ci3 = resp-parse-command buf 0 ni3 2 args 0 16 in
    let a32 = resp-load-arg args 2 in
    let ok-inl = (
      ok-inl1 && eq-u8 ci1.rc-status resp-ok && eq-u8 ci1.rc-cmd cmd-ping && eq-u8 ci1.rc-is-inline 1 && eq-u32 ci1.rc-argc 1
        && ok-inl2
        && eq-u8 ci3.rc-status resp-ok && eq-u8 ci3.rc-cmd cmd-set && eq-u32 ci3.rc-argc 3
        && dma-eq-str buf a32.span-off a32.span-len "hello world") in
    let p-mb0 = resp-write-array-header buf 0 4096 3 in
    let l-set = copy-str-to-dma "SET" scratch in
    let p-mb1 = resp-write-bulk-raw buf p-mb0 4096 scratch l-set in
    let l-key = copy-str-to-dma "bin:key" scratch in
    let p-mb2 = resp-write-bulk-raw buf p-mb1 4096 scratch l-key in
    let _bv0 = raw-store-u8 scratch 0 in
    let _bv1 = raw-store-u8 (wrapping-add-i64 scratch 1) 13 in
    let _bv2 = raw-store-u8 (wrapping-add-i64 scratch 2) 10 in
    let _bv3 = raw-store-u8 (wrapping-add-i64 scratch 3) 42 in
    let p-mb3 = resp-write-bulk-raw buf p-mb2 4096 scratch 4 in
    let ok-mb-inc = check-cmd-need-more-prefixes buf args 0 p-mb3 in
    let p16-len = build-pipe16 buf 0 0 16 in
    let br16 = resp-extract-pipeline-batch buf 0 p16-len 9 cmds 32 args 128 16 in
    let ok-p16 = (
      eq-u8 br16.rb-status resp-ok
        && eq-u32 br16.rb-parsed-count 16
        && eq-u32 br16.rb-bytes-consumed p16-len
        && eq-u32 br16.rb-total-args 40
        && verify-pipe16 buf cmds args 0 16) in
    let part-n = copy-str-to-dma "*2\r\n$3\r\nGE" (wrapping-add-i64 buf (u32-to-i64 p16-len)) in
    let br16p = resp-extract-pipeline-batch buf 0 (wrapping-add-u32 p16-len part-n) 9 cmds 32 args 128 16 in
    let ok-p16-partial = (
      eq-u8 br16p.rb-status resp-need-more
        && eq-u32 br16p.rb-parsed-count 16
        && eq-u32 br16p.rb-bytes-consumed p16-len) in
    ok-inl && ok-mb-inc && ok-p16 && ok-p16-partial

  run-part-e :: I64 -> I64 ->{MVar} Bool
  run-part-e tx scratch =
    let w0 = resp-write-ok tx 0 512 in
    let w1 = resp-write-pong tx w0 512 in
    let w2 = resp-write-null-v2 tx w1 512 in
    let w3 = resp-write-null-v3 tx w2 512 in
    let w4 = resp-write-int tx w3 512 0 in
    let w5 = resp-write-int tx w4 512 1 in
    let w6 = resp-write-int tx w5 512 (-9223372036854775808) in
    let w7 = resp-write-int tx w6 512 9223372036854775807 in
    let w8 = resp-write-bool tx w7 512 True in
    let w9 = resp-write-bool tx w8 512 False in
    let s-hi = copy-str-to-dma "hi" scratch in
    let w10 = resp-write-simple-str tx w9 512 scratch s-hi in
    let s-err = copy-str-to-dma "ERR bad" scratch in
    let w11 = resp-write-error tx w10 512 scratch s-err in
    let s-bulk = copy-str-to-dma "world" scratch in
    let w12 = resp-write-bulk-raw tx w11 512 scratch s-bulk in
    let w13 = resp-write-array-header tx w12 512 2 in
    let w14 = resp-write-map-header tx w13 512 1 in
    dma-eq-str tx 0 w14 "+OK\r\n+PONG\r\n$-1\r\n_\r\n:0\r\n:1\r\n:-9223372036854775808\r\n:9223372036854775807\r\n#t\r\n#f\r\n+hi\r\n-ERR bad\r\n$5\r\nworld\r\n*2\r\n%1\r\n"
      && gt-u32 (resp-write-ok tx 0 4) 4
      && gt-u32 (resp-write-pong tx 0 6) 6
      && gt-u32 (resp-write-int tx 0 3 0) 3

  run-part-f :: I64 -> I64 ->{MVar} Bool
  run-part-f buf args =
    let d01 = expect-val-err buf "$-2\r\n" resp-err-bad-bulk-len in
    let d02 = expect-val-err buf "$-999\r\n" resp-err-bad-bulk-len in
    let d03 = expect-val-err buf "*-2\r\n" resp-err-bad-bulk-len in
    let d04 = expect-val-err buf "!-1\r\n" resp-err-bad-bulk-len in
    let d05 = expect-val-err buf "=-1\r\n" resp-err-bad-bulk-len in
    let d06 = expect-val-err buf "%-1\r\n" resp-err-bad-bulk-len in
    let d07 = expect-val-err buf "~-1\r\n" resp-err-bad-bulk-len in
    let d08 = expect-val-err buf ">-1\r\n" resp-err-bad-bulk-len in
    let d09 = expect-val-err buf ":9223372036854775808\r\n" resp-err-int-overflow in
    let d10 = expect-val-err buf ":-9223372036854775809\r\n" resp-err-int-overflow in
    let d11 = expect-val-err buf "$99999999999999999999\r\n" resp-err-int-overflow in
    let d12 = expect-val-err buf "$536870913\r\n" resp-err-bad-bulk-len in
    let d13 = expect-val-err buf "$3\r\nfooXX" resp-err-missing-crlf in
    let d14 = expect-val-err buf "$3\r\nfoo\rX" resp-err-missing-crlf in
    let d15 = expect-val-err buf "$3\r\nfoo\n" resp-err-bare-lf in
    let d16 = expect-val-err buf "+OK\n" resp-err-bare-lf in
    let d17 = expect-val-err buf "+OK\rX" resp-err-missing-crlf in
    let d18 = expect-val-err buf "$abc\r\n" resp-err-bad-int in
    let d19 = expect-val-err buf "$+3\r\nabc\r\n" resp-err-bad-int in
    let d20 = expect-val-err buf ":\r\n" resp-err-bad-int in
    let d21 = expect-val-err buf "_extra\r\n" resp-err-bad-null in
    let d22 = expect-val-err buf "#true\r\n" resp-err-bad-bool in
    let d23 = expect-val-err buf "#x\r\n" resp-err-bad-bool in
    let d24 = expect-val-err buf ",1.2.3\r\n" resp-err-bad-double in
    let d25 = expect-val-err buf "(123abc\r\n" resp-err-bad-bignum in
    let d26 = expect-val-err buf "=3\r\ntxt\r\n" resp-err-bad-verbatim in
    let d27 = expect-val-err buf "=5\r\ntxt-a\r\n" resp-err-bad-verbatim in
    let d28 = expect-val-err buf "?invalid\r\n" resp-err-invalid-prefix in
    let d29 = expect-cmd-err buf args "*9\r\n$1\r\na\r\n" 8 resp-err-too-many-args in
    let d30 = expect-cmd-err buf args "*1\r\n$-1\r\n" 8 resp-err-bad-bulk-len in
    let d31 = expect-cmd-err buf args "*1\r\n+PING\r\n" 8 resp-err-bad-cmd-arg in
    let d32 = expect-cmd-err buf args "PING\n" 8 resp-err-bare-lf in
    let d33 = expect-cmd-err buf args "SET k \"unterminated\r\n" 8 resp-err-bad-inline in
    let d34 = expect-cmd-err buf args "*-1\r\n" 8 resp-err-bad-bulk-len in
    let d35 = expect-cmd-err buf args "*0\r\n" 8 resp-err-bad-cmd-arg in
    let deep-len = write-deep-arrays buf 0 0 66 in
    let d36 = eq-u8 (resp-parse-value buf 0 deep-len 0).rv-status resp-err-depth-limit in
    d01 && d02 && d03 && d04 && d05 && d06 && d07 && d08 && d09 && d10 && d11 && d12
      && d13 && d14 && d15 && d16 && d17 && d18 && d19 && d20 && d21 && d22 && d23 && d24
      && d25 && d26 && d27 && d28 && d29 && d30 && d31 && d32 && d33 && d34 && d35 && d36

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 32768 4096 in
    if eq-u64 dma 0 then 99
    else
      let buf = wrapping-u64-to-i64 dma in
      let cmds = wrapping-add-i64 buf 8192 in
      let args = wrapping-add-i64 buf 12288 in
      let tx = wrapping-add-i64 buf 16384 in
      let scratch = wrapping-add-i64 buf 24576 in
      let ok-a1 = run-part-a1 buf scratch in
      let ok-a2 = run-part-a2 buf in
      let ok-b = run-part-b buf in
      let ok-cd = run-part-cd buf cmds args scratch in
      let ok-e = run-part-e tx scratch in
      let ok-f = run-part-f buf args in
      let _ = engine-dma-free dma 32768 in
      if not ok-a1 then 10
      else if not ok-a2 then 11
      else if not ok-b then 12
      else if not ok-cd then 13
      else if not ok-e then 15
      else if not ok-f then 16
      else 0
"##;
    build_and_run_leak_checked("conformance-pipelining-dos", src, 0);
}

// =========================================================================
// 3. 1,000-trial Effect.Test.Hegel differential round-trip fuzz suite over
//    randomized RESP2/RESP3 commands, binary payloads & arbitrary split points
// =========================================================================

#[test]
fn hegel_1000_trial_differential_resp_roundtrip_and_splits() {
    let src = r##"
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
  import Prim.Atomic
  import Store.Kv.Resp

  -- Structured generated command case for Hegel differential fuzzing
  record GenCmdCase where
    gc-cmd-idx :: Nat
    gc-case-mask :: Nat
    gc-args :: List (List Nat)
    gc-split-pct :: Nat

  -- Structured generated RESP3 value case for Hegel differential fuzzing
  record GenValCase where
    gv-kind-sel :: Nat
    gv-int-sign :: Bool
    gv-int-mag :: Nat
    gv-bool :: Bool
    gv-payload :: List Nat
    gv-split-pct :: Nat

  -- Byte generator biased to exercise binary-safe tricky bytes (\0, \r, \n, *, $, 0xFF)
  gen-tricky-byte :: Gen Nat
  gen-tricky-byte = gen-one-of [
    gen-pure 0,
    gen-pure 10,
    gen-pure 13,
    gen-pure 36,
    gen-pure 42,
    gen-pure 58,
    gen-pure 255,
    gen-range 32 126
  ]

  -- Safe simple-line byte generator (no \r or \n)
  gen-simple-byte :: Gen Nat
  gen-simple-byte = gen-range 32 126

  gen-cmd-case :: Gen GenCmdCase
  gen-cmd-case =
    gen-bind (gen-range 0 15) (\cmd-idx ->
    gen-bind (gen-nat 127) (\cmask ->
    gen-bind (gen-list 4 (gen-list 16 gen-tricky-byte)) (\extra-args ->
    gen-map (\sp -> GenCmdCase cmd-idx cmask extra-args sp) (gen-range 1 99))))

  gen-val-case :: Gen GenValCase
  gen-val-case =
    gen-bind (gen-range 0 12) (\ksel ->
    gen-bind gen-bool (\sgn ->
    gen-bind (gen-nat 1000000000) (\mag ->
    gen-bind gen-bool (\bv ->
    gen-bind (gen-list 16 gen-tricky-byte) (\pay ->
    gen-map (\sp -> GenValCase ksel sgn mag bv pay sp) (gen-range 1 99))))))

  cmd-name-for :: Nat -> String
  cmd-name-for idx =
    if idx == 1 then "GET"
    else if idx == 2 then "SET"
    else if idx == 3 then "DEL"
    else if idx == 4 then "MGET"
    else if idx == 5 then "MSET"
    else if idx == 6 then "INCR"
    else if idx == 7 then "DECR"
    else if idx == 8 then "EXPIRE"
    else if idx == 9 then "TTL"
    else if idx == 10 then "EXISTS"
    else if idx == 11 then "PING"
    else if idx == 12 then "HELLO"
    else if idx == 13 then "QUIT"
    else if idx == 14 then "COMMAND"
    else if idx == 15 then "INFO"
    else "XCUSTOM"

  nat-to-i64-safe :: Nat -> I64
  nat-to-i64-safe n = if n <= 9223372036854775807 then nat-to-i64 n else 0

  nat-to-u32-safe :: Nat -> U32
  nat-to-u32-safe n = wrapping-i64-to-u32 (nat-to-i64-safe n)

  nat-to-u8-safe :: Nat -> U8
  nat-to-u8-safe n = wrapping-i64-to-u8 (nat-to-i64-safe n)

  expected-cmd-code :: Nat -> U8
  expected-cmd-code idx =
    if idx <= 15 then nat-to-u8-safe idx
    else cmd-unknown

  -- Write command name into `dst` toggling ASCII case bits using `mask`.
  write-cased-name :: Bytes -> Nat -> Nat -> I64 -> Nat ->{MVar} Unit
  write-cased-name b i n dst mask =
    if i >= n then ()
    else if i < bytes-len b then
      let ch = bytes-get-u8 b i in
      let bit = bit-and-u32 (wrapping-shr-u32 (nat-to-u32-safe mask) (nat-to-u32-safe i)) 1 in
      let out-ch = if eq-u32 bit 1 then bit-or-u8 ch 32 else ch in
      let _ = raw-store-u8 (wrapping-add-i64 dst (nat-to-i64-safe i)) out-ch in
      write-cased-name b (i + 1) n dst mask
    else ()

  write-nat-list-to-dma :: List Nat -> I64 -> U32 ->{MVar} U32
  write-nat-list-to-dma xs dst off = case xs of
    Nil -> off
    Cons b rest ->
      let _ = raw-store-u8 (wrapping-add-i64 dst (u32-to-i64 off)) (nat-to-u8-safe b) in
      write-nat-list-to-dma rest dst (wrapping-add-u32 off 1)

  -- Strip \r and \n when writing simple strings / errors
  write-simple-list-to-dma :: List Nat -> I64 -> U32 ->{MVar} U32
  write-simple-list-to-dma xs dst off = case xs of
    Nil -> off
    Cons b rest ->
      let b8 = nat-to-u8-safe b in
      let safe = if eq-u8 b8 10 || eq-u8 b8 13 then 95 else b8 in
      let _ = raw-store-u8 (wrapping-add-i64 dst (u32-to-i64 off)) safe in
      write-simple-list-to-dma rest dst (wrapping-add-u32 off 1)

  eq-dma-nat-list :: I64 -> U32 -> List Nat ->{MVar} Bool
  eq-dma-nat-list base off xs = case xs of
    Nil -> True
    Cons b rest ->
      let got = raw-load-u8 (wrapping-add-i64 base (u32-to-i64 off)) in
      if eq-u8 got (nat-to-u8-safe b) then
        eq-dma-nat-list base (wrapping-add-u32 off 1) rest
      else False

  eq-dma-simple-list :: I64 -> U32 -> List Nat ->{MVar} Bool
  eq-dma-simple-list base off xs = case xs of
    Nil -> True
    Cons b rest ->
      let b8 = nat-to-u8-safe b in
      let safe = if eq-u8 b8 10 || eq-u8 b8 13 then 95 else b8 in
      let got = raw-load-u8 (wrapping-add-i64 base (u32-to-i64 off)) in
      if eq-u8 got safe then eq-dma-simple-list base (wrapping-add-u32 off 1) rest
      else False

  list-length :: forall a. List a -> U32
  list-length xs = case xs of
    Nil -> 0
    Cons _ r -> wrapping-add-u32 1 (list-length r)

  encode-extra-args :: I64 -> U32 -> I64 -> List (List Nat) ->{MVar} U32
  encode-extra-args buf pos scratch xss = case xss of
    Nil -> pos
    Cons xs rest ->
      let len = write-nat-list-to-dma xs scratch 0 in
      let p1 = resp-write-bulk-raw buf pos 4096 scratch len in
      encode-extra-args buf p1 scratch rest

  verify-extra-args :: I64 -> I64 -> U32 -> List (List Nat) ->{MVar} Bool
  verify-extra-args buf args idx xss = case xss of
    Nil -> True
    Cons xs rest ->
      let sp = resp-load-arg args idx in
      let exp-len = list-length xs in
      if ne-u32 sp.span-len exp-len then False
      else if not (eq-dma-nat-list buf sp.span-off xs) then False
      else verify-extra-args buf args (wrapping-add-u32 idx 1) rest

  -- Differential check of one generated RESP command + arbitrary split boundary + pipeline batch
  check-one-cmd :: I64 -> I64 -> I64 -> I64 -> GenCmdCase ->{MVar} Bool
  check-one-cmd buf cmds args scratch gc =
    let argc = wrapping-add-u32 1 (list-length gc.gc-args) in
    let p0 = resp-write-array-header buf 0 4096 argc in
    let name-b = string-to-bytes (cmd-name-for gc.gc-cmd-idx) in
    let name-n = bytes-len name-b in
    let _wn = write-cased-name name-b 0 name-n scratch gc.gc-case-mask in
    let p1 = resp-write-bulk-raw buf p0 4096 scratch (nat-to-u32-safe name-n) in
    let total = encode-extra-args buf p1 scratch gc.gc-args in
    if gt-u32 total 4096 then False
    else
      -- Arbitrary split boundary must yield RespNeedMore
      let split-raw = div-u32 (wrapping-mul-u32 total (nat-to-u32-safe gc.gc-split-pct)) 100 in
      let split = if eq-u32 split-raw 0 then 1 else if ge-u32 split-raw total then wrapping-sub-u32 total 1 else split-raw in
      let cv-part = resp-parse-command buf 0 split 4 args 0 16 in
      if ne-u8 cv-part.rc-status resp-need-more then False
      else
        -- Full command must decode identically
        let cv = resp-parse-command buf 0 total 4 args 0 16 in
        let ok-single = (
          eq-u8 cv.rc-status resp-ok
            && eq-u8 cv.rc-cmd (expected-cmd-code gc.gc-cmd-idx)
            && eq-u32 cv.rc-argc argc
            && eq-u32 cv.rc-consumed total
            && verify-extra-args buf args 1 gc.gc-args) in
        if not ok-single then False
        else
          -- Duplicate command back-to-back into a 2-command pipeline + split second command
          let _dup = copy-dma-range (wrapping-add-i64 buf (u32-to-i64 total)) buf total in
          let split2 = wrapping-add-u32 total split in
          let br-part = resp-extract-pipeline-batch buf 0 split2 4 cmds 8 args 64 16 in
          let br-full = resp-extract-pipeline-batch buf 0 (wrapping-add-u32 total total) 4 cmds 8 args 64 16 in
          eq-u8 br-part.rb-status resp-need-more
            && eq-u32 br-part.rb-parsed-count 1
            && eq-u32 br-part.rb-bytes-consumed total
            && eq-u8 br-full.rb-status resp-ok
            && eq-u32 br-full.rb-parsed-count 2
            && eq-u32 br-full.rb-bytes-consumed (wrapping-add-u32 total total)

  copy-dma-range :: I64 -> I64 -> U32 ->{MVar} Unit
  copy-dma-range dst src rem =
    if eq-u32 rem 0 then ()
    else
      let _ = raw-store-u8 dst (raw-load-u8 src) in
      copy-dma-range (wrapping-add-i64 dst 1) (wrapping-add-i64 src 1) (wrapping-sub-u32 rem 1)

  -- Differential check of one generated RESP2/RESP3 value + split boundary
  check-one-val :: I64 -> I64 -> GenValCase ->{MVar} Bool
  check-one-val buf scratch gv =
    let k = gv.gv-kind-sel in
    let enc-len = (
      if k == 0 then
        let n = write-simple-list-to-dma gv.gv-payload scratch 0 in
        resp-write-simple-str buf 0 4096 scratch n
      else if k == 1 then
        let n = write-simple-list-to-dma gv.gv-payload scratch 0 in
        resp-write-error buf 0 4096 scratch n
      else if k == 2 then
        let iv = if gv.gv-int-sign then wrapping-neg-i64 (nat-to-i64-safe gv.gv-int-mag) else nat-to-i64-safe gv.gv-int-mag in
        resp-write-int buf 0 4096 iv
      else if k == 3 then
        let n = write-nat-list-to-dma gv.gv-payload scratch 0 in
        resp-write-bulk-raw buf 0 4096 scratch n
      else if k == 4 then resp-write-null-v2 buf 0 4096
      else if k == 5 then resp-write-null-array buf 0 4096
      else if k == 6 then resp-write-null-v3 buf 0 4096
      else if k == 7 then resp-write-bool buf 0 4096 gv.gv-bool
      else if k == 8 then
        let n = write-nat-list-to-dma gv.gv-payload scratch 0 in
        resp-write-bulk-error buf 0 4096 scratch n
      else if k == 9 then
        let _e0 = raw-store-u8 scratch 116 in
        let _e1 = raw-store-u8 (wrapping-add-i64 scratch 1) 120 in
        let _e2 = raw-store-u8 (wrapping-add-i64 scratch 2) 116 in
        let pay-addr = wrapping-add-i64 scratch 16 in
        let n = write-nat-list-to-dma gv.gv-payload pay-addr 0 in
        resp-write-verbatim buf 0 4096 scratch pay-addr n
      else if k == 10 then
        -- Map with 1 pair: +k -> bulk(payload)
        let p0 = resp-write-map-header buf 0 4096 1 in
        let p1 = resp-write-ok buf p0 4096 in
        let n = write-nat-list-to-dma gv.gv-payload scratch 0 in
        resp-write-bulk-raw buf p1 4096 scratch n
      else if k == 11 then
        -- Set with 2 elements: #t, :mag
        let p0 = resp-write-set-header buf 0 4096 2 in
        let p1 = resp-write-bool buf p0 4096 gv.gv-bool in
        resp-write-int buf p1 4096 (nat-to-i64-safe gv.gv-int-mag)
      else
        -- Push with 2 elements: +pubsub, bulk(payload)
        let p0 = resp-write-push-header buf 0 4096 2 in
        let p1 = resp-write-pong buf p0 4096 in
        let n = write-nat-list-to-dma gv.gv-payload scratch 0 in
        resp-write-bulk-raw buf p1 4096 scratch n) in
    if gt-u32 enc-len 4096 || eq-u32 enc-len 0 then False
    else
      let split-raw = div-u32 (wrapping-mul-u32 enc-len (nat-to-u32-safe gv.gv-split-pct)) 100 in
      let split = if eq-u32 split-raw 0 then 1 else if ge-u32 split-raw enc-len then wrapping-sub-u32 enc-len 1 else split-raw in
      let rv-part = resp-parse-value buf 0 split 6 in
      if ne-u8 rv-part.rv-status resp-need-more then False
      else
        let rv = resp-parse-value buf 0 enc-len 6 in
        if ne-u8 rv.rv-status resp-ok || ne-u32 rv.rv-consumed enc-len then False
        else if k == 0 then eq-u8 rv.rv-kind resp-kind-simple-string && eq-dma-simple-list buf rv.rv-data-off gv.gv-payload
        else if k == 1 then eq-u8 rv.rv-kind resp-kind-simple-error && eq-dma-simple-list buf rv.rv-data-off gv.gv-payload
        else if k == 2 then
          let iv = if gv.gv-int-sign then wrapping-neg-i64 (nat-to-i64-safe gv.gv-int-mag) else nat-to-i64-safe gv.gv-int-mag in
          eq-u8 rv.rv-kind resp-kind-integer && eq-i64 rv.rv-int iv
        else if k == 3 then eq-u8 rv.rv-kind resp-kind-bulk-string && eq-u32 rv.rv-data-len (list-length gv.gv-payload) && eq-dma-nat-list buf rv.rv-data-off gv.gv-payload
        else if k == 4 then eq-u8 rv.rv-kind resp-kind-null-bulk && eq-i64 rv.rv-int (-1)
        else if k == 5 then eq-u8 rv.rv-kind resp-kind-null-array && eq-i64 rv.rv-int (-1)
        else if k == 6 then eq-u8 rv.rv-kind resp-kind-null
        else if k == 7 then eq-u8 rv.rv-kind resp-kind-bool && eq-u8 rv.rv-bool (if gv.gv-bool then 1 else 0)
        else if k == 8 then eq-u8 rv.rv-kind resp-kind-bulk-error && eq-dma-nat-list buf rv.rv-data-off gv.gv-payload
        else if k == 9 then eq-u8 rv.rv-kind resp-kind-verbatim && eq-u32 rv.rv-data-len (list-length gv.gv-payload) && eq-dma-nat-list buf rv.rv-data-off gv.gv-payload
        else if k == 10 then eq-u8 rv.rv-kind resp-kind-map && eq-i64 rv.rv-int 1 && eq-u32 rv.rv-aux-off 2
        else if k == 11 then eq-u8 rv.rv-kind resp-kind-set && eq-i64 rv.rv-int 2
        else eq-u8 rv.rv-kind resp-kind-push && eq-i64 rv.rv-int 2

  -- Drive 1,000 live Hegel choice-stream trials for both commands and values!
  run-hegel-dma-trials :: I64 -> I64 -> I64 -> I64 -> HegelSeed -> U32 -> U32 ->{MVar} Bool
  run-hegel-dma-trials buf cmds args scratch seed trial total =
    if ge-u32 trial total then True
    else
      case gen-run-live seed 512 gen-cmd-case of
        None -> False
        Some (gc, _, seed1) ->
          if not (check-one-cmd buf cmds args scratch gc) then False
          else
            case gen-run-live seed1 512 gen-val-case of
              None -> False
              Some (gv, _, seed2) ->
                if not (check-one-val buf scratch gv) then False
                else run-hegel-dma-trials buf cmds args scratch seed2 (wrapping-add-u32 trial 1) total

  main :: Unit ->{IO, MVar} Int
  main _ =
    -- 1. Pure 1,000-trial `hegel-check-with` over enum/code bijections
    let cfg = HegelConfig 1000 128 40 (hegel-seed 6379) in
    let prop-codes = (\p -> case p of
      (s, k) ->
        let s8 = nat-to-u8-safe s in
        let k8 = nat-to-u8-safe k in
        eq-u8 (resp-status-to-u8 (resp-status-from-u8 s8)) s8
          && eq-u8 (resp-val-kind-to-u8 (resp-val-kind-from-u8 k8)) k8
          && eq-u8 (resp-cmd-kind-to-u8 (resp-cmd-kind-from-u8 k8)) k8) in
    case hegel-check-with cfg (gen-pair (gen-range 0 17) (gen-range 0 15)) prop-codes of
      Err _ -> 1
      Ok _ ->
        -- 2. 1,000-trial Hegel-driven differential DMA round-trip + split-boundary fuzzing
        let dma = engine-dma-alloc 16384 4096 in
        if eq-u64 dma 0 then 2
        else
          let buf = wrapping-u64-to-i64 dma in
          let cmds = wrapping-add-i64 buf 8192 in
          let args = wrapping-add-i64 buf 10240 in
          let scratch = wrapping-add-i64 buf 12288 in
          let ok = run-hegel-dma-trials buf cmds args scratch (hegel-seed 6379) 0 1000 in
          let _ = engine-dma-free dma 16384 in
          if ok then 0 else 3
"##;
    build_and_run_leak_checked("hegel-1000-diff", src, 0);
}

// =========================================================================
// 4. Steady-state 100,000 pipelined GET/SET parse + response-encode loop
//    asserting allocs == 0, atomic_rc == 0, and leaked == 0
// =========================================================================

#[test]
fn steady_state_100000_pipelined_get_set_zero_alloc_hot_loop() {
    let src = r##"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.String
  import Engine.Types
  import Prim.Atomic
  import Store.Kv.Resp

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

  build-pipe16 :: I64 -> U32 -> U32 -> U32 ->{MVar} U32
  build-pipe16 buf pos i total =
    if ge-u32 i total then pos
    else
      let wire = (if eq-u32 (rem-u32 i 2) 0 then
          "*3\r\n$3\r\nSET\r\n$8\r\nkey:0001\r\n$8\r\nval:0001\r\n"
        else
          "*2\r\n$3\r\nGET\r\n$8\r\nkey:0001\r\n") in
      let n = copy-str-to-dma wire (wrapping-add-i64 buf (u32-to-i64 pos)) in
      build-pipe16 buf (wrapping-add-u32 pos n) (wrapping-add-u32 i 1) total

  -- Coalesce responses for all `n` parsed commands in the batch directly into `tx`.
  #[no-alloc]
  encode-batch-responses :: I64 -> U32 -> U32 -> I64 -> U32 -> U32 -> I64 -> U32 ->{MVar} U32
  encode-batch-responses cmds i n tx tx-pos tx-cap val-addr val-len =
    if ge-u32 i n then tx-pos
    else
      let cv = resp-load-cmd cmds i in
      let next-pos = (
        if eq-u8 cv.rc-cmd cmd-set then
          resp-write-ok tx tx-pos tx-cap
        else if eq-u8 cv.rc-cmd cmd-get then
          resp-write-bulk-raw tx tx-pos tx-cap val-addr val-len
        else
          resp-write-null-v2 tx tx-pos tx-cap) in
      encode-batch-responses cmds (wrapping-add-u32 i 1) n tx next-pos tx-cap val-addr val-len

  -- 6,250 batches x 16 pipelined GET/SET commands = 100,000 parsed & encoded ops.
  -- 100% #[no-alloc]: MUST execute with 0 heap allocations and 0 atomic RC ops!
  #[no-alloc]
  hot-loop100000 :: I64 -> U32 -> I64 -> I64 -> I64 -> I64 -> U32 -> U32 -> U32 -> U32 ->{MVar} U32
  hot-loop100000 rx rx-len cmds args tx val-addr val-len batch limit acc-cmds =
    if ge-u32 batch limit then acc-cmds
    else
      let br = resp-extract-pipeline-batch rx 0 rx-len 0 cmds 16 args 64 16 in
      if ne-u8 br.rb-status resp-ok then 0
      else if ne-u32 br.rb-parsed-count 16 then 0
      else if ne-u32 br.rb-bytes-consumed rx-len then 0
      else
        let tx-len = encode-batch-responses cmds 0 16 tx 0 4096 val-addr val-len in
        if gt-u32 tx-len 4096 then 0
        else if eq-u32 tx-len 0 then 0
        else hot-loop100000 rx rx-len cmds args tx val-addr val-len (wrapping-add-u32 batch 1) limit (wrapping-add-u32 acc-cmds br.rb-parsed-count)

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 16384 4096 in
    if eq-u64 dma 0 then 99
    else
      let rx = wrapping-u64-to-i64 dma in
      let cmds = wrapping-add-i64 rx 4096 in
      let args = wrapping-add-i64 rx 6144 in
      let tx = wrapping-add-i64 rx 8192 in
      let val-addr = wrapping-add-i64 rx 12288 in
      let val-len = copy-str-to-dma "val:0001" val-addr in
      let rx-len = build-pipe16 rx 0 0 16 in

      -- Warmup 1 batch before resetting counters
      let _warm = hot-loop100000 rx rx-len cmds args tx val-addr val-len 0 1 0 in

      let _rst = engine-thread-counters-reset () in
      let total-cmds = hot-loop100000 rx rx-len cmds args tx val-addr val-len 0 6250 0 in
      let allocs = engine-thread-alloc-count () in
      let atomic-rcs = engine-thread-atomic-rc-count () in
      let _f = engine-dma-free dma 16384 in
      if ne-u32 total-cmds 100000 then 10
      else if ne-u64 allocs 0 then 11
      else if ne-u64 atomic-rcs 0 then 12
      else 0
"##;
    let dir = fresh_dir("hot-100k");
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let exe = dir.join("resp-hot-100k");
    let opts = DriverOptions::default();
    common::drive_with_profile_opts(
        Mode::Build,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        Some(exe.clone()),
        &TargetProfile::native(),
        &opts,
    )
    .expect("build hot-100k");

    let t0 = Instant::now();
    let out = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "128M")
        .output()
        .expect("run hot-100k");
    let elapsed = t0.elapsed();
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let _ = std::fs::remove_dir_all(&dir);

    assert_eq!(
        out.status.code(),
        Some(0),
        "hot-100k failed: exit {:?}\nstdout:\n{stdout}\nstderr:\n{stderr}",
        out.status.code()
    );
    assert!(
        stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
        "hot-100k expected clean leak check (`leaked=0`), got stderr:\n{stderr}"
    );
    let ops_per_sec = 100_000.0 / elapsed.as_secs_f64();
    eprintln!(
        "[Store.Kv.Resp 100k pipelined GET/SET] elapsed={:.2}ms (~{:.2}M cmds/s end-to-end incl. process launch) allocs=0 atomic_rc=0 leaked=0",
        elapsed.as_secs_f64() * 1000.0,
        ops_per_sec / 1_000_000.0
    );
}
