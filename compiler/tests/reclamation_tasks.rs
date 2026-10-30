//! Reclamation across tasks (design/reclamation.md section 8, slice S9):
//! a spawn marks everything its body reaches as atomically counted, the
//! task's result moves to its join, and whichever thread drops a shared
//! block last frees it. Each program is built with reference counting
//! (`TargetProfile::reclaim`) and runs under `CINCH_LEAK_CHECK=1`, plainly
//! and from a `rc_debug` build under `CINCH_RC_DEBUG=1`, where a count
//! operation on a freed block traps (exit 71) instead of reading reused
//! memory.
//!
//! The sizes are chosen so that a missing atomic fails every run, not by
//! luck: the shared-list program has five threads (the parent and four
//! tasks) traversing the same 1000-cell list 150 times each at once, so
//! about 1.5 million dup and drop pairs land on 1000 shared headers that
//! fit in cache together. With marking switched off in a copy of the
//! runtime (the counts left plain and racing), it faulted in 10 of 10 runs,
//! and so did the shared-tree program in 6 of 6; with marking, both pass
//! every run.

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::{driver_with_profile, Mode};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

fn project(tag: &str, src: &str) -> PathBuf {
    static N: AtomicU64 = AtomicU64::new(0);
    let n = N.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("cinch_rct_{tag}_{}_{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    std::fs::write(dir.join("Main.cinch"), src).expect("write source");
    dir
}

fn build(dir: &Path, debug: bool) -> PathBuf {
    let exe = dir.join(if debug { "prog_dbg" } else { "prog" });
    let profile = TargetProfile { reclaim: true, rc_debug: debug, ..TargetProfile::native() };
    let (src, out) = (dir.join("Main.cinch"), exe.clone());
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver_with_profile(Mode::Build, StageMode::Refresh, &src, Some(&out), &profile))
        .expect("driver thread")
        .join()
        .expect("driver thread panicked")
        .unwrap_or_else(|e| panic!("build failed: {e}"));
    exe
}

fn run(exe: &Path, debug: bool) -> Output {
    let mut cmd = Command::new(exe);
    cmd.env("CINCH_HEAP_LIMIT", "512M").env("CINCH_LEAK_CHECK", "1").env_remove("CINCH_ALLOC_STATS");
    if debug {
        cmd.env("CINCH_RC_DEBUG", "1");
    }
    cmd.output().expect("spawn program")
}

fn leaked(out: &Output) -> u64 {
    let err = String::from_utf8_lossy(&out.stderr);
    let line = err.lines().find(|l| l.starts_with("cinch-leak:")).unwrap_or_else(|| panic!("no leak line: {err}"));
    line.split_whitespace()
        .find_map(|w| w.strip_prefix("leaked="))
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| panic!("no leaked= in {line}"))
}

