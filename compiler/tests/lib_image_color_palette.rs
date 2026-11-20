//! Canonical fixed-palette documentation and selected public compiler runtime regression.
#[path = "common/corpus_frontend.rs"]
mod corpus_frontend;

/// Preserve generated sources, original defining interfaces and diagnostics for every outcome.
struct FixedPaletteProject {
    /// The complete project remains owned until its path is retained at the test boundary.
    directory: Option<tempfile::TempDir>,
}

impl FixedPaletteProject {
    /// Create one independent current-compiler project without using a frozen producer cache.
    fn new() -> Self {
        Self {
            directory: Some(
                tempfile::Builder::new()
                    .prefix("cinch-fixed-palette-")
                    .tempdir()
                    .unwrap(),
            ),
        }
    }

    /// Observe the live project path without releasing its cleanup or failure guard.
    fn path(&self) -> &std::path::Path {
        self.directory.as_ref().unwrap().path()
    }
}

impl Drop for FixedPaletteProject {
    /// Keep all source and proof inputs plain; successful outputs can be archived after review.
    fn drop(&mut self) {
        let path = self.directory.take().unwrap().keep();
        let outcome = if std::thread::panicking() {
            "failed"
        } else {
            "completed"
        };
        eprintln!(
            "retained {outcome} fixed-palette project: {}",
            path.display()
        );
    }
}

/// Both public factories and every private fixed-table helper have canonical prose.
#[test]
fn fixed_palette_declarations_have_canonical_documentation() {
    let source =
        include_str!("../../experiments/image-color-palette-v1/sources/Image.Color.Palette.cinch");
    let parsed = cinch::cinchdoc::parse_source(
        source,
        Some("Image.Color.Palette"),
        "Image.Color.Palette",
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

/// Execute all472 entries/1024 independent palette queries and retained factory aliases with both heaps.
#[test]
fn selected_compiler_executes_fixed_palettes_with_normal_and_poison_profiles() {
    use std::path::Path;
    use std::process::Command;
    let compiler_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repository = compiler_root.parent().unwrap();
    let work = FixedPaletteProject::new();
    let runtime = compiler_root.join("runtime/target/release/libcinchruntime.a");
    let frontend = corpus_frontend::Frontend::selected(&runtime);
    let mut command = Command::new("python3");
    let report_path = match &frontend {
        corpus_frontend::Frontend::Rust(_) => {
            command.arg(repository.join("tooling/check_color_palette_ci.py"))
                .arg("--compiler").arg(env!("CARGO_BIN_EXE_cinch"))
                .arg("--runtime").arg(&runtime)
                .arg("--work").arg(work.path());
            work.path().join("report.json")
        }
        corpus_frontend::Frontend::Native(directory) => {
            let output = work.path().join("functional");
            command.arg(repository.join("tooling/check_color.py"))
                .arg("--native").arg(directory)
                .arg("--source").arg(repository.join("experiments/image-color-v1/sources/Image.Color.cinch"))
                .arg("--palette").arg(repository.join("experiments/image-color-palette-v1/sources/Image.Color.Palette.cinch"))
                .arg("--work").arg(&output);
            output.join("report.json")
        }
    };
    let result = command
        .env("CARGO_BUILD_JOBS", "1")
        .env("RUST_TEST_THREADS", "1")
        .env("RUST_MIN_STACK", "67108864")
        .current_dir(repository)
        .output()
        .expect("execute selected public fixed-palette fixture");
    assert!(
        result.status.success(),
        "fixed-palette runtime failed:\n{}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    let report: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(report_path).unwrap())
            .unwrap();
    assert_eq!(report["status"], "passed");
    let functional = match frontend {
        corpus_frontend::Frontend::Rust(_) => {
            let initialized = &report["runtime_initialization"];
            assert_eq!(initialized["archive_path"].as_str(), runtime.to_str());
            assert_eq!(initialized["archive_sha256"], initialized["snapshot_sha256"]);
            assert_eq!(report["functional"]["runtime_sha256"], initialized["archive_sha256"]);
            &report["functional"]
        }
        corpus_frontend::Frontend::Native(_) => {
            frontend.check_identity(&report);
            assert!(report["defining_interfaces"]["Image.Color.Palette"].is_string());
            &report
        }
    };
    assert_eq!(functional["status"], "passed");
    assert_eq!(functional["compiler_host_stack_bytes"], 64 * 1024 * 1024);
    assert_eq!(functional["foreign_fixture"]["exact_table_entries"], 472);
    assert_eq!(functional["foreign_fixture"]["palette_use_queries"], 1024);
    let profiles = functional["profiles"].as_array().unwrap();
    assert_eq!(profiles.len(), 2);
    for (profile, (poison, heap)) in profiles.iter().zip([("0", "32M"), ("1", "256M")]) {
        assert_eq!(profile["poison"], poison);
        assert_eq!(profile["heap"], heap);
        assert_eq!(profile["stack"], "8M");
        assert!(profile["stderr"].as_str().unwrap().contains("leaked=0"));
    }
}
