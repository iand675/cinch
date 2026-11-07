# Data.Machine

Fixed-width and target-sized scalar operations. Names make checked, wrapping, or saturating overflow policy explicit.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Machine`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Machine.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Text.Codec](Text.Codec.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Machine: the machine scalar types and their operations.

The machine scalar types follow the primitive types specification:
`I8`-`I64`, `U8`-`U64`, `Usize`, `Isize`, `F32` and `F64` are core
runtime types. Names follow decision D6: an operation is a prelude
name of the form `<op><Type>` (`wrapping-add-u32`) and a conversion
is `<src>To<dst>` (`u8-to-i32`). The `checked*`, `saturating*` and
`wrapping*` conversion families capitalize the source type
(`checked-u8-to-i64`); the total family lowercases it (`u8-to-i32`).
The specification's dotted `U32.wrappingAdd` spelling is a
presentation form (the presentation aliases in the checker).

Overflow behavior is never implicit: `checked` returns Option and
never the wrapped value, `wrapping` is two's complement wrap,
`saturating` clamps to the bounds. There is no Add/Sub/Mul/Neg
instance for machine types: a class method has a single result type
and cannot express which family is meant (primitive types
specification section 8). `F32` and `F64` have no Eq/Ord instances.

Module scheme (design/base-library.md): this is the machine
scalars' vocabulary home (the scheme names the mathematical types;
the machine family groups here), carrying its instance family
(Eq/Ord per integer type, `Bits` over the integers, and the `I64`
`Text.Codec` instance that matches the A6 codec generator's INTEGER
column: 8 bytes little-endian).

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-6162732d663332"></a>

## `abs-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">abs-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6162732d663634"></a>

## `abs-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">abs-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6164642d663332"></a>

## `add-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">add-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">F32</span>))</code></pre>

F32.

[Source](/reference-sources/Data.Machine.cinch) · line 445

<a id="value-6164642d663634"></a>

## `add-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">add-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>))</code></pre>

F64.

[Source](/reference-sources/Data.Machine.cinch) · line 468

<a id="value-6269742d616e642d693136"></a>

## `bit-and-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d616e642d693332"></a>

## `bit-and-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d616e642d693634"></a>

## `bit-and-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d616e642d6938"></a>

## `bit-and-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d616e642d6973697a65"></a>

## `bit-and-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d616e642d753136"></a>

## `bit-and-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d616e642d753332"></a>

## `bit-and-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d616e642d753634"></a>

## `bit-and-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d616e642d7538"></a>

## `bit-and-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d616e642d7573697a65"></a>

## `bit-and-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6e6f742d693136"></a>

## `bit-not-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6e6f742d693332"></a>

## `bit-not-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6e6f742d693634"></a>

## `bit-not-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6e6f742d6938"></a>

## `bit-not-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6e6f742d6973697a65"></a>

## `bit-not-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6e6f742d753136"></a>

## `bit-not-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6e6f742d753332"></a>

## `bit-not-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6e6f742d753634"></a>

## `bit-not-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6e6f742d7538"></a>

## `bit-not-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6e6f742d7573697a65"></a>

## `bit-not-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6f722d693136"></a>

## `bit-or-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6f722d693332"></a>

## `bit-or-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6f722d693634"></a>

## `bit-or-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6f722d6938"></a>

## `bit-or-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6f722d6973697a65"></a>

## `bit-or-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6f722d753136"></a>

## `bit-or-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6f722d753332"></a>

## `bit-or-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6f722d753634"></a>

## `bit-or-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6f722d7538"></a>

## `bit-or-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d6f722d7573697a65"></a>

## `bit-or-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d786f722d693136"></a>

## `bit-xor-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d786f722d693332"></a>

## `bit-xor-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d786f722d693634"></a>

## `bit-xor-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d786f722d6938"></a>

## `bit-xor-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d786f722d6973697a65"></a>

## `bit-xor-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d786f722d753136"></a>

## `bit-xor-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d786f722d753332"></a>

## `bit-xor-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d786f722d753634"></a>

## `bit-xor-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d786f722d7538"></a>

## `bit-xor-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6269742d786f722d7573697a65"></a>

## `bit-xor-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6164642d693136"></a>

## `checked-add-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-add-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6164642d693332"></a>

## `checked-add-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-add-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6164642d693634"></a>

## `checked-add-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-add-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6164642d6938"></a>

## `checked-add-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-add-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6164642d6973697a65"></a>

## `checked-add-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-add-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6164642d753136"></a>

## `checked-add-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-add-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6164642d753332"></a>

## `checked-add-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-add-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6164642d753634"></a>

## `checked-add-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-add-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6164642d7538"></a>

## `checked-add-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-add-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6164642d7573697a65"></a>

## `checked-add-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-add-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6469762d693136"></a>

## `checked-div-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-div-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6469762d693332"></a>

## `checked-div-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-div-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6469762d693634"></a>

## `checked-div-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-div-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6469762d6938"></a>

## `checked-div-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-div-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6469762d6973697a65"></a>

## `checked-div-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-div-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6469762d753136"></a>

## `checked-div-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-div-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6469762d753332"></a>

## `checked-div-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-div-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6469762d753634"></a>

## `checked-div-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-div-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6469762d7538"></a>

## `checked-div-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-div-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6469762d7573697a65"></a>

## `checked-div-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-div-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6633322d746f2d693136"></a>

## `checked-f32-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f32-to-i16</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6633322d746f2d693332"></a>

## `checked-f32-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f32-to-i32</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6633322d746f2d693634"></a>

## `checked-f32-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f32-to-i64</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6633322d746f2d6938"></a>

## `checked-f32-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f32-to-i8</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6633322d746f2d6973697a65"></a>

## `checked-f32-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f32-to-isize</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6633322d746f2d753136"></a>

## `checked-f32-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f32-to-u16</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6633322d746f2d753332"></a>

## `checked-f32-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f32-to-u32</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6633322d746f2d753634"></a>

## `checked-f32-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f32-to-u64</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6633322d746f2d7538"></a>

## `checked-f32-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f32-to-u8</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6633322d746f2d7573697a65"></a>

## `checked-f32-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f32-to-usize</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6636342d746f2d693136"></a>

## `checked-f64-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f64-to-i16</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6636342d746f2d693332"></a>

## `checked-f64-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f64-to-i32</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6636342d746f2d693634"></a>

## `checked-f64-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f64-to-i64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6636342d746f2d6938"></a>

## `checked-f64-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f64-to-i8</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6636342d746f2d6973697a65"></a>

## `checked-f64-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f64-to-isize</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6636342d746f2d753136"></a>

## `checked-f64-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f64-to-u16</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6636342d746f2d753332"></a>

## `checked-f64-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f64-to-u32</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6636342d746f2d753634"></a>

## `checked-f64-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f64-to-u64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6636342d746f2d7538"></a>

## `checked-f64-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f64-to-u8</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6636342d746f2d7573697a65"></a>

## `checked-f64-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-f64-to-usize</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6931362d746f2d693136"></a>

## `checked-i16-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i16-to-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6931362d746f2d693332"></a>

## `checked-i16-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i16-to-i32</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6931362d746f2d693634"></a>

## `checked-i16-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i16-to-i64</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6931362d746f2d6938"></a>

## `checked-i16-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i16-to-i8</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6931362d746f2d6973697a65"></a>

## `checked-i16-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i16-to-isize</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6931362d746f2d753136"></a>

## `checked-i16-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i16-to-u16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6931362d746f2d753332"></a>

## `checked-i16-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i16-to-u32</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6931362d746f2d753634"></a>

## `checked-i16-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i16-to-u64</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6931362d746f2d7538"></a>

## `checked-i16-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i16-to-u8</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6931362d746f2d7573697a65"></a>

## `checked-i16-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i16-to-usize</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6933322d746f2d693136"></a>

## `checked-i32-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i32-to-i16</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6933322d746f2d693332"></a>

## `checked-i32-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i32-to-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6933322d746f2d693634"></a>

## `checked-i32-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i32-to-i64</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6933322d746f2d6938"></a>

## `checked-i32-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i32-to-i8</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6933322d746f2d6973697a65"></a>

## `checked-i32-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i32-to-isize</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6933322d746f2d753136"></a>

## `checked-i32-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i32-to-u16</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6933322d746f2d753332"></a>

## `checked-i32-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i32-to-u32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6933322d746f2d753634"></a>

## `checked-i32-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i32-to-u64</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6933322d746f2d7538"></a>

## `checked-i32-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i32-to-u8</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6933322d746f2d7573697a65"></a>

## `checked-i32-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i32-to-usize</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6936342d746f2d693136"></a>

## `checked-i64-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i64-to-i16</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6936342d746f2d693332"></a>

## `checked-i64-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i64-to-i32</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6936342d746f2d693634"></a>

## `checked-i64-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i64-to-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6936342d746f2d6938"></a>

