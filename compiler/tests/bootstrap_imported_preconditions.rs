//! Real emitted contracts are consumed by the compiled Cinch frontend.
mod common;
use cinch::interface::Iface;
fn quote(text: &str) -> String {
    let mut out = String::from("\"");
    for ch in text.chars() {
        match ch { '"' => out.push_str("\\\""), '\\' => out.push_str("\\\\"), c if c <= '\u{1f}' => out.push_str(&format!("\\u{:04x}", c as u32)), c => out.push(c) }
    }
    out.push('"'); out
}


#[test]
fn imported_conditions_use_actual_caller_arguments() {
    let source = common::project("condition_source", &[
        ("A.cinch", "module A where\n  pred :: using (n > 0) => (n :: Nat) -> Nat\n  pred actual = actual - 1\n  signature Gate a where\n    gate :: a -> a\n  keep-with :: forall a. using (witness :: Gate a, n > 0) => (n :: Nat) -> a -> a\n  keep-with n value = value\n  difference :: using (left >= right) => (left :: Nat) -> (right :: Nat) -> Nat\n  difference a b = a - b\n"),
        ("Facade.cinch", "module Facade where\n  import A (pred as previous, difference, Gate, keep-with)\n  export previous, difference, Gate, keep-with\n"),
        ("Main.cinch", "module Main where\n  import Facade\n  main :: Int\n  main = 0\n"),
    ]);
    common::check_library_fixture_native(&source, "Main.cinch").expect("emit portable preconditions");
    let dir = common::project("condition_consumer", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Builtin.cinch", include_str!("../../bootstrap/Bootstrap.Builtin.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.ImportedPreconditions.cinch")),
    ]);
    let mut rows = Vec::new();
    for file in std::fs::read_dir(source.join("build")).unwrap() {
        let path = file.unwrap().path();
        if path.extension().is_some_and(|s| s == "cinchif") {
            let iface = Iface::from_json(&std::fs::read_to_string(&path).unwrap()).unwrap();
            if iface.module != "Main" {
                rows.push(format!("[{},{},{}]", quote(&iface.module), quote(&iface.iface_hash()), quote(&path.display().to_string())));
            }
        }
    }
    rows.sort();
    std::fs::write(dir.join("interfaces.json"), format!("{{\"schema\":1,\"interfaces\":[{}]}}", rows.join(","))).unwrap();
    let exe = common::build(&dir, "Main.cinch", "imported-preconditions").expect("build imported precondition consumer");
    let cases = [
        ("literal", "answer :: Nat\n  answer = previous 1", 0),
        ("branch", "safe :: Nat -> Nat\n  safe n = if n > 0 then previous n else 0", 0),
        ("forwarded condition", "safe :: using (x > 0) => (x :: Nat) -> Nat\n  safe actual = previous actual", 0),
        ("two arguments", "answer :: Nat\n  answer = difference 7 2", 0),
        ("partial application", "answer :: Nat\n  answer = let partial = difference 7 in partial 2", 0),
        ("mixed contexts", "forward :: forall a. using (witness :: Gate a) => a -> a\n  forward x = keep-with 1 x", 0),
        ("mixed contexts reject bad condition", "bad :: forall a. using (witness :: Gate a) => a -> a\n  bad x = keep-with 0 x", 4),
        ("rejected literal", "bad :: Nat\n  bad = previous 0", 4),
        ("ignored result", "bad :: Nat\n  bad = let ignored = previous 0 in 0", 4),
        ("wrong argument", "bad :: Nat -> Nat -> Nat\n  bad n m = if m > 0 then previous n else 0", 4),
        ("shadowed argument", "bad :: Nat -> Nat\n  bad n = if n > 0 then (let n = (0 :: Nat) in previous n) else 0", 4),
        ("reversed arguments", "bad :: Nat\n  bad = difference 2 7", 4),
        ("addition argument", "safe :: Nat -> Nat\n  safe n = previous (n + 1)", 0),
        ("addition guard", "safe :: Nat -> Nat\n  safe n = if n + 1 > 1 then previous n else 0", 0),
        ("multiplication argument", "safe :: Nat -> Nat\n  safe n = previous (n * 2 + 1)", 0),
        ("insufficient arithmetic guard", "bad :: Nat -> Nat\n  bad n = if n + 1 > 0 then previous n else 0", 4),
        ("integer subtraction argument", "step :: using (n > 0) => (n :: Int) -> Int\n  step actual = actual - 1\n  safe :: Int -> Int\n  safe n = if n > 1 then step (n - 1) else 0", 0),
        ("unsafe integer subtraction argument", "step :: using (n > 0) => (n :: Int) -> Int\n  step actual = actual - 1\n  bad :: Int -> Int\n  bad n = step (n - 1)", 4),
        ("shadowed comparison", "infix 3 >\n  (>) :: Nat -> Nat -> Bool\n  (>) x y = True\n  bad :: Nat -> Nat\n  bad n = if n > 0 then previous n else 0", 4),
        ("lexical ordering dictionary is opaque", "bad :: using (ordering :: Ord Nat) => Nat -> Nat\n  bad n = if n > 0 then previous n else 0", 4),
    ];
    for (name, body, wanted) in cases {
        let input = format!("module Main where\n  import Facade (previous, difference, Gate, keep-with)\n  {body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"), &input).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "imported-report.txt");
        assert_eq!(output.status.code(), Some(wanted), "{name}: {input}\n{report}\n{}", String::from_utf8_lossy(&output.stderr));
        if wanted == 4 { assert!(report.contains("precondition is not established"), "{name}: {report}"); }
    }
    common::cleanup(&source); common::cleanup(&dir);
}
