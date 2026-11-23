# Data.Time.Span

Calendar-aware spans that keep each unit (years through nanoseconds), with balancing and rounding.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Time.Span`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Time.Span.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Unit](Data.Time.Unit.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Span](Data.Time.Span.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Unit](Data.Time.Unit.md), [Data.Time.Weekday](Data.Time.Weekday.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Time.Span: a calendar-aware duration that keeps each unit.

A `Span` is TC39 Temporal's Duration as rendered by Jiff's `Span`:
ten unit values (years, months, weeks, days, hours, minutes, seconds,
milliseconds, microseconds, nanoseconds) that all share one sign.
"1 month 2 days" stays one month and two days: how long that is
depends on the date it is added to, which is why a `Span` is not a
`Duration` (Data.Time.Duration, an exact count of nanoseconds).

Representation. One sign (-1, 0, 1) and ten non-negative `I64`
magnitudes behind a private constructor (Jiff's layout,
design/datetime.md section 4). As held today (verified against the
emitted LLVM): one heap object of eleven unboxed `I64` words, read
at offsets 0, 8, .. 80; the class index is not stored in it.
Every magnitude is within its unit's limit (Jiff's limits, chosen so
any single unit spans at most the supported date range):

    years         19998          hours         175307616
    months        239976         minutes       10518456960
    weeks         1043497        seconds       631107417600
    days          7304484        milliseconds  631107417600000
                                 microseconds  631107417600000000
                                 nanoseconds   9223372036854775807

so the clock units of any span total fewer than 1.3e21 nanoseconds,
and days and weeks read as 24-hour days add fewer than 1.3e21 more:
every sum formed here is below 2^72 and lives in an `I128` with room
to spare. That bound is the overflow argument for every `wrapping-`
`I128` operation in this module; the few places where a caller's
unbounded value enters (a `Duration`, a rounding increment times a
unit) use a checked operation or a prior range test and say so.

The index (design/datetime.md T6). `Span (c :: SpanClass)` says
whether the span may hold calendar units. `Span Clock` holds hours
and smaller only: its length is a fixed number of nanoseconds, so it
can be compared, totalled, rounded and converted to a `Duration` with
no anchor, and those operations take `Span Clock` and cannot be
handed anything else. `Span Calendar` may hold days and larger; the
`-relative` forms take any class and an anchor (`Relative`). The
setters for hours and smaller preserve the class; the setters for
days and larger answer `Span Calendar` (even when the value set is
zero: the index is an upper bound, not a census). `span-to-clock` is
the checked way down and `span-to-calendar` the free way up.

A day is a calendar unit here, as in Jiff: Temporal lets a civil
Duration treat days as 24 hours with no anchor, and that convention
is available only under a name that says so
(`span-to-duration-civil-days`).

Design law shape.
  - All non-zero units share the span's sign. `span-negate` flips it,
    `span-abs` clears it; each accessor returns its unit with the
    sign applied, so `span-with-<unit> (span-<unit> s) s` is `Some s`.
  - `span-fieldwise-eq` is the ONLY equality: unit by unit. There is
    deliberately NO `Eq` instance. 2 hours and 120 minutes are
    different spans that have the same length, and 1 month and 30
    days are different spans whose lengths agree only sometimes; an
    `==` would have to pick one reading silently. Ask for
    `span-fieldwise-eq` (same fields) or `span-compare` /
    `span-compare-relative` (same length) by name.
  - `span-balance` changes only how a length is spelled: the result
    has the same length (the same `span-total` in every unit, the
    same `span-compare` class) with every unit above `largest` zero
    and every unit below it within its natural bound (minutes below
    60 and so on). Balancing is not rounding (T8): it never changes
    the length.
  - `span-round` is Temporal's Duration.prototype.round: round the
    length to a multiple of `increment` of `smallest` under `mode`,
    then balance up to `largest`. With a calendar `smallest` (week,
    month, year) the two candidates are the anchor plus the span's
    value truncated to the increment and that plus one increment
    (NudgeToCalendarUnit), and a result that fills a larger unit is
    carried up (BubbleRelativeDuration). When that window does not
    hold the point the span reaches, the two candidates are taken
    one increment further (ComputeNudgeWindow's additionalShift):
    from 2024-01-31T12:00, 1 year 28 days 23 hours reaches
    2025-03-01T11:00, past the anchor plus 1 year 1 month
    (2025-02-28T12:00, the day constrained), so the candidates are
    1 year 1 month and 1 year 2 months and `Ceil` answers the
    latter. A rounding up therefore never lands before the point
    and a rounding down never after it.
  - `span-total unit` is the exact length in `unit` as a reduced
    rational `(numerator, denominator)` with a positive denominator.
    For a calendar unit it is the whole units plus the fraction of
    the next one, measured on the anchor's own calendar (24 hours is
    1/30 of a month from 1 June and 1/31 from 1 July), in the same
    window the rounding uses (the span above is 13 + 23/744 months).
  - `span-compare` orders spans by length; anchored, by where they
    land from the anchor.
  - The anchored forms are all one construction, Temporal's: add the
    span to the anchor (`span-add-to-datetime`), then take the
    difference from the anchor to that point in the units asked for
    (`span-difference-civil`). Adding applies years and months first
    with the day constrained to the target month's length
    (2024-05-31 + 1 month = 2024-06-30), then weeks and days, then
    the clock units with a 24-hour civil day. The difference is
    DifferenceISODateTime: the largest whole count of each unit, from
    the largest down, that does not pass the end point, the day of
    the month being compared unconstrained (2024-05-31 until
    2024-09-30 is 3 months 30 days).

The zoned seam. `Relative` has two constructors, a civil `Date` (read
at midnight) and a civil `DateTime`. Every anchored operation reaches
the anchor through two private functions only: `origin-of` (the
anchor's civil reading) and `line` (a civil reading's position on a
nanosecond line; for civil anchors every day is 24 hours). A zoned
anchor (Data.Time.Zoned.Arithmetic, lane F) needs a third
constructor, `line` through the zone's civil-to-instant mapping, and
Temporal's NudgeToZonedTime beside `nudge-day-time`; nothing else in
the engine assumes a 24-hour day. Lane F did not take that road (it
would make this module import the zone modules): the zoned anchor
(`span-round-zoned` and its family) is in
Data.Time.Zoned.Arithmetic, built on this module's exports
(`span-difference-civil`, `span-difference-nanos`, `span-time-nanos`,
`span-from-units`) and on the four readers of `SpanRound`
(`span-round-largest`, `span-round-smallest`, `span-round-increment`,
`span-round-mode`) exported for it.

No operation faults (T2): refined arguments (`span-of-hours 2`),
`Option` for the setters and simple conversions, and
`Result SpanError` wherever more than one refusal is possible.

NAMED REFUSALS:
  - `Unit`: the unit type is `TimeUnit` (see Data.Time.Unit).
  - an `Eq` instance: refused on purpose (see the law shape).
  - an erased index: the index costs nothing in the object, but a
    function polymorphic in it (`forall (c :: SpanClass).`) receives
    the index as a run-time word argument, and a caller at a known
    class builds and drops a `Clock` or `Calendar` constructor value
    to pass it (seen in the emitted LLVM for `span-negate`). Nothing
    here reads it. Kept, because the index is what the spec asks
    for; it becomes free when the compiler erases unread indices.
  - an index variable without its binder: `Span c -> I64` alone is
    refused ("unknown name c in index position"), so every signature
    polymorphic in the class spells `forall (c :: SpanClass).`.
  - arithmetic in the index: the class of a sum or of a rounded span
    is not computed. The clock-only forms answer `Span Clock`; every
    anchored form answers `Span Calendar` whatever units the result
    has, and `span-to-clock` recovers the stronger index by a test.
  - a refined rational or a float as the total: `span-total` returns
    a pair of `I128` (refinements inside tuples only check for
    literals, and a wide scalar has no solver sort), reduced, with a
    positive denominator. `span-total-f64` divides the two.
  - `span-to-duration-days-24h`: that spelling does not lex (a hyphen
    followed by a digit); the function is
    `span-to-duration-civil-days`.
  - a days-are-24-hours anchor (Jiff's `SpanRelativeTo::
    days_are_24_hours`): not offered; anchor at any civil date
    instead (a civil day is always 24 hours).
  - Temporal's refusal of Duration.round with neither unit given: not
    ported. `span-round-default` rounds to nanoseconds, which only
    rebalances up to the span's own largest unit.
  - the 0.5.1 reference polyfill as the last word on calendar
    rounding: the current specification text (read 2026-11-17) moves
    a year or month rounding window one increment on when it does
    not hold the destination, and that is followed here (`settled`),
    as it is in Data.Time.Zoned.Arithmetic. That polyfill measures
    in the first window regardless (progress above 1), so in that
    one corner (a month or year window whose end had its day
    constrained, and a destination less than a day past it) its
    answers differ from these.
  - rounding at the very end of the range: the further candidate of
    the window is always read, shifted or not, and when it lies
    outside the range the answer is `SpanOutOfRange` even if the
    mode would not have chosen it.
  - text: no printer or parser lives here (Text.Time.Duration).

</details>

# Types and constructors

<a id="type-52656c6174697665"></a>

## `Relative`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Relative</span></code></pre>

What a span with calendar units is measured from. A `Date` is read
at midnight. (The zoned anchor is lane F's; the header describes
the seam.)

[Source](/reference-sources/Data.Time.Span.cinch) · line 1150

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RelativeDate</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Relative</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RelativeDateTime</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Relative</span>)</code></pre>

<a id="type-5370616e"></a>

## `Span`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Span</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>)</code></pre>

The sign (-1, 0, 1), then the ten magnitudes from years down to
nanoseconds (header). Not exported: the sign and limit invariants.

[Source](/reference-sources/Data.Time.Span.cinch) · line 228

<a id="type-5370616e436c617373"></a>

## `SpanClass`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SpanClass</span></code></pre>

Whether a span may hold calendar units (day and larger).

[Source](/reference-sources/Data.Time.Span.cinch) · line 222

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Clock</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Calendar</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span></code></pre>

<a id="type-5370616e4572726f72"></a>

## `SpanError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SpanError</span></code></pre>

