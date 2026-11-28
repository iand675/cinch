//! Formal LRAT verification (`TrustTier::KernelCert`), containment, cross-target codegen,
//! and end-to-end native execution tests for `Prim.SimdExt` and `Data.Bytes.Word` 128-bit loads/stores.
mod common;

use cinch::driver::Mode;
use cinch::prim_verify::prelude_bridge::extract_file_obligations;
use cinch::prim_verify::{Policy, TrustTier, VerificationStatus, VerifierEngine};
use std::path::Path;
use std::process::Command;

/// `cinch verify-prims --policy=jpl seed/prelude/Prim/SimdExt.cinch` must discharge
/// 100% of obligations to LRAT-certified `TrustTier::KernelCert` with 0 Disproved and 0 Assumed.
#[test]
fn prim_simd_ext_verify_prims_jpl_100_percent_kernel_cert() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let simd_ext_path = manifest_dir.join("prelude/Prim/SimdExt.cinch");

    // 1. Run the `cinch verify-prims --policy=jpl --json` CLI and assert zero skips/failures.
    let output = Command::new(env!("CARGO_BIN_EXE_cinch"))
        .args(["verify-prims", "--policy=jpl", "--json"])
        .arg(&simd_ext_path)
        .output()
        .expect("run cinch verify-prims");
    assert!(
        output.status.success(),
        "verify-prims --policy=jpl failed:\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let summary: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|e| {
        panic!(
            "invalid JSON from verify-prims: {e}\nstdout={}",
            String::from_utf8_lossy(&output.stdout)
        )
    });
    assert_eq!(summary["policy"].as_str(), Some("jpl"));
    assert_eq!(summary["skipped_unsafe"].as_u64(), Some(0), "expected 0 Assumed/unsafe bodies: {summary}");
    assert_eq!(summary["skipped_unsupported"].as_u64(), Some(0), "expected 0 unsupported bodies: {summary}");
    assert_eq!(summary["disproved"].as_u64(), Some(0), "expected 0 disproved obligations: {summary}");
    assert_eq!(summary["rejected_by_policy"].as_u64(), Some(0), "expected 0 policy-rejected obligations: {summary}");
    assert_eq!(summary["unverified"].as_u64(), Some(0), "expected 0 open/unverified obligations: {summary}");
    let total = summary["obligations_total"].as_u64().unwrap();
    let verified = summary["verified"].as_u64().unwrap();
    assert!(total >= 20, "expected >= 20 obligations (well-defined + cross-target), got {total}");
    assert_eq!(verified, total, "every obligation must verify: {summary}");

    // 2. Independently verify every obligation via `VerifierEngine(Policy::jpl_strict())`
    //    and assert every single receipt reaches `TrustTier::KernelCert` (LRAT).
    let mut engine = VerifierEngine::default_engine(Policy::jpl_strict());
    let sems = engine.semantics_refs();
    let extracted = extract_file_obligations(&simd_ext_path, &sems).expect("extract Prim.SimdExt");
    let mut cert_count = 0usize;
    for body in &extracted {
        assert!(!body.is_unsafe, "{}.{} must not be unsafe", body.module, body.prim_name);
        assert!(
            body.skip_reason.is_none(),
            "{}.{} ({}) unexpectedly skipped: {:?}",
            body.module,
            body.prim_name,
            body.when_desc,
            body.skip_reason
        );
        for ob in &body.obligations {
            match &engine.verify(ob).status {
                VerificationStatus::Verified { receipt, .. } => {
                    assert_eq!(
                        receipt.tier,
                        TrustTier::KernelCert,
                        "obligation {} must reach KernelCert, got {:?}",
                        ob.name,
                        receipt.tier
                    );
                    cert_count += 1;
                }
                other => panic!("obligation {} failed under JPL strict: {other:?}", ob.name),
            }
        }
    }
    assert_eq!(cert_count as u64, total);
}

