//! Function preconditions stay attached until checked at each call.
mod common;

#[test]
fn function_preconditions_use_actual_arguments_and_cannot_be_ignored() {
    let dir = common::project("function_preconditions", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.FunctionPreconditions.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "function-preconditions").expect("build the Cinch function precondition checker");
    let cases = [
        ("partial Boolean right conjunction proves subtraction", "safe :: Bool -> Nat -> Nat -> Nat\n  safe flag n m = if flag && n >= m then n - m else 0", 0),
        ("partial Boolean left conjunction retains subtraction", "safe :: Bool -> Nat -> Nat -> Nat\n  safe flag n m = if n >= m && flag then n - m else 0", 0),
        ("partial Boolean right disjunction false proves subtraction", "safe :: Bool -> Nat -> Nat -> Nat\n  safe flag n m = if flag || n < m then 0 else n - m", 0),
        ("partial Boolean nested conjunction proves call", "difference :: using (n >= m) => (n :: Nat) -> (m :: Nat) -> Nat\n  difference n m = n - m\n  safe :: Bool -> Bool -> Nat -> Nat -> Nat\n  safe first second n m = if first && (second && n >= m) then difference n m else 0", 0),
        ("partial Boolean false conjunction cannot prove subtraction", "bad :: Bool -> Nat -> Nat -> Nat\n  bad flag n m = if flag && n >= m then 0 else n - m", 4),
        ("partial Boolean true disjunction cannot prove subtraction", "bad :: Bool -> Nat -> Nat -> Nat\n  bad flag n m = if flag || n >= m then n - m else 0", 4),
        ("partial Boolean right fact keeps actual identities", "difference :: using (n >= m) => (n :: Nat) -> (m :: Nat) -> Nat\n  difference n m = n - m\n  bad :: Bool -> Nat -> Nat -> Nat\n  bad flag n m = if flag && n >= m then difference m n else 0", 4),
        ("partial Boolean fact cannot escape its branch", "difference :: using (n >= m) => (n :: Nat) -> (m :: Nat) -> Nat\n  difference n m = n - m\n  bad :: Bool -> Nat -> Nat -> Nat\n  bad flag n m = let ignored = if flag && n >= m then difference n m else 0 in difference n m", 4),
        ("partial Boolean nested disjunction false proves call", "difference :: using (n >= m) => (n :: Nat) -> (m :: Nat) -> Nat\n  difference n m = n - m\n  safe :: Bool -> Bool -> Nat -> Nat -> Nat\n  safe first second n m = if first || (second || n < m) then 0 else difference n m", 0),
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
        if name.starts_with("partial Boolean ") {
            let oracle = cinch::driver::check_sources(&source, "partial-boolean-branch-oracle.cinch");
            assert_eq!(oracle.is_ok(), wanted == 0, "Native partial Boolean oracle: {name}\n{oracle:?}");
        }
        std::fs::write(dir.join("source.cinch"), &source).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "signature-type-report.txt");
        assert_eq!(output.status.code(), Some(wanted), "{name}; project {}\n{source}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
        if wanted == 4 {
            let expected = if name == "missing precondition" || matches!(name, "partial Boolean false conjunction cannot prove subtraction" | "partial Boolean true disjunction cannot prove subtraction") { "ordering evidence" } else { "precondition is not established" };
            assert!(report.contains(expected), "{name}: {report}");
        }
    }
    // These source forms are independently accepted/refused by Native;
    // qualified contexts must not turn erased evidence into runtime data.
    for (name, body, wanted, diagnostic) in [
        ("unnamed effectful False domain", "use :: forall r. using (io :: Io r) => False ->{IO r} Int\n  use h = 0", 0, None),
        ("named effectful False domain", "use :: forall r. using (io :: Io r) => (h :: False) ->{IO r} Int\n  use h = 0", 0, None),
        ("generic effectful proof domain", "use :: forall r (p :: Prop). using (io :: Io r) => p ->{IO r} Dec p\n  use h = Yes h", 0, None),
        ("logical context retains proof", "use :: using (True) => True -> Dec True\n  use h = Yes h", 0, None),
        ("numeric context across erased argument", "use :: using (n > 0) => (n :: Nat) -> True -> Nat\n  use n h = n - 1", 0, None),
        ("erased argument cannot discharge numeric context", "use :: using (n > 0) => (n :: Nat) -> True -> Nat\n  use n h = n - 1\n  bad :: True -> Nat\n  bad h = use 0 h", 4, Some("precondition")),
        ("erased call retains IO", "use :: forall r. using (io :: Io r) => True ->{IO r} Int\n  use h = 0\n  bad :: forall r. using (io :: Io r) => True -> Int\n  bad h = use h", 4, Some("effect")),
        ("runtime integer cannot become qualified proof", "use :: forall r. using (io :: Io r) => True ->{IO r} Int\n  use h = 0\n  bad :: forall r. using (io :: Io r) => Int ->{IO r} Int\n  bad value = use value", 4, None),
        ("unrelated qualified proof cannot prove False", "use :: forall r. using (io :: Io r) => False ->{IO r} Int\n  use h = 0\n  bad :: forall r. using (io :: Io r) => True ->{IO r} Int\n  bad h = use h", 4, None),
        ("erased binder cannot become numeric index", "bad :: using (h > 0) => (h :: True) -> Nat\n  bad h = 0", 4, None),
        ("ordinary Boolean stays runtime data", "use :: forall r. using (io :: Io r) => True ->{IO r} Bool\n  use h = True", 0, None),
        ("ordinary Boolean cannot become qualified proof", "use :: forall r. using (io :: Io r) => True ->{IO r} Int\n  use h = 0\n  bad :: forall r. using (io :: Io r) => Bool ->{IO r} Int\n  bad value = use value", 4, None),
    ] {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        let oracle = cinch::driver::check_sources(&source, "qualified-proof-arrow-oracle.cinch");
        assert_eq!(oracle.is_ok(), wanted == 0, "Native oracle {name}: {oracle:?}");
        std::fs::write(dir.join("source.cinch"), &source).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "signature-type-report.txt");
        assert_eq!(output.status.code(), Some(wanted), "{name}; {source}\n{report}\n{}", String::from_utf8_lossy(&output.stderr));
        if let Some(message) = diagnostic { assert!(report.contains(message), "{name}: {report}"); }
        println!("qualified proof source case passed: {name}");
    }
    common::cleanup(&dir);
}
