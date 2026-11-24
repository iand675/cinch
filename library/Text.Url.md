# Text.Url

Immutable byte-preserving URLs, percent escaping, query multimaps, parsing and reference resolution.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Url`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Url.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Parser](Text.Url.Parser.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Resolve](Text.Url.Resolve.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

Non-embedded source closure: [Text.Url](Text.Url.md), [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Ip](Text.Url.Ip.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Parser](Text.Url.Parser.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Resolve](Text.Url.Resolve.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Byte-preserving URLs, percent escaping, query multimaps and reference resolution.
Parsed decoded fields retain arbitrary bytes. Immutable URLs and query values
snapshot caller buffers; observations return copied byte storage. Text views
are explicit optional UTF-8 conversions. Existing URI APIs remain independent.

</details>

# Types and constructors

<a id="type-55726c"></a>

## `Url`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Url</span></code></pre>

An immutable URL whose component byte buffers have no external mutable aliases.

[Source](/reference-sources/Text.Url.Model.cinch) · line 34

<a id="type-55726c436f6d706f6e656e7473"></a>

## `UrlComponents`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UrlComponents</span></code></pre>

Explicit URL fields: scheme, opaque, user, host, path, fragment, raw query,
raw path hint, raw fragment hint, forced query delimiter and omitted host.
Byte fields may contain arbitrary bytes; constructing a URL snapshots them.

[Source](/reference-sources/Text.Url.Model.cinch) · line 29

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlParts</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Userinfo</span>.<span class="tok-constructor">UrlUserinfo</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">UrlComponents</span>)))))))))))</code></pre>

<a id="type-55726c456e636f64696e67"></a>

## `UrlEncoding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UrlEncoding</span></code></pre>

The independent byte grammar of one URL component.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 25

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlPath</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlPathSegment</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlHost</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlZone</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlUserPassword</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlQueryComponent</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlFragment</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span></code></pre>

<a id="type-55726c4572726f72"></a>

## `UrlError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UrlError</span> (<span class="tok-variable">cause</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A typed cause and the original input for one named URL operation.

[Source](/reference-sources/Text.Url.Error.cinch) · line 158

<a id="type-55726c4661696c757265"></a>

## `UrlFailure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UrlFailure</span></code></pre>

An immutable failure class, component-relative byte offset and exact copied byte detail.

[Source](/reference-sources/Text.Url.Error.cinch) · line 52

<a id="type-55726c4661696c7572654b696e64"></a>

## `UrlFailureKind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UrlFailureKind</span></code></pre>

A precise class of malformed URL, percent escape or query input.

[Source](/reference-sources/Text.Url.Error.cinch) · line 23

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlInvalidEscape</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlInvalidHost</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlControlByte</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlMissingScheme</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlInvalidRequest</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlEmptyRequest</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlPathColon</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlInvalidPort</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlMissingBracket</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlInvalidIpLiteral</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlInvalidUserinfo</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlQuerySemicolon</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlTooManyParameters</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<a id="type-55726c55736572696e666f"></a>

## `UrlUserinfo`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UrlUserinfo</span></code></pre>

Private copied username and optional copied password; an empty password remains present.

[Source](/reference-sources/Text.Url.Userinfo.cinch) · line 12

<a id="type-55726c56616c756573"></a>

## `UrlValues`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UrlValues</span></code></pre>

An immutable query multimap with present-but-empty lists represented explicitly.

[Source](/reference-sources/Text.Url.Query.cinch) · line 39

# Values

<a id="value-75726c2d617070656e642d62696e617279"></a>

## `url-append-binary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-append-binary</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append serialized URL bytes to a copied prefix without mutating either input.

[Source](/reference-sources/Text.Url.Model.cinch) · line 232

<a id="value-75726c2d636c6f6e65"></a>

## `url-clone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-clone</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>)</code></pre>

Retain an independent immutable URL version without copying private buffers.

[Source](/reference-sources/Text.Url.Model.cinch) · line 54

<a id="value-75726c2d636f6d706f6e656e7473"></a>

## `url-components`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-components</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">UrlComponents</span>)</code></pre>

Return a fresh editable component snapshot without exposing the private owners.

[Source](/reference-sources/Text.Url.Model.cinch) · line 44

