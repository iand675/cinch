# Crypto.MlDsa

FIPS 204 ML-DSA-44, ML-DSA-65, and ML-DSA-87 post-quantum digital signatures: deterministic and hedged signing, verification, and key/signature codecs.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.MlDsa`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.MlDsa.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Crypto.ConstantTime](Crypto.ConstantTime.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Sha3](Text.Sha3.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Crypto.ConstantTime](Crypto.ConstantTime.md), [Crypto.MlDsa](Crypto.MlDsa.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Text.Binary](Text.Binary.md), [Text.Sha3](Text.Sha3.md).

<details><summary>Behaviour and boundary notes from the source</summary>

ML-DSA (FIPS 204), the module-lattice digital signature algorithm, with the ML-DSA-44, ML-DSA-65, and ML-DSA-87 parameter sets.
Key pairs are derived deterministically from a 32-byte seed or loaded from
encoded FIPS 204 private and public key bytes. Signing supports both
deterministic mode and hedged mode with 32 bytes of caller-supplied
randomness, using the pure ML-DSA domain framing (0x00 || len(ctx) || ctx || M)
with context strings up to 255 bytes. Polynomials are stored as 1024-byte
buffers of 256 little-endian 32-bit coefficients reduced modulo q = 8380417,
using exact 64-bit modular arithmetic and the FIPS 204 256-point NTT.

</details>

# Types and constructors

<a id="type-4d6c4473614572726f72"></a>

## `MlDsaError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MlDsaError</span></code></pre>

A rejected ML-DSA input or signing failure.

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 104

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlDsaInvalidSeedSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlDsaInvalidPublicKey</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlDsaInvalidPrivateKey</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlDsaInvalidSignature</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlDsaInvalidRandomnessSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlDsaContextTooLong</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlDsaSigningFailed</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaError</span></code></pre>

<a id="type-4d6c447361506172616d6574657273"></a>

## `MlDsaParameters`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MlDsaParameters</span></code></pre>

An ML-DSA parameter set from FIPS 204 Table 1.

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 35

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlDsa44</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaParameters</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlDsa65</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaParameters</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlDsa87</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaParameters</span></code></pre>

<a id="type-4d6c447361507269766174654b6579"></a>

## `MlDsaPrivateKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MlDsaPrivateKey</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 143

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlDsaPrivateKey</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaPublicKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaPrivateKey</span>))))))</code></pre>

<a id="type-4d6c4473615075626c69634b6579"></a>

## `MlDsaPublicKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MlDsaPublicKey</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 138

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlDsaPublicKey</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaParameters</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaPublicKey</span>)))))</code></pre>

# Values

<a id="value-6d6c6473612d6572726f722d6d657373616765"></a>

## `mldsa-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mldsa-error-message</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an ML-DSA error as a diagnostic message.

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 121

<a id="value-6d6c6473612d6b65792d66726f6d2d73656564"></a>

## `mldsa-key-from-seed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mldsa-key-from-seed</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaParameters</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaPrivateKey</span>)))</code></pre>

Derive an ML-DSA key pair from a 32-byte seed xi (FIPS 204 Algorithm 6).

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 166

<a id="value-6d6c6473612d707269766174652d6b6579"></a>

## `mldsa-private-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mldsa-private-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaParameters</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaPrivateKey</span>)))</code></pre>

Parse and validate an encoded ML-DSA private key (FIPS 204 skDecode), checking secret bounds and public-key consistency.

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 208

<a id="value-6d6c6473612d707269766174652d6b65792d6279746573"></a>

## `mldsa-private-key-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mldsa-private-key-bytes</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaPrivateKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return a fresh copy of an ML-DSA private key's encoded bytes.

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 154

<a id="value-6d6c6473612d707269766174652d6b65792d706172616d6574657273"></a>

## `mldsa-private-key-parameters`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mldsa-private-key-parameters</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaPrivateKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaParameters</span>)</code></pre>

Return the parameter set of an ML-DSA private key.

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 158

<a id="value-6d6c6473612d707269766174652d6b65792d73697a65"></a>

## `mldsa-private-key-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mldsa-private-key-size</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaParameters</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Encoded private key size in bytes: 2560 for ML-DSA-44, 4032 for ML-DSA-65, 4896 for ML-DSA-87.

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 96

<a id="value-6d6c6473612d7075626c69632d6b6579"></a>

## `mldsa-public-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mldsa-public-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaParameters</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaPublicKey</span>)))</code></pre>

Parse and validate an encoded ML-DSA public key (FIPS 204 pkDecode).

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 194

<a id="value-6d6c6473612d7075626c69632d6b65792d6279746573"></a>

## `mldsa-public-key-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mldsa-public-key-bytes</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaPublicKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return a fresh copy of an ML-DSA public key's encoded bytes.

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 150

<a id="value-6d6c6473612d7075626c69632d6b65792d6f66"></a>

## `mldsa-public-key-of`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mldsa-public-key-of</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaPrivateKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaPublicKey</span>)</code></pre>

Return the public key corresponding to an ML-DSA private key.

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 162

<a id="value-6d6c6473612d7075626c69632d6b65792d706172616d6574657273"></a>

## `mldsa-public-key-parameters`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mldsa-public-key-parameters</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaPublicKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaParameters</span>)</code></pre>

Return the parameter set of an ML-DSA public key.

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 146

<a id="value-6d6c6473612d7075626c69632d6b65792d73697a65"></a>

## `mldsa-public-key-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mldsa-public-key-size</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaParameters</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Encoded public key size in bytes: 1312 for ML-DSA-44, 1952 for ML-DSA-65, 2592 for ML-DSA-87.

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 92

<a id="value-6d6c6473612d736565642d73697a65"></a>

## `mldsa-seed-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mldsa-seed-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Key-generation seed size in bytes (32).

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 88

<a id="value-6d6c6473612d7369676e2d64657465726d696e6973746963"></a>

## `mldsa-sign-deterministic`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mldsa-sign-deterministic</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaPrivateKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Produce a deterministic pure ML-DSA signature (FIPS 204 Algorithms 2 and 7 with rnd = 0^32).

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 255

<a id="value-6d6c6473612d7369676e2d77697468"></a>

## `mldsa-sign-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mldsa-sign-with</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaPrivateKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

Produce a pure ML-DSA signature using 32 bytes of explicit randomness (FIPS 204 Algorithms 2 and 7).

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 259

<a id="value-6d6c6473612d7369676e61747572652d73697a65"></a>

## `mldsa-signature-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mldsa-signature-size</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaParameters</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Encoded signature size in bytes: 2420 for ML-DSA-44, 3309 for ML-DSA-65, 4627 for ML-DSA-87.

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 100

<a id="value-6d6c6473612d766572696679"></a>

## `mldsa-verify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mldsa-verify</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlDsa</span>.<span class="tok-constructor">MlDsaPublicKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bool</span>))))</code></pre>

Verify a pure ML-DSA signature over msg and ctx (FIPS 204 Algorithms 3 and 8).

[Source](/reference-sources/Crypto.MlDsa.cinch) · line 309

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/MlDsa.cinch`. Source SHA-256: `e68ea78c717b58d088aa25f5c05ede0888066595d9c05b29ce27a9c89a5dfb67`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