## `checked-i64-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i64-to-i8</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6936342d746f2d6973697a65"></a>

## `checked-i64-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i64-to-isize</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6936342d746f2d753136"></a>

## `checked-i64-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i64-to-u16</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6936342d746f2d753332"></a>

## `checked-i64-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i64-to-u32</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6936342d746f2d753634"></a>

## `checked-i64-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i64-to-u64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6936342d746f2d7538"></a>

## `checked-i64-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i64-to-u8</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6936342d746f2d7573697a65"></a>

## `checked-i64-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i64-to-usize</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d69382d746f2d693136"></a>

## `checked-i8-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i8-to-i16</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d69382d746f2d693332"></a>

## `checked-i8-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i8-to-i32</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d69382d746f2d693634"></a>

## `checked-i8-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i8-to-i64</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d69382d746f2d6938"></a>

## `checked-i8-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i8-to-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>)</code></pre>

Checked, saturating and wrapping integer conversions; float to
integer is checked or saturating (it is never total).

[Source](/reference-sources/Data.Machine.cinch) · line 572

<a id="value-636865636b65642d69382d746f2d6973697a65"></a>

## `checked-i8-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i8-to-isize</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d69382d746f2d753136"></a>

## `checked-i8-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i8-to-u16</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d69382d746f2d753332"></a>

## `checked-i8-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i8-to-u32</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d69382d746f2d753634"></a>

## `checked-i8-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i8-to-u64</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d69382d746f2d7538"></a>

## `checked-i8-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i8-to-u8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d69382d746f2d7573697a65"></a>

## `checked-i8-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-i8-to-usize</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6973697a652d746f2d693136"></a>

## `checked-isize-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-isize-to-i16</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6973697a652d746f2d693332"></a>

## `checked-isize-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-isize-to-i32</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6973697a652d746f2d693634"></a>

## `checked-isize-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-isize-to-i64</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6973697a652d746f2d6938"></a>

## `checked-isize-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-isize-to-i8</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6973697a652d746f2d6973697a65"></a>

## `checked-isize-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-isize-to-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6973697a652d746f2d753136"></a>

## `checked-isize-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-isize-to-u16</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6973697a652d746f2d753332"></a>

## `checked-isize-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-isize-to-u32</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6973697a652d746f2d753634"></a>

## `checked-isize-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-isize-to-u64</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6973697a652d746f2d7538"></a>

## `checked-isize-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-isize-to-u8</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6973697a652d746f2d7573697a65"></a>

## `checked-isize-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-isize-to-usize</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6d756c2d693136"></a>

## `checked-mul-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-mul-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6d756c2d693332"></a>

## `checked-mul-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-mul-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6d756c2d693634"></a>

## `checked-mul-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-mul-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6d756c2d6938"></a>

## `checked-mul-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-mul-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6d756c2d6973697a65"></a>

## `checked-mul-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-mul-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6d756c2d753136"></a>

## `checked-mul-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-mul-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6d756c2d753332"></a>

## `checked-mul-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-mul-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6d756c2d753634"></a>

## `checked-mul-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-mul-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6d756c2d7538"></a>

## `checked-mul-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-mul-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6d756c2d7573697a65"></a>

## `checked-mul-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-mul-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6e65672d693136"></a>

## `checked-neg-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-neg-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6e65672d693332"></a>

## `checked-neg-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-neg-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6e65672d693634"></a>

## `checked-neg-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-neg-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6e65672d6938"></a>

## `checked-neg-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-neg-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6e65672d6973697a65"></a>

## `checked-neg-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-neg-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6e65672d753136"></a>

## `checked-neg-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-neg-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6e65672d753332"></a>

## `checked-neg-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-neg-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6e65672d753634"></a>

## `checked-neg-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-neg-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6e65672d7538"></a>

## `checked-neg-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-neg-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d6e65672d7573697a65"></a>

## `checked-neg-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-neg-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d72656d2d693136"></a>

## `checked-rem-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-rem-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d72656d2d693332"></a>

## `checked-rem-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-rem-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d72656d2d693634"></a>

## `checked-rem-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-rem-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d72656d2d6938"></a>

## `checked-rem-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-rem-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d72656d2d6973697a65"></a>

## `checked-rem-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-rem-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d72656d2d753136"></a>

## `checked-rem-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-rem-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d72656d2d753332"></a>

## `checked-rem-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-rem-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d72656d2d753634"></a>

## `checked-rem-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-rem-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d72656d2d7538"></a>

## `checked-rem-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-rem-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d72656d2d7573697a65"></a>

## `checked-rem-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-rem-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d73686c2d693136"></a>

## `checked-shl-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shl-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d73686c2d693332"></a>

## `checked-shl-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shl-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d73686c2d693634"></a>

## `checked-shl-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shl-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d73686c2d6938"></a>

## `checked-shl-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shl-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d73686c2d6973697a65"></a>

## `checked-shl-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shl-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d73686c2d753136"></a>

## `checked-shl-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shl-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d73686c2d753332"></a>

## `checked-shl-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shl-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d73686c2d753634"></a>

## `checked-shl-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shl-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d73686c2d7538"></a>

## `checked-shl-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shl-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d73686c2d7573697a65"></a>

## `checked-shl-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shl-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7368722d693136"></a>

## `checked-shr-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shr-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7368722d693332"></a>

## `checked-shr-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shr-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7368722d693634"></a>

## `checked-shr-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shr-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7368722d6938"></a>

## `checked-shr-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shr-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7368722d6973697a65"></a>

## `checked-shr-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shr-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7368722d753136"></a>

## `checked-shr-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shr-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7368722d753332"></a>

## `checked-shr-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shr-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7368722d753634"></a>

## `checked-shr-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shr-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7368722d7538"></a>

## `checked-shr-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shr-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7368722d7573697a65"></a>

## `checked-shr-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-shr-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7375622d693136"></a>

## `checked-sub-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-sub-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7375622d693332"></a>

## `checked-sub-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-sub-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7375622d693634"></a>

## `checked-sub-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-sub-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7375622d6938"></a>

## `checked-sub-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-sub-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7375622d6973697a65"></a>

## `checked-sub-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-sub-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7375622d753136"></a>

## `checked-sub-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-sub-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7375622d753332"></a>

## `checked-sub-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-sub-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7375622d753634"></a>

## `checked-sub-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-sub-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7375622d7538"></a>

## `checked-sub-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-sub-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7375622d7573697a65"></a>

## `checked-sub-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-sub-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7531362d746f2d693136"></a>

## `checked-u16-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u16-to-i16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7531362d746f2d693332"></a>

## `checked-u16-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u16-to-i32</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7531362d746f2d693634"></a>

## `checked-u16-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u16-to-i64</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7531362d746f2d6938"></a>

## `checked-u16-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u16-to-i8</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7531362d746f2d6973697a65"></a>

## `checked-u16-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u16-to-isize</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7531362d746f2d753136"></a>

## `checked-u16-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u16-to-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7531362d746f2d753332"></a>

## `checked-u16-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u16-to-u32</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7531362d746f2d753634"></a>

## `checked-u16-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u16-to-u64</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7531362d746f2d7538"></a>

## `checked-u16-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u16-to-u8</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7531362d746f2d7573697a65"></a>

## `checked-u16-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u16-to-usize</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7533322d746f2d693136"></a>

## `checked-u32-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u32-to-i16</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7533322d746f2d693332"></a>

## `checked-u32-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u32-to-i32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7533322d746f2d693634"></a>

## `checked-u32-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u32-to-i64</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7533322d746f2d6938"></a>

## `checked-u32-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u32-to-i8</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7533322d746f2d6973697a65"></a>

## `checked-u32-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u32-to-isize</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7533322d746f2d753136"></a>

## `checked-u32-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u32-to-u16</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7533322d746f2d753332"></a>

## `checked-u32-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u32-to-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7533322d746f2d753634"></a>

## `checked-u32-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u32-to-u64</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7533322d746f2d7538"></a>

## `checked-u32-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u32-to-u8</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7533322d746f2d7573697a65"></a>

## `checked-u32-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u32-to-usize</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7536342d746f2d693136"></a>

## `checked-u64-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u64-to-i16</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7536342d746f2d693332"></a>

## `checked-u64-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u64-to-i32</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7536342d746f2d693634"></a>

## `checked-u64-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u64-to-i64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7536342d746f2d6938"></a>

## `checked-u64-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u64-to-i8</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7536342d746f2d6973697a65"></a>

## `checked-u64-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u64-to-isize</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7536342d746f2d753136"></a>

## `checked-u64-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u64-to-u16</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7536342d746f2d753332"></a>

## `checked-u64-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u64-to-u32</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7536342d746f2d753634"></a>

## `checked-u64-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u64-to-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7536342d746f2d7538"></a>

## `checked-u64-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u64-to-u8</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7536342d746f2d7573697a65"></a>

## `checked-u64-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u64-to-usize</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d75382d746f2d693136"></a>

