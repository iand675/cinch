//! Erased proof arrows preserve latent effects and distinct source/runtime arity.
mod common;

#[test]
fn erased_arrows_preserve_scope_effects_and_reject_runtime_proof_arguments() {
    let dir = common::project("proof_arrows", &[
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Bootstrap.Kinds.cinch", include_str!("../../bootstrap/Bootstrap.Kinds.cinch")),
        ("Bootstrap.Global.cinch", include_str!("../../bootstrap/Bootstrap.Global.cinch")),
        ("Bootstrap.Instance.cinch", include_str!("../../bootstrap/Bootstrap.Instance.cinch")),
        ("Bootstrap.Typing.cinch", include_str!("../../bootstrap/Bootstrap.Typing.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.ProofArrows.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "proof-arrows")
        .expect("build erased-arrow type and scope checks");
    for size in [0, 1, 17, 129, 4096] {
        std::fs::write(dir.join("nonce.txt"), vec![b'x'; size]).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "proof arrow case at nonce {size}; project {}\n{}",
            dir.display(), String::from_utf8_lossy(&output.stderr));
    }
    common::cleanup(&dir);
}
