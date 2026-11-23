# Data.Array.Edit

Checked immutable array range edits, predicate deletion, adjacent compaction, reversal and concatenation.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Array.Edit`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Array.Edit.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Array](Data.Array.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Relation.Eq](Relation.Eq.md).

Non-embedded source closure: [Data.Array](Data.Array.md), [Data.Array.Edit](Data.Array.Edit.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Persistent edits over immutable arrays. Half-open ranges are checked
before constructing a result, and the original arrays remain usable.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-61727261792d636f6d70616374"></a>

## `array-compact`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-compact</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">equal</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">a</span>) =&gt; ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>))</code></pre>

Remove adjacent duplicate values using element equality.

[Source](/reference-sources/Data.Array.Edit.cinch) · line 57

<a id="value-61727261792d636f6d706163742d6279"></a>

## `array-compact-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-compact-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">e</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Array</span> <span class="tok-variable">a</span>))</code></pre>

Keep the first element of each adjacent run. Compare each current
element with its immediate predecessor, passing the current element first.

[Source](/reference-sources/Data.Array.Edit.cinch) · line 45

<a id="value-61727261792d636f6e636174"></a>

## `array-concat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-concat</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>)) -&gt; (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>))</code></pre>

Concatenate arrays in order, walking each input once. The temporary list
keeps earlier prefixes from being copied for every subsequent input.

[Source](/reference-sources/Data.Array.Edit.cinch) · line 66

<a id="value-61727261792d64656c6574652d6279"></a>

## `array-delete-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-delete-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">e</span> . ((<span class="tok-variable">a</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>) -&gt; (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Array</span> <span class="tok-variable">a</span>))</code></pre>

Delete each matching element. Run the predicate once per element from left to right.

[Source](/reference-sources/Data.Array.Edit.cinch) · line 34

<a id="value-61727261792d64656c6574652d72616e6765"></a>

## `array-delete-range`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-delete-range</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>)))))</code></pre>

Delete exactly the elements in the half-open range.

[Source](/reference-sources/Data.Array.Edit.cinch) · line 30

<a id="value-61727261792d696e73657274"></a>

## `array-insert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-insert</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>)))))</code></pre>

Insert all new elements at an index in the closed range from zero to length.

[Source](/reference-sources/Data.Array.Edit.cinch) · line 26

<a id="value-61727261792d7265706c6163652d72616e6765"></a>

## `array-replace-range`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-replace-range</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>))))))</code></pre>

Replace [start,end) with the replacement's elements. Both bounds may
equal the array length; reversed or out-of-range bounds return None.

[Source](/reference-sources/Data.Array.Edit.cinch) · line 15

<a id="value-61727261792d72657665727365"></a>

## `array-reverse`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-reverse</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>))</code></pre>

Return the elements in reverse order.

[Source](/reference-sources/Data.Array.Edit.cinch) · line 61

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Array/Edit.cinch`. Source SHA-256: `25857a7cd5dff21d743ddb2871c837c770397ac57e05d524349fc39ccc780bdb`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
