# Text.Http

Pure HTTP/1.1 wire parsing/rendering, chunked transfer coding, status texts, canonical headers, forms and Go 1.22 ServeMux pattern routing.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Http`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Http.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Cookie](Text.Cookie.md), [Text.Show](Text.Show.md), [Text.TextProto](Text.TextProto.md), [Text.Url](Text.Url.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Text.Cookie](Text.Cookie.md), [Text.Http](Text.Http.md), [Text.TextProto](Text.TextProto.md), [Text.Url](Text.Url.md), [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Ip](Text.Url.Ip.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Parser](Text.Url.Parser.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Resolve](Text.Url.Resolve.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Pure HTTP/1.1 protocol framing, status codes, canonical headers, request/response models and ServeMux routing.
Provides standard HTTP status codes and reason phrases, canonical MIME header operations built on
Text.TextProto, RFC 9112 HTTP/1.1 wire parsing and rendering with Content-Length and chunked transfer
encoding, query and URL-encoded form helpers built on Text.Url, cookie helpers built on Text.Cookie,
and a method/host/wildcard ServeMux router with specificity precedence, conflict detection, trailing-slash
redirects and 405 Allow generation.

</details>

# Types and constructors

<a id="type-48747470486561646572"></a>

## `HttpHeader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HttpHeader</span></code></pre>

Canonical HTTP header multimap backed by Text.TextProto.MimeHeader.

[Source](/reference-sources/Text.Http.cinch) · line 671

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpHeaderMap</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">MimeHeader</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span>)</code></pre>

<a id="type-4874747050726f746f4572726f72"></a>

## `HttpProtoError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HttpProtoError</span></code></pre>

A named HTTP/1.1 framing, parsing or route pattern error.

[Source](/reference-sources/Text.Http.cinch) · line 432

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpProtoIncomplete</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpProtoBadRequestLine</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpProtoBadStatusLine</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpProtoBadHeader</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpProtoInvalidVersion</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpProtoInvalidContentLength</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpProtoConflictingContentLength</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpProtoUnsupportedTransferEncoding</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpProtoBadChunk</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpProtoTruncatedBody</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpProtoTooLarge</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpProtoBadPattern</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpProtoPatternConflict</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span>)</code></pre>

<a id="type-4874747056657273696f6e"></a>

## `HttpVersion`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HttpVersion</span></code></pre>

Major and minor version numbers.

[Source](/reference-sources/Text.Http.cinch) · line 792

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpVersion</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpVersion</span>))</code></pre>

<a id="type-487474705769726552657175657374"></a>

## `HttpWireRequest`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HttpWireRequest</span></code></pre>

Request fields in the order listed above.

[Source](/reference-sources/Text.Http.cinch) · line 842

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpWireRequest</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span>)))))))))))))))</code></pre>

<a id="type-4874747057697265526573706f6e7365"></a>

## `HttpWireResponse`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HttpWireResponse</span></code></pre>

Response fields in the order listed above.

[Source](/reference-sources/Text.Http.cinch) · line 1130

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpWireResponse</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>)))))))</code></pre>

<a id="type-4d75784d61746368"></a>

## `MuxMatch`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MuxMatch</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The outcome of matching an HTTP request against a ServeMux.

[Source](/reference-sources/Text.Http.cinch) · line 1991

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MuxMatched</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">MuxMatch</span> <span class="tok-variable">a</span>)))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MuxRedirect</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">MuxMatch</span> <span class="tok-variable">a</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MuxMethodNotAllowed</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">MuxMatch</span> <span class="tok-variable">a</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MuxNotFound</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">MuxMatch</span> <span class="tok-variable">a</span></code></pre>

<a id="type-4d75785061747465726e"></a>

## `MuxPattern`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MuxPattern</span></code></pre>

Pattern fields: original string, method, host, segments.

[Source](/reference-sources/Text.Http.cinch) · line 1693

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MuxPattern</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">MuxSegment</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">MuxPattern</span>))))</code></pre>

<a id="type-4d75785365676d656e74"></a>

## `MuxSegment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MuxSegment</span></code></pre>

One parsed segment in a ServeMux route pattern.

[Source](/reference-sources/Text.Http.cinch) · line 1680

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SegLiteral</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">MuxSegment</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SegWild</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">MuxSegment</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SegMulti</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">MuxSegment</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SegExactEnd</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">MuxSegment</span></code></pre>

<a id="type-5061747465726e52656c6174696f6e"></a>

## `PatternRelation`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PatternRelation</span></code></pre>

Specificity relationship between two route patterns.

[Source](/reference-sources/Text.Http.cinch) · line 1839

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PatDisjoint</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">PatternRelation</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PatEquivalent</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">PatternRelation</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PatMoreSpecific</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">PatternRelation</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PatLessSpecific</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">PatternRelation</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PatOverlaps</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">PatternRelation</span></code></pre>

<a id="type-53657276654d7578"></a>

## `ServeMux`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ServeMux</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Registered entries.

[Source](/reference-sources/Text.Http.cinch) · line 1965

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ServeMux</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">MuxEntry</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">ServeMux</span> <span class="tok-variable">a</span>)</code></pre>

# Values

<a id="value-687474702d63616e6f6e6963616c2d6865616465722d6b6579"></a>

## `http-canonical-header-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-canonical-header-key</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Canonicalize an HTTP header field name using Text.TextProto.

[Source](/reference-sources/Text.Http.cinch) · line 676

<a id="value-687474702d6368756e6b2d6465636f6465"></a>

## `http-chunk-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-chunk-decode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Decode an HTTP/1.1 chunked body and optional trailing headers starting at offset start.
Returns decoded body bytes, trailers, and the next byte offset after the terminating blank line.

[Source](/reference-sources/Text.Http.cinch) · line 1279

<a id="value-687474702d6368756e6b2d656e636f6465"></a>

