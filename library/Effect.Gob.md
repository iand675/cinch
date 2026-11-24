# Effect.Gob

Streaming Go gob encoders and decoders over Effect.Stream preserving type IDs across multi-message streams.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Gob`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Gob.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Stream](Effect.Stream.md), [Text.Gob](Text.Gob.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Effect.Gob](Effect.Gob.md), [Effect.Stream](Effect.Stream.md), [Text.Gob](Text.Gob.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Streaming gob binary encoders and decoders over Effect.Stream byte handles.
Encoders and decoders preserve transmitted user type IDs across multiple
messages so repeated struct, slice, array and map descriptors are sent once.
Every operation returns the advanced handle on both success and failure.
Clean EOF between messages yields GobStreamEndOfInput; EOF after a type
definition or mid-frame yields a truncation format error. Borrowed streams
are never closed or flushed.

</details>

# Types and constructors

<a id="type-476f624465636f646572"></a>

## `GobDecoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GobDecoder</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Immutable streaming gob decoder pairing a ByteReader with persistent wire-type state.

[Source](/reference-sources/Effect.Gob.cinch) · line 55

<a id="type-476f62456e636f646572"></a>

## `GobEncoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GobEncoder</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Immutable streaming gob encoder pairing a ByteWriter with persistent type-ID state.

[Source](/reference-sources/Effect.Gob.cinch) · line 50

<a id="type-476f6253747265616d4572726f72"></a>

## `GobStreamError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GobStreamError</span></code></pre>

Named errors produced by streaming gob encoders and decoders.

[Source](/reference-sources/Effect.Gob.cinch) · line 29

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobStreamEndOfInput</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobStreamError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobStreamInputFailure</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobStreamError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobStreamOutputFailure</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobStreamError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobStreamFormatFailure</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobStreamError</span>)</code></pre>

# Values

<a id="value-676f622d73747265616d2d6465636f6465"></a>

## `gob-stream-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-stream-decode</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobDecoder</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobStreamError</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobDecoder</span> <span class="tok-variable">r</span>)))</code></pre>

Decode the next value from the gob stream, updating persistent wire-type state.
Returns GobStreamEndOfInput at clean EOF before the next message.

[Source](/reference-sources/Effect.Gob.cinch) · line 215

<a id="value-676f622d73747265616d2d6465636f64652d616c6c"></a>

## `gob-stream-decode-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-stream-decode-all</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobDecoder</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobStreamError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>))), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobDecoder</span> <span class="tok-variable">r</span>)))</code></pre>

Decode all remaining values from the gob stream until clean EOF.

[Source](/reference-sources/Effect.Gob.cinch) · line 228

<a id="value-676f622d73747265616d2d6465636f646572"></a>

## `gob-stream-decoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-stream-decoder</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobDecoder</span> <span class="tok-variable">r</span>))</code></pre>

Create a new streaming gob decoder over the given ByteReader.

[Source](/reference-sources/Effect.Gob.cinch) · line 74

<a id="value-676f622d73747265616d2d6465636f6465722d736f75726365"></a>

## `gob-stream-decoder-source`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-stream-decoder-source</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobDecoder</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Extract the underlying advanced ByteReader from a streaming gob decoder.

[Source](/reference-sources/Effect.Gob.cinch) · line 78

<a id="value-676f622d73747265616d2d6465636f6465722d7374617465"></a>

## `gob-stream-decoder-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-stream-decoder-state</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobDecoder</span> <span class="tok-variable">r</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobDecoderState</span>)</code></pre>

Extract the persistent GobDecoderState from a streaming gob decoder.

[Source](/reference-sources/Effect.Gob.cinch) · line 83

<a id="value-676f622d73747265616d2d656e636f6465"></a>

## `gob-stream-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-stream-encode</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobEncoder</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobStreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobEncoder</span> <span class="tok-variable">r</span>)))))</code></pre>

Encode one typed value onto the stream, reusing previously transmitted type IDs.
Type-mismatch errors fail before touching the sink or advancing encoder state.

[Source](/reference-sources/Effect.Gob.cinch) · line 117

<a id="value-676f622d73747265616d2d656e636f64652d616c6c"></a>

## `gob-stream-encode-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-stream-encode-all</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobEncoder</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobStreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobEncoder</span> <span class="tok-variable">r</span>))))</code></pre>

Encode every (GobType, GobValue) pair onto the stream, reusing transmitted type IDs.

[Source](/reference-sources/Effect.Gob.cinch) · line 136

<a id="value-676f622d73747265616d2d656e636f646572"></a>

## `gob-stream-encoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-stream-encoder</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobEncoder</span> <span class="tok-variable">r</span>))</code></pre>

Create a new streaming gob encoder over the given ByteWriter.

[Source](/reference-sources/Effect.Gob.cinch) · line 60

<a id="value-676f622d73747265616d2d656e636f6465722d73696e6b"></a>

## `gob-stream-encoder-sink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-stream-encoder-sink</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobEncoder</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))</code></pre>

Extract the underlying advanced ByteWriter from a streaming gob encoder.

[Source](/reference-sources/Effect.Gob.cinch) · line 64

<a id="value-676f622d73747265616d2d656e636f6465722d7374617465"></a>

## `gob-stream-encoder-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-stream-encoder-state</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobEncoder</span> <span class="tok-variable">r</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobEncoderState</span>)</code></pre>

Extract the persistent GobEncoderState from a streaming gob encoder.

[Source](/reference-sources/Effect.Gob.cinch) · line 69

<a id="value-676f622d73747265616d2d6572726f722d6d657373616765"></a>

## `gob-stream-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-stream-error-message</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobStreamError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a human-readable diagnostic message for a GobStreamError.

[Source](/reference-sources/Effect.Gob.cinch) · line 40

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Gob.cinch`. Source SHA-256: `075a8f1d670f1a9775ea580d2579de883389f007277b37f927e2a3d319107942`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
