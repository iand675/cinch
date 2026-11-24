# Effect.Scanner

Advanced byte-stream tokenization with bounded buffers, fresh tokens and explicit checked protocol failures.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Scanner</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Scanner/source/">View source</a><a href="/packages/search/?q=%2BEffect.Scanner">Search module</a><a href="/reference-sources/Effect.Scanner.cinch" download="Effect.Scanner.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Int](Data.Int.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Buffered.Core](Effect.Buffered.Core.md), [Effect.Io](Effect.Io.md), [Effect.Scanner.Split](Effect.Scanner.Split.md), [Effect.Stream](Effect.Stream.md).

<details class="doc-closure"><summary>Ship-source closure (10 files)</summary>

Copy these modules beside your entry file before importing `Effect.Scanner`: [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Effect.Buffered.Core](Effect.Buffered.Core.md), [Effect.Buffered.Internal.Cell](Effect.Buffered.Internal.Cell.md), [Effect.Buffered.Internal.Scan](Effect.Buffered.Internal.Scan.md), [Effect.Scanner](Effect.Scanner.md), [Effect.Scanner.Split](Effect.Scanner.Split.md), [Effect.Stream](Effect.Stream.md).

```sh
cp seed/prelude/Data/Bytes/Utf8.cinch ./Data.Bytes.Utf8.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Data/Unicode/Kernel.cinch ./Data.Unicode.Kernel.cinch
cp seed/prelude/Data/Unicode/Tables.cinch ./Data.Unicode.Tables.cinch
cp seed/prelude/Effect/Buffered/Core.cinch ./Effect.Buffered.Core.cinch
cp seed/prelude/Effect/Buffered/Internal/Cell.cinch ./Effect.Buffered.Internal.Cell.cinch
cp seed/prelude/Effect/Buffered/Internal/Scan.cinch ./Effect.Buffered.Internal.Scan.cinch
cp seed/prelude/Effect/Scanner.cinch ./Effect.Scanner.cinch
cp seed/prelude/Effect/Scanner/Split.cinch ./Effect.Scanner.Split.cinch
cp seed/prelude/Effect/Stream.cinch ./Effect.Stream.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Advanced byte-stream tokenization with bounded buffers, fresh tokens and explicit checked protocol failures.
Configuration is immutable and becomes fixed after the first scan. Custom
split callbacks receive snapshots; token getters never expose private backing.

</details>

<details class="doc-synopsis" open><summary>Synopsis (12 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Scanner">Scanner</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Scanner</strong> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-scanner">scanner</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scanner</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Core/#t-BufferSource" title="Effect.Buffered.Core.BufferSource">BufferSource</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-scanner-buffer">scanner-buffer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scanner-buffer</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span>)</code></li><li><a class="syn-name" href="#v-scanner-buffer-size">scanner-buffer-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scanner-buffer-size</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span>)</code></li><li><a class="syn-name" href="#v-scanner-bytes">scanner-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scanner-bytes</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-scanner-error">scanner-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scanner-error</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a></code></li><li><a class="syn-name" href="#v-scanner-from-stream">scanner-from-stream</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scanner-from-stream</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-scanner-max-token-size">scanner-max-token-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scanner-max-token-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-scanner-next">scanner-next</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scanner-next</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span>)</code></li><li><a class="syn-name" href="#v-scanner-split">scanner-split</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scanner-split</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner.Split/#t-ScanSplit" title="Effect.Scanner.Split.ScanSplit">ScanSplit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span>)</code></li><li><a class="syn-name" href="#v-scanner-text">scanner-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scanner-text</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-scanner-token">scanner-token</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scanner-token</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Scanner">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Scanner</strong> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#t-Scanner" title="Permalink">#</a><a href="/packages/stdlib/Effect.Scanner/source/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">

An advanced tokenizer retains its authoritative source, split capability and last token independently.

</div>
</article>

# Values

<article class="doc-entry" id="v-scanner">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scanner</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Buffered.Core/#t-BufferSource" title="Effect.Buffered.Core.BufferSource">BufferSource</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-scanner" title="Permalink">#</a><a href="/packages/stdlib/Effect.Scanner/source/#L36" title="View source at line 36">Source</a></span></div>
<div class="doc-body">

Construct an unstarted line scanner with lazy initial buffer allocation and the default token maximum.

</div>
</article>

<article class="doc-entry" id="v-scanner-buffer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scanner-buffer</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span>)</code><span class="doc-sig-links"><a href="#v-scanner-buffer" title="Permalink">#</a><a href="/packages/stdlib/Effect.Scanner/source/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

Configure an initial private buffer length and a signed growth maximum before scanning begins.
The supplied byte contents are ignored and never retained. Existing initial capacity may exceed the growth maximum.

</div>
</article>

<article class="doc-entry" id="v-scanner-buffer-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scanner-buffer-size</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span>)</code><span class="doc-sig-links"><a href="#v-scanner-buffer-size" title="Permalink">#</a><a href="/packages/stdlib/Effect.Scanner/source/#L49" title="View source at line 49">Source</a></span></div>
<div class="doc-body">

Select explicit initial buffer capacity without allocating or exposing caller-owned backing storage.

</div>
</article>

<article class="doc-entry" id="v-scanner-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scanner-bytes</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-scanner-bytes" title="Permalink">#</a><a href="/packages/stdlib/Effect.Scanner/source/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

Observe fresh last-token bytes; an absent token is represented by fresh empty bytes.

</div>
</article>

<article class="doc-entry" id="v-scanner-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scanner-error</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a></code><span class="doc-sig-links"><a href="#v-scanner-error" title="Permalink">#</a><a href="/packages/stdlib/Effect.Scanner/source/#L83" title="View source at line 83">Source</a></span></div>
<div class="doc-body">

Observe the first retained non-end error; ordinary source end-of-input is successful termination.

</div>
</article>

<article class="doc-entry" id="v-scanner-from-stream">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scanner-from-stream</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-scanner-from-stream" title="Permalink">#</a><a href="/packages/stdlib/Effect.Scanner/source/#L40" title="View source at line 40">Source</a></span></div>
<div class="doc-body">

Adapt an existing stream reader under its established positive-empty-success end-of-input contract.

</div>
</article>

<article class="doc-entry" id="v-scanner-max-token-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scanner-max-token-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-scanner-max-token-size" title="Permalink">#</a><a href="/packages/stdlib/Effect.Scanner/source/#L32" title="View source at line 32">Source</a></span></div>
<div class="doc-body">

The default maximum allocated token buffer is 65,536 bytes, including any required delimiter.

</div>
</article>

<article class="doc-entry" id="v-scanner-next">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scanner-next</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span>)</code><span class="doc-sig-links"><a href="#v-scanner-next" title="Permalink">#</a><a href="/packages/stdlib/Effect.Scanner/source/#L218" title="View source at line 218">Source</a></span></div>
<div class="doc-body">

Advance to the next token, returning an explicit authoritative scanner and a Boolean indicating token presence.
Final empty tokens are delivered once. Invalid advances, counts, excessive
terminal empty tokens and later configuration changes are checked named failures.

</div>
</article>

<article class="doc-entry" id="v-scanner-split">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scanner-split</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner.Split/#t-ScanSplit" title="Effect.Scanner.Split.ScanSplit">ScanSplit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span>)</code><span class="doc-sig-links"><a href="#v-scanner-split" title="Permalink">#</a><a href="/packages/stdlib/Effect.Scanner/source/#L55" title="View source at line 55">Source</a></span></div>
<div class="doc-body">

Select a pure custom split capability before the first scan; later changes return a checked refusal.

</div>
</article>

<article class="doc-entry" id="v-scanner-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scanner-text</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-scanner-text" title="Permalink">#</a><a href="/packages/stdlib/Effect.Scanner/source/#L79" title="View source at line 79">Source</a></span></div>
<div class="doc-body">

Check the complete last-token bytes for UTF-8 validity without discarding malformed bytes from scanner-bytes.
An absent or empty token yields Some empty text; malformed bytes yield None.

</div>
</article>

<article class="doc-entry" id="v-scanner-token">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scanner-token</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Scanner/#t-Scanner" title="Effect.Scanner.Scanner">Scanner</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-scanner-token" title="Permalink">#</a><a href="/packages/stdlib/Effect.Scanner/source/#L67" title="View source at line 67">Source</a></span></div>
<div class="doc-body">

Observe a fresh optional last token, preserving absence separately from an explicitly empty token.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Scanner.cinch`](/packages/stdlib/Effect.Scanner/source/). Source SHA-256: `dd6d190f2b53e1fb80b10e48d64a29363b82041895660675d2b383c637be06aa`.
