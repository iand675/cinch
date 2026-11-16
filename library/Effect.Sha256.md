# Effect.Sha256

Inspectable SHA-224/SHA-256 states, generic digest sinks and bounded reader-to-hash copying with advanced source continuations.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Sha256`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Sha256.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Effect.Stream](Effect.Stream.md), [Text.Sha256](Text.Sha256.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Sha256](Effect.Sha256.md), [Effect.Stream](Effect.Stream.md), [Text.Binary](Text.Binary.md), [Text.Sha256](Text.Sha256.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Inspectable SHA-224/SHA-256 hashing sinks and bounded reader-to-digest copying.
The immutable digest state is the sink handle. Writes accept every byte and
never retain input buffers. Copying returns the advanced reader and the last
valid digest on failure, and never closes a borrowed source.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-7368613235362d636f7079"></a>

## `sha256-copy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha256-copy</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256State</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>), <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256State</span>)))))</code></pre>

Hash successful reader chunks through EOF, preserving both advanced handles on errors.
A zero chunk size is rejected before reading. Invalid source counts do not enter the digest.

[Source](/reference-sources/Effect.Sha256.cinch) · line 27

<a id="value-7368613235362d68617368696e672d777269746572"></a>

## `sha256-hashing-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha256-hashing-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256State</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))</code></pre>

Adapt a digest state to the generic byte-writer protocol.
Use the explicit state or sha256-copy when the advanced digest must be inspected.

[Source](/reference-sources/Effect.Sha256.cinch) · line 21

<a id="value-7368613235362d7772697465"></a>

## `sha256-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha256-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256State</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256State</span>)))</code></pre>

Hash a whole chunk, returning its accepted count and the advanced immutable state.

[Source](/reference-sources/Effect.Sha256.cinch) · line 16

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Sha256.cinch`. Source SHA-256: `531b0b74ab549b22d7ee6837eb4090db8b31800860afa8bec249f2f4fd64048f`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
