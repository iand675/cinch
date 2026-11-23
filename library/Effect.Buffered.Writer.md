# Effect.Buffered.Writer

Buffered byte output with sticky failures, exact partial progress and independently owned reservations.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Buffered.Writer`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Buffered.Writer.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Buffered.Core](Effect.Buffered.Core.md), [Effect.Io](Effect.Io.md), [Effect.Stream](Effect.Stream.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Buffered.Core](Effect.Buffered.Core.md), [Effect.Buffered.Internal.Cell](Effect.Buffered.Internal.Cell.md), [Effect.Buffered.Writer](Effect.Buffered.Writer.md), [Effect.Stream](Effect.Stream.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Buffered byte output with sticky failures, exact partial progress and independently owned reservations.
Buffered input is snapshotted into immutable chunk spines. Flushes join once,
retain an unwritten suffix on failure, and never expose private buffer storage.

</details>

# Types and constructors

<a id="type-4275666665725370616365"></a>

## `BufferSpace`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BufferSpace</span></code></pre>

A fresh bounded reservation snapshots appended bytes independently of any writer's private buffer.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 35

<a id="type-4275666665726564577269746572"></a>

## `BufferedWriter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BufferedWriter</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

An advanced buffered output with immutable private pending chunks and sticky failure state.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 30

# Values

<a id="value-6275666665722d73706163652d617070656e64"></a>

## `buffer-space-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-space-append</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferSpace</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferSpace</span>)))</code></pre>

Snapshot bytes into a reservation, refusing capacity overflow without changing either prior value or caller bytes.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 206

<a id="value-6275666665722d73706163652d6279746573"></a>

## `buffer-space-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-space-bytes</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferSpace</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Produce fresh mutable bytes from a reservation; mutation cannot affect earlier observations or its stored pieces.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 213

<a id="value-6275666665722d73706163652d6361706163697479"></a>

## `buffer-space-capacity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-space-capacity</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferSpace</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe a reservation's advertised capacity, independent of later writer changes.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 198

<a id="value-6275666665722d73706163652d6c656e677468"></a>

## `buffer-space-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-space-length</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferSpace</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe the exact byte length appended to a reservation so far.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 202

<a id="value-6275666665722d73706163652d72657365727665"></a>

## `buffer-space-reserve`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-space-reserve</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferSpace</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferSpace</span>))</code></pre>

Increase a fresh reservation's advertised capacity without copying earlier immutable pieces or affecting its originating writer.
This is explicit growth for callers that need to append beyond the writer's originally available capacity.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 309

<a id="value-62756666657265642d666c757368"></a>

## `buffered-flush`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-flush</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Offer all pending bytes once, converting unexplained short writes into sticky BufferShortWrite failures.
A partial failure retains exactly the unaccepted suffix in fresh private storage.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 93

<a id="value-62756666657265642d666c7573682d7468726f756768"></a>

## `buffered-flush-through`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-flush-through</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Flush this buffer and then any optional underlying flush capability, retaining advanced handles and failures at every layer.
Ordinary buffered-flush remains a single-layer operation. This explicit
operation drains nested buffered-writer adapters all the way to their transport.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 321

<a id="value-62756666657265642d726561642d66726f6d"></a>

## `buffered-read-from`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-read-from</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">TransferEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>))))</code></pre>

Buffer a complete source, dispatching an underlying sink's optional direct-read capability only after pending output is flushed.
End-of-input succeeds and flushes an exactly full final buffer; a partially full
final buffer remains pending. Source failures retain their accepted prefix without becoming sticky sink failures.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 294

<a id="value-62756666657265642d7772697465"></a>

## `buffered-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">WriteEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Write arbitrary bytes, directly forwarding a large input when the buffer is empty and retaining smaller suffixes.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 145

<a id="value-62756666657265642d77726974652d62797465"></a>

## `buffered-write-byte`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-write-byte</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">U8</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Buffer one byte, flushing a full buffer first; a preceding sticky error accepts no byte.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 157

<a id="value-62756666657265642d77726974652d63686172"></a>

## `buffered-write-char`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-write-char</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Char</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">WriteEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Write a scalar character using its exact shortest UTF-8 encoding.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 190

<a id="value-62756666657265642d77726974652d72756e65"></a>

