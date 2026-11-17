//! Wide generic arrays use owned element boxes while source list, Option and
//! tuple layouts retain inline i128 fields. Packed-wide arrays are distinct.

mod common;

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{DriverOptions, Mode};

/// Compile checked-RC programs at both consumer optimization levels and require
/// complete values and balanced ownership under ordinary and poisoned heaps.
#[test]
fn wide_arrays_tensors_and_vector_copies_preserve_full_payloads() {
    let dir = common::project("wide_array_boundaries", &[
        ("Data.Array.cinch", include_str!("../prelude/Data/Array.cinch")),
        ("Data.Tensor.cinch", include_str!("../prelude/Data/Tensor.cinch")),
        ("Data.Columnar.cinch", include_str!("../prelude/Data/Columnar.cinch")),
        ("Data.Mutable.Vec.cinch", include_str!("../prelude/Data/Mutable/Vec.cinch")),
        ("Main.cinch", include_str!("fixtures/wide-array-boundaries.cinch")),
    ]);
    for level in [0, 2] {
        let path = dir.join("Main.cinch");
        let exe = dir.join(format!("wide-arrays-o{level}"));
        let output = exe.clone();
        let result = std::thread::Builder::new().stack_size(64 * 1024 * 1024)
            .spawn(move || cinch::driver::driver_with_profile_opts(
                Mode::Build, StageMode::Refresh, &path, Some(&output),
                &TargetProfile { reclaim: true, rc_debug: true, ..TargetProfile::native() },
                &DriverOptions { opt_level: Some(level), ..DriverOptions::default() },
            )).unwrap().join().unwrap();
        assert_eq!(result.unwrap_or_else(|e| panic!("{e}; source: {}", dir.display())), 0);
        for (debug, heap) in [("0", "32M"), ("1", "256M")] {
            let result = std::process::Command::new(&exe)
                .env("CINCH_RC_DEBUG", debug).env("CINCH_HEAP_LIMIT", heap)
                .env("CINCH_STACK_SIZE", "8M").env("CINCH_LEAK_CHECK", "1")
                .output().unwrap();
            let err = String::from_utf8_lossy(&result.stderr);
            assert_eq!(result.status.code(), Some(0), "O{level}, debug={debug}: {err}; {}", dir.display());
            assert!(err.split_whitespace().any(|w| w == "leaked=0"), "{err}");
        }
    }
    common::cleanup(&dir);
}

/// Direct wide tuple fields need a columnar layout bridge; refusing the first
/// zip prevents the word-only runtime from constructing a malformed source row.
#[test]
fn columnar_zip_refuses_direct_wide_tuple_fields_by_name() {
    for count in [0, 1] {
        for wide_left in [false, true] {
            let wide = format!("columnar-column (array-replicate {count} max-bound-u128)");
            let ordinary = format!("columnar-column (array-replicate {count} True)");
            let (left, right) = if wide_left { (&wide, &ordinary) } else { (&ordinary, &wide) };
            let source = format!(r#"module Main where
  import Data.Array
  import Data.Columnar
  import Data.Wide
  -- | Keep both columns reachable so their actual row layout must be emitted.
  main :: Int
  main = case columnar-zip ({left}) ({right}) of
    None -> 1
    Some table -> if columnar-rows table == {count} then 0 else 2
"#);
            let dir = common::project("wide_columnar_refusal", &[
                ("Data.Array.cinch", include_str!("../prelude/Data/Array.cinch")),
                ("Data.Columnar.cinch", include_str!("../prelude/Data/Columnar.cinch")),
                ("Main.cinch", &source),
            ]);
            let result = common::drive(Mode::EmitIr, &dir, "Main.cinch", Some(dir.join("refused.ll")));
            let message = result.expect_err("wide columnar zip must refuse before LLVM");
            assert!(message.contains("columnarZip") && message.contains("wide tuple layout bridge"), "{message}");
            common::cleanup(&dir);
        }
    }
}
