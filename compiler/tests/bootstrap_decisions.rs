//! Decision evidence is tied to executable fixed-semantics predicates.
mod common;

#[test]
fn core_decisions_recheck_tests_constructor_authority_and_branch_evidence() {
    let dir = common::project("core_decisions", &[
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Bootstrap.Kinds.cinch", include_str!("../../bootstrap/Bootstrap.Kinds.cinch")),
        ("Bootstrap.Global.cinch", include_str!("../../bootstrap/Bootstrap.Global.cinch")),
        ("Bootstrap.Instance.cinch", include_str!("../../bootstrap/Bootstrap.Instance.cinch")),
        ("Bootstrap.Typing.cinch", include_str!("../../bootstrap/Bootstrap.Typing.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.Decisions.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "core-decisions")
        .expect("build fixed-semantics decision checker");
    for size in [0, 1, 17, 129, 4096] {
        std::fs::write(dir.join("nonce.txt"), vec![b'x'; size]).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "decision case at nonce {size}; project {}\n{}",
            dir.display(), String::from_utf8_lossy(&output.stderr));
    }
    common::cleanup(&dir);
}
