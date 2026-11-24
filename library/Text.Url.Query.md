# Text.Url.Query

Persistent byte-preserving query multimaps with deterministic encoding, checked parsing and copied observations.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Url.Query</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Url.Query/source/">View source</a><a href="/packages/search/?q=%2BText.Url.Query">Search module</a><a href="/reference-sources/Text.Url.Query.cinch" download="Text.Url.Query.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Scan](Text.Url.Scan.md).

<details class="doc-closure"><summary>Ship-source closure (4 files)</summary>

Copy these modules beside your entry file before importing `Text.Url.Query`: [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Scan](Text.Url.Scan.md).

```sh
cp seed/prelude/Text/Url/Codec.cinch ./Text.Url.Codec.cinch
cp seed/prelude/Text/Url/Error.cinch ./Text.Url.Error.cinch
cp seed/prelude/Text/Url/Query.cinch ./Text.Url.Query.cinch
cp seed/prelude/Text/Url/Scan.cinch ./Text.Url.Scan.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Persistent, byte-preserving URL query multimaps and bounded form decoding.
Keys are ordered lexicographically by raw bytes; values preserve insertion
order. Parsing retains valid pairs after malformed fields and reports one
precise failure. Raw semicolons reject their field and take diagnostic
precedence. The default limit counts raw ampersand-separated fields, including
empty fields; a zero explicit limit disables it. All external byte buffers are
copied. A balanced tree bounds lookup and updates even for sorted input.

</details>

<details class="doc-synopsis" open><summary>Synopsis (16 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-UrlValues">UrlValues</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlValues</strong></code></li><li><a class="syn-name" href="#v-url-parse-query">url-parse-query</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-parse-query</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>)</code></li><li><a class="syn-name" href="#v-url-parse-query-limit">url-parse-query-limit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-parse-query-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>)</code></li><li><a class="syn-name" href="#v-url-values-add">url-values-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code></li><li><a class="syn-name" href="#v-url-values-clone">url-values-clone</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-clone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code></li><li><a class="syn-name" href="#v-url-values-delete">url-values-delete</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-delete</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code></li><li><a class="syn-name" href="#v-url-values-empty">url-values-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-empty</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code></li><li><a class="syn-name" href="#v-url-values-encode">url-values-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-values-from-pairs">url-values-from-pairs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-from-pairs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code></li><li><a class="syn-name" href="#v-url-values-get">url-values-get</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-get</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-values-get-all">url-values-get-all</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-get-all</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-values-has">url-values-has</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-has</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-url-values-pairs">url-values-pairs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-pairs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-url-values-set">url-values-set</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-set</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code></li><li><a class="syn-name" href="#v-url-values-set-all">url-values-set-all</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-set-all</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code></li><li><a class="syn-name" href="#v-url-values-size">url-values-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-UrlValues">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlValues</strong></code><span class="doc-sig-links"><a href="#t-UrlValues" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Query/source/#L39" title="View source at line 39">Source</a></span></div>
<div class="doc-body">

An immutable query multimap with present-but-empty lists represented explicitly.

</div>
</article>

# Values

<article class="doc-entry" id="v-url-parse-query">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-parse-query</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>)</code><span class="doc-sig-links"><a href="#v-url-parse-query" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Query/source/#L255" title="View source at line 255">Source</a></span></div>
<div class="doc-body">

Parse a query with a maximum of ten thousand raw ampersand-separated fields.

</div>
</article>

<article class="doc-entry" id="v-url-parse-query-limit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-parse-query-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>)</code><span class="doc-sig-links"><a href="#v-url-parse-query-limit" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Query/source/#L248" title="View source at line 248">Source</a></span></div>
<div class="doc-body">

Parse query bytes with an inclusive field limit; zero disables the bound.
Valid fields survive malformed escapes and raw-semicolon fields. Excess raw
fields reject the complete query before decoding; offsets address original bytes.

</div>
</article>

<article class="doc-entry" id="v-url-values-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-values-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code><span class="doc-sig-links"><a href="#v-url-values-add" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Query/source/#L137" title="View source at line 137">Source</a></span></div>
<div class="doc-body">

Append a copied value to a key and return a new multimap version.

</div>
</article>

<article class="doc-entry" id="v-url-values-clone">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-values-clone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code><span class="doc-sig-links"><a href="#v-url-values-clone" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Query/source/#L133" title="View source at line 133">Source</a></span></div>
<div class="doc-body">

Retain an independent immutable version with no externally mutable aliases.

</div>
</article>

<article class="doc-entry" id="v-url-values-delete">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-values-delete</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code><span class="doc-sig-links"><a href="#v-url-values-delete" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Query/source/#L153" title="View source at line 153">Source</a></span></div>
<div class="doc-body">

Remove a key and all its values while retaining earlier versions unchanged.

</div>
</article>

<article class="doc-entry" id="v-url-values-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-values-empty</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code><span class="doc-sig-links"><a href="#v-url-values-empty" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Query/source/#L125" title="View source at line 125">Source</a></span></div>
<div class="doc-body">

Construct an empty immutable query multimap.

</div>
</article>

<article class="doc-entry" id="v-url-values-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-values-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-values-encode" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Query/source/#L213" title="View source at line 213">Source</a></span></div>
<div class="doc-body">

Render sorted keys and insertion-ordered values as ASCII query bytes, omitting empty lists.

</div>
</article>

<article class="doc-entry" id="v-url-values-from-pairs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-values-from-pairs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code><span class="doc-sig-links"><a href="#v-url-values-from-pairs" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Query/source/#L187" title="View source at line 187">Source</a></span></div>
<div class="doc-body">

Snapshot pairs in order, replacing earlier lists when a key is repeated.

</div>
</article>

<article class="doc-entry" id="v-url-values-get">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-values-get</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-values-get" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Query/source/#L161" title="View source at line 161">Source</a></span></div>
<div class="doc-body">

Return a fresh first value, or fresh empty bytes for absent or empty-list keys.

</div>
</article>

<article class="doc-entry" id="v-url-values-get-all">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-values-get-all</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-values-get-all" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Query/source/#L168" title="View source at line 168">Source</a></span></div>
<div class="doc-body">

Return fresh value buffers in insertion order; absent and empty-list keys return an empty list.

</div>
</article>

<article class="doc-entry" id="v-url-values-has">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-values-has</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-url-values-has" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Query/source/#L157" title="View source at line 157">Source</a></span></div>
<div class="doc-body">

Test key presence independently of whether its value list is empty.

</div>
</article>

<article class="doc-entry" id="v-url-values-pairs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-values-pairs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-url-values-pairs" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Query/source/#L183" title="View source at line 183">Source</a></span></div>
<div class="doc-body">

Return keys in raw-byte order and copied value lists in insertion order.

</div>
</article>

<article class="doc-entry" id="v-url-values-set">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-values-set</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code><span class="doc-sig-links"><a href="#v-url-values-set" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Query/source/#L143" title="View source at line 143">Source</a></span></div>
<div class="doc-body">

Replace a key's values with one copied value.

</div>
</article>

<article class="doc-entry" id="v-url-values-set-all">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-values-set-all</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code><span class="doc-sig-links"><a href="#v-url-values-set-all" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Query/source/#L149" title="View source at line 149">Source</a></span></div>
<div class="doc-body">

Replace a key with a copied list; an empty list keeps the key present but emits no encoded pair.

</div>
</article>

<article class="doc-entry" id="v-url-values-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-values-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-url-values-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Query/source/#L129" title="View source at line 129">Source</a></span></div>
<div class="doc-body">

Count distinct keys, including keys with an empty value list.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Url/Query.cinch`](/packages/stdlib/Text.Url.Query/source/). Source SHA-256: `d21bfb7ca2d495f7e67202e8cec6ad27723f767d2d997a5afe9f1b07409e85d3`.