## `http-chunk-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-chunk-encode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Encode raw body bytes using HTTP/1.1 chunked transfer encoding with a given maximum chunk size.

[Source](/reference-sources/Text.Http.cinch) · line 1258

<a id="value-687474702d636c65616e2d70617468"></a>

## `http-clean-path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-clean-path</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the canonical URL path for p, eliminating "." and ".." elements and repeated slashes,
preserving a trailing slash when present.

[Source](/reference-sources/Text.Http.cinch) · line 1637

<a id="value-687474702d6865616465722d616464"></a>

## `http-header-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-header-add</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span>)))</code></pre>

Append a value for a header field, canonicalizing the field name.

[Source](/reference-sources/Text.Http.cinch) · line 684

<a id="value-687474702d6865616465722d636c6f6e65"></a>

## `http-header-clone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-header-clone</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span>)</code></pre>

Return an independent copy of a header map.

[Source](/reference-sources/Text.Http.cinch) · line 734

<a id="value-687474702d6865616465722d64656c"></a>

## `http-header-del`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-header-del</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span>))</code></pre>

Delete a header field and all its values.

[Source](/reference-sources/Text.Http.cinch) · line 712

<a id="value-687474702d6865616465722d656d707479"></a>

## `http-header-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-header-empty</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span></code></pre>

An empty HTTP header map.

[Source](/reference-sources/Text.Http.cinch) · line 680

<a id="value-687474702d6865616465722d656e7472696573"></a>

## `http-header-entries`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-header-entries</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))))</code></pre>

All header fields with their ordered value lists.

[Source](/reference-sources/Text.Http.cinch) · line 722

<a id="value-687474702d6865616465722d66726f6d2d6d696d65"></a>

## `http-header-from-mime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-header-from-mime</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">MimeHeader</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span>)</code></pre>

Wrap a Text.TextProto.MimeHeader as an HttpHeader.

[Source](/reference-sources/Text.Http.cinch) · line 776

<a id="value-687474702d6865616465722d66726f6d2d7061697273"></a>

## `http-header-from-pairs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-header-from-pairs</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span>)</code></pre>

Construct a header map from name/value pairs in order.

[Source](/reference-sources/Text.Http.cinch) · line 738

<a id="value-687474702d6865616465722d676574"></a>

## `http-header-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-header-get</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Retrieve the first value for a header field, if present.

[Source](/reference-sources/Text.Http.cinch) · line 694

<a id="value-687474702d6865616465722d6b657973"></a>

## `http-header-keys`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-header-keys</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

All canonical header keys in insertion order.

[Source](/reference-sources/Text.Http.cinch) · line 717

<a id="value-687474702d6865616465722d72656e646572"></a>

## `http-header-render`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-header-render</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Render HTTP headers as wire lines followed by the terminating CRLF blank line.

[Source](/reference-sources/Text.Http.cinch) · line 766

<a id="value-687474702d6865616465722d736574"></a>

## `http-header-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-header-set</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span>)))</code></pre>

Replace all values for a header field with a single value.

[Source](/reference-sources/Text.Http.cinch) · line 689

<a id="value-687474702d6865616465722d746f2d6d696d65"></a>

## `http-header-to-mime`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-header-to-mime</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">TextProto</span>.<span class="tok-constructor">MimeHeader</span>)</code></pre>

Unwrap the underlying Text.TextProto.MimeHeader.

[Source](/reference-sources/Text.Http.cinch) · line 771

<a id="value-687474702d6865616465722d746f2d7061697273"></a>

## `http-header-to-pairs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-header-to-pairs</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Flatten a header map into ordered (CanonicalKey, Value) pairs.

[Source](/reference-sources/Text.Http.cinch) · line 749

<a id="value-687474702d6865616465722d76616c756573"></a>

## `http-header-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-header-values</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Retrieve all values for a header field in insertion order.

[Source](/reference-sources/Text.Http.cinch) · line 701

<a id="value-687474702d6e65772d72657175657374"></a>

## `http-new-request`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-new-request</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span>)))</code></pre>

Construct a fresh HTTP/1.1 request from method, target URI/path, and body bytes.

[Source](/reference-sources/Text.Http.cinch) · line 845

<a id="value-687474702d70617273652d72657175657374"></a>

## `http-parse-request`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-parse-request</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span>, <span class="tok-constructor">Nat</span>)))</code></pre>

Parse one HTTP/1.1 request from wire bytes, returning the request and exact bytes consumed.

[Source](/reference-sources/Text.Http.cinch) · line 1412

<a id="value-687474702d70617273652d726571756573742d6c696d6974"></a>

## `http-parse-request-limit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-parse-request-limit</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span>, <span class="tok-constructor">Nat</span>)))))</code></pre>

Parse one HTTP/1.1 request with explicit header and body byte limits (0 disables a limit).

[Source](/reference-sources/Text.Http.cinch) · line 1416

<a id="value-687474702d70617273652d726573706f6e7365"></a>

## `http-parse-response`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-parse-response</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>, <span class="tok-constructor">Nat</span>)))</code></pre>

Parse an HTTP/1.1 response assuming a GET request.

[Source](/reference-sources/Text.Http.cinch) · line 1514

<a id="value-687474702d70617273652d726573706f6e73652d666f72"></a>

## `http-parse-response-for`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-parse-response-for</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Parse an HTTP/1.1 response for a given request method (handling HEAD and bodyless statuses).

[Source](/reference-sources/Text.Http.cinch) · line 1518

<a id="value-687474702d70617273652d726573706f6e73652d6c696d6974"></a>

## `http-parse-response-limit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-parse-response-limit</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>, <span class="tok-constructor">Nat</span>))))))</code></pre>

Parse an HTTP/1.1 response with explicit request method, header limit, and body limit.

[Source](/reference-sources/Text.Http.cinch) · line 1522

<a id="value-687474702d70617273652d76657273696f6e"></a>

