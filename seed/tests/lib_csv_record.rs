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

/// Compile actual private byte classifiers, UTF-8 lead conversion and raw word loads for foreign objects.
/// This explicitly invokes the Rust emitter independently of the selected host frontend.
#[test]
fn rust_csv_kernels_compile_for_wasm_arm_and_big_endian_targets() {
    let source = include_str!("../prelude/Text/Csv/Record.cinch");
    let mut kernels = String::from("-- | Isolated CSV SIMD classifier compilation.\nmodule Main where\n  import Data.Machine\n");
    for name in [
        "contains-byte8",
        "special8",
        "scalar-first",
        "scan-match16",
        "scan-special16",
    ] {
        let start = source.find(&format!("  prim {name} ::")).unwrap();
        let remaining = &source[start..];
        let end = remaining.find("\n  -- |").unwrap_or(remaining.len());
        kernels.push_str("\n  -- | Actual private byte-classification or UTF-8 lead kernel.\n");
        kernels.push_str(&remaining[..end]);
    }
    let words = include_str!("../prelude/Data/Bytes/Word.cinch");
    let start = words.find("  prim load-le64 ::").unwrap();
    let remaining = &words[start..];
    let end = remaining.find("\n  -- |").unwrap_or(remaining.len());
    kernels.push_str(
        "\n  -- | Actual private exact unaligned word load with target endianness normalization.\n",
    );
    kernels.push_str(&remaining[..end]);
    kernels.push_str("\n  -- | Retain the complete fixture input while the private word load finishes.\n  keep :: Bytes -> U64 -> U64\n  keep bytes value = case bytes-len bytes of _ -> value\n  -- | Retain the complete fixture owner across a private sixteen-byte load.\n  keep-flag :: Bytes -> U8 -> U8\n  keep-flag bytes value = case bytes-len bytes of _ -> value\n  -- | Keep the actual classifiers and memory load reachable without claiming foreign execution.\n  main :: Int\n  main = let bytes = bytes-new 16 in let word = keep bytes (load-le64 (bytes-data bytes) 0) in let a = keep-flag bytes (scan-match16 (bytes-data bytes) 10) in let b = keep-flag bytes (scan-special16 (bytes-data bytes) 44 13) in if eq-u8 a b && eq-u8 (contains-byte8 word 10) (special8 word (scalar-first 955)) then 0 else 1\n");
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
        let input = kernels.clone();
        let ir = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let program =
                    cinch::driver::check_sources_profile(&input, "csv-kernels.cinch", &profile)?;
                cinch::codegen::emit_for(&profile, &program)
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("<8 x i8>") && ir.contains("<16 x i8>"));
        let path = directory.path().join(format!("{arch}.ll"));
        std::fs::write(&path, &ir).unwrap();
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
