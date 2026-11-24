# Effect.Test.Quick

Deterministic property-based testing engine with Arbitrary instances over Data.Rand.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Test.Quick`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Test.Quick.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Float](Data.Float.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Rand](Data.Rand.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md).

Non-embedded source closure: [Data.Array](Data.Array.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.ChaCha8](Data.ChaCha8.md), [Data.Float](Data.Float.md), [Data.Rand](Data.Rand.md), [Effect.Test.Quick](Effect.Test.Quick.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Deterministic property-based testing over Data.Rand's PCG generator.
Provides the Arbitrary typeclass with instances for primitive scalars,
text, byte buffers, optional values, lists, pairs, and triples, plus
configurable property and function-equivalence checkers returning explicit
counterexamples on failure.

</details>

# Types and constructors

<a id="type-517569636b436f6e666967"></a>

## `QuickConfig`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">QuickConfig</span></code></pre>

Maximum number of trials, compound size hint, and initial PCG state.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 37

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">QuickConfig</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Pcg</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickConfig</span>)))</code></pre>

<a id="type-517569636b457175616c4661696c757265"></a>

## `QuickEqualFailure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">QuickEqualFailure</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>)</code></pre>

One-based failing trial count, shared input, left output, and right output.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 75

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">QuickEqualFailure</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> . (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">b</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickEqualFailure</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>))))</code></pre>

<a id="type-517569636b4661696c757265"></a>

## `QuickFailure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">QuickFailure</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

One-based failing trial count, counterexample input, and diagnostic reason.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 58

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">QuickFailure</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickFailure</span> <span class="tok-variable">a</span>)))</code></pre>

# Signatures and methods

<a id="class-417262697472617279"></a>

## `Arbitrary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Arbitrary</span> <span class="tok-variable">a</span></code></pre>

Generator of arbitrary test values driven by an explicit PCG state and a size hint.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 30

<a id="method-4172626974726172792e617262697472617279"></a>

## `Arbitrary.arbitrary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">arbitrary</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">Arbitrary</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Pcg</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-variable">a</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Pcg</span>)))</code></pre>

Produce a pseudorandom value and the next PCG generator state using the given size hint.

# Values

<a id="value-64656661756c742d717569636b2d636f6e666967"></a>

## `default-quick-config`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">default-quick-config</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickConfig</span></code></pre>

Default configuration: 100 trials, size bound 50, deterministic PCG seed (1, 2).

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 40

<a id="value-717569636b2d636865636b"></a>

## `quick-check`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quick-check</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">arb</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">Arbitrary</span> <span class="tok-variable">a</span>) =&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickFailure</span> <span class="tok-variable">a</span>) <span class="tok-constructor">Nat</span>))</code></pre>

Run a one-argument property using default-quick-config.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 267

<a id="value-717569636b2d636865636b2d657175616c"></a>

## `quick-check-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quick-check-equal</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">arb</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">Arbitrary</span> <span class="tok-variable">a</span>, <span class="tok-variable">eqb</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">b</span>) =&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickEqualFailure</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>) <span class="tok-constructor">Nat</span>)))</code></pre>

Check whether two functions agree on generated inputs using default-quick-config.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 307

<a id="value-717569636b2d636865636b2d657175616c2d77697468"></a>

## `quick-check-equal-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quick-check-equal-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">arb</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">Arbitrary</span> <span class="tok-variable">a</span>, <span class="tok-variable">eqb</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">b</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickConfig</span> -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickEqualFailure</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>) <span class="tok-constructor">Nat</span>))))</code></pre>

Check whether two functions produce equal outputs over generated inputs under explicit configuration.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 289

<a id="value-717569636b2d636865636b2d77697468"></a>

## `quick-check-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quick-check-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">arb</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">Arbitrary</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickConfig</span> -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickFailure</span> <span class="tok-variable">a</span>) <span class="tok-constructor">Nat</span>)))</code></pre>

Run a one-argument property under explicit configuration and return either the failing trial or total passing trials.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 251

<a id="value-717569636b2d636865636b32"></a>

## `quick-check2`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quick-check2</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">arba</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">Arbitrary</span> <span class="tok-variable">a</span>, <span class="tok-variable">arbb</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">Arbitrary</span> <span class="tok-variable">b</span>) =&gt; ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span> -&gt; <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickFailure</span> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>)) <span class="tok-constructor">Nat</span>))</code></pre>

Run a two-argument property using default-quick-config.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 276

<a id="value-717569636b2d636865636b322d77697468"></a>

