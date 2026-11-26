//! Module hygiene is observed through values, not only through symbol text.
mod common;

#[test]
fn source_module_imports_exports_and_lexical_scopes_resolve() {
    let dir = common::project("resolve", &[
        ("Cinch.Ast.cinch", include_str!("../../compiler/Cinch.Ast.cinch")),
        ("Cinch.Builtin.cinch", include_str!("../../compiler/Cinch.Builtin.cinch")),
        ("Cinch.Report.cinch", include_str!("../../compiler/Cinch.Report.cinch")),
        ("Cinch.Parse.cinch", include_str!("../../compiler/Cinch.Parse.cinch")),
        ("Cinch.Resolve.cinch", include_str!("../../compiler/Cinch.Resolve.cinch")),
        ("Cinch.Inputs.cinch", include_str!("../../compiler/Cinch.Inputs.cinch")),
        ("Cinch.Contracts.cinch", include_str!("../../compiler/Cinch.Contracts.cinch")),
        ("Cinch.Load.cinch", include_str!("../../compiler/Cinch.Load.cinch")),
        ("Cinch.Interface.cinch", include_str!("../../compiler/Cinch.Interface.cinch")),
        ("Cinch.Check.cinch", include_str!("../../compiler/Cinch.Check.cinch")),
        ("Cinch.Eval.cinch", include_str!("../../compiler/Cinch.Eval.cinch")),
        ("Main.cinch", include_str!("../../compiler/programs/Run.Resolve.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "resolve").expect("build namespace runner");
    let code = std::process::Command::new(exe).current_dir(&dir)
        .env("CINCH_HEAP_LIMIT", std::env::var("CINCH_HEAP_LIMIT").unwrap_or_else(|_| "2G".into()))
        .status().expect("run namespace runner").code().expect("runner exit code");
    let report = common::read_output(&dir, "resolve-report.txt");
    assert_eq!(code, 0, "resolution case {code}: values or named refusals; project: {}\n{report}", dir.display());
    common::cleanup(&dir);
}
