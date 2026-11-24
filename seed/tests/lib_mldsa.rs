//! ML-DSA-44, ML-DSA-65, and ML-DSA-87 (FIPS 204) verified against an independent
//! definition-level Python FIPS 204 reference implementation: key derivation from
//! 32-byte seeds, private and public key serialization roundtrips, deterministic
//! and hedged signing with pure ML-DSA context framing, verification, tamper
//! rejection, malformed key and hint rejection, canonical documentation, and
//! multi-target LLVM lowering.
mod common;

/// Compile and run one batch of fixture rows; the exit status is the first failing row, 200 if none.
fn run_batch(rows: &[String]) {
    let dir = common::project("mldsa", &[
        ("Crypto.MlDsa.cinch", include_str!("../prelude/Crypto/MlDsa.cinch")),
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

/// Seeds derive the FIPS 204 reference keys; deterministic and hedged signatures verify and reject tampering.
#[test]
fn mldsa_matches_fips204_reference_vectors() {
    let rows = vec![
        "vec 44 \"000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f\" \"\" \"\" \"0000000000000000000000000000000000000000000000000000000000000000\" \"373c7bf2cac5bd2a6c35933bab0fa1c951f22247e1333383fcb618822080373f\" \"da0665a6d807669d06aeb7010507aa6bd4640641055462e38e37344a274f36a6\" \"b133eb5687c518c66a52bd5dc8b0889ef903c6b5b367e3de2cc01e08ba7960b1\"".to_string(),
        "vec 44 \"424950575e656c737a81888f969da4abb2b9c0c7ced5dce3eaf1f8ff060d141b\" \"4649505320323034204d4c2d4453412d34342064657465726d696e6973746963\" \"63696e63682d637478\" \"0000000000000000000000000000000000000000000000000000000000000000\" \"f8ec60f419852707851f1cb632391bc5c21b19f962399645b0492a434d4f24e8\" \"2031e436c72bbfa5a7d52e08d533e6e15bc5ca81cb96f2bb38850bc0f0966c58\" \"c2aec94f2c08c0d860a1d328388711aeb3baf9f659e37260363563f3b5b5018d\"".to_string(),
        "vec 44 \"7f868d949ba2a9b0b7bec5ccd3dae1e8eff6fd040b121920272e353c434a5158\" \"4649505320323034204d4c2d4453412d343420686564676564\" \"ababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababab\" \"a5b2bfccd9e6f3000d1a2734414e5b6875828f9ca9b6c3d0ddeaf704111e2b38\" \"8641bbeb58ce4c113a00a2597c414330c4f1a9f2705d866a5daf21617001f99e\" \"00b25d4bc3dd2220837fd4b1051fc39e021eabe02acd58f0f5704222c9007561\" \"6f63ecab5a0306fc61a4bf6ba776cb1446a125481aeb140a1e55cdc14b2e5901\"".to_string(),
        "vec 65 \"11181f262d343b424950575e656c737a81888f969da4abb2b9c0c7ced5dce3ea\" \"4649505320323034204d4c2d4453412d36352064657465726d696e6973746963\" \"\" \"0000000000000000000000000000000000000000000000000000000000000000\" \"12632ec69c7522ef8ec8bfe7fabb9f8e93ec2bb50ff297a510ec00d95f3244fe\" \"115207e63e6426db3f5148e75c86a245c05e6dcf065239f1cf58d534ace23071\" \"20be5d79c20d8be05bc654c7d7a34c35c7fae173b67665ad15c32f4a785aa66d\"".to_string(),
        "vec 65 \"222930373e454c535a61686f767d848b9299a0a7aeb5bcc3cad1d8dfe6edf4fb\" \"4649505320323034204d4c2d4453412d363520686564676564\" \"6374782d3635\" \"5a6774818e9ba8b5c2cfdce9f603101d2a3744515e6b7885929facb9c6d3e0ed\" \"c26c8780a06679f44a24ed6cf5c76bf8b31dcf4339e495cc6abdd52352a99b71\" \"5fd4ec1c41ed061207807e63470cf299b6010e8d7009e9cdc2fec706a409c5e9\" \"597cdea91ca8d7fbb9667699e1114c8b53d3c8ca303434b0f6788d123a97aef0\"".to_string(),
        "vec 87 \"333a41484f565d646b727980878e959ca3aab1b8bfc6cdd4dbe2e9f0f7fe050c\" \"4649505320323034204d4c2d4453412d38372064657465726d696e6973746963\" \"6374782d3837\" \"0000000000000000000000000000000000000000000000000000000000000000\" \"040d2fe263fe6eba68542b5051896d79aa566102837af1190ffa9bace70348d7\" \"d49148fdb4340b74b97b50b28afbbd7e255739e268548518582e31e868a1c6cf\" \"6bc10c12961f5c7f54dfd6b19a7018b4ecde9077befcd18fad2511d2e9a08858\"".to_string(),
        "vec 87 \"99a0a7aeb5bcc3cad1d8dfe6edf4fb020910171e252c333a41484f565d646b72\" \"4649505320323034204d4c2d4453412d383720686564676564\" \"\" \"3c495663707d8a97a4b1becbd8e5f2ff0c192633404d5a6774818e9ba8b5c2cf\" \"5b98d42046a18c5de54e889f1b948351a732555ecbded120670e9196e399ebe0\" \"966ea0793ecb50cd75f47b7e754a416d8e2b8acffb85c5a6ff691c5786782bab\" \"7e5363f80eb866f3cf65ed5270ec80efe44e71eaafb1a216d01b23bf7bc710bd\"".to_string(),
        "errors ()".to_string(),
    ];
    run_batch(&rows);
}

/// The ML-DSA kernels lower cleanly for 64-bit ARM, WebAssembly, and big-endian PowerPC64.
#[test]
fn mldsa_kernels_compile_for_arm_wasm_and_big_endian_targets() {
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
            ("Crypto.MlDsa", include_str!("../prelude/Crypto/MlDsa.cinch")),
            ("Crypto.ConstantTime", include_str!("../prelude/Crypto/ConstantTime.cinch")),
            ("Text.Sha3", include_str!("../prelude/Text/Sha3.cinch")),
            ("Text.Binary", include_str!("../prelude/Text/Binary.cinch")),
            ("Data.Bytes.Build", include_str!("../prelude/Data/Bytes/Build.cinch")),
            ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), text).unwrap();
        }
        std::fs::write(
            project.join("Main.cinch"),
            "module Main where\n  import Crypto.MlDsa\n  -- | Reach keygen, sign, and verify kernels.\n  main :: Int\n  main = case mldsa-key-from-seed MlDsa44 (bytes-new 32) of\n    Err e -> 1\n    Ok sk -> case mldsa-sign-deterministic sk (bytes-new 4) (bytes-new 0) of\n      Err e -> 2\n      Ok sig -> if mldsa-verify (mldsa-public-key-of sk) (bytes-new 4) (bytes-new 0) sig then 0 else 3\n",
        )
        .unwrap();
        let ir_path = project.join("program.ll");
        let entry = project.join("Main.cinch");
        let ir = std::thread::Builder::new()
            .stack_size(256 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::driver_with_profile_opts(
                    cinch::driver::Mode::EmitIr,
                    cinch::check::StageMode::Refresh,
                    &entry,
                    Some(&ir_path),
                    &profile,
                    &cinch::driver::DriverOptions::default(),
                )
                .map(|_| std::fs::read_to_string(&ir_path).unwrap())
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("Crypto.MlDsa.ntt-butterfly"));
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
fn mldsa_declarations_have_canonical_documentation() {
    common::assert_documented("Crypto.MlDsa", include_str!("../prelude/Crypto/MlDsa.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference ML-DSA vector consumers.
module Main where
  import Crypto.MlDsa
  import Data.Bytes
  import Data.List
  import Data.Nat
  import Data.Result
  import Data.String
  import Text.Hex
  import Text.Sha3

  -- | Decode bytes from hex.
  b :: String -> Bytes
  b text = case hex-decode text of Ok v -> v; Err e -> bytes-new 0

  -- | Map a security level to its parameter set.
  params :: Nat -> MlDsaParameters
  params lvl = if lvl == 44 then MlDsa44 else if lvl == 65 then MlDsa65 else MlDsa87

  -- | Flip the high bit of byte idx in a fresh copy of src.
  flip-byte :: Bytes -> Nat -> Bytes
  flip-byte src idx =
    let n = bytes-len src in
    let out = bytes-new n in
    let copied = bytes-copy-range out src 0 0 n in
    if idx < n then
      let cur = bytes-get out idx in
      let next = if cur < 128 then cur + 128 else cur - 128 in
      if next < 256 then let w = bytes-set out idx next in out else out
    else out

  -- | Set one byte in a fresh copy of src.
  set-byte :: Bytes -> Nat -> Nat -> Bytes
  set-byte src idx val =
    let n = bytes-len src in
    let out = bytes-new n in
    let copied = bytes-copy-range out src 0 0 n in
    if idx < n && val < 256 then let w = bytes-set out idx val in out else out

  -- | Check key derivation, sk/pk roundtrip, deterministic or hedged signing, verification, and tamper rejection.
  vec :: Nat -> String -> String -> String -> String -> String -> String -> String -> Bool
  vec lvl xi-hex msg-hex ctx-hex rnd-hex pk-hash sk-hash sig-hash =
    let p = params lvl in
    let xi = b xi-hex in
    let msg = b msg-hex in
    let ctx = b ctx-hex in
    let rnd = b rnd-hex in
    case mldsa-key-from-seed p xi of
      Err e -> False
      Ok sk ->
        let pk = mldsa-public-key-of sk in
        let pkb = mldsa-public-key-bytes pk in
        let skb = mldsa-private-key-bytes sk in
        let sizes-ok = bytes-len pkb == mldsa-public-key-size p && bytes-len skb == mldsa-private-key-size p in
        let hashes-ok = hex-encode (sha3-sum256 pkb) == pk-hash && hex-encode (sha3-sum256 skb) == sk-hash in
        let reload-ok = (
          case mldsa-private-key p skb of
            Err e -> False
            Ok sk2 -> hex-encode (sha3-sum256 (mldsa-public-key-bytes (mldsa-public-key-of sk2))) == pk-hash
        ) in
        let sign-ok = (
          case mldsa-public-key p pkb of
            Err e -> False
            Ok pk2 ->
              let sig-res = if rnd == bytes-new 32 then mldsa-sign-deterministic sk msg ctx else mldsa-sign-with sk msg ctx rnd in
              case sig-res of
                Err e -> False
                Ok sig ->
                  bytes-len sig == mldsa-signature-size p
                    && hex-encode (sha3-sum256 sig) == sig-hash
                    && mldsa-verify pk2 msg ctx sig
                    && not (mldsa-verify pk2 (if bytes-len msg == 0 then bytes-new 1 else flip-byte msg 0) ctx sig)
                    && not (mldsa-verify pk2 msg (if bytes-len ctx == 0 then bytes-new 1 else flip-byte ctx 0) sig)
                    && not (mldsa-verify pk2 msg ctx (flip-byte sig 0))
                    && not (mldsa-verify pk2 msg ctx (flip-byte sig 80))
                    && not (mldsa-verify pk2 msg ctx (flip-byte sig (saturating-sub (bytes-len sig) 1)))
        ) in
        sizes-ok && hashes-ok && reload-ok && sign-ok

  -- | Size constants, error constructors, malformed key/hint rejection, and messages.
  errors :: Unit -> Bool
  errors unit =
    mldsa-seed-size == 32
      && mldsa-public-key-size MlDsa44 == 1312 && mldsa-private-key-size MlDsa44 == 2560 && mldsa-signature-size MlDsa44 == 2420
      && mldsa-public-key-size MlDsa65 == 1952 && mldsa-private-key-size MlDsa65 == 4032 && mldsa-signature-size MlDsa65 == 3309
      && mldsa-public-key-size MlDsa87 == 2592 && mldsa-private-key-size MlDsa87 == 4896 && mldsa-signature-size MlDsa87 == 4627
      && (case mldsa-key-from-seed MlDsa44 (bytes-new 31) of Err (MlDsaInvalidSeedSize 31) -> True; other -> False)
      && (case mldsa-public-key MlDsa44 (bytes-new 1311) of Err MlDsaInvalidPublicKey -> True; other -> False)
      && (case mldsa-private-key MlDsa44 (bytes-new 2559) of Err MlDsaInvalidPrivateKey -> True; other -> False)
      && (case mldsa-key-from-seed MlDsa44 (bytes-new 32) of
        Err e -> False
        Ok sk ->
          let pk = mldsa-public-key-of sk in
          let skb = mldsa-private-key-bytes sk in
          (case mldsa-sign-with sk (bytes-new 1) (bytes-new 0) (bytes-new 31) of Err (MlDsaInvalidRandomnessSize 31) -> True; other -> False)
            && (case mldsa-sign-deterministic sk (bytes-new 1) (bytes-new 256) of Err (MlDsaContextTooLong 256) -> True; other -> False)
            && not (mldsa-verify pk (bytes-new 1) (bytes-new 256) (bytes-new 2420))
            && not (mldsa-verify pk (bytes-new 1) (bytes-new 0) (bytes-new 2419))
            && (case mldsa-private-key MlDsa44 (flip-byte skb 70) of Err MlDsaInvalidPrivateKey -> True; other -> False)
            && (case mldsa-private-key MlDsa44 (set-byte skb 128 255) of Err MlDsaInvalidPrivateKey -> True; other -> False)
            && (case mldsa-private-key MlDsa44 (flip-byte skb 2500) of Err MlDsaInvalidPrivateKey -> True; other -> False)
            && (case mldsa-sign-deterministic sk (bytes-new 4) (bytes-new 0) of
              Err e -> False
              Ok sig ->
                not (mldsa-verify pk (bytes-new 4) (bytes-new 0) (set-byte sig 2415 1)))
            && (case mldsa-private-key-parameters sk of MlDsa44 -> True; other -> False)
            && (case mldsa-public-key-parameters pk of MlDsa44 -> True; other -> False))
      && mldsa-error-message MlDsaInvalidPublicKey == "mldsa: invalid public key"
      && mldsa-error-message MlDsaInvalidPrivateKey == "mldsa: invalid private key"
      && mldsa-error-message MlDsaInvalidSignature == "mldsa: invalid signature"
      && mldsa-error-message MlDsaSigningFailed == "mldsa: signing failed"

  -- | Index of the first failing row, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
