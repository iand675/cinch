# Data.Wide

Unboxed 128-bit integers with checked, wrapping and saturating arithmetic, bit operations, numeric conversions and explicit overflow policies.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Wide`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Wide.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Bits](Algebra.Bits.md), [Data.Int](Data.Int.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Prim.Wide](Prim.Wide.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Show](Text.Show.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Wide: the wide scalar types `I128` and `U128` and their
operations (design/wide-scalars.md).

The two types are the primitive types specification's optional
128-bit extension (section 2). Their names are kernel types like every
machine scalar's; their operations live here, in a module a program
imports explicitly (`import Data.Wide`), so the core prelude and every
existing program are unchanged. Names follow decision D6, as in
`Data.Machine`: `wrapping-add-i128`, `eq-u128`.

A wide value is unboxed: an LLVM `i128` in every local, parameter and
result, never a word. Overflow is never implicit, exactly as for the
narrower types: `wrapping` is two's complement wrap modulo 2^128 and
`saturating` clamps to the bounds.

The bounds carry no refinement: the refinement solver's integer sorts
are `i128` ranges, which cannot state `U128`'s maximum (2^128 - 1), so
`max-bound-u128 :: U128` is declared plain until the solver carries a
wider sort (design/wide-scalars.md section 4).

Phase 1 of the design lands in increments. This module carries what
the emitter lowers so far: bounds, the checked, wrapping and saturating
families of addition, subtraction, multiplication and negation, checked
division and remainder, the bitwise operations, checked and wrapping
shifts, rotates, bit inspection, comparison, conversions with every
other numeric type and with `Int` and `Nat`, and the `Eq`, `Ord`,
`Bits` and `Show` instances.

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

<a id="value-6269742d616e642d69313238"></a>

## `bit-and-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6269742d616e642d75313238"></a>

## `bit-and-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

Bitwise AND over two U128 values.

[Source](/reference-sources/Prim.Wide.cinch) · line 24

<a id="value-6269742d6e6f742d69313238"></a>

## `bit-not-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6269742d6e6f742d75313238"></a>

## `bit-not-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6269742d6f722d69313238"></a>

## `bit-or-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6269742d6f722d75313238"></a>

## `bit-or-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6269742d786f722d69313238"></a>

## `bit-xor-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6269742d786f722d75313238"></a>

## `bit-xor-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6164642d69313238"></a>

## `checked-add-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-add-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>)))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6164642d75313238"></a>

## `checked-add-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-add-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>)))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6469762d69313238"></a>

## `checked-div-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-div-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>)))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6469762d75313238"></a>

## `checked-div-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-div-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>)))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6633322d746f2d69313238"></a>

## `checked-f32-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f32-to-i128</span> :: (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6633322d746f2d75313238"></a>

## `checked-f32-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f32-to-u128</span> :: (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6636342d746f2d69313238"></a>

## `checked-f64-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f64-to-i128</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6636342d746f2d75313238"></a>

## `checked-f64-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f64-to-u128</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d693132382d746f2d69313238"></a>

## `checked-i128-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i128-to-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d693132382d746f2d693136"></a>

## `checked-i128-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i128-to-i16</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d693132382d746f2d693332"></a>

## `checked-i128-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i128-to-i32</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d693132382d746f2d693634"></a>

## `checked-i128-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i128-to-i64</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d693132382d746f2d6938"></a>

## `checked-i128-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i128-to-i8</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d693132382d746f2d6973697a65"></a>

## `checked-i128-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i128-to-isize</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d693132382d746f2d75313238"></a>

## `checked-i128-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i128-to-u128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d693132382d746f2d753136"></a>

## `checked-i128-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i128-to-u16</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d693132382d746f2d753332"></a>

## `checked-i128-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i128-to-u32</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d693132382d746f2d753634"></a>

## `checked-i128-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i128-to-u64</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d693132382d746f2d7538"></a>

## `checked-i128-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i128-to-u8</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d693132382d746f2d7573697a65"></a>

## `checked-i128-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i128-to-usize</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6931362d746f2d69313238"></a>

## `checked-i16-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i16-to-i128</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6931362d746f2d75313238"></a>

## `checked-i16-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i16-to-u128</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6933322d746f2d69313238"></a>

