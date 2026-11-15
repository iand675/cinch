# Data.Mutable.UnionFind

Disjoint sets with union by size and path halving: near-constant find, union and same-set queries, and appendable elements.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.UnionFind`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.UnionFind.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md).

Non-embedded source closure: [Data.Mutable.UnionFind](Data.Mutable.UnionFind.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Mutable.UnionFind: disjoint sets (union-find), as in every
compiler's type unifier and every Kruskal.

Elements are the Nats 0 .. n - 1, `union-find-add` appends one more.
Each element keeps a parent word and each root its set's size, both in
raw slot arrays. `union-find-find` halves paths as it walks (every node
on the way is pointed at its grandparent), and `union-find-union` hangs
the smaller set under the larger, which together make a sequence of m
operations cost O(m alpha(n)), effectively constant per operation.

The trusted core is the private `cinch.runtime.slots*` imports (the
compiler's `slots.rs`); none of them leaves the module. Every word below
the element count is written when its element is added.

</details>

# Types and constructors

<a id="type-556e696f6e46696e64"></a>

## `UnionFind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UnionFind</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The header (element count, set count) and one cell holding the
words: parent of element i at 2i, size of the set rooted at i at
2i + 1.

[Source](/reference-sources/Data.Mutable.UnionFind.cinch) · line 33

# Values

<a id="value-756e696f6e2d66696e642d616464"></a>

## `union-find-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">union-find-add</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">UnionFind</span>.<span class="tok-constructor">UnionFind</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

A new singleton set: its element.

[Source](/reference-sources/Data.Mutable.UnionFind.cinch) · line 75

<a id="value-756e696f6e2d66696e642d66696e64"></a>

## `union-find-find`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">union-find-find</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">UnionFind</span>.<span class="tok-constructor">UnionFind</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>))</code></pre>

The representative of x's set (`None` when x is not an element).

[Source](/reference-sources/Data.Mutable.UnionFind.cinch) · line 110

<a id="value-756e696f6e2d66696e642d6c656e677468"></a>

## `union-find-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">union-find-length</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">UnionFind</span>.<span class="tok-constructor">UnionFind</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.UnionFind.cinch)

<a id="value-756e696f6e2d66696e642d6e6577"></a>

## `union-find-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">union-find-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">UnionFind</span>.<span class="tok-constructor">UnionFind</span> <span class="tok-variable">r</span>)</code></pre>

`n` singleton sets.

[Source](/reference-sources/Data.Mutable.UnionFind.cinch) · line 60

<a id="value-756e696f6e2d66696e642d73616d65"></a>

## `union-find-same`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">union-find-same</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">UnionFind</span>.<span class="tok-constructor">UnionFind</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

[Source](/reference-sources/Data.Mutable.UnionFind.cinch)

<a id="value-756e696f6e2d66696e642d7365742d636f756e74"></a>

## `union-find-set-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">union-find-set-count</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">UnionFind</span>.<span class="tok-constructor">UnionFind</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

The number of disjoint sets.

[Source](/reference-sources/Data.Mutable.UnionFind.cinch) · line 105

<a id="value-756e696f6e2d66696e642d7365742d73697a65"></a>

## `union-find-set-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">union-find-set-size</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">UnionFind</span>.<span class="tok-constructor">UnionFind</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>))</code></pre>

The size of x's set (0 when x is not an element).

[Source](/reference-sources/Data.Mutable.UnionFind.cinch) · line 165

<a id="value-756e696f6e2d66696e642d756e696f6e"></a>

## `union-find-union`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">union-find-union</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">UnionFind</span>.<span class="tok-constructor">UnionFind</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

Merge the sets of x and y: `True` when they were separate (and both
are elements).

[Source](/reference-sources/Data.Mutable.UnionFind.cinch) · line 130

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Mutable/UnionFind.cinch`. Source SHA-256: `727e954bd65cccc23e096583f2e09110a598ca323a6a4170d7d3270a230afe1b`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
