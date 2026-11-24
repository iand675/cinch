# Data.Sorted

Lists refined by sortedness, with construction and combination that preserve ordering.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Sorted</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Sorted/source/">View source</a><a href="/packages/search/?q=%2BData.Sorted">Search module</a><a href="/reference-sources/Data.Sorted.cinch" download="Data.Sorted.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Semigroup](Algebra.Semigroup.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Sorted: the sorted list as a refinement (refinement types
design; decision D151).

`type Sorted = refine (xs :: List Nat) with Ordered xs`: `Sorted` IS
`List Nat` at runtime (the witness is `Prop`-erased, decision D31).
The predicate form: `Ordered` decides lists of numerics (the
ordering relation over an abstract element type is refused naming the
predicate: the `Dec` world decides numerics, not dictionaries).

The total constructor `to-sorted` discharges at runtime through the
`Dec`/`decide` world and returns `Option`; the transparent
constructor `Sorted xs w` takes the list and its witness (a `Dec
(Ordered xs)` discharge that erases, or `Yes` evidence).

The instance below is the sorted MERGE, not list append: append of
two sorted lists is not sorted in general, so the container's `<>`
does NOT transfer (the preservation law rejects it naming
`Sorted.preserve-<>`; asserted in seed/tests/refinements.rs,
together with the deliberately non-preserving reverse-append).

</details>

<details class="doc-synopsis" open><summary>Synopsis (3 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Sorted">Sorted</a><code class="syn-sig"><span class="tok-keyword">type</span> <strong class="doc-decl-name tok-constructor">Sorted</strong> = <span class="tok-keyword">refine</span> (<span class="tok-variable">xs</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-keyword">with</span> Ordered xs</code></li><li><a class="syn-name" href="#v-merge">merge</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">merge</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-to-sorted">to-sorted</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-sorted</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sorted/#t-Sorted" title="Data.Sorted.Sorted">Sorted</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Sorted">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">type</span> <strong class="doc-decl-name tok-constructor">Sorted</strong> = <span class="tok-keyword">refine</span> (<span class="tok-variable">xs</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-keyword">with</span> Ordered xs</code><span class="doc-sig-links"><a href="#t-Sorted" title="Permalink">#</a><a href="/packages/stdlib/Data.Sorted/source/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sorted/#t-Sorted" title="Data.Sorted.Sorted">Sorted</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Sorted/#instances">Data.Sorted</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-merge">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">merge</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-merge" title="Permalink">#</a><a href="/packages/stdlib/Data.Sorted/source/#L36" title="View source at line 36">Source</a></span></div>
<div class="doc-body">

The sorted merge: the output of merging sorted lists is sorted
(preservation), and merge is associative (`Semigroup.assoc`);
both checked sample-wise by the law layer.

</div>
</article>

<article class="doc-entry" id="v-to-sorted">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-sorted</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sorted/#t-Sorted" title="Data.Sorted.Sorted">Sorted</a></code><span class="doc-sig-links"><a href="#v-to-sorted" title="Permalink">#</a><a href="/packages/stdlib/Data.Sorted/source/#L28" title="View source at line 28">Source</a></span></div>
<div class="doc-body">

The total constructor (refinement types design: the total API).

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Semigroup_Sorted"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sorted/#t-Sorted" title="Data.Sorted.Sorted">Sorted</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Sorted.Semigroup_Sorted</code></span></li>
</ul>
