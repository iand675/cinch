# Data.Time.Arithmetic

Calendar-aware spans applied to dates, wall-clock times, date-times and instants, with explicit overflow results.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Time.Arithmetic</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Time.Arithmetic/source/">View source</a><a href="/packages/search/?q=%2BData.Time.Arithmetic">Search module</a><a href="/reference-sources/Data.Time.Arithmetic.cinch" download="Data.Time.Arithmetic.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Span](Data.Time.Span.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Unit](Data.Time.Unit.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis"><summary>Synopsis (33 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Difference">Difference</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Difference</strong></code></li><li><a class="syn-name" href="#t-Round">Round</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Round</strong></code></li><li><a class="syn-name" href="#v-date-add-span">date-add-span</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">date-add-span</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-date-since">date-since</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">date-since</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span></code></li><li><a class="syn-name" href="#v-date-since-with">date-since-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">date-since-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code></li><li><a class="syn-name" href="#v-date-sub-span">date-sub-span</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">date-sub-span</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-date-until">date-until</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">date-until</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span></code></li><li><a class="syn-name" href="#v-date-until-with">date-until-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">date-until-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code></li><li><a class="syn-name" href="#v-datetime-add-span">datetime-add-span</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">datetime-add-span</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-datetime-round">datetime-round</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">datetime-round</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Round" title="Data.Time.Arithmetic.Round">Round</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-datetime-since">datetime-since</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">datetime-since</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span></code></li><li><a class="syn-name" href="#v-datetime-since-with">datetime-since-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">datetime-since-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code></li><li><a class="syn-name" href="#v-datetime-sub-span">datetime-sub-span</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">datetime-sub-span</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-datetime-until">datetime-until</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">datetime-until</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span></code></li><li><a class="syn-name" href="#v-datetime-until-with">datetime-until-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">datetime-until-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code></li><li><a class="syn-name" href="#v-difference-default">difference-default</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">difference-default</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a></code></li><li><a class="syn-name" href="#v-difference-largest">difference-largest</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">difference-largest</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a></code></li><li><a class="syn-name" href="#v-round-default">round-default</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">round-default</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Round" title="Data.Time.Arithmetic.Round">Round</a></code></li><li><a class="syn-name" href="#v-round-to">round-to</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">round-to</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Round" title="Data.Time.Arithmetic.Round">Round</a></code></li><li><a class="syn-name" href="#v-time-add-span-wrapping">time-add-span-wrapping</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">time-add-span-wrapping</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code></li><li><a class="syn-name" href="#v-time-round">time-round</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">time-round</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Round" title="Data.Time.Arithmetic.Round">Round</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code></li><li><a class="syn-name" href="#v-time-since">time-since</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">time-since</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span></code></li><li><a class="syn-name" href="#v-time-since-with">time-since-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">time-since-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span>)</code></li><li><a class="syn-name" href="#v-time-sub-span-wrapping">time-sub-span-wrapping</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">time-sub-span-wrapping</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code></li><li><a class="syn-name" href="#v-time-until">time-until</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">time-until</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span></code></li><li><a class="syn-name" href="#v-time-until-with">time-until-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">time-until-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span>)</code></li><li><a class="syn-name" href="#v-timestamp-add-span">timestamp-add-span</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-add-span</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-timestamp-round">timestamp-round</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-round</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Round" title="Data.Time.Arithmetic.Round">Round</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-timestamp-since-span">timestamp-since-span</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-since-span</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span></code></li><li><a class="syn-name" href="#v-timestamp-since-span-with">timestamp-since-span-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-since-span-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span>)</code></li><li><a class="syn-name" href="#v-timestamp-sub-span">timestamp-sub-span</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-sub-span</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-timestamp-until-span">timestamp-until-span</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-until-span</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span></code></li><li><a class="syn-name" href="#v-timestamp-until-span-with">timestamp-until-span-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-until-span-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span>)</code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Difference">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Difference</strong></code><span class="doc-sig-links"><a href="#t-Difference" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L198" title="View source at line 198">Source</a></span></div>
<div class="doc-body">

Largest unit (unset: the larger of the operand type's default and
the smallest unit), smallest unit (unset: the operand type's
default), increment, mode.

<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-LargestOption" title="Data.Time.Unit.LargestOption">LargestOption</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Arithmetic/#instances">Data.Time.Arithmetic</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-RoundingOptions" title="Data.Time.Unit.RoundingOptions">RoundingOptions</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Arithmetic/#instances">Data.Time.Arithmetic</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-Round">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Round</strong></code><span class="doc-sig-links"><a href="#t-Round" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L393" title="View source at line 393">Source</a></span></div>
<div class="doc-body">

Smallest unit, increment, mode.

<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-RoundingOptions" title="Data.Time.Unit.RoundingOptions">RoundingOptions</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Round" title="Data.Time.Arithmetic.Round">Round</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Arithmetic/#instances">Data.Time.Arithmetic</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-date-add-span">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">date-add-span</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-date-add-span" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L148" title="View source at line 148">Source</a></span></div>
<div class="doc-body">

The date `s` after `x` (header: largest unit first, the day
constrained, the clock units balanced into whole days). `None`
when the result leaves -9999-01-01..=9999-12-31.

</div>
</article>

<article class="doc-entry" id="v-date-since">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">date-since</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span></code><span class="doc-sig-links"><a href="#v-date-since" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L267" title="View source at line 267">Source</a></span></div>
<div class="doc-body">

The days since `b`, to `a`: the negation of `date-until a b`.

</div>
</article>

<article class="doc-entry" id="v-date-since-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">date-since-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code><span class="doc-sig-links"><a href="#v-date-since-with" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L288" title="View source at line 288">Source</a></span></div>
</article>

<article class="doc-entry" id="v-date-sub-span">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">date-sub-span</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-date-sub-span" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L151" title="View source at line 151">Source</a></span></div>
</article>

<article class="doc-entry" id="v-date-until">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">date-until</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span></code><span class="doc-sig-links"><a href="#v-date-until" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L261" title="View source at line 261">Source</a></span></div>
<div class="doc-body">

The days from `a` to `b`: 2024-02-01 until 2024-03-01 is 29 days.
Total: the whole range is 7304483 days, inside the day limit, so
the `None` arm is unreachable.

</div>
</article>

<article class="doc-entry" id="v-date-until-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">date-until-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code><span class="doc-sig-links"><a href="#v-date-until-with" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L285" title="View source at line 285">Source</a></span></div>
<div class="doc-body">

The span from `a` to `b` under the options. Units are `Day` to
`Year`; the default smallest unit is `Day`.
2024-05-31 until 2024-09-30 with largest `Month` is 3 months 30
days.
  Err SpanBadUnits      a clock unit, or largest below smallest
  Err SpanBadIncrement  the increment is below 1 or above
                        1000000000
  Err SpanOutOfRange    a rounding candidate leaves the date range

</div>
</article>

<article class="doc-entry" id="v-datetime-add-span">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">datetime-add-span</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-datetime-add-span" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L157" title="View source at line 157">Source</a></span></div>
<div class="doc-body">

The date-time `s` after `x`: 2024-02-01T17:30 plus 1 month 1 hour
30 minutes is 2024-03-01T19:00. `None` when the date leaves the
range.

</div>
</article>

<article class="doc-entry" id="v-datetime-round">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">datetime-round</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Round" title="Data.Time.Arithmetic.Round">Round</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-datetime-round" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L457" title="View source at line 457">Source</a></span></div>
<div class="doc-body">

Round a date-time (PlainDateTime.prototype.round): the time of day
is rounded and a carry moves the date, so
2024-11-02T17:46:20.123456789 to `Day` is 2024-11-03T00:00.
  Err SpanBadUnits      week or larger
  Err SpanBadIncrement  as for `time-round`; for `Day`, anything
                        but 1
  Err SpanOutOfRange    the carry leaves the date range

</div>
</article>

<article class="doc-entry" id="v-datetime-since">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">datetime-since</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span></code><span class="doc-sig-links"><a href="#v-datetime-since" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L305" title="View source at line 305">Source</a></span></div>
</article>

<article class="doc-entry" id="v-datetime-since-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">datetime-since-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code><span class="doc-sig-links"><a href="#v-datetime-since-with" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L323" title="View source at line 323">Source</a></span></div>
</article>

<article class="doc-entry" id="v-datetime-sub-span">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">datetime-sub-span</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-datetime-sub-span" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L160" title="View source at line 160">Source</a></span></div>
</article>

<article class="doc-entry" id="v-datetime-until">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">datetime-until</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span></code><span class="doc-sig-links"><a href="#v-datetime-until" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L300" title="View source at line 300">Source</a></span></div>
<div class="doc-body">

The span from `a` to `b` in days and clock units, exact:
2024-05-15T14:30:54 until 2024-05-31T19:13:27.123456789 is 16 days
4 hours 42 minutes 33 seconds 123 ms 456 us 789 ns. Total: at most
7304483 days and less than a day of clock units, all inside their
limits, and nothing is rounded, so the `Err` arm is unreachable.

</div>
</article>

<article class="doc-entry" id="v-datetime-until-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">datetime-until-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code><span class="doc-sig-links"><a href="#v-datetime-until-with" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L320" title="View source at line 320">Source</a></span></div>
<div class="doc-body">

The span from `a` to `b` under the options; every unit is allowed.
  Err SpanBadUnits      largest below smallest
  Err SpanBadIncrement  the increment is not valid for the
                        smallest unit
  Err SpanOutOfRange    a rounding candidate leaves the date range
  Err SpanUnitLimit     a unit of the result passes its limit

</div>
</article>

<article class="doc-entry" id="v-difference-default">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">difference-default</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a></code><span class="doc-sig-links"><a href="#v-difference-default" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L202" title="View source at line 202">Source</a></span></div>
<div class="doc-body">

Both units unset, increment 1, `Trunc`.

</div>
</article>

<article class="doc-entry" id="v-difference-largest">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">difference-largest</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a></code><span class="doc-sig-links"><a href="#v-difference-largest" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L206" title="View source at line 206">Source</a></span></div>
<div class="doc-body">

The common case: the default with a largest unit.

</div>
</article>

<article class="doc-entry" id="v-round-default">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">round-default</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Round" title="Data.Time.Arithmetic.Round">Round</a></code><span class="doc-sig-links"><a href="#v-round-default" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L397" title="View source at line 397">Source</a></span></div>
<div class="doc-body">

`Nanosecond`, 1, `HalfExpand`: rounds nothing until a unit is set.

</div>
</article>

<article class="doc-entry" id="v-round-to">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">round-to</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Round" title="Data.Time.Arithmetic.Round">Round</a></code><span class="doc-sig-links"><a href="#v-round-to" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L401" title="View source at line 401">Source</a></span></div>
<div class="doc-body">

The common case: the default with a smallest unit.

</div>
</article>

<article class="doc-entry" id="v-time-add-span-wrapping">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">time-add-span-wrapping</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code><span class="doc-sig-links"><a href="#v-time-add-span-wrapping" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L170" title="View source at line 170">Source</a></span></div>
<div class="doc-body">

The time of day `s` after `t`, wrapping around midnight. The
span's calendar units (days and larger) are ignored, as Temporal
specifies. Total.

The clock nanoseconds (below 1.3e21) are reduced modulo one day
first, so the count handed to lane B's wrapping addition is within
+-86399999999999 and the narrowing is exact.

</div>
</article>

<article class="doc-entry" id="v-time-round">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">time-round</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Round" title="Data.Time.Arithmetic.Round">Round</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code><span class="doc-sig-links"><a href="#v-time-round" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L445" title="View source at line 445">Source</a></span></div>
<div class="doc-body">

Round a time of day (PlainTime.prototype.round). Rounding up past
the end of the day wraps to midnight: 23:59:40 to the minute is
00:00.
  Err SpanBadUnits      a calendar unit
  Err SpanBadIncrement  the increment does not divide the next
                        larger unit, or equals it

</div>
</article>

<article class="doc-entry" id="v-time-since">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">time-since</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span></code><span class="doc-sig-links"><a href="#v-time-since" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L338" title="View source at line 338">Source</a></span></div>
</article>

<article class="doc-entry" id="v-time-since-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">time-since-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span>)</code><span class="doc-sig-links"><a href="#v-time-since-with" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L355" title="View source at line 355">Source</a></span></div>
</article>

