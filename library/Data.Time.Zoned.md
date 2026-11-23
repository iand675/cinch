# Data.Time.Zoned

Instants paired with a time zone, with civil field access.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Time.Zoned`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Time.Zoned.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone](Data.Time.Zone.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone](Data.Time.Zone.md), [Data.Time.Zone.Posix](Data.Time.Zone.Posix.md), [Data.Time.Zone.Types](Data.Time.Zone.Types.md), [Data.Time.Zoned](Data.Time.Zoned.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Time.Zoned: an instant in a place
(design/datetime.md, layer L2, lane C).

A `Zoned` is a `Timestamp` together with the `TimeZone` it is to be
read in, with that reading (the offset in force and the civil
date-time) computed once and kept. It is what "2026-11-01 17:30 in
New York" means: unlike a civil `DateTime` it names one instant,
and unlike a `Timestamp` it knows how to show itself and what
"tomorrow" is.

REPRESENTATION. Four fields behind the private constructor
`MkZoned`: the instant, the zone, and the cached `Offset` and
`DateTime` (section 4 of the design note). The constructor is not
exported, so the cache cannot disagree with the instant.

Design law shape. INVARIANT: for every `Zoned` value `x`,
`zoned-offset x` is `zone-offset-at (zoned-time-zone x)
(zoned-timestamp x)` and `zoned-datetime x` is the civil reading of
the instant at that offset. `zoned-from-timestamp` is total and
`zoned-timestamp (zoned-from-timestamp z t)` is `t`.
`zoned-from-datetime s z d` resolves the civil reading with the
strategy `s` (T4; `Data.Time.Zone.Types`' table): when it answers
`Ok x` and `d` is not in a gap, `zoned-datetime x` is `d`; in a gap
`d` does not exist and the result reads as the shifted time (see
the table). `zoned-with-time-zone` keeps the instant.
`zoned-start-of-day x` is the first instant whose civil date in the
zone is `x`'s: midnight when midnight exists, its first occurrence
when it occurs twice, and the instant the clocks jumped when the
day begins inside a gap (so 01:00 in America/Santiago on the day
daylight time begins). Equality is the instant AND the zone's name;
`zoned-cmp` orders by the instant alone.

NAMED REFUSALS:
  - a silent disambiguation (T4): `zoned-from-datetime` takes the
    strategy; the named forms carry it.
  - an `Ord` instance: the order is on instants and equality also
    looks at the zone, so `Ord` would call two values neither less
    nor greater yet unequal. `zoned-cmp` is the instant order.
  - equality of zones by their rules: names are compared (two
    lookups of one name agree; a zone and its alias do not).
  - arithmetic and rounding: `Data.Time.Zoned.Arithmetic` (lane F).
  - text: `Text.Time.Rfc9557` (lane F).
  - lookup of the zone by name, and "now": `Effect.Time.Zone`.

</details>

# Types and constructors

<a id="type-5a6f6e6564"></a>

## `Zoned`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Zoned</span></code></pre>

Instant, zone, cached offset and civil reading (the invariant in
the header). Not exported.

[Source](/reference-sources/Data.Time.Zoned.cinch) · line 77

# Values

<a id="value-7a6f6e65642d616262726576696174696f6e"></a>

## `zoned-abbreviation`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-abbreviation</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Data.Time.Zoned.cinch)

<a id="value-7a6f6e65642d636d70"></a>

## `zoned-cmp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-cmp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>))</code></pre>

The order of the instants (the zones are not consulted).

[Source](/reference-sources/Data.Time.Zoned.cinch) · line 221

<a id="value-7a6f6e65642d64617465"></a>

## `zoned-date`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-date</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Date</span>.<span class="tok-constructor">Date</span>)</code></pre>

[Source](/reference-sources/Data.Time.Zoned.cinch)

<a id="value-7a6f6e65642d6461746574696d65"></a>

## `zoned-datetime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-datetime</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span>)</code></pre>

[Source](/reference-sources/Data.Time.Zoned.cinch)

<a id="value-7a6f6e65642d646179"></a>

## `zoned-day`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-day</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">31</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.cinch)

<a id="value-7a6f6e65642d66726f6d2d6461746574696d65"></a>

## `zoned-from-datetime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-from-datetime</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">Disambiguation</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">AmbiguityError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))))</code></pre>

A civil reading in a zone, by the given strategy. `Err` when
`Reject` meets a gap or a fold, or the instant is outside the
timestamp range. The result is rebuilt from the instant, so the
invariant holds whatever the reading was (in a gap the cached
reading is the shifted one, not the argument).

[Source](/reference-sources/Data.Time.Zoned.cinch) · line 96

<a id="value-7a6f6e65642d66726f6d2d6461746574696d652d636f6d70617469626c65"></a>

## `zoned-from-datetime-compatible`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-from-datetime-compatible</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">AmbiguityError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.cinch)

<a id="value-7a6f6e65642d66726f6d2d6461746574696d652d6561726c696572"></a>

## `zoned-from-datetime-earlier`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-from-datetime-earlier</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">AmbiguityError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.cinch)

<a id="value-7a6f6e65642d66726f6d2d6461746574696d652d6c61746572"></a>

