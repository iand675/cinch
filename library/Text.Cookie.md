# Text.Cookie

HTTP cookies: parse Set-Cookie and Cookie header lines, serialize cookies as Set-Cookie values and validate them.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Cookie</code></div><div class="doc-mod-bar-links"><a href="/library/source/Text.Cookie/">View source</a><a href="/library/search/?q=%2BText.Cookie">Search module</a><a href="/reference-sources/Text.Cookie.cinch" download="Text.Cookie.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

<details class="doc-closure"><summary>Ship-source closure (2 files)</summary>

Copy these modules beside your entry file before importing `Text.Cookie`: [Data.Bytes.Build](Data.Bytes.Build.md), [Text.Cookie](Text.Cookie.md).

```sh
cp seed/prelude/Data/Bytes/Build.cinch ./Data.Bytes.Build.cinch
cp seed/prelude/Text/Cookie.cinch ./Text.Cookie.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis"><summary>Synopsis (29 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Cookie">Cookie</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Cookie</strong></code></li><li><a class="syn-name" href="#t-CookieError">CookieError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CookieError</strong></code></li><li><a class="syn-name" href="#t-SameSite">SameSite</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SameSite</strong></code></li><li><a class="syn-name" href="#v-cookie-ascii-lower">cookie-ascii-lower</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-ascii-lower</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-cookie-domain">cookie-domain</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-domain</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-cookie-domain-valid">cookie-domain-valid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-domain-valid</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-cookie-error-message">cookie-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-CookieError" title="Text.Cookie.CookieError">CookieError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-cookie-expires">cookie-expires</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-expires</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-cookie-http-only">cookie-http-only</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-http-only</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-cookie-is-ipv4">cookie-is-ipv4</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-is-ipv4</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-cookie-max-age">cookie-max-age</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-max-age</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-cookie-name">cookie-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-name</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-cookie-new">cookie-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-new</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a></code></li><li><a class="syn-name" href="#v-cookie-parse">cookie-parse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-parse</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-CookieError" title="Text.Cookie.CookieError">CookieError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a>)</code></li><li><a class="syn-name" href="#v-cookie-parse-set">cookie-parse-set</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-parse-set</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-CookieError" title="Text.Cookie.CookieError">CookieError</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a></code></li><li><a class="syn-name" href="#v-cookie-partitioned">cookie-partitioned</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-partitioned</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-cookie-path">cookie-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-path</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-cookie-quoted">cookie-quoted</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-quoted</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-cookie-raw">cookie-raw</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-raw</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-cookie-raw-expires">cookie-raw-expires</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-raw-expires</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-cookie-read">cookie-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-read</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a></code></li><li><a class="syn-name" href="#v-cookie-same-site">cookie-same-site</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-same-site</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-SameSite" title="Text.Cookie.SameSite">SameSite</a></code></li><li><a class="syn-name" href="#v-cookie-secure">cookie-secure</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-secure</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-cookie-string">cookie-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-cookie-time-format">cookie-time-format</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-time-format</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-cookie-time-parse">cookie-time-parse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-time-parse</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-cookie-unparsed">cookie-unparsed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-unparsed</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-cookie-valid">cookie-valid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-valid</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-CookieError" title="Text.Cookie.CookieError">CookieError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-cookie-value">cookie-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-value</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Cookie">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Cookie</strong></code><span class="doc-sig-links"><a href="#t-Cookie" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L52" title="View source at line 52">Source</a></span></div>
<div class="doc-body">

Fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Cookie"><code><strong class="tok-constructor">Cookie</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-SameSite" title="Text.Cookie.SameSite">SameSite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-CookieError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CookieError</strong></code><span class="doc-sig-links"><a href="#t-CookieError" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L115" title="View source at line 115">Source</a></span></div>
<div class="doc-body">

Why a cookie line or cookie was refused.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-CookieBlank"><code><strong class="tok-constructor">CookieBlank</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-CookieError" title="Text.Cookie.CookieError">CookieError</a></code></li><li id="k-CookieNoEquals"><code><strong class="tok-constructor">CookieNoEquals</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-CookieError" title="Text.Cookie.CookieError">CookieError</a></code></li><li id="k-CookieBadName"><code><strong class="tok-constructor">CookieBadName</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-CookieError" title="Text.Cookie.CookieError">CookieError</a></code></li><li id="k-CookieBadValue"><code><strong class="tok-constructor">CookieBadValue</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-CookieError" title="Text.Cookie.CookieError">CookieError</a></code></li><li id="k-CookieInvalid"><code><strong class="tok-constructor">CookieInvalid</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-CookieError" title="Text.Cookie.CookieError">CookieError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-SameSite">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SameSite</strong></code><span class="doc-sig-links"><a href="#t-SameSite" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L35" title="View source at line 35">Source</a></span></div>
<div class="doc-body">

The SameSite attribute.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SameSiteUnset"><code><strong class="tok-constructor">SameSiteUnset</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-SameSite" title="Text.Cookie.SameSite">SameSite</a></code></li><li id="k-SameSiteDefault"><code><strong class="tok-constructor">SameSiteDefault</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-SameSite" title="Text.Cookie.SameSite">SameSite</a></code></li><li id="k-SameSiteLax"><code><strong class="tok-constructor">SameSiteLax</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-SameSite" title="Text.Cookie.SameSite">SameSite</a></code></li><li id="k-SameSiteStrict"><code><strong class="tok-constructor">SameSiteStrict</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-SameSite" title="Text.Cookie.SameSite">SameSite</a></code></li><li id="k-SameSiteNone"><code><strong class="tok-constructor">SameSiteNone</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-SameSite" title="Text.Cookie.SameSite">SameSite</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-cookie-ascii-lower">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-ascii-lower</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-cookie-ascii-lower" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L202" title="View source at line 202">Source</a></span></div>
<div class="doc-body">

ASCII lower case, or None when the text has a non-ASCII byte.

</div>
</article>

<article class="doc-entry" id="v-cookie-domain">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-domain</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-cookie-domain" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L75" title="View source at line 75">Source</a></span></div>
<div class="doc-body">

The Domain attribute.

</div>
</article>

<article class="doc-entry" id="v-cookie-domain-valid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-domain-valid</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-cookie-domain-valid" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L565" title="View source at line 565">Source</a></span></div>
<div class="doc-body">

Whether a domain is a valid cookie domain: a host name (letters, digits, hyphens, labels of 1 to 63 bytes,
at least one letter, at most 255 bytes, an optional leading dot) or an IPv4 address.

</div>
</article>

<article class="doc-entry" id="v-cookie-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-CookieError" title="Text.Cookie.CookieError">CookieError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-cookie-error-message" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L128" title="View source at line 128">Source</a></span></div>
<div class="doc-body">

A readable description of an error.

</div>
</article>

<article class="doc-entry" id="v-cookie-expires">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-expires</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-cookie-expires" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L79" title="View source at line 79">Source</a></span></div>
<div class="doc-body">

The expiry in Unix seconds.

</div>
</article>

<article class="doc-entry" id="v-cookie-http-only">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-http-only</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-cookie-http-only" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L95" title="View source at line 95">Source</a></span></div>
<div class="doc-body">

The HttpOnly attribute.

</div>
</article>

<article class="doc-entry" id="v-cookie-is-ipv4">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-is-ipv4</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-cookie-is-ipv4" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L589" title="View source at line 589">Source</a></span></div>
<div class="doc-body">

Whether text is a dotted-decimal IPv4 address (four fields of 0 to 255 without leading zeros).

</div>
</article>

<article class="doc-entry" id="v-cookie-max-age">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-max-age</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-cookie-max-age" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L87" title="View source at line 87">Source</a></span></div>
<div class="doc-body">

Max-Age: positive seconds, 0 for none, -1 for delete now.

</div>
</article>

<article class="doc-entry" id="v-cookie-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-name</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-cookie-name" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L59" title="View source at line 59">Source</a></span></div>
<div class="doc-body">

The name.

</div>
</article>

<article class="doc-entry" id="v-cookie-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-new</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a></code><span class="doc-sig-links"><a href="#v-cookie-new" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L55" title="View source at line 55">Source</a></span></div>
<div class="doc-body">

A cookie with only a name and value.

</div>
</article>

<article class="doc-entry" id="v-cookie-parse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-parse</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-CookieError" title="Text.Cookie.CookieError">CookieError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a>)</code><span class="doc-sig-links"><a href="#v-cookie-parse" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L264" title="View source at line 264">Source</a></span></div>
<div class="doc-body">

Parse a Cookie header line into its name-value pairs; any malformed pair refuses the line.

</div>
</article>

<article class="doc-entry" id="v-cookie-parse-set">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-parse-set</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-CookieError" title="Text.Cookie.CookieError">CookieError</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a></code><span class="doc-sig-links"><a href="#v-cookie-parse-set" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L305" title="View source at line 305">Source</a></span></div>
<div class="doc-body">

Parse a Set-Cookie header line.

</div>
</article>

<article class="doc-entry" id="v-cookie-partitioned">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-partitioned</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-cookie-partitioned" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L103" title="View source at line 103">Source</a></span></div>
<div class="doc-body">

The Partitioned attribute.

</div>
</article>

<article class="doc-entry" id="v-cookie-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-path</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-cookie-path" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L71" title="View source at line 71">Source</a></span></div>
<div class="doc-body">

The Path attribute.

</div>
</article>

<article class="doc-entry" id="v-cookie-quoted">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-quoted</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-cookie-quoted" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L67" title="View source at line 67">Source</a></span></div>
<div class="doc-body">

Whether the value was quoted.

</div>
</article>

<article class="doc-entry" id="v-cookie-raw">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-raw</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-cookie-raw" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L107" title="View source at line 107">Source</a></span></div>
<div class="doc-body">

The raw Set-Cookie line.

</div>
</article>

<article class="doc-entry" id="v-cookie-raw-expires">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-raw-expires</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-cookie-raw-expires" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L83" title="View source at line 83">Source</a></span></div>
<div class="doc-body">

The raw Expires text.

</div>
</article>

<article class="doc-entry" id="v-cookie-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-read</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a></code><span class="doc-sig-links"><a href="#v-cookie-read" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L287" title="View source at line 287">Source</a></span></div>
<div class="doc-body">

Read a Cookie header line leniently: malformed pairs are skipped; a non-empty filter keeps only that name.

</div>
</article>

<article class="doc-entry" id="v-cookie-same-site">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-same-site</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-SameSite" title="Text.Cookie.SameSite">SameSite</a></code><span class="doc-sig-links"><a href="#v-cookie-same-site" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L99" title="View source at line 99">Source</a></span></div>
<div class="doc-body">

The SameSite attribute.

</div>
</article>

<article class="doc-entry" id="v-cookie-secure">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-secure</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-cookie-secure" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L91" title="View source at line 91">Source</a></span></div>
<div class="doc-body">

The Secure attribute.

</div>
</article>

<article class="doc-entry" id="v-cookie-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-string</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-cookie-string" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L622" title="View source at line 622">Source</a></span></div>
<div class="doc-body">

The cookie as a Set-Cookie value (empty when the name is invalid).

</div>
</article>

<article class="doc-entry" id="v-cookie-time-format">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-time-format</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-cookie-time-format" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L460" title="View source at line 460">Source</a></span></div>
<div class="doc-body">

Unix seconds as "Mon, 02 Jan 2006 15:04:05 GMT".

</div>
</article>

<article class="doc-entry" id="v-cookie-time-parse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-time-parse</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-cookie-time-parse" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L486" title="View source at line 486">Source</a></span></div>
<div class="doc-body">

Parse "Mon, 02 Jan 2006 15:04:05 MST" or "Mon, 02-Jan-2006 15:04:05 MST" to Unix seconds.

</div>
</article>

<article class="doc-entry" id="v-cookie-unparsed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-unparsed</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-cookie-unparsed" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L111" title="View source at line 111">Source</a></span></div>
<div class="doc-body">

The attributes that were not understood, in order.

</div>
</article>

<article class="doc-entry" id="v-cookie-valid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-valid</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-CookieError" title="Text.Cookie.CookieError">CookieError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-cookie-valid" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L664" title="View source at line 664">Source</a></span></div>
<div class="doc-body">

Check a cookie: a token name, an expiry from 1601 when set, value and path bytes, a valid domain, and Secure
when Partitioned.

</div>
</article>

<article class="doc-entry" id="v-cookie-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-value</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-cookie-value" title="Permalink">#</a><a href="/library/source/Text.Cookie/#L63" title="View source at line 63">Source</a></span></div>
<div class="doc-body">

The value.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Cookie.cinch`](/library/source/Text.Cookie/). Source SHA-256: `30a3d34e01b8800167c59dad6112b180b6a1185f80e2ed3d5cc2270efd2af216`.
