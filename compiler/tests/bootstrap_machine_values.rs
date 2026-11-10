//! Machine values retain their exact carrier, range and opaque arithmetic.
mod common;
#[test]
fn type_and_core_recheck_machine_ranges_and_actual_typed_scope() {
    let dir = common::project("machine_value_kernel", &[
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.MachineValues.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "machine-values").expect("build real machine-value Type/Core checker");
    for size in [0, 1, 17, 129, 4096] {
        std::fs::write(dir.join("nonce.txt"), vec![b'x'; size]).unwrap();
        let result = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(result.status.code(), Some(0), "machine-value kernel check {size}; project {}\n{}", dir.display(), String::from_utf8_lossy(&result.stderr));
    }
    common::cleanup(&dir);
}
#[test]
fn source_machine_literals_and_refinements_keep_range_and_carrier() {
    let dir = common::project("machine_value_source", &[
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.SourceRefinements.cinch")),
    ]);
    let exe = common::build_refinement_checker(&dir, "machine-source").expect("build actual Cinch machine-value source checker");
    for (carrier, low, high, below, above) in [
        ("I8", "-128", "127", "-129", "128"),
        ("I16", "-32768", "32767", "-32769", "32768"),
        ("I32", "-2147483648", "2147483647", "-2147483649", "2147483648"),
        ("I64", "-9223372036854775808", "9223372036854775807", "-9223372036854775809", "9223372036854775808"),
        ("U8", "0", "255", "-1", "256"),
        ("U16", "0", "65535", "-1", "65536"),
        ("U32", "0", "4294967295", "-1", "4294967296"),
        ("U64", "0", "18446744073709551615", "-1", "18446744073709551616"),
    ] {
        for (literal, status) in [(low, 0), (high, 0), (below, 4), (above, 4)] {
            let source = format!("module Main where\n  value :: {carrier}\n  value = {literal}\n  main :: Int\n  main = 0\n");
            std::fs::write(dir.join("source.cinch"), &source).unwrap();
            let result = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
            assert_eq!(result.status.code(), Some(status), "{carrier} literal {literal}; project {}\n{}", dir.display(), common::read_output(&dir, "refinement-report.txt"));
        }
    }
    for (name, body, status) in [
        ("machine literal refinement", "value :: refined (out >= 0) I64\n  value = 0", 0),
        ("machine negative literal refinement", "value :: refined (out < 0) I8\n  value = -128", 0),
        ("false literal refinement", "value :: refined (out > 0) I64\n  value = 0", 4),
        ("project machine refinement", "keep :: refined (n >= 0) I64 -> I64\n  keep n = n", 0),
        ("weaken machine refinement", "keep :: refined (n >= 1) I64 -> refined (out >= 0) I64\n  keep n = n", 0),
        ("machine value needs refinement evidence", "bad :: I64 -> refined (out > 0) I64\n  bad n = n", 4),
        ("unsigned carrier supplies nonnegative bound", "keep :: U8 -> refined (out >= 0) U8\n  keep n = n", 0),
        ("signed carrier supplies upper bound", "keep :: I8 -> refined (out <= 127) I8\n  keep n = n", 0),
        ("carrier bound cannot strengthen", "bad :: I8 -> refined (out > -128) I8\n  bad n = n", 4),
        ("case keeps expected machine carrier", "choose :: Bool -> I64\n  choose b = case b of\n    True -> -1\n    False -> 1", 0),
        ("case checks machine literal range", "bad :: Bool -> I8\n  bad b = case b of\n    True -> 128\n    False -> 0", 4),
        ("bounded literal pattern", "choose :: I8 -> Nat\n  choose 127 = 1\n  choose _ = 0", 0),
        ("out of range literal pattern", "bad :: I8 -> Nat\n  bad 128 = 1\n  bad _ = 0", 4),
        ("machine arithmetic stays opaque", "bad :: (n :: I64) -> refined (out > n + 1) I64\n  bad n = n", 4),
    ] {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"), &source).unwrap();
        let result = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(result.status.code(), Some(status), "{name}; project {}\n{source}\n{}", dir.display(), common::read_output(&dir, "refinement-report.txt"));
    }
    common::cleanup(&dir);
}
