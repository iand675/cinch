//! `Effect.Find` consumer semantics over a real tree on disk (P40, the
//! Ruby `Find` port): pre-order traversal of a tree two levels deep
//! (with an empty directory among its leaves), the root-first
//! property, `Prune` (the pruned subtree's entries absent while its
//! siblings still appear, and pruning a file a no-op), `Stop` (the
//! reported prefix ending at the stopping entry), the
//! `find-files`/`find-dirs` filters, and the named error path for a
//! root that does not exist.
//!
//! The tree's directory skeleton comes from Rust because `Effect.Fs`
//! has no directory creation (`fs-list`/`fs-metadata` are the whole
//! filesystem surface the walk may use, and the walk adds no externs);
//! the probe then writes one file per subtree through `Fs` itself, so
//! the walk is asserted over paths the capability created. The walk
//! itself runs as a real program through the same `driver` the CLI
//! uses.
//!
//! Each check is a top-level `check-*` function returning 0 or its bit
//! and `main` sums them, so a nonzero exit code is a bitmask naming the
//! failed checks: bit 1 is the `Fs` setup write, bit 2 the root-first
//! property, bit 4 the full pre-order sequence, bit 8 pruning, bit 16
//! stopping, bit 32 pruning at the root, bit 64 the filters, and in the
//! second probe bit 1 the missing root and bit 2 the empty root. The
//! checks are one shallow definition each (the checker's
//! per-expression recursion budget on the default test thread is why).

use cinch::driver::{driver, Mode};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

const FIND: &str = include_str!("../prelude/Effect/Find.cinch");

