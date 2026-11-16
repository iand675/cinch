# Text.Url.Parser

Checked general and request URL parsing with precise scheme, host, port and escape rules.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Url.Parser`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Url.Parser.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Ip](Text.Url.Ip.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

Non-embedded source closure: [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Ip](Text.Url.Ip.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Parser](Text.Url.Parser.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Byte-preserving hierarchical and opaque URL parsing with precise typed failures.
Raw queries and opaque bodies are preserved without interpreting percent signs.
Decoded user, host, path and fragment bytes retain non-UTF-8 data. HTTP and HTTPS
authorities reject unbracketed multiple-colon hosts; other schemes retain the
lexical final-port convention used by multi-host connection strings.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-75726c2d7061727365"></a>

## `url-parse`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-parse</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span>) <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>))</code></pre>

Parse an arbitrary byte URL/reference; fragment failures retain full original input context.

[Source](/reference-sources/Text.Url.Parser.cinch) · line 162

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

<a id="value-75726c2d756e6d61727368616c2d62696e617279"></a>

## `url-unmarshal-binary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-unmarshal-binary</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span>) <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>))</code></pre>

Decode the byte serialization by parsing a fresh immutable URL value.

[Source](/reference-sources/Text.Url.Parser.cinch) · line 180

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Url/Parser.cinch`. Source SHA-256: `d880177f125cbdf7d9546cb5526ce6f6717ead1c719b180724745be6ef1c4f7d`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
