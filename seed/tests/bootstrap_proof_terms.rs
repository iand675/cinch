//! Named proof references exist only in erased Core operands and scopes.
mod common;

#[test]
fn core_proof_scopes_preserve_effects_and_refuse_runtime_elimination() {
    let dir = common::project("proof_terms", &[
        ("Cinch.Type.cinch", include_str!("../../compiler/Cinch.Type.cinch")),
        ("Cinch.Core.cinch", include_str!("../../compiler/Cinch.Core.cinch")),
        ("Cinch.Kinds.cinch", include_str!("../../compiler/Cinch.Kinds.cinch")),
        ("Cinch.Global.cinch", include_str!("../../compiler/Cinch.Global.cinch")),
        ("Cinch.Instance.cinch", include_str!("../../compiler/Cinch.Instance.cinch")),
        ("Cinch.Typing.cinch", include_str!("../../compiler/Cinch.Typing.cinch")),
        ("Main.cinch", include_str!("../../compiler/programs/Run.ProofTerms.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "proof-terms")
        .expect("build erased proof term checker");
    for size in [0, 1, 17, 129, 4096] {
        std::fs::write(dir.join("nonce.txt"), vec![b'x'; size]).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "proof term case at nonce {size}; project {}\n{}",
            dir.display(), String::from_utf8_lossy(&output.stderr));
    }
    common::cleanup(&dir);
}
