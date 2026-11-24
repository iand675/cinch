# Text.Sha512

Portable SHA-384, SHA-512, SHA-512/224 and SHA-512/256 with two-lane SIMD schedules, native rotates, persistent states and checked binary checkpoints.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Sha512`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Sha512.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Binary](Text.Binary.md), [Text.Sha512](Text.Sha512.md).

<details><summary>Behaviour and boundary notes from the source</summary>

SHA-384, SHA-512, SHA-512/224 and SHA-512/256 digests, persistent streaming states and binary checkpoints.
Compression expands two schedule words per step in independent SIMD lanes and
evaluates every one of the eighty ordered rounds exactly once with native
64-bit rotates. Updates read exact 128-byte blocks while retaining their input
owner, and copy pending tails into private storage. Retaining a state branches
the computation. Digest and checkpoint words use network order. Byte counts
wrap at 64 bits, so the encoded 128-bit message length is exact for every
input shorter than 2^64 bytes. Checkpoint input and returned buffers never
alias the state.

</details>

# Types and constructors

<a id="type-5368613531325374617465"></a>

## `Sha512State`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Sha512State</span></code></pre>

An immutable variant code, wrapping byte count, copied partial block and chaining value.

[Source](/reference-sources/Text.Sha512.cinch) · line 53

<a id="type-53686135313253746174654572726f72"></a>

## `Sha512StateError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Sha512StateError</span></code></pre>

A checkpoint's complete variant identifier or exact byte length is invalid.

[Source](/reference-sources/Text.Sha512.cinch) · line 293

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidSha512StateIdentifier</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512StateError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidSha512StateSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512StateError</span>))</code></pre>

# Values

<a id="value-7368613338342d73697a65"></a>

## `sha384-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha384-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

SHA-384 digest size in bytes, always 48.

[Source](/reference-sources/Text.Sha512.cinch) · line 32

<a id="value-7368613338342d7374617465"></a>

## `sha384-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha384-state</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512State</span></code></pre>

Initial SHA-384 state, with its distinct chaining value.

[Source](/reference-sources/Text.Sha512.cinch) · line 67

<a id="value-7368613338342d73756d"></a>

## `sha384-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha384-sum</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Compute a fresh 48-byte network-order SHA-384 digest of a complete input.

[Source](/reference-sources/Text.Sha512.cinch) · line 83

<a id="value-7368613531322d626c6f636b2d73697a65"></a>

## `sha512-block-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512-block-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Compression block size in bytes for every variant, always 128.

[Source](/reference-sources/Text.Sha512.cinch) · line 44

<a id="value-7368613531322d73697a65"></a>

## `sha512-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

SHA-512 digest size in bytes, always 64.

[Source](/reference-sources/Text.Sha512.cinch) · line 28

<a id="value-7368613531322d7374617465"></a>

## `sha512-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512-state</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512State</span></code></pre>

Initial SHA-512 state.

[Source](/reference-sources/Text.Sha512.cinch) · line 63

<a id="value-7368613531322d73746174652d617070656e64"></a>

## `sha512-state-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512-state-append</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512State</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the selected digest to a copied prefix, preserving the state and caller buffer.

[Source](/reference-sources/Text.Sha512.cinch) · line 269

<a id="value-7368613531322d73746174652d617070656e642d73617665"></a>

## `sha512-state-append-save`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512-state-append-save</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512State</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the complete checkpoint to a copied prefix without mutating either input.

[Source](/reference-sources/Text.Sha512.cinch) · line 289

<a id="value-7368613531322d73746174652d626c6f636b2d73697a65"></a>

## `sha512-state-block-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512-state-block-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512State</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Compression block size in bytes, always 128 for every variant.

[Source](/reference-sources/Text.Sha512.cinch) · line 208

<a id="value-7368613531322d73746174652d6572726f722d6d657373616765"></a>

## `sha512-state-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512-state-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512StateError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render identifier errors or complete required and actual checkpoint byte lengths.

[Source](/reference-sources/Text.Sha512.cinch) · line 300

