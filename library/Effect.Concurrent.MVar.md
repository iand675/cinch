# Effect.Concurrent.MVar

Native single-place shared variables with FIFO handoff, blocking take/put/read/swap, nonblocking operations, and cancellable waits through the task scheduler.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Effect.Concurrent.MVar</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Concurrent.MVar/source/">View source</a><a href="/packages/search/?q=%2BEffect.Concurrent.MVar">Search module</a><a href="/reference-sources/Effect.Concurrent.MVar.cinch" download="Effect.Concurrent.MVar.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Option](Data.Option.md).

<details class="doc-notes"><summary>Module notes</summary>

A native, shared single-place variable with FIFO handoff.

Take waits while empty; put waits while full. Read leaves the value
in place and swap replaces it atomically. Blocked takers (including
swaps) and putters are served in registration order. A put completes
every reader waiting for that value, even when a taker receives it
immediately. Nonblocking calls cannot steal reserved values or space.

Each variable owns a runtime mutex and FIFO wait queues. Waiting parks
the current scheduler context, giving its worker to another task. No
STM, global variable registry, or busy polling is involved. The MVar
effect labels access; blocking calls additionally carry Diverge.

The -or-cancel forms remove pending requests before returning None
(False for put). A committed handoff wins a cancellation race: the
call returns the value/success, so cancellation cannot lose a value.
Plain calls wait regardless of cooperative cancellation. As with
other cancellable waits, immediately available operations succeed
even when cancellation has already been requested.

Constructors are private. A handle is Copy, but crossing a task
boundary requires its contents to be shareable. is-empty is only a
snapshot. FIFO starts at registration, not before acquiring the mutex;
no bound on scheduling or mutex acquisition is promised.

</details>

