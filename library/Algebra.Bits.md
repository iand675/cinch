# Algebra.Bits

Bit operations, shifts, and the corresponding operator/fixity vocabulary.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Algebra.Bits`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Algebra.Bits.cinch) · [How library modules are loaded](00-index.md)

<details><summary>Behaviour and boundary notes from the source</summary>

Algebra.Bits: the bit manipulation class.

The `Bits` class unifies the per-type primitive families so generic
code can be written over machine integers (primitive types
specification section 4). All methods are total: the bitwise
operations are total, shifts reduce the count modulo the width (the
wrapping family), and rotates are total. The checked shift family
stays as the per-type `checkedShlX`/`checkedShrX` primitives.
`test-bit`/`set-bit`/`clear-bit`/`complement-bit` follow the mask
rule: bit position n means position n modulo the width, consistent
with the shift rule. Instances live with their type (Data.Machine).

The traditional operator spellings below are wrappers over the
class methods (decision D90), like the weight wrappers, never new
compiler mechanisms. Bitwise NOT has no operator spelling because
both candidate symbols are reserved punctuation (`!` and `~`); use
`bit-not`.

</details>

# Types and constructors

# Signatures and methods

<a id="class-42697473"></a>

## `Bits`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span></code></pre>

Bit manipulation (primitive types specification section 4).

The `Bits` class unifies the per-type primitive families so generic
code can be written over machine integers. All methods are total,
following the specification: the bitwise operations are total,
shifts reduce the count modulo the width (the wrapping family), and
rotates are total. The checked shift family stays as the per-type
`checkedShlX`/`checkedShrX` primitives. `test-bit`/`set-bit`/
`clear-bit`/`complement-bit` are expressed through the mask rule: bit
position n means position n modulo the width, consistent with the
shift rule.

[Source](/reference-sources/Algebra.Bits.cinch) · line 41

<a id="method-426974732e6269742d616e64"></a>

## `Bits.bit-and`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-and</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e6269742d6f72"></a>

## `Bits.bit-or`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-or</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e6269742d786f72"></a>

## `Bits.bit-xor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-xor</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e6269742d6e6f74"></a>

## `Bits.bit-not`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bit-not</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>)</code></pre>



<a id="method-426974732e73686966742d6c656674"></a>

## `Bits.shift-left`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shift-left</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e73686966742d7269676874"></a>

## `Bits.shift-right`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shift-right</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e726f746174652d6c656674"></a>

## `Bits.rotate-left`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-left</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e726f746174652d7269676874"></a>

## `Bits.rotate-right`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rotate-right</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e746573742d626974"></a>

## `Bits.test-bit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">test-bit</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>



<a id="method-426974732e7365742d626974"></a>

## `Bits.set-bit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">set-bit</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e636c6561722d626974"></a>

## `Bits.clear-bit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">clear-bit</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e636f6d706c656d656e742d626974"></a>

## `Bits.complement-bit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">complement-bit</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-variable">a</span>))</code></pre>



<a id="method-426974732e636f756e742d6f6e6573"></a>

## `Bits.count-ones`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">count-ones</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>



<a id="method-426974732e6c656164696e672d7a65726f73"></a>

## `Bits.leading-zeros`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">leading-zeros</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>



<a id="method-426974732e747261696c696e672d7a65726f73"></a>

## `Bits.trailing-zeros`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">trailing-zeros</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>



# Values

<a id="value-26"></a>

## `&`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">&amp;</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">bits</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>))</code></pre>

Traditional bitwise operator spellings over the `Bits` class
(decision D90): wrappers like the weight wrappers, never new
compiler mechanisms. Bitwise NOT has no operator spelling because
both candidate symbols are reserved punctuation (`!` and `~`); use
`bit-not`. Machine integers deliberately get no arithmetic operator
instances (primitive types specification section 8): `%` and `/`
below cover the mathematical `Nat` and `Int` only.

[Source](/reference-sources/Algebra.Bits.cinch) · line 65

<a id="value-3c3c"></a>

## `<<`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">&lt;&lt;</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">bits</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Algebra.Bits.cinch)

<a id="value-3e3e"></a>

## `>>`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">&gt;&gt;</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">bits</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Algebra.Bits.cinch)

<a id="value-5e"></a>

## `^`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">^</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">bits</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Algebra.Bits.cinch)

<a id="value-7c"></a>

## `|`

<pre class="cinch-highlight"><code class="language-cinch">| :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">bits</span> :: <span class="tok-constructor">Bits</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Algebra.Bits.cinch)

# Operator fixities

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">infixl</span> <span class="tok-number">6</span> <span class="tok-operator">&amp;</span>
<span class="tok-keyword">infixl</span> <span class="tok-number">7</span> <span class="tok-operator">&lt;&lt;</span>
<span class="tok-keyword">infixl</span> <span class="tok-number">7</span> <span class="tok-operator">&gt;&gt;</span>
<span class="tok-keyword">infixl</span> <span class="tok-number">5</span> <span class="tok-operator">^</span>
<span class="tok-keyword">infixl</span> <span class="tok-number">4</span> |</code></pre>

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Algebra/Bits.cinch`. Source SHA-256: `8901a9f4e10c73cbb55e7bb14808c700a7c7e14b7142d222808696754429c712`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
