# Data.Time.Unit

Span units, rounding modes and the shared rounding kernel.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Time.Unit</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Time.Unit/source/">View source</a><a href="/packages/search/?q=%2BData.Time.Unit">Search module</a><a href="/reference-sources/Data.Time.Unit.cinch" download="Data.Time.Unit.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Time.Unit: the units of a span, the rounding modes and the
rounding kernel.

`TimeUnit` names the ten units a `Span` keeps (design/datetime.md
L3, where it is called `Unit`; see NAMED REFUSALS), ordered from
`Nanosecond` (smallest) to `Year` (largest). `RoundMode`
is TC39 Temporal's nine rounding modes. `round-i128` is the one
rounding kernel every rounding operation in the family goes through:
an exact integer rounding of a signed `I128` quantity to a multiple
of a positive `I128` increment. No float appears anywhere.

Design law shape.
  - `unit-rank` is strictly increasing from `Nanosecond` (0) to
    `Year` (9); `unit-cmp`, `unit-larger-of` and `unit-smaller-of`
    are that order. `unit-next-larger` and `unit-next-smaller` are
    inverse steps on it and are `None` at the two ends.
  - Units split in two. `Hour` and smaller have a fixed length:
    `unit-nanos` is `Some` of it. `Day` and larger are calendar units
    (`unit-is-calendar`): their length depends on a date (and, for a
    day, on a zone), so `unit-nanos` is `None`. `nanos-per-civil-day` is
    the civil 24-hour day, a named constant for the places where
    Temporal itself treats a day as 24 hours (civil date-times, which
    have no zone).
  - `round-i128 mode inc x` is Temporal's RoundNumberToIncrement: the
    result is a multiple of `inc`, it equals `x` when `x` already is
    one, and otherwise it is one of the two multiples of `inc` that
    bracket `x`, chosen by the mode:
      Ceil       toward positive infinity
      Floor      toward negative infinity
      Expand     away from zero
      Trunc      toward zero
      HalfCeil   to the nearer; a tie toward positive infinity
      HalfFloor  to the nearer; a tie toward negative infinity
      HalfExpand to the nearer; a tie away from zero
      HalfTrunc  to the nearer; a tie toward zero
      HalfEven   to the nearer; a tie to the even multiple
    so `round-i128 mode inc (neg x)` is
    `neg (round-i128 (round-mode-negate mode) inc x)`.
  - `round-i128-as-if-positive` is Temporal's
    RoundNumberToIncrementAsIfPositive (used for instants, whose sign
    is an accident of the epoch): `Expand` behaves as `Ceil`, `Trunc`
    as `Floor`, `HalfExpand` as `HalfCeil`, `HalfTrunc` as
    `HalfFloor`, for either sign of `x`.
  - The three increment validators are Temporal's
    ValidateTemporalRoundingIncrement with the three maxima it uses
    (see each function).

NAMED REFUSALS:
  - the type name `Unit`: the compiler answers "Unit is a builtin
    primitive type and cannot be redefined", so the type is
    `TimeUnit`. The module keeps the spec's name.
  - the name `nanos-per-day-24h` (and any `-24h` spelling): a hyphen
    followed by a digit does not lex inside an identifier, so the
    24-hour day is `nanos-per-civil-day`.
  - a refined increment: a wide scalar has no solver sort, so
    "increment > 0" cannot be an `I128` parameter's type. `round-i128`
    answers `None` for an increment below 1 (and for the one input
    whose magnitude does not exist, `I128`'s minimum, and for a result
    that leaves `I128`). Callers in this family pass increments they
    validated, and say so.
  - a float kernel: not offered; rounding is exact integer work.

</details>

