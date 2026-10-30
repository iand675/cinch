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
//! header states the same law). The handle is linear and threaded: every
//! operation takes it and hands it back, `pty-close` consumes it, so the
//! scenario passes the value each call returned and the closed handle
//! leaves the program. A handle word the caller invented is still the
//! named error.
//!
//! The second test pins the discipline the threading buys. A program
//! that uses a handle twice, or on one branch only, or after
//! `pty-close`, is refused by the checker with the multiplicity
//! diagnostic; a program that opens a terminal and never closes it is
//! accepted, which is the honest boundary of what the discipline covers
//! (a local binder defaults to `%many`, so it cannot be annotated).

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

/// The module under test: the `Effect.Pty` home with its intrinsic
/// declarations.
const PTY: &str = include_str!("../prelude/Effect/Pty.cinch");

/// Write the module under test beside `main_src` in a fresh scratch
/// project (the resolution the driver documents: a source module beside
/// the entry shadows the embedded prelude copy of the same name).
fn write_project(name: &str, main_src: &str) -> PathBuf {
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
    dir
}

/// Compile the scratch project in `mode`. The checker's elaboration
/// recursion runs near the default test-thread stack, so the compile
/// happens on a thread with a generous stack (as
/// `tests/runtime_net_prims.rs` does).
fn compile(name: &str, main_src: &str, mode: Mode) -> Result<i32, String> {
    let path = write_project(name, main_src).join("Main.cinch");
    let compiled = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver(mode, &path, None).map_err(|e| e.to_string()))
        .expect("spawn compiler thread")
        .join();
    match compiled {
        Ok(result) => result,
        Err(_) => panic!("{name}: the compiler thread panicked"),
    }
}

