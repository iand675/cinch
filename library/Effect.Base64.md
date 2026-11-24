# Effect.Base64

Explicitly finalized Base64 writers and bounded decoding readers with sticky errors and isolated output chunks.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Base64</code></div><div class="doc-mod-bar-links"><a href="/library/source/Effect.Base64/">View source</a><a href="/library/search/?q=%2BEffect.Base64">Search module</a><a href="/reference-sources/Effect.Base64.cinch" download="Effect.Base64.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Effect.Stream](Effect.Stream.md), [Text.Base64.Encoding](Text.Base64.Encoding.md).

<details class="doc-closure"><summary>Ship-source closure (4 files)</summary>

Copy these modules beside your entry file before importing `Effect.Base64`: [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Base64](Effect.Base64.md), [Effect.Stream](Effect.Stream.md), [Text.Base64.Encoding](Text.Base64.Encoding.md).

```sh
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Effect/Base64.cinch ./Effect.Base64.cinch
cp seed/prelude/Effect/Stream.cinch ./Effect.Stream.cinch
cp seed/prelude/Text/Base64/Encoding.cinch ./Text.Base64.Encoding.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Incremental Base64 writers and decoding byte readers with explicit continuations.
Writers retain at most two copied input bytes and finish explicitly. Readers
delay failures until preceding complete output is drained. Zero reads do not
advance either source; errors remain sticky. No adapter closes a borrowed sink.

</details>

<details class="doc-synopsis" open><summary>Synopsis (6 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Base64Writer">Base64Writer</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base64Writer</strong> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-base64-finish">base64-finish</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-finish</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Base64/#t-Base64Writer" title="Effect.Base64.Base64Writer">Base64Writer</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a>, <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span>)</code></li><li><a class="syn-name" href="#v-base64-reader">base64-reader</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-reader</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-base64-write">base64-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Base64/#t-Base64Writer" title="Effect.Base64.Base64Writer">Base64Writer</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Effect.Base64/#t-Base64Writer" title="Effect.Base64.Base64Writer">Base64Writer</a> <span class="tok-variable">r</span>)</code></li><li><a class="syn-name" href="#v-base64-writer">base64-writer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-writer</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/library/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Base64/#t-Base64Writer" title="Effect.Base64.Base64Writer">Base64Writer</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-base64-writer-sink">base64-writer-sink</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-writer-sink</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Base64/#t-Base64Writer" title="Effect.Base64.Base64Writer">Base64Writer</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Base64Writer">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base64Writer</strong> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#t-Base64Writer" title="Permalink">#</a><a href="/library/source/Effect.Base64/#L18" title="View source at line 18">Source</a></span></div>
<div class="doc-body">

Inspectable encoding continuation retaining the advanced underlying sink on failure.

</div>
</article>

# Values

<article class="doc-entry" id="v-base64-finish">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-finish</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Base64/#t-Base64Writer" title="Effect.Base64.Base64Writer">Base64Writer</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a>, <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span>)</code><span class="doc-sig-links"><a href="#v-base64-finish" title="Permalink">#</a><a href="/library/source/Effect.Base64/#L46" title="View source at line 46">Source</a></span></div>
<div class="doc-body">

Emit the final short quantum and return the underlying sink on either outcome.
This does not flush or close the borrowed sink; later unencoded writes remain possible.

</div>
</article>

<article class="doc-entry" id="v-base64-reader">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-reader</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-base64-reader" title="Permalink">#</a><a href="/library/source/Effect.Base64/#L69" title="View source at line 69">Source</a></span></div>
<div class="doc-body">

Decode bounded source chunks through EOF, preserving complete output before a later failure.
CR/LF are ignored, strictness follows the selected encoding, and zero reads are inert.

</div>
</article>

<article class="doc-entry" id="v-base64-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Base64/#t-Base64Writer" title="Effect.Base64.Base64Writer">Base64Writer</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Effect.Base64/#t-Base64Writer" title="Effect.Base64.Base64Writer">Base64Writer</a> <span class="tok-variable">r</span>)</code><span class="doc-sig-links"><a href="#v-base64-write" title="Permalink">#</a><a href="/library/source/Effect.Base64/#L36" title="View source at line 36">Source</a></span></div>
<div class="doc-body">

Accept complete input, retrying short encoded writes and retaining at most two input bytes.
Failures preserve the advanced sink and never replay its accepted encoded prefix.

</div>
</article>

<article class="doc-entry" id="v-base64-writer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-writer</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/library/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Base64/#t-Base64Writer" title="Effect.Base64.Base64Writer">Base64Writer</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-base64-writer" title="Permalink">#</a><a href="/library/source/Effect.Base64/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">

Begin encoding to a sink. Finish before observing the complete encoded output.

</div>
</article>

<article class="doc-entry" id="v-base64-writer-sink">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-writer-sink</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Base64/#t-Base64Writer" title="Effect.Base64.Base64Writer">Base64Writer</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-base64-writer-sink" title="Permalink">#</a><a href="/library/source/Effect.Base64/#L29" title="View source at line 29">Source</a></span></div>
<div class="doc-body">

Observe the advanced sink, including after a failed write.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Base64.cinch`](/library/source/Effect.Base64/). Source SHA-256: `4cb4efcac5c5f5cc20a4db01503a21baf397adb8a0c1104a2ee19a0ebf773231`.
