# Effect.Clock

Runtime monotonic and wall-clock readings under an I/O capability. Clocks are not ambient compile-time inputs.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Clock</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Clock/source/">View source</a><a href="/packages/search/?q=%2BEffect.Clock">Search module</a><a href="/reference-sources/Effect.Clock.cinch" download="Effect.Clock.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md).

<details class="doc-notes"><summary>Module notes</summary>

Effect.Clock: the runtime clocks, durations and timeouts (P34).

The clock is a runtime capability in the `Io r` mold (decision D45's
determinism rule): every reading requires the `Io r` capability and
performs `IO r`, so nothing ambient exists and a program's build
output cannot depend on when it compiles. Staging use is refused
outright rather than recorded: a `stage` body's row is pure and the
compile-time interpreter refuses foreign calls ("a foreign import
cannot run at compile time"), so no clock reading can enter a build
product (the env/capability precedent of example 05 records
declared inputs; the clock records nothing because it cannot
participate).

Durations are milliseconds (`Duration`), the unit the `Io r`
suspension (`ioSleep`) takes. `timeout` runs a delayed computation
under the P12 structured machinery and cancels it at the deadline:
the computation is watched by a sibling task whose failure at the
deadline cancels it (the `joinAllOrCancel` failure policy), and its
result is discarded if cancellation was requested during the run.
A value is returned only when the computation ran without observing
a cancellation request; at the deadline either outcome is allowed.
Cancellation is cooperative (concurrency specification section 4):
the computation is not killed, it observes the request at its next
suspension point.

Named refusals (boundaries this module deliberately does not
cross):
  compile-time readings: see above (D45; there is no staging path);
  pre-epoch or far-future wall times: `cinch_clock_wall_ns` clamps
    to `0..=i64::MAX` nanoseconds (the Unix epoch to the year 2262);
  killing a timed-out computation: P12 cancellation is cooperative;
  a work row wider than `{IO r, Cancel r}`: the worked shape is the
    delayed computation (sleeps and observes cancellation); wider
    rows need the checker's row-variable unification to carry
    labels through a rigid tail, which it currently drops (see the
    report), so `timeout` states its worked row exactly.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary.

</details>

<details class="doc-synopsis" open><summary>Synopsis (8 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Duration">Duration</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Duration</strong></code></li><li><a class="syn-name" href="#t-Timeout">Timeout</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Timeout</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-clock-monotonic-ns">clock-monotonic-ns</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">clock-monotonic-ns</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-clock-wall-ns">clock-wall-ns</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">clock-wall-ns</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-duration-from-millis">duration-from-millis</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-from-millis</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-duration-millis">duration-millis</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">duration-millis</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-monotonic-elapsed">monotonic-elapsed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">monotonic-elapsed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-timeout">timeout</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">timeout</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <span class="tok-variable">a</span>) <span class="tok-keyword">%1</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Async">Async</a> r, <a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Timeout" title="Effect.Clock.Timeout">Timeout</a> <span class="tok-variable">a</span></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Duration">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Duration</strong></code><span class="doc-sig-links"><a href="#t-Duration" title="Permalink">#</a><a href="/packages/stdlib/Effect.Clock/source/#L52" title="View source at line 52">Source</a></span></div>
<div class="doc-body">

A length of time in milliseconds.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Duration"><code><strong class="tok-constructor">Duration</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Duration/#instances">Data.Time.Duration</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Time.Duration/#instances">Data.Time.Duration</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-Timeout">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Timeout</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-Timeout" title="Permalink">#</a><a href="/packages/stdlib/Effect.Clock/source/#L88" title="View source at line 88">Source</a></span></div>
<div class="doc-body">

The outcome of a computation raced against its deadline: it
finished (`Finished x`), or the deadline fired first and cancelled
it (`TimedOut`).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-TimedOut"><code><strong class="tok-constructor">TimedOut</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Timeout" title="Effect.Clock.Timeout">Timeout</a> <span class="tok-variable">a</span></code></li><li id="k-Finished"><code><strong class="tok-constructor">Finished</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Timeout" title="Effect.Clock.Timeout">Timeout</a> <span class="tok-variable">a</span></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-clock-monotonic-ns">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">clock-monotonic-ns</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-clock-monotonic-ns" title="Permalink">#</a><a href="/packages/stdlib/Effect.Clock/source/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

Nanoseconds since an arbitrary origin: non-decreasing across
calls (two readings never go backwards).

</div>
</article>

<article class="doc-entry" id="v-clock-wall-ns">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">clock-wall-ns</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-clock-wall-ns" title="Permalink">#</a><a href="/packages/stdlib/Effect.Clock/source/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">

Nanoseconds since the Unix epoch (the clamped range above).

</div>
</article>

<article class="doc-entry" id="v-duration-from-millis">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-from-millis</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-duration-from-millis" title="Permalink">#</a><a href="/packages/stdlib/Effect.Clock/source/#L55" title="View source at line 55">Source</a></span></div>
</article>

<article class="doc-entry" id="v-duration-millis">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">duration-millis</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-duration-millis" title="Permalink">#</a><a href="/packages/stdlib/Effect.Clock/source/#L58" title="View source at line 58">Source</a></span></div>
</article>

<article class="doc-entry" id="v-monotonic-elapsed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">monotonic-elapsed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-monotonic-elapsed" title="Permalink">#</a><a href="/packages/stdlib/Effect.Clock/source/#L65" title="View source at line 65">Source</a></span></div>
<div class="doc-body">

The time between two monotonic readings (the first earlier):
milliseconds, truncated. A duration is never negative, so readings
passed in the other order measure zero.

</div>
</article>

<article class="doc-entry" id="v-timeout">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">timeout</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <span class="tok-variable">a</span>) <span class="tok-keyword">%1</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Async">Async</a> r, <a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Timeout" title="Effect.Clock.Timeout">Timeout</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-timeout" title="Permalink">#</a><a href="/packages/stdlib/Effect.Clock/source/#L124" title="View source at line 124">Source</a></span></div>
<div class="doc-body">

Run `work` under the deadline `d`: `Finished` its value when it
ran first, `TimedOut` when the deadline fired first and cancelled
it (the worked shape and its boundary above).

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Clock.cinch`](/packages/stdlib/Effect.Clock/source/). Source SHA-256: `df444a92ce56f1388be9014ddcdf637971c3d3c9082eb85f4d5e50c795e6522e`.
