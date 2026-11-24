# Text.Sha3

SHA3-224/256/384/512, SHAKE128/256 and cSHAKE128/256 over a register-resident Keccak permutation, with persistent sponges, output readers and checkpoints.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Sha3`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Sha3.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Binary](Text.Binary.md), [Text.Sha3](Text.Sha3.md).

<details><summary>Behaviour and boundary notes from the source</summary>

SHA3-224/256/384/512 digests, SHAKE128/256 and cSHAKE128/256 extendable output, persistent sponges and checkpoints.
Keccak-f[1600] runs as generated straight-line code over twenty-five private
64-bit lanes with native rotates; every round's theta, rho, pi and chi are
fused per output lane so the lanes stay in registers. A sponge is sequential
across rounds, so single-message hashing is scalar rather than SIMD. Updates
absorb whole rate-sized blocks directly from the retained input owner and copy
pending tails into private storage. Retaining a state or reader branches the
computation; buffers returned or accepted are never aliased by a state.

</details>

# Types and constructors

<a id="type-536861335374617465"></a>

## `Sha3State`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Sha3State</span></code></pre>

An absorbing sponge: variant code, lanes to restore on reset, copied pending bytes and current lanes.
Codes 1-4 are SHA3-224/256/384/512, 5-6 SHAKE128/256 and 7-8 cSHAKE128/256.
Pending bytes are always fewer than one rate-sized block.

[Source](/reference-sources/Text.Sha3.cinch) · line 35

<a id="type-5368613353746174654572726f72"></a>

## `Sha3StateError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Sha3StateError</span></code></pre>

A checkpoint's identifier, exact byte length or pending length is invalid.

[Source](/reference-sources/Text.Sha3.cinch) · line 341

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidSha3StateIdentifier</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3StateError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidSha3StateSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3StateError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidSha3StateTail</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3StateError</span>)</code></pre>

<a id="type-5368616b65526561646572"></a>

## `ShakeReader`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ShakeReader</span></code></pre>

A squeezing sponge for extendable output: variant code, bytes already read from the current block, lanes.

[Source](/reference-sources/Text.Sha3.cinch) · line 40

# Values

<a id="value-637368616b653132382d7374617465"></a>

## `cshake128-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cshake128-state</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span>))</code></pre>

cSHAKE128 with a function name and customization string. With both empty it is exactly SHAKE128.

[Source](/reference-sources/Text.Sha3.cinch) · line 93

<a id="value-637368616b653235362d7374617465"></a>

## `cshake256-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cshake256-state</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span>))</code></pre>

cSHAKE256 with a function name and customization string. With both empty it is exactly SHAKE256.

[Source](/reference-sources/Text.Sha3.cinch) · line 97

<a id="value-736861332d73746174652d617070656e64"></a>

## `sha3-state-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-state-append</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the digest to a copied prefix, preserving the state and caller buffer.

[Source](/reference-sources/Text.Sha3.cinch) · line 276

<a id="value-736861332d73746174652d617070656e642d73617665"></a>

## `sha3-state-append-save`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-state-append-save</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the complete checkpoint to a copied prefix without mutating either input.

[Source](/reference-sources/Text.Sha3.cinch) · line 337

<a id="value-736861332d73746174652d626c6f636b2d73697a65"></a>

## `sha3-state-block-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-state-block-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Rate in bytes: the block size absorbed per permutation.

[Source](/reference-sources/Text.Sha3.cinch) · line 240

<a id="value-736861332d73746174652d6572726f722d6d657373616765"></a>

## `sha3-state-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-state-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3StateError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render identifier, size and pending-length errors.

[Source](/reference-sources/Text.Sha3.cinch) · line 350

<a id="value-736861332d73746174652d6c6f6164"></a>

## `sha3-state-load`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-state-load</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3StateError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span>)))</code></pre>

Restore a checkpoint for the template's variant, copying only live pending bytes.
Identifier checks precede the exact-size check, which precedes the pending-length check.

[Source](/reference-sources/Text.Sha3.cinch) · line 371

<a id="value-736861332d73746174652d7265736574"></a>

## `sha3-state-reset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-state-reset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span>)</code></pre>

Return to the variant's initial lanes, keeping any cSHAKE customization; the supplied state is unchanged.

[Source](/reference-sources/Text.Sha3.cinch) · line 232

<a id="value-736861332d73746174652d73617665"></a>

## `sha3-state-save`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-state-save</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return 573 checkpoint bytes: ASCII kec, the variant byte, initial lanes, current lanes,
the pending length byte and the pending bytes zero-padded to the largest rate (168).

