# Crypto.MlKem

ML-KEM-768 and ML-KEM-1024 key encapsulation (FIPS 203) with branch-free SIMD lane kernels.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Crypto.MlKem</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Crypto.MlKem/source/">View source</a><a href="/packages/search/?q=%2BCrypto.MlKem">Search module</a><a href="/reference-sources/Crypto.MlKem.cinch" download="Crypto.MlKem.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Crypto.ConstantTime](Crypto.ConstantTime.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Sha3](Text.Sha3.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

ML-KEM (FIPS 203), the module-lattice key-encapsulation mechanism, with the ML-KEM-768 and ML-KEM-1024 parameter sets.
A decapsulation key is derived from a 64-byte seed and kept as that seed (its
byte form here); the encapsulation key is its public half. Encapsulation
turns 32 bytes of fresh randomness into a 32-byte shared key and a
ciphertext; decapsulation recovers the key, and a ciphertext that does not
re-encrypt to itself yields a pseudorandom rejection key instead, selected
in constant time. Encapsulation keys from outside are checked for
coefficients outside the field. Polynomials are immutable values of
thirty-two 128-bit lanes of eight coefficients; transforms, products,
compression and sampling run as generated branch-free lane kernels with
exact arithmetic modulo q = 3329. Effect.Crypto draws seeds and randomness
from the operating system.

</details>

<details class="doc-synopsis" open><summary>Synopsis (18 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-MlKemDecapsulationKey">MlKemDecapsulationKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MlKemDecapsulationKey</strong></code></li><li><a class="syn-name" href="#t-MlKemEncapsulationKey">MlKemEncapsulationKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MlKemEncapsulationKey</strong></code></li><li><a class="syn-name" href="#t-MlKemError">MlKemError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MlKemError</strong></code></li><li><a class="syn-name" href="#t-MlKemParameters">MlKemParameters</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MlKemParameters</strong></code></li><li><a class="syn-name" href="#v-mlkem-ciphertext-size">mlkem-ciphertext-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mlkem-ciphertext-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemParameters" title="Crypto.MlKem.MlKemParameters">MlKemParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-mlkem-decapsulate">mlkem-decapsulate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mlkem-decapsulate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemDecapsulationKey" title="Crypto.MlKem.MlKemDecapsulationKey">MlKemDecapsulationKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemError" title="Crypto.MlKem.MlKemError">MlKemError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-mlkem-decapsulation-key">mlkem-decapsulation-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mlkem-decapsulation-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemParameters" title="Crypto.MlKem.MlKemParameters">MlKemParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemError" title="Crypto.MlKem.MlKemError">MlKemError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemDecapsulationKey" title="Crypto.MlKem.MlKemDecapsulationKey">MlKemDecapsulationKey</a></code></li><li><a class="syn-name" href="#v-mlkem-decapsulation-key-bytes">mlkem-decapsulation-key-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mlkem-decapsulation-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemDecapsulationKey" title="Crypto.MlKem.MlKemDecapsulationKey">MlKemDecapsulationKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-mlkem-decapsulation-key-parameters">mlkem-decapsulation-key-parameters</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mlkem-decapsulation-key-parameters</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemDecapsulationKey" title="Crypto.MlKem.MlKemDecapsulationKey">MlKemDecapsulationKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemParameters" title="Crypto.MlKem.MlKemParameters">MlKemParameters</a></code></li><li><a class="syn-name" href="#v-mlkem-encapsulate-with">mlkem-encapsulate-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mlkem-encapsulate-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemEncapsulationKey" title="Crypto.MlKem.MlKemEncapsulationKey">MlKemEncapsulationKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemError" title="Crypto.MlKem.MlKemError">MlKemError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-mlkem-encapsulation-key">mlkem-encapsulation-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mlkem-encapsulation-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemParameters" title="Crypto.MlKem.MlKemParameters">MlKemParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemError" title="Crypto.MlKem.MlKemError">MlKemError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemEncapsulationKey" title="Crypto.MlKem.MlKemEncapsulationKey">MlKemEncapsulationKey</a></code></li><li><a class="syn-name" href="#v-mlkem-encapsulation-key-bytes">mlkem-encapsulation-key-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mlkem-encapsulation-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemEncapsulationKey" title="Crypto.MlKem.MlKemEncapsulationKey">MlKemEncapsulationKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-mlkem-encapsulation-key-of">mlkem-encapsulation-key-of</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mlkem-encapsulation-key-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemDecapsulationKey" title="Crypto.MlKem.MlKemDecapsulationKey">MlKemDecapsulationKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemEncapsulationKey" title="Crypto.MlKem.MlKemEncapsulationKey">MlKemEncapsulationKey</a></code></li><li><a class="syn-name" href="#v-mlkem-encapsulation-key-parameters">mlkem-encapsulation-key-parameters</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mlkem-encapsulation-key-parameters</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemEncapsulationKey" title="Crypto.MlKem.MlKemEncapsulationKey">MlKemEncapsulationKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemParameters" title="Crypto.MlKem.MlKemParameters">MlKemParameters</a></code></li><li><a class="syn-name" href="#v-mlkem-encapsulation-key-size">mlkem-encapsulation-key-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mlkem-encapsulation-key-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemParameters" title="Crypto.MlKem.MlKemParameters">MlKemParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-mlkem-error-message">mlkem-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mlkem-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemError" title="Crypto.MlKem.MlKemError">MlKemError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-mlkem-seed-size">mlkem-seed-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mlkem-seed-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-mlkem-shared-key-size">mlkem-shared-key-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mlkem-shared-key-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-MlKemDecapsulationKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MlKemDecapsulationKey</strong></code><span class="doc-sig-links"><a href="#t-MlKemDecapsulationKey" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlKem/source/#L103" title="View source at line 103">Source</a></span></div>
<div class="doc-body">

Fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MlKemDecapsulationKey"><code><strong class="tok-constructor">MlKemDecapsulationKey</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-constructor">Poly</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemEncapsulationKey" title="Crypto.MlKem.MlKemEncapsulationKey">MlKemEncapsulationKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemDecapsulationKey" title="Crypto.MlKem.MlKemDecapsulationKey">MlKemDecapsulationKey</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MlKemEncapsulationKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MlKemEncapsulationKey</strong></code><span class="doc-sig-links"><a href="#t-MlKemEncapsulationKey" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlKem/source/#L98" title="View source at line 98">Source</a></span></div>
<div class="doc-body">

Fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MlKemEncapsulationKey"><code><strong class="tok-constructor">MlKemEncapsulationKey</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemParameters" title="Crypto.MlKem.MlKemParameters">MlKemParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-constructor">Poly</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-constructor">Poly</span>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemEncapsulationKey" title="Crypto.MlKem.MlKemEncapsulationKey">MlKemEncapsulationKey</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MlKemError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MlKemError</strong></code><span class="doc-sig-links"><a href="#t-MlKemError" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlKem/source/#L73" title="View source at line 73">Source</a></span></div>
<div class="doc-body">

A rejected input.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MlKemInvalidSeedSize"><code><strong class="tok-constructor">MlKemInvalidSeedSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemError" title="Crypto.MlKem.MlKemError">MlKemError</a></code></li><li id="k-MlKemInvalidEncapsulationKey"><code><strong class="tok-constructor">MlKemInvalidEncapsulationKey</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemError" title="Crypto.MlKem.MlKemError">MlKemError</a></code></li><li id="k-MlKemInvalidCiphertextSize"><code><strong class="tok-constructor">MlKemInvalidCiphertextSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemError" title="Crypto.MlKem.MlKemError">MlKemError</a></code></li><li id="k-MlKemInvalidRandomnessSize"><code><strong class="tok-constructor">MlKemInvalidRandomnessSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemError" title="Crypto.MlKem.MlKemError">MlKemError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MlKemParameters">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MlKemParameters</strong></code><span class="doc-sig-links"><a href="#t-MlKemParameters" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlKem/source/#L38" title="View source at line 38">Source</a></span></div>
<div class="doc-body">

An ML-KEM parameter set.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MlKem768"><code><strong class="tok-constructor">MlKem768</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemParameters" title="Crypto.MlKem.MlKemParameters">MlKemParameters</a></code></li><li id="k-MlKem1024"><code><strong class="tok-constructor">MlKem1024</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemParameters" title="Crypto.MlKem.MlKemParameters">MlKemParameters</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-mlkem-ciphertext-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mlkem-ciphertext-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemParameters" title="Crypto.MlKem.MlKemParameters">MlKemParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-mlkem-ciphertext-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlKem/source/#L69" title="View source at line 69">Source</a></span></div>
<div class="doc-body">

Ciphertext bytes: 1088 for ML-KEM-768 and 1568 for ML-KEM-1024.

</div>
</article>

<article class="doc-entry" id="v-mlkem-decapsulate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mlkem-decapsulate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemDecapsulationKey" title="Crypto.MlKem.MlKemDecapsulationKey">MlKemDecapsulationKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemError" title="Crypto.MlKem.MlKemError">MlKemError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-mlkem-decapsulate" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlKem/source/#L246" title="View source at line 246">Source</a></span></div>
<div class="doc-body">

Decapsulate (FIPS 203 Algorithm 18): the shared key, or the rejection key J(z || c) when the ciphertext
does not re-encrypt to itself. The choice is made in constant time.

</div>
</article>

<article class="doc-entry" id="v-mlkem-decapsulation-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mlkem-decapsulation-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemParameters" title="Crypto.MlKem.MlKemParameters">MlKemParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemError" title="Crypto.MlKem.MlKemError">MlKemError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemDecapsulationKey" title="Crypto.MlKem.MlKemDecapsulationKey">MlKemDecapsulationKey</a></code><span class="doc-sig-links"><a href="#v-mlkem-decapsulation-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlKem/source/#L126" title="View source at line 126">Source</a></span></div>
<div class="doc-body">

Derive a decapsulation key from a 64-byte seed d || z (FIPS 203 Algorithms 13 and 16).

</div>
</article>

<article class="doc-entry" id="v-mlkem-decapsulation-key-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mlkem-decapsulation-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemDecapsulationKey" title="Crypto.MlKem.MlKemDecapsulationKey">MlKemDecapsulationKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-mlkem-decapsulation-key-bytes" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlKem/source/#L114" title="View source at line 114">Source</a></span></div>
<div class="doc-body">

The decapsulation key's 64-byte seed, from which it can be rebuilt.

</div>
</article>

<article class="doc-entry" id="v-mlkem-decapsulation-key-parameters">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mlkem-decapsulation-key-parameters</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemDecapsulationKey" title="Crypto.MlKem.MlKemDecapsulationKey">MlKemDecapsulationKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemParameters" title="Crypto.MlKem.MlKemParameters">MlKemParameters</a></code><span class="doc-sig-links"><a href="#v-mlkem-decapsulation-key-parameters" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlKem/source/#L118" title="View source at line 118">Source</a></span></div>
<div class="doc-body">

The decapsulation key's parameter set.

</div>
</article>

<article class="doc-entry" id="v-mlkem-encapsulate-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mlkem-encapsulate-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemEncapsulationKey" title="Crypto.MlKem.MlKemEncapsulationKey">MlKemEncapsulationKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemError" title="Crypto.MlKem.MlKemError">MlKemError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-mlkem-encapsulate-with" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlKem/source/#L217" title="View source at line 217">Source</a></span></div>
<div class="doc-body">

Encapsulate with explicit 32-byte randomness m (FIPS 203 Algorithm 17): the shared key and the ciphertext.
The randomness must be fresh and secret for each call; Effect.Crypto supplies it from the operating system.

</div>
</article>

<article class="doc-entry" id="v-mlkem-encapsulation-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mlkem-encapsulation-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemParameters" title="Crypto.MlKem.MlKemParameters">MlKemParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemError" title="Crypto.MlKem.MlKemError">MlKemError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemEncapsulationKey" title="Crypto.MlKem.MlKemEncapsulationKey">MlKemEncapsulationKey</a></code><span class="doc-sig-links"><a href="#v-mlkem-encapsulation-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlKem/source/#L142" title="View source at line 142">Source</a></span></div>
<div class="doc-body">

Parse and check an encapsulation key: the right length, and every coefficient below q (FIPS 203 section 7.2).

</div>
</article>

<article class="doc-entry" id="v-mlkem-encapsulation-key-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mlkem-encapsulation-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemEncapsulationKey" title="Crypto.MlKem.MlKemEncapsulationKey">MlKemEncapsulationKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-mlkem-encapsulation-key-bytes" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlKem/source/#L110" title="View source at line 110">Source</a></span></div>
<div class="doc-body">

The encapsulation key's bytes.

</div>
</article>

<article class="doc-entry" id="v-mlkem-encapsulation-key-of">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mlkem-encapsulation-key-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemDecapsulationKey" title="Crypto.MlKem.MlKemDecapsulationKey">MlKemDecapsulationKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemEncapsulationKey" title="Crypto.MlKem.MlKemEncapsulationKey">MlKemEncapsulationKey</a></code><span class="doc-sig-links"><a href="#v-mlkem-encapsulation-key-of" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlKem/source/#L122" title="View source at line 122">Source</a></span></div>
<div class="doc-body">

The public encapsulation key of a decapsulation key.

</div>
</article>

<article class="doc-entry" id="v-mlkem-encapsulation-key-parameters">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mlkem-encapsulation-key-parameters</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemEncapsulationKey" title="Crypto.MlKem.MlKemEncapsulationKey">MlKemEncapsulationKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemParameters" title="Crypto.MlKem.MlKemParameters">MlKemParameters</a></code><span class="doc-sig-links"><a href="#v-mlkem-encapsulation-key-parameters" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlKem/source/#L106" title="View source at line 106">Source</a></span></div>
<div class="doc-body">

The encapsulation key's parameter set.

</div>
</article>

<article class="doc-entry" id="v-mlkem-encapsulation-key-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mlkem-encapsulation-key-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemParameters" title="Crypto.MlKem.MlKemParameters">MlKemParameters</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-mlkem-encapsulation-key-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlKem/source/#L65" title="View source at line 65">Source</a></span></div>
<div class="doc-body">

Encapsulation-key bytes: 1184 for ML-KEM-768 and 1568 for ML-KEM-1024.

</div>
</article>

<article class="doc-entry" id="v-mlkem-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mlkem-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.MlKem/#t-MlKemError" title="Crypto.MlKem.MlKemError">MlKemError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-mlkem-error-message" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlKem/source/#L84" title="View source at line 84">Source</a></span></div>
<div class="doc-body">

Render an error.

</div>
</article>

<article class="doc-entry" id="v-mlkem-seed-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mlkem-seed-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-mlkem-seed-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlKem/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

Decapsulation-key seeds are 64 bytes (d then z).

</div>
</article>

<article class="doc-entry" id="v-mlkem-shared-key-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mlkem-shared-key-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-mlkem-shared-key-size" title="Permalink">#</a><a href="/packages/stdlib/Crypto.MlKem/source/#L57" title="View source at line 57">Source</a></span></div>
<div class="doc-body">

Shared keys are 32 bytes.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Crypto/MlKem.cinch`](/packages/stdlib/Crypto.MlKem/source/). Source SHA-256: `130822b4a06af07c585a99e840d37c81576b642ae74a492e8d017c4a641bf6ed`.
