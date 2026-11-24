//! TLS 1.3 (`Crypto.Tls` and `Effect.Tls`) verification suite:
//! 1. Canonical documentation check (`common::assert_documented`) on both modules.
//! 2. RFC 8446 / RFC 8448 key schedule, Finished MAC, KeyUpdate, TLS-Exporter, and AEAD record
//!    seal/open vectors generated independently by Go 1.24 (`tooling/tls_vectors.go`).
//! 3. Pure in-memory Cinch Client <-> Cinch Server handshake matrix across all 3 RFC 8446 cipher
//!    suites, X25519 + P-256 + HelloRetryRequest, all 3 certificate signature families
//!    (Ed25519, ECDSA-P256 3-cert chain, RSA-PSS-SHA256), ALPN, SNI, KeyUpdate, large
//!    multi-record payloads, 1-byte fragmented delivery, close_notify, and security rejections.
//! 4. Real live loopback TCP interop (`Effect.Net` + `Effect.Tls` + `Effect.Random` + `Effect.Clock`)
//!    against Go 1.24 `crypto/tls` in BOTH directions (Cinch Client <-> Go Server AND
//!    Go Client <-> Cinch Server).
mod common;

use serde_json::Value;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// All non-embedded source-distributed prelude modules transitively required by `Crypto.Tls` and `Effect.Tls`.
const DEPS: &[(&str, &str)] = &[
    ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
    ("Data.Bytes.Utf8.cinch", include_str!("../prelude/Data/Bytes/Utf8.cinch")),
    ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
    ("Data.Unicode.Tables.cinch", include_str!("../prelude/Data/Unicode/Tables.cinch")),
    ("Data.Unicode.Kernel.cinch", include_str!("../prelude/Data/Unicode/Kernel.cinch")),
    ("Data.IpAddr.cinch", include_str!("../prelude/Data/IpAddr.cinch")),
    ("Data.NetIp.cinch", include_str!("../prelude/Data/NetIp.cinch")),
    ("Effect.Stream.cinch", include_str!("../prelude/Effect/Stream.cinch")),
    ("Effect.Net.cinch", include_str!("../prelude/Effect/Net.cinch")),
    ("Effect.Random.cinch", include_str!("../prelude/Effect/Random.cinch")),
    ("Text.Asn1.cinch", include_str!("../prelude/Text/Asn1.cinch")),
    ("Text.Base64.Encoding.cinch", include_str!("../prelude/Text/Base64/Encoding.cinch")),
    ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
    ("Text.Digest.cinch", include_str!("../prelude/Text/Digest.cinch")),
    ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
    ("Text.Md5.cinch", include_str!("../prelude/Text/Md5.cinch")),
    ("Text.Pem.cinch", include_str!("../prelude/Text/Pem.cinch")),
    ("Text.Sha1.cinch", include_str!("../prelude/Text/Sha1.cinch")),
    ("Text.Sha256.cinch", include_str!("../prelude/Text/Sha256.cinch")),
    ("Text.Sha512.cinch", include_str!("../prelude/Text/Sha512.cinch")),
    ("Crypto.ConstantTime.cinch", include_str!("../prelude/Crypto/ConstantTime.cinch")),
    ("Crypto.Hmac.cinch", include_str!("../prelude/Crypto/Hmac.cinch")),
    ("Crypto.Kdf.cinch", include_str!("../prelude/Crypto/Kdf.cinch")),
    ("Crypto.Cipher.cinch", include_str!("../prelude/Crypto/Cipher.cinch")),
    ("Crypto.Aes.cinch", include_str!("../prelude/Crypto/Aes.cinch")),
    ("Crypto.ChaCha20Poly1305.cinch", include_str!("../prelude/Crypto/ChaCha20Poly1305.cinch")),
    ("Crypto.P256.cinch", include_str!("../prelude/Crypto/P256.cinch")),
    ("Crypto.Ecdsa.cinch", include_str!("../prelude/Crypto/Ecdsa.cinch")),
    ("Crypto.Field25519.cinch", include_str!("../prelude/Crypto/Field25519.cinch")),
    ("Crypto.Ed25519.cinch", include_str!("../prelude/Crypto/Ed25519.cinch")),
    ("Crypto.X25519.cinch", include_str!("../prelude/Crypto/X25519.cinch")),
    ("Crypto.Rsa.cinch", include_str!("../prelude/Crypto/Rsa.cinch")),
    ("Crypto.Pkix.cinch", include_str!("../prelude/Crypto/Pkix.cinch")),
    ("Crypto.X509.cinch", include_str!("../prelude/Crypto/X509.cinch")),
    ("Crypto.Tls.cinch", include_str!("../prelude/Crypto/Tls.cinch")),
    ("Effect.Tls.cinch", include_str!("../prelude/Effect/Tls.cinch")),
];

fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/tls-vectors.json")).unwrap()
}