The refusals of this module and of Data.Time.Arithmetic.

[Source](/reference-sources/Data.Time.Span.cinch) · line 232

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SpanSignConflict</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SpanUnitLimit</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SpanNeedsAnchor</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SpanBadUnits</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SpanBadIncrement</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SpanOutOfRange</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span></code></pre>

<a id="type-5370616e526f756e64"></a>

## `SpanRound`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SpanRound</span></code></pre>

The options of `span-round` (T7): the largest unit of the result
(unset means the larger of the span's own largest unit and
`smallest`), the smallest unit, the increment and the mode.

[Source](/reference-sources/Data.Time.Span.cinch) · line 1192

# Values

<a id="value-7370616e2d616273"></a>

## `span-abs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-abs</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d6164642d746f2d64617465"></a>

## `span-add-to-date`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-add-to-date</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>))</code></pre>

Temporal's PlainDate.prototype.add: the clock units are balanced
into whole 24-hour days, truncated toward zero, and join the days
(ToDateDurationRecordWithoutTime); the remainder below a day is
dropped. `None` when the date leaves the range.

[Source](/reference-sources/Data.Time.Span.cinch) · line 757

<a id="value-7370616e2d6164642d746f2d6461746574696d65"></a>

## `span-add-to-datetime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-add-to-datetime</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>))</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d62616c616e6365"></a>