<a id="value-75726c2d656d707479"></a>

## `url-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-empty</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span></code></pre>

Construct the empty relative reference.

[Source](/reference-sources/Text.Url.Model.cinch) · line 50

<a id="value-75726c2d6572726f72"></a>

## `url-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-error</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">cause</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-variable">cause</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-variable">cause</span>))))))</code></pre>

Wrap any typed cause with copied input and caller-supplied temporary/timeout flags.

[Source](/reference-sources/Text.Url.Error.cinch) · line 163

<a id="value-75726c2d6572726f722d6361757365"></a>

## `url-error-cause`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-error-cause</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">cause</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-variable">cause</span>) -&gt; <span class="tok-variable">cause</span>)</code></pre>

Unwrap the original typed cause without erasing its type or information.

[Source](/reference-sources/Text.Url.Error.cinch) · line 175

<a id="value-75726c2d6572726f722d696e707574"></a>

## `url-error-input`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-error-input</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">cause</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-variable">cause</span>) -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return a fresh copy of the original operation input, without requiring valid UTF-8.

[Source](/reference-sources/Text.Url.Error.cinch) · line 171

<a id="value-75726c2d6572726f722d6d657373616765"></a>

## `url-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-error-message</span> :: ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a URL parsing or escaping context with its structured built-in failure.

[Source](/reference-sources/Text.Url.Error.cinch) · line 193

<a id="value-75726c2d6572726f722d6f7065726174696f6e"></a>

## `url-error-operation`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-error-operation</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">cause</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-variable">cause</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Observe the operation name associated with the failure.

[Source](/reference-sources/Text.Url.Error.cinch) · line 167

<a id="value-75726c2d6572726f722d72656e646572"></a>

## `url-error-render`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-error-render</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">cause</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">cause</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-variable">cause</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Render any typed contextual error using its supplied cause formatter.

[Source](/reference-sources/Text.Url.Error.cinch) · line 187

<a id="value-75726c2d6572726f722d74656d706f72617279"></a>

## `url-error-temporary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-error-temporary</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">cause</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-variable">cause</span>) -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Observe explicit temporary classification; pure parsing failures are not temporary.

[Source](/reference-sources/Text.Url.Error.cinch) · line 179

<a id="value-75726c2d6572726f722d74696d656f7574"></a>

## `url-error-timeout`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-error-timeout</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">cause</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-variable">cause</span>) -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Observe explicit timeout classification; pure parsing failures do not time out.

[Source](/reference-sources/Text.Url.Error.cinch) · line 183

<a id="value-75726c2d657363617065"></a>

## `url-escape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-escape</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Encode one complete component into fresh bytes, preserving no caller aliases.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 276

<a id="value-75726c2d657363617065642d667261676d656e74"></a>

## `url-escaped-fragment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-escaped-fragment</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Choose a valid raw fragment spelling only when it decodes to the stored fragment.

[Source](/reference-sources/Text.Url.Model.cinch) · line 111

<a id="value-75726c2d657363617065642d6c656e677468"></a>

## `url-escaped-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-escaped-length</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Nat</span>))</code></pre>

Exact percent-encoded byte count; query-space substitution does not change length.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 187

<a id="value-75726c2d657363617065642d70617468"></a>

## `url-escaped-path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-escaped-path</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Choose a valid raw path spelling only when it decodes to the stored path.

[Source](/reference-sources/Text.Url.Model.cinch) · line 102

<a id="value-75726c2d6661696c757265"></a>

## `url-failure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-failure</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span>)))</code></pre>

Construct a byte-precise failure without losing non-UTF-8 detail.

[Source](/reference-sources/Text.Url.Error.cinch) · line 62

<a id="value-75726c2d6661696c7572652d64657461696c"></a>

## `url-failure-detail`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-failure-detail</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return a fresh copy of the exact offending bytes, including non-UTF-8 bytes.

[Source](/reference-sources/Text.Url.Error.cinch) · line 74

<a id="value-75726c2d6661696c7572652d6b696e64"></a>

## `url-failure-kind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-failure-kind</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span>)</code></pre>

Observe the named failure class.

[Source](/reference-sources/Text.Url.Error.cinch) · line 66

