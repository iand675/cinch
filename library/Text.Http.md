# Text.Http

Pure HTTP/1.1 wire parsing/rendering, chunked transfer coding, status texts, canonical headers, forms and Go 1.22 ServeMux pattern routing.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Http</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Http/source/">View source</a><a href="/packages/search/?q=%2BText.Http">Search module</a><a href="/reference-sources/Text.Http.cinch" download="Text.Http.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Cookie](Text.Cookie.md), [Text.Show](Text.Show.md), [Text.TextProto](Text.TextProto.md), [Text.Url](Text.Url.md).

<details class="doc-notes"><summary>Module notes</summary>

Pure HTTP/1.1 protocol framing, status codes, canonical headers, request/response models and ServeMux routing.
Provides standard HTTP status codes and reason phrases, canonical MIME header operations built on
Text.TextProto, RFC 9112 HTTP/1.1 wire parsing and rendering with Content-Length and chunked transfer
encoding, query and URL-encoded form helpers built on Text.Url, cookie helpers built on Text.Cookie,
and a method/host/wildcard ServeMux router with specificity precedence, conflict detection, trailing-slash
redirects and 405 Allow generation.

</details>

<details class="doc-synopsis"><summary>Synopsis (187 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-HttpHeader">HttpHeader</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HttpHeader</strong></code></li><li><a class="syn-name" href="#t-HttpProtoError">HttpProtoError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HttpProtoError</strong></code></li><li><a class="syn-name" href="#t-HttpVersion">HttpVersion</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HttpVersion</strong></code></li><li><a class="syn-name" href="#t-HttpWireRequest">HttpWireRequest</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HttpWireRequest</strong></code></li><li><a class="syn-name" href="#t-HttpWireResponse">HttpWireResponse</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HttpWireResponse</strong></code></li><li><a class="syn-name" href="#t-MuxMatch">MuxMatch</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MuxMatch</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#t-MuxPattern">MuxPattern</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MuxPattern</strong></code></li><li><a class="syn-name" href="#t-MuxSegment">MuxSegment</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MuxSegment</strong></code></li><li><a class="syn-name" href="#t-PatternRelation">PatternRelation</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PatternRelation</strong></code></li><li><a class="syn-name" href="#t-ServeMux">ServeMux</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ServeMux</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-http-canonical-header-key">http-canonical-header-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-canonical-header-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-http-chunk-decode">http-chunk-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-chunk-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-http-chunk-encode">http-chunk-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-chunk-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-http-clean-path">http-clean-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-clean-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-http-header-add">http-header-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-header-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code></li><li><a class="syn-name" href="#v-http-header-clone">http-header-clone</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-header-clone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code></li><li><a class="syn-name" href="#v-http-header-del">http-header-del</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-header-del</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code></li><li><a class="syn-name" href="#v-http-header-empty">http-header-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-header-empty</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code></li><li><a class="syn-name" href="#v-http-header-entries">http-header-entries</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-header-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-http-header-from-mime">http-header-from-mime</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-header-from-mime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code></li><li><a class="syn-name" href="#v-http-header-from-pairs">http-header-from-pairs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-header-from-pairs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code></li><li><a class="syn-name" href="#v-http-header-get">http-header-get</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-header-get</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-http-header-keys">http-header-keys</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-header-keys</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-http-header-render">http-header-render</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-header-render</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-http-header-set">http-header-set</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-header-set</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code></li><li><a class="syn-name" href="#v-http-header-to-mime">http-header-to-mime</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-header-to-mime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a></code></li><li><a class="syn-name" href="#v-http-header-to-pairs">http-header-to-pairs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-header-to-pairs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-http-header-values">http-header-values</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-header-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-http-new-request">http-new-request</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-new-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code></li><li><a class="syn-name" href="#v-http-parse-request">http-parse-request</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-parse-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-http-parse-request-limit">http-parse-request-limit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-parse-request-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-http-parse-response">http-parse-response</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-parse-response</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-http-parse-response-for">http-parse-response-for</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-parse-response-for</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-http-parse-response-limit">http-parse-response-limit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-parse-response-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-http-parse-version">http-parse-version</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-parse-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpVersion" title="Text.Http.HttpVersion">HttpVersion</a></code></li><li><a class="syn-name" href="#v-http-proto-error-message">http-proto-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-proto-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-http-render-request">http-render-request</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-render-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-http-render-request-chunked">http-render-request-chunked</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-render-request-chunked</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-http-render-response">http-render-response</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-render-response</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-http-render-response-chunked">http-render-response-chunked</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-render-response-chunked</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-http-render-response-for">http-render-response-for</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-render-response-for</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-http-response-bytes">http-response-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-response-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code></li><li><a class="syn-name" href="#v-http-response-error">http-response-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-response-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code></li><li><a class="syn-name" href="#v-http-response-new">http-response-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-response-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code></li><li><a class="syn-name" href="#v-http-response-redirect">http-response-redirect</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-response-redirect</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code></li><li><a class="syn-name" href="#v-http-response-text">http-response-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-response-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code></li><li><a class="syn-name" href="#v-http-set-cookie">http-set-cookie</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-set-cookie</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code></li><li><a class="syn-name" href="#v-http-status-text">http-status-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-status-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-http-version-major">http-version-major</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-version-major</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpVersion" title="Text.Http.HttpVersion">HttpVersion</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-http-version-minor">http-version-minor</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-version-minor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpVersion" title="Text.Http.HttpVersion">HttpVersion</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-http-version-string">http-version-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-version-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpVersion" title="Text.Http.HttpVersion">HttpVersion</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-http10">http10</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http10</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpVersion" title="Text.Http.HttpVersion">HttpVersion</a></code></li><li><a class="syn-name" href="#v-http11">http11</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http11</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpVersion" title="Text.Http.HttpVersion">HttpVersion</a></code></li><li><a class="syn-name" href="#v-method-connect">method-connect</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">method-connect</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-method-delete">method-delete</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">method-delete</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-method-get">method-get</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">method-get</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-method-head">method-head</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">method-head</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-method-options">method-options</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">method-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-method-patch">method-patch</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">method-patch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-method-post">method-post</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">method-post</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-method-put">method-put</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">method-put</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-method-trace">method-trace</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">method-trace</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-mux-allow-header">mux-allow-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mux-allow-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-mux-empty">mux-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mux-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-ServeMux" title="Text.Http.ServeMux">ServeMux</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-mux-handle">mux-handle</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mux-handle</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-ServeMux" title="Text.Http.ServeMux">ServeMux</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-ServeMux" title="Text.Http.ServeMux">ServeMux</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-mux-handler">mux-handler</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mux-handler</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-ServeMux" title="Text.Http.ServeMux">ServeMux</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxMatch" title="Text.Http.MuxMatch">MuxMatch</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-mux-match">mux-match</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mux-match</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-ServeMux" title="Text.Http.ServeMux">ServeMux</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxMatch" title="Text.Http.MuxMatch">MuxMatch</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-mux-parse-pattern">mux-parse-pattern</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mux-parse-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxPattern" title="Text.Http.MuxPattern">MuxPattern</a></code></li><li><a class="syn-name" href="#v-mux-pattern-compare">mux-pattern-compare</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mux-pattern-compare</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxPattern" title="Text.Http.MuxPattern">MuxPattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxPattern" title="Text.Http.MuxPattern">MuxPattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-PatternRelation" title="Text.Http.PatternRelation">PatternRelation</a></code></li><li><a class="syn-name" href="#v-mux-pattern-host">mux-pattern-host</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mux-pattern-host</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxPattern" title="Text.Http.MuxPattern">MuxPattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-mux-pattern-method">mux-pattern-method</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mux-pattern-method</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxPattern" title="Text.Http.MuxPattern">MuxPattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-mux-pattern-string">mux-pattern-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mux-pattern-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxPattern" title="Text.Http.MuxPattern">MuxPattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-req-add-cookie">req-add-cookie</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-add-cookie</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code></li><li><a class="syn-name" href="#v-req-add-header">req-add-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-add-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code></li><li><a class="syn-name" href="#v-req-body">req-body</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-body</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-req-close">req-close</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-close</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-req-content-length">req-content-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-content-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-req-cookie">req-cookie</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-cookie</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a></code></li><li><a class="syn-name" href="#v-req-cookies">req-cookies</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-cookies</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a></code></li><li><a class="syn-name" href="#v-req-form-value">req-form-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-form-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-req-header">req-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-req-header-values">req-header-values</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-header-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-req-headers">req-headers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code></li><li><a class="syn-name" href="#v-req-host">req-host</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-host</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-req-method">req-method</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-method</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-req-parse-form">req-parse-form</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-parse-form</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code></li><li><a class="syn-name" href="#v-req-path">req-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-req-path-value">req-path-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-path-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-req-path-values">req-path-values</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-path-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-req-post-form">req-post-form</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-post-form</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code></li><li><a class="syn-name" href="#v-req-proto">req-proto</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-proto</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-req-proto-major">req-proto-major</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-proto-major</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-req-proto-minor">req-proto-minor</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-proto-minor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-req-query">req-query</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-query</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code></li><li><a class="syn-name" href="#v-req-query-param">req-query-param</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-query-param</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-req-raw-path">req-raw-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-raw-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-req-raw-query">req-raw-query</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-raw-query</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-req-remote-addr">req-remote-addr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-remote-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-req-request-uri">req-request-uri</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-request-uri</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-req-trailers">req-trailers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-trailers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code></li><li><a class="syn-name" href="#v-req-with-body">req-with-body</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-with-body</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code></li><li><a class="syn-name" href="#v-req-with-header">req-with-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-with-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code></li><li><a class="syn-name" href="#v-req-with-headers">req-with-headers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-with-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code></li><li><a class="syn-name" href="#v-req-with-host">req-with-host</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-with-host</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code></li><li><a class="syn-name" href="#v-req-with-path-values">req-with-path-values</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-with-path-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code></li><li><a class="syn-name" href="#v-req-with-proto">req-with-proto</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-with-proto</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpVersion" title="Text.Http.HttpVersion">HttpVersion</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code></li><li><a class="syn-name" href="#v-req-with-remote-addr">req-with-remote-addr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">req-with-remote-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code></li><li><a class="syn-name" href="#v-resp-add-cookie">resp-add-cookie</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resp-add-cookie</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code></li><li><a class="syn-name" href="#v-resp-add-header">resp-add-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resp-add-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code></li><li><a class="syn-name" href="#v-resp-body">resp-body</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resp-body</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-resp-cookies">resp-cookies</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resp-cookies</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a></code></li><li><a class="syn-name" href="#v-resp-header">resp-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resp-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-resp-header-values">resp-header-values</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resp-header-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-resp-headers">resp-headers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resp-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code></li><li><a class="syn-name" href="#v-resp-proto-major">resp-proto-major</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resp-proto-major</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-resp-proto-minor">resp-proto-minor</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resp-proto-minor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-resp-reason">resp-reason</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resp-reason</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-resp-status">resp-status</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resp-status</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-resp-trailers">resp-trailers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resp-trailers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code></li><li><a class="syn-name" href="#v-resp-with-body">resp-with-body</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resp-with-body</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code></li><li><a class="syn-name" href="#v-resp-with-header">resp-with-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resp-with-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code></li><li><a class="syn-name" href="#v-resp-with-headers">resp-with-headers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resp-with-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code></li><li><a class="syn-name" href="#v-resp-with-reason">resp-with-reason</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resp-with-reason</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code></li><li><a class="syn-name" href="#v-resp-with-status">resp-with-status</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resp-with-status</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code></li><li><a class="syn-name" href="#v-resp-with-trailers">resp-with-trailers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resp-with-trailers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code></li><li><a class="syn-name" href="#v-status-accepted">status-accepted</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-accepted</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-already-reported">status-already-reported</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-already-reported</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-bad-gateway">status-bad-gateway</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-bad-gateway</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-bad-request">status-bad-request</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-bad-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-conflict">status-conflict</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-conflict</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-continue">status-continue</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-continue</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-created">status-created</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-created</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-early-hints">status-early-hints</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-early-hints</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-expectation-failed">status-expectation-failed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-expectation-failed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-failed-dependency">status-failed-dependency</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-failed-dependency</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-forbidden">status-forbidden</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-forbidden</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-found">status-found</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-found</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-gateway-timeout">status-gateway-timeout</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-gateway-timeout</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-gone">status-gone</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-gone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-http-version-not-supported">status-http-version-not-supported</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-http-version-not-supported</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-im-used">status-im-used</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-im-used</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-insufficient-storage">status-insufficient-storage</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-insufficient-storage</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-internal-server-error">status-internal-server-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-internal-server-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-length-required">status-length-required</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-length-required</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-locked">status-locked</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-locked</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-loop-detected">status-loop-detected</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-loop-detected</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-method-not-allowed">status-method-not-allowed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-method-not-allowed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-misdirected-request">status-misdirected-request</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-misdirected-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-moved-permanently">status-moved-permanently</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-moved-permanently</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-multi-status">status-multi-status</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-multi-status</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-multiple-choices">status-multiple-choices</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-multiple-choices</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-network-authentication-required">status-network-authentication-required</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-network-authentication-required</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-no-content">status-no-content</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-no-content</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-non-authoritative-info">status-non-authoritative-info</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-non-authoritative-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-not-acceptable">status-not-acceptable</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-not-acceptable</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-not-extended">status-not-extended</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-not-extended</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-not-found">status-not-found</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-not-found</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-not-implemented">status-not-implemented</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-not-implemented</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-not-modified">status-not-modified</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-not-modified</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-ok">status-ok</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-ok</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-partial-content">status-partial-content</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-partial-content</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-payment-required">status-payment-required</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-payment-required</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-permanent-redirect">status-permanent-redirect</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-permanent-redirect</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-precondition-failed">status-precondition-failed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-precondition-failed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-precondition-required">status-precondition-required</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-precondition-required</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-processing">status-processing</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-processing</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-proxy-auth-required">status-proxy-auth-required</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-proxy-auth-required</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-request-entity-too-large">status-request-entity-too-large</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-request-entity-too-large</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-request-header-fields-too-large">status-request-header-fields-too-large</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-request-header-fields-too-large</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-request-timeout">status-request-timeout</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-request-timeout</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-request-uri-too-long">status-request-uri-too-long</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-request-uri-too-long</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-requested-range-not-satisfiable">status-requested-range-not-satisfiable</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-requested-range-not-satisfiable</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-reset-content">status-reset-content</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-reset-content</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-see-other">status-see-other</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-see-other</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-service-unavailable">status-service-unavailable</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-service-unavailable</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-switching-protocols">status-switching-protocols</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-switching-protocols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-teapot">status-teapot</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-teapot</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-temporary-redirect">status-temporary-redirect</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-temporary-redirect</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-too-early">status-too-early</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-too-early</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-too-many-requests">status-too-many-requests</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-too-many-requests</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-unauthorized">status-unauthorized</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-unauthorized</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-unavailable-for-legal-reasons">status-unavailable-for-legal-reasons</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-unavailable-for-legal-reasons</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-unprocessable-entity">status-unprocessable-entity</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-unprocessable-entity</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-unsupported-media-type">status-unsupported-media-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-unsupported-media-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-upgrade-required">status-upgrade-required</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-upgrade-required</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-use-proxy">status-use-proxy</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-use-proxy</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-status-variant-also-negotiates">status-variant-also-negotiates</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">status-variant-also-negotiates</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-HttpHeader">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HttpHeader</strong></code><span class="doc-sig-links"><a href="#t-HttpHeader" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L671" title="View source at line 671">Source</a></span></div>
<div class="doc-body">

Canonical HTTP header multimap backed by Text.TextProto.MimeHeader.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HttpHeaderMap"><code><strong class="tok-constructor">HttpHeaderMap</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-HttpProtoError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HttpProtoError</strong></code><span class="doc-sig-links"><a href="#t-HttpProtoError" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L432" title="View source at line 432">Source</a></span></div>
<div class="doc-body">

A named HTTP/1.1 framing, parsing or route pattern error.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HttpProtoIncomplete"><code><strong class="tok-constructor">HttpProtoIncomplete</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a></code></li><li id="k-HttpProtoBadRequestLine"><code><strong class="tok-constructor">HttpProtoBadRequestLine</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a></code></li><li id="k-HttpProtoBadStatusLine"><code><strong class="tok-constructor">HttpProtoBadStatusLine</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a></code></li><li id="k-HttpProtoBadHeader"><code><strong class="tok-constructor">HttpProtoBadHeader</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a></code></li><li id="k-HttpProtoInvalidVersion"><code><strong class="tok-constructor">HttpProtoInvalidVersion</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a></code></li><li id="k-HttpProtoInvalidContentLength"><code><strong class="tok-constructor">HttpProtoInvalidContentLength</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a></code></li><li id="k-HttpProtoConflictingContentLength"><code><strong class="tok-constructor">HttpProtoConflictingContentLength</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a></code></li><li id="k-HttpProtoUnsupportedTransferEncoding"><code><strong class="tok-constructor">HttpProtoUnsupportedTransferEncoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a></code></li><li id="k-HttpProtoBadChunk"><code><strong class="tok-constructor">HttpProtoBadChunk</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a></code></li><li id="k-HttpProtoTruncatedBody"><code><strong class="tok-constructor">HttpProtoTruncatedBody</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a></code></li><li id="k-HttpProtoTooLarge"><code><strong class="tok-constructor">HttpProtoTooLarge</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a></code></li><li id="k-HttpProtoBadPattern"><code><strong class="tok-constructor">HttpProtoBadPattern</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a></code></li><li id="k-HttpProtoPatternConflict"><code><strong class="tok-constructor">HttpProtoPatternConflict</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-HttpVersion">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HttpVersion</strong></code><span class="doc-sig-links"><a href="#t-HttpVersion" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L792" title="View source at line 792">Source</a></span></div>
<div class="doc-body">

Major and minor version numbers.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HttpVersion"><code><strong class="tok-constructor">HttpVersion</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpVersion" title="Text.Http.HttpVersion">HttpVersion</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-HttpWireRequest">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HttpWireRequest</strong></code><span class="doc-sig-links"><a href="#t-HttpWireRequest" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L842" title="View source at line 842">Source</a></span></div>
<div class="doc-body">

Request fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HttpWireRequest"><code><strong class="tok-constructor">HttpWireRequest</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-HttpWireResponse">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HttpWireResponse</strong></code><span class="doc-sig-links"><a href="#t-HttpWireResponse" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1130" title="View source at line 1130">Source</a></span></div>
<div class="doc-body">

Response fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HttpWireResponse"><code><strong class="tok-constructor">HttpWireResponse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MuxMatch">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MuxMatch</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-MuxMatch" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1991" title="View source at line 1991">Source</a></span></div>
<div class="doc-body">

The outcome of matching an HTTP request against a ServeMux.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MuxMatched"><code><strong class="tok-constructor">MuxMatched</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxMatch" title="Text.Http.MuxMatch">MuxMatch</a> <span class="tok-variable">a</span></code></li><li id="k-MuxRedirect"><code><strong class="tok-constructor">MuxRedirect</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxMatch" title="Text.Http.MuxMatch">MuxMatch</a> <span class="tok-variable">a</span></code></li><li id="k-MuxMethodNotAllowed"><code><strong class="tok-constructor">MuxMethodNotAllowed</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxMatch" title="Text.Http.MuxMatch">MuxMatch</a> <span class="tok-variable">a</span></code></li><li id="k-MuxNotFound"><code><strong class="tok-constructor">MuxNotFound</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxMatch" title="Text.Http.MuxMatch">MuxMatch</a> <span class="tok-variable">a</span></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MuxPattern">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MuxPattern</strong></code><span class="doc-sig-links"><a href="#t-MuxPattern" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1693" title="View source at line 1693">Source</a></span></div>
<div class="doc-body">

Pattern fields: original string, method, host, segments.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MuxPattern"><code><strong class="tok-constructor">MuxPattern</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxSegment" title="Text.Http.MuxSegment">MuxSegment</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxPattern" title="Text.Http.MuxPattern">MuxPattern</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MuxSegment">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MuxSegment</strong></code><span class="doc-sig-links"><a href="#t-MuxSegment" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1680" title="View source at line 1680">Source</a></span></div>
<div class="doc-body">

One parsed segment in a ServeMux route pattern.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SegLiteral"><code><strong class="tok-constructor">SegLiteral</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxSegment" title="Text.Http.MuxSegment">MuxSegment</a></code></li><li id="k-SegWild"><code><strong class="tok-constructor">SegWild</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxSegment" title="Text.Http.MuxSegment">MuxSegment</a></code></li><li id="k-SegMulti"><code><strong class="tok-constructor">SegMulti</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxSegment" title="Text.Http.MuxSegment">MuxSegment</a></code></li><li id="k-SegExactEnd"><code><strong class="tok-constructor">SegExactEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxSegment" title="Text.Http.MuxSegment">MuxSegment</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PatternRelation">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PatternRelation</strong></code><span class="doc-sig-links"><a href="#t-PatternRelation" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1839" title="View source at line 1839">Source</a></span></div>
<div class="doc-body">

Specificity relationship between two route patterns.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PatDisjoint"><code><strong class="tok-constructor">PatDisjoint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-PatternRelation" title="Text.Http.PatternRelation">PatternRelation</a></code></li><li id="k-PatEquivalent"><code><strong class="tok-constructor">PatEquivalent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-PatternRelation" title="Text.Http.PatternRelation">PatternRelation</a></code></li><li id="k-PatMoreSpecific"><code><strong class="tok-constructor">PatMoreSpecific</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-PatternRelation" title="Text.Http.PatternRelation">PatternRelation</a></code></li><li id="k-PatLessSpecific"><code><strong class="tok-constructor">PatLessSpecific</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-PatternRelation" title="Text.Http.PatternRelation">PatternRelation</a></code></li><li id="k-PatOverlaps"><code><strong class="tok-constructor">PatOverlaps</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-PatternRelation" title="Text.Http.PatternRelation">PatternRelation</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ServeMux">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ServeMux</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-ServeMux" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1965" title="View source at line 1965">Source</a></span></div>
<div class="doc-body">

Registered entries.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ServeMux"><code><strong class="tok-constructor">ServeMux</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-constructor">MuxEntry</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-ServeMux" title="Text.Http.ServeMux">ServeMux</a> <span class="tok-variable">a</span></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-http-canonical-header-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-canonical-header-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-http-canonical-header-key" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L676" title="View source at line 676">Source</a></span></div>
<div class="doc-body">

Canonicalize an HTTP header field name using Text.TextProto.

</div>
</article>

<article class="doc-entry" id="v-http-chunk-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-chunk-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-http-chunk-decode" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1279" title="View source at line 1279">Source</a></span></div>
<div class="doc-body">

Decode an HTTP/1.1 chunked body and optional trailing headers starting at offset start.
Returns decoded body bytes, trailers, and the next byte offset after the terminating blank line.

</div>
</article>

<article class="doc-entry" id="v-http-chunk-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-chunk-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-http-chunk-encode" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1258" title="View source at line 1258">Source</a></span></div>
<div class="doc-body">

Encode raw body bytes using HTTP/1.1 chunked transfer encoding with a given maximum chunk size.

</div>
</article>

<article class="doc-entry" id="v-http-clean-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-clean-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-http-clean-path" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1637" title="View source at line 1637">Source</a></span></div>
<div class="doc-body">

Return the canonical URL path for p, eliminating "." and ".." elements and repeated slashes,
preserving a trailing slash when present.

</div>
</article>

<article class="doc-entry" id="v-http-header-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-header-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code><span class="doc-sig-links"><a href="#v-http-header-add" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L684" title="View source at line 684">Source</a></span></div>
<div class="doc-body">

Append a value for a header field, canonicalizing the field name.

</div>
</article>

<article class="doc-entry" id="v-http-header-clone">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-header-clone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code><span class="doc-sig-links"><a href="#v-http-header-clone" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L734" title="View source at line 734">Source</a></span></div>
<div class="doc-body">

Return an independent copy of a header map.

</div>
</article>

<article class="doc-entry" id="v-http-header-del">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-header-del</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code><span class="doc-sig-links"><a href="#v-http-header-del" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L712" title="View source at line 712">Source</a></span></div>
<div class="doc-body">

Delete a header field and all its values.

</div>
</article>

<article class="doc-entry" id="v-http-header-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-header-empty</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code><span class="doc-sig-links"><a href="#v-http-header-empty" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L680" title="View source at line 680">Source</a></span></div>
<div class="doc-body">

An empty HTTP header map.

</div>
</article>

<article class="doc-entry" id="v-http-header-entries">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-header-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-http-header-entries" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L722" title="View source at line 722">Source</a></span></div>
<div class="doc-body">

All header fields with their ordered value lists.

</div>
</article>

<article class="doc-entry" id="v-http-header-from-mime">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-header-from-mime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code><span class="doc-sig-links"><a href="#v-http-header-from-mime" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L776" title="View source at line 776">Source</a></span></div>
<div class="doc-body">

Wrap a Text.TextProto.MimeHeader as an HttpHeader.

</div>
</article>

<article class="doc-entry" id="v-http-header-from-pairs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-header-from-pairs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code><span class="doc-sig-links"><a href="#v-http-header-from-pairs" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L738" title="View source at line 738">Source</a></span></div>
<div class="doc-body">

Construct a header map from name/value pairs in order.

</div>
</article>

<article class="doc-entry" id="v-http-header-get">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-header-get</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-http-header-get" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L694" title="View source at line 694">Source</a></span></div>
<div class="doc-body">

Retrieve the first value for a header field, if present.

</div>
</article>

<article class="doc-entry" id="v-http-header-keys">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-header-keys</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-http-header-keys" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L717" title="View source at line 717">Source</a></span></div>
<div class="doc-body">

All canonical header keys in insertion order.

</div>
</article>

<article class="doc-entry" id="v-http-header-render">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-header-render</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-http-header-render" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L766" title="View source at line 766">Source</a></span></div>
<div class="doc-body">

Render HTTP headers as wire lines followed by the terminating CRLF blank line.

</div>
</article>

<article class="doc-entry" id="v-http-header-set">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-header-set</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code><span class="doc-sig-links"><a href="#v-http-header-set" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L689" title="View source at line 689">Source</a></span></div>
<div class="doc-body">

Replace all values for a header field with a single value.

</div>
</article>

<article class="doc-entry" id="v-http-header-to-mime">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-header-to-mime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.TextProto/#t-MimeHeader" title="Text.TextProto.MimeHeader">MimeHeader</a></code><span class="doc-sig-links"><a href="#v-http-header-to-mime" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L771" title="View source at line 771">Source</a></span></div>
<div class="doc-body">

Unwrap the underlying Text.TextProto.MimeHeader.

</div>
</article>

<article class="doc-entry" id="v-http-header-to-pairs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-header-to-pairs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-http-header-to-pairs" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L749" title="View source at line 749">Source</a></span></div>
<div class="doc-body">

Flatten a header map into ordered (CanonicalKey, Value) pairs.

</div>
</article>

<article class="doc-entry" id="v-http-header-values">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-header-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-http-header-values" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L701" title="View source at line 701">Source</a></span></div>
<div class="doc-body">

Retrieve all values for a header field in insertion order.

</div>
</article>

<article class="doc-entry" id="v-http-new-request">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-new-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code><span class="doc-sig-links"><a href="#v-http-new-request" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L845" title="View source at line 845">Source</a></span></div>
<div class="doc-body">

Construct a fresh HTTP/1.1 request from method, target URI/path, and body bytes.

</div>
</article>

<article class="doc-entry" id="v-http-parse-request">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-parse-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-http-parse-request" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1412" title="View source at line 1412">Source</a></span></div>
<div class="doc-body">

Parse one HTTP/1.1 request from wire bytes, returning the request and exact bytes consumed.

</div>
</article>

<article class="doc-entry" id="v-http-parse-request-limit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-parse-request-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-http-parse-request-limit" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1416" title="View source at line 1416">Source</a></span></div>
<div class="doc-body">

Parse one HTTP/1.1 request with explicit header and body byte limits (0 disables a limit).

</div>
</article>

<article class="doc-entry" id="v-http-parse-response">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-parse-response</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-http-parse-response" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1514" title="View source at line 1514">Source</a></span></div>
<div class="doc-body">

Parse an HTTP/1.1 response assuming a GET request.

</div>
</article>

<article class="doc-entry" id="v-http-parse-response-for">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-parse-response-for</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-http-parse-response-for" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1518" title="View source at line 1518">Source</a></span></div>
<div class="doc-body">

Parse an HTTP/1.1 response for a given request method (handling HEAD and bodyless statuses).

</div>
</article>

<article class="doc-entry" id="v-http-parse-response-limit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-parse-response-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-http-parse-response-limit" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1522" title="View source at line 1522">Source</a></span></div>
<div class="doc-body">

Parse an HTTP/1.1 response with explicit request method, header limit, and body limit.

</div>
</article>

<article class="doc-entry" id="v-http-parse-version">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-parse-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpVersion" title="Text.Http.HttpVersion">HttpVersion</a></code><span class="doc-sig-links"><a href="#v-http-parse-version" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L816" title="View source at line 816">Source</a></span></div>
<div class="doc-body">

Parse an HTTP version token "HTTP/<major>.<minor>".

</div>
</article>

<article class="doc-entry" id="v-http-proto-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-proto-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-http-proto-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L461" title="View source at line 461">Source</a></span></div>
<div class="doc-body">

Render a readable diagnostic for an HTTP protocol error.

</div>
</article>

<article class="doc-entry" id="v-http-render-request">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-render-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-http-render-request" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1482" title="View source at line 1482">Source</a></span></div>
<div class="doc-body">

Render an HTTP/1.1 request to wire bytes using Content-Length when a body is present.

</div>
</article>

<article class="doc-entry" id="v-http-render-request-chunked">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-render-request-chunked</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-http-render-request-chunked" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1496" title="View source at line 1496">Source</a></span></div>
<div class="doc-body">

Render an HTTP/1.1 request using Transfer-Encoding: chunked with the specified chunk size.

</div>
</article>

<article class="doc-entry" id="v-http-render-response">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-render-response</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-http-render-response" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1593" title="View source at line 1593">Source</a></span></div>
<div class="doc-body">

Render an HTTP/1.1 response to wire bytes with automatic status text and Content-Length.

</div>
</article>

<article class="doc-entry" id="v-http-render-response-chunked">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-render-response-chunked</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-http-render-response-chunked" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1618" title="View source at line 1618">Source</a></span></div>
<div class="doc-body">

Render an HTTP/1.1 response using Transfer-Encoding: chunked.

</div>
</article>

<article class="doc-entry" id="v-http-render-response-for">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-render-response-for</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-http-render-response-for" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1597" title="View source at line 1597">Source</a></span></div>
<div class="doc-body">

Render an HTTP/1.1 response for a specific request method (omitting wire body for HEAD).

</div>
</article>

<article class="doc-entry" id="v-http-response-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-response-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code><span class="doc-sig-links"><a href="#v-http-response-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1144" title="View source at line 1144">Source</a></span></div>
<div class="doc-body">

Build an HTTP/1.1 response with an explicit Content-Type and body bytes.

</div>
</article>

<article class="doc-entry" id="v-http-response-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-response-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code><span class="doc-sig-links"><a href="#v-http-response-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1150" title="View source at line 1150">Source</a></span></div>
<div class="doc-body">

Build a standard plain-text error response with X-Content-Type-Options: nosniff.

</div>
</article>

<article class="doc-entry" id="v-http-response-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-response-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code><span class="doc-sig-links"><a href="#v-http-response-new" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1133" title="View source at line 1133">Source</a></span></div>
<div class="doc-body">

Build an HTTP/1.1 response with status code, headers and body bytes.

</div>
</article>

<article class="doc-entry" id="v-http-response-redirect">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-response-redirect</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code><span class="doc-sig-links"><a href="#v-http-response-redirect" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1157" title="View source at line 1157">Source</a></span></div>
<div class="doc-body">

Build an HTTP redirect response with Location header.

</div>
</article>

<article class="doc-entry" id="v-http-response-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-response-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code><span class="doc-sig-links"><a href="#v-http-response-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1138" title="View source at line 1138">Source</a></span></div>
<div class="doc-body">

Build a UTF-8 plain text HTTP/1.1 response.

</div>
</article>

<article class="doc-entry" id="v-http-set-cookie">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-set-cookie</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code><span class="doc-sig-links"><a href="#v-http-set-cookie" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L780" title="View source at line 780">Source</a></span></div>
<div class="doc-body">

Append a Set-Cookie header for a Cookie value when valid to serialize.

</div>
</article>

<article class="doc-entry" id="v-http-status-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-status-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-http-status-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L325" title="View source at line 325">Source</a></span></div>
<div class="doc-body">

The standard reason phrase for an HTTP status code, or empty text if unknown.

</div>
</article>

<article class="doc-entry" id="v-http-version-major">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-version-major</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpVersion" title="Text.Http.HttpVersion">HttpVersion</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-http-version-major" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L803" title="View source at line 803">Source</a></span></div>
<div class="doc-body">

The major version number.

</div>
</article>

<article class="doc-entry" id="v-http-version-minor">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-version-minor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpVersion" title="Text.Http.HttpVersion">HttpVersion</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-http-version-minor" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L807" title="View source at line 807">Source</a></span></div>
<div class="doc-body">

The minor version number.

</div>
</article>

<article class="doc-entry" id="v-http-version-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-version-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpVersion" title="Text.Http.HttpVersion">HttpVersion</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-http-version-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L811" title="View source at line 811">Source</a></span></div>
<div class="doc-body">

Render an HTTP version token such as "HTTP/1.1".

</div>
</article>

<article class="doc-entry" id="v-http10">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http10</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpVersion" title="Text.Http.HttpVersion">HttpVersion</a></code><span class="doc-sig-links"><a href="#v-http10" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L795" title="View source at line 795">Source</a></span></div>
<div class="doc-body">

HTTP/1.0 protocol version.

</div>
</article>

<article class="doc-entry" id="v-http11">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http11</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpVersion" title="Text.Http.HttpVersion">HttpVersion</a></code><span class="doc-sig-links"><a href="#v-http11" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L799" title="View source at line 799">Source</a></span></div>
<div class="doc-body">

HTTP/1.1 protocol version.

</div>
</article>

<article class="doc-entry" id="v-method-connect">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">method-connect</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-method-connect" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L416" title="View source at line 416">Source</a></span></div>
<div class="doc-body">

The HTTP CONNECT method token.

</div>
</article>

<article class="doc-entry" id="v-method-delete">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">method-delete</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-method-delete" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L412" title="View source at line 412">Source</a></span></div>
<div class="doc-body">

The HTTP DELETE method token.

</div>
</article>

<article class="doc-entry" id="v-method-get">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">method-get</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-method-get" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L392" title="View source at line 392">Source</a></span></div>
<div class="doc-body">

The HTTP GET method token.

</div>
</article>

<article class="doc-entry" id="v-method-head">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">method-head</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-method-head" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L396" title="View source at line 396">Source</a></span></div>
<div class="doc-body">

The HTTP HEAD method token.

</div>
</article>

<article class="doc-entry" id="v-method-options">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">method-options</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-method-options" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L420" title="View source at line 420">Source</a></span></div>
<div class="doc-body">

The HTTP OPTIONS method token.

</div>
</article>

<article class="doc-entry" id="v-method-patch">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">method-patch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-method-patch" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L408" title="View source at line 408">Source</a></span></div>
<div class="doc-body">

The HTTP PATCH method token.

</div>
</article>

<article class="doc-entry" id="v-method-post">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">method-post</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-method-post" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L400" title="View source at line 400">Source</a></span></div>
<div class="doc-body">

The HTTP POST method token.

</div>
</article>

<article class="doc-entry" id="v-method-put">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">method-put</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-method-put" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L404" title="View source at line 404">Source</a></span></div>
<div class="doc-body">

The HTTP PUT method token.

</div>
</article>

<article class="doc-entry" id="v-method-trace">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">method-trace</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-method-trace" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L424" title="View source at line 424">Source</a></span></div>
<div class="doc-body">

The HTTP TRACE method token.

</div>
</article>

<article class="doc-entry" id="v-mux-allow-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mux-allow-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-mux-allow-header" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L2156" title="View source at line 2156">Source</a></span></div>
<div class="doc-body">

Format a list of allowed methods as a standard HTTP Allow header value.

</div>
</article>

<article class="doc-entry" id="v-mux-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mux-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-ServeMux" title="Text.Http.ServeMux">ServeMux</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-mux-empty" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1968" title="View source at line 1968">Source</a></span></div>
<div class="doc-body">

An empty ServeMux with no registered routes.

</div>
</article>

<article class="doc-entry" id="v-mux-handle">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mux-handle</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-ServeMux" title="Text.Http.ServeMux">ServeMux</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-ServeMux" title="Text.Http.ServeMux">ServeMux</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-mux-handle" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1972" title="View source at line 1972">Source</a></span></div>
<div class="doc-body">

Register a pattern and handler, rejecting invalid syntax or conflicting routes.

</div>
</article>

<article class="doc-entry" id="v-mux-handler">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mux-handler</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-ServeMux" title="Text.Http.ServeMux">ServeMux</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxMatch" title="Text.Http.MuxMatch">MuxMatch</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-mux-handler" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L2002" title="View source at line 2002">Source</a></span></div>
<div class="doc-body">

Match an HTTP wire request against a ServeMux.

</div>
</article>

<article class="doc-entry" id="v-mux-match">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mux-match</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-ServeMux" title="Text.Http.ServeMux">ServeMux</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxMatch" title="Text.Http.MuxMatch">MuxMatch</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-mux-match" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L2007" title="View source at line 2007">Source</a></span></div>
<div class="doc-body">

Match a method, host and path (optionally containing "?query") against a ServeMux.

</div>
</article>

<article class="doc-entry" id="v-mux-parse-pattern">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mux-parse-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpProtoError" title="Text.Http.HttpProtoError">HttpProtoError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxPattern" title="Text.Http.MuxPattern">MuxPattern</a></code><span class="doc-sig-links"><a href="#v-mux-parse-pattern" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1712" title="View source at line 1712">Source</a></span></div>
<div class="doc-body">

Parse and validate a route pattern of the form "[METHOD ][HOST]/[PATH]".

</div>
</article>

<article class="doc-entry" id="v-mux-pattern-compare">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mux-pattern-compare</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxPattern" title="Text.Http.MuxPattern">MuxPattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxPattern" title="Text.Http.MuxPattern">MuxPattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-PatternRelation" title="Text.Http.PatternRelation">PatternRelation</a></code><span class="doc-sig-links"><a href="#v-mux-pattern-compare" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1880" title="View source at line 1880">Source</a></span></div>
<div class="doc-body">

Compare two route patterns for precedence and conflicts.
When patterns would otherwise overlap or invert on path/method, a pattern with an explicit host
takes precedence over a host-less pattern.

</div>
</article>

<article class="doc-entry" id="v-mux-pattern-host">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mux-pattern-host</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxPattern" title="Text.Http.MuxPattern">MuxPattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-mux-pattern-host" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1704" title="View source at line 1704">Source</a></span></div>
<div class="doc-body">

Host constraint of a parsed route pattern (empty if unconstrained).

</div>
</article>

<article class="doc-entry" id="v-mux-pattern-method">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mux-pattern-method</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxPattern" title="Text.Http.MuxPattern">MuxPattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-mux-pattern-method" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1700" title="View source at line 1700">Source</a></span></div>
<div class="doc-body">

Method constraint of a parsed route pattern (empty if unconstrained).

</div>
</article>

<article class="doc-entry" id="v-mux-pattern-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mux-pattern-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-MuxPattern" title="Text.Http.MuxPattern">MuxPattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-mux-pattern-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1696" title="View source at line 1696">Source</a></span></div>
<div class="doc-body">

Original text of a parsed route pattern.

</div>
</article>

<article class="doc-entry" id="v-req-add-cookie">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-add-cookie</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code><span class="doc-sig-links"><a href="#v-req-add-cookie" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1029" title="View source at line 1029">Source</a></span></div>
<div class="doc-body">

Append a cookie name=value pair to the request's Cookie header.

</div>
</article>

<article class="doc-entry" id="v-req-add-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-add-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code><span class="doc-sig-links"><a href="#v-req-add-header" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L987" title="View source at line 987">Source</a></span></div>
<div class="doc-body">

Return a request with an additional header value appended.

</div>
</article>

<article class="doc-entry" id="v-req-body">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-body</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-req-body" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L946" title="View source at line 946">Source</a></span></div>
<div class="doc-body">

The decoded body bytes of a request.

</div>
</article>

<article class="doc-entry" id="v-req-close">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-close</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-req-close" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L977" title="View source at line 977">Source</a></span></div>
<div class="doc-body">

Whether the connection should be closed after replying to this request.

</div>
</article>

<article class="doc-entry" id="v-req-content-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-content-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-req-content-length" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L973" title="View source at line 973">Source</a></span></div>
<div class="doc-body">

The declared content length (-1 when chunked or unspecified).

</div>
</article>

<article class="doc-entry" id="v-req-cookie">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-cookie</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a></code><span class="doc-sig-links"><a href="#v-req-cookie" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1053" title="View source at line 1053">Source</a></span></div>
<div class="doc-body">

Find the first request cookie matching name.

</div>
</article>

<article class="doc-entry" id="v-req-cookies">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-cookies</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a></code><span class="doc-sig-links"><a href="#v-req-cookies" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1037" title="View source at line 1037">Source</a></span></div>
<div class="doc-body">

Parse all cookies carried in the request's Cookie headers via Text.Cookie.

</div>
</article>

<article class="doc-entry" id="v-req-form-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-form-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-req-form-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1115" title="View source at line 1115">Source</a></span></div>
<div class="doc-body">

Look up the first form value (body parameters before query parameters).

</div>
</article>

<article class="doc-entry" id="v-req-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-req-header" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L938" title="View source at line 938">Source</a></span></div>
<div class="doc-body">

Retrieve the first value for a request header.

</div>
</article>

<article class="doc-entry" id="v-req-header-values">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-header-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-req-header-values" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L942" title="View source at line 942">Source</a></span></div>
<div class="doc-body">

Retrieve all values for a request header.

</div>
</article>

<article class="doc-entry" id="v-req-headers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code><span class="doc-sig-links"><a href="#v-req-headers" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L934" title="View source at line 934">Source</a></span></div>
<div class="doc-body">

The canonical header map of a request.

</div>
</article>

<article class="doc-entry" id="v-req-host">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-host</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-req-host" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L906" title="View source at line 906">Source</a></span></div>
<div class="doc-body">

The host of a request.

</div>
</article>

<article class="doc-entry" id="v-req-method">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-method</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-req-method" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L898" title="View source at line 898">Source</a></span></div>
<div class="doc-body">

The HTTP method of a request.

</div>
</article>

<article class="doc-entry" id="v-req-parse-form">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-parse-form</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code><span class="doc-sig-links"><a href="#v-req-parse-form" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1094" title="View source at line 1094">Source</a></span></div>
<div class="doc-body">

Parse both URL-encoded body parameters (taking priority) and URL query parameters.

</div>
</article>

<article class="doc-entry" id="v-req-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-req-path" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L910" title="View source at line 910">Source</a></span></div>
<div class="doc-body">

The decoded path of a request.

</div>
</article>

<article class="doc-entry" id="v-req-path-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-path-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-req-path-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L958" title="View source at line 958">Source</a></span></div>
<div class="doc-body">

Look up a single matched route path parameter by name.

</div>
</article>

<article class="doc-entry" id="v-req-path-values">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-path-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-req-path-values" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L954" title="View source at line 954">Source</a></span></div>
<div class="doc-body">

All matched route path parameter bindings.

</div>
</article>

<article class="doc-entry" id="v-req-post-form">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-post-form</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code><span class="doc-sig-links"><a href="#v-req-post-form" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1078" title="View source at line 1078">Source</a></span></div>
<div class="doc-body">

Parse application/x-www-form-urlencoded parameters from a POST, PUT or PATCH request body.

</div>
</article>

<article class="doc-entry" id="v-req-proto">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-proto</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-req-proto" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L930" title="View source at line 930">Source</a></span></div>
<div class="doc-body">

The HTTP protocol version string (such as "HTTP/1.1") of a request.

</div>
</article>

<article class="doc-entry" id="v-req-proto-major">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-proto-major</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-req-proto-major" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L922" title="View source at line 922">Source</a></span></div>
<div class="doc-body">

The HTTP major version number of a request.

</div>
</article>

<article class="doc-entry" id="v-req-proto-minor">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-proto-minor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-req-proto-minor" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L926" title="View source at line 926">Source</a></span></div>
<div class="doc-body">

The HTTP minor version number of a request.

</div>
</article>

<article class="doc-entry" id="v-req-query">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-query</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code><span class="doc-sig-links"><a href="#v-req-query" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1063" title="View source at line 1063">Source</a></span></div>
<div class="doc-body">

Parse the request's raw query string into URL values via Text.Url.

</div>
</article>

<article class="doc-entry" id="v-req-query-param">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-query-param</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-req-query-param" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1069" title="View source at line 1069">Source</a></span></div>
<div class="doc-body">

Look up the first value of a query parameter by name.

</div>
</article>

<article class="doc-entry" id="v-req-raw-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-raw-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-req-raw-path" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L914" title="View source at line 914">Source</a></span></div>
<div class="doc-body">

The raw escaped path of a request.

</div>
</article>

<article class="doc-entry" id="v-req-raw-query">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-raw-query</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-req-raw-query" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L918" title="View source at line 918">Source</a></span></div>
<div class="doc-body">

The raw query string (without leading '?') of a request.

</div>
</article>

<article class="doc-entry" id="v-req-remote-addr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-remote-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-req-remote-addr" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L969" title="View source at line 969">Source</a></span></div>
<div class="doc-body">

The remote address string recorded on a request.

</div>
</article>

<article class="doc-entry" id="v-req-request-uri">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-request-uri</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-req-request-uri" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L902" title="View source at line 902">Source</a></span></div>
<div class="doc-body">

The raw request-target URI of a request.

</div>
</article>

<article class="doc-entry" id="v-req-trailers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-trailers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code><span class="doc-sig-links"><a href="#v-req-trailers" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L950" title="View source at line 950">Source</a></span></div>
<div class="doc-body">

The trailing headers decoded after a chunked request body.

</div>
</article>

<article class="doc-entry" id="v-req-with-body">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-with-body</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code><span class="doc-sig-links"><a href="#v-req-with-body" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1005" title="View source at line 1005">Source</a></span></div>
<div class="doc-body">

Return a request with its body bytes replaced and content length updated.

</div>
</article>

<article class="doc-entry" id="v-req-with-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-with-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code><span class="doc-sig-links"><a href="#v-req-with-header" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L981" title="View source at line 981">Source</a></span></div>
<div class="doc-body">

Return a request with a header replaced.

</div>
</article>

<article class="doc-entry" id="v-req-with-headers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-with-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code><span class="doc-sig-links"><a href="#v-req-with-headers" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L993" title="View source at line 993">Source</a></span></div>
<div class="doc-body">

Return a request with its entire header map replaced.

</div>
</article>

<article class="doc-entry" id="v-req-with-host">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-with-host</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code><span class="doc-sig-links"><a href="#v-req-with-host" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L999" title="View source at line 999">Source</a></span></div>
<div class="doc-body">

Return a request with its Host field updated.

</div>
</article>

<article class="doc-entry" id="v-req-with-path-values">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-with-path-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code><span class="doc-sig-links"><a href="#v-req-with-path-values" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1011" title="View source at line 1011">Source</a></span></div>
<div class="doc-body">

Return a request with matched route path parameters attached.

</div>
</article>

<article class="doc-entry" id="v-req-with-proto">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-with-proto</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpVersion" title="Text.Http.HttpVersion">HttpVersion</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code><span class="doc-sig-links"><a href="#v-req-with-proto" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1023" title="View source at line 1023">Source</a></span></div>
<div class="doc-body">

Return a request with its protocol version updated.

</div>
</article>

<article class="doc-entry" id="v-req-with-remote-addr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">req-with-remote-addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireRequest" title="Text.Http.HttpWireRequest">HttpWireRequest</a></code><span class="doc-sig-links"><a href="#v-req-with-remote-addr" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1017" title="View source at line 1017">Source</a></span></div>
<div class="doc-body">

Return a request with its remote address updated.

</div>
</article>

<article class="doc-entry" id="v-resp-add-cookie">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resp-add-cookie</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code><span class="doc-sig-links"><a href="#v-resp-add-cookie" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1249" title="View source at line 1249">Source</a></span></div>
<div class="doc-body">

Append a Set-Cookie header to a response.

</div>
</article>

<article class="doc-entry" id="v-resp-add-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resp-add-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code><span class="doc-sig-links"><a href="#v-resp-add-header" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1229" title="View source at line 1229">Source</a></span></div>
<div class="doc-body">

Return a response with an additional header value appended.

</div>
</article>

<article class="doc-entry" id="v-resp-body">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resp-body</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-resp-body" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1194" title="View source at line 1194">Source</a></span></div>
<div class="doc-body">

The decoded body bytes of a response.

</div>
</article>

<article class="doc-entry" id="v-resp-cookies">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resp-cookies</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a></code><span class="doc-sig-links"><a href="#v-resp-cookies" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1202" title="View source at line 1202">Source</a></span></div>
<div class="doc-body">

Parse all valid Set-Cookie headers from a response via Text.Cookie.

</div>
</article>

<article class="doc-entry" id="v-resp-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resp-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-resp-header" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1186" title="View source at line 1186">Source</a></span></div>
<div class="doc-body">

Look up the first value of a response header.

</div>
</article>

<article class="doc-entry" id="v-resp-header-values">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resp-header-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-resp-header-values" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1190" title="View source at line 1190">Source</a></span></div>
<div class="doc-body">

Look up all values of a response header.

</div>
</article>

<article class="doc-entry" id="v-resp-headers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resp-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code><span class="doc-sig-links"><a href="#v-resp-headers" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1182" title="View source at line 1182">Source</a></span></div>
<div class="doc-body">

The canonical header map of a response.

</div>
</article>

<article class="doc-entry" id="v-resp-proto-major">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resp-proto-major</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-resp-proto-major" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1174" title="View source at line 1174">Source</a></span></div>
<div class="doc-body">

The HTTP major version of a response.

</div>
</article>

<article class="doc-entry" id="v-resp-proto-minor">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resp-proto-minor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-resp-proto-minor" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1178" title="View source at line 1178">Source</a></span></div>
<div class="doc-body">

The HTTP minor version of a response.

</div>
</article>

<article class="doc-entry" id="v-resp-reason">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resp-reason</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-resp-reason" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1170" title="View source at line 1170">Source</a></span></div>
<div class="doc-body">

The reason phrase of a response.

</div>
</article>

<article class="doc-entry" id="v-resp-status">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resp-status</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-resp-status" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1166" title="View source at line 1166">Source</a></span></div>
<div class="doc-body">

The HTTP status code of a response.

</div>
</article>

<article class="doc-entry" id="v-resp-trailers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resp-trailers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a></code><span class="doc-sig-links"><a href="#v-resp-trailers" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1198" title="View source at line 1198">Source</a></span></div>
<div class="doc-body">

The trailing headers of a response.

</div>
</article>

<article class="doc-entry" id="v-resp-with-body">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resp-with-body</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code><span class="doc-sig-links"><a href="#v-resp-with-body" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1239" title="View source at line 1239">Source</a></span></div>
<div class="doc-body">

Return a response with its body bytes replaced.

</div>
</article>

<article class="doc-entry" id="v-resp-with-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resp-with-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code><span class="doc-sig-links"><a href="#v-resp-with-header" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1224" title="View source at line 1224">Source</a></span></div>
<div class="doc-body">

Return a response with a header replaced.

</div>
</article>

<article class="doc-entry" id="v-resp-with-headers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resp-with-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code><span class="doc-sig-links"><a href="#v-resp-with-headers" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1234" title="View source at line 1234">Source</a></span></div>
<div class="doc-body">

Return a response with its entire header map replaced.

</div>
</article>

<article class="doc-entry" id="v-resp-with-reason">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resp-with-reason</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code><span class="doc-sig-links"><a href="#v-resp-with-reason" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1219" title="View source at line 1219">Source</a></span></div>
<div class="doc-body">

Return a response with an explicit reason phrase.

</div>
</article>

<article class="doc-entry" id="v-resp-with-status">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resp-with-status</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code><span class="doc-sig-links"><a href="#v-resp-with-status" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1214" title="View source at line 1214">Source</a></span></div>
<div class="doc-body">

Return a response with an updated status code and matching default reason phrase.

</div>
</article>

<article class="doc-entry" id="v-resp-with-trailers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resp-with-trailers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpHeader" title="Text.Http.HttpHeader">HttpHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Http/#t-HttpWireResponse" title="Text.Http.HttpWireResponse">HttpWireResponse</a></code><span class="doc-sig-links"><a href="#v-resp-with-trailers" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L1244" title="View source at line 1244">Source</a></span></div>
<div class="doc-body">

Return a response with its trailers replaced.

</div>
</article>

<article class="doc-entry" id="v-status-accepted">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-accepted</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-accepted" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L101" title="View source at line 101">Source</a></span></div>
<div class="doc-body">

HTTP 202 Accepted.

</div>
</article>

<article class="doc-entry" id="v-status-already-reported">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-already-reported</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-already-reported" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L125" title="View source at line 125">Source</a></span></div>
<div class="doc-body">

HTTP 208 Already Reported.

</div>
</article>

<article class="doc-entry" id="v-status-bad-gateway">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-bad-gateway</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-bad-gateway" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L289" title="View source at line 289">Source</a></span></div>
<div class="doc-body">

HTTP 502 Bad Gateway.

</div>
</article>

<article class="doc-entry" id="v-status-bad-request">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-bad-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-bad-request" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L165" title="View source at line 165">Source</a></span></div>
<div class="doc-body">

HTTP 400 Bad Request.

</div>
</article>

<article class="doc-entry" id="v-status-conflict">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-conflict</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-conflict" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L201" title="View source at line 201">Source</a></span></div>
<div class="doc-body">

HTTP 409 Conflict.

</div>
</article>

<article class="doc-entry" id="v-status-continue">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-continue</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-continue" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L77" title="View source at line 77">Source</a></span></div>
<div class="doc-body">

HTTP 100 Continue.

</div>
</article>

<article class="doc-entry" id="v-status-created">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-created</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-created" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L97" title="View source at line 97">Source</a></span></div>
<div class="doc-body">

HTTP 201 Created.

</div>
</article>

<article class="doc-entry" id="v-status-early-hints">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-early-hints</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-early-hints" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L89" title="View source at line 89">Source</a></span></div>
<div class="doc-body">

HTTP 103 Early Hints.

</div>
</article>

<article class="doc-entry" id="v-status-expectation-failed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-expectation-failed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-expectation-failed" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L233" title="View source at line 233">Source</a></span></div>
<div class="doc-body">

HTTP 417 Expectation Failed.

</div>
</article>

<article class="doc-entry" id="v-status-failed-dependency">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-failed-dependency</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-failed-dependency" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L253" title="View source at line 253">Source</a></span></div>
<div class="doc-body">

HTTP 424 Failed Dependency.

</div>
</article>

<article class="doc-entry" id="v-status-forbidden">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-forbidden</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-forbidden" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L177" title="View source at line 177">Source</a></span></div>
<div class="doc-body">

HTTP 403 Forbidden.

</div>
</article>

<article class="doc-entry" id="v-status-found">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-found</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-found" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L141" title="View source at line 141">Source</a></span></div>
<div class="doc-body">

HTTP 302 Found.

</div>
</article>

<article class="doc-entry" id="v-status-gateway-timeout">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-gateway-timeout</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-gateway-timeout" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L297" title="View source at line 297">Source</a></span></div>
<div class="doc-body">

HTTP 504 Gateway Timeout.

</div>
</article>

<article class="doc-entry" id="v-status-gone">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-gone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-gone" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L205" title="View source at line 205">Source</a></span></div>
<div class="doc-body">

HTTP 410 Gone.

</div>
</article>

<article class="doc-entry" id="v-status-http-version-not-supported">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-http-version-not-supported</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-http-version-not-supported" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L301" title="View source at line 301">Source</a></span></div>
<div class="doc-body">

HTTP 505 HTTP Version Not Supported.

</div>
</article>

<article class="doc-entry" id="v-status-im-used">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-im-used</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-im-used" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L129" title="View source at line 129">Source</a></span></div>
<div class="doc-body">

HTTP 226 IM Used.

</div>
</article>

<article class="doc-entry" id="v-status-insufficient-storage">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-insufficient-storage</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-insufficient-storage" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L309" title="View source at line 309">Source</a></span></div>
<div class="doc-body">

HTTP 507 Insufficient Storage.

</div>
</article>

<article class="doc-entry" id="v-status-internal-server-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-internal-server-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-internal-server-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L281" title="View source at line 281">Source</a></span></div>
<div class="doc-body">

HTTP 500 Internal Server Error.

</div>
</article>

<article class="doc-entry" id="v-status-length-required">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-length-required</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-length-required" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L209" title="View source at line 209">Source</a></span></div>
<div class="doc-body">

HTTP 411 Length Required.

</div>
</article>

<article class="doc-entry" id="v-status-locked">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-locked</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-locked" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L249" title="View source at line 249">Source</a></span></div>
<div class="doc-body">

HTTP 423 Locked.

</div>
</article>

<article class="doc-entry" id="v-status-loop-detected">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-loop-detected</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-loop-detected" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L313" title="View source at line 313">Source</a></span></div>
<div class="doc-body">

HTTP 508 Loop Detected.

</div>
</article>

<article class="doc-entry" id="v-status-method-not-allowed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-method-not-allowed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-method-not-allowed" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L185" title="View source at line 185">Source</a></span></div>
<div class="doc-body">

HTTP 405 Method Not Allowed.

</div>
</article>

<article class="doc-entry" id="v-status-misdirected-request">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-misdirected-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-misdirected-request" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L241" title="View source at line 241">Source</a></span></div>
<div class="doc-body">

HTTP 421 Misdirected Request.

</div>
</article>

<article class="doc-entry" id="v-status-moved-permanently">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-moved-permanently</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-moved-permanently" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L137" title="View source at line 137">Source</a></span></div>
<div class="doc-body">

HTTP 301 Moved Permanently.

</div>
</article>

<article class="doc-entry" id="v-status-multi-status">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-multi-status</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-multi-status" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L121" title="View source at line 121">Source</a></span></div>
<div class="doc-body">

HTTP 207 Multi-Status.

</div>
</article>

<article class="doc-entry" id="v-status-multiple-choices">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-multiple-choices</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-multiple-choices" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L133" title="View source at line 133">Source</a></span></div>
<div class="doc-body">

HTTP 300 Multiple Choices.

</div>
</article>

<article class="doc-entry" id="v-status-network-authentication-required">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-network-authentication-required</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-network-authentication-required" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L321" title="View source at line 321">Source</a></span></div>
<div class="doc-body">

HTTP 511 Network Authentication Required.

</div>
</article>

<article class="doc-entry" id="v-status-no-content">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-no-content</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-no-content" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L109" title="View source at line 109">Source</a></span></div>
<div class="doc-body">

HTTP 204 No Content.

</div>
</article>

<article class="doc-entry" id="v-status-non-authoritative-info">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-non-authoritative-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-non-authoritative-info" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L105" title="View source at line 105">Source</a></span></div>
<div class="doc-body">

HTTP 203 Non-Authoritative Information.

</div>
</article>

<article class="doc-entry" id="v-status-not-acceptable">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-not-acceptable</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-not-acceptable" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L189" title="View source at line 189">Source</a></span></div>
<div class="doc-body">

HTTP 406 Not Acceptable.

</div>
</article>

<article class="doc-entry" id="v-status-not-extended">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-not-extended</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-not-extended" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L317" title="View source at line 317">Source</a></span></div>
<div class="doc-body">

HTTP 510 Not Extended.

</div>
</article>

<article class="doc-entry" id="v-status-not-found">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-not-found</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-not-found" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L181" title="View source at line 181">Source</a></span></div>
<div class="doc-body">

HTTP 404 Not Found.

</div>
</article>

<article class="doc-entry" id="v-status-not-implemented">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-not-implemented</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-not-implemented" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L285" title="View source at line 285">Source</a></span></div>
<div class="doc-body">

HTTP 501 Not Implemented.

</div>
</article>

<article class="doc-entry" id="v-status-not-modified">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-not-modified</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-not-modified" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L149" title="View source at line 149">Source</a></span></div>
<div class="doc-body">

HTTP 304 Not Modified.

</div>
</article>

<article class="doc-entry" id="v-status-ok">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-ok</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-ok" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L93" title="View source at line 93">Source</a></span></div>
<div class="doc-body">

HTTP 200 OK.

</div>
</article>

<article class="doc-entry" id="v-status-partial-content">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-partial-content</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-partial-content" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L117" title="View source at line 117">Source</a></span></div>
<div class="doc-body">

HTTP 206 Partial Content.

</div>
</article>

<article class="doc-entry" id="v-status-payment-required">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-payment-required</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-payment-required" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L173" title="View source at line 173">Source</a></span></div>
<div class="doc-body">

HTTP 402 Payment Required.

</div>
</article>

<article class="doc-entry" id="v-status-permanent-redirect">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-permanent-redirect</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-permanent-redirect" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L161" title="View source at line 161">Source</a></span></div>
<div class="doc-body">

HTTP 308 Permanent Redirect.

</div>
</article>

<article class="doc-entry" id="v-status-precondition-failed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-precondition-failed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-precondition-failed" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L213" title="View source at line 213">Source</a></span></div>
<div class="doc-body">

HTTP 412 Precondition Failed.

</div>
</article>

<article class="doc-entry" id="v-status-precondition-required">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-precondition-required</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-precondition-required" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L265" title="View source at line 265">Source</a></span></div>
<div class="doc-body">

HTTP 428 Precondition Required.

</div>
</article>

<article class="doc-entry" id="v-status-processing">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-processing</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-processing" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L85" title="View source at line 85">Source</a></span></div>
<div class="doc-body">

HTTP 102 Processing.

</div>
</article>

<article class="doc-entry" id="v-status-proxy-auth-required">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-proxy-auth-required</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-proxy-auth-required" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L193" title="View source at line 193">Source</a></span></div>
<div class="doc-body">

HTTP 407 Proxy Authentication Required.

</div>
</article>

<article class="doc-entry" id="v-status-request-entity-too-large">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-request-entity-too-large</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-request-entity-too-large" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L217" title="View source at line 217">Source</a></span></div>
<div class="doc-body">

HTTP 413 Request Entity Too Large.

</div>
</article>

<article class="doc-entry" id="v-status-request-header-fields-too-large">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-request-header-fields-too-large</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-request-header-fields-too-large" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L273" title="View source at line 273">Source</a></span></div>
<div class="doc-body">

HTTP 431 Request Header Fields Too Large.

</div>
</article>

<article class="doc-entry" id="v-status-request-timeout">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-request-timeout</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-request-timeout" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L197" title="View source at line 197">Source</a></span></div>
<div class="doc-body">

HTTP 408 Request Timeout.

</div>
</article>

<article class="doc-entry" id="v-status-request-uri-too-long">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-request-uri-too-long</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-request-uri-too-long" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L221" title="View source at line 221">Source</a></span></div>
<div class="doc-body">

HTTP 414 Request URI Too Long.

</div>
</article>

<article class="doc-entry" id="v-status-requested-range-not-satisfiable">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-requested-range-not-satisfiable</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-requested-range-not-satisfiable" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L229" title="View source at line 229">Source</a></span></div>
<div class="doc-body">

HTTP 416 Requested Range Not Satisfiable.

</div>
</article>

<article class="doc-entry" id="v-status-reset-content">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-reset-content</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-reset-content" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L113" title="View source at line 113">Source</a></span></div>
<div class="doc-body">

HTTP 205 Reset Content.

</div>
</article>

<article class="doc-entry" id="v-status-see-other">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-see-other</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-see-other" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L145" title="View source at line 145">Source</a></span></div>
<div class="doc-body">

HTTP 303 See Other.

</div>
</article>

<article class="doc-entry" id="v-status-service-unavailable">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-service-unavailable</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-service-unavailable" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L293" title="View source at line 293">Source</a></span></div>
<div class="doc-body">

HTTP 503 Service Unavailable.

</div>
</article>

<article class="doc-entry" id="v-status-switching-protocols">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-switching-protocols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-switching-protocols" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L81" title="View source at line 81">Source</a></span></div>
<div class="doc-body">

HTTP 101 Switching Protocols.

</div>
</article>

<article class="doc-entry" id="v-status-teapot">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-teapot</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-teapot" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L237" title="View source at line 237">Source</a></span></div>
<div class="doc-body">

HTTP 418 I'm a teapot.

</div>
</article>

<article class="doc-entry" id="v-status-temporary-redirect">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-temporary-redirect</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-temporary-redirect" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L157" title="View source at line 157">Source</a></span></div>
<div class="doc-body">

HTTP 307 Temporary Redirect.

</div>
</article>

<article class="doc-entry" id="v-status-too-early">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-too-early</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-too-early" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L257" title="View source at line 257">Source</a></span></div>
<div class="doc-body">

HTTP 425 Too Early.

</div>
</article>

<article class="doc-entry" id="v-status-too-many-requests">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-too-many-requests</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-too-many-requests" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L269" title="View source at line 269">Source</a></span></div>
<div class="doc-body">

HTTP 429 Too Many Requests.

</div>
</article>

<article class="doc-entry" id="v-status-unauthorized">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-unauthorized</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-unauthorized" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L169" title="View source at line 169">Source</a></span></div>
<div class="doc-body">

HTTP 401 Unauthorized.

</div>
</article>

<article class="doc-entry" id="v-status-unavailable-for-legal-reasons">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-unavailable-for-legal-reasons</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-unavailable-for-legal-reasons" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L277" title="View source at line 277">Source</a></span></div>
<div class="doc-body">

HTTP 451 Unavailable For Legal Reasons.

</div>
</article>

<article class="doc-entry" id="v-status-unprocessable-entity">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-unprocessable-entity</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-unprocessable-entity" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L245" title="View source at line 245">Source</a></span></div>
<div class="doc-body">

HTTP 422 Unprocessable Entity.

</div>
</article>

<article class="doc-entry" id="v-status-unsupported-media-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-unsupported-media-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-unsupported-media-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L225" title="View source at line 225">Source</a></span></div>
<div class="doc-body">

HTTP 415 Unsupported Media Type.

</div>
</article>

<article class="doc-entry" id="v-status-upgrade-required">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-upgrade-required</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-upgrade-required" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L261" title="View source at line 261">Source</a></span></div>
<div class="doc-body">

HTTP 426 Upgrade Required.

</div>
</article>

<article class="doc-entry" id="v-status-use-proxy">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-use-proxy</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-use-proxy" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L153" title="View source at line 153">Source</a></span></div>
<div class="doc-body">

HTTP 305 Use Proxy.

</div>
</article>

<article class="doc-entry" id="v-status-variant-also-negotiates">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">status-variant-also-negotiates</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-status-variant-also-negotiates" title="Permalink">#</a><a href="/packages/stdlib/Text.Http/source/#L305" title="View source at line 305">Source</a></span></div>
<div class="doc-body">

HTTP 506 Variant Also Negotiates.

</div>
</article>