/// End-to-end native execution tests routed through `common::drive_with_profile`:
/// - `Data.Bytes.Word.bytes-read-le128` / `bytes-write-le128` across alignments `0..15`
/// - `Prim.SimdExt.simd-u8x16-shuffle` on ASCII/HTTP LUT lookup + OOB indices `16..127` & `128..255` -> `0`
/// - `Prim.SimdExt.simd-u8x16-xor` / `or` / `not`
/// - `Prim.SimdExt.hw-crc32c-u8`, `hw-crc32c-u32`, `hw-crc32c-u64` against RFC 3720 `"123456789"` == `0xE3069283`
#[test]
fn prim_simd_ext_and_bytes_word_le128_native_execution() {
    let dir = tempfile::tempdir().unwrap();
    let main_src = r#"-- | End-to-end runtime tests for Data.Bytes.Word 128-bit loads/stores and Prim.SimdExt.
module Main where
  import Data.Bytes
  import Data.Bytes.Word
  import Data.Machine
  import Data.Nat
  import Data.Wide
  import Prim.SimdExt

  -- | Fill a buffer with deterministic non-zero sentinel bytes so untouched surroundings can be checked.
  fill-pattern :: Bytes -> Nat -> Unit
  fill-pattern b i =
    if i < bytes-len b then
      let v = wrapping-i64-to-u8 (nat-to-i64 (nat-rem (i * 17 + 3) 251 + 1)) in
      let _ = bytes-set-u8 b i v in
      fill-pattern b (i + 1)
    else ()

  -- | Pack 16 explicit U8 bytes (lane 0 = LSB, lane 15 = MSB) into a U128 via bytes-read-le128.
  pack16 :: U8 -> U8 -> U8 -> U8 -> U8 -> U8 -> U8 -> U8 -> U8 -> U8 -> U8 -> U8 -> U8 -> U8 -> U8 -> U8 -> U128
  pack16 b0 b1 b2 b3 b4 b5 b6 b7 b8 b9 b10 b11 b12 b13 b14 b15 =
    let buf = bytes-from-u8s [b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11, b12, b13, b14, b15] in
    if 16 <= bytes-len buf then bytes-read-le128 buf 0 else 0

  -- | Extract byte lane `k` (0..15) of a U128 via little-endian shift + trunc.
  lane-u8 :: U128 -> U32 -> U8
  lane-u8 x k = wrapping-u128-to-u8 (wrapping-shr-u128 x (wrapping-mul-u32 k 8))

  -- | Verify bytes-write-le128 + bytes-read-le128 at offset `pos` (0..15) and confirm surrounding bytes are intact.
  check-align-one :: Nat -> Bool
  check-align-one pos =
    let buf = bytes-new 48 in
    let _ = fill-pattern buf 0 in
    let ref-buf = bytes-new 48 in
    let _ = fill-pattern ref-buf 0 in
    let val = pack16 0x01 0x23 0x45 0x67 0x89 0xAB 0xCD 0xEF 0x10 0x32 0x54 0x76 0x98 0xBA 0xDC 0xFE in
    if pos + 16 <= bytes-len buf then
      let _ = bytes-write-le128 buf pos val in
      let got = bytes-read-le128 buf pos in
      let roundtrip = eq-u128 got val in
      let bytes-match = check-le128-bytes buf pos val 0 in
      let before-ok = check-slice-eq buf ref-buf 0 pos in
      let after-ok = check-slice-eq buf ref-buf (pos + 16) 48 in
      roundtrip && bytes-match && before-ok && after-ok
    else False

  -- | Verify each of the 16 written bytes matches little-endian lane order.
  check-le128-bytes :: Bytes -> Nat -> U128 -> Nat -> Bool
  check-le128-bytes b pos val k =
    if k >= 16 then True
    else if pos + k < bytes-len b then
      let got = bytes-get-u8 b (pos + k) in
      let exp = lane-u8 val (wrapping-i64-to-u32 (nat-to-i64 k)) in
      eq-u8 got exp && check-le128-bytes b pos val (k + 1)
    else False

  -- | Verify two buffers agree on [i, end).
  check-slice-eq :: Bytes -> Bytes -> Nat -> Nat -> Bool
  check-slice-eq a b i end =
    if i >= end then True
    else if i < bytes-len a && i < bytes-len b then
      eq-u8 (bytes-get-u8 a i) (bytes-get-u8 b i) && check-slice-eq a b (i + 1) end
    else False

  -- | Test all alignments 0..15 for bytes-read-le128 / bytes-write-le128.
  check-all-alignments :: Nat -> Bool
  check-all-alignments pos =
    if pos >= 16 then True
    else check-align-one pos && check-all-alignments (pos + 1)

  -- | Check simd-u8x16-shuffle on an HTTP/ASCII character-class LUT and across ALL 256 index byte values.
  check-shuffle-all256 :: U128 -> Nat -> Bool
  check-shuffle-all256 tbl base =
    if base >= 256 then True
    else
      let idx = make-seq16 base in
      let out = simd-u8x16-shuffle tbl idx in
      check-shuffle-block tbl base out 0 && check-shuffle-all256 tbl (base + 16)

  -- | Build 16 consecutive byte indices [base, base+1, ..., base+15].
  make-seq16 :: Nat -> U128
  make-seq16 base =
    let b = bytes-new 16 in
    let _ = fill-seq b base 0 in
    if 16 <= bytes-len b then bytes-read-le128 b 0 else 0

  fill-seq :: Bytes -> Nat -> Nat -> Unit
  fill-seq b base i =
    if i < 16 && i < bytes-len b && base + i < 256 then
      let _ = bytes-set-u8 b i (wrapping-i64-to-u8 (nat-to-i64 (base + i))) in
      fill-seq b base (i + 1)
    else ()

  -- | Verify each output lane of a 16-index shuffle block: 0..15 -> tbl[idx], 16..255 -> 0.
  check-shuffle-block :: U128 -> Nat -> U128 -> Nat -> Bool
  check-shuffle-block tbl base out lane =
    if lane >= 16 then True
    else
      let idx = base + lane in
      let got = lane-u8 out (wrapping-i64-to-u32 (nat-to-i64 lane)) in
      let exp = if idx < 16 then lane-u8 tbl (wrapping-i64-to-u32 (nat-to-i64 idx)) else 0 in
      eq-u8 got exp && check-shuffle-block tbl base out (lane + 1)

  -- | HTTP/ASCII low-nibble LUT lookup test mixing in-bounds (0..15), mid OOB (16..127), and high OOB (128..255).
  check-shuffle-http-lut :: Bool
  check-shuffle-http-lut =
    -- Bitmask LUT indexed by low nibble (e.g. HTTP separator classification: CR=13, LF=10, ':'=10, ' '=0)
    let lut = pack16 0x01 0x02 0x04 0x08 0x10 0x20 0x40 0x80 0x11 0x22 0x33 0x44 0x55 0x66 0x77 0x88 in
    let idx = pack16 0 10 13 15 16 17 63 127 128 129 200 255 3 7 32 0x81 in
    let exp = pack16 0x01 0x33 0x66 0x88 0 0 0 0 0 0 0 0 0x08 0x80 0 0 in
    let got = simd-u8x16-shuffle lut idx in
    eq-u128 got exp && check-shuffle-all256 lut 0

  -- | Verify bitwise lane ops (xor, or, not).
  check-bitwise :: Bool
  check-bitwise =
    let a = pack16 0x00 0x0F 0xF0 0xFF 0x55 0xAA 0x12 0x34 0x56 0x78 0x9A 0xBC 0xDE 0xF0 0x01 0x80 in
    let b = pack16 0xFF 0xF0 0x0F 0x00 0xAA 0x55 0x34 0x12 0x78 0x56 0xBC 0x9A 0xF0 0xDE 0x80 0x01 in
    let x = simd-u8x16-xor a b in
    let o = simd-u8x16-or a b in
    let n = simd-u8x16-not a in
    eq-u128 x (bit-xor-u128 a b)
      && eq-u128 o (bit-or-u128 a b)
      && eq-u128 n (bit-not-u128 a)
      && eq-u128 (simd-u8x16-xor a a) 0
      && eq-u128 (simd-u8x16-or a n) (bit-not-u128 0)

  -- | Fold all bytes of a buffer using hw-crc32c-u8.
  crc32c-u8-loop :: Bytes -> Nat -> U32 -> U32
  crc32c-u8-loop b i acc =
    if i < bytes-len b then
      crc32c-u8-loop b (i + 1) (hw-crc32c-u8 acc (bytes-get-u8 b i))
    else acc

  -- | Verify hw-crc32c-u8, hw-crc32c-u32, and hw-crc32c-u64 against RFC 3720 `"123456789"` == 0xE3069283.
  check-crc32c-rfc3720 :: Bool
  check-crc32c-rfc3720 =
    let msg = bytes-from-u8s [49, 50, 51, 52, 53, 54, 55, 56, 57] in
    let init = 0xFFFFFFFF in
    let c-u8 = bit-xor-u32 (crc32c-u8-loop msg 0 init) 0xFFFFFFFF in
    let c-u32 = (if 9 <= bytes-len msg then
      let w0 = bytes-read-le32 msg 0 in
      let w1 = bytes-read-le32 msg 4 in
      let b8 = bytes-get-u8 msg 8 in
      bit-xor-u32 (hw-crc32c-u8 (hw-crc32c-u32 (hw-crc32c-u32 init w0) w1) b8) 0xFFFFFFFF
      else 0) in
    let c-u64 = (if 9 <= bytes-len msg then
      let q0 = bytes-read-le64 msg 0 in
      let b8 = bytes-get-u8 msg 8 in
      bit-xor-u32 (hw-crc32c-u8 (hw-crc32c-u64 init q0) b8) 0xFFFFFFFF
      else 0) in
    -- Also verify multi-width consistency on a 25-byte buffer (2x u64 + 2x u32 + 1x u8 == 25x u8).
    let buf25 = bytes-new 25 in
    let _ = fill-pattern buf25 0 in
    let ref25 = bit-xor-u32 (crc32c-u8-loop buf25 0 init) 0xFFFFFFFF in
    let mix25 = (if 25 <= bytes-len buf25 then
      let s1 = hw-crc32c-u64 init (bytes-read-le64 buf25 0) in
      let s2 = hw-crc32c-u64 s1 (bytes-read-le64 buf25 8) in
      let s3 = hw-crc32c-u32 s2 (bytes-read-le32 buf25 16) in
      let s4 = hw-crc32c-u32 s3 (bytes-read-le32 buf25 20) in
      let s5 = hw-crc32c-u8 s4 (bytes-get-u8 buf25 24) in
      bit-xor-u32 s5 0xFFFFFFFF
      else 0) in
    eq-u32 c-u8 0xE3069283
      && eq-u32 c-u32 0xE3069283
      && eq-u32 c-u64 0xE3069283
      && eq-u32 ref25 mix25

  main :: Int
  main =
    if not (check-all-alignments 0) then 1
    else if not check-shuffle-http-lut then 2
    else if not check-bitwise then 3
    else if not check-crc32c-rfc3720 then 4
    else 0
"#;
    std::fs::write(dir.path().join("Main.cinch"), main_src).unwrap();
    let exe = dir.path().join("prim_simd_ext_consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    common::drive_with_profile(
        Mode::Build,
        cinch::check::StageMode::Refresh,
        dir.path(),
        "Main.cinch",
        Some(exe.clone()),
        &profile,
    )
    .expect("build Prim.SimdExt native consumer");

    let output = Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "32M")
        .output()
        .expect("run native consumer");
    assert_eq!(
        output.status.code(),
        Some(0),
        "native execution failed with code {:?}:\nstdout={}\nstderr={}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("leaked=0"),
        "expected leaked=0, got: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Cross-target compilation: `Prim.SimdExt` must emit valid LLVM IR that `clang` assembles
/// cleanly on `x86_64-unknown-linux-gnu` (masked `pshufb`, `crc32b/l/q`),
/// `aarch64-unknown-linux-gnu` (NEON `tbl`, `crc32cb/w/x`), and `wasm32-unknown-unknown` (portable baseline).
#[test]
fn prim_simd_ext_compiles_for_x86_aarch64_and_wasm32() {
    let dir = tempfile::tempdir().unwrap();
    let src = format!(
        "{}\nmodule Main where\n  import Data.Machine\n  import Data.Wide\n  import Prim.SimdExt\n\n  main :: Int\n  main =\n    let s = simd-u8x16-shuffle 0x0F0E0D0C0B0A09080706050403020100 0x000102030405060708090A0B0C0D0E0F in\n    let x = simd-u8x16-not (simd-u8x16-or (simd-u8x16-xor s 1) 2) in\n    let c = hw-crc32c-u64 (hw-crc32c-u32 (hw-crc32c-u8 0xFFFFFFFF 49) 0x35343332) 0x3938373635343332 in\n    if eq-u128 x 0 && eq-u32 c 0 then 1 else 0\n",
        include_str!("../prelude/Prim/SimdExt.cinch")
    );
    for (triple, arch) in [
        ("x86_64-unknown-linux-gnu", "x86_64"),
        ("aarch64-unknown-linux-gnu", "aarch64"),
        ("wasm32-unknown-unknown", "wasm32"),
    ] {
        let profile = cinch::driver::target_profile(&cinch::driver::DriverOptions {
            target: Some(triple.into()),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(profile.arch, arch);
        let src_owned = src.to_string();
        let ir = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || common::source_frontend::emit_ir_profile(&src_owned, &profile))
            .unwrap()
            .join()
            .unwrap()
            .unwrap_or_else(|e| panic!("{triple} emit_ir_profile failed: {e}"));
        let ll_path = dir.path().join(format!("{arch}.ll"));
        std::fs::write(&ll_path, ir).unwrap();
        let out = Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
            .args(["-target", triple, "-O2", "-c", "-x", "ir"])
            .arg(&ll_path)
            .arg("-o")
            .arg(ll_path.with_extension("o"))
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{triple} clang failed:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
}

/// Containment negative tests: non-`unsafe` inline `asm` is strictly rejected unless ALL three
/// invariants hold (body has `justify verify`, no `sideeffect`/`~{memory}`, whitelisted register-only op).
#[test]
fn prim_verified_register_asm_containment_rejects_unjustified_sideeffect_and_unknown_ops() {
    let cases = [
        (
            "missing-justify-verify",
            r#"module Main where
  prim bad-no-verify :: U32 -> U8 -> U32 where
    llvm {
      %r = call i32 asm "crc32b $2, $0", "=r,0,q"(i32 %0, i8 %1)
      ret i32 %r
    }
  main :: Int
  main = 0
"#,
        ),
        (
            "sideeffect-asm",
            r#"module Main where
  prim bad-sideeffect :: U32 -> U8 -> U32 where
    llvm {
      %r = call i32 asm sideeffect "crc32b $2, $0", "=r,0,q"(i32 %0, i8 %1)
      ret i32 %r
    } justify verify
  main :: Int
  main = 0
"#,
        ),
        (
            "memory-clobber",
            r#"module Main where
  prim bad-mem-clobber :: U32 -> U8 -> U32 where
    llvm {
      %r = call i32 asm "crc32b $2, $0", "=r,0,q,~{memory}"(i32 %0, i8 %1)
      ret i32 %r
    } justify verify
  main :: Int
  main = 0
"#,
        ),
        (
            "unmodeled-instruction",
            r#"module Main where
  prim bad-nop :: U32 -> U32 where
    llvm {
      %r = call i32 asm "nop", "=r,0"(i32 %0)
      ret i32 %r
    } justify verify
  main :: Int
  main = 0
"#,
        ),
    ];

    for (label, src) in cases {
        let src_owned = src.to_string();
        let label_owned = label.to_string();
        let res = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || common::source_frontend::check_sources(&src_owned, &format!("{label_owned}.cinch")))
            .unwrap()
            .join()
            .unwrap();
        let err = res.expect_err(&format!("{label} MUST be rejected by containment"));
        assert!(
            err.contains("containment") || err.contains("unsafe"),
            "{label}: unexpected error: {err}"
        );
    }
}
