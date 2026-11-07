# Text.Codec

Encoding/decoding signatures and codec helpers with explicit failure results.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Text.Codec`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Codec.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Json](Data.Json.md), [Data.Result](Data.Result.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Codec: conversion to and from `Bytes` and `Json`.

Module scheme (design/base-library.md): `Text.Codec` aligns with the
A6 `CodecGenerator.derive` vocabulary (modules specification
section 11): the generated codec pair is `encodeT :: T -> Bytes`
and `decodeT :: Bytes -> Option T`, in a fixed little-endian column
format. The class states that pair once so the generator's output
and hand-written codecs share one vocabulary.

Derived codec instances where expressible: exactly the A6 column
encoders' covered set (decision D65), `I64` as 8 bytes
little-endian (Data.Machine) and `Bool` as one byte (Data.Bool).
REAL needs float bit access and TEXT a length-prefixed copy (D65);
those instances land with their generators. The class law shape is
the round trip: `decode (encode x) == Some x` (asserted over sample
values in compiler/tests/prelude_split.rs).

`JsonCodec` is the JSON-value column beside it (P31): the same law
shape over the `Json` family, with a named decode error instead of
`Option` (a JSON column carries a shape to report). Its instances
live with their types (Data.Machine's `JsonCodec I64`, Data.Bool's
`JsonCodec Bool`, and `Text.Json`'s identity column), and the
error vocabulary sits with the class, like `Text.Parse`'s
`ParseError` sits with its parsers.

</details>

# Types and constructors

<a id="type-4a736f6e4465636f64654572726f72"></a>

## `JsonDecodeError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JsonDecodeError</span></code></pre>

The named decode errors: a wrong shape (the expected column kind)
and a value outside the column's range.

[Source](/reference-sources/Text.Codec.cinch) · line 40

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TypeMismatch</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">JsonDecodeError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">OutOfRange</span> :: (<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">JsonDecodeError</span>)</code></pre>

# Signatures and methods

<a id="class-436f646563"></a>

## `Codec`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Codec</span> <span class="tok-variable">a</span></code></pre>

[Source](/reference-sources/Text.Codec.cinch)

<a id="method-436f6465632e656e636f6465"></a>

## `Codec.encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">encode</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Codec</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>



<a id="method-436f6465632e6465636f6465"></a>

## `Codec.decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">decode</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Codec</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>



<a id="class-4a736f6e436f646563"></a>

## `JsonCodec`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">JsonCodec</span> <span class="tok-variable">a</span></code></pre>

[Source](/reference-sources/Text.Codec.cinch)

<a id="method-4a736f6e436f6465632e656e636f64652d6a736f6e"></a>

## `JsonCodec.encode-json`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">encode-json</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">JsonCodec</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Json</span>)</code></pre>



<a id="method-4a736f6e436f6465632e6465636f64652d6a736f6e"></a>

## `JsonCodec.decode-json`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">decode-json</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">JsonCodec</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Json</span> -&gt; <span class="tok-constructor">Result</span> <span class="tok-constructor">JsonDecodeError</span> <span class="tok-variable">a</span>)</code></pre>



# Values

<a id="value-6a736f6e2d6572726f722d6d657373616765"></a>

## `json-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">json-error-message</span> :: (<span class="tok-constructor">JsonDecodeError</span> -&gt; <span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Codec.cinch)

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Codec.cinch`. Source SHA-256: `91c7d33762b281f44b718ea2a456032086ab1e8f2af32054fae3ca414e1afb29`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
