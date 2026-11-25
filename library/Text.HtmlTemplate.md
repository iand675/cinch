# Text.HtmlTemplate

Context-aware auto-escaping HTML template engine over Text.Template and Text.Html with typed safe content wrappers for HTML, attributes, URLs, JS, CSS and srcset.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.HtmlTemplate</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.HtmlTemplate/source/">View source</a><a href="/packages/search/?q=%2BText.HtmlTemplate">Search module</a><a href="/reference-sources/Text.HtmlTemplate.cinch" download="Text.HtmlTemplate.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Text.Html](Text.Html.md), [Text.Show](Text.Show.md), [Text.Template](Text.Template.md).

<details class="doc-notes"><summary>Module notes</summary>

Contextual HTML templates: data-driven HTML documents with automatic,
context-aware escaping to prevent cross-site scripting and injection attacks.
Templates share the syntax and control flow of `Text.Template` (fields,
variables, pipelines, `if`/`else if`, `range` with `break` and `continue`,
`with`/`else with`, `define`, `block`, `template`, trim markers, comments,
custom delimiters, missing-key modes, builtins, and custom functions) while
tracking the exact HTML, attribute, URL, srcset, JavaScript, CSS, RCDATA, and
comment parsing state at every action and static text span.

Values are represented by `HtmlValue`, which wraps `TemplateValue` along with
typed safe wrappers (`HvSafeHtml`, `HvSafeHtmlAttr`, `HvSafeCss`, `HvSafeJs`,
`HvSafeJsStr`, `HvSafeUrl`, `HvSafeSrcset`) and nested `HvList`/`HvMap`
containers so trusted content bypasses only the escaping layer that matches
its declared type. All lexing, parsing, contextual analysis, escaping, and
evaluation are implemented in pure portable code with zero inline LLVM.

</details>

