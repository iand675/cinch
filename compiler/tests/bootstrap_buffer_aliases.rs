//! Strict let definitions retain actual value identity through typed proof contexts.
mod common;
#[test]
fn buffer_aliases_keep_identity_and_do_not_borrow_other_buffer_facts() {
    let dir=common::project("buffer_aliases",&[
        ("Bootstrap.Check.cinch",include_str!("../../bootstrap/Bootstrap.Check.cinch")),
        ("Bootstrap.Resolve.cinch",include_str!("../../bootstrap/Bootstrap.Resolve.cinch")),
        ("Bootstrap.Contracts.cinch",include_str!("../../bootstrap/Bootstrap.Contracts.cinch")),
        ("Bootstrap.Load.cinch",include_str!("../../bootstrap/Bootstrap.Load.cinch")),
        ("Bootstrap.Interface.cinch",include_str!("../../bootstrap/Bootstrap.Interface.cinch")),
        ("Main.cinch",include_str!("../../bootstrap/programs/Run.SourceRefinements.cinch")),
    ]);
    let exe=common::build(&dir,"Main.cinch","buffer-aliases").expect("build independent Core alias checker");
    for (name,body,status) in [
        ("direct buffer","same :: (b :: Bytes) -> refined (out == extent b) Nat\n  same b = extent b",0),
        ("strict buffer alias","same :: (b :: Bytes) -> refined (out == extent b) Nat\n  same b = let alias = b in extent alias",0),
        ("positive buffer alias","same :: using (extent b > 0) => (b :: Bytes) -> refined (out > 0) Nat\n  same b = let alias = b in extent alias",0),
        ("precondition through alias","first :: using (i < extent b) => (b :: Bytes) -> (i :: Nat) -> Nat\n  first b i = i\n  same :: using (i < extent b) => (b :: Bytes) -> (i :: Nat) -> Nat\n  same b i = let alias = b in first alias i",0),
        ("different buffer alias","bad :: using (extent b > 0) => (b :: Bytes) -> (c :: Bytes) -> refined (out > 0) Nat\n  bad b c = let alias = c in extent alias",4),
        ("shadowed alias","bad :: using (extent b > 0) => (b :: Bytes) -> (c :: Bytes) -> refined (out > 0) Nat\n  bad b c = let alias = b in let alias = c in extent alias",4),
        ("wrong precondition buffer alias","first :: using (i < extent b) => (b :: Bytes) -> (i :: Nat) -> Nat\n  first b i = i\n  bad :: using (i < extent b) => (b :: Bytes) -> (c :: Bytes) -> (i :: Nat) -> Nat\n  bad b c i = let alias = c in first alias i",4),
        ("refined buffer alias","type NonEmpty = refine (b :: Bytes) with extent b > 0\n  size :: NonEmpty -> refined (out > 0) Nat\n  size b = let alias = b in extent alias",0),
        ("alias chain","same :: (b :: Bytes) -> refined (out == extent b) Nat\n  same b = let first = b in let second = first in extent second",0),
    ] {
        let source=format!("module Main where\n  foreign import ccall \"cinch.runtime.bytesLen\" extent :: Bytes -> Nat\n  {body}\n  main :: Int\n  main = 0\n");
        std::fs::write(dir.join("source.cinch"),&source).unwrap();
        let result=std::process::Command::new(&exe).current_dir(&dir).output().unwrap();
        let report=common::read_output(&dir,"refinement-report.txt");
        assert_eq!(result.status.code(),Some(status),"{name}; project {}\n{source}\n{report}\n{}",dir.display(),String::from_utf8_lossy(&result.stderr));
    }
    common::cleanup(&dir);
}
