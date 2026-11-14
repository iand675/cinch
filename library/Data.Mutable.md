# Data.Mutable

One import for the mutable containers. Every container is indexed by a region r and every operation carries the Mut r label, a capability like IO r, so purity stays a checked property.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Mutable.Array](Data.Mutable.Array.md), [Data.Mutable.BTree](Data.Mutable.BTree.md), [Data.Mutable.BTreeSet](Data.Mutable.BTreeSet.md), [Data.Mutable.BitSet](Data.Mutable.BitSet.md), [Data.Mutable.Deque](Data.Mutable.Deque.md), [Data.Mutable.HashSet](Data.Mutable.HashSet.md), [Data.Mutable.HashTable](Data.Mutable.HashTable.md), [Data.Mutable.Heap](Data.Mutable.Heap.md), [Data.Mutable.IndexMap](Data.Mutable.IndexMap.md), [Data.Mutable.LruCache](Data.Mutable.LruCache.md), [Data.Mutable.UnionFind](Data.Mutable.UnionFind.md), [Data.Mutable.Var](Data.Mutable.Var.md), [Data.Mutable.Vec](Data.Mutable.Vec.md).

Non-embedded source closure: [Data.Mutable](Data.Mutable.md), [Data.Mutable.Array](Data.Mutable.Array.md), [Data.Mutable.BTree](Data.Mutable.BTree.md), [Data.Mutable.BTreeSet](Data.Mutable.BTreeSet.md), [Data.Mutable.BitSet](Data.Mutable.BitSet.md), [Data.Mutable.Deque](Data.Mutable.Deque.md), [Data.Mutable.HashSet](Data.Mutable.HashSet.md), [Data.Mutable.HashTable](Data.Mutable.HashTable.md), [Data.Mutable.Heap](Data.Mutable.Heap.md), [Data.Mutable.IndexMap](Data.Mutable.IndexMap.md), [Data.Mutable.LruCache](Data.Mutable.LruCache.md), [Data.Mutable.UnionFind](Data.Mutable.UnionFind.md), [Data.Mutable.Var](Data.Mutable.Var.md), [Data.Mutable.Vec](Data.Mutable.Vec.md).

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
  Data.Mutable.BTree      pooled degree-8 B-tree map      (absl::btree_map)
  Data.Mutable.BTreeSet   the B-tree's set                (absl::btree_set)
  Data.Mutable.IndexMap   insertion-ordered hash map      (folly::F14VectorMap, Rust indexmap)
  Data.Mutable.LruCache   bounded LRU cache               (folly::EvictingCacheMap)
  Data.Mutable.BitSet     word-wide dynamic bitset        (boost::dynamic_bitset)
  Data.Mutable.UnionFind  disjoint sets                   (union by size, path halving)

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

<a id="type-42547265654d6170"></a>

## `BTreeMap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BTreeMap</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The header (`meta`: entry count, root, height, free head, high
water, free count, node capacity) and the cell holding the pool.

[Source](/reference-sources/Data.Mutable.BTree.cinch) · line 73

<a id="type-4254726565536574"></a>

## `BTreeSet`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BTreeSet</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="type-426974536574"></a>

## `BitSet`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BitSet</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

One cell holding the current words.

[Source](/reference-sources/Data.Mutable.BitSet.cinch) · line 38

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

<a id="type-496e6465784d6170"></a>

## `IndexMap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">IndexMap</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The length word, and the cells holding the current entries and index.

[Source](/reference-sources/Data.Mutable.IndexMap.cinch) · line 63

<a id="type-4c72754361636865"></a>

## `LruCache`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">LruCache</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>)</code></pre>

`meta`: size, capacity, head, tail, free head. `links`: node i's
previous at 2i, next at 2i + 1.

[Source](/reference-sources/Data.Mutable.LruCache.cinch) · line 44

<a id="type-4d75744172726179"></a>

## `MutArray`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MutArray</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Array.cinch)

<a id="type-556e696f6e46696e64"></a>

## `UnionFind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UnionFind</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The header (element count, set count) and one cell holding the
words: parent of element i at 2i, size of the set rooted at i at
2i + 1.

