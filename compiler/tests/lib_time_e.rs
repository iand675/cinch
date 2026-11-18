//! Datetime lane E (text) tests: `Text.Time.Scan`, `Text.Time.Iso` and
//! `Text.Time.Strtime` (design/datetime.md, layer L4; T9 strict parsing
//! by default, T10 errors are sums).
//!
//! The consumer-visible semantics, per the modules' stated law shapes:
//!   Iso:     the printed forms of `Date` (four-digit years and the
//!            signed six-digit expanded year), `TimeOfDay` (the
//!            shortest exact fraction for every length 1..=9, and the
//!            truncating fixed precision), `DateTime` and `Timestamp`
//!            (UTC with `Z`, and the civil reading at a chosen offset);
//!            the round trip of each at ordinary values, at both range
//!            ends, at year 0 and negative years, and over a walk of
//!            the whole date range; known instants
//!            (`2026-10-31T05:15:00Z` is Unix second 1793423700) and
//!            equal instants written at different offsets; every
//!            strict refusal with its named error and byte position
//!            (`2024-2-9`, `2024-02-30`, `2024-13-01`, `junk`, trailing
//!            input, a date-time where a date is expected and the
//!            reverse, `24:00:00`, `23:60:00`, `23:59:60`, an empty
//!            fraction, ten fraction digits, a date-time with no offset
//!            given to the timestamp parser, an offset beyond
//!            +-25:59:59, 29 February in a common year, an instant
//!            outside the range); the offset as written (`Z`, numeric,
//!            `-00:00`); the lenient forms (space and `t` separators,
//!            `z`, the basic date, second 60 read as 59); and the
//!            cursor-level parsers and buffer writers the RFC 9557
//!            module builds on.
//!   Scan:    the byte cursor (out-of-range reads are -1), the digit
//!            readers, the digit and byte writers, slices.
//!   Strtime: every conversion printed at known values (names, the
//!            twelve-hour clock at midnight, noon and 23:59, day of
//!            year, both weekday numberings, ISO week dates at year
//!            boundaries, the POSIX week numbers, the composites);
//!            every flag and width on numbers and text; the fraction
//!            and offset forms; `%Z`, `%Q`, `%s`; the missing-field
//!            refusals; parsing (the fields a format does not name stay
//!            absent, flexible digit counts, names in any case and
//!            either length, white space, offsets, dates named by day
//!            of year, ISO week or POSIX week); every parse, range,
//!            format and conversion refusal by name (an unknown
//!            conversion, a bare `%`, `%Z` when parsing, a weekday that
//!            contradicts its date); the `BrokenDown` setters, getters
//!            and conversions; a compiled `Format` applied many times;
//!            and print-then-parse round trips over the whole range.
//!
//! Every program is the real module sources beside a `Main`, compiled
//! and run through the same driver the CLI uses (a multi-file project,
//! since the modules are not embedded in the prelude). An on-disk
//! module ships beside `Main.cinch` under its dotted name
//! (`Text.Time.Iso.cinch`). Each fixture is a complete `module Main`
//! under `fixtures/time/e/`; each check in it is a top-level `check-*`
//! function returning 0 or its bit, and `main` sums them, so a nonzero
//! exit code is a bitmask naming the failed checks (check k contributes
//! bit 2^(k-1); at most eight checks per fixture, so the mask fits a
//! process exit code).

mod common;

use cinch::driver::Mode;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const DURATION: &str = include_str!("../prelude/Data/Time/Duration.cinch");
const TIMESTAMP: &str = include_str!("../prelude/Data/Time/Timestamp.cinch");
const WEEKDAY: &str = include_str!("../prelude/Data/Time/Weekday.cinch");
const DATE: &str = include_str!("../prelude/Data/Time/Date.cinch");
const TIME_OF_DAY: &str = include_str!("../prelude/Data/Time/TimeOfDay.cinch");
const DATE_TIME: &str = include_str!("../prelude/Data/Time/DateTime.cinch");
const SCAN: &str = include_str!("../prelude/Text/Time/Scan.cinch");
const ISO: &str = include_str!("../prelude/Text/Time/Iso.cinch");
const STRTIME: &str = include_str!("../prelude/Text/Time/Strtime.cinch");

