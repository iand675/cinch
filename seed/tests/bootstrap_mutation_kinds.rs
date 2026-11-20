//! Mutation effect labels retain scoped kind authority without changing ownership constructors.
mod common;

/// Compile the actual authority checker and verify its positive and forged-Core cases under bounded reclamation.
#[test]
fn mutation_row_labels_preserve_borrow_and_nominal_kind_authority() {
    let directory = common::project(
        "mutation_kinds",
        &[
            (
                "Bootstrap.Type.cinch",
                include_str!("../../compiler/Bootstrap.Type.cinch"),
            ),
            (
                "Bootstrap.Kinds.cinch",
                include_str!("../../compiler/Bootstrap.Kinds.cinch"),
            ),
            (
                "Main.cinch",
                include_str!("../../compiler/programs/Run.MutationKinds.cinch"),
            ),
        ],
    );
    let executable = directory.join("mutation-kinds");
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
    .expect("build mutation/ownership kind authority checker");
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
            "debug={debug}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
    common::cleanup(&directory);
}

/// The source producer accepts a concrete region-indexed declaration with the built-in mutation row.
#[test]
fn explicit_mutation_regions_close_without_shadowing_ownership_kinds() {
    let directory = common::project(
        "mutation_region_source",
        &[(
            "Main.cinch",
            include_str!("../../compiler/programs/Run.MutationRegion.cinch"),
        )],
    );
    let code = common::run(&directory, "Main.cinch");
    assert_eq!(code, 0);
    common::cleanup(&directory);
}
