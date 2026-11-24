# Data.CookieJar

An RFC 6265 cookie jar as an immutable value: store the cookies responses set and choose the cookies a request sends.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.CookieJar`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.CookieJar.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Cookie](Text.Cookie.md), [Text.Punycode](Text.Punycode.md), [Text.Url](Text.Url.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.CookieJar](Data.CookieJar.md), [Text.Cookie](Text.Cookie.md), [Text.Punycode](Text.Punycode.md), [Text.Url](Text.Url.md), [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Ip](Text.Url.Ip.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Parser](Text.Url.Parser.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Resolve](Text.Url.Resolve.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

<details><summary>Behaviour and boundary notes from the source</summary>

A cookie jar (RFC 6265 storage model): it stores the cookies servers set and answers which to send with
a request.

Cookies are grouped by the request host's registrable domain: one label more than the public suffix when
a public suffix function is given, otherwise the last two labels (IP addresses group by themselves). A
Domain attribute must cover the request host and, with a public suffix function, must not be a public
suffix itself (unless it equals the host, which makes the cookie host-only). A cookie without a Domain is
host-only. Paths default to the request path's directory; Max-Age wins over Expires; a cookie whose
Max-Age is negative or whose Expires has passed deletes the stored cookie with the same domain, path and
name. Session cookies never expire. Hosts are lower-cased, lose a trailing dot and any port, and have
non-ASCII labels Punycode-encoded.

A jar is an immutable value: setting cookies returns the next jar, and reading cookies returns them (longest
path first by text order, then oldest, then first set) with the jar that has dropped expired cookies and
recorded the access. Times are Unix seconds and are passed in, so the jar is deterministic.

</details>

# Types and constructors

<a id="type-436f6f6b69654a6172"></a>

## `CookieJar`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CookieJar</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Data.CookieJar.cinch) · line 38

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CookieJar</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">CookieJar</span>.<span class="tok-constructor">Entry</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">CookieJar</span>.<span class="tok-constructor">CookieJar</span>)))</code></pre>

# Values

<a id="value-636f6f6b69652d6a61722d6e6577"></a>

## `cookie-jar-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-jar-new</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">CookieJar</span>.<span class="tok-constructor">CookieJar</span>)</code></pre>

An empty jar, optionally with a public suffix function (domain to its public suffix).

[Source](/reference-sources/Data.CookieJar.cinch) · line 41

<a id="value-6a61722d636f6f6b696573"></a>

## `jar-cookies`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jar-cookies</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">CookieJar</span>.<span class="tok-constructor">CookieJar</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span>), <span class="tok-constructor">Data</span>.<span class="tok-constructor">CookieJar</span>.<span class="tok-constructor">CookieJar</span>))))))</code></pre>

The cookies to send with a request to scheme://host/path at a time, and the jar after dropping expired
cookies and recording the access.

[Source](/reference-sources/Data.CookieJar.cinch) · line 339

<a id="value-6a61722d636f6f6b6965732d75726c"></a>

## `jar-cookies-url`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jar-cookies-url</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">CookieJar</span>.<span class="tok-constructor">CookieJar</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span>), <span class="tok-constructor">Data</span>.<span class="tok-constructor">CookieJar</span>.<span class="tok-constructor">CookieJar</span>))))</code></pre>

The cookies to send with a request to a URL at a time, and the next jar.

[Source](/reference-sources/Data.CookieJar.cinch) · line 415

<a id="value-6a61722d7365742d636f6f6b696573"></a>

## `jar-set-cookies`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jar-set-cookies</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">CookieJar</span>.<span class="tok-constructor">CookieJar</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span>) -&gt; (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">CookieJar</span>.<span class="tok-constructor">CookieJar</span>))))))</code></pre>

Store the cookies a response to scheme://host/path set at a time.

[Source](/reference-sources/Data.CookieJar.cinch) · line 266

<a id="value-6a61722d7365742d636f6f6b6965732d75726c"></a>

## `jar-set-cookies-url`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jar-set-cookies-url</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">CookieJar</span>.<span class="tok-constructor">CookieJar</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span>) -&gt; (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">CookieJar</span>.<span class="tok-constructor">CookieJar</span>))))</code></pre>

Store the cookies a response from a URL set at a time.

[Source](/reference-sources/Data.CookieJar.cinch) · line 411

<a id="value-6a61722d73697a65"></a>

## `jar-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jar-size</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">CookieJar</span>.<span class="tok-constructor">CookieJar</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

How many cookies the jar holds.

[Source](/reference-sources/Data.CookieJar.cinch) · line 45

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/CookieJar.cinch`. Source SHA-256: `a19367e858100095283000a643f73df3673a96b128c0298156d85504f2bbd716`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
