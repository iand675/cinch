//! Full-width constant evaluation agrees with emitted scalar operations at both optimization levels.
mod common;

/// Compile actual scalar APIs and observe exact results under bounded ordinary and poisoned profiles.
fn run(source: &str, levels: &[u8]) {
    for &level in levels {
        let directory = tempfile::tempdir().unwrap();
        let input = directory.path().join("Main.cinch");
        std::fs::write(&input, source).unwrap();
        let executable = directory.path().join("consumer");
        let output = executable.clone();
        std::thread::Builder::new()
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let profile = cinch::core::TargetProfile {
                    reclaim: true,
                    rc_debug: true,
                    ..cinch::core::TargetProfile::native()
                };
                cinch::driver::driver_with_profile_opts(
                    cinch::driver::Mode::Build,
                    cinch::check::StageMode::Refresh,
                    &input,
                    Some(&output),
                    &profile,
                    &cinch::driver::DriverOptions { opt_level: Some(level), ..Default::default() },
                )
            }).unwrap().join().unwrap().unwrap();
        for poisoned in [false, true] {
            let result = std::process::Command::new(&executable)
                .env("CINCH_HEAP_LIMIT", if poisoned { "256M" } else { "32M" })
                .env("CINCH_STACK_SIZE", "8M")
                .env("CINCH_RC_DEBUG", if poisoned { "1" } else { "0" })
                .env("CINCH_LEAK_CHECK", "1")
                .output().unwrap();
            let diagnostics = String::from_utf8_lossy(&result.stderr);
            if result.status.code() != Some(0) {
                let retained = directory.keep();
                panic!("O{level}, poisoned={poisoned}: exit {:?}; {diagnostics}\nretained regression: {}",
                    result.status.code(), retained.display());
            }
            assert!(diagnostics.contains("cinch-leak: live=0") && diagnostics.contains("leaked=0"), "{diagnostics}");
        }
    }
}

/// Constant folding and emitted operations retain exact unsigned magnitudes at both optimization levels.
#[test]
fn full_width_constants_agree_with_emitted_operations() {
    run(include_str!("fixtures/word-constant-folding.cinch"), &[0, 2]);
}

/// Staged saturation follows the same bounds without host signed-overflow faults.
#[test]
fn staged_negation_preserves_signed_and_unsigned_saturation() {
    run(include_str!("fixtures/word-negation-staging.cinch"), &[0]);
}
