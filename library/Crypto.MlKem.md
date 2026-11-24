# Crypto.MlKem

ML-KEM-768 and ML-KEM-1024 key encapsulation (FIPS 203) with branch-free SIMD lane kernels.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.MlKem`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.MlKem.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Crypto.ConstantTime](Crypto.ConstantTime.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Text.Sha3](Text.Sha3.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Crypto.ConstantTime](Crypto.ConstantTime.md), [Crypto.MlKem](Crypto.MlKem.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Text.Binary](Text.Binary.md), [Text.Sha3](Text.Sha3.md).

<details><summary>Behaviour and boundary notes from the source</summary>

ML-KEM (FIPS 203), the module-lattice key-encapsulation mechanism, with the ML-KEM-768 and ML-KEM-1024 parameter sets.
A decapsulation key is derived from a 64-byte seed and kept as that seed (its
byte form here); the encapsulation key is its public half. Encapsulation
turns 32 bytes of fresh randomness into a 32-byte shared key and a
ciphertext; decapsulation recovers the key, and a ciphertext that does not
re-encrypt to itself yields a pseudorandom rejection key instead, selected
in constant time. Encapsulation keys from outside are checked for
coefficients outside the field. Polynomials are immutable values of
thirty-two 128-bit lanes of eight coefficients; transforms, products,
compression and sampling run as portable branch-free 128-bit SIMD lane ops with
exact arithmetic modulo q = 3329. Effect.Crypto draws seeds and randomness
from the operating system.

</details>

# Types and constructors

<a id="type-4d6c4b656d446563617073756c6174696f6e4b6579"></a>

## `MlKemDecapsulationKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MlKemDecapsulationKey</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Crypto.MlKem.cinch) · line 103

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlKemDecapsulationKey</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">Poly</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemEncapsulationKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemDecapsulationKey</span>))))</code></pre>

<a id="type-4d6c4b656d456e63617073756c6174696f6e4b6579"></a>

## `MlKemEncapsulationKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MlKemEncapsulationKey</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Crypto.MlKem.cinch) · line 98

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlKemEncapsulationKey</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemParameters</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">Poly</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">Poly</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemEncapsulationKey</span>)))))</code></pre>

<a id="type-4d6c4b656d4572726f72"></a>

## `MlKemError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MlKemError</span></code></pre>

A rejected input.

[Source](/reference-sources/Crypto.MlKem.cinch) · line 73

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlKemInvalidSeedSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlKemInvalidEncapsulationKey</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlKemInvalidCiphertextSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlKemInvalidRandomnessSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemError</span>)</code></pre>

<a id="type-4d6c4b656d506172616d6574657273"></a>

## `MlKemParameters`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MlKemParameters</span></code></pre>

An ML-KEM parameter set.

[Source](/reference-sources/Crypto.MlKem.cinch) · line 38

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlKem768</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemParameters</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">MlKem1024</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemParameters</span></code></pre>

# Values

<a id="value-6d6c6b656d2d636970686572746578742d73697a65"></a>

## `mlkem-ciphertext-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mlkem-ciphertext-size</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemParameters</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Ciphertext bytes: 1088 for ML-KEM-768 and 1568 for ML-KEM-1024.

[Source](/reference-sources/Crypto.MlKem.cinch) · line 69

<a id="value-6d6c6b656d2d646563617073756c617465"></a>

## `mlkem-decapsulate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mlkem-decapsulate</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemDecapsulationKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Decapsulate (FIPS 203 Algorithm 18): the shared key, or the rejection key J(z || c) when the ciphertext
does not re-encrypt to itself. The choice is made in constant time.

[Source](/reference-sources/Crypto.MlKem.cinch) · line 246

<a id="value-6d6c6b656d2d646563617073756c6174696f6e2d6b6579"></a>

## `mlkem-decapsulation-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mlkem-decapsulation-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemParameters</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemDecapsulationKey</span>)))</code></pre>

Derive a decapsulation key from a 64-byte seed d || z (FIPS 203 Algorithms 13 and 16).

[Source](/reference-sources/Crypto.MlKem.cinch) · line 126

<a id="value-6d6c6b656d2d646563617073756c6174696f6e2d6b65792d6279746573"></a>