## `http-parse-version`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-parse-version</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpVersion</span>))</code></pre>

Parse an HTTP version token "HTTP/<major>.<minor>".

[Source](/reference-sources/Text.Http.cinch) · line 816

<a id="value-687474702d70726f746f2d6572726f722d6d657373616765"></a>

## `http-proto-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-proto-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a readable diagnostic for an HTTP protocol error.

[Source](/reference-sources/Text.Http.cinch) · line 461

<a id="value-687474702d72656e6465722d72657175657374"></a>

## `http-render-request`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-render-request</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Render an HTTP/1.1 request to wire bytes using Content-Length when a body is present.

[Source](/reference-sources/Text.Http.cinch) · line 1482

<a id="value-687474702d72656e6465722d726571756573742d6368756e6b6564"></a>

## `http-render-request-chunked`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-render-request-chunked</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Render an HTTP/1.1 request using Transfer-Encoding: chunked with the specified chunk size.

[Source](/reference-sources/Text.Http.cinch) · line 1496

<a id="value-687474702d72656e6465722d726573706f6e7365"></a>

## `http-render-response`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-render-response</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Render an HTTP/1.1 response to wire bytes with automatic status text and Content-Length.

[Source](/reference-sources/Text.Http.cinch) · line 1593

<a id="value-687474702d72656e6465722d726573706f6e73652d6368756e6b6564"></a>

## `http-render-response-chunked`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-render-response-chunked</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Render an HTTP/1.1 response using Transfer-Encoding: chunked.

[Source](/reference-sources/Text.Http.cinch) · line 1618

<a id="value-687474702d72656e6465722d726573706f6e73652d666f72"></a>

## `http-render-response-for`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-render-response-for</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Render an HTTP/1.1 response for a specific request method (omitting wire body for HEAD).

[Source](/reference-sources/Text.Http.cinch) · line 1597

<a id="value-687474702d726573706f6e73652d6279746573"></a>

## `http-response-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-response-bytes</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>)))</code></pre>

Build an HTTP/1.1 response with an explicit Content-Type and body bytes.

[Source](/reference-sources/Text.Http.cinch) · line 1144

<a id="value-687474702d726573706f6e73652d6572726f72"></a>

## `http-response-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-response-error</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>))</code></pre>

Build a standard plain-text error response with X-Content-Type-Options: nosniff.

[Source](/reference-sources/Text.Http.cinch) · line 1150

<a id="value-687474702d726573706f6e73652d6e6577"></a>

## `http-response-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-response-new</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>)))</code></pre>

Build an HTTP/1.1 response with status code, headers and body bytes.

[Source](/reference-sources/Text.Http.cinch) · line 1133

<a id="value-687474702d726573706f6e73652d7265646972656374"></a>

## `http-response-redirect`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-response-redirect</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>))</code></pre>

Build an HTTP redirect response with Location header.

[Source](/reference-sources/Text.Http.cinch) · line 1157

<a id="value-687474702d726573706f6e73652d74657874"></a>

## `http-response-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-response-text</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>))</code></pre>

Build a UTF-8 plain text HTTP/1.1 response.

[Source](/reference-sources/Text.Http.cinch) · line 1138

<a id="value-687474702d7365742d636f6f6b6965"></a>

## `http-set-cookie`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-set-cookie</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span>))</code></pre>

Append a Set-Cookie header for a Cookie value when valid to serialize.

[Source](/reference-sources/Text.Http.cinch) · line 780

<a id="value-687474702d7374617475732d74657874"></a>

## `http-status-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-status-text</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The standard reason phrase for an HTTP status code, or empty text if unknown.

[Source](/reference-sources/Text.Http.cinch) · line 325

<a id="value-687474702d76657273696f6e2d6d616a6f72"></a>

## `http-version-major`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-version-major</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpVersion</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The major version number.

[Source](/reference-sources/Text.Http.cinch) · line 803

<a id="value-687474702d76657273696f6e2d6d696e6f72"></a>

## `http-version-minor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-version-minor</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpVersion</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The minor version number.

[Source](/reference-sources/Text.Http.cinch) · line 807

<a id="value-687474702d76657273696f6e2d737472696e67"></a>

## `http-version-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-version-string</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpVersion</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an HTTP version token such as "HTTP/1.1".

[Source](/reference-sources/Text.Http.cinch) · line 811

<a id="value-687474703130"></a>

## `http10`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http10</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpVersion</span></code></pre>

HTTP/1.0 protocol version.

[Source](/reference-sources/Text.Http.cinch) · line 795

<a id="value-687474703131"></a>

## `http11`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http11</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpVersion</span></code></pre>

HTTP/1.1 protocol version.

[Source](/reference-sources/Text.Http.cinch) · line 799

<a id="value-6d6574686f642d636f6e6e656374"></a>

## `method-connect`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">method-connect</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span></code></pre>

The HTTP CONNECT method token.

[Source](/reference-sources/Text.Http.cinch) · line 416

<a id="value-6d6574686f642d64656c657465"></a>

## `method-delete`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">method-delete</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span></code></pre>

The HTTP DELETE method token.

[Source](/reference-sources/Text.Http.cinch) · line 412

<a id="value-6d6574686f642d676574"></a>

## `method-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">method-get</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span></code></pre>

The HTTP GET method token.

[Source](/reference-sources/Text.Http.cinch) · line 392

<a id="value-6d6574686f642d68656164"></a>

## `method-head`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">method-head</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span></code></pre>

The HTTP HEAD method token.

[Source](/reference-sources/Text.Http.cinch) · line 396

<a id="value-6d6574686f642d6f7074696f6e73"></a>

## `method-options`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">method-options</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span></code></pre>

The HTTP OPTIONS method token.

[Source](/reference-sources/Text.Http.cinch) · line 420

<a id="value-6d6574686f642d7061746368"></a>

## `method-patch`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">method-patch</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span></code></pre>

