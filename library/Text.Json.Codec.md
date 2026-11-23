# Text.Json.Codec

Typed JSON codecs for records, persistent maps, explicit key capabilities, machine integers, binary buffers and exact binary32/binary64 values.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Json.Codec`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Json.Codec.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Array](Data.Array.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Json.Binary32](Text.Json.Binary32.md), [Text.Json.Value](Text.Json.Value.md).

Non-embedded source closure: [Data.Array](Data.Array.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Json.Binary32](Text.Json.Binary32.md), [Text.Json.Codec](Text.Json.Codec.md), [Text.Json.Value](Text.Json.Value.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Typed JSON descriptors, explicit application record capabilities and immutable conversion policies.
Descriptors make type metadata and custom conversion callbacks explicit. Decoding
can retain a partially updated value together with its first structured error.

</details>

# Types and constructors

<a id="type-4a736f6e4465636f646564"></a>

## `JsonDecoded`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonDecoded</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A converted application value, its first error and whether continuation was refused.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 258

<a id="type-4a736f6e456e636f646564"></a>

## `JsonEncoded`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonEncoded</span></code></pre>

An encoding outcome retains successful output or the exact prefix emitted before its first refusal.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 446

<a id="type-4a736f6e4669656c64"></a>

## `JsonField`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonField</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A field's typed projection/update callbacks are sealed together with its omission metadata.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1320

<a id="type-4a736f6e4669656c644d61746368"></a>

## `JsonFieldMatch`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonFieldMatch</span></code></pre>

Per-field name matching takes precedence over record-wide folding behavior.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1298

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonFieldDefaultMatch</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonFieldMatch</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonFieldExactMatch</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonFieldMatch</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonFieldFoldMatch</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonFieldMatch</span></code></pre>

<a id="type-4a736f6e4669656c644f7074696f6e73"></a>

## `JsonFieldOptions`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonFieldOptions</span></code></pre>

Immutable application field metadata, independent of reflection or source-language tags.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1307

<a id="type-4a736f6e4b6579"></a>

## `JsonKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonKey</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Typed object-key capabilities make ordering and custom text/token methods explicit.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1940

<a id="type-4a736f6e4d6170"></a>

## `JsonMap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonMap</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Native immutable maps distinguish a null map from an allocated empty map.
Application keys must retain a stable total ordering while stored in a map.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 2062

<a id="type-4a736f6e4d6574686f6473"></a>

## `JsonMethods`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonMethods</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Independent JSON/text conversion methods with explicit encoding addressability requirements.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 648

<a id="type-4a736f6e506f6c6963696573"></a>

## `JsonPolicies`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonPolicies</span></code></pre>

An immutable set of enabled conversion policies.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 75

<a id="type-4a736f6e506f6c696379"></a>

## `JsonPolicy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonPolicy</span></code></pre>

Independently selectable typed conversion semantics.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 48

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonAddressableMethodsOnly</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicy</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonByteArrayNumbers</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicy</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonNamedBytesAsBinary</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicy</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonDurationNanoseconds</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicy</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonPreserveNameDelimiters</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicy</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonPreserveScalarNull</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicy</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonOmitEmptyApplicationValues</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicy</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonLooseBase64Padding</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicy</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonLooseTimestamps</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicy</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonContinueAfterTypeErrors</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicy</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonQuoteScalarFields</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicy</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JsonAllowArrayLengthMismatch</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicy</span></code></pre>

<a id="type-4a736f6e506f6c6963794f7074696f6e"></a>

## `JsonPolicyOption`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonPolicyOption</span></code></pre>

One immutable policy assignment; later assignments override earlier ones.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 80

<a id="type-4a736f6e526561644f7074696f6e73"></a>

## `JsonReadOptions`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonReadOptions</span></code></pre>

Decoding configuration independent of any particular application type.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 225

<a id="type-4a736f6e54797065"></a>

## `JsonType`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonType</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Explicit type metadata and conversion callbacks replace implicit runtime reflection.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 291

<a id="type-4a736f6e57726974654f7074696f6e73"></a>

## `JsonWriteOptions`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonWriteOptions</span></code></pre>

Encoding configuration shared by nested descriptors and the final formatter.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 175

# Values

<a id="value-6a736f6e2d6164647265737361626c652d6d6574686f64732d6f6e6c79"></a>

## `json-addressable-methods-only`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-addressable-methods-only</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicyOption</span>)</code></pre>

