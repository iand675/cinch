//! Pure polymorphism is checked by the Cinch elaborator, not the seed.
mod common;

#[test]
fn schemes_constructor_instantiation_and_scope_facts_are_checked_in_cinch() {
    let dir = common::project("infer", &[
        ("Cinch.Ast.cinch", include_str!("../../compiler/Cinch.Ast.cinch")),
        ("Cinch.Report.cinch", include_str!("../../compiler/Cinch.Report.cinch")),
        ("Cinch.Check.cinch", include_str!("../../compiler/Cinch.Check.cinch")),
        ("Cinch.Resolve.cinch", include_str!("../../compiler/Cinch.Resolve.cinch")),
        ("Cinch.Inputs.cinch", include_str!("../../compiler/Cinch.Inputs.cinch")),
        ("Cinch.Builtin.cinch", include_str!("../../compiler/Cinch.Builtin.cinch")),
        ("Cinch.Contracts.cinch", include_str!("../../compiler/Cinch.Contracts.cinch")),
        ("Cinch.Load.cinch", include_str!("../../compiler/Cinch.Load.cinch")),
        ("Cinch.Interface.cinch", include_str!("../../compiler/Cinch.Interface.cinch")),
        ("Main.cinch", include_str!("../../compiler/programs/Run.Infer.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "infer").expect("build Cinch inference runner");
    let out = std::process::Command::new(exe).current_dir(&dir).output().expect("run inference runner");
    let report = common::read_output(&dir, "infer-report.txt");
    assert_eq!(out.status.code(), Some(0), "inference case failed; project {}\n{}\n{}", dir.display(), String::from_utf8_lossy(&out.stderr), report);
    common::cleanup(&dir);
}
