//! Integration, RFC 9000 varint & frame codec, RFC 9001 Appendix A bit-for-bit
//! packet protection test vectors, core-routable Connection ID steering, and
//! 50,000-packet zero-allocation hot-loop tests for
//! `Net.Quic.{Types,Varint,Frame,Packet}`.
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
        "cinch-net-quic-crypto-{tag}-{}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn build_and_run_leak_checked(tag: &str, src: &str, expected_code: i32) -> (String, String) {
    let dir = fresh_dir(tag);
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let exe = dir.join(format!("quic-crypto-{tag}"));
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
// 1. Stdlib manifest registration & #[no-alloc] module check
// =========================================================================

#[test]
fn net_quic_crypto_modules_registered_and_no_alloc_checked() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, rel_path) in [
        ("Net.Quic.Types", "Net/Quic/Types.cinch"),
        ("Net.Quic.Varint", "Net/Quic/Varint.cinch"),
        ("Net.Quic.Frame", "Net/Quic/Frame.cinch"),
        ("Net.Quic.Packet", "Net/Quic/Packet.cinch"),
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

    // Type-check a caller that invokes #[no-alloc] entrypoints from all 4 modules inside #[no-alloc].
    let dir = fresh_dir("no-alloc-check");
    std::fs::write(
        dir.join("Main.cinch"),
        r#"
module Main where
  import Data.Machine
  import Engine.Types
  import Net.Quic.Types
  import Net.Quic.Varint
  import Net.Quic.Frame
  import Net.Quic.Packet

  #[no-alloc]
  check-no-alloc-chain :: I64 -> U32 -> QuicCryptoKey ->{MVar} U32
  check-no-alloc-chain buf cap key =
    let cid = quic-cid-encode-core (core-id 7) 12345 0 8 in
    let hlen = quic-encode-short-header buf cap cid 0 0 42 2 in
    let flen = quic-frame-encode-ping buf hlen cap in
    let _v = quic-varint-len 15293 in
    let _pn = quic-pn-decode 0xa82f30ea 0x9b32 16 in
    quic-packet-protect-in-place key buf 9 hlen flen 42

  main :: Int
  main = 0
"#,
    )
    .expect("write Main.cinch");

    let opts = DriverOptions::default();
    let res = common::drive_with_profile_opts(
        Mode::Check,
        StageMode::Refresh,
        &dir,
        "Main.cinch",
        None,
        &TargetProfile::native(),
        &opts,
    );
    let _ = std::fs::remove_dir_all(&dir);
    res.expect("Net.Quic.* #[no-alloc] check must succeed");
}

// =========================================================================
// 2. RFC 9000 §16 & §A.1 Varint exhaustive boundaries, non-minimal decode,
//    fixed-width 2B/4B encoding, and overflow/truncation rejection
// =========================================================================

#[test]
fn quic_varint_rfc9000_boundaries_and_vectors() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Result
  import Engine.Types
  import Net.Quic.Varint
  import Prim.Atomic
  import Text.Hex

  hex-must :: String -> Bytes
  hex-must s = case hex-decode s of
    Ok b -> b
    Err _ -> bytes-new 0

  copy-to-dma :: Bytes -> Nat -> I64 -> Nat ->{MVar} Unit
  copy-to-dma src i dst n =
    if i >= n then ()
    else if i < bytes-len src then
      let _ = raw-store-u8 (wrapping-add-i64 dst (nat-to-i64 i)) (bytes-get-u8 src i) in
      copy-to-dma src (i + 1) dst n
    else ()

  dma-eq-hex :: I64 -> String ->{MVar} Bool
  dma-eq-hex addr hex =
    let exp = hex-must hex in
    dma-eq-bytes addr exp 0 (bytes-len exp)

  dma-eq-bytes :: I64 -> Bytes -> Nat -> Nat ->{MVar} Bool
  dma-eq-bytes addr exp i n =
    if i >= n then True
    else if i < bytes-len exp then
      if eq-u8 (raw-load-u8 (wrapping-add-i64 addr (nat-to-i64 i))) (bytes-get-u8 exp i) then
        dma-eq-bytes addr exp (i + 1) n
      else False
    else False

  check-roundtrip :: I64 -> U64 -> U32 ->{MVar} Bool
  check-roundtrip buf v exp-len =
    if ne-u32 (quic-varint-len v) exp-len then False
    else
      let w = quic-varint-encode-raw buf 0 16 v in
      if ne-u32 w exp-len then False
      else
        let d = quic-varint-decode-raw buf 0 w in
        if not (varint-decoded-ok d) then False
        else if ne-u32 d.consumed exp-len then False
        else eq-u64 d.val v

  check-vector :: I64 -> String -> U64 -> U32 ->{MVar} Bool
  check-vector buf hex exp-val exp-len =
    let b = hex-must hex in
    let n = bytes-len b in
    let _ = copy-to-dma b 0 buf n in
    let d = quic-varint-decode-raw buf 0 (wrapping-i64-to-u32 (nat-to-i64 n)) in
    if not (varint-decoded-ok d) then False
    else if ne-u64 d.val exp-val then False
    else eq-u32 d.consumed exp-len

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 4096 4096 in
    if eq-u64 dma 0 then 99
    else
      let buf = wrapping-u64-to-i64 dma in
      let rc = (if not (check-roundtrip buf 0 1) then 1
        else if not (check-roundtrip buf 63 1) then 2
        else if not (check-roundtrip buf 64 2) then 3
        else if not (check-roundtrip buf 16383 2) then 4
        else if not (check-roundtrip buf 16384 4) then 5
        else if not (check-roundtrip buf 1073741823 4) then 6
        else if not (check-roundtrip buf 1073741824 8) then 7
        else if not (check-roundtrip buf 4611686018427387903 8) then 8
        -- Overflow rejection: 2^62 = 4611686018427387904
        else if ne-u32 (quic-varint-len 4611686018427387904) 0 then 9
        else if ne-u32 (quic-varint-encode-raw buf 0 16 4611686018427387904) 0 then 10
        -- RFC 9000 Appendix A.1 exact test vectors
        else if not (check-vector buf "c2197c5eff14e88c" 151288809941952652 8) then 11
        else if not (check-vector buf "9d7f3e7d" 494878333 4) then 12
        else if not (check-vector buf "7bbd" 15293 2) then 13
        else if not (check-vector buf "25" 37 1) then 14
        -- Non-minimal encoding (RFC 9000 §A.1: 0x4025 -> 37 in 2 bytes)
        else if not (check-vector buf "4025" 37 2) then 15
        -- Verify re-encoding exact RFC 9000 A.1 bytes
        else if ne-u32 (quic-varint-encode-raw buf 0 16 151288809941952652) 8 then 16
        else if not (dma-eq-hex buf "c2197c5eff14e88c") then 17
        else if ne-u32 (quic-varint-encode-raw buf 0 16 494878333) 4 then 18
        else if not (dma-eq-hex buf "9d7f3e7d") then 19
        else if ne-u32 (quic-varint-encode-raw buf 0 16 15293) 2 then 20
        else if not (dma-eq-hex buf "7bbd") then 21
        -- Fixed-width 2-byte and 4-byte encoders
        else if ne-u32 (quic-varint-encode2b buf 0 16 37) 2 then 22
        else if not (dma-eq-hex buf "4025") then 23
        else if ne-u32 (quic-varint-encode2b buf 0 16 16384) 0 then 24
        else if ne-u32 (quic-varint-encode4b buf 0 16 37) 4 then 25
        else if not (dma-eq-hex buf "80000025") then 26
        else if ne-u32 (quic-varint-encode4b buf 0 16 1073741824) 0 then 27
        -- Truncation & small buffer checks
        else if ne-u32 (quic-varint-encode-raw buf 0 1 64) 0 then 28
        else if varint-decoded-ok (quic-varint-decode-raw buf 0 0) then 29
        else if varint-decoded-ok (quic-varint-decode-raw buf 0 3) then 30
        else 0) in
      let _ = engine-dma-free dma 4096 in
      rc
