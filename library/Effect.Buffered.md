# Effect.Buffered

Buffered input and output components with explicit independent advanced handles.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Buffered`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Buffered.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Effect.Buffered.Reader](Effect.Buffered.Reader.md), [Effect.Buffered.Writer](Effect.Buffered.Writer.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Effect.Buffered](Effect.Buffered.md), [Effect.Buffered.Core](Effect.Buffered.Core.md), [Effect.Buffered.Internal.Cell](Effect.Buffered.Internal.Cell.md), [Effect.Buffered.Internal.Scan](Effect.Buffered.Internal.Scan.md), [Effect.Buffered.Reader](Effect.Buffered.Reader.md), [Effect.Buffered.Writer](Effect.Buffered.Writer.md), [Effect.Stream](Effect.Stream.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Buffered input and output components with explicit independent advanced handles.
Reading and writing preserve each other's state. Apply a component operation,
then install its returned component to continue through the combined handle.

</details>

# Types and constructors

<a id="type-427566666572656452656164577269746572"></a>

## `BufferedReadWriter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BufferedReadWriter</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A paired reader and writer over the same I/O region, with no implicit flushing or read-ahead coupling.

[Source](/reference-sources/Effect.Buffered.cinch) · line 13

# Values

<a id="value-62756666657265642d726561642d777269746572"></a>

## `buffered-read-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-read-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">BufferedReadWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Pair existing buffered components without reading, writing or flushing either one.

[Source](/reference-sources/Effect.Buffered.cinch) · line 18

<a id="value-62756666657265642d726561642d7772697465722d726561646572"></a>

## `buffered-read-writer-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-read-writer-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">BufferedReadWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>))</code></pre>

Observe the current reader component; continue with the advanced reader returned by its next operation.

[Source](/reference-sources/Effect.Buffered.cinch) · line 22

<a id="value-62756666657265642d726561642d7772697465722d776974682d726561646572"></a>

## `buffered-read-writer-with-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-read-writer-with-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">BufferedReadWriter</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">BufferedReadWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Install an advanced reader while preserving the independently advancing writer.

[Source](/reference-sources/Effect.Buffered.cinch) · line 30

<a id="value-62756666657265642d726561642d7772697465722d776974682d777269746572"></a>

## `buffered-read-writer-with-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-read-writer-with-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">BufferedReadWriter</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">BufferedReadWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Install an advanced writer while preserving unread input, source errors and reader history.

[Source](/reference-sources/Effect.Buffered.cinch) · line 34

<a id="value-62756666657265642d726561642d7772697465722d777269746572"></a>

## `buffered-read-writer-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-read-writer-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">BufferedReadWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>))</code></pre>

Observe the current writer component without performing I/O or implicitly flushing buffered bytes.

[Source](/reference-sources/Effect.Buffered.cinch) · line 26

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Buffered.cinch`. Source SHA-256: `a9669f4c6996da84707ead19c237498e0b043fcac8dc1e4cc945ccaecff22054`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