<a id="value-75726c2d6661696c7572652d6d657373616765"></a>

## `url-failure-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-failure-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a named failure while retaining its raw detail separately for programmatic handling.

[Source](/reference-sources/Text.Url.Error.cinch) · line 140

<a id="value-75726c2d6661696c7572652d6f6666736574"></a>

## `url-failure-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-failure-offset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe the zero-based byte position within the component being processed.

[Source](/reference-sources/Text.Url.Error.cinch) · line 70

<a id="value-75726c2d6661696c7572652d7368696674"></a>

## `url-failure-shift`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-failure-shift</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span>))</code></pre>

Rebase a component-relative failure into a larger original byte interval.

[Source](/reference-sources/Text.Url.Error.cinch) · line 78

<a id="value-75726c2d666f7263652d7175657279"></a>

## `url-force-query`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-force-query</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Observe force query without exposing mutable storage.

[Source](/reference-sources/Text.Url.Model.cinch) · line 94

<a id="value-75726c2d667261676d656e74"></a>

## `url-fragment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-fragment</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe fragment as a fresh byte buffer.

[Source](/reference-sources/Text.Url.Model.cinch) · line 78

<a id="value-75726c2d66726f6d2d636f6d706f6e656e7473"></a>

## `url-from-components`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-from-components</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">UrlComponents</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>)</code></pre>

Snapshot every externally mutable byte field in a component bundle.

[Source](/reference-sources/Text.Url.Model.cinch) · line 39

<a id="value-75726c2d686f7374"></a>

## `url-host`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-host</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe host as a fresh byte buffer.

[Source](/reference-sources/Text.Url.Model.cinch) · line 70

<a id="value-75726c2d686f73746e616d65"></a>

## `url-hostname`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-hostname</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return host bytes without a valid optional port and without enclosing IP brackets.

[Source](/reference-sources/Text.Url.Model.cinch) · line 198

<a id="value-75726c2d69732d6162736f6c757465"></a>

## `url-is-absolute`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-is-absolute</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

A URL is absolute precisely when its scheme is nonempty.

[Source](/reference-sources/Text.Url.Model.cinch) · line 183

<a id="value-75726c2d6a6f696e"></a>

## `url-join`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-join</span> :: (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span>) <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>)))</code></pre>

Parse a base and join escaped elements, wrapping join failures with the original base bytes.

[Source](/reference-sources/Text.Url.Resolve.cinch) · line 132

<a id="value-75726c2d6a6f696e2d70617468"></a>

## `url-join-path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-join-path</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>))</code></pre>

Join escaped paths; malformed percent escapes retain the original immutable URL value.

[Source](/reference-sources/Text.Url.Resolve.cinch) · line 128

<a id="value-75726c2d6a6f696e2d706174682d636865636b6564"></a>

## `url-join-path-checked`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-join-path-checked</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>)))</code></pre>

Join escaped path elements with lexical separator/dot cleaning and report malformed escapes.
Traversal above the root is clamped; the base path's absolute/relative form is preserved.

[Source](/reference-sources/Text.Url.Resolve.cinch) · line 116

<a id="value-75726c2d6d61727368616c2d62696e617279"></a>

## `url-marshal-binary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-marshal-binary</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Serialize a URL to its fresh byte representation.

[Source](/reference-sources/Text.Url.Model.cinch) · line 228

<a id="value-75726c2d6f6d69742d686f7374"></a>

## `url-omit-host`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-omit-host</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Observe omit host without exposing mutable storage.

[Source](/reference-sources/Text.Url.Model.cinch) · line 98

<a id="value-75726c2d6f7061717565"></a>

## `url-opaque`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-opaque</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe opaque as a fresh byte buffer.

[Source](/reference-sources/Text.Url.Model.cinch) · line 62

<a id="value-75726c2d7061727365"></a>

## `url-parse`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-parse</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span>) <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>))</code></pre>

Parse an arbitrary byte URL/reference; fragment failures retain full original input context.

[Source](/reference-sources/Text.Url.Parser.cinch) · line 162

<a id="value-75726c2d70617273652d7175657279"></a>

## `url-parse-query`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-parse-query</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span>)))</code></pre>