The HTTP PATCH method token.

[Source](/reference-sources/Text.Http.cinch) · line 408

<a id="value-6d6574686f642d706f7374"></a>

## `method-post`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">method-post</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span></code></pre>

The HTTP POST method token.

[Source](/reference-sources/Text.Http.cinch) · line 400

<a id="value-6d6574686f642d707574"></a>

## `method-put`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">method-put</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span></code></pre>

The HTTP PUT method token.

[Source](/reference-sources/Text.Http.cinch) · line 404

<a id="value-6d6574686f642d7472616365"></a>

## `method-trace`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">method-trace</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span></code></pre>

The HTTP TRACE method token.

[Source](/reference-sources/Text.Http.cinch) · line 424

<a id="value-6d75782d616c6c6f772d686561646572"></a>

## `mux-allow-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mux-allow-header</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Format a list of allowed methods as a standard HTTP Allow header value.

[Source](/reference-sources/Text.Http.cinch) · line 2156

<a id="value-6d75782d656d707479"></a>

## `mux-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mux-empty</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">ServeMux</span> <span class="tok-variable">a</span>)</code></pre>

An empty ServeMux with no registered routes.

[Source](/reference-sources/Text.Http.cinch) · line 1968

<a id="value-6d75782d68616e646c65"></a>

## `mux-handle`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mux-handle</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-variable">a</span> -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">ServeMux</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">ServeMux</span> <span class="tok-variable">a</span>)))))</code></pre>

Register a pattern and handler, rejecting invalid syntax or conflicting routes.

[Source](/reference-sources/Text.Http.cinch) · line 1972

<a id="value-6d75782d68616e646c6572"></a>

## `mux-handler`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mux-handler</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">ServeMux</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">MuxMatch</span> <span class="tok-variable">a</span>)))</code></pre>

Match an HTTP wire request against a ServeMux.

[Source](/reference-sources/Text.Http.cinch) · line 2002

<a id="value-6d75782d6d61746368"></a>

## `mux-match`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mux-match</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">ServeMux</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">MuxMatch</span> <span class="tok-variable">a</span>)))))</code></pre>

Match a method, host and path (optionally containing "?query") against a ServeMux.

[Source](/reference-sources/Text.Http.cinch) · line 2007

<a id="value-6d75782d70617273652d7061747465726e"></a>

## `mux-parse-pattern`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mux-parse-pattern</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">MuxPattern</span>))</code></pre>

Parse and validate a route pattern of the form "[METHOD ][HOST]/[PATH]".

[Source](/reference-sources/Text.Http.cinch) · line 1712

<a id="value-6d75782d7061747465726e2d636f6d70617265"></a>

## `mux-pattern-compare`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mux-pattern-compare</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">MuxPattern</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">MuxPattern</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">PatternRelation</span>))</code></pre>

Compare two route patterns for precedence and conflicts.
When patterns would otherwise overlap or invert on path/method, a pattern with an explicit host
takes precedence over a host-less pattern.

[Source](/reference-sources/Text.Http.cinch) · line 1880

<a id="value-6d75782d7061747465726e2d686f7374"></a>

## `mux-pattern-host`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mux-pattern-host</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">MuxPattern</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Host constraint of a parsed route pattern (empty if unconstrained).

[Source](/reference-sources/Text.Http.cinch) · line 1704

<a id="value-6d75782d7061747465726e2d6d6574686f64"></a>

## `mux-pattern-method`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mux-pattern-method</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">MuxPattern</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Method constraint of a parsed route pattern (empty if unconstrained).

[Source](/reference-sources/Text.Http.cinch) · line 1700

<a id="value-6d75782d7061747465726e2d737472696e67"></a>

## `mux-pattern-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mux-pattern-string</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">MuxPattern</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Original text of a parsed route pattern.

[Source](/reference-sources/Text.Http.cinch) · line 1696

<a id="value-7265712d6164642d636f6f6b6965"></a>

## `req-add-cookie`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-add-cookie</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span>))</code></pre>

Append a cookie name=value pair to the request's Cookie header.

[Source](/reference-sources/Text.Http.cinch) · line 1029

<a id="value-7265712d6164642d686561646572"></a>

## `req-add-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-add-header</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span>)))</code></pre>

Return a request with an additional header value appended.

[Source](/reference-sources/Text.Http.cinch) · line 987

<a id="value-7265712d626f6479"></a>

## `req-body`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-body</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The decoded body bytes of a request.

[Source](/reference-sources/Text.Http.cinch) · line 946

<a id="value-7265712d636c6f7365"></a>

## `req-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-close</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether the connection should be closed after replying to this request.

[Source](/reference-sources/Text.Http.cinch) · line 977

<a id="value-7265712d636f6e74656e742d6c656e677468"></a>

## `req-content-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-content-length</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Int</span>)</code></pre>

The declared content length (-1 when chunked or unspecified).

[Source](/reference-sources/Text.Http.cinch) · line 973

<a id="value-7265712d636f6f6b6965"></a>

## `req-cookie`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-cookie</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span>)))</code></pre>

Find the first request cookie matching name.

[Source](/reference-sources/Text.Http.cinch) · line 1053

<a id="value-7265712d636f6f6b696573"></a>

## `req-cookies`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-cookies</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span>))</code></pre>

Parse all cookies carried in the request's Cookie headers via Text.Cookie.

[Source](/reference-sources/Text.Http.cinch) · line 1037

<a id="value-7265712d666f726d2d76616c7565"></a>

## `req-form-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-form-value</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Look up the first form value (body parameters before query parameters).

[Source](/reference-sources/Text.Http.cinch) · line 1115

<a id="value-7265712d686561646572"></a>

## `req-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-header</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Retrieve the first value for a request header.

[Source](/reference-sources/Text.Http.cinch) · line 938

<a id="value-7265712d6865616465722d76616c756573"></a>

