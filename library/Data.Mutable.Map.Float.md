# Data.Mutable.Map.Float

Explicit IEEE floating key policies without global equality or hashing instances.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Mutable.Map.Float</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Mutable.Map.Float/source/">View source</a><a href="/packages/search/?q=%2BData.Mutable.Map.Float">Search module</a><a href="/reference-sources/Data.Mutable.Map.Float.cinch" download="Data.Mutable.Map.Float.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Machine](Data.Machine.md), [Data.Mutable.Map](Data.Mutable.Map.md), [Data.Nat](Data.Nat.md).

<details class="doc-notes"><summary>Module notes</summary>

Explicit IEEE floating key policies without global equality or hashing instances.
Signed zeros share a digest and compare equally. NaNs remain non-reflexive:
they can be stored and enumerated but cannot be found or deleted by key.

</details>

<details class="doc-synopsis" open><summary>Synopsis (2 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-map-f32-ieee-policy">map-f32-ieee-policy</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-f32-ieee-policy</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable.Map/#t-MapKeyPolicy" title="Data.Mutable.Map.MapKeyPolicy">MapKeyPolicy</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code></li><li><a class="syn-name" href="#v-map-f64-ieee-policy">map-f64-ieee-policy</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-f64-ieee-policy</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable.Map/#t-MapKeyPolicy" title="Data.Mutable.Map.MapKeyPolicy">MapKeyPolicy</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-map-f32-ieee-policy">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-f32-ieee-policy</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable.Map/#t-MapKeyPolicy" title="Data.Mutable.Map.MapKeyPolicy">MapKeyPolicy</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code><span class="doc-sig-links"><a href="#v-map-f32-ieee-policy" title="Permalink">#</a><a href="/packages/stdlib/Data.Mutable.Map.Float/source/#L42" title="View source at line 42">Source</a></span></div>
<div class="doc-body">

Select binary32 IEEE equality explicitly, including non-reflexive NaN keys.

</div>
</article>

<article class="doc-entry" id="v-map-f64-ieee-policy">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-f64-ieee-policy</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable.Map/#t-MapKeyPolicy" title="Data.Mutable.Map.MapKeyPolicy">MapKeyPolicy</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-map-f64-ieee-policy" title="Permalink">#</a><a href="/packages/stdlib/Data.Mutable.Map.Float/source/#L46" title="View source at line 46">Source</a></span></div>
<div class="doc-body">

Select binary64 IEEE equality explicitly, including equivalent signed zeros.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Mutable/Map/Float.cinch`](/packages/stdlib/Data.Mutable.Map.Float/source/). Source SHA-256: `ae5cf39b89f2d062b225740c1b5105ed47934a52755a5886c6705f005a4f317a`.
