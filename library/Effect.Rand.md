# Effect.Rand

Pseudorandom sources seeded from operating-system entropy.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Rand`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Rand.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.ChaCha8](Data.ChaCha8.md), [Data.Rand](Data.Rand.md), [Data.Result](Data.Result.md), [Effect.Random](Effect.Random.md).

Non-embedded source closure: [Data.Array](Data.Array.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.ChaCha8](Data.ChaCha8.md), [Data.Float](Data.Float.md), [Data.Rand](Data.Rand.md), [Effect.Rand](Effect.Rand.md), [Effect.Random](Effect.Random.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Pseudorandom sources seeded from the operating system: the effectful entry to Data.Rand.

Each call draws fresh seed material from Effect.Random's entropy source (128 bits for PCG, 256 for ChaCha8),
so two sources made this way are independent and unpredictable; drawing from them is then pure and
replayable (keep the source, or its encoding, to replay). Use a ChaCha8 source when outputs must also be
hard to predict from earlier outputs.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-72616e646f6d2d63686163686138"></a>

## `random-chacha8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">random-chacha8</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">ChaCha8</span>.<span class="tok-constructor">ChaCha8Error</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">ChaCha8</span>.<span class="tok-constructor">ChaCha8</span>))</code></pre>

A ChaCha8 generator seeded with 256 bits of operating-system entropy (the seed always has the required
32 bytes, so the result is Ok whenever the entropy source answers).

[Source](/reference-sources/Effect.Rand.cinch) · line 28

<a id="value-72616e646f6d2d706367"></a>

## `random-pcg`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">random-pcg</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Rand</span>.<span class="tok-constructor">Pcg</span>)</code></pre>

A PCG generator seeded with 128 bits of operating-system entropy.

[Source](/reference-sources/Effect.Rand.cinch) · line 23

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Rand.cinch`. Source SHA-256: `1849a085f5e7c61bd4d30491b389f120737eeaee6049ff0839e428e717b7efdf`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