[Source](/reference-sources/Text.Sha3.cinch) · line 314

<a id="value-736861332d73746174652d73697a65"></a>

## `sha3-state-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-state-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Digest size of a fixed variant, or the default output length (32 or 64) of an extendable one.

[Source](/reference-sources/Text.Sha3.cinch) · line 236

<a id="value-736861332d73746174652d73756d"></a>

## `sha3-state-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-state-sum</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return the fixed-size digest, or the default-length output of an extendable variant, without changing the state.

[Source](/reference-sources/Text.Sha3.cinch) · line 272

<a id="value-736861332d73746174652d757064617465"></a>

## `sha3-state-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-state-update</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span>))</code></pre>

Absorb a complete chunk without mutating any previous state or retaining the input buffer.
Empty chunks preserve the supplied state.

[Source](/reference-sources/Text.Sha3.cinch) · line 152

<a id="value-736861332d7374617465323234"></a>

## `sha3-state224`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-state224</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span></code></pre>

Initial SHA3-224 state.

[Source](/reference-sources/Text.Sha3.cinch) · line 69

<a id="value-736861332d7374617465323536"></a>

## `sha3-state256`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-state256</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span></code></pre>

Initial SHA3-256 state.

[Source](/reference-sources/Text.Sha3.cinch) · line 73

<a id="value-736861332d7374617465333834"></a>

## `sha3-state384`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-state384</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span></code></pre>

Initial SHA3-384 state.

[Source](/reference-sources/Text.Sha3.cinch) · line 77

<a id="value-736861332d7374617465353132"></a>

## `sha3-state512`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-state512</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span></code></pre>

Initial SHA3-512 state.

[Source](/reference-sources/Text.Sha3.cinch) · line 81

<a id="value-736861332d73756d323234"></a>

## `sha3-sum224`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-sum224</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Compute a fresh 28-byte SHA3-224 digest.

[Source](/reference-sources/Text.Sha3.cinch) · line 127

<a id="value-736861332d73756d323536"></a>

## `sha3-sum256`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-sum256</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Compute a fresh 32-byte SHA3-256 digest.

[Source](/reference-sources/Text.Sha3.cinch) · line 131

<a id="value-736861332d73756d333834"></a>

## `sha3-sum384`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-sum384</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Compute a fresh 48-byte SHA3-384 digest.

[Source](/reference-sources/Text.Sha3.cinch) · line 135

<a id="value-736861332d73756d353132"></a>

## `sha3-sum512`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha3-sum512</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Compute a fresh 64-byte SHA3-512 digest.

[Source](/reference-sources/Text.Sha3.cinch) · line 139

<a id="value-7368616b652d72656164"></a>

## `shake-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shake-read</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">ShakeReader</span>)))</code></pre>

Finalize a copy of the sponge and read its first output bytes; the reader continues the stream.
Fixed-size variants may be read past their digest size, which yields their underlying sponge output.

[Source](/reference-sources/Text.Sha3.cinch) · line 281

<a id="value-7368616b652d7265616465722d72656164"></a>

## `shake-reader-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shake-reader-read</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">ShakeReader</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">ShakeReader</span>)))</code></pre>

Read the next output bytes, permuting whenever a rate-sized block is exhausted.

[Source](/reference-sources/Text.Sha3.cinch) · line 286

<a id="value-7368616b653132382d7374617465"></a>

## `shake128-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shake128-state</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span></code></pre>

Initial SHAKE128 state, with 128-bit security and a 168-byte rate.

[Source](/reference-sources/Text.Sha3.cinch) · line 85

<a id="value-7368616b653132382d73756d"></a>

## `shake128-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shake128-sum</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Read exactly the requested number of SHAKE128 output bytes for a complete input.

[Source](/reference-sources/Text.Sha3.cinch) · line 143

<a id="value-7368616b653235362d7374617465"></a>

## `shake256-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shake256-state</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha3</span>.<span class="tok-constructor">Sha3State</span></code></pre>

Initial SHAKE256 state, with 256-bit security and a 136-byte rate.

[Source](/reference-sources/Text.Sha3.cinch) · line 89

<a id="value-7368616b653235362d73756d"></a>

## `shake256-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">shake256-sum</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Read exactly the requested number of SHAKE256 output bytes for a complete input.

[Source](/reference-sources/Text.Sha3.cinch) · line 147

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Sha3.cinch`. Source SHA-256: `26c3d9fe682494d2a244a66098253c9a9a0871eb5bbc1f2405c8b9f99c56b30c`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
