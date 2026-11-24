# Data.Time.Zoned.Arithmetic

Arithmetic, differences and rounding on zoned date-times.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Time.Zoned.Arithmetic`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Arithmetic](Data.Time.Arithmetic.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.Span](Data.Time.Span.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Unit](Data.Time.Unit.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zoned](Data.Time.Zoned.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Time.Arithmetic](Data.Time.Arithmetic.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.Span](Data.Time.Span.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Unit](Data.Time.Unit.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zone.Posix](Data.Time.Zone.Posix.md), [Data.Time.Zone.Types](Data.Time.Zone.Types.md), [Data.Time.Zoned](Data.Time.Zoned.md), [Data.Time.Zoned.Arithmetic](Data.Time.Zoned.Arithmetic.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Time.Zoned.Arithmetic: arithmetic, differences and rounding on
an instant in a place (design/datetime.md, layer L3 on L2, lane F).

The semantics are TC39 Temporal's ZonedDateTime (add, subtract,
until, since, round, startOfDay, hoursInDay, with) and its
Duration operations with a zoned relativeTo, as rendered by Jiff's
`Zoned`. Ported, not redesigned (section 6 of the design note).

Design law shape.
  - ADD (AddZonedDateTime). A span with no calendar unit (years,
    months, weeks and days all zero) is added on the instant, so
    `zoned-add-span (span-of-hours 1)` is exactly one elapsed hour
    whatever the clocks did (01:30 -05:00 on New York's spring
    transition day plus one hour is 03:30 -04:00). Otherwise the
    years, months, weeks and days are added to the CIVIL date with
    the day constrained to the target month (2024-05-31 + 1 month is
    2024-06-30), the time of day is kept, that civil reading is
    resolved in the zone by the options' `Disambiguation` (T4;
    `Compatible` by default), and the clock units are then added on
    the instant. So `+ 1 day` and `+ 24 hours` differ on a
    transition day, and `zoned-sub-span s` is `zoned-add-span
    (span-negate s)`.
  - DIFFERENCE (DifferenceZonedDateTime). With the largest unit
    `Hour` or smaller (the default is `Hour`) the difference is the
    exact elapsed time between the instants, and the two values may
    be in different zones. With `Day` or larger both values must be
    in the same zone (by name) and the calendar part is counted on
    the civil readings in the first value's zone: the largest whole
    count of each unit for which `a` plus that count, read at `a`'s
    time of day and resolved `Compatible`, does not pass `b`, and
    the remainder is elapsed time. The remainder is spelled in hours
    and smaller and is NOT folded into days, so it can reach 24
    hours and more on a 25-hour day (Temporal spells a zoned
    difference with TemporalDurationFromInternal(.., hour)).
    `zoned-since a b` is the negation of `zoned-until a b`, and
    with no rounding `zoned-add-span (zoned-until a b) a` is `b`
    for the largest units `Hour` and smaller always, and for `Day`
    unless `a` is the second occurrence of a repeated civil reading
    (next point).
  - A WART THAT IS PORTED. The intermediate reading of a calendar
    difference is "`a`'s time of day on a date at or next to
    `b`'s", resolved `Compatible`. When `a` is the SECOND 01:30 of
    a fold and the calendar part is zero days, that intermediate is
    the FIRST 01:30, an hour before `a`, so the remainder is an hour
    longer than the time that elapsed (the second 01:30 on
    2024-11-03 in New York until 00:30 on the 4th is 24 hours with
    largest `Day` and 23 by default). This is what Temporal's
    algorithm answers and it is kept; the default (hours) form is
    always the elapsed time. Two readings on ONE civil date never
    go through an intermediate: their difference is the elapsed
    time.
  - A DAY is the time between two consecutive civil starts of day in
    the zone: 23 hours on 2024-03-10 in New York, 25 on 2024-11-03.
    Rounding a difference or a span uses the same idea from the
    anchor's time of day: the "day" after the calendar part runs
    from where that part lands to the same civil time one civil day
    on. A unit below a day is rounded and compared against that
    length (NudgeToZonedTime), and rounding to days measures
    progress through it (NudgeToCalendarUnit): 12 hours 15 minutes
    from 17:30 on 2024-11-02 rounds DOWN to 0 days, because 17:30 to
    17:30 the next day is 25 hours.
  - ROUND a value (ZonedDateTime.prototype.round). To `Day`: within
    [start of the civil day, start of the next civil day] by the
    real length of that day. To a smaller unit: the civil reading is
    rounded (as `datetime-round`) and resolved again preferring the
    offset the value already has (below).
  - PREFER THE CURRENT OFFSET (InterpretISODateTimeOffset with the
    offset option "prefer"). When a civil reading is re-resolved for
    a value that already has an offset (rounding, and the
    `zoned-with-` family), the value's own offset is kept if the new
    reading can have it; otherwise the reading is resolved by the
    strategy (`Compatible` unless the `-resolving` form names
    another). So 01:30 -05:00 in the autumn fold stays at -05:00
    when its minutes are changed.
  - SPANS RELATIVE TO A ZONED VALUE (`span-round-zoned` and its
    family) are one construction: add the span to the anchor (ADD,
    `Compatible`), then take the DIFFERENCE from the anchor to that
    instant in the units asked for. 2756 hours from 2020-01-01 in
    Europe/Amsterdam is 3 months 23 days 21 hours (one of those days
    was 23 hours long; the civil anchor of `Data.Time.Span` gives 20
    hours).
  - The civil conveniences (`zoned-tomorrow`, `zoned-first-of-month`,
    `zoned-nth-weekday`, ..) are lane B's operation on the civil
    date, the same time of day, resolved `Compatible`.

Where the engine lives. Lane D's header names a seam (a third
`Relative` constructor inside Data.Time.Span). It is not used: that
would make Data.Time.Span import the zone modules, and lane D's
test harness would have to ship them. The zoned engine is here
instead, built on Data.Time.Span's exports (`span-difference-civil`
for the calendar count between two dates, `span-difference-nanos`
and `span-from-units` for the spelling) and on the rounding kernel
of Data.Time.Unit. The one addition to Data.Time.Span is four
readers of the abstract `SpanRound` (`span-round-largest`,
`-smallest`, `-increment`, `-mode`), so that `span-round-zoned`
takes the same options value as `span-round`.

Overflow. Instants are within +-3.8e20 nanoseconds and a span's
clock units total below 1.3e21 (Data.Time.Span's header), so every
sum and difference of them formed here is below 2^72 and the
`wrapping-` `I128` operations on them are exact. Products with a
rounding increment (at most 1e9) and a unit count (at most about
1e9) stay below 2^120; each such product says so where it is
formed. `I64` unit counts are span fields (within their limits)
plus or minus one increment, so their `wrapping-` sums are exact.

No operation faults (T2): `Result ZonedError` wherever a refusal is
possible, `Option` where the only refusal is the range.

NAMED REFUSALS:
  - a silent disambiguation (T4): every operation that resolves a
    civil reading either takes the strategy (`ZonedArithmetic`, the
    `-resolving` forms) or says in this header that it is
    `Compatible` (the conveniences, and the intermediate readings of
    a difference or a rounding, where Temporal fixes it).
  - a calendar difference across zones: `ZonedZonesDiffer`. Which
    zone's days would be counted is not something to pick silently.
    Move one value with `zoned-with-time-zone` first.
  - folding the remainder of a calendar difference into days: not
    done (see DIFFERENCE); a remainder of 24 hours 59 minutes on a
    25-hour day is spelled so.
  - `Day` rounding of a value with an increment other than 1, and
    week, month or year as a value's rounding unit: refused, as in
    Temporal.
  - `zoned-hours-in-day` as a number: a day's length is not always a
    whole number of hours (Lord Howe's half-hour shift), and there
    is no float in this family. `zoned-day-length` is the exact
    `Duration`.
  - operator instances (T2).
  - rounding at the very end of the range: every rounding of a
    difference or a span reads the further candidate (the far end of
    the unit's window, or the civil day after the calendar part) to
    measure against, and answers `ZonedOutOfRange` when that lies
    outside the range even if the mode would not have chosen it.
  - the 0.5.1 reference polyfill as the last word: three steps of
    the current specification text (read 2026-11-17) are later than
    it and are followed here, so three kinds of answer differ from
    that polyfill: two readings on one civil date differ by elapsed
    time (DifferenceZonedDateTime step 4); a calendar rounding
    window that does not hold the destination is moved one increment
    on (ComputeNudgeWindow's additionalShift); and the window's
    start can be the anchor's own instant (next point).
  - ONE DELIBERATE DEPARTURE from the specification's letter.
    ComputeNudgeWindow says "If r1 = 0, let startEpochNs be
    originEpochNs". Here the window starts at the anchor's own
    instant only when the whole start candidate is zero (no larger
    units before it either). For years the two are the same. For a
    smaller unit after non-zero larger ones (1 month 5 hours rounded
    to days: r1 is 0 days) the letter would measure the day's
    progress from the anchor, a month back, and round 5 hours up to
    a day; that cannot be meant, so the candidate "anchor plus one
    month" is resolved in the zone as every other candidate is. What
    the rule is for is kept: an anchor that is the second occurrence
    of a repeated reading is not resolved again to the first.

</details>

# Types and constructors

<a id="type-5a6f6e656441726974686d65746963"></a>

## `ZonedArithmetic`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ZonedArithmetic</span></code></pre>

The options of zoned addition (T7): how a civil reading that the
calendar units land on is resolved when it does not exist or
occurs twice.

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 404

<a id="type-5a6f6e6564446966666572656e6365"></a>

## `ZonedDifference`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ZonedDifference</span></code></pre>

The options of a zoned difference (T7): largest unit (unset: the
larger of `Hour` and the smallest unit), smallest unit (unset:
`Nanosecond`), increment, mode.

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 839

<a id="type-5a6f6e65644572726f72"></a>

## `ZonedError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ZonedError</span></code></pre>

