# Data.Word

Bit lengths, byte reversal, signed rotations and checked carry/borrow, wide multiply, divide and remainder operations over fixed and native-width words.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Word`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Word.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Machine](Data.Machine.md), [Data.Result](Data.Result.md), [Data.Wide](Data.Wide.md).

Non-embedded source closure: [Data.Word](Data.Word.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Fixed-width word utilities and checked double-width unsigned arithmetic.
Carry and borrow values use Bool. Wide products return high then low words;
division returns quotient then remainder and names zero-divisor/overflow errors.
Bit and byte reversal are supplied by Data.Machine. Wide arithmetic uses
Data.Machine and Data.Wide with exact double-width intermediates. Native-width
operations follow Usize's target width.

</details>

# Types and constructors

<a id="type-576f72644469766973696f6e4572726f72"></a>

## `WordDivisionError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">WordDivisionError</span></code></pre>

An undefined unsigned division is an explicit result, never a machine trap.

[Source](/reference-sources/Data.Word.cinch) · line 27

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">WordDivideByZero</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Word</span>.<span class="tok-constructor">WordDivisionError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">WordQuotientOverflow</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Word</span>.<span class="tok-constructor">WordDivisionError</span></code></pre>

# Values

<a id="value-6164642d63617272792d753332"></a>

## `add-carry-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">add-carry-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">U32</span>, <span class="tok-constructor">Bool</span>))))</code></pre>

Add two 32-bit words and a carry, returning the low word and the outgoing carry.

[Source](/reference-sources/Data.Word.cinch) · line 99

<a id="value-6164642d63617272792d753634"></a>

## `add-carry-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">add-carry-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">U64</span>, <span class="tok-constructor">Bool</span>))))</code></pre>

Add two 64-bit words and a carry, returning the low word and the outgoing carry.

[Source](/reference-sources/Data.Word.cinch) · line 106

<a id="value-6164642d63617272792d7573697a65"></a>

## `add-carry-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">add-carry-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Usize</span>, <span class="tok-constructor">Bool</span>))))</code></pre>

Add native words and a carry, returning the low word and outgoing carry.

[Source](/reference-sources/Data.Word.cinch) · line 205

<a id="value-6269742d6c656e6774682d753136"></a>

## `bit-length-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-length-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

The significant bit count of a U16; zero has length zero.

[Source](/reference-sources/Data.Word.cinch) · line 47

<a id="value-6269742d6c656e6774682d753332"></a>

## `bit-length-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-length-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

The significant bit count of a U32; zero has length zero.

[Source](/reference-sources/Data.Word.cinch) · line 60

<a id="value-6269742d6c656e6774682d753634"></a>

## `bit-length-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-length-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

The significant bit count of a U64; zero has length zero.

[Source](/reference-sources/Data.Word.cinch) · line 73

<a id="value-6269742d6c656e6774682d7538"></a>

## `bit-length-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-length-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

The significant bit count of a U8; zero has length zero.

[Source](/reference-sources/Data.Word.cinch) · line 38

<a id="value-6269742d6c656e6774682d7573697a65"></a>

## `bit-length-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-length-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

The significant bit count of a Usize; zero has length zero.

[Source](/reference-sources/Data.Word.cinch) · line 86

<a id="value-6469762d776964652d70726f76656e2d753332"></a>

## `div-wide-proven-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">div-wide-proven-u32</span> :: <span class="tok-keyword">using</span> (<span class="tok-variable">arg2</span> <span class="tok-operator">/=</span> <span class="tok-number">0</span>, <span class="tok-variable">arg0</span> <span class="tok-operator">&lt;</span> <span class="tok-variable">arg2</span>) =&gt; ((<span class="tok-variable">arg0</span> :: <span class="tok-constructor">U32</span>) -&gt; ((<span class="tok-variable">arg1</span> :: <span class="tok-constructor">U32</span>) -&gt; ((<span class="tok-variable">arg2</span> :: <span class="tok-constructor">U32</span>) -&gt; (<span class="tok-constructor">U32</span>, <span class="tok-constructor">U32</span>))))</code></pre>

Divide a double-width dividend when the caller proves a nonzero divisor
and a fitting quotient. The tuple result avoids allocating an error sum.

