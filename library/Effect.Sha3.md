# Effect.Sha3

Inspectable SHA-3/SHAKE states, generic digest sinks, bounded reader-to-hash copying and SHAKE output as a generic reader.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Sha3`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Sha3.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Effect.Stream](Effect.Stream.md), [Text.Sha3](Text.Sha3.md).

Non-embedded source closure: [Effect.Sha3](Effect.Sha3.md), [Effect.Stream](Effect.Stream.md), [Text.Binary](Text.Binary.md), [Text.Sha3](Text.Sha3.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Inspectable SHA-3 and SHAKE hashing sinks and bounded reader-to-digest copying.
The immutable digest state is the sink handle. Writes accept every byte and
never retain input buffers. Copying returns the advanced reader and the last
valid digest on failure, and never closes a borrowed source. Extendable-output
readers adapt to the generic reader protocol as unbounded sources.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-736861332d636f7079"></a>

## `sha3-copy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-copy</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>), <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span>)))))</code></pre>

Hash successful reader chunks through EOF, preserving both advanced handles on errors.
A zero chunk size is rejected before reading. Invalid source counts do not enter the digest.

[Source](/reference-sources/Effect.Sha3.cinch) · line 28

<a id="value-736861332d68617368696e672d777269746572"></a>

## `sha3-hashing-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-hashing-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))</code></pre>

Adapt a digest state to the generic byte-writer protocol.
Use the explicit state or sha3-copy when the advanced digest must be inspected.

[Source](/reference-sources/Effect.Sha3.cinch) · line 22

<a id="value-736861332d7772697465"></a>

## `sha3-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span>)))</code></pre>

Hash a whole chunk, returning its accepted count and the advanced immutable state.

[Source](/reference-sources/Effect.Sha3.cinch) · line 17

<a id="value-7368616b652d627974652d726561646572"></a>

## `shake-byte-reader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shake-byte-reader</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">ShakeReader</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>))</code></pre>

Adapt a squeezing sponge to the generic reader protocol: every read returns exactly the
requested number of output bytes, so the source never reaches end of input. Bound it with
reader-limit when a finite stream is required.

[Source](/reference-sources/Effect.Sha3.cinch) · line 43

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Sha3.cinch`. Source SHA-256: `b08dbb1bd12da929f2c38cdad56b119dcc35f59205372e687872ed8ec0d5fe92`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
