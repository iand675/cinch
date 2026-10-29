//! Effect.Signal consumer semantics: the host POSIX number table, the
//! zero-signal probe, and the named delivery errors.
//!
//! Each test compiles a real project: the module under test
//! (`compiler/prelude/Effect/Signal.cinch`) written beside `Main.cinch`
//! and driven through the CLI pipeline like `runtime_prims.rs`. The
//! checks are a bitmask: check k contributes 2^(k-1) and `main` sums
//! them, so a nonzero exit code names the failed checks, and every
//! failure mode of one check shares its bit (total stays under 256).
//!
//! PLATFORM ASSUMPTION: the module's numbers are the HOST's POSIX table
//! (`Effect.Signal`'s header). SIGHUP 1, SIGINT 2, SIGKILL 9 and
//! SIGTERM 15 are identical on Darwin and Linux, so the strict pins
//! below are honest everywhere. `SigUsr1`/`SigUsr2` are NOT shipped:
//! their numbers differ by platform (Darwin 30/31, Linux 10/12, verified
//! with `clang -E` on the CI host) and no runtime intrinsic supplies a
//! portable mapping, so a single source spelling would send the wrong
//! signal on one platform.
//!
//! There is no `getpid` intrinsic: the current pid is read through the
//! `Effect.Process` capability (`sh -c 'echo $PPID'`), as
//! `compiler/tests/runtime_prims.rs` does.

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const SIGNAL: &str = include_str!("../prelude/Effect/Signal.cinch");

fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_signaltest_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    std::fs::write(dir.join("Signal.cinch"), SIGNAL).expect("write Signal");
    std::fs::write(dir.join("Main.cinch"), main_src).expect("write Main");
    // The checker's elaboration recursion runs near the default
    // test-thread stack; compile on a thread with a generous stack (as
    // `tests/lib_base64.rs` and `tests/runtime_prims.rs` do).
    let path = dir.join("Main.cinch");
    let compiled = std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || driver(Mode::Run, &path, None))
        .expect("spawn compiler thread")
        .join()
        .expect("compiler thread panicked");
    match compiled {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_ok(name: &str, main_src: &str) {
    let code = run_main(name, main_src);
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

/// Every shipped signal's number agrees with the host POSIX table. The
/// four numbers are identical on Darwin and Linux (see the assumption
/// above), so these pins are strict.
#[test]
fn signal_numbers_are_the_host_posix_values() {
    expect_ok(
        "numbers",
        r#"
module Main where

  import Effect.Signal

  check-none :: Unit -> Int
  check-none u = if signal-number SigNone == 0 then 0 else 1

  check-hup :: Unit -> Int
  check-hup u = if signal-number SigHup == 1 then 0 else 2

  check-int :: Unit -> Int
  check-int u = if signal-number SigInt == 2 then 0 else 4

  check-kill :: Unit -> Int
  check-kill u = if signal-number SigKill == 9 then 0 else 8

  check-term :: Unit -> Int
  check-term u = if signal-number SigTerm == 15 then 0 else 16

  main :: Unit -> Int
  main u = check-none () + check-hup () + check-int () + check-kill () + check-term ()
"#,
    );
}

/// The zero-signal probe delivers nothing and reports success: against
/// this process (the pid from `sh -c 'echo $PPID'`) and against pid 0
/// (the caller's process group), `kill(pid, 0)` succeeds.
#[test]
fn signal_probe_succeeds_for_the_own_process_and_the_group() {
    expect_ok(
        "probe",
        r#"
module Main where

  import Data.Bytes
  import Data.List
  import Data.Nat
  import Data.Path
  import Effect.Process
  import Effect.Signal

  is-digit :: Nat -> Bool
  is-digit c = if c >= 48 then c <= 57 else False

  digits-nat :: Bytes -> Nat -> Nat -> Nat
  digits-nat b i acc = case i == bytes-len b of
    True -> acc
    False -> case is-digit (bytes-get b i) of
      True -> digits-nat b (i + 1) (acc * 10 + (bytes-get b i - 48))
      False -> acc

  self-pid :: forall r. using (io :: Io r) => Path ->{IO r, Diverge} Nat
  self-pid sh = case process-run (command sh (Cons "-c" (Cons "echo $PPID" Nil))) "" of
    Err e -> 0
    Ok p -> case p of
      (code, out) -> digits-nat out 0 0

  probe-ok :: forall r. using (io :: Io r) => I64 ->{IO r} Bool
  probe-ok pid = case signal-probe pid of
    Ok u -> True
    Err e -> False

  main :: Unit ->{IO r, Diverge} Int
  main u = with (io = newIO ()) in
    let pid = self-pid (path-from-string "/bin/sh") in
    let c1 = if pid > 0 then 0 else 1 in
    let c2 = if probe-ok (nat-to-i64 pid) then 0 else 2 in
    let c3 = if probe-ok 0 then 0 else 4 in
    c1 + c2 + c3
"#,
    );
}

/// The two failure variants the boundary can produce: a returning `-1`
/// (here, a pid no process can hold, 2147483647, which `kill(2)` answers
/// with ESRCH) is `SignalNotDelivered`; a pid outside the host's `i32`
/// range is the runtime's `INVALID_ARGUMENT` sentinel, `SignalInvalidNumber`.
#[test]
fn signal_send_reports_named_errors_for_absent_and_out_of_range_pids() {
    expect_ok(
        "errors",
        r#"
module Main where

  import Effect.Signal

  not-delivered :: forall r. using (io :: Io r) => I64 ->{IO r} Bool
  not-delivered pid = case signal-send pid SigKill of
    Ok u -> False
    Err e -> case e of
      SignalNotDelivered -> True
      SignalInvalidNumber -> False

  invalid-number :: forall r. using (io :: Io r) => I64 ->{IO r} Bool
  invalid-number pid = case signal-send pid SigInt of
    Ok u -> False
    Err e -> case e of
      SignalInvalidNumber -> True
      SignalNotDelivered -> False

  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let c1 = if not-delivered 2147483647 then 0 else 1 in
    let c2 = if invalid-number 3000000000 then 0 else 2 in
    c1 + c2
"#,
    );
}
