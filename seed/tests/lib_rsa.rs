//! RSA signatures, encryption, PKCS#1 DER key serialization, validation and key generation
//! verified against independent Go 1.24 `crypto/rsa` and `crypto/x509` reference vectors
//! (`tooling/rsa_vectors.go` -> `seed/tests/fixtures/rsa-vectors.json`).
mod common;

use serde_json::Value;

/// Compile and run one batch of fixture rows; exit code 200 means all rows passed, otherwise the failing row index.
fn run_batch(rows: &[String]) {
    let dir = common::project("rsa", &[
        ("Crypto.Rsa.cinch", include_str!("../prelude/Crypto/Rsa.cinch")),
        ("Crypto.ConstantTime.cinch", include_str!("../prelude/Crypto/ConstantTime.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Text.Asn1.cinch", include_str!("../prelude/Text/Asn1.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Text.Md5.cinch", include_str!("../prelude/Text/Md5.cinch")),
        ("Text.Sha1.cinch", include_str!("../prelude/Text/Sha1.cinch")),
        ("Text.Sha256.cinch", include_str!("../prelude/Text/Sha256.cinch")),
        ("Text.Sha512.cinch", include_str!("../prelude/Text/Sha512.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every row in this batch.\n  main :: Int\n  main = first [{}] 0\n", rows.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "exit status is the first failing row index in the batch; 200 means every row passed");
    common::cleanup(&dir);
}

fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/rsa-vectors.json")).unwrap()
}

fn s(v: &Value, k: &str) -> String {
    format!("{:?}", v[k].as_str().unwrap())
}

fn hash_ctor(name: &str) -> &'static str {
    match name {
        "none" => "RsaNoHash",
        "md5" => "RsaMd5",
        "sha1" => "RsaSha1",
        "sha224" => "RsaSha224",
        "sha256" => "RsaSha256",
        "sha384" => "RsaSha384",
        "sha512" => "RsaSha512",
        "sha512_224" => "RsaSha512t224",
        "sha512_256" => "RsaSha512t256",
        other => panic!("unexpected hash name: {other}"),
    }
}

fn key_der<'a>(f: &'a Value, name: &str) -> (&'a str, &'a str) {
    let k = f["keys"].as_array().unwrap().iter().find(|k| k["name"].as_str().unwrap() == name).unwrap();
    (k["pkcs1_public_der"].as_str().unwrap(), k["pkcs1_private_der"].as_str().unwrap())
}

/// 1024-bit and 2048-bit key construction, validation, PKCS#1 DER roundtrips and malformed DER rejection.
#[test]
fn rsa_keys_and_pkcs1_der_roundtrip() {
    let f = fixture();
    let mut rows = Vec::new();
    for k in f["keys"].as_array().unwrap() {
        rows.push(format!(
            "check-key {} {} {} {} {} {} {} {} {} {} {} {}",
            k["bits"].as_u64().unwrap(),
            k["size"].as_u64().unwrap(),
            s(k, "n"),
            k["e"].as_u64().unwrap(),
            s(k, "d"),
            s(k, "p"),
            s(k, "q"),
            s(k, "dp"),
            s(k, "dq"),
            s(k, "qinv"),
            s(k, "pkcs1_public_der"),
            s(k, "pkcs1_private_der"),
        ));
    }
    for bad in f["bad_public_ders"].as_array().unwrap() {
        rows.push(format!("bad-pub-der {:?}", bad.as_str().unwrap()));
    }
    for bad in f["bad_private_ders"].as_array().unwrap() {
        rows.push(format!("bad-priv-der {:?}", bad.as_str().unwrap()));
    }
    rows.push("check-error-messages".to_string());
    run_batch(&rows);
}

/// All 18 PKCS#1 v1.5 signature vectors across 1024-bit and 2048-bit keys and all supported hashes.
#[test]
fn rsa_pkcs1v15_signatures_match_go() {
    let f = fixture();
    let mut rows = Vec::new();
    for v in f["pkcs1v15_signs"].as_array().unwrap() {
        let (_, priv_der) = key_der(&f, v["key"].as_str().unwrap());
        rows.push(format!(
            "check-pkcs1v15-sig {:?} {} {} {}",
            priv_der,
            hash_ctor(v["hash"].as_str().unwrap()),
            s(v, "digest"),
            s(v, "sig"),
        ));
    }
    let (_, priv1024) = key_der(&f, "rsa1024");
    rows.push(format!("check-pkcs1v15-edges {:?}", priv1024));
    run_batch(&rows);
}

/// All 15 RSASSA-PSS vectors across 1024-bit and 2048-bit keys, SHA-1/224/256/384/512, and all salt policies.
#[test]
fn rsa_pss_signatures_match_go() {
    let f = fixture();
    let mut rows = Vec::new();
    for v in f["pss_signs"].as_array().unwrap() {
        let (_, priv_der) = key_der(&f, v["key"].as_str().unwrap());
        let policy = match v["salt_policy"].as_str().unwrap() {
            "auto" => "PssSaltAuto".to_string(),
            "equals_hash" => "PssSaltEqualsHash".to_string(),
            "exact" => format!("(PssSaltExact {})", v["salt_len"].as_u64().unwrap()),
            other => panic!("unknown policy: {other}"),
        };
        rows.push(format!(
            "check-pss-sig {:?} {} {} {} {} {}",
            priv_der,
            hash_ctor(v["hash"].as_str().unwrap()),
            s(v, "digest"),
            policy,
            s(v, "salt"),
            s(v, "sig"),
        ));
    }
    let (_, priv1024) = key_der(&f, "rsa1024");
    rows.push(format!("check-pss-edges {:?}", priv1024));
    run_batch(&rows);
}

/// All 10 OAEP vectors and 6 PKCS#1 v1.5 encryption vectors across 1024-bit and 2048-bit keys.
#[test]
fn rsa_oaep_and_pkcs1v15_encryption_match_go() {
    let f = fixture();
    let mut rows = Vec::new();
    for v in f["oaep_encs"].as_array().unwrap() {
        let (_, priv_der) = key_der(&f, v["key"].as_str().unwrap());
        rows.push(format!(
            "check-oaep {:?} {} {} {} {} {}",
            priv_der,
            hash_ctor(v["hash"].as_str().unwrap()),
            s(v, "label"),
            s(v, "message"),
            s(v, "seed"),
            s(v, "ciphertext"),
        ));
    }
    for v in f["pkcs1v15_encs"].as_array().unwrap() {
        let (_, priv_der) = key_der(&f, v["key"].as_str().unwrap());
        rows.push(format!(
            "check-pkcs1v15-enc {:?} {} {} {}",
            priv_der,
            s(v, "message"),
            s(v, "pad"),
            s(v, "ciphertext"),
        ));
    }
    let (_, priv1024) = key_der(&f, "rsa1024");
    let (_, priv2048) = key_der(&f, "rsa2048");
    rows.push(format!("check-enc-edges {:?} {:?}", priv1024, priv2048));
    run_batch(&rows);
}

/// All 11 tampered/invalid signatures and 12 tampered/invalid ciphertexts are rejected.
#[test]
fn rsa_tampered_signatures_and_ciphertexts_rejected() {
    let f = fixture();
    let mut rows = Vec::new();
    for v in f["bad_sigs"].as_array().unwrap() {
        let (pub_der, _) = key_der(&f, v["key"].as_str().unwrap());
        let fn_name = match v["kind"].as_str().unwrap() {
            "pkcs1v15" => "bad-pkcs1v15-sig",
            "pss" => "bad-pss-sig",
            other => panic!("unknown sig kind: {other}"),
        };
        rows.push(format!(
            "{} {:?} {} {} {}",
            fn_name,
            pub_der,
            hash_ctor(v["hash"].as_str().unwrap()),
            s(v, "digest"),
            s(v, "sig"),
        ));
    }
    for v in f["bad_encs"].as_array().unwrap() {
        let (_, priv_der) = key_der(&f, v["key"].as_str().unwrap());
        match v["kind"].as_str().unwrap() {
            "oaep" => rows.push(format!(
                "bad-oaep-ct {:?} {} {} {}",
                priv_der,
                hash_ctor(v["hash"].as_str().unwrap()),
                s(v, "label"),
                s(v, "ciphertext"),
            )),
            "pkcs1v15" => rows.push(format!(
                "bad-pkcs1v15-ct {:?} {}",
                priv_der,
                s(v, "ciphertext"),
            )),
            other => panic!("unknown enc kind: {other}"),
        }
    }
    run_batch(&rows);
}

/// Deterministic seeded RSA key generation produces valid 1024-bit keys that pass all operations.
#[test]
fn rsa_seeded_key_generation() {
    run_batch(&["check-keygen".to_string()]);
}

/// Every public and private declaration in Crypto.Rsa has canonical documentation.
#[test]
fn rsa_declarations_have_canonical_documentation() {
    common::assert_documented("Crypto.Rsa", include_str!("../prelude/Crypto/Rsa.cinch"));
}

const HELPERS: &str = r#"-- | Reference RSA test consumers.
module Main where
  import Crypto.Rsa
  import Data.Bytes
  import Data.Nat
  import Data.Result
  import Data.String
  import Text.Hex
  import Text.Sha256

  -- | Decode hex string to Bytes.
  b :: String -> Bytes
  b text = case hex-decode text of Ok v -> v; Err _ -> bytes-new 0

  -- | Decode hex string to Nat.
  n-of :: String -> Nat
  n-of text = rsa-os2ip (b text)

  -- | Verify key construction, CRT fields, validation, and PKCS#1 DER roundtrips.
  check-key :: Nat -> Nat -> String -> Nat -> String -> String -> String -> String -> String -> String -> String -> String -> Bool
  check-key bits size nh e dh ph qh dph dqh qinvh pub-der priv-der =
    case rsa-private-key (n-of nh) e (n-of dh) (n-of ph) (n-of qh) of
      Err _ -> False
      Ok priv ->
        let pub = rsa-public-key-of priv in
        rsa-key-bits pub == bits
          && rsa-key-size pub == size
          && rsa-private-dp priv == n-of dph
          && rsa-private-dq priv == n-of dqh
          && rsa-private-qinv priv == n-of qinvh
          && (case rsa-validate-private-key priv of Ok _ -> True; Err _ -> False)
          && hex-encode (rsa-marshal-pkcs1-public-key pub) == pub-der
          && hex-encode (rsa-marshal-pkcs1-private-key priv) == priv-der
          && (case rsa-parse-pkcs1-public-key (b pub-der) of Ok p2 -> rsa-public-key-equal pub p2; Err _ -> False)
          && (case rsa-parse-pkcs1-private-key (b priv-der) of Ok k2 -> rsa-private-key-equal priv k2; Err _ -> False)

  -- | Malformed PKCS#1 public key DER must be rejected.
  bad-pub-der :: String -> Bool
  bad-pub-der der = case rsa-parse-pkcs1-public-key (b der) of Err _ -> True; Ok _ -> False

  -- | Malformed PKCS#1 private key DER must be rejected.
  bad-priv-der :: String -> Bool
  bad-priv-der der = case rsa-parse-pkcs1-private-key (b der) of Err _ -> True; Ok _ -> False

  -- | Every error variant renders a non-empty message, and invalid keys and hash helpers behave as specified.
  check-error-messages :: Bool
  check-error-messages =
    rsa-error-message RsaMessageTooLong /= ""
      && rsa-error-message RsaDecryptionError /= ""
      && rsa-error-message RsaVerificationError /= ""
      && rsa-error-message RsaInvalidPublicKey /= ""
      && rsa-error-message RsaInvalidPrivateKey /= ""
      && rsa-error-message RsaInvalidDigestLength /= ""
      && rsa-error-message RsaUnsupportedHash /= ""
      && rsa-error-message RsaInvalidParameter /= ""
      && rsa-error-message RsaInvalidEncoding /= ""
      && rsa-hash-size RsaNoHash == 0
      && rsa-hash-size RsaMd5 == 16
      && rsa-hash-size RsaSha1 == 20
      && rsa-hash-size RsaSha224 == 28
      && rsa-hash-size RsaSha256 == 32
      && rsa-hash-size RsaSha384 == 48
      && rsa-hash-size RsaSha512 == 64
      && rsa-hash-size RsaSha512t224 == 28
      && rsa-hash-size RsaSha512t256 == 32
      && (case rsa-hash-sum RsaNoHash (bytes-new 0) of Err RsaUnsupportedHash -> True; other -> False)
      && (case rsa-hash-sum RsaSha256 (bytes-new 0) of Ok d -> bytes-len d == 32; Err _ -> False)
      && (case rsa-public-key 0 65537 of Err RsaInvalidPublicKey -> True; other -> False)
      && (case rsa-public-key 65536 65537 of Err RsaInvalidPublicKey -> True; other -> False)
      && (case rsa-public-key 65537 2 of Err RsaInvalidPublicKey -> True; other -> False)
      && (case rsa-public-key 65537 4 of Err RsaInvalidPublicKey -> True; other -> False)
      && (case rsa-public-key 65537 2147483649 of Err RsaInvalidPublicKey -> True; other -> False)
      && (case rsa-private-key (65537 * 65537) 65537 1 65537 65537 of Err RsaInvalidPrivateKey -> True; other -> False)
      && (case rsa-private-key 65537 3 1 257 257 of Err RsaInvalidPrivateKey -> True; other -> False)

  -- | Verify PKCS#1 v1.5 signing matches reference bytes and verifies cleanly.
  check-pkcs1v15-sig :: String -> RsaHash -> String -> String -> Bool
  check-pkcs1v15-sig priv-der h digest-hex sig-hex =
    case rsa-parse-pkcs1-private-key (b priv-der) of
      Err _ -> False
      Ok priv ->
        let dig = b digest-hex in
        (case rsa-sign-pkcs1v15 priv h dig of
          Err _ -> False
          Ok sig -> hex-encode sig == sig-hex)
        && (case rsa-verify-pkcs1v15 (rsa-public-key-of priv) h dig (b sig-hex) of
          Ok _ -> True
          Err _ -> False)

  -- | Verify PKCS#1 v1.5 error handling for wrong digest length and oversized raw input.
  check-pkcs1v15-edges :: String -> Bool
  check-pkcs1v15-edges priv-der =
    case rsa-parse-pkcs1-private-key (b priv-der) of
      Err _ -> False
      Ok priv ->
        (case rsa-sign-pkcs1v15 priv RsaSha256 (bytes-new 16) of Err RsaInvalidDigestLength -> True; other -> False)
          && (case rsa-sign-pkcs1v15 priv RsaNoHash (bytes-new 120) of Err RsaMessageTooLong -> True; other -> False)

  -- | Verify deterministic PSS signing matches reference bytes, verifies under policy and auto, and seeded signing roundtrips.
  check-pss-sig :: String -> RsaHash -> String -> PssSaltLength -> String -> String -> Bool
  check-pss-sig priv-der h digest-hex policy salt-hex sig-hex =
    case rsa-parse-pkcs1-private-key (b priv-der) of
      Err _ -> False
      Ok priv ->
        let pub = rsa-public-key-of priv in
        let dig = b digest-hex in
        (case rsa-sign-pss-deterministic priv h dig (b salt-hex) of
          Err _ -> False
          Ok sig -> hex-encode sig == sig-hex)
        && (case rsa-verify-pss pub h dig (b sig-hex) policy of Ok _ -> True; Err _ -> False)
        && (case rsa-verify-pss pub h dig (b sig-hex) PssSaltAuto of Ok _ -> True; Err _ -> False)
        && (case rsa-sign-pss priv h dig policy (bytes-from-u8s [1, 2, 3, 4]) of
          Err _ -> False
          Ok s2 ->
            (case rsa-verify-pss pub h dig s2 policy of Ok _ -> True; Err _ -> False)
              && (case rsa-verify-pss pub h dig s2 PssSaltAuto of Ok _ -> True; Err _ -> False))

  -- | Verify PSS edge cases: unsupported RsaNoHash, 1024-bit SHA-512 EqualsHash overflow, and salt-policy mismatch.
  check-pss-edges :: String -> Bool
  check-pss-edges priv-der =
    case rsa-parse-pkcs1-private-key (b priv-der) of
      Err _ -> False
      Ok priv ->
        let pub = rsa-public-key-of priv in
        let d256 = sha256-sum (bytes-from-u8s [1, 2, 3]) in
        (case rsa-sign-pss-deterministic priv RsaNoHash d256 (bytes-new 16) of Err RsaUnsupportedHash -> True; other -> False)
          && (case rsa-sign-pss priv RsaSha512 (bytes-new 64) PssSaltEqualsHash (bytes-from-u8s [7]) of Err RsaMessageTooLong -> True; other -> False)
          && (case rsa-sign-pss-deterministic priv RsaSha256 d256 (bytes-new 20) of
            Err _ -> False
            Ok sig ->
              (case rsa-verify-pss pub RsaSha256 d256 sig PssSaltEqualsHash of Err RsaVerificationError -> True; other -> False)
                && (case rsa-verify-pss pub RsaSha256 d256 sig (PssSaltExact 20) of Ok _ -> True; other -> False))

  -- | Verify OAEP encryption with explicit seed matches reference ciphertext and decrypts back to plaintext.
  check-oaep :: String -> RsaHash -> String -> String -> String -> String -> Bool
  check-oaep priv-der h label-hex msg-hex seed-hex ct-hex =
    case rsa-parse-pkcs1-private-key (b priv-der) of
      Err _ -> False
      Ok priv ->
        let pub = rsa-public-key-of priv in
        let lbl = b label-hex in
        let msg = b msg-hex in
        (case rsa-encrypt-oaep-with-seed pub h lbl msg (b seed-hex) of
          Err _ -> False
          Ok ct -> hex-encode ct == ct-hex)
        && (case rsa-decrypt-oaep priv h lbl (b ct-hex) of
          Err _ -> False
          Ok pt -> hex-encode pt == msg-hex)

  -- | Verify PKCS#1 v1.5 encryption with explicit pad matches reference ciphertext and decrypts back to plaintext.
  check-pkcs1v15-enc :: String -> String -> String -> String -> Bool
  check-pkcs1v15-enc priv-der msg-hex pad-hex ct-hex =
    case rsa-parse-pkcs1-private-key (b priv-der) of
      Err _ -> False
      Ok priv ->
        let pub = rsa-public-key-of priv in
        let msg = b msg-hex in
        (case rsa-encrypt-pkcs1v15-with-pad pub msg (b pad-hex) of
          Err _ -> False
          Ok ct -> hex-encode ct == ct-hex)
        && (case rsa-decrypt-pkcs1v15 priv (b ct-hex) of
          Err _ -> False
          Ok pt -> hex-encode pt == msg-hex)

  -- | Verify encryption edge cases, seeded PKCS#1 v1.5 encryption, and constant-time session key decryption.
  check-enc-edges :: String -> String -> Bool
  check-enc-edges priv1024-der priv2048-der =
    case (rsa-parse-pkcs1-private-key (b priv1024-der), rsa-parse-pkcs1-private-key (b priv2048-der)) of
      (Ok priv1024, Ok priv2048) ->
        let pub1024 = rsa-public-key-of priv1024 in
        let pub2048 = rsa-public-key-of priv2048 in
        let session = bytes-new 32 in
        let fallback = bytes-from-u8s [9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9] in
        (case rsa-encrypt-oaep-with-seed pub1024 RsaSha256 (bytes-new 0) (bytes-new 63) (bytes-new 32) of Err RsaMessageTooLong -> True; other -> False)
          && (case rsa-encrypt-oaep-with-seed pub1024 RsaSha256 (bytes-new 0) (bytes-new 10) (bytes-new 16) of Err RsaInvalidParameter -> True; other -> False)
          && (case rsa-encrypt-pkcs1v15-with-pad pub1024 (bytes-new 118) (bytes-new 8) of Err RsaMessageTooLong -> True; other -> False)
          && (case rsa-encrypt-pkcs1v15-with-pad pub1024 (bytes-new 10) (bytes-new 115) of Err RsaInvalidParameter -> True; other -> False)
          && (case rsa-encrypt-oaep-mgf pub1024 RsaSha256 RsaSha1 (bytes-from-u8s [1, 2]) (bytes-from-u8s [3, 4, 5]) (sha256-sum (bytes-from-u8s [9])) of
            Err _ -> False
            Ok ct ->
              (case rsa-decrypt-oaep-mgf priv1024 RsaSha256 RsaSha1 (bytes-from-u8s [1, 2]) ct of Ok pt -> pt == bytes-from-u8s [3, 4, 5]; Err _ -> False)
                && (case rsa-decrypt-oaep priv1024 RsaSha256 (bytes-from-u8s [1, 2]) ct of Err RsaDecryptionError -> True; other -> False))
          && (case rsa-encrypt-pkcs1v15-from-seed pub2048 session (bytes-from-u8s [42, 43, 44]) of
            Err _ -> False
            Ok ct ->
              (case rsa-decrypt-pkcs1v15-session-key priv2048 ct fallback of Ok k -> k == session; Err _ -> False)
                && (let bad = bytes-new 256 in
                    case rsa-decrypt-pkcs1v15-session-key priv2048 bad fallback of Ok k -> k == fallback; Err _ -> False))
      other -> False

  -- | Tampered PKCS#1 v1.5 signature must fail verification.
  bad-pkcs1v15-sig :: String -> RsaHash -> String -> String -> Bool
  bad-pkcs1v15-sig pub-der h digest-hex sig-hex =
    case rsa-parse-pkcs1-public-key (b pub-der) of
      Err _ -> False
      Ok pub -> case rsa-verify-pkcs1v15 pub h (b digest-hex) (b sig-hex) of Err RsaVerificationError -> True; other -> False

  -- | Tampered PSS signature must fail verification.
  bad-pss-sig :: String -> RsaHash -> String -> String -> Bool
  bad-pss-sig pub-der h digest-hex sig-hex =
    case rsa-parse-pkcs1-public-key (b pub-der) of
      Err _ -> False
      Ok pub -> case rsa-verify-pss pub h (b digest-hex) (b sig-hex) PssSaltAuto of Err RsaVerificationError -> True; other -> False

  -- | Tampered OAEP ciphertext must fail decryption.
  bad-oaep-ct :: String -> RsaHash -> String -> String -> Bool
  bad-oaep-ct priv-der h label-hex ct-hex =
    case rsa-parse-pkcs1-private-key (b priv-der) of
      Err _ -> False
      Ok priv -> case rsa-decrypt-oaep priv h (b label-hex) (b ct-hex) of Err RsaDecryptionError -> True; other -> False

  -- | Tampered PKCS#1 v1.5 ciphertext must fail decryption.
  bad-pkcs1v15-ct :: String -> String -> Bool
  bad-pkcs1v15-ct priv-der ct-hex =
    case rsa-parse-pkcs1-private-key (b priv-der) of
      Err _ -> False
      Ok priv -> case rsa-decrypt-pkcs1v15 priv (b ct-hex) of Err RsaDecryptionError -> True; other -> False

  -- | Deterministic 1024-bit key generation roundtrips PKCS#1 DER, signatures and encryption.
  check-keygen :: Bool
  check-keygen =
    let seed = bytes-from-u8s [11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 101, 102, 103, 104, 105, 106] in
    let bad-bits = case rsa-generate-key-from-seed 32 seed of Err RsaInvalidParameter -> True; other -> False in
    let bad-seed = case rsa-generate-key-from-seed 1024 (bytes-new 0) of Err RsaInvalidParameter -> True; other -> False in
    bad-bits && bad-seed && (case (rsa-generate-key-from-seed 1024 seed, rsa-generate-key-from-seed 1024 seed) of
      (Ok k1, Ok k2) ->
        let pub = rsa-public-key-of k1 in
        let dig = sha256-sum (bytes-from-u8s [1, 2, 3, 4, 5]) in
        let msg = bytes-from-u8s [72, 101, 108, 108, 111] in
        rsa-private-key-equal k1 k2
          && rsa-key-bits pub == 1024
          && rsa-key-size pub == 128
          && (case rsa-validate-private-key k1 of Ok _ -> True; Err _ -> False)
          && (case rsa-parse-pkcs1-private-key (rsa-marshal-pkcs1-private-key k1) of Ok k3 -> rsa-private-key-equal k1 k3; Err _ -> False)
          && (case rsa-sign-pkcs1v15 k1 RsaSha256 dig of Ok sig -> (case rsa-verify-pkcs1v15 pub RsaSha256 dig sig of Ok _ -> True; Err _ -> False); Err _ -> False)
          && (case rsa-sign-pss k1 RsaSha256 dig PssSaltEqualsHash seed of Ok sig -> (case rsa-verify-pss pub RsaSha256 dig sig PssSaltEqualsHash of Ok _ -> True; Err _ -> False); Err _ -> False)
          && (case rsa-encrypt-oaep-with-seed pub RsaSha256 (bytes-new 0) msg (sha256-sum seed) of Ok ct -> (case rsa-decrypt-oaep k1 RsaSha256 (bytes-new 0) ct of Ok pt -> pt == msg; Err _ -> False); Err _ -> False)
          && (case rsa-encrypt-pkcs1v15-from-seed pub msg seed of Ok ct -> (case rsa-decrypt-pkcs1v15 k1 ct of Ok pt -> pt == msg; Err _ -> False); Err _ -> False)
      other -> False)

  -- | Index of the first failing row, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
