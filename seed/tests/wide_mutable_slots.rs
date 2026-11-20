//! Public mutable cells and vectors preserve both limbs through word-based storage.

mod common;

/// High-limb observations exercise replacement, overlap, growth, clearing and signed payloads.
const SOURCE: &str = include_str!("fixtures/wide-mutable-slots.cinch");

/// Actual optimized and unoptimized containers reclaim every boxed wide owner in both profiles.
#[test]
fn wide_cells_and_vectors_preserve_both_limbs_and_ownership() {
    for level in [0, 2] {
        let directory = tempfile::tempdir().unwrap();
        for (name, source) in [
            ("Main.cinch", SOURCE),
            ("Data.Wide.cinch", include_str!("../prelude/Data/Wide.cinch")),
            ("Data.Mutable.Var.cinch", include_str!("../prelude/Data/Mutable/Var.cinch")),
            ("Data.Mutable.Vec.cinch", include_str!("../prelude/Data/Mutable/Vec.cinch")),
        ] {
            std::fs::write(directory.path().join(name), source).unwrap();
        }
        let entry = directory.path().join("Main.cinch");
        let executable = directory.path().join("consumer");
        let output = executable.clone();
        let built = std::thread::Builder::new().stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let profile = cinch::core::TargetProfile {
                    reclaim: true, rc_debug: true, ..cinch::core::TargetProfile::native()
                };
                cinch::driver::driver_with_profile_opts(
                    cinch::driver::Mode::Build, cinch::check::StageMode::Refresh,
                    &entry, Some(&output), &profile,
                    &cinch::driver::DriverOptions { opt_level: Some(level), ..Default::default() },
                )
            }).unwrap().join().unwrap();
        if let Err(error) = built {
            let retained = directory.keep();
            panic!("O{level}: {error}; retained source: {}", retained.display());
        }
        for poisoned in [false, true] {
            let result = std::process::Command::new(&executable)
                .env("CINCH_HEAP_LIMIT", if poisoned { "256M" } else { "32M" })
                .env("CINCH_STACK_SIZE", "8M")
                .env("CINCH_RC_DEBUG", if poisoned { "1" } else { "0" })
                .env("CINCH_LEAK_CHECK", "1").output().unwrap();
            let diagnostic = String::from_utf8_lossy(&result.stderr);
            if result.status.code() != Some(0) || !diagnostic.contains("cinch-leak: live=0 immortal=0 leaked=0") {
                let retained = directory.keep();
                panic!("O{level}, poisoned={poisoned}: {:?}; {diagnostic}; retained: {}", result.status.code(), retained.display());
            }
            assert!(result.stdout.is_empty());
        }
    }
}
