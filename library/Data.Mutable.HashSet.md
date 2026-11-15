# Data.Mutable.HashSet

A mutable hash set: the HashTable with a Unit value per member.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.HashSet`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.HashSet.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Hash](Data.Hash.md), [Data.Mutable.HashTable](Data.Mutable.HashTable.md), [Data.Option](Data.Option.md), [Relation.Eq](Relation.Eq.md).

Non-embedded source closure: [Data.Mutable.HashSet](Data.Mutable.HashSet.md), [Data.Mutable.HashTable](Data.Mutable.HashTable.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Mutable.HashSet: the mutable hash set (abseil `flat_hash_set`).

A `HashSet r a` is a `Data.Mutable.HashTable r a Unit`: the same
chunked, tagged, tombstone-free table, with a `Unit` value per entry (a
raw word nothing counts). Every cost and boundary is the table's; see
that module's header.

</details>

# Types and constructors

<a id="type-48617368536574"></a>

## `HashSet`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HashSet</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.HashSet.cinch)

# Values

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

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Mutable/HashSet.cinch`. Source SHA-256: `dd418d95ba3d0f8453ee803fdd25bbd0ed2fbb4dcdca5669b37fd5acfe477977`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
