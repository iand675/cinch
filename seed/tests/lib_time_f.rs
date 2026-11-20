//! Datetime lane F (zoned arithmetic) tests:
//! `Data.Time.Zoned.Arithmetic` (design/datetime.md, layer L3 on L2).
//!
//! The semantics are TC39 Temporal's ZonedDateTime (add, subtract,
//! until, since, round, startOfDay, hoursInDay, with) and its Duration
//! operations with a zoned relativeTo, as rendered by Jiff. The
//! consumer-visible behaviour, per the module's stated law shape:
//!   Add:        calendar units move the civil reading and are
//!               resolved again in the zone by the options'
//!               disambiguation (2024-05-31T13:45 + 1 month is
//!               2024-06-30T13:45; one day into the fold or the gap
//!               under each strategy); clock units move the instant
//!               (01:30 -05:00 + 1 hour is 03:30 -04:00), so one day
//!               and 24 hours differ on transition days; the day
//!               Pacific/Apia skipped; the range ends.
//!   Days:       tomorrow and yesterday (25 and 23 hours away on the
//!               transition days), the start of the next civil day,
//!               the exact length of a day (23, 24, 25 hours; a day
//!               that starts at 01:00 in America/Santiago), and lane
//!               B's month, year and weekday operations in a zone.
//!   Difference: the default is exact elapsed hours and works across
//!               zones; with days or larger the calendar part is
//!               counted on civil readings in one zone (another zone
//!               is a named refusal) and the rest is elapsed time; the
//!               laws `since = -until` and `a + (a until b) = b` on a
//!               sweep over both 2024 transitions; the range ends.
//!   Rounding a difference: a day is as long as it was (12 h 15 m of
//!               New York's 25-hour day rounds down to 0 days), hours
//!               against the real day, week, month and year units, the
//!               carry into larger units, `since` with directed modes.
//!   Round:      a value to the day within its real day, to smaller
//!               units on the civil reading keeping the value's
//!               offset.
//!   With:       replacing the date or the time keeps the offset when
//!               the new reading can have it, else the strategy.
//!   Spans:      balance, round, total, compare and sums relative to a
//!               zoned anchor (2756 hours from 2020-01-01 in
//!               Europe/Amsterdam is 3 months 23 days 21 hours).
//! Expected values come from the Jiff 1.0 announcement, from the TC39
//! Temporal reference polyfill and from Python's zoneinfo (tzdata
//! 2026e); where the current specification text is later than that
//! polyfill the fixture says so beside the vector.
//!
//! Every program is the real module sources beside a `Main`, compiled
//! and run through the same driver the CLI uses (a multi-file project,
//! since the modules are not embedded in the prelude). An on-disk
//! module ships beside `Main.cinch` under its dotted name
//! (`Data.Time.Zoned.Arithmetic.cinch`). Each fixture is a complete
//! `module Main` under `fixtures/time/f/`; each check in it is a
//! top-level `check-*` function returning 0 or its bit, and `main`
//! sums them, so a nonzero exit code is a bitmask naming the failed
//! checks (check k contributes bit 2^(k-1); at most eight checks per
//! fixture, so the mask fits a process exit code). An exit code of 255
//! means a TZif file could not be loaded.
//!
//! The TZif files are NOT copied into the temp project: the fixtures
//! open lane C's files by the relative path
//! `tests/fixtures/time/c/tzif/...`, which resolves because tests run
//! with the current directory at the package root (`seed/`).

mod common;

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

// Lane A (machine time) and lane B (civil time).
const DURATION: &str = include_str!("../prelude/Data/Time/Duration.cinch");
const MONOTONIC: &str = include_str!("../prelude/Data/Time/Monotonic.cinch");
const TIMESTAMP: &str = include_str!("../prelude/Data/Time/Timestamp.cinch");
const EFFECT_TIME: &str = include_str!("../prelude/Effect/Time.cinch");
const WEEKDAY: &str = include_str!("../prelude/Data/Time/Weekday.cinch");
const DATE: &str = include_str!("../prelude/Data/Time/Date.cinch");
const TIME_OF_DAY: &str = include_str!("../prelude/Data/Time/TimeOfDay.cinch");
const DATE_TIME: &str = include_str!("../prelude/Data/Time/DateTime.cinch");

// Lane C (zones).
const OFFSET: &str = include_str!("../prelude/Data/Time/Offset.cinch");
const ZONE_TYPES: &str = include_str!("../prelude/Data/Time/Zone/Types.cinch");
const ZONE_POSIX: &str = include_str!("../prelude/Data/Time/Zone/Posix.cinch");
const ZONE: &str = include_str!("../prelude/Data/Time/Zone.cinch");
const ZONE_TZIF: &str = include_str!("../prelude/Data/Time/Zone/Tzif.cinch");
const ZONED: &str = include_str!("../prelude/Data/Time/Zoned.cinch");
const EFFECT_TIME_ZONE: &str = include_str!("../prelude/Effect/Time/Zone.cinch");

