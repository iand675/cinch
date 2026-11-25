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

/// Compile the actual public JSON vector classifier and hex-float paths for little- and big-endian foreign objects.
#[test]
fn rust_json_kernels_compile_for_wasm_arm_and_big_endian_targets() {
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
            std::fs::write(project.join(format!("{name}.cinch")), contents).unwrap();
        }
        std::fs::write(
            project.join("Main.cinch"),
            "-- | Reachable public JSON value scanning and float conversion paths for foreign targets.\nmodule Main where\n  import Text.Json.Value\n  import Data.Bytes\n  import Data.Machine\n  import Data.Nat\n  import Data.Result\n\n  -- | Exercise public 16-byte SIMD classification and hex-float rounding paths across targets.\n  main :: Int\n  main =\n    let input = bytes-new 32 in\n    let quoted = json-quote-bytes input True in\n    let parsed = json-number-float64-result (json-number-from-bytes input) in\n    case json-compact quoted of\n      Ok out -> if bytes-len out > 0 && is-finite-f64 (json-numeric-value parsed) then 0 else 1\n      Err err -> 1\n",
        )
        .unwrap();
        let entry = project.join("Main.cinch");
        let path = project.join(format!("{arch}.ll"));
        let ir_path = path.clone();
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
        assert!(
            ir.contains("<16 x i8>") && ir.contains("bitcast <16 x i1>") && ir.contains("load i128")
        );
        assert!(ir.contains("bitcast i64"));
        if endian == "big" {
            assert!(ir.contains("call i128 @llvm.bswap.i128"));
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
