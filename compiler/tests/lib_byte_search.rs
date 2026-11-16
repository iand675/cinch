//! Exact raw-byte offsets, broad-word lane masks and borrowed backing owners.
mod common;
use cinch::driver::Mode;
use serde_json::Value;

/// Runtime consumers compare actual public operations with independent exact offsets and counts.
const HELPERS: &str = include_str!("fixtures/byte-search-consumer.cinchpart");
/// The implementation is source-distributed and preserves the embedded buffer representation.
const SEARCH: &str = include_str!("../prelude/Data/Bytes/Search.cinch");

/// Read independently generated reference observations without requiring the SDK at test time.
fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/byte-search-vectors.json")).unwrap()
}

/// Build a complete consumer with bounded source batches.
fn consumer(checks: &[String], width: usize) -> String {
    let mut source = HELPERS.to_owned();
    let mut names = Vec::new();
    for (i, batch) in checks.chunks(width).enumerate() {
        source.push_str(&format!("\n  -- | Observe independent byte-search fixture batch {i}.\n  checks{i} :: Unit -> Bool\n  checks{i} unit = all [\n"));
        source.push_str(
            &batch
                .iter()
                .map(|value| format!("    ({value})"))
                .collect::<Vec<_>>()
                .join(",\n"),
        );
        source.push_str("\n    ]\n");
        names.push(format!("checks{i} ()"));
    }
    let mut dispatch = "False".to_owned();
    for (i, name) in names.iter().enumerate().rev() {
        dispatch = format!("if batch == \"{i}\" then {name} else {dispatch}");
    }
    source.push_str(&format!("\n  -- | Select one bounded poison-quarantine batch.\n  selected :: String -> Bool\n  selected batch = {dispatch}\n  -- | Observe one public query batch.\n  main :: Unit ->{{IO r}} Int\n  main unit = with (io = newIO ()) in case process-args () of\n    Ok (Cons batch Nil) -> if selected batch then 0 else 1\n    other -> 2\n"));

    source
}

/// Compile the selected frontend or an explicit Rust O2 ownership probe and check both reclamation modes.
fn run(checks: Vec<String>, optimized: bool) {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("Data.Bytes.Search.cinch"), SEARCH).unwrap();
    let input = directory.path().join("Main.cinch");
    let width = if checks.iter().any(|check| check.contains("owners-case")) {
        1
    } else {
        64
    };
    std::fs::write(&input, consumer(&checks, width)).unwrap();
    let exe = directory.path().join("consumer");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    if optimized {
        let (entry, output, profile) = (input.clone(), exe.clone(), profile.clone());
        std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                cinch::driver::driver_with_profile_opts(
                    Mode::Build,
                    cinch::check::StageMode::Refresh,
                    &entry,
                    Some(&output),
                    &profile,
                    &cinch::driver::DriverOptions {
                        opt_level: Some(2),
                        ..Default::default()
                    },
                )
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
    } else {
        common::drive_with_profile(
            Mode::Build,
            cinch::check::StageMode::Refresh,
            directory.path(),
            "Main.cinch",
            Some(exe.clone()),
            &profile,
        )
        .unwrap();
    }
    for debug in [false, true] {
        for batch in 0..checks.len().div_ceil(width) {
            let output = std::process::Command::new(&exe)
                .arg(batch.to_string())
                .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
                .env("CINCH_LEAK_CHECK", "1")
                .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
                .env("CINCH_STACK_SIZE", "8M")
                .output()
                .unwrap();
            assert_eq!(
                output.status.code(),
                Some(0),
                "debug={debug}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
        }
    }
}

/// Every byte value and complete-word fringe matches independent first/last/count observations.
#[test]
fn exact_byte_queries_match_independent_offsets_and_counts() {
    let f = fixture();
    run(
        f["rows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                format!(
                    "search-case {} {} {} ({}) ({}) {}",
                    r["mode"], r["size"], r["needle"], r["first"], r["last"], r["count"]
                )
            })
            .collect(),
        false,
    );
}

/// Temporary inputs remain owned until all complete-word and fringe loads finish.
#[test]
fn exact_byte_queries_retain_temporary_backing_owners() {
    run(vec!["owners-case ()".into(); 10], false);
}

/// Optimized raw scans retain backing owners through repeated sixteen-kibibyte-plus-one pipelines.
#[test]
fn rust_exact_byte_queries_optimized_temporary_owners() {
    run(vec!["owners-case ()".into(); 10], true);
}

/// Actual raw loads and packed lane arithmetic compile for portable and big-endian object profiles.
#[test]
fn byte_search_kernels_compile_for_wasm_arm_and_big_endian_targets() {
    let directory = tempfile::tempdir().unwrap();
    let start = SEARCH.find("  -- | Keep one backing owner").unwrap();
    let end = SEARCH
        .find("  -- | Convert a private signed search sentinel")
        .unwrap();
    let kernels=format!("-- | Isolated exact byte-search implementation kernels.\nmodule Main where\n  import Data.Bytes\n  import Data.Bool\n  import Data.Machine\n  import Data.Nat\n{}\n{}",&SEARCH[start..end],include_str!("fixtures/byte-search-kernel-entry.cinchpart"));
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
        let source = kernels.clone();
        let ir = std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let program = cinch::driver::check_sources_profile(
                    &source,
                    "byte-search-kernels.cinch",
                    &profile,
                )?;
                cinch::codegen::emit_for(&profile, &program)
            })
            .unwrap()
            .join()
            .unwrap()
            .unwrap();
        assert!(ir.contains("llvm.ctpop.i64"));
        let path = directory.path().join(format!("{arch}.ll"));
        std::fs::write(&path, ir).unwrap();
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

/// All new implementation declarations have canonical owned documentation.
#[test]
fn byte_search_declarations_have_canonical_documentation() {
    let parsed = cinch::cinchdoc::parse_source(
        SEARCH,
        Some("Data.Bytes.Search"),
        "Data.Bytes.Search",
        None,
        false,
    )
    .unwrap();
    assert!(parsed.doc.is_some());
    assert!(parsed.diagnostics.is_empty());
    assert!(parsed.orphans.is_empty());
    let mut declarations = Vec::new();
    cinch::cinchdoc::flatten(&parsed.declarations, &mut declarations);
    for declaration in declarations {
        let doc = declaration
            .doc
            .as_ref()
            .unwrap_or_else(|| panic!("missing docs: {}", declaration.id));
        assert!(!doc.legacy);
        let text = doc.markdown.to_lowercase();
        assert!(!text.contains("golang") && !text.contains("go's") && !text.contains("go "));
    }
}
