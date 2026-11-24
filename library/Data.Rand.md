# Data.Rand

Pseudorandom sources as values (PCG and ChaCha8) and their distributions: bounded integers, floats, permutations, ziggurat normal and exponential variates, and Zipf.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Rand`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Rand.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Array](Data.Array.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.ChaCha8](Data.ChaCha8.md), [Data.Float](Data.Float.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.Wide](Data.Wide.md).

Non-embedded source closure: [Data.Array](Data.Array.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.ChaCha8](Data.ChaCha8.md), [Data.Float](Data.Float.md), [Data.Rand](Data.Rand.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Pseudorandom generation over explicit sources: the PCG and ChaCha8 generators, and every distribution a
program needs from them (uniform integers in a range, floats in [0, 1), permutations and shuffles, standard
normal and exponential variates, and Zipf-distributed integers).

A source is any value that yields a 64-bit draw together with the source to draw from next (the Source
signature); PCG and ChaCha8 are instances. Every function takes a source and returns its result with the next
source, so generation is deterministic and replayable, and different parts of a program can hold independent
streams. Seed sources from Effect.Random when unpredictability matters; ChaCha8 is the one to use when the
outputs must also be hard to predict from earlier outputs.

PCG is the 128-bit linear congruential generator with the DXSM ("double xorshift multiply") output function:
one 128-bit multiply-add per draw and the reference generator's constants and encoding. Bounded integers use
Lemire's multiply-and-reject method (an unbiased result from one 64x64-bit product, redrawing only when the
low half lands in the biased sliver), with a mask for powers of two; results are those of a 64-bit host.
Normal and exponential variates use the Marsaglia-Tsang ziggurat with 128 and 256 strips; the tables are
constant lookup functions over the strip index. Zipf variates use rejection-inversion. Outputs match the
reference generators draw for draw.

Cost: each draw returns a new source value, and the current compiler boxes it (a single-field wrapper, or a
pair holding a 128-bit word, is a small heap object), so a draw costs one allocation: about 25 ns per raw
PCG draw on amd64, where the reference's mutable generator takes about 2 ns. The arithmetic itself is one
128-bit multiply-add and the DXSM fold.

</details>

# Types and constructors

<a id="type-506367"></a>

## `Pcg`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Pcg</span></code></pre>

The state, high word first.

[Source](/reference-sources/Data.Rand.cinch) · line 54

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Pcg</span> :: (<span class="tok-constructor">U128</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Pcg</span>)</code></pre>

<a id="type-5063674572726f72"></a>

## `PcgError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PcgError</span></code></pre>

Why a PCG encoding was refused.

[Source](/reference-sources/Data.Rand.cinch) · line 57

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PcgBadEncoding</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">PcgError</span></code></pre>

<a id="type-5a697066"></a>

## `Zipf`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Zipf</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Data.Rand.cinch) · line 338

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Zipf</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Zipf</span>))))))))</code></pre>

# Signatures and methods

<a id="class-536f75726365"></a>

## `Source`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Source</span> <span class="tok-variable">s</span></code></pre>

A source of uniformly distributed 64-bit values: a draw and the source to draw from next.

[Source](/reference-sources/Data.Rand.cinch) · line 47

<a id="method-536f757263652e736f757263652d75696e743634"></a>

## `Source.source-uint64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">source-uint64</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Source</span> <span class="tok-variable">s</span>) =&gt; (<span class="tok-variable">s</span> -&gt; (<span class="tok-constructor">U64</span>, <span class="tok-variable">s</span>))</code></pre>

The next uniform 64-bit value and the source after it.

# Values

<a id="value-7063672d6572726f722d6d657373616765"></a>

## `pcg-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pcg-error-message</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">PcgError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

A readable description of an error.

[Source](/reference-sources/Data.Rand.cinch) · line 62

<a id="value-7063672d6d61727368616c"></a>

## `pcg-marshal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pcg-marshal</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Pcg</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The state encoding: "pcg:" and the two halves big-endian (20 bytes).

[Source](/reference-sources/Data.Rand.cinch) · line 107

<a id="value-7063672d6e6577"></a>

