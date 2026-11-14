# Data.Mutable.BTreeSet

An ordered mutable set: the pooled B-tree with a Unit value per member.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.BTreeSet`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.BTreeSet.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Mutable.BTree](Data.Mutable.BTree.md), [Data.Option](Data.Option.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Mutable.BTree](Data.Mutable.BTree.md), [Data.Mutable.BTreeSet](Data.Mutable.BTreeSet.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Mutable.BTreeSet: the ordered mutable set (abseil `btree_set`).

A `BTreeSet r a` is a `Data.Mutable.BTree r a Unit`: the same pooled
degree-8 B-tree, with a `Unit` value per member (a raw word nothing
counts). Members come out in ascending order; every cost and boundary is
the tree's (see that module's header).

</details>

# Types and constructors

<a id="type-4254726565536574"></a>

## `BTreeSet`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BTreeSet</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

# Values

<a id="value-62747265652d7365742d6365696c696e67"></a>

## `btree-set-ceiling`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-ceiling</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

The least member not below `x`.

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch) · line 74

<a id="value-62747265652d7365742d636c656172"></a>

## `btree-set-clear`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-clear</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d64656c657465"></a>

## `btree-set-delete`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-delete</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Remove `x`: `True` when it was a member.

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch) · line 52

<a id="value-62747265652d7365742d666c6f6f72"></a>

## `btree-set-floor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-floor</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

The greatest member not above `x`.

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch) · line 70

<a id="value-62747265652d7365742d666f6c64"></a>

## `btree-set-fold`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-fold</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>)) -&gt; (<span class="tok-variable">b</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">b</span>)))</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d666f6c642d72616e6765"></a>

## `btree-set-fold-range`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-fold-range</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>)) -&gt; (<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">b</span>)))))</code></pre>

A left fold over the members in [lo, hi), ascending.

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch) · line 81

<a id="value-62747265652d7365742d66726f6d2d6c697374"></a>

## `btree-set-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-from-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d696e73657274"></a>

## `btree-set-insert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-insert</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>))</code></pre>

Add `x`: `True` when it was not already a member.

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch) · line 41

<a id="value-62747265652d7365742d69732d656d707479"></a>

## `btree-set-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-is-empty</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d6d6178"></a>

## `btree-set-max`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-max</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d6d656d626572"></a>

## `btree-set-member`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-member</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d6d696e"></a>

## `btree-set-min`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-min</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d6e6577"></a>

## `btree-set-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d706f702d6d6178"></a>

## `btree-set-pop-max`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-pop-max</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d706f702d6d696e"></a>

## `btree-set-pop-min`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-pop-min</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d73697a65"></a>

## `btree-set-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-size</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d746f2d6c697374"></a>

## `btree-set-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-to-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

<a id="value-62747265652d7365742d766572696679"></a>

## `btree-set-verify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">btree-set-verify</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">o</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BTreeSet</span>.<span class="tok-constructor">BTreeSet</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BTreeSet.cinch)

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Mutable/BTreeSet.cinch`. Source SHA-256: `95b4465c365cead885ece4adbe28d10605df080e6a4903123093775f696b0d04`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
