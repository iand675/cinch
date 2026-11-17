//! Canonical fixed-palette documentation and current Rust public runtime regression.

/// Keep generated sources, original defining interfaces and diagnostics on any assertion failure.
struct FixedPaletteProject {
    /// The complete temporary project remains owned until success or explicit panic retention.
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
    /// Retain all failed project bytes; clean only fully successful test output.
    fn drop(&mut self) {
        if std::thread::panicking() {
            let path = self.directory.take().unwrap().keep();
            eprintln!("retained failed fixed-palette project: {}", path.display());
        }
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
fn rust_cli_executes_fixed_palettes_with_normal_and_poison_profiles() {
    use std::path::Path;
    use std::process::Command;
    for selector in [
        "CINCH_TEST_COMPILER",
        "CINCH_TEST_INTERFACES",
        "CINCH_TEST_OBJECTS",
        "CINCH_TEST_RUNTIME_ARCHIVE",
    ] {
        assert!(std::env::var_os(selector).is_none(), "Rust-only fixed-palette regression refuses {selector}; Native qualification needs an explicit matched runner");
    }
    let compiler_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repository = compiler_root.parent().unwrap();
    let work = FixedPaletteProject::new();
    let runtime = compiler_root.join("runtime/target/release/libcinchruntime.a");
    let result = Command::new("python3")
        .arg(repository.join("tooling/check_color_palette_ci.py"))
        .arg("--compiler")
        .arg(env!("CARGO_BIN_EXE_cinch"))
        .arg("--runtime")
        .arg(&runtime)
        .arg("--work")
        .arg(work.path())
        .env("CARGO_BUILD_JOBS", "1")
        .env("RUST_TEST_THREADS", "1")
        .env("RUST_MIN_STACK", "67108864")
        .current_dir(repository)
        .output()
        .expect("execute current public Rust fixed-palette fixture");
    assert!(
        result.status.success(),
        "fixed-palette runtime failed:\n{}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
    let report: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(work.path().join("report.json")).unwrap())
            .unwrap();
    assert_eq!(report["status"], "passed");
    let initialized = &report["runtime_initialization"];
    let functional = &report["functional"];
    assert_eq!(initialized["archive_path"].as_str(), runtime.to_str());
    assert_eq!(
        initialized["archive_sha256"],
        initialized["snapshot_sha256"]
    );
    assert_eq!(functional["runtime_sha256"], initialized["archive_sha256"]);
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
