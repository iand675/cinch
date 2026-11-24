# Data.NonEmpty

Lists refined by nonemptiness. The total constructor returns Option; transferred operations must preserve the predicate.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.NonEmpty</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.NonEmpty/source/">View source</a><a href="/packages/search/?q=%2BData.NonEmpty">Search module</a><a href="/reference-sources/Data.NonEmpty.cinch" download="Data.NonEmpty.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Semigroup](Algebra.Semigroup.md), [Category.Applicative](Category.Applicative.md), [Category.Foldable](Category.Foldable.md), [Category.Traversable](Category.Traversable.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.NonEmpty: the non-empty list as a refinement (refinement types
design; decision D151: `NonEmpty` is a decidable predicate over
`List`, not a wrapper).

`type NonEmpty a = refine (xs :: List a) with NotNull xs`: `NonEmpty`
IS `List a` at runtime (the witness is `Prop`-erased, decision D31),
so `unrefine` projects at zero cost and no delegation code exists.
The transparent constructor `NonEmpty xs w` takes the list and its
witness: a `Dec (NotNull xs)` discharge that erases, or the `Yes`
evidence of a `decide`.

The total constructor `non-empty` discharges at runtime through the
`Dec`/`decide` world and returns `Option` (the total API of the
design note). Forging is impossible: a witness is checked evidence,
and evidence about mutable state dies with the write (the D192
invalidation rule).

Instances are the container's own methods (the transfer rule),
checked sample-wise by the P18 law layer exactly when they preserve
the predicate: `Semigroup` transfers (append of non-empties is
non-empty), `Monoid` cannot (its identity is empty; both asserted in
seed/tests/refinements.rs). The traversals preserve
non-emptiness too, though through the applicative they report `not
checked` at samples (the honest law boundary).

Status: this module is the P28 refinement of the bespoke `NonEmpty`
data type it replaces (decision D151).

</details>

<details class="doc-synopsis" open><summary>Synopsis (2 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-NonEmpty">NonEmpty</a><code class="syn-sig"><span class="tok-keyword">type</span> <strong class="doc-decl-name tok-constructor">NonEmpty</strong> <span class="tok-variable">a</span> = <span class="tok-keyword">refine</span> (<span class="tok-variable">xs</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>) <span class="tok-keyword">with</span> NotNull xs</code></li><li><a class="syn-name" href="#v-non-empty">non-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">non-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NonEmpty/#t-NonEmpty" title="Data.NonEmpty.NonEmpty">NonEmpty</a> <span class="tok-variable">a</span>)</code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-NonEmpty">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">type</span> <strong class="doc-decl-name tok-constructor">NonEmpty</strong> <span class="tok-variable">a</span> = <span class="tok-keyword">refine</span> (<span class="tok-variable">xs</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>) <span class="tok-keyword">with</span> NotNull xs</code><span class="doc-sig-links"><a href="#t-NonEmpty" title="Permalink">#</a><a href="/packages/stdlib/Data.NonEmpty/source/#L36" title="View source at line 36">Source</a></span></div>
<div class="doc-body">
<details class="doc-instances"><summary>Instances (3)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Foldable/#c-Foldable" title="Category.Foldable.Foldable">Foldable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NonEmpty/#t-NonEmpty" title="Data.NonEmpty.NonEmpty">NonEmpty</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.NonEmpty/#instances">Data.NonEmpty</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NonEmpty/#t-NonEmpty" title="Data.NonEmpty.NonEmpty">NonEmpty</a> <span class="tok-variable">x</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.NonEmpty/#instances">Data.NonEmpty</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Traversable/#c-Traversable" title="Category.Traversable.Traversable">Traversable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NonEmpty/#t-NonEmpty" title="Data.NonEmpty.NonEmpty">NonEmpty</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.NonEmpty/#instances">Data.NonEmpty</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-non-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">non-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NonEmpty/#t-NonEmpty" title="Data.NonEmpty.NonEmpty">NonEmpty</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-non-empty" title="Permalink">#</a><a href="/packages/stdlib/Data.NonEmpty/source/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

The total constructor (refinement types design: the total API):
the runtime decision lives in the `Dec`/`decide` world and its
witness erases into the transparent constructor.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Foldable_NonEmpty"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Foldable/#c-Foldable" title="Category.Foldable.Foldable">Foldable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NonEmpty/#t-NonEmpty" title="Data.NonEmpty.NonEmpty">NonEmpty</a></code> <span class="doc-inst-meta">Implicit · <code>Data.NonEmpty.Foldable_NonEmpty</code></span><div class="doc-child-prose">The container's own folds (the transfer rule): they read the refined value as the list it is.</div></li>
<li id="i-Semigroup_NonEmpty"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NonEmpty/#t-NonEmpty" title="Data.NonEmpty.NonEmpty">NonEmpty</a> <span class="tok-variable">x</span>)</code> <span class="doc-inst-meta">Implicit · <code>Data.NonEmpty.Semigroup_NonEmpty</code></span><div class="doc-child-prose">The free semigroup: the container's own &lt;&gt; (list append), preserved (append of non-empties is non-empty), associative by Semigroup.assoc; both checked sample-wise in the law layer.</div></li>
<li id="i-Traversable_NonEmpty"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Traversable/#c-Traversable" title="Category.Traversable.Traversable">Traversable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NonEmpty/#t-NonEmpty" title="Data.NonEmpty.NonEmpty">NonEmpty</a></code> <span class="doc-inst-meta">Implicit · <code>Data.NonEmpty.Traversable_NonEmpty</code></span><div class="doc-child-prose">The container's own traversal: traverse on the list it is, yielding the refined shape (the representation is the list).</div></li>
</ul>
