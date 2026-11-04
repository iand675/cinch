//! Source Pi checking and actual term/index association, with runtime inputs.
mod common;
#[test]
fn source_dependent_functions_use_checked_argument_values() {
    let dir = common::project("source_pi", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Parse.cinch", include_str!("../../bootstrap/Bootstrap.Parse.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Bootstrap.Infer.cinch", include_str!("../../bootstrap/Bootstrap.Infer.cinch")),
        ("Bootstrap.Nominal.cinch", include_str!("../../bootstrap/Bootstrap.Nominal.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.SourcePi.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "source-pi").expect("build source Pi checker");
    let nat = "  data At (n :: Nat) where\n    Mark :: At n\n  make :: (n :: Nat) -> At n\n  make actual = Mark\n";
    let int = "  data AtI (n :: Int) where\n    MarkI :: AtI n\n  make-i :: (n :: Int) -> AtI n\n  make-i actual = MarkI\n";
    let cases = [
        ("nondependent named binder", "", "identity :: (n :: Nat) -> Nat\n  identity value = value", None),
        ("generic carrier avoids quantifier collision", "", "identity :: forall a. (x :: a) -> a\n  identity value = value", None),
        ("actual argument identity", nat, "forward :: (n :: Nat) -> At n\n  forward renamed = make renamed", None),
        ("numeric application", nat, "forget :: forall (n :: Nat). At n -> Int\n  forget Mark = 0\n  answer :: Int\n  answer = forget (make 3)", None),
        ("large numeric application", nat, "forget :: forall (n :: Nat). At n -> Int\n  forget Mark = 0\n  answer :: Int\n  answer = forget (make 340282366920938463463374607431768211456)", None),
        ("hexadecimal mathematical literal", nat, "forget :: forall (n :: Nat). At n -> Int\n  forget Mark = 0\n  answer :: Int\n  answer = forget (make 0x100)", None),
        ("integer negative argument", int, "forget :: forall (n :: Int). AtI n -> Int\n  forget MarkI = 0\n  answer :: Int\n  answer = forget (make-i (-3))", None),
        ("lambda against Pi", nat, "forward :: (n :: Nat) -> At n\n  forward = \\renamed -> make renamed", None),
        ("strict let literal beta", nat, "let-result :: At 3\n  let-result = let index = (3 :: Nat) in make index", None),
        ("strict let outer argument alias", nat, "forward :: (n :: Nat) -> At n\n  forward renamed = let alias = renamed in make alias", None),
        ("strict let alias chain", nat, "forward :: (n :: Nat) -> At n\n  forward renamed = let first = renamed in let second = first in make second", None),
        ("strict let mathematical beta", nat, "answer :: At 3\n  answer = let index = ((1 :: Nat) + 2) in make index", None),
        ("strict let type annotation alias", nat, "forward :: (n :: Nat) -> At n\n  forward renamed = let alias = renamed in (Mark :: At alias)", None),
        ("generic function-valued carrier", "  data AtFn (f :: Nat -> Nat) where\n    MarkFn :: AtFn f\n", "build :: (f :: Nat -> Nat) -> AtFn f\n  build function = MarkFn\n  forward :: (g :: Nat -> Nat) -> AtFn g\n  forward renamed = build renamed", None),
        ("generic Bool carrier", "  data Flag (b :: Bool) where\n    Flagged :: Flag b\n", "build :: (b :: Bool) -> Flag b\n  build actual = Flagged\n  forward :: (b :: Bool) -> Flag b\n  forward renamed = build renamed", None),
        ("opaque computation retains a private witness", nat, "forget :: forall (n :: Nat). At n -> Nat\n  forget Mark = 7\n  use :: (Nat -> Nat) -> Nat -> Nat\n  use compute n = forget (make (compute n))", None),
        ("opaque dependent result projects to its carrier", "", "keep :: (n :: Nat) -> refined (out == n) Nat\n  keep n = n\n  use :: (Nat -> Nat) -> Nat -> Nat\n  use compute n = keep (compute n)", None),
        ("opaque effectful argument retains evaluation effects", "", "keep :: (n :: Nat) -> refined (out == n) Nat\n  keep n = n\n  use :: forall r. (Unit ->{IO r} Nat) -> Unit ->{IO r} Nat\n  use compute u = keep (compute u)", None),
        ("opaque argument cannot erase evaluation effects", "", "keep :: (n :: Nat) -> refined (out == n) Nat\n  keep n = n\n  bad :: forall r. (Unit ->{IO r} Nat) -> Unit -> Nat\n  bad compute u = keep (compute u)", Some("effect")),
        ("carrier mismatch", nat, "bad :: Int -> At 0\n  bad value = make value", Some("type mismatch")),
        ("same name does not repair wrong identity", nat, "bad :: (n :: Nat) -> At n\n  bad n = let n = (2 :: Nat) in make n", Some("type mismatch")),
        ("strict let wrong constant", nat, "bad :: At 4\n  bad = let index = (3 :: Nat) in make index", Some("type mismatch")),
        ("strict let cannot reflect unknown call", int, "loop :: Int -> Int\n  loop x = loop x\n  bad :: AtI 0\n  bad = let index = loop 0 in make-i index", Some("type mismatch")),
        ("Nat arithmetic retains its carrier", nat, "forward :: (n :: Nat) -> At (n + 2)\n  forward value = let index = value + 2 in make index", None),
        ("Nat subtraction requires ordering evidence", nat, "bad :: (n :: Nat) -> Nat\n  bad value = value - 1", Some("ordering evidence")),
        ("symbolic addition is not conversion", nat, "bad :: (n :: Nat) -> At (n + 0)\n  bad value = make value", Some("type mismatch")),
        ("undefined subtraction in type", nat, "bad :: At (0 - 1) -> Int\n  bad value = 0", Some("source index")),
        ("no unchecked symbolic subtraction", nat, "bad :: (n :: Nat) -> At (n - 1)\n  bad value = Mark", Some("source index")),
        ("argument type is not an index value", nat, "bad :: (n :: Bool) -> At n\n  bad value = Mark", Some("type mismatch")),
        ("type is not its own universe", "", "bad :: (a :: Type) -> a -> a\n  bad value x = x", Some("unknown type constructor")),
        // Runtime witnesses remain opaque: the private value cannot unify
        // with zero, including when unchecked evaluation is multiplied by zero.
        ("unknown calls cannot be reflected", int, "loop :: Int -> Int\n  loop x = loop x\n  bad :: Int -> AtI 0\n  bad x = make-i (loop x)", Some("type mismatch")),
        ("zero cannot erase a partial argument", int, "loop :: Int -> Int\n  loop x = loop x\n  bad :: Int -> AtI 0\n  bad x = make-i (0 * loop x)", Some("type mismatch")),
        ("dependent declaration kind cannot capture caller binder", "  data Dep a (x :: a) where\n    Pack :: Dep a x\n", "bad :: forall a (x :: a). Dep Int x -> Int\n  bad value = 0", Some("binder-aware kind application")),
        ("ghost forall index has no runtime binding", "", "bad :: forall (n :: Nat). Nat\n  bad = n", Some("unbound variable")),
    ];
    for (name, prefix, body, refusal) in cases {
        let source = format!("module Main where\n{prefix}  {body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"), &source).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "source-pi-report.txt");
        assert_eq!(output.status.code(), Some(if refusal.is_some() {4} else {0}), "case {name}; project {}\n{source}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
        if let Some(message) = refusal { assert!(report.contains(message), "case {name} expected {message:?}: {report}"); }
    }
    common::cleanup(&dir);
}
