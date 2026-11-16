# Data.Mutable.Heap

A 4-ary priority queue with its ordering fixed at construction: O(log n) push and pop, O(1) peek, and an O(n) Floyd build from a list.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.Heap`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.Heap.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Mutable.Heap](Data.Mutable.Heap.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Mutable.Heap: the priority queue (C++ `std::priority_queue`, as a
4-ary heap like Boost's `d_ary_heap`).

A `Heap r a` keeps its elements in one dense slot array arranged as a
4-ary heap under an ordering fixed when it is made: the least element
is at slot 0, and the children of slot i are 4i + 1 .. 4i + 4. Four
children per node halve the depth of a binary heap, so a pop does
fewer levels of sifting and touches one cache line per level for word
elements; the extra comparisons per level are cheap next to the
misses they save.

The ordering is supplied at each operation that compares, as C++'s
`std::push_heap`/`std::pop_heap` take their comparator: `heap-push` and
`heap-pop` order by the `Ord` instance (a min-heap), and the `-by` forms
take any strict "comes first" relation, so a max-heap is
`heap-push-by (\x y -> y < x)` with the matching `heap-pop-by`. Passing
the ordering per call rather than storing it in the heap is what lets the
compiler specialise it: a comparator stored in a value is a closure call
on every comparison, one passed as a static argument is cloned into the
sift loops and inlined (design/mutable-containers.md section 6). The
contract that comes with it: every operation on one heap uses the same
ordering, or the heap order is the mixture's (memory-safe, but not an
order).

Costs: `heap-push` and `heap-pop` are O(log n) (amortised for the
storage growth); `heap-peek` and `heap-length` are O(1);
`heap-from-list` is Floyd's O(n) bottom-up build.

Sifting moves the element into a hole rather than swapping, so each
level costs one move and no reference count operation, and a pop sinks
its hole to a leaf before placing the last element (libstdc++'s
bottom-up `__adjust_heap`), which saves a comparison per level.

The invariant that makes every read sound: slots below the length are
written, and only those are read.

The trusted core is the private `cinch.runtime.slots*` imports (the
compiler's `slots.rs`); none of them leaves the module.

</details>

# Types and constructors

<a id="type-48656170"></a>

## `Heap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Heap</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The length word and the cell holding the storage.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 60

# Values

<a id="value-686561702d636c656172"></a>

## `heap-clear`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-clear</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

Drop every element; the capacity stays.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 213

<a id="value-686561702d647261696e"></a>

## `heap-drain`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-drain</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)</code></pre>

Pop everything in `Ord` order, leaving the heap empty.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 271

<a id="value-686561702d647261696e2d6279"></a>

## `heap-drain-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-drain-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))</code></pre>

Pop everything in `first`'s order, leaving the heap empty.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 275

<a id="value-686561702d66726f6d2d6c697374"></a>

## `heap-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-from-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

Lists

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 223

<a id="value-686561702d66726f6d2d6c6973742d6279"></a>

## `heap-from-list-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-from-list-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))</code></pre>

Floyd's build: write the list in, then sift every internal node
down from the last one, O(n) in all.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 228

<a id="value-686561702d69732d656d707479"></a>

## `heap-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-is-empty</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Heap.cinch)

<a id="value-686561702d6c656e677468"></a>

## `heap-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-length</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Heap.cinch)

<a id="value-686561702d6e6577"></a>

## `heap-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

Construction

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 105

<a id="value-686561702d7065656b"></a>

## `heap-peek`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-peek</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

The least element.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 156

<a id="value-686561702d706f70"></a>

## `heap-pop`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-pop</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

Remove and return the least element under the `Ord` instance.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 164

<a id="value-686561702d706f702d6279"></a>

## `heap-pop-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-pop-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

Remove and return the element `first` puts first.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 168

<a id="value-686561702d70757368"></a>

## `heap-push`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-push</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Push under the `Ord` instance (a min-heap).

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 129

<a id="value-686561702d707573682d6279"></a>

## `heap-push-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-push-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)))</code></pre>

Push under `first`: `first x y` holds when x must come out before y
(a strict order).

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 134

<a id="value-686561702d746f2d6c697374"></a>

## `heap-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-to-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)</code></pre>

The elements in storage order (a heap order, not a sorted one).

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 258

<a id="value-686561702d776974682d6361706163697479"></a>

## `heap-with-capacity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-with-capacity</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

An empty heap with room for `n` elements before its first growth.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 109

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Mutable/Heap.cinch`. Source SHA-256: `0ff42c7b1f7a9ab5fedb7858d77b9687be658a1e928333a3a07144d24aeebd0a`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
