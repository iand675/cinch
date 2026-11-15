# Data.Mutable.Array

A fixed-length mutable array whose slots always hold values, with copying freeze and thaw against the immutable Array.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.Array`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.Array.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md).

Non-embedded source closure: [Data.Mutable.Array](Data.Mutable.Array.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Mutable.Array: the fixed-length mutable array (C++ `std::array`
held by reference, Haskell's `MutableArray`).

A `MutArray r a` has the length it was made with and every slot always
holds a value: `mut-array-new n x` fills, `mut-array-from-list` and
`mut-array-thaw` copy in. Reads and writes are O(1) index loads and
stores; a write releases the value it replaces.

It is the mutable twin of `Data.Array`: `mut-array-freeze` copies out an
immutable `Array a` (a fresh block, so no later write reaches it), and
`mut-array-thaw` copies an `Array a` in. For a sequence that changes
length, use `Data.Mutable.Vec`.

The trusted core is the private `cinch.runtime.slots*` imports (the
compiler's `slots.rs`); none of them leaves the module. Every slot is
written before the array is returned, so no read meets an empty slot.

</details>

# Types and constructors

<a id="type-4d75744172726179"></a>

## `MutArray`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MutArray</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Array.cinch)

# Values

<a id="value-6d75742d61727261792d66696c6c"></a>

## `mut-array-fill`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-fill</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Every element becomes `x`.

[Source](/reference-sources/Data.Mutable.Array.cinch) · line 143

<a id="value-6d75742d61727261792d666f6c64"></a>

## `mut-array-fold`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-fold</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">r</span> . ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>)) -&gt; (<span class="tok-variable">b</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">b</span>)))</code></pre>

[Source](/reference-sources/Data.Mutable.Array.cinch)

<a id="value-6d75742d61727261792d667265657a65"></a>

## `mut-array-freeze`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-freeze</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Array</span> <span class="tok-variable">a</span>)</code></pre>

An immutable copy (a fresh block).

[Source](/reference-sources/Data.Mutable.Array.cinch) · line 170

<a id="value-6d75742d61727261792d66726f6d2d6c697374"></a>

## `mut-array-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-from-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Array.cinch)

<a id="value-6d75742d61727261792d676574"></a>

## `mut-array-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-get</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.Array.cinch)

<a id="value-6d75742d61727261792d6c656e677468"></a>

## `mut-array-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-length</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Array.cinch)

<a id="value-6d75742d61727261792d6d6f64696679"></a>

## `mut-array-modify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-modify</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

Rewrite element `i` through `f`, moving it out and back.

[Source](/reference-sources/Data.Mutable.Array.cinch) · line 116

<a id="value-6d75742d61727261792d6e6577"></a>

## `mut-array-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))</code></pre>

`n` copies of `x`.

[Source](/reference-sources/Data.Mutable.Array.cinch) · line 51

<a id="value-6d75742d61727261792d736574"></a>

## `mut-array-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-set</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

Replace element `i`: `False` (and nothing written) when out of range.

[Source](/reference-sources/Data.Mutable.Array.cinch) · line 107

<a id="value-6d75742d61727261792d73776170"></a>

## `mut-array-swap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-swap</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)))</code></pre>

[Source](/reference-sources/Data.Mutable.Array.cinch)

<a id="value-6d75742d61727261792d746162756c617465"></a>

## `mut-array-tabulate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-tabulate</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Nat</span> -&gt; <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))</code></pre>

Element i is `f i`.

[Source](/reference-sources/Data.Mutable.Array.cinch) · line 65

<a id="value-6d75742d61727261792d74686177"></a>

## `mut-array-thaw`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-thaw</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Array</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

A mutable copy of an immutable array.

[Source](/reference-sources/Data.Mutable.Array.cinch) · line 179

<a id="value-6d75742d61727261792d746f2d6c697374"></a>

## `mut-array-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mut-array-to-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Array</span>.<span class="tok-constructor">MutArray</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Array.cinch)

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Mutable/Array.cinch`. Source SHA-256: `94e1495897c24970b02075aeb56bb1181bf54735059dd1828679581c3772490e`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
