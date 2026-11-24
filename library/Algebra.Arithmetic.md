# Algebra.Arithmetic

Arithmetic operation signatures and their fixities, including NonZero divisors and InRange refinements.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Algebra.Arithmetic</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Algebra.Arithmetic/source/">View source</a><a href="/packages/search/?q=%2BAlgebra.Arithmetic">Search module</a><a href="/reference-sources/Algebra.Arithmetic.cinch" download="Algebra.Arithmetic.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

<details class="doc-notes"><summary>Module notes</summary>

Algebra.Arithmetic: the arithmetic operation classes.

The primitive types specification's arithmetic classes (section 8):
`Add`, `Sub`, `Mul` and `Neg` on the mathematical types, `Div` and
`Rem` for their division. These are operation classes, not the
named algebraic structures of the base library scheme
(design/base-library.md's Algebra.* family is Semigroup, Monoid,
Group, Semiring, Lattice); they group here as the arithmetic
operations kind. Machine scalars get NO arithmetic instances: a
class method has a single result type and cannot express which
overflow family is meant (primitive types specification section 8).
Division and remainder are total by construction: the divisor is a
`NonZero a` (design/no-faults.md). Instances live with their type.

</details>

<details class="doc-synopsis" open><summary>Synopsis (14 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-InRange">InRange</a><code class="syn-sig"><span class="tok-keyword">type</span> <strong class="doc-decl-name tok-constructor">InRange</strong> (<span class="tok-variable">lo</span> :: <span class="tok-constructor">Int</span>) (<span class="tok-variable">hi</span> :: <span class="tok-constructor">Int</span>) <span class="tok-variable">a</span> = <span class="tok-keyword">refine</span> (<span class="tok-variable">x</span> :: <span class="tok-variable">a</span>) <span class="tok-keyword">with</span> (lo &lt;= x &amp;&amp; x &lt;= hi)</code></li><li><a class="syn-name" href="#t-NonZero">NonZero</a><code class="syn-sig"><span class="tok-keyword">type</span> <strong class="doc-decl-name tok-constructor">NonZero</strong> <span class="tok-variable">a</span> = <span class="tok-keyword">refine</span> (<span class="tok-variable">n</span> :: <span class="tok-variable">a</span>) <span class="tok-keyword">with</span> NonZero n</code></li><li><a class="syn-name" href="#c-Add">Add</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Add</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Add-op-2b">+</a><code class="syn-sig"><strong class="tok-variable">+</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Add" title="Algebra.Arithmetic.Add">Add</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#c-Div">Div</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Div</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Div-op-2f">/</a><code class="syn-sig"><strong class="tok-variable">/</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Div" title="Algebra.Arithmetic.Div">Div</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#c-Mul">Mul</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Mul</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Mul-op-2a">*</a><code class="syn-sig"><strong class="tok-variable">*</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Mul" title="Algebra.Arithmetic.Mul">Mul</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#c-Neg">Neg</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Neg</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Neg-negate">negate</a><code class="syn-sig"><strong class="tok-variable">negate</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Neg" title="Algebra.Arithmetic.Neg">Neg</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#c-Rem">Rem</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Rem</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Rem-op-25">%</a><code class="syn-sig"><strong class="tok-variable">%</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Rem" title="Algebra.Arithmetic.Rem">Rem</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#c-Sub">Sub</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Sub</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Sub--">-</a><code class="syn-sig"><strong class="tok-variable">-</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Sub" title="Algebra.Arithmetic.Sub">Sub</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-InRange">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">type</span> <strong class="doc-decl-name tok-constructor">InRange</strong> (<span class="tok-variable">lo</span> :: <span class="tok-constructor">Int</span>) (<span class="tok-variable">hi</span> :: <span class="tok-constructor">Int</span>) <span class="tok-variable">a</span> = <span class="tok-keyword">refine</span> (<span class="tok-variable">x</span> :: <span class="tok-variable">a</span>) <span class="tok-keyword">with</span> (lo &lt;= x &amp;&amp; x &lt;= hi)</code><span class="doc-sig-links"><a href="#t-InRange" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Arithmetic/source/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

A range of a number, the result type of every operation whose
result lies in known bounds by construction (`u32-to-i64 :: U32 ->
InRange 0 4294967295 I64`, `bytes-get`'s `InRange 0 255 Nat`,
`char-code`'s `InRange 0 1114111 Nat`). The bounds are inclusive
index parameters, so the predicate is two comparisons the
difference-logic discharge reads directly: a let-bound result
carries `lo <= x && x <= hi` as a fact, an application in an
obligation carries it for its term, and where no range is demanded
the value is its number (`c - 48`, `nat-add c 1`).

</div>
</article>

<article class="doc-entry" id="t-NonZero">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">type</span> <strong class="doc-decl-name tok-constructor">NonZero</strong> <span class="tok-variable">a</span> = <span class="tok-keyword">refine</span> (<span class="tok-variable">n</span> :: <span class="tok-variable">a</span>) <span class="tok-keyword">with</span> NonZero n</code><span class="doc-sig-links"><a href="#t-NonZero" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Arithmetic/source/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">

The divisor type, declared here because the `Div`/`Rem` classes
and the `Nat`/`Int` homes (checked right after this module) name
it in their signatures; its total constructors and instances live
in its public home, `Data.NonZero`. One refinement states the
predicate for every carrier it decides (`NonZero Nat`, `NonZero
Int`, a machine integer): the carrier is the container, so a
`NonZero Nat` IS its `Nat` at runtime, and a numeral decides its
predicate at compile time.

<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Monoid/#c-Monoid" title="Algebra.Monoid.Monoid">Monoid</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.NonZero/#instances">Data.NonZero</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.NonZero/#instances">Data.NonZero</a></span></li></ul></details>
</div>
</article>

# Signatures

<article class="doc-entry" id="c-Add">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Add</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Add" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Arithmetic/source/#L44" title="View source at line 44">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Add-op-2b"><div class="doc-method-row"><code><strong class="tok-variable">+</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Add" title="Algebra.Arithmetic.Add">Add</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Add-op-2b" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Arithmetic/source/#L45">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (3)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Add" title="Algebra.Arithmetic.Add">Add</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Complex/#instances">Data.Complex</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Add" title="Algebra.Arithmetic.Add">Add</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Int/#instances">Data.Int</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Add" title="Algebra.Arithmetic.Add">Add</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Nat/#instances">Data.Nat</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-Div">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Div</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Div" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Arithmetic/source/#L68" title="View source at line 68">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Div-op-2f"><div class="doc-method-row"><code><strong class="tok-variable">/</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Div" title="Algebra.Arithmetic.Div">Div</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Div-op-2f" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Arithmetic/source/#L69">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Div" title="Algebra.Arithmetic.Div">Div</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Int/#instances">Data.Int</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Div" title="Algebra.Arithmetic.Div">Div</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Nat/#instances">Data.Nat</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-Mul">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Mul</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Mul" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Arithmetic/source/#L50" title="View source at line 50">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Mul-op-2a"><div class="doc-method-row"><code><strong class="tok-variable">*</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Mul" title="Algebra.Arithmetic.Mul">Mul</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Mul-op-2a" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Arithmetic/source/#L51">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (3)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Mul" title="Algebra.Arithmetic.Mul">Mul</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Complex/#instances">Data.Complex</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Mul" title="Algebra.Arithmetic.Mul">Mul</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Int/#instances">Data.Int</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Mul" title="Algebra.Arithmetic.Mul">Mul</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Nat/#instances">Data.Nat</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-Neg">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Neg</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Neg" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Arithmetic/source/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Neg-negate"><div class="doc-method-row"><code><strong class="tok-variable">negate</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Neg" title="Algebra.Arithmetic.Neg">Neg</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Neg-negate" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Arithmetic/source/#L54">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Neg" title="Algebra.Arithmetic.Neg">Neg</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Complex/#instances">Data.Complex</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Neg" title="Algebra.Arithmetic.Neg">Neg</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Int/#instances">Data.Int</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-Rem">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Rem</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Rem" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Arithmetic/source/#L65" title="View source at line 65">Source</a></span></div>
<div class="doc-body">

Modulo and division for the mathematical types (decision D91).
The divisor carries its evidence: a `NonZero a`, so no instance
can reach a zero divisor. A numeral divisor needs nothing written
(`n / 10` and `n % 10` decide `NonZero 10` at compile time); any
other divisor is a threaded `NonZero a` or the product of a
runtime decision (`decide (NonZero d)`). A total division whose
divisor may be zero is not this operation: `Data.Complex.div`'s
IEEE quotient stays a named function. Machine integers keep the
checked family only, per the primitive types specification.

<div class="doc-children"><h4>Methods</h4><ul><li id="m-Rem-op-25"><div class="doc-method-row"><code><strong class="tok-variable">%</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Rem" title="Algebra.Arithmetic.Rem">Rem</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Rem-op-25" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Arithmetic/source/#L66">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Rem" title="Algebra.Arithmetic.Rem">Rem</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Int/#instances">Data.Int</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Rem" title="Algebra.Arithmetic.Rem">Rem</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Nat/#instances">Data.Nat</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-Sub">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Sub</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Sub" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Arithmetic/source/#L47" title="View source at line 47">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Sub--"><div class="doc-method-row"><code><strong class="tok-variable">-</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Sub" title="Algebra.Arithmetic.Sub">Sub</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Sub--" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Arithmetic/source/#L48">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (3)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Sub" title="Algebra.Arithmetic.Sub">Sub</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Complex/#instances">Data.Complex</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Sub" title="Algebra.Arithmetic.Sub">Sub</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Int/#instances">Data.Int</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Sub" title="Algebra.Arithmetic.Sub">Sub</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Nat/#instances">Data.Nat</a></span></li></ul></details>
</div>
</article>

# Operator fixities

| Operator | Associativity | Precedence |
| --- | --- | --- |
| `%` | `infixl` | `9` |
| `*` | `infixl` | `9` |
| `+` | `infixl` | `8` |
| `-` | `infixl` | `8` |
| `/` | `infixl` | `9` |

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Algebra/Arithmetic.cinch`](/packages/stdlib/Algebra.Arithmetic/source/). Source SHA-256: `2fac63baa07b5f42a8c907b0ba821bde5401a6e50f76f5d81f24e30d392f5ee0`.
