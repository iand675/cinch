# Data.Wide

Unboxed 128-bit integers with checked, wrapping and saturating arithmetic, bit operations, numeric conversions and explicit overflow policies.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Wide`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Wide.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Bits](Algebra.Bits.md), [Data.Int](Data.Int.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Show](Text.Show.md).

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

This module exports no type declarations.

# Values

<a id="value-6269742d616e642d69313238"></a>

## `bit-and-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">I128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-6269742d616e642d75313238"></a>

## `bit-and-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U128</span>))</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

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

<a id="value-636f756e742d6f6e65732d69313238"></a>

## `count-ones-i128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-i128</span> :: (<span class="tok-constructor">I128</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Wide.cinch)

<a id="value-636f756e742d6f6e65732d75313238"></a>

## `count-ones-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-u128</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

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

Conversions with every numeric type, in the families of the primitive
types specification section 7: total where every value fits (and from
any integer to a float), checked and saturating for every integer pair
and from a float, wrapping between integers.

[Source](/reference-sources/Data.Wide.cinch) · line 123

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

[Source](/reference-sources/Data.Wide.cinch) · line 294

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

[Source](/reference-sources/Data.Wide.cinch) · line 41

<a id="value-6d696e2d626f756e642d75313238"></a>

## `min-bound-u128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-bound-u128</span> :: <span class="tok-constructor">U128</span></code></pre>

U128 (128 bits).

[Source](/reference-sources/Data.Wide.cinch) · line 80

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

[Source](/reference-sources/Data.Wide.cinch)

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

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Wide.cinch`. Source SHA-256: `d475d7a4c34a7015f3e118fac9422d1283d7d6799b4e334a422f042344fb3133`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
