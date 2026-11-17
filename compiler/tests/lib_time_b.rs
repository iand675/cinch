//! Datetime lane B (civil time) tests: `Data.Time.Weekday`,
//! `Data.Time.Date`, `Data.Time.TimeOfDay` and `Data.Time.DateTime`
//! (design/datetime.md, layer L1).
//!
//! The consumer-visible semantics, per the modules' stated law shapes:
//!   Weekday:   the ISO (Monday one) and C (Sunday zero) numberings in
//!              both directions, the checked readers' refusals, the
//!              seven-day cycle (`next`, `previous`, `add-days` for
//!              every `I64` including the two extremes),
//!              `days-until`/`days-since` over all 49 pairs, and the
//!              Monday-first order.
//!   Date:      the Gregorian leap rule at its century edges and before
//!              year 1 (1900, 2000, 2100, 0, -4, -100), the month
//!              lengths, every construction refusal, the range ends
//!              (-9999-01-01 and 9999-12-31 exist, one day past each
//!              does not), `date-constrain` and the total `date-of`,
//!              the epoch-day line (anchors, both ends, and walks over
//!              several thousand consecutive days checking the inverse,
//!              weekday succession, `tomorrow`/`yesterday` and the word
//!              order), `add-days`/`diff-days`, weekday anchors,
//!              `day-of-year`, first/last of month and year, month and
//!              year arithmetic with Temporal's "constrain" overflow
//!              (Jiff's vectors), `nth-weekday-of-month`,
//!              `nth-weekday`, and ISO 8601 week dates.
//!   TimeOfDay: field round trips, every construction refusal, the
//!              bounds, the wrapping addition with positive and
//!              negative day carry including the extreme `I64` counts,
//!              the signed difference and the order.
//!   DateTime:  parts and forwarded fields, `add-seconds-nanos`
//!              carrying forward across a month end and backward
//!              across a year start (and refusing at the range ends),
//!              the forwarded date arithmetic, the `I128` nanosecond
//!              difference, the "as if UTC" epoch conversions including
//!              one nanosecond before the epoch, and the order's
//!              agreement with both lines.
//!
//! Every program is the real module sources beside a `Main`, compiled
//! and run through the same driver the CLI uses (a multi-file project,
//! since the modules are not embedded in the prelude). An on-disk
//! module ships beside `Main.cinch` under its dotted name
//! (`Data.Time.Date.cinch`). Each fixture is a complete `module Main`
//! under `fixtures/time/b/`; each check in it is a top-level `check-*`
//! function returning 0 or its bit, and `main` sums them, so a nonzero
//! exit code is a bitmask naming the failed checks (check k contributes
//! bit 2^(k-1); at most eight checks per fixture, so the mask fits a
//! process exit code).

mod common;

use cinch::driver::Mode;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const WEEKDAY: &str = include_str!("../prelude/Data/Time/Weekday.cinch");
const DATE: &str = include_str!("../prelude/Data/Time/Date.cinch");
const TIME_OF_DAY: &str = include_str!("../prelude/Data/Time/TimeOfDay.cinch");
const DATE_TIME: &str = include_str!("../prelude/Data/Time/DateTime.cinch");

const FIX_WEEKDAY: &str = include_str!("fixtures/time/b/weekday.cinch");
const FIX_DATE_CONSTRUCT: &str = include_str!("fixtures/time/b/date_construct.cinch");
const FIX_DATE_EPOCH: &str = include_str!("fixtures/time/b/date_epoch.cinch");
const FIX_DATE_CALENDAR: &str = include_str!("fixtures/time/b/date_calendar.cinch");
const FIX_TIME_OF_DAY: &str = include_str!("fixtures/time/b/time_of_day.cinch");
const FIX_DATETIME: &str = include_str!("fixtures/time/b/datetime.cinch");

/// Compile and run a multi-module project (the four civil modules plus
/// `Main`), returning the observed exit code.
fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_time_b_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [
        ("Data.Time.Weekday.cinch", WEEKDAY),
        ("Data.Time.Date.cinch", DATE),
        ("Data.Time.TimeOfDay.cinch", TIME_OF_DAY),
        ("Data.Time.DateTime.cinch", DATE_TIME),
        ("Main.cinch", main_src),
    ] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    match common::drive(Mode::Run, &dir, "Main.cinch", None) {
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

#[test]
fn weekday_numberings_cycle_and_order() {
    expect_ok("weekday", FIX_WEEKDAY);
}

#[test]
fn date_leap_rule_month_lengths_and_construction_refusals() {
    expect_ok("date-construct", FIX_DATE_CONSTRUCT);
}

#[test]
fn date_epoch_days_round_trip_and_consecutive_day_walks() {
    expect_ok("date-epoch", FIX_DATE_EPOCH);
}

#[test]
fn date_weekdays_constrained_month_arithmetic_and_iso_weeks() {
    expect_ok("date-calendar", FIX_DATE_CALENDAR);
}

#[test]
fn time_of_day_fields_bounds_and_wrapping_addition() {
    expect_ok("time-of-day", FIX_TIME_OF_DAY);
}

#[test]
fn datetime_carrying_arithmetic_and_epoch_conversions() {
    expect_ok("datetime", FIX_DATETIME);
}

#[test]
fn a_failed_check_is_named_by_its_bit() {
    // The bitmask convention this file rests on: check k contributes
    // 2^(k-1), so a program whose third check is the only one that
    // trips exits with 4 and nothing else. 1900 is not a leap year.
    expect_exit(
        "bit-names-the-failed-check",
        r#"
module Main where
  import Data.Time.Date as D

  check-a :: Int
  check-a = 0

  check-b :: Int
  check-b = 0

  check-c :: Int
  check-c = if D.is-leap-year 1900 then 0 else 4

  main :: Int
  main = check-a + check-b + check-c
"#,
        4,
    );
}
