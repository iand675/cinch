# Effect.Sha1

Inspectable SHA-1 states, generic digest sinks and bounded reader-to-hash copying for legacy formats.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Sha1`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Sha1.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Effect.Stream](Effect.Stream.md), [Text.Sha1](Text.Sha1.md).

Non-embedded source closure: [Effect.Sha1](Effect.Sha1.md), [Effect.Stream](Effect.Stream.md), [Text.Binary](Text.Binary.md), [Text.Sha1](Text.Sha1.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Inspectable SHA-1 hashing sinks and bounded reader-to-digest copying.
The immutable digest state is the sink handle. Writes accept every byte and
never retain input buffers. Copying returns the advanced reader and the last
valid digest on failure, and never closes a borrowed source.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-736861312d636f7079"></a>

## `sha1-copy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha1-copy</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1State</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>), <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1State</span>)))))</code></pre>

Hash successful reader chunks through EOF, preserving both advanced handles on errors.
A zero chunk size is rejected before reading. Invalid source counts do not enter the digest.

[Source](/reference-sources/Effect.Sha1.cinch) · line 27

<a id="value-736861312d68617368696e672d777269746572"></a>

## `sha1-hashing-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha1-hashing-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1State</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))</code></pre>

Adapt a digest state to the generic byte-writer protocol.
Use the explicit state or sha1-copy when the advanced digest must be inspected.

[Source](/reference-sources/Effect.Sha1.cinch) · line 21

<a id="value-736861312d7772697465"></a>

## `sha1-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha1-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1State</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1State</span>)))</code></pre>

Hash a whole chunk, returning its accepted count and the advanced immutable state.

[Source](/reference-sources/Effect.Sha1.cinch) · line 16

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Sha1.cinch`. Source SHA-256: `7fe972afface0debe29d9ce3b39d01d7e1dfc16444702fb10b98986343db1201`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
