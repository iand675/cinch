# Effect.Stream

Composable byte readers and writers with explicit continuation state, memory and descriptor adapters, limits, mirrors, and exact or bulk transfers.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Stream`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Stream.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Prim.Sys](Prim.Sys.md).

Non-embedded source closure: [Effect.Stream](Effect.Stream.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Composable byte readers and writers with explicit continuation state.
Every operation returns its advanced handle, including on failure. A positive
read returning an empty buffer means end of input; zero-length reads never
invoke a source. Writers report the number accepted and may write a prefix.
Range copies use the byte-buffer bulk primitive. Memory sources snapshot
their input, and memory sinks snapshot each accepted chunk, so later mutation
cannot alter data retained by a stream. Descriptor adapters borrow their
descriptor: the caller remains responsible for closing it.

</details>

# Types and constructors

<a id="type-42797465526561646572"></a>

## `ByteReader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ByteReader</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A source whose callback returns both its result and next continuation.
A callback must return no more than the requested number of bytes.

[Source](/reference-sources/Effect.Stream.cinch) · line 33

<a id="type-42797465577269746572"></a>

## `ByteWriter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ByteWriter</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A sink accepting chunks. The advanced handle is returned after every write.

[Source](/reference-sources/Effect.Stream.cinch) · line 40

<a id="type-53747265616d4572726f72"></a>

## `StreamError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">StreamError</span></code></pre>

Named stream failures. Short operations carry their actual progress.