<a id="value-7368613531322d73746174652d6c6f6164"></a>

## `sha512-state-load`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512-state-load</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512State</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512StateError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512State</span>)))</code></pre>

Restore every chaining and count bit, copying only the live partial block into private storage.
The template selects the variant. Identifier checks precede exact-size checks; unused
checkpoint tail bytes are ignored and serialized as zeros on the next save.

[Source](/reference-sources/Text.Sha512.cinch) · line 312

<a id="value-7368613531322d73746174652d7265736574"></a>

## `sha512-state-reset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512-state-reset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512State</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512State</span>)</code></pre>

Return the initial state for the same variant, preserving the supplied state.

[Source](/reference-sources/Text.Sha512.cinch) · line 195

<a id="value-7368613531322d73746174652d73617665"></a>

## `sha512-state-save`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512-state-save</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512State</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return 204 checkpoint bytes: ASCII sha, the variant byte, chaining words, zero-padded tail and count.
Variant bytes 4, 5, 6 and 7 are SHA-384, SHA-512/224, SHA-512/256 and SHA-512. Numbers use network order.

[Source](/reference-sources/Text.Sha512.cinch) · line 274

<a id="value-7368613531322d73746174652d73697a65"></a>

## `sha512-state-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512-state-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512State</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Digest size in bytes for the state's variant: 48, 28, 32 or 64.

[Source](/reference-sources/Text.Sha512.cinch) · line 199

<a id="value-7368613531322d73746174652d73756d"></a>

## `sha512-state-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512-state-sum</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512State</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return the selected network-order digest in fresh storage without changing the state.

[Source](/reference-sources/Text.Sha512.cinch) · line 263

<a id="value-7368613531322d73746174652d757064617465"></a>

## `sha512-state-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512-state-update</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512State</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512State</span>))</code></pre>

Consume a complete chunk without mutating any previous state or retaining the input buffer.
Empty chunks preserve the supplied state. Counts use explicit unsigned 64-bit wrapping.

[Source](/reference-sources/Text.Sha512.cinch) · line 106

<a id="value-7368613531322d73756d"></a>

## `sha512-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512-sum</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Compute a fresh 64-byte network-order SHA-512 digest of a complete input.

[Source](/reference-sources/Text.Sha512.cinch) · line 79

<a id="value-736861353132743232342d73697a65"></a>

## `sha512t224-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512t224-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

SHA-512/224 digest size in bytes, always 28.

[Source](/reference-sources/Text.Sha512.cinch) · line 36

<a id="value-736861353132743232342d7374617465"></a>

## `sha512t224-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512t224-state</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512State</span></code></pre>

Initial SHA-512/224 state, whose chaining value comes from the standard generation function.

[Source](/reference-sources/Text.Sha512.cinch) · line 71

<a id="value-736861353132743232342d73756d"></a>

## `sha512t224-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512t224-sum</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Compute a fresh 28-byte network-order SHA-512/224 digest of a complete input.

[Source](/reference-sources/Text.Sha512.cinch) · line 87

<a id="value-736861353132743235362d73697a65"></a>

## `sha512t256-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512t256-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

SHA-512/256 digest size in bytes, always 32.

[Source](/reference-sources/Text.Sha512.cinch) · line 40

<a id="value-736861353132743235362d7374617465"></a>

## `sha512t256-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512t256-state</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha512</span>.<span class="tok-constructor">Sha512State</span></code></pre>

Initial SHA-512/256 state, whose chaining value comes from the standard generation function.

[Source](/reference-sources/Text.Sha512.cinch) · line 75

<a id="value-736861353132743235362d73756d"></a>

## `sha512t256-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha512t256-sum</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Compute a fresh 32-byte network-order SHA-512/256 digest of a complete input.

[Source](/reference-sources/Text.Sha512.cinch) · line 91

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Sha512.cinch`. Source SHA-256: `545a627c6c71aeb06162bd87ff5e124d792cc8e6280af48cac1177a75de37317`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
