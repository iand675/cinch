# Data.Mutable.BitSet

A dynamic bitset in 64-bit words: test/set/toggle, popcount, next-set-bit scanning, and word-at-a-time union, intersection and difference.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.BitSet`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.BitSet.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md).

Non-embedded source closure: [Data.Mutable.BitSet](Data.Mutable.BitSet.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Mutable.BitSet: the dynamic bitset (`boost::dynamic_bitset`,
C++ `std::vector<bool>` done as words).

A `BitSet r` stores bit i in bit i mod 64 of word i / 64 of one raw slot
array, so a test or a set is one load, one mask and (for a set) one
store, and the whole-set operations run a word at a time: `bitset-count`
is a popcount per word, the unions and intersections are one `or`/`and`
per word, and `bitset-next-set` skips 64 clear bits per step with a
trailing-zero count.

The set grows on demand: setting a bit past the current words doubles
the word count until it fits (a move of raw words). Reading a bit past
the words is just `False`, so there is no "length" a reader can step
off; `bitset-capacity` reports the bits currently backed.

The trusted core is the private `cinch.runtime.slots*` imports (the
compiler's `slots.rs`); none of them leaves the module. Raw words have
no holes: every word of the array is written (zero) when it is made.

</details>

# Types and constructors

<a id="type-426974536574"></a>

## `BitSet`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">BitSet</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

One cell holding the current words.

[Source](/reference-sources/Data.Mutable.BitSet.cinch) · line 38

# Values

<a id="value-6269747365742d6361706163697479"></a>

## `bitset-capacity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-capacity</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BitSet.cinch)

<a id="value-6269747365742d636c656172"></a>

## `bitset-clear`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-clear</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.BitSet.cinch)

<a id="value-6269747365742d636c6561722d616c6c"></a>

## `bitset-clear-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-clear-all</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BitSet.cinch)

<a id="value-6269747365742d636f756e74"></a>

## `bitset-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-count</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

The number of set bits: one popcount per word.

[Source](/reference-sources/Data.Mutable.BitSet.cinch) · line 128

<a id="value-6269747365742d646966666572656e63652d77697468"></a>

## `bitset-difference-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-difference-with</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

`a` becomes a & ~b.

[Source](/reference-sources/Data.Mutable.BitSet.cinch) · line 235

<a id="value-6269747365742d66726f6d2d6c697374"></a>

## `bitset-from-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-from-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BitSet.cinch)

<a id="value-6269747365742d696e746572736563742d77697468"></a>

## `bitset-intersect-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-intersect-with</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

`a` becomes a & b.

[Source](/reference-sources/Data.Mutable.BitSet.cinch) · line 220

<a id="value-6269747365742d69732d656d707479"></a>

## `bitset-is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-is-empty</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.BitSet.cinch)

<a id="value-6269747365742d6e6577"></a>

## `bitset-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span>)</code></pre>

An empty set with `n` bits backed before its first growth.

[Source](/reference-sources/Data.Mutable.BitSet.cinch) · line 83

<a id="value-6269747365742d6e6578742d736574"></a>

## `bitset-next-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-next-set</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>))</code></pre>

The least set bit at or above `i`.

[Source](/reference-sources/Data.Mutable.BitSet.cinch) · line 146

<a id="value-6269747365742d736574"></a>

## `bitset-set`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-set</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.BitSet.cinch)

<a id="value-6269747365742d74657374"></a>

## `bitset-test`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-test</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.BitSet.cinch)

<a id="value-6269747365742d746f2d6c697374"></a>

## `bitset-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-to-list</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>)</code></pre>

The set bits, ascending.

[Source](/reference-sources/Data.Mutable.BitSet.cinch) · line 165

<a id="value-6269747365742d746f67676c65"></a>

## `bitset-toggle`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-toggle</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.BitSet.cinch)

<a id="value-6269747365742d756e696f6e2d77697468"></a>

## `bitset-union-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bitset-union-with</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">BitSet</span>.<span class="tok-constructor">BitSet</span> <span class="tok-variable">r</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

`a` becomes a | b (growing to b's words).

[Source](/reference-sources/Data.Mutable.BitSet.cinch) · line 205

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Mutable/BitSet.cinch`. Source SHA-256: `1ce604a52fb81e9b084c070dcd4370a4eb9b161b70cb723a38b45be31264b5cc`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
