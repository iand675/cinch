//! Defining constructor contracts retain the source proof slot and zero payload.
mod common;

#[test]
fn decision_constructors_check_erasure_metadata_and_source_evidence() {
    let dir = common::project("source_constructors", &[
        ("Bootstrap.Parse.cinch", include_str!("../../bootstrap/Bootstrap.Parse.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Infer.cinch", include_str!("../../bootstrap/Bootstrap.Infer.cinch")),
        ("Bootstrap.Builtin.cinch", include_str!("../../bootstrap/Bootstrap.Builtin.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.SourceConstructors.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "source-constructors").expect("build source constructor checker");
    let yes = "wrap :: forall (p :: Prop). (h :: p) -> Dec p\n  wrap h = Yes h";
    let cases = [
        ("positive proof constructor", yes, true),
        ("negative proof constructor", "reject :: forall (p :: Prop). (h :: Not p) -> Dec p\n  reject h = No h", true),
        ("both decision alternatives", "extract :: forall (p :: Prop). Dec p -> Int\n  extract value = case value of Yes h -> 1; No h -> 0", true),
        ("vacuous assumption remains scoped", "wrap :: (h :: False) -> Dec False\n  wrap h = Yes h", true),
        ("ordinary source constructor may share spelling", "data Number where\n    Yes :: Int -> Number\n  extract :: Number -> Int\n  extract value = case value of Yes n -> n", true),
        ("literal cannot be positive evidence", "bad :: Dec False\n  bad = Yes 1", false),
        ("positive evidence is not negative evidence", "bad :: forall (p :: Prop). (h :: p) -> Dec p\n  bad h = No h", false),
        ("negative evidence is not positive evidence", "bad :: forall (p :: Prop). (h :: Not p) -> Dec p\n  bad h = Yes h", false),
        ("different propositions remain distinct", "bad :: forall (p :: Prop) (q :: Prop). (h :: p) -> Dec q\n  bad h = Yes h", false),
        ("decision must cover both alternatives", "extract :: forall (p :: Prop). Dec p -> Int\n  extract value = case value of Yes h -> 1", false),
        ("evidence cannot be inspected", "bad :: forall (p :: Prop). Dec p -> Int\n  bad value = case value of Yes True -> 1; No _ -> 0", false),
        ("proof binder is not a runtime result", "bad :: forall (p :: Prop). Dec p -> Int\n  bad value = case value of Yes h -> h; No _ -> 0", false),
    ];
    for (name, body, accepted) in cases {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"), &source).unwrap();
        std::fs::write(dir.join("mutation.txt"), "").unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "source-constructors-report.txt");
        assert_eq!(output.status.code(), Some(if accepted {0} else {4}), "case {name}; project {}\n{source}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
    }
    let source = format!("module Main where\n  {yes}\n  main :: Int\n  main = 0\n");
    std::fs::write(dir.join("source.cinch"), &source).unwrap();
    for field in ["name", "family", "parameter_kind", "runtime_abi", "tag", "arity", "runtime_arity", "scheme", "erased"] {
        std::fs::write(dir.join("mutation.txt"), field).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "source-constructors-report.txt");
        assert_eq!(output.status.code(), Some(4), "mutated {field}; project {}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
        assert!(report.contains("kernel proof/erasure contract"), "mutated {field}: {report}");
    }
    common::cleanup(&dir);
}
