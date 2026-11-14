# Data.Mutable

One import for the mutable containers. Every container is indexed by a region r and every operation carries the Mut r label, a capability like IO r, so purity stays a checked property.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Mutable.Array](Data.Mutable.Array.md), [Data.Mutable.Deque](Data.Mutable.Deque.md), [Data.Mutable.HashSet](Data.Mutable.HashSet.md), [Data.Mutable.HashTable](Data.Mutable.HashTable.md), [Data.Mutable.Heap](Data.Mutable.Heap.md), [Data.Mutable.Var](Data.Mutable.Var.md), [Data.Mutable.Vec](Data.Mutable.Vec.md).

Non-embedded source closure: [Data.Mutable](Data.Mutable.md), [Data.Mutable.Array](Data.Mutable.Array.md), [Data.Mutable.Deque](Data.Mutable.Deque.md), [Data.Mutable.HashSet](Data.Mutable.HashSet.md), [Data.Mutable.HashTable](Data.Mutable.HashTable.md), [Data.Mutable.Heap](Data.Mutable.Heap.md), [Data.Mutable.Var](Data.Mutable.Var.md), [Data.Mutable.Vec](Data.Mutable.Vec.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Mutable: the mutable containers, behind one import.

Every container is indexed by a region `r`, and every operation on it
carries the `Mut r` label in its row:

  word-counts :: forall r. List String ->{Mut r} HashTable r String Nat
  word-counts ws =
    let t = hash-table-new () in
    let counted = count-into t ws in
    t

`Mut r` is a capability exactly like `IO r`: a definition whose row
lacks it cannot touch a container of region `r`, so a pure function
stays pure and every function that mutates says so in its type. A
program discharges the label at its entry point
(`main :: Unit ->{Mut r} Int`). Labels need no declaration, so there is
nothing to import for the row itself.

The family (each module's header gives its layout, costs and the
invariant that keeps its reads sound):

  Data.Mutable.Vec        growable vector, introsort      (std::vector)
  Data.Mutable.HashTable  SwissTable/F14 hash table       (absl::flat_hash_map, folly::F14ValueMap)
  Data.Mutable.HashSet    the table's set                 (absl::flat_hash_set)
  Data.Mutable.Deque      power-of-two ring buffer        (Rust VecDeque)
  Data.Mutable.Heap       4-ary priority queue            (boost::d_ary_heap)
  Data.Mutable.Array      fixed-length mutable array      (std::array by reference)
  Data.Mutable.Var        one mutable cell                (IORef, STRef)

Named refusals (boundaries the family deliberately does not cross):
  a pure runner: `run-mut :: (forall r. Unit ->{Mut r} a) -> a` (the
    `ST` runner) needs a rank-2 argument, which the checker refuses
    ("quantifiers are only supported at the outermost position"), and a
    handler cannot stand in for it, because a handler only dispatches
    operations its handled expression performs directly (decision D23),
    never ones a called container operation performs. Without the
    quantifier a runner would let a container escape the scope that ran
    it and be mutated under a pure type, so none is offered;
  cross-task sharing: a container is not synchronized, so it must stay
    with the task that made it (`Effect.Stm`'s `TVar` is the shared
    mutable cell);
  aliasing views: no container hands out an `Array` that aliases its
    storage; `vec-freeze` and `mut-array-freeze` copy.

</details>

# Types and constructors

<a id="type-4465717565"></a>

## `Deque`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Deque</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The header: `head` at word 0 and `len` at word 1, and one cell
holding the current ring.

[Source](/reference-sources/Data.Mutable.Deque.cinch) · line 46

<a id="type-48617368536574"></a>

## `HashSet`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HashSet</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.HashSet.cinch)

<a id="type-486173685461626c65"></a>

## `HashTable`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HashTable</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The table's handle: one cell holding the current generation,
replaced when the table grows.

[Source](/reference-sources/Data.Mutable.HashTable.cinch) · line 97

<a id="type-48656170"></a>

## `Heap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Heap</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The ordering, the length word, and the cell holding the storage.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 49

<a id="type-4d75744172726179"></a>

## `MutArray`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MutArray</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Array.cinch)

<a id="type-566172"></a>

## `Var`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Var</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Var.cinch)

<a id="type-566563"></a>

## `Vec`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Vec</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The header: one raw word (the length) and one cell holding the
current storage, replaced when the vector reallocates.

[Source](/reference-sources/Data.Mutable.Vec.cinch) · line 64

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

<a id="value-686173682d7365742d636c656172"></a>

## `hash-set-clear`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-set-clear</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashSet</span>.<span class="tok-constructor">HashSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.HashSet.cinch)

<a id="value-686173682d7365742d64656c657465"></a>

## `hash-set-delete`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-set-delete</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">a</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashSet</span>.<span class="tok-constructor">HashSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Remove `x`: `True` when it was a member.

[Source](/reference-sources/Data.Mutable.HashSet.cinch) · line 45

<a id="value-686173682d7365742d666f6c64"></a>

## `hash-set-fold`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-set-fold</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>)) -&gt; (<span class="tok-variable">b</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashSet</span>.<span class="tok-constructor">HashSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">b</span>)))</code></pre>

[Source](/reference-sources/Data.Mutable.HashSet.cinch)

<a id="value-686173682d7365742d666f722d65616368"></a>

## `hash-set-for-each`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-set-for-each</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashSet</span>.<span class="tok-constructor">HashSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; ((<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Run an action on every member present when it starts.

[Source](/reference-sources/Data.Mutable.HashSet.cinch) · line 77

<a id="value-686173682d7365742d66726f6d2d6c697374"></a>

## `hash-set-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-set-from-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">a</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashSet</span>.<span class="tok-constructor">HashSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.HashSet.cinch)

<a id="value-686173682d7365742d696e73657274"></a>

## `hash-set-insert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-set-insert</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">a</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashSet</span>.<span class="tok-constructor">HashSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Add `x`: `True` when it was not already a member.

[Source](/reference-sources/Data.Mutable.HashSet.cinch) · line 38

<a id="value-686173682d7365742d69732d656d707479"></a>

## `hash-set-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-set-is-empty</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashSet</span>.<span class="tok-constructor">HashSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.HashSet.cinch)

<a id="value-686173682d7365742d6d656d626572"></a>

## `hash-set-member`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-set-member</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">a</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashSet</span>.<span class="tok-constructor">HashSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.HashSet.cinch)

<a id="value-686173682d7365742d6e6577"></a>

## `hash-set-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-set-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashSet</span>.<span class="tok-constructor">HashSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.HashSet.cinch)

<a id="value-686173682d7365742d72657365727665"></a>

## `hash-set-reserve`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-set-reserve</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashSet</span>.<span class="tok-constructor">HashSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.HashSet.cinch)

<a id="value-686173682d7365742d73697a65"></a>

## `hash-set-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-set-size</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashSet</span>.<span class="tok-constructor">HashSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.HashSet.cinch)

<a id="value-686173682d7365742d746f2d6c697374"></a>

## `hash-set-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-set-to-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashSet</span>.<span class="tok-constructor">HashSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)</code></pre>

The members in the table's slot order (unstable; see the table).

[Source](/reference-sources/Data.Mutable.HashSet.cinch) · line 70

<a id="value-686173682d7365742d776974682d6361706163697479"></a>

## `hash-set-with-capacity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-set-with-capacity</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashSet</span>.<span class="tok-constructor">HashSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.HashSet.cinch)

<a id="value-686173682d7461626c652d6361706163697479"></a>

## `hash-table-capacity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-capacity</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

The entry count the table takes before it next grows.

[Source](/reference-sources/Data.Mutable.HashTable.cinch) · line 339

<a id="value-686173682d7461626c652d636c656172"></a>

## `hash-table-clear`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-clear</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

Remove every entry; the capacity stays.

[Source](/reference-sources/Data.Mutable.HashTable.cinch) · line 557

<a id="value-686173682d7461626c652d64656c657465"></a>

## `hash-table-delete`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-delete</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">v</span>))</code></pre>

Remove `key`: its value, or `None` when it was absent.

[Source](/reference-sources/Data.Mutable.HashTable.cinch) · line 533

<a id="value-686173682d7461626c652d666f6c64"></a>

## `hash-table-fold`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-fold</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">b</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt; <span class="tok-variable">b</span>))) -&gt; (<span class="tok-variable">b</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">b</span>)))</code></pre>

A fold over the entries in slot order (see the refusal in the
header), through a pure function over a snapshot of the table.

[Source](/reference-sources/Data.Mutable.HashTable.cinch) · line 588

<a id="value-686173682d7461626c652d666f722d65616368"></a>

## `hash-table-for-each`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-for-each</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; ((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Run an action on every entry present when it starts (see the
refusal in the header).

