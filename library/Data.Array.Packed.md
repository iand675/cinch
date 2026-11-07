# Data.Array.Packed

Packed scalar constructors for Array. The element format travels with the buffer, so generic reads and updates keep it.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Array.Packed`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Array.Packed.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Array](Data.Array.md), [Data.Core](Data.Core.md).

Non-embedded source closure: [Data.Array](Data.Array.md), [Data.Array.Packed](Data.Array.Packed.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Packed scalar constructors. Result is an ordinary Array: the element
format travels with the buffer, so generic reads and updates preserve it.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-61727261792d66726f6d2d6368617273"></a>

## `array-from-chars`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-from-chars</span> :: (<span class="tok-constructor">List</span> <span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Array</span> <span class="tok-constructor">Char</span>)</code></pre>

[Source](/reference-sources/Data.Array.Packed.cinch)

<a id="value-61727261792d66726f6d2d66333273"></a>

## `array-from-f32s`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-from-f32s</span> :: (<span class="tok-constructor">List</span> <span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Array</span> <span class="tok-constructor">F32</span>)</code></pre>

[Source](/reference-sources/Data.Array.Packed.cinch)

<a id="value-61727261792d66726f6d2d66363473"></a>

## `array-from-f64s`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-from-f64s</span> :: (<span class="tok-constructor">List</span> <span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Array</span> <span class="tok-constructor">F64</span>)</code></pre>

[Source](/reference-sources/Data.Array.Packed.cinch)

<a id="value-61727261792d66726f6d2d69313673"></a>

## `array-from-i16s`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-from-i16s</span> :: (<span class="tok-constructor">List</span> <span class="tok-constructor">I16</span> -&gt; <span class="tok-constructor">Array</span> <span class="tok-constructor">I16</span>)</code></pre>

[Source](/reference-sources/Data.Array.Packed.cinch)

<a id="value-61727261792d66726f6d2d69333273"></a>

## `array-from-i32s`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-from-i32s</span> :: (<span class="tok-constructor">List</span> <span class="tok-constructor">I32</span> -&gt; <span class="tok-constructor">Array</span> <span class="tok-constructor">I32</span>)</code></pre>

[Source](/reference-sources/Data.Array.Packed.cinch)

<a id="value-61727261792d66726f6d2d69363473"></a>

## `array-from-i64s`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-from-i64s</span> :: (<span class="tok-constructor">List</span> <span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Array</span> <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Array.Packed.cinch)

<a id="value-61727261792d66726f6d2d693873"></a>

## `array-from-i8s`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-from-i8s</span> :: (<span class="tok-constructor">List</span> <span class="tok-constructor">I8</span> -&gt; <span class="tok-constructor">Array</span> <span class="tok-constructor">I8</span>)</code></pre>

[Source](/reference-sources/Data.Array.Packed.cinch)

<a id="value-61727261792d66726f6d2d6973697a6573"></a>

## `array-from-isizes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-from-isizes</span> :: (<span class="tok-constructor">List</span> <span class="tok-constructor">Isize</span> -&gt; <span class="tok-constructor">Array</span> <span class="tok-constructor">Isize</span>)</code></pre>

[Source](/reference-sources/Data.Array.Packed.cinch)

<a id="value-61727261792d66726f6d2d75313673"></a>

## `array-from-u16s`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-from-u16s</span> :: (<span class="tok-constructor">List</span> <span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Array</span> <span class="tok-constructor">U16</span>)</code></pre>

[Source](/reference-sources/Data.Array.Packed.cinch)

<a id="value-61727261792d66726f6d2d75333273"></a>

## `array-from-u32s`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-from-u32s</span> :: (<span class="tok-constructor">List</span> <span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Array</span> <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Array.Packed.cinch)

<a id="value-61727261792d66726f6d2d75363473"></a>

## `array-from-u64s`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-from-u64s</span> :: (<span class="tok-constructor">List</span> <span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Array</span> <span class="tok-constructor">U64</span>)</code></pre>

[Source](/reference-sources/Data.Array.Packed.cinch)

<a id="value-61727261792d66726f6d2d753873"></a>

## `array-from-u8s`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-from-u8s</span> :: (<span class="tok-constructor">List</span> <span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Array</span> <span class="tok-constructor">U8</span>)</code></pre>

[Source](/reference-sources/Data.Array.Packed.cinch)

<a id="value-61727261792d66726f6d2d7573697a6573"></a>

## `array-from-usizes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-from-usizes</span> :: (<span class="tok-constructor">List</span> <span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Array</span> <span class="tok-constructor">Usize</span>)</code></pre>

[Source](/reference-sources/Data.Array.Packed.cinch)

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Array/Packed.cinch`. Source SHA-256: `799f936d2712685f6ade99dd777c79c7c039c1e7ca28b8988700fde0f104b364`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
