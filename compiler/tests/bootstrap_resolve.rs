//! Module hygiene is observed through values, not only through symbol text.
mod common;

#[test]
fn source_module_imports_exports_and_lexical_scopes_resolve() {
    let dir = common::project("resolve", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Builtin.cinch", include_str!("../../bootstrap/Bootstrap.Builtin.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Parse.cinch", include_str!("../../bootstrap/Bootstrap.Parse.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Eval.cinch", include_str!("../../bootstrap/Bootstrap.Eval.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.Resolve.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "resolve").expect("build namespace runner");
    let code = std::process::Command::new(exe).current_dir(&dir)
        .env("CINCH_HEAP_LIMIT", std::env::var("CINCH_HEAP_LIMIT").unwrap_or_else(|_| "2G".into()))
        .status().expect("run namespace runner").code().expect("runner exit code");
    let report = common::read_output(&dir, "resolve-report.txt");
    assert_eq!(code, 0, "resolution case {code}: values or named refusals; project: {}\n{report}", dir.display());
    common::cleanup(&dir);
}
