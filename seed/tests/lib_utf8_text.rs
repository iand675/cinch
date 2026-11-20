//! Immutable-text UTF-8 observations retain exact scalar metadata and sealed backing ownership.
mod common;

use cinch::driver::Mode;

/// Compile the real facade and its byte dependencies against the current embedded String family.
fn run(source: &str, groups: &[&str]) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("Text.Utf8", include_str!("../prelude/Text/Utf8.cinch")),
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
    let mut runs = vec![(false, None)];
    if groups.is_empty() {
        runs.push((true, None));
    } else {
        runs.extend(groups.iter().map(|name| (true, Some(*name))));
    }
    for (debug, group) in runs {
        let mut command = std::process::Command::new(&executable);
        if let Some(group) = group {
            command.arg(group);
        }
        let output = command
            .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_STACK_SIZE", "8M")
            .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "debug={debug}, group={group:?}: {}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
}

/// Empty text, all width boundaries, numeric replacement and mutable-copy isolation agree.
#[test]
fn utf8_text_scalar_boundaries_and_copy_isolation() {
    run(include_str!("fixtures/utf8-text-controls.cinch"), &[]);
}

/// Checked-in independent observations run without an SDK, script interpreter or reference runtime.
#[test]
fn utf8_text_matches_804_independent_observations_without_sdk() {
    let source = include_str!("fixtures/utf8-text-consumer.cinch");
    let groups: Vec<_> = source
        .lines()
        .filter_map(|line| {
            let name = line.strip_prefix("  ")?.strip_suffix(" :: Unit -> Bool")?;
            (name.starts_with("rows-") || name.starts_with("long-")).then_some(name)
        })
        .collect();
    assert_eq!(groups.len(), 99);
    assert_eq!(
        source
            .lines()
            .filter(|line| line.contains(" unit = let text = string-repeat "))
            .count(),
        52
    );
    run(source, &groups);
}

/// Every new facade declaration has a canonical comment independent of any reference implementation.
#[test]
fn utf8_text_declarations_have_canonical_documentation() {
    let source = include_str!("../prelude/Text/Utf8.cinch");
    let name = "Text.Utf8";
    let parsed = cinch::cinchdoc::parse_source(source, Some(name), name, None, false).unwrap();
    assert!(parsed.doc.is_some());
    assert!(parsed.diagnostics.is_empty());
    assert!(parsed.orphans.is_empty());
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    assert_eq!(declarations.len(), 6);
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
