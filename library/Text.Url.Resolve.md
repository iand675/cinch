# Text.Url.Resolve

URL reference resolution, lexical path joining and copied persistent URL updates.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Url.Resolve</code></div><div class="doc-mod-bar-links"><a href="/library/source/Text.Url.Resolve/">View source</a><a href="/library/search/?q=%2BText.Url.Resolve">Search module</a><a href="/reference-sources/Text.Url.Resolve.cinch" download="Text.Url.Resolve.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Parser](Text.Url.Parser.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

<details class="doc-closure"><summary>Ship-source closure (9 files)</summary>

Copy these modules beside your entry file before importing `Text.Url.Resolve`: [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Ip](Text.Url.Ip.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Parser](Text.Url.Parser.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Resolve](Text.Url.Resolve.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

```sh
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

Byte-preserving reference resolution and lexical URL path joining.
Resolution preserves repeated separators and encoded dot bytes. Joining cleans
separators and literal dot segments; elements are already escaped URL paths.

</details>

<details class="doc-synopsis" open><summary>Synopsis (5 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-url-join">url-join</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-join</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-join-path">url-join-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-join-path</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-join-path-checked">url-join-path-checked</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-join-path-checked</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-parse-reference">url-parse-reference</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-parse-reference</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-resolve-reference">url-resolve-reference</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-resolve-reference</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-url-join">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-join</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-join" title="Permalink">#</a><a href="/library/source/Text.Url.Resolve/#L132" title="View source at line 132">Source</a></span></div>
<div class="doc-body">

Parse a base and join escaped elements, wrapping join failures with the original base bytes.

</div>
</article>

<article class="doc-entry" id="v-url-join-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-join-path</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-join-path" title="Permalink">#</a><a href="/library/source/Text.Url.Resolve/#L128" title="View source at line 128">Source</a></span></div>
<div class="doc-body">

Join escaped paths; malformed percent escapes retain the original immutable URL value.

</div>
</article>

<article class="doc-entry" id="v-url-join-path-checked">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-join-path-checked</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-join-path-checked" title="Permalink">#</a><a href="/library/source/Text.Url.Resolve/#L116" title="View source at line 116">Source</a></span></div>
<div class="doc-body">

Join escaped path elements with lexical separator/dot cleaning and report malformed escapes.
Traversal above the root is clamped; the base path's absolute/relative form is preserved.

</div>
</article>

<article class="doc-entry" id="v-url-parse-reference">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-parse-reference</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-parse-reference" title="Permalink">#</a><a href="/library/source/Text.Url.Resolve/#L101" title="View source at line 101">Source</a></span></div>
<div class="doc-body">

Parse and resolve reference bytes, retaining a typed parse failure and its exact input.

</div>
</article>

<article class="doc-entry" id="v-url-resolve-reference">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-resolve-reference</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-resolve-reference" title="Permalink">#</a><a href="/library/source/Text.Url.Resolve/#L81" title="View source at line 81">Source</a></span></div>
<div class="doc-body">

Resolve an already parsed reference against a base with independent immutable result ownership.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Url/Resolve.cinch`](/library/source/Text.Url.Resolve/). Source SHA-256: `5fd9ec9bddbd778d81d392e1e0e18b6be2f037cbd0f1b15fe1811e4eaf91613e`.