"#;
    build_and_run_leak_checked("varint-rfc9000", src, 0);
}

// =========================================================================
// 3. RFC 9001 Appendix A bit-for-bit test vectors (A.2, A.3, A.4, A.5)
// =========================================================================

#[test]
fn rfc9001_appendix_a_bit_for_bit_vectors() {
    let src = r#"
module Main where
  import Crypto.Tls
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Result
  import Engine.Types
  import Net.Quic.Types
  import Net.Quic.Varint
  import Net.Quic.Frame
  import Net.Quic.Packet
  import Net.Tls.Fast
  import Prim.Atomic
  import Text.Hex

  hex-must :: String -> Bytes
  hex-must s = case hex-decode s of
    Ok b -> b
    Err _ -> bytes-new 0

  bytes-eq :: Bytes -> Bytes -> Bool
  bytes-eq a b = eq-i64 (bytes-compare a b) 0

  copy-to-dma :: Bytes -> Nat -> I64 -> Nat ->{MVar} Unit
  copy-to-dma src i dst n =
    if i >= n then ()
    else if i < bytes-len src then
      let _ = raw-store-u8 (wrapping-add-i64 dst (nat-to-i64 i)) (bytes-get-u8 src i) in
      copy-to-dma src (i + 1) dst n
    else ()

  dma-eq-bytes :: I64 -> Bytes -> Nat -> Nat ->{MVar} Bool
  dma-eq-bytes addr exp i n =
    if i >= n then True
    else if i < bytes-len exp then
      if eq-u8 (raw-load-u8 (wrapping-add-i64 addr (nat-to-i64 i))) (bytes-get-u8 exp i) then
        dma-eq-bytes addr exp (i + 1) n
      else False
    else False

  -- RFC 9001 Appendix A.2: Client Initial (DCID 8394c8f03e515708, PN = 2)
  test-a2 :: I64 ->{IO, MVar} Int
  test-a2 buf =
    let dcid = conn-id-from-bytes (hex-must "8394c8f03e515708") in
    case quic-initial-crypto-keys-new dcid of
      (ck, sk) ->
        let _ = quic-crypto-key-destroy sk in
        if not (quic-crypto-key-is-valid ck) then 101
        else
          let ch = hex-must "010000ed0303ebf8fa56f12939b9584a3896472ec40bb863cfd3e86804fe3a47f06a2b69484c00000413011302010000c000000010000e00000b6578616d706c652e636f6dff01000100000a00080006001d0017001800100007000504616c706e000500050100000000003300260024001d00209370b2c9caa47fbae4f213ce7b9bf6c1d3ba0f963003ee6c96be1763ed0660a7002b0003020304000d0010000e0403050306030203080408050806002d00020101001c00024001003900320408ffffffffffffffff05048000ffff07048000ffff0801100104800075300901100f088394c8f03e51570806048000ffff" in
          let ch-len = bytes-len ch in
          let payload-len = (1162 :: U32) in
          -- Unprotected header: 22B (pn_off = 18, pn_len = 4, wire_length = 4 + 1162 + 16 = 1182 = 0x449e)
          let hdr-len = quic-encode-long-header buf 2048 (quic-version-v1 ()) (packet-type-initial ()) dcid (conn-id-empty ()) 0 0 1182 2 2 4 in
          let exp-unprot-hdr = hex-must "c300000001088394c8f03e5157080000449e00000002" in
          let ch-scratch = wrapping-add-i64 buf 2048 in
          let _c = copy-to-dma ch 0 ch-scratch ch-len in
          let ch-len-u32 = if ch-len <= 2048 then wrapping-i64-to-u32 (nat-to-i64 ch-len) else 0 in
          let crypto-written = quic-frame-encode-crypto buf hdr-len 2048 0 ch-scratch ch-len-u32 in
          let pad-len = wrapping-sub-u32 payload-len crypto-written in
          let pad-written = quic-frame-encode-padding buf (wrapping-add-u32 hdr-len crypto-written) 2048 pad-len in
          let rc = (if ne-u32 hdr-len 22 then 102
            else if not (dma-eq-bytes buf exp-unprot-hdr 0 22) then 103
            else if ne-u32 (wrapping-add-u32 crypto-written pad-written) payload-len then 104
            else
              let wire-len = quic-packet-protect-in-place ck buf 18 hdr-len payload-len 2 in
              -- Exact RFC 9001 A.2 protected header (22B) + ciphertext sample (16B)
              let exp-prefix = hex-must "c000000001088394c8f03e5157080000449e7b9aec34d1b1c98dd7689fb8ec11d242b123dc9b" in
              if ne-u32 wire-len 1200 then 105
              else if not (dma-eq-bytes buf exp-prefix 0 38) then 106
              else
                -- Now unprotect in-place and verify exact CRYPTO frame + ClientHello recovered!
                let dec = quic-packet-unprotect-in-place ck buf wire-len 0 quic-pn-none in
                if not (quic-packet-header-ok dec) then 107
                else if ne-u64 dec.qph-full-pn 2 then 108
                else if ne-u32 dec.qph-hdr-len 22 then 109
                else if ne-u32 dec.qph-payload-len payload-len then 110
                else
                  let body-addr = wrapping-add-i64 buf (u32-to-i64 dec.qph-hdr-len) in
                  let fr = quic-frame-decode-raw body-addr 0 dec.qph-payload-len in
                  if not (quic-frame-ok fr) then 111
                  else if ne-u64 fr.qf-type frame-type-crypto then 112
                  else if ne-u64 fr.qf-f0 0 then 113
                  else if ne-u32 fr.qf-data-len 241 then 114
                  else if not (dma-eq-bytes (wrapping-add-i64 body-addr (u32-to-i64 fr.qf-data-off)) ch 0 ch-len) then 115
                  else
                    -- Verify remaining 917 bytes are all PADDING
                    let skipped = quic-frame-skip-padding-raw body-addr fr.qf-consumed dec.qph-payload-len in
                    if ne-u32 skipped pad-len then 116
                    else 0) in
          let _ = quic-crypto-key-destroy ck in
          rc

  -- RFC 9001 Appendix A.3: Server Initial (PN = 1, full 135-byte wire match!)
  test-a3 :: I64 ->{IO, MVar} Int
  test-a3 buf =
    let client-dcid = conn-id-from-bytes (hex-must "8394c8f03e515708") in
    let server-scid = conn-id-from-bytes (hex-must "f067a5502a4262b5") in
    case quic-initial-crypto-keys-new client-dcid of
      (ck, sk) ->
        let _ = quic-crypto-key-destroy ck in
        if not (quic-crypto-key-is-valid sk) then 201
        else
          let payload = hex-must "02000000000600405a020000560303eefce7f7b37ba1d1632e96677825ddf73988cfc79825df566dc5430b9a045a1200130100002e00330024001d00209d3c940d89690b84d08a60993c144eca684d1081287c834d5311bcf32bb9da1a002b00020304" in
          let plen = bytes-len payload in
          let exp-wire = hex-must "cf000000010008f067a5502a4262b5004075c0d95a482cd0991cd25b0aac406a5816b6394100f37a1c69797554780bb38cc5a99f5ede4cf73c3ec2493a1839b3dbcba3f6ea46c5b7684df3548e7ddeb9c3bf9c73cc3f3bded74b562bfb19fb84022f8ef4cdd93795d77d06edbb7aaf2f58891850abbdca3d20398c276456cbc42158407dd074ee" in
          let hdr-len = quic-encode-long-header buf 2048 (quic-version-v1 ()) (packet-type-initial ()) (conn-id-empty ()) server-scid 0 0 117 2 1 2 in
          let _c = copy-to-dma payload 0 (wrapping-add-i64 buf (u32-to-i64 hdr-len)) plen in
          let plen-u32 = if plen <= 2048 then wrapping-i64-to-u32 (nat-to-i64 plen) else 0 in
          let rc = (if ne-u32 hdr-len 20 then 202
            else
              let wire-len = quic-packet-protect-in-place sk buf 18 hdr-len plen-u32 1 in
              if ne-u32 wire-len 135 then 203
              else if not (dma-eq-bytes buf exp-wire 0 135) then 204
              else
                -- Unprotect and verify ACK + CRYPTO frames inside Server Initial
                let dec = quic-packet-unprotect-in-place sk buf wire-len 0 quic-pn-none in
                if not (quic-packet-header-ok dec) then 205
                else if ne-u64 dec.qph-full-pn 1 then 206
                else if ne-u32 dec.qph-payload-len 99 then 207
                else if not (dma-eq-bytes (wrapping-add-i64 buf (u32-to-i64 dec.qph-hdr-len)) payload 0 plen) then 208
                else
                  let body-addr = wrapping-add-i64 buf (u32-to-i64 dec.qph-hdr-len) in
                  let f-ack = quic-frame-decode-raw body-addr 0 dec.qph-payload-len in
                  if not (quic-frame-ok f-ack) || ne-u64 f-ack.qf-type frame-type-ack then 209
                  else
                    let f-crypto = quic-frame-decode-raw body-addr f-ack.qf-consumed dec.qph-payload-len in
                    if not (quic-frame-ok f-crypto) || ne-u64 f-crypto.qf-type frame-type-crypto || ne-u32 f-crypto.qf-data-len 90 then 210
                    else 0) in
          let _ = quic-crypto-key-destroy sk in
          rc

  -- RFC 9001 Appendix A.4: Retry Integrity Tag (04a265ba2eff4d829058fb3f0f2496ba)
  test-a4 :: I64 ->{IO, MVar} Int
  test-a4 buf =
    let odcid = conn-id-from-bytes (hex-must "8394c8f03e515708") in
    let scid = conn-id-from-bytes (hex-must "f067a5502a4262b5") in
    let tok = hex-must "746f6b656e" in
    let exp-no-tag = hex-must "ff000000010008f067a5502a4262b5746f6b656e" in
    let exp-tag = hex-must "04a265ba2eff4d829058fb3f0f2496ba" in
    let exp-full = hex-must "ff000000010008f067a5502a4262b5746f6b656e04a265ba2eff4d829058fb3f0f2496ba" in
    let tok-scratch = wrapping-add-i64 buf 2048 in
    let _tc = copy-to-dma tok 0 tok-scratch 5 in
    let hlen = quic-encode-retry-header buf 2048 (quic-version-v1 ()) 0x0f (conn-id-empty ()) scid tok-scratch 5 in
    if ne-u32 hlen 20 then 61
    else if not (dma-eq-bytes buf exp-no-tag 0 20) then 62
    else
      let tag = quic-retry-compute-tag (quic-version-v1 ()) odcid exp-no-tag in
      if not (bytes-eq tag exp-tag) then 63
      else if not (quic-retry-verify-packet (quic-version-v1 ()) odcid exp-full) then 64
      else
        -- Verify raw zero-alloc Retry tag compute/verify + tamper rejection
        let rkey = quic-retry-key-v1-new () in
        let scratch = wrapping-add-i64 buf 1024 in
        let ok1 = quic-retry-tag-compute-raw rkey odcid buf hlen scratch (wrapping-add-i64 buf (u32-to-i64 hlen)) in
        let ok2 = quic-retry-tag-verify-raw rkey odcid buf (wrapping-add-u32 hlen 16) scratch in
        let dec = quic-decode-retry-header buf (wrapping-add-u32 hlen 16) in
        -- Flip one bit in tag -> must fail verification!
        let last-p = wrapping-add-i64 buf (u32-to-i64 (wrapping-add-u32 hlen 15)) in
        let _flip = raw-store-u8 last-p (bit-xor-u8 (raw-load-u8 last-p) 1) in
        let bad = quic-retry-tag-verify-raw rkey odcid buf (wrapping-add-u32 hlen 16) scratch in
        let _d = tls-aead-key-destroy rkey in
        if not ok1 then 65
        else if not ok2 then 66
        else if not (quic-packet-header-ok dec) || ne-u32 dec.qph-token-len 5 then 67
        else if bad then 68
        else 0

  -- RFC 9001 Appendix A.5: ChaCha20-Poly1305 Short Header (PN = 654360564, 21B wire match!)
  test-a5 :: I64 ->{IO, MVar} Int
  test-a5 buf =
    let secret = hex-must "9ac312a7f877468ebe69422748ad00a15443f18203a07d6060f688f30f21632b" in
    let key = quic-crypto-key-from-secret TlsChacha20Poly1305Sha256 secret in
    if not (quic-crypto-key-is-valid key) then 401
    else
      let pn = (654360564 :: U64) in
      let hdr-len = quic-encode-short-header buf 2048 (conn-id-empty ()) 0 0 pn 3 in
      let exp-unprot = hex-must "4200bff4" in
      let flen = quic-frame-encode-ping buf hdr-len 2048 in
      let exp-wire = hex-must "4cfe4189655e5cd55c41f69080575d7999c25a5bfb" in
      let rc = (if ne-u32 hdr-len 4 then 402
        else if not (dma-eq-bytes buf exp-unprot 0 4) then 403
        else if ne-u32 flen 1 then 404
        else
          let wire-len = quic-packet-protect-in-place key buf 1 hdr-len flen pn in
          if ne-u32 wire-len 21 then 405
          else if not (dma-eq-bytes buf exp-wire 0 21) then 406
          else
            -- Unprotect with largest-pn = 654360563 -> reconstructs exact PN = 654360564 + PING frame!
            let dec = quic-packet-unprotect-in-place key buf wire-len 0 654360563 in
            if not (quic-packet-header-ok dec) then 407
            else if ne-u64 dec.qph-full-pn pn then 408
            else if ne-u32 dec.qph-payload-len 1 then 409
            else
              let fr = quic-frame-decode-raw (wrapping-add-i64 buf (u32-to-i64 dec.qph-hdr-len)) 0 dec.qph-payload-len in
              if not (quic-frame-ok fr) || ne-u64 fr.qf-type frame-type-ping then 410
              else 0) in
      let _ = quic-crypto-key-destroy key in
      rc

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 8192 4096 in
    if eq-u64 dma 0 then 99
    else
      let buf = wrapping-u64-to-i64 dma in
      let r2 = test-a2 buf in
      let r3 = if r2 == 0 then test-a3 buf else r2 in
      let r4 = if r3 == 0 then test-a4 buf else r3 in
      let r5 = if r4 == 0 then test-a5 buf else r4 in
      let _ = engine-dma-free dma 8192 in
      r5
"#;
    build_and_run_leak_checked("rfc9001-appendix-a", src, 0);
}

