# Data.Time.Timestamp

Instants on the Unix time line with nanosecond precision and checked arithmetic.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Time.Timestamp</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Time.Timestamp/source/">View source</a><a href="/packages/search/?q=%2BData.Time.Timestamp">Search module</a><a href="/reference-sources/Data.Time.Timestamp.cinch" download="Data.Time.Timestamp.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Time.Duration](Data.Time.Duration.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Time.Timestamp: an instant on the Unix time line
(design/datetime.md, layer L0 "machine time", lane A).

A `Timestamp` answers "when did that happen?": a count of
nanoseconds since 1970-01-01T00:00:00Z, with no leap seconds (Unix
time, as in Jiff and Temporal). It has no calendar fields and no
zone. Showing one to a person needs a `TimeZone`
(`Data.Time.Offset`, `Data.Time.Zone`), and that conversion is the
only road from machine time to civil time. The representation
constructor is not exported, so an out-of-range instant has no
representation.

REPRESENTATION. One `I128` nanosecond count behind the private
constructor `MkTimestamp` (the same shape and the same one-allocation
wrapper cost as `Data.Time.Duration`; see its header).

RANGE (T3). `-9999-01-02T01:59:59Z ..= 9999-12-30T22:00:00.999999999Z`
(`min-timestamp`, `max-timestamp`), Jiff's bounds. Derivation:
  - civil years are -9999..=9999 and offsets are within 25:59:59 of
    UTC (93599 seconds), so the range is the civil range pulled in
    by one maximal offset at each end; then every timestamp has a
    civil reading in every offset.
  - -9999-01-01 is epoch day -4371587: 0001-01-01 is day -719162,
    and the 10000 years -9999..=0 are 25 Gregorian cycles of 146097
    days (3652425 days) before it. In seconds, -377705116800; plus
    93599 is -377705023201; in nanoseconds
    -377705023201000000000.
  - 9999-12-31 is epoch day 2932896, so 9999-12-31T23:59:59 is
    253402300799 seconds; less 93599 is 253402207200
    (9999-12-30T22:00:00); the last nanosecond of that second is
    253402207200999999999.
Both bounds are past `I64` nanoseconds (about +-9.22e18), which is
why the representation is wide and why `timestamp-from-unix-nanos`
on an `I64` is total.

Design law shape. The nanosecond count is the value: equality and
`timestamp-cmp` are the integer order, which is the order of the
instants. The Unix conversions floor: `to-unix-seconds` is the
greatest whole second not after the instant and `subsec-nanos` is
the rest, always `0..=999999999`, so 1 ns before the epoch is second
-1 with 999999999 ns, and `from-unix-seconds-and-nanos` inverts
`to-unix-seconds-and-nanos` exactly. `timestamp-since a b` is
`a - b` and `timestamp-until a b` is `b - a`, so each is the other's
negation; `checked-add t (since u t)` is `u`; `checked-add` and
`checked-sub` of the same duration undo each other whenever both
exist. A `checked-` result is `Some` exactly when the true result is
in the range; the `saturating-` twin clamps to the bounds. The
checks are seed/tests/lib_time_a.rs.

NAMED REFUSALS (boundaries this module deliberately does not
cross):
  - instants outside the range above: construction and arithmetic
    answer `None` (or clamp, in the `saturating-` spellings).
  - leap seconds: not modelled (Unix time).
  - calendar fields, formatting and parsing: `Data.Time.Offset`,
    `Data.Time.Zone` and `Text.Time.Iso` own those.
  - reading the clock: `Effect.Time.now`.
  - operator instances for arithmetic (T2); only `Eq` and `Ord`.
  - `Int`/`Nat` (bignums): neither appears.
  - an allocation-free wrapper: as `Data.Time.Duration`.
  - a refined component in the pair `to-unix-seconds-and-nanos`
    returns: a tuple whose component type is refined accepts only a
    literal there ("type mismatch: expected I64, got InRange 0
    999999999 I64" for any computed value), so the pair is
    `(I64, I64)` and the bound is carried by `timestamp-subsec-nanos`.

Naming: every export is prefixed `timestamp-` (the bounds are
`min-timestamp`/`max-timestamp`); see `Data.Time.Duration`'s header.

</details>

<details class="doc-synopsis"><summary>Synopsis (24 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Timestamp">Timestamp</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Timestamp</strong></code></li><li><a class="syn-name" href="#v-max-timestamp">max-timestamp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">max-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-min-timestamp">min-timestamp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">min-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-timestamp-checked-add">timestamp-checked-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-checked-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-timestamp-checked-sub">timestamp-checked-sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-checked-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-timestamp-cmp">timestamp-cmp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-timestamp-epoch">timestamp-epoch</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-epoch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-timestamp-from-unix-micros">timestamp-from-unix-micros</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-from-unix-micros</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-timestamp-from-unix-millis">timestamp-from-unix-millis</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-from-unix-millis</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-timestamp-from-unix-nanos">timestamp-from-unix-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-from-unix-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-timestamp-from-unix-nanos-i128">timestamp-from-unix-nanos-i128</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-from-unix-nanos-i128</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-timestamp-from-unix-seconds">timestamp-from-unix-seconds</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-from-unix-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-timestamp-from-unix-seconds-and-nanos">timestamp-from-unix-seconds-and-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-from-unix-seconds-and-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-timestamp-saturating-add">timestamp-saturating-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-saturating-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-timestamp-saturating-sub">timestamp-saturating-sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-saturating-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-timestamp-since">timestamp-since</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-since</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-timestamp-subsec-nanos">timestamp-subsec-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-subsec-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999999999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-timestamp-to-unix-micros">timestamp-to-unix-micros</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-to-unix-micros</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-timestamp-to-unix-millis">timestamp-to-unix-millis</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-to-unix-millis</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-timestamp-to-unix-nanos">timestamp-to-unix-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-to-unix-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a></code></li><li><a class="syn-name" href="#v-timestamp-to-unix-nanos-i64">timestamp-to-unix-nanos-i64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-to-unix-nanos-i64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-timestamp-to-unix-seconds">timestamp-to-unix-seconds</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-to-unix-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-timestamp-to-unix-seconds-and-nanos">timestamp-to-unix-seconds-and-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-to-unix-seconds-and-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code></li><li><a class="syn-name" href="#v-timestamp-until">timestamp-until</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timestamp-until</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Timestamp">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Timestamp</strong></code><span class="doc-sig-links"><a href="#t-Timestamp" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L98" title="View source at line 98">Source</a></span></div>
<div class="doc-body">

Nanoseconds since the Unix epoch. Not exported (the range
invariant above); named apart from the type so that
`export Timestamp` does not export it.

<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Timestamp/#instances">Data.Time.Timestamp</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Timestamp/#instances">Data.Time.Timestamp</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-max-timestamp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">max-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-max-timestamp" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L164" title="View source at line 164">Source</a></span></div>
<div class="doc-body">

9999-12-30T22:00:00.999999999Z.

</div>
</article>

<article class="doc-entry" id="v-min-timestamp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">min-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-min-timestamp" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L160" title="View source at line 160">Source</a></span></div>
<div class="doc-body">

-9999-01-02T01:59:59Z.

</div>
</article>

<article class="doc-entry" id="v-timestamp-checked-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-checked-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-timestamp-checked-add" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L241" title="View source at line 241">Source</a></span></div>
<div class="doc-body">

The instant `d` after `t` (before it for a negative `d`).

</div>
</article>

<article class="doc-entry" id="v-timestamp-checked-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-checked-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-timestamp-checked-sub" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L247" title="View source at line 247">Source</a></span></div>
<div class="doc-body">

The instant `d` before `t`.

</div>
</article>

<article class="doc-entry" id="v-timestamp-cmp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-timestamp-cmp" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L274" title="View source at line 274">Source</a></span></div>
<div class="doc-body">

Order.

</div>
</article>

<article class="doc-entry" id="v-timestamp-epoch">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-epoch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-timestamp-epoch" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L156" title="View source at line 156">Source</a></span></div>
<div class="doc-body">

1970-01-01T00:00:00Z.

</div>
</article>

<article class="doc-entry" id="v-timestamp-from-unix-micros">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-from-unix-micros</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-timestamp-from-unix-micros" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L174" title="View source at line 174">Source</a></span></div>
</article>

<article class="doc-entry" id="v-timestamp-from-unix-millis">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-from-unix-millis</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-timestamp-from-unix-millis" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L171" title="View source at line 171">Source</a></span></div>
</article>

<article class="doc-entry" id="v-timestamp-from-unix-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-from-unix-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-timestamp-from-unix-nanos" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L179" title="View source at line 179">Source</a></span></div>
<div class="doc-body">

Total: every `I64` nanosecond count (1677-09-21 to 2262-04-11) is
inside the range, whose bounds are past `I64` on both sides.

</div>
</article>

<article class="doc-entry" id="v-timestamp-from-unix-nanos-i128">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-from-unix-nanos-i128</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-timestamp-from-unix-nanos-i128" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L182" title="View source at line 182">Source</a></span></div>
</article>

<article class="doc-entry" id="v-timestamp-from-unix-seconds">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-from-unix-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-timestamp-from-unix-seconds" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L168" title="View source at line 168">Source</a></span></div>
<div class="doc-body">

`None` outside -377705023201..=253402207200.

</div>
</article>

<article class="doc-entry" id="v-timestamp-from-unix-seconds-and-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-from-unix-seconds-and-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-timestamp-from-unix-seconds-and-nanos" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L190" title="View source at line 190">Source</a></span></div>
<div class="doc-body">

`secs` whole seconds plus `nanos` nanoseconds. The two are added,
so any `I64` nanosecond part is accepted and normalised (the pair
`(-1, 999999999)` is 1 ns before the epoch, the pair
`to-unix-seconds-and-nanos` gives for it). The wrapping sum is
exact: the seconds term is below 2^93 and the other below 2^63.

</div>
</article>

<article class="doc-entry" id="v-timestamp-saturating-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-saturating-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-timestamp-saturating-add" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L253" title="View source at line 253">Source</a></span></div>
<div class="doc-body">

The same, clamped to `min-timestamp`/`max-timestamp`.

</div>
</article>

<article class="doc-entry" id="v-timestamp-saturating-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-saturating-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-timestamp-saturating-sub" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L256" title="View source at line 256">Source</a></span></div>
</article>

<article class="doc-entry" id="v-timestamp-since">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-since</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-timestamp-since" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L263" title="View source at line 263">Source</a></span></div>
<div class="doc-body">

`timestamp-since a b` is `a - b`: the time from `b` to `a`,
positive when `a` is the later. Total: two counts in the range
differ by at most 631107230401999999999 (below 2^70), so the
wrapping subtraction is exact and far inside a duration's range.

</div>
</article>

<article class="doc-entry" id="v-timestamp-subsec-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-subsec-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 999999999 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-timestamp-subsec-nanos" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L222" title="View source at line 222">Source</a></span></div>
<div class="doc-body">

The nanoseconds past the floored second: `0..=999999999`. The
value is `n - floor(n / 10^9) * 10^9`, which is in that interval,
so both wrapping steps are exact; the guard restates the bound for
the checker (a wide scalar has no solver sort) and its `else` arm
is unreachable.

</div>
</article>

<article class="doc-entry" id="v-timestamp-to-unix-micros">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-to-unix-micros</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-timestamp-to-unix-micros" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L205" title="View source at line 205">Source</a></span></div>
</article>

<article class="doc-entry" id="v-timestamp-to-unix-millis">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-to-unix-millis</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-timestamp-to-unix-millis" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L202" title="View source at line 202">Source</a></span></div>
</article>

<article class="doc-entry" id="v-timestamp-to-unix-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-to-unix-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a></code><span class="doc-sig-links"><a href="#v-timestamp-to-unix-nanos" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L209" title="View source at line 209">Source</a></span></div>
<div class="doc-body">

The whole count, exactly.

</div>
</article>

<article class="doc-entry" id="v-timestamp-to-unix-nanos-i64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-to-unix-nanos-i64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-timestamp-to-unix-nanos-i64" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L214" title="View source at line 214">Source</a></span></div>
<div class="doc-body">

`None` outside `I64` nanoseconds (before 1677-09-21T00:12:43.145224192Z
or after 2262-04-11T23:47:16.854775807Z).

</div>
</article>

<article class="doc-entry" id="v-timestamp-to-unix-seconds">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-to-unix-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-timestamp-to-unix-seconds" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L199" title="View source at line 199">Source</a></span></div>
<div class="doc-body">

Observation. Every conversion to a coarser unit floors (rounds
toward the past), before the epoch as after it.

</div>
</article>

<article class="doc-entry" id="v-timestamp-to-unix-seconds-and-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-to-unix-seconds-and-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>)</code><span class="doc-sig-links"><a href="#v-timestamp-to-unix-seconds-and-nanos" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L233" title="View source at line 233">Source</a></span></div>
<div class="doc-body">

The floored second and the nanoseconds past it (`0..=999999999`),
together: what the zone layer splits an instant into. The second
component is a plain `I64` here (named refusal in the header);
`timestamp-subsec-nanos` is the same number with its bound in the
type.

</div>
</article>

<article class="doc-entry" id="v-timestamp-until">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timestamp-until</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-timestamp-until" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Timestamp/source/#L267" title="View source at line 267">Source</a></span></div>
<div class="doc-body">

`timestamp-until a b` is `b - a`: the time from `a` to `b`.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_Timestamp"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Timestamp.Eq_Timestamp</code></span></li>
<li id="i-Ord_Timestamp"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Timestamp.Ord_Timestamp</code></span></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Time/Timestamp.cinch`](/packages/stdlib/Data.Time.Timestamp/source/). Source SHA-256: `c4ac068fadce1317eb55fe3e867e7c85e2e98449390bd95c01eb7c7982380614`.
