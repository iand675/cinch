//! AES block encryption on hardware and constant-time portable paths, keystream modes,
//! CBC and GCM against independent vectors from `tooling/aes_vectors.py` (OpenSSL through
//! the Python cryptography package, plus FIPS-197 and SP 800-38D known answers).
mod common;

use cinch::driver::Mode;
use serde_json::Value;

/// Execute real portable source with bounded live memory and separate poisoned-free validation.
fn run(body: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("Crypto.Aes", include_str!("../prelude/Crypto/Aes.cinch")),
        ("Crypto.Cipher", include_str!("../prelude/Crypto/Cipher.cinch")),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
        ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), source).unwrap();
    }
    std::fs::write(
        directory.path().join("Main.cinch"),
        format!("{HELPERS}\n  -- | Observe the complete runtime fixture result.\n  main :: Int\n  main = if ({body}) then 0 else 1\n"),
    )
    .unwrap();
    let exe = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile { reclaim: true, rc_debug: true, ..cinch::core::TargetProfile::native() };
    common::drive_with_profile(Mode::Build, cinch::check::StageMode::Refresh, directory.path(), "Main.cinch", Some(exe.clone()), &profile).unwrap();
    for debug in [false, true] {
        let output = std::process::Command::new(&exe)
            .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_STACK_SIZE", "8M")
            .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(0), "debug={debug}: {}", String::from_utf8_lossy(&output.stderr));
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
}

/// Read fixed independent results without requiring any reference library during tests.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/aes-vectors.json")).unwrap()
}

/// FIPS-197 and independent single-block vectors match on both the hardware and portable paths.
#[test]
fn aes_blocks_match_fips_and_independent_vectors_on_both_paths() {
    let f = fixture();
    let checks = f["blocks"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| format!("block-case {:?} {:?} {:?}", row["key"].as_str().unwrap(), row["plain"].as_str().unwrap(), row["cipher"].as_str().unwrap()))
        .collect::<Vec<_>>();
    run(&format!("all [{}] && hardware-selected ()", checks.join(",\n")));
}

/// CTR, OFB, CFB and CBC match one-shot vectors, and chunked streams equal one-shot output at every split.
#[test]
fn keystream_and_cbc_modes_match_vectors_and_resume_across_splits() {
    let f = fixture();
    let checks = f["streams"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            let splits = format!("[{}]", row["splits"].as_array().unwrap().iter().map(Value::to_string).collect::<Vec<_>>().join(","));
            format!(
                "stream-case {:?} {:?} {} {:?} {:?} {:?} {:?} {splits}",
                row["key"].as_str().unwrap(),
                row["iv"].as_str().unwrap(),
                row["size"],
                row["ctr"].as_str().unwrap(),
                row["ofb"].as_str().unwrap(),
                row["cfb"].as_str().unwrap(),
                row.get("cbc").and_then(Value::as_str).unwrap_or(""),
            )
        })
        .collect::<Vec<_>>();
    let wrap = &f["wrap"];
    run(&format!(
        "all [{}] && wrap-case {:?} {:?} {:?}",
        checks.join(",\n"),
        wrap["key"].as_str().unwrap(),
        wrap["iv"].as_str().unwrap(),
        wrap["ctr"].as_str().unwrap()
    ));
}

/// GCM seals and opens every tag size, nonce size and AAD shape, and rejects every tampered input.
#[test]
fn gcm_matches_vectors_and_rejects_tampering_on_both_paths() {
    let f = fixture();
    let checks = f["gcm"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            format!(
                "gcm-case {:?} {:?} {} {:?} {} {:?}",
                row["key"].as_str().unwrap(),
                row["nonce"].as_str().unwrap(),
                row["size"],
                row["aad"].as_str().unwrap(),
                row["tag"],
                row["sealed"].as_str().unwrap()
            )
        })
        .collect::<Vec<_>>();
    for batch in checks.chunks(20) {
        run(&format!("all [{}]", batch.join(",\n")));
    }
}