## `span-balance`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-balance</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>)))</code></pre>

Respell a clock span with units up to `largest` (`Hour` or
smaller): 90 minutes to `Hour` is 1 hour 30 minutes, and back to
`Minute` is 90 minutes.
  Err SpanNeedsAnchor  `largest` is a calendar unit
  Err SpanUnitLimit    a unit of the result passes its limit

[Source](/reference-sources/Data.Time.Span.cinch) · line 1168

<a id="value-7370616e2d62616c616e63652d72656c6174697665"></a>

## `span-balance-relative`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-balance-relative</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Relative</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))))</code></pre>

Respell any span with units up to `largest`, measured from the
anchor: 2756 hours from 2020-01-01 to `Year` is 3 months 23 days
20 hours.
  Err SpanOutOfRange  the anchor plus the span leaves the range
  Err SpanUnitLimit   a unit of the result passes its limit

[Source](/reference-sources/Data.Time.Span.cinch) · line 1178

<a id="value-7370616e2d636865636b65642d616464"></a>

## `span-checked-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-checked-add</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>)))</code></pre>

The sum of two clock spans, spelled up to the larger of their
largest units (Temporal's AddDurations). `None` when a unit of the
result passes its limit.

[Source](/reference-sources/Data.Time.Span.cinch) · line 1397

<a id="value-7370616e2d636865636b65642d6164642d72656c6174697665"></a>

## `span-checked-add-relative`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-checked-add-relative</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Relative</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">b</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))))</code></pre>

The sum of any two spans measured from the anchor (Jiff's
`Span::checked_add` with a relative date-time): the anchor moves by
`a`, then by `b`, and the result is the difference from the anchor
to that point, spelled up to the larger of the two spans' largest
units.
  Err SpanOutOfRange  a step leaves the range
  Err SpanUnitLimit   a unit of the result passes its limit

[Source](/reference-sources/Data.Time.Span.cinch) · line 1412

<a id="value-7370616e2d636865636b65642d737562"></a>

## `span-checked-sub`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-checked-sub</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d636865636b65642d7375622d72656c6174697665"></a>

## `span-checked-sub-relative`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-checked-sub-relative</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Relative</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">b</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))))</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d636c6f636b2d66726f6d2d756e697473"></a>

## `span-clock-from-units`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-clock-from-units</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>)))))))</code></pre>

The six clock units at once, from hours down to nanoseconds.

[Source](/reference-sources/Data.Time.Span.cinch) · line 449

<a id="value-7370616e2d636f6d70617265"></a>

## `span-compare`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-compare</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>))</code></pre>

Clock spans by length: 2 hours and 120 minutes are `EQ`.

[Source](/reference-sources/Data.Time.Span.cinch) · line 1356

<a id="value-7370616e2d636f6d706172652d72656c6174697665"></a>

## `span-compare-relative`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-compare-relative</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Relative</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">b</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>)))</code></pre>

Any two spans by length, measured from the anchor (Temporal's
Duration.compare with a plain relativeTo: the date parts are turned
into days from the anchor's date, and a civil day is 24 hours).
One month and 30 days are `EQ` from 1 June and differ from 1 July.
  Err SpanOutOfRange  the anchor plus a span's date part leaves the
                      range

[Source](/reference-sources/Data.Time.Span.cinch) · line 1379

<a id="value-7370616e2d64617973"></a>

## `span-days`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-days</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d646966666572656e63652d636976696c"></a>

## `span-difference-civil`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-difference-civil</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundMode</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>)))))))</code></pre>

The engine behind every civil `until`/`since` and every anchored
span operation: the span from `a` to `b` with units up to
`largest`, rounded to `inc` of `smallest` under `mode` relative to
`a` (Temporal's DifferencePlainDateTimeWithRounding and
TemporalDurationFromInternal). Data.Time.Arithmetic gives it its
everyday names and defaults.
  Err SpanBadUnits      `largest` is smaller than `smallest`
  Err SpanBadIncrement  `inc` is not valid for `smallest`
                        (`unit-span-increment-ok`)
  Err SpanOutOfRange    a rounding candidate lies outside the
                        date range
  Err SpanUnitLimit     a unit of the result passes its limit

[Source](/reference-sources/Data.Time.Span.cinch) · line 1114

<a id="value-7370616e2d646966666572656e63652d6e616e6f73"></a>

## `span-difference-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-difference-nanos</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundMode</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>))))))</code></pre>

The engine for differences that are a plain nanosecond count (two
times of day, two timestamps) and for clock-only spans: round
`nanos` to `inc` of `smallest` under `mode` (RoundTimeDuration),
then spell it in units up to `largest`. Both units must be `Hour`
or smaller.
  Err SpanBadUnits      a calendar unit, or `largest` smaller than
                        `smallest`
  Err SpanBadIncrement  `inc` is not valid for `smallest`
  Err SpanUnitLimit     the count is `I128`'s minimum, the rounded
                        value leaves `I128`, or a unit of the
                        result passes its limit

[Source](/reference-sources/Data.Time.Span.cinch) · line 1133

<a id="value-7370616e2d6669656c64776973652d6571"></a>

## `span-fieldwise-eq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-fieldwise-eq</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">b</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Unit by unit, and the sign. The only equality (header).

[Source](/reference-sources/Data.Time.Span.cinch) · line 580

<a id="value-7370616e2d66726f6d2d6475726174696f6e"></a>

## `span-from-duration`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-from-duration</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>)))</code></pre>

A `Duration` spelled in units up to `largest`, which must be `Hour`
or smaller (a duration has no days: `None` for a calendar unit).
`None` also when a unit would pass its limit (a duration can be far
longer than any span).

[Source](/reference-sources/Data.Time.Span.cinch) · line 699

<a id="value-7370616e2d66726f6d2d756e697473"></a>

## `span-from-units`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-from-units</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>)))))))))))</code></pre>

All ten units at once, signed, from years down to nanoseconds.
`Err SpanUnitLimit` when a unit is beyond its limit,
`Err SpanSignConflict` when two units have opposite signs.

[Source](/reference-sources/Data.Time.Span.cinch) · line 445

<a id="value-7370616e2d686f757273"></a>

## `span-hours`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-hours</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d69732d636c6f636b"></a>

## `span-is-clock`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-is-clock</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether every calendar unit (days and larger) is zero.

[Source](/reference-sources/Data.Time.Span.cinch) · line 603

<a id="value-7370616e2d69732d6e65676174697665"></a>

## `span-is-negative`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-is-negative</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d69732d706f736974697665"></a>

## `span-is-positive`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-is-positive</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d69732d7a65726f"></a>

## `span-is-zero`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-is-zero</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d6c6172676573742d756e6974"></a>

## `span-largest-unit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-largest-unit</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span>)</code></pre>

The largest non-zero unit; `Nanosecond` for the zero span
(Temporal's DefaultTemporalLargestUnit).

[Source](/reference-sources/Data.Time.Span.cinch) · line 589

<a id="value-7370616e2d6d6963726f7365636f6e6473"></a>

## `span-microseconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-microseconds</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d6d696c6c697365636f6e6473"></a>

## `span-milliseconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-milliseconds</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d6d696e75746573"></a>

## `span-minutes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-minutes</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d6d6f6e746873"></a>

## `span-months`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-months</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d6e616e6f7365636f6e6473"></a>

## `span-nanoseconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-nanoseconds</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d6e6567617465"></a>

## `span-negate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-negate</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span>)</code></pre>

Total: only the sign changes.

[Source](/reference-sources/Data.Time.Span.cinch) · line 571

<a id="value-7370616e2d6f662d64617973"></a>

## `span-of-days`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-of-days</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-7304484</span> <span class="tok-number">7304484</span> <span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d6f662d686f757273"></a>

## `span-of-hours`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-of-hours</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-175307616</span> <span class="tok-number">175307616</span> <span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d6f662d6d6963726f7365636f6e6473"></a>

## `span-of-microseconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-of-microseconds</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-631107417600000000</span> <span class="tok-number">631107417600000000</span> <span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d6f662d6d696c6c697365636f6e6473"></a>

## `span-of-milliseconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-of-milliseconds</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-631107417600000</span> <span class="tok-number">631107417600000</span> <span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d6f662d6d696e75746573"></a>

## `span-of-minutes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-of-minutes</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-10518456960</span> <span class="tok-number">10518456960</span> <span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d6f662d6d6f6e746873"></a>

## `span-of-months`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-of-months</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-239976</span> <span class="tok-number">239976</span> <span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d6f662d6e616e6f7365636f6e6473"></a>

## `span-of-nanoseconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-of-nanoseconds</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-9223372036854775807</span> <span class="tok-number">9223372036854775807</span> <span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d6f662d7365636f6e6473"></a>

## `span-of-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-of-seconds</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-631107417600</span> <span class="tok-number">631107417600</span> <span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d6f662d7765656b73"></a>

## `span-of-weeks`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-of-weeks</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-1043497</span> <span class="tok-number">1043497</span> <span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d6f662d7965617273"></a>

## `span-of-years`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-of-years</span> :: (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-19998</span> <span class="tok-number">19998</span> <span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>)</code></pre>

One unit, total: the limit is in the argument's type, so
`span-of-hours 2` checks with nothing else written and a literal
beyond the limit is refused at compile time.

[Source](/reference-sources/Data.Time.Span.cinch) · line 455

<a id="value-7370616e2d726f756e64"></a>

## `span-round`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-round</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanRound</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>)))</code></pre>

Round a clock span: 2 hours 30 minutes to `Hour` is 3 hours under
`HalfExpand` and 2 under `Trunc`.
  Err SpanNeedsAnchor   a calendar unit is asked for
  Err SpanBadUnits      `largest` is smaller than `smallest`
  Err SpanBadIncrement  the increment is not valid for `smallest`
  Err SpanUnitLimit     a unit of the result passes its limit

[Source](/reference-sources/Data.Time.Span.cinch) · line 1253

<a id="value-7370616e2d726f756e642d64656661756c74"></a>

## `span-round-default`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-round-default</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanRound</span></code></pre>

Largest unset, smallest `Nanosecond`, increment 1, `HalfExpand`
(Temporal's defaults for Duration.prototype.round).

[Source](/reference-sources/Data.Time.Span.cinch) · line 1197

<a id="value-7370616e2d726f756e642d696e6372656d656e74"></a>

## `span-round-increment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-round-increment</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanRound</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d726f756e642d6c617267657374"></a>

## `span-round-largest`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-round-largest</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanRound</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span>))</code></pre>

The validated settings of a `SpanRound` for a given span, for an
engine outside this module (Data.Time.Zoned.Arithmetic, whose
anchor is a `Zoned`): the resolved largest unit or the refusal
(as `span-round` would answer it), and the three plain knobs.

[Source](/reference-sources/Data.Time.Span.cinch) · line 1232

<a id="value-7370616e2d726f756e642d6d6f6465"></a>

## `span-round-mode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-round-mode</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanRound</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">RoundMode</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d726f756e642d72656c6174697665"></a>

## `span-round-relative`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-round-relative</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanRound</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Relative</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>))))</code></pre>

Round any span, measured from the anchor: 1 month 1 day 1 hour
from 2022-01-01 with largest and smallest `Day` is 32 days.
  Err SpanBadUnits, SpanBadIncrement  as above
  Err SpanOutOfRange  the anchor plus the span, or a rounding
                      candidate, leaves the range
  Err SpanUnitLimit   a unit of the result passes its limit

[Source](/reference-sources/Data.Time.Span.cinch) · line 1267

<a id="value-7370616e2d726f756e642d736d616c6c657374"></a>

## `span-round-smallest`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-round-smallest</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanRound</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d7365636f6e6473"></a>

## `span-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-seconds</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d7369676e"></a>

## `span-sign`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-sign</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-1</span> <span class="tok-number">1</span> <span class="tok-constructor">I64</span>)</code></pre>

-1, 0 or 1.

[Source](/reference-sources/Data.Time.Span.cinch) · line 556

<a id="value-7370616e2d74696d652d6e616e6f73"></a>

## `span-time-nanos`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-time-nanos</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

The clock units (hours and smaller) as one signed nanosecond count;
the calendar units are not part of it. Below 1.3e21 in magnitude
(header), so the wrapping sums are exact.

[Source](/reference-sources/Data.Time.Span.cinch) · line 618

<a id="value-7370616e2d746f2d63616c656e646172"></a>

## `span-to-calendar`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-to-calendar</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>)</code></pre>

The free way up.

[Source](/reference-sources/Data.Time.Span.cinch) · line 612

<a id="value-7370616e2d746f2d636c6f636b"></a>

## `span-to-clock`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-to-clock</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span>))</code></pre>

The checked way down the index: `None` when a calendar unit is
non-zero.

[Source](/reference-sources/Data.Time.Span.cinch) · line 608

<a id="value-7370616e2d746f2d6475726174696f6e"></a>

## `span-to-duration`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-to-duration</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>)</code></pre>

Total on the clock class: the exact length. (The count is below
1.3e21, inside `Duration`'s range, so the `None` arm is
unreachable.)

[Source](/reference-sources/Data.Time.Span.cinch) · line 707

<a id="value-7370616e2d746f2d6475726174696f6e2d636865636b6564"></a>

## `span-to-duration-checked`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-to-duration-checked</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>)</code></pre>

The same for any class, by a test: `None` when a calendar unit
(days or larger) is non-zero, since its length depends on a date.

[Source](/reference-sources/Data.Time.Span.cinch) · line 714

<a id="value-7370616e2d746f2d6475726174696f6e2d636976696c2d64617973"></a>

## `span-to-duration-civil-days`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-to-duration-civil-days</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Duration</span>.<span class="tok-constructor">Duration</span>)</code></pre>

The named convention "a day is 24 hours" (and so a week is 168):
exact for civil time, wrong across a zone transition, which is why
it has to be asked for by name. Years and months have no fixed
length under any convention: `None`.

[Source](/reference-sources/Data.Time.Span.cinch) · line 722

<a id="value-7370616e2d746f74616c"></a>

## `span-total`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-total</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">I128</span>, <span class="tok-constructor">I128</span>)))</code></pre>

The exact length of a clock span in `unit` (`Hour` or smaller), as
a reduced fraction: 90 minutes in hours is (3, 2).
  Err SpanNeedsAnchor  `unit` is a calendar unit

[Source](/reference-sources/Data.Time.Span.cinch) · line 1293

<a id="value-7370616e2d746f74616c2d663634"></a>

## `span-total-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-total-f64</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> <span class="tok-constructor">F64</span>))</code></pre>

The totals as the nearest `F64` to the exact fraction's two halves
divided (what Temporal and Jiff return).

[Source](/reference-sources/Data.Time.Span.cinch) · line 1342

<a id="value-7370616e2d746f74616c2d72656c6174697665"></a>

## `span-total-relative`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-total-relative</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Relative</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> (<span class="tok-constructor">I128</span>, <span class="tok-constructor">I128</span>))))</code></pre>

The exact length of any span in `unit`, measured from the anchor
(Temporal's Duration.prototype.total with a plain relativeTo). For
week, month and year: the whole units, plus the fraction of the
next one that the remainder covers on the anchor's calendar
(NudgeToCalendarUnit's total, increment 1).
  Err SpanOutOfRange  the anchor plus the span, or the end of the
                      unit being measured, leaves the range

[Source](/reference-sources/Data.Time.Span.cinch) · line 1305

<a id="value-7370616e2d746f74616c2d72656c61746976652d663634"></a>

## `span-total-relative-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-total-relative-f64</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Unit</span>.<span class="tok-constructor">TimeUnit</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Relative</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanError</span> <span class="tok-constructor">F64</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d7765656b73"></a>

## `span-weeks`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-weeks</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d776974682d64617973"></a>

## `span-with-days`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-with-days</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d776974682d686f757273"></a>

## `span-with-hours`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-with-hours</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d776974682d6d6963726f7365636f6e6473"></a>

## `span-with-microseconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-with-microseconds</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d776974682d6d696c6c697365636f6e6473"></a>

## `span-with-milliseconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-with-milliseconds</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d776974682d6d696e75746573"></a>

## `span-with-minutes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-with-minutes</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d776974682d6d6f6e746873"></a>

## `span-with-months`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-with-months</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d776974682d6e616e6f7365636f6e6473"></a>

## `span-with-nanoseconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-with-nanoseconds</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d776974682d7365636f6e6473"></a>

## `span-with-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-with-seconds</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d776974682d7765656b73"></a>

## `span-with-weeks`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-with-weeks</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Span.cinch)

<a id="value-7370616e2d776974682d7965617273"></a>

## `span-with-years`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-with-years</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Calendar</span>)))</code></pre>

Set one unit to a signed value, keeping the others. `None` when the
value is beyond the unit's limit or its sign conflicts with another
non-zero unit. Setting a unit to zero always succeeds. The four
calendar setters answer `Span Calendar`; the six clock setters keep
the class.

[Source](/reference-sources/Data.Time.Span.cinch) · line 490

<a id="value-7370616e2d7965617273"></a>

## `span-years`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-years</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">c</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">SpanClass</span>) . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-variable">c</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

Each unit with the span's sign applied.

[Source](/reference-sources/Data.Time.Span.cinch) · line 525

<a id="value-7370616e2d7a65726f"></a>

## `span-zero`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span-zero</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Span</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Span</span>.<span class="tok-constructor">Clock</span></code></pre>

Construction.

[Source](/reference-sources/Data.Time.Span.cinch) · line 439

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Time.Span.LargestOption_SpanRound` | `Data.Time.Unit.LargestOption Data.Time.Span.SpanRound` | Implicit |
| `Data.Time.Span.RoundingOptions_SpanRound` | `Data.Time.Unit.RoundingOptions Data.Time.Span.SpanRound` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Time/Span.cinch`. Source SHA-256: `4d4838e0610c2143ead9b6c2cc853f0a9a7c724aa833d6d892ada772a14c39c1`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
