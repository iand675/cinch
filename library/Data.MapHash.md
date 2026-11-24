# Data.MapHash

Fast keyed 64-bit non-cryptographic hash with persistent streaming state, one-shot byte/string fast paths and MapHashable typed comparable hashing.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.MapHash`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.MapHash.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Data.MapHash](Data.MapHash.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Fast, keyed, non-cryptographic 64-bit hashing for hash tables, byte streams and comparable values.

Each MapSeed carries a 128-bit secret key that selects an independent hash function.
Construct deterministic seeds with maphash-seed-from-u64, maphash-seed-from-u128 or
maphash-seed-from-bytes, or draw unpredictable per-table seeds from operating-system
entropy through Effect.MapHash.maphash-make-seed.

MapHash is an immutable persistent streaming state: retaining any intermediate value
acts as an O(1) snapshot without copying or aliasing mutable buffers. The computed
digest depends only on the seed and the sequence of written bytes, never on how writes
are split across maphash-write, maphash-write-string and maphash-write-byte. One-shot
maphash-bytes and maphash-string produce the exact same digest as streaming writes.

Typed values implement MapHashable using prefix-free canonical encodings so composite
values such as pairs, options and lists avoid boundary collisions, and floating-point
instances normalize negative zero to positive zero. Because MapHash is pure, NaN values
hash deterministically to a canonical quiet-NaN bit pattern.

These functions are intended for hash tables and in-memory indexing; they are not
cryptographic hashes.

</details>

# Types and constructors

<a id="type-4d617048617368"></a>

## `MapHash`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MapHash</span></code></pre>

Immutable streaming hash state holding its seed, 128-bit accumulator, 0..7-byte tail and total length.

[Source](/reference-sources/Data.MapHash.cinch) · line 165

<a id="type-4d617053656564"></a>

## `MapSeed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MapSeed</span></code></pre>

A 128-bit key selecting a specific MapHash function.
Equal seeds produce identical digests for identical inputs; distinct seeds produce independent digests.

[Source](/reference-sources/Data.MapHash.cinch) · line 111

# Signatures and methods

<a id="class-4d61704861736861626c65"></a>

## `MapHashable`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">MapHashable</span> <span class="tok-variable">a</span></code></pre>

Types that admit canonical, prefix-free hashing into a MapHash stream.

[Source](/reference-sources/Data.MapHash.cinch) · line 380

<a id="method-4d61704861736861626c652e6d6170686173682d77726974652d76616c7565"></a>

## `MapHashable.maphash-write-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-write-value</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHashable</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHash</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHash</span>))</code></pre>

Feed a canonical prefix-free representation of a value into a MapHash state.

# Values

<a id="value-6d6170686173682d626c6f636b2d73697a65"></a>

## `maphash-block-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-block-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Preferred streaming write block size in bytes (128 bytes).

[Source](/reference-sources/Data.MapHash.cinch) · line 161

<a id="value-6d6170686173682d6279746573"></a>

## `maphash-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-bytes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapSeed</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

Hash a byte buffer in one pass with the given seed, matching streaming maphash-write and maphash-sum64.

[Source](/reference-sources/Data.MapHash.cinch) · line 314

<a id="value-6d6170686173682d636f6d70617261626c65"></a>

## `maphash-comparable`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-comparable</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">mh</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHashable</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapSeed</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

Compute the seeded 64-bit digest of a comparable value.

[Source](/reference-sources/Data.MapHash.cinch) · line 389

<a id="value-6d6170686173682d6e6577"></a>

## `maphash-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-new</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapSeed</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHash</span>)</code></pre>

Create a new empty MapHash initialized with the given seed.

[Source](/reference-sources/Data.MapHash.cinch) · line 170

<a id="value-6d6170686173682d7265736574"></a>

## `maphash-reset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-reset</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHash</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHash</span>)</code></pre>

Discard all previously written bytes while keeping the same seed.

[Source](/reference-sources/Data.MapHash.cinch) · line 187

<a id="value-6d6170686173682d73656564"></a>

## `maphash-seed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-seed</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHash</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapSeed</span>)</code></pre>

Return the MapSeed governing this MapHash.

[Source](/reference-sources/Data.MapHash.cinch) · line 178

<a id="value-6d6170686173682d736565642d6279746573"></a>

## `maphash-seed-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-seed-bytes</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapSeed</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode a MapSeed as a fresh 16-byte big-endian buffer.