// Lane D (spans and civil arithmetic).
const UNIT: &str = include_str!("../prelude/Data/Time/Unit.cinch");
const SPAN: &str = include_str!("../prelude/Data/Time/Span.cinch");
const ARITHMETIC: &str = include_str!("../prelude/Data/Time/Arithmetic.cinch");

// Lane F.
const ZONED_ARITHMETIC: &str = include_str!("../prelude/Data/Time/Zoned/Arithmetic.cinch");

const FIX_ADD_SPAN: &str = include_str!("fixtures/time/f/add_span.cinch");
const FIX_DAYS: &str = include_str!("fixtures/time/f/days.cinch");
const FIX_UNTIL_SINCE: &str = include_str!("fixtures/time/f/until_since.cinch");
const FIX_DIFFERENCE_OPTIONS: &str = include_str!("fixtures/time/f/difference_options.cinch");
const FIX_ROUND: &str = include_str!("fixtures/time/f/round.cinch");
const FIX_WITH_FIELDS: &str = include_str!("fixtures/time/f/with_fields.cinch");
const FIX_SPAN_ZONED: &str = include_str!("fixtures/time/f/span_zoned.cinch");
const FIX_CIVIL_AGREEMENT: &str = include_str!("fixtures/time/f/civil_agreement.cinch");

/// Compile and run a multi-module project (the lane F module, the lane
/// A to D modules it and the fixtures import, plus `Main`), returning
/// the observed exit code.
fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_time_f_{}_{}_{}",
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
        ("Data.Time.Weekday.cinch", WEEKDAY),
        ("Data.Time.Date.cinch", DATE),
        ("Data.Time.TimeOfDay.cinch", TIME_OF_DAY),
        ("Data.Time.DateTime.cinch", DATE_TIME),
        ("Data.Time.Offset.cinch", OFFSET),
        ("Data.Time.Zone.Types.cinch", ZONE_TYPES),
        ("Data.Time.Zone.Posix.cinch", ZONE_POSIX),
        ("Data.Time.Zone.cinch", ZONE),
        ("Data.Time.Zone.Tzif.cinch", ZONE_TZIF),
        ("Data.Time.Zoned.cinch", ZONED),
        ("Effect.Time.Zone.cinch", EFFECT_TIME_ZONE),
        ("Data.Time.Unit.cinch", UNIT),
        ("Data.Time.Span.cinch", SPAN),
        ("Data.Time.Arithmetic.cinch", ARITHMETIC),
        ("Data.Time.Zoned.Arithmetic.cinch", ZONED_ARITHMETIC),
        ("Main.cinch", main_src),
    ] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    match common::run_executable(&dir, "Main.cinch") {
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
fn adding_spans_calendar_on_the_reading_clock_on_the_instant() {
    expect_ok("add-span", FIX_ADD_SPAN);
}

#[test]
fn civil_days_tomorrow_day_length_and_month_year_weekday_operations() {
    expect_ok("days", FIX_DAYS);
}

#[test]
fn differences_default_calendar_cross_zone_laws_and_range() {
    expect_ok("until-since", FIX_UNTIL_SINCE);
}

#[test]
fn differences_rounded_against_the_real_length_of_a_day() {
    expect_ok("difference-options", FIX_DIFFERENCE_OPTIONS);
}

#[test]
fn rounding_a_zoned_value_to_the_day_and_to_clock_units() {
    expect_ok("round", FIX_ROUND);
}

#[test]
fn replacing_date_or_time_prefers_the_current_offset() {
    expect_ok("with-fields", FIX_WITH_FIELDS);
}

#[test]
fn spans_balanced_rounded_totalled_compared_and_summed_from_a_zoned_anchor() {
    expect_ok("span-zoned", FIX_SPAN_ZONED);
}

#[test]
fn civil_and_zoned_engines_agree_in_fixed_offset_zones() {
    expect_ok("civil-agreement", FIX_CIVIL_AGREEMENT);
}

#[test]
fn a_failed_check_is_named_by_its_bit() {
    // The bitmask convention this file rests on: check k contributes
    // 2^(k-1), so a program whose third check is the only one that
    // trips exits with 4 and nothing else. One hour after the epoch in
    // a fixed zone is not the epoch.
    expect_exit(
        "bit-names-the-failed-check",
        r#"
module Main where
  import Data.Option
  import Data.Result
  import Relation.Eq
  import Data.Time.Timestamp
  import Data.Time.Zone
  import Data.Time.Zoned
  import Data.Time.Span
  import Data.Time.Zoned.Arithmetic

  check-a :: Int
  check-a = 0

  check-b :: Int
  check-b = 0

  check-c :: Int
  check-c =
    let x = zoned-from-timestamp zone-utc timestamp-epoch in
    case zoned-add-span (span-of-hours 1) x of
      Err _ -> 0
      Ok y -> if zoned-timestamp y == timestamp-epoch then 0 else 4

  main :: Int
  main = check-a + check-b + check-c
"#,
        4,
    );
}
