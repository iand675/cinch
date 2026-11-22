//! Independent guarded-body admission uses actual code and full callable guards.
mod common;
#[test]
fn guarded_calls_recheck_actual_body_declarations_and_stale_aliases() {
    let dir = common::project("guarded_body_association", &[
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Bootstrap.Global.cinch", include_str!("../../bootstrap/Bootstrap.Global.cinch")),
        ("Bootstrap.Typing.cinch", include_str!("../../bootstrap/Bootstrap.Typing.cinch")),
        ("Bootstrap.Instance.cinch", include_str!("../../bootstrap/Bootstrap.Instance.cinch")),
        ("Bootstrap.Kinds.cinch", include_str!("../../bootstrap/Bootstrap.Kinds.cinch")),
        ("Bootstrap.Guarded.cinch", include_str!("../../bootstrap/Bootstrap.Guarded.cinch")),
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Proof.cinch", include_str!("../../bootstrap/Bootstrap.Proof.cinch")),
        ("Bootstrap.Nominal.cinch", include_str!("../../bootstrap/Bootstrap.Nominal.cinch")),
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.GuardedAssociation.cinch")),
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
