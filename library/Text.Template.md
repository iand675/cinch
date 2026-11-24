# Text.Template

Data-driven text templates: pipelines, control actions, definitions, builtins and printf over dynamic values.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Template`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Template.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Text.Template](Text.Template.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-4d697373696e674b6579"></a>

## `MissingKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MissingKey</span></code></pre>

What a missing map key evaluates to.

[Source](/reference-sources/Text.Template.cinch) · line 125

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MissingKeyDefault</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">MissingKey</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MissingKeyZero</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">MissingKey</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MissingKeyError</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">MissingKey</span></code></pre>

<a id="type-54656d706c617465"></a>

## `Template`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Template</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Text.Template.cinch) · line 141

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Template</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">MissingKey</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateFunction</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">Node</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">Template</span>))))))</code></pre>

<a id="type-54656d706c6174654572726f72"></a>

## `TemplateError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TemplateError</span></code></pre>

A failure: parsing ("template: name:line: problem") or executing ("template: name: problem").

[Source](/reference-sources/Text.Template.cinch) · line 114

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TemplateParseError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TemplateExecError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateError</span>)</code></pre>

<a id="type-54656d706c61746546756e6374696f6e"></a>

## `TemplateFunction`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TemplateFunction</span></code></pre>

The function.

[Source](/reference-sources/Text.Template.cinch) · line 136

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TemplateFunction</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateFunction</span>)</code></pre>

<a id="type-54656d706c61746556616c7565"></a>

## `TemplateValue`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TemplateValue</span></code></pre>

A dynamic value: lists and maps are spines, and map entries are kept in increasing key order.

[Source](/reference-sources/Text.Template.cinch) · line 42

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TvNil</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TvBool</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TvInt</span> :: (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TvFloat</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TvString</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TvList</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TvListEnd</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TvMap</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span>)))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TvMapEnd</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span></code></pre>

# Values

<a id="value-74656d706c6174652d64656c696d73"></a>

## `template-delims`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-delims</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">Template</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">Template</span>)))</code></pre>

Set the delimiters used by later parses; empty strings restore the defaults.

[Source](/reference-sources/Text.Template.cinch) · line 148

<a id="value-74656d706c6174652d6572726f722d6d657373616765"></a>

## `template-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an error.

[Source](/reference-sources/Text.Template.cinch) · line 121

<a id="value-74656d706c6174652d65786563757465"></a>

## `template-execute`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-execute</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">Template</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Execute the main template with data.

[Source](/reference-sources/Text.Template.cinch) · line 223

<a id="value-74656d706c6174652d657865637574652d6e616d6564"></a>

## `template-execute-named`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-execute-named</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">Template</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Execute a named template of the set with data.

[Source](/reference-sources/Text.Template.cinch) · line 227

<a id="value-74656d706c6174652d66726f6d2d6a736f6e"></a>

## `template-from-json`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-from-json</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span>)</code></pre>

Convert a JSON value: null, booleans, integers, floats, strings, arrays and objects.

[Source](/reference-sources/Text.Template.cinch) · line 84

<a id="value-74656d706c6174652d66756e6373"></a>

## `template-funcs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-funcs</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">Template</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateFunction</span>)) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">Template</span>))</code></pre>

Add functions (later entries replace earlier ones with the same name).

[Source](/reference-sources/Text.Template.cinch) · line 157

<a id="value-74656d706c6174652d68746d6c2d657363617065"></a>

## `template-html-escape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-html-escape</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Escape for HTML text: quotes, "&", "<" and ">" become references and NUL becomes U+FFFD. Plain runs are found
sixteen bytes at a time.

[Source](/reference-sources/Text.Template.cinch) · line 1919

<a id="value-74656d706c6174652d69732d74727565"></a>

## `template-is-true`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-is-true</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Truth: false for nil, false, zero, the empty string and empty lists and maps.

[Source](/reference-sources/Text.Template.cinch) · line 97

<a id="value-74656d706c6174652d6a732d657363617065"></a>

## `template-js-escape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-js-escape</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Escape for JavaScript strings: backslash and quotes get backslashes; "<", ">", "&", "=" and ASCII controls
become \u00XX; non-printable code points become \uXXXX.

[Source](/reference-sources/Text.Template.cinch) · line 1944

<a id="value-74656d706c6174652d6c697374"></a>

## `template-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-list</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span>)</code></pre>

A list value from elements in order.

[Source](/reference-sources/Text.Template.cinch) · line 63

<a id="value-74656d706c6174652d6d6170"></a>

## `template-map`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-map</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span>)) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span>)</code></pre>

A map value from entries in any order; a repeated key keeps its last value.

[Source](/reference-sources/Text.Template.cinch) · line 67

<a id="value-74656d706c6174652d6d697373696e672d6b6579"></a>

## `template-missing-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-missing-key</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">Template</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">MissingKey</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">Template</span>))</code></pre>

Set the missing-key mode.

[Source](/reference-sources/Text.Template.cinch) · line 153

<a id="value-74656d706c6174652d6e616d65"></a>

## `template-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-name</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">Template</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The set's main name.

[Source](/reference-sources/Text.Template.cinch) · line 165

<a id="value-74656d706c6174652d6e616d6573"></a>

## `template-names`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-names</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">Template</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Names of the templates defined so far.

[Source](/reference-sources/Text.Template.cinch) · line 169

<a id="value-74656d706c6174652d6e6577"></a>

## `template-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-new</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">Template</span>)</code></pre>

An empty set named name, with "{{" and "}}" delimiters.

[Source](/reference-sources/Text.Template.cinch) · line 144

<a id="value-74656d706c6174652d7061727365"></a>

## `template-parse`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-parse</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">Template</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">Template</span>)))</code></pre>

Parse text as the main template, adding the templates it defines. A body that is only white space does not
replace a template that already has one.

[Source](/reference-sources/Text.Template.cinch) · line 178

<a id="value-74656d706c6174652d737072696e7466"></a>

## `template-sprintf`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-sprintf</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Template</span>.<span class="tok-constructor">TemplateValue</span>) -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Format arguments by a format string: verbs %v %d %b %o %O %x %X %c %q %U %e %E %f %F %g %G %s %t %T and %%,
flags "-+# 0", widths and precisions (including "*"). Problems are written in place as %!verb(type=value),
%!verb(MISSING), %!(NOVERB) and a trailing %!(EXTRA type=value, ...).

[Source](/reference-sources/Text.Template.cinch) · line 1513

<a id="value-74656d706c6174652d75726c2d71756572792d657363617065"></a>

## `template-url-query-escape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-url-query-escape</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Escape for a URL query component: letters, digits and "-_.~" stay, a space becomes "+", other bytes become %XX.

[Source](/reference-sources/Text.Template.cinch) · line 1966

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Template.cinch`. Source SHA-256: `c88aa762a5a75fd763c618f7aacebe7540ce8bc76b0f87e1472752c5b36a3fc6`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
