//! Stable map entries preserve complete reference traces, live mutation and IEEE key policies.
mod common;

use cinch::driver::Mode;
use std::path::PathBuf;

/// Render independent recorded observations without requiring the reference SDK in this gate.
fn rendered(arguments: &[&str]) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    let directory = tempfile::tempdir().unwrap();
    let source = directory.path().join("Main.cinch");
    let output = std::process::Command::new("python3")
        .arg(root.join("tooling/generate_maps_cases.py"))
        .args(arguments)
        .arg("--output")
        .arg(&source)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    std::fs::read_to_string(source).unwrap()
}

/// Select the actual library sources rather than accepting a stale embedded implementation.
fn project(consumer: &str) -> tempfile::TempDir {
    let directory = tempfile::tempdir().unwrap();
    for (name, contents) in [
        (
            "Data.Mutable.Map",
            include_str!("../prelude/Data/Mutable/Map.cinch"),
        ),
        (
            "Data.Mutable.Map.Optional",
            include_str!("../prelude/Data/Mutable/Map/Optional.cinch"),
        ),
        (
            "Data.Mutable.Map.Float",
            include_str!("../prelude/Data/Mutable/Map/Float.cinch"),
        ),
        (
            "Data.Mutable.HashTable",
            include_str!("../prelude/Data/Mutable/HashTable.cinch"),
        ),
        (
            "Data.Mutable.LinkedList",
            include_str!("../prelude/Data/Mutable/LinkedList.cinch"),
        ),
        (
            "Data.Mutable.Var",
            include_str!("../prelude/Data/Mutable/Var.cinch"),
        ),
    ] {
        std::fs::write(directory.path().join(format!("{name}.cinch")), contents).unwrap();
    }
    std::fs::write(directory.path().join("Main.cinch"), consumer).unwrap();
    directory
}

/// Build the selected source and check bounded reclamation separately from poisoned quarantine.
fn run(consumer: &str) {
    let directory = project(consumer);
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
    for poisoned in [false, true] {
        let output = std::process::Command::new(&executable)
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
            "poisoned={poisoned}: {diagnostics}"
        );
        assert!(
            diagnostics.contains("cinch-leak: live=0 immortal=0 leaked=0"),
            "{diagnostics}"
        );
    }
}

/// Cover collection, overwrite, clone, copy, traversal, deletion and heterogeneous equality.
#[test]
fn complete_independent_map_traces_preserve_every_state() {
    for index in 0..8 {
        run(&rendered(&["--case", &index.to_string()]));
    }
}

/// Preserve signaling/quiet NaN payload multiplicities and signed-zero replacement semantics.
#[test]
fn explicit_ieee_key_policies_preserve_all_recorded_bits() {
    for width in [32, 64] {
        run(&rendered(&["--float-width", &width.to_string()]));
    }
}

/// Pin live identities across callback growth, reuse, clear, nested traversal and retained owners.
#[test]
fn live_mutation_owners_and_late_producer_callbacks_are_safe() {
    run(include_str!("fixtures/mutable-map-control.cinch"));
}

/// Preserve non-reflexive values, predicate replacements and current right-side equality lookups.
#[test]
fn value_nan_and_predicate_mutation_contracts_remain_explicit() {
    run(&rendered(&["--edge-control"]));
}

/// Require actual IO-capable sources and callbacks without reducing the API's effect row.
#[test]
fn map_sources_and_callbacks_compose_with_io_effects() {
    run(include_str!("fixtures/mutable-map-effect-control.cinch"));
}

/// Distinct source and destination regions must remain usable in higher-order composition.
#[test]
fn independent_regions_compose_without_implicit_identity_unification() {
    for consumer in [
        include_str!("fixtures/maps-region-pair.cinch"),
        include_str!("fixtures/maps-region-source.cinch"),
    ] {
        let directory = project(consumer);
        common::drive(Mode::Check, directory.path(), "Main.cinch", None).unwrap();
    }
}

/// Keep mutable observations behind their declared region capability even from a pure caller.
#[test]
fn a_pure_function_cannot_observe_mutable_map_state() {
    let directory = project(include_str!(
        "fixtures/mutable-map-negative-pure-read.cinch"
    ));
    let error = common::drive(Mode::Check, directory.path(), "Main.cinch", None).unwrap_err();
    assert!(error.to_lowercase().contains("effect"), "{error}");
}
