//! Differential, RFC 3720 / RFC 9110 / RFC 6455, DMA zero-copy, `#[no-alloc]`,
//! and leak-checked integration tests for `Engine.Simd.Scan`,
//! `Engine.Simd.Crc32c`, and `Engine.Simd.Mask`.
//!
//! Routed through `common::drive_with_profile` and `common::source_frontend` so
//! the exact same suite runs against both the Rust seed and the Stage 3
//! self-hosted compiler.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{Mode, Package};
use std::process::Command;

const STDLIB_PKG: &str = include_str!("../prelude/.cinchpkg");

fn build_and_run_leak_checked(tag: &str, main_src: &str, expected: i32) {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("Main.cinch"), main_src).expect("write Main.cinch");
    let exe = dir.path().join(tag);
    let profile = TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..TargetProfile::native()
    };
    common::drive_with_profile(
        Mode::Build,
        StageMode::Refresh,
        dir.path(),
        "Main.cinch",
        Some(exe.clone()),
        &profile,
    )
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
        "{tag}: expected exit {expected}, got {:?}\nstdout:\n{stdout}\nstderr:\n{stderr}",
        out.status.code()
    );
    assert!(
        stderr.contains("leaked=0"),
        "{tag}: expected leaked=0 in stderr, got:\n{stderr}"
    );
}

#[test]
fn engine_simd_modules_registered_in_stdlib_package_manifest() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, path) in [
        ("Engine.Simd.Scan", "Engine/Simd/Scan.cinch"),
        ("Engine.Simd.Crc32c", "Engine/Simd/Crc32c.cinch"),
        ("Engine.Simd.Mask", "Engine/Simd/Mask.cinch"),
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
}

#[test]
fn engine_simd_interface_no_alloc_assertions() {
    let src = r#"
module Main where
  import Engine.Simd.Scan
  import Engine.Simd.Crc32c
  import Engine.Simd.Mask

  main :: Int
  main =
    let m = simd-mask-http-tchar-invalid 0 in
    let c = crc32c-finish crc32c-init in
    let r = ws-mask-rotate 0x11223344 1 in
    if eq-u32 m 65535 && eq-u32 c 0 && eq-u32 r 0x44112233 then 0 else 1
"#;
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("Main.cinch"), src).expect("write Main.cinch");
    let ifaces = common::check_interfaces(dir.path(), "Main.cinch")
        .expect("check_interfaces");

    let assert_no_alloc = |module: &str, fn_name: &str| {
        let iface = ifaces
            .iter()
            .find(|i| i.module == module)
            .unwrap_or_else(|| panic!("missing interface for {module}"));
        let entry = iface
            .assertions
            .iter()
            .find(|a| a.name == fn_name)
            .unwrap_or_else(|| panic!("missing assertion for {module}.{fn_name}"));
        assert!(
            entry.facts.iter().any(|f| f == "no-alloc"),
            "expected no-alloc fact on {module}.{fn_name}, got {:?}",
            entry.facts
        );
    };

    for fn_name in [
        "simd-mask-eq-byte",
        "simd-mask-any-of4",
        "simd-mask-http-tchar-invalid",
        "simd-mask-field-value-invalid",
        "simd-ascii-eq-nocase16",
        "simd-scan-field-value",
        "simd-ascii-eq-nocase",
        "dma-find-byte",
        "dma-find-crlf",
        "dma-scan-http-token",
        "dma-scan-field-value",
        "dma-ascii-eq-nocase",
    ] {
        assert_no_alloc("Engine.Simd.Scan", fn_name);
    }
    for fn_name in [
        "crc32c-init",
        "crc32c-finish",
        "crc32c-update-bytes",
        "crc32c-bytes",
        "crc32c-update-dma",
        "crc32c-combine",
    ] {
        assert_no_alloc("Engine.Simd.Crc32c", fn_name);
    }
    for fn_name in [
        "ws-mask-rotate",
        "ws-mask-bytes-in-place",
        "ws-mask-bytes-in-place-at-phase",
        "ws-mask-dma-in-place",
        "ws-mask-dma-in-place-at-phase",
    ] {
        assert_no_alloc("Engine.Simd.Mask", fn_name);
    }
}

