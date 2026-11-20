//! Exact JSON syntax, formatting, numeric ranges and source-buffer ownership.
mod common;

use cinch::driver::Mode;

/// Build a source-distributed JSON consumer and check ordinary and poisoned reclamation.
fn run(source: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (name, contents) in [
        (
            "Text.Json.Binary32",
            include_str!("../prelude/Text/Json/Binary32.cinch"),
        ),
        (
            "Text.Json.Value",
            include_str!("../prelude/Text/Json/Value.cinch"),
        ),
        ("Data.Array", include_str!("../prelude/Data/Array.cinch")),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
        (
            "Data.Unicode.Kernel",
            include_str!("../prelude/Data/Unicode/Kernel.cinch"),
        ),
        (
            "Data.Unicode.Tables",
            include_str!("../prelude/Data/Unicode/Tables.cinch"),
        ),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), contents).unwrap();
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
        let result = std::process::Command::new(&executable)
            .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
            .env("CINCH_STACK_SIZE", "8M")
            .output()
            .unwrap();
        assert_eq!(
            result.status.code(),
            Some(0),
            "debug={debug}: stdout={} stderr={}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
        let diagnostic = String::from_utf8_lossy(&result.stderr);
        assert!(diagnostic.contains("live=0") && diagnostic.contains("leaked=0"));
    }
}

/// Independent arbitrary byte grammar and formatting cases retain exact errors and offsets.
#[test]
fn json_value_syntax_and_formatting_match_independent_observations() {
    run(include_str!("fixtures/json-codec-syntax.cinch"));
}

/// Control bytes, scalar boundaries and malformed UTF-8 have complete encode/decode comparisons.
#[test]
fn json_value_quoting_preserves_required_escapes_and_replacement_scalars() {
    run(include_str!("fixtures/json-codec-quote.cinch"));
}

/// Exact signed limits, subnormals, overflow and original number spellings are independently pinned.
#[test]
fn json_value_numbers_preserve_lexemes_and_checked_ranges() {
    run(include_str!("fixtures/json-codec-number.cinch"));
}

/// Arbitrary byte indentation fragments preserve malformed bytes, NUL and unusual line prefixes.
#[test]
fn json_value_byte_indentation_preserves_arbitrary_prefixes() {
    run(include_str!("fixtures/json-codec-indent.cinch"));
}

/// Mutable source/export aliases, 64KiB fields, maximum depth and every lexer split remain bounded.
#[test]
fn json_value_buffers_positions_and_chunk_boundaries_are_independent() {
    run(include_str!("fixtures/json-codec-control.cinch"));
}

/// Every public/private declaration and constructor owns canonical standard-library documentation.
#[test]
fn json_value_declarations_have_canonical_documentation() {
    let source = include_str!("../prelude/Text/Json/Value.cinch");
    let parsed = cinch::cinchdoc::parse_source(
        source,
        Some("Text.Json.Value"),
        "Text.Json.Value",
        None,
        false,
    )
    .unwrap();
    assert!(parsed.doc.is_some());
    assert!(parsed.diagnostics.is_empty() && parsed.orphans.is_empty());
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    for declaration in declarations {
        let doc = declaration
            .doc
            .as_ref()
            .unwrap_or_else(|| panic!("{}: missing documentation", declaration.id));
        assert!(!doc.legacy);
        let text = doc.markdown.to_lowercase();
        assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
    }
}

/// Compile the actual private vector classifier for little- and big-endian foreign objects.
#[test]
fn rust_json_kernels_compile_for_wasm_arm_and_big_endian_targets() {
    let source = include_str!("../prelude/Text/Json/Value.cinch");
    let start = source.find("  prim block16 ::").unwrap();
    let remaining = &source[start..];
    let end = remaining.find("\n  -- |").unwrap_or(remaining.len());
    let mut kernels = String::from("-- | Isolated JSON vector classification.\nmodule Main where\n  import Data.Machine\n  -- | Actual complete-interval load-only classifier.\n");
    kernels.push_str(&remaining[..end]);
    for name in ["numeric-hex-round", "numeric-float-from-bits"] {
        let start = source.find(&format!("  prim {name} ::")).unwrap();
        let rest = &source[start..];
        let end = rest.find("\n  -- |").unwrap_or(rest.len());
        kernels.push_str("\n  -- | Actual raw numeric representation and rounding primitive.\n");
        kernels.push_str(&rest[..end]);
    }
    kernels.push_str("\n  -- | Retain the source owner through its checked private load.\n  keep :: Bytes -> U8 -> U8\n  keep bytes value = case bytes-len bytes of _ -> value\n  -- | Keep the complete classifier reachable without claiming foreign execution.\n  main :: Int\n  main = let bytes = bytes-new 16 in let a = keep bytes (block16 (bytes-data bytes) 0) in let b = keep bytes (block16 (bytes-data bytes) 1) in let prefix = wrapping-i64-to-u64 (bytes-data bytes) in let count = bit-and-u32 (wrapping-i64-to-u32 (bytes-data bytes)) 127 in if eq-u8 a b && is-finite-f64 (numeric-float-from-bits (numeric-hex-round prefix count 1)) then 0 else 1\n");
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
                    cinch::driver::check_sources_profile(&input, "json-kernels.cinch", &profile)?;
                cinch::codegen::emit_for(&profile, &program)
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("<16 x i8>") && ir.contains("load <16 x i8>"));
        assert!(
            ir.contains("icmp ult i32") && ir.contains("select i1") && ir.contains("bitcast i64")
        );
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
