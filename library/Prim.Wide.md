# Prim.Wide

Leaf inline-LLVM 128-bit SIMD lane primitives and target-guarded hardware AES/CLMUL instructions (AES-NI, ARMv8 AES, PCLMULQDQ, PMULL) with portable fallbacks, re-exported through Data.Wide and shared directly by Data.String.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Prim.Wide`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Prim.Wide.cinch) · [How library modules are loaded](00-index.md)

<details><summary>Behaviour and boundary notes from the source</summary>

Portable 128-bit SIMD lane primitives and hardware AES/CLMUL leaf instructions over U128.
Every lane operation treats a U128 as a packed vector with lane 0 in the
least-significant bits. Target-guarded hardware bodies (AES-NI, ARMv8 AES,
PCLMULQDQ, PMULL) carry explicit `when` guards and portable fallbacks, and
are re-exported through Data.Wide.

</details>

# Types and constructors

# Signatures and methods

<a id="class-41726d416573"></a>

## `ArmAes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">ArmAes</span> </code></pre>

Target-profile evidence that the ARMv8 AES and PMULL instructions may execute.
Only profiles that guarantee the cryptography extension should name this capability.

[Source](/reference-sources/Prim.Wide.cinch) · line 19

# Values

<a id="value-6165732d6465632d6c617374"></a>

## `aes-dec-last`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">aes-dec-last</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Final hardware AES equivalent-inverse decryption round (InvShiftRows, InvSubBytes, AddRoundKey); callers must guard with cpu-has-aes.

[Source](/reference-sources/Prim.Wide.cinch) · line 559

<a id="value-6165732d6465632d726f756e64"></a>

## `aes-dec-round`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">aes-dec-round</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

One hardware AES equivalent-inverse decryption round (InvShiftRows, InvSubBytes, InvMixColumns, AddRoundKey); callers must guard with cpu-has-aes.

[Source](/reference-sources/Prim.Wide.cinch) · line 538

<a id="value-6165732d656e632d6c617374"></a>

## `aes-enc-last`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">aes-enc-last</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Final hardware AES encryption round (SubBytes, ShiftRows, AddRoundKey); callers must guard with cpu-has-aes.

[Source](/reference-sources/Prim.Wide.cinch) · line 517

<a id="value-6165732d656e632d726f756e64"></a>

## `aes-enc-round`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">aes-enc-round</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

One hardware AES encryption round (SubBytes, ShiftRows, MixColumns, AddRoundKey); callers must guard with cpu-has-aes.

[Source](/reference-sources/Prim.Wide.cinch) · line 496

<a id="value-6269742d616e642d75313238"></a>

## `bit-and-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Bitwise AND over two U128 values.

[Source](/reference-sources/Prim.Wide.cinch) · line 24

<a id="value-636c6d756c2d753634"></a>

## `clmul-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">clmul-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

64x64-bit hardware carry-less multiplication into U128; callers must guard with cpu-has-clmul.

[Source](/reference-sources/Prim.Wide.cinch) · line 580

<a id="value-6370752d6861732d616573"></a>

## `cpu-has-aes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cpu-has-aes</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Report non-zero when hardware AES round instructions may execute on the target processor.

[Source](/reference-sources/Prim.Wide.cinch) · line 600

<a id="value-6370752d6861732d636c6d756c"></a>

## `cpu-has-clmul`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cpu-has-clmul</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Report non-zero when hardware carry-less multiplication instructions may execute on the target processor.

[Source](/reference-sources/Prim.Wide.cinch) · line 615

<a id="value-73696d642d62697473656c656374"></a>

## `simd-bitselect`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-bitselect</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>)))</code></pre>

Bitwise select across 128 bits: (mask & when-one) | (~mask & when-zero).

[Source](/reference-sources/Prim.Wide.cinch) · line 31

<a id="value-73696d642d69313678382d61736872"></a>

## `simd-i16x8-ashr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-i16x8-ashr</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise arithmetic right shift (masked modulo 16) on 8 signed I16 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 178

