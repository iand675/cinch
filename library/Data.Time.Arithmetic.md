# Data.Time.Arithmetic

Calendar-aware spans applied to dates, wall-clock times, date-times and instants, with explicit overflow results.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Time.Arithmetic`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Time.Arithmetic.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Span](Data.Time.Span.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Unit](Data.Time.Unit.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Time.Arithmetic](Data.Time.Arithmetic.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Span](Data.Time.Span.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Unit](Data.Time.Unit.md), [Data.Time.Weekday](Data.Time.Weekday.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Time.Arithmetic: spans applied to dates, times and instants.

Adding a `Span` to a civil value, taking the `Span` between two
values, and rounding a value to a unit. The semantics are TC39
Temporal's (PlainDate, PlainDateTime, PlainTime and Instant: `add`,
`subtract`, `until`, `since`, `round`) as rendered by Jiff; the
algorithms themselves live in Data.Time.Span (the engine, which the
anchored span operations share) and this module gives them their
everyday names, defaults and option values. Zoned arithmetic is
Data.Time.Zoned.Arithmetic (lane F).

Design law shape.
  - Adding (`date-add-span`, `datetime-add-span`). Units are applied
    largest first: years and months move the month exactly and
    constrain the day to the target month's length
    (2024-05-31 + 1 month = 2024-06-30; 2025-01-31 + 1 month =
    2025-02-28), then weeks and days, then the clock units, a civil
    day being 24 hours. A `Date` has no time of day: the clock units
    of a span added to a date are balanced into whole days, truncated
    toward zero, and the rest is dropped (PlainDate.prototype.add).
    `-sub-span` adds the negated span. Adding a span and then
    subtracting it does NOT always return the start
    (2024-05-31 + 1 month - 1 month = 2024-05-30): the day was
    constrained on the way out.
  - `time-add-span-wrapping` wraps around midnight and IGNORES the
    span's calendar units (PlainTime.prototype.add: a whole number of
    days cannot move a time of day).
  - `timestamp-add-span` accepts clock units only: an instant has no
    calendar, so a span with a non-zero day or larger unit is
    `Err SpanNeedsAnchor` (Instant.prototype.add). Convert through a
    zone to add calendar units.
  - Differences. `x-until a b` is the span from `a` to `b` (positive
    when `b` is later) and `x-since a b` is the span since `b`,
    measured to `a` (positive when `b` is earlier). The defaults are
    Temporal's: the largest unit is `Day` for dates and date-times,
    `Hour` for times of day and `Second` for timestamps; the smallest
    is `Nanosecond` (`Day` for dates); the increment is 1 and the
    mode `Trunc`. With the defaults `x-since a b` is
    `span-negate (x-until a b)`, and `x-until a b` added to `a` is
    `b`.
    With a calendar largest unit the count is anchored at the FIRST
    argument: the largest whole number of years, then months, that
    added to `a` does not pass `b`, then the days left over. So
    `until` and `add` are inverses from the start
    (`a + (a until b) = b` whenever no rounding is asked for) but
    not from the end: 2024-01-31 until 2024-03-01 with largest
    `Month` is 1 month 1 day (January 31 plus one month is February
    29, plus one day), while 2024-03-01 minus that span is
    2024-01-31 only by luck of the month lengths, and
    2024-03-31 until 2024-02-29 is -1 month although
    2024-02-29 + 1 month is 2024-03-29.
  - The `-with` forms take a `Difference` (T7): `with-largest`,
    `with-smallest`, `with-increment`, `with-mode` over
    `difference-default`. An unset largest unit is the larger of the
    type's default and the smallest unit. Rounding to a calendar
    smallest unit is relative to the first argument's calendar
    (Temporal's RoundRelativeDuration). For `-since-with` the
    rounding mode is negated before the difference is taken and the
    result is negated after (so `Ceil` still means "toward positive
    infinity" for the span returned), exactly as Temporal specifies.
  - Rounding a value (`time-round`, `datetime-round`,
    `timestamp-round`) takes a `Round`: smallest unit, increment,
    mode, by default `Nanosecond`, 1, `HalfExpand`. The increment
    must divide the next larger unit evenly and be smaller than it
    (30 minutes, 6 hours; not 7 minutes, not 60 minutes); a
    `DateTime` may be rounded to `Day` with increment 1, which can
    carry into the date; a `TimeOfDay` rounded up at the end of the
    day wraps to midnight; a `Timestamp`'s increment must divide a
    24-hour day and it rounds on the number line whatever its sign
    (`Trunc` is toward the past on both sides of the epoch).

No operation faults (T2). Additions answer `Option` (the one refusal
is leaving the range) except on timestamps, which have two; default
differences are total; everything with options answers
`Result SpanError`. This module has no error type of its own: its
refusals are the span's (`Data.Time.Span.SpanError`).

NAMED REFUSALS:
  - the names `timestamp-until` / `timestamp-since`: taken by
    Data.Time.Timestamp (lane A), where they answer a `Duration`.
    The span forms are `timestamp-until-span` and
    `timestamp-since-span`.
  - `Unit`: the unit type is `TimeUnit` (see Data.Time.Unit).
  - `Option` for value rounding: `time-round`, `datetime-round` and
    `timestamp-round` answer `Result SpanError`, because besides the
    range end there are two more refusals (a unit the value cannot be
    rounded to, an invalid increment).
  - one overloaded `until`: a difference's result class depends on
    the operand type (`Span Calendar` for dates and date-times,
    `Span Clock` for times and timestamps), so each type has its own
    function.

</details>

# Types and constructors

<a id="type-446966666572656e6365"></a>

## `Difference`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Difference</span></code></pre>

Largest unit (unset: the larger of the operand type's default and
the smallest unit), smallest unit (unset: the operand type's
default), increment, mode.

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 198

<a id="type-526f756e64"></a>

## `Round`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Round</span></code></pre>

Smallest unit, increment, mode.

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 393

# Values

<a id="value-646174652d6164642d7370616e"></a>

## `date-add-span`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">date-add-span</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>))</code></pre>

The date `s` after `x` (header: largest unit first, the day
constrained, the clock units balanced into whole days). `None`
when the result leaves -9999-01-01..=9999-12-31.

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 148

<a id="value-646174652d73696e6365"></a>

## `date-since`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">date-since</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))</code></pre>

The days since `b`, to `a`: the negation of `date-until a b`.

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 267

<a id="value-646174652d73696e63652d77697468"></a>

## `date-since-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">date-since-with</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">Difference</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))))</code></pre>

