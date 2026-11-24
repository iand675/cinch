# Crypto.Field25519

Constant-time arithmetic modulo 2^255 - 19 with 128-bit column products, shared by X25519 and Ed25519.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.Field25519`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.Field25519.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Wide](Data.Wide.md).

Non-embedded source closure: [Crypto.Field25519](Crypto.Field25519.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Constant-time arithmetic in the field of integers modulo 2^255 - 19, shared by X25519 and Ed25519.
An element is five 51-bit limbs (little-endian radix 2^51). Each product
column accumulates its five 64x64-bit products in a 128-bit `U128` register,
so a multiplication is five independent wide columns and one carry chain;
additions and subtractions carry immediately, keeping every limb below 2^52
so the next product cannot overflow. Every operation is free of
data-dependent branches and memory access: conditional swaps and selections
use masks, inversion is a fixed addition chain, and comparisons reduce to
the canonical encoding first.

</details>

# Types and constructors

<a id="type-4665"></a>

## `Fe`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Fe</span></code></pre>

Limbs from least to most significant, radix 2^51.

[Source](/reference-sources/Crypto.Field25519.cinch) · line 22

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Fe</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span>)))))</code></pre>

# Values

<a id="value-66652d616464"></a>

## `fe-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fe-add</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span>))</code></pre>

Sum with an immediate carry.

[Source](/reference-sources/Crypto.Field25519.cinch) · line 56

<a id="value-66652d657175616c"></a>

## `fe-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fe-equal</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

One when the two values are equal modulo p, otherwise zero.

[Source](/reference-sources/Crypto.Field25519.cinch) · line 272

<a id="value-66652d66726f6d2d6279746573"></a>

## `fe-from-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fe-from-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span>))</code></pre>

Decode 32 little-endian bytes at an offset, ignoring the top bit. Out-of-range input reads as zero.

[Source](/reference-sources/Crypto.Field25519.cinch) · line 209

<a id="value-66652d66726f6d2d753634"></a>

## `fe-from-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fe-from-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span>)</code></pre>

A small element below 2^51.

[Source](/reference-sources/Crypto.Field25519.cinch) · line 33

<a id="value-66652d696e76657274"></a>

## `fe-invert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fe-invert</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span>)</code></pre>

Multiplicative inverse a^(p - 2) by the fixed addition chain; the inverse of zero is zero.

[Source](/reference-sources/Crypto.Field25519.cinch) · line 181

<a id="value-66652d69732d6e65676174697665"></a>

## `fe-is-negative`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fe-is-negative</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Whether the canonical value is odd (the sign used by point encodings).

[Source](/reference-sources/Crypto.Field25519.cinch) · line 261

<a id="value-66652d69732d7a65726f"></a>

## `fe-is-zero`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fe-is-zero</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

One when the value is zero modulo p, otherwise zero, computed without branching.

[Source](/reference-sources/Crypto.Field25519.cinch) · line 265

<a id="value-66652d6c696d6273"></a>

## `fe-limbs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fe-limbs</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; (<span class="tok-constructor">U64</span>, <span class="tok-constructor">U64</span>, <span class="tok-constructor">U64</span>, <span class="tok-constructor">U64</span>, <span class="tok-constructor">U64</span>))</code></pre>

The five limbs, for constant tables and tests.

[Source](/reference-sources/Crypto.Field25519.cinch) · line 37

<a id="value-66652d6d756c"></a>

## `fe-mul`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fe-mul</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span>))</code></pre>

Product: five 128-bit columns, then one carry chain.

[Source](/reference-sources/Crypto.Field25519.cinch) · line 75

<a id="value-66652d6d756c2d736d616c6c"></a>

## `fe-mul-small`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fe-mul-small</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span>))</code></pre>

Multiply by a constant below 2^13 (such as 121665): one carry chain over 128-bit limbs.

[Source](/reference-sources/Crypto.Field25519.cinch) · line 152

<a id="value-66652d6e6567"></a>

## `fe-neg`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fe-neg</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span>)</code></pre>

Negation.

[Source](/reference-sources/Crypto.Field25519.cinch) · line 71

<a id="value-66652d6f6e65"></a>

## `fe-one`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fe-one</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span></code></pre>

One.

[Source](/reference-sources/Crypto.Field25519.cinch) · line 29

<a id="value-66652d706f773232353233"></a>

## `fe-pow22523`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fe-pow22523</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span>)</code></pre>

a^((p - 5) / 8), used to compute square roots.

[Source](/reference-sources/Crypto.Field25519.cinch) · line 185

<a id="value-66652d73656c656374"></a>

## `fe-select`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fe-select</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span>)))</code></pre>

Choose b when the low bit of choice is set and a otherwise, without branching.

[Source](/reference-sources/Crypto.Field25519.cinch) · line 205

<a id="value-66652d737175617265"></a>

## `fe-square`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fe-square</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span>)</code></pre>

Square using symmetric cross-product doubling (15 wide multiplies instead of 25).

[Source](/reference-sources/Crypto.Field25519.cinch) · line 87

<a id="value-66652d737562"></a>

## `fe-sub`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fe-sub</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span>))</code></pre>

Difference, computed as a + 4p - b so no limb underflows, with an immediate carry.

[Source](/reference-sources/Crypto.Field25519.cinch) · line 62

<a id="value-66652d73776170"></a>

## `fe-swap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fe-swap</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span>, <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span>))))</code></pre>

Swap two elements when the low bit of choice is set, without branching.

[Source](/reference-sources/Crypto.Field25519.cinch) · line 193

<a id="value-66652d746f2d6279746573"></a>

## `fe-to-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fe-to-bytes</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode the canonical value as 32 little-endian bytes in fresh storage.

[Source](/reference-sources/Crypto.Field25519.cinch) · line 250

<a id="value-66652d7a65726f"></a>

## `fe-zero`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fe-zero</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Field25519</span>.<span class="tok-constructor">Fe</span></code></pre>

Zero.

[Source](/reference-sources/Crypto.Field25519.cinch) · line 25

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/Field25519.cinch`. Source SHA-256: `c7b37219c021c73c1bc924e27660d55f8393b565364b26cd430b2255520547c3`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
