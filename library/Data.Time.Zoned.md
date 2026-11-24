# Data.Time.Zoned

Instants paired with a time zone, with civil field access.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Time.Zoned</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Time.Zoned/source/">View source</a><a href="/packages/search/?q=%2BData.Time.Zoned">Search module</a><a href="/reference-sources/Data.Time.Zoned.cinch" download="Data.Time.Zoned.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Time.Date](Data.Time.Date.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Offset](Data.Time.Offset.md), [Data.Time.TimeOfDay](Data.Time.TimeOfDay.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Data.Time.Weekday](Data.Time.Weekday.md), [Data.Time.Zone](Data.Time.Zone.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis"><summary>Synopsis (27 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Zoned">Zoned</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Zoned</strong></code></li><li><a class="syn-name" href="#v-zoned-abbreviation">zoned-abbreviation</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-abbreviation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-zoned-cmp">zoned-cmp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-zoned-date">zoned-date</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code></li><li><a class="syn-name" href="#v-zoned-datetime">zoned-datetime</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-datetime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-zoned-day">zoned-day</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 31 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-zoned-from-datetime">zoned-from-datetime</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-from-datetime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-from-datetime-compatible">zoned-from-datetime-compatible</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-from-datetime-compatible</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-from-datetime-earlier">zoned-from-datetime-earlier</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-from-datetime-earlier</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-from-datetime-later">zoned-from-datetime-later</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-from-datetime-later</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-from-datetime-reject">zoned-from-datetime-reject</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-from-datetime-reject</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-from-timestamp">zoned-from-timestamp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-from-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-hour">zoned-hour</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-hour</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 23 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-zoned-info">zoned-info</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a></code></li><li><a class="syn-name" href="#v-zoned-is-dst">zoned-is-dst</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-is-dst</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-zoned-minute">zoned-minute</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-minute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-zoned-month">zoned-month</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 12 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-zoned-offset">zoned-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code></li><li><a class="syn-name" href="#v-zoned-second">zoned-second</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-second</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-zoned-start-of-day">zoned-start-of-day</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-start-of-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-subsec-nanosecond">zoned-subsec-nanosecond</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-subsec-nanosecond</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999999999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-zoned-time-of-day">zoned-time-of-day</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-time-of-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code></li><li><a class="syn-name" href="#v-zoned-time-zone">zoned-time-zone</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-time-zone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code></li><li><a class="syn-name" href="#v-zoned-timestamp">zoned-timestamp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-zoned-weekday">zoned-weekday</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-weekday</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code></li><li><a class="syn-name" href="#v-zoned-with-time-zone">zoned-with-time-zone</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-with-time-zone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code></li><li><a class="syn-name" href="#v-zoned-year">zoned-year</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zoned-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> - 9999 9999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Zoned">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Zoned</strong></code><span class="doc-sig-links"><a href="#t-Zoned" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L77" title="View source at line 77">Source</a></span></div>
<div class="doc-body">

Instant, zone, cached offset and civil reading (the invariant in
the header). Not exported.

<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Zoned/#instances">Data.Time.Zoned</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-zoned-abbreviation">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-abbreviation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-zoned-abbreviation" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L141" title="View source at line 141">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-cmp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-zoned-cmp" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L221" title="View source at line 221">Source</a></span></div>
<div class="doc-body">

The order of the instants (the zones are not consulted).

</div>
</article>

<article class="doc-entry" id="v-zoned-date">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-date</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code><span class="doc-sig-links"><a href="#v-zoned-date" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L144" title="View source at line 144">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-datetime">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-datetime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-zoned-datetime" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L129" title="View source at line 129">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-day">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 31 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-zoned-day" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L156" title="View source at line 156">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-from-datetime">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-from-datetime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-Disambiguation" title="Data.Time.Zone.Types.Disambiguation">Disambiguation</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-from-datetime" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L96" title="View source at line 96">Source</a></span></div>
<div class="doc-body">

A civil reading in a zone, by the given strategy. `Err` when
`Reject` meets a gap or a fold, or the instant is outside the
timestamp range. The result is rebuilt from the instant, so the
invariant holds whatever the reading was (in a gap the cached
reading is the shifted one, not the argument).

</div>
</article>

<article class="doc-entry" id="v-zoned-from-datetime-compatible">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-from-datetime-compatible</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-from-datetime-compatible" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L101" title="View source at line 101">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-from-datetime-earlier">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-from-datetime-earlier</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-from-datetime-earlier" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L104" title="View source at line 104">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-from-datetime-later">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-from-datetime-later</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-from-datetime-later" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L107" title="View source at line 107">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-from-datetime-reject">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-from-datetime-reject</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-AmbiguityError" title="Data.Time.Zone.Types.AmbiguityError">AmbiguityError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-from-datetime-reject" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L110" title="View source at line 110">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-from-timestamp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-from-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-from-timestamp" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L86" title="View source at line 86">Source</a></span></div>
<div class="doc-body">

An instant read in a zone. Total: every instant has one offset in
every zone and a civil reading at every offset.

</div>
</article>

<article class="doc-entry" id="v-zoned-hour">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-hour</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 23 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-zoned-hour" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L162" title="View source at line 162">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-info">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-OffsetInfo" title="Data.Time.Zone.Types.OffsetInfo">OffsetInfo</a></code><span class="doc-sig-links"><a href="#v-zoned-info" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L135" title="View source at line 135">Source</a></span></div>
<div class="doc-body">

Offset, daylight flag and abbreviation at the instant (looked up;
only the offset is cached).

</div>
</article>

<article class="doc-entry" id="v-zoned-is-dst">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-is-dst</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-zoned-is-dst" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L138" title="View source at line 138">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-minute">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-minute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-zoned-minute" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L165" title="View source at line 165">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-month">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 12 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-zoned-month" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L153" title="View source at line 153">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code><span class="doc-sig-links"><a href="#v-zoned-offset" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L125" title="View source at line 125">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-second">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-second</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-zoned-second" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L168" title="View source at line 168">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-start-of-day">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-start-of-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-start-of-day" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L196" title="View source at line 196">Source</a></span></div>
<div class="doc-body">

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

</div>
</article>

<article class="doc-entry" id="v-zoned-subsec-nanosecond">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-subsec-nanosecond</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999999999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-zoned-subsec-nanosecond" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L171" title="View source at line 171">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-time-of-day">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-time-of-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code><span class="doc-sig-links"><a href="#v-zoned-time-of-day" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L147" title="View source at line 147">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-time-zone">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-time-zone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a></code><span class="doc-sig-links"><a href="#v-zoned-time-zone" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L121" title="View source at line 121">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-timestamp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-zoned-timestamp" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L117" title="View source at line 117">Source</a></span></div>
<div class="doc-body">

Parts. Each is a field read or a field of the cached reading.

</div>
</article>

<article class="doc-entry" id="v-zoned-weekday">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-weekday</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code><span class="doc-sig-links"><a href="#v-zoned-weekday" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L159" title="View source at line 159">Source</a></span></div>
</article>

<article class="doc-entry" id="v-zoned-with-time-zone">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-with-time-zone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zone/#t-TimeZone" title="Data.Time.Zone.TimeZone">TimeZone</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code><span class="doc-sig-links"><a href="#v-zoned-with-time-zone" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L179" title="View source at line 179">Source</a></span></div>
<div class="doc-body">

The same instant read in another zone.

</div>
</article>

<article class="doc-entry" id="v-zoned-year">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zoned-year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> - 9999 9999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-zoned-year" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Zoned/source/#L150" title="View source at line 150">Source</a></span></div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_Zoned"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Zoned.Eq_Zoned</code></span><div class="doc-child-prose">The same instant in a zone of the same name.</div></li>
</ul>
