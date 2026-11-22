//! Checked implicit factory selection and evidence/guard reconstruction in Cinch.
mod common;
#[test]
fn instance_factories_preserve_scope_coherence_and_actual_method_semantics() {
    let dir = common::project("instance_resolution", &[
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Bootstrap.Dictionary.cinch", include_str!("../../bootstrap/Bootstrap.Dictionary.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.InstanceResolution.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "instance-resolution").expect("build the Cinch checked-instance runner");
    for size in [0, 1, 17, 129, 4096] {
        std::fs::write(dir.join("nonce.txt"), vec![b'x'; size]).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "instance/guard check at nonce {size}; failing group is exit code; project {}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
    }
    common::cleanup(&dir);
}