[Source](/reference-sources/Data.MapHash.cinch) · line 135

<a id="value-6d6170686173682d736565642d66726f6d2d6279746573"></a>

## `maphash-seed-from-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-seed-from-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapSeed</span>))</code></pre>

Decode a MapSeed from a 16-byte big-endian buffer, returning None for any other length.

[Source](/reference-sources/Data.MapHash.cinch) · line 144

<a id="value-6d6170686173682d736565642d66726f6d2d75313238"></a>

## `maphash-seed-from-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-seed-from-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapSeed</span>)</code></pre>

Construct a MapSeed directly from a 128-bit unsigned key.

[Source](/reference-sources/Data.MapHash.cinch) · line 131

<a id="value-6d6170686173682d736565642d66726f6d2d753634"></a>

## `maphash-seed-from-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-seed-from-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapSeed</span>)</code></pre>

Construct a 128-bit MapSeed deterministically from a 64-bit word.

[Source](/reference-sources/Data.MapHash.cinch) · line 125

<a id="value-6d6170686173682d7365742d73656564"></a>

## `maphash-set-seed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-set-seed</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHash</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapSeed</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHash</span>))</code></pre>

Replace the seed and discard any previously written bytes.

[Source](/reference-sources/Data.MapHash.cinch) · line 183

<a id="value-6d6170686173682d73697a65"></a>

## `maphash-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Digest width in bytes (8 bytes for a 64-bit digest).

[Source](/reference-sources/Data.MapHash.cinch) · line 157

<a id="value-6d6170686173682d737472696e67"></a>

## `maphash-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapSeed</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

Hash a UTF-8 string in one pass with the given seed, matching maphash-write-string and maphash-sum64.

[Source](/reference-sources/Data.MapHash.cinch) · line 322

<a id="value-6d6170686173682d73756d"></a>

## `maphash-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-sum</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHash</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the 8-byte big-endian digest of h to prefix, returning a fresh buffer.

[Source](/reference-sources/Data.MapHash.cinch) · line 283

<a id="value-6d6170686173682d73756d3634"></a>

## `maphash-sum64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-sum64</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHash</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Return the current 64-bit digest of h without altering h.

[Source](/reference-sources/Data.MapHash.cinch) · line 277

<a id="value-6d6170686173682d7772697465"></a>

## `maphash-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-write</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHash</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHash</span>))</code></pre>

Add all bytes of b to the stream hashed by h, returning the updated MapHash.

[Source](/reference-sources/Data.MapHash.cinch) · line 248

<a id="value-6d6170686173682d77726974652d62797465"></a>

## `maphash-write-byte`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-write-byte</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHash</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHash</span>))</code></pre>

Add one byte to the stream hashed by h, returning the updated MapHash.

[Source](/reference-sources/Data.MapHash.cinch) · line 261

<a id="value-6d6170686173682d77726974652d636f6d70617261626c65"></a>

## `maphash-write-comparable`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-write-comparable</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">mh</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHashable</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHash</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHash</span>))</code></pre>

Feed a comparable value into a MapHash stream.

[Source](/reference-sources/Data.MapHash.cinch) · line 385

<a id="value-6d6170686173682d77726974652d737472696e67"></a>

## `maphash-write-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maphash-write-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHash</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">MapHash</span>.<span class="tok-constructor">MapHash</span>))</code></pre>

Add the UTF-8 bytes of s to the stream hashed by h, returning the updated MapHash.

