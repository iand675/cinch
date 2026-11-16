# Data.Mutable.HeapView

In-place heap ordering and indexed repairs through explicit borrowed storage callbacks.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.HeapView`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.HeapView.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md).

Non-embedded source closure: [Data.Mutable.HeapView](Data.Mutable.HeapView.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Binary heap algorithms over caller-owned mutable indexed storage.
The callbacks preserve application-specific element identities and index
bookkeeping. They must describe one coherent sequence: comparison and swap
keep its length unchanged, append adds exactly one element, and remove-last
removes exactly its last element. Comparisons form a strict weak ordering.
Comparisons and swaps are sequenced; arbitrary callbacks cannot be reordered
or evaluated in SIMD lanes. Use Data.Mutable.Heap for dense owned storage.

</details>

# Types and constructors

<a id="type-4865617056696577"></a>

## `HeapView`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HeapView</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

An opaque set of mutable length, comparison, swap, append and remove-last callbacks.

[Source](/reference-sources/Data.Mutable.HeapView.cinch) · line 18

# Values

<a id="value-686561702d76696577"></a>

## `heap-view`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-view</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)) -&gt; ((<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)) -&gt; ((<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>) -&gt; ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HeapView</span>.<span class="tok-constructor">HeapView</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)))))</code></pre>

Bind coherent indexed-storage callbacks without allocating or changing their sequence.
The last callback returns None only for an empty sequence. Callback contract
violations are application errors; this module never accesses raw storage.

[Source](/reference-sources/Data.Mutable.HeapView.cinch) · line 25

<a id="value-686561702d766965772d666978"></a>

## `heap-view-fix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-view-fix</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HeapView</span>.<span class="tok-constructor">HeapView</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Repair a changed element in logarithmic work without removing or reappending it.
False means the index was absent; no comparison or swap occurs in that case.

[Source](/reference-sources/Data.Mutable.HeapView.cinch) · line 91

<a id="value-686561702d766965772d696e6974"></a>

## `heap-view-init`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-view-init</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HeapView</span>.<span class="tok-constructor">HeapView</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

Restore every subtree bottom-up in linear comparison work.

[Source](/reference-sources/Data.Mutable.HeapView.cinch) · line 49

<a id="value-686561702d766965772d6c656e677468"></a>

## `heap-view-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-view-length</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HeapView</span>.<span class="tok-constructor">HeapView</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

Observe the current sequence length through its callback.

[Source](/reference-sources/Data.Mutable.HeapView.cinch) · line 29

<a id="value-686561702d766965772d706f70"></a>

## `heap-view-pop`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-view-pop</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HeapView</span>.<span class="tok-constructor">HeapView</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

Remove the first element, or return None without invoking swap/remove callbacks when empty.

[Source](/reference-sources/Data.Mutable.HeapView.cinch) · line 67

<a id="value-686561702d766965772d70757368"></a>

## `heap-view-push`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-view-push</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HeapView</span>.<span class="tok-constructor">HeapView</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Append an element and repair its path toward the root in logarithmic work.

[Source](/reference-sources/Data.Mutable.HeapView.cinch) · line 60

<a id="value-686561702d766965772d72656d6f7665"></a>

## `heap-view-remove`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-view-remove</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HeapView</span>.<span class="tok-constructor">HeapView</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

Remove any existing position and repair the one displaced element.
An out-of-range position returns None and invokes no mutation callback.

[Source](/reference-sources/Data.Mutable.HeapView.cinch) · line 78

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Mutable/HeapView.cinch`. Source SHA-256: `89e735c9db876832b79fc1a321be54d13661ecdfb4a662be6653780afd9e4a8c`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
