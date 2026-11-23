# Text.Bzip2

Decompress concatenated bzip2 streams with table-driven Huffman decoding and verified block and stream CRCs.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Bzip2`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Bzip2.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Bzip2](Text.Bzip2.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Bzip2 decompression: concatenated streams of Burrows-Wheeler blocks with Huffman and move-to-front coding.
A block passes through its stages in order. Symbols are read with canonical
Huffman codes (two to six tables, switched every fifty symbols by
move-to-front coded selectors) through a 1024-entry direct lookup with a
canonical range scan for codes longer than ten bits. Zero-run symbols and
move-to-front indices rebuild the transformed block while counting each byte
value with carry-only counter updates; it is inverted through a vector of
32-bit successor links kept beside the block's bytes.
Following that vector feeds an explicit state machine that undoes the initial
run-length coding straight into the output, filling repeats by doubling
memmoves. Bits are read as 64-bit
big-endian windows, CRCs fold four bytes per branch-free step, and every block
CRC and each stream's combined CRC are verified. Obsolete randomized blocks
are rejected; streams that follow the first are decoded in turn.

</details>

# Types and constructors

<a id="type-427a6970324572726f72"></a>

## `Bzip2Error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Bzip2Error</span></code></pre>

A malformed or corrupted bzip2 stream.

[Source](/reference-sources/Text.Bzip2.cinch) · line 29

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Bzip2BadHeader</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Bzip2</span>.<span class="tok-constructor">Bzip2Error</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Bzip2BadBlock</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Bzip2</span>.<span class="tok-constructor">Bzip2Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Bzip2Truncated</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Bzip2</span>.<span class="tok-constructor">Bzip2Error</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Bzip2ChecksumMismatch</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Bzip2</span>.<span class="tok-constructor">Bzip2Error</span></code></pre>

# Values

<a id="value-627a6970322d6465636f6d7072657373"></a>

## `bzip2-decompress`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bzip2-decompress</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Bzip2</span>.<span class="tok-constructor">Bzip2Error</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Decompress a complete input of one or more concatenated streams.

[Source](/reference-sources/Text.Bzip2.cinch) · line 113

<a id="value-627a6970322d6572726f722d6d657373616765"></a>

## `bzip2-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bzip2-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Bzip2</span>.<span class="tok-constructor">Bzip2Error</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a bzip2 error.

[Source](/reference-sources/Text.Bzip2.cinch) · line 40

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Bzip2.cinch`. Source SHA-256: `ac64666d69ed61d0f44e7ae56cbb1a840564046919b9da0718ae7b8e50bc9283`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
