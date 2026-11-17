# Text.Json.Value

Owned JSON documents and navigation tapes, raw number spellings, exact numeric values and input-bearing errors, resumable lexical state and byte-preserving formatting.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Json.Value`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Json.Value.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Array](Data.Array.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Text.Json.Binary32](Text.Json.Binary32.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Array](Data.Array.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Text.Json.Binary32](Text.Json.Binary32.md), [Text.Json.Value](Text.Json.Value.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Exact JSON syntax, immutable source-backed values and byte-preserving formatting.
Documents own one byte snapshot and an immutable navigation tape. Returned
byte buffers are independent copies; scalar strings replace malformed UTF-8
and unmatched surrogate escapes while number spellings remain exact.

</details>

# Types and constructors

<a id="type-4a736f6e446f63756d656e74"></a>

## `JsonDocument`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonDocument</span></code></pre>

An immutable document retaining one owned source snapshot and its navigation tape.

[Source](/reference-sources/Text.Json.Value.cinch) · line 590

<a id="type-4a736f6e4572726f72"></a>

## `JsonError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonError</span></code></pre>

Structured JSON syntax, type, configuration and custom conversion diagnostics.

[Source](/reference-sources/Text.Json.Value.cinch) · line 213

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonSyntaxError</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonTypeError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>))))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonTypeCause</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>)))))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonTypeInfoError</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonTypeInfo</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonUnsupportedType</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonValueError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonTargetError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonUnknownField</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>)))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonCustomError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonReturnedError</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonWrittenError</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonWriteInfo</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonPathError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonEncodingError</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonInvalidValueError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonNumberError</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonSemanticError</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>))</code></pre>

<a id="type-4a736f6e4b696e64"></a>

## `JsonKind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonKind</span></code></pre>

Public value categories, including an explicit invalid internal representation category.

[Source](/reference-sources/Text.Json.Value.cinch) · line 538

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonNullKind</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonBooleanKind</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonNumberKind</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonStringKind</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonArrayKind</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonObjectKind</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonInvalidKind</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonKind</span></code></pre>

<a id="type-4a736f6e4c65785374617465"></a>

## `JsonLexState`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonLexState</span></code></pre>

An immutable resumable lexer state; source bytes remain owned by its caller.

[Source](/reference-sources/Text.Json.Value.cinch) · line 842

<a id="type-4a736f6e4c657853746570"></a>

## `JsonLexStep`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonLexStep</span></code></pre>

Incremental lexical progress distinguishes a complete token, clean EOF and a required continuation.

[Source](/reference-sources/Text.Json.Value.cinch) · line 865

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonLexEnd</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexStep</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonLexNeedInput</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexState</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexStep</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonLexToken</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexeme</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexStep</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonLexFailure</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexStep</span>))</code></pre>

<a id="type-4a736f6e4c6578546167"></a>

## `JsonLexTag`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonLexTag</span></code></pre>

Lexical categories shared by immutable documents and incremental stream readers.

[Source](/reference-sources/Text.Json.Value.cinch) · line 781

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonLexNull</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonLexTrue</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonLexFalse</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonLexNumber</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonLexString</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonLexArrayStart</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonLexArrayEnd</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonLexObjectStart</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonLexObjectEnd</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonLexColon</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexTag</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonLexComma</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexTag</span></code></pre>

<a id="type-4a736f6e4c6578656d65"></a>

## `JsonLexeme`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonLexeme</span></code></pre>

One lexical token's original byte interval.

[Source](/reference-sources/Text.Json.Value.cinch) · line 806

<a id="type-4a736f6e4d656d626572"></a>

## `JsonMember`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonMember</span></code></pre>

An object member retains its decoded name and source-backed value view.

[Source](/reference-sources/Text.Json.Value.cinch) · line 600

<a id="type-4a736f6e4e756d626572"></a>

## `JsonNumber`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonNumber</span></code></pre>

An exact number spelling with checked, raw-scalar and owned-byte construction.

[Source](/reference-sources/Text.Json.Value.cinch) · line 477

<a id="type-4a736f6e4e756d657269634572726f72"></a>

## `JsonNumericError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonNumericError</span></code></pre>

Immutable original function, input bytes and numeric refusal reason.

[Source](/reference-sources/Text.Json.Value.cinch) · line 64

<a id="type-4a736f6e4e756d657269634572726f724b696e64"></a>

## `JsonNumericErrorKind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonNumericErrorKind</span></code></pre>

The independent syntax or representability reason for a numeric conversion refusal.