fn lit(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\r' => out.push_str("\\r"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn suite_ctor(name: &str) -> &'static str {
    match name {
        "TLS_AES_128_GCM_SHA256" => "TlsAes128GcmSha256",
        "TLS_AES_256_GCM_SHA384" => "TlsAes256GcmSha384",
        "TLS_CHACHA20_POLY1305_SHA256" => "TlsChacha20Poly1305Sha256",
        other => panic!("unknown suite: {other}"),
    }
}

fn run_cinch(tag: &str, main_src: &str) -> i32 {
    let mut files: Vec<(&str, &str)> = DEPS.to_vec();
    files.push(("Main.cinch", main_src));
    let dir = common::project(tag, &files);
    let code = common::run(&dir, "Main.cinch");
    common::cleanup(&dir);
    code
}

/// 1. Every declaration in `Crypto.Tls` and `Effect.Tls` carries canonical documentation.
#[test]
fn tls_modules_have_canonical_documentation() {
    common::assert_documented("Crypto.Tls", include_str!("../prelude/Crypto/Tls.cinch"));
    common::assert_documented("Effect.Tls", include_str!("../prelude/Effect/Tls.cinch"));
}

/// 2. Verify RFC 8446 / RFC 8448 §3 key schedule (SHA-256 & SHA-384), Finished MAC,
/// KeyUpdate, TLS-Exporter, and AEAD record seal/open + tamper rejection against Go 1.24 vectors.
#[test]
fn tls_rfc8446_key_schedule_and_records_match_vectors() {
    let f = fixture();
    let mut rows: Vec<String> = Vec::new();

    for ks in f["key_schedules"].as_array().unwrap() {
        let suite = suite_ctor(ks["suite"].as_str().unwrap());
        rows.push(format!(
            "check-key-schedule {suite} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
            lit(ks["shared_secret_hex"].as_str().unwrap()),
            lit(ks["ch_sh_hash_hex"].as_str().unwrap()),
            lit(ks["ch_sf_hash_hex"].as_str().unwrap()),
            lit(ks["ch_cf_hash_hex"].as_str().unwrap()),
            lit(ks["early_secret_hex"].as_str().unwrap()),
            lit(ks["derived_early_hex"].as_str().unwrap()),
            lit(ks["handshake_secret_hex"].as_str().unwrap()),
            lit(ks["client_hs_traffic_hex"].as_str().unwrap()),
            lit(ks["server_hs_traffic_hex"].as_str().unwrap()),
            lit(ks["client_hs_key_hex"].as_str().unwrap()),
            lit(ks["client_hs_iv_hex"].as_str().unwrap()),
            lit(ks["server_hs_key_hex"].as_str().unwrap()),
            lit(ks["server_hs_iv_hex"].as_str().unwrap()),
            lit(ks["server_verify_data_hex"].as_str().unwrap()),
            lit(ks["derived_hs_hex"].as_str().unwrap()),
            lit(ks["master_secret_hex"].as_str().unwrap()),
            lit(ks["client_ap_traffic_hex"].as_str().unwrap()),
            lit(ks["server_ap_traffic_hex"].as_str().unwrap()),
            lit(ks["client_ap_key_hex"].as_str().unwrap()),
            lit(ks["client_ap_iv_hex"].as_str().unwrap()),
            lit(ks["client_ap_traffic_next_hex"].as_str().unwrap()),
            lit(ks["exporter_master_hex"].as_str().unwrap()),
            lit(ks["exported_material_hex"].as_str().unwrap()),
            lit(ks["resumption_master_hex"].as_str().unwrap()),
        ));
    }

    for rv in f["records"].as_array().unwrap() {
        let suite = suite_ctor(rv["suite"].as_str().unwrap());
        let seq = rv["seq"].as_u64().unwrap();
        let ctype = rv["content_type"].as_u64().unwrap();
        let pad = rv["pad_zeros"].as_u64().unwrap();
        rows.push(format!(
            "check-record {suite} {} {} {seq} {} {ctype} {} {pad} {}",
            lit(rv["key_hex"].as_str().unwrap()),
            lit(rv["iv_hex"].as_str().unwrap()),
            lit(rv["nonce_hex"].as_str().unwrap()),
            lit(rv["plaintext_hex"].as_str().unwrap()),
            lit(rv["wire_hex"].as_str().unwrap()),
        ));
    }

    rows.push("check-chacha-record-roundtrip".to_string());
    rows.push("check-identifiers-and-alerts".to_string());

    let main_src = format!(
        r#"-- | RFC 8446 key schedule and record AEAD vector verification.
module Main where
  import Crypto.Tls
  import Data.Bool
  import Data.Bytes
  import Data.Int
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Text.Hex
  import Text.Sha256
  import Text.Sha512

  -- | Decode a lowercase hex string to bytes.
  unhex :: String -> Bytes
  unhex s = case hex-decode s of Ok b -> b; Err _ -> bytes-new 0

  -- | Encode bytes to lowercase hex string.
  tohex :: Bytes -> String
  tohex b = hex-encode b

  -- | Unwrap a `Result TlsError Bytes`, yielding empty bytes on error.
  okb :: Result TlsError Bytes -> Bytes
  okb r = case r of Ok b -> b; Err _ -> bytes-new 0

  -- | Clone a byte buffer into fresh storage.
  clone-bytes :: Bytes -> Bytes
  clone-bytes b =
    let n = bytes-len b in
    let out = bytes-new n in
    let u = bytes-copy-range out b 0 0 n in
    out

  -- | Flip one byte in a copy of `b` at index `i`.
  flip-byte :: Bytes -> Nat -> Bytes
  flip-byte b i =
    let out = clone-bytes b in
    let u = if i < bytes-len out then bytes-set out i (if bytes-get out i == 0 then 1 else 0) else () in
    out

  -- | Verify a complete RFC 8446 key schedule derivation chain against Go 1.24 vectors.
  check-key-schedule :: TlsCipherSuite -> String -> String -> String -> String -> String -> String -> String -> String -> String -> String -> String -> String -> String -> String -> String -> String -> String -> String -> String -> String -> String -> String -> String -> String -> Bool
  check-key-schedule suite shared-h chsh-h chsf-h chcf-h want-early want-de want-hs want-chs want-shs want-chsk want-chsi want-shsk want-shsi want-svd want-dhs want-ms want-cap want-sap want-capk want-capi want-capn want-exp want-expm want-res =
    let hlen = case suite of TlsAes256GcmSha384 -> 48; _ -> 32 in
    let zeros = bytes-new hlen in
    let empty-h = case suite of TlsAes256GcmSha384 -> sha384-sum (bytes-new 0); _ -> sha256-sum (bytes-new 0) in
    let early = tls-hkdf-extract suite zeros zeros in
    let de = okb (tls-derive-secret suite early "derived" empty-h) in
    let hs = tls-hkdf-extract suite de (unhex shared-h) in
    let chs = okb (tls-derive-secret suite hs "c hs traffic" (unhex chsh-h)) in
    let shs = okb (tls-derive-secret suite hs "s hs traffic" (unhex chsh-h)) in
    let (chsk, chsi) = (case tls-traffic-keys suite chs of Ok p -> p; Err _ -> (bytes-new 0, bytes-new 0)) in
    let (shsk, shsi) = (case tls-traffic-keys suite shs of Ok p -> p; Err _ -> (bytes-new 0, bytes-new 0)) in
    let svd = okb (tls-finished-verify-data suite shs (unhex chsf-h)) in
    let dhs = okb (tls-derive-secret suite hs "derived" empty-h) in
    let ms = tls-hkdf-extract suite dhs zeros in
    let cap = okb (tls-derive-secret suite ms "c ap traffic" (unhex chsf-h)) in
    let sap = okb (tls-derive-secret suite ms "s ap traffic" (unhex chsf-h)) in
    let (capk, capi) = (case tls-traffic-keys suite cap of Ok p -> p; Err _ -> (bytes-new 0, bytes-new 0)) in
    let capn = okb (tls-update-traffic-secret suite cap) in
    let exp = okb (tls-derive-secret suite ms "exp master" (unhex chsf-h)) in
    let expm = okb (tls-export-keying-material suite exp "EXPORTER-cinch-test" (string-to-bytes "exporter-context") 32) in
    let res = okb (tls-derive-secret suite ms "res master" (unhex chcf-h)) in
    tohex early == want-early
    && tohex de == want-de
    && tohex hs == want-hs
    && tohex chs == want-chs
    && tohex shs == want-shs
    && tohex chsk == want-chsk
    && tohex chsi == want-chsi
    && tohex shsk == want-shsk
    && tohex shsi == want-shsi
    && tohex svd == want-svd
    && tohex dhs == want-dhs
    && tohex ms == want-ms
    && tohex cap == want-cap
    && tohex sap == want-sap
    && tohex capk == want-capk
    && tohex capi == want-capi
    && tohex capn == want-capn
    && tohex exp == want-exp
    && tohex expm == want-expm
    && tohex res == want-res

  -- | Verify record nonce, opening Go-sealed (possibly zero-padded) record, sealing unpadded record, and tamper rejection.
  check-record :: TlsCipherSuite -> String -> String -> Nat -> String -> Nat -> String -> Nat -> String -> Bool
  check-record suite key-h iv-h seq want-nonce want-ct pt-h pad wire-h =
    let key = unhex key-h in
    let iv = unhex iv-h in
    let pt = unhex pt-h in
    let wire = unhex wire-h in
    let ok-nonce = tohex (tls-record-nonce iv seq) == want-nonce in
    let ok-open = (case tls-record-open suite key iv seq wire of
      Ok (ct, got-pt) -> ct == want-ct && tohex got-pt == pt-h
      Err _ -> False) in
    let ok-seal = (case tls-record-seal suite key iv seq want-ct pt of
      Err _ -> False
      Ok sealed ->
        let match-exact = if pad == 0 then tohex sealed == wire-h else True in
        let rt = (case tls-record-open suite key iv seq sealed of
          Ok (ct2, pt2) -> ct2 == want-ct && tohex pt2 == pt-h
          Err _ -> False) in
        match-exact && rt) in
    let ok-tamper = (case tls-record-open suite key iv seq (flip-byte wire 10) of
      Err TlsBadRecordMac -> True
      _ -> False) in
    let ok-wrong-seq = (case tls-record-open suite key iv (seq + 1) wire of
      Err TlsBadRecordMac -> True
      _ -> False) in
    ok-nonce && ok-open && ok-seal && ok-tamper && ok-wrong-seq

  -- | Verify ChaCha20-Poly1305 record seal/open and tamper rejection.
  check-chacha-record-roundtrip :: Bool
  check-chacha-record-roundtrip =
    let key = sha256-sum (string-to-bytes "chacha-key") in
    let iv = bytes-from-u8s [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12] in
    let pt = string-to-bytes "ChaCha20-Poly1305 TLS 1.3 record test" in
    case tls-record-seal TlsChacha20Poly1305Sha256 key iv 77 23 pt of
      Err _ -> False
      Ok wire ->
        let ok1 = (case tls-record-open TlsChacha20Poly1305Sha256 key iv 77 wire of
          Ok (23, got) -> eq-i64 (bytes-compare got pt) 0
          _ -> False) in
        let ok2 = (case tls-record-open TlsChacha20Poly1305Sha256 key iv 77 (flip-byte wire 8) of
          Err TlsBadRecordMac -> True
          _ -> False) in
        ok1 && ok2

  -- | Verify wire ID roundtrips and alert rendering helpers.
  check-identifiers-and-alerts :: Bool
  check-identifiers-and-alerts =
    tls-version-id Tls13 == 772
    && tls-version-name Tls13 == "TLS 1.3"
    && tls-version-equal Tls13 Tls13
    && tls-cipher-suite-id TlsAes128GcmSha256 == 4865
    && tls-cipher-suite-id TlsAes256GcmSha384 == 4866
    && tls-cipher-suite-id TlsChacha20Poly1305Sha256 == 4867
    && tls-cipher-suite-name TlsAes128GcmSha256 == "TLS_AES_128_GCM_SHA256"
    && tls-curve-id CurveX25519 == 29
    && tls-curve-id CurveP256 == 23
    && tls-signature-scheme-id SigEd25519 == 2055
    && tls-signature-scheme-id SigEcdsaSecp256r1Sha256 == 1027
    && tls-signature-scheme-id SigRsaPssRsaeSha256 == 2052
    && tls-alert-code (tls-alert-from-code 20) == 20
    && tls-alert-message AlertBadRecordMac == "bad record MAC"
    && not (tls-error-message TlsBadRecordMac == "")

  -- | Return 200 when all rows pass, or the 0-based index of the first failing row.
  first-fail :: List Bool -> Int -> Int
  first-fail xs i = case xs of
    Nil -> 200
    Cons b rest -> if b then first-fail rest (i + 1) else i

  -- | Entry point.
  main :: Int
  main = first-fail [
    {}
  ] 0
"#,
        rows.join(",\n    ")
    );

    let code = run_cinch("tls_vectors", &main_src);
    assert_eq!(
        code, 200,
        "tls_vectors: exit status {code} is first failing row (200 = all {} rows passed)",
        rows.len()
    );
}

/// 3. Pure in-memory Cinch Client <-> Cinch Server handshake matrix across all 3 cipher suites,
/// X25519 + P-256 + HelloRetryRequest, all 3 cert signature families (Ed25519, ECDSA-P256 3-cert
/// chain, RSA-PSS-SHA256), ALPN, SNI, KeyUpdate, large multi-record transfer, 1-byte fragmented
/// delivery, close_notify, and security rejections.
#[test]
fn tls_in_memory_handshake_matrix_and_security_rejections() {
    let f = fixture();
    let certs = f["certs"].as_array().unwrap();
    let ed = &certs[0];
    let ec = &certs[1];
    let rsa = &certs[2];

    let main_src = format!(
        r#"-- | Pure in-memory TLS 1.3 Client <-> Server handshake matrix and negative security tests.
module Main where
  import Crypto.Tls
  import Crypto.X509
  import Data.Bool
  import Data.Bytes
  import Data.Bytes.Build
  import Data.Int
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Text.Sha256

  -- | Fixture certificates and keys generated by Go 1.24.
  ed-root :: Bytes
  ed-root = string-to-bytes {ed_root}
  ed-chain :: Bytes
  ed-chain = string-to-bytes {ed_chain}
  ed-key :: Bytes
  ed-key = string-to-bytes {ed_key}

  ec-root :: Bytes
  ec-root = string-to-bytes {ec_root}
  ec-chain :: Bytes
  ec-chain = string-to-bytes {ec_chain}
  ec-key :: Bytes
  ec-key = string-to-bytes {ec_key}

  rsa-root :: Bytes
  rsa-root = string-to-bytes {rsa_root}
  rsa-chain :: Bytes
  rsa-chain = string-to-bytes {rsa_chain}
  rsa-key :: Bytes
  rsa-key = string-to-bytes {rsa_key}

  -- | Verification timestamp inside all fixture validity windows.
  valid-now :: I64
  valid-now = 1700003600

  -- | Load a root CA `CertPool` from PEM bytes.
  pool-from-pem :: Bytes -> CertPool
  pool-from-pem pem = case cert-pool-append-pem cert-pool-new pem of (p, _) -> p

  -- | Drive a pure sans-I/O handshake to completion (supports 1-RTT and 2-RTT HelloRetryRequest).
  drive-hs :: TlsClient -> TlsServer -> Bytes -> Nat -> Result TlsError (TlsClient, TlsServer)
  drive-hs cli srv to-srv fuel =
    if fuel == 0 then Err (TlsProtocolError AlertInternalError "handshake loop exceeded fuel")
    else case tls-server-feed srv to-srv of
      Err e -> Err e
      Ok s-out ->
        let srv1 = tls-out-state s-out in
        let to-cli = tls-out-wire s-out in
        if tls-client-handshake-complete cli && tls-server-handshake-complete srv1 then Ok (cli, srv1)
        else case tls-client-feed cli to-cli of
          Err e -> Err e
          Ok c-out ->
            let cli1 = tls-out-state c-out in
            let next-to-srv = tls-out-wire c-out in
            if tls-client-handshake-complete cli1 && tls-server-handshake-complete srv1 then Ok (cli1, srv1)
            else drive-hs cli1 srv1 next-to-srv (if fuel > 0 then fuel - 1 else 0)

  -- | Clone a byte buffer into fresh storage.
  clone-bytes :: Bytes -> Bytes
  clone-bytes b =
    let n = bytes-len b in
    let out = bytes-new n in
    let u = bytes-copy-range out b 0 0 n in
    out

  -- | Feed wire bytes into a client one chunk of size `step` at a time to test arbitrary fragmentation.
  feed-cli-chunked :: TlsClient -> Bytes -> Nat -> Nat -> Bytes -> Bytes -> Result TlsError (TlsClient, Bytes, Bytes, Bool)
  feed-cli-chunked cli wire pos step w-acc a-acc =
    let n = bytes-len wire in
    if pos >= n then Ok (cli, w-acc, a-acc, tls-state-peer-closed (tls-client-state cli))
    else
      let rem = if pos <= n then n - pos else 0 in
      let take = if rem > step then step else rem in
      let chunk = bytes-new take in
      let u = if pos + take <= bytes-len wire then bytes-copy-range chunk wire 0 pos take else () in
      case tls-client-feed cli chunk of
        Err e -> Err e
        Ok out ->
          feed-cli-chunked
            (tls-out-state out)
            wire
            (pos + take)
            step
            (bytes-concat [w-acc, tls-out-wire out])
            (bytes-concat [a-acc, tls-out-data out])

  -- | Full handshake + bidirectional data + KeyUpdate + Exporter + close_notify scenario.
  run-scenario :: Bytes -> Bytes -> Bytes -> String -> List TlsCipherSuite -> List TlsCipherSuite -> List TlsCurve -> List TlsCurve -> List String -> List String -> String -> Nat -> Bool
  run-scenario root-pem chain-pem key-pem sni c-suites s-suites c-curves s-curves c-alpn s-alpn want-alpn chunk-size =
    case tls-load-x509-key-pair chain-pem key-pem of
      Err _ -> False
      Ok cert ->
        let cli-cfg = (tls-config-with-now valid-now
          (tls-config-with-alpn c-alpn
            (tls-config-with-curves c-curves
              (tls-config-with-cipher-suites c-suites
                (tls-config-with-roots (pool-from-pem root-pem)
                  (tls-config-with-server-name sni tls-config-default)))))) in
        let srv-cfg = (tls-config-with-alpn s-alpn
          (tls-config-with-curves s-curves
            (tls-config-with-cipher-suites s-suites
              (tls-config-with-certificate cert tls-config-default)))) in
        let c-ent = sha256-sum (string-to-bytes (string-concat "client-entropy-" sni)) in
        let s-ent = sha256-sum (string-to-bytes (string-concat "server-entropy-" sni)) in
        case tls-client-start cli-cfg c-ent of
          Err _ -> False
          Ok (cli0, ch-wire) -> case tls-server-new srv-cfg s-ent of
            Err _ -> False
            Ok srv0 -> case drive-hs cli0 srv0 ch-wire 6 of
              Err _ -> False
              Ok (cli1, srv1) ->
                let cst = tls-client-state cli1 in
                let sst = tls-server-state srv1 in
                let ok-meta = (
                  tls-state-handshake-complete cst
                  && tls-state-handshake-complete sst
                  && tls-cipher-suite-equal (tls-state-cipher-suite cst) (tls-state-cipher-suite sst)
                  && tls-state-server-name sst == sni
                  && tls-state-negotiated-protocol cst == want-alpn
                  && tls-state-negotiated-protocol sst == want-alpn) in
                let ok-exp = (case (tls-client-export-keying-material cli1 "EXPORTER-cinch" (string-to-bytes "ctx") 32, tls-server-export-keying-material srv1 "EXPORTER-cinch" (string-to-bytes "ctx") 32) of
                  (Ok a, Ok b) -> bytes-len a == 32 && eq-i64 (bytes-compare a b) 0
                  _ -> False) in
                -- Client -> Server message
                let msg1 = string-to-bytes "ping-from-cinch-client-over-tls13" in
                let ok-io = (case tls-client-send cli1 msg1 of
                  Err _ -> False
                  Ok (cli2, w1) -> case tls-server-feed srv1 w1 of
                    Err _ -> False
                    Ok s-out1 ->
                      let srv2 = tls-out-state s-out1 in
                      if not (eq-i64 (bytes-compare (tls-out-data s-out1) msg1) 0) then False
                      else
                        -- KeyUpdate with request_update = True from client -> server -> client!
                        case tls-client-key-update cli2 True of
                          Err _ -> False
                          Ok (cli3, ku-wire) -> case tls-server-feed srv2 ku-wire of
                            Err _ -> False
                            Ok s-ku ->
                              let srv3 = tls-out-state s-ku in
                              let ku-reply = tls-out-wire s-ku in
                              case tls-client-feed cli3 ku-reply of
                                Err _ -> False
                                Ok c-ku ->
                                  let cli4 = tls-out-state c-ku in
                                  -- Post-KeyUpdate bidirectional transfer (including multi-record > 16 KiB if chunk-size == 17)
                                  let msg2 = if chunk-size == 17 then bytes-repeat 2000 (string-to-bytes "0123456789") else string-to-bytes "pong-after-key-update" in
                                  case tls-server-send srv3 msg2 of
                                    Err _ -> False
                                    Ok (srv4, w2) -> case feed-cli-chunked cli4 w2 0 chunk-size (bytes-new 0) (bytes-new 0) of
                                      Err _ -> False
                                      Ok (cli5, _, got2, _) ->
                                        if not (eq-i64 (bytes-compare got2 msg2) 0) then False
                                        else
                                          -- Clean close_notify from client -> server
                                          case tls-client-close cli5 of
                                            Err _ -> False
                                            Ok (_, cn-wire) -> case tls-server-feed srv4 cn-wire of
                                              Ok s-cn -> tls-out-eof s-cn && tls-state-peer-closed (tls-server-state (tls-out-state s-cn))
                                              Err _ -> False) in
                ok-meta && ok-exp && ok-io

  -- | Verify bad hostname is rejected with X509HostnameMismatch.
  check-wrong-hostname :: Bool
  check-wrong-hostname =
    case tls-load-x509-key-pair ed-chain ed-key of
      Err _ -> False
      Ok cert ->
        let cli-cfg = (tls-config-with-now valid-now
          (tls-config-with-roots (pool-from-pem ed-root)
            (tls-config-with-server-name "wrong.example" tls-config-default))) in
        let srv-cfg = tls-config-with-certificate cert tls-config-default in
        case (tls-client-start cli-cfg (sha256-sum (string-to-bytes "c1")), tls-server-new srv-cfg (sha256-sum (string-to-bytes "s1"))) of
          (Ok (cli0, ch), Ok srv0) -> case drive-hs cli0 srv0 ch 4 of
            Err (TlsCertVerifyFailed (X509HostnameMismatch _)) -> True
            _ -> False
          _ -> False

  -- | Verify untrusted CA is rejected with X509UnknownAuthority.
  check-untrusted-ca :: Bool
  check-untrusted-ca =
    case tls-load-x509-key-pair ed-chain ed-key of
      Err _ -> False
      Ok cert ->
        let cli-cfg = tls-config-with-now valid-now (tls-config-with-server-name "ed25519.test" tls-config-default) in
        let srv-cfg = tls-config-with-certificate cert tls-config-default in
        case (tls-client-start cli-cfg (sha256-sum (string-to-bytes "c2")), tls-server-new srv-cfg (sha256-sum (string-to-bytes "s2"))) of
          (Ok (cli0, ch), Ok srv0) -> case drive-hs cli0 srv0 ch 4 of
            Err (TlsCertVerifyFailed X509UnknownAuthority) -> True
            _ -> False
          _ -> False

  -- | Verify expired certificate is rejected with X509Expired.
  check-expired-cert :: Bool
  check-expired-cert =
    case tls-load-x509-key-pair ed-chain ed-key of
      Err _ -> False
      Ok cert ->
        let cli-cfg = (tls-config-with-now 1800000000
          (tls-config-with-roots (pool-from-pem ed-root)
            (tls-config-with-server-name "ed25519.test" tls-config-default))) in
        let srv-cfg = tls-config-with-certificate cert tls-config-default in
        case (tls-client-start cli-cfg (sha256-sum (string-to-bytes "c3")), tls-server-new srv-cfg (sha256-sum (string-to-bytes "s3"))) of
          (Ok (cli0, ch), Ok srv0) -> case drive-hs cli0 srv0 ch 4 of
            Err (TlsCertVerifyFailed X509Expired) -> True
            _ -> False
          _ -> False

  -- | Verify tampered application data record fails with TlsBadRecordMac.
  check-tampered-record :: Bool
  check-tampered-record =
    case tls-load-x509-key-pair ed-chain ed-key of
      Err _ -> False
      Ok cert ->
        let cli-cfg = (tls-config-with-now valid-now
          (tls-config-with-roots (pool-from-pem ed-root)
            (tls-config-with-server-name "ed25519.test" tls-config-default))) in
        let srv-cfg = tls-config-with-certificate cert tls-config-default in
        case (tls-client-start cli-cfg (sha256-sum (string-to-bytes "c4")), tls-server-new srv-cfg (sha256-sum (string-to-bytes "s4"))) of
          (Ok (cli0, ch), Ok srv0) -> case drive-hs cli0 srv0 ch 4 of
            Err _ -> False
            Ok (cli1, srv1) -> case tls-client-send cli1 (string-to-bytes "secret message") of
              Err _ -> False
              Ok (_, wire) ->
                let bad = clone-bytes wire in
                let u = if 10 < bytes-len bad then bytes-set bad 10 (if bytes-get bad 10 == 0 then 1 else 0) else () in
                case tls-server-feed srv1 bad of
                  Err TlsBadRecordMac -> True
                  _ -> False
          _ -> False

  -- | Verify disjoint cipher suites fail cleanly with TlsNoCommonCipherSuite.
  check-disjoint-suites :: Bool
  check-disjoint-suites =
    case tls-load-x509-key-pair ed-chain ed-key of
      Err _ -> False
      Ok cert ->
        let cli-cfg = tls-config-with-cipher-suites [TlsAes128GcmSha256] tls-config-default in
        let srv-cfg = tls-config-with-cipher-suites [TlsAes256GcmSha384] (tls-config-with-certificate cert tls-config-default) in
        case (tls-client-start cli-cfg (sha256-sum (string-to-bytes "c5")), tls-server-new srv-cfg (sha256-sum (string-to-bytes "s5"))) of
          (Ok (cli0, ch), Ok srv0) -> case tls-server-feed srv0 ch of
            Err TlsNoCommonCipherSuite -> True
            _ -> False
          _ -> False

  -- | Return 200 when all rows pass, or the 0-based index of the first failing row.
  first-fail :: List Bool -> Int -> Int
  first-fail xs i = case xs of
    Nil -> 200
    Cons b rest -> if b then first-fail rest (i + 1) else i

  -- | Entry point running the full handshake matrix and negative checks.
  main :: Int
  main = first-fail [
    -- 0: Ed25519 + AES-128-GCM-SHA256 + X25519 + ALPN h2 + large 20 KiB multi-record transfer
    run-scenario ed-root ed-chain ed-key "ed25519.test" [TlsAes128GcmSha256] [TlsAes128GcmSha256] [CurveX25519] [CurveX25519] ["h2", "http/1.1"] ["http/1.1", "h2"] "http/1.1" 17,
    -- 1: ECDSA-P256 (3-cert chain with intermediate) + AES-256-GCM-SHA384 + P-256 + ALPN h2 + 7-byte chunks
    run-scenario ec-root ec-chain ec-key "ecdsa.test" [TlsAes256GcmSha384] [TlsAes256GcmSha384] [CurveP256] [CurveP256] ["h2"] ["h2", "http/1.1"] "h2" 7,
    -- 2: RSA-PSS-SHA256 + ChaCha20-Poly1305-SHA256 + X25519 + 1-byte-at-a-time fragmented delivery
    run-scenario rsa-root rsa-chain rsa-key "rsa.test" [TlsChacha20Poly1305Sha256] [TlsChacha20Poly1305Sha256] [CurveX25519] [CurveX25519] Nil Nil "" 1,
    -- 3: HelloRetryRequest! Client offers [X25519, P-256] (sends X25519 share), Server requires [P-256] -> HRR -> CH2(P-256) -> 2-RTT handshake!
    run-scenario ed-root ed-chain ed-key "ed25519.test" [TlsAes128GcmSha256, TlsChacha20Poly1305Sha256] [TlsChacha20Poly1305Sha256] [CurveX25519, CurveP256] [CurveP256] ["h2"] ["h2"] "h2" 64,
    -- 4: Negative security checks
    check-wrong-hostname,
    check-untrusted-ca,
    check-expired-cert,
    check-tampered-record,
    check-disjoint-suites
  ] 0
"#,
        ed_root = lit(ed["root_pem"].as_str().unwrap()),
        ed_chain = lit(ed["chain_pem"].as_str().unwrap()),
        ed_key = lit(ed["key_pem"].as_str().unwrap()),
        ec_root = lit(ec["root_pem"].as_str().unwrap()),
        ec_chain = lit(ec["chain_pem"].as_str().unwrap()),
        ec_key = lit(ec["key_pem"].as_str().unwrap()),
        rsa_root = lit(rsa["root_pem"].as_str().unwrap()),
        rsa_chain = lit(rsa["chain_pem"].as_str().unwrap()),
        rsa_key = lit(rsa["key_pem"].as_str().unwrap()),
    );

    let code = run_cinch("tls_matrix", &main_src);
    assert_eq!(
        code, 200,
        "tls_matrix: exit status {code} is first failing scenario (200 = all 9 passed)"
    );
}

/// Build `tooling/tls_vectors.go` once into a temporary executable for fast live loopback interop.
fn build_go_tls_helper() -> (PathBuf, PathBuf) {
    let tmp_dir = std::env::temp_dir().join(format!("cinch-tls-go-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&tmp_dir);
    let bin = tmp_dir.join("tls_vectors");
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let status = Command::new("go")
        .arg("build")
        .arg("-o")
        .arg(&bin)
        .arg("tooling/tls_vectors.go")
        .current_dir(&repo_root)
        .env("GOCACHE", "/tmp/go-cache")
        .env("GOTELEMETRY", "off")
        .status()
        .expect("go build tooling/tls_vectors.go");
    assert!(status.success(), "go build tooling/tls_vectors.go failed");
    (tmp_dir, bin)
}

/// Spawn Go live TLS peer (`--mode=server` or `--mode=client`), wait for `PORT=<n>`, return `(child, port)`.
fn spawn_go_peer(bin: &PathBuf, args: &[&str]) -> (std::process::Child, u16, BufReader<std::process::ChildStdout>) {
    let mut child = Command::new(bin)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("spawn go tls peer");
    let stdout = child.stdout.take().expect("child stdout");
    let mut reader = BufReader::new(stdout);
    let mut line = String::new();
    reader.read_line(&mut line).expect("read PORT line from go peer");
    let port: u16 = line
        .trim()
        .strip_prefix("PORT=")
        .unwrap_or_else(|| panic!("expected PORT=<n>, got {line:?}"))
        .parse()
        .expect("parse port");
    (child, port, reader)
}

/// 4. Real live loopback TCP interop (`Effect.Net` + `Effect.Tls` + `Effect.Random` + `Effect.Clock`)
/// against Go 1.24 `crypto/tls` in BOTH directions:
///   A. Cinch `Effect.Tls` Client (`tls-dial`) <-> Go 1.24 `crypto/tls` Server (Ed25519+AES128+X25519,
///      ECDSA-P256-chain+AES256+P256, RSA-PSS+ChaCha20+X25519)
///   B. Go 1.24 `crypto/tls` Client <-> Cinch `Effect.Tls` Server (`tls-server` / `tls-accept`)
///      (Ed25519+default Go MLKEM/X25519, ECDSA-P256-chain+P256+AES256, RSA-PSS+ChaCha20)
#[test]
fn tls_live_loopback_interop_with_go_124() {
    let (tmp_dir, go_bin) = build_go_tls_helper();

    // --- Direction A: Cinch Effect.Tls Client -> Go 1.24 crypto/tls Server (3 live peers) ---
    let ca1 = tmp_dir.join("srv1_ca.pem");
    let ca2 = tmp_dir.join("srv2_ca.pem");
    let ca3 = tmp_dir.join("srv3_ca.pem");

    let (mut srv1, p1, _) = spawn_go_peer(
        &go_bin,
        &["-mode=server", "-cert-type=ed25519", "-alpn=h2,http/1.1", "-ca-out", ca1.to_str().unwrap()],
    );
    let (mut srv2, p2, _) = spawn_go_peer(
        &go_bin,
        &["-mode=server", "-cert-type=ecdsa", "-alpn=http/1.1", "-ca-out", ca2.to_str().unwrap()],
    );
    let (mut srv3, p3, _) = spawn_go_peer(
        &go_bin,
        &["-mode=server", "-cert-type=rsa", "-ca-out", ca3.to_str().unwrap()],
    );

    let ca1_pem = std::fs::read_to_string(&ca1).unwrap();
    let ca2_pem = std::fs::read_to_string(&ca2).unwrap();
    let ca3_pem = std::fs::read_to_string(&ca3).unwrap();

    // --- Direction B: Go 1.24 crypto/tls Client -> Cinch Effect.Tls Server (3 live peers) ---
    let c1_chain = tmp_dir.join("cli1_chain.pem");
    let c1_key = tmp_dir.join("cli1_key.pem");
    let c2_chain = tmp_dir.join("cli2_chain.pem");
    let c2_key = tmp_dir.join("cli2_key.pem");
    let c3_chain = tmp_dir.join("cli3_chain.pem");
    let c3_key = tmp_dir.join("cli3_key.pem");

    let (mut cli1, cp1, mut r1) = spawn_go_peer(
        &go_bin,
        &[
            "-mode=client", "-cert-type=ed25519", "-sni=server.test", "-alpn=h2,http/1.1",
            "-chain-out", c1_chain.to_str().unwrap(), "-key-out", c1_key.to_str().unwrap(),
        ],
    );
    let (mut cli2, cp2, mut r2) = spawn_go_peer(
        &go_bin,
        &[
            "-mode=client", "-cert-type=ecdsa", "-sni=ecdsa.live.test", "-alpn=http/1.1", "-curve=p256",
            "-chain-out", c2_chain.to_str().unwrap(), "-key-out", c2_key.to_str().unwrap(),
        ],
    );
    let (mut cli3, cp3, mut r3) = spawn_go_peer(
        &go_bin,
        &[
            "-mode=client", "-cert-type=rsa", "-sni=rsa.live.test", "-curve=x25519",
            "-chain-out", c3_chain.to_str().unwrap(), "-key-out", c3_key.to_str().unwrap(),
        ],
    );

    let c1_chain_pem = std::fs::read_to_string(&c1_chain).unwrap();
    let c1_key_pem = std::fs::read_to_string(&c1_key).unwrap();
    let c2_chain_pem = std::fs::read_to_string(&c2_chain).unwrap();
    let c2_key_pem = std::fs::read_to_string(&c2_key).unwrap();
    let c3_chain_pem = std::fs::read_to_string(&c3_chain).unwrap();
    let c3_key_pem = std::fs::read_to_string(&c3_key).unwrap();

    let main_src = format!(
        r#"-- | Live loopback TCP interop between Effect.Tls and Go 1.24 crypto/tls in both directions.
module Main where
  import Crypto.Tls
  import Crypto.X509
  import Data.Bool
  import Data.Bytes
  import Data.Int
  import Data.List
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Io
  import Effect.Net
  import Effect.Tls

  -- | Load a root CA `CertPool` from PEM text.
  pool-of :: String -> CertPool
  pool-of pem = case cert-pool-append-pem cert-pool-new (string-to-bytes pem) of (p, _) -> p

  -- | Decode bytes to UTF-8 text.
  text :: Bytes -> String
  text b = case string-from-bytes b of Some s -> s; None -> ""

  -- | Run one live Cinch TLS 1.3 Client -> Go 1.24 TLS 1.3 Server loopback exchange.
  run-cinch-client :: forall (r :: Region). using (io :: Io r) => Nat -> String -> List TlsCipherSuite -> List TlsCurve -> List String -> String -> String ->{{IO r}} Bool
  run-cinch-client port ca-pem suites curves alpn want-alpn want-suite =
    let cfg = (tls-config-with-alpn alpn
      (tls-config-with-curves curves
        (tls-config-with-cipher-suites suites
          (tls-config-with-roots (pool-of ca-pem)
            (tls-config-with-server-name "server.test" tls-config-default))))) in
    case tls-dial "127.0.0.1" port cfg of
      Err _ -> False
      Ok conn0 ->
        let st = tls-connection-state conn0 in
        if not (tls-state-handshake-complete st && tls-state-negotiated-protocol st == want-alpn) then False
        else case tls-write conn0 (string-to-bytes "HELLO-FROM-CINCH") of
          Err _ -> False
          Ok (nw, conn1) ->
            if nw /= 16 then False
            else case tls-read conn1 4096 of
              Err _ -> False
              Ok (reply, conn2) ->
                let want = string-concat "GO-SRV-ACK:HELLO-FROM-CINCH|sni=server.test|alpn=" (string-concat want-alpn (string-concat "|suite=" want-suite)) in
                let ok-msg = text reply == want in
                case tls-close conn2 of
                  Ok _ -> ok-msg
                  Err _ -> False

  -- | Run one live Go 1.24 TLS 1.3 Client -> Cinch TLS 1.3 Server loopback exchange.
  run-cinch-server :: forall (r :: Region). using (io :: Io r) => Nat -> String -> String -> List TlsCipherSuite -> List TlsCurve -> List String -> String -> String ->{{IO r}} Bool
  run-cinch-server port chain-pem key-pem suites curves alpn want-sni want-alpn =
    case tls-load-x509-key-pair (string-to-bytes chain-pem) (string-to-bytes key-pem) of
      Err _ -> False
      Ok cert ->
        let cfg = (tls-config-with-alpn alpn
          (tls-config-with-curves curves
            (tls-config-with-cipher-suites suites
              (tls-config-with-certificate cert tls-config-default)))) in
        case tls-accept cfg (\() -> case net-connect "127.0.0.1" port of Ok s -> Ok (Some s); Err e -> Err e) of
          Ok (Some conn0) ->
            let st = tls-connection-state conn0 in
            if not (tls-state-handshake-complete st && tls-state-server-name st == want-sni && tls-state-negotiated-protocol st == want-alpn) then False
            else case tls-read conn0 4096 of
              Err _ -> False
              Ok (msg, conn1) ->
                let reply = string-concat "CINCH-SRV-ACK:" (text msg) in
                case tls-write conn1 (string-to-bytes reply) of
                  Err _ -> False
                  Ok (_, conn2) -> case tls-close conn2 of
                    Ok _ -> text msg == "HELLO-FROM-GO-CLIENT"
                    Err _ -> False
          _ -> False

  -- | Return 200 when all live checks pass, or the 0-based index of the first failure.
  first-fail :: List Bool -> Int -> Int
  first-fail xs i = case xs of
    Nil -> 200
    Cons b rest -> if b then first-fail rest (i + 1) else i

  -- | Run all 6 live TCP loopback interop scenarios under `IO r`.
  main :: Unit ->{{IO r}} Int
  main u = with (io = newIO ()) in
    let r0 = run-cinch-client {p1} {ca1_pem} [TlsAes128GcmSha256] [CurveX25519] ["h2", "http/1.1"] "h2" "TLS_AES_128_GCM_SHA256" in
    let r1 = run-cinch-client {p2} {ca2_pem} [TlsAes256GcmSha384] [CurveP256] ["h2", "http/1.1"] "http/1.1" "TLS_AES_256_GCM_SHA384" in
    let r2 = run-cinch-client {p3} {ca3_pem} [TlsChacha20Poly1305Sha256] [CurveX25519] Nil "" "TLS_CHACHA20_POLY1305_SHA256" in
    let r3 = run-cinch-server {cp1} {c1_chain_pem} {c1_key_pem} [TlsAes128GcmSha256] [CurveX25519, CurveP256] ["h2", "http/1.1"] "server.test" "h2" in
    let r4 = run-cinch-server {cp2} {c2_chain_pem} {c2_key_pem} [TlsAes256GcmSha384] [CurveP256] ["http/1.1"] "ecdsa.live.test" "http/1.1" in
    let r5 = run-cinch-server {cp3} {c3_chain_pem} {c3_key_pem} [TlsChacha20Poly1305Sha256] [CurveX25519] Nil "rsa.live.test" "" in
    first-fail [r0, r1, r2, r3, r4, r5] 0
"#,
        ca1_pem = lit(&ca1_pem),
        ca2_pem = lit(&ca2_pem),
        ca3_pem = lit(&ca3_pem),
        c1_chain_pem = lit(&c1_chain_pem),
        c1_key_pem = lit(&c1_key_pem),
        c2_chain_pem = lit(&c2_chain_pem),
        c2_key_pem = lit(&c2_key_pem),
        c3_chain_pem = lit(&c3_chain_pem),
        c3_key_pem = lit(&c3_key_pem),
    );

    let code = run_cinch("tls_live_go_interop", &main_src);

    let s1 = srv1.wait().expect("wait go srv1");
    let s2 = srv2.wait().expect("wait go srv2");
    let s3 = srv3.wait().expect("wait go srv3");

    let mut out1 = String::new();
    let _ = r1.read_line(&mut out1);
    let mut out2 = String::new();
    let _ = r2.read_line(&mut out2);
    let mut out3 = String::new();
    let _ = r3.read_line(&mut out3);

    let g1 = cli1.wait().expect("wait go cli1");
    let g2 = cli2.wait().expect("wait go cli2");
    let g3 = cli3.wait().expect("wait go cli3");

    let _ = std::fs::remove_dir_all(&tmp_dir);

    assert_eq!(code, 200, "Cinch live interop exit status {code} (200 = all 6 passed)");
    assert!(s1.success(), "Go server 1 (Ed25519/AES128/X25519) failed");
    assert!(s2.success(), "Go server 2 (ECDSA-P256-chain/AES256/P256) failed");
    assert!(s3.success(), "Go server 3 (RSA-PSS/ChaCha20/X25519) failed");
    assert!(g1.success(), "Go client 1 (Ed25519/default MLKEM+X25519) failed: {out1:?}");
    assert!(g2.success(), "Go client 2 (ECDSA-P256-chain/P256/AES256) failed: {out2:?}");
    assert!(g3.success(), "Go client 3 (RSA-PSS/ChaCha20/X25519) failed: {out3:?}");
    assert!(out1.contains("alpn=h2|suite=TLS_AES_128_GCM_SHA256"), "unexpected cli1 output: {out1:?}");
    assert!(out2.contains("alpn=http/1.1|suite=TLS_AES_256_GCM_SHA384"), "unexpected cli2 output: {out2:?}");
    assert!(out3.contains("suite=TLS_CHACHA20_POLY1305_SHA256"), "unexpected cli3 output: {out3:?}");
}