## `pcg-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pcg-new</span> :: (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Pcg</span>))</code></pre>

A PCG generator from two seed words (the state's high and low halves).

[Source](/reference-sources/Data.Rand.cinch) · line 95

<a id="value-7063672d73656564"></a>

## `pcg-seed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pcg-seed</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Pcg</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Pcg</span>)))</code></pre>

The generator reseeded with two words (the same as a new generator).

[Source](/reference-sources/Data.Rand.cinch) · line 99

<a id="value-7063672d75696e743634"></a>

## `pcg-uint64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pcg-uint64</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Pcg</span> -&gt; (<span class="tok-constructor">U64</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Pcg</span>))</code></pre>

The next 64-bit value and generator.

[Source](/reference-sources/Data.Rand.cinch) · line 103

<a id="value-7063672d756e6d61727368616c"></a>

## `pcg-unmarshal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pcg-unmarshal</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">PcgError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Pcg</span>))</code></pre>

A generator from its encoding.

[Source](/reference-sources/Data.Rand.cinch) · line 121

<a id="value-72616e642d6578702d666c6f61743634"></a>

## `rand-exp-float64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rand-exp-float64</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Source</span> <span class="tok-variable">s</span>) =&gt; (<span class="tok-variable">s</span> -&gt; (<span class="tok-constructor">F64</span>, <span class="tok-variable">s</span>))</code></pre>

A standard exponential variate (rate 1).

[Source](/reference-sources/Data.Rand.cinch) · line 312

<a id="value-72616e642d666c6f61743332"></a>

## `rand-float32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rand-float32</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Source</span> <span class="tok-variable">s</span>) =&gt; (<span class="tok-variable">s</span> -&gt; (<span class="tok-constructor">F32</span>, <span class="tok-variable">s</span>))</code></pre>

A uniform binary32 value in [0, 1) with 24 random bits.

[Source](/reference-sources/Data.Rand.cinch) · line 200

<a id="value-72616e642d666c6f61743634"></a>

## `rand-float64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rand-float64</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Source</span> <span class="tok-variable">s</span>) =&gt; (<span class="tok-variable">s</span> -&gt; (<span class="tok-constructor">F64</span>, <span class="tok-variable">s</span>))</code></pre>

A uniform value in [0, 1) with 53 random bits.

[Source](/reference-sources/Data.Rand.cinch) · line 196

<a id="value-72616e642d696e743332"></a>

## `rand-int32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rand-int32</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Source</span> <span class="tok-variable">s</span>) =&gt; (<span class="tok-variable">s</span> -&gt; (<span class="tok-constructor">I32</span>, <span class="tok-variable">s</span>))</code></pre>

A uniform non-negative 32-bit integer (31 random bits).

[Source](/reference-sources/Data.Rand.cinch) · line 152

<a id="value-72616e642d696e7433322d6e"></a>

## `rand-int32-n`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rand-int32-n</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Source</span> <span class="tok-variable">s</span>) =&gt; (<span class="tok-variable">s</span> -&gt; (<span class="tok-constructor">I32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">I32</span>, <span class="tok-variable">s</span>))))</code></pre>

A uniform 32-bit integer in [0, n), or None when n is not positive.

[Source](/reference-sources/Data.Rand.cinch) · line 188

<a id="value-72616e642d696e743634"></a>

## `rand-int64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rand-int64</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Source</span> <span class="tok-variable">s</span>) =&gt; (<span class="tok-variable">s</span> -&gt; (<span class="tok-constructor">I64</span>, <span class="tok-variable">s</span>))</code></pre>

A uniform non-negative 64-bit integer (63 random bits).

[Source](/reference-sources/Data.Rand.cinch) · line 144

<a id="value-72616e642d696e7436342d6e"></a>

## `rand-int64-n`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rand-int64-n</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Source</span> <span class="tok-variable">s</span>) =&gt; (<span class="tok-variable">s</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">I64</span>, <span class="tok-variable">s</span>))))</code></pre>

A uniform integer in [0, n), or None when n is not positive.

[Source](/reference-sources/Data.Rand.cinch) · line 180

<a id="value-72616e642d6e6f726d2d666c6f61743634"></a>

## `rand-norm-float64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rand-norm-float64</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Source</span> <span class="tok-variable">s</span>) =&gt; (<span class="tok-variable">s</span> -&gt; (<span class="tok-constructor">F64</span>, <span class="tok-variable">s</span>))</code></pre>

A standard normal variate (mean 0, standard deviation 1).

[Source](/reference-sources/Data.Rand.cinch) · line 277

<a id="value-72616e642d7065726d"></a>

## `rand-perm`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rand-perm</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Source</span> <span class="tok-variable">s</span>) =&gt; (<span class="tok-variable">s</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">I64</span>), <span class="tok-variable">s</span>)))</code></pre>

A random permutation of 0 .. n-1.

[Source](/reference-sources/Data.Rand.cinch) · line 212

<a id="value-72616e642d73687566666c65"></a>

## `rand-shuffle`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rand-shuffle</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Source</span> <span class="tok-variable">s</span>) =&gt; (<span class="tok-variable">s</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>), <span class="tok-variable">s</span>)))</code></pre>

The list's elements in a uniformly random order (the same order rand-perm's indices give).

