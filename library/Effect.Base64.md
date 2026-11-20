# Effect.Base64

Explicitly finalized Base64 writers and bounded decoding readers with sticky errors and isolated output chunks.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Base64`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Base64.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Effect.Stream](Effect.Stream.md), [Text.Base64.Encoding](Text.Base64.Encoding.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Base64](Effect.Base64.md), [Effect.Stream](Effect.Stream.md), [Text.Base64.Encoding](Text.Base64.Encoding.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Incremental Base64 writers and decoding byte readers with explicit continuations.
Writers retain at most two copied input bytes and finish explicitly. Readers
delay failures until preceding complete output is drained. Zero reads do not
advance either source; errors remain sticky. No adapter closes a borrowed sink.

</details>

# Types and constructors

<a id="type-426173653634577269746572"></a>

## `Base64Writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Base64Writer</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Inspectable encoding continuation retaining the advanced underlying sink on failure.

[Source](/reference-sources/Effect.Base64.cinch) · line 18

# Values

<a id="value-6261736536342d66696e697368"></a>

## `base64-finish`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-finish</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Base64Writer</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Emit the final short quantum and return the underlying sink on either outcome.
This does not flush or close the borrowed sink; later unencoded writes remain possible.

[Source](/reference-sources/Effect.Base64.cinch) · line 46

<a id="value-6261736536342d726561646572"></a>

## `base64-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>)))</code></pre>

Decode bounded source chunks through EOF, preserving complete output before a later failure.
CR/LF are ignored, strictness follows the selected encoding, and zero reads are inert.

[Source](/reference-sources/Effect.Base64.cinch) · line 69

<a id="value-6261736536342d7772697465"></a>

## `base64-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Base64Writer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Base64Writer</span> <span class="tok-variable">r</span>))))</code></pre>

Accept complete input, retrying short encoded writes and retaining at most two input bytes.
Failures preserve the advanced sink and never replay its accepted encoded prefix.

[Source](/reference-sources/Effect.Base64.cinch) · line 36

<a id="value-6261736536342d777269746572"></a>

## `base64-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Encoding</span>.<span class="tok-constructor">Base64Encoding</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Base64Writer</span> <span class="tok-variable">r</span>)))</code></pre>

Begin encoding to a sink. Finish before observing the complete encoded output.

[Source](/reference-sources/Effect.Base64.cinch) · line 25

<a id="value-6261736536342d7772697465722d73696e6b"></a>

## `base64-writer-sink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">base64-writer-sink</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Base64</span>.<span class="tok-constructor">Base64Writer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))</code></pre>

Observe the advanced sink, including after a failed write.

[Source](/reference-sources/Effect.Base64.cinch) · line 29

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Base64.cinch`. Source SHA-256: `4cb4efcac5c5f5cc20a4db01503a21baf397adb8a0c1104a2ee19a0ebf773231`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
