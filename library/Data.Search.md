# Data.Search

Indexed binary search with monotone predicates, insertion positions and effectful three-way comparisons.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Search</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Search/source/">View source</a><a href="/packages/search/?q=%2BData.Search">Search module</a><a href="/reference-sources/Data.Search.cinch" download="Data.Search.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Nat](Data.Nat.md), [Data.Ordering](Data.Ordering.md).

<details class="doc-notes"><summary>Module notes</summary>

Binary search over natural-number indices. Callbacks may compute, read or
compare elements on demand. Only indices below the supplied size are probed.

</details>

<details class="doc-synopsis" open><summary>Synopsis (2 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-search-find">search-find</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">search-find</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Row</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{|e} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a>) <span class="tok-operator">-&gt;</span>{|e} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code></li><li><a class="syn-name" href="#v-search-first">search-first</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">search-first</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Row</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{|e} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span>{|e} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-search-find">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">search-find</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Row</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{|e} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a>) <span class="tok-operator">-&gt;</span>{|e} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code><span class="doc-sig-links"><a href="#v-search-find" title="Permalink">#</a><a href="/packages/stdlib/Data.Search/source/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">

Find the first equal element or its insertion index. The callback compares
the sought key to the indexed element, returning GT, then EQ, then LT across
an ascending sequence. Empty ranges perform no comparisons.

</div>
</article>

<article class="doc-entry" id="v-search-first">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">search-first</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Row</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{|e} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>) <span class="tok-operator">-&gt;</span>{|e} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-search-first" title="Permalink">#</a><a href="/packages/stdlib/Data.Search/source/#L12" title="View source at line 12">Source</a></span></div>
<div class="doc-body">

Return the first index whose predicate is true, or the size when none
qualifies. The predicate must be false on a prefix and true thereafter.

</div>
</article>
