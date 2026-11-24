# Effect.Buffered.Core

Byte-stream events with signed progress, simultaneous failures, shared advanced continuations and explicit transfer capabilities.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Buffered.Core`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Buffered.Core.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Int](Data.Int.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Buffered.Internal.Cell](Effect.Buffered.Internal.Cell.md), [Effect.Io](Effect.Io.md), [Effect.Stream](Effect.Stream.md).

Non-embedded source closure: [Effect.Buffered.Core](Effect.Buffered.Core.md), [Effect.Buffered.Internal.Cell](Effect.Buffered.Internal.Cell.md), [Effect.Stream](Effect.Stream.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Explicit byte-stream events, advanced continuations and transfer capabilities for buffered I/O.
Raw callbacks report signed counts and may return data together with a failure.
Buffered adapters validate those counts before reading or copying caller storage.

</details>

# Types and constructors

<a id="type-4275666665724572726f72"></a>

## `BufferError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BufferError</span></code></pre>

A stream boundary, capacity refusal, invalid operation or retained transport failure.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 32

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BufferEnd</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BufferFull</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BufferNegativeCount</span> :: (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BufferInvalidUnreadByte</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BufferInvalidUnreadRune</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BufferInvalidReadCount</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span>)))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BufferInvalidWriteCount</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BufferNoProgress</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BufferShortWrite</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BufferSourceError</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BufferSinkError</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BufferCustomError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span>)</code></pre>

<a id="type-42756666657253696e6b"></a>

## `BufferSink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BufferSink</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A sink continuation with an optional direct-from-source transfer capability.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 100

<a id="type-427566666572536f75726365"></a>

## `BufferSource`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BufferSource</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A source continuation with an optional direct-to-sink transfer capability.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 91

<a id="type-526561644576656e74"></a>

## `ReadEvent`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ReadEvent</span></code></pre>

Report a signed count, supplied bytes and optional terminal status without losing partial progress.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 78

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ReadEvent</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">ReadEvent</span>)))</code></pre>

<a id="type-5472616e736665724576656e74"></a>

## `TransferEvent`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TransferEvent</span></code></pre>

Preserve the transport's complete signed64-bit count independently of byte-buffer indexing.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 88

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TransferEvent</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">TransferEvent</span>))</code></pre>

<a id="type-57726974654576656e74"></a>

## `WriteEvent`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">WriteEvent</span></code></pre>

Report a signed accepted count and optional failure; adapters validate its range.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 83

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">WriteEvent</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">WriteEvent</span>))</code></pre>

# Values

<a id="value-6275666665722d6572726f722d6d657373616765"></a>

## `buffer-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-error-message</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Describe a buffered operation while preserving typed metadata in its original error value.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 59

<a id="value-6275666665722d726561642d6163636570746564"></a>

## `buffer-read-accepted`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-read-accepted</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">ReadEvent</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Validate a source's signed report before using it as a byte count or buffer index.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 249

<a id="value-6275666665722d73696e6b"></a>

## `buffer-sink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-sink</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">WriteEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>))) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>))</code></pre>

Construct a sink whose raw signed counts remain observable to buffered consumers.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 117

<a id="value-6275666665722d73696e6b2d666c757368"></a>

## `buffer-sink-flush`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-sink-flush</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span>)), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>)))</code></pre>

Invoke an advertised flush and return its advanced handle, preserving absence separately from successful flushing.
None performs no transport I/O. Some None reports a successful flush;
Some (Some failure) retains its original structured failure and continuation.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 337

<a id="value-6275666665722d73696e6b2d66726f6d2d73747265616d"></a>

## `buffer-sink-from-stream`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-sink-from-stream</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>))</code></pre>

Adapt an existing sink, preserving its original structured errors and authoritative next handle.
A failure-only stream result reports zero progress; richer raw callbacks can report data and error together.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 275

<a id="value-6275666665722d73696e6b2d6861732d666c757368"></a>

## `buffer-sink-has-flush`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-sink-has-flush</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

Observe the current sink's flush capability without invoking a flush or transport write.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 327

<a id="value-6275666665722d73696e6b2d6861732d74657874"></a>

## `buffer-sink-has-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-sink-has-text</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

