//! Canonical documentation and real public-CLI multipart runtime regressions.

/// Every public/private scanner, session, backend adapter and constructor has canonical prose.
#[test]
fn multipart_declarations_have_canonical_documentation() {
    for (module, source) in [
        (
            "Text.Multipart",
            include_str!("../prelude/Text/Multipart.cinch"),
        ),
        (
            "Effect.Multipart",
            include_str!("../prelude/Effect/Multipart.cinch"),
        ),
    ] {
        let parsed =
            cinch::cinchdoc::parse_source(source, Some(module), module, None, false).unwrap();
        assert!(parsed.doc.is_some(), "{module}");
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
}

/// Current Rust-only CLI consumers execute committed values, exact stream traces
/// and mutation controls with the real foreign object under both heap profiles.
/// An ordinary compiler build refreshes and captures the exact host runtime.
#[test]
fn rust_cli_executes_multipart_with_normal_and_poison_profiles() {
    use std::path::Path;
    use std::process::Command;

    for selector in [
        "CINCH_TEST_COMPILER",
        "CINCH_TEST_INTERFACES",
        "CINCH_TEST_OBJECTS",
        "CINCH_TEST_RUNTIME_ARCHIVE",
    ] {
        assert!(std::env::var_os(selector).is_none(), "Rust-only multipart regression refuses {selector}; use the explicit standalone Native runner");
    }
    let compiler_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repository = compiler_root.parent().unwrap();
    let runtime = compiler_root.join("runtime/target/release/libcinchruntime.a");
    let work = tempfile::tempdir().unwrap();
    let result = Command::new("python3")
        .arg(repository.join("tooling/check_multipart.py"))
        .arg("--work")
        .arg(work.path())
        .arg("--rust")
        .arg(env!("CARGO_BIN_EXE_cinch"))
        .arg("--rust-runtime")
        .arg(&runtime)
        .arg("--initialize-rust-runtime")
        .args([
            "--group-size",
            "32",
            "--form-group-size",
            "32",
            "--only",
            "dispositions-0288",
            "readers-0000",
            "late-headers-0000",
            "writers-0000",
            "forms-0000",
            "ownership-0000",
            "entropy-0000",
        ])
        .env("CARGO_BUILD_JOBS", "1")
        .env("RUST_TEST_THREADS", "1")
        .env("RUST_MIN_STACK", "67108864")
        .current_dir(repository)
        .output()
        .expect("execute committed multipart corpus through the current Rust CLI");
    assert!(
        result.status.success(),
        "Rust multipart regression failed:\n{}\n{}",
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
    assert_eq!(rows.len(), 7);
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