## `checked-u8-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u8-to-i16</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d75382d746f2d693332"></a>

## `checked-u8-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u8-to-i32</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d75382d746f2d693634"></a>

## `checked-u8-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u8-to-i64</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d75382d746f2d6938"></a>

## `checked-u8-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u8-to-i8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d75382d746f2d6973697a65"></a>

## `checked-u8-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u8-to-isize</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d75382d746f2d753136"></a>

## `checked-u8-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u8-to-u16</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d75382d746f2d753332"></a>

## `checked-u8-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u8-to-u32</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d75382d746f2d753634"></a>

## `checked-u8-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u8-to-u64</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d75382d746f2d7538"></a>

## `checked-u8-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u8-to-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d75382d746f2d7573697a65"></a>

## `checked-u8-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-u8-to-usize</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7573697a652d746f2d693136"></a>

## `checked-usize-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-usize-to-i16</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7573697a652d746f2d693332"></a>

## `checked-usize-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-usize-to-i32</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7573697a652d746f2d693634"></a>

## `checked-usize-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-usize-to-i64</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7573697a652d746f2d6938"></a>

## `checked-usize-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-usize-to-i8</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7573697a652d746f2d6973697a65"></a>

## `checked-usize-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-usize-to-isize</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7573697a652d746f2d753136"></a>

## `checked-usize-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-usize-to-u16</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7573697a652d746f2d753332"></a>

## `checked-usize-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-usize-to-u32</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7573697a652d746f2d753634"></a>

## `checked-usize-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-usize-to-u64</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7573697a652d746f2d7538"></a>

## `checked-usize-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-usize-to-u8</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636865636b65642d7573697a652d746f2d7573697a65"></a>

## `checked-usize-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-usize-to-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Option</span> <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636f756e742d6f6e65732d693136"></a>

## `count-ones-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636f756e742d6f6e65732d693332"></a>

## `count-ones-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636f756e742d6f6e65732d693634"></a>

## `count-ones-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636f756e742d6f6e65732d6938"></a>

## `count-ones-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636f756e742d6f6e65732d6973697a65"></a>

## `count-ones-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636f756e742d6f6e65732d753136"></a>

## `count-ones-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636f756e742d6f6e65732d753332"></a>

## `count-ones-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636f756e742d6f6e65732d753634"></a>

## `count-ones-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636f756e742d6f6e65732d7538"></a>

## `count-ones-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-636f756e742d6f6e65732d7573697a65"></a>

## `count-ones-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6469762d663332"></a>

## `div-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">div-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">F32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6469762d663634"></a>

## `div-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">div-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-65712d693136"></a>

## `eq-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-65712d693332"></a>

## `eq-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-65712d693634"></a>

## `eq-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-65712d6938"></a>

## `eq-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-65712d6973697a65"></a>

## `eq-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-65712d753136"></a>

## `eq-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-65712d753332"></a>

## `eq-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-65712d753634"></a>

## `eq-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-65712d7538"></a>

## `eq-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-65712d7573697a65"></a>

## `eq-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6633322d746f2d663332"></a>

## `f32-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f32-to-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6633322d746f2d663634"></a>

## `f32-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f32-to-f64</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6636342d66726f6d2d646563696d616c"></a>

## `f64-from-decimal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f64-from-decimal</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6636342d746f2d646563696d616c"></a>

## `f64-to-decimal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f64-to-decimal</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The P31 text conversions of `F64` (compiler/runtime/src/text.rs;
the `cinch.runtime.textF64*` intrinsics, nm-verified there):
one float's shortest round-trip decimal text (scientific form)
and the float one decimal spelling stands for (correctly
rounded). A spelling that is not a decimal number takes the
defined `MalformedDecimal` fault (exit 71).

[Source](/reference-sources/Data.Machine.cinch) · line 1219

<a id="value-6636342d746f2d663332"></a>

## `f64-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f64-to-f32</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6636342d746f2d663634"></a>

## `f64-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f64-to-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6665712d663332"></a>

## `feq-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">feq-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6665712d663634"></a>

## `feq-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">feq-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6667652d663332"></a>

## `fge-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fge-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6667652d663634"></a>

## `fge-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fge-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6667742d663332"></a>

## `fgt-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fgt-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6667742d663634"></a>

## `fgt-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fgt-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-666c652d663332"></a>

## `fle-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fle-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-666c652d663634"></a>

## `fle-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fle-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-666c742d663332"></a>

## `flt-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flt-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-666c742d663634"></a>

## `flt-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flt-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-666e652d663332"></a>

## `fne-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fne-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-666e652d663634"></a>

## `fne-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fne-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67652d693136"></a>

## `ge-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67652d693332"></a>

## `ge-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67652d693634"></a>

## `ge-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67652d6938"></a>

## `ge-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67652d6973697a65"></a>

## `ge-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67652d753136"></a>

## `ge-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67652d753332"></a>

## `ge-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67652d753634"></a>

## `ge-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67652d7538"></a>

## `ge-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67652d7573697a65"></a>

## `ge-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67742d693136"></a>

## `gt-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67742d693332"></a>

## `gt-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67742d693634"></a>

## `gt-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67742d6938"></a>

## `gt-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67742d6973697a65"></a>

## `gt-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67742d753136"></a>

## `gt-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67742d753332"></a>

## `gt-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67742d753634"></a>

## `gt-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67742d7538"></a>

## `gt-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-67742d7573697a65"></a>

## `gt-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6931362d746f2d663332"></a>

## `i16-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i16-to-f32</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6931362d746f2d663634"></a>

## `i16-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i16-to-f64</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6931362d746f2d693136"></a>

## `i16-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i16-to-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6931362d746f2d693332"></a>

## `i16-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i16-to-i32</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I16</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">-32768</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">32767</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6931362d746f2d693634"></a>

## `i16-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i16-to-i64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I16</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">-32768</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">32767</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6931362d746f2d6973697a65"></a>

## `i16-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i16-to-isize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I16</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">-32768</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">32767</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6933322d746f2d663332"></a>

## `i32-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i32-to-f32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6933322d746f2d663634"></a>

## `i32-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i32-to-f64</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6933322d746f2d693332"></a>

## `i32-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i32-to-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6933322d746f2d693634"></a>

## `i32-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i32-to-i64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I32</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">-2147483648</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">2147483647</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6933322d746f2d6973697a65"></a>

## `i32-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i32-to-isize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I32</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">-2147483648</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">2147483647</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6936342d6c652d62797465"></a>

## `i64-le-byte`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i64-le-byte</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">255</span> <span class="tok-constructor">Nat</span>))</code></pre>

One little-endian byte of an `I64` at shift `k` bits (the A6
INTEGER column format, modules specification section 11).

[Source](/reference-sources/Data.Machine.cinch) · line 1210

<a id="value-6936342d746f2d663332"></a>

## `i64-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i64-to-f32</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6936342d746f2d663634"></a>

## `i64-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i64-to-f64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6936342d746f2d693634"></a>

## `i64-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i64-to-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6936342d746f2d6973697a65"></a>

## `i64-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i64-to-isize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I64</span>) -&gt; <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>) <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-69382d746f2d663332"></a>

## `i8-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i8-to-f32</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-69382d746f2d663634"></a>

## `i8-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i8-to-f64</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-69382d746f2d693136"></a>

## `i8-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i8-to-i16</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">-128</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">127</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-69382d746f2d693332"></a>

## `i8-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i8-to-i32</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">-128</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">127</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-69382d746f2d693634"></a>

## `i8-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i8-to-i64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">-128</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">127</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-69382d746f2d6938"></a>

## `i8-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i8-to-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

Primitive types specification, section 7: explicit conversions for
every ordered pair of numeric machine types. The total family
(`u8-to-i32`) exists when the conversion preserves every value, and
for integer-to-float and float-to-float, which round per IEEE 754.
`checked` returns Option (None on NaN and out-of-range floats),
`saturating` clamps (NaN maps to zero), `wrapping` truncates in
two's complement and is defined between integer types only. A
total integer conversion's result is its source's value: it states
that (`r == x`, a fact over the argument wherever the argument has
a reading) and the source's range (`InRange 0 255 r` for a `U8`,
which holds even where it has none), so a converted byte is a
non-negative `I64` for `nat-from-i64` and below 256 wherever that
matters.

[Source](/reference-sources/Data.Machine.cinch) · line 504

<a id="value-69382d746f2d6973697a65"></a>

## `i8-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i8-to-isize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">-128</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">127</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-69732d66696e6974652d663332"></a>

## `is-finite-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-finite-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-69732d66696e6974652d663634"></a>

## `is-finite-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-finite-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-69732d696e66696e6974652d663332"></a>

## `is-infinite-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-infinite-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-69732d696e66696e6974652d663634"></a>

## `is-infinite-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-infinite-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-69732d6e612d6e663332"></a>

## `is-na-nf32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-na-nf32</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-69732d6e612d6e663634"></a>

