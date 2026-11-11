# Effect.Sequencing

The direct, Result, and Option do providers. One lexical provider/container is selected per definition.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Effect.Sequencing`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Sequencing.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Option](Data.Option.md), [Data.Result](Data.Result.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Sequencing: the `do` sequencing providers (decision D22).

The same `do` spelling types differently per provider: direct
sequencing is the default and a `using` binder with a provider head
selects another. `pure` and `bind` are the provider's class methods
(decision D2). `do` picks its provider lexically per definition:
exactly one container per definition, no polymorphism. The
Category.* classes abstract over containers for polymorphic code;
the two mechanisms do not interact.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary. The provider instances live with their
container's type (Data.Option, Data.Result); the direct provider is
the identity and its instance lives here with its class.

</details>

# Types and constructors

# Signatures and methods

<a id="class-44697265637453657175656e63696e67"></a>

## `DirectSequencing`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">DirectSequencing</span> </code></pre>

Sequencing providers for `do` notation (runtime capabilities,
effects and sequencing specification, "Unqualified do through
modules and typeclasses"; design section 5). The same `do` spelling
types differently per provider: direct sequencing is the default and
a `using` binder with a provider head selects another. `pure` and
`bind` are the provider's class methods (decision D2).

[Source](/reference-sources/Effect.Sequencing.cinch) · line 29

<a id="method-44697265637453657175656e63696e672e70757265"></a>

## `DirectSequencing.pure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pure</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sequencing</span>.<span class="tok-constructor">DirectSequencing</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>)</code></pre>



<a id="method-44697265637453657175656e63696e672e62696e64"></a>

## `DirectSequencing.bind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bind</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">ef</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sequencing</span>.<span class="tok-constructor">DirectSequencing</span>) =&gt; (<span class="tok-variable">a</span> -&gt; ((<span class="tok-variable">a</span> -&gt;{<span class="tok-variable">ef</span>} <span class="tok-variable">b</span>) -&gt;{<span class="tok-variable">ef</span>} <span class="tok-variable">b</span>))</code></pre>



<a id="class-4f7074696f6e53657175656e63696e67"></a>

## `OptionSequencing`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">OptionSequencing</span> </code></pre>

[Source](/reference-sources/Effect.Sequencing.cinch)

<a id="method-4f7074696f6e53657175656e63696e672e70757265"></a>

## `OptionSequencing.pure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pure</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sequencing</span>.<span class="tok-constructor">OptionSequencing</span>) =&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>



<a id="method-4f7074696f6e53657175656e63696e672e62696e64"></a>

## `OptionSequencing.bind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bind</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">ef</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sequencing</span>.<span class="tok-constructor">OptionSequencing</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span> -&gt; ((<span class="tok-variable">a</span> -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">b</span>) -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">b</span>))</code></pre>



<a id="class-526573756c7453657175656e63696e67"></a>

## `ResultSequencing`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">ResultSequencing</span> <span class="tok-variable">e</span></code></pre>

[Source](/reference-sources/Effect.Sequencing.cinch)

<a id="method-526573756c7453657175656e63696e672e70757265"></a>

## `ResultSequencing.pure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pure</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sequencing</span>.<span class="tok-constructor">ResultSequencing</span> <span class="tok-variable">e</span>) =&gt; (<span class="tok-variable">a</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span>) <span class="tok-variable">a</span>))</code></pre>



<a id="method-526573756c7453657175656e63696e672e62696e64"></a>

## `ResultSequencing.bind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bind</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">ef</span> . <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Sequencing</span>.<span class="tok-constructor">ResultSequencing</span> <span class="tok-variable">e</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span> -&gt; ((<span class="tok-variable">a</span> -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">b</span>) -&gt;{<span class="tok-variable">ef</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">b</span>))</code></pre>



# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Effect.Sequencing.SeqDirect` | `Effect.Sequencing.DirectSequencing` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Sequencing.cinch`. Source SHA-256: `1d94fcf10e01ff6bdacecc5db92317769f719cdabf7f30cf03ef5665e2f1f255`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
