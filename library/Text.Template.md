# Text.Template

Data-driven text templates: pipelines, control actions, definitions, builtins and printf over dynamic values.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Template</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Template/source/">View source</a><a href="/packages/search/?q=%2BText.Template">Search module</a><a href="/reference-sources/Text.Template.cinch" download="Text.Template.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Text.Show](Text.Show.md).

<details class="doc-closure"><summary>Ship-source closure (5 files)</summary>

Copy these modules beside your entry file before importing `Text.Template`: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Text.Template](Text.Template.md).

```sh
cp seed/prelude/Data/Bytes/Build.cinch ./Data.Bytes.Build.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Data/Unicode/Kernel.cinch ./Data.Unicode.Kernel.cinch
cp seed/prelude/Data/Unicode/Tables.cinch ./Data.Unicode.Tables.cinch
cp seed/prelude/Text/Template.cinch ./Text.Template.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Text templates: data-driven text with actions in delimiters, as in "Hello, {{.name}}!".
A template set holds a main template and the templates it defines. Text is
copied through; actions evaluate pipelines over a dynamic value (nil,
booleans, integers, floats, strings, lists and string-keyed maps) and print
the result. Actions cover fields and variables, function calls and
pipelines, if/else-if, range (lists, maps in key order, and integers, with
break and continue), with/else-with, template invocation, define and block,
comments, trim markers ("{{- " and " -}}") and custom delimiters. The
builtin functions are and, or, not, len, index, slice, print, printf,
println, eq, ne, lt, le, gt, ge, html, js and urlquery; printf follows the
usual verbs, flags, widths and precisions, with exactly rounded floats.
Further functions take dynamic values and may fail. The source is lexed,
then parsed into node spines, then executed by an explicit evaluator; the
search for delimiters and HTML escaping scan sixteen bytes at a time.

</details>

<details class="doc-synopsis"><summary>Synopsis (23 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-MissingKey">MissingKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MissingKey</strong></code></li><li><a class="syn-name" href="#t-Template">Template</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Template</strong></code></li><li><a class="syn-name" href="#t-TemplateError">TemplateError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TemplateError</strong></code></li><li><a class="syn-name" href="#t-TemplateFunction">TemplateFunction</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TemplateFunction</strong></code></li><li><a class="syn-name" href="#t-TemplateValue">TemplateValue</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TemplateValue</strong></code></li><li><a class="syn-name" href="#v-template-delims">template-delims</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-delims</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a></code></li><li><a class="syn-name" href="#v-template-error-message">template-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateError" title="Text.Template.TemplateError">TemplateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-template-execute">template-execute</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-execute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateError" title="Text.Template.TemplateError">TemplateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-template-execute-named">template-execute-named</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-execute-named</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateError" title="Text.Template.TemplateError">TemplateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-template-from-json">template-from-json</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-from-json</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a></code></li><li><a class="syn-name" href="#v-template-funcs">template-funcs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-funcs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateFunction" title="Text.Template.TemplateFunction">TemplateFunction</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a></code></li><li><a class="syn-name" href="#v-template-html-escape">template-html-escape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-html-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-template-is-true">template-is-true</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-is-true</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-template-js-escape">template-js-escape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-js-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-template-list">template-list</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-list</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a></code></li><li><a class="syn-name" href="#v-template-map">template-map</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-map</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a></code></li><li><a class="syn-name" href="#v-template-missing-key">template-missing-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-missing-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-MissingKey" title="Text.Template.MissingKey">MissingKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a></code></li><li><a class="syn-name" href="#v-template-name">template-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-template-names">template-names</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-names</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-template-new">template-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a></code></li><li><a class="syn-name" href="#v-template-parse">template-parse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateError" title="Text.Template.TemplateError">TemplateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a></code></li><li><a class="syn-name" href="#v-template-sprintf">template-sprintf</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-sprintf</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-template-url-query-escape">template-url-query-escape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-url-query-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-MissingKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MissingKey</strong></code><span class="doc-sig-links"><a href="#t-MissingKey" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L125" title="View source at line 125">Source</a></span></div>
<div class="doc-body">

What a missing map key evaluates to.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MissingKeyDefault"><code><strong class="tok-constructor">MissingKeyDefault</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-MissingKey" title="Text.Template.MissingKey">MissingKey</a></code></li><li id="k-MissingKeyZero"><code><strong class="tok-constructor">MissingKeyZero</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-MissingKey" title="Text.Template.MissingKey">MissingKey</a></code></li><li id="k-MissingKeyError"><code><strong class="tok-constructor">MissingKeyError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-MissingKey" title="Text.Template.MissingKey">MissingKey</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Template">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Template</strong></code><span class="doc-sig-links"><a href="#t-Template" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L141" title="View source at line 141">Source</a></span></div>
<div class="doc-body">

Fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Template"><code><strong class="tok-constructor">Template</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-MissingKey" title="Text.Template.MissingKey">MissingKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateFunction" title="Text.Template.TemplateFunction">TemplateFunction</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <span class="tok-constructor">Node</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-TemplateError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TemplateError</strong></code><span class="doc-sig-links"><a href="#t-TemplateError" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L114" title="View source at line 114">Source</a></span></div>
<div class="doc-body">

A failure: parsing ("template: name:line: problem") or executing ("template: name: problem").

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-TemplateParseError"><code><strong class="tok-constructor">TemplateParseError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateError" title="Text.Template.TemplateError">TemplateError</a></code></li><li id="k-TemplateExecError"><code><strong class="tok-constructor">TemplateExecError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateError" title="Text.Template.TemplateError">TemplateError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-TemplateFunction">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TemplateFunction</strong></code><span class="doc-sig-links"><a href="#t-TemplateFunction" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L136" title="View source at line 136">Source</a></span></div>
<div class="doc-body">

The function.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-TemplateFunction"><code><strong class="tok-constructor">TemplateFunction</strong> :: (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateFunction" title="Text.Template.TemplateFunction">TemplateFunction</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-TemplateValue">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TemplateValue</strong></code><span class="doc-sig-links"><a href="#t-TemplateValue" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L42" title="View source at line 42">Source</a></span></div>
<div class="doc-body">

A dynamic value: lists and maps are spines, and map entries are kept in increasing key order.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-TvNil"><code><strong class="tok-constructor">TvNil</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a></code></li><li id="k-TvBool"><code><strong class="tok-constructor">TvBool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a></code></li><li id="k-TvInt"><code><strong class="tok-constructor">TvInt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a></code></li><li id="k-TvFloat"><code><strong class="tok-constructor">TvFloat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a></code></li><li id="k-TvString"><code><strong class="tok-constructor">TvString</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a></code></li><li id="k-TvList"><code><strong class="tok-constructor">TvList</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a></code></li><li id="k-TvListEnd"><code><strong class="tok-constructor">TvListEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a></code></li><li id="k-TvMap"><code><strong class="tok-constructor">TvMap</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a></code></li><li id="k-TvMapEnd"><code><strong class="tok-constructor">TvMapEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-template-delims">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-delims</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a></code><span class="doc-sig-links"><a href="#v-template-delims" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L148" title="View source at line 148">Source</a></span></div>
<div class="doc-body">

Set the delimiters used by later parses; empty strings restore the defaults.

</div>
</article>

<article class="doc-entry" id="v-template-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateError" title="Text.Template.TemplateError">TemplateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-template-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L121" title="View source at line 121">Source</a></span></div>
<div class="doc-body">

Render an error.

</div>
</article>

<article class="doc-entry" id="v-template-execute">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-execute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateError" title="Text.Template.TemplateError">TemplateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-template-execute" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L223" title="View source at line 223">Source</a></span></div>
<div class="doc-body">

Execute the main template with data.

</div>
</article>

<article class="doc-entry" id="v-template-execute-named">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-execute-named</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateError" title="Text.Template.TemplateError">TemplateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-template-execute-named" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L227" title="View source at line 227">Source</a></span></div>
<div class="doc-body">

Execute a named template of the set with data.

</div>
</article>

<article class="doc-entry" id="v-template-from-json">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-from-json</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a></code><span class="doc-sig-links"><a href="#v-template-from-json" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L84" title="View source at line 84">Source</a></span></div>
<div class="doc-body">

Convert a JSON value: null, booleans, integers, floats, strings, arrays and objects.

</div>
</article>

<article class="doc-entry" id="v-template-funcs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-funcs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateFunction" title="Text.Template.TemplateFunction">TemplateFunction</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a></code><span class="doc-sig-links"><a href="#v-template-funcs" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L157" title="View source at line 157">Source</a></span></div>
<div class="doc-body">

Add functions (later entries replace earlier ones with the same name).

</div>
</article>

<article class="doc-entry" id="v-template-html-escape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-html-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-template-html-escape" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L1919" title="View source at line 1919">Source</a></span></div>
<div class="doc-body">

Escape for HTML text: quotes, "&", "<" and ">" become references and NUL becomes U+FFFD. Plain runs are found
sixteen bytes at a time.

</div>
</article>

<article class="doc-entry" id="v-template-is-true">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-is-true</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-template-is-true" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L97" title="View source at line 97">Source</a></span></div>
<div class="doc-body">

Truth: false for nil, false, zero, the empty string and empty lists and maps.

</div>
</article>

<article class="doc-entry" id="v-template-js-escape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-js-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-template-js-escape" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L1944" title="View source at line 1944">Source</a></span></div>
<div class="doc-body">

Escape for JavaScript strings: backslash and quotes get backslashes; "<", ">", "&", "=" and ASCII controls
become \u00XX; non-printable code points become \uXXXX.

</div>
</article>

<article class="doc-entry" id="v-template-list">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-list</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a></code><span class="doc-sig-links"><a href="#v-template-list" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L63" title="View source at line 63">Source</a></span></div>
<div class="doc-body">

A list value from elements in order.

</div>
</article>

<article class="doc-entry" id="v-template-map">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-map</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a></code><span class="doc-sig-links"><a href="#v-template-map" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L67" title="View source at line 67">Source</a></span></div>
<div class="doc-body">

A map value from entries in any order; a repeated key keeps its last value.

</div>
</article>

<article class="doc-entry" id="v-template-missing-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-missing-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-MissingKey" title="Text.Template.MissingKey">MissingKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a></code><span class="doc-sig-links"><a href="#v-template-missing-key" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L153" title="View source at line 153">Source</a></span></div>
<div class="doc-body">

Set the missing-key mode.

</div>
</article>

<article class="doc-entry" id="v-template-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-template-name" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L165" title="View source at line 165">Source</a></span></div>
<div class="doc-body">

The set's main name.

</div>
</article>

<article class="doc-entry" id="v-template-names">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-names</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-template-names" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L169" title="View source at line 169">Source</a></span></div>
<div class="doc-body">

Names of the templates defined so far.

</div>
</article>

<article class="doc-entry" id="v-template-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a></code><span class="doc-sig-links"><a href="#v-template-new" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L144" title="View source at line 144">Source</a></span></div>
<div class="doc-body">

An empty set named name, with "{{" and "}}" delimiters.

</div>
</article>

<article class="doc-entry" id="v-template-parse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateError" title="Text.Template.TemplateError">TemplateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-Template" title="Text.Template.Template">Template</a></code><span class="doc-sig-links"><a href="#v-template-parse" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L178" title="View source at line 178">Source</a></span></div>
<div class="doc-body">

Parse text as the main template, adding the templates it defines. A body that is only white space does not
replace a template that already has one.

</div>
</article>

<article class="doc-entry" id="v-template-sprintf">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-sprintf</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-template-sprintf" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L1513" title="View source at line 1513">Source</a></span></div>
<div class="doc-body">

Format arguments by a format string: verbs %v %d %b %o %O %x %X %c %q %U %e %E %f %F %g %G %s %t %T and %%,
flags "-+# 0", widths and precisions (including "*"). Problems are written in place as %!verb(type=value),
%!verb(MISSING), %!(NOVERB) and a trailing %!(EXTRA type=value, ...).

</div>
</article>

<article class="doc-entry" id="v-template-url-query-escape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-url-query-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-template-url-query-escape" title="Permalink">#</a><a href="/packages/stdlib/Text.Template/source/#L1966" title="View source at line 1966">Source</a></span></div>
<div class="doc-body">

Escape for a URL query component: letters, digits and "-_.~" stay, a space becomes "+", other bytes become %XX.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Template.cinch`](/packages/stdlib/Text.Template/source/). Source SHA-256: `c88aa762a5a75fd763c618f7aacebe7540ce8bc76b0f87e1472752c5b36a3fc6`.
