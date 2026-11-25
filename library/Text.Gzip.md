# Text.Gzip

Whole-buffer gzip member encoding, checked decompression and concatenated members.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Gzip</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Gzip/source/">View source</a><a href="/packages/search/?q=%2BText.Gzip">Search module</a><a href="/reference-sources/Text.Gzip.cinch" download="Text.Gzip.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Text.Flate](Text.Flate.md), [Text.Zlib](Text.Zlib.md).

<details class="doc-notes"><summary>Module notes</summary>

Whole-buffer gzip members (RFC 1952). Decoding accepts stored, fixed and
dynamic DEFLATE blocks and checks each member's header, CRC-32 and ISIZE.
Encoders use a zero timestamp and unknown OS. Choose default DEFLATE
compression, an explicit compression level, stored blocks, or a
fixed-Huffman run encoder with stored fallback.

</details>

<details class="doc-synopsis" open><summary>Synopsis (7 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-GzipError">GzipError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GzipError</strong></code></li><li><a class="syn-name" href="#v-gzip-decode">gzip-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gzip-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gzip/#t-GzipError" title="Text.Gzip.GzipError">GzipError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gzip-decode-member">gzip-decode-member</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gzip-decode-member</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gzip/#t-GzipError" title="Text.Gzip.GzipError">GzipError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-gzip-encode">gzip-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gzip-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gzip-encode-level">gzip-encode-level</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gzip-encode-level</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Flate/#t-FlateCompression" title="Text.Flate.FlateCompression">FlateCompression</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gzip-encode-rle">gzip-encode-rle</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gzip-encode-rle</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gzip-encode-stored">gzip-encode-stored</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gzip-encode-stored</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-GzipError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GzipError</strong></code><span class="doc-sig-links"><a href="#t-GzipError" title="Permalink">#</a><a href="/packages/stdlib/Text.Gzip/source/#L21" title="View source at line 21">Source</a></span></div>
<div class="doc-body">

Errors distinguish malformed framing, truncation, compressed data and trailer mismatches.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-GzipInvalidHeader"><code><strong class="tok-constructor">GzipInvalidHeader</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gzip/#t-GzipError" title="Text.Gzip.GzipError">GzipError</a></code></li><li id="k-GzipTruncated"><code><strong class="tok-constructor">GzipTruncated</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gzip/#t-GzipError" title="Text.Gzip.GzipError">GzipError</a></code></li><li id="k-GzipInvalidDeflate"><code><strong class="tok-constructor">GzipInvalidDeflate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gzip/#t-GzipError" title="Text.Gzip.GzipError">GzipError</a></code></li><li id="k-GzipChecksumMismatch"><code><strong class="tok-constructor">GzipChecksumMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gzip/#t-GzipError" title="Text.Gzip.GzipError">GzipError</a></code></li><li id="k-GzipSizeMismatch"><code><strong class="tok-constructor">GzipSizeMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gzip/#t-GzipError" title="Text.Gzip.GzipError">GzipError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-gzip-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gzip-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gzip/#t-GzipError" title="Text.Gzip.GzipError">GzipError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gzip-decode" title="Permalink">#</a><a href="/packages/stdlib/Text.Gzip/source/#L138" title="View source at line 138">Source</a></span></div>
<div class="doc-body">

Decode a complete sequence of one or more members in order.

</div>
</article>

<article class="doc-entry" id="v-gzip-decode-member">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gzip-decode-member</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gzip/#t-GzipError" title="Text.Gzip.GzipError">GzipError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-gzip-decode-member" title="Permalink">#</a><a href="/packages/stdlib/Text.Gzip/source/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

Decode one member at an absolute byte position and return the next position.

</div>
</article>

<article class="doc-entry" id="v-gzip-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gzip-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gzip-encode" title="Permalink">#</a><a href="/packages/stdlib/Text.Gzip/source/#L39" title="View source at line 39">Source</a></span></div>
<div class="doc-body">

Encode one member using default DEFLATE compression with no optional header fields.

</div>
</article>

<article class="doc-entry" id="v-gzip-encode-level">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gzip-encode-level</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Flate/#t-FlateCompression" title="Text.Flate.FlateCompression">FlateCompression</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gzip-encode-level" title="Permalink">#</a><a href="/packages/stdlib/Text.Gzip/source/#L44" title="View source at line 44">Source</a></span></div>
<div class="doc-body">

Encode one member at the given DEFLATE compression level, falling back
to stored blocks if the level is invalid.

</div>
</article>

<article class="doc-entry" id="v-gzip-encode-rle">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gzip-encode-rle</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gzip-encode-rle" title="Permalink">#</a><a href="/packages/stdlib/Text.Gzip/source/#L57" title="View source at line 57">Source</a></span></div>
<div class="doc-body">

Encode runs using distance-one matches, falling back to stored blocks
for data that would expand beyond the stored representation.

</div>
</article>

<article class="doc-entry" id="v-gzip-encode-stored">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gzip-encode-stored</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gzip-encode-stored" title="Permalink">#</a><a href="/packages/stdlib/Text.Gzip/source/#L52" title="View source at line 52">Source</a></span></div>
<div class="doc-body">

Encode one member using uncompressed stored DEFLATE blocks.

</div>
</article>
