//! Refinement binders preserve predicates, scope and conversion boundaries in compiled Cinch.
mod common;

#[test]
fn refinement_core_rechecks_actual_values_and_contexts() {
    let dir = common::project("refinement_core", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.RefinementCore.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "refinement-core").expect("build the Cinch refinement Core runner");
    // Each size changes identities above 2^128 after the executable is built.
    for size in [0, 1, 17, 129, 4096] {
        std::fs::write(dir.join("nonce.txt"), vec![b'x'; size]).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "refinement check failed at nonce size {size}, first failed check is its exit code; project {}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
    }
    common::cleanup(&dir);
}
