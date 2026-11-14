# Data.Mutable.HashTable

A mutable hash table after SwissTable and F14: 8-slot chunks with 7-bit tags matched eight at a time, a 64-bit finalizer over the Hash digest, and overflow counts that make deletion tombstone-free.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.HashTable`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.HashTable.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Hash](Data.Hash.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Relation.Eq](Relation.Eq.md).

Non-embedded source closure: [Data.Mutable.HashTable](Data.Mutable.HashTable.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Mutable.HashTable: the mutable hash table, after Google's
SwissTable (abseil `flat_hash_map`) and Facebook's F14 (folly
`F14ValueMap`).

Layout. Slots come in chunks of eight. Each chunk has one 64-bit
control word, one byte per slot: 0 for an empty slot, and for a full
one a 7-bit tag of the key's hash with the high bit set (`0x80 | t`).
Keys and values live in two parallel slot arrays, so a probe touches
the control word first and a key only when its tag matched. The table
holds at most 7 entries per chunk (a 7/8 maximum load), and the chunk
count is a power of two.

Probing (SwissTable's group match, done in SWAR). The control word is
matched against a key's tag eight slots at a time with the classic
zero-byte trick: `x = ctrl ^ (tag * 0x0101..01)` has a zero byte
exactly where the tag matches, and `(x - 0x0101..01) & ~x & 0x8080..80`
lights those bytes (a rare false positive is settled by `Eq`, never a
false negative). Empty bytes are `~ctrl & 0x8080..80`. A hit is the
trailing-zero count over eight. A miss moves to the next chunk by a
per-key odd stride (F14's double hashing, `2 * tag + 1`), which visits
every chunk of a power-of-two table.

Deletion without tombstones (F14's overflow counts). Every chunk keeps
a count of the entries whose probe passed it because it was full. A
lookup stops at the first chunk with no match and a zero count; a
delete retraces its key's probe from the home chunk and decrements the
count of every chunk it passed. So a delete-heavy workload never fills
the table with tombstones, never needs a same-size rehash, and lookups
of absent keys stay short.

Hashing. `Hash` gives a `Nat` digest, and many digests are weak (the
`Hash Nat` instance is the identity), so the low 64 bits are passed
through a 64-bit finalizer (two xor-shift-multiply rounds) before the
table uses them: the tag is the top 7 bits, the home chunk the low bits
masked by the chunk count.

Costs: lookup, insert and delete are expected O(1); growth doubles the
chunk count and moves every entry once (no reference count touched,
only the digest recomputed), so n inserts cost O(n) amortised.

The invariant that makes every read sound: a slot is read only when its
control byte says it is full, a slot is written before its byte is set,
and a byte is cleared before its slot is taken. Growth clears the old
table's control words as it empties them.

Named refusals:
  iteration order: entries are listed in slot order, which depends on
    the digests and the growth history; nothing may rely on it (the
    ordered views are Data.Map's);
  iteration under mutation: `hash-table-fold` reads a snapshot through a
    pure function, and `hash-table-for-each` runs its action over the
    entries listed when it started, so an action that changes the table
    never meets a moved-out slot;
  adversarial digests: the finalizer spreads weak digests, but a `Hash`
    instance that maps many keys to one digest still puts them in one
    probe sequence (correctness holds; only probe length degrades).

The trusted core is the private `cinch.runtime.slots*` imports (the
compiler's `slots.rs`); none of them leaves the module.

</details>

# Types and constructors

<a id="type-486173685461626c65"></a>

## `HashTable`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HashTable</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The table's handle: one cell holding the current generation,
replaced when the table grows.

[Source](/reference-sources/Data.Mutable.HashTable.cinch) · line 97

# Values

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

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Mutable/HashTable.cinch`. Source SHA-256: `ea2ddd0f2f15f5a527f7cfa5b97a12eec135ae4fb34d373f774ad79438fe3a98`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