## `checked-i32-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i32-to-i128</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6933322d746f2d75313238"></a>

## `checked-i32-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i32-to-u128</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6936342d746f2d69313238"></a>

## `checked-i64-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i64-to-i128</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6936342d746f2d75313238"></a>

## `checked-i64-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i64-to-u128</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d69382d746f2d69313238"></a>

## `checked-i8-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i8-to-i128</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d69382d746f2d75313238"></a>

## `checked-i8-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i8-to-u128</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d696e742d746f2d69313238"></a>

## `checked-int-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-int-to-i128</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d696e742d746f2d75313238"></a>

## `checked-int-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-int-to-u128</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6973697a652d746f2d69313238"></a>

## `checked-isize-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-isize-to-i128</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6973697a652d746f2d75313238"></a>

## `checked-isize-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-isize-to-u128</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6d756c2d69313238"></a>

## `checked-mul-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-mul-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>)))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6d756c2d75313238"></a>

## `checked-mul-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-mul-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>)))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6e61742d746f2d69313238"></a>

## `checked-nat-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-nat-to-i128</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6e61742d746f2d75313238"></a>

## `checked-nat-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-nat-to-u128</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6e65672d69313238"></a>

## `checked-neg-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-neg-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d6e65672d75313238"></a>

## `checked-neg-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-neg-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d72656d2d69313238"></a>

## `checked-rem-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-rem-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>)))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d72656d2d75313238"></a>

## `checked-rem-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-rem-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>)))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d73686c2d69313238"></a>

## `checked-shl-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shl-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>)))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d73686c2d75313238"></a>

## `checked-shl-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shl-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>)))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d7368722d69313238"></a>

## `checked-shr-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shr-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>)))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d7368722d75313238"></a>

## `checked-shr-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shr-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>)))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d7375622d69313238"></a>

## `checked-sub-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-sub-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>)))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d7375622d75313238"></a>

## `checked-sub-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-sub-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>)))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d753132382d746f2d69313238"></a>

## `checked-u128-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u128-to-i128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d753132382d746f2d693136"></a>

## `checked-u128-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u128-to-i16</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d753132382d746f2d693332"></a>

## `checked-u128-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u128-to-i32</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d753132382d746f2d693634"></a>

## `checked-u128-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u128-to-i64</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d753132382d746f2d6938"></a>

## `checked-u128-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u128-to-i8</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d753132382d746f2d6973697a65"></a>

## `checked-u128-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u128-to-isize</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d753132382d746f2d75313238"></a>

## `checked-u128-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u128-to-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d753132382d746f2d753136"></a>

## `checked-u128-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u128-to-u16</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d753132382d746f2d753332"></a>

## `checked-u128-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u128-to-u32</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d753132382d746f2d753634"></a>

## `checked-u128-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u128-to-u64</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d753132382d746f2d7538"></a>

## `checked-u128-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u128-to-u8</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d753132382d746f2d7573697a65"></a>

## `checked-u128-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u128-to-usize</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d7531362d746f2d69313238"></a>

## `checked-u16-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u16-to-i128</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d7531362d746f2d75313238"></a>

## `checked-u16-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u16-to-u128</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d7533322d746f2d69313238"></a>

## `checked-u32-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u32-to-i128</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d7533322d746f2d75313238"></a>

## `checked-u32-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u32-to-u128</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d7536342d746f2d69313238"></a>

## `checked-u64-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u64-to-i128</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d7536342d746f2d75313238"></a>

## `checked-u64-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u64-to-u128</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d75382d746f2d69313238"></a>

## `checked-u8-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u8-to-i128</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d75382d746f2d75313238"></a>

## `checked-u8-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u8-to-u128</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d7573697a652d746f2d69313238"></a>

## `checked-usize-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-usize-to-i128</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636865636b65642d7573697a652d746f2d75313238"></a>

## `checked-usize-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-usize-to-u128</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636c6d756c2d753634"></a>

## `clmul-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">clmul-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

64x64-bit hardware carry-less multiplication into U128; callers must guard with cpu-has-clmul.

[Source](/reference-sources/Prim.Wide.cinch) · line 580

<a id="value-636f756e742d6f6e65732d69313238"></a>

## `count-ones-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636f756e742d6f6e65732d75313238"></a>

