//! Independent fixtures, chunk invariance and lifetime checks for 128-bit FNV hashes.
mod common;

use cinch::driver::Mode;
use serde_json::Value;

/// Compile the real source modules and run a consumer with ownership validation.
fn run(checks: Vec<String>) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("Text.Fnv", include_str!("../prelude/Text/Fnv.cinch")),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), source).unwrap();
    }
    let mut source = HELPERS.to_string();
    source.push_str("\n  -- | Evaluate all runtime fixtures, returning the first failing position.\n  checks :: List Bool\n  checks = [\n");
    source.push_str(
        &checks
            .iter()
            .map(|check| format!("    ({check})"))
            .collect::<Vec<_>>()
            .join(",\n"),
    );
    source.push_str("\n    ]\n  -- | Exit successfully only when every independent check agrees.\n  main :: Int\n  main = inspect checks 0\n");
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
}

/// Whole buffers and every two-chunk split match independent reference digests.
#[test]
fn fnv128_matches_reference_vectors_and_every_chunk_split() {
    let fixture: Value =
        serde_json::from_str(include_str!("fixtures/go-stdlib-vectors.json")).unwrap();
    let mut checks = Vec::new();
    for row in fixture["buffers"].as_array().unwrap() {
        let input = row["hex"].as_str().unwrap();
        for (name, flag) in [("fnv128", "False"), ("fnv128a", "True")] {
            let digest = row[format!("{name}_hex")].as_str().unwrap();
            let expected = u128::from_str_radix(digest, 16).unwrap();
            checks.push(format!(
                "chunks-is {flag} (input \"{input}\") ({}, {}) 0",
                expected >> 64,
                expected as u64
            ));
        }
    }
    run(checks);
}

/// A separate scalar model covers arbitrary previous states and every block-tail length.
#[test]
fn fnv128_updates_preserve_arbitrary_high_bits_and_exact_tails() {
    let mut checks = Vec::new();
    for state in [
        0,
        1,
        u128::MAX,
        1u128 << 127,
        0x0123456789abcdef_fedcba9876543210,
    ] {
        for len in 0..=24 {
            let bytes: Vec<u8> = (0..len).map(|i| (i * 37 + 11) as u8).collect();
            let input: String = bytes.iter().map(|v| format!("{v:02x}")).collect();
            for alternate in [false, true] {
                let mut expected = state;
                for byte in &bytes {
                    if alternate {
                        expected ^= *byte as u128;
                    }
                    expected = expected.wrapping_mul(0x1000000000000000000013b);
                    if !alternate {
                        expected ^= *byte as u128;
                    }
                }
                let operation = if alternate {
                    "fnv128a-update"
                } else {
                    "fnv128-update"
                };
                checks.push(format!(
                    "hash-is ({operation} ({}, {}) (input \"{input}\")) ({}, {})",
                    state >> 64,
                    state as u64,
                    expected >> 64,
                    expected as u64
                ));
            }
        }
    }
    run(checks);
}

/// Long buffers exercise the recursive block loop beyond the stack depth of boxed helper returns.
#[test]
fn fnv128_large_buffers_keep_a_constant_stack() {
    let mut expected = 0x6c62272e07bb014262b821756295c58du128;
    for _ in 0..1048576 {
        expected = expected.wrapping_mul(0x1000000000000000000013b);
    }
    run(vec![format!(
        "hash-is (fnv128a (bytes-new 1048576)) ({}, {})",
        expected >> 64,
        expected as u64
    )]);
}

/// Cross compilation preserves the wide recurrence on 32-bit and both byte orders.
#[test]
fn fnv128_kernels_compile_for_32_bit_arm_and_big_endian_targets() {
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
            let mut profile = cinch::core::TargetProfile::native();
            profile.name = "fnv128-powerpc64-kernel-check".into();
            profile.triple = triple.into();
            profile.arch = arch.into();
            profile.endian = endian.into();
            profile
        });
        let source = format!("{}\n{}\nmodule Main where\n  import Text.Fnv\n  main :: Int\n  main = case fnv128a (bytes-new 17) of (high, low) -> if eq-u64 high low then 1 else 0\n", include_str!("../prelude/Data/Bytes/Word.cinch"), include_str!("../prelude/Text/Fnv.cinch"));
        let (profile, program) = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::check_sources_profile(&source, "fnv128-target.cinch", &profile)
                    .map(|program| (profile, program))
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        let ir = cinch::codegen::emit_for(&profile, &program).unwrap();
        assert!(ir.contains("mul i128"));
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("consumer.ll");
        std::fs::write(&path, ir).unwrap();
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

/// Documentation uses the library's own vocabulary and the canonical comment form.
#[test]
fn fnv128_declarations_have_canonical_documentation() {
    let source = include_str!("../prelude/Text/Fnv.cinch");
    assert!(!source.to_lowercase().contains("golang"));
    let parsed =
        cinch::cinchdoc::parse_source(source, Some("Text.Fnv"), "Text.Fnv", None, false).unwrap();
    assert!(parsed.diagnostics.is_empty());
    assert!(parsed.orphans.is_empty());
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    for declaration in declarations {
        let doc = declaration
            .doc
            .as_ref()
            .unwrap_or_else(|| panic!("{}: missing docs", declaration.id));
        assert!(!doc.legacy);
        let text = doc.markdown.to_lowercase();
        assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
    }
}

/// Runtime reads prevent reference-vector checks from becoming literal-only hashes.
const HELPERS: &str = r#"module Main where
  import Text.Fnv
  import Text.Hex

  -- | Decode independent fixture bytes at runtime.
  input :: String -> Bytes
  input text = case hex-decode text of Err error -> bytes-new 0; Ok bytes -> bytes

  -- | Compare both numeric words of a full 128-bit digest, high word first.
  hash-is :: (U64, U64) -> (U64, U64) -> Bool
  hash-is actual expected = case actual of (ah, al) -> case expected of (eh, el) -> eq-u64 ah eh && eq-u64 al el

  -- | Snapshot a checked byte range so chunk updates receive separate owned buffers.
  part :: Bytes -> Nat -> Nat -> Bytes
  part bytes start end = if start <= end && end <= bytes-len bytes then
    let out = bytes-new (end - start) in
    let copied = bytes-copy-range out bytes 0 start (end - start) in out
    else bytes-new 0

  -- | Verify every split, including empty prefix, empty suffix and the unsplit input.
  chunks-is :: Bool -> Bytes -> (U64, U64) -> Nat -> Bool
  chunks-is alternate bytes expected i =
    if i <= bytes-len bytes then
      let first = part bytes 0 i in
      let second = part bytes i (bytes-len bytes) in
      let actual = if alternate then fnv128a-update (fnv128a first) second else fnv128-update (fnv128 first) second in
      hash-is actual expected && chunks-is alternate bytes expected (i + 1)
    else True

  -- | Return a stable nonzero position for the first failed consumer check.
  inspect :: List Bool -> Int -> Int
  inspect checks n = case checks of
    Nil -> 0
    Cons valid rest -> if valid then inspect rest (n + 1) else n + 1
"#;
