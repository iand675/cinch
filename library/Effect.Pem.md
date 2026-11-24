# Effect.Pem

Validated envelope output through explicit advanced writer continuations, bounded chunks and linear short-write draining.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Pem</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Pem/source/">View source</a><a href="/packages/search/?q=%2BEffect.Pem">Search module</a><a href="/reference-sources/Effect.Pem.cinch" download="Effect.Pem.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Text.Pem](Text.Pem.md).

<details class="doc-notes"><summary>Module notes</summary>

One-shot PEM writing with bounded payload chunks and explicit advanced sink continuations.
Validation happens before output. Positive short writes are drained; transport
failure returns the advanced sink without closing or flushing borrowed resources.

</details>

<details class="doc-synopsis" open><summary>Synopsis (3 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-PemError">PemError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PemError</strong></code></li><li><a class="syn-name" href="#v-pem-error-message">pem-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pem-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pem/#t-PemError" title="Text.Pem.PemError">PemError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pem-write">pem-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pem-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pem/#t-PemError" title="Text.Pem.PemError">PemError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span>)</code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-PemError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PemError</strong></code><span class="doc-sig-links"><a href="#t-PemError" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L38" title="View source at line 38">Source</a></span></div>
<div class="doc-body">

Encoding validation or transport failure, with no implicit sink closure.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PemInvalidHeaderKey"><code><strong class="tok-constructor">PemInvalidHeaderKey</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pem/#t-PemError" title="Text.Pem.PemError">PemError</a></code></li><li id="k-PemOutputError"><code><strong class="tok-constructor">PemOutputError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pem/#t-PemError" title="Text.Pem.PemError">PemError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-pem-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pem-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pem/#t-PemError" title="Text.Pem.PemError">PemError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pem-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

Explain a header refusal or identify a retained underlying stream failure.

</div>
</article>

<article class="doc-entry" id="v-pem-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pem-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pem/#t-PemError" title="Text.Pem.PemError">PemError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span>)</code><span class="doc-sig-links"><a href="#v-pem-write" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L753" title="View source at line 753">Source</a></span></div>
<div class="doc-body">

Write an envelope with bounded payload storage and explicit continuation ownership.
Validate all keys before any callback. Positive short writes are retried; zero progress,
invalid counts and sink errors preserve the advanced writer and never replay accepted
output. Chunk boundaries are unspecified. This operation never flushes or closes its sink.

</div>
</article>
