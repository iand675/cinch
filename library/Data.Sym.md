# Data.Sym

Interned symbols with identity equality, deterministic spelling order, and String conversions.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Sym</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Sym/source/">View source</a><a href="/packages/search/?q=%2BData.Sym">Search module</a><a href="/reference-sources/Data.Sym.cinch" download="Data.Sym.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Hash](Data.Hash.md), [Data.String](Data.String.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Sym: interned symbols.

`Sym` is a core runtime type (primitive types specification section
9): one word, a handle to a process-wide interned name cell. Two
equal names are the same cell, so `Eq` is identity (an O(1) pointer
test) and `Ord` is lexicographic by name (deterministic across runs;
never insertion order). `Show` prints `'foo`. Nothing here exposes a
cell address as data: the foreign boundary takes `sym-name` (or
`Bytes`), and the type has no C counterpart.

Literals are constants first: `'foo` interns at compile time and is
emitted as a constant (the P14 constant-emission path), and `intern`
of an equal name at runtime resolves to the same cell (the runtime
table is keyed by name). Symbol literal patterns test identity.

Module scheme (design/base-library.md): the type's home carries its
instance family (the `Eq`, `Ord` and `Show` instances over the
runtime cells).

</details>

<details class="doc-synopsis" open><summary>Synopsis (2 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-intern">intern</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">intern</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sym/" title="Sym">Sym</a></code></li><li><a class="syn-name" href="#v-sym-name">sym-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sym-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sym/" title="Sym">Sym</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-intern">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">intern</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sym/" title="Sym">Sym</a></code><span class="doc-sig-links"><a href="#v-intern" title="Permalink">#</a><a href="/packages/stdlib/Data.Sym/source/#L39" title="View source at line 39">Source</a></span></div>
<div class="doc-body">

The prelude surface of section 9.

</div>
</article>

<article class="doc-entry" id="v-sym-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sym-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sym/" title="Sym">Sym</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-sym-name" title="Permalink">#</a><a href="/packages/stdlib/Data.Sym/source/#L42" title="View source at line 42">Source</a></span></div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_Sym"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sym/" title="Sym">Sym</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Sym.Eq_Sym</code></span><div class="doc-child-prose">Eq is identity: the same interned cell (section 9).</div></li>
<li id="i-Hash_Sym"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sym/" title="Sym">Sym</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Sym.Hash_Sym</code></span><div class="doc-child-prose">The digest is the interned name's bytes through the Bytes instance (Data.Bytes owns the fold), read straight off the intrinsic layer with no String in between.</div></li>
<li id="i-Ord_Sym"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sym/" title="Sym">Sym</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Sym.Ord_Sym</code></span><div class="doc-child-prose">Ord is lexicographic by name: symCmp's immediate against 0 (the natCmp/intCmp reading), never the cell addresses.</div></li>
<li id="i-Show_Sym"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Sym/" title="Sym">Sym</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Sym.Show_Sym</code></span><div class="doc-child-prose">Show prints 'foo (section 9): the tick is char-string '\'' (raw ASCII 39), the name renders as itself.</div></li>
</ul>
