# Text.Url

Immutable byte-preserving URLs, percent escaping, query multimaps, parsing and reference resolution.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Url</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Url/source/">View source</a><a href="/packages/search/?q=%2BText.Url">Search module</a><a href="/reference-sources/Text.Url.cinch" download="Text.Url.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Parser](Text.Url.Parser.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Resolve](Text.Url.Resolve.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

<details class="doc-closure"><summary>Ship-source closure (10 files)</summary>

Copy these modules beside your entry file before importing `Text.Url`: [Text.Url](Text.Url.md), [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Ip](Text.Url.Ip.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Parser](Text.Url.Parser.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Resolve](Text.Url.Resolve.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

```sh
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

Byte-preserving URLs, percent escaping, query multimaps and reference resolution.
Parsed decoded fields retain arbitrary bytes. Immutable URLs and query values
snapshot caller buffers; observations return copied byte storage. Text views
are explicit optional UTF-8 conversions. Existing URI APIs remain independent.

</details>

<details class="doc-synopsis"><summary>Synopsis (92 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Url">Url</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Url</strong></code></li><li><a class="syn-name" href="#t-UrlComponents">UrlComponents</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlComponents</strong></code></li><li><a class="syn-name" href="#t-UrlEncoding">UrlEncoding</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlEncoding</strong></code></li><li><a class="syn-name" href="#t-UrlError">UrlError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlError</strong> <span class="tok-variable">cause</span></code></li><li><a class="syn-name" href="#t-UrlFailure">UrlFailure</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlFailure</strong></code></li><li><a class="syn-name" href="#t-UrlFailureKind">UrlFailureKind</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlFailureKind</strong></code></li><li><a class="syn-name" href="#t-UrlUserinfo">UrlUserinfo</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlUserinfo</strong></code></li><li><a class="syn-name" href="#t-UrlValues">UrlValues</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlValues</strong></code></li><li><a class="syn-name" href="#v-url-append-binary">url-append-binary</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-append-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-clone">url-clone</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-clone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-components">url-components</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlComponents" title="Text.Url.Model.UrlComponents">UrlComponents</a></code></li><li><a class="syn-name" href="#v-url-empty">url-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-empty</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-error">url-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-error</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span></code></li><li><a class="syn-name" href="#v-url-error-cause">url-error-cause</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-error-cause</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">cause</span></code></li><li><a class="syn-name" href="#v-url-error-input">url-error-input</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-error-input</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-error-message">url-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-url-error-operation">url-error-operation</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-error-operation</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-url-error-render">url-error-render</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-error-render</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. (<span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-url-error-temporary">url-error-temporary</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-error-temporary</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-url-error-timeout">url-error-timeout</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-error-timeout</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-url-escape">url-escape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-escaped-fragment">url-escaped-fragment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-escaped-fragment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-escaped-length">url-escaped-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-escaped-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-url-escaped-path">url-escaped-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-escaped-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-failure">url-failure</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-failure</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a></code></li><li><a class="syn-name" href="#v-url-failure-detail">url-failure-detail</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-failure-detail</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-failure-kind">url-failure-kind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-failure-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li><a class="syn-name" href="#v-url-failure-message">url-failure-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-failure-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-url-failure-offset">url-failure-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-failure-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-url-failure-shift">url-failure-shift</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-failure-shift</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a></code></li><li><a class="syn-name" href="#v-url-force-query">url-force-query</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-force-query</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-url-fragment">url-fragment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-fragment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-from-components">url-from-components</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-from-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlComponents" title="Text.Url.Model.UrlComponents">UrlComponents</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-host">url-host</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-host</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-hostname">url-hostname</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-hostname</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-is-absolute">url-is-absolute</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-is-absolute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-url-join">url-join</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-join</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-join-path">url-join-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-join-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-join-path-checked">url-join-path-checked</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-join-path-checked</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-marshal-binary">url-marshal-binary</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-marshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-omit-host">url-omit-host</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-omit-host</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-url-opaque">url-opaque</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-opaque</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-parse">url-parse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-parse-query">url-parse-query</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-parse-query</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>)</code></li><li><a class="syn-name" href="#v-url-parse-query-limit">url-parse-query-limit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-parse-query-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>)</code></li><li><a class="syn-name" href="#v-url-parse-reference">url-parse-reference</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-parse-reference</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-parse-request-uri">url-parse-request-uri</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-parse-request-uri</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-parse-text">url-parse-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-parse-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-password">url-password</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-password</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-path">url-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-path-escape">url-path-escape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-path-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-path-escape-text">url-path-escape-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-path-escape-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-url-path-unescape">url-path-unescape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-path-unescape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-port">url-port</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-port</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-query">url-query</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-query</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code></li><li><a class="syn-name" href="#v-url-query-escape">url-query-escape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-query-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-query-escape-text">url-query-escape-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-query-escape-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-url-query-unescape">url-query-unescape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-query-unescape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-quote-bytes">url-quote-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-quote-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-url-raw-fragment">url-raw-fragment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-raw-fragment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-raw-path">url-raw-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-raw-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-raw-query">url-raw-query</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-raw-query</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-redacted">url-redacted</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-redacted</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-request-uri">url-request-uri</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-request-uri</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-resolve-reference">url-resolve-reference</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-resolve-reference</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-scheme">url-scheme</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-scheme</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-to-bytes">url-to-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-to-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-to-text">url-to-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-to-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-url-unescape">url-unescape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-unescape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-unescape-fragment-hint">url-unescape-fragment-hint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-unescape-fragment-hint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-unescape-path-hint">url-unescape-path-hint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-unescape-path-hint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-unmarshal-binary">url-unmarshal-binary</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-unmarshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-user">url-user</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-user</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a></code></li><li><a class="syn-name" href="#v-url-user-password">url-user-password</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-user-password</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a></code></li><li><a class="syn-name" href="#v-url-userinfo">url-userinfo</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-userinfo</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a></code></li><li><a class="syn-name" href="#v-url-userinfo-bytes">url-userinfo-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-userinfo-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-username">url-username</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-username</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-values-add">url-values-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code></li><li><a class="syn-name" href="#v-url-values-clone">url-values-clone</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-clone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code></li><li><a class="syn-name" href="#v-url-values-delete">url-values-delete</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-delete</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code></li><li><a class="syn-name" href="#v-url-values-empty">url-values-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-empty</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code></li><li><a class="syn-name" href="#v-url-values-encode">url-values-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-values-from-pairs">url-values-from-pairs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-from-pairs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code></li><li><a class="syn-name" href="#v-url-values-get">url-values-get</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-get</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-values-get-all">url-values-get-all</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-get-all</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-values-has">url-values-has</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-has</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-url-values-pairs">url-values-pairs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-pairs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-url-values-set">url-values-set</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-set</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code></li><li><a class="syn-name" href="#v-url-values-set-all">url-values-set-all</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-set-all</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code></li><li><a class="syn-name" href="#v-url-values-size">url-values-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-values-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-url-with-fragment">url-with-fragment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-with-fragment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-with-path">url-with-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-with-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Url">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Url</strong></code><span class="doc-sig-links"><a href="#t-Url" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L34" title="View source at line 34">Source</a></span></div>
<div class="doc-body">

An immutable URL whose component byte buffers have no external mutable aliases.

</div>
</article>

<article class="doc-entry" id="t-UrlComponents">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlComponents</strong></code><span class="doc-sig-links"><a href="#t-UrlComponents" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L29" title="View source at line 29">Source</a></span></div>
<div class="doc-body">

Explicit URL fields: scheme, opaque, user, host, path, fragment, raw query,
raw path hint, raw fragment hint, forced query delimiter and omitted host.
Byte fields may contain arbitrary bytes; constructing a URL snapshots them.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UrlParts"><code><strong class="tok-constructor">UrlParts</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlComponents" title="Text.Url.Model.UrlComponents">UrlComponents</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-UrlEncoding">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlEncoding</strong></code><span class="doc-sig-links"><a href="#t-UrlEncoding" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L24" title="View source at line 24">Source</a></span></div>
<div class="doc-body">

The independent byte grammar of one URL component.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UrlPath"><code><strong class="tok-constructor">UrlPath</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a></code></li><li id="k-UrlPathSegment"><code><strong class="tok-constructor">UrlPathSegment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a></code></li><li id="k-UrlHost"><code><strong class="tok-constructor">UrlHost</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a></code></li><li id="k-UrlZone"><code><strong class="tok-constructor">UrlZone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a></code></li><li id="k-UrlUserPassword"><code><strong class="tok-constructor">UrlUserPassword</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a></code></li><li id="k-UrlQueryComponent"><code><strong class="tok-constructor">UrlQueryComponent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a></code></li><li id="k-UrlFragment"><code><strong class="tok-constructor">UrlFragment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-UrlError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlError</strong> <span class="tok-variable">cause</span></code><span class="doc-sig-links"><a href="#t-UrlError" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L158" title="View source at line 158">Source</a></span></div>
<div class="doc-body">

A typed cause and the original input for one named URL operation.

</div>
</article>

<article class="doc-entry" id="t-UrlFailure">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlFailure</strong></code><span class="doc-sig-links"><a href="#t-UrlFailure" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L52" title="View source at line 52">Source</a></span></div>
<div class="doc-body">

An immutable failure class, component-relative byte offset and exact copied byte detail.

</div>
</article>

<article class="doc-entry" id="t-UrlFailureKind">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlFailureKind</strong></code><span class="doc-sig-links"><a href="#t-UrlFailureKind" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L23" title="View source at line 23">Source</a></span></div>
<div class="doc-body">

A precise class of malformed URL, percent escape or query input.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UrlInvalidEscape"><code><strong class="tok-constructor">UrlInvalidEscape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlInvalidHost"><code><strong class="tok-constructor">UrlInvalidHost</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlControlByte"><code><strong class="tok-constructor">UrlControlByte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlMissingScheme"><code><strong class="tok-constructor">UrlMissingScheme</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlInvalidRequest"><code><strong class="tok-constructor">UrlInvalidRequest</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlEmptyRequest"><code><strong class="tok-constructor">UrlEmptyRequest</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlPathColon"><code><strong class="tok-constructor">UrlPathColon</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlInvalidPort"><code><strong class="tok-constructor">UrlInvalidPort</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlMissingBracket"><code><strong class="tok-constructor">UrlMissingBracket</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlInvalidIpLiteral"><code><strong class="tok-constructor">UrlInvalidIpLiteral</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlInvalidUserinfo"><code><strong class="tok-constructor">UrlInvalidUserinfo</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlQuerySemicolon"><code><strong class="tok-constructor">UrlQuerySemicolon</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlTooManyParameters"><code><strong class="tok-constructor">UrlTooManyParameters</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-UrlUserinfo">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlUserinfo</strong></code><span class="doc-sig-links"><a href="#t-UrlUserinfo" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Userinfo/source/#L12" title="View source at line 12">Source</a></span></div>
<div class="doc-body">

Private copied username and optional copied password; an empty password remains present.

</div>
</article>

<article class="doc-entry" id="t-UrlValues">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlValues</strong></code><span class="doc-sig-links"><a href="#t-UrlValues" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Query/source/#L39" title="View source at line 39">Source</a></span></div>
<div class="doc-body">

An immutable query multimap with present-but-empty lists represented explicitly.

</div>
</article>

# Values

<article class="doc-entry" id="v-url-append-binary">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-append-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-append-binary" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L232" title="View source at line 232">Source</a></span></div>
<div class="doc-body">

Append serialized URL bytes to a copied prefix without mutating either input.

</div>
</article>

<article class="doc-entry" id="v-url-clone">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-clone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-clone" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L54" title="View source at line 54">Source</a></span></div>
<div class="doc-body">

Retain an independent immutable URL version without copying private buffers.

</div>
</article>

<article class="doc-entry" id="v-url-components">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlComponents" title="Text.Url.Model.UrlComponents">UrlComponents</a></code><span class="doc-sig-links"><a href="#v-url-components" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L44" title="View source at line 44">Source</a></span></div>
<div class="doc-body">

Return a fresh editable component snapshot without exposing the private owners.

</div>
</article>

<article class="doc-entry" id="v-url-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-empty</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-empty" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L50" title="View source at line 50">Source</a></span></div>
<div class="doc-body">

Construct the empty relative reference.

</div>
</article>

<article class="doc-entry" id="v-url-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-error</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span></code><span class="doc-sig-links"><a href="#v-url-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L163" title="View source at line 163">Source</a></span></div>
<div class="doc-body">

Wrap any typed cause with copied input and caller-supplied temporary/timeout flags.

</div>
</article>

<article class="doc-entry" id="v-url-error-cause">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-error-cause</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">cause</span></code><span class="doc-sig-links"><a href="#v-url-error-cause" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L175" title="View source at line 175">Source</a></span></div>
<div class="doc-body">

Unwrap the original typed cause without erasing its type or information.

</div>
</article>

<article class="doc-entry" id="v-url-error-input">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-error-input</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-error-input" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L171" title="View source at line 171">Source</a></span></div>
<div class="doc-body">

Return a fresh copy of the original operation input, without requiring valid UTF-8.

</div>
</article>

<article class="doc-entry" id="v-url-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-url-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L193" title="View source at line 193">Source</a></span></div>
<div class="doc-body">

Render a URL parsing or escaping context with its structured built-in failure.

</div>
</article>

<article class="doc-entry" id="v-url-error-operation">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-error-operation</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-url-error-operation" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L167" title="View source at line 167">Source</a></span></div>
<div class="doc-body">

Observe the operation name associated with the failure.

</div>
</article>

<article class="doc-entry" id="v-url-error-render">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-error-render</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. (<span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-url-error-render" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L187" title="View source at line 187">Source</a></span></div>
<div class="doc-body">

Render any typed contextual error using its supplied cause formatter.

</div>
</article>

<article class="doc-entry" id="v-url-error-temporary">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-error-temporary</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-url-error-temporary" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L179" title="View source at line 179">Source</a></span></div>
<div class="doc-body">

Observe explicit temporary classification; pure parsing failures are not temporary.

</div>
</article>

<article class="doc-entry" id="v-url-error-timeout">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-error-timeout</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-url-error-timeout" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L183" title="View source at line 183">Source</a></span></div>
<div class="doc-body">

Observe explicit timeout classification; pure parsing failures do not time out.

</div>
</article>

<article class="doc-entry" id="v-url-escape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-escape" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L241" title="View source at line 241">Source</a></span></div>
<div class="doc-body">

Encode one complete component into fresh bytes, preserving no caller aliases.

</div>
</article>

<article class="doc-entry" id="v-url-escaped-fragment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-escaped-fragment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-escaped-fragment" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L111" title="View source at line 111">Source</a></span></div>
<div class="doc-body">

Choose a valid raw fragment spelling only when it decodes to the stored fragment.

</div>
</article>

<article class="doc-entry" id="v-url-escaped-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-escaped-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-url-escaped-length" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L178" title="View source at line 178">Source</a></span></div>
<div class="doc-body">

Exact percent-encoded byte count; query-space substitution does not change length.

</div>
</article>

<article class="doc-entry" id="v-url-escaped-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-escaped-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-escaped-path" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L102" title="View source at line 102">Source</a></span></div>
<div class="doc-body">

Choose a valid raw path spelling only when it decodes to the stored path.

</div>
</article>

<article class="doc-entry" id="v-url-failure">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-failure</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a></code><span class="doc-sig-links"><a href="#v-url-failure" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

Construct a byte-precise failure without losing non-UTF-8 detail.

</div>
</article>

<article class="doc-entry" id="v-url-failure-detail">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-failure-detail</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-failure-detail" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

Return a fresh copy of the exact offending bytes, including non-UTF-8 bytes.

</div>
</article>

<article class="doc-entry" id="v-url-failure-kind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-failure-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code><span class="doc-sig-links"><a href="#v-url-failure-kind" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">

Observe the named failure class.

</div>
</article>

<article class="doc-entry" id="v-url-failure-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-failure-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-url-failure-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L140" title="View source at line 140">Source</a></span></div>
<div class="doc-body">

Render a named failure while retaining its raw detail separately for programmatic handling.

</div>
</article>

<article class="doc-entry" id="v-url-failure-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-failure-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-url-failure-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

Observe the zero-based byte position within the component being processed.

</div>
</article>

<article class="doc-entry" id="v-url-failure-shift">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-failure-shift</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a></code><span class="doc-sig-links"><a href="#v-url-failure-shift" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

Rebase a component-relative failure into a larger original byte interval.

</div>
</article>

<article class="doc-entry" id="v-url-force-query">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-force-query</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-url-force-query" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L94" title="View source at line 94">Source</a></span></div>
<div class="doc-body">

Observe force query without exposing mutable storage.

</div>
</article>

<article class="doc-entry" id="v-url-fragment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-fragment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-fragment" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

Observe fragment as a fresh byte buffer.

</div>
</article>

<article class="doc-entry" id="v-url-from-components">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-from-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlComponents" title="Text.Url.Model.UrlComponents">UrlComponents</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-from-components" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L39" title="View source at line 39">Source</a></span></div>
<div class="doc-body">

Snapshot every externally mutable byte field in a component bundle.

</div>
</article>

<article class="doc-entry" id="v-url-host">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-host</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-host" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

Observe host as a fresh byte buffer.

</div>
</article>

<article class="doc-entry" id="v-url-hostname">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-hostname</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-hostname" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L198" title="View source at line 198">Source</a></span></div>
<div class="doc-body">

Return host bytes without a valid optional port and without enclosing IP brackets.

</div>
</article>

<article class="doc-entry" id="v-url-is-absolute">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-is-absolute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-url-is-absolute" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L183" title="View source at line 183">Source</a></span></div>
<div class="doc-body">

A URL is absolute precisely when its scheme is nonempty.

</div>
</article>

<article class="doc-entry" id="v-url-join">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-join</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-join" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Resolve/source/#L132" title="View source at line 132">Source</a></span></div>
<div class="doc-body">

Parse a base and join escaped elements, wrapping join failures with the original base bytes.

</div>
</article>

<article class="doc-entry" id="v-url-join-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-join-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-join-path" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Resolve/source/#L128" title="View source at line 128">Source</a></span></div>
<div class="doc-body">

Join escaped paths; malformed percent escapes retain the original immutable URL value.

</div>
</article>

<article class="doc-entry" id="v-url-join-path-checked">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-join-path-checked</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-join-path-checked" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Resolve/source/#L116" title="View source at line 116">Source</a></span></div>
<div class="doc-body">

Join escaped path elements with lexical separator/dot cleaning and report malformed escapes.
Traversal above the root is clamped; the base path's absolute/relative form is preserved.

</div>
</article>

<article class="doc-entry" id="v-url-marshal-binary">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-marshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-marshal-binary" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L228" title="View source at line 228">Source</a></span></div>
<div class="doc-body">

Serialize a URL to its fresh byte representation.

</div>
</article>

<article class="doc-entry" id="v-url-omit-host">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-omit-host</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-url-omit-host" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L98" title="View source at line 98">Source</a></span></div>
<div class="doc-body">

Observe omit host without exposing mutable storage.

</div>
</article>

<article class="doc-entry" id="v-url-opaque">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-opaque</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-opaque" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

Observe opaque as a fresh byte buffer.

</div>
</article>

<article class="doc-entry" id="v-url-parse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-parse" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Parser/source/#L162" title="View source at line 162">Source</a></span></div>
<div class="doc-body">

Parse an arbitrary byte URL/reference; fragment failures retain full original input context.

</div>
</article>

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

<article class="doc-entry" id="v-url-parse-reference">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-parse-reference</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-parse-reference" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Resolve/source/#L101" title="View source at line 101">Source</a></span></div>
<div class="doc-body">

Parse and resolve reference bytes, retaining a typed parse failure and its exact input.

</div>
</article>

<article class="doc-entry" id="v-url-parse-request-uri">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-parse-request-uri</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-parse-request-uri" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Parser/source/#L176" title="View source at line 176">Source</a></span></div>
<div class="doc-body">

Parse an absolute request URL, absolute path or asterisk without treating hash as a fragment delimiter.

</div>
</article>

<article class="doc-entry" id="v-url-parse-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-parse-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-parse-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Parser/source/#L172" title="View source at line 172">Source</a></span></div>
<div class="doc-body">

Parse a UTF-8 URL while retaining arbitrary percent-decoded field bytes.

</div>
</article>

<article class="doc-entry" id="v-url-password">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-password</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-password" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Userinfo/source/#L29" title="View source at line 29">Source</a></span></div>
<div class="doc-body">

Return a copied optional password, distinguishing absence from an empty password.

</div>
</article>

<article class="doc-entry" id="v-url-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-path" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

Observe path as a fresh byte buffer.

</div>
</article>

<article class="doc-entry" id="v-url-path-escape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-path-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-path-escape" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L423" title="View source at line 423">Source</a></span></div>
<div class="doc-body">

Escape one arbitrary-byte path segment, including every slash inside the segment.

</div>
</article>

<article class="doc-entry" id="v-url-path-escape-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-path-escape-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-url-path-escape-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L439" title="View source at line 439">Source</a></span></div>
<div class="doc-body">

Escape UTF-8 text as an ASCII path segment; decoding remains byte based.

</div>
</article>

<article class="doc-entry" id="v-url-path-unescape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-path-unescape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-path-unescape" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L427" title="View source at line 427">Source</a></span></div>
<div class="doc-body">

Percent-decode a path component without treating plus as a space.

</div>
</article>

<article class="doc-entry" id="v-url-port">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-port</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-port" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L205" title="View source at line 205">Source</a></span></div>
<div class="doc-body">

Return lexical decimal port bytes, including an empty result for an absent or empty port.

</div>
</article>

<article class="doc-entry" id="v-url-query">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-query</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlValues" title="Text.Url.Query.UrlValues">UrlValues</a></code><span class="doc-sig-links"><a href="#v-url-query" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L209" title="View source at line 209">Source</a></span></div>
<div class="doc-body">

Parse raw query fields with the default bound, discarding malformed pairs and their diagnostics.

</div>
</article>

<article class="doc-entry" id="v-url-query-escape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-query-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-query-escape" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L431" title="View source at line 431">Source</a></span></div>
<div class="doc-body">

Escape one arbitrary-byte query key or value, using plus for space.

</div>
</article>

<article class="doc-entry" id="v-url-query-escape-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-query-escape-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-url-query-escape-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L443" title="View source at line 443">Source</a></span></div>
<div class="doc-body">

Escape UTF-8 text as an ASCII query component; decoding remains byte based.

</div>
</article>

<article class="doc-entry" id="v-url-query-unescape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-query-unescape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-query-unescape" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L435" title="View source at line 435">Source</a></span></div>
<div class="doc-body">

Decode a query key or value, mapping plus to space and retaining arbitrary decoded bytes.

</div>
</article>

<article class="doc-entry" id="v-url-quote-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-quote-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-url-quote-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L129" title="View source at line 129">Source</a></span></div>
<div class="doc-body">

Produce an ASCII byte-exact quoted diagnostic; non-ASCII bytes use hexadecimal escapes.

</div>
</article>

<article class="doc-entry" id="v-url-raw-fragment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-raw-fragment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-raw-fragment" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L90" title="View source at line 90">Source</a></span></div>
<div class="doc-body">

Observe raw fragment as a fresh byte buffer.

</div>
</article>

<article class="doc-entry" id="v-url-raw-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-raw-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-raw-path" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L86" title="View source at line 86">Source</a></span></div>
<div class="doc-body">

Observe raw path as a fresh byte buffer.

</div>
</article>

<article class="doc-entry" id="v-url-raw-query">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-raw-query</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-raw-query" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">

Observe raw query as a fresh byte buffer.

</div>
</article>

<article class="doc-entry" id="v-url-redacted">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-redacted</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-redacted" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L213" title="View source at line 213">Source</a></span></div>
<div class="doc-body">

Return fresh URL bytes with a present password replaced by five lowercase x bytes.

</div>
</article>

<article class="doc-entry" id="v-url-request-uri">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-request-uri</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-request-uri" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L220" title="View source at line 220">Source</a></span></div>
<div class="doc-body">

Render a request target, excluding fragment and using slash for an empty hierarchical path.

</div>
</article>

<article class="doc-entry" id="v-url-resolve-reference">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-resolve-reference</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-resolve-reference" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Resolve/source/#L81" title="View source at line 81">Source</a></span></div>
<div class="doc-body">

Resolve an already parsed reference against a base with independent immutable result ownership.

</div>
</article>

<article class="doc-entry" id="v-url-scheme">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-scheme</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-scheme" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L58" title="View source at line 58">Source</a></span></div>
<div class="doc-body">

Observe scheme as a fresh byte buffer.

</div>
</article>

<article class="doc-entry" id="v-url-to-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-to-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-to-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L164" title="View source at line 164">Source</a></span></div>
<div class="doc-body">

Reassemble fresh URL bytes, preserving raw query and opaque bytes exactly.

</div>
</article>

<article class="doc-entry" id="v-url-to-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-to-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-url-to-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L179" title="View source at line 179">Source</a></span></div>
<div class="doc-body">

Decode rendered bytes as UTF-8 only when every preserved byte is valid text.

</div>
</article>

<article class="doc-entry" id="v-url-unescape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-unescape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlEncoding" title="Text.Url.Codec.UrlEncoding">UrlEncoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-unescape" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L376" title="View source at line 376">Source</a></span></div>
<div class="doc-body">

Decode a complete component to fresh arbitrary bytes, or return a precise failure without partial output.

</div>
</article>

<article class="doc-entry" id="v-url-unescape-fragment-hint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-unescape-fragment-hint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-unescape-fragment-hint" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L419" title="View source at line 419">Source</a></span></div>
<div class="doc-body">

Decode a raw fragment hint only when its pchar spellings and percent triplets are valid.

</div>
</article>

<article class="doc-entry" id="v-url-unescape-path-hint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-unescape-path-hint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-unescape-path-hint" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Codec/source/#L415" title="View source at line 415">Source</a></span></div>
<div class="doc-body">

Decode a raw path hint only when its pchar spellings and percent triplets are valid.

</div>
</article>

<article class="doc-entry" id="v-url-unmarshal-binary">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-unmarshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-unmarshal-binary" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Parser/source/#L180" title="View source at line 180">Source</a></span></div>
<div class="doc-body">

Decode the byte serialization by parsing a fresh immutable URL value.

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

<article class="doc-entry" id="v-url-userinfo">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-userinfo</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlUserinfo" title="Text.Url.Userinfo.UrlUserinfo">UrlUserinfo</a></code><span class="doc-sig-links"><a href="#v-url-userinfo" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">

Observe user without exposing mutable storage.

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

<article class="doc-entry" id="v-url-with-fragment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-with-fragment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-with-fragment" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L128" title="View source at line 128">Source</a></span></div>
<div class="doc-body">

Set an escaped fragment, preserving a noncanonical raw hint and decoded bytes.

</div>
</article>

<article class="doc-entry" id="v-url-with-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-with-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-with-path" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Model/source/#L120" title="View source at line 120">Source</a></span></div>
<div class="doc-body">

Set an escaped path on a copied field snapshot, preserving a noncanonical raw hint.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Url.cinch`](/packages/stdlib/Text.Url/source/). Source SHA-256: `ace795fa48c2bfdab5f1c984b072eff6ad2ca68c11403d965ad610937b45001c`.
