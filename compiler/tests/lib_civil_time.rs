//! Civil time family tests: `Data.Date` (the proleptic Gregorian
//! calendar date) and `Data.Time` (a date plus nanoseconds since
//! midnight, and the UTC wall-nanosecond conversions).
//!
//! The consumer-visible semantics, per the modules' stated law shapes:
//!   Date:   the Gregorian leap rule at its century edge (1900 and
//!           2100 are not leap, 2000 is), the month lengths it fixes,
//!           day arithmetic crossing month and year boundaries (and
//!           undoing itself), `day-of-week` against known anchors
//!           (2026-10-29 is a Thursday, 1970-01-01 a Thursday,
//!           1969-12-31 a Wednesday, 2000-01-01 a Saturday),
//!           `day-of-year` at the year boundaries, the epoch-day
//!           round trip and `diff-days` (including before the epoch),
//!           the construction refusals (2023-02-29 and friends have no
//!           representation), and ISO 8601 rendering/reading including
//!           zero padding, the sign of a negative year and the two
//!           named parse errors.
//!   Time:   the day's nanosecond window, the field accessors of
//!           `nanoseconds-of-day`, the epoch-nanosecond round trip
//!           (including the pre-epoch instant -1), duration arithmetic
//!           that carries the date forward across a month and backward
//!           across a year, `diff`/`cmp`, and ISO 8601 rendering and
//!           reading (fraction digits, optional `Z`, and the named
//!           refusals, including the date half's error carried through
//!           as `BadDatePart`).
//!
//! Every program is the real module sources beside a `Main`, compiled
//! and run through the same driver the CLI uses (a multi-file project,
//! since the modules are not embedded in the prelude). Each check is a
//! top-level `check-*` function returning 0 or its bit; `main` sums
//! them, so a nonzero exit code is a bitmask naming the failed checks
//! (check k contributes bit 2^(k-1), and any failure mode of one check
//! shares that bit). The checks are top-level because a `case` in a
//! `let` binder's right-hand side must indent past the binder's column
//! under the layout rules.

use cinch::driver::{driver, Mode};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const DATE: &str = include_str!("../prelude/Data/Date.cinch");
const TIME: &str = include_str!("../prelude/Data/Time.cinch");

