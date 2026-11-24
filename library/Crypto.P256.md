# Crypto.P256

Constant-time NIST P-256 keys, SEC 1 point encodings, ECDH and scalar/point arithmetic over generated Montgomery kernels.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.P256`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.P256.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md).

Non-embedded source closure: [Crypto.P256](Crypto.P256.md), [Data.Bytes.Word](Data.Bytes.Word.md).

<details><summary>Behaviour and boundary notes from the source</summary>

The NIST P-256 curve (secp256r1): constant-time field, scalar and point arithmetic, keys and ECDH.
Field elements and scalars are fixed 256-bit values in Montgomery form, held as
two U128 halves and operated on by straight-line pure-Cinch limb arithmetic
over Data.Wide and Data.Machine, so no arithmetic branches on secret data and
field and scalar operations return unboxed pairs without heap allocation. Points
use projective coordinates and the complete addition and doubling formulas for
a = -3 (Renes, Costello and Batina), which have no exceptional cases; scalar
multiplication is a fixed 4-bit window whose table entry is chosen by scanning
every entry. Public keys are SEC 1 points (uncompressed, or compressed on
input), checked to lie on the curve; private keys are scalars in [1, n - 1].
ECDH yields the shared point's x-coordinate. Crypto.Ecdsa signs and verifies
over this module.

</details>

# Types and constructors

<a id="type-503235364572726f72"></a>

## `P256Error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">P256Error</span></code></pre>

A rejected key or scalar.

[Source](/reference-sources/Crypto.P256.cinch) · line 35

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">P256InvalidPrivateKey</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Error</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">P256InvalidPublicKey</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Error</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">P256InvalidScalar</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Error</span></code></pre>

<a id="type-50323536506f696e74"></a>

## `P256Point`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">P256Point</span></code></pre>

The point.

[Source](/reference-sources/Crypto.P256.cinch) · line 177

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">P256Point</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">Point</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Point</span>)</code></pre>

<a id="type-50323536507269766174654b6579"></a>

## `P256PrivateKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">P256PrivateKey</span></code></pre>

Scalar, then bytes.

[Source](/reference-sources/Crypto.P256.cinch) · line 53

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">P256PrivateKey</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">Sc</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PrivateKey</span>))</code></pre>

<a id="type-503235365075626c69634b6579"></a>

## `P256PublicKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">P256PublicKey</span></code></pre>

x, then y.

[Source](/reference-sources/Crypto.P256.cinch) · line 58

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">P256PublicKey</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">Fe</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">Fe</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PublicKey</span>))</code></pre>

<a id="type-503235365363616c6172"></a>

## `P256Scalar`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">P256Scalar</span></code></pre>

The Montgomery-form value.

[Source](/reference-sources/Crypto.P256.cinch) · line 141

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">P256Scalar</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">Sc</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Scalar</span>)</code></pre>

# Values

<a id="value-703235362d626173652d6d756c74"></a>

## `p256-base-mult`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-base-mult</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Scalar</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Point</span>)</code></pre>

s * G.

[Source](/reference-sources/Crypto.P256.cinch) · line 184

<a id="value-703235362d65636468"></a>

## `p256-ecdh`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-ecdh</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PrivateKey</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PublicKey</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Error</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

ECDH: the x-coordinate (32 bytes) of the private scalar times the peer's point.

[Source](/reference-sources/Crypto.P256.cinch) · line 109

<a id="value-703235362d6572726f722d6d657373616765"></a>

## `p256-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-error-message</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Error</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an error.

[Source](/reference-sources/Crypto.P256.cinch) · line 44

<a id="value-703235362d706f696e742d616464"></a>

## `p256-point-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-point-add</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Point</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Point</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Point</span>))</code></pre>

P + Q.

[Source](/reference-sources/Crypto.P256.cinch) · line 192

<a id="value-703235362d706f696e742d69732d6964656e74697479"></a>

## `p256-point-is-identity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-point-is-identity</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Point</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether a point is the identity.

[Source](/reference-sources/Crypto.P256.cinch) · line 196

<a id="value-703235362d706f696e742d6d756c74"></a>

## `p256-point-mult`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-point-mult</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Scalar</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Point</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Point</span>))</code></pre>

s * P.

[Source](/reference-sources/Crypto.P256.cinch) · line 188

<a id="value-703235362d706f696e742d782d7363616c6172"></a>

## `p256-point-x-scalar`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-point-x-scalar</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Point</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Scalar</span>))</code></pre>

The affine x-coordinate reduced modulo n (None for the identity).

[Source](/reference-sources/Crypto.P256.cinch) · line 200

<a id="value-703235362d707269766174652d6b6579"></a>

