# Data.Mutable.Vec

A growable vector: amortised O(1) push and pop, O(1) indexing, middle insertion and removal, in-place introsort, and copying freeze/thaw against Array.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.Vec`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.Vec.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Mutable.Vec](Data.Mutable.Vec.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Mutable.Vec: the growable vector (C++ `std::vector`, Rust `Vec`).

A `Vec r a` is a dense sequence in one contiguous slot array: elements
`0 .. length - 1` are written, the capacity beyond them is empty, and
growth doubles the capacity (floor 4), so `n` pushes move fewer than
`2n` elements and allocate `O(log n)` times. A reallocation MOVES the
elements (one word copy each, no reference count touched), which is
why doubling, not folly's 1.5x, is the right factor here: the factor
trades copy work against slack, and the copy is already the cheap kind.

Costs: `vec-push` and `vec-pop` are amortised O(1); `vec-get`,
`vec-set`, `vec-length` and `vec-swap` are O(1); `vec-insert` and
`vec-remove` shift the tail (O(n - i), one memmove); `vec-swap-remove`
is O(1) and does not keep order; `vec-sort` is introsort (median-of-
three quicksort, insertion sort under 16 elements, heapsort past a
2 log2 n depth), O(n log n) worst case and in place.

The invariant that makes every read sound: a slot below the length is
written, a slot at or above it is empty, and only slots below the
length are ever read. Every operation re-reads the current storage, so
a callback that pushes during `vec-for-each` (which may reallocate)
cannot hand a moved-out slot back.

Named refusals:
  aliasing views: there is no borrowed slice of a vector; `vec-freeze`
    copies into an immutable `Array a` (O(n)) and `vec-from-array`
    copies back, so an immutable value never observes a later write;
  iteration under mutation: `vec-for-each` visits indices 0, 1, ...
    while they are below the CURRENT length, so a callback that
    removes or inserts shifts what it visits (memory-safe, but the
    visit set is the callback's business).

The trusted core is the private `cinch.runtime.slots*` imports (the
compiler's `slots.rs`); none of them leaves the module.

</details>

# Types and constructors

<a id="type-566563"></a>

## `Vec`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Vec</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The header: one raw word (the length) and one cell holding the
current storage, replaced when the vector reallocates.

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 64

# Values

<a id="value-7665632d6361706163697479"></a>

## `vec-capacity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-capacity</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Vec.cinch)

<a id="value-7665632d636c656172"></a>

## `vec-clear`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-clear</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

Drop every element; the capacity stays.

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 286

<a id="value-7665632d666f6c64"></a>

## `vec-fold`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-fold</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>)) -&gt; (<span class="tok-variable">b</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">b</span>)))</code></pre>

A left fold in index order over the elements present when it
starts. The callback is pure, so it cannot change the vector.

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 337

<a id="value-7665632d666f722d65616368"></a>

## `vec-for-each`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-for-each</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; ((<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Run an action on each element in index order. The storage and the
length are re-read at every step, so an action that changes this
vector is memory-safe (it decides what is visited after it).

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 349

<a id="value-7665632d667265657a65"></a>

## `vec-freeze`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-freeze</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Array</span> <span class="tok-variable">a</span>)</code></pre>

An immutable copy of the elements. The copy is a fresh block with
every slot written, so it is an ordinary `Array a` and no later
write to the vector reaches it.

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 397

<a id="value-7665632d66726f6d2d6172726179"></a>

## `vec-from-array`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-from-array</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Vec.cinch)

<a id="value-7665632d66726f6d2d6c697374"></a>

## `vec-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-from-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

Lists and folds

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 309

<a id="value-7665632d676574"></a>

## `vec-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-get</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

Indexing

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 190

<a id="value-7665632d696e73657274"></a>

## `vec-insert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-insert</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

Insert at `i`, shifting `i ..` up by one: `False` (and nothing
inserted) when `i` exceeds the length (`i == length` appends).

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 244

<a id="value-7665632d69732d656d707479"></a>

## `vec-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-is-empty</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Vec.cinch)

<a id="value-7665632d6c617374"></a>

## `vec-last`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-last</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Vec.cinch)

<a id="value-7665632d6c656e677468"></a>

## `vec-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-length</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Vec.cinch)

<a id="value-7665632d6d61702d696e2d706c616365"></a>

## `vec-map-in-place`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-map-in-place</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Rewrite every element through `f`, moving each out and back.

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 362

<a id="value-7665632d6d6f64696679"></a>

## `vec-modify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-modify</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

Rewrite element `i` through `f`, moving it out and back so a unique
element is updated without a copy. `False` when out of range.

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 209

<a id="value-7665632d6e6577"></a>

## `vec-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

Construction and size

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 125

<a id="value-7665632d706f70"></a>

## `vec-pop`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-pop</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Vec.cinch)

<a id="value-7665632d70757368"></a>

## `vec-push`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-push</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

The ends

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 160

<a id="value-7665632d707573682d616c6c"></a>

## `vec-push-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-push-all</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.Vec.cinch)

<a id="value-7665632d72656d6f7665"></a>

## `vec-remove`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-remove</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

Remove element `i`, shifting the tail down: the element, or `None`
when out of range.

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 258

<a id="value-7665632d72657365727665"></a>

## `vec-reserve`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-reserve</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Make room for `extra` more elements without a further reallocation.

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 151

<a id="value-7665632d72657665727365"></a>

## `vec-reverse`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-reverse</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Vec.cinch)

<a id="value-7665632d736574"></a>

## `vec-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-set</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

Replace element `i`: `False` (and nothing written) when `i` is not
below the length.

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 199

<a id="value-7665632d736f7274"></a>

## `vec-sort`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-sort</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

Sorting: introsort over [lo, hi)

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 425

<a id="value-7665632d736f72742d6279"></a>

## `vec-sort-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-sort-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.Vec.cinch)

<a id="value-7665632d73776170"></a>

## `vec-swap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-swap</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

Exchange elements `i` and `j`: `False` when either is out of range.

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 220

<a id="value-7665632d737761702d72656d6f7665"></a>

## `vec-swap-remove`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-swap-remove</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

Remove element `i` by moving the last element into its place: O(1),
and the order of the rest is not kept.

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 272

<a id="value-7665632d746f2d6c697374"></a>

## `vec-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-to-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)</code></pre>

The elements in index order (built from the back, one cons each).

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 323

<a id="value-7665632d7472756e63617465"></a>

## `vec-truncate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-truncate</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Keep the first `k` elements (all of them when `k` is at least the
length).

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 295

<a id="value-7665632d776974682d6361706163697479"></a>

## `vec-with-capacity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-with-capacity</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Vec</span>.<span class="tok-constructor">Vec</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

An empty vector with room for `n` elements before its first
reallocation.

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 130

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Mutable/Vec.cinch`. Source SHA-256: `61fff082140825bba9490ae212de6dcfec740a684d0cc336adc6058c9b89dde7`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