[Source](/reference-sources/Data.Mutable.UnionFind.cinch) · line 33

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

<a id="value-6269747365742d6361706163697479"></a>

## `bitset-capacity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-capacity</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BitSet.cinch)

<a id="value-6269747365742d636c656172"></a>

## `bitset-clear`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-clear</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.BitSet.cinch)

<a id="value-6269747365742d636c6561722d616c6c"></a>

## `bitset-clear-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-clear-all</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BitSet.cinch)

<a id="value-6269747365742d636f756e74"></a>

## `bitset-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-count</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

The number of set bits: one popcount per word.

[Source](/reference-sources/Data.Mutable.BitSet.cinch) · line 128

<a id="value-6269747365742d646966666572656e63652d77697468"></a>

## `bitset-difference-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-difference-with</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

`a` becomes a & ~b.

[Source](/reference-sources/Data.Mutable.BitSet.cinch) · line 235

<a id="value-6269747365742d66726f6d2d6c697374"></a>

## `bitset-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-from-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BitSet.cinch)

<a id="value-6269747365742d696e746572736563742d77697468"></a>

## `bitset-intersect-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-intersect-with</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

`a` becomes a & b.

[Source](/reference-sources/Data.Mutable.BitSet.cinch) · line 220

<a id="value-6269747365742d69732d656d707479"></a>

## `bitset-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-is-empty</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BitSet.cinch)

<a id="value-6269747365742d6e6577"></a>

## `bitset-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span>)</code></pre>

An empty set with `n` bits backed before its first growth.

[Source](/reference-sources/Data.Mutable.BitSet.cinch) · line 83

<a id="value-6269747365742d6e6578742d736574"></a>

## `bitset-next-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-next-set</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>))</code></pre>

The least set bit at or above `i`.

[Source](/reference-sources/Data.Mutable.BitSet.cinch) · line 146

<a id="value-6269747365742d736574"></a>

## `bitset-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-set</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.BitSet.cinch)

<a id="value-6269747365742d74657374"></a>

## `bitset-test`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-test</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.BitSet.cinch)

<a id="value-6269747365742d746f2d6c697374"></a>

## `bitset-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-to-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>)</code></pre>

The set bits, ascending.

[Source](/reference-sources/Data.Mutable.BitSet.cinch) · line 165

<a id="value-6269747365742d746f67676c65"></a>

## `bitset-toggle`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-toggle</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.BitSet.cinch)

<a id="value-6269747365742d756e696f6e2d77697468"></a>

## `bitset-union-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-union-with</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