[Source](/reference-sources/Data.Mutable.HashTable.cinch) · line 615

<a id="value-686173682d7461626c652d66726f6d2d6c697374"></a>

## `hash-table-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-from-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)</code></pre>

Lists and folds

[Source](/reference-sources/Data.Mutable.HashTable.cinch) · line 571

<a id="value-686173682d7461626c652d696e73657274"></a>

## `hash-table-insert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-insert</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)))</code></pre>

Set `key` to `val`, replacing a present value (the key already stored
is kept).

[Source](/reference-sources/Data.Mutable.HashTable.cinch) · line 457

<a id="value-686173682d7461626c652d696e736572742d77697468"></a>

## `hash-table-insert-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-insert-with</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; ((<span class="tok-variable">v</span> -&gt; (<span class="tok-variable">v</span> -&gt; <span class="tok-variable">v</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))))</code></pre>

Insert `val`, or combine it with the present value as `f val old`
(the counting idiom: `hash-table-insert-with (+) t word 1`).

[Source](/reference-sources/Data.Mutable.HashTable.cinch) · line 483

<a id="value-686173682d7461626c652d69732d656d707479"></a>

## `hash-table-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-is-empty</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.HashTable.cinch)

<a id="value-686173682d7461626c652d6b657973"></a>

## `hash-table-keys`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-keys</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">k</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.HashTable.cinch)

