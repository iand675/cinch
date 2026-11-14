# Data.Mutable.LruCache

A bounded least-recently-used cache: a hash table to nodes threaded on a recency list, with O(1) get, put and eviction.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.LruCache`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.LruCache.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Hash](Data.Hash.md), [Data.Machine](Data.Machine.md), [Data.Mutable.HashTable](Data.Mutable.HashTable.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Relation.Eq](Relation.Eq.md).

Non-embedded source closure: [Data.Mutable.HashTable](Data.Mutable.HashTable.md), [Data.Mutable.LruCache](Data.Mutable.LruCache.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Mutable.LruCache: the bounded least-recently-used cache (folly
`EvictingCacheMap`).

A cache of fixed capacity: `lru-put` into a full cache evicts the entry
used least recently and hands it back, `lru-get` marks an entry used.
Every operation is O(1).

Layout. A `Data.Mutable.HashTable` maps each key to its node, and the
nodes are slots of three arrays made once at the capacity: the key, the
value, and two raw link words (the previous and next node) per node,
threaded into a doubly linked recency list from the most recently used
(the head) to the least (the tail). Unused nodes sit on a free list
threaded through the same next words. A hit therefore costs one table
lookup and four link writes, and no operation allocates after the cache
is made (beyond the table's own growth up to the capacity).

The invariant that makes every read sound: a node's key and value slots
are written exactly while it is on the recency list, and the table maps
exactly the listed keys to their nodes.

The trusted core is the private `cinch.runtime.slots*` imports (the
compiler's `slots.rs`); none of them leaves the module.

</details>

# Types and constructors

<a id="type-4c72754361636865"></a>

## `LruCache`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">LruCache</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>)</code></pre>

`meta`: size, capacity, head, tail, free head. `links`: node i's
previous at 2i, next at 2i + 1.

[Source](/reference-sources/Data.Mutable.LruCache.cinch) · line 44

# Values

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

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Mutable/LruCache.cinch`. Source SHA-256: `4ade3198bc4fd7a8686d5c6ffa38a61a80dfaa62153c15bd9543a4d10e5116de`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