/// Every misuse is reported with its precise error before any output is produced.
#[test]
fn cipher_errors_are_precise_and_rendered() {
    run("errors ()");
}

/// Every public and private declaration has canonical module-owned documentation.
#[test]
fn aes_and_cipher_declarations_have_canonical_documentation() {
    common::assert_documented("Crypto.Aes", include_str!("../prelude/Crypto/Aes.cinch"));
    common::assert_documented("Crypto.Cipher", include_str!("../prelude/Crypto/Cipher.cinch"));
}

/// The portable lanes, AES-NI, ARMv8 AES (with ArmAes evidence), PCLMULQDQ and detection bodies
/// all assemble for their targets; foreign execution is not claimed.
#[test]
fn aes_kernels_compile_for_x86_arm_wasm_and_big_endian_targets() {
    let directory = tempfile::tempdir().unwrap();
    let mut source = String::from("module Main where\n  import Crypto.Aes\n  import Crypto.Cipher\n  import Data.Wide\n  -- | Reach every kernel.\n  main :: Int\n  main = case aes-key (bytes-new 32) of\n    Ok key -> let c = aes-cipher key in case gcm-seal (gcm c) (bytes-new 12) (bytes-new 40) (bytes-new 3) of\n      Ok sealed -> if bytes-len sealed == 56 && eq-u128 (aes-decrypt-word key (aes-encrypt-word key 7)) 7 then 0 else 1\n      Err e -> 2\n    Err e -> 3\n");
    source.push('\n');
    for (triple, arch, endian, features) in [
        ("x86_64-unknown-linux-gnu", "x86_64", "little", vec![]),
        ("aarch64-unknown-linux-gnu", "aarch64", "little", vec!["ArmAes"]),
        ("aarch64-unknown-linux-gnu", "aarch64", "little", vec![]),
        ("wasm32-unknown-unknown", "wasm32", "little", vec![]),
        ("powerpc64-unknown-linux-gnu", "powerpc64", "big", vec![]),
    ] {
        let mut profile = cinch::driver::target_profile(&cinch::driver::DriverOptions { target: Some(triple.into()), ..Default::default() })
            .unwrap_or_else(|_| {
                let mut profile = cinch::core::TargetProfile::native();
                profile.triple = triple.into();
                profile.arch = arch.into();
                profile.endian = endian.into();
                profile
            });
        for feature in &features {
            profile.features.push((*feature).into());
        }
        let project = directory.path().join(format!("{arch}-{}", features.len()));
        std::fs::create_dir_all(&project).unwrap();
        for (name, text) in [
            ("Crypto.Aes", include_str!("../prelude/Crypto/Aes.cinch")),
            ("Crypto.Cipher", include_str!("../prelude/Crypto/Cipher.cinch")),
            ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), text).unwrap();
        }
        std::fs::write(project.join("Main.cinch"), &source).unwrap();
        let ir_path = project.join("program.ll");
        let entry = project.join("Main.cinch");
        let ir = std::thread::Builder::new()
            .stack_size(256 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::driver_with_profile_opts(Mode::EmitIr, cinch::check::StageMode::Refresh, &entry, Some(&ir_path), &profile, &cinch::driver::DriverOptions::default())
                    .map(|_| std::fs::read_to_string(&ir_path).unwrap())
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("<16 x i8>"));
        if arch == "x86_64" {
            assert!(ir.contains("aesenc") && ir.contains("pclmulqdq"));
        }
        if !features.is_empty() {
            assert!(ir.contains("aese") && ir.contains("aesmc"));
        }
        let output = std::process::Command::new(std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()))
            .args(["-target", triple, "-O2", "-c", "-x", "ir"])
            .arg(project.join("program.ll"))
            .arg("-o")
            .arg(project.join("program.o"))
            .output()
            .unwrap();
        assert!(output.status.success(), "{triple} {features:?}: {}", String::from_utf8_lossy(&output.stderr));
    }
}

