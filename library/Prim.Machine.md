# Prim.Machine

Leaf inline-LLVM scalar machine primitives (fixed-width wrapping/saturating/overflow/bitwise/shift/rotate/bitscan/comparison/conversion operations and word-size bridges), re-exported through Data.Machine and shared directly by Data.Char, Data.Bytes, and Data.String.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Prim.Machine`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Prim.Machine.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Leaf machine scalar primitives over fixed-width integers, pointer-width bridges, and floats.
Every operation lowers through an explicit inline LLVM `prim` fragment so neither
compiler maintains an independent `cinch.prim.*` scalar lowering table. Re-exported
through Data.Machine and shared directly by Data.Char, Data.Bytes, and Data.String.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-6269742d616e642d693136"></a>

## `bit-and-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d616e642d693332"></a>

## `bit-and-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d616e642d693634"></a>

## `bit-and-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d616e642d6938"></a>

## `bit-and-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d616e642d753136"></a>

## `bit-and-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d616e642d753332"></a>

## `bit-and-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d616e642d753634"></a>

## `bit-and-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d616e642d7538"></a>

## `bit-and-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d6e6f742d693136"></a>

## `bit-not-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d6e6f742d693332"></a>

## `bit-not-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d6e6f742d693634"></a>

## `bit-not-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d6e6f742d6938"></a>

## `bit-not-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d6e6f742d753136"></a>

## `bit-not-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d6e6f742d753332"></a>

## `bit-not-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d6e6f742d753634"></a>

## `bit-not-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d6e6f742d7538"></a>

## `bit-not-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d6f722d693136"></a>

## `bit-or-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d6f722d693332"></a>

## `bit-or-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d6f722d693634"></a>

## `bit-or-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d6f722d6938"></a>

## `bit-or-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d6f722d753136"></a>

## `bit-or-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d6f722d753332"></a>

## `bit-or-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d6f722d753634"></a>

## `bit-or-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d6f722d7538"></a>

## `bit-or-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d786f722d693136"></a>

## `bit-xor-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d786f722d693332"></a>

## `bit-xor-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d786f722d693634"></a>

## `bit-xor-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d786f722d6938"></a>

## `bit-xor-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d786f722d753136"></a>

## `bit-xor-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d786f722d753332"></a>

## `bit-xor-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d786f722d753634"></a>

## `bit-xor-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6269742d786f722d7538"></a>

## `bit-xor-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-636f756e742d6f6e65732d693136"></a>

## `count-ones-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-636f756e742d6f6e65732d693332"></a>

## `count-ones-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-636f756e742d6f6e65732d693634"></a>

## `count-ones-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-636f756e742d6f6e65732d6938"></a>

## `count-ones-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-636f756e742d6f6e65732d753136"></a>

## `count-ones-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-636f756e742d6f6e65732d753332"></a>

## `count-ones-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-636f756e742d6f6e65732d753634"></a>

## `count-ones-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-636f756e742d6f6e65732d7538"></a>

## `count-ones-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-65712d693136"></a>

## `eq-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-65712d693332"></a>

## `eq-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-65712d693634"></a>

## `eq-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-65712d6938"></a>

## `eq-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-65712d753136"></a>

## `eq-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-65712d753332"></a>

## `eq-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-65712d753634"></a>

## `eq-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-65712d7538"></a>

## `eq-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6633322d746f2d663634"></a>

## `f32-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f32-to-f64</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6636342d666974732d693136"></a>

## `f64-fits-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f64-fits-i16</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6636342d666974732d693332"></a>

## `f64-fits-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f64-fits-i32</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6636342d666974732d693634"></a>

## `f64-fits-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f64-fits-i64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6636342d666974732d6938"></a>

## `f64-fits-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f64-fits-i8</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6636342d666974732d753136"></a>

## `f64-fits-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f64-fits-u16</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6636342d666974732d753332"></a>

## `f64-fits-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f64-fits-u32</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6636342d666974732d753634"></a>

## `f64-fits-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f64-fits-u64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6636342d666974732d7538"></a>

## `f64-fits-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f64-fits-u8</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6636342d746f2d663332"></a>

## `f64-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">f64-to-f32</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-67652d693136"></a>

## `ge-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-67652d693332"></a>

## `ge-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-67652d693634"></a>

## `ge-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-67652d6938"></a>

