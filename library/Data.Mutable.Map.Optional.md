# Data.Mutable.Map.Optional

Explicit absent-map adapters without hidden null handles.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.Map.Optional`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.Map.Optional.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Core](Data.Core.md), [Data.Hash](Data.Hash.md), [Data.Mutable.Map](Data.Mutable.Map.md), [Data.Mutable.Var](Data.Mutable.Var.md), [Relation.Eq](Relation.Eq.md).

Non-embedded source closure: [Data.Mutable.HashTable](Data.Mutable.HashTable.md), [Data.Mutable.LinkedList](Data.Mutable.LinkedList.md), [Data.Mutable.Map](Data.Mutable.Map.md), [Data.Mutable.Map.Optional](Data.Mutable.Map.Optional.md), [Data.Mutable.Var](Data.Mutable.Var.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Explicit absent-map adapters without hidden null handles.
Empty and absent inputs compare equally and yield no entries. Cloning
preserves absence; assigning an actual binding to an absent destination
returns MapMissingDestination instead of allocating it implicitly.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-6f7074696f6e616c2d6d61702d616c6c"></a>

## `optional-map-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">optional-map-all</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">ef</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt; ((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Visit no entries for an absent input, or apply ordinary live pair traversal.

[Source](/reference-sources/Data.Mutable.Map.Optional.cinch) · line 27

<a id="value-6f7074696f6e616c2d6d61702d636c6f6e65"></a>

## `optional-map-clone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">optional-map-clone</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>))))</code></pre>

Preserve absence, or shallowly clone into independent present container cells.

[Source](/reference-sources/Data.Mutable.Map.Optional.cinch) · line 69

<a id="value-6f7074696f6e616c2d6d61702d636f6c6c656374"></a>

## `optional-map-collect`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">optional-map-collect</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">ef</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">h</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Hash</span>.<span class="tok-constructor">Hash</span> <span class="tok-variable">k</span>, <span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Unit</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)))</code></pre>

Collect with captured default key methods, returning an allocated present empty map when needed.

[Source](/reference-sources/Data.Mutable.Map.Optional.cinch) · line 81

<a id="value-6f7074696f6e616c2d6d61702d636f6c6c6563742d77697468"></a>

## `optional-map-collect-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">optional-map-collect-with</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">ef</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapKeyPolicy</span> <span class="tok-variable">k</span> -&gt; (((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Unit</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>))))</code></pre>

Collect with an explicit policy, returning a present map even for an empty source.

[Source](/reference-sources/Data.Mutable.Map.Optional.cinch) · line 75

<a id="value-6f7074696f6e616c2d6d61702d636f7079"></a>

## `optional-map-copy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">optional-map-copy</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">s</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">s</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">s</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapError</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Copy into the supplied destination; absence is unchanged for an empty source.

[Source](/reference-sources/Data.Mutable.Map.Optional.cinch) · line 61

<a id="value-6f7074696f6e616c2d6d61702d64656c6574652d6966"></a>

## `optional-map-delete-if`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">optional-map-delete-if</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">ef</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt; ((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Apply the deletion predicate only to present live entries; absence is successful and inert.

[Source](/reference-sources/Data.Mutable.Map.Optional.cinch) · line 85

<a id="value-6f7074696f6e616c2d6d61702d657175616c"></a>

## `optional-map-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">optional-map-equal</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">s</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">values</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">v</span>) =&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">s</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">s</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Compare bindings with value equality, identifying absence with a present empty map.

[Source](/reference-sources/Data.Mutable.Map.Optional.cinch) · line 95

<a id="value-6f7074696f6e616c2d6d61702d657175616c2d6279"></a>

## `optional-map-equal-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">optional-map-equal-by</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">ef</span> <span class="tok-variable">r</span> <span class="tok-variable">s</span> . ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span> -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">s</span> <span class="tok-variable">k</span> <span class="tok-variable">b</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>, <span class="tok-constructor">Mut</span> <span class="tok-variable">s</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

Compare absence as empty and use an explicit predicate for corresponding present values.

[Source](/reference-sources/Data.Mutable.Map.Optional.cinch) · line 89

<a id="value-6f7074696f6e616c2d6d61702d696e73657274"></a>

## `optional-map-insert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">optional-map-insert</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">ef</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt; (((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Unit</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MapError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Consume an empty source successfully for an absent destination; refuse its first actual binding.

[Source](/reference-sources/Data.Mutable.Map.Optional.cinch) · line 50

<a id="value-6f7074696f6e616c2d6d61702d6b657973"></a>

## `optional-map-keys`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">optional-map-keys</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">ef</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt; ((<span class="tok-variable">k</span> -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Visit current keys, with an absent input exhausting without any callback.

[Source](/reference-sources/Data.Mutable.Map.Optional.cinch) · line 31

<a id="value-6f7074696f6e616c2d6d61702d736f75726365"></a>

## `optional-map-source`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">optional-map-source</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">ef</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt; ((<span class="tok-variable">k</span> -&gt; (<span class="tok-variable">v</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>)) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Supply live pairs to a stop-obeying consumer; absence supplies none.

[Source](/reference-sources/Data.Mutable.Map.Optional.cinch) · line 39

<a id="value-6f7074696f6e616c2d6d61702d76616c756573"></a>

## `optional-map-values`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">optional-map-values</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-variable">ef</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Map</span>.<span class="tok-constructor">MutableMap</span> <span class="tok-variable">r</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) -&gt; ((<span class="tok-variable">v</span> -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span> | <span class="tok-variable">ef</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Visit current values, preserving duplicates and treating absence as empty.

[Source](/reference-sources/Data.Mutable.Map.Optional.cinch) · line 35

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Mutable/Map/Optional.cinch`. Source SHA-256: `7e9b280b6e9d7b0a545479f0ff6c705ecac5ce477608bda3b0c04e644c81620a`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
