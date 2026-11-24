# Data.NonZero

Total nonzero-number constructors. The NonZero type is also available through the arithmetic vocabulary for safe divisors.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.NonZero</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.NonZero/source/">View source</a><a href="/packages/search/?q=%2BData.NonZero">Search module</a><a href="/reference-sources/Data.NonZero.cinch" download="Data.NonZero.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.NonZero: the non-zero number as a refinement (refinement types
design; decision D151).

`type NonZero a = refine (n :: a) with NonZero n` (declared in
Algebra.Arithmetic, whose `Div`/`Rem` classes name it, and in the
implicit surface from there; this is its public home): a `NonZero
Nat` IS its `Nat` at runtime, a `NonZero Int` its `Int` (the witness
is `Prop`-erased, decision D31). The carrier is any type the
predicate `NonZero` decides (`Nat`, `Int`, machine integers); any
other carrier is refused naming the predicate.

A numeral is a `NonZero` with nothing written: `10` at `NonZero Nat`
decides its predicate at compile time (and `0` is a compile error).
The total constructors `non-zero`/`non-zero-nat` discharge at
runtime through the `Dec`/`decide` world and return `Option`; the
transparent constructor `NonZero n w` takes the number and its
witness.

The instances below are the multiplicative ones on `NonZero Int`,
written at the container's types (the transfer rule): the product of
non-zeros is non-zero (preserved) and 1 is non-zero, so unlike
`Monoid (NonEmpty a)` (whose identity is empty) `Monoid (NonZero
Int)` exists. The unit laws state over `Semigroup.<>`, which a class
constraint does not entail; the law's own context supplies the
`Semigroup (NonZero Int)` dictionary, so the check reads this
module's product and identity and decides both laws at samples
(asserted in seed/tests/refinements.rs).

</details>

<details class="doc-synopsis" open><summary>Synopsis (2 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-non-zero">non-zero</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">non-zero</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a>)</code></li><li><a class="syn-name" href="#v-non-zero-nat">non-zero-nat</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">non-zero-nat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-non-zero">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">non-zero</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a>)</code><span class="doc-sig-links"><a href="#v-non-zero" title="Permalink">#</a><a href="/packages/stdlib/Data.NonZero/source/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

The total constructors (refinement types design: the total API).

</div>
</article>

<article class="doc-entry" id="v-non-zero-nat">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">non-zero-nat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-non-zero-nat" title="Permalink">#</a><a href="/packages/stdlib/Data.NonZero/source/#L42" title="View source at line 42">Source</a></span></div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Monoid_NonZero"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Monoid/#c-Monoid" title="Algebra.Monoid.Monoid">Monoid</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a>)</code> <span class="doc-inst-meta">Implicit · <code>Data.NonZero.Monoid_NonZero</code></span><div class="doc-child-prose">The multiplicative identity is 1: non-zero, so the transfer preserves the refinement (the contrast with Monoid (NonEmpty a), whose identity violates non-emptiness).</div></li>
<li id="i-Semigroup_NonZero"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a>)</code> <span class="doc-inst-meta">Implicit · <code>Data.NonZero.Semigroup_NonZero</code></span><div class="doc-child-prose">The multiplicative semigroup on non-zero integers: preserved (a non-zero product of non-zeros), associative (both checked sample-wise by the law layer).</div></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/NonZero.cinch`](/packages/stdlib/Data.NonZero/source/). Source SHA-256: `d09f99b17ce4e051a2ac0435c0563f094fe8b0fba310ece953a3c67d40f7327a`.