// =========================================================================
// 4. All 22+ QUIC frame types + RFC 9221 DATAGRAM round-trips, PN window
//    reconstruction edge cases, and core-routable CID codec
// =========================================================================

#[test]
fn all_quic_frames_pn_decode_and_core_cid_roundtrips() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Machine
  import Engine.Types
  import Net.Quic.Types
  import Net.Quic.Varint
  import Net.Quic.Frame
  import Net.Quic.Packet
  import Prim.Atomic

  fill-seq :: I64 -> U32 -> U32 -> U8 ->{MVar} Unit
  fill-seq addr i n seed =
    if ge-u32 i n then ()
    else
      let _ = raw-store-u8 (wrapping-add-i64 addr (u32-to-i64 i)) (wrapping-add-u8 (wrapping-u32-to-u8 i) seed) in
      fill-seq addr (wrapping-add-u32 i 1) n seed

  eq-raw :: I64 -> I64 -> U32 -> U32 ->{MVar} Bool
  eq-raw a b i n =
    if ge-u32 i n then True
    else if eq-u8 (raw-load-u8 (wrapping-add-i64 a (u32-to-i64 i))) (raw-load-u8 (wrapping-add-i64 b (u32-to-i64 i))) then
      eq-raw a b (wrapping-add-u32 i 1) n
    else False

  test-types-and-cid :: I64 ->{MVar} Int
  test-types-and-cid buf =
    -- Versions & v1/v2 long-header packet-type bits
    if not (quic-version-is-v1 (quic-version-v1 ())) || not (quic-version-is-v2 (quic-version-v2 ())) then 1
    else if ne-u8 (packet-type-to-long-bits (quic-version-v1 ()) (packet-type-initial ())) 0 then 2
    else if ne-u8 (packet-type-to-long-bits (quic-version-v2 ()) (packet-type-initial ())) 1 then 3
    else if not (packet-type-eq (packet-type-from-long-bits (quic-version-v2 ()) 0) (packet-type-retry ())) then 4
    -- StreamId client/server bidi/uni helpers (RFC 9000 §2.1)
    else
      let s-cb = stream-id-make 5 False False in
      let s-sb = stream-id-make 5 True False in
      let s-cu = stream-id-make 5 False True in
      let s-su = stream-id-make 5 True True in
      if ne-u64 (stream-id-value s-cb) 20 || not (stream-id-is-client s-cb) || not (stream-id-is-bidi s-cb) then 5
      else if ne-u64 (stream-id-value s-sb) 21 || not (stream-id-is-server s-sb) || not (stream-id-is-bidi s-sb) then 6
      else if ne-u64 (stream-id-value s-cu) 22 || not (stream-id-is-client s-cu) || not (stream-id-is-uni s-cu) then 7
      else if ne-u64 (stream-id-value s-su) 23 || not (stream-id-is-server s-su) || not (stream-id-is-uni s-su) then 8
      else if ne-u64 (stream-id-seq s-su) 5 then 9
      -- Crypto transport error code (0x0100 | alert)
      else
        let ce = quic-transport-error-crypto 42 in
        if not (quic-transport-error-is-crypto ce) || ne-u8 (quic-transport-error-alert ce) 42 then 10
        else
          -- Core-routable CID:
          -- (a) Cleartext BPF steering mode (mask-key == 0): packet[1..2] == be16(core_id)
          let cid-bpf = quic-cid-encode-core (core-id 0x1234) 987654321 0 12 in
          let hlen = quic-encode-short-header buf 64 cid-bpf 0 0 1 1 in
          let b1 = raw-load-u8 (wrapping-add-i64 buf 1) in
          let b2 = raw-load-u8 (wrapping-add-i64 buf 2) in
          let dec-bpf = quic-cid-decode-core cid-bpf 0 in
          -- (b) Obfuscated mode (mask-key != 0): recovers exact core_id AND hides core_id across nonces
          let mkey = (0xdeadbeefcafebabe :: U64) in
          let cid-obf1 = quic-cid-encode-core (core-id 42) 111111 mkey 16 in
          let cid-obf2 = quic-cid-encode-core (core-id 42) 222222 mkey 16 in
          let dec-obf1 = quic-cid-decode-core cid-obf1 mkey in
          let dec-obf2 = quic-cid-decode-core cid-obf2 mkey in
          let same-prefix = (eq-u8 (conn-id-get-byte cid-obf1 0) (conn-id-get-byte cid-obf2 0) && eq-u8 (conn-id-get-byte cid-obf1 1) (conn-id-get-byte cid-obf2 1)) in
          if ne-u32 hlen 14 then 11
          else if ne-u8 b1 0x12 || ne-u8 b2 0x34 then 12
          else if not (core-id-eq dec-bpf (core-id 0x1234)) then 13
          else if ne-u8 (conn-id-len cid-obf1) 16 then 14
          else if not (core-id-eq dec-obf1 (core-id 42)) || not (core-id-eq dec-obf2 (core-id 42)) then 15
          else if same-prefix then 16
          else 0

  test-pn-window :: Unit -> Int
  test-pn-window _ =
    -- RFC 9000 Appendix A.3 exact example
    if ne-u64 (quic-pn-decode 0xa82f30ea 0x9b32 16) 0xa82f9b32 then 21
    -- Wrap-up across 16-bit boundary
    else if ne-u64 (quic-pn-decode 0x00fffe 0x0002 16) 0x010002 then 22
    -- Wrap-down across 16-bit boundary (reordered packet)
    else if ne-u64 (quic-pn-decode 0x010001 0xfffe 16) 0x00fffe then 23
    -- First packet (largest-pn = quic-pn-none)
    else if ne-u64 (quic-pn-decode quic-pn-none 0x0000 8) 0 then 24
    else if ne-u64 (quic-pn-decode quic-pn-none 0x0002 32) 2 then 25
    -- RFC 9000 Appendix A.2 encode-len examples
    else if ne-u32 (quic-pn-encode-len 0xac5c02 0xabe8b3) 2 then 26
    else if ne-u32 (quic-pn-encode-len 0xace8fe 0xabe8bc) 3 then 27
    else if ne-u32 (quic-pn-encode-len 5 quic-pn-none) 1 then 28
    else 0

  test-all-frames :: I64 ->{MVar} Int
  test-all-frames buf =
    let src = wrapping-add-i64 buf 2048 in
    let _ = fill-seq src 0 64 0x41 in
    let cap = (1024 :: U32) in
    -- 1. PADDING (0x00) & PING (0x01)
    let w-pad = quic-frame-encode-padding buf 0 cap 4 in
    let w-ping = quic-frame-encode-ping buf 4 cap in
    let f-pad = quic-frame-decode-raw buf 0 5 in
    let skip-p = quic-frame-skip-padding-raw buf 0 5 in
    let f-ping = quic-frame-decode-raw buf 4 5 in
    if ne-u32 w-pad 4 || ne-u32 w-ping 1 || not (quic-frame-ok f-pad) || ne-u64 f-pad.qf-type frame-type-padding || ne-u32 skip-p 4 then 31
    else if not (quic-frame-ok f-ping) || ne-u64 f-ping.qf-type frame-type-ping || ne-u32 f-ping.qf-consumed 1 then 32
    else
      -- 2. ACK (0x02) with 2 additional ranges
      let a0 = quic-frame-encode-ack-header buf 0 cap False 100 15 2 3 in
      let a1 = quic-ack-range-encode-raw buf a0 cap 4 2 in
      let a2 = quic-ack-range-encode-raw buf (wrapping-add-u32 a0 a1) cap 1 5 in
      let atot = wrapping-add-u32 (wrapping-add-u32 a0 a1) a2 in
      let f-ack = quic-frame-decode-raw buf 0 atot in
      let ar1 = quic-ack-range-decode-raw buf f-ack.qf-data-off atot in
      let ar2 = quic-ack-range-decode-raw buf (wrapping-add-u32 f-ack.qf-data-off ar1.ack-consumed) atot in
      if not (quic-frame-ok f-ack) || ne-u64 f-ack.qf-type frame-type-ack || ne-u64 f-ack.qf-f0 100 || ne-u64 f-ack.qf-f1 15 || ne-u64 f-ack.qf-f2 2 || ne-u64 f-ack.qf-f3 3 || ne-u32 f-ack.qf-consumed atot then 33
      else if not (quic-ack-range-ok ar1) || ne-u64 ar1.ack-gap 4 || ne-u64 ar1.ack-range-len 2 then 34
      else if not (quic-ack-range-ok ar2) || ne-u64 ar2.ack-gap 1 || ne-u64 ar2.ack-range-len 5 then 35
      else
        -- 3. ACK_ECN (0x03) with 1 additional range + ECN counts
        let e0 = quic-frame-encode-ack-header buf 0 cap True 500 20 1 10 in
        let e1 = quic-ack-range-encode-raw buf e0 cap 2 7 in
        let e2 = quic-frame-encode-ack-ecn-tail buf (wrapping-add-u32 e0 e1) cap 11 22 33 in
        let etot = wrapping-add-u32 (wrapping-add-u32 e0 e1) e2 in
        let f-ecn = quic-frame-decode-raw buf 0 etot in
        if not (quic-frame-ok f-ecn) || ne-u64 f-ecn.qf-type frame-type-ack-ecn || ne-u64 f-ecn.qf-f4 11 || ne-u64 f-ecn.qf-f5 22 || ne-u64 f-ecn.qf-f6 33 || ne-u32 f-ecn.qf-consumed etot then 36
        else
          -- 4. RESET_STREAM (0x04) & 5. STOP_SENDING (0x05)
          let w-rs = quic-frame-encode-reset-stream buf 0 cap (stream-id-of 4) 0x100 8192 in
          let f-rs = quic-frame-decode-raw buf 0 w-rs in
          let w-ss = quic-frame-encode-stop-sending buf 0 cap (stream-id-of 8) 0x200 in
          let f-ss = quic-frame-decode-raw buf 0 w-ss in
          if not (quic-frame-ok f-rs) || ne-u64 f-rs.qf-type frame-type-reset-stream || ne-u64 f-rs.qf-f0 4 || ne-u64 f-rs.qf-f1 0x100 || ne-u64 f-rs.qf-f2 8192 then 37
          else if not (quic-frame-ok f-ss) || ne-u64 f-ss.qf-type frame-type-stop-sending || ne-u64 f-ss.qf-f0 8 || ne-u64 f-ss.qf-f1 0x200 then 38
          else
            -- 6. CRYPTO (0x06) & 7. NEW_TOKEN (0x07)
            let w-cr = quic-frame-encode-crypto buf 0 cap 128 src 24 in
            let f-cr = quic-frame-decode-raw buf 0 w-cr in
            if not (quic-frame-ok f-cr) || ne-u64 f-cr.qf-type frame-type-crypto || ne-u64 f-cr.qf-f0 128 || ne-u32 f-cr.qf-data-len 24 || not (eq-raw (wrapping-add-i64 buf (u32-to-i64 f-cr.qf-data-off)) src 0 24) then 39
            else
              let w-nt = quic-frame-encode-new-token buf 0 cap src 16 in
              let f-nt = quic-frame-decode-raw buf 0 w-nt in
              if not (quic-frame-ok f-nt) || ne-u64 f-nt.qf-type frame-type-new-token || ne-u32 f-nt.qf-data-len 16 || not (eq-raw (wrapping-add-i64 buf (u32-to-i64 f-nt.qf-data-off)) src 0 16) then 40
              else
                -- 8. STREAM (0x08..0x0f): OFF+LEN+FIN (0x0f) AND no-OFF no-LEN no-FIN (0x08)
                let w-s1 = quic-frame-encode-stream buf 0 cap (stream-id-of 12) 4096 True True src 32 in
                let f-s1 = quic-frame-decode-raw buf 0 w-s1 in
                if not (quic-frame-ok f-s1) || ne-u64 f-s1.qf-type 0x0f || ne-u64 f-s1.qf-f0 12 || ne-u64 f-s1.qf-f1 4096 || ne-u32 f-s1.qf-data-len 32 || ne-u8 f-s1.qf-flags 7 || not (eq-raw (wrapping-add-i64 buf (u32-to-i64 f-s1.qf-data-off)) src 0 32) then 41
                else
                  let w-s2 = quic-frame-encode-stream buf 0 cap (stream-id-of 16) 0 False False src 19 in
                  let f-s2 = quic-frame-decode-raw buf 0 w-s2 in
                  if not (quic-frame-ok f-s2) || ne-u64 f-s2.qf-type 0x08 || ne-u64 f-s2.qf-f0 16 || ne-u64 f-s2.qf-f1 0 || ne-u32 f-s2.qf-data-len 19 || ne-u8 f-s2.qf-flags 0 then 42
                  else
                    -- 9..16 Flow-control & blocked frames (0x10..0x17)
                    let w10 = quic-frame-encode-max-data buf 0 cap 1048576 in
                    let f10 = quic-frame-decode-raw buf 0 w10 in
                    let w11 = quic-frame-encode-max-stream-data buf 0 cap (stream-id-of 4) 65536 in
                    let f11 = quic-frame-decode-raw buf 0 w11 in
                    let w12 = quic-frame-encode-max-streams buf 0 cap False 128 in
                    let f12 = quic-frame-decode-raw buf 0 w12 in
                    let w13 = quic-frame-encode-max-streams buf 0 cap True 64 in
                    let f13 = quic-frame-decode-raw buf 0 w13 in
                    let w14 = quic-frame-encode-data-blocked buf 0 cap 99999 in
                    let f14 = quic-frame-decode-raw buf 0 w14 in
                    let w15 = quic-frame-encode-stream-data-blocked buf 0 cap (stream-id-of 8) 32768 in
                    let f15 = quic-frame-decode-raw buf 0 w15 in
                    let w16 = quic-frame-encode-streams-blocked buf 0 cap False 50 in
                    let f16 = quic-frame-decode-raw buf 0 w16 in
                    let w17 = quic-frame-encode-streams-blocked buf 0 cap True 25 in
                    let f17 = quic-frame-decode-raw buf 0 w17 in
                    if not (quic-frame-ok f10) || ne-u64 f10.qf-f0 1048576 then 43
                    else if not (quic-frame-ok f11) || ne-u64 f11.qf-f0 4 || ne-u64 f11.qf-f1 65536 then 44
                    else if not (quic-frame-ok f12) || ne-u64 f12.qf-type 0x12 || ne-u64 f12.qf-f0 128 || ne-u8 f12.qf-flags 0 then 45
                    else if not (quic-frame-ok f13) || ne-u64 f13.qf-type 0x13 || ne-u64 f13.qf-f0 64 || ne-u8 f13.qf-flags 1 then 46
                    else if not (quic-frame-ok f14) || ne-u64 f14.qf-f0 99999 then 47
                    else if not (quic-frame-ok f15) || ne-u64 f15.qf-f0 8 || ne-u64 f15.qf-f1 32768 then 48
                    else if not (quic-frame-ok f16) || ne-u64 f16.qf-type 0x16 || ne-u64 f16.qf-f0 50 then 49
                    else if not (quic-frame-ok f17) || ne-u64 f17.qf-type 0x17 || ne-u64 f17.qf-f0 25 then 50
                    else
                      -- 17. NEW_CONNECTION_ID (0x18) & 18. RETIRE_CONNECTION_ID (0x19)
                      let ncid = quic-cid-encode-core (core-id 3) 55555 0 8 in
                      let w18 = quic-frame-encode-new-connection-id buf 0 cap 7 2 ncid src in
                      let f18 = quic-frame-decode-raw buf 0 w18 in
                      let dec-cid = conn-id-read-raw (wrapping-add-i64 buf (u32-to-i64 f18.qf-data-off)) (wrapping-u32-to-u8 f18.qf-data-len) in
                      let tok-ok = eq-raw (wrapping-add-i64 buf (u32-to-i64 f18.qf-aux-off)) src 0 16 in
                      if not (quic-frame-ok f18) || ne-u64 f18.qf-f0 7 || ne-u64 f18.qf-f1 2 || not (conn-id-eq dec-cid ncid) || not tok-ok then 51
                      else
                        let w19 = quic-frame-encode-retire-connection-id buf 0 cap 6 in
                        let f19 = quic-frame-decode-raw buf 0 w19 in
                        if not (quic-frame-ok f19) || ne-u64 f19.qf-f0 6 then 52
                        else
                          -- 19. PATH_CHALLENGE (0x1a) & 20. PATH_RESPONSE (0x1b)
                          let w1a = quic-frame-encode-path-challenge buf 0 cap 0x0102030405060708 in
                          let f1a = quic-frame-decode-raw buf 0 w1a in
                          let w1b = quic-frame-encode-path-response buf 0 cap 0xfedcba9876543210 in
                          let f1b = quic-frame-decode-raw buf 0 w1b in
                          if not (quic-frame-ok f1a) || ne-u64 f1a.qf-f0 0x0102030405060708 || ne-u32 f1a.qf-consumed 9 then 53
                          else if not (quic-frame-ok f1b) || ne-u64 f1b.qf-f0 0xfedcba9876543210 || ne-u32 f1b.qf-consumed 9 then 54
                          else
                            -- 21. CONNECTION_CLOSE transport (0x1c) & app (0x1d) & 22. HANDSHAKE_DONE (0x1e)
                            let w1c = quic-frame-encode-connection-close-transport buf 0 cap (quic-transport-error-of quic-err-protocol-violation) frame-type-ack src 10 in
                            let f1c = quic-frame-decode-raw buf 0 w1c in
                            if not (quic-frame-ok f1c) || ne-u64 f1c.qf-f0 quic-err-protocol-violation || ne-u64 f1c.qf-f1 frame-type-ack || ne-u32 f1c.qf-data-len 10 || ne-u8 f1c.qf-flags 0 || not (eq-raw (wrapping-add-i64 buf (u32-to-i64 f1c.qf-data-off)) src 0 10) then 55
                            else
                              let w1d = quic-frame-encode-connection-close-app buf 0 cap 0x0100 src 12 in
                              let f1d = quic-frame-decode-raw buf 0 w1d in
                              if not (quic-frame-ok f1d) || ne-u64 f1d.qf-f0 0x0100 || ne-u32 f1d.qf-data-len 12 || ne-u8 f1d.qf-flags 1 || not (eq-raw (wrapping-add-i64 buf (u32-to-i64 f1d.qf-data-off)) src 0 12) then 56
                              else
                                let w1e = quic-frame-encode-handshake-done buf 0 cap in
                                let f1e = quic-frame-decode-raw buf 0 w1e in
                                if not (quic-frame-ok f1e) || ne-u64 f1e.qf-type frame-type-handshake-done || ne-u32 f1e.qf-consumed 1 then 57
                                else
                                  -- 23. RFC 9221 DATAGRAM without length (0x30) & with length (0x31)
                                  let w30 = quic-frame-encode-datagram buf 0 cap False src 27 in
                                  let f30 = quic-frame-decode-raw buf 0 w30 in
                                  if not (quic-frame-ok f30) || ne-u64 f30.qf-type frame-type-datagram || ne-u32 f30.qf-data-len 27 || ne-u8 f30.qf-flags 0 || not (eq-raw (wrapping-add-i64 buf (u32-to-i64 f30.qf-data-off)) src 0 27) then 58
                                  else
                                    let w31 = quic-frame-encode-datagram buf 0 cap True src 33 in
                                    let f31 = quic-frame-decode-raw buf 0 w31 in
                                    if not (quic-frame-ok f31) || ne-u64 f31.qf-type frame-type-datagram-len || ne-u32 f31.qf-data-len 33 || ne-u8 f31.qf-flags 1 || not (eq-raw (wrapping-add-i64 buf (u32-to-i64 f31.qf-data-off)) src 0 33) then 59
                                    else 0

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 8192 4096 in
    if eq-u64 dma 0 then 99
    else
      let buf = wrapping-u64-to-i64 dma in
      let r1 = test-types-and-cid buf in
      let r2 = if r1 == 0 then test-pn-window () else r1 in
      let r3 = if r2 == 0 then test-all-frames buf else r2 in
      let _ = engine-dma-free dma 8192 in
      r3
