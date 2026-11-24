# Effect.Flate

Incremental raw DEFLATE streaming readers and writers over Effect.Stream with sync flush and preset dictionaries.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Flate`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Flate.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Stream](Effect.Stream.md), [Text.Flate](Text.Flate.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Flate](Effect.Flate.md), [Effect.Stream](Effect.Stream.md), [Text.Flate](Text.Flate.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Streaming raw DEFLATE (RFC 1951) readers and writers over Effect.Stream.
Writers buffer uncompressed input into bounded blocks, support explicit
byte-aligned sync flushes and idempotent finish, retain a 32 KiB sliding
window across chunks and preset dictionaries, and keep failures sticky.
Readers decode incrementally one DEFLATE block at a time with optional
preset dictionaries and output byte limits, stopping at the final block.

</details>

# Types and constructors

<a id="type-466c617465577269746572"></a>

## `FlateWriter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">FlateWriter</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Explicitly advanced raw DEFLATE compressing sink with sticky error state
and explicit flush and finish transitions.

[Source](/reference-sources/Effect.Flate.cinch) · line 90

# Values

<a id="value-666c6174652d66696e697368"></a>

## `flate-finish`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-finish</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateWriter</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Compress any remaining buffered input as the final DEFLATE block (BFINAL=1),
pad to a byte boundary, write it to the sink, and transition to finished state.
Calling flate-finish on an already finished writer succeeds without writing.

[Source](/reference-sources/Effect.Flate.cinch) · line 184

<a id="value-666c6174652d666c757368"></a>

## `flate-flush`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-flush</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateWriter</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Compress all pending buffered input as a non-final block and emit an RFC 1951
byte-aligned sync flush so a reader can immediately decode everything written so far.

[Source](/reference-sources/Effect.Flate.cinch) · line 160

<a id="value-666c6174652d726561646572"></a>

## `flate-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Decompress a raw DEFLATE byte stream incrementally without a preset dictionary.

[Source](/reference-sources/Effect.Flate.cinch) · line 211

<a id="value-666c6174652d7265616465722d64696374"></a>

## `flate-reader-dict`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-reader-dict</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>)))</code></pre>

Decompress a raw DEFLATE byte stream incrementally using a preset dictionary.

[Source](/reference-sources/Effect.Flate.cinch) · line 215

<a id="value-666c6174652d7265616465722d6c696d6974"></a>

## `flate-reader-limit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-reader-limit</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))))</code></pre>

Decompress a raw DEFLATE byte stream incrementally with an explicit output
byte bound and preset dictionary (up to its last 32 KiB).

[Source](/reference-sources/Effect.Flate.cinch) · line 220

<a id="value-666c6174652d7772697465"></a>

## `flate-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Buffer input bytes and emit complete non-final DEFLATE blocks whenever the
pending buffer reaches 32 KiB. Reports accepted input bytes on success.

[Source](/reference-sources/Effect.Flate.cinch) · line 131

<a id="value-666c6174652d777269746572"></a>

## `flate-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateCompression</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Create a streaming raw DEFLATE writer at the given compression level.
Returns FlateBadLevel if the requested level is out of range.

[Source](/reference-sources/Effect.Flate.cinch) · line 101

<a id="value-666c6174652d7772697465722d64696374"></a>

## `flate-writer-dict`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-writer-dict</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateCompression</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateWriter</span> <span class="tok-variable">r</span>)))))</code></pre>

Create a streaming raw DEFLATE writer seeded with a preset dictionary (last 32 KiB).
Returns FlateBadLevel if the requested level is out of range.

[Source](/reference-sources/Effect.Flate.cinch) · line 106

<a id="value-666c6174652d7772697465722d7265736574"></a>

## `flate-writer-reset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-writer-reset</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateWriter</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateWriter</span> <span class="tok-variable">r</span>)))</code></pre>

Reset a writer to compress a fresh stream into a new sink while preserving
its validated compression level and initial preset dictionary.

[Source](/reference-sources/Effect.Flate.cinch) · line 123

<a id="value-666c6174652d7772697465722d73696e6b"></a>

## `flate-writer-sink`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-writer-sink</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))</code></pre>

Inspect the advanced underlying sink, including after flush, finish, or failure.

[Source](/reference-sources/Effect.Flate.cinch) · line 115

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Flate.cinch`. Source SHA-256: `a51aa60a0c27c1e9c1045b20c01848ed90b5d7b497e8f6eefb173c7e76432f32`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
