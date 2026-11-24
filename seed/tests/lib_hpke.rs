//! HPKE (RFC 9180) against an independent implementation (`tooling/hpke_vectors.py`, pyhpke): for
//! DHKEM(X25519) and DHKEM(P-256) with each KDF, AEAD and mode, derived key pairs, encapsulated keys, three
//! sequential seals and two exports match; recipient contexts open the seals, refuse tampered ones and
//! export the same secrets; export-only suites refuse to seal; mismatched and malformed inputs are refused.
mod common;

use serde_json::Value;

/// Compile and run one batch of fixture rows; the exit status is the first failing row, 200 if none.
fn run_batch(rows: &[String]) {
    let dir = common::project("hpke", &[
        ("Crypto.Hpke.cinch", include_str!("../prelude/Crypto/Hpke.cinch")),
        ("Crypto.ChaCha20Poly1305.cinch", include_str!("../prelude/Crypto/ChaCha20Poly1305.cinch")),
        ("Crypto.Aes.cinch", include_str!("../prelude/Crypto/Aes.cinch")),
        ("Crypto.Cipher.cinch", include_str!("../prelude/Crypto/Cipher.cinch")),
        ("Crypto.ConstantTime.cinch", include_str!("../prelude/Crypto/ConstantTime.cinch")),
        ("Crypto.Field25519.cinch", include_str!("../prelude/Crypto/Field25519.cinch")),
        ("Crypto.Hmac.cinch", include_str!("../prelude/Crypto/Hmac.cinch")),
        ("Crypto.Kdf.cinch", include_str!("../prelude/Crypto/Kdf.cinch")),
        ("Crypto.P256.cinch", include_str!("../prelude/Crypto/P256.cinch")),
        ("Crypto.X25519.cinch", include_str!("../prelude/Crypto/X25519.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Text.Digest.cinch", include_str!("../prelude/Text/Digest.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Text.Sha512.cinch", include_str!("../prelude/Text/Sha512.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every row in this batch.\n  main :: Int\n  main = first [{}] 0\n", rows.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing row index in the batch; 200 means every row passed");
    common::cleanup(&dir);
}

/// A string field as a Cinch literal.
fn s(v: &Value, k: &str) -> String {
    format!("{:?}", v[k].as_str().unwrap())
}

/// Every suite and mode matches the reference end to end.
#[test]
fn hpke_suites_and_modes_match_reference() {
    let rows: Value = serde_json::from_str(include_str!("fixtures/hpke-vectors.json")).unwrap();
    let mut calls = rows.as_array().unwrap().iter().map(|r| {
        let messages = r["messages"].as_array().unwrap().iter().map(|m| format!("({}, {}, {})", s(m, "aad"), s(m, "plaintext"), s(m, "sealed"))).collect::<Vec<_>>().join(", ");
        let exports = r["exports"].as_array().unwrap().iter().map(|m| format!("({}, {}, {})", s(m, "context"), m["length"], s(m, "value"))).collect::<Vec<_>>().join(", ");
        format!("hpke-row (Names {} {} {} {}) (Inputs {} {} {} {} {} {}) (Keys {} {} {} {} {} {}) [{}] [{}]",
            s(r, "kem"), s(r, "kdf"), s(r, "aead"), s(r, "mode"), s(r, "info"), s(r, "psk"), s(r, "psk_id"), s(r, "ikm_r"), s(r, "ikm_s"), s(r, "ikm_e"),
            s(r, "sk_r"), s(r, "pk_r"), s(r, "sk_s"), s(r, "pk_s"), s(r, "sk_e"), s(r, "enc"), messages, exports)
    }).collect::<Vec<_>>();
    calls.push("refusals".into());
    for batch in calls.chunks(30) {
        run_batch(batch);
    }
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn hpke_declarations_have_canonical_documentation() {
    common::assert_documented("Crypto.Hpke", include_str!("../prelude/Crypto/Hpke.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference HPKE consumers.
module Main where
  import Crypto.Hpke
  import Text.Hex
  -- | Bytes from hex.
  b :: String -> Bytes
  b text = case hex-decode text of Ok v -> v; Err e -> bytes-new 0
  -- | Suite and mode names.
  data Names where
    -- | KEM, KDF, AEAD and mode.
    Names :: String -> String -> String -> String -> Names
  -- | Info, PSK, PSK identifier and the three keying materials.
  data Inputs where
    -- | Fields in the order listed above.
    Inputs :: String -> String -> String -> String -> String -> String -> Inputs
  -- | Expected recipient, sender and ephemeral keys and the encapsulated key.
  data Keys where
    -- | Fields: recipient private and public, sender private and public, ephemeral private, encapsulated key.
    Keys :: String -> String -> String -> String -> String -> String -> Keys
  -- | The named KEM.
  kem-of :: String -> HpkeKem
  kem-of n = if n == "p256" then HpkeDhkemP256 else HpkeDhkemX25519
  -- | The named suite.
  suite-of :: Names -> HpkeSuite
  suite-of n = case n of
    Names kem kdf aead mode ->
      let k = (if kdf == "sha384" then HpkeHkdfSha384 else if kdf == "sha512" then HpkeHkdfSha512 else HpkeHkdfSha256) in
      let a = (if aead == "aes128gcm" then HpkeAes128Gcm else if aead == "aes256gcm" then HpkeAes256Gcm else if aead == "chacha20poly1305" then HpkeChaCha20Poly1305 else HpkeExportOnly) in
      hpke-suite (kem-of kem) k a
  -- | Whether the mode is authenticated.
  auth :: Names -> Bool
  auth n = case n of Names kem kdf aead mode -> mode == "auth" || mode == "authpsk"
  -- | The PSK pair when the mode uses one.
  psk-of :: Names -> Inputs -> Option (Bytes, Bytes)
  psk-of n i = case (n, i) of
    (Names kem kdf aead mode, Inputs info psk id r s e) -> if mode == "psk" || mode == "authpsk" then Some (b psk, b id) else None
  -- | A derived key and its expected encodings.
  derives :: HpkeKem -> String -> String -> String -> Bool
  derives kem ikm sk pk = case hpke-derive-key-pair kem (b ikm) of
    Ok k -> hex-encode (hpke-private-key-bytes k) == sk && (pk == "" || hex-encode (hpke-public-key-bytes (hpke-public-key-of k)) == pk)
    Err e -> False
  -- | Derive a key that the row has already checked.
  key :: HpkeKem -> String -> HpkePrivateKey
  key kem ikm = case hpke-derive-key-pair kem (b ikm) of
    Ok k -> k
    Err e -> key kem "00"
  -- | One fixture row.
  hpke-row :: Names -> Inputs -> Keys -> List (String, String, String) -> List (String, Nat, String) -> Bool
  hpke-row n i k messages exports = case (i, k) of
    (Inputs info psk id ir is ie, Keys skr pkr sks pks ske enc) ->
      let s = suite-of n in
      let kem = hpke-suite-kem s in
      let kr = key kem ir in
      let ks = key kem is in
      derives kem ir skr pkr && derives kem is sks pks && derives kem ie ske ""
        && (case hpke-setup-sender s (hpke-public-key-of kr) (b info) (if auth n then Some ks else None) (psk-of n i) (key kem ie) of
          Err e -> False
          Ok setup -> (case setup of
            (got, sender) -> hex-encode got == enc
              && (case hpke-setup-recipient s kr got (b info) (if auth n then Some (hpke-public-key-of ks) else None) (psk-of n i) of
                Err e -> False
                Ok recipient -> conversation sender recipient messages && exported sender recipient exports && export-only s sender)))
  -- | Seal and open each message in turn; a tampered seal does not open.
  conversation :: HpkeContext -> HpkeContext -> List (String, String, String) -> Bool
  conversation sender recipient messages = case messages of
    Nil -> True
    Cons m rest -> (case m of
      (aad, pt, sealed) -> (case (hpke-seal sender (b aad) (b pt), hpke-open recipient (b aad) (b sealed)) of
        (Ok sent, Ok got) -> (case (sent, got) of
          ((ct, s2), (opened, r2)) -> hex-encode ct == sealed && hex-encode opened == pt
            && (case hpke-open recipient (b (string-concat aad "01")) (b sealed) of Err HpkeOpenFailed -> True; other -> False)
            && hpke-sequence s2 == hpke-sequence sender + 1 && conversation s2 r2 rest)
        other -> False))
  -- | Both contexts export the expected secrets.
  exported :: HpkeContext -> HpkeContext -> List (String, Nat, String) -> Bool
  exported sender recipient exports = case exports of
    Nil -> True
    Cons x rest -> (case x of
      (ctx, n, value) -> (case (hpke-export sender (b ctx) n, hpke-export recipient (b ctx) n) of
        (Ok a, Ok c) -> hex-encode a == value && hex-encode c == value && exported sender recipient rest
        other -> False))
  -- | Export-only suites refuse to seal.
  export-only :: HpkeSuite -> HpkeContext -> Bool
  export-only s ctx = case hpke-suite-aead s of
    HpkeExportOnly -> (case hpke-seal ctx (bytes-new 0) (bytes-new 1) of Err HpkeExportOnlyAead -> True; other -> False)
    other -> True
  -- | Mismatched suites, malformed keys and encapsulations, inconsistent PSKs and long exports are refused.
  refusals :: Bool
  refusals =
    let x = key HpkeDhkemX25519 "01" in
    let p = key HpkeDhkemP256 "02" in
    let sx = hpke-suite HpkeDhkemX25519 HpkeHkdfSha256 HpkeAes128Gcm in
    let sp = hpke-suite HpkeDhkemP256 HpkeHkdfSha256 HpkeAes128Gcm in
    (case hpke-setup-sender sx (hpke-public-key-of p) (bytes-new 0) None None x of Err HpkeKeyMismatch -> True; other -> False)
    && (case hpke-setup-sender sx (hpke-public-key-of x) (bytes-new 0) None (Some (bytes-new 32, bytes-new 0)) x of Err HpkeInconsistentPsk -> True; other -> False)
    && (case hpke-setup-recipient sp p (bytes-new 64) (bytes-new 0) None None of Err HpkeInvalidEncapsulation -> True; other -> False)
    && (case hpke-setup-recipient sp p (bytes-new 65) (bytes-new 0) None None of Err HpkeInvalidEncapsulation -> True; other -> False)
    && (case hpke-setup-recipient sx x (bytes-new 32) (bytes-new 0) None None of Err HpkeInvalidEncapsulation -> True; other -> False)
    && (case hpke-public-key HpkeDhkemP256 (bytes-new 65) of Err HpkeInvalidKey -> True; other -> False)
    && (case hpke-private-key HpkeDhkemP256 (bytes-new 32) of Err HpkeInvalidKey -> True; other -> False)
    && (case hpke-setup-sender sx (hpke-public-key-of x) (bytes-new 0) None None x of
      Ok setup -> (case setup of (enc, ctx) -> (case hpke-export ctx (bytes-new 0) 8161 of Err HpkeExportTooLong -> True; other -> False))
      Err e -> False)
  -- | The index of the first failing row, or 200.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
