# Data.Search

Indexed binary search with monotone predicates, insertion positions and effectful three-way comparisons.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Search`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Search.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Nat](Data.Nat.md), [Data.Ordering](Data.Ordering.md).

Non-embedded source closure: [Data.Search](Data.Search.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Binary search over natural-number indices. Callbacks may compute, read or
compare elements on demand. Only indices below the supplied size are probed.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-7365617263682d66696e64"></a>

## `search-find`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">search-find</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>) -&gt;{<span class="tok-variable">e</span>} (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Bool</span>)))</code></pre>

Find the first equal element or its insertion index. The callback compares
the sought key to the indexed element, returning GT, then EQ, then LT across
an ascending sequence. Empty ranges perform no comparisons.

[Source](/reference-sources/Data.Search.cinch) · line 25

<a id="value-7365617263682d6669727374"></a>

## `search-first`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">search-first</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> . (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Bool</span>) -&gt;{<span class="tok-variable">e</span>} <span class="tok-constructor">Nat</span>))</code></pre>

Return the first index whose predicate is true, or the size when none
qualifies. The predicate must be false on a prefix and true thereafter.

[Source](/reference-sources/Data.Search.cinch) · line 12

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Search.cinch`. Source SHA-256: `50f1ab5bcb33be8cd461431948d00652353604ba9deafa2e498777fd9560e00c`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