[Source](/reference-sources/Effect.Stream.cinch) · line 59

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StreamSystemError</span> :: (<span class="tok-constructor">Prim</span>.<span class="tok-constructor">Sys</span>.<span class="tok-constructor">Linux</span>.<span class="tok-constructor">SysError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StreamUnexpectedEnd</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StreamShortWrite</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StreamInvalidCount</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StreamInvalidChunkSize</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">StreamEncodingError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span>)))</code></pre>

# Values

<a id="value-7265616465722d636f6e636174"></a>

## `reader-concat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reader-concat</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>)) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Read sources in order, skipping exhausted sources. An empty list is EOF.

[Source](/reference-sources/Effect.Stream.cinch) · line 165

<a id="value-7265616465722d66726f6d2d6279746573"></a>

## `reader-from-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reader-from-bytes</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Snapshot a byte buffer and start reading at its beginning.

[Source](/reference-sources/Effect.Stream.cinch) · line 102

<a id="value-7265616465722d66726f6d2d6664"></a>

## `reader-from-fd`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reader-from-fd</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Borrow a descriptor at its current position. Reads advance its OS cursor;
duplicating this reader shares that cursor. This adapter never closes it.

[Source](/reference-sources/Effect.Stream.cinch) · line 129

<a id="value-7265616465722d66726f6d2d737472696e67"></a>

## `reader-from-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reader-from-string</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Read UTF-8 bytes of text, starting at its beginning.

[Source](/reference-sources/Effect.Stream.cinch) · line 106

<a id="value-7265616465722d6c696d6974"></a>

## `reader-limit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reader-limit</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>)))</code></pre>

Expose at most limit more bytes without reading beyond that limit.

[Source](/reference-sources/Effect.Stream.cinch) · line 151

<a id="value-7265616465722d746565"></a>

## `reader-tee`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reader-tee</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>)))</code></pre>

Mirror every returned chunk to a sink before delivering it to the reader.
No buffering occurs; a sink failure is returned with both continuations advanced.

[Source](/reference-sources/Effect.Stream.cinch) · line 371

<a id="value-7265616465722d7465652d777269746572"></a>

## `reader-tee-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reader-tee-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Inspect a tee reader's advanced mirror sink, including after read failures.
This exposes persistent memory-sink results without implicit shared mutation.

[Source](/reference-sources/Effect.Stream.cinch) · line 376

<a id="value-73747265616d2d636f7079"></a>

## `stream-copy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stream-copy</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))))))</code></pre>

Copy until EOF with a positive chunk size. Return bytes transferred and
both advanced handles; failures preserve those handles as well.

[Source](/reference-sources/Effect.Stream.cinch) · line 335

<a id="value-73747265616d2d636f70792d6e"></a>

## `stream-copy-n`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stream-copy-n</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>)))))))</code></pre>

Copy exactly count bytes without reading beyond that boundary. An early
EOF is a named error. Both returned handles reflect completed operations.

[Source](/reference-sources/Effect.Stream.cinch) · line 352

<a id="value-73747265616d2d72656164"></a>

## `stream-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stream-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Bytes</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))))</code></pre>

Read at most n bytes and always return the advanced reader. An empty
successful positive read means EOF. A zero request leaves the reader alone.

[Source](/reference-sources/Effect.Stream.cinch) · line 83

<a id="value-73747265616d2d726561642d616c6c"></a>

## `stream-read-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stream-read-all</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Bytes</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))))</code></pre>

Read until EOF using positive-sized chunks. Every chunk is copied once
into the final output; failures still return the advanced reader.

[Source](/reference-sources/Effect.Stream.cinch) · line 320

<a id="value-73747265616d2d726561642d61742d6c65617374"></a>

## `stream-read-at-least`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stream-read-at-least</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Bytes</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>)))))</code></pre>

Read a minimum into a maximum-sized result. A minimum greater than the
maximum is rejected before invoking the reader.

[Source](/reference-sources/Effect.Stream.cinch) · line 302

<a id="value-73747265616d2d726561642d6578616374"></a>

## `stream-read-exact`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stream-read-exact</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Bytes</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))))</code></pre>

Read exactly n bytes or report how many were obtained before EOF.

[Source](/reference-sources/Effect.Stream.cinch) · line 297

<a id="value-73747265616d2d726561646572"></a>

## `stream-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stream-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Bytes</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Wrap a custom effectful reader. stream-read validates its result length.

[Source](/reference-sources/Effect.Stream.cinch) · line 74

<a id="value-73747265616d2d7772697465"></a>

## `stream-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stream-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Attempt one write and return the advanced sink. Empty input does not invoke
callbacks. Counts larger than the input are rejected as protocol errors.

[Source](/reference-sources/Effect.Stream.cinch) · line 206

<a id="value-73747265616d2d77726974652d616c6c"></a>

## `stream-write-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stream-write-all</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Write a full chunk despite short writes. Zero progress becomes a named
short-write error instead of an infinite retry. The next sink is retained.

[Source](/reference-sources/Effect.Stream.cinch) · line 244

<a id="value-73747265616d2d77726974652d737472696e67"></a>

## `stream-write-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stream-write-string</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Encode text as UTF-8 and completely write the resulting bytes.

[Source](/reference-sources/Effect.Stream.cinch) · line 262

<a id="value-73747265616d2d777269746572"></a>

## `stream-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">stream-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))</code></pre>

Wrap a custom effectful writer. stream-write validates its accepted count.

[Source](/reference-sources/Effect.Stream.cinch) · line 78

<a id="value-7772697465722d627566666572"></a>

## `writer-buffer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">writer-buffer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>)</code></pre>

Create an empty persistent memory sink. Inspect its contents with writer-bytes.

[Source](/reference-sources/Effect.Stream.cinch) · line 179

<a id="value-7772697465722d6279746573"></a>

## `writer-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">writer-bytes</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Flatten copied memory chunks once into a fresh buffer. Other sinks return None.

[Source](/reference-sources/Effect.Stream.cinch) · line 220

<a id="value-7772697465722d636f6d706f6e656e7473"></a>

## `writer-components`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">writer-components</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Inspect the advanced children of a broadcast sink. Other sink types return None.

[Source](/reference-sources/Effect.Stream.cinch) · line 283

<a id="value-7772697465722d636f6e636174"></a>

## `writer-concat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">writer-concat</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>)) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))</code></pre>

Broadcast each chunk in list order. Empty lists accept and discard it.
An error stops before subsequent sinks, retaining already advanced sinks.

[Source](/reference-sources/Effect.Stream.cinch) · line 267

<a id="value-7772697465722d64697363617264"></a>

## `writer-discard`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">writer-discard</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>)</code></pre>

Create a sink that accepts all input without retaining it.

[Source](/reference-sources/Effect.Stream.cinch) · line 183

<a id="value-7772697465722d66726f6d2d6664"></a>

## `writer-from-fd`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">writer-from-fd</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))</code></pre>

Borrow a descriptor for writing; the caller retains close responsibility.

[Source](/reference-sources/Effect.Stream.cinch) · line 187

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Stream.cinch`. Source SHA-256: `6079746ad326c94b72c6c3b41f3cb5d652d67a0f7ced213031747e6e342bd7c1`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