## `count-ones-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

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

<a id="value-6469762d75313238"></a>

## `div-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">div-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-65712d69313238"></a>

## `eq-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-65712d75313238"></a>

## `eq-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-67652d69313238"></a>

## `ge-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-67652d75313238"></a>

## `ge-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-67742d69313238"></a>

## `gt-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-67742d75313238"></a>

## `gt-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-693132382d746f2d663332"></a>

## `i128-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i128-to-f32</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-693132382d746f2d663634"></a>

## `i128-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i128-to-f64</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-693132382d746f2d69313238"></a>

## `i128-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i128-to-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6931362d746f2d69313238"></a>

## `i16-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i16-to-i128</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6933322d746f2d69313238"></a>

## `i32-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i32-to-i128</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6936342d746f2d69313238"></a>

## `i64-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i64-to-i128</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-69382d746f2d69313238"></a>

## `i8-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i8-to-i128</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-696e742d66726f6d2d69313238"></a>

## `int-from-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">int-from-i128</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I128</span>) -&gt; <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>) <span class="tok-constructor">Int</span>)</code></pre>

The mathematical types. Every `I128` is an `Int` and every `U128` a
`Nat`, keeping the value; the other direction states the range it
needs (a guard supplies it), and the `checked` forms answer `None`
outside it (primitive types specification section 7). The `I128`
pair keeps the value as a refinement, as `Data.Int`'s `I64` pair does.
A wide scalar has no solver sort (its range is past the solver's
`i128` arithmetic), so no bound follows from its type: a guard, or a
`checked` form, supplies `int-to-i128`'s range. The runtime carries
the 128-bit value across with the C ABI's `__int128`.

[Source](/reference-sources/Data.Wide.cinch) · line 915

<a id="value-696e742d66726f6d2d75313238"></a>

## `int-from-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">int-from-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">Int</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-696e742d746f2d69313238"></a>

## `int-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">int-to-i128</span> :: <span class="tok-keyword">using</span> (<span class="tok-number">-170141183460469231731687303715884105728</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">arg0</span>, <span class="tok-variable">arg0</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">170141183460469231731687303715884105727</span>) =&gt; ((<span class="tok-variable">arg0</span> :: <span class="tok-constructor">Int</span>) -&gt; <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">arg0</span>) <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6973697a652d746f2d69313238"></a>

## `isize-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">isize-to-i128</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6c652d69313238"></a>

## `le-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6c652d75313238"></a>

## `le-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6c656164696e672d7a65726f732d69313238"></a>

## `leading-zeros-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6c656164696e672d7a65726f732d75313238"></a>

## `leading-zeros-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6c742d69313238"></a>

## `lt-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6c742d75313238"></a>

## `lt-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6d61782d626f756e642d69313238"></a>

## `max-bound-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-bound-i128</span> :: <span class="tok-constructor">I128</span></code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6d61782d626f756e642d75313238"></a>

## `max-bound-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-bound-u128</span> :: <span class="tok-constructor">U128</span></code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6d696e2d626f756e642d69313238"></a>

## `min-bound-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-bound-i128</span> :: <span class="tok-constructor">I128</span></code></pre>

I128 (128 bits).

[Source](/reference-sources/Data.Wide.cinch) · line 52

<a id="value-6d696e2d626f756e642d75313238"></a>

## `min-bound-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-bound-u128</span> :: <span class="tok-constructor">U128</span></code></pre>

U128 (128 bits).

[Source](/reference-sources/Data.Wide.cinch) · line 259

<a id="value-6e61742d66726f6d2d75313238"></a>

## `nat-from-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">nat-from-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6e61742d746f2d75313238"></a>

## `nat-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">nat-to-u128</span> :: <span class="tok-keyword">using</span> (<span class="tok-variable">arg0</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">340282366920938463463374607431768211455</span>) =&gt; ((<span class="tok-variable">arg0</span> :: <span class="tok-constructor">Nat</span>) -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6e652d69313238"></a>

## `ne-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6e652d75313238"></a>

## `ne-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-72656d2d75313238"></a>

## `rem-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rem-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-726576657273652d626974732d69313238"></a>

## `reverse-bits-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-726576657273652d626974732d75313238"></a>

## `reverse-bits-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-726f746174652d6c6566742d69313238"></a>

