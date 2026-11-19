//! Datetime lane C (time zones) tests: `Data.Time.Offset`,
//! `Data.Time.Zone` (with `Data.Time.Zone.Types`),
//! `Data.Time.Zone.Posix`, `Data.Time.Zone.Tzif`, `Data.Time.Zoned` and
//! `Effect.Time.Zone` (design/datetime.md, layer L2 and its L5 reader).
//!
//! The consumer-visible semantics, per the modules' stated law shapes:
//!   Offset:  construction and its refusals at +-25:59:59, components,
//!            negation, the order, and the two conversions between an
//!            instant and a civil reading at an offset, including both
//!            ends of the timestamp range.
//!   Posix:   the TZ rule grammar, accepted and refused production by
//!            production with the named error; a northern and a
//!            southern rule read on each side of every transition, in
//!            far-future years, and on the civil side (gap, fold); the
//!            `Jn` and `n` day forms; RFC 9636's signed and 24:00
//!            times of change; the "always daylight" spelling.
//!   Zone:    fixed zones; the Disambiguation-by-Ambiguous table
//!            (Temporal's and Jiff's four strategies); hand-built
//!            transition tables with and without a tail rule; the
//!            ambiguity derived from a table.
//!   Tzif:    the eight shipped IANA files (tzdata 2026e) on the Jiff
//!            1.0 announcement's examples and on Sydney, Kolkata,
//!            Amsterdam, London, Apia (the skipped day), Santiago
//!            (midnight gaps) and UTC; the footer rule past the table
//!            (2038, 2100, 2500); the first local time type before the
//!            first transition; and every named refusal on truncated
//!            and corrupted input.
//!   Zoned:   the cached reading's invariant, construction under each
//!            strategy, changing zone, the start of the day where it is
//!            not 00:00, the instant order and instant-and-name
//!            equality.
//!   Effect:  zone names that cannot leave the database directory,
//!            lookups refused by stage, the system zone and
//!            `zoned-now`.
//!
//! Every program is the real module sources beside a `Main`, compiled
//! and run through the same driver the CLI uses (a multi-file project,
//! since the modules are not embedded in the prelude). An on-disk
//! module ships beside `Main.cinch` under its dotted name
//! (`Data.Time.Zone.cinch`). Each fixture is a complete `module Main`
//! under `fixtures/time/c/`; each check in it is a top-level `check-*`
//! function returning 0 or its bit, and `main` sums them, so a nonzero
//! exit code is a bitmask naming the failed checks (check k contributes
//! bit 2^(k-1); at most eight checks per fixture, so the mask fits a
//! process exit code). In the fixtures that read TZif files, bit 128
//! means a file could not be loaded.
//!
//! The TZif files are NOT copied into the temp project: the fixtures
//! open them by the relative path `tests/fixtures/time/c/tzif/...`,
//! which resolves because tests run with the current directory at the
//! package root (`compiler/`).

mod common;

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

// Lane A (machine time) and lane B (civil time), which lane C builds on.
const DURATION: &str = include_str!("../prelude/Data/Time/Duration.cinch");
const MONOTONIC: &str = include_str!("../prelude/Data/Time/Monotonic.cinch");
const TIMESTAMP: &str = include_str!("../prelude/Data/Time/Timestamp.cinch");
const EFFECT_TIME: &str = include_str!("../prelude/Effect/Time.cinch");
const WEEKDAY: &str = include_str!("../prelude/Data/Time/Weekday.cinch");
const DATE: &str = include_str!("../prelude/Data/Time/Date.cinch");
const TIME_OF_DAY: &str = include_str!("../prelude/Data/Time/TimeOfDay.cinch");
const DATE_TIME: &str = include_str!("../prelude/Data/Time/DateTime.cinch");

// Lane C.
const OFFSET: &str = include_str!("../prelude/Data/Time/Offset.cinch");
const ZONE_TYPES: &str = include_str!("../prelude/Data/Time/Zone/Types.cinch");
const ZONE_POSIX: &str = include_str!("../prelude/Data/Time/Zone/Posix.cinch");
const ZONE: &str = include_str!("../prelude/Data/Time/Zone.cinch");
const ZONE_TZIF: &str = include_str!("../prelude/Data/Time/Zone/Tzif.cinch");
const ZONED: &str = include_str!("../prelude/Data/Time/Zoned.cinch");
const EFFECT_TIME_ZONE: &str = include_str!("../prelude/Effect/Time/Zone.cinch");

const FIX_OFFSET: &str = include_str!("fixtures/time/c/offset.cinch");
const FIX_POSIX_PARSE: &str = include_str!("fixtures/time/c/posix_parse.cinch");
const FIX_POSIX_RULES: &str = include_str!("fixtures/time/c/posix_rules.cinch");
const FIX_ZONE_PURE: &str = include_str!("fixtures/time/c/zone_pure.cinch");
const FIX_ZONE_NEW_YORK: &str = include_str!("fixtures/time/c/zone_new_york.cinch");
const FIX_ZONE_WORLD: &str = include_str!("fixtures/time/c/zone_world.cinch");
const FIX_ZONED: &str = include_str!("fixtures/time/c/zoned.cinch");
const FIX_TZIF_REFUSALS: &str = include_str!("fixtures/time/c/tzif_refusals.cinch");
const FIX_TZDB: &str = include_str!("fixtures/time/c/tzdb.cinch");

/// Compile and run a multi-module project (lanes A, B and C beside
/// `Main`), returning the observed exit code.
fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_time_c_{}_{}_{}",
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
fn offset_bounds_components_and_conversions_at_the_range_ends() {
    expect_ok("offset", FIX_OFFSET);
}

#[test]
fn posix_tz_grammar_accepts_and_refuses_by_production() {
    expect_ok("posix-parse", FIX_POSIX_PARSE);
}

#[test]
fn posix_rules_north_and_south_offsets_ambiguity_and_transitions() {
    expect_ok("posix-rules", FIX_POSIX_RULES);
}

#[test]
fn zone_fixed_offsets_disambiguation_table_and_hand_built_tables() {
    expect_ok("zone-pure", FIX_ZONE_PURE);
}

#[test]
fn zone_new_york_jiff_examples_fold_gap_footer_rule_and_sweep() {
    expect_ok("zone-new-york", FIX_ZONE_NEW_YORK);
}

#[test]
fn zone_world_sydney_kolkata_amsterdam_london_apia_santiago_utc() {
    expect_ok("zone-world", FIX_ZONE_WORLD);
}

#[test]
fn zoned_invariant_strategies_other_zone_and_start_of_day() {
    expect_ok("zoned", FIX_ZONED);
}

#[test]
fn tzif_named_refusals_on_truncated_and_corrupted_input() {
    expect_ok("tzif-refusals", FIX_TZIF_REFUSALS);
}

#[test]
fn tzdb_name_validation_lookup_refusals_and_system_zone() {
    expect_ok("tzdb", FIX_TZDB);
}

#[test]
fn a_failed_check_is_named_by_its_bit() {
    // The bitmask convention this file rests on: check k contributes
    // 2^(k-1), so a program whose third check is the only one that
    // trips exits with 4 and nothing else. `EST5EDT` has no rule.
    expect_exit(
        "bit-names-the-failed-check",
        r#"
module Main where
  import Data.Result
  import Data.Time.Zone.Posix

  check-a :: Int
  check-a = 0

  check-b :: Int
  check-b = 0

  check-c :: Int
  check-c = case parse-posix-tz "EST5EDT" of
    Ok _ -> 0
    Err _ -> 4

  main :: Int
  main = check-a + check-b + check-c
"#,
        4,
    );
}
