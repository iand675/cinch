//! Machine time tests (design/datetime.md, lane A): `Data.Time.Duration`
//! (an exact signed nanosecond length), `Data.Time.Monotonic` (a
//! monotonic clock reading), `Data.Time.Timestamp` (an instant on the
//! Unix time line) and `Effect.Time` (the typed clock readers, sleeping,
//! deadlines and the bridge to `Effect.Clock`'s milliseconds).
//!
//! The consumer-visible semantics, per the modules' stated law shapes:
//!   Duration:  the unit constructors and observers (negative values,
//!              truncation toward zero, the signed subsecond part), the
//!              range `-(2^127 - 1) ..= 2^127 - 1` and its refusals,
//!              the saturating twins, total negation, multiplication,
//!              truncating and flooring division, the ordering laws.
//!   Timestamp: the bounds (-9999-01-02T01:59:59Z and
//!              9999-12-30T22:00:00.999999999Z, one nanosecond past
//!              each refused), the Unix conversions with the pre-epoch
//!              floor (1 ns before the epoch is second -1 with
//!              999999999 subsecond nanoseconds), add/sub round trips,
//!              since/until antisymmetry, the order.
//!   Monotonic: elapsed time in both argument orders, moving a reading,
//!              the `I64` refusals, the order.
//!   Effect.Time: non-decreasing monotonic readings, a plausible wall
//!              clock, sleeping at least the duration (and zero for a
//!              negative one), deadlines, `timeout-after`, and the
//!              rounding and saturation of the millisecond bridge.
//!
//! Every program is the real module sources beside a `Main` (the
//! fixtures under `fixtures/time/a/`), compiled and run through the
//! same driver the CLI uses (a multi-file project, since the modules
//! are not embedded in the prelude). Each check is a top-level
//! `check-*` function returning 0 or its bit; `main` sums them, so a
//! nonzero exit code is a bitmask naming the failed checks.

mod common;

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const DURATION: &str = include_str!("../prelude/Data/Time/Duration.cinch");
const MONOTONIC: &str = include_str!("../prelude/Data/Time/Monotonic.cinch");
const TIMESTAMP: &str = include_str!("../prelude/Data/Time/Timestamp.cinch");
const EFFECT_TIME: &str = include_str!("../prelude/Effect/Time.cinch");

const DURATION_UNITS: &str = include_str!("fixtures/time/a/duration_units.cinch");
const DURATION_BOUNDS: &str = include_str!("fixtures/time/a/duration_bounds.cinch");
const DURATION_ARITH: &str = include_str!("fixtures/time/a/duration_arith.cinch");
const TIMESTAMP_BOUNDS: &str = include_str!("fixtures/time/a/timestamp_bounds.cinch");
const TIMESTAMP_ARITH: &str = include_str!("fixtures/time/a/timestamp_arith.cinch");
const MONOTONIC_FIXTURE: &str = include_str!("fixtures/time/a/monotonic.cinch");
const CLOCK_READERS: &str = include_str!("fixtures/time/a/clock_readers.cinch");
const CLOCK_SLEEP_DEADLINE: &str = include_str!("fixtures/time/a/clock_sleep_deadline.cinch");

/// Write the machine-time family beside `Main` in a fresh project
/// directory (each module under its dotted file name) and drive it.
fn drive_main(name: &str, main_src: &str) -> Result<i32, String> {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_time_a_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [
        ("Data.Time.Duration.cinch", DURATION),
        ("Data.Time.Monotonic.cinch", MONOTONIC),
        ("Data.Time.Timestamp.cinch", TIMESTAMP),
        ("Effect.Time.cinch", EFFECT_TIME),
        ("Main.cinch", main_src),
    ] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    common::run_executable(&dir, "Main.cinch")
}

/// Compile and run the project, returning the observed exit code.
fn run_main(name: &str, main_src: &str) -> i32 {
    match drive_main(name, main_src) {
        Ok(code) => code,
        Err(e) => panic!("{name}: compilation failed: {e}"),
    }
}

fn expect_ok(name: &str, main_src: &str) {
    let code = run_main(name, main_src);
    assert_eq!(code, 0, "{name}: check bitmask {code}");
}

