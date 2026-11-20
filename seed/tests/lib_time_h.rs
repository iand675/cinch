//! Datetime lane H (duration text) tests: `Text.Time.Duration`
//! (design/datetime.md, layer L4; T7 options are values built by
//! functions, T9 strict parsing by default, T10 errors are sums).
//!
//! The consumer-visible semantics, per the module's stated law shape:
//!   ISO 8601 durations (as TC39 Temporal reads them): `PT5H30M` is a
//!     span of 5 hours 30 minutes and a duration of 19800 seconds and
//!     prints back from both; `P11Y2M28DT5H30M` is a span and is
//!     refused as a duration (days or larger); the sign of the whole
//!     duration; `PT0S` for zero; a fraction on the last time
//!     component converted exactly (`PT1.5H` is 1 hour 30 minutes,
//!     `PT0.5M` is 30 seconds, `.` or `,`); every refusal by name and
//!     byte position (`P`, `PT`, `P1`, `1Y`, `P1Y1Y`, `P1M1Y`,
//!     `PT1S1M`, `P1DT`, `P-1D`, `--P1D`, white space, `PT1.5H30M`,
//!     `P1.5D`, ten fraction digits); lower case refused by name in the
//!     strict parsers and read by the lenient ones.
//!   The friendly format (Jiff's): every documented example with its
//!     fields (`40d`, `1 year, 1 month, 1 day, 01:01:01.1`,
//!     `2h 30m ago`), every designator of every unit including the two
//!     Unicode mu spellings, fractions (`0.0021s` is 2 milliseconds 100
//!     microseconds), the clock form, separators and signs; every
//!     refusal (`1h 1d`, `1h 2h`, `1.5d`, `1.5h 2m`, `1 fortnight`,
//!     `-1h ago`, a bare number, trailing input, a unit over its
//!     limit).
//!   Printing: ISO folds sub-second units into fractional seconds
//!     (2000 milliseconds is `PT2S`: the stated loss); the friendly
//!     default is lossless (`2000ms`, `6y 10mo 3d 23h`); every
//!     `FriendlyPrinter` knob (designator style with singular forms,
//!     spacing, comma, sign style, fractional unit and precision, the
//!     clock form, the zero unit, the Unicode micro sign).
//!   Laws: friendly print then parse is the identity fieldwise for
//!     every designator style, spacing, comma and sign style over a
//!     sweep of spans and the edge spans; ISO print then parse keeps
//!     the calendar units, hours and minutes and the length; a
//!     `Duration` round trips exactly in both formats, up to the
//!     largest (2^127 - 1 nanoseconds) and the smallest.
//!   Limits: the largest value of each unit reads and prints, one more
//!     is refused; numbers of 38 digits are read and 39 refused.
//!
//! Every program is the real module sources beside a `Main`, compiled
//! and run through the same driver the CLI uses (a multi-file project,
//! since the modules are not embedded in the prelude). An on-disk
//! module ships beside `Main.cinch` under its dotted name
//! (`Text.Time.Duration.cinch`). Each fixture is a complete
//! `module Main` under `fixtures/time/h/`; each check in it is a
//! top-level `check-*` function returning 0 or its bit, and `main` sums
//! them, so a nonzero exit code is a bitmask naming the failed checks
//! (check k contributes bit 2^(k-1); at most eight checks per fixture,
//! so the mask fits a process exit code).

mod common;

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const DURATION: &str = include_str!("../prelude/Data/Time/Duration.cinch");
const TIMESTAMP: &str = include_str!("../prelude/Data/Time/Timestamp.cinch");
const WEEKDAY: &str = include_str!("../prelude/Data/Time/Weekday.cinch");
const DATE: &str = include_str!("../prelude/Data/Time/Date.cinch");
const TIME_OF_DAY: &str = include_str!("../prelude/Data/Time/TimeOfDay.cinch");
const DATE_TIME: &str = include_str!("../prelude/Data/Time/DateTime.cinch");
const UNIT: &str = include_str!("../prelude/Data/Time/Unit.cinch");
const SPAN: &str = include_str!("../prelude/Data/Time/Span.cinch");
const SCAN: &str = include_str!("../prelude/Text/Time/Scan.cinch");
const DURATION_TEXT: &str = include_str!("../prelude/Text/Time/Duration.cinch");

const FIX_ISO_PARSE: &str = include_str!("fixtures/time/h/iso_parse.cinch");
const FIX_ISO_REFUSALS: &str = include_str!("fixtures/time/h/iso_refusals.cinch");
const FIX_FRIENDLY_PARSE: &str = include_str!("fixtures/time/h/friendly_parse.cinch");
const FIX_FRIENDLY_REFUSALS: &str = include_str!("fixtures/time/h/friendly_refusals.cinch");
const FIX_PRINT: &str = include_str!("fixtures/time/h/print.cinch");
const FIX_ROUND_TRIP: &str = include_str!("fixtures/time/h/round_trip.cinch");
const FIX_DURATION_TEXT: &str = include_str!("fixtures/time/h/duration_text.cinch");
const FIX_LIMITS: &str = include_str!("fixtures/time/h/limits.cinch");

/// Compile and run a multi-module project (the machine, civil and span
/// modules the duration text module imports, the scanner, the module
/// itself, plus `Main`), returning the observed exit code.
fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_time_h_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [
        ("Data.Time.Duration.cinch", DURATION),
        ("Data.Time.Timestamp.cinch", TIMESTAMP),
        ("Data.Time.Weekday.cinch", WEEKDAY),
        ("Data.Time.Date.cinch", DATE),
        ("Data.Time.TimeOfDay.cinch", TIME_OF_DAY),
        ("Data.Time.DateTime.cinch", DATE_TIME),
        ("Data.Time.Unit.cinch", UNIT),
        ("Data.Time.Span.cinch", SPAN),
        ("Text.Time.Scan.cinch", SCAN),
        ("Text.Time.Duration.cinch", DURATION_TEXT),
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

#[test]
fn iso_durations_read_as_spans_with_fractions_signs_and_lenient_case() {
    expect_ok("iso-parse", FIX_ISO_PARSE);
}

#[test]
fn iso_duration_refusals_by_name_and_position() {
    expect_ok("iso-refusals", FIX_ISO_REFUSALS);
}

#[test]
fn friendly_examples_designators_fractions_clock_form_and_signs() {
    expect_ok("friendly-parse", FIX_FRIENDLY_PARSE);
}

#[test]
fn friendly_refusals_by_name_and_position() {
    expect_ok("friendly-refusals", FIX_FRIENDLY_REFUSALS);
}

#[test]
fn span_printing_iso_friendly_default_and_every_printer_option() {
    expect_ok("print", FIX_PRINT);
}

#[test]
fn span_round_trip_laws_lossless_friendly_and_length_preserving_iso() {
    expect_ok("round-trip", FIX_ROUND_TRIP);
}

#[test]
fn exact_durations_print_parse_refuse_days_and_round_trip_to_the_extremes() {
    expect_ok("duration-text", FIX_DURATION_TEXT);
}

#[test]
fn unit_limits_and_number_size_in_both_formats() {
    expect_ok("limits", FIX_LIMITS);
}
