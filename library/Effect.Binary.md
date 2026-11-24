# Effect.Binary

Explicit fixed and variable binary stream codecs preserving progress and advanced handles after failures.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Binary`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Binary.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Stream](Effect.Stream.md), [Text.Binary.Layout](Text.Binary.Layout.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Complex](Data.Complex.md), [Data.Float](Data.Float.md), [Effect.Binary](Effect.Binary.md), [Effect.Stream](Effect.Stream.md), [Text.Binary](Text.Binary.md), [Text.Binary.Layout](Text.Binary.Layout.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Typed binary stream reads and writes with explicit advanced reader/writer continuations.
Fixed reads decode only after a complete private frame is available. Empty EOF
is distinct from truncation and transport failures. Varint readers consume at
most ten bytes and never read into the next frame. Writers preserve known
progress on failure and adapt retry sizes so tiny short writes do not cause
repeated whole-remainder copying. Borrowed streams are never closed or flushed.

</details>

# Types and constructors

<a id="type-42696e61727953747265616d4572726f72"></a>

## `BinaryStreamError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BinaryStreamError</span></code></pre>

Binary stream failures retain either transport progress or an explicit typed-format error.

[Source](/reference-sources/Effect.Binary.cinch) · line 22

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BinaryEndOfInput</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryStreamError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BinaryInputFailure</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryStreamError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BinaryOutputFailure</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryStreamError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BinaryFormatFailure</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryStreamError</span>)</code></pre>

# Values

<a id="value-62696e6172792d72656164"></a>

## `binary-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryStreamError</span> <span class="tok-variable">a</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>)))))</code></pre>

Read one fixed layout, returning the advanced source on success, EOF or any failure.
Zero-sized layouts do not invoke the source. A reused callback buffer is copied before advancing.

[Source](/reference-sources/Effect.Binary.cinch) · line 56

<a id="value-62696e6172792d726561642d75766172696e74"></a>

## `binary-read-uvarint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-read-uvarint</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryStreamError</span> (<span class="tok-constructor">U64</span>, <span class="tok-constructor">Nat</span>)), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>)))</code></pre>

Read one unsigned base-128 integer, stopping at its terminator or after ten consumed bytes.
Empty EOF is BinaryEndOfInput; partial EOF preserves its count in BinaryInputFailure.

[Source](/reference-sources/Effect.Binary.cinch) · line 114

<a id="value-62696e6172792d726561642d766172696e74"></a>

## `binary-read-varint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-read-varint</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryStreamError</span> (<span class="tok-constructor">I64</span>, <span class="tok-constructor">Nat</span>)), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>)))</code></pre>

Read and inverse-zigzag one signed integer without reading bytes from the following frame.

[Source](/reference-sources/Effect.Binary.cinch) · line 140

<a id="value-62696e6172792d73747265616d2d6572726f722d636f756e74"></a>

## `binary-stream-error-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-stream-error-count</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryStreamError</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Known completed input or output bytes associated with a transport failure; other errors return zero.

[Source](/reference-sources/Effect.Binary.cinch) · line 33

<a id="value-62696e6172792d73747265616d2d6572726f722d6d657373616765"></a>

## `binary-stream-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-stream-error-message</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryStreamError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Describe EOF, guaranteed stream progress or the underlying typed-format failure.
Transport causes remain inspectable in the error constructor.

[Source](/reference-sources/Effect.Binary.cinch) · line 41

<a id="value-62696e6172792d7772697465"></a>

## `binary-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryStreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))))))</code></pre>

Write a whole private encoding, returning accepted wire bytes and the advanced writer.
Invalid values fail before touching the writer; zero-sized values do not invoke it.

[Source](/reference-sources/Effect.Binary.cinch) · line 80

<a id="value-62696e6172792d77726974652d75766172696e74"></a>

## `binary-write-uvarint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-write-uvarint</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">U64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryStreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Write one minimal unsigned varint, preserving accepted-byte progress and the advanced sink.

[Source](/reference-sources/Effect.Binary.cinch) · line 149

<a id="value-62696e6172792d77726974652d766172696e74"></a>

## `binary-write-varint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-write-varint</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">I64</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryStreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Write one minimal signed zigzag varint, preserving accepted-byte progress and the advanced sink.

[Source](/reference-sources/Effect.Binary.cinch) · line 153

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Binary.cinch`. Source SHA-256: `ccc9fe29127f391045105575e80fb446b3452798f2e3166f939521c51b10470c`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