<a id="value-686173682d7461626c652d6c6f6f6b7570"></a>

## `hash-table-lookup`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-lookup</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">v</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.HashTable.cinch)

<a id="value-686173682d7461626c652d6c6f6f6b75702d6f72"></a>

## `hash-table-lookup-or`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-lookup-or</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">v</span>)))</code></pre>

The value at `key`, or `dflt`: a lookup that allocates nothing.

[Source](/reference-sources/Data.Mutable.HashTable.cinch) · line 388

<a id="value-686173682d7461626c652d6d656d626572"></a>

## `hash-table-member`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-member</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.HashTable.cinch)

<a id="value-686173682d7461626c652d6d6f64696679"></a>

## `hash-table-modify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-modify</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt; ((<span class="tok-variable">v</span> -&gt; <span class="tok-variable">v</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

Rewrite the value at `key` through `f`, in place: `False` when the
key is absent.

[Source](/reference-sources/Data.Mutable.HashTable.cinch) · line 504

<a id="value-686173682d7461626c652d6e6577"></a>

## `hash-table-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)</code></pre>

Construction and size

[Source](/reference-sources/Data.Mutable.HashTable.cinch) · line 316

<a id="value-686173682d7461626c652d72657365727665"></a>

## `hash-table-reserve`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-reserve</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Make room for `n` entries in all without a further growth.

[Source](/reference-sources/Data.Mutable.HashTable.cinch) · line 344

<a id="value-686173682d7461626c652d73697a65"></a>

## `hash-table-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-size</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.HashTable.cinch)

<a id="value-686173682d7461626c652d746f2d6c697374"></a>

## `hash-table-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-to-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.HashTable.cinch)

<a id="value-686173682d7461626c652d7472792d696e73657274"></a>

## `hash-table-try-insert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-try-insert</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

Insert only when `key` is absent: `True` when it was inserted.

[Source](/reference-sources/Data.Mutable.HashTable.cinch) · line 470

<a id="value-686173682d7461626c652d757073657274"></a>

## `hash-table-upsert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-upsert</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">v</span> -&gt; <span class="tok-variable">v</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)))</code></pre>

Set `key` to `f` of its present value (`None` when absent).

[Source](/reference-sources/Data.Mutable.HashTable.cinch) · line 516

<a id="value-686173682d7461626c652d76616c756573"></a>

## `hash-table-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-values</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">v</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.HashTable.cinch)

<a id="value-686173682d7461626c652d776974682d6361706163697479"></a>

## `hash-table-with-capacity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hash-table-with-capacity</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">HashTable</span>.<span class="tok-constructor">HashTable</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)</code></pre>

An empty table that holds `n` entries before its first growth.

[Source](/reference-sources/Data.Mutable.HashTable.cinch) · line 320

<a id="value-686561702d636c656172"></a>

## `heap-clear`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-clear</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

Drop every element; the capacity stays.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 194

<a id="value-686561702d647261696e"></a>

## `heap-drain`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-drain</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)</code></pre>

Pop everything: the elements in the heap's order, leaving it empty.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 252

<a id="value-686561702d66726f6d2d6c697374"></a>

## `heap-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-from-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

Lists

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 204

<a id="value-686561702d66726f6d2d6c6973742d6279"></a>

## `heap-from-list-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-from-list-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))</code></pre>

Floyd's build: write the list in, then sift every internal node
down from the last one, O(n) in all.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 209

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

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

A min-heap under the `Ord` instance.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 99

<a id="value-686561702d6e65772d6279"></a>

