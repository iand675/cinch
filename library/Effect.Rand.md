# Effect.Rand

Pseudorandom sources seeded from operating-system entropy.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Rand</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Rand/source/">View source</a><a href="/packages/search/?q=%2BEffect.Rand">Search module</a><a href="/reference-sources/Effect.Rand.cinch" download="Effect.Rand.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.ChaCha8](Data.ChaCha8.md), [Data.Rand](Data.Rand.md), [Data.Result](Data.Result.md), [Effect.Random](Effect.Random.md).

<details class="doc-notes"><summary>Module notes</summary>

Pseudorandom sources seeded from the operating system: the effectful entry to Data.Rand.

Each call draws fresh seed material from Effect.Random's entropy source (128 bits for PCG, 256 for ChaCha8),
so two sources made this way are independent and unpredictable; drawing from them is then pure and
replayable (keep the source, or its encoding, to replay). Use a ChaCha8 source when outputs must also be
hard to predict from earlier outputs.

</details>

<details class="doc-synopsis" open><summary>Synopsis (2 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-random-chacha8">random-chacha8</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">random-chacha8</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8Error" title="Data.ChaCha8.ChaCha8Error">ChaCha8Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a></code></li><li><a class="syn-name" href="#v-random-pcg">random-pcg</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">random-pcg</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-random-chacha8">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">random-chacha8</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8Error" title="Data.ChaCha8.ChaCha8Error">ChaCha8Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a></code><span class="doc-sig-links"><a href="#v-random-chacha8" title="Permalink">#</a><a href="/packages/stdlib/Effect.Rand/source/#L28" title="View source at line 28">Source</a></span></div>
<div class="doc-body">

A ChaCha8 generator seeded with 256 bits of operating-system entropy (the seed always has the required
32 bytes, so the result is Ok whenever the entropy source answers).

</div>
</article>

<article class="doc-entry" id="v-random-pcg">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">random-pcg</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a></code><span class="doc-sig-links"><a href="#v-random-pcg" title="Permalink">#</a><a href="/packages/stdlib/Effect.Rand/source/#L23" title="View source at line 23">Source</a></span></div>
<div class="doc-body">

A PCG generator seeded with 128 bits of operating-system entropy.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Rand.cinch`](/packages/stdlib/Effect.Rand/source/). Source SHA-256: `1849a085f5e7c61bd4d30491b389f120737eeaee6049ff0839e428e717b7efdf`.
