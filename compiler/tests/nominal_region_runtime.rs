//! Erased nominal identities preserve managed, scalar and wide payload ownership.

mod common;

/// Exercise actual construction and mutation with one erased identity and one payload class.
const SOURCE: &str = r#"-- | Execute nominal runtime identities with distinct payload representations.
module Main where
  import Data.Bytes
  import Data.Machine
  import Data.Wide
  import Data.Mutable.Var
  -- | Retain a mutable cell while erasing its runtime identity from value arguments.
  data Box (r :: Region) a where
    -- | Own the cell without storing a representation argument for its identity.
    Boxed :: Var r a -> Box r a
  -- | Allocate a cell under the explicitly bound runtime identity.
  wrap :: forall (r :: Region) a. a ->{Mut r} Box r a
  wrap value = Boxed (var-new value)
  -- | Exchange payload ownership through the nominal constructor.
  exchange :: forall (r :: Region) a. Box r a -> a ->{Mut r} a
  exchange boxed value = case boxed of Boxed cell -> var-swap cell value
  -- | Acquire the current payload through the same erased identity.
  observe :: forall (r :: Region) a. Box r a ->{Mut r} a
  observe boxed = case boxed of Boxed cell -> var-read cell
  -- | Fix the immediate payload representation independently from numeric defaulting.
  machine-box :: forall (r :: Region). I64 ->{Mut r} Box r I64
  machine-box value = wrap value
  -- | Fix a two-word payload representation independently from numeric defaulting.
  wide-box :: forall (r :: Region). U128 ->{Mut r} Box r U128
  wide-box value = wrap value
  -- | Observe moved managed owners and both scalar widths before releasing every cell.
  main :: Unit ->{IO r, Mut r} Int
  main unit = with (io = newIO ()) in
    let original = bytes-new 31 in
    let replacement = bytes-new 47 in
    let managed = wrap original in
    let previous = exchange managed replacement in
    let current = observe managed in
    let immediate = machine-box 42 in
    let old-immediate = exchange immediate 43 in
    let wide = wide-box 340282366920938463463374607431768211450 in
    let old-wide = exchange wide 340282366920938463463374607431768211451 in
    if bytes-len previous == 31 && bytes-len current == 47
      && old-immediate == 42 && observe immediate == 43
      && old-wide == 340282366920938463463374607431768211450
      && observe wide == 340282366920938463463374607431768211451
    then 0 else 1
"#;

/// Both optimization levels preserve actual payloads with checked RC and poisoned reclamation.
#[test]
fn nominal_region_payload_classes_execute_without_identity_arguments() {
    for level in [0, 2] {
        let directory = tempfile::tempdir().unwrap();
        for (name, source) in [
            ("Main.cinch", SOURCE),
            ("Data.Mutable.Var.cinch", include_str!("../prelude/Data/Mutable/Var.cinch")),
            ("Data.Wide.cinch", include_str!("../prelude/Data/Wide.cinch")),
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
