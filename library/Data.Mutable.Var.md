# Data.Mutable.Var

A single mutable cell, written in place; not synchronized (TVar is the shared cell).

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Mutable.Var`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Mutable.Var.cinch) · [How library modules are loaded](00-index.md)

Non-embedded source closure: [Data.Mutable.Var](Data.Mutable.Var.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Mutable.Var: the single mutable cell (Haskell's `IORef`/`STRef`,
OCaml's `ref`).

A `Var r a` always holds one value. `var-write` releases the value it
replaces; `var-modify` moves the value out and back, so a uniquely held
value is rewritten without a copy. It is not synchronized: the shared
cell between tasks is `Effect.Stm`'s `TVar`.

The trusted core is the private `cinch.runtime.slots*` imports (the
compiler's `slots.rs`); none of them leaves the module. The one slot is
written before the cell is returned.

</details>

# Types and constructors

<a id="type-566172"></a>

## `Var`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Var</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Var.cinch)

# Values

<a id="value-7661722d6d6f64696679"></a>

## `var-modify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">var-modify</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Var</span>.<span class="tok-constructor">Var</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.Var.cinch)

<a id="value-7661722d6e6577"></a>

## `var-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">var-new</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Var</span>.<span class="tok-constructor">Var</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Var.cinch)

<a id="value-7661722d72656164"></a>

## `var-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">var-read</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Var</span>.<span class="tok-constructor">Var</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Data.Mutable.Var.cinch)

<a id="value-7661722d73776170"></a>

## `var-swap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">var-swap</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Var</span>.<span class="tok-constructor">Var</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-variable">a</span>))</code></pre>

Store `x` and return the value it replaced.

[Source](/reference-sources/Data.Mutable.Var.cinch) · line 54

<a id="value-7661722d7772697465"></a>

## `var-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">var-write</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Mutable</span>.<span class="tok-constructor">Var</span>.<span class="tok-constructor">Var</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">Mut</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

[Source](/reference-sources/Data.Mutable.Var.cinch)

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Mutable/Var.cinch`. Source SHA-256: `1940d102ebcfdd02df9b0f81279f991e5419a480937179f0a635b1de17f25b26`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
