# Effect.Stream.Offset

Positional byte writers, signed cursor movement and streaming at a base offset.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Stream.Offset`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Stream.Offset.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Int](Data.Int.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Effect.Stream](Effect.Stream.md).

Non-embedded source closure: [Effect.Stream](Effect.Stream.md), [Effect.Stream.Offset](Effect.Stream.Offset.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Positional writers and sequential views beginning at an arbitrary byte offset.
A positional callback returns its advanced sink even when a write fails.

</details>

# Types and constructors

<a id="type-427974655772697465724174"></a>

## `ByteWriterAt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ByteWriterAt</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A sink addressed by absolute offset, with explicit continuation state.

[Source](/reference-sources/Effect.Stream.Offset.cinch) · line 16

<a id="type-4f6666736574577269746572"></a>

## `OffsetWriter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">OffsetWriter</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A base offset and relative sequential cursor over a positional sink.

[Source](/reference-sources/Effect.Stream.Offset.cinch) · line 33

# Values

<a id="value-6f66667365742d6f75746572"></a>

## `offset-outer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-outer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">OffsetWriter</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">ByteWriterAt</span> <span class="tok-variable">r</span>), <span class="tok-constructor">Nat</span>))</code></pre>

Recover the latest sink continuation and its original base offset.

[Source](/reference-sources/Effect.Stream.Offset.cinch) · line 45

<a id="value-6f66667365742d706f736974696f6e"></a>

## `offset-position`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-position</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">OffsetWriter</span> <span class="tok-variable">r</span>) -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Current position relative to the base.

[Source](/reference-sources/Effect.Stream.Offset.cinch) · line 41

<a id="value-6f66667365742d7365656b2d63757272656e74"></a>

## `offset-seek-current`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-seek-current</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">OffsetWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">OffsetWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Move the cursor by a signed distance. A position before the base returns None.

[Source](/reference-sources/Effect.Stream.Offset.cinch) · line 53

<a id="value-6f66667365742d7365656b2d7374617274"></a>

## `offset-seek-start`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-seek-start</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">OffsetWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">OffsetWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Set an absolute position within the view.

[Source](/reference-sources/Effect.Stream.Offset.cinch) · line 49

<a id="value-6f66667365742d73747265616d2d777269746572"></a>

## `offset-stream-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-stream-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">OffsetWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))</code></pre>

Adapt an offset writer to generic streaming and full-write retry operations.

[Source](/reference-sources/Effect.Stream.Offset.cinch) · line 73

<a id="value-6f66667365742d7772697465"></a>

## `offset-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">OffsetWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">OffsetWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Advance the cursor by the accepted count. On an error the cursor stays at
the last confirmed position while the sink continues from its returned state.

[Source](/reference-sources/Effect.Stream.Offset.cinch) · line 66

<a id="value-6f66667365742d77726974652d6174"></a>

## `offset-write-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-write-at</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">OffsetWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">OffsetWriter</span> <span class="tok-variable">r</span>)))))</code></pre>

Write at a relative offset, preserving the cursor and retaining the advanced sink.

[Source](/reference-sources/Effect.Stream.Offset.cinch) · line 59

<a id="value-6f66667365742d777269746572"></a>

## `offset-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">offset-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">ByteWriterAt</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">OffsetWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Start a sequential writer at relative position zero.

[Source](/reference-sources/Effect.Stream.Offset.cinch) · line 37

<a id="value-73747265616d2d77726974652d6174"></a>

## `stream-write-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stream-write-at</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">ByteWriterAt</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">ByteWriterAt</span> <span class="tok-variable">r</span>)))))</code></pre>

Submit bytes at an absolute offset. Empty writes preserve the sink, and
an invalid reported count returns an error together with the advanced sink.

[Source](/reference-sources/Effect.Stream.Offset.cinch) · line 25

<a id="value-7772697465722d6174"></a>

## `writer-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">writer-at</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">ByteWriterAt</span> <span class="tok-variable">r</span>)))) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">Offset</span>.<span class="tok-constructor">ByteWriterAt</span> <span class="tok-variable">r</span>))</code></pre>

Wrap a positional callback reporting the count accepted and the next sink.

[Source](/reference-sources/Effect.Stream.Offset.cinch) · line 20

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Stream/Offset.cinch`. Source SHA-256: `1828e5e2f5547face4ed67fdbeeba348c9a52e3679de240a8d79003904c48bbd`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