Addressability-sensitive custom methods and text-first object keys.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 127

<a id="value-6a736f6e2d616c6c6f772d61727261792d6c656e6774682d6d69736d61746368"></a>

## `json-allow-array-length-mismatch`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-allow-array-length-mismatch</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicyOption</span>)</code></pre>

Fixed arrays accept different source lengths, zeroing unused elements.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 171

<a id="value-6a736f6e2d626f6f6c2d74797065"></a>

## `json-bool-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-bool-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-constructor">Bool</span>)</code></pre>

The boolean descriptor with false as both zero and empty.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 792

<a id="value-6a736f6e2d627974652d61727261792d6e756d62657273"></a>

## `json-byte-array-numbers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-byte-array-numbers</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicyOption</span>)</code></pre>

Fixed byte arrays use numeric arrays instead of binary text.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 131

<a id="value-6a736f6e2d627974652d6b6579"></a>

## `json-byte-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-byte-key</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">k</span> -&gt; <span class="tok-constructor">I64</span>)) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; (<span class="tok-variable">k</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>))) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonReadOptions</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonMember</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-variable">k</span>))) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonKey</span> <span class="tok-variable">k</span>)))))</code></pre>

Convert raw member-name bytes, normalizing malformed UTF-8 before sorting and JSON quoting.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1959

<a id="value-6a736f6e2d627974652d736c6963652d74797065"></a>

## `json-byte-slice-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-byte-slice-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Nullable binary byte slices preserve the null/empty distinction and numeric-array update behavior.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1890

<a id="value-6a736f6e2d62797465732d74797065"></a>

## `json-bytes-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-bytes-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-constructor">Bytes</span>)</code></pre>

Non-null binary bytes; null clears to an empty buffer and marshaling always emits binary text.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1858

<a id="value-6a736f6e2d636f6e6669677572652d706f6c6963696573"></a>

## `json-configure-policies`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-configure-policies</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicies</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicyOption</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicies</span>))</code></pre>

Apply assignments left to right, retaining the last setting for each policy.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 113

<a id="value-6a736f6e2d636f6e74696e75652d61667465722d747970652d6572726f7273"></a>

## `json-continue-after-type-errors`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-continue-after-type-errors</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicyOption</span>)</code></pre>

Recoverable field mismatches retain the first error and continue.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 163

<a id="value-6a736f6e2d637573746f6d2d74797065"></a>

## `json-custom-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-custom-type</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Unit</span> -&gt; <span class="tok-variable">a</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>))) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonReadOptions</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonDecoded</span> <span class="tok-variable">a</span>)))) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>)))))))</code></pre>

Construct a custom typed capability with explicit defaults, omission tests and update outcomes.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 575

<a id="value-6a736f6e2d6465636f6465642d6572726f72"></a>

## `json-decoded-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-decoded-error</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonDecoded</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>))</code></pre>

Inspect the first retained structured conversion error.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 279

<a id="value-6a736f6e2d6465636f6465642d6661696c757265"></a>

## `json-decoded-failure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-decoded-failure</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonDecoded</span> <span class="tok-variable">a</span>)))</code></pre>

Construct an unrecoverable custom conversion refusal while retaining the supplied value.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 267

<a id="value-6a736f6e2d6465636f6465642d666174616c"></a>

## `json-decoded-fatal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-decoded-fatal</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonDecoded</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether a custom callback or syntax failure stopped subsequent updates.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 283

<a id="value-6a736f6e2d6465636f6465642d6f6b"></a>

## `json-decoded-ok`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-decoded-ok</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonDecoded</span> <span class="tok-variable">a</span>))</code></pre>

Construct a successful descriptor result.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 263

<a id="value-6a736f6e2d6465636f6465642d726573756c74"></a>

## `json-decoded-result`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-decoded-result</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonDecoded</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-variable">a</span>))</code></pre>

Require successful conversion when a partial application value is not useful.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 287

<a id="value-6a736f6e2d6465636f6465642d76616c7565"></a>

## `json-decoded-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-decoded-value</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonDecoded</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-variable">a</span>)</code></pre>

Read the resulting application value even when conversion also reported an error.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 275

<a id="value-6a736f6e2d6465636f6465642d776974682d6572726f72"></a>

## `json-decoded-with-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-decoded-with-error</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonDecoded</span> <span class="tok-variable">a</span>)))</code></pre>

Construct a recoverable type/range mismatch while retaining the supplied value.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 271