fn expect_exit(name: &str, main_src: &str, expected: i32) {
    let code = run_main(name, main_src);
    assert_eq!(code, expected, "{name}: expected exit {expected}, got {code}");
}

/// The program must be refused at compile time with a message
/// containing `needle`.
fn expect_refused(name: &str, main_src: &str, needle: &str) {
    match drive_main(name, main_src) {
        Ok(code) => panic!("{name}: expected a compile error containing `{needle}`, but it ran (exit {code})"),
        Err(e) => assert!(e.contains(needle), "{name}: expected `{needle}` in: {e}"),
    }
}

// -------------------------------------------------------------- Duration

#[test]
fn duration_unit_constructors_and_observers() {
    expect_ok("duration-units", DURATION_UNITS);
}

#[test]
fn duration_bounds_refusals_and_saturation() {
    expect_ok("duration-bounds", DURATION_BOUNDS);
}

#[test]
fn duration_arithmetic_division_and_ordering() {
    expect_ok("duration-arith", DURATION_ARITH);
}

// ------------------------------------------------------------- Timestamp

#[test]
fn timestamp_bounds_and_unix_conversions() {
    expect_ok("timestamp-bounds", TIMESTAMP_BOUNDS);
}

#[test]
fn timestamp_arithmetic_since_until_and_ordering() {
    expect_ok("timestamp-arith", TIMESTAMP_ARITH);
}

// ------------------------------------------------------------- Monotonic

#[test]
fn monotonic_elapsed_add_and_ordering() {
    expect_ok("monotonic", MONOTONIC_FIXTURE);
}

// ----------------------------------------------------------- Effect.Time

#[test]
fn clock_readers_are_ordered_and_plausible() {
    expect_ok("clock-readers", CLOCK_READERS);
}

#[test]
fn sleeping_deadlines_timeouts_and_the_millisecond_bridge() {
    expect_ok("clock-sleep-deadline", CLOCK_SLEEP_DEADLINE);
}

// ----------------------------------------------------------- the surface

#[test]
fn the_representation_constructors_are_private() {
    // The range invariants rest on this: no importer can build a
    // `Duration`, `Timestamp` or `Monotonic` from a raw scalar except
    // through the exported functions. (`export T` exports a
    // constructor named `T` too, which is why the constructors are
    // named `MkDuration` and so on; neither spelling is reachable here.)
    expect_refused(
        "private-duration-constructor",
        r#"
module Main where
  import Data.Time.Duration

  main :: Int
  main = if duration-is-zero (MkDuration 0) then 0 else 1
"#,
        "MkDuration",
    );
    expect_refused(
        "private-timestamp-constructor",
        r#"
module Main where
  import Data.Time.Timestamp

  main :: Int
  main = if Timestamp 0 == timestamp-epoch then 0 else 1
"#,
        "unknown name Timestamp",
    );
    expect_refused(
        "private-monotonic-constructor",
        r#"
module Main where
  import Data.Time.Monotonic

  main :: Int
  main = if MkMonotonic 0 == monotonic-from-raw-nanos 0 then 0 else 1
"#,
        "MkMonotonic",
    );
}

#[test]
fn a_zero_divisor_is_refused_at_compile_time() {
    // `duration-div` takes a `NonZero I64`, so the literal 0 does not
    // check (no run-time division fault exists to test).
    expect_refused(
        "zero-divisor",
        r#"
module Main where
  import Data.Time.Duration

  main :: Int
  main = if duration-is-zero (duration-div (duration-from-seconds 1) 0) then 0 else 1
"#,
        "NonZero",
    );
}

#[test]
fn a_failed_check_is_named_by_its_bit() {
    // The bitmask convention this file rests on: check k contributes
    // 2^(k-1), so a program whose third check is the only one that
    // trips exits with 4 and nothing else.
    expect_exit(
        "bit-names-the-failed-check",
        r#"
module Main where
  import Data.Time.Duration

  check-a :: Int
  check-a = 0

  check-b :: Int
  check-b = 0

  check-c :: Int
  check-c = if duration-from-seconds 1 == duration-from-millis 999 then 0 else 4

  main :: Int
  main = check-a + check-b + check-c
"#,
        4,
    );
}
