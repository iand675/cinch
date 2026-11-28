# Data.List

Finite strict lists: construction, transformation, filtering, splitting, searching, folds, and collection instances.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.List</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.List/source/">View source</a><a href="/packages/search/?q=%2BData.List">Search module</a><a href="/reference-sources/Data.List.cinch" download="Data.List.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md), [Category.Alternative](Category.Alternative.md), [Category.Applicative](Category.Applicative.md), [Category.Foldable](Category.Foldable.md), [Category.Functor](Category.Functor.md), [Category.Monad](Category.Monad.md), [Category.Traversable](Category.Traversable.md), [Data.Core](Data.Core.md), [Data.FromList](Data.FromList.md), [Data.Ordering](Data.Ordering.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.List: the inductive list: `Nil` or `Cons x xs`.

Module scheme (design/base-library.md): the type's home carries its
instance family (the parked-instance deviation of D159 is over) and
its `FromList` instance (collection literals at `List` are the
identity). `append-list` is the list append the instance family
shares; it is deliberately not in the export list (the curated
surface spells append through `Semigroup`'s `<>`).

`map` here is the concrete list map (the prelude surface's `map`,
decision D106); the `Functor` class method of the same name is
reached through its constraint or an explicit `import
Category.Functor`, which shadows this one per the import shadowing
rule (decision D145).

The concrete `base` combinators live here beside the type: the four
partial projections, the pairing and splitting family, the searching
family, the bounded generators (`iterate`/`unfold`/`cycle` take their
bound as a parameter, because every list-producing walk in this
language must end), the scans and the element-seeded folds,
`transpose`, the stable merge sort, the run grouping, the joining
family, and `nub`/`subsequences`. The container-generic twins
(`sum`, `elem`, the folds with an element seed, ...) live in
`Category.Foldable`, which is reached with an explicit `import`; a
program that wants the list operation without a class constraint uses
the name here, which is the `map` situation over again.

</details>

<details class="doc-synopsis"><summary>Synopsis (43 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-List">List</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">List</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-op-3a">:</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">:</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-break">break</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">break</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-cycle">cycle</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cycle</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-drop">drop</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">drop</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-elem-index">elem-index</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">elem-index</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-find">find</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">find</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-find-index">find-index</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">find-index</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-fold-left1">fold-left1</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fold-left1</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-fold-right1">fold-right1</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fold-right1</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-foldr">foldr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">foldr</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-group-by">group-by</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">group-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-head">head</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">head</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-init">init</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">init</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-intercalate">intercalate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">intercalate</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-intersperse">intersperse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">intersperse</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-is-empty">is-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-iterate">iterate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">iterate</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-last">last</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">last</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-length">length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">length</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-list-length">list-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">list-length</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-lookup">lookup</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">lookup</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#v-map">map</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-nth">nth</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nth</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-nub">nub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">nub</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-partition">partition</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">partition</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-replicate">replicate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">replicate</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-reverse">reverse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">reverse</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-scanl">scanl</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scanl</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-scanr">scanr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scanr</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#v-sort">sort</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sort</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-sort-by">sort-by</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sort-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-span">span</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">span</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-split-at">split-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">split-at</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-subsequences">subsequences</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">subsequences</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-sum-list">sum-list</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sum-list</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">add</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Add" title="Algebra.Arithmetic.Add">Add</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tail">tail</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tail</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-take">take</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">take</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-transpose">transpose</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">transpose</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-unfold">unfold</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unfold</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>)) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-unzip">unzip</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unzip</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">b</span>)</code></li><li><a class="syn-name" href="#v-zip">zip</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>)</code></li><li><a class="syn-name" href="#v-zip-with">zip-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">c</span></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-List">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">List</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-List" title="Permalink">#</a><a href="/packages/stdlib/Data.Core/source/#L26" title="View source at line 26">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Nil"><code><strong class="tok-constructor">Nil</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li id="k-Cons"><code><strong class="tok-constructor">Cons</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li></ul></div>
<details class="doc-instances"><summary>Instances (11)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Alternative/#c-Alternative" title="Category.Alternative.Alternative">Alternative</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Applicative/#c-Applicative" title="Category.Applicative.Applicative">Applicative</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Foldable/#c-Foldable" title="Category.Foldable.Foldable">Foldable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.FromList/#c-FromList" title="Data.FromList.FromList">FromList</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">e</span>) <span class="tok-variable">e</span></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Monoid/#c-Monoid" title="Algebra.Monoid.Monoid">Monoid</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">t</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">t</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Traversable/#c-Traversable" title="Category.Traversable.Traversable">Traversable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Quick/#c-Arbitrary" title="Effect.Test.Quick.Arbitrary">Arbitrary</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Effect.Test.Quick/#instances">Effect.Test.Quick</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-op-3a">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">:</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-op-3a" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L63" title="View source at line 63">Source</a></span></div>
<div class="doc-body">

`List`, `Nil` and `Cons` are declared in `Data.Core` and re-exported
here (that module's header says why: `Data.List` and `Data.Option`
name each other in signatures, so neither can be checked first).
This home carries the type's instance family and its operations.

</div>
</article>

<article class="doc-entry" id="v-break">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">break</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-break" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L225" title="View source at line 225">Source</a></span></div>
<div class="doc-body">

The longest prefix whose elements do NOT satisfy `p`, and the rest.
Written out rather than spelled `span` of a negated predicate:
`not` lives in `Data.Bool`, which is checked after this module.

</div>
</article>

<article class="doc-entry" id="v-cycle">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cycle</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-cycle" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L304" title="View source at line 304">Source</a></span></div>
<div class="doc-body">

The list repeated `n` times. `base`'s `cycle` on an empty list
never returns; here the empty list stays empty, because every
list-producing walk in this language must end.

</div>
</article>

<article class="doc-entry" id="v-drop">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">drop</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-drop" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L112" title="View source at line 112">Source</a></span></div>
<div class="doc-body">

Everything but the first `n` elements.

</div>
</article>

<article class="doc-entry" id="v-elem-index">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">elem-index</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-elem-index" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L268" title="View source at line 268">Source</a></span></div>
<div class="doc-body">

The position of the first element equal to `x` (`None` when
absent).

</div>
</article>

<article class="doc-entry" id="v-find">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">find</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-find" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L245" title="View source at line 245">Source</a></span></div>
<div class="doc-body">

The first element `p` accepts (`None` when none does).

</div>
</article>

<article class="doc-entry" id="v-find-index">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">find-index</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-find-index" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L255" title="View source at line 255">Source</a></span></div>
<div class="doc-body">

The position of the first element `p` accepts (`None` when none
does). The position is a `Nat`: it counts from zero, so it cannot
be negative.

</div>
</article>

<article class="doc-entry" id="v-fold-left1">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fold-left1</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-fold-left1" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L335" title="View source at line 335">Source</a></span></div>
<div class="doc-body">

The left fold seeded with the first element, so an empty list has
nothing to return (`None`; `base`'s `foldl1` faults).

</div>
</article>

<article class="doc-entry" id="v-fold-right1">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fold-right1</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-fold-right1" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L348" title="View source at line 348">Source</a></span></div>
<div class="doc-body">

The right fold seeded with the first element (`None` on an empty
list).

</div>
</article>

<article class="doc-entry" id="v-foldr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">foldr</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-foldr" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L73" title="View source at line 73">Source</a></span></div>
</article>

<article class="doc-entry" id="v-group-by">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">group-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-group-by" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L433" title="View source at line 433">Source</a></span></div>
<div class="doc-body">

The runs of adjacent elements the `same` test groups together: a
run starts at an element and grows while `same` holds of that
element and the next candidate, so `same` is the same-group test
rather than an equality (`base`'s `groupBy`).

</div>
</article>

<article class="doc-entry" id="v-head">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">head</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-head" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L145" title="View source at line 145">Source</a></span></div>
<div class="doc-body">

The partial projections of a list: the first element, everything
but the first, the last element, and everything but the last. Each
reports the empty list as `None` (the prelude's partiality
convention: a partial accessor returns `Option`, never a fault).
`head` is `nth 0` spelled for the reader.

</div>
</article>

<article class="doc-entry" id="v-init">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">init</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-init" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L162" title="View source at line 162">Source</a></span></div>
</article>

<article class="doc-entry" id="v-intercalate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">intercalate</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-intercalate" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L459" title="View source at line 459">Source</a></span></div>
<div class="doc-body">

The lists joined with `sep` between them (`base`'s `intercalate`,
which is `concat` of `intersperse sep`); `Nil` and a singleton come
back unchanged. The join is the private `join-lists` below rather
than `Category.Foldable`'s class-level `concat`: this is a list
operation, its join is four lines, and a home needs no class
constraint to concatenate its own type.

</div>
</article>

<article class="doc-entry" id="v-intersperse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">intersperse</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-intersperse" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L442" title="View source at line 442">Source</a></span></div>
<div class="doc-body">

`sep` between the elements, one separator fewer than the list has
elements, so `Nil` and a singleton come back unchanged (`base`'s
`intersperse`).

</div>
</article>

<article class="doc-entry" id="v-is-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-empty" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L135" title="View source at line 135">Source</a></span></div>
</article>

<article class="doc-entry" id="v-iterate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">iterate</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-iterate" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L286" title="View source at line 286">Source</a></span></div>
<div class="doc-body">

The enumeration `x, f x, f (f x), ...` cut to `n` elements. The
bound is what keeps the walk total (`base`'s `iterate` is
infinite).

</div>
</article>

<article class="doc-entry" id="v-last">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">last</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-last" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L155" title="View source at line 155">Source</a></span></div>
</article>

<article class="doc-entry" id="v-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">length</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-length" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L78" title="View source at line 78">Source</a></span></div>
</article>

<article class="doc-entry" id="v-list-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">list-length</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-list-length" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L87" title="View source at line 87">Source</a></span></div>
<div class="doc-body">

The same count as a `Nat`, for the indexing and sizing arithmetic
that works in `Nat` (`take`/`drop`/`nth` below, offsets, widths).
`length` stays `Int` because the prelude surface has always
returned `Int` there.

</div>
</article>

<article class="doc-entry" id="v-lookup">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">lookup</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">k</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#v-lookup" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L275" title="View source at line 275">Source</a></span></div>
<div class="doc-body">

The value paired with `key`: the first pair whose key matches wins,
so a repeated key keeps the earliest value (`None` when absent).
`lookup` is the association-list read `Data.Map`/`Data.HashMap`
build their trees over.

</div>
</article>

<article class="doc-entry" id="v-map">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-map" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L68" title="View source at line 68">Source</a></span></div>
</article>

<article class="doc-entry" id="v-nth">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nth</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-nth" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L120" title="View source at line 120">Source</a></span></div>
<div class="doc-body">

The element at index `n` (`None` past the end).

</div>
</article>

<article class="doc-entry" id="v-nub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">nub</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">eq</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-nub" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L471" title="View source at line 471">Source</a></span></div>
<div class="doc-body">

The list with later duplicates removed, keeping the first
occurrence of each element (`base`'s `nub`; the `Eq` constraint is
what decides sameness).

</div>
</article>

<article class="doc-entry" id="v-partition">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">partition</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-partition" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L235" title="View source at line 235">Source</a></span></div>
<div class="doc-body">

The elements that satisfy `p` and the ones that do not, each in
input order (so the two results interleave back into the input).

</div>
</article>

<article class="doc-entry" id="v-replicate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">replicate</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-replicate" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L130" title="View source at line 130">Source</a></span></div>
<div class="doc-body">

`x` repeated `n` times.

</div>
</article>

<article class="doc-entry" id="v-reverse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">reverse</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-reverse" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L95" title="View source at line 95">Source</a></span></div>
<div class="doc-body">

Reversal (the accumulator is the reverse built so far, so the walk
is linear; `reverse-into` is the private twin every accumulator
walk here shares).

</div>
</article>

<article class="doc-entry" id="v-scanl">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scanl</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-scanl" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L313" title="View source at line 313">Source</a></span></div>
<div class="doc-body">

The successive reductions from the left, the seed first: `scanl f z
xs` has one element more than `xs` (`base`'s `scanl`).

</div>
</article>

<article class="doc-entry" id="v-scanr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scanr</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#v-scanr" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L320" title="View source at line 320">Source</a></span></div>
<div class="doc-body">

The successive reductions from the right: the first element is the
whole fold and the seed is last (`base`'s `scanr`).

</div>
</article>

<article class="doc-entry" id="v-sort">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sort</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-sort" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L416" title="View source at line 416">Source</a></span></div>
<div class="doc-body">

The list in ascending order under `Ord` (a stable merge sort).

</div>
</article>

<article class="doc-entry" id="v-sort-by">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sort-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-sort-by" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L387" title="View source at line 387">Source</a></span></div>
<div class="doc-body">

Merge sort under a three-way comparator. Stable: the left element
is kept unless the right one compares strictly before it, so equal
elements stay in their input order (`base`'s `sortBy`).

</div>
</article>

<article class="doc-entry" id="v-span">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">span</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-span" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L214" title="View source at line 214">Source</a></span></div>
<div class="doc-body">

The longest prefix whose elements satisfy `p`, and the rest.

</div>
</article>

<article class="doc-entry" id="v-split-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">split-at</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-split-at" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L205" title="View source at line 205">Source</a></span></div>
<div class="doc-body">

The first `n` elements and the rest; the split clamps at the end,
so `split-at n` of a shorter list is the whole list and `Nil`.

</div>
</article>

<article class="doc-entry" id="v-subsequences">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">subsequences</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-subsequences" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L489" title="View source at line 489">Source</a></span></div>
<div class="doc-body">

Every subsequence, the empty one first, in `base`'s order:
`subsequences [a, b]` is `[[], [a], [b], [a, b]]`. The count is
`2^n` for a list of `n` elements, so this is the one combinator
here whose result can dwarf its input; it still ends, because the
input does.

</div>
</article>

<article class="doc-entry" id="v-sum-list">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sum-list</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">add</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#c-Add" title="Algebra.Arithmetic.Add">Add</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-sum-list" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L506" title="View source at line 506">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tail">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tail</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-tail" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L150" title="View source at line 150">Source</a></span></div>
</article>

<article class="doc-entry" id="v-take">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">take</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-take" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L104" title="View source at line 104">Source</a></span></div>
<div class="doc-body">

The first `n` elements (fewer when the list is shorter).

</div>
</article>

<article class="doc-entry" id="v-transpose">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">transpose</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-transpose" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L365" title="View source at line 365">Source</a></span></div>
<div class="doc-body">

The transpose of a ragged list of lists: round k is the k-th
element of every row that still has one, so a short row contributes
nothing to the later rounds and the result has as many rows as the
longest input row (`base`'s `transpose`). A rectangular `Matrix`
record keeps its own row-level helper; this is the general list
operation.

</div>
</article>

<article class="doc-entry" id="v-unfold">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unfold</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. (<span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>)) <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-unfold" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L295" title="View source at line 295">Source</a></span></div>
<div class="doc-body">

The unfolding: `f` either yields the next element together with the
next seed, or stops. `base` spells this `unfoldr`; the `r` suffix
is historical (the walk is not a right fold), so the name here is
the plain one.

</div>
</article>

<article class="doc-entry" id="v-unzip">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unzip</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">b</span>)</code><span class="doc-sig-links"><a href="#v-unzip" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L196" title="View source at line 196">Source</a></span></div>
<div class="doc-body">

The inverse of `zip`: the first components and the second
components, each in order.

</div>
</article>

<article class="doc-entry" id="v-zip">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>)</code><span class="doc-sig-links"><a href="#v-zip" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L179" title="View source at line 179">Source</a></span></div>
<div class="doc-body">

Pairing, truncated at the shorter side (so a length mismatch
shortens the result, never faults).

</div>
</article>

<article class="doc-entry" id="v-zip-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-with</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">c</span></code><span class="doc-sig-links"><a href="#v-zip-with" title="Permalink">#</a><a href="/packages/stdlib/Data.List/source/#L187" title="View source at line 187">Source</a></span></div>
<div class="doc-body">

`zip` with the pair built by `f`, truncated the same way.

</div>
</article>

# Operator fixities

| Operator | Associativity | Precedence |
| --- | --- | --- |
| `:` | `infixr` | `5` |

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Alternative_List"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Alternative/#c-Alternative" title="Category.Alternative.Alternative">Alternative</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">Implicit · <code>Data.List.Alternative_List</code></span><div class="doc-child-prose">Choice is concatenation (moved from Category.Alternative).</div></li>
<li id="i-Applicative_List"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Applicative/#c-Applicative" title="Category.Applicative.Applicative">Applicative</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">Implicit · <code>Data.List.Applicative_List</code></span><div class="doc-child-prose">pure is the singleton list; &lt;&gt; applies every function to every value in order (the list of all results; moved from Category.Applicative).</div></li>
<li id="i-Foldable_List"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Foldable/#c-Foldable" title="Category.Foldable.Foldable">Foldable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">Implicit · <code>Data.List.Foldable_List</code></span><div class="doc-child-prose">The list fold (moved from Category.Foldable).</div></li>
<li id="i-FromList_List"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.FromList/#c-FromList" title="Data.FromList.FromList">FromList</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">e</span>) <span class="tok-variable">e</span></code> <span class="doc-inst-meta">Implicit · <code>Data.List.FromList_List</code></span></li>
<li id="i-Functor_List"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">Implicit · <code>Data.List.Functor_List</code></span><div class="doc-child-prose">Functor: map over every element (moved from Category.Functor).</div></li>
<li id="i-Monad_List"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">Implicit · <code>Data.List.Monad_List</code></span><div class="doc-child-prose">Bind is flatMap: every element's continuation result, in order (moved from Category.Monad).</div></li>
<li id="i-Monoid_List"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Monoid/#c-Monoid" title="Algebra.Monoid.Monoid">Monoid</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">t</span>)</code> <span class="doc-inst-meta">Implicit · <code>Data.List.Monoid_List</code></span><div class="doc-child-prose">Monoid under append; the empty list is the identity (moved from Algebra.Monoid).</div></li>
<li id="i-Semigroup_List"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">t</span>)</code> <span class="doc-inst-meta">Implicit · <code>Data.List.Semigroup_List</code></span><div class="doc-child-prose">Semigroup under append: associative because list concatenation is (moved from Algebra.Semigroup at the prelude split).</div></li>
<li id="i-Traversable_List"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Traversable/#c-Traversable" title="Category.Traversable.Traversable">Traversable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">Implicit · <code>Data.List.Traversable_List</code></span><div class="doc-child-prose">Left-to-right; the whole list rebuilds under one f (moved from Category.Traversable).</div></li>
</ul>