[Source](/reference-sources/Text.Json.Value.cinch) · line 57

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonNumericSyntax</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericErrorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonNumericRange</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericErrorKind</span></code></pre>

<a id="type-4a736f6e4e756d65726963526573756c74"></a>

## `JsonNumericResult`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonNumericResult</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A numeric value and its optional refusal, preserving saturated and infinite outcomes.

[Source](/reference-sources/Text.Json.Value.cinch) · line 69

<a id="type-4a736f6e5261774d657373616765"></a>

## `JsonRawMessage`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonRawMessage</span></code></pre>

An immutable raw JSON message; null and a non-null empty buffer are distinct.

[Source](/reference-sources/Text.Json.Value.cinch) · line 515

<a id="type-4a736f6e53656d616e746963496e666f"></a>

## `JsonSemanticInfo`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonSemanticInfo</span></code></pre>

Owned operation context for a conversion that stops at the first semantic failure.

[Source](/reference-sources/Text.Json.Value.cinch) · line 107

<a id="type-4a736f6e54797065496e666f"></a>

## `JsonTypeInfo`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonTypeInfo</span></code></pre>

Immutable optional application identity, actual value, source position and record context for a type refusal.

[Source](/reference-sources/Text.Json.Value.cinch) · line 74

<a id="type-4a736f6e56616c7565"></a>

## `JsonValue`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonValue</span></code></pre>

An immutable value view retaining its document and selected node without copying the input.

[Source](/reference-sources/Text.Json.Value.cinch) · line 595

<a id="type-4a736f6e5772697465496e666f"></a>

## `JsonWriteInfo`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonWriteInfo</span></code></pre>

Owned bytes already emitted and the logical compact prefix before a failing next value.

[Source](/reference-sources/Text.Json.Value.cinch) · line 192

# Values

<a id="value-6a736f6e2d617070656e642d636f6d70616374"></a>

## `json-append-compact`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-append-compact</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Append compact validated JSON while retaining numeric spellings and original string escape bytes.

[Source](/reference-sources/Text.Json.Value.cinch) · line 2126

<a id="value-6a736f6e2d617070656e642d68746d6c2d657363617065"></a>

## `json-append-html-escape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-append-html-escape</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append escaped source bytes to an independent copy of an existing destination prefix.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1978

<a id="value-6a736f6e2d617070656e642d696e64656e74"></a>

## `json-append-indent`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-append-indent</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

Append indented JSON, dropping leading whitespace and preserving original trailing whitespace.

[Source](/reference-sources/Text.Json.Value.cinch) · line 2134

<a id="value-6a736f6e2d617070656e642d696e64656e742d6279746573"></a>

## `json-append-indent-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-append-indent-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

Append indented JSON with arbitrary byte prefixes, preserving malformed bytes and embedded NUL.

[Source](/reference-sources/Text.Json.Value.cinch) · line 2143

<a id="value-6a736f6e2d61727261792d676574"></a>

## `json-array-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-array-get</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span>)))</code></pre>

Read one direct array element by zero-based index, retaining its original document owner.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1696

<a id="value-6a736f6e2d61727261792d6c656e677468"></a>

## `json-array-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-array-length</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>))</code></pre>

Read the element count without traversing nested values or copying the source.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1688

<a id="value-6a736f6e2d61727261792d76616c756573"></a>

## `json-array-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-array-values</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span>)))</code></pre>

Collect array element views in source order without copying their payload bytes.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1708

<a id="value-6a736f6e2d636f6d70616374"></a>

## `json-compact`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-compact</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Remove insignificant whitespace from one complete validated JSON value.

[Source](/reference-sources/Text.Json.Value.cinch) · line 2130

<a id="value-6a736f6e2d646f63756d656e74"></a>

## `json-document`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-document</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonDocument</span>))</code></pre>

Validate and snapshot one complete JSON document, preserving all original source spellings.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1238

<a id="value-6a736f6e2d646f63756d656e742d696e707574"></a>

## `json-document-input`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-document-input</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonDocument</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Copy the complete original document bytes before exposing mutable storage.

[Source](/reference-sources/Text.Json.Value.cinch) · line 617

<a id="value-6a736f6e2d646f63756d656e742d76616c7565"></a>

## `json-document-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-document-value</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonDocument</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span>)</code></pre>

Obtain the validated root view without copying document storage.

[Source](/reference-sources/Text.Json.Value.cinch) · line 613

<a id="value-6a736f6e2d656d7074792d6e756d626572"></a>

