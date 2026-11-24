# Effect.Log

Structured log operations and records with runtime capability/effect accounting.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Log</code></div><div class="doc-mod-bar-links"><a href="/library/source/Effect.Log/">View source</a><a href="/library/search/?q=%2BEffect.Log">Search module</a><a href="/reference-sources/Effect.Log.cinch" download="Effect.Log.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Json](Data.Json.md), [Data.List](Data.List.md).

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Effect.Log`: [Effect.Log](Effect.Log.md).

```sh
cp seed/prelude/Effect/Log.cinch ./Effect.Log.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Effect.Log: the `Log r` signal vocabulary (the P32 observability
rows): log levels, metric kinds, the `log`/`metric` effect
operations, and the structured `Json` records they carry.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary. The observability story is two families in the
`Io r` mold (the runtime capabilities, effects and sequencing
amendments, D21/D81): region-parameterized effect labels over the
runtime identity `r`, so a signature quantifies `forall r.` and its
body row carries `{Log r}` (here) and `{Trace r}` (Effect.Trace).
The operations are ordinary functions performing their label; a
handler interprets them (the P6 story), and the emitting evidence is
the ordinary `Io r` capability the handler holds. Unhandled, the
operations are just their effect rows and `main`'s boundary refuses
them; that is the whole capability story.

THE RECORDS (reported): a `log`/`metric` payload is a structured
`Json` record (`log-record`/`metric-record` build the shapes);
fields are a `Json` object built with `fields`/`num`/`str`/`flag`/
`arr` so a call site stays free of ceremony. The record shapes are
the contract `Text.Otlp` exports over the wire (OTLP/JSON).

</details>

