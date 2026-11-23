# Data.Array.Search

Immutable array comparison, indexed lookups, binary search and sortedness with explicit callback policies.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Array.Search`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Array.Search.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Array](Data.Array.md), [Data.Compare](Data.Compare.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Search](Data.Search.md), [Relation.Eq](Relation.Eq.md).

Non-embedded source closure: [Data.Array](Data.Array.md), [Data.Array.Search](Data.Array.Search.md), [Data.Compare](Data.Compare.md), [Data.Search](Data.Search.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Comparison and lookup on immutable arrays. Linear searches stop at the
first match. Binary searches return the first equivalent element or its
insertion position. Callback variants preserve the caller's effects.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-61727261792d62696e6172792d736561726368"></a>

## `array-binary-search`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-binary-search</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">ordered</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Compare</span>.<span class="tok-constructor">Compare</span> <span class="tok-variable">a</span>) =&gt; ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Bool</span>)))</code></pre>

Find the first equivalent value or its insertion position using Compare.

[Source](/reference-sources/Data.Array.Search.cinch) · line 78

<a id="value-61727261792d62696e6172792d7365617263682d6279"></a>

## `array-binary-search-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-binary-search-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">e</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>)) -&gt; (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span> -&gt;{<span class="tok-variable">e</span>} (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Bool</span>))))</code></pre>

Find a key in ascending order. The callback compares an array element
with the key, allowing a different key type. Duplicate keys return the first index.

[Source](/reference-sources/Data.Array.Search.cinch) · line 72

<a id="value-61727261792d636f6d70617265"></a>

## `array-compare`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-compare</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">ordered</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Compare</span>.<span class="tok-constructor">Compare</span> <span class="tok-variable">a</span>) =&gt; ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>))</code></pre>

Lexicographic comparison using the element Compare instance.

[Source](/reference-sources/Data.Array.Search.cinch) · line 67

<a id="value-61727261792d636f6d706172652d6279"></a>

## `array-compare-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-compare-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">e</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>)) -&gt; (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Array</span> <span class="tok-variable">b</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>)))</code></pre>

Lexicographically compare arrays, putting a proper prefix before the longer array.

[Source](/reference-sources/Data.Array.Search.cinch) · line 54

<a id="value-61727261792d636f6e7461696e73"></a>

## `array-contains`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-contains</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">equal</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">a</span>) =&gt; ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Whether an element equals the requested value.

[Source](/reference-sources/Data.Array.Search.cinch) · line 35

<a id="value-61727261792d636f6e7461696e732d6279"></a>

## `array-contains-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-contains-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">e</span> . ((<span class="tok-variable">a</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>) -&gt; (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Whether an element satisfies the predicate.

[Source](/reference-sources/Data.Array.Search.cinch) · line 31

<a id="value-61727261792d657175616c"></a>

## `array-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-equal</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">equal</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">a</span>) =&gt; ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Compare corresponding elements using their equality instance.

[Source](/reference-sources/Data.Array.Search.cinch) · line 50

<a id="value-61727261792d657175616c2d6279"></a>

## `array-equal-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-equal-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">e</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Array</span> <span class="tok-variable">b</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

Compare lengths before calling the element equality predicate. An unequal
pair ends comparison immediately. The two element types may differ.

[Source](/reference-sources/Data.Array.Search.cinch) · line 40

<a id="value-61727261792d696e646578"></a>

## `array-index`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-index</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">equal</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">a</span>) =&gt; ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Locate the first element equal to the requested value.

[Source](/reference-sources/Data.Array.Search.cinch) · line 27

<a id="value-61727261792d696e6465782d6279"></a>

## `array-index-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-index-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">e</span> . ((<span class="tok-variable">a</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>) -&gt; (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>))</code></pre>

Locate the first element satisfying a predicate, visiting elements in order.

[Source](/reference-sources/Data.Array.Search.cinch) · line 18

<a id="value-61727261792d69732d736f72746564"></a>

## `array-is-sorted`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-is-sorted</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">ordered</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Compare</span>.<span class="tok-constructor">Compare</span> <span class="tok-variable">a</span>) =&gt; ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Test nondecreasing order using Compare, including its floating NaN policy.

[Source](/reference-sources/Data.Array.Search.cinch) · line 95

<a id="value-61727261792d69732d736f727465642d6279"></a>

## `array-is-sorted-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-is-sorted-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">e</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>)) -&gt; (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Test whether the array is in nondecreasing order, stopping at the first inversion.

[Source](/reference-sources/Data.Array.Search.cinch) · line 82

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Array/Search.cinch`. Source SHA-256: `a10366c85b000d05aa12b145ed82a50b461557ac57098b9b052b2d03f5c7f305`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
