//! UTF-16 scalar, surrogate and byte-codec consumers against independent fixtures.
mod common;

use cinch::driver::Mode;
use serde_json::Value;

/// Render code units with their explicit machine carrier.
fn units(values: &Value) -> String {
    format!(
        "([{}] :: List U16)",
        values
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

/// Compile actual library sources and execute with ownership checks and a bounded heap.
fn execute(source: &str) -> String {
    let dir = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("Text.Utf16", include_str!("../prelude/Text/Utf16.cinch")),
        ("Text.Binary", include_str!("../prelude/Text/Binary.cinch")),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
    ] {
        std::fs::write(dir.path().join(format!("{name}.cinch")), source).unwrap();
    }
    std::fs::write(dir.path().join("Main.cinch"), source).unwrap();
    let exe = dir.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    common::drive_with_profile(
        Mode::Build,
        cinch::check::StageMode::Refresh,
        dir.path(),
        "Main.cinch",
        Some(exe.clone()),
        &profile,
    )
    .unwrap();
    let output = std::process::Command::new(&exe)
        .env("CINCH_RC_DEBUG", "1")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_HEAP_LIMIT", "32M")
        .output()
        .unwrap();
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    std::fs::read_to_string(exe.with_extension("ll")).unwrap()
}

/// Scalar boundaries and all representative pair combinations preserve replacement semantics.
#[test]
fn utf16_matches_scalar_and_surrogate_reference_vectors() {
    let data: Value =
        serde_json::from_str(include_str!("fixtures/go-stdlib-vectors.json")).unwrap();
    let mut checks = Vec::new();
    for row in data["utf16"]["scalars"].as_array().unwrap() {
        let value = format!("({} :: I64)", row["value"]);
        let length = row["length"].as_i64().unwrap();
        checks.push(format!(
            "(case utf16-rune-length {value} of None -> {}; Some n -> n == {})",
            if length < 0 { "True" } else { "False" },
            length.max(0)
        ));
        checks.push(format!(
            "utf16-is-surrogate {value} == {}",
            if row["surrogate"].as_bool().unwrap() {
                "True"
            } else {
                "False"
            }
        ));
        checks.push(format!(
            "(case utf16-encode-rune {value} of (a,b) -> equal [a,b] {})",
            units(&row["pair"])
        ));
        checks.push(format!(
            "equal (utf16-encode [{value}]) {}",
            units(&row["encoded"])
        ));
        checks.push(format!(
            "equal (utf16-append-rune [65,55296] {value}) (Cons 65 (Cons 55296 {}))",
            units(&row["encoded"])
        ));
    }
    for row in data["utf16"]["pairs"].as_array().unwrap() {
        checks.push(format!(
            "eq-i64 (utf16-decode-rune ({} :: I64) ({} :: I64)) ({} :: I64)",
            row["high"], row["low"], row["decoded"]
        ));
    }
    checks.push(
        "(case utf16-rune-length (-9223372036854775808 :: I64) of None -> True; Some n -> False)"
            .into(),
    );
    checks.push(
        "(case utf16-rune-length (9223372036854775807 :: I64) of None -> True; Some n -> False)"
            .into(),
    );
    run_checks(checks);
}

/// Whole-text codecs preserve byte order, unpaired-surrogate recovery, marks and ASCII block tails.
#[test]
fn utf16_byte_codecs_match_text_and_reject_odd_lengths() {
    let data: Value =
        serde_json::from_str(include_str!("fixtures/go-stdlib-vectors.json")).unwrap();
    let mut checks = Vec::new();
    for row in data["utf16"]["sequences"].as_array().unwrap() {
        let values = units(&row["units"]);
        let expected = row["decoded_hex"].as_str().unwrap();
        checks.push(format!(
            "hex-encode (string-to-bytes (pack (utf16-decode {values}))) == \"{expected}\""
        ));
        for order in ["LittleEndian", "BigEndian"] {
            checks.push(format!(
                "decode-hex {order} (wire {order} {values}) == \"{expected}\""
            ));
            checks.push(format!("roundtrip {order} (pack (utf16-decode {values}))"));
            checks.push(format!("eq-i64 (bytes-compare (utf16-encode-bytes {order} (pack (utf16-decode {values}))) (wire {order} {})) 0", units(&row["normalized_units"])));
        }
    }
    checks.push("ascii-lengths 0".into());
    checks.push("all-units 0".into());
    checks.push("odd-lengths 1".into());
    let ir = run_checks(checks);
    // The self-built compiler gives private primitives hygienic symbol names.
    // Each direction must retain its actual SWAR pair mask in the emitted IR.
    assert!(ir.matches("281470681808895").count() >= 2);
    assert!(ir.contains("71777214294589695"));
}

/// Every new declaration, including private SWAR helpers, has a canonical doc comment.
#[test]
fn utf16_declarations_have_canonical_documentation() {
    let source = include_str!("../prelude/Text/Utf16.cinch");
    let parsed =
        cinch::cinchdoc::parse_source(source, Some("Text.Utf16"), "Text.Utf16", None, false)
            .unwrap();
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
    assert!(parsed.orphans.is_empty(), "{:?}", parsed.orphans);
    assert!(!source.to_lowercase().contains("golang"));
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    for declaration in declarations {
        let doc = declaration
            .doc
            .as_ref()
            .unwrap_or_else(|| panic!("{}: missing docs", declaration.id));
        assert!(!doc.legacy, "{}: use canonical docs", declaration.id);
        let text = doc.markdown.to_lowercase();
        assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
    }
}

/// The actual SWAR UTF-16 paths compile for WebAssembly, little-endian ARM and big-endian PowerPC.
/// Cross compilation checks lowering, without claiming execution on those machines.
#[test]
fn utf16_swar_kernels_compile_for_arm_and_big_endian() {
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
            assert_eq!(arch, "powerpc64");
            let mut profile = cinch::core::TargetProfile::native();
            profile.triple = triple.into();
            profile.arch = arch.into();
            profile.endian = endian.into();
            profile
        });
        let project = directory.path().join(arch);
        std::fs::create_dir_all(&project).unwrap();
        for (name, source) in [
            ("Text.Utf16", include_str!("../prelude/Text/Utf16.cinch")),
            ("Text.Binary", include_str!("../prelude/Text/Binary.cinch")),
            (
                "Data.Bytes.Word",
                include_str!("../prelude/Data/Bytes/Word.cinch"),
            ),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), source).unwrap();
        }
        std::fs::write(
            project.join("Main.cinch"),
            "-- | Reachable public UTF-16 encode and decode paths for foreign targets.\nmodule Main where\n  import Text.Utf16\n  import Text.Binary\n  import Data.Bytes\n  import Data.Nat\n  import Data.Result\n\n  -- | Exercise public SWAR widening, narrowing and byte-order swap paths across targets.\n  main :: Int\n  main =\n    let raw = bytes-new 16 in\n    case utf16-decode-bytes BigEndian raw of\n      Err err -> 1\n      Ok text ->\n        let le = utf16-encode-bytes LittleEndian text in\n        let be = utf16-encode-bytes BigEndian text in\n        if bytes-len le == 16 && bytes-len be == 16 then 0 else 1\n",
        )
        .unwrap();
        let entry = project.join("Main.cinch");
        let path = project.join(format!("{arch}.ll"));
        let ir_path = path.clone();
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
        assert!(ir.contains("281470681808895") && ir.contains("71777214294589695"));
        if endian == "big" {
            assert!(
                ir.contains("call i64 @llvm.bswap.i64") && ir.contains("call i32 @llvm.bswap.i32")
            );
        }
        let output = std::process::Command::new(
            std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()),
        )
        .args(["-target", triple, "-O2", "-c", "-x", "ir"])
        .arg(&path)
        .arg("-o")
        .arg(path.with_extension("o"))
        .output()
        .unwrap();
        assert!(
            output.status.success(),
            "{triple}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

/// Keep checks observable through compiled consumers while assigning a useful failure code.
fn run_checks(checks: Vec<String>) -> String {
    let mut source = HELPERS.to_string();
    for (index, check) in checks.iter().enumerate() {
        source.push_str(&format!(
            "\n  step{index} :: Int\n  step{index} = if ({check}) then step{} else {}\n",
            index + 1,
            index % 250 + 1
        ));
    }
    source.push_str(&format!(
        "\n  step{} :: Int\n  step{} = 0\n  main :: Int\n  main = step0\n",
        checks.len(),
        checks.len()
    ));
    execute(&source)
}

/// Shared test consumers exercise explicit words, independent wire assembly and runtime boundaries.
const HELPERS: &str = r#"module Main where
  import Text.Utf16
  import Text.Binary
  import Text.Hex
  import Data.Machine

  -- | Compare lists without relying on a machine-word Eq dictionary.
  equal :: List U16 -> List U16 -> Bool
  equal a b = case (a,b) of
    (Nil,Nil) -> True
    (Cons x xs, Cons y ys) -> eq-u16 x y && equal xs ys
    other -> False
  -- | Join two independent byte buffers.
  join :: Bytes -> Bytes -> Bytes
  join a b = let n = bytes-len a in let out = bytes-new (n + bytes-len b) in
    let first = bytes-copy-range out a 0 0 n in
    let second = bytes-copy-range out b n 0 (bytes-len b) in out
  -- | Assemble the reference units using the separately tested integer wire codec.
  wire :: ByteOrder -> List U16 -> Bytes
  wire order values = case values of
    Nil -> bytes-new 0
    Cons value rest -> join (binary-u16 order value) (wire order rest)
  -- | Observe decoded output as hex, reserving a sentinel for unexpected failures.
  decode-hex :: ByteOrder -> Bytes -> String
  decode-hex order input = case utf16-decode-bytes order input of
    Err error -> "failure"
    Ok text -> hex-encode (string-to-bytes text)
  -- | Round-trip valid text through the explicit-order byte codec.
  roundtrip :: ByteOrder -> String -> Bool
  roundtrip order text = case utf16-decode-bytes order (utf16-encode-bytes order text) of
    Err error -> False
    Ok decoded -> decoded == text
  -- | Cover both sides of four-byte/eight-byte SWAR boundaries and embedded NUL.
  ascii-lengths :: Nat -> Bool
  ascii-lengths n = if n > 33 then True else
    let text = string-concat (string-repeat n "a") "\u{0}z" in
    roundtrip LittleEndian text && roundtrip BigEndian text &&
    bytes-len (utf16-encode-bytes LittleEndian text) == (n + 2) * 2 && ascii-lengths (n + 1)
  -- | Exhaust every single code unit, including every unpaired surrogate.
  all-units :: Nat -> Bool
  all-units n = if n > 65535 then True else
    let unit = wrapping-i64-to-u16 (nat-to-i64 n) in
    case utf16-decode [unit] of
      Cons c Nil -> char-code c == (if 55296 <= n && n < 57344 then 65533 else n) && all-units (n + 1)
      other -> False
  -- | Odd inputs fail before decoding, with the exact final-byte offset.
  odd-lengths :: Nat -> Bool
  odd-lengths n = if n > 35 then True else
    case utf16-decode-bytes LittleEndian (bytes-new n) of
      Err (OddUtf16ByteCount offset) -> offset + 1 == n && odd-lengths (n + 2)
      other -> False
"#;
