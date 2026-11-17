//! Immutable UUID values, complete independent observations and real serialized generation.
mod common;
use cinch::driver::Mode;

/// Compile actual candidate modules and execute normal and separately poisoned owner checks.
fn run(source: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (module, source) in [
        ("Data.Uuid", include_str!("../prelude/Data/Uuid.cinch")),
        ("Effect.Uuid", include_str!("../prelude/Effect/Uuid.cinch")),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
        (
            "Effect.Clock",
            include_str!("../prelude/Effect/Clock.cinch"),
        ),
        (
            "Effect.Random",
            include_str!("../prelude/Effect/Random.cinch"),
        ),
        (
            "Effect.Concurrent.MVar",
            include_str!("../prelude/Effect/Concurrent/MVar.cinch"),
        ),
    ] {
        std::fs::write(directory.path().join(format!("{module}.cinch")), source).unwrap();
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
    for poisoned in [false, true] {
        let output = std::process::Command::new(&executable)
            .env("CINCH_HEAP_LIMIT", if poisoned { "256M" } else { "32M" })
            .env("CINCH_STACK_SIZE", "8M")
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_RC_DEBUG", if poisoned { "1" } else { "0" })
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "poisoned={poisoned}: {}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr)
            .contains("cinch-leak: live=0 immortal=0 leaked=0"));
    }
}

/// Render committed public-operation vectors without requiring a reference SDK during CI.
#[test]
fn uuid_matches_all_independent_public_observations() {
    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/uuid-vectors.json")).unwrap();
    for (mode, expected) in [("parse", 660), ("append", 204), ("compare", 370), ("v7", 9)] {
        let count = fixture[mode].as_array().unwrap().len();
        assert_eq!(count, expected);
        for start in (0..count).step_by(128) {
            let directory = tempfile::tempdir().unwrap();
            let path = directory.path().join("Main.cinch");
            let status = std::process::Command::new("python3")
                .arg(repository.join("tooling/generate_uuid_cases.py"))
                .args([
                    "--mode",
                    mode,
                    "--start",
                    &start.to_string(),
                    "--count",
                    "128",
                    "--output",
                ])
                .arg(&path)
                .status()
                .unwrap();
            assert!(status.success());
            run(&std::fs::read_to_string(path).unwrap());
        }
    }
}

/// Immutable buffers, trusted extraction and explicit clock range/exhaustion preserve values on refusal.
#[test]
fn uuid_preserves_ownership_and_checked_boundaries() {
    run(include_str!("fixtures/uuid-control.cinch"));
}

/// Actual system random and wall-clock providers feed the explicitly owned serialized generator.
#[test]
fn uuid_uses_real_entropy_clock_and_shared_generator_state() {
    run(include_str!("fixtures/uuid-io.cinch"));
}

/// Every public/private declaration, constructor, primitive and instance has canonical documentation.
#[test]
fn uuid_declarations_have_canonical_documentation() {
    for (module, source) in [
        ("Data.Uuid", include_str!("../prelude/Data/Uuid.cinch")),
        ("Effect.Uuid", include_str!("../prelude/Effect/Uuid.cinch")),
    ] {
        let parsed =
            cinch::cinchdoc::parse_source(source, Some(module), module, None, false).unwrap();
        assert!(parsed.doc.is_some() && parsed.diagnostics.is_empty() && parsed.orphans.is_empty());
        let mut declarations = Vec::new();
        cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
        for declaration in declarations {
            let doc = declaration
                .doc
                .as_ref()
                .unwrap_or_else(|| panic!("{}", declaration.id));
            assert!(!doc.legacy);
            let text = doc.markdown.to_lowercase();
            assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
        }
    }
}