## `rotate-left-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-726f746174652d6c6566742d75313238"></a>

## `rotate-left-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-726f746174652d72696768742d69313238"></a>

## `rotate-right-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-726f746174652d72696768742d75313238"></a>

## `rotate-right-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d6164642d69313238"></a>

## `saturating-add-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d6164642d75313238"></a>

## `saturating-add-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d6633322d746f2d69313238"></a>

## `saturating-f32-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f32-to-i128</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d6633322d746f2d75313238"></a>

## `saturating-f32-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f32-to-u128</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d6636342d746f2d69313238"></a>

## `saturating-f64-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-i128</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d6636342d746f2d75313238"></a>

## `saturating-f64-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-u128</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d693132382d746f2d69313238"></a>

## `saturating-i128-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i128-to-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d693132382d746f2d693136"></a>

## `saturating-i128-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i128-to-i16</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d693132382d746f2d693332"></a>

## `saturating-i128-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i128-to-i32</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d693132382d746f2d693634"></a>

## `saturating-i128-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i128-to-i64</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d693132382d746f2d6938"></a>

## `saturating-i128-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i128-to-i8</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d693132382d746f2d6973697a65"></a>

## `saturating-i128-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i128-to-isize</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d693132382d746f2d75313238"></a>

## `saturating-i128-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i128-to-u128</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d693132382d746f2d753136"></a>

## `saturating-i128-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i128-to-u16</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d693132382d746f2d753332"></a>

## `saturating-i128-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i128-to-u32</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d693132382d746f2d753634"></a>

## `saturating-i128-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i128-to-u64</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d693132382d746f2d7538"></a>

## `saturating-i128-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i128-to-u8</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d693132382d746f2d7573697a65"></a>

## `saturating-i128-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i128-to-usize</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d6931362d746f2d69313238"></a>

## `saturating-i16-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i16-to-i128</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d6931362d746f2d75313238"></a>

## `saturating-i16-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i16-to-u128</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d6933322d746f2d69313238"></a>

## `saturating-i32-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i32-to-i128</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d6933322d746f2d75313238"></a>

## `saturating-i32-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i32-to-u128</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d6936342d746f2d69313238"></a>

## `saturating-i64-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i64-to-i128</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d6936342d746f2d75313238"></a>

## `saturating-i64-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i64-to-u128</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d69382d746f2d69313238"></a>

## `saturating-i8-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i8-to-i128</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d69382d746f2d75313238"></a>

## `saturating-i8-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i8-to-u128</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d6973697a652d746f2d69313238"></a>

## `saturating-isize-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-isize-to-i128</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d6973697a652d746f2d75313238"></a>

## `saturating-isize-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-isize-to-u128</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d6d756c2d69313238"></a>

## `saturating-mul-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d6d756c2d75313238"></a>

## `saturating-mul-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d6e65672d69313238"></a>

## `saturating-neg-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-neg-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d6e65672d75313238"></a>

## `saturating-neg-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-neg-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d7375622d69313238"></a>

## `saturating-sub-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d7375622d75313238"></a>

## `saturating-sub-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d753132382d746f2d69313238"></a>

## `saturating-u128-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u128-to-i128</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d753132382d746f2d693136"></a>

## `saturating-u128-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u128-to-i16</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d753132382d746f2d693332"></a>

## `saturating-u128-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u128-to-i32</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d753132382d746f2d693634"></a>

## `saturating-u128-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u128-to-i64</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d753132382d746f2d6938"></a>

## `saturating-u128-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u128-to-i8</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d753132382d746f2d6973697a65"></a>

## `saturating-u128-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u128-to-isize</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d753132382d746f2d75313238"></a>

## `saturating-u128-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u128-to-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d753132382d746f2d753136"></a>

## `saturating-u128-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u128-to-u16</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d753132382d746f2d753332"></a>

## `saturating-u128-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u128-to-u32</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d753132382d746f2d753634"></a>

## `saturating-u128-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u128-to-u64</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d753132382d746f2d7538"></a>

## `saturating-u128-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u128-to-u8</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d753132382d746f2d7573697a65"></a>

## `saturating-u128-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u128-to-usize</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d7531362d746f2d69313238"></a>

