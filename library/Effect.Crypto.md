# Effect.Crypto

Ed25519, X25519 and AES key generation and random-nonce GCM over the operating system random source.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Crypto`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Crypto.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Crypto.Aes](Crypto.Aes.md), [Crypto.Cipher](Crypto.Cipher.md), [Crypto.Ecdsa](Crypto.Ecdsa.md), [Crypto.Ed25519](Crypto.Ed25519.md), [Crypto.Hpke](Crypto.Hpke.md), [Crypto.MlKem](Crypto.MlKem.md), [Crypto.P256](Crypto.P256.md), [Crypto.X25519](Crypto.X25519.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Result](Data.Result.md), [Effect.Random](Effect.Random.md).

Non-embedded source closure: [Crypto.Aes](Crypto.Aes.md), [Crypto.ChaCha20Poly1305](Crypto.ChaCha20Poly1305.md), [Crypto.Cipher](Crypto.Cipher.md), [Crypto.ConstantTime](Crypto.ConstantTime.md), [Crypto.Ecdsa](Crypto.Ecdsa.md), [Crypto.Ed25519](Crypto.Ed25519.md), [Crypto.Field25519](Crypto.Field25519.md), [Crypto.Hmac](Crypto.Hmac.md), [Crypto.Hpke](Crypto.Hpke.md), [Crypto.Kdf](Crypto.Kdf.md), [Crypto.MlKem](Crypto.MlKem.md), [Crypto.P256](Crypto.P256.md), [Crypto.X25519](Crypto.X25519.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Crypto](Effect.Crypto.md), [Effect.Random](Effect.Random.md), [Text.Binary](Text.Binary.md), [Text.Digest](Text.Digest.md), [Text.Sha3](Text.Sha3.md), [Text.Sha512](Text.Sha512.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Key generation and randomized encryption over the operating system's cryptographic random source.
The pure modules Crypto.Ed25519, Crypto.X25519, Crypto.P256, Crypto.Ecdsa, Crypto.MlKem, Crypto.Hpke, Crypto.Aes and Crypto.Cipher take
their secret material as arguments; this module draws it from Effect.Random so
callers never construct keys or nonces from predictable values. Random-nonce
GCM prepends a fresh nonce to every sealed message, which makes nonce reuse
under one key statistically negligible for up to about 2^32 messages.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-6165732d67656e65726174652d6b6579"></a>

## `aes-generate-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">aes-generate-key</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Aes</span>.<span class="tok-constructor">AesKeyError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Aes</span>.<span class="tok-constructor">AesKey</span>))</code></pre>

A new AES key of 16, 24 or 32 random bytes; other sizes are rejected.

[Source](/reference-sources/Effect.Crypto.cinch) · line 73

<a id="value-65636473612d7369676e2d72616e646f6d"></a>

## `ecdsa-sign-random`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ecdsa-sign-random</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PrivateKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Ecdsa</span>.<span class="tok-constructor">EcdsaError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Sign a digest with an RFC 6979 nonce hedged by 32 fresh random bytes (DER encoding).

[Source](/reference-sources/Effect.Crypto.cinch) · line 43

<a id="value-656432353531392d67656e6572617465"></a>

## `ed25519-generate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ed25519-generate</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Ed25519</span>.<span class="tok-constructor">Ed25519PrivateKey</span>)</code></pre>

A new Ed25519 private key from a fresh 32-byte random seed.

[Source](/reference-sources/Effect.Crypto.cinch) · line 25

<a id="value-67636d2d6f70656e2d72616e646f6d"></a>

## `gcm-open-random`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gcm-open-random</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">Gcm</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Open a message produced by gcm-seal-random: split off the leading nonce, then verify and decrypt.

[Source](/reference-sources/Effect.Crypto.cinch) · line 85

<a id="value-67636d2d7365616c2d72616e646f6d"></a>

## `gcm-seal-random`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gcm-seal-random</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">Gcm</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Cipher</span>.<span class="tok-constructor">CipherError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Seal with a fresh random nonce of the instance's nonce size, returning nonce, ciphertext and tag.

[Source](/reference-sources/Effect.Crypto.cinch) · line 77

<a id="value-68706b652d67656e6572617465"></a>

## `hpke-generate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-generate</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeKem</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePrivateKey</span>)</code></pre>

A new HPKE key pair for a KEM, derived from 32 fresh random bytes.

[Source](/reference-sources/Effect.Crypto.cinch) · line 47

<a id="value-68706b652d7365616c2d72616e646f6d"></a>

## `hpke-seal-random`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-seal-random</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeSuite</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePublicKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Bytes</span>)))))))</code></pre>

Single-shot HPKE sealing with a fresh ephemeral key: the encapsulated key and the ciphertext.

[Source](/reference-sources/Effect.Crypto.cinch) · line 57

<a id="value-68706b652d73657475702d73656e6465722d72616e646f6d"></a>

## `hpke-setup-sender-random`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hpke-setup-sender-random</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeSuite</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePublicKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkePrivateKey</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Bytes</span>)) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Hpke</span>.<span class="tok-constructor">HpkeContext</span>)))))))</code></pre>

Set up an HPKE sender context with a fresh ephemeral key: the encapsulated key and the context.

[Source](/reference-sources/Effect.Crypto.cinch) · line 53

<a id="value-6d6c6b656d2d656e63617073756c617465"></a>

## `mlkem-encapsulate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mlkem-encapsulate</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemEncapsulationKey</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Bytes</span>))</code></pre>

Encapsulate to a key with 32 fresh random bytes: the shared key and the ciphertext.

[Source](/reference-sources/Effect.Crypto.cinch) · line 67

<a id="value-6d6c6b656d2d67656e6572617465"></a>

## `mlkem-generate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mlkem-generate</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemParameters</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemDecapsulationKey</span>)</code></pre>

A new ML-KEM decapsulation key from a fresh 64-byte random seed.

[Source](/reference-sources/Effect.Crypto.cinch) · line 61

<a id="value-703235362d67656e6572617465"></a>

## `p256-generate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-generate</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PrivateKey</span>)</code></pre>

A new P-256 private key: 32 random bytes, drawn again until they encode a value in [1, n - 1].

[Source](/reference-sources/Effect.Crypto.cinch) · line 37

<a id="value-7832353531392d67656e6572617465"></a>

## `x25519-generate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">x25519-generate</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X25519</span>.<span class="tok-constructor">X25519PrivateKey</span>)</code></pre>

A new X25519 private key from 32 fresh random bytes.

[Source](/reference-sources/Effect.Crypto.cinch) · line 31

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Crypto.cinch`. Source SHA-256: `957cd112fcd28d0fe72c0849c395aae3e32851e4d48bfac1fa232562ee006725`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
