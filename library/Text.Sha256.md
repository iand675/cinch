# Text.Sha256

Portable SHA-224/SHA-256 with generated SIMD schedules, persistent states, network-order digests and checked binary checkpoints.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Sha256`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Sha256.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Binary](Text.Binary.md), [Text.Sha256](Text.Sha256.md).

<details><summary>Behaviour and boundary notes from the source</summary>

SHA-224 and SHA-256 digests, persistent streaming states and binary checkpoints.
Compression expands four schedule words in SIMD and evaluates every ordered
round once. Updates read exact 64-byte blocks while retaining their input
owner, and copy pending tails into private storage. Retaining a state branches
the computation. Digest and checkpoint words use network order; byte counts
wrap at 64 bits. Checkpoint input and returned buffers never alias the state.

</details>

# Types and constructors

<a id="type-5368613235365374617465"></a>

## `Sha256State`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Sha256State</span></code></pre>

An immutable variant, wrapping byte count, copied partial block and chaining value.

[Source](/reference-sources/Text.Sha256.cinch) · line 41

<a id="type-53686132353653746174654572726f72"></a>

## `Sha256StateError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Sha256StateError</span></code></pre>

A checkpoint's complete variant identifier or exact byte length is invalid.

[Source](/reference-sources/Text.Sha256.cinch) · line 282

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidSha256StateIdentifier</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256StateError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidSha256StateSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256StateError</span>))</code></pre>

# Values

<a id="value-7368613232342d73697a65"></a>

## `sha224-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha224-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

SHA-224 digest size in bytes, always 28.

[Source](/reference-sources/Text.Sha256.cinch) · line 28

<a id="value-7368613232342d7374617465"></a>

## `sha224-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha224-state</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256State</span></code></pre>

Initial SHA-224 state with its distinct FIPS 180-4 chaining value and no pending bytes.

[Source](/reference-sources/Text.Sha256.cinch) · line 50

<a id="value-7368613232342d73756d"></a>

## `sha224-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha224-sum</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Compute a fresh 28-byte network-order SHA-224 digest of a complete input.

[Source](/reference-sources/Text.Sha256.cinch) · line 58

<a id="value-7368613235362d626c6f636b2d73697a65"></a>

## `sha256-block-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha256-block-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Compression block size in bytes for both variants, always 64.

[Source](/reference-sources/Text.Sha256.cinch) · line 32

<a id="value-7368613235362d73697a65"></a>

## `sha256-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha256-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

SHA-256 digest size in bytes, always 32.

[Source](/reference-sources/Text.Sha256.cinch) · line 24

<a id="value-7368613235362d7374617465"></a>

## `sha256-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha256-state</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256State</span></code></pre>

Initial SHA-256 state with the FIPS 180-4 chaining value and no pending bytes.

[Source](/reference-sources/Text.Sha256.cinch) · line 46

<a id="value-7368613235362d73746174652d617070656e64"></a>

## `sha256-state-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha256-state-append</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256State</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the selected digest to a copied prefix, preserving the state and caller buffer.

[Source](/reference-sources/Text.Sha256.cinch) · line 262

<a id="value-7368613235362d73746174652d617070656e642d73617665"></a>

## `sha256-state-append-save`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha256-state-append-save</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256State</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the complete checkpoint to a copied prefix without mutating either input.

[Source](/reference-sources/Text.Sha256.cinch) · line 278

<a id="value-7368613235362d73746174652d626c6f636b2d73697a65"></a>

## `sha256-state-block-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha256-state-block-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256State</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Compression block size in bytes, always 64 for either variant.

[Source](/reference-sources/Text.Sha256.cinch) · line 173

<a id="value-7368613235362d73746174652d6572726f722d6d657373616765"></a>

## `sha256-state-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha256-state-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256StateError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render identifier errors or complete required and actual checkpoint byte lengths.

[Source](/reference-sources/Text.Sha256.cinch) · line 289

<a id="value-7368613235362d73746174652d6c6f6164"></a>

## `sha256-state-load`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha256-state-load</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256State</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256StateError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256State</span>)))</code></pre>

Restore every chaining/count bit, copying only the live partial block into private storage.
The template selects the variant. Identifier checks precede exact-size checks;
unused checkpoint tail bytes are ignored and serialized as zeros on the next save.

[Source](/reference-sources/Text.Sha256.cinch) · line 301

<a id="value-7368613235362d73746174652d7265736574"></a>

## `sha256-state-reset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha256-state-reset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256State</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256State</span>)</code></pre>

Return the initial state for the same variant, preserving the supplied state.

[Source](/reference-sources/Text.Sha256.cinch) · line 161

<a id="value-7368613235362d73746174652d73617665"></a>

## `sha256-state-save`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha256-state-save</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256State</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return 108 checkpoint bytes: sha, variant byte 2/3, chaining words, zero-padded tail and count.
Variant 2 is SHA-224 and variant 3 is SHA-256. Every numeric field uses network order.

[Source](/reference-sources/Text.Sha256.cinch) · line 267

<a id="value-7368613235362d73746174652d73697a65"></a>

## `sha256-state-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha256-state-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256State</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">28</span> <span class="tok-number">32</span> <span class="tok-constructor">Nat</span>))</code></pre>

Digest size in bytes, bounded by 28 and 32 and fixed by the state's selected variant.

[Source](/reference-sources/Text.Sha256.cinch) · line 165

<a id="value-7368613235362d73746174652d73756d"></a>

## `sha256-state-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha256-state-sum</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256State</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return the selected network-order digest in fresh storage without changing the state.

[Source](/reference-sources/Text.Sha256.cinch) · line 226

<a id="value-7368613235362d73746174652d757064617465"></a>

## `sha256-state-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha256-state-update</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256State</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha256</span>.<span class="tok-constructor">Sha256State</span>))</code></pre>

Consume a complete chunk without mutating any previous state or retaining the input buffer.
Empty chunks preserve the supplied state. Counts use explicit unsigned 64-bit wrapping.

[Source](/reference-sources/Text.Sha256.cinch) · line 75

<a id="value-7368613235362d73756d"></a>

## `sha256-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha256-sum</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Compute a fresh 32-byte network-order SHA-256 digest of a complete input.

[Source](/reference-sources/Text.Sha256.cinch) · line 54

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Sha256.cinch`. Source SHA-256: `95e5a6dd624575bf80bf7df22ba632819dd500d4a0c5af4f32d16d4b2befcea8`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