## `json-empty-number`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-empty-number</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumber</span></code></pre>

An explicitly absent number spelling; typed marshaling represents this default as zero.
Checked integer and float conversions reject the empty spelling.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1382

<a id="value-6a736f6e2d6572726f722d6361757365"></a>

## `json-error-cause`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-error-cause</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>))</code></pre>

Inspect a wrapped custom conversion or path diagnostic without flattening it.

[Source](/reference-sources/Text.Json.Value.cinch) · line 339

<a id="value-6a736f6e2d6572726f722d696e76616c69642d76616c7565"></a>

## `json-error-invalid-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-error-invalid-value</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether a value failure lacks a valid application value, after custom/path context is removed.

[Source](/reference-sources/Text.Json.Value.cinch) · line 398

<a id="value-6a736f6e2d6572726f722d6d657373616765"></a>

## `json-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a diagnostic using JSON and application type names.

[Source](/reference-sources/Text.Json.Value.cinch) · line 250

<a id="value-6a736f6e2d6572726f722d6e756d65726963"></a>

## `json-error-numeric`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-error-numeric</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericError</span>))</code></pre>

Recover numeric metadata through custom, path and typed-cause wrappers.

[Source](/reference-sources/Text.Json.Value.cinch) · line 411

<a id="value-6a736f6e2d6572726f722d6f6666736574"></a>

## `json-error-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-error-offset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Original byte offset when one exists, otherwise zero.

[Source](/reference-sources/Text.Json.Value.cinch) · line 271

<a id="value-6a736f6e2d6572726f722d70617468"></a>

## `json-error-path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-error-path</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Application member/index path carried by a diagnostic.

[Source](/reference-sources/Text.Json.Value.cinch) · line 292

<a id="value-6a736f6e2d6572726f722d7072657365727665642d636f6e74657874"></a>

## `json-error-preserved-context`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-error-preserved-context</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>))</code></pre>

Recover an origin-protected cause at the public encoding boundary without altering its error family.

[Source](/reference-sources/Text.Json.Value.cinch) · line 364

<a id="value-6a736f6e2d6572726f722d7265636f7264"></a>

## `json-error-record`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-error-record</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Inspect the root record name independently from the escaped field path.

[Source](/reference-sources/Text.Json.Value.cinch) · line 394

<a id="value-6a736f6e2d6572726f722d72657475726e6564"></a>

## `json-error-returned`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-error-returned</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>))</code></pre>

Recover an original method-returned error through containing conversion wrappers.

[Source](/reference-sources/Text.Json.Value.cinch) · line 368

<a id="value-6a736f6e2d6572726f722d73656d616e746963"></a>

## `json-error-semantic`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-error-semantic</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span>))</code></pre>

Recover semantic source context through containing custom, path and typed-cause adapters.

[Source](/reference-sources/Text.Json.Value.cinch) · line 425

<a id="value-6a736f6e2d6572726f722d74797065"></a>

## `json-error-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-error-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Application type label associated with a conversion failure.

[Source](/reference-sources/Text.Json.Value.cinch) · line 307

<a id="value-6a736f6e2d6572726f722d747970652d696e666f"></a>

## `json-error-type-info`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-error-type-info</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonTypeInfo</span>))</code></pre>

Recover exact type metadata through ordinary path and conversion context without flattening its cause.

[Source](/reference-sources/Text.Json.Value.cinch) · line 381

<a id="value-6a736f6e2d6572726f722d76616c7565"></a>

## `json-error-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-error-value</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Actual value description associated with a conversion failure.

[Source](/reference-sources/Text.Json.Value.cinch) · line 323

<a id="value-6a736f6e2d6572726f722d77726974652d696e666f"></a>

## `json-error-write-info`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-error-write-info</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonWriteInfo</span>))</code></pre>

Inspect output context attached by an encoding boundary without treating a returned callback cause as emitted output.

[Source](/reference-sources/Text.Json.Value.cinch) · line 352

<a id="value-6a736f6e2d6572726f722d7772697474656e2d6279746573"></a>

## `json-error-written-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-error-written-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Export independently owned partial output when an encoding boundary retained it.

[Source](/reference-sources/Text.Json.Value.cinch) · line 356

<a id="value-6a736f6e2d68746d6c2d657363617065"></a>

## `json-html-escape`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-html-escape</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Escape HTML-sensitive bytes and line separators without validating the source as JSON.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1984

<a id="value-6a736f6e2d696e64656e74"></a>

