//! Scoped source evidence, retained certificates, and independent Core checks.
mod common;

#[test]
fn guarded_nat_subtraction_uses_actual_branch_facts() {
    let dir = common::project("guarded_arithmetic", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.GuardedArithmetic.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "guarded-arithmetic").expect("build the Cinch guarded-arithmetic checker");
    let cases = [
        ("positive branch", "pred :: Nat -> Nat\n  pred n = if n > 0 then n - 1 else 0", false),
        ("negative branch", "pred :: Nat -> Nat\n  pred n = if n == 0 then 0 else n - 1", false),
        ("two actual arguments", "difference :: Nat -> Nat -> Nat\n  difference x y = if x >= y then x - y else 0", false),
        ("opposite comparison", "difference :: Nat -> Nat -> Nat\n  difference x y = if x < y then 0 else x - y", false),
        ("nested conditions", "difference :: Nat -> Nat -> Nat\n  difference x y = if x > 0 then (if y <= x then x - y else 0) else 0", false),
        ("Nat domain fact", "keep :: Nat -> Nat\n  keep n = n - 0", false),
        ("reflexive difference", "zero :: Nat -> Nat\n  zero n = n - n", false),
        ("constant arithmetic", "answer :: Nat\n  answer = (3 :: Nat) - 2", false),
        ("nested certificates", "twice :: Nat -> Nat\n  twice n = if n > 1 then (n - 1) - 1 else 0", false),
        ("short circuit conjunction", "pred :: Nat -> Nat\n  pred n = if n > 0 && n - 1 > 0 then n - 1 else 0", false),
        ("short circuit disjunction", "pred :: Nat -> Nat\n  pred n = if n == 0 || n == 1 then 0 else n - 1", false),
        ("local equality fact", "pred :: Nat -> Nat\n  pred n = if n > 0 then (let alias = n in alias - 1) else 0", false),
        ("later inference adds a Nat domain fact", "pred :: Nat -> Nat\n  pred n = let f = \\x -> if n > 0 then (let ignored = n - 1 in x) else (0 :: Nat) in f (0 :: Nat)", false),
        ("case guard", "pred :: Nat -> Nat\n  pred n = case n of\n    value | value > 0 -> value - 1\n    _ -> 0", false),
        ("equality condition", "difference :: Nat -> Nat\n  difference n = if n == 5 then n - 5 else 0", false),
        ("large mathematical bound", "difference :: Nat -> Nat\n  difference n = if n >= 340282366920938463463374607431768211456 then n - 340282366920938463463374607431768211456 else 0", false),
        ("unreachable branch", "answer :: Nat\n  answer = if False then (0 :: Nat) - 1 else 0", false),
        ("unguarded subtraction", "bad :: Nat -> Nat\n  bad n = n - 1", true),
        ("insufficient bound", "bad :: Nat -> Nat\n  bad n = if n >= 0 then n - 1 else 0", true),
        ("wrong branch", "bad :: Nat -> Nat\n  bad n = if n > 0 then 0 else n - 1", true),
        ("different argument identity", "bad :: Nat -> Nat -> Nat\n  bad n m = if m > 0 then n - 1 else 0", true),
        ("shadowed argument identity", "bad :: Nat -> Nat\n  bad n = if n > 0 then (let n = (0 :: Nat) in n - 1) else 0", true),
        ("branch fact cannot escape let", "bad :: Nat -> Nat\n  bad n = let ignored = if n > 0 then n - 1 else (0 :: Nat) in n - 1", true),
        ("guard fact cannot escape its alternative", "bad :: Nat -> Nat\n  bad n = case n of\n    value | value > 0 -> (0 :: Nat)\n    other -> other - 1", true),
        ("unknown predicate call", "positive :: Nat -> Bool\n  positive n = True\n  bad :: Nat -> Nat\n  bad n = if positive n then n - 1 else 0", true),
        ("negative constant result", "bad :: Nat\n  bad = (0 :: Nat) - 1", true),
    ];
    for (name, body, refused) in cases {
        let source = format!("module Main where\n  {body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"), &source).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report = common::read_output(&dir, "guarded-report.txt");
        assert_eq!(output.status.code(), Some(if refused {4} else {0}), "{name}; project {}\n{source}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
        if refused { assert!(report.contains("ordering evidence"), "{name}: {report}"); }
    }
    common::cleanup(&dir);
}
