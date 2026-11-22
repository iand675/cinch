//! Effect and region types must survive real executable unification.
mod common;

#[test]
fn closed_effect_rows_regions_and_scope_are_checked_in_cinch() {
    let dir = common::project("effect_types", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Bootstrap.Dictionary.cinch", include_str!("../../bootstrap/Bootstrap.Dictionary.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.EffectTypes.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "effect-types").expect("build Cinch effect type runner");
    for input in ["", "a", "region", "distinct regions"] {
        std::fs::write(dir.join("case.txt"), input).unwrap();
        let out = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(out.status.code(), Some(0), "input {input:?}; project {}\n{}\n{}", dir.display(), common::read_output(&dir, "effect-report.txt"), String::from_utf8_lossy(&out.stderr));
    }
    common::cleanup(&dir);
}
