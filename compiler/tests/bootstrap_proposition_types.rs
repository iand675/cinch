//! Proposition kinds and non-erasing function-contract types in compiled Cinch.
mod common;

#[test]
fn proposition_contract_types_preserve_scope_and_runtime_separation() {
    let dir = common::project("proposition_types", &[
        ("Bootstrap.Ast.cinch", include_str!("../../bootstrap/Bootstrap.Ast.cinch")),
        ("Bootstrap.Report.cinch", include_str!("../../bootstrap/Bootstrap.Report.cinch")),
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.PropositionTypes.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "proposition-types").expect("build the Cinch proposition type runner");
    // Each size changes identities above 2^128 after the executable is built.
    for size in [0, 1, 17, 129, 4096] {
        std::fs::write(dir.join("nonce.txt"), vec![b'x'; size]).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "proposition check failed at nonce size {size}, first failed check is its exit code; project {}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
    }
    common::cleanup(&dir);
}
