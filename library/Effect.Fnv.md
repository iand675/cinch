# Effect.Fnv

Inspectable FNV hashing sinks, generic byte-writer composition and bounded reader-to-hash copying with advanced continuations.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Fnv</code></div><div class="doc-mod-bar-links"><a href="/library/source/Effect.Fnv/">View source</a><a href="/library/search/?q=%2BEffect.Fnv">Search module</a><a href="/reference-sources/Effect.Fnv.cinch" download="Effect.Fnv.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Effect.Stream](Effect.Stream.md), [Text.Fnv.State](Text.Fnv.State.md).

<details class="doc-closure"><summary>Ship-source closure (6 files)</summary>

Copy these modules beside your entry file before importing `Effect.Fnv`: [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Fnv](Effect.Fnv.md), [Effect.Stream](Effect.Stream.md), [Text.Binary](Text.Binary.md), [Text.Fnv](Text.Fnv.md), [Text.Fnv.State](Text.Fnv.State.md).

```sh
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Effect/Fnv.cinch ./Effect.Fnv.cinch
cp seed/prelude/Effect/Stream.cinch ./Effect.Stream.cinch
cp seed/prelude/Text/Binary.cinch ./Text.Binary.cinch
cp seed/prelude/Text/Fnv.cinch ./Text.Fnv.cinch
cp seed/prelude/Text/Fnv/State.cinch ./Text.Fnv.State.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

FNV hashing sinks with explicit, inspectable continuation states.
Every write accepts its entire input and preserves earlier hash handles.
Stream copying returns the advanced source and hash even on source failure.
No adapter retains input buffers or closes borrowed readers.

</details>

<details class="doc-synopsis" open><summary>Synopsis (6 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-FnvWriter">FnvWriter</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FnvWriter</strong></code></li><li><a class="syn-name" href="#v-fnv-copy">fnv-copy</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv-copy</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Fnv/#t-FnvWriter" title="Effect.Fnv.FnvWriter">FnvWriter</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, (<a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span>, <a class="doc-type-link tok-constructor" href="/library/Effect.Fnv/#t-FnvWriter" title="Effect.Fnv.FnvWriter">FnvWriter</a>))</code></li><li><a class="syn-name" href="#v-fnv-hashing-writer">fnv-hashing-writer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv-hashing-writer</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code></li><li><a class="syn-name" href="#v-fnv-write">fnv-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Fnv/#t-FnvWriter" title="Effect.Fnv.FnvWriter">FnvWriter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Effect.Fnv/#t-FnvWriter" title="Effect.Fnv.FnvWriter">FnvWriter</a>)</code></li><li><a class="syn-name" href="#v-fnv-writer">fnv-writer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv-writer</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Fnv/#t-FnvWriter" title="Effect.Fnv.FnvWriter">FnvWriter</a></code></li><li><a class="syn-name" href="#v-fnv-writer-state">fnv-writer-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv-writer-state</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Fnv/#t-FnvWriter" title="Effect.Fnv.FnvWriter">FnvWriter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-FnvWriter">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">FnvWriter</strong></code><span class="doc-sig-links"><a href="#t-FnvWriter" title="Permalink">#</a><a href="/library/source/Effect.Fnv/#L17" title="View source at line 17">Source</a></span></div>
<div class="doc-body">

An immutable scalar hash sink with no borrowed runtime resources.
Generic ByteWriter adapters carry their own runtime identity.

</div>
</article>

# Values

<article class="doc-entry" id="v-fnv-copy">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv-copy</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Fnv/#t-FnvWriter" title="Effect.Fnv.FnvWriter">FnvWriter</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, (<a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span>, <a class="doc-type-link tok-constructor" href="/library/Effect.Fnv/#t-FnvWriter" title="Effect.Fnv.FnvWriter">FnvWriter</a>))</code><span class="doc-sig-links"><a href="#v-fnv-copy" title="Permalink">#</a><a href="/library/source/Effect.Fnv/#L48" title="View source at line 48">Source</a></span></div>
<div class="doc-body">

Hash a reader through EOF, returning the accepted count and both advanced handles.
A zero chunk size is rejected without reading. Source errors retain the hash of
every earlier successful chunk and the advanced source returned with the failure.

</div>
</article>

<article class="doc-entry" id="v-fnv-hashing-writer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv-hashing-writer</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span></code><span class="doc-sig-links"><a href="#v-fnv-hashing-writer" title="Permalink">#</a><a href="/library/source/Effect.Fnv/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

Expose a hash-only sink through the generic writer protocol for composition.
Use FnvWriter or fnv-copy when the advanced hash must remain directly inspectable.

</div>
</article>

<article class="doc-entry" id="v-fnv-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Fnv/#t-FnvWriter" title="Effect.Fnv.FnvWriter">FnvWriter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Effect.Fnv/#t-FnvWriter" title="Effect.Fnv.FnvWriter">FnvWriter</a>)</code><span class="doc-sig-links"><a href="#v-fnv-write" title="Permalink">#</a><a href="/library/source/Effect.Fnv/#L31" title="View source at line 31">Source</a></span></div>
<div class="doc-body">

Hash every input byte and return its accepted count and advanced sink.
Empty input succeeds without changing the digest; resource exhaustion follows runtime policy.

</div>
</article>

<article class="doc-entry" id="v-fnv-writer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv-writer</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Fnv/#t-FnvWriter" title="Effect.Fnv.FnvWriter">FnvWriter</a></code><span class="doc-sig-links"><a href="#v-fnv-writer" title="Permalink">#</a><a href="/library/source/Effect.Fnv/#L22" title="View source at line 22">Source</a></span></div>
<div class="doc-body">

Begin a sink from any initial or restored FNV state.

</div>
</article>

<article class="doc-entry" id="v-fnv-writer-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv-writer-state</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Fnv/#t-FnvWriter" title="Effect.Fnv.FnvWriter">FnvWriter</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Fnv.State/#t-FnvState" title="Text.Fnv.State.FnvState">FnvState</a></code><span class="doc-sig-links"><a href="#v-fnv-writer-state" title="Permalink">#</a><a href="/library/source/Effect.Fnv/#L26" title="View source at line 26">Source</a></span></div>
<div class="doc-body">

Inspect the current hash without advancing the sink or changing earlier handles.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Fnv.cinch`](/library/source/Effect.Fnv/). Source SHA-256: `7253c58aac971fe5a2879b5cfda24593dbc52cd69bdbaff1d7ddfdf1c9db5464`.
