# Text.Hex

SIMD hexadecimal text/byte codecs, prefix-preserving decoding, append and destination operations, and formatted byte dumps.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Hex</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Hex/source/">View source</a><a href="/packages/search/?q=%2BText.Hex">Search module</a><a href="/reference-sources/Text.Hex.cinch" download="Text.Hex.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details class="doc-notes"><summary>Module notes</summary>

Hexadecimal conversion between byte buffers and text.
Encoding emits two lowercase ASCII characters per byte. Decoding accepts either
case, without whitespace or prefixes. Invalid-byte offsets count UTF-8 bytes;
an invalid final byte takes precedence over an odd-length error. Whole-text
decoding rejects the input on error; prefix and append APIs retain completed
bytes alongside the error. Complete blocks use inline LLVM SIMD nibble lanes;
short tails and precise error reporting use the scalar path.

</details>

<details class="doc-synopsis" open><summary>Synopsis (14 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-HexError">HexError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HexError</strong></code></li><li><a class="syn-name" href="#v-hex-decode">hex-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Hex/#t-HexError" title="Text.Hex.HexError">HexError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-hex-decode-append">hex-decode-append</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-decode-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Hex/#t-HexError" title="Text.Hex.HexError">HexError</a>)</code></li><li><a class="syn-name" href="#v-hex-decode-into">hex-decode-into</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-decode-into</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Hex/#t-HexError" title="Text.Hex.HexError">HexError</a>)</code></li><li><a class="syn-name" href="#v-hex-decode-prefix">hex-decode-prefix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-decode-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Hex/#t-HexError" title="Text.Hex.HexError">HexError</a>)</code></li><li><a class="syn-name" href="#v-hex-decoded-length">hex-decoded-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-decoded-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-hex-dump">hex-dump</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-dump</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-hex-dump-bytes">hex-dump-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-dump-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-hex-encode">hex-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-hex-encode-append">hex-encode-append</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-encode-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-hex-encode-bytes">hex-encode-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-encode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-hex-encode-into">hex-encode-into</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-encode-into</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Hex/#t-HexError" title="Text.Hex.HexError">HexError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-hex-encoded-length">hex-encoded-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-encoded-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-hex-error-message">hex-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Hex/#t-HexError" title="Text.Hex.HexError">HexError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-HexError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HexError</strong></code><span class="doc-sig-links"><a href="#t-HexError" title="Permalink">#</a><a href="/packages/stdlib/Text.Hex/source/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">

A hexadecimal input or destination is invalid. Prefix APIs return completed output alongside this error.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-OddHexLength"><code><strong class="tok-constructor">OddHexLength</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Hex/#t-HexError" title="Text.Hex.HexError">HexError</a></code></li><li id="k-InvalidHexByte"><code><strong class="tok-constructor">InvalidHexByte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Hex/#t-HexError" title="Text.Hex.HexError">HexError</a></code></li><li id="k-HexShortDestination"><code><strong class="tok-constructor">HexShortDestination</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Hex/#t-HexError" title="Text.Hex.HexError">HexError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-hex-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Hex/#t-HexError" title="Text.Hex.HexError">HexError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-hex-decode" title="Permalink">#</a><a href="/packages/stdlib/Text.Hex/source/#L94" title="View source at line 94">Source</a></span></div>
<div class="doc-body">

Decode text atomically: malformed input returns only an error, without a partial buffer.

</div>
</article>

<article class="doc-entry" id="v-hex-decode-append">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-decode-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Hex/#t-HexError" title="Text.Hex.HexError">HexError</a>)</code><span class="doc-sig-links"><a href="#v-hex-decode-append" title="Permalink">#</a><a href="/packages/stdlib/Text.Hex/source/#L106" title="View source at line 106">Source</a></span></div>
<div class="doc-body">

Append decoded bytes to a copied prefix, retaining completed pairs even if later input is invalid.
Error offsets refer to the encoded input, independently of the prefix length.

</div>
</article>

<article class="doc-entry" id="v-hex-decode-into">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-decode-into</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Hex/#t-HexError" title="Text.Hex.HexError">HexError</a>)</code><span class="doc-sig-links"><a href="#v-hex-decode-into" title="Permalink">#</a><a href="/packages/stdlib/Text.Hex/source/#L119" title="View source at line 119">Source</a></span></div>
<div class="doc-body">

Decode into a destination and return completed bytes plus any decoding error.
A short range returns zero and HexShortDestination before mutation; malformed input
retains its valid decoded prefix. The source is snapshotted so overlap is safe.

</div>
</article>

<article class="doc-entry" id="v-hex-decode-prefix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-decode-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Hex/#t-HexError" title="Text.Hex.HexError">HexError</a>)</code><span class="doc-sig-links"><a href="#v-hex-decode-prefix" title="Permalink">#</a><a href="/packages/stdlib/Text.Hex/source/#L101" title="View source at line 101">Source</a></span></div>
<div class="doc-body">

