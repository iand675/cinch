# Data.Ordering

The LT, EQ, and GT comparison result vocabulary and its instances.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Data.Ordering</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Ordering/source/">View source</a><a href="/packages/search/?q=%2BData.Ordering">Search module</a><a href="/reference-sources/Data.Ordering.cinch" download="Data.Ordering.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Ordering: the three-way comparison result.

Module scheme (design/base-library.md): the type's home carries its
instance family (the parked-instance deviation of D159 is over).

</details>

<details class="doc-synopsis" open><summary>Synopsis (1 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Ordering">Ordering</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ordering</strong></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Ordering">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ordering</strong></code><span class="doc-sig-links"><a href="#t-Ordering" title="Permalink">#</a><a href="/packages/stdlib/Data.Ordering/source/#L13" title="View source at line 13">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-LT"><code><strong class="tok-constructor">LT</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li id="k-EQ"><code><strong class="tok-constructor">EQ</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li id="k-GT"><code><strong class="tok-constructor">GT</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (3)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Monoid/#c-Monoid" title="Algebra.Monoid.Monoid">Monoid</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Ordering/#instances">Data.Ordering</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Ordering/#instances">Data.Ordering</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Text.Show/#instances">Text.Show</a></span></li></ul></details>
</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Monoid_Ordering"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Monoid/#c-Monoid" title="Algebra.Monoid.Monoid">Monoid</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Ordering.Monoid_Ordering</code></span><div class="doc-child-prose">EQ is the identity (moved from Algebra.Monoid).</div></li>
<li id="i-Semigroup_Ordering"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Ordering.Semigroup_Ordering</code></span><div class="doc-child-prose">Lexicographic composition of comparisons: compare x y &lt;&gt; compare y z gives compare x z when the first is not EQ (moved from Algebra.Semigroup at the prelude split).</div></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Ordering.cinch`](/packages/stdlib/Data.Ordering/source/). Source SHA-256: `7157a1cbcc1e708ab1800e1e48c42e83aca986ee496d29b1f8d9392a708df537`.