/// Compile and run a two-module project (the family sources plus
/// `Main`), returning the observed exit code.
fn run_main(name: &str, main_src: &str) -> i32 {
    static N: AtomicUsize = AtomicUsize::new(0);
    let dir: PathBuf = std::env::temp_dir().join(format!(
        "cinch_civil_{}_{}_{}",
        std::process::id(),
        N.fetch_add(1, Ordering::SeqCst),
        name
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("temp project dir");
    for (file, src) in [
        ("Date.cinch", DATE),
        ("Time.cinch", TIME),
        ("Main.cinch", main_src),
    ] {
        std::fs::write(dir.join(file), src).expect("write module");
    }
    match driver(Mode::Run, &dir.join("Main.cinch"), None) {
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

// ------------------------------------------------------------------ Date

#[test]
fn date_leap_rule_and_month_lengths() {
    expect_ok(
        "date-leap",
        r#"
module Main where
  import Data.Date

  check-century-years :: Int
  check-century-years =
    if is-leap-year 1900 then 1
    else if is-leap-year 2100 then 1
    else if is-leap-year 1800 then 1
    else 0

  check-leap-rule :: Int
  check-leap-rule =
    if is-leap-year 2000 then
      if is-leap-year 2024 then
        if is-leap-year 2023 then 2 else 0
      else 2
    else 2

  check-february-lengths :: Int
  check-february-lengths =
    if days-in-month 2024 2 == 29 then
      if days-in-month 2023 2 == 28 then
        if days-in-month 2000 2 == 29 then
          if days-in-month 1900 2 == 28 then 0 else 4
        else 4
      else 4
    else 4

  check-other-month-lengths :: Int
  check-other-month-lengths =
    if days-in-month 2024 1 == 31 then
      if days-in-month 2024 4 == 30 then
        if days-in-month 2024 6 == 30 then
          if days-in-month 2024 11 == 30 then
            if days-in-month 2024 12 == 31 then 0 else 8
          else 8
        else 8
      else 8
    else 8

  main :: Int
  main = check-century-years + check-leap-rule + check-february-lengths + check-other-month-lengths
"#,
    );
}

#[test]
fn date_day_arithmetic_crosses_month_and_year_boundaries() {
    expect_ok(
        "date-add",
        r#"
module Main where
  import Data.Date

  same :: Option Date -> Option Date -> Bool
  same a b = case a of
    None -> case b of
      None -> True
      Some _ -> False
    Some x -> case b of
      None -> False
      Some y -> x == y

  after :: Nat -> Int -> Nat -> Nat -> Option Date
  after n y m d = case date y m d of
    None -> None
    Some x -> Some (add-days n x)

  before :: Nat -> Int -> Nat -> Nat -> Option Date
  before n y m d = case date y m d of
    None -> None
    Some x -> Some (sub-days n x)

  check-leap-february :: Int
  check-leap-february = if same (after 1 2024 2 28) (date 2024 2 29) then 0 else 1

  check-common-february :: Int
  check-common-february = if same (after 1 2023 2 28) (date 2023 3 1) then 0 else 2

  check-year-boundaries :: Int
  check-year-boundaries =
    if same (after 1 2023 12 31) (date 2024 1 1) then
      if same (before 1 2024 1 1) (date 2023 12 31) then 0 else 4
    else 4

  check-back-over-february :: Int
  check-back-over-february = if same (before 1 2024 3 1) (date 2024 2 29) then 0 else 8

  check-forward-chain :: Int
  check-forward-chain = if same (after 2 2024 2 27) (date 2024 2 29) then 0 else 16

  check-backward-chain :: Int
  check-backward-chain = if same (before 2 2023 3 1) (date 2023 2 27) then 0 else 32

  main :: Int
  main = check-leap-february + check-common-february + check-year-boundaries + check-back-over-february + check-forward-chain + check-backward-chain
"#,
    );
}

#[test]
fn date_weekday_and_day_of_year_against_known_anchors() {
    expect_ok(
        "date-anchors",
        r#"
module Main where
  import Data.Date

  wday :: Int -> Nat -> Nat -> Nat
  wday y m d = case date y m d of
    None -> 99
    Some x -> day-of-week x

  doy :: Int -> Nat -> Nat -> Nat
  doy y m d = case date y m d of
    None -> 999
    Some x -> day-of-year x

  check-today :: Int
  check-today = if wday 2026 10 29 == 4 then 0 else 1

  check-epoch-anchor :: Int
  check-epoch-anchor = if wday 1970 1 1 == 4 then 0 else 2

  check-pre-epoch-anchors :: Int
  check-pre-epoch-anchors =
    if wday 1969 12 31 == 3 then
      if wday 2000 1 1 == 6 then 0 else 4
    else 4

  check-leap-day-anchors :: Int
  check-leap-day-anchors =
    if wday 2024 2 29 == 4 then
      if wday 2023 3 1 == 3 then 0 else 8
    else 8

  check-year-start :: Int
  check-year-start =
    if doy 2024 1 1 == 1 then
      if doy 2023 1 1 == 1 then 0 else 16
    else 16

  check-year-end :: Int
  check-year-end =
    if doy 2024 12 31 == 366 then
      if doy 2023 12 31 == 365 then 0 else 32
    else 32

  check-march-first :: Int
  check-march-first =
    if doy 2024 3 1 == 61 then
      if doy 2023 3 1 == 60 then
        if doy 2024 2 29 == 60 then 0 else 64
      else 64
    else 64

  main :: Int
  main = check-today + check-epoch-anchor + check-pre-epoch-anchors + check-leap-day-anchors + check-year-start + check-year-end + check-march-first
"#,
    );
}

#[test]
fn date_epoch_days_round_trip_and_differences() {
    expect_ok(
        "date-epoch",
        r#"
module Main where
  import Data.Date

  epoch :: Int -> Nat -> Nat -> Int
  epoch y m d = case date y m d of
    None -> 999999
    Some x -> to-epoch-days x

  same :: Option Date -> Option Date -> Bool
  same a b = case a of
    None -> case b of
      None -> True
      Some _ -> False
    Some x -> case b of
      None -> False
      Some y -> x == y

  day :: Int -> Date
  day z = from-epoch-days z

  check-origin :: Int
  check-origin = if epoch 1970 1 1 == 0 then 0 else 1

  check-known-days :: Int
  check-known-days =
    if epoch 2024 1 1 == 19723 then
      if epoch 2026 10 29 == 20755 then
        if epoch 2000 1 1 == 10957 then 0 else 2
      else 2
    else 2

  check-pre-epoch-days :: Int
  check-pre-epoch-days =
    if epoch 1969 12 31 == -1 then
      if epoch 1970 1 2 == 1 then
        if epoch 1969 1 1 == -365 then 0 else 4
      else 4
    else 4

  check-from-days :: Int
  check-from-days =
    if same (Some (day 0)) (date 1970 1 1) then
      if same (Some (day (-1))) (date 1969 12 31) then
        if same (Some (day 19723)) (date 2024 1 1) then 0 else 8
      else 8
    else 8

  check-round-trip :: Int
  check-round-trip =
    if to-epoch-days (day 0) == 0 then
      if to-epoch-days (day (-1000)) == -1000 then
        if to-epoch-days (day 20755) == 20755 then 0 else 16
      else 16
    else 16

  check-diff-signs :: Int
  check-diff-signs =
    if diff-days (day 0) (day 1) == 1 then
      if diff-days (day 1) (day 0) == -1 then 0 else 32
    else 32

  check-diff-inverts-add-and-sub :: Int
  check-diff-inverts-add-and-sub =
    if diff-days (day 19723) (add-days 5 (day 19723)) == 5 then
      if diff-days (day 19723) (sub-days 5 (day 19723)) == -5 then 0 else 64
    else 64

  main :: Int
  main = check-origin + check-known-days + check-pre-epoch-days + check-from-days + check-round-trip + check-diff-signs + check-diff-inverts-add-and-sub
"#,
    );
}

#[test]
fn date_construction_refusals_and_iso_8601() {
    expect_ok(
        "date-iso",
        r#"
module Main where
  import Data.Date

  valid :: Int -> Nat -> Nat -> Bool
  valid y m d = case date y m d of
    None -> False
    Some _ -> True

  text-of :: Int -> Nat -> Nat -> String
  text-of y m d = case date y m d of
    None -> "?"
    Some x -> to-string x

  round-text :: Int -> Nat -> Nat -> Bool
  round-text y m d = case date y m d of
    None -> False
    Some x -> case from-string (to-string x) of
      Err _ -> False
      Ok z -> x == z

  parse-kind :: String -> Nat
  parse-kind s = case from-string s of
    Ok _ -> 0
    Err e -> case e of
      NotADate -> 1
      InvalidDate -> 2

  check-accepts-real-dates :: Int
  check-accepts-real-dates =
    if valid 2024 2 29 then
      if valid 2024 12 31 then
        if valid 1 1 1 then 0 else 1
      else 1
    else 1

  check-refuses-impossible-days :: Int
  check-refuses-impossible-days =
    if valid 2023 2 29 then 2
    else if valid 2023 4 31 then 2
    else if valid 1900 2 29 then 2
    else if valid 2024 1 0 then 2
    else if valid 2024 0 1 then 2
    else if valid 2024 13 1 then 2
    else 0

  check-renders :: Int
  check-renders =
    if text-of 2024 2 29 == "2024-02-29" then
      if text-of 2026 10 29 == "2026-10-29" then 0 else 4
    else 4

  check-pads-and-signs-years :: Int
  check-pads-and-signs-years =
    if text-of 5 1 1 == "0005-01-01" then
      if text-of (-44) 3 15 == "-0044-03-15" then
        if text-of 12345 6 7 == "12345-06-07" then 0 else 8
      else 8
    else 8

  check-round-trips :: Int
  check-round-trips =
    if round-text 2024 2 29 then
      if round-text 2026 10 29 then
        if round-text 5 1 1 then
          if round-text (-44) 3 15 then
            if round-text 1970 1 1 then 0 else 16
          else 16
        else 16
      else 16
    else 16

  check-refuses-fields-on-parse :: Int
  check-refuses-fields-on-parse = if parse-kind "2023-02-29" == 2 then 0 else 32

  check-refuses-shapes-on-parse :: Int
  check-refuses-shapes-on-parse =
    if parse-kind "2024-2-9" == 1 then
      if parse-kind "junk" == 1 then
        if parse-kind "" == 1 then
          if parse-kind "2024-02-29T00:00:00" == 1 then
            if parse-kind "2024-02-29 " == 1 then
              if parse-kind "2024/02/29" == 1 then
                if parse-kind "123-01-01" == 1 then 0 else 64
              else 64
            else 64
          else 64
        else 64
      else 64
    else 64

  check-parses-padded-and-negative-years :: Int
  check-parses-padded-and-negative-years =
    if parse-kind "-0044-03-15" == 0 then
      if parse-kind "0005-01-01" == 0 then 0 else 128
    else 128

  main :: Int
  main = check-accepts-real-dates + check-refuses-impossible-days + check-renders + check-pads-and-signs-years + check-round-trips + check-refuses-fields-on-parse + check-refuses-shapes-on-parse + check-parses-padded-and-negative-years
"#,
    );
}

// ------------------------------------------------------------------ Time

#[test]
fn time_day_window_and_field_accessors() {
    expect_ok(
        "time-fields",
        r#"
module Main where
  import Data.Time
  import Data.Date (Date, date, from-epoch-days, to-string as date-text)

  mk :: Int -> Nat -> Nat -> Date
  mk y mo d = case date y mo d of
    None -> from-epoch-days 0
    Some x -> x

  mk-time :: Int -> Nat -> Nat -> Nat -> Time
  mk-time y mo d n = case time (mk y mo d) n of
    None -> from-epoch-nanos 0
    Some t -> t

  accepts :: Nat -> Bool
  accepts n = case time (mk 2024 2 29) n of
    None -> False
    Some _ -> True

  fields-at :: Nat -> (Nat, Nat, Nat, Nat)
  fields-at n = case time (mk 2024 2 29) n of
    None -> (99, 99, 99, 99)
    Some t -> (hour t, minute t, second t, nanosecond t)

  check-day-window :: Int
  check-day-window =
    if accepts 0 then
      if accepts 86399999999999 then
        if accepts 86400000000000 then 1 else 0
      else 1
    else 1

  check-midday-fields :: Int
  check-midday-fields = case fields-at 45296123456789 of
    (12, 34, 56, 123456789) -> 0
    _ -> 2

  check-midnight-fields :: Int
  check-midnight-fields = case fields-at 0 of
    (0, 0, 0, 0) -> 0
    _ -> 4

  check-day-end-fields :: Int
  check-day-end-fields = case fields-at 86399999999999 of
    (23, 59, 59, 999999999) -> 0
    _ -> 8

  check-observations :: Int
  check-observations =
    if nanoseconds-of-day (mk-time 2024 2 29 123) == 123 then
      if date-text (date-part (mk-time 2024 2 29 456)) == "2024-02-29" then 0 else 16
    else 16

  main :: Int
  main = check-day-window + check-midday-fields + check-midnight-fields + check-day-end-fields + check-observations
"#,
    );
}

#[test]
fn time_iso_8601_round_trips_and_refusals() {
    expect_ok(
        "time-iso",
        r#"
module Main where
  import Data.Time
  import Data.Date (Date, date, from-epoch-days, DateParseError, NotADate, InvalidDate)

  mk :: Int -> Nat -> Nat -> Date
  mk y mo d = case date y mo d of
    None -> from-epoch-days 0
    Some x -> x

  mk-time :: Int -> Nat -> Nat -> Nat -> Time
  mk-time y mo d n = case time (mk y mo d) n of
    None -> from-epoch-nanos 0
    Some t -> t

  midnight :: Int -> Nat -> Nat -> String
  midnight y mo d = to-string (mk-time y mo d 0)

  nanos-text :: Nat -> String
  nanos-text n = to-string (mk-time 2024 2 29 n)

  round-text :: String -> Bool
  round-text s = case from-string s of
    Err _ -> False
    Ok t -> to-string t == s

  parse-kind :: String -> Nat
  parse-kind s = case from-string s of
    Ok _ -> 0
    Err e -> case e of
      NotATime -> 1
      TimeFieldsOutOfRange -> 2
      BadDatePart de -> case de of
        NotADate -> 3
        InvalidDate -> 4

  check-plain :: Int
  check-plain =
    if midnight 2024 2 29 == "2024-02-29T00:00:00" then
      if round-text "2024-02-29T12:34:56" then 0 else 1
    else 1

  check-fraction :: Int
  check-fraction =
    if nanos-text 45296123456789 == "2024-02-29T12:34:56.123456789" then
      if round-text "2024-02-29T12:34:56.000000001" then 0 else 2
    else 2

  check-padded-and-pre-epoch :: Int
  check-padded-and-pre-epoch =
    if midnight 5 1 1 == "0005-01-01T00:00:00" then
      if round-text "1969-12-31T23:59:59.999999999" then 0 else 4
    else 4

  check-z-designator :: Int
  check-z-designator = case from-string "2024-02-29T12:34:56Z" of
    Err _ -> 8
    Ok t -> if to-string t == "2024-02-29T12:34:56" then 0 else 8

  check-refuses-field-overflow :: Int
  check-refuses-field-overflow =
    if parse-kind "2024-02-29T24:00:00" == 2 then
      if parse-kind "2024-02-29T00:60:00" == 2 then
        if parse-kind "2024-02-29T00:00:60" == 2 then 0 else 16
      else 16
    else 16

  check-refuses-shapes :: Int
  check-refuses-shapes =
    if parse-kind "2024-02-29T12:34" == 1 then
      if parse-kind "2024-02-29 12:34:56" == 1 then
        if parse-kind "" == 1 then
          if parse-kind "2024-02-29" == 1 then 0 else 32
        else 32
      else 32
    else 32

  check-refuses-fractions :: Int
  check-refuses-fractions =
    if parse-kind "2024-02-29T12:34:56." == 1 then
      if parse-kind "2024-02-29T12:34:56.1234567890" == 1 then 0 else 64
    else 64

  check-carries-the-date-error :: Int
  check-carries-the-date-error =
    if parse-kind "2023-02-29T00:00:00" == 4 then
      if parse-kind "2024-2-9T00:00:00" == 3 then 0 else 128
    else 128

  main :: Int
  main = check-plain + check-fraction + check-padded-and-pre-epoch + check-z-designator + check-refuses-field-overflow + check-refuses-shapes + check-refuses-fractions + check-carries-the-date-error
"#,
    );
}

#[test]
fn time_epoch_nanoseconds_durations_and_ordering() {
    expect_ok(
        "time-epoch",
        r#"
module Main where
  import Data.Time
  import Effect.Clock
  import Data.Date (Date, date, from-epoch-days)

  mk :: Int -> Nat -> Nat -> Date
  mk y mo d = case date y mo d of
    None -> from-epoch-days 0
    Some x -> x

  mk-time :: Int -> Nat -> Nat -> Nat -> Time
  mk-time y mo d n = case time (mk y mo d) n of
    None -> from-epoch-nanos 0
    Some t -> t

  plus-ms :: Int -> Nat -> Nat -> Nat -> Nat -> String
  plus-ms y mo d n ms = to-string (add-duration (duration-from-millis ms) (mk-time y mo d n))

  minus-ms :: Int -> Nat -> Nat -> Nat -> Nat -> String
  minus-ms y mo d n ms = to-string (sub-duration (duration-from-millis ms) (mk-time y mo d n))

  cmp-tag :: Time -> Time -> Nat
  cmp-tag a b = case cmp a b of
    LT -> 1
    EQ -> 2
    GT -> 3

  check-epoch-origin :: Int
  check-epoch-origin =
    if to-epoch-nanos (mk-time 1970 1 1 0) == 0 then
      if to-epoch-nanos (mk-time 1970 1 1 1) == 1 then 0 else 1
    else 1

  check-from-epoch-nanos :: Int
  check-from-epoch-nanos =
    if to-string (from-epoch-nanos 0) == "1970-01-01T00:00:00" then
      if to-string (from-epoch-nanos (-1)) == "1969-12-31T23:59:59.999999999" then 0 else 2
    else 2

  check-nanos-round-trip :: Int
  check-nanos-round-trip =
    let t = mk-time 2024 2 29 45296123456789 in
    if to-epoch-nanos (from-epoch-nanos (to-epoch-nanos t)) == to-epoch-nanos t then
      if to-string t == "2024-02-29T12:34:56.123456789" then 0 else 4
    else 4

  check-add-duration-carries-the-day :: Int
  check-add-duration-carries-the-day =
    if plus-ms 2024 2 28 86399500000000 500 == "2024-02-29T00:00:00" then
      if plus-ms 2023 2 28 86399000000000 1000 == "2023-03-01T00:00:00" then 0 else 8
    else 8

  check-sub-duration-carries-the-day :: Int
  check-sub-duration-carries-the-day =
    if minus-ms 2024 1 1 0 1000 == "2023-12-31T23:59:59" then
      if minus-ms 2023 3 1 0 1000 == "2023-02-28T23:59:59" then 0 else 16
    else 16

  check-diff :: Int
  check-diff =
    if duration-millis (diff (mk-time 2024 2 29 0) (mk-time 2024 2 29 1500000000)) == 1500 then
      if duration-millis (diff (mk-time 2024 2 29 1500000000) (mk-time 2024 2 29 0)) == 1500 then
        if duration-millis (diff (mk-time 2024 2 29 0) (mk-time 2024 2 29 500000)) == 0 then 0 else 32
      else 32
    else 32

  check-ordering :: Int
  check-ordering =
    if cmp-tag (mk-time 2024 2 29 0) (mk-time 2024 2 29 1) == 1 then
      if cmp-tag (mk-time 2024 2 29 1) (mk-time 2024 2 29 1) == 2 then
        if cmp-tag (mk-time 2024 3 1 0) (mk-time 2024 2 29 0) == 3 then
          if (mk-time 2024 2 29 7) == (mk-time 2024 2 29 7) then 0 else 64
        else 64
      else 64
    else 64

  check-clock-pairing :: Int
  check-clock-pairing =
    let wall = int-from-i64 (nat-to-i64 45296123456789) in
    if to-epoch-nanos (from-epoch-nanos wall) == wall then 0 else 128

  main :: Int
  main = check-epoch-origin + check-from-epoch-nanos + check-nanos-round-trip + check-add-duration-carries-the-day + check-sub-duration-carries-the-day + check-diff + check-ordering + check-clock-pairing
"#,
    );
}

// ----------------------------------------------------------- the project

#[test]
fn modules_load_as_a_multi_file_project() {
    // Both family sources beside one `Main`: the imports resolve
    // (`Data.Time` pulls `Data.Date` and `Effect.Clock`), and both
    // modules are usable from one program. `Data.Date` is selected by
    // name because its `cmp`/`to-string`/`from-string` are also
    // `Data.Time`'s (an unselected whole import of both is refused
    // naming the collision).
    expect_ok(
        "project",
        r#"
module Main where
  import Data.Date (Date, date, from-epoch-days, to-epoch-days)
  import Data.Time

  epoch :: Int
  epoch = case date 2024 2 29 of
    None -> 1
    Some d -> if to-epoch-days d == 19782 then 0 else 2

  nanos :: Int
  nanos = case time (from-epoch-days 0) 123 of
    None -> 4
    Some t -> if nanoseconds-of-day t == 123 then 0 else 8

  main :: Int
  main = epoch + nanos
"#,
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
  import Data.Date

  check-a :: Int
  check-a = 0

  check-b :: Int
  check-b = 0

  check-c :: Int
  check-c = if is-leap-year 1900 then 0 else 4

  main :: Int
  main = check-a + check-b + check-c
"#,
        4,
    );
}