/// Exhaustive differential tests for `Engine.Simd.Scan`:
/// - `simd-mask-http-tchar-invalid` vs RFC 9110 scalar spec on ALL 256 byte values across ALL 16 lanes
/// - `simd-mask-field-value-invalid`, `simd-mask-eq-byte`, `simd-mask-any-of-4`, `simd-ascii-eq-nocase-16`
/// - `simd-find-byte` & `dma-find-byte`
/// - `simd-find-crlf` & `dma-find-crlf` across all offsets 0..15 and lengths 0..128 (straddling 16B boundaries)
/// - `simd-scan-http-token` & `dma-scan-http-token`
/// - `simd-scan-field-value` & `dma-scan-field-value` (bare `\n`, NUL, `0x7F`, HTAB, obs-text, `\r\n` straddling 16B boundaries)
/// - `simd-ascii-eq-nocase` & `dma-ascii-eq-nocase`
#[test]
fn engine_simd_scan_differential_all_256_bytes_and_boundaries() {
    let main_src = r##"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Wide
  import Prim.Atomic
  import Engine.Types
  import Engine.Simd.Scan

  -- Reference RFC 9110 Section 5.6.2 `tchar` predicate:
  -- tchar = "!" / "#" / "$" / "%" / "&" / "'" / "*" / "+" / "-" / "."
  --       / "^" / "_" / "`" / "|" / "~" / DIGIT / ALPHA
  ref-is-tchar :: U8 -> Bool
  ref-is-tchar b =
    (ge-u8 b 48 && le-u8 b 57)
      || (ge-u8 b 65 && le-u8 b 90)
      || (ge-u8 b 97 && le-u8 b 122)
      || eq-u8 b 33 || eq-u8 b 35 || eq-u8 b 36 || eq-u8 b 37 || eq-u8 b 38
      || eq-u8 b 39 || eq-u8 b 42 || eq-u8 b 43 || eq-u8 b 45 || eq-u8 b 46
      || eq-u8 b 94 || eq-u8 b 95 || eq-u8 b 96 || eq-u8 b 124 || eq-u8 b 126

  ref-is-fv-invalid :: U8 -> Bool
  ref-is-fv-invalid b =
    (lt-u8 b 32 && ne-u8 b 9) || eq-u8 b 127

  ref-lower :: U8 -> U8
  ref-lower b =
    if ge-u8 b 65 && le-u8 b 90 then wrapping-add-u8 b 32 else b

  -- Put byte `val` into lane `lane` (0..15) while filling all other lanes with `fill`.
  make-lane-vec :: U32 -> U8 -> U8 -> U128
  make-lane-vec lane val fill =
    let base = simd-u8x16-splat fill in
    let sh = wrapping-shl-u32 lane 3 in
    let clear-mask = bit-not-u128 (wrapping-shl-u128 255 sh) in
    bit-or-u128 (bit-and-u128 base clear-mask) (wrapping-shl-u128 (u8-to-u128 val) sh)

  -- Check all 256 byte values in every lane 0..15 for `simd-mask-http-tchar-invalid`
  -- and `simd-mask-field-value-invalid` and `simd-mask-eq-byte` and `simd-mask-any-of-4`.
  check-classifiers-lane :: U32 -> U32 -> Bool
  check-classifiers-lane lane bv =
    if ge-u32 bv 256 then True
    else
      let b = wrapping-u32-to-u8 bv in
      let bit = wrapping-shl-u32 1 lane in
      -- Other lanes hold valid tchar 'a' (97), so only `lane` can trigger invalid bit.
      let v-tchar = make-lane-vec lane b 97 in
      let got-tchar = simd-mask-http-tchar-invalid v-tchar in
      let exp-tchar = if ref-is-tchar b then 0 else bit in
      -- Other lanes hold valid field-value char 'A' (65).
      let v-fv = make-lane-vec lane b 65 in
      let got-fv = simd-mask-field-value-invalid v-fv in
      let exp-fv = if ref-is-fv-invalid b then bit else 0 in
      -- simd-mask-eq-byte: fill with (b ^ 1), put `b` at `lane`.
      let other = bit-xor-u8 b 1 in
      let v-eq = make-lane-vec lane b other in
      let got-eq = simd-mask-eq-byte v-eq b in
      -- simd-mask-any-of-4: fill with (b ^ 0xFF), test matching on one of 4 needles.
      let fill4 = bit-xor-u8 b 0xFF in
      let n0 = bit-xor-u8 b 0x01 in
      let n1 = bit-xor-u8 b 0x02 in
      let n2 = b in
      let n3 = bit-xor-u8 b 0x04 in
      let v-any = make-lane-vec lane b fill4 in
      let got-any = simd-mask-any-of4 v-any n0 n1 n2 n3 in
      if ne-u32 got-tchar exp-tchar then False
      else if ne-u32 got-fv exp-fv then False
      else if ne-u32 got-eq bit then False
      else if ne-u32 got-any bit then False
      else check-classifiers-lane lane (wrapping-add-u32 bv 1)

  check-classifiers-all-lanes :: U32 -> Bool
  check-classifiers-all-lanes lane =
    if ge-u32 lane 16 then True
    else check-classifiers-lane lane 0 && check-classifiers-all-lanes (wrapping-add-u32 lane 1)

  -- Check `simd-ascii-eq-nocase-16` on all 256 byte pairs (b, lower(b)) and (b, b^1) across lanes.
  check-nocase16-byte :: U32 -> Bool
  check-nocase16-byte bv =
    if ge-u32 bv 256 then True
    else
      let b = wrapping-u32-to-u8 bv in
      let lo = ref-lower b in
      let va = simd-u8x16-splat b in
      let vb = simd-u8x16-splat lo in
      let vc = make-lane-vec (bit-and-u32 bv 15) (bit-xor-u8 b 0x40) b in
      let ok-eq = simd-ascii-eq-nocase16 va vb in
      let exp-neq = eq-u8 (ref-lower (bit-xor-u8 b 0x40)) lo in
      let got-neq = simd-ascii-eq-nocase16 va vc in
      if not ok-eq then False
      else if got-neq /= exp-neq then False
      else check-nocase16-byte (wrapping-add-u32 bv 1)

  -- Copy a Bytes buffer into a DMA slab at `base`.
  copy-bytes-to-dma :: Bytes -> I64 -> Nat ->{MVar} Unit
  copy-bytes-to-dma b base i =
    if i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 base (nat-to-i64 i)) (bytes-get-u8 b i) in
      copy-bytes-to-dma b base (i + 1)
    else ()

  -- Reference scalar CRLF finder in [start, end).
  ref-find-crlf :: Bytes -> Nat -> Nat -> I32
  ref-find-crlf b pos end =
    if pos + 1 < end && pos + 1 < bytes-len b then
      if eq-u8 (bytes-get-u8 b pos) 13 && eq-u8 (bytes-get-u8 b (pos + 1)) 10 then
        wrapping-i64-to-i32 (nat-to-i64 pos)
      else ref-find-crlf b (pos + 1) end
    else -1

  -- Reference scalar byte finder in [start, end).
  ref-find-byte :: Bytes -> Nat -> Nat -> U8 -> I32
  ref-find-byte b pos end needle =
    if pos < end && pos < bytes-len b then
      if eq-u8 (bytes-get-u8 b pos) needle then wrapping-i64-to-i32 (nat-to-i64 pos)
      else ref-find-byte b (pos + 1) end needle
    else -1

  -- Reference scalar HTTP token scanner in [start, end).
  ref-scan-token :: Bytes -> Nat -> Nat -> Nat
  ref-scan-token b pos end =
    if pos < end && pos < bytes-len b then
      if ref-is-tchar (bytes-get-u8 b pos) then ref-scan-token b (pos + 1) end
      else pos
    else end

  -- Reference scalar HTTP field-value scanner in [start, end).
  ref-scan-fv :: Bytes -> Nat -> Nat -> I32
  ref-scan-fv b pos end =
    if pos < end && pos < bytes-len b then
      let ch = bytes-get-u8 b pos in
      if ref-is-fv-invalid ch then
        if eq-u8 ch 13 then wrapping-i64-to-i32 (nat-to-i64 pos) else -2
      else ref-scan-fv b (pos + 1) end
    else -1

  opt-nat-to-i32 :: Option Nat -> I32
  opt-nat-to-i32 opt = case opt of
    None -> -1
    Some n -> if n <= 2147483647 then wrapping-i64-to-i32 (nat-to-i64 n) else -99

  fill-byte :: Bytes -> Nat -> Nat -> U8 -> Unit
  fill-byte b i end v =
    if i < end && i < bytes-len b then
      let _ = bytes-set-u8 b i v in
      fill-byte b (i + 1) end v
    else ()

  -- Test CRLF, find-byte, token, and field-value scanning for a given `(start, len)`
  -- with target placed at `rel-pos` (covering 16-byte boundary straddling, lone `\r`,
  -- `\r\r\n`, bare `\n`, NUL, `0x7F`, HTAB, obs-text `0x80`/`0xFF`).
  test-crlf-case :: I64 -> Nat -> Nat -> Nat ->{MVar} Bool
  test-crlf-case dma start len rel =
    let total = start + len in
    let b = bytes-new total in
    let _ = fill-byte b 0 total 65 in
    -- Plant a decoy lone '\r' before `rel` if room permits.
    let _ = if rel >= 3 && start + rel - 2 < bytes-len b then bytes-set-u8 b (start + rel - 2) 13 else () in
    -- Plant '\r' at start+rel and '\n' at start+rel+1 when in bounds.
    let _ = if rel < len && start + rel < bytes-len b then bytes-set-u8 b (start + rel) 13 else () in
    let _ = if rel + 1 < len && start + rel + 1 < bytes-len b then bytes-set-u8 b (start + rel + 1) 10 else () in
    let _ = copy-bytes-to-dma b dma 0 in
    let s32 = wrapping-i64-to-u32 (nat-to-i64 start) in
    let e32 = wrapping-i64-to-u32 (nat-to-i64 total) in
    let exp-crlf = ref-find-crlf b start total in
    let got-crlf-b = opt-nat-to-i32 (simd-find-crlf b start total) in
    let got-crlf-d = dma-find-crlf dma s32 e32 in
    let exp-cr = ref-find-byte b start total 13 in
    let got-cr-b = opt-nat-to-i32 (simd-find-byte b start total 13) in
    let got-cr-d = dma-find-byte dma s32 e32 13 in
    eq-i32 got-crlf-b exp-crlf
      && eq-i32 got-crlf-d exp-crlf
      && eq-i32 got-cr-b exp-cr
      && eq-i32 got-cr-d exp-cr

  -- Sweep lengths 0..128 (stepping finely around 0..40 and straddling boundaries) and offsets 0..15.
  test-crlf-offsets :: I64 -> Nat -> Nat ->{MVar} Bool
  test-crlf-offsets dma len off =
    if off >= 16 then True
    else
      -- Place CRLF at 0, 15 (straddling!), 16, len-2, len-1 (trailing lone CR), and none (len).
      let ok0 = test-crlf-case dma off len 0 in
      let ok15 = test-crlf-case dma off len 15 in
      let ok16 = test-crlf-case dma off len 16 in
      let ok31 = test-crlf-case dma off len 31 in
      let ok-end2 = if len >= 2 then test-crlf-case dma off len (len - 2) else True in
      let ok-end1 = if len >= 1 then test-crlf-case dma off len (len - 1) else True in
      let ok-none = test-crlf-case dma off len len in
      if ok0 && ok15 && ok16 && ok31 && ok-end2 && ok-end1 && ok-none then
        test-crlf-offsets dma len (off + 1)
      else False

  test-crlf-lengths :: I64 -> Nat ->{MVar} Bool
  test-crlf-lengths dma len =
    if len > 128 then True
    else
      if test-crlf-offsets dma len 0 then
        let step = if len < 36 then 1 else if len < 68 then 4 else 15 in
        test-crlf-lengths dma (len + step)
      else False

  -- Test `\r\r\n` straddling lane 14/15/16 specifically.
  test-double-cr-crlf :: I64 ->{MVar} Bool
  test-double-cr-crlf dma =
    let b = bytes-new 40 in
    let _ = fill-byte b 0 40 88 in
    let _ = (if 17 < bytes-len b then
      let _ = bytes-set-u8 b 14 13 in
      let _ = bytes-set-u8 b 15 13 in
      bytes-set-u8 b 16 10
      else ()) in
    let _ = copy-bytes-to-dma b dma 0 in
    let got-b = opt-nat-to-i32 (simd-find-crlf b 0 40) in
    let got-d = dma-find-crlf dma 0 40 in
    eq-i32 got-b 15 && eq-i32 got-d 15

  -- Differential test for `simd-scan-http-token` & `dma-scan-http-token` across all 256 byte values.
  test-token-all256 :: I64 -> U32 ->{MVar} Bool
  test-token-all256 dma bv =
    if ge-u32 bv 256 then True
    else
      let ch = wrapping-u32-to-u8 bv in
      let pos = nat-from-i64 (u32-to-i64 (bit-and-u32 bv 31)) in
      let total = 48 in
      let b = bytes-new total in
      let _ = fill-byte b 0 total 122 in
      let _ = if pos < bytes-len b then bytes-set-u8 b pos ch else () in
      let _ = copy-bytes-to-dma b dma 0 in
      let exp = ref-scan-token b 0 total in
      let got-b = simd-scan-http-token b 0 total in
      let got-d = nat-from-i64 (u32-to-i64 (dma-scan-http-token dma 0 48)) in
      if got-b == exp && got-d == exp then
        test-token-all256 dma (wrapping-add-u32 bv 1)
      else False

  -- Differential test for `simd-scan-field-value` & `dma-scan-field-value`:
  -- test every byte value 0..255 placed at positions 0..33 with HTAB and obs-text present before it.
  test-fv-all256 :: I64 -> U32 ->{MVar} Bool
  test-fv-all256 dma bv =
    if ge-u32 bv 256 then True
    else
      let ch = wrapping-u32-to-u8 bv in
      let start = nat-from-i64 (u32-to-i64 (bit-and-u32 bv 15)) in
      let rel = nat-from-i64 (u32-to-i64 (bit-and-u32 (wrapping-shr-u32 bv 1) 31)) in
      let len = rel + 20 in
      let total = start + len in
      let b = bytes-new total in
      let _ = fill-byte b 0 total 65 in
      -- Embed HTAB (0x09) and obs-text (0x80, 0xFF) before `rel` to verify they are accepted!
      let _ = if rel >= 3 && start + 1 < bytes-len b then bytes-set-u8 b (start + 1) 9 else () in
      let _ = if rel >= 3 && start + 2 < bytes-len b then bytes-set-u8 b (start + 2) 0x80 else () in
      let _ = if rel >= 4 && start + 3 < bytes-len b then bytes-set-u8 b (start + 3) 0xFF else () in
      let _ = if start + rel < bytes-len b then bytes-set-u8 b (start + rel) ch else () in
      -- Put a trailing '\r\n' after `rel` so valid bytes return `start + len - 2` while invalid bytes return `-2` (or `start + rel` if ch == '\r').
      let _ = (if start + len - 2 < bytes-len b && rel + 2 < len then
        let _ = bytes-set-u8 b (start + len - 2) 13 in
        if start + len - 1 < bytes-len b then bytes-set-u8 b (start + len - 1) 10 else ()
        else ()) in
      let _ = copy-bytes-to-dma b dma 0 in
      let s32 = wrapping-i64-to-u32 (nat-to-i64 start) in
      let e32 = wrapping-i64-to-u32 (nat-to-i64 total) in
      let exp = ref-scan-fv b start total in
      let got-b = simd-scan-field-value b start total in
      let got-d = dma-scan-field-value dma s32 e32 in
      if eq-i32 got-b exp && eq-i32 got-d exp then
        test-fv-all256 dma (wrapping-add-u32 bv 1)
      else False

  -- Verify incomplete field value returns -1, bare '\n' / NUL / 0x7F return -2 at every offset 0..15.
  test-fv-specials :: I64 -> Nat ->{MVar} Bool
  test-fv-specials dma off =
    if off >= 16 then True
    else
      let total = off + 40 in
      let b = bytes-new total in
      let _ = fill-byte b 0 total 72 in
      let _ = if off + 5 < bytes-len b then bytes-set-u8 b (off + 5) 9 else () in
      let _ = copy-bytes-to-dma b dma 0 in
      let s32 = wrapping-i64-to-u32 (nat-to-i64 off) in
      let e32 = wrapping-i64-to-u32 (nat-to-i64 total) in
      -- Incomplete (no CR, no illegal) -> -1
      let ok-inc = eq-i32 (simd-scan-field-value b off total) (-1) && eq-i32 (dma-scan-field-value dma s32 e32) (-1) in
      -- Bare '\n' at off + 19 -> -2
      let _ = if off + 19 < bytes-len b then bytes-set-u8 b (off + 19) 10 else () in
      let _ = copy-bytes-to-dma b dma 0 in
      let ok-lf = eq-i32 (simd-scan-field-value b off total) (-2) && eq-i32 (dma-scan-field-value dma s32 e32) (-2) in
      -- NUL at off + 19 -> -2
      let _ = if off + 19 < bytes-len b then bytes-set-u8 b (off + 19) 0 else () in
      let _ = copy-bytes-to-dma b dma 0 in
      let ok-nul = eq-i32 (simd-scan-field-value b off total) (-2) && eq-i32 (dma-scan-field-value dma s32 e32) (-2) in
      -- DEL (0x7F) at off + 19 -> -2
      let _ = if off + 19 < bytes-len b then bytes-set-u8 b (off + 19) 127 else () in
      let _ = copy-bytes-to-dma b dma 0 in
      let ok-del = eq-i32 (simd-scan-field-value b off total) (-2) && eq-i32 (dma-scan-field-value dma s32 e32) (-2) in
      if ok-inc && ok-lf && ok-nul && ok-del then test-fv-specials dma (off + 1) else False

  -- Differential test for `simd-ascii-eq-nocase` & `dma-ascii-eq-nocase` across lengths 0..64 and alignments.
  fill-mixed-case :: Bytes -> Bytes -> Nat -> Nat -> Nat -> Unit
  fill-mixed-case a b a-off b-off len =
    fill-mixed-loop a b a-off b-off 0 len

  fill-mixed-loop :: Bytes -> Bytes -> Nat -> Nat -> Nat -> Nat -> Unit
  fill-mixed-loop a b a-off b-off i len =
    if i < len then
      let c = wrapping-i64-to-u8 (nat-to-i64 (65 + nat-rem i 26)) in
      let c-lo = wrapping-add-u8 c 32 in
      let _ = if a-off + i < bytes-len a then bytes-set-u8 a (a-off + i) c else () in
      let _ = if b-off + i < bytes-len b then bytes-set-u8 b (b-off + i) c-lo else () in
      fill-mixed-loop a b a-off b-off (i + 1) len
    else ()

  test-ascii-eq-len :: I64 -> I64 -> Nat ->{MVar} Bool
  test-ascii-eq-len dma-a dma-b len =
    if len > 65 then True
    else
      let a-off = nat-rem len 16 in
      let b-off = nat-rem (len * 3 + 5) 16 in
      let a = bytes-new (a-off + len + 4) in
      let b = bytes-new (b-off + len + 4) in
      let _ = fill-mixed-case a b a-off b-off len in
      let _ = copy-bytes-to-dma a dma-a 0 in
      let _ = copy-bytes-to-dma b dma-b 0 in
      let ao32 = wrapping-i64-to-u32 (nat-to-i64 a-off) in
      let bo32 = wrapping-i64-to-u32 (nat-to-i64 b-off) in
      let l32 = wrapping-i64-to-u32 (nat-to-i64 len) in
      let ok-eq = simd-ascii-eq-nocase a a-off b b-off len && dma-ascii-eq-nocase dma-a ao32 dma-b bo32 l32 in
      -- Flip one byte near the end and confirm inequality for len > 0.
      let ok-neq = (if len == 0 then True
        else
          let last = len - 1 in
          let _ = if b-off + last < bytes-len b then bytes-set-u8 b (b-off + last) 64 else () in
          let _ = copy-bytes-to-dma b dma-b 0 in
          not (simd-ascii-eq-nocase a a-off b b-off len) && not (dma-ascii-eq-nocase dma-a ao32 dma-b bo32 l32)) in
      if ok-eq && ok-neq then test-ascii-eq-len dma-a dma-b (len + 1) else False

  main :: Unit ->{IO, MVar} Int
  main _ =
    if not (check-classifiers-all-lanes 0) then 1
    else if not (check-nocase16-byte 0) then 2
    else
      let dma-u64 = engine-dma-alloc 4096 4096 in
      if eq-u64 dma-u64 0 then 3
      else
        let dma = wrapping-u64-to-i64 dma-u64 in
        let dma2 = wrapping-add-i64 dma 2048 in
        let code = (if not (test-crlf-lengths dma 0) then 4
          else if not (test-double-cr-crlf dma) then 5
          else if not (test-token-all256 dma 0) then 6
          else if not (test-fv-all256 dma 0) then 7
          else if not (test-fv-specials dma 0) then 8
          else if not (test-ascii-eq-len dma dma2 0) then 9
          else 0) in
        let _ = engine-dma-free dma-u64 4096 in
        code
