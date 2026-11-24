# Effect.Buffered.Internal.Cell

Initialized opaque continuation cells whose allocation, observation and replacement require their I/O region.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Buffered.Internal.Cell</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Buffered.Internal.Cell/source/">View source</a><a href="/packages/search/?q=%2BEffect.Buffered.Internal.Cell">Search module</a><a href="/reference-sources/Effect.Buffered.Internal.Cell.cinch" download="Effect.Buffered.Internal.Cell.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

<details class="doc-notes"><summary>Module notes</summary>

Internal managed continuation cells whose allocation, reads and writes require I/O.
The sole slot is initialized before a cell can be observed. Neither its array
nor a raw address is exported; the runtime retains and releases stored values.

</details>

<details class="doc-synopsis" open><summary>Synopsis (4 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-IoCell">IoCell</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">IoCell</strong> <span class="tok-variable">r</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-cell-new">cell-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cell-new</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Internal.Cell/#t-IoCell" title="Effect.Buffered.Internal.Cell.IoCell">IoCell</a> <span class="tok-variable">r</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-cell-read">cell-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cell-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Internal.Cell/#t-IoCell" title="Effect.Buffered.Internal.Cell.IoCell">IoCell</a> <span class="tok-variable">r</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-cell-write">cell-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cell-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Internal.Cell/#t-IoCell" title="Effect.Buffered.Internal.Cell.IoCell">IoCell</a> <span class="tok-variable">r</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-IoCell">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">IoCell</strong> <span class="tok-variable">r</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-IoCell" title="Permalink">#</a><a href="/packages/stdlib/Effect.Buffered.Internal.Cell/source/#L17" title="View source at line 17">Source</a></span></div>
<div class="doc-body">

An initialized managed slot, scoped by its I/O region and value type.

</div>
</article>

# Values

<article class="doc-entry" id="v-cell-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cell-new</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Internal.Cell/#t-IoCell" title="Effect.Buffered.Internal.Cell.IoCell">IoCell</a> <span class="tok-variable">r</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-cell-new" title="Permalink">#</a><a href="/packages/stdlib/Effect.Buffered.Internal.Cell/source/#L22" title="View source at line 22">Source</a></span></div>
<div class="doc-body">

Allocate exactly one slot and initialize it before returning the opaque cell.

</div>
</article>

<article class="doc-entry" id="v-cell-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cell-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Internal.Cell/#t-IoCell" title="Effect.Buffered.Internal.Cell.IoCell">IoCell</a> <span class="tok-variable">r</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-cell-read" title="Permalink">#</a><a href="/packages/stdlib/Effect.Buffered.Internal.Cell/source/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">

Retain and observe the cell's current value through the managed slot operation.

</div>
</article>

<article class="doc-entry" id="v-cell-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cell-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Internal.Cell/#t-IoCell" title="Effect.Buffered.Internal.Cell.IoCell">IoCell</a> <span class="tok-variable">r</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-cell-write" title="Permalink">#</a><a href="/packages/stdlib/Effect.Buffered.Internal.Cell/source/#L31" title="View source at line 31">Source</a></span></div>
<div class="doc-body">

Replace the initialized value, retaining the replacement and releasing the previous value.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Buffered/Internal/Cell.cinch`](/packages/stdlib/Effect.Buffered.Internal.Cell/source/). Source SHA-256: `3c33309272f788521ec17bf6fa60ccd331afdb311049bb2977cedaf0ac067564`.
