# Effect.Uuid

Secure UUID generation with explicit serialized version 7 ownership.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Uuid</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Uuid/source/">View source</a><a href="/packages/search/?q=%2BEffect.Uuid">Search module</a><a href="/reference-sources/Effect.Uuid.cinch" download="Effect.Uuid.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Core](Data.Core.md), [Data.Nat](Data.Nat.md), [Data.Uuid](Data.Uuid.md), [Effect.Clock](Effect.Clock.md), [Effect.Concurrent.MVar](Effect.Concurrent.MVar.md), [Effect.Random](Effect.Random.md).

<details class="doc-notes"><summary>Module notes</summary>

Secure UUID generation with explicit serialized version 7 ownership.
Entropy comes from the system random capability and time from the wall clock.
One shared generator orders timestamp transitions; separate generators define
independent sequences. Entropy acquisition occurs after releasing the state
cell, so overlapping calls are ordered at that transition rather than by
completion time. Plain blocking operations preserve the full effect row.

</details>

<details class="doc-synopsis" open><summary>Synopsis (5 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-UuidV7Generator">UuidV7Generator</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UuidV7Generator</strong></code></li><li><a class="syn-name" href="#v-uuid-new">uuid-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-new</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code></li><li><a class="syn-name" href="#v-uuid-new-v4">uuid-new-v4</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-new-v4</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code></li><li><a class="syn-name" href="#v-uuid-new-v7">uuid-new-v7</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-new-v7</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Uuid/#t-UuidV7Generator" title="Effect.Uuid.UuidV7Generator">UuidV7Generator</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code></li><li><a class="syn-name" href="#v-uuid-v7-generator">uuid-v7-generator</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uuid-v7-generator</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Uuid/#t-UuidV7Generator" title="Effect.Uuid.UuidV7Generator">UuidV7Generator</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-UuidV7Generator">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UuidV7Generator</strong></code><span class="doc-sig-links"><a href="#t-UuidV7Generator" title="Permalink">#</a><a href="/packages/stdlib/Effect.Uuid/source/#L18" title="View source at line 18">Source</a></span></div>
<div class="doc-body">

One explicitly shared serialized timestamp sequence with no public access to its state cell.

</div>
</article>

# Values

<article class="doc-entry" id="v-uuid-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-new</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code><span class="doc-sig-links"><a href="#v-uuid-new" title="Permalink">#</a><a href="/packages/stdlib/Effect.Uuid/source/#L23" title="View source at line 23">Source</a></span></div>
<div class="doc-body">

Generate the current default version 4 value using 16 fresh secure random bytes.

</div>
</article>

<article class="doc-entry" id="v-uuid-new-v4">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-new-v4</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code><span class="doc-sig-links"><a href="#v-uuid-new-v4" title="Permalink">#</a><a href="/packages/stdlib/Effect.Uuid/source/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">

Acquire real secure entropy and preserve 122 random bits after version/variant masking.

</div>
</article>

<article class="doc-entry" id="v-uuid-new-v7">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-new-v7</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Uuid/#t-UuidV7Generator" title="Effect.Uuid.UuidV7Generator">UuidV7Generator</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-UuidError" title="Data.Uuid.UuidError">UuidError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code><span class="doc-sig-links"><a href="#v-uuid-new-v7" title="Permalink">#</a><a href="/packages/stdlib/Effect.Uuid/source/#L38" title="View source at line 38">Source</a></span></div>
<div class="doc-body">

Sample real wall time under the state lock, commit its transition, then acquire eight secure random bytes.
A whole-second clock rollback resets the ordering constraint. Same-second
rollback and equal timestamps advance the logical fraction monotonically.
Checked clock refusals restore the old state before returning an error.

</div>
</article>

<article class="doc-entry" id="v-uuid-v7-generator">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uuid-v7-generator</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Uuid/#t-UuidV7Generator" title="Effect.Uuid.UuidV7Generator">UuidV7Generator</a></code><span class="doc-sig-links"><a href="#v-uuid-v7-generator" title="Permalink">#</a><a href="/packages/stdlib/Effect.Uuid/source/#L31" title="View source at line 31">Source</a></span></div>
<div class="doc-body">

Allocate one serialized version 7 sequence; retain and share this handle for shared ordering.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Uuid.cinch`](/packages/stdlib/Effect.Uuid/source/). Source SHA-256: `fd4fb84c957afb908f2fb7d8cf1fb4ea1416fd09ec64b4a376cfd3a526cc55a3`.
