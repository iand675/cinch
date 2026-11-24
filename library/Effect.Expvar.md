# Effect.Expvar

Region-scoped live metrics registry (Int, Float, String, Map, Func) with deterministic sorted JSON snapshots and /debug/vars HTTP response adapter.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Expvar`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Expvar.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Mutable.Var](Data.Mutable.Var.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Text.Http](Text.Http.md), [Text.Json](Text.Json.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Mutable.Var](Data.Mutable.Var.md), [Effect.Expvar](Effect.Expvar.md), [Text.Builder](Text.Builder.md), [Text.Cookie](Text.Cookie.md), [Text.Http](Text.Http.md), [Text.Json](Text.Json.md), [Text.Lex](Text.Lex.md), [Text.Parse](Text.Parse.md), [Text.Scan](Text.Scan.md), [Text.TextProto](Text.TextProto.md), [Text.Url](Text.Url.md), [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Ip](Text.Url.Ip.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Parser](Text.Url.Parser.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Resolve](Text.Url.Resolve.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Publishable live metrics, structured variable containers, and JSON snapshots for diagnostic HTTP endpoints.

Every metric variable and registry is explicitly parameterized by its mutable region without ambient global
state. Registries and nested metric maps maintain entries in sorted UTF-8 key order so snapshots, traversals,
and HTTP responses are deterministic.

</details>

# Types and constructors

<a id="type-4578707661724572726f72"></a>

## `ExpvarError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ExpvarError</span></code></pre>

Named errors reported by metric registration operations.

[Source](/reference-sources/Effect.Expvar.cinch) · line 56

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ExpvarAlreadyPublished</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarError</span></code></pre>

<a id="type-457870766172466c6f6174"></a>

## `ExpvarFloat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ExpvarFloat</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>)</code></pre>

A mutable 64-bit floating-point metric variable in region r.

[Source](/reference-sources/Effect.Expvar.cinch) · line 82

<a id="type-45787076617246756e63"></a>

## `ExpvarFunc`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ExpvarFunc</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>)</code></pre>

A dynamic metric computed on demand as a JSON value within region r.

[Source](/reference-sources/Effect.Expvar.cinch) · line 92

<a id="type-457870766172496e74"></a>

## `ExpvarInt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ExpvarInt</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>)</code></pre>

A mutable 64-bit signed integer metric variable in region r.

[Source](/reference-sources/Effect.Expvar.cinch) · line 77

<a id="type-4578707661724d6170"></a>

## `ExpvarMap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ExpvarMap</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>)</code></pre>

A mutable string-keyed sub-map of metric variables in region r, kept in sorted key order.

[Source](/reference-sources/Effect.Expvar.cinch) · line 175

<a id="type-4578707661725265676973747279"></a>

## `ExpvarRegistry`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ExpvarRegistry</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>)</code></pre>

An explicit mutable registry of named published metrics in region r.

[Source](/reference-sources/Effect.Expvar.cinch) · line 337

<a id="type-457870766172537472696e67"></a>

## `ExpvarString`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ExpvarString</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>)</code></pre>

A mutable UTF-8 string metric variable in region r.

[Source](/reference-sources/Effect.Expvar.cinch) · line 87

<a id="type-457870766172566172"></a>

## `ExpvarVar`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ExpvarVar</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>)</code></pre>

A unified published metric variable of any supported metric kind in region r.

[Source](/reference-sources/Effect.Expvar.cinch) · line 180

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">VarInt</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarInt</span> <span class="tok-variable">r</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarVar</span> <span class="tok-variable">r</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">VarFloat</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarFloat</span> <span class="tok-variable">r</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarVar</span> <span class="tok-variable">r</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">VarString</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarString</span> <span class="tok-variable">r</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarVar</span> <span class="tok-variable">r</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">VarMap</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarMap</span> <span class="tok-variable">r</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarVar</span> <span class="tok-variable">r</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">VarFunc</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>) -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarVar</span> <span class="tok-variable">r</span>)</code></pre>

