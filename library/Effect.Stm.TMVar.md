# Effect.Stm.TMVar

A transactional variable that is either empty or full: take waits while empty, put waits while full. The building block for handoff and mutual exclusion inside transactions.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Effect.Stm.TMVar`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Stm.TMVar.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Effect.Stm](Effect.Stm.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-544d566172"></a>

## `TMVar`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TMVar</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

[Source](/reference-sources/Effect.Stm.TMVar.cinch)

# Values

<a id="value-746d7661722d69732d656d707479"></a>

## `tmvar-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tmvar-is-empty</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TMVar</span>.<span class="tok-constructor">TMVar</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Effect.Stm.TMVar.cinch)

<a id="value-746d7661722d6e6577"></a>

## `tmvar-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tmvar-new</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TMVar</span>.<span class="tok-constructor">TMVar</span> <span class="tok-variable">a</span>)))</code></pre>

A full variable holding `x`.

[Source](/reference-sources/Effect.Stm.TMVar.cinch) · line 32

<a id="value-746d7661722d6e65772d656d707479"></a>

## `tmvar-new-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tmvar-new-empty</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TMVar</span>.<span class="tok-constructor">TMVar</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Effect.Stm.TMVar.cinch)

<a id="value-746d7661722d6e65772d656d7074792d696f"></a>

## `tmvar-new-empty-io`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tmvar-new-empty-io</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">STM</span>} <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TMVar</span>.<span class="tok-constructor">TMVar</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Effect.Stm.TMVar.cinch)

<a id="value-746d7661722d6e65772d696f"></a>

## `tmvar-new-io`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tmvar-new-io</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">STM</span>} <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TMVar</span>.<span class="tok-constructor">TMVar</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Effect.Stm.TMVar.cinch)

<a id="value-746d7661722d707574"></a>

## `tmvar-put`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tmvar-put</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TMVar</span>.<span class="tok-constructor">TMVar</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Fill the variable. Waits while it is full.

[Source](/reference-sources/Effect.Stm.TMVar.cinch) · line 51

<a id="value-746d7661722d72656164"></a>

## `tmvar-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tmvar-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TMVar</span>.<span class="tok-constructor">TMVar</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">a</span>))</code></pre>

The value, leaving it in place. Waits while the variable is empty.

[Source](/reference-sources/Effect.Stm.TMVar.cinch) · line 57

<a id="value-746d7661722d73776170"></a>

## `tmvar-swap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tmvar-swap</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TMVar</span>.<span class="tok-constructor">TMVar</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">a</span>)))</code></pre>

Replace the value and answer the old one. Waits while empty.

[Source](/reference-sources/Effect.Stm.TMVar.cinch) · line 63

<a id="value-746d7661722d74616b65"></a>

## `tmvar-take`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tmvar-take</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TMVar</span>.<span class="tok-constructor">TMVar</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-variable">a</span>))</code></pre>

The value, leaving the variable empty. Waits while it is empty.

[Source](/reference-sources/Effect.Stm.TMVar.cinch) · line 45

<a id="value-746d7661722d7472792d707574"></a>

## `tmvar-try-put`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tmvar-try-put</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TMVar</span>.<span class="tok-constructor">TMVar</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> <span class="tok-constructor">Bool</span>)))</code></pre>

`tmvar-put` that answers whether it filled the variable instead of
waiting.

[Source](/reference-sources/Effect.Stm.TMVar.cinch) · line 76

<a id="value-746d7661722d7472792d72656164"></a>

## `tmvar-try-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tmvar-try-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TMVar</span>.<span class="tok-constructor">TMVar</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)))</code></pre>

[Source](/reference-sources/Effect.Stm.TMVar.cinch)

<a id="value-746d7661722d7472792d74616b65"></a>

## `tmvar-try-take`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tmvar-try-take</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">TMVar</span>.<span class="tok-constructor">TMVar</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stm</span>.<span class="tok-constructor">STM</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)))</code></pre>

`tmvar-take` that answers `None` instead of waiting.

[Source](/reference-sources/Effect.Stm.TMVar.cinch) · line 69

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Stm/TMVar.cinch`. Source SHA-256: `6b18623aeea5cc48d7e0b68c901b7242d38f9ca28f142d9a8f609a6771f9fba4`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
