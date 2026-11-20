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

/// Actual byte arithmetic and exact-load kernels compile for wasm, ARM and a big-endian object.
#[test]
fn utf8_readonly_kernels_compile_for_foreign_targets() {
    let source = include_str!("../prelude/Data/Bytes/Utf8.cinch");
    let mut kernels = String::from("-- | Actual private UTF-8 arithmetic and source loads.\nmodule Main where\n  import Data.Bytes\n  import Data.Machine\n");
    for name in [
        "load-one",
        "load-four",
        "load-eight",
        "ascii-eight",
        "ascii-sixteen",
        "decode-word",
        "encode-word",
        "scan-lt",
        "scan-ge",
        "scan-gt",
        "scan-eq",
        "scan-add",
        "scan-sub",
        "scan-address",
        "packed-width",
        "packed-valid",
        "scan-simple",
        "scan-ready",
        "scan-four",
        "scan-from-packed",
        "scan-width",
        "scan-valid",
    ] {
        let start = source.find(&format!("  prim {name} ::")).unwrap();
        let rest = &source[start..];
        let end = rest.find("\n  -- |").unwrap_or(rest.len());
        kernels.push_str(
            "\n  -- | Isolated actual checked arithmetic or caller-proved exact source load.\n",
        );
        kernels.push_str(&rest[..end]);
    }
    kernels.push_str("\n  -- | Retain source storage throughout exact private reads.\n  keep :: forall a. Bytes -> a -> a\n  keep bytes value = case bytes-len bytes of _ -> value\n  -- | Give each computed boolean a distinct observable result bit.\n  flag :: Bool -> U64 -> U64\n  flag value mask = if value then mask else 0\n  -- | Reach actual read-only kernels without algebraically contradictory exit conditions.\n  main :: Int\n  main = let bytes = bytes-new 16 in let base = bytes-data bytes in\n    let one = keep bytes (load-one base) in let four = keep bytes (load-four base) in\n    let eight = keep bytes (load-eight base) in let address = scan-address base eight in\n    let packed = decode-word four one in let encoded = encode-word (wrapping-u32-to-i32 packed) in\n    let simple = scan-simple one in let checked = scan-four four in let converted = scan-from-packed packed in\n    let a = scan-width checked in let b = packed-width packed in let next = scan-add a b in let rest = scan-sub encoded next in\n    let x = bit-or-u64 (flag (ascii-eight eight) 1) (flag (keep bytes (ascii-sixteen base)) 2) in\n    let y = bit-or-u64 (flag (scan-lt next rest) 4) (flag (scan-ge rest eight) 8) in\n    let z = bit-or-u64 (flag (scan-gt encoded next) 16) (flag (scan-eq next rest) 32) in\n    let p = bit-or-u64 (flag (packed-valid packed) 64) (flag (scan-valid converted) 128) in\n    let q = bit-or-u64 (flag (scan-ready simple) 256) (flag (ne-i64 address base) 512) in\n    let flags = bit-or-u64 (bit-or-u64 x y) (bit-or-u64 (bit-or-u64 z p) q) in\n    let score = scan-add rest flags in if eq-u64 score eight then 0 else 1\n");
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
