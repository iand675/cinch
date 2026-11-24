# Text.Otlp

OTLP/JSON record construction/rendering for structured log/trace data and trace-context helpers.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Otlp</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Otlp/source/">View source</a><a href="/packages/search/?q=%2BText.Otlp">Search module</a><a href="/reference-sources/Text.Otlp.cinch" download="Text.Otlp.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Effect.Log](Effect.Log.md), [Effect.Trace](Effect.Trace.md), [Text.Show](Text.Show.md).

<details class="doc-closure"><summary>Ship-source closure (3 files)</summary>

Copy these modules beside your entry file before importing `Text.Otlp`: [Effect.Log](Effect.Log.md), [Effect.Trace](Effect.Trace.md), [Text.Otlp](Text.Otlp.md).

```sh
cp seed/prelude/Effect/Log.cinch ./Effect.Log.cinch
cp seed/prelude/Effect/Trace.cinch ./Effect.Trace.cinch
cp seed/prelude/Text/Otlp.cinch ./Text.Otlp.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Text.Otlp: the observability wire formats (the P32
structured/OTLP/W3C story).

Module scheme (design/base-library.md): `Text.*` is conversion to and
from String/Bytes. This module is the P32 wire layer over the
`Effect.Log`/`Effect.Trace` record shapes and `Text.Json`'s canonical
printer:

  * the W3C `traceparent` header parsed and formatted around
    `Effect.Trace`'s `TraceContext` (trace context recommendation,
    section 3.2: `version-traceid-spanid-flags`, lowercase
    hexadecimal, `ff` version forbidden, zero trace/span ids
    forbidden, version `00` exactly the four fields and later
    versions parsed through their first 55 characters);
  * the OTLP/JSON export envelope (`otlp-export`) built over
    `Text.Json`: the collected records (log, metric and nested span
    records) become `resourceLogs`/`resourceMetrics`/
    `resourceSpans` in the protobuf-JSON mapping shape (JSON only;
    the protobuf binary encoding is a named boundary, not this
    module's job).

EXPORT ASSIGNMENT (reported): the trace context supplies the trace
id, the parent span id and the trace flags; the export walk assigns
each span a span id (the visit index zero-padded to 16 decimal
digits, which is valid lowercase hex) and nests children under their
parent, so logs and metrics inside a scope carry the enclosing span
id. Times (`timeUnixNano`) are deliberately absent: the P34
platform layer owns the clock and the export shape does not invent
one.

</details>

<details class="doc-synopsis" open><summary>Synopsis (5 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-any-value">any-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">any-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li><a class="syn-name" href="#v-attributes">attributes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">attributes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li><a class="syn-name" href="#v-format-traceparent">format-traceparent</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">format-traceparent</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Trace/#t-TraceContext" title="Effect.Trace.TraceContext">TraceContext</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-otlp-export">otlp-export</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">otlp-export</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Trace/#t-TraceContext" title="Effect.Trace.TraceContext">TraceContext</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li><a class="syn-name" href="#v-parse-traceparent">parse-traceparent</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-traceparent</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Trace/#t-TraceContext" title="Effect.Trace.TraceContext">TraceContext</a> <span class="tok-variable">r</span>)</code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-any-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">any-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-any-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Otlp/source/#L276" title="View source at line 276">Source</a></span></div>
<div class="doc-body">

One `Json` value as an OTLP `AnyValue` (the protobuf-JSON
mapping): strings, int64-as-string, doubles, bools, arrays as
`arrayValue`, objects as `kvlistValue`, `JNull` as the empty
value.

</div>
</article>

<article class="doc-entry" id="v-attributes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">attributes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-attributes" title="Permalink">#</a><a href="/packages/stdlib/Text.Otlp/source/#L263" title="View source at line 263">Source</a></span></div>
<div class="doc-body">

A record's fields as OTLP attributes: `[{"key": ..., "value":
<AnyValue>}, ...]` in object-spine order.

</div>
</article>

<article class="doc-entry" id="v-format-traceparent">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">format-traceparent</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Trace/#t-TraceContext" title="Effect.Trace.TraceContext">TraceContext</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-format-traceparent" title="Permalink">#</a><a href="/packages/stdlib/Text.Otlp/source/#L84" title="View source at line 84">Source</a></span></div>
<div class="doc-body">

Format the context back to its `traceparent` header: parse and
format round trip exactly (the header is canonical lowercase).

</div>
</article>

<article class="doc-entry" id="v-otlp-export">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">otlp-export</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Trace/#t-TraceContext" title="Effect.Trace.TraceContext">TraceContext</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-otlp-export" title="Permalink">#</a><a href="/packages/stdlib/Text.Otlp/source/#L140" title="View source at line 140">Source</a></span></div>
<div class="doc-body">

The envelope over the collected records: `resourceLogs` carries
the log records, `resourceMetrics` the metric records and
`resourceSpans` the span trees (flattened with parent links). The
trace context stamps every signal: its trace id, the parent span
id of the roots (the incoming span) and the span's trace flags.

</div>
</article>

<article class="doc-entry" id="v-parse-traceparent">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-traceparent</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Trace/#t-TraceContext" title="Effect.Trace.TraceContext">TraceContext</a> <span class="tok-variable">r</span>)</code><span class="doc-sig-links"><a href="#v-parse-traceparent" title="Permalink">#</a><a href="/packages/stdlib/Text.Otlp/source/#L65" title="View source at line 65">Source</a></span></div>
<div class="doc-body">

Parse a `traceparent` header into the trace context of some
identity `r`. Refused (as `None`) unless the header is exactly the
section 3.2 grammar: 55 lowercase-hex characters in four
dash-delimited fields (a later version may carry trailing fields,
which are ignored), version not `ff`, version `00` with no
trailing data, and neither id all zeros.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Otlp.cinch`](/packages/stdlib/Text.Otlp/source/). Source SHA-256: `650ee7a5b2e4c12c7c01d4218d520cc4dbd97432fedd85291c8831abdf543b9d`.
