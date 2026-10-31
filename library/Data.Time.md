# Data.Time

Time values, durations, and conversions with explicit units and admitted ranges.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Time`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Time.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Date](Data.Date.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Clock](Effect.Clock.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Date](Data.Date.md), [Data.Time](Data.Time.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Time: the civil date-time: a calendar date plus a time of day.

A `Time` is a `Date` (Data.Date's proleptic Gregorian calendar date)
and a nanosecond count since midnight, in `0..=86399999999999`. The
representation constructor is not exported (D143's surface rule):
`time` is the only way in, and it refuses a nanosecond count at or
past the day's end, so a time of day outside the day cannot exist.
Duration arithmetic moves on the wall-nanosecond line below and can
carry the day, which is why a "time" here is a date-time and not a
clock reading.

The instant conversions are UTC-EXPLICIT: the Unix epoch in
nanoseconds (`to-epoch-nanos`/`from-epoch-nanos`) is the quantity
`Effect.Clock.clock-wall-ns` reads, and epoch days come from
Data.Date, so the two families agree by construction and no offset
enters the arithmetic. The clock hands that reading back as a `Nat`
and `from-epoch-nanos` takes the `Int`, so a consumer pairs them with
the prelude's `Nat`/`Int` conversion (`nat-to-int n`, of any size);
the clock's own clamp (the epoch to the year 2262) is its boundary,
not this family's. Nothing in this slice consults a timezone
database, because no such thing exists here.

Design law shape. On top of Data.Date's calendar laws: the
wall-nanosecond conversions invert on the whole `Int` line, with
`to-epoch-nanos (1970-01-01T00:00:00)` at 0 by construction;
`add-duration` and `sub-duration` undo each other, and
`add-duration (duration-from-millis m) t` moves exactly m
milliseconds, carrying into the date (2024-02-28T23:59:59 + 1s is
2024-02-29T00:00:00) and, backwards, out of the year
(2024-01-01T00:00:00 - 1s is 2023-12-31T23:59:59); `diff` is the
`Duration` between two times and agrees with `cmp` (`diff` is 0
exactly when `cmp` is `EQ`); `cmp` is a total order on the
wall-nanosecond line; `hour`/`minute`/`second`/`nanosecond` are the
fields of `nanoseconds-of-day`; and ISO 8601 `to-string`/
`from-string` round trip. The checks are
compiler/tests/lib_civil_time.rs.

NAMED REFUSALS (boundaries this module deliberately does not
cross):
  - timezones, UTC offsets and zone conversion: a `Time` carries no
    zone (leap seconds are not modelled either), so every conversion
    here is UTC-explicit; there is no local-time or offset surface
    and no tz database to consult.
  - locale-aware formatting: the ISO 8601 `YYYY-MM-DDTHH:MM:SS`
    rendering is the only one, and there are no locale or name data
    to render otherwise.
  - leap seconds: a day is exactly 86400 seconds by construction, so
    `second` never reads 60 and no `23:59:60` exists (the calendar
    has no table of inserted leap seconds, and the runtime clock
    counts SI nanoseconds, which are exactly what this family
    stores).
  - a `Time` before 1970 in `Duration` arithmetic: `Duration` is an
    unsigned millisecond count (Effect.Clock's `Duration :: Nat ->
    Duration`), so an instant before the epoch is representable as a
    `Time` and as an epoch nanosecond, but the distance `diff`
    reports has no sign; `cmp` is where direction lives.
  - sub-millisecond `Duration`s: `Duration` is milliseconds, so
    `add-duration`/`sub-duration`/`diff` move whole milliseconds and
    the nanosecond field is untouched by them.

Home (design/base-library.md): `Data.*` holds the concrete types, so
the `Eq`/`Ord` instances live with `Time` here; `Duration` is
Effect.Clock's (the P34 capability vocabulary). Ported item: Ruby's
`Time` and `DateTime` (design/stdlib-parity-ruby.md, P38 "Time:
civil `Date`/`Time`/`DateTime` (calendars, formatting, parsing)
atop the P34 `Clock` capability").

</details>

# Types and constructors

<a id="type-54696d65"></a>

## `Time`

```cinch
data Time
```

The civil date-time: the date and the nanoseconds since its
midnight, below one day. Not exported (the invariant above);
`time` is the only constructor.

[Source](/reference-sources/Data.Time.cinch) · line 95

```cinch
Time :: (Date -> (Nat -> Time))
```

The civil date-time: the date and the nanoseconds since its
midnight, below one day. Not exported (the invariant above);
`time` is the only constructor.

<a id="type-54696d6550617273654572726f72"></a>

## `TimeParseError`

```cinch
data TimeParseError
```

The literal reader's errors, one per refusal: the text is not a
date-time, its time fields are out of range, or its date half
failed its own reader (carried through unchanged, so the date
diagnostics survive the composition).

[Source](/reference-sources/Data.Time.cinch) · line 102

```cinch
NotATime :: TimeParseError
```

```cinch
TimeFieldsOutOfRange :: TimeParseError
```

```cinch
BadDatePart :: (DateParseError -> TimeParseError)
```

# Values

<a id="value-6164642d6475726174696f6e"></a>

## `add-duration`

```cinch
add-duration :: (Duration -> (Time -> Time))
```

Move forward by `d` (a whole number of milliseconds; the day
carries).

[Source](/reference-sources/Data.Time.cinch) · line 167

<a id="value-636d70"></a>

## `cmp`

```cinch
cmp :: (Time -> (Time -> Ordering))
```

[Source](/reference-sources/Data.Time.cinch) · line 190

<a id="value-646174652d70617274"></a>

## `date-part`

```cinch
date-part :: (Time -> Date)
```

[Source](/reference-sources/Data.Time.cinch) · line 126

<a id="value-64696666"></a>

## `diff`

```cinch
diff :: (Time -> (Time -> Duration))
```

The distance between two instants in whole milliseconds (truncated
toward zero): unsigned, because `Duration` is, so the caller reads
direction from `cmp` (the refusal above).

[Source](/reference-sources/Data.Time.cinch) · line 183

<a id="value-66726f6d2d65706f63682d6e616e6f73"></a>

## `from-epoch-nanos`

```cinch
from-epoch-nanos :: (Int -> Time)
```

The inverse: every `Int` is an instant, so the result is total and
the floored split keeps the day's remainder non-negative for the
pre-epoch instants too.

[Source](/reference-sources/Data.Time.cinch) · line 159

<a id="value-66726f6d2d737472696e67"></a>

## `from-string`

```cinch
from-string :: (String -> Result TimeParseError Time)
```

Strict ISO 8601 date-time: the date half (Data.Date's reader, so
its `NotADate`/`InvalidDate` survive as `BadDatePart`), `T`, then
`HH:MM:SS` with all fields two digits, an optional `.` and 1..9
fraction digits, and an optional trailing `Z`, with nothing before
or after. `NotATime` is the shape refusal (a space separator, no
seconds, an empty fraction, junk after `Z`); `TimeFieldsOutOfRange`
is the fields refusal (`24:00:00`, `00:60:00`).

[Source](/reference-sources/Data.Time.cinch) · line 215

<a id="value-686f7572"></a>

## `hour`

```cinch
hour :: (Time -> Nat)
```

[Source](/reference-sources/Data.Time.cinch) · line 134

<a id="value-6d696e757465"></a>

## `minute`

```cinch
minute :: (Time -> Nat)
```

[Source](/reference-sources/Data.Time.cinch) · line 137

<a id="value-6e616e6f7365636f6e64"></a>

## `nanosecond`

```cinch
nanosecond :: (Time -> Nat)
```

[Source](/reference-sources/Data.Time.cinch) · line 143

<a id="value-6e616e6f7365636f6e64732d6f662d646179"></a>

## `nanoseconds-of-day`

```cinch
nanoseconds-of-day :: (Time -> Nat)
```

[Source](/reference-sources/Data.Time.cinch) · line 130

<a id="value-7365636f6e64"></a>

## `second`

```cinch
second :: (Time -> Nat)
```

[Source](/reference-sources/Data.Time.cinch) · line 140

<a id="value-7375622d6475726174696f6e"></a>

## `sub-duration`

```cinch
sub-duration :: (Duration -> (Time -> Time))
```

Move backward by `d`.

[Source](/reference-sources/Data.Time.cinch) · line 171

<a id="value-74696d65"></a>

## `time`

```cinch
time :: (Date -> (Nat -> Option Time))
```

The only constructor: a nanosecond count inside the day. At or past
the day's end is `None`, so an out-of-range `Time` has no
representation.

[Source](/reference-sources/Data.Time.cinch) · line 123

<a id="value-746f2d65706f63682d6e616e6f73"></a>

## `to-epoch-nanos`

```cinch
to-epoch-nanos :: (Time -> Int)
```

The wall-nanosecond line (the Unix epoch; `clock-wall-ns` reads
this quantity) and the duration arithmetic on it.

Nanoseconds since 1970-01-01T00:00:00Z (negative before the epoch).

[Source](/reference-sources/Data.Time.cinch) · line 152

<a id="value-746f2d737472696e67"></a>

## `to-string`

```cinch
to-string :: (Time -> String)
```

ISO 8601 rendering and reading.

`YYYY-MM-DDTHH:MM:SS` (the date half is Data.Date's rendering), with
`.fffffffff` appended when the sub-second field is non-zero. No
offset designator is emitted: the value is UTC by construction, and
the reader accepts a trailing `Z` (RFC 3339's UTC designator) as
well as its absence.

[Source](/reference-sources/Data.Time.cinch) · line 204

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Time.Eq_Time` | `Eq Time` | Implicit |
| `Data.Time.Ord_Time` | `Ord Time` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Time.cinch`. Source SHA-256: `6847730c2f3ac9f9200a4eef90a5ab7797facdc3555fb3e2f337f8b07569bc3a`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
