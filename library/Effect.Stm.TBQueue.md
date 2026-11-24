# Effect.Stm.TBQueue

A bounded first-in first-out queue for transactions: a write waits while the queue is at capacity, a read waits while it is empty.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Stm.TBQueue</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Stm.TBQueue/source/">View source</a><a href="/packages/search/?q=%2BEffect.Stm.TBQueue">Search module</a><a href="/reference-sources/Effect.Stm.TBQueue.cinch" download="Effect.Stm.TBQueue.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.List](Data.List.md), [Effect.Stm](Effect.Stm.md).

<details class="doc-notes"><summary>Module notes</summary>

Effect.Stm.TBQueue: a bounded first-in, first-out queue of
transactions.

The bounded channel of the concurrency specification's section 5,
built from transactions: the capacity is explicit, a writer waits
while the queue is full, and a reader waits while it is empty, so a
producer cannot outrun its consumer by more than the capacity.

Ownership of an element passes at a commit, and only there: a write
that has not committed has put nothing in the queue, and a read that
has not committed has taken nothing out. A task cancelled while it
waits (`atomically-or-cancel`) therefore neither loses an element nor
leaves half of one behind.

The representation is `Effect.Stm.TQueue`'s two lists plus a count.
The count is one more variable every write and every read both
change, so unlike the unbounded queue a writer and a reader do
conflict; that is the price of the bound.

Module scheme (design/base-library.md): a concrete type over
`Effect.Stm`, shipped beside it.

</details>

<details class="doc-synopsis" open><summary>Synopsis (11 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-TBQueue">TBQueue</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TBQueue</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tbqueue-capacity">tbqueue-capacity</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tbqueue-capacity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tbqueue-is-empty">tbqueue-is-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tbqueue-is-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-tbqueue-is-full">tbqueue-is-full</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tbqueue-is-full</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-tbqueue-length">tbqueue-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tbqueue-length</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tbqueue-new">tbqueue-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tbqueue-new</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-tbqueue-new-io">tbqueue-new-io</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tbqueue-new-io</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="STM">STM</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tbqueue-read">tbqueue-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tbqueue-read</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tbqueue-try-read">tbqueue-try-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tbqueue-try-read</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>)</code></li><li><a class="syn-name" href="#v-tbqueue-try-write">tbqueue-try-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tbqueue-try-write</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-tbqueue-write">tbqueue-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tbqueue-write</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-TBQueue">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TBQueue</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-TBQueue" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TBQueue/source/#L33" title="View source at line 33">Source</a></span></div>
<div class="doc-body">

The capacity, the number of elements held, the front (next out
first) and the back (last in first).

</div>
</article>

# Values

<article class="doc-entry" id="v-tbqueue-capacity">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tbqueue-capacity</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tbqueue-capacity" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TBQueue/source/#L47" title="View source at line 47">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tbqueue-is-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tbqueue-is-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-tbqueue-is-empty" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TBQueue/source/#L82" title="View source at line 82">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tbqueue-is-full">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tbqueue-is-full</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-tbqueue-is-full" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TBQueue/source/#L85" title="View source at line 85">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tbqueue-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tbqueue-length</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tbqueue-length" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TBQueue/source/#L79" title="View source at line 79">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tbqueue-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tbqueue-new</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-tbqueue-new" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TBQueue/source/#L38" title="View source at line 38">Source</a></span></div>
<div class="doc-body">

A queue that holds at most `capacity` elements. A capacity of zero
makes a queue no write can ever enter.

</div>
</article>

<article class="doc-entry" id="v-tbqueue-new-io">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tbqueue-new-io</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/" title="STM">STM</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-tbqueue-new-io" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TBQueue/source/#L44" title="View source at line 44">Source</a></span></div>
</article>

<article class="doc-entry" id="v-tbqueue-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tbqueue-read</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-tbqueue-read" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TBQueue/source/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

The oldest element, removed. Waits while the queue is empty.

</div>
</article>

<article class="doc-entry" id="v-tbqueue-try-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tbqueue-try-read</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>)</code><span class="doc-sig-links"><a href="#v-tbqueue-try-read" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TBQueue/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

The oldest element, removed, or `None` when the queue is empty.

</div>
</article>

<article class="doc-entry" id="v-tbqueue-try-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tbqueue-try-write</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-tbqueue-try-write" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TBQueue/source/#L51" title="View source at line 51">Source</a></span></div>
<div class="doc-body">

Append `x`, or answer False when the queue is full.

</div>
</article>

<article class="doc-entry" id="v-tbqueue-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tbqueue-write</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm.TBQueue/#t-TBQueue" title="Effect.Stm.TBQueue.TBQueue">TBQueue</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-tbqueue-write" title="Permalink">#</a><a href="/packages/stdlib/Effect.Stm.TBQueue/source/#L58" title="View source at line 58">Source</a></span></div>
<div class="doc-body">

Append `x`. Waits while the queue is full.

</div>
</article>
