# Text.Mime

Byte-preserving media types, encoded words, immutable parameter maps and explicit extension registries.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Mime</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Mime/source/">View source</a><a href="/packages/search/?q=%2BText.Mime">Search module</a><a href="/reference-sources/Text.Mime.cinch" download="Text.Mime.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.Hash](Data.Hash.md), [Data.HashMap](Data.HashMap.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Relation.Eq](Relation.Eq.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

Media type values, raw encoded words and explicit immutable extension contexts.
Wire values and parameter values preserve arbitrary bytes. String conveniences
validate UTF-8 explicitly. Returned buffers and context observations are fresh;
input mutation cannot change retained parameters or registered associations.

</details>

<details class="doc-synopsis"><summary>Synopsis (36 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-MimeError">MimeError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MimeError</strong></code></li><li><a class="syn-name" href="#t-MimeHeaderPart">MimeHeaderPart</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MimeHeaderPart</strong></code></li><li><a class="syn-name" href="#t-MimeMediaResult">MimeMediaResult</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MimeMediaResult</strong></code></li><li><a class="syn-name" href="#t-MimeParameters">MimeParameters</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MimeParameters</strong></code></li><li><a class="syn-name" href="#t-MimeRegistry">MimeRegistry</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MimeRegistry</strong></code></li><li><a class="syn-name" href="#t-MimeWordEncoder">MimeWordEncoder</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MimeWordEncoder</strong></code></li><li><a class="syn-name" href="#t-MimeWordToken">MimeWordToken</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MimeWordToken</strong></code></li><li><a class="syn-name" href="#v-mime-b-encoding">mime-b-encoding</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-b-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeWordEncoder" title="Text.Mime.MimeWordEncoder">MimeWordEncoder</a></code></li><li><a class="syn-name" href="#v-mime-charset-name">mime-charset-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-charset-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-mime-convert-known-charset">mime-convert-known-charset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-convert-known-charset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-mime-error-message">mime-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-mime-format-media-type">mime-format-media-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-format-media-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeParameters" title="Text.Mime.MimeParameters">MimeParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-mime-header-step">mime-header-step</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-header-step</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeHeaderPart" title="Text.Mime.MimeHeaderPart">MimeHeaderPart</a></code></li><li><a class="syn-name" href="#v-mime-media-base">mime-media-base</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-media-base</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeMediaResult" title="Text.Mime.MimeMediaResult">MimeMediaResult</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-mime-media-error">mime-media-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-media-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeMediaResult" title="Text.Mime.MimeMediaResult">MimeMediaResult</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a></code></li><li><a class="syn-name" href="#v-mime-media-parameters">mime-media-parameters</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-media-parameters</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeMediaResult" title="Text.Mime.MimeMediaResult">MimeMediaResult</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeParameters" title="Text.Mime.MimeParameters">MimeParameters</a></code></li><li><a class="syn-name" href="#v-mime-parameter-find">mime-parameter-find</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-parameter-find</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeParameters" title="Text.Mime.MimeParameters">MimeParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-mime-parameter-pairs">mime-parameter-pairs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-parameter-pairs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeParameters" title="Text.Mime.MimeParameters">MimeParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-mime-parameter-set">mime-parameter-set</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-parameter-set</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeParameters" title="Text.Mime.MimeParameters">MimeParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeParameters" title="Text.Mime.MimeParameters">MimeParameters</a></code></li><li><a class="syn-name" href="#v-mime-parameters">mime-parameters</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-parameters</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeParameters" title="Text.Mime.MimeParameters">MimeParameters</a></code></li><li><a class="syn-name" href="#v-mime-parameters-empty">mime-parameters-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-parameters-empty</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeParameters" title="Text.Mime.MimeParameters">MimeParameters</a></code></li><li><a class="syn-name" href="#v-mime-parse-media-type">mime-parse-media-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-parse-media-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeMediaResult" title="Text.Mime.MimeMediaResult">MimeMediaResult</a></code></li><li><a class="syn-name" href="#v-mime-q-encoding">mime-q-encoding</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-q-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeWordEncoder" title="Text.Mime.MimeWordEncoder">MimeWordEncoder</a></code></li><li><a class="syn-name" href="#v-mime-quote-bytes">mime-quote-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-quote-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-mime-registry-add">mime-registry-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-registry-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeRegistry" title="Text.Mime.MimeRegistry">MimeRegistry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeRegistry" title="Text.Mime.MimeRegistry">MimeRegistry</a></code></li><li><a class="syn-name" href="#v-mime-registry-associations">mime-registry-associations</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-registry-associations</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeRegistry" title="Text.Mime.MimeRegistry">MimeRegistry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeRegistry" title="Text.Mime.MimeRegistry">MimeRegistry</a></code></li><li><a class="syn-name" href="#v-mime-registry-default">mime-registry-default</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-registry-default</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeRegistry" title="Text.Mime.MimeRegistry">MimeRegistry</a></code></li><li><a class="syn-name" href="#v-mime-registry-empty">mime-registry-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-registry-empty</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeRegistry" title="Text.Mime.MimeRegistry">MimeRegistry</a></code></li><li><a class="syn-name" href="#v-mime-registry-extensions">mime-registry-extensions</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-registry-extensions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeRegistry" title="Text.Mime.MimeRegistry">MimeRegistry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-mime-registry-type">mime-registry-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-registry-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeRegistry" title="Text.Mime.MimeRegistry">MimeRegistry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-mime-word-encode-bytes">mime-word-encode-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-word-encode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeWordEncoder" title="Text.Mime.MimeWordEncoder">MimeWordEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-mime-word-encode-text">mime-word-encode-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-word-encode-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeWordEncoder" title="Text.Mime.MimeWordEncoder">MimeWordEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-mime-word-encoder-byte">mime-word-encoder-byte</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-word-encoder-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeWordEncoder" title="Text.Mime.MimeWordEncoder">MimeWordEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-mime-word-token">mime-word-token</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-word-token</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeWordToken" title="Text.Mime.MimeWordToken">MimeWordToken</a></code></li><li><a class="syn-name" href="#v-mime-word-token-charset">mime-word-token-charset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-word-token-charset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeWordToken" title="Text.Mime.MimeWordToken">MimeWordToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-mime-word-token-content">mime-word-token-content</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mime-word-token-content</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeWordToken" title="Text.Mime.MimeWordToken">MimeWordToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-MimeError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MimeError</strong></code><span class="doc-sig-links"><a href="#t-MimeError" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

A precise syntax, registration or encoded-word failure without mutable payloads.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MimeNoMediaType"><code><strong class="tok-constructor">MimeNoMediaType</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a></code></li><li id="k-MimeExpectedSlash"><code><strong class="tok-constructor">MimeExpectedSlash</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a></code></li><li id="k-MimeExpectedSubtype"><code><strong class="tok-constructor">MimeExpectedSubtype</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a></code></li><li id="k-MimeUnexpectedContent"><code><strong class="tok-constructor">MimeUnexpectedContent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a></code></li><li id="k-MimeInvalidMediaParameter"><code><strong class="tok-constructor">MimeInvalidMediaParameter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a></code></li><li id="k-MimeDuplicateParameter"><code><strong class="tok-constructor">MimeDuplicateParameter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a></code></li><li id="k-MimeInvalidExtension"><code><strong class="tok-constructor">MimeInvalidExtension</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a></code></li><li id="k-MimeInvalidWord"><code><strong class="tok-constructor">MimeInvalidWord</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a></code></li><li id="k-MimeInvalidHexByte"><code><strong class="tok-constructor">MimeInvalidHexByte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 255 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a></code></li><li id="k-MimeBase64Corruption"><code><strong class="tok-constructor">MimeBase64Corruption</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a></code></li><li id="k-MimeUnhandledCharset"><code><strong class="tok-constructor">MimeUnhandledCharset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a></code></li><li id="k-MimeInvalidUtf8"><code><strong class="tok-constructor">MimeInvalidUtf8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MimeHeaderPart">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MimeHeaderPart</strong></code><span class="doc-sig-links"><a href="#t-MimeHeaderPart" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L1030" title="View source at line 1030">Source</a></span></div>
<div class="doc-body">

A single header scan observation, preserving literal prefixes and candidate spelling.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MimeHeaderEnd"><code><strong class="tok-constructor">MimeHeaderEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeHeaderPart" title="Text.Mime.MimeHeaderPart">MimeHeaderPart</a></code></li><li id="k-MimeHeaderWord"><code><strong class="tok-constructor">MimeHeaderWord</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeWordToken" title="Text.Mime.MimeWordToken">MimeWordToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeHeaderPart" title="Text.Mime.MimeHeaderPart">MimeHeaderPart</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MimeMediaResult">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MimeMediaResult</strong></code><span class="doc-sig-links"><a href="#t-MimeMediaResult" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L201" title="View source at line 201">Source</a></span></div>
<div class="doc-body">

Media parsing observations preserve a valid base but no parameter map on parameter errors.

</div>
</article>

<article class="doc-entry" id="t-MimeParameters">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MimeParameters</strong></code><span class="doc-sig-links"><a href="#t-MimeParameters" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L150" title="View source at line 150">Source</a></span></div>
<div class="doc-body">

Persistent exact-key parameter map with privately copied values.

</div>
</article>

<article class="doc-entry" id="t-MimeRegistry">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MimeRegistry</strong></code><span class="doc-sig-links"><a href="#t-MimeRegistry" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L1065" title="View source at line 1065">Source</a></span></div>
<div class="doc-body">

Explicit persistent association context. Exact spellings and lowercase
fallbacks are distinct; inverse lists preserve historical registered types.

</div>
</article>

<article class="doc-entry" id="t-MimeWordEncoder">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MimeWordEncoder</strong></code><span class="doc-sig-links"><a href="#t-MimeWordEncoder" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L769" title="View source at line 769">Source</a></span></div>
<div class="doc-body">

Preserve any U8 selector, including nonstandard values, without silently restricting it.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MimeWordEncoder"><code><strong class="tok-constructor">MimeWordEncoder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeWordEncoder" title="Text.Mime.MimeWordEncoder">MimeWordEncoder</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MimeWordToken">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MimeWordToken</strong></code><span class="doc-sig-links"><a href="#t-MimeWordToken" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L873" title="View source at line 873">Source</a></span></div>
<div class="doc-body">

A parsed encoded-word charset label and decoded transfer body.

</div>
</article>

# Values

<article class="doc-entry" id="v-mime-b-encoding">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-b-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeWordEncoder" title="Text.Mime.MimeWordEncoder">MimeWordEncoder</a></code><span class="doc-sig-links"><a href="#v-mime-b-encoding" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L772" title="View source at line 772">Source</a></span></div>
<div class="doc-body">

Standard Base64 encoded-word selector, with canonical byte value 98.

</div>
</article>

<article class="doc-entry" id="v-mime-charset-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-charset-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-mime-charset-name" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L995" title="View source at line 995">Source</a></span></div>
<div class="doc-body">

Produce the lowercase callback label, repairing each malformed source byte separately.

</div>
</article>

<article class="doc-entry" id="v-mime-convert-known-charset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-convert-known-charset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-mime-convert-known-charset" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L987" title="View source at line 987">Source</a></span></div>
<div class="doc-body">

Convert the built-in UTF-8, Latin-1 and US-ASCII charsets. UTF-8-labelled
payload bytes pass through without validation; an unknown label returns None.

</div>
</article>

<article class="doc-entry" id="v-mime-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-mime-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L83" title="View source at line 83">Source</a></span></div>
<div class="doc-body">

Render exact familiar wire diagnostics without discarding their error kind.

</div>
</article>

<article class="doc-entry" id="v-mime-format-media-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-format-media-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeParameters" title="Text.Mime.MimeParameters">MimeParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-mime-format-media-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L573" title="View source at line 573">Source</a></span></div>
<div class="doc-body">

Serialize a valid token or major/subtype pair with sorted parameter names.
Invalid type/name arguments return empty; arbitrary value bytes are preserved
through quoted or extended encoding, and all successful output is UTF-8 ASCII.

</div>
</article>

<article class="doc-entry" id="v-mime-header-step">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-header-step</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeHeaderPart" title="Text.Mime.MimeHeaderPart">MimeHeaderPart</a></code><span class="doc-sig-links"><a href="#v-mime-header-step" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L1044" title="View source at line 1044">Source</a></span></div>
<div class="doc-body">

Parse one header candidate using permissive header rules, which differ from
full-word rules: empty charsets and literal question marks in Q bodies remain observable.

</div>
</article>

<article class="doc-entry" id="v-mime-media-base">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-media-base</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeMediaResult" title="Text.Mime.MimeMediaResult">MimeMediaResult</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-mime-media-base" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L206" title="View source at line 206">Source</a></span></div>
<div class="doc-body">

Observe the normalized base, or empty when the base/duplicate policy failed.

</div>
</article>

<article class="doc-entry" id="v-mime-media-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-media-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeMediaResult" title="Text.Mime.MimeMediaResult">MimeMediaResult</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a></code><span class="doc-sig-links"><a href="#v-mime-media-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L214" title="View source at line 214">Source</a></span></div>
<div class="doc-body">

Observe the exact parse failure without flattening it to text.

</div>
</article>

<article class="doc-entry" id="v-mime-media-parameters">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-media-parameters</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeMediaResult" title="Text.Mime.MimeMediaResult">MimeMediaResult</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeParameters" title="Text.Mime.MimeParameters">MimeParameters</a></code><span class="doc-sig-links"><a href="#v-mime-media-parameters" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L210" title="View source at line 210">Source</a></span></div>
<div class="doc-body">

Observe a non-null successful parameter context, including an empty context.

</div>
</article>

<article class="doc-entry" id="v-mime-parameter-find">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-parameter-find</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeParameters" title="Text.Mime.MimeParameters">MimeParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-mime-parameter-find" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L175" title="View source at line 175">Source</a></span></div>
<div class="doc-body">

Observe one value through a fresh copy; absence remains distinct from an empty value.

</div>
</article>

<article class="doc-entry" id="v-mime-parameter-pairs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-parameter-pairs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeParameters" title="Text.Mime.MimeParameters">MimeParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-mime-parameter-pairs" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L196" title="View source at line 196">Source</a></span></div>
<div class="doc-body">

Return fresh key/value buffers in original raw-key order, before case folding.

</div>
</article>

<article class="doc-entry" id="v-mime-parameter-set">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-parameter-set</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeParameters" title="Text.Mime.MimeParameters">MimeParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeParameters" title="Text.Mime.MimeParameters">MimeParameters</a></code><span class="doc-sig-links"><a href="#v-mime-parameter-set" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L159" title="View source at line 159">Source</a></span></div>
<div class="doc-body">

Insert one exact key/value snapshot; later equal exact keys replace earlier values.

</div>
</article>

<article class="doc-entry" id="v-mime-parameters">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-parameters</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeParameters" title="Text.Mime.MimeParameters">MimeParameters</a></code><span class="doc-sig-links"><a href="#v-mime-parameters" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L164" title="View source at line 164">Source</a></span></div>
<div class="doc-body">

Construct a context in source order, so the last equal exact key wins.

</div>
</article>

<article class="doc-entry" id="v-mime-parameters-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-parameters-empty</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeParameters" title="Text.Mime.MimeParameters">MimeParameters</a></code><span class="doc-sig-links"><a href="#v-mime-parameters-empty" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L155" title="View source at line 155">Source</a></span></div>
<div class="doc-body">

Begin with an empty, non-null parameter context.

</div>
</article>

<article class="doc-entry" id="v-mime-parse-media-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-parse-media-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeMediaResult" title="Text.Mime.MimeMediaResult">MimeMediaResult</a></code><span class="doc-sig-links"><a href="#v-mime-parse-media-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L516" title="View source at line 516">Source</a></span></div>
<div class="doc-body">

Parse media syntax and RFC 2231 continuations. Successful parameters are non-null;
parameter errors retain the valid base, while conflicting duplicates clear it.

</div>
</article>

<article class="doc-entry" id="v-mime-q-encoding">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-q-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeWordEncoder" title="Text.Mime.MimeWordEncoder">MimeWordEncoder</a></code><span class="doc-sig-links"><a href="#v-mime-q-encoding" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L776" title="View source at line 776">Source</a></span></div>
<div class="doc-body">

Standard Q encoded-word selector, with canonical byte value 113.

</div>
</article>

<article class="doc-entry" id="v-mime-quote-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-quote-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-mime-quote-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L1026" title="View source at line 1026">Source</a></span></div>
<div class="doc-body">

Return an immutable quoted diagnostic spelling of arbitrary bytes.

</div>
</article>

<article class="doc-entry" id="v-mime-registry-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-registry-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeRegistry" title="Text.Mime.MimeRegistry">MimeRegistry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeRegistry" title="Text.Mime.MimeRegistry">MimeRegistry</a></code><span class="doc-sig-links"><a href="#v-mime-registry-add" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L1085" title="View source at line 1085">Source</a></span></div>
<div class="doc-body">

Register one exact association atomically. Text defaults apply only to a
literal lowercase text/ prefix, and missing or empty charsets become utf-8.

</div>
</article>

<article class="doc-entry" id="v-mime-registry-associations">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-registry-associations</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeRegistry" title="Text.Mime.MimeRegistry">MimeRegistry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeRegistry" title="Text.Mime.MimeRegistry">MimeRegistry</a></code><span class="doc-sig-links"><a href="#v-mime-registry-associations" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L1140" title="View source at line 1140">Source</a></span></div>
<div class="doc-body">

Import associations explicitly in order, returning the first registration
error without modifying the supplied context. Host-specific data is supplied
as ordinary associations rather than read through ambient initialization.

</div>
</article>

<article class="doc-entry" id="v-mime-registry-default">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-registry-default</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeRegistry" title="Text.Mime.MimeRegistry">MimeRegistry</a></code><span class="doc-sig-links"><a href="#v-mime-registry-default" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L1150" title="View source at line 1150">Source</a></span></div>
<div class="doc-body">

Stable built-in associations. Additional host-specific records are imported
through mime-registry-associations, keeping configuration explicit and reproducible.

</div>
</article>

<article class="doc-entry" id="v-mime-registry-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-registry-empty</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeRegistry" title="Text.Mime.MimeRegistry">MimeRegistry</a></code><span class="doc-sig-links"><a href="#v-mime-registry-empty" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L1070" title="View source at line 1070">Source</a></span></div>
<div class="doc-body">

Begin with no hidden host or process-wide registry state.

</div>
</article>

<article class="doc-entry" id="v-mime-registry-extensions">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-registry-extensions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeRegistry" title="Text.Mime.MimeRegistry">MimeRegistry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-mime-registry-extensions" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L1127" title="View source at line 1127">Source</a></span></div>
<div class="doc-body">

Return sorted historical lowercase extensions after full media-type validation.
Parameters do not affect the inverse lookup, but parameter errors remain visible.

</div>
</article>

<article class="doc-entry" id="v-mime-registry-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-registry-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeRegistry" title="Text.Mime.MimeRegistry">MimeRegistry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-mime-registry-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L1108" title="View source at line 1108">Source</a></span></div>
<div class="doc-body">

Resolve the exact spelling first, then its Unicode lowercase fallback.
Unknown extensions return a fresh empty buffer. Observations never alias a context.

</div>
</article>

<article class="doc-entry" id="v-mime-word-encode-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-word-encode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeWordEncoder" title="Text.Mime.MimeWordEncoder">MimeWordEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-mime-word-encode-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L856" title="View source at line 856">Source</a></span></div>
<div class="doc-body">

Encode arbitrary source bytes. Plain ASCII including tab is returned freshly
unchanged. Only UTF-8-labelled encoded words split at scalar boundaries;
other labels remain unvalidated and unsplit, with no implicit transcoding.

</div>
</article>

<article class="doc-entry" id="v-mime-word-encode-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-word-encode-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeWordEncoder" title="Text.Mime.MimeWordEncoder">MimeWordEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-mime-word-encode-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L864" title="View source at line 864">Source</a></span></div>
<div class="doc-body">

Checked String convenience retaining exact byte-valued selector behavior.
Use the byte API when a nonstandard selector creates non-UTF-8 header bytes.

</div>
</article>

<article class="doc-entry" id="v-mime-word-encoder-byte">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-word-encoder-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeWordEncoder" title="Text.Mime.MimeWordEncoder">MimeWordEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-mime-word-encoder-byte" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L780" title="View source at line 780">Source</a></span></div>
<div class="doc-body">

Observe the exact byte-valued selector used in an encoded-word header.

</div>
</article>

<article class="doc-entry" id="v-mime-word-token">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-word-token</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeError" title="Text.Mime.MimeError">MimeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeWordToken" title="Text.Mime.MimeWordToken">MimeWordToken</a></code><span class="doc-sig-links"><a href="#v-mime-word-token" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L958" title="View source at line 958">Source</a></span></div>
<div class="doc-body">

Parse a complete single encoded word with exactly four question delimiters.
Empty transfer bodies are accepted; charset labels remain unvalidated bytes.

</div>
</article>

<article class="doc-entry" id="v-mime-word-token-charset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-word-token-charset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeWordToken" title="Text.Mime.MimeWordToken">MimeWordToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-mime-word-token-charset" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L878" title="View source at line 878">Source</a></span></div>
<div class="doc-body">

Observe an independent copy of the exact unnormalized charset label.

</div>
</article>

<article class="doc-entry" id="v-mime-word-token-content">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mime-word-token-content</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Mime/#t-MimeWordToken" title="Text.Mime.MimeWordToken">MimeWordToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-mime-word-token-content" title="Permalink">#</a><a href="/packages/stdlib/Text.Mime/source/#L882" title="View source at line 882">Source</a></span></div>
<div class="doc-body">

Observe independent decoded transfer bytes before any charset conversion.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_MimeKey"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-constructor">MimeKey</span></code> <span class="doc-inst-meta">Implicit · <code>Text.Mime.Eq_MimeKey</code></span><div class="doc-child-prose">Compare only private immutable key contents, without changing buffer equality policies.</div></li>
<li id="i-Hash_MimeKey"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <span class="tok-constructor">MimeKey</span></code> <span class="doc-inst-meta">Implicit · <code>Text.Mime.Hash_MimeKey</code></span><div class="doc-child-prose">Reuse stable byte hashing only over immutable privately owned snapshots.</div></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Mime.cinch`](/packages/stdlib/Text.Mime/source/). Source SHA-256: `399fb3c576ab4f4c13a0749dfa25dc72af00078636758a86e73c942018fd46a6`.
