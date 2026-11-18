//! Configurable byte Base64: independent wire/error fixtures, alias isolation and stream continuations.
mod common;

use cinch::driver::Mode;
use std::path::PathBuf;

/// Project a fixed offline fixture group without requiring a reference SDK during tests.
fn case_source(name: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let output = std::process::Command::new("python3")
        .arg(root.join("tooling/validate_base64_encoding.py"))
        .args(["--emit", name])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

/// Compile the selected frontend explicitly and verify normal reclamation separately from poison quarantine.
fn run(name: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (module, source) in [
        (
            "Text.Base64.Encoding",
            include_str!("../prelude/Text/Base64/Encoding.cinch"),
        ),
        (
            "Effect.Base64",
            include_str!("../prelude/Effect/Base64.cinch"),
        ),
        (
            "Effect.Stream",
            include_str!("../prelude/Effect/Stream.cinch"),
        ),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
    ] {
        std::fs::write(directory.path().join(format!("{module}.cinch")), source).unwrap();
    }
    std::fs::write(directory.path().join("Main.cinch"), case_source(name)).unwrap();
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
            .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
            .env("CINCH_STACK_SIZE", "8M")
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "{name} debug={debug}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
}

/// Fixed standard/URL and raw configurations preserve all independent wires, length bounds and feed splits.
#[test]
fn base64_fixed_configurations_match_independent_wire_and_split_cases() {
    for index in 0..5 {
        run(&format!("valid-{index}"));
    }
}

/// Custom alphabets, including high bytes and NUL/255 padding, preserve every byte and independent result.
#[test]
fn base64_custom_configurations_match_independent_wire_and_split_cases() {
    for index in 5..9 {
        run(&format!("valid-{index}"));
    }
}

/// Malformed inputs retain exactly the independently recorded complete output prefix before failing.
#[test]
fn base64_malformed_inputs_preserve_independent_progress() {
    for index in 0..9 {
        run(&format!("malformed-{index}"));
    }
}

/// Strict unused-bit checks and constructor errors have precise module-owned diagnostics.
#[test]
fn base64_strict_bits_and_configuration_failures_are_precise() {
    run("strict");
}

/// Caller-buffer writes, shrinking overlap, returned tables and retained input never alias unexpectedly.
#[test]
fn base64_buffers_and_retained_states_are_independent() {
    run("buffers");
}

/// Odd short writes, sticky failures, delayed decoder errors and explicit finishing preserve continuations.
#[test]
fn base64_streams_finish_and_preserve_advanced_handles() {
    run("streams");
}

/// A 64 KiB input survives one-byte encoded source chunks under ordinary bounded memory.
#[test]
fn base64_one_byte_source_is_bounded_and_releases_storage() {
    run("long-stream");
}

/// New public and private declarations carry canonical module-owned documentation.
#[test]
fn base64_encoding_declarations_have_canonical_documentation() {
    for (name, source) in [
        (
            "Text.Base64.Encoding",
            include_str!("../prelude/Text/Base64/Encoding.cinch"),
        ),
        (
            "Effect.Base64",
            include_str!("../prelude/Effect/Base64.cinch"),
        ),
    ] {
        let parsed = cinch::cinchdoc::parse_source(source, Some(name), name, None, false).unwrap();
        assert!(parsed.doc.is_some());
        assert!(
            parsed.diagnostics.is_empty(),
            "{name}: {:?}",
            parsed.diagnostics
        );
        assert!(parsed.orphans.is_empty(), "{name}: {:?}", parsed.orphans);
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
}

/// The actual SIMD and exact-load kernels compile on 32-bit and big-endian non-x86 targets.
/// This explicitly exercises Rust's target emitter and does not claim foreign execution.
#[test]
fn base64_encoding_kernels_compile_for_wasm_arm_and_big_endian_targets() {
    let directory = tempfile::tempdir().unwrap();
    let source = include_str!("../prelude/Text/Base64/Encoding.cinch");
    let marker = "  -- Generated pure Base64 SIMD kernels";
    let mut kernel = String::from("-- | Reachable private Base64 kernels for object-target validation.\nmodule Main where\n  import Data.Machine\n  import Data.Bytes\n");
    kernel.push_str(&source[source.find(marker).unwrap()..]);
    kernel.push_str("\n  -- | Retain the table while its exact private loads complete.\n  keep :: Bytes -> U64 -> U64\n  keep bytes value = case bytes-len bytes of _ -> value\n  -- | Observe every primitive result so foreign object lowering cannot discard the kernels.\n  main :: Int\n  main = let table = bytes-new 256 in let mapped = map-fixed (encode-values 1108152157446) 1 in let decoded = decode-values (classify-fixed mapped 1) in let gathered = keep table (lookup-eight (bytes-data table) 0) in if eq-u64 decoded gathered then 0 else 1\n");
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
        let text = kernel.clone();
        let ir = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let program =
                    cinch::driver::check_sources_profile(&text, "base64-kernels.cinch", &profile)?;
                cinch::codegen::emit_for(&profile, &program)
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("<8 x i16>"));
        let path = directory.path().join(format!("{arch}.ll"));
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
