//! Complete owned PEM framing, byte metadata, exact recovery and stream continuations without an SDK.
mod common;
use cinch::driver::Mode;

/// Compile actual source modules and keep normal live-space separate from poisoned quarantine groups.
fn run(source: &str, groups: &[Vec<String>]) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("Text.Pem", include_str!("../prelude/Text/Pem.cinch")),
        ("Effect.Pem", include_str!("../prelude/Effect/Pem.cinch")),
        (
            "Effect.Stream",
            include_str!("../prelude/Effect/Stream.cinch"),
        ),
        (
            "Text.Base64.Encoding",
            include_str!("../prelude/Text/Base64/Encoding.cinch"),
        ),
        (
            "Data.Bytes.Utf8",
            include_str!("../prelude/Data/Bytes/Utf8.cinch"),
        ),
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
    let mut runs = vec![(false, Vec::new())];
    if groups.is_empty() {
        runs.push((true, Vec::new()));
    } else {
        runs.extend(groups.iter().cloned().map(|group| (true, group)));
    }
    for (poisoned, group) in runs {
        let output = std::process::Command::new(&executable)
            .args(&group)
            .env("CINCH_RC_DEBUG", if poisoned { "1" } else { "0" })
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_HEAP_LIMIT", if poisoned { "256M" } else { "32M" })
            .env("CINCH_STACK_SIZE", "8M")
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "poisoned={poisoned}, group={group:?}: {}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
}

/// Checked-in independent bytes verify every field and remainder without a reference toolchain.
#[test]
fn pem_matches_all_independent_encoder_decoder_and_remainder_observations() {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/pem-vectors.json")).unwrap();
    let mut groups = Vec::new();
    for (mode, name, expected) in [(0, "encodes", 473), (1, "decodes", 1716)] {
        let count = fixture[name].as_array().unwrap().len();
        assert_eq!(count, expected);
        groups
            .extend((0..count.div_ceil(8)).map(|index| vec![mode.to_string(), index.to_string()]));
    }
    run(include_str!("fixtures/pem-consumer.cinch"), &groups);
}

/// Snapshot mutation, persistent edits, large metadata and short/failing writers preserve advanced state.
#[test]
fn pem_ownership_persistent_headers_and_writer_continuations() {
    run(include_str!("fixtures/pem-controls.cinch"), &[]);
}

/// Actual marker/vector code checks every lane and byte, including high-bit wanted-byte arguments.
#[test]
fn pem_actual_readonly_kernels_match_exhaustive_lane_mutations() {
    run(include_str!("fixtures/pem-kernels.cinch"), &[]);
}

/// Every declaration and constructor has a canonical comment independent of a reference language.
#[test]
fn pem_declarations_have_canonical_documentation() {
    for (name, source) in [
        ("Text.Pem", include_str!("../prelude/Text/Pem.cinch")),
        ("Effect.Pem", include_str!("../prelude/Effect/Pem.cinch")),
    ] {
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
}

/// Actual ten primitive bodies and scanner callers compile for wasm, ARM and big-endian PowerPC.
#[test]
fn pem_actual_readonly_kernels_compile_for_portable_targets() {
    let source = include_str!("fixtures/pem-kernels-foreign.cinch");
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
        let input = source.to_owned();
        let ir = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let program =
                    cinch::driver::check_sources_profile(&input, "pem-kernels.cinch", &profile)?;
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
