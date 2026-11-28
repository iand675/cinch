# Data.Time.Zoned.Arithmetic

Arithmetic, differences and rounding on zoned date-times.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Time.Zoned.Arithmetic</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/">View source</a><a href="/packages/search/?q=%2BData.Time.Zoned.Arithmetic">Search module</a><a href="/reference-sources/Data.Time.Zoned.Arithmetic.cinch" download="Data.Time.Zoned.Arithmetic.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Arithmetic](Data.Time.Arithmetic.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.Span](Data.Time.Span.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Unit](Data.Time.Unit.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zoned](Data.Time.Zoned.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis"><summary>Synopsis (45 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ZonedArithmetic">ZonedArithmetic</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZonedArithmetic</strong></code></li><li><a class="syn-name" href="#t-ZonedDifference">ZonedDifference</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZonedDifference</strong></code></li><li><a class="syn-name" href="#t-ZonedError">ZonedError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZonedError</strong></code></li><li><a class="syn-name" href="#t-ZonedRound">ZonedRound</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZonedRound</strong></code></li><li><a class="syn-name" href="#v-span-balance-zoned">span-balance-zoned</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-balance-zoned</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">c</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code></li><li><a class="syn-name" href="#v-span-checked-add-zoned">span-checked-add-zoned</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-checked-add-zoned</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code></li><li><a class="syn-name" href="#v-span-checked-sub-zoned">span-checked-sub-zoned</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-checked-sub-zoned</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code></li><li><a class="syn-name" href="#v-span-compare-zoned">span-compare-zoned</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-compare-zoned</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-span-round-zoned">span-round-zoned</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-round-zoned</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">c</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanRound" title="Data.Time.Span.SpanRound">SpanRound</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code></li><li><a class="syn-name" href="#v-span-total-zoned">span-total-zoned</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-total-zoned</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">c</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a>)</code></li><li><a class="syn-name" href="#v-with-disambiguation">with-disambiguation</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">with-disambiguation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedArithmetic" title="Data.Time.Zoned.Arithmetic.ZonedArithmetic">ZonedArithmetic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedArithmetic" title="Data.Time.Zoned.Arithmetic.ZonedArithmetic">ZonedArithmetic</a></code></li><li><a class="syn-name" href="#v-zoned-add-duration">zoned-add-duration</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-add-duration</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-add-span">zoned-add-span</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-add-span</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">c</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-add-span-with">zoned-add-span-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-add-span-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">c</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedArithmetic" title="Data.Time.Zoned.Arithmetic.ZonedArithmetic">ZonedArithmetic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-arithmetic-default">zoned-arithmetic-default</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-arithmetic-default</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedArithmetic" title="Data.Time.Zoned.Arithmetic.ZonedArithmetic">ZonedArithmetic</a></code></li><li><a class="syn-name" href="#v-zoned-arithmetic-disambiguation">zoned-arithmetic-disambiguation</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-arithmetic-disambiguation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedArithmetic" title="Data.Time.Zoned.Arithmetic.ZonedArithmetic">ZonedArithmetic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a></code></li><li><a class="syn-name" href="#v-zoned-day-length">zoned-day-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-day-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-zoned-difference-default">zoned-difference-default</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-difference-default</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedDifference" title="Data.Time.Zoned.Arithmetic.ZonedDifference">ZonedDifference</a></code></li><li><a class="syn-name" href="#v-zoned-difference-largest">zoned-difference-largest</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-difference-largest</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedDifference" title="Data.Time.Zoned.Arithmetic.ZonedDifference">ZonedDifference</a></code></li><li><a class="syn-name" href="#v-zoned-first-of-month">zoned-first-of-month</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-first-of-month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-first-of-year">zoned-first-of-year</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-first-of-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-last-of-month">zoned-last-of-month</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-last-of-month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-last-of-year">zoned-last-of-year</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-last-of-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-nth-weekday">zoned-nth-weekday</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-nth-weekday</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-nth-weekday-of-month">zoned-nth-weekday-of-month</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-nth-weekday-of-month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-round">zoned-round</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-round</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-round-default">zoned-round-default</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-round-default</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedRound" title="Data.Time.Zoned.Arithmetic.ZonedRound">ZonedRound</a></code></li><li><a class="syn-name" href="#v-zoned-round-to">zoned-round-to</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-round-to</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedRound" title="Data.Time.Zoned.Arithmetic.ZonedRound">ZonedRound</a></code></li><li><a class="syn-name" href="#v-zoned-round-with">zoned-round-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-round-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedRound" title="Data.Time.Zoned.Arithmetic.ZonedRound">ZonedRound</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-since">zoned-since</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-since</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span></code></li><li><a class="syn-name" href="#v-zoned-since-with">zoned-since-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-since-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedDifference" title="Data.Time.Zoned.Arithmetic.ZonedDifference">ZonedDifference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code></li><li><a class="syn-name" href="#v-zoned-start-of-next-day">zoned-start-of-next-day</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-start-of-next-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-sub-duration">zoned-sub-duration</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-sub-duration</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-sub-span">zoned-sub-span</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-sub-span</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">c</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-sub-span-with">zoned-sub-span-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-sub-span-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">c</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedArithmetic" title="Data.Time.Zoned.Arithmetic.ZonedArithmetic">ZonedArithmetic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-tomorrow">zoned-tomorrow</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-tomorrow</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-until">zoned-until</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-until</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span></code></li><li><a class="syn-name" href="#v-zoned-until-with">zoned-until-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-until-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedDifference" title="Data.Time.Zoned.Arithmetic.ZonedDifference">ZonedDifference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code></li><li><a class="syn-name" href="#v-zoned-with-date">zoned-with-date</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-with-date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-with-date-resolving">zoned-with-date-resolving</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-with-date-resolving</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-with-datetime">zoned-with-datetime</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-with-datetime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-with-datetime-resolving">zoned-with-datetime-resolving</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-with-datetime-resolving</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-with-time">zoned-with-time</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-with-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-with-time-resolving">zoned-with-time-resolving</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-with-time-resolving</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-yesterday">zoned-yesterday</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-yesterday</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ZonedArithmetic">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZonedArithmetic</strong></code><span class="doc-sig-links"><a href="#t-ZonedArithmetic" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L404" title="View source at line 404">Source</a></span></div>
<div class="doc-body">

The options of zoned addition (T7): how a civil reading that the
calendar units land on is resolved when it does not exist or
occurs twice.

</div>
</article>

<article class="doc-entry" id="t-ZonedDifference">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZonedDifference</strong></code><span class="doc-sig-links"><a href="#t-ZonedDifference" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L839" title="View source at line 839">Source</a></span></div>
<div class="doc-body">

The options of a zoned difference (T7): largest unit (unset: the
larger of `Hour` and the smallest unit), smallest unit (unset:
`Nanosecond`), increment, mode.

<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-LargestOption" title="Data.Time.Unit.LargestOption">LargestOption</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedDifference" title="Data.Time.Zoned.Arithmetic.ZonedDifference">ZonedDifference</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#instances">Data.Time.Zoned.Arithmetic</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-RoundingOptions" title="Data.Time.Unit.RoundingOptions">RoundingOptions</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedDifference" title="Data.Time.Zoned.Arithmetic.ZonedDifference">ZonedDifference</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#instances">Data.Time.Zoned.Arithmetic</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-ZonedError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZonedError</strong></code><span class="doc-sig-links"><a href="#t-ZonedError" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L206" title="View source at line 206">Source</a></span></div>
<div class="doc-body">

The refusals of this module.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ZonedOutOfRange"><code><strong class="tok-constructor">ZonedOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a></code></li><li id="k-ZonedRejectedGap"><code><strong class="tok-constructor">ZonedRejectedGap</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a></code></li><li id="k-ZonedRejectedFold"><code><strong class="tok-constructor">ZonedRejectedFold</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a></code></li><li id="k-ZonedZonesDiffer"><code><strong class="tok-constructor">ZonedZonesDiffer</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a></code></li><li id="k-ZonedBadUnits"><code><strong class="tok-constructor">ZonedBadUnits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a></code></li><li id="k-ZonedBadIncrement"><code><strong class="tok-constructor">ZonedBadIncrement</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a></code></li><li id="k-ZonedUnitLimit"><code><strong class="tok-constructor">ZonedUnitLimit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ZonedRound">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZonedRound</strong></code><span class="doc-sig-links"><a href="#t-ZonedRound" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L970" title="View source at line 970">Source</a></span></div>
<div class="doc-body">

The options of `zoned-round-with` (T7): smallest unit, increment,
mode.

<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-RoundingOptions" title="Data.Time.Unit.RoundingOptions">RoundingOptions</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedRound" title="Data.Time.Zoned.Arithmetic.ZonedRound">ZonedRound</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#instances">Data.Time.Zoned.Arithmetic</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-span-balance-zoned">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-balance-zoned</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">c</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code><span class="doc-sig-links"><a href="#v-span-balance-zoned" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L1059" title="View source at line 1059">Source</a></span></div>
<div class="doc-body">

Respell any span with units up to `largest`, measured from the
anchor (T8: not rounding). 2756 hours from 2020-01-01T00:00 in
Europe/Amsterdam to `Year` is 3 months 23 days 21 hours; 1 day
from 2024-11-03T00:00 in New York to `Hour` is 25 hours.

</div>
</article>

<article class="doc-entry" id="v-span-checked-add-zoned">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-checked-add-zoned</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code><span class="doc-sig-links"><a href="#v-span-checked-add-zoned" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L1137" title="View source at line 1137">Source</a></span></div>
<div class="doc-body">

The sum of any two spans measured from the anchor (Jiff's
`Span::checked_add` with a zoned relative): the anchor moves by
`a`, then by `b`, and the result is the difference from the anchor
to that instant, spelled up to the larger of the two spans'
largest units.
  Err ZonedOutOfRange  a step leaves the range
  Err ZonedUnitLimit   a unit of the result passes its limit

</div>
</article>

<article class="doc-entry" id="v-span-checked-sub-zoned">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-checked-sub-zoned</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code><span class="doc-sig-links"><a href="#v-span-checked-sub-zoned" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L1147" title="View source at line 1147">Source</a></span></div>
</article>

<article class="doc-entry" id="v-span-compare-zoned">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-compare-zoned</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-span-compare-zoned" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L1117" title="View source at line 1117">Source</a></span></div>
<div class="doc-body">

Any two spans by where they land from the anchor (Temporal's
Duration.compare with a zoned relativeTo): 1 day and 24 hours are
`EQ` from most instants and differ from 2024-11-03T00:00 in New
York. Spans with no calendar units compare by elapsed time.
  Err ZonedOutOfRange  the anchor plus a span leaves the range

</div>
</article>

<article class="doc-entry" id="v-span-round-zoned">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-round-zoned</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">c</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanRound" title="Data.Time.Span.SpanRound">SpanRound</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code><span class="doc-sig-links"><a href="#v-span-round-zoned" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L1048" title="View source at line 1048">Source</a></span></div>
<div class="doc-body">

Round any span, measured from the anchor (Temporal's
Duration.prototype.round with a zoned relativeTo), with the same
options value as `span-round`.
  Err ZonedBadUnits, ZonedBadIncrement  as `span-round`
  Err ZonedOutOfRange  the anchor plus the span, an intermediate
                       reading or a rounding candidate leaves the
                       range
  Err ZonedUnitLimit   a unit of the result passes its limit

</div>
</article>

<article class="doc-entry" id="v-span-total-zoned">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-total-zoned</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">c</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a>)</code><span class="doc-sig-links"><a href="#v-span-total-zoned" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L1089" title="View source at line 1089">Source</a></span></div>
<div class="doc-body">

The exact length of any span in `unit`, measured from the anchor,
as a reduced fraction with a positive denominator (Temporal's
Duration.prototype.total with a zoned relativeTo). `Hour` and
smaller: the elapsed time over the unit's length. `Day` and
larger: the whole units plus the fraction of the next one, the
next one measured in the zone (1 hour from 2024-11-03T00:00 in
New York is 1/25 of a day).
  Err ZonedOutOfRange  the anchor plus the span, or the end of
                       the unit being measured, leaves the range

</div>
</article>

<article class="doc-entry" id="v-with-disambiguation">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">with-disambiguation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedArithmetic" title="Data.Time.Zoned.Arithmetic.ZonedArithmetic">ZonedArithmetic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedArithmetic" title="Data.Time.Zoned.Arithmetic.ZonedArithmetic">ZonedArithmetic</a></code><span class="doc-sig-links"><a href="#v-with-disambiguation" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L411" title="View source at line 411">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-add-duration">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-add-duration</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-add-duration" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L442" title="View source at line 442">Source</a></span></div>
<div class="doc-body">

Exact elapsed time, on the instant. `None` when the instant leaves
the range.

</div>
</article>

<article class="doc-entry" id="v-zoned-add-span">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-add-span</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">c</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-add-span" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L434" title="View source at line 434">Source</a></span></div>
<div class="doc-body">

The same, `Compatible`: 2024-05-31T13:45 in New York plus one
month is 2024-06-30T13:45 there.

</div>
</article>

<article class="doc-entry" id="v-zoned-add-span-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-add-span-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">c</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedArithmetic" title="Data.Time.Zoned.Arithmetic.ZonedArithmetic">ZonedArithmetic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-add-span-with" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L423" title="View source at line 423">Source</a></span></div>
<div class="doc-body">

Add a span (the header's ADD law) under the options.
  Err ZonedOutOfRange    the date or the instant leaves the range
  Err ZonedRejectedGap,
      ZonedRejectedFold  the options say `Reject` and the calendar
                         units land on such a reading

</div>
</article>

<article class="doc-entry" id="v-zoned-arithmetic-default">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-arithmetic-default</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedArithmetic" title="Data.Time.Zoned.Arithmetic.ZonedArithmetic">ZonedArithmetic</a></code><span class="doc-sig-links"><a href="#v-zoned-arithmetic-default" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L408" title="View source at line 408">Source</a></span></div>
<div class="doc-body">

`Compatible`.

</div>
</article>

<article class="doc-entry" id="v-zoned-arithmetic-disambiguation">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-arithmetic-disambiguation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedArithmetic" title="Data.Time.Zoned.Arithmetic.ZonedArithmetic">ZonedArithmetic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a></code><span class="doc-sig-links"><a href="#v-zoned-arithmetic-disambiguation" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L414" title="View source at line 414">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-day-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-day-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-zoned-day-length" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L490" title="View source at line 490">Source</a></span></div>
<div class="doc-body">

The length of `x`'s civil day in its zone (Temporal's hoursInDay,
exact): from the start of the day to the start of the next.
23 hours on 2024-03-10 in New York, 25 on 2024-11-03. `None` at
the two ends of the range.

</div>
</article>

<article class="doc-entry" id="v-zoned-difference-default">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-difference-default</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedDifference" title="Data.Time.Zoned.Arithmetic.ZonedDifference">ZonedDifference</a></code><span class="doc-sig-links"><a href="#v-zoned-difference-default" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L843" title="View source at line 843">Source</a></span></div>
<div class="doc-body">

Both units unset, increment 1, `Trunc`.

</div>
</article>

<article class="doc-entry" id="v-zoned-difference-largest">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-difference-largest</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedDifference" title="Data.Time.Zoned.Arithmetic.ZonedDifference">ZonedDifference</a></code><span class="doc-sig-links"><a href="#v-zoned-difference-largest" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L847" title="View source at line 847">Source</a></span></div>
<div class="doc-body">

The common case: the default with a largest unit.

</div>
</article>

<article class="doc-entry" id="v-zoned-first-of-month">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-first-of-month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-first-of-month" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L518" title="View source at line 518">Source</a></span></div>
<div class="doc-body">

Lane B's civil operation on the date, the time of day kept,
resolved `Compatible`.

</div>
</article>

<article class="doc-entry" id="v-zoned-first-of-year">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-first-of-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-first-of-year" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L524" title="View source at line 524">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-last-of-month">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-last-of-month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-last-of-month" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L521" title="View source at line 521">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-last-of-year">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-last-of-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-last-of-year" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L527" title="View source at line 527">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-nth-weekday">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-nth-weekday</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-nth-weekday" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L532" title="View source at line 532">Source</a></span></div>
<div class="doc-body">

`Err ZonedOutOfRange` also when the civil operation refuses (`n`
of zero, a weekday the month does not have that many of).

</div>
</article>

<article class="doc-entry" id="v-zoned-nth-weekday-of-month">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-nth-weekday-of-month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-nth-weekday-of-month" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L535" title="View source at line 535">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-round">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-round</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-round" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L1029" title="View source at line 1029">Source</a></span></div>
<div class="doc-body">

Round to a unit, increment 1, `HalfExpand`.

</div>
</article>

<article class="doc-entry" id="v-zoned-round-default">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-round-default</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedRound" title="Data.Time.Zoned.Arithmetic.ZonedRound">ZonedRound</a></code><span class="doc-sig-links"><a href="#v-zoned-round-default" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L974" title="View source at line 974">Source</a></span></div>
<div class="doc-body">

`Nanosecond`, 1, `HalfExpand`: rounds nothing until a unit is set.

</div>
</article>

<article class="doc-entry" id="v-zoned-round-to">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-round-to</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedRound" title="Data.Time.Zoned.Arithmetic.ZonedRound">ZonedRound</a></code><span class="doc-sig-links"><a href="#v-zoned-round-to" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L978" title="View source at line 978">Source</a></span></div>
<div class="doc-body">

The common case: the default with a smallest unit.

</div>
</article>

<article class="doc-entry" id="v-zoned-round-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-round-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedRound" title="Data.Time.Zoned.Arithmetic.ZonedRound">ZonedRound</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-round-with" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L1017" title="View source at line 1017">Source</a></span></div>
<div class="doc-body">

Round a value (the header's ROUND law): 17:46:20.123456789 on
2024-11-02 in New York to 30 minutes is 18:00 -04:00; 11:45:20 on
2024-11-03 to `Day` is 2024-11-04T00:00 -05:00, because 11:45 is
past the middle of a 25-hour day.
  Err ZonedBadUnits      week or larger
  Err ZonedBadIncrement  the increment does not divide the next
                         larger unit, or equals it; for `Day`,
                         anything but 1
  Err ZonedOutOfRange    the result leaves the range

</div>
</article>

<article class="doc-entry" id="v-zoned-since">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-since</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span></code><span class="doc-sig-links"><a href="#v-zoned-since" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L896" title="View source at line 896">Source</a></span></div>
<div class="doc-body">

The elapsed time since `b`, to `a`: the negation of
`zoned-until a b`.

</div>
</article>

<article class="doc-entry" id="v-zoned-since-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-since-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedDifference" title="Data.Time.Zoned.Arithmetic.ZonedDifference">ZonedDifference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code><span class="doc-sig-links"><a href="#v-zoned-since-with" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L913" title="View source at line 913">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-start-of-next-day">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-start-of-next-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-start-of-next-day" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L479" title="View source at line 479">Source</a></span></div>
<div class="doc-body">

The first instant of the civil day after `x`'s. `None` at the end
of the range. (When a zone skipped a whole civil day, as
Pacific/Apia skipped 2011-12-30, this is the start of the day
after the skipped one, as in Temporal.)

</div>
</article>

<article class="doc-entry" id="v-zoned-sub-duration">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-sub-duration</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-sub-duration" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L447" title="View source at line 447">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-sub-span">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-sub-span</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">c</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-sub-span" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L437" title="View source at line 437">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-sub-span-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-sub-span-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">c</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedArithmetic" title="Data.Time.Zoned.Arithmetic.ZonedArithmetic">ZonedArithmetic</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-sub-span-with" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L429" title="View source at line 429">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-tomorrow">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-tomorrow</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-tomorrow" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L510" title="View source at line 510">Source</a></span></div>
<div class="doc-body">

The same civil time on the next civil day, `Compatible`: tomorrow
of 2024-11-02T17:25 in New York is 2024-11-03T17:25, 25 hours
later; of 2024-03-09T02:30 it is 2024-03-10T03:30 (02:30 does not
exist). For another strategy add `span-of-days 1` with options.
  Err ZonedOutOfRange  the end of the range

</div>
</article>

<article class="doc-entry" id="v-zoned-until">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-until</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Clock</span></code><span class="doc-sig-links"><a href="#v-zoned-until" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L888" title="View source at line 888">Source</a></span></div>
<div class="doc-body">

The elapsed time from `a` to `b` in hours and smaller, exact; the
zones may differ. 2024-11-02T17:25 until 2024-11-03T17:25 in New
York is 25 hours. Total: the whole timestamp range is inside the
hour limit, so the `Err` arm is unreachable.

</div>
</article>

<article class="doc-entry" id="v-zoned-until-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-until-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedDifference" title="Data.Time.Zoned.Arithmetic.ZonedDifference">ZonedDifference</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-Span" title="Data.Time.Span.Span">Span</a> <span class="tok-constructor">Calendar</span>)</code><span class="doc-sig-links"><a href="#v-zoned-until-with" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L910" title="View source at line 910">Source</a></span></div>
<div class="doc-body">

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

</div>
</article>

<article class="doc-entry" id="v-zoned-with-date">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-with-date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-with-date" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L958" title="View source at line 958">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-with-date-resolving">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-with-date-resolving</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-with-date-resolving" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L948" title="View source at line 948">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-with-datetime">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-with-datetime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-with-datetime" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L955" title="View source at line 955">Source</a></span></div>
<div class="doc-body">

The same, `Compatible`.

</div>
</article>

<article class="doc-entry" id="v-zoned-with-datetime-resolving">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-with-datetime-resolving</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-with-datetime-resolving" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L943" title="View source at line 943">Source</a></span></div>
<div class="doc-body">

The value with its civil date, time of day or both replaced
(ZonedDateTime.prototype.with), keeping the value's offset when
the new reading can have it and otherwise resolving by the
strategy.
  Err ZonedRejectedGap, ZonedRejectedFold  the strategy is
                       `Reject` and the offset could not be kept
  Err ZonedOutOfRange  the instant leaves the range

</div>
</article>

<article class="doc-entry" id="v-zoned-with-time">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-with-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-with-time" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L961" title="View source at line 961">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-with-time-resolving">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-with-time-resolving</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-with-time-resolving" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L951" title="View source at line 951">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-yesterday">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-yesterday</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedError" title="Data.Time.Zoned.Arithmetic.ZonedError">ZonedError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-yesterday" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/source/#L513" title="View source at line 513">Source</a></span></div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-LargestOption_ZonedDifference"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-LargestOption" title="Data.Time.Unit.LargestOption">LargestOption</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedDifference" title="Data.Time.Zoned.Arithmetic.ZonedDifference">ZonedDifference</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Zoned.Arithmetic.LargestOption_ZonedDifference</code></span></li>
<li id="i-RoundingOptions_ZonedDifference"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-RoundingOptions" title="Data.Time.Unit.RoundingOptions">RoundingOptions</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedDifference" title="Data.Time.Zoned.Arithmetic.ZonedDifference">ZonedDifference</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Zoned.Arithmetic.RoundingOptions_ZonedDifference</code></span></li>
<li id="i-RoundingOptions_ZonedRound"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-RoundingOptions" title="Data.Time.Unit.RoundingOptions">RoundingOptions</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedRound" title="Data.Time.Zoned.Arithmetic.ZonedRound">ZonedRound</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Zoned.Arithmetic.RoundingOptions_ZonedRound</code></span></li>
</ul>
