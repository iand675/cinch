# Text.Binary.Layout

Typed scalar, floating-point, complex and aggregate binary layouts with checked buffers, exact sizes and packed array kernels.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Binary.Layout`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Binary.Layout.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.Complex](Data.Complex.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Complex](Data.Complex.md), [Text.Binary](Text.Binary.md), [Text.Binary.Layout](Text.Binary.Layout.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Typed fixed-size binary layouts with checked offsets, native wire order and exact float bits.
Layouts describe concrete scalars, arrays, byte fields, padding and products;
pure inverse views adapt products to application record types. Fields have no
implicit alignment. Encoding validates values before allocation and commits
caller-buffer output only after a complete private encoding succeeds.
Decoding returns fresh byte fields and a consumed count, leaving trailing input.

</details>

# Types and constructors

<a id="type-42696e6172794c61796f7574"></a>

## `BinaryLayout`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BinaryLayout</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A typed fixed-size wire descriptor; its constructor and trusted buffer callbacks are private.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 97

<a id="type-42696e6172794c61796f75744572726f72"></a>

## `BinaryLayoutError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BinaryLayoutError</span></code></pre>

Fixed-layout, range and variable-integer failures, with exact progress where available.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 65

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BinaryShortInput</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BinaryShortDestination</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BinaryInvalidOffset</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BinaryLengthMismatch</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BinaryIncompleteVarint</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BinaryVarintOverflow</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span>)</code></pre>

<a id="type-42696e6172794f72646572"></a>

## `BinaryOrder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BinaryOrder</span></code></pre>

Wire significance, including an explicit target-native order that retains its own name.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 37

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BinaryLittleEndian</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BinaryBigEndian</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BinaryNativeEndian</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span></code></pre>

# Values

<a id="value-62696e6172792d617070656e64"></a>

## `binary-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-append</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

Append a complete private encoding to a copied prefix, preserving both caller values.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 172

<a id="value-62696e6172792d617070656e642d753136"></a>

## `binary-append-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-append-u16</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Bytes</span>)))</code></pre>

Append an unsigned sixteen-bit field without changing the caller's prefix.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 716

<a id="value-62696e6172792d617070656e642d753332"></a>

## `binary-append-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-append-u32</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bytes</span>)))</code></pre>

Append an unsigned thirty-two-bit field without changing the caller's prefix.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 720

<a id="value-62696e6172792d617070656e642d753634"></a>

## `binary-append-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-append-u64</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bytes</span>)))</code></pre>

Append an unsigned sixty-four-bit field without changing the caller's prefix.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 724

<a id="value-62696e6172792d617070656e642d75766172696e74"></a>

## `binary-append-uvarint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-append-uvarint</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append a complete unsigned varint to a copied prefix.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 764

<a id="value-62696e6172792d617070656e642d766172696e74"></a>

## `binary-append-varint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-append-varint</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append a complete signed varint to a copied prefix.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 768

<a id="value-62696e6172792d6172726179"></a>

## `binary-array`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-array</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))))</code></pre>

A fixed-count homogeneous array represented by a list; nested arrays and records are supported.
Encoding rejects length mismatches; decoding obtains its element count from the descriptor.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 652

<a id="value-62696e6172792d626f6f6c"></a>

## `binary-bool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-bool</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-constructor">Bool</span>)</code></pre>

Encode False as zero and True as one; every nonzero decoded byte is True.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 265

<a id="value-62696e6172792d6279746573"></a>

## `binary-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-bytes</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-constructor">Bytes</span>))</code></pre>

An exact-size byte field. Decoded values and encoded output never retain a mutable input alias.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 372

<a id="value-62696e6172792d636f6d706c65782d76616c7565"></a>

## `binary-complex-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-complex-value</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Complex</span>.<span class="tok-constructor">Complex</span>)</code></pre>

The standard binary64 complex value, encoded as real then imaginary components in sixteen bytes.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 679

<a id="value-62696e6172792d636f6d706c6578313238"></a>

## `binary-complex128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-complex128</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> (<span class="tok-constructor">F64</span>, <span class="tok-constructor">F64</span>))</code></pre>

A complex binary128 field represented by two binary64 components, real part first.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 667

<a id="value-62696e6172792d636f6d706c65783634"></a>

## `binary-complex64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-complex64</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> (<span class="tok-constructor">F32</span>, <span class="tok-constructor">F32</span>))</code></pre>

A complex binary64 field represented by two binary32 components, real part first.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 663

<a id="value-62696e6172792d6465636f6465"></a>

## `binary-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-decode</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span> (<span class="tok-variable">a</span>, <span class="tok-constructor">Nat</span>)))))</code></pre>

