# Effect.Buffered.Reader

Buffered byte reading with explicit advanced handles and independently owned observations.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Buffered.Reader`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Buffered.Reader.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Buffered.Core](Effect.Buffered.Core.md), [Effect.Buffered.Internal.Scan](Effect.Buffered.Internal.Scan.md), [Effect.Io](Effect.Io.md), [Effect.Stream](Effect.Stream.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Effect.Buffered.Core](Effect.Buffered.Core.md), [Effect.Buffered.Internal.Cell](Effect.Buffered.Internal.Cell.md), [Effect.Buffered.Internal.Scan](Effect.Buffered.Internal.Scan.md), [Effect.Buffered.Reader](Effect.Buffered.Reader.md), [Effect.Stream](Effect.Stream.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Buffered byte reading with explicit advanced handles and independently owned observations.
Private buffers are immutable between states. Refills copy only the retained
unread prefix; peeks and read results never expose or alias buffer storage.

</details>

# Types and constructors

<a id="type-4275666665726564526561646572"></a>

## `BufferedReader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BufferedReader</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

An advanced buffered input; always continue with the handle returned by an operation.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 46

<a id="type-436f756e744576656e74"></a>

## `CountEvent`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CountEvent</span></code></pre>

Confirmed nonnegative progress and optional failure.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 33

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CountEvent</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">CountEvent</span>))</code></pre>

<a id="type-4c696e654576656e74"></a>

## `LineEvent`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">LineEvent</span></code></pre>

A line fragment excludes line-ending bytes; a prefix has no terminal error.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 297

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LineEvent</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">LineEvent</span>)))</code></pre>

<a id="type-546578744576656e74"></a>

## `TextEvent`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TextEvent</span></code></pre>

Fresh raw bytes, checked text if valid, and the independent source termination status.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 439

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TextEvent</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">TextEvent</span>)))</code></pre>

# Values

<a id="value-62756666657265642d64697363617264"></a>

## `buffered-discard`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-discard</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Int</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">CountEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>))))</code></pre>

Discard a signed count; negative and zero requests preserve prior unread history and perform no I/O.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 248

<a id="value-62756666657265642d7065656b"></a>

## `buffered-peek`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-peek</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Int</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">ReadEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>))))</code></pre>

Observe fresh prefix bytes without consuming them; nonnegative peeks invalidate both unread permissions.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 209

<a id="value-62756666657265642d72656164"></a>

## `buffered-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">ReadEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>))))</code></pre>

Read up to a natural count with at most one underlying read, returning fresh bytes and any direct error.
A zero request performs no source I/O. Buffered bytes precede a pending error; a
large empty-buffer read can return a nonempty prefix together with that source's error.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 191

<a id="value-62756666657265642d726561642d62797465"></a>

## `buffered-read-byte`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-read-byte</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span> <span class="tok-constructor">U8</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>)))</code></pre>

Read one byte, invalidating rune unread permission while retaining the last byte across an unsuccessful read.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 270

<a id="value-62756666657265642d726561642d6279746573"></a>

## `buffered-read-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-read-bytes</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">U8</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">ReadEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>))))</code></pre>

Read through the next delimiter across arbitrarily many fixed windows, preserving a final partial result and error.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 403

<a id="value-62756666657265642d726561642d696e746f"></a>

## `buffered-read-into`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-read-into</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">CountEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>))))))</code></pre>

Read into a checked destination interval; an invalid interval performs no source I/O or destination mutation.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 310

<a id="value-62756666657265642d726561642d6c696e65"></a>

## `buffered-read-line`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-read-line</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">LineEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>)))</code></pre>

Read a fresh line fragment excluding LF or CR/LF; a capacity fragment reports prefix=True and no error.
Nonempty final unterminated lines suppress the terminal error. Byte-unread restores the actual final input byte.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 425

<a id="value-62756666657265642d726561642d72756e65"></a>

## `buffered-read-rune`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-read-rune</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span> (<span class="tok-constructor">Char</span>, <span class="tok-constructor">Nat</span>)), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>)))</code></pre>

Read one Unicode scalar and its encoded width; malformed or truncated input consumes one replacement byte.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 338

