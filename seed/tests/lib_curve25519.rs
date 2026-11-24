//! X25519 key agreement and Ed25519 signatures against RFC 7748 and RFC 8032 vectors and
//! independent exchanges and signatures from `tooling/curve25519_vectors.py` (OpenSSL through
//! the Python cryptography package), plus constructed rejections.
mod common;

use serde_json::Value;

/// Compile and run checks; the exit status is the first failing check, 200 if none.
fn run(checks: &[String]) {
    let dir = common::project("curve25519", &[
        ("Crypto.X25519.cinch", include_str!("../prelude/Crypto/X25519.cinch")),
        ("Crypto.Ed25519.cinch", include_str!("../prelude/Crypto/Ed25519.cinch")),
        ("Crypto.Field25519.cinch", include_str!("../prelude/Crypto/Field25519.cinch")),
        ("Text.Sha512.cinch", include_str!("../prelude/Text/Sha512.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every check.\n  main :: Int\n  main = first [{}] 0\n", checks.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing check; 200 means every check passed");
    common::cleanup(&dir);
}

/// Read the fixture.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/curve25519-vectors.json")).unwrap()
}

/// Exchanges, public keys, non-canonical peer coordinates, low-order rejection and the iterated RFC test.
#[test]
fn x25519_matches_rfc_and_independent_exchanges() {
    let f = fixture();
    let mut checks = f["exchanges"].as_array().unwrap().iter()
        .map(|r| format!("exchange {:?} {:?} {:?}", r["scalar"].as_str().unwrap(), r["point"].as_str().unwrap(), r["shared"].as_str().unwrap()))
        .collect::<Vec<_>>();
    checks.extend(f["keys"].as_array().unwrap().iter().map(|r| format!("key-case {:?} {:?}", r["secret"].as_str().unwrap(), r["public"].as_str().unwrap())));
    let u = &f["unreduced"];
    checks.push(format!("exchange {:?} {:?} {:?}", u["scalar"].as_str().unwrap(), u["point"].as_str().unwrap(), u["shared"].as_str().unwrap()));
    checks.extend(f["low_order"].as_array().unwrap().iter().map(|p| format!("low-order {:?}", p.as_str().unwrap())));
    checks.push("iterate 1 (input \"0900000000000000000000000000000000000000000000000000000000000000\") (input \"0900000000000000000000000000000000000000000000000000000000000000\") == \"422c8e7a6227d7bca1350b3e2bb7279f7897b87bb6854b783c60e80311ae3079\"".into());
    checks.push("iterate 1000 (input \"0900000000000000000000000000000000000000000000000000000000000000\") (input \"0900000000000000000000000000000000000000000000000000000000000000\") == \"684cf59ba83309552800ef566f2f4d3c1c3887c49360e3875f2eb94d99532c51\"".into());
    checks.push("x25519-errors ()".into());
    run(&checks);
}

/// Public keys and deterministic signatures match, signatures verify, and tampering or malformed input is rejected.
#[test]
fn ed25519_matches_rfc_and_independent_signatures_and_rejects_forgeries() {
    let f = fixture();
    let mut checks = f["signatures"].as_array().unwrap().iter()
        .map(|r| format!("sign-case {:?} {:?} {:?} {:?}", r["seed"].as_str().unwrap(), r["message"].as_str().unwrap(), r["public"].as_str().unwrap(), r["signature"].as_str().unwrap()))
        .collect::<Vec<_>>();
    checks.extend(f["rejects"].as_array().unwrap().iter().map(|r| format!("not (ed25519-verify (input {:?}) (input {:?}) (input {:?}))", r["public"].as_str().unwrap(), r["message"].as_str().unwrap(), r["signature"].as_str().unwrap())));
    checks.push("(case ed25519-private-key (bytes-new 31) of Err (InvalidEd25519SeedSize 31) -> True; other -> False)".into());
    run(&checks);
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn curve25519_declarations_have_canonical_documentation() {
    common::assert_documented("Crypto.Field25519", include_str!("../prelude/Crypto/Field25519.cinch"));
    common::assert_documented("Crypto.X25519", include_str!("../prelude/Crypto/X25519.cinch"));
    common::assert_documented("Crypto.Ed25519", include_str!("../prelude/Crypto/Ed25519.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Curve25519 vector consumers.
module Main where
  import Crypto.X25519
  import Crypto.Ed25519
  import Text.Hex
  -- | Parse recorded bytes.
  input :: String -> Bytes
  input t = case hex-decode t of Ok b -> b; Err e -> bytes-new 0
  -- | One exchange.
  exchange :: String -> String -> String -> Bool
  exchange a u o = case x25519 (input a) (input u) of Ok out -> hex-encode out == o; Err e -> False
  -- | Public key derivation through both entry points, and agreement between two keys.
  key-case :: String -> String -> Bool
  key-case secret public = case x25519-private-key (input secret) of
    Err e -> False
    Ok key -> hex-encode (x25519-public-key key) == public && hex-encode (x25519-private-bytes key) == secret
      && (case x25519-base (input secret) of Ok out -> hex-encode out == public; Err e -> False)
  -- | Low-order peers are rejected.
  low-order :: String -> Bool
  low-order point = case x25519 (input "a546e36bf0527c9d3b16154b82465edd62144c0ac1fc5a18506a2244ba449ac4") (input point) of
    Err LowOrderPeer -> True
    other -> False
  -- | The RFC 7748 iteration: (k, u) becomes (X25519(k, u), k).
  iterate :: Nat -> Bytes -> Bytes -> String
  iterate n k u = if n == 0 then hex-encode k else case x25519 k u of
    Ok out -> iterate (n - 1) out k
    Err e -> ""
  -- | Size errors and messages.
  x25519-errors :: Unit -> Bool
  x25519-errors unit =
    (case x25519 (bytes-new 31) (bytes-new 32) of Err (InvalidX25519KeySize 31) -> True; other -> False)
      && (case x25519 (bytes-new 32) (bytes-new 33) of Err (InvalidX25519PeerSize 33) -> True; other -> False)
      && (case x25519-private-key (bytes-new 1) of Err (InvalidX25519KeySize 1) -> True; other -> False)
      && x25519-error-message LowOrderPeer == "X25519 peer public key has low order"
  -- | A copy with one byte incremented.
  flip :: Bytes -> Nat -> Bytes
  flip b i = let out = bytes-new (bytes-len b) in let c = bytes-copy-range out b 0 0 (bytes-len b) in
    let v = if i < bytes-len out then nat-rem (bytes-get out i + 1) 256 else 0 in let w = if i < bytes-len out && v < 256 then bytes-set out i v else () in out
  -- | Key derivation, deterministic signing, verification and rejection of tampered messages and signatures.
  sign-case :: String -> String -> String -> String -> Bool
  sign-case seed msg pub sig = case ed25519-private-key (input seed) of
    Err e -> False
    Ok key -> hex-encode (ed25519-public-key key) == pub && hex-encode (ed25519-seed key) == seed && hex-encode (ed25519-sign key (input msg)) == sig
      && ed25519-verify (input pub) (input msg) (input sig)
      && (bytes-len (input msg) == 0 || not (ed25519-verify (input pub) (flip (input msg) 0) (input sig)))
      && not (ed25519-verify (input pub) (input msg) (flip (input sig) 5))
      && not (ed25519-verify (input pub) (input msg) (flip (input sig) 40))
      && not (ed25519-verify (flip (input pub) 3) (input msg) (input sig))
  -- | Index of the first failing check, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
