//! Datetime lane D (spans, rounding and calendar arithmetic) tests:
//! `Data.Time.Unit`, `Data.Time.Span` and `Data.Time.Arithmetic`
//! (design/datetime.md, layer L3).
//!
//! The semantics are TC39 Temporal's (Duration, and add / subtract /
//! until / since / round on PlainDate, PlainDateTime, PlainTime and
//! Instant) as rendered by Jiff. The consumer-visible behaviour, per
//! the modules' stated law shapes:
//!   Unit:       the unit order and its two classes (fixed length and
//!               calendar), the rounding kernel for all nine modes (a
//!               literal table with negatives and exact halves, and an
//!               exhaustive comparison against an independent
//!               number-line reference), the kernel's refusals, the
//!               as-if-positive variant, the negation law, and the
//!               three increment validators.
//!   Span:       construction (total one-unit constructors over
//!               refined literals, checked setters), the shared-sign
//!               rule, every unit's limit, negation, fieldwise
//!               equality against comparison by length (2 hours is not
//!               fieldwise 120 minutes but compares equal), the class
//!               index, `Duration` conversions; unanchored balance,
//!               round, total, compare and sums on clock spans;
//!               anchored balance (2756 hours from 2020-01-01 is 3
//!               months 23 days 20 hours), round (1 month 1 day 1 hour
//!               from 2022-01-01 is 32 days), total (24 hours is 1/30
//!               of a month from 1 June and 1/31 from 1 July), compare
//!               and sums; every refusal.
//!   Arithmetic: adding spans to dates (2024-05-31 + 1 month =
//!               2024-06-30, 2024-02-29 + 1 year 2 months 5 days =
//!               2025-05-04), date-times, times of day (wrapping) and
//!               timestamps (clock units only); default differences
//!               and their laws; differences with options (2024-05-31
//!               until 2024-09-30 is 3 months 30 days; the Jiff
//!               announcement's 16 d 4 h 30 m); the month-end cases
//!               where `until` and `add` are not inverses; rounding of
//!               times, date-times and timestamps; every refusal.
//!   Vectors:    differential vectors whose expected values were
//!               produced by the TC39 Temporal reference polyfill.
//!
//! Every program is the real module sources beside a `Main`, compiled
//! and run through the same driver the CLI uses (a multi-file project,
//! since the modules are not embedded in the prelude). An on-disk
//! module ships beside `Main.cinch` under its dotted name
//! (`Data.Time.Span.cinch`). Each fixture is a complete `module Main`
//! under `fixtures/time/d/`; each check in it is a top-level `check-*`
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
const DURATION: &str = include_str!("../prelude/Data/Time/Duration.cinch");
const TIMESTAMP: &str = include_str!("../prelude/Data/Time/Timestamp.cinch");
const UNIT: &str = include_str!("../prelude/Data/Time/Unit.cinch");
const SPAN: &str = include_str!("../prelude/Data/Time/Span.cinch");
const ARITHMETIC: &str = include_str!("../prelude/Data/Time/Arithmetic.cinch");

const FIX_UNIT_ROUND: &str = include_str!("fixtures/time/d/unit_round.cinch");
const FIX_SPAN_CORE: &str = include_str!("fixtures/time/d/span_core.cinch");
const FIX_SPAN_CLOCK: &str = include_str!("fixtures/time/d/span_clock.cinch");
const FIX_SPAN_RELATIVE: &str = include_str!("fixtures/time/d/span_relative.cinch");
const FIX_ADD_SPAN: &str = include_str!("fixtures/time/d/add_span.cinch");
const FIX_UNTIL_SINCE: &str = include_str!("fixtures/time/d/until_since.cinch");
const FIX_DIFFERENCE_OPTIONS: &str = include_str!("fixtures/time/d/difference_options.cinch");
const FIX_ROUND_VALUES: &str = include_str!("fixtures/time/d/round_values.cinch");
const FIX_TEMPORAL_VECTORS: &str = include_str!("fixtures/time/d/temporal_vectors.cinch");

/// Compile and run a multi-module project (the lane D modules, the
/// lane A and lane B modules they import, plus `Main`), returning the
/// observed exit code.
fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_time_d_{}_{}_{}",
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
        ("Data.Time.Duration.cinch", DURATION),
        ("Data.Time.Timestamp.cinch", TIMESTAMP),
        ("Data.Time.Unit.cinch", UNIT),
        ("Data.Time.Span.cinch", SPAN),
        ("Data.Time.Arithmetic.cinch", ARITHMETIC),
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
fn unit_order_and_rounding_kernel_for_all_nine_modes() {
    expect_ok("unit-round", FIX_UNIT_ROUND);
}

#[test]
fn span_construction_sign_limits_fieldwise_equality_and_durations() {
    expect_ok("span-core", FIX_SPAN_CORE);
}

#[test]
fn clock_span_balance_round_total_compare_and_sums() {
    expect_ok("span-clock", FIX_SPAN_CLOCK);
}

#[test]
fn anchored_span_balance_round_total_compare_and_sums() {
    expect_ok("span-relative", FIX_SPAN_RELATIVE);
}

#[test]
fn adding_spans_to_dates_datetimes_times_and_timestamps() {
    expect_ok("add-span", FIX_ADD_SPAN);
}

#[test]
fn default_differences_their_laws_and_month_end_cases() {
    expect_ok("until-since", FIX_UNTIL_SINCE);
}

#[test]
fn differences_with_largest_smallest_increment_and_mode() {
    expect_ok("difference-options", FIX_DIFFERENCE_OPTIONS);
}

#[test]
fn rounding_times_datetimes_and_timestamps() {
    expect_ok("round-values", FIX_ROUND_VALUES);
}

#[test]
fn temporal_reference_polyfill_vectors() {
    expect_ok("temporal-vectors", FIX_TEMPORAL_VECTORS);
}

#[test]
fn a_failed_check_is_named_by_its_bit() {
    // The bitmask convention this file rests on: check k contributes
    // 2^(k-1), so a program whose third check is the only one that
    // trips exits with 4 and nothing else. 2 hours and 120 minutes are
    // the same length but not the same span.
    expect_exit(
        "bit-names-the-failed-check",
        r#"
module Main where
  import Data.Time.Span

  check-a :: Int
  check-a = 0

  check-b :: Int
  check-b = 0

  check-c :: Int
  check-c = if span-fieldwise-eq (span-of-hours 2) (span-of-minutes 120) then 0 else 4

  main :: Int
  main = check-a + check-b + check-c
"#,
        4,
    );
}