## `json-indent`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-indent</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Indent one complete JSON value with arbitrary per-line prefix and indentation strings.

[Source](/reference-sources/Text.Json.Value.cinch) · line 2138

<a id="value-6a736f6e2d696e64656e742d6279746573"></a>

## `json-indent-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-indent-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Indent one complete value with byte-exact line-prefix and indentation fragments.

[Source](/reference-sources/Text.Json.Value.cinch) · line 2147

<a id="value-6a736f6e2d696e64656e742d707265666978"></a>

## `json-indent-prefix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-indent-prefix</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Format a lexically complete JSON prefix without requiring its containers or pending members to be complete.
Every token must be complete. This operation retains only token spelling
and indentation; it does not accept a prefix as a complete JSON value.

[Source](/reference-sources/Text.Json.Value.cinch) · line 2107

<a id="value-6a736f6e2d6c65782d72656164"></a>

## `json-lex-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-lex-read</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexState</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexStep</span>)))))</code></pre>

Scan at most one token from a checked owned buffer interval, resuming a partial prior token.
Base is the absolute position of buffer byte zero; returned cursors are local to this buffer.

[Source](/reference-sources/Text.Json.Value.cinch) · line 981

<a id="value-6a736f6e2d6c65782d7374617274"></a>

## `json-lex-start`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-lex-start</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexState</span></code></pre>

Begin scanning one token without assuming any source chunk is already available.

[Source](/reference-sources/Text.Json.Value.cinch) · line 861

<a id="value-6a736f6e2d6c6578656d652d656e64"></a>

## `json-lexeme-end`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-lexeme-end</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexeme</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe a lexical token's original exclusive end.

[Source](/reference-sources/Text.Json.Value.cinch) · line 815

<a id="value-6a736f6e2d6c6578656d652d7374617274"></a>

## `json-lexeme-start`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-lexeme-start</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexeme</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe a lexical token's original start.

[Source](/reference-sources/Text.Json.Value.cinch) · line 811

<a id="value-6a736f6e2d6c6578656d652d746167"></a>

## `json-lexeme-tag`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-lexeme-tag</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexeme</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonLexTag</span>)</code></pre>

Observe a lexical category without decoding its source bytes.

[Source](/reference-sources/Text.Json.Value.cinch) · line 819

<a id="value-6a736f6e2d6d656d6265722d6b6579"></a>

## `json-member-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-member-key</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonMember</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span>))</code></pre>

Retain the exact original quoted member-name token as an immutable source-backed string view.

[Source](/reference-sources/Text.Json.Value.cinch) · line 2165

<a id="value-6a736f6e2d6d656d6265722d6e616d65"></a>

## `json-member-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-member-name</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonMember</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Observe a decoded object member name.

[Source](/reference-sources/Text.Json.Value.cinch) · line 605

<a id="value-6a736f6e2d6d656d6265722d76616c7565"></a>

## `json-member-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-member-value</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonMember</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span>)</code></pre>

Observe the member value while retaining its source document.

[Source](/reference-sources/Text.Json.Value.cinch) · line 609

<a id="value-6a736f6e2d6e756d626572"></a>

## `json-number`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-number</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumber</span>))</code></pre>

Validate a complete number spelling without accepting surrounding whitespace or another value.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1394

<a id="value-6a736f6e2d6e756d6265722d6279746573"></a>

## `json-number-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-number-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumber</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Export a fresh exact spelling independently of scalar text projection.

[Source](/reference-sources/Text.Json.Value.cinch) · line 498

<a id="value-6a736f6e2d6e756d6265722d636865636b"></a>

## `json-number-check`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-number-check</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumber</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumber</span>))</code></pre>

Validate the exact spelling of a raw number without accepting repaired or surrounding text.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1634

<a id="value-6a736f6e2d6e756d6265722d666c6f61743332"></a>

## `json-number-float32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-number-float32</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumber</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">F32</span>))</code></pre>

Round an exact number directly to finite binary32, rejecting empty spellings and overflow.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1649

<a id="value-6a736f6e2d6e756d6265722d666c6f61743634"></a>

## `json-number-float64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-number-float64</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumber</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">F64</span>))</code></pre>

Convert an exact number to a finite binary float, naming overflow instead of returning infinity.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1641

<a id="value-6a736f6e2d6e756d6265722d666c6f617436342d726573756c74"></a>

## `json-number-float64-result`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-number-float64-result</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumber</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericResult</span> <span class="tok-constructor">F64</span>))</code></pre>

