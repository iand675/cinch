//! Platform layer tests (P34): the runtime clocks under the `Io r`
//! capability (decision D45's determinism rule), `Data.Path` and
//! `Effect.Fs` over a real scratch directory (write/read round trip,
//! listing, metadata, atomic rename and the named error vocabulary),
//! `Effect.Process` driving real subprocesses (args, env, stdin and
//! stdout pipes, exit codes), the Cinch test runner (the DX sample:
//! a real program that runs cases as subprocesses and reports a
//! summary), and the `Data.Set`/`Data.HashMap` perf shape (10k
//! inserts, the honest numbers printed from the program's own
//! monotonic clock). The Set/HashMap law pins themselves stay in
//! tests/prelude_split.rs and stay green beside these.
//!
//! Every program is a real `Main` compiled and run through the same
//! driver the CLI uses. Clock readings are asserted inside the
//! programs (the platform's own clock is the only honest meter), and
//! the staging probes pin that a build cannot observe the clock at
//! all.

use cinch::driver::{check_sources, run_source, run_source_io};
use std::path::{Path, PathBuf};
use std::time::Instant;

// ----------------------------------------------------------------
// Shared helpers
// ----------------------------------------------------------------

fn expect_ok(name: &str, src: &str, expected: i32) {
    match run_source(src) {
        Ok(code) => assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}"),
        Err(e) => panic!("{name}: compilation failed: {e}"),
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

/// A fresh scratch directory for one probe (the fs probes own their
/// listings, so each gets an empty directory).
fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("cinch_platform_{tag}_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

/// A path as a Cinch string literal (the fs probes splice their
/// scratch directory into the program text).
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

// ----------------------------------------------------------------
// Clock (Effect.Clock)
// ----------------------------------------------------------------

#[test]
fn clock_readings_never_go_backwards_and_the_wall_clock_answers() {
    // The monotonic ordering pin: a chain of 1000 back-to-back pairs,
    // each read no smaller than the one before it. The wall clock
    // answers a value after the epoch.
    expect_ok(
        "clock-order",
        r#"
module Main where

  import Effect.Clock

  ordered :: forall r. using (io :: Io r) => Nat -> Nat ->{IO r} Bool
  ordered n prev = case n == 0 of
    True -> True
    False -> let cur = clock-monotonic-ns () in
      case prev <= cur of
        False -> False
        True -> ordered (n - 1) cur

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let t0 = clock-monotonic-ns () in
    let w = clock-wall-ns () in
    case ordered 1000 t0 of
      False -> 1
      True -> case w > 0 of
        False -> 2
        True -> 0
"#,
        0,
    );
}

#[test]
fn timeout_cancels_a_delayed_computation() {
    // The duration/timeout probe over the P12 machinery
    // (Effect.Clock's `timeout`): a quick delayed computation runs to
    // its value within a wide deadline (`Finished 7`), and a long one
    // is cancelled at the deadline (`TimedOut`). The cancelled
    // computation observes the request at its `pollCancel` point and
    // writes `C` as its cleanup marker (the concurrency slice's
    // cancellation probe shape), and the program's own monotonic clock
    // confirms it returned long before the computation's own delay.
    expect_io(
        "timeout-cancel",
        r#"
module Main where

  import Data.Bytes
  import Effect.Clock

  setByte :: Bytes -> Nat -> Nat -> Bytes
  setByte b i v = case bytes-set b i v of
    _ -> b

  fill :: Nat -> Bytes
  fill v = setByte (bytes-new 1) 0 v

  putByte :: forall r. using (io :: Io r) => Nat ->{IO r} Unit
  putByte v = ioWrite (fill v)

  -- A quick delayed computation at the worked row: it runs to its
  -- value well within the deadline.
  quick :: forall r. using (io :: Io r) => Unit ->{IO r, Cancel r} Nat
  quick u = case pollCancel () of
    True -> 0
    False -> case ioSleep 1 of
      _ -> 7

  -- The long delayed computation: it spins until cancellation and
  -- writes `C` (the cleanup marker) at its `pollCancel` point.
  spin :: forall r. using (io :: Io r) => Unit ->{IO r, Cancel r} Nat
  spin u = case pollCancel () of
    True -> case putByte 67 of
      _ -> 0
    False -> case ioSleep 5 of
      _ -> spin ()

  main :: Unit ->{Async r, IO r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in
    let t0 = clock-monotonic-ns () in
    case timeout (duration-from-millis 2000) (\v -> quick v) of
      TimedOut -> 1
      Finished x -> case x == 7 of
        False -> 2
        True -> case timeout (duration-from-millis 50) (\v -> spin v) of
          Finished y -> 3
          TimedOut -> let t1 = clock-monotonic-ns () in
            case duration-millis (monotonic-elapsed t0 t1) < 2000 of
              False -> 4
              True -> 0
"#,
        b"",
        0,
        "C",
    );
}

#[test]
fn staging_cannot_observe_the_clock() {
    // D45's determinism rule, pinned both ways: a `stage` body is
    // pure (the capability-minting attempt is a row mismatch), and
    // even a capability-typed stage definition cannot run the clock at
    // compile time (the compile-time interpreter refuses foreign
    // calls), so a program's build output cannot depend on when it
    // compiles.
    expect_err(
        "stage-pure",
        r#"
module Main where

  import Effect.Clock

  stage stamp :: Unit -> Code Nat
  stamp u = quoteNat (with (io = newIO ()) in clock-wall-ns ())

  generate do
    emit (quoteDecls where
      stamped :: Nat
      stamped = $(stamp ()))

  main :: Int
  main = 0
"#,
        "effect rows do not match",
    );
    expect_err(
        "stage-foreign",
        r#"
module Main where

  import Effect.Clock

  stage stamp :: forall r. Unit ->{IO r} Code Nat
  stamp u = quoteNat (with (io = newIO ()) in clock-wall-ns ())

  generate do
    emit (quoteDecls where
      stamped :: Nat
      stamped = $(stamp ()))

  main :: Int
  main = 0
"#,
        "a foreign import cannot run at compile time",
    );
}

// ----------------------------------------------------------------
// Filesystem (Data.Path, Effect.Fs)
// ----------------------------------------------------------------

#[test]
fn fs_write_read_list_metadata_and_named_errors() {
    // The round trip on a real scratch directory: write, read back the
    // full content, stat the file (kind and size), list the directory
    // (the sorted ascending listing), and the named error vocabulary
    // for a missing path (`NotFound`, named in the probe). The checks
    // are one shallow definition each (the checker's per-expression
    // recursion budget on the default test thread is why).
    let dir = scratch("fs_round");
    let src = r#"
module Main where

  import Data.Path
  import Effect.Fs

  check-write-read :: forall r. using (io :: Io r) => Path ->{IO r} Bool
  check-write-read p = case fs-write p (string-to-bytes "hello world") of
    Err e -> False
    Ok _ -> case fs-read p of
      Err e -> False
      Ok b -> case string-from-bytes b of
        None -> False
        Some s -> s == "hello world"

  check-meta :: forall r. using (io :: Io r) => Path ->{IO r} Bool
  check-meta p = case fs-metadata p of
    Err e -> False
    Ok m -> case file-meta-kind m of
      FileIsFile -> file-meta-size m == 11
      _ -> False

  check-list :: forall r. using (io :: Io r) => Path ->{IO r} Bool
  check-list dir = case fs-list dir of
    Err e -> False
    Ok names -> case names of
      Nil -> False
      Cons n1 rest -> case n1 == "a.txt" of
        False -> False
        True -> case rest of
          Nil -> True
          _ -> False

  check-missing :: forall r. using (io :: Io r) => Path ->{IO r} Bool
  check-missing p = case fs-read p of
    Ok _ -> False
    Err e -> case e of
      NotFound -> True
      _ -> False

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let dir = path-from-string __DIR__ in
    let p = path-join dir (path-from-string "a.txt") in
    let missing = path-join dir (path-from-string "missing.txt") in
    case check-write-read p of
      False -> 10
      True -> case check-meta p of
        False -> 11
        True -> case check-list dir of
          False -> 12
          True -> case check-missing missing of
            False -> 13
            True -> 0
"#;
    expect_ok(
        "fs-roundtrip",
        src.replace("__DIR__", &lit(&dir)).as_str(),
        0,
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn fs_rename_is_atomic_for_the_target() {
    // The atomic rename probe: a large new content is staged in a
    // second file and renamed over the target. The target then reads
    // back as the FULL new content (the byte length and the text both
    // match: never a partial), and the staging file is gone (`NotFound`).
    // The atomicity itself is POSIX `rename`'s contract, which the
    // runtime calls; the probe pins the observable end state.
    let dir = scratch("fs_rename");
    let src = r#"
module Main where

  import Data.Path
  import Effect.Fs

  repeat-str :: Nat -> String -> String
  repeat-str n s = case n == 0 of
    True -> ""
    False -> string-concat s (repeat-str (n - 1) s)

  check-staged :: forall r. using (io :: Io r) => Path -> Path -> String ->{IO r} Bool
  check-staged a b big = case fs-write a (string-to-bytes "old") of
    Err e -> False
    Ok _ -> case fs-write b (string-to-bytes big) of
      Err e -> False
      Ok _ -> case fs-rename b a of
        Err e -> False
        Ok _ -> True

  check-target :: forall r. using (io :: Io r) => Path -> String ->{IO r} Bool
  check-target a big = case fs-read a of
    Err e -> False
    Ok rb -> case string-from-bytes rb of
      None -> False
      Some s -> case string-byte-length s == 4000 of
        False -> False
        True -> s == big

  check-gone :: forall r. using (io :: Io r) => Path ->{IO r} Bool
  check-gone b = case fs-read b of
    Ok _ -> False
    Err e -> case e of
      NotFound -> True
      _ -> False

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let dir = path-from-string __DIR__ in
    let a = path-join dir (path-from-string "a.txt") in
    let b = path-join dir (path-from-string "b.txt") in
    let big = repeat-str 4000 "x" in
    case check-staged a b big of
      False -> 10
      True -> case check-target a big of
        False -> 11
        True -> case check-gone b of
          False -> 12
          True -> 0
"#;
    expect_ok(
        "fs-rename-atomic",
        src.replace("__DIR__", &lit(&dir)).as_str(),
        0,
    );
    let _ = std::fs::remove_dir_all(&dir);
}

// ----------------------------------------------------------------
// Processes (Effect.Process)
// ----------------------------------------------------------------

#[test]
fn process_captures_exit_code_stdout_args_and_env() {
    // A real subprocess through the pipes: arguments and environment
    // arrive (`/bin/sh` echoes both), the stdout pipe captures the
    // output, the exit code comes through (`exit 7` reports 7), and
    // stdin is fed to the child (`/bin/cat` round trips the payload).
    expect_ok(
        "process-capture",
        r#"
module Main where

  import Data.List
  import Data.Path
  import Effect.Process

  probe-args-env :: forall r. using (io :: Io r) => Path ->{IO r, Diverge} Bool
  probe-args-env sh =
    let probe = command-with-env sh (Cons "-c" (Cons "echo \"arg:$1 env:$PROBE\"" (Cons "sh" (Cons "A" Nil)))) (Cons ("PROBE", "visible") Nil) in
    case process-run probe "" of
      Err e -> False
      Ok p -> case p of
        (code, o) -> case string-from-bytes o of
          None -> False
          Some s -> code == 0 && s == "arg:A env:visible\n"

  probe-exit :: forall r. using (io :: Io r) => Path ->{IO r, Diverge} Bool
  probe-exit sh = case process-run (command sh (Cons "-c" (Cons "exit 7" Nil))) "" of
    Err e -> False
    Ok p -> case p of
      (code, _) -> code == 7

  probe-stdin :: forall r. using (io :: Io r) => Path ->{IO r, Diverge} Bool
  probe-stdin cat = case process-run (command cat Nil) "payload" of
    Err e -> False
    Ok p -> case p of
      (code, o) -> case string-from-bytes o of
        None -> False
        Some s -> code == 0 && s == "payload"

  main :: Unit ->{IO r, Diverge} Int
  main u = with (io = newIO ()) in
    let sh = path-from-string "/bin/sh" in
    let cat = path-from-string "/bin/cat" in
    case probe-args-env sh of
      False -> 10
      True -> case probe-exit sh of
        False -> 11
        True -> case probe-stdin cat of
          False -> 12
          True -> 0
"#,
        0,
    );
}

#[test]
fn the_cinch_test_runner_reports_a_summary() {
    // The DX sample: a real Cinch test runner. It runs its cases as
    // subprocesses (args, env, exit-code and stdin cases), reports one
    // line per case and a `N passed, M failed` summary, and exits 0
    // exactly when every case passed.
    expect_io(
        "cinch-test-runner",
        r#"
module Main where

  import Data.List
  import Data.Path
  import Effect.Process

  data Case where
    Case :: String -> Command -> String -> Nat -> String -> Case

  report :: forall r. using (io :: Io r) => String -> String ->{IO r} Unit
  report name verdict = ioWrite (string-to-bytes (string-concat (string-concat "case " name) verdict))

  run-case :: forall r. using (io :: Io r) => Case ->{IO r, Diverge} Bool
  run-case c = case c of
    Case name cmd input expected out -> case process-run cmd input of
      Err e -> case report name ": FAIL (spawn error)\n" of
        _ -> False
      Ok p -> case p of
        (code, ob) -> case string-from-bytes ob of
          None -> case report name ": FAIL (bad output)\n" of
            _ -> False
          Some s -> case code == expected && s == out of
            True -> case report name ": ok\n" of
              _ -> True
            False -> case report name ": FAIL\n" of
              _ -> False

  run-all :: forall r. using (io :: Io r) => List Case -> Nat -> Nat ->{IO r, Diverge} (Nat, Nat)
  run-all cases passed failed = case cases of
    Nil -> case ioWrite (string-to-bytes (string-concat (nat-to-string passed) (string-concat " passed, " (string-concat (nat-to-string failed) " failed\n")))) of
      _ -> (passed, failed)
    Cons c rest -> case run-case c of
      True -> run-all rest (passed + 1) failed
      False -> run-all rest passed (failed + 1)

  main :: Unit ->{IO r, Diverge} Int
  main u = with (io = newIO ()) in
    let sh = path-from-string "/bin/sh" in
    let cat = path-from-string "/bin/cat" in
    let cases = Cons (Case "echo-args" (command sh (Cons "-c" (Cons "echo \"$1 $2\"" (Cons "sh" (Cons "x" (Cons "y" Nil)))))) "" 0 "x y\n") (Cons (Case "env-echo" (command-with-env sh (Cons "-c" (Cons "echo \"env:$PROBE\"" Nil)) (Cons ("PROBE", "visible") Nil)) "" 0 "env:visible\n") (Cons (Case "exit-seven" (command sh (Cons "-c" (Cons "exit 7" Nil))) "" 7 "") (Cons (Case "stdin-cat" (command cat Nil) "payload" 0 "payload") Nil))) in
    case run-all cases 0 0 of
      (p, f) -> case f == 0 of
        True -> 0
        False -> 1
"#,
        b"",
        0,
        "case echo-args: ok\ncase env-echo: ok\ncase exit-seven: ok\ncase stdin-cat: ok\n4 passed, 0 failed\n",
    );
}

// ----------------------------------------------------------------
// Set and HashMap (Data.Set, Data.HashMap)
// ----------------------------------------------------------------

#[test]
fn set_and_hashmap_ten_thousand_inserts() {
    // The perf-shape sanity probe: 10k inserts into the balanced `Set`
    // and 10k into the tree-indexed `HashMap`, each program-timed on
    // its own monotonic clock and printed (the honest numbers are in
    // the test output). The behavioral pins stay in
    // tests/prelude_split.rs; here the shapes must stay linearithmic:
    // the bound is deliberately generous (an unbalanced rebuild of the
    // old stubs would be quadratic here), and the printed numbers are
    // the measurement.
    let src = r#"
module Main where

  import Data.Set
  import Data.HashMap
  import Effect.Clock

  fill-set :: Nat -> Set Nat -> Set Nat
  fill-set n s = case n == 0 of
    True -> s
    False -> fill-set (n - 1) (set-insert n s)

  fill-map :: Nat -> HashMap Nat Nat -> HashMap Nat Nat
  fill-map n m = case n == 0 of
    True -> m
    False -> fill-map (n - 1) (hash-map-insert n (n * 2) m)

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let t0 = clock-monotonic-ns () in
    let s = fill-set 10000 set-empty in
    let t1 = clock-monotonic-ns () in
    let m = fill-map 10000 hash-map-empty in
    let t2 = clock-monotonic-ns () in
    let set-ms = duration-millis (monotonic-elapsed t0 t1) in
    let map-ms = duration-millis (monotonic-elapsed t1 t2) in
    case ioWrite (string-to-bytes (string-concat "set-ms=" (string-concat (nat-to-string set-ms) (string-concat " map-ms=" (string-concat (nat-to-string map-ms) "\n"))))) of
      _ -> case set-size s == 10000 of
        False -> 1
        True -> case set-member 9999 s of
          False -> 2
          True -> case hash-map-lookup 9999 m of
            None -> 3
            Some v -> if v == 19998 then 0 else 4
"#;
    let started = Instant::now();
    let (code, out) = match run_source_io(src, b"") {
        Ok(x) => x,
        Err(e) => panic!("set/hashmap perf: compilation failed: {e}"),
    };
    let wall = started.elapsed();
    assert_eq!(code, 0, "set/hashmap perf: program exited {code} (output {out:?})");
    let parse = |key: &str| -> u64 {
        let rest = out.split(key).nth(1).unwrap_or_else(|| panic!("perf output {out:?} lacks {key}"));
        rest.split(|c: char| !c.is_ascii_digit())
            .next()
            .unwrap_or_default()
            .parse()
            .unwrap_or_else(|_| panic!("perf output {out:?} lacks a number after {key}"))
    };
    let set_ms = parse("set-ms=");
    let map_ms = parse("map-ms=");
    // Honest numbers: the program's own monotonic clock over its own
    // insert phases, printed unconditionally (run with --nocapture to
    // see them on a green run).
    eprintln!("set/hashmap perf: set-ms={set_ms} map-ms={map_ms} (whole run {wall:?})");
    assert!(
        set_ms + map_ms < 60_000,
        "10k inserts should stay linearithmic: set {set_ms}ms + map {map_ms}ms is past the bound"
    );
}