<article class="doc-entry" id="v-time-sub-span-wrapping">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">time-sub-span-wrapping</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code><span class="doc-sig-links"><a href="#v-time-sub-span-wrapping" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L174" title="View source at line 174">Source</a></span></div>
</article>

<article class="doc-entry" id="v-time-until">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">time-until</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span></code><span class="doc-sig-links"><a href="#v-time-until" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L333" title="View source at line 333">Source</a></span></div>
<div class="doc-body">

The span from `a` to `b` within one day, in hours and smaller;
negative when `b` is the earlier time (no wrapping). Total: less
than 24 hours, so the `Err` arm is unreachable.

</div>
</article>

<article class="doc-entry" id="v-time-until-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">time-until-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span>)</code><span class="doc-sig-links"><a href="#v-time-until-with" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L352" title="View source at line 352">Source</a></span></div>
<div class="doc-body">

Units are `Nanosecond` to `Hour`; the default largest unit is
`Hour`.
  Err SpanBadUnits      a calendar unit, or largest below smallest
  Err SpanBadIncrement  the increment is not valid for the
                        smallest unit

</div>
</article>

<article class="doc-entry" id="v-timestamp-add-span">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-add-span</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-timestamp-add-span" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L181" title="View source at line 181">Source</a></span></div>
<div class="doc-body">