## `is-na-nf64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-na-nf64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-69732d6e6f726d616c2d663332"></a>

## `is-normal-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-normal-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-69732d6e6f726d616c2d663634"></a>

## `is-normal-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-normal-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6973697a652d746f2d663332"></a>

## `isize-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">isize-to-f32</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6973697a652d746f2d663634"></a>

## `isize-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">isize-to-f64</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6973697a652d746f2d693634"></a>

## `isize-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">isize-to-i64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">Isize</span>) -&gt; <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>) <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6973697a652d746f2d6973697a65"></a>

## `isize-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">isize-to-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c652d693136"></a>

## `le-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c652d693332"></a>

## `le-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c652d693634"></a>

## `le-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c652d6938"></a>

## `le-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c652d6973697a65"></a>

## `le-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c652d753136"></a>

## `le-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c652d753332"></a>

## `le-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c652d753634"></a>

## `le-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c652d7538"></a>

## `le-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c652d7573697a65"></a>

## `le-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c656164696e672d7a65726f732d693136"></a>

## `leading-zeros-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c656164696e672d7a65726f732d693332"></a>

## `leading-zeros-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c656164696e672d7a65726f732d693634"></a>

## `leading-zeros-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c656164696e672d7a65726f732d6938"></a>

## `leading-zeros-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c656164696e672d7a65726f732d6973697a65"></a>

## `leading-zeros-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c656164696e672d7a65726f732d753136"></a>

## `leading-zeros-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c656164696e672d7a65726f732d753332"></a>

## `leading-zeros-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c656164696e672d7a65726f732d753634"></a>

## `leading-zeros-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c656164696e672d7a65726f732d7538"></a>

## `leading-zeros-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c656164696e672d7a65726f732d7573697a65"></a>

## `leading-zeros-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c742d693136"></a>

## `lt-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c742d693332"></a>

## `lt-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c742d693634"></a>

## `lt-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c742d6938"></a>

## `lt-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c742d6973697a65"></a>

## `lt-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c742d753136"></a>

## `lt-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c742d753332"></a>

## `lt-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c742d753634"></a>

## `lt-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c742d7538"></a>

## `lt-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6c742d7573697a65"></a>

## `lt-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6d61736b2d693634"></a>

## `mask-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mask-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; ((<span class="tok-variable">a1</span> :: <span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">9223372036854775807</span> <span class="tok-constructor">I64</span>) -&gt; <span class="tok-variable">refined</span> ((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">a1</span>)) <span class="tok-constructor">I64</span>))</code></pre>

A mask with a non-negative mask word: the `bit-and` of any word
with `m >= 0` lies in `[0, m]` (the and clears the sign bit and
every bit `m` lacks), so `mask-i64 x 255` is a byte. It is the same
intrinsic as `bit-and-i64`; only the declared range differs, which
`bit-and-i64` itself cannot state because a negative mask (`-1`,
the high bit) is an ordinary use of it.

[Source](/reference-sources/Data.Machine.cinch) · line 1194

<a id="value-6d61782d626f756e642d693136"></a>

## `max-bound-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-bound-i16</span> :: <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-number">32767</span>) <span class="tok-constructor">I16</span></code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6d61782d626f756e642d693332"></a>

## `max-bound-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-bound-i32</span> :: <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-number">2147483647</span>) <span class="tok-constructor">I32</span></code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6d61782d626f756e642d693634"></a>

## `max-bound-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-bound-i64</span> :: <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-number">9223372036854775807</span>) <span class="tok-constructor">I64</span></code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6d61782d626f756e642d6938"></a>

## `max-bound-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-bound-i8</span> :: <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-number">127</span>) <span class="tok-constructor">I8</span></code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6d61782d626f756e642d6973697a65"></a>

## `max-bound-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-bound-isize</span> :: <span class="tok-constructor">Isize</span></code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6d61782d626f756e642d753136"></a>

## `max-bound-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-bound-u16</span> :: <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-number">65535</span>) <span class="tok-constructor">U16</span></code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6d61782d626f756e642d753332"></a>

## `max-bound-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-bound-u32</span> :: <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-number">4294967295</span>) <span class="tok-constructor">U32</span></code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6d61782d626f756e642d753634"></a>

## `max-bound-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-bound-u64</span> :: <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-number">18446744073709551615</span>) <span class="tok-constructor">U64</span></code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6d61782d626f756e642d7538"></a>

## `max-bound-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-bound-u8</span> :: <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-number">255</span>) <span class="tok-constructor">U8</span></code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6d61782d626f756e642d7573697a65"></a>

## `max-bound-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">max-bound-usize</span> :: <span class="tok-constructor">Usize</span></code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6d6178696d756d2d663332"></a>

## `maximum-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maximum-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">F32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6d6178696d756d2d663634"></a>

## `maximum-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maximum-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6d696e2d626f756e642d693136"></a>

## `min-bound-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-bound-i16</span> :: <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-number">-32768</span>) <span class="tok-constructor">I16</span></code></pre>

I16 (16 bits).

[Source](/reference-sources/Data.Machine.cinch) · line 89

<a id="value-6d696e2d626f756e642d693332"></a>

## `min-bound-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-bound-i32</span> :: <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-number">-2147483648</span>) <span class="tok-constructor">I32</span></code></pre>

I32 (32 bits).

[Source](/reference-sources/Data.Machine.cinch) · line 128

<a id="value-6d696e2d626f756e642d693634"></a>

## `min-bound-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-bound-i64</span> :: <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-number">-9223372036854775808</span>) <span class="tok-constructor">I64</span></code></pre>

I64 (64 bits).

[Source](/reference-sources/Data.Machine.cinch) · line 167

<a id="value-6d696e2d626f756e642d6938"></a>

## `min-bound-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-bound-i8</span> :: <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-number">-128</span>) <span class="tok-constructor">I8</span></code></pre>

I8 (8 bits).
A bound states its value, so `nat-from-i64 max-bound-i64` and the
like discharge their range with nothing written.

[Source](/reference-sources/Data.Machine.cinch) · line 50

<a id="value-6d696e2d626f756e642d6973697a65"></a>

## `min-bound-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-bound-isize</span> :: <span class="tok-constructor">Isize</span></code></pre>

Isize (64 bits, target word).

[Source](/reference-sources/Data.Machine.cinch) · line 401

<a id="value-6d696e2d626f756e642d753136"></a>

## `min-bound-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-bound-u16</span> :: <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-number">0</span>) <span class="tok-constructor">U16</span></code></pre>

U16 (16 bits).

[Source](/reference-sources/Data.Machine.cinch) · line 245

<a id="value-6d696e2d626f756e642d753332"></a>

## `min-bound-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-bound-u32</span> :: <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-number">0</span>) <span class="tok-constructor">U32</span></code></pre>

U32 (32 bits).

[Source](/reference-sources/Data.Machine.cinch) · line 284

<a id="value-6d696e2d626f756e642d753634"></a>

## `min-bound-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-bound-u64</span> :: <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-number">0</span>) <span class="tok-constructor">U64</span></code></pre>

U64 (64 bits).

[Source](/reference-sources/Data.Machine.cinch) · line 323

<a id="value-6d696e2d626f756e642d7538"></a>

## `min-bound-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-bound-u8</span> :: <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-number">0</span>) <span class="tok-constructor">U8</span></code></pre>

U8 (8 bits).

[Source](/reference-sources/Data.Machine.cinch) · line 206

<a id="value-6d696e2d626f756e642d7573697a65"></a>

## `min-bound-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">min-bound-usize</span> :: <span class="tok-constructor">Usize</span></code></pre>

Usize (64 bits, target word).

[Source](/reference-sources/Data.Machine.cinch) · line 362

<a id="value-6d696e696d756d2d663332"></a>

## `minimum-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">minimum-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">F32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6d696e696d756d2d663634"></a>

## `minimum-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">minimum-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6d756c2d663332"></a>

## `mul-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mul-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">F32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6d756c2d663634"></a>

## `mul-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mul-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6e652d693136"></a>

## `ne-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6e652d693332"></a>

## `ne-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6e652d693634"></a>

## `ne-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6e652d6938"></a>

## `ne-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6e652d6973697a65"></a>

## `ne-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6e652d753136"></a>

## `ne-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6e652d753332"></a>

## `ne-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6e652d753634"></a>

## `ne-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6e652d7538"></a>

## `ne-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6e652d7573697a65"></a>

## `ne-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6e65672d663332"></a>

## `neg-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">neg-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-6e65672d663634"></a>

## `neg-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">neg-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726576657273652d626974732d693136"></a>

## `reverse-bits-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726576657273652d626974732d693332"></a>

## `reverse-bits-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726576657273652d626974732d693634"></a>

## `reverse-bits-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726576657273652d626974732d6938"></a>

## `reverse-bits-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726576657273652d626974732d6973697a65"></a>

