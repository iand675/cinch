# Text.Multipart

Multipart wire primitives over arbitrary bytes, exact multi-value headers and explicit delimiter observations.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Multipart</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Multipart/source/">View source</a><a href="/packages/search/?q=%2BText.Multipart">Search module</a><a href="/reference-sources/Text.Multipart.cinch" download="Text.Multipart.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Mime](Text.Mime.md).

<details class="doc-closure"><summary>Ship-source closure (7 files)</summary>

Copy these modules beside your entry file before importing `Text.Multipart`: [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Mime](Text.Mime.md), [Text.Multipart](Text.Multipart.md).

```sh
cp seed/prelude/Data/Bytes/Utf8.cinch ./Data.Bytes.Utf8.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Data/Unicode/Kernel.cinch ./Data.Unicode.Kernel.cinch
cp seed/prelude/Data/Unicode/Tables.cinch ./Data.Unicode.Tables.cinch
cp seed/prelude/Text/Base64/Encoding.cinch ./Text.Base64.Encoding.cinch
cp seed/prelude/Text/Mime.cinch ./Text.Mime.cinch
cp seed/prelude/Text/Multipart.cinch ./Text.Multipart.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Multipart wire primitives over arbitrary bytes, exact multi-value headers and
explicit delimiter observations. Retained values and returned byte views are
independently owned snapshots. Boundary scans never read outside a checked span.

</details>

<details class="doc-synopsis"><summary>Synopsis (29 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-MultipartBodyScan">MultipartBodyScan</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MultipartBodyScan</strong></code></li><li><a class="syn-name" href="#t-MultipartDelimiter">MultipartDelimiter</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MultipartDelimiter</strong></code></li><li><a class="syn-name" href="#t-MultipartHeaders">MultipartHeaders</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MultipartHeaders</strong></code></li><li><a class="syn-name" href="#t-MultipartSyntax">MultipartSyntax</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MultipartSyntax</strong></code></li><li><a class="syn-name" href="#v-multipart-body-scan">multipart-body-scan</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-body-scan</strong> :: <span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span> &lt;= 4096 <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg3</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg4</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartBodyScan" title="Text.Multipart.MultipartBodyScan">MultipartBodyScan</a></code></li><li><a class="syn-name" href="#v-multipart-check-boundary">multipart-check-boundary</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-check-boundary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartSyntax" title="Text.Multipart.MultipartSyntax">MultipartSyntax</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-multipart-delimiter-line">multipart-delimiter-line</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-delimiter-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartDelimiter" title="Text.Multipart.MultipartDelimiter">MultipartDelimiter</a></code></li><li><a class="syn-name" href="#v-multipart-field-disposition">multipart-field-disposition</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-field-disposition</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-multipart-file-disposition">multipart-file-disposition</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-file-disposition</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-multipart-find-byte">multipart-find-byte</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-find-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-multipart-form-content-type">multipart-form-content-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-form-content-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-multipart-header-append">multipart-header-append</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-header-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a></code></li><li><a class="syn-name" href="#v-multipart-header-append-raw">multipart-header-append-raw</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-header-append-raw</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a></code></li><li><a class="syn-name" href="#v-multipart-header-drop">multipart-header-drop</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-header-drop</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a></code></li><li><a class="syn-name" href="#v-multipart-header-drop-raw">multipart-header-drop-raw</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-header-drop-raw</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a></code></li><li><a class="syn-name" href="#v-multipart-header-get">multipart-header-get</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-header-get</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-multipart-header-key">multipart-header-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-header-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-multipart-header-pairs">multipart-header-pairs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-header-pairs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-multipart-header-set">multipart-header-set</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-header-set</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a></code></li><li><a class="syn-name" href="#v-multipart-header-set-raw">multipart-header-set-raw</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-header-set-raw</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a></code></li><li><a class="syn-name" href="#v-multipart-header-size">multipart-header-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-header-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-multipart-header-values">multipart-header-values</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-header-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-multipart-header-wire">multipart-header-wire</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-header-wire</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-multipart-headers">multipart-headers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a></code></li><li><a class="syn-name" href="#v-multipart-parse-header-line">multipart-parse-header-line</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-parse-header-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartSyntax" title="Text.Multipart.MultipartSyntax">MultipartSyntax</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-multipart-posix-filename">multipart-posix-filename</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-posix-filename</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-multipart-prefix">multipart-prefix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-multipart-syntax-message">multipart-syntax-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-syntax-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartSyntax" title="Text.Multipart.MultipartSyntax">MultipartSyntax</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-multipart-trim-header-line">multipart-trim-header-line</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">multipart-trim-header-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-MultipartBodyScan">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MultipartBodyScan</strong></code><span class="doc-sig-links"><a href="#t-MultipartBodyScan" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L377" title="View source at line 377">Source</a></span></div>
<div class="doc-body">

A body scan separates confirmed payload from incomplete delimiters and end.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MultipartBodyPrefix"><code><strong class="tok-constructor">MultipartBodyPrefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartBodyScan" title="Text.Multipart.MultipartBodyScan">MultipartBodyScan</a></code></li><li id="k-MultipartBodyNeedMore"><code><strong class="tok-constructor">MultipartBodyNeedMore</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartBodyScan" title="Text.Multipart.MultipartBodyScan">MultipartBodyScan</a></code></li><li id="k-MultipartBodyEnd"><code><strong class="tok-constructor">MultipartBodyEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartBodyScan" title="Text.Multipart.MultipartBodyScan">MultipartBodyScan</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MultipartDelimiter">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MultipartDelimiter</strong></code><span class="doc-sig-links"><a href="#t-MultipartDelimiter" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L508" title="View source at line 508">Source</a></span></div>
<div class="doc-body">

Delimiter recognition retains the first ordinary line's newline policy.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MultipartNoDelimiter"><code><strong class="tok-constructor">MultipartNoDelimiter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartDelimiter" title="Text.Multipart.MultipartDelimiter">MultipartDelimiter</a></code></li><li id="k-MultipartNextDelimiter"><code><strong class="tok-constructor">MultipartNextDelimiter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartDelimiter" title="Text.Multipart.MultipartDelimiter">MultipartDelimiter</a></code></li><li id="k-MultipartFinalDelimiter"><code><strong class="tok-constructor">MultipartFinalDelimiter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartDelimiter" title="Text.Multipart.MultipartDelimiter">MultipartDelimiter</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MultipartHeaders">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MultipartHeaders</strong></code><span class="doc-sig-links"><a href="#t-MultipartHeaders" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L113" title="View source at line 113">Source</a></span></div>
<div class="doc-body">

Privately retained exact keys and ordered values.

</div>
</article>

<article class="doc-entry" id="t-MultipartSyntax">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MultipartSyntax</strong></code><span class="doc-sig-links"><a href="#t-MultipartSyntax" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L32" title="View source at line 32">Source</a></span></div>
<div class="doc-body">

Named protocol refusals retain the original offending wire bytes.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MultipartBoundaryLength"><code><strong class="tok-constructor">MultipartBoundaryLength</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartSyntax" title="Text.Multipart.MultipartSyntax">MultipartSyntax</a></code></li><li id="k-MultipartBoundaryCharacter"><code><strong class="tok-constructor">MultipartBoundaryCharacter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartSyntax" title="Text.Multipart.MultipartSyntax">MultipartSyntax</a></code></li><li id="k-MultipartHeaderInitial"><code><strong class="tok-constructor">MultipartHeaderInitial</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartSyntax" title="Text.Multipart.MultipartSyntax">MultipartSyntax</a></code></li><li id="k-MultipartHeaderMissingColon"><code><strong class="tok-constructor">MultipartHeaderMissingColon</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartSyntax" title="Text.Multipart.MultipartSyntax">MultipartSyntax</a></code></li><li id="k-MultipartHeaderMalformed"><code><strong class="tok-constructor">MultipartHeaderMalformed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartSyntax" title="Text.Multipart.MultipartSyntax">MultipartSyntax</a></code></li><li id="k-MultipartMessageTooLarge"><code><strong class="tok-constructor">MultipartMessageTooLarge</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartSyntax" title="Text.Multipart.MultipartSyntax">MultipartSyntax</a></code></li><li id="k-MultipartEmptyBoundary"><code><strong class="tok-constructor">MultipartEmptyBoundary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartSyntax" title="Text.Multipart.MultipartSyntax">MultipartSyntax</a></code></li><li id="k-MultipartUnexpectedLine"><code><strong class="tok-constructor">MultipartUnexpectedLine</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartSyntax" title="Text.Multipart.MultipartSyntax">MultipartSyntax</a></code></li><li id="k-MultipartExpectedPart"><code><strong class="tok-constructor">MultipartExpectedPart</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartSyntax" title="Text.Multipart.MultipartSyntax">MultipartSyntax</a></code></li><li id="k-MultipartBufferFull"><code><strong class="tok-constructor">MultipartBufferFull</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartSyntax" title="Text.Multipart.MultipartSyntax">MultipartSyntax</a></code></li><li id="k-MultipartUnexpectedEnd"><code><strong class="tok-constructor">MultipartUnexpectedEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartSyntax" title="Text.Multipart.MultipartSyntax">MultipartSyntax</a></code></li><li id="k-MultipartInputEnd"><code><strong class="tok-constructor">MultipartInputEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartSyntax" title="Text.Multipart.MultipartSyntax">MultipartSyntax</a></code></li><li id="k-MultipartNoProgress"><code><strong class="tok-constructor">MultipartNoProgress</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartSyntax" title="Text.Multipart.MultipartSyntax">MultipartSyntax</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-multipart-body-scan">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-body-scan</strong> :: <span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span> &lt;= 4096 <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg3</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg4</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartBodyScan" title="Text.Multipart.MultipartBodyScan">MultipartBodyScan</a></code><span class="doc-sig-links"><a href="#v-multipart-body-scan" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L435" title="View source at line 435">Source</a></span></div>
<div class="doc-body">

Observe one bounded raw-body span; terminal status decides ambiguous suffixes.

</div>
</article>

<article class="doc-entry" id="v-multipart-check-boundary">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-check-boundary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartSyntax" title="Text.Multipart.MultipartSyntax">MultipartSyntax</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-multipart-check-boundary" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L267" title="View source at line 267">Source</a></span></div>
<div class="doc-body">

Validate a writer boundary without imposing this restriction on readers.

</div>
</article>

<article class="doc-entry" id="v-multipart-delimiter-line">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-delimiter-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartDelimiter" title="Text.Multipart.MultipartDelimiter">MultipartDelimiter</a></code><span class="doc-sig-links"><a href="#v-multipart-delimiter-line" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L526" title="View source at line 526">Source</a></span></div>
<div class="doc-body">

Classify a complete line without validating a reader's arbitrary boundary.

</div>
</article>

<article class="doc-entry" id="v-multipart-field-disposition">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-field-disposition</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-multipart-field-disposition" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L314" title="View source at line 314">Source</a></span></div>
<div class="doc-body">

Render the exact disposition for a named value field.

</div>
</article>

<article class="doc-entry" id="v-multipart-file-disposition">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-file-disposition</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-multipart-file-disposition" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L318" title="View source at line 318">Source</a></span></div>
<div class="doc-body">

Render the exact disposition for a named file field.

</div>
</article>

<article class="doc-entry" id="v-multipart-find-byte">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-find-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-multipart-find-byte" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L360" title="View source at line 360">Source</a></span></div>
<div class="doc-body">

Return the first target byte at or after start, or the input length.

</div>
</article>

<article class="doc-entry" id="v-multipart-form-content-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-form-content-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-multipart-form-content-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L281" title="View source at line 281">Source</a></span></div>
<div class="doc-body">

Render a form content type with exactly the required boundary quoting.

</div>
</article>

<article class="doc-entry" id="v-multipart-header-append">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-header-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a></code><span class="doc-sig-links"><a href="#v-multipart-header-append" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L217" title="View source at line 217">Source</a></span></div>
<div class="doc-body">

Append a value under a canonicalized lookup key.

</div>
</article>

<article class="doc-entry" id="v-multipart-header-append-raw">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-header-append-raw</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a></code><span class="doc-sig-links"><a href="#v-multipart-header-append-raw" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L205" title="View source at line 205">Source</a></span></div>
<div class="doc-body">

Append an independently retained raw-key value without changing key spelling.

</div>
</article>

<article class="doc-entry" id="v-multipart-header-drop">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-header-drop</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a></code><span class="doc-sig-links"><a href="#v-multipart-header-drop" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L221" title="View source at line 221">Source</a></span></div>
<div class="doc-body">

Delete a canonicalized lookup key without touching differently spelled raw keys.

</div>
</article>

<article class="doc-entry" id="v-multipart-header-drop-raw">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-header-drop-raw</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a></code><span class="doc-sig-links"><a href="#v-multipart-header-drop-raw" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L209" title="View source at line 209">Source</a></span></div>
<div class="doc-body">

Delete only the exact raw-key entry.

</div>
</article>

<article class="doc-entry" id="v-multipart-header-get">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-header-get</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-multipart-header-get" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L197" title="View source at line 197">Source</a></span></div>
<div class="doc-body">

Read the first canonical-key value, using an empty buffer for absent/empty lists.

</div>
</article>

<article class="doc-entry" id="v-multipart-header-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-header-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-multipart-header-key" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L187" title="View source at line 187">Source</a></span></div>
<div class="doc-body">

Canonicalize token keys; invalid and space-containing raw keys stay exact.

</div>
</article>

<article class="doc-entry" id="v-multipart-header-pairs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-header-pairs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-multipart-header-pairs" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L160" title="View source at line 160">Source</a></span></div>
<div class="doc-body">

Observe sorted raw keys and original per-key value order in fresh storage.

</div>
</article>

<article class="doc-entry" id="v-multipart-header-set">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-header-set</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a></code><span class="doc-sig-links"><a href="#v-multipart-header-set" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L213" title="View source at line 213">Source</a></span></div>
<div class="doc-body">

Replace the values under a canonicalized lookup key.

</div>
</article>

<article class="doc-entry" id="v-multipart-header-set-raw">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-header-set-raw</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a></code><span class="doc-sig-links"><a href="#v-multipart-header-set-raw" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L201" title="View source at line 201">Source</a></span></div>
<div class="doc-body">

Replace an exact raw key with privately owned values.

</div>
</article>

<article class="doc-entry" id="v-multipart-header-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-header-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-multipart-header-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L233" title="View source at line 233">Source</a></span></div>
<div class="doc-body">

Exact form-budget charge: four hundred plus per-key and value overhead.

</div>
</article>

<article class="doc-entry" id="v-multipart-header-values">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-header-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-multipart-header-values" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L193" title="View source at line 193">Source</a></span></div>
<div class="doc-body">

Read every value for a canonicalized lookup key into fresh snapshots.

</div>
</article>

<article class="doc-entry" id="v-multipart-header-wire">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-header-wire</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-multipart-header-wire" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L251" title="View source at line 251">Source</a></span></div>
<div class="doc-body">

Serialize a complete header block with sorted raw keys and a final blank line.

</div>
</article>

<article class="doc-entry" id="v-multipart-headers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartHeaders" title="Text.Multipart.MultipartHeaders">MultipartHeaders</a></code><span class="doc-sig-links"><a href="#v-multipart-headers" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L147" title="View source at line 147">Source</a></span></div>
<div class="doc-body">

Construct exact-key headers, including empty lists and arbitrary raw keys.

</div>
</article>

<article class="doc-entry" id="v-multipart-parse-header-line">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-parse-header-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartSyntax" title="Text.Multipart.MultipartSyntax">MultipartSyntax</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-multipart-parse-header-line" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L477" title="View source at line 477">Source</a></span></div>
<div class="doc-body">

Parse one unfolded logical header line, preserving exact refused bytes.

</div>
</article>

<article class="doc-entry" id="v-multipart-posix-filename">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-posix-filename</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-multipart-posix-filename" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L500" title="View source at line 500">Source</a></span></div>
<div class="doc-body">

Apply the POSIX basename byte policy, including empty and all-separator cases.

</div>
</article>

<article class="doc-entry" id="v-multipart-prefix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-multipart-prefix" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L373" title="View source at line 373">Source</a></span></div>
<div class="doc-body">

Test a complete raw prefix without exposing retained bytes.

</div>
</article>

<article class="doc-entry" id="v-multipart-syntax-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-syntax-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Multipart/#t-MultipartSyntax" title="Text.Multipart.MultipartSyntax">MultipartSyntax</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-multipart-syntax-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

Render exact wire diagnostics while retaining the typed refusal.

</div>
</article>

<article class="doc-entry" id="v-multipart-trim-header-line">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">multipart-trim-header-line</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-multipart-trim-header-line" title="Permalink">#</a><a href="/packages/stdlib/Text.Multipart/source/#L458" title="View source at line 458">Source</a></span></div>
<div class="doc-body">

Trim only header space and tab, never Unicode whitespace or body bytes.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Multipart.cinch`](/packages/stdlib/Text.Multipart/source/). Source SHA-256: `743519460a0f29b66af76583a0d1780b450a876d57559dc603b195a7a7d96ae2`.
