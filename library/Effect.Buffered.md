# Effect.Buffered

Buffered input and output components with explicit independent advanced handles.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Buffered</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Buffered/source/">View source</a><a href="/packages/search/?q=%2BEffect.Buffered">Search module</a><a href="/reference-sources/Effect.Buffered.cinch" download="Effect.Buffered.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Effect.Buffered.Reader](Effect.Buffered.Reader.md), [Effect.Buffered.Writer](Effect.Buffered.Writer.md).

<details class="doc-closure"><summary>Ship-source closure (10 files)</summary>

Copy these modules beside your entry file before importing `Effect.Buffered`: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Buffered](Effect.Buffered.md), [Effect.Buffered.Core](Effect.Buffered.Core.md), [Effect.Buffered.Internal.Cell](Effect.Buffered.Internal.Cell.md), [Effect.Buffered.Internal.Scan](Effect.Buffered.Internal.Scan.md), [Effect.Buffered.Reader](Effect.Buffered.Reader.md), [Effect.Buffered.Writer](Effect.Buffered.Writer.md), [Effect.Stream](Effect.Stream.md).

```sh
cp seed/prelude/Data/Bytes/Build.cinch ./Data.Bytes.Build.cinch
cp seed/prelude/Data/Bytes/Utf8.cinch ./Data.Bytes.Utf8.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Effect/Buffered.cinch ./Effect.Buffered.cinch
cp seed/prelude/Effect/Buffered/Core.cinch ./Effect.Buffered.Core.cinch
cp seed/prelude/Effect/Buffered/Internal/Cell.cinch ./Effect.Buffered.Internal.Cell.cinch
cp seed/prelude/Effect/Buffered/Internal/Scan.cinch ./Effect.Buffered.Internal.Scan.cinch
cp seed/prelude/Effect/Buffered/Reader.cinch ./Effect.Buffered.Reader.cinch
cp seed/prelude/Effect/Buffered/Writer.cinch ./Effect.Buffered.Writer.cinch
cp seed/prelude/Effect/Stream.cinch ./Effect.Stream.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Buffered input and output components with explicit independent advanced handles.
Reading and writing preserve each other's state. Apply a component operation,
then install its returned component to continue through the combined handle.

</details>

<details class="doc-synopsis" open><summary>Synopsis (6 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-BufferedReadWriter">BufferedReadWriter</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BufferedReadWriter</strong> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-buffered-read-writer">buffered-read-writer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">buffered-read-writer</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Reader/#t-BufferedReader" title="Effect.Buffered.Reader.BufferedReader">BufferedReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Writer/#t-BufferedWriter" title="Effect.Buffered.Writer.BufferedWriter">BufferedWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered/#t-BufferedReadWriter" title="Effect.Buffered.BufferedReadWriter">BufferedReadWriter</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-buffered-read-writer-reader">buffered-read-writer-reader</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">buffered-read-writer-reader</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered/#t-BufferedReadWriter" title="Effect.Buffered.BufferedReadWriter">BufferedReadWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Reader/#t-BufferedReader" title="Effect.Buffered.Reader.BufferedReader">BufferedReader</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-buffered-read-writer-with-reader">buffered-read-writer-with-reader</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">buffered-read-writer-with-reader</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered/#t-BufferedReadWriter" title="Effect.Buffered.BufferedReadWriter">BufferedReadWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Reader/#t-BufferedReader" title="Effect.Buffered.Reader.BufferedReader">BufferedReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered/#t-BufferedReadWriter" title="Effect.Buffered.BufferedReadWriter">BufferedReadWriter</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-buffered-read-writer-with-writer">buffered-read-writer-with-writer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">buffered-read-writer-with-writer</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered/#t-BufferedReadWriter" title="Effect.Buffered.BufferedReadWriter">BufferedReadWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Writer/#t-BufferedWriter" title="Effect.Buffered.Writer.BufferedWriter">BufferedWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered/#t-BufferedReadWriter" title="Effect.Buffered.BufferedReadWriter">BufferedReadWriter</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-buffered-read-writer-writer">buffered-read-writer-writer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">buffered-read-writer-writer</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered/#t-BufferedReadWriter" title="Effect.Buffered.BufferedReadWriter">BufferedReadWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Writer/#t-BufferedWriter" title="Effect.Buffered.Writer.BufferedWriter">BufferedWriter</a> <span class="tok-variable">r</span></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-BufferedReadWriter">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BufferedReadWriter</strong> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#t-BufferedReadWriter" title="Permalink">#</a><a href="/packages/stdlib/Effect.Buffered/source/#L13" title="View source at line 13">Source</a></span></div>
<div class="doc-body">

A paired reader and writer over the same I/O region, with no implicit flushing or read-ahead coupling.

</div>
</article>

# Values

<article class="doc-entry" id="v-buffered-read-writer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">buffered-read-writer</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Reader/#t-BufferedReader" title="Effect.Buffered.Reader.BufferedReader">BufferedReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Writer/#t-BufferedWriter" title="Effect.Buffered.Writer.BufferedWriter">BufferedWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered/#t-BufferedReadWriter" title="Effect.Buffered.BufferedReadWriter">BufferedReadWriter</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-buffered-read-writer" title="Permalink">#</a><a href="/packages/stdlib/Effect.Buffered/source/#L18" title="View source at line 18">Source</a></span></div>
<div class="doc-body">

Pair existing buffered components without reading, writing or flushing either one.

</div>
</article>

<article class="doc-entry" id="v-buffered-read-writer-reader">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">buffered-read-writer-reader</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered/#t-BufferedReadWriter" title="Effect.Buffered.BufferedReadWriter">BufferedReadWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Reader/#t-BufferedReader" title="Effect.Buffered.Reader.BufferedReader">BufferedReader</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-buffered-read-writer-reader" title="Permalink">#</a><a href="/packages/stdlib/Effect.Buffered/source/#L22" title="View source at line 22">Source</a></span></div>
<div class="doc-body">

Observe the current reader component; continue with the advanced reader returned by its next operation.

</div>
</article>

<article class="doc-entry" id="v-buffered-read-writer-with-reader">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">buffered-read-writer-with-reader</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered/#t-BufferedReadWriter" title="Effect.Buffered.BufferedReadWriter">BufferedReadWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Reader/#t-BufferedReader" title="Effect.Buffered.Reader.BufferedReader">BufferedReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered/#t-BufferedReadWriter" title="Effect.Buffered.BufferedReadWriter">BufferedReadWriter</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-buffered-read-writer-with-reader" title="Permalink">#</a><a href="/packages/stdlib/Effect.Buffered/source/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">

Install an advanced reader while preserving the independently advancing writer.

</div>
</article>

<article class="doc-entry" id="v-buffered-read-writer-with-writer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">buffered-read-writer-with-writer</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered/#t-BufferedReadWriter" title="Effect.Buffered.BufferedReadWriter">BufferedReadWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Writer/#t-BufferedWriter" title="Effect.Buffered.Writer.BufferedWriter">BufferedWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered/#t-BufferedReadWriter" title="Effect.Buffered.BufferedReadWriter">BufferedReadWriter</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-buffered-read-writer-with-writer" title="Permalink">#</a><a href="/packages/stdlib/Effect.Buffered/source/#L34" title="View source at line 34">Source</a></span></div>
<div class="doc-body">

Install an advanced writer while preserving unread input, source errors and reader history.

</div>
</article>

<article class="doc-entry" id="v-buffered-read-writer-writer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">buffered-read-writer-writer</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered/#t-BufferedReadWriter" title="Effect.Buffered.BufferedReadWriter">BufferedReadWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Writer/#t-BufferedWriter" title="Effect.Buffered.Writer.BufferedWriter">BufferedWriter</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-buffered-read-writer-writer" title="Permalink">#</a><a href="/packages/stdlib/Effect.Buffered/source/#L26" title="View source at line 26">Source</a></span></div>
<div class="doc-body">

Observe the current writer component without performing I/O or implicitly flushing buffered bytes.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Buffered.cinch`](/packages/stdlib/Effect.Buffered/source/). Source SHA-256: `a9669f4c6996da84707ead19c237498e0b043fcac8dc1e4cc945ccaecff22054`.
