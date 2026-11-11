# Effect.Concurrent

Shipped task/worker vocabulary supplementing the compiler-installed structured scope and join operations.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Effect.Concurrent`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Concurrent.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Result](Data.Result.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Concurrent: the structured concurrency worked vocabulary.

The worked vocabulary of the concurrency specification's `loadPair`
sketch (section 2). A `Request` names bytes to read from the
runtime's input source; `fetch` performs the read under the `Io r`
capability and reports a short read as a `LoadError`. `withScope`,
`spawn`, `joinAllOrCancel` and the `Io r` operations (`ioWrite`,
`ioRead`, `ioSleep`, `pollCancel`) are builtin values with
compiler-known schemes (like `newIO`), never foreign imports.

</details>

# Types and constructors

<a id="type-4c6f61644572726f72"></a>

## `LoadError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">LoadError</span></code></pre>

[Source](/reference-sources/Effect.Concurrent.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LoadError</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">LoadError</span>)</code></pre>

<a id="type-52657175657374"></a>

## `Request`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Request</span></code></pre>

Structured concurrency (concurrency specification, section 2): the
worked vocabulary of the spec's `loadPair` sketch. A `Request` names
bytes to read from the runtime's input source; `fetch` performs the
read under the `Io r` capability and reports a short read as a
`LoadError`. `withScope`, `spawn`, `joinAllOrCancel` and the `Io r`
operations (`ioWrite`, `ioRead`, `ioSleep`, `pollCancel`) are builtin
values with compiler-known schemes (like `newIO`), never foreign
imports.

[Source](/reference-sources/Effect.Concurrent.cinch) · line 24

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Request</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">Request</span>)</code></pre>

# Values

<a id="value-6665746368"></a>

## `fetch`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fetch</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">Request</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Concurrent</span>.<span class="tok-constructor">LoadError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

[Source](/reference-sources/Effect.Concurrent.cinch)

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Concurrent.cinch`. Source SHA-256: `07b1741a8b937105d3fefff700f6cf6ba415e1f8781e2924c396532bccf5d62a`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