Convert arbitrary decimal, hexadecimal, separator and special spellings with exact original outcomes.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1616

<a id="value-6a736f6e2d6e756d6265722d66726f6d2d6279746573"></a>

## `json-number-from-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-number-from-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumber</span>)</code></pre>

Snapshot an arbitrary byte spelling, preserving malformed UTF-8 for conversion diagnostics.

[Source](/reference-sources/Text.Json.Value.cinch) · line 494

<a id="value-6a736f6e2d6e756d6265722d66726f6d2d74657874"></a>

## `json-number-from-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-number-from-text</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumber</span>)</code></pre>

Preserve an arbitrary scalar spelling without treating its construction as numeric validation.

[Source](/reference-sources/Text.Json.Value.cinch) · line 490

<a id="value-6a736f6e2d6e756d6265722d696e743634"></a>

## `json-number-int64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-number-int64</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumber</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">I64</span>))</code></pre>

Convert an integer-only exact spelling to a signed machine integer with explicit range checks.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1657

<a id="value-6a736f6e2d6e756d6265722d696e7436342d726573756c74"></a>

## `json-number-int64-result`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-number-int64-result</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumber</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericResult</span> <span class="tok-constructor">I64</span>))</code></pre>

Convert arbitrary signed decimal spellings, returning saturated range outcomes and original metadata.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1427

<a id="value-6a736f6e2d6e756d6265722d74657874"></a>

## `json-number-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-number-text</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumber</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Project the original spelling to scalar text; the byte view retains malformed UTF-8 exactly.

[Source](/reference-sources/Text.Json.Value.cinch) · line 486

<a id="value-6a736f6e2d6e756d657269632d6572726f72"></a>

## `json-numeric-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-numeric-error</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericResult</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericError</span>))</code></pre>

Inspect an optional original numeric refusal without discarding its completed value.

[Source](/reference-sources/Text.Json.Value.cinch) · line 469

<a id="value-6a736f6e2d6e756d657269632d6572726f722d6279746573"></a>

## `json-numeric-error-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-numeric-error-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericError</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Export an independent copy of the original input, including malformed UTF-8.

[Source](/reference-sources/Text.Json.Value.cinch) · line 442

<a id="value-6a736f6e2d6e756d657269632d6572726f722d66756e6374696f6e"></a>

## `json-numeric-error-function`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-numeric-error-function</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Inspect the original operation label associated with a numeric refusal.

[Source](/reference-sources/Text.Json.Value.cinch) · line 438

<a id="value-6a736f6e2d6e756d657269632d6572726f722d696e666f"></a>

## `json-numeric-error-info`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-numeric-error-info</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericErrorKind</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericError</span>)))</code></pre>

Snapshot explicit operation, original bytes and refusal reason for an application error adapter.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1407

<a id="value-6a736f6e2d6e756d657269632d6572726f722d696e707574"></a>

## `json-numeric-error-input`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-numeric-error-input</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Project the original input to scalar text, replacing malformed UTF-8 while its byte view stays exact.

[Source](/reference-sources/Text.Json.Value.cinch) · line 446

<a id="value-6a736f6e2d6e756d657269632d6572726f722d6b696e64"></a>

## `json-numeric-error-kind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-numeric-error-kind</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericErrorKind</span>)</code></pre>

Inspect the exact syntax or range classification independently of its message.

[Source](/reference-sources/Text.Json.Value.cinch) · line 450

<a id="value-6a736f6e2d6e756d657269632d6572726f722d6d657373616765"></a>

## `json-numeric-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-numeric-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render numeric metadata using the original operation and a scalar-safe quoted input.

[Source](/reference-sources/Text.Json.Value.cinch) · line 458

<a id="value-6a736f6e2d6e756d657269632d69732d72616e6765"></a>

## `json-numeric-is-range`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-numeric-is-range</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericError</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether a numeric refusal reports representability rather than syntax.

[Source](/reference-sources/Text.Json.Value.cinch) · line 454

<a id="value-6a736f6e2d6e756d657269632d726573756c74"></a>

## `json-numeric-result`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-numeric-result</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericResult</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericError</span> <span class="tok-variable">a</span>))</code></pre>

Select an ordinary success or refusal when the caller does not need a partial numeric value.

[Source](/reference-sources/Text.Json.Value.cinch) · line 473

<a id="value-6a736f6e2d6e756d657269632d76616c7565"></a>

