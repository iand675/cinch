# Text.Pem

Owned byte-preserving envelope blocks, persistent headers, exact malformed-input recovery and bounded payload encoding.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Pem`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Pem.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Effect.Stream](Effect.Stream.md), [Text.Base64.Encoding](Text.Base64.Encoding.md).

Non-embedded source closure: [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Effect.Stream](Effect.Stream.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Pem](Text.Pem.md).

<details><summary>Behaviour and boundary notes from the source</summary>

PEM envelopes with immutable owned byte labels, headers and payloads.
Decoding preserves arbitrary byte metadata, recovers the next well-formed
envelope, and returns an independently owned remainder. Encoding validates
header keys before output, orders headers deterministically, and wraps the
standard padded Base64 alphabet at 64 columns. Writers retain continuations.

</details>

# Types and constructors

<a id="type-50656d426c6f636b"></a>

## `PemBlock`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PemBlock</span></code></pre>

An immutable envelope; labels and metadata may contain arbitrary non-UTF8 bytes.

[Source](/reference-sources/Text.Pem.cinch) · line 33

<a id="type-50656d4572726f72"></a>

## `PemError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PemError</span></code></pre>

Encoding validation or transport failure, with no implicit sink closure.

[Source](/reference-sources/Text.Pem.cinch) · line 38

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PemInvalidHeaderKey</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PemOutputError</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemError</span>)</code></pre>

<a id="type-50656d486561646572"></a>

## `PemHeader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PemHeader</span></code></pre>

Immutable header metadata; construction and byte observations copy caller storage.

[Source](/reference-sources/Text.Pem.cinch) · line 28

# Values

<a id="value-70656d2d626c6f636b"></a>

## `pem-block`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pem-block</span> :: (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemHeader</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemBlock</span>)))</code></pre>

Construct an immutable envelope; duplicate byte keys use the final supplied value.

[Source](/reference-sources/Text.Pem.cinch) · line 153

<a id="value-70656d2d626c6f636b2d6279746573"></a>

## `pem-block-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pem-block-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemBlock</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe an independently mutable copy of the decoded payload.

[Source](/reference-sources/Text.Pem.cinch) · line 161

<a id="value-70656d2d626c6f636b2d68656164657273"></a>

## `pem-block-headers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pem-block-headers</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemBlock</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemHeader</span>))</code></pre>

Observe immutable unique headers in byte-lexicographic key order.

[Source](/reference-sources/Text.Pem.cinch) · line 165

<a id="value-70656d2d626c6f636b2d74797065"></a>

## `pem-block-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pem-block-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemBlock</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe an independently mutable copy of the exact label bytes.

[Source](/reference-sources/Text.Pem.cinch) · line 157

<a id="value-70656d2d6465636f6465"></a>

## `pem-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pem-decode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemBlock</span>), <span class="tok-constructor">Bytes</span>))</code></pre>

Decode the next well-formed envelope and return a fresh independently mutable remainder.
Candidates start at a line boundary relative to recovery's current slice. Malformed
candidates are skipped by END trailer; absence or terminal malformed metadata returns
None and a fresh copy of the complete original input. Labels and headers preserve bytes.

[Source](/reference-sources/Text.Pem.cinch) · line 579

<a id="value-70656d2d656e636f64652d6279746573"></a>

## `pem-encode-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pem-encode-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemBlock</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Encode a complete fresh envelope or refuse a colon-bearing key without returning partial output.
Arbitrary labels and values are retained exactly, including line breaks; only header keys
containing a colon are refused. Empty payloads add no body line. Metadata is deterministic.

[Source](/reference-sources/Text.Pem.cinch) · line 683

<a id="value-70656d2d6572726f722d6d657373616765"></a>

## `pem-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pem-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Explain a header refusal or identify a retained underlying stream failure.

[Source](/reference-sources/Text.Pem.cinch) · line 45

<a id="value-70656d2d66696e642d686561646572"></a>

## `pem-find-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pem-find-header</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemBlock</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Find an exact case-sensitive byte key, returning a fresh mutable value when present.

[Source](/reference-sources/Text.Pem.cinch) · line 169

<a id="value-70656d2d686561646572"></a>

## `pem-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pem-header</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemHeader</span>))</code></pre>

Construct immutable byte metadata without validating or discarding unusual keys or values.

[Source](/reference-sources/Text.Pem.cinch) · line 61

<a id="value-70656d2d6865616465722d6b6579"></a>

## `pem-header-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pem-header-key</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemHeader</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe an independently mutable copy of a header key.

[Source](/reference-sources/Text.Pem.cinch) · line 65

<a id="value-70656d2d6865616465722d76616c7565"></a>

## `pem-header-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pem-header-value</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemHeader</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe an independently mutable copy of a header value.

[Source](/reference-sources/Text.Pem.cinch) · line 69

<a id="value-70656d2d776974682d6279746573"></a>

## `pem-with-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pem-with-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemBlock</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemBlock</span>))</code></pre>

Replace payload bytes with an owned snapshot while preserving metadata and the original block.

[Source](/reference-sources/Text.Pem.cinch) · line 211

<a id="value-70656d2d776974682d686561646572"></a>

## `pem-with-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pem-with-header</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemBlock</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemHeader</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemBlock</span>))</code></pre>

Replace or add a header persistently; both blocks retain immutable independently observable metadata.

[Source](/reference-sources/Text.Pem.cinch) · line 181

<a id="value-70656d2d776974682d68656164657273"></a>

## `pem-with-headers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pem-with-headers</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemBlock</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemHeader</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemBlock</span>))</code></pre>

Replace all metadata persistently, with sorted last-key-wins semantics and immutable payload sharing.

[Source](/reference-sources/Text.Pem.cinch) · line 203

<a id="value-70656d2d776974682d74797065"></a>

## `pem-with-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pem-with-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemBlock</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemBlock</span>))</code></pre>

Replace a label with an owned snapshot, preserving the original block and shared immutable fields.

[Source](/reference-sources/Text.Pem.cinch) · line 207

<a id="value-70656d2d776974686f75742d686561646572"></a>

## `pem-without-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pem-without-header</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemBlock</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemBlock</span>))</code></pre>

Remove a byte key persistently without copying unrelated private payload storage.

[Source](/reference-sources/Text.Pem.cinch) · line 185

<a id="value-70656d2d7772697465"></a>

## `pem-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pem-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemBlock</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Write an envelope with bounded payload storage and explicit continuation ownership.
Validate all keys before any callback. Positive short writes are retried; zero progress,
invalid counts and sink errors preserve the advanced writer and never replay accepted
output. Chunk boundaries are unspecified. This operation never flushes or closes its sink.

[Source](/reference-sources/Text.Pem.cinch) · line 753

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Pem.cinch`. Source SHA-256: `9478c3c41a71436180df02c7ac175c309f790f39d131440895f0a81c7f733efd`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
