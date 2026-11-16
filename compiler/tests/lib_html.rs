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

/// The actual SIMD, SWAR, exact-load and packed-name arithmetic remains legal on foreign objects.
/// This checks the Rust emitter explicitly; selected host frontend tests never fall back.
#[test]
fn html_kernels_compile_for_wasm_arm_and_big_endian_targets() {
    let source = include_str!("../prelude/Text/Html.cinch");
    let mut kernels = String::from("-- | Isolated actual HTML kernel compilation.\nmodule Main where\n  import Data.Bytes\n  import Data.Machine\n");
    for name in [
        "load-eight",
        "load-one",
        "special-eight",
        "extra-eight",
        "escape-span",
        "entity-name-hash",
        "pack-scalar",
    ] {
        let start = source.find(&format!("  prim {name} ::")).unwrap();
        let rest = &source[start..];
        let end = rest.find("\n  -- |").unwrap_or(rest.len());
        kernels.push_str(
            "\n  -- | Checked private HTML arithmetic or caller-proved exact source load.\n",
        );
        kernels.push_str(&rest[..end]);
    }
    kernels.push_str("\n  -- | Retain complete source storage until its exact private load finishes.\n  keep :: forall a. Bytes -> a -> a\n  keep bytes value = case bytes-len bytes of _ -> value\n  -- | Keep every isolated arithmetic and load kernel reachable without foreign execution.\n  main :: Int\n  main = let bytes = bytes-new 8 in let word = keep bytes (load-eight (bytes-data bytes)) in let byte = keep bytes (load-one (bytes-data bytes)) in let mask = special-eight word byte in let count = extra-eight word in let scalar = pack-scalar (wrapping-u64-to-u32 count) in let span = escape-span byte in let hash = entity-name-hash word scalar count (u8-to-u64 mask) (bit-or-u64 span 8) in if eq-u64 hash 0 then 0 else 1\n");
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
                    cinch::driver::check_sources_profile(&input, "html-kernels.cinch", &profile)?;
                cinch::codegen::emit_for(&profile, &program)
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("<8 x i8>"));
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
