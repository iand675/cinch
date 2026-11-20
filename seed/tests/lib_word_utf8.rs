//! Bounds, lifetime and UTF-8 boundary controls for native word/SWAR kernels.
mod common;

use cinch::driver::Mode;

/// Compare bytes against the platform's independently validated UTF-8 parser.
/// Every possible high byte and every alignment around the eight-byte fast path
/// is checked; multibyte sequences cross block boundaries and truncated tails.
#[test]
fn swar_validation_matches_utf8_for_all_byte_values_and_block_boundaries() {
    let mut vectors = Vec::new();
    for prefix in 0..=17 {
        for byte in 0..=255 {
            let mut bytes = vec![b'a'; prefix];
            bytes.push(byte);
            vectors.push(bytes);
        }
        for suffix in ["é", "€", "🦀"] {
            let mut bytes = vec![b'a'; prefix];
            bytes.extend(suffix.as_bytes());
            bytes.extend_from_slice(b"tail");
            for removed in 0..suffix.len() {
                vectors.push(bytes[..prefix + suffix.len() - removed].to_vec());
            }
            vectors.push(bytes);
        }
    }
    // Generate data at runtime rather than thousands of syntax-level test bodies.
    let corpus = vectors
        .iter()
        .map(|b| {
            let payload = b.iter().map(|v| format!("{v:02x}")).collect::<String>();
            format!("{} {}", u8::from(std::str::from_utf8(b).is_ok()), payload)
        })
        .collect::<Vec<_>>()
        .join("\n");
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(
        directory.path().join("Text.Hex.cinch"),
        include_str!("../prelude/Text/Hex.cinch"),
    )
    .unwrap();
    std::fs::write(
        directory.path().join("Data.Bytes.Word.cinch"),
        include_str!("../prelude/Data/Bytes/Word.cinch"),
    )
    .unwrap();
    // A line-oriented corpus is a runtime file, not compiler constant folding.
    let corpus_path = directory.path().join("corpus.txt");
    std::fs::write(&corpus_path, corpus).unwrap();
    let quoted = serde_json::to_string(&corpus_path.to_string_lossy()).unwrap();
    std::fs::write(
        directory.path().join("Text.Scan.cinch"),
        include_str!("../prelude/Text/Scan.cinch"),
    )
    .unwrap();
    let source = format!(
        r#"module Main where
  import Text.Hex
  import Text.Scan
  import Effect.Fs
  import Data.Path
  check :: List String -> Bool
  check lines = case lines of
    Nil -> True
    Cons line rest -> case hex-decode (string-drop 2 line) of
      Err _ -> False
      Ok b -> case string-from-bytes b of
        None -> string-starts-with "0 " line && check rest
        Some _ -> string-starts-with "1 " line && check rest
  main :: Unit ->{{IO r}} Int
  main unit = with (io = newIO ()) in
    case fs-read (path-from-string {quoted}) of
      Err _ -> 2
      Ok corpus -> case string-from-bytes corpus of
        None -> 3
        Some text -> if check (string-lines text) then 0 else 1
"#
    );
    std::fs::write(directory.path().join("Main.cinch"), source).unwrap();
    assert_eq!(
        common::drive(Mode::Run, directory.path(), "Main.cinch", None).unwrap(),
        0
    );
}

/// Ephemeral backing buffers remain alive across private raw loads under
/// poisoning and leak checking, including unaligned reads of the final word.
#[test]
fn bounded_word_loads_retain_ephemeral_buffers_and_release_them_afterwards() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(
        directory.path().join("Data.Bytes.Word.cinch"),
        include_str!("../prelude/Data/Bytes/Word.cinch"),
    )
    .unwrap();
    std::fs::write(directory.path().join("Main.cinch"), r#"module Main where
  import Data.Bytes.Word
  buffer :: Nat -> refined (bytes-len out == 9) Bytes
  buffer n =
    let b = bytes-new 9 in
    let first = bytes-set b 1 (nat-rem n 256) in
    let last = bytes-set b 8 127 in
    b
  loop :: Nat -> Bool
  loop n =
    if n == 0 then True
    else if eq-u64 (bytes-read-le64 (buffer n) 1) (bit-or-u64 0x7F00000000000000 (wrapping-i64-to-u64 (nat-to-i64 (nat-rem n 256))))
      && eq-u32 (bytes-read-le32 (buffer n) 1) (wrapping-i64-to-u32 (nat-to-i64 (nat-rem n 256)))
      then loop (n - 1) else False
  main :: Int
  main = if loop 1000 then 0 else 1
"#).unwrap();
    let executable = directory.path().join("words");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    common::drive_with_profile(
        Mode::Build,
        cinch::check::StageMode::Refresh,
        directory.path(),
        "Main.cinch",
        Some(executable.clone()),
        &profile,
    )
    .unwrap();
    let result = std::process::Command::new(executable)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "32M")
        .output()
        .unwrap();
    assert_eq!(
        result.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(String::from_utf8_lossy(&result.stderr).contains("leaked=0"));
}
