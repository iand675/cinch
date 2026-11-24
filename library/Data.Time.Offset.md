# Data.Time.Offset

Fixed offsets from UTC with validated ranges.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Time.Offset</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Time.Offset/source/">View source</a><a href="/packages/search/?q=%2BData.Time.Offset">Search module</a><a href="/reference-sources/Data.Time.Offset.cinch" download="Data.Time.Offset.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Time.DateTime](Data.Time.DateTime.md), [Data.Time.Timestamp](Data.Time.Timestamp.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Time.Offset: a fixed distance from UTC
(design/datetime.md, layer L2 "zone", lane C).

An `Offset` is a whole number of seconds east of UTC: the amount
added to an instant's UTC reading to get the civil reading in some
place at that instant. It is the smallest piece of the zone layer:
a `TimeZone` (`Data.Time.Zone`) is a rule that picks an `Offset`
for each instant, and the two conversions here (instant to civil
reading at an offset, and back) are the arithmetic every zone
conversion is built from.

REPRESENTATION. One `I64` (seconds east) behind the private
constructor `MkOffset`, always within -93599..=93599 (25:59:59,
T3, Jiff's bound). Boxed today like every one-field `data` (see
`Data.Time.Duration`'s header).

Design law shape. `offset-from-seconds (offset-to-seconds o)` is
`Some o`; the order and equality are those of the seconds.
`offset-negate` is an involution (the range is symmetric).
`offset-hours`, `offset-minutes` and `offset-seconds` are the
magnitude's components and `offset-is-negative` its sign, so
`|seconds| = hours * 3600 + minutes * 60 + seconds`.
`offset-timestamp-to-datetime o t` is the civil reading of `t` at
`o`: the "as if UTC" civil line position `unix seconds + o`. It is
total because the timestamp range was chosen (T3) so that every
instant has a civil reading in years -9999..=9999 at every offset.
`offset-datetime-to-timestamp o x` is its inverse: `Some t`
exactly when the instant `x - o` is inside the timestamp range, and
then `offset-timestamp-to-datetime o t` is `x`.

NAMED REFUSALS:
  - offsets beyond 25:59:59 either way: `offset-from-seconds`
    answers `None`; the refined constructors do not admit them.
  - sub-second offsets (some historical local mean times carry
    fractions in the source data; TZif stores whole seconds).
  - a faulting or silently clamping conversion: the unreachable
    `None` in `offset-timestamp-to-datetime` (the civil module
    cannot know the timestamp bound) reads as the nearest end of the
    civil range, `min-datetime` or `max-datetime`; no input reaches
    it (see the function).
  - text (`+05:30`): `Text.Time.Iso` owns it.
  - a refined `InRange` result type on the component accessors is
    kept; the values are produced by a clamp that never fires
    (machine arithmetic is opaque to the refinement checker).

</details>

<details class="doc-synopsis" open><summary>Synopsis (18 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Offset">Offset</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Offset</strong></code></li><li><a class="syn-name" href="#v-max-offset">max-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">max-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code></li><li><a class="syn-name" href="#v-min-offset">min-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">min-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code></li><li><a class="syn-name" href="#v-offset-cmp">offset-cmp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-offset-datetime-to-timestamp">offset-datetime-to-timestamp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-datetime-to-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code></li><li><a class="syn-name" href="#v-offset-from-hms">offset-from-hms</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-from-hms</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code></li><li><a class="syn-name" href="#v-offset-from-seconds">offset-from-seconds</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-from-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code></li><li><a class="syn-name" href="#v-offset-hours">offset-hours</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-hours</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 25 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-offset-is-negative">offset-is-negative</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-is-negative</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-offset-is-utc">offset-is-utc</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-is-utc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-offset-minutes">offset-minutes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-minutes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-offset-negate">offset-negate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-negate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code></li><li><a class="syn-name" href="#v-offset-of-hours">offset-of-hours</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-of-hours</strong> :: (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> - 25 25 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code></li><li><a class="syn-name" href="#v-offset-of-seconds">offset-of-seconds</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-of-seconds</strong> :: (<span class="tok-variable">s</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> - 93599 93599 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code></li><li><a class="syn-name" href="#v-offset-seconds">offset-seconds</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-offset-timestamp-to-datetime">offset-timestamp-to-datetime</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-timestamp-to-datetime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code></li><li><a class="syn-name" href="#v-offset-to-seconds">offset-to-seconds</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-to-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-offset-utc">offset-utc</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">offset-utc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Offset">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Offset</strong></code><span class="doc-sig-links"><a href="#t-Offset" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Offset/source/#L67" title="View source at line 67">Source</a></span></div>
<div class="doc-body">

Seconds east of UTC, within -93599..=93599. Not exported.

<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Offset/#instances">Data.Time.Offset</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Offset/#instances">Data.Time.Offset</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-max-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">max-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code><span class="doc-sig-links"><a href="#v-max-offset" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Offset/source/#L82" title="View source at line 82">Source</a></span></div>
</article>

<article class="doc-entry" id="v-min-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">min-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code><span class="doc-sig-links"><a href="#v-min-offset" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Offset/source/#L79" title="View source at line 79">Source</a></span></div>
<div class="doc-body">

-25:59:59 and +25:59:59.

</div>
</article>

<article class="doc-entry" id="v-offset-cmp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">offset-cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-offset-cmp" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Offset/source/#L189" title="View source at line 189">Source</a></span></div>
<div class="doc-body">

Order: the seconds' order (further east is greater).

</div>
</article>

<article class="doc-entry" id="v-offset-datetime-to-timestamp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">offset-datetime-to-timestamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code><span class="doc-sig-links"><a href="#v-offset-datetime-to-timestamp" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Offset/source/#L180" title="View source at line 180">Source</a></span></div>
<div class="doc-body">

The instant whose civil reading at `o` is `x`: the civil line
position less the offset. `None` when that instant is outside the
timestamp range (only within 26 hours of the ends of the civil
range). The civil seconds are within +-3.8e11, so the subtraction
cannot wrap.

</div>
</article>

<article class="doc-entry" id="v-offset-from-hms">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">offset-from-hms</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code><span class="doc-sig-links"><a href="#v-offset-from-hms" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Offset/source/#L104" title="View source at line 104">Source</a></span></div>
<div class="doc-body">

Sign and magnitude components: `negative` then hours, minutes and
seconds. `None` unless hours is 0..=25 and minutes and seconds are
0..=59. The sum is at most 25 * 3600 + 59 * 60 + 59 = 93599.

</div>
</article>

<article class="doc-entry" id="v-offset-from-seconds">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">offset-from-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code><span class="doc-sig-links"><a href="#v-offset-from-seconds" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Offset/source/#L86" title="View source at line 86">Source</a></span></div>
<div class="doc-body">

`None` outside -93599..=93599.

</div>
</article>

<article class="doc-entry" id="v-offset-hours">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">offset-hours</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 25 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-offset-hours" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Offset/source/#L141" title="View source at line 141">Source</a></span></div>
<div class="doc-body">

The magnitude's whole hours, minutes of the hour and seconds of
the minute. Each clamp below never fires (the magnitude is at most
93599); it exists because machine arithmetic is opaque to the
refinement checker.

</div>
</article>

<article class="doc-entry" id="v-offset-is-negative">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">offset-is-negative</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-offset-is-negative" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Offset/source/#L125" title="View source at line 125">Source</a></span></div>
<div class="doc-body">

West of UTC.

</div>
</article>

<article class="doc-entry" id="v-offset-is-utc">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">offset-is-utc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-offset-is-utc" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Offset/source/#L128" title="View source at line 128">Source</a></span></div>
</article>

<article class="doc-entry" id="v-offset-minutes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">offset-minutes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-offset-minutes" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Offset/source/#L146" title="View source at line 146">Source</a></span></div>
</article>

<article class="doc-entry" id="v-offset-negate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">offset-negate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code><span class="doc-sig-links"><a href="#v-offset-negate" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Offset/source/#L121" title="View source at line 121">Source</a></span></div>
<div class="doc-body">

The opposite offset. The range is symmetric, so this is total and
the negation cannot wrap.

</div>
</article>

<article class="doc-entry" id="v-offset-of-hours">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">offset-of-hours</strong> :: (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> - 25 25 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code><span class="doc-sig-links"><a href="#v-offset-of-hours" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Offset/source/#L98" title="View source at line 98">Source</a></span></div>
<div class="doc-body">

Whole hours. 25 * 3600 = 90000, inside the range, so the wrapping
product is exact and in range.

</div>
</article>

<article class="doc-entry" id="v-offset-of-seconds">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">offset-of-seconds</strong> :: (<span class="tok-variable">s</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> - 93599 93599 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code><span class="doc-sig-links"><a href="#v-offset-of-seconds" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Offset/source/#L93" title="View source at line 93">Source</a></span></div>
<div class="doc-body">

Total over the refined range, so a literal needs no `Option`.

</div>
</article>

<article class="doc-entry" id="v-offset-seconds">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">offset-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 59 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-offset-seconds" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Offset/source/#L151" title="View source at line 151">Source</a></span></div>
</article>

<article class="doc-entry" id="v-offset-timestamp-to-datetime">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">offset-timestamp-to-datetime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code><span class="doc-sig-links"><a href="#v-offset-timestamp-to-datetime" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Offset/source/#L167" title="View source at line 167">Source</a></span></div>
<div class="doc-body">

The civil reading of `t` at `o`. The Unix seconds are within
+-3.8e11 and the offset within +-93599, so the sum cannot wrap.
The timestamp range (T3) is the civil range pulled in by 93599
seconds at each end, so the sum always names a second of a year in
-9999..=9999 and the `None` arm is unreachable; if it were ever
taken (a change to either range), the answer is the nearest end of
the civil range rather than a fault.

</div>
</article>

<article class="doc-entry" id="v-offset-to-seconds">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">offset-to-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-offset-to-seconds" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Offset/source/#L115" title="View source at line 115">Source</a></span></div>
<div class="doc-body">

Reading.

</div>
</article>

<article class="doc-entry" id="v-offset-utc">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">offset-utc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code><span class="doc-sig-links"><a href="#v-offset-utc" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Offset/source/#L75" title="View source at line 75">Source</a></span></div>
<div class="doc-body">

UTC itself: zero seconds.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_Offset"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Offset.Eq_Offset</code></span></li>
<li id="i-Ord_Offset"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Time.Offset.Ord_Offset</code></span></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Time/Offset.cinch`](/packages/stdlib/Data.Time.Offset/source/). Source SHA-256: `74af282609866c44d75df7392c48eb71cafd85f5e1ee6edb2af30bb4a109cc2c`.