Parse a query with a maximum of ten thousand raw ampersand-separated fields.

[Source](/reference-sources/Text.Url.Query.cinch) · line 255

<a id="value-75726c2d70617273652d71756572792d6c696d6974"></a>

## `url-parse-query-limit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-parse-query-limit</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span>))))</code></pre>

Parse query bytes with an inclusive field limit; zero disables the bound.
Valid fields survive malformed escapes and raw-semicolon fields. Excess raw
fields reject the complete query before decoding; offsets address original bytes.

[Source](/reference-sources/Text.Url.Query.cinch) · line 248

<a id="value-75726c2d70617273652d7265666572656e6365"></a>

## `url-parse-reference`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-parse-reference</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span>) <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>)))</code></pre>

Parse and resolve reference bytes, retaining a typed parse failure and its exact input.

[Source](/reference-sources/Text.Url.Resolve.cinch) · line 101

<a id="value-75726c2d70617273652d726571756573742d757269"></a>

## `url-parse-request-uri`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-parse-request-uri</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span>) <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>))</code></pre>

Parse an absolute request URL, absolute path or asterisk without treating hash as a fragment delimiter.

[Source](/reference-sources/Text.Url.Parser.cinch) · line 176

<a id="value-75726c2d70617273652d74657874"></a>

## `url-parse-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-parse-text</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span>) <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>))</code></pre>

Parse a UTF-8 URL while retaining arbitrary percent-decoded field bytes.

[Source](/reference-sources/Text.Url.Parser.cinch) · line 172

<a id="value-75726c2d70617373776f7264"></a>

## `url-password`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-password</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Userinfo</span>.<span class="tok-constructor">UrlUserinfo</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Return a copied optional password, distinguishing absence from an empty password.

[Source](/reference-sources/Text.Url.Userinfo.cinch) · line 29

<a id="value-75726c2d70617468"></a>

## `url-path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-path</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe path as a fresh byte buffer.

[Source](/reference-sources/Text.Url.Model.cinch) · line 74

<a id="value-75726c2d706174682d657363617065"></a>

## `url-path-escape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-path-escape</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Escape one arbitrary-byte path segment, including every slash inside the segment.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 476

<a id="value-75726c2d706174682d6573636170652d74657874"></a>

## `url-path-escape-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-path-escape-text</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Escape UTF-8 text as an ASCII path segment; decoding remains byte based.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 492

<a id="value-75726c2d706174682d756e657363617065"></a>

## `url-path-unescape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-path-unescape</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Percent-decode a path component without treating plus as a space.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 480

<a id="value-75726c2d706f7274"></a>

## `url-port`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-port</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return lexical decimal port bytes, including an empty result for an absent or empty port.

[Source](/reference-sources/Text.Url.Model.cinch) · line 205

<a id="value-75726c2d7175657279"></a>

## `url-query`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-query</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span>)</code></pre>

Parse raw query fields with the default bound, discarding malformed pairs and their diagnostics.

[Source](/reference-sources/Text.Url.Model.cinch) · line 209

<a id="value-75726c2d71756572792d657363617065"></a>

## `url-query-escape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-query-escape</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Escape one arbitrary-byte query key or value, using plus for space.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 484

<a id="value-75726c2d71756572792d6573636170652d74657874"></a>

## `url-query-escape-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-query-escape-text</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Escape UTF-8 text as an ASCII query component; decoding remains byte based.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 496

<a id="value-75726c2d71756572792d756e657363617065"></a>

## `url-query-unescape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-query-unescape</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Decode a query key or value, mapping plus to space and retaining arbitrary decoded bytes.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 488

<a id="value-75726c2d71756f74652d6279746573"></a>

## `url-quote-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-quote-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Produce an ASCII byte-exact quoted diagnostic; non-ASCII bytes use hexadecimal escapes.

[Source](/reference-sources/Text.Url.Error.cinch) · line 129

<a id="value-75726c2d7261772d667261676d656e74"></a>

## `url-raw-fragment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-raw-fragment</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe raw fragment as a fresh byte buffer.

[Source](/reference-sources/Text.Url.Model.cinch) · line 90

<a id="value-75726c2d7261772d70617468"></a>

## `url-raw-path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-raw-path</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe raw path as a fresh byte buffer.