## `heap-new-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-new-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))</code></pre>

A heap whose least element is the one `first` puts first: `first x
y` holds when x must come out before y (a strict order).

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 104

<a id="value-686561702d7065656b"></a>

## `heap-peek`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-peek</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

The least element.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 147

<a id="value-686561702d706f70"></a>

## `heap-pop`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-pop</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

Remove and return the least element.

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 155

<a id="value-686561702d70757368"></a>

## `heap-push`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-push</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Push and pop

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 126

<a id="value-686561702d746f2d6c697374"></a>

## `heap-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-to-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)</code></pre>

The elements in storage order (a heap order, not a sorted one).

[Source](/reference-sources/Data.Mutable.Heap.cinch) · line 239

<a id="value-686561702d776974682d63617061636974792d6279"></a>

## `heap-with-capacity-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">heap-with-capacity-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Heap</span>.<span class="tok-constructor">Heap</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.Heap.cinch)

<a id="value-6d75742d61727261792d66696c6c"></a>

## `mut-array-fill`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-fill</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Every element becomes `x`.

[Source](/reference-sources/Data.Mutable.Array.cinch) · line 143

<a id="value-6d75742d61727261792d666f6c64"></a>

## `mut-array-fold`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-fold</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>)) -&gt; (<span class="tok-variable">b</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">b</span>)))</code></pre>

[Source](/reference-sources/Data.Mutable.Array.cinch)

<a id="value-6d75742d61727261792d667265657a65"></a>

## `mut-array-freeze`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-freeze</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Array</span> <span class="tok-variable">a</span>)</code></pre>

An immutable copy (a fresh block).

[Source](/reference-sources/Data.Mutable.Array.cinch) · line 170

<a id="value-6d75742d61727261792d66726f6d2d6c697374"></a>

## `mut-array-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-from-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Array.cinch)

<a id="value-6d75742d61727261792d676574"></a>

## `mut-array-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-get</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.Array.cinch)

<a id="value-6d75742d61727261792d6c656e677468"></a>

## `mut-array-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-length</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Array.cinch)

<a id="value-6d75742d61727261792d6d6f64696679"></a>

## `mut-array-modify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-modify</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

Rewrite element `i` through `f`, moving it out and back.

[Source](/reference-sources/Data.Mutable.Array.cinch) · line 116

<a id="value-6d75742d61727261792d6e6577"></a>

## `mut-array-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))</code></pre>

`n` copies of `x`.

[Source](/reference-sources/Data.Mutable.Array.cinch) · line 51

<a id="value-6d75742d61727261792d736574"></a>

## `mut-array-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-set</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

Replace element `i`: `False` (and nothing written) when out of range.

[Source](/reference-sources/Data.Mutable.Array.cinch) · line 107

<a id="value-6d75742d61727261792d73776170"></a>

## `mut-array-swap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-swap</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

[Source](/reference-sources/Data.Mutable.Array.cinch)

<a id="value-6d75742d61727261792d746162756c617465"></a>

## `mut-array-tabulate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-tabulate</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Nat</span> -&gt; <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))</code></pre>

Element i is `f i`.

[Source](/reference-sources/Data.Mutable.Array.cinch) · line 65

<a id="value-6d75742d61727261792d74686177"></a>

## `mut-array-thaw`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-thaw</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

A mutable copy of an immutable array.

[Source](/reference-sources/Data.Mutable.Array.cinch) · line 179

<a id="value-6d75742d61727261792d746f2d6c697374"></a>

## `mut-array-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-to-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Array.cinch)

<a id="value-7661722d6d6f64696679"></a>

## `var-modify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">var-modify</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Var</span>.<span class="tok-constructor">Var</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.Var.cinch)

<a id="value-7661722d6e6577"></a>

## `var-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">var-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Var</span>.<span class="tok-constructor">Var</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Var.cinch)

<a id="value-7661722d72656164"></a>

## `var-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">var-read</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Var</span>.<span class="tok-constructor">Var</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Var.cinch)

<a id="value-7661722d73776170"></a>

## `var-swap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">var-swap</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Var</span>.<span class="tok-constructor">Var</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">a</span>))</code></pre>

Store `x` and return the value it replaced.

[Source](/reference-sources/Data.Mutable.Var.cinch) · line 54

<a id="value-7661722d7772697465"></a>

## `var-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">var-write</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Var</span>.<span class="tok-constructor">Var</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.Var.cinch)

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

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Mutable.cinch`. Source SHA-256: `1fee11d2ed1b0f718a9f933b3f3f72fa831d769694a6f4ef25333c5b88a41c51`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
