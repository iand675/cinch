# Crypto.X25519

Constant-time X25519 Diffie-Hellman key agreement with low-order peer rejection.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.X25519`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.X25519.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Crypto.Field25519](Crypto.Field25519.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

Non-embedded source closure: [Crypto.Field25519](Crypto.Field25519.md), [Crypto.X25519](Crypto.X25519.md), [Data.Bytes.Word](Data.Bytes.Word.md).

<details><summary>Behaviour and boundary notes from the source</summary>

X25519 Diffie-Hellman key agreement (RFC 7748) in constant time.
A private key is 32 secret bytes, clamped as the function requires; the
public key is the private key applied to the base point u = 9. Scalar
multiplication is the Montgomery ladder over the u-coordinate with masked
conditional swaps and a fixed iteration count, so its timing and memory
accesses do not depend on the secret. A shared secret of all zero bytes,
produced by low-order peer keys, is rejected. Private keys should come from
a cryptographic random source such as Effect.Random.

</details>

# Types and constructors

<a id="type-5832353531394572726f72"></a>

## `X25519Error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">X25519Error</span></code></pre>

A rejected key or a degenerate shared secret.

[Source](/reference-sources/Crypto.X25519.cinch) · line 27

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidX25519KeySize</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X25519</span>.<span class="tok-constructor">X25519Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidX25519PeerSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X25519</span>.<span class="tok-constructor">X25519Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">LowOrderPeer</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X25519</span>.<span class="tok-constructor">X25519Error</span></code></pre>

<a id="type-583235353139507269766174654b6579"></a>

## `X25519PrivateKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">X25519PrivateKey</span></code></pre>

A private key: its 32 secret bytes (copied) and the derived public key.

[Source](/reference-sources/Crypto.X25519.cinch) · line 43

# Values

<a id="value-783235353139"></a>

## `x25519`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">x25519</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X25519</span>.<span class="tok-constructor">X25519Error</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

The raw function X25519(scalar, u) with clamping, rejecting an all-zero result.

[Source](/reference-sources/Crypto.X25519.cinch) · line 67

<a id="value-7832353531392d62617365"></a>

## `x25519-base`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">x25519-base</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X25519</span>.<span class="tok-constructor">X25519Error</span> <span class="tok-constructor">Bytes</span>))</code></pre>

X25519(scalar, 9): the public key for a private scalar.

[Source](/reference-sources/Crypto.X25519.cinch) · line 76

<a id="value-7832353531392d6572726f722d6d657373616765"></a>

## `x25519-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">x25519-error-message</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X25519</span>.<span class="tok-constructor">X25519Error</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an X25519 error.

[Source](/reference-sources/Crypto.X25519.cinch) · line 36

<a id="value-7832353531392d6b65792d73697a65"></a>

## `x25519-key-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">x25519-key-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Key, scalar and coordinate size in bytes, always 32.

[Source](/reference-sources/Crypto.X25519.cinch) · line 23

<a id="value-7832353531392d707269766174652d6279746573"></a>

## `x25519-private-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">x25519-private-bytes</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X25519</span>.<span class="tok-constructor">X25519PrivateKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The private key's secret bytes, in fresh storage.

[Source](/reference-sources/Crypto.X25519.cinch) · line 55

<a id="value-7832353531392d707269766174652d6b6579"></a>

## `x25519-private-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">x25519-private-key</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X25519</span>.<span class="tok-constructor">X25519Error</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X25519</span>.<span class="tok-constructor">X25519PrivateKey</span>))</code></pre>

Accept 32 secret bytes as a private key and derive its public key.

[Source](/reference-sources/Crypto.X25519.cinch) · line 48

<a id="value-7832353531392d7075626c69632d6b6579"></a>

## `x25519-public-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">x25519-public-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X25519</span>.<span class="tok-constructor">X25519PrivateKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The 32-byte public key, in fresh storage.

[Source](/reference-sources/Crypto.X25519.cinch) · line 59

<a id="value-7832353531392d736861726564"></a>

## `x25519-shared`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">x25519-shared</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X25519</span>.<span class="tok-constructor">X25519PrivateKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X25519</span>.<span class="tok-constructor">X25519Error</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

The shared secret with a peer's 32-byte public key; low-order peers are rejected.

[Source](/reference-sources/Crypto.X25519.cinch) · line 63

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/X25519.cinch`. Source SHA-256: `84f0ecaa26c3903f7cc622b5552258f3fd3736112d773307ed8ccd3e73958ca3`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
