# Text.TextProto

Text-protocol framing: lines, canonical MIME headers, status responses and dot encoding for HTTP, SMTP and mail.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.TextProto</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.TextProto/source/">View source</a><a href="/packages/search/?q=%2BText.TextProto">Search module</a><a href="/reference-sources/Text.TextProto.cinch" download="Text.TextProto.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details class="doc-closure"><summary>Ship-source closure (3 files)</summary>

Copy these modules beside your entry file before importing `Text.TextProto`: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Text.TextProto](Text.TextProto.md).

```sh
cp seed/prelude/Data/Bytes/Build.cinch ./Data.Bytes.Build.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Text/TextProto.cinch ./Text.TextProto.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Text protocol framing shared by HTTP, SMTP, NNTP and mail: lines, MIME headers, status responses and dot encoding.
Readers take a buffer and an offset and return the value and the offset
after it, so a caller can resume where a reader stopped. Lines end at LF
with an optional preceding CR. MIME headers are an ordered multimap keyed
by canonical header names: canonicalization lowercases eight bytes at a
time with SWAR, validates them as header-name bytes, and then uppercases the
first letter of every hyphen-separated word; keys with invalid bytes are
kept as given. Dot encoding stuffs leading dots and normalizes line endings
to CRLF, and dot decoding reverses it up to the terminating dot line.

</details>

<details class="doc-synopsis"><summary>Synopsis (21 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-MimeHeader">MimeHeader</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MimeHeader</strong></code></li><li><a class="syn-name" href="#t-TextProtoError">TextProtoError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TextProtoError</strong></code></li><li><a class="syn-name" href="#v-canonical-header-key">canonical-header-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">canonical-header-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-dot-encode">dot-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dot-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-header-add">header-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">header-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a></code></li><li><a class="syn-name" href="#v-header-delete">header-delete</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">header-delete</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a></code></li><li><a class="syn-name" href="#v-header-entries">header-entries</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">header-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-header-get">header-get</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">header-get</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-header-keys">header-keys</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">header-keys</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-header-set">header-set</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">header-set</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a></code></li><li><a class="syn-name" href="#v-header-values">header-values</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">header-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-mime-header">mime-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a></code></li><li><a class="syn-name" href="#v-read-code-line">read-code-line</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">read-code-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-TextProtoError" title="Text.TextProto.TextProtoError">TextProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-read-continued-line">read-continued-line</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">read-continued-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-TextProtoError" title="Text.TextProto.TextProtoError">TextProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-read-dot-bytes">read-dot-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">read-dot-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-TextProtoError" title="Text.TextProto.TextProtoError">TextProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-read-line">read-line</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">read-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-TextProtoError" title="Text.TextProto.TextProtoError">TextProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-read-mime-header">read-mime-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">read-mime-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-TextProtoError" title="Text.TextProto.TextProtoError">TextProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-read-response">read-response</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">read-response</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-TextProtoError" title="Text.TextProto.TextProtoError">TextProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-render-header">render-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">render-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-text-proto-error-message">text-proto-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">text-proto-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-TextProtoError" title="Text.TextProto.TextProtoError">TextProtoError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-trim-bytes">trim-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">trim-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-MimeHeader">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MimeHeader</strong></code><span class="doc-sig-links"><a href="#t-MimeHeader" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L46" title="View source at line 46">Source</a></span></div>
<div class="doc-body">

Header fields in first-insertion order of their canonical keys, each with its values in order.

</div>
</article>

<article class="doc-entry" id="t-TextProtoError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TextProtoError</strong></code><span class="doc-sig-links"><a href="#t-TextProtoError" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">

A malformed or truncated protocol element; offsets locate the offending line.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UnexpectedEnd"><code><strong class="tok-constructor">UnexpectedEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-TextProtoError" title="Text.TextProto.TextProtoError">TextProtoError</a></code></li><li id="k-MalformedHeader"><code><strong class="tok-constructor">MalformedHeader</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-TextProtoError" title="Text.TextProto.TextProtoError">TextProtoError</a></code></li><li id="k-MalformedCode"><code><strong class="tok-constructor">MalformedCode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-TextProtoError" title="Text.TextProto.TextProtoError">TextProtoError</a></code></li><li id="k-UnexpectedCode"><code><strong class="tok-constructor">UnexpectedCode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-TextProtoError" title="Text.TextProto.TextProtoError">TextProtoError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-canonical-header-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">canonical-header-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-canonical-header-key" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L166" title="View source at line 166">Source</a></span></div>
<div class="doc-body">

Canonical form of a header name: "content-TYPE" becomes "Content-Type". A name containing a byte
that is not a header token, or a space, is returned unchanged.

</div>
</article>

<article class="doc-entry" id="v-dot-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dot-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-dot-encode" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L418" title="View source at line 418">Source</a></span></div>
<div class="doc-body">

Dot-encode a body: a dot doubled at the start of each line, a lone LF expanded to CRLF, existing
CRLF kept, and the body closed with CRLF (unless it already ended a line) and ".\r\n".

</div>
</article>

<article class="doc-entry" id="v-header-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">header-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a></code><span class="doc-sig-links"><a href="#v-header-add" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L55" title="View source at line 55">Source</a></span></div>
<div class="doc-body">

Append a value to a key's values; the key is canonicalized.

</div>
</article>

<article class="doc-entry" id="v-header-delete">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">header-delete</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a></code><span class="doc-sig-links"><a href="#v-header-delete" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L88" title="View source at line 88">Source</a></span></div>
<div class="doc-body">

Remove a key and all its values.

</div>
</article>

<article class="doc-entry" id="v-header-entries">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">header-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-header-entries" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L106" title="View source at line 106">Source</a></span></div>
<div class="doc-body">

Every field with its values.

</div>
</article>

<article class="doc-entry" id="v-header-get">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">header-get</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-header-get" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

The first value for a key, if any.

</div>
</article>

<article class="doc-entry" id="v-header-keys">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">header-keys</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-header-keys" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L98" title="View source at line 98">Source</a></span></div>
<div class="doc-body">

Canonical keys in order.

</div>
</article>

<article class="doc-entry" id="v-header-set">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">header-set</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a></code><span class="doc-sig-links"><a href="#v-header-set" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

Replace a key's values with a single value.

</div>
</article>

<article class="doc-entry" id="v-header-values">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">header-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-header-values" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

Every value for a key, in order.

</div>
</article>

<article class="doc-entry" id="v-mime-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a></code><span class="doc-sig-links"><a href="#v-mime-header" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L51" title="View source at line 51">Source</a></span></div>
<div class="doc-body">

An empty header.

</div>
</article>

<article class="doc-entry" id="v-read-code-line">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">read-code-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-TextProtoError" title="Text.TextProto.TextProtoError">TextProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-read-code-line" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L320" title="View source at line 320">Source</a></span></div>
<div class="doc-body">

Read a single status line "ddd message" and check its code against an expected prefix (0 accepts any);
a continuation line "ddd-message" is malformed here. Returns the code, message and next offset.

</div>
</article>

<article class="doc-entry" id="v-read-continued-line">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">read-continued-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-TextProtoError" title="Text.TextProto.TextProtoError">TextProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-read-continued-line" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L250" title="View source at line 250">Source</a></span></div>
<div class="doc-body">

Read a line and any continuation lines (beginning with space or tab), joined by single spaces,
with surrounding whitespace removed.

</div>
</article>

<article class="doc-entry" id="v-read-dot-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">read-dot-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-TextProtoError" title="Text.TextProto.TextProtoError">TextProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-read-dot-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L372" title="View source at line 372">Source</a></span></div>
<div class="doc-body">

Decode a dot-encoded body up to the terminating ".\r\n" or ".\n": a leading dot is removed from each
line, CRLF becomes LF, and a CR not followed by LF is kept. Returns the body and the offset after the terminator.

</div>
</article>

<article class="doc-entry" id="v-read-line">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">read-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-TextProtoError" title="Text.TextProto.TextProtoError">TextProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-read-line" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L243" title="View source at line 243">Source</a></span></div>
<div class="doc-body">

Read one line without its line ending (LF or CRLF); returns the line and the offset after the LF.

</div>
</article>

<article class="doc-entry" id="v-read-mime-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">read-mime-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-TextProtoError" title="Text.TextProto.TextProtoError">TextProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-read-mime-header" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L265" title="View source at line 265">Source</a></span></div>
<div class="doc-body">

Read header lines up to and including the blank line that ends them.

</div>
</article>

<article class="doc-entry" id="v-read-response">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">read-response</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-TextProtoError" title="Text.TextProto.TextProtoError">TextProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-read-response" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L332" title="View source at line 332">Source</a></span></div>
<div class="doc-body">

Read a possibly multi-line response: lines "ddd-text" continue it, and a line "ddd text" with the same
code ends it; other lines inside are kept verbatim. Message lines are joined with LF.

</div>
</article>

<article class="doc-entry" id="v-render-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">render-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-render-header" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L448" title="View source at line 448">Source</a></span></div>
<div class="doc-body">

Render header fields as "Key: value\r\n" lines in order, followed by the blank line.

</div>
</article>

<article class="doc-entry" id="v-text-proto-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">text-proto-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-TextProtoError" title="Text.TextProto.TextProtoError">TextProtoError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-text-proto-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L38" title="View source at line 38">Source</a></span></div>
<div class="doc-body">

Render a text protocol error.

</div>
</article>

<article class="doc-entry" id="v-trim-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">trim-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-trim-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.TextProto/source/#L221" title="View source at line 221">Source</a></span></div>
<div class="doc-body">

Remove leading and trailing ASCII whitespace (space, tab, CR, LF).

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/TextProto.cinch`](/packages/stdlib/Text.TextProto/source/). Source SHA-256: `3bbe1d9e1cb3ceec6392119510c325ef7cac6357b3cd43a891cd7ce424c06a1b`.
