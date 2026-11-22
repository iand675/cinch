//! Pure polymorphism is checked by the Cinch elaborator, not the seed.
mod common;

#[test]
fn schemes_constructor_instantiation_and_scope_facts_are_checked_in_cinch() {
    let dir = common::project("infer", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Check.cinch", include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch", include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Builtin.cinch", include_str!("../../bootstrap/Bootstrap.Builtin.cinch")),
        ("Bootstrap.Contracts.cinch", include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch", include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch", include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.Infer.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "infer").expect("build Cinch inference runner");
    let out = std::process::Command::new(exe).current_dir(&dir).output().expect("run inference runner");
    let report = common::read_output(&dir, "infer-report.txt");
    assert_eq!(out.status.code(), Some(0), "inference case failed; project {}\n{}\n{}", dir.display(), String::from_utf8_lossy(&out.stderr), report);
    common::cleanup(&dir);
}
