# Algebra.Semigroup

Associative combination vocabulary and its law. Reported finite checks are distinct from universal proofs.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Algebra.Semigroup</code></div><div class="doc-mod-bar-links"><a href="/library/source/Algebra.Semigroup/">View source</a><a href="/library/search/?q=%2BAlgebra.Semigroup">Search module</a><a href="/reference-sources/Algebra.Semigroup.cinch" download="Algebra.Semigroup.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

<details class="doc-notes"><summary>Module notes</summary>

Algebra.Semigroup: associative binary combination on a carrier.

`Semigroup` is the value-level class of the base library scheme
(design/base-library.md): one operator method `<>`, associative by
law. This module declares the operator and owns its fixity. Consumers
inherit the declaration through imports and cannot redeclare it.

Laws (asserted over sample values in seed/tests/lib_algebra.rs):
  associativity: (x <> y) <> z == x <> (y <> z)
A class constraint entails nothing (the standing no-superbundle
rule): this module states no Monoid import and every function names
exactly the constraints it uses.

Instance homes. The ownership rule is "instances live with their
type; structural instances over functions and tuples live with the
class module". The carriers that were parked here (List, Option,
Ordering, String) got their instances at the prelude split: they
live in the types' Data.* homes now. Function and tuple instances
are structural and here permanently. The instance-carrying newtypes
(Sum, Product, Dual, First, Last, Endo) are declared in
Algebra.Monoid with both of their instances, because an instance
lives with its type and a newtype's home is its class module.

</details>

