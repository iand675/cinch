//! Documentation and independent fixture coverage for explicit comparison policies.

/// Public and private declarations must have canonical Cinch documentation.
#[test]
fn comparison_declarations_have_canonical_documentation() {
    let parsed = cinch::cinchdoc::parse_source(
        include_str!("../prelude/Data/Compare.cinch"),
        Some("Data.Compare"),
        "Data.Compare",
        None,
        false,
    )
    .unwrap();
    assert!(parsed.doc.is_some());
    assert!(parsed.diagnostics.is_empty(), "{:?}", parsed.diagnostics);
    assert!(parsed.orphans.is_empty(), "{:?}", parsed.orphans);
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    for declaration in declarations {
        let doc = declaration
            .doc
            .as_ref()
            .unwrap_or_else(|| panic!("{}: missing canonical documentation", declaration.id));
        assert!(!doc.legacy, "{}", declaration.id);
        let prose = doc.markdown.to_lowercase();
        assert!(!prose.contains("golang") && !prose.contains("go's") && !prose.contains("go "));
    }
}

/// The frozen corpus covers every operand pair, including NaN payloads and zero signs.
#[test]
fn comparison_fixture_corpus_is_complete() {
    let fixtures: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/compare-vectors.json")).unwrap();
    for field in ["f32", "f64"] {
        let rows = fixtures[field].as_array().unwrap();
        assert_eq!(rows.len(), 19 * 19, "{field}");
        assert!(rows
            .iter()
            .any(|row| row["compare"] == -1 && row["less"] == true));
        assert!(rows
            .iter()
            .any(|row| row["compare"] == 0 && row["less"] == false));
        assert!(rows
            .iter()
            .any(|row| row["compare"] == 1 && row["less"] == false));
    }
    assert_eq!(fixtures["integers"].as_array().unwrap().len(), 12 * 25);
    assert_eq!(fixtures["strings"].as_array().unwrap().len(), 10 * 10);
}

/// Rust-only CLI execution of committed scalar, NaN, string and default fixtures.
/// External frontend selectors are refused explicitly: this test cannot stand
/// in for Native coverage. The normal build initializes the current compiler's
/// exact host runtime; the Python runner gives compiler processes 64 MiB host
/// stacks and consumers separate 8 MiB stacks in normal and poisoned profiles.
#[test]
fn rust_cli_executes_comparison_fixtures_with_normal_and_poison_profiles() {
    use std::path::Path;
    use std::process::Command;

    for selector in [
        "CINCH_TEST_COMPILER",
        "CINCH_TEST_INTERFACES",
        "CINCH_TEST_OBJECTS",
        "CINCH_TEST_RUNTIME_ARCHIVE",
    ] {
        assert!(
            std::env::var_os(selector).is_none(),
            "Rust-only comparison CLI regression refuses external frontend selector {selector}; run the standalone Native corpus instead"
        );
    }
    let compiler_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repository = compiler_root.parent().unwrap();
    let runtime = compiler_root.join("runtime/target/release/libcinchruntime.a");
    let work = tempfile::tempdir().unwrap();
    let result = Command::new("python3")
        .arg(repository.join("tooling/check_compare.py"))
        .arg("--work")
        .arg(work.path())
        .arg("--rust")
        .arg(env!("CARGO_BIN_EXE_cinch"))
        .arg("--rust-runtime")
        .arg(&runtime)
        .arg("--initialize-rust-runtime")
        .args(["--only", "I8", "F32", "Default", "String"])
        .env("CARGO_BUILD_JOBS", "1")
        .env("RUST_TEST_THREADS", "1")
        .env("RUST_MIN_STACK", "67108864")
        .current_dir(repository)
        .output()
        .expect("run committed comparison corpus through current Rust CLI");
    assert!(
        result.status.success(),
        "Rust comparison runtime regression failed:\n{}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    let report: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(work.path().join("report.json")).unwrap())
            .unwrap();
    assert_eq!(report["compiler_host_stack_bytes"], 64 * 1024 * 1024);
    assert_eq!(report["frontends"].as_object().unwrap().len(), 1);
    assert_eq!(
        report["runtime_initialization"]["archive_path"].as_str(),
        runtime.to_str()
    );
    assert_eq!(
        report["frontends"]["rust"]["runtime_sha256"],
        report["runtime_initialization"]["archive_sha256"]
    );
    assert_eq!(
        report["runtime_initialization"]["snapshot_sha256"],
        report["runtime_initialization"]["archive_sha256"]
    );
    let rows = report["runs"].as_array().unwrap();
    assert_eq!(rows.len(), 4);
    let groups: std::collections::BTreeSet<_> = rows
        .iter()
        .map(|row| row["case"].as_str().unwrap())
        .collect();
    assert_eq!(
        groups,
        ["Default", "F32", "I8", "String"].into_iter().collect()
    );
    for row in rows {
        assert_eq!(row["frontend"], "rust");
        let profiles = row["profiles"].as_array().unwrap();
        assert_eq!(profiles.len(), 2);
        for (profile, (poison, heap)) in profiles.iter().zip([("0", "32M"), ("1", "256M")]) {
            assert_eq!(profile["poison"], poison);
            assert_eq!(profile["heap"], heap);
            assert_eq!(profile["stack"], "8M");
            assert!(profile["stderr"].as_str().unwrap().contains("leaked=0"));
        }
    }
}