# Values

<a id="value-6578707661722d656e7472696573"></a>

## `expvar-entries`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-entries</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarRegistry</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>)))</code></pre>

Traverse all published metrics in sorted key order, evaluating each to its current JSON value.

[Source](/reference-sources/Effect.Expvar.cinch) · line 441

<a id="value-6578707661722d6572726f722d6d657373616765"></a>

## `expvar-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-error-message</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a readable diagnostic message for a metric registry error.

[Source](/reference-sources/Effect.Expvar.cinch) · line 68

<a id="value-6578707661722d666c6f61742d616464"></a>

## `expvar-float-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-float-add</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarFloat</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">F64</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Add a floating-point delta to a float metric.

[Source](/reference-sources/Effect.Expvar.cinch) · line 131

<a id="value-6578707661722d666c6f61742d736574"></a>

## `expvar-float-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-float-set</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarFloat</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">F64</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Replace the current value of a float metric.

[Source](/reference-sources/Effect.Expvar.cinch) · line 138

<a id="value-6578707661722d666c6f61742d76616c7565"></a>

## `expvar-float-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-float-value</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarFloat</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">F64</span>)</code></pre>

Read the current floating-point value of a float metric.

[Source](/reference-sources/Effect.Expvar.cinch) · line 126

<a id="value-6578707661722d66756e632d76616c7565"></a>

## `expvar-func-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-func-value</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarFunc</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>)</code></pre>

Evaluate a dynamic metric callback to produce its current JSON representation.

[Source](/reference-sources/Effect.Expvar.cinch) · line 159

<a id="value-6578707661722d676574"></a>

## `expvar-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-get</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarRegistry</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarVar</span> <span class="tok-variable">r</span>))))</code></pre>

Retrieve a published metric variable by name from the registry.

[Source](/reference-sources/Effect.Expvar.cinch) · line 383

<a id="value-6578707661722d687474702d726573706f6e7365"></a>

## `expvar-http-response`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-http-response</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarRegistry</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>)</code></pre>

Build an HTTP 200 OK JSON response containing the live metric snapshot, ready for /debug/vars handlers.

[Source](/reference-sources/Effect.Expvar.cinch) · line 454

<a id="value-6578707661722d696e742d616464"></a>

## `expvar-int-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-int-add</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarInt</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Add a signed 64-bit delta (wrapping on overflow) to an integer metric.

[Source](/reference-sources/Effect.Expvar.cinch) · line 108

<a id="value-6578707661722d696e742d736574"></a>

## `expvar-int-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-int-set</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarInt</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Replace the current value of an integer metric.

[Source](/reference-sources/Effect.Expvar.cinch) · line 115

<a id="value-6578707661722d696e742d76616c7565"></a>

## `expvar-int-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-int-value</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarInt</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">I64</span>)</code></pre>

Read the current 64-bit signed integer value of an integer metric.

[Source](/reference-sources/Effect.Expvar.cinch) · line 103

<a id="value-6578707661722d6d61702d616464"></a>

## `expvar-map-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-map-add</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarMap</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)))</code></pre>

Increment an integer metric under key by delta, creating a new integer metric when absent.
If key currently holds a non-integer metric, the existing entry is left unchanged.

[Source](/reference-sources/Effect.Expvar.cinch) · line 264

<a id="value-6578707661722d6d61702d6164642d666c6f6174"></a>

## `expvar-map-add-float`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-map-add-float</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarMap</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">F64</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)))</code></pre>

Increment a floating-point metric under key by delta, creating a new float metric when absent.
If key currently holds a non-float metric, the existing entry is left unchanged.

[Source](/reference-sources/Effect.Expvar.cinch) · line 275

<a id="value-6578707661722d6d61702d64656c657465"></a>

## `expvar-map-delete`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-map-delete</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarMap</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Remove key and its associated metric variable from a metric map if present.