## `req-header-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-header-values</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Retrieve all values for a request header.

[Source](/reference-sources/Text.Http.cinch) · line 942

<a id="value-7265712d68656164657273"></a>

## `req-headers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-headers</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span>)</code></pre>

The canonical header map of a request.

[Source](/reference-sources/Text.Http.cinch) · line 934

<a id="value-7265712d686f7374"></a>

## `req-host`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-host</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The host of a request.

[Source](/reference-sources/Text.Http.cinch) · line 906

<a id="value-7265712d6d6574686f64"></a>

## `req-method`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-method</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The HTTP method of a request.

[Source](/reference-sources/Text.Http.cinch) · line 898

<a id="value-7265712d70617273652d666f726d"></a>

## `req-parse-form`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-parse-form</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span>))</code></pre>

Parse both URL-encoded body parameters (taking priority) and URL query parameters.

[Source](/reference-sources/Text.Http.cinch) · line 1094

<a id="value-7265712d70617468"></a>

## `req-path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-path</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The decoded path of a request.

[Source](/reference-sources/Text.Http.cinch) · line 910

<a id="value-7265712d706174682d76616c7565"></a>

## `req-path-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-path-value</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Look up a single matched route path parameter by name.

[Source](/reference-sources/Text.Http.cinch) · line 958

<a id="value-7265712d706174682d76616c756573"></a>

## `req-path-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-path-values</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

All matched route path parameter bindings.

[Source](/reference-sources/Text.Http.cinch) · line 954

<a id="value-7265712d706f73742d666f726d"></a>

## `req-post-form`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-post-form</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span>))</code></pre>

Parse application/x-www-form-urlencoded parameters from a POST, PUT or PATCH request body.

[Source](/reference-sources/Text.Http.cinch) · line 1078

<a id="value-7265712d70726f746f"></a>

## `req-proto`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-proto</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The HTTP protocol version string (such as "HTTP/1.1") of a request.

[Source](/reference-sources/Text.Http.cinch) · line 930

<a id="value-7265712d70726f746f2d6d616a6f72"></a>

## `req-proto-major`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-proto-major</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The HTTP major version number of a request.

[Source](/reference-sources/Text.Http.cinch) · line 922

<a id="value-7265712d70726f746f2d6d696e6f72"></a>

## `req-proto-minor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-proto-minor</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The HTTP minor version number of a request.

[Source](/reference-sources/Text.Http.cinch) · line 926

<a id="value-7265712d7175657279"></a>

## `req-query`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-query</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span>))</code></pre>

Parse the request's raw query string into URL values via Text.Url.

[Source](/reference-sources/Text.Http.cinch) · line 1063

<a id="value-7265712d71756572792d706172616d"></a>

## `req-query-param`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-query-param</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Look up the first value of a query parameter by name.

[Source](/reference-sources/Text.Http.cinch) · line 1069

<a id="value-7265712d7261772d70617468"></a>

## `req-raw-path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-raw-path</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The raw escaped path of a request.

[Source](/reference-sources/Text.Http.cinch) · line 914

<a id="value-7265712d7261772d7175657279"></a>

## `req-raw-query`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-raw-query</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The raw query string (without leading '?') of a request.

[Source](/reference-sources/Text.Http.cinch) · line 918

<a id="value-7265712d72656d6f74652d61646472"></a>

## `req-remote-addr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-remote-addr</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The remote address string recorded on a request.

[Source](/reference-sources/Text.Http.cinch) · line 969

<a id="value-7265712d726571756573742d757269"></a>

## `req-request-uri`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-request-uri</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The raw request-target URI of a request.

[Source](/reference-sources/Text.Http.cinch) · line 902

<a id="value-7265712d747261696c657273"></a>

## `req-trailers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-trailers</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span>)</code></pre>

The trailing headers decoded after a chunked request body.

[Source](/reference-sources/Text.Http.cinch) · line 950

<a id="value-7265712d776974682d626f6479"></a>

## `req-with-body`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-with-body</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span>))</code></pre>

Return a request with its body bytes replaced and content length updated.

[Source](/reference-sources/Text.Http.cinch) · line 1005

<a id="value-7265712d776974682d686561646572"></a>

## `req-with-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-with-header</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span>)))</code></pre>

Return a request with a header replaced.

[Source](/reference-sources/Text.Http.cinch) · line 981

<a id="value-7265712d776974682d68656164657273"></a>

## `req-with-headers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-with-headers</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span>))</code></pre>

Return a request with its entire header map replaced.

[Source](/reference-sources/Text.Http.cinch) · line 993

<a id="value-7265712d776974682d686f7374"></a>

## `req-with-host`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-with-host</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span>))</code></pre>

Return a request with its Host field updated.

[Source](/reference-sources/Text.Http.cinch) · line 999

<a id="value-7265712d776974682d706174682d76616c756573"></a>

## `req-with-path-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-with-path-values</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span>))</code></pre>

Return a request with matched route path parameters attached.

[Source](/reference-sources/Text.Http.cinch) · line 1011

<a id="value-7265712d776974682d70726f746f"></a>

## `req-with-proto`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-with-proto</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpVersion</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span>))</code></pre>

Return a request with its protocol version updated.

[Source](/reference-sources/Text.Http.cinch) · line 1023

<a id="value-7265712d776974682d72656d6f74652d61646472"></a>

## `req-with-remote-addr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">req-with-remote-addr</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span>))</code></pre>

Return a request with its remote address updated.

[Source](/reference-sources/Text.Http.cinch) · line 1017

<a id="value-726573702d6164642d636f6f6b6965"></a>

## `resp-add-cookie`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">resp-add-cookie</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>))</code></pre>

Append a Set-Cookie header to a response.

[Source](/reference-sources/Text.Http.cinch) · line 1249

<a id="value-726573702d6164642d686561646572"></a>

## `resp-add-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">resp-add-header</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>)))</code></pre>