`a` becomes a | b (growing to b's words).

[Source](/reference-sources/Data.Mutable.BitSet.cinch) · line 205

<a id="value-62747265652d6365696c696e67"></a>

## `btree-ceiling`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-ceiling</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>)))</code></pre>

The entry with the least key not below `key`.

[Source](/reference-sources/Data.Mutable.BTree.cinch) · line 692

<a id="value-62747265652d636c656172"></a>

## `btree-clear`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-clear</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

Remove every entry; the pool's capacity stays.

[Source](/reference-sources/Data.Mutable.BTree.cinch) · line 857

<a id="value-62747265652d64656c657465"></a>

## `btree-delete`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-delete</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">v</span>))</code></pre>

Remove `key`: its value, or `None` when it was absent.

[Source](/reference-sources/Data.Mutable.BTree.cinch) · line 447

<a id="value-62747265652d666c6f6f72"></a>

## `btree-floor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-floor</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>)))</code></pre>

The entry with the greatest key not above `key`.

[Source](/reference-sources/Data.Mutable.BTree.cinch) · line 671

<a id="value-62747265652d666f6c64"></a>

## `btree-fold`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-fold</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">b</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt; <span class="tok-variable">b</span>))) -&gt; (<span class="tok-variable">b</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">b</span>)))</code></pre>

A left fold over every entry in ascending key order.

[Source](/reference-sources/Data.Mutable.BTree.cinch) · line 717

<a id="value-62747265652d666f6c642d72616e6765"></a>

## `btree-fold-range`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-fold-range</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">b</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">k</span>) =&gt; ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt; <span class="tok-variable">b</span>))) -&gt; (<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">k</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">b</span>)))))</code></pre>

A left fold over the entries with lo <= key < hi, ascending: the
descent skips every subtree wholly below lo, and stops at the first
key at or above hi.

[Source](/reference-sources/Data.Mutable.BTree.cinch) · line 746

<a id="value-62747265652d666f722d65616368"></a>

## `btree-for-each`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-for-each</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; ((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Run an action on every entry present when it starts, in key order.

[Source](/reference-sources/Data.Mutable.BTree.cinch) · line 830

<a id="value-62747265652d66726f6d2d6c697374"></a>

## `btree-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-from-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTree.cinch)

<a id="value-62747265652d686569676874"></a>

## `btree-height`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-height</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

The number of levels (1 for a tree that is one leaf).

[Source](/reference-sources/Data.Mutable.BTree.cinch) · line 297

<a id="value-62747265652d696e73657274"></a>

## `btree-insert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-insert</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)))</code></pre>

Set `key` to `val`, replacing a present value (the key already stored
is kept).

[Source](/reference-sources/Data.Mutable.BTree.cinch) · line 367

<a id="value-62747265652d69732d656d707479"></a>

## `btree-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-is-empty</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTree.cinch)

<a id="value-62747265652d6b657973"></a>

## `btree-keys`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-keys</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">k</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTree.cinch)

<a id="value-62747265652d6c6f6f6b7570"></a>

## `btree-lookup`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-lookup</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">v</span>))</code></pre>

Lookup

[Source](/reference-sources/Data.Mutable.BTree.cinch) · line 306

<a id="value-62747265652d6c6f6f6b75702d6f72"></a>

## `btree-lookup-or`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-lookup-or</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">v</span>)))</code></pre>

[Source](/reference-sources/Data.Mutable.BTree.cinch)

<a id="value-62747265652d6d6178"></a>

## `btree-max`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-max</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.BTree.cinch)

<a id="value-62747265652d6d656d626572"></a>

## `btree-member`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-member</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.BTree.cinch)

<a id="value-62747265652d6d696e"></a>

## `btree-min`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-min</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.BTree.cinch)

<a id="value-62747265652d6d6f64696679"></a>

## `btree-modify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-modify</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt; ((<span class="tok-variable">v</span> -&gt; <span class="tok-variable">v</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

Rewrite the value at `key` through `f`, in place: `False` when absent.

[Source](/reference-sources/Data.Mutable.BTree.cinch) · line 344

<a id="value-62747265652d6e6577"></a>

## `btree-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)</code></pre>

Construction and size

[Source](/reference-sources/Data.Mutable.BTree.cinch) · line 267

<a id="value-62747265652d706f702d6d6178"></a>

## `btree-pop-max`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-pop-max</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.BTree.cinch)

<a id="value-62747265652d706f702d6d696e"></a>

## `btree-pop-min`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-pop-min</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.BTree.cinch)

<a id="value-62747265652d7365742d6365696c696e67"></a>

## `btree-set-ceiling`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-ceiling</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

The least member not below `x`.

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch) · line 74

<a id="value-62747265652d7365742d636c656172"></a>

## `btree-set-clear`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-clear</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d64656c657465"></a>

## `btree-set-delete`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-delete</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Remove `x`: `True` when it was a member.

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch) · line 52

<a id="value-62747265652d7365742d666c6f6f72"></a>

## `btree-set-floor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-floor</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

The greatest member not above `x`.

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch) · line 70

<a id="value-62747265652d7365742d666f6c64"></a>

## `btree-set-fold`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-fold</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>)) -&gt; (<span class="tok-variable">b</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">b</span>)))</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d666f6c642d72616e6765"></a>

## `btree-set-fold-range`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-fold-range</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>)) -&gt; (<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">b</span>)))))</code></pre>

A left fold over the members in [lo, hi), ascending.

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch) · line 81

<a id="value-62747265652d7365742d66726f6d2d6c697374"></a>