<a id="value-73696d642d69313678382d636d706c74"></a>

## `simd-i16x8-cmplt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-i16x8-cmplt</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise signed less-than mask on 8 I16 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 213

<a id="value-73696d642d69313678382d736d6178"></a>

## `simd-i16x8-smax`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-i16x8-smax</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise signed maximum on 8 I16 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 234

<a id="value-73696d642d69313678382d736d696e"></a>

## `simd-i16x8-smin`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-i16x8-smin</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise signed minimum on 8 I16 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 224

<a id="value-73696d642d69333278342d61736872"></a>

## `simd-i32x4-ashr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-i32x4-ashr</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise arithmetic right shift (masked modulo 32) on 4 signed I32 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 307

<a id="value-73696d642d69333278342d736d6178"></a>

## `simd-i32x4-smax`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-i32x4-smax</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise signed maximum on 4 I32 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 351

<a id="value-73696d642d69333278342d736d696e"></a>

## `simd-i32x4-smin`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-i32x4-smin</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise signed minimum on 4 I32 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 341

<a id="value-73696d642d75313678382d616464"></a>

## `simd-u16x8-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u16x8-add</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise wrapping addition on 8 U16 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 122

<a id="value-73696d642d75313678382d636d706571"></a>

## `simd-u16x8-cmpeq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u16x8-cmpeq</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise equality mask on 8 U16 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 191

<a id="value-73696d642d75313678382d636d706c74"></a>

## `simd-u16x8-cmplt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u16x8-cmplt</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise unsigned less-than mask on 8 U16 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 202

<a id="value-73696d642d75313678382d6d756c"></a>

## `simd-u16x8-mul`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u16x8-mul</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise wrapping multiplication on 8 U16 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 142

<a id="value-73696d642d75313678382d73686c"></a>

## `simd-u16x8-shl`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u16x8-shl</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise left shift (masked modulo 16) on 8 U16 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 152

<a id="value-73696d642d75313678382d736872"></a>

## `simd-u16x8-shr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u16x8-shr</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise logical right shift (masked modulo 16) on 8 U16 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 165

<a id="value-73696d642d75313678382d73706c6174"></a>

## `simd-u16x8-splat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u16x8-splat</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

Broadcast one U16 into all 8 16-bit lanes of a U128.

[Source](/reference-sources/Prim.Wide.cinch) · line 113

<a id="value-73696d642d75313678382d737562"></a>

## `simd-u16x8-sub`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u16x8-sub</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise wrapping subtraction on 8 U16 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 132

<a id="value-73696d642d75333278342d616464"></a>

## `simd-u32x4-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u32x4-add</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise wrapping addition on 4 U32 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 253

<a id="value-73696d642d75333278342d636d706571"></a>

## `simd-u32x4-cmpeq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u32x4-cmpeq</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise equality mask on 4 U32 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 361

<a id="value-73696d642d75333278342d636d706c74"></a>

## `simd-u32x4-cmplt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u32x4-cmplt</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise unsigned less-than mask on 4 U32 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 372

<a id="value-73696d642d75333278342d6d756c"></a>

## `simd-u32x4-mul`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u32x4-mul</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise wrapping multiplication on 4 U32 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 273

<a id="value-73696d642d75333278342d7061636b"></a>

## `simd-u32x4-pack`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u32x4-pack</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>))))</code></pre>

Pack four U32 lanes into a U128 with lane 0 in the least-significant 32 bits.

[Source](/reference-sources/Prim.Wide.cinch) · line 383

<a id="value-73696d642d75333278342d726f746c"></a>

## `simd-u32x4-rotl`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u32x4-rotl</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise left rotation on 4 U32 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 319

<a id="value-73696d642d75333278342d726f7472"></a>

## `simd-u32x4-rotr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u32x4-rotr</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise right rotation on 4 U32 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 330

<a id="value-73696d642d75333278342d73686c"></a>

