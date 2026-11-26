//! Independently check Core types and effects in compiled Cinch.
mod common;

#[test]
fn core_typing_rechecks_annotations_with_actual_declarations_and_scopes() {
    let dir = common::project("core_typing", &[
        ("Cinch.Type.cinch", include_str!("../../compiler/Cinch.Type.cinch")),
        ("Cinch.Dictionary.cinch", include_str!("../../compiler/Cinch.Dictionary.cinch")),
        ("Cinch.Core.cinch", include_str!("../../compiler/Cinch.Core.cinch")),
        ("Cinch.Instance.cinch", include_str!("../../compiler/Cinch.Instance.cinch")),
        ("Cinch.Global.cinch", include_str!("../../compiler/Cinch.Global.cinch")),
        ("Cinch.Kinds.cinch", include_str!("../../compiler/Cinch.Kinds.cinch")),
        ("Cinch.Typing.cinch", include_str!("../../compiler/Cinch.Typing.cinch")),
        ("Main.cinch", include_str!("../../compiler/programs/Run.CoreTyping.cinch")),
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
