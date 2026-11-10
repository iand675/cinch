//! Retained checked Core and source stay associated across the pipeline boundary.
mod common;
fn files() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Inputs.cinch", include_str!("../../bootstrap/Bootstrap.Inputs.cinch")),
        ("Bootstrap.Builtin.cinch", include_str!("../../bootstrap/Bootstrap.Builtin.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Ir.cinch", include_str!("../../bootstrap/Bootstrap.Ir.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.CheckedProgram.cinch")),
    ]
}
#[test]
fn checked_program_retains_runtime_source_core_and_its_typed_emitter_boundary() {
    let dir = common::project("checked_program", &files());
    let exe = common::build(&dir, "Main.cinch", "checked-program").expect("build actual Cinch checked boundary probe");
    let valid = "module Main where\n  data Box a where\n    Box :: a -> Box a\n  identity :: forall a. a -> a\n  identity x = x\n  unwrap :: forall a. Box a -> a\n  unwrap (Box value) = identity value\n  main :: Int\n  main = 0\n";
    for (source, expected) in [(valid.to_owned(), 0), (valid.replace("identity value", "identity True"), 4)] {
        std::fs::write(dir.join("input.cinch"), source).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(expected), "checked boundary: {}", String::from_utf8_lossy(&output.stderr));
    }
    common::cleanup(&dir);
}
#[test]
fn checked_program_constructor_is_not_an_external_authority_bypass() {
    let mut project = files();
    project.retain(|(name, _)| *name != "Main.cinch");
    project.push(("Main.cinch", "module Main where\n  import Bootstrap.Ast\n  import Bootstrap.Infer (Elaboration)\n  import Bootstrap.Check\n  forge :: Program -> Elaboration -> CheckedProgram\n  forge surface typed = MkCheckedProgram surface typed\n  main :: Int\n  main = 0\n"));
    let dir = common::project("checked_program_private", &project);
    let error = common::check(&dir, "Main.cinch").expect_err("private checked constructor must be inaccessible");
    assert!(error.contains("MkCheckedProgram"), "{error}");
    common::cleanup(&dir);
}
