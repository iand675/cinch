//! Defining constructor contracts retain the source proof slot and zero payload.
mod common;

#[test]
fn source_projection_preserves_carriers_and_kernel_authority() {
    let dir = common::project("source_projection", &[
        ("Bootstrap.Parse.cinch", include_str!("../../bootstrap/Bootstrap.Parse.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Infer.cinch", include_str!("../../bootstrap/Bootstrap.Infer.cinch")),
        ("Bootstrap.Builtin.cinch", include_str!("../../bootstrap/Bootstrap.Builtin.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.SourceProjection.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "source-projection").expect("build source constructor checker");
    let cases = [
        ("projected scalar is not callable", "type Positive = refine (n :: Int) with n > 0\n  project :: Positive -> Int\n  project n = unrefine n 42", 4, ""),
        ("projection under ordinary closure", "type Positive = refine (n :: Int) with n > 0\n  project :: Unit -> Positive -> Int\n  project unit = \\n -> unrefine n", 0, ""),
        ("vacuous named predicate is refused", "type Callable = refine (f :: Int -> Int) with True\n  project :: Callable -> Int\n  project f = unrefine f 42", 4, "predicate must mention its binder"),
        ("named comparison", "type Positive = refine (n :: Int) with n > 0\n  project :: Positive -> Int\n  project n = unrefine n", 0, ""),
        ("Nat carrier", "type Positive = refine (n :: Nat) with n > 0\n  project :: Positive -> Nat\n  project n = unrefine n", 0, ""),
        ("strict let", "type Positive = refine (n :: Int) with n > 0\n  project :: Positive -> Int\n  project n = let alias = n in unrefine alias", 0, ""),
        ("wrong result carrier", "type Positive = refine (n :: Nat) with n > 0\n  project :: Positive -> Bool\n  project n = unrefine n", 4, ""),
        ("plain input", "project :: Int -> Int\n  project n = unrefine n", 4, "unrefine projects a refinement"),
        ("plain collection", "data Collection a where\n    EmptyCollection :: Collection a\n  project :: Collection Int -> Collection Int\n  project n = unrefine n", 4, "unrefine projects a refinement"),
        ("missing argument", "project :: Int -> Int\n  project = unrefine", 4, "not a first-class function"),
    ];
    for (name, body, expected, diagnostic) in cases {
        std::fs::write(dir.join("source.cinch"), format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n")).unwrap();
        std::fs::write(dir.join("mutation.txt"), "").unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "source-projection-report.txt");
        assert_eq!(output.status.code(), Some(expected), "{name}; project {}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
        assert!(report.contains(diagnostic), "{name}: {report}");
        assert!(!report.contains("bootstrap.resolve:"), "projection must resolve before checking its carrier: {report}");
    }
    let valid = "module Main where\n  type Positive = refine (n :: Int) with n > 0\n  project :: Positive -> Int\n  project n = unrefine n\n  main :: Int\n  main = 0\n";
    for field in ["name", "operation", "runtime_abi", "first_class", "arity", "variadic"] {
        std::fs::write(dir.join("source.cinch"), valid).unwrap();
        std::fs::write(dir.join("mutation.txt"), field).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "source-projection-report.txt");
        assert_eq!(output.status.code(), Some(4), "mutated {field}: {report}");
        assert!(report.contains("kernel contract"), "mutated {field}: {report}");
    }
    for body in ["unrefine :: Int -> Int\n  unrefine n = n\n  project :: Int -> Int\n  project n = unrefine n", "project :: (Int -> Int) -> Int\n  project unrefine = unrefine 7"] {
        std::fs::write(dir.join("source.cinch"), format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n")).unwrap();
        std::fs::write(dir.join("mutation.txt"), "ordinary").unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "ordinary name must retain ordinary function behavior: {}", common::read_output(&dir, "source-projection-report.txt"));
    }
}