## `btree-set-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-from-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d696e73657274"></a>

## `btree-set-insert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-insert</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Add `x`: `True` when it was not already a member.

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch) · line 41

<a id="value-62747265652d7365742d69732d656d707479"></a>

## `btree-set-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-is-empty</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d6d6178"></a>

## `btree-set-max`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-max</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d6d656d626572"></a>

## `btree-set-member`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-member</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d6d696e"></a>

## `btree-set-min`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-min</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d6e6577"></a>

## `btree-set-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d706f702d6d6178"></a>

## `btree-set-pop-max`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-pop-max</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d706f702d6d696e"></a>

## `btree-set-pop-min`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-pop-min</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d73697a65"></a>

## `btree-set-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-size</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d746f2d6c697374"></a>

## `btree-set-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-to-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d766572696679"></a>

## `btree-set-verify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-verify</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d73697a65"></a>

## `btree-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-size</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTree.cinch)

<a id="value-62747265652d746f2d6c697374"></a>

## `btree-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-to-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>))</code></pre>

The entries in ascending key order (built from the right).

[Source](/reference-sources/Data.Mutable.BTree.cinch) · line 779

<a id="value-62747265652d76616c756573"></a>

## `btree-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-values</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">v</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTree.cinch)

<a id="value-62747265652d766572696679"></a>

## `btree-verify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-verify</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTree</span>.<span class="tok-constructor">BTreeMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

The tree's whole structural contract: keys strictly ascending in key
order across nodes, every non-root node holding 7 .. 15 keys, every
leaf at the same depth (the recorded height), and the entry count the
header records. `True` when all of it holds.

[Source](/reference-sources/Data.Mutable.BTree.cinch) · line 885

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

<a id="value-696e6465782d6d61702d636c656172"></a>

## `index-map-clear`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">index-map-clear</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">IndexMap</span>.<span class="tok-constructor">IndexMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.IndexMap.cinch)

<a id="value-696e6465782d6d61702d666f6c64"></a>

## `index-map-fold`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">index-map-fold</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">b</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt; <span class="tok-variable">b</span>))) -&gt; (<span class="tok-variable">b</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">IndexMap</span>.<span class="tok-constructor">IndexMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">b</span>)))</code></pre>

A left fold in insertion order.

[Source](/reference-sources/Data.Mutable.IndexMap.cinch) · line 589

<a id="value-696e6465782d6d61702d66726f6d2d6c697374"></a>

## `index-map-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">index-map-from-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">IndexMap</span>.<span class="tok-constructor">IndexMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.IndexMap.cinch)

<a id="value-696e6465782d6d61702d6765742d696e646578"></a>

## `index-map-get-index`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">index-map-get-index</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">IndexMap</span>.<span class="tok-constructor">IndexMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>)))</code></pre>

The entry at position `i` in insertion order.

[Source](/reference-sources/Data.Mutable.IndexMap.cinch) · line 355

<a id="value-696e6465782d6d61702d696e6465782d6f66"></a>

## `index-map-index-of`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">index-map-index-of</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">IndexMap</span>.<span class="tok-constructor">IndexMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>))</code></pre>

The position of `key` in insertion order.

[Source](/reference-sources/Data.Mutable.IndexMap.cinch) · line 349

<a id="value-696e6465782d6d61702d696e73657274"></a>

## `index-map-insert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">index-map-insert</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">IndexMap</span>.<span class="tok-constructor">IndexMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)))</code></pre>

Set `key` to `val`: a present key keeps its position (and its stored
key); a new one goes at the end.

[Source](/reference-sources/Data.Mutable.IndexMap.cinch) · line 369

<a id="value-696e6465782d6d61702d69732d656d707479"></a>

## `index-map-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">index-map-is-empty</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">IndexMap</span>.<span class="tok-constructor">IndexMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.IndexMap.cinch)

<a id="value-696e6465782d6d61702d6b657973"></a>

## `index-map-keys`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">index-map-keys</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">IndexMap</span>.<span class="tok-constructor">IndexMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">k</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.IndexMap.cinch)

<a id="value-696e6465782d6d61702d6c6f6f6b7570"></a>