/// Build `src` both ways and run each build `runs` times: every run
/// exits 0 with nothing leaked.
fn expect_clean(tag: &str, src: &str, runs: usize) {
    let dir = project(tag, src);
    for debug in [false, true] {
        let exe = build(&dir, debug);
        for i in 0..runs {
            let out = run(&exe, debug);
            assert_eq!(out.status.code(), Some(0), "{tag} (debug {debug}, run {i}): {out:?}");
            assert_eq!(leaked(&out), 0, "{tag} (debug {debug}, run {i}): {out:?}");
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// The parent and four tasks read one list at the same time, then drop
/// it: the list's cells are counted from five threads at once.
const SHARED_LIST: &str = r#"
module Main where
  import Data.List

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  sumL :: List Int -> Int -> Int
  sumL xs acc = case xs of
    Nil -> acc
    Cons h t -> sumL t (acc + h)

  sweep :: List Int -> Int -> Int -> Int
  sweep xs k acc = if k == 0 then acc else sweep xs (k - 1) (acc + sumL xs 0)

  work :: forall r. using (io :: Io r) => List Int ->{Async r, Cancel r, Diverge} Result LoadError Int
  work xs = withScope $ \s -> do
    a <- spawn s (\() -> Ok (sweep xs 150 0))
    b <- spawn s (\() -> Ok (sweep xs 150 0))
    c <- spawn s (\() -> Ok (sweep xs 150 0))
    d <- spawn s (\() -> Ok (sweep xs 150 0))
    m <- sweep xs 150 0
    case joinAllOrCancel s a b c d of
      Ok r -> case r of
        (p, q, v, w) -> Ok (p + q + v + w + m)
      Err e -> Err e

  main :: Unit ->{Async r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work (build 1000 Nil) of
    Ok n -> if n == 5 * 150 * 500500 then 0 else 1
    Err e -> 2
"#;

#[test]
fn tasks_share_a_list_with_their_parent_and_drop_it() {
    expect_clean("shared_list", SHARED_LIST, 20);
}

/// Task results cross back to the parent: one task's join is its own
/// `Ok` block; two tasks' join is a tuple whose map follows the payloads
/// (a list, counted, beside a raw `Bool`); a failed join keeps the `Err`
/// and drops the other member's list.
const JOINED_RESULTS: &str = r#"
module Main where
  import Data.List

  build :: Int -> List Int -> List Int
  build n acc = if n == 0 then acc else build (n - 1) (Cons n acc)

  sumL :: List Int -> Int -> Int
  sumL xs acc = case xs of
    Nil -> acc
    Cons h t -> sumL t (acc + h)

  one :: forall r. using (io :: Io r) => Int ->{Async r, Cancel r, Diverge} Result LoadError (List Int)
  one n = withScope $ \s -> do
    h <- spawn s (\() -> Ok (build n Nil))
    joinAllOrCancel s h

  two :: forall r. using (io :: Io r) => Int ->{Async r, Cancel r, Diverge} Result LoadError (List Int, Bool)
  two n = withScope $ \s -> do
    h <- spawn s (\() -> Ok (build n Nil))
    g <- spawn s (\() -> Ok (n > 10))
    joinAllOrCancel s h g

  bad :: forall r. using (io :: Io r) => Int ->{Async r, Cancel r, Diverge} Result LoadError (List Int, List Int)
  bad n = withScope $ \s -> do
    h <- spawn s (\() -> Ok (build n Nil))
    g <- spawn s (\() -> Err (LoadError 7))
    joinAllOrCancel s h g

  fromOne :: Result LoadError (List Int) -> Int
  fromOne r = case r of
    Ok xs -> sumL xs 0
    Err e -> 0 - 1000000

  fromTwo :: Result LoadError (List Int, Bool) -> Int
  fromTwo r = case r of
    Ok p -> case p of
      (xs, big) -> if big then sumL xs 0 else 0 - 1000000
    Err e -> 0 - 1000000

  fromBad :: Result LoadError (List Int, List Int) -> Int
  fromBad r = case r of
    Ok p -> 0 - 1000000
    Err e -> case e of
      LoadError k -> if k == 7 then 7 else 0 - 1000000

  round :: forall r. using (io :: Io r) => Int ->{Async r, Cancel r, Diverge} Int
  round n = fromOne (one n) + fromTwo (two n) + fromBad (bad n)

  rounds :: forall r. using (io :: Io r) => Int -> Int ->{Async r, Cancel r, Diverge} Int
  rounds k acc = if k == 0 then acc else rounds (k - 1) (acc + round 1000)

  main :: Unit ->{Async r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in if rounds 50 0 == 50 * (2 * 500500 + 7) then 0 else 1
"#;

#[test]
fn a_child_returns_fresh_structures_through_join() {
    expect_clean("joined", JOINED_RESULTS, 3);
}

/// The parent's only reference to each tree moves into four tasks that
/// finish at different times: the last of them frees it, on its own
/// thread.
const LAST_DROPS: &str = r#"
module Main where
  data Tree where
    Leaf :: Tree
    Node :: Tree -> Int -> Tree -> Tree

  grow :: Int -> Int -> Tree
  grow lo hi = if lo > hi then Leaf else
    let mid = (lo + hi) / 2 in Node (grow lo (mid - 1)) mid (grow (mid + 1) hi)

  total :: Tree -> Int
  total t = case t of
    Leaf -> 0
    Node l v r -> total l + v + total r

  sweep :: Tree -> Int -> Int -> Int
  sweep t k acc = if k == 0 then acc else sweep t (k - 1) (acc + total t)

  work :: forall r. using (io :: Io r) => Tree ->{Async r, Cancel r, Diverge} Result LoadError Int
  work t = withScope $ \s -> do
    a <- spawn s (\() -> Ok (sweep t 10 0))
    b <- spawn s (\() -> Ok (sweep t 20 0))
    c <- spawn s (\() -> Ok (sweep t 30 0))
    d <- spawn s (\() -> Ok (sweep t 40 0))
    case joinAllOrCancel s a b c d of
      Ok r -> case r of
        (p, q, v, w) -> Ok (p + q + v + w)
      Err e -> Err e

  rounds :: forall r. using (io :: Io r) => Int -> Int ->{Async r, Cancel r, Diverge} Int
  rounds k acc = if k == 0 then acc else case work (grow 1 2047) of
    Ok n -> rounds (k - 1) (acc + n)
    Err e -> 0 - 1

  main :: Unit ->{Async r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in if rounds 4 0 == 4 * 100 * 2096128 then 0 else 1
"#;

#[test]
fn the_last_task_to_finish_frees_a_shared_tree() {
    expect_clean("last_drops", LAST_DROPS, 5);
}