<details class="doc-synopsis" open><summary>Synopsis (15 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-MVar">MVar</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MVar</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-mvar-is-empty">mvar-is-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mvar-is-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-mvar-new">mvar-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mvar-new</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-mvar-new-empty">mvar-new-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mvar-new-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-mvar-put">mvar-put</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mvar-put</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-mvar-put-or-cancel">mvar-put-or-cancel</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mvar-put-or-cancel</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-mvar-read">mvar-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mvar-read</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-mvar-read-or-cancel">mvar-read-or-cancel</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mvar-read-or-cancel</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-mvar-swap">mvar-swap</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mvar-swap</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-mvar-swap-or-cancel">mvar-swap-or-cancel</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mvar-swap-or-cancel</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-mvar-take">mvar-take</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mvar-take</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-mvar-take-or-cancel">mvar-take-or-cancel</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mvar-take-or-cancel</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-mvar-try-put">mvar-try-put</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mvar-try-put</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-mvar-try-read">mvar-try-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mvar-try-read</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-mvar-try-take">mvar-try-take</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mvar-try-take</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-MVar">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MVar</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-MVar" title="Permalink">#</a><a href="/packages/stdlib/Effect.Concurrent.MVar/source/#L35" title="View source at line 35">Source</a></span></div>
<div class="doc-body">

A shared single-place variable whose operations preserve stored-value ownership. Constructors are private; task sharing requires shareable contents.

</div>
</article>

# Values

<article class="doc-entry" id="v-mvar-is-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mvar-is-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-mvar-is-empty" title="Permalink">#</a><a href="/packages/stdlib/Effect.Concurrent.MVar/source/#L105" title="View source at line 105">Source</a></span></div>
<div class="doc-body">

Return an instantaneous empty-state observation. The result does not reserve space or a value.

</div>
</article>

<article class="doc-entry" id="v-mvar-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mvar-new</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-mvar-new" title="Permalink">#</a><a href="/packages/stdlib/Effect.Concurrent.MVar/source/#L69" title="View source at line 69">Source</a></span></div>
<div class="doc-body">

Create a full shared variable containing the supplied value.

</div>
</article>

<article class="doc-entry" id="v-mvar-new-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mvar-new-empty</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-mvar-new-empty" title="Permalink">#</a><a href="/packages/stdlib/Effect.Concurrent.MVar/source/#L73" title="View source at line 73">Source</a></span></div>
<div class="doc-body">

Create an empty shared variable; a take or read waits until a value arrives.

</div>
</article>

<article class="doc-entry" id="v-mvar-put">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mvar-put</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-mvar-put" title="Permalink">#</a><a href="/packages/stdlib/Effect.Concurrent.MVar/source/#L81" title="View source at line 81">Source</a></span></div>
<div class="doc-body">

Wait until space is available and store the supplied value. Registered putters are served in FIFO order.

</div>
</article>

<article class="doc-entry" id="v-mvar-put-or-cancel">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mvar-put-or-cancel</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-mvar-put-or-cancel" title="Permalink">#</a><a href="/packages/stdlib/Effect.Concurrent.MVar/source/#L121" title="View source at line 121">Source</a></span></div>
<div class="doc-body">

Store the value and return True, or return False if cancellation wins before handoff. A committed put succeeds.

</div>
</article>

<article class="doc-entry" id="v-mvar-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mvar-read</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-mvar-read" title="Permalink">#</a><a href="/packages/stdlib/Effect.Concurrent.MVar/source/#L85" title="View source at line 85">Source</a></span></div>
<div class="doc-body">

Wait until a value is available and return it without emptying the variable.

</div>
</article>

<article class="doc-entry" id="v-mvar-read-or-cancel">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mvar-read-or-cancel</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-mvar-read-or-cancel" title="Permalink">#</a><a href="/packages/stdlib/Effect.Concurrent.MVar/source/#L113" title="View source at line 113">Source</a></span></div>
<div class="doc-body">

Read a value without removal, or return None if cancellation wins before delivery. Immediate reads succeed despite prior cancellation.

</div>
</article>

<article class="doc-entry" id="v-mvar-swap">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mvar-swap</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-mvar-swap" title="Permalink">#</a><a href="/packages/stdlib/Effect.Concurrent.MVar/source/#L89" title="View source at line 89">Source</a></span></div>
<div class="doc-body">

Wait until a value is available, replace it atomically, and return the previous value.

</div>
</article>

<article class="doc-entry" id="v-mvar-swap-or-cancel">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mvar-swap-or-cancel</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-mvar-swap-or-cancel" title="Permalink">#</a><a href="/packages/stdlib/Effect.Concurrent.MVar/source/#L117" title="View source at line 117">Source</a></span></div>
<div class="doc-body">

Exchange values or return None if cancellation wins before handoff. A committed exchange returns the previous value.

</div>
</article>

<article class="doc-entry" id="v-mvar-take">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mvar-take</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-mvar-take" title="Permalink">#</a><a href="/packages/stdlib/Effect.Concurrent.MVar/source/#L77" title="View source at line 77">Source</a></span></div>
<div class="doc-body">

Wait until a value is available, remove it, and return it. Registered takers are served in FIFO order.

</div>
</article>

<article class="doc-entry" id="v-mvar-take-or-cancel">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mvar-take-or-cancel</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/13-concurrency-and-runtime/" title="Cancel">Cancel</a> r, <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Diverge">Diverge</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-mvar-take-or-cancel" title="Permalink">#</a><a href="/packages/stdlib/Effect.Concurrent.MVar/source/#L109" title="View source at line 109">Source</a></span></div>
<div class="doc-body">

Take a value or return None if cancellation wins before handoff. An already committed handoff succeeds.

</div>
</article>

<article class="doc-entry" id="v-mvar-try-put">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mvar-try-put</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-mvar-try-put" title="Permalink">#</a><a href="/packages/stdlib/Effect.Concurrent.MVar/source/#L97" title="View source at line 97">Source</a></span></div>
<div class="doc-body">

Store the value and return True when space is immediately available; otherwise return False without waiting.

</div>
</article>

<article class="doc-entry" id="v-mvar-try-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mvar-try-read</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-mvar-try-read" title="Permalink">#</a><a href="/packages/stdlib/Effect.Concurrent.MVar/source/#L101" title="View source at line 101">Source</a></span></div>
<div class="doc-body">

Return an immediately available value without removing it, or return None without waiting.

</div>
</article>

<article class="doc-entry" id="v-mvar-try-take">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mvar-try-take</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/#t-MVar" title="Effect.Concurrent.MVar.MVar">MVar</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Concurrent.MVar/" title="MVar">MVar</a>} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-mvar-try-take" title="Permalink">#</a><a href="/packages/stdlib/Effect.Concurrent.MVar/source/#L93" title="View source at line 93">Source</a></span></div>
<div class="doc-body">

Remove and return an immediately available value, or return None without waiting.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Concurrent/MVar.cinch`](/packages/stdlib/Effect.Concurrent.MVar/source/). Source SHA-256: `1367ee0fbdb4b2ca7e6bb42c760182a437f1ad3b2f4f83d83cadfae83719f52e`.
