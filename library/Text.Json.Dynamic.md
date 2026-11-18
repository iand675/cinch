# Text.Json.Dynamic

Owned dynamic JSON trees, separate encoding and decoding descriptors, typed payloads and bounded construction.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Json.Dynamic`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Json.Dynamic.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Json.Codec](Text.Json.Codec.md), [Text.Json.Value](Text.Json.Value.md).

Non-embedded source closure: [Data.Array](Data.Array.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Json.Binary32](Text.Json.Binary32.md), [Text.Json.Codec](Text.Json.Codec.md), [Text.Json.Dynamic](Text.Json.Dynamic.md), [Text.Json.Value](Text.Json.Value.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Owned dynamic JSON values with exact number spellings and explicit container identity.
Array and object spines are private. Traversals use explicit work frames so
nesting consumes managed storage rather than one native activation per level.
Raw application strings and names retain exact bytes; encoding repairs malformed
UTF-8 before ordering names. Decoded objects retain the last occurrence of a name.
Explicit typed payloads retain their application value and descriptor capability;
arbitrary mutable application storage follows that descriptor's ownership contract.

</details>

# Types and constructors

<a id="type-4a736f6e416e79"></a>

## `JsonAny`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonAny</span></code></pre>

Opaque dynamic payloads with direct recursive container spines.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 30

# Values

<a id="value-6a736f6e2d616e792d6172726179"></a>

## `json-any-array`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-array</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span>)</code></pre>

Construct a present array in application order, retaining immutable child owners.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 108

<a id="value-6a736f6e2d616e792d61727261792d76616c756573"></a>

## `json-any-array-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-array-values</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span>)))</code></pre>

Inspect array items; a typed null array is represented by a present empty item list.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 180

<a id="value-6a736f6e2d616e792d61732d626f6f6c"></a>

## `json-any-as-bool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-as-bool</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bool</span>))</code></pre>

Inspect a present boolean, including false.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 156

<a id="value-6a736f6e2d616e792d61732d6279746573"></a>

## `json-any-as-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-as-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Inspect independently copied exact application string bytes.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 168

<a id="value-6a736f6e2d616e792d61732d666c6f61743634"></a>

## `json-any-as-float64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-as-float64</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">F64</span>))</code></pre>

Inspect a floating payload without coercing an exact-number payload.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 160

<a id="value-6a736f6e2d616e792d61732d6e756d626572"></a>

## `json-any-as-number`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-as-number</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumber</span>))</code></pre>

Inspect an exact-number payload without formatting floating values.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 164

<a id="value-6a736f6e2d616e792d61732d737472696e67"></a>

## `json-any-as-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-as-string</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Project application string bytes to scalars, replacing each malformed byte with U+FFFD.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 172

<a id="value-6a736f6e2d616e792d626f6f6c"></a>

## `json-any-bool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-bool</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span>)</code></pre>

Construct a present boolean application value.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 65

<a id="value-6a736f6e2d616e792d6279746573"></a>

## `json-any-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span>)</code></pre>

Snapshot arbitrary application string bytes, preserving malformed sequences until encoding.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 81

<a id="value-6a736f6e2d616e792d636f6e7461696e65722d69732d6e756c6c"></a>

## `json-any-container-is-null`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-container-is-null</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Distinguish a present typed null collection from a present empty collection.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 152

<a id="value-6a736f6e2d616e792d666c6f61743634"></a>

## `json-any-float64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-float64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span>)</code></pre>

Construct a binary64 payload; encoding reports nonfinite values explicitly.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 69

<a id="value-6a736f6e2d616e792d69732d6e756c6c"></a>

## `json-any-is-null`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-is-null</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Only an absent dynamic payload is zero or empty for application-field omission.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 148

<a id="value-6a736f6e2d616e792d6b696e64"></a>

## `json-any-kind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-kind</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonKind</span>)</code></pre>

Inspect known representation categories; an opaque typed callback has the unspecified category until encoded.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 132

<a id="value-6a736f6e2d616e792d6e756c6c"></a>

## `json-any-null`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-null</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span></code></pre>

Construct the absent dynamic application value.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 61

<a id="value-6a736f6e2d616e792d6e756c6c2d6172726179"></a>

## `json-any-null-array`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-null-array</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span></code></pre>

A typed null array remains a present dynamic payload and marshals as null.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 85

<a id="value-6a736f6e2d616e792d6e756c6c2d6f626a656374"></a>

## `json-any-null-object`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-null-object</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span></code></pre>

A typed null object remains a present dynamic payload and marshals as null.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 89

<a id="value-6a736f6e2d616e792d6e756d626572"></a>

## `json-any-number`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-number</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonNumber</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span>)</code></pre>

Construct an exact-number payload without silently converting it to binary64.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 73

<a id="value-6a736f6e2d616e792d6f626a656374"></a>

## `json-any-object`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-object</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span>)) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span>)</code></pre>

Construct an object with exact byte keys in lexical order; later equal keys replace earlier values.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 128

<a id="value-6a736f6e2d616e792d6f626a6563742d6d656d62657273"></a>

## `json-any-object-members`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-object-members</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span>))))</code></pre>

Inspect exact application object names in raw-byte order, each returned name independently owned.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 192

<a id="value-6a736f6e2d616e792d737472696e67"></a>

## `json-any-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span>)</code></pre>

Construct a present scalar string with independently owned UTF-8 bytes.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 77

<a id="value-6a736f6e2d616e792d74797065"></a>

## `json-any-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span>)</code></pre>

Dynamic descriptor: absent-only omission, exact-number selection and iterative owned traversal.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 379

<a id="value-6a736f6e2d616e792d7479706564"></a>

## `json-any-typed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-typed</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span>))</code></pre>

Retain an application value and its descriptor for use inside mixed dynamic containers.
Encoding invokes the descriptor with the current options. A typed payload is
present for omission, including a typed value whose descriptor emits null.
Mutable application data follows the supplied descriptor's aliasing contract.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 96

<a id="value-6a736f6e2d616e792d74797065642d6c6162656c"></a>

## `json-any-typed-label`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-typed-label</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Inspect an opaque typed payload's label without fabricating a decoded representation.

[Source](/reference-sources/Text.Json.Dynamic.cinch) · line 100

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Json/Dynamic.cinch`. Source SHA-256: `81d5911b9b49fcb34e1cf42b95189a13b332cfcaacea6bb0d28a6bfbe239ea93`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
