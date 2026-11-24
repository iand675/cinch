# Text.Url.Model

Immutable URL components, raw encoding hints, rendering and independent copied observations.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Url.Model</code></div><div class="doc-mod-bar-links"><a href="/library/source/Text.Url.Model/">View source</a><a href="/library/search/?q=%2BText.Url.Model">Search module</a><a href="/reference-sources/Text.Url.Model.cinch" download="Text.Url.Model.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

<details class="doc-closure"><summary>Ship-source closure (6 files)</summary>

Copy these modules beside your entry file before importing `Text.Url.Model`: [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

```sh
cp seed/prelude/Text/Url/Codec.cinch ./Text.Url.Codec.cinch
cp seed/prelude/Text/Url/Error.cinch ./Text.Url.Error.cinch
cp seed/prelude/Text/Url/Model.cinch ./Text.Url.Model.cinch
cp seed/prelude/Text/Url/Query.cinch ./Text.Url.Query.cinch
cp seed/prelude/Text/Url/Scan.cinch ./Text.Url.Scan.cinch
cp seed/prelude/Text/Url/Userinfo.cinch ./Text.Url.Userinfo.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Immutable byte-preserving URL components, rendering and independent observations.
Decoded fields remain bytes, including invalid UTF-8. Rendering returns bytes;
the optional text view succeeds only when the complete result is valid UTF-8.
Manual components are copied, raw encoding hints are checked before use, and
all byte getters return fresh storage. Retained versions share private owners.

</details>

<details class="doc-synopsis"><summary>Synopsis (31 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Url">Url</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Url</strong></code></li><li><a class="syn-name" href="#t-UrlComponents">UrlComponents</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlComponents</strong></code></li><li><a class="syn-name" href="#v-url-append-binary">url-append-binary</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-append-binary</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-clone">url-clone</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-clone</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-components">url-components</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-components</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlComponents" title="Text.Url.Model.UrlComponents">UrlComponents</a></code></li><li><a class="syn-name" href="#v-url-empty">url-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-empty</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-escaped-fragment">url-escaped-fragment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-escaped-fragment</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-escaped-path">url-escaped-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-escaped-path</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-force-query">url-force-query</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-force-query</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-url-fragment">url-fragment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-fragment</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-from-components">url-from-components</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-from-components</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlComponents" title="Text.Url.Model.UrlComponents">UrlComponents</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-host">url-host</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-host</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-hostname">url-hostname</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-hostname</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-is-absolute">url-is-absolute</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-is-absolute</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-url-marshal-binary">url-marshal-binary</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-marshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-omit-host">url-omit-host</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-omit-host</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-url-opaque">url-opaque</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-opaque</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-path">url-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-path</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-port">url-port</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-port</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-query">url-query</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-query</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code></li><li><a class="syn-name" href="#v-url-raw-fragment">url-raw-fragment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-raw-fragment</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-raw-path">url-raw-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-raw-path</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-raw-query">url-raw-query</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-raw-query</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-redacted">url-redacted</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-redacted</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-request-uri">url-request-uri</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-request-uri</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-scheme">url-scheme</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-scheme</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-to-bytes">url-to-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-to-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-to-text">url-to-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-to-text</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-url-userinfo">url-userinfo</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-userinfo</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a></code></li><li><a class="syn-name" href="#v-url-with-fragment">url-with-fragment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-with-fragment</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-with-path">url-with-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-with-path</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Url">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Url</strong></code><span class="doc-sig-links"><a href="#t-Url" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L34" title="View source at line 34">Source</a></span></div>
<div class="doc-body">

An immutable URL whose component byte buffers have no external mutable aliases.

</div>
</article>

<article class="doc-entry" id="t-UrlComponents">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlComponents</strong></code><span class="doc-sig-links"><a href="#t-UrlComponents" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L29" title="View source at line 29">Source</a></span></div>
<div class="doc-body">

Explicit URL fields: scheme, opaque, user, host, path, fragment, raw query,
raw path hint, raw fragment hint, forced query delimiter and omitted host.
Byte fields may contain arbitrary bytes; constructing a URL snapshots them.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UrlParts"><code><strong class="tok-constructor">UrlParts</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlComponents" title="Text.Url.Model.UrlComponents">UrlComponents</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-url-append-binary">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-append-binary</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-append-binary" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L232" title="View source at line 232">Source</a></span></div>
<div class="doc-body">

Append serialized URL bytes to a copied prefix without mutating either input.

</div>
</article>

<article class="doc-entry" id="v-url-clone">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-clone</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-clone" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L54" title="View source at line 54">Source</a></span></div>
<div class="doc-body">

Retain an independent immutable URL version without copying private buffers.

</div>
</article>

<article class="doc-entry" id="v-url-components">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-components</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlComponents" title="Text.Url.Model.UrlComponents">UrlComponents</a></code><span class="doc-sig-links"><a href="#v-url-components" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L44" title="View source at line 44">Source</a></span></div>
<div class="doc-body">

Return a fresh editable component snapshot without exposing the private owners.

</div>
</article>

<article class="doc-entry" id="v-url-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-empty</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-empty" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L50" title="View source at line 50">Source</a></span></div>
<div class="doc-body">

Construct the empty relative reference.

</div>
</article>

<article class="doc-entry" id="v-url-escaped-fragment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-escaped-fragment</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-escaped-fragment" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L111" title="View source at line 111">Source</a></span></div>
<div class="doc-body">

Choose a valid raw fragment spelling only when it decodes to the stored fragment.

</div>
</article>

<article class="doc-entry" id="v-url-escaped-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-escaped-path</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-escaped-path" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L102" title="View source at line 102">Source</a></span></div>
<div class="doc-body">

Choose a valid raw path spelling only when it decodes to the stored path.

</div>
</article>

<article class="doc-entry" id="v-url-force-query">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-force-query</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-url-force-query" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L94" title="View source at line 94">Source</a></span></div>
<div class="doc-body">

Observe force query without exposing mutable storage.

</div>
</article>

<article class="doc-entry" id="v-url-fragment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-fragment</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-fragment" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

Observe fragment as a fresh byte buffer.

</div>
</article>

<article class="doc-entry" id="v-url-from-components">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-from-components</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlComponents" title="Text.Url.Model.UrlComponents">UrlComponents</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-from-components" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L39" title="View source at line 39">Source</a></span></div>
<div class="doc-body">

Snapshot every externally mutable byte field in a component bundle.

</div>
</article>

<article class="doc-entry" id="v-url-host">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-host</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-host" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

Observe host as a fresh byte buffer.

</div>
</article>

<article class="doc-entry" id="v-url-hostname">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-hostname</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-hostname" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L198" title="View source at line 198">Source</a></span></div>
<div class="doc-body">

Return host bytes without a valid optional port and without enclosing IP brackets.

</div>
</article>

<article class="doc-entry" id="v-url-is-absolute">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-is-absolute</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-url-is-absolute" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L183" title="View source at line 183">Source</a></span></div>
<div class="doc-body">

A URL is absolute precisely when its scheme is nonempty.

</div>
</article>

<article class="doc-entry" id="v-url-marshal-binary">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-marshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-marshal-binary" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L228" title="View source at line 228">Source</a></span></div>
<div class="doc-body">

Serialize a URL to its fresh byte representation.

</div>
</article>

<article class="doc-entry" id="v-url-omit-host">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-omit-host</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-url-omit-host" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L98" title="View source at line 98">Source</a></span></div>
<div class="doc-body">

Observe omit host without exposing mutable storage.

</div>
</article>

<article class="doc-entry" id="v-url-opaque">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-opaque</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-opaque" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

Observe opaque as a fresh byte buffer.

</div>
</article>

<article class="doc-entry" id="v-url-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-path</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-path" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

Observe path as a fresh byte buffer.

</div>
</article>

<article class="doc-entry" id="v-url-port">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-port</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-port" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L205" title="View source at line 205">Source</a></span></div>
<div class="doc-body">

Return lexical decimal port bytes, including an empty result for an absent or empty port.

</div>
</article>

<article class="doc-entry" id="v-url-query">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-query</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code><span class="doc-sig-links"><a href="#v-url-query" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L209" title="View source at line 209">Source</a></span></div>
<div class="doc-body">

Parse raw query fields with the default bound, discarding malformed pairs and their diagnostics.

</div>
</article>

<article class="doc-entry" id="v-url-raw-fragment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-raw-fragment</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-raw-fragment" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L90" title="View source at line 90">Source</a></span></div>
<div class="doc-body">

Observe raw fragment as a fresh byte buffer.

</div>
</article>

<article class="doc-entry" id="v-url-raw-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-raw-path</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-raw-path" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L86" title="View source at line 86">Source</a></span></div>
<div class="doc-body">

Observe raw path as a fresh byte buffer.

</div>
</article>

<article class="doc-entry" id="v-url-raw-query">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-raw-query</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-raw-query" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">

Observe raw query as a fresh byte buffer.

</div>
</article>

<article class="doc-entry" id="v-url-redacted">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-redacted</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-redacted" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L213" title="View source at line 213">Source</a></span></div>
<div class="doc-body">

Return fresh URL bytes with a present password replaced by five lowercase x bytes.

</div>
</article>

<article class="doc-entry" id="v-url-request-uri">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-request-uri</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-request-uri" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L220" title="View source at line 220">Source</a></span></div>
<div class="doc-body">

Render a request target, excluding fragment and using slash for an empty hierarchical path.

</div>
</article>

<article class="doc-entry" id="v-url-scheme">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-scheme</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-scheme" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L58" title="View source at line 58">Source</a></span></div>
<div class="doc-body">

Observe scheme as a fresh byte buffer.

</div>
</article>

<article class="doc-entry" id="v-url-to-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-to-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-to-bytes" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L164" title="View source at line 164">Source</a></span></div>
<div class="doc-body">

Reassemble fresh URL bytes, preserving raw query and opaque bytes exactly.

</div>
</article>

<article class="doc-entry" id="v-url-to-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-to-text</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-url-to-text" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L179" title="View source at line 179">Source</a></span></div>
<div class="doc-body">

Decode rendered bytes as UTF-8 only when every preserved byte is valid text.

</div>
</article>

<article class="doc-entry" id="v-url-userinfo">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-userinfo</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a></code><span class="doc-sig-links"><a href="#v-url-userinfo" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">

Observe user without exposing mutable storage.

</div>
</article>

<article class="doc-entry" id="v-url-with-fragment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-with-fragment</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-with-fragment" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L128" title="View source at line 128">Source</a></span></div>
<div class="doc-body">

Set an escaped fragment, preserving a noncanonical raw hint and decoded bytes.

</div>
</article>

<article class="doc-entry" id="v-url-with-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-with-path</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-with-path" title="Permalink">#</a><a href="/library/source/Text.Url.Model/#L120" title="View source at line 120">Source</a></span></div>
<div class="doc-body">

Set an escaped path on a copied field snapshot, preserving a noncanonical raw hint.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Url/Model.cinch`](/library/source/Text.Url.Model/). Source SHA-256: `7fee092c7305cda028b8ec71763d8f1c76189cb35c76d1a02bb068db1aca7f75`.
