//! ChaCha20, Poly1305, ChaCha20-Poly1305 and XChaCha20-Poly1305 against independent implementations
//! (`tooling/chacha_vectors.py`: the cryptography library and libsodium): keystreams at every length
//! around block boundaries and counters near the 32-bit limit, including the RFC 8439 sample; Poly1305 tags
//! for random and carry-stressing keys and messages; seals that match byte for byte, open again, and refuse
//! tampered ciphertext, tags and additional data. The kernels also lower for other targets.
mod common;

use serde_json::Value;

/// Compile and run one batch of fixture rows; the exit status is the first failing row, 200 if none.
fn run_batch(rows: &[String]) {
    let dir = common::project("chacha", &[
        ("Crypto.ChaCha20Poly1305.cinch", include_str!("../prelude/Crypto/ChaCha20Poly1305.cinch")),
        ("Crypto.ConstantTime.cinch", include_str!("../prelude/Crypto/ConstantTime.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every row in this batch.\n  main :: Int\n  main = first [{}] 0\n", rows.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing row index in the batch; 200 means every row passed");
    common::cleanup(&dir);
}

/// The fixture.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/chacha-vectors.json")).unwrap()
}

/// A string field as a Cinch literal.
fn s(v: &Value, k: &str) -> String {
    format!("{:?}", v[k].as_str().unwrap())
}

/// Keystreams and Poly1305 tags match the references.
#[test]
fn chacha20_and_poly1305_match_reference() {
    let f = fixture();
    let mut rows = f["streams"].as_array().unwrap().iter().map(|r| format!("xors {} {} {} {} {}", s(r, "key"), s(r, "nonce"), r["counter"], s(r, "input"), s(r, "output"))).collect::<Vec<_>>();
    rows.extend(f["macs"].as_array().unwrap().iter().map(|r| format!("macs {} {} {}", s(r, "key"), s(r, "message"), s(r, "tag"))));
    for batch in rows.chunks(60) {
        run_batch(batch);
    }
}

/// Both AEADs seal byte for byte, open, and refuse tampering.
#[test]
fn chacha20poly1305_seals_match_reference_and_refuse_tampering() {
    let f = fixture();
    let mut rows = f["seals"].as_array().unwrap().iter().map(|r| format!("seals {} {} {} {} {}", s(r, "key"), s(r, "nonce"), s(r, "plaintext"), s(r, "aad"), s(r, "sealed"))).collect::<Vec<_>>();
    rows.extend(f["xseals"].as_array().unwrap().iter().map(|r| format!("xseals {} {} {} {} {}", s(r, "key"), s(r, "nonce"), s(r, "plaintext"), s(r, "aad"), s(r, "sealed"))));
    rows.push("refuses".into());
    run_batch(&rows);
}

/// The kernels lower for 64-bit ARM, WebAssembly and a big-endian target.
#[test]
fn chacha_kernels_compile_for_arm_wasm_and_big_endian_targets() {
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
            ("Crypto.ChaCha20Poly1305", include_str!("../prelude/Crypto/ChaCha20Poly1305.cinch")),
            ("Crypto.ConstantTime", include_str!("../prelude/Crypto/ConstantTime.cinch")),
            ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), text).unwrap();
        }
        std::fs::write(project.join("Main.cinch"), "module Main where\n  import Crypto.ChaCha20Poly1305\n  -- | Reach the kernels.\n  main :: Int\n  main = case xchacha20poly1305-seal (bytes-new 32) (bytes-new 24) (bytes-new 70) (bytes-new 3) of\n    Ok sealed -> (case xchacha20poly1305-open (bytes-new 32) (bytes-new 24) sealed (bytes-new 3) of Ok p -> (if bytes-len p == 70 then 0 else 3); Err e -> 1)\n    Err e -> 2\n").unwrap();
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
        assert!(ir.contains("<4 x i32>") && ir.contains("mul i128"));
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
fn chacha_declarations_have_canonical_documentation() {
    common::assert_documented("Crypto.ChaCha20Poly1305", include_str!("../prelude/Crypto/ChaCha20Poly1305.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference ChaCha20 and Poly1305 consumers.
module Main where
  import Crypto.ChaCha20Poly1305
  import Text.Hex
  -- | Bytes from hex.
  b :: String -> Bytes
  b text = case hex-decode text of Ok v -> v; Err e -> bytes-new 0
  -- | A copy with byte i flipped.
  flip :: Bytes -> Nat -> Bytes
  flip x i =
    let out = bytes-new (bytes-len x) in
    let c = (if bytes-len x <= bytes-len out then bytes-copy-range out x 0 0 (bytes-len x) else ()) in
    let v = nat-rem (bytes-get-or out i + 128) 256 in
    let f = (if i < bytes-len out && v < 256 then bytes-set out i v else ()) in
    out
  -- | A keystream row: the output, and XOR again restores the input.
  xors :: String -> String -> Nat -> String -> String -> Bool
  xors key nonce counter input output = case chacha20-xor (b key) (b nonce) counter (b input) of
    Ok out -> hex-encode out == output && (case chacha20-xor (b key) (b nonce) counter out of Ok back -> hex-encode back == input; Err e -> False)
    Err e -> False
  -- | A Poly1305 row: the tag, verification, and refusal of a flipped tag.
  macs :: String -> String -> String -> Bool
  macs key message tag = case poly1305-mac (b key) (b message) of
    Ok t -> hex-encode t == tag && poly1305-verify (b key) (b message) (b tag) && not (poly1305-verify (b key) (b message) (flip (b tag) 3))
    Err e -> False
  -- | The byte at an index, zero outside the buffer.
  bytes-get-or :: Bytes -> Nat -> Nat
  bytes-get-or x i = if i < bytes-len x then bytes-get x i else 0
  -- | The last index of a non-empty buffer.
  last-index :: Bytes -> Nat
  last-index x = if bytes-len x > 0 then bytes-len x - 1 else 0
  -- | Whether opening fails with the authentication error.
  refused :: Result ChaChaError Bytes -> Bool
  refused r = case r of Err ChaChaAuthentication -> True; other -> False
  -- | A ChaCha20-Poly1305 row.
  seals :: String -> String -> String -> String -> String -> Bool
  seals key nonce pt aad sealed = case chacha20poly1305-seal (b key) (b nonce) (b pt) (b aad) of
    Err e -> False
    Ok s -> hex-encode s == sealed
      && (case chacha20poly1305-open (b key) (b nonce) s (b aad) of Ok p -> hex-encode p == pt; Err e -> False)
      && refused (chacha20poly1305-open (b key) (b nonce) (flip s 0) (b aad))
      && refused (chacha20poly1305-open (b key) (b nonce) (flip s (last-index s)) (b aad))
      && refused (chacha20poly1305-open (b key) (b nonce) s (flip (b (string-concat aad "00")) 0))
  -- | An XChaCha20-Poly1305 row; the ciphertext is also the XChaCha20 keystream from counter 1.
  xseals :: String -> String -> String -> String -> String -> Bool
  xseals key nonce pt aad sealed = case xchacha20poly1305-seal (b key) (b nonce) (b pt) (b aad) of
    Err e -> False
    Ok s -> hex-encode s == sealed
      && (case xchacha20poly1305-open (b key) (b nonce) s (b aad) of Ok p -> hex-encode p == pt; Err e -> False)
      && refused (xchacha20poly1305-open (b key) (b nonce) (flip s 0) (b aad))
      && (case xchacha20-xor (b key) (b nonce) 1 (b pt) of Ok c -> string-concat (hex-encode c) "" == string-slice sealed 0 (2 * bytes-len (b pt)); Err e -> False)
  -- | Malformed keys, nonces, short seals and counter overflow are refused.
  refuses :: Bool
  refuses =
    (case chacha20-xor (bytes-new 31) (bytes-new 12) 0 (bytes-new 1) of Err (ChaChaKeySize n) -> n == 31; other -> False)
    && (case chacha20-xor (bytes-new 32) (bytes-new 8) 0 (bytes-new 1) of Err (ChaChaNonceSize n) -> n == 8; other -> False)
    && (case chacha20-xor (bytes-new 32) (bytes-new 12) 4294967295 (bytes-new 65) of Err ChaChaTooLarge -> True; other -> False)
    && refused (chacha20poly1305-open (bytes-new 32) (bytes-new 12) (bytes-new 15) (bytes-new 0))
    && (case xchacha20poly1305-seal (bytes-new 32) (bytes-new 12) (bytes-new 1) (bytes-new 0) of Err (ChaChaNonceSize n) -> n == 12; other -> False)
    && (case poly1305-mac (bytes-new 16) (bytes-new 1) of Err (ChaChaKeySize n) -> n == 16; other -> False)
  -- | The index of the first failing row, or 200.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
