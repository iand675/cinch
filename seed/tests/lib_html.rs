//! Complete HTML5 named/numeric references, byte preservation and buffer ownership.
mod common;

use cinch::driver::Mode;
use serde_json::Value;

/// Render a compact runtime fixture stream without placing large buffer setup in constant evaluation.
fn consumer() -> String {
    let fixture: Value = serde_json::from_str(include_str!("fixtures/html-vectors.json")).unwrap();
    let mut fields = String::new();
    for row in fixture["rows"].as_array().unwrap() {
        for key in ["input", "escaped", "unescaped"] {
            fields.push_str(row[key].as_str().unwrap());
            fields.push('\0');
        }
    }
    let literal = serde_json::to_string(&fields)
        .unwrap()
        .replace("\\u0000", "\\u{0}");
    let checks = fixture["long"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            format!(
                "long-case {} {} {} {} {} {} {}",
                row["size"],
                row["mode"],
                row["input_hash"],
                row["escaped_size"],
                row["escaped_hash"],
                row["unescaped_size"],
                row["unescaped_hash"]
            )
        })
        .collect::<Vec<_>>()
        .join(",\n      ");
    include_str!("fixtures/html-consumer.cinch.in")
        .replace("__ROWS__", &literal)
        .replace("__LONG__", &checks)
}

/// Build the actual source namespace and check ordinary live space separately from poisoned frees.
fn run(source: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("Text.Html", include_str!("../prelude/Text/Html.cinch")),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
        ("Text.Fnv", include_str!("../prelude/Text/Fnv.cinch")),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), source).unwrap();
    }
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
    for debug in [false, true] {
        let output = std::process::Command::new(&exe)
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

/// All named references, malformed numeric prefixes, exact scalar boundaries and raw bytes agree.
#[test]
fn html_reference_corpus_and_owned_buffers_match_independent_results() {
    run(&consumer());
}

/// Checked byte operands retain their ABI width through an explicit binding or typed wrapper.
#[test]
fn html_inline_descriptor_accepts_checked_byte_bindings() {
    for source in [
        include_str!("fixtures/html-abi-bound.cinch"),
        include_str!("fixtures/html-abi-wrapped.cinch"),
    ] {
        run(source);
    }
}

/// Every module-owned declaration has canonical documentation without borrowed language comparisons.
#[test]
fn html_declarations_have_canonical_documentation() {
    let source = include_str!("../prelude/Text/Html.cinch");
    let parsed =
        cinch::cinchdoc::parse_source(source, Some("Text.Html"), "Text.Html", None, false).unwrap();
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

/// The actual 16-byte SIMD escaping, unescaping and entity-lookup paths remain legal on foreign objects.
/// This checks the Rust emitter explicitly; selected host frontend tests never fall back.
#[test]
fn html_kernels_compile_for_wasm_arm_and_big_endian_targets() {
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
        std::fs::write(
            project.join("Text.Html.cinch"),
            include_str!("../prelude/Text/Html.cinch"),
        )
        .unwrap();
        std::fs::write(
            project.join("Main.cinch"),
            "-- | Reachable public HTML escaping and unescaping paths for foreign targets.\nmodule Main where\n  import Text.Html\n  import Data.Bytes\n  import Data.Nat\n\n  -- | Exercise public 16-byte SIMD escaping and unescaping paths across targets.\n  main :: Int\n  main =\n    let input = bytes-new 32 in\n    let escaped = html-escape-bytes input in\n    let unescaped = html-unescape-bytes escaped in\n    if bytes-len unescaped == bytes-len input && html-escaped-length input >= bytes-len input then 0 else 1\n",
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
        assert!(ir.contains("<16 x i8>") && ir.contains("bitcast <16 x i1>"));
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
