# Effect.Stm.TQueue

An unbounded first-in first-out queue for transactions, as two lists so that writers and readers touch different variables.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Effect.Stm.TQueue`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Stm.TQueue.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.List](Data.List.md), [Effect.Stm](Effect.Stm.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-545175657565"></a>

## `TQueue`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TQueue</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The front (next out first) and the back (last in first).

[Source](/reference-sources/Effect.Stm.TQueue.cinch) · line 27

# Values

<a id="value-7471756575652d666c757368"></a>

## `tqueue-flush`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tqueue-flush</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TQueue</span>.<span class="tok-constructor">TQueue</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

Every element, oldest first, leaving the queue empty.

[Source](/reference-sources/Effect.Stm.TQueue.cinch) · line 63

<a id="value-7471756575652d69732d656d707479"></a>

## `tqueue-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tqueue-is-empty</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TQueue</span>.<span class="tok-constructor">TQueue</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Effect.Stm.TQueue.cinch)

<a id="value-7471756575652d6e6577"></a>

## `tqueue-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tqueue-new</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TQueue</span>.<span class="tok-constructor">TQueue</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Effect.Stm.TQueue.cinch)

<a id="value-7471756575652d6e65772d696f"></a>

## `tqueue-new-io`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tqueue-new-io</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">STM</span>} <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TQueue</span>.<span class="tok-constructor">TQueue</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Effect.Stm.TQueue.cinch)

<a id="value-7471756575652d7065656b"></a>

## `tqueue-peek`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tqueue-peek</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TQueue</span>.<span class="tok-constructor">TQueue</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">a</span>))</code></pre>

The oldest element, left in place. Waits while the queue is empty.

[Source](/reference-sources/Effect.Stm.TQueue.cinch) · line 55

<a id="value-7471756575652d72656164"></a>

## `tqueue-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tqueue-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TQueue</span>.<span class="tok-constructor">TQueue</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">a</span>))</code></pre>

The oldest element, removed. Waits while the queue is empty.

[Source](/reference-sources/Effect.Stm.TQueue.cinch) · line 49

<a id="value-7471756575652d7472792d72656164"></a>

## `tqueue-try-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tqueue-try-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TQueue</span>.<span class="tok-constructor">TQueue</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)))</code></pre>

The oldest element, removed, or `None` when the queue is empty.

[Source](/reference-sources/Effect.Stm.TQueue.cinch) · line 41

<a id="value-7471756575652d756e676574"></a>

## `tqueue-unget`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tqueue-unget</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TQueue</span>.<span class="tok-constructor">TQueue</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Put `x` back at the front: the next read answers it.

[Source](/reference-sources/Effect.Stm.TQueue.cinch) · line 59

<a id="value-7471756575652d7772697465"></a>

## `tqueue-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tqueue-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TQueue</span>.<span class="tok-constructor">TQueue</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Append `x`. Never waits.

[Source](/reference-sources/Effect.Stm.TQueue.cinch) · line 37

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Stm/TQueue.cinch`. Source SHA-256: `706dd386f8128e43835dc680ac787a5f8af5d02625735ae59458f916c56f4eb2`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
