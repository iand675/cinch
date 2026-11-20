//! Source-owned phantom regions and strict value restrictions are checked by the Cinch elaborator.
mod common;

/// Exercise scoped kinds, alias monomorphism, implicit cells and polymorphic function values in checked source.
#[test]
fn mutable_regions_and_value_restrictions_preserve_source_authority() {
    let directory = common::project(
        "mutable_inference",
        &[
            (
                "Bootstrap.Check.cinch",
                include_str!("../../compiler/Bootstrap.Check.cinch"),
            ),
            (
                "Bootstrap.Resolve.cinch",
                include_str!("../../compiler/Bootstrap.Resolve.cinch"),
            ),
            (
                "Bootstrap.Inputs.cinch",
                include_str!("../../compiler/Bootstrap.Inputs.cinch"),
            ),
            (
                "Main.cinch",
                include_str!("../../compiler/programs/Run.MutableInference.cinch"),
            ),
        ],
    );
    let executable = directory.join("mutable-inference");
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
    .expect("build source mutation-region/value-restriction checker");
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
            "source case failed, debug={debug}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains("leaked=0"));
    }
    common::cleanup(&directory);
}