[Source](/reference-sources/Effect.Expvar.cinch) · line 285

<a id="value-6578707661722d6d61702d656e7472696573"></a>

## `expvar-map-entries`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-map-entries</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarMap</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarVar</span> <span class="tok-variable">r</span>))))</code></pre>

Snapshot all key-variable pairs of a metric map in sorted key order.

[Source](/reference-sources/Effect.Expvar.cinch) · line 303

<a id="value-6578707661722d6d61702d676574"></a>

## `expvar-map-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-map-get</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarMap</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarVar</span> <span class="tok-variable">r</span>))))</code></pre>

Look up the metric variable stored under key in a metric map.

[Source](/reference-sources/Effect.Expvar.cinch) · line 251

<a id="value-6578707661722d6d61702d696e6974"></a>

## `expvar-map-init`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-map-init</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarMap</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarMap</span> <span class="tok-variable">r</span>))</code></pre>

Clear all keys from a metric map and return the map handle.

[Source](/reference-sources/Effect.Expvar.cinch) · line 293

<a id="value-6578707661722d6d61702d6c656e"></a>

## `expvar-map-len`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-map-len</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarMap</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

Return the number of keys currently stored in a metric map.

[Source](/reference-sources/Effect.Expvar.cinch) · line 299

<a id="value-6578707661722d6d61702d736574"></a>

## `expvar-map-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-map-set</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarMap</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarVar</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)))</code></pre>

Store or replace a metric variable under key in a metric map.

[Source](/reference-sources/Effect.Expvar.cinch) · line 255

<a id="value-6578707661722d6d61702d746f2d6a736f6e"></a>

## `expvar-map-to-json`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-map-to-json</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarMap</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>)</code></pre>

Snapshot a metric map as a sorted JSON object.

[Source](/reference-sources/Effect.Expvar.cinch) · line 316

<a id="value-6578707661722d6e65772d666c6f6174"></a>

## `expvar-new-float`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-new-float</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">F64</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarFloat</span> <span class="tok-variable">r</span>))</code></pre>

Create a new floating-point metric initialized to the supplied F64 value.

[Source](/reference-sources/Effect.Expvar.cinch) · line 120

<a id="value-6578707661722d6e65772d66756e63"></a>

## `expvar-new-func`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-new-func</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarVar</span> <span class="tok-variable">r</span>))</code></pre>

Wrap a dynamic JSON-producing callback as an ExpvarVar.

[Source](/reference-sources/Effect.Expvar.cinch) · line 198

<a id="value-6578707661722d6e65772d696e74"></a>

## `expvar-new-int`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-new-int</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarInt</span> <span class="tok-variable">r</span>))</code></pre>

Create a new integer metric initialized to the supplied 64-bit value.

[Source](/reference-sources/Effect.Expvar.cinch) · line 97

<a id="value-6578707661722d6e65772d6d6170"></a>

## `expvar-new-map`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-new-map</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarMap</span> <span class="tok-variable">r</span>))</code></pre>

Create a new empty metric map in region r.

[Source](/reference-sources/Effect.Expvar.cinch) · line 202

<a id="value-6578707661722d6e65772d737472696e67"></a>

## `expvar-new-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-new-string</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarString</span> <span class="tok-variable">r</span>))</code></pre>

Create a new string metric initialized to the supplied text.

[Source](/reference-sources/Effect.Expvar.cinch) · line 143

<a id="value-6578707661722d7075626c697368"></a>

## `expvar-publish`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-publish</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarRegistry</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarVar</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarError</span> <span class="tok-constructor">Unit</span>))))</code></pre>

Publish a metric variable under name in the registry, rejecting duplicate names.

[Source](/reference-sources/Effect.Expvar.cinch) · line 372

<a id="value-6578707661722d7075626c6973682d666c6f6174"></a>