[Source](/reference-sources/Text.Url.Model.cinch) · line 86

<a id="value-75726c2d7261772d7175657279"></a>

## `url-raw-query`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-raw-query</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe raw query as a fresh byte buffer.

[Source](/reference-sources/Text.Url.Model.cinch) · line 82

<a id="value-75726c2d7265646163746564"></a>

## `url-redacted`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-redacted</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return fresh URL bytes with a present password replaced by five lowercase x bytes.

[Source](/reference-sources/Text.Url.Model.cinch) · line 213

<a id="value-75726c2d726571756573742d757269"></a>

## `url-request-uri`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-request-uri</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Render a request target, excluding fragment and using slash for an empty hierarchical path.

[Source](/reference-sources/Text.Url.Model.cinch) · line 220

<a id="value-75726c2d7265736f6c76652d7265666572656e6365"></a>

## `url-resolve-reference`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-resolve-reference</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>))</code></pre>

Resolve an already parsed reference against a base with independent immutable result ownership.

[Source](/reference-sources/Text.Url.Resolve.cinch) · line 81

<a id="value-75726c2d736368656d65"></a>

## `url-scheme`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-scheme</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe scheme as a fresh byte buffer.

[Source](/reference-sources/Text.Url.Model.cinch) · line 58

<a id="value-75726c2d746f2d6279746573"></a>

## `url-to-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-to-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Reassemble fresh URL bytes, preserving raw query and opaque bytes exactly.

[Source](/reference-sources/Text.Url.Model.cinch) · line 164

<a id="value-75726c2d746f2d74657874"></a>

## `url-to-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-to-text</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Decode rendered bytes as UTF-8 only when every preserved byte is valid text.

[Source](/reference-sources/Text.Url.Model.cinch) · line 179

<a id="value-75726c2d756e657363617065"></a>

## `url-unescape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-unescape</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Decode a complete component to fresh arbitrary bytes, or return a precise failure without partial output.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 411

<a id="value-75726c2d756e6573636170652d667261676d656e742d68696e74"></a>

## `url-unescape-fragment-hint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-unescape-fragment-hint</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Decode a raw fragment hint only when its pchar spellings and percent triplets are valid.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 472

<a id="value-75726c2d756e6573636170652d706174682d68696e74"></a>

## `url-unescape-path-hint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-unescape-path-hint</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Decode a raw path hint only when its pchar spellings and percent triplets are valid.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 468

<a id="value-75726c2d756e6d61727368616c2d62696e617279"></a>

## `url-unmarshal-binary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-unmarshal-binary</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span>) <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>))</code></pre>

Decode the byte serialization by parsing a fresh immutable URL value.

[Source](/reference-sources/Text.Url.Parser.cinch) · line 180

<a id="value-75726c2d75736572"></a>

## `url-user`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-user</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Userinfo</span>.<span class="tok-constructor">UrlUserinfo</span>)</code></pre>

Construct user information without a password, copying arbitrary username bytes.

[Source](/reference-sources/Text.Url.Userinfo.cinch) · line 17

<a id="value-75726c2d757365722d70617373776f7264"></a>

## `url-user-password`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-user-password</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Userinfo</span>.<span class="tok-constructor">UrlUserinfo</span>))</code></pre>

Construct user information with a present password, copying both byte buffers.

[Source](/reference-sources/Text.Url.Userinfo.cinch) · line 21

<a id="value-75726c2d75736572696e666f"></a>

## `url-userinfo`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-userinfo</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Userinfo</span>.<span class="tok-constructor">UrlUserinfo</span>))</code></pre>

Observe user without exposing mutable storage.

[Source](/reference-sources/Text.Url.Model.cinch) · line 66

<a id="value-75726c2d75736572696e666f2d6279746573"></a>

## `url-userinfo-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-userinfo-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Userinfo</span>.<span class="tok-constructor">UrlUserinfo</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Escape the username and optional colon-separated password to fresh ASCII bytes.

[Source](/reference-sources/Text.Url.Userinfo.cinch) · line 33

<a id="value-75726c2d757365726e616d65"></a>

## `url-username`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-username</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Userinfo</span>.<span class="tok-constructor">UrlUserinfo</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return fresh username bytes without decoding them as text.

