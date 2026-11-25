//! DES, Triple DES and RC4 against the cryptography library (`tooling/des_rc4_vectors.py`):
//! known answers, ECB, CBC and CTR (including counter wrap) for 8- and 24-byte keys, and RC4
//! keystreams for keys of 1 to 256 bytes consumed in two calls, with replay from a retained
//! stream. The DES kernels also lower for other targets.
mod common;

use serde_json::Value;

/// Compile and run one batch of fixture rows; the exit status is the first failing row, 200 if none.
fn run_batch(rows: &[String]) {
    let dir = common::project("des_rc4", &[
        ("Crypto.Des.cinch", include_str!("../prelude/Crypto/Des.cinch")),
        ("Crypto.Rc4.cinch", include_str!("../prelude/Crypto/Rc4.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every row in this batch.\n  main :: Int\n  main = first [{}] 0\n", rows.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing row index in the batch; 200 means every row passed");
    common::cleanup(&dir);
}

/// DES and Triple DES encrypt and decrypt every reference row in every mode.
#[test]
fn des_matches_reference_in_ecb_cbc_and_ctr() {
    let f: Value = serde_json::from_str(include_str!("fixtures/des-rc4-vectors.json")).unwrap();
    let mut rows = f["des"].as_array().unwrap().iter().map(|r| format!("des {:?} {:?} {:?} {:?} {:?}", r["mode"].as_str().unwrap(), r["key"].as_str().unwrap(), r["iv"].as_str().unwrap(), r["plain"].as_str().unwrap(), r["cipher"].as_str().unwrap())).collect::<Vec<_>>();
    rows.push("errors ()".to_string());
    run_batch(&rows);
}

/// RC4 keystreams match across split calls, and a retained stream replays.
#[test]
fn rc4_matches_reference_keystreams() {
    let f: Value = serde_json::from_str(include_str!("fixtures/des-rc4-vectors.json")).unwrap();
    let rows = f["rc4"].as_array().unwrap().iter().map(|r| format!("rc4 {:?} {:?} {:?} {}", r["key"].as_str().unwrap(), r["plain"].as_str().unwrap(), r["cipher"].as_str().unwrap(), r["split"])).collect::<Vec<_>>();
    run_batch(&rows);
}

/// The DES kernels lower for 64-bit ARM, WebAssembly and a big-endian target.
#[test]
fn des_kernels_compile_for_arm_wasm_and_big_endian_targets() {
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
            ("Crypto.Des", include_str!("../prelude/Crypto/Des.cinch")),
            ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
            ("Text.Binary", include_str!("../prelude/Text/Binary.cinch")),
            ("Data.Bytes.Build", include_str!("../prelude/Data/Bytes/Build.cinch")),
            ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), text).unwrap();
        }
        std::fs::write(project.join("Main.cinch"), "module Main where\n  import Crypto.Des\n  -- | Reach the kernels.\n  main :: Int\n  main = case des-cipher (bytes-new 8) of\n    Ok c -> (case des-encrypt-block c (bytes-new 8) of Ok b -> 0; Err e -> 1)\n    Err e -> 2\n").unwrap();
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
        assert!(ir.contains("call i32 @llvm.fshr.i32") && ir.contains("call i32 @llvm.fshl.i32"));
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
fn des_rc4_declarations_have_canonical_documentation() {
    common::assert_documented("Crypto.Des", include_str!("../prelude/Crypto/Des.cinch"));
    common::assert_documented("Crypto.Rc4", include_str!("../prelude/Crypto/Rc4.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference cipher vector consumers.
module Main where
  import Crypto.Des
  import Crypto.Rc4
  import Text.Hex
  import Data.Bytes.Build
  -- | Bytes from hex.
  b :: String -> Bytes
  b text = case hex-decode text of Ok v -> v; Err e -> bytes-new 0
  -- | The cipher for a key length.
  keyed :: Bytes -> Result DesError DesCipher
  keyed key = if bytes-len key == 8 then des-cipher key else triple-des-cipher key
  -- | A result equals the expected hex.
  is :: Result DesError Bytes -> String -> Bool
  is r want = case r of Ok v -> hex-encode v == want; Err e -> False
  -- | One DES row, both directions.
  des :: String -> String -> String -> String -> String -> Bool
  des mode key iv plain cipher = case keyed (b key) of
    Err e -> False
    Ok c ->
      if mode == "ecb" then is (des-encrypt-ecb c (b plain)) cipher && is (des-decrypt-ecb c (b cipher)) plain
        && (if bytes-len (b plain) == 8 then is (des-encrypt-block c (b plain)) cipher && is (des-decrypt-block c (b cipher)) plain else True)
      else if mode == "cbc" then is (des-cbc-encrypt c (b iv) (b plain)) cipher && is (des-cbc-decrypt c (b iv) (b cipher)) plain
      else is (des-ctr-xor c (b iv) (b plain)) cipher && is (des-ctr-xor c (b iv) (b cipher)) plain
  -- | Misuse is rejected by name.
  errors :: Unit -> Bool
  errors unit =
    (case des-cipher (bytes-new 7) of Err (DesInvalidKeySize 7) -> True; other -> False)
      && (case triple-des-cipher (bytes-new 16) of Err (DesInvalidKeySize 16) -> True; other -> False)
      && (case des-cipher (bytes-new 8) of
        Err e -> False
        Ok c -> (case des-encrypt-ecb c (bytes-new 9) of Err (DesInvalidInputSize 9) -> True; other -> False)
          && (case des-cbc-encrypt c (bytes-new 4) (bytes-new 8) of Err (DesInvalidIvSize 4) -> True; other -> False)
          && (case des-encrypt-block c (bytes-new 16) of Err (DesInvalidInputSize 16) -> True; other -> False))
      && (case rc4-new (bytes-new 0) of Err (Rc4InvalidKeySize 0) -> True; other -> False)
      && (case rc4-new (bytes-new 257) of Err (Rc4InvalidKeySize 257) -> True; other -> False)
      && des-error-message (DesInvalidKeySize 7) == "des: invalid key size 7"
      && rc4-error-message (Rc4InvalidKeySize 0) == "rc4: invalid key size 0"
  -- | The first n bytes.
  prefix :: Bytes -> Nat -> Bytes
  prefix v n = let out = bytes-new n in if n <= bytes-len v then let c = bytes-copy-range out v 0 0 n in out else out
  -- | The bytes from n on.
  suffix :: Bytes -> Nat -> Bytes
  suffix v n = if n <= bytes-len v then (let out = bytes-new (bytes-len v - n) in let c = bytes-copy-range out v 0 n (bytes-len v - n) in out) else bytes-new 0
  -- | One RC4 row: two calls give the reference keystream, and the first stream replays it.
  rc4 :: String -> String -> String -> Nat -> Bool
  rc4 key plain cipher split = case rc4-new (b key) of
    Err e -> False
    Ok s0 -> case rc4-xor s0 (prefix (b plain) split) of
      (s1, a) -> case rc4-xor s1 (suffix (b plain) split) of
        (s2, z) -> hex-encode (bytes-concat [a, z]) == cipher
          && (case rc4-xor s0 (b plain) of (s3, again) -> hex-encode again == cipher)
  -- | Index of the first failing row, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