The instant `s` after `t`. Only clock units are allowed
(Instant.prototype.add).
  Err SpanNeedsAnchor  a day or larger unit of `s` is non-zero
  Err SpanOutOfRange   the result leaves the timestamp range

</div>
</article>

<article class="doc-entry" id="v-timestamp-round">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-round</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Round" title="Data.Time.Arithmetic.Round">Round</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-timestamp-round" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L477" title="View source at line 477">Source</a></span></div>
<div class="doc-body">

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

</div>
</article>

<article class="doc-entry" id="v-timestamp-since-span">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-since-span</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span></code><span class="doc-sig-links"><a href="#v-timestamp-since-span" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L371" title="View source at line 371">Source</a></span></div>
</article>

<article class="doc-entry" id="v-timestamp-since-span-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-since-span-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span>)</code><span class="doc-sig-links"><a href="#v-timestamp-since-span-with" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L385" title="View source at line 385">Source</a></span></div>
</article>

<article class="doc-entry" id="v-timestamp-sub-span">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-sub-span</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanClass" title="Data.Time.Span.SpanClass">SpanClass</a>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-timestamp-sub-span" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L188" title="View source at line 188">Source</a></span></div>
</article>

<article class="doc-entry" id="v-timestamp-until-span">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-until-span</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span></code><span class="doc-sig-links"><a href="#v-timestamp-until-span" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L366" title="View source at line 366">Source</a></span></div>
<div class="doc-body">

