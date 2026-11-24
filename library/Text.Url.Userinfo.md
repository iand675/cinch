# Text.Url.Userinfo

Immutable copied URL user information with explicit password presence and checked percent encoding.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Url.Userinfo</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Url.Userinfo/source/">View source</a><a href="/packages/search/?q=%2BText.Url.Userinfo">Search module</a><a href="/reference-sources/Text.Url.Userinfo.cinch" download="Text.Url.Userinfo.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Scan](Text.Url.Scan.md).

<details class="doc-notes"><summary>Module notes</summary>

Immutable byte-preserving URL user information with explicit password presence.

</details>

<details class="doc-synopsis" open><summary>Synopsis (6 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-UrlUserinfo">UrlUserinfo</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlUserinfo</strong></code></li><li><a class="syn-name" href="#v-url-password">url-password</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-password</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-user">url-user</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-user</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a></code></li><li><a class="syn-name" href="#v-url-user-password">url-user-password</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-user-password</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a></code></li><li><a class="syn-name" href="#v-url-userinfo-bytes">url-userinfo-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-userinfo-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-username">url-username</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-username</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-UrlUserinfo">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlUserinfo</strong></code><span class="doc-sig-links"><a href="#t-UrlUserinfo" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Userinfo/source/#L12" title="View source at line 12">Source</a></span></div>
<div class="doc-body">

Private copied username and optional copied password; an empty password remains present.

</div>
</article>

# Values

<article class="doc-entry" id="v-url-password">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-password</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-password" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Userinfo/source/#L29" title="View source at line 29">Source</a></span></div>
<div class="doc-body">

Return a copied optional password, distinguishing absence from an empty password.

</div>
</article>

<article class="doc-entry" id="v-url-user">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-user</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a></code><span class="doc-sig-links"><a href="#v-url-user" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Userinfo/source/#L17" title="View source at line 17">Source</a></span></div>
<div class="doc-body">

Construct user information without a password, copying arbitrary username bytes.

</div>
</article>

<article class="doc-entry" id="v-url-user-password">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-user-password</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a></code><span class="doc-sig-links"><a href="#v-url-user-password" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Userinfo/source/#L21" title="View source at line 21">Source</a></span></div>
<div class="doc-body">

Construct user information with a present password, copying both byte buffers.

</div>
</article>

<article class="doc-entry" id="v-url-userinfo-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-userinfo-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-userinfo-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Userinfo/source/#L33" title="View source at line 33">Source</a></span></div>
<div class="doc-body">

Escape the username and optional colon-separated password to fresh ASCII bytes.

</div>
</article>

<article class="doc-entry" id="v-url-username">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-username</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-username" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Userinfo/source/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">

Return fresh username bytes without decoding them as text.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Url/Userinfo.cinch`](/packages/stdlib/Text.Url.Userinfo/source/). Source SHA-256: `cccdb3acac6ac60383d2a2f8ce9fd9fde4953de7f28aa8259a5d5436ec52758a`.