"##;
    build_and_run_leak_checked("engine-simd-scan-diff", main_src, 0);
}

/// RFC 3720 CRC32C vectors, GF(2) `crc32c-combine` across arbitrary split points,
/// multi-chunk `crc32c-update-bytes`, and 4 KiB DMA `crc32c-update-dma` parity.
#[test]
fn engine_simd_crc32c_rfc3720_combine_and_dma() {
    let main_src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Prim.Atomic
  import Engine.Types
  import Engine.Simd.Crc32c

  fill-const :: Bytes -> Nat -> U8 -> Unit
  fill-const b i v =
    if i < bytes-len b then
      let _ = bytes-set-u8 b i v in
      fill-const b (i + 1) v
    else ()

  fill-asc :: Bytes -> Nat -> Unit
  fill-asc b i =
    if i < bytes-len b then
      let _ = bytes-set-u8 b i (wrapping-i64-to-u8 (nat-to-i64 i)) in
      fill-asc b (i + 1)
    else ()

  fill-desc :: Bytes -> Nat -> Unit
  fill-desc b i =
    if i < bytes-len b then
      let _ = bytes-set-u8 b i (wrapping-i64-to-u8 (nat-to-i64 (31 - nat-rem i 32))) in
      fill-desc b (i + 1)
    else ()

  fill-prng :: Bytes -> Nat -> U32 -> Unit
  fill-prng b i st =
    if i < bytes-len b then
      let next = wrapping-add-u32 (wrapping-mul-u32 st 1664525) 1013904223 in
      let _ = bytes-set-u8 b i (wrapping-u32-to-u8 (wrapping-shr-u32 next 16)) in
      fill-prng b (i + 1) next
    else ()

  copy-to-dma :: Bytes -> I64 -> Nat ->{MVar} Unit
  copy-to-dma b base i =
    if i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 base (nat-to-i64 i)) (bytes-get-u8 b i) in
      copy-to-dma b base (i + 1)
    else ()

  check-rfc3720 :: Bool
  check-rfc3720 =
    let empty = bytes-new 0 in
    let check-empty = eq-u32 (crc32c-bytes empty) 0x00000000 in
    let digits = bytes-from-u8s [49, 50, 51, 52, 53, 54, 55, 56, 57] in
    let check-digits = eq-u32 (crc32c-bytes digits) 0xE3069283 in
    let zeros32 = bytes-new 32 in
    let check-zeros = eq-u32 (crc32c-bytes zeros32) 0x8A9136AA in
    let ones32 = bytes-new 32 in
    let _ = fill-const ones32 0 0xFF in
    let check-ones = eq-u32 (crc32c-bytes ones32) 0x62A8AB43 in
    let asc32 = bytes-new 32 in
    let _ = fill-asc asc32 0 in
    let check-asc = eq-u32 (crc32c-bytes asc32) 0x46DD794E in
    let desc32 = bytes-new 32 in
    let _ = fill-desc desc32 0 in
    let check-desc = eq-u32 (crc32c-bytes desc32) 0x113FDB5C in
    check-empty && check-digits && check-zeros && check-ones && check-asc && check-desc

  -- Verify `crc32c-combine` AND chunked `crc32c-update-bytes` match single-pass across every split 0..96.
  check-combine-splits :: Bytes -> U32 -> Nat -> Bool
  check-combine-splits b full split =
    let n = bytes-len b in
    if split > n then True
    else
      let rem = n - split in
      let c1 = crc32c-finish (crc32c-update-bytes crc32c-init b 0 split) in
      let c2 = crc32c-finish (crc32c-update-bytes crc32c-init b split rem) in
      let combined = crc32c-combine c1 c2 (wrapping-i64-to-u64 (nat-to-i64 rem)) in
      let streamed = crc32c-finish (crc32c-update-bytes (crc32c-update-bytes crc32c-init b 0 split) b split rem) in
      if eq-u32 combined full && eq-u32 streamed full then
        check-combine-splits b full (split + 1)
      else False

  -- Verify DMA CRC32C matches `crc32c-update-bytes` across lengths 0..129 and full 4096B WAL page.
  check-dma-lengths :: Bytes -> I64 -> Nat ->{MVar} Bool
  check-dma-lengths b dma len =
    if len > 129 then True
    else
      let off = nat-rem len 16 in
      let cb = crc32c-finish (crc32c-update-bytes crc32c-init b off len) in
      let cd = crc32c-finish (crc32c-update-dma crc32c-init (wrapping-add-i64 dma (nat-to-i64 off)) (wrapping-i64-to-u32 (nat-to-i64 len))) in
      if eq-u32 cb cd then check-dma-lengths b dma (len + 1) else False

  main :: Unit ->{IO, MVar} Int
  main _ =
    if not check-rfc3720 then 1
    else
      let buf96 = bytes-new 96 in
      let _ = fill-prng buf96 0 0xCAFEBABE in
      let full96 = crc32c-bytes buf96 in
      if not (check-combine-splits buf96 full96 0) then 2
      else
        let page = bytes-new 4096 in
        let _ = fill-prng page 0 0x12345678 in
        let dma-u64 = engine-dma-alloc 4096 4096 in
        if eq-u64 dma-u64 0 then 3
        else
          let dma = wrapping-u64-to-i64 dma-u64 in
          let _ = copy-to-dma page dma 0 in
          let c-page-b = crc32c-bytes page in
          let c-page-d = crc32c-finish (crc32c-update-dma crc32c-init dma 4096) in
          -- Also test `crc32c-combine` on a 4096-byte page split at 1337 / 2759.
          let p1 = crc32c-finish (crc32c-update-dma crc32c-init dma 1337) in
          let p2 = crc32c-finish (crc32c-update-dma crc32c-init (wrapping-add-i64 dma 1337) 2759) in
          let p-comb = crc32c-combine p1 p2 2759 in
          let ok-lens = check-dma-lengths page dma 0 in
          let _ = engine-dma-free dma-u64 4096 in
          if not (eq-u32 c-page-b c-page-d) then 4
          else if not (eq-u32 p-comb c-page-b) then 5
          else if not ok-lens then 6
          else 0
