# Data.Map

Ordered maps with comparison constraints, lookup, insertion, removal, traversal, and collection-literal support.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Map</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Map/source/">View source</a><a href="/packages/search/?q=%2BData.Map">Search module</a><a href="/reference-sources/Data.Map.cinch" download="Data.Map.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.FromMap](Data.FromMap.md), [Data.List](Data.List.md), [Data.Option](Data.Option.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Map: the ordered map.

An unbalanced binary search tree over `Ord k` (decision D105).
`map-insert` keeps the left subtree strictly smaller and the right
strictly greater, so a repeated key replaces the stored value in
place.

Module scheme (design/base-library.md): the type's home carries its
`FromMap` instance (map literals at `Map`).

</details>

<details class="doc-synopsis" open><summary>Synopsis (16 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Map">Map</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Map</strong> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-map-delete">map-delete</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-delete</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-map-empty">map-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-map-filter">map-filter</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-filter</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. (<span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-map-fold">map-fold</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-fold</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">b</span>. (<span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-map-from-pairs">map-from-pairs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-from-pairs</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-map-insert">map-insert</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-insert</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-map-is-empty">map-is-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-is-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-map-keys">map-keys</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-keys</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">k</span></code></li><li><a class="syn-name" href="#v-map-lookup">map-lookup</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-lookup</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-map-map">map-map</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-map</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">w</span>. (<span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">w</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">w</span></code></li><li><a class="syn-name" href="#v-map-size">map-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-size</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-map-to-list">map-to-list</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-to-list</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>)</code></li><li><a class="syn-name" href="#v-map-union">map-union</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-union</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-map-union-with">map-union-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-union-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-map-values">map-values</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-values</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">v</span></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Map">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Map</strong> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#t-Map" title="Permalink">#</a><a href="/packages/stdlib/Data.Map/source/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">

The ordered map: an unbalanced binary search tree over `Ord k`
(decision D105). `map-insert` keeps the left subtree strictly
smaller and the right strictly greater, so a repeated key replaces
the stored value in place.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Tip"><code><strong class="tok-constructor">Tip</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li id="k-Bin"><code><strong class="tok-constructor">Bin</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.FromMap/#c-FromMap" title="Data.FromMap.FromMap">FromMap</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) <span class="tok-variable">k</span> <span class="tok-variable">v</span></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Map/#instances">Data.Map</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-map-delete">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-delete</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-map-delete" title="Permalink">#</a><a href="/packages/stdlib/Data.Map/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

Removal: the search descends as `map-lookup` does, and the found
node's subtrees rejoin through `map-glue`. An absent key rebuilds
the same tree, so no view can tell a delete of an absent key from
the identity.

</div>
</article>

<article class="doc-entry" id="v-map-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-map-empty" title="Permalink">#</a><a href="/packages/stdlib/Data.Map/source/#L54" title="View source at line 54">Source</a></span></div>
<div class="doc-body">

The empty map, as a name (the family's other homes already spell
theirs `set-empty`/`hash-map-empty`; the bootstrap exposed `Tip`
only through the data declaration).

</div>
</article>

<article class="doc-entry" id="v-map-filter">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-filter</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. (<span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-map-filter" title="Permalink">#</a><a href="/packages/stdlib/Data.Map/source/#L166" title="View source at line 166">Source</a></span></div>
<div class="doc-body">

Keep the entries the predicate accepts; a dropped node's children
rejoin with `map-glue`, so no `Ord` is needed either (Data.Map's
`filterWithKey`). The predicate sees the KEY as well as the value:
the entry, not the bare value, is what this module lists.

</div>
</article>

<article class="doc-entry" id="v-map-fold">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-fold</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">b</span>. (<span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-map-fold" title="Permalink">#</a><a href="/packages/stdlib/Data.Map/source/#L177" title="View source at line 177">Source</a></span></div>
<div class="doc-body">

A RIGHT fold in ascending key order (Data.Map's `foldrWithKey`).
The right direction is the one that agrees with the view: folding
with `Cons` rebuilds `map-to-list` exactly, and the empty map is
the identity. The accumulator's role is the right (second) value.

</div>
</article>

<article class="doc-entry" id="v-map-from-pairs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-from-pairs</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-map-from-pairs" title="Permalink">#</a><a href="/packages/stdlib/Data.Map/source/#L45" title="View source at line 45">Source</a></span></div>
</article>

<article class="doc-entry" id="v-map-insert">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-insert</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-map-insert" title="Permalink">#</a><a href="/packages/stdlib/Data.Map/source/#L29" title="View source at line 29">Source</a></span></div>
</article>

<article class="doc-entry" id="v-map-is-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-is-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-map-is-empty" title="Permalink">#</a><a href="/packages/stdlib/Data.Map/source/#L150" title="View source at line 150">Source</a></span></div>
<div class="doc-body">

Named `map-is-empty` because the bare `is-empty` is the list's
(`List a -> Bool`) and this language has no overloading to share
one spelling across carriers; the module prefix is how the family
disambiguates. (`map-null` was the alternative; `null` is the
`Foldable` class method's name and would suggest that class.)

</div>
</article>

<article class="doc-entry" id="v-map-keys">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-keys</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">k</span></code><span class="doc-sig-links"><a href="#v-map-keys" title="Permalink">#</a><a href="/packages/stdlib/Data.Map/source/#L120" title="View source at line 120">Source</a></span></div>
<div class="doc-body">

The keys in ascending order.

</div>
</article>

<article class="doc-entry" id="v-map-lookup">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-lookup</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-map-lookup" title="Permalink">#</a><a href="/packages/stdlib/Data.Map/source/#L37" title="View source at line 37">Source</a></span></div>
</article>

<article class="doc-entry" id="v-map-map">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-map</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">w</span>. (<span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">w</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">w</span></code><span class="doc-sig-links"><a href="#v-map-map" title="Permalink">#</a><a href="/packages/stdlib/Data.Map/source/#L157" title="View source at line 157">Source</a></span></div>
<div class="doc-body">

Rewrite the values, leave the keys where they are. The shape is
preserved, so no `Ord` is needed (Data.Map's `map`).

</div>
</article>

<article class="doc-entry" id="v-map-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-size</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-map-size" title="Permalink">#</a><a href="/packages/stdlib/Data.Map/source/#L140" title="View source at line 140">Source</a></span></div>
<div class="doc-body">

The entry count: the O(n) walk (the node carries no size, only the
key/value pair).

</div>
</article>

<article class="doc-entry" id="v-map-to-list">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-to-list</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>)</code><span class="doc-sig-links"><a href="#v-map-to-list" title="Permalink">#</a><a href="/packages/stdlib/Data.Map/source/#L111" title="View source at line 111">Source</a></span></div>
<div class="doc-body">

The entries in ascending key order. The tree IS an ordered search
tree, so this order is observable and stable: the tests pin it
against a known construction, and a caller may rely on it.

</div>
</article>

<article class="doc-entry" id="v-map-union">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-union</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-map-union" title="Permalink">#</a><a href="/packages/stdlib/Data.Map/source/#L85" title="View source at line 85">Source</a></span></div>
<div class="doc-body">

The LEFT map's value wins a shared key (Data.Map's `union`): the
left map's entries are inserted into the right, and `map-insert`
replaces the stored value in place. Two n-entry maps cost
O(n log n).

</div>
</article>

<article class="doc-entry" id="v-map-union-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-union-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-map-union-with" title="Permalink">#</a><a href="/packages/stdlib/Data.Map/source/#L91" title="View source at line 91">Source</a></span></div>
<div class="doc-body">

The union of `map-union` with a shared key combined instead of
chosen: `f` sees the LEFT value first and the right value second
(Data.Map's `unionWith`).

</div>
</article>

<article class="doc-entry" id="v-map-values">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-values</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-map-values" title="Permalink">#</a><a href="/packages/stdlib/Data.Map/source/#L130" title="View source at line 130">Source</a></span></div>
<div class="doc-body">

The values in ascending KEY order (Data.Map's `elems`), never in
insertion order.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-FromMap_Map"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.FromMap/#c-FromMap" title="Data.FromMap.FromMap">FromMap</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) <span class="tok-variable">k</span> <span class="tok-variable">v</span></code> <span class="doc-inst-meta">Implicit · <code>Data.Map.FromMap_Map</code></span></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Map.cinch`](/packages/stdlib/Data.Map/source/). Source SHA-256: `b7688dd045304cd4a46530683c40f9e15335159ec9a016845d794294e81e538d`.
