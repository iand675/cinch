//! Sealed finite-array payload evidence admits recursion without granting unknown variance.
mod common;

use common::source_frontend::check_sources;
use serde::Deserialize;

/// One source verdict with a required semantic diagnostic for every refusal.
#[derive(Deserialize)]
struct Case {
    name: String,
    source: String,
    accepted: bool,
    diagnostic: String,
}

/// Check the same source corpus through the selected frontend and require named failures.
#[test]
fn source_array_payload_admission_preserves_strict_positivity_boundaries() {
    let cases: Vec<Case> = serde_json::from_str(include_str!(
        "fixtures/nested-array-positivity-vectors.json"
    ))
    .unwrap();
    for case in cases {
        match check_sources(&case.source, "nested-array-positivity") {
            Ok(()) => assert!(case.accepted, "{} unexpectedly checked", case.name),
            Err(error) => {
                assert!(!case.accepted, "{} failed: {error}", case.name);
                assert!(
                    error.contains(&case.diagnostic),
                    "{}: expected {:?}, got {error}",
                    case.name,
                    case.diagnostic
                );
            }
        }
    }
}

/// Execute the actual Native source elaborator in normal reclamation and poisoned-free profiles.
#[test]
fn cinch_source_array_payload_evidence_matches_the_independent_verdicts() {
    let directory = common::project(
        "nested_array_positivity",
        &[
            (
                "Bootstrap.Check.cinch",
                include_str!("../../bootstrap/Bootstrap.Check.cinch"),
            ),
            (
                "Bootstrap.Resolve.cinch",
                include_str!("../../bootstrap/Bootstrap.Resolve.cinch"),
            ),
            (
                "Bootstrap.Inputs.cinch",
                include_str!("../../bootstrap/Bootstrap.Inputs.cinch"),
            ),
            (
                "Bootstrap.Infer.cinch",
                include_str!("../../bootstrap/Bootstrap.Infer.cinch"),
            ),
            (
                "Main.cinch",
                include_str!("../../bootstrap/programs/Run.NestedArrayPositivity.cinch"),
            ),
        ],
    );
    let executable = directory.join("nested-array-positivity");
    let profile = cinch::core::TargetProfile {
        reclaim: true,
        rc_debug: true,
        ..cinch::core::TargetProfile::native()
    };
    common::drive_with_profile(
        cinch::driver::Mode::Build,
        cinch::check::StageMode::Refresh,
        &directory,
        "Main.cinch",
        Some(executable.clone()),
        &profile,
    )
    .expect("build sealed-array positivity checker");
    for poisoned in [false, true] {
        let output = std::process::Command::new(&executable)
            .env("CINCH_HEAP_LIMIT", if poisoned { "256M" } else { "32M" })
            .env("CINCH_STACK_SIZE", "8M")
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_RC_DEBUG", if poisoned { "1" } else { "0" })
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(0),
            "poisoned={poisoned}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
    common::cleanup(&directory);
}
