//! Prefix, append and caller-buffer hexadecimal codecs against independent fixtures.
mod common;

use cinch::driver::Mode;
use serde_json::Value;

/// Build runtime buffers from independent fixture octets, without calling the codec under test.
fn bytes(hex: &str) -> String {
    let values = hex
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16)
                .unwrap()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("(bytes-from-u8s [{values}])")
}

/// Compile source-distributed modules and validate every consumer allocation and free.
fn run(checks: Vec<String>) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), source).unwrap();
    }
    let mut source = HELPERS.to_string();
    source.push_str("\n  -- | Evaluate independent runtime fixtures and identify the first failure.\n  checks :: List Bool\n  checks = [\n");
    source.push_str(
        &checks
            .iter()
            .map(|check| format!("    ({check})"))
            .collect::<Vec<_>>()
            .join(",\n"),
    );
    source.push_str("\n    ]\n  -- | Exit successfully only when every consumer check agrees.\n  main :: Int\n  main = inspect checks 0\n");
    std::fs::write(directory.path().join("Main.cinch"), source).unwrap();
    let exe = directory.path().join("consumer");
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
        Some(exe.clone()),
        &profile,
    )
    .unwrap();
    let result = std::process::Command::new(exe)
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

/// Completed prefixes and exact invalid-byte offsets agree with independent decoder results.
#[test]
fn hexadecimal_prefix_and_append_match_reference_progress() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/go-stdlib-vectors.json")).unwrap();
    let mut checks = Vec::new();
    for row in fixture["hex_decodes"].as_array().unwrap() {
        let kind = match row["kind"].as_str().unwrap() {
            "" => 0,
            "length" => 1,
            "byte" => 2,
            other => panic!("{other}"),
        };
        checks.push(format!(
            "prefix-is {} {} {kind} {} {}",
            bytes(row["encoded_hex"].as_str().unwrap()),
            bytes(row["decoded_hex"].as_str().unwrap()),
            row["offset"].as_i64().unwrap().max(0),
            row["invalid"].as_i64().unwrap().max(0)
        ));
    }
    for row in fixture["buffers"].as_array().unwrap() {
        let raw = row["hex"].as_str().unwrap();
        let encoded: String = raw.as_bytes().iter().map(|v| format!("{v:02x}")).collect();
        checks.push(format!(
            "bytes-equal (hex-encode-bytes {}) {}",
            bytes(raw),
            bytes(&encoded)
        ));
        checks.push(format!(
            "bytes-equal (hex-encode-append (bytes-from-u8s [17,34,51]) {}) {}",
            bytes(raw),
            bytes(&format!("112233{encoded}"))
        ));
    }
    run(checks);
}

/// Capacity errors are atomic, valid prefixes survive errors, and overlapping decode inputs are snapshotted.
#[test]
fn hexadecimal_caller_buffers_preserve_bounds_progress_and_overlap() {
    run(vec![
        "put-encode".into(),
        "put-encode-short".into(),
        "put-decode".into(),
        "put-decode-short".into(),
        "put-decode-overlap".into(),
        "empty-and-invalid-offsets".into(),
    ]);
}

/// Every possible input byte is checked at every scalar/SIMD lane position across two blocks.
#[test]
fn hexadecimal_all_byte_values_preserve_first_error_and_prefix() {
    let digits: String = (0u16..256)
        .map(|v| {
            let b = v as u8;
            let digit = match b {
                b'0'..=b'9' => b - b'0',
                b'a'..=b'f' => b - b'a' + 10,
                b'A'..=b'F' => b - b'A' + 10,
                _ => 255,
            };
            format!("{digit:02x}")
        })
        .collect();
    run(vec![format!("all-positions {} 0", bytes(&digits))]);
}

