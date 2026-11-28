# Data.Finite

Exact finite domain enumeration for algebraic data, records, and compile-time state-machine action parameters.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Finite</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Finite/source/">View source</a><a href="/packages/search/?q=%2BData.Finite">Search module</a><a href="/reference-sources/Data.Finite.cinch" download="Data.Finite.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md).

<details class="doc-notes"><summary>Module notes</summary>

Exact finite domain enumeration for algebraic data, records, and compile-time
state-machine action parameters.

Built-in finite types (`Bool`, `Unit`, nullary enums, `Fin n`, `InRange lo hi Nat`,
`Option T`, and finite tuples) are enumerated automatically by `machine` actions.
Defining an `implicit instance Finite MyType` lets user records and payload-carrying
algebraic data types plug directly into the compiler's model checker without adding
special cases to the compiler, and also feeds `Effect.Test.Hegel.gen-finite`.

Symmetry soundness: listing a type `T` in `symmetry = [T]` is rejected when a
`Finite T` instance is in scope, because `enumerate` exposes a deterministic element
order that could break permutation symmetry (the same rule as `Ord T`).

</details>

<details class="doc-synopsis" open><summary>Synopsis (8 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#c-Finite">Finite</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Finite</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Finite-enumerate">enumerate</a><code class="syn-sig"><strong class="tok-variable">enumerate</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Finite/#c-Finite" title="Data.Finite.Finite">Finite</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-finite-count">finite-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">finite-count</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-finite-map">finite-map</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">finite-map</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-finite-product">finite-product</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">finite-product</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">c</span></code></li><li><a class="syn-name" href="#v-finite-product3">finite-product3</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">finite-product3</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span> <span class="tok-variable">d</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">d</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">d</span></code></li><li><a class="syn-name" href="#v-finite-range">finite-range</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">finite-range</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-finite-sum">finite-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">finite-sum</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li></ul>
</details>

# Signatures

<article class="doc-entry" id="c-Finite">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Finite</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Finite" title="Permalink">#</a><a href="/packages/stdlib/Data.Finite/source/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">

Exhaustive finite list of every value of type `a`.

<div class="doc-children"><h4>Methods</h4><ul><li id="m-Finite-enumerate"><div class="doc-method-row"><code><strong class="tok-variable">enumerate</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Finite/#c-Finite" title="Data.Finite.Finite">Finite</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Finite-enumerate" title="Permalink">#</a><a href="/packages/stdlib/Data.Finite/source/#L27">Source</a></span></div><div class="doc-child-prose">Produce all inhabitants of a on demand in a finite list.</div></li></ul></div>
<details class="doc-instances"><summary>Instances (6)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Finite/#c-Finite" title="Data.Finite.Finite">Finite</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Finite/#instances">Data.Finite</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Finite/#c-Finite" title="Data.Finite.Finite">Finite</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Finite/#instances">Data.Finite</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Finite/#c-Finite" title="Data.Finite.Finite">Finite</a> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Finite/#instances">Data.Finite</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Finite/#c-Finite" title="Data.Finite.Finite">Finite</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Finite/#instances">Data.Finite</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Finite/#c-Finite" title="Data.Finite.Finite">Finite</a> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>, <span class="tok-variable">c</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Finite/#instances">Data.Finite</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Finite/#c-Finite" title="Data.Finite.Finite">Finite</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Finite/#instances">Data.Finite</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-finite-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">finite-count</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-finite-count" title="Permalink">#</a><a href="/packages/stdlib/Data.Finite/source/#L65" title="View source at line 65">Source</a></span></div>
<div class="doc-body">

Count the number of elements in a finite domain list.

</div>
</article>

<article class="doc-entry" id="v-finite-map">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">finite-map</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-finite-map" title="Permalink">#</a><a href="/packages/stdlib/Data.Finite/source/#L35" title="View source at line 35">Source</a></span></div>
<div class="doc-body">

Map a pure constructor or function across a finite domain.

</div>
</article>

<article class="doc-entry" id="v-finite-product">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">finite-product</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">c</span></code><span class="doc-sig-links"><a href="#v-finite-product" title="Permalink">#</a><a href="/packages/stdlib/Data.Finite/source/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

Cartesian product of two finite domains combined by `f`.

</div>
</article>

<article class="doc-entry" id="v-finite-product3">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">finite-product3</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span> <span class="tok-variable">d</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">d</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">c</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">d</span></code><span class="doc-sig-links"><a href="#v-finite-product3" title="Permalink">#</a><a href="/packages/stdlib/Data.Finite/source/#L47" title="View source at line 47">Source</a></span></div>
<div class="doc-body">

Cartesian product of three finite domains combined by `f`.

</div>
</article>

<article class="doc-entry" id="v-finite-range">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">finite-range</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-finite-range" title="Permalink">#</a><a href="/packages/stdlib/Data.Finite/source/#L59" title="View source at line 59">Source</a></span></div>
<div class="doc-body">

Inclusive finite range of natural numbers `[lo .. hi]`.

</div>
</article>

<article class="doc-entry" id="v-finite-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">finite-sum</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-finite-sum" title="Permalink">#</a><a href="/packages/stdlib/Data.Finite/source/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">

Concatenate a list of finite branch domains (one per ADT constructor).

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Finite_Bool"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Finite/#c-Finite" title="Data.Finite.Finite">Finite</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Finite.Finite_Bool</code></span><div class="doc-child-prose">Finite domain of booleans: [False, True].</div></li>
<li id="i-Finite_Option"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Finite/#c-Finite" title="Data.Finite.Finite">Finite</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>)</code> <span class="doc-inst-meta">Implicit · <code>Data.Finite.Finite_Option</code></span><div class="doc-child-prose">Finite domain of Option a: None followed by Some x for every x in a.</div></li>
<li id="i-Finite_Pair"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Finite/#c-Finite" title="Data.Finite.Finite">Finite</a> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>)</code> <span class="doc-inst-meta">Implicit · <code>Data.Finite.Finite_Pair</code></span><div class="doc-child-prose">Finite domain of pairs (a, b).</div></li>
<li id="i-Finite_Result"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Finite/#c-Finite" title="Data.Finite.Finite">Finite</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span>)</code> <span class="doc-inst-meta">Implicit · <code>Data.Finite.Finite_Result</code></span><div class="doc-child-prose">Finite domain of Result e a: every Err e followed by every Ok a.</div></li>
<li id="i-Finite_Triple"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Finite/#c-Finite" title="Data.Finite.Finite">Finite</a> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>, <span class="tok-variable">c</span>)</code> <span class="doc-inst-meta">Implicit · <code>Data.Finite.Finite_Triple</code></span><div class="doc-child-prose">Finite domain of triples (a, b, c).</div></li>
<li id="i-Finite_Unit"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Finite/#c-Finite" title="Data.Finite.Finite">Finite</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Finite.Finite_Unit</code></span><div class="doc-child-prose">Finite domain of unit: [()].</div></li>
</ul>