<details class="doc-synopsis"><summary>Synopsis (30 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-HtmlTemplate">HtmlTemplate</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HtmlTemplate</strong></code></li><li><a class="syn-name" href="#t-HtmlTemplateError">HtmlTemplateError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HtmlTemplateError</strong></code></li><li><a class="syn-name" href="#t-HtmlTemplateFunction">HtmlTemplateFunction</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HtmlTemplateFunction</strong></code></li><li><a class="syn-name" href="#t-HtmlValue">HtmlValue</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HtmlValue</strong></code></li><li><a class="syn-name" href="#v-html-escape-string">html-escape-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-escape-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-html-from-template">html-from-template</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-from-template</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li><a class="syn-name" href="#v-html-is-true">html-is-true</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-is-true</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-html-list">html-list</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-list</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li><a class="syn-name" href="#v-html-map">html-map</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-map</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li><a class="syn-name" href="#v-html-template-delims">html-template-delims</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-template-delims</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a></code></li><li><a class="syn-name" href="#v-html-template-error-message">html-template-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-template-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplateError" title="Text.HtmlTemplate.HtmlTemplateError">HtmlTemplateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-html-template-execute">html-template-execute</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-template-execute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplateError" title="Text.HtmlTemplate.HtmlTemplateError">HtmlTemplateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-html-template-execute-named">html-template-execute-named</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-template-execute-named</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplateError" title="Text.HtmlTemplate.HtmlTemplateError">HtmlTemplateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-html-template-funcs">html-template-funcs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-template-funcs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplateFunction" title="Text.HtmlTemplate.HtmlTemplateFunction">HtmlTemplateFunction</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a></code></li><li><a class="syn-name" href="#v-html-template-missing-key">html-template-missing-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-template-missing-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-MissingKey" title="Text.Template.MissingKey">MissingKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a></code></li><li><a class="syn-name" href="#v-html-template-name">html-template-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-template-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-html-template-names">html-template-names</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-template-names</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-html-template-new">html-template-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-template-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a></code></li><li><a class="syn-name" href="#v-html-template-parse">html-template-parse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-template-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplateError" title="Text.HtmlTemplate.HtmlTemplateError">HtmlTemplateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a></code></li><li><a class="syn-name" href="#v-html-to-template">html-to-template</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-to-template</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a></code></li><li><a class="syn-name" href="#v-html-value-from-json">html-value-from-json</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">html-value-from-json</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li><a class="syn-name" href="#v-js-escape-string">js-escape-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">js-escape-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-safe-css">safe-css</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">safe-css</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li><a class="syn-name" href="#v-safe-html">safe-html</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">safe-html</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li><a class="syn-name" href="#v-safe-html-attr">safe-html-attr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">safe-html-attr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li><a class="syn-name" href="#v-safe-js">safe-js</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">safe-js</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li><a class="syn-name" href="#v-safe-js-str">safe-js-str</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">safe-js-str</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li><a class="syn-name" href="#v-safe-srcset">safe-srcset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">safe-srcset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li><a class="syn-name" href="#v-safe-url">safe-url</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">safe-url</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li><a class="syn-name" href="#v-url-query-escape-string">url-query-escape-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-query-escape-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-HtmlTemplate">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HtmlTemplate</strong></code><span class="doc-sig-links"><a href="#t-HtmlTemplate" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L195" title="View source at line 195">Source</a></span></div>
<div class="doc-body">

Fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HtmlTemplate"><code><strong class="tok-constructor">HtmlTemplate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-MissingKey" title="Text.Template.MissingKey">MissingKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplateFunction" title="Text.HtmlTemplate.HtmlTemplateFunction">HtmlTemplateFunction</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <span class="tok-constructor">Node</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-HtmlTemplateError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HtmlTemplateError</strong></code><span class="doc-sig-links"><a href="#t-HtmlTemplateError" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L172" title="View source at line 172">Source</a></span></div>
<div class="doc-body">

A template failure: syntax parsing, contextual escaping analysis, or runtime execution.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HtmlTemplateParseError"><code><strong class="tok-constructor">HtmlTemplateParseError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplateError" title="Text.HtmlTemplate.HtmlTemplateError">HtmlTemplateError</a></code></li><li id="k-HtmlTemplateEscapeError"><code><strong class="tok-constructor">HtmlTemplateEscapeError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplateError" title="Text.HtmlTemplate.HtmlTemplateError">HtmlTemplateError</a></code></li><li id="k-HtmlTemplateExecError"><code><strong class="tok-constructor">HtmlTemplateExecError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplateError" title="Text.HtmlTemplate.HtmlTemplateError">HtmlTemplateError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-HtmlTemplateFunction">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HtmlTemplateFunction</strong></code><span class="doc-sig-links"><a href="#t-HtmlTemplateFunction" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L190" title="View source at line 190">Source</a></span></div>
<div class="doc-body">

Function implementation over `HtmlValue` arguments.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HtmlTemplateFunction"><code><strong class="tok-constructor">HtmlTemplateFunction</strong> :: (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplateFunction" title="Text.HtmlTemplate.HtmlTemplateFunction">HtmlTemplateFunction</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-HtmlValue">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HtmlValue</strong></code><span class="doc-sig-links"><a href="#t-HtmlValue" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L48" title="View source at line 48">Source</a></span></div>
<div class="doc-body">

A dynamic value for HTML templates: plain `TemplateValue` payloads,
typed safe strings, and lists or sorted string-keyed maps of `HtmlValue`.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HvTemplate"><code><strong class="tok-constructor">HvTemplate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li id="k-HvSafeHtml"><code><strong class="tok-constructor">HvSafeHtml</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li id="k-HvSafeHtmlAttr"><code><strong class="tok-constructor">HvSafeHtmlAttr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li id="k-HvSafeCss"><code><strong class="tok-constructor">HvSafeCss</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li id="k-HvSafeJs"><code><strong class="tok-constructor">HvSafeJs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li id="k-HvSafeJsStr"><code><strong class="tok-constructor">HvSafeJsStr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li id="k-HvSafeUrl"><code><strong class="tok-constructor">HvSafeUrl</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li id="k-HvSafeSrcset"><code><strong class="tok-constructor">HvSafeSrcset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li id="k-HvList"><code><strong class="tok-constructor">HvList</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li id="k-HvListEnd"><code><strong class="tok-constructor">HvListEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li id="k-HvMap"><code><strong class="tok-constructor">HvMap</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li><li id="k-HvMapEnd"><code><strong class="tok-constructor">HvMapEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-html-escape-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-escape-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-html-escape-string" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L234" title="View source at line 234">Source</a></span></div>
<div class="doc-body">

Escape plain text for HTML text nodes (`"`, `'`, `&`, `<`, `>`, and NUL to U+FFFD).

</div>
</article>

<article class="doc-entry" id="v-html-from-template">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-from-template</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code><span class="doc-sig-links"><a href="#v-html-from-template" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L75" title="View source at line 75">Source</a></span></div>
<div class="doc-body">

Wrap a `TemplateValue` as an untrusted `HtmlValue`.

</div>
</article>

<article class="doc-entry" id="v-html-is-true">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-is-true</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-html-is-true" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L152" title="View source at line 152">Source</a></span></div>
<div class="doc-body">

Truthiness: false for nil, false, zero, empty strings, and empty lists or maps.

</div>
</article>

<article class="doc-entry" id="v-html-list">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-list</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code><span class="doc-sig-links"><a href="#v-html-list" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L123" title="View source at line 123">Source</a></span></div>
<div class="doc-body">

Build an `HtmlValue` list spine from elements in order.

</div>
</article>

<article class="doc-entry" id="v-html-map">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-map</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code><span class="doc-sig-links"><a href="#v-html-map" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L129" title="View source at line 129">Source</a></span></div>
<div class="doc-body">

Build an `HtmlValue` map spine with keys kept in ascending byte order; repeated keys keep their last value.

</div>
</article>

<article class="doc-entry" id="v-html-template-delims">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-template-delims</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a></code><span class="doc-sig-links"><a href="#v-html-template-delims" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L202" title="View source at line 202">Source</a></span></div>
<div class="doc-body">

Configure action delimiters for subsequent parses; empty strings restore `{{` and `}}`.

</div>
</article>

<article class="doc-entry" id="v-html-template-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-template-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplateError" title="Text.HtmlTemplate.HtmlTemplateError">HtmlTemplateError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-html-template-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L181" title="View source at line 181">Source</a></span></div>
<div class="doc-body">

Extract the diagnostic message from an `HtmlTemplateError`.

</div>
</article>

<article class="doc-entry" id="v-html-template-execute">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-template-execute</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplateError" title="Text.HtmlTemplate.HtmlTemplateError">HtmlTemplateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-html-template-execute" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L295" title="View source at line 295">Source</a></span></div>
<div class="doc-body">

Contextually escape and execute the main template with `data-value`.

</div>
</article>

<article class="doc-entry" id="v-html-template-execute-named">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-template-execute-named</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplateError" title="Text.HtmlTemplate.HtmlTemplateError">HtmlTemplateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-html-template-execute-named" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L299" title="View source at line 299">Source</a></span></div>
<div class="doc-body">

Contextually escape and execute a named template from the set with `data-value`.

</div>
</article>

<article class="doc-entry" id="v-html-template-funcs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-template-funcs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplateFunction" title="Text.HtmlTemplate.HtmlTemplateFunction">HtmlTemplateFunction</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a></code><span class="doc-sig-links"><a href="#v-html-template-funcs" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L213" title="View source at line 213">Source</a></span></div>
<div class="doc-body">

Register custom functions (later entries shadow earlier ones of the same name).

</div>
</article>

<article class="doc-entry" id="v-html-template-missing-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-template-missing-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-MissingKey" title="Text.Template.MissingKey">MissingKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a></code><span class="doc-sig-links"><a href="#v-html-template-missing-key" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L208" title="View source at line 208">Source</a></span></div>
<div class="doc-body">

Set the missing-key mode for map field lookups.

</div>
</article>

<article class="doc-entry" id="v-html-template-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-template-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-html-template-name" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L222" title="View source at line 222">Source</a></span></div>
<div class="doc-body">

The main template name of the set.

</div>
</article>

<article class="doc-entry" id="v-html-template-names">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-template-names</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-html-template-names" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L226" title="View source at line 226">Source</a></span></div>
<div class="doc-body">

The names of all templates currently defined in the set.

</div>
</article>

<article class="doc-entry" id="v-html-template-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-template-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a></code><span class="doc-sig-links"><a href="#v-html-template-new" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L198" title="View source at line 198">Source</a></span></div>
<div class="doc-body">

Create an empty HTML template set with default `{{` and `}}` delimiters.

</div>
</article>

<article class="doc-entry" id="v-html-template-parse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-template-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplateError" title="Text.HtmlTemplate.HtmlTemplateError">HtmlTemplateError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlTemplate" title="Text.HtmlTemplate.HtmlTemplate">HtmlTemplate</a></code><span class="doc-sig-links"><a href="#v-html-template-parse" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L246" title="View source at line 246">Source</a></span></div>
<div class="doc-body">

Parse template source into the set's main template and any `define`/`block` templates it declares.

</div>
</article>

<article class="doc-entry" id="v-html-to-template">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-to-template</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Template/#t-TemplateValue" title="Text.Template.TemplateValue">TemplateValue</a></code><span class="doc-sig-links"><a href="#v-html-to-template" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L79" title="View source at line 79">Source</a></span></div>
<div class="doc-body">

Strip safe content wrappers and convert an `HtmlValue` down to a plain `TemplateValue`.

</div>
</article>

<article class="doc-entry" id="v-html-value-from-json">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">html-value-from-json</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code><span class="doc-sig-links"><a href="#v-html-value-from-json" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L148" title="View source at line 148">Source</a></span></div>
<div class="doc-body">

Convert a `Json` value into an untrusted `HtmlValue`.

</div>
</article>

<article class="doc-entry" id="v-js-escape-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">js-escape-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-js-escape-string" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L238" title="View source at line 238">Source</a></span></div>
<div class="doc-body">

Escape plain text for JavaScript string literals.

</div>
</article>

<article class="doc-entry" id="v-safe-css">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">safe-css</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code><span class="doc-sig-links"><a href="#v-safe-css" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L103" title="View source at line 103">Source</a></span></div>
<div class="doc-body">

Construct a known-safe CSS value.

</div>
</article>

<article class="doc-entry" id="v-safe-html">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">safe-html</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code><span class="doc-sig-links"><a href="#v-safe-html" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L95" title="View source at line 95">Source</a></span></div>
<div class="doc-body">

Construct a known-safe HTML fragment value.

</div>
</article>

<article class="doc-entry" id="v-safe-html-attr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">safe-html-attr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code><span class="doc-sig-links"><a href="#v-safe-html-attr" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L99" title="View source at line 99">Source</a></span></div>
<div class="doc-body">

Construct a known-safe HTML attribute value.

</div>
</article>

<article class="doc-entry" id="v-safe-js">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">safe-js</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code><span class="doc-sig-links"><a href="#v-safe-js" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L107" title="View source at line 107">Source</a></span></div>
<div class="doc-body">

Construct a known-safe JavaScript expression value.

</div>
</article>

<article class="doc-entry" id="v-safe-js-str">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">safe-js-str</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code><span class="doc-sig-links"><a href="#v-safe-js-str" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L111" title="View source at line 111">Source</a></span></div>
<div class="doc-body">

Construct a known-safe JavaScript string payload value.

</div>
</article>

<article class="doc-entry" id="v-safe-srcset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">safe-srcset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code><span class="doc-sig-links"><a href="#v-safe-srcset" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L119" title="View source at line 119">Source</a></span></div>
<div class="doc-body">

Construct a known-safe `srcset` attribute value.

</div>
</article>

<article class="doc-entry" id="v-safe-url">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">safe-url</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.HtmlTemplate/#t-HtmlValue" title="Text.HtmlTemplate.HtmlValue">HtmlValue</a></code><span class="doc-sig-links"><a href="#v-safe-url" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L115" title="View source at line 115">Source</a></span></div>
<div class="doc-body">

Construct a known-safe URL value.

</div>
</article>

<article class="doc-entry" id="v-url-query-escape-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-query-escape-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-url-query-escape-string" title="Permalink">#</a><a href="/packages/stdlib/Text.HtmlTemplate/source/#L242" title="View source at line 242">Source</a></span></div>
<div class="doc-body">

Escape plain text for URL query components.

</div>
</article>
