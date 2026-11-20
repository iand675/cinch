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

A shared single-place variable whose operations preserve stored-value ownership. Constructors are private; task sharing requires shareable contents.

[Source](/reference-sources/Effect.Concurrent.MVar.cinch) · line 35

# Values

<a id="value-6d7661722d69732d656d707479"></a>

## `mvar-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-is-empty</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">MVar</span>} <span class="tok-constructor">Bool</span>)</code></pre>

Return an instantaneous empty-state observation. The result does not reserve space or a value.

[Source](/reference-sources/Effect.Concurrent.MVar.cinch) · line 105

<a id="value-6d7661722d6e6577"></a>

## `mvar-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-new</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">MVar</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span>))</code></pre>

Create a full shared variable containing the supplied value.

[Source](/reference-sources/Effect.Concurrent.MVar.cinch) · line 69

<a id="value-6d7661722d6e65772d656d707479"></a>

## `mvar-new-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-new-empty</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">MVar</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span>))</code></pre>

Create an empty shared variable; a take or read waits until a value arrives.

[Source](/reference-sources/Effect.Concurrent.MVar.cinch) · line 73

<a id="value-6d7661722d707574"></a>

## `mvar-put`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-put</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">MVar</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Wait until space is available and store the supplied value. Registered putters are served in FIFO order.

[Source](/reference-sources/Effect.Concurrent.MVar.cinch) · line 81

<a id="value-6d7661722d7075742d6f722d63616e63656c"></a>

## `mvar-put-or-cancel`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-put-or-cancel</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Cancel</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Diverge</span>, <span class="tok-constructor">MVar</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Store the value and return True, or return False if cancellation wins before handoff. A committed put succeeds.

[Source](/reference-sources/Effect.Concurrent.MVar.cinch) · line 121

<a id="value-6d7661722d72656164"></a>

## `mvar-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">MVar</span>} <span class="tok-variable">a</span>)</code></pre>

Wait until a value is available and return it without emptying the variable.

[Source](/reference-sources/Effect.Concurrent.MVar.cinch) · line 85

<a id="value-6d7661722d726561642d6f722d63616e63656c"></a>

## `mvar-read-or-cancel`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-read-or-cancel</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">Cancel</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Diverge</span>, <span class="tok-constructor">MVar</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

Read a value without removal, or return None if cancellation wins before delivery. Immediate reads succeed despite prior cancellation.

[Source](/reference-sources/Effect.Concurrent.MVar.cinch) · line 113

<a id="value-6d7661722d73776170"></a>

## `mvar-swap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-swap</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">MVar</span>} <span class="tok-variable">a</span>))</code></pre>

Wait until a value is available, replace it atomically, and return the previous value.

[Source](/reference-sources/Effect.Concurrent.MVar.cinch) · line 89

<a id="value-6d7661722d737761702d6f722d63616e63656c"></a>

## `mvar-swap-or-cancel`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-swap-or-cancel</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Cancel</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Diverge</span>, <span class="tok-constructor">MVar</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)))</code></pre>

Exchange values or return None if cancellation wins before handoff. A committed exchange returns the previous value.

[Source](/reference-sources/Effect.Concurrent.MVar.cinch) · line 117

<a id="value-6d7661722d74616b65"></a>

## `mvar-take`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-take</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">MVar</span>} <span class="tok-variable">a</span>)</code></pre>

Wait until a value is available, remove it, and return it. Registered takers are served in FIFO order.

[Source](/reference-sources/Effect.Concurrent.MVar.cinch) · line 77

<a id="value-6d7661722d74616b652d6f722d63616e63656c"></a>

## `mvar-take-or-cancel`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-take-or-cancel</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">Cancel</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Diverge</span>, <span class="tok-constructor">MVar</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

Take a value or return None if cancellation wins before handoff. An already committed handoff succeeds.

[Source](/reference-sources/Effect.Concurrent.MVar.cinch) · line 109

<a id="value-6d7661722d7472792d707574"></a>

## `mvar-try-put`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-try-put</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">MVar</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Store the value and return True when space is immediately available; otherwise return False without waiting.

[Source](/reference-sources/Effect.Concurrent.MVar.cinch) · line 97

<a id="value-6d7661722d7472792d72656164"></a>

## `mvar-try-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-try-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">MVar</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

Return an immediately available value without removing it, or return None without waiting.

[Source](/reference-sources/Effect.Concurrent.MVar.cinch) · line 101

<a id="value-6d7661722d7472792d74616b65"></a>

## `mvar-try-take`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mvar-try-take</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">MVar</span>.<span class="tok-constructor">MVar</span> <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">MVar</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

Remove and return an immediately available value, or return None without waiting.

[Source](/reference-sources/Effect.Concurrent.MVar.cinch) · line 93

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Concurrent/MVar.cinch`. Source SHA-256: `1367ee0fbdb4b2ca7e6bb42c760182a437f1ad3b2f4f83d83cadfae83719f52e`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
