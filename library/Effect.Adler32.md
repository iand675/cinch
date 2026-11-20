# Effect.Adler32

Inspectable scalar checksum sinks, generic byte writers and bounded reader-to-Adler-32 copying with advanced source continuations.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Adler32`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Adler32.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Effect.Stream](Effect.Stream.md), [Text.Adler32](Text.Adler32.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Adler32](Effect.Adler32.md), [Effect.Stream](Effect.Stream.md), [Text.Adler32](Text.Adler32.md), [Text.Binary](Text.Binary.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Inspectable Adler-32 hashing sinks and bounded reader-to-checksum copying.
The immutable scalar state is the sink handle. Writes accept every byte and
never retain input buffers. Copying returns the advanced reader and the last
valid checksum on failure, and never closes a borrowed source.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-61646c657233322d636f7079"></a>

## `adler32-copy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32-copy</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32State</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>), <span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32State</span>)))))</code></pre>

Hash successful reader chunks through EOF, preserving both advanced handles on errors.
A zero chunk size is rejected before reading. Invalid source counts do not enter the checksum.

[Source](/reference-sources/Effect.Adler32.cinch) · line 27

<a id="value-61646c657233322d68617368696e672d777269746572"></a>

## `adler32-hashing-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32-hashing-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32State</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))</code></pre>

Adapt a checksum state to the generic byte-writer protocol.
Use the explicit state or adler32-copy when the advanced checksum must be inspected.

[Source](/reference-sources/Effect.Adler32.cinch) · line 21

<a id="value-61646c657233322d7772697465"></a>

## `adler32-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32State</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32State</span>)))</code></pre>

Hash a whole chunk, returning its accepted count and the advanced immutable state.

[Source](/reference-sources/Effect.Adler32.cinch) · line 16

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Adler32.cinch`. Source SHA-256: `561a9c98abc71e5022289621323d3003d9419d7ac5e811310389bebf1c1d3566`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