Return a response with an additional header value appended.

[Source](/reference-sources/Text.Http.cinch) · line 1229

<a id="value-726573702d626f6479"></a>

## `resp-body`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">resp-body</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The decoded body bytes of a response.

[Source](/reference-sources/Text.Http.cinch) · line 1194

<a id="value-726573702d636f6f6b696573"></a>

## `resp-cookies`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">resp-cookies</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span>))</code></pre>

Parse all valid Set-Cookie headers from a response via Text.Cookie.

[Source](/reference-sources/Text.Http.cinch) · line 1202

<a id="value-726573702d686561646572"></a>

## `resp-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">resp-header</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Look up the first value of a response header.

[Source](/reference-sources/Text.Http.cinch) · line 1186

<a id="value-726573702d6865616465722d76616c756573"></a>

## `resp-header-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">resp-header-values</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Look up all values of a response header.

[Source](/reference-sources/Text.Http.cinch) · line 1190

<a id="value-726573702d68656164657273"></a>

## `resp-headers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">resp-headers</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span>)</code></pre>

The canonical header map of a response.

[Source](/reference-sources/Text.Http.cinch) · line 1182

<a id="value-726573702d70726f746f2d6d616a6f72"></a>

## `resp-proto-major`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">resp-proto-major</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The HTTP major version of a response.

[Source](/reference-sources/Text.Http.cinch) · line 1174

<a id="value-726573702d70726f746f2d6d696e6f72"></a>

## `resp-proto-minor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">resp-proto-minor</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The HTTP minor version of a response.

[Source](/reference-sources/Text.Http.cinch) · line 1178

<a id="value-726573702d726561736f6e"></a>

## `resp-reason`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">resp-reason</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The reason phrase of a response.

[Source](/reference-sources/Text.Http.cinch) · line 1170

<a id="value-726573702d737461747573"></a>

## `resp-status`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">resp-status</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The HTTP status code of a response.

[Source](/reference-sources/Text.Http.cinch) · line 1166

<a id="value-726573702d747261696c657273"></a>

## `resp-trailers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">resp-trailers</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span>)</code></pre>

The trailing headers of a response.

[Source](/reference-sources/Text.Http.cinch) · line 1198

<a id="value-726573702d776974682d626f6479"></a>

## `resp-with-body`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">resp-with-body</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>))</code></pre>

Return a response with its body bytes replaced.

[Source](/reference-sources/Text.Http.cinch) · line 1239

<a id="value-726573702d776974682d686561646572"></a>

## `resp-with-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">resp-with-header</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>)))</code></pre>

Return a response with a header replaced.

[Source](/reference-sources/Text.Http.cinch) · line 1224

<a id="value-726573702d776974682d68656164657273"></a>

## `resp-with-headers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">resp-with-headers</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>))</code></pre>

Return a response with its entire header map replaced.

[Source](/reference-sources/Text.Http.cinch) · line 1234

<a id="value-726573702d776974682d726561736f6e"></a>

## `resp-with-reason`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">resp-with-reason</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>))</code></pre>

Return a response with an explicit reason phrase.

[Source](/reference-sources/Text.Http.cinch) · line 1219

<a id="value-726573702d776974682d737461747573"></a>

## `resp-with-status`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">resp-with-status</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>))</code></pre>

Return a response with an updated status code and matching default reason phrase.

[Source](/reference-sources/Text.Http.cinch) · line 1214

<a id="value-726573702d776974682d747261696c657273"></a>

## `resp-with-trailers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">resp-with-trailers</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>))</code></pre>

Return a response with its trailers replaced.

[Source](/reference-sources/Text.Http.cinch) · line 1244

<a id="value-7374617475732d6163636570746564"></a>

## `status-accepted`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-accepted</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 202 Accepted.

[Source](/reference-sources/Text.Http.cinch) · line 101

<a id="value-7374617475732d616c72656164792d7265706f72746564"></a>

## `status-already-reported`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-already-reported</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 208 Already Reported.

[Source](/reference-sources/Text.Http.cinch) · line 125

<a id="value-7374617475732d6261642d67617465776179"></a>

## `status-bad-gateway`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-bad-gateway</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 502 Bad Gateway.

[Source](/reference-sources/Text.Http.cinch) · line 289

<a id="value-7374617475732d6261642d72657175657374"></a>

## `status-bad-request`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-bad-request</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 400 Bad Request.

[Source](/reference-sources/Text.Http.cinch) · line 165

<a id="value-7374617475732d636f6e666c696374"></a>

## `status-conflict`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-conflict</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 409 Conflict.

[Source](/reference-sources/Text.Http.cinch) · line 201

<a id="value-7374617475732d636f6e74696e7565"></a>

## `status-continue`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-continue</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 100 Continue.

[Source](/reference-sources/Text.Http.cinch) · line 77

<a id="value-7374617475732d63726561746564"></a>

## `status-created`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-created</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 201 Created.

[Source](/reference-sources/Text.Http.cinch) · line 97

<a id="value-7374617475732d6561726c792d68696e7473"></a>

## `status-early-hints`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-early-hints</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 103 Early Hints.

[Source](/reference-sources/Text.Http.cinch) · line 89

<a id="value-7374617475732d6578706563746174696f6e2d6661696c6564"></a>

## `status-expectation-failed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-expectation-failed</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 417 Expectation Failed.

[Source](/reference-sources/Text.Http.cinch) · line 233

<a id="value-7374617475732d6661696c65642d646570656e64656e6379"></a>

## `status-failed-dependency`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-failed-dependency</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 424 Failed Dependency.

[Source](/reference-sources/Text.Http.cinch) · line 253

<a id="value-7374617475732d666f7262696464656e"></a>

## `status-forbidden`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-forbidden</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 403 Forbidden.

[Source](/reference-sources/Text.Http.cinch) · line 177

<a id="value-7374617475732d666f756e64"></a>

