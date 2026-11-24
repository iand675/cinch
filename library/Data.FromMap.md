# Data.FromMap

The opt-in map-literal vocabulary used to interpret {key => value} at the expected container type.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Data.FromMap</code></div><div class="doc-mod-bar-links"><a href="/library/source/Data.FromMap/">View source</a><a href="/library/search/?q=%2BData.FromMap">Search module</a><a href="/reference-sources/Data.FromMap.cinch" download="Data.FromMap.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.List](Data.List.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.FromMap: the map-literal construction class.

`{k => v}` desugars to `from-map` over a `List (k, v)` of pairs at
the expected type's `FromMap m k v` instance; unconstrained it
defaults to `Map` (surface grammar local amendments section 10).
Instances live with their type (Data.Map, Data.HashMap).

</details>

<details class="doc-synopsis" open><summary>Synopsis (2 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#c-FromMap">FromMap</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">FromMap</strong> <span class="tok-variable">m</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-FromMap-from-map">from-map</a><code class="syn-sig"><strong class="tok-variable">from-map</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">m</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.FromMap/#c-FromMap" title="Data.FromMap.FromMap">FromMap</a> <span class="tok-variable">m</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">m</span></code></li></ul>
</details>

# Signatures

<article class="doc-entry" id="c-FromMap">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">FromMap</strong> <span class="tok-variable">m</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#c-FromMap" title="Permalink">#</a><a href="/library/source/Data.FromMap/#L13" title="View source at line 13">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-FromMap-from-map"><div class="doc-method-row"><code><strong class="tok-variable">from-map</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">m</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.FromMap/#c-FromMap" title="Data.FromMap.FromMap">FromMap</a> <span class="tok-variable">m</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">m</span></code><span class="doc-sig-links"><a href="#m-FromMap-from-map" title="Permalink">#</a><a href="/library/source/Data.FromMap/#L14">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Data.FromMap/#c-FromMap" title="Data.FromMap.FromMap">FromMap</a> (<a class="doc-type-link tok-constructor" href="/library/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) <span class="tok-variable">k</span> <span class="tok-variable">v</span></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.HashMap/#instances">Data.HashMap</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.FromMap/#c-FromMap" title="Data.FromMap.FromMap">FromMap</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) <span class="tok-variable">k</span> <span class="tok-variable">v</span></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Map/#instances">Data.Map</a></span></li></ul></details>
</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/FromMap.cinch`](/library/source/Data.FromMap/). Source SHA-256: `af8cdaf5ec98b0c87e4c9b2c85fabfa72cb0602ea5f14ba50da7ee9e99311fcf`.
