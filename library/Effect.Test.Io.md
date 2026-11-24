# Effect.Test.Io

Stream fault-injection, logging and conformance-testing adapters over Effect.Stream.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Test.Io`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Test.Io.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Stream](Effect.Stream.md).

Non-embedded source closure: [Effect.Stream](Effect.Stream.md), [Effect.Test.Io](Effect.Test.Io.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Stream test adapters and reader verification helpers built over Effect.Stream.
Wraps ByteReader and ByteWriter values to exercise short reads, single-byte
progress, lookahead EOF/error bundling, simulated timeouts, fixed errors,
truncated writes, and effectful read/write logging without global state.

</details>

# Types and constructors

<a id="type-496f546573744572726f72"></a>

## `IoTestError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">IoTestError</span></code></pre>

Named failure reported by test-reader when a reader violates stream invariants.

[Source](/reference-sources/Effect.Test.Io.cinch) · line 25

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IoTestStreamError</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Io</span>.<span class="tok-constructor">IoTestError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IoTestContentMismatch</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Io</span>.<span class="tok-constructor">IoTestError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IoTestShortRead</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Io</span>.<span class="tok-constructor">IoTestError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IoTestExtraData</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Io</span>.<span class="tok-constructor">IoTestError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IoTestZeroReadNonEmpty</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Io</span>.<span class="tok-constructor">IoTestError</span>)</code></pre>

# Values

<a id="value-646174612d6572722d726561646572"></a>

## `data-err-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">data-err-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Buffer ahead so trailing EOF or errors are observed immediately alongside the final data chunk.

[Source](/reference-sources/Effect.Test.Io.cinch) · line 108

<a id="value-6572722d726561646572"></a>

## `err-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">err-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Return the supplied stream error on every positive read without producing bytes.

[Source](/reference-sources/Effect.Test.Io.cinch) · line 176

<a id="value-6572722d74696d656f7574"></a>

## `err-timeout`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">err-timeout</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span></code></pre>

Sentinel stream error returned by timeout-reader on its second positive read.

[Source](/reference-sources/Effect.Test.Io.cinch) · line 47

<a id="value-68616c662d726561646572"></a>

## `half-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">half-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Limit every positive read to half the requested length, rounding up so at least one byte is requested.

[Source](/reference-sources/Effect.Test.Io.cinch) · line 95

<a id="value-696f2d746573742d6572726f722d6d657373616765"></a>

## `io-test-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">io-test-error-message</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Io</span>.<span class="tok-constructor">IoTestError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a concise diagnostic for a reader verification failure.

[Source](/reference-sources/Effect.Test.Io.cinch) · line 38

<a id="value-69732d74696d656f75742d6572726f72"></a>

## `is-timeout-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-timeout-error</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Check whether a stream error is the simulated timeout sentinel.

[Source](/reference-sources/Effect.Test.Io.cinch) · line 51

<a id="value-6f6e652d627974652d726561646572"></a>

## `one-byte-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">one-byte-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Limit every positive read on the wrapped reader to at most one byte.

[Source](/reference-sources/Effect.Test.Io.cinch) · line 84

<a id="value-726561642d6c6f67676572"></a>

## `read-logger`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">read-logger</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))))</code></pre>

Wrap a reader and invoke the supplied effectful logger with prefix and observed bytes on every read.

[Source](/reference-sources/Effect.Test.Io.cinch) · line 205

<a id="value-746573742d726561646572"></a>

## `test-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">test-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Io</span>.<span class="tok-constructor">IoTestError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Verify that a reader yields the expected bytes across zero-length reads, small reads, varied chunk sizes, and repeated EOF reads.

[Source](/reference-sources/Effect.Test.Io.cinch) · line 236

<a id="value-74696d656f75742d726561646572"></a>

## `timeout-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">timeout-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Return err-timeout with no data on the second positive read, then delegate all subsequent reads normally.

[Source](/reference-sources/Effect.Test.Io.cinch) · line 159

<a id="value-7472756e636174652d777269746572"></a>

## `truncate-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">truncate-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Forward at most limit bytes to the underlying writer, then silently accept subsequent writes.

[Source](/reference-sources/Effect.Test.Io.cinch) · line 186

<a id="value-77726974652d6c6f67676572"></a>

## `write-logger`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">write-logger</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Wrap a writer and invoke the supplied effectful logger with prefix and accepted bytes on every write.

[Source](/reference-sources/Effect.Test.Io.cinch) · line 220

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Test/Io.cinch`. Source SHA-256: `ebf58f9e727c66bae5e85d626c6b83cdafed7612d23078e631156ec8a0559821`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
