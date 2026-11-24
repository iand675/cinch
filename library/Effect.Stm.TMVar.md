# Effect.Stm.TMVar

A transactional variable that is either empty or full: take waits while empty, put waits while full. The building block for handoff and mutual exclusion inside transactions.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Effect.Stm.TMVar</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Stm.TMVar/source/">View source</a><a href="/packages/search/?q=%2BEffect.Stm.TMVar">Search module</a><a href="/reference-sources/Effect.Stm.TMVar.cinch" download="Effect.Stm.TMVar.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Effect.Stm](Effect.Stm.md).

<details class="doc-notes"><summary>Module notes</summary>

Effect.Stm.TMVar: a transactional variable that is either full or empty.

`tmvar-take` waits for a value and leaves the variable empty;
`tmvar-put` waits for room and leaves it full. Both wait by `retry`,
so they compose: a transaction that takes from two of them takes
both or neither, and `or-else` chooses between them.

Used as a lock, an empty variable is "held": take to acquire, put to
release. Used as a one-place channel, a put is a send and a take is
a receive, and the value changes hands at the taker's commit.

There is no fairness: when several tasks wait on one variable, the
next value goes to whichever of them commits first.

Module scheme (design/base-library.md): a concrete type over
`Effect.Stm`, shipped beside it.

</details>

<details class="doc-synopsis" open><summary>Synopsis (13 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-TMVar">TMVar</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TMVar</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tmvar-is-empty">tmvar-is-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tmvar-is-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-tmvar-new">tmvar-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tmvar-new</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-tmvar-new-empty">tmvar-new-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tmvar-new-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-tmvar-new-empty-io">tmvar-new-empty-io</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tmvar-new-empty-io</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="STM">STM</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tmvar-new-io">tmvar-new-io</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tmvar-new-io</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="STM">STM</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tmvar-put">tmvar-put</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tmvar-put</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-tmvar-read">tmvar-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tmvar-read</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tmvar-swap">tmvar-swap</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tmvar-swap</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tmvar-take">tmvar-take</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tmvar-take</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tmvar-try-put">tmvar-try-put</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tmvar-try-put</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-tmvar-try-read">tmvar-try-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tmvar-try-read</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-tmvar-try-take">tmvar-try-take</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tmvar-try-take</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>)</code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-TMVar">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TMVar</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-TMVar" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TMVar/source/#L25" title="View source at line 25">Source</a></span></div>
</article>

# Values

<article class="doc-entry" id="v-tmvar-is-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tmvar-is-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-tmvar-is-empty" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TMVar/source/#L84" title="View source at line 84">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tmvar-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tmvar-new</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-tmvar-new" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TMVar/source/#L32" title="View source at line 32">Source</a></span></div>
<div class="doc-body">

A full variable holding `x`.

</div>
</article>

<article class="doc-entry" id="v-tmvar-new-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tmvar-new-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-tmvar-new-empty" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TMVar/source/#L35" title="View source at line 35">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tmvar-new-empty-io">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tmvar-new-empty-io</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="STM">STM</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-tmvar-new-empty-io" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TMVar/source/#L41" title="View source at line 41">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tmvar-new-io">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tmvar-new-io</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="STM">STM</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-tmvar-new-io" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TMVar/source/#L38" title="View source at line 38">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tmvar-put">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tmvar-put</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-tmvar-put" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TMVar/source/#L51" title="View source at line 51">Source</a></span></div>
<div class="doc-body">

Fill the variable. Waits while it is full.

</div>
</article>

<article class="doc-entry" id="v-tmvar-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tmvar-read</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-tmvar-read" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TMVar/source/#L57" title="View source at line 57">Source</a></span></div>
<div class="doc-body">

The value, leaving it in place. Waits while the variable is empty.

</div>
</article>

<article class="doc-entry" id="v-tmvar-swap">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tmvar-swap</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-tmvar-swap" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TMVar/source/#L63" title="View source at line 63">Source</a></span></div>
<div class="doc-body">

Replace the value and answer the old one. Waits while empty.

</div>
</article>

<article class="doc-entry" id="v-tmvar-take">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tmvar-take</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-tmvar-take" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TMVar/source/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

The value, leaving the variable empty. Waits while it is empty.

</div>
</article>

<article class="doc-entry" id="v-tmvar-try-put">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tmvar-try-put</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-tmvar-try-put" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TMVar/source/#L76" title="View source at line 76">Source</a></span></div>
<div class="doc-body">

`tmvar-put` that answers whether it filled the variable instead of
waiting.

</div>
</article>

<article class="doc-entry" id="v-tmvar-try-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tmvar-try-read</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-tmvar-try-read" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TMVar/source/#L81" title="View source at line 81">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tmvar-try-take">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tmvar-try-take</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TMVar/#t-TMVar" title="Effect.Stm.TMVar.TMVar">TMVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-tmvar-try-take" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TMVar/source/#L69" title="View source at line 69">Source</a></span></div>
<div class="doc-body">

`tmvar-take` that answers `None` instead of waiting.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Stm/TMVar.cinch`](/packages/stdlib/Effect.Stm.TMVar/source/). Source SHA-256: `6b18623aeea5cc48d7e0b68c901b7242d38f9ca28f142d9a8f609a6771f9fba4`.
