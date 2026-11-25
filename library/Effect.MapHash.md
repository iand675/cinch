# Effect.MapHash

Operating-system entropy seed generation for Data.MapHash.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.MapHash</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.MapHash/source/">View source</a><a href="/packages/search/?q=%2BEffect.MapHash">Search module</a><a href="/reference-sources/Effect.MapHash.cinch" download="Effect.MapHash.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.MapHash](Data.MapHash.md), [Data.Nat](Data.Nat.md), [Data.Wide](Data.Wide.md), [Effect.Random](Effect.Random.md).

<details class="doc-notes"><summary>Module notes</summary>

Operating-system entropy seeding for Data.MapHash.

Each call to maphash-make-seed draws 128 bits of fresh entropy from Effect.Random so
independent hash tables obtain distinct, unpredictable keys. All hashing after seed
creation remains pure and deterministic.

</details>

<details class="doc-synopsis" open><summary>Synopsis (1 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-maphash-make-seed">maphash-make-seed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-make-seed</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-maphash-make-seed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-make-seed</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a></code><span class="doc-sig-links"><a href="#v-maphash-make-seed" title="Permalink">#</a><a href="/packages/stdlib/Effect.MapHash/source/#L23" title="View source at line 23">Source</a></span></div>
<div class="doc-body">

Draw 128 bits of operating-system entropy to produce an independent MapSeed.

</div>
</article>
