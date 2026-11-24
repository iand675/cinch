# Crypto.ChaCha20Poly1305

ChaCha20 and XChaCha20 streams, Poly1305 and the ChaCha20-Poly1305 and XChaCha20-Poly1305 AEADs with vector row kernels.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.ChaCha20Poly1305`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.ChaCha20Poly1305.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Crypto.ConstantTime](Crypto.ConstantTime.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md).

Non-embedded source closure: [Crypto.ChaCha20Poly1305](Crypto.ChaCha20Poly1305.md), [Crypto.ConstantTime](Crypto.ConstantTime.md).

<details><summary>Behaviour and boundary notes from the source</summary>

ChaCha20 and XChaCha20 stream encryption, the Poly1305 one-time authenticator, and the
ChaCha20-Poly1305 and XChaCha20-Poly1305 authenticated ciphers (RFC 8439 and the extended-nonce
construction through HChaCha20).

A ChaCha20 keystream block is the state after twenty rounds plus the initial state, computed in a single
pass as four <4 x i32> SIMD rows via Data.Wide: column rounds are vector quarter rounds and diagonal rounds
rotate rows one, two and three lanes with endian-portable 128-bit rotations. Poly1305 keeps its accumulator
in radix-2^64 limbs (h0, h1, h2 <= 4), multiplies via 64x64->128-bit Data.Wide products, reduces modulo
2^130 - 5 once per block and fully at the end in constant time with no data-dependent branches. Tags are
compared in constant time and nothing is decrypted before the tag verifies. Every returned buffer is fresh.

</details>

# Types and constructors

<a id="type-4368614368614572726f72"></a>

## `ChaChaError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ChaChaError</span></code></pre>

Why a ChaCha20 or Poly1305 operation was refused.

[Source](/reference-sources/Crypto.ChaCha20Poly1305.cinch) · line 28

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ChaChaKeySize</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">ChaCha20Poly1305</span>.<span class="tok-constructor">ChaChaError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ChaChaNonceSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">ChaCha20Poly1305</span>.<span class="tok-constructor">ChaChaError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ChaChaTooLarge</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">ChaCha20Poly1305</span>.<span class="tok-constructor">ChaChaError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ChaChaAuthentication</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">ChaCha20Poly1305</span>.<span class="tok-constructor">ChaChaError</span></code></pre>

# Values

<a id="value-6368616368612d6572726f722d6d657373616765"></a>

## `chacha-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">chacha-error-message</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">ChaCha20Poly1305</span>.<span class="tok-constructor">ChaChaError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

A readable description of an error.

[Source](/reference-sources/Crypto.ChaCha20Poly1305.cinch) · line 39

<a id="value-6368616368612d6b65792d73697a65"></a>

## `chacha-key-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">chacha-key-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Key bytes.

[Source](/reference-sources/Crypto.ChaCha20Poly1305.cinch) · line 47

<a id="value-6368616368612d6e6f6e63652d73697a65"></a>

## `chacha-nonce-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">chacha-nonce-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

ChaCha20 and ChaCha20-Poly1305 nonce bytes.

[Source](/reference-sources/Crypto.ChaCha20Poly1305.cinch) · line 51

<a id="value-63686163686132302d786f72"></a>

## `chacha20-xor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">chacha20-xor</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">ChaCha20Poly1305</span>.<span class="tok-constructor">ChaChaError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

XOR a message with the ChaCha20 keystream for a 32-byte key and 12-byte nonce, starting at a block counter.

[Source](/reference-sources/Crypto.ChaCha20Poly1305.cinch) · line 134

<a id="value-6368616368613230706f6c79313330352d6f70656e"></a>

## `chacha20poly1305-open`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">chacha20poly1305-open</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">ChaCha20Poly1305</span>.<span class="tok-constructor">ChaChaError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

Verify and decrypt a sealed message; nothing is decrypted unless the tag verifies.

[Source](/reference-sources/Crypto.ChaCha20Poly1305.cinch) · line 566

<a id="value-6368616368613230706f6c79313330352d7365616c"></a>

## `chacha20poly1305-seal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">chacha20poly1305-seal</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">ChaCha20Poly1305</span>.<span class="tok-constructor">ChaChaError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

Encrypt and authenticate: the ciphertext followed by the sixteen-byte tag.

[Source](/reference-sources/Crypto.ChaCha20Poly1305.cinch) · line 549

<a id="value-686368616368613230"></a>

## `hchacha20`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hchacha20</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">ChaCha20Poly1305</span>.<span class="tok-constructor">ChaChaError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

HChaCha20 of a 32-byte key and the first 16 bytes of a nonce: rows 0 and 3 of the state after twenty
rounds without the final addition, as a 32-byte subkey.

[Source](/reference-sources/Crypto.ChaCha20Poly1305.cinch) · line 251

<a id="value-706f6c79313330352d6d6163"></a>

## `poly1305-mac`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">poly1305-mac</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">ChaCha20Poly1305</span>.<span class="tok-constructor">ChaChaError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

The Poly1305 tag of a message under a 32-byte one-time key.

[Source](/reference-sources/Crypto.ChaCha20Poly1305.cinch) · line 312

<a id="value-706f6c79313330352d7461672d73697a65"></a>

## `poly1305-tag-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">poly1305-tag-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Poly1305 tag bytes.

[Source](/reference-sources/Crypto.ChaCha20Poly1305.cinch) · line 59

<a id="value-706f6c79313330352d766572696679"></a>

## `poly1305-verify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">poly1305-verify</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bool</span>)))</code></pre>

Whether a tag authenticates a message under a one-time key, compared in constant time.

[Source](/reference-sources/Crypto.ChaCha20Poly1305.cinch) · line 318

<a id="value-786368616368612d6e6f6e63652d73697a65"></a>

## `xchacha-nonce-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xchacha-nonce-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

XChaCha20 and XChaCha20-Poly1305 nonce bytes.

[Source](/reference-sources/Crypto.ChaCha20Poly1305.cinch) · line 55

<a id="value-7863686163686132302d786f72"></a>

## `xchacha20-xor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xchacha20-xor</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">ChaCha20Poly1305</span>.<span class="tok-constructor">ChaChaError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

XOR with the XChaCha20 keystream of a 32-byte key and 24-byte nonce: ChaCha20 under the HChaCha20 subkey of
the first sixteen nonce bytes, with four zero bytes and the last eight nonce bytes as its nonce.

[Source](/reference-sources/Crypto.ChaCha20Poly1305.cinch) · line 297

<a id="value-786368616368613230706f6c79313330352d6f70656e"></a>

## `xchacha20poly1305-open`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xchacha20poly1305-open</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">ChaCha20Poly1305</span>.<span class="tok-constructor">ChaChaError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

XChaCha20-Poly1305 opening with a 24-byte nonce.

[Source](/reference-sources/Crypto.ChaCha20Poly1305.cinch) · line 589

<a id="value-786368616368613230706f6c79313330352d7365616c"></a>

## `xchacha20poly1305-seal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xchacha20poly1305-seal</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">ChaCha20Poly1305</span>.<span class="tok-constructor">ChaChaError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

XChaCha20-Poly1305 sealing with a 24-byte nonce.

[Source](/reference-sources/Crypto.ChaCha20Poly1305.cinch) · line 582

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/ChaCha20Poly1305.cinch`. Source SHA-256: `134e40672a3dc0e225a2221d4dab7e708c90ff75549b0f048266be9b5c58c870`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
