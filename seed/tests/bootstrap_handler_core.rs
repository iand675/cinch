//! Independently check Core types and effects in compiled Cinch.
mod common;

#[test]
fn handlers_recheck_actual_slots_rows_and_one_shot_resumptions() {
    let dir = common::project("handler_core", &[
        ("Cinch.Type.cinch", include_str!("../../compiler/Cinch.Type.cinch")),
        ("Cinch.Core.cinch", include_str!("../../compiler/Cinch.Core.cinch")),
        ("Cinch.Instance.cinch", include_str!("../../compiler/Cinch.Instance.cinch")),
        ("Cinch.Global.cinch", include_str!("../../compiler/Cinch.Global.cinch")),
        ("Cinch.Kinds.cinch", include_str!("../../compiler/Cinch.Kinds.cinch")),
        ("Cinch.Typing.cinch", include_str!("../../compiler/Cinch.Typing.cinch")),
        ("Main.cinch", include_str!("../../compiler/programs/Run.HandlerCore.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "handler-core")
        .expect("build independent handler Core checker");
    for size in [0, 1, 17, 129, 4096] {
        std::fs::write(dir.join("nonce.txt"), vec![b'x'; size]).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "Core typing case at nonce {size}; project {}\n{}",
            dir.display(), String::from_utf8_lossy(&output.stderr));
    }
    common::cleanup(&dir);
}
