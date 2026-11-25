# Text.Asn1

ASN.1 value trees with a strict DER decoder and canonical DER encoder, including explicit and implicit tags and times.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Asn1</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Asn1/source/">View source</a><a href="/packages/search/?q=%2BText.Asn1">Search module</a><a href="/reference-sources/Text.Asn1.cinch" download="Text.Asn1.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details class="doc-notes"><summary>Module notes</summary>

ASN.1 values in DER: a typed value tree with a strict decoder and a canonical encoder.
Values cover booleans, arbitrary-precision integers and enumerations, bit and
octet strings, null, object identifiers, the common character string types,
UTC and generalized times, sequences, sets and raw values of any class and tag
(for explicit and implicit tagging). Decoding enforces DER: definite minimal
lengths, minimal two's-complement integers, booleans of 0x00 or 0xFF, zero
padding bits in bit strings, minimal object identifier arcs and no trailing
data. Encoding produces DER, sorting the members of a set by their encodings.

</details>

<details class="doc-synopsis"><summary>Synopsis (21 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Asn1">Asn1</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Asn1</strong></code></li><li><a class="syn-name" href="#t-Asn1Error">Asn1Error</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Asn1Error</strong></code></li><li><a class="syn-name" href="#v-asn1-decode">asn1-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">asn1-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1Error" title="Text.Asn1.Asn1Error">Asn1Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li><a class="syn-name" href="#v-asn1-encode">asn1-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">asn1-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-asn1-error-message">asn1-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">asn1-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1Error" title="Text.Asn1.Asn1Error">Asn1Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-asn1-explicit">asn1-explicit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">asn1-explicit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li><a class="syn-name" href="#v-asn1-implicit">asn1-implicit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">asn1-implicit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li><a class="syn-name" href="#v-asn1-members">asn1-members</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">asn1-members</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li><a class="syn-name" href="#v-asn1-sequence">asn1-sequence</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">asn1-sequence</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li><a class="syn-name" href="#v-asn1-set">asn1-set</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">asn1-set</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li><a class="syn-name" href="#v-asn1-time-seconds">asn1-time-seconds</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">asn1-time-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-class-application">class-application</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">class-application</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-class-context">class-context</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">class-context</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-class-private">class-private</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">class-private</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-class-universal">class-universal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">class-universal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tag-bmp-string">tag-bmp-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tag-bmp-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tag-ia5-string">tag-ia5-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tag-ia5-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tag-numeric-string">tag-numeric-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tag-numeric-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tag-printable-string">tag-printable-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tag-printable-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tag-t61-string">tag-t61-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tag-t61-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tag-utf8-string">tag-utf8-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tag-utf8-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Asn1">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Asn1</strong></code><span class="doc-sig-links"><a href="#t-Asn1" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L68" title="View source at line 68">Source</a></span></div>
<div class="doc-body">

An ASN.1 value.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Asn1Boolean"><code><strong class="tok-constructor">Asn1Boolean</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li id="k-Asn1Integer"><code><strong class="tok-constructor">Asn1Integer</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li id="k-Asn1Enumerated"><code><strong class="tok-constructor">Asn1Enumerated</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li id="k-Asn1BitString"><code><strong class="tok-constructor">Asn1BitString</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li id="k-Asn1OctetString"><code><strong class="tok-constructor">Asn1OctetString</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li id="k-Asn1Null"><code><strong class="tok-constructor">Asn1Null</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li id="k-Asn1Oid"><code><strong class="tok-constructor">Asn1Oid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li id="k-Asn1String"><code><strong class="tok-constructor">Asn1String</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li id="k-Asn1UtcTime"><code><strong class="tok-constructor">Asn1UtcTime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li id="k-Asn1GeneralizedTime"><code><strong class="tok-constructor">Asn1GeneralizedTime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li id="k-Asn1Sequence"><code><strong class="tok-constructor">Asn1Sequence</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li id="k-Asn1Set"><code><strong class="tok-constructor">Asn1Set</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li id="k-Asn1Member"><code><strong class="tok-constructor">Asn1Member</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li id="k-Asn1End"><code><strong class="tok-constructor">Asn1End</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li id="k-Asn1Raw"><code><strong class="tok-constructor">Asn1Raw</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Asn1Error">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Asn1Error</strong></code><span class="doc-sig-links"><a href="#t-Asn1Error" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L124" title="View source at line 124">Source</a></span></div>
<div class="doc-body">

A DER violation; offsets locate the element.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Asn1Truncated"><code><strong class="tok-constructor">Asn1Truncated</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1Error" title="Text.Asn1.Asn1Error">Asn1Error</a></code></li><li id="k-Asn1BadLength"><code><strong class="tok-constructor">Asn1BadLength</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1Error" title="Text.Asn1.Asn1Error">Asn1Error</a></code></li><li id="k-Asn1BadContent"><code><strong class="tok-constructor">Asn1BadContent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1Error" title="Text.Asn1.Asn1Error">Asn1Error</a></code></li><li id="k-Asn1TrailingData"><code><strong class="tok-constructor">Asn1TrailingData</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1Error" title="Text.Asn1.Asn1Error">Asn1Error</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-asn1-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">asn1-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1Error" title="Text.Asn1.Asn1Error">Asn1Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code><span class="doc-sig-links"><a href="#v-asn1-decode" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L166" title="View source at line 166">Source</a></span></div>
<div class="doc-body">

Decode exactly one DER value occupying the whole input.

</div>
</article>

<article class="doc-entry" id="v-asn1-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">asn1-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-asn1-encode" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L333" title="View source at line 333">Source</a></span></div>
<div class="doc-body">

Encode a value in DER.

</div>
</article>

<article class="doc-entry" id="v-asn1-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">asn1-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1Error" title="Text.Asn1.Asn1Error">Asn1Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-asn1-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L135" title="View source at line 135">Source</a></span></div>
<div class="doc-body">

Render an ASN.1 error.

</div>
</article>

<article class="doc-entry" id="v-asn1-explicit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">asn1-explicit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code><span class="doc-sig-links"><a href="#v-asn1-explicit" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L143" title="View source at line 143">Source</a></span></div>
<div class="doc-body">

Wrap a value in an explicit context-specific tag.

</div>
</article>

<article class="doc-entry" id="v-asn1-implicit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">asn1-implicit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code><span class="doc-sig-links"><a href="#v-asn1-implicit" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L147" title="View source at line 147">Source</a></span></div>
<div class="doc-body">

Replace a value's identifier with an implicit context-specific tag, keeping its content and form.

</div>
</article>

<article class="doc-entry" id="v-asn1-members">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">asn1-members</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code><span class="doc-sig-links"><a href="#v-asn1-members" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L109" title="View source at line 109">Source</a></span></div>
<div class="doc-body">

Members of a SEQUENCE or SET in order; other values have none.

</div>
</article>

<article class="doc-entry" id="v-asn1-sequence">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">asn1-sequence</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code><span class="doc-sig-links"><a href="#v-asn1-sequence" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L101" title="View source at line 101">Source</a></span></div>
<div class="doc-body">

A SEQUENCE of the given members.

</div>
</article>

<article class="doc-entry" id="v-asn1-set">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">asn1-set</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code><span class="doc-sig-links"><a href="#v-asn1-set" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L105" title="View source at line 105">Source</a></span></div>
<div class="doc-body">

A SET of the given members.

</div>
</article>

<article class="doc-entry" id="v-asn1-time-seconds">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">asn1-time-seconds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-asn1-time-seconds" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L469" title="View source at line 469">Source</a></span></div>
<div class="doc-body">

Unix seconds of a UTCTime (years 50-99 are 1950-1999) or GeneralizedTime in the Z form, ignoring
fractional seconds; None for any other value or text.

</div>
</article>

<article class="doc-entry" id="v-class-application">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">class-application</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-class-application" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L56" title="View source at line 56">Source</a></span></div>
<div class="doc-body">

The application class.

</div>
</article>

<article class="doc-entry" id="v-class-context">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">class-context</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-class-context" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L60" title="View source at line 60">Source</a></span></div>
<div class="doc-body">

The context-specific class, used for tagged fields.

</div>
</article>

<article class="doc-entry" id="v-class-private">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">class-private</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-class-private" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L64" title="View source at line 64">Source</a></span></div>
<div class="doc-body">

The private class.

</div>
</article>

<article class="doc-entry" id="v-class-universal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">class-universal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-class-universal" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L52" title="View source at line 52">Source</a></span></div>
<div class="doc-body">

The universal class.

</div>
</article>

<article class="doc-entry" id="v-tag-bmp-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tag-bmp-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tag-bmp-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L48" title="View source at line 48">Source</a></span></div>
<div class="doc-body">

Universal tag of BMPString.

</div>
</article>

<article class="doc-entry" id="v-tag-ia5-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tag-ia5-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tag-ia5-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L44" title="View source at line 44">Source</a></span></div>
<div class="doc-body">

Universal tag of IA5String.

</div>
</article>

<article class="doc-entry" id="v-tag-numeric-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tag-numeric-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tag-numeric-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L32" title="View source at line 32">Source</a></span></div>
<div class="doc-body">

Universal tag of NumericString.

</div>
</article>

<article class="doc-entry" id="v-tag-printable-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tag-printable-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tag-printable-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L36" title="View source at line 36">Source</a></span></div>
<div class="doc-body">

Universal tag of PrintableString.

</div>
</article>

<article class="doc-entry" id="v-tag-t61-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tag-t61-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tag-t61-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L40" title="View source at line 40">Source</a></span></div>
<div class="doc-body">

Universal tag of T61String (TeletexString).

</div>
</article>

<article class="doc-entry" id="v-tag-utf8-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tag-utf8-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tag-utf8-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Asn1/source/#L28" title="View source at line 28">Source</a></span></div>
<div class="doc-body">

Universal tag of UTF8String.

</div>
</article>
