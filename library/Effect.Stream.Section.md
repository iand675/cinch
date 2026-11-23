# Effect.Stream.Section

Positional byte sources, immutable bounded sections, signed seeks and sequential reader adapters.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Stream.Section`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Stream.Section.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Int](Data.Int.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Effect.Stream](Effect.Stream.md).

Non-embedded source closure: [Effect.Stream](Effect.Stream.md), [Effect.Stream.Section](Effect.Stream.Section.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Positional byte sources and bounded, independently seekable reader views.
All offsets and sizes are natural numbers. Short reads return the available
prefix, and a positive read returning empty bytes means EOF.

</details>

# Types and constructors

<a id="type-427974655265616465724174"></a>

## `ByteReaderAt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ByteReaderAt</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A source addressed by absolute offset. Each call is independent of cursor position.

[Source](/reference-sources/Effect.Stream.Section.cinch) · line 18

<a id="type-4279746553656374696f6e"></a>

## `ByteSection`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ByteSection</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A source, absolute start, declared size, and cursor relative to the start.

[Source](/reference-sources/Effect.Stream.Section.cinch) · line 50

<a id="type-5365656b4f726967696e"></a>

## `SeekOrigin`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SeekOrigin</span></code></pre>

Reference point for a signed seek offset.

[Source](/reference-sources/Effect.Stream.Section.cinch) · line 70

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SeekStart</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Section</span>.<span class="tok-constructor">SeekOrigin</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SeekCurrent</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Section</span>.<span class="tok-constructor">SeekOrigin</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SeekEnd</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Section</span>.<span class="tok-constructor">SeekOrigin</span></code></pre>

# Values

<a id="value-7265616465722d6174"></a>

## `reader-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reader-at</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Bytes</span>))) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Section</span>.<span class="tok-constructor">ByteReaderAt</span> <span class="tok-variable">r</span>))</code></pre>

Wrap a positional read callback. It receives offset and maximum byte count.

[Source](/reference-sources/Effect.Stream.Section.cinch) · line 23

<a id="value-7265616465722d61742d6279746573"></a>

## `reader-at-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reader-at-bytes</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Section</span>.<span class="tok-constructor">ByteReaderAt</span> <span class="tok-variable">r</span>))</code></pre>

Snapshot the caller's bytes. Each successful read returns independently owned bytes.

[Source](/reference-sources/Effect.Stream.Section.cinch) · line 35

<a id="value-7265616465722d73656374696f6e"></a>

## `reader-section`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reader-section</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Section</span>.<span class="tok-constructor">ByteReaderAt</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Section</span>.<span class="tok-constructor">ByteSection</span> <span class="tok-variable">r</span>))))</code></pre>

Create a view at position zero. Its declared size can extend beyond the source.

[Source](/reference-sources/Effect.Stream.Section.cinch) · line 54

<a id="value-73656374696f6e2d6f75746572"></a>

## `section-outer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">section-outer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Section</span>.<span class="tok-constructor">ByteSection</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Section</span>.<span class="tok-constructor">ByteReaderAt</span> <span class="tok-variable">r</span>), (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Nat</span>)))</code></pre>

Recover the original source, absolute start and declared size.

[Source](/reference-sources/Effect.Stream.Section.cinch) · line 58

<a id="value-73656374696f6e2d706f736974696f6e"></a>

## `section-position`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">section-position</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Section</span>.<span class="tok-constructor">ByteSection</span> <span class="tok-variable">r</span>) -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The sequential cursor relative to the beginning of the view.

[Source](/reference-sources/Effect.Stream.Section.cinch) · line 66

<a id="value-73656374696f6e2d72656164"></a>

## `section-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">section-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Section</span>.<span class="tok-constructor">ByteSection</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Bytes</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Section</span>.<span class="tok-constructor">ByteSection</span> <span class="tok-variable">r</span>))))</code></pre>

Advance only by the number of bytes returned. Errors preserve the cursor.

[Source](/reference-sources/Effect.Stream.Section.cinch) · line 92

<a id="value-73656374696f6e2d726561642d6174"></a>

## `section-read-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">section-read-at</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Section</span>.<span class="tok-constructor">ByteSection</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Read at a section-relative offset without changing its cursor or crossing its end.

[Source](/reference-sources/Effect.Stream.Section.cinch) · line 84

<a id="value-73656374696f6e2d726561646572"></a>

## `section-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">section-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Section</span>.<span class="tok-constructor">ByteSection</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Consume a section through the generic sequential reader protocol.

[Source](/reference-sources/Effect.Stream.Section.cinch) · line 99

<a id="value-73656374696f6e2d7365656b"></a>

## `section-seek`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">section-seek</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Section</span>.<span class="tok-constructor">ByteSection</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Section</span>.<span class="tok-constructor">SeekOrigin</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Section</span>.<span class="tok-constructor">ByteSection</span> <span class="tok-variable">r</span>)))))</code></pre>

Return a repositioned view, or None for a negative result. Seeking beyond EOF is valid.

[Source](/reference-sources/Effect.Stream.Section.cinch) · line 76

<a id="value-73656374696f6e2d73697a65"></a>

## `section-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">section-size</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Section</span>.<span class="tok-constructor">ByteSection</span> <span class="tok-variable">r</span>) -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The declared number of bytes in the view.

[Source](/reference-sources/Effect.Stream.Section.cinch) · line 62

<a id="value-73747265616d2d726561642d6174"></a>

## `stream-read-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stream-read-at</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Section</span>.<span class="tok-constructor">ByteReaderAt</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Read a bounded prefix. Zero-length reads skip the callback.

[Source](/reference-sources/Effect.Stream.Section.cinch) · line 27

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Stream/Section.cinch`. Source SHA-256: `ce1b65e973fd2e8d0cbc26d1b9876fa1cd38113fa3843c56cfc229b20954059e`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