The span from `a` to `b` in seconds and smaller (Temporal's default
for instants). Total: the whole timestamp range is fewer than
631107417600 seconds, the second limit, so the `Err` arm is
unreachable.

</div>
</article>

<article class="doc-entry" id="v-timestamp-until-span-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-until-span-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanError" title="Data.Time.Span.SpanError">SpanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span>)</code><span class="doc-sig-links"><a href="#v-timestamp-until-span-with" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Arithmetic/source/#L382" title="View source at line 382">Source</a></span></div>
<div class="doc-body">

Units are `Nanosecond` to `Hour`; the default largest unit is
`Second`.
  Err SpanBadUnits      a calendar unit, or largest below smallest
  Err SpanBadIncrement  the increment is not valid for the
                        smallest unit
  Err SpanUnitLimit     a unit of the result passes its limit
                        (the whole range in nanoseconds does not
                        fit the nanosecond unit)

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-LargestOption_Difference"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-LargestOption" title="Data.Time.Unit.LargestOption">LargestOption</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Arithmetic.LargestOption_Difference</code></span></li>
<li id="i-RoundingOptions_Difference"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-RoundingOptions" title="Data.Time.Unit.RoundingOptions">RoundingOptions</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Arithmetic.RoundingOptions_Difference</code></span></li>
<li id="i-RoundingOptions_Round"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-RoundingOptions" title="Data.Time.Unit.RoundingOptions">RoundingOptions</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Round" title="Data.Time.Arithmetic.Round">Round</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Arithmetic.RoundingOptions_Round</code></span></li>
</ul>
