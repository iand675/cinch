# Effect.Concurrent

Shipped task/worker vocabulary supplementing the compiler-installed structured scope and join operations.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Concurrent</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Concurrent/source/">View source</a><a href="/packages/search/?q=%2BEffect.Concurrent">Search module</a><a href="/reference-sources/Effect.Concurrent.cinch" download="Effect.Concurrent.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Result](Data.Result.md).

<details class="doc-notes"><summary>Module notes</summary>

Effect.Concurrent: the structured concurrency worked vocabulary.

The worked vocabulary of the concurrency specification's `loadPair`
sketch (section 2). A `Request` names bytes to read from the
runtime's input source; `fetch` performs the read under the `Io r`
capability and reports a short read as a `LoadError`. `withScope`,
`spawn`, `joinAllOrCancel` and the `Io r` operations (`ioWrite`,
`ioRead`, `ioSleep`, `pollCancel`) are builtin values with
compiler-known schemes (like `newIO`), never foreign imports.

</details>

<details class="doc-synopsis" open><summary>Synopsis (3 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-LoadError">LoadError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">LoadError</strong></code></li><li><a class="syn-name" href="#t-Request">Request</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Request</strong></code></li><li><a class="syn-name" href="#v-fetch">fetch</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fetch</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent/#t-Request" title="Effect.Concurrent.Request">Request</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent/#t-LoadError" title="Effect.Concurrent.LoadError">LoadError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-LoadError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">LoadError</strong></code><span class="doc-sig-links"><a href="#t-LoadError" title="Permalink">#</a><a href="/packages/stdlib/Effect.Concurrent/source/#L28" title="View source at line 28">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-LoadError"><code><strong class="tok-constructor">LoadError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent/#t-LoadError" title="Effect.Concurrent.LoadError">LoadError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Request">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Request</strong></code><span class="doc-sig-links"><a href="#t-Request" title="Permalink">#</a><a href="/packages/stdlib/Effect.Concurrent/source/#L24" title="View source at line 24">Source</a></span></div>
<div class="doc-body">

Structured concurrency (concurrency specification, section 2): the
worked vocabulary of the spec's `loadPair` sketch. A `Request` names
bytes to read from the runtime's input source; `fetch` performs the
read under the `Io r` capability and reports a short read as a
`LoadError`. `withScope`, `spawn`, `joinAllOrCancel` and the `Io r`
operations (`ioWrite`, `ioRead`, `ioSleep`, `pollCancel`) are builtin
values with compiler-known schemes (like `newIO`), never foreign
imports.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Request"><code><strong class="tok-constructor">Request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent/#t-Request" title="Effect.Concurrent.Request">Request</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-fetch">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fetch</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent/#t-Request" title="Effect.Concurrent.Request">Request</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent/#t-LoadError" title="Effect.Concurrent.LoadError">LoadError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-fetch" title="Permalink">#</a><a href="/packages/stdlib/Effect.Concurrent/source/#L30" title="View source at line 30">Source</a></span></div>
</article>
