//! ARC-borrowed retaining primitives must not capture frame-local managed constructors.
use std::path::Path;
use std::process::Command;

/// Require the actual named read-only control to retain a frame allocation.
fn has_control_frame(diagnostics: &str, name: &str) -> bool {
    diagnostics.lines().any(|line| {
        line.contains(&format!("cinch-stack: Main.{name} local ")) && line.contains(": slot ")
    })
}

/// Build one exact source with the frame pass enabled or disabled and run both reclamation profiles.
fn run(directory: &Path, no_stack: bool) {
    let executable = directory.join(if no_stack { "heap-only" } else { "frame-safe" });
    let mut command = Command::new(env!("CARGO_BIN_EXE_cinch"));
    command
        .current_dir(directory)
        .env("CINCH_STACK_TRACE", "1")
        .env("CINCH_NO_CPR", "1")
        .env("CINCH_NO_PRUNE", "1")
        .args([
            "build",
            "Main.cinch",
            "--opt-level",
            "2",
            "--no-inline",
            "--max-simplifier-iterations",
            "0",
            "-o",
        ])
        .arg(&executable);
    if no_stack {
        command.arg("--no-stack-alloc");
    }
    let built = command.output().unwrap();
    let diagnostics = String::from_utf8_lossy(&built.stderr);
    assert!(built.status.success(), "no_stack={no_stack}: {diagnostics}");
    if !no_stack {
        assert!(
            has_control_frame(&diagnostics, "read-managed"),
            "managed read-only frame was lost: {diagnostics}"
        );
        assert!(
            has_control_frame(&diagnostics, "read-raw"),
            "raw read-only frame was lost: {diagnostics}"
        );
    }
    for poisoned in [false, true] {
        let output = Command::new(&executable)
            .env("CINCH_HEAP_LIMIT", if poisoned { "256M" } else { "32M" })
            .env("CINCH_STACK_SIZE", "8M")
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_RC_DEBUG", if poisoned { "1" } else { "0" })
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "no_stack={no_stack}, poisoned={poisoned}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
}

/// Cover direct and two-helper Array Set/Replicate/TensorSet capture with a heap-only control and preserved readers.
#[test]
fn retained_managed_array_values_outlive_default_and_forwarding_call_frames() {
    let directory = tempfile::tempdir().unwrap();
    for (name, source) in [
        (
            "Data.Array.cinch",
            include_str!("../prelude/Data/Array.cinch"),
        ),
        (
            "Data.Tensor.cinch",
            include_str!("../prelude/Data/Tensor.cinch"),
        ),
    ] {
        std::fs::write(directory.path().join(name), source).unwrap();
    }
    let mut source = include_str!("fixtures/array-retain-frame.cinch").to_string();
    let cases = [
        "set-direct",
        "set-forwarded",
        "replicate-direct",
        "replicate-indirect",
        "tensor-direct",
        "tensor-indirect",
        "read-managed",
        "read-raw",
    ];
    source.push_str("\n  -- | Return a stable first-failure index after every retained-value and read-only control.\n  main :: Int\n  main = ");
    for (index, name) in cases.iter().enumerate() {
        source.push_str(&format!("if {name} () then "));
        if index + 1 == cases.len() {
            source.push_str("0");
        }
    }
    for index in (1..=cases.len()).rev() {
        source.push_str(&format!(" else {index}"));
    }
    source.push('\n');
    std::fs::write(directory.path().join("Main.cinch"), source).unwrap();
    run(directory.path(), false);
    run(directory.path(), true);
}
