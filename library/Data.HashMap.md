# Data.HashMap

Hash maps with explicit key constraints, lookup/update, and map-style combination operations.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Data.HashMap</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.HashMap/source/">View source</a><a href="/packages/search/?q=%2BData.HashMap">Search module</a><a href="/reference-sources/Data.HashMap.cinch" download="Data.HashMap.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Hash](Data.Hash.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.HashMap: the hash map over `Data.Hash`.

A tree-indexed bucket table: the key's hash digest indexes a
height-augmented AVL spine (Data.Set's balanced shape), and each
spine node holds the association list of the keys that hash to that
digest. A hash collision degrades the bucket to a scan without
losing either key, exactly as the bootstrap's fixed 8-bucket table
documented; what grows is the index: the spine rebalances as new
digests arrive, so the operations are O(log d) plus the bucket scan
for d distinct digests (the earlier family was the documented
fixed-size stub: "no resizing", now replaced). Keys need `Hash` and
`Eq`: hashing alone cannot decide a collision, and the `Hash` law
(Data.Hash) is what makes the bucket choice sound.

The map wraps its private spine (`HashMap :: Spine k v -> HashMap k
v`): the bootstrap's one-field node over a bucket-list spine grows
the balanced index behind the same constructor name (an observable
shape change, reported with P34; every operation's semantics is
unchanged and no test constructs a map by hand).

Named refusals (boundaries this module deliberately does not
cross):
  bucket-order observation: the buckets' entry order is an
    implementation detail (insertion order within a bucket), so
    `hash-map-to-list`/`hash-map-keys`/`hash-map-values` list the
    entries in that unstable order and nothing may depend on it (the
    ordered views live at Data.Map);
  a fixed bucket count: the digest, not `rem` of a constant, is the
    index (there is no `hash-buckets`);
  adversarial digests: the `Hash` law is the caller's contract (the
    spine balances whatever digests arrive; only bucket scans
    degrade).

Measured limits (compiler/programs/Run.Maps.cinch, asserted by
seed/tests/lib_hashmap_real.rs; spans are the program's own
monotonic clock, and the ranges below span runs on a loaded machine):
32000 distinct-digest keys insert in 0.2-0.4s and look up in
0.05-0.1s, where 8x the keys costs about 10x the time (n log n; an
8x jump to 64x is what a quadratic index or a fixed table with
unbounded buckets would show). There is no growth or rehash step to
trigger, because the spine holds one node per digest PRESENT and its
height is the only thing that grows. 256 distinct keys that share ONE
digest insert and look up in 0.7-0.9s each, which is the whole cost of
the named boundary above (the bucket scan, linear in the collision
class: 300 keys in one bucket measure at 0.8-0.9s, and correctness
does not move). The key count itself has no ceiling short of memory:
100000 distinct-digest keys insert in ~0.8s and look up in ~0.2s.

Module scheme (design/base-library.md): the type's home carries its
`FromMap` instance (`{k => v}` at `HashMap k v`).

</details>

<details class="doc-synopsis" open><summary>Synopsis (16 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-HashMap">HashMap</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HashMap</strong> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-hash-map-delete">hash-map-delete</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hash-map-delete</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-hash-map-empty">hash-map-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hash-map-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-hash-map-filter">hash-map-filter</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hash-map-filter</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-hash-map-fold">hash-map-fold</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hash-map-fold</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">b</span>. (<span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-hash-map-from-pairs">hash-map-from-pairs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hash-map-from-pairs</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-hash-map-insert">hash-map-insert</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hash-map-insert</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-hash-map-is-empty">hash-map-is-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hash-map-is-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-hash-map-keys">hash-map-keys</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hash-map-keys</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">k</span></code></li><li><a class="syn-name" href="#v-hash-map-lookup">hash-map-lookup</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hash-map-lookup</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-hash-map-map">hash-map-map</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hash-map-map</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">w</span>. (<span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">w</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">w</span></code></li><li><a class="syn-name" href="#v-hash-map-size">hash-map-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hash-map-size</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-hash-map-to-list">hash-map-to-list</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hash-map-to-list</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>)</code></li><li><a class="syn-name" href="#v-hash-map-union">hash-map-union</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hash-map-union</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-hash-map-union-with">hash-map-union-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hash-map-union-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-hash-map-values">hash-map-values</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hash-map-values</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">v</span></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-HashMap">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HashMap</strong> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#t-HashMap" title="Permalink">#</a><a href="/packages/stdlib/Data.HashMap/source/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HashMap"><code><strong class="tok-constructor">HashMap</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-constructor">Spine</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.FromMap/#c-FromMap" title="Data.FromMap.FromMap">FromMap</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) <span class="tok-variable">k</span> <span class="tok-variable">v</span></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.HashMap/#instances">Data.HashMap</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-hash-map-delete">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hash-map-delete</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-hash-map-delete" title="Permalink">#</a><a href="/packages/stdlib/Data.HashMap/source/#L164" title="View source at line 164">Source</a></span></div>
<div class="doc-body">

Removal: the bucket loses the key's entry, and a bucket left empty
drops its digest node from the spine, so deleting every key leaves
the same `STip` spine `hash-map-empty` has (nothing accumulates).

</div>
</article>

<article class="doc-entry" id="v-hash-map-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hash-map-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-hash-map-empty" title="Permalink">#</a><a href="/packages/stdlib/Data.HashMap/source/#L139" title="View source at line 139">Source</a></span></div>
</article>

<article class="doc-entry" id="v-hash-map-filter">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hash-map-filter</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-hash-map-filter" title="Permalink">#</a><a href="/packages/stdlib/Data.HashMap/source/#L325" title="View source at line 325">Source</a></span></div>
<div class="doc-body">

Keep the entries the predicate accepts, rebuilt through
`hash-map-from-pairs`. The rebuild, not a structural filter, is why
the constraints are stated: an emptied bucket would leave a digest
node to drop, and dropping one is Data.Set's arbitrary-height
rejoin problem. The predicate sees the key as well as the value.

</div>
</article>

<article class="doc-entry" id="v-hash-map-fold">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hash-map-fold</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">b</span>. (<span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-hash-map-fold" title="Permalink">#</a><a href="/packages/stdlib/Data.HashMap/source/#L342" title="View source at line 342">Source</a></span></div>
<div class="doc-body">

A RIGHT fold in `hash-map-to-list`'s order: the fold visits every
entry exactly once, and that order is the buckets' unstable order,
so an order-dependent combining function must not be relied on
(folding with `Cons` rebuilds `hash-map-to-list` itself, and the
empty map is the identity). The right direction is what makes the
`Cons` rebuild agree with the view.

</div>
</article>

<article class="doc-entry" id="v-hash-map-from-pairs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hash-map-from-pairs</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-hash-map-from-pairs" title="Permalink">#</a><a href="/packages/stdlib/Data.HashMap/source/#L155" title="View source at line 155">Source</a></span></div>
<div class="doc-body">

Pairs fold in from the head, so an EARLIER pair wins a repeated
key (the head is inserted last into the fold of the tail), like
Data.Map's `map-from-pairs`.

</div>
</article>

<article class="doc-entry" id="v-hash-map-insert">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hash-map-insert</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-hash-map-insert" title="Permalink">#</a><a href="/packages/stdlib/Data.HashMap/source/#L148" title="View source at line 148">Source</a></span></div>
</article>

<article class="doc-entry" id="v-hash-map-is-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hash-map-is-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-hash-map-is-empty" title="Permalink">#</a><a href="/packages/stdlib/Data.HashMap/source/#L264" title="View source at line 264">Source</a></span></div>
<div class="doc-body">

Named `hash-map-is-empty` because the bare `is-empty` is the
list's (`List a -> Bool`) and there is no overloading to share one
spelling; the module prefix is how the family disambiguates.

</div>
</article>

<article class="doc-entry" id="v-hash-map-keys">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hash-map-keys</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">k</span></code><span class="doc-sig-links"><a href="#v-hash-map-keys" title="Permalink">#</a><a href="/packages/stdlib/Data.HashMap/source/#L233" title="View source at line 233">Source</a></span></div>
</article>

<article class="doc-entry" id="v-hash-map-lookup">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hash-map-lookup</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-hash-map-lookup" title="Permalink">#</a><a href="/packages/stdlib/Data.HashMap/source/#L142" title="View source at line 142">Source</a></span></div>
</article>

<article class="doc-entry" id="v-hash-map-map">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hash-map-map</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">w</span>. (<span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">w</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">w</span></code><span class="doc-sig-links"><a href="#v-hash-map-map" title="Permalink">#</a><a href="/packages/stdlib/Data.HashMap/source/#L305" title="View source at line 305">Source</a></span></div>
<div class="doc-body">

Rewrite the values: the keys and their digests are unchanged, so
the spine's shape and cached heights carry over and no `Hash`/`Eq`
is needed (Data.Map's `map`).

</div>
</article>

<article class="doc-entry" id="v-hash-map-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hash-map-size</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-hash-map-size" title="Permalink">#</a><a href="/packages/stdlib/Data.HashMap/source/#L252" title="View source at line 252">Source</a></span></div>
<div class="doc-body">

The entry count: the buckets' lengths summed over the spine.

</div>
</article>

<article class="doc-entry" id="v-hash-map-to-list">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hash-map-to-list</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>)</code><span class="doc-sig-links"><a href="#v-hash-map-to-list" title="Permalink">#</a><a href="/packages/stdlib/Data.HashMap/source/#L219" title="View source at line 219">Source</a></span></div>
<div class="doc-body">

The entries in the buckets' unstable order (see the refusal in the
header): the ordered views live at Data.Map.

</div>
</article>

<article class="doc-entry" id="v-hash-map-union">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hash-map-union</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-hash-map-union" title="Permalink">#</a><a href="/packages/stdlib/Data.HashMap/source/#L279" title="View source at line 279">Source</a></span></div>
<div class="doc-body">

The LEFT map's value wins a shared key (Data.Map's `union`): the
left map's entries are inserted into the right, and
`hash-map-insert` replaces the stored value in place. The listing
that drives the walk is in the buckets' order, which cannot matter
here: the result is a map, and the walk visits each left entry
exactly once.

</div>
</article>

<article class="doc-entry" id="v-hash-map-union-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hash-map-union-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-hash-map-union-with" title="Permalink">#</a><a href="/packages/stdlib/Data.HashMap/source/#L285" title="View source at line 285">Source</a></span></div>
<div class="doc-body">

The union of `hash-map-union` with a shared key combined instead
of chosen: `f` sees the LEFT value first and the right value
second (Data.Map's `unionWith`).

</div>
</article>

<article class="doc-entry" id="v-hash-map-values">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hash-map-values</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-hash-map-values" title="Permalink">#</a><a href="/packages/stdlib/Data.HashMap/source/#L236" title="View source at line 236">Source</a></span></div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-FromMap_HashMap"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.FromMap/#c-FromMap" title="Data.FromMap.FromMap">FromMap</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) <span class="tok-variable">k</span> <span class="tok-variable">v</span></code> <span class="doc-inst-meta">Implicit · <code>Data.HashMap.FromMap_HashMap</code></span><div class="doc-child-prose">Map literals at HashMap (surface grammar local amendments section 10).</div></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/HashMap.cinch`](/packages/stdlib/Data.HashMap/source/). Source SHA-256: `3f1612214a25a5fa2d2613d8a76da79331430ed7394b56e6203fcd0d51a18d6b`.
