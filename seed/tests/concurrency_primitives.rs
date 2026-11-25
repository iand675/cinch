//! End-to-end tests for Increment 1 concurrency primitives and zero-contention
//! parallelism:
//! - `Effect.Concurrent.Atomic` (`AtomicI64`, `AtomicU64`, `AtomicBool`, `atomic-fence`)
//! - `Effect.Concurrent.RwLock` (concurrent readers + writer swaps over counted values)
//! - `Ref r a` region-scoped shared borrow capture into `parMap`
//! - `parMap` inside `cinch.runtime.arenaEnterPhase` / `cinch.runtime.arenaExitPhase`

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::Mode;

mod common;
use common::drive_with_profile;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

fn project(tag: &str, src: &str) -> PathBuf {
    static N: AtomicU64 = AtomicU64::new(0);
    let n = N.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("cinch_conc_{tag}_{}_{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    std::fs::write(dir.join("Main.cinch"), src).expect("write source");
    dir
}

fn build(dir: &Path, debug: bool) -> PathBuf {
    let exe = dir.join(if debug { "prog_dbg" } else { "prog" });
    let profile = TargetProfile {
        reclaim: true,
        rc_debug: debug,
        ..TargetProfile::native()
    };
    drive_with_profile(
        Mode::Build,
        StageMode::Refresh,
        dir,
        "Main.cinch",
        Some(exe.clone()),
        &profile,
    )
    .unwrap_or_else(|e| panic!("build failed: {e}"));
    exe
}

fn run(exe: &Path, debug: bool, threads: usize, stats: bool) -> Output {
    let mut cmd = Command::new(exe);
    cmd.env("CINCH_HEAP_LIMIT", "512M")
        .env("CINCH_LEAK_CHECK", "1")
        .env("CINCH_THREADS", threads.to_string());
    if stats {
        cmd.env("CINCH_ALLOC_STATS", "1");
    } else {
        cmd.env_remove("CINCH_ALLOC_STATS");
    }
    if debug {
        cmd.env("CINCH_RC_DEBUG", "1");
    }
    let mut child = cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn program");
    let start = Instant::now();
    while child.try_wait().expect("poll program").is_none() {
        if start.elapsed() > Duration::from_secs(30) {
            child.kill().expect("kill hung test program");
            panic!("test program timed out: {:?}", child.wait_with_output());
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    child.wait_with_output().expect("program output")
}

fn leaked(out: &Output) -> u64 {
    let err = String::from_utf8_lossy(&out.stderr);
    let line = err
        .lines()
        .find(|l| l.starts_with("cinch-leak:"))
        .unwrap_or_else(|| panic!("no leak line: {err}"));
    line.split_whitespace()
        .find_map(|w| w.strip_prefix("leaked="))
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| panic!("no leaked= in {line}"))
}

fn expect_clean(tag: &str, src: &str, runs: usize) {
    let dir = project(tag, src);
    for debug in [false, true] {
        let exe = build(&dir, debug);
        for _ in 0..runs {
            let out = run(&exe, debug, 4, false);
            assert!(
                out.status.success(),
                "{tag} (debug={debug}) failed with {:?}\nstdout: {}\nstderr: {}",
                out.status.code(),
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr),
            );
            assert_eq!(
                leaked(&out),
                0,
                "{tag} (debug={debug}) leaked blocks: {}",
                String::from_utf8_lossy(&out.stderr)
            );
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn atomic_multithreaded_fetch_add_cas_and_bool() {
    expect_clean(
        "atomic_mt",
        r#"
module Main where
  import Effect.Concurrent.Atomic
  import Data.Machine

  bump :: AtomicI64 -> I64 ->{MVar, Diverge} I64
  bump counter n = if n == 0 then 0 else case atomic-i64-fetch-add counter 1 AcqRel of
    _ -> bump counter (wrapping-sub-i64 n 1)

  cas-add :: AtomicI64 -> I64 ->{MVar, Diverge} Unit
  cas-add counter delta =
    let cur = atomic-i64-load counter Acquire in
    if atomic-i64-cas counter cur (wrapping-add-i64 cur delta) AcqRel Acquire then ()
    else cas-add counter delta

  cas-loop :: AtomicI64 -> I64 ->{MVar, Diverge} I64
  cas-loop counter n = if n == 0 then 0 else case cas-add counter 2 of
    _ -> cas-loop counter (wrapping-sub-i64 n 1)

  work :: forall r. using (io :: Io r) => AtomicI64 -> AtomicI64 ->{Async r, Cancel r, Diverge, MVar} Result LoadError (I64, I64, I64, I64)
  work c1 c2 = withScope $ \s -> do
    a <- spawn s (\() -> Ok (bump c1 2000))
    b <- spawn s (\() -> Ok (bump c1 2000))
    c <- spawn s (\() -> Ok (cas-loop c2 500))
    d <- spawn s (\() -> Ok (cas-loop c2 500))
    joinAllOrCancel s a b c d

  main :: Unit ->{Async r, Cancel r, Diverge, MVar} Int
  main u = with (io = newIO ()) in do
    c1 <- atomic-i64-new 0
    c2 <- atomic-i64-new 0
    flag <- atomic-bool-new False
    u64c <- atomic-u64-new (10 :: U64)
    prev-u <- atomic-u64-fetch-add u64c (5 :: U64) SeqCst
    cur-u <- atomic-u64-load u64c SeqCst
    cas-ok <- atomic-bool-cas flag False True AcqRel Acquire
    cas-no <- atomic-bool-cas flag False True AcqRel Acquire
    _ <- atomic-fence SeqCst
    res <- work c1 c2
    v1 <- atomic-i64-load c1 SeqCst
    v2 <- atomic-i64-load c2 SeqCst
    f <- atomic-bool-load flag Acquire
    pure (case res of
      Ok _ ->
        if v1 == 4000 && v2 == 2000 && f && cas-ok && not cas-no && prev-u == (10 :: U64) && cur-u == (15 :: U64)
        then 0
        else 1
      Err _ -> 2)
"#,
        2,
    );
}

#[test]
fn rwlock_concurrent_readers_and_writer_swaps_counted_values() {
    expect_clean(
        "rwlock_mt",
        r#"
module Main where
  import Effect.Concurrent.RwLock
  import Data.List
  import Data.Machine

  read-loop :: RwLock (List Nat) -> I64 -> Nat ->{MVar, Diverge} Nat
  read-loop lock n acc = if n == 0 then acc else
    let xs = rwlock-read lock in
    read-loop lock (wrapping-sub-i64 n 1) (acc + sum-list 0 xs)

  write-loop :: RwLock (List Nat) -> I64 -> Nat ->{MVar, Diverge} Nat
  write-loop lock n k = if n == 0 then k else
    let old = rwlock-swap lock [1, 2, 3] in
    write-loop lock (wrapping-sub-i64 n 1) (k + sum-list 0 old)

  work :: forall r. using (io :: Io r) => RwLock (List Nat) ->{Async r, Cancel r, Diverge, MVar} Result LoadError (Nat, Nat, Nat, Nat)
  work lock = withScope $ \s -> do
    r1 <- spawn s (\() -> Ok (read-loop lock 400 0))
    r2 <- spawn s (\() -> Ok (read-loop lock 400 0))
    r3 <- spawn s (\() -> Ok (read-loop lock 400 0))
    w1 <- spawn s (\() -> Ok (write-loop lock 200 0))
    joinAllOrCancel s r1 r2 r3 w1

  main :: Unit ->{Async r, Cancel r, Diverge, MVar} Int
  main u = with (io = newIO ()) in do
    lock <- rwlock-new ([1, 2, 3] :: List Nat)
    tr <- rwlock-try-read lock
    tw <- rwlock-try-write lock [1, 2, 3]
    res <- work lock
    _ <- rwlock-write lock [10, 20]
    after <- rwlock-read lock
    pure (case (tr, res) of
      (Some _, Ok (a, b, c, w)) ->
        if tw && a == 2400 && b == 2400 && c == 2400 && w == 1200 && sum-list 0 after == 30
        then 0
        else 1
      _ -> 2)
"#,
        2,
    );
}

#[test]
fn par_map_captures_region_ref_borrow_without_leaks() {
    expect_clean(
        "par_map_ref",
        r#"
module Main where
  import Data.Array
  import Data.List

  data Env where
    MkEnv :: Nat -> List Nat -> Env

  sum-ref-list :: forall r. Nat -> Ref r (List Nat) -> Nat
  sum-ref-list acc xs = case xs of
    Nil -> acc
    Cons x rest -> sum-ref-list (acc + copyOut x) rest

  score :: forall r. Ref r Env -> Nat -> Nat
  score env x = case env of
    MkEnv tag weights -> copyOut tag + sum-ref-list 0 weights + x

  run-with-ref :: forall r. Ref r Env -> Nat
  run-with-ref ref =
    let xs = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16] in
    let ys = parMap (\x -> score ref x) xs in
    sum-list 0 ys

  main :: Int
  main =
    let env = MkEnv 5 [10, 20, 30] in
    let total = run-with-ref (borrow env) in
    -- Each of 16 items adds 5 + 60 = 65; sum(1..16) = 136; 16*65 + 136 = 1176
    if total == 1176 then 0 else 1
"#,
        2,
    );
}

#[test]
fn phase_arena_with_par_map_and_heap_leaves_is_zero_leak_and_immortal() {
    let src = r#"
module Main where
  import Data.Array
  import Data.List
  import Data.String

  foreign import ccall "cinch.runtime.arenaEnterPhase" arena-enter-phase :: I64 -> I64
  foreign import ccall "cinch.runtime.arenaExitPhase" arena-exit-phase :: I64 -> I64

  data SharedTree where
    Leaf :: String -> Nat -> SharedTree
    Node :: SharedTree -> SharedTree -> SharedTree

  tree-weight :: SharedTree -> Nat
  tree-weight t = case t of
    Leaf s n -> bytes-len (string-to-bytes s) + n
    Node l r -> tree-weight l + tree-weight r

  build-tree :: Nat -> SharedTree
  build-tree d =
    if d == 0 then Leaf (string-concat "payload-" (nat-to-string d)) 7
    else Node (build-tree (d - 1)) (build-tree (d - 1))

  phase-work :: Unit -> Int
  phase-work u =
    let arena = arena-enter-phase 262144 in
    let shared = build-tree 7 in
    let xs = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24] in
    let out = parMap (\x -> tree-weight shared + x) xs in
    let total = sum-list 0 out in
    -- 128 leaves * (9 + 7) = 2048 per tree; 24 * 2048 + 300 = 49452
    let status = if total == 49452 then 0 else 1 in
    case arena-exit-phase arena of _ -> status

  main :: Int
  main = phase-work ()
"#;
    let dir = project("phase_arena_par", src);
    for debug in [false, true] {
        let exe = build(&dir, debug);
        let out = run(&exe, debug, 4, true);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            out.status.success(),
            "phase_arena_par (debug={debug}) failed with {:?}\nstderr: {stderr}",
            out.status.code()
        );
        assert_eq!(leaked(&out), 0, "phase_arena_par leaked: {stderr}");
        // Verify arena actually served immortal blocks.
        let arena_line = stderr
            .lines()
            .find(|l| l.contains("arena-blocks="))
            .unwrap_or_else(|| panic!("expected arena-blocks in stderr: {stderr}"));
        let arena_blocks: u64 = arena_line
            .split_whitespace()
            .find_map(|w| w.strip_prefix("arena-blocks="))
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        assert!(
            arena_blocks > 0,
            "expected phase arena to serve >0 blocks, got: {arena_line}"
        );
    }
    let _ = std::fs::remove_dir_all(&dir);
}