Observe whether the current sink supports an independent raw-text write operation.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 293

<a id="value-6275666665722d73696e6b2d6861732d7472616e73666572"></a>

## `buffer-sink-has-transfer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-sink-has-transfer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

Observe the current direct-transfer capability without invoking any transport operation.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 144

<a id="value-6275666665722d73696e6b2d7472616e73666572"></a>

## `buffer-sink-transfer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-sink-transfer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">TransferEvent</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>))))</code></pre>

Invoke an optional sink transfer; retained read callbacks and the returned source share coherent progress.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 241

<a id="value-6275666665722d73696e6b2d776974682d666c757368"></a>

## `buffer-sink-with-flush`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-sink-with-flush</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>))) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>)))</code></pre>

Attach an optional flush operation whose advanced sink remains authoritative on both success and failure.
A callback flushes buffered output owned by that sink; custom transports define
whether their own subordinate layers require additional explicit finalization.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 321

<a id="value-6275666665722d73696e6b2d776974682d74657874"></a>

## `buffer-sink-with-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-sink-with-text</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">WriteEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>))) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>)))</code></pre>

Attach an optional text-write operation receiving arbitrary bytes, including malformed UTF-8.
Its returned sink is authoritative. Ordinary byte writes retain only capabilities
advertised by their own returned sink, just like direct transfer callbacks.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 287

<a id="value-6275666665722d73696e6b2d776974682d7472616e73666572"></a>

## `buffer-sink-with-transfer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-sink-with-transfer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>) -&gt; (((<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">ReadEvent</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">TransferEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>))) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>)))</code></pre>

Attach a direct transfer; the supplied read operation preserves the other handle's progress.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 128

<a id="value-6275666665722d73696e6b2d7772697465"></a>

## `buffer-sink-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-sink-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">WriteEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>))))</code></pre>

Invoke one raw sink callback with caller-owned bytes, retaining its advanced handle on every outcome.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 160

<a id="value-6275666665722d73696e6b2d77726974652d74657874"></a>

## `buffer-sink-write-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-sink-write-text</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">WriteEvent</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>))))</code></pre>

Invoke only an explicitly advertised raw-text operation; absence performs no transport I/O.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 301

<a id="value-6275666665722d736f75726365"></a>

## `buffer-source`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-source</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">ReadEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>))) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>))</code></pre>

Construct a source whose raw events are validated by each buffered consumer before copying.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 113

<a id="value-6275666665722d736f757263652d66726f6d2d73747265616d"></a>

## `buffer-source-from-stream`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-source-from-stream</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>))</code></pre>

Adapt an existing source while preserving its original failures and explicit next continuation.
A successful empty positive read becomes end-of-input; a zero request remains nonterminal.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 263

<a id="value-6275666665722d736f757263652d6861732d7472616e73666572"></a>

## `buffer-source-has-transfer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-source-has-transfer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

Observe the current direct-transfer capability, including after a retained callback advances a proxy.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 137

<a id="value-6275666665722d736f757263652d72656164"></a>

## `buffer-source-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-source-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">ReadEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>))))</code></pre>

Invoke one raw source callback, retaining its event and advanced handle even for malformed counts.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 153

<a id="value-6275666665722d736f757263652d7472616e73666572"></a>

## `buffer-source-transfer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-source-transfer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">TransferEvent</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>))))</code></pre>

Invoke an optional source transfer; absence performs no transport I/O and returns the original handles.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 233

<a id="value-6275666665722d736f757263652d776974682d7472616e73666572"></a>

## `buffer-source-with-transfer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-source-with-transfer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>) -&gt; (((<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">WriteEvent</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">TransferEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>))) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>)))</code></pre>

Attach a direct transfer; the supplied write operation preserves the other handle's progress.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 121

<a id="value-6275666665722d77726974652d6163636570746564"></a>

## `buffer-write-accepted`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffer-write-accepted</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">WriteEvent</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Validate a sink's signed report before advancing any buffered byte interval.

[Source](/reference-sources/Effect.Buffered.Core.cinch) · line 256

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Buffered/Core.cinch`. Source SHA-256: `5c63e279b003fbc6ace432fa73821711e5072d651be7736ad53ce64583a9b951`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