## `saturating-u16-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u16-to-i128</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d7531362d746f2d75313238"></a>

## `saturating-u16-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u16-to-u128</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d7533322d746f2d69313238"></a>

## `saturating-u32-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u32-to-i128</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d7533322d746f2d75313238"></a>

## `saturating-u32-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u32-to-u128</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d7536342d746f2d69313238"></a>

## `saturating-u64-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u64-to-i128</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d7536342d746f2d75313238"></a>

## `saturating-u64-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u64-to-u128</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d75382d746f2d69313238"></a>

## `saturating-u8-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u8-to-i128</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d75382d746f2d75313238"></a>

## `saturating-u8-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u8-to-u128</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d7573697a652d746f2d69313238"></a>

## `saturating-usize-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-usize-to-i128</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-73617475726174696e672d7573697a652d746f2d75313238"></a>

## `saturating-usize-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-usize-to-u128</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

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

<a id="value-737761702d62797465732d69313238"></a>

## `swap-bytes-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">swap-bytes-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-737761702d62797465732d75313238"></a>

## `swap-bytes-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">swap-bytes-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-747261696c696e672d7a65726f732d69313238"></a>

## `trailing-zeros-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-747261696c696e672d7a65726f732d75313238"></a>

## `trailing-zeros-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-753132382d746f2d663332"></a>

## `u128-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u128-to-f32</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-753132382d746f2d663634"></a>

## `u128-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u128-to-f64</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-753132382d746f2d75313238"></a>

## `u128-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u128-to-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7531362d746f2d69313238"></a>

## `u16-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-i128</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7531362d746f2d75313238"></a>

## `u16-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-u128</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7533322d746f2d69313238"></a>

## `u32-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u32-to-i128</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7533322d746f2d75313238"></a>

## `u32-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u32-to-u128</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7536342d746f2d69313238"></a>

## `u64-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u64-to-i128</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7536342d746f2d75313238"></a>

## `u64-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u64-to-u128</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-75382d746f2d69313238"></a>

## `u8-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-i128</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-75382d746f2d75313238"></a>

## `u8-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-u128</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7573697a652d746f2d69313238"></a>

## `usize-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">usize-to-i128</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7573697a652d746f2d75313238"></a>

## `usize-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">usize-to-u128</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d6164642d69313238"></a>

## `wrapping-add-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d6164642d75313238"></a>

## `wrapping-add-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d693132382d746f2d69313238"></a>

## `wrapping-i128-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i128-to-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d693132382d746f2d693136"></a>

## `wrapping-i128-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i128-to-i16</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d693132382d746f2d693332"></a>

## `wrapping-i128-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i128-to-i32</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d693132382d746f2d693634"></a>

## `wrapping-i128-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i128-to-i64</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d693132382d746f2d6938"></a>

## `wrapping-i128-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i128-to-i8</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d693132382d746f2d6973697a65"></a>

## `wrapping-i128-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i128-to-isize</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d693132382d746f2d75313238"></a>

## `wrapping-i128-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i128-to-u128</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

Conversions with every numeric type.

[Source](/reference-sources/Data.Wide.cinch) · line 457

<a id="value-7772617070696e672d693132382d746f2d753136"></a>

## `wrapping-i128-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i128-to-u16</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d693132382d746f2d753332"></a>

## `wrapping-i128-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i128-to-u32</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d693132382d746f2d753634"></a>

## `wrapping-i128-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i128-to-u64</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d693132382d746f2d7538"></a>

## `wrapping-i128-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i128-to-u8</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d693132382d746f2d7573697a65"></a>

## `wrapping-i128-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i128-to-usize</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d6931362d746f2d69313238"></a>

## `wrapping-i16-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i16-to-i128</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d6931362d746f2d75313238"></a>

## `wrapping-i16-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i16-to-u128</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d6933322d746f2d69313238"></a>

## `wrapping-i32-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i32-to-i128</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d6933322d746f2d75313238"></a>

## `wrapping-i32-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i32-to-u128</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d6936342d746f2d69313238"></a>

## `wrapping-i64-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-i128</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d6936342d746f2d75313238"></a>

## `wrapping-i64-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-u128</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d69382d746f2d69313238"></a>

## `wrapping-i8-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i8-to-i128</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d69382d746f2d75313238"></a>