## `status-found`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-found</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 302 Found.

[Source](/reference-sources/Text.Http.cinch) · line 141

<a id="value-7374617475732d676174657761792d74696d656f7574"></a>

## `status-gateway-timeout`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-gateway-timeout</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 504 Gateway Timeout.

[Source](/reference-sources/Text.Http.cinch) · line 297

<a id="value-7374617475732d676f6e65"></a>

## `status-gone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-gone</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 410 Gone.

[Source](/reference-sources/Text.Http.cinch) · line 205

<a id="value-7374617475732d687474702d76657273696f6e2d6e6f742d737570706f72746564"></a>

## `status-http-version-not-supported`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-http-version-not-supported</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 505 HTTP Version Not Supported.

[Source](/reference-sources/Text.Http.cinch) · line 301

<a id="value-7374617475732d696d2d75736564"></a>

## `status-im-used`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-im-used</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 226 IM Used.

[Source](/reference-sources/Text.Http.cinch) · line 129

<a id="value-7374617475732d696e73756666696369656e742d73746f72616765"></a>

## `status-insufficient-storage`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-insufficient-storage</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 507 Insufficient Storage.

[Source](/reference-sources/Text.Http.cinch) · line 309

<a id="value-7374617475732d696e7465726e616c2d7365727665722d6572726f72"></a>

## `status-internal-server-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-internal-server-error</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 500 Internal Server Error.

[Source](/reference-sources/Text.Http.cinch) · line 281

<a id="value-7374617475732d6c656e6774682d7265717569726564"></a>

## `status-length-required`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-length-required</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 411 Length Required.

[Source](/reference-sources/Text.Http.cinch) · line 209

<a id="value-7374617475732d6c6f636b6564"></a>

## `status-locked`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-locked</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 423 Locked.

[Source](/reference-sources/Text.Http.cinch) · line 249

<a id="value-7374617475732d6c6f6f702d6465746563746564"></a>

## `status-loop-detected`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-loop-detected</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 508 Loop Detected.

[Source](/reference-sources/Text.Http.cinch) · line 313

<a id="value-7374617475732d6d6574686f642d6e6f742d616c6c6f776564"></a>

## `status-method-not-allowed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-method-not-allowed</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 405 Method Not Allowed.

[Source](/reference-sources/Text.Http.cinch) · line 185

<a id="value-7374617475732d6d697364697265637465642d72657175657374"></a>

## `status-misdirected-request`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-misdirected-request</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 421 Misdirected Request.

[Source](/reference-sources/Text.Http.cinch) · line 241

<a id="value-7374617475732d6d6f7665642d7065726d616e656e746c79"></a>

## `status-moved-permanently`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-moved-permanently</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 301 Moved Permanently.

[Source](/reference-sources/Text.Http.cinch) · line 137

<a id="value-7374617475732d6d756c74692d737461747573"></a>

## `status-multi-status`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-multi-status</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 207 Multi-Status.

[Source](/reference-sources/Text.Http.cinch) · line 121

<a id="value-7374617475732d6d756c7469706c652d63686f69636573"></a>

## `status-multiple-choices`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-multiple-choices</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 300 Multiple Choices.

[Source](/reference-sources/Text.Http.cinch) · line 133

<a id="value-7374617475732d6e6574776f726b2d61757468656e7469636174696f6e2d7265717569726564"></a>

## `status-network-authentication-required`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-network-authentication-required</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 511 Network Authentication Required.

[Source](/reference-sources/Text.Http.cinch) · line 321

<a id="value-7374617475732d6e6f2d636f6e74656e74"></a>

## `status-no-content`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-no-content</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 204 No Content.

[Source](/reference-sources/Text.Http.cinch) · line 109

<a id="value-7374617475732d6e6f6e2d617574686f72697461746976652d696e666f"></a>

## `status-non-authoritative-info`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-non-authoritative-info</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 203 Non-Authoritative Information.

[Source](/reference-sources/Text.Http.cinch) · line 105

<a id="value-7374617475732d6e6f742d61636365707461626c65"></a>

## `status-not-acceptable`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-not-acceptable</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 406 Not Acceptable.

[Source](/reference-sources/Text.Http.cinch) · line 189

<a id="value-7374617475732d6e6f742d657874656e646564"></a>

## `status-not-extended`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-not-extended</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 510 Not Extended.

[Source](/reference-sources/Text.Http.cinch) · line 317

<a id="value-7374617475732d6e6f742d666f756e64"></a>

## `status-not-found`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-not-found</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 404 Not Found.

[Source](/reference-sources/Text.Http.cinch) · line 181

<a id="value-7374617475732d6e6f742d696d706c656d656e746564"></a>

## `status-not-implemented`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-not-implemented</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 501 Not Implemented.

[Source](/reference-sources/Text.Http.cinch) · line 285

<a id="value-7374617475732d6e6f742d6d6f646966696564"></a>

## `status-not-modified`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-not-modified</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 304 Not Modified.

[Source](/reference-sources/Text.Http.cinch) · line 149

<a id="value-7374617475732d6f6b"></a>

## `status-ok`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-ok</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 200 OK.

[Source](/reference-sources/Text.Http.cinch) · line 93

<a id="value-7374617475732d7061727469616c2d636f6e74656e74"></a>

## `status-partial-content`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-partial-content</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 206 Partial Content.

[Source](/reference-sources/Text.Http.cinch) · line 117

<a id="value-7374617475732d7061796d656e742d7265717569726564"></a>

## `status-payment-required`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-payment-required</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 402 Payment Required.

[Source](/reference-sources/Text.Http.cinch) · line 173

<a id="value-7374617475732d7065726d616e656e742d7265646972656374"></a>

## `status-permanent-redirect`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-permanent-redirect</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 308 Permanent Redirect.

[Source](/reference-sources/Text.Http.cinch) · line 161

