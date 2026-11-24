# Text.Url.Parser

Checked general and request URL parsing with precise scheme, host, port and escape rules.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Url.Parser</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Url.Parser/source/">View source</a><a href="/packages/search/?q=%2BText.Url.Parser">Search module</a><a href="/reference-sources/Text.Url.Parser.cinch" download="Text.Url.Parser.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Ip](Text.Url.Ip.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

<details class="doc-closure"><summary>Ship-source closure (8 files)</summary>

Copy these modules beside your entry file before importing `Text.Url.Parser`: [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Ip](Text.Url.Ip.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Parser](Text.Url.Parser.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

```sh
cp seed/prelude/Text/Url/Codec.cinch ./Text.Url.Codec.cinch
cp seed/prelude/Text/Url/Error.cinch ./Text.Url.Error.cinch
cp seed/prelude/Text/Url/Ip.cinch ./Text.Url.Ip.cinch
cp seed/prelude/Text/Url/Model.cinch ./Text.Url.Model.cinch
cp seed/prelude/Text/Url/Parser.cinch ./Text.Url.Parser.cinch
cp seed/prelude/Text/Url/Query.cinch ./Text.Url.Query.cinch
cp seed/prelude/Text/Url/Scan.cinch ./Text.Url.Scan.cinch
cp seed/prelude/Text/Url/Userinfo.cinch ./Text.Url.Userinfo.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Byte-preserving hierarchical and opaque URL parsing with precise typed failures.
Raw queries and opaque bodies are preserved without interpreting percent signs.
Decoded user, host, path and fragment bytes retain non-UTF-8 data. HTTP and HTTPS
authorities reject unbracketed multiple-colon hosts; other schemes retain the
lexical final-port convention used by multi-host connection strings.

</details>

<details class="doc-synopsis" open><summary>Synopsis (4 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-url-parse">url-parse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-parse-request-uri">url-parse-request-uri</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-parse-request-uri</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-parse-text">url-parse-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-parse-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li><li><a class="syn-name" href="#v-url-unmarshal-binary">url-unmarshal-binary</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-unmarshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-url-parse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-parse" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Parser/source/#L162" title="View source at line 162">Source</a></span></div>
<div class="doc-body">

Parse an arbitrary byte URL/reference; fragment failures retain full original input context.

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

<article class="doc-entry" id="v-url-unmarshal-binary">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-unmarshal-binary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a>) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-Url" title="Text.Url.Model.Url">Url</a></code><span class="doc-sig-links"><a href="#v-url-unmarshal-binary" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Parser/source/#L180" title="View source at line 180">Source</a></span></div>
<div class="doc-body">

Decode the byte serialization by parsing a fresh immutable URL value.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Url/Parser.cinch`](/packages/stdlib/Text.Url.Parser/source/). Source SHA-256: `d880177f125cbdf7d9546cb5526ce6f6717ead1c719b180724745be6ef1c4f7d`.
