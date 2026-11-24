# Effect.Crc32

CRC-32 stream writers and bounded copying into an inspectable checksum state.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Crc32</code></div><div class="doc-mod-bar-links"><a href="/library/source/Effect.Crc32/">View source</a><a href="/library/search/?q=%2BEffect.Crc32">Search module</a><a href="/reference-sources/Effect.Crc32.cinch" download="Effect.Crc32.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Effect.Stream](Effect.Stream.md), [Text.Crc32](Text.Crc32.md).

<details class="doc-closure"><summary>Ship-source closure (5 files)</summary>

Copy these modules beside your entry file before importing `Effect.Crc32`: [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Crc32](Effect.Crc32.md), [Effect.Stream](Effect.Stream.md), [Text.Binary](Text.Binary.md), [Text.Crc32](Text.Crc32.md).

```sh
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Effect/Crc32.cinch ./Effect.Crc32.cinch
cp seed/prelude/Effect/Stream.cinch ./Effect.Stream.cinch
cp seed/prelude/Text/Binary.cinch ./Text.Binary.cinch
cp seed/prelude/Text/Crc32.cinch ./Text.Crc32.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Inspectable CRC-32 hashing sinks and bounded reader-to-checksum copying.
The immutable table and checksum state is the sink handle. Writes accept every byte and
never retain input buffers. Copying returns the advanced reader and the last
valid checksum on failure, and never closes a borrowed source.

</details>

<details class="doc-synopsis" open><summary>Synopsis (3 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-crc32-copy">crc32-copy</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-copy</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, (<a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span>, <a class="doc-type-link tok-constructor" href="/library/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a>))</code></li><li><a class="syn-name" href="#v-crc32-hashing-writer">crc32-hashing-writer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-hashing-writer</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-crc32-write">crc32-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a>)</code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-crc32-copy">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-copy</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, (<a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span>, <a class="doc-type-link tok-constructor" href="/library/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a>))</code><span class="doc-sig-links"><a href="#v-crc32-copy" title="Permalink">#</a><a href="/library/source/Effect.Crc32/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">

Hash successful reader chunks through EOF, preserving both advanced handles on errors.
A zero chunk size is rejected before reading. Invalid source counts do not enter the checksum.

</div>
</article>

<article class="doc-entry" id="v-crc32-hashing-writer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-hashing-writer</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-crc32-hashing-writer" title="Permalink">#</a><a href="/library/source/Effect.Crc32/#L21" title="View source at line 21">Source</a></span></div>
<div class="doc-body">

Adapt a checksum state to the generic byte-writer protocol.
Use the explicit state or crc32-copy when the advanced checksum must be inspected.

</div>
</article>

<article class="doc-entry" id="v-crc32-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Text.Crc32/#t-Crc32State" title="Text.Crc32.Crc32State">Crc32State</a>)</code><span class="doc-sig-links"><a href="#v-crc32-write" title="Permalink">#</a><a href="/library/source/Effect.Crc32/#L16" title="View source at line 16">Source</a></span></div>
<div class="doc-body">

Hash a whole chunk, returning its accepted count and the advanced immutable state.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Crc32.cinch`](/library/source/Effect.Crc32/). Source SHA-256: `5e6605c274df05c9cab7b75073b307563fc1cfc157152c615a712b53887e296d`.
