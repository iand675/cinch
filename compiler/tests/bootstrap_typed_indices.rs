//! Dependent substitutions retain refined carriers and explicit numeric projections.
mod common;

#[test]
fn typed_indices_preserve_sorts_scope_and_numeric_conversion() {
    let dir = common::project("typed_indices", &[
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.TypedIndices.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "typed-indices").expect("build typed index runner");
    for size in [0, 1, 17, 129, 4096] {
        std::fs::write(dir.join("nonce.txt"), vec![b'x'; size]).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "typed index check failed at nonce size {size}; exit code names first failed check; project {}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
    }
    common::cleanup(&dir);
}