/// Compile and run a two-module project (the module under test beside
/// `Main.cinch`) in a fresh temp directory, returning the exit code.
fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_findtest_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [("Find.cinch", FIND), ("Main.cinch", main_src)] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    match driver(Mode::Run, &dir.join("Main.cinch"), None) {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

/// A fresh empty scratch directory (each probe owns its tree).
fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cinch_find_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

/// A path as a Cinch string literal.
fn lit(p: &Path) -> String {
    let mut out = String::from("\"");
    for ch in p.to_string_lossy().chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// The probe tree (the walk's expectations read off its shape):
///
///   root/
///     a/
///       f1.txt      a file
///       nested/
///         deep.txt
///         deep2.txt
///       z.txt
///     b/            (f2.txt is written through `Fs` by the probe)
///     empty/        an empty directory
///
/// `fs-list` sorts names ascending (D335), so the pre-order walk of
/// this tree is root, a, a/f1.txt, a/nested, a/nested/deep.txt,
/// a/nested/deep2.txt, a/z.txt, b, b/f2.txt, empty.
fn tree(tag: &str) -> PathBuf {
    let root = scratch(tag);
    for d in ["a", "a/nested", "b", "empty"] {
        std::fs::create_dir_all(root.join(d)).expect("tree dir");
    }
    for (rel, body) in [
        ("a/f1.txt", "f1"),
        ("a/nested/deep.txt", "deep"),
        ("a/nested/deep2.txt", "deep2"),
        ("a/z.txt", "z"),
    ] {
        std::fs::write(root.join(rel), body).expect("tree file");
    }
    root
}

const WALK_MAIN: &str = r#"
module Main where

  -- `<>` (the curated list append the expectations are built with) is
  -- not in the facade's surface, so the class module is imported; the
  -- `infixr 6 <>` that `Algebra.Semigroup` declares travels with that
  -- import, so this module parses it infix with no local declaration.
  import Algebra.Semigroup
  import Data.Path
  import Effect.Fs
  import Effect.Find

  root :: Path
  root = path-from-string __DIR__

  under :: Path -> String -> Path
  under p n = path-join p (path-from-string n)

  a-dir :: Path
  a-dir = under root "a"

  b-dir :: Path
  b-dir = under root "b"

  empty-dir :: Path
  empty-dir = under root "empty"

  nested :: Path
  nested = under a-dir "nested"

  f1 :: Path
  f1 = under a-dir "f1.txt"

  zed :: Path
  zed = under a-dir "z.txt"

  deep :: Path
  deep = under nested "deep.txt"

  deep2 :: Path
  deep2 = under nested "deep2.txt"

  f2 :: Path
  f2 = under b-dir "f2.txt"

  -- The file this program writes through `Fs`: the walk must report it,
  -- so the seed is what makes the tree the capability's, not Rust's.
  seed :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  seed u = case fs-write f2 (string-to-bytes "f2") of
    Ok _ -> True
    Err e -> False

  -- Path identity for the checks: the texts the walk reports.
  same-paths :: List Path -> List Path -> Bool
  same-paths xs ys = case xs of
    Nil -> case ys of
      Nil -> True
      Cons b bs -> False
    Cons x xr -> case ys of
      Nil -> False
      Cons y yr -> case path-to-string x == path-to-string y of
        True -> same-paths xr yr
        False -> False

  continue-all :: Path -> Step
  continue-all p = Continue

  -- The pre-order expectation in the pieces the checks share: the head
  -- [root, a, a/f1.txt], then [nested, nested/deep.txt,
  -- nested/deep2.txt], then [a/z.txt, b, b/f2.txt, empty].
  order-head :: List Path
  order-head = Cons root (Cons a-dir (Cons f1 Nil))

  order-nested :: List Path
  order-nested = Cons nested (Cons deep (Cons deep2 Nil))

  order-tail :: List Path
  order-tail = Cons zed (Cons b-dir (Cons f2 (Cons empty-dir Nil)))

  expected-order :: List Path
  expected-order = order-head <> order-nested <> order-tail

  check-root-first :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-root-first u = case find root continue-all of
    Err e -> False
    Ok ps -> case ps of
      Nil -> False
      Cons p rest -> path-to-string p == path-to-string root

  check-order :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-order u = case find root continue-all of
    Err e -> False
    Ok ps -> same-paths ps expected-order

  -- Pruning `nested` drops its contents; pruning a FILE (`a/f1.txt`)
  -- must keep reporting it, so one visitor pins both.
  pruner :: Path -> Step
  pruner p = case path-to-string p == path-to-string nested of
    True -> Prune
    False -> case path-to-string p == path-to-string f1 of
      True -> Prune
      False -> Continue

  prune-order :: List Path
  prune-order = Cons root (Cons a-dir (Cons f1 (Cons nested Nil))) <> order-tail

  check-prune :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-prune u = case find root pruner of
    Err e -> False
    Ok ps -> same-paths ps prune-order

  stopper :: Path -> Step
  stopper p = case path-to-string p == path-to-string deep of
    True -> Stop
    False -> Continue

  stop-order :: List Path
  stop-order = order-head <> Cons nested (Cons deep Nil)

  check-stop :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-stop u = case find root stopper of
    Err e -> False
    Ok ps -> same-paths ps stop-order

  prune-every :: Path -> Step
  prune-every p = Prune

  -- Pruning at the root reports the root and nothing else.
  check-root-prune :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-root-prune u = case find root prune-every of
    Err e -> False
    Ok ps -> same-paths ps (Cons root Nil)

  files-order :: List Path
  files-order = Cons f1 (Cons deep (Cons deep2 Nil)) <> Cons zed (Cons f2 Nil)

  dirs-order :: List Path
  dirs-order = Cons root (Cons a-dir (Cons nested Nil)) <> Cons b-dir (Cons empty-dir Nil)

  check-filters :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-filters u = case find-files root of
    Err e -> False
    Ok fs -> case same-paths fs files-order of
      False -> False
      True -> case find-dirs root of
        Err e -> False
        Ok ds -> same-paths ds dirs-order

  checks :: forall r. using (io :: Io r) => Unit ->{IO r} Int
  checks u =
    let c1 = if check-root-first () then 0 else 2 in
    let c2 = if check-order () then 0 else 4 in
    let c3 = if check-prune () then 0 else 8 in
    let c4 = if check-stop () then 0 else 16 in
    let c5 = if check-root-prune () then 0 else 32 in
    let c6 = if check-filters () then 0 else 64 in
    c1 + c2 + c3 + c4 + c5 + c6

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    case seed () of
      False -> 1
      True -> checks ()
"#;

const EDGES_MAIN: &str = r#"
module Main where

  import Data.Path
  import Effect.Fs
  import Effect.Find

  root :: Path
  root = path-from-string __DIR__

  missing :: Path
  missing = path-join root (path-from-string "no-such-dir")

  continue-all :: Path -> Step
  continue-all p = Continue

  -- A root that does not exist is the NAMED error, not a fault and not
  -- an empty result, and the filters report it the same way.
  check-missing :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-missing u = case find missing continue-all of
    Ok _ -> False
    Err e -> case e of
      NotFound -> case find-files missing of
        Ok _ -> False
        Err e2 -> case e2 of
          NotFound -> True
          _ -> False
      _ -> False

  -- An empty root walks to itself, exactly once: the empty directory is
  -- a leaf with no entries to descend into.
  check-empty-root :: forall r. using (io :: Io r) => Unit ->{IO r} Bool
  check-empty-root u = case find root continue-all of
    Err e -> False
    Ok ps -> case ps of
      Nil -> False
      Cons p rest -> case rest of
        Nil -> path-to-string p == path-to-string root
        Cons q qs -> False

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let c1 = if check-missing () then 0 else 1 in
    let c2 = if check-empty-root () then 0 else 2 in
    c1 + c2
"#;

#[test]
fn find_walks_pre_order_with_prune_stop_and_filters() {
    let root = tree("walk");
    let src = WALK_MAIN.replace("__DIR__", &lit(&root));
    let code = run_main("find-walk", &src);
    let _ = std::fs::remove_dir_all(&root);
    assert_eq!(
        code, 0,
        "find-walk: bitmask {code} (1 seed write, 2 root first, 4 pre-order, \
         8 prune, 16 stop, 32 root prune, 64 filters)"
    );
}

#[test]
fn find_reports_a_missing_root_and_an_empty_root_as_itself() {
    let root = scratch("empty_root");
    let src = EDGES_MAIN.replace("__DIR__", &lit(&root));
    let code = run_main("find-edges", &src);
    let _ = std::fs::remove_dir_all(&root);
    assert_eq!(
        code, 0,
        "find-edges: bitmask {code} (1 missing root, 2 empty root)"
    );
}
