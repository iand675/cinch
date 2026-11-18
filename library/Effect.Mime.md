# Effect.Mime

Encoded-word and header decoding with explicit charset reader capabilities, typed errors and advanced writer continuations.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Mime`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Mime.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Stream](Effect.Stream.md), [Text.Mime](Text.Mime.md).

Non-embedded source closure: [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Effect.Mime](Effect.Mime.md), [Effect.Stream](Effect.Stream.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Mime](Text.Mime.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Encoded-word decoding with explicit, inspectable charset reader capabilities.
Callbacks preserve arbitrary caller error values and may return converted
bytes with EOF or failure simultaneously. Header conversion failure discards
the returned header output. Borrowed readers and writers are never closed.

</details>

# Types and constructors

<a id="type-4d696d6543686172736574526561646572"></a>

## `MimeCharsetReader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MimeCharsetReader</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Explicit converted-reader continuation with arbitrary caller error values.

[Source](/reference-sources/Effect.Mime.cinch) · line 37

<a id="type-4d696d6543686172736574537461747573"></a>

## `MimeCharsetStatus`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MimeCharsetStatus</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Status delivered alongside one converted byte prefix.

[Source](/reference-sources/Effect.Mime.cinch) · line 28

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeCharsetMore</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeCharsetStatus</span> <span class="tok-variable">e</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeCharsetEnd</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeCharsetStatus</span> <span class="tok-variable">e</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeCharsetFailure</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . (<span class="tok-variable">e</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeCharsetStatus</span> <span class="tok-variable">e</span>)</code></pre>

<a id="type-4d696d654465636f64654661696c757265"></a>

## `MimeDecodeFailure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MimeDecodeFailure</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Decoding distinguishes syntax, callback creation and converted-reader failures.

