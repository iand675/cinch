//! Stable circular cursors preserve independent join/split traces and managed owners.
mod common;

use cinch::driver::{DriverOptions, Mode};
use std::path::PathBuf;

/// Render independent offline observations without requiring the reference SDK during tests.
fn source(index: usize, mode: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let output = std::process::Command::new("python3")
        .arg(root.join("tooling/generate_ring_cases.py"))
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

/// Build actual source with generated count checks, then separate reclamation from poisoned quarantine.
fn run(index: usize, mode: &str) {
    let directory = tempfile::tempdir().unwrap();
    for (name, contents) in [
        (
            "Data.Mutable.Ring",
            include_str!("../prelude/Data/Mutable/Ring.cinch"),
        ),
        (
            "Data.Mutable.Vec",
            include_str!("../prelude/Data/Mutable/Vec.cinch"),
        ),
        (
            "Data.Mutable.Var",
            include_str!("../prelude/Data/Mutable/Var.cinch"),
        ),
        ("Data.Array", include_str!("../prelude/Data/Array.cinch")),
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
    let profiles: Vec<(bool, Vec<String>)> = if mode == "window" {
        (0..96)
            .step_by(4)
            .map(|start| (true, vec![start.to_string(), (start + 4).to_string()]))
            .collect()
    } else if index == 7 && mode == "trace" {
        vec![(false, vec![])]
    } else {
        vec![(false, vec![]), (true, vec![])]
    };
    for (poisoned, arguments) in profiles {
        let output = std::process::Command::new(&executable)
            .args(&arguments)
            .env("CINCH_HEAP_LIMIT", if poisoned { "256M" } else { "32M" })
            .env("CINCH_STACK_SIZE", "8M")
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_RC_DEBUG", if poisoned { "1" } else { "0" })
            .output()
            .unwrap();
        let diagnostics = String::from_utf8_lossy(&output.stderr);
        assert_eq!(
            output.status.code(),
            Some(0),
            "case {index}, {mode}, poisoned={poisoned}, arguments={arguments:?}: {diagnostics}"
        );
        assert!(
            diagnostics.contains("cinch-leak: live=0") && diagnostics.contains("leaked=0"),
            "{diagnostics}"
        );
    }
    if index == 7 && mode == "trace" {
        // Quarantine retains freed test observations. Replaying each prefix
        // with a bounded observation window covers every oracle state without
        // increasing the poisoned heap budget or changing normal coverage.
        run(index, "window");
    }
}

/// Check empty and short sequences against every complete recorded action and alias observation.
#[test]
fn small_rings_preserve_complete_independent_traces() {
    for index in 0..4 {
        run(index, "trace");
    }
}

/// Check geometric growth and relocation while retaining every original cursor.
#[test]
fn grown_rings_preserve_complete_independent_traces() {
    for index in 4..6 {
        run(index, "trace");
    }
}

/// Check large cycles and complete bidirectional observations from every retained node.
#[test]
fn large_rings_preserve_complete_independent_traces() {
    for index in 6..8 {
        run(index, "trace");
    }
}

/// Check retained byte owners, fresh copies, callback forwarding, huge moves and layout refusals.
#[test]
fn aliases_preserve_split_components_and_managed_payload_owners() {
    run(0, "control");
}

/// Check the actual SIMD lane kernel against full-width independently computed results.
#[test]
fn independent_link_offset_lanes_preserve_wrapping_arithmetic() {
    run(0, "kernel");
}

/// Require portable vector IR and real object emission for both narrower and wider target backends.
#[test]
fn link_offset_kernel_emits_portable_aarch64_and_wasm_objects() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("Main.cinch"), source(0, "kernel")).unwrap();
    for triple in ["aarch64-unknown-linux-gnu", "wasm32-unknown-unknown"] {
        let ir = directory.path().join(format!("{triple}.ll"));
        let options = DriverOptions {
            target: Some(triple.into()),
            ..DriverOptions::default()
        };
        common::drive_opts(
            Mode::EmitIr,
            directory.path(),
            "Main.cinch",
            Some(ir.clone()),
            &options,
        )
        .unwrap();
        let contents = std::fs::read_to_string(&ir).unwrap();
        assert!(
            contents.contains("add <2 x i64>"),
            "missing independent vector lane addition for {triple}"
        );
        let output = std::process::Command::new("clang")
            .args(["-target", triple, "-O2", "-c", "-x", "ir"])
            .arg(&ir)
            .arg("-o")
            .arg(ir.with_extension("o"))
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{triple}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
