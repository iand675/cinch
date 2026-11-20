# Effect.Ascii85

ASCII85 encoding writers and decoding byte readers with explicit finalization, bounded read-ahead and sticky stream errors.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Ascii85`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Ascii85.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Stream](Effect.Stream.md), [Text.Ascii85](Text.Ascii85.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Ascii85](Effect.Ascii85.md), [Effect.Stream](Effect.Stream.md), [Text.Ascii85](Text.Ascii85.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Incremental ASCII85 writers and decoding byte readers.
Writers buffer at most three input bytes between writes; finish explicitly
flushes that tail and returns the advanced underlying sink. Neither adapter
closes descriptors. Decoder failures report the encoding name, original byte
offset and diagnostic through StreamEncodingError. Read/write failures are
sticky and never replay already consumed input or accepted output.

</details>

# Types and constructors

<a id="type-41736369693835577269746572"></a>

## `Ascii85Writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Ascii85Writer</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

An explicitly advanced encoding continuation and its underlying sink.

[Source](/reference-sources/Effect.Ascii85.cinch) · line 20

# Values

<a id="value-617363696938352d66696e697368"></a>

## `ascii85-finish`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-finish</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Writer</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Flush a final short quantum and return the underlying sink on either outcome.
The returned sink can be inspected, flushed, closed or used for later unencoded data.

[Source](/reference-sources/Effect.Ascii85.cinch) · line 48

<a id="value-617363696938352d726561646572"></a>

## `ascii85-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Wrap a reader as a decoding source. Zero reads do not advance either source.
The adapter reads bounded 4096-byte input chunks and retains decoded surplus.

[Source](/reference-sources/Effect.Ascii85.cinch) · line 73

<a id="value-617363696938352d7772697465"></a>

## `ascii85-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Writer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Writer</span> <span class="tok-variable">r</span>))))</code></pre>

Accept all input, writing complete quanta and retaining at most three bytes.
Short underlying writes are retried; any failure is retained in the next handle.

[Source](/reference-sources/Effect.Ascii85.cinch) · line 38

<a id="value-617363696938352d777269746572"></a>

## `ascii85-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Writer</span> <span class="tok-variable">r</span>))</code></pre>

Begin encoding to a sink. Finish before inspecting the complete encoded output.

[Source](/reference-sources/Effect.Ascii85.cinch) · line 27

<a id="value-617363696938352d7772697465722d73696e6b"></a>

## `ascii85-writer-sink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii85-writer-sink</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Ascii85</span>.<span class="tok-constructor">Ascii85Writer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))</code></pre>

Observe the advanced underlying sink, including after a failed write.

[Source](/reference-sources/Effect.Ascii85.cinch) · line 31

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Ascii85.cinch`. Source SHA-256: `ce62b87682e7b8b63754a346c01a3db666e0a062693fa958251dac6c89febb27`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
