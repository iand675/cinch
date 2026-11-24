# Effect.QuotedPrintable

Incremental quoted-printable readers and writers with explicit binary policy, partial progress and sticky errors.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.QuotedPrintable`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.QuotedPrintable.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Effect.Stream](Effect.Stream.md), [Text.QuotedPrintable](Text.QuotedPrintable.md).

Non-embedded source closure: [Effect.QuotedPrintable](Effect.QuotedPrintable.md), [Effect.Stream](Effect.Stream.md), [Text.QuotedPrintable](Text.QuotedPrintable.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Quoted-printable stream continuations with explicit input and error progress.
Writers retain an immutable partial line and return the advanced borrowed sink.
Readers preserve decoded bytes returned alongside a terminal status. The generic
ByteReader bridge drains those bytes before reporting the error. Zero reads and
empty byte writes follow Effect.Stream's inert-operation contract.

</details>

# Types and constructors

<a id="type-51756f7465645072696e7461626c6552656164537461747573"></a>

## `QuotedPrintableReadStatus`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">QuotedPrintableReadStatus</span></code></pre>

Status returned alongside decoded bytes, so a partial read never loses its prefix.

[Source](/reference-sources/Effect.QuotedPrintable.cinch) · line 89

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">QuotedPrintableReady</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableReadStatus</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">QuotedPrintableEof</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableReadStatus</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">QuotedPrintableReadFailure</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableReadStatus</span>)</code></pre>

<a id="type-51756f7465645072696e7461626c65526561646572"></a>

## `QuotedPrintableReader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">QuotedPrintableReader</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Explicit decoder continuation owning its line state and decoded surplus.
Always continue with the returned handle; descriptor sources share their OS cursor.

[Source](/reference-sources/Effect.QuotedPrintable.cinch) · line 99

<a id="type-51756f7465645072696e7461626c65577269746572"></a>

## `QuotedPrintableWriter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">QuotedPrintableWriter</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

An explicitly advanced encoder and its borrowed output sink.

[Source](/reference-sources/Effect.QuotedPrintable.cinch) · line 23

# Values

<a id="value-71756f7465642d7072696e7461626c652d66696e697368"></a>

## `quoted-printable-finish`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-finish</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableWriter</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Quote the final trailing whitespace, flush retained bytes and return the
advanced sink on either outcome. The borrowed sink is never closed. Use the
returned sink for later unencoded data; the encoder handle is now complete.

[Source](/reference-sources/Effect.QuotedPrintable.cinch) · line 80

<a id="value-71756f7465642d7072696e7461626c652d72656164"></a>

## `quoted-printable-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Effect</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableReadStatus</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableReader</span> <span class="tok-variable">r</span>))))</code></pre>

Read up to the request while preserving a simultaneous partial prefix and
terminal status. EOF and failures remain sticky; zero reads are always inert.

[Source](/reference-sources/Effect.QuotedPrintable.cinch) · line 146

<a id="value-71756f7465642d7072696e7461626c652d726561646572"></a>

## `quoted-printable-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableReader</span> <span class="tok-variable">r</span>))</code></pre>

Begin decoding an independently owned continuation over a borrowed source.

[Source](/reference-sources/Effect.QuotedPrintable.cinch) · line 104

<a id="value-71756f7465642d7072696e7461626c652d7265616465722d61732d73747265616d"></a>

## `quoted-printable-reader-as-stream`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-reader-as-stream</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Adapt explicit read statuses to the generic byte-reader protocol. A
partial prefix is returned first; its error is retained for the next read.
Neither returned chunks nor later calls can mutate codec continuation state.

[Source](/reference-sources/Effect.QuotedPrintable.cinch) · line 200

<a id="value-71756f7465642d7072696e7461626c652d7265616465722d736f75726365"></a>

## `quoted-printable-reader-source`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-reader-source</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Observe the source after its exact buffered read-ahead, even after failure.

[Source](/reference-sources/Effect.QuotedPrintable.cinch) · line 108

<a id="value-71756f7465642d7072696e7461626c652d7772697465"></a>

## `quoted-printable-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span>) <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Accept one input buffer and return its reportable input count. On failure
the error carries that count and the advanced handle retains the exact sink
prefix. Printable batches count only after their entire batch succeeds.
Short underlying writes are retried; failures are sticky and never replayed.
An empty input succeeds with zero and leaves even a failed handle untouched.

[Source](/reference-sources/Effect.QuotedPrintable.cinch) · line 66

<a id="value-71756f7465642d7072696e7461626c652d777269746572"></a>

## `quoted-printable-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableMode</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Begin writing with an explicit text or binary newline policy.

[Source](/reference-sources/Effect.QuotedPrintable.cinch) · line 30

<a id="value-71756f7465642d7072696e7461626c652d7772697465722d6d6f6465"></a>

## `quoted-printable-writer-mode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-writer-mode</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableMode</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Change the mode for subsequent input without flushing the retained line.

[Source](/reference-sources/Effect.QuotedPrintable.cinch) · line 34

<a id="value-71756f7465642d7072696e7461626c652d7772697465722d706f6c696379"></a>

## `quoted-printable-writer-policy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-writer-policy</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableWriter</span> <span class="tok-variable">r</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableMode</span>)</code></pre>

Observe the current policy, including after failure or a later policy update.

[Source](/reference-sources/Effect.QuotedPrintable.cinch) · line 46

<a id="value-71756f7465642d7072696e7461626c652d7772697465722d73696e6b"></a>

## `quoted-printable-writer-sink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quoted-printable-writer-sink</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">QuotedPrintable</span>.<span class="tok-constructor">QuotedPrintableWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))</code></pre>

Observe the advanced borrowed sink, including after a failed write.

[Source](/reference-sources/Effect.QuotedPrintable.cinch) · line 40

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/QuotedPrintable.cinch`. Source SHA-256: `10a4170d30c13f663f72ed741df8f381ae6098fe230055098f65ec607c66fd97`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