<a id="value-6a736f6e2d64656661756c742d6669656c642d6f7074696f6e73"></a>

## `json-default-field-options`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-default-field-options</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonFieldOptions</span></code></pre>

Ordinary visible fields participate at depth zero without omission or quoting.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1316

<a id="value-6a736f6e2d64656661756c742d706f6c6963696573"></a>

## `json-default-policies`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-default-policies</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicies</span></code></pre>

Established permissive, merge-preserving typed conversion semantics.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 101

<a id="value-6a736f6e2d64656661756c742d726561642d6f7074696f6e73"></a>

## `json-default-read-options`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-default-read-options</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonReadOptions</span></code></pre>

Floating dynamic numbers, ignored unknown fields and established merge semantics.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 234

<a id="value-6a736f6e2d64656661756c742d77726974652d6f7074696f6e73"></a>

## `json-default-write-options`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-default-write-options</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span></code></pre>

Compact encoding with HTML escaping and established typed semantics.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 184

<a id="value-6a736f6e2d6475726174696f6e2d6e616e6f7365636f6e6473"></a>

## `json-duration-nanoseconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-duration-nanoseconds</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicyOption</span>)</code></pre>

Duration descriptors encode an integral nanosecond count.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 139

<a id="value-6a736f6e2d656d7074792d706f6c6963696573"></a>

## `json-empty-policies`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-empty-policies</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicies</span></code></pre>

Explicitly disable all conversion choices before applying selected assignments.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 105

<a id="value-6a736f6e2d656e636f6465642d6279746573"></a>

## `json-encoded-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-encoded-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonEncoded</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Export an independent copy of successful output or the retained partial prefix.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 451

<a id="value-6a736f6e2d656e636f6465642d6572726f72"></a>

## `json-encoded-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-encoded-error</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonEncoded</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span>))</code></pre>

Inspect the original diagnostic without exposing internal output-transport wrappers.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 455

<a id="value-6a736f6e2d656e636f6465642d726573756c74"></a>

## `json-encoded-result`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-encoded-result</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonEncoded</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Require successful encoding when a partial output prefix is not useful.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 459

<a id="value-6a736f6e2d6669656c64"></a>

## `json-field`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-field</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">b</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonField</span> <span class="tok-variable">a</span>)))))</code></pre>

Construct an ordinary visible field without omission, quoting or embedding metadata.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1415

<a id="value-6a736f6e2d6669656c642d6e616d65"></a>

## `json-field-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-field-name</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonField</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Inspect a field's declared JSON member name.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1325

<a id="value-6a736f6e2d6669656c642d6f7074696f6e73"></a>

## `json-field-options`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-field-options</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonFieldMatch</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonFieldOptions</span>))))))</code></pre>

Configure explicit omission, quoting, embedding dominance and member-name matching.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1312

<a id="value-6a736f6e2d6669656c642d77697468"></a>

## `json-field-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-field-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonFieldOptions</span> -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">b</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonField</span> <span class="tok-variable">a</span>))))))</code></pre>

Construct one typed field using explicit projection and functional replacement capabilities.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1406

<a id="value-6a736f6e2d66697865642d61727261792d74797065"></a>

## `json-fixed-array-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-fixed-array-type</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>))))</code></pre>

Compose an exact fixed-array descriptor with immutable element defaults and checked lengths.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1091

<a id="value-6a736f6e2d66697865642d62797465732d74797065"></a>

## `json-fixed-bytes-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-fixed-bytes-type</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Fixed-size owned byte buffers, numeric by default, with explicit binary-string alternate policy.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1927

<a id="value-6a736f6e2d666c6f617433322d74797065"></a>

## `json-float32-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-float32-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-constructor">F32</span>)</code></pre>

The finite binary32 descriptor with exact rounding and shortest JSON decimal formatting.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1215

<a id="value-6a736f6e2d666c6f617436342d74797065"></a>

## `json-float64-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-float64-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-constructor">F64</span>)</code></pre>

The finite binary64 descriptor with shortest JSON decimal formatting.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1191

<a id="value-6a736f6e2d696e7431362d74797065"></a>

## `json-int16-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-int16-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-constructor">I16</span>)</code></pre>

Signed sixteen-bit integers with exact range refusal.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1733

<a id="value-6a736f6e2d696e7433322d74797065"></a>

## `json-int32-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-int32-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-constructor">I32</span>)</code></pre>

Signed thirty-two-bit integers with exact range refusal.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1736

