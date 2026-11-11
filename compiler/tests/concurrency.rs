//! Structured-concurrency tests (P12): structured scopes with `spawn`
//! and task handles, `joinAllOrCancel` with the documented failure
//! policy, task captures under the ownership discipline, the blocking
//! call policy, the `Io r` runtime operations end to end, and tasks as
//! green threads on a bounded number of OS threads (concurrency
//! specification sections 2, 3, 4 and 6, the runtime capabilities,
//! effects and sequencing spec, and design/green-threads.md).

mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

use cinch::check::StageMode;
use cinch::core::TargetProfile;
use cinch::driver::Mode;
use common::source_frontend::{check_source_interfaces, check_sources, run_source, run_source_io, run_source_objects};

fn expect_ok(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_err(name: &str, src: &str, needle: &str) {
    match check_sources(src, "<test>") {
        Ok(_) => panic!("{name}: expected an error containing {needle:?}, but the program checked"),
        Err(e) => {
            let msg = e.to_string();
            assert!(
                msg.contains(needle),
                "{name}: error {msg:?} does not contain {needle:?}"
            );
        }
    }
}

/// Compile, run with `input` on the runtime's input source, and assert
/// the exit code and the captured output.
fn expect_io(name: &str, src: &str, input: &[u8], expected: i32, output: &str) {
    match run_source_io(src, input) {
        Ok((code, out)) => {
            assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}");
            assert_eq!(out, output, "{name}: expected output {output:?}, got {out:?}");
        }
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

/// Compile a C fixture to an object file and return its path (the
/// foreign binding tests' helper).
fn c_fixture(name: &str, src: &str) -> String {
    let dir = std::env::temp_dir().join(format!("slconc-{}-{}", name, std::process::id()));
    std::fs::create_dir_all(&dir).expect("scratch dir");
    let c_path = dir.join(format!("{name}.c"));
    let o_path = dir.join(format!("{name}.o"));
    std::fs::write(&c_path, src).expect("write fixture");
    let clang = std::env::var("CINCH_CLANG").unwrap_or_else(|_| "clang".to_string());
    let status = std::process::Command::new(&clang)
        .arg("-c")
        .arg(&c_path)
        .arg("-o")
        .arg(&o_path)
        .status()
        .expect("run clang");
    assert!(status.success(), "{name}: clang failed to compile the C fixture");
    o_path.to_string_lossy().into_owned()
}

// ------------------------------------------- structured scopes and joins (section 2)

#[test]
fn spawned_task_results_join_correctly() {
    // Two tasks joined in handle order: the join's tuple carries each
    // body's `Ok` payload beside its handle.
    expect_ok(
        "join-pair",
        r#"
module Main where
  work :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Result LoadError (Nat, Nat)
  work u = withScope $ \s -> do
    h <- spawn s (\() -> Ok 20)
    h2 <- spawn s (\() -> Ok 2)
    joinAllOrCancel s h h2

  main :: Unit ->{Async r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    Ok p -> case p of
      (a, b) -> if a * 10 + b == 202 then 0 else 1
    Err e -> 2
"#,
        0,
    );
    // The single-handle shape joins one task's payload.
    expect_ok(
        "join-one",
        r#"
module Main where
  work :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Result LoadError Nat
  work u = withScope $ \s -> do
    h <- spawn s (\() -> Ok 7)
    joinAllOrCancel s h

  main :: Unit ->{Async r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    Ok n -> if n == 7 then 0 else 1
    Err e -> 2
"#,
        0,
    );
}

#[test]
fn dropping_a_handle_is_rejected() {
    // The handle reaches a sink that could drop it: refused at the call.
    expect_err(
        "handle-to-sink",
        r#"
module Main where
  ignore :: forall a. a -> Unit
  ignore x = ()

  bad :: forall r. using (io :: Io r) => Unit ->{Async r, Diverge} Unit
  bad u = withScope $ \s -> ignore (spawn s (\() -> Ok 1))
"#,
        "consumed exactly once",
    );
    // A bound handle that is never joined: refused by its usage
    // obligation (dropping would silently detach execution).
    expect_err(
        "handle-never-joined",
        r#"
module Main where
  bad :: forall r. using (io :: Io r) => Unit ->{Async r, Diverge} Unit
  bad u = withScope $ \s -> case spawn s (\() -> Ok 1) of
    h -> ()
"#,
        "task handle",
    );
}

#[test]
fn task_handles_cannot_escape_their_scope() {
    expect_err(
        "handle-escapes",
        r#"
module Main where
  bad :: forall r. using (io :: Io r) => Unit ->{Async r, Diverge} Nat
  bad u = withScope $ \s -> spawn s (\() -> Ok 1)
"#,
        "leaves its `withScope`",
    );
}

// ------------------------------------------- cancellation and the failure policy (section 4)

#[test]
fn join_all_or_cancel_cancels_siblings_on_failure() {
    // The first task fails with `Err (LoadError 2)`. The sibling is a
    // running loop that observes the cancellation request at its
    // `pollCancel` point, writes `C` to the output sink, and returns:
    // the join waits for that cleanup acknowledgment before returning
    // the first failure.
    expect_io(
        "cancel-sibling",
        r#"
module Main where
  -- One-byte buffer: binding `bytes-new 1` gives `bytes-len b == 1`,
  -- which proves index 0; the value is typed as a byte.
  fill :: InRange 0 255 Nat -> Bytes
  fill v = let b = bytes-new 1 in case bytes-set b 0 v of
    _ -> b

  putByte :: forall r. using (io :: Io r) => InRange 0 255 Nat ->{IO r} Unit
  putByte v = ioWrite (fill v)

  -- A long-running task: cancellation is observed at a defined point
  -- (the `pollCancel` suspension), then cleanup completes.
  spin :: forall r. using (io :: Io r) => Unit ->{IO r, Cancel r} Result LoadError Nat
  spin u = case pollCancel () of
    True -> case putByte 67 of
      _ -> Ok 0
    False -> case ioSleep 5 of
      _ -> spin ()

  work :: forall r. using (io :: Io r) => Unit ->{Async r, IO r, Cancel r, Diverge} Result LoadError (Nat, Nat)
  work u = withScope $ \s -> do
    h <- spawn s (\() -> Err (LoadError 2))
    h2 <- spawn s (\() -> spin ())
    joinAllOrCancel s h h2

  main :: Unit ->{Async r, IO r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    Ok p -> 1
    Err e -> case e of
      LoadError c -> if c == 2 then 0 else 3
"#,
        b"",
        0,
        "C",
    );
}

// ------------------------------------------- task captures under P5 (section 3)

#[test]
fn a_linear_capture_transfers_into_a_task_exactly_once() {
    // The `%1` capture is consumed by the task body exactly once: the
    // transfer is visible to the usage checker at the capture.
    expect_ok(
        "linear-transfer",
        r#"
module Main where
  consume :: forall r. using (io :: Io r) => Bytes %1 ->{IO r} Nat
  consume b = bytes-len b

  work :: forall r. using (io :: Io r) => Bytes %1 ->{Async r, IO r, Cancel r, Diverge} Result LoadError Nat
  work x = withScope $ \s -> do
    h <- spawn s (\() -> Ok (consume x))
    joinAllOrCancel s h

  main :: Unit ->{Async r, IO r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work (bytes-new 3) of
    Ok n -> if n == 3 then 0 else 1
    Err e -> 2
"#,
        0,
    );
}

#[test]
fn double_capture_of_a_linear_value_is_rejected() {
    // Two tasks capture the same `%1` value: it would be consumed twice.
    expect_err(
        "double-capture",
        r#"
module Main where
  consume :: forall r. using (io :: Io r) => Bytes %1 ->{IO r} Nat
  consume b = bytes-len b

  work :: forall r. using (io :: Io r) => Bytes %1 ->{Async r, IO r, Cancel r, Diverge} Result LoadError (Nat, Nat)
  work x = withScope $ \s -> do
    h <- spawn s (\() -> Ok (consume x))
    h2 <- spawn s (\() -> Ok (consume x))
    joinAllOrCancel s h h2
"#,
        "used 2 time(s)",
    );
}

#[test]
fn an_affine_thunk_moves_into_a_task_by_transfer() {
    // The lazy specification's ownership rule (section 5): an affine
    // thunk moves across the task boundary by ownership transfer.
    expect_ok(
        "thunk-transfer",
        r#"
module Main where
  work :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Result LoadError Nat
  work u = let t = delay (\() -> 21) in withScope $ \s -> do
    h <- spawn s (\() -> Ok (force t))
    joinAllOrCancel s h

  main :: Unit ->{Async r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    Ok n -> if n == 21 then 0 else 1
    Err e -> 2
"#,
        0,
    );
}

#[test]
fn a_mut_borrow_capture_is_rejected() {
    // An exclusive borrow cannot cross a task boundary (section 3).
    expect_err(
        "mut-capture",
        r#"
module Main where
  readMut :: forall g. Mut g Nat -> Nat
  readMut m = copyOut m

  bad :: forall r g. using (io :: Io r) => Mut g Nat ->{Async r, Cancel r, Diverge} Result LoadError Nat
  bad m = withScope $ \s -> do
    h <- spawn s (\() -> Ok (readMut m))
    joinAllOrCancel s h
"#,
        "exclusive `Mut` borrow cannot cross a task boundary",
    );
}

#[test]
fn ref_and_copy_captures_are_accepted() {
    // A shared `Ref` borrow is `Share` evidence: read-only access may
    // cross into the task.
    expect_ok(
        "ref-capture",
        r#"
module Main where
  work :: forall r g. using (io :: Io r) => Ref g Nat ->{Async r, Cancel r, Diverge} Result LoadError Nat
  work b = withScope $ \s -> do
    h <- spawn s (\() -> Ok (copyOut b))
    joinAllOrCancel s h

  main :: Unit ->{Async r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work (borrow 41) of
    Ok n -> if n == 41 then 0 else 1
    Err e -> 2
"#,
        0,
    );
    // A `Copy` value is shareable evidence by structure.
    expect_ok(
        "copy-capture",
        r#"
module Main where
  work :: forall r. using (io :: Io r) => Nat ->{Async r, Cancel r, Diverge} Result LoadError Nat
  work n = withScope $ \s -> do
    h <- spawn s (\() -> Ok (n + 1))
    joinAllOrCancel s h

  main :: Unit ->{Async r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work 41 of
    Ok n -> if n == 42 then 0 else 1
    Err e -> 2
"#,
        0,
    );
}

#[test]
fn task_shared_ref_requires_a_shareable_referent() {
    let source = r#"
module Main where
  touch :: forall g. Ref g Bytes -> Nat
  touch b = 42

  bad :: forall r g. using (io :: Io r) => Ref g Bytes ->{Async r, Cancel r, Diverge} Result LoadError Nat
  bad b = withScope $ \s -> do
    h <- spawn s (\() -> Ok (touch b))
    joinAllOrCancel s h
  main :: Int
  main = 0
"#;
    let aggregate = source.replace("Ref g Bytes", "Option (Ref g Bytes)");
    let transferred = source.replace("=> Ref g Bytes ->", "=> Ref g Bytes %1 ->");
    for (name, fixture) in [
        ("shared-ref-bytes", source),
        ("aggregate-ref-bytes", aggregate.as_str()),
        ("transferred-ref-bytes", transferred.as_str()),
    ] {
        expect_err(name, fixture, "without shareable evidence");
    }
    // The wrapper's type has no reference-shaped argument. The actual
    // constructor field still contains a borrow after the move.
    let nominal = r#"
module Main where
  data Parcel where
    Pack :: forall g. Ref g Bytes -> Parcel
  touch :: Parcel -> Nat
  touch parcel = 42
  inspect :: forall r. using (io :: Io r) => Parcel %1 ->{Async r, Cancel r, Diverge} Result LoadError Nat
  inspect parcel = withScope $ \s -> do
    h <- spawn s (\() -> Ok (touch parcel))
    joinAllOrCancel s h
  main :: Int
  main = 0
"#;
    expect_err("transferred-nominal-ref-bytes", nominal, "without shareable evidence");
    let safe = nominal.replace("Ref g Bytes", "Ref g Nat");
    check_sources(&safe, "<transferred-nominal-ref-nat>")
        .expect("a transferred wrapper may contain a shareable reference");
}

#[test]
fn an_unshared_mutable_capture_is_rejected() {
    // `Bytes` is a mutable buffer: sharing it across tasks without a
    // transfer would race (section 3).
    expect_err(
        "bytes-capture",
        r#"
module Main where
  bad :: forall r. using (io :: Io r) => Bytes ->{Async r, Cancel r, Diverge} Result LoadError Nat
  bad b = withScope $ \s -> do
    h <- spawn s (\() -> Ok (bytes-len b))
    joinAllOrCancel s h
"#,
        "without shareable evidence",
    );
}

#[test]
fn local_task_callbacks_retain_their_captures_through_aliases() {
    let opaque = include_str!("../../bootstrap/programs/task-capture-counterexamples/opaque-mutable-capture.cinch");
    expect_err("opaque-bytes-capture", opaque, "without shareable evidence");

    let aliased = opaque.replace(
        "in withScope",
        "in let alias = callback in withScope",
    ).replace("spawn s callback", "spawn s alias");
    expect_err("aliased-bytes-capture", &aliased, "without shareable evidence");

    let nested = opaque.replace(
        "let callback = \\() -> Ok (bytes-len bytes)",
        "let get-length = \\() -> bytes-len bytes in let callback = \\() -> Ok (get-length ())",
    );
    expect_err("transitive-bytes-capture", &nested, "without shareable evidence");

    let safe = aliased.replace("Bytes ->", "Nat ->").replace("bytes-len bytes", "bytes");
    check_sources(&safe, "safe-task-callback.cinch").expect("aliased callback captures a shareable Nat");
}

#[test]
fn source_task_forwarders_check_actual_captures() {
    let bad = include_str!("../../bootstrap/programs/task-capture-counterexamples/wrapped-mutable-capture.cinch");
    expect_err("forwarded-bytes-capture", bad, "without shareable evidence");
    let safe = include_str!("../../bootstrap/programs/task-capture-counterexamples/wrapper-safe.cinch");
    let run = safe.replace("Bytes ->", "Unit ->").replace(
        "  main :: Int\n  main = 0",
        "  main :: Unit ->{Async r, Cancel r, Diverge} Int\n  main u = with (io = newIO ()) in case work () of\n    Ok n -> if n == 42 then 0 else 1\n    Err e -> 2",
    );
    expect_ok("forwarded-closed-callback", &run, 0);
    let interfaces = check_source_interfaces(safe, "source-forwarder.cinch").unwrap();
    let main = interfaces.iter().find(|iface| iface.module == "Main").unwrap();
    assert!(!main.exports.values.iter().any(|value| value.name == "fork"));
    let escaped = safe.replace(
        "  main :: Int",
        "  alias :: forall r s x a e. Scope r s -> (Unit ->{|e} Result x a) ->{Async r | e} Task s x a\n  alias = fork\n  main :: Int",
    );
    expect_err("first-class-source-forwarder", &escaped, "must be fully applied");
    let kernel = escaped.replace("alias = fork", "alias = spawn");
    expect_err("first-class-spawn", &kernel, "spawn must be applied directly");
}

#[test]
fn a_thread_shared_memo_cell_is_rejected() {
    // The lazy specification's required rejection case (section 9): a
    // `Lazy` cell is single-threaded by construction and does not cross
    // its region's thread; a shared memo needs an explicit synchronized
    // structure, not this mechanism.
    expect_err(
        "memo-capture",
        r#"
module Main where
  bad :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Result LoadError Nat
  bad u = let ~y = bytes-len (bytes-new 2) in withScope $ \s -> do
    h <- spawn s (\() -> Ok (copyOut (demand y)))
    joinAllOrCancel s h
"#,
        "thread-shared memo",
    );
}

// ------------------------------------------- tasks as green threads (design/green-threads.md)

/// Build `src` as an executable in a project of its own, counting
/// references when `reclaim`.
fn built(tag: &str, src: &str, reclaim: bool) -> (PathBuf, PathBuf) {
    let dir = common::project(tag, &[("Main.cinch", src)]);
    let exe = dir.join("prog");
    let profile = TargetProfile { reclaim, ..TargetProfile::native() };
    common::drive_with_profile(Mode::Build, StageMode::Refresh, &dir, "Main.cinch", Some(exe.clone()), &profile)
        .unwrap_or_else(|e| panic!("{tag}: build: {e}"));
    (dir, exe)
}

/// Run `exe` under `env`. A scheduler bug is usually a hang, so a run
/// that has not finished in two minutes is ended and fails the test.
fn run_bounded(exe: &Path, env: &[(&str, &str)]) -> Output {
    let mut cmd = Command::new(exe);
    cmd.env_remove("CINCH_ALLOC_STATS")
        .env_remove("CINCH_THREADS")
        .env("CINCH_HEAP_LIMIT", "512M")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (k, v) in env {
        cmd.env(k, v);
    }
    let mut child = cmd.spawn().expect("spawn program");
    let limit = Instant::now() + Duration::from_secs(120);
    while child.try_wait().expect("wait for program").is_none() {
        if Instant::now() >= limit {
            let _ = child.kill();
            panic!("{} under {env:?} did not finish in two minutes", exe.display());
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    child.wait_with_output().expect("collect program output")
}

/// What a C fixture adds so that its process has a fixed thread budget
/// and cannot hang: the budget is read when the first task is queued,
/// after every constructor has run.
fn pinned(threads: u32) -> String {
    format!(
        r#"
#include <pthread.h>
#include <stdlib.h>
#include <unistd.h>
__attribute__((constructor)) static void pin_budget(void) {{
    setenv("CINCH_THREADS", "{threads}", 1);
    alarm(120);
}}
"#
    )
}

#[test]
fn one_thread_suspends_a_task_mid_body_and_resumes_it() {
    // The program's thread is the only thread that runs tasks. Task A
    // notes 1 and then waits, in the middle of its body, for a wake only
    // task B gives (the runtime's wait for a blocked transaction, called
    // directly). B starts after A, on the same thread, while A's frames
    // are suspended: it notes 2 and wakes A, which resumes and notes 3.
    // With one OS thread per task this needs two threads; with tasks run
    // to completion on one stack it cannot finish at all.
    let o = c_fixture(
        "onethread",
        &format!(
            r#"{}
static long long noted = 0;
long long fix_note(long long digit) {{ noted = noted * 10 + digit; return noted; }}
long long fix_noted(long long unused) {{ return noted; }}
"#,
            pinned(1)
        ),
    );
    let src = r#"
module Main where
  foreign import ccall "fix_note" note :: I64 ->{IO} I64
  foreign import ccall "fix_noted" noted :: I64 ->{IO} I64
  foreign import ccall "cinch_stm_wait_begin" waitBegin :: I64 ->{IO} I64
  foreign import ccall "cinch_stm_wait_park" waitPark :: I64 ->{IO} I64
  foreign import ccall "cinch_stm_wake" wake :: I64 ->{IO} I64

  waiter :: Unit ->{IO} Result LoadError I64
  waiter u = case note 1 of
    _ -> case waitBegin 4 of
      _ -> case waitPark 0 of
        _ -> Ok (note 3)

  waker :: Unit ->{IO} Result LoadError I64
  waker u = case note 2 of
    _ -> Ok (wake 4)

  work :: forall r. using (io :: Io r) => Unit ->{Async r, IO, Cancel r, Diverge} Result LoadError (I64, I64)
  work u = withScope $ \s -> do
    a <- spawn s (\() -> waiter ())
    b <- spawn s (\() -> waker ())
    joinAllOrCancel s a b

  main :: Unit ->{Async r, IO, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    Ok p -> if noted 0 == 123 then 0 else 1
    Err e -> 2
"#;
    match run_source_objects(src, &[&o]) {
        Ok(code) => assert_eq!(code, 0, "one-thread: expected exit 0, got {code}"),
        Err(e) => panic!("one-thread: compilation failed: {e}"),
    }
}

#[test]
fn ten_thousand_tasks_in_one_scope_join_and_leak_nothing() {
    // Every task is spawned before the first is joined, so ten thousand
    // are alive at once in one scope, on at most four threads. (The
    // spawn sits in a function of its own because a handle bound inside
    // one branch of an `if` is not accepted as joined on every path.)
    let src = r#"
module Main where
  fan :: forall r s. using (io :: Io r) => Scope r s -> Int ->{Async r, Cancel r, Diverge} Result LoadError Int
  fan s n = if n == 0 then Ok 0 else more s n

  more :: forall r s. using (io :: Io r) => Scope r s -> Int ->{Async r, Cancel r, Diverge} Result LoadError Int
  more s n = do
    h <- spawn s (\() -> Ok n)
    case fan s (n - 1) of
      Ok sum -> case joinAllOrCancel s h of
        Ok mine -> Ok (sum + mine)
        Err e -> Err e
      Err e -> case joinAllOrCancel s h of
        Ok mine -> Err e
        Err late -> Err e

  work :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Result LoadError Int
  work u = withScope (\s -> fan s 10000)

  main :: Unit ->{Async r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    Ok total -> if total == 50005000 then 0 else 1
    Err e -> 2
"#;
    let (dir, exe) = built("green_many", src, true);
    for threads in ["4", "1"] {
        let out = run_bounded(&exe, &[("CINCH_THREADS", threads), ("CINCH_LEAK_CHECK", "1")]);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert_eq!(out.status.code(), Some(0), "{threads} threads: {stderr}");
        assert!(
            stderr.contains("cinch-leak: live=0 immortal=0 leaked=0"),
            "{threads} threads: {stderr}"
        );
    }
    common::cleanup(&dir);
}

#[test]
fn the_failure_policy_and_sleeps_hold_on_one_thread() {
    let prelude = r#"
module Main where
  fill :: InRange 0 255 Nat -> Bytes
  fill v = let b = bytes-new 1 in case bytes-set b 0 v of
    _ -> b

  putByte :: forall r. using (io :: Io r) => InRange 0 255 Nat ->{IO r} Unit
  putByte v = ioWrite (fill v)
"#;
    // The sibling polls for cancellation in a loop with no sleep, and is
    // spawned first. On one thread the failing task behind it only runs
    // because the poll point yields, and the sibling only stops because
    // the request that follows is observed at that same point.
    let polling = format!(
        r#"{prelude}
  spin :: forall r. using (io :: Io r) => Unit ->{{IO r, Cancel r}} Result LoadError Nat
  spin u = case pollCancel () of
    True -> case putByte 67 of
      _ -> Ok 0
    False -> spin ()

  work :: forall r. using (io :: Io r) => Unit ->{{Async r, IO r, Cancel r, Diverge}} Result LoadError (Nat, Nat)
  work u = withScope $ \s -> do
    h <- spawn s (\() -> spin ())
    h2 <- spawn s (\() -> Err (LoadError 2))
    joinAllOrCancel s h h2

  main :: Unit ->{{Async r, IO r, Cancel r, Diverge}} Int
  main u = with (io = newIO ()) in case work () of
    Ok p -> 1
    Err e -> case e of
      LoadError c -> if c == 2 then 0 else 3
"#
    );
    let (dir, exe) = built("green_poll", &polling, false);
    for threads in ["1", "2"] {
        let out = run_bounded(&exe, &[("CINCH_THREADS", threads)]);
        assert_eq!(out.status.code(), Some(0), "polling, {threads} threads: {out:?}");
        assert_eq!(String::from_utf8_lossy(&out.stdout), "C", "polling, {threads} threads");
    }
    common::cleanup(&dir);
    // A sleeping task holds no thread: on one thread the task spawned
    // after the sleeper writes first, and the sleeper writes when its
    // time has passed.
    let sleeping = format!(
        r#"{prelude}
  sleeper :: forall r. using (io :: Io r) => Unit ->{{IO r, Cancel r}} Result LoadError Nat
  sleeper u = case ioSleep 30 of
    _ -> case putByte 83 of
      _ -> Ok 1

  quick :: forall r. using (io :: Io r) => Unit ->{{IO r, Cancel r}} Result LoadError Nat
  quick u = case putByte 81 of
    _ -> Ok 2

  work :: forall r. using (io :: Io r) => Unit ->{{Async r, IO r, Cancel r, Diverge}} Result LoadError (Nat, Nat)
  work u = withScope $ \s -> do
    h <- spawn s (\() -> sleeper ())
    h2 <- spawn s (\() -> quick ())
    joinAllOrCancel s h h2

  main :: Unit ->{{Async r, IO r, Cancel r, Diverge}} Int
  main u = with (io = newIO ()) in case work () of
    Ok p -> case p of
      (a, b) -> if a + b == 3 then 0 else 1
    Err e -> 2
"#
    );
    let (dir, exe) = built("green_sleep", &sleeping, false);
    let begun = Instant::now();
    let out = run_bounded(&exe, &[("CINCH_THREADS", "1")]);
    assert_eq!(out.status.code(), Some(0), "sleeping: {out:?}");
    assert_eq!(String::from_utf8_lossy(&out.stdout), "QS", "the sleeper held the thread");
    assert!(begun.elapsed() >= Duration::from_millis(30), "the sleep returned early");
    common::cleanup(&dir);
}

#[test]
fn a_blocking_foreign_call_does_not_hold_up_the_other_tasks() {
    // Two threads, and two tasks that each block one in a `Blocking`
    // foreign call until a third task has run. The third only runs if
    // the generated bracket around the two calls told the scheduler,
    // which then starts a thread beyond the budget for it.
    let o = c_fixture(
        "gate",
        &format!(
            r#"{}
static pthread_mutex_t gate = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t moved = PTHREAD_COND_INITIALIZER;
static int waiting = 0;
static int open = 0;
long long fix_wait_at_gate(long long x) {{
    pthread_mutex_lock(&gate);
    waiting++;
    pthread_cond_broadcast(&moved);
    while (!open) pthread_cond_wait(&moved, &gate);
    pthread_mutex_unlock(&gate);
    return x + 1;
}}
long long fix_open_gate(long long x) {{
    pthread_mutex_lock(&gate);
    while (waiting < 2) pthread_cond_wait(&moved, &gate);
    open = 1;
    pthread_cond_broadcast(&moved);
    pthread_mutex_unlock(&gate);
    return x;
}}
"#,
            pinned(2)
        ),
    );
    let src = r#"
module Main where
  foreign import ccall "fix_wait_at_gate" waitAtGate :: I64 ->{IO, Blocking} I64
  foreign import ccall "fix_open_gate" openGate :: I64 ->{IO, Blocking} I64

  work :: forall r. using (io :: Io r) => Unit ->{Async r, IO, Blocking, Cancel r, Diverge} Result LoadError (I64, I64, I64)
  work u = withScope $ \s -> do
    a <- spawn s (\() -> Ok (waitAtGate 10))
    b <- spawn s (\() -> Ok (waitAtGate 20))
    c <- spawn s (\() -> Ok (openGate 30))
    joinAllOrCancel s a b c

  main :: Unit ->{Async r, IO, Blocking, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    Ok r -> case r of
      (x, y, z) -> if x == 11 then (if y == 21 then (if z == 30 then 0 else 1) else 1) else 1
    Err e -> 2
"#;
    match run_source_objects(src, &[&o]) {
        Ok(code) => assert_eq!(code, 0, "blocking-gate: expected exit 0, got {code}"),
        Err(e) => panic!("blocking-gate: compilation failed: {e}"),
    }
}

// ------------------------------------------- the blocking call policy (section 9)

#[test]
fn blocking_foreign_call_is_accounted_in_the_task_row() {
    let o = c_fixture(
        "blockinc",
        r#"
long long fix_block_inc(long long x) { return x + 1; }
"#,
    );
    // The `Blocking` foreign call blocks only its own task (the call is
    // a blocking region of the scheduler, the offload contract) and its
    // label is accounted in the task body's row, which `spawn` folds
    // into the spawning scope's row and the program's declared row.
    let src = r#"
module Main where
  foreign import ccall "fix_block_inc" blockingInc :: I64 ->{IO, Blocking} I64

  work :: forall r. using (io :: Io r) => Unit ->{Async r, IO, Blocking, Cancel r, Diverge} Result LoadError I64
  work u = withScope $ \s -> do
    h <- spawn s (\() -> Ok (blockingInc 41))
    joinAllOrCancel s h

  main :: Unit ->{Async r, IO, Blocking, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    Ok n -> if n == 42 then 0 else 1
    Err e -> 2
"#;
    match run_source_objects(src, &[&o]) {
        Ok(code) => assert_eq!(code, 0, "blocking-task: expected exit 0, got {code}"),
        Err(e) => panic!("blocking-task: compilation failed: {e}"),
    }
    // The row must account for it: a program declaring a row without
    // `Blocking` is refused by effect honesty.
    expect_err(
        "blocking-unaccounted",
        r#"
module Main where
  foreign import ccall "fix_block_inc" blockingInc :: I64 ->{IO, Blocking} I64

  work :: forall r. using (io :: Io r) => Unit ->{Async r, IO, Cancel r, Diverge} Result LoadError I64
  work u = withScope $ \s -> do
    h <- spawn s (\() -> Ok (blockingInc 41))
    joinAllOrCancel s h
"#,
        "Blocking",
    );
}

// ------------------------------------------- the Io r operations end to end (section 9)

#[test]
fn io_read_and_write_round_trip() {
    // `fetch` (the sketch's vocabulary) reads the runtime's input
    // source through `ioRead`; the program writes the bytes back
    // through `ioWrite`, and the test observes the output.
    expect_io(
        "fetch-round-trip",
        r#"
module Main where
  main :: Unit ->{IO r, Diverge} Int
  main u = with (io = newIO ()) in case fetch (Request 8) of
    Ok b -> case ioWrite b of
      _ -> if bytes-len b == 5 then 0 else 1
    Err e -> 2
"#,
        b"hello",
        0,
        "hello",
    );
    // A short read is the `LoadError` outcome of the sketch.
    expect_io(
        "fetch-short-read",
        r#"
module Main where
  main :: Unit ->{IO r, Diverge} Int
  main u = with (io = newIO ()) in case fetch (Request 8) of
    Ok b -> 1
    Err e -> case e of
      LoadError c -> if c == 1 then 0 else 3
"#,
        b"",
        0,
        "",
    );
}

// ------------------------------------------- the acceptance probe (P15 catalogue)

#[test]
fn example_11_structured_tasks_accepts() {
    let src = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../examples/source/11-structured-tasks.cinch"
    ));
    match check_sources(src, "11-structured-tasks.cinch") {
        Ok(_) => {}
        Err(e) => panic!("11-structured-tasks: rejected: {e}"),
    }
}

#[test]
fn partial_application_cannot_hide_a_scoped_token_in_an_escaping_closure() {
    let helper = r#"
module Main where
  hide :: forall r s. using (io :: Io r) => Scope r s -> Unit ->{Async r, Cancel r, Diverge} Result Int Nat
  hide s u = do
    h <- spawn s (\() -> Ok 7)
    joinAllOrCancel s h
"#;
    for (body, result) in [
        (r"withScope (\s -> hide s)", "Unit ->{Async r, Cancel r, Diverge} Result Int Nat"),
        (r"withScope (\s -> let escaped = hide s in escaped)", "Unit ->{Async r, Cancel r, Diverge} Result Int Nat"),
        (r"withScope (\s -> (hide s, 0))", "(Unit ->{Async r, Cancel r, Diverge} Result Int Nat, Int)"),
    ] {
        expect_err("hidden-scope-capture", &format!("{helper}\n  bad :: forall r. using (io :: Io r) => Unit ->{{Async r, Cancel r, Diverge}} ({result})\n  bad u = {body}\n"), "region escape");
    }
    // Saturating the same helper consumes the scoped token before exit;
    // only its ordinary Result returns, with no retained scope handle.
    expect_ok("saturated-scope-helper", &format!(r#"{helper}
  work :: forall r. using (io :: Io r) => Unit ->{{Async r, Cancel r, Diverge}} Result Int Nat
  work u = withScope (\s -> hide s ())
  main :: Unit ->{{Async r, Cancel r, Diverge}} Int
  main u = with (io = newIO ()) in case work () of
    Ok n -> if n == 7 then 0 else 1
    Err e -> 2
"#), 0);
}

#[test]
fn aggregate_results_cannot_hide_scoped_closure_captures() {
    expect_err("helper-hidden-in-tuple", r#"
module Main where
  hide :: forall r s. using (io :: Io r) => Scope r s -> Unit ->{Async r, Cancel r, Diverge} Result Int Nat
  hide s u = do
    h <- spawn s (\() -> Ok 7)
    joinAllOrCancel s h
  pack :: forall r s. using (io :: Io r) => Scope r s -> (Unit ->{Async r, Cancel r, Diverge} Result Int Nat, Int)
  pack s = (hide s, 0)
  bad :: forall r. using (io :: Io r) => Unit ->{Diverge} (Unit ->{Async r, Cancel r, Diverge} Result Int Nat, Int)
  bad u = withScope (\s -> pack s)
"#, "region escape");
    expect_err("helper-hidden-in-data", r#"
module Main where
  data Holder where
    Holder :: (Unit -> Int) -> Holder
  pack :: forall r s. Scope r s -> Holder
  pack s = Holder (\u -> case s of _ -> 7)
  bad :: forall r. using (io :: Io r) => Unit ->{Diverge} Holder
  bad u = withScope (\s -> pack s)
"#, "region escape");
    expect_ok("scope-helper-returns-ordinary-data", r#"
module Main where
  data NumberBox where
    NumberBox :: Int -> NumberBox
  wrap :: forall r s. Scope r s -> NumberBox
  wrap s = NumberBox 7
  main :: Unit ->{Diverge} Int
  main u = with (io = newIO ()) in case withScope (\s -> wrap s) of
    NumberBox n -> if n == 7 then 0 else 1
"#, 0);
}


#[test]
fn checked_private_and_recursive_data_results_do_not_capture_scope_regions() {
    expect_ok("private-plain-data-not-caller-shadow", r#"
module Boxes where
  export wrap, unbox
  data NumberBox where
    NumberBox :: Int -> NumberBox
  wrap :: forall r s. Scope r s -> NumberBox
  wrap s = NumberBox 7
  unbox :: NumberBox -> Int
  unbox (NumberBox n) = n
module Main where
  import Boxes (wrap, unbox)
  data NumberBox where
    NumberBox :: (Unit -> Int) -> NumberBox
  main :: Unit ->{Diverge} Int
  main u = with (io = newIO ()) in if unbox (withScope (\s -> wrap s)) == 7 then 0 else 1
"#, 0);
    expect_ok("recursive-plain-data-result", r#"
module Main where
  data Tree where
    Leaf :: Int -> Tree
    Branch :: Tree -> Tree -> Tree
  wrap :: forall r s. Scope r s -> Tree
  wrap s = Branch (Leaf 3) (Leaf 4)
  sum :: Tree -> Int
  sum tree = case tree of Leaf n -> n; Branch left right -> sum left + sum right
  main :: Unit ->{Diverge} Int
  main u = with (io = newIO ()) in if sum (withScope (\s -> wrap s)) == 7 then 0 else 1
"#, 0);
}
