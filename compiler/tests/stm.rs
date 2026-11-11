//! `Effect.Stm` end to end: what a transaction guarantees, observed from
//! programs.
//!
//! Each test compiles a real project that imports the modules under test
//! (`Effect.Stm` and the structures under it) from the base package, as
//! a consumer does. Every program is built with reference counting
//! twice, plainly and as an `rc_debug` build, and runs under
//! `CINCH_LEAK_CHECK=1`; the debug build also runs under
//! `CINCH_RC_DEBUG=1`, where a count operation on a freed block traps and
//! a freed cell reads as poison. A transaction replaces counted values
//! under concurrent readers, so "exits 0 with nothing leaked, in both
//! builds" is the claim that the cell's two rules hold
//! (`compiler/runtime/src/stm.rs`), not a formality.
//!
//! The concurrent programs are sized so that the bug each one names shows
//! on every run when it is present, and they run several times because a
//! race that needs an unlucky schedule is still a race.

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::Mode;

mod common;
use common::drive_with_profile;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

fn project(tag: &str, src: &str) -> PathBuf {
    static N: AtomicU64 = AtomicU64::new(0);
    let n = N.fetch_add(1, Ordering::SeqCst);
    let dir = std::env::temp_dir().join(format!("cinch_stm_{tag}_{}_{n}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("project directory");
    std::fs::write(dir.join("Main.cinch"), src).expect("write source");
    dir
}

fn build(dir: &Path, debug: bool) -> PathBuf {
    let exe = dir.join(if debug { "prog_dbg" } else { "prog" });
    let profile = TargetProfile { reclaim: true, rc_debug: debug, ..TargetProfile::native() };
    drive_with_profile(Mode::Build, StageMode::Refresh, dir, "Main.cinch", Some(exe.clone()), &profile)
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
/// exits 0 with nothing leaked. A program reports which of its checks
/// failed through its exit code.
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

/// `src` is refused by the checker with a message containing `needle`.
fn expect_refused(tag: &str, src: &str, needle: &str) {
    let dir = project(tag, src);
    match common::check(&dir, "Main.cinch") {
        Ok(()) => panic!("{tag}: accepted, expected a refusal containing {needle:?}"),
        Err(e) => assert!(e.contains(needle), "{tag}: refused with {e:?}, expected {needle:?}"),
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// One transaction's reads see its own earlier writes, and what it wrote
/// is there afterwards: the exit code names the first step that was
/// wrong.
#[test]
fn a_transaction_reads_its_own_writes_and_commits_them() {
    expect_clean(
        "own_writes",
        r#"
module Main where
  import Effect.Stm
  import Data.List

  step :: using (seq :: StmSequencing) => TVar Nat -> TVar (List Nat) -> STM Nat
  step n xs = do
    a <- tvar-read n
    _ <- tvar-write n (a + 1)
    b <- tvar-read n
    _ <- tvar-write n (b + 1)
    _ <- tvar-modify xs (\l -> Cons b l)
    l <- tvar-read xs
    old <- tvar-swap n 100
    pure (old + sum-list 0 l)

  main :: Unit ->{STM, Diverge} Int
  main u =
    let n = tvar-new-io (40 :: Nat) in
    let xs = tvar-new-io [1, 2, 3] in
    let r = atomically (step n xs) in
    if r == 89 then
      (if tvar-read-io n == 100 then
        (if sum-list 0 (tvar-read-io xs) == 47 then 0 else 3)
       else 2)
    else 1
"#,
        1,
    );
}

/// `or-else` runs its second transaction only when the first would wait,
/// and nothing the first wrote survives; `retry` inside it does not block.
/// `tvar-state` threads a result out of the transition. `Data.Option`
/// exports an `or-else` of its own, so a module that wants both imports
/// one of them `hiding` it.
#[test]
fn or_else_discards_the_branch_that_would_wait() {
    expect_clean(
        "or_else",
        r#"
module Main where
  import Effect.Stm
  import Data.Option hiding (or-else)
  import Category.Alternative

  spoil :: using (seq :: StmSequencing) => TVar Nat -> STM Nat
  spoil t = do
    _ <- tvar-write t 999
    retry

  main :: Unit ->{STM, Diverge} Int
  main u =
    let t = tvar-new-io (7 :: Nat) in
    let a = atomically (or-else (spoil t) (tvar-read t)) in
    let b = atomically (or-else (tvar-read t) (spoil t)) in
    let c = atomically (spoil t <|> empty <|> stm-map (\x -> x + 1) (tvar-read t)) in
    let d = atomically (tvar-state t (\x -> (x * 3, x + 5))) in
    if a == 7 then
      (if b == 7 then
        (if c == 8 then
          (if d == 21 then (if tvar-read-io t == 12 then 0 else 5) else 4)
         else 3)
       else 2)
    else 1
"#,
        1,
    );
}

/// Four tasks add to one variable 20000 times each. A lost update (two
/// commits reading the same value) leaves the total short.
#[test]
fn concurrent_increments_are_not_lost() {
    expect_clean(
        "increments",
        r#"
module Main where
  import Effect.Stm
  import Data.Machine

  bump :: TVar Nat -> I64 ->{STM, Diverge} Nat
  bump t n = if n == 0 then 0 else case atomically (tvar-modify t (\x -> x + 1)) of
    _ -> bump t (wrapping-sub-i64 n 1)

  work :: forall r. using (io :: Io r) => TVar Nat ->{Async r, Cancel r, Diverge, STM} Result LoadError (Nat, Nat, Nat, Nat)
  work t = withScope $ \s -> do
    a <- spawn s (\() -> Ok (bump t 20000))
    b <- spawn s (\() -> Ok (bump t 20000))
    c <- spawn s (\() -> Ok (bump t 20000))
    d <- spawn s (\() -> Ok (bump t 20000))
    joinAllOrCancel s a b c d

  main :: Unit ->{Async r, Cancel r, Diverge, STM} Int
  main u = with (io = newIO ()) in
    let t = tvar-new-io (0 :: Nat) in
    case work t of
      Ok _ -> if tvar-read-io t == 80000 then 0 else 1
      Err _ -> 2
"#,
        3,
    );
}

/// Three tasks move units between eight accounts while a fourth sums all
/// eight in one transaction, 20000 times. Every sum is the constant total:
/// a transaction that could observe one account after a transfer and
/// another before it would see a different one.
#[test]
fn a_transaction_sees_one_snapshot_of_many_variables() {
    expect_clean(
        "snapshot",
        r#"
module Main where
  import Effect.Stm
  import Data.Machine

  nth-or :: List (TVar Nat) -> I64 -> TVar Nat -> TVar Nat
  nth-or xs i d = case xs of
    Nil -> d
    Cons x rest -> if i == 0 then x else nth-or rest (wrapping-sub-i64 i 1) d

  transfer :: using (seq :: StmSequencing) => TVar Nat -> TVar Nat -> STM Unit
  transfer src dst = do
    a <- tvar-read src
    if a == 0 then pure () else do
      _ <- tvar-write src (a - 1)
      tvar-modify dst (\b -> b + 1)

  next :: I64 -> I64
  next s = bit-and-i64 (wrapping-add-i64 (wrapping-mul-i64 s 6364136223846793005) 1442695040888963407) 9223372036854775807

  pick :: I64 -> I64
  pick s = bit-and-i64 (wrapping-shr-i64 s 33) 7

  mover :: List (TVar Nat) -> TVar Nat -> I64 -> I64 ->{STM, Diverge} Nat
  mover accts d n seed = if n == 0 then 0 else
    let s1 = next seed in
    let s2 = next s1 in
    case atomically (transfer (nth-or accts (pick s1) d) (nth-or accts (pick s2) d)) of
      _ -> mover accts d (wrapping-sub-i64 n 1) s2

  total :: using (seq :: StmSequencing) => List (TVar Nat) -> STM Nat
  total xs = case xs of
    Nil -> pure 0
    Cons x rest -> do
      a <- tvar-read x
      b <- total rest
      pure (a + b)

  audit :: List (TVar Nat) -> I64 -> Nat ->{STM, Diverge} Nat
  audit accts n bad = if n == 0 then bad else
    let t = atomically (total accts) in
    audit accts (wrapping-sub-i64 n 1) (if t == 800 then bad else bad + 1)

  work :: forall r. using (io :: Io r) => List (TVar Nat) -> TVar Nat ->{Async r, Cancel r, Diverge, STM} Result LoadError (Nat, Nat, Nat, Nat)
  work accts d = withScope $ \s -> do
    a <- spawn s (\() -> Ok (mover accts d 20000 1))
    b <- spawn s (\() -> Ok (mover accts d 20000 2))
    c <- spawn s (\() -> Ok (mover accts d 20000 3))
    w <- spawn s (\() -> Ok (audit accts 20000 0))
    joinAllOrCancel s a b c w

  accounts :: I64 ->{STM} List (TVar Nat)
  accounts n = if n == 0 then Nil else Cons (tvar-new-io 100) (accounts (wrapping-sub-i64 n 1))

  main :: Unit ->{Async r, Cancel r, Diverge, STM} Int
  main u = with (io = newIO ()) in
    let accts = accounts 8 in
    let d = tvar-new-io 0 in
    case work accts d of
      Ok p -> (case p of
        (_, _, _, bad) -> if bad == 0 then (if atomically (total accts) == 800 then 0 else 3) else 1)
      Err _ -> 2
"#,
        3,
    );
}

/// A consumer waits in `retry` for a one-place slot to fill and a producer
/// waits for it to empty, 5000 times each way. A wake lost once would
/// leave both asleep and the test hung; an element delivered twice or
/// dropped changes the sum.
#[test]
fn retry_waits_until_a_commit_changes_what_it_read() {
    expect_clean(
        "handoff",
        r#"
module Main where
  import Effect.Stm
  import Effect.Stm.TMVar
  import Data.Machine

  produce :: TMVar Nat -> I64 -> Nat ->{STM, Diverge} Nat
  produce slot n x = if n == 0 then 0 else case atomically (tmvar-put slot x) of
    _ -> produce slot (wrapping-sub-i64 n 1) (x + 1)

  consume :: TMVar Nat -> I64 -> Nat ->{STM, Diverge} Nat
  consume slot n acc = if n == 0 then acc else
    let x = atomically (tmvar-take slot) in consume slot (wrapping-sub-i64 n 1) (acc + x)

  work :: forall r. using (io :: Io r) => TMVar Nat ->{Async r, Cancel r, Diverge, STM} Result LoadError (Nat, Nat)
  work slot = withScope $ \s -> do
    a <- spawn s (\() -> Ok (consume slot 5000 0))
    b <- spawn s (\() -> Ok (produce slot 5000 1))
    joinAllOrCancel s a b

  main :: Unit ->{Async r, Cancel r, Diverge, STM} Int
  main u = with (io = newIO ()) in
    let slot = tmvar-new-empty-io () in
    case work slot of
      Ok p -> (case p of
        (total, _) -> if total == 12502500 then 0 else 1)
      Err _ -> 2
"#,
        3,
    );
}

/// A task waiting in `retry` for a variable nobody writes is asked to
/// cancel when its sibling fails. `atomically-or-cancel` returns `None`,
/// the task records that it did, and the join returns the sibling's
/// failure. Without the cancellation wake the join would never return.
#[test]
fn a_cancelled_task_stops_waiting_in_retry() {
    expect_clean(
        "cancel",
        r#"
module Main where
  import Effect.Stm

  wait-for :: using (seq :: StmSequencing) => TVar Nat -> STM Nat
  wait-for t = do
    n <- tvar-read t
    if n == 0 then retry else pure n

  waiter :: forall r. using (io :: Io r) => TVar Nat -> TVar Nat ->{STM, Cancel r, Diverge} Result LoadError Nat
  waiter t seen = case atomically-or-cancel (wait-for t) of
    Some n -> Ok n
    None -> case atomically (tvar-write seen 1) of
      _ -> Ok 0

  work :: forall r. using (io :: Io r) => TVar Nat -> TVar Nat ->{Async r, IO r, Cancel r, Diverge, STM} Result LoadError (Nat, Nat)
  work t seen = withScope $ \s -> do
    a <- spawn s (\() -> waiter t seen)
    b <- spawn s (\() -> case ioSleep 20 of
      _ -> Err (LoadError 2))
    joinAllOrCancel s a b

  main :: Unit ->{Async r, IO r, Cancel r, Diverge, STM} Int
  main u = with (io = newIO ()) in
    let t = tvar-new-io (0 :: Nat) in
    let seen = tvar-new-io (0 :: Nat) in
    case work t seen of
      Ok _ -> 1
      Err e -> (case e of
        LoadError c -> if c == 2 then (if tvar-read-io seen == 1 then 0 else 3) else 2)
"#,
        3,
    );
}

/// Two tasks keep replacing a list in a shared variable while two others
/// read it and sum it, 20000 times each. Every list sums to 100, so a
/// reader that got a list a writer had already released (a count taken
/// outside the pin), or a list that was never marked shared, fails the
/// sum, the leak check, or the debug build's freed-block trap.
#[test]
fn replaced_values_stay_alive_for_the_readers_that_hold_them() {
    expect_clean(
        "replace",
        r#"
module Main where
  import Effect.Stm
  import Data.Machine
  import Data.List

  fresh :: I64 -> List Nat
  fresh k = if bit-and-i64 k 1 == 0 then [10, 20, 30, 40] else [40, 30, 20, 5, 5]

  writer :: TVar (List Nat) -> I64 ->{STM, Diverge} Nat
  writer t n = if n == 0 then 0 else case atomically (tvar-write t (fresh n)) of
    _ -> writer t (wrapping-sub-i64 n 1)

  reader :: TVar (List Nat) -> I64 -> Nat ->{STM, Diverge} Nat
  reader t n bad = if n == 0 then bad else
    let xs = atomically (tvar-read t) in
    reader t (wrapping-sub-i64 n 1) (if sum-list 0 xs == 100 then bad else bad + 1)

  work :: forall r. using (io :: Io r) => TVar (List Nat) ->{Async r, Cancel r, Diverge, STM} Result LoadError (Nat, Nat, Nat, Nat)
  work t = withScope $ \s -> do
    a <- spawn s (\() -> Ok (writer t 20000))
    b <- spawn s (\() -> Ok (writer t 20000))
    c <- spawn s (\() -> Ok (reader t 20000 0))
    d <- spawn s (\() -> Ok (reader t 20000 0))
    joinAllOrCancel s a b c d

  main :: Unit ->{Async r, Cancel r, Diverge, STM} Int
  main u = with (io = newIO ()) in
    let t = tvar-new-io (fresh 0) in
    case work t of
      Ok p -> (case p of
        (_, _, bad1, bad2) -> if bad1 + bad2 == 0 then 0 else 1)
      Err _ -> 2
"#,
        3,
    );
}

/// One transaction creates a variable, stores it in a shared variable and
/// writes to the new variable as well. The second write's value must be
/// marked shared although its variable was private when the commit began;
/// another task then reads through the shared variable while the first
/// keeps replacing the inner value.
#[test]
fn a_variable_published_and_written_in_one_transaction_is_shared_whole() {
    expect_clean(
        "publish",
        r#"
module Main where
  import Effect.Stm
  import Data.Machine
  import Data.List

  renew :: using (seq :: StmSequencing) => TVar (TVar (List Nat)) -> I64 -> STM Unit
  renew outer k = do
    inner <- tvar-new [1]
    _ <- tvar-write outer inner
    tvar-write inner (if bit-and-i64 k 1 == 0 then [50, 50] else [25, 25, 50])

  peek :: using (seq :: StmSequencing) => TVar (TVar (List Nat)) -> STM Nat
  peek outer = do
    inner <- tvar-read outer
    xs <- tvar-read inner
    pure (sum-list 0 xs)

  writer :: TVar (TVar (List Nat)) -> I64 ->{STM, Diverge} Nat
  writer outer n = if n == 0 then 0 else case atomically (renew outer n) of
    _ -> writer outer (wrapping-sub-i64 n 1)

  reader :: TVar (TVar (List Nat)) -> I64 -> Nat ->{STM, Diverge} Nat
  reader outer n bad = if n == 0 then bad else
    let s = atomically (peek outer) in
    reader outer (wrapping-sub-i64 n 1) (if s == 100 then bad else bad + 1)

  work :: forall r. using (io :: Io r) => TVar (TVar (List Nat)) ->{Async r, Cancel r, Diverge, STM} Result LoadError (Nat, Nat, Nat)
  work outer = withScope $ \s -> do
    a <- spawn s (\() -> Ok (writer outer 10000))
    b <- spawn s (\() -> Ok (reader outer 10000 0))
    c <- spawn s (\() -> Ok (reader outer 10000 0))
    joinAllOrCancel s a b c

  main :: Unit ->{Async r, Cancel r, Diverge, STM} Int
  main u = with (io = newIO ()) in
    let outer = tvar-new-io (tvar-new-io [100]) in
    case work outer of
      Ok p -> (case p of
        (_, bad1, bad2) -> if bad1 + bad2 == 0 then 0 else 1)
      Err _ -> 2
"#,
        3,
    );
}

/// A bounded queue of capacity 4 between one producer and two consumers:
/// every element arrives exactly once (the sum), in order per consumer is
/// not claimed, and the producer never finds more than the capacity
/// queued (a write that got past the bound would show in `peak`).
#[test]
fn a_bounded_queue_delivers_each_element_once_within_its_capacity() {
    expect_clean(
        "tbqueue",
        r#"
module Main where
  import Effect.Stm
  import Effect.Stm.TBQueue
  import Data.Machine

  push :: using (seq :: StmSequencing) => TBQueue Nat -> TVar Nat -> Nat -> STM Unit
  push q peak x = do
    _ <- tbqueue-write q x
    n <- tbqueue-length q
    m <- tvar-read peak
    if m < n then tvar-write peak n else pure ()

  produce :: TBQueue Nat -> TVar Nat -> I64 -> Nat ->{STM, Diverge} Nat
  produce q peak n x = if n == 0 then 0 else case atomically (push q peak x) of
    _ -> produce q peak (wrapping-sub-i64 n 1) (x + 1)

  consume :: TBQueue Nat -> I64 -> Nat ->{STM, Diverge} Nat
  consume q n acc = if n == 0 then acc else
    let x = atomically (tbqueue-read q) in consume q (wrapping-sub-i64 n 1) (acc + x)

  work :: forall r. using (io :: Io r) => TBQueue Nat -> TVar Nat ->{Async r, Cancel r, Diverge, STM} Result LoadError (Nat, Nat, Nat)
  work q peak = withScope $ \s -> do
    a <- spawn s (\() -> Ok (consume q 2000 0))
    b <- spawn s (\() -> Ok (consume q 2000 0))
    c <- spawn s (\() -> Ok (produce q peak 4000 1))
    joinAllOrCancel s a b c

  main :: Unit ->{Async r, Cancel r, Diverge, STM} Int
  main u = with (io = newIO ()) in
    let q = tbqueue-new-io 4 in
    let peak = tvar-new-io (0 :: Nat) in
    case work q peak of
      Ok p -> (case p of
        (x, y, _) ->
          if x + y == 8002000 then
            (if tvar-read-io peak <= 4 then (if atomically (tbqueue-is-empty q) then 0 else 4) else 3)
          else 1)
      Err _ -> 2
"#,
        3,
    );
}

/// The unbounded queue keeps arrival order across the point where the
/// front list runs dry and the back list is reversed into it, `unget`
/// puts an element back at the front, and `flush` answers the rest in
/// order.
#[test]
fn a_queue_is_first_in_first_out_across_its_two_lists() {
    expect_clean(
        "tqueue",
        r#"
module Main where
  import Effect.Stm
  import Effect.Stm.TQueue
  import Data.List

  script :: using (seq :: StmSequencing) => TQueue Nat -> STM Nat
  script q = do
    _ <- tqueue-write q 1
    _ <- tqueue-write q 2
    _ <- tqueue-write q 3
    a <- tqueue-read q
    _ <- tqueue-write q 4
    b <- tqueue-read q
    c <- tqueue-peek q
    _ <- tqueue-unget q 9
    d <- tqueue-read q
    rest <- tqueue-flush q
    gone <- tqueue-is-empty q
    pure (if gone then a * 10000 + b * 1000 + c * 100 + d * 10 + (if ordered rest then 2 else 0) else 0)

  ordered :: List Nat -> Bool
  ordered xs = case xs of
    Cons 3 (Cons 4 Nil) -> True
    _ -> False

  main :: Unit ->{STM, Diverge} Int
  main u =
    let q = tqueue-new-io () in
    let r = atomically (script q) in
    let again = atomically (stm-then (tqueue-write q 3) (stm-then (tqueue-write q 4) (tqueue-flush q))) in
    if r == 12392 then (if ordered again then 0 else 2) else 1
"#,
        1,
    );
}

/// A semaphore of two permits around a section four tasks enter 2000
/// times each: the number inside never exceeds two, and every permit is
/// back at the end.
#[test]
fn a_semaphore_admits_no_more_than_its_permits() {
    expect_clean(
        "tsem",
        r#"
module Main where
  import Effect.Stm
  import Effect.Stm.TSem
  import Data.Machine

  enter :: using (seq :: StmSequencing) => TSem -> TVar Nat -> TVar Nat -> STM Unit
  enter sem inside peak = do
    _ <- tsem-wait sem
    n <- tvar-read inside
    _ <- tvar-write inside (n + 1)
    m <- tvar-read peak
    if m < n + 1 then tvar-write peak (n + 1) else pure ()

  leave :: using (seq :: StmSequencing) => TSem -> TVar Nat -> STM Unit
  leave sem inside = do
    n <- tvar-read inside
    _ <- (if 0 < n then tvar-write inside (n - 1) else pure ())
    tsem-signal sem

  visit :: TSem -> TVar Nat -> TVar Nat -> I64 ->{STM, Diverge} Nat
  visit sem inside peak n = if n == 0 then 0 else
    case atomically (enter sem inside peak) of
      _ -> case atomically (leave sem inside) of
        _ -> visit sem inside peak (wrapping-sub-i64 n 1)

  work :: forall r. using (io :: Io r) => TSem -> TVar Nat -> TVar Nat ->{Async r, Cancel r, Diverge, STM} Result LoadError (Nat, Nat, Nat, Nat)
  work sem inside peak = withScope $ \s -> do
    a <- spawn s (\() -> Ok (visit sem inside peak 2000))
    b <- spawn s (\() -> Ok (visit sem inside peak 2000))
    c <- spawn s (\() -> Ok (visit sem inside peak 2000))
    d <- spawn s (\() -> Ok (visit sem inside peak 2000))
    joinAllOrCancel s a b c d

  main :: Unit ->{Async r, Cancel r, Diverge, STM} Int
  main u = with (io = newIO ()) in
    let sem = tsem-new-io 2 in
    let inside = tvar-new-io (0 :: Nat) in
    let peak = tvar-new-io (0 :: Nat) in
    case work sem inside peak of
      Ok _ ->
        if tvar-read-io peak <= 2 then
          (if atomically (tsem-available sem) == 2 then (if tvar-read-io inside == 0 then 0 else 4) else 3)
        else 1
      Err _ -> 2
"#,
        3,
    );
}

/// A transaction is built from pure functions: a continuation that writes
/// to stdout has an effect row, and `stm-bind` takes none.
#[test]
fn io_inside_a_transaction_is_refused() {
    expect_refused(
        "io",
        r#"
module Main where
  import Effect.Stm

  loud :: forall r. using (io :: Io r) => TVar Nat -> STM Nat
  loud t = stm-bind (tvar-read t) (\n -> case ioWrite (bytes-new 1) of
    _ -> stm-pure n)

  main :: Int
  main = 0
"#,
        "effect rows do not match",
    );
}

/// A `do` block's continuations run once per attempt, so a value that
/// must be used exactly once cannot be captured by one.
#[test]
fn a_linear_value_cannot_cross_a_transaction_step() {
    expect_refused(
        "linear",
        r#"
module Main where
  import Effect.Stm

  bad :: using (seq :: StmSequencing) => TVar Nat -> Bytes %1 -> STM Nat
  bad t b = do
    a <- tvar-read t
    pure (a + bytes-len b)

  main :: Int
  main = 0
"#,
        "multiplicity %1",
    );
}

/// A variable is reached from every task that holds it, so sharing one
/// shares its value: a variable of a mutable buffer cannot be captured by
/// two tasks.
#[test]
fn a_variable_of_an_unshareable_value_cannot_cross_into_two_tasks() {
    expect_refused(
        "unshareable",
        r#"
module Main where
  import Effect.Stm

  size :: TVar Bytes ->{STM, Diverge} Nat
  size t = bytes-len (atomically (tvar-read t))

  work :: forall r. using (io :: Io r) => TVar Bytes ->{Async r, Cancel r, Diverge, STM} Result LoadError (Nat, Nat)
  work t = withScope $ \s -> do
    a <- spawn s (\() -> Ok (size t))
    b <- spawn s (\() -> Ok (size t))
    joinAllOrCancel s a b

  main :: Int
  main = 0
"#,
        "shareable evidence",
    );
}
