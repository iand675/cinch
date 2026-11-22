//! Source proof binders and applications are checked by compiled Cinch.
mod common;

#[test]
fn source_proofs_stay_erased_scoped_and_distinct_from_runtime_values() {
    let dir = common::project("source_proofs", &[
        ("Bootstrap.Parse.cinch", include_str!("../../bootstrap/Bootstrap.Parse.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Infer.cinch", include_str!("../../bootstrap/Bootstrap.Infer.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.SourceProofs.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "source-proofs").expect("build source proof checker");
    let take = "take :: forall (p :: Prop). (h :: p) -> Int\n  take h = 1\n  ";
    let cases = [
        ("named polymorphic proof", "", "take :: forall (p :: Prop). (h :: p) -> Int\n  take h = 1", true),
        ("unnamed proof arrow", "", "take :: forall (p :: Prop). p -> Int\n  take h = 1", true),
        ("wildcard erased evidence", "", "take :: forall (p :: Prop). (h :: p) -> Int\n  take _ = 1", true),
        ("forward actual proof", take, "forward :: forall (p :: Prop). (h :: p) -> Int\n  forward evidence = take evidence", true),
        ("proof lambda", take, "forward :: forall (p :: Prop). (h :: p) -> Int\n  forward = \\evidence -> take evidence", true),
        ("explicit truth domain", "", "take :: (h :: True) -> Int\n  take h = 1", true),
        ("erased call retains declared divergence", "take :: forall (p :: Prop). (h :: p) ->{Diverge} Int\n  take h = take h\n  ", "forward :: forall (p :: Prop). (h :: p) ->{Diverge} Int\n  forward h = take h", true),
        ("erased call cannot hide divergence", "take :: forall (p :: Prop). (h :: p) ->{Diverge} Int\n  take h = take h\n  ", "bad :: forall (p :: Prop). (h :: p) -> Int\n  bad h = take h", false),
        ("different proposition", "not-p :: forall (p :: Prop). (h :: Not p) -> Int\n  not-p h = 1\n  ", "bad :: forall (p :: Prop). (h :: p) -> Int\n  bad h = not-p h", false),
        ("literal is not proof", take, "bad :: Int\n  bad = take 1", false),
        ("runtime local is not proof", take, "bad :: Int -> Int\n  bad x = take x", false),
        ("computed operand cannot be erased", take, "bad :: forall (p :: Prop). (h :: p) -> Int\n  bad h = take (let discarded = 1 in h)", false),
        ("proof cannot be an integer", "", "bad :: forall (p :: Prop). (h :: p) -> Int\n  bad h = h", false),
        ("proof cannot be condition", "", "bad :: forall (p :: Prop). (h :: p) -> Int\n  bad h = if h then 1 else 0", false),
        ("proof identity cannot be inspected", "", "bad :: forall (p :: Prop). (h :: p) -> Int\n  bad True = 1", false),
        ("runtime shadow removes proof access", take, "bad :: forall (p :: Prop). (h :: p) -> Int\n  bad h = let h = 1 in take h", false),
        ("proof result needs total proof language", "", "bad :: forall (p :: Prop). (h :: p) -> p\n  bad h = h", false),
        ("proof cannot supply runtime index", "data At (n :: Int) where\n    Mark :: At n\n  ", "bad :: forall (p :: Prop). (h :: p) -> At h\n  bad h = Mark", false),
    ];
    for (name, prefix, body, accepted) in cases {
        let source = format!("module Main where\n  {prefix}{body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"), &source).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "source-proofs-report.txt");
        assert_eq!(output.status.code(), Some(if accepted {0} else {4}),
            "case {name}; project {}\n{source}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
    }
    common::cleanup(&dir);
}