The refusals of this module.

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 206

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZonedOutOfRange</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZonedRejectedGap</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZonedRejectedFold</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZonedZonesDiffer</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZonedBadUnits</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZonedBadIncrement</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ZonedUnitLimit</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span></code></pre>

<a id="type-5a6f6e6564526f756e64"></a>

## `ZonedRound`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ZonedRound</span></code></pre>

The options of `zoned-round-with` (T7): smallest unit, increment,
mode.

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 970

# Values

<a id="value-7370616e2d62616c616e63652d7a6f6e6564"></a>

## `span-balance-zoned`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-balance-zoned</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))))</code></pre>

Respell any span with units up to `largest`, measured from the
anchor (T8: not rounding). 2756 hours from 2020-01-01T00:00 in
Europe/Amsterdam to `Year` is 3 months 23 days 21 hours; 1 day
from 2024-11-03T00:00 in New York to `Hour` is 25 hours.

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 1059

<a id="value-7370616e2d636865636b65642d6164642d7a6f6e6564"></a>

## `span-checked-add-zoned`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-checked-add-zoned</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">b</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))))</code></pre>

The sum of any two spans measured from the anchor (Jiff's
`Span::checked_add` with a zoned relative): the anchor moves by
`a`, then by `b`, and the result is the difference from the anchor
to that instant, spelled up to the larger of the two spans'
largest units.
  Err ZonedOutOfRange  a step leaves the range
  Err ZonedUnitLimit   a unit of the result passes its limit

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 1137