## `json-numeric-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-numeric-value</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumericResult</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-variable">a</span>)</code></pre>

Inspect the completed value even when conversion also reports an error.

[Source](/reference-sources/Text.Json.Value.cinch) · line 465

<a id="value-6a736f6e2d6f626a6563742d6c6f6f6b7570"></a>

## `json-object-lookup`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-object-lookup</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span>)))</code></pre>

Find the final exact decoded member name; later duplicates replace earlier matches.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1743

<a id="value-6a736f6e2d6f626a6563742d6d656d62657273"></a>

## `json-object-members`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-object-members</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonMember</span>)))</code></pre>

Read object members in original order, preserving repeated decoded names.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1720

<a id="value-6a736f6e2d7072656669782d6e6578742d6f6666736574"></a>

## `json-prefix-next-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-prefix-next-offset</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Nat</span>)))))</code></pre>

Count the logical byte position of a next value after a lexically complete prefix.
The caller includes any upcoming colon or comma in the prefix. Pretty
mode additionally counts colon spacing and pending container indentation.
This is a diagnostic operation, not complete-document validation.

[Source](/reference-sources/Text.Json.Value.cinch) · line 2122

<a id="value-6a736f6e2d70726573657276652d6572726f722d636f6e74657874"></a>

## `json-preserve-error-context`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-preserve-error-context</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>)</code></pre>

Retain application-owned context across automatic encoding container decoration.

[Source](/reference-sources/Text.Json.Value.cinch) · line 360

<a id="value-6a736f6e2d71756f7465"></a>

## `json-quote`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-quote</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Encode an immutable scalar string with explicit HTML escaping policy.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1940

<a id="value-6a736f6e2d71756f74652d6279746573"></a>

## `json-quote-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-quote-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Encode arbitrary byte-string contents as valid quoted JSON, replacing each malformed UTF-8 byte.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1933

<a id="value-6a736f6e2d7261772d6279746573"></a>

## `json-raw-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-raw-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonRawMessage</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Obtain an independent copy of raw storage, preserving the distinction between null and empty.

[Source](/reference-sources/Text.Json.Value.cinch) · line 528

<a id="value-6a736f6e2d7261772d66726f6d2d6279746573"></a>

## `json-raw-from-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-raw-from-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonRawMessage</span>)</code></pre>

Snapshot an arbitrary raw message without changing whitespace or validating its syntax.

[Source](/reference-sources/Text.Json.Value.cinch) · line 524

<a id="value-6a736f6e2d7261772d6e756c6c"></a>

## `json-raw-null`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-raw-null</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonRawMessage</span></code></pre>

The absent raw message, which encodes as the JSON null literal.

[Source](/reference-sources/Text.Json.Value.cinch) · line 520

<a id="value-6a736f6e2d7261772d7265706c616365"></a>

## `json-raw-replace`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-raw-replace</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonRawMessage</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonRawMessage</span>))</code></pre>

Replace a raw message with a fresh snapshot while retained earlier messages stay independent.

[Source](/reference-sources/Text.Json.Value.cinch) · line 534

<a id="value-6a736f6e2d73656d616e7469632d616374696f6e"></a>

## `json-semantic-action`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-semantic-action</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Inspect the conversion operation, such as marshal or unmarshal.

[Source](/reference-sources/Text.Json.Value.cinch) · line 156

<a id="value-6a736f6e2d73656d616e7469632d6279746573"></a>

## `json-semantic-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-semantic-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Export an independent copy of the original complete source token.

[Source](/reference-sources/Text.Json.Value.cinch) · line 168

<a id="value-6a736f6e2d73656d616e7469632d6669656c642d70617468"></a>

## `json-semantic-field-path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-semantic-field-path</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Inspect conventional field context only when supplied independently from the precise semantic pointer.

[Source](/reference-sources/Text.Json.Value.cinch) · line 184

<a id="value-6a736f6e2d73656d616e7469632d696e666f"></a>

## `json-semantic-info`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-semantic-info</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span>)))))</code></pre>

Snapshot exact source bytes while constructing explicit semantic conversion metadata.

[Source](/reference-sources/Text.Json.Value.cinch) · line 112

<a id="value-6a736f6e2d73656d616e7469632d696e666f2d776974682d6f7074696f6e616c2d74797065"></a>

## `json-semantic-info-with-optional-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-semantic-info-with-optional-type</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span>)))))))</code></pre>

Snapshot exact semantic bytes while preserving absence of an application type independently of an empty label.

[Source](/reference-sources/Text.Json.Value.cinch) · line 132