<details class="doc-synopsis"><summary>Synopsis (21 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-LogLevel">LogLevel</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">LogLevel</strong></code></li><li><a class="syn-name" href="#t-MetricKind">MetricKind</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MetricKind</strong></code></li><li><a class="syn-name" href="#v-arr">arr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">arr</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li><a class="syn-name" href="#v-counter">counter</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">counter</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-MetricKind" title="Effect.Log.MetricKind">MetricKind</a></code></li><li><a class="syn-name" href="#v-debug">debug</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">debug</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-LogLevel" title="Effect.Log.LogLevel">LogLevel</a></code></li><li><a class="syn-name" href="#v-error">error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">error</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-LogLevel" title="Effect.Log.LogLevel">LogLevel</a></code></li><li><a class="syn-name" href="#v-fields">fields</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fields</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li><a class="syn-name" href="#v-flag">flag</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">flag</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li><a class="syn-name" href="#v-gauge">gauge</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gauge</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-MetricKind" title="Effect.Log.MetricKind">MetricKind</a></code></li><li><a class="syn-name" href="#v-histogram">histogram</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">histogram</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-MetricKind" title="Effect.Log.MetricKind">MetricKind</a></code></li><li><a class="syn-name" href="#v-info">info</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">info</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-LogLevel" title="Effect.Log.LogLevel">LogLevel</a></code></li><li><a class="syn-name" href="#v-level-name">level-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">level-name</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-LogLevel" title="Effect.Log.LogLevel">LogLevel</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-log">log</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">log</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-LogLevel" title="Effect.Log.LogLevel">LogLevel</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span>{<span class="tok-constructor">Log</span> r} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-log-record">log-record</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">log-record</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-LogLevel" title="Effect.Log.LogLevel">LogLevel</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li><a class="syn-name" href="#v-metric">metric</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">metric</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-MetricKind" title="Effect.Log.MetricKind">MetricKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span>{<span class="tok-constructor">Log</span> r} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-metric-kind-name">metric-kind-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">metric-kind-name</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-MetricKind" title="Effect.Log.MetricKind">MetricKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-metric-record">metric-record</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">metric-record</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-MetricKind" title="Effect.Log.MetricKind">MetricKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li><a class="syn-name" href="#v-no-fields">no-fields</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">no-fields</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li><a class="syn-name" href="#v-num">num</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">num</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li><a class="syn-name" href="#v-str">str</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">str</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li><a class="syn-name" href="#v-warn">warn</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">warn</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-LogLevel" title="Effect.Log.LogLevel">LogLevel</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-LogLevel">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">LogLevel</strong></code><span class="doc-sig-links"><a href="#t-LogLevel" title="Permalink">#</a><a href="/library/source/Effect.Log/#L36" title="View source at line 36">Source</a></span></div>
<div class="doc-body">

The level of a log record. The lowercase values (`info` and
friends) are the DX spelling at a `log` call; the constructors
surface (D308) for handler clauses that match on them.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Debug"><code><strong class="tok-constructor">Debug</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-LogLevel" title="Effect.Log.LogLevel">LogLevel</a></code></li><li id="k-Info"><code><strong class="tok-constructor">Info</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-LogLevel" title="Effect.Log.LogLevel">LogLevel</a></code></li><li id="k-Warn"><code><strong class="tok-constructor">Warn</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-LogLevel" title="Effect.Log.LogLevel">LogLevel</a></code></li><li id="k-Error"><code><strong class="tok-constructor">Error</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-LogLevel" title="Effect.Log.LogLevel">LogLevel</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MetricKind">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MetricKind</strong></code><span class="doc-sig-links"><a href="#t-MetricKind" title="Permalink">#</a><a href="/library/source/Effect.Log/#L67" title="View source at line 67">Source</a></span></div>
<div class="doc-body">

The instrument family of a metric record: a counter accumulates
deltas, a gauge carries the latest reading, a histogram observes
values into buckets. The lowercase values are the DX spelling at a
`metric` call.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Counter"><code><strong class="tok-constructor">Counter</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-MetricKind" title="Effect.Log.MetricKind">MetricKind</a></code></li><li id="k-Gauge"><code><strong class="tok-constructor">Gauge</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-MetricKind" title="Effect.Log.MetricKind">MetricKind</a></code></li><li id="k-Histogram"><code><strong class="tok-constructor">Histogram</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-MetricKind" title="Effect.Log.MetricKind">MetricKind</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-arr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">arr</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-arr" title="Permalink">#</a><a href="/library/source/Effect.Log/#L117" title="View source at line 117">Source</a></span></div>
</article>

<article class="doc-entry" id="v-counter">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">counter</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-MetricKind" title="Effect.Log.MetricKind">MetricKind</a></code><span class="doc-sig-links"><a href="#v-counter" title="Permalink">#</a><a href="/library/source/Effect.Log/#L72" title="View source at line 72">Source</a></span></div>
</article>

<article class="doc-entry" id="v-debug">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">debug</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-LogLevel" title="Effect.Log.LogLevel">LogLevel</a></code><span class="doc-sig-links"><a href="#v-debug" title="Permalink">#</a><a href="/library/source/Effect.Log/#L42" title="View source at line 42">Source</a></span></div>
</article>

<article class="doc-entry" id="v-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">error</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-LogLevel" title="Effect.Log.LogLevel">LogLevel</a></code><span class="doc-sig-links"><a href="#v-error" title="Permalink">#</a><a href="/library/source/Effect.Log/#L51" title="View source at line 51">Source</a></span></div>
</article>

<article class="doc-entry" id="v-fields">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fields</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-fields" title="Permalink">#</a><a href="/library/source/Effect.Log/#L102" title="View source at line 102">Source</a></span></div>
</article>

<article class="doc-entry" id="v-flag">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">flag</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-flag" title="Permalink">#</a><a href="/library/source/Effect.Log/#L114" title="View source at line 114">Source</a></span></div>
</article>

<article class="doc-entry" id="v-gauge">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gauge</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-MetricKind" title="Effect.Log.MetricKind">MetricKind</a></code><span class="doc-sig-links"><a href="#v-gauge" title="Permalink">#</a><a href="/library/source/Effect.Log/#L75" title="View source at line 75">Source</a></span></div>
</article>

<article class="doc-entry" id="v-histogram">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">histogram</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-MetricKind" title="Effect.Log.MetricKind">MetricKind</a></code><span class="doc-sig-links"><a href="#v-histogram" title="Permalink">#</a><a href="/library/source/Effect.Log/#L78" title="View source at line 78">Source</a></span></div>
</article>

<article class="doc-entry" id="v-info">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">info</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-LogLevel" title="Effect.Log.LogLevel">LogLevel</a></code><span class="doc-sig-links"><a href="#v-info" title="Permalink">#</a><a href="/library/source/Effect.Log/#L45" title="View source at line 45">Source</a></span></div>
</article>

<article class="doc-entry" id="v-level-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">level-name</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-LogLevel" title="Effect.Log.LogLevel">LogLevel</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-level-name" title="Permalink">#</a><a href="/library/source/Effect.Log/#L56" title="View source at line 56">Source</a></span></div>
<div class="doc-body">

The level's text in a record (`"debug"`, `"info"`, `"warn"`,
`"error"`).