/// Dump rows agree byte-for-byte with independent fixtures, including every byte and short rows.
#[test]
fn hexadecimal_dumps_match_reference_columns_and_offset_wrap() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/go-stdlib-vectors.json")).unwrap();
    let mut checks = Vec::new();
    for row in fixture["buffers"].as_array().unwrap() {
        let dump = row["hex_dump"].as_str().unwrap();
        let expected: String = dump
            .as_bytes()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        let input = bytes(row["hex"].as_str().unwrap());
        checks.push(format!(
            "bytes-equal (hex-dump-bytes 0 {input}) {}",
            bytes(&expected)
        ));
        checks.push(format!(
            "hex-dump {input} == {}",
            serde_json::to_string(dump).unwrap()
        ));
    }
    let mut expected = String::new();
    for offset in [u32::MAX - 15, 0] {
        expected.push_str(&format!("{offset:08x}  "));
        for i in 0..16 {
            expected.push_str("00 ");
            if i == 7 {
                expected.push(' ');
            }
        }
        expected.push_str(" |................|\n");
    }
    let expected: String = expected
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    checks.push(format!(
        "bytes-equal (hex-dump-bytes 4294967280 (bytes-new 32)) {}",
        bytes(&expected)
    ));
    run(checks);
}

/// Error diagnostics preserve the original position and both capacity counts in library-owned terms.
#[test]
fn hexadecimal_errors_render_actionable_diagnostics() {
    run(vec![
        "hex-error-message OddHexLength == \"unpaired final hexadecimal digit\"".into(),
        "hex-error-message (InvalidHexByte 17 255) == \"invalid hexadecimal byte 255 at byte 17\"".into(),
        "hex-error-message (HexShortDestination 8 3) == \"hexadecimal output requires 8 bytes; available 3\"".into(),
    ]);
}

