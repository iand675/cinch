# Data.Enum

Finite enumeration and the class behind inclusive range literals, including an explicit next element for a step.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Enum`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Enum.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Core](Data.Core.md), [Data.List](Data.List.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Enum: the enumeration class behind the range literals.

`[e1..e2]` desugars to `from-list (enum-from-to e1 e2)` and
`[e1, e2 .. e3]` to `from-list (enum-from-then-to e1 e2 e3)` at the
expected type's `FromList c e` instance (the collection-literal
machinery of the surface grammar local amendments, section 10): the
container comes from the expected type, the enumeration steps through
this class, and a stage-known construction folds to a constant value
at compile time like every collection literal.

Module scheme (design/base-library.md): the class and the range
builders live here; instances live with their type (Data.Int,
Data.Nat). The enumeration codes are consecutive (`to-enum`/
`from-enum` are mutually inverse and neighbors differ by one), so the
range builders walk codes. A bounded instance answers `None` past
its edges (`pred` of its least value, `to-enum` of a code outside
the type), so no method faults (design/no-faults.md section 3); a
range stops where its codes leave the type.

</details>

# Types and constructors

# Signatures and methods

<a id="class-456e756d"></a>

## `Enum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Enum</span> <span class="tok-variable">a</span></code></pre>

[Source](/reference-sources/Data.Enum.cinch)

<a id="method-456e756d2e73756363"></a>

## `Enum.succ`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">succ</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Enum</span>.<span class="tok-constructor">Enum</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>)</code></pre>



<a id="method-456e756d2e70726564"></a>

## `Enum.pred`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pred</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Enum</span>.<span class="tok-constructor">Enum</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>



<a id="method-456e756d2e746f2d656e756d"></a>

## `Enum.to-enum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">to-enum</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Enum</span>.<span class="tok-constructor">Enum</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>



<a id="method-456e756d2e66726f6d2d656e756d"></a>

## `Enum.from-enum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">from-enum</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Enum</span>.<span class="tok-constructor">Enum</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Int</span>)</code></pre>



# Values

<a id="value-656e756d2d66726f6d2d7468656e2d746f"></a>

## `enum-from-then-to`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">enum-from-then-to</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Enum</span>.<span class="tok-constructor">Enum</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))))</code></pre>

`[lo, next .. hi]`'s builder: the codes from `lo` toward `hi`
inclusive, step `next - lo`.

[Source](/reference-sources/Data.Enum.cinch) · line 54

<a id="value-656e756d2d66726f6d2d746f"></a>

## `enum-from-to`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">enum-from-to</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Enum</span>.<span class="tok-constructor">Enum</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

`[lo..hi]`'s builder: the codes from `lo` to `hi` inclusive, step
one.

[Source](/reference-sources/Data.Enum.cinch) · line 49

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Enum.cinch`. Source SHA-256: `ebd926c5608f05da69741e21a3ef67ee563ed92d3622d7bbd50e860ec51818e8`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
