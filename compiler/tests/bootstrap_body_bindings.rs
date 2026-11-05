//! Final Core checks bind total semantics to actual global declarations.
mod common;

#[test]
fn compiled_cinch_rechecks_body_bindings_refinements_and_changed_callees() {
    let dir = common::project("body_bindings", &[
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Bootstrap.Instance.cinch", include_str!("../../bootstrap/Bootstrap.Instance.cinch")),
        ("Bootstrap.Global.cinch", include_str!("../../bootstrap/Bootstrap.Global.cinch")),
        ("Bootstrap.Kinds.cinch", include_str!("../../bootstrap/Bootstrap.Kinds.cinch")),
        ("Bootstrap.Typing.cinch", include_str!("../../bootstrap/Bootstrap.Typing.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.BodyBindings.cinch")),
    ]);
    for (file, low, high) in [("bodies.json", 1, 4), ("changed.json", 5, 7)] {
        let source = format!("module Main where\n  import Algebra.Arithmetic (InRange)\n  width :: Nat -> InRange {low} {high} Nat\n  width cp = if cp < 128 then {low} else {high}\n  main :: Int\n  main = 0\n");
        let checked = cinch::driver::check_sources(&source, "actual-body-declaration.cinch").unwrap();
        std::fs::write(dir.join(file), checked.total_bodies.closure(["Main.width"]).unwrap().to_json()).unwrap();
    }
    let exe = common::build(&dir, "Main.cinch", "body-bindings").expect("compile body/declaration association checker");
    for size in [0, 1, 17, 129, 4096] {
        std::fs::write(dir.join("nonce.txt"), vec![b'x'; size]).unwrap();
        let result = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(result.status.code(), Some(0), "body binding at nonce{size}; project {}\n{}", dir.display(), String::from_utf8_lossy(&result.stderr));
    }
    common::cleanup(&dir);
}


#[test]
fn compiled_cinch_source_bodies_require_the_actual_root_and_dependency_definitions() {
    let dir = common::project("source_body_bindings", &[
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Bootstrap.Global.cinch", include_str!("../../bootstrap/Bootstrap.Global.cinch")),
        ("Bootstrap.Instance.cinch", include_str!("../../bootstrap/Bootstrap.Instance.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.SourceBodyBindings.cinch")),
    ]);
    let exe = common::build(&dir, "Main.cinch", "source-body-bindings").expect("compile actual source-body association checker");
    for size in [0, 8, 64] {
        std::fs::write(dir.join("nonce.txt"), vec![b'x'; size]).unwrap();
        let result = std::process::Command::new(&exe).current_dir(&dir)
            .env("CINCH_HEAP_LIMIT", "32M").env("CINCH_RC_DEBUG", "1").env("CINCH_LEAK_CHECK", "1").output().unwrap();
        assert_eq!(result.status.code(), Some(0), "source-body authority at nonce{size}; project {}\n{}", dir.display(), String::from_utf8_lossy(&result.stderr));
        assert!(String::from_utf8_lossy(&result.stderr).contains("cinch-leak: live=0 immortal=0 leaked=0"));
    }
    common::cleanup(&dir);
}
