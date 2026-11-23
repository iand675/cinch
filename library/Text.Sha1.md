# Text.Sha1

SHA-1 for legacy interoperability, with a four-lane SIMD schedule, persistent states and checked binary checkpoints.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Sha1`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Sha1.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Binary](Text.Binary.md), [Text.Sha1](Text.Sha1.md).

<details><summary>Behaviour and boundary notes from the source</summary>

SHA-1 digests, persistent streaming states and binary checkpoints.
SHA-1 is broken for collision resistance: use it only to interoperate with
existing formats and protocols, never for new signatures or integrity checks.
Compression expands four schedule words per step in SIMD lanes and evaluates
each of the eighty ordered rounds once with native rotates. Updates read exact
64-byte blocks while retaining their input owner and copy pending tails into
private storage. Retaining a state branches the computation. Digest and
checkpoint words use network order; byte counts wrap at 64 bits.

</details>

# Types and constructors

<a id="type-536861315374617465"></a>

## `Sha1State`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Sha1State</span></code></pre>

An immutable wrapping byte count, copied partial block and chaining value.

[Source](/reference-sources/Text.Sha1.cinch) · line 38

<a id="type-5368613153746174654572726f72"></a>

## `Sha1StateError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Sha1StateError</span></code></pre>

A checkpoint's identifier or exact byte length is invalid.

[Source](/reference-sources/Text.Sha1.cinch) · line 211

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidSha1StateIdentifier</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1StateError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidSha1StateSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1StateError</span>))</code></pre>

# Values

<a id="value-736861312d626c6f636b2d73697a65"></a>

## `sha1-block-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha1-block-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Compression block size in bytes, always 64.

[Source](/reference-sources/Text.Sha1.cinch) · line 29

<a id="value-736861312d73697a65"></a>

## `sha1-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha1-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Digest size in bytes, always 20.

[Source](/reference-sources/Text.Sha1.cinch) · line 25

<a id="value-736861312d7374617465"></a>

## `sha1-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha1-state</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1State</span></code></pre>

Initial state with no pending bytes.

[Source](/reference-sources/Text.Sha1.cinch) · line 47

<a id="value-736861312d73746174652d617070656e64"></a>

## `sha1-state-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha1-state-append</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1State</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the digest to a copied prefix, preserving the state and caller buffer.

[Source](/reference-sources/Text.Sha1.cinch) · line 191

<a id="value-736861312d73746174652d617070656e642d73617665"></a>

## `sha1-state-append-save`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha1-state-append-save</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1State</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append the complete checkpoint to a copied prefix without mutating either input.

[Source](/reference-sources/Text.Sha1.cinch) · line 207

<a id="value-736861312d73746174652d626c6f636b2d73697a65"></a>

## `sha1-state-block-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha1-state-block-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1State</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Compression block size in bytes, always 64.

[Source](/reference-sources/Text.Sha1.cinch) · line 152

<a id="value-736861312d73746174652d6572726f722d6d657373616765"></a>

## `sha1-state-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha1-state-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1StateError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render identifier errors or complete required and actual checkpoint byte lengths.

[Source](/reference-sources/Text.Sha1.cinch) · line 218

<a id="value-736861312d73746174652d6c6f6164"></a>

## `sha1-state-load`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha1-state-load</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1StateError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1State</span>))</code></pre>

Restore every chaining and count bit, copying only the live partial block into private storage.
Identifier checks precede exact-size checks; unused tail bytes are ignored and saved as zeros.

[Source](/reference-sources/Text.Sha1.cinch) · line 229

<a id="value-736861312d73746174652d7265736574"></a>

## `sha1-state-reset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha1-state-reset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1State</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1State</span>)</code></pre>

Return the initial state, preserving the supplied state.

[Source](/reference-sources/Text.Sha1.cinch) · line 144

<a id="value-736861312d73746174652d73617665"></a>

## `sha1-state-save`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha1-state-save</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1State</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return 96 checkpoint bytes: ASCII sha, version byte 1, chaining words, zero-padded tail and count.
Every numeric field uses network order.

[Source](/reference-sources/Text.Sha1.cinch) · line 196

<a id="value-736861312d73746174652d73697a65"></a>

## `sha1-state-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha1-state-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1State</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Digest size in bytes, always 20.

[Source](/reference-sources/Text.Sha1.cinch) · line 148

<a id="value-736861312d73746174652d73756d"></a>

## `sha1-state-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha1-state-sum</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1State</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return the network-order digest in fresh storage without changing the state.

[Source](/reference-sources/Text.Sha1.cinch) · line 186

<a id="value-736861312d73746174652d757064617465"></a>

## `sha1-state-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha1-state-update</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1State</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Sha1</span>.<span class="tok-constructor">Sha1State</span>))</code></pre>

Consume a complete chunk without mutating any previous state or retaining the input buffer.
Empty chunks preserve the supplied state. Counts use explicit unsigned 64-bit wrapping.

[Source](/reference-sources/Text.Sha1.cinch) · line 61

<a id="value-736861312d73756d"></a>

## `sha1-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sha1-sum</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Compute a fresh 20-byte network-order digest of a complete input.

[Source](/reference-sources/Text.Sha1.cinch) · line 51

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Sha1.cinch`. Source SHA-256: `7c4c6ab9c67ff50be3b4273fd01b709a4ad61b5a0943835f38b438f1142ea095`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
