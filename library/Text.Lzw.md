# Text.Lzw

Variable-width-code LZW compression and decompression in either bit order, as used by GIF, TIFF and PDF.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Lzw</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Lzw/source/">View source</a><a href="/packages/search/?q=%2BText.Lzw">Search module</a><a href="/reference-sources/Text.Lzw.cinch" download="Text.Lzw.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details class="doc-notes"><summary>Module notes</summary>

Lempel-Ziv-Welch compression with variable-width codes, as used by GIF, TIFF and PDF.
Codes start one bit wider than the literal width (2 to 8 bits) and grow to
twelve bits. Code 2^w clears the dictionary and code 2^w + 1 ends the data.
Compression always begins with a clear code and emits another clear code
whenever all 4096 codes are in use. Codes are packed least-significant bit
first (GIF) or most-significant bit first (TIFF and PDF); the "early change"
variant some TIFF writers use is not supported. Dictionaries live in private
byte tables, so compression is a single hashed pass and decompression
expands each code directly into its final output position.

</details>

<details class="doc-synopsis" open><summary>Synopsis (7 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-LzwEnd">LzwEnd</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">LzwEnd</strong></code></li><li><a class="syn-name" href="#t-LzwError">LzwError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">LzwError</strong></code></li><li><a class="syn-name" href="#t-LzwOrder">LzwOrder</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">LzwOrder</strong></code></li><li><a class="syn-name" href="#v-lzw-compress">lzw-compress</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lzw-compress</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwOrder" title="Text.Lzw.LzwOrder">LzwOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwError" title="Text.Lzw.LzwError">LzwError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-lzw-decompress">lzw-decompress</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lzw-decompress</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwOrder" title="Text.Lzw.LzwOrder">LzwOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwError" title="Text.Lzw.LzwError">LzwError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-lzw-decompress-bounded">lzw-decompress-bounded</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lzw-decompress-bounded</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwOrder" title="Text.Lzw.LzwOrder">LzwOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwError" title="Text.Lzw.LzwError">LzwError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwEnd" title="Text.Lzw.LzwEnd">LzwEnd</a>)</code></li><li><a class="syn-name" href="#v-lzw-error-message">lzw-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lzw-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwError" title="Text.Lzw.LzwError">LzwError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-LzwEnd">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">LzwEnd</strong></code><span class="doc-sig-links"><a href="#t-LzwEnd" title="Permalink">#</a><a href="/packages/stdlib/Text.Lzw/source/#L239" title="View source at line 239">Source</a></span></div>
<div class="doc-body">

How a bounded decompression stopped.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-LzwEnded"><code><strong class="tok-constructor">LzwEnded</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwEnd" title="Text.Lzw.LzwEnd">LzwEnd</a></code></li><li id="k-LzwExhausted"><code><strong class="tok-constructor">LzwExhausted</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwEnd" title="Text.Lzw.LzwEnd">LzwEnd</a></code></li><li id="k-LzwOverflow"><code><strong class="tok-constructor">LzwOverflow</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwEnd" title="Text.Lzw.LzwEnd">LzwEnd</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-LzwError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">LzwError</strong></code><span class="doc-sig-links"><a href="#t-LzwError" title="Permalink">#</a><a href="/packages/stdlib/Text.Lzw/source/#L33" title="View source at line 33">Source</a></span></div>
<div class="doc-body">

A rejected parameter or malformed compressed stream.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidLiteralWidth"><code><strong class="tok-constructor">InvalidLiteralWidth</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwError" title="Text.Lzw.LzwError">LzwError</a></code></li><li id="k-LiteralTooLarge"><code><strong class="tok-constructor">LiteralTooLarge</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwError" title="Text.Lzw.LzwError">LzwError</a></code></li><li id="k-InvalidLzwCode"><code><strong class="tok-constructor">InvalidLzwCode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwError" title="Text.Lzw.LzwError">LzwError</a></code></li><li id="k-UnexpectedLzwEnd"><code><strong class="tok-constructor">UnexpectedLzwEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwError" title="Text.Lzw.LzwError">LzwError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-LzwOrder">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">LzwOrder</strong></code><span class="doc-sig-links"><a href="#t-LzwOrder" title="Permalink">#</a><a href="/packages/stdlib/Text.Lzw/source/#L26" title="View source at line 26">Source</a></span></div>
<div class="doc-body">

Bit order used to pack codes into bytes.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-LzwLsb"><code><strong class="tok-constructor">LzwLsb</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwOrder" title="Text.Lzw.LzwOrder">LzwOrder</a></code></li><li id="k-LzwMsb"><code><strong class="tok-constructor">LzwMsb</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwOrder" title="Text.Lzw.LzwOrder">LzwOrder</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-lzw-compress">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lzw-compress</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwOrder" title="Text.Lzw.LzwOrder">LzwOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwError" title="Text.Lzw.LzwError">LzwError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-lzw-compress" title="Permalink">#</a><a href="/packages/stdlib/Text.Lzw/source/#L116" title="View source at line 116">Source</a></span></div>
<div class="doc-body">

Compress a complete input. Every byte must be below 2^width.

</div>
</article>

<article class="doc-entry" id="v-lzw-decompress">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lzw-decompress</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwOrder" title="Text.Lzw.LzwOrder">LzwOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwError" title="Text.Lzw.LzwError">LzwError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-lzw-decompress" title="Permalink">#</a><a href="/packages/stdlib/Text.Lzw/source/#L231" title="View source at line 231">Source</a></span></div>
<div class="doc-body">

Decompress a complete stream through its end-of-data code.

</div>
</article>

<article class="doc-entry" id="v-lzw-decompress-bounded">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lzw-decompress-bounded</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwOrder" title="Text.Lzw.LzwOrder">LzwOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwError" title="Text.Lzw.LzwError">LzwError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwEnd" title="Text.Lzw.LzwEnd">LzwEnd</a>)</code><span class="doc-sig-links"><a href="#v-lzw-decompress-bounded" title="Permalink">#</a><a href="/packages/stdlib/Text.Lzw/source/#L250" title="View source at line 250">Source</a></span></div>
<div class="doc-body">

Decompress until the end-of-data code or until limit bytes have been produced, whichever comes first.
After the limit only clear codes are consumed: a data code stops with LzwOverflow, the end of the input
with LzwExhausted. Running out of input before the limit is still UnexpectedLzwEnd.

</div>
</article>

<article class="doc-entry" id="v-lzw-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lzw-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwError" title="Text.Lzw.LzwError">LzwError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-lzw-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Lzw/source/#L44" title="View source at line 44">Source</a></span></div>
<div class="doc-body">

Render an LZW error.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Lzw.cinch`](/packages/stdlib/Text.Lzw/source/). Source SHA-256: `afc9d6247c42c1a0e7acd3cedbd0febb7fdfe91b87de5092078a9d731ec734c4`.
