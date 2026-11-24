# Data.Array.Edit

Checked immutable array range edits, predicate deletion, adjacent compaction, reversal and concatenation.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Data.Array.Edit</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Array.Edit/source/">View source</a><a href="/packages/search/?q=%2BData.Array.Edit">Search module</a><a href="/reference-sources/Data.Array.Edit.cinch" download="Data.Array.Edit.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Array](Data.Array.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Relation.Eq](Relation.Eq.md).

<details class="doc-closure"><summary>Ship-source closure (2 files)</summary>

Copy these modules beside your entry file before importing `Data.Array.Edit`: [Data.Array](Data.Array.md), [Data.Array.Edit](Data.Array.Edit.md).

```sh
cp seed/prelude/Data/Array.cinch ./Data.Array.cinch
cp seed/prelude/Data/Array/Edit.cinch ./Data.Array.Edit.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Persistent edits over immutable arrays. Half-open ranges are checked
before constructing a result, and the original arrays remain usable.

</details>

<details class="doc-synopsis" open><summary>Synopsis (8 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-array-compact">array-compact</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-compact</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">equal</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-array-compact-by">array-compact-by</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-compact-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">e</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-array-concat">array-concat</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-concat</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-array-delete-by">array-delete-by</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-delete-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">e</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-array-delete-range">array-delete-range</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-delete-range</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-array-insert">array-insert</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-insert</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-array-replace-range">array-replace-range</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-replace-range</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-array-reverse">array-reverse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">array-reverse</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-array-compact">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-compact</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">equal</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-array-compact" title="Permalink">#</a><a href="/packages/stdlib/Data.Array.Edit/source/#L57" title="View source at line 57">Source</a></span></div>
<div class="doc-body">

Remove adjacent duplicate values using element equality.

</div>
</article>

<article class="doc-entry" id="v-array-compact-by">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-compact-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">e</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-array-compact-by" title="Permalink">#</a><a href="/packages/stdlib/Data.Array.Edit/source/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

Keep the first element of each adjacent run. Compare each current
element with its immediate predecessor, passing the current element first.

</div>
</article>

<article class="doc-entry" id="v-array-concat">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-concat</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-array-concat" title="Permalink">#</a><a href="/packages/stdlib/Data.Array.Edit/source/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">

Concatenate arrays in order, walking each input once. The temporary list
keeps earlier prefixes from being copied for every subsequent input.

</div>
</article>

<article class="doc-entry" id="v-array-delete-by">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-delete-by</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">e</span>. (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{e} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-array-delete-by" title="Permalink">#</a><a href="/packages/stdlib/Data.Array.Edit/source/#L34" title="View source at line 34">Source</a></span></div>
<div class="doc-body">

Delete each matching element. Run the predicate once per element from left to right.

</div>
</article>

<article class="doc-entry" id="v-array-delete-range">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-delete-range</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-array-delete-range" title="Permalink">#</a><a href="/packages/stdlib/Data.Array.Edit/source/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">

Delete exactly the elements in the half-open range.

</div>
</article>

<article class="doc-entry" id="v-array-insert">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-insert</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-array-insert" title="Permalink">#</a><a href="/packages/stdlib/Data.Array.Edit/source/#L26" title="View source at line 26">Source</a></span></div>
<div class="doc-body">

Insert all new elements at an index in the closed range from zero to length.

</div>
</article>

<article class="doc-entry" id="v-array-replace-range">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-replace-range</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-array-replace-range" title="Permalink">#</a><a href="/packages/stdlib/Data.Array.Edit/source/#L15" title="View source at line 15">Source</a></span></div>
<div class="doc-body">

Replace [start,end) with the replacement's elements. Both bounds may
equal the array length; reversed or out-of-range bounds return None.

</div>
</article>

<article class="doc-entry" id="v-array-reverse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">array-reverse</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-array-reverse" title="Permalink">#</a><a href="/packages/stdlib/Data.Array.Edit/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

Return the elements in reverse order.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Array/Edit.cinch`](/packages/stdlib/Data.Array.Edit/source/). Source SHA-256: `25857a7cd5dff21d743ddb2871c837c770397ac57e05d524349fc39ccc780bdb`.
