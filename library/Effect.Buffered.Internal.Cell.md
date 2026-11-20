# Effect.Buffered.Internal.Cell

Initialized opaque continuation cells whose allocation, observation and replacement require their I/O region.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Buffered.Internal.Cell`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Buffered.Internal.Cell.cinch) · [How library modules are loaded](00-index.md)

Non-embedded source closure: [Effect.Buffered.Internal.Cell](Effect.Buffered.Internal.Cell.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Internal managed continuation cells whose allocation, reads and writes require I/O.
The sole slot is initialized before a cell can be observed. Neither its array
nor a raw address is exported; the runtime retains and releases stored values.

</details>

# Types and constructors

<a id="type-496f43656c6c"></a>

## `IoCell`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">IoCell</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

An initialized managed slot, scoped by its I/O region and value type.

[Source](/reference-sources/Effect.Buffered.Internal.Cell.cinch) · line 17

# Values

<a id="value-63656c6c2d6e6577"></a>

## `cell-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cell-new</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Internal</span>.<span class="tok-constructor">Cell</span>.<span class="tok-constructor">IoCell</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>))</code></pre>

Allocate exactly one slot and initialize it before returning the opaque cell.

[Source](/reference-sources/Effect.Buffered.Internal.Cell.cinch) · line 22

<a id="value-63656c6c2d72656164"></a>

## `cell-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cell-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Internal</span>.<span class="tok-constructor">Cell</span>.<span class="tok-constructor">IoCell</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-variable">a</span>)</code></pre>

Retain and observe the cell's current value through the managed slot operation.

[Source](/reference-sources/Effect.Buffered.Internal.Cell.cinch) · line 27

<a id="value-63656c6c2d7772697465"></a>

## `cell-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cell-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Buffered</span>.<span class="tok-constructor">Internal</span>.<span class="tok-constructor">Cell</span>.<span class="tok-constructor">IoCell</span> <span class="tok-variable">r</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Unit</span>))</code></pre>

Replace the initialized value, retaining the replacement and releasing the previous value.

[Source](/reference-sources/Effect.Buffered.Internal.Cell.cinch) · line 31

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Buffered/Internal/Cell.cinch`. Source SHA-256: `3c33309272f788521ec17bf6fa60ccd331afdb311049bb2977cedaf0ac067564`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
