//! Immutable UUID values, complete independent observations and real serialized generation.
mod common;
use cinch::driver::Mode;
use sha2::{Digest, Sha256};

/// Retain every generated source and compiler artifact when qualification unwinds.
struct QualificationDirectory(Option<tempfile::TempDir>);

impl QualificationDirectory {
    /// Create one owned project that cleans up only after a successful qualification.
    fn new() -> Self {
        Self(Some(tempfile::tempdir().unwrap()))
    }

    /// Locate the project without transferring its cleanup responsibility.
    fn path(&self) -> &std::path::Path {
        self.0.as_ref().unwrap().path()
    }
}

impl Drop for QualificationDirectory {
    /// Preserve failing compiler, process and portable-object projects during unwinding.
    fn drop(&mut self) {
        if std::thread::panicking() {
            if let Some(directory) = self.0.take() {
                eprintln!(
                    "retained failed UUID qualification: {}",
                    directory.keep().display()
                );
            }
        }
    }
}

/// Select one explicitly requested source file or the byte-identical embedded baseline, recording exact bytes and provenance.
fn uuid_data_source(directory: &QualificationDirectory) -> String {
    let requested = std::env::var_os("CINCH_TEST_UUID_DATA_SOURCE").map(std::path::PathBuf::from);
    let (source, selected_path, mode) = match requested.as_ref() {
        Some(path) => {
            let selected = path.canonicalize().unwrap_or_else(|error| {
                let _ = std::fs::write(
                    directory.path().join("source-selection.error"),
                    format!("{}: {error}", path.display()),
                );
                panic!(
                    "explicit UUID source path {} could not be resolved: {error}",
                    path.display()
                );
            });
            let source = std::fs::read_to_string(&selected).unwrap_or_else(|error| {
                let _ = std::fs::write(
                    directory.path().join("source-selection.error"),
                    format!("{}: {error}", selected.display()),
                );
                panic!(
                    "explicit UUID source {} could not be read: {error}",
                    selected.display()
                );
            });
            (source, selected, "explicit-file")
        }
        None => (
            include_str!("../../experiments/uuid-candidate-v1/sources/Data.Uuid.cinch").to_owned(),
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("experiments/uuid-candidate-v1/sources/Data.Uuid.cinch"),
            "embedded-baseline",
        ),
    };
    let sha256 = format!("{:x}", Sha256::digest(source.as_bytes()));
    eprintln!(
        "UUID data source mode={mode} path={} sha256={sha256}",
        selected_path.display()
    );
    let selection = serde_json::json!({
        "module": "Data.Uuid", "mode": mode, "requested_path": requested,
        "selected_path": selected_path, "actual_source_sha256": sha256,
        "silent_fallback": false,
    });
    std::fs::write(directory.path().join("Data.Uuid.cinch"), source.as_bytes()).unwrap();
    std::fs::write(
        directory.path().join("uuid-source-selection.json"),
        serde_json::to_vec_pretty(&selection).unwrap(),
    )
    .unwrap();
    source
}

