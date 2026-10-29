//! Effect.Pty consumer semantics: a real pseudo-terminal round trip
//! through the real compiler pipeline.
//!
//! Each test compiles a real project: the module under test
//! (`compiler/prelude/Effect/Pty.cinch`) written beside `Main.cinch` and
//! driven through the CLI pipeline like `runtime_net_prims.rs`. The
//! checks are a bitmask: check k contributes 2^(k-1) and `main` sums
//! them, so a nonzero exit code names the failed checks.
//!
//! The acceptance scenario: spawn `/bin/sh` in a pseudo-terminal, write
//! `echo hi\n`, read until the transcript contains `hi`, write `exit\n`,
//! wait, and assert the exit status. A pseudo-terminal echoes and may
//! interleave its input with the child's output, so the check matches on
//! the substring's presence rather than an exact buffer (the module's
//! header states the same law). A read and a write after the close are
//! the named error, and so is an invented handle.

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

/// The module under test: the `Effect.Pty` home with its intrinsic
/// declarations.
const PTY: &str = include_str!("../prelude/Effect/Pty.cinch");

fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_ptytest_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [("Pty.cinch", PTY), ("Main.cinch", main_src)] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    let path = dir.join("Main.cinch");
    // The checker's elaboration recursion runs near the default
    // test-thread stack; compile on a thread with a generous stack (as
    // `tests/runtime_net_prims.rs` does).
    let compiled = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver(Mode::Run, &path, None).map_err(|e| e.to_string()))
        .expect("spawn compiler thread")
        .join();
    match compiled {
        Ok(Ok(code)) => code,
        Ok(Err(e)) => panic!("{name}: compilation failed: {e}"),
        Err(_) => panic!("{name}: the compiler thread panicked"),
    }
}

fn expect_ok(name: &str, main_src: &str) {
    let code = run_main(name, main_src);
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

/// The pseudo-terminal round trip, exit status, and named errors.
#[test]
fn pty_round_trip_exit_status_and_named_errors() {
    let src = r#"
module Main where

  import Data.Bytes
  import Data.List
  import Data.Machine
  import Data.Nat
  import Data.Option
  import Data.Path
  import Data.Result
  import Data.String
  import Effect.Pty

  -- The transcript buffer and its filled length. A pseudo-terminal
  -- echoes and may interleave, so the checks accumulate and match the
  -- substring's presence, never an exact buffer.
  data Transcript where
    Transcript :: Bytes -> Nat -> Transcript

  append-bytes :: Transcript -> Bytes -> Transcript
  append-bytes t bs = case t of
    Transcript buf n -> case bytes-len bs of
      m -> Transcript buf (copy-into buf n bs 0 m)

  copy-into :: Bytes -> Nat -> Bytes -> Nat -> Nat -> Nat
  copy-into dst di src si sn = case si == sn of
    True -> di
    False -> case bytes-set dst di (bytes-get src si) of
      _ -> copy-into dst (di + 1) src (si + 1) sn

  has-hi :: Transcript -> Bool
  has-hi t = case t of
    Transcript buf n -> scan-hi buf 0 n

  scan-hi :: Bytes -> Nat -> Nat -> Bool
  scan-hi buf i n = case i + 1 < n of
    True -> case bytes-get buf i == 104 of
      True -> case bytes-get buf (i + 1) == 105 of
        True -> True
        False -> scan-hi buf (i + 1) n
      False -> scan-hi buf (i + 1) n
    False -> False

  -- Read until the transcript holds "hi" or the fuel runs out. A read
  -- returns what one platform call produced (a short read is legal).
  read-until :: forall r. using (io :: Io r) => Pty -> Transcript -> Nat ->{IO r} Result PtyError Transcript
  read-until p t fuel = case fuel == 0 of
    True -> Ok t
    False -> case pty-read p 512 of
      Err e -> Err e
      Ok bs -> case has-hi (append-bytes t bs) of
        True -> Ok (append-bytes t bs)
        False -> read-until p (append-bytes t bs) (fuel - 1)

  -- Open, size the window, spawn sh, write a command, read until "hi",
  -- ask sh to exit, wait for its status, close, and the closed handle.
  check-round-trip :: forall r. using (io :: Io r) => Path ->{IO r} Int
  check-round-trip sh = case pty-open () of
    Err e -> 1
    Ok p -> case pty-resize p 80 24 of
      Err e -> 2
      Ok u -> case pty-spawn p None (pty-command sh Nil) of
        Err e -> 3
        Ok sp -> case pty-write sp (string-to-bytes "echo hi\n") of
          Err e -> 4
          Ok n -> case n == 8 of
            False -> 5
            True -> case read-until sp (Transcript (bytes-new 8192) 0) 64 of
              Err e -> 6
              Ok t -> case has-hi t of
                False -> 7
                True -> case pty-write sp (string-to-bytes "exit\n") of
                  Err e -> 8
                  Ok n2 -> case pty-wait sp of
                    Err e -> 9
                    Ok code -> case code == 0 of
                      False -> 10
                      True -> case pty-close sp of
                        Err e -> 11
                        Ok u2 -> case pty-read sp 16 of
                          Ok b -> 12
                          Err e2 -> case e2 of
                            PtyInvalidArgument -> case pty-write sp (string-to-bytes "x") of
                              Ok n3 -> 13
                              Err e3 -> case e3 of
                                PtyInvalidArgument -> case pty-read (Pty 4096) 16 of
                                  Ok b2 -> 14
                                  Err e4 -> case e4 of
                                    PtyInvalidArgument -> 0
                                    _ -> 15
                                _ -> 16
                            _ -> 17
                        _ -> 18

  -- A window size outside the u16 range is the named argument error, a
  -- program that does not exist is the named not-found error (and the
  -- failed spawn closes the handle), and a handle with no child refuses
  -- a wait.
  check-named :: forall r. using (io :: Io r) => Path ->{IO r} Int
  check-named sh = case pty-open () of
    Err e -> 32
    Ok p -> case pty-resize p 70000 24 of
      Ok u -> 33
      Err e -> case e of
        PtyInvalidArgument -> case pty-wait p of
          Ok code -> 34
          Err e2 -> case e2 of
            PtyInvalidArgument -> case pty-spawn p None (pty-command (path-from-string "/nonexistent/cinch-program") Nil) of
              Ok q -> 35
              Err e3 -> case e3 of
                PtyNotFound -> case pty-close p of
                  Err e4 -> case e4 of
                    PtyInvalidArgument -> 0
                    _ -> 36
                  Ok u2 -> 37
                _ -> 38
            _ -> 39
        _ -> 40

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let sh = path-from-string "/bin/sh" in
    case check-round-trip sh of
      c1 -> case check-named sh of
        c2 -> c1 + c2
"#;
    expect_ok("pty-round-trip", src);
}
