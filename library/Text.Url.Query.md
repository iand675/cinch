# Text.Url.Query

Persistent byte-preserving query multimaps with deterministic encoding, checked parsing and copied observations.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Url.Query`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Url.Query.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Scan](Text.Url.Scan.md).

Non-embedded source closure: [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Scan](Text.Url.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Persistent, byte-preserving URL query multimaps and bounded form decoding.
Keys are ordered lexicographically by raw bytes; values preserve insertion
order. Parsing retains valid pairs after malformed fields and reports one
precise failure. Raw semicolons reject their field and take diagnostic
precedence. The default limit counts raw ampersand-separated fields, including
empty fields; a zero explicit limit disables it. All external byte buffers are
copied. A balanced tree bounds lookup and updates even for sorted input.

</details>

# Types and constructors

<a id="type-55726c56616c756573"></a>

## `UrlValues`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UrlValues</span></code></pre>

An immutable query multimap with present-but-empty lists represented explicitly.

[Source](/reference-sources/Text.Url.Query.cinch) · line 39

# Values

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

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Url/Query.cinch`. Source SHA-256: `d21bfb7ca2d495f7e67202e8cec6ad27723f767d2d997a5afe9f1b07409e85d3`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
