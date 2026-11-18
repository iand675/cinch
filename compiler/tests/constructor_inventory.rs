//! Checked constructor inventories survive specialization without inventing ABI fields.
mod common;

/// A zero-arity generic class value reaches None while Some's constructor unit
/// is dead. Both normal and poisoned executions must keep that valid program
/// accepted, even though the original checked ABI still contains both tags.
#[test]
fn none_only_zero_arity_class_default_survives_constructor_pruning() {
    let source = r#"-- | A generic value method that needs only the absent Option constructor.
module Main where
  import Data.Core

  -- | A distinguished value chosen explicitly for a carrier.
  signature Zero a where
    -- | Obtain the carrier's distinguished value without a value argument.
    zero :: a

  -- | Every option carrier chooses absence, independently of its payload type.
  implicit instance Zero_Option a :: Zero (Option a) where
    zero = None

  -- | Complete successfully after the value-method dictionary is lowered.
  main :: Int
  main = 0
"#;
    let directory = common::project("none_only_constructor_inventory", &[("Main.cinch", source)]);
    let executable = directory.join("consumer");
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
    .unwrap_or_else(|error| panic!("None-only class default failed to build: {error}"));
    let ir = std::fs::read_to_string(executable.with_extension("ll")).unwrap();
    assert!(
        !ir.lines()
            .any(|line| line.starts_with("define ") && line.contains("Data.Core.Some")),
        "the regression must exercise a pruned Some constructor unit"
    );
    for (poison, heap) in [("0", "32M"), ("1", "256M")] {
        let result = std::process::Command::new(&executable)
            .env("CINCH_LEAK_CHECK", "1")
            .env("CINCH_RC_DEBUG", poison)
            .env("CINCH_HEAP_LIMIT", heap)
            .env("CINCH_STACK_SIZE", "8M")
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "heap={heap}, poison={poison}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(String::from_utf8_lossy(&result.stderr).contains("leaked=0"));
    }
    common::cleanup(&directory);
}
