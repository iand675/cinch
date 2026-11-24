# Text.Utf16

UTF-16 scalar and surrogate operations, code-unit sequences and explicit-order byte codecs with SWAR ASCII blocks.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Utf16</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Utf16/source/">View source</a><a href="/packages/search/?q=%2BText.Utf16">Search module</a><a href="/reference-sources/Text.Utf16.cinch" download="Text.Utf16.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md).

<details class="doc-notes"><summary>Module notes</summary>

UTF-16 code units, surrogate pairs and explicit-byte-order text conversion.
Invalid scalar inputs and unpaired surrogates become U+FFFD. Byte decoders
reject an odd final byte; a byte-order mark is ordinary text, never inferred
or removed. Buffer codecs process four ASCII units at once using inline
LLVM SWAR widening and narrowing, with bounded loads and exact scalar tails.

</details>

<details class="doc-synopsis" open><summary>Synopsis (10 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Utf16Error">Utf16Error</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Utf16Error</strong></code></li><li><a class="syn-name" href="#v-utf16-append-rune">utf16-append-rune</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf16-append-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-utf16-decode">utf16-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf16-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-utf16-decode-bytes">utf16-decode-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf16-decode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Utf16/#t-Utf16Error" title="Text.Utf16.Utf16Error">Utf16Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-utf16-decode-rune">utf16-decode-rune</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf16-decode-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-utf16-encode">utf16-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf16-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-utf16-encode-bytes">utf16-encode-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf16-encode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-utf16-encode-rune">utf16-encode-rune</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf16-encode-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>)</code></li><li><a class="syn-name" href="#v-utf16-is-surrogate">utf16-is-surrogate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf16-is-surrogate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-utf16-rune-length">utf16-rune-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf16-rune-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Utf16Error">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Utf16Error</strong></code><span class="doc-sig-links"><a href="#t-Utf16Error" title="Permalink">#</a><a href="/packages/stdlib/Text.Utf16/source/#L24" title="View source at line 24">Source</a></span></div>
<div class="doc-body">

An incomplete byte sequence, carrying the offset of its unpaired final byte.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-OddUtf16ByteCount"><code><strong class="tok-constructor">OddUtf16ByteCount</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Utf16/#t-Utf16Error" title="Text.Utf16.Utf16Error">Utf16Error</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-utf16-append-rune">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf16-append-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-utf16-append-rune" title="Permalink">#</a><a href="/packages/stdlib/Text.Utf16/source/#L69" title="View source at line 69">Source</a></span></div>
<div class="doc-body">

Append one scalar to an immutable code-unit list, preserving its existing units.
This copies the list spine in linear time; byte codecs avoid list-spine construction.

</div>
</article>

<article class="doc-entry" id="v-utf16-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf16-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-utf16-decode" title="Permalink">#</a><a href="/packages/stdlib/Text.Utf16/source/#L79" title="View source at line 79">Source</a></span></div>
<div class="doc-body">

Decode code units in order, replacing each unpaired surrogate independently.

</div>
</article>

<article class="doc-entry" id="v-utf16-decode-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf16-decode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Utf16/#t-Utf16Error" title="Text.Utf16.Utf16Error">Utf16Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-utf16-decode-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Utf16/source/#L190" title="View source at line 190">Source</a></span></div>
<div class="doc-body">

Decode explicit-order UTF-16 bytes. An odd byte count is an error before conversion.
Unpaired surrogates become U+FFFD; byte-order marks remain in the resulting text.

</div>
</article>

<article class="doc-entry" id="v-utf16-decode-rune">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf16-decode-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-utf16-decode-rune" title="Permalink">#</a><a href="/packages/stdlib/Text.Utf16/source/#L48" title="View source at line 48">Source</a></span></div>
<div class="doc-body">

Decode a high/low surrogate pair, returning U+FFFD if either position is invalid.

</div>
</article>

<article class="doc-entry" id="v-utf16-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf16-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-utf16-encode" title="Permalink">#</a><a href="/packages/stdlib/Text.Utf16/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

Encode numeric scalar values in order; invalid scalar values each produce one replacement unit.

</div>
</article>

<article class="doc-entry" id="v-utf16-encode-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf16-encode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Binary/#t-ByteOrder" title="Text.Binary.ByteOrder">ByteOrder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-utf16-encode-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Utf16/source/#L157" title="View source at line 157">Source</a></span></div>
<div class="doc-body">

Encode valid UTF-8 text to explicit-order UTF-16 bytes, with no added byte-order mark.

</div>
</article>

<article class="doc-entry" id="v-utf16-encode-rune">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf16-encode-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>)</code><span class="doc-sig-links"><a href="#v-utf16-encode-rune" title="Permalink">#</a><a href="/packages/stdlib/Text.Utf16/source/#L39" title="View source at line 39">Source</a></span></div>
<div class="doc-body">

Encode a supplementary scalar as a surrogate pair. All other inputs return two U+FFFD units.

</div>
</article>

<article class="doc-entry" id="v-utf16-is-surrogate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf16-is-surrogate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-utf16-is-surrogate" title="Permalink">#</a><a href="/packages/stdlib/Text.Utf16/source/#L29" title="View source at line 29">Source</a></span></div>
<div class="doc-body">

Whether a numeric code point lies in the high or low surrogate range.

</div>
</article>

<article class="doc-entry" id="v-utf16-rune-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf16-rune-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-utf16-rune-length" title="Permalink">#</a><a href="/packages/stdlib/Text.Utf16/source/#L33" title="View source at line 33">Source</a></span></div>
<div class="doc-body">

One or two code units for a scalar, or None for a negative, surrogate or out-of-range input.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Utf16.cinch`](/packages/stdlib/Text.Utf16/source/). Source SHA-256: `fb5bfa4782fa832245102d8fc57818a374c4176c6a7c608f21a669030f54e8b8`.
