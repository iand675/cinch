# Text.Url.Codec

Checked percent codecs for URL components, exact escaped sizing and optional UTF-8 text views.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Url.Codec`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Url.Codec.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Url.Error](Text.Url.Error.md).

Non-embedded source closure: [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Byte-preserving percent escaping for URL components.
Query components map spaces to plus and decode plus as space. Other contexts
preserve plus. Percent decoding validates the complete input before returning
fresh bytes; arbitrary decoded bytes never pass through a text conversion.
Host and zone policies enforce their additional escape restrictions. SIMD
classifiers skip complete ordinary spans; private exact-load loops retain
backing owners. Checked bulk copies write percent triplets and ordinary runs.

</details>

# Types and constructors

<a id="type-55726c456e636f64696e67"></a>

## `UrlEncoding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UrlEncoding</span></code></pre>

The independent byte grammar of one URL component.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 24

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlPath</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlPathSegment</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlHost</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlZone</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlUserPassword</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlQueryComponent</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlFragment</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span></code></pre>

# Values

<a id="value-75726c2d657363617065"></a>

## `url-escape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-escape</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Encode one complete component into fresh bytes, preserving no caller aliases.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 241

<a id="value-75726c2d657363617065642d6c656e677468"></a>

## `url-escaped-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-escaped-length</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Nat</span>))</code></pre>

Exact percent-encoded byte count; query-space substitution does not change length.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 178

<a id="value-75726c2d706174682d657363617065"></a>

## `url-path-escape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-path-escape</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Escape one arbitrary-byte path segment, including every slash inside the segment.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 423

<a id="value-75726c2d706174682d6573636170652d74657874"></a>

## `url-path-escape-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-path-escape-text</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Escape UTF-8 text as an ASCII path segment; decoding remains byte based.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 439

<a id="value-75726c2d706174682d756e657363617065"></a>

## `url-path-unescape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-path-unescape</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Percent-decode a path component without treating plus as a space.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 427

<a id="value-75726c2d71756572792d657363617065"></a>

## `url-query-escape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-query-escape</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Escape one arbitrary-byte query key or value, using plus for space.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 431

<a id="value-75726c2d71756572792d6573636170652d74657874"></a>

## `url-query-escape-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-query-escape-text</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Escape UTF-8 text as an ASCII query component; decoding remains byte based.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 443

<a id="value-75726c2d71756572792d756e657363617065"></a>

## `url-query-unescape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-query-unescape</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Decode a query key or value, mapping plus to space and retaining arbitrary decoded bytes.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 435

<a id="value-75726c2d756e657363617065"></a>

## `url-unescape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-unescape</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">UrlEncoding</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Decode a complete component to fresh arbitrary bytes, or return a precise failure without partial output.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 376

<a id="value-75726c2d756e6573636170652d667261676d656e742d68696e74"></a>

## `url-unescape-fragment-hint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-unescape-fragment-hint</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Decode a raw fragment hint only when its pchar spellings and percent triplets are valid.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 419

<a id="value-75726c2d756e6573636170652d706174682d68696e74"></a>

## `url-unescape-path-hint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-unescape-path-hint</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Decode a raw path hint only when its pchar spellings and percent triplets are valid.

[Source](/reference-sources/Text.Url.Codec.cinch) · line 415

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Url/Codec.cinch`. Source SHA-256: `17b17b7cd4e76ee256793a6167a3148d3a9aa9528164ade05aab8fee53f7b539`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
