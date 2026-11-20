# Text.Url.Resolve

URL reference resolution, lexical path joining and copied persistent URL updates.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Url.Resolve`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Url.Resolve.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Parser](Text.Url.Parser.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

Non-embedded source closure: [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Ip](Text.Url.Ip.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Parser](Text.Url.Parser.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Resolve](Text.Url.Resolve.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Byte-preserving reference resolution and lexical URL path joining.
Resolution preserves repeated separators and encoded dot bytes. Joining cleans
separators and literal dot segments; elements are already escaped URL paths.

</details>

# Types and constructors

This module exports no type declarations.

# Values

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

<a id="value-75726c2d70617273652d7265666572656e6365"></a>

## `url-parse-reference`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-parse-reference</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span>) <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>)))</code></pre>

Parse and resolve reference bytes, retaining a typed parse failure and its exact input.

[Source](/reference-sources/Text.Url.Resolve.cinch) · line 101

<a id="value-75726c2d7265736f6c76652d7265666572656e6365"></a>

## `url-resolve-reference`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-resolve-reference</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>))</code></pre>

Resolve an already parsed reference against a base with independent immutable result ownership.

[Source](/reference-sources/Text.Url.Resolve.cinch) · line 81

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Url/Resolve.cinch`. Source SHA-256: `5fd9ec9bddbd778d81d392e1e0e18b6be2f037cbd0f1b15fe1811e4eaf91613e`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
