# Data.FromList

The opt-in collection-literal vocabulary used to interpret [elements] at the expected container type.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Data.FromList</code></div><div class="doc-mod-bar-links"><a href="/library/source/Data.FromList/">View source</a><a href="/library/search/?q=%2BData.FromList">Search module</a><a href="/reference-sources/Data.FromList.cinch" download="Data.FromList.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Core](Data.Core.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.FromList: the collection-literal construction class.

`[a, b, c]` desugars to `from-list` at the expected type's
`FromList c e` instance; unconstrained it defaults to `List`
(surface grammar local amendments section 10). Instances opt in per
container and live with their type (Data.List, Data.Bytes,
Data.Set).

</details>

<details class="doc-synopsis" open><summary>Synopsis (2 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#c-FromList">FromList</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">FromList</strong> <span class="tok-variable">c</span> <span class="tok-variable">e</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-FromList-from-list">from-list</a><code class="syn-sig"><strong class="tok-variable">from-list</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">c</span> <span class="tok-variable">e</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.FromList/#c-FromList" title="Data.FromList.FromList">FromList</a> <span class="tok-variable">c</span> <span class="tok-variable">e</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span></code></li></ul>
</details>

# Signatures

<article class="doc-entry" id="c-FromList">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">FromList</strong> <span class="tok-variable">c</span> <span class="tok-variable">e</span></code><span class="doc-sig-links"><a href="#c-FromList" title="Permalink">#</a><a href="/library/source/Data.FromList/#L19" title="View source at line 19">Source</a></span></div>
<div class="doc-body">

Collection literals (surface grammar local amendments, section 10):
`[a, b, c]` desugars to `from-list` at the expected type's `FromList
c e` instance and `{k => v}` to `from-map` over a `List (k, v)` of
pairs at the expected type's `FromMap m k v` instance; unconstrained
they default to `List` and `Map`. Instances opt in per container.

<div class="doc-children"><h4>Methods</h4><ul><li id="m-FromList-from-list"><div class="doc-method-row"><code><strong class="tok-variable">from-list</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">c</span> <span class="tok-variable">e</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.FromList/#c-FromList" title="Data.FromList.FromList">FromList</a> <span class="tok-variable">c</span> <span class="tok-variable">e</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span></code><span class="doc-sig-links"><a href="#m-FromList-from-list" title="Permalink">#</a><a href="/library/source/Data.FromList/#L20">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (4)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Data.FromList/#c-FromList" title="Data.FromList.FromList">FromList</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span>) <span class="tok-variable">a</span></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Array/#instances">Data.Array</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.FromList/#c-FromList" title="Data.FromList.FromList">FromList</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U8">U8</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Bytes/#instances">Data.Bytes</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.FromList/#c-FromList" title="Data.FromList.FromList">FromList</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">e</span>) <span class="tok-variable">e</span></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.FromList/#c-FromList" title="Data.FromList.FromList">FromList</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span>) <span class="tok-variable">a</span></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Set/#instances">Data.Set</a></span></li></ul></details>
</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/FromList.cinch`](/library/source/Data.FromList/). Source SHA-256: `62cc79932ad61b2dc2a269fa592573ca8d514cc0f99ff34ddbf0be60160fd3d8`.
