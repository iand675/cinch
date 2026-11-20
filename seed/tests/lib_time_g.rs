//! Datetime lane G (zoned text) tests: `Text.Time.Rfc9557`,
//! `Text.Time.Zoned` and `Text.Time.Rfc2822` (design/datetime.md, layer
//! L4; T4 ambiguity is a value, T5 zone lookup is not zone use, T9
//! strict parsing by default, T10 errors are sums).
//!
//! The consumer-visible semantics, per the modules' stated law shapes:
//!   Rfc9557: the printed form of a `Zoned` (civil reading, offset in
//!            force, bracketed zone: IANA names, `[UTC]`, fixed
//!            offsets, offsets with seconds, the shortest exact
//!            fraction and the fixed precision) and `zoned-to-rfc3339`;
//!            the pure parser's `ZonedText` (the offset as written:
//!            none, `Z`, hour-only, with seconds; the zone annotation
//!            as a name or an offset; the critical flag; `key=value`
//!            annotations kept in order); every parse refusal by name
//!            and byte position in strict and lenient mode (no zone
//!            annotation, a missing `]`, `[]`, two zone annotations, a
//!            zone annotation after a key, `[!u-foo=bar]`,
//!            `[u-ca=hebrew]`, sub-minute bracket offsets, a date with
//!            no time); resolution by Temporal's
//!            InterpretISODateTimeOffset (Sydney's fold under each
//!            strategy and with each offset, a wrong offset under
//!            `OffsetReject`, `OffsetUse`, `OffsetPrefer` and
//!            `OffsetIgnore`, `Z` as an exact instant, New York's gap,
//!            a whole-minute offset matching a sub-minute zone offset,
//!            a date with no time as the first instant of the day
//!            including Santiago's missing midnight and the day Apia
//!            skipped, both ends of the range); and the round trip over
//!            sweeps of every shipped zone, by the minute around
//!            transitions, with both occurrences of folded hours.
//!   Zoned:   RFC 3339 at an `Offset` (total printer, typed parser);
//!            `zoned-strftime` with `%z`, `%Z` and `%Q`;
//!            `strptime` fields to a `Zoned` in a zone, at their own
//!            offset, and refused when they name neither; the
//!            effectful `zoned-from-string`, its lenient and
//!            policy-taking forms, `zoned-text-lookup` and
//!            `zoned-strptime`, with the refusing stage named.
//!   Rfc2822: `Sat, 13 Jul 2024 15:09:59 -0400` parsed to the instant
//!            and the offset and printed back; the day name checked
//!            against the date; obsolete years, zone names, comments
//!            and folding in lenient mode only; every refusal by name
//!            and position; the two printing refusals.
//!
//! Every program is the real module sources beside a `Main`, compiled
//! and run through the same driver the CLI uses (a multi-file project,
//! since the modules are not embedded in the prelude). An on-disk
//! module ships beside `Main.cinch` under its dotted name
//! (`Text.Time.Rfc9557.cinch`). Each fixture is a complete
//! `module Main` under `fixtures/time/g/`; each check in it is a
//! top-level `check-*` function returning 0 or its bit, and `main` sums
//! them, so a nonzero exit code is a bitmask naming the failed checks
//! (check k contributes bit 2^(k-1); at most eight checks per fixture,
//! so the mask fits a process exit code). In the fixtures that read
//! TZif files, a file that could not be loaded exits 128, or 255 where
//! all eight bits are in use.
//!
//! The TZif files are lane C's and are NOT copied into the temp
//! project: the fixtures open them by the relative path
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

// Lane E (text).
const SCAN: &str = include_str!("../prelude/Text/Time/Scan.cinch");
const ISO: &str = include_str!("../prelude/Text/Time/Iso.cinch");
const STRTIME: &str = include_str!("../prelude/Text/Time/Strtime.cinch");

// Lane G.
const RFC9557: &str = include_str!("../prelude/Text/Time/Rfc9557.cinch");
const TEXT_ZONED: &str = include_str!("../prelude/Text/Time/Zoned.cinch");
const RFC2822: &str = include_str!("../prelude/Text/Time/Rfc2822.cinch");