[Source](/reference-sources/Data.Word.cinch) · line 176

<a id="value-6469762d776964652d70726f76656e2d753634"></a>

## `div-wide-proven-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">div-wide-proven-u64</span> :: <span class="tok-keyword">using</span> (<span class="tok-variable">arg2</span> <span class="tok-operator">/=</span> <span class="tok-number">0</span>, <span class="tok-variable">arg0</span> <span class="tok-operator">&lt;</span> <span class="tok-variable">arg2</span>) =&gt; ((<span class="tok-variable">arg0</span> :: <span class="tok-constructor">U64</span>) -&gt; ((<span class="tok-variable">arg1</span> :: <span class="tok-constructor">U64</span>) -&gt; ((<span class="tok-variable">arg2</span> :: <span class="tok-constructor">U64</span>) -&gt; (<span class="tok-constructor">U64</span>, <span class="tok-constructor">U64</span>))))</code></pre>

Divide a double-width dividend under nonzero-divisor and quotient-fit proofs.
The tuple result permits unboxed worker returns for already validated inputs.

[Source](/reference-sources/Data.Word.cinch) · line 184

<a id="value-6469762d776964652d753332"></a>

## `div-wide-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">div-wide-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Word</span>.<span class="tok-constructor">WordDivisionError</span> (<span class="tok-constructor">U32</span>, <span class="tok-constructor">U32</span>)))))</code></pre>

Divide a high/low 64-bit dividend by a 32-bit divisor, with a checked one-word quotient.

[Source](/reference-sources/Data.Word.cinch) · line 156

<a id="value-6469762d776964652d753634"></a>

## `div-wide-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">div-wide-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Word</span>.<span class="tok-constructor">WordDivisionError</span> (<span class="tok-constructor">U64</span>, <span class="tok-constructor">U64</span>)))))</code></pre>

Divide a high/low 128-bit dividend by a 64-bit divisor, with a checked one-word quotient.

[Source](/reference-sources/Data.Word.cinch) · line 167

<a id="value-6469762d776964652d7573697a65"></a>

## `div-wide-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">div-wide-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Word</span>.<span class="tok-constructor">WordDivisionError</span> (<span class="tok-constructor">Usize</span>, <span class="tok-constructor">Usize</span>)))))</code></pre>

Divide a high/low native double word, checking zero and quotient overflow.

[Source](/reference-sources/Data.Word.cinch) · line 229

<a id="value-6d756c2d776964652d753332"></a>

## `mul-wide-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mul-wide-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span>, <span class="tok-constructor">U32</span>)))</code></pre>

The exact 64-bit product of two 32-bit words, returned high word first.

[Source](/reference-sources/Data.Word.cinch) · line 129

<a id="value-6d756c2d776964652d753634"></a>

## `mul-wide-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mul-wide-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span>, <span class="tok-constructor">U64</span>)))</code></pre>

The exact 128-bit product of two 64-bit words, returned high word first.

[Source](/reference-sources/Data.Word.cinch) · line 134

<a id="value-6d756c2d776964652d7573697a65"></a>

## `mul-wide-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mul-wide-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span>, <span class="tok-constructor">Usize</span>)))</code></pre>

The exact double-width product of native words, returned high word first.

[Source](/reference-sources/Data.Word.cinch) · line 221

<a id="value-72656d2d776964652d753332"></a>

## `rem-wide-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rem-wide-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Word</span>.<span class="tok-constructor">WordDivisionError</span> <span class="tok-constructor">U32</span>))))</code></pre>

The remainder of a high/low 64-bit dividend. The high word need not be below the divisor.

[Source](/reference-sources/Data.Word.cinch) · line 188

<a id="value-72656d2d776964652d753634"></a>

## `rem-wide-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rem-wide-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Word</span>.<span class="tok-constructor">WordDivisionError</span> <span class="tok-constructor">U64</span>))))</code></pre>

The remainder of a high/low 128-bit dividend. The high word need not be below the divisor.

[Source](/reference-sources/Data.Word.cinch) · line 198

<a id="value-72656d2d776964652d7573697a65"></a>

## `rem-wide-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rem-wide-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Word</span>.<span class="tok-constructor">WordDivisionError</span> <span class="tok-constructor">Usize</span>))))</code></pre>

