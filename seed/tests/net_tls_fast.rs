//! Integration, formal-verification (M6 `Tls13Machine`), Hegel simulation,
//! differential AEAD parity, Linux kTLS ABI, RFC 9001 QUIC-TLS test vector,
//! 0-RTT anti-replay Bloom filter, and 5,000-record zero-allocation hot-loop
//! tests for `Net.Tls.{Fast,Ktls,QuicBridge,Fast.Spec}`.
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
        "cinch-net-tls-fast-{tag}-{}-{n}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create temp dir");
    dir
}

fn build_and_run_leak_checked(tag: &str, src: &str, expected_code: i32) -> (String, String) {
    let dir = fresh_dir(tag);
    std::fs::write(dir.join("Main.cinch"), src).expect("write Main.cinch");
    let exe = dir.join(format!("tls-fast-{tag}"));
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
// 1. Stdlib manifest registration
// =========================================================================

#[test]
fn net_tls_fast_modules_registered_in_stdlib_package_manifest() {
    let pkg = Package::parse(STDLIB_PKG).expect("parse seed/prelude/.cinchpkg");
    for (name, rel_path) in [
        ("Net.Tls.Fast", "Net/Tls/Fast.cinch"),
        ("Net.Tls.Ktls", "Net/Tls/Ktls.cinch"),
        ("Net.Tls.QuicBridge", "Net/Tls/QuicBridge.cinch"),
        ("Net.Tls.Fast.Spec", "Net/Tls/Fast/Spec.cinch"),
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
}

// =========================================================================
// 2. Formal Model M6 (`Tls13Machine`): capped=false, 0 violations under
//    CINCH_MODEL_FAIL_ON_CAPPED=1 + Hegel randomized simulation
// =========================================================================

#[test]
fn formal_model_m6_tls13_machine_verifies_uncapped_zero_violations() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("prelude/Net/Tls/Fast/Spec.cinch");
    let spec_str = spec_path.to_string_lossy().into_owned();
    let cinch_bin = env!("CARGO_BIN_EXE_cinch");

    // 1. Check via `common::drive_with_profile_opts` with `model_fail_on_capped = true`
    let dir = fresh_dir("m6-spec-check");
    std::fs::write(
        dir.join("Main.cinch"),
        r#"
module Main where
  import Net.Tls.Fast.Spec

  main :: Int
  main = if tls-safety-ok tls-init then 0 else 1
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
    .expect("Net.Tls.Fast.Spec must check cleanly with model_fail_on_capped=true");

    // 2. CLI `cinch model inspect <Spec.cinch> Tls13Machine --json` with CINCH_MODEL_FAIL_ON_CAPPED=1
    let inspect_out = Command::new(cinch_bin)
        .args(["model", "inspect", &spec_str, "Tls13Machine", "--json"])
        .env("CINCH_MODEL_FAIL_ON_CAPPED", "1")
        .output()
        .expect("run cinch model inspect Tls13Machine");
    let stdout = String::from_utf8_lossy(&inspect_out.stdout);
    let stderr = String::from_utf8_lossy(&inspect_out.stderr);
    assert!(
        inspect_out.status.success(),
        "M6 Tls13Machine failed under CINCH_MODEL_FAIL_ON_CAPPED=1:\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    let json: serde_json::Value =
        serde_json::from_str(&stdout).unwrap_or_else(|e| panic!("invalid JSON: {e}\n{stdout}"));
    let machines = json
        .as_array()
        .expect("top-level machines array in model inspect JSON");
    let m6 = machines
        .iter()
        .find(|m| m.get("name").and_then(|v| v.as_str()) == Some("Tls13Machine"))
        .unwrap_or_else(|| panic!("Tls13Machine missing from JSON:\n{stdout}"));
    let capped = m6.get("capped").and_then(|v| v.as_bool()).expect("capped bool");
    let states = m6.get("state_count").and_then(|v| v.as_u64()).expect("state_count");
    let edges = m6.get("edge_count").and_then(|v| v.as_u64()).expect("edge_count");
    let deadlocks = m6
        .get("deadlocks")
        .and_then(|v| v.as_array())
        .expect("deadlocks array");
    assert!(!capped, "M6 Tls13Machine MUST have capped=false; json:\n{stdout}");
    assert!(deadlocks.is_empty(), "M6 Tls13Machine MUST have 0 deadlocks: {deadlocks:?}");
    assert!(states > 10 && states < 65536, "unexpected M6 state count: {states}");

    let contracts = m6
        .get("contracts")
        .and_then(|v| v.as_array())
        .expect("contracts array");
    assert!(
        contracts.len() >= 4,
        "expected >= 4 verified contracts (noAuthBypassOrReplay, alertAbsorbing, tlsDeadlockFree, validHandshakeConnects), got {contracts:?}"
    );
    for c in contracts {
        assert_eq!(
            c.get("passed").and_then(|v| v.as_bool()),
            Some(true),
            "contract failed: {c:?}"
        );
    }

    let refines = m6
        .get("refinements")
        .and_then(|v| v.as_array())
        .expect("refinements array");
    let m6_ref = refines
        .iter()
        .find(|r| {
            r.get("concrete").and_then(|v| v.as_str()) == Some("Tls13Machine")
                && r.get("abstract").and_then(|v| v.as_str()) == Some("AbstractSecureChannel")
        })
        .unwrap_or_else(|| {
            panic!("missing Tls13Machine refines AbstractSecureChannel in JSON:\n{stdout}")
        });
    assert_eq!(
        m6_ref.get("passed").and_then(|v| v.as_bool()),
        Some(true),
        "refinement failed: {m6_ref:?}"
    );

    eprintln!(
        "[M6 Tls13Machine] state_count={states} edge_count={edges} capped={capped} contracts={} refinement=PASS",
        contracts.len()
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn m6_tls13_hegel_simulation_verifies_invariants_and_alert_absorption() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.List
  import Data.Nat
  import Data.Option
  import Effect.Test.Hegel
  import Net.Tls.Fast.Spec

  gen-send-action :: Gen SendAction
  gen-send-action = gen-one-of [gen-pure SendHrr, gen-pure SendFlight]

  gen-fault-kind :: Gen FaultKind
  gen-fault-kind = gen-one-of [gen-pure FaultEarlyAppData, gen-pure FaultBadFin, gen-pure FaultBadRecord]

  tls-actions-with :: Fin 3 -> Fin 3 -> Fin 3 -> List (StepAction TlsState)
  tls-actions-with f0 f1 f2 = [
    step-action "recv" tls-step-recv,
    step-action-gen "send" gen-send-action tls-step-send,
    step-action-gen "zero-rtt" (gen-one-of [gen-pure f0, gen-pure f1, gen-pure f2]) tls-step-zero-rtt,
    step-action-gen "fault" gen-fault-kind tls-step-fault,
    step-action "close" tls-step-close,
    step-action "terminal" tls-step-terminal
  ]

  main :: Int
  main =
    case decide (0 < 3) of
      No _ -> 2
      Yes w0 -> case decide (1 < 3) of
        No _ -> 3
        Yes w1 -> case decide (2 < 3) of
          No _ -> 4
          Yes w2 ->
            let cfg = HegelConfig 400 256 60 (hegel-seed 9001) in
            let acts = tls-actions-with (Fin 0 w0) (Fin 1 w1) (Fin 2 w2) in
            case hegel-check-machine-with cfg 40 tls-init acts (\s -> tls-safety-ok s && (not s.alert-entered || hs-state-eq s.hs-state HsAlert)) of
              Ok _ -> 0
              Err _ -> 1
"#;
    build_and_run_leak_checked("m6-hegel", src, 0);
}

// =========================================================================
// 3. Differential AEAD parity: Net.Tls.Fast vs Crypto.Tls.tls-record-seal/open
//    across AES-128-GCM, AES-256-GCM, and ChaCha20-Poly1305 + tamper zeroing
// =========================================================================

#[test]
fn differential_in_place_aead_matches_crypto_tls_byte_for_byte() {
    let src = r#"
module Main where
  import Crypto.Tls
  import Data.Bool
  import Data.Bytes
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Result
  import Engine.Types
  import Net.Tls.Fast
  import Prim.Atomic

  make-pattern :: Nat -> U8 -> Bytes
  make-pattern n seed =
    let b = bytes-new n in
    let _ = fill-pat b 0 n seed in
    b

  fill-pat :: Bytes -> Nat -> Nat -> U8 -> Unit
  fill-pat b i n seed =
    if i >= n then ()
    else if i < bytes-len b then
      let v = wrapping-add-u8 (wrapping-mul-u8 (wrapping-i64-to-u8 (nat-to-i64 i)) 37) seed in
      let _ = bytes-set-u8 b i v in
      fill-pat b (i + 1) n seed
    else ()

  copy-to-dma :: Bytes -> Nat -> I64 -> Nat ->{MVar} Unit
  copy-to-dma src i dst n =
    if i >= n then ()
    else if i < bytes-len src then
      let _ = raw-store-u8 (wrapping-add-i64 dst (nat-to-i64 i)) (bytes-get-u8 src i) in
      copy-to-dma src (i + 1) dst n
    else ()

  dma-eq-bytes :: I64 -> Bytes -> Nat -> Nat ->{MVar} Bool
  dma-eq-bytes addr expected i n =
    if i >= n then True
    else if i < bytes-len expected then
      let got = raw-load-u8 (wrapping-add-i64 addr (nat-to-i64 i)) in
      let exp = bytes-get-u8 expected i in
      if eq-u8 got exp then dma-eq-bytes addr expected (i + 1) n
      else False
    else False

  dma-all-zero :: I64 -> Nat -> Nat ->{MVar} Bool
  dma-all-zero addr i n =
    if i >= n then True
    else if i <= 9223372036854775807 then
      let b = raw-load-u8 (wrapping-add-i64 addr (nat-to-i64 i)) in
      if eq-u8 b 0 then dma-all-zero addr (i + 1) n
      else False
    else False

  check-one-case :: TlsCipherSuite -> TlsAeadKey -> Bytes -> Bytes -> I64 -> Nat -> Nat -> U8 ->{MVar} Bool
  check-one-case suite fast-key raw-key iv dma-buf seq-nat plen inner-type =
    if seq-nat > 100000 || plen > 16384 then False
    else
      let pt = make-pattern plen (wrapping-i64-to-u8 (nat-to-i64 (seq-nat + plen + 11))) in
      case tls-record-seal suite raw-key iv seq-nat (u8-to-nat inner-type) pt of
        Err _ -> False
        Ok ref-wire ->
          let _ = copy-to-dma pt 0 (wrapping-add-i64 dma-buf 5) plen in
          let seq-u64 = wrapping-i64-to-u64 (nat-to-i64 seq-nat) in
          let plen-u32 = wrapping-i64-to-u32 (nat-to-i64 plen) in
          let wire-len = tls-fast-record-seal-in-place fast-key seq-u64 dma-buf plen-u32 inner-type in
          let expected-wire-len = plen + 22 in
          if u32-to-nat wire-len /= expected-wire-len || bytes-len ref-wire /= expected-wire-len then False
          else if not (dma-eq-bytes dma-buf ref-wire 0 expected-wire-len) then False
          else
            -- Open in-place and verify exact plaintext + inner content type
            let opened = tls-fast-record-open-in-place fast-key seq-u64 dma-buf wire-len in
            if not (tls-fast-record-ok opened) then False
            else if u32-to-nat (tls-fast-record-len opened) /= plen then False
            else if ne-u8 (tls-fast-record-type opened) inner-type then False
            else if not (dma-eq-bytes (tls-fast-record-addr opened) pt 0 plen) then False
            else
              -- Re-seal, flip one bit in the auth tag, verify open FAILS and zeroes plaintext region!
              let w2 = tls-fast-record-seal-in-place fast-key seq-u64 dma-buf plen-u32 inner-type in
              let tag-last = wrapping-add-i64 dma-buf (wrapping-sub-i64 (u32-to-i64 w2) 1) in
              let orig-byte = raw-load-u8 tag-last in
              let _ = raw-store-u8 tag-last (bit-xor-u8 orig-byte 1) in
              let bad = tls-fast-record-open-in-place fast-key seq-u64 dma-buf w2 in
              if tls-fast-record-ok bad then False
              else dma-all-zero (wrapping-add-i64 dma-buf 5) 0 (plen + 1)

  check-lengths :: TlsCipherSuite -> TlsAeadKey -> Bytes -> Bytes -> I64 -> Nat -> List Nat ->{MVar} Bool
  check-lengths suite fast-key raw-key iv dma-buf seq lengths = case lengths of
    Nil -> True
    Cons n rest ->
      if check-one-case suite fast-key raw-key iv dma-buf seq n 23 then
        check-lengths suite fast-key raw-key iv dma-buf (seq + 17) rest
      else False

  check-suite :: TlsCipherSuite -> Nat -> U8 -> I64 ->{IO, MVar} Bool
  check-suite suite klen seed dma-buf =
    let key = make-pattern klen seed in
    let iv = make-pattern 12 (wrapping-add-u8 seed 99) in
    let fk = tls-aead-key-new suite key iv in
    if not (tls-aead-key-is-valid fk) then False
    else
      let ok = check-lengths suite fk key iv dma-buf 0 [0, 1, 15, 16, 17, 63, 64, 65, 255, 1024] in
      let _ = tls-aead-key-destroy fk in
      ok

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 32768 4096 in
    if eq-u64 dma 0 then 99
    else
      let buf = wrapping-u64-to-i64 dma in
      let ok1 = check-suite TlsAes128GcmSha256 16 13 buf in
      let ok2 = check-suite TlsAes256GcmSha384 32 41 buf in
      let ok3 = check-suite TlsChacha20Poly1305Sha256 32 73 buf in
      let _ = engine-dma-free dma 32768 in
      if ok1 && ok2 && ok3 then 0
      else if not ok1 then 1
      else if not ok2 then 2
      else 3
"#;
    build_and_run_leak_checked("diff-aead", src, 0);
}

// =========================================================================
// 4. Linux kTLS ABI layout (40B / 56B) + clean non-Linux IoUnsupported fallback
// =========================================================================

#[test]
fn ktls_abi_layout_and_platform_fallback() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Result
  import Engine.Types
  import Net.Tls.Ktls

  check-ktls :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  check-ktls _ =
    let key16 = bytes-from-u8s [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16] in
    let key32 = bytes-from-u8s [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32] in
    let iv12 = bytes-from-u8s [0xaa,0xbb,0xcc,0xdd, 0x11,0x22,0x33,0x44,0x55,0x66,0x77,0x88] in
    let seq = (0x0102030405060708 :: U64) in
    case ktls-pack-aes128-gcm key16 iv12 seq of
      Err _ -> 1
      Ok b40 ->
        if bytes-len b40 /= 40 then 2
        -- version = 0x0304 LE -> [0x04, 0x03]
        else if ne-u8 (bytes-get-u8 b40 0) 0x04 || ne-u8 (bytes-get-u8 b40 1) 0x03 then 3
        -- cipher_type = 51 LE -> [51, 0]
        else if ne-u8 (bytes-get-u8 b40 2) 51 || ne-u8 (bytes-get-u8 b40 3) 0 then 4
        -- iv[8] at 4..12 == iv12[4..12]
        else if ne-u8 (bytes-get-u8 b40 4) 0x11 || ne-u8 (bytes-get-u8 b40 11) 0x88 then 5
        -- key[16] at 12..28
        else if ne-u8 (bytes-get-u8 b40 12) 1 || ne-u8 (bytes-get-u8 b40 27) 16 then 6
        -- salt[4] at 28..32 == iv12[0..4]
        else if ne-u8 (bytes-get-u8 b40 28) 0xaa || ne-u8 (bytes-get-u8 b40 31) 0xdd then 7
        -- rec_seq[8] BE at 32..40
        else if ne-u8 (bytes-get-u8 b40 32) 0x01 || ne-u8 (bytes-get-u8 b40 39) 0x08 then 8
        else case ktls-pack-aes256-gcm key32 iv12 seq of
          Err _ -> 9
          Ok b56a ->
            if bytes-len b56a /= 56 then 10
            else case ktls-pack-chacha20-poly1305 key32 iv12 seq of
              Err _ -> 11
              Ok b56c ->
                if bytes-len b56c /= 56 then 12
                else case ktls-build-aes128-gcm-info key16 iv12 seq of
                  Err _ -> 13
                  Ok info ->
                    if ne-u16 info.tls128-version 0x0304 || ne-u16 info.tls128-cipher-type 51 then 14
                    else if not ktls-supported then
                      case ktls-enable-tx (fd-of 3) key16 iv12 seq of
                        Err e1 ->
                          if not (io-error-eq e1 IoUnsupported) then 15
                          else case ktls-enable-rx (fd-of 3) key16 iv12 seq of
                            Err e2 -> if io-error-eq e2 IoUnsupported then 0 else 16
                            Ok _ -> 17
                        Ok _ -> 18
                    else 0

  main :: Unit ->{IO r} Int
  main _ = with (io = newIO ()) in check-ktls ()
"#;
    build_and_run_leak_checked("ktls-abi", src, 0);
}

// =========================================================================
// 5. RFC 9001 Appendix A.1/A.2 official QUIC v1 Initial Secrets + Header
//    Protection + Transport Params round-trip + 0-RTT AntiReplayFilter
// =========================================================================

#[test]
fn quic_bridge_rfc9001_vectors_transport_params_and_anti_replay() {
    let src = r#"
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Result
  import Engine.Types
  import Net.Tls.QuicBridge
  import Text.Hex

  hex-must :: String -> Bytes
  hex-must s = case hex-decode s of
    Ok b -> b
    Err _ -> bytes-new 0

  bytes-eq :: Bytes -> Bytes -> Bool
  bytes-eq a b = eq-i64 (bytes-compare a b) 0

  check-anti-replay-hot :: AntiReplayFilter -> U64 -> U64 -> U64 ->{MVar} Bool
  check-anti-replay-hot f i n now-ns =
    if ge-u64 i n then True
    else
      let nonce = wrapping-add-u64 1000000 i in
      if not (anti-replay-check-and-insert f nonce now-ns) then False
      else if anti-replay-check-and-insert f nonce now-ns then False
      else check-anti-replay-hot f (wrapping-add-u64 i 1) n now-ns

  main :: Unit ->{IO, MVar} Int
  main _ =
    -- 1. RFC 9001 Appendix A.1 official QUIC v1 Initial Secret test vector
    let dcid = hex-must "8394c8f03e515708" in
    case quic-derive-initial-secrets dcid of
      (client-in, server-in) ->
        let exp-client-in = hex-must "c00cf151ca5be075ed0ebfb5c80323c42d6b7db67881289af4008f1f6c357aea" in
        let exp-server-in = hex-must "3c199828fd139efd216c155ad844cc81fb82fa8d7446fa7d78be803acdda951b" in
        if not (bytes-eq client-in exp-client-in) then 1
        else if not (bytes-eq server-in exp-server-in) then 2
        else
          let ck = quic-derive-packet-keys client-in in
          let sk = quic-derive-packet-keys server-in in
          let exp-ck = hex-must "1f369613dd76d5467730efcbe3b1a22d" in
          let exp-civ = hex-must "fa044b2f42a3fd3b46fb255c" in
          let exp-chp = hex-must "9f50449e04a0e810283a1e9933adedd2" in
          let exp-sk = hex-must "cf3a5331653c364c88f0f379b6067e37" in
          let exp-siv = hex-must "0ac1493ca1905853b0bba03e" in
          let exp-shp = hex-must "c206b8d9b9f0f37644430b490eeaa314" in
          if not (bytes-eq ck.qpk-key exp-ck) then 3
          else if not (bytes-eq ck.qpk-iv exp-civ) then 4
          else if not (bytes-eq ck.qpk-hp exp-chp) then 5
          else if not (bytes-eq sk.qpk-key exp-sk) then 6
          else if not (bytes-eq sk.qpk-iv exp-siv) then 7
          else if not (bytes-eq sk.qpk-hp exp-shp) then 8
          else
            -- 2. RFC 9001 Appendix A.2 AES-128-ECB Header Protection mask vector
            let sample = hex-must "d1b1c98dd7689fb8ec11d242b123dc9b" in
            let hp-mask = quic-header-protect-mask-aes128 ck.qpk-hp sample in
            let exp-mask = hex-must "437b9aec36" in
            if not (bytes-eq hp-mask exp-mask) then 9
            else
              -- 3. Transport parameters encode / decode round-trip (incl. RFC 9221 datagram)
              let tp = QuicTransportParams {
                tp-max-idle-timeout = 60000,
                tp-max-udp-payload-size = 1472,
                tp-initial-max-data = 4194304,
                tp-initial-max-stream-data-bidi-local = 524288,
                tp-initial-max-stream-data-bidi-remote = 524288,
                tp-initial-max-stream-data-uni = 262144,
                tp-initial-max-streams-bidi = 256,
                tp-initial-max-streams-uni = 64,
                tp-ack-delay-exponent = 4,
                tp-max-ack-delay = 50,
                tp-active-connection-id-limit = 8,
                tp-max-datagram-frame-size = 1200
              } in
              let wire-tp = quic-transport-params-encode tp in
              case quic-transport-params-decode wire-tp of
                Err _ -> 10
                Ok dec ->
                  if ne-u64 dec.tp-max-idle-timeout 60000
                    || ne-u64 dec.tp-max-udp-payload-size 1472
                    || ne-u64 dec.tp-initial-max-data 4194304
                    || ne-u64 dec.tp-initial-max-stream-data-bidi-local 524288
                    || ne-u64 dec.tp-initial-max-stream-data-bidi-remote 524288
                    || ne-u64 dec.tp-initial-max-stream-data-uni 262144
                    || ne-u64 dec.tp-initial-max-streams-bidi 256
                    || ne-u64 dec.tp-initial-max-streams-uni 64
                    || ne-u64 dec.tp-ack-delay-exponent 4
                    || ne-u64 dec.tp-max-ack-delay 50
                    || ne-u64 dec.tp-active-connection-id-limit 8
                    || ne-u64 dec.tp-max-datagram-frame-size 1200
                  then 11
                  else
                    -- 4. 0-RTT AntiReplayFilter: reject duplicates, survive 1 rotation, expire after 2, 0 allocs
                    let ar = anti-replay-new 1024 1000000000 in
                    if not (anti-replay-is-valid ar) then 12
                    else
                      let _ = engine-thread-counters-reset () in
                      let hot-ok = check-anti-replay-hot ar 0 500 1000 in
                      let allocs = engine-thread-alloc-count () in
                      let rcs = engine-thread-atomic-rc-count () in
                      -- After 1 window rotation: nonce 1000000 is in prev-gen -> STILL rejected!
                      let still-rej = not (anti-replay-check-and-insert ar 1000000 1000001500) in
                      -- After 2 full window rotations: both gens expired -> accepted fresh!
                      let fresh-after2 = anti-replay-check-and-insert ar 1000000 4000001500 in
                      let _ = anti-replay-destroy ar in
                      if not hot-ok then 13
                      else if ne-u64 allocs 0 then 14
                      else if ne-u64 rcs 0 then 15
                      else if not still-rej then 16
                      else if not fresh-after2 then 17
                      else 0
"#;
    build_and_run_leak_checked("quic-bridge", src, 0);
}

// =========================================================================
// 6. Steady-state 5,000-record in-place seal+open hot loop:
//    0 heap allocs, 0 atomic RC ops, CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1 (leaked=0)
// =========================================================================

#[test]
fn steady_state_5000_records_zero_heap_allocs_zero_atomic_rc() {
    let src = r#"
module Main where
  import Crypto.Tls
  import Data.Bool
  import Data.Bytes
  import Data.Machine
  import Data.Nat
  import Data.Result
  import Engine.Types
  import Net.Tls.Fast
  import Prim.Atomic

  #[no-alloc]
  write-payload :: I64 -> U32 -> U32 -> U8 ->{MVar} Unit
  write-payload addr i len seed =
    if ge-u32 i len then ()
    else
      let _ = raw-store-u8 (wrapping-add-i64 addr (u32-to-i64 i)) (wrapping-add-u8 (wrapping-u32-to-u8 i) seed) in
      write-payload addr (wrapping-add-u32 i 1) len seed

  #[no-alloc]
  verify-payload :: I64 -> U32 -> U32 -> U8 ->{MVar} Bool
  verify-payload addr i len seed =
    if ge-u32 i len then eq-u32 0 0
    else
      let got = raw-load-u8 (wrapping-add-i64 addr (u32-to-i64 i)) in
      let exp = wrapping-add-u8 (wrapping-u32-to-u8 i) seed in
      if eq-u8 got exp then verify-payload addr (wrapping-add-u32 i 1) len seed
      else eq-u32 0 1

  #[no-alloc]
  hot-loop :: TlsFastConn -> I64 -> U32 -> U32 ->{MVar} U32
  hot-loop conn rec-addr i total =
    if ge-u32 i total then total
    else
      let plen = (256 :: U32) in
      let seed = wrapping-u32-to-u8 i in
      let body-addr = wrapping-add-i64 rec-addr 5 in
      let _ = write-payload body-addr 0 plen seed in
      let wire-len = tls-fast-conn-seal-in-place conn rec-addr plen 23 in
      if ne-u32 wire-len 278 then 0
      else
        let opened = tls-fast-conn-open-in-place conn rec-addr wire-len in
        if not (tls-fast-record-ok opened) then 0
        else if ne-u32 (tls-fast-record-len opened) plen then 0
        else if ne-u8 (tls-fast-record-type opened) 23 then 0
        else if not (verify-payload (tls-fast-record-addr opened) 0 plen seed) then 0
        else hot-loop conn rec-addr (wrapping-add-u32 i 1) total

  run-suite5000 :: TlsCipherSuite -> Bytes -> Bytes -> I64 ->{IO, MVar} Int
  run-suite5000 suite key iv rec-addr =
    let conn = tls-fast-conn-from-keys suite key iv key iv in
    if not (tls-fast-conn-is-valid conn) then 10
    else
      let _ = engine-thread-counters-reset () in
      let done = hot-loop conn rec-addr 0 5000 in
      let allocs = engine-thread-alloc-count () in
      let rcs = engine-thread-atomic-rc-count () in
      let _ = tls-fast-conn-destroy conn in
      if ne-u32 done 5000 then 11
      else if ne-u64 allocs 0 then 12
      else if ne-u64 rcs 0 then 13
      else 0

  main :: Unit ->{IO, MVar} Int
  main _ =
    let dma = engine-dma-alloc 4096 4096 in
    if eq-u64 dma 0 then 99
    else
      let rec-addr = wrapping-u64-to-i64 dma in
      let key16 = bytes-from-u8s [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16] in
      let key32 = bytes-from-u8s [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32] in
      let iv12 = bytes-from-u8s [0x10,0x20,0x30,0x40,0x50,0x60,0x70,0x80,0x90,0xa0,0xb0,0xc0] in
      let r1 = run-suite5000 TlsAes128GcmSha256 key16 iv12 rec-addr in
      let r2 = run-suite5000 TlsChacha20Poly1305Sha256 key32 iv12 rec-addr in
      let _ = engine-dma-free dma 4096 in
      if r1 /= 0 then r1
      else if r2 /= 0 then r2 + 20
      else 0
"#;
    build_and_run_leak_checked("hot-5000-zero-alloc", src, 0);
}
