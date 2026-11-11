# Effect.Stm.TSem

A counting semaphore for transactions: wait takes a permit or retries, signal returns one.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Effect.Stm.TSem`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Stm.TSem.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Effect.Stm](Effect.Stm.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-5453656d"></a>

## `TSem`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TSem</span></code></pre>

[Source](/reference-sources/Effect.Stm.TSem.cinch)

# Values

<a id="value-7473656d2d617661696c61626c65"></a>

## `tsem-available`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tsem-available</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TSem</span>.<span class="tok-constructor">TSem</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Nat</span>))</code></pre>

The permits available at the transaction's instant.

[Source](/reference-sources/Effect.Stm.TSem.cinch) · line 47

<a id="value-7473656d2d6e6577"></a>

## `tsem-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tsem-new</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TSem</span>.<span class="tok-constructor">TSem</span>))</code></pre>

A semaphore with `permits` permits.

[Source](/reference-sources/Effect.Stm.TSem.cinch) · line 23

<a id="value-7473656d2d6e65772d696f"></a>

## `tsem-new-io`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tsem-new-io</span> :: (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">STM</span>} <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TSem</span>.<span class="tok-constructor">TSem</span>)</code></pre>

[Source](/reference-sources/Effect.Stm.TSem.cinch)

<a id="value-7473656d2d7369676e616c"></a>

## `tsem-signal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tsem-signal</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TSem</span>.<span class="tok-constructor">TSem</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Unit</span>))</code></pre>

Return a permit.

[Source](/reference-sources/Effect.Stm.TSem.cinch) · line 39

<a id="value-7473656d2d7369676e616c2d6e"></a>

## `tsem-signal-n`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tsem-signal-n</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TSem</span>.<span class="tok-constructor">TSem</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Return `k` permits at once.

[Source](/reference-sources/Effect.Stm.TSem.cinch) · line 43

<a id="value-7473656d2d7472792d77616974"></a>

## `tsem-try-wait`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tsem-try-wait</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TSem</span>.<span class="tok-constructor">TSem</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Bool</span>))</code></pre>

Take a permit, or answer False when there is none.

[Source](/reference-sources/Effect.Stm.TSem.cinch) · line 30

<a id="value-7473656d2d77616974"></a>

## `tsem-wait`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tsem-wait</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TSem</span>.<span class="tok-constructor">TSem</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Unit</span>))</code></pre>

Take a permit. Waits while there is none.

[Source](/reference-sources/Effect.Stm.TSem.cinch) · line 35

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Stm/TSem.cinch`. Source SHA-256: `a493de2f8f16e4be3fa641174b9d1c669597641fd64857cde9fa5f8d80c4947d`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
