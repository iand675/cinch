# Text.Hex

SIMD hexadecimal text/byte codecs, prefix-preserving decoding, append operations and checked destination buffers.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Hex`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Hex.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Hex](Text.Hex.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Hexadecimal conversion between byte buffers and text.
Encoding emits two lowercase ASCII characters per byte. Decoding accepts either
case, without whitespace or prefixes. Invalid-byte offsets count UTF-8 bytes;
an invalid final byte takes precedence over an odd-length error. Whole-text
decoding rejects the input on error; prefix and append APIs retain completed
bytes alongside the error. Complete blocks use inline LLVM SIMD nibble lanes;
short tails and precise error reporting use the scalar path.

</details>

# Types and constructors

<a id="type-4865784572726f72"></a>

## `HexError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HexError</span></code></pre>

A hexadecimal input or destination is invalid. Prefix APIs return completed output alongside this error.

[Source](/reference-sources/Text.Hex.cinch) · line 24

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">OddHexLength</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidHexByte</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HexShortDestination</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexError</span>))</code></pre>

# Values

<a id="value-6865782d6465636f6465"></a>

## `hex-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-decode</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Decode text atomically: malformed input returns only an error, without a partial buffer.

[Source](/reference-sources/Text.Hex.cinch) · line 86

<a id="value-6865782d6465636f64652d617070656e64"></a>

## `hex-decode-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-decode-append</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexError</span>))))</code></pre>

Append decoded bytes to a copied prefix, retaining completed pairs even if later input is invalid.
Error offsets refer to the encoded input, independently of the prefix length.

[Source](/reference-sources/Text.Hex.cinch) · line 98

<a id="value-6865782d6465636f64652d696e746f"></a>

## `hex-decode-into`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-decode-into</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexError</span>)))))</code></pre>

Decode into a destination and return completed bytes plus any decoding error.
A short range returns zero and HexShortDestination before mutation; malformed input
retains its valid decoded prefix. The source is snapshotted so overlap is safe.

[Source](/reference-sources/Text.Hex.cinch) · line 111

<a id="value-6865782d6465636f64652d707265666978"></a>

## `hex-decode-prefix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-decode-prefix</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexError</span>)))</code></pre>

Decode all complete valid pairs from arbitrary encoded bytes, preserving a prefix on error.
Invalid byte offsets refer to the original input, including invalid non-UTF-8 bytes.

[Source](/reference-sources/Text.Hex.cinch) · line 93

<a id="value-6865782d6465636f6465642d6c656e677468"></a>

## `hex-decoded-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-decoded-length</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The maximum complete decoded byte count for an encoded length: floor(length / 2). This does not validate the input.

[Source](/reference-sources/Text.Hex.cinch) · line 37

<a id="value-6865782d656e636f6465"></a>

## `hex-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-encode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Encode all bytes as lowercase hexadecimal ASCII. Empty input produces empty text; complete blocks use SIMD and the tail is exact.

[Source](/reference-sources/Text.Hex.cinch) · line 45

<a id="value-6865782d656e636f64652d617070656e64"></a>

## `hex-encode-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-encode-append</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append encoded bytes to a copied prefix. Neither input buffer is mutated or retained by alias.

[Source](/reference-sources/Text.Hex.cinch) · line 60

<a id="value-6865782d656e636f64652d6279746573"></a>

## `hex-encode-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-encode-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode directly to a fresh lowercase ASCII buffer, without a text conversion.

[Source](/reference-sources/Text.Hex.cinch) · line 54

<a id="value-6865782d656e636f64652d696e746f"></a>

## `hex-encode-into`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-encode-into</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexError</span> <span class="tok-constructor">Nat</span>))))</code></pre>

Encode into a destination starting at an offset and return the written byte count.
The entire destination range is checked before writing. Source/destination overlap is safe.

[Source](/reference-sources/Text.Hex.cinch) · line 69

<a id="value-6865782d656e636f6465642d6c656e677468"></a>

## `hex-encoded-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-encoded-length</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The exact encoded character and byte count for a buffer of the given length: twice its length.

[Source](/reference-sources/Text.Hex.cinch) · line 33

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Hex.cinch`. Source SHA-256: `2753b6e34d7e386ddd0328079925f036e66b362da0f24407ce95cdb19e90ecfb`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
