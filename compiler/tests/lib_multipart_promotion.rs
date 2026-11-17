//! SDK-free gates for the isolated Region-carrier and both-effect candidate.

/// The review candidate keeps canonical documentation on every declaration.
#[test]
fn isolated_multipart_candidate_has_canonical_documentation() {
    for (module, source) in [
        (
            "Effect.Multipart",
            include_str!("../../experiments/multipart-promotion-v1/sources/Effect.Multipart.cinch"),
        ),
        (
            "Text.Multipart",
            include_str!("../../experiments/multipart-promotion-v1/sources/Text.Multipart.cinch"),
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
                .unwrap_or_else(|| panic!("missing documentation: {}", declaration.id));
            assert!(!doc.legacy, "{}", declaration.id);
            let text = doc.markdown.to_lowercase();
            assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
        }
    }
}

/// The current Rust CLI checks genuine kinds and executes live Part/File/sink
/// callbacks, retained aliases, short writes and prefix errors under both heaps.
#[test]
fn rust_cli_executes_isolated_multipart_factories_with_region_metadata() {
    for selector in [
        "CINCH_TEST_COMPILER",
        "CINCH_TEST_INTERFACES",
        "CINCH_TEST_OBJECTS",
        "CINCH_TEST_RUNTIME_ARCHIVE",
    ] {
        assert!(std::env::var_os(selector).is_none(), "Rust-only candidate gate refuses {selector}; standalone historical Native gates remain separate");
    }
    let compiler_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let repository = compiler_root.parent().unwrap();
    let runtime = compiler_root.join("runtime/target/release/libcinchruntime.a");
    let work = tempfile::tempdir().unwrap();
    let output = std::process::Command::new("python3")
        .arg(repository.join("tooling/qualify_multipart_promotion.py"))
        .arg("--work")
        .arg(work.path())
        .arg("--compiler")
        .arg(env!("CARGO_BIN_EXE_cinch"))
        .arg("--runtime")
        .arg(&runtime)
        .arg("--initialize-rust-runtime")
        .env("CARGO_BUILD_JOBS", "1")
        .env("RUST_TEST_THREADS", "1")
        .env("RUST_MIN_STACK", "67108864")
        .current_dir(repository)
        .output()
        .expect("run maintained isolated multipart pipeline");
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let read = |name: &str| -> serde_json::Value {
        serde_json::from_slice(&std::fs::read(work.path().join(name)).unwrap()).unwrap()
    };
    let report = read("report.json");
    assert_eq!(report["compiler_host_stack_bytes"], 64 * 1024 * 1024);
    assert_eq!(
        report["runtime_initialization"]["archive_path"].as_str(),
        runtime.to_str()
    );
    assert_eq!(
        report["runtime_initialization"]["archive_sha256"],
        report["runtime_sha256"]
    );
    assert_eq!(
        report["runtime_initialization"]["snapshot_sha256"],
        report["runtime_sha256"]
    );
    assert_eq!(report["phases"].as_array().unwrap().len(), 3);
    let metadata = read("metadata/report.json");
    for (key, count) in [("public_contracts", 12), ("private_contracts", 7)] {
        let contracts = metadata[key].as_array().unwrap();
        assert_eq!(contracts.len(), count);
        for contract in contracts {
            assert_eq!(contract["parameters"][0]["kind"], "Region");
            assert!(!contract["constructors"].as_array().unwrap().is_empty());
        }
    }
    let live = read("source/runtime-rust.json");
    assert_eq!(live["runtime_sha256"], report["runtime_sha256"]);
    assert_eq!(live["compiler_sha256"], report["compiler_sha256"]);
    assert_eq!(live["runs"].as_array().unwrap().len(), 1);
    let profiles = live["runs"][0]["profiles"].as_array().unwrap();
    assert_eq!(profiles.len(), 2);
    for (profile, (poison, heap)) in profiles.iter().zip([("0", "32M"), ("1", "256M")]) {
        assert_eq!(profile["poison"], poison);
        assert_eq!(profile["heap"], heap);
        assert_eq!(profile["stack"], "8M");
        assert!(profile["stderr"].as_str().unwrap().contains("leaked=0"));
    }
}
