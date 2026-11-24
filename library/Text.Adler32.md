# Text.Adler32

Portable SIMD Adler-32 with bounded machine-word segments, persistent states, network-order digests and checked versioned checkpoints.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Adler32`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Adler32.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Text.Binary](Text.Binary.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Adler32](Text.Adler32.md), [Text.Binary](Text.Binary.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Adler-32 checksums, persistent accumulators and versioned binary checkpoints.
Updates consume sixteen bytes with packed parallel weighted reductions and normalize
the two sums after segments of at most 5552 bytes. Accumulators use explicit
32-bit wrapping; normal states satisfy the segment's no-overflow bound.
Digest bytes use network order. Retaining a state branches the computation,
and all returned byte buffers are fresh. This checksum does not authenticate data.

</details>

# Types and constructors

<a id="type-41646c657233325374617465"></a>

## `Adler32State`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Adler32State</span></code></pre>

An immutable scalar checksum accumulator; input buffers are never retained.

[Source](/reference-sources/Text.Adler32.cinch) · line 84

<a id="type-41646c6572333253746174654572726f72"></a>

## `Adler32StateError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Adler32StateError</span></code></pre>

A checkpoint identifier or complete byte length is invalid.

[Source](/reference-sources/Text.Adler32.cinch) · line 136

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidAdler32StateIdentifier</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32StateError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidAdler32StateSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32StateError</span>))</code></pre>

# Values

<a id="value-61646c657233322d7374617465"></a>

## `adler32-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32-state</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32State</span></code></pre>

Initial accumulator: low sum 1 and high sum 0.

[Source](/reference-sources/Text.Adler32.cinch) · line 89

<a id="value-61646c657233322d73746174652d617070656e64"></a>

## `adler32-state-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32-state-append</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32State</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the network-order digest to a copied prefix without changing either argument.

[Source](/reference-sources/Text.Adler32.cinch) · line 124

<a id="value-61646c657233322d73746174652d617070656e642d73617665"></a>

## `adler32-state-append-save`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32-state-append-save</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32State</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append a complete binary checkpoint to a copied prefix, preserving the state.

[Source](/reference-sources/Text.Adler32.cinch) · line 132

<a id="value-61646c657233322d73746174652d626c6f636b2d73697a65"></a>

## `adler32-state-block-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32-state-block-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32State</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Preferred input granularity in bytes, always four; updates accept any chunk length.

[Source](/reference-sources/Text.Adler32.cinch) · line 105

<a id="value-61646c657233322d73746174652d6572726f722d6d657373616765"></a>

## `adler32-state-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32-state-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32StateError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a checkpoint diagnostic with the complete required and actual lengths.

[Source](/reference-sources/Text.Adler32.cinch) · line 143

<a id="value-61646c657233322d73746174652d6c6f6164"></a>

## `adler32-state-load`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32-state-load</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32StateError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32State</span>))</code></pre>

Restore a complete checkpoint without retaining its mutable input buffer.
Identifier checks precede exact-size checks. Every stored word is accepted;
ordinary checksum states should be produced by initialization and updates.

[Source](/reference-sources/Text.Adler32.cinch) · line 151

<a id="value-61646c657233322d73746174652d7265736574"></a>

## `adler32-state-reset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32-state-reset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32State</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32State</span>)</code></pre>

Return the initial accumulator, leaving the supplied state unchanged.

[Source](/reference-sources/Text.Adler32.cinch) · line 97

<a id="value-61646c657233322d73746174652d73617665"></a>

## `adler32-state-save`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32-state-save</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32State</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return an eight-byte checkpoint: ASCII adl, version byte 1, and the network-order checksum.

[Source](/reference-sources/Text.Adler32.cinch) · line 128

<a id="value-61646c657233322d73746174652d73697a65"></a>

## `adler32-state-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32-state-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32State</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Digest size in bytes, always four.

[Source](/reference-sources/Text.Adler32.cinch) · line 101

<a id="value-61646c657233322d73746174652d73756d"></a>

## `adler32-state-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32-state-sum</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32State</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return four fresh checksum bytes in network order without advancing or resetting.

[Source](/reference-sources/Text.Adler32.cinch) · line 113

<a id="value-61646c657233322d73746174652d757064617465"></a>

## `adler32-state-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32-state-update</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32State</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32State</span>))</code></pre>

Consume a complete chunk without changing any earlier retained state or input bytes.

[Source](/reference-sources/Text.Adler32.cinch) · line 93

<a id="value-61646c657233322d73746174652d776f7264"></a>

## `adler32-state-word`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32-state-word</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Adler32</span>.<span class="tok-constructor">Adler32State</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Observe the complete numeric checksum without advancing the state.

[Source](/reference-sources/Text.Adler32.cinch) · line 109

<a id="value-61646c657233322d73756d"></a>

## `adler32-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32-sum</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Checksum all input bytes from the standard initial state 1. Empty input yields 1.

[Source](/reference-sources/Text.Adler32.cinch) · line 25

<a id="value-61646c657233322d757064617465"></a>

## `adler32-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32-update</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

Continue a finished checksum with more bytes. Empty input preserves every supplied bit.
Full segments use explicit machine-word wrapping before modular reduction.

[Source](/reference-sources/Text.Adler32.cinch) · line 30

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Adler32.cinch`. Source SHA-256: `022af63e7649f9677139f4752d8a1a642c9c71ed079bef0ad4af8ad39ecdbdc4`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