[Source](/reference-sources/Text.Url.Userinfo.cinch) · line 25

<a id="value-75726c2d76616c7565732d616464"></a>

## `url-values-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-values-add</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span>)))</code></pre>

Append a copied value to a key and return a new multimap version.

[Source](/reference-sources/Text.Url.Query.cinch) · line 137

<a id="value-75726c2d76616c7565732d636c6f6e65"></a>

## `url-values-clone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-values-clone</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span>)</code></pre>

Retain an independent immutable version with no externally mutable aliases.

[Source](/reference-sources/Text.Url.Query.cinch) · line 133

<a id="value-75726c2d76616c7565732d64656c657465"></a>

## `url-values-delete`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-values-delete</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span>))</code></pre>

Remove a key and all its values while retaining earlier versions unchanged.

[Source](/reference-sources/Text.Url.Query.cinch) · line 153

<a id="value-75726c2d76616c7565732d656d707479"></a>

## `url-values-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-values-empty</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span></code></pre>

Construct an empty immutable query multimap.

[Source](/reference-sources/Text.Url.Query.cinch) · line 125

<a id="value-75726c2d76616c7565732d656e636f6465"></a>

## `url-values-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-values-encode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Render sorted keys and insertion-ordered values as ASCII query bytes, omitting empty lists.

[Source](/reference-sources/Text.Url.Query.cinch) · line 213

<a id="value-75726c2d76616c7565732d66726f6d2d7061697273"></a>

## `url-values-from-pairs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-values-from-pairs</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>))) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span>)</code></pre>

Snapshot pairs in order, replacing earlier lists when a key is repeated.

[Source](/reference-sources/Text.Url.Query.cinch) · line 187

<a id="value-75726c2d76616c7565732d676574"></a>

## `url-values-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-values-get</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Return a fresh first value, or fresh empty bytes for absent or empty-list keys.

[Source](/reference-sources/Text.Url.Query.cinch) · line 161

<a id="value-75726c2d76616c7565732d6765742d616c6c"></a>

## `url-values-get-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-values-get-all</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Return fresh value buffers in insertion order; absent and empty-list keys return an empty list.

[Source](/reference-sources/Text.Url.Query.cinch) · line 168

<a id="value-75726c2d76616c7565732d686173"></a>

## `url-values-has`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-values-has</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Test key presence independently of whether its value list is empty.

[Source](/reference-sources/Text.Url.Query.cinch) · line 157

<a id="value-75726c2d76616c7565732d7061697273"></a>

## `url-values-pairs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-values-pairs</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Return keys in raw-byte order and copied value lists in insertion order.

[Source](/reference-sources/Text.Url.Query.cinch) · line 183

<a id="value-75726c2d76616c7565732d736574"></a>

## `url-values-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-values-set</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span>)))</code></pre>

Replace a key's values with one copied value.

[Source](/reference-sources/Text.Url.Query.cinch) · line 143

<a id="value-75726c2d76616c7565732d7365742d616c6c"></a>

## `url-values-set-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-values-set-all</span> :: (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span>)))</code></pre>

Replace a key with a copied list; an empty list keeps the key present but emits no encoded pair.

[Source](/reference-sources/Text.Url.Query.cinch) · line 149

<a id="value-75726c2d76616c7565732d73697a65"></a>

## `url-values-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-values-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Count distinct keys, including keys with an empty value list.

[Source](/reference-sources/Text.Url.Query.cinch) · line 129

<a id="value-75726c2d776974682d667261676d656e74"></a>

## `url-with-fragment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-with-fragment</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>)))</code></pre>

Set an escaped fragment, preserving a noncanonical raw hint and decoded bytes.

[Source](/reference-sources/Text.Url.Model.cinch) · line 128

<a id="value-75726c2d776974682d70617468"></a>

## `url-with-path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-with-path</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>)))</code></pre>

Set an escaped path on a copied field snapshot, preserving a noncanonical raw hint.

[Source](/reference-sources/Text.Url.Model.cinch) · line 120

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Url.cinch`. Source SHA-256: `ace795fa48c2bfdab5f1c984b072eff6ad2ca68c11403d965ad610937b45001c`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
