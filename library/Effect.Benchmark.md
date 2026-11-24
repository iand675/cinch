# Effect.Benchmark

Benchmark case execution and timing/reporting helpers over runtime clocks.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Benchmark</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Benchmark/source/">View source</a><a href="/packages/search/?q=%2BEffect.Benchmark">Search module</a><a href="/reference-sources/Effect.Benchmark.cinch" download="Effect.Benchmark.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.String](Data.String.md), [Effect.Clock](Effect.Clock.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

Effect.Benchmark: timing `IO r` actions against the runtime's
monotonic clock, the Ruby `Benchmark` surface (the tooling entry in
design/stdlib-parity-ruby.md, ported onto the P34 `Effect.Clock`).

The clock is the monotonic reading only (`clock-monotonic-ns`, never
`clock-wall-ns`), so an interval is a natural number of nanoseconds
and never runs backwards. As with Ruby's `Benchmark`, no attempt is
made to subtract the measurement overhead (the two clock reads and
the report record): an interval is the raw span around the action,
and two measurements are comparable because they pay the same
overhead.

Design law shape:
  value transparency: `benchmark-measure label work` returns
    `work`'s value unchanged (timing never consumes or alters the
    timed result);
  non-negativity: the monotonic clock is non-decreasing, so the
    elapsed nanoseconds (and `benchmark-realtime`'s `Duration`) are
    always non-negative;
  agreement: `benchmark-realtime` and `benchmark-measure` read the
    same interval around the same action (the former truncates to
    whole milliseconds), and `benchmark-table`'s total is the sum
    of its rows;
  ordering only: measurements are compared to each other, never to
    an absolute wall-clock budget.

Named refusals (boundaries this module deliberately does not cross):
  overhead subtraction: not attempted (Ruby does not either); the
    clock reads stay part of the interval;
  absolute-duration claims: the module offers no thresholds or
    budgets, and its tests order measurements rather than pin
    absolute times;
  wall-clock and calendar: only the monotonic clock is consulted;
    dates and formatting are the Time family's business
    (Effect.Clock's header), not here;
  statistics: no minimum/maximum/mean/deviation aggregation and no
    Ruby `Benchmark.bmbm` warm-up pass; a report is one interval and
    the table adds the rows up;
  compile-time readings: `Effect.Clock`'s refusal (D45): the timing
    operations need the `Io r` capability and cannot run in a
    `stage` body.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary, and timing is a runtime capability in the
`Io r` mold (D45), so the home is Effect.

</details>

<details class="doc-synopsis" open><summary>Synopsis (6 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-BenchmarkReport">BenchmarkReport</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BenchmarkReport</strong></code></li><li><a class="syn-name" href="#v-benchmark-elapsed-ns">benchmark-elapsed-ns</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">benchmark-elapsed-ns</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Benchmark/#t-BenchmarkReport" title="Effect.Benchmark.BenchmarkReport">BenchmarkReport</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-benchmark-label">benchmark-label</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">benchmark-label</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Benchmark/#t-BenchmarkReport" title="Effect.Benchmark.BenchmarkReport">BenchmarkReport</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-benchmark-measure">benchmark-measure</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">benchmark-measure</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Benchmark/#t-BenchmarkReport" title="Effect.Benchmark.BenchmarkReport">BenchmarkReport</a>, <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-benchmark-realtime">benchmark-realtime</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">benchmark-realtime</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a></code></li><li><a class="syn-name" href="#v-benchmark-table">benchmark-table</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">benchmark-table</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Benchmark/#t-BenchmarkReport" title="Effect.Benchmark.BenchmarkReport">BenchmarkReport</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-BenchmarkReport">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BenchmarkReport</strong></code><span class="doc-sig-links"><a href="#t-BenchmarkReport" title="Permalink">#</a><a href="/packages/stdlib/Effect.Benchmark/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

One measurement: the label the caller supplied and the elapsed
monotonic nanoseconds around the action. The constructor is the
smart surface's only source of reports, so a report's interval is
always one this module read.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-BenchmarkReport"><code><strong class="tok-constructor">BenchmarkReport</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Benchmark/#t-BenchmarkReport" title="Effect.Benchmark.BenchmarkReport">BenchmarkReport</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-benchmark-elapsed-ns">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">benchmark-elapsed-ns</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Benchmark/#t-BenchmarkReport" title="Effect.Benchmark.BenchmarkReport">BenchmarkReport</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-benchmark-elapsed-ns" title="Permalink">#</a><a href="/packages/stdlib/Effect.Benchmark/source/#L69" title="View source at line 69">Source</a></span></div>
</article>

<article class="doc-entry" id="v-benchmark-label">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">benchmark-label</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Benchmark/#t-BenchmarkReport" title="Effect.Benchmark.BenchmarkReport">BenchmarkReport</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-benchmark-label" title="Permalink">#</a><a href="/packages/stdlib/Effect.Benchmark/source/#L65" title="View source at line 65">Source</a></span></div>
</article>

<article class="doc-entry" id="v-benchmark-measure">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">benchmark-measure</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Benchmark/#t-BenchmarkReport" title="Effect.Benchmark.BenchmarkReport">BenchmarkReport</a>, <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-benchmark-measure" title="Permalink">#</a><a href="/packages/stdlib/Effect.Benchmark/source/#L79" title="View source at line 79">Source</a></span></div>
<div class="doc-body">

Run `work` and report the monotonic span around it, together with
`work`'s own value so a measurement never discards the result. The
elapsed is `t1 - t0`; the monotonic clock guarantees `t1 >= t0`,
but that is the OS's promise, not a fact the checker sees, so the
span clamps at zero (a duration is never negative).

</div>
</article>

<article class="doc-entry" id="v-benchmark-realtime">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">benchmark-realtime</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Clock/#t-Duration" title="Effect.Clock.Duration">Duration</a></code><span class="doc-sig-links"><a href="#v-benchmark-realtime" title="Permalink">#</a><a href="/packages/stdlib/Effect.Benchmark/source/#L90" title="View source at line 90">Source</a></span></div>
<div class="doc-body">

The same interval as `benchmark-measure`, truncated to whole
milliseconds (`Effect.Clock`'s `monotonic-elapsed`); the action's
value is deliberately dropped, so callers wanting it use
`benchmark-measure`.

</div>
</article>

<article class="doc-entry" id="v-benchmark-table">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">benchmark-table</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Benchmark/#t-BenchmarkReport" title="Effect.Benchmark.BenchmarkReport">BenchmarkReport</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-benchmark-table" title="Permalink">#</a><a href="/packages/stdlib/Effect.Benchmark/source/#L102" title="View source at line 102">Source</a></span></div>
<div class="doc-body">

The table: a header, one row per report in input order, and the
total row. Labels are the caller's, printed verbatim. The rows are
flattened once: nesting `string-concat` would re-copy the rest of the
table at every row.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Benchmark.cinch`](/packages/stdlib/Effect.Benchmark/source/). Source SHA-256: `59fcce379631b8530523a7370db82c170c0e5d6e0a7b5961e80fd270883e8ba3`.