## `ge-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-67652d753136"></a>

## `ge-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-67652d753332"></a>

## `ge-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-67652d753634"></a>

## `ge-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-67652d7538"></a>

## `ge-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-67742d693136"></a>

## `gt-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-67742d693332"></a>

## `gt-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-67742d693634"></a>

## `gt-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-67742d6938"></a>

## `gt-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-67742d753136"></a>

## `gt-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-67742d753332"></a>

## `gt-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-67742d753634"></a>

## `gt-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-67742d7538"></a>

## `gt-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6931362d746f2d663332"></a>

## `i16-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i16-to-f32</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6931362d746f2d663634"></a>

## `i16-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i16-to-f64</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6931362d746f2d693332"></a>

## `i16-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i16-to-i32</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I16</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">-32768</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">32767</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6931362d746f2d693634"></a>

## `i16-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i16-to-i64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I16</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">-32768</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">32767</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6931362d746f2d6973697a65"></a>

## `i16-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i16-to-isize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I16</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">-32768</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">32767</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6933322d746f2d663332"></a>

## `i32-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i32-to-f32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6933322d746f2d663634"></a>

## `i32-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i32-to-f64</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6933322d746f2d693634"></a>

## `i32-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i32-to-i64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I32</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">-2147483648</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">2147483647</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6933322d746f2d6973697a65"></a>

## `i32-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i32-to-isize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I32</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">-2147483648</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">2147483647</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6936342d746f2d663332"></a>

## `i64-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i64-to-f32</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6936342d746f2d663634"></a>

## `i64-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i64-to-f64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6936342d746f2d6973697a65"></a>

## `i64-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i64-to-isize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I64</span>) -&gt; <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>) <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-69382d746f2d663332"></a>

## `i8-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i8-to-f32</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-69382d746f2d663634"></a>

## `i8-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i8-to-f64</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-69382d746f2d693136"></a>

## `i8-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i8-to-i16</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">-128</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">127</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-69382d746f2d693332"></a>

## `i8-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i8-to-i32</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">-128</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">127</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-69382d746f2d693634"></a>

## `i8-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i8-to-i64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">-128</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">127</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-69382d746f2d6973697a65"></a>

## `i8-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">i8-to-isize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">I8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">-128</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">127</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6973697a652d746f2d693634"></a>

## `isize-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">isize-to-i64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">Isize</span>) -&gt; <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>) <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c652d693136"></a>

## `le-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c652d693332"></a>

## `le-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c652d693634"></a>

## `le-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c652d6938"></a>

## `le-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c652d753136"></a>

## `le-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c652d753332"></a>

## `le-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c652d753634"></a>

## `le-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c652d7538"></a>

## `le-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c656164696e672d7a65726f732d693136"></a>

## `leading-zeros-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c656164696e672d7a65726f732d693332"></a>

## `leading-zeros-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c656164696e672d7a65726f732d693634"></a>

## `leading-zeros-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c656164696e672d7a65726f732d6938"></a>

## `leading-zeros-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c656164696e672d7a65726f732d753136"></a>

## `leading-zeros-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c656164696e672d7a65726f732d753332"></a>

## `leading-zeros-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c656164696e672d7a65726f732d753634"></a>

## `leading-zeros-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c656164696e672d7a65726f732d7538"></a>

## `leading-zeros-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c742d693136"></a>

## `lt-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c742d693332"></a>

## `lt-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c742d693634"></a>

## `lt-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c742d6938"></a>

## `lt-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c742d753136"></a>

## `lt-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c742d753332"></a>

## `lt-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c742d753634"></a>

## `lt-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6c742d7538"></a>

## `lt-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6d61736b2d693634"></a>

## `mask-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mask-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; ((<span class="tok-variable">a1</span> :: <span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">9223372036854775807</span> <span class="tok-constructor">I64</span>) -&gt; <span class="tok-variable">refined</span> ((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">a1</span>)) <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6e652d693136"></a>

## `ne-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6e652d693332"></a>

## `ne-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6e652d693634"></a>

## `ne-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6e652d6938"></a>

## `ne-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6e652d753136"></a>

## `ne-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6e652d753332"></a>

## `ne-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6e652d753634"></a>

## `ne-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6e652d7538"></a>

## `ne-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d6164642d693136"></a>