## `index-map-lookup`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">index-map-lookup</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">IndexMap</span>.<span class="tok-constructor">IndexMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">v</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.IndexMap.cinch)

<a id="value-696e6465782d6d61702d6c6f6f6b75702d6f72"></a>

## `index-map-lookup-or`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">index-map-lookup-or</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">IndexMap</span>.<span class="tok-constructor">IndexMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">v</span>)))</code></pre>

[Source](/reference-sources/Data.Mutable.IndexMap.cinch)

<a id="value-696e6465782d6d61702d6d656d626572"></a>

## `index-map-member`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">index-map-member</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">IndexMap</span>.<span class="tok-constructor">IndexMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.IndexMap.cinch)

<a id="value-696e6465782d6d61702d6e6577"></a>

## `index-map-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">index-map-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">IndexMap</span>.<span class="tok-constructor">IndexMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)</code></pre>

Construction and size

[Source](/reference-sources/Data.Mutable.IndexMap.cinch) · line 285

<a id="value-696e6465782d6d61702d706f70"></a>

## `index-map-pop`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">index-map-pop</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">IndexMap</span>.<span class="tok-constructor">IndexMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>))</code></pre>

Remove and return the last entry.

[Source](/reference-sources/Data.Mutable.IndexMap.cinch) · line 542

<a id="value-696e6465782d6d61702d73686966742d72656d6f7665"></a>

## `index-map-shift-remove`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">index-map-shift-remove</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">IndexMap</span>.<span class="tok-constructor">IndexMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">v</span>))</code></pre>

Remove `key` keeping the order of the rest: the tail moves down one
place and every index slot past the hole is re-pointed (O(n)).

[Source](/reference-sources/Data.Mutable.IndexMap.cinch) · line 495

<a id="value-696e6465782d6d61702d73697a65"></a>

## `index-map-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">index-map-size</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">IndexMap</span>.<span class="tok-constructor">IndexMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.IndexMap.cinch)

<a id="value-696e6465782d6d61702d737761702d72656d6f7665"></a>

## `index-map-swap-remove`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">index-map-swap-remove</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">IndexMap</span>.<span class="tok-constructor">IndexMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">v</span>))</code></pre>

Remove `key` by moving the last entry into its place: O(1), and the
moved entry changes position.

[Source](/reference-sources/Data.Mutable.IndexMap.cinch) · line 453

<a id="value-696e6465782d6d61702d746f2d6c697374"></a>

## `index-map-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">index-map-to-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">IndexMap</span>.<span class="tok-constructor">IndexMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.IndexMap.cinch)

<a id="value-696e6465782d6d61702d76616c756573"></a>

## `index-map-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">index-map-values</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">IndexMap</span>.<span class="tok-constructor">IndexMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">v</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.IndexMap.cinch)

<a id="value-6c72752d6361706163697479"></a>

## `lru-capacity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lru-capacity</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LruCache</span>.<span class="tok-constructor">LruCache</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.LruCache.cinch)

<a id="value-6c72752d636c656172"></a>

## `lru-clear`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lru-clear</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LruCache</span>.<span class="tok-constructor">LruCache</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

Drop every entry; the capacity stays.

[Source](/reference-sources/Data.Mutable.LruCache.cinch) · line 218

<a id="value-6c72752d64656c657465"></a>

## `lru-delete`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lru-delete</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LruCache</span>.<span class="tok-constructor">LruCache</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">v</span>))</code></pre>

Remove `key`: its value, or `None` when it was absent.

[Source](/reference-sources/Data.Mutable.LruCache.cinch) · line 196

<a id="value-6c72752d676574"></a>

## `lru-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lru-get</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LruCache</span>.<span class="tok-constructor">LruCache</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">v</span>))</code></pre>

The value at `key`, marking it the most recently used.

[Source](/reference-sources/Data.Mutable.LruCache.cinch) · line 135

<a id="value-6c72752d6d656d626572"></a>

## `lru-member`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lru-member</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LruCache</span>.<span class="tok-constructor">LruCache</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.LruCache.cinch)

