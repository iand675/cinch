# Data.Array.Search

Immutable array comparison, indexed lookups, binary search and sortedness with explicit callback policies.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Data.Array.Search</code></div><div class="doc-mod-bar-links"><a href="/library/source/Data.Array.Search/">View source</a><a href="/library/search/?q=%2BData.Array.Search">Search module</a><a href="/reference-sources/Data.Array.Search.cinch" download="Data.Array.Search.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Array](Data.Array.md), [Data.Compare](Data.Compare.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Search](Data.Search.md), [Relation.Eq](Relation.Eq.md).

<details class="doc-closure"><summary>Ship-source closure (4 files)</summary>

Copy these modules beside your entry file before importing `Data.Array.Search`: [Data.Array](Data.Array.md), [Data.Array.Search](Data.Array.Search.md), [Data.Compare](Data.Compare.md), [Data.Search](Data.Search.md).

```sh
cp seed/prelude/Data/Array.cinch ./Data.Array.cinch
cp seed/prelude/Data/Array/Search.cinch ./Data.Array.Search.cinch
cp seed/prelude/Data/Compare.cinch ./Data.Compare.cinch
cp seed/prelude/Data/Search.cinch ./Data.Search.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Comparison and lookup on immutable arrays. Linear searches stop at the
first match. Binary searches return the first equivalent element or its
insertion position. Callback variants preserve the caller's effects.

</details>

<details class="doc-synopsis" open><summary>Synopsis (12 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-array-binary-search">array-binary-search</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-binary-search</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ordered</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Compare/#c-Compare" title="Data.Compare.Compare">Compare</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a>)</code></li><li><a class="syn-name" href="#v-array-binary-search-by">array-binary-search-by</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-binary-search-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">e</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span>{e} (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a>)</code></li><li><a class="syn-name" href="#v-array-compare">array-compare</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-compare</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ordered</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Compare/#c-Compare" title="Data.Compare.Compare">Compare</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-array-compare-by">array-compare-by</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-compare-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">e</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li><a class="syn-name" href="#v-array-contains">array-contains</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-contains</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">equal</span> :: <a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-array-contains-by">array-contains-by</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-contains-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">e</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-array-equal">array-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-equal</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">equal</span> :: <a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-array-equal-by">array-equal-by</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-equal-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">e</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-array-index">array-index</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-index</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">equal</span> :: <a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-array-index-by">array-index-by</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-index-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">e</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-array-is-sorted">array-is-sorted</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-is-sorted</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ordered</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Compare/#c-Compare" title="Data.Compare.Compare">Compare</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-array-is-sorted-by">array-is-sorted-by</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-is-sorted-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">e</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-array-binary-search">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-binary-search</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ordered</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Compare/#c-Compare" title="Data.Compare.Compare">Compare</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a>)</code><span class="doc-sig-links"><a href="#v-array-binary-search" title="Permalink">#</a><a href="/library/source/Data.Array.Search/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

Find the first equivalent value or its insertion position using Compare.

</div>
</article>

<article class="doc-entry" id="v-array-binary-search-by">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-binary-search-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">e</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span>{e} (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a>)</code><span class="doc-sig-links"><a href="#v-array-binary-search-by" title="Permalink">#</a><a href="/library/source/Data.Array.Search/#L72" title="View source at line 72">Source</a></span></div>
<div class="doc-body">

Find a key in ascending order. The callback compares an array element
with the key, allowing a different key type. Duplicate keys return the first index.

</div>
</article>

<article class="doc-entry" id="v-array-compare">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-compare</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ordered</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Compare/#c-Compare" title="Data.Compare.Compare">Compare</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-array-compare" title="Permalink">#</a><a href="/library/source/Data.Array.Search/#L67" title="View source at line 67">Source</a></span></div>
<div class="doc-body">

Lexicographic comparison using the element Compare instance.

</div>
</article>

<article class="doc-entry" id="v-array-compare-by">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-compare-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">e</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code><span class="doc-sig-links"><a href="#v-array-compare-by" title="Permalink">#</a><a href="/library/source/Data.Array.Search/#L54" title="View source at line 54">Source</a></span></div>
<div class="doc-body">

Lexicographically compare arrays, putting a proper prefix before the longer array.

</div>
</article>

<article class="doc-entry" id="v-array-contains">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-contains</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">equal</span> :: <a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-array-contains" title="Permalink">#</a><a href="/library/source/Data.Array.Search/#L35" title="View source at line 35">Source</a></span></div>
<div class="doc-body">

Whether an element equals the requested value.

</div>
</article>

<article class="doc-entry" id="v-array-contains-by">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-contains-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">e</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-array-contains-by" title="Permalink">#</a><a href="/library/source/Data.Array.Search/#L31" title="View source at line 31">Source</a></span></div>
<div class="doc-body">

Whether an element satisfies the predicate.

</div>
</article>

<article class="doc-entry" id="v-array-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-equal</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">equal</span> :: <a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-array-equal" title="Permalink">#</a><a href="/library/source/Data.Array.Search/#L50" title="View source at line 50">Source</a></span></div>
<div class="doc-body">

Compare corresponding elements using their equality instance.

</div>
</article>

<article class="doc-entry" id="v-array-equal-by">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-equal-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">e</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-array-equal-by" title="Permalink">#</a><a href="/library/source/Data.Array.Search/#L40" title="View source at line 40">Source</a></span></div>
<div class="doc-body">

Compare lengths before calling the element equality predicate. An unequal
pair ends comparison immediately. The two element types may differ.

</div>
</article>

<article class="doc-entry" id="v-array-index">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-index</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">equal</span> :: <a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-array-index" title="Permalink">#</a><a href="/library/source/Data.Array.Search/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">

Locate the first element equal to the requested value.

</div>
</article>

<article class="doc-entry" id="v-array-index-by">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-index-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">e</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-array-index-by" title="Permalink">#</a><a href="/library/source/Data.Array.Search/#L18" title="View source at line 18">Source</a></span></div>
<div class="doc-body">

Locate the first element satisfying a predicate, visiting elements in order.

</div>
</article>

<article class="doc-entry" id="v-array-is-sorted">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-is-sorted</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ordered</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Compare/#c-Compare" title="Data.Compare.Compare">Compare</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-array-is-sorted" title="Permalink">#</a><a href="/library/source/Data.Array.Search/#L95" title="View source at line 95">Source</a></span></div>
<div class="doc-body">

Test nondecreasing order using Compare, including its floating NaN policy.

</div>
</article>

<article class="doc-entry" id="v-array-is-sorted-by">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-is-sorted-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">e</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-array-is-sorted-by" title="Permalink">#</a><a href="/library/source/Data.Array.Search/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">

Test whether the array is in nondecreasing order, stopping at the first inversion.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Array/Search.cinch`](/library/source/Data.Array.Search/). Source SHA-256: `a10366c85b000d05aa12b145ed82a50b461557ac57098b9b052b2d03f5c7f305`.