[Source](/reference-sources/Data.MapHash.cinch) · line 257

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.MapHash.Eq_MapSeed` | `Relation.Eq.Eq Data.MapHash.MapSeed` | Implicit |
| `Data.MapHash.MapHashable_Bool` | `Data.MapHash.MapHashable Bool` | Implicit |
| `Data.MapHash.MapHashable_Bytes` | `Data.MapHash.MapHashable Bytes` | Implicit |
| `Data.MapHash.MapHashable_Char` | `Data.MapHash.MapHashable Char` | Implicit |
| `Data.MapHash.MapHashable_F32` | `Data.MapHash.MapHashable F32` | Implicit |
| `Data.MapHash.MapHashable_F64` | `Data.MapHash.MapHashable F64` | Implicit |
| `Data.MapHash.MapHashable_I128` | `Data.MapHash.MapHashable I128` | Implicit |
| `Data.MapHash.MapHashable_I16` | `Data.MapHash.MapHashable I16` | Implicit |
| `Data.MapHash.MapHashable_I32` | `Data.MapHash.MapHashable I32` | Implicit |
| `Data.MapHash.MapHashable_I64` | `Data.MapHash.MapHashable I64` | Implicit |
| `Data.MapHash.MapHashable_I8` | `Data.MapHash.MapHashable I8` | Implicit |
| `Data.MapHash.MapHashable_Int` | `Data.MapHash.MapHashable Int` | Implicit |
| `Data.MapHash.MapHashable_List` | `Data.MapHash.MapHashable (Data.Core.List a)` | Implicit |
| `Data.MapHash.MapHashable_Nat` | `Data.MapHash.MapHashable Nat` | Implicit |
| `Data.MapHash.MapHashable_Option` | `Data.MapHash.MapHashable (Data.Core.Option a)` | Implicit |
| `Data.MapHash.MapHashable_Pair` | `Data.MapHash.MapHashable (a, b)` | Implicit |
| `Data.MapHash.MapHashable_String` | `Data.MapHash.MapHashable Data.String.String` | Implicit |
| `Data.MapHash.MapHashable_U128` | `Data.MapHash.MapHashable U128` | Implicit |
| `Data.MapHash.MapHashable_U16` | `Data.MapHash.MapHashable U16` | Implicit |
| `Data.MapHash.MapHashable_U32` | `Data.MapHash.MapHashable U32` | Implicit |
| `Data.MapHash.MapHashable_U64` | `Data.MapHash.MapHashable U64` | Implicit |
| `Data.MapHash.MapHashable_U8` | `Data.MapHash.MapHashable U8` | Implicit |

**`Data.MapHash.Eq_MapSeed`**: Equality on 128-bit hash seeds.
**`Data.MapHash.MapHashable_Bool`**: Hash a boolean as one byte (0 for False, 1 for True).
**`Data.MapHash.MapHashable_Bytes`**: Hash a byte buffer framed by its 64-bit byte length so adjacent buffers cannot collide.
**`Data.MapHash.MapHashable_Char`**: Hash a Unicode scalar value by its 32-bit code point.
**`Data.MapHash.MapHashable_F32`**: Hash a 32-bit float, normalizing negative zero to positive zero and NaN to a canonical quiet NaN.
**`Data.MapHash.MapHashable_F64`**: Hash a 64-bit float, normalizing negative zero to positive zero and NaN to a canonical quiet NaN.
**`Data.MapHash.MapHashable_I128`**: Hash a signed 128-bit integer as sixteen little-endian two's-complement bytes.
**`Data.MapHash.MapHashable_I16`**: Hash a signed 16-bit integer as two little-endian two's-complement bytes.
**`Data.MapHash.MapHashable_I32`**: Hash a signed 32-bit integer as four little-endian two's-complement bytes.
**`Data.MapHash.MapHashable_I64`**: Hash a signed 64-bit integer as eight little-endian two's-complement bytes.
**`Data.MapHash.MapHashable_I8`**: Hash a signed 8-bit integer by its two's-complement byte.
**`Data.MapHash.MapHashable_Int`**: Hash an arbitrary-precision signed integer with a sign byte followed by its magnitude limbs.
**`Data.MapHash.MapHashable_List`**: Hash a list in a single tail-recursive pass using prefix-free element tags.
**`Data.MapHash.MapHashable_Nat`**: Hash an arbitrary-precision natural number using a self-delimiting limb encoding.
**`Data.MapHash.MapHashable_Option`**: Hash an optional value with a presence tag byte before any contained value.
**`Data.MapHash.MapHashable_Pair`**: Hash a pair by hashing its first and second components in order.
**`Data.MapHash.MapHashable_String`**: Hash a string framed by its 64-bit UTF-8 byte length so adjacent strings cannot collide.
**`Data.MapHash.MapHashable_U128`**: Hash an unsigned 128-bit integer as sixteen little-endian bytes.
**`Data.MapHash.MapHashable_U16`**: Hash an unsigned 16-bit integer as two little-endian bytes.
**`Data.MapHash.MapHashable_U32`**: Hash an unsigned 32-bit integer as four little-endian bytes.
**`Data.MapHash.MapHashable_U64`**: Hash an unsigned 64-bit integer as eight little-endian bytes.
**`Data.MapHash.MapHashable_U8`**: Hash an unsigned 8-bit integer directly.

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/MapHash.cinch`. Source SHA-256: `62117ab5590660522c3914fbf7b0a878c7d31c068f10dc7325d165bfa12744e4`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