<a id="value-6a736f6e2d696e7436342d6b6579"></a>

## `json-int64-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-int64-key</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonKey</span> <span class="tok-constructor">I64</span>)</code></pre>

Signed64 keys use exact decimal text and original member-name range diagnostics.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 2132

<a id="value-6a736f6e2d696e7436342d74797065"></a>

## `json-int64-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-int64-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-constructor">I64</span>)</code></pre>

The exact signed 64-bit integer descriptor.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 843

<a id="value-6a736f6e2d696e74382d74797065"></a>

## `json-int8-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-int8-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-constructor">I8</span>)</code></pre>

Signed eight-bit integers with exact range refusal.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1730

<a id="value-6a736f6e2d6973697a652d74797065"></a>

## `json-isize-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-isize-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-constructor">Isize</span>)</code></pre>

Target-width signed integers with platform-specific checked bounds.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1739

<a id="value-6a736f6e2d6a736f6e2d6b6579"></a>

## `json-json-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-json-key</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">k</span>) -&gt; ((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">k</span> -&gt; <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonKey</span> <span class="tok-variable">k</span>)))</code></pre>

Use typed JSON key capabilities with numeric stringification and explicit invalid-member-name errors.
Dedicated string/integer key descriptors provide their ordinary primitive text rules.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 2045

<a id="value-6a736f6e2d6b6579"></a>

## `json-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-key</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">k</span> -&gt; <span class="tok-constructor">I64</span>)) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; (<span class="tok-variable">k</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonReadOptions</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonMember</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-variable">k</span>))) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonKey</span> <span class="tok-variable">k</span>)))))</code></pre>

Construct an object-key descriptor with explicit comparator and conversion capabilities.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1947

<a id="value-6a736f6e2d6b65792d6c6162656c"></a>

## `json-key-label`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-key-label</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonKey</span> <span class="tok-variable">k</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Observe the declared application key family.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 2053

<a id="value-6a736f6e2d6c6973742d74797065"></a>

## `json-list-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-list-type</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

Compose non-null arrays represented by ordinary immutable lists.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1025

<a id="value-6a736f6e2d6c6f6f73652d6261736536342d70616464696e67"></a>

## `json-loose-base64-padding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-loose-base64-padding</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicyOption</span>)</code></pre>

Binary text permits embedded CR/LF while preserving the padded alphabet grammar.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 155

<a id="value-6a736f6e2d6c6f6f73652d74696d657374616d7073"></a>

## `json-loose-timestamps`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-loose-timestamps</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicyOption</span>)</code></pre>

Time descriptors permit the explicitly documented compatibility spellings.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 159

<a id="value-6a736f6e2d6d61702d656d707479"></a>

## `json-map-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-map-empty</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonKey</span> <span class="tok-variable">k</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonMap</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>))</code></pre>

Construct an allocated empty map, which marshals as an empty object.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 2071

<a id="value-6a736f6e2d6d61702d656e7472696573"></a>

## `json-map-entries`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-map-entries</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonMap</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>)))</code></pre>

Enumerate application keys in their comparator's order, independently of encoded JSON name sorting.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 2091

<a id="value-6a736f6e2d6d61702d66726f6d2d6c697374"></a>

## `json-map-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-map-from-list</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonKey</span> <span class="tok-variable">k</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonMap</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)))</code></pre>

Construct an allocated map from ordered entries; later comparator-equal keys replace earlier ones.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 2099

<a id="value-6a736f6e2d6d61702d69732d6e756c6c"></a>

## `json-map-is-null`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-map-is-null</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonMap</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Observe nullness independently of the number of stored members.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 2075

<a id="value-6a736f6e2d6d61702d6c6f6f6b7570"></a>

## `json-map-lookup`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-map-lookup</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">k</span> -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonMap</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">v</span>)))</code></pre>

Look up one application key along a logarithmically bounded tail traversal.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 2083

<a id="value-6a736f6e2d6d61702d6e756c6c"></a>

## `json-map-null`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-map-null</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonKey</span> <span class="tok-variable">k</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonMap</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>))</code></pre>

Construct a null map retaining the supplied key family's ordering capability.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 2067

<a id="value-6a736f6e2d6d61702d736574"></a>

## `json-map-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-map-set</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonMap</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonMap</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>))))</code></pre>

Persistently insert or replace an application key, allocating a null map on its first insertion.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 2087

<a id="value-6a736f6e2d6d61702d73697a65"></a>

