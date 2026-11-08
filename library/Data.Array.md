# Data.Array

Immutable contiguous arrays with optional failed reads, updates, slices, and ranges, plus list conversions.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Array`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Array.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.FromList](Data.FromList.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Array](Data.Array.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Immutable contiguous arrays. Failed reads/updates/ranges return None.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-61727261792d617070656e64"></a>

## `array-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-append</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>)))</code></pre>

[Source](/reference-sources/Data.Array.cinch)

<a id="value-61727261792d66726f6d2d6c697374"></a>

## `array-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-from-list</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Data.Array.cinch)

<a id="value-61727261792d676574"></a>

## `array-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-get</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)))</code></pre>

[Source](/reference-sources/Data.Array.cinch)

<a id="value-61727261792d6c656e677468"></a>

## `array-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-length</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Array.cinch)

<a id="value-61727261792d7265706c6963617465"></a>

## `array-replicate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-replicate</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>)))</code></pre>

[Source](/reference-sources/Data.Array.cinch)

<a id="value-61727261792d736574"></a>

## `array-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-set</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>)))))</code></pre>

[Source](/reference-sources/Data.Array.cinch)

<a id="value-61727261792d736c696365"></a>

## `array-slice`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-slice</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>)))))</code></pre>

[Source](/reference-sources/Data.Array.cinch)

<a id="value-61727261792d746162756c617465"></a>

## `array-tabulate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-tabulate</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Nat</span> -&gt; <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>)))</code></pre>

[Source](/reference-sources/Data.Array.cinch)

<a id="value-61727261792d746f2d6c697374"></a>

## `array-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">array-to-list</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Array</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Data.Array.cinch)

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Array.FromList_Array` | `Data.FromList.FromList (Array a) a` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Array.cinch`. Source SHA-256: `3c90379a58326e1c86b3b5c0a6fbdbbb95374acc26ab0adca138633f7caa87ee`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