fn run_main(name: &str, main_src: &str) -> i32 {
    match compile(name, main_src, Mode::Run) {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_ok(name: &str, main_src: &str) {
    let code = run_main(name, main_src);
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

/// The checker must refuse the program, and its diagnostic must carry
/// every needle.
fn expect_refused(name: &str, main_src: &str, needles: &[&str]) {
    match compile(name, main_src, Mode::Check) {
        Ok(_) => panic!("{name}: expected the checker to refuse the program, but it checked"),
        Err(e) => {
            for n in needles {
                assert!(e.contains(n), "{name}: error {e:?} does not contain {n:?}");
            }
        }
    }
}

/// The checker must accept the program (checked, never run: these are
/// discipline probes, not scenarios).
fn expect_checks(name: &str, main_src: &str) {
    match compile(name, main_src, Mode::Check) {
        Ok(_) => {}
        Err(e) => panic!("{name}: expected the program to check, but: {e}"),
    }
}

/// A misuse probe: the module under test's imports, one declaration and
/// a pure `main`, so the only interesting thing is the declaration.
fn probe(decl: &str) -> String {
    format!(
        "module Main where\n\n  import Data.Bytes\n  import Data.Nat\n  import Data.Result\n  import Effect.Pty\n\n{decl}\n  main :: Int\n  main = 0\n"
    )
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

  -- The transcript so far, exactly as long as what was read. A
  -- pseudo-terminal echoes and may interleave, so the checks accumulate
  -- and match the substring's presence, never an exact buffer. Each
  -- read makes a fresh buffer sized to both parts, so every copy is in
  -- bounds by the buffer's own length.
  data Transcript where
    Transcript :: Bytes -> Transcript

  append-bytes :: Transcript -> Bytes -> Transcript
  append-bytes t bs = case t of
    Transcript buf ->
      let out = bytes-new (bytes-len buf + bytes-len bs) in
      let u = bytes-copy-range out buf 0 0 (bytes-len buf) in
      case bytes-copy-range out bs (bytes-len buf) 0 (bytes-len bs) of
        _ -> Transcript out

  has-hi :: Transcript -> Bool
  has-hi t = case t of
    Transcript buf -> scan-hi buf 0

  scan-hi :: Bytes -> Nat -> Bool
  scan-hi buf i = case i + 1 < bytes-len buf of
    True -> case bytes-get buf i == 104 of
      True -> case bytes-get buf (i + 1) == 105 of
        True -> True
        False -> scan-hi buf (i + 1)
      False -> scan-hi buf (i + 1)
    False -> False

  -- Read until the transcript holds "hi" or the fuel runs out. The
  -- handle is a `%1` parameter, so the loop threads the value the last
  -- read returned; a read returns what one platform call produced (a
  -- short read is legal).
  read-until :: forall r. using (io :: Io r) => Pty %1 -> Transcript -> Nat ->{IO r} (Pty, Result PtyError Transcript)
  read-until p t fuel = case fuel == 0 of
    True -> (p, Ok t)
    False -> case pty-read p 512 of
      (p1, Err e) -> (p1, Err e)
      (p1, Ok bs) -> case has-hi (append-bytes t bs) of
        True -> (p1, Ok (append-bytes t bs))
        False -> read-until p1 (append-bytes t bs) (fuel - 1)

  -- Open, size the window, spawn sh, write a command, read until "hi",
  -- ask sh to exit, wait for its status, close, and the fabricated
  -- handle word. Every operation hands the handle back, so each step
  -- binds the pair the last one answered.
  check-round-trip :: forall r. using (io :: Io r) => Path ->{IO r} Int
  check-round-trip sh = case pty-open () of
    Err e -> 1
    Ok p -> case pty-resize p 80 24 of
      (p1, Err e) -> 2
      (p1, Ok u) -> case pty-spawn p1 None (pty-command sh Nil) of
        Err e -> 3
        Ok sp -> case pty-write sp (string-to-bytes "echo hi\n") of
          (sp1, Err e) -> 4
          (sp1, Ok n) -> case n == 8 of
            False -> 5
            True -> case read-until sp1 (Transcript (bytes-new 0)) 64 of
              (sp2, Err e) -> 6
              (sp2, Ok t) -> case has-hi t of
                False -> 7
                True -> case pty-write sp2 (string-to-bytes "exit\n") of
                  (sp3, Err e) -> 8
                  (sp3, Ok n2) -> case pty-wait sp3 of
                    (sp4, Err e) -> 9
                    (sp4, Ok code) -> case code == 0 of
                      False -> 10
                      True -> case pty-close sp4 of
                        Err e -> 11
                        Ok u2 -> case pty-read (Pty 4096) 16 of
                          (q, Ok b) -> 12
                          (q, Err e2) -> case e2 of
                            PtyInvalidArgument -> 0
                            _ -> 13

  -- A window size outside the u16 range is the named argument error, a
  -- program that does not exist is the named not-found error (and the
  -- failed spawn frees the handle, so the program has nothing left to
  -- close), and a handle with no child refuses a wait.
  check-named :: forall r. using (io :: Io r) => Path ->{IO r} Int
  check-named sh = case pty-open () of
    Err e -> 32
    Ok p -> case pty-resize p 70000 24 of
      (p1, Ok u) -> 33
      (p1, Err e) -> case e of
        PtyInvalidArgument -> case pty-wait p1 of
          (p2, Ok code) -> 34
          (p2, Err e2) -> case e2 of
            PtyInvalidArgument -> case pty-spawn p2 None (pty-command (path-from-string "/nonexistent/cinch-program") Nil) of
              Ok q -> 35
              Err e3 -> case e3 of
                PtyNotFound -> 0
                _ -> 36
            _ -> 37
        _ -> 38

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let sh = path-from-string "/bin/sh" in
    case check-round-trip sh of
      c1 -> case check-named sh of
        c2 -> c1 + c2
"#;
    expect_ok("pty-round-trip", src);
}

/// The threading's guarantee: the checker refuses a handle used twice,
/// used on one branch only, or closed twice, and accepts the program
/// that opens a terminal and never closes it (the boundary the module's
/// header states).
#[test]
fn pty_threaded_handle_discipline() {
    // The pre-read handle used again: the shape the threaded signature
    // forbids, and the shape the pre-threading surface allowed.
    expect_refused(
        "pty-after-read",
        &probe(
            r#"  misuse :: forall r. using (io :: Io r) => Pty %1 ->{IO r} Result PtyError Bytes
  misuse p = case pty-read p 16 of
    (p1, Err e) -> Err e
    (p1, Ok b) -> case pty-read p 16 of
      (p2, r) -> r"#,
        ),
        &["multiplicity %1", "must be used exactly once on every path"],
    );
    // Closed on one branch only: the other path drops the handle.
    expect_refused(
        "pty-close-one-branch",
        &probe(
            r#"  misuse :: forall r. using (io :: Io r) => Bool -> Pty %1 ->{IO r} Result PtyError Nat
  misuse b p = case b of
    True -> case pty-close p of
      Err e -> Err e
      Ok u -> Ok 0
    False -> Ok 0"#,
        ),
        &["multiplicity %1", "unused on some path"],
    );
    // Closed twice: the second close has no handle to consume.
    expect_refused(
        "pty-double-close",
        &probe(
            r#"  misuse :: forall r. using (io :: Io r) => Pty %1 ->{IO r} Result PtyError Unit
  misuse p = case pty-close p of
    Err e -> Err e
    Ok u -> pty-close p"#,
        ),
        &["multiplicity %1", "must be used exactly once on every path"],
    );
    // The honest boundary: a terminal opened and never closed checks,
    // because a local binder defaults to `%many` and cannot be
    // annotated. Closing the last handle is a caller obligation.
    expect_checks(
        "pty-never-closed",
        &probe(
            r#"  leaky :: forall r. using (io :: Io r) => Nat ->{IO r} Result PtyError Nat
  leaky n = case pty-open () of
    Err e -> Err e
    Ok p -> Ok n"#,
        ),
    );
}
