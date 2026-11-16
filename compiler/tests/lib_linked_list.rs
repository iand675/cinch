//! Stable mutable list handles preserve exact sequence, identity, membership and detached-value behavior.
mod common;

use cinch::driver::Mode;
use std::path::PathBuf;

/// Project an independently generated trace without requiring its reference SDK at test time.
fn source(index: usize, mode: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let output = std::process::Command::new("python3")
        .arg(root.join("tooling/generate_linked_list_cases.py"))
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

/// Compile the actual module, separating ordinary reclamation from poisoned-free quarantine validation.
fn run(index: usize, mode: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (name, contents) in [
        (
            "Data.Mutable.LinkedList",
            include_str!("../prelude/Data/Mutable/LinkedList.cinch"),
        ),
        (
            "Data.Mutable.Vec",
            include_str!("../prelude/Data/Mutable/Vec.cinch"),
        ),
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
            "case {index}, {mode}, poisoned={debug}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let errors = String::from_utf8_lossy(&output.stderr);
        assert!(
            errors.contains("cinch-leak: live=0") && errors.contains("leaked=0"),
            "{errors}"
        );
    }
}

/// Check empty and short sequences with all independently recorded structural actions.
#[test]
fn small_lists_preserve_complete_reference_traces() {
    for index in 0..4 {
        run(index, "trace");
    }
}

/// Check geometric growth, clear generations, slot reuse and retained removed handles.
#[test]
fn grown_lists_preserve_complete_reference_traces() {
    for index in 4..6 {
        run(index, "trace");
    }
}

/// Check long traversals and bounded self-copy while retaining every detached node.
#[test]
fn large_lists_preserve_complete_reference_traces() {
    for index in 6..8 {
        run(index, "trace");
    }
}

/// Check named stale/foreign refusals and owned byte values after the root is destroyed.
#[test]
fn checked_handles_preserve_detached_value_ownership() {
    run(0, "control");
}
