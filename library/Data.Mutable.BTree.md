# Data.Mutable.BTree

An ordered mutable map: a degree-8 B-tree whose nodes are indices into one pooled set of slot arrays, with floor, ceiling, range folds, pop-min/max and a structural verifier.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.BTree`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.BTree.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Mutable.BTree](Data.Mutable.BTree.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Mutable.BTree: the ordered mutable map (abseil `btree_map`).

A B-tree of minimum degree 8: every node holds 7 .. 15 keys (the root
0 .. 15) and an internal node one more child than keys, so a million
entries sit at most five levels deep and a search compares a handful of
keys per level inside one node's contiguous slots. That locality is
abseil's argument for a B-tree over a red-black tree, and it holds here
with more force: a node is not a heap block at all.

Layout (a node pool). Nodes are indices into three slot arrays shared by
the whole tree: node x owns key and value slots 15x .. 15x + 14, and 18
words of `nodes` from 18x (its key count, a leaf flag, and up to 16 child
indices). Allocating a node takes an index from a free list or the high
water mark; freeing one pushes it back. The pool doubles when an
operation could run out, which moves every slot once (no count touched).
So the tree's shape costs no allocation per node, no reference count per
child pointer, and no pointer chase into a separate block per level.

Algorithms. CLRS chapter 18 in its single-pass form: an insertion splits
every full node on its way down, so it never has to back up; a deletion
tops every node it descends into up to at least 8 keys first (by
rotating a key in from a sibling or merging with one), so the key it
finally removes from a leaf never leaves an underfull node behind. A
search within a node is a binary search with `Ord`'s `<`; two keys are
equal when neither is less, so no `Eq` is needed.

Costs: lookup, insert, delete, floor and ceiling are O(log n); min and
max are O(log n) walks; folds and `btree-to-list` are O(n) in key order;
`btree-fold-range lo hi` is O(log n + the entries in [lo, hi)).

The invariant that makes every read sound: node x's key and value slots
below its count are written and the rest are empty, and a node's child
words are read only below its count + 1 and only when it is internal.
`btree-verify` checks the whole structural contract (order, counts, equal
leaf depth, the entry count), as abseil's `verify()` does.

Named refusals:
  iteration under mutation: the folds are pure, so they cannot change
    the tree; `btree-for-each` runs its action over the entries listed
    when it started;
  keys whose `Ord` is not a strict total order: the tree's shape is only
    as sound as `<` (as for every comparison-ordered container).

The trusted core is the private `cinch.runtime.slots*` imports (the
compiler's `slots.rs`); none of them leaves the module.

</details>

# Types and constructors

<a id="type-42547265654d6170"></a>

## `BTreeMap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BTreeMap</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The header (`meta`: entry count, root, height, free head, high
water, free count, node capacity) and the cell holding the pool.

[Source](/reference-sources/Data.Mutable.BTree.cinch) · line 73

# Values

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

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Mutable/BTree.cinch`. Source SHA-256: `9741c4de3eb73bcf097d0dc27f48d9ace038c8a9ca7e4a8062e6e6f41cc19c54`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
