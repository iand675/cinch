# Data.Enum

Finite enumeration and the class behind inclusive range literals, including an explicit next element for a step.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Enum</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Enum/source/">View source</a><a href="/packages/search/?q=%2BData.Enum">Search module</a><a href="/reference-sources/Data.Enum.cinch" download="Data.Enum.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Core](Data.Core.md), [Data.List](Data.List.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Enum: the enumeration class behind the range literals.

`[e1..e2]` desugars to `from-list (enum-from-to e1 e2)` and
`[e1, e2 .. e3]` to `from-list (enum-from-then-to e1 e2 e3)` at the
expected type's `FromList c e` instance (the collection-literal
machinery of the surface grammar local amendments, section 10): the
container comes from the expected type, the enumeration steps through
this class, and a stage-known construction folds to a constant value
at compile time like every collection literal.

Module scheme (design/base-library.md): the class and the range
builders live here; instances live with their type (Data.Int,
Data.Nat). The enumeration codes are consecutive (`to-enum`/
`from-enum` are mutually inverse and neighbors differ by one), so the
range builders walk codes. A bounded instance answers `None` past
its edges (`pred` of its least value, `to-enum` of a code outside
the type), so no method faults (design/no-faults.md section 3); a
range stops where its codes leave the type.

</details>

<details class="doc-synopsis" open><summary>Synopsis (7 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#c-Enum">Enum</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Enum</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Enum-succ">succ</a><code class="syn-sig"><strong class="tok-variable">succ</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Enum-pred">pred</a><code class="syn-sig"><strong class="tok-variable">pred</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Enum-to-enum">to-enum</a><code class="syn-sig"><strong class="tok-variable">to-enum</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Enum-from-enum">from-enum</a><code class="syn-sig"><strong class="tok-variable">from-enum</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-enum-from-then-to">enum-from-then-to</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">enum-from-then-to</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">e</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-enum-from-to">enum-from-to</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">enum-from-to</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">e</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li></ul>
</details>

# Signatures

<article class="doc-entry" id="c-Enum">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Enum</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Enum" title="Permalink">#</a><a href="/packages/stdlib/Data.Enum/source/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Enum-succ"><div class="doc-method-row"><code><strong class="tok-variable">succ</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Enum-succ" title="Permalink">#</a><a href="/packages/stdlib/Data.Enum/source/#L31">Source</a></span></div></li><li id="m-Enum-pred"><div class="doc-method-row"><code><strong class="tok-variable">pred</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Enum-pred" title="Permalink">#</a><a href="/packages/stdlib/Data.Enum/source/#L32">Source</a></span></div></li><li id="m-Enum-to-enum"><div class="doc-method-row"><code><strong class="tok-variable">to-enum</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Enum-to-enum" title="Permalink">#</a><a href="/packages/stdlib/Data.Enum/source/#L33">Source</a></span></div></li><li id="m-Enum-from-enum"><div class="doc-method-row"><code><strong class="tok-variable">from-enum</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#m-Enum-from-enum" title="Permalink">#</a><a href="/packages/stdlib/Data.Enum/source/#L34">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Int/#instances">Data.Int</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Nat/#instances">Data.Nat</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-enum-from-then-to">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">enum-from-then-to</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">e</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-enum-from-then-to" title="Permalink">#</a><a href="/packages/stdlib/Data.Enum/source/#L54" title="View source at line 54">Source</a></span></div>
<div class="doc-body">

`[lo, next .. hi]`'s builder: the codes from `lo` toward `hi`
inclusive, step `next - lo`.

</div>
</article>

<article class="doc-entry" id="v-enum-from-to">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">enum-from-to</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">e</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-enum-from-to" title="Permalink">#</a><a href="/packages/stdlib/Data.Enum/source/#L49" title="View source at line 49">Source</a></span></div>
<div class="doc-body">

`[lo..hi]`'s builder: the codes from `lo` to `hi` inclusive, step
one.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Enum.cinch`](/packages/stdlib/Data.Enum/source/). Source SHA-256: `ebd926c5608f05da69741e21a3ef67ee563ed92d3622d7bbd50e860ec51818e8`.