## `reverse-bits-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726576657273652d626974732d753136"></a>

## `reverse-bits-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726576657273652d626974732d753332"></a>

## `reverse-bits-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726576657273652d626974732d753634"></a>

## `reverse-bits-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726576657273652d626974732d7538"></a>

## `reverse-bits-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726576657273652d626974732d7573697a65"></a>

## `reverse-bits-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d6c6566742d693136"></a>

## `rotate-left-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d6c6566742d693332"></a>

## `rotate-left-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d6c6566742d693634"></a>

## `rotate-left-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d6c6566742d6938"></a>

## `rotate-left-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d6c6566742d6973697a65"></a>

## `rotate-left-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d6c6566742d753136"></a>

## `rotate-left-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d6c6566742d753332"></a>

## `rotate-left-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d6c6566742d753634"></a>

## `rotate-left-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d6c6566742d7538"></a>

## `rotate-left-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d6c6566742d7573697a65"></a>

## `rotate-left-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d72696768742d693136"></a>

## `rotate-right-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d72696768742d693332"></a>

## `rotate-right-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d72696768742d693634"></a>

## `rotate-right-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d72696768742d6938"></a>

## `rotate-right-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d72696768742d6973697a65"></a>

## `rotate-right-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d72696768742d753136"></a>

## `rotate-right-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d72696768742d753332"></a>

## `rotate-right-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d72696768742d753634"></a>

## `rotate-right-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d72696768742d7538"></a>

## `rotate-right-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-726f746174652d72696768742d7573697a65"></a>

## `rotate-right-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6164642d693136"></a>

## `saturating-add-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6164642d693332"></a>

## `saturating-add-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6164642d693634"></a>

## `saturating-add-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6164642d6938"></a>

## `saturating-add-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6164642d6973697a65"></a>

## `saturating-add-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6164642d753136"></a>

## `saturating-add-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6164642d753332"></a>

## `saturating-add-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6164642d753634"></a>

## `saturating-add-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6164642d7538"></a>

## `saturating-add-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6164642d7573697a65"></a>

## `saturating-add-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6633322d746f2d693136"></a>

## `saturating-f32-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f32-to-i16</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6633322d746f2d693332"></a>

## `saturating-f32-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f32-to-i32</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6633322d746f2d693634"></a>

## `saturating-f32-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f32-to-i64</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6633322d746f2d6938"></a>

## `saturating-f32-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f32-to-i8</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6633322d746f2d6973697a65"></a>

## `saturating-f32-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f32-to-isize</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6633322d746f2d753136"></a>

## `saturating-f32-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f32-to-u16</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6633322d746f2d753332"></a>

## `saturating-f32-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f32-to-u32</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6633322d746f2d753634"></a>

## `saturating-f32-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f32-to-u64</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6633322d746f2d7538"></a>

## `saturating-f32-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f32-to-u8</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6633322d746f2d7573697a65"></a>

## `saturating-f32-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f32-to-usize</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6636342d746f2d693136"></a>

## `saturating-f64-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-i16</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6636342d746f2d693332"></a>

## `saturating-f64-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-i32</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6636342d746f2d693634"></a>

## `saturating-f64-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-i64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6636342d746f2d6938"></a>

## `saturating-f64-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-i8</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6636342d746f2d6973697a65"></a>

## `saturating-f64-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-isize</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6636342d746f2d753136"></a>

## `saturating-f64-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-u16</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6636342d746f2d753332"></a>

## `saturating-f64-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-u32</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6636342d746f2d753634"></a>

## `saturating-f64-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-u64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6636342d746f2d7538"></a>

## `saturating-f64-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-u8</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6636342d746f2d7573697a65"></a>

## `saturating-f64-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-usize</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6931362d746f2d693136"></a>

## `saturating-i16-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i16-to-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6931362d746f2d693332"></a>

## `saturating-i16-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i16-to-i32</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6931362d746f2d693634"></a>

## `saturating-i16-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i16-to-i64</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6931362d746f2d6938"></a>

## `saturating-i16-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i16-to-i8</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6931362d746f2d6973697a65"></a>

## `saturating-i16-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i16-to-isize</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6931362d746f2d753136"></a>

## `saturating-i16-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i16-to-u16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6931362d746f2d753332"></a>

## `saturating-i16-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i16-to-u32</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6931362d746f2d753634"></a>

## `saturating-i16-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i16-to-u64</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6931362d746f2d7538"></a>

## `saturating-i16-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i16-to-u8</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6931362d746f2d7573697a65"></a>

## `saturating-i16-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i16-to-usize</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6933322d746f2d693136"></a>

## `saturating-i32-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i32-to-i16</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6933322d746f2d693332"></a>

## `saturating-i32-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i32-to-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6933322d746f2d693634"></a>

## `saturating-i32-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i32-to-i64</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6933322d746f2d6938"></a>

## `saturating-i32-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i32-to-i8</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6933322d746f2d6973697a65"></a>

## `saturating-i32-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i32-to-isize</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6933322d746f2d753136"></a>

## `saturating-i32-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i32-to-u16</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6933322d746f2d753332"></a>

## `saturating-i32-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i32-to-u32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6933322d746f2d753634"></a>

## `saturating-i32-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i32-to-u64</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6933322d746f2d7538"></a>

## `saturating-i32-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i32-to-u8</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6933322d746f2d7573697a65"></a>

## `saturating-i32-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i32-to-usize</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6936342d746f2d693136"></a>

## `saturating-i64-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i64-to-i16</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6936342d746f2d693332"></a>

## `saturating-i64-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i64-to-i32</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6936342d746f2d693634"></a>

## `saturating-i64-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i64-to-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6936342d746f2d6938"></a>

## `saturating-i64-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i64-to-i8</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6936342d746f2d6973697a65"></a>

## `saturating-i64-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i64-to-isize</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6936342d746f2d753136"></a>

## `saturating-i64-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i64-to-u16</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6936342d746f2d753332"></a>

## `saturating-i64-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i64-to-u32</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6936342d746f2d753634"></a>

## `saturating-i64-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i64-to-u64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6936342d746f2d7538"></a>

## `saturating-i64-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i64-to-u8</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6936342d746f2d7573697a65"></a>

## `saturating-i64-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i64-to-usize</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d69382d746f2d693136"></a>

## `saturating-i8-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i8-to-i16</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d69382d746f2d693332"></a>

## `saturating-i8-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i8-to-i32</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d69382d746f2d693634"></a>

## `saturating-i8-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i8-to-i64</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d69382d746f2d6938"></a>

## `saturating-i8-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i8-to-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d69382d746f2d6973697a65"></a>

## `saturating-i8-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i8-to-isize</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d69382d746f2d753136"></a>

## `saturating-i8-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i8-to-u16</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d69382d746f2d753332"></a>

## `saturating-i8-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i8-to-u32</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d69382d746f2d753634"></a>

## `saturating-i8-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i8-to-u64</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d69382d746f2d7538"></a>

## `saturating-i8-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i8-to-u8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d69382d746f2d7573697a65"></a>

## `saturating-i8-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-i8-to-usize</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6973697a652d746f2d693136"></a>

## `saturating-isize-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-isize-to-i16</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6973697a652d746f2d693332"></a>

## `saturating-isize-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-isize-to-i32</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6973697a652d746f2d693634"></a>

## `saturating-isize-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-isize-to-i64</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6973697a652d746f2d6938"></a>

## `saturating-isize-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-isize-to-i8</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6973697a652d746f2d6973697a65"></a>

## `saturating-isize-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-isize-to-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6973697a652d746f2d753136"></a>

## `saturating-isize-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-isize-to-u16</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6973697a652d746f2d753332"></a>

## `saturating-isize-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-isize-to-u32</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6973697a652d746f2d753634"></a>

## `saturating-isize-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-isize-to-u64</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6973697a652d746f2d7538"></a>

## `saturating-isize-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-isize-to-u8</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6973697a652d746f2d7573697a65"></a>

## `saturating-isize-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-isize-to-usize</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6d756c2d693136"></a>

## `saturating-mul-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6d756c2d693332"></a>

## `saturating-mul-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6d756c2d693634"></a>

## `saturating-mul-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6d756c2d6938"></a>

## `saturating-mul-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6d756c2d6973697a65"></a>

## `saturating-mul-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6d756c2d753136"></a>

## `saturating-mul-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6d756c2d753332"></a>

## `saturating-mul-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6d756c2d753634"></a>

## `saturating-mul-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6d756c2d7538"></a>

## `saturating-mul-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6d756c2d7573697a65"></a>

## `saturating-mul-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6e65672d693136"></a>

## `saturating-neg-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-neg-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6e65672d693332"></a>

## `saturating-neg-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-neg-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6e65672d693634"></a>

## `saturating-neg-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-neg-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6e65672d6938"></a>

## `saturating-neg-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-neg-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6e65672d6973697a65"></a>

## `saturating-neg-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-neg-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6e65672d753136"></a>