The remainder of a high/low native double word, checking only for a zero divisor.

[Source](/reference-sources/Data.Word.cinch) · line 239

<a id="value-726576657273652d62797465732d753136"></a>

## `reverse-bytes-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bytes-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">U16</span>)</code></pre>

Reverse byte order within a U16; bit order within each byte is preserved.

[Source](/reference-sources/Data.Word.cinch) · line 52

<a id="value-726576657273652d62797465732d753332"></a>

## `reverse-bytes-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bytes-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Reverse byte order within a U32; bit order within each byte is preserved.

[Source](/reference-sources/Data.Word.cinch) · line 65

<a id="value-726576657273652d62797465732d753634"></a>

## `reverse-bytes-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bytes-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">U64</span>)</code></pre>

Reverse byte order within a U64; bit order within each byte is preserved.

[Source](/reference-sources/Data.Word.cinch) · line 78

<a id="value-726576657273652d62797465732d7573697a65"></a>

## `reverse-bytes-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-bytes-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; <span class="tok-constructor">Usize</span>)</code></pre>

Reverse byte order within a Usize; bit order within each byte is preserved.

[Source](/reference-sources/Data.Word.cinch) · line 91

<a id="value-726f746174652d7369676e65642d753136"></a>

## `rotate-signed-u16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-signed-u16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U16</span>))</code></pre>

Rotate a U16 left by a signed count; negative counts rotate right and counts wrap by the width.

[Source](/reference-sources/Data.Word.cinch) · line 56

<a id="value-726f746174652d7369676e65642d753332"></a>

## `rotate-signed-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-signed-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

Rotate a U32 left by a signed count; negative counts rotate right and counts wrap by the width.

[Source](/reference-sources/Data.Word.cinch) · line 69

<a id="value-726f746174652d7369676e65642d753634"></a>

## `rotate-signed-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-signed-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U64</span>))</code></pre>

Rotate a U64 left by a signed count; negative counts rotate right and counts wrap by the width.

[Source](/reference-sources/Data.Word.cinch) · line 82

<a id="value-726f746174652d7369676e65642d7538"></a>

## `rotate-signed-u8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-signed-u8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">U8</span>))</code></pre>

Rotate a U8 left by a signed count; negative counts rotate right and counts wrap by the width.

[Source](/reference-sources/Data.Word.cinch) · line 43

<a id="value-726f746174652d7369676e65642d7573697a65"></a>

## `rotate-signed-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-signed-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Usize</span>))</code></pre>

Rotate a Usize left by a signed count; negative counts rotate right and counts wrap by the width.

[Source](/reference-sources/Data.Word.cinch) · line 95

<a id="value-7375622d626f72726f772d753332"></a>

## `sub-borrow-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sub-borrow-u32</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">U32</span>, <span class="tok-constructor">Bool</span>))))</code></pre>

Subtract a 32-bit word and an incoming borrow, returning the low word and outgoing borrow.

[Source](/reference-sources/Data.Word.cinch) · line 113

<a id="value-7375622d626f72726f772d753634"></a>

## `sub-borrow-u64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sub-borrow-u64</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">U64</span>, <span class="tok-constructor">Bool</span>))))</code></pre>

Subtract a 64-bit word and an incoming borrow, returning the low word and outgoing borrow.

[Source](/reference-sources/Data.Word.cinch) · line 121

<a id="value-7375622d626f72726f772d7573697a65"></a>

## `sub-borrow-usize`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sub-borrow-usize</span> :: (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Usize</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Usize</span>, <span class="tok-constructor">Bool</span>))))</code></pre>

Subtract native words and a borrow, returning the low word and outgoing borrow.

[Source](/reference-sources/Data.Word.cinch) · line 213

<a id="value-776f72642d62697473"></a>

## `word-bits`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">word-bits</span> :: <span class="tok-constructor">U32</span></code></pre>

The native unsigned word width, 32 or 64, determined by the target's Usize operations.

[Source](/reference-sources/Data.Word.cinch) · line 34

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Word.cinch`. Source SHA-256: `0242fdfc1db2ec8d84ca3dda30d80624e74e42dad42b6479a63176a316b2e3e1`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
