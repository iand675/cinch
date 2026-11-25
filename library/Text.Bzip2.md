# Text.Bzip2

Decompress concatenated bzip2 streams with table-driven Huffman decoding and verified block and stream CRCs.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Bzip2</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Bzip2/source/">View source</a><a href="/packages/search/?q=%2BText.Bzip2">Search module</a><a href="/reference-sources/Text.Bzip2.cinch" download="Text.Bzip2.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis" open><summary>Synopsis (3 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Bzip2Error">Bzip2Error</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Bzip2Error</strong></code></li><li><a class="syn-name" href="#v-bzip2-decompress">bzip2-decompress</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bzip2-decompress</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Bzip2/#t-Bzip2Error" title="Text.Bzip2.Bzip2Error">Bzip2Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-bzip2-error-message">bzip2-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bzip2-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Bzip2/#t-Bzip2Error" title="Text.Bzip2.Bzip2Error">Bzip2Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Bzip2Error">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Bzip2Error</strong></code><span class="doc-sig-links"><a href="#t-Bzip2Error" title="Permalink">#</a><a href="/packages/stdlib/Text.Bzip2/source/#L28" title="View source at line 28">Source</a></span></div>
<div class="doc-body">

A malformed or corrupted bzip2 stream.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Bzip2BadHeader"><code><strong class="tok-constructor">Bzip2BadHeader</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Bzip2/#t-Bzip2Error" title="Text.Bzip2.Bzip2Error">Bzip2Error</a></code></li><li id="k-Bzip2BadBlock"><code><strong class="tok-constructor">Bzip2BadBlock</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Bzip2/#t-Bzip2Error" title="Text.Bzip2.Bzip2Error">Bzip2Error</a></code></li><li id="k-Bzip2Truncated"><code><strong class="tok-constructor">Bzip2Truncated</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Bzip2/#t-Bzip2Error" title="Text.Bzip2.Bzip2Error">Bzip2Error</a></code></li><li id="k-Bzip2ChecksumMismatch"><code><strong class="tok-constructor">Bzip2ChecksumMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Bzip2/#t-Bzip2Error" title="Text.Bzip2.Bzip2Error">Bzip2Error</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-bzip2-decompress">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bzip2-decompress</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Bzip2/#t-Bzip2Error" title="Text.Bzip2.Bzip2Error">Bzip2Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-bzip2-decompress" title="Permalink">#</a><a href="/packages/stdlib/Text.Bzip2/source/#L105" title="View source at line 105">Source</a></span></div>
<div class="doc-body">

Decompress a complete input of one or more concatenated streams.

</div>
</article>

<article class="doc-entry" id="v-bzip2-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bzip2-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Bzip2/#t-Bzip2Error" title="Text.Bzip2.Bzip2Error">Bzip2Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-bzip2-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Bzip2/source/#L39" title="View source at line 39">Source</a></span></div>
<div class="doc-body">

Render a bzip2 error.

</div>
</article>
