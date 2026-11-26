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

/// Explain a stable diagnostic exit without collapsing private context failures into source cases.
fn failure_description(code: Option<i32>) -> String {
    /// Stable labels match the one-based private fixture order.
    const CONTEXT_CASES: [&str; 14] = [
        "actual sealed primitive payload",
        "missing compiler-owned certificate",
        "competing local type header",
        "competing imported defining identity",
        "competing local alias name",
        "ordinary imported flag preserves real builtin authority",
        "value binding does not replace a type identity",
        "unrelated qualified constructor",
        "shadowed higher-kinded binder",
        "forged primitive constructor kind",
        "oversaturated primitive application",
        "recursive quantified value domain",
        "array-wrapped quantified value domain",
        "uncertified structural refinement wrapper",
    ];
    match code {
        Some(value) if (101..=114).contains(&value) => format!(
            "private context condition {}: {}",
            value - 100,
            CONTEXT_CASES[(value - 101) as usize]
        ),
        Some(value) if value > 0 => {
            let cases: Vec<Case> = serde_json::from_str(include_str!(
                "fixtures/nested-array-positivity-vectors.json"
            ))
            .unwrap();
            match cases.get((value - 1) as usize) {
                Some(case) => format!("source case {value}: {}", case.name),
                None => format!("unexpected process exit {value}"),
            }
        }
        other => format!("process status {other:?}"),
    }
}

/// Execute the actual Native source elaborator in normal reclamation and poisoned-free profiles.
#[test]
fn cinch_source_array_payload_evidence_matches_the_independent_verdicts() {
    let inference = format!(
        "{}\n{}",
        include_str!("../../compiler/Cinch.Infer.cinch"),
        include_str!("fixtures/nested-array-positivity-context.cinch")
    );
    let source = include_str!("../../compiler/programs/Run.NestedArrayPositivity.cinch");
    let marker = "main = first-failure ";
    assert_eq!(source.matches(marker).count(), 1);
    let checked_source = source.replace(
        marker,
        "main = let context-failure = payload-context-first-failure () in if context-failure == 0 then first-failure ",
    );
    let checked_source = format!("{} else 100 + context-failure\n", checked_source.trim_end());
    let directory = common::project(
        "nested_array_positivity",
        &[
            (
                "Cinch.Check.cinch",
                include_str!("../../compiler/Cinch.Check.cinch"),
            ),
            (
                "Cinch.Resolve.cinch",
                include_str!("../../compiler/Cinch.Resolve.cinch"),
            ),
            (
                "Cinch.Inputs.cinch",
                include_str!("../../compiler/Cinch.Inputs.cinch"),
            ),
            ("Cinch.Infer.cinch", &inference),
            ("Main.cinch", &checked_source),
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
            "poisoned={poisoned}, {}: {}",
            failure_description(output.status.code()),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
    common::cleanup(&directory);
}

/// Finite nested arrays retain managed leaves through persistent updates, shared values and temporary-owner release.
#[test]
fn recursive_array_runtime_owners_survive_persistent_aliases() {
    let directory = common::project(
        "nested_array_runtime",
        &[
            (
                "Data.Array.cinch",
                include_str!("../prelude/Data/Array.cinch"),
            ),
            (
                "Main.cinch",
                include_str!("fixtures/nested-array-runtime-control.cinch"),
            ),
        ],
    );
    let executable = directory.join("nested-array-runtime");
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
    .expect("build actual finite recursive-array runtime fixture");
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
