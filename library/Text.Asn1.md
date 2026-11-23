# Text.Asn1

ASN.1 value trees with a strict DER decoder and canonical DER encoder, including explicit and implicit tags and times.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Asn1`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Asn1.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Text.Asn1](Text.Asn1.md).

<details><summary>Behaviour and boundary notes from the source</summary>

ASN.1 values in DER: a typed value tree with a strict decoder and a canonical encoder.
Values cover booleans, arbitrary-precision integers and enumerations, bit and
octet strings, null, object identifiers, the common character string types,
UTC and generalized times, sequences, sets and raw values of any class and tag
(for explicit and implicit tagging). Decoding enforces DER: definite minimal
lengths, minimal two's-complement integers, booleans of 0x00 or 0xFF, zero
padding bits in bit strings, minimal object identifier arcs and no trailing
data. Encoding produces DER, sorting the members of a set by their encodings.

</details>

# Types and constructors

<a id="type-41736e31"></a>

## `Asn1`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Asn1</span></code></pre>

An ASN.1 value.

[Source](/reference-sources/Text.Asn1.cinch) · line 68

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Asn1Boolean</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Asn1Integer</span> :: (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Asn1Enumerated</span> :: (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Asn1BitString</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Asn1OctetString</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Asn1Null</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Asn1Oid</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Asn1String</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Asn1UtcTime</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Asn1GeneralizedTime</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Asn1Sequence</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Asn1Set</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Asn1Member</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Asn1End</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Asn1Raw</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>))))</code></pre>

<a id="type-41736e314572726f72"></a>

## `Asn1Error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Asn1Error</span></code></pre>

A DER violation; offsets locate the element.

[Source](/reference-sources/Text.Asn1.cinch) · line 124

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Asn1Truncated</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Asn1BadLength</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Asn1BadContent</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Asn1TrailingData</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1Error</span>)</code></pre>

# Values

<a id="value-61736e312d6465636f6465"></a>

## `asn1-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">asn1-decode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1Error</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>))</code></pre>

Decode exactly one DER value occupying the whole input.

[Source](/reference-sources/Text.Asn1.cinch) · line 166

<a id="value-61736e312d656e636f6465"></a>

## `asn1-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">asn1-encode</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode a value in DER.

[Source](/reference-sources/Text.Asn1.cinch) · line 333

<a id="value-61736e312d6572726f722d6d657373616765"></a>

## `asn1-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">asn1-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1Error</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an ASN.1 error.

[Source](/reference-sources/Text.Asn1.cinch) · line 135

<a id="value-61736e312d6578706c69636974"></a>

## `asn1-explicit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">asn1-explicit</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>))</code></pre>

Wrap a value in an explicit context-specific tag.

[Source](/reference-sources/Text.Asn1.cinch) · line 143

<a id="value-61736e312d696d706c69636974"></a>

## `asn1-implicit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">asn1-implicit</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>))</code></pre>

Replace a value's identifier with an implicit context-specific tag, keeping its content and form.

[Source](/reference-sources/Text.Asn1.cinch) · line 147

<a id="value-61736e312d6d656d62657273"></a>

## `asn1-members`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">asn1-members</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>))</code></pre>

Members of a SEQUENCE or SET in order; other values have none.

[Source](/reference-sources/Text.Asn1.cinch) · line 109

<a id="value-61736e312d73657175656e6365"></a>

## `asn1-sequence`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">asn1-sequence</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>)</code></pre>

A SEQUENCE of the given members.

[Source](/reference-sources/Text.Asn1.cinch) · line 101

<a id="value-61736e312d736574"></a>

## `asn1-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">asn1-set</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>) -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>)</code></pre>

A SET of the given members.

[Source](/reference-sources/Text.Asn1.cinch) · line 105

<a id="value-61736e312d74696d652d7365636f6e6473"></a>

## `asn1-time-seconds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">asn1-time-seconds</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Int</span>))</code></pre>

Unix seconds of a UTCTime (years 50-99 are 1950-1999) or GeneralizedTime in the Z form, ignoring
fractional seconds; None for any other value or text.

[Source](/reference-sources/Text.Asn1.cinch) · line 469

<a id="value-636c6173732d6170706c69636174696f6e"></a>

## `class-application`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">class-application</span> :: <span class="tok-constructor">Nat</span></code></pre>

The application class.

[Source](/reference-sources/Text.Asn1.cinch) · line 56

<a id="value-636c6173732d636f6e74657874"></a>

## `class-context`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">class-context</span> :: <span class="tok-constructor">Nat</span></code></pre>

The context-specific class, used for tagged fields.

[Source](/reference-sources/Text.Asn1.cinch) · line 60

<a id="value-636c6173732d70726976617465"></a>

## `class-private`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">class-private</span> :: <span class="tok-constructor">Nat</span></code></pre>

The private class.

[Source](/reference-sources/Text.Asn1.cinch) · line 64

<a id="value-636c6173732d756e6976657273616c"></a>

## `class-universal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">class-universal</span> :: <span class="tok-constructor">Nat</span></code></pre>

The universal class.

[Source](/reference-sources/Text.Asn1.cinch) · line 52

<a id="value-7461672d626d702d737472696e67"></a>

## `tag-bmp-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tag-bmp-string</span> :: <span class="tok-constructor">Nat</span></code></pre>

Universal tag of BMPString.

[Source](/reference-sources/Text.Asn1.cinch) · line 48

<a id="value-7461672d6961352d737472696e67"></a>

## `tag-ia5-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tag-ia5-string</span> :: <span class="tok-constructor">Nat</span></code></pre>

Universal tag of IA5String.

[Source](/reference-sources/Text.Asn1.cinch) · line 44

<a id="value-7461672d6e756d657269632d737472696e67"></a>

## `tag-numeric-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tag-numeric-string</span> :: <span class="tok-constructor">Nat</span></code></pre>

Universal tag of NumericString.

[Source](/reference-sources/Text.Asn1.cinch) · line 32

<a id="value-7461672d7072696e7461626c652d737472696e67"></a>

## `tag-printable-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tag-printable-string</span> :: <span class="tok-constructor">Nat</span></code></pre>

Universal tag of PrintableString.

[Source](/reference-sources/Text.Asn1.cinch) · line 36

<a id="value-7461672d7436312d737472696e67"></a>

## `tag-t61-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tag-t61-string</span> :: <span class="tok-constructor">Nat</span></code></pre>

Universal tag of T61String (TeletexString).

[Source](/reference-sources/Text.Asn1.cinch) · line 40

<a id="value-7461672d757466382d737472696e67"></a>

## `tag-utf8-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tag-utf8-string</span> :: <span class="tok-constructor">Nat</span></code></pre>

Universal tag of UTF8String.

[Source](/reference-sources/Text.Asn1.cinch) · line 28

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Asn1.cinch`. Source SHA-256: `ce041c4c148382136cc9669fe4558c3dcd23b93ab894f85fb2aa9ecdcf2c2e31`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
