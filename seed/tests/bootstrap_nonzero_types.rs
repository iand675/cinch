//! Structural NonZero retains its type identity and validates actual proof operands.
mod common;

#[test]
fn structural_nonzero_types_and_core_preserve_scope_and_evidence() {
    let dir = common::project("nonzero_types", &[
        ("Cinch.Type.cinch", include_str!("../../compiler/Cinch.Type.cinch")),
        ("Cinch.Core.cinch", include_str!("../../compiler/Cinch.Core.cinch")),
        ("Main.cinch", include_str!("../../compiler/programs/Run.NonZeroTypes.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "nonzero-types").expect("build structural NonZero type/Core runner");
    for size in [0, 1, 17, 129, 4096] {
        std::fs::write(dir.join("nonce.txt"), vec![b'x'; size]).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "NonZero check failed at nonce size {size}; exit code names first failing check; project {}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
    }
    common::cleanup(&dir);
}