[Source](/reference-sources/Data.Time.Arithmetic.cinch)

<a id="value-646174652d7375622d7370616e"></a>

## `date-sub-span`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">date-sub-span</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>))</code></pre>

[Source](/reference-sources/Data.Time.Arithmetic.cinch)

<a id="value-646174652d756e74696c"></a>

## `date-until`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">date-until</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))</code></pre>

The days from `a` to `b`: 2024-02-01 until 2024-03-01 is 29 days.
Total: the whole range is 7304483 days, inside the day limit, so
the `None` arm is unreachable.

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 261

<a id="value-646174652d756e74696c2d77697468"></a>

## `date-until-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">date-until-with</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">Difference</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))))</code></pre>

The span from `a` to `b` under the options. Units are `Day` to
`Year`; the default smallest unit is `Day`.
2024-05-31 until 2024-09-30 with largest `Month` is 3 months 30
days.
  Err SpanBadUnits      a clock unit, or largest below smallest
  Err SpanBadIncrement  the increment is below 1 or above
                        1000000000
  Err SpanOutOfRange    a rounding candidate leaves the date range

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 285

<a id="value-6461746574696d652d6164642d7370616e"></a>

## `datetime-add-span`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">datetime-add-span</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>))</code></pre>

The date-time `s` after `x`: 2024-02-01T17:30 plus 1 month 1 hour
30 minutes is 2024-03-01T19:00. `None` when the date leaves the
range.

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 157

<a id="value-6461746574696d652d726f756e64"></a>

## `datetime-round`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">datetime-round</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">Round</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>)))</code></pre>