</div>
</article>

<article class="doc-entry" id="v-log">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">log</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-LogLevel" title="Effect.Log.LogLevel">LogLevel</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span>{<span class="tok-constructor">Log</span> r} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-log" title="Permalink">#</a><a href="/library/source/Effect.Log/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-log-record">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">log-record</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-LogLevel" title="Effect.Log.LogLevel">LogLevel</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-log-record" title="Permalink">#</a><a href="/library/source/Effect.Log/#L123" title="View source at line 123">Source</a></span></div>
<div class="doc-body">

A log record: `{"level": ..., "message": ..., "fields": ...}`.

</div>
</article>

<article class="doc-entry" id="v-metric">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">metric</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-MetricKind" title="Effect.Log.MetricKind">MetricKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span>{<span class="tok-constructor">Log</span> r} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-metric" title="Permalink">#</a><a href="/library/source/Effect.Log/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-metric-kind-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">metric-kind-name</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-MetricKind" title="Effect.Log.MetricKind">MetricKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-metric-kind-name" title="Permalink">#</a><a href="/library/source/Effect.Log/#L81" title="View source at line 81">Source</a></span></div>
</article>

<article class="doc-entry" id="v-metric-record">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">metric-record</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-MetricKind" title="Effect.Log.MetricKind">MetricKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-metric-record" title="Permalink">#</a><a href="/library/source/Effect.Log/#L131" title="View source at line 131">Source</a></span></div>
<div class="doc-body">

A metric record: `{"metric": ..., "name": ..., "value": ...,
"fields": ...}`.

</div>
</article>

<article class="doc-entry" id="v-no-fields">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">no-fields</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-no-fields" title="Permalink">#</a><a href="/library/source/Effect.Log/#L99" title="View source at line 99">Source</a></span></div>
<div class="doc-body">

The field payload of a record: a `Json` object. `no-fields` is the
empty payload; `fields` builds one from its pairs (the DX shape:
`fields [("port", num 8080), ("host", str "localhost")]`); `num`,
`str` and `flag` are the scalar spellings; `arr` is a `Json`
array.

</div>
</article>

<article class="doc-entry" id="v-num">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">num</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-num" title="Permalink">#</a><a href="/library/source/Effect.Log/#L108" title="View source at line 108">Source</a></span></div>
</article>

<article class="doc-entry" id="v-str">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">str</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-str" title="Permalink">#</a><a href="/library/source/Effect.Log/#L111" title="View source at line 111">Source</a></span></div>
</article>

<article class="doc-entry" id="v-warn">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">warn</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Log/#t-LogLevel" title="Effect.Log.LogLevel">LogLevel</a></code><span class="doc-sig-links"><a href="#v-warn" title="Permalink">#</a><a href="/library/source/Effect.Log/#L48" title="View source at line 48">Source</a></span></div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Log.cinch`](/library/source/Effect.Log/). Source SHA-256: `c32dae495122e7b3963d8fa2fd19fab81031f29093493080206d0191e7db09ed`.
