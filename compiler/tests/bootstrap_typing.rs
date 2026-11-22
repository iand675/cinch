//! Independently check Core types and effects in compiled Cinch.
mod common;

#[test]
fn core_typing_rechecks_annotations_with_actual_declarations_and_scopes() {
    let dir = common::project("core_typing", &[
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Bootstrap.Instance.cinch", include_str!("../../bootstrap/Bootstrap.Instance.cinch")),
        ("Bootstrap.Global.cinch", include_str!("../../bootstrap/Bootstrap.Global.cinch")),
        ("Bootstrap.Kinds.cinch", include_str!("../../bootstrap/Bootstrap.Kinds.cinch")),
        ("Bootstrap.Typing.cinch", include_str!("../../bootstrap/Bootstrap.Typing.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.CoreTyping.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "core-typing")
        .expect("build independent Core type checker");
    for size in [0, 1, 17, 129, 4096] {
        std::fs::write(dir.join("nonce.txt"), vec![b'x'; size]).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "Core typing case at nonce {size}; project {}\n{}",
            dir.display(), String::from_utf8_lossy(&output.stderr));
    }
    common::cleanup(&dir);
}
