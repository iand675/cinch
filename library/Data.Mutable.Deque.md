# Data.Mutable.Deque

A double-ended queue as a power-of-two ring buffer: amortised O(1) push and pop at both ends and O(1) indexing.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.Deque`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.Deque.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md).

Non-embedded source closure: [Data.Mutable.Deque](Data.Mutable.Deque.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Mutable.Deque: the double-ended queue (Rust `VecDeque`, a ring
buffer rather than C++'s segmented `std::deque`).

The elements live in one slot array used as a ring: `head` is the
physical slot of element 0, and element `i` is at `(head + i) & (cap -
1)`. The capacity is a power of two (0, then 4, then doubling), so the
wrap is a mask. Growth moves the two halves of the ring into fresh
storage in order (two word moves, no reference count touched) and
resets `head` to 0.

Costs: push and pop at either end are amortised O(1); `deque-get` and
`deque-set` are O(1); `deque-to-list` and the folds are O(n).

The invariant that makes every read sound: exactly the `len` ring
positions from `head` are written, and only those are read.

Named refusals:
  middle insertion and removal: a ring makes them O(n) with two-sided
    shifting, so they are left to `Data.Mutable.Vec`;
  iteration under mutation: `deque-for-each` visits indices while they
    are below the CURRENT length and re-reads the storage each step.

The trusted core is the private `cinch.runtime.slots*` imports (the
compiler's `slots.rs`); none of them leaves the module.

</details>

# Types and constructors

<a id="type-4465717565"></a>

## `Deque`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Deque</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The header: `head` at word 0 and `len` at word 1, and one cell
holding the current ring.

[Source](/reference-sources/Data.Mutable.Deque.cinch) · line 46

# Values

<a id="value-64657175652d6261636b"></a>

## `deque-back`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deque-back</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Deque</span>.<span class="tok-constructor">Deque</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Deque.cinch)

<a id="value-64657175652d6361706163697479"></a>

## `deque-capacity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deque-capacity</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Deque</span>.<span class="tok-constructor">Deque</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Deque.cinch)

<a id="value-64657175652d636c656172"></a>

## `deque-clear`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deque-clear</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Deque</span>.<span class="tok-constructor">Deque</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

Drop every element; the capacity stays.

[Source](/reference-sources/Data.Mutable.Deque.cinch) · line 210

<a id="value-64657175652d666f6c64"></a>

## `deque-fold`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deque-fold</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>)) -&gt; (<span class="tok-variable">b</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Deque</span>.<span class="tok-constructor">Deque</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">b</span>)))</code></pre>

A left fold front to back through a pure function.

[Source](/reference-sources/Data.Mutable.Deque.cinch) · line 251

<a id="value-64657175652d666f722d65616368"></a>

## `deque-for-each`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deque-for-each</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Deque</span>.<span class="tok-constructor">Deque</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; ((<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Run an action front to back; the ring, head and length are re-read
at every step.

[Source](/reference-sources/Data.Mutable.Deque.cinch) · line 262

<a id="value-64657175652d66726f6d2d6c697374"></a>

## `deque-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deque-from-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Deque</span>.<span class="tok-constructor">Deque</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Deque.cinch)

<a id="value-64657175652d66726f6e74"></a>

## `deque-front`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deque-front</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Deque</span>.<span class="tok-constructor">Deque</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Deque.cinch)

<a id="value-64657175652d676574"></a>

## `deque-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deque-get</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Deque</span>.<span class="tok-constructor">Deque</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.Deque.cinch)

<a id="value-64657175652d69732d656d707479"></a>

## `deque-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deque-is-empty</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Deque</span>.<span class="tok-constructor">Deque</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Deque.cinch)

<a id="value-64657175652d6c656e677468"></a>

## `deque-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deque-length</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Deque</span>.<span class="tok-constructor">Deque</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Deque.cinch)

<a id="value-64657175652d6e6577"></a>

## `deque-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deque-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Deque</span>.<span class="tok-constructor">Deque</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Deque.cinch)

<a id="value-64657175652d706f702d6261636b"></a>

## `deque-pop-back`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deque-pop-back</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Deque</span>.<span class="tok-constructor">Deque</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Deque.cinch)

<a id="value-64657175652d706f702d66726f6e74"></a>

## `deque-pop-front`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deque-pop-front</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Deque</span>.<span class="tok-constructor">Deque</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Deque.cinch)

<a id="value-64657175652d707573682d6261636b"></a>

## `deque-push-back`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deque-push-back</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Deque</span>.<span class="tok-constructor">Deque</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.Deque.cinch)

<a id="value-64657175652d707573682d66726f6e74"></a>

## `deque-push-front`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deque-push-front</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Deque</span>.<span class="tok-constructor">Deque</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.Deque.cinch)

<a id="value-64657175652d736574"></a>

## `deque-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deque-set</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Deque</span>.<span class="tok-constructor">Deque</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

Replace element `i`: `False` (and nothing written) when out of range.

[Source](/reference-sources/Data.Mutable.Deque.cinch) · line 201

<a id="value-64657175652d746f2d6c697374"></a>

## `deque-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deque-to-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Deque</span>.<span class="tok-constructor">Deque</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)</code></pre>

Front to back.

[Source](/reference-sources/Data.Mutable.Deque.cinch) · line 238

<a id="value-64657175652d776974682d6361706163697479"></a>

## `deque-with-capacity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deque-with-capacity</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Deque</span>.<span class="tok-constructor">Deque</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

An empty deque with room for `n` elements (rounded up to a power of
two) before its first growth.

[Source](/reference-sources/Data.Mutable.Deque.cinch) · line 121

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Mutable/Deque.cinch`. Source SHA-256: `3761e4888415de3d3fa427aefef583cfcb7cfb4f481759ed906b7c3a841eaada`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