Decode one complete value and return its consumed byte count; trailing bytes remain available.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 159

<a id="value-62696e6172792d6465636f64652d6174"></a>

## `binary-decode-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-decode-at</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span> (<span class="tok-variable">a</span>, <span class="tok-constructor">Nat</span>))))))</code></pre>

Decode at a checked zero-based offset without changing the input or retaining byte-field aliases.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 163

<a id="value-62696e6172792d6465636f64652d75766172696e74"></a>

## `binary-decode-uvarint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-decode-uvarint</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span> (<span class="tok-constructor">U64</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Decode an unsigned varint with consumed counts, distinguishing incomplete prefixes and overflow.
Complete first-eight-byte quanta reuse the existing SWAR payload compactor.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 793

<a id="value-62696e6172792d6465636f64652d766172696e74"></a>

## `binary-decode-varint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-decode-varint</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span> (<span class="tok-constructor">I64</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Decode a signed varint with the same checked prefix counts and full-width inverse zigzag mapping.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 805

<a id="value-62696e6172792d656e636f6465"></a>

## `binary-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-encode</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Encode a validated value into one fresh complete buffer, returning no partial bytes on error.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 137

<a id="value-62696e6172792d656e636f64652d696e746f"></a>

## `binary-encode-into`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-encode-into</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span> <span class="tok-constructor">Nat</span>))))))</code></pre>

Encode into an offset only after both complete capacity and value validation succeed.
A private encoding makes nested validation failures and source/destination overlap atomic.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 148

<a id="value-62696e6172792d656e636f64652d75766172696e74"></a>

## `binary-encode-uvarint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-encode-uvarint</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode a complete unsigned integer with its shortest base-128 representation.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 740

<a id="value-62696e6172792d656e636f64652d766172696e74"></a>

## `binary-encode-varint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-encode-varint</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode a signed integer by zigzag mapping all I64 values before base-128 encoding.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 744

<a id="value-62696e6172792d663332"></a>

## `binary-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-f32</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-constructor">F32</span>)</code></pre>

One bit-preserving binary32 floating-point field.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 329

<a id="value-62696e6172792d6633322d62697473"></a>

## `binary-f32-bits`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-f32-bits</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Observe all binary32 bits without arithmetic or NaN canonicalization.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 301

<a id="value-62696e6172792d6633322d66726f6d2d62697473"></a>

## `binary-f32-from-bits`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-f32-from-bits</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

Construct binary32 from its complete bit pattern, including signed zeros and NaN payloads.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 308

<a id="value-62696e6172792d663634"></a>

## `binary-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-f64</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-constructor">F64</span>)</code></pre>

One bit-preserving binary64 floating-point field.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 333

<a id="value-62696e6172792d6636342d62697473"></a>

## `binary-f64-bits`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-f64-bits</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Observe all binary64 bits without arithmetic or NaN canonicalization.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 315

<a id="value-62696e6172792d6636342d66726f6d2d62697473"></a>

## `binary-f64-from-bits`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-f64-from-bits</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

Construct binary64 from its complete bit pattern, including signed zeros and NaN payloads.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 322

<a id="value-62696e6172792d693136"></a>

## `binary-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-i16</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-constructor">I16</span>)</code></pre>

One signed two's-complement sixteen-bit field.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 281

<a id="value-62696e6172792d693332"></a>

## `binary-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-i32</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-constructor">I32</span>)</code></pre>

One signed two's-complement thirty-two-bit field.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 289

<a id="value-62696e6172792d693634"></a>

## `binary-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-i64</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-constructor">I64</span>)</code></pre>

One signed two's-complement sixty-four-bit field.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 297

<a id="value-62696e6172792d6938"></a>

## `binary-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-i8</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-constructor">I8</span>)</code></pre>

One signed two's-complement eight-bit field.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 273

<a id="value-62696e6172792d6c61796f75742d6572726f722d6d657373616765"></a>

## `binary-layout-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-layout-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render complete required/available counts, invalid offsets or variable-integer progress.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 80

<a id="value-62696e6172792d6d6170"></a>

## `binary-map`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-map</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; ((<span class="tok-variable">b</span> -&gt; <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-variable">b</span>))))</code></pre>

Adapt a fixed wire view to an application type with pure inverse conversions.
Round trips require the supplied conversions to be inverses on the represented domain.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 396

<a id="value-62696e6172792d6d61782d766172696e743136"></a>

## `binary-max-varint16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-max-varint16</span> :: <span class="tok-constructor">Nat</span></code></pre>

