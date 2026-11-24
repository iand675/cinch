//! Independent configurable CSV dialects, original byte positions and incremental stream ownership.
mod common;

use cinch::driver::Mode;

/// Compile a real source-distributed consumer through the selected frontend and validate both RC modes.
fn run(source: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (name, contents) in [
        (
            "Text.Csv.Record",
            include_str!("../prelude/Text/Csv/Record.cinch"),
        ),
        ("Effect.Csv", include_str!("../prelude/Effect/Csv.cinch")),
        (
            "Effect.Stream",
            include_str!("../prelude/Effect/Stream.cinch"),
        ),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), contents).unwrap();
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
    for debug in [false, true] {
        let result = std::process::Command::new(&executable)
            .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
            .env("CINCH_STACK_SIZE", "8M")
            .output()
            .unwrap();
        assert_eq!(
            result.status.code(),
            Some(0),
            "debug={debug}: stdout={} stderr={}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(String::from_utf8_lossy(&result.stderr).contains("leaked=0"));
    }
}

/// Configurable pure readers preserve independent fields, positions, progress and quote/count errors.
#[test]
fn csv_records_match_independent_reader_policies_positions_and_errors() {
    run(include_str!("fixtures/csv-record-read.cinch"));
}

/// Minimal quoting and LF/CRLF transformations reproduce the independent writer's complete bytes.
#[test]
fn csv_records_match_independent_writer_quoting_and_newlines() {
    run(include_str!("fixtures/csv-record-write.cinch"));
}

/// Stream readers remain independent of chunk boundaries, including one-byte UTF-8 fragments.
#[test]
fn csv_streams_match_independent_records_and_writer_bytes_across_chunk_sizes() {
    run(include_str!("fixtures/csv-record-stream.cinch"));
}

/// Raw records and writers preserve invalid UTF-8, embedded NUL and exact byte positions.
#[test]
fn csv_byte_records_and_streams_preserve_arbitrary_field_bytes() {
    run(include_str!("fixtures/csv-record-bytes.cinch"));
}

/// Prefixes, retained readers, source/sink errors and mutable buffer reuse retain precise semantics.
#[test]
fn csv_stream_failures_progress_and_retained_buffers_are_independent() {
    run(include_str!("fixtures/csv-record-control.cinch"));
}

/// Generated 64 KiB one-byte input and a long multiline field fit ordinary heap and stack bounds.
#[test]
fn csv_stream_one_byte_sources_and_multiline_records_are_bounded() {
    run(include_str!("fixtures/csv-record-bounded.cinch"));
}

/// Every new public/private declaration owns a canonical comment without foreign-library framing.
#[test]
fn csv_record_declarations_have_canonical_documentation() {
    for (name, source) in [
        (
            "Text.Csv.Record",
            include_str!("../prelude/Text/Csv/Record.cinch"),
        ),
        ("Effect.Csv", include_str!("../prelude/Effect/Csv.cinch")),
    ] {
        let parsed = cinch::cinchdoc::parse_source(source, Some(name), name, None, false).unwrap();
        assert!(parsed.doc.is_some());
        assert!(parsed.diagnostics.is_empty());
        assert!(parsed.orphans.is_empty());
        let mut declarations = Vec::new();
        cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
        for declaration in declarations {
            let doc = declaration
                .doc
                .as_ref()
                .unwrap_or_else(|| panic!("{}: missing documentation", declaration.id));
            assert!(!doc.legacy);
            let text = doc.markdown.to_lowercase();
            assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
        }
    }
}

/// Compile actual 16-byte SIMD CSV field scanning and quoting paths for foreign objects.
/// This explicitly invokes the Rust emitter independently of the selected host frontend.
#[test]
fn rust_csv_kernels_compile_for_wasm_arm_and_big_endian_targets() {
    let directory = tempfile::tempdir().unwrap();
    for (triple, arch, endian) in [
        ("wasm32-unknown-unknown", "wasm32", "little"),
        ("aarch64-unknown-linux-gnu", "aarch64", "little"),
        ("powerpc64-unknown-linux-gnu", "powerpc64", "big"),
    ] {
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
            profile
        });
        let project = directory.path().join(arch);
        std::fs::create_dir_all(&project).unwrap();
        for (name, contents) in [
            (
                "Text.Csv.Record",
                include_str!("../prelude/Text/Csv/Record.cinch"),
            ),
            (
                "Data.Bytes.Word",
                include_str!("../prelude/Data/Bytes/Word.cinch"),
            ),
        ] {
            std::fs::write(project.join(format!("{name}.cinch")), contents).unwrap();
        }
        std::fs::write(
            project.join("Main.cinch"),
            "-- | Reachable public CSV reading and rendering paths for foreign targets.\nmodule Main where\n  import Text.Csv.Record\n  import Data.Bytes\n  import Data.List\n  import Data.Nat\n  import Data.Result\n\n  -- | Exercise public 16-byte SIMD field scanning and quoting paths across targets.\n  main :: Int\n  main =\n    let field = bytes-new 32 in\n    let rendered = csv-render-byte-record csv-default-options False [field, field] in\n    case csv-byte-reader-read-all (csv-byte-reader csv-default-options rendered) of\n      Ok rows -> if list-length rows == 1 then 0 else 1\n      Err err -> 1\n",
        )
        .unwrap();
        let entry = project.join("Main.cinch");
        let path = project.join(format!("{arch}.ll"));
        let ir_path = path.clone();
        let ir = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::driver_with_profile_opts(
                    cinch::driver::Mode::EmitIr,
                    cinch::check::StageMode::Refresh,
                    &entry,
                    Some(&ir_path),
                    &profile,
                    &cinch::driver::DriverOptions::default(),
                )
                .map(|_| std::fs::read_to_string(&ir_path).unwrap())
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(
            ir.contains("<16 x i8>") && ir.contains("bitcast <16 x i1>") && ir.contains("load i128")
        );
        if endian == "big" {
            assert!(ir.contains("call i128 @llvm.bswap.i128"));
        }
        let output = std::process::Command::new(
            std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".into()),
        )
        .args(["-target", triple, "-O2", "-c", "-x", "ir"])
        .arg(&path)
        .arg("-o")
        .arg(path.with_extension("o"))
        .output()
        .unwrap();
        assert!(
            output.status.success(),
            "{triple}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