<a id="value-7370616e2d636865636b65642d7375622d7a6f6e6564"></a>

## `span-checked-sub-zoned`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-checked-sub-zoned</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">b</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch)

<a id="value-7370616e2d636f6d706172652d7a6f6e6564"></a>

## `span-compare-zoned`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-compare-zoned</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">b</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>)))</code></pre>

Any two spans by where they land from the anchor (Temporal's
Duration.compare with a zoned relativeTo): 1 day and 24 hours are
`EQ` from most instants and differ from 2024-11-03T00:00 in New
York. Spans with no calendar units compare by elapsed time.
  Err ZonedOutOfRange  the anchor plus a span leaves the range

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 1117

<a id="value-7370616e2d726f756e642d7a6f6e6564"></a>

## `span-round-zoned`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-round-zoned</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanRound</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))))</code></pre>

Round any span, measured from the anchor (Temporal's
Duration.prototype.round with a zoned relativeTo), with the same
options value as `span-round`.
  Err ZonedBadUnits, ZonedBadIncrement  as `span-round`
  Err ZonedOutOfRange  the anchor plus the span, an intermediate
                       reading or a rounding candidate leaves the
                       range
  Err ZonedUnitLimit   a unit of the result passes its limit

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 1048

<a id="value-7370616e2d746f74616c2d7a6f6e6564"></a>

