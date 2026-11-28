# Effect.Trace

Scoped tracing/span vocabulary and context propagation tied to the runtime identity.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Trace</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Trace/source/">View source</a><a href="/packages/search/?q=%2BEffect.Trace">Search module</a><a href="/reference-sources/Effect.Trace.cinch" download="Effect.Trace.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.String](Data.String.md), [Effect.Log](Effect.Log.md).

<details class="doc-notes"><summary>Module notes</summary>

Effect.Trace: the `Trace r` scope vocabulary (the P32 observability
rows): spans as lexical scopes, and the W3C trace context carried by
the runtime identity `r`.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary; this family is the scope signals beside
`Effect.Log`'s point signals. `span`/`end-span` are ordinary
operations in the `Io r` mold (D21/D81): `{Trace r}` on the row of
`forall r.` signatures, interpreted by a handler, refused at
`main`'s boundary when unhandled.

SPANS AS LEXICAL SCOPES (reported): `span name fields` opens the
scope and `end-span name` closes it, so a scope is the lexical
region between the two statements (a bracketed block of the
computation, named at both ends). A `span` whose scope runs to the
end of the handled computation needs no close (the tail scope), so
nested spans read innermost-last with no ceremony. The collecting
handler gets this shape for free: the close marker rides the
resumption chain (`end-marker`), and `span-scope` wraps exactly the
records up to the matching close (or, with no matching close, the
whole tail) in the span record. This is the `withScope` shape of the
roadmap under the bootstrap handler calculus (D23): dispatch is
lexical to the handled computation's own expression, so a
body-taking `span` could not carry handled operations in its body;
the bracket pair is the honest lexical scope until call-dispatched
handling lands.

THE TRACE CONTEXT (reported): the W3C traceparent fields (version,
trace-id, span-id, trace flags) indexed by the runtime identity
(`TraceContext r`): a context is built for one `r` and every
function that carries it quantifies the same `r`, so the context
travels with the identity the `{Log r}`/`{Trace r}` rows name. The
header text conversion lives in `Text.Otlp` (the wire formats).

</details>

<details class="doc-synopsis" open><summary>Synopsis (6 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-TraceContext">TraceContext</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TraceContext</strong> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-end-marker">end-marker</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">end-marker</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li><a class="syn-name" href="#v-end-span">end-span</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">end-span</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<span class="tok-constructor">Trace</span> r} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-span">span</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span>{<span class="tok-constructor">Trace</span> r} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-span-record">span-record</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-record</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li><a class="syn-name" href="#v-span-scope">span-scope</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span-scope</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-TraceContext">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TraceContext</strong> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#t-TraceContext" title="Permalink">#</a><a href="/packages/stdlib/Effect.Trace/source/#L59" title="View source at line 59">Source</a></span></div>
<div class="doc-body">

The W3C trace context of runtime identity `r` (trace context
recommendation, section 3.2): the traceparent header's fields,
version (`0`-`255`, `255` is the forbidden `ff`), the 32
lowercase-hex trace id, the 16 lowercase-hex parent span id and
the trace flags (`0`-`255`), all indexed by the identity the
context propagates through.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-TraceContext"><code><strong class="tok-constructor">TraceContext</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Trace/#t-TraceContext" title="Effect.Trace.TraceContext">TraceContext</a> <span class="tok-variable">r</span></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-end-marker">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">end-marker</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-end-marker" title="Permalink">#</a><a href="/packages/stdlib/Effect.Trace/source/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

The scope-end marker a collecting handler's `end-span` clause
conses onto the resumption chain; `span-scope` consumes it. The
name is the scope being closed (the bracket pair's check).

</div>
</article>

<article class="doc-entry" id="v-end-span">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">end-span</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<span class="tok-constructor">Trace</span> r} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-end-span" title="Permalink">#</a><a href="/packages/stdlib/Effect.Trace/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-span">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span>{<span class="tok-constructor">Trace</span> r} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-span" title="Permalink">#</a><a href="/packages/stdlib/Effect.Trace/source/" title="View source">Source</a></span></div>
</article>

<article class="doc-entry" id="v-span-record">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-record</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-span-record" title="Permalink">#</a><a href="/packages/stdlib/Effect.Trace/source/#L65" title="View source at line 65">Source</a></span></div>
<div class="doc-body">

A span record: `{"name": ..., "fields": ..., "children": [...]}`,
where children are the collected records of the scope (log,
metric and nested span records).

</div>
</article>

<article class="doc-entry" id="v-span-scope">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span-scope</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-span-scope" title="Permalink">#</a><a href="/packages/stdlib/Effect.Trace/source/#L80" title="View source at line 80">Source</a></span></div>
<div class="doc-body">

The collecting handler's `span` clause body: wrap `scope` (the
resumption's records) in the span record, children ending at the
matching close marker (or the tail when the scope is unclosed).

</div>
</article>
