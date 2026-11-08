# Data.Producer

Partial producer vocabulary, distinct from codata whose next observation is guaranteed productive.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Producer`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Producer.cinch) · [How library modules are loaded](00-index.md)

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Producer: the vector step form and the producer library.

The step form (modules and compilation specification section 6):
`Done`, `Skip(state)` and `Yield(value, state)`. Producer pipelines
lower to state machines; the fusion rules compose them. `vec-stream`
is a producer over a borrowed vector: the block walk of the step
form is the `vecStreamStep` intrinsic. `scalar-sum-wrap32` is the
scalar model (platform proofs and SIMD specification section 5):
the sum modulo 2^32 over the borrowed vector.

</details>

# Types and constructors

<a id="type-50726f6475636572"></a>

## `Producer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Producer</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A producer: a state with its step function.

[Source](/reference-sources/Data.Producer.cinch) · line 23

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Producer</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span> . ((<span class="tok-variable">s</span> -&gt; <span class="tok-constructor">Step</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">s</span> -&gt; <span class="tok-constructor">Producer</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span>))</code></pre>

<a id="type-53746570"></a>

## `Step`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Step</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

The vector step form (modules section 6): `Done`, `Skip(state)` and
`Yield(value, state)`. Producer pipelines lower to state machines.

[Source](/reference-sources/Data.Producer.cinch) · line 17

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Done</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span> . <span class="tok-constructor">Step</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Skip</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span> . (<span class="tok-variable">s</span> -&gt; <span class="tok-constructor">Step</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Yield</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span> . (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">s</span> -&gt; <span class="tok-constructor">Step</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span>))</code></pre>

<a id="type-54616b655374"></a>

## `TakeSt`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TakeSt</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>)</code></pre>

[Source](/reference-sources/Data.Producer.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TakeSt</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span> . (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-variable">s</span> -&gt; <span class="tok-constructor">TakeSt</span> <span class="tok-variable">s</span>))</code></pre>

# Values

<a id="value-70726f642d666f6c64"></a>

## `prod-fold`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prod-fold</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">b</span> -&gt; ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>)) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Producer</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-variable">b</span>)))</code></pre>

[Source](/reference-sources/Data.Producer.cinch)

<a id="value-70726f642d666f6c642d676f"></a>

## `prod-fold-go`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prod-fold-go</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">s</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Step</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span>)) -&gt; ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>)) -&gt; (<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">s</span> -&gt; <span class="tok-variable">b</span>))))</code></pre>

[Source](/reference-sources/Data.Producer.cinch)

<a id="value-70726f642d6d6170"></a>

## `prod-map`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prod-map</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Producer</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Producer</span> <span class="tok-variable">s</span> <span class="tok-variable">b</span>)))</code></pre>

[Source](/reference-sources/Data.Producer.cinch)

<a id="value-70726f642d74616b65"></a>

## `prod-take`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prod-take</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Producer</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Producer</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">TakeSt</span> <span class="tok-variable">s</span>) <span class="tok-variable">a</span>)))</code></pre>

[Source](/reference-sources/Data.Producer.cinch)

<a id="value-70726f642d74616b652d73746570"></a>

## `prod-take-step`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prod-take-step</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">s</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">s</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Step</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span>)) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">TakeSt</span> <span class="tok-variable">s</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">Step</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Producer</span>.<span class="tok-constructor">TakeSt</span> <span class="tok-variable">s</span>) <span class="tok-variable">a</span>)))</code></pre>

[Source](/reference-sources/Data.Producer.cinch)

<a id="value-7363616c61722d73756d2d777261703332"></a>

## `scalar-sum-wrap32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scalar-sum-wrap32</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">n</span> :: <span class="tok-constructor">Nat</span>) <span class="tok-variable">region</span> . (<span class="tok-constructor">Ref</span> <span class="tok-variable">region</span> <span class="tok-constructor">Vec</span> <span class="tok-variable">n</span> <span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

[Source](/reference-sources/Data.Producer.cinch)

<a id="value-7665632d73747265616d"></a>

## `vec-stream`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">vec-stream</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> (<span class="tok-variable">n</span> :: <span class="tok-constructor">Nat</span>) <span class="tok-variable">region</span> . (<span class="tok-constructor">Ref</span> <span class="tok-variable">region</span> <span class="tok-constructor">Vec</span> <span class="tok-variable">n</span> <span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Producer</span> (<span class="tok-constructor">Ref</span> <span class="tok-variable">region</span> <span class="tok-constructor">Vec</span> <span class="tok-variable">n</span> <span class="tok-variable">a</span>) <span class="tok-variable">a</span>)</code></pre>

A producer over a borrowed vector: the block walk of the step form
is the `vecStreamStep` intrinsic.

[Source](/reference-sources/Data.Producer.cinch) · line 62

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Producer.cinch`. Source SHA-256: `47b6a989c18527ed19e80914ff23860e4421c239eba407effa85fe855f94f30f`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