## `span-total-zoned`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-total-zoned</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> (<span class="tok-constructor">I128</span>, <span class="tok-constructor">I128</span>))))</code></pre>

The exact length of any span in `unit`, measured from the anchor,
as a reduced fraction with a positive denominator (Temporal's
Duration.prototype.total with a zoned relativeTo). `Hour` and
smaller: the elapsed time over the unit's length. `Day` and
larger: the whole units plus the fraction of the next one, the
next one measured in the zone (1 hour from 2024-11-03T00:00 in
New York is 1/25 of a day).
  Err ZonedOutOfRange  the anchor plus the span, or the end of
                       the unit being measured, leaves the range

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 1089

<a id="value-776974682d646973616d626967756174696f6e"></a>

## `with-disambiguation`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">with-disambiguation</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Disambiguation</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedArithmetic</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedArithmetic</span>))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch)

<a id="value-7a6f6e65642d6164642d6475726174696f6e"></a>

## `zoned-add-duration`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-add-duration</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))</code></pre>

Exact elapsed time, on the instant. `None` when the instant leaves
the range.

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 442

<a id="value-7a6f6e65642d6164642d7370616e"></a>

## `zoned-add-span`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-add-span</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))</code></pre>

The same, `Compatible`: 2024-05-31T13:45 in New York plus one
month is 2024-06-30T13:45 there.

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 434

<a id="value-7a6f6e65642d6164642d7370616e2d77697468"></a>

## `zoned-add-span-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-add-span-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedArithmetic</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))</code></pre>

Add a span (the header's ADD law) under the options.
  Err ZonedOutOfRange    the date or the instant leaves the range
  Err ZonedRejectedGap,
      ZonedRejectedFold  the options say `Reject` and the calendar
                         units land on such a reading

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 423

<a id="value-7a6f6e65642d61726974686d657469632d64656661756c74"></a>

## `zoned-arithmetic-default`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-arithmetic-default</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedArithmetic</span></code></pre>

`Compatible`.

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 408

<a id="value-7a6f6e65642d61726974686d657469632d646973616d626967756174696f6e"></a>

## `zoned-arithmetic-disambiguation`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-arithmetic-disambiguation</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedArithmetic</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Disambiguation</span>)</code></pre>

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch)

<a id="value-7a6f6e65642d6461792d6c656e677468"></a>

## `zoned-day-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-day-length</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>))</code></pre>

The length of `x`'s civil day in its zone (Temporal's hoursInDay,
exact): from the start of the day to the start of the next.
23 hours on 2024-03-10 in New York, 25 on 2024-11-03. `None` at
the two ends of the range.

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 490

<a id="value-7a6f6e65642d646966666572656e63652d64656661756c74"></a>

## `zoned-difference-default`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-difference-default</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedDifference</span></code></pre>

Both units unset, increment 1, `Trunc`.

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 843