## `saturating-neg-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-neg-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6e65672d753332"></a>

## `saturating-neg-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-neg-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6e65672d753634"></a>

## `saturating-neg-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-neg-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6e65672d7538"></a>

## `saturating-neg-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-neg-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d6e65672d7573697a65"></a>

## `saturating-neg-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-neg-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7375622d693136"></a>

## `saturating-sub-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7375622d693332"></a>

## `saturating-sub-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7375622d693634"></a>

## `saturating-sub-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7375622d6938"></a>

## `saturating-sub-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7375622d6973697a65"></a>

## `saturating-sub-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7375622d753136"></a>

## `saturating-sub-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7375622d753332"></a>

## `saturating-sub-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7375622d753634"></a>

## `saturating-sub-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7375622d7538"></a>

## `saturating-sub-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7375622d7573697a65"></a>

## `saturating-sub-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7531362d746f2d693136"></a>

## `saturating-u16-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u16-to-i16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7531362d746f2d693332"></a>

## `saturating-u16-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u16-to-i32</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7531362d746f2d693634"></a>

## `saturating-u16-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u16-to-i64</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7531362d746f2d6938"></a>

## `saturating-u16-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u16-to-i8</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7531362d746f2d6973697a65"></a>

## `saturating-u16-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u16-to-isize</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7531362d746f2d753136"></a>

## `saturating-u16-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u16-to-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7531362d746f2d753332"></a>

## `saturating-u16-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u16-to-u32</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7531362d746f2d753634"></a>

## `saturating-u16-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u16-to-u64</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7531362d746f2d7538"></a>

## `saturating-u16-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u16-to-u8</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7531362d746f2d7573697a65"></a>

## `saturating-u16-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u16-to-usize</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7533322d746f2d693136"></a>

## `saturating-u32-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u32-to-i16</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7533322d746f2d693332"></a>

## `saturating-u32-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u32-to-i32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7533322d746f2d693634"></a>

## `saturating-u32-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u32-to-i64</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7533322d746f2d6938"></a>

## `saturating-u32-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u32-to-i8</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7533322d746f2d6973697a65"></a>

## `saturating-u32-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u32-to-isize</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7533322d746f2d753136"></a>

## `saturating-u32-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u32-to-u16</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7533322d746f2d753332"></a>

## `saturating-u32-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u32-to-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7533322d746f2d753634"></a>

## `saturating-u32-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u32-to-u64</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7533322d746f2d7538"></a>

## `saturating-u32-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u32-to-u8</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7533322d746f2d7573697a65"></a>

## `saturating-u32-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u32-to-usize</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7536342d746f2d693136"></a>

## `saturating-u64-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u64-to-i16</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7536342d746f2d693332"></a>

## `saturating-u64-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u64-to-i32</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7536342d746f2d693634"></a>

## `saturating-u64-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u64-to-i64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7536342d746f2d6938"></a>

## `saturating-u64-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u64-to-i8</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7536342d746f2d6973697a65"></a>

## `saturating-u64-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u64-to-isize</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7536342d746f2d753136"></a>

## `saturating-u64-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u64-to-u16</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7536342d746f2d753332"></a>

## `saturating-u64-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u64-to-u32</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7536342d746f2d753634"></a>

## `saturating-u64-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u64-to-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7536342d746f2d7538"></a>

## `saturating-u64-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u64-to-u8</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7536342d746f2d7573697a65"></a>

## `saturating-u64-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u64-to-usize</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d75382d746f2d693136"></a>

## `saturating-u8-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u8-to-i16</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d75382d746f2d693332"></a>

## `saturating-u8-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u8-to-i32</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d75382d746f2d693634"></a>

## `saturating-u8-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u8-to-i64</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d75382d746f2d6938"></a>

## `saturating-u8-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u8-to-i8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d75382d746f2d6973697a65"></a>

## `saturating-u8-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u8-to-isize</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d75382d746f2d753136"></a>

## `saturating-u8-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u8-to-u16</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d75382d746f2d753332"></a>

## `saturating-u8-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u8-to-u32</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d75382d746f2d753634"></a>

## `saturating-u8-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u8-to-u64</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d75382d746f2d7538"></a>

## `saturating-u8-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u8-to-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d75382d746f2d7573697a65"></a>

## `saturating-u8-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-u8-to-usize</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7573697a652d746f2d693136"></a>

## `saturating-usize-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-usize-to-i16</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7573697a652d746f2d693332"></a>

## `saturating-usize-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-usize-to-i32</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7573697a652d746f2d693634"></a>

## `saturating-usize-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-usize-to-i64</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7573697a652d746f2d6938"></a>

## `saturating-usize-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-usize-to-i8</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7573697a652d746f2d6973697a65"></a>

## `saturating-usize-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-usize-to-isize</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7573697a652d746f2d753136"></a>

## `saturating-usize-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-usize-to-u16</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7573697a652d746f2d753332"></a>

## `saturating-usize-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-usize-to-u32</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7573697a652d746f2d753634"></a>

## `saturating-usize-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-usize-to-u64</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7573697a652d746f2d7538"></a>

## `saturating-usize-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-usize-to-u8</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-73617475726174696e672d7573697a652d746f2d7573697a65"></a>

## `saturating-usize-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-usize-to-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7369676e2d6269742d663332"></a>

## `sign-bit-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sign-bit-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7369676e2d6269742d663634"></a>

## `sign-bit-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sign-bit-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-737172742d663332"></a>

## `sqrt-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sqrt-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-737172742d663634"></a>

## `sqrt-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sqrt-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7375622d663332"></a>

## `sub-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sub-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">F32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7375622d663634"></a>

## `sub-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sub-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-737761702d62797465732d693136"></a>

## `swap-bytes-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">swap-bytes-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-737761702d62797465732d693332"></a>

## `swap-bytes-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">swap-bytes-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-737761702d62797465732d693634"></a>

## `swap-bytes-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">swap-bytes-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-737761702d62797465732d6938"></a>

## `swap-bytes-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">swap-bytes-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-737761702d62797465732d6973697a65"></a>

## `swap-bytes-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">swap-bytes-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-737761702d62797465732d753136"></a>

## `swap-bytes-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">swap-bytes-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-737761702d62797465732d753332"></a>

## `swap-bytes-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">swap-bytes-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-737761702d62797465732d753634"></a>

## `swap-bytes-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">swap-bytes-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-737761702d62797465732d7538"></a>

## `swap-bytes-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">swap-bytes-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-737761702d62797465732d7573697a65"></a>

## `swap-bytes-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">swap-bytes-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-746f74616c2d6f726465722d663332"></a>

## `total-order-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">total-order-f32</span> :: (<span class="tok-constructor">F32</span> -&gt; (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Ordering</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-746f74616c2d6f726465722d663634"></a>

## `total-order-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">total-order-f64</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Ordering</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-747261696c696e672d7a65726f732d693136"></a>

## `trailing-zeros-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-747261696c696e672d7a65726f732d693332"></a>

## `trailing-zeros-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-747261696c696e672d7a65726f732d693634"></a>

## `trailing-zeros-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-747261696c696e672d7a65726f732d6938"></a>

## `trailing-zeros-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-747261696c696e672d7a65726f732d6973697a65"></a>

## `trailing-zeros-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-747261696c696e672d7a65726f732d753136"></a>

## `trailing-zeros-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-747261696c696e672d7a65726f732d753332"></a>

## `trailing-zeros-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-747261696c696e672d7a65726f732d753634"></a>

## `trailing-zeros-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-747261696c696e672d7a65726f732d7538"></a>

## `trailing-zeros-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-747261696c696e672d7a65726f732d7573697a65"></a>

## `trailing-zeros-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7531362d746f2d663332"></a>

## `u16-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-f32</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7531362d746f2d663634"></a>

## `u16-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-f64</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7531362d746f2d693332"></a>

## `u16-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-i32</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U16</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">65535</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7531362d746f2d693634"></a>

## `u16-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-i64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U16</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">65535</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7531362d746f2d6973697a65"></a>

## `u16-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-isize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U16</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">65535</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7531362d746f2d6e6174"></a>

## `u16-to-nat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-nat</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">65535</span> <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7531362d746f2d753136"></a>

## `u16-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7531362d746f2d753332"></a>

## `u16-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-u32</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U16</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">65535</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7531362d746f2d753634"></a>

## `u16-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-u64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U16</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">65535</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7531362d746f2d7573697a65"></a>

## `u16-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-usize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U16</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">65535</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7533322d746f2d663332"></a>

## `u32-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u32-to-f32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7533322d746f2d663634"></a>

## `u32-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u32-to-f64</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7533322d746f2d693634"></a>

## `u32-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u32-to-i64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U32</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">4294967295</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7533322d746f2d6973697a65"></a>

## `u32-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u32-to-isize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U32</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">4294967295</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7533322d746f2d6e6174"></a>

