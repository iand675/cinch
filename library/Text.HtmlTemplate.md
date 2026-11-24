# Text.HtmlTemplate

Context-aware auto-escaping HTML template engine over Text.Template and Text.Html with typed safe content wrappers for HTML, attributes, URLs, JS, CSS and srcset.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.HtmlTemplate`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.HtmlTemplate.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Text.Html](Text.Html.md), [Text.Show](Text.Show.md), [Text.Template](Text.Template.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Text.Html](Text.Html.md), [Text.HtmlTemplate](Text.HtmlTemplate.md), [Text.Template](Text.Template.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-48746d6c54656d706c617465"></a>

## `HtmlTemplate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HtmlTemplate</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 195

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HtmlTemplate</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">MissingKey</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplateFunction</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">Node</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplate</span>))))))</code></pre>

<a id="type-48746d6c54656d706c6174654572726f72"></a>

## `HtmlTemplateError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HtmlTemplateError</span></code></pre>

A template failure: syntax parsing, contextual escaping analysis, or runtime execution.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 172

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HtmlTemplateParseError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplateError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HtmlTemplateEscapeError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplateError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HtmlTemplateExecError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplateError</span>)</code></pre>

<a id="type-48746d6c54656d706c61746546756e6374696f6e"></a>

## `HtmlTemplateFunction`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HtmlTemplateFunction</span></code></pre>

Function implementation over `HtmlValue` arguments.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 190

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HtmlTemplateFunction</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplateFunction</span>)</code></pre>

<a id="type-48746d6c56616c7565"></a>

## `HtmlValue`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HtmlValue</span></code></pre>

A dynamic value for HTML templates: plain `TemplateValue` payloads,
typed safe strings, and lists or sorted string-keyed maps of `HtmlValue`.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 48

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HvTemplate</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HvSafeHtml</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HvSafeHtmlAttr</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HvSafeCss</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HvSafeJs</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HvSafeJsStr</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HvSafeUrl</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HvSafeSrcset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HvList</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HvListEnd</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HvMap</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HvMapEnd</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span></code></pre>

# Values

<a id="value-68746d6c2d6573636170652d737472696e67"></a>

## `html-escape-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-escape-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Escape plain text for HTML text nodes (`"`, `'`, `&`, `<`, `>`, and NUL to U+FFFD).

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 234

<a id="value-68746d6c2d66726f6d2d74656d706c617465"></a>

## `html-from-template`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-from-template</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)</code></pre>

Wrap a `TemplateValue` as an untrusted `HtmlValue`.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 75

<a id="value-68746d6c2d69732d74727565"></a>

## `html-is-true`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-is-true</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Truthiness: false for nil, false, zero, empty strings, and empty lists or maps.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 152

<a id="value-68746d6c2d6c697374"></a>

## `html-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-list</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)</code></pre>

Build an `HtmlValue` list spine from elements in order.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 123

<a id="value-68746d6c2d6d6170"></a>

## `html-map`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-map</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)</code></pre>

Build an `HtmlValue` map spine with keys kept in ascending byte order; repeated keys keep their last value.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 129

<a id="value-68746d6c2d74656d706c6174652d64656c696d73"></a>

## `html-template-delims`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-template-delims</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplate</span>)))</code></pre>

Configure action delimiters for subsequent parses; empty strings restore `{{` and `}}`.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 202

<a id="value-68746d6c2d74656d706c6174652d6572726f722d6d657373616765"></a>

## `html-template-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-template-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplateError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Extract the diagnostic message from an `HtmlTemplateError`.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 181

<a id="value-68746d6c2d74656d706c6174652d65786563757465"></a>

## `html-template-execute`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-template-execute</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplate</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplateError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Contextually escape and execute the main template with `data-value`.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 295

<a id="value-68746d6c2d74656d706c6174652d657865637574652d6e616d6564"></a>

## `html-template-execute-named`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-template-execute-named</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplateError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Contextually escape and execute a named template from the set with `data-value`.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 299

<a id="value-68746d6c2d74656d706c6174652d66756e6373"></a>

## `html-template-funcs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-template-funcs</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplate</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplateFunction</span>)) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplate</span>))</code></pre>

Register custom functions (later entries shadow earlier ones of the same name).

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 213

<a id="value-68746d6c2d74656d706c6174652d6d697373696e672d6b6579"></a>

## `html-template-missing-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-template-missing-key</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplate</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">MissingKey</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplate</span>))</code></pre>

Set the missing-key mode for map field lookups.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 208

<a id="value-68746d6c2d74656d706c6174652d6e616d65"></a>

## `html-template-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-template-name</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplate</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The main template name of the set.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 222

<a id="value-68746d6c2d74656d706c6174652d6e616d6573"></a>

## `html-template-names`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-template-names</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

The names of all templates currently defined in the set.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 226

<a id="value-68746d6c2d74656d706c6174652d6e6577"></a>

## `html-template-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-template-new</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplate</span>)</code></pre>

Create an empty HTML template set with default `{{` and `}}` delimiters.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 198

<a id="value-68746d6c2d74656d706c6174652d7061727365"></a>

## `html-template-parse`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-template-parse</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplateError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlTemplate</span>)))</code></pre>

Parse template source into the set's main template and any `define`/`block` templates it declares.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 246

<a id="value-68746d6c2d746f2d74656d706c617465"></a>

## `html-to-template`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-to-template</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span>)</code></pre>

Strip safe content wrappers and convert an `HtmlValue` down to a plain `TemplateValue`.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 79

<a id="value-68746d6c2d76616c75652d66726f6d2d6a736f6e"></a>

## `html-value-from-json`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">html-value-from-json</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)</code></pre>

Convert a `Json` value into an untrusted `HtmlValue`.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 148

<a id="value-6a732d6573636170652d737472696e67"></a>

## `js-escape-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">js-escape-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Escape plain text for JavaScript string literals.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 238

<a id="value-736166652d637373"></a>

## `safe-css`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">safe-css</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)</code></pre>

Construct a known-safe CSS value.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 103

<a id="value-736166652d68746d6c"></a>

## `safe-html`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">safe-html</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)</code></pre>

Construct a known-safe HTML fragment value.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 95

<a id="value-736166652d68746d6c2d61747472"></a>

## `safe-html-attr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">safe-html-attr</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)</code></pre>

Construct a known-safe HTML attribute value.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 99

<a id="value-736166652d6a73"></a>

## `safe-js`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">safe-js</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)</code></pre>

Construct a known-safe JavaScript expression value.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 107

<a id="value-736166652d6a732d737472"></a>

## `safe-js-str`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">safe-js-str</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)</code></pre>

Construct a known-safe JavaScript string payload value.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 111

<a id="value-736166652d737263736574"></a>

## `safe-srcset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">safe-srcset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)</code></pre>

Construct a known-safe `srcset` attribute value.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 119

<a id="value-736166652d75726c"></a>

## `safe-url`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">safe-url</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">HtmlTemplate</span>.<span class="tok-constructor">HtmlValue</span>)</code></pre>

Construct a known-safe URL value.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 115

<a id="value-75726c2d71756572792d6573636170652d737472696e67"></a>

## `url-query-escape-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-query-escape-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Escape plain text for URL query components.

[Source](/reference-sources/Text.HtmlTemplate.cinch) · line 242

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/HtmlTemplate.cinch`. Source SHA-256: `b59039a0dfcfa93792a7ba02c39b5253ccb713a813ee83ee5a7ea4b46a3f61ae`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
