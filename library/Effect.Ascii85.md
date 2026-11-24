# Effect.Ascii85

ASCII85 encoding writers and decoding byte readers with explicit finalization, bounded read-ahead and sticky stream errors.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Ascii85</code></div><div class="doc-mod-bar-links"><a href="/library/source/Effect.Ascii85/">View source</a><a href="/library/search/?q=%2BEffect.Ascii85">Search module</a><a href="/reference-sources/Effect.Ascii85.cinch" download="Effect.Ascii85.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Stream](Effect.Stream.md), [Text.Ascii85](Text.Ascii85.md).

<details class="doc-closure"><summary>Ship-source closure (4 files)</summary>

Copy these modules beside your entry file before importing `Effect.Ascii85`: [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Ascii85](Effect.Ascii85.md), [Effect.Stream](Effect.Stream.md), [Text.Ascii85](Text.Ascii85.md).

```sh
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Effect/Ascii85.cinch ./Effect.Ascii85.cinch
cp seed/prelude/Effect/Stream.cinch ./Effect.Stream.cinch
cp seed/prelude/Text/Ascii85.cinch ./Text.Ascii85.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Incremental ASCII85 writers and decoding byte readers.
Writers buffer at most three input bytes between writes; finish explicitly
flushes that tail and returns the advanced underlying sink. Neither adapter
closes descriptors. Decoder failures report the encoding name, original byte
offset and diagnostic through StreamEncodingError. Read/write failures are
sticky and never replay already consumed input or accepted output.

</details>

<details class="doc-synopsis" open><summary>Synopsis (6 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Ascii85Writer">Ascii85Writer</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ascii85Writer</strong> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-ascii85-finish">ascii85-finish</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-finish</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Ascii85/#t-Ascii85Writer" title="Effect.Ascii85.Ascii85Writer">Ascii85Writer</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a>, <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span>)</code></li><li><a class="syn-name" href="#v-ascii85-reader">ascii85-reader</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-reader</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-ascii85-write">ascii85-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Ascii85/#t-Ascii85Writer" title="Effect.Ascii85.Ascii85Writer">Ascii85Writer</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Effect.Ascii85/#t-Ascii85Writer" title="Effect.Ascii85.Ascii85Writer">Ascii85Writer</a> <span class="tok-variable">r</span>)</code></li><li><a class="syn-name" href="#v-ascii85-writer">ascii85-writer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-writer</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Ascii85/#t-Ascii85Writer" title="Effect.Ascii85.Ascii85Writer">Ascii85Writer</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-ascii85-writer-sink">ascii85-writer-sink</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-writer-sink</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Ascii85/#t-Ascii85Writer" title="Effect.Ascii85.Ascii85Writer">Ascii85Writer</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Ascii85Writer">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ascii85Writer</strong> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#t-Ascii85Writer" title="Permalink">#</a><a href="/library/source/Effect.Ascii85/#L20" title="View source at line 20">Source</a></span></div>
<div class="doc-body">

An explicitly advanced encoding continuation and its underlying sink.

</div>
</article>

# Values

<article class="doc-entry" id="v-ascii85-finish">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-finish</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Ascii85/#t-Ascii85Writer" title="Effect.Ascii85.Ascii85Writer">Ascii85Writer</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a>, <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span>)</code><span class="doc-sig-links"><a href="#v-ascii85-finish" title="Permalink">#</a><a href="/library/source/Effect.Ascii85/#L48" title="View source at line 48">Source</a></span></div>
<div class="doc-body">

Flush a final short quantum and return the underlying sink on either outcome.
The returned sink can be inspected, flushed, closed or used for later unencoded data.

</div>
</article>

<article class="doc-entry" id="v-ascii85-reader">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-reader</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-ascii85-reader" title="Permalink">#</a><a href="/library/source/Effect.Ascii85/#L73" title="View source at line 73">Source</a></span></div>
<div class="doc-body">

Wrap a reader as a decoding source. Zero reads do not advance either source.
The adapter reads bounded 4096-byte input chunks and retains decoded surplus.

</div>
</article>

<article class="doc-entry" id="v-ascii85-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Ascii85/#t-Ascii85Writer" title="Effect.Ascii85.Ascii85Writer">Ascii85Writer</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Effect.Ascii85/#t-Ascii85Writer" title="Effect.Ascii85.Ascii85Writer">Ascii85Writer</a> <span class="tok-variable">r</span>)</code><span class="doc-sig-links"><a href="#v-ascii85-write" title="Permalink">#</a><a href="/library/source/Effect.Ascii85/#L38" title="View source at line 38">Source</a></span></div>
<div class="doc-body">

Accept all input, writing complete quanta and retaining at most three bytes.
Short underlying writes are retried; any failure is retained in the next handle.

</div>
</article>

<article class="doc-entry" id="v-ascii85-writer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-writer</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Ascii85/#t-Ascii85Writer" title="Effect.Ascii85.Ascii85Writer">Ascii85Writer</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-ascii85-writer" title="Permalink">#</a><a href="/library/source/Effect.Ascii85/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">

Begin encoding to a sink. Finish before inspecting the complete encoded output.

</div>
</article>

<article class="doc-entry" id="v-ascii85-writer-sink">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-writer-sink</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Ascii85/#t-Ascii85Writer" title="Effect.Ascii85.Ascii85Writer">Ascii85Writer</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-ascii85-writer-sink" title="Permalink">#</a><a href="/library/source/Effect.Ascii85/#L31" title="View source at line 31">Source</a></span></div>
<div class="doc-body">

Observe the advanced underlying sink, including after a failed write.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Ascii85.cinch`](/library/source/Effect.Ascii85/). Source SHA-256: `ce62b87682e7b8b63754a346c01a3db666e0a062693fa958251dac6c89febb27`.