/// Compile actual candidate modules and execute normal and separately poisoned owner checks.
fn run(source: &str) {
    let directory = QualificationDirectory::new();
    let data_source = uuid_data_source(&directory);
    for (module, source) in [
        ("Data.Uuid", data_source.as_str()),
        (
            "Effect.Uuid",
            include_str!("../../experiments/uuid-candidate-v1/sources/Effect.Uuid.cinch"),
        ),
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
    .unwrap_or_else(|error| {
        let _ = std::fs::write(
            directory.path().join("compiler.error"),
            format!("{error:?}"),
        );
        panic!("UUID compilation failed: {error:?}");
    });
    for poisoned in [false, true] {
        let output = std::process::Command::new(&executable)
            .env("CINCH_HEAP_LIMIT", if poisoned { "256M" } else { "32M" })
            .env("CINCH_STACK_SIZE", "8M")
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_RC_DEBUG", if poisoned { "1" } else { "0" })
            .output()
            .unwrap();
        let label = if poisoned { "poison" } else { "normal" };
        std::fs::write(
            directory.path().join(format!("{label}.stdout")),
            &output.stdout,
        )
        .unwrap();
        std::fs::write(
            directory.path().join(format!("{label}.stderr")),
            &output.stderr,
        )
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
    for (mode, expected) in [
        ("parse", 660),
        ("append", 204),
        ("compare", 370),
        ("v7", 9),
        ("generated", 48),
    ] {
        let count = fixture[mode].as_array().unwrap().len();
        assert_eq!(count, expected);
        for start in (0..count).step_by(128) {
            let directory = QualificationDirectory::new();
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
                .output()
                .unwrap();
            std::fs::write(directory.path().join("generator.stdout"), &status.stdout).unwrap();
            std::fs::write(directory.path().join("generator.stderr"), &status.stderr).unwrap();
            assert!(
                status.status.success(),
                "{}",
                String::from_utf8_lossy(&status.stderr)
            );
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
    let directory = QualificationDirectory::new();
    let data_source = uuid_data_source(&directory);
    for (module, source) in [
        ("Data.Uuid", data_source.as_str()),
        (
            "Effect.Uuid",
            include_str!("../../experiments/uuid-candidate-v1/sources/Effect.Uuid.cinch"),
        ),
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

/// Actual complete immutable adapters retain endian/vector/packed bodies and compile for32/64-bit and big-endian targets.
#[test]
fn uuid_full_adapters_compile_for_portable_targets() {
    for (triple, arch, endian) in [
        ("wasm32-unknown-unknown", "wasm32", "little"),
        ("aarch64-unknown-linux-gnu", "aarch64", "little"),
        ("powerpc64-unknown-linux-gnu", "powerpc64", "big"),
    ] {
        let directory = QualificationDirectory::new();
        let data_source = uuid_data_source(&directory);
        for (module, source) in [
            ("Data.Uuid", data_source.as_str()),
            ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
            (
                "Data.Bytes.Word",
                include_str!("../prelude/Data/Bytes/Word.cinch"),
            ),
        ] {
            std::fs::write(directory.path().join(format!("{module}.cinch")), source).unwrap();
        }
        std::fs::write(
            directory.path().join("Main.cinch"),
            include_str!("fixtures/uuid-portable.cinch"),
        )
        .unwrap();
        let profile = cinch::driver::target_profile(&cinch::driver::DriverOptions {
            target: Some(triple.into()),
            ..Default::default()
        })
        .unwrap_or_else(|_| {
            assert_eq!(arch, "powerpc64");
            let mut profile = cinch::core::TargetProfile::native();
            profile.triple = triple.into();
            profile.arch = arch.into();
            profile.endian = endian.into();
            profile.os = "linux".into();
            profile.word_size = 64;
            profile
        });
        let ir_path = directory.path().join("consumer.ll");
        common::drive_with_profile(
            Mode::EmitIr,
            cinch::check::StageMode::Refresh,
            directory.path(),
            "Main.cinch",
            Some(ir_path.clone()),
            &profile,
        )
        .unwrap_or_else(|error| {
            let _ = std::fs::write(
                directory.path().join("compiler.error"),
                format!("{error:?}"),
            );
            panic!("UUID portable emission failed for {triple}: {error:?}");
        });
        let ir = std::fs::read_to_string(&ir_path).unwrap();
        for actual_body in ["Data.Uuid.compare-packed", "@llvm.bswap.i64", "<8 x i8>"] {
            assert!(
                ir.contains(actual_body),
                "{triple}: missing actual adapter body {actual_body}"
            );
        }
        let output = std::process::Command::new(
            std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()),
        )
        .args(["-target", triple, "-O2", "-c", "-x", "ir"])
        .arg(&ir_path)
        .arg("-o")
        .arg(directory.path().join("consumer.o"))
        .output()
        .unwrap();
        std::fs::write(directory.path().join("clang.stdout"), &output.stdout).unwrap();
        std::fs::write(directory.path().join("clang.stderr"), &output.stderr).unwrap();
        assert!(
            output.status.success(),
            "{triple}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
