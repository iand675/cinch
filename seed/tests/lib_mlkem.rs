//! ML-KEM-768 and ML-KEM-1024 against vectors from a definition-level FIPS 203 reference
//! (`tooling/mlkem_vectors.py`) that the reference SDK confirmed (`tooling/mlkem_check.go`):
//! encapsulation keys derived from seeds, byte-exact encapsulation for explicit randomness,
//! decapsulation of those and of SDK-produced ciphertexts, implicit rejection of corrupted
//! ciphertexts, and rejection of malformed encapsulation keys. The kernels also lower for
//! other targets.
mod common;

use serde_json::Value;

/// Compile and run one batch of fixture rows; the exit status is the first failing row, 200 if none.
fn run_batch(rows: &[String]) {
    let dir = common::project("mlkem", &[
        ("Crypto.MlKem.cinch", include_str!("../prelude/Crypto/MlKem.cinch")),
        ("Crypto.ConstantTime.cinch", include_str!("../prelude/Crypto/ConstantTime.cinch")),
        ("Text.Sha3.cinch", include_str!("../prelude/Text/Sha3.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every row in this batch.\n  main :: Int\n  main = first [{}] 0\n", rows.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing row index in the batch; 200 means every row passed");
    common::cleanup(&dir);
}

/// The fixture.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/mlkem-vectors.json")).unwrap()
}

/// Seeds derive the reference keys; encapsulation, decapsulation and implicit rejection match.
#[test]
fn mlkem_matches_reference_keys_ciphertexts_and_shared_keys() {
    let f = fixture();
    let mut rows = f["rows"].as_array().unwrap().iter().map(|r| format!("full {} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", r["size"], r["seed"].as_str().unwrap(), r["m"].as_str().unwrap(), r["ek"].as_str().unwrap(), r["c"].as_str().unwrap(), r["key"].as_str().unwrap(), r["bad"].as_str().unwrap(), r["rejected"].as_str().unwrap())).collect::<Vec<_>>();
    rows.extend(f["sdk"].as_array().unwrap().iter().map(|r| format!("decaps {} {:?} {:?} {:?}", r["size"], r["seed"].as_str().unwrap(), r["c"].as_str().unwrap(), r["key"].as_str().unwrap())));
    rows.extend(f["invalid"].as_array().unwrap().iter().map(|r| format!("invalid {:?}", r["ek"].as_str().unwrap())));
    rows.push("errors ()".to_string());
    for batch in rows.chunks(10) {
        run_batch(batch);
    }
}

/// The lane kernels lower for 64-bit ARM, WebAssembly and a big-endian target.
#[test]
fn mlkem_kernels_compile_for_arm_wasm_and_big_endian_targets() {
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
            ("Crypto.MlKem", include_str!("../prelude/Crypto/MlKem.cinch")),
            ("Crypto.ConstantTime", include_str!("../prelude/Crypto/ConstantTime.cinch")),
            ("Text.Sha3", include_str!("../prelude/Text/Sha3.cinch")),
            ("Text.Binary", include_str!("../prelude/Text/Binary.cinch")),
            ("Data.Bytes.Build", include_str!("../prelude/Data/Bytes/Build.cinch")),
            ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), text).unwrap();
        }
        std::fs::write(project.join("Main.cinch"), "module Main where\n  import Crypto.MlKem\n  -- | Reach the kernels.\n  main :: Int\n  main = case mlkem-decapsulation-key MlKem768 (bytes-new 64) of\n    Ok dk -> (case mlkem-encapsulation-key MlKem768 (mlkem-encapsulation-key-bytes (mlkem-encapsulation-key-of dk)) of Ok ek -> (case mlkem-encapsulate-with ek (bytes-new 32) of Ok r -> 0; Err e -> 1); Err e -> 3)\n    Err e -> 2\n").unwrap();
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
        assert!(ir.contains("<8 x i16>") && ir.contains("<4 x i32>"));
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
fn mlkem_declarations_have_canonical_documentation() {
    common::assert_documented("Crypto.MlKem", include_str!("../prelude/Crypto/MlKem.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference ML-KEM vector consumers.
module Main where
  import Crypto.MlKem
  import Text.Hex
  -- | Bytes from hex.
  b :: String -> Bytes
  b text = case hex-decode text of Ok v -> v; Err e -> bytes-new 0
  -- | The parameter set for a size.
  params :: Nat -> MlKemParameters
  params size = if size == 768 then MlKem768 else MlKem1024
  -- | Decapsulation gives the expected key.
  opens :: MlKemDecapsulationKey -> String -> String -> Bool
  opens dk c key = case mlkem-decapsulate dk (b c) of Ok k -> hex-encode k == key; Err e -> False
  -- | A full row: key derivation, parsing, encapsulation, decapsulation and implicit rejection.
  full :: Nat -> String -> String -> String -> String -> String -> String -> String -> Bool
  full size seed m ek c key bad rejected = case mlkem-decapsulation-key (params size) (b seed) of
    Err e -> False
    Ok dk ->
      hex-encode (mlkem-encapsulation-key-bytes (mlkem-encapsulation-key-of dk)) == ek
        && hex-encode (mlkem-decapsulation-key-bytes dk) == seed
        && (case mlkem-encapsulation-key (params size) (b ek) of
          Err e -> False
          Ok parsed -> case mlkem-encapsulate-with parsed (b m) of
            Err e -> False
            Ok (k, ct) -> hex-encode k == key && hex-encode ct == c)
        && opens dk c key && opens dk bad rejected
  -- | Decapsulation of a ciphertext the reference SDK produced.
  decaps :: Nat -> String -> String -> String -> Bool
  decaps size seed c key = case mlkem-decapsulation-key (params size) (b seed) of Ok dk -> opens dk c key; Err e -> False
  -- | A malformed ML-KEM-768 encapsulation key is rejected.
  invalid :: String -> Bool
  invalid ek = case mlkem-encapsulation-key MlKem768 (b ek) of Err MlKemInvalidEncapsulationKey -> True; other -> False
  -- | Size refusals and messages.
  errors :: Unit -> Bool
  errors unit =
    (case mlkem-decapsulation-key MlKem768 (bytes-new 63) of Err (MlKemInvalidSeedSize 63) -> True; other -> False)
      && (case mlkem-decapsulation-key MlKem1024 (bytes-new 64) of
        Err e -> False
        Ok dk -> (case mlkem-decapsulate dk (bytes-new 1088) of Err (MlKemInvalidCiphertextSize 1088) -> True; other -> False)
          && (case mlkem-encapsulate-with (mlkem-encapsulation-key-of dk) (bytes-new 31) of Err (MlKemInvalidRandomnessSize 31) -> True; other -> False)
          && (case mlkem-decapsulation-key-parameters dk of MlKem1024 -> True; MlKem768 -> False))
      && mlkem-ciphertext-size MlKem768 == 1088 && mlkem-encapsulation-key-size MlKem1024 == 1568 && mlkem-ciphertext-size MlKem1024 == 1568
      && mlkem-error-message MlKemInvalidEncapsulationKey == "mlkem: invalid encapsulation key"
  -- | Index of the first failing row, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
