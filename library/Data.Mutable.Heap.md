# Data.Mutable.Heap

A 4-ary priority queue with its ordering fixed at construction: O(log n) push and pop, O(1) peek, and an O(n) Floyd build from a list.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.Heap`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.Heap.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Mutable.Heap](Data.Mutable.Heap.md).

<details><summary>Behaviour and boundary notes from the source</summary>

A mutable priority queue using a dense four-way heap and checked indexed repairs.

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

An opaque mutable heap tied to its mutation region.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 70

# Values

<a id="value-686561702d636c656172"></a>

## `heap-clear`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-clear</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

Release every element while retaining reusable storage capacity.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 246

<a id="value-686561702d647261696e"></a>

## `heap-drain`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-drain</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)</code></pre>

Remove all elements in natural order, leaving an empty heap.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 311

<a id="value-686561702d647261696e2d6279"></a>

## `heap-drain-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-drain-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))</code></pre>

Remove all elements under the supplied consistent ordering, leaving an empty heap.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 316

<a id="value-686561702d666978"></a>

## `heap-fix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-fix</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Repair an existing element after its externally mutable priority changes.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 378

<a id="value-686561702d6669782d6279"></a>

## `heap-fix-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-fix-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

Restore one changed position, returning False without mutation for an absent index.
The comparator and all other priorities must remain stable during repair.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 383

<a id="value-686561702d66726f6d2d6c697374"></a>

## `heap-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-from-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

Build a min-heap from a list in linear work.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 257

<a id="value-686561702d66726f6d2d6c6973742d6279"></a>

## `heap-from-list-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-from-list-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))</code></pre>

Build a heap bottom-up under a consistent strict ordering in linear work.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 263

<a id="value-686561702d676574"></a>

## `heap-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-get</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

Observe a checked position in the heap's current storage order.
Indices change during repairs; applications needing stable identities can
keep them in element values or use Data.Mutable.HeapView callbacks.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 334

<a id="value-686561702d696e6974"></a>

## `heap-init`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-init</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

Rebuild all subtrees under the natural ordering in linear work.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 395

<a id="value-686561702d696e69742d6279"></a>

## `heap-init-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-init-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Reestablish every subtree after changing the ordering or multiple mutable priorities.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 399

<a id="value-686561702d69732d656d707479"></a>

## `heap-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-is-empty</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

Test whether the heap contains no elements.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 144

<a id="value-686561702d6c656e677468"></a>

## `heap-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-length</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

Observe the number of initialized heap elements in constant time.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 140

<a id="value-686561702d6e6577"></a>

## `heap-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

Allocate an empty heap in the current mutation region.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 125

<a id="value-686561702d7065656b"></a>

## `heap-peek`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-peek</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

Observe the first element without removal, or None when empty.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 183

<a id="value-686561702d706f70"></a>

## `heap-pop`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-pop</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

Remove the naturally least element, or None when empty.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 192

<a id="value-686561702d706f702d6279"></a>

## `heap-pop-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-pop-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

Remove the first element under the supplied consistent strict ordering.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 197

<a id="value-686561702d70757368"></a>

## `heap-push`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-push</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Insert under the natural ordering, preserving the min-heap invariant.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 153

<a id="value-686561702d707573682d6279"></a>

## `heap-push-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-push-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)))</code></pre>

Insert under a strict ordering consistent with the heap's other operations.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 159

<a id="value-686561702d72656d6f7665"></a>

## `heap-remove`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-remove</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

Remove any checked position under the natural ordering.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 357

<a id="value-686561702d72656d6f76652d6279"></a>

## `heap-remove-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-remove-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)))</code></pre>

Remove a checked position and repair the displaced last element in logarithmic work.
The returned value is exactly the element previously at that position.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 362

<a id="value-686561702d7265706c616365"></a>

## `heap-replace`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-replace</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)))</code></pre>

Replace one existing position and repair heap order under the natural ordering.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 340

<a id="value-686561702d7265706c6163652d6279"></a>

## `heap-replace-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-replace-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))))</code></pre>

Replace a checked position and return its old value, repairing in logarithmic work.
An absent index returns None without changing the heap.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 345

<a id="value-686561702d746f2d6c697374"></a>

## `heap-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-to-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)</code></pre>

Copy the current storage order, which is a heap order rather than a sorted sequence.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 296

<a id="value-686561702d776974682d6361706163697479"></a>

## `heap-with-capacity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-with-capacity</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

Allocate an empty heap with room for the requested number of elements.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 130

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Mutable/Heap.cinch`. Source SHA-256: `3346efd37186c29673917fc2285adebe1cf3494584a4b06faa2c885cd61b27c3`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