## `json-map-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-map-size</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonMap</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Read the cached map size in constant time.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 2079

<a id="value-6a736f6e2d6d61702d74797065"></a>

## `json-map-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-map-type</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonKey</span> <span class="tok-variable">k</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">v</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonMap</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>))))</code></pre>

Compose a map descriptor with explicit key capabilities, sorted encoding and logarithmic updates.
The same application ordering must be used to construct prior map values.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 2237

<a id="value-6a736f6e2d6d61727368616c"></a>

## `json-marshal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-marshal</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Marshal one complete compact value with established defaults.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 479

<a id="value-6a736f6e2d6d61727368616c2d617474656d7074"></a>

## `json-marshal-attempt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-marshal-attempt</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonEncoded</span>)))</code></pre>

Marshal with policy-selected partial output and the original error family on failure.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 463

<a id="value-6a736f6e2d6d61727368616c2d696e64656e74"></a>

## `json-marshal-indent`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-marshal-indent</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

Marshal one value with an explicit indentation pass, including empty prefix and step strings.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 483

<a id="value-6a736f6e2d6d61727368616c2d76616c7565"></a>

## `json-marshal-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-marshal-value</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Encode one compact fragment with a directly inspectable original error family.
Complete-fragment callbacks return their own refusal without exposing the
private partial-output transport used between ordinary containers.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 362

<a id="value-6a736f6e2d6d61727368616c2d77697468"></a>

## `json-marshal-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-marshal-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Marshal one complete value using explicit HTML, indentation and conversion policies.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 473

<a id="value-6a736f6e2d6d6574686f6473"></a>

## `json-methods`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-methods</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Bool</span>, (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>))))) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Bool</span>, (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>))))) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonReadOptions</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonDecoded</span> <span class="tok-variable">a</span>))))) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonReadOptions</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonDecoded</span> <span class="tok-variable">a</span>))))) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonMethods</span> <span class="tok-variable">a</span>)))))</code></pre>

Compose optional methods; JSON takes precedence over text in each direction when its encoding receiver is eligible.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 653

<a id="value-6a736f6e2d6d6574686f64732d776974682d72656365697665722d7479706573"></a>

## `json-methods-with-receiver-types`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-methods-with-receiver-types</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonMethods</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonMethods</span> <span class="tok-variable">a</span>))))</code></pre>

Set the independently named JSON and text encoding receivers without changing the application value type.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 661

<a id="value-6a736f6e2d6e616d65642d62797465732d61732d62696e617279"></a>

## `json-named-bytes-as-binary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-named-bytes-as-binary</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicyOption</span>)</code></pre>

Binary text formatting permits named byte-element descriptors.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 135

<a id="value-6a736f6e2d6e6f2d6d6574686f6473"></a>

## `json-no-methods`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-no-methods</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonMethods</span> <span class="tok-variable">a</span>)</code></pre>

An empty method set preserves every ordinary descriptor capability.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 657

<a id="value-6a736f6e2d6e756c6c61626c652d74797065"></a>

## `json-nullable-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-nullable-type</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)))</code></pre>

Compose an explicit nullable/pointee descriptor with ordinary typed update semantics.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 918

<a id="value-6a736f6e2d6e756d6265722d74797065"></a>

## `json-number-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-number-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumber</span>)</code></pre>

Exact number lexemes with checked conversions and an absent-spelling zero default.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 876

<a id="value-6a736f6e2d6f6d69742d656d7074792d6170706c69636174696f6e2d76616c756573"></a>

## `json-omit-empty-application-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-omit-empty-application-values</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicyOption</span>)</code></pre>

Empty-field omission uses the application value classification.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 151

<a id="value-6a736f6e2d706f6c6963696573"></a>

## `json-policies`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-policies</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicyOption</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicies</span>)</code></pre>

Configure the established defaults using explicit immutable assignments.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 123

<a id="value-6a736f6e2d706f6c6963792d656e61626c6564"></a>

## `json-policy-enabled`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-policy-enabled</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicies</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicy</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Inspect one named policy without revealing its representation.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 109

<a id="value-6a736f6e2d70726573657276652d6e616d652d64656c696d6974657273"></a>

## `json-preserve-name-delimiters`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-preserve-name-delimiters</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicyOption</span>)</code></pre>

Case-insensitive record matching preserves underscores and dashes.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 143

<a id="value-6a736f6e2d70726573657276652d7363616c61722d6e756c6c"></a>