## `ov-add-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-add-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d6164642d693332"></a>

## `ov-add-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-add-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d6164642d693634"></a>

## `ov-add-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-add-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d6164642d6938"></a>

## `ov-add-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-add-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d6164642d753136"></a>

## `ov-add-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-add-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d6164642d753332"></a>

## `ov-add-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-add-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d6164642d753634"></a>

## `ov-add-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-add-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d6164642d7538"></a>

## `ov-add-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-add-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d6d756c2d693136"></a>

## `ov-mul-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-mul-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d6d756c2d693332"></a>

## `ov-mul-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-mul-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d6d756c2d693634"></a>

## `ov-mul-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-mul-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d6d756c2d6938"></a>

## `ov-mul-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-mul-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d6d756c2d753136"></a>

## `ov-mul-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-mul-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d6d756c2d753332"></a>

## `ov-mul-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-mul-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d6d756c2d753634"></a>

## `ov-mul-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-mul-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d6d756c2d7538"></a>

## `ov-mul-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-mul-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d7375622d693136"></a>

## `ov-sub-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-sub-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d7375622d693332"></a>

## `ov-sub-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-sub-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d7375622d693634"></a>

## `ov-sub-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-sub-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d7375622d6938"></a>

## `ov-sub-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-sub-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d7375622d753136"></a>

## `ov-sub-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-sub-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d7375622d753332"></a>

## `ov-sub-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-sub-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d7375622d753634"></a>

## `ov-sub-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-sub-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-6f762d7375622d7538"></a>

## `ov-sub-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ov-sub-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7261772d6469762d753136"></a>

## `raw-div-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raw-div-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7261772d6469762d753332"></a>

## `raw-div-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raw-div-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7261772d6469762d753634"></a>

## `raw-div-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raw-div-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7261772d6469762d7538"></a>

## `raw-div-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raw-div-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7261772d72656d2d753136"></a>

## `raw-rem-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raw-rem-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7261772d72656d2d753332"></a>

## `raw-rem-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raw-rem-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7261772d72656d2d753634"></a>

## `raw-rem-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raw-rem-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7261772d72656d2d7538"></a>

## `raw-rem-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raw-rem-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7261772d736469762d693136"></a>

## `raw-sdiv-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raw-sdiv-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7261772d736469762d693332"></a>

## `raw-sdiv-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raw-sdiv-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7261772d736469762d693634"></a>

## `raw-sdiv-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raw-sdiv-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7261772d736469762d6938"></a>

## `raw-sdiv-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raw-sdiv-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7261772d7372656d2d693136"></a>

## `raw-srem-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raw-srem-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7261772d7372656d2d693332"></a>

## `raw-srem-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raw-srem-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7261772d7372656d2d693634"></a>

## `raw-srem-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raw-srem-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7261772d7372656d2d6938"></a>

## `raw-srem-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raw-srem-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726576657273652d626974732d693136"></a>

## `reverse-bits-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726576657273652d626974732d693332"></a>

## `reverse-bits-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726576657273652d626974732d693634"></a>

## `reverse-bits-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726576657273652d626974732d6938"></a>

## `reverse-bits-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726576657273652d626974732d753136"></a>

## `reverse-bits-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726576657273652d626974732d753332"></a>

## `reverse-bits-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726576657273652d626974732d753634"></a>

## `reverse-bits-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726576657273652d626974732d7538"></a>

## `reverse-bits-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bits-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726f746174652d6c6566742d693136"></a>

## `rotate-left-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726f746174652d6c6566742d693332"></a>

## `rotate-left-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726f746174652d6c6566742d693634"></a>

## `rotate-left-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726f746174652d6c6566742d6938"></a>

## `rotate-left-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726f746174652d6c6566742d753136"></a>

## `rotate-left-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726f746174652d6c6566742d753332"></a>

## `rotate-left-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726f746174652d6c6566742d753634"></a>

## `rotate-left-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726f746174652d6c6566742d7538"></a>

## `rotate-left-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726f746174652d72696768742d693136"></a>

## `rotate-right-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726f746174652d72696768742d693332"></a>

## `rotate-right-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726f746174652d72696768742d693634"></a>

## `rotate-right-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726f746174652d72696768742d6938"></a>

## `rotate-right-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726f746174652d72696768742d753136"></a>