## `u32-to-nat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u32-to-nat</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">4294967295</span> <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7533322d746f2d753332"></a>

## `u32-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u32-to-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7533322d746f2d753634"></a>

## `u32-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u32-to-u64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U32</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">4294967295</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7533322d746f2d7573697a65"></a>

## `u32-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u32-to-usize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U32</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">4294967295</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7536342d746f2d663332"></a>

## `u64-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u64-to-f32</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7536342d746f2d663634"></a>

## `u64-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u64-to-f64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7536342d746f2d753634"></a>

## `u64-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u64-to-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7536342d746f2d7573697a65"></a>

## `u64-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u64-to-usize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U64</span>) -&gt; <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>) <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-75382d746f2d663332"></a>

## `u8-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-f32</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-75382d746f2d663634"></a>

## `u8-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-f64</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-75382d746f2d693136"></a>

## `u8-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-i16</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">255</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-75382d746f2d693332"></a>

## `u8-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-i32</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">255</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-75382d746f2d693634"></a>

## `u8-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-i64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">255</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-75382d746f2d6973697a65"></a>

## `u8-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-isize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">255</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-75382d746f2d6e6174"></a>

## `u8-to-nat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-nat</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">255</span> <span class="tok-constructor">Nat</span>)</code></pre>

The unsigned machine integers as the `Nat` of the same value: the
widening conversion's range is the evidence `nat-from-i64` needs,
and the result keeps it.

[Source](/reference-sources/Data.Machine.cinch) · line 1199

<a id="value-75382d746f2d753136"></a>

## `u8-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-u16</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">255</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-75382d746f2d753332"></a>

## `u8-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-u32</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">255</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-75382d746f2d753634"></a>

## `u8-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-u64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">255</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-75382d746f2d7538"></a>

## `u8-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-75382d746f2d7573697a65"></a>

## `u8-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-usize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">255</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7573697a652d746f2d663332"></a>

## `usize-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">usize-to-f32</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7573697a652d746f2d663634"></a>

## `usize-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">usize-to-f64</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7573697a652d746f2d753634"></a>

## `usize-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">usize-to-u64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">Usize</span>) -&gt; <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>) <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7573697a652d746f2d7573697a65"></a>

## `usize-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">usize-to-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6164642d693136"></a>

## `wrapping-add-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6164642d693332"></a>

## `wrapping-add-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6164642d693634"></a>

## `wrapping-add-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6164642d6938"></a>

## `wrapping-add-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6164642d6973697a65"></a>

## `wrapping-add-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6164642d753136"></a>

## `wrapping-add-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6164642d753332"></a>

## `wrapping-add-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6164642d753634"></a>

## `wrapping-add-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6164642d7538"></a>

## `wrapping-add-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6164642d7573697a65"></a>

## `wrapping-add-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6931362d746f2d693136"></a>

## `wrapping-i16-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i16-to-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6931362d746f2d693332"></a>

## `wrapping-i16-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i16-to-i32</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6931362d746f2d693634"></a>

## `wrapping-i16-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i16-to-i64</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6931362d746f2d6938"></a>

## `wrapping-i16-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i16-to-i8</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6931362d746f2d6973697a65"></a>

## `wrapping-i16-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i16-to-isize</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6931362d746f2d753136"></a>

## `wrapping-i16-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i16-to-u16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6931362d746f2d753332"></a>

## `wrapping-i16-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i16-to-u32</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6931362d746f2d753634"></a>

## `wrapping-i16-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i16-to-u64</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6931362d746f2d7538"></a>

## `wrapping-i16-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i16-to-u8</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6931362d746f2d7573697a65"></a>

## `wrapping-i16-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i16-to-usize</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6933322d746f2d693136"></a>

## `wrapping-i32-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i32-to-i16</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6933322d746f2d693332"></a>

## `wrapping-i32-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i32-to-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6933322d746f2d693634"></a>

## `wrapping-i32-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i32-to-i64</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6933322d746f2d6938"></a>

## `wrapping-i32-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i32-to-i8</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6933322d746f2d6973697a65"></a>

## `wrapping-i32-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i32-to-isize</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6933322d746f2d753136"></a>

## `wrapping-i32-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i32-to-u16</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6933322d746f2d753332"></a>

## `wrapping-i32-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i32-to-u32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6933322d746f2d753634"></a>

## `wrapping-i32-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i32-to-u64</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6933322d746f2d7538"></a>

## `wrapping-i32-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i32-to-u8</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6933322d746f2d7573697a65"></a>

## `wrapping-i32-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i32-to-usize</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6936342d746f2d693136"></a>

## `wrapping-i64-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-i16</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6936342d746f2d693332"></a>

## `wrapping-i64-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-i32</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6936342d746f2d693634"></a>

## `wrapping-i64-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6936342d746f2d6938"></a>

## `wrapping-i64-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-i8</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6936342d746f2d6973697a65"></a>

## `wrapping-i64-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-isize</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6936342d746f2d753136"></a>

## `wrapping-i64-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-u16</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6936342d746f2d753332"></a>

## `wrapping-i64-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-u32</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6936342d746f2d753634"></a>

## `wrapping-i64-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-u64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6936342d746f2d7538"></a>

## `wrapping-i64-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-u8</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6936342d746f2d7573697a65"></a>

## `wrapping-i64-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-usize</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d69382d746f2d693136"></a>

## `wrapping-i8-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i8-to-i16</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d69382d746f2d693332"></a>

## `wrapping-i8-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i8-to-i32</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d69382d746f2d693634"></a>

## `wrapping-i8-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i8-to-i64</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d69382d746f2d6938"></a>

## `wrapping-i8-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i8-to-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d69382d746f2d6973697a65"></a>

## `wrapping-i8-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i8-to-isize</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d69382d746f2d753136"></a>

## `wrapping-i8-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i8-to-u16</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d69382d746f2d753332"></a>

## `wrapping-i8-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i8-to-u32</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d69382d746f2d753634"></a>

## `wrapping-i8-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i8-to-u64</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d69382d746f2d7538"></a>

## `wrapping-i8-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i8-to-u8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d69382d746f2d7573697a65"></a>

## `wrapping-i8-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i8-to-usize</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6973697a652d746f2d693136"></a>

## `wrapping-isize-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-isize-to-i16</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6973697a652d746f2d693332"></a>

## `wrapping-isize-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-isize-to-i32</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6973697a652d746f2d693634"></a>

## `wrapping-isize-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-isize-to-i64</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6973697a652d746f2d6938"></a>

## `wrapping-isize-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-isize-to-i8</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6973697a652d746f2d6973697a65"></a>

## `wrapping-isize-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-isize-to-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6973697a652d746f2d753136"></a>

## `wrapping-isize-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-isize-to-u16</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6973697a652d746f2d753332"></a>

## `wrapping-isize-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-isize-to-u32</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6973697a652d746f2d753634"></a>

## `wrapping-isize-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-isize-to-u64</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6973697a652d746f2d7538"></a>

## `wrapping-isize-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-isize-to-u8</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6973697a652d746f2d7573697a65"></a>

## `wrapping-isize-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-isize-to-usize</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6d756c2d693136"></a>

## `wrapping-mul-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6d756c2d693332"></a>

## `wrapping-mul-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6d756c2d693634"></a>

## `wrapping-mul-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6d756c2d6938"></a>

## `wrapping-mul-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6d756c2d6973697a65"></a>

## `wrapping-mul-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6d756c2d753136"></a>

## `wrapping-mul-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6d756c2d753332"></a>

## `wrapping-mul-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6d756c2d753634"></a>

## `wrapping-mul-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6d756c2d7538"></a>

## `wrapping-mul-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6d756c2d7573697a65"></a>

## `wrapping-mul-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6e65672d693136"></a>

## `wrapping-neg-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6e65672d693332"></a>

## `wrapping-neg-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6e65672d693634"></a>

## `wrapping-neg-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6e65672d6938"></a>

## `wrapping-neg-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6e65672d6973697a65"></a>

## `wrapping-neg-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6e65672d753136"></a>

## `wrapping-neg-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6e65672d753332"></a>

## `wrapping-neg-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6e65672d753634"></a>

## `wrapping-neg-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6e65672d7538"></a>

## `wrapping-neg-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d6e65672d7573697a65"></a>

## `wrapping-neg-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d73686c2d693136"></a>

## `wrapping-shl-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d73686c2d693332"></a>

## `wrapping-shl-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d73686c2d693634"></a>

## `wrapping-shl-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d73686c2d6938"></a>

## `wrapping-shl-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d73686c2d6973697a65"></a>

## `wrapping-shl-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d73686c2d753136"></a>

## `wrapping-shl-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d73686c2d753332"></a>

## `wrapping-shl-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d73686c2d753634"></a>

## `wrapping-shl-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d73686c2d7538"></a>

## `wrapping-shl-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d73686c2d7573697a65"></a>

## `wrapping-shl-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7368722d693136"></a>

