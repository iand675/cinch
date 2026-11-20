# Text.Url.Userinfo

Immutable copied URL user information with explicit password presence and checked percent encoding.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Url.Userinfo`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Url.Userinfo.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Scan](Text.Url.Scan.md).

Non-embedded source closure: [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Immutable byte-preserving URL user information with explicit password presence.

</details>

# Types and constructors

<a id="type-55726c55736572696e666f"></a>

## `UrlUserinfo`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UrlUserinfo</span></code></pre>

Private copied username and optional copied password; an empty password remains present.

[Source](/reference-sources/Text.Url.Userinfo.cinch) · line 12

# Values

<a id="value-75726c2d70617373776f7264"></a>

## `url-password`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-password</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Userinfo</span>.<span class="tok-constructor">UrlUserinfo</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Return a copied optional password, distinguishing absence from an empty password.

[Source](/reference-sources/Text.Url.Userinfo.cinch) · line 29

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

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Url/Userinfo.cinch`. Source SHA-256: `cccdb3acac6ac60383d2a2f8ce9fd9fde4953de7f28aa8259a5d5436ec52758a`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