## `json-preserve-scalar-null`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-preserve-scalar-null</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicyOption</span>)</code></pre>

Null scalars retain their value and sequence elements merge.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 147

<a id="value-6a736f6e2d71756f74652d7363616c61722d6669656c6473"></a>

## `json-quote-scalar-fields`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-quote-scalar-fields</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicyOption</span>)</code></pre>

Quoted fields permit boolean and string values as well as numbers.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 167

<a id="value-6a736f6e2d7261772d74797065"></a>

## `json-raw-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-raw-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonRawMessage</span>)</code></pre>

Owned raw JSON fragments with copying assignment and validated marshaling.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 898

<a id="value-6a736f6e2d726561642d6f7074696f6e73"></a>

## `json-read-options`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-read-options</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicies</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonReadOptions</span>)))</code></pre>

Construct immutable decoding options without mutating an existing decoder branch.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 230

<a id="value-6a736f6e2d726561642d706f6c6963696573"></a>

## `json-read-policies`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-read-policies</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonReadOptions</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicies</span>)</code></pre>

Typed conversion choices used while applying a decoded value.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 246

<a id="value-6a736f6e2d726561642d72656a6563742d756e6b6e6f776e"></a>

## `json-read-reject-unknown`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-read-reject-unknown</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonReadOptions</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether a member unmatched by a record descriptor is an error.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 242

<a id="value-6a736f6e2d726561642d737472696e676966792d6e756d62657273"></a>

## `json-read-stringify-numbers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-read-stringify-numbers</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonReadOptions</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether numeric descriptors require quoted source numbers in this nested context.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 250

<a id="value-6a736f6e2d726561642d7573652d6e756d626572"></a>

## `json-read-use-number`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-read-use-number</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonReadOptions</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether dynamic-value decoding preserves exact numbers instead of converting to floats.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 238

<a id="value-6a736f6e2d726561642d776974682d737472696e676966792d6e756d62657273"></a>

## `json-read-with-stringify-numbers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-read-with-stringify-numbers</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonReadOptions</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonReadOptions</span>))</code></pre>

Select nested numeric quoting without changing dynamic-number or unknown-field policies.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 254

<a id="value-6a736f6e2d7265636f72642d74797065"></a>

## `json-record-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-record-type</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Unit</span> -&gt; <span class="tok-variable">a</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonField</span> <span class="tok-variable">a</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>)))))</code></pre>

Construct a record descriptor with explicit default/zero capabilities and compiled field dominance.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1618

<a id="value-6a736f6e2d736c6963652d74797065"></a>

## `json-slice-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-slice-type</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))))</code></pre>

Distinguish a null sequence from a present empty array using an explicit optional list.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1029

<a id="value-6a736f6e2d737472696e672d6b6579"></a>

## `json-string-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-string-key</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonKey</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

String keys preserve exact decoded spelling and sort by their encoded UTF-8 bytes.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 2103

<a id="value-6a736f6e2d737472696e672d74797065"></a>

## `json-string-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-string-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The scalar-string descriptor with empty text as both zero and empty.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 806

<a id="value-6a736f6e2d7374727563747572616c2d74797065"></a>

## `json-structural-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-structural-type</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonKind</span> -&gt; ((<span class="tok-constructor">Unit</span> -&gt; <span class="tok-variable">a</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>))) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonReadOptions</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonDecoded</span> <span class="tok-variable">a</span>)))) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>))))))))</code></pre>

Compose a structural descriptor with an explicit category and validated output; child errors retain their original identity.
The category controls containing-field quoting. Successful callback fragments
are checked as complete JSON values before entering a containing descriptor.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 589

<a id="value-6a736f6e2d747970652d64656661756c74"></a>

## `json-type-default`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-default</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-variable">a</span>)</code></pre>

Construct a fresh default application value through its descriptor capability.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 340

<a id="value-6a736f6e2d747970652d69732d656d707479"></a>

## `json-type-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-is-empty</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Apply the descriptor's application empty classifier.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 348

<a id="value-6a736f6e2d747970652d69732d7a65726f"></a>

## `json-type-is-zero`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-is-zero</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Apply the descriptor's application zero classifier.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 344

<a id="value-6a736f6e2d747970652d6b696e64"></a>

## `json-type-kind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-kind</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonKind</span>)</code></pre>

Established write representation category; custom/raw fragments use the explicit unspecified category.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 296

<a id="value-6a736f6e2d747970652d6c6162656c"></a>