<a id="value-62756666657265642d726561642d736c696365"></a>

## `buffered-read-slice`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-read-slice</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">U8</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">ReadEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>))))</code></pre>

Read through a delimiter or one fixed buffer window, returning independently owned bytes including the delimiter.
A terminal source error accompanies an unterminated fragment; a full window reports BufferFull.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 391

<a id="value-62756666657265642d726561642d737472696e67"></a>

## `buffered-read-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-read-string</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">U8</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">TextEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>))))</code></pre>

Read through a delimiter and check its complete raw result for valid UTF-8 without dropping malformed bytes.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 442

<a id="value-62756666657265642d726561646572"></a>

## `buffered-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>))</code></pre>

Buffer a raw source using the default 4096-byte capacity.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 70

<a id="value-62756666657265642d7265616465722d61732d736f75726365"></a>

## `buffered-reader-as-source`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-reader-as-source</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>))</code></pre>

Adapt a buffered handle as a raw source with an explicit direct-transfer capability.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 452

<a id="value-62756666657265642d7265616465722d6275666665726564"></a>

## `buffered-reader-buffered`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-reader-buffered</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe the currently buffered unread byte count without invoking the underlying source.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 91

<a id="value-62756666657265642d7265616465722d66726f6d2d73747265616d"></a>

## `buffered-reader-from-stream`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-reader-from-stream</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>))</code></pre>

Buffer an existing stream source under its established empty-success-means-end contract.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 79

<a id="value-62756666657265642d7265616465722d66726f6d2d73747265616d2d73697a6564"></a>

## `buffered-reader-from-stream-sized`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-reader-from-stream-sized</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Int</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>)))</code></pre>

Select explicit capacity while adapting an existing stream source without changing its error contract.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 83

<a id="value-62756666657265642d7265616465722d7265627566666572"></a>

## `buffered-reader-rebuffer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-reader-rebuffer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Int</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>)))</code></pre>

Reuse an existing buffered handle when its capacity suffices; otherwise buffer its advanced read operation.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 300

<a id="value-62756666657265642d7265616465722d7265736574"></a>

## `buffered-reader-reset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-reader-reset</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>)))</code></pre>

Discard buffered data, pending errors and unread permissions, retaining capacity with a new source.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 97

<a id="value-62756666657265642d7265616465722d72657365742d73656c66"></a>

## `buffered-reader-reset-self`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-reader-reset-self</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>))</code></pre>

Resetting a wrapper to itself is an explicit no-op, preserving its unread data and history.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 103

<a id="value-62756666657265642d7265616465722d73697a65"></a>

## `buffered-reader-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-reader-size</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe the fixed buffer capacity without performing I/O.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 87

<a id="value-62756666657265642d7265616465722d73697a6564"></a>

## `buffered-reader-sized`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-reader-sized</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Int</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSource</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>)))</code></pre>

Buffer a raw source with a capacity of at least 16 bytes; negative requests use that minimum.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 74

<a id="value-62756666657265642d756e726561642d62797465"></a>

## `buffered-unread-byte`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-unread-byte</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>)))</code></pre>

Unread at most the most recently read byte; a successful unread clears both unread permissions.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 287

<a id="value-62756666657265642d756e726561642d72756e65"></a>

## `buffered-unread-rune`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-unread-rune</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>)))</code></pre>

Unread exactly the preceding successful scalar read; successful rewind clears both unread permissions.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 355

<a id="value-62756666657265642d77726974652d746f"></a>

## `buffered-write-to`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">buffered-write-to</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">TransferEvent</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Reader</span>.<span class="tok-constructor">BufferedReader</span> <span class="tok-variable">r</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">BufferSink</span> <span class="tok-variable">r</span>))))</code></pre>

Drain buffered bytes, then dispatch optional direct transfer capabilities in source-first order.
Every returned handle is authoritative. Ordinary end-of-input is suppressed;
malformed sink counts and unexplained short writes return named checked failures.

[Source](/reference-sources/Effect.Buffered.Reader.cinch) · line 512

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Buffered/Reader.cinch`. Source SHA-256: `b0eb53793ea2111749580e5726abaec76f84fb3a1df3b9f3706d1240b696b160`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
