# Effect.Hex

Hexadecimal encoding writers, decoding readers and formatted dump sinks with explicit continuations and sticky errors.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Hex</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Hex/source/">View source</a><a href="/packages/search/?q=%2BEffect.Hex">Search module</a><a href="/reference-sources/Effect.Hex.cinch" download="Effect.Hex.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Stream](Effect.Stream.md), [Text.Hex](Text.Hex.md).

<details class="doc-notes"><summary>Module notes</summary>

Hexadecimal encoding writers, decoding readers and formatted dump sinks.
Every operation returns its advanced continuation, including after failure.
Decoder errors are delayed until completed output has been read; a final
unpaired digit is StreamUnexpectedEnd. All stream failures are sticky.
Dump sinks buffer a partial sixteen-byte row until explicitly finished.
No adapter closes or flushes its underlying descriptor or sink.

</details>

<details class="doc-synopsis" open><summary>Synopsis (11 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-HexDumper">HexDumper</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HexDumper</strong> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#t-HexWriter">HexWriter</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HexWriter</strong> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-hex-dump-finish">hex-dump-finish</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-dump-finish</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexDumper" title="Effect.Hex.HexDumper">HexDumper</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexDumper" title="Effect.Hex.HexDumper">HexDumper</a> <span class="tok-variable">r</span>)</code></li><li><a class="syn-name" href="#v-hex-dump-write">hex-dump-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-dump-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexDumper" title="Effect.Hex.HexDumper">HexDumper</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexDumper" title="Effect.Hex.HexDumper">HexDumper</a> <span class="tok-variable">r</span>)</code></li><li><a class="syn-name" href="#v-hex-dumper">hex-dumper</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-dumper</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexDumper" title="Effect.Hex.HexDumper">HexDumper</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-hex-dumper-sink">hex-dumper-sink</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-dumper-sink</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexDumper" title="Effect.Hex.HexDumper">HexDumper</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-hex-encoding-writer">hex-encoding-writer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-encoding-writer</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-hex-reader">hex-reader</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-reader</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-hex-write">hex-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexWriter" title="Effect.Hex.HexWriter">HexWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexWriter" title="Effect.Hex.HexWriter">HexWriter</a> <span class="tok-variable">r</span>)</code></li><li><a class="syn-name" href="#v-hex-writer">hex-writer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-writer</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexWriter" title="Effect.Hex.HexWriter">HexWriter</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-hex-writer-sink">hex-writer-sink</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-writer-sink</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexWriter" title="Effect.Hex.HexWriter">HexWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-HexDumper">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HexDumper</strong> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#t-HexDumper" title="Permalink">#</a><a href="/packages/stdlib/Effect.Hex/source/#L123" title="View source at line 123">Source</a></span></div>
<div class="doc-body">

A formatted dump continuation with at most fifteen pending input bytes.

</div>
</article>

<article class="doc-entry" id="t-HexWriter">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HexWriter</strong> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#t-HexWriter" title="Permalink">#</a><a href="/packages/stdlib/Effect.Hex/source/#L23" title="View source at line 23">Source</a></span></div>
<div class="doc-body">

An explicitly advanced hexadecimal encoding sink with sticky failure state.

</div>
</article>

# Values

<article class="doc-entry" id="v-hex-dump-finish">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-dump-finish</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexDumper" title="Effect.Hex.HexDumper">HexDumper</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexDumper" title="Effect.Hex.HexDumper">HexDumper</a> <span class="tok-variable">r</span>)</code><span class="doc-sig-links"><a href="#v-hex-dump-finish" title="Permalink">#</a><a href="/packages/stdlib/Effect.Hex/source/#L167" title="View source at line 167">Source</a></span></div>
<div class="doc-body">

Flush the final partial row and retain a finished handle. This operation is idempotent.
It does not flush or close the underlying sink; inspect that sink separately when needed.

</div>
</article>

<article class="doc-entry" id="v-hex-dump-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-dump-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexDumper" title="Effect.Hex.HexDumper">HexDumper</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexDumper" title="Effect.Hex.HexDumper">HexDumper</a> <span class="tok-variable">r</span>)</code><span class="doc-sig-links"><a href="#v-hex-dump-write" title="Permalink">#</a><a href="/packages/stdlib/Effect.Hex/source/#L144" title="View source at line 144">Source</a></span></div>
<div class="doc-body">

Emit completed rows and snapshot the unfinished tail. Report accepted input bytes on success.
Finished handles reject writes with StreamClosed; output failures preserve their advanced sink.

</div>
</article>

<article class="doc-entry" id="v-hex-dumper">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-dumper</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexDumper" title="Effect.Hex.HexDumper">HexDumper</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-hex-dumper" title="Permalink">#</a><a href="/packages/stdlib/Effect.Hex/source/#L132" title="View source at line 132">Source</a></span></div>
<div class="doc-body">

Begin a hexadecimal dump whose first row starts at offset zero.

</div>
</article>

<article class="doc-entry" id="v-hex-dumper-sink">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-dumper-sink</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexDumper" title="Effect.Hex.HexDumper">HexDumper</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-hex-dumper-sink" title="Permalink">#</a><a href="/packages/stdlib/Effect.Hex/source/#L136" title="View source at line 136">Source</a></span></div>
<div class="doc-body">

Inspect output already accepted by the underlying dump sink, including after errors or finish.

</div>
</article>

<article class="doc-entry" id="v-hex-encoding-writer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-encoding-writer</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-hex-encoding-writer" title="Permalink">#</a><a href="/packages/stdlib/Effect.Hex/source/#L50" title="View source at line 50">Source</a></span></div>
<div class="doc-body">

Expose an encoding sink through the generic byte-writer protocol for composition.
Use the explicit HexWriter interface when the advanced underlying sink must be inspected.

</div>
</article>

<article class="doc-entry" id="v-hex-reader">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-reader</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-hex-reader" title="Permalink">#</a><a href="/packages/stdlib/Effect.Hex/source/#L87" title="View source at line 87">Source</a></span></div>
<div class="doc-body">

Decode a source in bounded 4096-byte input chunks, accepting either hexadecimal case.
Completed bytes precede any error. Zero reads do not advance the source; an unmatched
final digit reports StreamUnexpectedEnd 2 1. Invalid offsets count original input bytes.

</div>
</article>

<article class="doc-entry" id="v-hex-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexWriter" title="Effect.Hex.HexWriter">HexWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexWriter" title="Effect.Hex.HexWriter">HexWriter</a> <span class="tok-variable">r</span>)</code><span class="doc-sig-links"><a href="#v-hex-write" title="Permalink">#</a><a href="/packages/stdlib/Effect.Hex/source/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

Write all encoded output, retrying short writes, and report accepted input bytes.
On error the returned writer preserves the advanced sink and will not replay output.

</div>
</article>

<article class="doc-entry" id="v-hex-writer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-writer</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexWriter" title="Effect.Hex.HexWriter">HexWriter</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-hex-writer" title="Permalink">#</a><a href="/packages/stdlib/Effect.Hex/source/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">

Begin writing lowercase hexadecimal bytes to a sink. No final flush is required.

</div>
</article>

<article class="doc-entry" id="v-hex-writer-sink">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-writer-sink</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Hex/#t-HexWriter" title="Effect.Hex.HexWriter">HexWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-hex-writer-sink" title="Permalink">#</a><a href="/packages/stdlib/Effect.Hex/source/#L34" title="View source at line 34">Source</a></span></div>
<div class="doc-body">

Inspect the advanced underlying sink, including after a failed encoded write.

</div>
</article>