## `mlkem-decapsulation-key-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mlkem-decapsulation-key-bytes</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemDecapsulationKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The decapsulation key's 64-byte seed, from which it can be rebuilt.

[Source](/reference-sources/Crypto.MlKem.cinch) · line 114

<a id="value-6d6c6b656d2d646563617073756c6174696f6e2d6b65792d706172616d6574657273"></a>

## `mlkem-decapsulation-key-parameters`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mlkem-decapsulation-key-parameters</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemDecapsulationKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemParameters</span>)</code></pre>

The decapsulation key's parameter set.

[Source](/reference-sources/Crypto.MlKem.cinch) · line 118

<a id="value-6d6c6b656d2d656e63617073756c6174652d77697468"></a>

## `mlkem-encapsulate-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mlkem-encapsulate-with</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemEncapsulationKey</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Bytes</span>))))</code></pre>

Encapsulate with explicit 32-byte randomness m (FIPS 203 Algorithm 17): the shared key and the ciphertext.
The randomness must be fresh and secret for each call; Effect.Crypto supplies it from the operating system.

[Source](/reference-sources/Crypto.MlKem.cinch) · line 217

<a id="value-6d6c6b656d2d656e63617073756c6174696f6e2d6b6579"></a>

## `mlkem-encapsulation-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mlkem-encapsulation-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemParameters</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemEncapsulationKey</span>)))</code></pre>

Parse and check an encapsulation key: the right length, and every coefficient below q (FIPS 203 section 7.2).

[Source](/reference-sources/Crypto.MlKem.cinch) · line 142

<a id="value-6d6c6b656d2d656e63617073756c6174696f6e2d6b65792d6279746573"></a>

## `mlkem-encapsulation-key-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mlkem-encapsulation-key-bytes</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemEncapsulationKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The encapsulation key's bytes.

[Source](/reference-sources/Crypto.MlKem.cinch) · line 110

<a id="value-6d6c6b656d2d656e63617073756c6174696f6e2d6b65792d6f66"></a>

## `mlkem-encapsulation-key-of`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mlkem-encapsulation-key-of</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemDecapsulationKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemEncapsulationKey</span>)</code></pre>

The public encapsulation key of a decapsulation key.

[Source](/reference-sources/Crypto.MlKem.cinch) · line 122

<a id="value-6d6c6b656d2d656e63617073756c6174696f6e2d6b65792d706172616d6574657273"></a>

## `mlkem-encapsulation-key-parameters`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mlkem-encapsulation-key-parameters</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemEncapsulationKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemParameters</span>)</code></pre>

The encapsulation key's parameter set.

[Source](/reference-sources/Crypto.MlKem.cinch) · line 106

<a id="value-6d6c6b656d2d656e63617073756c6174696f6e2d6b65792d73697a65"></a>

## `mlkem-encapsulation-key-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mlkem-encapsulation-key-size</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemParameters</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Encapsulation-key bytes: 1184 for ML-KEM-768 and 1568 for ML-KEM-1024.

[Source](/reference-sources/Crypto.MlKem.cinch) · line 65

<a id="value-6d6c6b656d2d6572726f722d6d657373616765"></a>

## `mlkem-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mlkem-error-message</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">MlKem</span>.<span class="tok-constructor">MlKemError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an error.

[Source](/reference-sources/Crypto.MlKem.cinch) · line 84

<a id="value-6d6c6b656d2d736565642d73697a65"></a>

## `mlkem-seed-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mlkem-seed-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Decapsulation-key seeds are 64 bytes (d then z).

[Source](/reference-sources/Crypto.MlKem.cinch) · line 61

<a id="value-6d6c6b656d2d7368617265642d6b65792d73697a65"></a>

## `mlkem-shared-key-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mlkem-shared-key-size</span> :: <span class="tok-constructor">Nat</span></code></pre>

Shared keys are 32 bytes.

[Source](/reference-sources/Crypto.MlKem.cinch) · line 57

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/MlKem.cinch`. Source SHA-256: `f690ebbc0e580315519d705e0efdc68d2c71b68a0ea99d952674545814bc7b7f`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
