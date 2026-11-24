# Text.Md5

MD5 for legacy interoperability, with persistent states, little-endian digests and checked binary checkpoints.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Md5`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Md5.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Text.Binary](Text.Binary.md).

Non-embedded source closure: [Text.Binary](Text.Binary.md), [Text.Md5](Text.Md5.md).

<details><summary>Behaviour and boundary notes from the source</summary>

MD5 digests, persistent streaming states and binary checkpoints.
MD5 is broken for collision resistance: use it only to interoperate with
existing formats and protocols, never for signatures or integrity checks.
Compression loads checked 64-byte little-endian blocks directly from the input
buffer and evaluates all sixty-four sequential rounds in registers with native
rotates. Updates copy pending tails into private storage. Retaining a state
branches the computation. Digest words and the length field use little-endian
order as the algorithm defines; checkpoint fields use network order.

</details>

# Types and constructors

<a id="type-4d64355374617465"></a>

## `Md5State`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Md5State</span></code></pre>

An immutable wrapping byte count, copied partial block and chaining value.

[Source](/reference-sources/Text.Md5.cinch) · line 38

<a id="type-4d643553746174654572726f72"></a>

## `Md5StateError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Md5StateError</span></code></pre>

A checkpoint's identifier or exact byte length is invalid.

[Source](/reference-sources/Text.Md5.cinch) · line 183

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidMd5StateIdentifier</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Md5</span>.<span class="tok-constructor">Md5StateError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidMd5StateSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Md5</span>.<span class="tok-constructor">Md5StateError</span>))</code></pre>

# Values

<a id="value-6d64352d626c6f636b2d73697a65"></a>

## `md5-block-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">md5-block-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Compression block size in bytes, always 64.

[Source](/reference-sources/Text.Md5.cinch) · line 29

<a id="value-6d64352d73697a65"></a>

## `md5-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">md5-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Digest size in bytes, always 16.

[Source](/reference-sources/Text.Md5.cinch) · line 25

<a id="value-6d64352d7374617465"></a>

## `md5-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">md5-state</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Md5</span>.<span class="tok-constructor">Md5State</span></code></pre>

Initial state with no pending bytes.

[Source](/reference-sources/Text.Md5.cinch) · line 47

<a id="value-6d64352d73746174652d617070656e64"></a>

## `md5-state-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">md5-state-append</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Md5</span>.<span class="tok-constructor">Md5State</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the digest to a copied prefix, preserving the state and caller buffer.

[Source](/reference-sources/Text.Md5.cinch) · line 163

<a id="value-6d64352d73746174652d617070656e642d73617665"></a>

## `md5-state-append-save`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">md5-state-append-save</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Md5</span>.<span class="tok-constructor">Md5State</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the complete checkpoint to a copied prefix without mutating either input.

[Source](/reference-sources/Text.Md5.cinch) · line 179

<a id="value-6d64352d73746174652d626c6f636b2d73697a65"></a>

## `md5-state-block-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">md5-state-block-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Md5</span>.<span class="tok-constructor">Md5State</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Compression block size in bytes, always 64.

[Source](/reference-sources/Text.Md5.cinch) · line 116

<a id="value-6d64352d73746174652d6572726f722d6d657373616765"></a>

## `md5-state-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">md5-state-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Md5</span>.<span class="tok-constructor">Md5StateError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render identifier errors or complete required and actual checkpoint byte lengths.

[Source](/reference-sources/Text.Md5.cinch) · line 190

<a id="value-6d64352d73746174652d6c6f6164"></a>

## `md5-state-load`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">md5-state-load</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Md5</span>.<span class="tok-constructor">Md5StateError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Md5</span>.<span class="tok-constructor">Md5State</span>))</code></pre>

Restore every chaining and count bit, copying only the live partial block into private storage.
Identifier checks precede exact-size checks; unused tail bytes are ignored and saved as zeros.

[Source](/reference-sources/Text.Md5.cinch) · line 201

<a id="value-6d64352d73746174652d7265736574"></a>

## `md5-state-reset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">md5-state-reset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Md5</span>.<span class="tok-constructor">Md5State</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Md5</span>.<span class="tok-constructor">Md5State</span>)</code></pre>

Return the initial state, preserving the supplied state.

[Source](/reference-sources/Text.Md5.cinch) · line 108

<a id="value-6d64352d73746174652d73617665"></a>

## `md5-state-save`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">md5-state-save</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Md5</span>.<span class="tok-constructor">Md5State</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return 92 checkpoint bytes: ASCII md5, version byte 1, chaining words, zero-padded tail and count.
Every numeric field uses network order.

[Source](/reference-sources/Text.Md5.cinch) · line 168

<a id="value-6d64352d73746174652d73697a65"></a>

## `md5-state-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">md5-state-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Md5</span>.<span class="tok-constructor">Md5State</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Digest size in bytes, always 16.

[Source](/reference-sources/Text.Md5.cinch) · line 112

<a id="value-6d64352d73746174652d73756d"></a>

## `md5-state-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">md5-state-sum</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Md5</span>.<span class="tok-constructor">Md5State</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return the digest in fresh storage without changing the state.

[Source](/reference-sources/Text.Md5.cinch) · line 158

<a id="value-6d64352d73746174652d757064617465"></a>

## `md5-state-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">md5-state-update</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Md5</span>.<span class="tok-constructor">Md5State</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Md5</span>.<span class="tok-constructor">Md5State</span>))</code></pre>

Consume a complete chunk without mutating any previous state or retaining the input buffer.
Empty chunks preserve the supplied state. Counts use explicit unsigned 64-bit wrapping.

[Source](/reference-sources/Text.Md5.cinch) · line 61

<a id="value-6d64352d73756d"></a>

## `md5-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">md5-sum</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Compute a fresh 16-byte digest of a complete input.

[Source](/reference-sources/Text.Md5.cinch) · line 51

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Md5.cinch`. Source SHA-256: `806945ac4ce172a564958c978fb92064ccb1f422ec2bec5c5f276fb38931ced3`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
