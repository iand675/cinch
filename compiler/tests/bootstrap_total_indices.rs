//! Total calls retain their checked semantics, typed arguments and lexical scope.
mod common;

#[test]
fn compiled_cinch_total_indices_preserve_body_authority_and_scoped_arguments() {
    let dir = common::project("total_indices", &[
        ("Bootstrap.Type.cinch", include_str!("../../bootstrap/Bootstrap.Type.cinch")),
        ("Bootstrap.Core.cinch", include_str!("../../bootstrap/Bootstrap.Core.cinch")),
        ("Bootstrap.Kinds.cinch", include_str!("../../bootstrap/Bootstrap.Kinds.cinch")),
        ("Main.cinch", include_str!("../../bootstrap/programs/Run.TotalIndices.cinch")),
    ]);
    let source = "module Main where\n  width :: Nat -> Nat\n  width cp = if cp < 128 then 1 else 2\n  main :: Int\n  main = 0\n";
    for (file, text) in [("bodies.json", source.to_string()), ("changed.json", source.replace("then 1 else", "then 7 else"))] {
        let checked = cinch::driver::check_sources(&text, "typed-total-source.cinch").unwrap();
        std::fs::write(dir.join(file), checked.total_bodies.closure(["Main.width"]).unwrap().to_json()).unwrap();
    }
    let exe = common::build(&dir, "Main.cinch", "total-indices").expect("compile typed total-call checker");
    for size in [0, 1, 17, 129, 4096] {
        std::fs::write(dir.join("nonce.txt"), vec![b'x'; size]).unwrap();
        let output = std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        assert_eq!(output.status.code(), Some(0), "typed total-call check {size}; project {}\n{}", dir.display(), String::from_utf8_lossy(&output.stderr));
    }
    common::cleanup(&dir);
}
