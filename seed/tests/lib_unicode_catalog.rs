//! Exact named Unicode catalogs, code-point range tables and immutable observations.
mod common;

use cinch::driver::Mode;

/// Keep the actual kernel, generated tables and checked machine-word helpers beside the consumer.
fn run(source: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        ("Data.Unicode.Kernel", include_str!("../prelude/Data/Unicode/Kernel.cinch")),
        ("Data.Unicode", include_str!("../prelude/Data/Unicode.cinch")),
        ("Data.Unicode.Range", include_str!("../prelude/Data/Unicode/Range.cinch")),
        ("Data.Unicode.Names", include_str!("../prelude/Data/Unicode/Names.cinch")),
        ("Data.Unicode.Tables", include_str!("../prelude/Data/Unicode/Tables.cinch")),
        ("Data.Bytes.Word", include_str!("../prelude/Data/Bytes/Word.cinch")),
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
        Mode::Build, cinch::check::StageMode::Refresh, directory.path(),
        "Main.cinch", Some(executable.clone()), &profile,
    ).unwrap();
    for debug in [false, true] {
        let output = std::process::Command::new(&executable)
            .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_STACK_SIZE", "8M")
            .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
            .output().unwrap();
        assert_eq!(output.status.code(), Some(0), "debug={debug}: {}\n{}",
            String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
}

/// Numeric membership admits surrogate categories while Char construction remains scalar-only.
#[test]
fn unicode_named_tables_and_checked_custom_ranges() {
    run(include_str!("fixtures/unicode-catalog-consumer.cinch"));
}

/// Every facade, catalog selector, constructor and range implementation helper has canonical docs.
#[test]
fn unicode_catalog_declarations_have_canonical_documentation() {
    for (name, source) in [
        ("Data.Unicode", include_str!("../prelude/Data/Unicode.cinch")),
        ("Data.Unicode.Range", include_str!("../prelude/Data/Unicode/Range.cinch")),
        ("Data.Unicode.Names", include_str!("../prelude/Data/Unicode/Names.cinch")),
    ] {
        let parsed = cinch::cinchdoc::parse_source(source, Some(name), name, None, false).unwrap();
        assert!(parsed.doc.is_some());
        assert!(parsed.diagnostics.is_empty());
        assert!(parsed.orphans.is_empty());
        let mut declarations = Vec::new();
        cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
        for declaration in declarations {
            let doc = declaration.doc.as_ref().unwrap_or_else(|| panic!("missing docs: {}", declaration.id));
            assert!(!doc.legacy);
            let text = doc.markdown.to_lowercase();
            assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
        }
    }
}