## `zoned-from-datetime-later`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-from-datetime-later</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">AmbiguityError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.cinch)

<a id="value-7a6f6e65642d66726f6d2d6461746574696d652d72656a656374"></a>

## `zoned-from-datetime-reject`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-from-datetime-reject</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">DateTime</span>.<span class="tok-constructor">DateTime</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">AmbiguityError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>)))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.cinch)

<a id="value-7a6f6e65642d66726f6d2d74696d657374616d70"></a>

## `zoned-from-timestamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-from-timestamp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))</code></pre>

An instant read in a zone. Total: every instant has one offset in
every zone and a civil reading at every offset.

[Source](/reference-sources/Data.Time.Zoned.cinch) · line 86

<a id="value-7a6f6e65642d686f7572"></a>

## `zoned-hour`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-hour</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">23</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.cinch)

<a id="value-7a6f6e65642d696e666f"></a>

## `zoned-info`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-info</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">Types</span>.<span class="tok-constructor">OffsetInfo</span>)</code></pre>

Offset, daylight flag and abbreviation at the instant (looked up;
only the offset is cached).

[Source](/reference-sources/Data.Time.Zoned.cinch) · line 135

<a id="value-7a6f6e65642d69732d647374"></a>

## `zoned-is-dst`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-is-dst</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Time.Zoned.cinch)

<a id="value-7a6f6e65642d6d696e757465"></a>

## `zoned-minute`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-minute</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">59</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.cinch)

<a id="value-7a6f6e65642d6d6f6e7468"></a>

## `zoned-month`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-month</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">1</span> <span class="tok-number">12</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.cinch)

<a id="value-7a6f6e65642d6f6666736574"></a>

## `zoned-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-offset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">Offset</span>)</code></pre>

[Source](/reference-sources/Data.Time.Zoned.cinch)

<a id="value-7a6f6e65642d7365636f6e64"></a>

## `zoned-second`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-second</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">59</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.cinch)

<a id="value-7a6f6e65642d73746172742d6f662d646179"></a>

## `zoned-start-of-day`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-start-of-day</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))</code></pre>

The first instant of `x`'s civil day in its zone (the header's
law). `None` only when that instant is before the timestamp
range (the first day of the range).

Midnight of the day is resolved: one occurrence is the answer; of
two (a fold over midnight) the earlier is the first instant of
the day. In a gap, midnight was skipped and the day begins at the
transition that skipped it: reading midnight with the offset
AFTER the gap gives an instant before that transition and less
than a gap's length from it, so the transition is the next one
after that instant. (Shifting midnight forward by the gap's
length, the "compatible" reading, is only right when the gap
begins exactly at midnight.) The fallback arm, a gap with no
transition after it, cannot occur and reads as "compatible".

[Source](/reference-sources/Data.Time.Zoned.cinch) · line 196

<a id="value-7a6f6e65642d7375627365632d6e616e6f7365636f6e64"></a>

## `zoned-subsec-nanosecond`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-subsec-nanosecond</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">999999999</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.cinch)

<a id="value-7a6f6e65642d74696d652d6f662d646179"></a>

## `zoned-time-of-day`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-time-of-day</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">TimeOfDay</span>.<span class="tok-constructor">TimeOfDay</span>)</code></pre>

[Source](/reference-sources/Data.Time.Zoned.cinch)

<a id="value-7a6f6e65642d74696d652d7a6f6e65"></a>

## `zoned-time-zone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-time-zone</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span>)</code></pre>

[Source](/reference-sources/Data.Time.Zoned.cinch)

<a id="value-7a6f6e65642d74696d657374616d70"></a>

## `zoned-timestamp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-timestamp</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Timestamp</span>.<span class="tok-constructor">Timestamp</span>)</code></pre>

Parts. Each is a field read or a field of the cached reading.

[Source](/reference-sources/Data.Time.Zoned.cinch) · line 117

<a id="value-7a6f6e65642d7765656b646179"></a>

## `zoned-weekday`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-weekday</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Weekday</span>.<span class="tok-constructor">Weekday</span>)</code></pre>

[Source](/reference-sources/Data.Time.Zoned.cinch)

<a id="value-7a6f6e65642d776974682d74696d652d7a6f6e65"></a>

## `zoned-with-time-zone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-with-time-zone</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zone</span>.<span class="tok-constructor">TimeZone</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span>))</code></pre>

The same instant read in another zone.

[Source](/reference-sources/Data.Time.Zoned.cinch) · line 179

<a id="value-7a6f6e65642d79656172"></a>

## `zoned-year`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zoned-year</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Time</span>.<span class="tok-constructor">Zoned</span>.<span class="tok-constructor">Zoned</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">-9999</span> <span class="tok-number">9999</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Time.Zoned.cinch)

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Time.Zoned.Eq_Zoned` | `Relation.Eq.Eq Data.Time.Zoned.Zoned` | Implicit |

**`Data.Time.Zoned.Eq_Zoned`**: The same instant in a zone of the same name.

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Time/Zoned.cinch`. Source SHA-256: `399a55deb46acb1139e9c71c041af7242622cb6f9d798114bdc0e04d561c1de0`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
