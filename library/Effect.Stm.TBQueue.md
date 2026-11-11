# Effect.Stm.TBQueue

A bounded first-in first-out queue for transactions: a write waits while the queue is at capacity, a read waits while it is empty.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Effect.Stm.TBQueue`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Stm.TBQueue.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.List](Data.List.md), [Effect.Stm](Effect.Stm.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-54425175657565"></a>

## `TBQueue`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TBQueue</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The capacity, the number of elements held, the front (next out
first) and the back (last in first).

[Source](/reference-sources/Effect.Stm.TBQueue.cinch) · line 33

# Values

<a id="value-746271756575652d6361706163697479"></a>

## `tbqueue-capacity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tbqueue-capacity</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TBQueue</span>.<span class="tok-constructor">TBQueue</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Effect.Stm.TBQueue.cinch)

<a id="value-746271756575652d69732d656d707479"></a>

## `tbqueue-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tbqueue-is-empty</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TBQueue</span>.<span class="tok-constructor">TBQueue</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Effect.Stm.TBQueue.cinch)

<a id="value-746271756575652d69732d66756c6c"></a>

## `tbqueue-is-full`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tbqueue-is-full</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TBQueue</span>.<span class="tok-constructor">TBQueue</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Effect.Stm.TBQueue.cinch)

<a id="value-746271756575652d6c656e677468"></a>

## `tbqueue-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tbqueue-length</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TBQueue</span>.<span class="tok-constructor">TBQueue</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Nat</span>))</code></pre>

[Source](/reference-sources/Effect.Stm.TBQueue.cinch)

<a id="value-746271756575652d6e6577"></a>

## `tbqueue-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tbqueue-new</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TBQueue</span>.<span class="tok-constructor">TBQueue</span> <span class="tok-variable">a</span>)))</code></pre>

A queue that holds at most `capacity` elements. A capacity of zero
makes a queue no write can ever enter.

[Source](/reference-sources/Effect.Stm.TBQueue.cinch) · line 38

<a id="value-746271756575652d6e65772d696f"></a>

## `tbqueue-new-io`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tbqueue-new-io</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">STM</span>} <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TBQueue</span>.<span class="tok-constructor">TBQueue</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Effect.Stm.TBQueue.cinch)

<a id="value-746271756575652d72656164"></a>

## `tbqueue-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tbqueue-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TBQueue</span>.<span class="tok-constructor">TBQueue</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">a</span>))</code></pre>

The oldest element, removed. Waits while the queue is empty.

[Source](/reference-sources/Effect.Stm.TBQueue.cinch) · line 74

<a id="value-746271756575652d7472792d72656164"></a>

## `tbqueue-try-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tbqueue-try-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TBQueue</span>.<span class="tok-constructor">TBQueue</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)))</code></pre>

The oldest element, removed, or `None` when the queue is empty.

[Source](/reference-sources/Effect.Stm.TBQueue.cinch) · line 62

<a id="value-746271756575652d7472792d7772697465"></a>

## `tbqueue-try-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tbqueue-try-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TBQueue</span>.<span class="tok-constructor">TBQueue</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Bool</span>)))</code></pre>

Append `x`, or answer False when the queue is full.

[Source](/reference-sources/Effect.Stm.TBQueue.cinch) · line 51

<a id="value-746271756575652d7772697465"></a>

## `tbqueue-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tbqueue-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TBQueue</span>.<span class="tok-constructor">TBQueue</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Append `x`. Waits while the queue is full.

[Source](/reference-sources/Effect.Stm.TBQueue.cinch) · line 58

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Stm/TBQueue.cinch`. Source SHA-256: `07e856790c35d526f998a0e3befd2bd31dc39b2b7e3354c64d75554e44ed7dfe`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
