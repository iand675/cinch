# Text.Json.Legacy

Explicit legacy codec policy and descriptor bridges over owned dynamic values.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Json.Legacy`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Json.Legacy.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Codec](Text.Codec.md), [Text.Json.Codec](Text.Json.Codec.md), [Text.Json.Dynamic](Text.Json.Dynamic.md), [Text.Json.Value](Text.Json.Value.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Array](Data.Array.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Json.Binary32](Text.Json.Binary32.md), [Text.Json.Codec](Text.Json.Codec.md), [Text.Json.Dynamic](Text.Json.Dynamic.md), [Text.Json.Legacy](Text.Json.Legacy.md), [Text.Json.Value](Text.Json.Value.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Explicit bridges between dynamic JSON and the established Json-shaped codec family.
Integer spellings become arbitrary-precision integers; fractional or exponent
spellings become finite binary64 values. Exact-number spelling and typed null
collection identity are intentionally projected by this explicit conversion.
Malformed application text, invalid numbers and malformed legacy spines are
refused instead of being silently repaired. Existing legacy APIs are unchanged.
Opaque typed application payloads are projected through their encoding capability
once, then decoded with exact number spellings before the established projection.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-6a736f6e2d616e792d66726f6d2d6c6567616379"></a>

## `json-any-from-legacy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-from-legacy</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span>))</code></pre>

Convert established Json-shaped values to owned dynamic values; later duplicate object names replace earlier values.
Nonfinite floating payloads remain present and are refused only if subsequently
marshaled by the strict dynamic descriptor. Malformed container tails are errors.

[Source](/reference-sources/Text.Json.Legacy.cinch) · line 66

<a id="value-6a736f6e2d616e792d746f2d6c6567616379"></a>

## `json-any-to-legacy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-any-to-legacy</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Dynamic</span>.<span class="tok-constructor">JsonAny</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Value</span>.<span class="tok-constructor">JsonError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>))</code></pre>

Explicitly project dynamic values into the established Json shape with named representability refusals.
Integer precision and floating payload bits are retained. Exact fractional
spelling becomes a finite binary64 value; typed null collections become JNull.

[Source](/reference-sources/Text.Json.Legacy.cinch) · line 158

<a id="value-6a736f6e2d6c65676163792d74797065"></a>

## `json-legacy-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-legacy-type</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">codec</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonCodec</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Unit</span> -&gt; <span class="tok-variable">a</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Codec</span>.<span class="tok-constructor">JsonType</span> <span class="tok-variable">a</span>)))))</code></pre>

Adapt an existing Json-shaped codec using explicit labels, defaults and application omission classifiers.
This is an explicit capability adapter rather than a global change to either
codec family. Refused decoding retains the supplied prior application value.

[Source](/reference-sources/Text.Json.Legacy.cinch) · line 193

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Json/Legacy.cinch`. Source SHA-256: `c486fb1b879d656fb5a61b1e5d2b098563e269a3d53a273cc1b9f50198fba826`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
