# Text.Xml

Decode XML into namespace-aware tokens or element trees and encode tokens with escaping and indentation.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Xml`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Xml.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Text.Xml](Text.Xml.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-586d6c41747472"></a>

## `XmlAttr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">XmlAttr</span></code></pre>

Name, then value.

[Source](/reference-sources/Text.Xml.cinch) · line 59

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlAttr</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlAttr</span>))</code></pre>

<a id="type-586d6c4465636f646572"></a>

## `XmlDecoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">XmlDecoder</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Text.Xml.cinch) · line 128

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlDecoder</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">Frame</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlName</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlDecoder</span>))))))</code></pre>

<a id="type-586d6c456e636f646572"></a>

## `XmlEncoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">XmlEncoder</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Text.Xml.cinch) · line 679

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlEncoder</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlEncoder</span>)))))))))))</code></pre>

<a id="type-586d6c4572726f72"></a>

## `XmlError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">XmlError</span></code></pre>

Why decoding or encoding failed.

[Source](/reference-sources/Text.Xml.cinch) · line 89

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlSyntaxError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlUnsupportedVersion</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlUnsupportedEncoding</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlDepthLimit</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlEncodeError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlError</span>)</code></pre>

<a id="type-586d6c4e616d65"></a>

## `XmlName`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">XmlName</span></code></pre>

Namespace, then local part.

[Source](/reference-sources/Text.Xml.cinch) · line 42

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlName</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlName</span>))</code></pre>

<a id="type-586d6c4e6f6465"></a>

## `XmlNode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">XmlNode</span></code></pre>

A node in a sibling spine: each node holds the rest of its spine, and an element also holds the spine of
its children.

[Source](/reference-sources/Text.Xml.cinch) · line 918

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlElementNode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlAttr</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span>))))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlTextNode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlCommentNode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlProcInstNode</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span>)))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlDirectiveNode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlNodesEnd</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span></code></pre>

<a id="type-586d6c546f6b656e"></a>

## `XmlToken`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">XmlToken</span></code></pre>

One unit of an XML document.

[Source](/reference-sources/Text.Xml.cinch) · line 74

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlStartElement</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlAttr</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlToken</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlEndElement</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlName</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlToken</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlCharData</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlToken</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlComment</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlToken</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlProcInst</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlToken</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">XmlDirective</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlToken</span>)</code></pre>

# Values

<a id="value-786d6c2d61747472"></a>

## `xml-attr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-attr</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlAttr</span>))</code></pre>

Make an attribute.

[Source](/reference-sources/Text.Xml.cinch) · line 62

<a id="value-786d6c2d617474722d6e616d65"></a>

## `xml-attr-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-attr-name</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlAttr</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlName</span>)</code></pre>

An attribute's name.

[Source](/reference-sources/Text.Xml.cinch) · line 66

<a id="value-786d6c2d617474722d76616c7565"></a>

## `xml-attr-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-attr-value</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlAttr</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

An attribute's value.

[Source](/reference-sources/Text.Xml.cinch) · line 70

<a id="value-786d6c2d617474726962757465"></a>

## `xml-attribute`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-attribute</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

The value of an element's attribute with a local name and no namespace.

[Source](/reference-sources/Text.Xml.cinch) · line 1008

<a id="value-786d6c2d6368696c642d656c656d656e7473"></a>

## `xml-child-elements`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-child-elements</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span>)))</code></pre>

The child elements of an element with a local name (any namespace), in order.

[Source](/reference-sources/Text.Xml.cinch) · line 994

<a id="value-786d6c2d6465636f646572"></a>

## `xml-decoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-decoder</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlDecoder</span>)</code></pre>

A decoder at the start of a document, without a depth limit.

[Source](/reference-sources/Text.Xml.cinch) · line 131

<a id="value-786d6c2d6465636f6465722d6f6666736574"></a>

## `xml-decoder-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-decoder-offset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlDecoder</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The byte offset of the next unread input.

[Source](/reference-sources/Text.Xml.cinch) · line 139

<a id="value-786d6c2d6465636f6465722d776974682d64657074682d6c696d6974"></a>

## `xml-decoder-with-depth-limit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-decoder-with-depth-limit</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlDecoder</span>))</code></pre>

A decoder that rejects elements nested deeper than the limit.

[Source](/reference-sources/Text.Xml.cinch) · line 135

<a id="value-786d6c2d646f63756d656e74"></a>

## `xml-document`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-document</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span>))</code></pre>

Parse a document into its top-level spine (the declaration, comments, the root element and any other nodes).