[Source](/reference-sources/Data.Rand.cinch) · line 216

<a id="value-72616e642d75696e743332"></a>

## `rand-uint32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rand-uint32</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Source</span> <span class="tok-variable">s</span>) =&gt; (<span class="tok-variable">s</span> -&gt; (<span class="tok-constructor">U32</span>, <span class="tok-variable">s</span>))</code></pre>

A uniform 32-bit value (the draw's high half).

[Source](/reference-sources/Data.Rand.cinch) · line 148

<a id="value-72616e642d75696e7433322d6e"></a>

## `rand-uint32-n`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rand-uint32-n</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Source</span> <span class="tok-variable">s</span>) =&gt; (<span class="tok-variable">s</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">U32</span>, <span class="tok-variable">s</span>))))</code></pre>

A uniform 32-bit value in [0, n), or None when n is 0.

[Source](/reference-sources/Data.Rand.cinch) · line 184

<a id="value-72616e642d75696e743634"></a>

## `rand-uint64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rand-uint64</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Source</span> <span class="tok-variable">s</span>) =&gt; (<span class="tok-variable">s</span> -&gt; (<span class="tok-constructor">U64</span>, <span class="tok-variable">s</span>))</code></pre>

A uniform 64-bit value.

[Source](/reference-sources/Data.Rand.cinch) · line 140

<a id="value-72616e642d75696e7436342d6e"></a>

## `rand-uint64-n`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rand-uint64-n</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Source</span> <span class="tok-variable">s</span>) =&gt; (<span class="tok-variable">s</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">U64</span>, <span class="tok-variable">s</span>))))</code></pre>

A uniform value in [0, n), or None when n is 0.

[Source](/reference-sources/Data.Rand.cinch) · line 176

<a id="value-7a6970662d6e6577"></a>

## `zipf-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zipf-new</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">U64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Zipf</span>))))</code></pre>

The distribution for s > 1 and v >= 1 (None otherwise) over [0, imax].

[Source](/reference-sources/Data.Rand.cinch) · line 349

<a id="value-7a6970662d75696e743634"></a>

## `zipf-uint64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zipf-uint64</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Source</span> <span class="tok-variable">s</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Zipf</span> -&gt; (<span class="tok-variable">s</span> -&gt; (<span class="tok-constructor">U64</span>, <span class="tok-variable">s</span>)))</code></pre>

A Zipf-distributed integer, by rejection-inversion.

[Source](/reference-sources/Data.Rand.cinch) · line 363

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Rand.Source_ChaCha8` | `Data.Rand.Source Data.ChaCha8.ChaCha8` | Implicit |
| `Data.Rand.Source_Pcg` | `Data.Rand.Source Data.Rand.Pcg` | Implicit |

**`Data.Rand.Source_ChaCha8`**: ChaCha8 as a source.
**`Data.Rand.Source_Pcg`**: PCG as a source.

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Data/Rand.cinch`. Source SHA-256: `0d7a056acd1c959a1d4ca2de348445c4a4951e04ee69fd5fb8c0e06ba4b84183`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