## `wrapping-shr-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7368722d693332"></a>

## `wrapping-shr-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7368722d693634"></a>

## `wrapping-shr-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7368722d6938"></a>

## `wrapping-shr-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7368722d6973697a65"></a>

## `wrapping-shr-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7368722d753136"></a>

## `wrapping-shr-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7368722d753332"></a>

## `wrapping-shr-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7368722d753634"></a>

## `wrapping-shr-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7368722d7538"></a>

## `wrapping-shr-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7368722d7573697a65"></a>

## `wrapping-shr-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7375622d693136"></a>

## `wrapping-sub-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7375622d693332"></a>

## `wrapping-sub-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7375622d693634"></a>

## `wrapping-sub-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7375622d6938"></a>

## `wrapping-sub-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7375622d6973697a65"></a>

## `wrapping-sub-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-isize</span> :: (<span class="tok-constructor">Isize</span> -&gt; (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Isize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7375622d753136"></a>

## `wrapping-sub-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7375622d753332"></a>

## `wrapping-sub-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7375622d753634"></a>

## `wrapping-sub-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7375622d7538"></a>

## `wrapping-sub-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7375622d7573697a65"></a>

## `wrapping-sub-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Usize</span>))</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7531362d746f2d693136"></a>

## `wrapping-u16-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u16-to-i16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7531362d746f2d693332"></a>

## `wrapping-u16-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u16-to-i32</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7531362d746f2d693634"></a>

## `wrapping-u16-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u16-to-i64</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7531362d746f2d6938"></a>

## `wrapping-u16-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u16-to-i8</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7531362d746f2d6973697a65"></a>

## `wrapping-u16-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u16-to-isize</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7531362d746f2d753136"></a>

## `wrapping-u16-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u16-to-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7531362d746f2d753332"></a>

## `wrapping-u16-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u16-to-u32</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7531362d746f2d753634"></a>

## `wrapping-u16-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u16-to-u64</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7531362d746f2d7538"></a>

## `wrapping-u16-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u16-to-u8</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7531362d746f2d7573697a65"></a>

## `wrapping-u16-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u16-to-usize</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7533322d746f2d693136"></a>

## `wrapping-u32-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u32-to-i16</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7533322d746f2d693332"></a>

## `wrapping-u32-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u32-to-i32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7533322d746f2d693634"></a>

## `wrapping-u32-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u32-to-i64</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7533322d746f2d6938"></a>

## `wrapping-u32-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u32-to-i8</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7533322d746f2d6973697a65"></a>

## `wrapping-u32-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u32-to-isize</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7533322d746f2d753136"></a>

## `wrapping-u32-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u32-to-u16</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7533322d746f2d753332"></a>

## `wrapping-u32-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u32-to-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7533322d746f2d753634"></a>

## `wrapping-u32-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u32-to-u64</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7533322d746f2d7538"></a>

## `wrapping-u32-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u32-to-u8</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7533322d746f2d7573697a65"></a>

## `wrapping-u32-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u32-to-usize</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7536342d746f2d693136"></a>

## `wrapping-u64-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-i16</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7536342d746f2d693332"></a>

## `wrapping-u64-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-i32</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7536342d746f2d693634"></a>

## `wrapping-u64-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-i64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7536342d746f2d6938"></a>

## `wrapping-u64-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-i8</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7536342d746f2d6973697a65"></a>

## `wrapping-u64-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-isize</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7536342d746f2d753136"></a>

## `wrapping-u64-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-u16</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7536342d746f2d753332"></a>

## `wrapping-u64-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-u32</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7536342d746f2d753634"></a>

## `wrapping-u64-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7536342d746f2d7538"></a>

## `wrapping-u64-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-u8</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7536342d746f2d7573697a65"></a>

## `wrapping-u64-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-usize</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d75382d746f2d693136"></a>

## `wrapping-u8-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u8-to-i16</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d75382d746f2d693332"></a>

## `wrapping-u8-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u8-to-i32</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d75382d746f2d693634"></a>

## `wrapping-u8-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u8-to-i64</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d75382d746f2d6938"></a>

## `wrapping-u8-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u8-to-i8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d75382d746f2d6973697a65"></a>

## `wrapping-u8-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u8-to-isize</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d75382d746f2d753136"></a>

## `wrapping-u8-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u8-to-u16</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d75382d746f2d753332"></a>

## `wrapping-u8-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u8-to-u32</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d75382d746f2d753634"></a>

## `wrapping-u8-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u8-to-u64</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d75382d746f2d7538"></a>

## `wrapping-u8-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u8-to-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d75382d746f2d7573697a65"></a>

## `wrapping-u8-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u8-to-usize</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7573697a652d746f2d693136"></a>

## `wrapping-usize-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-usize-to-i16</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7573697a652d746f2d693332"></a>

## `wrapping-usize-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-usize-to-i32</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7573697a652d746f2d693634"></a>

## `wrapping-usize-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-usize-to-i64</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7573697a652d746f2d6938"></a>

## `wrapping-usize-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-usize-to-i8</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7573697a652d746f2d6973697a65"></a>

## `wrapping-usize-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-usize-to-isize</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7573697a652d746f2d753136"></a>

## `wrapping-usize-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-usize-to-u16</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7573697a652d746f2d753332"></a>

## `wrapping-usize-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-usize-to-u32</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7573697a652d746f2d753634"></a>

## `wrapping-usize-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-usize-to-u64</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7573697a652d746f2d7538"></a>

## `wrapping-usize-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-usize-to-u8</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7772617070696e672d7573697a652d746f2d7573697a65"></a>

## `wrapping-usize-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-usize-to-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Machine.cinch)

<a id="value-7a65726f2d753332"></a>

## `zero-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zero-u32</span> :: <span class="tok-constructor">U32</span></code></pre>

The scalar model (platform section 5): the sum modulo 2^32 over the
borrowed vector.

[Source](/reference-sources/Data.Machine.cinch) · line 1015

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Machine.Bits_I16` | `Bits I16` | Implicit |
| `Data.Machine.Bits_I32` | `Bits I32` | Implicit |
| `Data.Machine.Bits_I64` | `Bits I64` | Implicit |
| `Data.Machine.Bits_I8` | `Bits I8` | Implicit |
| `Data.Machine.Bits_Isize` | `Bits Isize` | Implicit |
| `Data.Machine.Bits_U16` | `Bits U16` | Implicit |
| `Data.Machine.Bits_U32` | `Bits U32` | Implicit |
| `Data.Machine.Bits_U64` | `Bits U64` | Implicit |
| `Data.Machine.Bits_U8` | `Bits U8` | Implicit |
| `Data.Machine.Bits_Usize` | `Bits Usize` | Implicit |
| `Data.Machine.Codec_I64` | `Codec I64` | Implicit |
| `Data.Machine.Eq_I16` | `Eq I16` | Implicit |
| `Data.Machine.Eq_I32` | `Eq I32` | Implicit |
| `Data.Machine.Eq_I64` | `Eq I64` | Implicit |
| `Data.Machine.Eq_I8` | `Eq I8` | Implicit |
| `Data.Machine.Eq_Isize` | `Eq Isize` | Implicit |
| `Data.Machine.Eq_U16` | `Eq U16` | Implicit |
| `Data.Machine.Eq_U32` | `Eq U32` | Implicit |
| `Data.Machine.Eq_U64` | `Eq U64` | Implicit |
| `Data.Machine.Eq_U8` | `Eq U8` | Implicit |
| `Data.Machine.Eq_Usize` | `Eq Usize` | Implicit |
| `Data.Machine.JsonCodec_I64` | `JsonCodec I64` | Implicit |
| `Data.Machine.Ord_I16` | `Ord I16` | Implicit |
| `Data.Machine.Ord_I32` | `Ord I32` | Implicit |
| `Data.Machine.Ord_I64` | `Ord I64` | Implicit |
| `Data.Machine.Ord_I8` | `Ord I8` | Implicit |
| `Data.Machine.Ord_Isize` | `Ord Isize` | Implicit |
| `Data.Machine.Ord_U16` | `Ord U16` | Implicit |
| `Data.Machine.Ord_U32` | `Ord U32` | Implicit |
| `Data.Machine.Ord_U64` | `Ord U64` | Implicit |
| `Data.Machine.Ord_U8` | `Ord U8` | Implicit |
| `Data.Machine.Ord_Usize` | `Ord Usize` | Implicit |

**`Data.Machine.Codec_I64`**: The A6 column codec for integers (decision D65's covered set):
8 bytes little-endian.
**`Data.Machine.JsonCodec_I64`**: The A6 JSON column codec for integers (decision D65's covered
set): `JsonCodec`'s round-trip law (`decode-json (encode-json x)`
is `Ok x`), with the named range error.

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Machine.cinch`. Source SHA-256: `c06a8e2d260d6f8bab809c4fda390c7f713b2719c46e3ca170097f0b1e2c7032`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
