//! Check retained global identities and dictionaries in compiled Cinch.
mod common;

#[test]
fn globals_recheck_instantiations_roles_slots_and_lexical_dictionaries() {
    let dir = common::project("global_references", &[
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Bootstrap.Instance.cinch", include_str!("../../bootstrap/Bootstrap.Instance.cinch")),
        ("Bootstrap.Global.cinch", include_str!("../../bootstrap/Bootstrap.Global.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.GlobalReferences.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "global-references")
        .expect("build independent global reference checker");
    for size in [0, 1, 17, 129, 4096] {
        std::fs::write(dir.join("nonce.txt"), vec![b'x'; size]).unwrap();
        let result = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(result.status.code(), Some(0),
            "global reference check at nonce {size}; project {}\n{}",
            dir.display(), String::from_utf8_lossy(&result.stderr));
    }
    common::cleanup(&dir);
}

#[test]
fn hidden_constructor_parameters_cannot_be_chosen_by_a_pattern_annotation() {
    let dir = common::project("constructor_scopes", &[
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Bootstrap.Instance.cinch", include_str!("../../bootstrap/Bootstrap.Instance.cinch")),
        ("Bootstrap.Global.cinch", include_str!("../../bootstrap/Bootstrap.Global.cinch")),
        ("Bootstrap.Typing.cinch", include_str!("../../bootstrap/Bootstrap.Typing.cinch")),
        ("Bootstrap.Kinds.cinch", include_str!("../../bootstrap/Bootstrap.Kinds.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.ConstructorScopes.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "constructor-scopes").expect("build actual Cinch constructor-scope checker");
    for size in [0, 1, 17, 129, 4096] {
        std::fs::write(dir.join("nonce.txt"), vec![b'x'; size]).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir)
            .env("CINCH_HEAP_LIMIT", "32M").env("CINCH_RC_DEBUG", "1").env("CINCH_LEAK_CHECK", "1").output().unwrap();
        assert_eq!(output.status.code(), Some(0), "constructor-scope control {size}: {}\nproject {}", String::from_utf8_lossy(&output.stderr), dir.display());
        assert!(String::from_utf8_lossy(&output.stderr).contains("cinch-leak: live=0 immortal=0 leaked=0"));
    }
    common::cleanup(&dir);
}
