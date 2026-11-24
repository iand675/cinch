# Effect.Buffered.Internal.Scan

Exact bounded byte scans shared by buffered input and token splitters.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Buffered.Internal.Scan`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Buffered.Internal.Scan.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Wide](Data.Wide.md).

Non-embedded source closure: [Effect.Buffered.Internal.Scan](Effect.Buffered.Internal.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Exact bounded byte scans shared by buffered input and token splitters.
Complete sixteen-byte blocks use portable SIMD lane comparisons; scalar
fringes never read before the requested start or after the half-open end.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-61736369692d737061636573"></a>

## `ascii-spaces`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii-spaces</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>)))</code></pre>

Advance past ASCII whitespace in a bounded span, returning an absolute endpoint; invalid bounds leave the offset unchanged.

[Source](/reference-sources/Effect.Buffered.Internal.Scan.cinch) · line 86

<a id="value-61736369692d776f7264"></a>

## `ascii-word`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ascii-word</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>)))</code></pre>

Advance past ASCII non-whitespace bytes, stopping before Unicode fringes for scalar decoding.

[Source](/reference-sources/Effect.Buffered.Internal.Scan.cinch) · line 123

<a id="value-66696e642d62797465"></a>

## `find-byte`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">find-byte</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>)))))</code></pre>

Find a byte in a checked half-open owner span, returning its absolute offset or None for absence or invalid bounds.

[Source](/reference-sources/Effect.Buffered.Internal.Scan.cinch) · line 52

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Buffered/Internal/Scan.cinch`. Source SHA-256: `851f155927bdbd4386078c6581e2ed9733b78295c1d0b1d2b41b278c22edb76a`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
