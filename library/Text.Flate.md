# Text.Flate

RFC 1951 raw DEFLATE compression (hash-chain LZ77 and Package-Merge dynamic/fixed Huffman) and bounded decompression with preset dictionaries.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Flate`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Flate.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Flate](Text.Flate.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Raw DEFLATE (RFC 1951) compression and decompression with preset dictionaries.
Compression supports stored blocks, fixed Huffman blocks, and dynamic Huffman
blocks with length-limited canonical codes generated via package-merge, a
3-byte hash-chain LZ77 match finder over a 32 KiB sliding window, lazy
matching at higher levels, and Huffman-only mode. Decompression supports
stored, fixed, and dynamic blocks with 9-bit direct table lookup, canonical
overflow decoding, preset dictionaries, exact consumed-byte reporting, and
optional output bounds.

</details>

# Types and constructors

<a id="type-466c617465436f6d7072657373696f6e"></a>

## `FlateCompression`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">FlateCompression</span></code></pre>

Compression level controlling LZ77 search depth and Huffman coding.

[Source](/reference-sources/Text.Flate.cinch) · line 38

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FlateNoCompression</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateCompression</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FlateBestSpeed</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateCompression</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FlateLevel</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateCompression</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FlateDefaultCompression</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateCompression</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FlateBestCompression</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateCompression</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FlateHuffmanOnly</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateCompression</span></code></pre>

<a id="type-466c6174654572726f72"></a>

## `FlateError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">FlateError</span></code></pre>

Named failures for raw DEFLATE encoding and decoding.

[Source](/reference-sources/Text.Flate.cinch) · line 53

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FlateCorruptInput</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FlateUnexpectedEof</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FlateBadLevel</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FlateOutputLimitExceeded</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">FlateUnsupportedBlock</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateError</span></code></pre>

# Values

<a id="value-666c6174652d636f6d7072657373"></a>

## `flate-compress`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-compress</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateCompression</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Compress an input buffer at the given compression level (no preset dictionary).

[Source](/reference-sources/Text.Flate.cinch) · line 1537

<a id="value-666c6174652d636f6d70726573732d6368756e6b"></a>

## `flate-compress-chunk`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-compress-chunk</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateCompression</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">U64</span>, <span class="tok-constructor">Nat</span>, <span class="tok-constructor">Bytes</span>))))))))</code></pre>

Compress one streaming chunk using a prior sliding window and carried-over
0..7 bit accumulator. Returns whole emitted bytes, new leftover bits, bit count,
and the updated 32 KiB sliding window.

[Source](/reference-sources/Text.Flate.cinch) · line 1553

<a id="value-666c6174652d636f6d70726573732d64696374"></a>

## `flate-compress-dict`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-compress-dict</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateCompression</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Compress an input buffer at the given level using a preset dictionary (last 32 KiB).

[Source](/reference-sources/Text.Flate.cinch) · line 1541

<a id="value-666c6174652d6465636f6d7072657373"></a>

## `flate-decompress`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-decompress</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Nat</span>)))</code></pre>

Decompress a raw DEFLATE stream starting at offset 0.
Returns the decompressed bytes and the exact count of input bytes consumed.

[Source](/reference-sources/Text.Flate.cinch) · line 652

<a id="value-666c6174652d6465636f6d70726573732d6174"></a>

## `flate-decompress-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-decompress-at</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Decompress a raw DEFLATE stream starting at byte offset start.
Returns the decompressed bytes and the byte offset immediately after the stream.

[Source](/reference-sources/Text.Flate.cinch) · line 657

<a id="value-666c6174652d6465636f6d70726573732d61742d646963742d6c696d6974"></a>

## `flate-decompress-at-dict-limit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-decompress-at-dict-limit</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Nat</span>))))))</code></pre>

Decompress a raw DEFLATE stream at byte offset start with a preset dictionary
and output byte limit. Returns decoded bytes and the next byte offset.

[Source](/reference-sources/Text.Flate.cinch) · line 674

<a id="value-666c6174652d6465636f6d70726573732d626c6f636b"></a>

## `flate-decompress-block`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-decompress-block</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Nat</span>, <span class="tok-constructor">Bool</span>))))))</code></pre>

Decode a single DEFLATE block starting at an absolute bit position with a
sliding-window history buffer. Returns block output, next bit position, and BFINAL.

[Source](/reference-sources/Text.Flate.cinch) · line 686

<a id="value-666c6174652d6465636f6d70726573732d64696374"></a>

## `flate-decompress-dict`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-decompress-dict</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Decompress a raw DEFLATE stream using a preset dictionary (up to its last 32 KiB).

[Source](/reference-sources/Text.Flate.cinch) · line 665

<a id="value-666c6174652d6465636f6d70726573732d646963742d6c696d6974"></a>

## `flate-decompress-dict-limit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-decompress-dict-limit</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Nat</span>)))))</code></pre>

Decompress a raw DEFLATE stream using a preset dictionary and explicit output limit.

[Source](/reference-sources/Text.Flate.cinch) · line 669

<a id="value-666c6174652d6465636f6d70726573732d6c696d6974"></a>

## `flate-decompress-limit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-decompress-limit</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Decompress a raw DEFLATE stream with an explicit maximum output size in bytes.

[Source](/reference-sources/Text.Flate.cinch) · line 661

<a id="value-666c6174652d6572726f722d6d657373616765"></a>

## `flate-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a human-readable diagnostic for a raw DEFLATE failure.

[Source](/reference-sources/Text.Flate.cinch) · line 66

<a id="value-666c6174652d6c6576656c2d66726f6d2d696e74"></a>

## `flate-level-from-int`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-level-from-int</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateCompression</span>))</code></pre>

Convert an integer compression level (-2 for Huffman-only, -1 for default,
0 for no compression, 1..9 for explicit levels) into a FlateCompression value.

[Source](/reference-sources/Text.Flate.cinch) · line 76

<a id="value-666c6174652d73796e632d666c757368"></a>

## `flate-sync-flush`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flate-sync-flush</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">U64</span>, <span class="tok-constructor">Nat</span>)))</code></pre>

Emit an RFC 1951 sync flush (non-final empty stored block) aligning the bitstream
to a byte boundary so all prior compressed bytes are immediately decodable.

[Source](/reference-sources/Text.Flate.cinch) · line 1568

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Flate.cinch`. Source SHA-256: `2950d4890c729a4306cb2ecd1be43e805b4862926709ba822b8974fa502b47e4`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
