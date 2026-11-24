# Text.Cookie

HTTP cookies: parse Set-Cookie and Cookie header lines, serialize cookies as Set-Cookie values and validate them.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Cookie`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Cookie.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Text.Cookie](Text.Cookie.md).

<details><summary>Behaviour and boundary notes from the source</summary>

HTTP cookies (RFC 6265): parsing Set-Cookie and Cookie header lines, serializing a cookie as a
Set-Cookie value, and validating cookies.

A cookie's name must be an HTTP token. Its value may be double-quoted (the quotes are remembered and
restored) and may hold any printable ASCII except double quote, semicolon and backslash. Set-Cookie
attributes are matched case-insensitively; values that are not valid cookie values, unknown attributes
and unreadable Max-Age or Expires values are kept as unparsed text. Max-Age zero or negative means delete
now (recorded as -1). Expires accepts "Mon, 02 Jan 2006 15:04:05 GMT" and the hyphenated
"Mon, 02-Jan-2006 15:04:05 GMT" forms; the zone must be well formed but the clock reading counts as UTC.
Times are Unix seconds. Serialization drops bytes that cannot appear in a value or path, drops an invalid
Domain, and writes Expires only for years from 1601.

</details>

# Types and constructors

<a id="type-436f6f6b6965"></a>

## `Cookie`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Cookie</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Text.Cookie.cinch) · line 52

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Cookie</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">SameSite</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span>))))))))))))))</code></pre>

<a id="type-436f6f6b69654572726f72"></a>

## `CookieError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CookieError</span></code></pre>

Why a cookie line or cookie was refused.

[Source](/reference-sources/Text.Cookie.cinch) · line 115

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CookieBlank</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">CookieError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CookieNoEquals</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">CookieError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CookieBadName</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">CookieError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CookieBadValue</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">CookieError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CookieInvalid</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">CookieError</span>)</code></pre>

<a id="type-53616d6553697465"></a>

## `SameSite`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SameSite</span></code></pre>

The SameSite attribute.

[Source](/reference-sources/Text.Cookie.cinch) · line 35

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SameSiteUnset</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">SameSite</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SameSiteDefault</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">SameSite</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SameSiteLax</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">SameSite</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SameSiteStrict</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">SameSite</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SameSiteNone</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">SameSite</span></code></pre>

# Values

<a id="value-636f6f6b69652d61736369692d6c6f776572"></a>

## `cookie-ascii-lower`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-ascii-lower</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

ASCII lower case, or None when the text has a non-ASCII byte.

[Source](/reference-sources/Text.Cookie.cinch) · line 202

<a id="value-636f6f6b69652d646f6d61696e"></a>

## `cookie-domain`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-domain</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The Domain attribute.

[Source](/reference-sources/Text.Cookie.cinch) · line 75

<a id="value-636f6f6b69652d646f6d61696e2d76616c6964"></a>

## `cookie-domain-valid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-domain-valid</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether a domain is a valid cookie domain: a host name (letters, digits, hyphens, labels of 1 to 63 bytes,
at least one letter, at most 255 bytes, an optional leading dot) or an IPv4 address.

[Source](/reference-sources/Text.Cookie.cinch) · line 565

<a id="value-636f6f6b69652d6572726f722d6d657373616765"></a>

## `cookie-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">CookieError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

A readable description of an error.

[Source](/reference-sources/Text.Cookie.cinch) · line 128

<a id="value-636f6f6b69652d65787069726573"></a>

## `cookie-expires`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-expires</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Int</span>))</code></pre>

The expiry in Unix seconds.

[Source](/reference-sources/Text.Cookie.cinch) · line 79

<a id="value-636f6f6b69652d687474702d6f6e6c79"></a>

## `cookie-http-only`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-http-only</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

The HttpOnly attribute.

[Source](/reference-sources/Text.Cookie.cinch) · line 95

<a id="value-636f6f6b69652d69732d69707634"></a>

## `cookie-is-ipv4`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-is-ipv4</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether text is a dotted-decimal IPv4 address (four fields of 0 to 255 without leading zeros).

[Source](/reference-sources/Text.Cookie.cinch) · line 589

<a id="value-636f6f6b69652d6d61782d616765"></a>

## `cookie-max-age`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-max-age</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span> -&gt; <span class="tok-constructor">Int</span>)</code></pre>

Max-Age: positive seconds, 0 for none, -1 for delete now.

[Source](/reference-sources/Text.Cookie.cinch) · line 87

