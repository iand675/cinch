//! Canonical documentation and real public-CLI MIME runtime regressions.

#[path = "common/corpus_frontend.rs"]
mod corpus_frontend;

/// Every public/private codec, adapter and constructor has canonical prose.
#[test]
fn mime_declarations_have_canonical_documentation() {
    for (module, source) in [
        ("Text.Mime", include_str!("../prelude/Text/Mime.cinch")),
        ("Effect.Mime", include_str!("../prelude/Effect/Mime.cinch")),
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

/// Execute the committed fixtures through the selected public CLI under both heap profiles.
#[test]
fn selected_cli_executes_mime_with_normal_and_poison_profiles() {
    use std::path::Path;
    use std::process::Command;

    let compiler_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repository = compiler_root.parent().unwrap();
    let runtime = compiler_root.join("runtime/target/release/libcinchruntime.a");
    let work = tempfile::tempdir().unwrap();
    let frontend = corpus_frontend::Frontend::selected(&runtime);
    let result = Command::new("python3")
        .arg(repository.join("tooling/check_mime.py"))
        .arg("--work")
        .arg(work.path())
        .args(frontend.arguments())
        .args([
            "--only",
            "media-0000",
            "formats-0000",
            "encoded-0000",
            "decoded-callbacks",
            "writes-0000",
            "registry",
            "ownership",
        ])
        .env("CARGO_BUILD_JOBS", "1")
        .env("RUST_TEST_THREADS", "1")
        .env("RUST_MIN_STACK", "67108864")
        .current_dir(repository)
        .output()
        .expect("execute committed codec corpus through the selected CLI");
    assert!(
        result.status.success(),
        "MIME regression failed:\n{}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    let report: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(work.path().join("report.json")).unwrap())
            .unwrap();
    assert_eq!(report["compiler_host_stack_bytes"], 64 * 1024 * 1024);
    frontend.check_identity(&report);
    let rows = report["runs"].as_array().unwrap();
    assert_eq!(rows.len(), 7);
    for row in rows {
        assert_eq!(row["frontend"], frontend.label());
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