Round a date-time (PlainDateTime.prototype.round): the time of day
is rounded and a carry moves the date, so
2024-11-02T17:46:20.123456789 to `Day` is 2024-11-03T00:00.
  Err SpanBadUnits      week or larger
  Err SpanBadIncrement  as for `time-round`; for `Day`, anything
                        but 1
  Err SpanOutOfRange    the carry leaves the date range

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 457

<a id="value-6461746574696d652d73696e6365"></a>

## `datetime-since`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">datetime-since</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))</code></pre>

[Source](/reference-sources/Data.Time.Arithmetic.cinch)

<a id="value-6461746574696d652d73696e63652d77697468"></a>

## `datetime-since-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">datetime-since-with</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">Difference</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))))</code></pre>

[Source](/reference-sources/Data.Time.Arithmetic.cinch)

<a id="value-6461746574696d652d7375622d7370616e"></a>

## `datetime-sub-span`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">datetime-sub-span</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>))</code></pre>

[Source](/reference-sources/Data.Time.Arithmetic.cinch)

<a id="value-6461746574696d652d756e74696c"></a>

## `datetime-until`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">datetime-until</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))</code></pre>

The span from `a` to `b` in days and clock units, exact:
2024-05-15T14:30:54 until 2024-05-31T19:13:27.123456789 is 16 days
4 hours 42 minutes 33 seconds 123 ms 456 us 789 ns. Total: at most
7304483 days and less than a day of clock units, all inside their
limits, and nothing is rounded, so the `Err` arm is unreachable.

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 300

<a id="value-6461746574696d652d756e74696c2d77697468"></a>

## `datetime-until-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">datetime-until-with</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">Difference</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))))</code></pre>

The span from `a` to `b` under the options; every unit is allowed.
  Err SpanBadUnits      largest below smallest
  Err SpanBadIncrement  the increment is not valid for the
                        smallest unit
  Err SpanOutOfRange    a rounding candidate leaves the date range
  Err SpanUnitLimit     a unit of the result passes its limit

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 320

<a id="value-646966666572656e63652d64656661756c74"></a>

## `difference-default`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">difference-default</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">Difference</span></code></pre>

Both units unset, increment 1, `Trunc`.

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 202

<a id="value-646966666572656e63652d6c617267657374"></a>

## `difference-largest`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">difference-largest</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">Difference</span>)</code></pre>

The common case: the default with a largest unit.

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 206

<a id="value-726f756e642d64656661756c74"></a>

## `round-default`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">round-default</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">Round</span></code></pre>

`Nanosecond`, 1, `HalfExpand`: rounds nothing until a unit is set.

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 397

<a id="value-726f756e642d746f"></a>

## `round-to`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">round-to</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">Round</span>)</code></pre>

The common case: the default with a smallest unit.

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 401

<a id="value-74696d652d6164642d7370616e2d7772617070696e67"></a>

## `time-add-span-wrapping`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">time-add-span-wrapping</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span>))</code></pre>

The time of day `s` after `t`, wrapping around midnight. The
span's calendar units (days and larger) are ignored, as Temporal
specifies. Total.

The clock nanoseconds (below 1.3e21) are reduced modulo one day
first, so the count handed to lane B's wrapping addition is within
+-86399999999999 and the narrowing is exact.

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 170

<a id="value-74696d652d726f756e64"></a>

## `time-round`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">time-round</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">Round</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span>)))</code></pre>

Round a time of day (PlainTime.prototype.round). Rounding up past
the end of the day wraps to midnight: 23:59:40 to the minute is
00:00.
  Err SpanBadUnits      a calendar unit
  Err SpanBadIncrement  the increment does not divide the next
                        larger unit, or equals it

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 445

<a id="value-74696d652d73696e6365"></a>

## `time-since`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">time-since</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>))</code></pre>

[Source](/reference-sources/Data.Time.Arithmetic.cinch)

<a id="value-74696d652d73696e63652d77697468"></a>

## `time-since-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">time-since-with</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">Difference</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>))))</code></pre>

[Source](/reference-sources/Data.Time.Arithmetic.cinch)

<a id="value-74696d652d7375622d7370616e2d7772617070696e67"></a>

## `time-sub-span-wrapping`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">time-sub-span-wrapping</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span>))</code></pre>

