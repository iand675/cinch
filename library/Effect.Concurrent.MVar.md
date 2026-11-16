# Effect.Concurrent.MVar

Native single-place shared variables with FIFO handoff, blocking take/put/read/swap, nonblocking operations, and cancellable waits through the task scheduler.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Effect.Concurrent.MVar`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Concurrent.MVar.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Option](Data.Option.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-4d566172"></a>

## `MVar`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MVar</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

[Source](/reference-sources/Effect.Concurrent.MVar.cinch)

# Values

<a id="value-6d7661722d69732d656d707479"></a>

## `mvar-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-is-empty</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">MVar</span>} <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Effect.Concurrent.MVar.cinch)

<a id="value-6d7661722d6e6577"></a>

## `mvar-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">MVar</span>} <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Effect.Concurrent.MVar.cinch)

<a id="value-6d7661722d6e65772d656d707479"></a>

## `mvar-new-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-new-empty</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">MVar</span>} <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Effect.Concurrent.MVar.cinch)

<a id="value-6d7661722d707574"></a>

## `mvar-put`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-put</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">MVar</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Unit</span>))</code></pre>

[Source](/reference-sources/Effect.Concurrent.MVar.cinch)

<a id="value-6d7661722d7075742d6f722d63616e63656c"></a>

## `mvar-put-or-cancel`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-put-or-cancel</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">MVar</span>, <span class="tok-constructor">Cancel</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Effect.Concurrent.MVar.cinch)

<a id="value-6d7661722d72656164"></a>

## `mvar-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-read</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">MVar</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Effect.Concurrent.MVar.cinch)

<a id="value-6d7661722d726561642d6f722d63616e63656c"></a>

## `mvar-read-or-cancel`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-read-or-cancel</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">MVar</span>, <span class="tok-constructor">Cancel</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Effect.Concurrent.MVar.cinch)

<a id="value-6d7661722d73776170"></a>

## `mvar-swap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-swap</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">MVar</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Effect.Concurrent.MVar.cinch)

<a id="value-6d7661722d737761702d6f722d63616e63656c"></a>

## `mvar-swap-or-cancel`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-swap-or-cancel</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">MVar</span>, <span class="tok-constructor">Cancel</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Effect.Concurrent.MVar.cinch)

<a id="value-6d7661722d74616b65"></a>

## `mvar-take`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-take</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">MVar</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Effect.Concurrent.MVar.cinch)

<a id="value-6d7661722d74616b652d6f722d63616e63656c"></a>

## `mvar-take-or-cancel`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-take-or-cancel</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">MVar</span>, <span class="tok-constructor">Cancel</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Diverge</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Effect.Concurrent.MVar.cinch)

<a id="value-6d7661722d7472792d707574"></a>

## `mvar-try-put`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-try-put</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">MVar</span>} <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Effect.Concurrent.MVar.cinch)

<a id="value-6d7661722d7472792d72656164"></a>

## `mvar-try-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-try-read</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">MVar</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Effect.Concurrent.MVar.cinch)

<a id="value-6d7661722d7472792d74616b65"></a>

## `mvar-try-take`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-try-take</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">MVar</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Effect.Concurrent.MVar.cinch)

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Concurrent/MVar.cinch`. Source SHA-256: `ae0fdb5a7b2b398681d7598c2a606e536c739e98e4a8188a1169092b49f68de6`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
