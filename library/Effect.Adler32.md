# Effect.Adler32

Inspectable scalar checksum sinks, generic byte writers and bounded reader-to-Adler-32 copying with advanced source continuations.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Adler32</code></div><div class="doc-mod-bar-links"><a href="/library/source/Effect.Adler32/">View source</a><a href="/library/search/?q=%2BEffect.Adler32">Search module</a><a href="/reference-sources/Effect.Adler32.cinch" download="Effect.Adler32.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Effect.Stream](Effect.Stream.md), [Text.Adler32](Text.Adler32.md).

<details class="doc-closure"><summary>Ship-source closure (5 files)</summary>

Copy these modules beside your entry file before importing `Effect.Adler32`: [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Adler32](Effect.Adler32.md), [Effect.Stream](Effect.Stream.md), [Text.Adler32](Text.Adler32.md), [Text.Binary](Text.Binary.md).

```sh
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Effect/Adler32.cinch ./Effect.Adler32.cinch
cp seed/prelude/Effect/Stream.cinch ./Effect.Stream.cinch
cp seed/prelude/Text/Adler32.cinch ./Text.Adler32.cinch
cp seed/prelude/Text/Binary.cinch ./Text.Binary.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Inspectable Adler-32 hashing sinks and bounded reader-to-checksum copying.
The immutable scalar state is the sink handle. Writes accept every byte and
never retain input buffers. Copying returns the advanced reader and the last
valid checksum on failure, and never closes a borrowed source.

</details>

<details class="doc-synopsis" open><summary>Synopsis (3 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-adler32-copy">adler32-copy</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32-copy</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, (<a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span>, <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a>))</code></li><li><a class="syn-name" href="#v-adler32-hashing-writer">adler32-hashing-writer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32-hashing-writer</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-adler32-write">adler32-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a>)</code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-adler32-copy">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32-copy</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, (<a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span>, <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a>))</code><span class="doc-sig-links"><a href="#v-adler32-copy" title="Permalink">#</a><a href="/library/source/Effect.Adler32/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">

Hash successful reader chunks through EOF, preserving both advanced handles on errors.
A zero chunk size is rejected before reading. Invalid source counts do not enter the checksum.

</div>
</article>

<article class="doc-entry" id="v-adler32-hashing-writer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32-hashing-writer</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-adler32-hashing-writer" title="Permalink">#</a><a href="/library/source/Effect.Adler32/#L21" title="View source at line 21">Source</a></span></div>
<div class="doc-body">

Adapt a checksum state to the generic byte-writer protocol.
Use the explicit state or adler32-copy when the advanced checksum must be inspected.

</div>
</article>

<article class="doc-entry" id="v-adler32-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Text.Adler32/#t-Adler32State" title="Text.Adler32.Adler32State">Adler32State</a>)</code><span class="doc-sig-links"><a href="#v-adler32-write" title="Permalink">#</a><a href="/library/source/Effect.Adler32/#L16" title="View source at line 16">Source</a></span></div>
<div class="doc-body">

Hash a whole chunk, returning its accepted count and the advanced immutable state.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Adler32.cinch`](/library/source/Effect.Adler32/). Source SHA-256: `561a9c98abc71e5022289621323d3003d9419d7ac5e811310389bebf1c1d3566`.
