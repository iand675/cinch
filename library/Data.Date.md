# Data.Date

Civil date values, validation, parsing/formatting, and calendar calculations.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Date`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Date.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Date](Data.Date.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Date: the proleptic Gregorian calendar date.

A `Date` is a calendar date in the proleptic Gregorian calendar: a
year (`Int`, so the years before 1 are representable and the year 0
is ISO 8601's astronomical year 0), a month (1..=12) and a day of
month (1..=that month's length that year). The representation
constructor is not exported (D143's surface rule is what makes the
omission bind): `date` is the only way in, and it refuses a month
outside 1..=12 and a day outside the month's own length, so an
invalid date cannot exist. Arithmetic preserves the invariant by
construction: days move on the epoch-day line
(`to-epoch-days`/`from-epoch-days`), never field by field.

Design law shape. A year is a leap year exactly when it is divisible
by 4 and not by 100 unless also by 400 (`is-leap-year`), and that one
predicate fixes every month length (`days-in-month`). On top of it:
the epoch-day conversions invert on the whole line, so
`to-epoch-days` is a bijection with `from-epoch-days` and 1970-01-01
is 0 by construction; `add-days` and `sub-days` undo each other and
`diff-days` measures their difference (`diff-days d (add-days n d)`
is n, and adding m then n days is adding m + n); `cmp` is a total
order whose sign is `diff-days`'s; `day-of-year` is 1 on January 1
and the year's length on December 31; `day-of-week` is
`to-epoch-days` read modulo 7 with 1970-01-01 (a Thursday) at 4,
which is Ruby's `wday` convention (0 is Sunday); and ISO 8601
`to-string`/`from-string` round trip. The checks are
compiler/tests/lib_civil_time.rs.

NAMED REFUSALS (boundaries this module deliberately does not
cross):
  - timezones and UTC offsets: a `Date` is a calendar value, not an
    instant, and carries no zone; no offset and no tz database exist
    in this slice (Data.Time's conversions are UTC-explicit, and its
    wall-nanosecond line is the only instant surface here).
  - locale-aware formatting, and month or weekday names: the
    bootstrap carries no locale or name data, so the ISO 8601
    `YYYY-MM-DD` rendering is the only one.
  - the Julian calendar and the Gregorian reform: the proleptic
    Gregorian calendar is extended before 1582 by definition, and no
    reform cutover is modelled (a date before it is read and written
    proleptically).
  - a day count wider than `I64`: `add-days`/`sub-days` take `Nat`
    counts and convert through `nat-to-int`, the machine-width step
    every `Nat`-to-`Int` conversion in the prelude takes (a count
    above `I64`'s range faults in the runtime conversion).

Home (design/base-library.md): `Data.*` holds the concrete types, so
the `Eq`/`Ord` instances live with `Date` here. Ported item: Ruby's
`Date` (design/stdlib-parity-ruby.md, P38 "Time: civil
`Date`/`Time`/`DateTime` (calendars, formatting, parsing) atop the
P34 `Clock` capability").

</details>

# Types and constructors

<a id="type-44617465"></a>

## `Date`

```cinch
data Date
```

The calendar date: year, month, day of month. Not exported (the
validity invariant above; `date` is the only constructor).

[Source](/reference-sources/Data.Date.cinch) · line 78

```cinch
Date :: (Int -> (Nat -> (Nat -> Date)))
```

The calendar date: year, month, day of month. Not exported (the
validity invariant above; `date` is the only constructor).

<a id="type-4461746550617273654572726f72"></a>

## `DateParseError`

```cinch
data DateParseError
```

The literal reader's errors, one per refusal: the text is not a
date at all (`[-]dddd-MM-DD`), or the fields name no date. A parse
never yields a partially trusted value, so there is no third case.

[Source](/reference-sources/Data.Date.cinch) · line 84

```cinch
NotADate :: DateParseError
```

```cinch
InvalidDate :: DateParseError
```

# Values

<a id="value-6164642d64617973"></a>

## `add-days`

```cinch
add-days :: (Nat -> (Date -> Date))
```

`n` days later (crossing month and year boundaries by construction).

[Source](/reference-sources/Data.Date.cinch) · line 184

<a id="value-636d70"></a>

## `cmp`

```cinch
cmp :: (Date -> (Date -> Ordering))
```

[Source](/reference-sources/Data.Date.cinch) · line 197

<a id="value-64617465"></a>

## `date`

```cinch
date :: (Int -> (Nat -> (Nat -> Option Date)))
```

Construction and observation.

The only constructor: month 1..=12, day 1..=that month's length.
`None` is the refusal for every other field combination, so an
out-of-range `Date` has no representation.

[Source](/reference-sources/Data.Date.cinch) · line 95

<a id="value-646179"></a>

## `day`

```cinch
day :: (Date -> Nat)
```

[Source](/reference-sources/Data.Date.cinch) · line 111

<a id="value-6461792d6f662d7765656b"></a>

## `day-of-week`

```cinch
day-of-week :: (Date -> Nat)
```

The weekday, 0 for Sunday through 6 for Saturday (Ruby's `wday`).
1970-01-01 (day 0) is a Thursday, hence the +4; the modulo is
floored so the pre-epoch days land in range too.

[Source](/reference-sources/Data.Date.cinch) · line 211

<a id="value-6461792d6f662d79656172"></a>

## `day-of-year`

```cinch
day-of-year :: (Date -> Nat)
```

1 on January 1, the year's length on December 31.

[Source](/reference-sources/Data.Date.cinch) · line 203

<a id="value-646179732d696e2d6d6f6e7468"></a>

## `days-in-month`

```cinch
days-in-month :: (Int -> (Nat -> Nat))
```

The month's length in `y`: 31, 30, 29 or 28. A month outside
1..=12 is 0 (out of the calendar; `date` refuses it before this
is consulted with one).

[Source](/reference-sources/Data.Date.cinch) · line 127

<a id="value-646966662d64617973"></a>

## `diff-days`

```cinch
diff-days :: (Date -> (Date -> Int))
```

The days elapsed from the first argument to the second: `diff-days
start finish`, negative when `finish` precedes `start`. The exact
inverse of `add-days`/`sub-days` in both directions.

[Source](/reference-sources/Data.Date.cinch) · line 194

<a id="value-66726f6d2d65706f63682d64617973"></a>

## `from-epoch-days`

```cinch
from-epoch-days :: (Int -> Date)
```

The inverse: every `Int` is a day, so the result is total. The
fields come out of the decomposition already in range (the
construction is the proof), so the private constructor is used
directly rather than routing through `date`'s `Option`.

[Source](/reference-sources/Data.Date.cinch) · line 169

<a id="value-66726f6d2d737472696e67"></a>

## `from-string`

```cinch
from-string :: (String -> Result DateParseError Date)
```

Strict ISO 8601: an optional `-`, at least four year digits, and
two-digit month and day separated by `-`, with nothing before or
after. `NotADate` is the shape refusal (`2024-2-9`, `junk`, a
trailing time part); `InvalidDate` is the fields refusal
(`2023-02-29`, month 13, day 0).

[Source](/reference-sources/Data.Date.cinch) · line 241

<a id="value-69732d6c6561702d79656172"></a>

## `is-leap-year`

```cinch
is-leap-year :: (Int -> Bool)
```

The Gregorian leap rule, total over `Int` (the proleptic calendar
defines it for every year, including year 0 and the negative
ones). Ruby's `Date.gregorian_leap?` is the same predicate.

[Source](/reference-sources/Data.Date.cinch) · line 118

<a id="value-6d6f6e7468"></a>

## `month`

```cinch
month :: (Date -> Nat)
```

[Source](/reference-sources/Data.Date.cinch) · line 107

<a id="value-7375622d64617973"></a>

## `sub-days`

```cinch
sub-days :: (Nat -> (Date -> Date))
```

`n` days earlier.

[Source](/reference-sources/Data.Date.cinch) · line 188

<a id="value-746f2d65706f63682d64617973"></a>

## `to-epoch-days`

```cinch
to-epoch-days :: (Date -> Int)
```

The epoch-day line (1970-01-01 is day 0) and the calendar
arithmetic on it.

The day number of `d` counted from 1970-01-01 (negative before it).
This is the shifted-era decomposition (days-from-civil), which is
exactly the calendar's month term plus the year term; the era and
year-of-era steps use FLOORED division so the decomposition stays
at a non-negative remainder for the negative years too.

[Source](/reference-sources/Data.Date.cinch) · line 153

<a id="value-746f2d737472696e67"></a>

## `to-string`

```cinch
to-string :: (Date -> String)
```

[Source](/reference-sources/Data.Date.cinch) · line 227

<a id="value-79656172"></a>

## `year`

```cinch
year :: (Date -> Int)
```

[Source](/reference-sources/Data.Date.cinch) · line 103

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Date.Eq_Date` | `Eq Date` | Implicit |
| `Data.Date.Ord_Date` | `Ord Date` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Date.cinch`. Source SHA-256: `010d01be4d6ef58d6642ba06d09cac5782186ed9f20e2e627864672b9eaa6233`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
