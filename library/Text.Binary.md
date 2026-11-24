# Text.Binary

Little- and big-endian integer encoding, checked buffer access, and signed/unsigned varints.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Binary`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Binary.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md).

Non-embedded source closure: [Text.Binary](Text.Binary.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Unsigned fixed-width integers and signed/unsigned variable-length wire encodings.
Reads accept zero-based byte offsets and return named errors for short input.
Varint reads also return the number of consumed bytes, excluding the offset.
Encoders allocate fresh buffers; put operations check the complete destination
range before writing. Signed varints use zigzag encoding across the full I64
range. Fixed-width words use byte swaps; varint prefix detection
and payload compaction use SWAR, with an exact scalar tail.

</details>

# Types and constructors

<a id="type-42696e6172794572726f72"></a>

## `BinaryError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BinaryError</span></code></pre>

A binary input or destination cannot satisfy the requested operation.

[Source](/reference-sources/Text.Binary.cinch) · line 31

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UnexpectedBinaryEnd</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BinaryVarintOverflow</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryError</span></code></pre>

<a id="type-427974654f72646572"></a>

## `ByteOrder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ByteOrder</span></code></pre>

Byte significance for fixed-width wire encodings; independent of host byte order.

[Source](/reference-sources/Text.Binary.cinch) · line 24

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LittleEndian</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">ByteOrder</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BigEndian</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">ByteOrder</span></code></pre>

# Values

<a id="value-62696e6172792d7075742d753136"></a>

## `binary-put-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-put-u16</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">ByteOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryError</span> <span class="tok-constructor">Unit</span>)))))</code></pre>

Write an unsigned 16-bit field into an existing buffer at the zero-based offset. Check all 2 destination bytes before mutation; failure leaves the buffer unchanged.

[Source](/reference-sources/Text.Binary.cinch) · line 103

<a id="value-62696e6172792d7075742d753332"></a>

## `binary-put-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-put-u32</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">ByteOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryError</span> <span class="tok-constructor">Unit</span>)))))</code></pre>

Write an unsigned 32-bit field into an existing buffer at the zero-based offset. Check all 4 destination bytes before mutation; failure leaves the buffer unchanged.

[Source](/reference-sources/Text.Binary.cinch) · line 107

<a id="value-62696e6172792d7075742d753634"></a>

## `binary-put-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-put-u64</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">ByteOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryError</span> <span class="tok-constructor">Unit</span>)))))</code></pre>

Write an unsigned 64-bit field into an existing buffer at the zero-based offset. Check all 8 destination bytes before mutation; failure leaves the buffer unchanged.

[Source](/reference-sources/Text.Binary.cinch) · line 113

<a id="value-62696e6172792d726561642d753136"></a>

## `binary-read-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-read-u16</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">ByteOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryError</span> <span class="tok-constructor">U16</span>))))</code></pre>

Read an unsigned 16-bit field at the zero-based byte offset. Fewer than 2 available bytes returns UnexpectedBinaryEnd; trailing bytes are untouched.

[Source](/reference-sources/Text.Binary.cinch) · line 79

<a id="value-62696e6172792d726561642d753332"></a>

## `binary-read-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-read-u32</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">ByteOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryError</span> <span class="tok-constructor">U32</span>))))</code></pre>

Read an unsigned 32-bit field at the zero-based byte offset. Fewer than 4 available bytes returns UnexpectedBinaryEnd; trailing bytes are untouched.

[Source](/reference-sources/Text.Binary.cinch) · line 85

<a id="value-62696e6172792d726561642d753634"></a>

## `binary-read-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-read-u64</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">ByteOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryError</span> <span class="tok-constructor">U64</span>))))</code></pre>

Read an unsigned 64-bit field at the zero-based byte offset. Fewer than 8 available bytes returns UnexpectedBinaryEnd; trailing bytes are untouched.

[Source](/reference-sources/Text.Binary.cinch) · line 91

<a id="value-62696e6172792d726561642d75766172696e74"></a>

## `binary-read-uvarint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-read-uvarint</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryError</span> (<span class="tok-constructor">U64</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Read a terminated unsigned varint at the byte offset and return (value, bytes-consumed). Nonminimal encodings are accepted; incomplete input and 64-bit overflow are distinct errors.

[Source](/reference-sources/Text.Binary.cinch) · line 146

<a id="value-62696e6172792d726561642d766172696e74"></a>

## `binary-read-varint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-read-varint</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">BinaryError</span> (<span class="tok-constructor">I64</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Read and inverse-zigzag an unsigned varint at the byte offset, returning (signed-value, bytes-consumed). Shares the unsigned decoder's truncation and overflow errors.

[Source](/reference-sources/Text.Binary.cinch) · line 160

<a id="value-62696e6172792d753136"></a>

## `binary-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-u16</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">ByteOrder</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Allocate exactly 2 bytes encoding an unsigned 16-bit value in the selected byte order.

[Source](/reference-sources/Text.Binary.cinch) · line 50

<a id="value-62696e6172792d753332"></a>

## `binary-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-u32</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">ByteOrder</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Allocate exactly 4 bytes encoding an unsigned 32-bit value in the selected byte order.

[Source](/reference-sources/Text.Binary.cinch) · line 54

<a id="value-62696e6172792d753634"></a>

## `binary-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-u64</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">ByteOrder</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Allocate exactly 8 bytes encoding an unsigned 64-bit value in the selected byte order.

[Source](/reference-sources/Text.Binary.cinch) · line 61

<a id="value-62696e6172792d75766172696e74"></a>

## `binary-uvarint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-uvarint</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode a U64 in one to ten little-endian base-128 bytes, with the high bit marking continuation. The emitted representation is minimal.

[Source](/reference-sources/Text.Binary.cinch) · line 125

<a id="value-62696e6172792d766172696e74"></a>

## `binary-varint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-varint</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Zigzag-encode an I64, then encode it as an unsigned varint. Zero maps to 0, -1 to 1, and 1 to 2; both I64 extrema are supported.

[Source](/reference-sources/Text.Binary.cinch) · line 129

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Binary.cinch`. Source SHA-256: `4dc328c991d32ed9442ca60f6a090d3f2a30d6d141565f26d260702b9d7410b6`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