const FIX_RFC9557_PRINT: &str = include_str!("fixtures/time/g/rfc9557_print.cinch");
const FIX_RFC9557_PARSE: &str = include_str!("fixtures/time/g/rfc9557_parse.cinch");
const FIX_RFC9557_REFUSALS: &str = include_str!("fixtures/time/g/rfc9557_refusals.cinch");
const FIX_RFC9557_RESOLVE: &str = include_str!("fixtures/time/g/rfc9557_resolve.cinch");
const FIX_RFC9557_ROUND_TRIP: &str = include_str!("fixtures/time/g/rfc9557_round_trip.cinch");
const FIX_ZONED_TEXT: &str = include_str!("fixtures/time/g/zoned_text.cinch");
const FIX_ZONED_EFFECT: &str = include_str!("fixtures/time/g/zoned_effect.cinch");
const FIX_RFC2822: &str = include_str!("fixtures/time/g/rfc2822.cinch");

/// Compile and run a multi-module project (lanes A, B, C and E and the
/// three lane G modules beside `Main`), returning the observed exit
/// code.
fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_time_g_{}_{}_{}",
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
        ("Text.Time.Scan.cinch", SCAN),
        ("Text.Time.Iso.cinch", ISO),
        ("Text.Time.Strtime.cinch", STRTIME),
        ("Text.Time.Rfc9557.cinch", RFC9557),
        ("Text.Time.Zoned.cinch", TEXT_ZONED),
        ("Text.Time.Rfc2822.cinch", RFC2822),
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
fn rfc9557_prints_named_fixed_and_utc_zones_fractions_and_rfc3339() {
    expect_ok("rfc9557-print", FIX_RFC9557_PRINT);
}

#[test]
fn rfc9557_parser_keeps_offset_zone_annotation_and_other_annotations() {
    expect_ok("rfc9557-parse", FIX_RFC9557_PARSE);
}

#[test]
fn rfc9557_refuses_by_name_and_position_in_strict_and_lenient_mode() {
    expect_ok("rfc9557-refusals", FIX_RFC9557_REFUSALS);
}

#[test]
fn rfc9557_resolves_offsets_folds_gaps_and_dates_as_temporal_does() {
    expect_ok("rfc9557-resolve", FIX_RFC9557_RESOLVE);
}

#[test]
fn rfc9557_round_trips_sweeps_of_every_zone_across_transitions() {
    expect_ok("rfc9557-round-trip", FIX_RFC9557_ROUND_TRIP);
}

#[test]
fn zoned_text_rfc3339_at_an_offset_strftime_and_strptime_fields() {
    expect_ok("zoned-text", FIX_ZONED_TEXT);
}

#[test]
fn zoned_from_string_and_strptime_look_up_resolve_and_name_the_stage() {
    expect_ok("zoned-effect", FIX_ZONED_EFFECT);
}

#[test]
fn rfc2822_parses_strict_and_lenient_refuses_by_name_and_prints_back() {
    expect_ok("rfc2822", FIX_RFC2822);
}

#[test]
fn a_failed_check_is_named_by_its_bit() {
    // The bitmask convention this file rests on: check k contributes
    // 2^(k-1), so a program whose third check is the only one that
    // trips exits with 4 and nothing else. A timestamp with no zone
    // annotation is not a zoned date-time.
    expect_exit(
        "bit-names-the-failed-check",
        r#"
module Main where
  import Data.Result
  import Text.Time.Rfc9557

  parses :: String -> Bool
  parses s = case parse-zoned-text s of
    Err _ -> False
    Ok _ -> True

  check-named :: Int
  check-named = if parses "2026-11-12T15:36:00-05:00[America/New_York]" then 0 else 1

  check-fixed :: Int
  check-fixed = if parses "2026-11-12T15:36:00+05:30[+05:30]" then 0 else 2

  check-deliberately-wrong :: Int
  check-deliberately-wrong = if parses "2026-11-12T15:36:00-05:00" then 0 else 4

  main :: Int
  main = check-named + check-fixed + check-deliberately-wrong
"#,
        4,
    );
}