## `buffered-write-rune`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-write-rune</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">I32</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">WriteEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Write a numeric Unicode scalar; invalid, negative and surrogate values encode one replacement scalar.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 181

<a id="value-62756666657265642d77726974652d7370616365"></a>

## `buffered-write-space`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-write-space</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferSpace</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">WriteEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Commit a reservation through the ordinary checked writer, even if another write advanced the original writer meanwhile.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 217

<a id="value-62756666657265642d77726974652d74657874"></a>

## `buffered-write-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-write-text</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">WriteEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Write a valid immutable string and report accepted encoded bytes, preserving sticky write failure semantics.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 153

<a id="value-62756666657265642d77726974652d746578742d6279746573"></a>

## `buffered-write-text-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-write-text-bytes</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">WriteEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Write arbitrary text bytes through fixed-buffer string semantics without requiring UTF-8 validity.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 149

<a id="value-62756666657265642d777269746572"></a>

## `buffered-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>))</code></pre>

Buffer a raw sink using the default 4096-byte capacity.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 46

<a id="value-62756666657265642d7772697465722d61732d73696e6b"></a>

## `buffered-writer-as-sink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-writer-as-sink</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>))</code></pre>

Forward a buffered writer as a raw sink with both direct-read and arbitrary-byte text-write capabilities.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 221

<a id="value-62756666657265642d7772697465722d617661696c61626c65"></a>

## `buffered-writer-available`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-writer-available</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe remaining private buffer capacity, excluding bytes already accepted into the buffer.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 70

<a id="value-62756666657265642d7772697465722d6275666665726564"></a>

## `buffered-writer-buffered`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-writer-buffered</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe pending bytes that still require a successful flush.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 66

<a id="value-62756666657265642d7772697465722d66696e697368"></a>

## `buffered-writer-finish`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-writer-finish</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>)))</code></pre>

Finish buffered output through owned sink layers and return their advanced transport capability without private buffer exposure.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 331

<a id="value-62756666657265642d7772697465722d66726f6d2d73747265616d"></a>

## `buffered-writer-from-stream`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-writer-from-stream</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>))</code></pre>

Buffer a stream sink while preserving its original failure and short-count event semantics.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 54

<a id="value-62756666657265642d7772697465722d66726f6d2d73747265616d2d73697a6564"></a>

## `buffered-writer-from-stream-sized`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-writer-from-stream-sized</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Int</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Select explicit output capacity while adapting an existing stream sink.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 58

<a id="value-62756666657265642d7772697465722d7265627566666572"></a>

## `buffered-writer-rebuffer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-writer-rebuffer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Int</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Reuse a sufficiently large writer; otherwise add an independent outer buffer around its advancing sink.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 243

<a id="value-62756666657265642d7772697465722d7265736574"></a>

## `buffered-writer-reset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-writer-reset</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Replace the underlying sink, discard pending data and clear sticky failure while retaining capacity.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 74

<a id="value-62756666657265642d7772697465722d72657365742d73656c66"></a>

## `buffered-writer-reset-self`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-writer-reset-self</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>))</code></pre>

Resetting a buffered output to itself is an explicit no-op.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 78

<a id="value-62756666657265642d7772697465722d73696e6b"></a>

## `buffered-writer-sink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-writer-sink</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>))</code></pre>

Observe the authoritative underlying sink after applying the buffered writer's returned advanced handle.
This exposes transport capabilities, never private buffered bytes or storage.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 315

<a id="value-62756666657265642d7772697465722d73697a65"></a>

## `buffered-writer-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-writer-size</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe the fixed output buffer capacity without invoking a sink.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 62

<a id="value-62756666657265642d7772697465722d73697a6564"></a>

## `buffered-writer-sized`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-writer-sized</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Int</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Select any positive capacity; zero and negative requests select the default 4096 bytes.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 50

<a id="value-62756666657265642d7772697465722d7370616365"></a>

## `buffered-writer-space`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-writer-space</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferedWriter</span> <span class="tok-variable">r</span>) -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Writer</span>.<span class="tok-constructor">BufferSpace</span>)</code></pre>

Obtain a fresh empty bounded reservation advertising the writer's currently unused capacity.

[Source](/reference-sources/Effect.Buffered.Writer.cinch) · line 194

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Buffered/Writer.cinch`. Source SHA-256: `bf3bda7ec2866c61b178b17ff37b7a52a202a9e0d86b0a44cea0069cd2e2382a`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
