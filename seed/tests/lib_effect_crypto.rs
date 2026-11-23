//! Randomized key generation and random-nonce GCM: generated keys sign, verify and agree,
//! independent generations differ, and random-nonce messages open and reject tampering.
mod common;

/// Generate, use and cross-check keys and sealed messages in one native run.
#[test]
fn generated_keys_and_random_nonces_work_end_to_end() {
    let dir = common::project("effect_crypto", &[
        ("Effect.Crypto.cinch", include_str!("../prelude/Effect/Crypto.cinch")),
        ("Effect.Random.cinch", include_str!("../prelude/Effect/Random.cinch")),
        ("Crypto.Aes.cinch", include_str!("../prelude/Crypto/Aes.cinch")),
        ("Crypto.Cipher.cinch", include_str!("../prelude/Crypto/Cipher.cinch")),
        ("Crypto.Ed25519.cinch", include_str!("../prelude/Crypto/Ed25519.cinch")),
        ("Crypto.X25519.cinch", include_str!("../prelude/Crypto/X25519.cinch")),
        ("Crypto.Field25519.cinch", include_str!("../prelude/Crypto/Field25519.cinch")),
        ("Text.Sha512.cinch", include_str!("../prelude/Text/Sha512.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Main.cinch", MAIN),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 0);
    common::cleanup(&dir);
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn effect_crypto_declarations_have_canonical_documentation() {
    common::assert_documented("Effect.Crypto", include_str!("../prelude/Effect/Crypto.cinch"));
}

const MAIN: &str = r#"-- | End-to-end randomized crypto checks.
module Main where
  import Effect.Crypto
  import Crypto.Aes
  import Crypto.Cipher
  import Crypto.Ed25519
  import Crypto.X25519
  -- | Byte equality.
  same :: Bytes -> Bytes -> Bool
  same a b = eq-i64 (bytes-compare a b) 0
  -- | Agreement between two generated keys.
  agree :: X25519PrivateKey -> X25519PrivateKey -> Bool
  agree a b = case (x25519-shared a (x25519-public-key b), x25519-shared b (x25519-public-key a)) of
    (Ok s1, Ok s2) -> same s1 s2
    other -> False
  -- | Run the checks.
  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in
    let k1 = ed25519-generate () in
    let k2 = ed25519-generate () in
    let message = string-to-bytes "generated keys sign" in
    let signed = ed25519-verify (ed25519-public-key k1) message (ed25519-sign k1 message) && not (same (ed25519-seed k1) (ed25519-seed k2)) in
    let x1 = x25519-generate () in
    let x2 = x25519-generate () in
    let agreed = agree x1 x2 in
    case aes-generate-key 32 of
      Err e -> 3
      Ok key ->
        let g = gcm (aes-cipher key) in
        (case (gcm-seal-random g message (bytes-new 2), gcm-seal-random g message (bytes-new 2)) of
          (Ok a, Ok b) ->
            let opened = case gcm-open-random g a (bytes-new 2) of Ok out -> same out message; Err e -> False in
            let distinct = not (same a b) && bytes-len a == 12 + bytes-len message + 16 in
            let rejected = case gcm-open-random g a (bytes-new 3) of Err AuthenticationFailed -> True; other -> False in
            if signed && agreed && opened && distinct && rejected then 0 else 1
          other -> 2)
"#;
