# Text.Html

HTML text/byte escaping, complete HTML5 character references, exact escaped sizing and SIMD/SWAR scanning.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Html</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Html/source/">View source</a><a href="/packages/search/?q=%2BText.Html">Search module</a><a href="/reference-sources/Text.Html.cinch" download="Text.Html.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md).

<details class="doc-notes"><summary>Module notes</summary>

HTML text escaping and HTML5 named and numeric character references.
Escaping replaces only ampersand, both quotation marks and angle brackets.
Unescaping recognizes semicolon-terminated names, the documented legacy
semicolon-free names, decimal/hexadecimal numeric references and Windows-1252
numeric replacements. Unknown references remain unchanged. Reference expansion
is a single pass: newly decoded ampersands are not decoded again.
Byte APIs preserve unrelated arbitrary bytes; text APIs preserve valid UTF-8.
All returned byte buffers are fresh. Complete scanning blocks use portable
16-byte SIMD classification over checked byte buffers.

</details>

<details class="doc-synopsis" open><summary>Synopsis (5 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-html-escape">html-escape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-html-escape-bytes">html-escape-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-escape-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-html-escaped-length">html-escaped-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-escaped-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-html-unescape">html-unescape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-unescape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-html-unescape-bytes">html-unescape-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-unescape-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-html-escape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-html-escape" title="Permalink">#</a><a href="/packages/stdlib/Text.Html/source/#L217" title="View source at line 217">Source</a></span></div>
<div class="doc-body">

Escape ampersand, apostrophe, double quotation mark and angle brackets in UTF-8 text.
Apostrophes use &#39; and double quotation marks use &#34;.

</div>
</article>

<article class="doc-entry" id="v-html-escape-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-escape-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-html-escape-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Html/source/#L212" title="View source at line 212">Source</a></span></div>
<div class="doc-body">

Escape arbitrary input bytes into fresh storage. Unrelated non-UTF-8 bytes are preserved exactly.

</div>
</article>

<article class="doc-entry" id="v-html-escaped-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-escaped-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-html-escaped-length" title="Permalink">#</a><a href="/packages/stdlib/Text.Html/source/#L83" title="View source at line 83">Source</a></span></div>
<div class="doc-body">

Exact escaped byte count. Only five ASCII bytes expand; all other bytes retain their size.

</div>
</article>

<article class="doc-entry" id="v-html-unescape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-unescape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-html-unescape" title="Permalink">#</a><a href="/packages/stdlib/Text.Html/source/#L421" title="View source at line 421">Source</a></span></div>
<div class="doc-body">

Decode named and numeric HTML references once, preserving unknown reference text.
Legacy names can omit a semicolon; invalid numeric scalars become U+FFFD.
Numeric accumulation wraps at 32 bits before scalar replacement.

</div>
</article>

<article class="doc-entry" id="v-html-unescape-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-unescape-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-html-unescape-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Html/source/#L415" title="View source at line 415">Source</a></span></div>
<div class="doc-body">

Decode HTML5 references into fresh arbitrary bytes, retaining unrelated bytes exactly.
Complete named references may expand: for example, &nLt; produces two scalars and six UTF-8 bytes.

</div>
</article>
