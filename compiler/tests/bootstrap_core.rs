//! The compiled Cinch elaborator retains the actual checked terms. Each
//! input is supplied after compilation, preventing constant evaluation of
//! the test source from substituting for the runtime frontend.
mod common;

#[test]
fn typed_terms_retain_instantiation_patterns_and_polymorphic_scope() {
    let dir = common::project("typed_core", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Bootstrap.Dictionary.cinch", include_str!("../../bootstrap/Bootstrap.Dictionary.cinch")),
        ("Bootstrap.Infer.cinch", include_str!("../../bootstrap/Bootstrap.Infer.cinch")),
        ("Bootstrap.Nominal.cinch", include_str!("../../bootstrap/Bootstrap.Nominal.cinch")),
        ("Bootstrap.Parse.cinch", include_str!("../../bootstrap/Bootstrap.Parse.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.Core.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "core").expect("build Cinch typed-core reader");
    let mut cases = vec![
        ("evidence", "module Main where\n  main :: Int\n  main = 0\n", None),
        ("data", "module Main where\n  export Empty, Box, Wrap, main\n  data Empty a where {}\n  data Box a where\n    Hide :: a -> Box a\n    Keep :: Box a\n  data Wrap (f :: Type -> Type) a where\n    Wrap :: f a -> Wrap f a\n  main :: Int\n  main = 0\n", None),
        ("shadow", "module Main where\n  main :: Int\n  main = let x = 1 in (\\x -> x) x\n", None),
        ("as", "module Main where\n  keep :: (Int, Bool) -> (Int, Bool)\n  keep whole@(x, flag) = whole\n  main :: Int\n  main = 0\n", None),
        ("rejected", "module Main where\n  bad :: (Int, Bool) -> (Int, Bool)\n  bad whole@(whole, flag) = whole\n  main :: Int\n  main = 0\n", Some("duplicate pattern binder")),
        ("rejected", "module Main where\n  bad :: (Int, Bool) -> (Int, Bool)\n  bad (x, flag) as whole = whole\n  main :: Int\n  main = 0\n", Some("declaration needs")),
        ("scope", "module Main where\n  main :: Int\n  main = 0\n", None),
        ("polymorphic", "module Main where\n  identity :: forall a. a -> a\n  identity x = x\n  main :: Int\n  main = if identity True then identity 0 else 1\n", None),
        ("constructor", "module Main where\n  data Box a where\n    Box :: a -> Box a\n  unbox :: forall a. Box a -> a\n  unbox (Box x) = x\n  main :: Int\n  main = unbox (Box 0)\n", None),
        ("nested", "module Main where\n  main :: Int\n  main = let outer = \\x -> let local = \\y -> (x, y) in local in case outer 7 True of (n, flag) -> if flag then n else 0\n", None),
        ("private", "module Main where\n  ignore :: forall a. a -> Int\n  ignore x = 0\n  main :: Int\n  main = ignore (\\z -> z)\n", None),
        ("guard", "module Main where\n  choose :: Int -> Int\n  choose x | x > 0 = x\n  choose _ = 0\n  main :: Int\n  main = choose 2\n", None),
        ("rejected", "module Main where\n  main :: Int\n  main = 0\n  bad :: forall a. a -> a\n  bad x = 1\n", Some("type mismatch")),
        ("rejected", "module Main where\n  data Choice where\n    First :: Int -> Choice\n    Second :: Choice\n  get :: Choice -> Int\n  get = \\(First x) -> x\n  main :: Int\n  main = 0\n", Some("non-exhaustive")),
        ("rejected", "module Main where\n  main :: Int\n  main = (\\x x -> x) 1 2\n", Some("duplicate pattern binder")),
        ("and", "module Main where\n  identity :: forall a. a -> a\n  identity x = x\n  main :: Int\n  main = if False && identity True then 0 else 1\n", None),
        ("or", "module Main where\n  identity :: forall a. a -> a\n  identity x = x\n  main :: Int\n  main = if True || identity False then 0 else 1\n", None),
        ("rejected", "module Main where\n  main :: Int\n  main = if 1 && True then 0 else 1\n", Some("type mismatch")),
    ];
    // Bigger than an individual compiler module, with fresh inference cells
    // in every body. The source is still supplied only after compilation.
    let mut many = String::from("module Main where\n  main :: Int\n  main = 0\n");
    for index in 0..1500 {
        many.push_str(&format!("  f{index} :: Int -> Int\n  f{index} x = x + 0\n"));
    }
    cases.push(("many", many.as_str(), None));
    for (index, (mode, source, diagnostic)) in cases.into_iter().enumerate() {
        std::fs::write(dir.join("case.txt"), mode).unwrap();
        std::fs::write(dir.join("source.cinch"), source).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().expect("run typed-core reader");
        let report = common::read_output(&dir, "core-report.txt");
        assert_eq!(output.status.code(), Some(0), "core case {index} ({mode}); project {}\n{source}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
        if let Some(message) = diagnostic {
            assert!(report.contains(message), "case {index} must refuse with {message}: {report}");
        } else {
            assert!(report.is_empty(), "case {index}: {report}");
        }
    }
    common::cleanup(&dir);
}
