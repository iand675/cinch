//! Optimized comparisons must read boxed naturals before their owners are released.

mod common;

/// Public arithmetic produces fresh boxed operands from a runtime byte count.
const SOURCE: &str = r#"-- | Compare independently allocated mathematical naturals.
module Main where
  import Data.Bytes
  import Data.Nat
  import Effect.Process
  -- | Observe equality and both orderings around immediate and multi-limb boundaries.
  ordered :: Nat -> Nat -> Bool
  ordered base size =
    let value = nat-add base size in
    let next = nat-add value 1 in
    nat-cmp (nat-add base size) value == 0
      && nat-cmp (nat-add base size) next < 0
      && nat-cmp (nat-add next 1) value > 0
  -- | Runtime inputs prevent constant folding from replacing the comparisons.
  main :: Unit ->{IO r} Int
  main unit = with (io = newIO ()) in case process-args () of
    Ok (Cons input Nil) ->
      let size = bytes-len (string-to-bytes input) in
      if ordered 9223372036854775807 size
        && ordered 18446744073709551616000000000000000000000000000000000000000000000000000000000000 size
      then 0 else 1
    other -> 2
"#;

/// Both generated counting paths preserve boxed reads under native optimization and poisoning.
#[test]
fn boxed_comparisons_precede_owned_operand_release() {
    for checked_counts in [true, false] {
        for level in [0, 2] {
            let project = tempfile::tempdir().unwrap();
            let entry = project.path().join("Main.cinch");
            let executable = project.path().join("consumer");
            std::fs::write(&entry, SOURCE).unwrap();
            let profile = cinch::core::TargetProfile {
                reclaim: true,
                rc_debug: checked_counts,
                ..cinch::core::TargetProfile::native()
            };
            common::drive_with_profile_opts(
                cinch::driver::Mode::Build,
                cinch::check::StageMode::Refresh,
                project.path(),
                "Main.cinch",
                Some(executable.clone()),
                &profile,
                &cinch::driver::DriverOptions { opt_level: Some(level), ..Default::default() },
            ).expect("build the comparison program");
            for poisoned in [false, true] {
                for input in ["", "x", "xx", "sixteen-byte-key"] {
                    let result = std::process::Command::new(&executable).arg(input)
                        .env("CINCH_RC_DEBUG", if poisoned { "1" } else { "0" })
                        .env("CINCH_HEAP_LIMIT", if poisoned { "256M" } else { "32M" })
                        .env("CINCH_STACK_SIZE", "8M")
                        .env("CINCH_LEAK_CHECK", "1")
                        .output().unwrap();
                    let diagnostics = String::from_utf8_lossy(&result.stderr);
                    if result.status.code() != Some(0) {
                        let retained = project.keep();
                        panic!("checked_counts={checked_counts}, O{level}, poisoned={poisoned}, input={input:?}: exit {:?}; {diagnostics}\nretained comparison: {}", result.status.code(), retained.display());
                    }
                    assert!(diagnostics.contains("cinch-leak: live=0 immortal=0 leaked=0"), "{diagnostics}");
                }
            }
        }
    }
}