Decode all complete valid pairs from arbitrary encoded bytes, preserving a prefix on error.
Invalid byte offsets refer to the original input, including invalid non-UTF-8 bytes.

</div>
</article>

<article class="doc-entry" id="v-hex-decoded-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-decoded-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-hex-decoded-length" title="Permalink">#</a><a href="/packages/stdlib/Text.Hex/source/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

The maximum complete decoded byte count for an encoded length: floor(length / 2). This does not validate the input.

</div>
</article>

<article class="doc-entry" id="v-hex-dump">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-dump</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-hex-dump" title="Permalink">#</a><a href="/packages/stdlib/Text.Hex/source/#L266" title="View source at line 266">Source</a></span></div>
<div class="doc-body">

Render sixteen-byte rows with eight-digit offsets, grouped lowercase hex and printable ASCII.
Non-printable bytes become dots. Empty input is empty text; no final offset-only row is emitted.

</div>
</article>

<article class="doc-entry" id="v-hex-dump-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-dump-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-hex-dump-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Hex/source/#L274" title="View source at line 274">Source</a></span></div>
<div class="doc-body">

Render a dump as ASCII bytes starting at an explicit 32-bit line offset.
Offsets wrap modulo 2^32. A short last row is padded only before its ASCII column.

</div>
</article>

<article class="doc-entry" id="v-hex-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-hex-encode" title="Permalink">#</a><a href="/packages/stdlib/Text.Hex/source/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">

Encode all bytes as lowercase hexadecimal ASCII. Empty input produces empty text; complete blocks use SIMD and the tail is exact.

</div>
</article>

<article class="doc-entry" id="v-hex-encode-append">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-encode-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-hex-encode-append" title="Permalink">#</a><a href="/packages/stdlib/Text.Hex/source/#L68" title="View source at line 68">Source</a></span></div>
<div class="doc-body">

Append encoded bytes to a copied prefix. Neither input buffer is mutated or retained by alias.

</div>
</article>

<article class="doc-entry" id="v-hex-encode-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-encode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-hex-encode-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Hex/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

Encode directly to a fresh lowercase ASCII buffer, without a text conversion.

</div>
</article>

<article class="doc-entry" id="v-hex-encode-into">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-encode-into</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Hex/#t-HexError" title="Text.Hex.HexError">HexError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-hex-encode-into" title="Permalink">#</a><a href="/packages/stdlib/Text.Hex/source/#L77" title="View source at line 77">Source</a></span></div>
<div class="doc-body">

Encode into a destination starting at an offset and return the written byte count.
The entire destination range is checked before writing. Source/destination overlap is safe.

</div>
</article>

<article class="doc-entry" id="v-hex-encoded-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-encoded-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-hex-encoded-length" title="Permalink">#</a><a href="/packages/stdlib/Text.Hex/source/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

The exact encoded character and byte count for a buffer of the given length: twice its length.

</div>
</article>

<article class="doc-entry" id="v-hex-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Hex/#t-HexError" title="Text.Hex.HexError">HexError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-hex-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Hex/source/#L34" title="View source at line 34">Source</a></span></div>
<div class="doc-body">

Render a diagnostic with original byte positions or the required output capacity.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Hex.cinch`](/packages/stdlib/Text.Hex/source/). Source SHA-256: `41be25b23d90a36c81618cfde993f3114916ab34e5bb5cec670fbb449de69f4c`.
