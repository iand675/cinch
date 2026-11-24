# Effect.Base32

Explicitly finalized Base32 encoding writers and bounded decoding readers with precise errors, retained cursors and sticky failures.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Base32</code></div><div class="doc-mod-bar-links"><a href="/library/source/Effect.Base32/">View source</a><a href="/library/search/?q=%2BEffect.Base32">Search module</a><a href="/reference-sources/Effect.Base32.cinch" download="Effect.Base32.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Effect.Stream](Effect.Stream.md), [Text.Base32](Text.Base32.md).

<details class="doc-closure"><summary>Ship-source closure (4 files)</summary>

Copy these modules beside your entry file before importing `Effect.Base32`: [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Base32](Effect.Base32.md), [Effect.Stream](Effect.Stream.md), [Text.Base32](Text.Base32.md).

```sh
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Effect/Base32.cinch ./Effect.Base32.cinch
cp seed/prelude/Effect/Stream.cinch ./Effect.Stream.cinch
cp seed/prelude/Text/Base32.cinch ./Text.Base32.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Configurable Base32 encoding writers and strict decoding byte readers.
Writers retain at most four input bytes until explicitly finished. Readers
consume bounded chunks, preserve decoded output before a later error, and
count malformed input in original bytes including CR/LF. Failures are sticky.
Neither adapter closes or flushes a borrowed source or sink.

</details>

<details class="doc-synopsis" open><summary>Synopsis (6 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Base32Writer">Base32Writer</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base32Writer</strong> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-base32-finish">base32-finish</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-finish</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Base32/#t-Base32Writer" title="Effect.Base32.Base32Writer">Base32Writer</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a>, <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span>)</code></li><li><a class="syn-name" href="#v-base32-reader">base32-reader</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-reader</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-base32-write">base32-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Base32/#t-Base32Writer" title="Effect.Base32.Base32Writer">Base32Writer</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Effect.Base32/#t-Base32Writer" title="Effect.Base32.Base32Writer">Base32Writer</a> <span class="tok-variable">r</span>)</code></li><li><a class="syn-name" href="#v-base32-writer">base32-writer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-writer</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/library/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Base32/#t-Base32Writer" title="Effect.Base32.Base32Writer">Base32Writer</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-base32-writer-sink">base32-writer-sink</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-writer-sink</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Base32/#t-Base32Writer" title="Effect.Base32.Base32Writer">Base32Writer</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Base32Writer">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base32Writer</strong> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#t-Base32Writer" title="Permalink">#</a><a href="/library/source/Effect.Base32/#L19" title="View source at line 19">Source</a></span></div>
<div class="doc-body">

An explicitly advanced encoding continuation with its owned tail and underlying sink.

</div>
</article>

# Values

<article class="doc-entry" id="v-base32-finish">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-finish</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Base32/#t-Base32Writer" title="Effect.Base32.Base32Writer">Base32Writer</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a>, <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span>)</code><span class="doc-sig-links"><a href="#v-base32-finish" title="Permalink">#</a><a href="/library/source/Effect.Base32/#L47" title="View source at line 47">Source</a></span></div>
<div class="doc-body">

Emit the final short quantum and configured padding, returning the advanced sink on either outcome.
This finalizes the adapter's encoding only; the borrowed sink is not closed or flushed.

</div>
</article>

<article class="doc-entry" id="v-base32-reader">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-reader</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-base32-reader" title="Permalink">#</a><a href="/library/source/Effect.Base32/#L71" title="View source at line 71">Source</a></span></div>
<div class="doc-body">

Begin strict configurable decoding. Zero reads preserve both source and decoder state.

</div>
</article>

<article class="doc-entry" id="v-base32-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Base32/#t-Base32Writer" title="Effect.Base32.Base32Writer">Base32Writer</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Effect.Base32/#t-Base32Writer" title="Effect.Base32.Base32Writer">Base32Writer</a> <span class="tok-variable">r</span>)</code><span class="doc-sig-links"><a href="#v-base32-write" title="Permalink">#</a><a href="/library/source/Effect.Base32/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

Accept a chunk, retrying short output writes and copying the unfinished input fringe.
Success reports the complete input byte count. Failure returns a sticky advanced handle.

</div>
</article>

<article class="doc-entry" id="v-base32-writer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-writer</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/library/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Base32/#t-Base32Writer" title="Effect.Base32.Base32Writer">Base32Writer</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-base32-writer" title="Permalink">#</a><a href="/library/source/Effect.Base32/#L26" title="View source at line 26">Source</a></span></div>
<div class="doc-body">

Begin a configurable encoding. Finish explicitly before observing complete output.

</div>
</article>

<article class="doc-entry" id="v-base32-writer-sink">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-writer-sink</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Base32/#t-Base32Writer" title="Effect.Base32.Base32Writer">Base32Writer</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-base32-writer-sink" title="Permalink">#</a><a href="/library/source/Effect.Base32/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">

Inspect the advanced output handle, including after an unsuccessful encoded write.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Base32.cinch`](/library/source/Effect.Base32/). Source SHA-256: `38f8d4b0a0ac51fed5ed5251d0b16093e8983986ea36330edede81bb69705b90`.