"#;
    build_and_run_leak_checked("frames-pn-cid", src, 0);
}

// =========================================================================
// 5. Steady-state 50,000 in-place 1-RTT packet protect+unprotect loop:
//    allocs == 0, atomic_rc == 0, CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1 (leaked=0)
// =========================================================================

#[test]
fn steady_state_50000_1rtt_packets_zero_allocs_zero_atomic_rc() {
    let src = r#"
module Main where
  import Crypto.Tls
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Engine.Types
  import Net.Quic.Types
  import Net.Quic.Frame
  import Net.Quic.Packet
  import Prim.Atomic

  #[no-alloc]
  write-pat :: I64 -> U32 -> U32 -> U8 ->{MVar} Unit
  write-pat addr i len seed =
    if ge-u32 i len then ()
    else
      let _ = raw-store-u8 (wrapping-add-i64 addr (u32-to-i64 i)) (wrapping-add-u8 (wrapping-u32-to-u8 i) seed) in
      write-pat addr (wrapping-add-u32 i 1) len seed

  #[no-alloc]
  verify-pat :: I64 -> U32 -> U32 -> U8 ->{MVar} Bool
  verify-pat addr i len seed =
    if ge-u32 i len then eq-u32 0 0
    else
      let got = raw-load-u8 (wrapping-add-i64 addr (u32-to-i64 i)) in
      let exp = wrapping-add-u8 (wrapping-u32-to-u8 i) seed in
      if eq-u8 got exp then verify-pat addr (wrapping-add-u32 i 1) len seed
      else eq-u32 0 1

  #[no-alloc]
  hot-loop :: QuicCryptoKey -> ConnId -> I64 -> I64 -> U32 -> U32 -> U64 ->{MVar} U32
  hot-loop key cid pkt-addr src-addr i total largest-pn =
    if ge-u32 i total then total
    else
      let pn = u32-to-u64 i in
      let pn-len = quic-pn-encode-len pn largest-pn in
      let dcid-len = u8-to-u32 (conn-id-len cid) in
      let pn-off = wrapping-add-u32 1 dcid-len in
      let hdr-len = quic-encode-short-header pkt-addr 2048 cid 0 (wrapping-u32-to-u8 (bit-and-u32 i 1)) pn pn-len in
      let seed = wrapping-u32-to-u8 i in
      let _p = write-pat src-addr 0 64 seed in
      let flen = quic-frame-encode-stream pkt-addr hdr-len 2000 (stream-id-of 4) (wrapping-shl-u64 pn 6) (eq-u32 0 0) (eq-u32 0 1) src-addr 64 in
      let wire-len = quic-packet-protect-in-place key pkt-addr pn-off hdr-len flen pn in
      if eq-u32 wire-len 0 then 0
      else
        let dec = quic-packet-unprotect-in-place key pkt-addr wire-len dcid-len largest-pn in
        if eq-u8 dec.qph-ok 0 then 0
        else if ne-u64 dec.qph-full-pn pn then 0
        else if ne-u32 dec.qph-payload-len flen then 0
        else
          let body-addr = wrapping-add-i64 pkt-addr (u32-to-i64 dec.qph-hdr-len) in
          let fr = quic-frame-decode-raw body-addr 0 dec.qph-payload-len in
          if eq-u8 fr.qf-ok 0 then 0
          else if ne-u32 fr.qf-data-len 64 then 0
          else if verify-pat (wrapping-add-i64 body-addr (u32-to-i64 fr.qf-data-off)) 0 64 seed then
            hot-loop key cid pkt-addr src-addr (wrapping-add-u32 i 1) total pn
          else 0

  run-suite :: TlsCipherSuite -> Bytes -> Bytes -> Bytes -> I64 -> I64 -> U32 ->{IO, MVar} Int
  run-suite suite k iv hp pkt-addr src-addr iters =
    let key = quic-crypto-key-new suite k iv hp in
    if not (quic-crypto-key-is-valid key) then 10
    else
      let cid = quic-cid-encode-core (core-id 5) 424242 0x123456789abcdef0 8 in
      let _ = engine-thread-counters-reset () in
      let done = hot-loop key cid pkt-addr src-addr 0 iters quic-pn-none in
      let allocs = engine-thread-alloc-count () in
      let rcs = engine-thread-atomic-rc-count () in
      let _ = quic-crypto-key-destroy key in
      if ne-u32 done iters then 11
      else if ne-u64 allocs 0 then 12
      else if ne-u64 rcs 0 then 13
      else 0

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 8192 4096 in
    if eq-u64 dma 0 then 99
    else
      let pkt-addr = wrapping-u64-to-i64 dma in
      let src-addr = wrapping-add-i64 pkt-addr 4096 in
      let k16 = bytes-from-u8s [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16] in
      let hp16 = bytes-from-u8s [17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32] in
      let k32 = bytes-from-u8s [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32] in
      let hp32 = bytes-from-u8s [33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64] in
      let iv12 = bytes-from-u8s [0x10,0x20,0x30,0x40,0x50,0x60,0x70,0x80,0x90,0xa0,0xb0,0xc0] in
      -- 25,000 AES-128-GCM + 25,000 ChaCha20-Poly1305 = 50,000 steady-state 1-RTT packets!
      let r1 = run-suite TlsAes128GcmSha256 k16 iv12 hp16 pkt-addr src-addr 25000 in
      let r2 = run-suite TlsChacha20Poly1305Sha256 k32 iv12 hp32 pkt-addr src-addr 25000 in
      let _ = engine-dma-free dma 8192 in
      if r1 /= 0 then r1
      else if r2 /= 0 then r2 + 20
      else 0
"#;
    build_and_run_leak_checked("hot-50000-zero-alloc", src, 0);
}
