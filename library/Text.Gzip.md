# Text.Gzip

Whole-buffer gzip member encoding, checked decompression and concatenated members.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Gzip`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Gzip.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Text.Zlib](Text.Zlib.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Gzip](Text.Gzip.md), [Text.Zlib](Text.Zlib.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Whole-buffer gzip members (RFC 1952). Decoding accepts stored, fixed and
dynamic DEFLATE blocks and checks each member's header, CRC-32 and ISIZE.
Encoders use a zero timestamp and unknown OS. Choose stored blocks or a
fixed-Huffman run encoder with stored fallback.

</details>

# Types and constructors

<a id="type-477a69704572726f72"></a>

## `GzipError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GzipError</span></code></pre>

Errors distinguish malformed framing, truncation, compressed data and trailer mismatches.

[Source](/reference-sources/Text.Gzip.cinch) · line 19

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GzipInvalidHeader</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gzip</span>.<span class="tok-constructor">GzipError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GzipTruncated</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gzip</span>.<span class="tok-constructor">GzipError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GzipInvalidDeflate</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gzip</span>.<span class="tok-constructor">GzipError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GzipChecksumMismatch</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gzip</span>.<span class="tok-constructor">GzipError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GzipSizeMismatch</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gzip</span>.<span class="tok-constructor">GzipError</span></code></pre>

# Values

<a id="value-677a69702d6465636f6465"></a>

## `gzip-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gzip-decode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gzip</span>.<span class="tok-constructor">GzipError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Decode a complete sequence of one or more members in order.

[Source](/reference-sources/Text.Gzip.cinch) · line 109

<a id="value-677a69702d6465636f64652d6d656d626572"></a>

## `gzip-decode-member`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gzip-decode-member</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gzip</span>.<span class="tok-constructor">GzipError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Decode one member at an absolute byte position and return the next position.

[Source](/reference-sources/Text.Gzip.cinch) · line 49

<a id="value-677a69702d656e636f6465"></a>

## `gzip-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gzip-encode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode one member with no optional header fields.

[Source](/reference-sources/Text.Gzip.cinch) · line 27

<a id="value-677a69702d656e636f64652d726c65"></a>

## `gzip-encode-rle`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gzip-encode-rle</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode runs using distance-one matches, falling back to stored blocks
for data that would expand beyond the stored representation.

[Source](/reference-sources/Text.Gzip.cinch) · line 32

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Gzip.cinch`. Source SHA-256: `a94c55457e743050595f6fdb22931c834ca66a352cb820372ecaba7728c8a403`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
