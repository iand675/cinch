# Effect.Stream.Section

Positional byte sources, immutable bounded sections, signed seeks and sequential reader adapters.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Stream.Section</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Stream.Section/source/">View source</a><a href="/packages/search/?q=%2BEffect.Stream.Section">Search module</a><a href="/reference-sources/Effect.Stream.Section.cinch" download="Effect.Stream.Section.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Int](Data.Int.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Effect.Stream](Effect.Stream.md).

<details class="doc-closure"><summary>Ship-source closure (2 files)</summary>

Copy these modules beside your entry file before importing `Effect.Stream.Section`: [Effect.Stream](Effect.Stream.md), [Effect.Stream.Section](Effect.Stream.Section.md).

```sh
cp seed/prelude/Effect/Stream.cinch ./Effect.Stream.cinch
cp seed/prelude/Effect/Stream/Section.cinch ./Effect.Stream.Section.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Positional byte sources and bounded, independently seekable reader views.
All offsets and sizes are natural numbers. Short reads return the available
prefix, and a positive read returning empty bytes means EOF.

</details>

<details class="doc-synopsis" open><summary>Synopsis (14 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ByteReaderAt">ByteReaderAt</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ByteReaderAt</strong> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#t-ByteSection">ByteSection</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ByteSection</strong> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#t-SeekOrigin">SeekOrigin</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SeekOrigin</strong></code></li><li><a class="syn-name" href="#v-reader-at">reader-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">reader-at</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteReaderAt" title="Effect.Stream.Section.ByteReaderAt">ByteReaderAt</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-reader-at-bytes">reader-at-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">reader-at-bytes</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteReaderAt" title="Effect.Stream.Section.ByteReaderAt">ByteReaderAt</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-reader-section">reader-section</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">reader-section</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteReaderAt" title="Effect.Stream.Section.ByteReaderAt">ByteReaderAt</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-section-outer">section-outer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">section-outer</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteReaderAt" title="Effect.Stream.Section.ByteReaderAt">ByteReaderAt</a> <span class="tok-variable">r</span>, (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>))</code></li><li><a class="syn-name" href="#v-section-position">section-position</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">section-position</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-section-read">section-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">section-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span>)</code></li><li><a class="syn-name" href="#v-section-read-at">section-read-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">section-read-at</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-section-reader">section-reader</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">section-reader</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-section-seek">section-seek</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">section-seek</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-SeekOrigin" title="Effect.Stream.Section.SeekOrigin">SeekOrigin</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span>)</code></li><li><a class="syn-name" href="#v-section-size">section-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">section-size</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-stream-read-at">stream-read-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">stream-read-at</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteReaderAt" title="Effect.Stream.Section.ByteReaderAt">ByteReaderAt</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ByteReaderAt">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ByteReaderAt</strong> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#t-ByteReaderAt" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stream.Section/source/#L18" title="View source at line 18">Source</a></span></div>
<div class="doc-body">

A source addressed by absolute offset. Each call is independent of cursor position.

</div>
</article>

<article class="doc-entry" id="t-ByteSection">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ByteSection</strong> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#t-ByteSection" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stream.Section/source/#L50" title="View source at line 50">Source</a></span></div>
<div class="doc-body">

A source, absolute start, declared size, and cursor relative to the start.

</div>
</article>

<article class="doc-entry" id="t-SeekOrigin">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SeekOrigin</strong></code><span class="doc-sig-links"><a href="#t-SeekOrigin" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stream.Section/source/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

Reference point for a signed seek offset.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SeekStart"><code><strong class="tok-constructor">SeekStart</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-SeekOrigin" title="Effect.Stream.Section.SeekOrigin">SeekOrigin</a></code></li><li id="k-SeekCurrent"><code><strong class="tok-constructor">SeekCurrent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-SeekOrigin" title="Effect.Stream.Section.SeekOrigin">SeekOrigin</a></code></li><li id="k-SeekEnd"><code><strong class="tok-constructor">SeekEnd</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-SeekOrigin" title="Effect.Stream.Section.SeekOrigin">SeekOrigin</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-reader-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">reader-at</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteReaderAt" title="Effect.Stream.Section.ByteReaderAt">ByteReaderAt</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-reader-at" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stream.Section/source/#L23" title="View source at line 23">Source</a></span></div>
<div class="doc-body">

Wrap a positional read callback. It receives offset and maximum byte count.

</div>
</article>

<article class="doc-entry" id="v-reader-at-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">reader-at-bytes</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteReaderAt" title="Effect.Stream.Section.ByteReaderAt">ByteReaderAt</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-reader-at-bytes" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stream.Section/source/#L35" title="View source at line 35">Source</a></span></div>
<div class="doc-body">

Snapshot the caller's bytes. Each successful read returns independently owned bytes.

</div>
</article>

<article class="doc-entry" id="v-reader-section">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">reader-section</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteReaderAt" title="Effect.Stream.Section.ByteReaderAt">ByteReaderAt</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-reader-section" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stream.Section/source/#L54" title="View source at line 54">Source</a></span></div>
<div class="doc-body">

Create a view at position zero. Its declared size can extend beyond the source.

</div>
</article>

<article class="doc-entry" id="v-section-outer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">section-outer</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteReaderAt" title="Effect.Stream.Section.ByteReaderAt">ByteReaderAt</a> <span class="tok-variable">r</span>, (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>))</code><span class="doc-sig-links"><a href="#v-section-outer" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stream.Section/source/#L58" title="View source at line 58">Source</a></span></div>
<div class="doc-body">

Recover the original source, absolute start and declared size.

</div>
</article>

<article class="doc-entry" id="v-section-position">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">section-position</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-section-position" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stream.Section/source/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">

The sequential cursor relative to the beginning of the view.

</div>
</article>

<article class="doc-entry" id="v-section-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">section-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span>)</code><span class="doc-sig-links"><a href="#v-section-read" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stream.Section/source/#L92" title="View source at line 92">Source</a></span></div>
<div class="doc-body">

Advance only by the number of bytes returned. Errors preserve the cursor.

</div>
</article>

<article class="doc-entry" id="v-section-read-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">section-read-at</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-section-read-at" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stream.Section/source/#L84" title="View source at line 84">Source</a></span></div>
<div class="doc-body">

Read at a section-relative offset without changing its cursor or crossing its end.

</div>
</article>

<article class="doc-entry" id="v-section-reader">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">section-reader</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-section-reader" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stream.Section/source/#L99" title="View source at line 99">Source</a></span></div>
<div class="doc-body">

Consume a section through the generic sequential reader protocol.

</div>
</article>

<article class="doc-entry" id="v-section-seek">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">section-seek</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-SeekOrigin" title="Effect.Stream.Section.SeekOrigin">SeekOrigin</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span>)</code><span class="doc-sig-links"><a href="#v-section-seek" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stream.Section/source/#L76" title="View source at line 76">Source</a></span></div>
<div class="doc-body">

Return a repositioned view, or None for a negative result. Seeking beyond EOF is valid.

</div>
</article>

<article class="doc-entry" id="v-section-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">section-size</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteSection" title="Effect.Stream.Section.ByteSection">ByteSection</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-section-size" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stream.Section/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

The declared number of bytes in the view.

</div>
</article>

<article class="doc-entry" id="v-stream-read-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">stream-read-at</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream.Section/#t-ByteReaderAt" title="Effect.Stream.Section.ByteReaderAt">ByteReaderAt</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-stream-read-at" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stream.Section/source/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">

Read a bounded prefix. Zero-length reads skip the callback.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Stream/Section.cinch`](/packages/stdlib/Effect.Stream.Section/source/). Source SHA-256: `ce1b65e973fd2e8d0cbc26d1b9876fa1cd38113fa3843c56cfc229b20954059e`.