<a id="value-6c72752d6e6577"></a>

## `lru-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lru-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LruCache</span>.<span class="tok-constructor">LruCache</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)</code></pre>

An empty cache holding at most `capacity` entries (at least one).

[Source](/reference-sources/Data.Mutable.LruCache.cinch) · line 98

<a id="value-6c72752d7065656b"></a>

## `lru-peek`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lru-peek</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LruCache</span>.<span class="tok-constructor">LruCache</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">v</span>))</code></pre>

The value at `key`, leaving the recency order alone.

[Source](/reference-sources/Data.Mutable.LruCache.cinch) · line 144

<a id="value-6c72752d707574"></a>

## `lru-put`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lru-put</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LruCache</span>.<span class="tok-constructor">LruCache</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt; (<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>))))</code></pre>

Set `key` to `val` and mark it the most recently used. A full cache
first evicts its least recently used entry, which is returned.

[Source](/reference-sources/Data.Mutable.LruCache.cinch) · line 160

<a id="value-6c72752d73697a65"></a>

## `lru-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lru-size</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LruCache</span>.<span class="tok-constructor">LruCache</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.LruCache.cinch)

<a id="value-6c72752d746f2d6c697374"></a>

## `lru-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lru-to-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">LruCache</span>.<span class="tok-constructor">LruCache</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>))</code></pre>

The entries from the most recently used to the least.

[Source](/reference-sources/Data.Mutable.LruCache.cinch) · line 208

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

<a id="value-756e696f6e2d66696e642d616464"></a>

## `union-find-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">union-find-add</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">UnionFind</span>.<span class="tok-constructor">UnionFind</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

A new singleton set: its element.

[Source](/reference-sources/Data.Mutable.UnionFind.cinch) · line 75

<a id="value-756e696f6e2d66696e642d66696e64"></a>

## `union-find-find`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">union-find-find</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">UnionFind</span>.<span class="tok-constructor">UnionFind</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>))</code></pre>

The representative of x's set (`None` when x is not an element).

[Source](/reference-sources/Data.Mutable.UnionFind.cinch) · line 110

<a id="value-756e696f6e2d66696e642d6c656e677468"></a>

## `union-find-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">union-find-length</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">UnionFind</span>.<span class="tok-constructor">UnionFind</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.UnionFind.cinch)

<a id="value-756e696f6e2d66696e642d6e6577"></a>

## `union-find-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">union-find-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">UnionFind</span>.<span class="tok-constructor">UnionFind</span> <span class="tok-variable">r</span>)</code></pre>

`n` singleton sets.

[Source](/reference-sources/Data.Mutable.UnionFind.cinch) · line 60

<a id="value-756e696f6e2d66696e642d73616d65"></a>

## `union-find-same`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">union-find-same</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">UnionFind</span>.<span class="tok-constructor">UnionFind</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

[Source](/reference-sources/Data.Mutable.UnionFind.cinch)

<a id="value-756e696f6e2d66696e642d7365742d636f756e74"></a>

## `union-find-set-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">union-find-set-count</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">UnionFind</span>.<span class="tok-constructor">UnionFind</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

The number of disjoint sets.

[Source](/reference-sources/Data.Mutable.UnionFind.cinch) · line 105

<a id="value-756e696f6e2d66696e642d7365742d73697a65"></a>

## `union-find-set-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">union-find-set-size</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">UnionFind</span>.<span class="tok-constructor">UnionFind</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>))</code></pre>

The size of x's set (0 when x is not an element).

[Source](/reference-sources/Data.Mutable.UnionFind.cinch) · line 165

<a id="value-756e696f6e2d66696e642d756e696f6e"></a>

## `union-find-union`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">union-find-union</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">UnionFind</span>.<span class="tok-constructor">UnionFind</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

Merge the sets of x and y: `True` when they were separate (and both
are elements).

[Source](/reference-sources/Data.Mutable.UnionFind.cinch) · line 130

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

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Mutable.cinch`. Source SHA-256: `41f43a5edd93e517e7aa298c9ae7b3af24b56313cc49dd20dfaccd380713aeaa`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
