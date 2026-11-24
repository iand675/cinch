# Data.CookieJar

An RFC 6265 cookie jar as an immutable value: store the cookies responses set and choose the cookies a request sends.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Data.CookieJar</code></div><div class="doc-mod-bar-links"><a href="/library/source/Data.CookieJar/">View source</a><a href="/library/search/?q=%2BData.CookieJar">Search module</a><a href="/reference-sources/Data.CookieJar.cinch" download="Data.CookieJar.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Cookie](Text.Cookie.md), [Text.Punycode](Text.Punycode.md), [Text.Url](Text.Url.md).

<details class="doc-closure"><summary>Ship-source closure (14 files)</summary>

Copy these modules beside your entry file before importing `Data.CookieJar`: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.CookieJar](Data.CookieJar.md), [Text.Cookie](Text.Cookie.md), [Text.Punycode](Text.Punycode.md), [Text.Url](Text.Url.md), [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Ip](Text.Url.Ip.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Parser](Text.Url.Parser.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Resolve](Text.Url.Resolve.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

```sh
cp seed/prelude/Data/Bytes/Build.cinch ./Data.Bytes.Build.cinch
cp seed/prelude/Data/CookieJar.cinch ./Data.CookieJar.cinch
cp seed/prelude/Text/Cookie.cinch ./Text.Cookie.cinch
cp seed/prelude/Text/Punycode.cinch ./Text.Punycode.cinch
cp seed/prelude/Text/Url.cinch ./Text.Url.cinch
cp seed/prelude/Text/Url/Codec.cinch ./Text.Url.Codec.cinch
cp seed/prelude/Text/Url/Error.cinch ./Text.Url.Error.cinch
cp seed/prelude/Text/Url/Ip.cinch ./Text.Url.Ip.cinch
cp seed/prelude/Text/Url/Model.cinch ./Text.Url.Model.cinch
cp seed/prelude/Text/Url/Parser.cinch ./Text.Url.Parser.cinch
cp seed/prelude/Text/Url/Query.cinch ./Text.Url.Query.cinch
cp seed/prelude/Text/Url/Resolve.cinch ./Text.Url.Resolve.cinch
cp seed/prelude/Text/Url/Scan.cinch ./Text.Url.Scan.cinch
cp seed/prelude/Text/Url/Userinfo.cinch ./Text.Url.Userinfo.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis" open><summary>Synopsis (7 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-CookieJar">CookieJar</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CookieJar</strong></code></li><li><a class="syn-name" href="#v-cookie-jar-new">cookie-jar-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cookie-jar-new</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a></code></li><li><a class="syn-name" href="#v-jar-cookies">jar-cookies</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jar-cookies</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a>, <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a>)</code></li><li><a class="syn-name" href="#v-jar-cookies-url">jar-cookies-url</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jar-cookies-url</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a>, <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a>)</code></li><li><a class="syn-name" href="#v-jar-set-cookies">jar-set-cookies</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jar-set-cookies</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a></code></li><li><a class="syn-name" href="#v-jar-set-cookies-url">jar-set-cookies-url</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jar-set-cookies-url</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a></code></li><li><a class="syn-name" href="#v-jar-size">jar-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jar-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-CookieJar">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CookieJar</strong></code><span class="doc-sig-links"><a href="#t-CookieJar" title="Permalink">#</a><a href="/library/source/Data.CookieJar/#L38" title="View source at line 38">Source</a></span></div>
<div class="doc-body">

Fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-CookieJar"><code><strong class="tok-constructor">CookieJar</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-constructor">Entry</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-cookie-jar-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cookie-jar-new</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a></code><span class="doc-sig-links"><a href="#v-cookie-jar-new" title="Permalink">#</a><a href="/library/source/Data.CookieJar/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

An empty jar, optionally with a public suffix function (domain to its public suffix).

</div>
</article>

<article class="doc-entry" id="v-jar-cookies">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jar-cookies</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a>, <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a>)</code><span class="doc-sig-links"><a href="#v-jar-cookies" title="Permalink">#</a><a href="/library/source/Data.CookieJar/#L339" title="View source at line 339">Source</a></span></div>
<div class="doc-body">

The cookies to send with a request to scheme://host/path at a time, and the jar after dropping expired
cookies and recording the access.

</div>
</article>

<article class="doc-entry" id="v-jar-cookies-url">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jar-cookies-url</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a>, <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a>)</code><span class="doc-sig-links"><a href="#v-jar-cookies-url" title="Permalink">#</a><a href="/library/source/Data.CookieJar/#L415" title="View source at line 415">Source</a></span></div>
<div class="doc-body">

The cookies to send with a request to a URL at a time, and the next jar.

</div>
</article>

<article class="doc-entry" id="v-jar-set-cookies">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jar-set-cookies</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a></code><span class="doc-sig-links"><a href="#v-jar-set-cookies" title="Permalink">#</a><a href="/library/source/Data.CookieJar/#L266" title="View source at line 266">Source</a></span></div>
<div class="doc-body">

Store the cookies a response to scheme://host/path set at a time.

</div>
</article>

<article class="doc-entry" id="v-jar-set-cookies-url">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jar-set-cookies-url</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Text.Cookie/#t-Cookie" title="Text.Cookie.Cookie">Cookie</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a></code><span class="doc-sig-links"><a href="#v-jar-set-cookies-url" title="Permalink">#</a><a href="/library/source/Data.CookieJar/#L411" title="View source at line 411">Source</a></span></div>
<div class="doc-body">

Store the cookies a response from a URL set at a time.

</div>
</article>

<article class="doc-entry" id="v-jar-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jar-size</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.CookieJar/#t-CookieJar" title="Data.CookieJar.CookieJar">CookieJar</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-jar-size" title="Permalink">#</a><a href="/library/source/Data.CookieJar/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

How many cookies the jar holds.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/CookieJar.cinch`](/library/source/Data.CookieJar/). Source SHA-256: `a19367e858100095283000a643f73df3673a96b128c0298156d85504f2bbd716`.
