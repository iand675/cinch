# Text.Punycode

Punycode (RFC 3492) encoding and decoding, and ASCII-compatible host names with xn-- labels.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Punycode</code></div><div class="doc-mod-bar-links"><a href="/library/source/Text.Punycode/">View source</a><a href="/library/search/?q=%2BText.Punycode">Search module</a><a href="/reference-sources/Text.Punycode.cinch" download="Text.Punycode.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Text.Punycode`: [Text.Punycode](Text.Punycode.md).

```sh
cp seed/prelude/Text/Punycode.cinch ./Text.Punycode.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Punycode (RFC 3492): the bootstring encoding of Unicode labels in the ASCII letters, digits and hyphen
that internationalized domain names use, and the "xn--" per-label conversion of host names.

Encoding copies a label's ASCII code points, adds a hyphen when there were any, and then emits the
variable-length deltas that insert each remaining code point in increasing order, adapting the bias
after every insertion. Decoding runs the same state machine backwards and refuses digits that are not
base-36, deltas that overflow 32 bits and code points beyond U+10FFFF. Case folding and the other
IDNA mapping steps are not applied.

</details>

<details class="doc-synopsis" open><summary>Synopsis (5 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-PunycodeError">PunycodeError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PunycodeError</strong></code></li><li><a class="syn-name" href="#v-punycode-decode">punycode-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">punycode-decode</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Punycode/#t-PunycodeError" title="Text.Punycode.PunycodeError">PunycodeError</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-punycode-encode">punycode-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">punycode-encode</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Punycode/#t-PunycodeError" title="Text.Punycode.PunycodeError">PunycodeError</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-punycode-error-message">punycode-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">punycode-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Punycode/#t-PunycodeError" title="Text.Punycode.PunycodeError">PunycodeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-punycode-to-ascii">punycode-to-ascii</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">punycode-to-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Punycode/#t-PunycodeError" title="Text.Punycode.PunycodeError">PunycodeError</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-PunycodeError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PunycodeError</strong></code><span class="doc-sig-links"><a href="#t-PunycodeError" title="Permalink">#</a><a href="/library/source/Text.Punycode/#L21" title="View source at line 21">Source</a></span></div>
<div class="doc-body">

Why a label could not be converted.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PunycodeOverflow"><code><strong class="tok-constructor">PunycodeOverflow</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Punycode/#t-PunycodeError" title="Text.Punycode.PunycodeError">PunycodeError</a></code></li><li id="k-PunycodeBadDigit"><code><strong class="tok-constructor">PunycodeBadDigit</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Punycode/#t-PunycodeError" title="Text.Punycode.PunycodeError">PunycodeError</a></code></li><li id="k-PunycodeBadInput"><code><strong class="tok-constructor">PunycodeBadInput</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Punycode/#t-PunycodeError" title="Text.Punycode.PunycodeError">PunycodeError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-punycode-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">punycode-decode</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Punycode/#t-PunycodeError" title="Text.Punycode.PunycodeError">PunycodeError</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-punycode-decode" title="Permalink">#</a><a href="/library/source/Text.Punycode/#L202" title="View source at line 202">Source</a></span></div>
<div class="doc-body">

Decode a label (without the "xn--" prefix).

</div>
</article>

<article class="doc-entry" id="v-punycode-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">punycode-encode</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Punycode/#t-PunycodeError" title="Text.Punycode.PunycodeError">PunycodeError</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-punycode-encode" title="Permalink">#</a><a href="/library/source/Text.Punycode/#L141" title="View source at line 141">Source</a></span></div>
<div class="doc-body">

Encode a label (without the "xn--" prefix).

</div>
</article>

<article class="doc-entry" id="v-punycode-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">punycode-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Punycode/#t-PunycodeError" title="Text.Punycode.PunycodeError">PunycodeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-punycode-error-message" title="Permalink">#</a><a href="/library/source/Text.Punycode/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">

A readable description of an error.

</div>
</article>

<article class="doc-entry" id="v-punycode-to-ascii">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">punycode-to-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Punycode/#t-PunycodeError" title="Text.Punycode.PunycodeError">PunycodeError</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-punycode-to-ascii" title="Permalink">#</a><a href="/library/source/Text.Punycode/#L267" title="View source at line 267">Source</a></span></div>
<div class="doc-body">

A host name with every non-ASCII label encoded as "xn--" and its Punycode.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Punycode.cinch`](/library/source/Text.Punycode/). Source SHA-256: `c27491d86e8dfe07722cadc036211354ecff896d53959e94b6d534e7a72da3b6`.