<details class="doc-synopsis" open><summary>Synopsis (5 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#c-Semigroup">Semigroup</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Semigroup</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Semigroup-op-3c3e">&lt;&gt;</a><code class="syn-sig"><strong class="tok-variable">&lt;&gt;</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-combine-pairs">combine-pairs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">combine-pairs</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">sa</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-sconcat">sconcat</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sconcat</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">s</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-stimes">stimes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">stimes</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">s</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li></ul>
</details>

# Signatures

<article class="doc-entry" id="c-Semigroup">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Semigroup</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Semigroup" title="Permalink">#</a><a href="/library/source/Algebra.Semigroup/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Semigroup-op-3c3e"><div class="doc-method-row"><code><strong class="tok-variable">&lt;&gt;</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Semigroup-op-3c3e" title="Permalink">#</a><a href="/library/source/Algebra.Semigroup/#L28">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (18)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <a class="doc-type-link tok-constructor" href="/library/Algebra.Monoid/#t-All" title="Algebra.Monoid.All">All</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Algebra.Monoid/#instances">Algebra.Monoid</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <a class="doc-type-link tok-constructor" href="/library/Algebra.Monoid/#t-Any" title="Algebra.Monoid.Any">Any</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Algebra.Monoid/#instances">Algebra.Monoid</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<a class="doc-type-link tok-constructor" href="/library/Algebra.Monoid/#t-Dual" title="Algebra.Monoid.Dual">Dual</a> <span class="tok-variable">t</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Algebra.Monoid/#instances">Algebra.Monoid</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<a class="doc-type-link tok-constructor" href="/library/Algebra.Monoid/#t-Endo" title="Algebra.Monoid.Endo">Endo</a> <span class="tok-variable">t</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Algebra.Monoid/#instances">Algebra.Monoid</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<a class="doc-type-link tok-constructor" href="/library/Algebra.Monoid/#t-First" title="Algebra.Monoid.First">First</a> <span class="tok-variable">t</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Algebra.Monoid/#instances">Algebra.Monoid</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<a class="doc-type-link tok-constructor" href="/library/Algebra.Monoid/#t-Last" title="Algebra.Monoid.Last">Last</a> <span class="tok-variable">t</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Algebra.Monoid/#instances">Algebra.Monoid</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <a class="doc-type-link tok-constructor" href="/library/Algebra.Monoid/#t-Product" title="Algebra.Monoid.Product">Product</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Algebra.Monoid/#instances">Algebra.Monoid</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <a class="doc-type-link tok-constructor" href="/library/Algebra.Monoid/#t-Sum" title="Algebra.Monoid.Sum">Sum</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Algebra.Monoid/#instances">Algebra.Monoid</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<span class="tok-variable">t</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">u</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Algebra.Semigroup/#instances">Algebra.Semigroup</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<span class="tok-variable">t</span>, <span class="tok-variable">u</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Algebra.Semigroup/#instances">Algebra.Semigroup</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <a class="doc-type-link tok-constructor" href="/library/Data.Ascii/#t-Ascii" title="Data.Ascii.Ascii">Ascii</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Ascii/#instances">Data.Ascii</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">t</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<a class="doc-type-link tok-constructor" href="/library/Data.NonEmpty/#t-NonEmpty" title="Data.NonEmpty.NonEmpty">NonEmpty</a> <span class="tok-variable">x</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Data.NonEmpty/#instances">Data.NonEmpty</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Data.NonZero/#instances">Data.NonZero</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">t</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Option/#instances">Data.Option</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Ordering/#instances">Data.Ordering</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <a class="doc-type-link tok-constructor" href="/library/Data.Sorted/#t-Sorted" title="Data.Sorted.Sorted">Sorted</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Sorted/#instances">Data.Sorted</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.String/#instances">Data.String</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-combine-pairs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">combine-pairs</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">sa</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-combine-pairs" title="Permalink">#</a><a href="/library/source/Algebra.Semigroup/#L81" title="View source at line 81">Source</a></span></div>
<div class="doc-body">

One round of a balanced combine: adjacent pairs combined in order,
an odd last value carried unchanged (shared with Algebra.Monoid's
`mconcat`).

</div>
</article>

<article class="doc-entry" id="v-sconcat">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sconcat</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">s</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-sconcat" title="Permalink">#</a><a href="/library/source/Algebra.Semigroup/#L71" title="View source at line 71">Source</a></span></div>
<div class="doc-body">

`sconcat`: combine the whole list with `<>`, in order, or `None`
when there is nothing to combine. The carrier is `List a` and the
emptiness is reported as an `Option` (the prelude's partiality
convention). `NonEmpty` would be the more precise argument, but its
home (Data.NonEmpty) imports this module for `<>`, so naming it here
would be a cycle; the `Option` result is the version that lives in
this module.

The bracketing is balanced (rounds of `combine-pairs`), which the
associativity law makes equal to base's right fold `foldr1 (<>)`
with the same n - 1 combinations; for a `<>` that copies its
operands (String) the right fold re-copies the accumulated suffix at
every step, O(n^2) bytes, where the rounds copy O(n log n).

</div>
</article>

<article class="doc-entry" id="v-stimes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">stimes</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">s</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-stimes" title="Permalink">#</a><a href="/library/source/Algebra.Semigroup/#L97" title="View source at line 97">Source</a></span></div>
<div class="doc-body">

`stimes n x`: `n` copies of `x` combined with `<>`. A `Nat` cannot
be negative, but it can be zero, and a bare `Semigroup` has no
identity to return; base's `stimes 0` is an error and the prelude
refuses faults, so `0` reads as one copy. That is the reading under
which the law `stimes (m + n) x == stimes m x <> stimes n x` is
stated for `m, n >= 1`. By repeated doubling (associativity makes
any bracketing of the n copies equal): O(log n) combinations, where
chaining one copy at a time costs n - 1 and, for a `<>` that copies
its operands, O(n^2) bytes.

</div>
</article>

# Operator fixities

| Operator | Associativity | Precedence |
| --- | --- | --- |
| `<>` | `infixr` | `6` |

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Semigroup_Function"><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<span class="tok-variable">t</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">u</span>)</code> <span class="doc-inst-meta">Implicit · <code>Algebra.Semigroup.Semigroup_Function</code></span><div class="doc-child-prose">Structural: pointwise combination of functions into a semigroup.</div></li>
<li id="i-Semigroup_Pair"><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<span class="tok-variable">t</span>, <span class="tok-variable">u</span>)</code> <span class="doc-inst-meta">Implicit · <code>Algebra.Semigroup.Semigroup_Pair</code></span><div class="doc-child-prose">Structural: tuples combine componentwise, each side under its own constraint.</div></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Algebra/Semigroup.cinch`](/library/source/Algebra.Semigroup/). Source SHA-256: `03b8c3463a181da5deaa4de35921c2bfb69520b4aed1c0a21043fc72f9be6272`.
