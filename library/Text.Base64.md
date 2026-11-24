# Text.Base64

Base64 encoding/decoding over bytes with explicit invalid-input results.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Base64</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Base64/source/">View source</a><a href="/packages/search/?q=%2BText.Base64">Search module</a><a href="/reference-sources/Text.Base64.cinch" download="Text.Base64.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details class="doc-notes"><summary>Module notes</summary>

Text.Base64: Base64 text to and from `Bytes` (RFC 4648).

Module scheme (design/base-library.md): `Text.*` is conversion to and
from String/Bytes; this module is the Base64 half of the Ruby surface
curated in design/stdlib-parity-ruby.md (`Base64`, the collections/
text-utilities bullet). The carrier split is decision D88's: the
encoded form is ASCII text, a `String`; the decoded form is raw bytes,
a `Bytes`.

The Ruby methods behind each name (the port's provenance):
  `base64-encode`      = `Base64.strict_encode64` (the standard
                         alphabet, no line wrapping);
  `base64-decode`      = `Base64.strict_decode64` (named errors, not
                         exceptions);
  `base64-url-encode`  = `Base64.urlsafe_encode64` (default padding);
  `base64-url-decode`  = `Base64.urlsafe_decode64`;
  `base64-decode-lenient` = `Base64.decode64` (every non-alphabet
                         character, line breaks included, is ignored
                         and the result is the bytes; it never fails).
The `Bytes`/`String` distinction is carried entirely by the
signatures (encode takes `Bytes`, yields `String`; decode the
other way), so no duplicate spellings are needed.

DESIGN LAW SHAPE (RFC 4648 section 4; the round trip is the module's
contract):
  base64-decode (base64-encode b) == Ok b         for every Bytes
  base64-decode-lenient (base64-encode b) == b    for every Bytes
  string-byte-length (base64-encode b) == 4 * ceil (bytes-len b / 3)
and the padding law: the encoding of an n-byte buffer ends in
`=` * ((3 - n rem 3) rem 3) pad characters, so lengths 0/3 mod 3
carry no pad, 2 mod 3 one pad, 1 mod 3 two. The canonical alphabet is
used for encoding; decoding accepts exactly it (plus `=` in the
padding positions).

Named refusals (the strict decoder is total over its own alphabet and
refuses everything else with a named error, never a fault):
  a character outside the alphabet (`InvalidBase64Character`): the
    standard decoder refuses `-`/`_`, the URL-safe decoder refuses
    nothing the standard one accepts (Ruby's union behaviour);
  a length that is not a multiple of 4 (`InvalidBase64Length`): the
    input can never be a complete sequence of 4-character groups;
  a misplaced or excessive pad (`InvalidBase64Padding`): `=` is legal
    only in the last group's last one or two positions, never earlier
    and never as a third pad.
Not carried (unproductions this module deliberately does not build):
  line wrapping (Ruby's `encode64`, 60 characters plus newline) and
  the MIME mode; the decoder's whitespace tolerance is the only nod to
  wrapped input, through `base64-decode-lenient`.

</details>

<details class="doc-synopsis" open><summary>Synopsis (6 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Base64Error">Base64Error</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base64Error</strong></code></li><li><a class="syn-name" href="#v-base64-decode">base64-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64/#t-Base64Error" title="Text.Base64.Base64Error">Base64Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-base64-decode-lenient">base64-decode-lenient</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-decode-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-base64-encode">base64-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-base64-url-decode">base64-url-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-url-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64/#t-Base64Error" title="Text.Base64.Base64Error">Base64Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-base64-url-encode">base64-url-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-url-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Base64Error">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base64Error</strong></code><span class="doc-sig-links"><a href="#t-Base64Error" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64/source/#L68" title="View source at line 68">Source</a></span></div>
<div class="doc-body">

The named decode failures (the house style: Text.Parse's named
vocabulary shape, never a fault).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidBase64Character"><code><strong class="tok-constructor">InvalidBase64Character</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64/#t-Base64Error" title="Text.Base64.Base64Error">Base64Error</a></code></li><li id="k-InvalidBase64Length"><code><strong class="tok-constructor">InvalidBase64Length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64/#t-Base64Error" title="Text.Base64.Base64Error">Base64Error</a></code></li><li id="k-InvalidBase64Padding"><code><strong class="tok-constructor">InvalidBase64Padding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64/#t-Base64Error" title="Text.Base64.Base64Error">Base64Error</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-base64-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64/#t-Base64Error" title="Text.Base64.Base64Error">Base64Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-base64-decode" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64/source/#L191" title="View source at line 191">Source</a></span></div>
<div class="doc-body">

Strict decoding.

</div>
</article>

<article class="doc-entry" id="v-base64-decode-lenient">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-decode-lenient</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-base64-decode-lenient" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64/source/#L310" title="View source at line 310">Source</a></span></div>
<div class="doc-body">

Lenient decoding (Ruby's `decode64`): drop every non-alphabet
scalar and pack what remains, never failing.

</div>
</article>

<article class="doc-entry" id="v-base64-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-base64-encode" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64/source/#L129" title="View source at line 129">Source</a></span></div>
<div class="doc-body">

Encoding.

</div>
</article>

<article class="doc-entry" id="v-base64-url-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-url-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64/#t-Base64Error" title="Text.Base64.Base64Error">Base64Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-base64-url-decode" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64/source/#L194" title="View source at line 194">Source</a></span></div>
</article>

<article class="doc-entry" id="v-base64-url-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-url-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-base64-url-encode" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64/source/#L132" title="View source at line 132">Source</a></span></div>
</article>
