# Data.Mutable.IndexMap

An insertion-ordered hash map: dense entries with a tagged chunk index over them, positional access, O(1) swap-remove and order-keeping shift-remove.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.IndexMap`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.IndexMap.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Hash](Data.Hash.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Relation.Eq](Relation.Eq.md).

Non-embedded source closure: [Data.Mutable.IndexMap](Data.Mutable.IndexMap.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Mutable.IndexMap: the insertion-ordered hash map (Facebook F14's
`F14VectorMap`, Rust's `indexmap`, Python's `dict`).

Entries live densely in insertion order in three parallel slot arrays
(keys, values, and each key's 64-bit digest), and a separate index finds
them: the same chunked table as `Data.Mutable.HashTable` (8-slot chunks,
7-bit tags matched eight at a time, F14 overflow counts instead of
tombstones), whose slots hold entry positions rather than entries. So:

  * iteration is a walk over a dense array, in insertion order, with no
    empty slots to skip, which is F14's argument for the vector layout;
  * entries are addressable by position (`index-map-get-index`), and a
    key's position is a lookup away (`index-map-index-of`);
  * the index is rebuilt from the stored digests when it grows, so keys
    are never rehashed (an expensive `Hash`, a long `String`, pays once).

Removal comes in the two flavours `indexmap` offers. `swap-remove` is
O(1): the last entry moves into the hole (its index slot is found by its
stored digest and re-pointed), so the order of the rest changes.
`shift-remove` keeps the order: the tail moves down one place (one
memmove) and every index slot past the hole is re-pointed, which is O(n).

The invariant that makes every read sound: entries below the length are
written and the rest empty; an index slot is read only when its control
byte is full, and a full index slot always names an entry below the
length.

The trusted core is the private `cinch.runtime.slots*` imports (the
compiler's `slots.rs`); none of them leaves the module.

</details>

# Types and constructors

<a id="type-496e6465784d6170"></a>

## `IndexMap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">IndexMap</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The length word, and the cells holding the current entries and index.

[Source](/reference-sources/Data.Mutable.IndexMap.cinch) · line 63

# Values

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

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Mutable/IndexMap.cinch`. Source SHA-256: `119e94ae13c00611a834c4947d4db4c9bfed98b86da803c8ce707c16fe0cf501`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
