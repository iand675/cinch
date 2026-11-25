# Text.Url.Codec

Checked percent codecs for URL components, exact escaped sizing and optional UTF-8 text views.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Url.Codec</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Url.Codec/source/">View source</a><a href="/packages/search/?q=%2BText.Url.Codec">Search module</a><a href="/reference-sources/Text.Url.Codec.cinch" download="Text.Url.Codec.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Text.Url.Error](Text.Url.Error.md).

<details class="doc-notes"><summary>Module notes</summary>

Byte-preserving percent escaping for URL components.
Query components map spaces to plus and decode plus as space. Other contexts
preserve plus. Percent decoding validates the complete input before returning
fresh bytes; arbitrary decoded bytes never pass through a text conversion.
Host and zone policies enforce their additional escape restrictions. Portable
16-byte SIMD classifiers skip complete ordinary spans over checked byte buffers.
Checked bulk copies write percent triplets and ordinary runs.

</details>

<details class="doc-synopsis" open><summary>Synopsis (12 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-UrlEncoding">UrlEncoding</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlEncoding</strong></code></li><li><a class="syn-name" href="#v-url-escape">url-escape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-escaped-length">url-escaped-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-escaped-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-url-path-escape">url-path-escape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-path-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-path-escape-text">url-path-escape-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-path-escape-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-url-path-unescape">url-path-unescape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-path-unescape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-query-escape">url-query-escape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-query-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-query-escape-text">url-query-escape-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-query-escape-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-url-query-unescape">url-query-unescape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-query-unescape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-unescape">url-unescape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-unescape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-unescape-fragment-hint">url-unescape-fragment-hint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-unescape-fragment-hint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-unescape-path-hint">url-unescape-path-hint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-unescape-path-hint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-UrlEncoding">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlEncoding</strong></code><span class="doc-sig-links"><a href="#t-UrlEncoding" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">

The independent byte grammar of one URL component.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UrlPath"><code><strong class="tok-constructor">UrlPath</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a></code></li><li id="k-UrlPathSegment"><code><strong class="tok-constructor">UrlPathSegment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a></code></li><li id="k-UrlHost"><code><strong class="tok-constructor">UrlHost</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a></code></li><li id="k-UrlZone"><code><strong class="tok-constructor">UrlZone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a></code></li><li id="k-UrlUserPassword"><code><strong class="tok-constructor">UrlUserPassword</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a></code></li><li id="k-UrlQueryComponent"><code><strong class="tok-constructor">UrlQueryComponent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a></code></li><li id="k-UrlFragment"><code><strong class="tok-constructor">UrlFragment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-url-escape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-escape" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L276" title="View source at line 276">Source</a></span></div>
<div class="doc-body">

Encode one complete component into fresh bytes, preserving no caller aliases.

</div>
</article>

<article class="doc-entry" id="v-url-escaped-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-escaped-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-url-escaped-length" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L187" title="View source at line 187">Source</a></span></div>
<div class="doc-body">

Exact percent-encoded byte count; query-space substitution does not change length.

</div>
</article>

<article class="doc-entry" id="v-url-path-escape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-path-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-path-escape" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L476" title="View source at line 476">Source</a></span></div>
<div class="doc-body">

Escape one arbitrary-byte path segment, including every slash inside the segment.

</div>
</article>

<article class="doc-entry" id="v-url-path-escape-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-path-escape-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-url-path-escape-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L492" title="View source at line 492">Source</a></span></div>
<div class="doc-body">

Escape UTF-8 text as an ASCII path segment; decoding remains byte based.

</div>
</article>

<article class="doc-entry" id="v-url-path-unescape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-path-unescape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-path-unescape" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L480" title="View source at line 480">Source</a></span></div>
<div class="doc-body">

Percent-decode a path component without treating plus as a space.

</div>
</article>

<article class="doc-entry" id="v-url-query-escape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-query-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-query-escape" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L484" title="View source at line 484">Source</a></span></div>
<div class="doc-body">

Escape one arbitrary-byte query key or value, using plus for space.

</div>
</article>

<article class="doc-entry" id="v-url-query-escape-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-query-escape-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-url-query-escape-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L496" title="View source at line 496">Source</a></span></div>
<div class="doc-body">

Escape UTF-8 text as an ASCII query component; decoding remains byte based.

</div>
</article>

<article class="doc-entry" id="v-url-query-unescape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-query-unescape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-query-unescape" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L488" title="View source at line 488">Source</a></span></div>
<div class="doc-body">

Decode a query key or value, mapping plus to space and retaining arbitrary decoded bytes.

</div>
</article>

<article class="doc-entry" id="v-url-unescape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-unescape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-unescape" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L411" title="View source at line 411">Source</a></span></div>
<div class="doc-body">

Decode a complete component to fresh arbitrary bytes, or return a precise failure without partial output.

</div>
</article>

<article class="doc-entry" id="v-url-unescape-fragment-hint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-unescape-fragment-hint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-unescape-fragment-hint" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L472" title="View source at line 472">Source</a></span></div>
<div class="doc-body">

Decode a raw fragment hint only when its pchar spellings and percent triplets are valid.

</div>
</article>

<article class="doc-entry" id="v-url-unescape-path-hint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-unescape-path-hint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-unescape-path-hint" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L468" title="View source at line 468">Source</a></span></div>
<div class="doc-body">

Decode a raw path hint only when its pchar spellings and percent triplets are valid.

</div>
</article>