<a id="value-6a736f6e2d73656d616e7469632d696e666f2d776974682d76616c7565"></a>

## `json-semantic-info-with-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-semantic-info-with-value</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span>)))))))</code></pre>

Snapshot source context and independently reported value bytes, retaining an explicit lexical kind byte.
Kind bytes use the opening delimiter, n/t/f for null and booleans, digit
zero for a number, and zero when the conversion does not identify a kind.

[Source](/reference-sources/Text.Json.Value.cinch) · line 128

<a id="value-6a736f6e2d73656d616e7469632d6b696e64"></a>

## `json-semantic-kind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-semantic-kind</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonKind</span>)</code></pre>

Inspect the semantic source category while retaining exact lexical distinctions through the byte getter.

[Source](/reference-sources/Text.Json.Value.cinch) · line 140

<a id="value-6a736f6e2d73656d616e7469632d6b696e642d62797465"></a>

## `json-semantic-kind-byte`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-semantic-kind-byte</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

Inspect the exact lexical kind byte, preserving true, false and unspecified distinctions.

[Source](/reference-sources/Text.Json.Value.cinch) · line 136

<a id="value-6a736f6e2d73656d616e7469632d6f6666736574"></a>

## `json-semantic-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-semantic-offset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Inspect the original zero-based byte offset at which semantic conversion began.

[Source](/reference-sources/Text.Json.Value.cinch) · line 160

<a id="value-6a736f6e2d73656d616e7469632d706f696e746572"></a>

## `json-semantic-pointer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-semantic-pointer</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Inspect the escaped JSON pointer identifying the original source value.

[Source](/reference-sources/Text.Json.Value.cinch) · line 164

<a id="value-6a736f6e2d73656d616e7469632d7265706f727465642d6279746573"></a>

## `json-semantic-reported-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-semantic-reported-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Export a fresh copy of the independently reported value, which may be empty despite retained original source bytes.

[Source](/reference-sources/Text.Json.Value.cinch) · line 148

<a id="value-6a736f6e2d73656d616e7469632d74797065"></a>

## `json-semantic-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-semantic-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Inspect the application type as text, using empty text when no identity was supplied.

[Source](/reference-sources/Text.Json.Value.cinch) · line 172

<a id="value-6a736f6e2d73656d616e7469632d747970652d6f7074696f6e"></a>

## `json-semantic-type-option`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-semantic-type-option</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Inspect the optional application identity without conflating absence and an explicitly empty label.

[Source](/reference-sources/Text.Json.Value.cinch) · line 176

<a id="value-6a736f6e2d73656d616e7469632d776974682d6669656c642d70617468"></a>

## `json-semantic-with-field-path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-semantic-with-field-path</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span>))</code></pre>

Replace conventional field context while sharing immutable source, identity and reported-value owners.

[Source](/reference-sources/Text.Json.Value.cinch) · line 188

<a id="value-6a736f6e2d73656d616e7469632d776974682d6f6666736574"></a>

## `json-semantic-with-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-semantic-with-offset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span>))</code></pre>

Replace only the semantic byte offset while sharing immutable token and reported-value owners.

[Source](/reference-sources/Text.Json.Value.cinch) · line 209

<a id="value-6a736f6e2d73656d616e7469632d776974682d706f696e746572"></a>

## `json-semantic-with-pointer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-semantic-with-pointer</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span>))</code></pre>

Replace only the pointer while retaining immutable source, reported value and operation metadata.

[Source](/reference-sources/Text.Json.Value.cinch) · line 152

<a id="value-6a736f6e2d73656d616e7469632d776974682d74797065"></a>

## `json-semantic-with-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-semantic-with-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonSemanticInfo</span>))</code></pre>

Replace only optional application identity while retaining immutable source and reported-value owners.

[Source](/reference-sources/Text.Json.Value.cinch) · line 180

<a id="value-6a736f6e2d747970652d6572726f722d6669656c64"></a>

## `json-type-error-field`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-error-field</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonTypeInfo</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Inspect the original escaped application field path.

[Source](/reference-sources/Text.Json.Value.cinch) · line 99

<a id="value-6a736f6e2d747970652d6572726f722d6f6666736574"></a>

## `json-type-error-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-error-offset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonTypeInfo</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Inspect the original byte offset reported by a type refusal.

[Source](/reference-sources/Text.Json.Value.cinch) · line 91

<a id="value-6a736f6e2d747970652d6572726f722d7265636f7264"></a>