[Source](/reference-sources/Data.Time.Arithmetic.cinch)

<a id="value-74696d652d756e74696c"></a>

## `time-until`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">time-until</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>))</code></pre>

The span from `a` to `b` within one day, in hours and smaller;
negative when `b` is the earlier time (no wrapping). Total: less
than 24 hours, so the `Err` arm is unreachable.

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 333

<a id="value-74696d652d756e74696c2d77697468"></a>

## `time-until-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">time-until-with</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">Difference</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>))))</code></pre>

Units are `Nanosecond` to `Hour`; the default largest unit is
`Hour`.
  Err SpanBadUnits      a calendar unit, or largest below smallest
  Err SpanBadIncrement  the increment is not valid for the
                        smallest unit

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 352

<a id="value-74696d657374616d702d6164642d7370616e"></a>

## `timestamp-add-span`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-add-span</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>))</code></pre>

The instant `s` after `t`. Only clock units are allowed
(Instant.prototype.add).
  Err SpanNeedsAnchor  a day or larger unit of `s` is non-zero
  Err SpanOutOfRange   the result leaves the timestamp range

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 181

<a id="value-74696d657374616d702d726f756e64"></a>

## `timestamp-round`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-round</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">Round</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)))</code></pre>

Round an instant (Instant.prototype.round). The increment, in its
unit, must divide a 24-hour day evenly (it may equal it). The
rounding is on the number line, the same on both sides of the
epoch (RoundNumberToIncrementAsIfPositive).
  Err SpanBadUnits      a calendar unit
  Err SpanBadIncrement  the increment does not divide a day
  Err SpanOutOfRange    the rounded instant leaves the range

The quantum is at most 86400e9 and the count within +-3.8e20, so
the kernel cannot refuse; its `None` arm is mapped to a refusal
anyway.

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 477

<a id="value-74696d657374616d702d73696e63652d7370616e"></a>

## `timestamp-since-span`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-since-span</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>))</code></pre>

[Source](/reference-sources/Data.Time.Arithmetic.cinch)

<a id="value-74696d657374616d702d73696e63652d7370616e2d77697468"></a>

## `timestamp-since-span-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-since-span-with</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">Difference</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>))))</code></pre>

[Source](/reference-sources/Data.Time.Arithmetic.cinch)

<a id="value-74696d657374616d702d7375622d7370616e"></a>

## `timestamp-sub-span`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-sub-span</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>))</code></pre>

[Source](/reference-sources/Data.Time.Arithmetic.cinch)

<a id="value-74696d657374616d702d756e74696c2d7370616e"></a>

## `timestamp-until-span`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-until-span</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>))</code></pre>

The span from `a` to `b` in seconds and smaller (Temporal's default
for instants). Total: the whole timestamp range is fewer than
631107417600 seconds, the second limit, so the `Err` arm is
unreachable.

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 366

<a id="value-74696d657374616d702d756e74696c2d7370616e2d77697468"></a>

## `timestamp-until-span-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timestamp-until-span-with</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">Difference</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>))))</code></pre>

Units are `Nanosecond` to `Hour`; the default largest unit is
`Second`.
  Err SpanBadUnits      a calendar unit, or largest below smallest
  Err SpanBadIncrement  the increment is not valid for the
                        smallest unit
  Err SpanUnitLimit     a unit of the result passes its limit
                        (the whole range in nanoseconds does not
                        fit the nanosecond unit)

[Source](/reference-sources/Data.Time.Arithmetic.cinch) · line 382

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Time.Arithmetic.LargestOption_Difference` | `Data.Time.Unit.LargestOption Data.Time.Arithmetic.Difference` | Implicit |
| `Data.Time.Arithmetic.RoundingOptions_Difference` | `Data.Time.Unit.RoundingOptions Data.Time.Arithmetic.Difference` | Implicit |
| `Data.Time.Arithmetic.RoundingOptions_Round` | `Data.Time.Unit.RoundingOptions Data.Time.Arithmetic.Round` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Time/Arithmetic.cinch`. Source SHA-256: `79b5d74e2656408fa3faea242d6cd61e557304402eb839d827fd01645e9c6c38`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
