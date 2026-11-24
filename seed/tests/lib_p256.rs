//! P-256 ECDSA and ECDH against the cryptography library (`tooling/p256_vectors.py`): key
//! derivation and both point encodings, deterministic RFC 6979 signatures (raw and DER) over
//! SHA-256 digests, verification of randomized and malleated signatures over digests of four
//! sizes and rejection of tampered ones, ECDH agreement, compressed-point decoding, and
//! rejection of malformed keys, points and DER. The limb kernels also lower for other targets.
mod common;

use serde_json::Value;

/// Compile and run one batch of fixture rows; the exit status is the first failing row, 200 if none.
fn run_batch(rows: &[String]) {
    let dir = common::project("p256", &[
        ("Crypto.Ecdsa.cinch", include_str!("../prelude/Crypto/Ecdsa.cinch")),
        ("Crypto.P256.cinch", include_str!("../prelude/Crypto/P256.cinch")),
        ("Crypto.Hmac.cinch", include_str!("../prelude/Crypto/Hmac.cinch")),
        ("Crypto.ConstantTime.cinch", include_str!("../prelude/Crypto/ConstantTime.cinch")),
        ("Text.Digest.cinch", include_str!("../prelude/Text/Digest.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every row in this batch.\n  main :: Int\n  main = first [{}] 0\n", rows.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing row index in the batch; 200 means every row passed");
    common::cleanup(&dir);
}

/// The fixture.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/p256-vectors.json")).unwrap()
}

/// A string field.
fn s(v: &Value, k: &str) -> String {
    format!("{:?}", v[k].as_str().unwrap())
}

/// Keys, deterministic signatures and ECDH match the reference.
#[test]
fn p256_keys_signatures_and_agreement_match_reference() {
    let f = fixture();
    let mut rows = f["keys"].as_array().unwrap().iter().map(|r| format!("key {} {} {}", s(r, "d"), s(r, "public"), s(r, "compressed"))).collect::<Vec<_>>();
    rows.extend(f["signs"].as_array().unwrap().iter().map(|r| format!("signs {} {} {} {}", s(r, "d"), s(r, "digest"), s(r, "rs"), s(r, "der"))));
    rows.extend(f["ecdh"].as_array().unwrap().iter().map(|r| format!("agrees {} {} {}", s(r, "d"), s(r, "peer"), s(r, "shared"))));
    for batch in rows.chunks(31) {
        run_batch(batch);
    }
}

/// Verification accepts valid and malleated signatures, rejects tampered digests, and malformed inputs are refused.
#[test]
fn p256_verification_and_malformed_inputs() {
    let f = fixture();
    let mut rows = f["verifies"].as_array().unwrap().iter().map(|r| format!("verifies {} {} {} {} {}", s(r, "public"), s(r, "digest"), s(r, "der"), s(r, "rs"), if r["valid"].as_bool().unwrap() { "True" } else { "False" })).collect::<Vec<_>>();
    rows.extend(f["bad_points"].as_array().unwrap().iter().map(|v| format!("bad-point {:?}", v.as_str().unwrap())));
    rows.extend(f["bad_keys"].as_array().unwrap().iter().map(|v| format!("bad-key {:?}", v.as_str().unwrap())));
    let sign0 = &f["signs"][0];
    rows.extend(f["bad_ders"].as_array().unwrap().iter().map(|v| format!("bad-der {} {} {:?}", s(sign0, "d"), s(sign0, "digest"), v.as_str().unwrap())));
    for batch in rows.chunks(27) {
        run_batch(batch);
    }
}

/// The limb kernels lower for 64-bit ARM, WebAssembly and a big-endian target.
#[test]
fn p256_kernels_compile_for_arm_wasm_and_big_endian_targets() {
    let directory = tempfile::tempdir().unwrap();
    for (triple, arch, endian) in [
        ("aarch64-unknown-linux-gnu", "aarch64", "little"),
        ("wasm32-unknown-unknown", "wasm32", "little"),
        ("powerpc64-unknown-linux-gnu", "powerpc64", "big"),
    ] {
        let profile = cinch::driver::target_profile(&cinch::driver::DriverOptions { target: Some(triple.into()), ..Default::default() })
            .unwrap_or_else(|_| {
                let mut profile = cinch::core::TargetProfile::native();
                profile.triple = triple.into();
                profile.arch = arch.into();
                profile.endian = endian.into();
                profile
            });
        let project = directory.path().join(arch);
        std::fs::create_dir_all(&project).unwrap();
        for (name, text) in [
            ("Crypto.Ecdsa", include_str!("../prelude/Crypto/Ecdsa.cinch")),
            ("Crypto.P256", include_str!("../prelude/Crypto/P256.cinch")),
            ("Crypto.Hmac", include_str!("../prelude/Crypto/Hmac.cinch")),
            ("Crypto.ConstantTime", include_str!("../prelude/Crypto/ConstantTime.cinch")),
            ("Text.Digest", include_str!("../prelude/Text/Digest.cinch")),
            ("Data.Bytes.Build", include_str!("../prelude/Data/Bytes/Build.cinch")),
            ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), text).unwrap();
        }
        std::fs::write(project.join("Main.cinch"), "module Main where\n  import Crypto.P256\n  import Crypto.Ecdsa\n  -- | Reach the kernels.\n  main :: Int\n  main = case p256-private-key (bytes-from-u8s [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7]) of\n    Ok k -> (case ecdsa-sign k (bytes-new 32) of Ok s -> (if ecdsa-verify (p256-public-key-of k) (bytes-new 32) s then 0 else 3); Err e -> 1)\n    Err e -> 2\n").unwrap();
        let ir_path = project.join("program.ll");
        let entry = project.join("Main.cinch");
        let ir = std::thread::Builder::new()
            .stack_size(256 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::driver_with_profile_opts(cinch::driver::Mode::EmitIr, cinch::check::StageMode::Refresh, &entry, Some(&ir_path), &profile, &cinch::driver::DriverOptions::default())
                    .map(|_| std::fs::read_to_string(&ir_path).unwrap())
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("mul i128"));
        let output = std::process::Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
            .args(["-target", triple, "-O2", "-c", "-x", "ir"])
            .arg(project.join("program.ll"))
            .arg("-o")
            .arg(project.join("program.o"))
            .output()
            .unwrap();
        assert!(output.status.success(), "{triple}: {}", String::from_utf8_lossy(&output.stderr));
    }
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn p256_declarations_have_canonical_documentation() {
    common::assert_documented("Crypto.P256", include_str!("../prelude/Crypto/P256.cinch"));
    common::assert_documented("Crypto.Ecdsa", include_str!("../prelude/Crypto/Ecdsa.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference P-256 consumers.
module Main where
  import Crypto.P256
  import Crypto.Ecdsa
  import Text.Hex
  -- | Bytes from hex.
  b :: String -> Bytes
  b text = case hex-decode text of Ok v -> v; Err e -> bytes-new 0
  -- | A key row: the public key in both encodings, and parsing both back.
  key :: String -> String -> String -> Bool
  key d public compressed = case p256-private-key (b d) of
    Err e -> False
    Ok k ->
      let pub = p256-public-key-of k in
      hex-encode (p256-public-key-bytes pub) == public && hex-encode (p256-public-key-compressed pub) == compressed
        && (case p256-public-key (b compressed) of Ok q -> hex-encode (p256-public-key-bytes q) == public; Err e -> False)
        && (case p256-public-key (b public) of Ok q -> hex-encode (p256-public-key-compressed q) == compressed; Err e -> False)
        && hex-encode (p256-private-key-bytes k) == d
  -- | A deterministic signature row.
  signs :: String -> String -> String -> String -> Bool
  signs d digest rs der = case p256-private-key (b d) of
    Err e -> False
    Ok k ->
      (case ecdsa-sign k (b digest) of Ok sig -> hex-encode sig == rs; Err e -> False)
        && (case ecdsa-sign-der k (b digest) of Ok sig -> hex-encode sig == der; Err e -> False)
        && ecdsa-verify-der (p256-public-key-of k) (b digest) (b der)
  -- | An ECDH row.
  agrees :: String -> String -> String -> Bool
  agrees d peer shared = case (p256-private-key (b d), p256-public-key (b peer)) of
    (Ok k, Ok q) -> (case p256-ecdh k q of Ok sec -> hex-encode sec == shared; Err e -> False)
    other -> False
  -- | A verification row: the raw form (possibly with s replaced by n - s) and the original DER agree with the expectation.
  verifies :: String -> String -> String -> String -> Bool -> Bool
  verifies public digest der rs valid = case p256-public-key (b public) of
    Err e -> False
    Ok q -> ecdsa-verify q (b digest) (b rs) == valid && ecdsa-verify-der q (b digest) (b der) == valid
  -- | A malformed point is rejected.
  bad-point :: String -> Bool
  bad-point p = case p256-public-key (b p) of Err P256InvalidPublicKey -> True; other -> False
  -- | A malformed private key is rejected.
  bad-key :: String -> Bool
  bad-key d = case p256-private-key (b d) of Err P256InvalidPrivateKey -> True; other -> False
  -- | A malformed DER signature fails verification.
  bad-der :: String -> String -> String -> Bool
  bad-der d digest der = case p256-private-key (b d) of Ok k -> not (ecdsa-verify-der (p256-public-key-of k) (b digest) (b der)); Err e -> False
  -- | Index of the first failing row, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
