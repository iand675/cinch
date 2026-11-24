# Effect.Md5

Inspectable MD5 states, generic digest sinks and bounded reader-to-hash copying for legacy formats.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Md5</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Md5/source/">View source</a><a href="/packages/search/?q=%2BEffect.Md5">Search module</a><a href="/reference-sources/Effect.Md5.cinch" download="Effect.Md5.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Effect.Stream](Effect.Stream.md), [Text.Md5](Text.Md5.md).

<details class="doc-closure"><summary>Ship-source closure (5 files)</summary>

Copy these modules beside your entry file before importing `Effect.Md5`: [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Md5](Effect.Md5.md), [Effect.Stream](Effect.Stream.md), [Text.Binary](Text.Binary.md), [Text.Md5](Text.Md5.md).

```sh
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Effect/Md5.cinch ./Effect.Md5.cinch
cp seed/prelude/Effect/Stream.cinch ./Effect.Stream.cinch
cp seed/prelude/Text/Binary.cinch ./Text.Binary.cinch
cp seed/prelude/Text/Md5.cinch ./Text.Md5.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Inspectable MD5 hashing sinks and bounded reader-to-digest copying.
The immutable digest state is the sink handle. Writes accept every byte and
never retain input buffers. Copying returns the advanced reader and the last
valid digest on failure, and never closes a borrowed source.

</details>

<details class="doc-synopsis" open><summary>Synopsis (3 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-md5-copy">md5-copy</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">md5-copy</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a>))</code></li><li><a class="syn-name" href="#v-md5-hashing-writer">md5-hashing-writer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">md5-hashing-writer</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-md5-write">md5-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">md5-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a>)</code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-md5-copy">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">md5-copy</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a>))</code><span class="doc-sig-links"><a href="#v-md5-copy" title="Permalink">#</a><a href="/packages/stdlib/Effect.Md5/source/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">

Hash successful reader chunks through EOF, preserving both advanced handles on errors.
A zero chunk size is rejected before reading. Invalid source counts do not enter the digest.

</div>
</article>

<article class="doc-entry" id="v-md5-hashing-writer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">md5-hashing-writer</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-md5-hashing-writer" title="Permalink">#</a><a href="/packages/stdlib/Effect.Md5/source/#L21" title="View source at line 21">Source</a></span></div>
<div class="doc-body">

Adapt a digest state to the generic byte-writer protocol.
Use the explicit state or md5-copy when the advanced digest must be inspected.

</div>
</article>

<article class="doc-entry" id="v-md5-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">md5-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Md5/#t-Md5State" title="Text.Md5.Md5State">Md5State</a>)</code><span class="doc-sig-links"><a href="#v-md5-write" title="Permalink">#</a><a href="/packages/stdlib/Effect.Md5/source/#L16" title="View source at line 16">Source</a></span></div>
<div class="doc-body">

Hash a whole chunk, returning its accepted count and the advanced immutable state.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Md5.cinch`](/packages/stdlib/Effect.Md5/source/). Source SHA-256: `fc92cfe09b3a1ff53d9b2ed9fc70bf130d965602dbacac76c93ec5c4ad21e1c3`.
