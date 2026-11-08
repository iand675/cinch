# Effect.Log

Structured log operations and records with runtime capability/effect accounting.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Log`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Log.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Json](Data.Json.md), [Data.List](Data.List.md).

Non-embedded source closure: [Effect.Log](Effect.Log.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-4c6f674c6576656c"></a>

## `LogLevel`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">LogLevel</span></code></pre>

The level of a log record. The lowercase values (`info` and
friends) are the DX spelling at a `log` call; the constructors
surface (D308) for handler clauses that match on them.

[Source](/reference-sources/Effect.Log.cinch) · line 36

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Debug</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">LogLevel</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Info</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">LogLevel</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Warn</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">LogLevel</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Error</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">LogLevel</span></code></pre>

<a id="type-4d65747269634b696e64"></a>

## `MetricKind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MetricKind</span></code></pre>

The instrument family of a metric record: a counter accumulates
deltas, a gauge carries the latest reading, a histogram observes
values into buckets. The lowercase values are the DX spelling at a
`metric` call.

[Source](/reference-sources/Effect.Log.cinch) · line 67

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Counter</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">MetricKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Gauge</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">MetricKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Histogram</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">MetricKind</span></code></pre>

# Values

<a id="value-617272"></a>

## `arr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">arr</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>)</code></pre>

[Source](/reference-sources/Effect.Log.cinch)

<a id="value-636f756e746572"></a>

## `counter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">counter</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">MetricKind</span></code></pre>

[Source](/reference-sources/Effect.Log.cinch)

<a id="value-6465627567"></a>

## `debug`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">debug</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">LogLevel</span></code></pre>

[Source](/reference-sources/Effect.Log.cinch)

<a id="value-6572726f72"></a>

## `error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">error</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">LogLevel</span></code></pre>

[Source](/reference-sources/Effect.Log.cinch)

<a id="value-6669656c6473"></a>

## `fields`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fields</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>)) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>)</code></pre>

[Source](/reference-sources/Effect.Log.cinch)

<a id="value-666c6167"></a>

## `flag`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flag</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>)</code></pre>

[Source](/reference-sources/Effect.Log.cinch)

<a id="value-6761756765"></a>

## `gauge`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gauge</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">MetricKind</span></code></pre>

[Source](/reference-sources/Effect.Log.cinch)

<a id="value-686973746f6772616d"></a>

## `histogram`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">histogram</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">MetricKind</span></code></pre>

[Source](/reference-sources/Effect.Log.cinch)

<a id="value-696e666f"></a>

## `info`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">info</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">LogLevel</span></code></pre>

[Source](/reference-sources/Effect.Log.cinch)

<a id="value-6c6576656c2d6e616d65"></a>

## `level-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">level-name</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">LogLevel</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The level's text in a record (`"debug"`, `"info"`, `"warn"`,
`"error"`).

[Source](/reference-sources/Effect.Log.cinch) · line 56

<a id="value-6c6f67"></a>

## `log`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">log</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">LogLevel</span> -&gt; (<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Json</span> -&gt;{<span class="tok-constructor">Log</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)))</code></pre>

[Source](/reference-sources/Effect.Log.cinch)

<a id="value-6c6f672d7265636f7264"></a>

## `log-record`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">log-record</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">LogLevel</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>)))</code></pre>

A log record: `{"level": ..., "message": ..., "fields": ...}`.

[Source](/reference-sources/Effect.Log.cinch) · line 123

<a id="value-6d6574726963"></a>

## `metric`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">metric</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">MetricKind</span> -&gt; (<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Json</span> -&gt;{<span class="tok-constructor">Log</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))))</code></pre>

[Source](/reference-sources/Effect.Log.cinch)

<a id="value-6d65747269632d6b696e642d6e616d65"></a>

## `metric-kind-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">metric-kind-name</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">MetricKind</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Effect.Log.cinch)

<a id="value-6d65747269632d7265636f7264"></a>

## `metric-record`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">metric-record</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">MetricKind</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>))))</code></pre>

A metric record: `{"metric": ..., "name": ..., "value": ...,
"fields": ...}`.

[Source](/reference-sources/Effect.Log.cinch) · line 131

<a id="value-6e6f2d6669656c6473"></a>

## `no-fields`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">no-fields</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span></code></pre>

The field payload of a record: a `Json` object. `no-fields` is the
empty payload; `fields` builds one from its pairs (the DX shape:
`fields [("port", num 8080), ("host", str "localhost")]`); `num`,
`str` and `flag` are the scalar spellings; `arr` is a `Json`
array.

[Source](/reference-sources/Effect.Log.cinch) · line 99

<a id="value-6e756d"></a>

## `num`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">num</span> :: (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>)</code></pre>

[Source](/reference-sources/Effect.Log.cinch)

<a id="value-737472"></a>

## `str`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">str</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>)</code></pre>

[Source](/reference-sources/Effect.Log.cinch)

<a id="value-7761726e"></a>

## `warn`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">warn</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Log</span>.<span class="tok-constructor">LogLevel</span></code></pre>

[Source](/reference-sources/Effect.Log.cinch)

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Log.cinch`. Source SHA-256: `c32dae495122e7b3963d8fa2fd19fab81031f29093493080206d0191e7db09ed`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