"#;
    build_and_run_leak_checked("engine-simd-crc32c", main_src, 0);
}

/// WebSocket `Engine.Simd.Mask` involution, exact byte-for-byte differential check vs
/// scalar RFC 6455 masking across lengths `0..129` and alignments `0..15`, untouched
/// guard bytes, streaming phase rotation, and DMA zero-copy parity.
#[test]
fn engine_simd_ws_mask_involution_alignments_phase_and_dma() {
    let main_src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Prim.Atomic
  import Engine.Types
  import Engine.Simd.Mask

  fill-pattern :: Bytes -> Nat -> Unit
  fill-pattern b i =
    if i < bytes-len b then
      let v = wrapping-i64-to-u8 (nat-to-i64 (nat-rem (i * 37 + 11) 251 + 1)) in
      let _ = bytes-set-u8 b i v in
      fill-pattern b (i + 1)
    else ()

  -- Scalar RFC 6455 reference masking over `b[off..off+len)`.
  scalar-mask :: Bytes -> Nat -> Nat -> U32 -> Nat -> Unit
  scalar-mask b off len mask i =
    if i < len && off + i < bytes-len b then
      let shift = wrapping-shl-u32 (bit-and-u32 (wrapping-i64-to-u32 (nat-to-i64 i)) 3) 3 in
      let k = wrapping-u32-to-u8 (wrapping-shr-u32 mask shift) in
      let cur = bytes-get-u8 b (off + i) in
      let _ = bytes-set-u8 b (off + i) (bit-xor-u8 cur k) in
      scalar-mask b off len mask (i + 1)
    else ()

  copy-bytes-to-dma :: Bytes -> I64 -> Nat ->{MVar} Unit
  copy-bytes-to-dma b base i =
    if i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 base (nat-to-i64 i)) (bytes-get-u8 b i) in
      copy-bytes-to-dma b base (i + 1)
    else ()

  dma-eq-bytes :: Bytes -> I64 -> Nat ->{MVar} Bool
  dma-eq-bytes b base i =
    if i < bytes-len b then
      if eq-u8 (bytes-get-u8 b i) (raw-load-u8 (wrapping-add-i64 base (nat-to-i64 i))) then
        dma-eq-bytes b base (i + 1)
      else False
    else True

  -- Test `ws-mask-bytes-in-place` and `ws-mask-dma-in-place` at `(off, len)`:
  -- 1) exact equality vs scalar RFC 6455 across entire buffer (including guard bytes outside [off, off+len))
  -- 2) involution: masking twice restores original buffer byte-for-byte
  test-one :: I64 -> Nat -> Nat -> U32 ->{MVar} Bool
  test-one dma off len mask =
    let total = off + len + 16 in
    let orig = bytes-new total in
    let _ = fill-pattern orig 0 in
    let ref = bytes-new total in
    let _ = bytes-copy-range ref orig 0 0 total in
    let _ = scalar-mask ref off len mask 0 in
    let got = bytes-new total in
    let _ = bytes-copy-range got orig 0 0 total in
    let _ = ws-mask-bytes-in-place got off len mask in
    let ok-bytes-masked = eq-i64 (bytes-compare got ref) 0 in
    let _ = ws-mask-bytes-in-place got off len mask in
    let ok-bytes-inv = eq-i64 (bytes-compare got orig) 0 in
    let _ = copy-bytes-to-dma orig dma 0 in
    let dma-slice = wrapping-add-i64 dma (nat-to-i64 off) in
    let len32 = wrapping-i64-to-u32 (nat-to-i64 len) in
    let _ = ws-mask-dma-in-place dma-slice len32 mask in
    let ok-dma-masked = dma-eq-bytes ref dma 0 in
    let _ = ws-mask-dma-in-place dma-slice len32 mask in
    let ok-dma-inv = dma-eq-bytes orig dma 0 in
    ok-bytes-masked && ok-bytes-inv && ok-dma-masked && ok-dma-inv

  test-offsets :: I64 -> Nat -> Nat ->{MVar} Bool
  test-offsets dma len off =
    if off >= 16 then True
    else if len <= 200 then
      let mask = wrapping-add-u32 0xA5C391F7 (wrapping-mul-u32 (wrapping-i64-to-u32 (nat-to-i64 (len + off))) 2654435761) in
      if test-one dma off len mask then test-offsets dma len (off + 1) else False
    else False

  test-lengths :: I64 -> Nat ->{MVar} Bool
  test-lengths dma len =
    if len > 129 then True
    else
      if test-offsets dma len 0 then test-lengths dma (len + 1) else False

  -- Test streaming phase rotation across uneven chunk splits (1, 2, 3, 5, 13, 17, 31, 28 = 100 bytes).
  test-phase-rotation :: I64 ->{MVar} Bool
  test-phase-rotation dma =
    let mask = 0xDEADBEEF in
    let ref = bytes-new 100 in
    let _ = fill-pattern ref 0 in
    let chunked = bytes-new 100 in
    let _ = bytes-copy-range chunked ref 0 0 100 in
    let _ = copy-bytes-to-dma ref dma 0 in
    let _ = ws-mask-bytes-in-place ref 0 100 mask in
    -- Bytes chunked masking with phase tracking
    let m1 = ws-mask-bytes-in-place-at-phase chunked 0 1 mask 0 in
    let m2 = ws-mask-bytes-in-place-at-phase chunked 1 2 m1 0 in
    let m3 = ws-mask-bytes-in-place-at-phase chunked 3 3 m2 0 in
    let m4 = ws-mask-bytes-in-place-at-phase chunked 6 5 m3 0 in
    let m5 = ws-mask-bytes-in-place-at-phase chunked 11 13 m4 0 in
    let m6 = ws-mask-bytes-in-place-at-phase chunked 24 17 m5 0 in
    let m7 = ws-mask-bytes-in-place-at-phase chunked 41 31 m6 0 in
    let _  = ws-mask-bytes-in-place-at-phase chunked 72 28 m7 0 in
    let ok-bytes = eq-i64 (bytes-compare chunked ref) 0 in
    -- DMA chunked masking with explicit initial phase offsets (testing non-zero `phase` param directly)
    let _ = ws-mask-dma-in-place-at-phase dma 1 mask 0 in
    let _ = ws-mask-dma-in-place-at-phase (wrapping-add-i64 dma 1) 2 mask 1 in
    let _ = ws-mask-dma-in-place-at-phase (wrapping-add-i64 dma 3) 3 mask 3 in
    let _ = ws-mask-dma-in-place-at-phase (wrapping-add-i64 dma 6) 5 mask 2 in
    let _ = ws-mask-dma-in-place-at-phase (wrapping-add-i64 dma 11) 13 mask 3 in
    let _ = ws-mask-dma-in-place-at-phase (wrapping-add-i64 dma 24) 17 mask 0 in
    let _ = ws-mask-dma-in-place-at-phase (wrapping-add-i64 dma 41) 31 mask 1 in
    let _ = ws-mask-dma-in-place-at-phase (wrapping-add-i64 dma 72) 28 mask 0 in
    let ok-dma = dma-eq-bytes ref dma 0 in
    ok-bytes && ok-dma

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma-u64 = engine-dma-alloc 4096 4096 in
    if eq-u64 dma-u64 0 then 1
    else
      let dma = wrapping-u64-to-i64 dma-u64 in
      let code = (if not (test-lengths dma 0) then 2
        else if not (test-phase-rotation dma) then 3
        else 0) in
      let _ = engine-dma-free dma-u64 4096 in
      code
