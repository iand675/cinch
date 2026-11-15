# Text.Hex

Lowercase hexadecimal encoding and case-insensitive decoding with named errors and byte offsets.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Hex`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Hex.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Hex](Text.Hex.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Hexadecimal conversion between byte buffers and text.
Encoding emits two lowercase ASCII characters per byte. Decoding accepts either
case, without whitespace or prefixes. Invalid-byte offsets count UTF-8 bytes;
an invalid final byte takes precedence over an odd-length error. No partial
buffer is returned on error. Complete blocks use inline LLVM SIMD nibble lanes;
short tails and precise error reporting use the scalar path.

</details>

# Types and constructors

<a id="type-4865784572726f72"></a>

## `HexError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HexError</span></code></pre>

The entire hexadecimal input could not be decoded; no partial buffer accompanies an error.

[Source](/reference-sources/Text.Hex.cinch) · line 22

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">OddHexLength</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidHexByte</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexError</span>))</code></pre>

# Values

<a id="value-6865782d6465636f6465"></a>

## `hex-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-decode</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Hex</span>.<span class="tok-constructor">HexError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Decode either-case hexadecimal text. Whitespace and prefixes are invalid; errors identify the first invalid byte or an unmatched final digit.

[Source](/reference-sources/Text.Hex.cinch) · line 74

<a id="value-6865782d6465636f6465642d6c656e677468"></a>

## `hex-decoded-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-decoded-length</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The maximum complete decoded byte count for an encoded length: floor(length / 2). This does not validate the input.

[Source](/reference-sources/Text.Hex.cinch) · line 33

<a id="value-6865782d656e636f6465"></a>

## `hex-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-encode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Encode all bytes as lowercase hexadecimal ASCII. Empty input produces empty text; complete blocks use SIMD and the tail is exact.

[Source](/reference-sources/Text.Hex.cinch) · line 41

<a id="value-6865782d656e636f6465642d6c656e677468"></a>

## `hex-encoded-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-encoded-length</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The exact encoded character and byte count for a buffer of the given length: twice its length.

[Source](/reference-sources/Text.Hex.cinch) · line 29

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Hex.cinch`. Source SHA-256: `6a9bd7d383931fadbf35503562d57a26c92fd52f2d8f3961b6bdbdc29622e522`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
