//! Bzip2 decompression against libbzip2 encodings from `tooling/bzip2_vectors.py`: six corpus
//! shapes (including long runs and a geometric distribution that forces long Huffman codes),
//! block sizes 1, 5 and 9, multi-block and multi-stream inputs, and the empty stream. Decoded
//! output is compared by SHA-256 digest and length. Malformed inputs report precise errors.
mod common;

use serde_json::Value;

/// Compile and run one batch of fixture rows; the exit status is the first failing row, 200 if none.
fn run_batch(rows: &[String]) {
    let dir = common::project("bzip2", &[
        ("Text.Bzip2.cinch", include_str!("../prelude/Text/Bzip2.cinch")),
        ("Text.Sha256.cinch", include_str!("../prelude/Text/Sha256.cinch")),
        ("Text.Binary.cinch", include_str!("../prelude/Text/Binary.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Data.Bytes.Word.cinch", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ("Text.Hex.cinch", include_str!("../prelude/Text/Hex.cinch")),
        ("Main.cinch", &format!("{HELPERS}\n  -- | Every row in this batch.\n  main :: Int\n  main = first [{}] 0\n", rows.join(",\n    "))),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 200, "the exit status is the first failing row index in the batch; 200 means every row passed");
    common::cleanup(&dir);
}

/// Every libbzip2 encoding decodes to its corpus.
#[test]
fn bzip2_decodes_reference_streams() {
    let fixture: Value = serde_json::from_str(include_str!("fixtures/bzip2-vectors.json")).unwrap();
    let checks = fixture["rows"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| format!("decodes {} {} {:?} {:?}", row["kind"], row["size"], row["packed"].as_str().unwrap(), row["digest"].as_str().unwrap()))
        .collect::<Vec<_>>();
    for batch in checks.chunks(25) {
        run_batch(batch);
    }
}

/// Malformed headers, truncations, corrupted CRCs, randomized blocks and trailing bytes are rejected.
#[test]
fn bzip2_rejects_malformed_streams() {
    let fixture: Value = serde_json::from_str(include_str!("fixtures/bzip2-vectors.json")).unwrap();
    let mut checks = fixture["errors"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| format!("rejects {:?} {:?}", row["packed"].as_str().unwrap(), row["error"].as_str().unwrap()))
        .collect::<Vec<_>>();
    checks.push("messages ()".to_string());
    run_batch(&checks);
}

/// The bit-window and CRC kernels lower for 64-bit ARM, WebAssembly and a big-endian target.
#[test]
fn bzip2_kernels_compile_for_arm_wasm_and_big_endian_targets() {
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
            ("Text.Bzip2", include_str!("../prelude/Text/Bzip2.cinch")),
            ("Data.Bytes.Build", include_str!("../prelude/Data/Bytes/Build.cinch")),
            ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), text).unwrap();
        }
        std::fs::write(project.join("Main.cinch"), "module Main where\n  import Text.Bzip2\n  import Data.Bytes.Build\n  -- | Reach the kernels.\n  main :: Int\n  main = case bzip2-decompress (bytes-from-u8s [66, 90, 104, 57, 23, 114, 69, 56, 80, 144, 0, 0, 0, 0]) of\n    Ok out -> if bytes-len out == 0 then 0 else 1\n    Err e -> 2\n").unwrap();
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
        assert!(ir.contains("@llvm.bswap.i64") && ir.contains("@llvm.bswap.i32"));
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
fn bzip2_declarations_have_canonical_documentation() {
    common::assert_documented("Text.Bzip2", include_str!("../prelude/Text/Bzip2.cinch"));
}

/// Checked consumer helpers.
const HELPERS: &str = r#"-- | Reference bzip2 vector consumers.
module Main where
  import Text.Bzip2
  import Data.Bytes.Build
  import Text.Hex
  import Text.Sha256
  -- | Parse independently recorded bytes.
  input :: String -> Bytes
  input text = case hex-decode text of Ok b -> b; Err e -> bytes-new 0
  -- | The expected output: one corpus, or for kind 6 the two-stream concatenation.
  expected :: Nat -> Nat -> Bytes
  expected kind size = if kind == 6 then bytes-concat [corpus 2 700, corpus 4 900] else corpus kind size
  -- | Rebuild a corpus with the generator's six shapes.
  corpus :: Nat -> Nat -> Bytes
  corpus kind size = let out = bytes-new size in let filled = fill out kind 0 12345 in out
  -- | Fill corpus bytes in order, advancing a 32-bit linear congruential state before each byte.
  fill :: Bytes -> Nat -> Nat -> U32 -> Unit
  fill out kind i state =
    if i < bytes-len out then
      let next = wrapping-add-u32 (wrapping-mul-u32 state 1103515245) 12345 in
      let value = value-of kind i next in
      let written = if value < 256 then bytes-set out i value else () in
      fill out kind (i + 1) next
    else ()
  -- | One corpus byte for the generator kind.
  value-of :: Nat -> Nat -> U32 -> Nat
  value-of kind i s =
    if kind == 0 then nat-rem (i * 37 + 11) 256
    else if kind == 1 then 97 + nat-rem i 3
    else if kind == 2 then u32-to-nat (bit-and-u32 (wrapping-shr-u32 s 16) 255)
    else if kind == 3 then nat-rem (nat-div i 300) 7 * 40
    else if kind == 4 then 65 + u32-to-nat (bit-and-u32 (wrapping-shr-u32 s 16) 3)
    else u32-to-nat (trailing-zeros-u32 s)
  -- | One row: the stream decodes to a corpus of the recorded digest and length.
  decodes :: Nat -> Nat -> String -> String -> Bool
  decodes kind size packed digest = case bzip2-decompress (input packed) of
    Ok out -> bytes-len out == bytes-len (expected kind size) && hex-encode (sha256-sum out) == digest && hex-encode (sha256-sum (expected kind size)) == digest
    Err e -> False
  -- | One malformed input yields its expected error.
  rejects :: String -> String -> Bool
  rejects packed kind = case bzip2-decompress (input packed) of
    Ok out -> False
    Err e -> case e of
      Bzip2BadHeader -> kind == "header"
      Bzip2Truncated -> kind == "truncated"
      Bzip2ChecksumMismatch -> kind == "checksum"
      Bzip2BadBlock 112 -> kind == "block112"
      Bzip2BadBlock 32 -> kind == "block32"
      Bzip2BadBlock other -> False
  -- | Error messages.
  messages :: Unit -> Bool
  messages unit =
    bzip2-error-message (Bzip2BadBlock 7) == "bzip2: invalid block at bit 7"
      && bzip2-error-message Bzip2Truncated == "bzip2: truncated stream"
      && bzip2-error-message Bzip2BadHeader == "bzip2: invalid stream header"
      && bzip2-error-message Bzip2ChecksumMismatch == "bzip2: checksum mismatch"
  -- | Index of the first failing row, or 200 when all pass.
  first :: List Bool -> Int -> Int
  first values i = case values of Nil -> 200; Cons x rest -> if x then first rest (i + 1) else i
"#;
