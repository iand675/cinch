# Effect.Stm.TQueue

An unbounded first-in first-out queue for transactions, as two lists so that writers and readers touch different variables.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Effect.Stm.TQueue</code></div><div class="doc-mod-bar-links"><a href="/library/source/Effect.Stm.TQueue/">View source</a><a href="/library/search/?q=%2BEffect.Stm.TQueue">Search module</a><a href="/reference-sources/Effect.Stm.TQueue.cinch" download="Effect.Stm.TQueue.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.List](Data.List.md), [Effect.Stm](Effect.Stm.md).

<details class="doc-notes"><summary>Module notes</summary>

Effect.Stm.TQueue: an unbounded first-in, first-out queue of
transactions.

The queue is two lists in two variables: writers push onto the back
list, readers pop from the front list, and a reader that finds the
front empty takes the whole back list, reversed. Each element is
moved once, so a write and a read cost constant time amortized, and
writers and readers touch different variables: a write does not
conflict with a read unless the front has run dry.

`tqueue-read` waits while the queue is empty. Nothing bounds the
queue, so a writer never waits and a producer that outruns its
consumer grows the heap; `Effect.Stm.TBQueue` is the bounded form.

Module scheme (design/base-library.md): a concrete type over
`Effect.Stm`, shipped beside it.

</details>

<details class="doc-synopsis" open><summary>Synopsis (10 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-TQueue">TQueue</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TQueue</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tqueue-flush">tqueue-flush</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tqueue-flush</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TQueue/#t-TQueue" title="Effect.Stm.TQueue.TQueue">TQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-tqueue-is-empty">tqueue-is-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tqueue-is-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TQueue/#t-TQueue" title="Effect.Stm.TQueue.TQueue">TQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-tqueue-new">tqueue-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tqueue-new</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TQueue/#t-TQueue" title="Effect.Stm.TQueue.TQueue">TQueue</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-tqueue-new-io">tqueue-new-io</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tqueue-new-io</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/library/Effect.Stm/" title="STM">STM</a>} <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TQueue/#t-TQueue" title="Effect.Stm.TQueue.TQueue">TQueue</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tqueue-peek">tqueue-peek</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tqueue-peek</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TQueue/#t-TQueue" title="Effect.Stm.TQueue.TQueue">TQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tqueue-read">tqueue-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tqueue-read</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TQueue/#t-TQueue" title="Effect.Stm.TQueue.TQueue">TQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tqueue-try-read">tqueue-try-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tqueue-try-read</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TQueue/#t-TQueue" title="Effect.Stm.TQueue.TQueue">TQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-tqueue-unget">tqueue-unget</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tqueue-unget</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TQueue/#t-TQueue" title="Effect.Stm.TQueue.TQueue">TQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-tqueue-write">tqueue-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tqueue-write</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TQueue/#t-TQueue" title="Effect.Stm.TQueue.TQueue">TQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-TQueue">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TQueue</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-TQueue" title="Permalink">#</a><a href="/library/source/Effect.Stm.TQueue/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">

The front (next out first) and the back (last in first).

</div>
</article>

# Values

<article class="doc-entry" id="v-tqueue-flush">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tqueue-flush</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TQueue/#t-TQueue" title="Effect.Stm.TQueue.TQueue">TQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-tqueue-flush" title="Permalink">#</a><a href="/library/source/Effect.Stm.TQueue/#L63" title="View source at line 63">Source</a></span></div>
<div class="doc-body">

Every element, oldest first, leaving the queue empty.

</div>
</article>

<article class="doc-entry" id="v-tqueue-is-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tqueue-is-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TQueue/#t-TQueue" title="Effect.Stm.TQueue.TQueue">TQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-tqueue-is-empty" title="Permalink">#</a><a href="/library/source/Effect.Stm.TQueue/#L74" title="View source at line 74">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tqueue-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tqueue-new</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TQueue/#t-TQueue" title="Effect.Stm.TQueue.TQueue">TQueue</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-tqueue-new" title="Permalink">#</a><a href="/library/source/Effect.Stm.TQueue/#L30" title="View source at line 30">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tqueue-new-io">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tqueue-new-io</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/library/Effect.Stm/" title="STM">STM</a>} <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TQueue/#t-TQueue" title="Effect.Stm.TQueue.TQueue">TQueue</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-tqueue-new-io" title="Permalink">#</a><a href="/library/source/Effect.Stm.TQueue/#L33" title="View source at line 33">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tqueue-peek">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tqueue-peek</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TQueue/#t-TQueue" title="Effect.Stm.TQueue.TQueue">TQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-tqueue-peek" title="Permalink">#</a><a href="/library/source/Effect.Stm.TQueue/#L55" title="View source at line 55">Source</a></span></div>
<div class="doc-body">

The oldest element, left in place. Waits while the queue is empty.

</div>
</article>

<article class="doc-entry" id="v-tqueue-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tqueue-read</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TQueue/#t-TQueue" title="Effect.Stm.TQueue.TQueue">TQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-tqueue-read" title="Permalink">#</a><a href="/library/source/Effect.Stm.TQueue/#L49" title="View source at line 49">Source</a></span></div>
<div class="doc-body">

The oldest element, removed. Waits while the queue is empty.

</div>
</article>

<article class="doc-entry" id="v-tqueue-try-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tqueue-try-read</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TQueue/#t-TQueue" title="Effect.Stm.TQueue.TQueue">TQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-tqueue-try-read" title="Permalink">#</a><a href="/library/source/Effect.Stm.TQueue/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

The oldest element, removed, or `None` when the queue is empty.

</div>
</article>

<article class="doc-entry" id="v-tqueue-unget">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tqueue-unget</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TQueue/#t-TQueue" title="Effect.Stm.TQueue.TQueue">TQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-tqueue-unget" title="Permalink">#</a><a href="/library/source/Effect.Stm.TQueue/#L59" title="View source at line 59">Source</a></span></div>
<div class="doc-body">

Put `x` back at the front: the next read answers it.

</div>
</article>

<article class="doc-entry" id="v-tqueue-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tqueue-write</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Effect.Stm.TQueue/#t-TQueue" title="Effect.Stm.TQueue.TQueue">TQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-tqueue-write" title="Permalink">#</a><a href="/library/source/Effect.Stm.TQueue/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

Append `x`. Never waits.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Stm/TQueue.cinch`](/library/source/Effect.Stm.TQueue/). Source SHA-256: `706dd386f8128e43835dc680ac787a5f8af5d02625735ae59458f916c56f4eb2`.
