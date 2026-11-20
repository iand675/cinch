# Effect.Base32

Explicitly finalized Base32 encoding writers and bounded decoding readers with precise errors, retained cursors and sticky failures.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Base32`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Base32.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Effect.Stream](Effect.Stream.md), [Text.Base32](Text.Base32.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Base32](Effect.Base32.md), [Effect.Stream](Effect.Stream.md), [Text.Base32](Text.Base32.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Configurable Base32 encoding writers and strict decoding byte readers.
Writers retain at most four input bytes until explicitly finished. Readers
consume bounded chunks, preserve decoded output before a later error, and
count malformed input in original bytes including CR/LF. Failures are sticky.
Neither adapter closes or flushes a borrowed source or sink.

</details>

# Types and constructors

<a id="type-426173653332577269746572"></a>

## `Base32Writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Base32Writer</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

An explicitly advanced encoding continuation with its owned tail and underlying sink.

[Source](/reference-sources/Effect.Base32.cinch) · line 19

# Values

<a id="value-6261736533322d66696e697368"></a>

## `base32-finish`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-finish</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Writer</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Emit the final short quantum and configured padding, returning the advanced sink on either outcome.
This finalizes the adapter's encoding only; the borrowed sink is not closed or flushed.

[Source](/reference-sources/Effect.Base32.cinch) · line 47

<a id="value-6261736533322d726561646572"></a>

## `base32-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>)))</code></pre>

Begin strict configurable decoding. Zero reads preserve both source and decoder state.

[Source](/reference-sources/Effect.Base32.cinch) · line 71

<a id="value-6261736533322d7772697465"></a>

## `base32-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Writer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Writer</span> <span class="tok-variable">r</span>))))</code></pre>

Accept a chunk, retrying short output writes and copying the unfinished input fringe.
Success reports the complete input byte count. Failure returns a sticky advanced handle.

[Source](/reference-sources/Effect.Base32.cinch) · line 37

<a id="value-6261736533322d777269746572"></a>

## `base32-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Encoding</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Writer</span> <span class="tok-variable">r</span>)))</code></pre>

Begin a configurable encoding. Finish explicitly before observing complete output.

[Source](/reference-sources/Effect.Base32.cinch) · line 26

<a id="value-6261736533322d7772697465722d73696e6b"></a>

## `base32-writer-sink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base32-writer-sink</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Base32</span>.<span class="tok-constructor">Base32Writer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))</code></pre>

Inspect the advanced output handle, including after an unsuccessful encoded write.

[Source](/reference-sources/Effect.Base32.cinch) · line 30

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Base32.cinch`. Source SHA-256: `38f8d4b0a0ac51fed5ed5251d0b16093e8983986ea36330edede81bb69705b90`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
