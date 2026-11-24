# Data.Set

Ordered sets and their membership, combination, difference, and conversion operations.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Set</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Set/source/">View source</a><a href="/packages/search/?q=%2BData.Set">Search module</a><a href="/reference-sources/Data.Set.cinch" download="Data.Set.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

<details class="doc-notes"><summary>Module notes</summary>

Data.Set: the ordered set.

A height-augmented AVL tree over `Ord a`: `set-insert` keeps the
left subtree strictly smaller and the right strictly greater (so a
repeated element keeps one copy) and rebalances on the way out, so
the operations are O(log n) and the tree is never deeper than
~1.44 log2(n). The earlier family was the documented UNBALANCED
bootstrap BST (decision D105's shape, "a balanced tree is a later
family, same surface"): this is that later family, and the
observable semantics are exactly as before (a repeated element
keeps one copy, insert/lookup round trip, `set-to-list` lists the
tree's elements in ascending order, each exactly once).

The node carries its cached height (`Bin h x l r`): the balanced
family needs O(1) balance metadata, so the bootstrap's three-field
node grows its height field (an observable shape change, reported
with P34; the operations' semantics are unchanged and no test
constructs a node by hand). A hand-built `Bin` tree is still a
correct search tree for every operation (`set-member`,
`set-to-list`, `set-size` never read heights), and `set-insert`
repairs the heights along the path it rebuilds; heights a hand-built
node lies about may cost it a rebalance, never correctness.

Named refusals (boundaries this module deliberately does not
cross):
  an arbitrary-height join: `balance` fixes subtrees that differ by
    at most two levels, so the ordered operations that drop a node
    rejoin through the right subtree's minimum (`set-glue`), and
    `set-filter`/`set-map` rebuild through `set-from-list` rather
    than a structural rejoin. That rebuild is why `set-filter`
    states `Ord a` where Data.Set's `filter` needs nothing;
  a size-augmented node: the node carries the height (AVL), so
    `set-size` stays the O(n) walk it has always been.

The bootstrap's deletion and union refusals are over: the family now
ships `set-delete`, `set-union`, `set-intersection`,
`set-difference`, `set-subset`, `set-map`, `set-filter` and
`set-fold` (the union behavior the bootstrap asserted test-locally
in seed/tests/prelude_split.rs is now the family's own).

Module scheme (design/base-library.md): the type's home carries its
`FromList` instance (`[a, b, c]` at `Set a` folds `set-insert`).

</details>

<details class="doc-synopsis" open><summary>Synopsis (16 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Set">Set</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Set</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-set-delete">set-delete</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">set-delete</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-set-difference">set-difference</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">set-difference</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-set-empty">set-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">set-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-set-filter">set-filter</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">set-filter</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-set-fold">set-fold</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">set-fold</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-set-from-list">set-from-list</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">set-from-list</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-set-insert">set-insert</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">set-insert</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-set-intersection">set-intersection</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">set-intersection</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-set-is-empty">set-is-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">set-is-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-set-map">set-map</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">set-map</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">b</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-set-member">set-member</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">set-member</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-set-size">set-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">set-size</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-set-subset">set-subset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">set-subset</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-set-to-list">set-to-list</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">set-to-list</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-set-union">set-union</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">set-union</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Set">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Set</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-Set" title="Permalink">#</a><a href="/packages/stdlib/Data.Set/source/#L50" title="View source at line 50">Source</a></span></div>
<div class="doc-body">

The ordered set: a height-augmented AVL tree over `Ord a`.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Tip"><code><strong class="tok-constructor">Tip</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span></code></li><li id="k-Bin"><code><strong class="tok-constructor">Bin</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span></code></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.FromList/#c-FromList" title="Data.FromList.FromList">FromList</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span>) <span class="tok-variable">a</span></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Set/#instances">Data.Set</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-set-delete">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">set-delete</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-set-delete" title="Permalink">#</a><a href="/packages/stdlib/Data.Set/source/#L141" title="View source at line 141">Source</a></span></div>
<div class="doc-body">

Removal: the search descends as `set-member` does, and a found
node's subtrees rejoin through the right subtree's smallest element
(the AVL delete), so the height invariant survives. An absent
element rebuilds the same tree.

</div>
</article>

<article class="doc-entry" id="v-set-difference">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">set-difference</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-set-difference" title="Permalink">#</a><a href="/packages/stdlib/Data.Set/source/#L208" title="View source at line 208">Source</a></span></div>
<div class="doc-body">

The elements of the left set that the right set does NOT hold.

</div>
</article>

<article class="doc-entry" id="v-set-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">set-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-set-empty" title="Permalink">#</a><a href="/packages/stdlib/Data.Set/source/#L54" title="View source at line 54">Source</a></span></div>
</article>

<article class="doc-entry" id="v-set-filter">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">set-filter</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-set-filter" title="Permalink">#</a><a href="/packages/stdlib/Data.Set/source/#L243" title="View source at line 243">Source</a></span></div>
<div class="doc-body">

The elements the predicate accepts, rebuilt from the surviving
ascending listing. The rebuild is why this states `Ord a` where
Data.Set's `filter` needs nothing: `balance` only fixes subtrees
that differ by at most two levels, so a structural filter (which
can empty one whole side) has no join to use and reinserts
instead. The rebuild also means duplicates cannot appear and the
ascending order is re-derived.

</div>
</article>

<article class="doc-entry" id="v-set-fold">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">set-fold</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-set-fold" title="Permalink">#</a><a href="/packages/stdlib/Data.Set/source/#L255" title="View source at line 255">Source</a></span></div>
<div class="doc-body">

A RIGHT fold in ascending order (Data.Set's `foldr`): folding with
`Cons` rebuilds `set-to-list`, and the empty set is the identity.

</div>
</article>

<article class="doc-entry" id="v-set-from-list">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">set-from-list</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-set-from-list" title="Permalink">#</a><a href="/packages/stdlib/Data.Set/source/#L132" title="View source at line 132">Source</a></span></div>
</article>

<article class="doc-entry" id="v-set-insert">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">set-insert</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-set-insert" title="Permalink">#</a><a href="/packages/stdlib/Data.Set/source/#L102" title="View source at line 102">Source</a></span></div>
</article>

<article class="doc-entry" id="v-set-intersection">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">set-intersection</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-set-intersection" title="Permalink">#</a><a href="/packages/stdlib/Data.Set/source/#L197" title="View source at line 197">Source</a></span></div>
<div class="doc-body">

The elements of the left set that the right set also holds.

</div>
</article>

<article class="doc-entry" id="v-set-is-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">set-is-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-set-is-empty" title="Permalink">#</a><a href="/packages/stdlib/Data.Set/source/#L179" title="View source at line 179">Source</a></span></div>
<div class="doc-body">

Named `set-is-empty` because the bare `is-empty` is the list's
(`List a -> Bool`) and there is no overloading to share one
spelling; the module prefix is how the family disambiguates.

</div>
</article>

<article class="doc-entry" id="v-set-map">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">set-map</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">b</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-set-map" title="Permalink">#</a><a href="/packages/stdlib/Data.Set/source/#L233" title="View source at line 233">Source</a></span></div>
<div class="doc-body">

The image of a set under a function (Data.Set's `map`): every
element is rewritten and reinserted, so two elements that collide
keep one copy and the result is ordered by the NEW values, hence
`Ord b` and never `Ord a`.

</div>
</article>

<article class="doc-entry" id="v-set-member">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">set-member</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-set-member" title="Permalink">#</a><a href="/packages/stdlib/Data.Set/source/#L110" title="View source at line 110">Source</a></span></div>
</article>

<article class="doc-entry" id="v-set-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">set-size</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-set-size" title="Permalink">#</a><a href="/packages/stdlib/Data.Set/source/#L127" title="View source at line 127">Source</a></span></div>
</article>

<article class="doc-entry" id="v-set-subset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">set-subset</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-set-subset" title="Permalink">#</a><a href="/packages/stdlib/Data.Set/source/#L221" title="View source at line 221">Source</a></span></div>
<div class="doc-body">

Whether every element of the first set is in the second (Data.Set's
`isSubsetOf`, kept under the module prefix, which is how every
other ordered operation here names itself).

</div>
</article>

<article class="doc-entry" id="v-set-to-list">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">set-to-list</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-set-to-list" title="Permalink">#</a><a href="/packages/stdlib/Data.Set/source/#L124" title="View source at line 124">Source</a></span></div>
</article>

<article class="doc-entry" id="v-set-union">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">set-union</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-set-union" title="Permalink">#</a><a href="/packages/stdlib/Data.Set/source/#L188" title="View source at line 188">Source</a></span></div>
<div class="doc-body">

The union inserts the left set's ascending listing into the right;
`set-insert` keeps one copy per element and rebalances, so the
result is a proper AVL tree. The result is symmetric in its
elements but the cost is not: O(m log n) for m and n elements.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-FromList_Set"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.FromList/#c-FromList" title="Data.FromList.FromList">FromList</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span>) <span class="tok-variable">a</span></code> <span class="doc-inst-meta">Implicit · <code>Data.Set.FromList_Set</code></span><div class="doc-child-prose">Collection literals at Set (surface grammar local amendments section 10).</div></li>
</ul>