## `wrapping-i8-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i8-to-u128</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d6973697a652d746f2d69313238"></a>

## `wrapping-isize-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-isize-to-i128</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d6973697a652d746f2d75313238"></a>

## `wrapping-isize-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-isize-to-u128</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d6d756c2d69313238"></a>

## `wrapping-mul-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d6d756c2d75313238"></a>

## `wrapping-mul-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d6e65672d69313238"></a>

## `wrapping-neg-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d6e65672d75313238"></a>

## `wrapping-neg-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d73686c2d69313238"></a>

## `wrapping-shl-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d73686c2d75313238"></a>

## `wrapping-shl-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d7368722d69313238"></a>

## `wrapping-shr-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d7368722d75313238"></a>

## `wrapping-shr-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d7375622d69313238"></a>

## `wrapping-sub-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d7375622d75313238"></a>

## `wrapping-sub-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d753132382d746f2d69313238"></a>

## `wrapping-u128-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u128-to-i128</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d753132382d746f2d693136"></a>

## `wrapping-u128-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u128-to-i16</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d753132382d746f2d693332"></a>

## `wrapping-u128-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u128-to-i32</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d753132382d746f2d693634"></a>

## `wrapping-u128-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u128-to-i64</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d753132382d746f2d6938"></a>

## `wrapping-u128-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u128-to-i8</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d753132382d746f2d6973697a65"></a>

## `wrapping-u128-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u128-to-isize</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d753132382d746f2d75313238"></a>

## `wrapping-u128-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u128-to-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d753132382d746f2d753136"></a>

## `wrapping-u128-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u128-to-u16</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d753132382d746f2d753332"></a>

## `wrapping-u128-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u128-to-u32</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d753132382d746f2d753634"></a>

## `wrapping-u128-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u128-to-u64</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d753132382d746f2d7538"></a>

## `wrapping-u128-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u128-to-u8</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d753132382d746f2d7573697a65"></a>

## `wrapping-u128-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u128-to-usize</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d7531362d746f2d69313238"></a>

## `wrapping-u16-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u16-to-i128</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d7531362d746f2d75313238"></a>

## `wrapping-u16-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u16-to-u128</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d7533322d746f2d69313238"></a>

## `wrapping-u32-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u32-to-i128</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d7533322d746f2d75313238"></a>

## `wrapping-u32-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u32-to-u128</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d7536342d746f2d69313238"></a>

## `wrapping-u64-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-i128</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d7536342d746f2d75313238"></a>

## `wrapping-u64-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-u128</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d75382d746f2d69313238"></a>

## `wrapping-u8-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u8-to-i128</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d75382d746f2d75313238"></a>

## `wrapping-u8-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u8-to-u128</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d7573697a652d746f2d69313238"></a>

## `wrapping-usize-to-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-usize-to-i128</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">I128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-7772617070696e672d7573697a652d746f2d75313238"></a>

## `wrapping-usize-to-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-usize-to-u128</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">U128</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Wide.Bits_I128` | `Algebra.Bits.Bits I128` | Implicit |
| `Data.Wide.Bits_U128` | `Algebra.Bits.Bits U128` | Implicit |
| `Data.Wide.Eq_I128` | `Relation.Eq.Eq I128` | Implicit |
| `Data.Wide.Eq_U128` | `Relation.Eq.Eq U128` | Implicit |
| `Data.Wide.Ord_I128` | `Relation.Ord.Ord I128` | Implicit |
| `Data.Wide.Ord_U128` | `Relation.Ord.Ord U128` | Implicit |
| `Data.Wide.Show_I128` | `Text.Show.Show I128` | Implicit |
| `Data.Wide.Show_U128` | `Text.Show.Show U128` | Implicit |

**`Data.Wide.Bits_I128`**: The `Bits` instances, with the modulo-width shift and mask rules of
`Data.Machine`'s.
**`Data.Wide.Eq_I128`**: The order instances: signed order on `I128`, unsigned on `U128`
(primitive types specification section 8).
**`Data.Wide.Show_I128`**: `Show` is the decimal of the value, through `Int`.

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Wide.cinch`. Source SHA-256: `53bf486efc9f780532a0a152ebfe06f072242a9c984f5becd4bb1bb9ad34f2fa`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