[Source](/reference-sources/Text.Xml.cinch) · line 938

<a id="value-786d6c2d656e636f6465"></a>

## `xml-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-encode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlEncoder</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlToken</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Encode a token sequence and check that every element is closed.

[Source](/reference-sources/Text.Xml.cinch) · line 690

<a id="value-786d6c2d656e636f64652d746f6b656e"></a>

## `xml-encode-token`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-encode-token</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlEncoder</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlToken</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlEncoder</span>)))</code></pre>

Write one token.

[Source](/reference-sources/Text.Xml.cinch) · line 703

<a id="value-786d6c2d656e636f646572"></a>

## `xml-encoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-encoder</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlEncoder</span></code></pre>

An encoder that writes tokens without added white space.

[Source](/reference-sources/Text.Xml.cinch) · line 682

<a id="value-786d6c2d656e636f6465722d636c6f7365"></a>

## `xml-encoder-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-encoder-close</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlEncoder</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Finish: the bytes written, or an error naming the innermost unclosed tag.

[Source](/reference-sources/Text.Xml.cinch) · line 696

<a id="value-786d6c2d656e636f6465722d696e64656e74"></a>

## `xml-encoder-indent`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-encoder-indent</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlEncoder</span>))</code></pre>

An encoder that starts each element on a new line with the prefix and one indent per level.

[Source](/reference-sources/Text.Xml.cinch) · line 686

<a id="value-786d6c2d6572726f722d6d657373616765"></a>

## `xml-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an error.

[Source](/reference-sources/Text.Xml.cinch) · line 102

<a id="value-786d6c2d6573636170652d617474726962757465"></a>

## `xml-escape-attribute`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-escape-attribute</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Escape an attribute value or other quoted text (line feeds become references).

[Source](/reference-sources/Text.Xml.cinch) · line 876

<a id="value-786d6c2d6573636170652d74657874"></a>

## `xml-escape-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-escape-text</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Escape character data (line feeds stay as they are).

[Source](/reference-sources/Text.Xml.cinch) · line 872

<a id="value-786d6c2d6e616d65"></a>

## `xml-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlName</span>))</code></pre>

Make a name from a namespace and a local part.

[Source](/reference-sources/Text.Xml.cinch) · line 45

<a id="value-786d6c2d6e616d652d6c6f63616c"></a>

## `xml-name-local`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-name-local</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlName</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The local part.

[Source](/reference-sources/Text.Xml.cinch) · line 53

<a id="value-786d6c2d6e616d652d7370616365"></a>

## `xml-name-space`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-name-space</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlName</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The namespace part.

[Source](/reference-sources/Text.Xml.cinch) · line 49

<a id="value-786d6c2d6e657874"></a>

## `xml-next`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-next</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlDecoder</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlToken</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlDecoder</span>))))</code></pre>

The next token with namespaces translated and end tags matched; None at the end of the document.

[Source](/reference-sources/Text.Xml.cinch) · line 170

<a id="value-786d6c2d6e6578742d726177"></a>

## `xml-next-raw`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-next-raw</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlDecoder</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlToken</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlDecoder</span>))))</code></pre>

The next raw token: names keep their prefixes and end tags are not matched against start tags.

[Source](/reference-sources/Text.Xml.cinch) · line 155

<a id="value-786d6c2d6e6f646573"></a>

## `xml-nodes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-nodes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span>))</code></pre>

The nodes of a spine as a list, in order.

[Source](/reference-sources/Text.Xml.cinch) · line 980

<a id="value-786d6c2d72656e646572"></a>

## `xml-render`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-render</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlEncoder</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Encode a spine back to XML with an encoder. Names carry decoded namespace URLs, so qualified attributes
(including namespace declarations, whose namespace is "xmlns") are written with generated prefixes.

[Source](/reference-sources/Text.Xml.cinch) · line 1028

<a id="value-786d6c2d74657874"></a>

## `xml-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-text</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlNode</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The character data of a node and its descendants, in document order.

[Source](/reference-sources/Text.Xml.cinch) · line 1020

<a id="value-786d6c2d746f6b656e73"></a>

## `xml-tokens`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xml-tokens</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Xml</span>.<span class="tok-constructor">XmlToken</span>)))</code></pre>

Decode every token of a document.

[Source](/reference-sources/Text.Xml.cinch) · line 143

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Xml.cinch`. Source SHA-256: `3b6e28f6684a6a7fcccc4c6c3bdf996162acd9906568b30df6a59478a9da046d`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
