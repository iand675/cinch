# Text.Json.Value

Owned JSON documents and navigation tapes, raw number spellings, exact numeric values and input-bearing errors, resumable lexical state and byte-preserving formatting.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Json.Value</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Json.Value/source/">View source</a><a href="/packages/search/?q=%2BText.Json.Value">Search module</a><a href="/reference-sources/Text.Json.Value.cinch" download="Text.Json.Value.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Array](Data.Array.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Wide](Data.Wide.md), [Text.Json.Binary32](Text.Json.Binary32.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

Exact JSON syntax, immutable source-backed values and byte-preserving formatting.
Documents own one byte snapshot and an immutable navigation tape. Returned
byte buffers are independent copies; scalar strings replace malformed UTF-8
and unmatched surrogate escapes while number spellings remain exact.

</details>

<details class="doc-synopsis"><summary>Synopsis (137 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-JsonDocument">JsonDocument</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonDocument</strong></code></li><li><a class="syn-name" href="#t-JsonError">JsonError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonError</strong></code></li><li><a class="syn-name" href="#t-JsonKind">JsonKind</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonKind</strong></code></li><li><a class="syn-name" href="#t-JsonLexState">JsonLexState</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonLexState</strong></code></li><li><a class="syn-name" href="#t-JsonLexStep">JsonLexStep</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonLexStep</strong></code></li><li><a class="syn-name" href="#t-JsonLexTag">JsonLexTag</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonLexTag</strong></code></li><li><a class="syn-name" href="#t-JsonLexeme">JsonLexeme</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonLexeme</strong></code></li><li><a class="syn-name" href="#t-JsonMember">JsonMember</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonMember</strong></code></li><li><a class="syn-name" href="#t-JsonNumber">JsonNumber</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonNumber</strong></code></li><li><a class="syn-name" href="#t-JsonNumericError">JsonNumericError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonNumericError</strong></code></li><li><a class="syn-name" href="#t-JsonNumericErrorKind">JsonNumericErrorKind</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonNumericErrorKind</strong></code></li><li><a class="syn-name" href="#t-JsonNumericResult">JsonNumericResult</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonNumericResult</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#t-JsonRawMessage">JsonRawMessage</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonRawMessage</strong></code></li><li><a class="syn-name" href="#t-JsonScanResult">JsonScanResult</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonScanResult</strong></code></li><li><a class="syn-name" href="#t-JsonSemanticInfo">JsonSemanticInfo</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonSemanticInfo</strong></code></li><li><a class="syn-name" href="#t-JsonSyntacticInfo">JsonSyntacticInfo</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonSyntacticInfo</strong></code></li><li><a class="syn-name" href="#t-JsonTypeInfo">JsonTypeInfo</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonTypeInfo</strong></code></li><li><a class="syn-name" href="#t-JsonValue">JsonValue</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonValue</strong></code></li><li><a class="syn-name" href="#t-JsonWriteInfo">JsonWriteInfo</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonWriteInfo</strong></code></li><li><a class="syn-name" href="#v-json-append-compact">json-append-compact</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-append-compact</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-append-html-escape">json-append-html-escape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-append-html-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-append-indent">json-append-indent</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-append-indent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-append-indent-bytes">json-append-indent-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-append-indent-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-array-get">json-array-get</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-array-get</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a></code></li><li><a class="syn-name" href="#v-json-array-length">json-array-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-array-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-json-array-values">json-array-values</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-array-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a>)</code></li><li><a class="syn-name" href="#v-json-compact">json-compact</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-compact</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-document">json-document</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-document</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonDocument" title="Text.Json.Value.JsonDocument">JsonDocument</a></code></li><li><a class="syn-name" href="#v-json-document-input">json-document-input</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-document-input</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonDocument" title="Text.Json.Value.JsonDocument">JsonDocument</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-document-value">json-document-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-document-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonDocument" title="Text.Json.Value.JsonDocument">JsonDocument</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a></code></li><li><a class="syn-name" href="#v-json-empty-number">json-empty-number</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-empty-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a></code></li><li><a class="syn-name" href="#v-json-error-cause">json-error-cause</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-error-cause</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li><a class="syn-name" href="#v-json-error-invalid-value">json-error-invalid-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-error-invalid-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-error-message">json-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-error-numeric">json-error-numeric</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-error-numeric</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a></code></li><li><a class="syn-name" href="#v-json-error-offset">json-error-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-error-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-json-error-path">json-error-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-error-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-error-preserved-context">json-error-preserved-context</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-error-preserved-context</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li><a class="syn-name" href="#v-json-error-record">json-error-record</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-error-record</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-error-returned">json-error-returned</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-error-returned</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li><a class="syn-name" href="#v-json-error-semantic">json-error-semantic</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-error-semantic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a></code></li><li><a class="syn-name" href="#v-json-error-syntactic">json-error-syntactic</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-error-syntactic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSyntacticInfo" title="Text.Json.Value.JsonSyntacticInfo">JsonSyntacticInfo</a></code></li><li><a class="syn-name" href="#v-json-error-type">json-error-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-error-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-error-type-info">json-error-type-info</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-error-type-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonTypeInfo" title="Text.Json.Value.JsonTypeInfo">JsonTypeInfo</a></code></li><li><a class="syn-name" href="#v-json-error-value">json-error-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-error-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-error-write-info">json-error-write-info</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-error-write-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonWriteInfo" title="Text.Json.Value.JsonWriteInfo">JsonWriteInfo</a></code></li><li><a class="syn-name" href="#v-json-error-written-bytes">json-error-written-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-error-written-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-html-escape">json-html-escape</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-html-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-indent">json-indent</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-indent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-indent-bytes">json-indent-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-indent-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-indent-prefix">json-indent-prefix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-indent-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-lex-read">json-lex-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-lex-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexState" title="Text.Json.Value.JsonLexState">JsonLexState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexStep" title="Text.Json.Value.JsonLexStep">JsonLexStep</a></code></li><li><a class="syn-name" href="#v-json-lex-read-positioned">json-lex-read-positioned</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-lex-read-positioned</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexState" title="Text.Json.Value.JsonLexState">JsonLexState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexStep" title="Text.Json.Value.JsonLexStep">JsonLexStep</a></code></li><li><a class="syn-name" href="#v-json-lex-start">json-lex-start</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-lex-start</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexState" title="Text.Json.Value.JsonLexState">JsonLexState</a></code></li><li><a class="syn-name" href="#v-json-lexeme-end">json-lexeme-end</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-lexeme-end</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexeme" title="Text.Json.Value.JsonLexeme">JsonLexeme</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-json-lexeme-start">json-lexeme-start</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-lexeme-start</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexeme" title="Text.Json.Value.JsonLexeme">JsonLexeme</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-json-lexeme-tag">json-lexeme-tag</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-lexeme-tag</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexeme" title="Text.Json.Value.JsonLexeme">JsonLexeme</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexTag" title="Text.Json.Value.JsonLexTag">JsonLexTag</a></code></li><li><a class="syn-name" href="#v-json-member-key">json-member-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-member-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonMember" title="Text.Json.Value.JsonMember">JsonMember</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a></code></li><li><a class="syn-name" href="#v-json-member-name">json-member-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-member-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonMember" title="Text.Json.Value.JsonMember">JsonMember</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-member-value">json-member-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-member-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonMember" title="Text.Json.Value.JsonMember">JsonMember</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a></code></li><li><a class="syn-name" href="#v-json-number">json-number</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a></code></li><li><a class="syn-name" href="#v-json-number-bytes">json-number-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-number-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-number-check">json-number-check</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-number-check</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a></code></li><li><a class="syn-name" href="#v-json-number-float32">json-number-float32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-number-float32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code></li><li><a class="syn-name" href="#v-json-number-float64">json-number-float64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-number-float64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-json-number-float64-result">json-number-float64-result</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-number-float64-result</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericResult" title="Text.Json.Value.JsonNumericResult">JsonNumericResult</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-json-number-from-bytes">json-number-from-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-number-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a></code></li><li><a class="syn-name" href="#v-json-number-from-text">json-number-from-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-number-from-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a></code></li><li><a class="syn-name" href="#v-json-number-int64">json-number-int64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-number-int64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-json-number-int64-result">json-number-int64-result</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-number-int64-result</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericResult" title="Text.Json.Value.JsonNumericResult">JsonNumericResult</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-json-number-text">json-number-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-number-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-numeric-error">json-numeric-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-numeric-error</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericResult" title="Text.Json.Value.JsonNumericResult">JsonNumericResult</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a></code></li><li><a class="syn-name" href="#v-json-numeric-error-bytes">json-numeric-error-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-numeric-error-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-numeric-error-function">json-numeric-error-function</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-numeric-error-function</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-numeric-error-info">json-numeric-error-info</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-numeric-error-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericErrorKind" title="Text.Json.Value.JsonNumericErrorKind">JsonNumericErrorKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a></code></li><li><a class="syn-name" href="#v-json-numeric-error-input">json-numeric-error-input</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-numeric-error-input</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-numeric-error-kind">json-numeric-error-kind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-numeric-error-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericErrorKind" title="Text.Json.Value.JsonNumericErrorKind">JsonNumericErrorKind</a></code></li><li><a class="syn-name" href="#v-json-numeric-error-message">json-numeric-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-numeric-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-numeric-is-range">json-numeric-is-range</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-numeric-is-range</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-numeric-result">json-numeric-result</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-numeric-result</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericResult" title="Text.Json.Value.JsonNumericResult">JsonNumericResult</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-numeric-value">json-numeric-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-numeric-value</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericResult" title="Text.Json.Value.JsonNumericResult">JsonNumericResult</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-json-object-lookup">json-object-lookup</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-object-lookup</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a></code></li><li><a class="syn-name" href="#v-json-object-members">json-object-members</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-object-members</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonMember" title="Text.Json.Value.JsonMember">JsonMember</a>)</code></li><li><a class="syn-name" href="#v-json-prefix-next-offset">json-prefix-next-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-prefix-next-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-json-preserve-error-context">json-preserve-error-context</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-preserve-error-context</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li><a class="syn-name" href="#v-json-quote">json-quote</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-quote</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-quote-bytes">json-quote-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-quote-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-raw-bytes">json-raw-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-raw-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonRawMessage" title="Text.Json.Value.JsonRawMessage">JsonRawMessage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-raw-from-bytes">json-raw-from-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-raw-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonRawMessage" title="Text.Json.Value.JsonRawMessage">JsonRawMessage</a></code></li><li><a class="syn-name" href="#v-json-raw-null">json-raw-null</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-raw-null</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonRawMessage" title="Text.Json.Value.JsonRawMessage">JsonRawMessage</a></code></li><li><a class="syn-name" href="#v-json-raw-replace">json-raw-replace</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-raw-replace</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonRawMessage" title="Text.Json.Value.JsonRawMessage">JsonRawMessage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonRawMessage" title="Text.Json.Value.JsonRawMessage">JsonRawMessage</a></code></li><li><a class="syn-name" href="#v-json-scan-document">json-scan-document</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-scan-document</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonScanResult" title="Text.Json.Value.JsonScanResult">JsonScanResult</a></code></li><li><a class="syn-name" href="#v-json-scan-error">json-scan-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-scan-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonScanResult" title="Text.Json.Value.JsonScanResult">JsonScanResult</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li><a class="syn-name" href="#v-json-scan-value">json-scan-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-scan-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonScanResult" title="Text.Json.Value.JsonScanResult">JsonScanResult</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a></code></li><li><a class="syn-name" href="#v-json-semantic-action">json-semantic-action</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-semantic-action</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-semantic-bytes">json-semantic-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-semantic-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-semantic-field-path">json-semantic-field-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-semantic-field-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-semantic-info">json-semantic-info</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-semantic-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a></code></li><li><a class="syn-name" href="#v-json-semantic-info-with-optional-type">json-semantic-info-with-optional-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-semantic-info-with-optional-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a></code></li><li><a class="syn-name" href="#v-json-semantic-info-with-value">json-semantic-info-with-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-semantic-info-with-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a></code></li><li><a class="syn-name" href="#v-json-semantic-kind">json-semantic-kind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-semantic-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code></li><li><a class="syn-name" href="#v-json-semantic-kind-byte">json-semantic-kind-byte</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-semantic-kind-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-json-semantic-offset">json-semantic-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-semantic-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-json-semantic-pointer">json-semantic-pointer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-semantic-pointer</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-semantic-reported-bytes">json-semantic-reported-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-semantic-reported-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-semantic-type">json-semantic-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-semantic-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-semantic-type-option">json-semantic-type-option</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-semantic-type-option</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-semantic-with-field-path">json-semantic-with-field-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-semantic-with-field-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a></code></li><li><a class="syn-name" href="#v-json-semantic-with-offset">json-semantic-with-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-semantic-with-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a></code></li><li><a class="syn-name" href="#v-json-semantic-with-pointer">json-semantic-with-pointer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-semantic-with-pointer</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a></code></li><li><a class="syn-name" href="#v-json-semantic-with-type">json-semantic-with-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-semantic-with-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a></code></li><li><a class="syn-name" href="#v-json-syntactic-info">json-syntactic-info</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-syntactic-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSyntacticInfo" title="Text.Json.Value.JsonSyntacticInfo">JsonSyntacticInfo</a></code></li><li><a class="syn-name" href="#v-json-syntactic-offset">json-syntactic-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-syntactic-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSyntacticInfo" title="Text.Json.Value.JsonSyntacticInfo">JsonSyntacticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-json-syntactic-pointer">json-syntactic-pointer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-syntactic-pointer</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSyntacticInfo" title="Text.Json.Value.JsonSyntacticInfo">JsonSyntacticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-type-error-field">json-type-error-field</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-error-field</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonTypeInfo" title="Text.Json.Value.JsonTypeInfo">JsonTypeInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-type-error-offset">json-type-error-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-error-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonTypeInfo" title="Text.Json.Value.JsonTypeInfo">JsonTypeInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-json-type-error-record">json-type-error-record</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-error-record</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonTypeInfo" title="Text.Json.Value.JsonTypeInfo">JsonTypeInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-type-error-type">json-type-error-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-error-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonTypeInfo" title="Text.Json.Value.JsonTypeInfo">JsonTypeInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-type-error-value">json-type-error-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-error-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonTypeInfo" title="Text.Json.Value.JsonTypeInfo">JsonTypeInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-type-info">json-type-info</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonTypeInfo" title="Text.Json.Value.JsonTypeInfo">JsonTypeInfo</a></code></li><li><a class="syn-name" href="#v-json-type-info-with-record-field">json-type-info-with-record-field</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-type-info-with-record-field</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonTypeInfo" title="Text.Json.Value.JsonTypeInfo">JsonTypeInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonTypeInfo" title="Text.Json.Value.JsonTypeInfo">JsonTypeInfo</a></code></li><li><a class="syn-name" href="#v-json-valid">json-valid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-valid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-value-bool">json-value-bool</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-value-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-value-complete">json-value-complete</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-value-complete</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-json-value-container-refusal">json-value-container-refusal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-value-container-refusal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li><a class="syn-name" href="#v-json-value-end">json-value-end</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-value-end</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-json-value-kind">json-value-kind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-value-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code></li><li><a class="syn-name" href="#v-json-value-number">json-value-number</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-value-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a></code></li><li><a class="syn-name" href="#v-json-value-refusal">json-value-refusal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-value-refusal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li><a class="syn-name" href="#v-json-value-source">json-value-source</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-value-source</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-value-start">json-value-start</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-value-start</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-json-value-string">json-value-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-value-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-json-value-structural-refusal">json-value-structural-refusal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-value-structural-refusal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li><a class="syn-name" href="#v-json-write-info">json-write-info</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-write-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonWriteInfo" title="Text.Json.Value.JsonWriteInfo">JsonWriteInfo</a></code></li><li><a class="syn-name" href="#v-json-write-logical-prefix">json-write-logical-prefix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-write-logical-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonWriteInfo" title="Text.Json.Value.JsonWriteInfo">JsonWriteInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-write-written-bytes">json-write-written-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-write-written-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonWriteInfo" title="Text.Json.Value.JsonWriteInfo">JsonWriteInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-json-zero-number">json-zero-number</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">json-zero-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-JsonDocument">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonDocument</strong></code><span class="doc-sig-links"><a href="#t-JsonDocument" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L662" title="View source at line 662">Source</a></span></div>
<div class="doc-body">

An immutable document retaining one owned source snapshot and its navigation tape.

</div>
</article>

<article class="doc-entry" id="t-JsonError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonError</strong></code><span class="doc-sig-links"><a href="#t-JsonError" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L238" title="View source at line 238">Source</a></span></div>
<div class="doc-body">

Structured JSON syntax, type, configuration and custom conversion diagnostics.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-JsonSyntaxError"><code><strong class="tok-constructor">JsonSyntaxError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li id="k-JsonSyntacticError"><code><strong class="tok-constructor">JsonSyntacticError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSyntacticInfo" title="Text.Json.Value.JsonSyntacticInfo">JsonSyntacticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li id="k-JsonTypeError"><code><strong class="tok-constructor">JsonTypeError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li id="k-JsonTypeCause"><code><strong class="tok-constructor">JsonTypeCause</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li id="k-JsonTypeInfoError"><code><strong class="tok-constructor">JsonTypeInfoError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonTypeInfo" title="Text.Json.Value.JsonTypeInfo">JsonTypeInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li id="k-JsonUnsupportedType"><code><strong class="tok-constructor">JsonUnsupportedType</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li id="k-JsonValueError"><code><strong class="tok-constructor">JsonValueError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li id="k-JsonTargetError"><code><strong class="tok-constructor">JsonTargetError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li id="k-JsonUnknownField"><code><strong class="tok-constructor">JsonUnknownField</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li id="k-JsonCustomError"><code><strong class="tok-constructor">JsonCustomError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li id="k-JsonReturnedError"><code><strong class="tok-constructor">JsonReturnedError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li id="k-JsonWrittenError"><code><strong class="tok-constructor">JsonWrittenError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonWriteInfo" title="Text.Json.Value.JsonWriteInfo">JsonWriteInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li id="k-JsonPathError"><code><strong class="tok-constructor">JsonPathError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li id="k-JsonEncodingError"><code><strong class="tok-constructor">JsonEncodingError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li id="k-JsonInvalidValueError"><code><strong class="tok-constructor">JsonInvalidValueError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li id="k-JsonNumberError"><code><strong class="tok-constructor">JsonNumberError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li><li id="k-JsonSemanticError"><code><strong class="tok-constructor">JsonSemanticError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-JsonKind">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonKind</strong></code><span class="doc-sig-links"><a href="#t-JsonKind" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L589" title="View source at line 589">Source</a></span></div>
<div class="doc-body">

Public value categories, including an explicit invalid internal representation category.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-JsonNullKind"><code><strong class="tok-constructor">JsonNullKind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code></li><li id="k-JsonBooleanKind"><code><strong class="tok-constructor">JsonBooleanKind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code></li><li id="k-JsonNumberKind"><code><strong class="tok-constructor">JsonNumberKind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code></li><li id="k-JsonStringKind"><code><strong class="tok-constructor">JsonStringKind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code></li><li id="k-JsonArrayKind"><code><strong class="tok-constructor">JsonArrayKind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code></li><li id="k-JsonObjectKind"><code><strong class="tok-constructor">JsonObjectKind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code></li><li id="k-JsonInvalidKind"><code><strong class="tok-constructor">JsonInvalidKind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-JsonLexState">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonLexState</strong></code><span class="doc-sig-links"><a href="#t-JsonLexState" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L954" title="View source at line 954">Source</a></span></div>
<div class="doc-body">

An immutable resumable lexer state; source bytes remain owned by its caller.

</div>
</article>

<article class="doc-entry" id="t-JsonLexStep">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonLexStep</strong></code><span class="doc-sig-links"><a href="#t-JsonLexStep" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L977" title="View source at line 977">Source</a></span></div>
<div class="doc-body">

Incremental lexical progress distinguishes a complete token, clean EOF and a required continuation.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-JsonLexEnd"><code><strong class="tok-constructor">JsonLexEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexStep" title="Text.Json.Value.JsonLexStep">JsonLexStep</a></code></li><li id="k-JsonLexNeedInput"><code><strong class="tok-constructor">JsonLexNeedInput</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexState" title="Text.Json.Value.JsonLexState">JsonLexState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexStep" title="Text.Json.Value.JsonLexStep">JsonLexStep</a></code></li><li id="k-JsonLexToken"><code><strong class="tok-constructor">JsonLexToken</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexeme" title="Text.Json.Value.JsonLexeme">JsonLexeme</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexStep" title="Text.Json.Value.JsonLexStep">JsonLexStep</a></code></li><li id="k-JsonLexFailure"><code><strong class="tok-constructor">JsonLexFailure</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexStep" title="Text.Json.Value.JsonLexStep">JsonLexStep</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-JsonLexTag">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonLexTag</strong></code><span class="doc-sig-links"><a href="#t-JsonLexTag" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L893" title="View source at line 893">Source</a></span></div>
<div class="doc-body">

Lexical categories shared by immutable documents and incremental stream readers.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-JsonLexNull"><code><strong class="tok-constructor">JsonLexNull</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexTag" title="Text.Json.Value.JsonLexTag">JsonLexTag</a></code></li><li id="k-JsonLexTrue"><code><strong class="tok-constructor">JsonLexTrue</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexTag" title="Text.Json.Value.JsonLexTag">JsonLexTag</a></code></li><li id="k-JsonLexFalse"><code><strong class="tok-constructor">JsonLexFalse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexTag" title="Text.Json.Value.JsonLexTag">JsonLexTag</a></code></li><li id="k-JsonLexNumber"><code><strong class="tok-constructor">JsonLexNumber</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexTag" title="Text.Json.Value.JsonLexTag">JsonLexTag</a></code></li><li id="k-JsonLexString"><code><strong class="tok-constructor">JsonLexString</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexTag" title="Text.Json.Value.JsonLexTag">JsonLexTag</a></code></li><li id="k-JsonLexArrayStart"><code><strong class="tok-constructor">JsonLexArrayStart</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexTag" title="Text.Json.Value.JsonLexTag">JsonLexTag</a></code></li><li id="k-JsonLexArrayEnd"><code><strong class="tok-constructor">JsonLexArrayEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexTag" title="Text.Json.Value.JsonLexTag">JsonLexTag</a></code></li><li id="k-JsonLexObjectStart"><code><strong class="tok-constructor">JsonLexObjectStart</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexTag" title="Text.Json.Value.JsonLexTag">JsonLexTag</a></code></li><li id="k-JsonLexObjectEnd"><code><strong class="tok-constructor">JsonLexObjectEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexTag" title="Text.Json.Value.JsonLexTag">JsonLexTag</a></code></li><li id="k-JsonLexColon"><code><strong class="tok-constructor">JsonLexColon</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexTag" title="Text.Json.Value.JsonLexTag">JsonLexTag</a></code></li><li id="k-JsonLexComma"><code><strong class="tok-constructor">JsonLexComma</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexTag" title="Text.Json.Value.JsonLexTag">JsonLexTag</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-JsonLexeme">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonLexeme</strong></code><span class="doc-sig-links"><a href="#t-JsonLexeme" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L918" title="View source at line 918">Source</a></span></div>
<div class="doc-body">

One lexical token's original byte interval.

</div>
</article>

<article class="doc-entry" id="t-JsonMember">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonMember</strong></code><span class="doc-sig-links"><a href="#t-JsonMember" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L725" title="View source at line 725">Source</a></span></div>
<div class="doc-body">

An object member retains its decoded name and source-backed value view.

</div>
</article>

<article class="doc-entry" id="t-JsonNumber">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonNumber</strong></code><span class="doc-sig-links"><a href="#t-JsonNumber" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L528" title="View source at line 528">Source</a></span></div>
<div class="doc-body">

An exact number spelling with checked, raw-scalar and owned-byte construction.

</div>
</article>

<article class="doc-entry" id="t-JsonNumericError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonNumericError</strong></code><span class="doc-sig-links"><a href="#t-JsonNumericError" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L67" title="View source at line 67">Source</a></span></div>
<div class="doc-body">

Immutable original function, input bytes and numeric refusal reason.

</div>
</article>

<article class="doc-entry" id="t-JsonNumericErrorKind">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonNumericErrorKind</strong></code><span class="doc-sig-links"><a href="#t-JsonNumericErrorKind" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L60" title="View source at line 60">Source</a></span></div>
<div class="doc-body">

The independent syntax or representability reason for a numeric conversion refusal.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-JsonNumericSyntax"><code><strong class="tok-constructor">JsonNumericSyntax</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericErrorKind" title="Text.Json.Value.JsonNumericErrorKind">JsonNumericErrorKind</a></code></li><li id="k-JsonNumericRange"><code><strong class="tok-constructor">JsonNumericRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericErrorKind" title="Text.Json.Value.JsonNumericErrorKind">JsonNumericErrorKind</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-JsonNumericResult">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonNumericResult</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-JsonNumericResult" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L72" title="View source at line 72">Source</a></span></div>
<div class="doc-body">

A numeric value and its optional refusal, preserving saturated and infinite outcomes.

</div>
</article>

<article class="doc-entry" id="t-JsonRawMessage">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonRawMessage</strong></code><span class="doc-sig-links"><a href="#t-JsonRawMessage" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L566" title="View source at line 566">Source</a></span></div>
<div class="doc-body">

An immutable raw JSON message; null and a non-null empty buffer are distinct.

</div>
</article>

<article class="doc-entry" id="t-JsonScanResult">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonScanResult</strong></code><span class="doc-sig-links"><a href="#t-JsonScanResult" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L672" title="View source at line 672">Source</a></span></div>
<div class="doc-body">

An explicit partial scan retains a source-backed root and its optional first grammar refusal.

</div>
</article>

<article class="doc-entry" id="t-JsonSemanticInfo">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonSemanticInfo</strong></code><span class="doc-sig-links"><a href="#t-JsonSemanticInfo" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L132" title="View source at line 132">Source</a></span></div>
<div class="doc-body">

Owned operation context for a conversion that stops at the first semantic failure.

</div>
</article>

<article class="doc-entry" id="t-JsonSyntacticInfo">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonSyntacticInfo</strong></code><span class="doc-sig-links"><a href="#t-JsonSyntacticInfo" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">

Exact zero-based syntax checkpoint and escaped pointer retained independently from diagnostic prose.

</div>
</article>

<article class="doc-entry" id="t-JsonTypeInfo">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonTypeInfo</strong></code><span class="doc-sig-links"><a href="#t-JsonTypeInfo" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L99" title="View source at line 99">Source</a></span></div>
<div class="doc-body">

Immutable optional application identity, actual value, source position and record context for a type refusal.

</div>
</article>

<article class="doc-entry" id="t-JsonValue">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonValue</strong></code><span class="doc-sig-links"><a href="#t-JsonValue" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L667" title="View source at line 667">Source</a></span></div>
<div class="doc-body">

An immutable value view retaining its document and selected node without copying the input.

</div>
</article>

<article class="doc-entry" id="t-JsonWriteInfo">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JsonWriteInfo</strong></code><span class="doc-sig-links"><a href="#t-JsonWriteInfo" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L217" title="View source at line 217">Source</a></span></div>
<div class="doc-body">

Owned bytes already emitted and the logical compact prefix before a failing next value.

</div>
</article>

# Values

<article class="doc-entry" id="v-json-append-compact">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-append-compact</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-append-compact" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L2400" title="View source at line 2400">Source</a></span></div>
<div class="doc-body">

Append compact validated JSON while retaining numeric spellings and original string escape bytes.

</div>
</article>

<article class="doc-entry" id="v-json-append-html-escape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-append-html-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-append-html-escape" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L2252" title="View source at line 2252">Source</a></span></div>
<div class="doc-body">

Append escaped source bytes to an independent copy of an existing destination prefix.

</div>
</article>

<article class="doc-entry" id="v-json-append-indent">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-append-indent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-append-indent" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L2408" title="View source at line 2408">Source</a></span></div>
<div class="doc-body">

Append indented JSON, dropping leading whitespace and preserving original trailing whitespace.

</div>
</article>

<article class="doc-entry" id="v-json-append-indent-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-append-indent-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-append-indent-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L2417" title="View source at line 2417">Source</a></span></div>
<div class="doc-body">

Append indented JSON with arbitrary byte prefixes, preserving malformed bytes and embedded NUL.

</div>
</article>

<article class="doc-entry" id="v-json-array-get">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-array-get</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a></code><span class="doc-sig-links"><a href="#v-json-array-get" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1964" title="View source at line 1964">Source</a></span></div>
<div class="doc-body">

Read one direct array element by zero-based index, retaining its original document owner.

</div>
</article>

<article class="doc-entry" id="v-json-array-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-array-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-json-array-length" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1955" title="View source at line 1955">Source</a></span></div>
<div class="doc-body">

Read the element count without traversing nested values or copying the source.

</div>
</article>

<article class="doc-entry" id="v-json-array-values">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-array-values</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a>)</code><span class="doc-sig-links"><a href="#v-json-array-values" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1981" title="View source at line 1981">Source</a></span></div>
<div class="doc-body">

Collect array element views in source order without copying their payload bytes.

</div>
</article>

<article class="doc-entry" id="v-json-compact">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-compact</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-compact" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L2404" title="View source at line 2404">Source</a></span></div>
<div class="doc-body">

Remove insignificant whitespace from one complete validated JSON value.

</div>
</article>

<article class="doc-entry" id="v-json-document">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-document</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonDocument" title="Text.Json.Value.JsonDocument">JsonDocument</a></code><span class="doc-sig-links"><a href="#v-json-document" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1373" title="View source at line 1373">Source</a></span></div>
<div class="doc-body">

Validate and snapshot one complete JSON document, preserving all original source spellings.

</div>
</article>

<article class="doc-entry" id="v-json-document-input">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-document-input</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonDocument" title="Text.Json.Value.JsonDocument">JsonDocument</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-document-input" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L742" title="View source at line 742">Source</a></span></div>
<div class="doc-body">

Copy the complete original document bytes before exposing mutable storage.

</div>
</article>

<article class="doc-entry" id="v-json-document-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-document-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonDocument" title="Text.Json.Value.JsonDocument">JsonDocument</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a></code><span class="doc-sig-links"><a href="#v-json-document-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L738" title="View source at line 738">Source</a></span></div>
<div class="doc-body">

Obtain the validated root view without copying document storage.

</div>
</article>

<article class="doc-entry" id="v-json-empty-number">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-empty-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a></code><span class="doc-sig-links"><a href="#v-json-empty-number" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1664" title="View source at line 1664">Source</a></span></div>
<div class="doc-body">

An explicitly absent number spelling; typed marshaling represents this default as zero.
Checked integer and float conversions reject the empty spelling.

</div>
</article>

<article class="doc-entry" id="v-json-error-cause">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-error-cause</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code><span class="doc-sig-links"><a href="#v-json-error-cause" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L371" title="View source at line 371">Source</a></span></div>
<div class="doc-body">

Inspect a wrapped custom conversion or path diagnostic without flattening it.

</div>
</article>

<article class="doc-entry" id="v-json-error-invalid-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-error-invalid-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-error-invalid-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L446" title="View source at line 446">Source</a></span></div>
<div class="doc-body">

Whether a value failure lacks a valid application value, after custom/path context is removed.

</div>
</article>

<article class="doc-entry" id="v-json-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L277" title="View source at line 277">Source</a></span></div>
<div class="doc-body">

Render a diagnostic using JSON and application type names.

</div>
</article>

<article class="doc-entry" id="v-json-error-numeric">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-error-numeric</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a></code><span class="doc-sig-links"><a href="#v-json-error-numeric" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L460" title="View source at line 460">Source</a></span></div>
<div class="doc-body">

Recover numeric metadata through custom, path and typed-cause wrappers.

</div>
</article>

<article class="doc-entry" id="v-json-error-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-error-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-json-error-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L299" title="View source at line 299">Source</a></span></div>
<div class="doc-body">

Original byte offset when one exists, otherwise zero.

</div>
</article>

<article class="doc-entry" id="v-json-error-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-error-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-error-path" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L321" title="View source at line 321">Source</a></span></div>
<div class="doc-body">

Application member/index path carried by a diagnostic.

</div>
</article>

<article class="doc-entry" id="v-json-error-preserved-context">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-error-preserved-context</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code><span class="doc-sig-links"><a href="#v-json-error-preserved-context" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L397" title="View source at line 397">Source</a></span></div>
<div class="doc-body">

Recover an origin-protected cause at the public encoding boundary without altering its error family.

</div>
</article>

<article class="doc-entry" id="v-json-error-record">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-error-record</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-error-record" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L442" title="View source at line 442">Source</a></span></div>
<div class="doc-body">

Inspect the root record name independently from the escaped field path.

</div>
</article>

<article class="doc-entry" id="v-json-error-returned">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-error-returned</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code><span class="doc-sig-links"><a href="#v-json-error-returned" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L401" title="View source at line 401">Source</a></span></div>
<div class="doc-body">

Recover an original method-returned error through containing conversion wrappers.

</div>
</article>

<article class="doc-entry" id="v-json-error-semantic">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-error-semantic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a></code><span class="doc-sig-links"><a href="#v-json-error-semantic" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L475" title="View source at line 475">Source</a></span></div>
<div class="doc-body">

Recover semantic source context through containing custom, path and typed-cause adapters.

</div>
</article>

<article class="doc-entry" id="v-json-error-syntactic">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-error-syntactic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSyntacticInfo" title="Text.Json.Value.JsonSyntacticInfo">JsonSyntacticInfo</a></code><span class="doc-sig-links"><a href="#v-json-error-syntactic" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L428" title="View source at line 428">Source</a></span></div>
<div class="doc-body">

Recover precise grammar context through transparent ownership and conversion adapters.

</div>
</article>

<article class="doc-entry" id="v-json-error-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-error-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-error-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L337" title="View source at line 337">Source</a></span></div>
<div class="doc-body">

Application type label associated with a conversion failure.

</div>
</article>

<article class="doc-entry" id="v-json-error-type-info">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-error-type-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonTypeInfo" title="Text.Json.Value.JsonTypeInfo">JsonTypeInfo</a></code><span class="doc-sig-links"><a href="#v-json-error-type-info" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L415" title="View source at line 415">Source</a></span></div>
<div class="doc-body">

Recover exact type metadata through ordinary path and conversion context without flattening its cause.

</div>
</article>

<article class="doc-entry" id="v-json-error-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-error-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-error-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L354" title="View source at line 354">Source</a></span></div>
<div class="doc-body">

Actual value description associated with a conversion failure.

</div>
</article>

<article class="doc-entry" id="v-json-error-write-info">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-error-write-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonWriteInfo" title="Text.Json.Value.JsonWriteInfo">JsonWriteInfo</a></code><span class="doc-sig-links"><a href="#v-json-error-write-info" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L385" title="View source at line 385">Source</a></span></div>
<div class="doc-body">

Inspect output context attached by an encoding boundary without treating a returned callback cause as emitted output.

</div>
</article>

<article class="doc-entry" id="v-json-error-written-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-error-written-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-error-written-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L389" title="View source at line 389">Source</a></span></div>
<div class="doc-body">

Export independently owned partial output when an encoding boundary retained it.

</div>
</article>

<article class="doc-entry" id="v-json-html-escape">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-html-escape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-html-escape" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L2258" title="View source at line 2258">Source</a></span></div>
<div class="doc-body">

Escape HTML-sensitive bytes and line separators without validating the source as JSON.

</div>
</article>

<article class="doc-entry" id="v-json-indent">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-indent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-indent" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L2412" title="View source at line 2412">Source</a></span></div>
<div class="doc-body">

Indent one complete JSON value with arbitrary per-line prefix and indentation strings.

</div>
</article>

<article class="doc-entry" id="v-json-indent-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-indent-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-indent-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L2421" title="View source at line 2421">Source</a></span></div>
<div class="doc-body">

Indent one complete value with byte-exact line-prefix and indentation fragments.

</div>
</article>

<article class="doc-entry" id="v-json-indent-prefix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-indent-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-indent-prefix" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L2381" title="View source at line 2381">Source</a></span></div>
<div class="doc-body">

Format a lexically complete JSON prefix without requiring its containers or pending members to be complete.
Every token must be complete. This operation retains only token spelling
and indentation; it does not accept a prefix as a complete JSON value.

</div>
</article>

<article class="doc-entry" id="v-json-lex-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-lex-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexState" title="Text.Json.Value.JsonLexState">JsonLexState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexStep" title="Text.Json.Value.JsonLexStep">JsonLexStep</a></code><span class="doc-sig-links"><a href="#v-json-lex-read" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1101" title="View source at line 1101">Source</a></span></div>
<div class="doc-body">

Scan at most one token from a checked owned buffer interval, resuming a partial prior token.
Base is the absolute position of buffer byte zero; returned cursors are local to this buffer.

</div>
</article>

<article class="doc-entry" id="v-json-lex-read-positioned">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-lex-read-positioned</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexState" title="Text.Json.Value.JsonLexState">JsonLexState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexStep" title="Text.Json.Value.JsonLexStep">JsonLexStep</a></code><span class="doc-sig-links"><a href="#v-json-lex-read-positioned" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1107" title="View source at line 1107">Source</a></span></div>
<div class="doc-body">

Retain an exact lexical refusal checkpoint without changing successful token or continuation ownership.

</div>
</article>

<article class="doc-entry" id="v-json-lex-start">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-lex-start</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexState" title="Text.Json.Value.JsonLexState">JsonLexState</a></code><span class="doc-sig-links"><a href="#v-json-lex-start" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L973" title="View source at line 973">Source</a></span></div>
<div class="doc-body">

Begin scanning one token without assuming any source chunk is already available.

</div>
</article>

<article class="doc-entry" id="v-json-lexeme-end">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-lexeme-end</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexeme" title="Text.Json.Value.JsonLexeme">JsonLexeme</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-json-lexeme-end" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L927" title="View source at line 927">Source</a></span></div>
<div class="doc-body">

Observe a lexical token's original exclusive end.

</div>
</article>

<article class="doc-entry" id="v-json-lexeme-start">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-lexeme-start</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexeme" title="Text.Json.Value.JsonLexeme">JsonLexeme</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-json-lexeme-start" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L923" title="View source at line 923">Source</a></span></div>
<div class="doc-body">

Observe a lexical token's original start.

</div>
</article>

<article class="doc-entry" id="v-json-lexeme-tag">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-lexeme-tag</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexeme" title="Text.Json.Value.JsonLexeme">JsonLexeme</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonLexTag" title="Text.Json.Value.JsonLexTag">JsonLexTag</a></code><span class="doc-sig-links"><a href="#v-json-lexeme-tag" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L931" title="View source at line 931">Source</a></span></div>
<div class="doc-body">

Observe a lexical category without decoding its source bytes.

</div>
</article>

<article class="doc-entry" id="v-json-member-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-member-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonMember" title="Text.Json.Value.JsonMember">JsonMember</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a></code><span class="doc-sig-links"><a href="#v-json-member-key" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L2439" title="View source at line 2439">Source</a></span></div>
<div class="doc-body">

Retain the exact original quoted member-name token as an immutable source-backed string view.

</div>
</article>

<article class="doc-entry" id="v-json-member-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-member-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonMember" title="Text.Json.Value.JsonMember">JsonMember</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-member-name" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L730" title="View source at line 730">Source</a></span></div>
<div class="doc-body">

Observe a decoded object member name.

</div>
</article>

<article class="doc-entry" id="v-json-member-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-member-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonMember" title="Text.Json.Value.JsonMember">JsonMember</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a></code><span class="doc-sig-links"><a href="#v-json-member-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L734" title="View source at line 734">Source</a></span></div>
<div class="doc-body">

Observe the member value while retaining its source document.

</div>
</article>

<article class="doc-entry" id="v-json-number">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a></code><span class="doc-sig-links"><a href="#v-json-number" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1676" title="View source at line 1676">Source</a></span></div>
<div class="doc-body">

Validate a complete number spelling without accepting surrounding whitespace or another value.

</div>
</article>

<article class="doc-entry" id="v-json-number-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-number-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-number-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L549" title="View source at line 549">Source</a></span></div>
<div class="doc-body">

Export a fresh exact spelling independently of scalar text projection.

</div>
</article>

<article class="doc-entry" id="v-json-number-check">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-number-check</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a></code><span class="doc-sig-links"><a href="#v-json-number-check" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1901" title="View source at line 1901">Source</a></span></div>
<div class="doc-body">

Validate the exact spelling of a raw number without accepting repaired or surrounding text.

</div>
</article>

<article class="doc-entry" id="v-json-number-float32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-number-float32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code><span class="doc-sig-links"><a href="#v-json-number-float32" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1916" title="View source at line 1916">Source</a></span></div>
<div class="doc-body">

Round an exact number directly to finite binary32, rejecting empty spellings and overflow.

</div>
</article>

<article class="doc-entry" id="v-json-number-float64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-number-float64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-json-number-float64" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1908" title="View source at line 1908">Source</a></span></div>
<div class="doc-body">

Convert an exact number to a finite binary float, naming overflow instead of returning infinity.

</div>
</article>

<article class="doc-entry" id="v-json-number-float64-result">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-number-float64-result</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericResult" title="Text.Json.Value.JsonNumericResult">JsonNumericResult</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-json-number-float64-result" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1883" title="View source at line 1883">Source</a></span></div>
<div class="doc-body">

Convert arbitrary decimal, hexadecimal, separator and special spellings with exact original outcomes.

</div>
</article>

<article class="doc-entry" id="v-json-number-from-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-number-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a></code><span class="doc-sig-links"><a href="#v-json-number-from-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L545" title="View source at line 545">Source</a></span></div>
<div class="doc-body">

Snapshot an arbitrary byte spelling, preserving malformed UTF-8 for conversion diagnostics.

</div>
</article>

<article class="doc-entry" id="v-json-number-from-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-number-from-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a></code><span class="doc-sig-links"><a href="#v-json-number-from-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L541" title="View source at line 541">Source</a></span></div>
<div class="doc-body">

Preserve an arbitrary scalar spelling without treating its construction as numeric validation.

</div>
</article>

<article class="doc-entry" id="v-json-number-int64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-number-int64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-json-number-int64" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1924" title="View source at line 1924">Source</a></span></div>
<div class="doc-body">

Convert an integer-only exact spelling to a signed machine integer with explicit range checks.

</div>
</article>

<article class="doc-entry" id="v-json-number-int64-result">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-number-int64-result</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericResult" title="Text.Json.Value.JsonNumericResult">JsonNumericResult</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-json-number-int64-result" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1709" title="View source at line 1709">Source</a></span></div>
<div class="doc-body">

Convert arbitrary signed decimal spellings, returning saturated range outcomes and original metadata.

</div>
</article>

<article class="doc-entry" id="v-json-number-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-number-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-number-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L537" title="View source at line 537">Source</a></span></div>
<div class="doc-body">

Project the original spelling to scalar text; the byte view retains malformed UTF-8 exactly.

</div>
</article>

<article class="doc-entry" id="v-json-numeric-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-numeric-error</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericResult" title="Text.Json.Value.JsonNumericResult">JsonNumericResult</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a></code><span class="doc-sig-links"><a href="#v-json-numeric-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L520" title="View source at line 520">Source</a></span></div>
<div class="doc-body">

Inspect an optional original numeric refusal without discarding its completed value.

</div>
</article>

<article class="doc-entry" id="v-json-numeric-error-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-numeric-error-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-numeric-error-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L493" title="View source at line 493">Source</a></span></div>
<div class="doc-body">

Export an independent copy of the original input, including malformed UTF-8.

</div>
</article>

<article class="doc-entry" id="v-json-numeric-error-function">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-numeric-error-function</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-numeric-error-function" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L489" title="View source at line 489">Source</a></span></div>
<div class="doc-body">

Inspect the original operation label associated with a numeric refusal.

</div>
</article>

<article class="doc-entry" id="v-json-numeric-error-info">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-numeric-error-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericErrorKind" title="Text.Json.Value.JsonNumericErrorKind">JsonNumericErrorKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a></code><span class="doc-sig-links"><a href="#v-json-numeric-error-info" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1689" title="View source at line 1689">Source</a></span></div>
<div class="doc-body">

Snapshot explicit operation, original bytes and refusal reason for an application error adapter.

</div>
</article>

<article class="doc-entry" id="v-json-numeric-error-input">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-numeric-error-input</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-numeric-error-input" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L497" title="View source at line 497">Source</a></span></div>
<div class="doc-body">

Project the original input to scalar text, replacing malformed UTF-8 while its byte view stays exact.

</div>
</article>

<article class="doc-entry" id="v-json-numeric-error-kind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-numeric-error-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericErrorKind" title="Text.Json.Value.JsonNumericErrorKind">JsonNumericErrorKind</a></code><span class="doc-sig-links"><a href="#v-json-numeric-error-kind" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L501" title="View source at line 501">Source</a></span></div>
<div class="doc-body">

Inspect the exact syntax or range classification independently of its message.

</div>
</article>

<article class="doc-entry" id="v-json-numeric-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-numeric-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-numeric-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L509" title="View source at line 509">Source</a></span></div>
<div class="doc-body">

Render numeric metadata using the original operation and a scalar-safe quoted input.

</div>
</article>

<article class="doc-entry" id="v-json-numeric-is-range">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-numeric-is-range</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-numeric-is-range" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L505" title="View source at line 505">Source</a></span></div>
<div class="doc-body">

Whether a numeric refusal reports representability rather than syntax.

</div>
</article>

<article class="doc-entry" id="v-json-numeric-result">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-numeric-result</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericResult" title="Text.Json.Value.JsonNumericResult">JsonNumericResult</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericError" title="Text.Json.Value.JsonNumericError">JsonNumericError</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-numeric-result" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L524" title="View source at line 524">Source</a></span></div>
<div class="doc-body">

Select an ordinary success or refusal when the caller does not need a partial numeric value.

</div>
</article>

<article class="doc-entry" id="v-json-numeric-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-numeric-value</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumericResult" title="Text.Json.Value.JsonNumericResult">JsonNumericResult</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-json-numeric-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L516" title="View source at line 516">Source</a></span></div>
<div class="doc-body">

Inspect the completed value even when conversion also reports an error.

</div>
</article>

<article class="doc-entry" id="v-json-object-lookup">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-object-lookup</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a></code><span class="doc-sig-links"><a href="#v-json-object-lookup" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L2017" title="View source at line 2017">Source</a></span></div>
<div class="doc-body">

Find the final exact decoded member name; later duplicates replace earlier matches.

</div>
</article>

<article class="doc-entry" id="v-json-object-members">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-object-members</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonMember" title="Text.Json.Value.JsonMember">JsonMember</a>)</code><span class="doc-sig-links"><a href="#v-json-object-members" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1993" title="View source at line 1993">Source</a></span></div>
<div class="doc-body">

Read object members in original order, preserving repeated decoded names.

</div>
</article>

<article class="doc-entry" id="v-json-prefix-next-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-prefix-next-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-json-prefix-next-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L2396" title="View source at line 2396">Source</a></span></div>
<div class="doc-body">

Count the logical byte position of a next value after a lexically complete prefix.
The caller includes any upcoming colon or comma in the prefix. Pretty
mode additionally counts colon spacing and pending container indentation.
This is a diagnostic operation, not complete-document validation.

</div>
</article>

<article class="doc-entry" id="v-json-preserve-error-context">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-preserve-error-context</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code><span class="doc-sig-links"><a href="#v-json-preserve-error-context" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L393" title="View source at line 393">Source</a></span></div>
<div class="doc-body">

Retain application-owned context across automatic encoding container decoration.

</div>
</article>

<article class="doc-entry" id="v-json-quote">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-quote</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-quote" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L2214" title="View source at line 2214">Source</a></span></div>
<div class="doc-body">

Encode an immutable scalar string with explicit HTML escaping policy.

</div>
</article>

<article class="doc-entry" id="v-json-quote-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-quote-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-quote-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L2207" title="View source at line 2207">Source</a></span></div>
<div class="doc-body">

Encode arbitrary byte-string contents as valid quoted JSON, replacing each malformed UTF-8 byte.

</div>
</article>

<article class="doc-entry" id="v-json-raw-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-raw-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonRawMessage" title="Text.Json.Value.JsonRawMessage">JsonRawMessage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-raw-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L579" title="View source at line 579">Source</a></span></div>
<div class="doc-body">

Obtain an independent copy of raw storage, preserving the distinction between null and empty.

</div>
</article>

<article class="doc-entry" id="v-json-raw-from-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-raw-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonRawMessage" title="Text.Json.Value.JsonRawMessage">JsonRawMessage</a></code><span class="doc-sig-links"><a href="#v-json-raw-from-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L575" title="View source at line 575">Source</a></span></div>
<div class="doc-body">

Snapshot an arbitrary raw message without changing whitespace or validating its syntax.

</div>
</article>

<article class="doc-entry" id="v-json-raw-null">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-raw-null</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonRawMessage" title="Text.Json.Value.JsonRawMessage">JsonRawMessage</a></code><span class="doc-sig-links"><a href="#v-json-raw-null" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L571" title="View source at line 571">Source</a></span></div>
<div class="doc-body">

The absent raw message, which encodes as the JSON null literal.

</div>
</article>

<article class="doc-entry" id="v-json-raw-replace">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-raw-replace</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonRawMessage" title="Text.Json.Value.JsonRawMessage">JsonRawMessage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonRawMessage" title="Text.Json.Value.JsonRawMessage">JsonRawMessage</a></code><span class="doc-sig-links"><a href="#v-json-raw-replace" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L585" title="View source at line 585">Source</a></span></div>
<div class="doc-body">

Replace a raw message with a fresh snapshot while retained earlier messages stay independent.

</div>
</article>

<article class="doc-entry" id="v-json-scan-document">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-scan-document</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonScanResult" title="Text.Json.Value.JsonScanResult">JsonScanResult</a></code><span class="doc-sig-links"><a href="#v-json-scan-document" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1383" title="View source at line 1383">Source</a></span></div>
<div class="doc-body">

Scan once while retaining explicit partial value state and the first original grammar refusal.

</div>
</article>

<article class="doc-entry" id="v-json-scan-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-scan-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonScanResult" title="Text.Json.Value.JsonScanResult">JsonScanResult</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code><span class="doc-sig-links"><a href="#v-json-scan-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L681" title="View source at line 681">Source</a></span></div>
<div class="doc-body">

Inspect the first grammar refusal independently from completed child views.

</div>
</article>

<article class="doc-entry" id="v-json-scan-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-scan-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonScanResult" title="Text.Json.Value.JsonScanResult">JsonScanResult</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a></code><span class="doc-sig-links"><a href="#v-json-scan-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L677" title="View source at line 677">Source</a></span></div>
<div class="doc-body">

Inspect a scan's retained root without implying that its source was a complete document.

</div>
</article>

<article class="doc-entry" id="v-json-semantic-action">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-semantic-action</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-semantic-action" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L181" title="View source at line 181">Source</a></span></div>
<div class="doc-body">

Inspect the conversion operation, such as marshal or unmarshal.

</div>
</article>

<article class="doc-entry" id="v-json-semantic-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-semantic-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-semantic-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L193" title="View source at line 193">Source</a></span></div>
<div class="doc-body">

Export an independent copy of the original complete source token.

</div>
</article>

<article class="doc-entry" id="v-json-semantic-field-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-semantic-field-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-semantic-field-path" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L209" title="View source at line 209">Source</a></span></div>
<div class="doc-body">

Inspect conventional field context only when supplied independently from the precise semantic pointer.

</div>
</article>

<article class="doc-entry" id="v-json-semantic-info">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-semantic-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a></code><span class="doc-sig-links"><a href="#v-json-semantic-info" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L137" title="View source at line 137">Source</a></span></div>
<div class="doc-body">

Snapshot exact source bytes while constructing explicit semantic conversion metadata.

</div>
</article>

<article class="doc-entry" id="v-json-semantic-info-with-optional-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-semantic-info-with-optional-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a></code><span class="doc-sig-links"><a href="#v-json-semantic-info-with-optional-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L157" title="View source at line 157">Source</a></span></div>
<div class="doc-body">

Snapshot exact semantic bytes while preserving absence of an application type independently of an empty label.

</div>
</article>

<article class="doc-entry" id="v-json-semantic-info-with-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-semantic-info-with-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a></code><span class="doc-sig-links"><a href="#v-json-semantic-info-with-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L153" title="View source at line 153">Source</a></span></div>
<div class="doc-body">

Snapshot source context and independently reported value bytes, retaining an explicit lexical kind byte.
Kind bytes use the opening delimiter, n/t/f for null and booleans, digit
zero for a number, and zero when the conversion does not identify a kind.

</div>
</article>

<article class="doc-entry" id="v-json-semantic-kind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-semantic-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code><span class="doc-sig-links"><a href="#v-json-semantic-kind" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L165" title="View source at line 165">Source</a></span></div>
<div class="doc-body">

Inspect the semantic source category while retaining exact lexical distinctions through the byte getter.

</div>
</article>

<article class="doc-entry" id="v-json-semantic-kind-byte">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-semantic-kind-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-json-semantic-kind-byte" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L161" title="View source at line 161">Source</a></span></div>
<div class="doc-body">

Inspect the exact lexical kind byte, preserving true, false and unspecified distinctions.

</div>
</article>

<article class="doc-entry" id="v-json-semantic-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-semantic-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-json-semantic-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L185" title="View source at line 185">Source</a></span></div>
<div class="doc-body">

Inspect the original zero-based byte offset at which semantic conversion began.

</div>
</article>

<article class="doc-entry" id="v-json-semantic-pointer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-semantic-pointer</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-semantic-pointer" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L189" title="View source at line 189">Source</a></span></div>
<div class="doc-body">

Inspect the escaped JSON pointer identifying the original source value.

</div>
</article>

<article class="doc-entry" id="v-json-semantic-reported-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-semantic-reported-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-semantic-reported-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L173" title="View source at line 173">Source</a></span></div>
<div class="doc-body">

Export a fresh copy of the independently reported value, which may be empty despite retained original source bytes.

</div>
</article>

<article class="doc-entry" id="v-json-semantic-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-semantic-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-semantic-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L197" title="View source at line 197">Source</a></span></div>
<div class="doc-body">

Inspect the application type as text, using empty text when no identity was supplied.

</div>
</article>

<article class="doc-entry" id="v-json-semantic-type-option">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-semantic-type-option</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-semantic-type-option" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L201" title="View source at line 201">Source</a></span></div>
<div class="doc-body">

Inspect the optional application identity without conflating absence and an explicitly empty label.

</div>
</article>

<article class="doc-entry" id="v-json-semantic-with-field-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-semantic-with-field-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a></code><span class="doc-sig-links"><a href="#v-json-semantic-with-field-path" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L213" title="View source at line 213">Source</a></span></div>
<div class="doc-body">

Replace conventional field context while sharing immutable source, identity and reported-value owners.

</div>
</article>

<article class="doc-entry" id="v-json-semantic-with-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-semantic-with-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a></code><span class="doc-sig-links"><a href="#v-json-semantic-with-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L234" title="View source at line 234">Source</a></span></div>
<div class="doc-body">

Replace only the semantic byte offset while sharing immutable token and reported-value owners.

</div>
</article>

<article class="doc-entry" id="v-json-semantic-with-pointer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-semantic-with-pointer</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a></code><span class="doc-sig-links"><a href="#v-json-semantic-with-pointer" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L177" title="View source at line 177">Source</a></span></div>
<div class="doc-body">

Replace only the pointer while retaining immutable source, reported value and operation metadata.

</div>
</article>

<article class="doc-entry" id="v-json-semantic-with-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-semantic-with-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSemanticInfo" title="Text.Json.Value.JsonSemanticInfo">JsonSemanticInfo</a></code><span class="doc-sig-links"><a href="#v-json-semantic-with-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L205" title="View source at line 205">Source</a></span></div>
<div class="doc-body">

Replace only optional application identity while retaining immutable source and reported-value owners.

</div>
</article>

<article class="doc-entry" id="v-json-syntactic-info">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-syntactic-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSyntacticInfo" title="Text.Json.Value.JsonSyntacticInfo">JsonSyntacticInfo</a></code><span class="doc-sig-links"><a href="#v-json-syntactic-info" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L87" title="View source at line 87">Source</a></span></div>
<div class="doc-body">

Construct precise syntax context without replacing its original structured refusal.

</div>
</article>

<article class="doc-entry" id="v-json-syntactic-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-syntactic-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSyntacticInfo" title="Text.Json.Value.JsonSyntacticInfo">JsonSyntacticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-json-syntactic-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L91" title="View source at line 91">Source</a></span></div>
<div class="doc-body">

Inspect the exact byte checkpoint before the offending token or at incomplete input.

</div>
</article>

<article class="doc-entry" id="v-json-syntactic-pointer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-syntactic-pointer</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonSyntacticInfo" title="Text.Json.Value.JsonSyntacticInfo">JsonSyntacticInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-syntactic-pointer" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L95" title="View source at line 95">Source</a></span></div>
<div class="doc-body">

Inspect the original escaped grammar pointer of a syntax refusal.

</div>
</article>

<article class="doc-entry" id="v-json-type-error-field">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-error-field</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonTypeInfo" title="Text.Json.Value.JsonTypeInfo">JsonTypeInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-type-error-field" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L124" title="View source at line 124">Source</a></span></div>
<div class="doc-body">

Inspect the original escaped application field path.

</div>
</article>

<article class="doc-entry" id="v-json-type-error-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-error-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonTypeInfo" title="Text.Json.Value.JsonTypeInfo">JsonTypeInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-json-type-error-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L116" title="View source at line 116">Source</a></span></div>
<div class="doc-body">

Inspect the original byte offset reported by a type refusal.

</div>
</article>

<article class="doc-entry" id="v-json-type-error-record">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-error-record</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonTypeInfo" title="Text.Json.Value.JsonTypeInfo">JsonTypeInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-type-error-record" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L120" title="View source at line 120">Source</a></span></div>
<div class="doc-body">

Inspect the independently named root application record.

</div>
</article>

<article class="doc-entry" id="v-json-type-error-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-error-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonTypeInfo" title="Text.Json.Value.JsonTypeInfo">JsonTypeInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-type-error-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L108" title="View source at line 108">Source</a></span></div>
<div class="doc-body">

Inspect the optional application identity of a type refusal.

</div>
</article>

<article class="doc-entry" id="v-json-type-error-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-error-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonTypeInfo" title="Text.Json.Value.JsonTypeInfo">JsonTypeInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-type-error-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L112" title="View source at line 112">Source</a></span></div>
<div class="doc-body">

Inspect the original source value description of a type refusal.

</div>
</article>

<article class="doc-entry" id="v-json-type-info">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonTypeInfo" title="Text.Json.Value.JsonTypeInfo">JsonTypeInfo</a></code><span class="doc-sig-links"><a href="#v-json-type-info" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L104" title="View source at line 104">Source</a></span></div>
<div class="doc-body">

Construct explicit type metadata without conflating an absent identity and an empty application label.

</div>
</article>

<article class="doc-entry" id="v-json-type-info-with-record-field">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-type-info-with-record-field</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonTypeInfo" title="Text.Json.Value.JsonTypeInfo">JsonTypeInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonTypeInfo" title="Text.Json.Value.JsonTypeInfo">JsonTypeInfo</a></code><span class="doc-sig-links"><a href="#v-json-type-info-with-record-field" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L128" title="View source at line 128">Source</a></span></div>
<div class="doc-body">

Replace record and field context while sharing immutable identity and value metadata.

</div>
</article>

<article class="doc-entry" id="v-json-valid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-valid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-valid" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1400" title="View source at line 1400">Source</a></span></div>
<div class="doc-body">

Validate JSON byte grammar without constructing a document or decoding scalar strings.

</div>
</article>

<article class="doc-entry" id="v-json-value-bool">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-value-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-value-bool" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L792" title="View source at line 792">Source</a></span></div>
<div class="doc-body">

Read a boolean literal without treating other categories as false.

</div>
</article>

<article class="doc-entry" id="v-json-value-complete">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-value-complete</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-json-value-complete" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L685" title="View source at line 685">Source</a></span></div>
<div class="doc-body">

Whether one selected value was completely recognized before its containing scan stopped.

</div>
</article>

<article class="doc-entry" id="v-json-value-container-refusal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-value-container-refusal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code><span class="doc-sig-links"><a href="#v-json-value-container-refusal" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L717" title="View source at line 717">Source</a></span></div>
<div class="doc-body">

Recover the refusal after traversing the selected incomplete container without reinterpreting skipped child values.

</div>
</article>

<article class="doc-entry" id="v-json-value-end">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-value-end</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-json-value-end" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L758" title="View source at line 758">Source</a></span></div>
<div class="doc-body">

Observe the selected value's original exclusive source end.

</div>
</article>

<article class="doc-entry" id="v-json-value-kind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-value-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonKind" title="Text.Json.Value.JsonKind">JsonKind</a></code><span class="doc-sig-links"><a href="#v-json-value-kind" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L762" title="View source at line 762">Source</a></span></div>
<div class="doc-body">

Observe a value category without traversing or decoding its subtree.

</div>
</article>

<article class="doc-entry" id="v-json-value-number">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-value-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a></code><span class="doc-sig-links"><a href="#v-json-value-number" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1668" title="View source at line 1668">Source</a></span></div>
<div class="doc-body">

Read a validated exact number from a source-backed scalar view.

</div>
</article>

<article class="doc-entry" id="v-json-value-refusal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-value-refusal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code><span class="doc-sig-links"><a href="#v-json-value-refusal" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L700" title="View source at line 700">Source</a></span></div>
<div class="doc-body">

Recover a scan refusal only for an incomplete value, preserving completed sibling independence.

</div>
</article>

<article class="doc-entry" id="v-json-value-source">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-value-source</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-value-source" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L784" title="View source at line 784">Source</a></span></div>
<div class="doc-body">

Copy the selected value's retained source interval, preserving exact bytes without implying completeness.

</div>
</article>

<article class="doc-entry" id="v-json-value-start">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-value-start</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-json-value-start" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L754" title="View source at line 754">Source</a></span></div>
<div class="doc-body">

Observe the selected value's zero-based original source start.

</div>
</article>

<article class="doc-entry" id="v-json-value-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-value-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-json-value-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L2027" title="View source at line 2027">Source</a></span></div>
<div class="doc-body">

Decode a string value without exposing mutable source storage.

</div>
</article>

<article class="doc-entry" id="v-json-value-structural-refusal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-value-structural-refusal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonValue" title="Text.Json.Value.JsonValue">JsonValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonError" title="Text.Json.Value.JsonError">JsonError</a></code><span class="doc-sig-links"><a href="#v-json-value-structural-refusal" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L721" title="View source at line 721">Source</a></span></div>
<div class="doc-body">

Recover the refusal after traversing every retained container in a structural value tree.

</div>
</article>

<article class="doc-entry" id="v-json-write-info">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-write-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonWriteInfo" title="Text.Json.Value.JsonWriteInfo">JsonWriteInfo</a></code><span class="doc-sig-links"><a href="#v-json-write-info" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L222" title="View source at line 222">Source</a></span></div>
<div class="doc-body">

Snapshot written output and logical delimiter context independently from caller mutation.

</div>
</article>

<article class="doc-entry" id="v-json-write-logical-prefix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-write-logical-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonWriteInfo" title="Text.Json.Value.JsonWriteInfo">JsonWriteInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-write-logical-prefix" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L230" title="View source at line 230">Source</a></span></div>
<div class="doc-body">

Export the compact next-value context, including delimiters counted before they are emitted.

</div>
</article>

<article class="doc-entry" id="v-json-write-written-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-write-written-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonWriteInfo" title="Text.Json.Value.JsonWriteInfo">JsonWriteInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-json-write-written-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L226" title="View source at line 226">Source</a></span></div>
<div class="doc-body">

Export a fresh copy of the output emitted before an encoding refusal.

</div>
</article>

<article class="doc-entry" id="v-json-zero-number">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">json-zero-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Json.Value/#t-JsonNumber" title="Text.Json.Value.JsonNumber">JsonNumber</a></code><span class="doc-sig-links"><a href="#v-json-zero-number" title="Permalink">#</a><a href="/packages/stdlib/Text.Json.Value/source/#L1659" title="View source at line 1659">Source</a></span></div>
<div class="doc-body">

The invariant exact integer-zero number, usable as a total descriptor default.

</div>
</article>