## `rotate-right-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726f746174652d72696768742d753332"></a>

## `rotate-right-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726f746174652d72696768742d753634"></a>

## `rotate-right-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-726f746174652d72696768742d7538"></a>

## `rotate-right-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6164642d693136"></a>

## `saturating-add-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6164642d693332"></a>

## `saturating-add-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6164642d693634"></a>

## `saturating-add-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6164642d6938"></a>

## `saturating-add-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6164642d753136"></a>

## `saturating-add-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6164642d753332"></a>

## `saturating-add-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6164642d753634"></a>

## `saturating-add-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6164642d7538"></a>

## `saturating-add-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-add-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6636342d746f2d693136"></a>

## `saturating-f64-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-i16</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6636342d746f2d693332"></a>

## `saturating-f64-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-i32</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6636342d746f2d693634"></a>

## `saturating-f64-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-i64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6636342d746f2d6938"></a>

## `saturating-f64-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-i8</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6636342d746f2d753136"></a>

## `saturating-f64-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-u16</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6636342d746f2d753332"></a>

## `saturating-f64-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-u32</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6636342d746f2d753634"></a>

## `saturating-f64-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-u64</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6636342d746f2d7538"></a>

## `saturating-f64-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-f64-to-u8</span> :: (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6d756c2d693136"></a>

## `saturating-mul-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6d756c2d693332"></a>

## `saturating-mul-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6d756c2d693634"></a>

## `saturating-mul-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6d756c2d6938"></a>

## `saturating-mul-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6d756c2d753136"></a>

## `saturating-mul-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6d756c2d753332"></a>

## `saturating-mul-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6d756c2d753634"></a>

## `saturating-mul-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d6d756c2d7538"></a>

## `saturating-mul-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-mul-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d7375622d693136"></a>

## `saturating-sub-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d7375622d693332"></a>

## `saturating-sub-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d7375622d693634"></a>

## `saturating-sub-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d7375622d6938"></a>

## `saturating-sub-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d7375622d753136"></a>

## `saturating-sub-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d7375622d753332"></a>

## `saturating-sub-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d7375622d753634"></a>

## `saturating-sub-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-73617475726174696e672d7375622d7538"></a>

## `saturating-sub-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">saturating-sub-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-737761702d62797465732d693136"></a>

## `swap-bytes-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">swap-bytes-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-737761702d62797465732d693332"></a>

## `swap-bytes-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">swap-bytes-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-737761702d62797465732d693634"></a>

## `swap-bytes-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">swap-bytes-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-737761702d62797465732d753136"></a>

## `swap-bytes-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">swap-bytes-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-737761702d62797465732d753332"></a>

## `swap-bytes-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">swap-bytes-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-737761702d62797465732d753634"></a>

## `swap-bytes-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">swap-bytes-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-747261696c696e672d7a65726f732d693136"></a>

## `trailing-zeros-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-747261696c696e672d7a65726f732d693332"></a>

## `trailing-zeros-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-747261696c696e672d7a65726f732d693634"></a>

## `trailing-zeros-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-747261696c696e672d7a65726f732d6938"></a>

## `trailing-zeros-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-747261696c696e672d7a65726f732d753136"></a>

## `trailing-zeros-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-747261696c696e672d7a65726f732d753332"></a>

## `trailing-zeros-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-747261696c696e672d7a65726f732d753634"></a>

## `trailing-zeros-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-747261696c696e672d7a65726f732d7538"></a>

## `trailing-zeros-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7531362d746f2d663332"></a>

## `u16-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-f32</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7531362d746f2d663634"></a>

## `u16-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-f64</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7531362d746f2d693332"></a>

## `u16-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-i32</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U16</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">65535</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7531362d746f2d693634"></a>

## `u16-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-i64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U16</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">65535</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7531362d746f2d6973697a65"></a>

## `u16-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-isize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U16</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">65535</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7531362d746f2d753332"></a>

## `u16-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-u32</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U16</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">65535</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7531362d746f2d753634"></a>

## `u16-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-u64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U16</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">65535</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7531362d746f2d7573697a65"></a>

## `u16-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u16-to-usize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U16</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">65535</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7533322d746f2d663332"></a>

## `u32-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u32-to-f32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7533322d746f2d663634"></a>

## `u32-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u32-to-f64</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7533322d746f2d693634"></a>

