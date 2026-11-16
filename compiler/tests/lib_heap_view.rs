//! Independent indexed heap traces, callback-visible bookkeeping and owned priority repairs.
mod common;

use cinch::driver::Mode;
use std::path::PathBuf;

/// Project one offline reference corpus without requiring the reference SDK during tests.
fn source(index: usize, mode: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let output = std::process::Command::new("python3")
        .arg(root.join("tooling/generate_heap_view_cases.py"))
        .args(["--case", &index.to_string(), "--mode", mode])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

/// Build the explicit frontend and separate ordinary reclamation from poisoned-free quarantine checks.
fn run(index: usize, mode: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (name, contents) in [
        (
            "Data.Mutable.Heap",
            include_str!("../prelude/Data/Mutable/Heap.cinch"),
        ),
        (
            "Data.Mutable.HeapView",
            include_str!("../prelude/Data/Mutable/HeapView.cinch"),
        ),
        (
            "Data.Mutable.Vec",
            include_str!("../prelude/Data/Mutable/Vec.cinch"),
        ),
        (
            "Data.Mutable.Var",
            include_str!("../prelude/Data/Mutable/Var.cinch"),
        ),
        (
            "Data.Bytes.Word",
            include_str!("../prelude/Data/Bytes/Word.cinch"),
        ),
        ("Text.Hex", include_str!("../prelude/Text/Hex.cinch")),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), contents).unwrap();
    }
    std::fs::write(directory.path().join("Main.cinch"), source(index, mode)).unwrap();
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
        let output = std::process::Command::new(&executable)
            .env("CINCH_HEAP_LIMIT", if debug { "256M" } else { "32M" })
            .env("CINCH_STACK_SIZE", "8M")
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_RC_DEBUG", if debug { "1" } else { "0" })
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "{mode}-{index} debug={debug}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
}

/// Exact callback traces cover short and empty initial heaps and all indexed operations.
#[test]
fn heap_view_short_traces_preserve_every_callback_and_identity_position() {
    for index in 0..8 {
        run(index, "trace");
    }
}

/// Larger heaps retain independently specified storage order, repair paths and bookkeeping.
#[test]
fn heap_view_large_traces_match_independent_indexed_operations() {
    for index in 8..16 {
        run(index, "trace");
    }
}

/// Owned four-way storage supports checked removal, replacement and externally changed priorities.
#[test]
fn owned_heap_repairs_and_rebuilds_match_independently_sorted_values() {
    for index in 0..16 {
        run(index, "owned");
    }
}

/// Empty callbacks remain untouched, huge absent indices are safe, and aliases observe mutations.
#[test]
fn heap_empty_alias_and_unbounded_index_policies_are_explicit() {
    run(0, "control");
}