## `quick-check2-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quick-check2-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">arba</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">Arbitrary</span> <span class="tok-variable">a</span>, <span class="tok-variable">arbb</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">Arbitrary</span> <span class="tok-variable">b</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickConfig</span> -&gt; ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span> -&gt; <span class="tok-constructor">Bool</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickFailure</span> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>)) <span class="tok-constructor">Nat</span>)))</code></pre>

Run a two-argument property under explicit configuration.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 271

<a id="value-717569636b2d636865636b33"></a>

## `quick-check3`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quick-check3</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">c</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">arba</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">Arbitrary</span> <span class="tok-variable">a</span>, <span class="tok-variable">arbb</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">Arbitrary</span> <span class="tok-variable">b</span>, <span class="tok-variable">arbc</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">Arbitrary</span> <span class="tok-variable">c</span>) =&gt; ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Bool</span>))) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickFailure</span> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>, <span class="tok-variable">c</span>)) <span class="tok-constructor">Nat</span>))</code></pre>

Run a three-argument property using default-quick-config.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 285

<a id="value-717569636b2d636865636b332d77697468"></a>

## `quick-check3-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quick-check3-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">c</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">arba</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">Arbitrary</span> <span class="tok-variable">a</span>, <span class="tok-variable">arbb</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">Arbitrary</span> <span class="tok-variable">b</span>, <span class="tok-variable">arbc</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">Arbitrary</span> <span class="tok-variable">c</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickConfig</span> -&gt; ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">c</span> -&gt; <span class="tok-constructor">Bool</span>))) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickFailure</span> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>, <span class="tok-variable">c</span>)) <span class="tok-constructor">Nat</span>)))</code></pre>

Run a three-argument property under explicit configuration.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 280

<a id="value-717569636b2d636f6e6669672d6d61782d636f756e74"></a>

## `quick-config-max-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quick-config-max-count</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickConfig</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Read the configured maximum number of trials.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 44

<a id="value-717569636b2d636f6e6669672d6d61782d73697a65"></a>

## `quick-config-max-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quick-config-max-size</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickConfig</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Read the configured compound-value size hint.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 48

<a id="value-717569636b2d636f6e6669672d73656564"></a>

## `quick-config-seed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quick-config-seed</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickConfig</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Pcg</span>)</code></pre>

Read the initial PCG seed stored in the configuration.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 52

<a id="value-717569636b2d657175616c2d636f756e74"></a>

## `quick-equal-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quick-equal-count</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickEqualFailure</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>) -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Return the one-based trial index on which the two functions disagreed.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 78

<a id="value-717569636b2d657175616c2d696e707574"></a>

## `quick-equal-input`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quick-equal-input</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickEqualFailure</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>) -&gt; <span class="tok-variable">a</span>)</code></pre>

Return the input value on which the two functions disagreed.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 82

<a id="value-717569636b2d657175616c2d6c656674"></a>

## `quick-equal-left`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quick-equal-left</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickEqualFailure</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>) -&gt; <span class="tok-variable">b</span>)</code></pre>

Return the output produced by the first function on the counterexample input.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 86

<a id="value-717569636b2d657175616c2d7269676874"></a>

## `quick-equal-right`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quick-equal-right</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickEqualFailure</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>) -&gt; <span class="tok-variable">b</span>)</code></pre>

Return the output produced by the second function on the counterexample input.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 90

<a id="value-717569636b2d6661696c7572652d636f756e74"></a>

## `quick-failure-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quick-failure-count</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickFailure</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Return the one-based trial index on which the property failed.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 61

<a id="value-717569636b2d6661696c7572652d696e707574"></a>

## `quick-failure-input`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quick-failure-input</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickFailure</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-variable">a</span>)</code></pre>

Return the input value that falsified the property.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 65

<a id="value-717569636b2d6661696c7572652d726561736f6e"></a>

## `quick-failure-reason`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">quick-failure-reason</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Test</span>.<span class="tok-constructor">Quick</span>.<span class="tok-constructor">QuickFailure</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the failure reason description.