Maximum bytes for a base-128 integer whose value fits sixteen bits.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 728

<a id="value-62696e6172792d6d61782d766172696e743332"></a>

## `binary-max-varint32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-max-varint32</span> :: <span class="tok-constructor">Nat</span></code></pre>

Maximum bytes for a base-128 integer whose value fits thirty-two bits.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 732

<a id="value-62696e6172792d6d61782d766172696e743634"></a>

## `binary-max-varint64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-max-varint64</span> :: <span class="tok-constructor">Nat</span></code></pre>

Maximum bytes for a base-128 integer whose value fits sixty-four bits.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 736

<a id="value-62696e6172792d6f726465722d6e616d65"></a>

## `binary-order-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-order-name</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Stable names for the three wire-order values.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 46

<a id="value-62696e6172792d70616464696e67"></a>

## `binary-padding`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-padding</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-constructor">Unit</span>))</code></pre>

Explicit zero-filled padding; decoding skips its bytes without requiring them to be zero.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 354

<a id="value-62696e6172792d70616972"></a>

## `binary-pair`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-pair</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-variable">b</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>))))</code></pre>

Sequence two checked fields without adding alignment or implicit padding.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 382

<a id="value-62696e6172792d7075742d753136"></a>

## `binary-put-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-put-u16</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span> <span class="tok-constructor">Unit</span>)))))</code></pre>

Write an unsigned sixteen-bit field after checking its entire destination interval.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 695

<a id="value-62696e6172792d7075742d753332"></a>

## `binary-put-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-put-u32</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span> <span class="tok-constructor">Unit</span>)))))</code></pre>

Write an unsigned thirty-two-bit field after checking its entire destination interval.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 699

<a id="value-62696e6172792d7075742d753634"></a>

## `binary-put-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-put-u64</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span> <span class="tok-constructor">Unit</span>)))))</code></pre>

Write an unsigned sixty-four-bit field after checking its entire destination interval.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 703

<a id="value-62696e6172792d7075742d75766172696e74"></a>

## `binary-put-uvarint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-put-uvarint</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span> <span class="tok-constructor">Nat</span>))))</code></pre>

Write a complete unsigned varint at an offset; short destinations remain unchanged.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 756

<a id="value-62696e6172792d7075742d766172696e74"></a>

## `binary-put-varint`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-put-varint</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span> <span class="tok-constructor">Nat</span>))))</code></pre>

Write a complete signed varint at an offset; short destinations remain unchanged.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 760

<a id="value-62696e6172792d726561642d753136"></a>

## `binary-read-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-read-u16</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span> <span class="tok-constructor">U16</span>))))</code></pre>

Read an unsigned sixteen-bit field at a checked offset in explicit or native wire order.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 683

<a id="value-62696e6172792d726561642d753332"></a>

## `binary-read-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-read-u32</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span> <span class="tok-constructor">U32</span>))))</code></pre>

Read an unsigned thirty-two-bit field at a checked offset.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 687

<a id="value-62696e6172792d726561642d753634"></a>

## `binary-read-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-read-u64</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryOrder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayoutError</span> <span class="tok-constructor">U64</span>))))</code></pre>

Read an unsigned sixty-four-bit field at a checked offset.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 691

<a id="value-62696e6172792d73697a65"></a>

## `binary-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-size</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Exact wire bytes described by a layout, independent of its values or target alignment.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 102

<a id="value-62696e6172792d753136"></a>

## `binary-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-u16</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-constructor">U16</span>)</code></pre>

One unsigned sixteen-bit field.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 277

<a id="value-62696e6172792d753332"></a>

## `binary-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-u32</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-constructor">U32</span>)</code></pre>

One unsigned thirty-two-bit field.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 285

<a id="value-62696e6172792d753634"></a>

## `binary-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-u64</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-constructor">U64</span>)</code></pre>

One unsigned sixty-four-bit field.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 293

<a id="value-62696e6172792d7538"></a>

## `binary-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-u8</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-constructor">U8</span>)</code></pre>

One unsigned eight-bit field.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 269

<a id="value-62696e6172792d756e6974"></a>

## `binary-unit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary-unit</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Binary</span>.<span class="tok-constructor">Layout</span>.<span class="tok-constructor">BinaryLayout</span> <span class="tok-constructor">Unit</span>)</code></pre>

Empty fixed layout; encoding emits no bytes and decoding consumes no input.

[Source](/reference-sources/Text.Binary.Layout.cinch) · line 337

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Binary/Layout.cinch`. Source SHA-256: `2bf98a83aaed9aeb72ac3e69d285d15aa28ff65b6543236b659bb775beb8f1d2`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
