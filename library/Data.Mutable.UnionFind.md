# Data.Mutable.UnionFind

Disjoint sets with union by size and path halving: near-constant find, union and same-set queries, and appendable elements.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Mutable.UnionFind</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Mutable.UnionFind/source/">View source</a><a href="/packages/search/?q=%2BData.Mutable.UnionFind">Search module</a><a href="/reference-sources/Data.Mutable.UnionFind.cinch" download="Data.Mutable.UnionFind.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Mutable.UnionFind: disjoint sets (union-find), as in every
compiler's type unifier and every Kruskal.

Elements are the Nats 0 .. n - 1, `union-find-add` appends one more.
Each element keeps a parent word and each root its set's size, both in
raw slot arrays. `union-find-find` halves paths as it walks (every node
on the way is pointed at its grandparent), and `union-find-union` hangs
the smaller set under the larger, which together make a sequence of m
operations cost O(m alpha(n)), effectively constant per operation.

The trusted core is the private `cinch.runtime.slots*` imports (the
compiler's `slots.rs`); none of them leaves the module. Every word below
the element count is written when its element is added.

</details>

<details class="doc-synopsis" open><summary>Synopsis (9 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-UnionFind">UnionFind</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnionFind</strong> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-union-find-add">union-find-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">union-find-add</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-UnionFind" title="Data.Mutable.UnionFind.UnionFind">UnionFind</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-union-find-find">union-find-find</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">union-find-find</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-UnionFind" title="Data.Mutable.UnionFind.UnionFind">UnionFind</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-union-find-length">union-find-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">union-find-length</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-UnionFind" title="Data.Mutable.UnionFind.UnionFind">UnionFind</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-union-find-new">union-find-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">union-find-new</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-UnionFind" title="Data.Mutable.UnionFind.UnionFind">UnionFind</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-union-find-same">union-find-same</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">union-find-same</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-UnionFind" title="Data.Mutable.UnionFind.UnionFind">UnionFind</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-union-find-set-count">union-find-set-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">union-find-set-count</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-UnionFind" title="Data.Mutable.UnionFind.UnionFind">UnionFind</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-union-find-set-size">union-find-set-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">union-find-set-size</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-UnionFind" title="Data.Mutable.UnionFind.UnionFind">UnionFind</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-union-find-union">union-find-union</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">union-find-union</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-UnionFind" title="Data.Mutable.UnionFind.UnionFind">UnionFind</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-UnionFind">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UnionFind</strong> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#t-UnionFind" title="Permalink">#</a><a href="/packages/stdlib/Data.Mutable.UnionFind/source/#L33" title="View source at line 33">Source</a></span></div>
<div class="doc-body">

The header (element count, set count) and one cell holding the
words: parent of element i at 2i, size of the set rooted at i at
2i + 1.

</div>
</article>

# Values

<article class="doc-entry" id="v-union-find-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">union-find-add</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-UnionFind" title="Data.Mutable.UnionFind.UnionFind">UnionFind</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-union-find-add" title="Permalink">#</a><a href="/packages/stdlib/Data.Mutable.UnionFind/source/#L75" title="View source at line 75">Source</a></span></div>
<div class="doc-body">

A new singleton set: its element.

</div>
</article>

<article class="doc-entry" id="v-union-find-find">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">union-find-find</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-UnionFind" title="Data.Mutable.UnionFind.UnionFind">UnionFind</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-union-find-find" title="Permalink">#</a><a href="/packages/stdlib/Data.Mutable.UnionFind/source/#L110" title="View source at line 110">Source</a></span></div>
<div class="doc-body">

The representative of x's set (`None` when x is not an element).

</div>
</article>

<article class="doc-entry" id="v-union-find-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">union-find-length</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-UnionFind" title="Data.Mutable.UnionFind.UnionFind">UnionFind</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-union-find-length" title="Permalink">#</a><a href="/packages/stdlib/Data.Mutable.UnionFind/source/#L101" title="View source at line 101">Source</a></span></div>
</article>

<article class="doc-entry" id="v-union-find-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">union-find-new</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-UnionFind" title="Data.Mutable.UnionFind.UnionFind">UnionFind</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-union-find-new" title="Permalink">#</a><a href="/packages/stdlib/Data.Mutable.UnionFind/source/#L60" title="View source at line 60">Source</a></span></div>
<div class="doc-body">

`n` singleton sets.

</div>
</article>

<article class="doc-entry" id="v-union-find-same">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">union-find-same</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-UnionFind" title="Data.Mutable.UnionFind.UnionFind">UnionFind</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-union-find-same" title="Permalink">#</a><a href="/packages/stdlib/Data.Mutable.UnionFind/source/#L156" title="View source at line 156">Source</a></span></div>
</article>

<article class="doc-entry" id="v-union-find-set-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">union-find-set-count</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-UnionFind" title="Data.Mutable.UnionFind.UnionFind">UnionFind</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-union-find-set-count" title="Permalink">#</a><a href="/packages/stdlib/Data.Mutable.UnionFind/source/#L105" title="View source at line 105">Source</a></span></div>
<div class="doc-body">

The number of disjoint sets.

</div>
</article>

<article class="doc-entry" id="v-union-find-set-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">union-find-set-size</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-UnionFind" title="Data.Mutable.UnionFind.UnionFind">UnionFind</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-union-find-set-size" title="Permalink">#</a><a href="/packages/stdlib/Data.Mutable.UnionFind/source/#L165" title="View source at line 165">Source</a></span></div>
<div class="doc-body">

The size of x's set (0 when x is not an element).

</div>
</article>

<article class="doc-entry" id="v-union-find-union">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">union-find-union</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-UnionFind" title="Data.Mutable.UnionFind.UnionFind">UnionFind</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Mut">Mut</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-union-find-union" title="Permalink">#</a><a href="/packages/stdlib/Data.Mutable.UnionFind/source/#L130" title="View source at line 130">Source</a></span></div>
<div class="doc-body">

Merge the sets of x and y: `True` when they were separate (and both
are elements).

</div>
</article>