## `json-type-label`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-label</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Application type name attached to structured conversion failures.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 312

<a id="value-6a736f6e2d747970652d726561642d6b696e64"></a>

## `json-type-read-kind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-read-kind</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonKind</span>)</code></pre>

Inspect the decoding category independently of encoding methods and their field-quoting behavior.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 308

<a id="value-6a736f6e2d747970652d726561642d6e616d65"></a>

## `json-type-read-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-read-name</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Inspect the independent root name used when reporting an application field path.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 316

<a id="value-6a736f6e2d747970652d776974682d6164647265737361626c652d656e636f646572"></a>

## `json-type-with-addressable-encoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-with-addressable-encoder</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>))) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>)))</code></pre>

Add an addressable-only JSON encoder with ordinary fallback for nonaddressable application values.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 611

<a id="value-6a736f6e2d747970652d776974682d6164647265737361626c652d746578742d656e636f646572"></a>

## `json-type-with-addressable-text-encoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-with-addressable-text-encoder</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>))) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>)))</code></pre>

Add an addressable-only text encoder with ordinary fallback and the current HTML escaping policy.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 615

<a id="value-6a736f6e2d747970652d776974682d6465636f646572"></a>

## `json-type-with-decoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-with-decoder</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonReadOptions</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonDecoded</span> <span class="tok-variable">a</span>)))) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>)))</code></pre>

Replace only JSON decoding, preserving ordinary encoding and its containing-field quoting behavior.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 619

<a id="value-6a736f6e2d747970652d776974682d656e636f646572"></a>

## `json-type-with-encoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-with-encoder</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>))) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>)))</code></pre>

Replace only JSON encoding, preserving ordinary decoding, defaults and application omission predicates.
Successful method fragments are validated and snapshotted before joining a
containing value. A method controls its own encoding representation.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 595

<a id="value-6a736f6e2d747970652d776974682d6d6574686f6473"></a>

## `json-type-with-methods`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-with-methods</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonMethods</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>)))</code></pre>

Install independent methods while preserving defaults, omission predicates and ordinary fallback capabilities.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 777

<a id="value-6a736f6e2d747970652d776974682d726561642d6964656e74697479"></a>

## `json-type-with-read-identity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-with-read-identity</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>))))</code></pre>

Supply independent application and root reporting names without inferring identities from display labels.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 335

<a id="value-6a736f6e2d747970652d776974682d746578742d6465636f646572"></a>

## `json-type-with-text-decoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-with-text-decoder</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonReadOptions</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonDecoded</span> <span class="tok-variable">a</span>)))) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>)))</code></pre>

Replace only text decoding, preserving encoding and passing independently owned decoded string bytes.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 643

<a id="value-6a736f6e2d747970652d776974682d746578742d656e636f646572"></a>

## `json-type-with-text-encoder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-with-text-encoder</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Bytes</span>))) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>)))</code></pre>

Replace only text encoding; method output is a JSON string and ordinary decoding retains its original kind.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 630

<a id="value-6a736f6e2d747970652d77726974652d6b696e64"></a>

## `json-type-write-kind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-write-kind</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonKind</span>)</code></pre>

Inspect the encoding category independently of decoding methods and their field-quoting behavior.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 300

<a id="value-6a736f6e2d747970652d77726974652d6b696e642d77697468"></a>

## `json-type-write-kind-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-type-write-kind-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonKind</span>))</code></pre>

Inspect the encoding category in the exact context used to select an encoding method.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 304

<a id="value-6a736f6e2d75696e7431362d74797065"></a>

## `json-uint16-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-uint16-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-constructor">U16</span>)</code></pre>

Unsigned sixteen-bit integers with exact range refusal.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1745

<a id="value-6a736f6e2d75696e7433322d74797065"></a>

## `json-uint32-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-uint32-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-constructor">U32</span>)</code></pre>

Unsigned thirty-two-bit integers with exact range refusal.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1748

<a id="value-6a736f6e2d75696e7436342d6b6579"></a>

## `json-uint64-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-uint64-key</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonKey</span> <span class="tok-constructor">U64</span>)</code></pre>

Unsigned64 keys use full-range decimal text and eight-digit SWAR conversion blocks.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 2138

<a id="value-6a736f6e2d75696e7436342d74797065"></a>

## `json-uint64-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-uint64-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-constructor">U64</span>)</code></pre>

Full-range unsigned64 numbers with checked parsing and SWAR decimal blocks.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1688