## `expvar-publish-float`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-publish-float</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarRegistry</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarFloat</span> <span class="tok-variable">r</span>))))</code></pre>

Create a zero-initialized floating-point metric and publish it under name.

[Source](/reference-sources/Effect.Expvar.cinch) · line 396

<a id="value-6578707661722d7075626c6973682d66756e63"></a>

## `expvar-publish-func`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-publish-func</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarRegistry</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarError</span> <span class="tok-constructor">Unit</span>))))</code></pre>

Publish a dynamic JSON-producing callback under name.

[Source](/reference-sources/Effect.Expvar.cinch) · line 420

<a id="value-6578707661722d7075626c6973682d696e74"></a>

## `expvar-publish-int`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-publish-int</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarRegistry</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarInt</span> <span class="tok-variable">r</span>))))</code></pre>

Create a zero-initialized integer metric and publish it under name.

[Source](/reference-sources/Effect.Expvar.cinch) · line 388

<a id="value-6578707661722d7075626c6973682d6d6170"></a>

## `expvar-publish-map`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-publish-map</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarRegistry</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarMap</span> <span class="tok-variable">r</span>))))</code></pre>

Create an empty metric map and publish it under name.

[Source](/reference-sources/Effect.Expvar.cinch) · line 412

<a id="value-6578707661722d7075626c6973682d737472696e67"></a>

## `expvar-publish-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-publish-string</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarRegistry</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarString</span> <span class="tok-variable">r</span>))))</code></pre>

Create an empty string metric and publish it under name.

[Source](/reference-sources/Effect.Expvar.cinch) · line 404

<a id="value-6578707661722d72656769737472792d6e6577"></a>

## `expvar-registry-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-registry-new</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarRegistry</span> <span class="tok-variable">r</span>))</code></pre>

Create a fresh empty metric registry in region r.

[Source](/reference-sources/Effect.Expvar.cinch) · line 342

<a id="value-6578707661722d736e617073686f742d6a736f6e"></a>

## `expvar-snapshot-json`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-snapshot-json</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarRegistry</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>)</code></pre>

Snapshot all published metrics in the registry as a JSON object with sorted keys.

[Source](/reference-sources/Effect.Expvar.cinch) · line 446

<a id="value-6578707661722d736e617073686f742d737472696e67"></a>

## `expvar-snapshot-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-snapshot-string</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarRegistry</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Snapshot all published metrics in the registry as a canonical JSON object string.

[Source](/reference-sources/Effect.Expvar.cinch) · line 450

<a id="value-6578707661722d737472696e672d736574"></a>

## `expvar-string-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-string-set</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarString</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Replace the current text value of a string metric.

[Source](/reference-sources/Effect.Expvar.cinch) · line 154

<a id="value-6578707661722d737472696e672d76616c7565"></a>

## `expvar-string-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-string-value</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarString</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Read the current text value of a string metric.

[Source](/reference-sources/Effect.Expvar.cinch) · line 149

<a id="value-6578707661722d7661722d737472696e67"></a>

## `expvar-var-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-var-string</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarVar</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render any metric variable as a canonical JSON string.

[Source](/reference-sources/Effect.Expvar.cinch) · line 329

<a id="value-6578707661722d7661722d746f2d6a736f6e"></a>

## `expvar-var-to-json`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">expvar-var-to-json</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Expvar</span>.<span class="tok-constructor">ExpvarVar</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>)</code></pre>

Snapshot any metric variable as a JSON value.

[Source](/reference-sources/Effect.Expvar.cinch) · line 320

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Effect.Expvar.Eq_ExpvarError` | `Relation.Eq.Eq Effect.Expvar.ExpvarError` | Implicit |

**`Effect.Expvar.Eq_ExpvarError`**: Structural equality for metric registry errors.

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Expvar.cinch`. Source SHA-256: `d832b0c207d08127415b03a6267218912885a5a4026351b2f3f4c3ac33ca6147`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