[Source](/reference-sources/Effect.Test.Quick.cinch) · line 69

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Effect.Test.Quick.Arbitrary_Bool` | `Effect.Test.Quick.Arbitrary Bool` | Implicit |
| `Effect.Test.Quick.Arbitrary_Bytes` | `Effect.Test.Quick.Arbitrary Bytes` | Implicit |
| `Effect.Test.Quick.Arbitrary_Char` | `Effect.Test.Quick.Arbitrary Char` | Implicit |
| `Effect.Test.Quick.Arbitrary_F64` | `Effect.Test.Quick.Arbitrary F64` | Implicit |
| `Effect.Test.Quick.Arbitrary_I16` | `Effect.Test.Quick.Arbitrary I16` | Implicit |
| `Effect.Test.Quick.Arbitrary_I32` | `Effect.Test.Quick.Arbitrary I32` | Implicit |
| `Effect.Test.Quick.Arbitrary_I64` | `Effect.Test.Quick.Arbitrary I64` | Implicit |
| `Effect.Test.Quick.Arbitrary_I8` | `Effect.Test.Quick.Arbitrary I8` | Implicit |
| `Effect.Test.Quick.Arbitrary_Int` | `Effect.Test.Quick.Arbitrary Int` | Implicit |
| `Effect.Test.Quick.Arbitrary_List` | `Effect.Test.Quick.Arbitrary (Data.Core.List a)` | Implicit |
| `Effect.Test.Quick.Arbitrary_Nat` | `Effect.Test.Quick.Arbitrary Nat` | Implicit |
| `Effect.Test.Quick.Arbitrary_Option` | `Effect.Test.Quick.Arbitrary (Data.Core.Option a)` | Implicit |
| `Effect.Test.Quick.Arbitrary_Pair` | `Effect.Test.Quick.Arbitrary (a, b)` | Implicit |
| `Effect.Test.Quick.Arbitrary_String` | `Effect.Test.Quick.Arbitrary Data.String.String` | Implicit |
| `Effect.Test.Quick.Arbitrary_Triple` | `Effect.Test.Quick.Arbitrary (a, b, c)` | Implicit |
| `Effect.Test.Quick.Arbitrary_U16` | `Effect.Test.Quick.Arbitrary U16` | Implicit |
| `Effect.Test.Quick.Arbitrary_U32` | `Effect.Test.Quick.Arbitrary U32` | Implicit |
| `Effect.Test.Quick.Arbitrary_U64` | `Effect.Test.Quick.Arbitrary U64` | Implicit |
| `Effect.Test.Quick.Arbitrary_U8` | `Effect.Test.Quick.Arbitrary U8` | Implicit |

**`Effect.Test.Quick.Arbitrary_Bool`**: Arbitrary booleans with equal probability of True and False.
**`Effect.Test.Quick.Arbitrary_Bytes`**: Arbitrary byte buffers with byte length drawn uniformly in [0, size].
**`Effect.Test.Quick.Arbitrary_Char`**: Arbitrary Unicode scalar values distributed uniformly across all non-surrogate code points.
**`Effect.Test.Quick.Arbitrary_F64`**: Arbitrary finite binary64 floats spanning [-max-f64, +max-f64].
**`Effect.Test.Quick.Arbitrary_I16`**: Arbitrary signed 16-bit integers across [-32768, 32767].
**`Effect.Test.Quick.Arbitrary_I32`**: Arbitrary signed 32-bit integers across the full 32-bit range.
**`Effect.Test.Quick.Arbitrary_I64`**: Arbitrary signed 64-bit integers across the full 64-bit range.
**`Effect.Test.Quick.Arbitrary_I8`**: Arbitrary signed 8-bit integers across [-128, 127].
**`Effect.Test.Quick.Arbitrary_Int`**: Arbitrary mathematical integers mixing small signed values and full 64-bit draws.
**`Effect.Test.Quick.Arbitrary_List`**: Arbitrary lists with length drawn uniformly in [0, size] and halved element size hints.
**`Effect.Test.Quick.Arbitrary_Nat`**: Arbitrary natural numbers mixing small size-bounded values and full 63-bit draws.
**`Effect.Test.Quick.Arbitrary_Option`**: Arbitrary optional values yielding None one quarter of the time and Some otherwise.
**`Effect.Test.Quick.Arbitrary_Pair`**: Arbitrary pairs generated independently in left-to-right order.
**`Effect.Test.Quick.Arbitrary_String`**: Arbitrary valid UTF-8 strings with scalar length drawn uniformly in [0, size].
**`Effect.Test.Quick.Arbitrary_Triple`**: Arbitrary triples generated independently in left-to-right order.
**`Effect.Test.Quick.Arbitrary_U16`**: Arbitrary unsigned 16-bit integers across [0, 65535].
**`Effect.Test.Quick.Arbitrary_U32`**: Arbitrary unsigned 32-bit integers across the full 32-bit range.
**`Effect.Test.Quick.Arbitrary_U64`**: Arbitrary unsigned 64-bit integers across the full 64-bit range.
**`Effect.Test.Quick.Arbitrary_U8`**: Arbitrary unsigned 8-bit integers across [0, 255].

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Test/Quick.cinch`. Source SHA-256: `09074ee87ada5bc478ed239aad09e33196f707b3e02c11461afd2fe27eb5b667`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