/// Checked consumers that decode compact fixture rows.
const HELPERS: &str = r#"-- | Independent AES fixture consumers.
module Main where
  import Crypto.Aes
  import Crypto.Cipher
  import Text.Hex
  -- | Build deterministic input independently of cipher calculations.
  data-bytes :: Nat -> Bytes
  data-bytes n = if n <= 65536 then
      let size = u32-to-nat (runtime-size (wrapping-i64-to-u32 (nat-to-i64 n))) in
      let b = bytes-new size in let done = fill-bytes b 0 in b
    else bytes-new 0
  -- | Keep constant corpus setup in the executable instead of the checker's byte-buffer evaluator.
  prim runtime-size :: U32 -> U32 where
    llvm {
      %value = or i32 %0, 0
      ret i32 %value
    }
  -- | Fill a full-byte permutation.
  fill-bytes :: Bytes -> Nat -> Unit
  fill-bytes b i = if i < bytes-len b then let w = bytes-set b i (nat-rem (i * 37 + 11) 256) in fill-bytes b (i + 1) else ()
  -- | Parse independently recorded bytes.
  input :: String -> Bytes
  input text = case hex-decode text of Ok b -> b; Err e -> bytes-new 0
  -- | Compare bytes with recorded hex.
  same :: Bytes -> String -> Bool
  same a hex = hex-encode a == hex
  -- | Conjoin fixture rows.
  all :: List Bool -> Bool
  all values = case values of Nil -> True; Cons first rest -> first && all rest
  -- | Copy an interval.
  part :: Bytes -> Nat -> Nat -> Bytes
  part b start n = if start + n <= bytes-len b then let out = bytes-new n in let c = bytes-copy-range out b 0 start n in out else bytes-new 0
  -- | Join two buffers.
  concat :: Bytes -> Bytes -> Bytes
  concat a b = let out = bytes-new (bytes-len a + bytes-len b) in
    let x = bytes-copy-range out a 0 0 (bytes-len a) in let y = bytes-copy-range out b (bytes-len a) 0 (bytes-len b) in out
  -- | Overwrite one byte when present.
  poke :: Bytes -> Nat -> Nat -> Unit
  poke b i v = if i < bytes-len b && v < 256 then bytes-set b i v else ()
  -- | A copy with one byte's low bit flipped.
  flipped :: Bytes -> Nat -> Bytes
  flipped b i = let out = part b 0 (bytes-len b) in let x = poke out i (if i < bytes-len b then nat-rem (bytes-get b i + 1) 256 else 0) in out
  -- | Expand a key with either constructor, or a zero key on error.
  key-of :: Bool -> String -> AesKey
  key-of hardware text = case (if hardware then aes-key (input text) else aes-portable-key (input text)) of
    Ok key -> key
    Err e -> key-of False "00000000000000000000000000000000"
  -- | Successful output equals the recorded hex.
  ok :: Result CipherError Bytes -> String -> Bool
  ok result hex = case result of Ok out -> same out hex; Err e -> False
  -- | This host has AES-NI and PCLMULQDQ, so the default constructors must select them.
  hardware-selected :: Unit -> Bool
  hardware-selected unit = aes-key-hardware (key-of True "000102030405060708090a0b0c0d0e0f")
    && not (aes-key-hardware (key-of False "000102030405060708090a0b0c0d0e0f"))
    && gcm-hardware (gcm (aes-cipher (key-of True "000102030405060708090a0b0c0d0e0f")))
  -- | Encrypt and decrypt one block on both paths, as bytes and as words.
  block-case :: String -> String -> String -> Bool
  block-case k p c = block-path True k p c && block-path False k p c
  -- | One path's block checks.
  block-path :: Bool -> String -> String -> String -> Bool
  block-path hardware k p c = let key = key-of hardware k in
    (case aes-encrypt-block key (input p) of Ok out -> same out c; Err e -> False)
      && (case aes-decrypt-block key (input c) of Ok out -> same out p; Err e -> False)
      && aes-key-size key * 2 == bytes-len (input k) * 2
  -- | Apply a fresh stream to a message in two chunks split at a point.
  split-stream :: Result CipherError XorStream -> Bytes -> Nat -> Bytes
  split-stream start message point = if point <= bytes-len message then split-at start (part message 0 point) (part message point (bytes-len message - point)) else bytes-new 0
  -- | Apply a fresh stream to two chunks in order and join the outputs.
  split-at :: Result CipherError XorStream -> Bytes -> Bytes -> Bytes
  split-at start first-chunk second-chunk = case start of
    Ok stream -> case stream-xor stream first-chunk of
      (first, next) -> case stream-xor next second-chunk of (second, last) -> concat first second
    Err e -> bytes-new 0
  -- | Every split of every keystream mode equals the one-shot vector.
  splits :: BlockCipher -> Bytes -> Bytes -> String -> String -> String -> List Nat -> Bool
  splits c iv m ctr ofb cfb points = case points of
    Nil -> True
    Cons point rest -> same (split-stream (ctr-stream c iv) m point) ctr && same (split-stream (ofb-stream c iv) m point) ofb
      && same (split-stream (cfb-encrypt-stream c iv) m point) cfb && same (split-stream (cfb-decrypt-stream c iv) (input cfb) point) (hex-encode m)
      && splits c iv m ctr ofb cfb rest
  -- | One-shot, inverse, split and CBC checks on both paths.
  stream-case :: String -> String -> Nat -> String -> String -> String -> String -> List Nat -> Bool
  stream-case k iv n ctr ofb cfb cbc points = stream-path True k iv n ctr ofb cfb cbc points && stream-path False k iv n ctr ofb cfb cbc points
  -- | One path's stream and CBC checks.
  stream-path :: Bool -> String -> String -> Nat -> String -> String -> String -> String -> List Nat -> Bool
  stream-path hardware k iv n ctr ofb cfb cbc points =
    let c = aes-cipher (key-of hardware k) in let m = data-bytes n in let v = input iv in
    ok (ctr-xor c v m) ctr && ok (ctr-xor c v (input ctr)) (hex-encode m) && ok (ofb-xor c v m) ofb && ok (ofb-xor c v (input ofb)) (hex-encode m)
      && ok (cfb-encrypt c v m) cfb && ok (cfb-decrypt c v (input cfb)) (hex-encode m)
      && splits c v m ctr ofb cfb points
      && (if nat-rem n 16 == 0 then ok (cbc-encrypt c v m) cbc && ok (cbc-decrypt c v (input cbc)) (hex-encode m) && cbc-chunks c v m cbc else True)
  -- | CBC split into two whole-block calls chains exactly like one call.
  cbc-chunks :: BlockCipher -> Bytes -> Bytes -> String -> Bool
  cbc-chunks c iv m expected =
    let point = 16 * nat-div (bytes-len m) 32 in
    if point <= bytes-len m then same (cbc-two c iv (part m 0 point) (part m point (bytes-len m - point))) expected else False
  -- | Encrypt two whole-block chunks with one chained state and join them.
  cbc-two :: BlockCipher -> Bytes -> Bytes -> Bytes -> Bytes
  cbc-two c iv first-chunk second-chunk = case cbc-encrypter c iv of
    Err e -> bytes-new 0
    Ok state -> case cbc-crypt-blocks state first-chunk of
      Err e -> bytes-new 0
      Ok (first, next) -> case cbc-crypt-blocks next second-chunk of
        Err e -> bytes-new 0
        Ok (second, last) -> concat first second
  -- | The counter wraps from all-ones to zero.
  wrap-case :: String -> String -> String -> Bool
  wrap-case k iv expected = ok (ctr-xor (aes-cipher (key-of True k)) (input iv) (data-bytes 48)) expected
    && ok (ctr-xor (aes-cipher (key-of False k)) (input iv) (data-bytes 48)) expected
  -- | Seal, open, and reject tampered ciphertext, tag, AAD and nonce on both paths.
  gcm-case :: String -> String -> Nat -> String -> Nat -> String -> Bool
  gcm-case k nonce n aad tag sealed = gcm-path True k nonce n aad tag sealed && gcm-path False k nonce n aad tag sealed
  -- | One path's GCM checks.
  gcm-path :: Bool -> String -> String -> Nat -> String -> Nat -> String -> Bool
  gcm-path hardware k nonce n aad tag sealed =
    let c = aes-cipher (key-of hardware k) in
    let configured = case gcm-with-nonce-size c (bytes-len (input nonce)) of
          Ok g -> (if tag == 16 then Ok g else gcm-tagged c (bytes-len (input nonce)) tag)
          Err e -> Err e in
    case configured of
      Err e -> False
      Ok g -> let s = input sealed in let a = input aad in let v = input nonce in
        gcm-tag-size g == tag && gcm-overhead g == tag && gcm-nonce-size g == bytes-len v
        && ok (gcm-seal g v (data-bytes n) a) sealed && ok (gcm-open g v s a) (hex-encode (data-bytes n))
        && rejected (gcm-open g v (flipped s 0) a) && rejected (gcm-open g v (flipped s (pred (bytes-len s))) a)
        && rejected (gcm-open g v s (concat a (bytes-from-u8s [1]))) && rejected (gcm-open g (flipped v 0) s a)
        && rejected (gcm-open g v (part s 0 (pred tag)) a)
  -- | The predecessor of a natural number, with zero for zero.
  pred :: Nat -> Nat
  pred n = if n > 0 then n - 1 else 0
  -- | A GCM instance with both a custom nonce size and tag size.
  gcm-tagged :: BlockCipher -> Nat -> Nat -> Result CipherError Gcm
  gcm-tagged c nonce tag = case gcm-with-tag-size c tag of
    Ok g -> if nonce == 12 then Ok g else Err (InvalidNonceSize 12 nonce)
    Err e -> Err e
  -- | Authentication failed without output.
  rejected :: Result CipherError Bytes -> Bool
  rejected result = case result of Err AuthenticationFailed -> True; other -> False
  -- | Every error constructor appears for its misuse, and messages render.
  errors :: Unit -> Bool
  errors unit =
    let c = aes-cipher (key-of True "000102030405060708090a0b0c0d0e0f") in
    (case aes-key (bytes-new 15) of Err (InvalidAesKeySize 15) -> True; other -> False)
      && (case aes-portable-key (bytes-new 33) of Err (InvalidAesKeySize 33) -> True; other -> False)
      && (case aes-encrypt-block (key-of True "000102030405060708090a0b0c0d0e0f") (bytes-new 17) of Err (InvalidAesBlockSize 17) -> True; other -> False)
      && (case ctr-xor c (bytes-new 15) (bytes-new 3) of Err (InvalidIvSize 16 15) -> True; other -> False)
      && (case cbc-encrypt c (bytes-new 16) (bytes-new 17) of Err (InvalidInputSize 17) -> True; other -> False)
      && (case gcm-with-tag-size c 11 of Err (InvalidTagSize 11) -> True; other -> False)
      && (case gcm-with-nonce-size c 0 of Err (InvalidNonceSize 12 0) -> True; other -> False)
      && (case gcm-seal (gcm c) (bytes-new 11) (bytes-new 1) (bytes-new 0) of Err (InvalidNonceSize 12 11) -> True; other -> False)
      && aes-key-error-message (InvalidAesKeySize 15) == "AES key must be 16, 24 or 32 bytes; received 15"
      && cipher-error-message AuthenticationFailed == "message authentication failed"
      && cipher-error-message (InvalidIvSize 16 15) == "initialization vector must be 16 bytes; received 15"
      && aes-block-size == 16 && cipher-block-size c == 16
"#;
