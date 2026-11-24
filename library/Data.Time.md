# Data.Time

Time values, durations, and conversions with explicit units and admitted ranges.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Time</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Time/source/">View source</a><a href="/packages/search/?q=%2BData.Time">Search module</a><a href="/reference-sources/Data.Time.cinch" download="Data.Time.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Date](Data.Date.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Clock](Effect.Clock.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

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
seed/tests/lib_civil_time.rs.

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

<details class="doc-synopsis" open><summary>Synopsis (17 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Time">Time</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Time</strong></code></li><li><a class="syn-name" href="#t-TimeParseError">TimeParseError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TimeParseError</strong></code></li><li><a class="syn-name" href="#v-add-duration">add-duration</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add-duration</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a></code></li><li><a class="syn-name" href="#v-cmp">cmp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-date-part">date-part</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">date-part</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-diff">diff</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">diff</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-from-epoch-nanos">from-epoch-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-epoch-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a></code></li><li><a class="syn-name" href="#v-from-string">from-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-TimeParseError" title="Data.Time.TimeParseError">TimeParseError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a></code></li><li><a class="syn-name" href="#v-hour">hour</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hour</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-minute">minute</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">minute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-nanosecond">nanosecond</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nanosecond</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-nanoseconds-of-day">nanoseconds-of-day</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nanoseconds-of-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-second">second</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">second</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-sub-duration">sub-duration</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sub-duration</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a></code></li><li><a class="syn-name" href="#v-time">time</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a></code></li><li><a class="syn-name" href="#v-to-epoch-nanos">to-epoch-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-epoch-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-to-string">to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Time">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Time</strong></code><span class="doc-sig-links"><a href="#t-Time" title="Permalink">#</a><a href="/packages/stdlib/Data.Time/source/#L95" title="View source at line 95">Source</a></span></div>
<div class="doc-body">

The civil date-time: the date and the nanoseconds since its
midnight, below one day. Not exported (the invariant above);
`time` is the only constructor.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Time"><code><strong class="tok-constructor">Time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time/#instances">Data.Time</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time/#instances">Data.Time</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-TimeParseError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TimeParseError</strong></code><span class="doc-sig-links"><a href="#t-TimeParseError" title="Permalink">#</a><a href="/packages/stdlib/Data.Time/source/#L102" title="View source at line 102">Source</a></span></div>
<div class="doc-body">

The literal reader's errors, one per refusal: the text is not a
date-time, its time fields are out of range, or its date half
failed its own reader (carried through unchanged, so the date
diagnostics survive the composition).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-NotATime"><code><strong class="tok-constructor">NotATime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-TimeParseError" title="Data.Time.TimeParseError">TimeParseError</a></code></li><li id="k-TimeFieldsOutOfRange"><code><strong class="tok-constructor">TimeFieldsOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-TimeParseError" title="Data.Time.TimeParseError">TimeParseError</a></code></li><li id="k-BadDatePart"><code><strong class="tok-constructor">BadDatePart</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-DateParseError" title="Data.Date.DateParseError">DateParseError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-TimeParseError" title="Data.Time.TimeParseError">TimeParseError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-add-duration">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add-duration</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a></code><span class="doc-sig-links"><a href="#v-add-duration" title="Permalink">#</a><a href="/packages/stdlib/Data.Time/source/#L167" title="View source at line 167">Source</a></span></div>
<div class="doc-body">

Move forward by `d` (a whole number of milliseconds; the day
carries).

</div>
</article>

<article class="doc-entry" id="v-cmp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-cmp" title="Permalink">#</a><a href="/packages/stdlib/Data.Time/source/#L190" title="View source at line 190">Source</a></span></div>
</article>

<article class="doc-entry" id="v-date-part">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">date-part</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-date-part" title="Permalink">#</a><a href="/packages/stdlib/Data.Time/source/#L126" title="View source at line 126">Source</a></span></div>
</article>

<article class="doc-entry" id="v-diff">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">diff</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-diff" title="Permalink">#</a><a href="/packages/stdlib/Data.Time/source/#L183" title="View source at line 183">Source</a></span></div>
<div class="doc-body">

The distance between two instants in whole milliseconds (truncated
toward zero): unsigned, because `Duration` is, so the caller reads
direction from `cmp` (the refusal above).

</div>
</article>

<article class="doc-entry" id="v-from-epoch-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-epoch-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a></code><span class="doc-sig-links"><a href="#v-from-epoch-nanos" title="Permalink">#</a><a href="/packages/stdlib/Data.Time/source/#L159" title="View source at line 159">Source</a></span></div>
<div class="doc-body">

The inverse: every `Int` is an instant, so the result is total and
the floored split keeps the day's remainder non-negative for the
pre-epoch instants too.

</div>
</article>

<article class="doc-entry" id="v-from-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-TimeParseError" title="Data.Time.TimeParseError">TimeParseError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a></code><span class="doc-sig-links"><a href="#v-from-string" title="Permalink">#</a><a href="/packages/stdlib/Data.Time/source/#L215" title="View source at line 215">Source</a></span></div>
<div class="doc-body">

Strict ISO 8601 date-time: the date half (Data.Date's reader, so
its `NotADate`/`InvalidDate` survive as `BadDatePart`), `T`, then
`HH:MM:SS` with all fields two digits, an optional `.` and 1..9
fraction digits, and an optional trailing `Z`, with nothing before
or after. `NotATime` is the shape refusal (a space separator, no
seconds, an empty fraction, junk after `Z`); `TimeFieldsOutOfRange`
is the fields refusal (`24:00:00`, `00:60:00`).

</div>
</article>

<article class="doc-entry" id="v-hour">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hour</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-hour" title="Permalink">#</a><a href="/packages/stdlib/Data.Time/source/#L134" title="View source at line 134">Source</a></span></div>
</article>

<article class="doc-entry" id="v-minute">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">minute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-minute" title="Permalink">#</a><a href="/packages/stdlib/Data.Time/source/#L137" title="View source at line 137">Source</a></span></div>
</article>

<article class="doc-entry" id="v-nanosecond">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nanosecond</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-nanosecond" title="Permalink">#</a><a href="/packages/stdlib/Data.Time/source/#L143" title="View source at line 143">Source</a></span></div>
</article>

<article class="doc-entry" id="v-nanoseconds-of-day">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nanoseconds-of-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-nanoseconds-of-day" title="Permalink">#</a><a href="/packages/stdlib/Data.Time/source/#L130" title="View source at line 130">Source</a></span></div>
</article>

<article class="doc-entry" id="v-second">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">second</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-second" title="Permalink">#</a><a href="/packages/stdlib/Data.Time/source/#L140" title="View source at line 140">Source</a></span></div>
</article>

<article class="doc-entry" id="v-sub-duration">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sub-duration</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a></code><span class="doc-sig-links"><a href="#v-sub-duration" title="Permalink">#</a><a href="/packages/stdlib/Data.Time/source/#L171" title="View source at line 171">Source</a></span></div>
<div class="doc-body">

Move backward by `d`.

</div>
</article>

<article class="doc-entry" id="v-time">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Date/#t-Date" title="Data.Date.Date">Date</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a></code><span class="doc-sig-links"><a href="#v-time" title="Permalink">#</a><a href="/packages/stdlib/Data.Time/source/#L123" title="View source at line 123">Source</a></span></div>
<div class="doc-body">

The only constructor: a nanosecond count inside the day. At or past
the day's end is `None`, so an out-of-range `Time` has no
representation.

</div>
</article>

<article class="doc-entry" id="v-to-epoch-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-epoch-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-to-epoch-nanos" title="Permalink">#</a><a href="/packages/stdlib/Data.Time/source/#L152" title="View source at line 152">Source</a></span></div>
<div class="doc-body">

Nanoseconds since 1970-01-01T00:00:00Z (negative before the epoch).

</div>
</article>

<article class="doc-entry" id="v-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-to-string" title="Permalink">#</a><a href="/packages/stdlib/Data.Time/source/#L204" title="View source at line 204">Source</a></span></div>
<div class="doc-body">

`YYYY-MM-DDTHH:MM:SS` (the date half is Data.Date's rendering), with
`.fffffffff` appended when the sub-second field is non-zero. No
offset designator is emitted: the value is UTC by construction, and
the reader accepts a trailing `Z` (RFC 3339's UTC designator) as
well as its absence.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_Time"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Eq_Time</code></span><div class="doc-child-prose">Instances (the class spellings of cmp; instances live with their type per design/base-library.md).</div></li>
<li id="i-Ord_Time"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time/#t-Time" title="Data.Time.Time">Time</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Ord_Time</code></span></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Time.cinch`](/packages/stdlib/Data.Time/source/). Source SHA-256: `eaebe31539cf831410efab83a69447edab72cad03b00a27d4b515ddd4d54761c`.
