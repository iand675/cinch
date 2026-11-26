//! Independent guarded-body admission uses actual code and full callable guards.
mod common;
#[test]
fn guarded_calls_recheck_actual_body_declarations_and_stale_aliases() {
    let dir = common::project("guarded_body_association", &[
        ("Cinch.Core.cinch", include_str!("../../compiler/Cinch.Core.cinch")),
        ("Cinch.Global.cinch", include_str!("../../compiler/Cinch.Global.cinch")),
        ("Cinch.Typing.cinch", include_str!("../../compiler/Cinch.Typing.cinch")),
        ("Cinch.Instance.cinch", include_str!("../../compiler/Cinch.Instance.cinch")),
        ("Cinch.Kinds.cinch", include_str!("../../compiler/Cinch.Kinds.cinch")),
        ("Cinch.Guarded.cinch", include_str!("../../compiler/Cinch.Guarded.cinch")),
        ("Cinch.Type.cinch", include_str!("../../compiler/Cinch.Type.cinch")),
        ("Cinch.Proof.cinch", include_str!("../../compiler/Cinch.Proof.cinch")),
        ("Cinch.Nominal.cinch", include_str!("../../compiler/Cinch.Nominal.cinch")),
        ("Cinch.Ast.cinch", include_str!("../../compiler/Cinch.Ast.cinch")),
        ("Cinch.Report.cinch", include_str!("../../compiler/Cinch.Report.cinch")),
        ("Main.cinch", include_str!("../../compiler/programs/Run.GuardedAssociation.cinch")),
    ]);
    let (_, interfaces) = cinch::driver::check_sources_ifaces("module Main where\n  main :: Int\n  main = 0\n", "guarded-kernel-base.cinch").unwrap();
    let nat = interfaces.iter().find(|iface| iface.module == "Data.Nat").unwrap();
    std::fs::write(dir.join("interface.json"), nat.to_json()).unwrap();
    let exe = common::build(&dir, "Main.cinch", "guarded-association").expect("compile independent guarded-body admission");
    let result = std::process::Command::new(exe).current_dir(&dir).output().unwrap();
    let report = common::read_output(&dir, "guarded-association-report.txt");
    assert_eq!(result.status.code(), Some(0), "project {}\n{report}\n{}", dir.display(), String::from_utf8_lossy(&result.stderr));
    common::cleanup(&dir);
}
