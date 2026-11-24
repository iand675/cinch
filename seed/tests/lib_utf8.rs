//! Exact raw-byte UTF-8 progress, fresh repair, atomic encoding and target-portable read-only kernels.
mod common;

use cinch::driver::Mode;

/// Compile the real byte module and verify ordinary live space separately from poisoned frees.
fn run(source: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        (
            "Data.Bytes.Utf8",
            include_str!("../prelude/Data/Bytes/Utf8.cinch"),
        ),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), source).unwrap();
    }
    std::fs::write(directory.path().join("Main.cinch"), source).unwrap();
    let executable = directory.path().join("consumer");
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
    for debug in [false, true] {
        let output = std::process::Command::new(&executable)
            .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_STACK_SIZE", "8M")
            .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "debug={debug}: {}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
}

/// Invalid numeric values, grouped repair, aliasing and exact-capacity refusal preserve all bytes.
#[test]
fn utf8_invalid_numeric_values_fresh_repair_and_atomic_encoding() {
    run(include_str!("fixtures/utf8-ownership-consumer.cinch"));
}

/// Absolute half-open span bounds preserve malformed fringes and never snapshot mutable owners.
#[test]
fn utf8_bounded_spans_preserve_exact_fringe_and_alias_semantics() {
    run(include_str!("fixtures/utf8-span-controls.cinch"));
}

/// Every byte-layer declaration, constructor and arithmetic helper has canonical documentation.
#[test]
fn utf8_declarations_have_canonical_documentation() {
    let source = include_str!("../prelude/Data/Bytes/Utf8.cinch");
    let name = "Data.Bytes.Utf8";
    let parsed = cinch::cinchdoc::parse_source(source, Some(name), name, None, false).unwrap();
    assert!(parsed.doc.is_some());
    assert!(parsed.diagnostics.is_empty());
    assert!(parsed.orphans.is_empty());
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    for declaration in declarations {
        let doc = declaration
            .doc
            .as_ref()
            .unwrap_or_else(|| panic!("missing docs: {}", declaration.id));
        assert!(!doc.legacy);
        let text = doc.markdown.to_lowercase();
        assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
    }
}

/// Actual byte arithmetic, SIMD ASCII blocks and exact-load kernels compile for wasm, ARM and a big-endian object.
#[test]
fn utf8_readonly_kernels_compile_for_foreign_targets() {
    let kernels = [
        include_str!("../prelude/Data/Bytes/Utf8.cinch"),
        r#"-- | Root complete public UTF-8 validation, decoding, counting, encoding and repair kernels.
module Main where
  import Data.Bool
  import Data.Bytes
  import Data.Bytes.Utf8
  import Data.Machine
  import Data.Nat
  import Data.Option

  -- | Exercise SIMD ASCII scanning, 4-byte multibyte decode, rune encoding and repair.
  main :: Int
  main =
    let raw = bytes-from-u8s [97,98,99,100,101,102,103,104,105,106,107,108,109,110,111,112,240,159,166,128] in
    let ok = utf8-valid raw in
    let count = utf8-count raw in
    let between = utf8-count-between raw 0 20 in
    let step = utf8-decode-at raw 16 in
    let last = utf8-last raw in
    let full = utf8-full-rune raw in
    let encoded = utf8-encode-rune 129408 in
    let repaired = utf8-repair raw encoded in
    let width-ok = utf8-step-width step == 4 && utf8-step-width last == 4 in
    let between-ok = case between of Some n -> n == 17; None -> False in
    if ok && count == 17 && between-ok && width-ok && full && bytes-len repaired == 20 then 0 else 1
"#,
    ]
    .join("\n");
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
        let input = kernels.clone();
        let ir = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let program =
                    cinch::driver::check_sources_profile(&input, "utf8-kernels.cinch", &profile)?;
                cinch::codegen::emit_for(&profile, &program)
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(
            ir.contains("load i128"),
            "{triple}: missing 16-byte SIMD ASCII load"
        );
        assert!(
            ir.contains("bitcast <16 x i1>"),
            "{triple}: missing SIMD ASCII movemask"
        );
        assert!(
            ir.contains("load i64"),
            "{triple}: missing 8-byte SWAR ASCII load"
        );
        assert!(
            ir.contains("load i32"),
            "{triple}: missing 4-byte UTF-8 lookahead load"
        );
        assert!(ir.contains("load i8"), "{triple}: missing scalar byte load");
        if endian == "big" {
            assert!(
                ir.contains("@llvm.bswap.i128")
                    && ir.contains("@llvm.bswap.i64")
                    && ir.contains("@llvm.bswap.i32"),
                "{triple}: missing big-endian word swaps"
            );
        }
        let path = directory.path().join(format!("{arch}.ll"));
        std::fs::write(&path, &ir).unwrap();
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