<a id="value-7a6f6e65642d646966666572656e63652d6c617267657374"></a>

## `zoned-difference-largest`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-difference-largest</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedDifference</span>)</code></pre>

The common case: the default with a largest unit.

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 847

<a id="value-7a6f6e65642d66697273742d6f662d6d6f6e7468"></a>

## `zoned-first-of-month`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-first-of-month</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))</code></pre>

Lane B's civil operation on the date, the time of day kept,
resolved `Compatible`.

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 518

<a id="value-7a6f6e65642d66697273742d6f662d79656172"></a>

## `zoned-first-of-year`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-first-of-year</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch)

<a id="value-7a6f6e65642d6c6173742d6f662d6d6f6e7468"></a>

## `zoned-last-of-month`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-last-of-month</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch)

<a id="value-7a6f6e65642d6c6173742d6f662d79656172"></a>

## `zoned-last-of-year`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-last-of-year</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch)

<a id="value-7a6f6e65642d6e74682d7765656b646179"></a>

## `zoned-nth-weekday`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-nth-weekday</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))))</code></pre>

`Err ZonedOutOfRange` also when the civil operation refuses (`n`
of zero, a weekday the month does not have that many of).

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 532

<a id="value-7a6f6e65642d6e74682d7765656b6461792d6f662d6d6f6e7468"></a>

## `zoned-nth-weekday-of-month`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-nth-weekday-of-month</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch)

<a id="value-7a6f6e65642d726f756e64"></a>

## `zoned-round`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-round</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))</code></pre>

Round to a unit, increment 1, `HalfExpand`.

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 1029

<a id="value-7a6f6e65642d726f756e642d64656661756c74"></a>

## `zoned-round-default`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-round-default</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedRound</span></code></pre>

`Nanosecond`, 1, `HalfExpand`: rounds nothing until a unit is set.

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 974

<a id="value-7a6f6e65642d726f756e642d746f"></a>

## `zoned-round-to`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-round-to</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedRound</span>)</code></pre>

The common case: the default with a smallest unit.

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 978

<a id="value-7a6f6e65642d726f756e642d77697468"></a>

## `zoned-round-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-round-with</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedRound</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))</code></pre>