/// Portable 128-bit SIMD and SWAR hex paths compile for 32-bit, ARM and big-endian targets.
#[test]
fn hexadecimal_dump_kernels_compile_for_word_widths_and_byte_orders() {
    let directory = tempfile::tempdir().unwrap();
    for (triple, arch, endian) in [
        ("wasm32-unknown-unknown", "wasm32", "little"),
        ("aarch64-unknown-linux-gnu", "aarch64", "little"),
        ("powerpc64-unknown-linux-gnu", "powerpc64", "big"),
    ] {
        let profile = cinch::driver::target_profile(&cinch::driver::DriverOptions {
            target: Some(triple.into()),
            ..Default::default()
        })
        .unwrap_or_else(|_| {
            // PowerPC checks LLVM portability only; it is not a supported driver link target.
            assert_eq!(arch, "powerpc64");
            let mut p = cinch::core::TargetProfile::native();
            p.name = "hex-powerpc64-kernel-check".into();
            p.triple = triple.into();
            p.arch = arch.into();
            p.endian = endian.into();
            p
        });
        let project = directory.path().join(arch);
        std::fs::create_dir_all(&project).unwrap();
        std::fs::write(
            project.join("Text.Hex.cinch"),
            include_str!("../prelude/Text/Hex.cinch"),
        )
        .unwrap();
        std::fs::write(
            project.join("Main.cinch"),
            "-- | Reachable public hexadecimal encode, decode and dump paths for foreign targets.\nmodule Main where\n  import Text.Hex\n  import Data.Bytes\n  import Data.Machine\n  import Data.Nat\n  import Data.Option\n\n  -- | Exercise public 128-bit SIMD encoding, decoding and dumping paths across targets.\n  main :: Int\n  main =\n    let input = bytes-new 32 in\n    let encoded = hex-encode-bytes input in\n    let dumped = hex-dump-bytes 0 input in\n    case hex-decode-prefix encoded of\n      (decoded, None) -> if bytes-len decoded == 32 && bytes-len dumped > 0 then 0 else 1\n      (decoded, Some err) -> 1\n",
        )
        .unwrap();
        let entry = project.join("Main.cinch");
        let ir_path = project.join("dump.ll");
        let ir = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
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
        assert!(ir.contains("<16 x i8>") && ir.contains("bitcast <16 x i1>"));
        if endian == "big" {
            assert!(ir.contains("call i128 @llvm.bswap.i128"));
        }
        let path = project.join("dump.ll");
        let result = std::process::Command::new(
            std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()),
        )
        .args(["-target", triple, "-O2", "-c", "-x", "ir"])
        .arg(&path)
        .arg("-o")
        .arg(path.with_extension("o"))
        .output()
        .unwrap();
        assert!(
            result.status.success(),
            "{triple}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
}

/// Public operations, errors and private SIMD/buffer kernels have canonical library documentation.
#[test]
fn hexadecimal_declarations_have_canonical_documentation() {
    let source = include_str!("../prelude/Text/Hex.cinch");
    let parsed =
        cinch::cinchdoc::parse_source(source, Some("Text.Hex"), "Text.Hex", None, false).unwrap();
    assert!(parsed.diagnostics.is_empty());
    assert!(parsed.orphans.is_empty());
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    for item in declarations {
        let doc = item
            .doc
            .as_ref()
            .unwrap_or_else(|| panic!("{}: missing docs", item.id));
        assert!(!doc.legacy);
        let text = doc.markdown.to_lowercase();
        assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
    }
}

/// Runtime helper consumers observe both completed bytes and errors, including bytes outside UTF-8.
const HELPERS: &str = r#"module Main where
  import Text.Hex

  -- | Compare byte contents without converting arbitrary bytes into text.
  bytes-equal :: Bytes -> Bytes -> Bool
  bytes-equal a b = eq-i64 (bytes-compare a b) 0

  -- | Match a fixture's independent error category, original offset and invalid byte.
  error-is :: Option HexError -> Nat -> Nat -> Nat -> Bool
  error-is error kind offset invalid = case error of
    None -> kind == 0
    Some OddHexLength -> kind == 1
    Some (InvalidHexByte position byte) -> kind == 2 && position == offset && byte == invalid
    Some (HexShortDestination required available) -> False

  -- | Exercise prefix and append APIs against the same independent expected output.
  prefix-is :: Bytes -> Bytes -> Nat -> Nat -> Nat -> Bool
  prefix-is input expected kind offset invalid = case hex-decode-prefix input of
    (out, error) -> bytes-equal out expected && error-is error kind offset invalid &&
      (case hex-decode-append (bytes-from-u8s [17,34,51]) input of
        (appended, failure) -> bytes-equal appended (join (bytes-from-u8s [17,34,51]) expected) && error-is failure kind offset invalid)

  -- | Join expected bytes independently through the bulk range-copy primitive.
  join :: Bytes -> Bytes -> Bytes
  join a b = let out = bytes-new (bytes-len a + bytes-len b) in
    let first = bytes-copy-range out a 0 0 (bytes-len a) in
    let last = bytes-copy-range out b (bytes-len a) 0 (bytes-len b) in out

  -- | Encode into an interior interval and preserve its sentinels and source.
  put-encode :: Bool
  put-encode = let out = bytes-from-u8s [85,85,85,85,85,85] in
    let input = bytes-from-u8s [171,205] in
    case hex-encode-into out 1 input of
      Ok count -> count == 4 && bytes-equal out (bytes-from-u8s [85,97,98,99,100,85]) && bytes-equal input (bytes-from-u8s [171,205])
      Err error -> False

  -- | Reject a short destination before changing its first byte.
  put-encode-short :: Bool
  put-encode-short = let out = bytes-from-u8s [85,85,85] in
    case hex-encode-into out 1 (bytes-from-u8s [171,205]) of
      Err (HexShortDestination required available) -> required == 4 && available == 2 && bytes-equal out (bytes-from-u8s [85,85,85])
      _ -> False

  -- | Preserve only the completed decoded pairs when a later byte is invalid.
  put-decode :: Bool
  put-decode = let out = bytes-from-u8s [85,85,85,85,85,85] in
    case hex-decode-into out 2 (bytes-from-u8s [48,48,49,49,103,103]) of
      (count, Some (InvalidHexByte offset byte)) -> count == 2 && offset == 4 && byte == 103 && bytes-equal out (bytes-from-u8s [85,85,0,17,85,85])
      _ -> False

  -- | Capacity validation precedes decoding and does not partially overwrite the destination.
  put-decode-short :: Bool
  put-decode-short = let out = bytes-from-u8s [85,85,85] in
    case hex-decode-into out 0 (bytes-from-u8s [48,48,48,49,48,50,48,51]) of
      (count, Some (HexShortDestination required available)) -> count == 0 && required == 4 && available == 3 && bytes-equal out (bytes-from-u8s [85,85,85])
      _ -> False

  -- | Decode over later encoded input without reading bytes already overwritten by output.
  put-decode-overlap :: Bool
  put-decode-overlap = let out = bytes-from-u8s [51,48,51,49,51,50,51,51,51,52,51,53,51,54,51,55] in
    case hex-decode-into out 4 out of
      (count, None) -> count == 8 && bytes-equal out (bytes-from-u8s [51,48,51,49,48,49,50,51,52,53,54,55,51,54,51,55])
      _ -> False

  -- | Empty operations at the end are valid; an offset beyond the end is rejected even for empty input.
  empty-and-invalid-offsets :: Bool
  empty-and-invalid-offsets = let out = bytes-from-u8s [85] in
    (case hex-encode-into out 1 (bytes-new 0) of Ok count -> count == 0; _ -> False) &&
    (case hex-decode-into out 1 (bytes-new 0) of (count, None) -> count == 0; _ -> False) &&
    (case hex-decode-into out 2 (bytes-new 0) of (count, Some (HexShortDestination required available)) -> count == 0 && required == 0 && available == 0; _ -> False) &&
    bytes-equal out (bytes-from-u8s [85])

  -- | Fill an independently allocated ASCII-zero corpus for lane-position tests.
  zero-digits :: Bytes -> Nat -> Unit
  zero-digits input i = if i < bytes-len input then
    let stored = bytes-set input i 48 in zero-digits input (i + 1)
    else ()

  -- | Check all input bytes at one position against an independent 256-byte nibble table.
  all-values :: Bytes -> Bytes -> Nat -> Nat -> Bool
  all-values digits input position byte = if byte < 256 && position < bytes-len input then
    let changed = bytes-set input position byte in
    let expected = if byte < bytes-len digits then bytes-get digits byte else 255 in
    let valid = (case hex-decode-prefix input of (out, error) -> if expected == 255 then bytes-len out == nat-div position 2 && error-is error 2 position byte else bytes-len out == 9 && error-is error 0 0 0 && values-is out 0 (nat-div position 2) (if nat-rem position 2 == 0 then expected * 16 else expected)) in
    let restored = bytes-set input position 48 in
    valid && all-values digits input position (byte + 1)
    else True

  -- | Observe every decoded byte after a valid replacement, not merely the output length.
  values-is :: Bytes -> Nat -> Nat -> Nat -> Bool
  values-is out i changed expected = if i < bytes-len out then
    bytes-get out i == (if i == changed then expected else 0) && values-is out (i + 1) changed expected
    else True

  -- | Move the replaced byte across two SIMD blocks and their scalar boundary.
  all-positions :: Bytes -> Nat -> Bool
  all-positions digits position = if position < 18 then
    let input = bytes-new 18 in
    let filled = zero-digits input 0 in
    all-values digits input position 0 && all-positions digits (position + 1)
    else True

  -- | Return the position of the first failed runtime assertion.
  inspect :: List Bool -> Int -> Int
  inspect checks n = case checks of Nil -> 0; Cons valid rest -> if valid then inspect rest (n + 1) else n + 1
"#;