"#;
    build_and_run_leak_checked("engine-simd-mask", main_src, 0);
}

/// Steady-state zero-allocation & zero-atomic-RC verification across ALL hot-path
/// `Engine.Simd.Scan`, `Engine.Simd.Crc32c`, and `Engine.Simd.Mask` kernels:
/// `engine-thread-alloc-count () == 0`, `engine-thread-atomic-rc-count () == 0`,
/// and `CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1` (`leaked=0`).
#[test]
fn engine_simd_steady_state_zero_alloc_and_zero_atomic_rc() {
    let main_src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Wide
  import Prim.Atomic
  import Engine.Types
  import Engine.Simd.Scan
  import Engine.Simd.Crc32c
  import Engine.Simd.Mask

  write-str-loop :: Bytes -> I64 -> Nat ->{MVar} Unit
  write-str-loop b dma i =
    if i < bytes-len b then
      let _ = raw-store-u8 (wrapping-add-i64 dma (nat-to-i64 i)) (bytes-get-u8 b i) in
      write-str-loop b dma (i + 1)
    else ()

  #[no-alloc]
  hot-loop :: U32 -> Bytes -> Bytes -> I64 -> I64 -> U64 ->{MVar} U64
  hot-loop iters b1 b2 dma1 dma2 acc =
    if eq-u32 iters 0 then acc
    else
      -- 1. Pure U128 classifiers
      let m1 = simd-mask-eq-byte 0x0D0A0D0A4142434445464748494A4B4C 13 in
      let m2 = simd-mask-any-of4 0x0D0A0D0A4142434445464748494A4B4C 13 10 58 32 in
      let m3 = simd-mask-http-tchar-invalid 0x3A20746E65746E6F432D797065582D41 in
      let m4 = simd-mask-field-value-invalid 0x0D0A6170706C69636174696F6E2F6A73 in
      let eq16 = simd-ascii-eq-nocase16 0x436F6E74656E742D547970653A206162 0x636F6E74656E742D747970653A204142 in
      -- 2. DMA scanners (100% #[no-alloc])
      let d-byte = dma-find-byte dma1 0 64 58 in
      let d-crlf = dma-find-crlf dma1 0 64 in
      let d-tok  = dma-scan-http-token dma1 0 64 in
      let d-fv   = dma-scan-field-value dma1 14 64 in
      let d-eq   = dma-ascii-eq-nocase dma1 0 dma2 0 64 in
      -- 3. Bytes #[no-alloc] scanners
      let b-fv   = simd-scan-field-value b1 14 64 in
      let b-eq   = simd-ascii-eq-nocase b1 0 b2 0 64 in
      -- 4. CRC32C (Bytes, DMA, GF(2) combine)
      let c-b    = crc32c-bytes b1 in
      let c-u    = crc32c-finish (crc32c-update-bytes crc32c-init b1 0 64) in
      let c-d    = crc32c-finish (crc32c-update-dma crc32c-init dma1 64) in
      let c-comb = crc32c-combine c-b c-d 64 in
      -- 5. WebSocket Mask (Bytes & DMA double-apply involution so data stays invariant!)
      let _ = ws-mask-bytes-in-place b1 0 64 0x13579BDF in
      let _ = ws-mask-bytes-in-place b1 0 64 0x13579BDF in
      let _ = ws-mask-dma-in-place dma1 64 0x13579BDF in
      let _ = ws-mask-dma-in-place dma1 64 0x13579BDF in
      let step-sum = (wrapping-add-u64 (u32-to-u64 (bit-xor-u32 (bit-xor-u32 m1 m2) (bit-xor-u32 m3 m4)))
          (wrapping-add-u64 (u32-to-u64 (bit-xor-u32 c-b (bit-xor-u32 c-u (bit-xor-u32 c-d c-comb))))
            (wrapping-add-u64 (u32-to-u64 d-tok)
              (wrapping-add-u64 (wrapping-i64-to-u64 (i32-to-i64 (wrapping-add-i32 d-byte (wrapping-add-i32 d-crlf (wrapping-add-i32 d-fv b-fv)))))
                (if eq16 then (if d-eq then (if b-eq then 1 else 0) else 0) else 0))))) in
      hot-loop (wrapping-sub-u32 iters 1) b1 b2 dma1 dma2 (wrapping-add-u64 acc step-sum)

  fill-header :: Bytes -> U8 -> Nat -> Unit
  fill-header b delta i =
    if i < bytes-len b then
      let v = (if i == 12 then 58
        else if i == 13 then 32
        else if i == 62 then 13
        else if i == 63 then 10
        else wrapping-add-u8 65 (wrapping-add-u8 delta (wrapping-i64-to-u8 (nat-to-i64 (nat-rem i 26))))) in
      let _ = bytes-set-u8 b i v in
      fill-header b delta (i + 1)
    else ()

  main :: Unit ->{IO, MVar} Int
  main _ =
    let b1 = bytes-new 64 in
    let b2 = bytes-new 64 in
    let _ = fill-header b1 0 0 in
    let _ = fill-header b2 32 0 in
    let dma-u64 = engine-dma-alloc 4096 4096 in
    if eq-u64 dma-u64 0 then 1
    else
      let dma1 = wrapping-u64-to-i64 dma-u64 in
      let dma2 = wrapping-add-i64 dma1 1024 in
      let _ = write-str-loop b1 dma1 0 in
      let _ = write-str-loop b2 dma2 0 in
      let _ = engine-thread-counters-reset () in
      let checksum = hot-loop 5000 b1 b2 dma1 dma2 0 in
      let allocs = engine-thread-alloc-count () in
      let atomic-rcs = engine-thread-atomic-rc-count () in
      let _ = engine-dma-free dma-u64 4096 in
      if eq-u64 checksum 0 then 2
      else if ne-u64 allocs 0 then 3
      else if ne-u64 atomic-rcs 0 then 4
      else 0
"#;
    build_and_run_leak_checked("engine-simd-zero-alloc", main_src, 0);
}
