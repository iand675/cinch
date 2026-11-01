//! Refinement source is supplied after compilation; checked Core retains evidence.
mod common;

#[test]
fn source_refinements_check_actual_values_and_preserve_effects() {
    let dir = common::project("source_refinements", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.SourceRefinements.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "source-refinements").expect("build the Cinch source refinement checker");
    let cases = [
        ("named refined parameter dependent result", "keep :: (n :: refined (r > 0) Nat) -> refined (out == n) Nat\n  keep n = n", 0, ""),
        ("named refined parameter dependent call", "keep :: (n :: refined (r > 0) Nat) -> refined (out == n) Nat\n  keep n = n\n  use :: (m :: refined (r > 0) Nat) -> refined (out == m) Nat\n  use m = keep m", 0, ""),
        ("dependent arithmetic retains projection", "copy :: (n :: Nat) -> refined (out == n) Nat\n  copy n = n\n  use :: (m :: refined (r > 0) Nat) -> refined (out == m + 1) Nat\n  use m = copy (m + 1)", 0, ""),
        ("dependent refined argument cannot borrow another identity", "bad :: (n :: refined (r > 0) Nat) -> (m :: refined (r > 0) Nat) -> refined (out == n) Nat\n  bad n m = m", 4, "refinement predicate is not established"),
        ("mixed Nat Int contract", "keep :: using (n < i) => (n :: Nat) -> (i :: Int) -> refined (out > n) Int\n  keep n i = i", 0, ""),
        ("mixed Nat Int contract cannot strengthen", "bad :: using (n <= i) => (n :: Nat) -> (i :: Int) -> refined (out > n) Int\n  bad n i = i", 4, "refinement predicate is not established"),
        ("fixed integer comparison contract", "keep :: using (n > 0) => (n :: I64) -> refined (out > 0) I64\n  keep n = n", 0, ""),
        ("fixed integer needs evidence", "bad :: (n :: I64) -> refined (out > 0) I64\n  bad n = n", 4, "refinement predicate is not established"),
        ("wrapping arithmetic remains opaque", "bad :: (n :: I64) -> refined (out > n + 1) I64\n  bad n = n", 4, "machine arithmetic cannot enter mathematical index conversion"),
        ("generic result proof", "keep :: forall a. a -> refined (True) a\n  keep x = x", 0, ""),
        ("generic refined identity", "keep :: forall a. refined (True) a -> refined (True) a\n  keep x = x", 0, ""),
        ("non-comparison point needs explicit projection", "bad :: forall a. refined (True) a -> a\n  bad x = x", 4, "type mismatch"),
        ("literal", "one :: refined (r > 0) Nat\n  one = 1", 0, ""),
        ("integer literal", "one :: refined (r < 0) Int\n  one = -1", 0, ""),
        ("global preserves result", "one :: refined (r > 0) Nat\n  one = 1\n  use :: refined (out > 0) Nat\n  use = one", 0, ""),
        ("global numeric projection", "one :: refined (r > 0) Nat\n  one = 1\n  use :: Nat\n  use = one", 0, ""),
        ("positive successor", "next :: Nat -> refined (r > 0) Nat\n  next n = n + 1", 0, ""),
        ("branch evidence", "positive :: Nat -> refined (r > 0) Nat\n  positive n = if n > 0 then n else 1", 0, ""),
        ("case branches", "choose :: Bool -> refined (r > 0) Nat\n  choose flag = case flag of True -> 1; False -> 2", 0, ""),
        ("guard evidence", "positive :: Int -> refined (r > 0) Int\n  positive n | n > 0 = n\n  positive _ = 1", 0, ""),
        ("let within expected result", "positive :: Nat -> refined (r > 0) Nat\n  positive n = let one = (1 :: Nat) in one", 0, ""),
        ("local preserves refinement", "keep :: refined (r > 0) Nat -> refined (out > 0) Nat\n  keep n = n", 0, ""),
        ("comparison weakening", "weaken :: refined (r > 0) Nat -> refined (out >= 0) Nat\n  weaken n = n", 0, ""),
        ("numeric parameter projection", "plain :: refined (r > 0) Nat -> Nat\n  plain n = n", 0, ""),
        ("refined arithmetic", "next :: Nat -> refined (r > 0) Nat\n  next n = n + 1\n  previous :: Nat -> Nat\n  previous n = let value = next n in value - 1", 0, ""),
        ("refined comparison", "positive :: refined (r > 0) Nat -> Bool\n  positive n = n > 0", 0, ""),
        ("refined equality", "zero :: refined (r > 0) Nat -> Bool\n  zero n = n == 0", 0, ""),
        ("higher order result", "apply :: (Nat -> refined (r > 0) Nat) -> Nat -> refined (out > 0) Nat\n  apply f n = f n", 0, ""),
        ("declared assumption", "keep :: using (n > 0) => (n :: Nat) -> refined (r > 0) Nat\n  keep n = n", 0, ""),
        ("dependent result", "next :: (n :: Nat) -> refined (r > n) Nat\n  next n = n + 1", 0, ""),
        ("effectful caller retains row", "next :: Unit ->{Diverge} refined (r > 0) Nat\n  next unit = 1\n  use :: Unit ->{Diverge} Nat\n  use unit = next unit", 0, ""),
        ("own operator gives no mathematical fact", "infix 3 >\n  (>) :: Nat -> Nat -> Bool\n  (>) x y = True\n  bad :: Nat -> refined (r > 0) Nat\n  bad n = if n > 0 then n else 1", 4, "refinement predicate is not established"),
        ("global name is not a result binder", "zeroGlobal :: Nat\n  zeroGlobal = 0\n  bad :: refined (zeroGlobal > 0) Nat\n  bad = 1", 4, "source index has no checked local binding"),
        ("argument name is not a result binder", "bad :: (r :: Nat) -> refined (r > 0) Nat\n  bad n = 1", 4, "refinement predicate is not established"),
        ("scalar annotation enters typed checking", "one :: Int\n  one = (1 :: refined (r > 0) Int)", 0, ""),
        ("annotation cannot erase failure", "bad :: Int\n  bad = (0 :: refined (r > 0) Int)", 4, "refinement predicate is not established"),
        ("ignored annotation stays strict", "bad :: Int\n  bad = let ignored = (0 :: refined (r > 0) Int) in 0", 4, "refinement predicate is not established"),
        ("zero rejected", "bad :: refined (r > 0) Nat\n  bad = 0", 4, "refinement predicate is not established"),
        ("negative rejected", "bad :: refined (r > 0) Int\n  bad = -1", 4, "refinement predicate is not established"),
        ("unproved parameter", "bad :: Nat -> refined (r > 0) Nat\n  bad n = n", 4, "refinement predicate is not established"),
        ("bad branch", "bad :: Nat -> refined (r > 0) Nat\n  bad n = if n > 0 then n else 0", 4, "refinement predicate is not established"),
        ("wrong identity", "bad :: Nat -> Nat -> refined (r > 0) Nat\n  bad n m = if m > 0 then n else 1", 4, "refinement predicate is not established"),
        ("shadowed identity", "bad :: Nat -> refined (r > 0) Nat\n  bad n = if n > 0 then (let n = (0 :: Nat) in n) else 1", 4, "refinement predicate is not established"),
        ("branch cannot escape", "bad :: Nat -> refined (r > 0) Nat\n  bad n = let ignored = if n > 0 then n else 1 in n", 4, "refinement predicate is not established"),
        ("unused false definition", "bad :: refined (r > 0) Nat\n  bad = 0\n  safe :: Nat\n  safe = 1", 4, "refinement predicate is not established"),
        ("strict argument", "need :: refined (r > 0) Nat -> Nat\n  need n = n\n  bad :: Nat\n  bad = let ignored = need 0 in 0", 4, "refinement predicate is not established"),
        ("weak cannot strengthen", "bad :: refined (r >= 0) Nat -> refined (out > 0) Nat\n  bad n = n", 4, "refinement predicate is not established"),
        ("float is opaque", "bad :: F64 -> refined (r > 0) F64\n  bad n = n", 4, "proof predicate needs checked mathematical carriers"),
        ("ambiguous result names", "bad :: refined (r > 0 && other > 1) Nat\n  bad = 2", 4, "more than one unbound value name"),
        ("unknown call stays opaque", "bad :: refined (measure r > 0) Nat\n  bad = 1", 4, "index expression needs checked scope, purity and totality"),
        ("effect cannot erase", "next :: Unit ->{Diverge} refined (r > 0) Nat\n  next unit = 1\n  bad :: Unit -> Nat\n  bad unit = next unit", 4, "effect"),
    ];
    for (name, body, wanted, diagnostic) in cases {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"), &source).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "refinement-report.txt");
        assert_eq!(output.status.code(), Some(wanted), "{name}; project {}\n{source}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
        if wanted != 0 { assert!(report.contains(diagnostic), "{name}: expected {diagnostic}: {report}"); }
        else { assert!(report.is_empty(), "{name}: {report}"); }
    }
    common::cleanup(&dir);
}