## `u32-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u32-to-i64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U32</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">4294967295</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7533322d746f2d6973697a65"></a>

## `u32-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u32-to-isize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U32</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">4294967295</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7533322d746f2d753634"></a>

## `u32-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u32-to-u64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U32</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">4294967295</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7533322d746f2d7573697a65"></a>

## `u32-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u32-to-usize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U32</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">4294967295</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7536342d746f2d663332"></a>

## `u64-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u64-to-f32</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7536342d746f2d663634"></a>

## `u64-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u64-to-f64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7536342d746f2d7573697a65"></a>

## `u64-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u64-to-usize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U64</span>) -&gt; <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>) <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-75382d746f2d663332"></a>

## `u8-to-f32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-f32</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-75382d746f2d663634"></a>

## `u8-to-f64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-f64</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-75382d746f2d693136"></a>

## `u8-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-i16</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">255</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-75382d746f2d693332"></a>

## `u8-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-i32</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">255</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-75382d746f2d693634"></a>

## `u8-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-i64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">255</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-75382d746f2d6973697a65"></a>

## `u8-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-isize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">255</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-75382d746f2d753136"></a>

## `u8-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-u16</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">255</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-75382d746f2d753332"></a>

## `u8-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-u32</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">255</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-75382d746f2d753634"></a>

## `u8-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-u64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">255</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-75382d746f2d7573697a65"></a>

## `u8-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u8-to-usize</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">U8</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">255</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7573697a652d746f2d753634"></a>

## `usize-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">usize-to-u64</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">Usize</span>) -&gt; <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>) <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-776f72642d73697a653332"></a>

## `word-size32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">word-size32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6164642d693136"></a>

## `wrapping-add-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6164642d693332"></a>

## `wrapping-add-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6164642d693634"></a>

## `wrapping-add-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6164642d6938"></a>

## `wrapping-add-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6164642d753136"></a>

## `wrapping-add-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6164642d753332"></a>

## `wrapping-add-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6164642d753634"></a>

## `wrapping-add-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6164642d7538"></a>

## `wrapping-add-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-add-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6931362d746f2d6938"></a>

## `wrapping-i16-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i16-to-i8</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6931362d746f2d753136"></a>

## `wrapping-i16-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i16-to-u16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6931362d746f2d753332"></a>

## `wrapping-i16-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i16-to-u32</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6931362d746f2d753634"></a>

## `wrapping-i16-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i16-to-u64</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6931362d746f2d7538"></a>

## `wrapping-i16-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i16-to-u8</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6933322d746f2d693136"></a>

## `wrapping-i32-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i32-to-i16</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6933322d746f2d6938"></a>

## `wrapping-i32-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i32-to-i8</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6933322d746f2d753136"></a>

## `wrapping-i32-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i32-to-u16</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6933322d746f2d753332"></a>

## `wrapping-i32-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i32-to-u32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6933322d746f2d753634"></a>

## `wrapping-i32-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i32-to-u64</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6933322d746f2d7538"></a>

## `wrapping-i32-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i32-to-u8</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6936342d746f2d693136"></a>

## `wrapping-i64-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-i16</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6936342d746f2d693332"></a>

## `wrapping-i64-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-i32</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6936342d746f2d6938"></a>

## `wrapping-i64-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-i8</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6936342d746f2d6973697a65"></a>

## `wrapping-i64-to-isize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-isize</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6936342d746f2d753136"></a>

## `wrapping-i64-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-u16</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6936342d746f2d753332"></a>

## `wrapping-i64-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-u32</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6936342d746f2d753634"></a>

## `wrapping-i64-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-u64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6936342d746f2d7538"></a>

## `wrapping-i64-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i64-to-u8</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d69382d746f2d753136"></a>

## `wrapping-i8-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i8-to-u16</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d69382d746f2d753332"></a>

## `wrapping-i8-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i8-to-u32</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d69382d746f2d753634"></a>

## `wrapping-i8-to-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i8-to-u64</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d69382d746f2d7538"></a>

## `wrapping-i8-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-i8-to-u8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6973697a652d746f2d693332"></a>

## `wrapping-isize-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-isize-to-i32</span> :: (<span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6d756c2d693136"></a>

## `wrapping-mul-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6d756c2d693332"></a>

## `wrapping-mul-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6d756c2d693634"></a>

