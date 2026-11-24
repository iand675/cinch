# Text.Utf16

UTF-16 scalar and surrogate operations, code-unit sequences and explicit-order byte codecs with SWAR ASCII blocks.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Utf16`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Utf16.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Binary](Text.Binary.md), [Text.Utf16](Text.Utf16.md).

<details><summary>Behaviour and boundary notes from the source</summary>

UTF-16 code units, surrogate pairs and explicit-byte-order text conversion.
Invalid scalar inputs and unpaired surrogates become U+FFFD. Byte decoders
reject an odd final byte; a byte-order mark is ordinary text, never inferred
or removed. Buffer codecs process four ASCII units at once using portable
SWAR widening and narrowing, with bounded loads and exact scalar tails.

</details>

# Types and constructors

<a id="type-55746631364572726f72"></a>

## `Utf16Error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Utf16Error</span></code></pre>

An incomplete byte sequence, carrying the offset of its unpaired final byte.

[Source](/reference-sources/Text.Utf16.cinch) · line 24

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">OddUtf16ByteCount</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Utf16</span>.<span class="tok-constructor">Utf16Error</span>)</code></pre>

# Values

<a id="value-75746631362d617070656e642d72756e65"></a>

## `utf16-append-rune`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf16-append-rune</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">U16</span>) -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">U16</span>)))</code></pre>

Append one scalar to an immutable code-unit list, preserving its existing units.
This copies the list spine in linear time; byte codecs avoid list-spine construction.

[Source](/reference-sources/Text.Utf16.cinch) · line 69

<a id="value-75746631362d6465636f6465"></a>

## `utf16-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf16-decode</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">U16</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Char</span>))</code></pre>

Decode code units in order, replacing each unpaired surrogate independently.

[Source](/reference-sources/Text.Utf16.cinch) · line 79

<a id="value-75746631362d6465636f64652d6279746573"></a>

## `utf16-decode-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf16-decode-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">ByteOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Utf16</span>.<span class="tok-constructor">Utf16Error</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Decode explicit-order UTF-16 bytes. An odd byte count is an error before conversion.
Unpaired surrogates become U+FFFD; byte-order marks remain in the resulting text.

[Source](/reference-sources/Text.Utf16.cinch) · line 174

<a id="value-75746631362d6465636f64652d72756e65"></a>

## `utf16-decode-rune`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf16-decode-rune</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

Decode a high/low surrogate pair, returning U+FFFD if either position is invalid.

[Source](/reference-sources/Text.Utf16.cinch) · line 48

<a id="value-75746631362d656e636f6465"></a>

## `utf16-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf16-encode</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">I64</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">U16</span>))</code></pre>

Encode numeric scalar values in order; invalid scalar values each produce one replacement unit.

[Source](/reference-sources/Text.Utf16.cinch) · line 62

<a id="value-75746631362d656e636f64652d6279746573"></a>

## `utf16-encode-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf16-encode-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">ByteOrder</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Encode valid UTF-8 text to explicit-order UTF-16 bytes, with no added byte-order mark.

[Source](/reference-sources/Text.Utf16.cinch) · line 141

<a id="value-75746631362d656e636f64652d72756e65"></a>

## `utf16-encode-rune`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf16-encode-rune</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">U16</span>, <span class="tok-constructor">U16</span>))</code></pre>

Encode a supplementary scalar as a surrogate pair. All other inputs return two U+FFFD units.

[Source](/reference-sources/Text.Utf16.cinch) · line 39

<a id="value-75746631362d69732d737572726f67617465"></a>

## `utf16-is-surrogate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf16-is-surrogate</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether a numeric code point lies in the high or low surrogate range.

[Source](/reference-sources/Text.Utf16.cinch) · line 29

<a id="value-75746631362d72756e652d6c656e677468"></a>

## `utf16-rune-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">utf16-rune-length</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>))</code></pre>

One or two code units for a scalar, or None for a negative, surrogate or out-of-range input.

[Source](/reference-sources/Text.Utf16.cinch) · line 33

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Utf16.cinch`. Source SHA-256: `e58c09586928b41581c0bb9c79d454a896c8f5ed2a5e8ff6b9b4676c6623549e`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