<a id="value-636f6f6b69652d6e616d65"></a>

## `cookie-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-name</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The name.

[Source](/reference-sources/Text.Cookie.cinch) · line 59

<a id="value-636f6f6b69652d6e6577"></a>

## `cookie-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-new</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span>))</code></pre>

A cookie with only a name and value.

[Source](/reference-sources/Text.Cookie.cinch) · line 55

<a id="value-636f6f6b69652d7061727365"></a>

## `cookie-parse`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-parse</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">CookieError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span>)))</code></pre>

Parse a Cookie header line into its name-value pairs; any malformed pair refuses the line.

[Source](/reference-sources/Text.Cookie.cinch) · line 264

<a id="value-636f6f6b69652d70617273652d736574"></a>

## `cookie-parse-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-parse-set</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">CookieError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span>))</code></pre>

Parse a Set-Cookie header line.

[Source](/reference-sources/Text.Cookie.cinch) · line 305

<a id="value-636f6f6b69652d706172746974696f6e6564"></a>

## `cookie-partitioned`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-partitioned</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

The Partitioned attribute.

[Source](/reference-sources/Text.Cookie.cinch) · line 103

<a id="value-636f6f6b69652d70617468"></a>

## `cookie-path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-path</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The Path attribute.

[Source](/reference-sources/Text.Cookie.cinch) · line 71

<a id="value-636f6f6b69652d71756f746564"></a>

## `cookie-quoted`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-quoted</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether the value was quoted.

[Source](/reference-sources/Text.Cookie.cinch) · line 67

<a id="value-636f6f6b69652d726177"></a>

## `cookie-raw`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-raw</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The raw Set-Cookie line.

[Source](/reference-sources/Text.Cookie.cinch) · line 107

<a id="value-636f6f6b69652d7261772d65787069726573"></a>

## `cookie-raw-expires`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-raw-expires</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The raw Expires text.

[Source](/reference-sources/Text.Cookie.cinch) · line 83

<a id="value-636f6f6b69652d72656164"></a>

## `cookie-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-read</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span>)))</code></pre>

Read a Cookie header line leniently: malformed pairs are skipped; a non-empty filter keeps only that name.

[Source](/reference-sources/Text.Cookie.cinch) · line 287

<a id="value-636f6f6b69652d73616d652d73697465"></a>

## `cookie-same-site`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-same-site</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">SameSite</span>)</code></pre>

The SameSite attribute.

[Source](/reference-sources/Text.Cookie.cinch) · line 99

<a id="value-636f6f6b69652d736563757265"></a>

## `cookie-secure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-secure</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

The Secure attribute.

[Source](/reference-sources/Text.Cookie.cinch) · line 91

<a id="value-636f6f6b69652d737472696e67"></a>

## `cookie-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-string</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The cookie as a Set-Cookie value (empty when the name is invalid).

[Source](/reference-sources/Text.Cookie.cinch) · line 622

<a id="value-636f6f6b69652d74696d652d666f726d6174"></a>

## `cookie-time-format`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-time-format</span> :: (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Unix seconds as "Mon, 02 Jan 2006 15:04:05 GMT".

[Source](/reference-sources/Text.Cookie.cinch) · line 460

<a id="value-636f6f6b69652d74696d652d7061727365"></a>

## `cookie-time-parse`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-time-parse</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Int</span>))</code></pre>

Parse "Mon, 02 Jan 2006 15:04:05 MST" or "Mon, 02-Jan-2006 15:04:05 MST" to Unix seconds.

[Source](/reference-sources/Text.Cookie.cinch) · line 486

<a id="value-636f6f6b69652d756e706172736564"></a>

## `cookie-unparsed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-unparsed</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

The attributes that were not understood, in order.

[Source](/reference-sources/Text.Cookie.cinch) · line 111

<a id="value-636f6f6b69652d76616c6964"></a>

## `cookie-valid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-valid</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">CookieError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Check a cookie: a token name, an expiry from 1601 when set, value and path bytes, a valid domain, and Secure
when Partitioned.

[Source](/reference-sources/Text.Cookie.cinch) · line 664

<a id="value-636f6f6b69652d76616c7565"></a>

## `cookie-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cookie-value</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Cookie</span>.<span class="tok-constructor">Cookie</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The value.

[Source](/reference-sources/Text.Cookie.cinch) · line 63

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Cookie.cinch`. Source SHA-256: `30a3d34e01b8800167c59dad6112b180b6a1185f80e2ed3d5cc2270efd2af216`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