[Source](/reference-sources/Effect.Mime.cinch) · line 117

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeWordSyntaxFailure</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeDecodeFailure</span> <span class="tok-variable">e</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeCharsetCallbackFailure</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . (<span class="tok-variable">e</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeDecodeFailure</span> <span class="tok-variable">e</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeCharsetReadFailure</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . (<span class="tok-variable">e</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeDecodeFailure</span> <span class="tok-variable">e</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeCharsetInvalidCount</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeDecodeFailure</span> <span class="tok-variable">e</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MimeOutputWriteFailure</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeDecodeFailure</span> <span class="tok-variable">e</span>)</code></pre>

<a id="type-4d696d65576f72644465636f646572"></a>

## `MimeWordDecoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MimeWordDecoder</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Decoder configuration retaining the exact optional callback capability.

[Source](/reference-sources/Effect.Mime.cinch) · line 96

# Values

<a id="value-6d696d652d636861727365742d726561646572"></a>

## `mime-charset-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-charset-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeCharsetStatus</span> <span class="tok-variable">e</span>)), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeCharsetReader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeCharsetReader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))</code></pre>

Wrap a caller reader capability. Positive reads must not exceed the request;
prefixes may accompany EOF/failure, and zero reads remain inert.

[Source](/reference-sources/Effect.Mime.cinch) · line 45

<a id="value-6d696d652d636861727365742d7265616465722d636f7079"></a>

## `mime-charset-reader-copy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-charset-reader-copy</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeCharsetReader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Observe the optional complete-copy capability without advancing the reader.

[Source](/reference-sources/Effect.Mime.cinch) · line 61

<a id="value-6d696d652d636861727365742d7265616465722d66726f6d2d6279746573"></a>

## `mime-charset-reader-from-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-charset-reader-from-bytes</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeCharsetReader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))</code></pre>

Construct a reader over privately copied bytes; positive reads deliver
data with More and a later empty read delivers End.

[Source](/reference-sources/Effect.Mime.cinch) · line 76

<a id="value-6d696d652d636861727365742d7265616465722d66726f6d2d73747265616d"></a>

## `mime-charset-reader-from-stream`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-charset-reader-from-stream</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> -&gt; <span class="tok-variable">e</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeCharsetReader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>)))</code></pre>

Bridge ordinary streams by explicitly mapping their error vocabulary into
the caller's error type. Empty positive stream reads mean EOF.

[Source](/reference-sources/Effect.Mime.cinch) · line 90

<a id="value-6d696d652d636861727365742d7265616465722d72656164"></a>

## `mime-charset-reader-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-charset-reader-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeCharsetReader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeCharsetStatus</span> <span class="tok-variable">e</span>)), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeCharsetReader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))))</code></pre>

Read one converted prefix; always continue with the returned handle.

[Source](/reference-sources/Effect.Mime.cinch) · line 49

<a id="value-6d696d652d636861727365742d7265616465722d776974682d636f7079"></a>

## `mime-charset-reader-with-copy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-charset-reader-with-copy</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-constructor">Bytes</span>)) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeCharsetReader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeCharsetReader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>)))</code></pre>

Attach an optional complete-copy capability, used once before pull reads.
This models readers that expose a direct complete-copy operation in addition to Read.

[Source](/reference-sources/Effect.Mime.cinch) · line 57

<a id="value-6d696d652d64656661756c742d776f72642d6465636f646572"></a>

## `mime-default-word-decoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-default-word-decoder</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordDecoder</span> <span class="tok-variable">r</span> <span class="tok-constructor">Unit</span>))</code></pre>

Select only the built-in UTF-8, Latin-1 and US-ASCII conversions.

[Source](/reference-sources/Effect.Mime.cinch) · line 105

<a id="value-6d696d652d6865616465722d6465636f6465"></a>

## `mime-header-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-header-decode</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordDecoder</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeDecodeFailure</span> <span class="tok-variable">e</span>) <span class="tok-constructor">Bytes</span>)))</code></pre>

Decode all header words from one retained input snapshot. Linear whitespace
between successful words is removed. Callback/read failure discards all output.

[Source](/reference-sources/Effect.Mime.cinch) · line 211

<a id="value-6d696d652d6865616465722d6465636f64652d74657874"></a>

## `mime-header-decode-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-header-decode-text</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordDecoder</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeDecodeFailure</span> <span class="tok-variable">e</span>) <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Checked header String convenience preserving callback and converted-reader errors.

[Source](/reference-sources/Effect.Mime.cinch) · line 227

<a id="value-6d696d652d6865616465722d7772697465"></a>

## `mime-header-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-header-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordDecoder</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeDecodeFailure</span> <span class="tok-variable">e</span>) <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>)))))</code></pre>

Decode the complete header before touching the borrowed sink; callback
failure leaves it untouched. Successful output uses ordinary exact stream progress.

[Source](/reference-sources/Effect.Mime.cinch) · line 237

<a id="value-6d696d652d776f72642d6465636f6465"></a>

## `mime-word-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-word-decode</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordDecoder</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeDecodeFailure</span> <span class="tok-variable">e</span>) <span class="tok-constructor">Bytes</span>)))</code></pre>

Decode one complete encoded word to fresh arbitrary bytes, preserving typed
caller errors. Any failure discards the returned decoded output completely.

[Source](/reference-sources/Effect.Mime.cinch) · line 185

<a id="value-6d696d652d776f72642d6465636f64652d74657874"></a>

## `mime-word-decode-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-word-decode-text</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordDecoder</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeDecodeFailure</span> <span class="tok-variable">e</span>) <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Checked String convenience; raw UTF-8-labelled payloads are never silently repaired.

[Source](/reference-sources/Effect.Mime.cinch) · line 223

<a id="value-6d696d652d776f72642d6465636f646572"></a>

## `mime-word-decoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-word-decoder</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeCharsetReader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))))) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordDecoder</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))</code></pre>

Construct a decoder with an explicit optional charset reader callback.

[Source](/reference-sources/Effect.Mime.cinch) · line 101

<a id="value-6d696d652d776f72642d6465636f6465722d636861727365742d726561646572"></a>

## `mime-word-decoder-charset-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-word-decoder-charset-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordDecoder</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeCharsetReader</span> <span class="tok-variable">r</span> <span class="tok-variable">e</span>))))))</code></pre>

Observe the exact optional callback, preserving its arbitrary caller error type.

[Source](/reference-sources/Effect.Mime.cinch) · line 109

<a id="value-6d696d652d776f72642d6465636f6465722d776974682d636861727365742d726561646572"></a>

## `mime-word-decoder-with-charset-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-word-decoder-with-charset-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">old</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">fresh</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">fresh</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeCharsetReader</span> <span class="tok-variable">r</span> <span class="tok-variable">fresh</span>))))) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordDecoder</span> <span class="tok-variable">r</span> <span class="tok-variable">old</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordDecoder</span> <span class="tok-variable">r</span> <span class="tok-variable">fresh</span>)))</code></pre>

Replace or clear a callback explicitly; the decoder has no hidden mutable fields.

[Source](/reference-sources/Effect.Mime.cinch) · line 113

<a id="value-6d696d652d776f72642d7772697465"></a>

## `mime-word-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mime-word-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Mime</span>.<span class="tok-constructor">MimeWordEncoder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))))))</code></pre>

Encode complete bytes, drain short writes through the shared stream helper,
and return the exact advanced borrowed sink on either outcome.

[Source](/reference-sources/Effect.Mime.cinch) · line 232

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Mime.cinch`. Source SHA-256: `1211c316b96474b7ed1d997092520055aca5cabba4b3c262d1a6f9d490fca215`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