## `simd-u32x4-shl`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u32x4-shl</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise left shift (masked modulo 32) on 4 U32 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 283

<a id="value-73696d642d75333278342d736872"></a>

## `simd-u32x4-shr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u32x4-shr</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise logical right shift (masked modulo 32) on 4 U32 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 295

<a id="value-73696d642d75333278342d73706c6174"></a>

## `simd-u32x4-splat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u32x4-splat</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

Broadcast one U32 into all 4 32-bit lanes of a U128.

[Source](/reference-sources/Prim.Wide.cinch) · line 244

<a id="value-73696d642d75333278342d737562"></a>

## `simd-u32x4-sub`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u32x4-sub</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise wrapping subtraction on 4 U32 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 263

<a id="value-73696d642d75363478322d616464"></a>

## `simd-u64x2-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u64x2-add</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise wrapping addition on 2 U64 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 411

<a id="value-73696d642d75363478322d7061636b"></a>

## `simd-u64x2-pack`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u64x2-pack</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Pack two U64 lanes into a U128 with lane 0 in the least-significant 64 bits.

[Source](/reference-sources/Prim.Wide.cinch) · line 481

<a id="value-73696d642d75363478322d726f746c"></a>

## `simd-u64x2-rotl`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u64x2-rotl</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise left rotation on 2 U64 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 457

<a id="value-73696d642d75363478322d726f7472"></a>

## `simd-u64x2-rotr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u64x2-rotr</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise right rotation on 2 U64 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 469

<a id="value-73696d642d75363478322d73686c"></a>

## `simd-u64x2-shl`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u64x2-shl</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise left shift (masked modulo 64) on 2 U64 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 431

<a id="value-73696d642d75363478322d736872"></a>

## `simd-u64x2-shr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u64x2-shr</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise logical right shift (masked modulo 64) on 2 U64 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 444

<a id="value-73696d642d75363478322d73706c6174"></a>

## `simd-u64x2-splat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u64x2-splat</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

Broadcast one U64 into both 64-bit lanes of a U128.

[Source](/reference-sources/Prim.Wide.cinch) · line 402

<a id="value-73696d642d75363478322d737562"></a>

## `simd-u64x2-sub`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u64x2-sub</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise wrapping subtraction on 2 U64 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 421

<a id="value-73696d642d75387831362d616464"></a>

## `simd-u8x16-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u8x16-add</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise wrapping addition on 16 U8 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 50

<a id="value-73696d642d75387831362d636d706571"></a>

## `simd-u8x16-cmpeq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u8x16-cmpeq</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise equality mask (0xFF per matching lane, 0x00 otherwise) on 16 U8 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 70

<a id="value-73696d642d75387831362d636d706774"></a>

## `simd-u8x16-cmpgt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u8x16-cmpgt</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise unsigned greater-than mask on 16 U8 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 92

<a id="value-73696d642d75387831362d636d706c74"></a>

## `simd-u8x16-cmplt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u8x16-cmplt</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise unsigned less-than mask on 16 U8 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 81

<a id="value-73696d642d75387831362d6d6f76656d61736b"></a>

## `simd-u8x16-movemask`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u8x16-movemask</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Extract the sign bit of each of the 16 byte lanes into the low 16 bits of a U32.

[Source](/reference-sources/Prim.Wide.cinch) · line 103

<a id="value-73696d642d75387831362d73706c6174"></a>

## `simd-u8x16-splat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u8x16-splat</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

Broadcast one U8 into all 16 byte lanes of a U128.

[Source](/reference-sources/Prim.Wide.cinch) · line 41

<a id="value-73696d642d75387831362d737562"></a>

## `simd-u8x16-sub`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">simd-u8x16-sub</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Lane-wise wrapping subtraction on 16 U8 lanes.

[Source](/reference-sources/Prim.Wide.cinch) · line 60

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Prim/Wide.cinch`. Source SHA-256: `62564558455adc4c274b06b6a87e56244c488bb922ecc72a349f5a52725f1988`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