## `p256-private-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-private-key</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Error</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PrivateKey</span>))</code></pre>

Parse a 32-byte big-endian private key in [1, n - 1].

[Source](/reference-sources/Crypto.P256.cinch) · line 61

<a id="value-703235362d707269766174652d6b65792d6279746573"></a>

## `p256-private-key-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-private-key-bytes</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PrivateKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The private key's 32 bytes.

[Source](/reference-sources/Crypto.P256.cinch) · line 68

<a id="value-703235362d707269766174652d7363616c6172"></a>

## `p256-private-scalar`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-private-scalar</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PrivateKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Scalar</span>)</code></pre>

The private key's scalar.

[Source](/reference-sources/Crypto.P256.cinch) · line 72

<a id="value-703235362d7075626c69632d6b6579"></a>

## `p256-public-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-public-key</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Error</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PublicKey</span>))</code></pre>

Parse a SEC 1 public key: 0x04 || x || y (65 bytes) or 0x02/0x03 || x (33 bytes); the point must be on the curve.

[Source](/reference-sources/Crypto.P256.cinch) · line 83

<a id="value-703235362d7075626c69632d6b65792d6279746573"></a>

## `p256-public-key-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-public-key-bytes</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PublicKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The uncompressed encoding (65 bytes).

[Source](/reference-sources/Crypto.P256.cinch) · line 96

<a id="value-703235362d7075626c69632d6b65792d636f6d70726573736564"></a>

## `p256-public-key-compressed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-public-key-compressed</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PublicKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The compressed encoding (33 bytes).

[Source](/reference-sources/Crypto.P256.cinch) · line 100

<a id="value-703235362d7075626c69632d6b65792d636f6f7264696e61746573"></a>

## `p256-public-key-coordinates`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-public-key-coordinates</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PublicKey</span> -&gt; (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Bytes</span>))</code></pre>

The affine coordinates as 32-byte big-endian values.

[Source](/reference-sources/Crypto.P256.cinch) · line 105

<a id="value-703235362d7075626c69632d6b65792d6f66"></a>

## `p256-public-key-of`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-public-key-of</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PrivateKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PublicKey</span>)</code></pre>

The public key d * G.

[Source](/reference-sources/Crypto.P256.cinch) · line 76

<a id="value-703235362d7075626c69632d706f696e74"></a>

## `p256-public-point`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-public-point</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PublicKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Point</span>)</code></pre>

A public key's point.

[Source](/reference-sources/Crypto.P256.cinch) · line 180

<a id="value-703235362d7363616c6172"></a>

## `p256-scalar`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-scalar</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Error</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Scalar</span>))</code></pre>

Parse a 32-byte big-endian scalar below n.

[Source](/reference-sources/Crypto.P256.cinch) · line 144

<a id="value-703235362d7363616c61722d616464"></a>

## `p256-scalar-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-scalar-add</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Scalar</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Scalar</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Scalar</span>))</code></pre>

Sum modulo n.

[Source](/reference-sources/Crypto.P256.cinch) · line 167

<a id="value-703235362d7363616c61722d6279746573"></a>

## `p256-scalar-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-scalar-bytes</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Scalar</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The scalar's 32-byte big-endian encoding.

[Source](/reference-sources/Crypto.P256.cinch) · line 155

<a id="value-703235362d7363616c61722d696e7665727365"></a>

## `p256-scalar-inverse`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-scalar-inverse</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Scalar</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Scalar</span>)</code></pre>

Inverse modulo n (zero maps to zero).

[Source](/reference-sources/Crypto.P256.cinch) · line 171

<a id="value-703235362d7363616c61722d69732d7a65726f"></a>

## `p256-scalar-is-zero`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-scalar-is-zero</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Scalar</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether the scalar is zero.

[Source](/reference-sources/Crypto.P256.cinch) · line 159

<a id="value-703235362d7363616c61722d6d756c"></a>

## `p256-scalar-mul`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-scalar-mul</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Scalar</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Scalar</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Scalar</span>))</code></pre>

Product modulo n.

[Source](/reference-sources/Crypto.P256.cinch) · line 163

<a id="value-703235362d7363616c61722d726564756365"></a>

## `p256-scalar-reduce`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">p256-scalar-reduce</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256Scalar</span>)</code></pre>

Interpret up to the first 32 bytes as a big-endian integer and reduce it modulo n (shorter inputs are left-padded).

[Source](/reference-sources/Crypto.P256.cinch) · line 149

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/P256.cinch`. Source SHA-256: `6587ebc2b98dc040e635544f5b181ff4dae6479692077d8a32a795c58b9baf5f`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