<a id="value-7374617475732d707265636f6e646974696f6e2d6661696c6564"></a>

## `status-precondition-failed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-precondition-failed</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 412 Precondition Failed.

[Source](/reference-sources/Text.Http.cinch) · line 213

<a id="value-7374617475732d707265636f6e646974696f6e2d7265717569726564"></a>

## `status-precondition-required`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-precondition-required</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 428 Precondition Required.

[Source](/reference-sources/Text.Http.cinch) · line 265

<a id="value-7374617475732d70726f63657373696e67"></a>

## `status-processing`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-processing</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 102 Processing.

[Source](/reference-sources/Text.Http.cinch) · line 85

<a id="value-7374617475732d70726f78792d617574682d7265717569726564"></a>

## `status-proxy-auth-required`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-proxy-auth-required</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 407 Proxy Authentication Required.

[Source](/reference-sources/Text.Http.cinch) · line 193

<a id="value-7374617475732d726571756573742d656e746974792d746f6f2d6c61726765"></a>

## `status-request-entity-too-large`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-request-entity-too-large</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 413 Request Entity Too Large.

[Source](/reference-sources/Text.Http.cinch) · line 217

<a id="value-7374617475732d726571756573742d6865616465722d6669656c64732d746f6f2d6c61726765"></a>

## `status-request-header-fields-too-large`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-request-header-fields-too-large</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 431 Request Header Fields Too Large.

[Source](/reference-sources/Text.Http.cinch) · line 273

<a id="value-7374617475732d726571756573742d74696d656f7574"></a>

## `status-request-timeout`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-request-timeout</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 408 Request Timeout.

[Source](/reference-sources/Text.Http.cinch) · line 197

<a id="value-7374617475732d726571756573742d7572692d746f6f2d6c6f6e67"></a>

## `status-request-uri-too-long`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-request-uri-too-long</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 414 Request URI Too Long.

[Source](/reference-sources/Text.Http.cinch) · line 221

<a id="value-7374617475732d7265717565737465642d72616e67652d6e6f742d7361746973666961626c65"></a>

## `status-requested-range-not-satisfiable`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-requested-range-not-satisfiable</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 416 Requested Range Not Satisfiable.

[Source](/reference-sources/Text.Http.cinch) · line 229

<a id="value-7374617475732d72657365742d636f6e74656e74"></a>

## `status-reset-content`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-reset-content</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 205 Reset Content.

[Source](/reference-sources/Text.Http.cinch) · line 113

<a id="value-7374617475732d7365652d6f74686572"></a>

## `status-see-other`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-see-other</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 303 See Other.

[Source](/reference-sources/Text.Http.cinch) · line 145

<a id="value-7374617475732d736572766963652d756e617661696c61626c65"></a>

## `status-service-unavailable`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-service-unavailable</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 503 Service Unavailable.

[Source](/reference-sources/Text.Http.cinch) · line 293

<a id="value-7374617475732d737769746368696e672d70726f746f636f6c73"></a>

## `status-switching-protocols`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-switching-protocols</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 101 Switching Protocols.

[Source](/reference-sources/Text.Http.cinch) · line 81

<a id="value-7374617475732d746561706f74"></a>

## `status-teapot`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-teapot</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 418 I'm a teapot.

[Source](/reference-sources/Text.Http.cinch) · line 237

<a id="value-7374617475732d74656d706f726172792d7265646972656374"></a>

## `status-temporary-redirect`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-temporary-redirect</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 307 Temporary Redirect.

[Source](/reference-sources/Text.Http.cinch) · line 157

<a id="value-7374617475732d746f6f2d6561726c79"></a>

## `status-too-early`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-too-early</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 425 Too Early.

[Source](/reference-sources/Text.Http.cinch) · line 257

<a id="value-7374617475732d746f6f2d6d616e792d7265717565737473"></a>

## `status-too-many-requests`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-too-many-requests</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 429 Too Many Requests.

[Source](/reference-sources/Text.Http.cinch) · line 269

<a id="value-7374617475732d756e617574686f72697a6564"></a>

## `status-unauthorized`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-unauthorized</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 401 Unauthorized.

[Source](/reference-sources/Text.Http.cinch) · line 169

<a id="value-7374617475732d756e617661696c61626c652d666f722d6c6567616c2d726561736f6e73"></a>

## `status-unavailable-for-legal-reasons`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-unavailable-for-legal-reasons</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 451 Unavailable For Legal Reasons.

[Source](/reference-sources/Text.Http.cinch) · line 277

<a id="value-7374617475732d756e70726f6365737361626c652d656e74697479"></a>

## `status-unprocessable-entity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-unprocessable-entity</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 422 Unprocessable Entity.

[Source](/reference-sources/Text.Http.cinch) · line 245

<a id="value-7374617475732d756e737570706f727465642d6d656469612d74797065"></a>

## `status-unsupported-media-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-unsupported-media-type</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 415 Unsupported Media Type.

[Source](/reference-sources/Text.Http.cinch) · line 225

<a id="value-7374617475732d757067726164652d7265717569726564"></a>

## `status-upgrade-required`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-upgrade-required</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 426 Upgrade Required.

[Source](/reference-sources/Text.Http.cinch) · line 261

<a id="value-7374617475732d7573652d70726f7879"></a>

## `status-use-proxy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-use-proxy</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 305 Use Proxy.

[Source](/reference-sources/Text.Http.cinch) · line 153

<a id="value-7374617475732d76617269616e742d616c736f2d6e65676f746961746573"></a>

## `status-variant-also-negotiates`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">status-variant-also-negotiates</span> :: <span class="tok-constructor">Nat</span></code></pre>

HTTP 506 Variant Also Negotiates.

[Source](/reference-sources/Text.Http.cinch) · line 305

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Http.cinch`. Source SHA-256: `6bd8ad591566a16cc25d7072b3ab5009d3f08ac1a98a4c74f626be9387ddad15`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
