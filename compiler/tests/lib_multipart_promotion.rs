//! SDK-free gates for the isolated Region-carrier and both-effect candidate.
#[path = "common/corpus_frontend.rs"]
mod corpus_frontend;

/// Keep the complete generated project plain when a qualification assertion fails.
struct QualificationDirectory(Option<tempfile::TempDir>);

impl QualificationDirectory {
    /// Locate the owned project without transferring cleanup responsibility.
    fn path(&self) -> &std::path::Path {
        self.0.as_ref().unwrap().path()
    }
}

impl Drop for QualificationDirectory {
    /// Preserve sources, interfaces, diagnostics and generated artifacts during unwinding.
    fn drop(&mut self) {
        if std::thread::panicking() {
            if let Some(directory) = self.0.take() {
                eprintln!(
                    "retained failed multipart qualification: {}",
                    directory.keep().display()
                );
            }
        }
    }
}

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

/// The selected compiler checks genuine kinds and executes live Part/File/sink
/// callbacks, retained aliases, short writes and prefix errors under both heaps.
#[test]
fn selected_compiler_executes_isolated_multipart_factories_with_region_metadata() {
    let compiler_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let repository = compiler_root.parent().unwrap();
    let runtime = compiler_root.join("runtime/target/release/libcinchruntime.a");
    let work = QualificationDirectory(Some(tempfile::tempdir().unwrap()));
    let frontend = corpus_frontend::Frontend::selected(&runtime);
    let output = std::process::Command::new("python3")
        .arg(repository.join("tooling/qualify_multipart_promotion.py"))
        .arg("--work")
        .arg(work.path())
        .args(frontend.arguments())
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
    let live = match frontend {
        corpus_frontend::Frontend::Rust(_) => {
            assert_eq!(report["runtime_initialization"]["archive_path"].as_str(), runtime.to_str());
            assert_eq!(report["runtime_initialization"]["archive_sha256"], report["runtime_sha256"]);
            assert_eq!(report["runtime_initialization"]["snapshot_sha256"], report["runtime_sha256"]);
            read("source/runtime-rust.json")
        }
        corpus_frontend::Frontend::Native(_) => {
            frontend.check_identity(&report);
            report.clone()
        }
    };
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
