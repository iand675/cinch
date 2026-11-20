//! Native MVar end-to-end tests, including reference-count debugging.

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
    let dir = std::env::temp_dir().join(format!("cinch_mvar_{tag}_{}_{n}", std::process::id()));
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

fn run(exe: &Path, debug: bool, threads: usize) -> Output {
    let mut cmd = Command::new(exe);
    cmd.env("CINCH_HEAP_LIMIT", "512M")
        .env("CINCH_LEAK_CHECK", "1")
        .env_remove("CINCH_ALLOC_STATS");
    cmd.env("CINCH_THREADS", threads.to_string());
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
            child.kill().expect("kill hung MVar program");
            panic!("MVar program timed out: {:?}", child.wait_with_output());
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

/// Build `src` both ways and run each build `runs` times: every run
/// exits 0 with nothing leaked. A program reports which of its checks
/// failed through its exit code.
fn expect_clean(tag: &str, src: &str, runs: usize) {
    let dir = project(tag, src);
    for debug in [false, true] {
        let exe = build(&dir, debug);
        for threads in [1, 4] {
            for i in 0..runs {
                let out = run(&exe, debug, threads);
                assert_eq!(
                    out.status.code(),
                    Some(0),
                    "{tag} (debug {debug}, threads {threads}, run {i}): {out:?}"
                );
                assert_eq!(
                    leaked(&out),
                    0,
                    "{tag} (debug {debug}, threads {threads}, run {i}): {out:?}"
                );
            }
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// `src` is refused by the checker with a message containing `needle`.
fn expect_refused(tag: &str, src: &str, needle: &str) {
    let dir = project(tag, src);
    match common::check(&dir, "Main.cinch") {
        Ok(()) => panic!("{tag}: accepted, expected a refusal containing {needle:?}"),
        Err(e) => assert!(
            e.contains(needle),
            "{tag}: refused with {e:?}, expected {needle:?}"
        ),
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn mvar_full_empty_read_swap_and_nonblocking_operations() {
    expect_clean(
        "mvar_basic",
        r#"
module Main where
  import Effect.Concurrent.MVar

  main :: Unit ->{MVar, Diverge} Int
  main u = do
    m <- mvar-new-empty ()
    empty <- mvar-is-empty m
    absent <- mvar-try-take m
    unread <- mvar-try-read m
    first <- mvar-try-put m (7 :: Nat)
    refused <- mvar-try-put m 99
    a <- mvar-read m
    b <- mvar-read m
    old <- mvar-swap m 8
    now <- mvar-try-read m
    taken <- mvar-take m
    gone <- mvar-is-empty m
    full <- mvar-new (42 :: Nat)
    _ <- mvar-put m 9
    last <- mvar-try-take m
    original <- mvar-take full
    pure (if empty && gone && first && not refused then
      (case (absent, unread, now, last) of
        (None, None, Some 8, Some 9) -> if a == 7 && b == 7 && old == 7 && taken == 8 && original == 42 then 0 else 2
        _ -> 3)
      else 1)
"#,
        1,
    );
}

/// Two-way backpressure on actual scheduler fibers, carrying freshly
/// allocated lists. Also checks publication and ownership of each value.
#[test]
fn mvar_concurrent_handoff_delivers_counted_values_once() {
    expect_clean(
        "mvar_handoff",
        r#"
module Main where
  import Effect.Concurrent.MVar
  import Data.List
  import Data.Machine

  produce :: MVar (List Nat) -> I64 -> Nat ->{MVar, Diverge} Nat
  produce m n x = if n == 0 then 0 else case mvar-put m [x, x + 1] of
    _ -> produce m (wrapping-sub-i64 n 1) (x + 1)

  consume :: MVar (List Nat) -> I64 -> Nat ->{MVar, Diverge} Nat
  consume m n total = if n == 0 then total else
    let xs = mvar-take m in consume m (wrapping-sub-i64 n 1) (total + sum-list 0 xs)

  work :: forall r. using (io :: Io r) => MVar (List Nat) ->{Async r, Cancel r, Diverge, MVar} Result LoadError (Nat, Nat, Nat, Nat)
  work m = withScope $ \s -> do
    a <- spawn s (\() -> Ok (consume m 1000 0))
    b <- spawn s (\() -> Ok (consume m 1000 0))
    c <- spawn s (\() -> Ok (produce m 1000 1))
    d <- spawn s (\() -> Ok (produce m 1000 1001))
    joinAllOrCancel s a b c d

  main :: Unit ->{Async r, Cancel r, Diverge, MVar} Int
  main u = with (io = newIO ()) in
    let m = mvar-new-empty () in case work m of
      Ok p -> (case p of
        (a, b, _, _) -> if a + b == 4004000 then (if mvar-is-empty m then 0 else 3) else 1)
      Err _ -> 2
"#,
        3,
    );
}

#[test]
fn mvar_cancelled_calls_unregister_from_all_wait_queues() {
    expect_clean(
        "mvar_cancel",
        r#"
module Main where
  import Effect.Concurrent.MVar

  cancel-take :: forall r. using (io :: Io r) => MVar Nat ->{Async r, IO r, Cancel r, Diverge, MVar} Result LoadError (Option Nat, Nat)
  cancel-take m = withScope $ \s -> do
    a <- spawn s (\() -> Ok (mvar-take-or-cancel m))
    b <- spawn s (\() -> case ioSleep 10 of
      _ -> Err (LoadError 1))
    joinAllOrCancel s a b

  cancel-read :: forall r. using (io :: Io r) => MVar Nat ->{Async r, IO r, Cancel r, Diverge, MVar} Result LoadError (Option Nat, Nat)
  cancel-read m = withScope $ \s -> do
    a <- spawn s (\() -> Ok (mvar-read-or-cancel m))
    b <- spawn s (\() -> case ioSleep 10 of
      _ -> Err (LoadError 1))
    joinAllOrCancel s a b

  cancel-swap :: forall r. using (io :: Io r) => MVar Nat ->{Async r, IO r, Cancel r, Diverge, MVar} Result LoadError (Option Nat, Nat)
  cancel-swap m = withScope $ \s -> do
    a <- spawn s (\() -> Ok (mvar-swap-or-cancel m 999))
    b <- spawn s (\() -> case ioSleep 10 of
      _ -> Err (LoadError 1))
    joinAllOrCancel s a b

  cancel-put :: forall r. using (io :: Io r) => MVar Nat ->{Async r, IO r, Cancel r, Diverge, MVar} Result LoadError (Bool, Nat)
  cancel-put m = withScope $ \s -> do
    a <- spawn s (\() -> Ok (mvar-put-or-cancel m 999))
    b <- spawn s (\() -> case ioSleep 10 of
      _ -> Err (LoadError 1))
    joinAllOrCancel s a b

  main :: Unit ->{Async r, IO r, Cancel r, Diverge, MVar} Int
  main u = with (io = newIO ()) in do
    m <- mvar-new-empty ()
    a <- cancel-take m
    b <- cancel-read m
    c <- cancel-swap m
    _ <- mvar-put m (42 :: Nat)
    d <- cancel-put m
    value <- mvar-take m
    _ <- mvar-put m 43
    next <- mvar-take m
    empty <- mvar-is-empty m
    pure (case (a, b, c, d) of
      (Err _, Err _, Err _, Err _) -> if value == 42 && next == 43 && empty then 0 else 1
      _ -> 2)
"#,
        3,
    );
}

#[test]
fn an_mvar_of_an_unshareable_value_cannot_cross_into_two_tasks() {
    expect_refused(
        "mvar_unshareable",
        r#"
module Main where
  import Effect.Concurrent.MVar

  size :: MVar Bytes ->{MVar, Diverge} Nat
  size m = bytes-len (mvar-read m)

  work :: forall r. using (io :: Io r) => MVar Bytes ->{Async r, Cancel r, Diverge, MVar} Result LoadError (Nat, Nat)
  work m = withScope $ \s -> do
    a <- spawn s (\() -> Ok (size m))
    b <- spawn s (\() -> Ok (size m))
    joinAllOrCancel s a b

  main :: Int
  main = 0
"#,
        "shareable evidence",
    );
}

#[test]
fn mvar_preserves_raw_wide_and_counted_representations() {
    expect_clean(
        "representations",
        r#"
module Main where
  import Effect.Concurrent.MVar
  import Data.Wide
  import Data.List

  main :: Unit ->{MVar, Diverge} Int
  main u = do
    zero <- mvar-new (0 :: I64)
    z <- mvar-try-take zero
    wide <- mvar-new (18446744073709551617 :: U128)
    a <- mvar-read wide
    b <- mvar-swap wide max-bound-u128
    c <- mvar-try-take wide
    no-wide <- mvar-try-read wide
    _ <- mvar-try-put wide 0
    wide-zero <- mvar-take wide
    big <- mvar-new (18446744073709551617 :: Nat)
    big-read <- mvar-read big
    _ <- mvar-try-put big 18446744073709551618
    big-take <- mvar-take big
    xs <- mvar-new [1, 2, 3]
    rejected <- mvar-try-put xs [99, 100]
    inner <- mvar-new [4, 5, 6]
    nested <- mvar-new inner
    shared-inner <- mvar-take nested
    first <- mvar-read xs
    second <- mvar-read shared-inner
    pure (case (z, c, no-wide) of
      (Some 0, Some max, None) ->
        if eq-u128 a 18446744073709551617 && eq-u128 b a && eq-u128 max max-bound-u128 && eq-u128 wide-zero 0 && big-read == big-take && big-take == 18446744073709551617 && not rejected && sum-list 0 first == 6 && sum-list 0 second == 15 then 0 else 1
      _ -> 2)
"#,
        1,
    );
}

#[test]
fn mvar_immediately_available_cancellable_calls_succeed() {
    expect_clean(
        "cancel_success",
        r#"
module Main where
  import Effect.Concurrent.MVar

  main :: Unit ->{MVar, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in do
    m <- mvar-new (0 :: I64)
    a <- mvar-read-or-cancel m
    b <- mvar-swap-or-cancel m 5
    c <- mvar-take-or-cancel m
    filled <- mvar-put-or-cancel m 10
    value <- mvar-take m
    pure (case (a, b, c) of
      (Some 0, Some 0, Some 5) -> if filled && value == 10 then 0 else 1
      _ -> 2)
"#,
        1,
    );
}

#[test]
fn mvar_access_cannot_be_hidden_in_a_pure_function() {
    expect_refused(
        "pure",
        r#"
module Main where
  import Effect.Concurrent.MVar
  bad :: MVar Nat -> Option Nat
  bad m = mvar-try-read m
  main :: Int
  main = 0
"#,
        "effect rows do not match",
    );
}

#[test]
fn mvar_intrinsics_cannot_be_redeclared_as_pure() {
    expect_refused(
        "pure_intrinsic",
        r#"
module Main where
  foreign import ccall "cinch.runtime.mvarNew" bad :: forall a. a -> MVarCell a
  main :: Int
  main = 0
"#,
        "intrinsic cinch.runtime.mvarNew has type",
    );
}

#[test]
fn mvar_cancellation_racing_handoff_never_loses_or_duplicates_a_value() {
    expect_clean(
        "cancel_race",
        r#"
module Main where
  import Effect.Concurrent.MVar
  import Data.Machine

  work :: forall r. using (io :: Io r) => MVar Nat -> MVar (Option Nat) -> MVar Bool ->{Async r, Cancel r, MVar, Diverge} Result LoadError (Nat, Nat, Nat)
  work m received sent = withScope $ \s -> do
    a <- spawn s (\() -> case mvar-take-or-cancel m of
      value -> case mvar-put received value of
        _ -> Ok 0)
    b <- spawn s (\() -> case mvar-put-or-cancel m 42 of
      success -> case mvar-put sent success of
        _ -> Ok 0)
    c <- spawn s (\() -> Err (LoadError 1))
    joinAllOrCancel s a b c

  race :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, MVar, Diverge} Bool
  race u = do
    m <- mvar-new-empty ()
    received <- mvar-new-empty ()
    sent <- mvar-new-empty ()
    outcome <- work m received sent
    got <- mvar-take received
    put <- mvar-take sent
    left <- mvar-try-take m
    pure (case outcome of
      Ok _ -> False
      Err _ -> case (put, got, left) of
        (True, Some 42, None) -> True
        (True, None, Some 42) -> True
        (False, None, None) -> True
        _ -> False)

  repeat :: forall r. using (io :: Io r) => I64 ->{Async r, Cancel r, MVar, Diverge} Bool
  repeat n = if n == 0 then True else if race () then repeat (wrapping-sub-i64 n 1) else False

  main :: Unit ->{Async r, Cancel r, MVar, Diverge} Int
  main u = with (io = newIO ()) in if repeat 200 then 0 else 1
"#,
        3,
    );
}