const FIX_ISO_DATE: &str = include_str!("fixtures/time/e/iso_date.cinch");
const FIX_ISO_TIME: &str = include_str!("fixtures/time/e/iso_time.cinch");
const FIX_ISO_DATETIME: &str = include_str!("fixtures/time/e/iso_datetime.cinch");
const FIX_ISO_TIMESTAMP: &str = include_str!("fixtures/time/e/iso_timestamp.cinch");
const FIX_ISO_PIECES: &str = include_str!("fixtures/time/e/iso_pieces.cinch");
const FIX_STRTIME_FORMAT: &str = include_str!("fixtures/time/e/strtime_format.cinch");
const FIX_STRTIME_FLAGS: &str = include_str!("fixtures/time/e/strtime_flags.cinch");
const FIX_STRTIME_PARSE: &str = include_str!("fixtures/time/e/strtime_parse.cinch");
const FIX_STRTIME_REFUSALS: &str = include_str!("fixtures/time/e/strtime_refusals.cinch");
const FIX_STRTIME_BROKEN_DOWN: &str = include_str!("fixtures/time/e/strtime_broken_down.cinch");

/// Compile and run a multi-module project (the machine and civil
/// modules the text modules import, the three text modules, plus
/// `Main`), returning the observed exit code.
fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_time_e_{}_{}_{}",
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
        ("Text.Time.Scan.cinch", SCAN),
        ("Text.Time.Iso.cinch", ISO),
        ("Text.Time.Strtime.cinch", STRTIME),
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
fn iso_date_forms_round_trips_and_refusals() {
    expect_ok("iso-date", FIX_ISO_DATE);
}

#[test]
fn iso_time_fraction_lengths_precision_and_refusals() {
    expect_ok("iso-time", FIX_ISO_TIME);
}

#[test]
fn iso_datetime_round_trips_strict_and_lenient_separators() {
    expect_ok("iso-datetime", FIX_ISO_DATETIME);
}

#[test]
fn iso_timestamp_instants_offsets_and_the_missing_offset_refusal() {
    expect_ok("iso-timestamp", FIX_ISO_TIMESTAMP);
}

#[test]
fn iso_cursor_parsers_buffer_writers_and_scan_helpers() {
    expect_ok("iso-pieces", FIX_ISO_PIECES);
}

#[test]
fn strtime_formats_every_conversion() {
    expect_ok("strtime-format", FIX_STRTIME_FORMAT);
}

#[test]
fn strtime_flags_widths_fractions_offsets_and_missing_fields() {
    expect_ok("strtime-flags", FIX_STRTIME_FLAGS);
}

#[test]
fn strtime_parses_fields_names_offsets_and_week_dates() {
    expect_ok("strtime-parse", FIX_STRTIME_PARSE);
}

#[test]
fn strtime_refuses_by_name_in_formats_inputs_and_conversions() {
    expect_ok("strtime-refusals", FIX_STRTIME_REFUSALS);
}

#[test]
fn strtime_broken_down_fields_compiled_formats_and_round_trips() {
    expect_ok("strtime-broken-down", FIX_STRTIME_BROKEN_DOWN);
}

#[test]
fn a_failed_check_is_named_by_its_bit() {
    // The bitmask convention this file rests on: check k contributes
    // 2^(k-1), so a program whose third check is the only one that
    // trips exits with 4 and nothing else. `2024-02-30` is not a date.
    expect_exit(
        "bit-names-the-failed-check",
        r#"
module Main where
  import Data.Result
  import Text.Time.Iso

  parses :: String -> Bool
  parses s = case date-from-string s of
    Err _ -> False
    Ok _ -> True

  check-leap-day :: Int
  check-leap-day = if parses "2024-02-29" then 0 else 1

  check-year-end :: Int
  check-year-end = if parses "2024-12-31" then 0 else 2

  check-deliberately-wrong :: Int
  check-deliberately-wrong = if parses "2024-02-30" then 0 else 4

  main :: Int
  main = check-leap-day + check-year-end + check-deliberately-wrong
"#,
        4,
    );
}