## `json-type-error-record`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-error-record</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonTypeInfo</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Inspect the independently named root application record.

[Source](/reference-sources/Text.Json.Value.cinch) · line 95

<a id="value-6a736f6e2d747970652d6572726f722d74797065"></a>

## `json-type-error-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-error-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonTypeInfo</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Inspect the optional application identity of a type refusal.

[Source](/reference-sources/Text.Json.Value.cinch) · line 83

<a id="value-6a736f6e2d747970652d6572726f722d76616c7565"></a>

## `json-type-error-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-error-value</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonTypeInfo</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Inspect the original source value description of a type refusal.

[Source](/reference-sources/Text.Json.Value.cinch) · line 87

<a id="value-6a736f6e2d747970652d696e666f"></a>

## `json-type-info`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-info</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonTypeInfo</span>)))))</code></pre>

Construct explicit type metadata without conflating an absent identity and an empty application label.

[Source](/reference-sources/Text.Json.Value.cinch) · line 79

<a id="value-6a736f6e2d747970652d696e666f2d776974682d7265636f72642d6669656c64"></a>

## `json-type-info-with-record-field`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-info-with-record-field</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonTypeInfo</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonTypeInfo</span>)))</code></pre>

Replace record and field context while sharing immutable identity and value metadata.

[Source](/reference-sources/Text.Json.Value.cinch) · line 103

<a id="value-6a736f6e2d76616c6964"></a>

## `json-valid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-valid</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Validate JSON byte grammar without constructing a document or decoding scalar strings.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1247

<a id="value-6a736f6e2d76616c75652d626f6f6c"></a>

## `json-value-bool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-value-bool</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bool</span>))</code></pre>

Read a boolean literal without treating other categories as false.

[Source](/reference-sources/Text.Json.Value.cinch) · line 660

<a id="value-6a736f6e2d76616c75652d656e64"></a>

## `json-value-end`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-value-end</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe the selected value's original exclusive source end.

[Source](/reference-sources/Text.Json.Value.cinch) · line 633

<a id="value-6a736f6e2d76616c75652d6b696e64"></a>

## `json-value-kind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-value-kind</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonKind</span>)</code></pre>

Observe a value category without traversing or decoding its subtree.

[Source](/reference-sources/Text.Json.Value.cinch) · line 637

<a id="value-6a736f6e2d76616c75652d6e756d626572"></a>

## `json-value-number`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-value-number</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumber</span>))</code></pre>

Read a validated exact number from a source-backed scalar view.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1386

<a id="value-6a736f6e2d76616c75652d736f75726365"></a>

## `json-value-source`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-value-source</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Copy exactly the selected value's original bytes, preserving duplicate members and number spellings.

[Source](/reference-sources/Text.Json.Value.cinch) · line 652

<a id="value-6a736f6e2d76616c75652d7374617274"></a>

## `json-value-start`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-value-start</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe the selected value's zero-based original source start.

[Source](/reference-sources/Text.Json.Value.cinch) · line 629

<a id="value-6a736f6e2d76616c75652d737472696e67"></a>

## `json-value-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-value-string</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Decode a string value without exposing mutable source storage.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1753

<a id="value-6a736f6e2d77726974652d696e666f"></a>

## `json-write-info`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-write-info</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonWriteInfo</span>))</code></pre>

Snapshot written output and logical delimiter context independently from caller mutation.

[Source](/reference-sources/Text.Json.Value.cinch) · line 197

<a id="value-6a736f6e2d77726974652d6c6f676963616c2d707265666978"></a>

## `json-write-logical-prefix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-write-logical-prefix</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonWriteInfo</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Export the compact next-value context, including delimiters counted before they are emitted.

[Source](/reference-sources/Text.Json.Value.cinch) · line 205

<a id="value-6a736f6e2d77726974652d7772697474656e2d6279746573"></a>

## `json-write-written-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-write-written-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonWriteInfo</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Export a fresh copy of the output emitted before an encoding refusal.

[Source](/reference-sources/Text.Json.Value.cinch) · line 201

<a id="value-6a736f6e2d7a65726f2d6e756d626572"></a>

## `json-zero-number`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-zero-number</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumber</span></code></pre>

The invariant exact integer-zero number, usable as a total descriptor default.

[Source](/reference-sources/Text.Json.Value.cinch) · line 1377

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Json/Value.cinch`. Source SHA-256: `a330ddfa8c6b0ee57f842efc1f73d3a8c46c9173314704121f644cedba7a3758`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