<a id="value-6a736f6e2d75696e74382d74797065"></a>

## `json-uint8-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-uint8-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-constructor">U8</span>)</code></pre>

Unsigned eight-bit integers with exact range refusal.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1742

<a id="value-6a736f6e2d756e6d61727368616c"></a>

## `json-unmarshal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-unmarshal</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-variable">a</span>)))</code></pre>

Decode into a fresh default value, rejecting syntax and conversion failures.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 543

<a id="value-6a736f6e2d756e6d61727368616c2d77697468"></a>

## `json-unmarshal-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-unmarshal-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonReadOptions</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-variable">a</span>))))</code></pre>

Decode into a fresh default value with explicit conversion policies.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 539

<a id="value-6a736f6e2d756e737570706f727465642d6b6579"></a>

## `json-unsupported-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-unsupported-key</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">k</span> -&gt; <span class="tok-constructor">I64</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonKey</span> <span class="tok-variable">k</span>)))</code></pre>

Declare an application key family that has no JSON conversion capability.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 2049

<a id="value-6a736f6e2d757064617465"></a>

## `json-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-update</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonDecoded</span> <span class="tok-variable">a</span>))))</code></pre>

Decode into an existing application value with established defaults and partial-update reporting.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 535

<a id="value-6a736f6e2d7570646174652d76616c7565"></a>

## `json-update-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-update-value</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonReadOptions</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonValue</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonDecoded</span> <span class="tok-variable">a</span>)))))</code></pre>

Apply a retained source value through its immutable typed update capability.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 370

<a id="value-6a736f6e2d7570646174652d77697468"></a>

## `json-update-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-update-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonReadOptions</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonDecoded</span> <span class="tok-variable">a</span>)))))</code></pre>

Apply policy-selected syntax validation while retaining advanced values and the first original refusal.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 487

<a id="value-6a736f6e2d7573697a652d74797065"></a>

## `json-usize-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-usize-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-constructor">Usize</span>)</code></pre>

Target-width unsigned integers with platform-specific checked bounds.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 1751

<a id="value-6a736f6e2d77726974652d6164647265737361626c65"></a>

## `json-write-addressable`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-write-addressable</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether the current application value permits an addressable-only encoding method.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 216

<a id="value-6a736f6e2d77726974652d68746d6c"></a>

## `json-write-html`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-write-html</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether string and raw fragments escape HTML-sensitive characters.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 188

<a id="value-6a736f6e2d77726974652d6f7074696f6e73"></a>

## `json-write-options`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-write-options</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicies</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span>)))))</code></pre>

Construct explicit options; pretty mode remains distinct from an empty indentation string.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 180

<a id="value-6a736f6e2d77726974652d706f6c6963696573"></a>

## `json-write-policies`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-write-policies</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonPolicies</span>)</code></pre>

Typed conversion choices used while constructing a value fragment.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 204

<a id="value-6a736f6e2d77726974652d707265666978"></a>

## `json-write-prefix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-write-prefix</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Prefix inserted before each continuation line.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 196

<a id="value-6a736f6e2d77726974652d707265747479"></a>

## `json-write-pretty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-write-pretty</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether a final whole-value indentation pass is requested.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 192

<a id="value-6a736f6e2d77726974652d73746570"></a>

## `json-write-step`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-write-step</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Indentation inserted once per container depth.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 200

<a id="value-6a736f6e2d77726974652d737472696e676966792d6e756d62657273"></a>

## `json-write-stringify-numbers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-write-stringify-numbers</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether numeric descriptors encode their values as JSON strings in this nested context.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 208

<a id="value-6a736f6e2d77726974652d776974682d616464726573736162696c697479"></a>

## `json-write-with-addressability`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-write-with-addressability</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span>))</code></pre>

Select an explicit application reference boundary while retaining every other encoding option.
This metadata permits method selection without exposing an application address.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 221

<a id="value-6a736f6e2d77726974652d776974682d737472696e676966792d6e756d62657273"></a>

## `json-write-with-stringify-numbers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-write-with-stringify-numbers</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonWriteOptions</span>))</code></pre>

Select nested numeric quoting without changing HTML or indentation configuration.

[Source](/reference-sources/Text.Json.Codec.cinch) · line 212

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Json/Codec.cinch`. Source SHA-256: `75c3031b7c0b3f359c8d00ec01afdc97276f949cb7268ca65ab46271209f4022`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