Round a value (the header's ROUND law): 17:46:20.123456789 on
2024-11-02 in New York to 30 minutes is 18:00 -04:00; 11:45:20 on
2024-11-03 to `Day` is 2024-11-04T00:00 -05:00, because 11:45 is
past the middle of a 25-hour day.
  Err ZonedBadUnits      week or larger
  Err ZonedBadIncrement  the increment does not divide the next
                         larger unit, or equals it; for `Day`,
                         anything but 1
  Err ZonedOutOfRange    the result leaves the range

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 1017

<a id="value-7a6f6e65642d73696e6365"></a>

## `zoned-since`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-since</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>))</code></pre>

The elapsed time since `b`, to `a`: the negation of
`zoned-until a b`.

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 896

<a id="value-7a6f6e65642d73696e63652d77697468"></a>

## `zoned-since-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-since-with</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedDifference</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch)

<a id="value-7a6f6e65642d73746172742d6f662d6e6578742d646179"></a>

## `zoned-start-of-next-day`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-start-of-next-day</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))</code></pre>

The first instant of the civil day after `x`'s. `None` at the end
of the range. (When a zone skipped a whole civil day, as
Pacific/Apia skipped 2011-12-30, this is the start of the day
after the skipped one, as in Temporal.)

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 479

<a id="value-7a6f6e65642d7375622d6475726174696f6e"></a>

## `zoned-sub-duration`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-sub-duration</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch)

<a id="value-7a6f6e65642d7375622d7370616e"></a>

## `zoned-sub-span`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-sub-span</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch)

<a id="value-7a6f6e65642d7375622d7370616e2d77697468"></a>

## `zoned-sub-span-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-sub-span-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedArithmetic</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch)

<a id="value-7a6f6e65642d746f6d6f72726f77"></a>

## `zoned-tomorrow`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-tomorrow</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))</code></pre>

The same civil time on the next civil day, `Compatible`: tomorrow
of 2024-11-02T17:25 in New York is 2024-11-03T17:25, 25 hours
later; of 2024-03-09T02:30 it is 2024-03-10T03:30 (02:30 does not
exist). For another strategy add `span-of-days 1` with options.
  Err ZonedOutOfRange  the end of the range

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 510

<a id="value-7a6f6e65642d756e74696c"></a>

## `zoned-until`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-until</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>))</code></pre>

The elapsed time from `a` to `b` in hours and smaller, exact; the
zones may differ. 2024-11-02T17:25 until 2024-11-03T17:25 in New
York is 25 hours. Total: the whole timestamp range is inside the
hour limit, so the `Err` arm is unreachable.

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 888

<a id="value-7a6f6e65642d756e74696c2d77697468"></a>

## `zoned-until-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-until-with</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedDifference</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))))</code></pre>

The span from `a` to `b` under the options (the header's
DIFFERENCE law): 2024-05-31T00:00 until 2024-09-30T00:00 in New
York with largest `Month` is 3 months 30 days.
  Err ZonedBadUnits      largest below smallest
  Err ZonedBadIncrement  the increment is not valid for the
                         smallest unit
  Err ZonedZonesDiffer   largest is `Day` or larger and the zones'
                         names differ
  Err ZonedOutOfRange    an intermediate reading or a rounding
                         candidate leaves the range
  Err ZonedUnitLimit     a unit of the result passes its limit

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 910

<a id="value-7a6f6e65642d776974682d64617465"></a>

## `zoned-with-date`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-with-date</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch)

<a id="value-7a6f6e65642d776974682d646174652d7265736f6c76696e67"></a>

## `zoned-with-date-resolving`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-with-date-resolving</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Disambiguation</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch)

<a id="value-7a6f6e65642d776974682d6461746574696d65"></a>

## `zoned-with-datetime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-with-datetime</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))</code></pre>

The same, `Compatible`.

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 955

<a id="value-7a6f6e65642d776974682d6461746574696d652d7265736f6c76696e67"></a>

## `zoned-with-datetime-resolving`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-with-datetime-resolving</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Disambiguation</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))))</code></pre>

The value with its civil date, time of day or both replaced
(ZonedDateTime.prototype.with), keeping the value's offset when
the new reading can have it and otherwise resolving by the
strategy.
  Err ZonedRejectedGap, ZonedRejectedFold  the strategy is
                       `Reject` and the offset could not be kept
  Err ZonedOutOfRange  the instant leaves the range

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch) · line 943

<a id="value-7a6f6e65642d776974682d74696d65"></a>

## `zoned-with-time`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-with-time</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch)

<a id="value-7a6f6e65642d776974682d74696d652d7265736f6c76696e67"></a>

## `zoned-with-time-resolving`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-with-time-resolving</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Disambiguation</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch)

<a id="value-7a6f6e65642d796573746572646179"></a>

## `zoned-yesterday`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-yesterday</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">ZonedError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.Arithmetic.cinch)

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Time.Zoned.Arithmetic.LargestOption_ZonedDifference` | `Data.Time.Unit.LargestOption Data.Time.Zoned.Arithmetic.ZonedDifference` | Implicit |
| `Data.Time.Zoned.Arithmetic.RoundingOptions_ZonedDifference` | `Data.Time.Unit.RoundingOptions Data.Time.Zoned.Arithmetic.ZonedDifference` | Implicit |
| `Data.Time.Zoned.Arithmetic.RoundingOptions_ZonedRound` | `Data.Time.Unit.RoundingOptions Data.Time.Zoned.Arithmetic.ZonedRound` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Time/Zoned/Arithmetic.cinch`. Source SHA-256: `a1b34d6e327fedb1d1a522b9d956f445c24ad1ee422a7b5b9c77f41c1930af5f`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
