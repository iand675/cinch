# Crypto.Ed25519

Deterministic Ed25519 signatures with constant-time signing and strict verification.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.Ed25519`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.Ed25519.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Crypto.Field25519](Crypto.Field25519.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Text.Sha512](Text.Sha512.md).

Non-embedded source closure: [Crypto.Ed25519](Crypto.Ed25519.md), [Crypto.Field25519](Crypto.Field25519.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Binary](Text.Binary.md), [Text.Sha512](Text.Sha512.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Ed25519 signatures (RFC 8032) with constant-time key handling and signing.
A private key is a 32-byte seed; signing hashes it with SHA-512 into a
clamped secret scalar and a nonce prefix, so signatures are deterministic.
Points use extended twisted Edwards coordinates with complete addition
formulas; scalar multiplication always doubles and adds and selects with
masks, and scalar arithmetic modulo the group order L is bit-serial with a
single masked conditional subtraction per bit, so secret values never drive
branches or memory addresses. Verification decodes the public key, rejects
signatures whose S is not below L, and compares the encoding of
[S]B - [k]A with R, the cofactorless check most implementations use.

</details>

# Types and constructors

<a id="type-456432353531394572726f72"></a>

## `Ed25519Error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Ed25519Error</span></code></pre>

A rejected seed.

[Source](/reference-sources/Crypto.Ed25519.cinch) · line 40

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidEd25519SeedSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Ed25519</span>.<span class="tok-constructor">Ed25519Error</span>)</code></pre>

<a id="type-45643235353139507269766174654b6579"></a>

## `Ed25519PrivateKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Ed25519PrivateKey</span></code></pre>

An expanded private key: the seed, the clamped secret scalar bytes, the nonce prefix and the public key.

[Source](/reference-sources/Crypto.Ed25519.cinch) · line 50

# Values

<a id="value-656432353531392d6572726f722d6d657373616765"></a>

## `ed25519-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ed25519-error-message</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Ed25519</span>.<span class="tok-constructor">Ed25519Error</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an Ed25519 error.

[Source](/reference-sources/Crypto.Ed25519.cinch) · line 45

<a id="value-656432353531392d707269766174652d6b6579"></a>

## `ed25519-private-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ed25519-private-key</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Ed25519</span>.<span class="tok-constructor">Ed25519Error</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Ed25519</span>.<span class="tok-constructor">Ed25519PrivateKey</span>))</code></pre>

Expand a 32-byte seed into a private key and derive its public key.

[Source](/reference-sources/Crypto.Ed25519.cinch) · line 55

<a id="value-656432353531392d7075626c69632d6b6579"></a>

## `ed25519-public-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ed25519-public-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Ed25519</span>.<span class="tok-constructor">Ed25519PrivateKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The 32-byte public key, in fresh storage.

[Source](/reference-sources/Crypto.Ed25519.cinch) · line 68

<a id="value-656432353531392d7075626c69632d6b65792d73697a65"></a>

## `ed25519-public-key-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ed25519-public-key-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Public key size in bytes, 32.

[Source](/reference-sources/Crypto.Ed25519.cinch) · line 32

<a id="value-656432353531392d73656564"></a>

## `ed25519-seed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ed25519-seed</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Ed25519</span>.<span class="tok-constructor">Ed25519PrivateKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The seed, in fresh storage.

[Source](/reference-sources/Crypto.Ed25519.cinch) · line 64

<a id="value-656432353531392d736565642d73697a65"></a>

## `ed25519-seed-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ed25519-seed-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Seed size in bytes, 32.

[Source](/reference-sources/Crypto.Ed25519.cinch) · line 28

<a id="value-656432353531392d7369672d73697a65"></a>

## `ed25519-sig-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ed25519-sig-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Signature size in bytes, 64.

[Source](/reference-sources/Crypto.Ed25519.cinch) · line 36

<a id="value-656432353531392d7369676e"></a>

## `ed25519-sign`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ed25519-sign</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Ed25519</span>.<span class="tok-constructor">Ed25519PrivateKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Sign a message: R = [r]B for r = SHA-512(prefix || message) mod L, and S = r + k s mod L
for k = SHA-512(R || public key || message) mod L. Returns the 64-byte signature R || S.

[Source](/reference-sources/Crypto.Ed25519.cinch) · line 73

<a id="value-656432353531392d766572696679"></a>

## `ed25519-verify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ed25519-verify</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bool</span>)))</code></pre>

Verify a 64-byte signature of a message under a 32-byte public key.

[Source](/reference-sources/Crypto.Ed25519.cinch) · line 83

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/Ed25519.cinch`. Source SHA-256: `c9653a1c9514393759e3c07a4a826ff1fdf856d9dcedd5eb4ac509130350c707`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
