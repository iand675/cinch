# Effect.Stm.TSem

A counting semaphore for transactions: wait takes a permit or retries, signal returns one.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Effect.Stm.TSem</code></div><div class="doc-mod-bar-links"><a href="/library/source/Effect.Stm.TSem/">View source</a><a href="/library/search/?q=%2BEffect.Stm.TSem">Search module</a><a href="/reference-sources/Effect.Stm.TSem.cinch" download="Effect.Stm.TSem.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Effect.Stm](Effect.Stm.md).

<details class="doc-notes"><summary>Module notes</summary>

Effect.Stm.TSem: a counting semaphore of transactions.

A semaphore holds a number of permits. `tsem-wait` takes one and
waits while there are none; `tsem-signal` returns one. Because both
are transactions, taking a permit composes with whatever else the
transaction does: it takes the permit and does the rest, or neither.

There is no fairness: a returned permit goes to whichever waiter
commits first.

Module scheme (design/base-library.md): a concrete type over
`Effect.Stm`, shipped beside it.

</details>

<details class="doc-synopsis" open><summary>Synopsis (8 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-TSem">TSem</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TSem</strong></code></li><li><a class="syn-name" href="#v-tsem-available">tsem-available</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tsem-available</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TSem/#t-TSem" title="Effect.Stm.TSem.TSem">TSem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tsem-new">tsem-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tsem-new</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TSem/#t-TSem" title="Effect.Stm.TSem.TSem">TSem</a></code></li><li><a class="syn-name" href="#v-tsem-new-io">tsem-new-io</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tsem-new-io</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/library/Effect.Stm/" title="STM">STM</a>} <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TSem/#t-TSem" title="Effect.Stm.TSem.TSem">TSem</a></code></li><li><a class="syn-name" href="#v-tsem-signal">tsem-signal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tsem-signal</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TSem/#t-TSem" title="Effect.Stm.TSem.TSem">TSem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-tsem-signal-n">tsem-signal-n</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tsem-signal-n</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TSem/#t-TSem" title="Effect.Stm.TSem.TSem">TSem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-tsem-try-wait">tsem-try-wait</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tsem-try-wait</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TSem/#t-TSem" title="Effect.Stm.TSem.TSem">TSem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-tsem-wait">tsem-wait</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tsem-wait</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TSem/#t-TSem" title="Effect.Stm.TSem.TSem">TSem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-TSem">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TSem</strong></code><span class="doc-sig-links"><a href="#t-TSem" title="Permalink">#</a><a href="/library/source/Effect.Stm.TSem/#L19" title="View source at line 19">Source</a></span></div>
</article>

# Values

<article class="doc-entry" id="v-tsem-available">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tsem-available</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TSem/#t-TSem" title="Effect.Stm.TSem.TSem">TSem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tsem-available" title="Permalink">#</a><a href="/library/source/Effect.Stm.TSem/#L47" title="View source at line 47">Source</a></span></div>
<div class="doc-body">

The permits available at the transaction's instant.

</div>
</article>

<article class="doc-entry" id="v-tsem-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tsem-new</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TSem/#t-TSem" title="Effect.Stm.TSem.TSem">TSem</a></code><span class="doc-sig-links"><a href="#v-tsem-new" title="Permalink">#</a><a href="/library/source/Effect.Stm.TSem/#L23" title="View source at line 23">Source</a></span></div>
<div class="doc-body">

A semaphore with `permits` permits.

</div>
</article>

<article class="doc-entry" id="v-tsem-new-io">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tsem-new-io</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/library/Effect.Stm/" title="STM">STM</a>} <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TSem/#t-TSem" title="Effect.Stm.TSem.TSem">TSem</a></code><span class="doc-sig-links"><a href="#v-tsem-new-io" title="Permalink">#</a><a href="/library/source/Effect.Stm.TSem/#L26" title="View source at line 26">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tsem-signal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tsem-signal</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TSem/#t-TSem" title="Effect.Stm.TSem.TSem">TSem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-tsem-signal" title="Permalink">#</a><a href="/library/source/Effect.Stm.TSem/#L39" title="View source at line 39">Source</a></span></div>
<div class="doc-body">

Return a permit.

</div>
</article>

<article class="doc-entry" id="v-tsem-signal-n">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tsem-signal-n</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TSem/#t-TSem" title="Effect.Stm.TSem.TSem">TSem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-tsem-signal-n" title="Permalink">#</a><a href="/library/source/Effect.Stm.TSem/#L43" title="View source at line 43">Source</a></span></div>
<div class="doc-body">

Return `k` permits at once.

</div>
</article>

<article class="doc-entry" id="v-tsem-try-wait">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tsem-try-wait</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TSem/#t-TSem" title="Effect.Stm.TSem.TSem">TSem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-tsem-try-wait" title="Permalink">#</a><a href="/library/source/Effect.Stm.TSem/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">

Take a permit, or answer False when there is none.

</div>
</article>

<article class="doc-entry" id="v-tsem-wait">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tsem-wait</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TSem/#t-TSem" title="Effect.Stm.TSem.TSem">TSem</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-tsem-wait" title="Permalink">#</a><a href="/library/source/Effect.Stm.TSem/#L35" title="View source at line 35">Source</a></span></div>
<div class="doc-body">

Take a permit. Waits while there is none.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Stm/TSem.cinch`](/library/source/Effect.Stm.TSem/). Source SHA-256: `a493de2f8f16e4be3fa641174b9d1c669597641fd64857cde9fa5f8d80c4947d`.
