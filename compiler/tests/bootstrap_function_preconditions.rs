//! Function preconditions stay attached until checked at each call.
mod common;

#[test]
fn function_preconditions_use_actual_arguments_and_cannot_be_ignored() {
    let dir = common::project("function_preconditions", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.FunctionPreconditions.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "function-preconditions").expect("build the Cinch function precondition checker");
    let cases = [
        ("declared assumption", "pred :: using (n > 0) => (n :: Nat) -> Nat\n  pred actual = actual - 1", 0),
        ("proved literal", "pred :: using (n > 0) => (n :: Nat) -> Nat\n  pred actual = actual - 1\n  answer :: Nat\n  answer = pred 1", 0),
        ("proved branch", "pred :: using (n > 0) => (n :: Nat) -> Nat\n  pred actual = actual - 1\n  safe :: Nat -> Nat\n  safe value = if value > 0 then pred value else 0", 0),
        ("callee argument name differs", "pred :: using (n > 0) => (n :: Nat) -> Nat\n  pred actual = actual - 1\n  safe :: using (i > 0) => (i :: Nat) -> Nat\n  safe value = pred value", 0),
        ("two arguments", "difference :: using (n >= m) => (n :: Nat) -> (m :: Nat) -> Nat\n  difference x y = x - y\n  answer :: Nat\n  answer = difference 7 3", 0),
        ("partial application", "difference :: using (n >= m) => (n :: Nat) -> (m :: Nat) -> Nat\n  difference x y = x - y\n  answer :: Nat\n  answer = let partial = difference 7 in partial 3", 0),
        ("conjunction", "subtract :: using (n > 0, n >= m) => (n :: Nat) -> (m :: Nat) -> Nat\n  subtract x y = x - y\n  answer :: Nat\n  answer = subtract 7 3", 0),
        ("conditional lambda", "pred :: using (n > 0) => (n :: Nat) -> Nat\n  pred = \\actual -> actual - 1\n  answer :: Nat\n  answer = pred 1", 0),
        ("false unused definition", "never :: using (False) => Nat\n  never = (0 :: Nat) - 1", 0),
        ("true nullary contract", "answer :: using (True) => Nat\n  answer = 3\n  use :: Nat\n  use = answer", 0),
        ("higher order preserves contract", "pred :: using (n > 0) => (n :: Nat) -> Nat\n  pred actual = actual - 1\n  identity :: forall a. a -> a\n  identity x = x\n  answer :: Nat\n  answer = let f = identity pred in f 1", 0),
        ("higher order rejects bad call", "pred :: using (n > 0) => (n :: Nat) -> Nat\n  pred actual = actual - 1\n  identity :: forall a. a -> a\n  identity x = x\n  bad :: Nat\n  bad = let f = identity pred in f 0", 4),
        ("recursive contract", "count :: using (n > 0) => (n :: Nat) -> Nat\n  count actual = if actual > 1 then count (actual - 1) else 0", 0),
        ("unsafe recursive contract", "count :: using (n > 0) => (n :: Nat) -> Nat\n  count actual = count (actual - 1)", 4),
        ("disjunction", "zero-or-positive :: using (n == 0 || n > 0) => (n :: Nat) -> Nat\n  zero-or-positive actual = actual\n  answer :: Nat\n  answer = zero-or-positive 3", 0),
        ("conditional effect arrow", "loop :: using (n > 0) => (n :: Nat) ->{Diverge} Nat\n  loop actual = actual - 1\n  safe :: Nat ->{Diverge} Nat\n  safe value = if value > 0 then loop value else 0", 0),
        ("zero argument", "pred :: using (n > 0) => (n :: Nat) -> Nat\n  pred actual = actual - 1\n  bad :: Nat\n  bad = pred 0", 4),
        ("ignored result", "pred :: using (n > 0) => (n :: Nat) -> Nat\n  pred actual = actual - 1\n  bad :: Nat\n  bad = let ignored = pred 0 in 0", 4),
        ("wrong identity", "pred :: using (n > 0) => (n :: Nat) -> Nat\n  pred actual = actual - 1\n  bad :: Nat -> Nat -> Nat\n  bad n m = if m > 0 then pred n else 0", 4),
        ("shadowed identity", "pred :: using (n > 0) => (n :: Nat) -> Nat\n  pred actual = actual - 1\n  bad :: Nat -> Nat\n  bad n = if n > 0 then (let n = (0 :: Nat) in pred n) else 0", 4),
        ("missing precondition", "pred :: (n :: Nat) -> Nat\n  pred actual = actual - 1", 4),
        ("false forced", "never :: using (False) => Nat\n  never = (0 :: Nat) - 1\n  bad :: Nat\n  bad = never", 4),
        ("false ignored", "never :: using (False) => Nat\n  never = (0 :: Nat) - 1\n  bad :: Nat\n  bad = let ignored = never in 0", 4),
        ("partial bad application", "difference :: using (n >= m) => (n :: Nat) -> (m :: Nat) -> Nat\n  difference x y = x - y\n  bad :: Nat\n  bad = let partial = difference 2 in partial 3", 4),
        ("guard cannot escape", "pred :: using (n > 0) => (n :: Nat) -> Nat\n  pred actual = actual - 1\n  bad :: Nat -> Nat\n  bad n = let ignored = if n > 0 then pred n else 0 in pred n", 4),
    ];
    for (name, body, wanted) in cases {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"), &source).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "signature-type-report.txt");
        assert_eq!(output.status.code(), Some(wanted), "{name}; project {}\n{source}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
        if wanted == 4 {
            let expected = if name == "missing precondition" { "ordering evidence" } else { "precondition is not established" };
            assert!(report.contains(expected), "{name}: {report}");
        }
    }
    common::cleanup(&dir);
}