<details class="doc-synopsis"><summary>Synopsis (24 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-RoundMode">RoundMode</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RoundMode</strong></code></li><li><a class="syn-name" href="#t-TimeUnit">TimeUnit</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TimeUnit</strong></code></li><li><a class="syn-name" href="#c-LargestOption">LargestOption</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">LargestOption</strong> <span class="tok-variable">o</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-LargestOption-with-largest">with-largest</a><code class="syn-sig"><strong class="tok-variable">with-largest</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">o</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-LargestOption" title="Data.Time.Unit.LargestOption">LargestOption</a> <span class="tok-variable">o</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">o</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">o</span></code></li><li><a class="syn-name" href="#c-RoundingOptions">RoundingOptions</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">RoundingOptions</strong> <span class="tok-variable">o</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-RoundingOptions-with-smallest">with-smallest</a><code class="syn-sig"><strong class="tok-variable">with-smallest</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">o</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-RoundingOptions" title="Data.Time.Unit.RoundingOptions">RoundingOptions</a> <span class="tok-variable">o</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">o</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">o</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-RoundingOptions-with-increment">with-increment</a><code class="syn-sig"><strong class="tok-variable">with-increment</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">o</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-RoundingOptions" title="Data.Time.Unit.RoundingOptions">RoundingOptions</a> <span class="tok-variable">o</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">o</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">o</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-RoundingOptions-with-mode">with-mode</a><code class="syn-sig"><strong class="tok-variable">with-mode</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">o</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-RoundingOptions" title="Data.Time.Unit.RoundingOptions">RoundingOptions</a> <span class="tok-variable">o</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-RoundMode" title="Data.Time.Unit.RoundMode">RoundMode</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">o</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">o</span></code></li><li><a class="syn-name" href="#v-nanos-per-civil-day">nanos-per-civil-day</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nanos-per-civil-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a></code></li><li><a class="syn-name" href="#v-round-i128">round-i128</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">round-i128</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-RoundMode" title="Data.Time.Unit.RoundMode">RoundMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a></code></li><li><a class="syn-name" href="#v-round-i128-as-if-positive">round-i128-as-if-positive</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">round-i128-as-if-positive</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-RoundMode" title="Data.Time.Unit.RoundMode">RoundMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a></code></li><li><a class="syn-name" href="#v-round-mode-negate">round-mode-negate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">round-mode-negate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-RoundMode" title="Data.Time.Unit.RoundMode">RoundMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-RoundMode" title="Data.Time.Unit.RoundMode">RoundMode</a></code></li><li><a class="syn-name" href="#v-unit-cmp">unit-cmp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unit-cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-unit-eq">unit-eq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unit-eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unit-is-calendar">unit-is-calendar</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unit-is-calendar</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unit-larger-of">unit-larger-of</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unit-larger-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a></code></li><li><a class="syn-name" href="#v-unit-nanos">unit-nanos</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unit-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a></code></li><li><a class="syn-name" href="#v-unit-next-larger">unit-next-larger</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unit-next-larger</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a></code></li><li><a class="syn-name" href="#v-unit-next-smaller">unit-next-smaller</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unit-next-smaller</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a></code></li><li><a class="syn-name" href="#v-unit-rank">unit-rank</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unit-rank</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-unit-round-increment-ok">unit-round-increment-ok</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unit-round-increment-ok</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unit-smaller-of">unit-smaller-of</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unit-smaller-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a></code></li><li><a class="syn-name" href="#v-unit-span-increment-ok">unit-span-increment-ok</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unit-span-increment-ok</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-unit-timestamp-increment-ok">unit-timestamp-increment-ok</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unit-timestamp-increment-ok</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-RoundMode">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RoundMode</strong></code><span class="doc-sig-links"><a href="#t-RoundMode" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L187" title="View source at line 187">Source</a></span></div>
<div class="doc-body">

Rounding modes.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Ceil"><code><strong class="tok-constructor">Ceil</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-RoundMode" title="Data.Time.Unit.RoundMode">RoundMode</a></code></li><li id="k-Floor"><code><strong class="tok-constructor">Floor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-RoundMode" title="Data.Time.Unit.RoundMode">RoundMode</a></code></li><li id="k-Expand"><code><strong class="tok-constructor">Expand</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-RoundMode" title="Data.Time.Unit.RoundMode">RoundMode</a></code></li><li id="k-Trunc"><code><strong class="tok-constructor">Trunc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-RoundMode" title="Data.Time.Unit.RoundMode">RoundMode</a></code></li><li id="k-HalfCeil"><code><strong class="tok-constructor">HalfCeil</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-RoundMode" title="Data.Time.Unit.RoundMode">RoundMode</a></code></li><li id="k-HalfFloor"><code><strong class="tok-constructor">HalfFloor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-RoundMode" title="Data.Time.Unit.RoundMode">RoundMode</a></code></li><li id="k-HalfExpand"><code><strong class="tok-constructor">HalfExpand</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-RoundMode" title="Data.Time.Unit.RoundMode">RoundMode</a></code></li><li id="k-HalfTrunc"><code><strong class="tok-constructor">HalfTrunc</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-RoundMode" title="Data.Time.Unit.RoundMode">RoundMode</a></code></li><li id="k-HalfEven"><code><strong class="tok-constructor">HalfEven</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-RoundMode" title="Data.Time.Unit.RoundMode">RoundMode</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-TimeUnit">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TimeUnit</strong></code><span class="doc-sig-links"><a href="#t-TimeUnit" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L86" title="View source at line 86">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Nanosecond"><code><strong class="tok-constructor">Nanosecond</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a></code></li><li id="k-Microsecond"><code><strong class="tok-constructor">Microsecond</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a></code></li><li id="k-Millisecond"><code><strong class="tok-constructor">Millisecond</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a></code></li><li id="k-Second"><code><strong class="tok-constructor">Second</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a></code></li><li id="k-Minute"><code><strong class="tok-constructor">Minute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a></code></li><li id="k-Hour"><code><strong class="tok-constructor">Hour</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a></code></li><li id="k-Day"><code><strong class="tok-constructor">Day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a></code></li><li id="k-Week"><code><strong class="tok-constructor">Week</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a></code></li><li id="k-Month"><code><strong class="tok-constructor">Month</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a></code></li><li id="k-Year"><code><strong class="tok-constructor">Year</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a></code></li></ul></div>
</div>
</article>

# Signatures

<article class="doc-entry" id="c-LargestOption">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">LargestOption</strong> <span class="tok-variable">o</span></code><span class="doc-sig-links"><a href="#c-LargestOption" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L380" title="View source at line 380">Source</a></span></div>
<div class="doc-body">

Options that also choose the largest unit of a result.

<div class="doc-children"><h4>Methods</h4><ul><li id="m-LargestOption-with-largest"><div class="doc-method-row"><code><strong class="tok-variable">with-largest</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">o</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-LargestOption" title="Data.Time.Unit.LargestOption">LargestOption</a> <span class="tok-variable">o</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">o</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">o</span></code><span class="doc-sig-links"><a href="#m-LargestOption-with-largest" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L381">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (3)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-LargestOption" title="Data.Time.Unit.LargestOption">LargestOption</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Arithmetic/#instances">Data.Time.Arithmetic</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-LargestOption" title="Data.Time.Unit.LargestOption">LargestOption</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanRound" title="Data.Time.Span.SpanRound">SpanRound</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Span/#instances">Data.Time.Span</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-LargestOption" title="Data.Time.Unit.LargestOption">LargestOption</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedDifference" title="Data.Time.Zoned.Arithmetic.ZonedDifference">ZonedDifference</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#instances">Data.Time.Zoned.Arithmetic</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-RoundingOptions">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">RoundingOptions</strong> <span class="tok-variable">o</span></code><span class="doc-sig-links"><a href="#c-RoundingOptions" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L374" title="View source at line 374">Source</a></span></div>
<div class="doc-body">

Options with a smallest unit, an increment and a mode.

<div class="doc-children"><h4>Methods</h4><ul><li id="m-RoundingOptions-with-smallest"><div class="doc-method-row"><code><strong class="tok-variable">with-smallest</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">o</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-RoundingOptions" title="Data.Time.Unit.RoundingOptions">RoundingOptions</a> <span class="tok-variable">o</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">o</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">o</span></code><span class="doc-sig-links"><a href="#m-RoundingOptions-with-smallest" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L375">Source</a></span></div></li><li id="m-RoundingOptions-with-increment"><div class="doc-method-row"><code><strong class="tok-variable">with-increment</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">o</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-RoundingOptions" title="Data.Time.Unit.RoundingOptions">RoundingOptions</a> <span class="tok-variable">o</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">o</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">o</span></code><span class="doc-sig-links"><a href="#m-RoundingOptions-with-increment" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L376">Source</a></span></div></li><li id="m-RoundingOptions-with-mode"><div class="doc-method-row"><code><strong class="tok-variable">with-mode</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">o</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-RoundingOptions" title="Data.Time.Unit.RoundingOptions">RoundingOptions</a> <span class="tok-variable">o</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-RoundMode" title="Data.Time.Unit.RoundMode">RoundMode</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">o</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">o</span></code><span class="doc-sig-links"><a href="#m-RoundingOptions-with-mode" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L377">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (5)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-RoundingOptions" title="Data.Time.Unit.RoundingOptions">RoundingOptions</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Difference" title="Data.Time.Arithmetic.Difference">Difference</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Arithmetic/#instances">Data.Time.Arithmetic</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-RoundingOptions" title="Data.Time.Unit.RoundingOptions">RoundingOptions</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Arithmetic/#t-Round" title="Data.Time.Arithmetic.Round">Round</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Arithmetic/#instances">Data.Time.Arithmetic</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-RoundingOptions" title="Data.Time.Unit.RoundingOptions">RoundingOptions</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Span/#t-SpanRound" title="Data.Time.Span.SpanRound">SpanRound</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Span/#instances">Data.Time.Span</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-RoundingOptions" title="Data.Time.Unit.RoundingOptions">RoundingOptions</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedDifference" title="Data.Time.Zoned.Arithmetic.ZonedDifference">ZonedDifference</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#instances">Data.Time.Zoned.Arithmetic</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#c-RoundingOptions" title="Data.Time.Unit.RoundingOptions">RoundingOptions</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#t-ZonedRound" title="Data.Time.Zoned.Arithmetic.ZonedRound">ZonedRound</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Zoned.Arithmetic/#instances">Data.Time.Zoned.Arithmetic</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-nanos-per-civil-day">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nanos-per-civil-day</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a></code><span class="doc-sig-links"><a href="#v-nanos-per-civil-day" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L154" title="View source at line 154">Source</a></span></div>
<div class="doc-body">

A civil day of exactly 24 hours, in nanoseconds. Used only where
Temporal says a day is 24 hours: civil date-time arithmetic (no
zone, so no transition can change a day's length) and the
conventions whose names say `days-24h`.

</div>
</article>

<article class="doc-entry" id="v-round-i128">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">round-i128</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-RoundMode" title="Data.Time.Unit.RoundMode">RoundMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a></code><span class="doc-sig-links"><a href="#v-round-i128" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L259" title="View source at line 259">Source</a></span></div>
<div class="doc-body">

The kernel: `x` rounded to a multiple of `inc` under `mode`
(Temporal's RoundNumberToIncrement; the header gives the law).
`None` when `inc < 1`, when `x` is `I128`'s minimum (its magnitude
does not exist) or when the rounded value leaves `I128`.

The work is on the magnitude `a = |x| = q * inc + r`. `a` is at
most 2^127 - 1 and `inc >= 1`, so the division is exact machine
division with no overflow; `q + 1` cannot overflow because
`q <= a`, and it is only formed when `r > 0`, where `q < a`. The
final product is checked.

</div>
</article>

<article class="doc-entry" id="v-round-i128-as-if-positive">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">round-i128-as-if-positive</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-RoundMode" title="Data.Time.Unit.RoundMode">RoundMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a></code><span class="doc-sig-links"><a href="#v-round-i128-as-if-positive" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L282" title="View source at line 282">Source</a></span></div>
<div class="doc-body">

Temporal's RoundNumberToIncrementAsIfPositive: the direction-free
modes are read on the number line rather than relative to zero, so
an instant before the epoch rounds the same way as one after it.

</div>
</article>

<article class="doc-entry" id="v-round-mode-negate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">round-mode-negate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-RoundMode" title="Data.Time.Unit.RoundMode">RoundMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-RoundMode" title="Data.Time.Unit.RoundMode">RoundMode</a></code><span class="doc-sig-links"><a href="#v-round-mode-negate" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L201" title="View source at line 201">Source</a></span></div>
<div class="doc-body">

Temporal's NegateRoundingMode: the mode that, applied to a negated
quantity, gives the negated result. Only the four modes that name a
direction on the number line change.

</div>
</article>

<article class="doc-entry" id="v-unit-cmp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unit-cmp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-unit-cmp" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L115" title="View source at line 115">Source</a></span></div>
</article>

<article class="doc-entry" id="v-unit-eq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unit-eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unit-eq" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L121" title="View source at line 121">Source</a></span></div>
</article>

<article class="doc-entry" id="v-unit-is-calendar">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unit-is-calendar</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unit-is-calendar" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L132" title="View source at line 132">Source</a></span></div>
<div class="doc-body">

Day and larger: units whose length is not a fixed number of
nanoseconds.

</div>
</article>

<article class="doc-entry" id="v-unit-larger-of">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unit-larger-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a></code><span class="doc-sig-links"><a href="#v-unit-larger-of" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L124" title="View source at line 124">Source</a></span></div>
</article>

<article class="doc-entry" id="v-unit-nanos">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unit-nanos</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a></code><span class="doc-sig-links"><a href="#v-unit-nanos" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L137" title="View source at line 137">Source</a></span></div>
<div class="doc-body">

The length of a fixed-length unit in nanoseconds; `None` for `Day`
and larger.

</div>
</article>

<article class="doc-entry" id="v-unit-next-larger">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unit-next-larger</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a></code><span class="doc-sig-links"><a href="#v-unit-next-larger" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L157" title="View source at line 157">Source</a></span></div>
</article>

<article class="doc-entry" id="v-unit-next-smaller">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unit-next-smaller</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a></code><span class="doc-sig-links"><a href="#v-unit-next-smaller" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L170" title="View source at line 170">Source</a></span></div>
</article>

<article class="doc-entry" id="v-unit-rank">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unit-rank</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-unit-rank" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L102" title="View source at line 102">Source</a></span></div>
<div class="doc-body">

The order: Nanosecond is the smallest unit, Year the largest.

</div>
</article>

<article class="doc-entry" id="v-unit-round-increment-ok">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unit-round-increment-ok</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unit-round-increment-ok" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L333" title="View source at line 333">Source</a></span></div>
<div class="doc-body">

For rounding a `TimeOfDay` or a `DateTime` (PlainTime.round,
PlainDateTime.round): the same maxima for hour and smaller; a day
increment must be exactly 1; week and larger are not rounding units
of a value.

</div>
</article>

<article class="doc-entry" id="v-unit-smaller-of">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unit-smaller-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a></code><span class="doc-sig-links"><a href="#v-unit-smaller-of" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L127" title="View source at line 127">Source</a></span></div>
</article>

<article class="doc-entry" id="v-unit-span-increment-ok">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unit-span-increment-ok</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unit-span-increment-ok" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L316" title="View source at line 316">Source</a></span></div>
<div class="doc-body">

For rounding a span and for `until`/`since`
(MaximumTemporalDurationRoundingIncrement, exclusive): hours must
divide 24 and be below it, minutes and seconds 60, milliseconds,
microseconds and nanoseconds 1000. Day and larger have no maximum:
any increment in 1..=1000000000.

</div>
</article>

<article class="doc-entry" id="v-unit-timestamp-increment-ok">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unit-timestamp-increment-ok</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Unit/#t-TimeUnit" title="Data.Time.Unit.TimeUnit">TimeUnit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-unit-timestamp-increment-ok" title="Permalink">#</a><a href="/packages/stdlib/Data.Time.Unit/source/#L351" title="View source at line 351">Source</a></span></div>
<div class="doc-body">

For rounding a `Timestamp` (Instant.round): the increment, in its
unit, must divide one 24-hour day evenly, and may equal it
(inclusive): hours divide 24, minutes 1440, seconds 86400, and so
on, within the general bound of 1000000000. Day and larger are
refused.

</div>
</article>