## `wrapping-mul-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6d756c2d6938"></a>

## `wrapping-mul-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6d756c2d753136"></a>

## `wrapping-mul-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6d756c2d753332"></a>

## `wrapping-mul-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6d756c2d753634"></a>

## `wrapping-mul-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6d756c2d7538"></a>

## `wrapping-mul-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-mul-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6e65672d693136"></a>

## `wrapping-neg-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6e65672d693332"></a>

## `wrapping-neg-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6e65672d693634"></a>

## `wrapping-neg-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6e65672d6938"></a>

## `wrapping-neg-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6e65672d753136"></a>

## `wrapping-neg-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6e65672d753332"></a>

## `wrapping-neg-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6e65672d753634"></a>

## `wrapping-neg-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d6e65672d7538"></a>

## `wrapping-neg-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-neg-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d73686c2d693136"></a>

## `wrapping-shl-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d73686c2d693332"></a>

## `wrapping-shl-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d73686c2d693634"></a>

## `wrapping-shl-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d73686c2d6938"></a>

## `wrapping-shl-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d73686c2d753136"></a>

## `wrapping-shl-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d73686c2d753332"></a>

## `wrapping-shl-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d73686c2d753634"></a>

## `wrapping-shl-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d73686c2d7538"></a>

## `wrapping-shl-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shl-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7368722d693136"></a>

## `wrapping-shr-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7368722d693332"></a>

## `wrapping-shr-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7368722d693634"></a>

## `wrapping-shr-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7368722d6938"></a>

## `wrapping-shr-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7368722d753136"></a>

## `wrapping-shr-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7368722d753332"></a>

## `wrapping-shr-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7368722d753634"></a>

## `wrapping-shr-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7368722d7538"></a>

## `wrapping-shr-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-shr-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7375622d693136"></a>

## `wrapping-sub-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-i16</span> :: (<span class="tok-constructor">I16</span> -&gt; (<span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">I16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7375622d693332"></a>

## `wrapping-sub-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-i32</span> :: (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">I32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7375622d693634"></a>

## `wrapping-sub-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-i64</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7375622d6938"></a>

## `wrapping-sub-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-i8</span> :: (<span class="tok-constructor">I8</span> -&gt; (<span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">I8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7375622d753136"></a>

## `wrapping-sub-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7375622d753332"></a>

## `wrapping-sub-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7375622d753634"></a>

## `wrapping-sub-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7375622d7538"></a>

## `wrapping-sub-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-sub-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7531362d746f2d693136"></a>

## `wrapping-u16-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u16-to-i16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7531362d746f2d6938"></a>

## `wrapping-u16-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u16-to-i8</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7531362d746f2d7538"></a>

## `wrapping-u16-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u16-to-u8</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7533322d746f2d693136"></a>

## `wrapping-u32-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u32-to-i16</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7533322d746f2d693332"></a>

## `wrapping-u32-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u32-to-i32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7533322d746f2d6938"></a>

## `wrapping-u32-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u32-to-i8</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7533322d746f2d753136"></a>

## `wrapping-u32-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u32-to-u16</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7533322d746f2d7538"></a>

## `wrapping-u32-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u32-to-u8</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7536342d746f2d693136"></a>

## `wrapping-u64-to-i16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-i16</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7536342d746f2d693332"></a>

## `wrapping-u64-to-i32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-i32</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7536342d746f2d693634"></a>

## `wrapping-u64-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-i64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7536342d746f2d6938"></a>

## `wrapping-u64-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-i8</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7536342d746f2d753136"></a>

## `wrapping-u64-to-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-u16</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7536342d746f2d753332"></a>

## `wrapping-u64-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-u32</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7536342d746f2d7538"></a>

## `wrapping-u64-to-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-u8</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7536342d746f2d7573697a65"></a>

## `wrapping-u64-to-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u64-to-usize</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d75382d746f2d6938"></a>

## `wrapping-u8-to-i8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-u8-to-i8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

<a id="value-7772617070696e672d7573697a652d746f2d753332"></a>

## `wrapping-usize-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">wrapping-usize-to-u32</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Prim.Machine.cinch)

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Prim/Machine.cinch`. Source SHA-256: `3f9376fc67e58890b4d2e62fe992055f5b4461b31fa96427e3cbb7badcad85d2`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
