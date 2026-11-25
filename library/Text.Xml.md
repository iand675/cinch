# Text.Xml

Decode XML into namespace-aware tokens or element trees and encode tokens with escaping and indentation.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Xml</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Xml/source/">View source</a><a href="/packages/search/?q=%2BText.Xml">Search module</a><a href="/reference-sources/Text.Xml.cinch" download="Text.Xml.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

XML 1.0 tokenizing, namespace-aware decoding, encoding and element trees.
The decoder is an explicit state machine over the input bytes: each step
yields one token (start and end elements, character data, comments,
processing instructions and directives) and the advanced decoder. Element
and attribute prefixes are translated to their namespace URLs, end tags must
match their start tags, self-closing elements yield a start and an end, and
CDATA sections become character data. Character and the five predefined
entity references are expanded, line endings are normalized to line feeds,
and text is checked for valid UTF-8 and the XML character range; names
follow XML 1.0's name-character classes. Errors carry the line where they
were detected. Plain runs of text are scanned sixteen bytes at a time.
The encoder writes tokens with escaping, optional indentation and generated
namespace prefixes for qualified attributes, and checks that tags balance.
An element tree (a sibling spine of nodes) can be built from a document.

</details>

<details class="doc-synopsis"><summary>Synopsis (33 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-XmlAttr">XmlAttr</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">XmlAttr</strong></code></li><li><a class="syn-name" href="#t-XmlDecoder">XmlDecoder</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">XmlDecoder</strong></code></li><li><a class="syn-name" href="#t-XmlEncoder">XmlEncoder</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">XmlEncoder</strong></code></li><li><a class="syn-name" href="#t-XmlError">XmlError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">XmlError</strong></code></li><li><a class="syn-name" href="#t-XmlName">XmlName</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">XmlName</strong></code></li><li><a class="syn-name" href="#t-XmlNode">XmlNode</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">XmlNode</strong></code></li><li><a class="syn-name" href="#t-XmlToken">XmlToken</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">XmlToken</strong></code></li><li><a class="syn-name" href="#v-xml-attr">xml-attr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-attr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlName" title="Text.Xml.XmlName">XmlName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlAttr" title="Text.Xml.XmlAttr">XmlAttr</a></code></li><li><a class="syn-name" href="#v-xml-attr-name">xml-attr-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-attr-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlAttr" title="Text.Xml.XmlAttr">XmlAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlName" title="Text.Xml.XmlName">XmlName</a></code></li><li><a class="syn-name" href="#v-xml-attr-value">xml-attr-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-attr-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlAttr" title="Text.Xml.XmlAttr">XmlAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-xml-attribute">xml-attribute</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-attribute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-xml-child-elements">xml-child-elements</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-child-elements</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a></code></li><li><a class="syn-name" href="#v-xml-decoder">xml-decoder</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-decoder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlDecoder" title="Text.Xml.XmlDecoder">XmlDecoder</a></code></li><li><a class="syn-name" href="#v-xml-decoder-offset">xml-decoder-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-decoder-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlDecoder" title="Text.Xml.XmlDecoder">XmlDecoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-xml-decoder-with-depth-limit">xml-decoder-with-depth-limit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-decoder-with-depth-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlDecoder" title="Text.Xml.XmlDecoder">XmlDecoder</a></code></li><li><a class="syn-name" href="#v-xml-document">xml-document</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-document</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a></code></li><li><a class="syn-name" href="#v-xml-encode">xml-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlEncoder" title="Text.Xml.XmlEncoder">XmlEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlToken" title="Text.Xml.XmlToken">XmlToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-xml-encode-token">xml-encode-token</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-encode-token</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlEncoder" title="Text.Xml.XmlEncoder">XmlEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlToken" title="Text.Xml.XmlToken">XmlToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlEncoder" title="Text.Xml.XmlEncoder">XmlEncoder</a></code></li><li><a class="syn-name" href="#v-xml-encoder">xml-encoder</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-encoder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlEncoder" title="Text.Xml.XmlEncoder">XmlEncoder</a></code></li><li><a class="syn-name" href="#v-xml-encoder-close">xml-encoder-close</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-encoder-close</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlEncoder" title="Text.Xml.XmlEncoder">XmlEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-xml-encoder-indent">xml-encoder-indent</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-encoder-indent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlEncoder" title="Text.Xml.XmlEncoder">XmlEncoder</a></code></li><li><a class="syn-name" href="#v-xml-error-message">xml-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-xml-escape-attribute">xml-escape-attribute</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-escape-attribute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-xml-escape-text">xml-escape-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-escape-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-xml-name">xml-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlName" title="Text.Xml.XmlName">XmlName</a></code></li><li><a class="syn-name" href="#v-xml-name-local">xml-name-local</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-name-local</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlName" title="Text.Xml.XmlName">XmlName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-xml-name-space">xml-name-space</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-name-space</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlName" title="Text.Xml.XmlName">XmlName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-xml-next">xml-next</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-next</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlDecoder" title="Text.Xml.XmlDecoder">XmlDecoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlToken" title="Text.Xml.XmlToken">XmlToken</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlDecoder" title="Text.Xml.XmlDecoder">XmlDecoder</a>))</code></li><li><a class="syn-name" href="#v-xml-next-raw">xml-next-raw</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-next-raw</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlDecoder" title="Text.Xml.XmlDecoder">XmlDecoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlToken" title="Text.Xml.XmlToken">XmlToken</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlDecoder" title="Text.Xml.XmlDecoder">XmlDecoder</a>))</code></li><li><a class="syn-name" href="#v-xml-nodes">xml-nodes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-nodes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a></code></li><li><a class="syn-name" href="#v-xml-render">xml-render</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-render</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlEncoder" title="Text.Xml.XmlEncoder">XmlEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-xml-text">xml-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-xml-tokens">xml-tokens</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xml-tokens</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlToken" title="Text.Xml.XmlToken">XmlToken</a>)</code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-XmlAttr">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">XmlAttr</strong></code><span class="doc-sig-links"><a href="#t-XmlAttr" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L59" title="View source at line 59">Source</a></span></div>
<div class="doc-body">

Name, then value.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-XmlAttr"><code><strong class="tok-constructor">XmlAttr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlName" title="Text.Xml.XmlName">XmlName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlAttr" title="Text.Xml.XmlAttr">XmlAttr</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-XmlDecoder">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">XmlDecoder</strong></code><span class="doc-sig-links"><a href="#t-XmlDecoder" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L128" title="View source at line 128">Source</a></span></div>
<div class="doc-body">

Fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-XmlDecoder"><code><strong class="tok-constructor">XmlDecoder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-constructor">Frame</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlName" title="Text.Xml.XmlName">XmlName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlDecoder" title="Text.Xml.XmlDecoder">XmlDecoder</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-XmlEncoder">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">XmlEncoder</strong></code><span class="doc-sig-links"><a href="#t-XmlEncoder" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L679" title="View source at line 679">Source</a></span></div>
<div class="doc-body">

Fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-XmlEncoder"><code><strong class="tok-constructor">XmlEncoder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlName" title="Text.Xml.XmlName">XmlName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlEncoder" title="Text.Xml.XmlEncoder">XmlEncoder</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-XmlError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">XmlError</strong></code><span class="doc-sig-links"><a href="#t-XmlError" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L89" title="View source at line 89">Source</a></span></div>
<div class="doc-body">

Why decoding or encoding failed.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-XmlSyntaxError"><code><strong class="tok-constructor">XmlSyntaxError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a></code></li><li id="k-XmlUnsupportedVersion"><code><strong class="tok-constructor">XmlUnsupportedVersion</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a></code></li><li id="k-XmlUnsupportedEncoding"><code><strong class="tok-constructor">XmlUnsupportedEncoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a></code></li><li id="k-XmlDepthLimit"><code><strong class="tok-constructor">XmlDepthLimit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a></code></li><li id="k-XmlEncodeError"><code><strong class="tok-constructor">XmlEncodeError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-XmlName">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">XmlName</strong></code><span class="doc-sig-links"><a href="#t-XmlName" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L42" title="View source at line 42">Source</a></span></div>
<div class="doc-body">

Namespace, then local part.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-XmlName"><code><strong class="tok-constructor">XmlName</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlName" title="Text.Xml.XmlName">XmlName</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-XmlNode">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">XmlNode</strong></code><span class="doc-sig-links"><a href="#t-XmlNode" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L918" title="View source at line 918">Source</a></span></div>
<div class="doc-body">

A node in a sibling spine: each node holds the rest of its spine, and an element also holds the spine of
its children.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-XmlElementNode"><code><strong class="tok-constructor">XmlElementNode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlName" title="Text.Xml.XmlName">XmlName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlAttr" title="Text.Xml.XmlAttr">XmlAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a></code></li><li id="k-XmlTextNode"><code><strong class="tok-constructor">XmlTextNode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a></code></li><li id="k-XmlCommentNode"><code><strong class="tok-constructor">XmlCommentNode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a></code></li><li id="k-XmlProcInstNode"><code><strong class="tok-constructor">XmlProcInstNode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a></code></li><li id="k-XmlDirectiveNode"><code><strong class="tok-constructor">XmlDirectiveNode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a></code></li><li id="k-XmlNodesEnd"><code><strong class="tok-constructor">XmlNodesEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-XmlToken">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">XmlToken</strong></code><span class="doc-sig-links"><a href="#t-XmlToken" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

One unit of an XML document.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-XmlStartElement"><code><strong class="tok-constructor">XmlStartElement</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlName" title="Text.Xml.XmlName">XmlName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlAttr" title="Text.Xml.XmlAttr">XmlAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlToken" title="Text.Xml.XmlToken">XmlToken</a></code></li><li id="k-XmlEndElement"><code><strong class="tok-constructor">XmlEndElement</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlName" title="Text.Xml.XmlName">XmlName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlToken" title="Text.Xml.XmlToken">XmlToken</a></code></li><li id="k-XmlCharData"><code><strong class="tok-constructor">XmlCharData</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlToken" title="Text.Xml.XmlToken">XmlToken</a></code></li><li id="k-XmlComment"><code><strong class="tok-constructor">XmlComment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlToken" title="Text.Xml.XmlToken">XmlToken</a></code></li><li id="k-XmlProcInst"><code><strong class="tok-constructor">XmlProcInst</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlToken" title="Text.Xml.XmlToken">XmlToken</a></code></li><li id="k-XmlDirective"><code><strong class="tok-constructor">XmlDirective</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlToken" title="Text.Xml.XmlToken">XmlToken</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-xml-attr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-attr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlName" title="Text.Xml.XmlName">XmlName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlAttr" title="Text.Xml.XmlAttr">XmlAttr</a></code><span class="doc-sig-links"><a href="#v-xml-attr" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

Make an attribute.

</div>
</article>

<article class="doc-entry" id="v-xml-attr-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-attr-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlAttr" title="Text.Xml.XmlAttr">XmlAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlName" title="Text.Xml.XmlName">XmlName</a></code><span class="doc-sig-links"><a href="#v-xml-attr-name" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">

An attribute's name.

</div>
</article>

<article class="doc-entry" id="v-xml-attr-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-attr-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlAttr" title="Text.Xml.XmlAttr">XmlAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-xml-attr-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

An attribute's value.

</div>
</article>

<article class="doc-entry" id="v-xml-attribute">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-attribute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-xml-attribute" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L1008" title="View source at line 1008">Source</a></span></div>
<div class="doc-body">

The value of an element's attribute with a local name and no namespace.

</div>
</article>

<article class="doc-entry" id="v-xml-child-elements">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-child-elements</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a></code><span class="doc-sig-links"><a href="#v-xml-child-elements" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L994" title="View source at line 994">Source</a></span></div>
<div class="doc-body">

The child elements of an element with a local name (any namespace), in order.

</div>
</article>

<article class="doc-entry" id="v-xml-decoder">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-decoder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlDecoder" title="Text.Xml.XmlDecoder">XmlDecoder</a></code><span class="doc-sig-links"><a href="#v-xml-decoder" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L131" title="View source at line 131">Source</a></span></div>
<div class="doc-body">

A decoder at the start of a document, without a depth limit.

</div>
</article>

<article class="doc-entry" id="v-xml-decoder-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-decoder-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlDecoder" title="Text.Xml.XmlDecoder">XmlDecoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-xml-decoder-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L139" title="View source at line 139">Source</a></span></div>
<div class="doc-body">

The byte offset of the next unread input.

</div>
</article>

<article class="doc-entry" id="v-xml-decoder-with-depth-limit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-decoder-with-depth-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlDecoder" title="Text.Xml.XmlDecoder">XmlDecoder</a></code><span class="doc-sig-links"><a href="#v-xml-decoder-with-depth-limit" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L135" title="View source at line 135">Source</a></span></div>
<div class="doc-body">

A decoder that rejects elements nested deeper than the limit.

</div>
</article>

<article class="doc-entry" id="v-xml-document">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-document</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a></code><span class="doc-sig-links"><a href="#v-xml-document" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L938" title="View source at line 938">Source</a></span></div>
<div class="doc-body">

Parse a document into its top-level spine (the declaration, comments, the root element and any other nodes).

</div>
</article>

<article class="doc-entry" id="v-xml-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlEncoder" title="Text.Xml.XmlEncoder">XmlEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlToken" title="Text.Xml.XmlToken">XmlToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-xml-encode" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L690" title="View source at line 690">Source</a></span></div>
<div class="doc-body">

Encode a token sequence and check that every element is closed.

</div>
</article>

<article class="doc-entry" id="v-xml-encode-token">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-encode-token</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlEncoder" title="Text.Xml.XmlEncoder">XmlEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlToken" title="Text.Xml.XmlToken">XmlToken</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlEncoder" title="Text.Xml.XmlEncoder">XmlEncoder</a></code><span class="doc-sig-links"><a href="#v-xml-encode-token" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L703" title="View source at line 703">Source</a></span></div>
<div class="doc-body">

Write one token.

</div>
</article>

<article class="doc-entry" id="v-xml-encoder">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-encoder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlEncoder" title="Text.Xml.XmlEncoder">XmlEncoder</a></code><span class="doc-sig-links"><a href="#v-xml-encoder" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L682" title="View source at line 682">Source</a></span></div>
<div class="doc-body">

An encoder that writes tokens without added white space.

</div>
</article>

<article class="doc-entry" id="v-xml-encoder-close">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-encoder-close</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlEncoder" title="Text.Xml.XmlEncoder">XmlEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-xml-encoder-close" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L696" title="View source at line 696">Source</a></span></div>
<div class="doc-body">

Finish: the bytes written, or an error naming the innermost unclosed tag.

</div>
</article>

<article class="doc-entry" id="v-xml-encoder-indent">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-encoder-indent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlEncoder" title="Text.Xml.XmlEncoder">XmlEncoder</a></code><span class="doc-sig-links"><a href="#v-xml-encoder-indent" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L686" title="View source at line 686">Source</a></span></div>
<div class="doc-body">

An encoder that starts each element on a new line with the prefix and one indent per level.

</div>
</article>

<article class="doc-entry" id="v-xml-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-xml-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L102" title="View source at line 102">Source</a></span></div>
<div class="doc-body">

Render an error.

</div>
</article>

<article class="doc-entry" id="v-xml-escape-attribute">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-escape-attribute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-xml-escape-attribute" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L876" title="View source at line 876">Source</a></span></div>
<div class="doc-body">

Escape an attribute value or other quoted text (line feeds become references).

</div>
</article>

<article class="doc-entry" id="v-xml-escape-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-escape-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-xml-escape-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L872" title="View source at line 872">Source</a></span></div>
<div class="doc-body">

Escape character data (line feeds stay as they are).

</div>
</article>

<article class="doc-entry" id="v-xml-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlName" title="Text.Xml.XmlName">XmlName</a></code><span class="doc-sig-links"><a href="#v-xml-name" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

Make a name from a namespace and a local part.

</div>
</article>

<article class="doc-entry" id="v-xml-name-local">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-name-local</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlName" title="Text.Xml.XmlName">XmlName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-xml-name-local" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">

The local part.

</div>
</article>

<article class="doc-entry" id="v-xml-name-space">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-name-space</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlName" title="Text.Xml.XmlName">XmlName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-xml-name-space" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L49" title="View source at line 49">Source</a></span></div>
<div class="doc-body">

The namespace part.

</div>
</article>

<article class="doc-entry" id="v-xml-next">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-next</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlDecoder" title="Text.Xml.XmlDecoder">XmlDecoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlToken" title="Text.Xml.XmlToken">XmlToken</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlDecoder" title="Text.Xml.XmlDecoder">XmlDecoder</a>))</code><span class="doc-sig-links"><a href="#v-xml-next" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L170" title="View source at line 170">Source</a></span></div>
<div class="doc-body">

The next token with namespaces translated and end tags matched; None at the end of the document.

</div>
</article>

<article class="doc-entry" id="v-xml-next-raw">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-next-raw</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlDecoder" title="Text.Xml.XmlDecoder">XmlDecoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlToken" title="Text.Xml.XmlToken">XmlToken</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlDecoder" title="Text.Xml.XmlDecoder">XmlDecoder</a>))</code><span class="doc-sig-links"><a href="#v-xml-next-raw" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L155" title="View source at line 155">Source</a></span></div>
<div class="doc-body">

The next raw token: names keep their prefixes and end tags are not matched against start tags.

</div>
</article>

<article class="doc-entry" id="v-xml-nodes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-nodes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a></code><span class="doc-sig-links"><a href="#v-xml-nodes" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L980" title="View source at line 980">Source</a></span></div>
<div class="doc-body">

The nodes of a spine as a list, in order.

</div>
</article>

<article class="doc-entry" id="v-xml-render">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-render</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlEncoder" title="Text.Xml.XmlEncoder">XmlEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-xml-render" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L1028" title="View source at line 1028">Source</a></span></div>
<div class="doc-body">

Encode a spine back to XML with an encoder. Names carry decoded namespace URLs, so qualified attributes
(including namespace declarations, whose namespace is "xmlns") are written with generated prefixes.

</div>
</article>

<article class="doc-entry" id="v-xml-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlNode" title="Text.Xml.XmlNode">XmlNode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-xml-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L1020" title="View source at line 1020">Source</a></span></div>
<div class="doc-body">

The character data of a node and its descendants, in document order.

</div>
</article>

<article class="doc-entry" id="v-xml-tokens">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xml-tokens</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlError" title="Text.Xml.XmlError">XmlError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Xml/#t-XmlToken" title="Text.Xml.XmlToken">XmlToken</a>)</code><span class="doc-sig-links"><a href="#v-xml-tokens" title="Permalink">#</a><a href="/packages/stdlib/Text.Xml/source/#L143" title="View source at line 143">Source</a></span></div>
<div class="doc-body">

Decode every token of a document.

</div>
</article>
